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
    C0(Bytes, u64),
    C1(Bytes),
}

#[derive(Clone)]
pub enum Adt33 {
    C0(Bytes, List<Adt23>),
}

#[derive(Clone)]
pub enum Adt34 {
    C0(Adt15, List<Adt23>),
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
    F67(u8, Bytes),
    F291(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F67(k0, k1) => f67(*k0, k1.clone(), p0),
            Fn0::F291(k0) => f291(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F70(Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn1::F70(k0) => f70(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F117,
    F293,
    F399,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn2::F117 => f117(p0, p1),
            Fn2::F293 => f293(p0, p1),
            Fn2::F399 => f399(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F188(Bytes, u64, u64),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Adt20)) -> R<(u64, Adt20)> {
        match self {
            Fn3::F188(k0, k1, k2) => f188(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F200(Bytes),
    F210(u64, Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn4::F200(k0) => f200(k0.clone(), p0),
            Fn4::F210(k0, k1) => Ok(f210(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F202(Bytes),
    F203(Bytes),
    F229(Bytes),
    F267(Bytes),
    F268(Bytes),
    F295(Bytes, u64, Bytes, u64),
    F348(Bytes, Bytes),
    F349(Bytes),
    F350(Bytes, u64),
    F386(Bytes),
    F408(Bytes),
    F410(Bytes),
    F422(Bytes, u64),
    F467(bool, u64, Bytes),
    F490(Bytes),
    F517(Bytes),
    F519(Bytes),
    F545(Bytes, bool),
    F548(u64, Bytes),
    F696(u64, Bytes, u64),
    F700(Bytes),
    F701(Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn5::F202(k0) => f202(k0.clone(), p0),
            Fn5::F203(k0) => f203(k0.clone(), p0),
            Fn5::F229(k0) => f229(k0.clone(), p0),
            Fn5::F267(k0) => f267(k0.clone(), p0),
            Fn5::F268(k0) => f268(k0.clone(), p0),
            Fn5::F295(k0, k1, k2, k3) => f295(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn5::F348(k0, k1) => f348(k0.clone(), k1.clone(), p0),
            Fn5::F349(k0) => f349(k0.clone(), p0),
            Fn5::F350(k0, k1) => f350(k0.clone(), *k1, p0),
            Fn5::F386(k0) => f386(k0.clone(), p0),
            Fn5::F408(k0) => f408(k0.clone(), p0),
            Fn5::F410(k0) => f410(k0.clone(), p0),
            Fn5::F422(k0, k1) => f422(k0.clone(), *k1, p0),
            Fn5::F467(k0, k1, k2) => f467(*k0, *k1, k2.clone(), p0),
            Fn5::F490(k0) => f490(k0.clone(), p0),
            Fn5::F517(k0) => f517(k0.clone(), p0),
            Fn5::F519(k0) => f519(k0.clone(), p0),
            Fn5::F545(k0, k1) => f545(k0.clone(), *k1, p0),
            Fn5::F548(k0, k1) => f548(*k0, k1.clone(), p0),
            Fn5::F696(k0, k1, k2) => f696(*k0, k1.clone(), *k2, p0),
            Fn5::F700(k0) => f700(k0.clone(), p0),
            Fn5::F701(k0) => f701(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F271(Bytes),
    F525(Bytes),
    F541(bool, Bytes),
    F578(bool, Bytes),
    F633(Bytes),
    F647(Bytes),
    F695(u64, Bytes),
    F698(u64, Bytes),
}

impl Fn6 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn6::F271(k0) => f271(k0.clone(), p0),
            Fn6::F525(k0) => f525(k0.clone(), p0),
            Fn6::F541(k0, k1) => f541(*k0, k1.clone(), p0),
            Fn6::F578(k0, k1) => f578(*k0, k1.clone(), p0),
            Fn6::F633(k0) => f633(k0.clone(), p0),
            Fn6::F647(k0) => f647(k0.clone(), p0),
            Fn6::F695(k0, k1) => f695(*k0, k1.clone(), p0),
            Fn6::F698(k0, k1) => f698(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F388,
}

impl Fn7 {
    pub fn apply(&self, p0: Adt30) -> R<Adt30> {
        match self {
            Fn7::F388 => f388(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F393,
    F756,
}

impl Fn8 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn8::F393 => f393(p0, p1),
            Fn8::F756 => f756(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F396,
}

impl Fn9 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn9::F396 => f396(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F506,
}

impl Fn10 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn10::F506 => f506(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F509,
}

impl Fn11 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn11::F509 => f509(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F511,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn12::F511 => f511(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F514,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn13::F514 => f514(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F604(Bytes),
}

impl Fn14 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn14::F604(k0) => f604(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F613,
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn15::F613 => f613(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F618,
}

impl Fn16 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn16::F618 => f618(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F620,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn17::F620 => f620(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F622,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn18::F622 => f622(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F625,
}

impl Fn19 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn19::F625 => f625(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F644(u64, Bytes),
}

impl Fn20 {
    pub fn apply(&self, p0: (u64, bool)) -> R<Option<(u64, bool)>> {
        match self {
            Fn20::F644(k0, k1) => f644(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F692,
}

impl Fn21 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn21::F692 => f692(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F726,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn22::F726 => f726(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F742,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn23::F742 => f742(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F751,
}

impl Fn24 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn24::F751 => f751(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F754,
}

impl Fn25 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn25::F754 => f754(p0),
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
    if f32(v0.clone(), f8(Str::lit("check-yaml"))) {
        f56(v3, v1)
    } else {
        if f32(v0.clone(), f8(Str::lit("read-webidl"))) {
            f57(v3, v1, v2)
        } else {
            if f32(v0.clone(), f8(Str::lit("read-xml"))) {
                f58(v3, v1)
            } else {
                if f32(v0.clone(), f8(Str::lit("read-xsd"))) {
                    f59(v3, v1)
                } else {
                    if f32(v0.clone(), f8(Str::lit("read-xsd-components"))) {
                        f60(v3, v1)
                    } else {
                        if f32(v0, f8(Str::lit("read-yaml"))) {
                            f61(v3, v1)
                        } else {
                            Ok(Adt12::C4(f31()))
                        }
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
    let v1: u64 = f62(v0)?;
    let m199 = {
        let a197 = f63({
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
    let v1: Bytes = f64(v0)?;
    let m229 = f65(v1.clone(), 0u64, {
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
        let (h236, _) = f66(Fn0::F67(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h236
    };
    let v4: u64 = {
        let (h237, _) = {
            let (_, h238) = v3.clone();
            h238
        };
        h237
    };
    Ok(f68(List::cons(f46(v0, v4, {
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
    let m251 = f69(Fn1::F70(v1), None::<Bytes>, v0);
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
    List::cons(f8(Str::lit("admit JSONL")), List::cons(f8(Str::lit("admit-encoding STAGED")), List::cons(f8(Str::lit("admit-registry AUTHORITY ROW CONSTRUCT|none read-OP ARTIFACT")), List::cons(f8(Str::lit("admit-translated FILE NAME read-OP ARG... CITATION...")), List::cons(f8(Str::lit("agenda")), List::cons(f8(Str::lit("build")), List::cons(f8(Str::lit("check")), List::cons(f8(Str::lit("check-yaml FILE")), List::cons(f8(Str::lit("ingest-authority BINDING")), List::cons(f8(Str::lit("ingest-index CSV")), List::cons(f8(Str::lit("merge-gate RULES")), List::cons(f8(Str::lit("next [SKIP...]")), List::cons(f8(Str::lit("propose-encoding SCHEMA NAME CITATION...")), List::cons(f8(Str::lit("publish")), List::cons(f8(Str::lit("read-admit NAME read-OP FILE ARG... CITATION...")), List::cons(f8(Str::lit("read-webidl FILE [IMPORT...]")), List::cons(f8(Str::lit("read-xml FILE")), List::cons(f8(Str::lit("read-xsd FILE")), List::cons(f8(Str::lit("read-xsd-components FILE")), List::cons(f8(Str::lit("read-yaml FILE")), List::cons(f8(Str::lit("record-decision JSONL")), List::cons(f8(Str::lit("record-error REFUSAL TIME RUN OPERATION...")), List::cons(f8(Str::lit("stage-admissions JSONL")), List::cons(f8(Str::lit("stage-bindings JSONL")), List::cons(f8(Str::lit("stage-decisions JSONL")), List::cons(f8(Str::lit("stage-encodings JSONL")), List::cons(f8(Str::lit("stage-engine")), List::cons(f8(Str::lit("stage-mappings JSONL")), List::cons(f8(Str::lit("stage-matrix JSONL")), List::cons(f8(Str::lit("stage-registries JSONL")), List::cons(f8(Str::lit("verify-translations ADMISSIONS")), List::<Bytes>::nil())))))))))))))))))))))))))))))))
}

pub fn f49(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m252 = v1;
    match m252.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m253 = f71(f72(v0.clone(), v2.clone()), v2.clone());
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
                        let m255 = f73(v2.clone(), f8(Str::lit("translation")));
                        match m255 {
                            None => {
                                Ok::<List<Adt2>, R<Adt12>>(v3)
                            }
                            Some(v4) => {
                                let v6: Bytes = {
                                    let m256 = f74(f73(v2, f8(Str::lit("construct"))));
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
                                    let m257 = f74(f73(v4.clone(), f8(Str::lit("output_sha256"))));
                                    match m257 {
                                        None => {
                                            Bytes::lit(&[])
                                        }
                                        Some(v7) => {
                                            v7
                                        }
                                    }
                                };
                                let m258 = f75(f76(f73(v4, f8(Str::lit("command")))));
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
                                                        if f32(f77(v20.clone())?, v8.clone()) {
                                                            Ok::<List<Adt2>, R<Adt12>>(v3)
                                                        } else {
                                                            Err::<List<Adt2>, R<Adt12>>(Ok(Adt12::C4(f7(List::cons(v6, List::cons(f8(Str::lit(": its reader command now prints output with sha256 ")), List::cons(f77(v20)?, List::cons(f8(Str::lit(", and the admission records ")), List::cons(v8, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))))
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
    Ok(f76(f78(v0, v1)?))
}

pub fn f52(v0: List<Bytes>) -> List<Bytes> {
    let m263 = v0;
    match m263.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f79(v1.clone()) {
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
    let m271 = f80(v4);
    match m271 {
        None => {
            Ok(Adt12::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
        }
        Some(v5) => {
            let m272 = f81(f82(List::cons(Adt2::C7(std::rc::Rc::new(v5.clone())), List::<Adt2>::nil())), f83(f51(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m272 {
                None => {
                    f85(v0, v2.clone(), Adt2::C10(f8(Str::lit("translation")), std::rc::Rc::new(Adt2::C7(std::rc::Rc::new(Adt2::C10(f8(Str::lit("command")), std::rc::Rc::new(Adt2::C6(std::rc::Rc::new(f86(v3)))), std::rc::Rc::new(Adt2::C10(f8(Str::lit("output_sha256")), std::rc::Rc::new(Adt2::C5(f77(v1.clone())?)), std::rc::Rc::new(Adt2::C8))))))), std::rc::Rc::new(v5)), f87(v2, f88(v1)?)?, f8(Str::lit("Encoding entered through admit-translated from the verdant_readers command recorded in its admission.")), List::<(Bytes, Bytes)>::nil())
                }
                Some(v6) => {
                    Ok(f84(v6))
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
                        if f79(v1.clone()) {
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

pub fn f56(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m276 = f49(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m276.uncons() {
        None => {
            let m277 = f89(f90(v0, v1))?;
            match m277 {
                Ok(v5) => {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f91(f92(v5)?)?, List::cons(f8(Str::lit(" documents\u{a}")), List::<Bytes>::nil())))))
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

pub fn f57(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<Adt12> {
    let m278 = f49(v0.clone(), List::cons(v1.clone(), v2.clone()));
    match m278.uncons() {
        None => {
            let m279 = f93(f90(v0.clone(), v1), f94(v0, v2))?;
            match m279 {
                Ok(v6) => {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("-- ")), List::cons(f91({
                        let (h280, _) = v6;
                        h280
                    })?, List::cons(f8(Str::lit(" Web IDL definitions translated\u{a}")), List::cons({
                        let (_, h281) = v6;
                        h281
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

pub fn f58(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m282 = f49(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m282.uncons() {
        None => {
            let m283 = f95(f90(v0, v1))?;
            match m283 {
                Ok(v5) => {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("well-formed XML: root ")), List::cons(f96(v5.clone()), List::cons(f8(Str::lit(", ")), List::cons(f91(f97(v5)?)?, List::cons(f8(Str::lit(" elements\u{a}")), List::<Bytes>::nil()))))))))
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

pub fn f59(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m284 = f49(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m284.uncons() {
        None => {
            let m285 = f98(f90(v0, v1))?;
            match m285 {
                Ok(v5) => {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("-- ")), List::cons(f91({
                        let (h286, _) = v5;
                        h286
                    })?, List::cons(f8(Str::lit(" simple types\u{a}")), List::cons({
                        let (_, h287) = v5;
                        h287
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

pub fn f60(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m288 = f49(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::<Bytes>::nil())));
    match m288.uncons() {
        None => {
            let m289 = f99(f100(v0.clone()), f90(v0, f8(Str::lit("model/authorities.toml"))), v1)?;
            match m289 {
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

pub fn f61(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m290 = f49(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m290.uncons() {
        None => {
            let m291 = f89(f90(v0, v1))?;
            match m291 {
                Ok(v5) => {
                    let m292 = f101(v5)?;
                    match m292 {
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

pub fn f62(v0: u8) -> R<u64> {
    let v1: u8 = v0;
    let a337 = {
        let a313 = {
            let a301 = {
                let a295 = {
                    let a293 = v1;
                    let a294 = 128u8;
                    fixed_u8::bit_and(a293, a294)
                };
                let a296 = 128u8;
                if equal(a295, a296) {
                    128u64
                } else {
                    0u64
                }
            };
            let a302 = {
                let a299 = {
                    let a297 = v1;
                    let a298 = 64u8;
                    fixed_u8::bit_and(a297, a298)
                };
                let a300 = 64u8;
                if equal(a299, a300) {
                    64u64
                } else {
                    0u64
                }
            };
            nat_add(a301, a302)?
        };
        let a314 = {
            let a311 = {
                let a305 = {
                    let a303 = v1;
                    let a304 = 32u8;
                    fixed_u8::bit_and(a303, a304)
                };
                let a306 = 32u8;
                if equal(a305, a306) {
                    32u64
                } else {
                    0u64
                }
            };
            let a312 = {
                let a309 = {
                    let a307 = v1;
                    let a308 = 16u8;
                    fixed_u8::bit_and(a307, a308)
                };
                let a310 = 16u8;
                if equal(a309, a310) {
                    16u64
                } else {
                    0u64
                }
            };
            nat_add(a311, a312)?
        };
        nat_add(a313, a314)?
    };
    let a338 = {
        let a335 = {
            let a323 = {
                let a317 = {
                    let a315 = v1;
                    let a316 = 8u8;
                    fixed_u8::bit_and(a315, a316)
                };
                let a318 = 8u8;
                if equal(a317, a318) {
                    8u64
                } else {
                    0u64
                }
            };
            let a324 = {
                let a321 = {
                    let a319 = v1;
                    let a320 = 4u8;
                    fixed_u8::bit_and(a319, a320)
                };
                let a322 = 4u8;
                if equal(a321, a322) {
                    4u64
                } else {
                    0u64
                }
            };
            nat_add(a323, a324)?
        };
        let a336 = {
            let a333 = {
                let a327 = {
                    let a325 = v1;
                    let a326 = 2u8;
                    fixed_u8::bit_and(a325, a326)
                };
                let a328 = 2u8;
                if equal(a327, a328) {
                    2u64
                } else {
                    0u64
                }
            };
            let a334 = {
                let a331 = {
                    let a329 = v1;
                    let a330 = 1u8;
                    fixed_u8::bit_and(a329, a330)
                };
                let a332 = 1u8;
                if equal(a331, a332) {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a333, a334)?
        };
        nat_add(a335, a336)?
    };
    nat_add(a337, a338)
}

pub fn f63(v0: u64) -> List<Str> {
    let a339 = v0;
    let a340 = 0u64;
    if nat_eq(a339, a340) {
        f102()
    } else {
        let a341 = v0;
        let a342 = 1u64;
        if nat_eq(a341, a342) {
            f103()
        } else {
            let a343 = v0;
            let a344 = 2u64;
            if nat_eq(a343, a344) {
                f104()
            } else {
                let a345 = v0;
                let a346 = 3u64;
                if nat_eq(a345, a346) {
                    f105()
                } else {
                    let a347 = v0;
                    let a348 = 4u64;
                    if nat_eq(a347, a348) {
                        f106()
                    } else {
                        let a349 = v0;
                        let a350 = 5u64;
                        if nat_eq(a349, a350) {
                            f107()
                        } else {
                            let a351 = v0;
                            let a352 = 6u64;
                            if nat_eq(a351, a352) {
                                f108()
                            } else {
                                let a353 = v0;
                                let a354 = 7u64;
                                if nat_eq(a353, a354) {
                                    f109()
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

pub fn f64(v0: Bytes) -> R<Bytes> {
    f110(v0.clone(), 0u64, {
        let a355 = v0;
        length_bytes(a355)
    }, false, false, 0u64, Bytes::lit(&[]))
}

pub fn f65(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt2, u64)>> {
    let m356 = v2;
    if m356 == 0 {
        Ok(None::<(Adt2, u64)>)
    } else {
        let v4 = m356 - 1;
        let v5: u64 = f111(v0.clone(), v1)?;
        let a357 = v3;
        let a358 = 1u64;
        if nat_eq(a357, a358) {
            let m359 = f65(v0.clone(), v1, v4, 0u64)?;
            match m359 {
                None => {
                    Ok(None::<(Adt2, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f111(v0.clone(), {
                        let (_, h360) = v6.clone();
                        h360
                    })?;
                    let a361 = v7;
                    let a362 = 44u64;
                    if nat_eq(a361, a362) {
                        let m366 = f65(v0, {
                            let a364 = {
                                let (_, h363) = v6;
                                h363
                            };
                            let a365 = 1u64;
                            nat_add(a364, a365)?
                        }, v4, 1u64)?;
                        match m366 {
                            None => {
                                Ok(None::<(Adt2, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt2::C9(std::rc::Rc::new({
                                    let (h367, _) = v6;
                                    h367
                                }), std::rc::Rc::new({
                                    let (h368, _) = v8;
                                    h368
                                })), {
                                    let (_, h369) = v8;
                                    h369
                                })))
                            }
                        }
                    } else {
                        let a370 = v7;
                        let a371 = 93u64;
                        if nat_eq(a370, a371) {
                            Ok(Some((Adt2::C9(std::rc::Rc::new({
                                let (h372, _) = v6;
                                h372
                            }), std::rc::Rc::new(Adt2::C8)), {
                                let a374 = {
                                    let (_, h373) = v6;
                                    h373
                                };
                                let a375 = 1u64;
                                nat_add(a374, a375)?
                            })))
                        } else {
                            Ok(None::<(Adt2, u64)>)
                        }
                    }
                }
            }
        } else {
            let a376 = v3;
            let a377 = 2u64;
            if nat_eq(a376, a377) {
                let m380 = f112(v0.clone(), {
                    let a378 = v1;
                    let a379 = 1u64;
                    nat_add(a378, a379)?
                })?;
                match m380 {
                    None => {
                        Ok(None::<(Adt2, u64)>)
                    }
                    Some(v9) => {
                        if f113({
                            let a381 = v5;
                            let a382 = 34u64;
                            nat_eq(a381, a382)
                        }, {
                            let a384 = f111(v0.clone(), {
                                let (h383, _) = v9.clone();
                                h383
                            })?;
                            let a385 = 58u64;
                            nat_eq(a384, a385)
                        }) {
                            let m389 = f65(v0.clone(), {
                                let a387 = {
                                    let (h386, _) = v9;
                                    h386
                                };
                                let a388 = 1u64;
                                nat_add(a387, a388)?
                            }, v4, 0u64)?;
                            match m389 {
                                None => {
                                    Ok(None::<(Adt2, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f111(v0.clone(), {
                                        let (_, h390) = v10.clone();
                                        h390
                                    })?;
                                    let a391 = v11;
                                    let a392 = 44u64;
                                    if nat_eq(a391, a392) {
                                        let m396 = f65(v0, {
                                            let a394 = {
                                                let (_, h393) = v10;
                                                h393
                                            };
                                            let a395 = 1u64;
                                            nat_add(a394, a395)?
                                        }, v4, 2u64)?;
                                        match m396 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt2::C10({
                                                    let (_, h397) = v9;
                                                    h397
                                                }, std::rc::Rc::new({
                                                    let (h398, _) = v10;
                                                    h398
                                                }), std::rc::Rc::new({
                                                    let (h399, _) = v12;
                                                    h399
                                                })), {
                                                    let (_, h400) = v12;
                                                    h400
                                                })))
                                            }
                                        }
                                    } else {
                                        let a401 = v11;
                                        let a402 = 125u64;
                                        if nat_eq(a401, a402) {
                                            Ok(Some((Adt2::C10({
                                                let (_, h403) = v9;
                                                h403
                                            }, std::rc::Rc::new({
                                                let (h404, _) = v10;
                                                h404
                                            }), std::rc::Rc::new(Adt2::C8)), {
                                                let a406 = {
                                                    let (_, h405) = v10;
                                                    h405
                                                };
                                                let a407 = 1u64;
                                                nat_add(a406, a407)?
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
                let a408 = v5;
                let a409 = 34u64;
                if nat_eq(a408, a409) {
                    let m412 = f112(v0, {
                        let a410 = v1;
                        let a411 = 1u64;
                        nat_add(a410, a411)?
                    })?;
                    match m412 {
                        None => {
                            Ok(None::<(Adt2, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt2::C5({
                                let (_, h413) = v13;
                                h413
                            }), {
                                let (h414, _) = v13;
                                h414
                            })))
                        }
                    }
                } else {
                    if f32(f46(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                        Ok(Some((Adt2::C0, {
                            let a415 = v1;
                            let a416 = 4u64;
                            nat_add(a415, a416)?
                        })))
                    } else {
                        if f32(f46(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                            Ok(Some((Adt2::C2(true), {
                                let a417 = v1;
                                let a418 = 4u64;
                                nat_add(a417, a418)?
                            })))
                        } else {
                            if f32(f46(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                                Ok(Some((Adt2::C2(false), {
                                    let a419 = v1;
                                    let a420 = 5u64;
                                    nat_add(a419, a420)?
                                })))
                            } else {
                                let a421 = v5;
                                let a422 = 91u64;
                                if nat_eq(a421, a422) {
                                    let a425 = f111(v0.clone(), {
                                        let a423 = v1;
                                        let a424 = 1u64;
                                        nat_add(a423, a424)?
                                    })?;
                                    let a426 = 93u64;
                                    if nat_eq(a425, a426) {
                                        Ok(Some((Adt2::C6(std::rc::Rc::new(Adt2::C8)), {
                                            let a427 = v1;
                                            let a428 = 2u64;
                                            nat_add(a427, a428)?
                                        })))
                                    } else {
                                        let m431 = f65(v0, {
                                            let a429 = v1;
                                            let a430 = 1u64;
                                            nat_add(a429, a430)?
                                        }, v4, 1u64)?;
                                        match m431 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v14) => {
                                                Ok(Some((Adt2::C6(std::rc::Rc::new({
                                                    let (h432, _) = v14;
                                                    h432
                                                })), {
                                                    let (_, h433) = v14;
                                                    h433
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let a434 = v5;
                                    let a435 = 123u64;
                                    if nat_eq(a434, a435) {
                                        let a438 = f111(v0.clone(), {
                                            let a436 = v1;
                                            let a437 = 1u64;
                                            nat_add(a436, a437)?
                                        })?;
                                        let a439 = 125u64;
                                        if nat_eq(a438, a439) {
                                            Ok(Some((Adt2::C7(std::rc::Rc::new(Adt2::C8)), {
                                                let a440 = v1;
                                                let a441 = 2u64;
                                                nat_add(a440, a441)?
                                            })))
                                        } else {
                                            let m444 = f65(v0, {
                                                let a442 = v1;
                                                let a443 = 1u64;
                                                nat_add(a442, a443)?
                                            }, v4, 2u64)?;
                                            match m444 {
                                                None => {
                                                    Ok(None::<(Adt2, u64)>)
                                                }
                                                Some(v15) => {
                                                    Ok(Some((Adt2::C7(std::rc::Rc::new({
                                                        let (h445, _) = v15;
                                                        h445
                                                    })), {
                                                        let (_, h446) = v15;
                                                        h446
                                                    })))
                                                }
                                            }
                                        }
                                    } else {
                                        let m447 = f114(v0, v1)?;
                                        match m447 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v16) => {
                                                Ok(Some(({
                                                    let (_, h448) = v16;
                                                    h448
                                                }, {
                                                    let (h449, _) = v16;
                                                    h449
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

pub fn f66(v0: Fn0, p454: u64, p455: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m453 = (p454, p455);
    {
        let mut m453 = m453;
        loop {
            let step = {
                let (v1, v2) = m453;
                let m450 = v1;
                if m450 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m450 - 1;
                    let m452 = {
                        let c451: Fn0 = v0.clone();
                        c451.apply(v2.clone())?
                    };
                    match m452 {
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
                Ok(next) => { m453 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f67(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f115(v1, v0, v2)
}

pub fn f68(v0: List<Bytes>) -> List<Bytes> {
    f116(Fn2::F117, List::<Bytes>::nil(), v0)
}

pub fn f69(v0: Fn1, p459: Option<Bytes>, p460: List<Bytes>) -> Option<Bytes> {
    let m458 = (p459, p460);
    {
        let mut m458 = m458;
        loop {
            let step = {
                let (v1, v2) = m458;
                let m456 = v2;
                match m456.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c457: Fn1 = v0.clone();
                            c457.apply(v1, v3)
                        }, v4))
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

pub fn f70(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m461 = v1;
    match m461 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a464 = {
                    let a462 = v3;
                    let a463 = v0;
                    append_bytes(a462, a463)
                };
                let a465 = v2;
                append_bytes(a464, a465)
            })
        }
    }
}

pub fn f71(p470: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m469 = p470;
    {
        let mut m469 = m469;
        loop {
            let step = {
                let v0 = m469;
                let m466 = v0;
                match m466.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h467, _) = v2;
                            h467
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h468) = v2;
                                h468
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m469 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f72(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Bytes>)> {
    let m474 = {
        let a472 = match v0 {
            Adt1::C0(h471, _, _, _) => {
                h471
            }
        };
        let a473 = f40(v1);
        index_list(a472, a473)
    };
    match m474 {
        None => {
            List::<(Bytes, Option<Bytes>)>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f73(p480: Adt2, v1: Bytes) -> Option<Adt2> {
    let m479 = p480;
    {
        let mut m479 = m479;
        loop {
            let step = {
                let v0 = m479;
                let m475 = v0;
                match m475 {
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
                    Adt2::C7(r476) => {
                        let v5 = (*r476).clone();
                        Ok::<Adt2, Option<Adt2>>(v5)
                    }
                    Adt2::C8 => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C9(_, _) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C10(v2, r477, r478) => {
                        let v3 = (*r477).clone();
                        let v4 = (*r478).clone();
                        if f32(v2, v1.clone()) {
                            Err::<Adt2, Option<Adt2>>(Some(v3))
                        } else {
                            Ok::<Adt2, Option<Adt2>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m479 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f74(v0: Option<Adt2>) -> Option<Bytes> {
    let m481 = v0;
    match m481 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m482 = v1;
            match m482 {
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

pub fn f75(v0: List<Adt2>) -> List<Bytes> {
    let m483 = v0;
    match m483.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m484 = f74(Some(v1));
            match m484 {
                None => {
                    f75(v2)
                }
                Some(v3) => {
                    List::cons(v3, f75(v2))
                }
            }
        }
    }
}

pub fn f76(v0: Option<Adt2>) -> List<Adt2> {
    let m485 = v0;
    match m485 {
        None => {
            List::<Adt2>::nil()
        }
        Some(v1) => {
            f118(v1)
        }
    }
}

pub fn f77(v0: Bytes) -> R<Bytes> {
    Ok(f119(f120(v0)?))
}

pub fn f78(v0: Adt1, v1: Bytes) -> R<Option<Adt2>> {
    f121(v0, v1)
}

pub fn f79(v0: Bytes) -> bool {
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

pub fn f80(v0: List<Bytes>) -> Option<Adt2> {
    let m486 = v0;
    match m486.uncons() {
        None => {
            None::<Adt2>
        }
        Some((v1, v2)) => {
            if f32(v1.clone(), f8(Str::lit("req"))) {
                let m487 = v2;
                match m487.uncons() {
                    None => {
                        None::<Adt2>
                    }
                    Some((v3, _)) => {
                        Some(Adt2::C10(f8(Str::lit("req")), std::rc::Rc::new(Adt2::C5(v3)), std::rc::Rc::new(Adt2::C8)))
                    }
                }
            } else {
                if f32(v1.clone(), f8(Str::lit("std"))) {
                    let m488 = v2;
                    match m488.uncons() {
                        None => {
                            None::<Adt2>
                        }
                        Some((v5, v6)) => {
                            Some(Adt2::C10(f8(Str::lit("std")), std::rc::Rc::new(Adt2::C7(std::rc::Rc::new(Adt2::C10(f8(Str::lit("authority")), std::rc::Rc::new(Adt2::C5(v5)), std::rc::Rc::new({
                                let m489 = v6;
                                match m489.uncons() {
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
                        let m490 = v2;
                        match m490.uncons() {
                            None => {
                                None::<Adt2>
                            }
                            Some((v9, v10)) => {
                                let m491 = v10;
                                match m491.uncons() {
                                    None => {
                                        None::<Adt2>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt2::C10(f8(Str::lit("der")), std::rc::Rc::new(Adt2::C7(std::rc::Rc::new(Adt2::C10(f8(Str::lit("check")), std::rc::Rc::new(Adt2::C5(v11)), std::rc::Rc::new(Adt2::C10(f8(Str::lit("inputs")), std::rc::Rc::new(Adt2::C6(std::rc::Rc::new(f86(v12)))), std::rc::Rc::new(Adt2::C10(f8(Str::lit("rule")), std::rc::Rc::new(Adt2::C5(v9)), std::rc::Rc::new(Adt2::C8))))))))), std::rc::Rc::new(Adt2::C8)))
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

pub fn f81(p494: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m493 = p494;
    {
        let mut m493 = m493;
        loop {
            let step = {
                let v0 = m493;
                let m492 = v0;
                match m492.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f122(v2.clone(), v1.clone()) {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v2))
                        } else {
                            Ok::<List<Bytes>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m493 = next; }
                Err(done) => break done,
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
            let m496 = f73(v1, f8(Str::lit("std")));
            match m496 {
                None => {
                    f82(v2)
                }
                Some(v3) => {
                    List::cons(f123(v3, f8(Str::lit("row"))), f82(v2))
                }
            }
        }
    }
}

pub fn f83(v0: List<Adt2>) -> List<Bytes> {
    let m497 = v0;
    match m497.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f124(f75(f76(f73(v1, f8(Str::lit("defers"))))), f83(v2))
        }
    }
}

pub fn f84(v0: Bytes) -> Adt12 {
    Adt12::C4(f7(List::cons(f8(Str::lit("index row ")), List::cons(v0, List::cons(f8(Str::lit(" is deferred by an owner decision, so nothing that cites it is admitted\u{a}")), List::<Bytes>::nil())))))
}

pub fn f85(v0: Adt1, v1: Bytes, v2: Adt2, v3: Bytes, v4: Bytes, v5: List<(Bytes, Bytes)>) -> R<Adt12> {
    let v6: Bytes = {
        let a498 = f8(Str::lit("Catalog."));
        let a499 = f125(v1.clone())?;
        append_bytes(a498, a499)
    };
    let v7: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f125(v1)?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
    Ok(Adt12::C3(f126(v5, List::cons((v7.clone(), v3), List::cons((f8(Str::lit("model/admissions.json")), f127(Adt2::C6(std::rc::Rc::new(f128(f129(f51(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v6.clone(), Adt2::C7(std::rc::Rc::new(Adt2::C10(f8(Str::lit("construct")), std::rc::Rc::new(Adt2::C5(v6.clone())), std::rc::Rc::new(v2)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f127(Adt2::C6(std::rc::Rc::new(f128(f129(f51(v0.clone(), f8(Str::lit("model/strata.json")))?, f8(Str::lit("path")), v7.clone(), Adt2::C7(std::rc::Rc::new(Adt2::C10(f8(Str::lit("path")), std::rc::Rc::new(Adt2::C5(v7.clone())), std::rc::Rc::new(Adt2::C10(f8(Str::lit("role")), std::rc::Rc::new(Adt2::C5(v4)), std::rc::Rc::new(Adt2::C10(f8(Str::lit("stratum")), std::rc::Rc::new(Adt2::C3(false, 0u64)), std::rc::Rc::new(Adt2::C8)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
        let a503 = f130(f90(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f46(v7.clone(), 4u64, {
            let a501 = {
                let a500 = v7;
                length_bytes(a500)
            };
            let a502 = 4u64;
            nat_sub(a501, a502)
        }))?;
        let a504 = f8(Str::lit("\u{a}"));
        append_bytes(a503, a504)
    }), List::cons(f131(v0, List::cons(v6.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v6, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f86(v0: List<Bytes>) -> Adt2 {
    let m505 = v0;
    match m505.uncons() {
        None => {
            Adt2::C8
        }
        Some((v1, v2)) => {
            Adt2::C9(std::rc::Rc::new(Adt2::C5(v1)), std::rc::Rc::new(f86(v2)))
        }
    }
}

pub fn f87(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\\begin{lexlean}{Catalog.")), List::cons(f125(v0)?, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(v1, List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil())))))))
}

pub fn f88(v0: Bytes) -> R<Bytes> {
    f132({
        let a507 = {
            let a506 = v0.clone();
            length_bytes(a506)
        };
        let a508 = 1u64;
        nat_add(a507, a508)?
    }, v0)
}

pub fn f89(v0: Bytes) -> R<Result<List<Adt2>, Bytes>> {
    let v1: Bytes = if f133(v0.clone(), Bytes::lit(&[0xef, 0xbb, 0xbf]), 0u64) {
        f46(v0.clone(), 3u64, {
            let a510 = {
                let a509 = v0;
                length_bytes(a509)
            };
            let a511 = 3u64;
            nat_sub(a510, a511)
        })
    } else {
        v0
    };
    let v2: Bytes = f134(v1)?;
    let v3: u64 = {
        let a512 = v2.clone();
        length_bytes(a512)
    };
    let v4: u64 = f135(v2.clone())?;
    let v5: u64 = f136(v2.clone(), 0u64)?;
    let a513 = v4;
    let a514 = v3;
    if nat_lt(a513, a514) {
        Ok(Err::<List<Adt2>, Bytes>(f137(Str::lit("the input is not a sequence of printable characters in UTF-8 (YAML 1.2.2 [1], section 5.2)"), v4)?))
    } else {
        let m517 = f138(v2.clone(), v5, {
            let a515 = v3;
            let a516 = 1u64;
            nat_add(a515, a516)?
        })?;
        match m517 {
            Ok(v7) => {
                let a518 = v3;
                let a519 = v7;
                if nat_le(a518, a519) {
                    Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
                } else {
                    f139(v2, v7, true, {
                        let a520 = v3;
                        let a521 = 1u64;
                        nat_add(a520, a521)?
                    })
                }
            }
            Err(v6) => {
                Ok(Err::<List<Adt2>, Bytes>(v6))
            }
        }
    }
}

pub fn f90(v0: Adt1, v1: Bytes) -> Bytes {
    let m522 = f71(f72(v0, v1.clone()), v1);
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

pub fn f91(v0: u64) -> R<Bytes> {
    f140(f141(), 10u64, v0)
}

pub fn f92(v0: List<Adt2>) -> R<u64> {
    let m524 = v0;
    match m524.uncons() {
        None => {
            Ok(0u64)
        }
        Some((_, v2)) => {
            let a525 = f92(v2)?;
            let a526 = 1u64;
            nat_add(a525, a526)
        }
    }
}

pub fn f93(v0: Bytes, v1: List<Bytes>) -> R<Result<(u64, Bytes), Bytes>> {
    let m527 = f142(v0)?;
    match m527 {
        Ok(v3) => {
            let m528 = f143(v1)?;
            match m528 {
                Ok(v5) => {
                    let v6: List<Adt13> = f144(v3.clone(), v5);
                    let v7: List<Adt16> = f145({
                        let a530 = {
                            let a529 = v6.clone();
                            length_list(a529)
                        };
                        let a531 = 1u64;
                        nat_add(a530, a531)?
                    }, f146(v6.clone(), v6.clone())?)?;
                    let v8: List<Bytes> = f147({
                        let a533 = {
                            let a532 = v7.clone();
                            length_list(a532)
                        };
                        let a534 = 1u64;
                        nat_add(a533, a534)?
                    }, f148(v3.clone()), List::<Bytes>::nil(), v7.clone());
                    let v9: List<(Bytes, (List<Bytes>, Bytes))> = f149(v7.clone(), v8);
                    let v10: (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>) = f150({
                        let a536 = {
                            let a535 = v9.clone();
                            length_list(a535)
                        };
                        let a537 = 1u64;
                        nat_add(a536, a537)?
                    }, v9, List::<Bytes>::nil(), List::<Bytes>::nil());
                    Ok(Ok::<(u64, Bytes), Bytes>(({
                        let a539 = {
                            let (h538, _) = v10.clone();
                            h538
                        };
                        length_list(a539)
                    }, f7(List::cons(f7(f151(v3, v7, v6)), List::cons(f7(f152({
                        let (_, h540) = v10;
                        h540
                    })), List::cons(f8(Str::lit("-- operations, constants and the other members that are not attributes are behavior, not data, and are not translated (Web IDL 2.5)\u{a}")), List::cons(f153(f154({
                        let (h541, _) = v10;
                        h541
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

pub fn f94(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m542 = v1;
    match m542.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f90(v0.clone(), v2), f94(v0, v3))
        }
    }
}

pub fn f95(v0: Bytes) -> R<Result<Adt17, Bytes>> {
    let v1: Bytes = if f133(v0.clone(), f8(Str::lit("\u{feff}")), 0u64) {
        f46(v0.clone(), 3u64, {
            let a544 = {
                let a543 = v0;
                length_bytes(a543)
            };
            let a545 = 3u64;
            nat_sub(a544, a545)
        })
    } else {
        v0
    };
    let v2: Bytes = f134(v1)?;
    let v3: u64 = {
        let a546 = v2.clone();
        length_bytes(a546)
    };
    let v4: u64 = f155(v2.clone())?;
    let a547 = v4;
    let a548 = v3;
    if nat_lt(a547, a548) {
        Ok(f156(f137(Str::lit("the input is not a sequence of Chars in UTF-8 (XML 1.0 [2], section 4.3.3)"), v4)?))
    } else {
        let m549 = f157(v2.clone())?;
        match m549 {
            Ok(v6) => {
                let m552 = f158(v2.clone(), v6, {
                    let a550 = v3;
                    let a551 = 2u64;
                    nat_add(a550, a551)?
                })?;
                match m552 {
                    Ok(v8) => {
                        if f159(v2.clone(), v8, Str::lit("<!DOCTYPE")) {
                            Ok(f156(f137(Str::lit("a document type declaration is not read; Verdant Studio reads documents that have none (XML 1.0 [28])"), v8)?))
                        } else {
                            let a553 = f160(v2.clone(), v8)?;
                            let a554 = 60u64;
                            if nat_eq(a553, a554) {
                                let m557 = f161(v2.clone(), v8, {
                                    let a555 = v3;
                                    let a556 = 2u64;
                                    nat_add(a555, a556)?
                                }, 0u64)?;
                                match m557 {
                                    Ok(v10) => {
                                        let m561 = f158(v2, {
                                            let (_, h558) = v10;
                                            h558
                                        }, {
                                            let a559 = v3;
                                            let a560 = 2u64;
                                            nat_add(a559, a560)?
                                        })?;
                                        match m561 {
                                            Ok(v12) => {
                                                let a562 = v12;
                                                let a563 = v3;
                                                if nat_eq(a562, a563) {
                                                    Ok(Ok::<Adt17, Bytes>({
                                                        let (h564, _) = v10;
                                                        h564
                                                    }))
                                                } else {
                                                    Ok(f156(f137(Str::lit("only comments, processing instructions and white space may follow the root element (XML 1.0 [1], [27])"), v12)?))
                                                }
                                            }
                                            Err(v11) => {
                                                Ok(f156(v11))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Ok(f156(v9))
                                    }
                                }
                            } else {
                                Ok(f156(f137(Str::lit("a document must have exactly one root element (XML 1.0 [1])"), v8)?))
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(f156(v7))
                    }
                }
            }
            Err(v5) => {
                Ok(f156(v5))
            }
        }
    }
}

pub fn f96(v0: Adt17) -> Bytes {
    let m565 = v0;
    match m565 {
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

pub fn f97(v0: Adt17) -> R<u64> {
    let m566 = v0;
    match m566 {
        Adt17::C0(_, _, r571) => {
            let v3 = (*r571).clone();
            let a567 = 1u64;
            let a568 = f97(v3)?;
            nat_add(a567, a568)
        }
        Adt17::C1(_, _, _) => {
            Ok(0u64)
        }
        Adt17::C2(_) => {
            Ok(0u64)
        }
        Adt17::C3(r572, r573) => {
            let v8 = (*r572).clone();
            let v9 = (*r573).clone();
            let a569 = f97(v8)?;
            let a570 = f97(v9)?;
            nat_add(a569, a570)
        }
        Adt17::C4 => {
            Ok(0u64)
        }
    }
}

pub fn f98(v0: Bytes) -> R<Result<(u64, Bytes), Bytes>> {
    let m574 = f95(v0)?;
    match m574 {
        Ok(v2) => {
            let m575 = v2;
            match m575 {
                Adt17::C0(v3, r587, r588) => {
                    let v4 = (*r587).clone();
                    let v5 = (*r588).clone();
                    let m576 = f163(v4.clone(), List::<(Bytes, Bytes)>::nil());
                    match m576 {
                        Ok(v7) => {
                            let m577 = f164(v7.clone(), v3)?;
                            match m577 {
                                Ok(v9) => {
                                    if f165(v9, Str::lit("schema")) {
                                        let v11: Bytes = {
                                            let m578 = f166(v4, f8(Str::lit("targetNamespace")));
                                            match m578 {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some(v10) => {
                                                    v10
                                                }
                                            }
                                        };
                                        let m579 = f167(v5, v7, v11)?;
                                        match m579 {
                                            Ok(v13) => {
                                                let v14: List<Bytes> = f168(v13.clone());
                                                let m580 = f169(v14.clone());
                                                match m580 {
                                                    None => {
                                                        let m581 = f170(v13.clone(), v14.clone());
                                                        match m581 {
                                                            None => {
                                                                let m585 = f171({
                                                                    let a583 = {
                                                                        let a582 = v14.clone();
                                                                        length_list(a582)
                                                                    };
                                                                    let a584 = 1u64;
                                                                    nat_add(a583, a584)?
                                                                }, v13, List::<Bytes>::nil(), List::<Bytes>::nil());
                                                                match m585 {
                                                                    Ok(v18) => {
                                                                        Ok(Ok::<(u64, Bytes), Bytes>(({
                                                                            let a586 = v14;
                                                                            length_list(a586)
                                                                        }, f153(f154(v18, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))))))
                                                                    }
                                                                    Err(v17) => {
                                                                        Ok(f162(v17))
                                                                    }
                                                                }
                                                            }
                                                            Some(v16) => {
                                                                Ok(f162(f7(List::cons(f8(Str::lit("the type ")), List::cons(v16, List::cons(f8(Str::lit(" is not a simple type of this schema document")), List::<Bytes>::nil()))))))
                                                            }
                                                        }
                                                    }
                                                    Some(v15) => {
                                                        Ok(f162(f7(List::cons(f8(Str::lit("two simple types derive the type name ")), List::cons(v15, List::<Bytes>::nil())))))
                                                    }
                                                }
                                            }
                                            Err(v12) => {
                                                Ok(f162(v12))
                                            }
                                        }
                                    } else {
                                        Ok(f162(f8(Str::lit("the root element must be xs:schema in the XML Schema namespace (XSD 1.1 Part 1, section 3.17.2)"))))
                                    }
                                }
                                Err(v8) => {
                                    Ok(f162(v8))
                                }
                            }
                        }
                        Err(v6) => {
                            Ok(f162(v6))
                        }
                    }
                }
                Adt17::C1(_, _, _) => {
                    Ok(f162(f8(Str::lit("the document has no root element"))))
                }
                Adt17::C2(_) => {
                    Ok(f162(f8(Str::lit("the document has no root element"))))
                }
                Adt17::C3(_, _) => {
                    Ok(f162(f8(Str::lit("the document has no root element"))))
                }
                Adt17::C4 => {
                    Ok(f162(f8(Str::lit("the document has no root element"))))
                }
            }
        }
        Err(v1) => {
            Ok(f162(f7(List::cons(f8(Str::lit("not well-formed XML: ")), List::cons(v1, List::<Bytes>::nil())))))
        }
    }
}

pub fn f99(v0: List<(Bytes, Option<Bytes>)>, v1: Bytes, v2: Bytes) -> R<Adt18> {
    let v3: List<(Bytes, Bytes)> = f172(v1)?;
    f173({
        let a590 = {
            let a589 = v3.clone();
            length_list(a589)
        };
        let a591 = 2u64;
        nat_add(a590, a591)?
    }, v0, v3, List::cons((v2, (f8(Str::lit("root")), Bytes::lit(&[]))), List::<(Bytes, (Bytes, Bytes))>::nil()), List::<Adt19>::nil(), List::<Bytes>::nil())
}

pub fn f100(v0: Adt1) -> List<(Bytes, Option<Bytes>)> {
    f174(match v0 {
        Adt1::C0(h592, _, _, _) => {
            h592
        }
    })
}

pub fn f101(v0: List<Adt2>) -> R<Result<Bytes, Bytes>> {
    let m593 = v0;
    match m593.uncons() {
        None => {
            Ok(Ok::<Bytes, Bytes>(Bytes::lit(&[])))
        }
        Some((v1, v2)) => {
            let m594 = f175(v1)?;
            match m594 {
                Ok(v4) => {
                    let m595 = f101(v2)?;
                    match m595 {
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

pub fn f102() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{9}"), List::cons(Str::lit("\u{a}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{d}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f103() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f104() -> List<Str> {
    List::cons(Str::lit(" "), List::cons(Str::lit("!"), List::cons(Str::lit("\""), List::cons(Str::lit("#"), List::cons(Str::lit("$"), List::cons(Str::lit("%"), List::cons(Str::lit("&"), List::cons(Str::lit("'"), List::cons(Str::lit("("), List::cons(Str::lit(")"), List::cons(Str::lit("*"), List::cons(Str::lit("+"), List::cons(Str::lit(","), List::cons(Str::lit("-"), List::cons(Str::lit("."), List::cons(Str::lit("/"), List::<Str>::nil()))))))))))))))))
}

pub fn f105() -> List<Str> {
    List::cons(Str::lit("0"), List::cons(Str::lit("1"), List::cons(Str::lit("2"), List::cons(Str::lit("3"), List::cons(Str::lit("4"), List::cons(Str::lit("5"), List::cons(Str::lit("6"), List::cons(Str::lit("7"), List::cons(Str::lit("8"), List::cons(Str::lit("9"), List::cons(Str::lit(":"), List::cons(Str::lit(";"), List::cons(Str::lit("<"), List::cons(Str::lit("="), List::cons(Str::lit(">"), List::cons(Str::lit("?"), List::<Str>::nil()))))))))))))))))
}

pub fn f106() -> List<Str> {
    List::cons(Str::lit("@"), List::cons(Str::lit("A"), List::cons(Str::lit("B"), List::cons(Str::lit("C"), List::cons(Str::lit("D"), List::cons(Str::lit("E"), List::cons(Str::lit("F"), List::cons(Str::lit("G"), List::cons(Str::lit("H"), List::cons(Str::lit("I"), List::cons(Str::lit("J"), List::cons(Str::lit("K"), List::cons(Str::lit("L"), List::cons(Str::lit("M"), List::cons(Str::lit("N"), List::cons(Str::lit("O"), List::<Str>::nil()))))))))))))))))
}

pub fn f107() -> List<Str> {
    List::cons(Str::lit("P"), List::cons(Str::lit("Q"), List::cons(Str::lit("R"), List::cons(Str::lit("S"), List::cons(Str::lit("T"), List::cons(Str::lit("U"), List::cons(Str::lit("V"), List::cons(Str::lit("W"), List::cons(Str::lit("X"), List::cons(Str::lit("Y"), List::cons(Str::lit("Z"), List::cons(Str::lit("["), List::cons(Str::lit("\\"), List::cons(Str::lit("]"), List::cons(Str::lit("^"), List::cons(Str::lit("_"), List::<Str>::nil()))))))))))))))))
}

pub fn f108() -> List<Str> {
    List::cons(Str::lit("`"), List::cons(Str::lit("a"), List::cons(Str::lit("b"), List::cons(Str::lit("c"), List::cons(Str::lit("d"), List::cons(Str::lit("e"), List::cons(Str::lit("f"), List::cons(Str::lit("g"), List::cons(Str::lit("h"), List::cons(Str::lit("i"), List::cons(Str::lit("j"), List::cons(Str::lit("k"), List::cons(Str::lit("l"), List::cons(Str::lit("m"), List::cons(Str::lit("n"), List::cons(Str::lit("o"), List::<Str>::nil()))))))))))))))))
}

pub fn f109() -> List<Str> {
    List::cons(Str::lit("p"), List::cons(Str::lit("q"), List::cons(Str::lit("r"), List::cons(Str::lit("s"), List::cons(Str::lit("t"), List::cons(Str::lit("u"), List::cons(Str::lit("v"), List::cons(Str::lit("w"), List::cons(Str::lit("x"), List::cons(Str::lit("y"), List::cons(Str::lit("z"), List::cons(Str::lit("{"), List::cons(Str::lit("|"), List::cons(Str::lit("}"), List::cons(Str::lit("~"), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f110(v0: Bytes, p623: u64, p624: u64, p625: bool, p626: bool, p627: u64, p628: Bytes) -> R<Bytes> {
    let m622 = (p623, (p624, (p625, (p626, (p627, p628)))));
    {
        let mut m622 = m622;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, (v5, v6))))) = m622;
                let m596 = v2;
                if m596 == 0 {
                    Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f176(v0.clone(), v5, v1, v6)))
                } else {
                    let v7 = m596 - 1;
                    let m599 = {
                        let a597 = v0.clone();
                        let a598 = v1;
                        index_bytes(a597, a598)
                    };
                    match m599 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f176(v0.clone(), v5, v1, v6)))
                        }
                        Some(v8) => {
                            let v9: u64 = f62(v8)?;
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                    let a600 = v1;
                                    let a601 = 1u64;
                                    nat_add(a600, a601)?
                                }, (v7, (if v4 {
                                    true
                                } else {
                                    let a602 = v9;
                                    let a603 = 34u64;
                                    !nat_eq(a602, a603)
                                }, (if v4 {
                                    false
                                } else {
                                    let a604 = v9;
                                    let a605 = 92u64;
                                    nat_eq(a604, a605)
                                }, (v5, v6))))))
                            } else {
                                let a606 = v9;
                                let a607 = 32u64;
                                if if if if nat_eq(a606, a607) {
                                    true
                                } else {
                                    let a608 = v9;
                                    let a609 = 9u64;
                                    nat_eq(a608, a609)
                                } {
                                    true
                                } else {
                                    let a610 = v9;
                                    let a611 = 10u64;
                                    nat_eq(a610, a611)
                                } {
                                    true
                                } else {
                                    let a612 = v9;
                                    let a613 = 13u64;
                                    nat_eq(a612, a613)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a614 = v1;
                                        let a615 = 1u64;
                                        nat_add(a614, a615)?
                                    }, (v7, (false, (false, ({
                                        let a616 = v1;
                                        let a617 = 1u64;
                                        nat_add(a616, a617)?
                                    }, f176(v0.clone(), v5, v1, v6)))))))
                                } else {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a618 = v1;
                                        let a619 = 1u64;
                                        nat_add(a618, a619)?
                                    }, (v7, ({
                                        let a620 = v9;
                                        let a621 = 34u64;
                                        nat_eq(a620, a621)
                                    }, (false, (v5, v6))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m622 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f111(v0: Bytes, v1: u64) -> R<u64> {
    let m631 = {
        let a629 = v0;
        let a630 = v1;
        index_bytes(a629, a630)
    };
    match m631 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a676 = {
                let a652 = {
                    let a640 = {
                        let a634 = {
                            let a632 = v3;
                            let a633 = 128u8;
                            fixed_u8::bit_and(a632, a633)
                        };
                        let a635 = 128u8;
                        if equal(a634, a635) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a641 = {
                        let a638 = {
                            let a636 = v3;
                            let a637 = 64u8;
                            fixed_u8::bit_and(a636, a637)
                        };
                        let a639 = 64u8;
                        if equal(a638, a639) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a640, a641)?
                };
                let a653 = {
                    let a650 = {
                        let a644 = {
                            let a642 = v3;
                            let a643 = 32u8;
                            fixed_u8::bit_and(a642, a643)
                        };
                        let a645 = 32u8;
                        if equal(a644, a645) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a651 = {
                        let a648 = {
                            let a646 = v3;
                            let a647 = 16u8;
                            fixed_u8::bit_and(a646, a647)
                        };
                        let a649 = 16u8;
                        if equal(a648, a649) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a650, a651)?
                };
                nat_add(a652, a653)?
            };
            let a677 = {
                let a674 = {
                    let a662 = {
                        let a656 = {
                            let a654 = v3;
                            let a655 = 8u8;
                            fixed_u8::bit_and(a654, a655)
                        };
                        let a657 = 8u8;
                        if equal(a656, a657) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a663 = {
                        let a660 = {
                            let a658 = v3;
                            let a659 = 4u8;
                            fixed_u8::bit_and(a658, a659)
                        };
                        let a661 = 4u8;
                        if equal(a660, a661) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a662, a663)?
                };
                let a675 = {
                    let a672 = {
                        let a666 = {
                            let a664 = v3;
                            let a665 = 2u8;
                            fixed_u8::bit_and(a664, a665)
                        };
                        let a667 = 2u8;
                        if equal(a666, a667) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a673 = {
                        let a670 = {
                            let a668 = v3;
                            let a669 = 1u8;
                            fixed_u8::bit_and(a668, a669)
                        };
                        let a671 = 1u8;
                        if equal(a670, a671) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a672, a673)?
                };
                nat_add(a674, a675)?
            };
            nat_add(a676, a677)
        }
    }
}

pub fn f112(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = f177(v0.clone(), v1, {
        let a679 = {
            let a678 = v0.clone();
            length_bytes(a678)
        };
        let a680 = 1u64;
        nat_add(a679, a680)?
    })?;
    let a681 = f111(v0.clone(), v2)?;
    let a682 = 34u64;
    if nat_eq(a681, a682) {
        Ok(Some(({
            let a683 = v2;
            let a684 = 1u64;
            nat_add(a683, a684)?
        }, f46(v0, v1, {
            let a685 = v2;
            let a686 = v1;
            nat_sub(a685, a686)
        }))))
    } else {
        f178(v0.clone(), v1, {
            let a688 = {
                let a687 = v0;
                length_bytes(a687)
            };
            let a689 = 1u64;
            nat_add(a688, a689)?
        }, Bytes::lit(&[]))
    }
}

pub fn f113(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f114(v0: Bytes, v1: u64) -> R<Option<(u64, Adt2)>> {
    let v2: bool = {
        let a690 = f111(v0.clone(), v1)?;
        let a691 = 45u64;
        nat_eq(a690, a691)
    };
    let v3: u64 = if v2 {
        let a692 = v1;
        let a693 = 1u64;
        nat_add(a692, a693)?
    } else {
        v1
    };
    let v4: u64 = f179(v0.clone(), v3)?;
    let v5: u64 = {
        let a694 = v4;
        let a695 = v3;
        nat_sub(a694, a695)
    };
    let v6: bool = {
        let a696 = f111(v0.clone(), v4)?;
        let a697 = 46u64;
        nat_eq(a696, a697)
    };
    let v7: u64 = if v6 {
        let a698 = v4;
        let a699 = 1u64;
        nat_add(a698, a699)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f179(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f111(v0.clone(), v8)?;
    let v10: bool = {
        let a700 = v9;
        let a701 = 101u64;
        if nat_eq(a700, a701) {
            true
        } else {
            let a702 = v9;
            let a703 = 69u64;
            nat_eq(a702, a703)
        }
    };
    let v11: u64 = f111(v0.clone(), {
        let a704 = v8;
        let a705 = 1u64;
        nat_add(a704, a705)?
    })?;
    let v12: bool = f113(v10, f180({
        let a706 = v11;
        let a707 = 43u64;
        nat_eq(a706, a707)
    }, {
        let a708 = v11;
        let a709 = 45u64;
        nat_eq(a708, a709)
    }));
    let v13: u64 = if v10 {
        if v12 {
            let a710 = v8;
            let a711 = 2u64;
            nat_add(a710, a711)?
        } else {
            let a712 = v8;
            let a713 = 1u64;
            nat_add(a712, a713)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f179(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f46(v0.clone(), v13, {
        let a714 = v14;
        let a715 = v13;
        nat_sub(a714, a715)
    });
    let v16: u64 = f181(v15.clone())?;
    let v17: Bytes = f46(v15.clone(), v16, {
        let a717 = {
            let a716 = v15;
            length_bytes(a716)
        };
        let a718 = v16;
        nat_sub(a717, a718)
    });
    let a719 = v5;
    let a720 = 0u64;
    if if nat_eq(a719, a720) {
        true
    } else {
        if f113({
            let a721 = 1u64;
            let a722 = v5;
            nat_lt(a721, a722)
        }, {
            let a723 = f111(v0.clone(), v3)?;
            let a724 = 48u64;
            nat_eq(a723, a724)
        }) {
            true
        } else {
            if f113(v6, {
                let a725 = v8;
                let a726 = v7;
                nat_eq(a725, a726)
            }) {
                true
            } else {
                f180(if v10 {
                    let a727 = v14;
                    let a728 = v13;
                    nat_eq(a727, a728)
                } else {
                    false
                }, {
                    let a730 = 18u64;
                    let a731 = {
                        let a729 = v17.clone();
                        length_bytes(a729)
                    };
                    nat_lt(a730, a731)
                })
            }
        }
    } {
        Ok(None::<(u64, Adt2)>)
    } else {
        let m735 = {
            let a733 = {
                let a732 = v17.clone();
                length_bytes(a732)
            };
            let a734 = 0u64;
            if nat_eq(a733, a734) {
                Some(0u64)
            } else {
                f182(v17)?
            }
        };
        match m735 {
            None => {
                Ok(None::<(u64, Adt2)>)
            }
            Some(v18) => {
                let m740 = f183(f46(v0.clone(), v3, v5), f46(v0, v7, {
                    let a736 = v8;
                    let a737 = v7;
                    nat_sub(a736, a737)
                }), f113(v12, {
                    let a738 = v11;
                    let a739 = 45u64;
                    nat_eq(a738, a739)
                }), v18)?;
                match m740 {
                    None => {
                        Ok(None::<(u64, Adt2)>)
                    }
                    Some(v19) => {
                        let a743 = 19u64;
                        let a744 = {
                            let a742 = {
                                let (h741, _) = v19.clone();
                                h741
                            };
                            length_bytes(a742)
                        };
                        if nat_lt(a743, a744) {
                            Ok(None::<(u64, Adt2)>)
                        } else {
                            let m746 = f182({
                                let (h745, _) = v19.clone();
                                h745
                            })?;
                            match m746 {
                                None => {
                                    Ok(None::<(u64, Adt2)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f113(v2, {
                                        let a747 = 0u64;
                                        let a748 = v20;
                                        nat_lt(a747, a748)
                                    });
                                    if f184(v21, v20) {
                                        Ok(Some((v14, {
                                            let a750 = {
                                                let (_, h749) = v19.clone();
                                                h749
                                            };
                                            let a751 = 0u64;
                                            if nat_eq(a750, a751) {
                                                Adt2::C3(v21, v20)
                                            } else {
                                                Adt2::C4(v21, v20, {
                                                    let (_, h752) = v19;
                                                    h752
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

pub fn f115(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h753, _) = v2.clone();
        h753
    };
    let v4: u64 = {
        let (h754, _) = {
            let (_, h755) = v2.clone();
            h755
        };
        h754
    };
    let v5: List<Bytes> = {
        let (_, h756) = {
            let (_, h757) = v2;
            h757
        };
        h756
    };
    let m758 = f185(v0.clone(), v3);
    match m758 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a759 = v6;
            let a760 = v1;
            if equal(a759, a760) {
                Ok(Some(({
                    let a761 = v3;
                    let a762 = 1u64;
                    nat_add(a761, a762)?
                }, ({
                    let a763 = v3;
                    let a764 = 1u64;
                    nat_add(a763, a764)?
                }, List::cons(f46(v0, v4, {
                    let a765 = v3;
                    let a766 = v4;
                    nat_sub(a765, a766)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a767 = v3;
                    let a768 = 1u64;
                    nat_add(a767, a768)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f116(v0: Fn2, p772: List<Bytes>, p773: List<Bytes>) -> List<Bytes> {
    let m771 = (p772, p773);
    {
        let mut m771 = m771;
        loop {
            let step = {
                let (v1, v2) = m771;
                let m769 = v2;
                match m769.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c770: Fn2 = v0.clone();
                            c770.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m771 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f117(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f118(v0: Adt2) -> List<Adt2> {
    let m774 = v0;
    match m774 {
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
        Adt2::C6(r775) => {
            let v3 = (*r775).clone();
            f118(v3)
        }
        Adt2::C7(_) => {
            List::<Adt2>::nil()
        }
        Adt2::C8 => {
            List::<Adt2>::nil()
        }
        Adt2::C9(r776, r777) => {
            let v1 = (*r776).clone();
            let v2 = (*r777).clone();
            List::cons(v1, f118(v2))
        }
        Adt2::C10(_, _, _) => {
            List::<Adt2>::nil()
        }
    }
}

pub fn f119(v0: Adt20) -> Bytes {
    let a798 = f186(match v0.clone() {
        Adt20::C0(h778, _, _, _, _, _, _, _) => {
            h778
        }
    });
    let a799 = {
        let a796 = f186(match v0.clone() {
            Adt20::C0(_, h779, _, _, _, _, _, _) => {
                h779
            }
        });
        let a797 = {
            let a794 = f186(match v0.clone() {
                Adt20::C0(_, _, h780, _, _, _, _, _) => {
                    h780
                }
            });
            let a795 = {
                let a792 = f186(match v0.clone() {
                    Adt20::C0(_, _, _, h781, _, _, _, _) => {
                        h781
                    }
                });
                let a793 = {
                    let a790 = f186(match v0.clone() {
                        Adt20::C0(_, _, _, _, h782, _, _, _) => {
                            h782
                        }
                    });
                    let a791 = {
                        let a788 = f186(match v0.clone() {
                            Adt20::C0(_, _, _, _, _, h783, _, _) => {
                                h783
                            }
                        });
                        let a789 = {
                            let a786 = f186(match v0.clone() {
                                Adt20::C0(_, _, _, _, _, _, h784, _) => {
                                    h784
                                }
                            });
                            let a787 = f186(match v0 {
                                Adt20::C0(_, _, _, _, _, _, _, h785) => {
                                    h785
                                }
                            });
                            append_bytes(a786, a787)
                        };
                        append_bytes(a788, a789)
                    };
                    append_bytes(a790, a791)
                };
                append_bytes(a792, a793)
            };
            append_bytes(a794, a795)
        };
        append_bytes(a796, a797)
    };
    append_bytes(a798, a799)
}

pub fn f120(v0: Bytes) -> R<Adt20> {
    let v1: u64 = {
        let a800 = v0.clone();
        length_bytes(a800)
    };
    let v2: u64 = {
        let a806 = {
            let a803 = {
                let a801 = v1;
                let a802 = 72u64;
                nat_add(a801, a802)?
            };
            let a804 = 64u64;
            let a805 = 0u64;
            nat_quot(a803, a804, a805)
        };
        let a807 = 64u64;
        nat_mul(a806, a807)?
    };
    let (_, h808) = f187(Fn3::F188(v0, v1, v2), {
        let a809 = v2;
        let a810 = 64u64;
        let a811 = 0u64;
        nat_quot(a809, a810, a811)
    }, (0u64, f189()))?;
    Ok(h808)
}

pub fn f121(v0: Adt1, v1: Bytes) -> R<Option<Adt2>> {
    let m812 = f190(f191(v0.clone(), v1.clone()), v1.clone());
    match m812 {
        None => {
            f43(f90(v0, v1))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f122(v0: Bytes, p815: List<Bytes>) -> bool {
    let m814 = p815;
    {
        let mut m814 = m814;
        loop {
            let step = {
                let v1 = m814;
                let m813 = v1;
                match m813.uncons() {
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
                Ok(next) => { m814 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f123(v0: Adt2, v1: Bytes) -> Bytes {
    let m816 = f74(f73(v0, v1));
    match m816 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f124(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m817 = v0;
    match m817.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f124(v3, v1))
        }
    }
}

pub fn f125(v0: Bytes) -> R<Bytes> {
    f192(v0.clone(), 0u64, {
        let a818 = v0;
        length_bytes(a818)
    }, true, Bytes::lit(&[]))
}

pub fn f126(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m819 = v0;
    match m819.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f126(v3, v1))
        }
    }
}

pub fn f127(v0: Adt2) -> R<Bytes> {
    let a820 = f193(v0)?;
    let a821 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a820, a821))
}

pub fn f128(v0: List<Adt2>) -> Adt2 {
    let m822 = v0;
    match m822.uncons() {
        None => {
            Adt2::C8
        }
        Some((v1, v2)) => {
            Adt2::C9(std::rc::Rc::new(v1), std::rc::Rc::new(f128(v2)))
        }
    }
}

pub fn f129(v0: List<Adt2>, v1: Bytes, v2: Bytes, v3: Adt2) -> List<Adt2> {
    let m823 = v0;
    match m823.uncons() {
        None => {
            List::cons(v3, List::<Adt2>::nil())
        }
        Some((v4, v5)) => {
            if f32(f123(v4.clone(), v1.clone()), v2.clone()) {
                List::cons(v3, f194(v5, v1, v2))
            } else {
                List::cons(v4, f129(v5, v1, v2, v3))
            }
        }
    }
}

pub fn f130(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f153(f195(f196(v0)?, v1)?, f8(Str::lit("\u{a}"))))
}

pub fn f131(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f127(Adt2::C6(std::rc::Rc::new(f86(f197(f124(v1, f75(f51(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f132(p834: u64, p835: Bytes) -> R<Bytes> {
    let m833 = (p834, p835);
    {
        let mut m833 = m833;
        loop {
            let step = {
                let (v0, v1) = m833;
                let m824 = v0;
                if m824 == 0 {
                    Err::<(u64, Bytes), R<Bytes>>(Ok(v1))
                } else {
                    let v2 = m824 - 1;
                    if f53(v1.clone(), f8(Str::lit("-- "))) {
                        let m825 = f198(v1.clone(), f8(Str::lit("\u{a}")))?;
                        match m825 {
                            None => {
                                Err::<(u64, Bytes), R<Bytes>>(Ok(v1))
                            }
                            Some(v3) => {
                                Ok::<(u64, Bytes), R<Bytes>>((v2, f46(v1.clone(), {
                                    let a826 = v3;
                                    let a827 = 1u64;
                                    nat_add(a826, a827)?
                                }, {
                                    let a831 = {
                                        let a828 = v1;
                                        length_bytes(a828)
                                    };
                                    let a832 = {
                                        let a829 = v3;
                                        let a830 = 1u64;
                                        nat_add(a829, a830)?
                                    };
                                    nat_sub(a831, a832)
                                })))
                            }
                        }
                    } else {
                        Err::<(u64, Bytes), R<Bytes>>(Ok(v1))
                    }
                }
            };
            match step {
                Ok(next) => { m833 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f133(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m840 = {
        let a837 = v0;
        let a838 = v2;
        let a839 = {
            let a836 = v1.clone();
            length_bytes(a836)
        };
        slice_bytes(a837, a838, a839)
    };
    match m840 {
        None => {
            false
        }
        Some(v3) => {
            let a841 = v3;
            let a842 = v1;
            equal(a841, a842)
        }
    }
}

pub fn f134(v0: Bytes) -> R<Bytes> {
    let (_, h843) = {
        let (h844, _) = f199(Fn4::F200(v0.clone()), {
            let a846 = {
                let a845 = v0;
                length_bytes(a845)
            };
            let a847 = 1u64;
            nat_add(a846, a847)?
        }, (0u64, Bytes::lit(&[])))?;
        h844
    };
    Ok(h843)
}

pub fn f135(v0: Bytes) -> R<u64> {
    let (h848, _) = f201(Fn5::F202(v0.clone()), {
        let a850 = {
            let a849 = v0;
            length_bytes(a849)
        };
        let a851 = 1u64;
        nat_add(a850, a851)?
    }, 0u64)?;
    Ok(h848)
}

pub fn f136(v0: Bytes, v1: u64) -> R<u64> {
    let (h852, _) = f201(Fn5::F203(v0.clone()), {
        let a854 = {
            let a853 = v0;
            length_bytes(a853)
        };
        let a855 = 1u64;
        nat_add(a854, a855)?
    }, v1)?;
    Ok(h852)
}

pub fn f137(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a856 = v0;
        utf8_encode(a856)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f91(v1)?, List::<Bytes>::nil())))))
}

pub fn f138(v0: Bytes, p865: u64, p866: u64) -> R<Result<u64, Bytes>> {
    let m864 = (p865, p866);
    {
        let mut m864 = m864;
        loop {
            let step = {
                let (v1, v2) = m864;
                let m857 = v2;
                if m857 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Ok::<u64, Bytes>(v1)))
                } else {
                    let v3 = m857 - 1;
                    let a859 = v1;
                    let a860 = {
                        let a858 = v0.clone();
                        length_bytes(a858)
                    };
                    if if nat_lt(a859, a860) {
                        if f159(v0.clone(), v1, Str::lit("...")) {
                            f204(v0.clone(), v1)?
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        let m863 = f205(v0.clone(), {
                            let a861 = v1;
                            let a862 = 3u64;
                            nat_add(a861, a862)?
                        })?;
                        match m863 {
                            Ok(v5) => {
                                Ok::<(u64, u64), R<Result<u64, Bytes>>>((f136(v0.clone(), v5)?, v3))
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
                Ok(next) => { m864 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f139(v0: Bytes, v1: u64, v2: bool, v3: u64) -> R<Result<List<Adt2>, Bytes>> {
    let m867 = v3;
    if m867 == 0 {
        Ok(Err::<List<Adt2>, Bytes>(f137(Str::lit("the stream holds more documents than it is long"), v1)?))
    } else {
        let v4 = m867 - 1;
        let v5: u64 = {
            let a868 = v0.clone();
            length_bytes(a868)
        };
        let a869 = f206(v0.clone(), v1)?;
        let a870 = 37u64;
        if nat_eq(a869, a870) {
            Ok(Err::<List<Adt2>, Bytes>(f137(Str::lit("a directive is not read (YAML 1.2.2 [82])"), v1)?))
        } else {
            let m877 = if if f159(v0.clone(), v1, Str::lit("---")) {
                f204(v0.clone(), v1)?
            } else {
                false
            } {
                f207(v0.clone(), {
                    let a871 = v1;
                    let a872 = 3u64;
                    nat_add(a871, a872)?
                }, 0u64, {
                    let a873 = v5;
                    let a874 = 2u64;
                    nat_add(a873, a874)?
                }, 1u64, false, 0u64, List::<Bytes>::nil(), Adt21::C0(0u64))?
            } else {
                if v2 {
                    f207(v0.clone(), v1, 0u64, {
                        let a875 = v5;
                        let a876 = 2u64;
                        nat_add(a875, a876)?
                    }, 0u64, false, 0u64, List::<Bytes>::nil(), Adt21::C0(0u64))?
                } else {
                    Adt22::C1(f137(Str::lit("only comments, a document end marker or a document start marker may follow a document (YAML 1.2.2 [207], [211])"), v1)?)
                }
            };
            match m877 {
                Adt22::C0(v7, v8) => {
                    let v9: u64 = f136(v0.clone(), v8)?;
                    let m880 = f138(v0.clone(), v9, {
                        let a878 = v5;
                        let a879 = 1u64;
                        nat_add(a878, a879)?
                    })?;
                    match m880 {
                        Ok(v11) => {
                            let a881 = v5;
                            let a882 = v11;
                            if nat_le(a881, a882) {
                                Ok(Ok::<List<Adt2>, Bytes>(List::cons(v7, List::<Adt2>::nil())))
                            } else {
                                let m885 = f139(v0, v11, f208({
                                    let a883 = v11;
                                    let a884 = v9;
                                    nat_eq(a883, a884)
                                }), v4)?;
                                match m885 {
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

pub fn f140(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a886 = v2;
    let a887 = 0u64;
    if nat_eq(a886, a887) {
        Ok(f209(v0, 0u64))
    } else {
        let (_, h888) = {
            let (h889, _) = f199(Fn4::F210(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h889
        };
        Ok(h888)
    }
}

pub fn f141() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f142(v0: Bytes) -> R<Result<List<Adt13>, Bytes>> {
    let m890 = f211(v0)?;
    match m890 {
        Ok(v2) => {
            f212({
                let a892 = {
                    let a891 = v2.clone();
                    length_list(a891)
                };
                let a893 = 1u64;
                nat_add(a892, a893)?
            }, v2, List::<Adt13>::nil())
        }
        Err(v1) => {
            Ok(Err::<List<Adt13>, Bytes>(v1))
        }
    }
}

pub fn f143(v0: List<Bytes>) -> R<Result<List<Adt13>, Bytes>> {
    let m894 = v0;
    match m894.uncons() {
        None => {
            Ok(Ok::<List<Adt13>, Bytes>(List::<Adt13>::nil()))
        }
        Some((v1, v2)) => {
            let m895 = f142(v1)?;
            match m895 {
                Ok(v4) => {
                    let m896 = f143(v2)?;
                    match m896 {
                        Ok(v6) => {
                            Ok(Ok::<List<Adt13>, Bytes>(f144(v4, v6)))
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

pub fn f144(v0: List<Adt13>, v1: List<Adt13>) -> List<Adt13> {
    let m897 = v0;
    match m897.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f144(v3, v1))
        }
    }
}

pub fn f145(p902: u64, p903: List<Adt16>) -> R<List<Adt16>> {
    let m901 = (p902, p903);
    {
        let mut m901 = m901;
        loop {
            let step = {
                let (v0, v1) = m901;
                let m898 = v0;
                if m898 == 0 {
                    Err::<(u64, List<Adt16>), R<List<Adt16>>>(Ok(v1))
                } else {
                    let v2 = m898 - 1;
                    let v3: List<Adt16> = f213(v1.clone(), v1.clone());
                    let a899 = f214(v3.clone())?;
                    let a900 = f214(v1.clone())?;
                    if nat_eq(a899, a900) {
                        Err::<(u64, List<Adt16>), R<List<Adt16>>>(Ok(v1))
                    } else {
                        Ok::<(u64, List<Adt16>), R<List<Adt16>>>((v2, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m901 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f146(v0: List<Adt13>, v1: List<Adt13>) -> R<List<Adt16>> {
    let m904 = v0;
    match m904.uncons() {
        None => {
            Ok(List::<Adt16>::nil())
        }
        Some((v2, v3)) => {
            if f215(f216(v2.clone())) {
                let v6: Adt16 = {
                    let m905 = f217(v1.clone(), v2.clone())?;
                    match m905 {
                        Ok(v5) => {
                            Adt16::C0(f218(v2), Bytes::lit(&[]), {
                                let (h906, _) = v5;
                                h906
                            }, {
                                let (_, h907) = v5;
                                h907
                            })
                        }
                        Err(v4) => {
                            Adt16::C0(f218(v2), v4, Bytes::lit(&[]), List::<Bytes>::nil())
                        }
                    }
                };
                Ok(List::cons(v6, f146(v3, v1)?))
            } else {
                f146(v3, v1)
            }
        }
    }
}

pub fn f147(p912: u64, p913: List<Bytes>, p914: List<Bytes>, v3: List<Adt16>) -> List<Bytes> {
    let m911 = (p912, (p913, p914));
    {
        let mut m911 = m911;
        loop {
            let step = {
                let (v0, (v1, v2)) = m911;
                let m908 = v0;
                if m908 == 0 {
                    Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v2)
                } else {
                    let v4 = m908 - 1;
                    let m909 = v1;
                    match m909.uncons() {
                        None => {
                            Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v2)
                        }
                        Some((v5, v6)) => {
                            if f122(v5.clone(), v2.clone()) {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v2)))
                            } else {
                                let m910 = f219(v3.clone(), v5.clone());
                                match m910 {
                                    None => {
                                        Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v2)))
                                    }
                                    Some(v7) => {
                                        if f32(f220(v7.clone()), Bytes::lit(&[])) {
                                            Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (f221(f222(v7), v6), List::cons(v5, v2))))
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
                Ok(next) => { m911 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f148(v0: List<Adt13>) -> List<Bytes> {
    let m915 = v0;
    match m915.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let v3: u64 = f216(v1.clone());
            let a916 = v3;
            let a917 = 0u64;
            if if nat_eq(a916, a917) {
                true
            } else {
                let a918 = v3;
                let a919 = 2u64;
                if nat_eq(a918, a919) {
                    true
                } else {
                    let a920 = v3;
                    let a921 = 3u64;
                    if nat_eq(a920, a921) {
                        true
                    } else {
                        let a922 = v3;
                        let a923 = 4u64;
                        if nat_eq(a922, a923) {
                            true
                        } else {
                            let a924 = v3;
                            let a925 = 5u64;
                            if nat_eq(a924, a925) {
                                true
                            } else {
                                let a926 = v3;
                                let a927 = 7u64;
                                if nat_eq(a926, a927) {
                                    true
                                } else {
                                    let a928 = v3;
                                    let a929 = 8u64;
                                    nat_eq(a928, a929)
                                }
                            }
                        }
                    }
                }
            } {
                List::cons(f218(v1), f148(v2))
            } else {
                f148(v2)
            }
        }
    }
}

pub fn f149(v0: List<Adt16>, v1: List<Bytes>) -> List<(Bytes, (List<Bytes>, Bytes))> {
    let m930 = v0;
    match m930.uncons() {
        None => {
            List::<(Bytes, (List<Bytes>, Bytes))>::nil()
        }
        Some((v2, v3)) => {
            if if f122(f223(v2.clone()), v1.clone()) {
                f32(f220(v2.clone()), Bytes::lit(&[]))
            } else {
                false
            } {
                List::cons((f223(v2.clone()), (f222(v2.clone()), f224(v2))), f149(v3, v1))
            } else {
                f149(v3, v1)
            }
        }
    }
}

pub fn f150(p939: u64, p940: List<(Bytes, (List<Bytes>, Bytes))>, p941: List<Bytes>, p942: List<Bytes>) -> (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>) {
    let m938 = (p939, (p940, (p941, p942)));
    {
        let mut m938 = m938;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m938;
                let m931 = v0;
                if m931 == 0 {
                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v3, v1))
                } else {
                    let v4 = m931 - 1;
                    let m932 = f225(v1.clone(), v2.clone());
                    match m932 {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v3, v1))
                        }
                        Some(v5) => {
                            Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v4, ({
                                let (_, h933) = v5.clone();
                                h933
                            }, (List::cons({
                                let (h934, _) = {
                                    let (h935, _) = v5.clone();
                                    h935
                                };
                                h934
                            }, v2), List::cons({
                                let (_, h936) = {
                                    let (h937, _) = v5;
                                    h937
                                };
                                h936
                            }, v3)))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m938 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f151(v0: List<Adt13>, v1: List<Adt16>, v2: List<Adt13>) -> List<Bytes> {
    let m943 = v0;
    match m943.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            let v5: u64 = f216(v3.clone());
            let a944 = v5;
            let a945 = 5u64;
            if if nat_eq(a944, a945) {
                true
            } else {
                let a946 = v5;
                let a947 = 7u64;
                nat_eq(a946, a947)
            } {
                let m950 = f226(v2.clone(), {
                    let a948 = v5;
                    let a949 = 5u64;
                    if nat_eq(a948, a949) {
                        0u64
                    } else {
                        2u64
                    }
                }, f218(v3.clone()));
                match m950 {
                    None => {
                        List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f227(v5), List::cons(f218(v3.clone()), List::cons(f8(Str::lit(": no input defines ")), List::cons(f218(v3), List::cons(f8(Str::lit(", so its members join no definition (Web IDL 2.2 and 2.7)\u{a}")), List::<Bytes>::nil()))))))), f151(v4, v1, v2))
                    }
                    Some(_) => {
                        f151(v4, v1, v2)
                    }
                }
            } else {
                let a951 = v5;
                let a952 = 8u64;
                if nat_eq(a951, a952) {
                    let m953 = f226(v2.clone(), 0u64, f218(v3.clone()));
                    match m953 {
                        None => {
                            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f218(v3.clone()), List::cons(f8(Str::lit(" includes ")), List::cons(f228(v3.clone()), List::cons(f8(Str::lit(": no input defines the interface ")), List::cons(f218(v3), List::cons(f8(Str::lit(" (Web IDL 2.3)\u{a}")), List::<Bytes>::nil())))))))), f151(v4, v1, v2))
                        }
                        Some(_) => {
                            f151(v4, v1, v2)
                        }
                    }
                } else {
                    if f215(v5) {
                        let m954 = f219(v1.clone(), f218(v3.clone()));
                        match m954 {
                            None => {
                                f151(v4, v1, v2)
                            }
                            Some(v8) => {
                                if f32(f220(v8.clone()), Bytes::lit(&[])) {
                                    f151(v4, v1, v2)
                                } else {
                                    List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f218(v3), List::cons(f8(Str::lit(": ")), List::cons(f220(v8), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), f151(v4, v1, v2))
                                }
                            }
                        }
                    } else {
                        f151(v4, v1, v2)
                    }
                }
            }
        }
    }
}

pub fn f152(v0: List<(Bytes, (List<Bytes>, Bytes))>) -> List<Bytes> {
    let m955 = v0;
    match m955.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons({
                let (h956, _) = v1;
                h956
            }, List::cons(f8(Str::lit(": it refers back to itself through its members, and the translator emits non-recursive declarations only\u{a}")), List::<Bytes>::nil())))), f152(v2))
        }
    }
}

pub fn f153(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m957 = v0;
    match m957.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m958 = v3.clone();
            match m958.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a961 = v2;
                    let a962 = {
                        let a959 = v1.clone();
                        let a960 = f153(v3, v1);
                        append_bytes(a959, a960)
                    };
                    append_bytes(a961, a962)
                }
            }
        }
    }
}

pub fn f154(p965: List<Bytes>, p966: List<Bytes>) -> List<Bytes> {
    let m964 = (p965, p966);
    {
        let mut m964 = m964;
        loop {
            let step = {
                let (v0, v1) = m964;
                let m963 = v0;
                match m963.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m964 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f155(v0: Bytes) -> R<u64> {
    let (h967, _) = f201(Fn5::F229(v0.clone()), {
        let a969 = {
            let a968 = v0;
            length_bytes(a968)
        };
        let a970 = 1u64;
        nat_add(a969, a970)?
    }, 0u64)?;
    Ok(h967)
}

pub fn f156(v0: Bytes) -> Result<Adt17, Bytes> {
    Err::<Adt17, Bytes>(v0)
}

pub fn f157(v0: Bytes) -> R<Result<u64, Bytes>> {
    if if f159(v0.clone(), 0u64, Str::lit("<?xml")) {
        f230(f160(v0.clone(), 5u64)?)?
    } else {
        false
    } {
        let m972 = f231(v0.clone(), {
            let a971 = Str::lit("?>");
            utf8_encode(a971)
        }, 5u64)?;
        match m972 {
            None => {
                Ok(f232(f137(Str::lit("the XML declaration is not closed (XML 1.0 [23])"), 0u64)?))
            }
            Some(v1) => {
                let m974 = f231(f46(v0.clone(), 0u64, v1), {
                    let a973 = Str::lit("encoding");
                    utf8_encode(a973)
                }, 5u64)?;
                match m974 {
                    None => {
                        Ok(Ok::<u64, Bytes>({
                            let a975 = v1;
                            let a976 = 2u64;
                            nat_add(a975, a976)?
                        }))
                    }
                    Some(v2) => {
                        let v3: u64 = f233(v0.clone(), {
                            let a977 = v2;
                            let a978 = 8u64;
                            nat_add(a977, a978)?
                        })?;
                        let v4: u64 = f233(v0.clone(), {
                            let a979 = v3;
                            let a980 = 1u64;
                            nat_add(a979, a980)?
                        })?;
                        let _: u64 = f160(v0.clone(), v4)?;
                        let v7: u64 = {
                            let m983 = f231(v0.clone(), f46(v0.clone(), v4, 1u64), {
                                let a981 = v4;
                                let a982 = 1u64;
                                nat_add(a981, a982)?
                            })?;
                            match m983 {
                                None => {
                                    v1
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        if f234(f46(v0, {
                            let a984 = v4;
                            let a985 = 1u64;
                            nat_add(a984, a985)?
                        }, {
                            let a988 = v7;
                            let a989 = {
                                let a986 = v4;
                                let a987 = 1u64;
                                nat_add(a986, a987)?
                            };
                            nat_sub(a988, a989)
                        })) {
                            Ok(Ok::<u64, Bytes>({
                                let a990 = v1;
                                let a991 = 2u64;
                                nat_add(a990, a991)?
                            }))
                        } else {
                            Ok(f232(f137(Str::lit("only the UTF-8 encoding is read (XML 1.0 section 4.3.3)"), 0u64)?))
                        }
                    }
                }
            }
        }
    } else {
        Ok(Ok::<u64, Bytes>(0u64))
    }
}

pub fn f158(v0: Bytes, p996: u64, p997: u64) -> R<Result<u64, Bytes>> {
    let m995 = (p996, p997);
    {
        let mut m995 = m995;
        loop {
            let step = {
                let (v1, v2) = m995;
                let m992 = v2;
                if m992 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f232(f137(Str::lit("the prolog is longer than the input"), v1)?)))
                } else {
                    let v3 = m992 - 1;
                    let v4: u64 = f233(v0.clone(), v1)?;
                    if f159(v0.clone(), v4, Str::lit("<!--")) {
                        let m993 = f235(v0.clone(), v4)?;
                        match m993 {
                            Ok(v6) => {
                                Ok::<(u64, u64), R<Result<u64, Bytes>>>((v6, v3))
                            }
                            Err(v5) => {
                                Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f232(v5)))
                            }
                        }
                    } else {
                        if f159(v0.clone(), v4, Str::lit("<?")) {
                            let m994 = f236(v0.clone(), v4)?;
                            match m994 {
                                Ok(v8) => {
                                    Ok::<(u64, u64), R<Result<u64, Bytes>>>((v8, v3))
                                }
                                Err(v7) => {
                                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f232(v7)))
                                }
                            }
                        } else {
                            Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Ok::<u64, Bytes>(v4)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m995 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f159(v0: Bytes, v1: u64, v2: Str) -> bool {
    f133(v0, {
        let a998 = v2;
        utf8_encode(a998)
    }, v1)
}

pub fn f160(v0: Bytes, v1: u64) -> R<u64> {
    f111(v0, v1)
}

pub fn f161(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Result<(Adt17, u64), Bytes>> {
    let m999 = v2;
    if m999 == 0 {
        Ok(f237(f137(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v4 = m999 - 1;
        let a1000 = v3;
        let a1001 = 0u64;
        if nat_eq(a1000, a1001) {
            let v5: u64 = f238(v0.clone(), {
                let a1002 = v1;
                let a1003 = 1u64;
                nat_add(a1002, a1003)?
            })?;
            let a1006 = v5;
            let a1007 = {
                let a1004 = v1;
                let a1005 = 1u64;
                nat_add(a1004, a1005)?
            };
            if nat_eq(a1006, a1007) {
                Ok(f237(f137(Str::lit("an element must start with a Name (XML 1.0 [40])"), v1)?))
            } else {
                let v6: Bytes = f46(v0.clone(), {
                    let a1008 = v1;
                    let a1009 = 1u64;
                    nat_add(a1008, a1009)?
                }, {
                    let a1012 = v5;
                    let a1013 = {
                        let a1010 = v1;
                        let a1011 = 1u64;
                        nat_add(a1010, a1011)?
                    };
                    nat_sub(a1012, a1013)
                });
                let m1014 = f239(v0.clone(), v5, v4, List::<Bytes>::nil())?;
                match m1014 {
                    Ok(v8) => {
                        if f159(v0.clone(), {
                            let (_, h1015) = v8.clone();
                            h1015
                        }, Str::lit("/>")) {
                            Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C0(v6, std::rc::Rc::new({
                                let (h1016, _) = v8;
                                h1016
                            }), std::rc::Rc::new(Adt17::C4)), {
                                let a1018 = {
                                    let (_, h1017) = v8;
                                    h1017
                                };
                                let a1019 = 2u64;
                                nat_add(a1018, a1019)?
                            })))
                        } else {
                            let m1023 = f161(v0.clone(), {
                                let a1021 = {
                                    let (_, h1020) = v8;
                                    h1020
                                };
                                let a1022 = 1u64;
                                nat_add(a1021, a1022)?
                            }, v4, 1u64)?;
                            match m1023 {
                                Ok(v10) => {
                                    let v11: u64 = {
                                        let a1025 = {
                                            let (_, h1024) = v10.clone();
                                            h1024
                                        };
                                        let a1026 = 2u64;
                                        nat_add(a1025, a1026)?
                                    };
                                    let v12: u64 = f238(v0.clone(), v11)?;
                                    if f32(f46(v0.clone(), v11, {
                                        let a1027 = v12;
                                        let a1028 = v11;
                                        nat_sub(a1027, a1028)
                                    }), v6.clone()) {
                                        let v13: u64 = f233(v0.clone(), v12)?;
                                        let a1029 = f160(v0, v13)?;
                                        let a1030 = 62u64;
                                        if nat_eq(a1029, a1030) {
                                            Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C0(v6, std::rc::Rc::new({
                                                let (h1031, _) = v8;
                                                h1031
                                            }), std::rc::Rc::new({
                                                let (h1032, _) = v10;
                                                h1032
                                            })), {
                                                let a1033 = v13;
                                                let a1034 = 1u64;
                                                nat_add(a1033, a1034)?
                                            })))
                                        } else {
                                            Ok(f237(f137(Str::lit("an end tag must close with > (XML 1.0 [42])"), v13)?))
                                        }
                                    } else {
                                        Ok(f237(f137(Str::lit("an end tag must name the element its start tag opened (XML 1.0 WFC: Element Type Match)"), {
                                            let (_, h1035) = v10;
                                            h1035
                                        })?))
                                    }
                                }
                                Err(v9) => {
                                    Ok(f237(v9))
                                }
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(f237(v7))
                    }
                }
            }
        } else {
            let a1036 = f160(v0.clone(), v1)?;
            let a1037 = 256u64;
            if nat_eq(a1036, a1037) {
                Ok(f237(f137(Str::lit("the input ends before an element is closed (XML 1.0 [39])"), v1)?))
            } else {
                if f159(v0.clone(), v1, Str::lit("</")) {
                    Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C4, v1)))
                } else {
                    if f159(v0.clone(), v1, Str::lit("<!--")) {
                        let m1038 = f235(v0.clone(), v1)?;
                        match m1038 {
                            Ok(v15) => {
                                f161(v0, v15, v4, 1u64)
                            }
                            Err(v14) => {
                                Ok(f237(v14))
                            }
                        }
                    } else {
                        if f159(v0.clone(), v1, Str::lit("<![CDATA[")) {
                            let m1042 = f231(v0.clone(), {
                                let a1039 = Str::lit("]]>");
                                utf8_encode(a1039)
                            }, {
                                let a1040 = v1;
                                let a1041 = 9u64;
                                nat_add(a1040, a1041)?
                            })?;
                            match m1042 {
                                None => {
                                    Ok(f237(f137(Str::lit("a CDATA section is not closed (XML 1.0 [18])"), v1)?))
                                }
                                Some(v16) => {
                                    let m1045 = f161(v0.clone(), {
                                        let a1043 = v16;
                                        let a1044 = 3u64;
                                        nat_add(a1043, a1044)?
                                    }, v4, 1u64)?;
                                    match m1045 {
                                        Ok(v18) => {
                                            Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C3(std::rc::Rc::new(Adt17::C2(f46(v0, {
                                                let a1046 = v1;
                                                let a1047 = 9u64;
                                                nat_add(a1046, a1047)?
                                            }, {
                                                let a1050 = v16;
                                                let a1051 = {
                                                    let a1048 = v1;
                                                    let a1049 = 9u64;
                                                    nat_add(a1048, a1049)?
                                                };
                                                nat_sub(a1050, a1051)
                                            }))), std::rc::Rc::new({
                                                let (h1052, _) = v18;
                                                h1052
                                            })), {
                                                let (_, h1053) = v18;
                                                h1053
                                            })))
                                        }
                                        Err(v17) => {
                                            Ok(f237(v17))
                                        }
                                    }
                                }
                            }
                        } else {
                            if f159(v0.clone(), v1, Str::lit("<?")) {
                                let m1054 = f236(v0.clone(), v1)?;
                                match m1054 {
                                    Ok(v20) => {
                                        f161(v0, v20, v4, 1u64)
                                    }
                                    Err(v19) => {
                                        Ok(f237(v19))
                                    }
                                }
                            } else {
                                if f159(v0.clone(), v1, Str::lit("<!")) {
                                    Ok(f237(f137(Str::lit("a markup declaration is not allowed in content (XML 1.0 [43])"), v1)?))
                                } else {
                                    let a1055 = f160(v0.clone(), v1)?;
                                    let a1056 = 60u64;
                                    if nat_eq(a1055, a1056) {
                                        let m1057 = f161(v0.clone(), v1, v4, 0u64)?;
                                        match m1057 {
                                            Ok(v22) => {
                                                let m1059 = f161(v0, {
                                                    let (_, h1058) = v22;
                                                    h1058
                                                }, v4, 1u64)?;
                                                match m1059 {
                                                    Ok(v24) => {
                                                        Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C3(std::rc::Rc::new({
                                                            let (h1060, _) = v22;
                                                            h1060
                                                        }), std::rc::Rc::new({
                                                            let (h1061, _) = v24;
                                                            h1061
                                                        })), {
                                                            let (_, h1062) = v24;
                                                            h1062
                                                        })))
                                                    }
                                                    Err(v23) => {
                                                        Ok(f237(v23))
                                                    }
                                                }
                                            }
                                            Err(v21) => {
                                                Ok(f237(v21))
                                            }
                                        }
                                    } else {
                                        let m1063 = f240(v0.clone(), v1, v4, Bytes::lit(&[]), false, 0u64)?;
                                        match m1063 {
                                            Ok(v26) => {
                                                let m1065 = f161(v0, {
                                                    let (_, h1064) = v26;
                                                    h1064
                                                }, v4, 1u64)?;
                                                match m1065 {
                                                    Ok(v28) => {
                                                        Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C3(std::rc::Rc::new(Adt17::C2({
                                                            let (h1066, _) = v26;
                                                            h1066
                                                        })), std::rc::Rc::new({
                                                            let (h1067, _) = v28;
                                                            h1067
                                                        })), {
                                                            let (_, h1068) = v28;
                                                            h1068
                                                        })))
                                                    }
                                                    Err(v27) => {
                                                        Ok(f237(v27))
                                                    }
                                                }
                                            }
                                            Err(v25) => {
                                                Ok(f237(v25))
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

pub fn f162(v0: Bytes) -> Result<(u64, Bytes), Bytes> {
    Err::<(u64, Bytes), Bytes>(v0)
}

pub fn f163(v0: Adt17, v1: List<(Bytes, Bytes)>) -> Result<List<(Bytes, Bytes)>, Bytes> {
    let m1069 = v0;
    match m1069 {
        Adt17::C0(_, _, _) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt17::C1(v2, v3, r1074) => {
            let v4 = (*r1074).clone();
            let m1070 = f163(v4, v1);
            match m1070 {
                Ok(v6) => {
                    if f32(v2.clone(), f8(Str::lit("xmlns"))) {
                        if if f32(v3.clone(), f242()) {
                            true
                        } else {
                            f32(v3.clone(), f243())
                        } {
                            f241(f8(Str::lit("the default namespace must not be the xml or xmlns namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Bytes>(List::cons((Bytes::lit(&[]), v3), v6))
                        }
                    } else {
                        if f53(v2.clone(), f8(Str::lit("xmlns:"))) {
                            let v7: Bytes = f46(v2.clone(), 6u64, {
                                let a1072 = {
                                    let a1071 = v2;
                                    length_bytes(a1071)
                                };
                                let a1073 = 6u64;
                                nat_sub(a1072, a1073)
                            });
                            if f32(v7.clone(), f8(Str::lit("xmlns"))) {
                                f241(f8(Str::lit("the prefix xmlns must not be declared (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                            } else {
                                if if f32(v7.clone(), f8(Str::lit("xml"))) {
                                    !f32(v3.clone(), f242())
                                } else {
                                    false
                                } {
                                    f241(f8(Str::lit("the prefix xml may be bound only to its own namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                } else {
                                    if if if f32(v3.clone(), f242()) {
                                        true
                                    } else {
                                        f32(v3.clone(), f243())
                                    } {
                                        !f32(v7.clone(), f8(Str::lit("xml")))
                                    } else {
                                        false
                                    } {
                                        f241(f8(Str::lit("the xml and xmlns namespace names must not be bound to another prefix (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                    } else {
                                        if f32(v3.clone(), Bytes::lit(&[])) {
                                            f241(f8(Str::lit("a prefix must not be undeclared (Namespaces in XML 1.0, section 3, NSC: No Prefix Undeclaring)")))
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
                    f241(v5)
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

pub fn f164(v0: List<(Bytes, Bytes)>, v1: Bytes) -> R<Result<(Bytes, Bytes), Bytes>> {
    let v2: Bytes = f244(v1.clone())?;
    if f32(v2.clone(), Bytes::lit(&[])) {
        let m1075 = f245(v0, Bytes::lit(&[]));
        match m1075 {
            None => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((Bytes::lit(&[]), v1)))
            }
            Some(v3) => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((v3, v1)))
            }
        }
    } else {
        if f32(v2.clone(), f8(Str::lit("xml"))) {
            Ok(Ok::<(Bytes, Bytes), Bytes>((f242(), f246(v1)?)))
        } else {
            let m1076 = f245(v0, v2.clone());
            match m1076 {
                None => {
                    Ok(Err::<(Bytes, Bytes), Bytes>(f7(List::cons(f8(Str::lit("the prefix ")), List::cons(v2, List::cons(f8(Str::lit(" is not declared (Namespaces in XML 1.0, section 5, NSC: Prefix Declared)")), List::<Bytes>::nil()))))))
                }
                Some(v4) => {
                    Ok(Ok::<(Bytes, Bytes), Bytes>((v4, f246(v1)?)))
                }
            }
        }
    }
}

pub fn f165(v0: (Bytes, Bytes), v1: Str) -> bool {
    if f32({
        let (h1077, _) = v0;
        h1077
    }, f247()) {
        f32({
            let (_, h1078) = v0;
            h1078
        }, {
            let a1079 = v1;
            utf8_encode(a1079)
        })
    } else {
        false
    }
}

pub fn f166(p1083: Adt17, v1: Bytes) -> Option<Bytes> {
    let m1082 = p1083;
    {
        let mut m1082 = m1082;
        loop {
            let step = {
                let v0 = m1082;
                let m1080 = v0;
                match m1080 {
                    Adt17::C0(_, _, _) => {
                        Err::<Adt17, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt17::C1(v2, v3, r1081) => {
                        let v4 = (*r1081).clone();
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
                Ok(next) => { m1082 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f167(v0: Adt17, v1: List<(Bytes, Bytes)>, v2: Bytes) -> R<Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>> {
    let m1084 = v0;
    match m1084 {
        Adt17::C0(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt17::C2(_) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt17::C3(r1095, r1096) => {
            let v3 = (*r1095).clone();
            let v4 = (*r1096).clone();
            let m1085 = f167(v4, v1.clone(), v2.clone())?;
            match m1085 {
                Ok(v6) => {
                    let m1086 = v3;
                    match m1086 {
                        Adt17::C0(v7, r1093, r1094) => {
                            let v8 = (*r1093).clone();
                            let v9 = (*r1094).clone();
                            let m1087 = f163(v8.clone(), v1);
                            match m1087 {
                                Ok(v11) => {
                                    let m1088 = f164(v11.clone(), v7)?;
                                    match m1088 {
                                        Ok(v13) => {
                                            if f165(v13, Str::lit("simpleType")) {
                                                let m1089 = f166(v8, f8(Str::lit("name")));
                                                match m1089 {
                                                    None => {
                                                        Ok(f248(f8(Str::lit("a top-level simpleType must have a name (XSD 1.1 Part 2, section 4.1.2)"))))
                                                    }
                                                    Some(v14) => {
                                                        let m1090 = f249(v14.clone(), v9, v11, v2)?;
                                                        match m1090 {
                                                            Ok(v16) => {
                                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::cons((f125(v14)?, (match v16.clone() {
                                                                    Adt24::C0(_, h1091) => {
                                                                        h1091
                                                                    }
                                                                }, match v16 {
                                                                    Adt24::C0(h1092, _) => {
                                                                        h1092
                                                                    }
                                                                })), v6)))
                                                            }
                                                            Err(v15) => {
                                                                Ok(f248(v15))
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                                            }
                                        }
                                        Err(v12) => {
                                            Ok(f248(v12))
                                        }
                                    }
                                }
                                Err(v10) => {
                                    Ok(f248(v10))
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
                    Ok(f248(v5))
                }
            }
        }
        Adt17::C4 => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
    }
}

pub fn f168(v0: List<(Bytes, (List<Bytes>, Bytes))>) -> List<Bytes> {
    let m1097 = v0;
    match m1097.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h1098, _) = v1;
                h1098
            }, f168(v2))
        }
    }
}

pub fn f169(p1101: List<Bytes>) -> Option<Bytes> {
    let m1100 = p1101;
    {
        let mut m1100 = m1100;
        loop {
            let step = {
                let v0 = m1100;
                let m1099 = v0;
                match m1099.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f122(v1.clone(), v2.clone()) {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v1))
                        } else {
                            Ok::<List<Bytes>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1100 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f170(p1107: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<Bytes> {
    let m1106 = p1107;
    {
        let mut m1106 = m1106;
        loop {
            let step = {
                let v0 = m1106;
                let m1102 = v0;
                match m1102.uncons() {
                    None => {
                        Err::<List<(Bytes, (List<Bytes>, Bytes))>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        let m1105 = f250({
                            let (h1103, _) = {
                                let (_, h1104) = v2;
                                h1104
                            };
                            h1103
                        }, v1.clone());
                        match m1105 {
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
                Ok(next) => { m1106 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f171(p1118: u64, p1119: List<(Bytes, (List<Bytes>, Bytes))>, p1120: List<Bytes>, p1121: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m1117 = (p1118, (p1119, (p1120, p1121)));
    {
        let mut m1117 = m1117;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m1117;
                let m1108 = v0;
                if m1108 == 0 {
                    let m1109 = v1;
                    match m1109.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(f251(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                        }
                    }
                } else {
                    let v6 = m1108 - 1;
                    let m1110 = v1.clone();
                    match m1110.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            let m1111 = f225(v1, v2.clone());
                            match m1111 {
                                None => {
                                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(f251(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                                }
                                Some(v9) => {
                                    Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>((v6, ({
                                        let (_, h1112) = v9.clone();
                                        h1112
                                    }, (List::cons({
                                        let (h1113, _) = {
                                            let (h1114, _) = v9.clone();
                                            h1114
                                        };
                                        h1113
                                    }, v2), List::cons({
                                        let (_, h1115) = {
                                            let (h1116, _) = v9;
                                            h1116
                                        };
                                        h1115
                                    }, v3)))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1117 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f172(v0: Bytes) -> R<List<(Bytes, Bytes)>> {
    f252(f196(v0)?, None::<Bytes>)
}

pub fn f173(p1142: u64, v1: List<(Bytes, Option<Bytes>)>, v2: List<(Bytes, Bytes)>, p1143: List<(Bytes, (Bytes, Bytes))>, p1144: List<Adt19>, p1145: List<Bytes>) -> R<Adt18> {
    let m1141 = (p1142, (p1143, (p1144, p1145)));
    {
        let mut m1141 = m1141;
        loop {
            let step = {
                let (v0, (v3, (v4, v5))) = m1141;
                let m1122 = v0;
                if m1122 == 0 {
                    Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>(Ok(Adt18::C1(f8(Str::lit("the schema documents import and include more documents than the bound authorities hold")))))
                } else {
                    let v6 = m1122 - 1;
                    let m1123 = v3;
                    match m1123.uncons() {
                        None => {
                            let m1124 = v5.clone();
                            match m1124.uncons() {
                                None => {
                                    let m1125 = f253(f254(v4, List::<Adt19>::nil()))?;
                                    match m1125 {
                                        Ok(v8) => {
                                            Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>(Ok(Adt18::C2({
                                                let (h1126, _) = v8;
                                                h1126
                                            }, {
                                                let (_, h1127) = v8;
                                                h1127
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
                                let (h1128, _) = v11.clone();
                                h1128
                            };
                            if if f122(v13.clone(), f255(v4.clone())) {
                                true
                            } else {
                                f122(v13.clone(), v5.clone())
                            } {
                                Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>((v6, (v12, (v4, v5))))
                            } else {
                                let m1129 = f256(v1.clone(), v13.clone());
                                match m1129 {
                                    None => {
                                        Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>((v6, (v12, (v4, List::cons(v13, v5)))))
                                    }
                                    Some(v14) => {
                                        let m1130 = v14;
                                        match m1130 {
                                            None => {
                                                Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>(Ok(Adt18::C1(f257(f8(Str::lit("the schema document ")), v13, f8(Str::lit(" is not in the repository"))))))
                                            }
                                            Some(v15) => {
                                                let m1131 = f258(v13.clone(), v15, f259(v2.clone(), v13))?;
                                                match m1131 {
                                                    Ok(v17) => {
                                                        let m1136 = f260(v17.clone(), {
                                                            let (h1132, _) = {
                                                                let (_, h1133) = v11.clone();
                                                                h1133
                                                            };
                                                            h1132
                                                        }, {
                                                            let (_, h1134) = {
                                                                let (_, h1135) = v11;
                                                                h1135
                                                            };
                                                            h1134
                                                        });
                                                        match m1136 {
                                                            None => {
                                                                let m1139 = f261(match v17.clone() {
                                                                    Adt19::C0(_, _, _, _, h1137, _) => {
                                                                        h1137
                                                                    }
                                                                }, match v17.clone() {
                                                                    Adt19::C0(_, _, _, h1138, _, _) => {
                                                                        h1138
                                                                    }
                                                                })?;
                                                                match m1139 {
                                                                    Ok(v20) => {
                                                                        let m1140 = f262(v20, v17.clone(), v2.clone())?;
                                                                        match m1140 {
                                                                            Ok(v22) => {
                                                                                Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>((v6, (f263(v12, v22), (List::cons(v17, v4), v5))))
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
                Ok(next) => { m1141 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f174(v0: List<List<(Bytes, Option<Bytes>)>>) -> List<(Bytes, Option<Bytes>)> {
    let m1146 = v0;
    match m1146.uncons() {
        None => {
            List::<(Bytes, Option<Bytes>)>::nil()
        }
        Some((v1, v2)) => {
            f264(v1, f174(v2))
        }
    }
}

pub fn f175(v0: Adt2) -> R<Result<Bytes, Bytes>> {
    let m1147 = f265(v0.clone());
    match m1147 {
        None => {
            Ok(Ok::<Bytes, Bytes>(f193(v0)?))
        }
        Some(v1) => {
            Ok(Err::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("the node tagged ")), List::cons(v1, List::cons(f8(Str::lit(" has no JSON text: a JSON value carries no tag (RFC 8259 section 3), and a YAML node keeps its tag (YAML 1.2.2 section 3.2.1.1)")), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f176(v0: Bytes, v1: u64, v2: u64, v3: Bytes) -> Bytes {
    let a1150 = v3;
    let a1151 = f46(v0, v1, {
        let a1148 = v2;
        let a1149 = v1;
        nat_sub(a1148, a1149)
    });
    append_bytes(a1150, a1151)
}

pub fn f177(v0: Bytes, p1164: u64, p1165: u64) -> R<u64> {
    let m1163 = (p1164, p1165);
    {
        let mut m1163 = m1163;
        loop {
            let step = {
                let (v1, v2) = m1163;
                let m1152 = v2;
                if m1152 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m1152 - 1;
                    let v4: u64 = f111(v0.clone(), v1)?;
                    let a1153 = v4;
                    let a1154 = 34u64;
                    if nat_eq(a1153, a1154) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a1155 = v4;
                        let a1156 = 92u64;
                        if nat_eq(a1155, a1156) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a1157 = v4;
                            let a1158 = 32u64;
                            if nat_lt(a1157, a1158) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a1159 = v4;
                                let a1160 = 256u64;
                                if nat_eq(a1159, a1160) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a1161 = v1;
                                        let a1162 = 1u64;
                                        nat_add(a1161, a1162)?
                                    }, v3))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1163 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f178(v0: Bytes, p1191: u64, p1192: u64, p1193: Bytes) -> R<Option<(u64, Bytes)>> {
    let m1190 = (p1191, (p1192, p1193));
    {
        let mut m1190 = m1190;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1190;
                let m1166 = v2;
                if m1166 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                } else {
                    let v4 = m1166 - 1;
                    let v5: u64 = f177(v0.clone(), v1, {
                        let a1170 = {
                            let a1168 = {
                                let a1167 = v0.clone();
                                length_bytes(a1167)
                            };
                            let a1169 = v1;
                            nat_sub(a1168, a1169)
                        };
                        let a1171 = 1u64;
                        nat_add(a1170, a1171)?
                    })?;
                    let v6: Bytes = {
                        let a1174 = v3;
                        let a1175 = f46(v0.clone(), v1, {
                            let a1172 = v5;
                            let a1173 = v1;
                            nat_sub(a1172, a1173)
                        });
                        append_bytes(a1174, a1175)
                    };
                    let v7: u64 = f111(v0.clone(), v5)?;
                    let a1176 = v7;
                    let a1177 = 34u64;
                    if nat_eq(a1176, a1177) {
                        Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(Some(({
                            let a1178 = v5;
                            let a1179 = 1u64;
                            nat_add(a1178, a1179)?
                        }, v6))))
                    } else {
                        let a1180 = v7;
                        let a1181 = 92u64;
                        if nat_eq(a1180, a1181) {
                            let m1182 = f266(v0.clone(), v5, v6)?;
                            match m1182 {
                                None => {
                                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                                }
                                Some(v8) => {
                                    let a1185 = {
                                        let (_, h1183) = {
                                            let (_, h1184) = v8.clone();
                                            h1184
                                        };
                                        h1183
                                    };
                                    let a1186 = 0u64;
                                    if nat_eq(a1185, a1186) {
                                        Ok::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(({
                                            let (h1187, _) = v8;
                                            h1187
                                        }, (v4, {
                                            let (h1188, _) = {
                                                let (_, h1189) = v8;
                                                h1189
                                            };
                                            h1188
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
                Ok(next) => { m1190 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f179(v0: Bytes, v1: u64) -> R<u64> {
    let (h1194, _) = f201(Fn5::F267(v0.clone()), {
        let a1196 = {
            let a1195 = v0;
            length_bytes(a1195)
        };
        let a1197 = 1u64;
        nat_add(a1196, a1197)?
    }, v1)?;
    Ok(h1194)
}

pub fn f180(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f181(v0: Bytes) -> R<u64> {
    let (h1198, _) = f201(Fn5::F268(v0.clone()), {
        let a1200 = {
            let a1199 = v0;
            length_bytes(a1199)
        };
        let a1201 = 1u64;
        nat_add(a1200, a1201)?
    }, 0u64)?;
    Ok(h1198)
}

pub fn f182(v0: Bytes) -> R<Option<u64>> {
    if f269(v0.clone())? {
        Ok(Some({
            let (_, h1202) = {
                let (h1203, _) = f270(Fn6::F271(v0.clone()), {
                    let a1204 = v0;
                    length_bytes(a1204)
                }, (0u64, 0u64))?;
                h1203
            };
            h1202
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f183(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f46(v1.clone(), 0u64, {
        let a1206 = {
            let a1205 = v1.clone();
            length_bytes(a1205)
        };
        let a1207 = f272(v1)?;
        nat_sub(a1206, a1207)
    });
    let v5: Bytes = {
        let a1208 = v0;
        let a1209 = v4.clone();
        append_bytes(a1208, a1209)
    };
    let v6: u64 = f181(v5.clone())?;
    let v7: Bytes = f46(v5.clone(), v6, {
        let a1211 = {
            let a1210 = v5;
            length_bytes(a1210)
        };
        let a1212 = v6;
        nat_sub(a1211, a1212)
    });
    let v8: u64 = {
        let a1213 = v4;
        length_bytes(a1213)
    };
    let v9: u64 = {
        let a1214 = v7.clone();
        length_bytes(a1214)
    };
    let a1215 = v9;
    let a1216 = 0u64;
    if nat_eq(a1215, a1216) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f180(v2, {
            let a1217 = v3;
            let a1218 = v8;
            nat_lt(a1217, a1218)
        }) {
            let v10: u64 = if v2 {
                let a1219 = v3;
                let a1220 = v8;
                nat_add(a1219, a1220)?
            } else {
                let a1221 = v8;
                let a1222 = v3;
                nat_sub(a1221, a1222)
            };
            let v11: u64 = f272(v7.clone())?;
            let a1223 = v10;
            let a1224 = v11;
            if nat_le(a1223, a1224) {
                Ok(Some((f46(v7, 0u64, {
                    let a1225 = v9;
                    let a1226 = v10;
                    nat_sub(a1225, a1226)
                }), 0u64)))
            } else {
                let a1229 = {
                    let a1227 = v10;
                    let a1228 = v11;
                    nat_sub(a1227, a1228)
                };
                let a1230 = 19u64;
                if nat_le(a1229, a1230) {
                    Ok(Some((f46(v7, 0u64, {
                        let a1231 = v9;
                        let a1232 = v11;
                        nat_sub(a1231, a1232)
                    }), {
                        let a1233 = v10;
                        let a1234 = v11;
                        nat_sub(a1233, a1234)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a1235 = v3;
                let a1236 = v8;
                nat_sub(a1235, a1236)
            };
            let a1239 = {
                let a1237 = v9;
                let a1238 = v12;
                nat_add(a1237, a1238)?
            };
            let a1240 = 19u64;
            if nat_le(a1239, a1240) {
                Ok(Some(({
                    let a1241 = v7;
                    let a1242 = f273(v12);
                    append_bytes(a1241, a1242)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f184(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1243 = 0u64;
        let a1244 = v1;
        if nat_lt(a1243, a1244) {
            let a1245 = v1;
            let a1246 = 9223372036854775808u64;
            nat_le(a1245, a1246)
        } else {
            false
        }
    } else {
        let a1247 = v1;
        let a1248 = 9223372036854775807u64;
        nat_le(a1247, a1248)
    }
}

pub fn f185(v0: Bytes, v1: u64) -> Option<u8> {
    let a1249 = v0;
    let a1250 = v1;
    index_bytes(a1249, a1250)
}

pub fn f186(v0: u32) -> Bytes {
    let a1279 = f274({
        let a1251 = f275(v0, 28u32);
        let a1252 = 15u32;
        fixed_u32::bit_and(a1251, a1252)
    });
    let a1280 = {
        let a1277 = f274({
            let a1253 = f275(v0, 24u32);
            let a1254 = 15u32;
            fixed_u32::bit_and(a1253, a1254)
        });
        let a1278 = {
            let a1275 = f274({
                let a1255 = f275(v0, 20u32);
                let a1256 = 15u32;
                fixed_u32::bit_and(a1255, a1256)
            });
            let a1276 = {
                let a1273 = f274({
                    let a1257 = f275(v0, 16u32);
                    let a1258 = 15u32;
                    fixed_u32::bit_and(a1257, a1258)
                });
                let a1274 = {
                    let a1271 = f274({
                        let a1259 = f275(v0, 12u32);
                        let a1260 = 15u32;
                        fixed_u32::bit_and(a1259, a1260)
                    });
                    let a1272 = {
                        let a1269 = f274({
                            let a1261 = f275(v0, 8u32);
                            let a1262 = 15u32;
                            fixed_u32::bit_and(a1261, a1262)
                        });
                        let a1270 = {
                            let a1267 = f274({
                                let a1263 = f275(v0, 4u32);
                                let a1264 = 15u32;
                                fixed_u32::bit_and(a1263, a1264)
                            });
                            let a1268 = f274({
                                let a1265 = v0;
                                let a1266 = 15u32;
                                fixed_u32::bit_and(a1265, a1266)
                            });
                            append_bytes(a1267, a1268)
                        };
                        append_bytes(a1269, a1270)
                    };
                    append_bytes(a1271, a1272)
                };
                append_bytes(a1273, a1274)
            };
            append_bytes(a1275, a1276)
        };
        append_bytes(a1277, a1278)
    };
    append_bytes(a1279, a1280)
}

pub fn f187(v0: Fn3, p1284: u64, p1285: (u64, Adt20)) -> R<(u64, Adt20)> {
    let m1283 = (p1284, p1285);
    {
        let mut m1283 = m1283;
        loop {
            let step = {
                let (v1, v2) = m1283;
                let m1281 = v1;
                if m1281 == 0 {
                    Err::<(u64, (u64, Adt20)), R<(u64, Adt20)>>(Ok(v2))
                } else {
                    let v3 = m1281 - 1;
                    Ok::<(u64, (u64, Adt20)), R<(u64, Adt20)>>((v3, {
                        let c1282: Fn3 = v0.clone();
                        c1282.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1283 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f188(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt20)) -> R<(u64, Adt20)> {
    Ok(({
        let a1287 = {
            let (h1286, _) = v3.clone();
            h1286
        };
        let a1288 = 64u64;
        nat_add(a1287, a1288)?
    }, f276({
        let (_, h1289) = v3;
        h1289
    }, f277(v0, v1, v2, {
        let (h1290, _) = v3;
        h1290
    })?)?))
}

pub fn f189() -> Adt20 {
    Adt20::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f190(p1295: List<(Bytes, Option<Adt2>)>, v1: Bytes) -> Option<Option<Adt2>> {
    let m1294 = p1295;
    {
        let mut m1294 = m1294;
        loop {
            let step = {
                let v0 = m1294;
                let m1291 = v0;
                match m1291.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(None::<Option<Adt2>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h1292, _) = v2;
                            h1292
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(Some({
                                let (_, h1293) = v2;
                                h1293
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1294 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f191(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Adt2>)> {
    let m1299 = {
        let a1297 = match v0 {
            Adt1::C0(_, _, _, h1296) => {
                h1296
            }
        };
        let a1298 = f40(v1);
        index_list(a1297, a1298)
    };
    match m1299 {
        None => {
            List::<(Bytes, Option<Adt2>)>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f192(v0: Bytes, p1320: u64, p1321: u64, p1322: bool, p1323: Bytes) -> R<Bytes> {
    let m1319 = (p1320, (p1321, (p1322, p1323)));
    {
        let mut m1319 = m1319;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1319;
                let m1300 = v2;
                if m1300 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1300 - 1;
                    let m1303 = {
                        let a1301 = v0.clone();
                        let a1302 = v1;
                        index_bytes(a1301, a1302)
                    };
                    match m1303 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f62(v6)?;
                            if f278(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1304 = v1;
                                    let a1305 = 1u64;
                                    nat_add(a1304, a1305)?
                                }, (v5, (true, v4))))
                            } else {
                                if f113(f279(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1306 = v1;
                                        let a1307 = 1u64;
                                        nat_add(a1306, a1307)?
                                    }, (v5, (false, {
                                        let a1311 = v4;
                                        let a1312 = f46({
                                            let a1308 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1308)
                                        }, {
                                            let a1309 = v7;
                                            let a1310 = 97u64;
                                            nat_sub(a1309, a1310)
                                        }, 1u64);
                                        append_bytes(a1311, a1312)
                                    }))))
                                } else {
                                    if f180(f279(v7), f180(f280(v7), f281(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1313 = v1;
                                            let a1314 = 1u64;
                                            nat_add(a1313, a1314)?
                                        }, (v5, (false, {
                                            let a1315 = v4;
                                            let a1316 = f46(v0.clone(), v1, 1u64);
                                            append_bytes(a1315, a1316)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1317 = v1;
                                            let a1318 = 1u64;
                                            nat_add(a1317, a1318)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1319 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f193(v0: Adt2) -> R<Bytes> {
    let m1324 = v0;
    match m1324 {
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
            f282(v4, v5)
        }
        Adt2::C4(v6, v7, v8) => {
            f283(v6, v7, v8)
        }
        Adt2::C5(v9) => {
            f284(v9)
        }
        Adt2::C6(r1325) => {
            let v10 = (*r1325).clone();
            Ok(f285(f286(v10)?))
        }
        Adt2::C7(r1326) => {
            let v11 = (*r1326).clone();
            f287(f288(v11)?)
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

pub fn f194(v0: List<Adt2>, v1: Bytes, v2: Bytes) -> List<Adt2> {
    let m1327 = v0;
    match m1327.uncons() {
        None => {
            List::<Adt2>::nil()
        }
        Some((v3, v4)) => {
            if f32(f123(v3.clone(), v1.clone()), v2.clone()) {
                f194(v4, v1, v2)
            } else {
                List::cons(v3, f194(v4, v1, v2))
            }
        }
    }
}

pub fn f195(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m1328 = v0;
    match m1328.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f53(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f153(f289(f197(List::cons(v1, f290(f44(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f195(v3, v1)?))
            }
        }
    }
}

pub fn f196(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a1329 = v0.clone();
        length_bytes(a1329)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h1330, _) = f66(Fn0::F291(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h1330
    };
    let v3: u64 = {
        let (h1331, _) = {
            let (_, h1332) = v2.clone();
            h1332
        };
        h1331
    };
    let v4: List<Bytes> = {
        let (_, h1333) = {
            let (_, h1334) = v2;
            h1334
        };
        h1333
    };
    let a1335 = v3;
    let a1336 = v1;
    if nat_lt(a1335, a1336) {
        Ok(f68(List::cons(f46(v0, v3, {
            let a1337 = v1;
            let a1338 = v3;
            nat_sub(a1337, a1338)
        }), v4)))
    } else {
        Ok(f68(v4))
    }
}

pub fn f197(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f292(f116(Fn2::F293, List::<Bytes>::nil(), f294(v0)?)))
}

pub fn f198(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a1339 = v0.clone();
        length_bytes(a1339)
    };
    let v3: u64 = {
        let a1340 = v1.clone();
        length_bytes(a1340)
    };
    let v4: u64 = {
        let (h1341, _) = f201(Fn5::F295(v1.clone(), v3, v0.clone(), v2), {
            let a1342 = v2;
            let a1343 = 1u64;
            nat_add(a1342, a1343)?
        }, 0u64)?;
        h1341
    };
    if f133(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f199(v0: Fn4, p1348: u64, p1349: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1347 = (p1348, p1349);
    {
        let mut m1347 = m1347;
        loop {
            let step = {
                let (v1, v2) = m1347;
                let m1344 = v1;
                if m1344 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1344 - 1;
                    let m1346 = {
                        let c1345: Fn4 = v0.clone();
                        c1345.apply(v2.clone())?
                    };
                    match m1346 {
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
                Ok(next) => { m1347 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f200(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f296(v0, v1)
}

pub fn f201(v0: Fn5, p1354: u64, p1355: u64) -> R<(u64, bool)> {
    let m1353 = (p1354, p1355);
    {
        let mut m1353 = m1353;
        loop {
            let step = {
                let (v1, v2) = m1353;
                let m1350 = v1;
                if m1350 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1350 - 1;
                    let m1352 = {
                        let c1351: Fn5 = v0.clone();
                        c1351.apply(v2)?
                    };
                    match m1352 {
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
                Ok(next) => { m1353 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f202(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f297(v0, v1)
}

pub fn f203(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f298(v0, v1)
}

pub fn f204(v0: Bytes, v1: u64) -> R<bool> {
    if if if f159(v0.clone(), v1, Str::lit("---")) {
        true
    } else {
        f159(v0.clone(), v1, Str::lit("..."))
    } {
        if f299(f206(v0.clone(), {
            let a1356 = v1;
            let a1357 = 3u64;
            nat_add(a1356, a1357)?
        })?) {
            true
        } else {
            f300(f206(v0.clone(), {
                let a1358 = v1;
                let a1359 = 3u64;
                nat_add(a1358, a1359)?
            })?)
        }
    } else {
        false
    } {
        let a1360 = f301(v0, v1)?;
        let a1361 = 0u64;
        Ok(nat_eq(a1360, a1361))
    } else {
        Ok(false)
    }
}

pub fn f205(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f302(v0.clone(), v1)?;
    let v3: u64 = f206(v0.clone(), v2)?;
    if if f300(v3) {
        true
    } else {
        let a1362 = v3;
        let a1363 = 35u64;
        if nat_eq(a1362, a1363) {
            let a1364 = v1;
            let a1365 = v2;
            nat_lt(a1364, a1365)
        } else {
            false
        }
    } {
        Ok(Ok::<u64, Bytes>(f303(v0, v2)?))
    } else {
        Ok(Err::<u64, Bytes>(f137(Str::lit("only white space and a comment may follow a node on its line (YAML 1.2.2 [77])"), v2)?))
    }
}

pub fn f206(v0: Bytes, v1: u64) -> R<u64> {
    f111(v0, v1)
}

pub fn f207(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: List<Bytes>, v8: Adt21) -> R<Adt22> {
    let m1366 = v3;
    if m1366 == 0 {
        Ok(Adt22::C1(f137(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v9 = m1366 - 1;
        let a1367 = v4;
        let a1368 = 0u64;
        if nat_eq(a1367, a1368) {
            let v10: u64 = f304(v0.clone(), v1)?;
            let v11: u64 = {
                let a1369 = v10;
                let a1370 = v1;
                nat_sub(a1369, a1370)
            };
            let a1371 = f206(v0.clone(), v10)?;
            let a1372 = 9u64;
            if nat_eq(a1371, a1372) {
                let v12: u64 = f302(v0.clone(), v10)?;
                if if f305(v0.clone(), v12)? {
                    true
                } else {
                    f306(v0.clone(), v12)?
                } {
                    Ok(Adt22::C1(f137(Str::lit("a tab cannot indent a block node (YAML 1.2.2 [63])"), v12)?))
                } else {
                    let m1373 = f307(v0.clone(), v12)?;
                    match m1373 {
                        None => {
                            f308(v0, v12, v2, v8)
                        }
                        Some(v13) => {
                            Ok(Adt22::C1(v13))
                        }
                    }
                }
            } else {
                let a1374 = f206(v0.clone(), v10)?;
                let a1375 = 33u64;
                if if nat_eq(a1374, a1375) {
                    f208(f306(v0.clone(), v10)?)
                } else {
                    false
                } {
                    if f309(v8) {
                        Ok(Adt22::C1(f137(Str::lit("a node holds at most one tag (YAML 1.2.2 [96])"), v10)?))
                    } else {
                        let v14: Adt21 = f310(v0.clone(), v10)?;
                        let m1376 = f311(v14.clone());
                        match m1376 {
                            None => {
                                f207(v0.clone(), f312(v0.clone(), v10)?, v2, v9, 1u64, false, {
                                    let a1378 = {
                                        let a1377 = v0;
                                        length_bytes(a1377)
                                    };
                                    let a1379 = 1u64;
                                    nat_add(a1378, a1379)?
                                }, List::<Bytes>::nil(), v14)
                            }
                            Some(v15) => {
                                Ok(Adt22::C1(v15))
                            }
                        }
                    }
                } else {
                    if f305(v0.clone(), v10)? {
                        f313(v8, true, v10, f207(v0, v10, v2, v9, 2u64, false, v11, List::<Bytes>::nil(), Adt21::C0(0u64))?)
                    } else {
                        let m1382 = {
                            let a1380 = f206(v0.clone(), v10)?;
                            let a1381 = 33u64;
                            if nat_eq(a1380, a1381) {
                                None::<Bytes>
                            } else {
                                f307(v0.clone(), v10)?
                            }
                        };
                        match m1382 {
                            None => {
                                if f306(v0.clone(), v10)? {
                                    f313(v8, false, v10, f207(v0, v10, v2, v9, 3u64, false, v11, List::<Bytes>::nil(), Adt21::C0(0u64))?)
                                } else {
                                    f308(v0, v10, v2, v8)
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
            let a1383 = v4;
            let a1384 = 1u64;
            if nat_eq(a1383, a1384) {
                let v17: u64 = f302(v0.clone(), v1)?;
                let v18: u64 = f206(v0.clone(), v17)?;
                if if f300(v18) {
                    true
                } else {
                    let a1385 = v18;
                    let a1386 = 35u64;
                    if nat_eq(a1385, a1386) {
                        let a1387 = v1;
                        let a1388 = v17;
                        nat_lt(a1387, a1388)
                    } else {
                        false
                    }
                } {
                    let v19: u64 = f136(v0.clone(), f303(v0.clone(), v17)?)?;
                    let a1390 = {
                        let a1389 = v0.clone();
                        length_bytes(a1389)
                    };
                    let a1391 = v19;
                    if if nat_le(a1390, a1391) {
                        true
                    } else {
                        f204(v0.clone(), v19)?
                    } {
                        f314(v8, v17, v19)
                    } else {
                        let v20: u64 = {
                            let a1392 = f304(v0.clone(), v19)?;
                            let a1393 = v19;
                            nat_sub(a1392, a1393)
                        };
                        let a1394 = v2;
                        let a1395 = v20;
                        if nat_le(a1394, a1395) {
                            f207(v0, v19, v2, v9, 0u64, false, 0u64, List::<Bytes>::nil(), v8)
                        } else {
                            if if if f208(v5) {
                                let a1396 = v20;
                                let a1397 = v6;
                                nat_eq(a1396, a1397)
                            } else {
                                false
                            } {
                                f305(v0.clone(), {
                                    let a1398 = v19;
                                    let a1399 = v20;
                                    nat_add(a1398, a1399)?
                                })?
                            } else {
                                false
                            } {
                                f313(v8, true, v17, f207(v0, {
                                    let a1400 = v19;
                                    let a1401 = v20;
                                    nat_add(a1400, a1401)?
                                }, v2, v9, 2u64, false, v20, List::<Bytes>::nil(), Adt21::C0(0u64))?)
                            } else {
                                f314(v8, v17, v19)
                            }
                        }
                    }
                } else {
                    let a1402 = v18;
                    let a1403 = 33u64;
                    if if nat_eq(a1402, a1403) {
                        f309(v8.clone())
                    } else {
                        false
                    } {
                        Ok(Adt22::C1(f137(Str::lit("a node holds at most one tag (YAML 1.2.2 [96])"), v17)?))
                    } else {
                        let a1404 = v18;
                        let a1405 = 33u64;
                        if if nat_eq(a1404, a1405) {
                            f208(if v5 {
                                f306(v0.clone(), v17)?
                            } else {
                                false
                            })
                        } else {
                            false
                        } {
                            let v21: Adt21 = f310(v0.clone(), v17)?;
                            let m1406 = f311(v21.clone());
                            match m1406 {
                                None => {
                                    f207(v0.clone(), f312(v0, v17)?, v2, v9, 1u64, v5, v6, List::<Bytes>::nil(), v21)
                                }
                                Some(v22) => {
                                    Ok(Adt22::C1(v22))
                                }
                            }
                        } else {
                            if if v5 {
                                f305(v0.clone(), v17)?
                            } else {
                                false
                            } {
                                let a1409 = f315(f316(v0.clone(), v1, v17), 9u64)?;
                                let a1410 = {
                                    let a1407 = v17;
                                    let a1408 = v1;
                                    nat_sub(a1407, a1408)
                                };
                                if nat_lt(a1409, a1410) {
                                    Ok(Adt22::C1(f137(Str::lit("a tab cannot indent a block node (YAML 1.2.2 [63])"), v17)?))
                                } else {
                                    f313(v8, true, v17, f207(v0.clone(), v17, v2, v9, 2u64, false, f301(v0, v17)?, List::<Bytes>::nil(), Adt21::C0(0u64))?)
                                }
                            } else {
                                let m1413 = {
                                    let a1411 = v18;
                                    let a1412 = 33u64;
                                    if nat_eq(a1411, a1412) {
                                        None::<Bytes>
                                    } else {
                                        f307(v0.clone(), v17)?
                                    }
                                };
                                match m1413 {
                                    None => {
                                        if if v5 {
                                            f306(v0.clone(), v17)?
                                        } else {
                                            false
                                        } {
                                            f313(v8, false, v17, f207(v0.clone(), v17, v2, v9, 3u64, false, f301(v0, v17)?, List::<Bytes>::nil(), Adt21::C0(0u64))?)
                                        } else {
                                            f308(v0, v17, v2, v8)
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
                let a1414 = v4;
                let a1415 = 2u64;
                if nat_eq(a1414, a1415) {
                    let m1420 = f207(v0.clone(), {
                        let a1416 = v1;
                        let a1417 = 1u64;
                        nat_add(a1416, a1417)?
                    }, {
                        let a1418 = v6;
                        let a1419 = 1u64;
                        nat_add(a1418, a1419)?
                    }, v9, 1u64, true, v6, List::<Bytes>::nil(), Adt21::C0(0u64))?;
                    match m1420 {
                        Adt22::C0(v25, v26) => {
                            let v27: u64 = f136(v0.clone(), v26)?;
                            let v28: u64 = {
                                let a1421 = f304(v0.clone(), v27)?;
                                let a1422 = v27;
                                nat_sub(a1421, a1422)
                            };
                            let a1424 = {
                                let a1423 = v0.clone();
                                length_bytes(a1423)
                            };
                            let a1425 = v27;
                            if nat_le(a1424, a1425) {
                                Ok(Adt22::C0(Adt2::C9(std::rc::Rc::new(v25), std::rc::Rc::new(Adt2::C8)), v27))
                            } else {
                                let a1426 = v28;
                                let a1427 = v6;
                                if if nat_eq(a1426, a1427) {
                                    f305(v0.clone(), {
                                        let a1428 = v27;
                                        let a1429 = v28;
                                        nat_add(a1428, a1429)?
                                    })?
                                } else {
                                    false
                                } {
                                    let m1432 = f207(v0, {
                                        let a1430 = v27;
                                        let a1431 = v28;
                                        nat_add(a1430, a1431)?
                                    }, v2, v9, 2u64, false, v6, List::<Bytes>::nil(), Adt21::C0(0u64))?;
                                    match m1432 {
                                        Adt22::C0(v30, v31) => {
                                            Ok(Adt22::C0(Adt2::C9(std::rc::Rc::new(v25), std::rc::Rc::new(v30)), v31))
                                        }
                                        Adt22::C1(v29) => {
                                            Ok(Adt22::C1(v29))
                                        }
                                    }
                                } else {
                                    let a1433 = v6;
                                    let a1434 = v28;
                                    if nat_lt(a1433, a1434) {
                                        Ok(Adt22::C1(f137(Str::lit("a line is indented more than its block sequence entry allows (YAML 1.2.2 [183])"), {
                                            let a1435 = v27;
                                            let a1436 = v28;
                                            nat_add(a1435, a1436)?
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
                    let m1437 = f317(v0.clone(), v1)?;
                    match m1437 {
                        Ok(v33) => {
                            let m1438 = f307(v0.clone(), v33)?;
                            match m1438 {
                                None => {
                                    let m1439 = f318(v0.clone(), v33)?;
                                    match m1439 {
                                        None => {
                                            Ok(Adt22::C1(f137(Str::lit("a block mapping entry must be an implicit key and : (YAML 1.2.2 [188])"), v33)?))
                                        }
                                        Some(v35) => {
                                            let v36: Bytes = f319(v0.clone(), v33)?;
                                            if f320(v36.clone(), v7.clone()) {
                                                Ok(Adt22::C1(f137(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                            } else {
                                                let m1444 = f207(v0.clone(), {
                                                    let a1440 = v35;
                                                    let a1441 = 1u64;
                                                    nat_add(a1440, a1441)?
                                                }, {
                                                    let a1442 = v6;
                                                    let a1443 = 1u64;
                                                    nat_add(a1442, a1443)?
                                                }, v9, 1u64, false, v6, List::<Bytes>::nil(), Adt21::C0(0u64))?;
                                                match m1444 {
                                                    Adt22::C0(v38, v39) => {
                                                        let v40: u64 = f136(v0.clone(), v39)?;
                                                        let v41: u64 = {
                                                            let a1445 = f304(v0.clone(), v40)?;
                                                            let a1446 = v40;
                                                            nat_sub(a1445, a1446)
                                                        };
                                                        let a1448 = {
                                                            let a1447 = v0.clone();
                                                            length_bytes(a1447)
                                                        };
                                                        let a1449 = v40;
                                                        if nat_le(a1448, a1449) {
                                                            Ok(Adt22::C0(Adt2::C10(v36, std::rc::Rc::new(v38), std::rc::Rc::new(Adt2::C8)), v40))
                                                        } else {
                                                            let a1450 = v41;
                                                            let a1451 = v6;
                                                            if if if nat_eq(a1450, a1451) {
                                                                f208(f305(v0.clone(), {
                                                                    let a1452 = v40;
                                                                    let a1453 = v41;
                                                                    nat_add(a1452, a1453)?
                                                                })?)
                                                            } else {
                                                                false
                                                            } {
                                                                f208(f204(v0.clone(), v40)?)
                                                            } else {
                                                                false
                                                            } {
                                                                let m1456 = f207(v0, {
                                                                    let a1454 = v40;
                                                                    let a1455 = v41;
                                                                    nat_add(a1454, a1455)?
                                                                }, v2, v9, 3u64, false, v6, List::cons(v36.clone(), v7), Adt21::C0(0u64))?;
                                                                match m1456 {
                                                                    Adt22::C0(v43, v44) => {
                                                                        Ok(Adt22::C0(Adt2::C10(v36, std::rc::Rc::new(v38), std::rc::Rc::new(v43)), v44))
                                                                    }
                                                                    Adt22::C1(v42) => {
                                                                        Ok(Adt22::C1(v42))
                                                                    }
                                                                }
                                                            } else {
                                                                let a1457 = v6;
                                                                let a1458 = v41;
                                                                if nat_lt(a1457, a1458) {
                                                                    Ok(Adt22::C1(f137(Str::lit("a line is indented more than its block mapping entry allows (YAML 1.2.2 [187])"), {
                                                                        let a1459 = v40;
                                                                        let a1460 = v41;
                                                                        nat_add(a1459, a1460)?
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

pub fn f208(v0: bool) -> bool {
    !v0
}

pub fn f209(v0: Bytes, v1: u64) -> Bytes {
    let m1464 = {
        let a1461 = v0;
        let a1462 = v1;
        let a1463 = 1u64;
        slice_bytes(a1461, a1462, a1463)
    };
    match m1464 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f210(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f321(v1, v0, v2)
}

pub fn f211(v0: Bytes) -> R<Result<List<Adt23>, Bytes>> {
    f322({
        let a1466 = {
            let a1465 = v0.clone();
            length_bytes(a1465)
        };
        let a1467 = 1u64;
        nat_add(a1466, a1467)?
    }, v0, 0u64, List::<Adt23>::nil())
}

pub fn f212(p1475: u64, p1476: List<Adt23>, p1477: List<Adt13>) -> R<Result<List<Adt13>, Bytes>> {
    let m1474 = (p1475, (p1476, p1477));
    {
        let mut m1474 = m1474;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1474;
                let m1468 = v0;
                if m1468 == 0 {
                    Err::<(u64, (List<Adt23>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>(Ok(Err::<List<Adt13>, Bytes>(f8(Str::lit("the input has more definitions than tokens")))))
                } else {
                    let v3 = m1468 - 1;
                    let m1469 = v1.clone();
                    match m1469.uncons() {
                        None => {
                            Err::<(u64, (List<Adt23>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>(Ok(Ok::<List<Adt13>, Bytes>(f323(v2, List::<Adt13>::nil()))))
                        }
                        Some((_, _)) => {
                            let m1470 = f324(v1)?;
                            match m1470 {
                                Ok(v7) => {
                                    let m1471 = f325(v3, v7)?;
                                    match m1471 {
                                        Ok(v9) => {
                                            Ok::<(u64, (List<Adt23>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>((v3, ({
                                                let (_, h1472) = v9;
                                                h1472
                                            }, List::cons({
                                                let (h1473, _) = v9;
                                                h1473
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
                Ok(next) => { m1474 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f213(v0: List<Adt16>, v1: List<Adt16>) -> List<Adt16> {
    let m1478 = v0;
    match m1478.uncons() {
        None => {
            List::<Adt16>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = if f32(f220(v2.clone()), Bytes::lit(&[])) {
                f326(f222(v2.clone()), v1.clone())
            } else {
                f220(v2.clone())
            };
            List::cons(Adt16::C0(f223(v2.clone()), v4, f224(v2.clone()), f222(v2)), f213(v3, v1))
        }
    }
}

pub fn f214(v0: List<Adt16>) -> R<u64> {
    let m1479 = v0;
    match m1479.uncons() {
        None => {
            Ok(0u64)
        }
        Some((v1, v2)) => {
            if f32(f220(v1), Bytes::lit(&[])) {
                f214(v2)
            } else {
                let a1480 = f214(v2)?;
                let a1481 = 1u64;
                nat_add(a1480, a1481)
            }
        }
    }
}

pub fn f215(v0: u64) -> bool {
    let a1482 = v0;
    let a1483 = 0u64;
    if nat_eq(a1482, a1483) {
        true
    } else {
        let a1484 = v0;
        let a1485 = 2u64;
        if nat_eq(a1484, a1485) {
            true
        } else {
            let a1486 = v0;
            let a1487 = 3u64;
            if nat_eq(a1486, a1487) {
                true
            } else {
                let a1488 = v0;
                let a1489 = 4u64;
                if nat_eq(a1488, a1489) {
                    true
                } else {
                    let a1490 = v0;
                    let a1491 = 9u64;
                    nat_eq(a1490, a1491)
                }
            }
        }
    }
}

pub fn f216(v0: Adt13) -> u64 {
    let m1492 = v0;
    match m1492 {
        Adt13::C0(v1, _, _, _, _, _) => {
            v1
        }
    }
}

pub fn f217(v0: List<Adt13>, v1: Adt13) -> R<Result<(Bytes, List<Bytes>), Bytes>> {
    let v2: u64 = f216(v1.clone());
    let v3: Bytes = f327(f218(v1.clone()))?;
    let a1493 = v2;
    let a1494 = 0u64;
    if if nat_eq(a1493, a1494) {
        true
    } else {
        let a1495 = v2;
        let a1496 = 2u64;
        nat_eq(a1495, a1496)
    } {
        let m1497 = f328(v0.clone(), v0.clone(), f218(v1.clone()));
        match m1497 {
            None => {
                let m1501 = f329({
                    let a1499 = {
                        let a1498 = v0.clone();
                        length_list(a1498)
                    };
                    let a1500 = 1u64;
                    nat_add(a1499, a1500)?
                }, v0, v1)?;
                match m1501 {
                    Ok(v5) => {
                        let a1503 = {
                            let a1502 = v5.clone();
                            length_list(a1502)
                        };
                        let a1504 = 0u64;
                        if nat_eq(a1503, a1504) {
                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f330(v3), List::<Bytes>::nil())))
                        } else {
                            let m1505 = f331(f332(f333(v5.clone()))?, v5)?;
                            match m1505 {
                                Ok(v7) => {
                                    Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f334(v3, {
                                        let (h1506, _) = v7;
                                        h1506
                                    }), {
                                        let (_, h1507) = v7;
                                        h1507
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
        let a1508 = v2;
        let a1509 = 3u64;
        if nat_eq(a1508, a1509) {
            let m1510 = f169(f335(v1.clone()));
            match m1510 {
                None => {
                    let m1511 = f336(f332(f335(v1.clone()))?, f337(f335(v1)), List::<Bytes>::nil())?;
                    match m1511 {
                        Ok(v10) => {
                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f338(v3, v10), List::<Bytes>::nil())))
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
            let a1512 = v2;
            let a1513 = 4u64;
            if nat_eq(a1512, a1513) {
                let v12: Adt15 = f339(v1);
                if !f32(f340(v12.clone()), Bytes::lit(&[])) {
                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f340(v12)))
                } else {
                    let a1514 = f341(v12.clone());
                    let a1515 = 1u64;
                    if nat_eq(a1514, a1515) {
                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f338(v3, List::cons(f342(v12.clone()), List::<Bytes>::nil())), f343(v12))))
                    } else {
                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f344(v3, f342(v12.clone())), f343(v12))))
                    }
                }
            } else {
                Ok(Err::<(Bytes, List<Bytes>), Bytes>(f7(f335(v1))))
            }
        }
    }
}

pub fn f218(v0: Adt13) -> Bytes {
    let m1516 = v0;
    match m1516 {
        Adt13::C0(_, v2, _, _, _, _) => {
            v2
        }
    }
}

pub fn f219(p1519: List<Adt16>, v1: Bytes) -> Option<Adt16> {
    let m1518 = p1519;
    {
        let mut m1518 = m1518;
        loop {
            let step = {
                let v0 = m1518;
                let m1517 = v0;
                match m1517.uncons() {
                    None => {
                        Err::<List<Adt16>, Option<Adt16>>(None::<Adt16>)
                    }
                    Some((v2, v3)) => {
                        if f32(f223(v2.clone()), v1.clone()) {
                            Err::<List<Adt16>, Option<Adt16>>(Some(v2))
                        } else {
                            Ok::<List<Adt16>, Option<Adt16>>(v3)
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

pub fn f220(v0: Adt16) -> Bytes {
    let m1520 = v0;
    match m1520 {
        Adt16::C0(_, v2, _, _) => {
            v2
        }
    }
}

pub fn f221(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m1521 = v0;
    match m1521.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f221(v3, v1))
        }
    }
}

pub fn f222(v0: Adt16) -> List<Bytes> {
    let m1522 = v0;
    match m1522 {
        Adt16::C0(_, _, _, v4) => {
            v4
        }
    }
}

pub fn f223(v0: Adt16) -> Bytes {
    let m1523 = v0;
    match m1523 {
        Adt16::C0(v1, _, _, _) => {
            v1
        }
    }
}

pub fn f224(v0: Adt16) -> Bytes {
    let m1524 = v0;
    match m1524 {
        Adt16::C0(_, _, v3, _) => {
            v3
        }
    }
}

pub fn f225(v0: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)> {
    let m1525 = v0;
    match m1525.uncons() {
        None => {
            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
        }
        Some((v2, v3)) => {
            let m1528 = f250({
                let (h1526, _) = {
                    let (_, h1527) = v2.clone();
                    h1527
                };
                h1526
            }, v1.clone());
            match m1528 {
                None => {
                    Some((({
                        let (h1529, _) = v2;
                        h1529
                    }, {
                        let (_, h1530) = {
                            let (_, h1531) = v2;
                            h1531
                        };
                        h1530
                    }), v3))
                }
                Some(_) => {
                    let m1532 = f225(v3, v1);
                    match m1532 {
                        None => {
                            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h1533, _) = v5;
                                h1533
                            }, List::cons(v2, {
                                let (_, h1534) = v5;
                                h1534
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f226(p1539: List<Adt13>, v1: u64, v2: Bytes) -> Option<Adt13> {
    let m1538 = p1539;
    {
        let mut m1538 = m1538;
        loop {
            let step = {
                let v0 = m1538;
                let m1535 = v0;
                match m1535.uncons() {
                    None => {
                        Err::<List<Adt13>, Option<Adt13>>(None::<Adt13>)
                    }
                    Some((v3, v4)) => {
                        let a1536 = f216(v3.clone());
                        let a1537 = v1;
                        if if nat_eq(a1536, a1537) {
                            f32(f218(v3.clone()), v2.clone())
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
                Ok(next) => { m1538 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f227(v0: u64) -> Bytes {
    let a1540 = v0;
    let a1541 = 5u64;
    if nat_eq(a1540, a1541) {
        f8(Str::lit("partial interface "))
    } else {
        let a1542 = v0;
        let a1543 = 7u64;
        if nat_eq(a1542, a1543) {
            f8(Str::lit("partial dictionary "))
        } else {
            f8(Str::lit(""))
        }
    }
}

pub fn f228(v0: Adt13) -> Bytes {
    let m1544 = v0;
    match m1544 {
        Adt13::C0(_, _, v3, _, _, _) => {
            v3
        }
    }
}

pub fn f229(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f345(v0, v1)
}

pub fn f230(v0: u64) -> R<bool> {
    f346(v0, f347())
}

pub fn f231(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let v3: u64 = {
        let (h1545, _) = f201(Fn5::F348(v1.clone(), v0.clone()), {
            let a1547 = {
                let a1546 = v0.clone();
                length_bytes(a1546)
            };
            let a1548 = 1u64;
            nat_add(a1547, a1548)?
        }, v2)?;
        h1545
    };
    if f133(v0, v1, v3) {
        Ok(Some(v3))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f232(v0: Bytes) -> Result<u64, Bytes> {
    Err::<u64, Bytes>(v0)
}

pub fn f233(v0: Bytes, v1: u64) -> R<u64> {
    let (h1549, _) = f201(Fn5::F349(v0.clone()), {
        let a1551 = {
            let a1550 = v0;
            length_bytes(a1550)
        };
        let a1552 = 1u64;
        nat_add(a1551, a1552)?
    }, v1)?;
    Ok(h1549)
}

pub fn f234(v0: Bytes) -> bool {
    f320(v0, List::cons(f8(Str::lit("UTF-8")), List::cons(f8(Str::lit("UTf-8")), List::cons(f8(Str::lit("UtF-8")), List::cons(f8(Str::lit("Utf-8")), List::cons(f8(Str::lit("uTF-8")), List::cons(f8(Str::lit("uTf-8")), List::cons(f8(Str::lit("utF-8")), List::cons(f8(Str::lit("utf-8")), List::<Bytes>::nil())))))))))
}

pub fn f235(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let m1556 = f231(v0.clone(), {
        let a1553 = Str::lit("--");
        utf8_encode(a1553)
    }, {
        let a1554 = v1;
        let a1555 = 4u64;
        nat_add(a1554, a1555)?
    })?;
    match m1556 {
        None => {
            Ok(f232(f137(Str::lit("a comment is not closed (XML 1.0 [15])"), v1)?))
        }
        Some(v2) => {
            let a1559 = f160(v0, {
                let a1557 = v2;
                let a1558 = 2u64;
                nat_add(a1557, a1558)?
            })?;
            let a1560 = 62u64;
            if nat_eq(a1559, a1560) {
                Ok(Ok::<u64, Bytes>({
                    let a1561 = v2;
                    let a1562 = 3u64;
                    nat_add(a1561, a1562)?
                }))
            } else {
                Ok(f232(f137(Str::lit("-- is not allowed inside a comment (XML 1.0 [15])"), v2)?))
            }
        }
    }
}

pub fn f236(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f238(v0.clone(), {
        let a1563 = v1;
        let a1564 = 2u64;
        nat_add(a1563, a1564)?
    })?;
    let a1567 = v2;
    let a1568 = {
        let a1565 = v1;
        let a1566 = 2u64;
        nat_add(a1565, a1566)?
    };
    if nat_eq(a1567, a1568) {
        Ok(f232(f137(Str::lit("a processing instruction must start with a Name (XML 1.0 [16])"), v1)?))
    } else {
        let a1571 = v2;
        let a1572 = {
            let a1569 = v1;
            let a1570 = 5u64;
            nat_add(a1569, a1570)?
        };
        if if nat_eq(a1571, a1572) {
            f320(f46(v0.clone(), {
                let a1573 = v1;
                let a1574 = 2u64;
                nat_add(a1573, a1574)?
            }, 3u64), List::cons(f8(Str::lit("XML")), List::cons(f8(Str::lit("XMl")), List::cons(f8(Str::lit("XmL")), List::cons(f8(Str::lit("Xml")), List::cons(f8(Str::lit("xML")), List::cons(f8(Str::lit("xMl")), List::cons(f8(Str::lit("xmL")), List::cons(f8(Str::lit("xml")), List::<Bytes>::nil())))))))))
        } else {
            false
        } {
            Ok(f232(f137(Str::lit("the target xml is reserved; an XML declaration may appear only at the very start (XML 1.0 [17], [22])"), v1)?))
        } else {
            let m1576 = f231(v0, {
                let a1575 = Str::lit("?>");
                utf8_encode(a1575)
            }, v2)?;
            match m1576 {
                None => {
                    Ok(f232(f137(Str::lit("a processing instruction is not closed (XML 1.0 [16])"), v1)?))
                }
                Some(v3) => {
                    Ok(Ok::<u64, Bytes>({
                        let a1577 = v3;
                        let a1578 = 2u64;
                        nat_add(a1577, a1578)?
                    }))
                }
            }
        }
    }
}

pub fn f237(v0: Bytes) -> Result<(Adt17, u64), Bytes> {
    Err::<(Adt17, u64), Bytes>(v0)
}

pub fn f238(v0: Bytes, v1: u64) -> R<u64> {
    let (h1579, _) = f201(Fn5::F350(v0.clone(), v1), {
        let a1581 = {
            let a1580 = v0;
            length_bytes(a1580)
        };
        let a1582 = 1u64;
        nat_add(a1581, a1582)?
    }, v1)?;
    Ok(h1579)
}

pub fn f239(v0: Bytes, v1: u64, v2: u64, v3: List<Bytes>) -> R<Result<(Adt17, u64), Bytes>> {
    let m1583 = v2;
    if m1583 == 0 {
        Ok(f237(f137(Str::lit("a tag has more attributes than the input has bytes"), v1)?))
    } else {
        let v4 = m1583 - 1;
        let v5: u64 = f233(v0.clone(), v1)?;
        let a1584 = f160(v0.clone(), v5)?;
        let a1585 = 62u64;
        if if nat_eq(a1584, a1585) {
            true
        } else {
            f159(v0.clone(), v5, Str::lit("/>"))
        } {
            Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C4, v5)))
        } else {
            let a1586 = v5;
            let a1587 = v1;
            if nat_eq(a1586, a1587) {
                Ok(f237(f137(Str::lit("attributes must be separated by white space (XML 1.0 [40])"), v1)?))
            } else {
                let v6: u64 = f238(v0.clone(), v5)?;
                let a1588 = v6;
                let a1589 = v5;
                if nat_eq(a1588, a1589) {
                    Ok(f237(f137(Str::lit("an attribute must start with a Name (XML 1.0 [41])"), v5)?))
                } else {
                    let v7: Bytes = f46(v0.clone(), v5, {
                        let a1590 = v6;
                        let a1591 = v5;
                        nat_sub(a1590, a1591)
                    });
                    if f320(v7.clone(), v3.clone()) {
                        Ok(f237(f137(Str::lit("an attribute name appears twice in one tag (XML 1.0 WFC: Unique Att Spec)"), v5)?))
                    } else {
                        let v8: u64 = f233(v0.clone(), v6)?;
                        let a1592 = f160(v0.clone(), v8)?;
                        let a1593 = 61u64;
                        if nat_eq(a1592, a1593) {
                            let v9: u64 = f233(v0.clone(), {
                                let a1594 = v8;
                                let a1595 = 1u64;
                                nat_add(a1594, a1595)?
                            })?;
                            let v10: u64 = f160(v0.clone(), v9)?;
                            let a1596 = v10;
                            let a1597 = 34u64;
                            if if nat_eq(a1596, a1597) {
                                true
                            } else {
                                let a1598 = v10;
                                let a1599 = 39u64;
                                nat_eq(a1598, a1599)
                            } {
                                let m1602 = f240(v0.clone(), {
                                    let a1600 = v9;
                                    let a1601 = 1u64;
                                    nat_add(a1600, a1601)?
                                }, v4, Bytes::lit(&[]), true, v10)?;
                                match m1602 {
                                    Ok(v12) => {
                                        let m1604 = f239(v0, {
                                            let (_, h1603) = v12;
                                            h1603
                                        }, v4, List::cons(v7.clone(), v3))?;
                                        match m1604 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C1(v7, {
                                                    let (h1605, _) = v12;
                                                    h1605
                                                }, std::rc::Rc::new({
                                                    let (h1606, _) = v14;
                                                    h1606
                                                })), {
                                                    let (_, h1607) = v14;
                                                    h1607
                                                })))
                                            }
                                            Err(v13) => {
                                                Ok(f237(v13))
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(f237(v11))
                                    }
                                }
                            } else {
                                Ok(f237(f137(Str::lit("an attribute value must be quoted (XML 1.0 [10])"), v9)?))
                            }
                        } else {
                            Ok(f237(f137(Str::lit("an attribute needs = and a value (XML 1.0 [25], [41])"), v8)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f240(v0: Bytes, p1647: u64, p1648: u64, p1649: Bytes, v4: bool, v5: u64) -> R<Result<(Bytes, u64), Bytes>> {
    let m1646 = (p1647, (p1648, p1649));
    {
        let mut m1646 = m1646;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1646;
                let m1608 = v2;
                if m1608 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f351(f137(Str::lit("the input ends inside a text run"), v1)?)))
                } else {
                    let v6 = m1608 - 1;
                    let v7: u64 = f160(v0.clone(), v1)?;
                    if if v4 {
                        let a1609 = v7;
                        let a1610 = v5;
                        nat_eq(a1609, a1610)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, {
                            let a1611 = v1;
                            let a1612 = 1u64;
                            nat_add(a1611, a1612)?
                        }))))
                    } else {
                        let a1613 = v7;
                        let a1614 = 256u64;
                        if nat_eq(a1613, a1614) {
                            if v4 {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f351(f137(Str::lit("an attribute value is not closed (XML 1.0 [10])"), v1)?)))
                            } else {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                            }
                        } else {
                            let a1615 = v7;
                            let a1616 = 60u64;
                            if nat_eq(a1615, a1616) {
                                if v4 {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f351(f137(Str::lit("< is not allowed in an attribute value (XML 1.0 [10], WFC: No < in Attribute Values)"), v1)?)))
                                } else {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                                }
                            } else {
                                let a1617 = v7;
                                let a1618 = 38u64;
                                if nat_eq(a1617, a1618) {
                                    let m1619 = f352(v0.clone(), v1)?;
                                    match m1619 {
                                        Ok(v9) => {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let (_, h1620) = v9;
                                                h1620
                                            }, (v6, {
                                                let a1622 = v3;
                                                let a1623 = {
                                                    let (h1621, _) = v9;
                                                    h1621
                                                };
                                                append_bytes(a1622, a1623)
                                            })))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f351(v8)))
                                        }
                                    }
                                } else {
                                    if v4 {
                                        let a1624 = v7;
                                        let a1625 = 9u64;
                                        if if nat_eq(a1624, a1625) {
                                            true
                                        } else {
                                            let a1626 = v7;
                                            let a1627 = 10u64;
                                            nat_eq(a1626, a1627)
                                        } {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let a1628 = v1;
                                                let a1629 = 1u64;
                                                nat_add(a1628, a1629)?
                                            }, (v6, {
                                                let a1630 = v3;
                                                let a1631 = f8(Str::lit(" "));
                                                append_bytes(a1630, a1631)
                                            })))
                                        } else {
                                            let v10: u64 = f353(v0.clone(), true, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v10, (v6, {
                                                let a1634 = v3;
                                                let a1635 = f46(v0.clone(), v1, {
                                                    let a1632 = v10;
                                                    let a1633 = v1;
                                                    nat_sub(a1632, a1633)
                                                });
                                                append_bytes(a1634, a1635)
                                            })))
                                        }
                                    } else {
                                        let a1636 = v7;
                                        let a1637 = 93u64;
                                        if nat_eq(a1636, a1637) {
                                            if f159(v0.clone(), v1, Str::lit("]]>")) {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f351(f137(Str::lit("]]> is not allowed in character data (XML 1.0 [14])"), v1)?)))
                                            } else {
                                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                    let a1638 = v1;
                                                    let a1639 = 1u64;
                                                    nat_add(a1638, a1639)?
                                                }, (v6, {
                                                    let a1640 = v3;
                                                    let a1641 = f8(Str::lit("]"));
                                                    append_bytes(a1640, a1641)
                                                })))
                                            }
                                        } else {
                                            let v11: u64 = f353(v0.clone(), false, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v11, (v6, {
                                                let a1644 = v3;
                                                let a1645 = f46(v0.clone(), v1, {
                                                    let a1642 = v11;
                                                    let a1643 = v1;
                                                    nat_sub(a1642, a1643)
                                                });
                                                append_bytes(a1644, a1645)
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
                Ok(next) => { m1646 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f241(v0: Bytes) -> Result<List<(Bytes, Bytes)>, Bytes> {
    Err::<List<(Bytes, Bytes)>, Bytes>(v0)
}

pub fn f242() -> Bytes {
    f8(Str::lit("http://www.w3.org/XML/1998/namespace"))
}

pub fn f243() -> Bytes {
    f8(Str::lit("http://www.w3.org/2000/xmlns/"))
}

pub fn f244(v0: Bytes) -> R<Bytes> {
    let m1650 = f231(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1650 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v1) => {
            Ok(f46(v0, 0u64, v1))
        }
    }
}

pub fn f245(p1655: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1654 = p1655;
    {
        let mut m1654 = m1654;
        loop {
            let step = {
                let v0 = m1654;
                let m1651 = v0;
                match m1651.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h1652, _) = v2;
                            h1652
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h1653) = v2;
                                h1653
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1654 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f246(v0: Bytes) -> R<Bytes> {
    let m1656 = f231(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1656 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(f46(v0.clone(), {
                let a1657 = v1;
                let a1658 = 1u64;
                nat_add(a1657, a1658)?
            }, {
                let a1662 = {
                    let a1659 = v0;
                    length_bytes(a1659)
                };
                let a1663 = {
                    let a1660 = v1;
                    let a1661 = 1u64;
                    nat_add(a1660, a1661)?
                };
                nat_sub(a1662, a1663)
            }))
        }
    }
}

pub fn f247() -> Bytes {
    f8(Str::lit("http://www.w3.org/2001/XMLSchema"))
}

pub fn f248(v0: Bytes) -> Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes> {
    Err::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v0)
}

pub fn f249(v0: Bytes, v1: Adt17, v2: List<(Bytes, Bytes)>, v3: Bytes) -> R<Result<Adt24, Bytes>> {
    let v4: Bytes = f125(v0.clone())?;
    let m1664 = f354(v1, v2)?;
    match m1664 {
        Ok(v6) => {
            let m1665 = v6;
            match m1665 {
                None => {
                    Ok(f355(f356(f8(Str::lit("a simpleType must have a restriction, list or union child (XSD 1.1 Part 2, section 4.1.2)")), v0)))
                }
                Some(v7) => {
                    let v8: Bytes = match v7.clone() {
                        Adt31::C0(h1666, _, _, _) => {
                            h1666
                        }
                    };
                    let v9: Adt17 = match v7.clone() {
                        Adt31::C0(_, h1667, _, _) => {
                            h1667
                        }
                    };
                    let v10: Adt17 = match v7.clone() {
                        Adt31::C0(_, _, h1668, _) => {
                            h1668
                        }
                    };
                    let v11: List<(Bytes, Bytes)> = match v7 {
                        Adt31::C0(_, _, _, h1669) => {
                            h1669
                        }
                    };
                    if f32(v8.clone(), f8(Str::lit("union"))) {
                        Ok(f355(f356(f8(Str::lit("a union is not translated, because a literal can be valid for more than one member type, so no LexLean inductive keeps its members disjoint (XSD 1.1 Part 2, section 2.4.1.3)")), v0)))
                    } else {
                        if f32(v8, f8(Str::lit("list"))) {
                            let m1670 = f166(v9, f8(Str::lit("itemType")));
                            match m1670 {
                                None => {
                                    Ok(f355(f356(f8(Str::lit("a list with an anonymous item type is not translated yet")), v0)))
                                }
                                Some(v12) => {
                                    let m1671 = f357(v11, v3, v12)?;
                                    match m1671 {
                                        Ok(v14) => {
                                            if f53(match v14.clone() {
                                                Adt24::C0(h1672, _) => {
                                                    h1672
                                                }
                                            }, f8(Str::lit("(List "))) {
                                                Ok(f355(f356(f8(Str::lit("the item type of a list must not itself be a list (XSD 1.1 Part 2, section 4.1.5)")), v0)))
                                            } else {
                                                Ok(Ok::<Adt24, Bytes>(Adt24::C0(f344(v4, f7(List::cons(f8(Str::lit("(List ")), List::cons(match v14.clone() {
                                                    Adt24::C0(h1673, _) => {
                                                        h1673
                                                    }
                                                }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))), match v14 {
                                                    Adt24::C0(_, h1674) => {
                                                        h1674
                                                    }
                                                })))
                                            }
                                        }
                                        Err(v13) => {
                                            Ok(f355(f356(v13, v0)))
                                        }
                                    }
                                }
                            }
                        } else {
                            let m1675 = f166(v9, f8(Str::lit("base")));
                            match m1675 {
                                None => {
                                    Ok(f355(f356(f8(Str::lit("a restriction with an anonymous base type is not translated yet")), v0)))
                                }
                                Some(v15) => {
                                    let m1676 = f357(v11.clone(), v3, v15)?;
                                    match m1676 {
                                        Ok(v17) => {
                                            let m1677 = f358(v10, v11)?;
                                            match m1677 {
                                                Ok(v19) => {
                                                    let v20: List<Adt2> = f359(v19, List::<Bytes>::nil())?;
                                                    let m1678 = v20.clone();
                                                    match m1678.uncons() {
                                                        None => {
                                                            Ok(Ok::<Adt24, Bytes>(Adt24::C0(f344(v4, match v17.clone() {
                                                                Adt24::C0(h1679, _) => {
                                                                    h1679
                                                                }
                                                            }), match v17 {
                                                                Adt24::C0(_, h1680) => {
                                                                    h1680
                                                                }
                                                            })))
                                                        }
                                                        Some((_, _)) => {
                                                            let m1681 = f336(f332(f75(v20.clone()))?, v20, List::<Bytes>::nil())?;
                                                            match m1681 {
                                                                Ok(v24) => {
                                                                    Ok(Ok::<Adt24, Bytes>(Adt24::C0(f338(v4, v24), List::<Bytes>::nil())))
                                                                }
                                                                Err(v23) => {
                                                                    Ok(f355(f356(v23, v0)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Err(v18) => {
                                                    Ok(f355(f356(v18, v0)))
                                                }
                                            }
                                        }
                                        Err(v16) => {
                                            Ok(f355(f356(v16, v0)))
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
            Ok(f355(f356(v5, v0)))
        }
    }
}

pub fn f250(p1684: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m1683 = p1684;
    {
        let mut m1683 = m1683;
        loop {
            let step = {
                let v0 = m1683;
                let m1682 = v0;
                match m1682.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f122(v2.clone(), v1.clone()) {
                            Ok::<List<Bytes>, Option<Bytes>>(v3)
                        } else {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1683 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f251(v0: Bytes) -> Result<List<Bytes>, Bytes> {
    Err::<List<Bytes>, Bytes>(v0)
}

pub fn f252(v0: List<Bytes>, v1: Option<Bytes>) -> R<List<(Bytes, Bytes)>> {
    let m1685 = v0;
    match m1685.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, v3)) => {
            if f32(v2.clone(), f8(Str::lit("[[authority]]"))) {
                f252(v3, None::<Bytes>)
            } else {
                let m1686 = f360(v2.clone(), f8(Str::lit("url = ")))?;
                match m1686 {
                    None => {
                        let m1687 = f360(v2, f8(Str::lit("artifact = ")))?;
                        match m1687 {
                            None => {
                                f252(v3, v1)
                            }
                            Some(v5) => {
                                let m1688 = v1.clone();
                                match m1688 {
                                    None => {
                                        f252(v3, v1)
                                    }
                                    Some(v6) => {
                                        Ok(List::cons((v6, v5), f252(v3, v1)?))
                                    }
                                }
                            }
                        }
                    }
                    Some(v4) => {
                        f252(v3, Some(v4))
                    }
                }
            }
        }
    }
}

pub fn f253(v0: List<Adt19>) -> R<Result<(u64, Bytes), Bytes>> {
    let m1689 = f361(v0.clone())?;
    match m1689 {
        Ok(v2) => {
            let v3: Adt25 = Adt25::C0(v2.clone(), f362(v0.clone()));
            let m1690 = f363(f364(v0)?, v3, v2)?;
            match m1690 {
                Ok(v5) => {
                    let m1691 = f365(f366(v5), List::<Adt28>::nil());
                    match m1691 {
                        Ok(v7) => {
                            let v8: List<Bytes> = f367(v7.clone());
                            let m1692 = f368(v7.clone(), v8.clone());
                            match m1692 {
                                None => {
                                    let v10: List<(Bytes, List<Bytes>)> = f369(v7.clone(), v8.clone(), f370(v7.clone())?);
                                    let m1698 = f371({
                                        let a1696 = {
                                            let a1695 = v8.clone();
                                            length_list(a1695)
                                        };
                                        let a1697 = 1u64;
                                        nat_add(a1696, a1697)?
                                    }, v7.clone(), f372(v7, f373(v10, v8.clone(), v8, List::<Bytes>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil());
                                    match m1698 {
                                        Ok(v12) => {
                                            Ok(Ok::<(u64, Bytes), Bytes>(({
                                                let a1699 = v12.clone();
                                                length_list(a1699)
                                            }, f153(v12, f8(Str::lit("\u{a}"))))))
                                        }
                                        Err(v11) => {
                                            Ok(Err::<(u64, Bytes), Bytes>(v11))
                                        }
                                    }
                                }
                                Some(v9) => {
                                    Ok(Err::<(u64, Bytes), Bytes>(f257(f8(Str::lit("the type ")), {
                                        let (h1693, _) = v9;
                                        h1693
                                    }, f257(f8(Str::lit(" uses ")), {
                                        let (_, h1694) = v9;
                                        h1694
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

pub fn f254(p1702: List<Adt19>, p1703: List<Adt19>) -> List<Adt19> {
    let m1701 = (p1702, p1703);
    {
        let mut m1701 = m1701;
        loop {
            let step = {
                let (v0, v1) = m1701;
                let m1700 = v0;
                match m1700.uncons() {
                    None => {
                        Err::<(List<Adt19>, List<Adt19>), List<Adt19>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt19>, List<Adt19>), List<Adt19>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1701 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f255(v0: List<Adt19>) -> List<Bytes> {
    let m1704 = v0;
    match m1704.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt19::C0(h1705, _, _, _, _, _) => {
                    h1705
                }
            }, f255(v2))
        }
    }
}

pub fn f256(p1710: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m1709 = p1710;
    {
        let mut m1709 = m1709;
        loop {
            let step = {
                let v0 = m1709;
                let m1706 = v0;
                match m1706.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h1707, _) = v2;
                            h1707
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h1708) = v2;
                                h1708
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
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

pub fn f257(v0: Bytes, v1: Bytes, v2: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(v1, List::cons(v2, List::<Bytes>::nil()))))
}

pub fn f258(v0: Bytes, v1: Bytes, v2: Option<Bytes>) -> R<Result<Adt19, Bytes>> {
    let m1711 = f95(v1.clone())?;
    match m1711 {
        Ok(v4) => {
            let m1712 = v4;
            match m1712 {
                Adt17::C0(v5, r1717, r1718) => {
                    let v6 = (*r1717).clone();
                    let v7 = (*r1718).clone();
                    let m1713 = f163(v6.clone(), List::<(Bytes, Bytes)>::nil());
                    match m1713 {
                        Ok(v9) => {
                            let m1714 = f164(v9.clone(), v5)?;
                            match m1714 {
                                Ok(v11) => {
                                    if f165(v11, Str::lit("schema")) {
                                        Ok(Ok::<Adt19, Bytes>(Adt19::C0(v0, v2, {
                                            let m1715 = f166(v6, f8(Str::lit("targetNamespace")));
                                            match m1715 {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some(v12) => {
                                                    v12
                                                }
                                            }
                                        }, v9, v7, {
                                            let a1716 = v1;
                                            length_bytes(a1716)
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
            Ok(Err::<Adt19, Bytes>(f257(v0, f8(Str::lit(" is not well-formed XML: ")), v3)))
        }
    }
}

pub fn f259(p1723: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1722 = p1723;
    {
        let mut m1722 = m1722;
        loop {
            let step = {
                let v0 = m1722;
                let m1719 = v0;
                match m1719.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (_, h1720) = v2;
                            h1720
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (h1721, _) = v2;
                                h1721
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1722 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f260(v0: Adt19, v1: Bytes, v2: Bytes) -> Option<Bytes> {
    if f32(v1.clone(), f8(Str::lit("root"))) {
        None::<Bytes>
    } else {
        if f32(match v0.clone() {
            Adt19::C0(_, _, h1724, _, _, _) => {
                h1724
            }
        }, v2.clone()) {
            None::<Bytes>
        } else {
            Some(f7(List::cons(match v0.clone() {
                Adt19::C0(h1725, _, _, _, _, _) => {
                    h1725
                }
            }, List::cons(f8(Str::lit(" has the target namespace ")), List::cons(match v0 {
                Adt19::C0(_, _, h1726, _, _, _) => {
                    h1726
                }
            }, List::cons(f8(Str::lit(", but the xs:")), List::cons(v1.clone(), List::cons(f8(Str::lit(" that names it requires ")), List::cons(v2, List::cons(if f32(v1, f8(Str::lit("include"))) {
                f8(Str::lit("; an included document without a target namespace is not adopted into the including one (XSD 1.1 Part 1, section 4.2.3)"))
            } else {
                f8(Str::lit(" (XSD 1.1 Part 1, section 4.2.6.2)"))
            }, List::<Bytes>::nil()))))))))))
        }
    }
}

pub fn f261(v0: Adt17, v1: List<(Bytes, Bytes)>) -> R<Result<List<Adt27>, Bytes>> {
    let m1727 = v0;
    match m1727 {
        Adt17::C0(_, _, _) => {
            Ok(Ok::<List<Adt27>, Bytes>(List::<Adt27>::nil()))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<List<Adt27>, Bytes>(List::<Adt27>::nil()))
        }
        Adt17::C2(_) => {
            Ok(Ok::<List<Adt27>, Bytes>(List::<Adt27>::nil()))
        }
        Adt17::C3(r1738, r1739) => {
            let v2 = (*r1738).clone();
            let v3 = (*r1739).clone();
            let m1728 = f261(v3, v1.clone())?;
            match m1728 {
                Ok(v5) => {
                    let m1729 = v2;
                    match m1729 {
                        Adt17::C0(v6, r1736, r1737) => {
                            let v7 = (*r1736).clone();
                            let v8 = (*r1737).clone();
                            let m1730 = f163(v7.clone(), v1);
                            match m1730 {
                                Ok(v10) => {
                                    let m1731 = f164(v10.clone(), v6)?;
                                    match m1731 {
                                        Ok(v12) => {
                                            if if f32({
                                                let (h1732, _) = v12.clone();
                                                h1732
                                            }, f247()) {
                                                f374(f32({
                                                    let (_, h1733) = v12.clone();
                                                    h1733
                                                }, f8(Str::lit("annotation"))))
                                            } else {
                                                false
                                            } {
                                                Ok(Ok::<List<Adt27>, Bytes>(List::cons(Adt27::C0({
                                                    let (h1734, _) = v12;
                                                    h1734
                                                }, {
                                                    let (_, h1735) = v12;
                                                    h1735
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

pub fn f262(v0: List<Adt27>, v1: Adt19, v2: List<(Bytes, Bytes)>) -> R<Result<List<(Bytes, (Bytes, Bytes))>, Bytes>> {
    let m1740 = v0;
    match m1740.uncons() {
        None => {
            Ok(Ok::<List<(Bytes, (Bytes, Bytes))>, Bytes>(List::<(Bytes, (Bytes, Bytes))>::nil()))
        }
        Some((v3, v4)) => {
            let m1741 = f262(v4, v1.clone(), v2.clone())?;
            match m1741 {
                Ok(v6) => {
                    if if f375(v3.clone(), Str::lit("redefine")) {
                        true
                    } else {
                        if f375(v3.clone(), Str::lit("override")) {
                            true
                        } else {
                            f375(v3.clone(), Str::lit("defaultOpenContent"))
                        }
                    } {
                        Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                            Adt19::C0(h1742, _, _, _, _, _) => {
                                h1742
                            }
                        }, List::cons(f8(Str::lit(": xs:")), List::cons(match v3 {
                            Adt27::C0(_, h1743, _, _, _) => {
                                h1743
                            }
                        }, List::cons(f8(Str::lit(" is not translated, because it changes components that other schema documents define (XSD 1.1 Part 1, sections 4.2.4, 4.2.5 and 3.4.2.3)")), List::<Bytes>::nil())))))))
                    } else {
                        if if f375(v3.clone(), Str::lit("import")) {
                            true
                        } else {
                            f375(v3.clone(), Str::lit("include"))
                        } {
                            let m1744 = f376(v3.clone(), Str::lit("schemaLocation"));
                            match m1744 {
                                None => {
                                    Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                                        Adt19::C0(h1745, _, _, _, _, _) => {
                                            h1745
                                        }
                                    }, List::cons(f8(Str::lit(": an xs:")), List::cons(match v3 {
                                        Adt27::C0(_, h1746, _, _, _) => {
                                            h1746
                                        }
                                    }, List::cons(f8(Str::lit(" with no schemaLocation names no document whose components could be read (XSD 1.1 Part 1, section 4.2.6.2)")), List::<Bytes>::nil())))))))
                                }
                                Some(v7) => {
                                    let m1748 = f377(match v1.clone() {
                                        Adt19::C0(_, h1747, _, _, _, _) => {
                                            h1747
                                        }
                                    }, v7)?;
                                    match m1748 {
                                        Ok(v9) => {
                                            let m1750 = f378(v2, v9.clone());
                                            match m1750 {
                                                None => {
                                                    Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                                                        Adt19::C0(h1751, _, _, _, _, _) => {
                                                            h1751
                                                        }
                                                    }, List::cons(f8(Str::lit(": the schema document ")), List::cons(v9, List::cons(f8(Str::lit(" is not a bound authority, so its components cannot be read; stage its binding first (XSD 1.1 Part 1, section 4.2.6.2)")), List::<Bytes>::nil())))))))
                                                }
                                                Some(v10) => {
                                                    Ok(Ok::<List<(Bytes, (Bytes, Bytes))>, Bytes>(List::cons((v10, (match v3.clone() {
                                                        Adt27::C0(_, h1752, _, _, _) => {
                                                            h1752
                                                        }
                                                    }, if f375(v3.clone(), Str::lit("include")) {
                                                        match v1 {
                                                            Adt19::C0(_, _, h1753, _, _, _) => {
                                                                h1753
                                                            }
                                                        }
                                                    } else {
                                                        f379(v3, Str::lit("namespace"), Str::lit(""))
                                                    })), v6)))
                                                }
                                            }
                                        }
                                        Err(v8) => {
                                            Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f257(match v1 {
                                                Adt19::C0(h1749, _, _, _, _, _) => {
                                                    h1749
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

pub fn f263(v0: List<(Bytes, (Bytes, Bytes))>, v1: List<(Bytes, (Bytes, Bytes))>) -> List<(Bytes, (Bytes, Bytes))> {
    let m1754 = v0;
    match m1754.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f263(v3, v1))
        }
    }
}

pub fn f264(v0: List<(Bytes, Option<Bytes>)>, v1: List<(Bytes, Option<Bytes>)>) -> List<(Bytes, Option<Bytes>)> {
    let m1755 = v0;
    match m1755.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f264(v3, v1))
        }
    }
}

pub fn f265(v0: Adt2) -> Option<Bytes> {
    let m1756 = v0;
    match m1756 {
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
        Adt2::C6(r1759) => {
            let v3 = (*r1759).clone();
            f265(v3)
        }
        Adt2::C7(r1760) => {
            let v4 = (*r1760).clone();
            f265(v4)
        }
        Adt2::C8 => {
            None::<Bytes>
        }
        Adt2::C9(r1761, r1762) => {
            let v5 = (*r1761).clone();
            let v6 = (*r1762).clone();
            let m1757 = f265(v5);
            match m1757 {
                None => {
                    f265(v6)
                }
                Some(v7) => {
                    Some(v7)
                }
            }
        }
        Adt2::C10(_, r1763, r1764) => {
            let v9 = (*r1763).clone();
            let v10 = (*r1764).clone();
            let m1758 = f265(v9);
            match m1758 {
                None => {
                    f265(v10)
                }
                Some(v11) => {
                    Some(v11)
                }
            }
        }
    }
}

pub fn f266(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f111(v0.clone(), {
        let a1765 = v1;
        let a1766 = 1u64;
        nat_add(a1765, a1766)?
    })?;
    let v4: Bytes = {
        let a1767 = v3;
        let a1768 = 117u64;
        if nat_eq(a1767, a1768) {
            f380(v0, v1)?
        } else {
            f381(v3)
        }
    };
    let a1769 = v4.clone();
    let a1770 = Bytes::lit(&[]);
    if equal(a1769, a1770) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a1771 = v3;
            let a1772 = 117u64;
            if nat_eq(a1771, a1772) {
                let a1773 = v1;
                let a1774 = 6u64;
                nat_add(a1773, a1774)?
            } else {
                let a1775 = v1;
                let a1776 = 2u64;
                nat_add(a1775, a1776)?
            }
        }, ({
            let a1777 = v2;
            let a1778 = v4;
            append_bytes(a1777, a1778)
        }, 0u64))))
    }
}

pub fn f267(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f382(v0, v1)
}

pub fn f268(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f383(v0, v1)
}

pub fn f269(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1779 = v0.clone();
        length_bytes(a1779)
    };
    let a1780 = 0u64;
    let a1781 = v1;
    if nat_lt(a1780, a1781) {
        let a1782 = f384(v0.clone())?;
        let a1783 = v1;
        if nat_eq(a1782, a1783) {
            let a1784 = 1u64;
            let a1785 = v1;
            if nat_lt(a1784, a1785) {
                let m1788 = {
                    let a1786 = v0;
                    let a1787 = 0u64;
                    index_bytes(a1786, a1787)
                };
                match m1788 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1789 = v2;
                        let a1790 = 48u8;
                        Ok(!equal(a1789, a1790))
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

pub fn f270(v0: Fn6, p1795: u64, p1796: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1794 = (p1795, p1796);
    {
        let mut m1794 = m1794;
        loop {
            let step = {
                let (v1, v2) = m1794;
                let m1791 = v1;
                if m1791 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1791 - 1;
                    let m1793 = {
                        let c1792: Fn6 = v0.clone();
                        c1792.apply(v2)?
                    };
                    match m1793 {
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
                Ok(next) => { m1794 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f271(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f385(v0, v1)
}

pub fn f272(v0: Bytes) -> R<u64> {
    let (h1797, _) = f201(Fn5::F386(v0.clone()), {
        let a1799 = {
            let a1798 = v0;
            length_bytes(a1798)
        };
        let a1800 = 1u64;
        nat_add(a1799, a1800)?
    }, 0u64)?;
    Ok(h1797)
}

pub fn f273(v0: u64) -> Bytes {
    let m1801 = v0;
    if m1801 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1801 - 1;
        let a1802 = f8(Str::lit("0"));
        let a1803 = f273(v1);
        append_bytes(a1802, a1803)
    }
}

pub fn f274(v0: u32) -> Bytes {
    let a1806 = {
        let a1804 = v0;
        let a1805 = 8u32;
        fixed_u32::bit_and(a1804, a1805)
    };
    let a1807 = 0u32;
    if equal(a1806, a1807) {
        let a1810 = {
            let a1808 = v0;
            let a1809 = 4u32;
            fixed_u32::bit_and(a1808, a1809)
        };
        let a1811 = 0u32;
        if equal(a1810, a1811) {
            let a1814 = {
                let a1812 = v0;
                let a1813 = 2u32;
                fixed_u32::bit_and(a1812, a1813)
            };
            let a1815 = 0u32;
            if equal(a1814, a1815) {
                let a1818 = {
                    let a1816 = v0;
                    let a1817 = 1u32;
                    fixed_u32::bit_and(a1816, a1817)
                };
                let a1819 = 0u32;
                if equal(a1818, a1819) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a1822 = {
                    let a1820 = v0;
                    let a1821 = 1u32;
                    fixed_u32::bit_and(a1820, a1821)
                };
                let a1823 = 0u32;
                if equal(a1822, a1823) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a1826 = {
                let a1824 = v0;
                let a1825 = 2u32;
                fixed_u32::bit_and(a1824, a1825)
            };
            let a1827 = 0u32;
            if equal(a1826, a1827) {
                let a1830 = {
                    let a1828 = v0;
                    let a1829 = 1u32;
                    fixed_u32::bit_and(a1828, a1829)
                };
                let a1831 = 0u32;
                if equal(a1830, a1831) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a1834 = {
                    let a1832 = v0;
                    let a1833 = 1u32;
                    fixed_u32::bit_and(a1832, a1833)
                };
                let a1835 = 0u32;
                if equal(a1834, a1835) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a1838 = {
            let a1836 = v0;
            let a1837 = 4u32;
            fixed_u32::bit_and(a1836, a1837)
        };
        let a1839 = 0u32;
        if equal(a1838, a1839) {
            let a1842 = {
                let a1840 = v0;
                let a1841 = 2u32;
                fixed_u32::bit_and(a1840, a1841)
            };
            let a1843 = 0u32;
            if equal(a1842, a1843) {
                let a1846 = {
                    let a1844 = v0;
                    let a1845 = 1u32;
                    fixed_u32::bit_and(a1844, a1845)
                };
                let a1847 = 0u32;
                if equal(a1846, a1847) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a1850 = {
                    let a1848 = v0;
                    let a1849 = 1u32;
                    fixed_u32::bit_and(a1848, a1849)
                };
                let a1851 = 0u32;
                if equal(a1850, a1851) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a1854 = {
                let a1852 = v0;
                let a1853 = 2u32;
                fixed_u32::bit_and(a1852, a1853)
            };
            let a1855 = 0u32;
            if equal(a1854, a1855) {
                let a1858 = {
                    let a1856 = v0;
                    let a1857 = 1u32;
                    fixed_u32::bit_and(a1856, a1857)
                };
                let a1859 = 0u32;
                if equal(a1858, a1859) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a1862 = {
                    let a1860 = v0;
                    let a1861 = 1u32;
                    fixed_u32::bit_and(a1860, a1861)
                };
                let a1863 = 0u32;
                if equal(a1862, a1863) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f275(v0: u32, v1: u32) -> u32 {
    let m1866 = {
        let a1864 = v0;
        let a1865 = v1;
        fixed_u32::shift_right(a1864, a1865)
    };
    match m1866 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f276(v0: Adt20, v1: Adt29) -> R<Adt20> {
    let v2: Adt30 = f387(Fn7::F388, 64u64, Adt30::C0(match v0.clone() {
        Adt20::C0(h1867, _, _, _, _, _, _, _) => {
            h1867
        }
    }, match v0.clone() {
        Adt20::C0(_, h1868, _, _, _, _, _, _) => {
            h1868
        }
    }, match v0.clone() {
        Adt20::C0(_, _, h1869, _, _, _, _, _) => {
            h1869
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, h1870, _, _, _, _) => {
            h1870
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, h1871, _, _, _) => {
            h1871
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, h1872, _, _) => {
            h1872
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h1873, _) => {
            h1873
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, _, h1874) => {
            h1874
        }
    }, v1, 0u64))?;
    Ok(Adt20::C0(f389(match v0.clone() {
        Adt20::C0(h1875, _, _, _, _, _, _, _) => {
            h1875
        }
    }, match v2.clone() {
        Adt30::C0(h1876, _, _, _, _, _, _, _, _, _) => {
            h1876
        }
    }), f389(match v0.clone() {
        Adt20::C0(_, h1877, _, _, _, _, _, _) => {
            h1877
        }
    }, match v2.clone() {
        Adt30::C0(_, h1878, _, _, _, _, _, _, _, _) => {
            h1878
        }
    }), f389(match v0.clone() {
        Adt20::C0(_, _, h1879, _, _, _, _, _) => {
            h1879
        }
    }, match v2.clone() {
        Adt30::C0(_, _, h1880, _, _, _, _, _, _, _) => {
            h1880
        }
    }), f389(match v0.clone() {
        Adt20::C0(_, _, _, h1881, _, _, _, _) => {
            h1881
        }
    }, match v2.clone() {
        Adt30::C0(_, _, _, h1882, _, _, _, _, _, _) => {
            h1882
        }
    }), f389(match v0.clone() {
        Adt20::C0(_, _, _, _, h1883, _, _, _) => {
            h1883
        }
    }, match v2.clone() {
        Adt30::C0(_, _, _, _, h1884, _, _, _, _, _) => {
            h1884
        }
    }), f389(match v0.clone() {
        Adt20::C0(_, _, _, _, _, h1885, _, _) => {
            h1885
        }
    }, match v2.clone() {
        Adt30::C0(_, _, _, _, _, h1886, _, _, _, _) => {
            h1886
        }
    }), f389(match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h1887, _) => {
            h1887
        }
    }, match v2.clone() {
        Adt30::C0(_, _, _, _, _, _, h1888, _, _, _) => {
            h1888
        }
    }), f389(match v0 {
        Adt20::C0(_, _, _, _, _, _, _, h1889) => {
            h1889
        }
    }, match v2 {
        Adt30::C0(_, _, _, _, _, _, _, h1890, _, _) => {
            h1890
        }
    })))
}

pub fn f277(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt29> {
    Ok(Adt29::C0(f390(v0.clone(), v1, v2, v3)?, f390(v0.clone(), v1, v2, {
        let a1891 = v3;
        let a1892 = 4u64;
        nat_add(a1891, a1892)?
    })?, f390(v0.clone(), v1, v2, {
        let a1893 = v3;
        let a1894 = 8u64;
        nat_add(a1893, a1894)?
    })?, f390(v0.clone(), v1, v2, {
        let a1895 = v3;
        let a1896 = 12u64;
        nat_add(a1895, a1896)?
    })?, f390(v0.clone(), v1, v2, {
        let a1897 = v3;
        let a1898 = 16u64;
        nat_add(a1897, a1898)?
    })?, f390(v0.clone(), v1, v2, {
        let a1899 = v3;
        let a1900 = 20u64;
        nat_add(a1899, a1900)?
    })?, f390(v0.clone(), v1, v2, {
        let a1901 = v3;
        let a1902 = 24u64;
        nat_add(a1901, a1902)?
    })?, f390(v0.clone(), v1, v2, {
        let a1903 = v3;
        let a1904 = 28u64;
        nat_add(a1903, a1904)?
    })?, f390(v0.clone(), v1, v2, {
        let a1905 = v3;
        let a1906 = 32u64;
        nat_add(a1905, a1906)?
    })?, f390(v0.clone(), v1, v2, {
        let a1907 = v3;
        let a1908 = 36u64;
        nat_add(a1907, a1908)?
    })?, f390(v0.clone(), v1, v2, {
        let a1909 = v3;
        let a1910 = 40u64;
        nat_add(a1909, a1910)?
    })?, f390(v0.clone(), v1, v2, {
        let a1911 = v3;
        let a1912 = 44u64;
        nat_add(a1911, a1912)?
    })?, f390(v0.clone(), v1, v2, {
        let a1913 = v3;
        let a1914 = 48u64;
        nat_add(a1913, a1914)?
    })?, f390(v0.clone(), v1, v2, {
        let a1915 = v3;
        let a1916 = 52u64;
        nat_add(a1915, a1916)?
    })?, f390(v0.clone(), v1, v2, {
        let a1917 = v3;
        let a1918 = 56u64;
        nat_add(a1917, a1918)?
    })?, f390(v0, v1, v2, {
        let a1919 = v3;
        let a1920 = 60u64;
        nat_add(a1919, a1920)?
    })?))
}

pub fn f278(v0: u64) -> bool {
    f180({
        let a1921 = v0;
        let a1922 = 95u64;
        nat_eq(a1921, a1922)
    }, f180({
        let a1923 = v0;
        let a1924 = 45u64;
        nat_eq(a1923, a1924)
    }, f180({
        let a1925 = v0;
        let a1926 = 32u64;
        nat_eq(a1925, a1926)
    }, f180({
        let a1927 = v0;
        let a1928 = 46u64;
        nat_eq(a1927, a1928)
    }, {
        let a1929 = v0;
        let a1930 = 47u64;
        nat_eq(a1929, a1930)
    }))))
}

pub fn f279(v0: u64) -> bool {
    f113({
        let a1931 = 97u64;
        let a1932 = v0;
        nat_le(a1931, a1932)
    }, {
        let a1933 = v0;
        let a1934 = 122u64;
        nat_le(a1933, a1934)
    })
}

pub fn f280(v0: u64) -> bool {
    f113({
        let a1935 = 65u64;
        let a1936 = v0;
        nat_le(a1935, a1936)
    }, {
        let a1937 = v0;
        let a1938 = 90u64;
        nat_le(a1937, a1938)
    })
}

pub fn f281(v0: u64) -> bool {
    f113({
        let a1939 = 48u64;
        let a1940 = v0;
        nat_le(a1939, a1940)
    }, {
        let a1941 = v0;
        let a1942 = 57u64;
        nat_le(a1941, a1942)
    })
}

pub fn f282(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1943 = 0u64;
        let a1944 = v1;
        nat_lt(a1943, a1944)
    } else {
        false
    } {
        let a1945 = Bytes::lit(&[0x2d]);
        let a1946 = f91(v1)?;
        Ok(append_bytes(a1945, a1946))
    } else {
        f91(v1)
    }
}

pub fn f283(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f91(v1)?;
    let v4: u64 = {
        let a1947 = v3.clone();
        length_bytes(a1947)
    };
    let v5: Bytes = {
        let a1948 = v2;
        let a1949 = v4;
        if nat_lt(a1948, a1949) {
            f7(List::cons(f46(v3.clone(), 0u64, {
                let a1950 = v4;
                let a1951 = v2;
                nat_sub(a1950, a1951)
            }), List::cons(f8(Str::lit(".")), List::cons(f46(v3, {
                let a1952 = v4;
                let a1953 = v2;
                nat_sub(a1952, a1953)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f273({
                let a1954 = v2;
                let a1955 = v4;
                nat_sub(a1954, a1955)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1956 = f8(Str::lit("-"));
        let a1957 = v5;
        Ok(append_bytes(a1956, a1957))
    } else {
        Ok(v5)
    }
}

pub fn f284(v0: Bytes) -> R<Bytes> {
    let a1962 = f177(v0.clone(), 0u64, {
        let a1959 = {
            let a1958 = v0.clone();
            length_bytes(a1958)
        };
        let a1960 = 1u64;
        nat_add(a1959, a1960)?
    })?;
    let a1963 = {
        let a1961 = v0.clone();
        length_bytes(a1961)
    };
    if nat_eq(a1962, a1963) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        let a1969 = {
            let a1967 = Bytes::lit(&[0x22]);
            let a1968 = f391(v0.clone(), 0u64, {
                let a1965 = {
                    let a1964 = v0;
                    length_bytes(a1964)
                };
                let a1966 = 1u64;
                nat_add(a1965, a1966)?
            }, Bytes::lit(&[]))?;
            append_bytes(a1967, a1968)
        };
        let a1970 = Bytes::lit(&[0x22]);
        Ok(append_bytes(a1969, a1970))
    }
}

pub fn f285(v0: List<Bytes>) -> Bytes {
    let a1973 = {
        let a1971 = Bytes::lit(&[0x5b]);
        let a1972 = f47(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1971, a1972)
    };
    let a1974 = Bytes::lit(&[0x5d]);
    append_bytes(a1973, a1974)
}

pub fn f286(v0: Adt2) -> R<List<Bytes>> {
    let m1975 = v0;
    match m1975 {
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
        Adt2::C9(r1976, r1977) => {
            let v12 = (*r1976).clone();
            let v13 = (*r1977).clone();
            Ok(List::cons(f193(v12)?, f286(v13)?))
        }
        Adt2::C10(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f287(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f392(Fn8::F393, List::<(Bytes, Bytes)>::nil(), f394(v0)?);
    let v2: List<Bytes> = f395(Fn9::F396, List::<Bytes>::nil(), v1)?;
    let a1980 = {
        let a1978 = Bytes::lit(&[0x7b]);
        let a1979 = f47(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1978, a1979)
    };
    let a1981 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1980, a1981))
}

pub fn f288(v0: Adt2) -> R<List<(Bytes, Bytes)>> {
    let m1982 = v0;
    match m1982 {
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
        Adt2::C10(v14, r1983, r1984) => {
            let v15 = (*r1983).clone();
            let v16 = (*r1984).clone();
            Ok(List::cons((v14, f193(v15)?), f288(v16)?))
        }
    }
}

pub fn f289(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1985 = v0;
    match m1985.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f397(v1)?, f289(v2)?))
        }
    }
}

pub fn f290(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m1986 = v0;
    match m1986.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f290(v3, false))
            } else {
                f290(v3, true)
            }
        }
    }
}

pub fn f291(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f398(v0, v1)
}

pub fn f292(v0: List<Bytes>) -> List<Bytes> {
    f116(Fn2::F399, List::<Bytes>::nil(), v0)
}

pub fn f293(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f400(v0, v1)
}

pub fn f294(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f401(f402(f403(v0))?))
}

pub fn f295(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f133(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1989 = {
            let a1987 = v4;
            let a1988 = v1;
            nat_add(a1987, a1988)?
        };
        let a1990 = v3;
        if nat_lt(a1989, a1990) {
            Ok(Some({
                let a1991 = v4;
                let a1992 = 1u64;
                nat_add(a1991, a1992)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f296(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h1993, _) = v1;
        h1993
    };
    let a1995 = {
        let a1994 = v0.clone();
        length_bytes(a1994)
    };
    let a1996 = v2;
    if nat_le(a1995, a1996) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let a1997 = f160(v0.clone(), v2)?;
        let a1998 = 13u64;
        if nat_eq(a1997, a1998) {
            Ok(Some(({
                let a2001 = f160(v0, {
                    let a1999 = v2;
                    let a2000 = 1u64;
                    nat_add(a1999, a2000)?
                })?;
                let a2002 = 10u64;
                if nat_eq(a2001, a2002) {
                    let a2003 = v2;
                    let a2004 = 2u64;
                    nat_add(a2003, a2004)?
                } else {
                    let a2005 = v2;
                    let a2006 = 1u64;
                    nat_add(a2005, a2006)?
                }
            }, {
                let a2008 = {
                    let (_, h2007) = v1;
                    h2007
                };
                let a2009 = f8(Str::lit("\u{a}"));
                append_bytes(a2008, a2009)
            })))
        } else {
            let v4: u64 = {
                let m2010 = f231(v0.clone(), f8(Str::lit("\u{d}")), v2)?;
                match m2010 {
                    None => {
                        let a2011 = v0.clone();
                        length_bytes(a2011)
                    }
                    Some(v3) => {
                        v3
                    }
                }
            };
            Ok(Some((v4, {
                let a2015 = {
                    let (_, h2012) = v1;
                    h2012
                };
                let a2016 = f46(v0, v2, {
                    let a2013 = v4;
                    let a2014 = v2;
                    nat_sub(a2013, a2014)
                });
                append_bytes(a2015, a2016)
            })))
        }
    }
}

pub fn f297(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2017 = f404(v0, v1)?;
    match m2017 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f405({
                let (h2018, _) = v2;
                h2018
            }) {
                Ok(Some({
                    let a2020 = v1;
                    let a2021 = {
                        let (_, h2019) = v2;
                        h2019
                    };
                    nat_add(a2020, a2021)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f298(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2023 = v1;
    let a2024 = {
        let a2022 = v0.clone();
        length_bytes(a2022)
    };
    if if nat_lt(a2023, a2024) {
        f406(v0.clone(), v1)?
    } else {
        false
    } {
        Ok(Some(f303(v0, v1)?))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f299(v0: u64) -> bool {
    let a2025 = v0;
    let a2026 = 32u64;
    if nat_eq(a2025, a2026) {
        true
    } else {
        let a2027 = v0;
        let a2028 = 9u64;
        nat_eq(a2027, a2028)
    }
}

pub fn f300(v0: u64) -> bool {
    let a2029 = v0;
    let a2030 = 10u64;
    if nat_eq(a2029, a2030) {
        true
    } else {
        let a2031 = v0;
        let a2032 = 256u64;
        nat_eq(a2031, a2032)
    }
}

pub fn f301(v0: Bytes, v1: u64) -> R<u64> {
    let a2033 = v1;
    let a2034 = f407(v0, v1)?;
    Ok(nat_sub(a2033, a2034))
}

pub fn f302(v0: Bytes, v1: u64) -> R<u64> {
    let (h2035, _) = f201(Fn5::F408(v0.clone()), {
        let a2037 = {
            let a2036 = v0;
            length_bytes(a2036)
        };
        let a2038 = 1u64;
        nat_add(a2037, a2038)?
    }, v1)?;
    Ok(h2035)
}

pub fn f303(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f409(v0.clone(), v1)?;
    let a2040 = v2;
    let a2041 = {
        let a2039 = v0;
        length_bytes(a2039)
    };
    if nat_lt(a2040, a2041) {
        let a2042 = v2;
        let a2043 = 1u64;
        nat_add(a2042, a2043)
    } else {
        Ok(v2)
    }
}

pub fn f304(v0: Bytes, v1: u64) -> R<u64> {
    let (h2044, _) = f201(Fn5::F410(v0.clone()), {
        let a2046 = {
            let a2045 = v0;
            length_bytes(a2045)
        };
        let a2047 = 1u64;
        nat_add(a2046, a2047)?
    }, v1)?;
    Ok(h2044)
}

pub fn f305(v0: Bytes, v1: u64) -> R<bool> {
    let a2048 = f206(v0.clone(), v1)?;
    let a2049 = 45u64;
    if nat_eq(a2048, a2049) {
        if f299(f206(v0.clone(), {
            let a2050 = v1;
            let a2051 = 1u64;
            nat_add(a2050, a2051)?
        })?) {
            Ok(true)
        } else {
            Ok(f300(f206(v0, {
                let a2052 = v1;
                let a2053 = 1u64;
                nat_add(a2052, a2053)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f306(v0: Bytes, v1: u64) -> R<bool> {
    let m2054 = f318(v0, v1)?;
    match m2054 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f307(v0: Bytes, v1: u64) -> R<Option<Bytes>> {
    let v2: u64 = f206(v0.clone(), v1)?;
    let v3: bool = f208(f411(f206(v0, {
        let a2055 = v1;
        let a2056 = 1u64;
        nat_add(a2055, a2056)?
    })?, true));
    let a2057 = v2;
    let a2058 = 58u64;
    if if nat_eq(a2057, a2058) {
        v3
    } else {
        false
    } {
        Ok(Some(f137(Str::lit("a null key is not read: Studio.Value keys are strings (YAML 1.2.2 section 10.3.2)"), v1)?))
    } else {
        let a2059 = v2;
        let a2060 = 38u64;
        if nat_eq(a2059, a2060) {
            Ok(Some(f137(Str::lit("an anchor is not read (YAML 1.2.2 [101])"), v1)?))
        } else {
            let a2061 = v2;
            let a2062 = 42u64;
            if nat_eq(a2061, a2062) {
                Ok(Some(f137(Str::lit("an alias is not read (YAML 1.2.2 [104])"), v1)?))
            } else {
                let a2063 = v2;
                let a2064 = 33u64;
                if nat_eq(a2063, a2064) {
                    Ok(Some(f137(Str::lit("a tag is not read (YAML 1.2.2 [97])"), v1)?))
                } else {
                    let a2065 = v2;
                    let a2066 = 63u64;
                    if if nat_eq(a2065, a2066) {
                        v3
                    } else {
                        false
                    } {
                        Ok(Some(f137(Str::lit("an explicit key is not read (YAML 1.2.2 [143], [189])"), v1)?))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f308(v0: Bytes, v1: u64, v2: u64, v3: Adt21) -> R<Adt22> {
    let v4: u64 = f206(v0.clone(), v1)?;
    let a2067 = v4;
    let a2068 = 124u64;
    if if nat_eq(a2067, a2068) {
        true
    } else {
        let a2069 = v4;
        let a2070 = 62u64;
        nat_eq(a2069, a2070)
    } {
        let m2071 = f412(v0, v1, v2)?;
        match m2071 {
            Adt22::C0(v6, v7) => {
                let m2072 = f74(Some(v6.clone()));
                match m2072 {
                    None => {
                        Ok(Adt22::C0(v6, v7))
                    }
                    Some(v8) => {
                        f413(v3, v8, true, v1, v7)
                    }
                }
            }
            Adt22::C1(v5) => {
                Ok(Adt22::C1(v5))
            }
        }
    } else {
        let a2073 = v4;
        let a2074 = 34u64;
        if if nat_eq(a2073, a2074) {
            true
        } else {
            let a2075 = v4;
            let a2076 = 39u64;
            nat_eq(a2075, a2076)
        } {
            let m2077 = f414(v0.clone(), v1, v2, false)?;
            match m2077 {
                Adt32::C0(v10, v11) => {
                    let m2078 = f205(v0, v11)?;
                    match m2078 {
                        Ok(v13) => {
                            f413(v3, v10, true, v1, v13)
                        }
                        Err(v12) => {
                            Ok(Adt22::C1(v12))
                        }
                    }
                }
                Adt32::C1(v9) => {
                    Ok(Adt22::C1(v9))
                }
            }
        } else {
            let a2079 = v4;
            let a2080 = 91u64;
            if if nat_eq(a2079, a2080) {
                true
            } else {
                let a2081 = v4;
                let a2082 = 123u64;
                nat_eq(a2081, a2082)
            } {
                let m2086 = f415(v0.clone(), v1, v2, {
                    let a2084 = {
                        let a2083 = v0.clone();
                        length_bytes(a2083)
                    };
                    let a2085 = 2u64;
                    nat_add(a2084, a2085)?
                }, 0u64, List::<Bytes>::nil(), v3)?;
                match m2086 {
                    Adt22::C0(v15, v16) => {
                        let a2087 = f206(v0.clone(), f302(v0.clone(), v16)?)?;
                        let a2088 = 58u64;
                        if nat_eq(a2087, a2088) {
                            Ok(Adt22::C1(f137(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [154])"), v1)?))
                        } else {
                            let m2089 = f205(v0, v16)?;
                            match m2089 {
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
                let m2090 = f307(v0.clone(), v1)?;
                match m2090 {
                    None => {
                        if f208(f416(v0.clone(), v1, false)?) {
                            Ok(Adt22::C1(f137(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                        } else {
                            let v20: u64 = f417(v0.clone(), v1, false)?;
                            let m2094 = f418(v0.clone(), v20, v2, {
                                let a2092 = {
                                    let a2091 = v0.clone();
                                    length_bytes(a2091)
                                };
                                let a2093 = 2u64;
                                nat_add(a2092, a2093)?
                            }, f316(v0, v1, v20))?;
                            match m2094 {
                                Adt32::C0(v22, v23) => {
                                    f413(v3, v22, false, v1, v23)
                                }
                                Adt32::C1(v21) => {
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

pub fn f309(v0: Adt21) -> bool {
    let m2095 = v0;
    match m2095 {
        Adt21::C0(v2) => {
            let a2096 = 0u64;
            let a2097 = v2;
            nat_lt(a2096, a2097)
        }
        Adt21::C1(_) => {
            true
        }
        Adt21::C2(_) => {
            true
        }
    }
}

pub fn f310(v0: Bytes, v1: u64) -> R<Adt21> {
    let v2: u64 = f419(v0.clone(), v1)?;
    let a2100 = f206(v0.clone(), {
        let a2098 = v1;
        let a2099 = 1u64;
        nat_add(a2098, a2099)?
    })?;
    let a2101 = 60u64;
    if nat_eq(a2100, a2101) {
        let a2102 = f206(v0.clone(), v2)?;
        let a2103 = 62u64;
        if nat_eq(a2102, a2103) {
            let v3: Bytes = f316(v0, {
                let a2104 = v1;
                let a2105 = 2u64;
                nat_add(a2104, a2105)?
            }, v2);
            if f159(v3.clone(), 0u64, Str::lit("tag:yaml.org,2002:")) {
                f420(f316(v3.clone(), 18u64, {
                    let a2106 = v3;
                    length_bytes(a2106)
                }), v1)
            } else {
                Ok(Adt21::C1(v3))
            }
        } else {
            Ok(Adt21::C2(f137(Str::lit("a verbatim tag must end with > (YAML 1.2.2 [98])"), v1)?))
        }
    } else {
        let a2109 = v2;
        let a2110 = {
            let a2107 = v1;
            let a2108 = 1u64;
            nat_add(a2107, a2108)?
        };
        if nat_eq(a2109, a2110) {
            Ok(Adt21::C0(8u64))
        } else {
            let a2113 = f206(v0.clone(), {
                let a2111 = v1;
                let a2112 = 1u64;
                nat_add(a2111, a2112)?
            })?;
            let a2114 = 33u64;
            if nat_eq(a2113, a2114) {
                f420(f316(v0, {
                    let a2115 = v1;
                    let a2116 = 2u64;
                    nat_add(a2115, a2116)?
                }, v2), v1)
            } else {
                let a2123 = f315(f316(v0.clone(), {
                    let a2117 = v1;
                    let a2118 = 1u64;
                    nat_add(a2117, a2118)?
                }, v2), 33u64)?;
                let a2124 = {
                    let a2121 = v2;
                    let a2122 = {
                        let a2119 = v1;
                        let a2120 = 1u64;
                        nat_add(a2119, a2120)?
                    };
                    nat_sub(a2121, a2122)
                };
                if nat_lt(a2123, a2124) {
                    Ok(Adt21::C2(f137(Str::lit("a named tag handle is not read: it needs a TAG directive (YAML 1.2.2 [92], [82])"), v1)?))
                } else {
                    Ok(Adt21::C1(f316(v0, v1, v2)))
                }
            }
        }
    }
}

pub fn f311(v0: Adt21) -> Option<Bytes> {
    let m2125 = v0;
    match m2125 {
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

pub fn f312(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f419(v0.clone(), v1)?;
    let a2128 = f206(v0.clone(), {
        let a2126 = v1;
        let a2127 = 1u64;
        nat_add(a2126, a2127)?
    })?;
    let a2129 = 60u64;
    if if nat_eq(a2128, a2129) {
        let a2130 = f206(v0, v2)?;
        let a2131 = 62u64;
        nat_eq(a2130, a2131)
    } else {
        false
    } {
        let a2132 = v2;
        let a2133 = 1u64;
        nat_add(a2132, a2133)
    } else {
        Ok(v2)
    }
}

pub fn f313(v0: Adt21, v1: bool, v2: u64, v3: Adt22) -> R<Adt22> {
    let m2134 = v3;
    match m2134 {
        Adt22::C0(v5, v6) => {
            let v7: Adt2 = if v1 {
                Adt2::C6(std::rc::Rc::new(v5))
            } else {
                Adt2::C7(std::rc::Rc::new(v5))
            };
            let m2135 = v0;
            match m2135 {
                Adt21::C0(v10) => {
                    let a2136 = v10;
                    let a2137 = 0u64;
                    if if if nat_eq(a2136, a2137) {
                        true
                    } else {
                        let a2138 = v10;
                        let a2139 = 8u64;
                        nat_eq(a2138, a2139)
                    } {
                        true
                    } else {
                        let a2140 = v10;
                        let a2141 = if v1 {
                            6u64
                        } else {
                            7u64
                        };
                        nat_eq(a2140, a2141)
                    } {
                        Ok(Adt22::C0(v7, v6))
                    } else {
                        Ok(Adt22::C1(f421(v2)?))
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

pub fn f314(v0: Adt21, v1: u64, v2: u64) -> R<Adt22> {
    let m2142 = v0;
    match m2142 {
        Adt21::C0(v5) => {
            let a2143 = v5;
            let a2144 = 1u64;
            if nat_eq(a2143, a2144) {
                Ok(Adt22::C0(Adt2::C5(Bytes::lit(&[])), v2))
            } else {
                let a2145 = v5;
                let a2146 = 8u64;
                if nat_eq(a2145, a2146) {
                    Ok(Adt22::C1(f137(Str::lit("an empty node with the non-specific tag ! is not read (YAML 1.2.2 [100])"), v1)?))
                } else {
                    let a2147 = v5;
                    let a2148 = 0u64;
                    if if nat_eq(a2147, a2148) {
                        true
                    } else {
                        let a2149 = v5;
                        let a2150 = 5u64;
                        nat_eq(a2149, a2150)
                    } {
                        Ok(Adt22::C0(Adt2::C0, v2))
                    } else {
                        Ok(Adt22::C1(f421(v1)?))
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

pub fn f315(v0: Bytes, v1: u64) -> R<u64> {
    let (h2151, _) = f201(Fn5::F422(v0.clone(), v1), {
        let a2153 = {
            let a2152 = v0;
            length_bytes(a2152)
        };
        let a2154 = 1u64;
        nat_add(a2153, a2154)?
    }, 0u64)?;
    Ok(h2151)
}

pub fn f316(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    f46(v0, v1, {
        let a2155 = v2;
        let a2156 = v1;
        nat_sub(a2155, a2156)
    })
}

pub fn f317(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let a2157 = f206(v0.clone(), v1)?;
    let a2158 = 33u64;
    if nat_eq(a2157, a2158) {
        let m2159 = f310(v0.clone(), v1)?;
        match m2159 {
            Adt21::C0(v4) => {
                let a2160 = v4;
                let a2161 = 1u64;
                if if nat_eq(a2160, a2161) {
                    true
                } else {
                    let a2162 = v4;
                    let a2163 = 8u64;
                    nat_eq(a2162, a2163)
                } {
                    Ok(Ok::<u64, Bytes>(f302(v0.clone(), f312(v0, v1)?)?))
                } else {
                    Ok(Err::<u64, Bytes>(f137(Str::lit("a mapping key tagged other than !!str is not read: Studio.Value keys are strings (YAML 1.2.2 section 10.3.2)"), v1)?))
                }
            }
            Adt21::C1(_) => {
                Ok(Err::<u64, Bytes>(f137(Str::lit("a mapping key tagged other than !!str is not read: Studio.Value keys are strings (YAML 1.2.2 section 10.3.2)"), v1)?))
            }
            Adt21::C2(v2) => {
                Ok(Err::<u64, Bytes>(v2))
            }
        }
    } else {
        Ok(Ok::<u64, Bytes>(v1))
    }
}

pub fn f318(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a2164 = f206(v0.clone(), v1)?;
        let a2165 = 33u64;
        if nat_eq(a2164, a2165) {
            f302(v0.clone(), f312(v0.clone(), v1)?)?
        } else {
            v1
        }
    };
    let v3: u64 = f206(v0.clone(), v2)?;
    let a2166 = v3;
    let a2167 = 34u64;
    if if nat_eq(a2166, a2167) {
        true
    } else {
        let a2168 = v3;
        let a2169 = 39u64;
        nat_eq(a2168, a2169)
    } {
        let m2170 = f414(v0.clone(), v2, 0u64, true)?;
        match m2170 {
            Adt32::C0(_, v6) => {
                let v7: u64 = f302(v0.clone(), v6)?;
                if f423(v0, v7)? {
                    Ok(Some(v7))
                } else {
                    Ok(None::<u64>)
                }
            }
            Adt32::C1(_) => {
                Ok(None::<u64>)
            }
        }
    } else {
        if f416(v0.clone(), v2, false)? {
            let v8: u64 = f302(v0.clone(), f417(v0.clone(), v2, false)?)?;
            if f423(v0, v8)? {
                Ok(Some(v8))
            } else {
                Ok(None::<u64>)
            }
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f319(v0: Bytes, v1: u64) -> R<Bytes> {
    let v2: u64 = f206(v0.clone(), v1)?;
    let a2171 = v2;
    let a2172 = 34u64;
    if if nat_eq(a2171, a2172) {
        true
    } else {
        let a2173 = v2;
        let a2174 = 39u64;
        nat_eq(a2173, a2174)
    } {
        let m2175 = f414(v0, v1, 0u64, true)?;
        match m2175 {
            Adt32::C0(v4, _) => {
                Ok(v4)
            }
            Adt32::C1(_) => {
                Ok(Bytes::lit(&[]))
            }
        }
    } else {
        Ok(f316(v0.clone(), v1, f417(v0, v1, false)?))
    }
}

pub fn f320(v0: Bytes, p2178: List<Bytes>) -> bool {
    let m2177 = p2178;
    {
        let mut m2177 = m2177;
        loop {
            let step = {
                let v1 = m2177;
                let m2176 = v1;
                match m2176.uncons() {
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
                Ok(next) => { m2177 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f321(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2179, _) = v2;
        h2179
    };
    let a2180 = v3;
    let a2181 = 0u64;
    if nat_eq(a2180, a2181) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2182 = v3;
            let a2183 = v1;
            let a2184 = 0u64;
            nat_quot(a2182, a2183, a2184)
        }, {
            let a2189 = f209(v0, {
                let a2185 = v3;
                let a2186 = v1;
                let a2187 = 0u64;
                nat_rem(a2185, a2186, a2187)
            });
            let a2190 = {
                let (_, h2188) = v2;
                h2188
            };
            append_bytes(a2189, a2190)
        }))
    }
}

pub fn f322(p2245: u64, v1: Bytes, p2246: u64, p2247: List<Adt23>) -> R<Result<List<Adt23>, Bytes>> {
    let m2244 = (p2245, (p2246, p2247));
    {
        let mut m2244 = m2244;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2244;
                let m2191 = v0;
                if m2191 == 0 {
                    Err::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>(Ok(Err::<List<Adt23>, Bytes>(f424(Str::lit("the input has more tokens than bytes"), v2)?)))
                } else {
                    let v4 = m2191 - 1;
                    let v5: u64 = f160(v1.clone(), v2)?;
                    let a2192 = v5;
                    let a2193 = 256u64;
                    if nat_eq(a2192, a2193) {
                        Err::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>(Ok(Ok::<List<Adt23>, Bytes>(f425(v3, List::<Adt23>::nil()))))
                    } else {
                        if f230(v5)? {
                            Ok::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>((v4, ({
                                let a2194 = v2;
                                let a2195 = 1u64;
                                nat_add(a2194, a2195)?
                            }, v3)))
                        } else {
                            if f159(v1.clone(), v2, Str::lit("//")) {
                                let m2196 = f231(v1.clone(), Bytes::lit(&[0x0a]), v2)?;
                                match m2196 {
                                    None => {
                                        Err::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>(Ok(Ok::<List<Adt23>, Bytes>(f425(v3, List::<Adt23>::nil()))))
                                    }
                                    Some(v6) => {
                                        Ok::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>((v4, ({
                                            let a2197 = v6;
                                            let a2198 = 1u64;
                                            nat_add(a2197, a2198)?
                                        }, v3)))
                                    }
                                }
                            } else {
                                if f159(v1.clone(), v2, Str::lit("/*")) {
                                    let m2202 = f231(v1.clone(), {
                                        let a2199 = Str::lit("*/");
                                        utf8_encode(a2199)
                                    }, {
                                        let a2200 = v2;
                                        let a2201 = 2u64;
                                        nat_add(a2200, a2201)?
                                    })?;
                                    match m2202 {
                                        None => {
                                            Err::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>(Ok(Err::<List<Adt23>, Bytes>(f424(Str::lit("a comment is not closed (Web IDL, IDL grammar: comment)"), v2)?)))
                                        }
                                        Some(v7) => {
                                            Ok::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>((v4, ({
                                                let a2203 = v7;
                                                let a2204 = 2u64;
                                                nat_add(a2203, a2204)?
                                            }, v3)))
                                        }
                                    }
                                } else {
                                    let a2205 = v5;
                                    let a2206 = 34u64;
                                    if nat_eq(a2205, a2206) {
                                        let m2209 = f231(v1.clone(), Bytes::lit(&[0x22]), {
                                            let a2207 = v2;
                                            let a2208 = 1u64;
                                            nat_add(a2207, a2208)?
                                        })?;
                                        match m2209 {
                                            None => {
                                                Err::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>(Ok(Err::<List<Adt23>, Bytes>(f424(Str::lit("a string is not closed (Web IDL, IDL grammar: string)"), v2)?)))
                                            }
                                            Some(v8) => {
                                                Ok::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>((v4, ({
                                                    let a2210 = v8;
                                                    let a2211 = 1u64;
                                                    nat_add(a2210, a2211)?
                                                }, List::cons(Adt23::C0(3u64, f46(v1.clone(), {
                                                    let a2212 = v2;
                                                    let a2213 = 1u64;
                                                    nat_add(a2212, a2213)?
                                                }, {
                                                    let a2216 = v8;
                                                    let a2217 = {
                                                        let a2214 = v2;
                                                        let a2215 = 1u64;
                                                        nat_add(a2214, a2215)?
                                                    };
                                                    nat_sub(a2216, a2217)
                                                })), v3))))
                                            }
                                        }
                                    } else {
                                        let v9: u64 = {
                                            let a2218 = v5;
                                            let a2219 = 95u64;
                                            if if nat_eq(a2218, a2219) {
                                                true
                                            } else {
                                                let a2220 = v5;
                                                let a2221 = 45u64;
                                                nat_eq(a2220, a2221)
                                            } {
                                                let a2222 = v2;
                                                let a2223 = 1u64;
                                                nat_add(a2222, a2223)?
                                            } else {
                                                v2
                                            }
                                        };
                                        if f426(f160(v1.clone(), v9)?) {
                                            let v10: u64 = f427(v1.clone(), 3u64, {
                                                let a2224 = v9;
                                                let a2225 = 1u64;
                                                nat_add(a2224, a2225)?
                                            })?;
                                            Ok::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>((v4, (v10, List::cons(Adt23::C0(0u64, f46(v1.clone(), v2, {
                                                let a2226 = v10;
                                                let a2227 = v2;
                                                nat_sub(a2226, a2227)
                                            })), v3))))
                                        } else {
                                            let v11: u64 = {
                                                let a2228 = v5;
                                                let a2229 = 45u64;
                                                if nat_eq(a2228, a2229) {
                                                    let a2230 = v2;
                                                    let a2231 = 1u64;
                                                    nat_add(a2230, a2231)?
                                                } else {
                                                    v2
                                                }
                                            };
                                            let v12: u64 = f428(v1.clone(), v11)?;
                                            let v13: u64 = f429(v1.clone(), v11)?;
                                            let a2232 = v12;
                                            let a2233 = v13;
                                            if nat_lt(a2232, a2233) {
                                                Ok::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>((v4, (v13, List::cons(Adt23::C0(2u64, f46(v1.clone(), v2, {
                                                    let a2234 = v13;
                                                    let a2235 = v2;
                                                    nat_sub(a2234, a2235)
                                                })), v3))))
                                            } else {
                                                let a2236 = v11;
                                                let a2237 = v12;
                                                if nat_lt(a2236, a2237) {
                                                    Ok::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>((v4, (v12, List::cons(Adt23::C0(1u64, f46(v1.clone(), v2, {
                                                        let a2238 = v12;
                                                        let a2239 = v2;
                                                        nat_sub(a2238, a2239)
                                                    })), v3))))
                                                } else {
                                                    if f159(v1.clone(), v2, Str::lit("...")) {
                                                        Ok::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>((v4, ({
                                                            let a2240 = v2;
                                                            let a2241 = 3u64;
                                                            nat_add(a2240, a2241)?
                                                        }, List::cons(Adt23::C0(4u64, f8(Str::lit("..."))), v3))))
                                                    } else {
                                                        Ok::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>((v4, ({
                                                            let a2242 = v2;
                                                            let a2243 = 1u64;
                                                            nat_add(a2242, a2243)?
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
                Ok(next) => { m2244 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f323(p2250: List<Adt13>, p2251: List<Adt13>) -> List<Adt13> {
    let m2249 = (p2250, p2251);
    {
        let mut m2249 = m2249;
        loop {
            let step = {
                let (v0, v1) = m2249;
                let m2248 = v0;
                match m2248.uncons() {
                    None => {
                        Err::<(List<Adt13>, List<Adt13>), List<Adt13>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt13>, List<Adt13>), List<Adt13>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2249 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f324(v0: List<Adt23>) -> R<Result<List<Adt23>, Bytes>> {
    if f430(v0.clone(), Str::lit("[")) {
        Ok(f431({
            let a2253 = {
                let a2252 = v0.clone();
                length_list(a2252)
            };
            let a2254 = 1u64;
            nat_add(a2253, a2254)?
        }, f432(v0), List::cons(f8(Str::lit("]")), List::<Bytes>::nil()), false))
    } else {
        Ok(Ok::<List<Adt23>, Bytes>(v0))
    }
}

pub fn f325(v0: u64, v1: List<Adt23>) -> R<Result<(Adt13, List<Adt23>), Bytes>> {
    if f430(v1.clone(), Str::lit("interface")) {
        if f430(f432(v1.clone()), Str::lit("mixin")) {
            f433(v0, 1u64, f432(f432(v1)), false)
        } else {
            f433(v0, 0u64, f432(v1), true)
        }
    } else {
        if f430(v1.clone(), Str::lit("partial")) {
            let v2: List<Adt23> = f432(v1);
            if f430(v2.clone(), Str::lit("interface")) {
                if f430(f432(v2.clone()), Str::lit("mixin")) {
                    f433(v0, 6u64, f432(f432(v2)), false)
                } else {
                    f433(v0, 5u64, f432(v2), false)
                }
            } else {
                if f430(v2.clone(), Str::lit("dictionary")) {
                    f433(v0, 7u64, f432(v2), false)
                } else {
                    if f430(v2.clone(), Str::lit("namespace")) {
                        f434(f432(v2), f8(Str::lit("a namespace holds operations and attributes of no object, so it is behavior, not data (Web IDL 2.6)")))
                    } else {
                        Ok(Err::<(Adt13, List<Adt23>), Bytes>(f435(Str::lit("interface, dictionary or namespace"), v2)))
                    }
                }
            }
        } else {
            if f430(v1.clone(), Str::lit("dictionary")) {
                f433(v0, 2u64, f432(v1), true)
            } else {
                if f430(v1.clone(), Str::lit("enum")) {
                    let m2255 = f436(f432(v1));
                    match m2255 {
                        Ok(v4) => {
                            let m2256 = f437(f438(v4.clone()), Str::lit("{"));
                            match m2256 {
                                Ok(v6) => {
                                    let m2257 = f439(v0, v6, List::<Bytes>::nil());
                                    match m2257 {
                                        Ok(v8) => {
                                            Ok(Ok::<(Adt13, List<Adt23>), Bytes>((Adt13::C0(3u64, f440(v4), Bytes::lit(&[]), List::<Adt14>::nil(), {
                                                let (h2258, _) = v8;
                                                h2258
                                            }, f441(Bytes::lit(&[]))), {
                                                let (_, h2259) = v8;
                                                h2259
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
                    if f430(v1.clone(), Str::lit("typedef")) {
                        let m2260 = f442(v0, f432(v1), 0u64)?;
                        match m2260 {
                            Ok(v10) => {
                                let m2261 = f436(f443(v10.clone()));
                                match m2261 {
                                    Ok(v12) => {
                                        let m2262 = f437(f438(v12.clone()), Str::lit(";"));
                                        match m2262 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt13, List<Adt23>), Bytes>((Adt13::C0(4u64, f440(v12), Bytes::lit(&[]), List::<Adt14>::nil(), List::<Bytes>::nil(), f444(v10)), v14)))
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
                        if f430(v1.clone(), Str::lit("callback")) {
                            if f430(f432(v1.clone()), Str::lit("interface")) {
                                f434(f432(f432(v1)), f8(Str::lit("a callback interface is implemented by user code, so it is behavior, not data (Web IDL 2.4)")))
                            } else {
                                f434(f432(v1), f8(Str::lit("a callback function is behavior, not data (Web IDL 2.10)")))
                            }
                        } else {
                            if f430(v1.clone(), Str::lit("namespace")) {
                                f434(f432(v1), f8(Str::lit("a namespace holds operations and attributes of no object, so it is behavior, not data (Web IDL 2.6)")))
                            } else {
                                let m2263 = f436(v1);
                                match m2263 {
                                    Ok(v16) => {
                                        let m2264 = f437(f438(v16.clone()), Str::lit("includes"));
                                        match m2264 {
                                            Ok(v18) => {
                                                let m2265 = f436(v18);
                                                match m2265 {
                                                    Ok(v20) => {
                                                        let m2266 = f437(f438(v20.clone()), Str::lit(";"));
                                                        match m2266 {
                                                            Ok(v22) => {
                                                                Ok(Ok::<(Adt13, List<Adt23>), Bytes>((Adt13::C0(8u64, f440(v16), f440(v20), List::<Adt14>::nil(), List::<Bytes>::nil(), f441(Bytes::lit(&[]))), v22)))
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

pub fn f326(p2270: List<Bytes>, v1: List<Adt16>) -> Bytes {
    let m2269 = p2270;
    {
        let mut m2269 = m2269;
        loop {
            let step = {
                let v0 = m2269;
                let m2267 = v0;
                match m2267.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m2268 = f219(v1.clone(), v2.clone());
                        match m2268 {
                            None => {
                                Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("refers to ")), List::cons(v2, List::cons(f8(Str::lit(", which no input defines (Web IDL 2.13.22 to 2.13.26)")), List::<Bytes>::nil())))))
                            }
                            Some(v4) => {
                                if f32(f220(v4.clone()), Bytes::lit(&[])) {
                                    Ok::<List<Bytes>, Bytes>(v3)
                                } else {
                                    Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("refers to ")), List::cons(v2, List::cons(f8(Str::lit(", which is not translated: ")), List::cons(f220(v4), List::<Bytes>::nil()))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2269 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f327(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f125(v0)?;
    if f122(v1.clone(), f445()) {
        f446(v1)
    } else {
        Ok(v1)
    }
}

pub fn f328(p2276: List<Adt13>, v1: List<Adt13>, v2: Bytes) -> Option<Bytes> {
    let m2275 = p2276;
    {
        let mut m2275 = m2275;
        loop {
            let step = {
                let v0 = m2275;
                let m2271 = v0;
                match m2271.uncons() {
                    None => {
                        Err::<List<Adt13>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v3, v4)) => {
                        let a2272 = f216(v3.clone());
                        let a2273 = 8u64;
                        if if nat_eq(a2272, a2273) {
                            f32(f218(v3.clone()), v2.clone())
                        } else {
                            false
                        } {
                            let m2274 = f226(v1.clone(), 1u64, f228(v3.clone()));
                            match m2274 {
                                None => {
                                    Err::<List<Adt13>, Option<Bytes>>(Some(f228(v3)))
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
                Ok(next) => { m2275 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f329(v0: u64, v1: List<Adt13>, v2: Adt13) -> R<Result<List<Adt14>, Bytes>> {
    let m2277 = v0;
    if m2277 == 0 {
        Ok(Err::<List<Adt14>, Bytes>(f7(List::cons(f8(Str::lit("the inheritance of ")), List::cons(f218(v2), List::cons(f8(Str::lit(" is cyclic (Web IDL 2.2 and 2.7: an interface or dictionary must not inherit from itself)")), List::<Bytes>::nil()))))))
    } else {
        let v3 = m2277 - 1;
        let v4: List<Adt14> = {
            let a2278 = f216(v2.clone());
            let a2279 = 2u64;
            if nat_eq(a2278, a2279) {
                f447(f448(f449(v2.clone()), f450(v1.clone(), 7u64, f218(v2.clone()))))?
            } else {
                f448(f449(v2.clone()), f448(f450(v1.clone(), 5u64, f218(v2.clone())), f451(v1.clone(), v1.clone(), f218(v2.clone()))))
            }
        };
        if f32(f228(v2.clone()), Bytes::lit(&[])) {
            Ok(Ok::<List<Adt14>, Bytes>(v4))
        } else {
            let m2280 = f226(v1.clone(), f216(v2.clone()), f228(v2.clone()));
            match m2280 {
                None => {
                    Ok(Err::<List<Adt14>, Bytes>(f7(List::cons(f8(Str::lit("inherits ")), List::cons(f228(v2.clone()), List::cons({
                        let a2281 = f216(v2);
                        let a2282 = 2u64;
                        if nat_eq(a2281, a2282) {
                            f8(Str::lit(", which no input defines as a dictionary (Web IDL 2.7)"))
                        } else {
                            f8(Str::lit(", which no input defines as an interface (Web IDL 2.2)"))
                        }
                    }, List::<Bytes>::nil()))))))
                }
                Some(v5) => {
                    let m2283 = f329(v3, v1, v5)?;
                    match m2283 {
                        Ok(v7) => {
                            Ok(Ok::<List<Adt14>, Bytes>(f448(v7, v4)))
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

pub fn f330(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: mk, fields: []}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))
}

pub fn f331(v0: bool, v1: List<Adt14>) -> R<Result<(List<Bytes>, List<Bytes>), Bytes>> {
    let m2284 = v1;
    match m2284.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::<Bytes>::nil(), List::<Bytes>::nil())))
        }
        Some((v2, v3)) => {
            let v4: Adt15 = f452(v2.clone());
            if !f32(f340(v4.clone()), Bytes::lit(&[])) {
                Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons(f453(v2), List::cons(f8(Str::lit(": ")), List::cons(f340(v4), List::<Bytes>::nil())))))))
            } else {
                let m2285 = f454(v0, f453(v2.clone()))?;
                match m2285 {
                    None => {
                        Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons(f453(v2), List::cons(f8(Str::lit(" has no field name")), List::<Bytes>::nil()))))))
                    }
                    Some(v5) => {
                        if f122(v5.clone(), f455(v0, f333(v3.clone()))?) {
                            Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("two of its members derive the field name ")), List::cons(v5, List::cons(f8(Str::lit(" (Web IDL 2.2 and 2.7: the members of one definition have distinct identifiers)")), List::<Bytes>::nil()))))))
                        } else {
                            let m2286 = f331(v0, v3)?;
                            match m2286 {
                                Ok(v7) => {
                                    Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v5, List::cons(f8(Str::lit(", type: ")), List::cons({
                                        let a2287 = f456(v2);
                                        let a2288 = 1u64;
                                        if nat_eq(a2287, a2288) {
                                            f7(List::cons(f8(Str::lit("(Option ")), List::cons(f342(v4.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                        } else {
                                            f342(v4.clone())
                                        }
                                    }, List::cons(f8(Str::lit("}")), List::<Bytes>::nil())))))), {
                                        let (h2289, _) = v7;
                                        h2289
                                    }), f221(f343(v4), {
                                        let (_, h2290) = v7;
                                        h2290
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

pub fn f332(p2294: List<Bytes>) -> R<bool> {
    let m2293 = p2294;
    {
        let mut m2293 = m2293;
        loop {
            let step = {
                let v0 = m2293;
                let m2291 = v0;
                match m2291.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m2292 = f457(v1)?;
                        match m2292 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f122(v3, f455(false, v2.clone())?) {
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
                Ok(next) => { m2293 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f333(v0: List<Adt14>) -> List<Bytes> {
    let m2295 = v0;
    match m2295.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f453(v1), f333(v2))
        }
    }
}

pub fn f334(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}")), List::cons(f153(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f335(v0: Adt13) -> List<Bytes> {
    let m2296 = v0;
    match m2296 {
        Adt13::C0(_, _, _, _, v5, _) => {
            v5
        }
    }
}

pub fn f336(v0: bool, v1: List<Adt2>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m2297 = v1;
    match m2297.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m2298 = f458(v0, v3.clone())?;
            match m2298 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f459(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f122(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m2299 = f336(v0, v4, List::cons(v5.clone(), v2))?;
                        match m2299 {
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

pub fn f337(v0: List<Bytes>) -> List<Adt2> {
    let m2300 = v0;
    match m2300.uncons() {
        None => {
            List::<Adt2>::nil()
        }
        Some((v1, v2)) => {
            List::cons(Adt2::C5(v1), f337(v2))
        }
    }
}

pub fn f338(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f153(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f339(v0: Adt13) -> Adt15 {
    let m2301 = v0;
    match m2301 {
        Adt13::C0(_, _, _, _, _, v6) => {
            v6
        }
    }
}

pub fn f340(v0: Adt15) -> Bytes {
    let m2302 = v0;
    match m2302 {
        Adt15::C0(_, _, _, v4, _) => {
            v4
        }
    }
}

pub fn f341(v0: Adt15) -> u64 {
    let m2303 = v0;
    match m2303 {
        Adt15::C0(v1, _, _, _, _) => {
            v1
        }
    }
}

pub fn f342(v0: Adt15) -> Bytes {
    let m2304 = v0;
    match m2304 {
        Adt15::C0(_, v2, _, _, _) => {
            v2
        }
    }
}

pub fn f343(v0: Adt15) -> List<Bytes> {
    let m2305 = v0;
    match m2305 {
        Adt15::C0(_, _, v3, _, _) => {
            v3
        }
    }
}

pub fn f344(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f345(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2306 = f404(v0, v1)?;
    match m2306 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f460({
                let (h2307, _) = v2;
                h2307
            })? {
                Ok(Some({
                    let a2309 = v1;
                    let a2310 = {
                        let (_, h2308) = v2;
                        h2308
                    };
                    nat_add(a2309, a2310)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f346(v0: u64, v1: Bytes) -> R<bool> {
    f461(v0, v1.clone(), 0u64, {
        let a2312 = {
            let a2311 = v1;
            length_bytes(a2311)
        };
        let a2313 = 3u64;
        let a2314 = 0u64;
        nat_quot(a2312, a2313, a2314)
    }, true)
}

pub fn f347() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{0}\u{0} "))
}

pub fn f348(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f462(v1, v0, v2)
}

pub fn f349(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f463(v0, v1)
}

pub fn f350(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f464(v0, v1, v2)
}

pub fn f351(v0: Bytes) -> Result<(Bytes, u64), Bytes> {
    Err::<(Bytes, u64), Bytes>(v0)
}

pub fn f352(v0: Bytes, v1: u64) -> R<Result<(Bytes, u64), Bytes>> {
    if f159(v0.clone(), v1, Str::lit("&#")) {
        let v2: bool = {
            let a2317 = f160(v0.clone(), {
                let a2315 = v1;
                let a2316 = 2u64;
                nat_add(a2315, a2316)?
            })?;
            let a2318 = 120u64;
            nat_eq(a2317, a2318)
        };
        let v3: u64 = if v2 {
            let a2319 = v1;
            let a2320 = 3u64;
            nat_add(a2319, a2320)?
        } else {
            let a2321 = v1;
            let a2322 = 2u64;
            nat_add(a2321, a2322)?
        };
        let v4: (u64, u64) = f465(v0.clone(), v2, v3)?;
        let a2324 = {
            let (h2323, _) = v4;
            h2323
        };
        let a2325 = v3;
        if if nat_eq(a2324, a2325) {
            true
        } else {
            let a2327 = f160(v0, {
                let (h2326, _) = v4;
                h2326
            })?;
            let a2328 = 59u64;
            !nat_eq(a2327, a2328)
        } {
            Ok(f351(f137(Str::lit("a character reference must be &#digits; or &#xhex; (XML 1.0 [66])"), v1)?))
        } else {
            if f460({
                let (_, h2329) = v4;
                h2329
            })? {
                Ok(Ok::<(Bytes, u64), Bytes>((f466({
                    let (_, h2330) = v4;
                    h2330
                })?, {
                    let a2332 = {
                        let (h2331, _) = v4;
                        h2331
                    };
                    let a2333 = 1u64;
                    nat_add(a2332, a2333)?
                })))
            } else {
                Ok(f351(f137(Str::lit("a character reference must name a Char (XML 1.0 [66] WFC: Legal Character)"), v1)?))
            }
        }
    } else {
        if f159(v0.clone(), v1, Str::lit("&lt;")) {
            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("<")), {
                let a2334 = v1;
                let a2335 = 4u64;
                nat_add(a2334, a2335)?
            })))
        } else {
            if f159(v0.clone(), v1, Str::lit("&gt;")) {
                Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit(">")), {
                    let a2336 = v1;
                    let a2337 = 4u64;
                    nat_add(a2336, a2337)?
                })))
            } else {
                if f159(v0.clone(), v1, Str::lit("&amp;")) {
                    Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("&")), {
                        let a2338 = v1;
                        let a2339 = 5u64;
                        nat_add(a2338, a2339)?
                    })))
                } else {
                    if f159(v0.clone(), v1, Str::lit("&apos;")) {
                        Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("'")), {
                            let a2340 = v1;
                            let a2341 = 6u64;
                            nat_add(a2340, a2341)?
                        })))
                    } else {
                        if f159(v0, v1, Str::lit("&quot;")) {
                            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("\"")), {
                                let a2342 = v1;
                                let a2343 = 6u64;
                                nat_add(a2342, a2343)?
                            })))
                        } else {
                            Ok(f351(f137(Str::lit("an entity reference names an entity that is not declared, and only the five predefined entities are (XML 1.0 section 4.1, WFC: Entity Declared)"), v1)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f353(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<u64> {
    let (h2344, _) = f201(Fn5::F467(v1, v2, v0.clone()), {
        let a2346 = {
            let a2345 = v0;
            length_bytes(a2345)
        };
        let a2347 = 1u64;
        nat_add(a2346, a2347)?
    }, v3)?;
    Ok(h2344)
}

pub fn f354(p2360: Adt17, v1: List<(Bytes, Bytes)>) -> R<Result<Option<Adt31>, Bytes>> {
    let m2359 = p2360;
    {
        let mut m2359 = m2359;
        loop {
            let step = {
                let v0 = m2359;
                let m2348 = v0;
                match m2348 {
                    Adt17::C0(_, _, _) => {
                        Err::<Adt17, R<Result<Option<Adt31>, Bytes>>>(Ok(Ok::<Option<Adt31>, Bytes>(None::<Adt31>)))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<Adt17, R<Result<Option<Adt31>, Bytes>>>(Ok(Ok::<Option<Adt31>, Bytes>(None::<Adt31>)))
                    }
                    Adt17::C2(_) => {
                        Err::<Adt17, R<Result<Option<Adt31>, Bytes>>>(Ok(Ok::<Option<Adt31>, Bytes>(None::<Adt31>)))
                    }
                    Adt17::C3(r2357, r2358) => {
                        let v2 = (*r2357).clone();
                        let v3 = (*r2358).clone();
                        let m2349 = v2;
                        match m2349 {
                            Adt17::C0(v4, r2355, r2356) => {
                                let v5 = (*r2355).clone();
                                let v6 = (*r2356).clone();
                                let m2350 = f163(v5.clone(), v1.clone());
                                match m2350 {
                                    Ok(v8) => {
                                        let m2351 = f164(v8.clone(), v4)?;
                                        match m2351 {
                                            Ok(v10) => {
                                                if f320({
                                                    let (_, h2352) = v10.clone();
                                                    h2352
                                                }, List::cons(f8(Str::lit("restriction")), List::cons(f8(Str::lit("list")), List::cons(f8(Str::lit("union")), List::<Bytes>::nil())))) {
                                                    if f32({
                                                        let (h2353, _) = v10;
                                                        h2353
                                                    }, f247()) {
                                                        Err::<Adt17, R<Result<Option<Adt31>, Bytes>>>(Ok(Ok::<Option<Adt31>, Bytes>(Some(Adt31::C0({
                                                            let (_, h2354) = v10;
                                                            h2354
                                                        }, v5, v6, v8)))))
                                                    } else {
                                                        Ok::<Adt17, R<Result<Option<Adt31>, Bytes>>>(v3)
                                                    }
                                                } else {
                                                    Ok::<Adt17, R<Result<Option<Adt31>, Bytes>>>(v3)
                                                }
                                            }
                                            Err(v9) => {
                                                Err::<Adt17, R<Result<Option<Adt31>, Bytes>>>(Ok(f468(v9)))
                                            }
                                        }
                                    }
                                    Err(v7) => {
                                        Err::<Adt17, R<Result<Option<Adt31>, Bytes>>>(Ok(f468(v7)))
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
                Ok(next) => { m2359 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f355(v0: Bytes) -> Result<Adt24, Bytes> {
    Err::<Adt24, Bytes>(v0)
}

pub fn f356(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit(" at simpleType ")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f357(v0: List<(Bytes, Bytes)>, v1: Bytes, v2: Bytes) -> R<Result<Adt24, Bytes>> {
    let m2361 = f164(v0, v2.clone())?;
    match m2361 {
        Ok(v4) => {
            if f32({
                let (h2362, _) = v4.clone();
                h2362
            }, f247()) {
                let m2364 = f469({
                    let (_, h2363) = v4;
                    h2363
                }, 16u64)?;
                match m2364 {
                    Ok(v6) => {
                        Ok(Ok::<Adt24, Bytes>(Adt24::C0(v6, List::<Bytes>::nil())))
                    }
                    Err(v5) => {
                        Ok(f355(v5))
                    }
                }
            } else {
                if f32({
                    let (h2365, _) = v4.clone();
                    h2365
                }, v1) {
                    let v7: Bytes = f125({
                        let (_, h2366) = v4;
                        h2366
                    })?;
                    Ok(Ok::<Adt24, Bytes>(Adt24::C0(v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                } else {
                    Ok(f355(f7(List::cons(f8(Str::lit("the type ")), List::cons(v2, List::cons(f8(Str::lit(" is in the namespace ")), List::cons({
                        let (h2367, _) = v4;
                        h2367
                    }, List::cons(f8(Str::lit(", which is neither XML Schema's nor this document's target namespace; imported schema documents are not read yet")), List::<Bytes>::nil()))))))))
                }
            }
        }
        Err(v3) => {
            Ok(f355(v3))
        }
    }
}

pub fn f358(v0: Adt17, v1: List<(Bytes, Bytes)>) -> R<Result<List<Adt2>, Bytes>> {
    let m2368 = v0;
    match m2368 {
        Adt17::C0(_, _, _) => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
        Adt17::C2(_) => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
        Adt17::C3(r2375, r2376) => {
            let v2 = (*r2375).clone();
            let v3 = (*r2376).clone();
            let m2369 = f358(v3, v1.clone())?;
            match m2369 {
                Ok(v5) => {
                    let m2370 = v2;
                    match m2370 {
                        Adt17::C0(v6, r2374, _) => {
                            let v7 = (*r2374).clone();
                            let m2371 = f163(v7.clone(), v1);
                            match m2371 {
                                Ok(v10) => {
                                    let m2372 = f164(v10, v6)?;
                                    match m2372 {
                                        Ok(v12) => {
                                            if f165(v12, Str::lit("enumeration")) {
                                                let m2373 = f166(v7, f8(Str::lit("value")));
                                                match m2373 {
                                                    None => {
                                                        Ok(f470(f8(Str::lit("an enumeration facet must have a value attribute (XSD 1.1 Part 2, section 4.3.5.2)"))))
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
                                            Ok(f470(v11))
                                        }
                                    }
                                }
                                Err(v9) => {
                                    Ok(f470(v9))
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
                    Ok(f470(v4))
                }
            }
        }
        Adt17::C4 => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
    }
}

pub fn f359(v0: List<Adt2>, v1: List<Bytes>) -> R<List<Adt2>> {
    let m2377 = v0;
    match m2377.uncons() {
        None => {
            Ok(List::<Adt2>::nil())
        }
        Some((v2, v3)) => {
            let v4: Bytes = f459(v2.clone())?;
            if f122(v4.clone(), v1.clone()) {
                f359(v3, v1)
            } else {
                Ok(List::cons(v2, f359(v3, List::cons(v4, v1))?))
            }
        }
    }
}

pub fn f360(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f53(v0.clone(), v1) {
        let m2378 = f44(v0, 34u8)?;
        match m2378.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m2379 = v3;
                match m2379.uncons() {
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

pub fn f361(v0: List<Adt19>) -> R<Result<List<Adt26>, Bytes>> {
    let m2380 = v0;
    match m2380.uncons() {
        None => {
            Ok(Ok::<List<Adt26>, Bytes>(List::<Adt26>::nil()))
        }
        Some((v1, v2)) => {
            let m2383 = f261(match v1.clone() {
                Adt19::C0(_, _, _, _, h2381, _) => {
                    h2381
                }
            }, match v1.clone() {
                Adt19::C0(_, _, _, h2382, _, _) => {
                    h2382
                }
            })?;
            match m2383 {
                Ok(v4) => {
                    let m2384 = f361(v2)?;
                    match m2384 {
                        Ok(v6) => {
                            Ok(Ok::<List<Adt26>, Bytes>(f471(f472(v4, match v1 {
                                Adt19::C0(_, _, h2385, _, _, _) => {
                                    h2385
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

pub fn f362(v0: List<Adt19>) -> List<Bytes> {
    let m2386 = v0;
    match m2386.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt19::C0(_, _, h2387, _, _, _) => {
                    h2387
                }
            }, f362(v2))
        }
    }
}

pub fn f363(v0: u64, v1: Adt25, v2: List<Adt26>) -> R<Result<List<Adt28>, Bytes>> {
    let m2388 = v2;
    match m2388.uncons() {
        None => {
            Ok(Ok::<List<Adt28>, Bytes>(List::<Adt28>::nil()))
        }
        Some((v3, v4)) => {
            let m2389 = f473(v0, v1.clone(), v3)?;
            match m2389 {
                Ok(v6) => {
                    let m2390 = f363(v0, v1, v4)?;
                    match m2390 {
                        Ok(v8) => {
                            Ok(Ok::<List<Adt28>, Bytes>(f474(v6, v8)))
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

pub fn f364(v0: List<Adt19>) -> R<u64> {
    let m2391 = v0;
    match m2391.uncons() {
        None => {
            Ok(64u64)
        }
        Some((v1, v2)) => {
            let a2393 = match v1 {
                Adt19::C0(_, _, _, _, _, h2392) => {
                    h2392
                }
            };
            let a2394 = f364(v2)?;
            nat_add(a2393, a2394)
        }
    }
}

pub fn f365(p2402: List<Adt28>, p2403: List<Adt28>) -> Result<List<Adt28>, Bytes> {
    let m2401 = (p2402, p2403);
    {
        let mut m2401 = m2401;
        loop {
            let step = {
                let (v0, v1) = m2401;
                let m2395 = v0;
                match m2395.uncons() {
                    None => {
                        Err::<(List<Adt28>, List<Adt28>), Result<List<Adt28>, Bytes>>(Ok::<List<Adt28>, Bytes>(f475(v1, List::<Adt28>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m2397 = f476(v1.clone(), match v2.clone() {
                            Adt28::C0(h2396, _, _, _) => {
                                h2396
                            }
                        });
                        match m2397 {
                            None => {
                                Ok::<(List<Adt28>, List<Adt28>), Result<List<Adt28>, Bytes>>((v3, List::cons(v2, v1)))
                            }
                            Some(v4) => {
                                if f32(match v4 {
                                    Adt28::C0(_, _, h2398, _) => {
                                        h2398
                                    }
                                }, match v2.clone() {
                                    Adt28::C0(_, _, h2399, _) => {
                                        h2399
                                    }
                                }) {
                                    Ok::<(List<Adt28>, List<Adt28>), Result<List<Adt28>, Bytes>>((v3, v1))
                                } else {
                                    Err::<(List<Adt28>, List<Adt28>), Result<List<Adt28>, Bytes>>(Err::<List<Adt28>, Bytes>(f257(f8(Str::lit("two components derive the type name ")), match v2 {
                                        Adt28::C0(h2400, _, _, _) => {
                                            h2400
                                        }
                                    }, f8(Str::lit(", so their LexLean declarations would collide")))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2401 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f366(v0: List<Adt28>) -> List<Adt28> {
    if if f122(f477(), f478(v0.clone())) {
        let m2404 = f476(v0.clone(), f477());
        match m2404 {
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
        List::cons(Adt28::C0(f477(), List::<Bytes>::nil(), f338(f477(), f479()), f479()), v0)
    } else {
        v0
    }
}

pub fn f367(v0: List<Adt28>) -> List<Bytes> {
    let m2405 = v0;
    match m2405.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt28::C0(h2406, _, _, _) => {
                    h2406
                }
            }, f367(v2))
        }
    }
}

pub fn f368(p2412: List<Adt28>, v1: List<Bytes>) -> Option<(Bytes, Bytes)> {
    let m2411 = p2412;
    {
        let mut m2411 = m2411;
        loop {
            let step = {
                let v0 = m2411;
                let m2407 = v0;
                match m2407.uncons() {
                    None => {
                        Err::<List<Adt28>, Option<(Bytes, Bytes)>>(None::<(Bytes, Bytes)>)
                    }
                    Some((v2, v3)) => {
                        let m2409 = f250(match v2.clone() {
                            Adt28::C0(_, h2408, _, _) => {
                                h2408
                            }
                        }, v1.clone());
                        match m2409 {
                            None => {
                                Ok::<List<Adt28>, Option<(Bytes, Bytes)>>(v3)
                            }
                            Some(v4) => {
                                Err::<List<Adt28>, Option<(Bytes, Bytes)>>(Some((match v2 {
                                    Adt28::C0(h2410, _, _, _) => {
                                        h2410
                                    }
                                }, v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2411 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f369(v0: List<Adt28>, v1: List<Bytes>, v2: u64) -> List<(Bytes, List<Bytes>)> {
    let m2413 = v1;
    match m2413.uncons() {
        None => {
            List::<(Bytes, List<Bytes>)>::nil()
        }
        Some((v3, v4)) => {
            List::cons((v3.clone(), f480(v2, v0.clone(), f481(v0.clone(), v3), List::<Bytes>::nil())), f369(v0, v4, v2))
        }
    }
}

pub fn f370(v0: List<Adt28>) -> R<u64> {
    let m2414 = v0;
    match m2414.uncons() {
        None => {
            Ok(1u64)
        }
        Some((v1, v2)) => {
            let a2419 = {
                let a2417 = {
                    let a2416 = match v1 {
                        Adt28::C0(_, h2415, _, _) => {
                            h2415
                        }
                    };
                    length_list(a2416)
                };
                let a2418 = 1u64;
                nat_add(a2417, a2418)?
            };
            let a2420 = f370(v2)?;
            nat_add(a2419, a2420)
        }
    }
}

pub fn f371(p2430: u64, v1: List<Adt28>, p2431: List<(List<Bytes>, List<Bytes>)>, p2432: List<Bytes>, p2433: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m2429 = (p2430, (p2431, (p2432, p2433)));
    {
        let mut m2429 = m2429;
        loop {
            let step = {
                let (v0, (v2, (v3, v4))) = m2429;
                let m2421 = v0;
                if m2421 == 0 {
                    let m2422 = v2;
                    match m2422.uncons() {
                        None => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v4))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(f8(Str::lit("the declarations could not be ordered so that each follows the types it uses"))))
                        }
                    }
                } else {
                    let v7 = m2421 - 1;
                    let m2423 = v2.clone();
                    match m2423.uncons() {
                        None => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v4))
                        }
                        Some((_, _)) => {
                            let m2424 = f482(v2, v3.clone());
                            match m2424 {
                                None => {
                                    Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(f8(Str::lit("the declarations could not be ordered so that each follows the types it uses"))))
                                }
                                Some(v10) => {
                                    let m2426 = f483(v1.clone(), {
                                        let (h2425, _) = v10.clone();
                                        h2425
                                    });
                                    match m2426 {
                                        Ok(v12) => {
                                            Ok::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>((v7, ({
                                                let (_, h2427) = v10;
                                                h2427
                                            }, (f484({
                                                let (h2428, _) = v10;
                                                h2428
                                            }, v3), f484(v4, v12)))))
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
                Ok(next) => { m2429 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f372(v0: List<Adt28>, v1: List<List<Bytes>>) -> List<(List<Bytes>, List<Bytes>)> {
    let m2434 = v1;
    match m2434.uncons() {
        None => {
            List::<(List<Bytes>, List<Bytes>)>::nil()
        }
        Some((v2, v3)) => {
            List::cons((v2.clone(), f485(v0.clone(), v2.clone(), v2)), f372(v0, v3))
        }
    }
}

pub fn f373(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> List<List<Bytes>> {
    let m2435 = v2;
    match m2435.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((v4, v5)) => {
            if f122(v4.clone(), v3.clone()) {
                f373(v0, v1, v5, v3)
            } else {
                let v6: List<Bytes> = f486(v0.clone(), v1.clone(), v4);
                List::cons(v6.clone(), f373(v0, v1, v5, f484(v6, v3)))
            }
        }
    }
}

pub fn f374(v0: bool) -> bool {
    !v0
}

pub fn f375(v0: Adt27, v1: Str) -> bool {
    f32(match v0 {
        Adt27::C0(_, h2436, _, _, _) => {
            h2436
        }
    }, f8(v1))
}

pub fn f376(v0: Adt27, v1: Str) -> Option<Bytes> {
    f166(match v0 {
        Adt27::C0(_, _, h2437, _, _) => {
            h2437
        }
    }, f8(v1))
}

pub fn f377(v0: Option<Bytes>, v1: Bytes) -> R<Result<Bytes, Bytes>> {
    let m2438 = f231(v1.clone(), f8(Str::lit("://")), 0u64)?;
    match m2438 {
        None => {
            let v3: Bytes = f257(f8(Str::lit("the relative schema location ")), v1.clone(), f8(Str::lit(" is not resolved, because only a bare file name relative to a bound document's URL is merged (RFC 3986, section 5.2.3)")));
            if if f53(v1.clone(), f8(Str::lit("/"))) {
                true
            } else {
                let m2439 = f231(v1.clone(), f8(Str::lit("..")), 0u64)?;
                match m2439 {
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
                let m2440 = v0;
                match m2440 {
                    None => {
                        Ok(Err::<Bytes, Bytes>(v3))
                    }
                    Some(v5) => {
                        let m2442 = f487(v5.clone(), 0u64, {
                            let a2441 = v5.clone();
                            length_bytes(a2441)
                        }, None::<u64>)?;
                        match m2442 {
                            None => {
                                Ok(Err::<Bytes, Bytes>(v3))
                            }
                            Some(v6) => {
                                Ok(Ok::<Bytes, Bytes>({
                                    let a2445 = f46(v5, 0u64, {
                                        let a2443 = v6;
                                        let a2444 = 1u64;
                                        nat_add(a2443, a2444)?
                                    });
                                    let a2446 = v1;
                                    append_bytes(a2445, a2446)
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

pub fn f378(p2451: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m2450 = p2451;
    {
        let mut m2450 = m2450;
        loop {
            let step = {
                let v0 = m2450;
                let m2447 = v0;
                match m2447.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h2448, _) = v2;
                            h2448
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h2449) = v2;
                                h2449
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2450 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f379(v0: Adt27, v1: Str, v2: Str) -> Bytes {
    let m2452 = f376(v0, v1);
    match m2452 {
        None => {
            f8(v2)
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f380(v0: Bytes, v1: u64) -> R<Bytes> {
    let a2455 = f46(v0.clone(), {
        let a2453 = v1;
        let a2454 = 2u64;
        nat_add(a2453, a2454)?
    }, 2u64);
    let a2456 = Bytes::lit(&[0x30, 0x30]);
    if equal(a2455, a2456) {
        let m2459 = f488(f46(v0, {
            let a2457 = v1;
            let a2458 = 4u64;
            nat_add(a2457, a2458)?
        }, 2u64))?;
        match m2459 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f111(v2.clone(), 0u64)?;
                let a2460 = v3;
                let a2461 = 32u64;
                if nat_lt(a2460, a2461) {
                    if f489(v3) {
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

pub fn f381(v0: u64) -> Bytes {
    let a2462 = v0;
    let a2463 = 34u64;
    if nat_eq(a2462, a2463) {
        Bytes::lit(&[0x22])
    } else {
        let a2464 = v0;
        let a2465 = 92u64;
        if nat_eq(a2464, a2465) {
            Bytes::lit(&[0x5c])
        } else {
            let a2466 = v0;
            let a2467 = 98u64;
            if nat_eq(a2466, a2467) {
                Bytes::lit(&[0x08])
            } else {
                let a2468 = v0;
                let a2469 = 102u64;
                if nat_eq(a2468, a2469) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a2470 = v0;
                    let a2471 = 110u64;
                    if nat_eq(a2470, a2471) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a2472 = v0;
                        let a2473 = 114u64;
                        if nat_eq(a2472, a2473) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a2474 = v0;
                            let a2475 = 116u64;
                            if nat_eq(a2474, a2475) {
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

pub fn f382(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f111(v0, v1)?;
    let a2476 = 47u64;
    let a2477 = v2;
    if if nat_lt(a2476, a2477) {
        let a2478 = v2;
        let a2479 = 58u64;
        nat_lt(a2478, a2479)
    } else {
        false
    } {
        Ok(Some({
            let a2480 = v1;
            let a2481 = 1u64;
            nat_add(a2480, a2481)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f383(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2482 = f111(v0, v1)?;
    let a2483 = 48u64;
    if nat_eq(a2482, a2483) {
        Ok(Some({
            let a2484 = v1;
            let a2485 = 1u64;
            nat_add(a2484, a2485)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f384(v0: Bytes) -> R<u64> {
    let (h2486, _) = f201(Fn5::F490(v0.clone()), {
        let a2487 = v0;
        length_bytes(a2487)
    }, 0u64)?;
    Ok(h2486)
}

pub fn f385(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m2491 = {
        let a2489 = v0;
        let a2490 = {
            let (h2488, _) = v1;
            h2488
        };
        index_bytes(a2489, a2490)
    };
    match m2491 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2493 = {
                    let (h2492, _) = v1;
                    h2492
                };
                let a2494 = 1u64;
                nat_add(a2493, a2494)?
            }, {
                let a2498 = {
                    let a2496 = {
                        let (_, h2495) = v1;
                        h2495
                    };
                    let a2497 = 10u64;
                    nat_mul(a2496, a2497)?
                };
                let a2499 = f491(v2)?;
                nat_add(a2498, a2499)?
            })))
        }
    }
}

pub fn f386(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f492(v0, v1)
}

pub fn f387(v0: Fn7, p2503: u64, p2504: Adt30) -> R<Adt30> {
    let m2502 = (p2503, p2504);
    {
        let mut m2502 = m2502;
        loop {
            let step = {
                let (v1, v2) = m2502;
                let m2500 = v1;
                if m2500 == 0 {
                    Err::<(u64, Adt30), R<Adt30>>(Ok(v2))
                } else {
                    let v3 = m2500 - 1;
                    Ok::<(u64, Adt30), R<Adt30>>((v3, {
                        let c2501: Fn7 = v0.clone();
                        c2501.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2502 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f388(v0: Adt30) -> R<Adt30> {
    f493(v0)
}

pub fn f389(v0: u32, v1: u32) -> u32 {
    let m2507 = {
        let a2505 = f494(v0);
        let a2506 = f494(v1);
        fixed_u64::checked_add(a2505, a2506)
    };
    match m2507 {
        None => {
            0u32
        }
        Some(v2) => {
            f495(v2)
        }
    }
}

pub fn f390(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a2510 = {
        let a2508 = v3;
        let a2509 = 8u64;
        nat_add(a2508, a2509)?
    };
    let a2511 = v2;
    if nat_eq(a2510, a2511) {
        Ok(f496({
            let a2514 = {
                let a2512 = v1;
                let a2513 = 8u64;
                nat_mul(a2512, a2513)?
            };
            let a2515 = 4294967296u64;
            let a2516 = 0u64;
            nat_quot(a2514, a2515, a2516)
        }))
    } else {
        let a2519 = {
            let a2517 = v3;
            let a2518 = 4u64;
            nat_add(a2517, a2518)?
        };
        let a2520 = v2;
        if nat_eq(a2519, a2520) {
            Ok(f496({
                let a2523 = {
                    let a2521 = v1;
                    let a2522 = 8u64;
                    nat_mul(a2521, a2522)?
                };
                let a2524 = 4294967296u64;
                let a2525 = 0u64;
                nat_rem(a2523, a2524, a2525)
            }))
        } else {
            Ok(f497(f498(v0.clone(), v1, v3), f498(v0.clone(), v1, {
                let a2526 = v3;
                let a2527 = 1u64;
                nat_add(a2526, a2527)?
            }), f498(v0.clone(), v1, {
                let a2528 = v3;
                let a2529 = 2u64;
                nat_add(a2528, a2529)?
            }), f498(v0, v1, {
                let a2530 = v3;
                let a2531 = 3u64;
                nat_add(a2530, a2531)?
            })))
        }
    }
}

pub fn f391(v0: Bytes, p2550: u64, p2551: u64, p2552: Bytes) -> R<Bytes> {
    let m2549 = (p2550, (p2551, p2552));
    {
        let mut m2549 = m2549;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2549;
                let m2532 = v2;
                if m2532 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m2532 - 1;
                    let v5: u64 = f177(v0.clone(), v1, {
                        let a2536 = {
                            let a2534 = {
                                let a2533 = v0.clone();
                                length_bytes(a2533)
                            };
                            let a2535 = v1;
                            nat_sub(a2534, a2535)
                        };
                        let a2537 = 1u64;
                        nat_add(a2536, a2537)?
                    })?;
                    let v6: Bytes = {
                        let a2540 = v3;
                        let a2541 = f46(v0.clone(), v1, {
                            let a2538 = v5;
                            let a2539 = v1;
                            nat_sub(a2538, a2539)
                        });
                        append_bytes(a2540, a2541)
                    };
                    let m2544 = {
                        let a2542 = v0.clone();
                        let a2543 = v5;
                        index_bytes(a2542, a2543)
                    };
                    match m2544 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v6))
                        }
                        Some(v7) => {
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                let a2545 = v5;
                                let a2546 = 1u64;
                                nat_add(a2545, a2546)?
                            }, (v4, {
                                let a2547 = v6;
                                let a2548 = f499(v0.clone(), v5, v7)?;
                                append_bytes(a2547, a2548)
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2549 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f392(v0: Fn8, p2556: List<(Bytes, Bytes)>, p2557: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m2555 = (p2556, p2557);
    {
        let mut m2555 = m2555;
        loop {
            let step = {
                let (v1, v2) = m2555;
                let m2553 = v2;
                match m2553.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c2554: Fn8 = v0.clone();
                            c2554.apply(v1, v3)
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

pub fn f393(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f500(v0, v1)
}

pub fn f394(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f501(v0)
}

pub fn f395(v0: Fn9, p2561: List<Bytes>, p2562: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m2560 = (p2561, p2562);
    {
        let mut m2560 = m2560;
        loop {
            let step = {
                let (v1, v2) = m2560;
                let m2558 = v2;
                match m2558.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c2559: Fn9 = v0.clone();
                            c2559.apply(v1, v3)?
                        }, v4))
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

pub fn f396(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f502(v1)?, v0))
}

pub fn f397(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(f503(v0.clone(), 0u64, {
        let a2563 = v0;
        length_bytes(a2563)
    }, Bytes::lit(&[]))?, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
}

pub fn f398(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h2564, _) = v1.clone();
        h2564
    };
    let v3: u64 = {
        let (h2565, _) = {
            let (_, h2566) = v1.clone();
            h2566
        };
        h2565
    };
    let v4: List<Bytes> = {
        let (_, h2567) = {
            let (_, h2568) = v1;
            h2568
        };
        h2567
    };
    let m2569 = f185(v0.clone(), v2);
    match m2569 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a2570 = v5;
            let a2571 = 10u8;
            if equal(a2570, a2571) {
                Ok(Some(({
                    let a2572 = v2;
                    let a2573 = 1u64;
                    nat_add(a2572, a2573)?
                }, ({
                    let a2574 = v2;
                    let a2575 = 1u64;
                    nat_add(a2574, a2575)?
                }, List::cons(f504(f46(v0, v3, {
                    let a2576 = v2;
                    let a2577 = v3;
                    nat_sub(a2576, a2577)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a2578 = v2;
                    let a2579 = 1u64;
                    nat_add(a2578, a2579)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f399(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f400(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m2580 = v0.clone();
    match m2580.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a2581 = v2;
            let a2582 = v1.clone();
            if equal(a2581, a2582) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f401(v0: List<(Bytes, ())>) -> List<Bytes> {
    f292(f505(Fn10::F506, List::<Bytes>::nil(), v0))
}

pub fn f402(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f507(f508(Fn11::F509, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m2587 = {
        let (h2583, _) = f510(Fn12::F511, {
            let a2585 = {
                let a2584 = v0;
                length_list(a2584)
            };
            let a2586 = 1u64;
            nat_add(a2585, a2586)?
        }, v1)?;
        h2583
    };
    match m2587.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f403(v0: List<Bytes>) -> List<(Bytes, ())> {
    f512(f513(Fn13::F514, List::<(Bytes, ())>::nil(), v0))
}

pub fn f404(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m2590 = {
        let a2588 = v0.clone();
        let a2589 = v1;
        index_bytes(a2588, a2589)
    };
    match m2590 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f62(v2)?;
            let a2591 = v3;
            let a2592 = 128u64;
            if nat_lt(a2591, a2592) {
                Ok(Some((v3, 1u64)))
            } else {
                if f113({
                    let a2593 = 194u64;
                    let a2594 = v3;
                    nat_le(a2593, a2594)
                }, {
                    let a2595 = v3;
                    let a2596 = 223u64;
                    nat_le(a2595, a2596)
                }) {
                    let m2599 = f515(v0, {
                        let a2597 = v1;
                        let a2598 = 1u64;
                        nat_add(a2597, a2598)?
                    })?;
                    match m2599 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f516({
                                let a2604 = {
                                    let a2602 = {
                                        let a2600 = v3;
                                        let a2601 = 192u64;
                                        nat_sub(a2600, a2601)
                                    };
                                    let a2603 = 64u64;
                                    nat_mul(a2602, a2603)?
                                };
                                let a2605 = v4;
                                nat_add(a2604, a2605)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f113({
                        let a2606 = 224u64;
                        let a2607 = v3;
                        nat_le(a2606, a2607)
                    }, {
                        let a2608 = v3;
                        let a2609 = 239u64;
                        nat_le(a2608, a2609)
                    }) {
                        let m2612 = f515(v0.clone(), {
                            let a2610 = v1;
                            let a2611 = 1u64;
                            nat_add(a2610, a2611)?
                        })?;
                        match m2612 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m2615 = f515(v0, {
                                    let a2613 = v1;
                                    let a2614 = 2u64;
                                    nat_add(a2613, a2614)?
                                })?;
                                match m2615 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f516({
                                            let a2624 = {
                                                let a2618 = {
                                                    let a2616 = v3;
                                                    let a2617 = 224u64;
                                                    nat_sub(a2616, a2617)
                                                };
                                                let a2619 = 4096u64;
                                                nat_mul(a2618, a2619)?
                                            };
                                            let a2625 = {
                                                let a2622 = {
                                                    let a2620 = v5;
                                                    let a2621 = 64u64;
                                                    nat_mul(a2620, a2621)?
                                                };
                                                let a2623 = v6;
                                                nat_add(a2622, a2623)?
                                            };
                                            nat_add(a2624, a2625)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f113({
                            let a2626 = 240u64;
                            let a2627 = v3;
                            nat_le(a2626, a2627)
                        }, {
                            let a2628 = v3;
                            let a2629 = 244u64;
                            nat_le(a2628, a2629)
                        }) {
                            let m2632 = f515(v0.clone(), {
                                let a2630 = v1;
                                let a2631 = 1u64;
                                nat_add(a2630, a2631)?
                            })?;
                            match m2632 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m2635 = f515(v0.clone(), {
                                        let a2633 = v1;
                                        let a2634 = 2u64;
                                        nat_add(a2633, a2634)?
                                    })?;
                                    match m2635 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m2638 = f515(v0, {
                                                let a2636 = v1;
                                                let a2637 = 3u64;
                                                nat_add(a2636, a2637)?
                                            })?;
                                            match m2638 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f516({
                                                        let a2651 = {
                                                            let a2641 = {
                                                                let a2639 = v3;
                                                                let a2640 = 240u64;
                                                                nat_sub(a2639, a2640)
                                                            };
                                                            let a2642 = 262144u64;
                                                            nat_mul(a2641, a2642)?
                                                        };
                                                        let a2652 = {
                                                            let a2649 = {
                                                                let a2643 = v7;
                                                                let a2644 = 4096u64;
                                                                nat_mul(a2643, a2644)?
                                                            };
                                                            let a2650 = {
                                                                let a2647 = {
                                                                    let a2645 = v8;
                                                                    let a2646 = 64u64;
                                                                    nat_mul(a2645, a2646)?
                                                                };
                                                                let a2648 = v9;
                                                                nat_add(a2647, a2648)?
                                                            };
                                                            nat_add(a2649, a2650)?
                                                        };
                                                        nat_add(a2651, a2652)?
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

pub fn f405(v0: u64) -> bool {
    let a2653 = v0;
    let a2654 = 9u64;
    if if if nat_eq(a2653, a2654) {
        true
    } else {
        let a2655 = v0;
        let a2656 = 10u64;
        nat_eq(a2655, a2656)
    } {
        true
    } else {
        let a2657 = v0;
        let a2658 = 13u64;
        if nat_eq(a2657, a2658) {
            true
        } else {
            let a2659 = 32u64;
            let a2660 = v0;
            if nat_le(a2659, a2660) {
                let a2661 = v0;
                let a2662 = 126u64;
                nat_le(a2661, a2662)
            } else {
                false
            }
        }
    } {
        true
    } else {
        let a2663 = v0;
        let a2664 = 133u64;
        if if nat_eq(a2663, a2664) {
            true
        } else {
            let a2665 = 160u64;
            let a2666 = v0;
            if nat_le(a2665, a2666) {
                let a2667 = v0;
                let a2668 = 55295u64;
                nat_le(a2667, a2668)
            } else {
                false
            }
        } {
            true
        } else {
            let a2669 = 57344u64;
            let a2670 = v0;
            if if nat_le(a2669, a2670) {
                let a2671 = v0;
                let a2672 = 65533u64;
                nat_le(a2671, a2672)
            } else {
                false
            } {
                true
            } else {
                let a2673 = 65536u64;
                let a2674 = v0;
                if nat_le(a2673, a2674) {
                    let a2675 = v0;
                    let a2676 = 1114111u64;
                    nat_le(a2675, a2676)
                } else {
                    false
                }
            }
        }
    }
}

pub fn f406(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f302(v0.clone(), v1)?;
    if f300(f206(v0.clone(), v2)?) {
        Ok(true)
    } else {
        let a2677 = f206(v0, v2)?;
        let a2678 = 35u64;
        Ok(nat_eq(a2677, a2678))
    }
}

pub fn f407(v0: Bytes, v1: u64) -> R<u64> {
    let (h2679, _) = f201(Fn5::F517(v0), {
        let a2680 = v1;
        let a2681 = 1u64;
        nat_add(a2680, a2681)?
    }, v1)?;
    Ok(h2679)
}

pub fn f408(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f518(v0, v1)
}

pub fn f409(v0: Bytes, v1: u64) -> R<u64> {
    let (h2682, _) = f201(Fn5::F519(v0.clone()), {
        let a2684 = {
            let a2683 = v0;
            length_bytes(a2683)
        };
        let a2685 = 1u64;
        nat_add(a2684, a2685)?
    }, v1)?;
    Ok(h2682)
}

pub fn f410(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f520(v0, v1)
}

pub fn f411(v0: u64, v1: bool) -> bool {
    f208(if if f299(v0) {
        true
    } else {
        f300(v0)
    } {
        true
    } else {
        if v1 {
            f521(v0)
        } else {
            false
        }
    })
}

pub fn f412(v0: Bytes, v1: u64, v2: u64) -> R<Adt22> {
    let v3: bool = {
        let a2686 = f206(v0.clone(), v1)?;
        let a2687 = 62u64;
        nat_eq(a2686, a2687)
    };
    let v4: u64 = f206(v0.clone(), {
        let a2688 = v1;
        let a2689 = 1u64;
        nat_add(a2688, a2689)?
    })?;
    let v5: u64 = f206(v0.clone(), {
        let a2690 = v1;
        let a2691 = 2u64;
        nat_add(a2690, a2691)?
    })?;
    let v6: u64 = f522(v4);
    let v7: u64 = f523(v4);
    let v8: bool = {
        let a2692 = 0u64;
        let a2693 = v6;
        if nat_lt(a2692, a2693) {
            true
        } else {
            let a2694 = 0u64;
            let a2695 = v7;
            nat_lt(a2694, a2695)
        }
    };
    let v9: u64 = {
        let a2696 = 0u64;
        let a2697 = v7;
        if nat_lt(a2696, a2697) {
            f522(v5)
        } else {
            0u64
        }
    };
    let v10: u64 = {
        let a2698 = 0u64;
        let a2699 = v6;
        if nat_lt(a2698, a2699) {
            f523(v5)
        } else {
            0u64
        }
    };
    let v11: u64 = {
        let a2700 = v6;
        let a2701 = v9;
        nat_add(a2700, a2701)?
    };
    let v12: u64 = {
        let a2702 = v7;
        let a2703 = v10;
        nat_add(a2702, a2703)?
    };
    let v13: u64 = {
        let a2712 = {
            let a2704 = v1;
            let a2705 = 1u64;
            nat_add(a2704, a2705)?
        };
        let a2713 = {
            let a2710 = if v8 {
                1u64
            } else {
                0u64
            };
            let a2711 = {
                let a2706 = 0u64;
                let a2707 = v9;
                if if nat_lt(a2706, a2707) {
                    true
                } else {
                    let a2708 = 0u64;
                    let a2709 = v10;
                    nat_lt(a2708, a2709)
                } {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a2710, a2711)?
        };
        nat_add(a2712, a2713)?
    };
    let m2714 = f205(v0.clone(), v13)?;
    match m2714 {
        Ok(v15) => {
            let a2715 = 0u64;
            let a2716 = v12;
            if nat_lt(a2715, a2716) {
                f524(v0, v15, {
                    let a2719 = {
                        let a2717 = v2;
                        let a2718 = v12;
                        nat_add(a2717, a2718)?
                    };
                    let a2720 = 1u64;
                    nat_sub(a2719, a2720)
                }, v3, v11)
            } else {
                let v16: (u64, u64) = {
                    let (h2721, _) = f270(Fn6::F525(v0.clone()), {
                        let a2723 = {
                            let a2722 = v0.clone();
                            length_bytes(a2722)
                        };
                        let a2724 = 1u64;
                        nat_add(a2723, a2724)?
                    }, (v15, 0u64))?;
                    h2721
                };
                let v17: u64 = f304(v0.clone(), {
                    let (h2725, _) = v16;
                    h2725
                })?;
                let a2726 = f206(v0.clone(), v17)?;
                let a2727 = 256u64;
                if nat_eq(a2726, a2727) {
                    Ok(Adt22::C0(Adt2::C5({
                        let a2728 = v11;
                        let a2729 = 2u64;
                        if nat_eq(a2728, a2729) {
                            f526(f527(v0.clone(), v15)?)
                        } else {
                            Bytes::lit(&[])
                        }
                    }), {
                        let a2730 = v0;
                        length_bytes(a2730)
                    }))
                } else {
                    let v18: u64 = f528({
                        let a2732 = v17;
                        let a2733 = {
                            let (h2731, _) = v16;
                            h2731
                        };
                        nat_sub(a2732, a2733)
                    }, v2);
                    let a2735 = v18;
                    let a2736 = {
                        let (_, h2734) = v16;
                        h2734
                    };
                    if nat_lt(a2735, a2736) {
                        Ok(Adt22::C1(f137(Str::lit("a leading empty line of a block scalar holds more spaces than its first content line (YAML 1.2.2 section 8.1.1.1)"), v15)?))
                    } else {
                        f524(v0, v15, v18, v3, v11)
                    }
                }
            }
        }
        Err(v14) => {
            Ok(Adt22::C1(v14))
        }
    }
}

pub fn f413(v0: Adt21, v1: Bytes, v2: bool, v3: u64, v4: u64) -> R<Adt22> {
    let m2737 = v0;
    match m2737 {
        Adt21::C0(v7) => {
            let a2738 = v7;
            let a2739 = 1u64;
            if if if nat_eq(a2738, a2739) {
                true
            } else {
                let a2740 = v7;
                let a2741 = 8u64;
                nat_eq(a2740, a2741)
            } {
                true
            } else {
                let a2742 = v7;
                let a2743 = 0u64;
                if nat_eq(a2742, a2743) {
                    v2
                } else {
                    false
                }
            } {
                Ok(Adt22::C0(Adt2::C5(v1), v4))
            } else {
                let a2744 = v7;
                let a2745 = 0u64;
                if nat_eq(a2744, a2745) {
                    f529(v1, v3, v4)
                } else {
                    let a2746 = v7;
                    let a2747 = 5u64;
                    if nat_eq(a2746, a2747) {
                        if f530(v1) {
                            Ok(Adt22::C0(Adt2::C0, v4))
                        } else {
                            Ok(Adt22::C1(f421(v3)?))
                        }
                    } else {
                        let a2748 = v7;
                        let a2749 = 2u64;
                        if if nat_eq(a2748, a2749) {
                            f531(v1.clone())?
                        } else {
                            let a2750 = v7;
                            let a2751 = 3u64;
                            if nat_eq(a2750, a2751) {
                                let m2752 = f532(v1.clone())?;
                                match m2752 {
                                    None => {
                                        f533(v1.clone())
                                    }
                                    Some(_) => {
                                        true
                                    }
                                }
                            } else {
                                let a2753 = v7;
                                let a2754 = 4u64;
                                if nat_eq(a2753, a2754) {
                                    if f534(v1.clone()) {
                                        true
                                    } else {
                                        f535(v1.clone())
                                    }
                                } else {
                                    false
                                }
                            }
                        } {
                            f529(v1, v3, v4)
                        } else {
                            Ok(Adt22::C1(f421(v3)?))
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

pub fn f414(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Adt32> {
    let a2755 = f206(v0.clone(), v1)?;
    let a2756 = 34u64;
    if nat_eq(a2755, a2756) {
        f536(v0.clone(), {
            let a2757 = v1;
            let a2758 = 1u64;
            nat_add(a2757, a2758)?
        }, v2, v3, {
            let a2760 = {
                let a2759 = v0;
                length_bytes(a2759)
            };
            let a2761 = 2u64;
            nat_add(a2760, a2761)?
        }, Bytes::lit(&[]))
    } else {
        f537(v0.clone(), {
            let a2762 = v1;
            let a2763 = 1u64;
            nat_add(a2762, a2763)?
        }, v2, v3, {
            let a2765 = {
                let a2764 = v0;
                length_bytes(a2764)
            };
            let a2766 = 2u64;
            nat_add(a2765, a2766)?
        }, Bytes::lit(&[]))
    }
}

pub fn f415(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: List<Bytes>, v6: Adt21) -> R<Adt22> {
    let m2767 = v3;
    if m2767 == 0 {
        Ok(Adt22::C1(f137(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v7 = m2767 - 1;
        let a2768 = v4;
        let a2769 = 1u64;
        if nat_eq(a2768, a2769) {
            let m2770 = f415(v0.clone(), v1, v2, v7, 0u64, List::<Bytes>::nil(), Adt21::C0(0u64))?;
            match m2770 {
                Adt22::C0(v9, v10) => {
                    let m2771 = f538(v0.clone(), v10, v2)?;
                    match m2771 {
                        Ok(v12) => {
                            let v13: u64 = f206(v0.clone(), v12)?;
                            let a2772 = v13;
                            let a2773 = 58u64;
                            if nat_eq(a2772, a2773) {
                                Ok(Adt22::C1(f137(Str::lit("a single-pair mapping inside a flow sequence is not read (YAML 1.2.2 [150])"), v12)?))
                            } else {
                                let a2774 = v13;
                                let a2775 = 93u64;
                                if nat_eq(a2774, a2775) {
                                    Ok(Adt22::C0(Adt2::C9(std::rc::Rc::new(v9), std::rc::Rc::new(Adt2::C8)), {
                                        let a2776 = v12;
                                        let a2777 = 1u64;
                                        nat_add(a2776, a2777)?
                                    }))
                                } else {
                                    let a2778 = v13;
                                    let a2779 = 44u64;
                                    if nat_eq(a2778, a2779) {
                                        let m2782 = f538(v0.clone(), {
                                            let a2780 = v12;
                                            let a2781 = 1u64;
                                            nat_add(a2780, a2781)?
                                        }, v2)?;
                                        match m2782 {
                                            Ok(v15) => {
                                                let a2783 = f206(v0.clone(), v15)?;
                                                let a2784 = 93u64;
                                                if nat_eq(a2783, a2784) {
                                                    Ok(Adt22::C0(Adt2::C9(std::rc::Rc::new(v9), std::rc::Rc::new(Adt2::C8)), {
                                                        let a2785 = v15;
                                                        let a2786 = 1u64;
                                                        nat_add(a2785, a2786)?
                                                    }))
                                                } else {
                                                    let m2787 = f415(v0, v15, v2, v7, 1u64, List::<Bytes>::nil(), Adt21::C0(0u64))?;
                                                    match m2787 {
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
                                        Ok(Adt22::C1(f137(Str::lit("a flow sequence entry must be followed by , or ] (YAML 1.2.2 [137])"), v12)?))
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
            let a2788 = v4;
            let a2789 = 2u64;
            if nat_eq(a2788, a2789) {
                let m2790 = f539(v0.clone(), v1, v2)?;
                match m2790 {
                    Adt32::C0(v20, v21) => {
                        let m2791 = f538(v0.clone(), v21, v2)?;
                        match m2791 {
                            Ok(v23) => {
                                let m2805 = {
                                    let a2792 = f206(v0.clone(), v23)?;
                                    let a2793 = 44u64;
                                    if if nat_eq(a2792, a2793) {
                                        true
                                    } else {
                                        let a2794 = f206(v0.clone(), v23)?;
                                        let a2795 = 125u64;
                                        nat_eq(a2794, a2795)
                                    } {
                                        Adt22::C0(Adt2::C0, v23)
                                    } else {
                                        if f208({
                                            let a2796 = f206(v0.clone(), v23)?;
                                            let a2797 = 58u64;
                                            nat_eq(a2796, a2797)
                                        }) {
                                            Adt22::C1(f137(Str::lit("a flow mapping entry must be a key, then : and its value, or a key alone (YAML 1.2.2 [142], [146])"), v23)?)
                                        } else {
                                            let m2800 = f538(v0.clone(), {
                                                let a2798 = v23;
                                                let a2799 = 1u64;
                                                nat_add(a2798, a2799)?
                                            }, v2)?;
                                            match m2800 {
                                                Ok(v25) => {
                                                    let a2801 = f206(v0.clone(), v25)?;
                                                    let a2802 = 44u64;
                                                    if if nat_eq(a2801, a2802) {
                                                        true
                                                    } else {
                                                        let a2803 = f206(v0.clone(), v25)?;
                                                        let a2804 = 125u64;
                                                        nat_eq(a2803, a2804)
                                                    } {
                                                        Adt22::C0(Adt2::C0, v25)
                                                    } else {
                                                        f415(v0.clone(), v25, v2, v7, 0u64, List::<Bytes>::nil(), Adt21::C0(0u64))?
                                                    }
                                                }
                                                Err(v24) => {
                                                    Adt22::C1(v24)
                                                }
                                            }
                                        }
                                    }
                                };
                                match m2805 {
                                    Adt22::C0(v27, v28) => {
                                        if f320(v20.clone(), v5.clone()) {
                                            Ok(Adt22::C1(f137(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                        } else {
                                            let m2806 = f538(v0.clone(), v28, v2)?;
                                            match m2806 {
                                                Ok(v30) => {
                                                    let v31: u64 = f206(v0.clone(), v30)?;
                                                    let a2807 = v31;
                                                    let a2808 = 125u64;
                                                    if nat_eq(a2807, a2808) {
                                                        Ok(Adt22::C0(Adt2::C10(v20, std::rc::Rc::new(v27), std::rc::Rc::new(Adt2::C8)), {
                                                            let a2809 = v30;
                                                            let a2810 = 1u64;
                                                            nat_add(a2809, a2810)?
                                                        }))
                                                    } else {
                                                        let a2811 = v31;
                                                        let a2812 = 44u64;
                                                        if nat_eq(a2811, a2812) {
                                                            let m2815 = f538(v0.clone(), {
                                                                let a2813 = v30;
                                                                let a2814 = 1u64;
                                                                nat_add(a2813, a2814)?
                                                            }, v2)?;
                                                            match m2815 {
                                                                Ok(v33) => {
                                                                    let a2816 = f206(v0.clone(), v33)?;
                                                                    let a2817 = 125u64;
                                                                    if nat_eq(a2816, a2817) {
                                                                        Ok(Adt22::C0(Adt2::C10(v20, std::rc::Rc::new(v27), std::rc::Rc::new(Adt2::C8)), {
                                                                            let a2818 = v33;
                                                                            let a2819 = 1u64;
                                                                            nat_add(a2818, a2819)?
                                                                        }))
                                                                    } else {
                                                                        let m2820 = f415(v0, v33, v2, v7, 2u64, List::cons(v20.clone(), v5), Adt21::C0(0u64))?;
                                                                        match m2820 {
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
                                                            Ok(Adt22::C1(f137(Str::lit("a flow mapping entry must be followed by , or } (YAML 1.2.2 [140])"), v30)?))
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
                    Adt32::C1(v19) => {
                        Ok(Adt22::C1(v19))
                    }
                }
            } else {
                let v37: u64 = f206(v0.clone(), v1)?;
                let a2821 = v37;
                let a2822 = 33u64;
                if nat_eq(a2821, a2822) {
                    if f309(v6) {
                        Ok(Adt22::C1(f137(Str::lit("a node holds at most one tag (YAML 1.2.2 [96])"), v1)?))
                    } else {
                        let v38: Adt21 = f310(v0.clone(), v1)?;
                        let m2823 = f311(v38.clone());
                        match m2823 {
                            None => {
                                let m2824 = f538(v0.clone(), f312(v0.clone(), v1)?, v2)?;
                                match m2824 {
                                    Ok(v41) => {
                                        let a2825 = f315(f8(Str::lit(",]}:")), f206(v0.clone(), v41)?)?;
                                        let a2826 = 4u64;
                                        if nat_lt(a2825, a2826) {
                                            f314(v38, v41, v41)
                                        } else {
                                            f415(v0, v41, v2, v7, 0u64, List::<Bytes>::nil(), v38)
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
                    let a2827 = v37;
                    let a2828 = 91u64;
                    if if nat_eq(a2827, a2828) {
                        true
                    } else {
                        let a2829 = v37;
                        let a2830 = 123u64;
                        nat_eq(a2829, a2830)
                    } {
                        let m2833 = f538(v0.clone(), {
                            let a2831 = v1;
                            let a2832 = 1u64;
                            nat_add(a2831, a2832)?
                        }, v2)?;
                        match m2833 {
                            Ok(v43) => {
                                f313(v6, {
                                    let a2834 = v37;
                                    let a2835 = 91u64;
                                    nat_eq(a2834, a2835)
                                }, v1, {
                                    let a2838 = f206(v0.clone(), v43)?;
                                    let a2839 = {
                                        let a2836 = v37;
                                        let a2837 = 2u64;
                                        nat_add(a2836, a2837)?
                                    };
                                    if nat_eq(a2838, a2839) {
                                        Adt22::C0(Adt2::C8, {
                                            let a2840 = v43;
                                            let a2841 = 1u64;
                                            nat_add(a2840, a2841)?
                                        })
                                    } else {
                                        f415(v0, v43, v2, v7, {
                                            let a2842 = v37;
                                            let a2843 = 91u64;
                                            if nat_eq(a2842, a2843) {
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
                        let a2844 = v37;
                        let a2845 = 34u64;
                        if if nat_eq(a2844, a2845) {
                            true
                        } else {
                            let a2846 = v37;
                            let a2847 = 39u64;
                            nat_eq(a2846, a2847)
                        } {
                            let m2848 = f414(v0, v1, v2, false)?;
                            match m2848 {
                                Adt32::C0(v45, v46) => {
                                    f413(v6, v45, true, v1, v46)
                                }
                                Adt32::C1(v44) => {
                                    Ok(Adt22::C1(v44))
                                }
                            }
                        } else {
                            let m2849 = f307(v0.clone(), v1)?;
                            match m2849 {
                                None => {
                                    if f208(f416(v0.clone(), v1, true)?) {
                                        Ok(Adt22::C1(f137(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                                    } else {
                                        let v48: u64 = f417(v0.clone(), v1, true)?;
                                        let v49: u64 = f302(v0.clone(), v48)?;
                                        let m2852 = {
                                            let a2850 = f206(v0.clone(), v49)?;
                                            let a2851 = 10u64;
                                            if nat_eq(a2850, a2851) {
                                                f538(v0.clone(), v49, v2)?
                                            } else {
                                                Ok::<u64, Bytes>(v49)
                                            }
                                        };
                                        match m2852 {
                                            Ok(v51) => {
                                                let a2853 = f206(v0.clone(), v49)?;
                                                let a2854 = 10u64;
                                                if if if nat_eq(a2853, a2854) {
                                                    f208(f521(f206(v0.clone(), v51)?))
                                                } else {
                                                    false
                                                } {
                                                    f208({
                                                        let a2855 = f206(v0.clone(), v51)?;
                                                        let a2856 = 58u64;
                                                        nat_eq(a2855, a2856)
                                                    })
                                                } else {
                                                    false
                                                } {
                                                    Ok(Adt22::C1(f137(Str::lit("a multi-line plain scalar inside a flow collection is not read (YAML 1.2.2 [135])"), v49)?))
                                                } else {
                                                    f413(v6, f316(v0, v1, v48), false, v1, v48)
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

pub fn f416(v0: Bytes, v1: u64, v2: bool) -> R<bool> {
    let v3: u64 = f206(v0.clone(), v1)?;
    if if f299(v3) {
        true
    } else {
        f300(v3)
    } {
        Ok(false)
    } else {
        let a2857 = v3;
        let a2858 = 45u64;
        if if if nat_eq(a2857, a2858) {
            true
        } else {
            let a2859 = v3;
            let a2860 = 63u64;
            nat_eq(a2859, a2860)
        } {
            true
        } else {
            let a2861 = v3;
            let a2862 = 58u64;
            nat_eq(a2861, a2862)
        } {
            Ok(f411(f206(v0, {
                let a2863 = v1;
                let a2864 = 1u64;
                nat_add(a2863, a2864)?
            })?, v2))
        } else {
            Ok(f208(f540(v3)?))
        }
    }
}

pub fn f417(v0: Bytes, v1: u64, v2: bool) -> R<u64> {
    let (_, h2865) = {
        let (h2866, _) = f270(Fn6::F541(v2, v0.clone()), {
            let a2868 = {
                let a2867 = v0;
                length_bytes(a2867)
            };
            let a2869 = 1u64;
            nat_add(a2868, a2869)?
        }, (v1, v1))?;
        h2866
    };
    Ok(h2865)
}

pub fn f418(v0: Bytes, p2894: u64, v2: u64, p2895: u64, p2896: Bytes) -> R<Adt32> {
    let m2893 = (p2894, (p2895, p2896));
    {
        let mut m2893 = m2893;
        loop {
            let step = {
                let (v1, (v3, v4)) = m2893;
                let m2870 = v3;
                if m2870 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C1(f137(Str::lit("a plain scalar has more lines than the input"), v1)?)))
                } else {
                    let v5 = m2870 - 1;
                    let v6: u64 = f302(v0.clone(), v1)?;
                    let v7: u64 = f206(v0.clone(), v6)?;
                    let a2871 = v7;
                    let a2872 = 58u64;
                    if nat_eq(a2871, a2872) {
                        Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C1(f542(v6)?)))
                    } else {
                        let a2873 = v7;
                        let a2874 = 35u64;
                        if if nat_eq(a2873, a2874) {
                            true
                        } else {
                            let a2875 = v7;
                            let a2876 = 256u64;
                            nat_eq(a2875, a2876)
                        } {
                            Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C0(v4, f303(v0.clone(), v6)?)))
                        } else {
                            let v8: (u64, u64) = f543(v0.clone(), {
                                let a2877 = v6;
                                let a2878 = 1u64;
                                nat_add(a2877, a2878)?
                            })?;
                            let v9: u64 = {
                                let (h2879, _) = v8;
                                h2879
                            };
                            let v10: u64 = f304(v0.clone(), v9)?;
                            let v11: u64 = f302(v0.clone(), v10)?;
                            let v12: u64 = f206(v0.clone(), v11)?;
                            let a2880 = v12;
                            let a2881 = 256u64;
                            if if if if nat_eq(a2880, a2881) {
                                true
                            } else {
                                let a2884 = {
                                    let a2882 = v10;
                                    let a2883 = v9;
                                    nat_sub(a2882, a2883)
                                };
                                let a2885 = v2;
                                nat_lt(a2884, a2885)
                            } {
                                true
                            } else {
                                let a2886 = v12;
                                let a2887 = 35u64;
                                nat_eq(a2886, a2887)
                            } {
                                true
                            } else {
                                f204(v0.clone(), v9)?
                            } {
                                Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C0(v4, {
                                    let a2888 = v6;
                                    let a2889 = 1u64;
                                    nat_add(a2888, a2889)?
                                })))
                            } else {
                                let v13: u64 = f417(v0.clone(), v11, false)?;
                                let a2890 = v13;
                                let a2891 = v11;
                                if nat_eq(a2890, a2891) {
                                    Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C1(f542(v11)?)))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Adt32>>((v13, (v5, f7(List::cons(v4, List::cons(f544({
                                        let (_, h2892) = v8;
                                        h2892
                                    }), List::cons(f316(v0.clone(), v11, v13), List::<Bytes>::nil())))))))
                                }
                            }
                        }
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

pub fn f419(v0: Bytes, v1: u64) -> R<u64> {
    let v2: bool = {
        let a2899 = f206(v0.clone(), {
            let a2897 = v1;
            let a2898 = 1u64;
            nat_add(a2897, a2898)?
        })?;
        let a2900 = 60u64;
        nat_eq(a2899, a2900)
    };
    let (h2901, _) = f201(Fn5::F545(v0.clone(), v2), {
        let a2903 = {
            let a2902 = v0;
            length_bytes(a2902)
        };
        let a2904 = 1u64;
        nat_add(a2903, a2904)?
    }, if v2 {
        let a2905 = v1;
        let a2906 = 2u64;
        nat_add(a2905, a2906)?
    } else {
        let a2907 = v1;
        let a2908 = 1u64;
        nat_add(a2907, a2908)?
    })?;
    Ok(h2901)
}

pub fn f420(v0: Bytes, v1: u64) -> R<Adt21> {
    let v2: u64 = f546(v0, List::cons(f8(Str::lit("str")), List::cons(f8(Str::lit("int")), List::cons(f8(Str::lit("float")), List::cons(f8(Str::lit("bool")), List::cons(f8(Str::lit("null")), List::cons(f8(Str::lit("seq")), List::cons(f8(Str::lit("map")), List::<Bytes>::nil()))))))), 1u64)?;
    let a2909 = v2;
    let a2910 = 0u64;
    if nat_eq(a2909, a2910) {
        Ok(Adt21::C2(f137(Str::lit("a tag outside the YAML 1.2.2 core schema is not read (YAML 1.2.2 section 10.3)"), v1)?))
    } else {
        Ok(Adt21::C0(v2))
    }
}

pub fn f421(v0: u64) -> R<Bytes> {
    f137(Str::lit("a node's content does not match its tag (YAML 1.2.2 section 10.3.2)"), v0)
}

pub fn f422(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f547(v0, v1, v2)
}

pub fn f423(v0: Bytes, v1: u64) -> R<bool> {
    let a2911 = f206(v0.clone(), v1)?;
    let a2912 = 58u64;
    if nat_eq(a2911, a2912) {
        if f299(f206(v0.clone(), {
            let a2913 = v1;
            let a2914 = 1u64;
            nat_add(a2913, a2914)?
        })?) {
            Ok(true)
        } else {
            Ok(f300(f206(v0, {
                let a2915 = v1;
                let a2916 = 1u64;
                nat_add(a2915, a2916)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f424(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a2917 = v0;
        utf8_encode(a2917)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f91(v1)?, List::<Bytes>::nil())))))
}

pub fn f425(p2920: List<Adt23>, p2921: List<Adt23>) -> List<Adt23> {
    let m2919 = (p2920, p2921);
    {
        let mut m2919 = m2919;
        loop {
            let step = {
                let (v0, v1) = m2919;
                let m2918 = v0;
                match m2918.uncons() {
                    None => {
                        Err::<(List<Adt23>, List<Adt23>), List<Adt23>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt23>, List<Adt23>), List<Adt23>>((v3, List::cons(v2, v1)))
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

pub fn f426(v0: u64) -> bool {
    let a2922 = 65u64;
    let a2923 = v0;
    if if nat_le(a2922, a2923) {
        let a2924 = v0;
        let a2925 = 90u64;
        nat_le(a2924, a2925)
    } else {
        false
    } {
        true
    } else {
        let a2926 = 97u64;
        let a2927 = v0;
        if nat_le(a2926, a2927) {
            let a2928 = v0;
            let a2929 = 122u64;
            nat_le(a2928, a2929)
        } else {
            false
        }
    }
}

pub fn f427(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h2930, _) = f201(Fn5::F548(v1, v0.clone()), {
        let a2932 = {
            let a2931 = v0;
            length_bytes(a2931)
        };
        let a2933 = 1u64;
        nat_add(a2932, a2933)?
    }, v2)?;
    Ok(h2930)
}

pub fn f428(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f160(v0.clone(), v1)?;
    let a2934 = v2;
    let a2935 = 48u64;
    if nat_eq(a2934, a2935) {
        let v3: u64 = f160(v0.clone(), {
            let a2936 = v1;
            let a2937 = 1u64;
            nat_add(a2936, a2937)?
        })?;
        let a2938 = v3;
        let a2939 = 88u64;
        if if if nat_eq(a2938, a2939) {
            true
        } else {
            let a2940 = v3;
            let a2941 = 120u64;
            nat_eq(a2940, a2941)
        } {
            f549(f160(v0.clone(), {
                let a2942 = v1;
                let a2943 = 2u64;
                nat_add(a2942, a2943)?
            })?)
        } else {
            false
        } {
            f427(v0, 1u64, {
                let a2944 = v1;
                let a2945 = 2u64;
                nat_add(a2944, a2945)?
            })
        } else {
            f427(v0, 2u64, {
                let a2946 = v1;
                let a2947 = 1u64;
                nat_add(a2946, a2947)?
            })
        }
    } else {
        if f550(v2) {
            f427(v0, 0u64, v1)
        } else {
            Ok(v1)
        }
    }
}

pub fn f429(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f427(v0.clone(), 0u64, v1)?;
    let a2948 = f160(v0.clone(), v2)?;
    let a2949 = 46u64;
    if nat_eq(a2948, a2949) {
        let v3: u64 = f427(v0.clone(), 0u64, {
            let a2950 = v2;
            let a2951 = 1u64;
            nat_add(a2950, a2951)?
        })?;
        let a2952 = v1;
        let a2953 = v2;
        if if nat_lt(a2952, a2953) {
            true
        } else {
            let a2956 = {
                let a2954 = v2;
                let a2955 = 1u64;
                nat_add(a2954, a2955)?
            };
            let a2957 = v3;
            nat_lt(a2956, a2957)
        } {
            f551(v0, v3)
        } else {
            Ok(v1)
        }
    } else {
        let a2958 = v1;
        let a2959 = v2;
        if nat_lt(a2958, a2959) {
            let v4: u64 = f551(v0, v2)?;
            let a2960 = v2;
            let a2961 = v4;
            if nat_lt(a2960, a2961) {
                Ok(v4)
            } else {
                Ok(v1)
            }
        } else {
            Ok(v1)
        }
    }
}

pub fn f430(v0: List<Adt23>, v1: Str) -> bool {
    let m2962 = v0;
    match m2962.uncons() {
        None => {
            false
        }
        Some((v2, _)) => {
            f552(v2, {
                let a2963 = v1;
                utf8_encode(a2963)
            })
        }
    }
}

pub fn f431(p2976: u64, p2977: List<Adt23>, p2978: List<Bytes>, v3: bool) -> Result<List<Adt23>, Bytes> {
    let m2975 = (p2976, (p2977, p2978));
    {
        let mut m2975 = m2975;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2975;
                let m2964 = v0;
                if m2964 == 0 {
                    Err::<(u64, (List<Adt23>, List<Bytes>)), Result<List<Adt23>, Bytes>>(Err::<List<Adt23>, Bytes>(f8(Str::lit("brackets nest deeper than the input is long"))))
                } else {
                    let v4 = m2964 - 1;
                    let m2965 = v1;
                    match m2965.uncons() {
                        None => {
                            Err::<(u64, (List<Adt23>, List<Bytes>)), Result<List<Adt23>, Bytes>>(Err::<List<Adt23>, Bytes>(f8(Str::lit("the input ends inside brackets or before the ; that closes a member (Web IDL, IDL grammar)"))))
                        }
                        Some((v5, v6)) => {
                            let a2966 = f553(v5.clone());
                            let a2967 = 3u64;
                            if nat_eq(a2966, a2967) {
                                Ok::<(u64, (List<Adt23>, List<Bytes>)), Result<List<Adt23>, Bytes>>((v4, (v6, v2)))
                            } else {
                                let v7: Bytes = f554(v5);
                                let v8: Bytes = f555(v7.clone());
                                if !f32(v8.clone(), Bytes::lit(&[])) {
                                    Ok::<(u64, (List<Adt23>, List<Bytes>)), Result<List<Adt23>, Bytes>>((v4, (v6, List::cons(v8, v2))))
                                } else {
                                    if f556(v7.clone()) {
                                        let m2968 = v2;
                                        match m2968.uncons() {
                                            None => {
                                                Err::<(u64, (List<Adt23>, List<Bytes>)), Result<List<Adt23>, Bytes>>(Err::<List<Adt23>, Bytes>(f7(List::cons(f8(Str::lit("the closing bracket ")), List::cons(v7, List::cons(f8(Str::lit(" has no opening bracket (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
                                            }
                                            Some((v9, v10)) => {
                                                if f32(v9.clone(), v7.clone()) {
                                                    if if !v3 {
                                                        let a2970 = {
                                                            let a2969 = v10.clone();
                                                            length_list(a2969)
                                                        };
                                                        let a2971 = 0u64;
                                                        nat_eq(a2970, a2971)
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
                                                let a2973 = {
                                                    let a2972 = v2.clone();
                                                    length_list(a2972)
                                                };
                                                let a2974 = 0u64;
                                                nat_eq(a2973, a2974)
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
                Ok(next) => { m2975 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f432(v0: List<Adt23>) -> List<Adt23> {
    let m2979 = v0;
    match m2979.uncons() {
        None => {
            List::<Adt23>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f433(v0: u64, v1: u64, v2: List<Adt23>, v3: bool) -> R<Result<(Adt13, List<Adt23>), Bytes>> {
    let m2980 = f436(v2);
    match m2980 {
        Ok(v5) => {
            let v6: bool = if v3 {
                f430(f438(v5.clone()), Str::lit(":"))
            } else {
                false
            };
            let m2981 = if v6 {
                f436(f432(f438(v5.clone())))
            } else {
                Ok::<Adt33, Bytes>(Adt33::C0(Bytes::lit(&[]), f438(v5.clone())))
            };
            match m2981 {
                Ok(v8) => {
                    let m2982 = f437(f438(v8.clone()), Str::lit("{"));
                    match m2982 {
                        Ok(v10) => {
                            let m2987 = {
                                let a2983 = v1;
                                let a2984 = 2u64;
                                if if nat_eq(a2983, a2984) {
                                    true
                                } else {
                                    let a2985 = v1;
                                    let a2986 = 7u64;
                                    nat_eq(a2985, a2986)
                                } {
                                    f557(v0, v10, List::<Adt14>::nil())?
                                } else {
                                    f558(v0, v10, List::<Adt14>::nil())?
                                }
                            };
                            match m2987 {
                                Ok(v12) => {
                                    Ok(Ok::<(Adt13, List<Adt23>), Bytes>((Adt13::C0(v1, f440(v5), f440(v8), {
                                        let (h2988, _) = v12;
                                        h2988
                                    }, List::<Bytes>::nil(), f441(Bytes::lit(&[]))), {
                                        let (_, h2989) = v12;
                                        h2989
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

pub fn f434(v0: List<Adt23>, v1: Bytes) -> R<Result<(Adt13, List<Adt23>), Bytes>> {
    let m2990 = f436(v0);
    match m2990 {
        Ok(v3) => {
            let m2991 = f559(f438(v3.clone()))?;
            match m2991 {
                Ok(v5) => {
                    Ok(Ok::<(Adt13, List<Adt23>), Bytes>((Adt13::C0(9u64, f440(v3), Bytes::lit(&[]), List::<Adt14>::nil(), List::cons(v1, List::<Bytes>::nil()), f441(Bytes::lit(&[]))), v5)))
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

pub fn f435(v0: Str, v1: List<Adt23>) -> Bytes {
    f7(List::cons(f8(Str::lit("expected ")), List::cons({
        let a2992 = v0;
        utf8_encode(a2992)
    }, List::cons(f8(Str::lit(" but found ")), List::cons(f560(v1), List::cons(f8(Str::lit(" (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
}

pub fn f436(v0: List<Adt23>) -> Result<Adt33, Bytes> {
    let m2993 = v0.clone();
    match m2993.uncons() {
        None => {
            Err::<Adt33, Bytes>(f435(Str::lit("an identifier"), v0))
        }
        Some((v1, v2)) => {
            let a2994 = f553(v1.clone());
            let a2995 = 0u64;
            if nat_eq(a2994, a2995) {
                Ok::<Adt33, Bytes>(Adt33::C0(f561(f554(v1)), v2))
            } else {
                Err::<Adt33, Bytes>(f435(Str::lit("an identifier"), v0))
            }
        }
    }
}

pub fn f437(v0: List<Adt23>, v1: Str) -> Result<List<Adt23>, Bytes> {
    if f430(v0.clone(), v1.clone()) {
        Ok::<List<Adt23>, Bytes>(f432(v0))
    } else {
        Err::<List<Adt23>, Bytes>(f435(v1, v0))
    }
}

pub fn f438(v0: Adt33) -> List<Adt23> {
    let m2996 = v0;
    match m2996 {
        Adt33::C0(_, v2) => {
            v2
        }
    }
}

pub fn f439(p3004: u64, p3005: List<Adt23>, p3006: List<Bytes>) -> Result<(List<Bytes>, List<Adt23>), Bytes> {
    let m3003 = (p3004, (p3005, p3006));
    {
        let mut m3003 = m3003;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3003;
                let m2997 = v0;
                if m2997 == 0 {
                    Err::<(u64, (List<Adt23>, List<Bytes>)), Result<(List<Bytes>, List<Adt23>), Bytes>>(Err::<(List<Bytes>, List<Adt23>), Bytes>(f8(Str::lit("an enumeration has more values than the input has tokens"))))
                } else {
                    let v3 = m2997 - 1;
                    let m2998 = v1.clone();
                    match m2998.uncons() {
                        None => {
                            Err::<(u64, (List<Adt23>, List<Bytes>)), Result<(List<Bytes>, List<Adt23>), Bytes>>(Err::<(List<Bytes>, List<Adt23>), Bytes>(f435(Str::lit("a string"), v1)))
                        }
                        Some((v4, v5)) => {
                            let a2999 = f553(v4.clone());
                            let a3000 = 3u64;
                            if !nat_eq(a2999, a3000) {
                                Err::<(u64, (List<Adt23>, List<Bytes>)), Result<(List<Bytes>, List<Adt23>), Bytes>>(Err::<(List<Bytes>, List<Adt23>), Bytes>(f435(Str::lit("a string"), v1)))
                            } else {
                                if f430(v5.clone(), Str::lit(",")) {
                                    if f430(f432(v5.clone()), Str::lit("}")) {
                                        let m3001 = f437(f432(f432(v5)), Str::lit(";"));
                                        match m3001 {
                                            Ok(v7) => {
                                                Err::<(u64, (List<Adt23>, List<Bytes>)), Result<(List<Bytes>, List<Adt23>), Bytes>>(Ok::<(List<Bytes>, List<Adt23>), Bytes>((f68(List::cons(f554(v4), v2)), v7)))
                                            }
                                            Err(v6) => {
                                                Err::<(u64, (List<Adt23>, List<Bytes>)), Result<(List<Bytes>, List<Adt23>), Bytes>>(Err::<(List<Bytes>, List<Adt23>), Bytes>(v6))
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (List<Adt23>, List<Bytes>)), Result<(List<Bytes>, List<Adt23>), Bytes>>((v3, (f432(v5), List::cons(f554(v4), v2))))
                                    }
                                } else {
                                    if f430(v5.clone(), Str::lit("}")) {
                                        let m3002 = f437(f432(v5), Str::lit(";"));
                                        match m3002 {
                                            Ok(v9) => {
                                                Err::<(u64, (List<Adt23>, List<Bytes>)), Result<(List<Bytes>, List<Adt23>), Bytes>>(Ok::<(List<Bytes>, List<Adt23>), Bytes>((f68(List::cons(f554(v4), v2)), v9)))
                                            }
                                            Err(v8) => {
                                                Err::<(u64, (List<Adt23>, List<Bytes>)), Result<(List<Bytes>, List<Adt23>), Bytes>>(Err::<(List<Bytes>, List<Adt23>), Bytes>(v8))
                                            }
                                        }
                                    } else {
                                        Err::<(u64, (List<Adt23>, List<Bytes>)), Result<(List<Bytes>, List<Adt23>), Bytes>>(Err::<(List<Bytes>, List<Adt23>), Bytes>(f435(Str::lit(", or }"), v5)))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3003 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f440(v0: Adt33) -> Bytes {
    let m3007 = v0;
    match m3007 {
        Adt33::C0(v1, _) => {
            v1
        }
    }
}

pub fn f441(v0: Bytes) -> Adt15 {
    Adt15::C0(0u64, v0, List::<Bytes>::nil(), Bytes::lit(&[]), Bytes::lit(&[]))
}

pub fn f442(v0: u64, v1: List<Adt23>, v2: u64) -> R<Result<Adt34, Bytes>> {
    let m3008 = v0;
    if m3008 == 0 {
        Ok(Err::<Adt34, Bytes>(f8(Str::lit("a type nests deeper than the input is long"))))
    } else {
        let v3 = m3008 - 1;
        let a3009 = v2;
        let a3010 = 0u64;
        if nat_eq(a3009, a3010) {
            let m3011 = f324(v1)?;
            match m3011 {
                Ok(v5) => {
                    if f430(v5.clone(), Str::lit("(")) {
                        let m3012 = f442(v3, f432(v5), 1u64)?;
                        match m3012 {
                            Ok(v7) => {
                                if f430(f443(v7.clone()), Str::lit("?")) {
                                    Ok(Ok::<Adt34, Bytes>(Adt34::C0(Adt15::C0(0u64, Bytes::lit(&[]), f343(f444(v7.clone())), f8(Str::lit("a nullable union type is not translated yet, because the typedef identifier names only the union (Web IDL 2.13.32)")), Bytes::lit(&[])), f432(f443(v7)))))
                                } else {
                                    Ok(Ok::<Adt34, Bytes>(Adt34::C0(f444(v7.clone()), f443(v7))))
                                }
                            }
                            Err(v6) => {
                                Ok(Err::<Adt34, Bytes>(v6))
                            }
                        }
                    } else {
                        let m3013 = v5.clone();
                        match m3013.uncons() {
                            None => {
                                Ok(Err::<Adt34, Bytes>(f435(Str::lit("a type"), v5)))
                            }
                            Some((v8, v9)) => {
                                let v10: Bytes = f554(v8.clone());
                                let a3014 = f553(v8);
                                let a3015 = 0u64;
                                if !nat_eq(a3014, a3015) {
                                    Ok(Err::<Adt34, Bytes>(f435(Str::lit("a type"), v5)))
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
                                        let m3016 = f437(v9, Str::lit("<"));
                                        match m3016 {
                                            Ok(v12) => {
                                                let m3017 = f442(v3, v12, 0u64)?;
                                                match m3017 {
                                                    Ok(v14) => {
                                                        let m3018 = f437(f443(v14.clone()), Str::lit(">"));
                                                        match m3018 {
                                                            Ok(v16) => {
                                                                Ok(f562(f563(Str::lit("List"), f444(v14)), v16))
                                                            }
                                                            Err(v15) => {
                                                                Ok(Err::<Adt34, Bytes>(v15))
                                                            }
                                                        }
                                                    }
                                                    Err(v13) => {
                                                        Ok(Err::<Adt34, Bytes>(v13))
                                                    }
                                                }
                                            }
                                            Err(v11) => {
                                                Ok(Err::<Adt34, Bytes>(v11))
                                            }
                                        }
                                    } else {
                                        if f32(v10.clone(), f8(Str::lit("async_sequence"))) {
                                            let m3019 = f437(v9, Str::lit("<"));
                                            match m3019 {
                                                Ok(v18) => {
                                                    let m3020 = f442(v3, v18, 0u64)?;
                                                    match m3020 {
                                                        Ok(v20) => {
                                                            let m3021 = f437(f443(v20), Str::lit(">"));
                                                            match m3021 {
                                                                Ok(v22) => {
                                                                    Ok(f562(f564(f8(Str::lit("async_sequence types are not translated, because an async sequence is produced over time and is not a value (Web IDL 2.13.29)"))), v22))
                                                                }
                                                                Err(v21) => {
                                                                    Ok(Err::<Adt34, Bytes>(v21))
                                                                }
                                                            }
                                                        }
                                                        Err(v19) => {
                                                            Ok(Err::<Adt34, Bytes>(v19))
                                                        }
                                                    }
                                                }
                                                Err(v17) => {
                                                    Ok(Err::<Adt34, Bytes>(v17))
                                                }
                                            }
                                        } else {
                                            if f32(v10.clone(), f8(Str::lit("Promise"))) {
                                                let m3022 = f437(v9, Str::lit("<"));
                                                match m3022 {
                                                    Ok(v24) => {
                                                        let m3023 = f442(v3, v24, 0u64)?;
                                                        match m3023 {
                                                            Ok(v26) => {
                                                                let m3024 = f437(f443(v26), Str::lit(">"));
                                                                match m3024 {
                                                                    Ok(v28) => {
                                                                        Ok(f562(f564(f8(Str::lit("Promise types are not translated, because a promise is a deferred result, not a value (Web IDL 2.13.31)"))), v28))
                                                                    }
                                                                    Err(v27) => {
                                                                        Ok(Err::<Adt34, Bytes>(v27))
                                                                    }
                                                                }
                                                            }
                                                            Err(v25) => {
                                                                Ok(Err::<Adt34, Bytes>(v25))
                                                            }
                                                        }
                                                    }
                                                    Err(v23) => {
                                                        Ok(Err::<Adt34, Bytes>(v23))
                                                    }
                                                }
                                            } else {
                                                if f32(v10.clone(), f8(Str::lit("record"))) {
                                                    let m3025 = f437(v9, Str::lit("<"));
                                                    match m3025 {
                                                        Ok(v30) => {
                                                            let m3026 = v30.clone();
                                                            match m3026.uncons() {
                                                                None => {
                                                                    Ok(Err::<Adt34, Bytes>(f435(Str::lit("a string type"), v30)))
                                                                }
                                                                Some((v31, v32)) => {
                                                                    if if f32(f554(v31.clone()), f8(Str::lit("DOMString"))) {
                                                                        true
                                                                    } else {
                                                                        if f32(f554(v31.clone()), f8(Str::lit("USVString"))) {
                                                                            true
                                                                        } else {
                                                                            f32(f554(v31.clone()), f8(Str::lit("ByteString")))
                                                                        }
                                                                    } {
                                                                        let m3027 = f437(v32, Str::lit(","));
                                                                        match m3027 {
                                                                            Ok(v34) => {
                                                                                let m3028 = f442(v3, v34, 0u64)?;
                                                                                match m3028 {
                                                                                    Ok(v36) => {
                                                                                        let m3029 = f437(f443(v36.clone()), Str::lit(">"));
                                                                                        match m3029 {
                                                                                            Ok(v38) => {
                                                                                                let m3030 = f565(f554(v31));
                                                                                                match m3030 {
                                                                                                    None => {
                                                                                                        Ok(Err::<Adt34, Bytes>(f435(Str::lit("a string type"), v30)))
                                                                                                    }
                                                                                                    Some(v39) => {
                                                                                                        let v40: Adt15 = f566(f444(v36));
                                                                                                        Ok(f562(Adt15::C0(0u64, f7(List::cons(f8(Str::lit("(List (")), List::cons(f342(v39), List::cons(f8(Str::lit(" * ")), List::cons(f342(v40.clone()), List::cons(f8(Str::lit("))")), List::<Bytes>::nil())))))), f343(v40.clone()), f340(v40), Bytes::lit(&[])), v38))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            Err(v37) => {
                                                                                                Ok(Err::<Adt34, Bytes>(v37))
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    Err(v35) => {
                                                                                        Ok(Err::<Adt34, Bytes>(v35))
                                                                                    }
                                                                                }
                                                                            }
                                                                            Err(v33) => {
                                                                                Ok(Err::<Adt34, Bytes>(v33))
                                                                            }
                                                                        }
                                                                    } else {
                                                                        Ok(Err::<Adt34, Bytes>(f435(Str::lit("DOMString, USVString or ByteString"), v30)))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Err(v29) => {
                                                            Ok(Err::<Adt34, Bytes>(v29))
                                                        }
                                                    }
                                                } else {
                                                    if f32(v10.clone(), f8(Str::lit("unsigned"))) {
                                                        if f430(v9.clone(), Str::lit("short")) {
                                                            Ok(f562(f441(f8(Str::lit("Nat"))), f432(v9)))
                                                        } else {
                                                            if f430(v9.clone(), Str::lit("long")) {
                                                                let v41: List<Adt23> = f432(v9);
                                                                Ok(f562(f441(f8(Str::lit("Nat"))), if f430(v41.clone(), Str::lit("long")) {
                                                                    f432(v41)
                                                                } else {
                                                                    v41
                                                                }))
                                                            } else {
                                                                Ok(Err::<Adt34, Bytes>(f435(Str::lit("short or long"), v9)))
                                                            }
                                                        }
                                                    } else {
                                                        if f32(v10.clone(), f8(Str::lit("short"))) {
                                                            Ok(f562(f441(f8(Str::lit("Int"))), v9))
                                                        } else {
                                                            if f32(v10.clone(), f8(Str::lit("long"))) {
                                                                Ok(f562(f441(f8(Str::lit("Int"))), if f430(v9.clone(), Str::lit("long")) {
                                                                    f432(v9)
                                                                } else {
                                                                    v9
                                                                }))
                                                            } else {
                                                                if f32(v10.clone(), f8(Str::lit("unrestricted"))) {
                                                                    if if f430(v9.clone(), Str::lit("float")) {
                                                                        true
                                                                    } else {
                                                                        f430(v9.clone(), Str::lit("double"))
                                                                    } {
                                                                        Ok(f562(f564(f8(Str::lit("unrestricted float and unrestricted double are not translated, because its value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.13 and 2.13.15)"))), f432(v9)))
                                                                    } else {
                                                                        Ok(Err::<Adt34, Bytes>(f435(Str::lit("float or double"), v9)))
                                                                    }
                                                                } else {
                                                                    let m3031 = f565(v10.clone());
                                                                    match m3031 {
                                                                        None => {
                                                                            let v42: Bytes = f561(v10);
                                                                            Ok(f562(Adt15::C0(0u64, f327(v42.clone())?, List::cons(v42.clone(), List::<Bytes>::nil()), Bytes::lit(&[]), v42), v9))
                                                                        }
                                                                        Some(v43) => {
                                                                            Ok(f562(v43, v9))
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
                    Ok(Err::<Adt34, Bytes>(v4))
                }
            }
        } else {
            let m3032 = f442(v3, v1, 0u64)?;
            match m3032 {
                Ok(v45) => {
                    let v46: Adt15 = f444(v45.clone());
                    let v48: Adt15 = {
                        let a3033 = f341(v46.clone());
                        let a3034 = 0u64;
                        if nat_eq(a3033, a3034) {
                            if f32(f340(v46.clone()), Bytes::lit(&[])) {
                                if f32(f567(v46.clone()), Bytes::lit(&[])) {
                                    Adt15::C0(1u64, Bytes::lit(&[]), f343(v46), f8(Str::lit("a union member type that is not a named definition gives its constructor no name, so the union is not translated yet (Web IDL 2.13.32)")), Bytes::lit(&[]))
                                } else {
                                    let m3035 = f454(false, f567(v46.clone()))?;
                                    match m3035 {
                                        None => {
                                            Adt15::C0(1u64, Bytes::lit(&[]), f343(v46.clone()), f7(List::cons(f8(Str::lit("the union member ")), List::cons(f567(v46), List::cons(f8(Str::lit(" has no constructor name")), List::<Bytes>::nil())))), Bytes::lit(&[]))
                                        }
                                        Some(v47) => {
                                            Adt15::C0(1u64, f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v47, List::cons(f8(Str::lit(", fields: [")), List::cons(f342(v46.clone()), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f343(v46), Bytes::lit(&[]), Bytes::lit(&[]))
                                        }
                                    }
                                }
                            } else {
                                Adt15::C0(1u64, Bytes::lit(&[]), f343(v46.clone()), f340(v46), Bytes::lit(&[]))
                            }
                        } else {
                            v46
                        }
                    };
                    if f430(f443(v45.clone()), Str::lit("or")) {
                        let m3036 = f442(v3, f432(f443(v45)), 2u64)?;
                        match m3036 {
                            Ok(v50) => {
                                Ok(Ok::<Adt34, Bytes>(Adt34::C0(Adt15::C0(1u64, if f32(f342(v48.clone()), Bytes::lit(&[])) {
                                    f342(f444(v50.clone()))
                                } else {
                                    if f32(f342(f444(v50.clone())), Bytes::lit(&[])) {
                                        f342(v48.clone())
                                    } else {
                                        f7(List::cons(f342(v48.clone()), List::cons(f8(Str::lit(",\u{a}")), List::cons(f342(f444(v50.clone())), List::<Bytes>::nil()))))
                                    }
                                }, f221(f343(v48.clone()), f343(f444(v50.clone()))), if f32(f340(v48.clone()), Bytes::lit(&[])) {
                                    f340(f444(v50.clone()))
                                } else {
                                    f340(v48)
                                }, Bytes::lit(&[])), f443(v50))))
                            }
                            Err(v49) => {
                                Ok(Err::<Adt34, Bytes>(v49))
                            }
                        }
                    } else {
                        let a3037 = v2;
                        let a3038 = 2u64;
                        if if nat_eq(a3037, a3038) {
                            f430(f443(v45.clone()), Str::lit(")"))
                        } else {
                            false
                        } {
                            Ok(Ok::<Adt34, Bytes>(Adt34::C0(v48, f432(f443(v45)))))
                        } else {
                            Ok(Err::<Adt34, Bytes>({
                                let a3039 = v2;
                                let a3040 = 1u64;
                                if nat_eq(a3039, a3040) {
                                    f435(Str::lit("or"), f443(v45))
                                } else {
                                    f435(Str::lit("or or )"), f443(v45))
                                }
                            }))
                        }
                    }
                }
                Err(v44) => {
                    Ok(Err::<Adt34, Bytes>(v44))
                }
            }
        }
    }
}

pub fn f443(v0: Adt34) -> List<Adt23> {
    let m3041 = v0;
    match m3041 {
        Adt34::C0(_, v2) => {
            v2
        }
    }
}

pub fn f444(v0: Adt34) -> Adt15 {
    let m3042 = v0;
    match m3042 {
        Adt34::C0(v1, _) => {
            v1
        }
    }
}

pub fn f445() -> List<Bytes> {
    List::cons(f8(Str::lit("Bool")), List::cons(f8(Str::lit("ContractViolation")), List::cons(f8(Str::lit("List")), List::cons(f8(Str::lit("Nat")), List::cons(f8(Str::lit("Option")), List::cons(f8(Str::lit("Prod")), List::cons(f8(Str::lit("ReasoningFailure")), List::cons(f8(Str::lit("Result")), List::<Bytes>::nil()))))))))
}

pub fn f446(v0: Bytes) -> R<Bytes> {
    let m3045 = {
        let a3043 = v0.clone();
        let a3044 = 0u64;
        index_bytes(a3043, a3044)
    };
    match m3045 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            let a3051 = {
                let a3046 = f8(Str::lit("u"));
                let a3047 = f568(8u64, f62(v1)?, 4u64, Bytes::lit(&[]));
                append_bytes(a3046, a3047)
            };
            let a3052 = f46(v0.clone(), 1u64, {
                let a3049 = {
                    let a3048 = v0;
                    length_bytes(a3048)
                };
                let a3050 = 1u64;
                nat_sub(a3049, a3050)
            });
            Ok(append_bytes(a3051, a3052))
        }
    }
}

pub fn f447(v0: List<Adt14>) -> R<List<Adt14>> {
    let m3053 = v0;
    match m3053.uncons() {
        None => {
            Ok(List::<Adt14>::nil())
        }
        Some((v1, v2)) => {
            f569(v1, f447(v2)?)
        }
    }
}

pub fn f448(v0: List<Adt14>, v1: List<Adt14>) -> List<Adt14> {
    let m3054 = v0;
    match m3054.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f448(v3, v1))
        }
    }
}

pub fn f449(v0: Adt13) -> List<Adt14> {
    let m3055 = v0;
    match m3055 {
        Adt13::C0(_, _, _, v4, _, _) => {
            v4
        }
    }
}

pub fn f450(v0: List<Adt13>, v1: u64, v2: Bytes) -> List<Adt14> {
    let m3056 = v0;
    match m3056.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v3, v4)) => {
            let a3057 = f216(v3.clone());
            let a3058 = v1;
            if if nat_eq(a3057, a3058) {
                f32(f218(v3.clone()), v2.clone())
            } else {
                false
            } {
                f448(f449(v3), f450(v4, v1, v2))
            } else {
                f450(v4, v1, v2)
            }
        }
    }
}

pub fn f451(v0: List<Adt13>, v1: List<Adt13>, v2: Bytes) -> List<Adt14> {
    let m3059 = v0;
    match m3059.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v3, v4)) => {
            let a3060 = f216(v3.clone());
            let a3061 = 8u64;
            if if nat_eq(a3060, a3061) {
                f32(f218(v3.clone()), v2.clone())
            } else {
                false
            } {
                f448(f570(v1.clone(), f228(v3)), f451(v4, v1, v2))
            } else {
                f451(v4, v1, v2)
            }
        }
    }
}

pub fn f452(v0: Adt14) -> Adt15 {
    let m3062 = v0;
    match m3062 {
        Adt14::C0(_, v2, _) => {
            v2
        }
    }
}

pub fn f453(v0: Adt14) -> Bytes {
    let m3063 = v0;
    match m3063 {
        Adt14::C0(v1, _, _) => {
            v1
        }
    }
}

pub fn f454(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f571(v1)
    } else {
        f457(v1)
    }
}

pub fn f455(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3064 = v1;
    match m3064.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3065 = f454(v0, v2)?;
            match m3065 {
                None => {
                    f455(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f455(v0, v3)?))
                }
            }
        }
    }
}

pub fn f456(v0: Adt14) -> u64 {
    let m3066 = v0;
    match m3066 {
        Adt14::C0(_, _, v3) => {
            v3
        }
    }
}

pub fn f457(v0: Bytes) -> R<Option<Bytes>> {
    let m3067 = f572(v0)?;
    match m3067 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            Ok(Some(f573(v1)?))
        }
    }
}

pub fn f458(v0: bool, v1: Adt2) -> R<Option<Bytes>> {
    let m3068 = v1;
    match m3068 {
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
                let a3079 = if f113(v7, {
                    let a3077 = 0u64;
                    let a3078 = v8;
                    nat_lt(a3077, a3078)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a3080 = f282(false, v8)?;
                append_bytes(a3079, a3080)
            }))
        }
        Adt2::C4(v3, v4, v5) => {
            let m3071 = f571({
                let a3069 = f8(Str::lit("n"));
                let a3070 = f283(false, v4, v5)?;
                append_bytes(a3069, a3070)
            })?;
            match m3071 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a3075 = f8(Str::lit("nMinus"));
                        let a3076 = f46(v6.clone(), 1u64, {
                            let a3073 = {
                                let a3072 = v6;
                                length_bytes(a3072)
                            };
                            let a3074 = 1u64;
                            nat_sub(a3073, a3074)
                        });
                        append_bytes(a3075, a3076)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt2::C5(v2) => {
            f454(v0, v2)
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

pub fn f459(v0: Adt2) -> R<Bytes> {
    f193(v0)
}

pub fn f460(v0: u64) -> R<bool> {
    f346(v0, f574())
}

pub fn f461(v0: u64, v1: Bytes, p3091: u64, p3092: u64, p3093: bool) -> R<bool> {
    let m3090 = (p3091, (p3092, p3093));
    {
        let mut m3090 = m3090;
        loop {
            let step = {
                let (v2, (v3, v4)) = m3090;
                let m3081 = v3;
                if m3081 == 0 {
                    Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                } else {
                    let v5 = m3081 - 1;
                    let v6: u64 = f575(v1.clone(), v2)?;
                    if v4 {
                        let a3082 = v0;
                        let a3083 = v6;
                        if nat_lt(a3082, a3083) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a3084 = v2;
                                let a3085 = 1u64;
                                nat_add(a3084, a3085)?
                            }, (v5, false)))
                        }
                    } else {
                        let a3086 = v0;
                        let a3087 = v6;
                        if nat_le(a3086, a3087) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(true))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a3088 = v2;
                                let a3089 = 1u64;
                                nat_add(a3088, a3089)?
                            }, (v5, true)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3090 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f462(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    if f133(v0.clone(), v1, v2) {
        Ok(None::<u64>)
    } else {
        let a3095 = v2;
        let a3096 = {
            let a3094 = v0;
            length_bytes(a3094)
        };
        if nat_lt(a3095, a3096) {
            Ok(Some({
                let a3097 = v2;
                let a3098 = 1u64;
                nat_add(a3097, a3098)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f463(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f230(f160(v0, v1)?)? {
        Ok(Some({
            let a3099 = v1;
            let a3100 = 1u64;
            nat_add(a3099, a3100)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f464(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let m3101 = f404(v0, v2)?;
    match m3101 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            let a3102 = v2;
            let a3103 = v1;
            if if nat_eq(a3102, a3103) {
                f576({
                    let (h3104, _) = v3;
                    h3104
                })?
            } else {
                f577({
                    let (h3105, _) = v3;
                    h3105
                })?
            } {
                Ok(Some({
                    let a3107 = v2;
                    let a3108 = {
                        let (_, h3106) = v3;
                        h3106
                    };
                    nat_add(a3107, a3108)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f465(v0: Bytes, v1: bool, v2: u64) -> R<(u64, u64)> {
    let (h3109, _) = f270(Fn6::F578(v1, v0.clone()), {
        let a3111 = {
            let a3110 = v0;
            length_bytes(a3110)
        };
        let a3112 = 1u64;
        nat_add(a3111, a3112)?
    }, (v2, 0u64))?;
    Ok(h3109)
}

pub fn f466(v0: u64) -> R<Bytes> {
    let a3113 = v0;
    let a3114 = 128u64;
    if nat_lt(a3113, a3114) {
        Ok(f579(f580(), v0))
    } else {
        let a3115 = v0;
        let a3116 = 2048u64;
        if nat_lt(a3115, a3116) {
            let a3124 = f579(f581(), {
                let a3122 = {
                    let a3120 = {
                        let a3117 = v0;
                        let a3118 = 64u64;
                        let a3119 = 0u64;
                        nat_quot(a3117, a3118, a3119)
                    };
                    let a3121 = 2u64;
                    nat_sub(a3120, a3121)
                };
                let a3123 = 2u64;
                nat_mul(a3122, a3123)?
            });
            let a3125 = f582(v0)?;
            Ok(append_bytes(a3124, a3125))
        } else {
            let a3126 = v0;
            let a3127 = 65536u64;
            if nat_lt(a3126, a3127) {
                let a3138 = f579(f583(), {
                    let a3131 = {
                        let a3128 = v0;
                        let a3129 = 4096u64;
                        let a3130 = 0u64;
                        nat_quot(a3128, a3129, a3130)
                    };
                    let a3132 = 3u64;
                    nat_mul(a3131, a3132)?
                });
                let a3139 = {
                    let a3136 = f582({
                        let a3133 = v0;
                        let a3134 = 64u64;
                        let a3135 = 0u64;
                        nat_quot(a3133, a3134, a3135)
                    })?;
                    let a3137 = f582(v0)?;
                    append_bytes(a3136, a3137)
                };
                Ok(append_bytes(a3138, a3139))
            } else {
                let a3153 = f579(Bytes::lit(&[0xf0, 0xf1, 0xf2, 0xf3, 0xf4]), {
                    let a3140 = v0;
                    let a3141 = 262144u64;
                    let a3142 = 0u64;
                    nat_quot(a3140, a3141, a3142)
                });
                let a3154 = {
                    let a3151 = f582({
                        let a3143 = v0;
                        let a3144 = 4096u64;
                        let a3145 = 0u64;
                        nat_quot(a3143, a3144, a3145)
                    })?;
                    let a3152 = {
                        let a3149 = f582({
                            let a3146 = v0;
                            let a3147 = 64u64;
                            let a3148 = 0u64;
                            nat_quot(a3146, a3147, a3148)
                        })?;
                        let a3150 = f582(v0)?;
                        append_bytes(a3149, a3150)
                    };
                    append_bytes(a3151, a3152)
                };
                Ok(append_bytes(a3153, a3154))
            }
        }
    }
}

pub fn f467(v0: bool, v1: u64, v2: Bytes, v3: u64) -> R<Option<u64>> {
    f584(v2, v0, v1, v3)
}

pub fn f468(v0: Bytes) -> Result<Option<Adt31>, Bytes> {
    Err::<Option<Adt31>, Bytes>(v0)
}

pub fn f469(p3158: Bytes, p3159: u64) -> R<Result<Bytes, Bytes>> {
    let m3157 = (p3158, p3159);
    {
        let mut m3157 = m3157;
        loop {
            let step = {
                let (v0, v1) = m3157;
                let m3155 = v1;
                if m3155 == 0 {
                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f585(f8(Str::lit("the built-in datatype hierarchy is deeper than XSD 1.1 Part 2 section 3 defines")))))
                } else {
                    let v2 = m3155 - 1;
                    if f320(v0.clone(), List::cons(f8(Str::lit("NMTOKENS")), List::cons(f8(Str::lit("IDREFS")), List::cons(f8(Str::lit("ENTITIES")), List::<Bytes>::nil())))) {
                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("(List Bytes)")))))
                    } else {
                        if f32(v0.clone(), f8(Str::lit("nonNegativeInteger"))) {
                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Nat")))))
                        } else {
                            if f32(v0.clone(), f8(Str::lit("integer"))) {
                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Int")))))
                            } else {
                                if f32(v0.clone(), f8(Str::lit("decimal"))) {
                                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f586())))
                                } else {
                                    if f32(v0.clone(), f8(Str::lit("boolean"))) {
                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Bool")))))
                                    } else {
                                        if f320(v0.clone(), List::cons(f8(Str::lit("float")), List::cons(f8(Str::lit("double")), List::<Bytes>::nil()))) {
                                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f585(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because its value space holds positive and negative zero, the infinities and NaN, which no exact LexLean number type holds (XSD 1.1 Part 2, sections 3.3.4 and 3.3.5)")), List::<Bytes>::nil())))))))
                                        } else {
                                            if f320(v0.clone(), List::cons(f8(Str::lit("anySimpleType")), List::cons(f8(Str::lit("anyAtomicType")), List::<Bytes>::nil()))) {
                                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f585(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because it is the union of every simple value space and has no one LexLean type (XSD 1.1 Part 2, sections 3.2.1 and 3.2.2)")), List::<Bytes>::nil())))))))
                                            } else {
                                                let m3156 = f587(v0.clone())?;
                                                match m3156 {
                                                    None => {
                                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f585(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not a built-in datatype of XSD 1.1 Part 2")), List::<Bytes>::nil())))))))
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
                Ok(next) => { m3157 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f470(v0: Bytes) -> Result<List<Adt2>, Bytes> {
    Err::<List<Adt2>, Bytes>(v0)
}

pub fn f471(v0: List<Adt26>, v1: List<Adt26>) -> List<Adt26> {
    let m3160 = v0;
    match m3160.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f471(v3, v1))
        }
    }
}

pub fn f472(v0: List<Adt27>, v1: Bytes) -> List<Adt26> {
    let m3161 = v0;
    match m3161.uncons() {
        None => {
            List::<Adt26>::nil()
        }
        Some((v2, v3)) => {
            if f122(match v2.clone() {
                Adt27::C0(_, h3162, _, _, _) => {
                    h3162
                }
            }, f588()) {
                let m3163 = f376(v2.clone(), Str::lit("name"));
                match m3163 {
                    None => {
                        f472(v3, v1)
                    }
                    Some(v4) => {
                        List::cons(Adt26::C0(match v2.clone() {
                            Adt27::C0(_, h3164, _, _, _) => {
                                h3164
                            }
                        }, v1.clone(), v4, v2), f472(v3, v1))
                    }
                }
            } else {
                f472(v3, v1)
            }
        }
    }
}

pub fn f473(v0: u64, v1: Adt25, v2: Adt26) -> R<Result<List<Adt28>, Bytes>> {
    let v3: Bytes = f125(match v2.clone() {
        Adt26::C0(_, _, h3165, _) => {
            h3165
        }
    })?;
    let v4: Adt27 = match v2.clone() {
        Adt26::C0(_, _, _, h3166) => {
            h3166
        }
    };
    let v5: Bytes = match v2.clone() {
        Adt26::C0(_, h3167, _, _) => {
            h3167
        }
    };
    let v6: Bytes = match v2.clone() {
        Adt26::C0(h3168, _, _, _) => {
            h3168
        }
    };
    if f32(v6.clone(), f8(Str::lit("complexType"))) {
        f589(v0, v1, v2)
    } else {
        if f32(v6.clone(), f8(Str::lit("simpleType"))) {
            let m3169 = f590(v3, v4, v5)?;
            match m3169 {
                Ok(v8) => {
                    Ok(Ok::<List<Adt28>, Bytes>(List::cons(v8, List::<Adt28>::nil())))
                }
                Err(v7) => {
                    Ok(Err::<List<Adt28>, Bytes>(v7))
                }
            }
        } else {
            if f32(v6.clone(), f8(Str::lit("element"))) {
                let m3170 = f376(v4.clone(), Str::lit("type"));
                match m3170 {
                    None => {
                        let m3171 = f591(v0, v1, v3, v5, v4)?;
                        match m3171 {
                            Ok(v11) => {
                                Ok(Ok::<List<Adt28>, Bytes>(match v11 {
                                    Adt35::C0(_, h3172) => {
                                        h3172
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
                    let m3173 = f376(v4.clone(), Str::lit("type"));
                    match m3173 {
                        None => {
                            let m3174 = f592(v1, v5, v3, v4)?;
                            match m3174 {
                                Ok(v14) => {
                                    Ok(Ok::<List<Adt28>, Bytes>(match v14 {
                                        Adt35::C0(_, h3175) => {
                                            h3175
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
                        let m3176 = f593(v4)?;
                        match m3176 {
                            Ok(v16) => {
                                let m3177 = f594(v16, List::cons(f8(Str::lit("sequence")), List::cons(f8(Str::lit("choice")), List::cons(f8(Str::lit("all")), List::<Bytes>::nil()))));
                                match m3177 {
                                    None => {
                                        Ok(Err::<List<Adt28>, Bytes>(f595(f8(Str::lit("a model group definition must have a sequence, choice or all child (XSD 1.1 Part 1, section 3.7.2)")), v3)))
                                    }
                                    Some(v17) => {
                                        f596(v0, v1, v3, v5, v17)
                                    }
                                }
                            }
                            Err(v15) => {
                                Ok(Err::<List<Adt28>, Bytes>(v15))
                            }
                        }
                    } else {
                        let m3178 = f593(v4)?;
                        match m3178 {
                            Ok(v19) => {
                                let m3179 = f597(v0, v1, v3, v5, v19, f598())?;
                                match m3179 {
                                    Ok(v21) => {
                                        Ok(Ok::<List<Adt28>, Bytes>(match v21 {
                                            Adt36::C0(_, h3180, _, _, _) => {
                                                h3180
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

pub fn f474(v0: List<Adt28>, v1: List<Adt28>) -> List<Adt28> {
    let m3181 = v0;
    match m3181.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f474(v3, v1))
        }
    }
}

pub fn f475(p3184: List<Adt28>, p3185: List<Adt28>) -> List<Adt28> {
    let m3183 = (p3184, p3185);
    {
        let mut m3183 = m3183;
        loop {
            let step = {
                let (v0, v1) = m3183;
                let m3182 = v0;
                match m3182.uncons() {
                    None => {
                        Err::<(List<Adt28>, List<Adt28>), List<Adt28>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt28>, List<Adt28>), List<Adt28>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3183 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f476(p3189: List<Adt28>, v1: Bytes) -> Option<Adt28> {
    let m3188 = p3189;
    {
        let mut m3188 = m3188;
        loop {
            let step = {
                let v0 = m3188;
                let m3186 = v0;
                match m3186.uncons() {
                    None => {
                        Err::<List<Adt28>, Option<Adt28>>(None::<Adt28>)
                    }
                    Some((v2, v3)) => {
                        if f32(match v2.clone() {
                            Adt28::C0(h3187, _, _, _) => {
                                h3187
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
                Ok(next) => { m3188 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f477() -> Bytes {
    f8(Str::lit("XmlNode"))
}

pub fn f478(v0: List<Adt28>) -> List<Bytes> {
    let m3190 = v0;
    match m3190.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f484(match v1 {
                Adt28::C0(_, h3191, _, _) => {
                    h3191
                }
            }, f478(v2))
        }
    }
}

pub fn f479() -> List<Bytes> {
    List::cons(f8(Str::lit("    {name: element, fields: [Bytes, Bytes, (List ((Bytes * Bytes) * Bytes)), (List XmlNode)]}")), List::cons(f8(Str::lit("    {name: text, fields: [Bytes]}")), List::<Bytes>::nil()))
}

pub fn f480(p3195: u64, v1: List<Adt28>, p3196: List<Bytes>, p3197: List<Bytes>) -> List<Bytes> {
    let m3194 = (p3195, (p3196, p3197));
    {
        let mut m3194 = m3194;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3194;
                let m3192 = v0;
                if m3192 == 0 {
                    Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v3)
                } else {
                    let v4 = m3192 - 1;
                    let m3193 = v2;
                    match m3193.uncons() {
                        None => {
                            Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v3)
                        }
                        Some((v5, v6)) => {
                            if f122(v5.clone(), v3.clone()) {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v3)))
                            } else {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (f484(f481(v1.clone(), v5.clone()), v6), List::cons(v5, v3))))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3194 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f481(v0: List<Adt28>, v1: Bytes) -> List<Bytes> {
    let m3198 = f476(v0, v1);
    match m3198 {
        None => {
            List::<Bytes>::nil()
        }
        Some(v2) => {
            match v2 {
                Adt28::C0(_, h3199, _, _) => {
                    h3199
                }
            }
        }
    }
}

pub fn f482(v0: List<(List<Bytes>, List<Bytes>)>, v1: List<Bytes>) -> Option<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)> {
    let m3200 = v0;
    match m3200.uncons() {
        None => {
            None::<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)>
        }
        Some((v2, v3)) => {
            let m3202 = f250({
                let (_, h3201) = v2.clone();
                h3201
            }, v1.clone());
            match m3202 {
                None => {
                    Some(({
                        let (h3203, _) = v2;
                        h3203
                    }, v3))
                }
                Some(_) => {
                    let m3204 = f482(v3, v1);
                    match m3204 {
                        None => {
                            None::<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h3205, _) = v5;
                                h3205
                            }, List::cons(v2, {
                                let (_, h3206) = v5;
                                h3206
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f483(v0: List<Adt28>, v1: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m3207 = v1.clone();
    match m3207.uncons() {
        None => {
            Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3208 = v3;
            match m3208.uncons() {
                None => {
                    let m3211 = f476(v0, v2.clone());
                    match m3211 {
                        None => {
                            Err::<List<Bytes>, Bytes>(f600(v2))
                        }
                        Some(v6) => {
                            if f122(v2.clone(), match v6.clone() {
                                Adt28::C0(_, h3212, _, _) => {
                                    h3212
                                }
                            }) {
                                let m3214 = match v6.clone() {
                                    Adt28::C0(_, _, _, h3213) => {
                                        h3213
                                    }
                                };
                                match m3214.uncons() {
                                    None => {
                                        Err::<List<Bytes>, Bytes>(f600(v2))
                                    }
                                    Some((_, _)) => {
                                        Ok::<List<Bytes>, Bytes>(List::cons(f338(v2, match v6 {
                                            Adt28::C0(_, _, _, h3215) => {
                                                h3215
                                            }
                                        }), List::<Bytes>::nil()))
                                    }
                                }
                            } else {
                                Ok::<List<Bytes>, Bytes>(List::cons(match v6 {
                                    Adt28::C0(_, _, h3216, _) => {
                                        h3216
                                    }
                                }, List::<Bytes>::nil()))
                            }
                        }
                    }
                }
                Some((_, _)) => {
                    f599(v0, v1, {
                        let a3209 = v2;
                        let a3210 = f8(Str::lit("Group"));
                        append_bytes(a3209, a3210)
                    })
                }
            }
        }
    }
}

pub fn f484(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m3217 = v0;
    match m3217.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f484(v3, v1))
        }
    }
}

pub fn f485(v0: List<Adt28>, v1: List<Bytes>, v2: List<Bytes>) -> List<Bytes> {
    let m3218 = v1;
    match m3218.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            f484(f601(f481(v0.clone(), v3), v2.clone()), f485(v0, v4, v2))
        }
    }
}

pub fn f486(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>, v2: Bytes) -> List<Bytes> {
    let m3219 = v1;
    match m3219.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if f32(v3.clone(), v2.clone()) {
                true
            } else {
                if f122(v3.clone(), f602(v0.clone(), v2.clone())) {
                    f122(v2.clone(), f602(v0.clone(), v3.clone()))
                } else {
                    false
                }
            } {
                List::cons(v3, f486(v0, v4, v2))
            } else {
                f486(v0, v4, v2)
            }
        }
    }
}

pub fn f487(v0: Bytes, p3228: u64, p3229: u64, p3230: Option<u64>) -> R<Option<u64>> {
    let m3227 = (p3228, (p3229, p3230));
    {
        let mut m3227 = m3227;
        loop {
            let step = {
                let (v1, (v2, v3)) = m3227;
                let m3220 = v2;
                if m3220 == 0 {
                    Err::<(u64, (u64, Option<u64>)), R<Option<u64>>>(Ok(v3))
                } else {
                    let v4 = m3220 - 1;
                    let v5: u64 = f160(v0.clone(), v1)?;
                    let a3221 = v5;
                    let a3222 = 256u64;
                    if nat_eq(a3221, a3222) {
                        Err::<(u64, (u64, Option<u64>)), R<Option<u64>>>(Ok(v3))
                    } else {
                        Ok::<(u64, (u64, Option<u64>)), R<Option<u64>>>(({
                            let a3223 = v1;
                            let a3224 = 1u64;
                            nat_add(a3223, a3224)?
                        }, (v4, {
                            let a3225 = v5;
                            let a3226 = 47u64;
                            if nat_eq(a3225, a3226) {
                                Some(v1)
                            } else {
                                v3
                            }
                        })))
                    }
                }
            };
            match step {
                Ok(next) => { m3227 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f488(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a3231 = v0.clone();
        length_bytes(a3231)
    };
    let a3235 = {
        let a3232 = v1;
        let a3233 = 2u64;
        let a3234 = 0u64;
        nat_rem(a3232, a3233, a3234)
    };
    let a3236 = 0u64;
    if nat_eq(a3235, a3236) {
        let (_, h3237) = {
            let (h3238, _) = f603(Fn14::F604(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h3238
        };
        Ok(h3237)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f489(v0: u64) -> bool {
    let a3239 = v0;
    let a3240 = 8u64;
    if nat_eq(a3239, a3240) {
        true
    } else {
        let a3241 = v0;
        let a3242 = 9u64;
        if nat_eq(a3241, a3242) {
            true
        } else {
            let a3243 = v0;
            let a3244 = 10u64;
            if nat_eq(a3243, a3244) {
                true
            } else {
                let a3245 = v0;
                let a3246 = 12u64;
                if nat_eq(a3245, a3246) {
                    true
                } else {
                    let a3247 = v0;
                    let a3248 = 13u64;
                    nat_eq(a3247, a3248)
                }
            }
        }
    }
}

pub fn f490(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3251 = {
        let a3249 = v0;
        let a3250 = v1;
        index_bytes(a3249, a3250)
    };
    match m3251 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f605(v2)? {
                Ok(Some({
                    let a3252 = v1;
                    let a3253 = 1u64;
                    nat_add(a3252, a3253)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f491(v0: u8) -> R<u64> {
    let a3300 = {
        let v1: u8 = v0;
        let a3298 = {
            let a3274 = {
                let a3262 = {
                    let a3256 = {
                        let a3254 = v1;
                        let a3255 = 128u8;
                        fixed_u8::bit_and(a3254, a3255)
                    };
                    let a3257 = 128u8;
                    if equal(a3256, a3257) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3263 = {
                    let a3260 = {
                        let a3258 = v1;
                        let a3259 = 64u8;
                        fixed_u8::bit_and(a3258, a3259)
                    };
                    let a3261 = 64u8;
                    if equal(a3260, a3261) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3262, a3263)?
            };
            let a3275 = {
                let a3272 = {
                    let a3266 = {
                        let a3264 = v1;
                        let a3265 = 32u8;
                        fixed_u8::bit_and(a3264, a3265)
                    };
                    let a3267 = 32u8;
                    if equal(a3266, a3267) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3273 = {
                    let a3270 = {
                        let a3268 = v1;
                        let a3269 = 16u8;
                        fixed_u8::bit_and(a3268, a3269)
                    };
                    let a3271 = 16u8;
                    if equal(a3270, a3271) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3272, a3273)?
            };
            nat_add(a3274, a3275)?
        };
        let a3299 = {
            let a3296 = {
                let a3284 = {
                    let a3278 = {
                        let a3276 = v1;
                        let a3277 = 8u8;
                        fixed_u8::bit_and(a3276, a3277)
                    };
                    let a3279 = 8u8;
                    if equal(a3278, a3279) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3285 = {
                    let a3282 = {
                        let a3280 = v1;
                        let a3281 = 4u8;
                        fixed_u8::bit_and(a3280, a3281)
                    };
                    let a3283 = 4u8;
                    if equal(a3282, a3283) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3284, a3285)?
            };
            let a3297 = {
                let a3294 = {
                    let a3288 = {
                        let a3286 = v1;
                        let a3287 = 2u8;
                        fixed_u8::bit_and(a3286, a3287)
                    };
                    let a3289 = 2u8;
                    if equal(a3288, a3289) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3295 = {
                    let a3292 = {
                        let a3290 = v1;
                        let a3291 = 1u8;
                        fixed_u8::bit_and(a3290, a3291)
                    };
                    let a3293 = 1u8;
                    if equal(a3292, a3293) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3294, a3295)?
            };
            nat_add(a3296, a3297)?
        };
        nat_add(a3298, a3299)?
    };
    let a3301 = 48u64;
    Ok(nat_sub(a3300, a3301))
}

pub fn f492(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a3302 = v0.clone();
        length_bytes(a3302)
    };
    let a3303 = v1;
    let a3304 = v2;
    if if nat_lt(a3303, a3304) {
        let a3309 = f111(v0, {
            let a3307 = v2;
            let a3308 = {
                let a3305 = v1;
                let a3306 = 1u64;
                nat_add(a3305, a3306)?
            };
            nat_sub(a3307, a3308)
        })?;
        let a3310 = 48u64;
        nat_eq(a3309, a3310)
    } else {
        false
    } {
        Ok(Some({
            let a3311 = v1;
            let a3312 = 1u64;
            nat_add(a3311, a3312)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f493(v0: Adt30) -> R<Adt30> {
    let v1: u32 = f606(match v0.clone() {
        Adt30::C0(_, _, _, _, _, _, _, _, _, h3313) => {
            h3313
        }
    });
    let v2: u32 = f389(match v0.clone() {
        Adt30::C0(_, _, _, _, _, _, _, h3314, _, _) => {
            h3314
        }
    }, f389(f607(match v0.clone() {
        Adt30::C0(_, _, _, _, h3315, _, _, _, _, _) => {
            h3315
        }
    }), f389(f608(match v0.clone() {
        Adt30::C0(_, _, _, _, h3316, _, _, _, _, _) => {
            h3316
        }
    }, match v0.clone() {
        Adt30::C0(_, _, _, _, _, h3317, _, _, _, _) => {
            h3317
        }
    }, match v0.clone() {
        Adt30::C0(_, _, _, _, _, _, h3318, _, _, _) => {
            h3318
        }
    }), f389(v1, {
        let m3321 = match v0.clone() {
            Adt30::C0(_, _, _, _, _, _, _, _, h3320, _) => {
                h3320
            }
        };
        match m3321 {
            Adt29::C0(h3319, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3319
            }
        }
    }))));
    let v3: u32 = f389(f609(match v0.clone() {
        Adt30::C0(h3322, _, _, _, _, _, _, _, _, _) => {
            h3322
        }
    }), f610(match v0.clone() {
        Adt30::C0(h3323, _, _, _, _, _, _, _, _, _) => {
            h3323
        }
    }, match v0.clone() {
        Adt30::C0(_, h3324, _, _, _, _, _, _, _, _) => {
            h3324
        }
    }, match v0.clone() {
        Adt30::C0(_, _, h3325, _, _, _, _, _, _, _) => {
            h3325
        }
    }));
    Ok(Adt30::C0(f389(v2, v3), match v0.clone() {
        Adt30::C0(h3326, _, _, _, _, _, _, _, _, _) => {
            h3326
        }
    }, match v0.clone() {
        Adt30::C0(_, h3327, _, _, _, _, _, _, _, _) => {
            h3327
        }
    }, match v0.clone() {
        Adt30::C0(_, _, h3328, _, _, _, _, _, _, _) => {
            h3328
        }
    }, f389(match v0.clone() {
        Adt30::C0(_, _, _, h3329, _, _, _, _, _, _) => {
            h3329
        }
    }, v2), match v0.clone() {
        Adt30::C0(_, _, _, _, h3330, _, _, _, _, _) => {
            h3330
        }
    }, match v0.clone() {
        Adt30::C0(_, _, _, _, _, h3331, _, _, _, _) => {
            h3331
        }
    }, match v0.clone() {
        Adt30::C0(_, _, _, _, _, _, h3332, _, _, _) => {
            h3332
        }
    }, f611(match v0.clone() {
        Adt30::C0(_, _, _, _, _, _, _, _, h3333, _) => {
            h3333
        }
    }), {
        let a3335 = match v0 {
            Adt30::C0(_, _, _, _, _, _, _, _, _, h3334) => {
                h3334
            }
        };
        let a3336 = 1u64;
        nat_add(a3335, a3336)?
    }))
}

pub fn f494(v0: u32) -> u64 {
    let m3338 = {
        let a3337 = v0;
        fixed_u64::convert(i128::from(a3337))
    };
    match m3338 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f495(v0: u64) -> u32 {
    let m3342 = {
        let a3341 = {
            let a3339 = v0;
            let a3340 = 4294967295u64;
            fixed_u64::bit_and(a3339, a3340)
        };
        fixed_u32::convert(i128::from(a3341))
    };
    match m3342 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f496(v0: u64) -> u32 {
    let (_, h3343) = {
        let (_, h3344) = f612(Fn15::F613, 32u64, (v0, (1u32, 0u32)));
        h3344
    };
    h3343
}

pub fn f497(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a3349 = f614(v0, 24u32);
    let a3350 = {
        let a3347 = f614(v1, 16u32);
        let a3348 = {
            let a3345 = f614(v2, 8u32);
            let a3346 = v3;
            fixed_u32::bit_or(a3345, a3346)
        };
        fixed_u32::bit_or(a3347, a3348)
    };
    fixed_u32::bit_or(a3349, a3350)
}

pub fn f498(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a3351 = v2;
    let a3352 = v1;
    if nat_lt(a3351, a3352) {
        let m3355 = {
            let a3353 = v0;
            let a3354 = v2;
            index_bytes(a3353, a3354)
        };
        match m3355 {
            None => {
                0u32
            }
            Some(v3) => {
                let m3357 = {
                    let a3356 = v3;
                    fixed_u32::convert(i128::from(a3356))
                };
                match m3357 {
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
        let a3358 = v2;
        let a3359 = v1;
        if nat_eq(a3358, a3359) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f499(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a3404 = {
            let a3380 = {
                let a3368 = {
                    let a3362 = {
                        let a3360 = v3;
                        let a3361 = 128u8;
                        fixed_u8::bit_and(a3360, a3361)
                    };
                    let a3363 = 128u8;
                    if equal(a3362, a3363) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3369 = {
                    let a3366 = {
                        let a3364 = v3;
                        let a3365 = 64u8;
                        fixed_u8::bit_and(a3364, a3365)
                    };
                    let a3367 = 64u8;
                    if equal(a3366, a3367) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3368, a3369)?
            };
            let a3381 = {
                let a3378 = {
                    let a3372 = {
                        let a3370 = v3;
                        let a3371 = 32u8;
                        fixed_u8::bit_and(a3370, a3371)
                    };
                    let a3373 = 32u8;
                    if equal(a3372, a3373) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3379 = {
                    let a3376 = {
                        let a3374 = v3;
                        let a3375 = 16u8;
                        fixed_u8::bit_and(a3374, a3375)
                    };
                    let a3377 = 16u8;
                    if equal(a3376, a3377) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3378, a3379)?
            };
            nat_add(a3380, a3381)?
        };
        let a3405 = {
            let a3402 = {
                let a3390 = {
                    let a3384 = {
                        let a3382 = v3;
                        let a3383 = 8u8;
                        fixed_u8::bit_and(a3382, a3383)
                    };
                    let a3385 = 8u8;
                    if equal(a3384, a3385) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3391 = {
                    let a3388 = {
                        let a3386 = v3;
                        let a3387 = 4u8;
                        fixed_u8::bit_and(a3386, a3387)
                    };
                    let a3389 = 4u8;
                    if equal(a3388, a3389) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3390, a3391)?
            };
            let a3403 = {
                let a3400 = {
                    let a3394 = {
                        let a3392 = v3;
                        let a3393 = 2u8;
                        fixed_u8::bit_and(a3392, a3393)
                    };
                    let a3395 = 2u8;
                    if equal(a3394, a3395) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3401 = {
                    let a3398 = {
                        let a3396 = v3;
                        let a3397 = 1u8;
                        fixed_u8::bit_and(a3396, a3397)
                    };
                    let a3399 = 1u8;
                    if equal(a3398, a3399) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3400, a3401)?
            };
            nat_add(a3402, a3403)?
        };
        nat_add(a3404, a3405)?
    };
    let a3406 = v4;
    let a3407 = 34u64;
    if nat_eq(a3406, a3407) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a3408 = v4;
        let a3409 = 92u64;
        if nat_eq(a3408, a3409) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a3410 = v4;
            let a3411 = 8u64;
            if nat_eq(a3410, a3411) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a3412 = v4;
                let a3413 = 12u64;
                if nat_eq(a3412, a3413) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a3414 = v4;
                    let a3415 = 10u64;
                    if nat_eq(a3414, a3415) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a3416 = v4;
                        let a3417 = 13u64;
                        if nat_eq(a3416, a3417) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a3418 = v4;
                            let a3419 = 9u64;
                            if nat_eq(a3418, a3419) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a3420 = v4;
                                let a3421 = 32u64;
                                if nat_lt(a3420, a3421) {
                                    let a3422 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a3423 = f615(v2)?;
                                    Ok(append_bytes(a3422, a3423))
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

pub fn f500(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m3424 = v0.clone();
    match m3424.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a3427 = {
                let (h3425, _) = v2;
                h3425
            };
            let a3428 = {
                let (h3426, _) = v1.clone();
                h3426
            };
            if equal(a3427, a3428) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f501(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f616(f617(Fn16::F618, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m3433 = {
        let (h3429, _) = f619(Fn17::F620, {
            let a3431 = {
                let a3430 = v0;
                length_list(a3430)
            };
            let a3432 = 1u64;
            nat_add(a3431, a3432)?
        }, v1)?;
        h3429
    };
    match m3433.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f502(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a3438 = {
        let a3435 = f284({
            let (h3434, _) = v0;
            h3434
        })?;
        let a3436 = Bytes::lit(&[0x3a]);
        append_bytes(a3435, a3436)
    };
    let a3439 = {
        let (_, h3437) = v0;
        h3437
    };
    Ok(append_bytes(a3438, a3439))
}

pub fn f503(v0: Bytes, p3465: u64, p3466: u64, p3467: Bytes) -> R<Bytes> {
    let m3464 = (p3465, (p3466, p3467));
    {
        let mut m3464 = m3464;
        loop {
            let step = {
                let (v1, (v2, v3)) = m3464;
                let m3440 = v2;
                if m3440 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m3440 - 1;
                    let m3443 = {
                        let a3441 = v0.clone();
                        let a3442 = v1;
                        index_bytes(a3441, a3442)
                    };
                    match m3443 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v6: u64 = f62(v5)?;
                            if f180({
                                let a3444 = v6;
                                let a3445 = 34u64;
                                nat_eq(a3444, a3445)
                            }, {
                                let a3446 = v6;
                                let a3447 = 92u64;
                                nat_eq(a3446, a3447)
                            }) {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a3448 = v1;
                                    let a3449 = 1u64;
                                    nat_add(a3448, a3449)?
                                }, (v4, {
                                    let a3452 = v3;
                                    let a3453 = {
                                        let a3450 = f8(Str::lit("\\"));
                                        let a3451 = f46(v0.clone(), v1, 1u64);
                                        append_bytes(a3450, a3451)
                                    };
                                    append_bytes(a3452, a3453)
                                })))
                            } else {
                                let a3454 = v6;
                                let a3455 = 10u64;
                                if nat_eq(a3454, a3455) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a3456 = v1;
                                        let a3457 = 1u64;
                                        nat_add(a3456, a3457)?
                                    }, (v4, {
                                        let a3458 = v3;
                                        let a3459 = f8(Str::lit("\\n"));
                                        append_bytes(a3458, a3459)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a3460 = v1;
                                        let a3461 = 1u64;
                                        nat_add(a3460, a3461)?
                                    }, (v4, {
                                        let a3462 = v3;
                                        let a3463 = f46(v0.clone(), v1, 1u64);
                                        append_bytes(a3462, a3463)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3464 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f504(v0: Bytes) -> Bytes {
    if f41(v0.clone(), Bytes::lit(&[0x0d])) {
        f46(v0.clone(), 0u64, {
            let a3469 = {
                let a3468 = v0;
                length_bytes(a3468)
            };
            let a3470 = 1u64;
            nat_sub(a3469, a3470)
        })
    } else {
        v0
    }
}

pub fn f505(v0: Fn10, p3474: List<Bytes>, p3475: List<(Bytes, ())>) -> List<Bytes> {
    let m3473 = (p3474, p3475);
    {
        let mut m3473 = m3473;
        loop {
            let step = {
                let (v1, v2) = m3473;
                let m3471 = v2;
                match m3471.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c3472: Fn10 = v0.clone();
                            c3472.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3473 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f506(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h3476, _) = v1;
        h3476
    }, v0)
}

pub fn f507(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f621(Fn18::F622, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f508(v0: Fn11, p3480: List<List<(Bytes, ())>>, p3481: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m3479 = (p3480, p3481);
    {
        let mut m3479 = m3479;
        loop {
            let step = {
                let (v1, v2) = m3479;
                let m3477 = v2;
                match m3477.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c3478: Fn11 = v0.clone();
                            c3478.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3479 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f509(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f510(v0: Fn12, p3486: u64, p3487: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m3485 = (p3486, p3487);
    {
        let mut m3485 = m3485;
        loop {
            let step = {
                let (v1, v2) = m3485;
                let m3482 = v1;
                if m3482 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3482 - 1;
                    let m3484 = {
                        let c3483: Fn12 = v0.clone();
                        c3483.apply(v2.clone())?
                    };
                    match m3484 {
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
                Ok(next) => { m3485 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f511(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f623(v0)
}

pub fn f512(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f624(Fn19::F625, List::<(Bytes, ())>::nil(), v0)
}

pub fn f513(v0: Fn13, p3491: List<(Bytes, ())>, p3492: List<Bytes>) -> List<(Bytes, ())> {
    let m3490 = (p3491, p3492);
    {
        let mut m3490 = m3490;
        loop {
            let step = {
                let (v1, v2) = m3490;
                let m3488 = v2;
                match m3488.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c3489: Fn13 = v0.clone();
                            c3489.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3490 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f514(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f515(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3495 = {
        let a3493 = v0;
        let a3494 = v1;
        index_bytes(a3493, a3494)
    };
    match m3495 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f62(v2)?;
            if f113({
                let a3496 = 128u64;
                let a3497 = v3;
                nat_le(a3496, a3497)
            }, {
                let a3498 = v3;
                let a3499 = 191u64;
                nat_le(a3498, a3499)
            }) {
                Ok(Some({
                    let a3500 = v3;
                    let a3501 = 128u64;
                    nat_sub(a3500, a3501)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f516(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f113({
        let a3502 = v1;
        let a3503 = v0;
        nat_le(a3502, a3503)
    }, f113({
        let a3504 = v0;
        let a3505 = v2;
        nat_le(a3504, a3505)
    }, f180({
        let a3506 = v0;
        let a3507 = 55296u64;
        nat_lt(a3506, a3507)
    }, {
        let a3508 = 57343u64;
        let a3509 = v0;
        nat_lt(a3508, a3509)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f517(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f626(v0, v1)
}

pub fn f518(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f299(f206(v0, v1)?) {
        Ok(Some({
            let a3510 = v1;
            let a3511 = 1u64;
            nat_add(a3510, a3511)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f519(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f627(v0, v1)
}

pub fn f520(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a3512 = f206(v0, v1)?;
    let a3513 = 32u64;
    if nat_eq(a3512, a3513) {
        Ok(Some({
            let a3514 = v1;
            let a3515 = 1u64;
            nat_add(a3514, a3515)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f521(v0: u64) -> bool {
    let a3516 = v0;
    let a3517 = 44u64;
    if if if nat_eq(a3516, a3517) {
        true
    } else {
        let a3518 = v0;
        let a3519 = 91u64;
        nat_eq(a3518, a3519)
    } {
        true
    } else {
        let a3520 = v0;
        let a3521 = 93u64;
        if nat_eq(a3520, a3521) {
            true
        } else {
            let a3522 = v0;
            let a3523 = 123u64;
            nat_eq(a3522, a3523)
        }
    } {
        true
    } else {
        let a3524 = v0;
        let a3525 = 125u64;
        nat_eq(a3524, a3525)
    }
}

pub fn f522(v0: u64) -> u64 {
    let a3526 = v0;
    let a3527 = 45u64;
    if nat_eq(a3526, a3527) {
        1u64
    } else {
        let a3528 = v0;
        let a3529 = 43u64;
        if nat_eq(a3528, a3529) {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f523(v0: u64) -> u64 {
    let a3530 = 49u64;
    let a3531 = v0;
    if if nat_le(a3530, a3531) {
        let a3532 = v0;
        let a3533 = 57u64;
        nat_le(a3532, a3533)
    } else {
        false
    } {
        let a3534 = v0;
        let a3535 = 48u64;
        nat_sub(a3534, a3535)
    } else {
        0u64
    }
}

pub fn f524(v0: Bytes, v1: u64, v2: u64, v3: bool, v4: u64) -> R<Adt22> {
    let m3536 = f628(v0.clone(), v1, v2)?;
    match m3536 {
        Adt38::C0(v6, v7) => {
            let v8: u64 = v7;
            let v9: u64 = f629(v0.clone(), v8, v2)?;
            if f630(v0.clone(), v8, v2)? {
                let m3540 = f631(v0.clone(), v8, v2, v3, {
                    let a3538 = {
                        let a3537 = v0;
                        length_bytes(a3537)
                    };
                    let a3539 = 2u64;
                    nat_add(a3538, a3539)?
                }, Bytes::lit(&[]), v6)?;
                match m3540 {
                    Adt39::C0(v11, v12, v13) => {
                        Ok(Adt22::C0(Adt2::C5(f7(List::cons(v11, List::cons({
                            let a3541 = v4;
                            let a3542 = 1u64;
                            if nat_eq(a3541, a3542) {
                                Bytes::lit(&[])
                            } else {
                                Bytes::lit(&[0x0a])
                            }
                        }, List::cons({
                            let a3543 = v4;
                            let a3544 = 2u64;
                            if nat_eq(a3543, a3544) {
                                f526(v12)
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
                    let a3545 = v4;
                    let a3546 = 2u64;
                    if nat_eq(a3545, a3546) {
                        f526(v6)
                    } else {
                        Bytes::lit(&[])
                    }
                }), {
                    let a3547 = f206(v0, v9)?;
                    let a3548 = 256u64;
                    if nat_eq(a3547, a3548) {
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

pub fn f525(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f632(v0, v1)
}

pub fn f526(v0: u64) -> Bytes {
    let m3549 = v0;
    if m3549 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m3549 - 1;
        let a3550 = Bytes::lit(&[0x0a]);
        let a3551 = f526(v1);
        append_bytes(a3550, a3551)
    }
}

pub fn f527(v0: Bytes, v1: u64) -> R<u64> {
    let (_, h3552) = {
        let (h3553, _) = f270(Fn6::F633(v0.clone()), {
            let a3555 = {
                let a3554 = v0;
                length_bytes(a3554)
            };
            let a3556 = 1u64;
            nat_add(a3555, a3556)?
        }, (v1, 0u64))?;
        h3553
    };
    Ok(h3552)
}

pub fn f528(v0: u64, v1: u64) -> u64 {
    let a3557 = v0;
    let a3558 = v1;
    if nat_lt(a3557, a3558) {
        v1
    } else {
        v0
    }
}

pub fn f529(v0: Bytes, v1: u64, v2: u64) -> R<Adt22> {
    if f530(v0.clone()) {
        Ok(Adt22::C0(Adt2::C0, v2))
    } else {
        if f534(v0.clone()) {
            Ok(Adt22::C0(Adt2::C2(true), v2))
        } else {
            if f535(v0.clone()) {
                Ok(Adt22::C0(Adt2::C2(false), v2))
            } else {
                if f533(v0.clone()) {
                    Ok(Adt22::C1(f137(Str::lit("an infinite or not-a-number float is not read: Studio.Value has none (YAML 1.2.2 section 10.3.2)"), v1)?))
                } else {
                    if if f159(v0.clone(), 0u64, Str::lit("0o")) {
                        let a3560 = 2u64;
                        let a3561 = {
                            let a3559 = v0.clone();
                            length_bytes(a3559)
                        };
                        nat_lt(a3560, a3561)
                    } else {
                        false
                    } {
                        f634(v0, 8u64, v1, v2)
                    } else {
                        if if f159(v0.clone(), 0u64, Str::lit("0x")) {
                            let a3563 = 2u64;
                            let a3564 = {
                                let a3562 = v0.clone();
                                length_bytes(a3562)
                            };
                            nat_lt(a3563, a3564)
                        } else {
                            false
                        } {
                            f634(v0, 16u64, v1, v2)
                        } else {
                            let m3565 = f532(v0.clone())?;
                            match m3565 {
                                None => {
                                    Ok(Adt22::C0(Adt2::C5(v0), v2))
                                }
                                Some(v3) => {
                                    let m3566 = f114(v3.clone(), 0u64)?;
                                    match m3566 {
                                        None => {
                                            Ok(Adt22::C1(f635(v1)?))
                                        }
                                        Some(v4) => {
                                            let a3569 = {
                                                let (h3567, _) = v4;
                                                h3567
                                            };
                                            let a3570 = {
                                                let a3568 = v3;
                                                length_bytes(a3568)
                                            };
                                            if nat_eq(a3569, a3570) {
                                                Ok(Adt22::C0({
                                                    let (_, h3571) = v4;
                                                    h3571
                                                }, v2))
                                            } else {
                                                Ok(Adt22::C1(f635(v1)?))
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

pub fn f530(v0: Bytes) -> bool {
    let a3573 = {
        let a3572 = v0.clone();
        length_bytes(a3572)
    };
    let a3574 = 0u64;
    if nat_eq(a3573, a3574) {
        true
    } else {
        f320(v0, List::cons(f8(Str::lit("~")), List::cons(f8(Str::lit("null")), List::cons(f8(Str::lit("Null")), List::cons(f8(Str::lit("NULL")), List::<Bytes>::nil())))))
    }
}

pub fn f531(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a3575 = v0.clone();
        length_bytes(a3575)
    };
    let v2: u64 = f206(v0.clone(), 0u64)?;
    let v3: u64 = {
        let a3576 = v2;
        let a3577 = 43u64;
        if if nat_eq(a3576, a3577) {
            true
        } else {
            let a3578 = v2;
            let a3579 = 45u64;
            nat_eq(a3578, a3579)
        } {
            1u64
        } else {
            0u64
        }
    };
    let a3580 = v3;
    let a3581 = v1;
    if if nat_lt(a3580, a3581) {
        let a3582 = f179(v0.clone(), v3)?;
        let a3583 = v1;
        nat_eq(a3582, a3583)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a3584 = 2u64;
        let a3585 = v1;
        if nat_lt(a3584, a3585) {
            if if f159(v0.clone(), 0u64, Str::lit("0o")) {
                let a3587 = {
                    let (h3586, _) = f636(v0.clone(), 8u64, 2u64)?;
                    h3586
                };
                let a3588 = v1;
                nat_eq(a3587, a3588)
            } else {
                false
            } {
                Ok(true)
            } else {
                if f159(v0.clone(), 0u64, Str::lit("0x")) {
                    let a3590 = {
                        let (h3589, _) = f636(v0, 16u64, 2u64)?;
                        h3589
                    };
                    let a3591 = v1;
                    Ok(nat_eq(a3590, a3591))
                } else {
                    Ok(false)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f532(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a3592 = v0.clone();
        length_bytes(a3592)
    };
    let v2: u64 = f206(v0.clone(), 0u64)?;
    let v3: u64 = {
        let a3593 = v2;
        let a3594 = 43u64;
        if if nat_eq(a3593, a3594) {
            true
        } else {
            let a3595 = v2;
            let a3596 = 45u64;
            nat_eq(a3595, a3596)
        } {
            1u64
        } else {
            0u64
        }
    };
    let v4: u64 = f179(v0.clone(), v3)?;
    let v5: bool = {
        let a3597 = f206(v0.clone(), v4)?;
        let a3598 = 46u64;
        nat_eq(a3597, a3598)
    };
    let v6: u64 = if v5 {
        let a3599 = v4;
        let a3600 = 1u64;
        nat_add(a3599, a3600)?
    } else {
        v4
    };
    let v7: u64 = if v5 {
        f179(v0.clone(), v6)?
    } else {
        v4
    };
    let v8: u64 = f206(v0.clone(), v7)?;
    let v9: bool = {
        let a3601 = v8;
        let a3602 = 101u64;
        if nat_eq(a3601, a3602) {
            true
        } else {
            let a3603 = v8;
            let a3604 = 69u64;
            nat_eq(a3603, a3604)
        }
    };
    let v10: u64 = f206(v0.clone(), {
        let a3605 = v7;
        let a3606 = 1u64;
        nat_add(a3605, a3606)?
    })?;
    let v11: bool = if v9 {
        let a3607 = v10;
        let a3608 = 43u64;
        if nat_eq(a3607, a3608) {
            true
        } else {
            let a3609 = v10;
            let a3610 = 45u64;
            nat_eq(a3609, a3610)
        }
    } else {
        false
    };
    let v12: u64 = if v9 {
        if v11 {
            let a3611 = v7;
            let a3612 = 2u64;
            nat_add(a3611, a3612)?
        } else {
            let a3613 = v7;
            let a3614 = 1u64;
            nat_add(a3613, a3614)?
        }
    } else {
        v7
    };
    let v13: u64 = if v9 {
        f179(v0.clone(), v12)?
    } else {
        v7
    };
    let v14: Bytes = f316(v0.clone(), v3, v4);
    let v15: u64 = f181(v14.clone())?;
    let a3615 = v13;
    let a3616 = v1;
    if if if nat_eq(a3615, a3616) {
        let a3617 = v3;
        let a3618 = v4;
        if nat_lt(a3617, a3618) {
            true
        } else {
            if v5 {
                let a3619 = v6;
                let a3620 = v7;
                nat_lt(a3619, a3620)
            } else {
                false
            }
        }
    } else {
        false
    } {
        if f208(v9) {
            true
        } else {
            let a3621 = v12;
            let a3622 = v13;
            nat_lt(a3621, a3622)
        }
    } else {
        false
    } {
        Ok(Some(f7(List::cons({
            let a3623 = v2;
            let a3624 = 45u64;
            if nat_eq(a3623, a3624) {
                Bytes::lit(&[0x2d])
            } else {
                Bytes::lit(&[])
            }
        }, List::cons({
            let a3626 = v15;
            let a3627 = {
                let a3625 = v14.clone();
                length_bytes(a3625)
            };
            if nat_eq(a3626, a3627) {
                Bytes::lit(&[0x30])
            } else {
                f316(v14.clone(), v15, {
                    let a3628 = v14;
                    length_bytes(a3628)
                })
            }
        }, List::cons({
            let a3629 = v6;
            let a3630 = v7;
            if nat_lt(a3629, a3630) {
                let a3631 = Bytes::lit(&[0x2e]);
                let a3632 = f316(v0.clone(), v6, v7);
                append_bytes(a3631, a3632)
            } else {
                Bytes::lit(&[])
            }
        }, List::cons(if v9 {
            let a3635 = if if v11 {
                let a3633 = v10;
                let a3634 = 45u64;
                nat_eq(a3633, a3634)
            } else {
                false
            } {
                Bytes::lit(&[0x65, 0x2d])
            } else {
                Bytes::lit(&[0x65])
            };
            let a3636 = f316(v0, v12, v13);
            append_bytes(a3635, a3636)
        } else {
            Bytes::lit(&[])
        }, List::<Bytes>::nil())))))))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f533(v0: Bytes) -> bool {
    f320(v0, List::cons(f8(Str::lit(".inf")), List::cons(f8(Str::lit(".Inf")), List::cons(f8(Str::lit(".INF")), List::cons(f8(Str::lit("+.inf")), List::cons(f8(Str::lit("+.Inf")), List::cons(f8(Str::lit("+.INF")), List::cons(f8(Str::lit("-.inf")), List::cons(f8(Str::lit("-.Inf")), List::cons(f8(Str::lit("-.INF")), List::cons(f8(Str::lit(".nan")), List::cons(f8(Str::lit(".NaN")), List::cons(f8(Str::lit(".NAN")), List::<Bytes>::nil())))))))))))))
}

pub fn f534(v0: Bytes) -> bool {
    f320(v0, List::cons(f8(Str::lit("true")), List::cons(f8(Str::lit("True")), List::cons(f8(Str::lit("TRUE")), List::<Bytes>::nil()))))
}

pub fn f535(v0: Bytes) -> bool {
    f320(v0, List::cons(f8(Str::lit("false")), List::cons(f8(Str::lit("False")), List::cons(f8(Str::lit("FALSE")), List::<Bytes>::nil()))))
}

pub fn f536(v0: Bytes, p3695: u64, v2: u64, v3: bool, p3696: u64, p3697: Bytes) -> R<Adt32> {
    let m3694 = (p3695, (p3696, p3697));
    {
        let mut m3694 = m3694;
        loop {
            let step = {
                let (v1, (v4, v5)) = m3694;
                let m3637 = v4;
                if m3637 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C1(f137(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                } else {
                    let v6 = m3637 - 1;
                    let v7: u64 = f206(v0.clone(), v1)?;
                    let a3638 = v7;
                    let a3639 = 34u64;
                    if nat_eq(a3638, a3639) {
                        Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C0(v5, {
                            let a3640 = v1;
                            let a3641 = 1u64;
                            nat_add(a3640, a3641)?
                        })))
                    } else {
                        let a3642 = v7;
                        let a3643 = 256u64;
                        if nat_eq(a3642, a3643) {
                            Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C1(f137(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                        } else {
                            let a3644 = v7;
                            let a3645 = 92u64;
                            if nat_eq(a3644, a3645) {
                                let v8: u64 = f206(v0.clone(), {
                                    let a3646 = v1;
                                    let a3647 = 1u64;
                                    nat_add(a3646, a3647)?
                                })?;
                                let m3648 = f637(v8)?;
                                match m3648 {
                                    None => {
                                        let a3653 = v8;
                                        let a3654 = 10u64;
                                        if nat_eq(a3653, a3654) {
                                            if v3 {
                                                Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C1(f638(v1)?)))
                                            } else {
                                                let m3657 = f639(v0.clone(), {
                                                    let a3655 = v1;
                                                    let a3656 = 1u64;
                                                    nat_add(a3655, a3656)?
                                                }, v2, v6, 0u64)?;
                                                match m3657 {
                                                    Adt38::C0(v11, v12) => {
                                                        Ok::<(u64, (u64, Bytes)), R<Adt32>>((v12, (v6, {
                                                            let a3658 = v5;
                                                            let a3659 = f526(v11);
                                                            append_bytes(a3658, a3659)
                                                        })))
                                                    }
                                                    Adt38::C1(v10) => {
                                                        Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C1(v10)))
                                                    }
                                                }
                                            }
                                        } else {
                                            let v13: u64 = {
                                                let a3660 = v8;
                                                let a3661 = 120u64;
                                                if nat_eq(a3660, a3661) {
                                                    2u64
                                                } else {
                                                    let a3662 = v8;
                                                    let a3663 = 117u64;
                                                    if nat_eq(a3662, a3663) {
                                                        4u64
                                                    } else {
                                                        let a3664 = v8;
                                                        let a3665 = 85u64;
                                                        if nat_eq(a3664, a3665) {
                                                            8u64
                                                        } else {
                                                            0u64
                                                        }
                                                    }
                                                }
                                            };
                                            let a3666 = v13;
                                            let a3667 = 0u64;
                                            if nat_eq(a3666, a3667) {
                                                Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C1(f137(Str::lit("an escape must be one of YAML 1.2.2 [42] to [62]"), v1)?)))
                                            } else {
                                                let m3670 = f640(v0.clone(), {
                                                    let a3668 = v1;
                                                    let a3669 = 2u64;
                                                    nat_add(a3668, a3669)?
                                                }, v13, 0u64)?;
                                                match m3670 {
                                                    None => {
                                                        Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C1(f137(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                    }
                                                    Some(v14) => {
                                                        let a3671 = 1114111u64;
                                                        let a3672 = v14;
                                                        if if nat_lt(a3671, a3672) {
                                                            true
                                                        } else {
                                                            let a3673 = 55296u64;
                                                            let a3674 = v14;
                                                            if nat_le(a3673, a3674) {
                                                                let a3675 = v14;
                                                                let a3676 = 57343u64;
                                                                nat_le(a3675, a3676)
                                                            } else {
                                                                false
                                                            }
                                                        } {
                                                            Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C1(f137(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                        } else {
                                                            Ok::<(u64, (u64, Bytes)), R<Adt32>>(({
                                                                let a3679 = {
                                                                    let a3677 = v1;
                                                                    let a3678 = 2u64;
                                                                    nat_add(a3677, a3678)?
                                                                };
                                                                let a3680 = v13;
                                                                nat_add(a3679, a3680)?
                                                            }, (v6, {
                                                                let a3681 = v5;
                                                                let a3682 = f466(v14)?;
                                                                append_bytes(a3681, a3682)
                                                            })))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Some(v9) => {
                                        Ok::<(u64, (u64, Bytes)), R<Adt32>>(({
                                            let a3649 = v1;
                                            let a3650 = 2u64;
                                            nat_add(a3649, a3650)?
                                        }, (v6, {
                                            let a3651 = v5;
                                            let a3652 = f466(v9)?;
                                            append_bytes(a3651, a3652)
                                        })))
                                    }
                                }
                            } else {
                                if if f299(v7) {
                                    true
                                } else {
                                    let a3683 = v7;
                                    let a3684 = 10u64;
                                    nat_eq(a3683, a3684)
                                } {
                                    let v15: u64 = f302(v0.clone(), v1)?;
                                    let a3685 = f206(v0.clone(), v15)?;
                                    let a3686 = 10u64;
                                    if nat_eq(a3685, a3686) {
                                        if v3 {
                                            Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C1(f638(v1)?)))
                                        } else {
                                            let m3687 = f639(v0.clone(), v15, v2, v6, 0u64)?;
                                            match m3687 {
                                                Adt38::C0(v17, v18) => {
                                                    Ok::<(u64, (u64, Bytes)), R<Adt32>>((v18, (v6, {
                                                        let a3688 = v5;
                                                        let a3689 = f544(v17);
                                                        append_bytes(a3688, a3689)
                                                    })))
                                                }
                                                Adt38::C1(v16) => {
                                                    Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C1(v16)))
                                                }
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (u64, Bytes)), R<Adt32>>((v15, (v6, {
                                            let a3690 = v5;
                                            let a3691 = f316(v0.clone(), v1, v15);
                                            append_bytes(a3690, a3691)
                                        })))
                                    }
                                } else {
                                    let v19: u64 = f641(v0.clone(), v1)?;
                                    Ok::<(u64, (u64, Bytes)), R<Adt32>>((v19, (v6, {
                                        let a3692 = v5;
                                        let a3693 = f316(v0.clone(), v1, v19);
                                        append_bytes(a3692, a3693)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3694 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f537(v0: Bytes, p3725: u64, v2: u64, v3: bool, p3726: u64, p3727: Bytes) -> R<Adt32> {
    let m3724 = (p3725, (p3726, p3727));
    {
        let mut m3724 = m3724;
        loop {
            let step = {
                let (v1, (v4, v5)) = m3724;
                let m3698 = v4;
                if m3698 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C1(f137(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                } else {
                    let v6 = m3698 - 1;
                    let v7: u64 = f206(v0.clone(), v1)?;
                    let a3699 = v7;
                    let a3700 = 39u64;
                    if nat_eq(a3699, a3700) {
                        let a3703 = f206(v0.clone(), {
                            let a3701 = v1;
                            let a3702 = 1u64;
                            nat_add(a3701, a3702)?
                        })?;
                        let a3704 = 39u64;
                        if nat_eq(a3703, a3704) {
                            Ok::<(u64, (u64, Bytes)), R<Adt32>>(({
                                let a3705 = v1;
                                let a3706 = 2u64;
                                nat_add(a3705, a3706)?
                            }, (v6, {
                                let a3707 = v5;
                                let a3708 = Bytes::lit(&[0x27]);
                                append_bytes(a3707, a3708)
                            })))
                        } else {
                            Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C0(v5, {
                                let a3709 = v1;
                                let a3710 = 1u64;
                                nat_add(a3709, a3710)?
                            })))
                        }
                    } else {
                        let a3711 = v7;
                        let a3712 = 256u64;
                        if nat_eq(a3711, a3712) {
                            Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C1(f137(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                        } else {
                            if if f299(v7) {
                                true
                            } else {
                                let a3713 = v7;
                                let a3714 = 10u64;
                                nat_eq(a3713, a3714)
                            } {
                                let v8: u64 = f302(v0.clone(), v1)?;
                                let a3715 = f206(v0.clone(), v8)?;
                                let a3716 = 10u64;
                                if nat_eq(a3715, a3716) {
                                    if v3 {
                                        Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C1(f638(v1)?)))
                                    } else {
                                        let m3717 = f639(v0.clone(), v8, v2, v6, 0u64)?;
                                        match m3717 {
                                            Adt38::C0(v10, v11) => {
                                                Ok::<(u64, (u64, Bytes)), R<Adt32>>((v11, (v6, {
                                                    let a3718 = v5;
                                                    let a3719 = f544(v10);
                                                    append_bytes(a3718, a3719)
                                                })))
                                            }
                                            Adt38::C1(v9) => {
                                                Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C1(v9)))
                                            }
                                        }
                                    }
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Adt32>>((v8, (v6, {
                                        let a3720 = v5;
                                        let a3721 = f316(v0.clone(), v1, v8);
                                        append_bytes(a3720, a3721)
                                    })))
                                }
                            } else {
                                let v12: u64 = f642(v0.clone(), v1)?;
                                Ok::<(u64, (u64, Bytes)), R<Adt32>>((v12, (v6, {
                                    let a3722 = v5;
                                    let a3723 = f316(v0.clone(), v1, v12);
                                    append_bytes(a3722, a3723)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3724 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f538(v0: Bytes, v1: u64, v2: u64) -> R<Result<u64, Bytes>> {
    let v3: (u64, bool) = {
        let (h3728, _) = f643(Fn20::F644(v2, v0.clone()), {
            let a3730 = {
                let a3729 = v0.clone();
                length_bytes(a3729)
            };
            let a3731 = 1u64;
            nat_add(a3730, a3731)?
        }, (v1, false))?;
        h3728
    };
    let (_, h3732) = v3;
    if h3732 {
        if f204(v0, {
            let (h3733, _) = v3;
            h3733
        })? {
            Ok(Err::<u64, Bytes>(f137(Str::lit("a document marker cannot appear inside a flow collection (YAML 1.2.2 [206])"), {
                let (h3734, _) = v3;
                h3734
            })?))
        } else {
            Ok(Err::<u64, Bytes>(f137(Str::lit("a line of a flow collection is indented less than its node (YAML 1.2.2 [80])"), {
                let (h3735, _) = v3;
                h3735
            })?))
        }
    } else {
        Ok(Ok::<u64, Bytes>({
            let (h3736, _) = v3;
            h3736
        }))
    }
}

pub fn f539(v0: Bytes, v1: u64, v2: u64) -> R<Adt32> {
    let a3737 = f206(v0.clone(), v1)?;
    let a3738 = 33u64;
    if nat_eq(a3737, a3738) {
        let m3739 = f317(v0.clone(), v1)?;
        match m3739 {
            Ok(v4) => {
                let m3740 = f538(v0.clone(), v4, v2)?;
                match m3740 {
                    Ok(v6) => {
                        let a3741 = f206(v0.clone(), v6)?;
                        let a3742 = 58u64;
                        if nat_eq(a3741, a3742) {
                            let a3745 = f312(v0, v1)?;
                            let a3746 = {
                                let a3743 = v1;
                                let a3744 = 1u64;
                                nat_add(a3743, a3744)?
                            };
                            if nat_eq(a3745, a3746) {
                                Ok(Adt32::C1(f137(Str::lit("an empty node with the non-specific tag ! is not read (YAML 1.2.2 [100])"), v1)?))
                            } else {
                                Ok(Adt32::C0(Bytes::lit(&[]), v6))
                            }
                        } else {
                            f645(v0, v6, v2)
                        }
                    }
                    Err(v5) => {
                        Ok(Adt32::C1(v5))
                    }
                }
            }
            Err(v3) => {
                Ok(Adt32::C1(v3))
            }
        }
    } else {
        f645(v0, v1, v2)
    }
}

pub fn f540(v0: u64) -> R<bool> {
    let a3747 = f315(f8(Str::lit(",[]{}-?:#&*!|>'\"@`")), v0)?;
    let a3748 = 18u64;
    if nat_lt(a3747, a3748) {
        Ok(true)
    } else {
        let a3749 = v0;
        let a3750 = 37u64;
        Ok(nat_eq(a3749, a3750))
    }
}

pub fn f541(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f646(v1, v0, v2)
}

pub fn f542(v0: u64) -> R<Bytes> {
    f137(Str::lit("a mapping value is not allowed here: a plain scalar holds ': ' (YAML 1.2.2 [130])"), v0)
}

pub fn f543(v0: Bytes, v1: u64) -> R<(u64, u64)> {
    let (h3751, _) = f270(Fn6::F647(v0.clone()), {
        let a3753 = {
            let a3752 = v0;
            length_bytes(a3752)
        };
        let a3754 = 1u64;
        nat_add(a3753, a3754)?
    }, (v1, 0u64))?;
    Ok(h3751)
}

pub fn f544(v0: u64) -> Bytes {
    let a3755 = v0;
    let a3756 = 0u64;
    if nat_eq(a3755, a3756) {
        Bytes::lit(&[0x20])
    } else {
        f526(v0)
    }
}

pub fn f545(v0: Bytes, v1: bool, v2: u64) -> R<Option<u64>> {
    f648(v0, v1, v2)
}

pub fn f546(v0: Bytes, p3761: List<Bytes>, p3762: u64) -> R<u64> {
    let m3760 = (p3761, p3762);
    {
        let mut m3760 = m3760;
        loop {
            let step = {
                let (v1, v2) = m3760;
                let m3757 = v1;
                match m3757.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<u64>>(Ok(0u64))
                    }
                    Some((v3, v4)) => {
                        if f32(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<u64>>(Ok(v2))
                        } else {
                            Ok::<(List<Bytes>, u64), R<u64>>((v4, {
                                let a3758 = v2;
                                let a3759 = 1u64;
                                nat_add(a3758, a3759)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3760 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f547(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a3764 = v2;
    let a3765 = {
        let a3763 = v0.clone();
        length_bytes(a3763)
    };
    if if nat_lt(a3764, a3765) {
        f208({
            let a3766 = f206(v0, v2)?;
            let a3767 = v1;
            nat_eq(a3766, a3767)
        })
    } else {
        false
    } {
        Ok(Some({
            let a3768 = v2;
            let a3769 = 1u64;
            nat_add(a3768, a3769)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f548(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f649(v1, v0, v2)
}

pub fn f549(v0: u64) -> bool {
    if f550(v0) {
        true
    } else {
        let a3770 = 65u64;
        let a3771 = v0;
        if if nat_le(a3770, a3771) {
            let a3772 = v0;
            let a3773 = 70u64;
            nat_le(a3772, a3773)
        } else {
            false
        } {
            true
        } else {
            let a3774 = 97u64;
            let a3775 = v0;
            if nat_le(a3774, a3775) {
                let a3776 = v0;
                let a3777 = 102u64;
                nat_le(a3776, a3777)
            } else {
                false
            }
        }
    }
}

pub fn f550(v0: u64) -> bool {
    let a3778 = 48u64;
    let a3779 = v0;
    if nat_le(a3778, a3779) {
        let a3780 = v0;
        let a3781 = 57u64;
        nat_le(a3780, a3781)
    } else {
        false
    }
}

pub fn f551(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f160(v0.clone(), v1)?;
    let a3782 = v2;
    let a3783 = 69u64;
    if if nat_eq(a3782, a3783) {
        true
    } else {
        let a3784 = v2;
        let a3785 = 101u64;
        nat_eq(a3784, a3785)
    } {
        let v3: u64 = f160(v0.clone(), {
            let a3786 = v1;
            let a3787 = 1u64;
            nat_add(a3786, a3787)?
        })?;
        let v4: u64 = {
            let a3788 = v3;
            let a3789 = 43u64;
            if if nat_eq(a3788, a3789) {
                true
            } else {
                let a3790 = v3;
                let a3791 = 45u64;
                nat_eq(a3790, a3791)
            } {
                let a3792 = v1;
                let a3793 = 2u64;
                nat_add(a3792, a3793)?
            } else {
                let a3794 = v1;
                let a3795 = 1u64;
                nat_add(a3794, a3795)?
            }
        };
        let v5: u64 = f427(v0, 0u64, v4)?;
        let a3796 = v4;
        let a3797 = v5;
        if nat_lt(a3796, a3797) {
            Ok(v5)
        } else {
            Ok(v1)
        }
    } else {
        Ok(v1)
    }
}

pub fn f552(v0: Adt23, v1: Bytes) -> bool {
    let a3798 = f553(v0.clone());
    let a3799 = 3u64;
    if !nat_eq(a3798, a3799) {
        f32(f554(v0), v1)
    } else {
        false
    }
}

pub fn f553(v0: Adt23) -> u64 {
    let m3800 = v0;
    match m3800 {
        Adt23::C0(v1, _) => {
            v1
        }
    }
}

pub fn f554(v0: Adt23) -> Bytes {
    let m3801 = v0;
    match m3801 {
        Adt23::C0(_, v2) => {
            v2
        }
    }
}

pub fn f555(v0: Bytes) -> Bytes {
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

pub fn f556(v0: Bytes) -> bool {
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

pub fn f557(p3810: u64, p3811: List<Adt23>, p3812: List<Adt14>) -> R<Result<(List<Adt14>, List<Adt23>), Bytes>> {
    let m3809 = (p3810, (p3811, p3812));
    {
        let mut m3809 = m3809;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3809;
                let m3802 = v0;
                if m3802 == 0 {
                    Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt23>), Bytes>(f8(Str::lit("a dictionary has more members than the input has tokens")))))
                } else {
                    let v3 = m3802 - 1;
                    if f430(v1.clone(), Str::lit("}")) {
                        let m3803 = f437(f432(v1), Str::lit(";"));
                        match m3803 {
                            Ok(v5) => {
                                Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Ok::<(List<Adt14>, List<Adt23>), Bytes>((f650(v2, List::<Adt14>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt23>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m3804 = f324(v1)?;
                        match m3804 {
                            Ok(v7) => {
                                let v8: bool = f430(v7.clone(), Str::lit("required"));
                                let m3805 = f442(v3, if v8 {
                                    f432(v7)
                                } else {
                                    v7
                                }, 0u64)?;
                                match m3805 {
                                    Ok(v10) => {
                                        let m3806 = f436(f443(v10.clone()));
                                        match m3806 {
                                            Ok(v12) => {
                                                if if f430(f438(v12.clone()), Str::lit("=")) {
                                                    !v8
                                                } else {
                                                    false
                                                } {
                                                    let m3807 = f559(f432(f438(v12.clone())))?;
                                                    match m3807 {
                                                        Ok(v14) => {
                                                            Ok::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>((v3, (v14, List::cons(Adt14::C0(f440(v12), f566(f444(v10)), 0u64), v2))))
                                                        }
                                                        Err(v13) => {
                                                            Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt23>), Bytes>(v13)))
                                                        }
                                                    }
                                                } else {
                                                    let m3808 = f437(f438(v12.clone()), Str::lit(";"));
                                                    match m3808 {
                                                        Ok(v16) => {
                                                            Ok::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>((v3, (v16, List::cons(Adt14::C0(f440(v12), f566(f444(v10)), if v8 {
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
                Ok(next) => { m3809 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f558(p3821: u64, p3822: List<Adt23>, p3823: List<Adt14>) -> R<Result<(List<Adt14>, List<Adt23>), Bytes>> {
    let m3820 = (p3821, (p3822, p3823));
    {
        let mut m3820 = m3820;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3820;
                let m3813 = v0;
                if m3813 == 0 {
                    Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt23>), Bytes>(f8(Str::lit("an interface has more members than the input has tokens")))))
                } else {
                    let v3 = m3813 - 1;
                    if f430(v1.clone(), Str::lit("}")) {
                        let m3814 = f437(f432(v1), Str::lit(";"));
                        match m3814 {
                            Ok(v5) => {
                                Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Ok::<(List<Adt14>, List<Adt23>), Bytes>((f650(v2, List::<Adt14>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt23>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m3815 = f324(v1)?;
                        match m3815 {
                            Ok(v7) => {
                                let v8: List<Adt23> = if if f430(v7.clone(), Str::lit("stringifier")) {
                                    true
                                } else {
                                    f430(v7.clone(), Str::lit("inherit"))
                                } {
                                    f432(v7.clone())
                                } else {
                                    v7.clone()
                                };
                                let v9: List<Adt23> = if f430(v8.clone(), Str::lit("readonly")) {
                                    f432(v8)
                                } else {
                                    v8
                                };
                                if f430(v9.clone(), Str::lit("attribute")) {
                                    let m3816 = f442(v3, f432(v9), 0u64)?;
                                    match m3816 {
                                        Ok(v11) => {
                                            let m3817 = f436(f443(v11.clone()));
                                            match m3817 {
                                                Ok(v13) => {
                                                    let m3818 = f437(f438(v13.clone()), Str::lit(";"));
                                                    match m3818 {
                                                        Ok(v15) => {
                                                            Ok::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>((v3, (v15, List::cons(Adt14::C0(f440(v13), f566(f444(v11)), 0u64), v2))))
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
                                    let m3819 = f559(v7)?;
                                    match m3819 {
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
                Ok(next) => { m3820 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f559(v0: List<Adt23>) -> R<Result<List<Adt23>, Bytes>> {
    Ok(f431({
        let a3825 = {
            let a3824 = v0.clone();
            length_list(a3824)
        };
        let a3826 = 1u64;
        nat_add(a3825, a3826)?
    }, v0, List::<Bytes>::nil(), true))
}

pub fn f560(v0: List<Adt23>) -> Bytes {
    let m3827 = v0;
    match m3827.uncons() {
        None => {
            f8(Str::lit("the end of the input"))
        }
        Some((v1, _)) => {
            f7(List::cons(f8(Str::lit("'")), List::cons(f554(v1), List::cons(f8(Str::lit("'")), List::<Bytes>::nil()))))
        }
    }
}

pub fn f561(v0: Bytes) -> Bytes {
    if f53(v0.clone(), f8(Str::lit("_"))) {
        f46(v0.clone(), 1u64, {
            let a3829 = {
                let a3828 = v0;
                length_bytes(a3828)
            };
            let a3830 = 1u64;
            nat_sub(a3829, a3830)
        })
    } else {
        v0
    }
}

pub fn f562(v0: Adt15, v1: List<Adt23>) -> Result<Adt34, Bytes> {
    if f430(v1.clone(), Str::lit("?")) {
        Ok::<Adt34, Bytes>(Adt34::C0(f563(Str::lit("Option"), v0), f432(v1)))
    } else {
        Ok::<Adt34, Bytes>(Adt34::C0(v0, v1))
    }
}

pub fn f563(v0: Str, v1: Adt15) -> Adt15 {
    let v2: Adt15 = f566(v1);
    Adt15::C0(0u64, f7(List::cons(f8(Str::lit("(")), List::cons({
        let a3831 = v0;
        utf8_encode(a3831)
    }, List::cons(f8(Str::lit(" ")), List::cons(f342(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), f343(v2.clone()), f340(v2), Bytes::lit(&[]))
}

pub fn f564(v0: Bytes) -> Adt15 {
    Adt15::C0(0u64, Bytes::lit(&[]), List::<Bytes>::nil(), v0, Bytes::lit(&[]))
}

pub fn f565(v0: Bytes) -> Option<Adt15> {
    f651(v0, f652())
}

pub fn f566(v0: Adt15) -> Adt15 {
    let a3832 = f341(v0.clone());
    let a3833 = 0u64;
    if nat_eq(a3832, a3833) {
        v0
    } else {
        Adt15::C0(0u64, Bytes::lit(&[]), f343(v0.clone()), if f32(f340(v0.clone()), Bytes::lit(&[])) {
            f653()
        } else {
            f340(v0)
        }, Bytes::lit(&[]))
    }
}

pub fn f567(v0: Adt15) -> Bytes {
    let m3834 = v0;
    match m3834 {
        Adt15::C0(_, _, _, _, v5) => {
            v5
        }
    }
}

pub fn f568(p3851: u64, p3852: u64, p3853: u64, p3854: Bytes) -> Bytes {
    let m3850 = (p3851, (p3852, (p3853, p3854)));
    {
        let mut m3850 = m3850;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m3850;
                let m3835 = v0;
                if m3835 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m3835 - 1;
                    if f113({
                        let a3836 = v1;
                        let a3837 = 0u64;
                        nat_eq(a3836, a3837)
                    }, {
                        let a3838 = v2;
                        let a3839 = 0u64;
                        nat_eq(a3838, a3839)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a3840 = v1;
                            let a3841 = 16u64;
                            let a3842 = 0u64;
                            nat_quot(a3840, a3841, a3842)
                        }, ({
                            let a3843 = v2;
                            let a3844 = 1u64;
                            nat_sub(a3843, a3844)
                        }, {
                            let a3848 = f654({
                                let a3845 = v1;
                                let a3846 = 16u64;
                                let a3847 = 0u64;
                                nat_rem(a3845, a3846, a3847)
                            });
                            let a3849 = v3;
                            append_bytes(a3848, a3849)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m3850 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f569(v0: Adt14, v1: List<Adt14>) -> R<List<Adt14>> {
    let m3855 = v1;
    match m3855.uncons() {
        None => {
            Ok(List::cons(v0, List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            if f655({
                let a3857 = {
                    let a3856 = f453(v0.clone());
                    length_bytes(a3856)
                };
                let a3858 = 1u64;
                nat_add(a3857, a3858)?
            }, f453(v0.clone()), f453(v2.clone()), 0u64)? {
                Ok(List::cons(v0, List::cons(v2, v3)))
            } else {
                Ok(List::cons(v2, f569(v0, v3)?))
            }
        }
    }
}

pub fn f570(v0: List<Adt13>, v1: Bytes) -> List<Adt14> {
    let m3859 = f226(v0.clone(), 1u64, v1.clone());
    match m3859 {
        None => {
            f450(v0, 6u64, v1)
        }
        Some(v2) => {
            f448(f449(v2), f450(v0, 6u64, v1))
        }
    }
}

pub fn f571(v0: Bytes) -> R<Option<Bytes>> {
    let m3861 = f656({
        let a3860 = v0.clone();
        length_bytes(a3860)
    }, v0.clone(), 0u64, Bytes::lit(&[]))?;
    match m3861 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a3863 = {
                let a3862 = v1.clone();
                length_bytes(a3862)
            };
            let a3864 = 0u64;
            if nat_eq(a3863, a3864) {
                let a3866 = {
                    let a3865 = v0;
                    length_bytes(a3865)
                };
                let a3867 = 0u64;
                if nat_eq(a3866, a3867) {
                    Ok(Some(f8(Str::lit("u"))))
                } else {
                    Ok(None::<Bytes>)
                }
            } else {
                Ok(Some(f573(v1)?))
            }
        }
    }
}

pub fn f572(v0: Bytes) -> R<Option<Bytes>> {
    if f657({
        let a3868 = v0.clone();
        length_bytes(a3868)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f658(v0)?;
        if f659(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m3871 = {
                let a3869 = v1.clone();
                let a3870 = 0u64;
                index_bytes(a3869, a3870)
            };
            match m3871 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f281(f62(v2)?) {
                        Ok(Some({
                            let a3872 = f8(Str::lit("n"));
                            let a3873 = v1;
                            append_bytes(a3872, a3873)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f660({
            let a3874 = v0.clone();
            length_bytes(a3874)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f573(v0: Bytes) -> R<Bytes> {
    if f122(v0.clone(), f661()) {
        f446(v0)
    } else {
        Ok(v0)
    }
}

pub fn f574() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{3}/\u{7f}\u{3}@\u{0}\u{3}\u{7f}}\u{4}\u{0}\u{0}C\u{7f}\u{7f}"))
}

pub fn f575(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a3875 = v1;
        let a3876 = 3u64;
        nat_mul(a3875, a3876)?
    };
    let a3887 = {
        let a3883 = {
            let a3877 = f111(v0.clone(), v2)?;
            let a3878 = 16384u64;
            nat_mul(a3877, a3878)?
        };
        let a3884 = {
            let a3881 = f111(v0.clone(), {
                let a3879 = v2;
                let a3880 = 1u64;
                nat_add(a3879, a3880)?
            })?;
            let a3882 = 128u64;
            nat_mul(a3881, a3882)?
        };
        nat_add(a3883, a3884)?
    };
    let a3888 = f111(v0, {
        let a3885 = v2;
        let a3886 = 2u64;
        nat_add(a3885, a3886)?
    })?;
    nat_add(a3887, a3888)
}

pub fn f576(v0: u64) -> R<bool> {
    f346(v0, f662())
}

pub fn f577(v0: u64) -> R<bool> {
    f346(v0, f663())
}

pub fn f578(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f664(v1, v0, v2)
}

pub fn f579(v0: Bytes, v1: u64) -> Bytes {
    f46(v0, v1, 1u64)
}

pub fn f580() -> Bytes {
    f8(Str::lit("\u{0}\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\u{9}\u{a}\u{b}\u{c}\u{d}\u{e}\u{f}\u{10}\u{11}\u{12}\u{13}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{1b}\u{1c}\u{1d}\u{1e}\u{1f} !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~\u{7f}"))
}

pub fn f581() -> Bytes {
    f8(Str::lit("\u{80}\u{c0}\u{100}\u{140}\u{180}\u{1c0}\u{200}\u{240}\u{280}\u{2c0}\u{300}\u{340}\u{380}\u{3c0}\u{400}\u{440}\u{480}\u{4c0}\u{500}\u{540}\u{580}\u{5c0}\u{600}\u{640}\u{680}\u{6c0}\u{700}\u{740}\u{780}\u{7c0}"))
}

pub fn f582(v0: u64) -> R<Bytes> {
    Ok(f579(f665(), {
        let a3894 = {
            let a3892 = {
                let a3889 = v0;
                let a3890 = 64u64;
                let a3891 = 0u64;
                nat_rem(a3889, a3890, a3891)
            };
            let a3893 = 2u64;
            nat_mul(a3892, a3893)?
        };
        let a3895 = 1u64;
        nat_add(a3894, a3895)?
    }))
}

pub fn f583() -> Bytes {
    f8(Str::lit("\u{800}\u{1000}\u{2000}\u{3000}\u{4000}\u{5000}\u{6000}\u{7000}\u{8000}\u{9000}\u{a000}\u{b000}\u{c000}\u{d000}\u{e000}\u{f000}"))
}

pub fn f584(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<Option<u64>> {
    if f666(f160(v0, v3)?, v1, v2)? {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3896 = v3;
            let a3897 = 1u64;
            nat_add(a3896, a3897)?
        }))
    }
}

pub fn f585(v0: Bytes) -> Result<Bytes, Bytes> {
    Err::<Bytes, Bytes>(v0)
}

pub fn f586() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f587(v0: Bytes) -> R<Option<Bytes>> {
    let m3898 = f231(v0.clone(), f8(Str::lit(" ")), 0u64)?;
    match m3898 {
        None => {
            let m3899 = f231(v0.clone(), f8(Str::lit(":")), 0u64)?;
            match m3899 {
                None => {
                    let v3: Bytes = f7(List::cons(f8(Str::lit(" ")), List::cons(v0, List::cons(f8(Str::lit(":")), List::<Bytes>::nil()))));
                    let m3900 = f231(f667(), v3.clone(), 0u64)?;
                    match m3900 {
                        None => {
                            Ok(None::<Bytes>)
                        }
                        Some(v4) => {
                            let v5: u64 = {
                                let a3902 = v4;
                                let a3903 = {
                                    let a3901 = v3;
                                    length_bytes(a3901)
                                };
                                nat_add(a3902, a3903)?
                            };
                            let m3904 = f231(f667(), f8(Str::lit(" ")), v5)?;
                            match m3904 {
                                None => {
                                    Ok(None::<Bytes>)
                                }
                                Some(v6) => {
                                    Ok(Some(f46(f667(), v5, {
                                        let a3905 = v6;
                                        let a3906 = v5;
                                        nat_sub(a3905, a3906)
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

pub fn f588() -> List<Bytes> {
    List::cons(f8(Str::lit("complexType")), List::cons(f8(Str::lit("simpleType")), List::cons(f8(Str::lit("element")), List::cons(f8(Str::lit("attribute")), List::cons(f8(Str::lit("attributeGroup")), List::cons(f8(Str::lit("group")), List::<Bytes>::nil()))))))
}

pub fn f589(v0: u64, v1: Adt25, v2: Adt26) -> R<Result<List<Adt28>, Bytes>> {
    let v3: Bytes = f125(match v2.clone() {
        Adt26::C0(_, _, h3907, _) => {
            h3907
        }
    })?;
    let v4: (Bytes, Bytes) = (match v2.clone() {
        Adt26::C0(_, h3908, _, _) => {
            h3908
        }
    }, match v2.clone() {
        Adt26::C0(_, _, h3909, _) => {
            h3909
        }
    });
    let v5: bool = f668(match v2.clone() {
        Adt26::C0(_, _, _, h3910) => {
            h3910
        }
    }, Str::lit("abstract"), Str::lit("true"));
    let m3912 = f669(match v1.clone() {
        Adt25::C0(h3911, _) => {
            h3911
        }
    }, v4, f374(v5))?;
    match m3912 {
        Ok(v7) => {
            if v5 {
                let m3913 = v7.clone();
                match m3913.uncons() {
                    None => {
                        Ok(Err::<List<Adt28>, Bytes>(f595(f8(Str::lit("an abstract complex type with no type derived from it in this schema set admits no instance (XSD 1.1 Part 1, sections 3.4.1 and 3.3.4.2)")), v3)))
                    }
                    Some((_, _)) => {
                        let m3914 = f670(v3, v7.clone(), f671(v7.clone())?, f671(v7)?)?;
                        match m3914 {
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
                let m3915 = v7.clone();
                match m3915.uncons() {
                    None => {
                        f672(v0, v1, v3, match v2.clone() {
                            Adt26::C0(_, h3916, _, _) => {
                                h3916
                            }
                        }, match v2 {
                            Adt26::C0(_, _, _, h3917) => {
                                h3917
                            }
                        })
                    }
                    Some((_, _)) => {
                        let v14: Bytes = f673(v3.clone());
                        let m3920 = f672(v0, v1, v14.clone(), match v2.clone() {
                            Adt26::C0(_, h3918, _, _) => {
                                h3918
                            }
                        }, match v2.clone() {
                            Adt26::C0(_, _, _, h3919) => {
                                h3919
                            }
                        })?;
                        match m3920 {
                            Ok(v16) => {
                                let m3922 = f670(v3, List::cons(match v2 {
                                    Adt26::C0(_, _, h3921, _) => {
                                        h3921
                                    }
                                }, v7.clone()), List::cons(v14.clone(), f671(v7.clone())?), List::cons(v14, f671(v7)?))?;
                                match m3922 {
                                    Ok(v18) => {
                                        Ok(Ok::<List<Adt28>, Bytes>(f474(v16, List::cons(v18, List::<Adt28>::nil()))))
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

pub fn f590(v0: Bytes, v1: Adt27, v2: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3925 = f249(v0.clone(), match v1.clone() {
        Adt27::C0(_, _, _, h3923, _) => {
            h3923
        }
    }, match v1 {
        Adt27::C0(_, _, _, _, h3924) => {
            h3924
        }
    }, v2)?;
    match m3925 {
        Ok(v4) => {
            Ok(Ok::<Adt28, Bytes>(Adt28::C0(f125(v0)?, match v4.clone() {
                Adt24::C0(_, h3926) => {
                    h3926
                }
            }, match v4 {
                Adt24::C0(h3927, _) => {
                    h3927
                }
            }, List::<Bytes>::nil())))
        }
        Err(v3) => {
            Ok(Err::<Adt28, Bytes>(v3))
        }
    }
}

pub fn f591(v0: u64, v1: Adt25, v2: Bytes, v3: Bytes, v4: Adt27) -> R<Result<Adt35, Bytes>> {
    let m3928 = v0;
    if m3928 == 0 {
        Ok(Err::<Adt35, Bytes>(f595(f8(Str::lit("the element declarations nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m3928 - 1;
        let m3929 = f376(v4.clone(), Str::lit("type"));
        match m3929 {
            None => {
                let m3933 = f593(v4.clone())?;
                match m3933 {
                    Ok(v10) => {
                        let m3934 = f594(v10, List::cons(f8(Str::lit("complexType")), List::cons(f8(Str::lit("simpleType")), List::<Bytes>::nil())));
                        match m3934 {
                            None => {
                                Ok(Ok::<Adt35, Bytes>(Adt35::C0(Adt24::C0(f675(v4, f477()), List::cons(f477(), List::<Bytes>::nil())), List::<Adt28>::nil())))
                            }
                            Some(v11) => {
                                if f375(v11.clone(), Str::lit("complexType")) {
                                    let m3935 = f672(v5, v1, v2.clone(), v3, v11)?;
                                    match m3935 {
                                        Ok(v13) => {
                                            Ok(Ok::<Adt35, Bytes>(Adt35::C0(Adt24::C0(f675(v4, v2.clone()), List::cons(v2, List::<Bytes>::nil())), v13)))
                                        }
                                        Err(v12) => {
                                            Ok(Err::<Adt35, Bytes>(v12))
                                        }
                                    }
                                } else {
                                    let m3936 = f590(v2.clone(), v11, v3)?;
                                    match m3936 {
                                        Ok(v15) => {
                                            Ok(Ok::<Adt35, Bytes>(Adt35::C0(Adt24::C0(f675(v4, v2.clone()), List::cons(v2, List::<Bytes>::nil())), List::cons(v15, List::<Adt28>::nil()))))
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
                let m3930 = f674(v1, v4.clone(), v6)?;
                match m3930 {
                    Ok(v8) => {
                        Ok(Ok::<Adt35, Bytes>(Adt35::C0(Adt24::C0(f675(v4, match v8.clone() {
                            Adt24::C0(h3931, _) => {
                                h3931
                            }
                        }), match v8 {
                            Adt24::C0(_, h3932) => {
                                h3932
                            }
                        }), List::<Adt28>::nil())))
                    }
                    Err(v7) => {
                        Ok(Err::<Adt35, Bytes>(f595(v7, v2)))
                    }
                }
            }
        }
    }
}

pub fn f592(v0: Adt25, v1: Bytes, v2: Bytes, v3: Adt27) -> R<Result<Adt35, Bytes>> {
    let m3937 = f376(v3.clone(), Str::lit("type"));
    match m3937 {
        None => {
            let m3939 = f593(v3)?;
            match m3939 {
                Ok(v8) => {
                    let m3940 = f594(v8, List::cons(f8(Str::lit("simpleType")), List::<Bytes>::nil()));
                    match m3940 {
                        None => {
                            Ok(Err::<Adt35, Bytes>(f595(f8(Str::lit("an attribute declaration with no type has the type xs:anySimpleType, which is not translated, because it is the union of every simple value space (XSD 1.1 Part 1, section 3.2.2.1; Part 2, section 3.2.1)")), v2)))
                        }
                        Some(v9) => {
                            let m3941 = f590(v2.clone(), v9, v1)?;
                            match m3941 {
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
            let m3938 = f674(v0, v3, v4)?;
            match m3938 {
                Ok(v6) => {
                    Ok(Ok::<Adt35, Bytes>(Adt35::C0(v6, List::<Adt28>::nil())))
                }
                Err(v5) => {
                    Ok(Err::<Adt35, Bytes>(f595(v5, v2)))
                }
            }
        }
    }
}

pub fn f593(v0: Adt27) -> R<Result<List<Adt27>, Bytes>> {
    f261(match v0.clone() {
        Adt27::C0(_, _, _, h3942, _) => {
            h3942
        }
    }, match v0 {
        Adt27::C0(_, _, _, _, h3943) => {
            h3943
        }
    })
}

pub fn f594(p3947: List<Adt27>, v1: List<Bytes>) -> Option<Adt27> {
    let m3946 = p3947;
    {
        let mut m3946 = m3946;
        loop {
            let step = {
                let v0 = m3946;
                let m3944 = v0;
                match m3944.uncons() {
                    None => {
                        Err::<List<Adt27>, Option<Adt27>>(None::<Adt27>)
                    }
                    Some((v2, v3)) => {
                        if f122(match v2.clone() {
                            Adt27::C0(_, h3945, _, _, _) => {
                                h3945
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
                Ok(next) => { m3946 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f595(v0: Bytes, v1: Bytes) -> Bytes {
    f257(v0, f8(Str::lit(" at ")), v1)
}

pub fn f596(v0: u64, v1: Adt25, v2: Bytes, v3: Bytes, v4: Adt27) -> R<Result<List<Adt28>, Bytes>> {
    let m3948 = v0;
    if m3948 == 0 {
        Ok(Err::<List<Adt28>, Bytes>(f595(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m3948 - 1;
        let m3949 = f593(v4.clone())?;
        match m3949 {
            Ok(v7) => {
                if f375(v4, Str::lit("choice")) {
                    let m3950 = f676(v5, v1, v2.clone(), v3, false, v7, f598())?;
                    match m3950 {
                        Ok(v9) => {
                            let m3954 = f670(v2, f677(match v9.clone() {
                                Adt36::C0(h3951, _, _, _, _) => {
                                    h3951
                                }
                            }), f678(match v9.clone() {
                                Adt36::C0(h3952, _, _, _, _) => {
                                    h3952
                                }
                            }), f679(match v9.clone() {
                                Adt36::C0(h3953, _, _, _, _) => {
                                    h3953
                                }
                            }))?;
                            match m3954 {
                                Ok(v11) => {
                                    Ok(Ok::<List<Adt28>, Bytes>(f474(match v9 {
                                        Adt36::C0(_, h3955, _, _, _) => {
                                            h3955
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
                    let m3956 = f676(v5, v1, v2.clone(), v3, true, v7, f598())?;
                    match m3956 {
                        Ok(v13) => {
                            let m3958 = f680(v2, match v13.clone() {
                                Adt36::C0(h3957, _, _, _, _) => {
                                    h3957
                                }
                            })?;
                            match m3958 {
                                Ok(v15) => {
                                    Ok(Ok::<List<Adt28>, Bytes>(f474(match v13 {
                                        Adt36::C0(_, h3959, _, _, _) => {
                                            h3959
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

pub fn f597(v0: u64, v1: Adt25, v2: Bytes, v3: Bytes, v4: List<Adt27>, v5: Adt36) -> R<Result<Adt36, Bytes>> {
    let m3960 = v0;
    if m3960 == 0 {
        Ok(Err::<Adt36, Bytes>(f595(f8(Str::lit("the attribute group references nest deeper than the schema set has components, so they refer to each other in a cycle (XSD 1.1 Part 1, section 3.6.3)")), v2)))
    } else {
        let v6 = m3960 - 1;
        let m3961 = v4;
        match m3961.uncons() {
            None => {
                Ok(Ok::<Adt36, Bytes>(v5))
            }
            Some((v7, v8)) => {
                if f375(v7.clone(), Str::lit("attribute")) {
                    if f668(v7.clone(), Str::lit("use"), Str::lit("prohibited")) {
                        f597(v6, v1, v2, v3, v8, v5)
                    } else {
                        let v9: bool = f668(v7.clone(), Str::lit("use"), Str::lit("required"));
                        let m3962 = f376(v7.clone(), Str::lit("ref"));
                        match m3962 {
                            None => {
                                let m3972 = f376(v7.clone(), Str::lit("name"));
                                match m3972 {
                                    None => {
                                        Ok(Err::<Adt36, Bytes>(f595(f8(Str::lit("a local attribute declaration must have a name or a ref (XSD 1.1 Part 1, section 3.2.3)")), v2)))
                                    }
                                    Some(v16) => {
                                        let m3975 = f592(v1.clone(), v3.clone(), {
                                            let a3973 = v2.clone();
                                            let a3974 = f125(v16.clone())?;
                                            append_bytes(a3973, a3974)
                                        }, v7)?;
                                        match m3975 {
                                            Ok(v18) => {
                                                f597(v6, v1, v2, v3, v8, f682(v5, v16, v18, v9))
                                            }
                                            Err(v17) => {
                                                Ok(Err::<Adt36, Bytes>(v17))
                                            }
                                        }
                                    }
                                }
                            }
                            Some(v10) => {
                                let m3964 = f164(match v7 {
                                    Adt27::C0(_, _, _, _, h3963) => {
                                        h3963
                                    }
                                }, v10.clone())?;
                                match m3964 {
                                    Ok(v12) => {
                                        if f32({
                                            let (h3965, _) = v12.clone();
                                            h3965
                                        }, f242()) {
                                            Ok(Err::<Adt36, Bytes>(f595(f257(f8(Str::lit("the attribute ")), v10, f8(Str::lit(" is declared by the schema document for the XML namespace, which this schema set does not import (XSD 1.1 Part 1, sections 3.2.2.3 and 4.2.6)"))), v2)))
                                        } else {
                                            let m3966 = f681(v1.clone(), Str::lit("attribute"), v12.clone());
                                            match m3966 {
                                                None => {
                                                    Ok(Err::<Adt36, Bytes>(f595(f257(f8(Str::lit("the attribute ")), v10, f8(Str::lit(" is not declared by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                                }
                                                Some(v13) => {
                                                    let m3970 = f592(v1.clone(), match v13.clone() {
                                                        Adt26::C0(_, h3967, _, _) => {
                                                            h3967
                                                        }
                                                    }, f125({
                                                        let (_, h3968) = v12.clone();
                                                        h3968
                                                    })?, match v13 {
                                                        Adt26::C0(_, _, _, h3969) => {
                                                            h3969
                                                        }
                                                    })?;
                                                    match m3970 {
                                                        Ok(v15) => {
                                                            f597(v6, v1, v2, v3, v8, f682(v5, {
                                                                let (_, h3971) = v12;
                                                                h3971
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
                                        Ok(Err::<Adt36, Bytes>(f595(v11, v2)))
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if f375(v7.clone(), Str::lit("attributeGroup")) {
                        let m3976 = f376(v7.clone(), Str::lit("ref"));
                        match m3976 {
                            None => {
                                Ok(Err::<Adt36, Bytes>(f595(f8(Str::lit("an attribute group reference must have a ref (XSD 1.1 Part 1, section 3.6.2)")), v2)))
                            }
                            Some(v19) => {
                                let m3978 = f164(match v7 {
                                    Adt27::C0(_, _, _, _, h3977) => {
                                        h3977
                                    }
                                }, v19.clone())?;
                                match m3978 {
                                    Ok(v21) => {
                                        let m3979 = f681(v1.clone(), Str::lit("attributeGroup"), v21.clone());
                                        match m3979 {
                                            None => {
                                                Ok(Err::<Adt36, Bytes>(f595(f257(f8(Str::lit("the attribute group ")), v19, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                            }
                                            Some(v22) => {
                                                let m3981 = f593(match v22.clone() {
                                                    Adt26::C0(_, _, _, h3980) => {
                                                        h3980
                                                    }
                                                })?;
                                                match m3981 {
                                                    Ok(v24) => {
                                                        let m3984 = f597(v6, v1.clone(), f125({
                                                            let (_, h3982) = v21;
                                                            h3982
                                                        })?, match v22 {
                                                            Adt26::C0(_, h3983, _, _) => {
                                                                h3983
                                                            }
                                                        }, v24, v5)?;
                                                        match m3984 {
                                                            Ok(v26) => {
                                                                f597(v6, v1, v2, v3, v8, v26)
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
                                        Ok(Err::<Adt36, Bytes>(f595(v20, v2)))
                                    }
                                }
                            }
                        }
                    } else {
                        if f375(v7, Str::lit("anyAttribute")) {
                            f597(v6, v1, v2, v3, v8, f683(v5, Adt37::C0(f8(Str::lit("anyAttribute")), f684(), List::<Bytes>::nil())))
                        } else {
                            f597(v6, v1, v2, v3, v8, v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f598() -> Adt36 {
    Adt36::C0(List::<Adt37>::nil(), List::<Adt28>::nil(), 0u64, 0u64, 0u64)
}

pub fn f599(v0: List<Adt28>, v1: List<Bytes>, v2: Bytes) -> Result<List<Bytes>, Bytes> {
    let m3985 = v1;
    match m3985.uncons() {
        None => {
            Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let m3986 = f476(v0.clone(), v3.clone());
            match m3986 {
                None => {
                    Err::<List<Bytes>, Bytes>(f600(v3))
                }
                Some(v5) => {
                    let m3988 = match v5.clone() {
                        Adt28::C0(_, _, _, h3987) => {
                            h3987
                        }
                    };
                    match m3988.uncons() {
                        None => {
                            Err::<List<Bytes>, Bytes>(f600(v3))
                        }
                        Some((_, _)) => {
                            let m3989 = f599(v0, v4, v2.clone());
                            match m3989 {
                                Ok(v9) => {
                                    Ok::<List<Bytes>, Bytes>(List::cons(f685(v3, match v5 {
                                        Adt28::C0(_, _, _, h3990) => {
                                            h3990
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

pub fn f600(v0: Bytes) -> Bytes {
    f595(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)")), v0)
}

pub fn f601(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m3991 = v0;
    match m3991.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f122(v2.clone(), v1.clone()) {
                f601(v3, v1)
            } else {
                List::cons(v2, f601(v3, v1))
            }
        }
    }
}

pub fn f602(p3996: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m3995 = p3996;
    {
        let mut m3995 = m3995;
        loop {
            let step = {
                let v0 = m3995;
                let m3992 = v0;
                match m3992.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h3993, _) = v2;
                            h3993
                        }, v1.clone()) {
                            let (_, h3994) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h3994)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
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

pub fn f603(v0: Fn14, p4001: u64, p4002: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m4000 = (p4001, p4002);
    {
        let mut m4000 = m4000;
        loop {
            let step = {
                let (v1, v2) = m4000;
                let m3997 = v1;
                if m3997 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3997 - 1;
                    let m3999 = {
                        let c3998: Fn14 = v0.clone();
                        c3998.apply(v2.clone())?
                    };
                    match m3999 {
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
                Ok(next) => { m4000 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f604(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f686(v0, v1)
}

pub fn f605(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4047 = {
            let a4023 = {
                let a4011 = {
                    let a4005 = {
                        let a4003 = v1;
                        let a4004 = 128u8;
                        fixed_u8::bit_and(a4003, a4004)
                    };
                    let a4006 = 128u8;
                    if equal(a4005, a4006) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4012 = {
                    let a4009 = {
                        let a4007 = v1;
                        let a4008 = 64u8;
                        fixed_u8::bit_and(a4007, a4008)
                    };
                    let a4010 = 64u8;
                    if equal(a4009, a4010) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4011, a4012)?
            };
            let a4024 = {
                let a4021 = {
                    let a4015 = {
                        let a4013 = v1;
                        let a4014 = 32u8;
                        fixed_u8::bit_and(a4013, a4014)
                    };
                    let a4016 = 32u8;
                    if equal(a4015, a4016) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4022 = {
                    let a4019 = {
                        let a4017 = v1;
                        let a4018 = 16u8;
                        fixed_u8::bit_and(a4017, a4018)
                    };
                    let a4020 = 16u8;
                    if equal(a4019, a4020) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4021, a4022)?
            };
            nat_add(a4023, a4024)?
        };
        let a4048 = {
            let a4045 = {
                let a4033 = {
                    let a4027 = {
                        let a4025 = v1;
                        let a4026 = 8u8;
                        fixed_u8::bit_and(a4025, a4026)
                    };
                    let a4028 = 8u8;
                    if equal(a4027, a4028) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4034 = {
                    let a4031 = {
                        let a4029 = v1;
                        let a4030 = 4u8;
                        fixed_u8::bit_and(a4029, a4030)
                    };
                    let a4032 = 4u8;
                    if equal(a4031, a4032) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4033, a4034)?
            };
            let a4046 = {
                let a4043 = {
                    let a4037 = {
                        let a4035 = v1;
                        let a4036 = 2u8;
                        fixed_u8::bit_and(a4035, a4036)
                    };
                    let a4038 = 2u8;
                    if equal(a4037, a4038) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4044 = {
                    let a4041 = {
                        let a4039 = v1;
                        let a4040 = 1u8;
                        fixed_u8::bit_and(a4039, a4040)
                    };
                    let a4042 = 1u8;
                    if equal(a4041, a4042) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4043, a4044)?
            };
            nat_add(a4045, a4046)?
        };
        nat_add(a4047, a4048)?
    };
    let a4049 = 48u64;
    let a4050 = v2;
    if nat_le(a4049, a4050) {
        let a4051 = v2;
        let a4052 = 57u64;
        Ok(nat_le(a4051, a4052))
    } else {
        Ok(false)
    }
}

pub fn f606(v0: u64) -> u32 {
    let a4053 = v0;
    let a4054 = 32u64;
    if nat_lt(a4053, a4054) {
        let a4055 = v0;
        let a4056 = 16u64;
        if nat_lt(a4055, a4056) {
            let a4057 = v0;
            let a4058 = 8u64;
            if nat_lt(a4057, a4058) {
                let a4059 = v0;
                let a4060 = 4u64;
                if nat_lt(a4059, a4060) {
                    let a4061 = v0;
                    let a4062 = 2u64;
                    if nat_lt(a4061, a4062) {
                        let a4063 = v0;
                        let a4064 = 1u64;
                        if nat_lt(a4063, a4064) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a4065 = v0;
                        let a4066 = 3u64;
                        if nat_lt(a4065, a4066) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a4067 = v0;
                    let a4068 = 6u64;
                    if nat_lt(a4067, a4068) {
                        let a4069 = v0;
                        let a4070 = 5u64;
                        if nat_lt(a4069, a4070) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a4071 = v0;
                        let a4072 = 7u64;
                        if nat_lt(a4071, a4072) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a4073 = v0;
                let a4074 = 12u64;
                if nat_lt(a4073, a4074) {
                    let a4075 = v0;
                    let a4076 = 10u64;
                    if nat_lt(a4075, a4076) {
                        let a4077 = v0;
                        let a4078 = 9u64;
                        if nat_lt(a4077, a4078) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a4079 = v0;
                        let a4080 = 11u64;
                        if nat_lt(a4079, a4080) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a4081 = v0;
                    let a4082 = 14u64;
                    if nat_lt(a4081, a4082) {
                        let a4083 = v0;
                        let a4084 = 13u64;
                        if nat_lt(a4083, a4084) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a4085 = v0;
                        let a4086 = 15u64;
                        if nat_lt(a4085, a4086) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a4087 = v0;
            let a4088 = 24u64;
            if nat_lt(a4087, a4088) {
                let a4089 = v0;
                let a4090 = 20u64;
                if nat_lt(a4089, a4090) {
                    let a4091 = v0;
                    let a4092 = 18u64;
                    if nat_lt(a4091, a4092) {
                        let a4093 = v0;
                        let a4094 = 17u64;
                        if nat_lt(a4093, a4094) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a4095 = v0;
                        let a4096 = 19u64;
                        if nat_lt(a4095, a4096) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a4097 = v0;
                    let a4098 = 22u64;
                    if nat_lt(a4097, a4098) {
                        let a4099 = v0;
                        let a4100 = 21u64;
                        if nat_lt(a4099, a4100) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a4101 = v0;
                        let a4102 = 23u64;
                        if nat_lt(a4101, a4102) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a4103 = v0;
                let a4104 = 28u64;
                if nat_lt(a4103, a4104) {
                    let a4105 = v0;
                    let a4106 = 26u64;
                    if nat_lt(a4105, a4106) {
                        let a4107 = v0;
                        let a4108 = 25u64;
                        if nat_lt(a4107, a4108) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a4109 = v0;
                        let a4110 = 27u64;
                        if nat_lt(a4109, a4110) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a4111 = v0;
                    let a4112 = 30u64;
                    if nat_lt(a4111, a4112) {
                        let a4113 = v0;
                        let a4114 = 29u64;
                        if nat_lt(a4113, a4114) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a4115 = v0;
                        let a4116 = 31u64;
                        if nat_lt(a4115, a4116) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a4117 = v0;
        let a4118 = 48u64;
        if nat_lt(a4117, a4118) {
            let a4119 = v0;
            let a4120 = 40u64;
            if nat_lt(a4119, a4120) {
                let a4121 = v0;
                let a4122 = 36u64;
                if nat_lt(a4121, a4122) {
                    let a4123 = v0;
                    let a4124 = 34u64;
                    if nat_lt(a4123, a4124) {
                        let a4125 = v0;
                        let a4126 = 33u64;
                        if nat_lt(a4125, a4126) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a4127 = v0;
                        let a4128 = 35u64;
                        if nat_lt(a4127, a4128) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a4129 = v0;
                    let a4130 = 38u64;
                    if nat_lt(a4129, a4130) {
                        let a4131 = v0;
                        let a4132 = 37u64;
                        if nat_lt(a4131, a4132) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a4133 = v0;
                        let a4134 = 39u64;
                        if nat_lt(a4133, a4134) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a4135 = v0;
                let a4136 = 44u64;
                if nat_lt(a4135, a4136) {
                    let a4137 = v0;
                    let a4138 = 42u64;
                    if nat_lt(a4137, a4138) {
                        let a4139 = v0;
                        let a4140 = 41u64;
                        if nat_lt(a4139, a4140) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a4141 = v0;
                        let a4142 = 43u64;
                        if nat_lt(a4141, a4142) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a4143 = v0;
                    let a4144 = 46u64;
                    if nat_lt(a4143, a4144) {
                        let a4145 = v0;
                        let a4146 = 45u64;
                        if nat_lt(a4145, a4146) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a4147 = v0;
                        let a4148 = 47u64;
                        if nat_lt(a4147, a4148) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a4149 = v0;
            let a4150 = 56u64;
            if nat_lt(a4149, a4150) {
                let a4151 = v0;
                let a4152 = 52u64;
                if nat_lt(a4151, a4152) {
                    let a4153 = v0;
                    let a4154 = 50u64;
                    if nat_lt(a4153, a4154) {
                        let a4155 = v0;
                        let a4156 = 49u64;
                        if nat_lt(a4155, a4156) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a4157 = v0;
                        let a4158 = 51u64;
                        if nat_lt(a4157, a4158) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a4159 = v0;
                    let a4160 = 54u64;
                    if nat_lt(a4159, a4160) {
                        let a4161 = v0;
                        let a4162 = 53u64;
                        if nat_lt(a4161, a4162) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a4163 = v0;
                        let a4164 = 55u64;
                        if nat_lt(a4163, a4164) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a4165 = v0;
                let a4166 = 60u64;
                if nat_lt(a4165, a4166) {
                    let a4167 = v0;
                    let a4168 = 58u64;
                    if nat_lt(a4167, a4168) {
                        let a4169 = v0;
                        let a4170 = 57u64;
                        if nat_lt(a4169, a4170) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a4171 = v0;
                        let a4172 = 59u64;
                        if nat_lt(a4171, a4172) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a4173 = v0;
                    let a4174 = 62u64;
                    if nat_lt(a4173, a4174) {
                        let a4175 = v0;
                        let a4176 = 61u64;
                        if nat_lt(a4175, a4176) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a4177 = v0;
                        let a4178 = 63u64;
                        if nat_lt(a4177, a4178) {
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

pub fn f607(v0: u32) -> u32 {
    let a4181 = {
        let a4179 = f687(v0, 6u32, 26u32);
        let a4180 = f687(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a4179, a4180)
    };
    let a4182 = f687(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a4181, a4182)
}

pub fn f608(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4188 = {
        let a4183 = v0;
        let a4184 = v1;
        fixed_u32::bit_and(a4183, a4184)
    };
    let a4189 = {
        let a4186 = {
            let a4185 = v0;
            fixed_u32::bit_not(a4185)
        };
        let a4187 = v2;
        fixed_u32::bit_and(a4186, a4187)
    };
    fixed_u32::bit_xor(a4188, a4189)
}

pub fn f609(v0: u32) -> u32 {
    let a4192 = {
        let a4190 = f687(v0, 2u32, 30u32);
        let a4191 = f687(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a4190, a4191)
    };
    let a4193 = f687(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a4192, a4193)
}

pub fn f610(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4202 = {
        let a4198 = {
            let a4194 = v0;
            let a4195 = v1;
            fixed_u32::bit_and(a4194, a4195)
        };
        let a4199 = {
            let a4196 = v0;
            let a4197 = v2;
            fixed_u32::bit_and(a4196, a4197)
        };
        fixed_u32::bit_xor(a4198, a4199)
    };
    let a4203 = {
        let a4200 = v1;
        let a4201 = v2;
        fixed_u32::bit_and(a4200, a4201)
    };
    fixed_u32::bit_xor(a4202, a4203)
}

pub fn f611(v0: Adt29) -> Adt29 {
    Adt29::C0(match v0.clone() {
        Adt29::C0(_, h4204, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4204
        }
    }, match v0.clone() {
        Adt29::C0(_, _, h4205, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4205
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, h4206, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4206
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, h4207, _, _, _, _, _, _, _, _, _, _, _) => {
            h4207
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, h4208, _, _, _, _, _, _, _, _, _, _) => {
            h4208
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, h4209, _, _, _, _, _, _, _, _, _) => {
            h4209
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, h4210, _, _, _, _, _, _, _, _) => {
            h4210
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, h4211, _, _, _, _, _, _, _) => {
            h4211
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, _, h4212, _, _, _, _, _, _) => {
            h4212
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, _, _, h4213, _, _, _, _, _) => {
            h4213
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, _, _, _, h4214, _, _, _, _) => {
            h4214
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, _, _, _, _, h4215, _, _, _) => {
            h4215
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h4216, _, _) => {
            h4216
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4217, _) => {
            h4217
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h4218) => {
            h4218
        }
    }, f389(f688(match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4219, _) => {
            h4219
        }
    }), f389(match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, _, h4220, _, _, _, _, _, _) => {
            h4220
        }
    }, f389(f689(match v0.clone() {
        Adt29::C0(_, h4221, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4221
        }
    }), match v0 {
        Adt29::C0(h4222, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4222
        }
    }))))
}

pub fn f612(v0: Fn15, p4226: u64, p4227: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m4225 = (p4226, p4227);
    {
        let mut m4225 = m4225;
        loop {
            let step = {
                let (v1, v2) = m4225;
                let m4223 = v1;
                if m4223 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m4223 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c4224: Fn15 = v0.clone();
                        c4224.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m4225 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f613(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h4228, _) = v0;
        h4228
    };
    let v2: u32 = {
        let (h4229, _) = {
            let (_, h4230) = v0;
            h4230
        };
        h4229
    };
    let v3: u32 = {
        let (_, h4231) = {
            let (_, h4232) = v0;
            h4232
        };
        h4231
    };
    ({
        let a4233 = v1;
        let a4234 = 2u64;
        let a4235 = 0u64;
        nat_quot(a4233, a4234, a4235)
    }, (f614(v2, 1u32), {
        let a4239 = {
            let a4236 = v1;
            let a4237 = 2u64;
            let a4238 = 0u64;
            nat_rem(a4236, a4237, a4238)
        };
        let a4240 = 1u64;
        if nat_eq(a4239, a4240) {
            let a4241 = v3;
            let a4242 = v2;
            fixed_u32::bit_or(a4241, a4242)
        } else {
            v3
        }
    }))
}

pub fn f614(v0: u32, v1: u32) -> u32 {
    let m4245 = {
        let a4243 = v0;
        let a4244 = v1;
        fixed_u32::shift_left(a4243, a4244)
    };
    match m4245 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f615(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4290 = {
            let a4266 = {
                let a4254 = {
                    let a4248 = {
                        let a4246 = v1;
                        let a4247 = 128u8;
                        fixed_u8::bit_and(a4246, a4247)
                    };
                    let a4249 = 128u8;
                    if equal(a4248, a4249) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4255 = {
                    let a4252 = {
                        let a4250 = v1;
                        let a4251 = 64u8;
                        fixed_u8::bit_and(a4250, a4251)
                    };
                    let a4253 = 64u8;
                    if equal(a4252, a4253) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4254, a4255)?
            };
            let a4267 = {
                let a4264 = {
                    let a4258 = {
                        let a4256 = v1;
                        let a4257 = 32u8;
                        fixed_u8::bit_and(a4256, a4257)
                    };
                    let a4259 = 32u8;
                    if equal(a4258, a4259) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4265 = {
                    let a4262 = {
                        let a4260 = v1;
                        let a4261 = 16u8;
                        fixed_u8::bit_and(a4260, a4261)
                    };
                    let a4263 = 16u8;
                    if equal(a4262, a4263) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4264, a4265)?
            };
            nat_add(a4266, a4267)?
        };
        let a4291 = {
            let a4288 = {
                let a4276 = {
                    let a4270 = {
                        let a4268 = v1;
                        let a4269 = 8u8;
                        fixed_u8::bit_and(a4268, a4269)
                    };
                    let a4271 = 8u8;
                    if equal(a4270, a4271) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4277 = {
                    let a4274 = {
                        let a4272 = v1;
                        let a4273 = 4u8;
                        fixed_u8::bit_and(a4272, a4273)
                    };
                    let a4275 = 4u8;
                    if equal(a4274, a4275) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4276, a4277)?
            };
            let a4289 = {
                let a4286 = {
                    let a4280 = {
                        let a4278 = v1;
                        let a4279 = 2u8;
                        fixed_u8::bit_and(a4278, a4279)
                    };
                    let a4281 = 2u8;
                    if equal(a4280, a4281) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4287 = {
                    let a4284 = {
                        let a4282 = v1;
                        let a4283 = 1u8;
                        fixed_u8::bit_and(a4282, a4283)
                    };
                    let a4285 = 1u8;
                    if equal(a4284, a4285) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4286, a4287)?
            };
            nat_add(a4288, a4289)?
        };
        nat_add(a4290, a4291)?
    };
    let a4298 = f209(f690(), {
        let a4292 = v2;
        let a4293 = 16u64;
        let a4294 = 0u64;
        nat_quot(a4292, a4293, a4294)
    });
    let a4299 = f209(f690(), {
        let a4295 = v2;
        let a4296 = 16u64;
        let a4297 = 0u64;
        nat_rem(a4295, a4296, a4297)
    });
    Ok(append_bytes(a4298, a4299))
}

pub fn f616(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f691(Fn21::F692, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f617(v0: Fn16, p4303: List<List<(Bytes, Bytes)>>, p4304: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m4302 = (p4303, p4304);
    {
        let mut m4302 = m4302;
        loop {
            let step = {
                let (v1, v2) = m4302;
                let m4300 = v2;
                match m4300.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c4301: Fn16 = v0.clone();
                            c4301.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m4302 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f618(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f619(v0: Fn17, p4309: u64, p4310: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m4308 = (p4309, p4310);
    {
        let mut m4308 = m4308;
        loop {
            let step = {
                let (v1, v2) = m4308;
                let m4305 = v1;
                if m4305 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4305 - 1;
                    let m4307 = {
                        let c4306: Fn17 = v0.clone();
                        c4306.apply(v2.clone())?
                    };
                    match m4307 {
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
                Ok(next) => { m4308 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f620(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f693(v0)
}

pub fn f621(v0: Fn18, p4314: List<List<(Bytes, ())>>, p4315: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m4313 = (p4314, p4315);
    {
        let mut m4313 = m4313;
        loop {
            let step = {
                let (v1, v2) = m4313;
                let m4311 = v2;
                match m4311.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c4312: Fn18 = v0.clone();
                            c4312.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m4313 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f622(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f623(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a4317 = {
        let a4316 = v0.clone();
        length_list(a4316)
    };
    let a4318 = 1u64;
    if nat_le(a4317, a4318) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f694(v0)?))
    }
}

pub fn f624(v0: Fn19, p4322: List<(Bytes, ())>, p4323: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m4321 = (p4322, p4323);
    {
        let mut m4321 = m4321;
        loop {
            let step = {
                let (v1, v2) = m4321;
                let m4319 = v2;
                match m4319.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c4320: Fn19 = v0.clone();
                            c4320.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m4321 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f625(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f626(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a4324 = 0u64;
    let a4325 = v1;
    if if nat_lt(a4324, a4325) {
        f208({
            let a4328 = f206(v0, {
                let a4326 = v1;
                let a4327 = 1u64;
                nat_sub(a4326, a4327)
            })?;
            let a4329 = 10u64;
            nat_eq(a4328, a4329)
        })
    } else {
        false
    } {
        Ok(Some({
            let a4330 = v1;
            let a4331 = 1u64;
            nat_sub(a4330, a4331)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f627(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f300(f206(v0, v1)?) {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a4332 = v1;
            let a4333 = 1u64;
            nat_add(a4332, a4333)?
        }))
    }
}

pub fn f628(v0: Bytes, v1: u64, v2: u64) -> R<Adt38> {
    let v3: (u64, u64) = {
        let (h4334, _) = f270(Fn6::F695(v2, v0.clone()), {
            let a4336 = {
                let a4335 = v0.clone();
                length_bytes(a4335)
            };
            let a4337 = 1u64;
            nat_add(a4336, a4337)?
        }, (v1, 0u64))?;
        h4334
    };
    let v4: u64 = f629(v0.clone(), {
        let (h4338, _) = v3;
        h4338
    }, v2)?;
    let a4339 = f206(v0, v4)?;
    let a4340 = 9u64;
    if if nat_eq(a4339, a4340) {
        let a4344 = {
            let a4342 = v4;
            let a4343 = {
                let (h4341, _) = v3;
                h4341
            };
            nat_sub(a4342, a4343)
        };
        let a4345 = v2;
        nat_lt(a4344, a4345)
    } else {
        false
    } {
        Ok(Adt38::C1(f137(Str::lit("a tab cannot indent a block scalar line (YAML 1.2.2 [63], [70])"), v4)?))
    } else {
        Ok(Adt38::C0({
            let (_, h4346) = v3;
            h4346
        }, {
            let (h4347, _) = v3;
            h4347
        }))
    }
}

pub fn f629(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h4348, _) = f201(Fn5::F696(v2, v0.clone(), v1), {
        let a4350 = {
            let a4349 = v0;
            length_bytes(a4349)
        };
        let a4351 = 1u64;
        nat_add(a4350, a4351)?
    }, v1)?;
    Ok(h4348)
}

pub fn f630(v0: Bytes, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = f629(v0.clone(), v1, v2)?;
    let a4354 = {
        let a4352 = v3;
        let a4353 = v1;
        nat_sub(a4352, a4353)
    };
    let a4355 = v2;
    if nat_eq(a4354, a4355) {
        Ok(f208({
            let a4356 = f206(v0, v3)?;
            let a4357 = 256u64;
            nat_eq(a4356, a4357)
        }))
    } else {
        Ok(false)
    }
}

pub fn f631(v0: Bytes, p4367: u64, v2: u64, v3: bool, p4368: u64, p4369: Bytes, p4370: u64) -> R<Adt39> {
    let m4366 = (p4367, (p4368, (p4369, p4370)));
    {
        let mut m4366 = m4366;
        loop {
            let step = {
                let (v1, (v4, (v5, v6))) = m4366;
                let m4358 = v4;
                if m4358 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<Adt39>>(Ok(Adt39::C1(f137(Str::lit("a block scalar has more lines than the input"), v1)?)))
                } else {
                    let v7 = m4358 - 1;
                    let v8: u64 = f629(v0.clone(), v1, v2)?;
                    let v9: bool = f208(f299(f206(v0.clone(), v8)?));
                    let v10: u64 = f409(v0.clone(), v8)?;
                    let v11: Bytes = f7(List::cons(v5, List::cons(f526(v6), List::cons(f316(v0.clone(), v8, v10), List::<Bytes>::nil()))));
                    let m4359 = f628(v0.clone(), f303(v0.clone(), v10)?, v2)?;
                    match m4359 {
                        Adt38::C0(v13, v14) => {
                            let v15: u64 = v14;
                            let v16: u64 = f629(v0.clone(), v15, v2)?;
                            if f630(v0.clone(), v15, v2)? {
                                Ok::<(u64, (u64, (Bytes, u64))), R<Adt39>>((v15, (v7, ({
                                    let a4362 = v11;
                                    let a4363 = if if if v3 {
                                        v9
                                    } else {
                                        false
                                    } {
                                        f208(f299(f206(v0.clone(), v16)?))
                                    } else {
                                        false
                                    } {
                                        let a4360 = v13;
                                        let a4361 = 0u64;
                                        if nat_eq(a4360, a4361) {
                                            Bytes::lit(&[0x20])
                                        } else {
                                            Bytes::lit(&[])
                                        }
                                    } else {
                                        Bytes::lit(&[0x0a])
                                    };
                                    append_bytes(a4362, a4363)
                                }, v13))))
                            } else {
                                Err::<(u64, (u64, (Bytes, u64))), R<Adt39>>(Ok(Adt39::C0(v11, v13, {
                                    let a4364 = f206(v0.clone(), v16)?;
                                    let a4365 = 256u64;
                                    if nat_eq(a4364, a4365) {
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
                Ok(next) => { m4366 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f632(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f304(v0.clone(), {
        let (h4371, _) = v1;
        h4371
    })?;
    let a4372 = f206(v0, v2)?;
    let a4373 = 10u64;
    if nat_eq(a4372, a4373) {
        Ok(Some(({
            let a4374 = v2;
            let a4375 = 1u64;
            nat_add(a4374, a4375)?
        }, f528({
            let (_, h4376) = v1;
            h4376
        }, {
            let a4378 = v2;
            let a4379 = {
                let (h4377, _) = v1;
                h4377
            };
            nat_sub(a4378, a4379)
        }))))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f633(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f697(v0, v1)
}

pub fn f634(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt22> {
    let v4: (u64, u64) = f636(v0.clone(), v1, 2u64)?;
    let a4382 = {
        let (h4380, _) = v4;
        h4380
    };
    let a4383 = {
        let a4381 = v0.clone();
        length_bytes(a4381)
    };
    if nat_eq(a4382, a4383) {
        if f184(false, {
            let (_, h4384) = v4;
            h4384
        }) {
            Ok(Adt22::C0(Adt2::C3(false, {
                let (_, h4385) = v4;
                h4385
            }), v3))
        } else {
            Ok(Adt22::C1(f635(v2)?))
        }
    } else {
        Ok(Adt22::C0(Adt2::C5(v0), v3))
    }
}

pub fn f635(v0: u64) -> R<Bytes> {
    f137(Str::lit("a number outside the range Studio.Value holds is not read (YAML 1.2.2 section 10.3.2)"), v0)
}

pub fn f636(v0: Bytes, v1: u64, v2: u64) -> R<(u64, u64)> {
    let (h4386, _) = f270(Fn6::F698(v1, v0.clone()), {
        let a4388 = {
            let a4387 = v0;
            length_bytes(a4387)
        };
        let a4389 = 1u64;
        nat_add(a4388, a4389)?
    }, (v2, 0u64))?;
    Ok(h4386)
}

pub fn f637(v0: u64) -> R<Option<u64>> {
    let v1: u64 = f315(f8(Str::lit("abtnvfr")), v0)?;
    let a4390 = v1;
    let a4391 = 7u64;
    if nat_lt(a4390, a4391) {
        Ok(Some({
            let a4392 = v1;
            let a4393 = 7u64;
            nat_add(a4392, a4393)?
        }))
    } else {
        let a4394 = v0;
        let a4395 = 48u64;
        if nat_eq(a4394, a4395) {
            Ok(Some(0u64))
        } else {
            let a4396 = v0;
            let a4397 = 9u64;
            if nat_eq(a4396, a4397) {
                Ok(Some(9u64))
            } else {
                let a4398 = v0;
                let a4399 = 101u64;
                if nat_eq(a4398, a4399) {
                    Ok(Some(27u64))
                } else {
                    let a4400 = v0;
                    let a4401 = 78u64;
                    if nat_eq(a4400, a4401) {
                        Ok(Some(133u64))
                    } else {
                        let a4402 = v0;
                        let a4403 = 95u64;
                        if nat_eq(a4402, a4403) {
                            Ok(Some(160u64))
                        } else {
                            let a4404 = v0;
                            let a4405 = 76u64;
                            if nat_eq(a4404, a4405) {
                                Ok(Some(8232u64))
                            } else {
                                let a4406 = v0;
                                let a4407 = 80u64;
                                if nat_eq(a4406, a4407) {
                                    Ok(Some(8233u64))
                                } else {
                                    let a4408 = f315(f8(Str::lit(" \"/\\")), v0)?;
                                    let a4409 = 4u64;
                                    if nat_lt(a4408, a4409) {
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

pub fn f638(v0: u64) -> R<Bytes> {
    f137(Str::lit("an implicit key must be on one line (YAML 1.2.2 [154])"), v0)
}

pub fn f639(v0: Bytes, p4424: u64, v2: u64, p4425: u64, p4426: u64) -> R<Adt38> {
    let m4423 = (p4424, (p4425, p4426));
    {
        let mut m4423 = m4423;
        loop {
            let step = {
                let (v1, (v3, v4)) = m4423;
                let m4410 = v3;
                if m4410 == 0 {
                    Err::<(u64, (u64, u64)), R<Adt38>>(Ok(Adt38::C1(f137(Str::lit("the input ends inside a scalar"), v1)?)))
                } else {
                    let v5 = m4410 - 1;
                    let v6: u64 = {
                        let a4411 = v1;
                        let a4412 = 1u64;
                        nat_add(a4411, a4412)?
                    };
                    let v7: u64 = f304(v0.clone(), v6)?;
                    let v8: u64 = f302(v0.clone(), v7)?;
                    let v9: u64 = f206(v0.clone(), v8)?;
                    if f204(v0.clone(), v6)? {
                        Err::<(u64, (u64, u64)), R<Adt38>>(Ok(Adt38::C1(f137(Str::lit("a document marker cannot appear inside a scalar (YAML 1.2.2 [206])"), v6)?)))
                    } else {
                        let a4413 = v9;
                        let a4414 = 10u64;
                        if nat_eq(a4413, a4414) {
                            Ok::<(u64, (u64, u64)), R<Adt38>>((v8, (v5, {
                                let a4415 = v4;
                                let a4416 = 1u64;
                                nat_add(a4415, a4416)?
                            })))
                        } else {
                            let a4417 = v9;
                            let a4418 = 256u64;
                            if nat_eq(a4417, a4418) {
                                Err::<(u64, (u64, u64)), R<Adt38>>(Ok(Adt38::C1(f137(Str::lit("the input ends inside a quoted scalar (YAML 1.2.2 [109], [120])"), v8)?)))
                            } else {
                                let a4421 = {
                                    let a4419 = v7;
                                    let a4420 = v6;
                                    nat_sub(a4419, a4420)
                                };
                                let a4422 = v2;
                                if nat_lt(a4421, a4422) {
                                    Err::<(u64, (u64, u64)), R<Adt38>>(Ok(Adt38::C1(f137(Str::lit("a continuation line of a scalar is indented less than its node (YAML 1.2.2 [69])"), v7)?)))
                                } else {
                                    Err::<(u64, (u64, u64)), R<Adt38>>(Ok(Adt38::C0(v4, v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4423 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f640(v0: Bytes, p4436: u64, p4437: u64, p4438: u64) -> R<Option<u64>> {
    let m4435 = (p4436, (p4437, p4438));
    {
        let mut m4435 = m4435;
        loop {
            let step = {
                let (v1, (v2, v3)) = m4435;
                let m4427 = v2;
                if m4427 == 0 {
                    Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(Some(v3)))
                } else {
                    let v4 = m4427 - 1;
                    let m4428 = f699(f206(v0.clone(), v1)?);
                    match m4428 {
                        None => {
                            Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(None::<u64>))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, u64)), R<Option<u64>>>(({
                                let a4429 = v1;
                                let a4430 = 1u64;
                                nat_add(a4429, a4430)?
                            }, (v4, {
                                let a4433 = {
                                    let a4431 = v3;
                                    let a4432 = 16u64;
                                    nat_mul(a4431, a4432)?
                                };
                                let a4434 = v5;
                                nat_add(a4433, a4434)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4435 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f641(v0: Bytes, v1: u64) -> R<u64> {
    let (h4439, _) = f201(Fn5::F700(v0.clone()), {
        let a4441 = {
            let a4440 = v0;
            length_bytes(a4440)
        };
        let a4442 = 1u64;
        nat_add(a4441, a4442)?
    }, v1)?;
    Ok(h4439)
}

pub fn f642(v0: Bytes, v1: u64) -> R<u64> {
    let (h4443, _) = f201(Fn5::F701(v0.clone()), {
        let a4445 = {
            let a4444 = v0;
            length_bytes(a4444)
        };
        let a4446 = 1u64;
        nat_add(a4445, a4446)?
    }, v1)?;
    Ok(h4443)
}

pub fn f643(v0: Fn20, p4451: u64, p4452: (u64, bool)) -> R<((u64, bool), bool)> {
    let m4450 = (p4451, p4452);
    {
        let mut m4450 = m4450;
        loop {
            let step = {
                let (v1, v2) = m4450;
                let m4447 = v1;
                if m4447 == 0 {
                    Err::<(u64, (u64, bool)), R<((u64, bool), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4447 - 1;
                    let m4449 = {
                        let c4448: Fn20 = v0.clone();
                        c4448.apply(v2)?
                    };
                    match m4449 {
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
                Ok(next) => { m4450 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f644(v0: u64, v1: Bytes, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    f702(v1, v0, v2)
}

pub fn f645(v0: Bytes, v1: u64, v2: u64) -> R<Adt32> {
    let v3: u64 = f206(v0.clone(), v1)?;
    let a4453 = v3;
    let a4454 = 34u64;
    if if nat_eq(a4453, a4454) {
        true
    } else {
        let a4455 = v3;
        let a4456 = 39u64;
        nat_eq(a4455, a4456)
    } {
        f414(v0, v1, v2, false)
    } else {
        let m4457 = f307(v0.clone(), v1)?;
        match m4457 {
            None => {
                let a4458 = v3;
                let a4459 = 91u64;
                if if nat_eq(a4458, a4459) {
                    true
                } else {
                    let a4460 = v3;
                    let a4461 = 123u64;
                    nat_eq(a4460, a4461)
                } {
                    Ok(Adt32::C1(f137(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [142])"), v1)?))
                } else {
                    if f416(v0.clone(), v1, true)? {
                        let v5: u64 = f417(v0.clone(), v1, true)?;
                        let v6: u64 = f302(v0.clone(), v5)?;
                        let a4462 = f206(v0.clone(), v6)?;
                        let a4463 = 10u64;
                        if nat_eq(a4462, a4463) {
                            let m4464 = f538(v0.clone(), v6, v2)?;
                            match m4464 {
                                Ok(v8) => {
                                    let v9: u64 = f206(v0.clone(), v8)?;
                                    let a4465 = v9;
                                    let a4466 = 58u64;
                                    if if if nat_eq(a4465, a4466) {
                                        true
                                    } else {
                                        let a4467 = v9;
                                        let a4468 = 44u64;
                                        nat_eq(a4467, a4468)
                                    } {
                                        true
                                    } else {
                                        let a4469 = v9;
                                        let a4470 = 125u64;
                                        nat_eq(a4469, a4470)
                                    } {
                                        Ok(Adt32::C0(f316(v0, v1, v5), v5))
                                    } else {
                                        Ok(Adt32::C1(f137(Str::lit("a multi-line plain scalar inside a flow collection is not read (YAML 1.2.2 [135])"), v6)?))
                                    }
                                }
                                Err(v7) => {
                                    Ok(Adt32::C1(v7))
                                }
                            }
                        } else {
                            Ok(Adt32::C0(f316(v0, v1, v5), v5))
                        }
                    } else {
                        Ok(Adt32::C1(f137(Str::lit("a flow mapping key must be a scalar (YAML 1.2.2 [142])"), v1)?))
                    }
                }
            }
            Some(v4) => {
                Ok(Adt32::C1(v4))
            }
        }
    }
}

pub fn f646(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = {
        let (h4471, _) = v2;
        h4471
    };
    let v4: u64 = f206(v0.clone(), v3)?;
    if f300(v4) {
        Ok(None::<(u64, u64)>)
    } else {
        if f299(v4) {
            Ok(Some(({
                let a4472 = v3;
                let a4473 = 1u64;
                nat_add(a4472, a4473)?
            }, {
                let (_, h4474) = v2;
                h4474
            })))
        } else {
            let a4475 = v4;
            let a4476 = 58u64;
            if if nat_eq(a4475, a4476) {
                f208(f411(f206(v0.clone(), {
                    let a4477 = v3;
                    let a4478 = 1u64;
                    nat_add(a4477, a4478)?
                })?, v1))
            } else {
                false
            } {
                Ok(None::<(u64, u64)>)
            } else {
                let a4479 = v4;
                let a4480 = 35u64;
                if if nat_eq(a4479, a4480) {
                    f299(f206(v0, {
                        let a4481 = v3;
                        let a4482 = 1u64;
                        nat_sub(a4481, a4482)
                    })?)
                } else {
                    false
                } {
                    Ok(None::<(u64, u64)>)
                } else {
                    if if v1 {
                        f521(v4)
                    } else {
                        false
                    } {
                        Ok(None::<(u64, u64)>)
                    } else {
                        Ok(Some(({
                            let a4483 = v3;
                            let a4484 = 1u64;
                            nat_add(a4483, a4484)?
                        }, {
                            let a4485 = v3;
                            let a4486 = 1u64;
                            nat_add(a4485, a4486)?
                        })))
                    }
                }
            }
        }
    }
}

pub fn f647(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f703(v0, v1)
}

pub fn f648(v0: Bytes, v1: bool, v2: u64) -> R<Option<u64>> {
    let v3: u64 = f206(v0, v2)?;
    if if if f299(v3) {
        true
    } else {
        f300(v3)
    } {
        true
    } else {
        if v1 {
            let a4487 = v3;
            let a4488 = 62u64;
            nat_eq(a4487, a4488)
        } else {
            f521(v3)
        }
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a4489 = v2;
            let a4490 = 1u64;
            nat_add(a4489, a4490)?
        }))
    }
}

pub fn f649(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    if f704(v1, f160(v0, v2)?) {
        Ok(Some({
            let a4491 = v2;
            let a4492 = 1u64;
            nat_add(a4491, a4492)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f650(p4495: List<Adt14>, p4496: List<Adt14>) -> List<Adt14> {
    let m4494 = (p4495, p4496);
    {
        let mut m4494 = m4494;
        loop {
            let step = {
                let (v0, v1) = m4494;
                let m4493 = v0;
                match m4493.uncons() {
                    None => {
                        Err::<(List<Adt14>, List<Adt14>), List<Adt14>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt14>, List<Adt14>), List<Adt14>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m4494 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f651(v0: Bytes, p4501: List<(Bytes, Adt15)>) -> Option<Adt15> {
    let m4500 = p4501;
    {
        let mut m4500 = m4500;
        loop {
            let step = {
                let v1 = m4500;
                let m4497 = v1;
                match m4497.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt15)>, Option<Adt15>>(None::<Adt15>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h4498, _) = v2;
                            h4498
                        }, v0.clone()) {
                            Err::<List<(Bytes, Adt15)>, Option<Adt15>>(Some({
                                let (_, h4499) = v2;
                                h4499
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt15)>, Option<Adt15>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4500 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f652() -> List<(Bytes, Adt15)> {
    List::cons((f8(Str::lit("boolean")), f441(f8(Str::lit("Bool")))), List::cons((f8(Str::lit("byte")), f441(f8(Str::lit("Int")))), List::cons((f8(Str::lit("octet")), f441(f8(Str::lit("Nat")))), List::cons((f8(Str::lit("bigint")), f441(f8(Str::lit("Int")))), List::cons((f8(Str::lit("float")), f441(f586())), List::cons((f8(Str::lit("double")), f441(f586())), List::cons((f8(Str::lit("DOMString")), f441(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("USVString")), f441(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ByteString")), f441(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ArrayBuffer")), f441(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("SharedArrayBuffer")), f441(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("DataView")), f441(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("Int8Array")), f441(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int16Array")), f441(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int32Array")), f441(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("BigInt64Array")), f441(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Uint8Array")), f441(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint16Array")), f441(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint32Array")), f441(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint8ClampedArray")), f441(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("BigUint64Array")), f441(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Float16Array")), f564(f8(Str::lit("Float16Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("Float32Array")), f564(f8(Str::lit("Float32Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("Float64Array")), f564(f8(Str::lit("Float64Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("any")), f564(f8(Str::lit("any is not translated, because it is the union of every Web IDL type and has no one LexLean type (Web IDL 2.13.1)")))), List::cons((f8(Str::lit("object")), f564(f8(Str::lit("object is not translated, because its values are references to JavaScript objects of any shape, which no LexLean type holds (Web IDL 2.13.20)")))), List::cons((f8(Str::lit("symbol")), f564(f8(Str::lit("symbol is not translated, because a symbol value has identity but no content that a LexLean type holds (Web IDL 2.13.21)")))), List::cons((f8(Str::lit("undefined")), f564(f8(Str::lit("undefined is not translated, because it is the type of the absence of a value (Web IDL 2.13.2)")))), List::<(Bytes, Adt15)>::nil()))))))))))))))))))))))))))))
}

pub fn f653() -> Bytes {
    f8(Str::lit("a union type is translated only as the whole body of a typedef, whose identifier names its inductive (Web IDL 2.11 and 2.13.32)"))
}

pub fn f654(v0: u64) -> Bytes {
    f46({
        let a4502 = Str::lit("0123456789ABCDEF");
        utf8_encode(a4502)
    }, v0, 1u64)
}

pub fn f655(p4513: u64, v1: Bytes, v2: Bytes, p4514: u64) -> R<bool> {
    let m4512 = (p4513, p4514);
    {
        let mut m4512 = m4512;
        loop {
            let step = {
                let (v0, v3) = m4512;
                let m4503 = v0;
                if m4503 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(false))
                } else {
                    let v4 = m4503 - 1;
                    let v5: u64 = f705(v1.clone(), v3)?;
                    let v6: u64 = f705(v2.clone(), v3)?;
                    let a4504 = v5;
                    let a4505 = v6;
                    if nat_lt(a4504, a4505) {
                        Err::<(u64, u64), R<bool>>(Ok(true))
                    } else {
                        let a4506 = v6;
                        let a4507 = v5;
                        if nat_lt(a4506, a4507) {
                            Err::<(u64, u64), R<bool>>(Ok(false))
                        } else {
                            let a4508 = v5;
                            let a4509 = 0u64;
                            if nat_eq(a4508, a4509) {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            } else {
                                Ok::<(u64, u64), R<bool>>((v4, {
                                    let a4510 = v3;
                                    let a4511 = 1u64;
                                    nat_add(a4510, a4511)?
                                }))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4512 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f656(p4527: u64, v1: Bytes, p4528: u64, p4529: Bytes) -> R<Option<Bytes>> {
    let m4526 = (p4527, (p4528, p4529));
    {
        let mut m4526 = m4526;
        loop {
            let step = {
                let (v0, (v2, v3)) = m4526;
                let m4515 = v0;
                if m4515 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m4515 - 1;
                    let a4517 = {
                        let a4516 = v1.clone();
                        length_bytes(a4516)
                    };
                    let a4518 = v2;
                    if nat_le(a4517, a4518) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m4519 = f404(v1.clone(), v2)?;
                        match m4519 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a4521 = v2;
                                    let a4522 = {
                                        let (_, h4520) = v5;
                                        h4520
                                    };
                                    nat_add(a4521, a4522)?
                                }, {
                                    let a4524 = v3;
                                    let a4525 = f706(v1.clone(), v2, {
                                        let (h4523, _) = v5;
                                        h4523
                                    });
                                    append_bytes(a4524, a4525)
                                })))
                            }
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

pub fn f657(p4539: u64, v1: Bytes, p4540: u64) -> R<bool> {
    let m4538 = (p4539, p4540);
    {
        let mut m4538 = m4538;
        loop {
            let step = {
                let (v0, v2) = m4538;
                let m4530 = v0;
                if m4530 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m4530 - 1;
                    let m4533 = {
                        let a4531 = v1.clone();
                        let a4532 = v2;
                        index_bytes(a4531, a4532)
                    };
                    match m4533 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a4534 = f62(v4)?;
                            let a4535 = 128u64;
                            if nat_lt(a4534, a4535) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a4536 = v2;
                                    let a4537 = 1u64;
                                    nat_add(a4536, a4537)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4538 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f658(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f192(v0.clone(), 0u64, {
        let a4541 = v0;
        length_bytes(a4541)
    }, false, Bytes::lit(&[]))?;
    let m4544 = {
        let a4542 = v1.clone();
        let a4543 = 0u64;
        index_bytes(a4542, a4543)
    };
    match m4544 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f62(v2)?;
            if f280(v3) {
                let a4551 = f46({
                    let a4545 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a4545)
                }, {
                    let a4546 = v3;
                    let a4547 = 65u64;
                    nat_sub(a4546, a4547)
                }, 1u64);
                let a4552 = f46(v1.clone(), 1u64, {
                    let a4549 = {
                        let a4548 = v1;
                        length_bytes(a4548)
                    };
                    let a4550 = 1u64;
                    nat_sub(a4549, a4550)
                });
                Ok(append_bytes(a4551, a4552))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f659(v0: Bytes) -> R<bool> {
    let m4555 = {
        let a4553 = v0;
        let a4554 = 0u64;
        index_bytes(a4553, a4554)
    };
    match m4555 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f180(f279(f62(v1)?), f280(f62(v1)?)))
        }
    }
}

pub fn f660(p4568: u64, v1: Bytes, p4569: u64, p4570: Bytes) -> R<Option<Bytes>> {
    let m4567 = (p4568, (p4569, p4570));
    {
        let mut m4567 = m4567;
        loop {
            let step = {
                let (v0, (v2, v3)) = m4567;
                let m4556 = v0;
                if m4556 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m4556 - 1;
                    let a4558 = {
                        let a4557 = v1.clone();
                        length_bytes(a4557)
                    };
                    let a4559 = v2;
                    if nat_le(a4558, a4559) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m4560 = f404(v1.clone(), v2)?;
                        match m4560 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a4562 = v2;
                                    let a4563 = {
                                        let (_, h4561) = v5;
                                        h4561
                                    };
                                    nat_add(a4562, a4563)?
                                }, {
                                    let a4565 = v3;
                                    let a4566 = f707({
                                        let (h4564, _) = v5;
                                        h4564
                                    });
                                    append_bytes(a4565, a4566)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4567 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f661() -> List<Bytes> {
    List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("termination_by")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("admit")), List::cons(f8(Str::lit("below")), List::cons(f8(Str::lit("brecOn")), List::cons(f8(Str::lit("casesOn")), List::cons(f8(Str::lit("ctorElim")), List::cons(f8(Str::lit("ctorElimType")), List::cons(f8(Str::lit("ctorIdx")), List::cons(f8(Str::lit("noConfusion")), List::cons(f8(Str::lit("noConfusionType")), List::cons(f8(Str::lit("rec")), List::cons(f8(Str::lit("recOn")), List::cons(f8(Str::lit("toCtorIdx")), List::<Bytes>::nil())))))))))))))))))))
}

pub fn f662() -> Bytes {
    f8(Str::lit("\u{0}\u{0}:\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{5}\u{7f}\u{0}\u{6}p\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f663() -> Bytes {
    f8(Str::lit("\u{0}\u{0}-\u{0}\u{0}.\u{0}\u{0}0\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}7\u{0}\u{1}7\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@?\u{0}@@\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f664(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f160(v0, {
        let (h4571, _) = v2;
        h4571
    })?;
    let a4573 = 1114111u64;
    let a4574 = {
        let (_, h4572) = v2;
        h4572
    };
    if nat_lt(a4573, a4574) {
        Ok(None::<(u64, u64)>)
    } else {
        let v4: u64 = f708(v3, v1);
        let a4575 = v4;
        let a4576 = 16u64;
        if nat_eq(a4575, a4576) {
            Ok(None::<(u64, u64)>)
        } else {
            Ok(Some(({
                let a4578 = {
                    let (h4577, _) = v2;
                    h4577
                };
                let a4579 = 1u64;
                nat_add(a4578, a4579)?
            }, {
                let a4583 = {
                    let a4581 = {
                        let (_, h4580) = v2;
                        h4580
                    };
                    let a4582 = if v1 {
                        16u64
                    } else {
                        10u64
                    };
                    nat_mul(a4581, a4582)?
                };
                let a4584 = v4;
                nat_add(a4583, a4584)?
            })))
        }
    }
}

pub fn f665() -> Bytes {
    f8(Str::lit("\u{80}\u{81}\u{82}\u{83}\u{84}\u{85}\u{86}\u{87}\u{88}\u{89}\u{8a}\u{8b}\u{8c}\u{8d}\u{8e}\u{8f}\u{90}\u{91}\u{92}\u{93}\u{94}\u{95}\u{96}\u{97}\u{98}\u{99}\u{9a}\u{9b}\u{9c}\u{9d}\u{9e}\u{9f}\u{a0}\u{a1}\u{a2}\u{a3}\u{a4}\u{a5}\u{a6}\u{a7}\u{a8}\u{a9}\u{aa}\u{ab}\u{ac}\u{ad}\u{ae}\u{af}\u{b0}\u{b1}\u{b2}\u{b3}\u{b4}\u{b5}\u{b6}\u{b7}\u{b8}\u{b9}\u{ba}\u{bb}\u{bc}\u{bd}\u{be}\u{bf}"))
}

pub fn f666(v0: u64, v1: bool, v2: u64) -> R<bool> {
    if v1 {
        let a4585 = v0;
        let a4586 = v2;
        if nat_eq(a4585, a4586) {
            Ok(true)
        } else {
            f346(v0, f709())
        }
    } else {
        f346(v0, f710())
    }
}

pub fn f667() -> Bytes {
    f8(Str::lit(" string:anyAtomicType boolean:anyAtomicType decimal:anyAtomicType float:anyAtomicType double:anyAtomicType duration:anyAtomicType dateTime:anyAtomicType time:anyAtomicType date:anyAtomicType gYearMonth:anyAtomicType gYear:anyAtomicType gMonthDay:anyAtomicType gDay:anyAtomicType gMonth:anyAtomicType hexBinary:anyAtomicType base64Binary:anyAtomicType anyURI:anyAtomicType QName:anyAtomicType NOTATION:anyAtomicType normalizedString:string token:normalizedString language:token NMTOKEN:token Name:token NCName:Name ID:NCName IDREF:NCName ENTITY:NCName integer:decimal nonPositiveInteger:integer long:integer nonNegativeInteger:integer negativeInteger:nonPositiveInteger int:long short:int byte:short unsignedLong:nonNegativeInteger positiveInteger:nonNegativeInteger unsignedInt:unsignedLong unsignedShort:unsignedInt unsignedByte:unsignedShort yearMonthDuration:duration dayTimeDuration:duration dateTimeStamp:dateTime NMTOKENS:anySimpleType IDREFS:anySimpleType ENTITIES:anySimpleType anyAtomicType:anySimpleType "))
}

pub fn f668(v0: Adt27, v1: Str, v2: Str) -> bool {
    let m4587 = f376(v0, v1);
    match m4587 {
        None => {
            false
        }
        Some(v3) => {
            f32(v3, f8(v2))
        }
    }
}

pub fn f669(v0: List<Adt26>, v1: (Bytes, Bytes), v2: bool) -> R<Result<List<Bytes>, Bytes>> {
    let m4588 = v0;
    match m4588.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m4589 = f669(v4, v1.clone(), v2)?;
            match m4589 {
                Ok(v6) => {
                    if f32(match v3.clone() {
                        Adt26::C0(h4590, _, _, _) => {
                            h4590
                        }
                    }, f8(Str::lit("complexType"))) {
                        let m4591 = f711(v3.clone())?;
                        match m4591 {
                            Ok(v8) => {
                                let m4592 = v8;
                                match m4592 {
                                    None => {
                                        Ok(Ok::<List<Bytes>, Bytes>(v6))
                                    }
                                    Some(v9) => {
                                        if if f32({
                                            let (h4593, _) = {
                                                let (_, h4594) = v9.clone();
                                                h4594
                                            };
                                            h4593
                                        }, {
                                            let (h4595, _) = v1;
                                            h4595
                                        }) {
                                            if f32({
                                                let (_, h4596) = {
                                                    let (_, h4597) = v9;
                                                    h4597
                                                };
                                                h4596
                                            }, {
                                                let (_, h4598) = v1;
                                                h4598
                                            }) {
                                                if f374(v2) {
                                                    true
                                                } else {
                                                    f375({
                                                        let (h4599, _) = v9;
                                                        h4599
                                                    }, Str::lit("extension"))
                                                }
                                            } else {
                                                false
                                            }
                                        } else {
                                            false
                                        } {
                                            Ok(Ok::<List<Bytes>, Bytes>(List::cons(match v3 {
                                                Adt26::C0(_, _, h4600, _) => {
                                                    h4600
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

pub fn f670(v0: Bytes, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<Adt28, Bytes>> {
    let m4601 = v1.clone();
    match m4601.uncons() {
        None => {
            Ok(Err::<Adt28, Bytes>(f595(f8(Str::lit("a choice with no particle that can occur matches no element sequence, so it has no LexLean constructor (XSD 1.1 Part 1, section 3.8.4.3)")), v0)))
        }
        Some((_, _)) => {
            let m4602 = f712(v1)?;
            match m4602 {
                Ok(v7) => {
                    let v8: List<Bytes> = f713(v7, v2);
                    Ok(Ok::<Adt28, Bytes>(Adt28::C0(v0.clone(), v3, f338(v0, v8.clone()), v8)))
                }
                Err(v6) => {
                    Ok(Err::<Adt28, Bytes>(f595(v6, v0)))
                }
            }
        }
    }
}

pub fn f671(v0: List<Bytes>) -> R<List<Bytes>> {
    let m4603 = v0;
    match m4603.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f125(v1)?, f671(v2)?))
        }
    }
}

pub fn f672(v0: u64, v1: Adt25, v2: Bytes, v3: Bytes, v4: Adt27) -> R<Result<List<Adt28>, Bytes>> {
    let m4604 = v0;
    if m4604 == 0 {
        Ok(Err::<List<Adt28>, Bytes>(f595(f8(Str::lit("the complex types nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m4604 - 1;
        let m4605 = f714(v5, v1, v2.clone(), v3, v4, f598())?;
        match m4605 {
            Ok(v7) => {
                let m4607 = f680(v2, match v7.clone() {
                    Adt36::C0(h4606, _, _, _, _) => {
                        h4606
                    }
                })?;
                match m4607 {
                    Ok(v9) => {
                        Ok(Ok::<List<Adt28>, Bytes>(f474(match v7 {
                            Adt36::C0(_, h4608, _, _, _) => {
                                h4608
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

pub fn f673(v0: Bytes) -> Bytes {
    let a4609 = v0;
    let a4610 = f8(Str::lit("Content"));
    append_bytes(a4609, a4610)
}

pub fn f674(v0: Adt25, v1: Adt27, v2: Bytes) -> R<Result<Adt24, Bytes>> {
    let m4612 = f164(match v1 {
        Adt27::C0(_, _, _, _, h4611) => {
            h4611
        }
    }, v2.clone())?;
    match m4612 {
        Ok(v4) => {
            if f32({
                let (h4613, _) = v4.clone();
                h4613
            }, f247()) {
                if f32({
                    let (_, h4614) = v4.clone();
                    h4614
                }, f8(Str::lit("anyType"))) {
                    Ok(Ok::<Adt24, Bytes>(Adt24::C0(f477(), List::cons(f477(), List::<Bytes>::nil()))))
                } else {
                    let m4616 = f469({
                        let (_, h4615) = v4;
                        h4615
                    }, 16u64)?;
                    match m4616 {
                        Ok(v6) => {
                            Ok(Ok::<Adt24, Bytes>(Adt24::C0(v6, List::<Bytes>::nil())))
                        }
                        Err(v5) => {
                            Ok(Err::<Adt24, Bytes>(v5))
                        }
                    }
                }
            } else {
                if f122({
                    let (h4617, _) = v4.clone();
                    h4617
                }, match v0.clone() {
                    Adt25::C0(_, h4618) => {
                        h4618
                    }
                }) {
                    let v7: Bytes = f125({
                        let (_, h4619) = v4.clone();
                        h4619
                    })?;
                    let m4620 = f681(v0.clone(), Str::lit("complexType"), v4.clone());
                    match m4620 {
                        None => {
                            let m4621 = f681(v0, Str::lit("simpleType"), v4);
                            match m4621 {
                                None => {
                                    Ok(Err::<Adt24, Bytes>(f257(f8(Str::lit("the type ")), v2, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)")))))
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
                    Ok(Err::<Adt24, Bytes>(f257(f8(Str::lit("the type ")), v2, f257(f8(Str::lit(" is in the namespace ")), {
                        let (h4622, _) = v4;
                        h4622
                    }, f8(Str::lit(", which no schema document of this set targets (XSD 1.1 Part 1, sections 3.17.6.2 and 4.2.6)"))))))
                }
            }
        }
        Err(v3) => {
            Ok(Err::<Adt24, Bytes>(v3))
        }
    }
}

pub fn f675(v0: Adt27, v1: Bytes) -> Bytes {
    if f668(v0, Str::lit("nillable"), Str::lit("true")) {
        f715(Str::lit("Option"), v1)
    } else {
        v1
    }
}

pub fn f676(p4627: u64, v1: Adt25, v2: Bytes, v3: Bytes, v4: bool, p4628: List<Adt27>, p4629: Adt36) -> R<Result<Adt36, Bytes>> {
    let m4626 = (p4627, (p4628, p4629));
    {
        let mut m4626 = m4626;
        loop {
            let step = {
                let (v0, (v5, v6)) = m4626;
                let m4623 = v0;
                if m4623 == 0 {
                    Err::<(u64, (List<Adt27>, Adt36)), R<Result<Adt36, Bytes>>>(Ok(Err::<Adt36, Bytes>(f595(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2.clone()))))
                } else {
                    let v7 = m4623 - 1;
                    let m4624 = v5;
                    match m4624.uncons() {
                        None => {
                            Err::<(u64, (List<Adt27>, Adt36)), R<Result<Adt36, Bytes>>>(Ok(Ok::<Adt36, Bytes>(v6)))
                        }
                        Some((v8, v9)) => {
                            let m4625 = f716(v7, v1.clone(), v2.clone(), v3.clone(), v4, v8, v6)?;
                            match m4625 {
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
                Ok(next) => { m4626 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f677(v0: List<Adt37>) -> List<Bytes> {
    let m4630 = v0;
    match m4630.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt37::C0(h4631, _, _) => {
                    h4631
                }
            }, f677(v2))
        }
    }
}

pub fn f678(v0: List<Adt37>) -> List<Bytes> {
    let m4632 = v0;
    match m4632.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt37::C0(_, h4633, _) => {
                    h4633
                }
            }, f678(v2))
        }
    }
}

pub fn f679(v0: List<Adt37>) -> List<Bytes> {
    let m4634 = v0;
    match m4634.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f484(match v1 {
                Adt37::C0(_, _, h4635) => {
                    h4635
                }
            }, f679(v2))
        }
    }
}

pub fn f680(v0: Bytes, v1: List<Adt37>) -> R<Result<Adt28, Bytes>> {
    let m4636 = f712(f677(v1.clone()))?;
    match m4636 {
        Ok(v3) => {
            let v4: List<Bytes> = f678(v1.clone());
            let v7: Bytes = {
                let m4637 = v1.clone();
                match m4637.uncons() {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some((_, _)) => {
                        f257(f8(Str::lit("\u{a}")), f153(f717(v3, v4.clone()), f8(Str::lit(",\u{a}"))), f8(Str::lit("\u{a}  ")))
                    }
                }
            };
            Ok(Ok::<Adt28, Bytes>(Adt28::C0(v0.clone(), f679(v1), f257(f8(Str::lit("structure{\u{a}  name: ")), v0, f257(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), v7, f8(Str::lit("]\u{a}}\u{a}")))), List::cons(f257(f8(Str::lit("    {name: mk, fields: [")), f153(v4, f8(Str::lit(", "))), f8(Str::lit("]}"))), List::<Bytes>::nil()))))
        }
        Err(v2) => {
            Ok(Err::<Adt28, Bytes>(f595(v2, v0)))
        }
    }
}

pub fn f681(v0: Adt25, v1: Str, v2: (Bytes, Bytes)) -> Option<Adt26> {
    f718(match v0 {
        Adt25::C0(h4638, _) => {
            h4638
        }
    }, f8(v1), {
        let (h4639, _) = v2;
        h4639
    }, {
        let (_, h4640) = v2;
        h4640
    })
}

pub fn f682(v0: Adt36, v1: Bytes, v2: Adt35, v3: bool) -> Adt36 {
    f719(f683(v0, Adt37::C0(v1, if v3 {
        let m4643 = match v2.clone() {
            Adt35::C0(h4642, _) => {
                h4642
            }
        };
        match m4643 {
            Adt24::C0(h4641, _) => {
                h4641
            }
        }
    } else {
        f715(Str::lit("Option"), {
            let m4646 = match v2.clone() {
                Adt35::C0(h4645, _) => {
                    h4645
                }
            };
            match m4646 {
                Adt24::C0(h4644, _) => {
                    h4644
                }
            }
        })
    }, {
        let m4649 = match v2.clone() {
            Adt35::C0(h4648, _) => {
                h4648
            }
        };
        match m4649 {
            Adt24::C0(_, h4647) => {
                h4647
            }
        }
    })), match v2 {
        Adt35::C0(_, h4650) => {
            h4650
        }
    })
}

pub fn f683(v0: Adt36, v1: Adt37) -> Adt36 {
    Adt36::C0(f720(match v0.clone() {
        Adt36::C0(h4651, _, _, _, _) => {
            h4651
        }
    }, List::cons(v1, List::<Adt37>::nil())), match v0.clone() {
        Adt36::C0(_, h4652, _, _, _) => {
            h4652
        }
    }, match v0.clone() {
        Adt36::C0(_, _, h4653, _, _) => {
            h4653
        }
    }, match v0.clone() {
        Adt36::C0(_, _, _, h4654, _) => {
            h4654
        }
    }, match v0 {
        Adt36::C0(_, _, _, _, h4655) => {
            h4655
        }
    })
}

pub fn f684() -> Bytes {
    f8(Str::lit("(List ((Bytes * Bytes) * Bytes))"))
}

pub fn f685(v0: Bytes, v1: List<Bytes>, v2: Bytes) -> Bytes {
    f257(f8(Str::lit("inductive{\u{a}  name: ")), v0, f257(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), f153(v1, f8(Str::lit(",\u{a}"))), f257(f8(Str::lit("\u{a}  ],\u{a}  mutual: ")), v2, f8(Str::lit("\u{a}}\u{a}")))))
}

pub fn f686(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h4656, _) = v1;
        h4656
    };
    let m4658 = {
        let (_, h4657) = v1;
        h4657
    };
    match m4658 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m4661 = {
                let a4659 = v0.clone();
                let a4660 = v2;
                index_bytes(a4659, a4660)
            };
            match m4661 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m4666 = {
                        let a4664 = v0;
                        let a4665 = {
                            let a4662 = v2;
                            let a4663 = 1u64;
                            nat_add(a4662, a4663)?
                        };
                        index_bytes(a4664, a4665)
                    };
                    match m4666 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f721(v4)? {
                                f721(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a4667 = v2;
                                    let a4668 = 2u64;
                                    nat_add(a4667, a4668)?
                                }, Some({
                                    let a4673 = v3;
                                    let a4674 = f209(f722(), {
                                        let a4671 = {
                                            let a4669 = f723(v4)?;
                                            let a4670 = 16u64;
                                            nat_mul(a4669, a4670)?
                                        };
                                        let a4672 = f723(v5)?;
                                        nat_add(a4671, a4672)?
                                    });
                                    append_bytes(a4673, a4674)
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

pub fn f687(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4675 = f275(v0, v1);
    let a4676 = f614(v0, v2);
    fixed_u32::bit_or(a4675, a4676)
}

pub fn f688(v0: u32) -> u32 {
    let a4679 = {
        let a4677 = f687(v0, 17u32, 15u32);
        let a4678 = f687(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a4677, a4678)
    };
    let a4680 = f275(v0, 10u32);
    fixed_u32::bit_xor(a4679, a4680)
}

pub fn f689(v0: u32) -> u32 {
    let a4683 = {
        let a4681 = f687(v0, 7u32, 25u32);
        let a4682 = f687(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a4681, a4682)
    };
    let a4684 = f275(v0, 3u32);
    fixed_u32::bit_xor(a4683, a4684)
}

pub fn f690() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f691(v0: Fn21, p4688: List<List<(Bytes, Bytes)>>, p4689: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m4687 = (p4688, p4689);
    {
        let mut m4687 = m4687;
        loop {
            let step = {
                let (v1, v2) = m4687;
                let m4685 = v2;
                match m4685.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c4686: Fn21 = v0.clone();
                            c4686.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m4687 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f692(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f693(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a4691 = {
        let a4690 = v0.clone();
        length_list(a4690)
    };
    let a4692 = 1u64;
    if nat_le(a4691, a4692) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f724(v0)?))
    }
}

pub fn f694(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f507({
        let (_, h4693) = {
            let (h4694, _) = f725(Fn22::F726, {
                let a4696 = {
                    let a4695 = v0.clone();
                    length_list(a4695)
                };
                let a4697 = 1u64;
                nat_add(a4696, a4697)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h4694
        };
        h4693
    }))
}

pub fn f695(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f727(v1, v0, v2)
}

pub fn f696(v0: u64, v1: Bytes, v2: u64, v3: u64) -> R<Option<u64>> {
    f728(v1, v2, v0, v3)
}

pub fn f697(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let a4700 = {
        let (h4698, _) = v1;
        h4698
    };
    let a4701 = {
        let a4699 = v0.clone();
        length_bytes(a4699)
    };
    if nat_lt(a4700, a4701) {
        Ok(Some((f303(v0, {
            let (h4702, _) = v1;
            h4702
        })?, {
            let a4704 = {
                let (_, h4703) = v1;
                h4703
            };
            let a4705 = 1u64;
            nat_add(a4704, a4705)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f698(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f729(v1, v0, v2)
}

pub fn f699(v0: u64) -> Option<u64> {
    let a4706 = 48u64;
    let a4707 = v0;
    if if nat_le(a4706, a4707) {
        let a4708 = v0;
        let a4709 = 57u64;
        nat_le(a4708, a4709)
    } else {
        false
    } {
        Some({
            let a4710 = v0;
            let a4711 = 48u64;
            nat_sub(a4710, a4711)
        })
    } else {
        let a4712 = 97u64;
        let a4713 = v0;
        if if nat_le(a4712, a4713) {
            let a4714 = v0;
            let a4715 = 102u64;
            nat_le(a4714, a4715)
        } else {
            false
        } {
            Some({
                let a4716 = v0;
                let a4717 = 87u64;
                nat_sub(a4716, a4717)
            })
        } else {
            let a4718 = 65u64;
            let a4719 = v0;
            if if nat_le(a4718, a4719) {
                let a4720 = v0;
                let a4721 = 70u64;
                nat_le(a4720, a4721)
            } else {
                false
            } {
                Some({
                    let a4722 = v0;
                    let a4723 = 55u64;
                    nat_sub(a4722, a4723)
                })
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f700(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f730(v0, v1)
}

pub fn f701(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f731(v0, v1)
}

pub fn f702(v0: Bytes, v1: u64, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    let v3: u64 = {
        let (h4724, _) = v2;
        h4724
    };
    let v4: u64 = f206(v0.clone(), v3)?;
    let (_, h4725) = v2;
    if h4725 {
        Ok(None::<(u64, bool)>)
    } else {
        if f299(v4) {
            Ok(Some(({
                let a4726 = v3;
                let a4727 = 1u64;
                nat_add(a4726, a4727)?
            }, false)))
        } else {
            let a4728 = v4;
            let a4729 = 35u64;
            if if nat_eq(a4728, a4729) {
                let a4730 = v3;
                let a4731 = 0u64;
                if if nat_eq(a4730, a4731) {
                    true
                } else {
                    f299(f206(v0.clone(), {
                        let a4732 = v3;
                        let a4733 = 1u64;
                        nat_sub(a4732, a4733)
                    })?)
                } {
                    true
                } else {
                    let a4736 = f206(v0.clone(), {
                        let a4734 = v3;
                        let a4735 = 1u64;
                        nat_sub(a4734, a4735)
                    })?;
                    let a4737 = 10u64;
                    nat_eq(a4736, a4737)
                }
            } else {
                false
            } {
                Ok(Some((f409(v0, v3)?, false)))
            } else {
                let a4738 = v4;
                let a4739 = 10u64;
                if nat_eq(a4738, a4739) {
                    let v5: u64 = {
                        let a4740 = v3;
                        let a4741 = 1u64;
                        nat_add(a4740, a4741)?
                    };
                    let v6: u64 = f304(v0.clone(), v5)?;
                    let v7: u64 = f302(v0.clone(), v6)?;
                    if if f204(v0.clone(), v5)? {
                        true
                    } else {
                        let a4744 = {
                            let a4742 = v6;
                            let a4743 = v5;
                            nat_sub(a4742, a4743)
                        };
                        let a4745 = v1;
                        if if nat_lt(a4744, a4745) {
                            f208(f300(f206(v0.clone(), v7)?))
                        } else {
                            false
                        } {
                            f208({
                                let a4746 = f206(v0, v7)?;
                                let a4747 = 35u64;
                                nat_eq(a4746, a4747)
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

pub fn f703(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f302(v0.clone(), {
        let (h4748, _) = v1;
        h4748
    })?;
    let a4749 = f206(v0, v2)?;
    let a4750 = 10u64;
    if nat_eq(a4749, a4750) {
        Ok(Some(({
            let a4751 = v2;
            let a4752 = 1u64;
            nat_add(a4751, a4752)?
        }, {
            let a4754 = {
                let (_, h4753) = v1;
                h4753
            };
            let a4755 = 1u64;
            nat_add(a4754, a4755)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f704(v0: u64, v1: u64) -> bool {
    let a4756 = v0;
    let a4757 = 0u64;
    if nat_eq(a4756, a4757) {
        f550(v1)
    } else {
        let a4758 = v0;
        let a4759 = 1u64;
        if nat_eq(a4758, a4759) {
            f549(v1)
        } else {
            let a4760 = v0;
            let a4761 = 2u64;
            if nat_eq(a4760, a4761) {
                f732(v1)
            } else {
                f733(v1)
            }
        }
    }
}

pub fn f705(v0: Bytes, v1: u64) -> R<u64> {
    let a4763 = v1;
    let a4764 = {
        let a4762 = v0.clone();
        length_bytes(a4762)
    };
    if nat_lt(a4763, a4764) {
        let a4765 = f160(v0, v1)?;
        let a4766 = 1u64;
        nat_add(a4765, a4766)
    } else {
        Ok(0u64)
    }
}

pub fn f706(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f180(f113(f180(f279(v2), f280(v2)), f113({
        let a4767 = v2;
        let a4768 = 117u64;
        !nat_eq(a4767, a4768)
    }, {
        let a4769 = v2;
        let a4770 = 85u64;
        !nat_eq(a4769, a4770)
    })), f113(f281(v2), {
        let a4771 = 0u64;
        let a4772 = v1;
        nat_lt(a4771, a4772)
    })) {
        f46(v0, v1, 1u64)
    } else {
        let a4773 = v2;
        let a4774 = 65536u64;
        if nat_lt(a4773, a4774) {
            let a4775 = f8(Str::lit("u"));
            let a4776 = f568(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a4775, a4776)
        } else {
            let a4777 = f8(Str::lit("U"));
            let a4778 = f568(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a4777, a4778)
        }
    }
}

pub fn f707(v0: u64) -> Bytes {
    let a4779 = f8(Str::lit("u"));
    let a4780 = f568(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a4779, a4780)
}

pub fn f708(v0: u64, v1: bool) -> u64 {
    let a4781 = v0;
    let a4782 = 48u64;
    if nat_lt(a4781, a4782) {
        16u64
    } else {
        let a4783 = v0;
        let a4784 = 57u64;
        if nat_le(a4783, a4784) {
            let a4785 = v0;
            let a4786 = 48u64;
            nat_sub(a4785, a4786)
        } else {
            if v1 {
                let a4787 = v0;
                let a4788 = 65u64;
                if nat_lt(a4787, a4788) {
                    16u64
                } else {
                    let a4789 = v0;
                    let a4790 = 70u64;
                    if nat_le(a4789, a4790) {
                        let a4791 = v0;
                        let a4792 = 55u64;
                        nat_sub(a4791, a4792)
                    } else {
                        let a4793 = v0;
                        let a4794 = 97u64;
                        if nat_lt(a4793, a4794) {
                            16u64
                        } else {
                            let a4795 = v0;
                            let a4796 = 102u64;
                            if nat_le(a4795, a4796) {
                                let a4797 = v0;
                                let a4798 = 87u64;
                                nat_sub(a4797, a4798)
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

pub fn f709() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f710() -> Bytes {
    f8(Str::lit("\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{0}]\u{0}\u{0}]\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f711(v0: Adt26) -> R<Result<Option<(Adt27, (Bytes, Bytes))>, Bytes>> {
    let m4800 = f593(match v0 {
        Adt26::C0(_, _, _, h4799) => {
            h4799
        }
    })?;
    match m4800 {
        Ok(v2) => {
            let m4801 = f594(v2, List::cons(f8(Str::lit("simpleContent")), List::cons(f8(Str::lit("complexContent")), List::<Bytes>::nil())));
            match m4801 {
                None => {
                    Ok(Ok::<Option<(Adt27, (Bytes, Bytes))>, Bytes>(None::<(Adt27, (Bytes, Bytes))>))
                }
                Some(v3) => {
                    let m4802 = f593(v3)?;
                    match m4802 {
                        Ok(v5) => {
                            let m4803 = f594(v5, List::cons(f8(Str::lit("extension")), List::cons(f8(Str::lit("restriction")), List::<Bytes>::nil())));
                            match m4803 {
                                None => {
                                    Ok(Ok::<Option<(Adt27, (Bytes, Bytes))>, Bytes>(None::<(Adt27, (Bytes, Bytes))>))
                                }
                                Some(v6) => {
                                    let m4804 = f376(v6.clone(), Str::lit("base"));
                                    match m4804 {
                                        None => {
                                            Ok(Ok::<Option<(Adt27, (Bytes, Bytes))>, Bytes>(None::<(Adt27, (Bytes, Bytes))>))
                                        }
                                        Some(v7) => {
                                            let m4806 = f164(match v6.clone() {
                                                Adt27::C0(_, _, _, _, h4805) => {
                                                    h4805
                                                }
                                            }, v7)?;
                                            match m4806 {
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

pub fn f712(v0: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f734(f332(v0.clone())?, v0, List::<Bytes>::nil())
}

pub fn f713(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m4807 = v0;
    match m4807.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m4808 = v1;
            match m4808.uncons() {
                None => {
                    List::<Bytes>::nil()
                }
                Some((v4, v5)) => {
                    List::cons(f257(f8(Str::lit("    {name: ")), v2, f257(f8(Str::lit(", fields: [")), v4, f8(Str::lit("]}")))), f713(v3, v5))
                }
            }
        }
    }
}

pub fn f714(v0: u64, v1: Adt25, v2: Bytes, v3: Bytes, v4: Adt27, v5: Adt36) -> R<Result<Adt36, Bytes>> {
    let m4809 = v0;
    if m4809 == 0 {
        Ok(Err::<Adt36, Bytes>(f595(f8(Str::lit("the type derivations nest deeper than the schema documents have bytes, so the types derive from each other in a cycle (XSD 1.1 Part 1, section 3.4.6.1)")), v2)))
    } else {
        let v6 = m4809 - 1;
        if f668(v4.clone(), Str::lit("mixed"), Str::lit("true")) {
            Ok(Err::<Adt36, Bytes>(f595(f8(Str::lit("a mixed content type interleaves character data with its elements, which a LexLean field list does not keep in order, so it is not translated (XSD 1.1 Part 1, section 3.4.2.3)")), v2)))
        } else {
            let m4810 = f593(v4)?;
            match m4810 {
                Ok(v8) => {
                    let m4811 = f594(v8.clone(), List::cons(f8(Str::lit("openContent")), List::cons(f8(Str::lit("assert")), List::<Bytes>::nil())));
                    match m4811 {
                        None => {
                            let m4813 = f594(v8.clone(), List::cons(f8(Str::lit("simpleContent")), List::cons(f8(Str::lit("complexContent")), List::<Bytes>::nil())));
                            match m4813 {
                                None => {
                                    let m4814 = f676(v6, v1.clone(), v2.clone(), v3.clone(), true, v8.clone(), v5)?;
                                    match m4814 {
                                        Ok(v11) => {
                                            f597(v6, v1, v2, v3, v8, v11)
                                        }
                                        Err(v10) => {
                                            Ok(Err::<Adt36, Bytes>(v10))
                                        }
                                    }
                                }
                                Some(v12) => {
                                    if f668(v12.clone(), Str::lit("mixed"), Str::lit("true")) {
                                        Ok(Err::<Adt36, Bytes>(f595(f8(Str::lit("a mixed content type interleaves character data with its elements, which a LexLean field list does not keep in order, so it is not translated (XSD 1.1 Part 1, section 3.4.2.3)")), v2)))
                                    } else {
                                        let m4815 = f593(v12.clone())?;
                                        match m4815 {
                                            Ok(v14) => {
                                                let m4816 = f594(v14, List::cons(f8(Str::lit("extension")), List::cons(f8(Str::lit("restriction")), List::<Bytes>::nil())));
                                                match m4816 {
                                                    None => {
                                                        Ok(Err::<Adt36, Bytes>(f595(f8(Str::lit("simpleContent and complexContent must have an extension or a restriction child (XSD 1.1 Part 1, sections 3.4.2.2 and 3.4.2.3)")), v2)))
                                                    }
                                                    Some(v15) => {
                                                        let m4817 = f376(v15.clone(), Str::lit("base"));
                                                        match m4817 {
                                                            None => {
                                                                Ok(Err::<Adt36, Bytes>(f595(f8(Str::lit("an extension or restriction must name its base type (XSD 1.1 Part 1, section 3.4.3)")), v2)))
                                                            }
                                                            Some(v16) => {
                                                                let m4819 = f164(match v15.clone() {
                                                                    Adt27::C0(_, _, _, _, h4818) => {
                                                                        h4818
                                                                    }
                                                                }, v16.clone())?;
                                                                match m4819 {
                                                                    Ok(v18) => {
                                                                        let m4820 = f593(v15.clone())?;
                                                                        match m4820 {
                                                                            Ok(v20) => {
                                                                                let v21: bool = if f32({
                                                                                    let (h4821, _) = v18.clone();
                                                                                    h4821
                                                                                }, f247()) {
                                                                                    f32({
                                                                                        let (_, h4822) = v18.clone();
                                                                                        h4822
                                                                                    }, f8(Str::lit("anyType")))
                                                                                } else {
                                                                                    false
                                                                                };
                                                                                if if f375(v15.clone(), Str::lit("restriction")) {
                                                                                    f374(v21)
                                                                                } else {
                                                                                    false
                                                                                } {
                                                                                    Ok(Err::<Adt36, Bytes>(f595(f257(f8(Str::lit("a restriction of the complex type ")), v16, f8(Str::lit(" is not translated, because the attribute uses it inherits and the content it narrows are not read yet (XSD 1.1 Part 1, sections 3.4.2.2 and 3.4.2.3)"))), v2)))
                                                                                } else {
                                                                                    let m4837 = if if v21 {
                                                                                        true
                                                                                    } else {
                                                                                        f32(match v12 {
                                                                                            Adt27::C0(_, h4823, _, _, _) => {
                                                                                                h4823
                                                                                            }
                                                                                        }, f8(Str::lit("complexContent")))
                                                                                    } {
                                                                                        if v21 {
                                                                                            Ok::<Adt36, Bytes>(v5)
                                                                                        } else {
                                                                                            let m4824 = f681(v1.clone(), Str::lit("complexType"), v18.clone());
                                                                                            match m4824 {
                                                                                                None => {
                                                                                                    Err::<Adt36, Bytes>(f595(f257(f8(Str::lit("the base type ")), v16, f8(Str::lit(" of a complex content extension is not a complex type of this schema set (XSD 1.1 Part 1, section 3.4.2.3)"))), v2.clone()))
                                                                                                }
                                                                                                Some(v22) => {
                                                                                                    let m4828 = f714(v6, v1.clone(), f125({
                                                                                                        let (_, h4825) = v18;
                                                                                                        h4825
                                                                                                    })?, match v22.clone() {
                                                                                                        Adt26::C0(_, h4826, _, _) => {
                                                                                                            h4826
                                                                                                        }
                                                                                                    }, match v22 {
                                                                                                        Adt26::C0(_, _, _, h4827) => {
                                                                                                            h4827
                                                                                                        }
                                                                                                    }, v5)?;
                                                                                                    match m4828 {
                                                                                                        Ok(v24) => {
                                                                                                            Ok::<Adt36, Bytes>(f735(v24))
                                                                                                        }
                                                                                                        Err(v23) => {
                                                                                                            Err::<Adt36, Bytes>(v23)
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        let m4829 = f681(v1.clone(), Str::lit("complexType"), v18.clone());
                                                                                        match m4829 {
                                                                                            None => {
                                                                                                let m4834 = f674(v1.clone(), v15, v16)?;
                                                                                                match m4834 {
                                                                                                    Ok(v29) => {
                                                                                                        Ok::<Adt36, Bytes>(f683(v5, Adt37::C0(f8(Str::lit("value")), match v29.clone() {
                                                                                                            Adt24::C0(h4835, _) => {
                                                                                                                h4835
                                                                                                            }
                                                                                                        }, match v29 {
                                                                                                            Adt24::C0(_, h4836) => {
                                                                                                                h4836
                                                                                                            }
                                                                                                        })))
                                                                                                    }
                                                                                                    Err(v28) => {
                                                                                                        Err::<Adt36, Bytes>(f595(v28, v2.clone()))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            Some(v25) => {
                                                                                                let m4833 = f714(v6, v1.clone(), f125({
                                                                                                    let (_, h4830) = v18;
                                                                                                    h4830
                                                                                                })?, match v25.clone() {
                                                                                                    Adt26::C0(_, h4831, _, _) => {
                                                                                                        h4831
                                                                                                    }
                                                                                                }, match v25 {
                                                                                                    Adt26::C0(_, _, _, h4832) => {
                                                                                                        h4832
                                                                                                    }
                                                                                                }, v5)?;
                                                                                                match m4833 {
                                                                                                    Ok(v27) => {
                                                                                                        Ok::<Adt36, Bytes>(f735(v27))
                                                                                                    }
                                                                                                    Err(v26) => {
                                                                                                        Err::<Adt36, Bytes>(v26)
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    };
                                                                                    match m4837 {
                                                                                        Ok(v31) => {
                                                                                            let m4838 = f676(v6, v1.clone(), v2.clone(), v3.clone(), true, v20.clone(), v31)?;
                                                                                            match m4838 {
                                                                                                Ok(v33) => {
                                                                                                    f597(v6, v1, v2, v3, v20, v33)
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
                                                                        Ok(Err::<Adt36, Bytes>(f595(v17, v2)))
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
                            Ok(Err::<Adt36, Bytes>(f595(f257(f8(Str::lit("xs:")), match v9 {
                                Adt27::C0(_, h4812, _, _, _) => {
                                    h4812
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

pub fn f715(v0: Str, v1: Bytes) -> Bytes {
    f257(f8(Str::lit("(")), f8(v0), f257(f8(Str::lit(" ")), v1, f8(Str::lit(")"))))
}

pub fn f716(v0: u64, v1: Adt25, v2: Bytes, v3: Bytes, v4: bool, v5: Adt27, v6: Adt36) -> R<Result<Adt36, Bytes>> {
    let m4839 = v0;
    if m4839 == 0 {
        Ok(Err::<Adt36, Bytes>(f595(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v7 = m4839 - 1;
        if f375(v5.clone(), Str::lit("element")) {
            let m4840 = f736(v7, v1, v2, v3, v5.clone())?;
            match m4840 {
                Ok(v9) => {
                    let m4846 = f737(v5, {
                        let m4845 = {
                            let m4844 = match v9.clone() {
                                Adt40::C0(_, h4843) => {
                                    h4843
                                }
                            };
                            match m4844 {
                                Adt35::C0(h4842, _) => {
                                    h4842
                                }
                            }
                        };
                        match m4845 {
                            Adt24::C0(h4841, _) => {
                                h4841
                            }
                        }
                    });
                    match m4846 {
                        None => {
                            Ok(Ok::<Adt36, Bytes>(v6))
                        }
                        Some(v10) => {
                            Ok(Ok::<Adt36, Bytes>(f719(f683(v6, Adt37::C0(match v9.clone() {
                                Adt40::C0(h4847, _) => {
                                    h4847
                                }
                            }, v10, {
                                let m4852 = {
                                    let m4851 = match v9.clone() {
                                        Adt40::C0(_, h4850) => {
                                            h4850
                                        }
                                    };
                                    match m4851 {
                                        Adt35::C0(h4849, _) => {
                                            h4849
                                        }
                                    }
                                };
                                match m4852 {
                                    Adt24::C0(_, h4848) => {
                                        h4848
                                    }
                                }
                            })), {
                                let m4855 = match v9 {
                                    Adt40::C0(_, h4854) => {
                                        h4854
                                    }
                                };
                                match m4855 {
                                    Adt35::C0(_, h4853) => {
                                        h4853
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
            if f375(v5.clone(), Str::lit("any")) {
                let m4856 = f737(v5, f477());
                match m4856 {
                    None => {
                        Ok(Ok::<Adt36, Bytes>(v6))
                    }
                    Some(v11) => {
                        Ok(Ok::<Adt36, Bytes>(f683(v6, Adt37::C0(f8(Str::lit("any")), v11, List::cons(f477(), List::<Bytes>::nil())))))
                    }
                }
            } else {
                if f375(v5.clone(), Str::lit("group")) {
                    let m4857 = f376(v5.clone(), Str::lit("ref"));
                    match m4857 {
                        None => {
                            Ok(Err::<Adt36, Bytes>(f595(f8(Str::lit("a model group reference must have a ref (XSD 1.1 Part 1, section 3.7.2)")), v2)))
                        }
                        Some(v12) => {
                            let m4859 = f164(match v5.clone() {
                                Adt27::C0(_, _, _, _, h4858) => {
                                    h4858
                                }
                            }, v12.clone())?;
                            match m4859 {
                                Ok(v14) => {
                                    let m4860 = f681(v1, Str::lit("group"), v14.clone());
                                    match m4860 {
                                        None => {
                                            Ok(Err::<Adt36, Bytes>(f595(f257(f8(Str::lit("the model group ")), v12, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                        }
                                        Some(_) => {
                                            let v16: Bytes = f125({
                                                let (_, h4861) = v14.clone();
                                                h4861
                                            })?;
                                            let m4862 = f737(v5, v16.clone());
                                            match m4862 {
                                                None => {
                                                    Ok(Ok::<Adt36, Bytes>(v6))
                                                }
                                                Some(v17) => {
                                                    Ok(Ok::<Adt36, Bytes>(f683(v6, Adt37::C0({
                                                        let (_, h4863) = v14;
                                                        h4863
                                                    }, v17, List::cons(v16, List::<Bytes>::nil())))))
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(v13) => {
                                    Ok(Err::<Adt36, Bytes>(f595(v13, v2)))
                                }
                            }
                        }
                    }
                } else {
                    if if f375(v5.clone(), Str::lit("sequence")) {
                        true
                    } else {
                        if f375(v5.clone(), Str::lit("choice")) {
                            true
                        } else {
                            f375(v5.clone(), Str::lit("all"))
                        }
                    } {
                        if if v4 {
                            if f738(v5.clone()) {
                                f374(f375(v5.clone(), Str::lit("choice")))
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m4864 = f593(v5)?;
                            match m4864 {
                                Ok(v19) => {
                                    f676(v7, v1, v2, v3, true, v19, v6)
                                }
                                Err(v18) => {
                                    Ok(Err::<Adt36, Bytes>(v18))
                                }
                            }
                        } else {
                            let v20: Bytes = match v5.clone() {
                                Adt27::C0(_, h4865, _, _, _) => {
                                    h4865
                                }
                            };
                            let v21: u64 = f739(v6.clone(), v20.clone())?;
                            let v22: Bytes = {
                                let a4866 = v21;
                                let a4867 = 1u64;
                                if nat_eq(a4866, a4867) {
                                    Bytes::lit(&[])
                                } else {
                                    f91(v21)?
                                }
                            };
                            let v23: Bytes = f257(v2, f125(v20.clone())?, v22.clone());
                            let m4868 = f596(v7, v1, v23.clone(), v3, v5.clone())?;
                            match m4868 {
                                Ok(v25) => {
                                    let m4869 = f737(v5, v23.clone());
                                    match m4869 {
                                        None => {
                                            Ok(Ok::<Adt36, Bytes>(f740(v6, v20)?))
                                        }
                                        Some(v26) => {
                                            Ok(Ok::<Adt36, Bytes>(f719(f683(f740(v6, v20.clone())?, Adt37::C0({
                                                let a4870 = v20;
                                                let a4871 = v22;
                                                append_bytes(a4870, a4871)
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

pub fn f717(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m4872 = v0;
    match m4872.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m4873 = v1;
            match m4873.uncons() {
                None => {
                    List::<Bytes>::nil()
                }
                Some((v4, v5)) => {
                    List::cons(f257(f8(Str::lit("    {name: ")), v2, f257(f8(Str::lit(", type: ")), v4, f8(Str::lit("}")))), f717(v3, v5))
                }
            }
        }
    }
}

pub fn f718(p4879: List<Adt26>, v1: Bytes, v2: Bytes, v3: Bytes) -> Option<Adt26> {
    let m4878 = p4879;
    {
        let mut m4878 = m4878;
        loop {
            let step = {
                let v0 = m4878;
                let m4874 = v0;
                match m4874.uncons() {
                    None => {
                        Err::<List<Adt26>, Option<Adt26>>(None::<Adt26>)
                    }
                    Some((v4, v5)) => {
                        if if f32(match v4.clone() {
                            Adt26::C0(h4875, _, _, _) => {
                                h4875
                            }
                        }, v1.clone()) {
                            if f32(match v4.clone() {
                                Adt26::C0(_, h4876, _, _) => {
                                    h4876
                                }
                            }, v2.clone()) {
                                f32(match v4.clone() {
                                    Adt26::C0(_, _, h4877, _) => {
                                        h4877
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
                Ok(next) => { m4878 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f719(v0: Adt36, v1: List<Adt28>) -> Adt36 {
    Adt36::C0(match v0.clone() {
        Adt36::C0(h4880, _, _, _, _) => {
            h4880
        }
    }, f474(match v0.clone() {
        Adt36::C0(_, h4881, _, _, _) => {
            h4881
        }
    }, v1), match v0.clone() {
        Adt36::C0(_, _, h4882, _, _) => {
            h4882
        }
    }, match v0.clone() {
        Adt36::C0(_, _, _, h4883, _) => {
            h4883
        }
    }, match v0 {
        Adt36::C0(_, _, _, _, h4884) => {
            h4884
        }
    })
}

pub fn f720(v0: List<Adt37>, v1: List<Adt37>) -> List<Adt37> {
    let m4885 = v0;
    match m4885.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f720(v3, v1))
        }
    }
}

pub fn f721(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4930 = {
            let a4906 = {
                let a4894 = {
                    let a4888 = {
                        let a4886 = v1;
                        let a4887 = 128u8;
                        fixed_u8::bit_and(a4886, a4887)
                    };
                    let a4889 = 128u8;
                    if equal(a4888, a4889) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4895 = {
                    let a4892 = {
                        let a4890 = v1;
                        let a4891 = 64u8;
                        fixed_u8::bit_and(a4890, a4891)
                    };
                    let a4893 = 64u8;
                    if equal(a4892, a4893) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4894, a4895)?
            };
            let a4907 = {
                let a4904 = {
                    let a4898 = {
                        let a4896 = v1;
                        let a4897 = 32u8;
                        fixed_u8::bit_and(a4896, a4897)
                    };
                    let a4899 = 32u8;
                    if equal(a4898, a4899) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4905 = {
                    let a4902 = {
                        let a4900 = v1;
                        let a4901 = 16u8;
                        fixed_u8::bit_and(a4900, a4901)
                    };
                    let a4903 = 16u8;
                    if equal(a4902, a4903) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4904, a4905)?
            };
            nat_add(a4906, a4907)?
        };
        let a4931 = {
            let a4928 = {
                let a4916 = {
                    let a4910 = {
                        let a4908 = v1;
                        let a4909 = 8u8;
                        fixed_u8::bit_and(a4908, a4909)
                    };
                    let a4911 = 8u8;
                    if equal(a4910, a4911) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4917 = {
                    let a4914 = {
                        let a4912 = v1;
                        let a4913 = 4u8;
                        fixed_u8::bit_and(a4912, a4913)
                    };
                    let a4915 = 4u8;
                    if equal(a4914, a4915) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4916, a4917)?
            };
            let a4929 = {
                let a4926 = {
                    let a4920 = {
                        let a4918 = v1;
                        let a4919 = 2u8;
                        fixed_u8::bit_and(a4918, a4919)
                    };
                    let a4921 = 2u8;
                    if equal(a4920, a4921) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4927 = {
                    let a4924 = {
                        let a4922 = v1;
                        let a4923 = 1u8;
                        fixed_u8::bit_and(a4922, a4923)
                    };
                    let a4925 = 1u8;
                    if equal(a4924, a4925) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4926, a4927)?
            };
            nat_add(a4928, a4929)?
        };
        nat_add(a4930, a4931)?
    };
    let a4932 = 48u64;
    let a4933 = v2;
    if if nat_le(a4932, a4933) {
        let a4934 = v2;
        let a4935 = 57u64;
        nat_le(a4934, a4935)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a4936 = 97u64;
        let a4937 = v2;
        if nat_le(a4936, a4937) {
            let a4938 = v2;
            let a4939 = 102u64;
            Ok(nat_le(a4938, a4939))
        } else {
            Ok(false)
        }
    }
}

pub fn f722() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f723(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4984 = {
            let a4960 = {
                let a4948 = {
                    let a4942 = {
                        let a4940 = v1;
                        let a4941 = 128u8;
                        fixed_u8::bit_and(a4940, a4941)
                    };
                    let a4943 = 128u8;
                    if equal(a4942, a4943) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4949 = {
                    let a4946 = {
                        let a4944 = v1;
                        let a4945 = 64u8;
                        fixed_u8::bit_and(a4944, a4945)
                    };
                    let a4947 = 64u8;
                    if equal(a4946, a4947) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4948, a4949)?
            };
            let a4961 = {
                let a4958 = {
                    let a4952 = {
                        let a4950 = v1;
                        let a4951 = 32u8;
                        fixed_u8::bit_and(a4950, a4951)
                    };
                    let a4953 = 32u8;
                    if equal(a4952, a4953) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4959 = {
                    let a4956 = {
                        let a4954 = v1;
                        let a4955 = 16u8;
                        fixed_u8::bit_and(a4954, a4955)
                    };
                    let a4957 = 16u8;
                    if equal(a4956, a4957) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4958, a4959)?
            };
            nat_add(a4960, a4961)?
        };
        let a4985 = {
            let a4982 = {
                let a4970 = {
                    let a4964 = {
                        let a4962 = v1;
                        let a4963 = 8u8;
                        fixed_u8::bit_and(a4962, a4963)
                    };
                    let a4965 = 8u8;
                    if equal(a4964, a4965) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4971 = {
                    let a4968 = {
                        let a4966 = v1;
                        let a4967 = 4u8;
                        fixed_u8::bit_and(a4966, a4967)
                    };
                    let a4969 = 4u8;
                    if equal(a4968, a4969) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4970, a4971)?
            };
            let a4983 = {
                let a4980 = {
                    let a4974 = {
                        let a4972 = v1;
                        let a4973 = 2u8;
                        fixed_u8::bit_and(a4972, a4973)
                    };
                    let a4975 = 2u8;
                    if equal(a4974, a4975) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4981 = {
                    let a4978 = {
                        let a4976 = v1;
                        let a4977 = 1u8;
                        fixed_u8::bit_and(a4976, a4977)
                    };
                    let a4979 = 1u8;
                    if equal(a4978, a4979) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4980, a4981)?
            };
            nat_add(a4982, a4983)?
        };
        nat_add(a4984, a4985)?
    };
    let a4986 = v2;
    let a4987 = 57u64;
    if nat_le(a4986, a4987) {
        let a4988 = v2;
        let a4989 = 48u64;
        Ok(nat_sub(a4988, a4989))
    } else {
        let a4990 = v2;
        let a4991 = 87u64;
        Ok(nat_sub(a4990, a4991))
    }
}

pub fn f724(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f616({
        let (_, h4992) = {
            let (h4993, _) = f741(Fn23::F742, {
                let a4995 = {
                    let a4994 = v0.clone();
                    length_list(a4994)
                };
                let a4996 = 1u64;
                nat_add(a4995, a4996)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h4993
        };
        h4992
    }))
}

pub fn f725(v0: Fn22, p5001: u64, p5002: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m5000 = (p5001, p5002);
    {
        let mut m5000 = m5000;
        loop {
            let step = {
                let (v1, v2) = m5000;
                let m4997 = v1;
                if m4997 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4997 - 1;
                    let m4999 = {
                        let c4998: Fn22 = v0.clone();
                        c4998.apply(v2.clone())?
                    };
                    match m4999 {
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
                Ok(next) => { m5000 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f726(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f743(v0)
}

pub fn f727(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f629(v0.clone(), {
        let (h5003, _) = v2;
        h5003
    }, v1)?;
    let a5004 = f206(v0, v3)?;
    let a5005 = 10u64;
    if nat_eq(a5004, a5005) {
        Ok(Some(({
            let a5006 = v3;
            let a5007 = 1u64;
            nat_add(a5006, a5007)?
        }, {
            let a5009 = {
                let (_, h5008) = v2;
                h5008
            };
            let a5010 = 1u64;
            nat_add(a5009, a5010)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f728(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a5011 = f206(v0, v3)?;
    let a5012 = 32u64;
    if if nat_eq(a5011, a5012) {
        let a5015 = {
            let a5013 = v3;
            let a5014 = v1;
            nat_sub(a5013, a5014)
        };
        let a5016 = v2;
        nat_lt(a5015, a5016)
    } else {
        false
    } {
        Ok(Some({
            let a5017 = v3;
            let a5018 = 1u64;
            nat_add(a5017, a5018)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f729(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m5020 = f699(f206(v0, {
        let (h5019, _) = v2;
        h5019
    })?);
    match m5020 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v3) => {
            let a5021 = v3;
            let a5022 = v1;
            if nat_lt(a5021, a5022) {
                Ok(Some(({
                    let a5024 = {
                        let (h5023, _) = v2;
                        h5023
                    };
                    let a5025 = 1u64;
                    nat_add(a5024, a5025)?
                }, {
                    let a5029 = {
                        let a5027 = {
                            let (_, h5026) = v2;
                            h5026
                        };
                        let a5028 = v1;
                        nat_mul(a5027, a5028)?
                    };
                    let a5030 = v3;
                    nat_add(a5029, a5030)?
                })))
            } else {
                Ok(None::<(u64, u64)>)
            }
        }
    }
}

pub fn f730(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f206(v0, v1)?;
    let a5031 = v2;
    let a5032 = 34u64;
    if if if if nat_eq(a5031, a5032) {
        true
    } else {
        let a5033 = v2;
        let a5034 = 92u64;
        nat_eq(a5033, a5034)
    } {
        true
    } else {
        f299(v2)
    } {
        true
    } else {
        f300(v2)
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a5035 = v1;
            let a5036 = 1u64;
            nat_add(a5035, a5036)?
        }))
    }
}

pub fn f731(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f206(v0, v1)?;
    let a5037 = v2;
    let a5038 = 39u64;
    if if if nat_eq(a5037, a5038) {
        true
    } else {
        f299(v2)
    } {
        true
    } else {
        f300(v2)
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a5039 = v1;
            let a5040 = 1u64;
            nat_add(a5039, a5040)?
        }))
    }
}

pub fn f732(v0: u64) -> bool {
    let a5041 = 48u64;
    let a5042 = v0;
    if nat_le(a5041, a5042) {
        let a5043 = v0;
        let a5044 = 55u64;
        nat_le(a5043, a5044)
    } else {
        false
    }
}

pub fn f733(v0: u64) -> bool {
    if f426(v0) {
        true
    } else {
        if f550(v0) {
            true
        } else {
            let a5045 = v0;
            let a5046 = 95u64;
            if nat_eq(a5045, a5046) {
                true
            } else {
                let a5047 = v0;
                let a5048 = 45u64;
                nat_eq(a5047, a5048)
            }
        }
    }
}

pub fn f734(v0: bool, v1: List<Bytes>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m5049 = v1;
    match m5049.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m5050 = f454(v0, v3.clone())?;
            match m5050 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f257(f8(Str::lit("the component name ")), v3, f8(Str::lit(" derives no LexLean name")))))
                }
                Some(v5) => {
                    if f122(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f257(f8(Str::lit("two components derive the name ")), v5, f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n; when those names are not distinct, every name keeps its letters and digits and writes each other character as a C11 universal character name)")))))
                    } else {
                        let m5051 = f734(v0, v4, List::cons(v5.clone(), v2))?;
                        match m5051 {
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

pub fn f735(v0: Adt36) -> Adt36 {
    Adt36::C0(match v0.clone() {
        Adt36::C0(h5052, _, _, _, _) => {
            h5052
        }
    }, match v0 {
        Adt36::C0(_, h5053, _, _, _) => {
            h5053
        }
    }, 0u64, 0u64, 0u64)
}

pub fn f736(v0: u64, v1: Adt25, v2: Bytes, v3: Bytes, v4: Adt27) -> R<Result<Adt40, Bytes>> {
    let m5054 = v0;
    if m5054 == 0 {
        Ok(Err::<Adt40, Bytes>(f595(f8(Str::lit("the element declarations nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m5054 - 1;
        let m5055 = f376(v4.clone(), Str::lit("ref"));
        match m5055 {
            None => {
                let m5066 = f376(v4.clone(), Str::lit("name"));
                match m5066 {
                    None => {
                        Ok(Err::<Adt40, Bytes>(f595(f8(Str::lit("a local element declaration must have a name or a ref (XSD 1.1 Part 1, section 3.3.3)")), v2)))
                    }
                    Some(v12) => {
                        let m5069 = f591(v5, v1, {
                            let a5067 = v2;
                            let a5068 = f125(v12.clone())?;
                            append_bytes(a5067, a5068)
                        }, v3, v4)?;
                        match m5069 {
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
                let m5057 = f164(match v4 {
                    Adt27::C0(_, _, _, _, h5056) => {
                        h5056
                    }
                }, v6.clone())?;
                match m5057 {
                    Ok(v8) => {
                        let m5058 = f681(v1.clone(), Str::lit("element"), v8.clone());
                        match m5058 {
                            None => {
                                Ok(Err::<Adt40, Bytes>(f595(f257(f8(Str::lit("the element ")), v6, f8(Str::lit(" is not declared by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                            }
                            Some(v9) => {
                                if if f668(match v9.clone() {
                                    Adt26::C0(_, _, _, h5059) => {
                                        h5059
                                    }
                                }, Str::lit("abstract"), Str::lit("true")) {
                                    true
                                } else {
                                    f744(match v1.clone() {
                                        Adt25::C0(h5060, _) => {
                                            h5060
                                        }
                                    }, v8.clone())?
                                } {
                                    Ok(Err::<Adt40, Bytes>(f595(f257(f8(Str::lit("the element ")), v6, f8(Str::lit(" heads a substitution group, so its particle also admits the members of that group, which is not translated (XSD 1.1 Part 1, section 3.3.6.4)"))), v2)))
                                } else {
                                    let m5064 = f591(v5, v1, f125({
                                        let (_, h5061) = v8.clone();
                                        h5061
                                    })?, match v9.clone() {
                                        Adt26::C0(_, h5062, _, _) => {
                                            h5062
                                        }
                                    }, match v9 {
                                        Adt26::C0(_, _, _, h5063) => {
                                            h5063
                                        }
                                    })?;
                                    match m5064 {
                                        Ok(v11) => {
                                            Ok(Ok::<Adt40, Bytes>(Adt40::C0({
                                                let (_, h5065) = v8;
                                                h5065
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
                        Ok(Err::<Adt40, Bytes>(f595(v7, v2)))
                    }
                }
            }
        }
    }
}

pub fn f737(v0: Adt27, v1: Bytes) -> Option<Bytes> {
    let v2: Bytes = f379(v0.clone(), Str::lit("maxOccurs"), Str::lit("1"));
    if f32(v2.clone(), f8(Str::lit("0"))) {
        None::<Bytes>
    } else {
        if f32(v2, f8(Str::lit("1"))) {
            if f668(v0, Str::lit("minOccurs"), Str::lit("0")) {
                Some(f715(Str::lit("Option"), v1))
            } else {
                Some(v1)
            }
        } else {
            Some(f715(Str::lit("List"), v1))
        }
    }
}

pub fn f738(v0: Adt27) -> bool {
    if f32(f379(v0.clone(), Str::lit("minOccurs"), Str::lit("1")), f8(Str::lit("1"))) {
        f32(f379(v0, Str::lit("maxOccurs"), Str::lit("1")), f8(Str::lit("1")))
    } else {
        false
    }
}

pub fn f739(v0: Adt36, v1: Bytes) -> R<u64> {
    if f32(v1.clone(), f8(Str::lit("sequence"))) {
        let a5071 = match v0 {
            Adt36::C0(_, _, h5070, _, _) => {
                h5070
            }
        };
        let a5072 = 1u64;
        nat_add(a5071, a5072)
    } else {
        if f32(v1, f8(Str::lit("choice"))) {
            let a5074 = match v0 {
                Adt36::C0(_, _, _, h5073, _) => {
                    h5073
                }
            };
            let a5075 = 1u64;
            nat_add(a5074, a5075)
        } else {
            let a5077 = match v0 {
                Adt36::C0(_, _, _, _, h5076) => {
                    h5076
                }
            };
            let a5078 = 1u64;
            nat_add(a5077, a5078)
        }
    }
}

pub fn f740(v0: Adt36, v1: Bytes) -> R<Adt36> {
    if f32(v1.clone(), f8(Str::lit("sequence"))) {
        Ok(Adt36::C0(match v0.clone() {
            Adt36::C0(h5079, _, _, _, _) => {
                h5079
            }
        }, match v0.clone() {
            Adt36::C0(_, h5080, _, _, _) => {
                h5080
            }
        }, {
            let a5082 = match v0.clone() {
                Adt36::C0(_, _, h5081, _, _) => {
                    h5081
                }
            };
            let a5083 = 1u64;
            nat_add(a5082, a5083)?
        }, match v0.clone() {
            Adt36::C0(_, _, _, h5084, _) => {
                h5084
            }
        }, match v0 {
            Adt36::C0(_, _, _, _, h5085) => {
                h5085
            }
        }))
    } else {
        if f32(v1, f8(Str::lit("choice"))) {
            Ok(Adt36::C0(match v0.clone() {
                Adt36::C0(h5086, _, _, _, _) => {
                    h5086
                }
            }, match v0.clone() {
                Adt36::C0(_, h5087, _, _, _) => {
                    h5087
                }
            }, match v0.clone() {
                Adt36::C0(_, _, h5088, _, _) => {
                    h5088
                }
            }, {
                let a5090 = match v0.clone() {
                    Adt36::C0(_, _, _, h5089, _) => {
                        h5089
                    }
                };
                let a5091 = 1u64;
                nat_add(a5090, a5091)?
            }, match v0 {
                Adt36::C0(_, _, _, _, h5092) => {
                    h5092
                }
            }))
        } else {
            Ok(Adt36::C0(match v0.clone() {
                Adt36::C0(h5093, _, _, _, _) => {
                    h5093
                }
            }, match v0.clone() {
                Adt36::C0(_, h5094, _, _, _) => {
                    h5094
                }
            }, match v0.clone() {
                Adt36::C0(_, _, h5095, _, _) => {
                    h5095
                }
            }, match v0.clone() {
                Adt36::C0(_, _, _, h5096, _) => {
                    h5096
                }
            }, {
                let a5098 = match v0 {
                    Adt36::C0(_, _, _, _, h5097) => {
                        h5097
                    }
                };
                let a5099 = 1u64;
                nat_add(a5098, a5099)?
            }))
        }
    }
}

pub fn f741(v0: Fn23, p5104: u64, p5105: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m5103 = (p5104, p5105);
    {
        let mut m5103 = m5103;
        loop {
            let step = {
                let (v1, v2) = m5103;
                let m5100 = v1;
                if m5100 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m5100 - 1;
                    let m5102 = {
                        let c5101: Fn23 = v0.clone();
                        c5101.apply(v2.clone())?
                    };
                    match m5102 {
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
                Ok(next) => { m5103 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f742(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f745(v0)
}

pub fn f743(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h5106, _) = v0;
        h5106
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h5107) = v0;
        h5107
    };
    let m5108 = v1;
    match m5108.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m5109 = v4;
            match m5109.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f746(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f744(p5118: List<Adt26>, v1: (Bytes, Bytes)) -> R<bool> {
    let m5117 = p5118;
    {
        let mut m5117 = m5117;
        loop {
            let step = {
                let v0 = m5117;
                let m5110 = v0;
                match m5110.uncons() {
                    None => {
                        Err::<List<Adt26>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if if f32(match v2.clone() {
                            Adt26::C0(h5111, _, _, _) => {
                                h5111
                            }
                        }, f8(Str::lit("element"))) {
                            let m5113 = f376(match v2.clone() {
                                Adt26::C0(_, _, _, h5112) => {
                                    h5112
                                }
                            }, Str::lit("substitutionGroup"));
                            match m5113 {
                                None => {
                                    false
                                }
                                Some(v4) => {
                                    f747({
                                        let m5116 = match v2 {
                                            Adt26::C0(_, _, _, h5115) => {
                                                h5115
                                            }
                                        };
                                        match m5116 {
                                            Adt27::C0(_, _, _, _, h5114) => {
                                                h5114
                                            }
                                        }
                                    }, f748(v4)?, v1.clone())?
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
                Ok(next) => { m5117 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f745(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h5119, _) = v0;
        h5119
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h5120) = v0;
        h5120
    };
    let m5121 = v1;
    match m5121.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m5122 = v4;
            match m5122.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f749(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f746(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a5127 = {
            let a5125 = {
                let a5123 = v0.clone();
                length_list(a5123)
            };
            let a5126 = {
                let a5124 = v1.clone();
                length_list(a5124)
            };
            nat_add(a5125, a5126)?
        };
        let a5128 = 1u64;
        nat_add(a5127, a5128)?
    };
    Ok(f512({
        let (_, h5129) = {
            let (_, h5130) = {
                let (h5131, _) = f750(Fn24::F751, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h5131
            };
            h5130
        };
        h5129
    }))
}

pub fn f747(v0: List<(Bytes, Bytes)>, p5139: List<Bytes>, v2: (Bytes, Bytes)) -> R<bool> {
    let m5138 = p5139;
    {
        let mut m5138 = m5138;
        loop {
            let step = {
                let v1 = m5138;
                let m5132 = v1;
                match m5132.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v3, v4)) => {
                        let m5133 = f164(v0.clone(), v3)?;
                        match m5133 {
                            Ok(v6) => {
                                if if f32({
                                    let (h5134, _) = v6;
                                    h5134
                                }, {
                                    let (h5135, _) = v2.clone();
                                    h5135
                                }) {
                                    f32({
                                        let (_, h5136) = v6;
                                        h5136
                                    }, {
                                        let (_, h5137) = v2.clone();
                                        h5137
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
                Ok(next) => { m5138 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f748(v0: Bytes) -> R<List<Bytes>> {
    f44(v0, 32u8)
}

pub fn f749(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a5144 = {
            let a5142 = {
                let a5140 = v0.clone();
                length_list(a5140)
            };
            let a5143 = {
                let a5141 = v1.clone();
                length_list(a5141)
            };
            nat_add(a5142, a5143)?
        };
        let a5145 = 1u64;
        nat_add(a5144, a5145)?
    };
    Ok(f752({
        let (_, h5146) = {
            let (_, h5147) = {
                let (h5148, _) = f753(Fn25::F754, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h5148
            };
            h5147
        };
        h5146
    }))
}

pub fn f750(v0: Fn24, p5153: u64, p5154: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m5152 = (p5153, p5154);
    {
        let mut m5152 = m5152;
        loop {
            let step = {
                let (v1, v2) = m5152;
                let m5149 = v1;
                if m5149 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m5149 - 1;
                    let m5151 = {
                        let c5150: Fn24 = v0.clone();
                        c5150.apply(v2.clone())
                    };
                    match m5151 {
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
                Ok(next) => { m5152 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f751(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f755(v0)
}

pub fn f752(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f392(Fn8::F756, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f753(v0: Fn25, p5159: u64, p5160: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m5158 = (p5159, p5160);
    {
        let mut m5158 = m5158;
        loop {
            let step = {
                let (v1, v2) = m5158;
                let m5155 = v1;
                if m5155 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m5155 - 1;
                    let m5157 = {
                        let c5156: Fn25 = v0.clone();
                        c5156.apply(v2.clone())
                    };
                    match m5157 {
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
                Ok(next) => { m5158 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f754(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f757(v0)
}

pub fn f755(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h5161, _) = v0.clone();
        h5161
    };
    let v2: List<(Bytes, ())> = {
        let (h5162, _) = {
            let (_, h5163) = v0.clone();
            h5163
        };
        h5162
    };
    let v3: List<(Bytes, ())> = {
        let (_, h5164) = {
            let (_, h5165) = v0;
            h5165
        };
        h5164
    };
    let m5166 = v1.clone();
    match m5166.uncons() {
        None => {
            let m5167 = v2;
            match m5167.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m5168 = v2.clone();
            match m5168.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f758({
                        let (h5169, _) = v8.clone();
                        h5169
                    }, {
                        let (h5170, _) = v6.clone();
                        h5170
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

pub fn f756(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f757(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h5171, _) = v0.clone();
        h5171
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h5172, _) = {
            let (_, h5173) = v0.clone();
            h5173
        };
        h5172
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h5174) = {
            let (_, h5175) = v0;
            h5175
        };
        h5174
    };
    let m5176 = v1.clone();
    match m5176.uncons() {
        None => {
            let m5177 = v2;
            match m5177.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m5178 = v2.clone();
            match m5178.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f758({
                        let (h5179, _) = v8.clone();
                        h5179
                    }, {
                        let (h5180, _) = v6.clone();
                        h5180
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

pub fn f758(v0: Bytes, v1: Bytes) -> bool {
    let a5185 = {
        let a5181 = v0;
        let a5182 = v1;
        compare_bytes(a5181, a5182)
    };
    let a5186 = {
        let a5183 = Bytes::lit(&[]);
        let a5184 = Bytes::lit(&[0x01]);
        compare_bytes(a5183, a5184)
    };
    equal(a5185, a5186)
}

pub fn run(p0: Option<Adt0>, p1: Adt3) -> R<(Adt0, Adt10)> {
    f0(p0, p1)
}
