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
    C0(bool),
    C1(bool, u64),
    C2(Bytes),
    C3(std::rc::Rc<Adt13>),
    C4(std::rc::Rc<Adt13>),
    C5,
    C6(std::rc::Rc<Adt13>, std::rc::Rc<Adt13>),
    C7(Bytes, std::rc::Rc<Adt13>, std::rc::Rc<Adt13>),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(u64, Bytes, Bytes, List<Adt15>, List<Bytes>, Adt16),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(Bytes, Adt16, u64),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(u64, Bytes, List<Bytes>, Bytes, Bytes),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(Bytes, Bytes, Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(Bytes, std::rc::Rc<Adt18>, std::rc::Rc<Adt18>),
    C1(Bytes, Bytes, std::rc::Rc<Adt18>),
    C2(Bytes),
    C3(std::rc::Rc<Adt18>, std::rc::Rc<Adt18>),
    C4,
}

#[derive(Clone)]
pub enum Adt19 {
    C0(List<Bytes>),
    C1(Bytes),
    C2(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt20 {
    C0(Bytes, Option<Bytes>, Bytes, List<(Bytes, Bytes)>, Adt18, u64),
}

#[derive(Clone)]
pub enum Adt21 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
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
    C0(Bytes, Bytes, Adt18, Adt18, List<(Bytes, Bytes)>),
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
    C0(Bytes, Adt18, Adt18, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Adt32 {
    C0(Bytes, List<Adt23>),
}

#[derive(Clone)]
pub enum Adt33 {
    C0(Adt16, List<Adt23>),
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
    F61(u8, Bytes),
    F292(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F61(k0, k1) => f61(*k0, k1.clone(), p0),
            Fn0::F292(k0) => f292(k0.clone(), p0),
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
    F100,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn2::F100 => f100(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F161(Bytes, u64, u64),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Adt21)) -> R<(u64, Adt21)> {
        match self {
            Fn3::F161(k0, k1, k2) => f161(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F186(u64, Bytes),
    F187(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn4::F186(k0, k1) => Ok(f186(*k0, k1.clone(), p0)),
            Fn4::F187(k0) => f187(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F189(Bytes),
    F228(Bytes),
    F229(Bytes),
    F237(Bytes),
    F238(Bytes),
    F276(Bytes, Bytes),
    F277(Bytes),
    F279(Bytes, u64),
    F342(Bytes),
    F390(bool, u64, Bytes),
    F420(Bytes),
    F422(Bytes),
    F431(Bytes, u64),
    F441(Bytes),
    F450(u64, Bytes),
    F508(Bytes),
    F510(Bytes),
    F633(u64, Bytes, u64),
    F636(Bytes),
    F637(Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn5::F189(k0) => f189(k0.clone(), p0),
            Fn5::F228(k0) => f228(k0.clone(), p0),
            Fn5::F229(k0) => f229(k0.clone(), p0),
            Fn5::F237(k0) => f237(k0.clone(), p0),
            Fn5::F238(k0) => f238(k0.clone(), p0),
            Fn5::F276(k0, k1) => f276(k0.clone(), k1.clone(), p0),
            Fn5::F277(k0) => f277(k0.clone(), p0),
            Fn5::F279(k0, k1) => f279(k0.clone(), *k1, p0),
            Fn5::F342(k0) => f342(k0.clone(), p0),
            Fn5::F390(k0, k1, k2) => f390(*k0, *k1, k2.clone(), p0),
            Fn5::F420(k0) => f420(k0.clone(), p0),
            Fn5::F422(k0) => f422(k0.clone(), p0),
            Fn5::F431(k0, k1) => f431(k0.clone(), *k1, p0),
            Fn5::F441(k0) => f441(k0.clone(), p0),
            Fn5::F450(k0, k1) => f450(*k0, k1.clone(), p0),
            Fn5::F508(k0) => f508(k0.clone(), p0),
            Fn5::F510(k0) => f510(k0.clone(), p0),
            Fn5::F633(k0, k1, k2) => f633(*k0, k1.clone(), *k2, p0),
            Fn5::F636(k0) => f636(k0.clone(), p0),
            Fn5::F637(k0) => f637(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F241(Bytes),
    F482(bool, Bytes),
    F516(Bytes),
    F525(bool, Bytes),
    F593(Bytes),
    F603(Bytes),
    F632(u64, Bytes),
    F640(u64, Bytes),
}

impl Fn6 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn6::F241(k0) => f241(k0.clone(), p0),
            Fn6::F482(k0, k1) => f482(*k0, k1.clone(), p0),
            Fn6::F516(k0) => f516(k0.clone(), p0),
            Fn6::F525(k0, k1) => f525(*k0, k1.clone(), p0),
            Fn6::F593(k0) => f593(k0.clone(), p0),
            Fn6::F603(k0) => f603(k0.clone(), p0),
            Fn6::F632(k0, k1) => f632(*k0, k1.clone(), p0),
            Fn6::F640(k0, k1) => f640(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F344,
}

impl Fn7 {
    pub fn apply(&self, p0: Adt30) -> R<Adt30> {
        match self {
            Fn7::F344 => f344(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F435,
    F673,
}

impl Fn8 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn8::F435 => f435(p0, p1),
            Fn8::F673 => f673(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F438,
}

impl Fn9 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn9::F438 => f438(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F538(Bytes),
}

impl Fn10 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn10::F538(k0) => f538(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F547,
}

impl Fn11 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn11::F547 => f547(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F601(u64, Bytes),
}

impl Fn12 {
    pub fn apply(&self, p0: (u64, bool)) -> R<Option<(u64, bool)>> {
        match self {
            Fn12::F601(k0, k1) => f601(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F608,
}

impl Fn13 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn13::F608 => f608(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F610,
}

impl Fn14 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn14::F610 => f610(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F643,
}

impl Fn15 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn15::F643 => f643(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F665,
}

impl Fn16 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn16::F665 => f665(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F672,
}

impl Fn17 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn17::F672 => f672(p0),
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
                        f34(v2, v4, v5, v1)
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
    Adt1::C0(f35(64u64), List::<(Bytes, List<Bytes>)>::nil(), List::<Bytes>::nil(), f36(64u64))
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
                Ok(List::cons(f37(v4)?, f23(v0, {
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
    Ok(Adt1::C0(f38(match v0.clone() {
        Adt1::C0(h154, _, _, _) => {
            h154
        }
    }, f39(v1.clone()), (v1.clone(), v2.clone())), match v0.clone() {
        Adt1::C0(_, h155, _, _) => {
            h155
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h156, _) => {
            h156
        }
    }, if f40(v1.clone(), f8(Str::lit(".json"))) {
        f41(match v0 {
            Adt1::C0(_, _, _, h157) => {
                h157
            }
        }, f39(v1.clone()), (v1, f42({
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
    let v1: List<Bytes> = f43(v0.clone(), 47u8)?;
    let v2: Bytes = f44(v1);
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
        Ok(f45(v0.clone(), 0u64, {
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
    f7(List::cons(f8(Str::lit("usage: verdant_studio ")), List::cons(f46(f47(), f8(Str::lit(" | "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))
}

pub fn f32(v0: Bytes, v1: Bytes) -> bool {
    let a182 = v0;
    let a183 = v1;
    equal(a182, a183)
}

pub fn f33(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m184 = f48(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m184.uncons() {
        None => {
            f49(v0.clone(), f50(v0, v1)?)
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f34(v0: Bytes, v1: Bytes, v2: List<Bytes>, v3: Adt1) -> R<Adt12> {
    if f32(v0.clone(), f8(Str::lit("read-webidl"))) {
        f51(v3, v1, v2)
    } else {
        if f32(v0.clone(), f8(Str::lit("read-xml"))) {
            f52(v3, v1)
        } else {
            if f32(v0.clone(), f8(Str::lit("read-xsd"))) {
                f53(v3, v1)
            } else {
                if f32(v0.clone(), f8(Str::lit("read-xsd-components"))) {
                    f54(v3, v1)
                } else {
                    if f32(v0, f8(Str::lit("read-yaml"))) {
                        f55(v3, v1)
                    } else {
                        Ok(Adt12::C4(f31()))
                    }
                }
            }
        }
    }
}

pub fn f35(v0: u64) -> List<List<(Bytes, Option<Bytes>)>> {
    let m185 = v0;
    if m185 == 0 {
        List::<List<(Bytes, Option<Bytes>)>>::nil()
    } else {
        let v1 = m185 - 1;
        List::cons(List::<(Bytes, Option<Bytes>)>::nil(), f35(v1))
    }
}

pub fn f36(v0: u64) -> List<List<(Bytes, Option<Adt2>)>> {
    let m186 = v0;
    if m186 == 0 {
        List::<List<(Bytes, Option<Adt2>)>>::nil()
    } else {
        let v1 = m186 - 1;
        List::cons(List::<(Bytes, Option<Adt2>)>::nil(), f36(v1))
    }
}

pub fn f37(v0: u8) -> R<Str> {
    let v1: u64 = f56(v0)?;
    let m195 = {
        let a193 = f57({
            let a187 = v1;
            let a188 = 16u64;
            let a189 = 0u64;
            nat_quot(a187, a188, a189)
        });
        let a194 = {
            let a190 = v1;
            let a191 = 16u64;
            let a192 = 0u64;
            nat_rem(a190, a191, a192)
        };
        index_list(a193, a194)
    };
    match m195 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f38(v0: List<List<(Bytes, Option<Bytes>)>>, v1: u64, v2: (Bytes, Option<Bytes>)) -> List<List<(Bytes, Option<Bytes>)>> {
    let m196 = v0;
    match m196.uncons() {
        None => {
            List::<List<(Bytes, Option<Bytes>)>>::nil()
        }
        Some((v3, v4)) => {
            let a197 = v1;
            let a198 = 0u64;
            if nat_eq(a197, a198) {
                List::cons(List::cons(v2, v3), v4)
            } else {
                List::cons(v3, f38(v4, {
                    let a199 = v1;
                    let a200 = 1u64;
                    nat_sub(a199, a200)
                }, v2))
            }
        }
    }
}

pub fn f39(v0: Bytes) -> u64 {
    let a202 = {
        let a201 = v0;
        length_bytes(a201)
    };
    let a203 = 64u64;
    let a204 = 0u64;
    nat_rem(a202, a203, a204)
}

pub fn f40(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a205 = v0.clone();
        length_bytes(a205)
    };
    let v3: u64 = {
        let a206 = v1.clone();
        length_bytes(a206)
    };
    let a207 = v3;
    let a208 = v2;
    if nat_le(a207, a208) {
        let m214 = {
            let a211 = v0;
            let a212 = {
                let a209 = v2;
                let a210 = v3;
                nat_sub(a209, a210)
            };
            let a213 = v3;
            slice_bytes(a211, a212, a213)
        };
        match m214 {
            None => {
                false
            }
            Some(v4) => {
                let a215 = v4;
                let a216 = v1;
                equal(a215, a216)
            }
        }
    } else {
        false
    }
}

pub fn f41(v0: List<List<(Bytes, Option<Adt2>)>>, v1: u64, v2: (Bytes, Option<Adt2>)) -> List<List<(Bytes, Option<Adt2>)>> {
    let m217 = v0;
    match m217.uncons() {
        None => {
            List::<List<(Bytes, Option<Adt2>)>>::nil()
        }
        Some((v3, v4)) => {
            let a218 = v1;
            let a219 = 0u64;
            if nat_eq(a218, a219) {
                List::cons(List::cons(v2, v3), v4)
            } else {
                List::cons(v3, f41(v4, {
                    let a220 = v1;
                    let a221 = 1u64;
                    nat_sub(a220, a221)
                }, v2))
            }
        }
    }
}

pub fn f42(v0: Bytes) -> R<Option<Adt2>> {
    let v1: Bytes = f58(v0)?;
    let m225 = f59(v1.clone(), 0u64, {
        let a223 = {
            let a222 = v1.clone();
            length_bytes(a222)
        };
        let a224 = 2u64;
        nat_add(a223, a224)?
    }, 0u64)?;
    match m225 {
        None => {
            Ok(None::<Adt2>)
        }
        Some(v2) => {
            let a228 = {
                let (_, h226) = v2;
                h226
            };
            let a229 = {
                let a227 = v1;
                length_bytes(a227)
            };
            if nat_eq(a228, a229) {
                Ok(Some({
                    let (h230, _) = v2;
                    h230
                }))
            } else {
                Ok(None::<Adt2>)
            }
        }
    }
}

pub fn f43(v0: Bytes, v1: u8) -> R<List<Bytes>> {
    let v2: u64 = {
        let a231 = v0.clone();
        length_bytes(a231)
    };
    let v3: (u64, (u64, List<Bytes>)) = {
        let (h232, _) = f60(Fn0::F61(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h232
    };
    let v4: u64 = {
        let (h233, _) = {
            let (_, h234) = v3.clone();
            h234
        };
        h233
    };
    Ok(f62(List::cons(f45(v0, v4, {
        let a235 = v2;
        let a236 = v4;
        nat_sub(a235, a236)
    }), {
        let (_, h237) = {
            let (_, h238) = v3;
            h238
        };
        h237
    })))
}

pub fn f44(p242: List<Bytes>) -> Bytes {
    let m241 = p242;
    {
        let mut m241 = m241;
        loop {
            let step = {
                let v0 = m241;
                let m239 = v0;
                match m239.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v1, v2)) => {
                        let m240 = v2.clone();
                        match m240.uncons() {
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
                Ok(next) => { m241 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f45(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m246 = {
        let a243 = v0;
        let a244 = v1;
        let a245 = v2;
        slice_bytes(a243, a244, a245)
    };
    match m246 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f46(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m247 = f63(Fn1::F64(v1), None::<Bytes>, v0);
    match m247 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f47() -> List<Bytes> {
    List::cons(f8(Str::lit("admit JSONL")), List::cons(f8(Str::lit("admit-encoding STAGED")), List::cons(f8(Str::lit("admit-translated FILE NAME read-OP ARG... CITATION...")), List::cons(f8(Str::lit("agenda")), List::cons(f8(Str::lit("build")), List::cons(f8(Str::lit("check")), List::cons(f8(Str::lit("ingest-authority BINDING")), List::cons(f8(Str::lit("ingest-index CSV")), List::cons(f8(Str::lit("merge-gate RULES")), List::cons(f8(Str::lit("next [SKIP...]")), List::cons(f8(Str::lit("propose-encoding SCHEMA NAME CITATION...")), List::cons(f8(Str::lit("publish")), List::cons(f8(Str::lit("read-webidl FILE [IMPORT...]")), List::cons(f8(Str::lit("read-xml FILE")), List::cons(f8(Str::lit("read-xsd FILE")), List::cons(f8(Str::lit("read-xsd-components FILE")), List::cons(f8(Str::lit("read-yaml FILE")), List::cons(f8(Str::lit("record-decision JSONL")), List::cons(f8(Str::lit("record-error REFUSAL TIME RUN OPERATION...")), List::cons(f8(Str::lit("stage-admissions JSONL")), List::cons(f8(Str::lit("stage-bindings JSONL")), List::cons(f8(Str::lit("stage-decisions JSONL")), List::cons(f8(Str::lit("stage-encodings JSONL")), List::cons(f8(Str::lit("stage-engine")), List::cons(f8(Str::lit("stage-mappings JSONL")), List::cons(f8(Str::lit("stage-matrix JSONL")), List::cons(f8(Str::lit("verify-translations ADMISSIONS")), List::<Bytes>::nil())))))))))))))))))))))))))))
}

pub fn f48(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m248 = v1;
    match m248.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m249 = f65(f66(v0.clone(), v2.clone()), v2.clone());
            match m249 {
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

pub fn f49(v0: Adt1, p258: List<Adt13>) -> R<Adt12> {
    let m257 = p258;
    {
        let mut m257 = m257;
        loop {
            let step = {
                let v1 = m257;
                let m250 = v1;
                match m250.uncons() {
                    None => {
                        Err::<List<Adt13>, R<Adt12>>(Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f8(Str::lit("translations: replayed\u{a}")))))
                    }
                    Some((v2, v3)) => {
                        let m251 = f67(v2.clone(), f8(Str::lit("translation")));
                        match m251 {
                            None => {
                                Ok::<List<Adt13>, R<Adt12>>(v3)
                            }
                            Some(v4) => {
                                let v6: Bytes = {
                                    let m252 = f68(f67(v2, f8(Str::lit("construct"))));
                                    match m252 {
                                        None => {
                                            Bytes::lit(&[])
                                        }
                                        Some(v5) => {
                                            v5
                                        }
                                    }
                                };
                                let v8: Bytes = {
                                    let m253 = f68(f67(v4.clone(), f8(Str::lit("output_sha256"))));
                                    match m253 {
                                        None => {
                                            Bytes::lit(&[])
                                        }
                                        Some(v7) => {
                                            v7
                                        }
                                    }
                                };
                                let m254 = f69(f70(f67(v4, f8(Str::lit("command")))));
                                match m254.uncons() {
                                    None => {
                                        Err::<List<Adt13>, R<Adt12>>(Ok(Adt12::C4(f7(List::cons(v6, List::cons(f8(Str::lit(" records no reader command\u{a}")), List::<Bytes>::nil()))))))
                                    }
                                    Some((v9, v10)) => {
                                        let m255 = v10;
                                        match m255.uncons() {
                                            None => {
                                                Err::<List<Adt13>, R<Adt12>>(Ok(Adt12::C4(f7(List::cons(v6, List::cons(f8(Str::lit(" records a reader command without a file\u{a}")), List::<Bytes>::nil()))))))
                                            }
                                            Some((v11, v12)) => {
                                                let m256 = f34(v9, v11, v12, v0.clone())?;
                                                match m256 {
                                                    Adt12::C0(v13) => {
                                                        Err::<List<Adt13>, R<Adt12>>(Ok(Adt12::C0(v13)))
                                                    }
                                                    Adt12::C1(v14) => {
                                                        Err::<List<Adt13>, R<Adt12>>(Ok(Adt12::C1(v14)))
                                                    }
                                                    Adt12::C2(_, _, _) => {
                                                        Err::<List<Adt13>, R<Adt12>>(Ok(Adt12::C4(f7(List::cons(v6, List::cons(f8(Str::lit(": its reader command does not print a translation\u{a}")), List::<Bytes>::nil()))))))
                                                    }
                                                    Adt12::C3(_, v20) => {
                                                        if f32(f71(v20.clone())?, v8.clone()) {
                                                            Ok::<List<Adt13>, R<Adt12>>(v3)
                                                        } else {
                                                            Err::<List<Adt13>, R<Adt12>>(Ok(Adt12::C4(f7(List::cons(v6, List::cons(f8(Str::lit(": its reader command now prints output with sha256 ")), List::cons(f71(v20)?, List::cons(f8(Str::lit(", and the admission records ")), List::cons(v8, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))))
                                                        }
                                                    }
                                                    Adt12::C4(v18) => {
                                                        Err::<List<Adt13>, R<Adt12>>(Ok(Adt12::C4(f7(List::cons(v6, List::cons(f8(Str::lit(": its reader command now refuses: ")), List::cons(v18, List::<Bytes>::nil())))))))
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
                Ok(next) => { m257 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f50(v0: Adt1, v1: Bytes) -> R<List<Adt13>> {
    Ok(f70(f72(v0, v1)?))
}

pub fn f51(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<Adt12> {
    let m259 = f48(v0.clone(), List::cons(v1.clone(), v2.clone()));
    match m259.uncons() {
        None => {
            let m260 = f73(f74(v0.clone(), v1), f75(v0, v2))?;
            match m260 {
                Ok(v6) => {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("-- ")), List::cons(f76({
                        let (h261, _) = v6;
                        h261
                    })?, List::cons(f8(Str::lit(" Web IDL definitions translated\u{a}")), List::cons({
                        let (_, h262) = v6;
                        h262
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

pub fn f52(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m263 = f48(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m263.uncons() {
        None => {
            let m264 = f77(f74(v0, v1))?;
            match m264 {
                Ok(v5) => {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("well-formed XML: root ")), List::cons(f78(v5.clone()), List::cons(f8(Str::lit(", ")), List::cons(f76(f79(v5)?)?, List::cons(f8(Str::lit(" elements\u{a}")), List::<Bytes>::nil()))))))))
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

pub fn f53(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m265 = f48(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m265.uncons() {
        None => {
            let m266 = f80(f74(v0, v1))?;
            match m266 {
                Ok(v5) => {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("-- ")), List::cons(f76({
                        let (h267, _) = v5;
                        h267
                    })?, List::cons(f8(Str::lit(" simple types\u{a}")), List::cons({
                        let (_, h268) = v5;
                        h268
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

pub fn f54(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m269 = f48(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::<Bytes>::nil())));
    match m269.uncons() {
        None => {
            let m270 = f81(f82(v0.clone()), f74(v0, f8(Str::lit("model/authorities.toml"))), v1)?;
            match m270 {
                Adt19::C0(v4) => {
                    Ok(Adt12::C0(v4))
                }
                Adt19::C1(v5) => {
                    Ok(Adt12::C4(f7(List::cons(f8(Str::lit("XSD not translated: ")), List::cons(v5, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
                Adt19::C2(v6, v7) => {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("-- ")), List::cons(f76(v6)?, List::cons(f8(Str::lit(" declarations\u{a}")), List::cons(v7, List::<Bytes>::nil())))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f55(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m271 = f48(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m271.uncons() {
        None => {
            let m272 = f83(f74(v0, v1))?;
            match m272 {
                Ok(v5) => {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f84(v5)?))
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

pub fn f56(v0: u8) -> R<u64> {
    let v1: u8 = v0;
    let a317 = {
        let a293 = {
            let a281 = {
                let a275 = {
                    let a273 = v1;
                    let a274 = 128u8;
                    fixed_u8::bit_and(a273, a274)
                };
                let a276 = 128u8;
                if equal(a275, a276) {
                    128u64
                } else {
                    0u64
                }
            };
            let a282 = {
                let a279 = {
                    let a277 = v1;
                    let a278 = 64u8;
                    fixed_u8::bit_and(a277, a278)
                };
                let a280 = 64u8;
                if equal(a279, a280) {
                    64u64
                } else {
                    0u64
                }
            };
            nat_add(a281, a282)?
        };
        let a294 = {
            let a291 = {
                let a285 = {
                    let a283 = v1;
                    let a284 = 32u8;
                    fixed_u8::bit_and(a283, a284)
                };
                let a286 = 32u8;
                if equal(a285, a286) {
                    32u64
                } else {
                    0u64
                }
            };
            let a292 = {
                let a289 = {
                    let a287 = v1;
                    let a288 = 16u8;
                    fixed_u8::bit_and(a287, a288)
                };
                let a290 = 16u8;
                if equal(a289, a290) {
                    16u64
                } else {
                    0u64
                }
            };
            nat_add(a291, a292)?
        };
        nat_add(a293, a294)?
    };
    let a318 = {
        let a315 = {
            let a303 = {
                let a297 = {
                    let a295 = v1;
                    let a296 = 8u8;
                    fixed_u8::bit_and(a295, a296)
                };
                let a298 = 8u8;
                if equal(a297, a298) {
                    8u64
                } else {
                    0u64
                }
            };
            let a304 = {
                let a301 = {
                    let a299 = v1;
                    let a300 = 4u8;
                    fixed_u8::bit_and(a299, a300)
                };
                let a302 = 4u8;
                if equal(a301, a302) {
                    4u64
                } else {
                    0u64
                }
            };
            nat_add(a303, a304)?
        };
        let a316 = {
            let a313 = {
                let a307 = {
                    let a305 = v1;
                    let a306 = 2u8;
                    fixed_u8::bit_and(a305, a306)
                };
                let a308 = 2u8;
                if equal(a307, a308) {
                    2u64
                } else {
                    0u64
                }
            };
            let a314 = {
                let a311 = {
                    let a309 = v1;
                    let a310 = 1u8;
                    fixed_u8::bit_and(a309, a310)
                };
                let a312 = 1u8;
                if equal(a311, a312) {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a313, a314)?
        };
        nat_add(a315, a316)?
    };
    nat_add(a317, a318)
}

pub fn f57(v0: u64) -> List<Str> {
    let a319 = v0;
    let a320 = 0u64;
    if nat_eq(a319, a320) {
        f85()
    } else {
        let a321 = v0;
        let a322 = 1u64;
        if nat_eq(a321, a322) {
            f86()
        } else {
            let a323 = v0;
            let a324 = 2u64;
            if nat_eq(a323, a324) {
                f87()
            } else {
                let a325 = v0;
                let a326 = 3u64;
                if nat_eq(a325, a326) {
                    f88()
                } else {
                    let a327 = v0;
                    let a328 = 4u64;
                    if nat_eq(a327, a328) {
                        f89()
                    } else {
                        let a329 = v0;
                        let a330 = 5u64;
                        if nat_eq(a329, a330) {
                            f90()
                        } else {
                            let a331 = v0;
                            let a332 = 6u64;
                            if nat_eq(a331, a332) {
                                f91()
                            } else {
                                let a333 = v0;
                                let a334 = 7u64;
                                if nat_eq(a333, a334) {
                                    f92()
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
    f93(v0.clone(), 0u64, {
        let a335 = v0;
        length_bytes(a335)
    }, false, false, 0u64, Bytes::lit(&[]))
}

pub fn f59(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt2, u64)>> {
    let m336 = v2;
    if m336 == 0 {
        Ok(None::<(Adt2, u64)>)
    } else {
        let v4 = m336 - 1;
        let v5: u64 = f94(v0.clone(), v1)?;
        let a337 = v3;
        let a338 = 1u64;
        if nat_eq(a337, a338) {
            let m339 = f59(v0.clone(), v1, v4, 0u64)?;
            match m339 {
                None => {
                    Ok(None::<(Adt2, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f94(v0.clone(), {
                        let (_, h340) = v6.clone();
                        h340
                    })?;
                    let a341 = v7;
                    let a342 = 44u64;
                    if nat_eq(a341, a342) {
                        let m346 = f59(v0, {
                            let a344 = {
                                let (_, h343) = v6;
                                h343
                            };
                            let a345 = 1u64;
                            nat_add(a344, a345)?
                        }, v4, 1u64)?;
                        match m346 {
                            None => {
                                Ok(None::<(Adt2, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt2::C9(std::rc::Rc::new({
                                    let (h347, _) = v6;
                                    h347
                                }), std::rc::Rc::new({
                                    let (h348, _) = v8;
                                    h348
                                })), {
                                    let (_, h349) = v8;
                                    h349
                                })))
                            }
                        }
                    } else {
                        let a350 = v7;
                        let a351 = 93u64;
                        if nat_eq(a350, a351) {
                            Ok(Some((Adt2::C9(std::rc::Rc::new({
                                let (h352, _) = v6;
                                h352
                            }), std::rc::Rc::new(Adt2::C8)), {
                                let a354 = {
                                    let (_, h353) = v6;
                                    h353
                                };
                                let a355 = 1u64;
                                nat_add(a354, a355)?
                            })))
                        } else {
                            Ok(None::<(Adt2, u64)>)
                        }
                    }
                }
            }
        } else {
            let a356 = v3;
            let a357 = 2u64;
            if nat_eq(a356, a357) {
                let m360 = f95(v0.clone(), {
                    let a358 = v1;
                    let a359 = 1u64;
                    nat_add(a358, a359)?
                })?;
                match m360 {
                    None => {
                        Ok(None::<(Adt2, u64)>)
                    }
                    Some(v9) => {
                        if f96({
                            let a361 = v5;
                            let a362 = 34u64;
                            nat_eq(a361, a362)
                        }, {
                            let a364 = f94(v0.clone(), {
                                let (h363, _) = v9.clone();
                                h363
                            })?;
                            let a365 = 58u64;
                            nat_eq(a364, a365)
                        }) {
                            let m369 = f59(v0.clone(), {
                                let a367 = {
                                    let (h366, _) = v9;
                                    h366
                                };
                                let a368 = 1u64;
                                nat_add(a367, a368)?
                            }, v4, 0u64)?;
                            match m369 {
                                None => {
                                    Ok(None::<(Adt2, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f94(v0.clone(), {
                                        let (_, h370) = v10.clone();
                                        h370
                                    })?;
                                    let a371 = v11;
                                    let a372 = 44u64;
                                    if nat_eq(a371, a372) {
                                        let m376 = f59(v0, {
                                            let a374 = {
                                                let (_, h373) = v10;
                                                h373
                                            };
                                            let a375 = 1u64;
                                            nat_add(a374, a375)?
                                        }, v4, 2u64)?;
                                        match m376 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt2::C10({
                                                    let (_, h377) = v9;
                                                    h377
                                                }, std::rc::Rc::new({
                                                    let (h378, _) = v10;
                                                    h378
                                                }), std::rc::Rc::new({
                                                    let (h379, _) = v12;
                                                    h379
                                                })), {
                                                    let (_, h380) = v12;
                                                    h380
                                                })))
                                            }
                                        }
                                    } else {
                                        let a381 = v11;
                                        let a382 = 125u64;
                                        if nat_eq(a381, a382) {
                                            Ok(Some((Adt2::C10({
                                                let (_, h383) = v9;
                                                h383
                                            }, std::rc::Rc::new({
                                                let (h384, _) = v10;
                                                h384
                                            }), std::rc::Rc::new(Adt2::C8)), {
                                                let a386 = {
                                                    let (_, h385) = v10;
                                                    h385
                                                };
                                                let a387 = 1u64;
                                                nat_add(a386, a387)?
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
                let a388 = v5;
                let a389 = 34u64;
                if nat_eq(a388, a389) {
                    let m392 = f95(v0, {
                        let a390 = v1;
                        let a391 = 1u64;
                        nat_add(a390, a391)?
                    })?;
                    match m392 {
                        None => {
                            Ok(None::<(Adt2, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt2::C5({
                                let (_, h393) = v13;
                                h393
                            }), {
                                let (h394, _) = v13;
                                h394
                            })))
                        }
                    }
                } else {
                    if f32(f45(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                        Ok(Some((Adt2::C0, {
                            let a395 = v1;
                            let a396 = 4u64;
                            nat_add(a395, a396)?
                        })))
                    } else {
                        if f32(f45(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                            Ok(Some((Adt2::C2(true), {
                                let a397 = v1;
                                let a398 = 4u64;
                                nat_add(a397, a398)?
                            })))
                        } else {
                            if f32(f45(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                                Ok(Some((Adt2::C2(false), {
                                    let a399 = v1;
                                    let a400 = 5u64;
                                    nat_add(a399, a400)?
                                })))
                            } else {
                                let a401 = v5;
                                let a402 = 91u64;
                                if nat_eq(a401, a402) {
                                    let a405 = f94(v0.clone(), {
                                        let a403 = v1;
                                        let a404 = 1u64;
                                        nat_add(a403, a404)?
                                    })?;
                                    let a406 = 93u64;
                                    if nat_eq(a405, a406) {
                                        Ok(Some((Adt2::C6(std::rc::Rc::new(Adt2::C8)), {
                                            let a407 = v1;
                                            let a408 = 2u64;
                                            nat_add(a407, a408)?
                                        })))
                                    } else {
                                        let m411 = f59(v0, {
                                            let a409 = v1;
                                            let a410 = 1u64;
                                            nat_add(a409, a410)?
                                        }, v4, 1u64)?;
                                        match m411 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v14) => {
                                                Ok(Some((Adt2::C6(std::rc::Rc::new({
                                                    let (h412, _) = v14;
                                                    h412
                                                })), {
                                                    let (_, h413) = v14;
                                                    h413
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let a414 = v5;
                                    let a415 = 123u64;
                                    if nat_eq(a414, a415) {
                                        let a418 = f94(v0.clone(), {
                                            let a416 = v1;
                                            let a417 = 1u64;
                                            nat_add(a416, a417)?
                                        })?;
                                        let a419 = 125u64;
                                        if nat_eq(a418, a419) {
                                            Ok(Some((Adt2::C7(std::rc::Rc::new(Adt2::C8)), {
                                                let a420 = v1;
                                                let a421 = 2u64;
                                                nat_add(a420, a421)?
                                            })))
                                        } else {
                                            let m424 = f59(v0, {
                                                let a422 = v1;
                                                let a423 = 1u64;
                                                nat_add(a422, a423)?
                                            }, v4, 2u64)?;
                                            match m424 {
                                                None => {
                                                    Ok(None::<(Adt2, u64)>)
                                                }
                                                Some(v15) => {
                                                    Ok(Some((Adt2::C7(std::rc::Rc::new({
                                                        let (h425, _) = v15;
                                                        h425
                                                    })), {
                                                        let (_, h426) = v15;
                                                        h426
                                                    })))
                                                }
                                            }
                                        }
                                    } else {
                                        let m427 = f97(v0, v1)?;
                                        match m427 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v16) => {
                                                Ok(Some(({
                                                    let (_, h428) = v16;
                                                    h428
                                                }, {
                                                    let (h429, _) = v16;
                                                    h429
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

pub fn f60(v0: Fn0, p434: u64, p435: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m433 = (p434, p435);
    {
        let mut m433 = m433;
        loop {
            let step = {
                let (v1, v2) = m433;
                let m430 = v1;
                if m430 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m430 - 1;
                    let m432 = {
                        let c431: Fn0 = v0.clone();
                        c431.apply(v2.clone())?
                    };
                    match m432 {
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
                Ok(next) => { m433 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f61(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f98(v1, v0, v2)
}

pub fn f62(v0: List<Bytes>) -> List<Bytes> {
    f99(Fn2::F100, List::<Bytes>::nil(), v0)
}

pub fn f63(v0: Fn1, p439: Option<Bytes>, p440: List<Bytes>) -> Option<Bytes> {
    let m438 = (p439, p440);
    {
        let mut m438 = m438;
        loop {
            let step = {
                let (v1, v2) = m438;
                let m436 = v2;
                match m436.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c437: Fn1 = v0.clone();
                            c437.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m438 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f64(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m441 = v1;
    match m441 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a444 = {
                    let a442 = v3;
                    let a443 = v0;
                    append_bytes(a442, a443)
                };
                let a445 = v2;
                append_bytes(a444, a445)
            })
        }
    }
}

pub fn f65(p450: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m449 = p450;
    {
        let mut m449 = m449;
        loop {
            let step = {
                let v0 = m449;
                let m446 = v0;
                match m446.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h447, _) = v2;
                            h447
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h448) = v2;
                                h448
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m449 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f66(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Bytes>)> {
    let m454 = {
        let a452 = match v0 {
            Adt1::C0(h451, _, _, _) => {
                h451
            }
        };
        let a453 = f39(v1);
        index_list(a452, a453)
    };
    match m454 {
        None => {
            List::<(Bytes, Option<Bytes>)>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f67(p460: Adt13, v1: Bytes) -> Option<Adt13> {
    let m459 = p460;
    {
        let mut m459 = m459;
        loop {
            let step = {
                let v0 = m459;
                let m455 = v0;
                match m455 {
                    Adt13::C0(_) => {
                        Err::<Adt13, Option<Adt13>>(None::<Adt13>)
                    }
                    Adt13::C1(_, _) => {
                        Err::<Adt13, Option<Adt13>>(None::<Adt13>)
                    }
                    Adt13::C2(_) => {
                        Err::<Adt13, Option<Adt13>>(None::<Adt13>)
                    }
                    Adt13::C3(_) => {
                        Err::<Adt13, Option<Adt13>>(None::<Adt13>)
                    }
                    Adt13::C4(r456) => {
                        let v5 = (*r456).clone();
                        Ok::<Adt13, Option<Adt13>>(v5)
                    }
                    Adt13::C5 => {
                        Err::<Adt13, Option<Adt13>>(None::<Adt13>)
                    }
                    Adt13::C6(_, _) => {
                        Err::<Adt13, Option<Adt13>>(None::<Adt13>)
                    }
                    Adt13::C7(v2, r457, r458) => {
                        let v3 = (*r457).clone();
                        let v4 = (*r458).clone();
                        if f32(v2, v1.clone()) {
                            Err::<Adt13, Option<Adt13>>(Some(v3))
                        } else {
                            Ok::<Adt13, Option<Adt13>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m459 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f68(v0: Option<Adt13>) -> Option<Bytes> {
    let m461 = v0;
    match m461 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m462 = v1;
            match m462 {
                Adt13::C0(_) => {
                    None::<Bytes>
                }
                Adt13::C1(_, _) => {
                    None::<Bytes>
                }
                Adt13::C2(v2) => {
                    Some(v2)
                }
                Adt13::C3(_) => {
                    None::<Bytes>
                }
                Adt13::C4(_) => {
                    None::<Bytes>
                }
                Adt13::C5 => {
                    None::<Bytes>
                }
                Adt13::C6(_, _) => {
                    None::<Bytes>
                }
                Adt13::C7(_, _, _) => {
                    None::<Bytes>
                }
            }
        }
    }
}

pub fn f69(v0: List<Adt13>) -> List<Bytes> {
    let m463 = v0;
    match m463.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m464 = f68(Some(v1));
            match m464 {
                None => {
                    f69(v2)
                }
                Some(v3) => {
                    List::cons(v3, f69(v2))
                }
            }
        }
    }
}

pub fn f70(v0: Option<Adt13>) -> List<Adt13> {
    let m465 = v0;
    match m465 {
        None => {
            List::<Adt13>::nil()
        }
        Some(v1) => {
            f101(v1)
        }
    }
}

pub fn f71(v0: Bytes) -> R<Bytes> {
    Ok(f102(f103(v0)?))
}

pub fn f72(v0: Adt1, v1: Bytes) -> R<Option<Adt13>> {
    let m466 = f104(v0, v1)?;
    match m466 {
        None => {
            Ok(None::<Adt13>)
        }
        Some(v2) => {
            Ok(f105(v2))
        }
    }
}

pub fn f73(v0: Bytes, v1: List<Bytes>) -> R<Result<(u64, Bytes), Bytes>> {
    let m467 = f106(v0)?;
    match m467 {
        Ok(v3) => {
            let m468 = f107(v1)?;
            match m468 {
                Ok(v5) => {
                    let v6: List<Adt14> = f108(v3.clone(), v5);
                    let v7: List<Adt17> = f109({
                        let a470 = {
                            let a469 = v6.clone();
                            length_list(a469)
                        };
                        let a471 = 1u64;
                        nat_add(a470, a471)?
                    }, f110(v6.clone(), v6.clone())?)?;
                    let v8: List<Bytes> = f111({
                        let a473 = {
                            let a472 = v7.clone();
                            length_list(a472)
                        };
                        let a474 = 1u64;
                        nat_add(a473, a474)?
                    }, f112(v3.clone()), List::<Bytes>::nil(), v7.clone());
                    let v9: List<(Bytes, (List<Bytes>, Bytes))> = f113(v7.clone(), v8);
                    let v10: (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>) = f114({
                        let a476 = {
                            let a475 = v9.clone();
                            length_list(a475)
                        };
                        let a477 = 1u64;
                        nat_add(a476, a477)?
                    }, v9, List::<Bytes>::nil(), List::<Bytes>::nil());
                    Ok(Ok::<(u64, Bytes), Bytes>(({
                        let a479 = {
                            let (h478, _) = v10.clone();
                            h478
                        };
                        length_list(a479)
                    }, f7(List::cons(f7(f115(v3, v7, v6)), List::cons(f7(f116({
                        let (_, h480) = v10;
                        h480
                    })), List::cons(f8(Str::lit("-- operations, constants and the other members that are not attributes are behavior, not data, and are not translated (Web IDL 2.5)\u{a}")), List::cons(f117(f118({
                        let (h481, _) = v10;
                        h481
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

pub fn f74(v0: Adt1, v1: Bytes) -> Bytes {
    let m482 = f65(f66(v0, v1.clone()), v1);
    match m482 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m483 = v2;
            match m483 {
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

pub fn f75(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m484 = v1;
    match m484.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f74(v0.clone(), v2), f75(v0, v3))
        }
    }
}

pub fn f76(v0: u64) -> R<Bytes> {
    f119(f120(), 10u64, v0)
}

pub fn f77(v0: Bytes) -> R<Result<Adt18, Bytes>> {
    let v1: Bytes = if f121(v0.clone(), f8(Str::lit("\u{feff}")), 0u64) {
        f45(v0.clone(), 3u64, {
            let a486 = {
                let a485 = v0;
                length_bytes(a485)
            };
            let a487 = 3u64;
            nat_sub(a486, a487)
        })
    } else {
        v0
    };
    let v2: Bytes = f122(v1)?;
    let v3: u64 = {
        let a488 = v2.clone();
        length_bytes(a488)
    };
    let v4: u64 = f123(v2.clone())?;
    let a489 = v4;
    let a490 = v3;
    if nat_lt(a489, a490) {
        Ok(f124(f125(Str::lit("the input is not a sequence of Chars in UTF-8 (XML 1.0 [2], section 4.3.3)"), v4)?))
    } else {
        let m491 = f126(v2.clone())?;
        match m491 {
            Ok(v6) => {
                let m494 = f127(v2.clone(), v6, {
                    let a492 = v3;
                    let a493 = 2u64;
                    nat_add(a492, a493)?
                })?;
                match m494 {
                    Ok(v8) => {
                        if f128(v2.clone(), v8, Str::lit("<!DOCTYPE")) {
                            Ok(f124(f125(Str::lit("a document type declaration is not read; Verdant Studio reads documents that have none (XML 1.0 [28])"), v8)?))
                        } else {
                            let a495 = f129(v2.clone(), v8)?;
                            let a496 = 60u64;
                            if nat_eq(a495, a496) {
                                let m499 = f130(v2.clone(), v8, {
                                    let a497 = v3;
                                    let a498 = 2u64;
                                    nat_add(a497, a498)?
                                }, 0u64)?;
                                match m499 {
                                    Ok(v10) => {
                                        let m503 = f127(v2, {
                                            let (_, h500) = v10;
                                            h500
                                        }, {
                                            let a501 = v3;
                                            let a502 = 2u64;
                                            nat_add(a501, a502)?
                                        })?;
                                        match m503 {
                                            Ok(v12) => {
                                                let a504 = v12;
                                                let a505 = v3;
                                                if nat_eq(a504, a505) {
                                                    Ok(Ok::<Adt18, Bytes>({
                                                        let (h506, _) = v10;
                                                        h506
                                                    }))
                                                } else {
                                                    Ok(f124(f125(Str::lit("only comments, processing instructions and white space may follow the root element (XML 1.0 [1], [27])"), v12)?))
                                                }
                                            }
                                            Err(v11) => {
                                                Ok(f124(v11))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Ok(f124(v9))
                                    }
                                }
                            } else {
                                Ok(f124(f125(Str::lit("a document must have exactly one root element (XML 1.0 [1])"), v8)?))
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(f124(v7))
                    }
                }
            }
            Err(v5) => {
                Ok(f124(v5))
            }
        }
    }
}

pub fn f78(v0: Adt18) -> Bytes {
    let m507 = v0;
    match m507 {
        Adt18::C0(v1, _, _) => {
            v1
        }
        Adt18::C1(_, _, _) => {
            Bytes::lit(&[])
        }
        Adt18::C2(_) => {
            Bytes::lit(&[])
        }
        Adt18::C3(_, _) => {
            Bytes::lit(&[])
        }
        Adt18::C4 => {
            Bytes::lit(&[])
        }
    }
}

pub fn f79(v0: Adt18) -> R<u64> {
    let m508 = v0;
    match m508 {
        Adt18::C0(_, _, r513) => {
            let v3 = (*r513).clone();
            let a509 = 1u64;
            let a510 = f79(v3)?;
            nat_add(a509, a510)
        }
        Adt18::C1(_, _, _) => {
            Ok(0u64)
        }
        Adt18::C2(_) => {
            Ok(0u64)
        }
        Adt18::C3(r514, r515) => {
            let v8 = (*r514).clone();
            let v9 = (*r515).clone();
            let a511 = f79(v8)?;
            let a512 = f79(v9)?;
            nat_add(a511, a512)
        }
        Adt18::C4 => {
            Ok(0u64)
        }
    }
}

pub fn f80(v0: Bytes) -> R<Result<(u64, Bytes), Bytes>> {
    let m516 = f77(v0)?;
    match m516 {
        Ok(v2) => {
            let m517 = v2;
            match m517 {
                Adt18::C0(v3, r529, r530) => {
                    let v4 = (*r529).clone();
                    let v5 = (*r530).clone();
                    let m518 = f132(v4.clone(), List::<(Bytes, Bytes)>::nil());
                    match m518 {
                        Ok(v7) => {
                            let m519 = f133(v7.clone(), v3)?;
                            match m519 {
                                Ok(v9) => {
                                    if f134(v9, Str::lit("schema")) {
                                        let v11: Bytes = {
                                            let m520 = f135(v4, f8(Str::lit("targetNamespace")));
                                            match m520 {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some(v10) => {
                                                    v10
                                                }
                                            }
                                        };
                                        let m521 = f136(v5, v7, v11)?;
                                        match m521 {
                                            Ok(v13) => {
                                                let v14: List<Bytes> = f137(v13.clone());
                                                let m522 = f138(v14.clone());
                                                match m522 {
                                                    None => {
                                                        let m523 = f139(v13.clone(), v14.clone());
                                                        match m523 {
                                                            None => {
                                                                let m527 = f140({
                                                                    let a525 = {
                                                                        let a524 = v14.clone();
                                                                        length_list(a524)
                                                                    };
                                                                    let a526 = 1u64;
                                                                    nat_add(a525, a526)?
                                                                }, v13, List::<Bytes>::nil(), List::<Bytes>::nil());
                                                                match m527 {
                                                                    Ok(v18) => {
                                                                        Ok(Ok::<(u64, Bytes), Bytes>(({
                                                                            let a528 = v14;
                                                                            length_list(a528)
                                                                        }, f117(f118(v18, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))))))
                                                                    }
                                                                    Err(v17) => {
                                                                        Ok(f131(v17))
                                                                    }
                                                                }
                                                            }
                                                            Some(v16) => {
                                                                Ok(f131(f7(List::cons(f8(Str::lit("the type ")), List::cons(v16, List::cons(f8(Str::lit(" is not a simple type of this schema document")), List::<Bytes>::nil()))))))
                                                            }
                                                        }
                                                    }
                                                    Some(v15) => {
                                                        Ok(f131(f7(List::cons(f8(Str::lit("two simple types derive the type name ")), List::cons(v15, List::<Bytes>::nil())))))
                                                    }
                                                }
                                            }
                                            Err(v12) => {
                                                Ok(f131(v12))
                                            }
                                        }
                                    } else {
                                        Ok(f131(f8(Str::lit("the root element must be xs:schema in the XML Schema namespace (XSD 1.1 Part 1, section 3.17.2)"))))
                                    }
                                }
                                Err(v8) => {
                                    Ok(f131(v8))
                                }
                            }
                        }
                        Err(v6) => {
                            Ok(f131(v6))
                        }
                    }
                }
                Adt18::C1(_, _, _) => {
                    Ok(f131(f8(Str::lit("the document has no root element"))))
                }
                Adt18::C2(_) => {
                    Ok(f131(f8(Str::lit("the document has no root element"))))
                }
                Adt18::C3(_, _) => {
                    Ok(f131(f8(Str::lit("the document has no root element"))))
                }
                Adt18::C4 => {
                    Ok(f131(f8(Str::lit("the document has no root element"))))
                }
            }
        }
        Err(v1) => {
            Ok(f131(f7(List::cons(f8(Str::lit("not well-formed XML: ")), List::cons(v1, List::<Bytes>::nil())))))
        }
    }
}

pub fn f81(v0: List<(Bytes, Option<Bytes>)>, v1: Bytes, v2: Bytes) -> R<Adt19> {
    let v3: List<(Bytes, Bytes)> = f141(v1)?;
    f142({
        let a532 = {
            let a531 = v3.clone();
            length_list(a531)
        };
        let a533 = 2u64;
        nat_add(a532, a533)?
    }, v0, v3, List::cons((v2, (f8(Str::lit("root")), Bytes::lit(&[]))), List::<(Bytes, (Bytes, Bytes))>::nil()), List::<Adt20>::nil(), List::<Bytes>::nil())
}

pub fn f82(v0: Adt1) -> List<(Bytes, Option<Bytes>)> {
    f143(match v0 {
        Adt1::C0(h534, _, _, _) => {
            h534
        }
    })
}

pub fn f83(v0: Bytes) -> R<Result<List<Adt2>, Bytes>> {
    let v1: Bytes = if f121(v0.clone(), Bytes::lit(&[0xef, 0xbb, 0xbf]), 0u64) {
        f45(v0.clone(), 3u64, {
            let a536 = {
                let a535 = v0;
                length_bytes(a535)
            };
            let a537 = 3u64;
            nat_sub(a536, a537)
        })
    } else {
        v0
    };
    let v2: Bytes = f122(v1)?;
    let v3: u64 = {
        let a538 = v2.clone();
        length_bytes(a538)
    };
    let v4: u64 = f144(v2.clone())?;
    let v5: u64 = f145(v2.clone(), 0u64)?;
    let a539 = v4;
    let a540 = v3;
    if nat_lt(a539, a540) {
        Ok(Err::<List<Adt2>, Bytes>(f125(Str::lit("the input is not a sequence of printable characters in UTF-8 (YAML 1.2.2 [1], section 5.2)"), v4)?))
    } else {
        let m543 = f146(v2.clone(), v5, {
            let a541 = v3;
            let a542 = 1u64;
            nat_add(a541, a542)?
        })?;
        match m543 {
            Ok(v7) => {
                let a544 = v3;
                let a545 = v7;
                if nat_le(a544, a545) {
                    Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
                } else {
                    f147(v2, v7, true, {
                        let a546 = v3;
                        let a547 = 1u64;
                        nat_add(a546, a547)?
                    })
                }
            }
            Err(v6) => {
                Ok(Err::<List<Adt2>, Bytes>(v6))
            }
        }
    }
}

pub fn f84(v0: List<Adt2>) -> R<Bytes> {
    let m548 = v0;
    match m548.uncons() {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some((v1, v2)) => {
            Ok(f7(List::cons(f148(v1)?, List::cons(f8(Str::lit("\u{a}")), List::cons(f84(v2)?, List::<Bytes>::nil())))))
        }
    }
}

pub fn f85() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{9}"), List::cons(Str::lit("\u{a}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{d}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f86() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f87() -> List<Str> {
    List::cons(Str::lit(" "), List::cons(Str::lit("!"), List::cons(Str::lit("\""), List::cons(Str::lit("#"), List::cons(Str::lit("$"), List::cons(Str::lit("%"), List::cons(Str::lit("&"), List::cons(Str::lit("'"), List::cons(Str::lit("("), List::cons(Str::lit(")"), List::cons(Str::lit("*"), List::cons(Str::lit("+"), List::cons(Str::lit(","), List::cons(Str::lit("-"), List::cons(Str::lit("."), List::cons(Str::lit("/"), List::<Str>::nil()))))))))))))))))
}

pub fn f88() -> List<Str> {
    List::cons(Str::lit("0"), List::cons(Str::lit("1"), List::cons(Str::lit("2"), List::cons(Str::lit("3"), List::cons(Str::lit("4"), List::cons(Str::lit("5"), List::cons(Str::lit("6"), List::cons(Str::lit("7"), List::cons(Str::lit("8"), List::cons(Str::lit("9"), List::cons(Str::lit(":"), List::cons(Str::lit(";"), List::cons(Str::lit("<"), List::cons(Str::lit("="), List::cons(Str::lit(">"), List::cons(Str::lit("?"), List::<Str>::nil()))))))))))))))))
}

pub fn f89() -> List<Str> {
    List::cons(Str::lit("@"), List::cons(Str::lit("A"), List::cons(Str::lit("B"), List::cons(Str::lit("C"), List::cons(Str::lit("D"), List::cons(Str::lit("E"), List::cons(Str::lit("F"), List::cons(Str::lit("G"), List::cons(Str::lit("H"), List::cons(Str::lit("I"), List::cons(Str::lit("J"), List::cons(Str::lit("K"), List::cons(Str::lit("L"), List::cons(Str::lit("M"), List::cons(Str::lit("N"), List::cons(Str::lit("O"), List::<Str>::nil()))))))))))))))))
}

pub fn f90() -> List<Str> {
    List::cons(Str::lit("P"), List::cons(Str::lit("Q"), List::cons(Str::lit("R"), List::cons(Str::lit("S"), List::cons(Str::lit("T"), List::cons(Str::lit("U"), List::cons(Str::lit("V"), List::cons(Str::lit("W"), List::cons(Str::lit("X"), List::cons(Str::lit("Y"), List::cons(Str::lit("Z"), List::cons(Str::lit("["), List::cons(Str::lit("\\"), List::cons(Str::lit("]"), List::cons(Str::lit("^"), List::cons(Str::lit("_"), List::<Str>::nil()))))))))))))))))
}

pub fn f91() -> List<Str> {
    List::cons(Str::lit("`"), List::cons(Str::lit("a"), List::cons(Str::lit("b"), List::cons(Str::lit("c"), List::cons(Str::lit("d"), List::cons(Str::lit("e"), List::cons(Str::lit("f"), List::cons(Str::lit("g"), List::cons(Str::lit("h"), List::cons(Str::lit("i"), List::cons(Str::lit("j"), List::cons(Str::lit("k"), List::cons(Str::lit("l"), List::cons(Str::lit("m"), List::cons(Str::lit("n"), List::cons(Str::lit("o"), List::<Str>::nil()))))))))))))))))
}

pub fn f92() -> List<Str> {
    List::cons(Str::lit("p"), List::cons(Str::lit("q"), List::cons(Str::lit("r"), List::cons(Str::lit("s"), List::cons(Str::lit("t"), List::cons(Str::lit("u"), List::cons(Str::lit("v"), List::cons(Str::lit("w"), List::cons(Str::lit("x"), List::cons(Str::lit("y"), List::cons(Str::lit("z"), List::cons(Str::lit("{"), List::cons(Str::lit("|"), List::cons(Str::lit("}"), List::cons(Str::lit("~"), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f93(v0: Bytes, p576: u64, p577: u64, p578: bool, p579: bool, p580: u64, p581: Bytes) -> R<Bytes> {
    let m575 = (p576, (p577, (p578, (p579, (p580, p581)))));
    {
        let mut m575 = m575;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, (v5, v6))))) = m575;
                let m549 = v2;
                if m549 == 0 {
                    Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f149(v0.clone(), v5, v1, v6)))
                } else {
                    let v7 = m549 - 1;
                    let m552 = {
                        let a550 = v0.clone();
                        let a551 = v1;
                        index_bytes(a550, a551)
                    };
                    match m552 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f149(v0.clone(), v5, v1, v6)))
                        }
                        Some(v8) => {
                            let v9: u64 = f56(v8)?;
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                    let a553 = v1;
                                    let a554 = 1u64;
                                    nat_add(a553, a554)?
                                }, (v7, (if v4 {
                                    true
                                } else {
                                    let a555 = v9;
                                    let a556 = 34u64;
                                    !nat_eq(a555, a556)
                                }, (if v4 {
                                    false
                                } else {
                                    let a557 = v9;
                                    let a558 = 92u64;
                                    nat_eq(a557, a558)
                                }, (v5, v6))))))
                            } else {
                                let a559 = v9;
                                let a560 = 32u64;
                                if if if if nat_eq(a559, a560) {
                                    true
                                } else {
                                    let a561 = v9;
                                    let a562 = 9u64;
                                    nat_eq(a561, a562)
                                } {
                                    true
                                } else {
                                    let a563 = v9;
                                    let a564 = 10u64;
                                    nat_eq(a563, a564)
                                } {
                                    true
                                } else {
                                    let a565 = v9;
                                    let a566 = 13u64;
                                    nat_eq(a565, a566)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a567 = v1;
                                        let a568 = 1u64;
                                        nat_add(a567, a568)?
                                    }, (v7, (false, (false, ({
                                        let a569 = v1;
                                        let a570 = 1u64;
                                        nat_add(a569, a570)?
                                    }, f149(v0.clone(), v5, v1, v6)))))))
                                } else {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a571 = v1;
                                        let a572 = 1u64;
                                        nat_add(a571, a572)?
                                    }, (v7, ({
                                        let a573 = v9;
                                        let a574 = 34u64;
                                        nat_eq(a573, a574)
                                    }, (false, (v5, v6))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m575 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f94(v0: Bytes, v1: u64) -> R<u64> {
    let m584 = {
        let a582 = v0;
        let a583 = v1;
        index_bytes(a582, a583)
    };
    match m584 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a629 = {
                let a605 = {
                    let a593 = {
                        let a587 = {
                            let a585 = v3;
                            let a586 = 128u8;
                            fixed_u8::bit_and(a585, a586)
                        };
                        let a588 = 128u8;
                        if equal(a587, a588) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a594 = {
                        let a591 = {
                            let a589 = v3;
                            let a590 = 64u8;
                            fixed_u8::bit_and(a589, a590)
                        };
                        let a592 = 64u8;
                        if equal(a591, a592) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a593, a594)?
                };
                let a606 = {
                    let a603 = {
                        let a597 = {
                            let a595 = v3;
                            let a596 = 32u8;
                            fixed_u8::bit_and(a595, a596)
                        };
                        let a598 = 32u8;
                        if equal(a597, a598) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a604 = {
                        let a601 = {
                            let a599 = v3;
                            let a600 = 16u8;
                            fixed_u8::bit_and(a599, a600)
                        };
                        let a602 = 16u8;
                        if equal(a601, a602) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a603, a604)?
                };
                nat_add(a605, a606)?
            };
            let a630 = {
                let a627 = {
                    let a615 = {
                        let a609 = {
                            let a607 = v3;
                            let a608 = 8u8;
                            fixed_u8::bit_and(a607, a608)
                        };
                        let a610 = 8u8;
                        if equal(a609, a610) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a616 = {
                        let a613 = {
                            let a611 = v3;
                            let a612 = 4u8;
                            fixed_u8::bit_and(a611, a612)
                        };
                        let a614 = 4u8;
                        if equal(a613, a614) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a615, a616)?
                };
                let a628 = {
                    let a625 = {
                        let a619 = {
                            let a617 = v3;
                            let a618 = 2u8;
                            fixed_u8::bit_and(a617, a618)
                        };
                        let a620 = 2u8;
                        if equal(a619, a620) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a626 = {
                        let a623 = {
                            let a621 = v3;
                            let a622 = 1u8;
                            fixed_u8::bit_and(a621, a622)
                        };
                        let a624 = 1u8;
                        if equal(a623, a624) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a625, a626)?
                };
                nat_add(a627, a628)?
            };
            nat_add(a629, a630)
        }
    }
}

pub fn f95(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = f150(v0.clone(), v1, {
        let a632 = {
            let a631 = v0.clone();
            length_bytes(a631)
        };
        let a633 = 1u64;
        nat_add(a632, a633)?
    })?;
    let a634 = f94(v0.clone(), v2)?;
    let a635 = 34u64;
    if nat_eq(a634, a635) {
        Ok(Some(({
            let a636 = v2;
            let a637 = 1u64;
            nat_add(a636, a637)?
        }, f45(v0, v1, {
            let a638 = v2;
            let a639 = v1;
            nat_sub(a638, a639)
        }))))
    } else {
        f151(v0.clone(), v1, {
            let a641 = {
                let a640 = v0;
                length_bytes(a640)
            };
            let a642 = 1u64;
            nat_add(a641, a642)?
        }, Bytes::lit(&[]))
    }
}

pub fn f96(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f97(v0: Bytes, v1: u64) -> R<Option<(u64, Adt2)>> {
    let v2: bool = {
        let a643 = f94(v0.clone(), v1)?;
        let a644 = 45u64;
        nat_eq(a643, a644)
    };
    let v3: u64 = if v2 {
        let a645 = v1;
        let a646 = 1u64;
        nat_add(a645, a646)?
    } else {
        v1
    };
    let v4: u64 = f152(v0.clone(), v3)?;
    let v5: u64 = {
        let a647 = v4;
        let a648 = v3;
        nat_sub(a647, a648)
    };
    let v6: bool = {
        let a649 = f94(v0.clone(), v4)?;
        let a650 = 46u64;
        nat_eq(a649, a650)
    };
    let v7: u64 = if v6 {
        let a651 = v4;
        let a652 = 1u64;
        nat_add(a651, a652)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f152(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f94(v0.clone(), v8)?;
    let v10: bool = {
        let a653 = v9;
        let a654 = 101u64;
        if nat_eq(a653, a654) {
            true
        } else {
            let a655 = v9;
            let a656 = 69u64;
            nat_eq(a655, a656)
        }
    };
    let v11: u64 = f94(v0.clone(), {
        let a657 = v8;
        let a658 = 1u64;
        nat_add(a657, a658)?
    })?;
    let v12: bool = f96(v10, f153({
        let a659 = v11;
        let a660 = 43u64;
        nat_eq(a659, a660)
    }, {
        let a661 = v11;
        let a662 = 45u64;
        nat_eq(a661, a662)
    }));
    let v13: u64 = if v10 {
        if v12 {
            let a663 = v8;
            let a664 = 2u64;
            nat_add(a663, a664)?
        } else {
            let a665 = v8;
            let a666 = 1u64;
            nat_add(a665, a666)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f152(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f45(v0.clone(), v13, {
        let a667 = v14;
        let a668 = v13;
        nat_sub(a667, a668)
    });
    let v16: u64 = f154(v15.clone())?;
    let v17: Bytes = f45(v15.clone(), v16, {
        let a670 = {
            let a669 = v15;
            length_bytes(a669)
        };
        let a671 = v16;
        nat_sub(a670, a671)
    });
    let a672 = v5;
    let a673 = 0u64;
    if if nat_eq(a672, a673) {
        true
    } else {
        if f96({
            let a674 = 1u64;
            let a675 = v5;
            nat_lt(a674, a675)
        }, {
            let a676 = f94(v0.clone(), v3)?;
            let a677 = 48u64;
            nat_eq(a676, a677)
        }) {
            true
        } else {
            if f96(v6, {
                let a678 = v8;
                let a679 = v7;
                nat_eq(a678, a679)
            }) {
                true
            } else {
                f153(if v10 {
                    let a680 = v14;
                    let a681 = v13;
                    nat_eq(a680, a681)
                } else {
                    false
                }, {
                    let a683 = 18u64;
                    let a684 = {
                        let a682 = v17.clone();
                        length_bytes(a682)
                    };
                    nat_lt(a683, a684)
                })
            }
        }
    } {
        Ok(None::<(u64, Adt2)>)
    } else {
        let m688 = {
            let a686 = {
                let a685 = v17.clone();
                length_bytes(a685)
            };
            let a687 = 0u64;
            if nat_eq(a686, a687) {
                Some(0u64)
            } else {
                f155(v17)?
            }
        };
        match m688 {
            None => {
                Ok(None::<(u64, Adt2)>)
            }
            Some(v18) => {
                let m693 = f156(f45(v0.clone(), v3, v5), f45(v0, v7, {
                    let a689 = v8;
                    let a690 = v7;
                    nat_sub(a689, a690)
                }), f96(v12, {
                    let a691 = v11;
                    let a692 = 45u64;
                    nat_eq(a691, a692)
                }), v18)?;
                match m693 {
                    None => {
                        Ok(None::<(u64, Adt2)>)
                    }
                    Some(v19) => {
                        let a696 = 19u64;
                        let a697 = {
                            let a695 = {
                                let (h694, _) = v19.clone();
                                h694
                            };
                            length_bytes(a695)
                        };
                        if nat_lt(a696, a697) {
                            Ok(None::<(u64, Adt2)>)
                        } else {
                            let m699 = f155({
                                let (h698, _) = v19.clone();
                                h698
                            })?;
                            match m699 {
                                None => {
                                    Ok(None::<(u64, Adt2)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f96(v2, {
                                        let a700 = 0u64;
                                        let a701 = v20;
                                        nat_lt(a700, a701)
                                    });
                                    if f157(v21, v20) {
                                        Ok(Some((v14, {
                                            let a703 = {
                                                let (_, h702) = v19.clone();
                                                h702
                                            };
                                            let a704 = 0u64;
                                            if nat_eq(a703, a704) {
                                                Adt2::C3(v21, v20)
                                            } else {
                                                Adt2::C4(v21, v20, {
                                                    let (_, h705) = v19;
                                                    h705
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

pub fn f98(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h706, _) = v2.clone();
        h706
    };
    let v4: u64 = {
        let (h707, _) = {
            let (_, h708) = v2.clone();
            h708
        };
        h707
    };
    let v5: List<Bytes> = {
        let (_, h709) = {
            let (_, h710) = v2;
            h710
        };
        h709
    };
    let m711 = f158(v0.clone(), v3);
    match m711 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a712 = v6;
            let a713 = v1;
            if equal(a712, a713) {
                Ok(Some(({
                    let a714 = v3;
                    let a715 = 1u64;
                    nat_add(a714, a715)?
                }, ({
                    let a716 = v3;
                    let a717 = 1u64;
                    nat_add(a716, a717)?
                }, List::cons(f45(v0, v4, {
                    let a718 = v3;
                    let a719 = v4;
                    nat_sub(a718, a719)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a720 = v3;
                    let a721 = 1u64;
                    nat_add(a720, a721)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f99(v0: Fn2, p725: List<Bytes>, p726: List<Bytes>) -> List<Bytes> {
    let m724 = (p725, p726);
    {
        let mut m724 = m724;
        loop {
            let step = {
                let (v1, v2) = m724;
                let m722 = v2;
                match m722.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c723: Fn2 = v0.clone();
                            c723.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m724 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f100(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f101(v0: Adt13) -> List<Adt13> {
    let m727 = v0;
    match m727 {
        Adt13::C0(_) => {
            List::<Adt13>::nil()
        }
        Adt13::C1(_, _) => {
            List::<Adt13>::nil()
        }
        Adt13::C2(_) => {
            List::<Adt13>::nil()
        }
        Adt13::C3(r728) => {
            let v3 = (*r728).clone();
            f101(v3)
        }
        Adt13::C4(_) => {
            List::<Adt13>::nil()
        }
        Adt13::C5 => {
            List::<Adt13>::nil()
        }
        Adt13::C6(r729, r730) => {
            let v1 = (*r729).clone();
            let v2 = (*r730).clone();
            List::cons(v1, f101(v2))
        }
        Adt13::C7(_, _, _) => {
            List::<Adt13>::nil()
        }
    }
}

pub fn f102(v0: Adt21) -> Bytes {
    let a751 = f159(match v0.clone() {
        Adt21::C0(h731, _, _, _, _, _, _, _) => {
            h731
        }
    });
    let a752 = {
        let a749 = f159(match v0.clone() {
            Adt21::C0(_, h732, _, _, _, _, _, _) => {
                h732
            }
        });
        let a750 = {
            let a747 = f159(match v0.clone() {
                Adt21::C0(_, _, h733, _, _, _, _, _) => {
                    h733
                }
            });
            let a748 = {
                let a745 = f159(match v0.clone() {
                    Adt21::C0(_, _, _, h734, _, _, _, _) => {
                        h734
                    }
                });
                let a746 = {
                    let a743 = f159(match v0.clone() {
                        Adt21::C0(_, _, _, _, h735, _, _, _) => {
                            h735
                        }
                    });
                    let a744 = {
                        let a741 = f159(match v0.clone() {
                            Adt21::C0(_, _, _, _, _, h736, _, _) => {
                                h736
                            }
                        });
                        let a742 = {
                            let a739 = f159(match v0.clone() {
                                Adt21::C0(_, _, _, _, _, _, h737, _) => {
                                    h737
                                }
                            });
                            let a740 = f159(match v0 {
                                Adt21::C0(_, _, _, _, _, _, _, h738) => {
                                    h738
                                }
                            });
                            append_bytes(a739, a740)
                        };
                        append_bytes(a741, a742)
                    };
                    append_bytes(a743, a744)
                };
                append_bytes(a745, a746)
            };
            append_bytes(a747, a748)
        };
        append_bytes(a749, a750)
    };
    append_bytes(a751, a752)
}

pub fn f103(v0: Bytes) -> R<Adt21> {
    let v1: u64 = {
        let a753 = v0.clone();
        length_bytes(a753)
    };
    let v2: u64 = {
        let a759 = {
            let a756 = {
                let a754 = v1;
                let a755 = 72u64;
                nat_add(a754, a755)?
            };
            let a757 = 64u64;
            let a758 = 0u64;
            nat_quot(a756, a757, a758)
        };
        let a760 = 64u64;
        nat_mul(a759, a760)?
    };
    let (_, h761) = f160(Fn3::F161(v0, v1, v2), {
        let a762 = v2;
        let a763 = 64u64;
        let a764 = 0u64;
        nat_quot(a762, a763, a764)
    }, (0u64, f162()))?;
    Ok(h761)
}

pub fn f104(v0: Adt1, v1: Bytes) -> R<Option<Adt2>> {
    let m765 = f163(f164(v0.clone(), v1.clone()), v1.clone());
    match m765 {
        None => {
            f42(f74(v0, v1))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f105(v0: Adt2) -> Option<Adt13> {
    let m766 = v0;
    match m766 {
        Adt2::C0 => {
            Some(Adt13::C3(std::rc::Rc::new(Adt13::C5)))
        }
        Adt2::C1(_, _) => {
            None::<Adt13>
        }
        Adt2::C2(v3) => {
            Some(Adt13::C0(v3))
        }
        Adt2::C3(v4, v5) => {
            Some(Adt13::C1(v4, v5))
        }
        Adt2::C4(_, _, _) => {
            None::<Adt13>
        }
        Adt2::C5(v9) => {
            Some(Adt13::C2(v9))
        }
        Adt2::C6(r773) => {
            let v10 = (*r773).clone();
            let m767 = f105(v10);
            match m767 {
                None => {
                    None::<Adt13>
                }
                Some(v11) => {
                    Some(Adt13::C3(std::rc::Rc::new(v11)))
                }
            }
        }
        Adt2::C7(r774) => {
            let v12 = (*r774).clone();
            let m768 = f105(v12);
            match m768 {
                None => {
                    None::<Adt13>
                }
                Some(v13) => {
                    Some(Adt13::C4(std::rc::Rc::new(v13)))
                }
            }
        }
        Adt2::C8 => {
            Some(Adt13::C5)
        }
        Adt2::C9(r775, r776) => {
            let v14 = (*r775).clone();
            let v15 = (*r776).clone();
            let m769 = f105(v14);
            match m769 {
                None => {
                    None::<Adt13>
                }
                Some(v16) => {
                    let m770 = f105(v15);
                    match m770 {
                        None => {
                            None::<Adt13>
                        }
                        Some(v17) => {
                            Some(Adt13::C6(std::rc::Rc::new(v16), std::rc::Rc::new(v17)))
                        }
                    }
                }
            }
        }
        Adt2::C10(v18, r777, r778) => {
            let v19 = (*r777).clone();
            let v20 = (*r778).clone();
            let m771 = f105(v19);
            match m771 {
                None => {
                    None::<Adt13>
                }
                Some(v21) => {
                    let m772 = f105(v20);
                    match m772 {
                        None => {
                            None::<Adt13>
                        }
                        Some(v22) => {
                            Some(Adt13::C7(v18, std::rc::Rc::new(v21), std::rc::Rc::new(v22)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f106(v0: Bytes) -> R<Result<List<Adt14>, Bytes>> {
    let m779 = f165(v0)?;
    match m779 {
        Ok(v2) => {
            f166({
                let a781 = {
                    let a780 = v2.clone();
                    length_list(a780)
                };
                let a782 = 1u64;
                nat_add(a781, a782)?
            }, v2, List::<Adt14>::nil())
        }
        Err(v1) => {
            Ok(Err::<List<Adt14>, Bytes>(v1))
        }
    }
}

pub fn f107(v0: List<Bytes>) -> R<Result<List<Adt14>, Bytes>> {
    let m783 = v0;
    match m783.uncons() {
        None => {
            Ok(Ok::<List<Adt14>, Bytes>(List::<Adt14>::nil()))
        }
        Some((v1, v2)) => {
            let m784 = f106(v1)?;
            match m784 {
                Ok(v4) => {
                    let m785 = f107(v2)?;
                    match m785 {
                        Ok(v6) => {
                            Ok(Ok::<List<Adt14>, Bytes>(f108(v4, v6)))
                        }
                        Err(v5) => {
                            Ok(Err::<List<Adt14>, Bytes>(v5))
                        }
                    }
                }
                Err(v3) => {
                    Ok(Err::<List<Adt14>, Bytes>(v3))
                }
            }
        }
    }
}

pub fn f108(v0: List<Adt14>, v1: List<Adt14>) -> List<Adt14> {
    let m786 = v0;
    match m786.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f108(v3, v1))
        }
    }
}

pub fn f109(p791: u64, p792: List<Adt17>) -> R<List<Adt17>> {
    let m790 = (p791, p792);
    {
        let mut m790 = m790;
        loop {
            let step = {
                let (v0, v1) = m790;
                let m787 = v0;
                if m787 == 0 {
                    Err::<(u64, List<Adt17>), R<List<Adt17>>>(Ok(v1))
                } else {
                    let v2 = m787 - 1;
                    let v3: List<Adt17> = f167(v1.clone(), v1.clone());
                    let a788 = f168(v3.clone())?;
                    let a789 = f168(v1.clone())?;
                    if nat_eq(a788, a789) {
                        Err::<(u64, List<Adt17>), R<List<Adt17>>>(Ok(v1))
                    } else {
                        Ok::<(u64, List<Adt17>), R<List<Adt17>>>((v2, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m790 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f110(v0: List<Adt14>, v1: List<Adt14>) -> R<List<Adt17>> {
    let m793 = v0;
    match m793.uncons() {
        None => {
            Ok(List::<Adt17>::nil())
        }
        Some((v2, v3)) => {
            if f169(f170(v2.clone())) {
                let v6: Adt17 = {
                    let m794 = f171(v1.clone(), v2.clone())?;
                    match m794 {
                        Ok(v5) => {
                            Adt17::C0(f172(v2), Bytes::lit(&[]), {
                                let (h795, _) = v5;
                                h795
                            }, {
                                let (_, h796) = v5;
                                h796
                            })
                        }
                        Err(v4) => {
                            Adt17::C0(f172(v2), v4, Bytes::lit(&[]), List::<Bytes>::nil())
                        }
                    }
                };
                Ok(List::cons(v6, f110(v3, v1)?))
            } else {
                f110(v3, v1)
            }
        }
    }
}

pub fn f111(p801: u64, p802: List<Bytes>, p803: List<Bytes>, v3: List<Adt17>) -> List<Bytes> {
    let m800 = (p801, (p802, p803));
    {
        let mut m800 = m800;
        loop {
            let step = {
                let (v0, (v1, v2)) = m800;
                let m797 = v0;
                if m797 == 0 {
                    Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v2)
                } else {
                    let v4 = m797 - 1;
                    let m798 = v1;
                    match m798.uncons() {
                        None => {
                            Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v2)
                        }
                        Some((v5, v6)) => {
                            if f173(v5.clone(), v2.clone()) {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v2)))
                            } else {
                                let m799 = f174(v3.clone(), v5.clone());
                                match m799 {
                                    None => {
                                        Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v2)))
                                    }
                                    Some(v7) => {
                                        if f32(f175(v7.clone()), Bytes::lit(&[])) {
                                            Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (f176(f177(v7), v6), List::cons(v5, v2))))
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
                Ok(next) => { m800 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f112(v0: List<Adt14>) -> List<Bytes> {
    let m804 = v0;
    match m804.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let v3: u64 = f170(v1.clone());
            let a805 = v3;
            let a806 = 0u64;
            if if nat_eq(a805, a806) {
                true
            } else {
                let a807 = v3;
                let a808 = 2u64;
                if nat_eq(a807, a808) {
                    true
                } else {
                    let a809 = v3;
                    let a810 = 3u64;
                    if nat_eq(a809, a810) {
                        true
                    } else {
                        let a811 = v3;
                        let a812 = 4u64;
                        if nat_eq(a811, a812) {
                            true
                        } else {
                            let a813 = v3;
                            let a814 = 5u64;
                            if nat_eq(a813, a814) {
                                true
                            } else {
                                let a815 = v3;
                                let a816 = 7u64;
                                if nat_eq(a815, a816) {
                                    true
                                } else {
                                    let a817 = v3;
                                    let a818 = 8u64;
                                    nat_eq(a817, a818)
                                }
                            }
                        }
                    }
                }
            } {
                List::cons(f172(v1), f112(v2))
            } else {
                f112(v2)
            }
        }
    }
}

pub fn f113(v0: List<Adt17>, v1: List<Bytes>) -> List<(Bytes, (List<Bytes>, Bytes))> {
    let m819 = v0;
    match m819.uncons() {
        None => {
            List::<(Bytes, (List<Bytes>, Bytes))>::nil()
        }
        Some((v2, v3)) => {
            if if f173(f178(v2.clone()), v1.clone()) {
                f32(f175(v2.clone()), Bytes::lit(&[]))
            } else {
                false
            } {
                List::cons((f178(v2.clone()), (f177(v2.clone()), f179(v2))), f113(v3, v1))
            } else {
                f113(v3, v1)
            }
        }
    }
}

pub fn f114(p828: u64, p829: List<(Bytes, (List<Bytes>, Bytes))>, p830: List<Bytes>, p831: List<Bytes>) -> (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>) {
    let m827 = (p828, (p829, (p830, p831)));
    {
        let mut m827 = m827;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m827;
                let m820 = v0;
                if m820 == 0 {
                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v3, v1))
                } else {
                    let v4 = m820 - 1;
                    let m821 = f180(v1.clone(), v2.clone());
                    match m821 {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v3, v1))
                        }
                        Some(v5) => {
                            Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v4, ({
                                let (_, h822) = v5.clone();
                                h822
                            }, (List::cons({
                                let (h823, _) = {
                                    let (h824, _) = v5.clone();
                                    h824
                                };
                                h823
                            }, v2), List::cons({
                                let (_, h825) = {
                                    let (h826, _) = v5;
                                    h826
                                };
                                h825
                            }, v3)))))
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

pub fn f115(v0: List<Adt14>, v1: List<Adt17>, v2: List<Adt14>) -> List<Bytes> {
    let m832 = v0;
    match m832.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            let v5: u64 = f170(v3.clone());
            let a833 = v5;
            let a834 = 5u64;
            if if nat_eq(a833, a834) {
                true
            } else {
                let a835 = v5;
                let a836 = 7u64;
                nat_eq(a835, a836)
            } {
                let m839 = f181(v2.clone(), {
                    let a837 = v5;
                    let a838 = 5u64;
                    if nat_eq(a837, a838) {
                        0u64
                    } else {
                        2u64
                    }
                }, f172(v3.clone()));
                match m839 {
                    None => {
                        List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f182(v5), List::cons(f172(v3.clone()), List::cons(f8(Str::lit(": no input defines ")), List::cons(f172(v3), List::cons(f8(Str::lit(", so its members join no definition (Web IDL 2.2 and 2.7)\u{a}")), List::<Bytes>::nil()))))))), f115(v4, v1, v2))
                    }
                    Some(_) => {
                        f115(v4, v1, v2)
                    }
                }
            } else {
                let a840 = v5;
                let a841 = 8u64;
                if nat_eq(a840, a841) {
                    let m842 = f181(v2.clone(), 0u64, f172(v3.clone()));
                    match m842 {
                        None => {
                            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f172(v3.clone()), List::cons(f8(Str::lit(" includes ")), List::cons(f183(v3.clone()), List::cons(f8(Str::lit(": no input defines the interface ")), List::cons(f172(v3), List::cons(f8(Str::lit(" (Web IDL 2.3)\u{a}")), List::<Bytes>::nil())))))))), f115(v4, v1, v2))
                        }
                        Some(_) => {
                            f115(v4, v1, v2)
                        }
                    }
                } else {
                    if f169(v5) {
                        let m843 = f174(v1.clone(), f172(v3.clone()));
                        match m843 {
                            None => {
                                f115(v4, v1, v2)
                            }
                            Some(v8) => {
                                if f32(f175(v8.clone()), Bytes::lit(&[])) {
                                    f115(v4, v1, v2)
                                } else {
                                    List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f172(v3), List::cons(f8(Str::lit(": ")), List::cons(f175(v8), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), f115(v4, v1, v2))
                                }
                            }
                        }
                    } else {
                        f115(v4, v1, v2)
                    }
                }
            }
        }
    }
}

pub fn f116(v0: List<(Bytes, (List<Bytes>, Bytes))>) -> List<Bytes> {
    let m844 = v0;
    match m844.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons({
                let (h845, _) = v1;
                h845
            }, List::cons(f8(Str::lit(": it refers back to itself through its members, and the translator emits non-recursive declarations only\u{a}")), List::<Bytes>::nil())))), f116(v2))
        }
    }
}

pub fn f117(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m846 = v0;
    match m846.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m847 = v3.clone();
            match m847.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a850 = v2;
                    let a851 = {
                        let a848 = v1.clone();
                        let a849 = f117(v3, v1);
                        append_bytes(a848, a849)
                    };
                    append_bytes(a850, a851)
                }
            }
        }
    }
}

pub fn f118(p854: List<Bytes>, p855: List<Bytes>) -> List<Bytes> {
    let m853 = (p854, p855);
    {
        let mut m853 = m853;
        loop {
            let step = {
                let (v0, v1) = m853;
                let m852 = v0;
                match m852.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m853 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f119(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a856 = v2;
    let a857 = 0u64;
    if nat_eq(a856, a857) {
        Ok(f184(v0, 0u64))
    } else {
        let (_, h858) = {
            let (h859, _) = f185(Fn4::F186(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h859
        };
        Ok(h858)
    }
}

pub fn f120() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f121(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m864 = {
        let a861 = v0;
        let a862 = v2;
        let a863 = {
            let a860 = v1.clone();
            length_bytes(a860)
        };
        slice_bytes(a861, a862, a863)
    };
    match m864 {
        None => {
            false
        }
        Some(v3) => {
            let a865 = v3;
            let a866 = v1;
            equal(a865, a866)
        }
    }
}

pub fn f122(v0: Bytes) -> R<Bytes> {
    let (_, h867) = {
        let (h868, _) = f185(Fn4::F187(v0.clone()), {
            let a870 = {
                let a869 = v0;
                length_bytes(a869)
            };
            let a871 = 1u64;
            nat_add(a870, a871)?
        }, (0u64, Bytes::lit(&[])))?;
        h868
    };
    Ok(h867)
}

pub fn f123(v0: Bytes) -> R<u64> {
    let (h872, _) = f188(Fn5::F189(v0.clone()), {
        let a874 = {
            let a873 = v0;
            length_bytes(a873)
        };
        let a875 = 1u64;
        nat_add(a874, a875)?
    }, 0u64)?;
    Ok(h872)
}

pub fn f124(v0: Bytes) -> Result<Adt18, Bytes> {
    Err::<Adt18, Bytes>(v0)
}

pub fn f125(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a876 = v0;
        utf8_encode(a876)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f76(v1)?, List::<Bytes>::nil())))))
}

pub fn f126(v0: Bytes) -> R<Result<u64, Bytes>> {
    if if f128(v0.clone(), 0u64, Str::lit("<?xml")) {
        f190(f129(v0.clone(), 5u64)?)?
    } else {
        false
    } {
        let m878 = f191(v0.clone(), {
            let a877 = Str::lit("?>");
            utf8_encode(a877)
        }, 5u64)?;
        match m878 {
            None => {
                Ok(f192(f125(Str::lit("the XML declaration is not closed (XML 1.0 [23])"), 0u64)?))
            }
            Some(v1) => {
                let m880 = f191(f45(v0.clone(), 0u64, v1), {
                    let a879 = Str::lit("encoding");
                    utf8_encode(a879)
                }, 5u64)?;
                match m880 {
                    None => {
                        Ok(Ok::<u64, Bytes>({
                            let a881 = v1;
                            let a882 = 2u64;
                            nat_add(a881, a882)?
                        }))
                    }
                    Some(v2) => {
                        let v3: u64 = f193(v0.clone(), {
                            let a883 = v2;
                            let a884 = 8u64;
                            nat_add(a883, a884)?
                        })?;
                        let v4: u64 = f193(v0.clone(), {
                            let a885 = v3;
                            let a886 = 1u64;
                            nat_add(a885, a886)?
                        })?;
                        let _: u64 = f129(v0.clone(), v4)?;
                        let v7: u64 = {
                            let m889 = f191(v0.clone(), f45(v0.clone(), v4, 1u64), {
                                let a887 = v4;
                                let a888 = 1u64;
                                nat_add(a887, a888)?
                            })?;
                            match m889 {
                                None => {
                                    v1
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        if f194(f45(v0, {
                            let a890 = v4;
                            let a891 = 1u64;
                            nat_add(a890, a891)?
                        }, {
                            let a894 = v7;
                            let a895 = {
                                let a892 = v4;
                                let a893 = 1u64;
                                nat_add(a892, a893)?
                            };
                            nat_sub(a894, a895)
                        })) {
                            Ok(Ok::<u64, Bytes>({
                                let a896 = v1;
                                let a897 = 2u64;
                                nat_add(a896, a897)?
                            }))
                        } else {
                            Ok(f192(f125(Str::lit("only the UTF-8 encoding is read (XML 1.0 section 4.3.3)"), 0u64)?))
                        }
                    }
                }
            }
        }
    } else {
        Ok(Ok::<u64, Bytes>(0u64))
    }
}

pub fn f127(v0: Bytes, p902: u64, p903: u64) -> R<Result<u64, Bytes>> {
    let m901 = (p902, p903);
    {
        let mut m901 = m901;
        loop {
            let step = {
                let (v1, v2) = m901;
                let m898 = v2;
                if m898 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f192(f125(Str::lit("the prolog is longer than the input"), v1)?)))
                } else {
                    let v3 = m898 - 1;
                    let v4: u64 = f193(v0.clone(), v1)?;
                    if f128(v0.clone(), v4, Str::lit("<!--")) {
                        let m899 = f195(v0.clone(), v4)?;
                        match m899 {
                            Ok(v6) => {
                                Ok::<(u64, u64), R<Result<u64, Bytes>>>((v6, v3))
                            }
                            Err(v5) => {
                                Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f192(v5)))
                            }
                        }
                    } else {
                        if f128(v0.clone(), v4, Str::lit("<?")) {
                            let m900 = f196(v0.clone(), v4)?;
                            match m900 {
                                Ok(v8) => {
                                    Ok::<(u64, u64), R<Result<u64, Bytes>>>((v8, v3))
                                }
                                Err(v7) => {
                                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f192(v7)))
                                }
                            }
                        } else {
                            Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Ok::<u64, Bytes>(v4)))
                        }
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

pub fn f128(v0: Bytes, v1: u64, v2: Str) -> bool {
    f121(v0, {
        let a904 = v2;
        utf8_encode(a904)
    }, v1)
}

pub fn f129(v0: Bytes, v1: u64) -> R<u64> {
    f94(v0, v1)
}

pub fn f130(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Result<(Adt18, u64), Bytes>> {
    let m905 = v2;
    if m905 == 0 {
        Ok(f197(f125(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v4 = m905 - 1;
        let a906 = v3;
        let a907 = 0u64;
        if nat_eq(a906, a907) {
            let v5: u64 = f198(v0.clone(), {
                let a908 = v1;
                let a909 = 1u64;
                nat_add(a908, a909)?
            })?;
            let a912 = v5;
            let a913 = {
                let a910 = v1;
                let a911 = 1u64;
                nat_add(a910, a911)?
            };
            if nat_eq(a912, a913) {
                Ok(f197(f125(Str::lit("an element must start with a Name (XML 1.0 [40])"), v1)?))
            } else {
                let v6: Bytes = f45(v0.clone(), {
                    let a914 = v1;
                    let a915 = 1u64;
                    nat_add(a914, a915)?
                }, {
                    let a918 = v5;
                    let a919 = {
                        let a916 = v1;
                        let a917 = 1u64;
                        nat_add(a916, a917)?
                    };
                    nat_sub(a918, a919)
                });
                let m920 = f199(v0.clone(), v5, v4, List::<Bytes>::nil())?;
                match m920 {
                    Ok(v8) => {
                        if f128(v0.clone(), {
                            let (_, h921) = v8.clone();
                            h921
                        }, Str::lit("/>")) {
                            Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C0(v6, std::rc::Rc::new({
                                let (h922, _) = v8;
                                h922
                            }), std::rc::Rc::new(Adt18::C4)), {
                                let a924 = {
                                    let (_, h923) = v8;
                                    h923
                                };
                                let a925 = 2u64;
                                nat_add(a924, a925)?
                            })))
                        } else {
                            let m929 = f130(v0.clone(), {
                                let a927 = {
                                    let (_, h926) = v8;
                                    h926
                                };
                                let a928 = 1u64;
                                nat_add(a927, a928)?
                            }, v4, 1u64)?;
                            match m929 {
                                Ok(v10) => {
                                    let v11: u64 = {
                                        let a931 = {
                                            let (_, h930) = v10.clone();
                                            h930
                                        };
                                        let a932 = 2u64;
                                        nat_add(a931, a932)?
                                    };
                                    let v12: u64 = f198(v0.clone(), v11)?;
                                    if f32(f45(v0.clone(), v11, {
                                        let a933 = v12;
                                        let a934 = v11;
                                        nat_sub(a933, a934)
                                    }), v6.clone()) {
                                        let v13: u64 = f193(v0.clone(), v12)?;
                                        let a935 = f129(v0, v13)?;
                                        let a936 = 62u64;
                                        if nat_eq(a935, a936) {
                                            Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C0(v6, std::rc::Rc::new({
                                                let (h937, _) = v8;
                                                h937
                                            }), std::rc::Rc::new({
                                                let (h938, _) = v10;
                                                h938
                                            })), {
                                                let a939 = v13;
                                                let a940 = 1u64;
                                                nat_add(a939, a940)?
                                            })))
                                        } else {
                                            Ok(f197(f125(Str::lit("an end tag must close with > (XML 1.0 [42])"), v13)?))
                                        }
                                    } else {
                                        Ok(f197(f125(Str::lit("an end tag must name the element its start tag opened (XML 1.0 WFC: Element Type Match)"), {
                                            let (_, h941) = v10;
                                            h941
                                        })?))
                                    }
                                }
                                Err(v9) => {
                                    Ok(f197(v9))
                                }
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(f197(v7))
                    }
                }
            }
        } else {
            let a942 = f129(v0.clone(), v1)?;
            let a943 = 256u64;
            if nat_eq(a942, a943) {
                Ok(f197(f125(Str::lit("the input ends before an element is closed (XML 1.0 [39])"), v1)?))
            } else {
                if f128(v0.clone(), v1, Str::lit("</")) {
                    Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C4, v1)))
                } else {
                    if f128(v0.clone(), v1, Str::lit("<!--")) {
                        let m944 = f195(v0.clone(), v1)?;
                        match m944 {
                            Ok(v15) => {
                                f130(v0, v15, v4, 1u64)
                            }
                            Err(v14) => {
                                Ok(f197(v14))
                            }
                        }
                    } else {
                        if f128(v0.clone(), v1, Str::lit("<![CDATA[")) {
                            let m948 = f191(v0.clone(), {
                                let a945 = Str::lit("]]>");
                                utf8_encode(a945)
                            }, {
                                let a946 = v1;
                                let a947 = 9u64;
                                nat_add(a946, a947)?
                            })?;
                            match m948 {
                                None => {
                                    Ok(f197(f125(Str::lit("a CDATA section is not closed (XML 1.0 [18])"), v1)?))
                                }
                                Some(v16) => {
                                    let m951 = f130(v0.clone(), {
                                        let a949 = v16;
                                        let a950 = 3u64;
                                        nat_add(a949, a950)?
                                    }, v4, 1u64)?;
                                    match m951 {
                                        Ok(v18) => {
                                            Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C3(std::rc::Rc::new(Adt18::C2(f45(v0, {
                                                let a952 = v1;
                                                let a953 = 9u64;
                                                nat_add(a952, a953)?
                                            }, {
                                                let a956 = v16;
                                                let a957 = {
                                                    let a954 = v1;
                                                    let a955 = 9u64;
                                                    nat_add(a954, a955)?
                                                };
                                                nat_sub(a956, a957)
                                            }))), std::rc::Rc::new({
                                                let (h958, _) = v18;
                                                h958
                                            })), {
                                                let (_, h959) = v18;
                                                h959
                                            })))
                                        }
                                        Err(v17) => {
                                            Ok(f197(v17))
                                        }
                                    }
                                }
                            }
                        } else {
                            if f128(v0.clone(), v1, Str::lit("<?")) {
                                let m960 = f196(v0.clone(), v1)?;
                                match m960 {
                                    Ok(v20) => {
                                        f130(v0, v20, v4, 1u64)
                                    }
                                    Err(v19) => {
                                        Ok(f197(v19))
                                    }
                                }
                            } else {
                                if f128(v0.clone(), v1, Str::lit("<!")) {
                                    Ok(f197(f125(Str::lit("a markup declaration is not allowed in content (XML 1.0 [43])"), v1)?))
                                } else {
                                    let a961 = f129(v0.clone(), v1)?;
                                    let a962 = 60u64;
                                    if nat_eq(a961, a962) {
                                        let m963 = f130(v0.clone(), v1, v4, 0u64)?;
                                        match m963 {
                                            Ok(v22) => {
                                                let m965 = f130(v0, {
                                                    let (_, h964) = v22;
                                                    h964
                                                }, v4, 1u64)?;
                                                match m965 {
                                                    Ok(v24) => {
                                                        Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C3(std::rc::Rc::new({
                                                            let (h966, _) = v22;
                                                            h966
                                                        }), std::rc::Rc::new({
                                                            let (h967, _) = v24;
                                                            h967
                                                        })), {
                                                            let (_, h968) = v24;
                                                            h968
                                                        })))
                                                    }
                                                    Err(v23) => {
                                                        Ok(f197(v23))
                                                    }
                                                }
                                            }
                                            Err(v21) => {
                                                Ok(f197(v21))
                                            }
                                        }
                                    } else {
                                        let m969 = f200(v0.clone(), v1, v4, Bytes::lit(&[]), false, 0u64)?;
                                        match m969 {
                                            Ok(v26) => {
                                                let m971 = f130(v0, {
                                                    let (_, h970) = v26;
                                                    h970
                                                }, v4, 1u64)?;
                                                match m971 {
                                                    Ok(v28) => {
                                                        Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C3(std::rc::Rc::new(Adt18::C2({
                                                            let (h972, _) = v26;
                                                            h972
                                                        })), std::rc::Rc::new({
                                                            let (h973, _) = v28;
                                                            h973
                                                        })), {
                                                            let (_, h974) = v28;
                                                            h974
                                                        })))
                                                    }
                                                    Err(v27) => {
                                                        Ok(f197(v27))
                                                    }
                                                }
                                            }
                                            Err(v25) => {
                                                Ok(f197(v25))
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

pub fn f131(v0: Bytes) -> Result<(u64, Bytes), Bytes> {
    Err::<(u64, Bytes), Bytes>(v0)
}

pub fn f132(v0: Adt18, v1: List<(Bytes, Bytes)>) -> Result<List<(Bytes, Bytes)>, Bytes> {
    let m975 = v0;
    match m975 {
        Adt18::C0(_, _, _) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt18::C1(v2, v3, r980) => {
            let v4 = (*r980).clone();
            let m976 = f132(v4, v1);
            match m976 {
                Ok(v6) => {
                    if f32(v2.clone(), f8(Str::lit("xmlns"))) {
                        if if f32(v3.clone(), f202()) {
                            true
                        } else {
                            f32(v3.clone(), f203())
                        } {
                            f201(f8(Str::lit("the default namespace must not be the xml or xmlns namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Bytes>(List::cons((Bytes::lit(&[]), v3), v6))
                        }
                    } else {
                        if f204(v2.clone(), f8(Str::lit("xmlns:"))) {
                            let v7: Bytes = f45(v2.clone(), 6u64, {
                                let a978 = {
                                    let a977 = v2;
                                    length_bytes(a977)
                                };
                                let a979 = 6u64;
                                nat_sub(a978, a979)
                            });
                            if f32(v7.clone(), f8(Str::lit("xmlns"))) {
                                f201(f8(Str::lit("the prefix xmlns must not be declared (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                            } else {
                                if if f32(v7.clone(), f8(Str::lit("xml"))) {
                                    !f32(v3.clone(), f202())
                                } else {
                                    false
                                } {
                                    f201(f8(Str::lit("the prefix xml may be bound only to its own namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                } else {
                                    if if if f32(v3.clone(), f202()) {
                                        true
                                    } else {
                                        f32(v3.clone(), f203())
                                    } {
                                        !f32(v7.clone(), f8(Str::lit("xml")))
                                    } else {
                                        false
                                    } {
                                        f201(f8(Str::lit("the xml and xmlns namespace names must not be bound to another prefix (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                    } else {
                                        if f32(v3.clone(), Bytes::lit(&[])) {
                                            f201(f8(Str::lit("a prefix must not be undeclared (Namespaces in XML 1.0, section 3, NSC: No Prefix Undeclaring)")))
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
                    f201(v5)
                }
            }
        }
        Adt18::C2(_) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt18::C3(_, _) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt18::C4 => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
    }
}

pub fn f133(v0: List<(Bytes, Bytes)>, v1: Bytes) -> R<Result<(Bytes, Bytes), Bytes>> {
    let v2: Bytes = f205(v1.clone())?;
    if f32(v2.clone(), Bytes::lit(&[])) {
        let m981 = f206(v0, Bytes::lit(&[]));
        match m981 {
            None => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((Bytes::lit(&[]), v1)))
            }
            Some(v3) => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((v3, v1)))
            }
        }
    } else {
        if f32(v2.clone(), f8(Str::lit("xml"))) {
            Ok(Ok::<(Bytes, Bytes), Bytes>((f202(), f207(v1)?)))
        } else {
            let m982 = f206(v0, v2.clone());
            match m982 {
                None => {
                    Ok(Err::<(Bytes, Bytes), Bytes>(f7(List::cons(f8(Str::lit("the prefix ")), List::cons(v2, List::cons(f8(Str::lit(" is not declared (Namespaces in XML 1.0, section 5, NSC: Prefix Declared)")), List::<Bytes>::nil()))))))
                }
                Some(v4) => {
                    Ok(Ok::<(Bytes, Bytes), Bytes>((v4, f207(v1)?)))
                }
            }
        }
    }
}

pub fn f134(v0: (Bytes, Bytes), v1: Str) -> bool {
    if f32({
        let (h983, _) = v0;
        h983
    }, f208()) {
        f32({
            let (_, h984) = v0;
            h984
        }, {
            let a985 = v1;
            utf8_encode(a985)
        })
    } else {
        false
    }
}

pub fn f135(p989: Adt18, v1: Bytes) -> Option<Bytes> {
    let m988 = p989;
    {
        let mut m988 = m988;
        loop {
            let step = {
                let v0 = m988;
                let m986 = v0;
                match m986 {
                    Adt18::C0(_, _, _) => {
                        Err::<Adt18, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt18::C1(v2, v3, r987) => {
                        let v4 = (*r987).clone();
                        if f32(v2, v1.clone()) {
                            Err::<Adt18, Option<Bytes>>(Some(v3))
                        } else {
                            Ok::<Adt18, Option<Bytes>>(v4)
                        }
                    }
                    Adt18::C2(_) => {
                        Err::<Adt18, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt18::C3(_, _) => {
                        Err::<Adt18, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt18::C4 => {
                        Err::<Adt18, Option<Bytes>>(None::<Bytes>)
                    }
                }
            };
            match step {
                Ok(next) => { m988 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f136(v0: Adt18, v1: List<(Bytes, Bytes)>, v2: Bytes) -> R<Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>> {
    let m990 = v0;
    match m990 {
        Adt18::C0(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt18::C1(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt18::C2(_) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt18::C3(r1001, r1002) => {
            let v3 = (*r1001).clone();
            let v4 = (*r1002).clone();
            let m991 = f136(v4, v1.clone(), v2.clone())?;
            match m991 {
                Ok(v6) => {
                    let m992 = v3;
                    match m992 {
                        Adt18::C0(v7, r999, r1000) => {
                            let v8 = (*r999).clone();
                            let v9 = (*r1000).clone();
                            let m993 = f132(v8.clone(), v1);
                            match m993 {
                                Ok(v11) => {
                                    let m994 = f133(v11.clone(), v7)?;
                                    match m994 {
                                        Ok(v13) => {
                                            if f134(v13, Str::lit("simpleType")) {
                                                let m995 = f135(v8, f8(Str::lit("name")));
                                                match m995 {
                                                    None => {
                                                        Ok(f209(f8(Str::lit("a top-level simpleType must have a name (XSD 1.1 Part 2, section 4.1.2)"))))
                                                    }
                                                    Some(v14) => {
                                                        let m996 = f210(v14.clone(), v9, v11, v2)?;
                                                        match m996 {
                                                            Ok(v16) => {
                                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::cons((f211(v14)?, (match v16.clone() {
                                                                    Adt24::C0(_, h997) => {
                                                                        h997
                                                                    }
                                                                }, match v16 {
                                                                    Adt24::C0(h998, _) => {
                                                                        h998
                                                                    }
                                                                })), v6)))
                                                            }
                                                            Err(v15) => {
                                                                Ok(f209(v15))
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                                            }
                                        }
                                        Err(v12) => {
                                            Ok(f209(v12))
                                        }
                                    }
                                }
                                Err(v10) => {
                                    Ok(f209(v10))
                                }
                            }
                        }
                        Adt18::C1(_, _, _) => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                        Adt18::C2(_) => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                        Adt18::C3(_, _) => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                        Adt18::C4 => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                    }
                }
                Err(v5) => {
                    Ok(f209(v5))
                }
            }
        }
        Adt18::C4 => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
    }
}

pub fn f137(v0: List<(Bytes, (List<Bytes>, Bytes))>) -> List<Bytes> {
    let m1003 = v0;
    match m1003.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h1004, _) = v1;
                h1004
            }, f137(v2))
        }
    }
}

pub fn f138(p1007: List<Bytes>) -> Option<Bytes> {
    let m1006 = p1007;
    {
        let mut m1006 = m1006;
        loop {
            let step = {
                let v0 = m1006;
                let m1005 = v0;
                match m1005.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f173(v1.clone(), v2.clone()) {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v1))
                        } else {
                            Ok::<List<Bytes>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1006 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f139(p1013: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<Bytes> {
    let m1012 = p1013;
    {
        let mut m1012 = m1012;
        loop {
            let step = {
                let v0 = m1012;
                let m1008 = v0;
                match m1008.uncons() {
                    None => {
                        Err::<List<(Bytes, (List<Bytes>, Bytes))>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        let m1011 = f212({
                            let (h1009, _) = {
                                let (_, h1010) = v2;
                                h1010
                            };
                            h1009
                        }, v1.clone());
                        match m1011 {
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
                Ok(next) => { m1012 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f140(p1024: u64, p1025: List<(Bytes, (List<Bytes>, Bytes))>, p1026: List<Bytes>, p1027: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m1023 = (p1024, (p1025, (p1026, p1027)));
    {
        let mut m1023 = m1023;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m1023;
                let m1014 = v0;
                if m1014 == 0 {
                    let m1015 = v1;
                    match m1015.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(f213(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                        }
                    }
                } else {
                    let v6 = m1014 - 1;
                    let m1016 = v1.clone();
                    match m1016.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            let m1017 = f180(v1, v2.clone());
                            match m1017 {
                                None => {
                                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(f213(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                                }
                                Some(v9) => {
                                    Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>((v6, ({
                                        let (_, h1018) = v9.clone();
                                        h1018
                                    }, (List::cons({
                                        let (h1019, _) = {
                                            let (h1020, _) = v9.clone();
                                            h1020
                                        };
                                        h1019
                                    }, v2), List::cons({
                                        let (_, h1021) = {
                                            let (h1022, _) = v9;
                                            h1022
                                        };
                                        h1021
                                    }, v3)))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1023 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f141(v0: Bytes) -> R<List<(Bytes, Bytes)>> {
    f214(f215(v0)?, None::<Bytes>)
}

pub fn f142(p1048: u64, v1: List<(Bytes, Option<Bytes>)>, v2: List<(Bytes, Bytes)>, p1049: List<(Bytes, (Bytes, Bytes))>, p1050: List<Adt20>, p1051: List<Bytes>) -> R<Adt19> {
    let m1047 = (p1048, (p1049, (p1050, p1051)));
    {
        let mut m1047 = m1047;
        loop {
            let step = {
                let (v0, (v3, (v4, v5))) = m1047;
                let m1028 = v0;
                if m1028 == 0 {
                    Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt20>, List<Bytes>))), R<Adt19>>(Ok(Adt19::C1(f8(Str::lit("the schema documents import and include more documents than the bound authorities hold")))))
                } else {
                    let v6 = m1028 - 1;
                    let m1029 = v3;
                    match m1029.uncons() {
                        None => {
                            let m1030 = v5.clone();
                            match m1030.uncons() {
                                None => {
                                    let m1031 = f216(f217(v4, List::<Adt20>::nil()))?;
                                    match m1031 {
                                        Ok(v8) => {
                                            Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt20>, List<Bytes>))), R<Adt19>>(Ok(Adt19::C2({
                                                let (h1032, _) = v8;
                                                h1032
                                            }, {
                                                let (_, h1033) = v8;
                                                h1033
                                            })))
                                        }
                                        Err(v7) => {
                                            Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt20>, List<Bytes>))), R<Adt19>>(Ok(Adt19::C1(v7)))
                                        }
                                    }
                                }
                                Some((_, _)) => {
                                    Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt20>, List<Bytes>))), R<Adt19>>(Ok(Adt19::C0(v5)))
                                }
                            }
                        }
                        Some((v11, v12)) => {
                            let v13: Bytes = {
                                let (h1034, _) = v11.clone();
                                h1034
                            };
                            if if f173(v13.clone(), f218(v4.clone())) {
                                true
                            } else {
                                f173(v13.clone(), v5.clone())
                            } {
                                Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt20>, List<Bytes>))), R<Adt19>>((v6, (v12, (v4, v5))))
                            } else {
                                let m1035 = f219(v1.clone(), v13.clone());
                                match m1035 {
                                    None => {
                                        Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt20>, List<Bytes>))), R<Adt19>>((v6, (v12, (v4, List::cons(v13, v5)))))
                                    }
                                    Some(v14) => {
                                        let m1036 = v14;
                                        match m1036 {
                                            None => {
                                                Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt20>, List<Bytes>))), R<Adt19>>(Ok(Adt19::C1(f220(f8(Str::lit("the schema document ")), v13, f8(Str::lit(" is not in the repository"))))))
                                            }
                                            Some(v15) => {
                                                let m1037 = f221(v13.clone(), v15, f222(v2.clone(), v13))?;
                                                match m1037 {
                                                    Ok(v17) => {
                                                        let m1042 = f223(v17.clone(), {
                                                            let (h1038, _) = {
                                                                let (_, h1039) = v11.clone();
                                                                h1039
                                                            };
                                                            h1038
                                                        }, {
                                                            let (_, h1040) = {
                                                                let (_, h1041) = v11;
                                                                h1041
                                                            };
                                                            h1040
                                                        });
                                                        match m1042 {
                                                            None => {
                                                                let m1045 = f224(match v17.clone() {
                                                                    Adt20::C0(_, _, _, _, h1043, _) => {
                                                                        h1043
                                                                    }
                                                                }, match v17.clone() {
                                                                    Adt20::C0(_, _, _, h1044, _, _) => {
                                                                        h1044
                                                                    }
                                                                })?;
                                                                match m1045 {
                                                                    Ok(v20) => {
                                                                        let m1046 = f225(v20, v17.clone(), v2.clone())?;
                                                                        match m1046 {
                                                                            Ok(v22) => {
                                                                                Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt20>, List<Bytes>))), R<Adt19>>((v6, (f226(v12, v22), (List::cons(v17, v4), v5))))
                                                                            }
                                                                            Err(v21) => {
                                                                                Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt20>, List<Bytes>))), R<Adt19>>(Ok(Adt19::C1(v21)))
                                                                            }
                                                                        }
                                                                    }
                                                                    Err(v19) => {
                                                                        Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt20>, List<Bytes>))), R<Adt19>>(Ok(Adt19::C1(v19)))
                                                                    }
                                                                }
                                                            }
                                                            Some(v18) => {
                                                                Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt20>, List<Bytes>))), R<Adt19>>(Ok(Adt19::C1(v18)))
                                                            }
                                                        }
                                                    }
                                                    Err(v16) => {
                                                        Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt20>, List<Bytes>))), R<Adt19>>(Ok(Adt19::C1(v16)))
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
                Ok(next) => { m1047 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f143(v0: List<List<(Bytes, Option<Bytes>)>>) -> List<(Bytes, Option<Bytes>)> {
    let m1052 = v0;
    match m1052.uncons() {
        None => {
            List::<(Bytes, Option<Bytes>)>::nil()
        }
        Some((v1, v2)) => {
            f227(v1, f143(v2))
        }
    }
}

pub fn f144(v0: Bytes) -> R<u64> {
    let (h1053, _) = f188(Fn5::F228(v0.clone()), {
        let a1055 = {
            let a1054 = v0;
            length_bytes(a1054)
        };
        let a1056 = 1u64;
        nat_add(a1055, a1056)?
    }, 0u64)?;
    Ok(h1053)
}

pub fn f145(v0: Bytes, v1: u64) -> R<u64> {
    let (h1057, _) = f188(Fn5::F229(v0.clone()), {
        let a1059 = {
            let a1058 = v0;
            length_bytes(a1058)
        };
        let a1060 = 1u64;
        nat_add(a1059, a1060)?
    }, v1)?;
    Ok(h1057)
}

pub fn f146(v0: Bytes, p1069: u64, p1070: u64) -> R<Result<u64, Bytes>> {
    let m1068 = (p1069, p1070);
    {
        let mut m1068 = m1068;
        loop {
            let step = {
                let (v1, v2) = m1068;
                let m1061 = v2;
                if m1061 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Ok::<u64, Bytes>(v1)))
                } else {
                    let v3 = m1061 - 1;
                    let a1063 = v1;
                    let a1064 = {
                        let a1062 = v0.clone();
                        length_bytes(a1062)
                    };
                    if if nat_lt(a1063, a1064) {
                        if f128(v0.clone(), v1, Str::lit("...")) {
                            f230(v0.clone(), v1)?
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        let m1067 = f231(v0.clone(), {
                            let a1065 = v1;
                            let a1066 = 3u64;
                            nat_add(a1065, a1066)?
                        })?;
                        match m1067 {
                            Ok(v5) => {
                                Ok::<(u64, u64), R<Result<u64, Bytes>>>((f145(v0.clone(), v5)?, v3))
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
                Ok(next) => { m1068 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f147(v0: Bytes, v1: u64, v2: bool, v3: u64) -> R<Result<List<Adt2>, Bytes>> {
    let m1071 = v3;
    if m1071 == 0 {
        Ok(Err::<List<Adt2>, Bytes>(f125(Str::lit("the stream holds more documents than it is long"), v1)?))
    } else {
        let v4 = m1071 - 1;
        let v5: u64 = {
            let a1072 = v0.clone();
            length_bytes(a1072)
        };
        let a1073 = f232(v0.clone(), v1)?;
        let a1074 = 37u64;
        if nat_eq(a1073, a1074) {
            Ok(Err::<List<Adt2>, Bytes>(f125(Str::lit("a directive is not read (YAML 1.2.2 [82])"), v1)?))
        } else {
            let m1081 = if if f128(v0.clone(), v1, Str::lit("---")) {
                f230(v0.clone(), v1)?
            } else {
                false
            } {
                f233(v0.clone(), {
                    let a1075 = v1;
                    let a1076 = 3u64;
                    nat_add(a1075, a1076)?
                }, 0u64, {
                    let a1077 = v5;
                    let a1078 = 2u64;
                    nat_add(a1077, a1078)?
                }, 1u64, false, 0u64, List::<Bytes>::nil())?
            } else {
                if v2 {
                    f233(v0.clone(), v1, 0u64, {
                        let a1079 = v5;
                        let a1080 = 2u64;
                        nat_add(a1079, a1080)?
                    }, 0u64, false, 0u64, List::<Bytes>::nil())?
                } else {
                    Adt22::C1(f125(Str::lit("only comments, a document end marker or a document start marker may follow a document (YAML 1.2.2 [207], [211])"), v1)?)
                }
            };
            match m1081 {
                Adt22::C0(v7, v8) => {
                    let v9: u64 = f145(v0.clone(), v8)?;
                    let m1084 = f146(v0.clone(), v9, {
                        let a1082 = v5;
                        let a1083 = 1u64;
                        nat_add(a1082, a1083)?
                    })?;
                    match m1084 {
                        Ok(v11) => {
                            let a1085 = v5;
                            let a1086 = v11;
                            if nat_le(a1085, a1086) {
                                Ok(Ok::<List<Adt2>, Bytes>(List::cons(v7, List::<Adt2>::nil())))
                            } else {
                                let m1089 = f147(v0, v11, f234({
                                    let a1087 = v11;
                                    let a1088 = v9;
                                    nat_eq(a1087, a1088)
                                }), v4)?;
                                match m1089 {
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

pub fn f148(v0: Adt2) -> R<Bytes> {
    f235(v0)
}

pub fn f149(v0: Bytes, v1: u64, v2: u64, v3: Bytes) -> Bytes {
    let a1092 = v3;
    let a1093 = f45(v0, v1, {
        let a1090 = v2;
        let a1091 = v1;
        nat_sub(a1090, a1091)
    });
    append_bytes(a1092, a1093)
}

pub fn f150(v0: Bytes, p1106: u64, p1107: u64) -> R<u64> {
    let m1105 = (p1106, p1107);
    {
        let mut m1105 = m1105;
        loop {
            let step = {
                let (v1, v2) = m1105;
                let m1094 = v2;
                if m1094 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m1094 - 1;
                    let v4: u64 = f94(v0.clone(), v1)?;
                    let a1095 = v4;
                    let a1096 = 34u64;
                    if nat_eq(a1095, a1096) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a1097 = v4;
                        let a1098 = 92u64;
                        if nat_eq(a1097, a1098) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a1099 = v4;
                            let a1100 = 32u64;
                            if nat_lt(a1099, a1100) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a1101 = v4;
                                let a1102 = 256u64;
                                if nat_eq(a1101, a1102) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a1103 = v1;
                                        let a1104 = 1u64;
                                        nat_add(a1103, a1104)?
                                    }, v3))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1105 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f151(v0: Bytes, p1133: u64, p1134: u64, p1135: Bytes) -> R<Option<(u64, Bytes)>> {
    let m1132 = (p1133, (p1134, p1135));
    {
        let mut m1132 = m1132;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1132;
                let m1108 = v2;
                if m1108 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                } else {
                    let v4 = m1108 - 1;
                    let v5: u64 = f150(v0.clone(), v1, {
                        let a1112 = {
                            let a1110 = {
                                let a1109 = v0.clone();
                                length_bytes(a1109)
                            };
                            let a1111 = v1;
                            nat_sub(a1110, a1111)
                        };
                        let a1113 = 1u64;
                        nat_add(a1112, a1113)?
                    })?;
                    let v6: Bytes = {
                        let a1116 = v3;
                        let a1117 = f45(v0.clone(), v1, {
                            let a1114 = v5;
                            let a1115 = v1;
                            nat_sub(a1114, a1115)
                        });
                        append_bytes(a1116, a1117)
                    };
                    let v7: u64 = f94(v0.clone(), v5)?;
                    let a1118 = v7;
                    let a1119 = 34u64;
                    if nat_eq(a1118, a1119) {
                        Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(Some(({
                            let a1120 = v5;
                            let a1121 = 1u64;
                            nat_add(a1120, a1121)?
                        }, v6))))
                    } else {
                        let a1122 = v7;
                        let a1123 = 92u64;
                        if nat_eq(a1122, a1123) {
                            let m1124 = f236(v0.clone(), v5, v6)?;
                            match m1124 {
                                None => {
                                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                                }
                                Some(v8) => {
                                    let a1127 = {
                                        let (_, h1125) = {
                                            let (_, h1126) = v8.clone();
                                            h1126
                                        };
                                        h1125
                                    };
                                    let a1128 = 0u64;
                                    if nat_eq(a1127, a1128) {
                                        Ok::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(({
                                            let (h1129, _) = v8;
                                            h1129
                                        }, (v4, {
                                            let (h1130, _) = {
                                                let (_, h1131) = v8;
                                                h1131
                                            };
                                            h1130
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
                Ok(next) => { m1132 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f152(v0: Bytes, v1: u64) -> R<u64> {
    let (h1136, _) = f188(Fn5::F237(v0.clone()), {
        let a1138 = {
            let a1137 = v0;
            length_bytes(a1137)
        };
        let a1139 = 1u64;
        nat_add(a1138, a1139)?
    }, v1)?;
    Ok(h1136)
}

pub fn f153(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f154(v0: Bytes) -> R<u64> {
    let (h1140, _) = f188(Fn5::F238(v0.clone()), {
        let a1142 = {
            let a1141 = v0;
            length_bytes(a1141)
        };
        let a1143 = 1u64;
        nat_add(a1142, a1143)?
    }, 0u64)?;
    Ok(h1140)
}

pub fn f155(v0: Bytes) -> R<Option<u64>> {
    if f239(v0.clone())? {
        Ok(Some({
            let (_, h1144) = {
                let (h1145, _) = f240(Fn6::F241(v0.clone()), {
                    let a1146 = v0;
                    length_bytes(a1146)
                }, (0u64, 0u64))?;
                h1145
            };
            h1144
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f156(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f45(v1.clone(), 0u64, {
        let a1148 = {
            let a1147 = v1.clone();
            length_bytes(a1147)
        };
        let a1149 = f242(v1)?;
        nat_sub(a1148, a1149)
    });
    let v5: Bytes = {
        let a1150 = v0;
        let a1151 = v4.clone();
        append_bytes(a1150, a1151)
    };
    let v6: u64 = f154(v5.clone())?;
    let v7: Bytes = f45(v5.clone(), v6, {
        let a1153 = {
            let a1152 = v5;
            length_bytes(a1152)
        };
        let a1154 = v6;
        nat_sub(a1153, a1154)
    });
    let v8: u64 = {
        let a1155 = v4;
        length_bytes(a1155)
    };
    let v9: u64 = {
        let a1156 = v7.clone();
        length_bytes(a1156)
    };
    let a1157 = v9;
    let a1158 = 0u64;
    if nat_eq(a1157, a1158) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f153(v2, {
            let a1159 = v3;
            let a1160 = v8;
            nat_lt(a1159, a1160)
        }) {
            let v10: u64 = if v2 {
                let a1161 = v3;
                let a1162 = v8;
                nat_add(a1161, a1162)?
            } else {
                let a1163 = v8;
                let a1164 = v3;
                nat_sub(a1163, a1164)
            };
            let v11: u64 = f242(v7.clone())?;
            let a1165 = v10;
            let a1166 = v11;
            if nat_le(a1165, a1166) {
                Ok(Some((f45(v7, 0u64, {
                    let a1167 = v9;
                    let a1168 = v10;
                    nat_sub(a1167, a1168)
                }), 0u64)))
            } else {
                let a1171 = {
                    let a1169 = v10;
                    let a1170 = v11;
                    nat_sub(a1169, a1170)
                };
                let a1172 = 19u64;
                if nat_le(a1171, a1172) {
                    Ok(Some((f45(v7, 0u64, {
                        let a1173 = v9;
                        let a1174 = v11;
                        nat_sub(a1173, a1174)
                    }), {
                        let a1175 = v10;
                        let a1176 = v11;
                        nat_sub(a1175, a1176)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a1177 = v3;
                let a1178 = v8;
                nat_sub(a1177, a1178)
            };
            let a1181 = {
                let a1179 = v9;
                let a1180 = v12;
                nat_add(a1179, a1180)?
            };
            let a1182 = 19u64;
            if nat_le(a1181, a1182) {
                Ok(Some(({
                    let a1183 = v7;
                    let a1184 = f243(v12);
                    append_bytes(a1183, a1184)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f157(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1185 = 0u64;
        let a1186 = v1;
        if nat_lt(a1185, a1186) {
            let a1187 = v1;
            let a1188 = 9223372036854775808u64;
            nat_le(a1187, a1188)
        } else {
            false
        }
    } else {
        let a1189 = v1;
        let a1190 = 9223372036854775807u64;
        nat_le(a1189, a1190)
    }
}

pub fn f158(v0: Bytes, v1: u64) -> Option<u8> {
    let a1191 = v0;
    let a1192 = v1;
    index_bytes(a1191, a1192)
}

pub fn f159(v0: u32) -> Bytes {
    let a1221 = f244({
        let a1193 = f245(v0, 28u32);
        let a1194 = 15u32;
        fixed_u32::bit_and(a1193, a1194)
    });
    let a1222 = {
        let a1219 = f244({
            let a1195 = f245(v0, 24u32);
            let a1196 = 15u32;
            fixed_u32::bit_and(a1195, a1196)
        });
        let a1220 = {
            let a1217 = f244({
                let a1197 = f245(v0, 20u32);
                let a1198 = 15u32;
                fixed_u32::bit_and(a1197, a1198)
            });
            let a1218 = {
                let a1215 = f244({
                    let a1199 = f245(v0, 16u32);
                    let a1200 = 15u32;
                    fixed_u32::bit_and(a1199, a1200)
                });
                let a1216 = {
                    let a1213 = f244({
                        let a1201 = f245(v0, 12u32);
                        let a1202 = 15u32;
                        fixed_u32::bit_and(a1201, a1202)
                    });
                    let a1214 = {
                        let a1211 = f244({
                            let a1203 = f245(v0, 8u32);
                            let a1204 = 15u32;
                            fixed_u32::bit_and(a1203, a1204)
                        });
                        let a1212 = {
                            let a1209 = f244({
                                let a1205 = f245(v0, 4u32);
                                let a1206 = 15u32;
                                fixed_u32::bit_and(a1205, a1206)
                            });
                            let a1210 = f244({
                                let a1207 = v0;
                                let a1208 = 15u32;
                                fixed_u32::bit_and(a1207, a1208)
                            });
                            append_bytes(a1209, a1210)
                        };
                        append_bytes(a1211, a1212)
                    };
                    append_bytes(a1213, a1214)
                };
                append_bytes(a1215, a1216)
            };
            append_bytes(a1217, a1218)
        };
        append_bytes(a1219, a1220)
    };
    append_bytes(a1221, a1222)
}

pub fn f160(v0: Fn3, p1226: u64, p1227: (u64, Adt21)) -> R<(u64, Adt21)> {
    let m1225 = (p1226, p1227);
    {
        let mut m1225 = m1225;
        loop {
            let step = {
                let (v1, v2) = m1225;
                let m1223 = v1;
                if m1223 == 0 {
                    Err::<(u64, (u64, Adt21)), R<(u64, Adt21)>>(Ok(v2))
                } else {
                    let v3 = m1223 - 1;
                    Ok::<(u64, (u64, Adt21)), R<(u64, Adt21)>>((v3, {
                        let c1224: Fn3 = v0.clone();
                        c1224.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1225 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f161(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt21)) -> R<(u64, Adt21)> {
    Ok(({
        let a1229 = {
            let (h1228, _) = v3.clone();
            h1228
        };
        let a1230 = 64u64;
        nat_add(a1229, a1230)?
    }, f246({
        let (_, h1231) = v3;
        h1231
    }, f247(v0, v1, v2, {
        let (h1232, _) = v3;
        h1232
    })?)?))
}

pub fn f162() -> Adt21 {
    Adt21::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f163(p1237: List<(Bytes, Option<Adt2>)>, v1: Bytes) -> Option<Option<Adt2>> {
    let m1236 = p1237;
    {
        let mut m1236 = m1236;
        loop {
            let step = {
                let v0 = m1236;
                let m1233 = v0;
                match m1233.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(None::<Option<Adt2>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h1234, _) = v2;
                            h1234
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(Some({
                                let (_, h1235) = v2;
                                h1235
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1236 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f164(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Adt2>)> {
    let m1241 = {
        let a1239 = match v0 {
            Adt1::C0(_, _, _, h1238) => {
                h1238
            }
        };
        let a1240 = f39(v1);
        index_list(a1239, a1240)
    };
    match m1241 {
        None => {
            List::<(Bytes, Option<Adt2>)>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f165(v0: Bytes) -> R<Result<List<Adt23>, Bytes>> {
    f248({
        let a1243 = {
            let a1242 = v0.clone();
            length_bytes(a1242)
        };
        let a1244 = 1u64;
        nat_add(a1243, a1244)?
    }, v0, 0u64, List::<Adt23>::nil())
}

pub fn f166(p1252: u64, p1253: List<Adt23>, p1254: List<Adt14>) -> R<Result<List<Adt14>, Bytes>> {
    let m1251 = (p1252, (p1253, p1254));
    {
        let mut m1251 = m1251;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1251;
                let m1245 = v0;
                if m1245 == 0 {
                    Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<List<Adt14>, Bytes>>>(Ok(Err::<List<Adt14>, Bytes>(f8(Str::lit("the input has more definitions than tokens")))))
                } else {
                    let v3 = m1245 - 1;
                    let m1246 = v1.clone();
                    match m1246.uncons() {
                        None => {
                            Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<List<Adt14>, Bytes>>>(Ok(Ok::<List<Adt14>, Bytes>(f249(v2, List::<Adt14>::nil()))))
                        }
                        Some((_, _)) => {
                            let m1247 = f250(v1)?;
                            match m1247 {
                                Ok(v7) => {
                                    let m1248 = f251(v3, v7)?;
                                    match m1248 {
                                        Ok(v9) => {
                                            Ok::<(u64, (List<Adt23>, List<Adt14>)), R<Result<List<Adt14>, Bytes>>>((v3, ({
                                                let (_, h1249) = v9;
                                                h1249
                                            }, List::cons({
                                                let (h1250, _) = v9;
                                                h1250
                                            }, v2))))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<List<Adt14>, Bytes>>>(Ok(Err::<List<Adt14>, Bytes>(v8)))
                                        }
                                    }
                                }
                                Err(v6) => {
                                    Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<List<Adt14>, Bytes>>>(Ok(Err::<List<Adt14>, Bytes>(v6)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1251 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f167(v0: List<Adt17>, v1: List<Adt17>) -> List<Adt17> {
    let m1255 = v0;
    match m1255.uncons() {
        None => {
            List::<Adt17>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = if f32(f175(v2.clone()), Bytes::lit(&[])) {
                f252(f177(v2.clone()), v1.clone())
            } else {
                f175(v2.clone())
            };
            List::cons(Adt17::C0(f178(v2.clone()), v4, f179(v2.clone()), f177(v2)), f167(v3, v1))
        }
    }
}

pub fn f168(v0: List<Adt17>) -> R<u64> {
    let m1256 = v0;
    match m1256.uncons() {
        None => {
            Ok(0u64)
        }
        Some((v1, v2)) => {
            if f32(f175(v1), Bytes::lit(&[])) {
                f168(v2)
            } else {
                let a1257 = f168(v2)?;
                let a1258 = 1u64;
                nat_add(a1257, a1258)
            }
        }
    }
}

pub fn f169(v0: u64) -> bool {
    let a1259 = v0;
    let a1260 = 0u64;
    if nat_eq(a1259, a1260) {
        true
    } else {
        let a1261 = v0;
        let a1262 = 2u64;
        if nat_eq(a1261, a1262) {
            true
        } else {
            let a1263 = v0;
            let a1264 = 3u64;
            if nat_eq(a1263, a1264) {
                true
            } else {
                let a1265 = v0;
                let a1266 = 4u64;
                if nat_eq(a1265, a1266) {
                    true
                } else {
                    let a1267 = v0;
                    let a1268 = 9u64;
                    nat_eq(a1267, a1268)
                }
            }
        }
    }
}

pub fn f170(v0: Adt14) -> u64 {
    let m1269 = v0;
    match m1269 {
        Adt14::C0(v1, _, _, _, _, _) => {
            v1
        }
    }
}

pub fn f171(v0: List<Adt14>, v1: Adt14) -> R<Result<(Bytes, List<Bytes>), Bytes>> {
    let v2: u64 = f170(v1.clone());
    let v3: Bytes = f253(f172(v1.clone()))?;
    let a1270 = v2;
    let a1271 = 0u64;
    if if nat_eq(a1270, a1271) {
        true
    } else {
        let a1272 = v2;
        let a1273 = 2u64;
        nat_eq(a1272, a1273)
    } {
        let m1274 = f254(v0.clone(), v0.clone(), f172(v1.clone()));
        match m1274 {
            None => {
                let m1278 = f255({
                    let a1276 = {
                        let a1275 = v0.clone();
                        length_list(a1275)
                    };
                    let a1277 = 1u64;
                    nat_add(a1276, a1277)?
                }, v0, v1)?;
                match m1278 {
                    Ok(v5) => {
                        let a1280 = {
                            let a1279 = v5.clone();
                            length_list(a1279)
                        };
                        let a1281 = 0u64;
                        if nat_eq(a1280, a1281) {
                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f256(v3), List::<Bytes>::nil())))
                        } else {
                            let m1282 = f257(f258(f259(v5.clone()))?, v5)?;
                            match m1282 {
                                Ok(v7) => {
                                    Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f260(v3, {
                                        let (h1283, _) = v7;
                                        h1283
                                    }), {
                                        let (_, h1284) = v7;
                                        h1284
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
        let a1285 = v2;
        let a1286 = 3u64;
        if nat_eq(a1285, a1286) {
            let m1287 = f138(f261(v1.clone()));
            match m1287 {
                None => {
                    let m1288 = f262(f258(f261(v1.clone()))?, f263(f261(v1)), List::<Bytes>::nil())?;
                    match m1288 {
                        Ok(v10) => {
                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f264(v3, v10), List::<Bytes>::nil())))
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
            let a1289 = v2;
            let a1290 = 4u64;
            if nat_eq(a1289, a1290) {
                let v12: Adt16 = f265(v1);
                if !f32(f266(v12.clone()), Bytes::lit(&[])) {
                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f266(v12)))
                } else {
                    let a1291 = f267(v12.clone());
                    let a1292 = 1u64;
                    if nat_eq(a1291, a1292) {
                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f264(v3, List::cons(f268(v12.clone()), List::<Bytes>::nil())), f269(v12))))
                    } else {
                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f270(v3, f268(v12.clone())), f269(v12))))
                    }
                }
            } else {
                Ok(Err::<(Bytes, List<Bytes>), Bytes>(f7(f261(v1))))
            }
        }
    }
}

pub fn f172(v0: Adt14) -> Bytes {
    let m1293 = v0;
    match m1293 {
        Adt14::C0(_, v2, _, _, _, _) => {
            v2
        }
    }
}

pub fn f173(v0: Bytes, p1296: List<Bytes>) -> bool {
    let m1295 = p1296;
    {
        let mut m1295 = m1295;
        loop {
            let step = {
                let v1 = m1295;
                let m1294 = v1;
                match m1294.uncons() {
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
                Ok(next) => { m1295 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f174(p1299: List<Adt17>, v1: Bytes) -> Option<Adt17> {
    let m1298 = p1299;
    {
        let mut m1298 = m1298;
        loop {
            let step = {
                let v0 = m1298;
                let m1297 = v0;
                match m1297.uncons() {
                    None => {
                        Err::<List<Adt17>, Option<Adt17>>(None::<Adt17>)
                    }
                    Some((v2, v3)) => {
                        if f32(f178(v2.clone()), v1.clone()) {
                            Err::<List<Adt17>, Option<Adt17>>(Some(v2))
                        } else {
                            Ok::<List<Adt17>, Option<Adt17>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1298 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f175(v0: Adt17) -> Bytes {
    let m1300 = v0;
    match m1300 {
        Adt17::C0(_, v2, _, _) => {
            v2
        }
    }
}

pub fn f176(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m1301 = v0;
    match m1301.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f176(v3, v1))
        }
    }
}

pub fn f177(v0: Adt17) -> List<Bytes> {
    let m1302 = v0;
    match m1302 {
        Adt17::C0(_, _, _, v4) => {
            v4
        }
    }
}

pub fn f178(v0: Adt17) -> Bytes {
    let m1303 = v0;
    match m1303 {
        Adt17::C0(v1, _, _, _) => {
            v1
        }
    }
}

pub fn f179(v0: Adt17) -> Bytes {
    let m1304 = v0;
    match m1304 {
        Adt17::C0(_, _, v3, _) => {
            v3
        }
    }
}

pub fn f180(v0: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)> {
    let m1305 = v0;
    match m1305.uncons() {
        None => {
            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
        }
        Some((v2, v3)) => {
            let m1308 = f212({
                let (h1306, _) = {
                    let (_, h1307) = v2.clone();
                    h1307
                };
                h1306
            }, v1.clone());
            match m1308 {
                None => {
                    Some((({
                        let (h1309, _) = v2;
                        h1309
                    }, {
                        let (_, h1310) = {
                            let (_, h1311) = v2;
                            h1311
                        };
                        h1310
                    }), v3))
                }
                Some(_) => {
                    let m1312 = f180(v3, v1);
                    match m1312 {
                        None => {
                            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h1313, _) = v5;
                                h1313
                            }, List::cons(v2, {
                                let (_, h1314) = v5;
                                h1314
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f181(p1319: List<Adt14>, v1: u64, v2: Bytes) -> Option<Adt14> {
    let m1318 = p1319;
    {
        let mut m1318 = m1318;
        loop {
            let step = {
                let v0 = m1318;
                let m1315 = v0;
                match m1315.uncons() {
                    None => {
                        Err::<List<Adt14>, Option<Adt14>>(None::<Adt14>)
                    }
                    Some((v3, v4)) => {
                        let a1316 = f170(v3.clone());
                        let a1317 = v1;
                        if if nat_eq(a1316, a1317) {
                            f32(f172(v3.clone()), v2.clone())
                        } else {
                            false
                        } {
                            Err::<List<Adt14>, Option<Adt14>>(Some(v3))
                        } else {
                            Ok::<List<Adt14>, Option<Adt14>>(v4)
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

pub fn f182(v0: u64) -> Bytes {
    let a1320 = v0;
    let a1321 = 5u64;
    if nat_eq(a1320, a1321) {
        f8(Str::lit("partial interface "))
    } else {
        let a1322 = v0;
        let a1323 = 7u64;
        if nat_eq(a1322, a1323) {
            f8(Str::lit("partial dictionary "))
        } else {
            f8(Str::lit(""))
        }
    }
}

pub fn f183(v0: Adt14) -> Bytes {
    let m1324 = v0;
    match m1324 {
        Adt14::C0(_, _, v3, _, _, _) => {
            v3
        }
    }
}

pub fn f184(v0: Bytes, v1: u64) -> Bytes {
    let m1328 = {
        let a1325 = v0;
        let a1326 = v1;
        let a1327 = 1u64;
        slice_bytes(a1325, a1326, a1327)
    };
    match m1328 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f185(v0: Fn4, p1333: u64, p1334: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1332 = (p1333, p1334);
    {
        let mut m1332 = m1332;
        loop {
            let step = {
                let (v1, v2) = m1332;
                let m1329 = v1;
                if m1329 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1329 - 1;
                    let m1331 = {
                        let c1330: Fn4 = v0.clone();
                        c1330.apply(v2.clone())?
                    };
                    match m1331 {
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
                Ok(next) => { m1332 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f186(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f271(v1, v0, v2)
}

pub fn f187(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f272(v0, v1)
}

pub fn f188(v0: Fn5, p1339: u64, p1340: u64) -> R<(u64, bool)> {
    let m1338 = (p1339, p1340);
    {
        let mut m1338 = m1338;
        loop {
            let step = {
                let (v1, v2) = m1338;
                let m1335 = v1;
                if m1335 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1335 - 1;
                    let m1337 = {
                        let c1336: Fn5 = v0.clone();
                        c1336.apply(v2)?
                    };
                    match m1337 {
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
                Ok(next) => { m1338 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f189(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f273(v0, v1)
}

pub fn f190(v0: u64) -> R<bool> {
    f274(v0, f275())
}

pub fn f191(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let v3: u64 = {
        let (h1341, _) = f188(Fn5::F276(v1.clone(), v0.clone()), {
            let a1343 = {
                let a1342 = v0.clone();
                length_bytes(a1342)
            };
            let a1344 = 1u64;
            nat_add(a1343, a1344)?
        }, v2)?;
        h1341
    };
    if f121(v0, v1, v3) {
        Ok(Some(v3))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f192(v0: Bytes) -> Result<u64, Bytes> {
    Err::<u64, Bytes>(v0)
}

pub fn f193(v0: Bytes, v1: u64) -> R<u64> {
    let (h1345, _) = f188(Fn5::F277(v0.clone()), {
        let a1347 = {
            let a1346 = v0;
            length_bytes(a1346)
        };
        let a1348 = 1u64;
        nat_add(a1347, a1348)?
    }, v1)?;
    Ok(h1345)
}

pub fn f194(v0: Bytes) -> bool {
    f278(v0, List::cons(f8(Str::lit("UTF-8")), List::cons(f8(Str::lit("UTf-8")), List::cons(f8(Str::lit("UtF-8")), List::cons(f8(Str::lit("Utf-8")), List::cons(f8(Str::lit("uTF-8")), List::cons(f8(Str::lit("uTf-8")), List::cons(f8(Str::lit("utF-8")), List::cons(f8(Str::lit("utf-8")), List::<Bytes>::nil())))))))))
}

pub fn f195(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let m1352 = f191(v0.clone(), {
        let a1349 = Str::lit("--");
        utf8_encode(a1349)
    }, {
        let a1350 = v1;
        let a1351 = 4u64;
        nat_add(a1350, a1351)?
    })?;
    match m1352 {
        None => {
            Ok(f192(f125(Str::lit("a comment is not closed (XML 1.0 [15])"), v1)?))
        }
        Some(v2) => {
            let a1355 = f129(v0, {
                let a1353 = v2;
                let a1354 = 2u64;
                nat_add(a1353, a1354)?
            })?;
            let a1356 = 62u64;
            if nat_eq(a1355, a1356) {
                Ok(Ok::<u64, Bytes>({
                    let a1357 = v2;
                    let a1358 = 3u64;
                    nat_add(a1357, a1358)?
                }))
            } else {
                Ok(f192(f125(Str::lit("-- is not allowed inside a comment (XML 1.0 [15])"), v2)?))
            }
        }
    }
}

pub fn f196(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f198(v0.clone(), {
        let a1359 = v1;
        let a1360 = 2u64;
        nat_add(a1359, a1360)?
    })?;
    let a1363 = v2;
    let a1364 = {
        let a1361 = v1;
        let a1362 = 2u64;
        nat_add(a1361, a1362)?
    };
    if nat_eq(a1363, a1364) {
        Ok(f192(f125(Str::lit("a processing instruction must start with a Name (XML 1.0 [16])"), v1)?))
    } else {
        let a1367 = v2;
        let a1368 = {
            let a1365 = v1;
            let a1366 = 5u64;
            nat_add(a1365, a1366)?
        };
        if if nat_eq(a1367, a1368) {
            f278(f45(v0.clone(), {
                let a1369 = v1;
                let a1370 = 2u64;
                nat_add(a1369, a1370)?
            }, 3u64), List::cons(f8(Str::lit("XML")), List::cons(f8(Str::lit("XMl")), List::cons(f8(Str::lit("XmL")), List::cons(f8(Str::lit("Xml")), List::cons(f8(Str::lit("xML")), List::cons(f8(Str::lit("xMl")), List::cons(f8(Str::lit("xmL")), List::cons(f8(Str::lit("xml")), List::<Bytes>::nil())))))))))
        } else {
            false
        } {
            Ok(f192(f125(Str::lit("the target xml is reserved; an XML declaration may appear only at the very start (XML 1.0 [17], [22])"), v1)?))
        } else {
            let m1372 = f191(v0, {
                let a1371 = Str::lit("?>");
                utf8_encode(a1371)
            }, v2)?;
            match m1372 {
                None => {
                    Ok(f192(f125(Str::lit("a processing instruction is not closed (XML 1.0 [16])"), v1)?))
                }
                Some(v3) => {
                    Ok(Ok::<u64, Bytes>({
                        let a1373 = v3;
                        let a1374 = 2u64;
                        nat_add(a1373, a1374)?
                    }))
                }
            }
        }
    }
}

pub fn f197(v0: Bytes) -> Result<(Adt18, u64), Bytes> {
    Err::<(Adt18, u64), Bytes>(v0)
}

pub fn f198(v0: Bytes, v1: u64) -> R<u64> {
    let (h1375, _) = f188(Fn5::F279(v0.clone(), v1), {
        let a1377 = {
            let a1376 = v0;
            length_bytes(a1376)
        };
        let a1378 = 1u64;
        nat_add(a1377, a1378)?
    }, v1)?;
    Ok(h1375)
}

pub fn f199(v0: Bytes, v1: u64, v2: u64, v3: List<Bytes>) -> R<Result<(Adt18, u64), Bytes>> {
    let m1379 = v2;
    if m1379 == 0 {
        Ok(f197(f125(Str::lit("a tag has more attributes than the input has bytes"), v1)?))
    } else {
        let v4 = m1379 - 1;
        let v5: u64 = f193(v0.clone(), v1)?;
        let a1380 = f129(v0.clone(), v5)?;
        let a1381 = 62u64;
        if if nat_eq(a1380, a1381) {
            true
        } else {
            f128(v0.clone(), v5, Str::lit("/>"))
        } {
            Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C4, v5)))
        } else {
            let a1382 = v5;
            let a1383 = v1;
            if nat_eq(a1382, a1383) {
                Ok(f197(f125(Str::lit("attributes must be separated by white space (XML 1.0 [40])"), v1)?))
            } else {
                let v6: u64 = f198(v0.clone(), v5)?;
                let a1384 = v6;
                let a1385 = v5;
                if nat_eq(a1384, a1385) {
                    Ok(f197(f125(Str::lit("an attribute must start with a Name (XML 1.0 [41])"), v5)?))
                } else {
                    let v7: Bytes = f45(v0.clone(), v5, {
                        let a1386 = v6;
                        let a1387 = v5;
                        nat_sub(a1386, a1387)
                    });
                    if f278(v7.clone(), v3.clone()) {
                        Ok(f197(f125(Str::lit("an attribute name appears twice in one tag (XML 1.0 WFC: Unique Att Spec)"), v5)?))
                    } else {
                        let v8: u64 = f193(v0.clone(), v6)?;
                        let a1388 = f129(v0.clone(), v8)?;
                        let a1389 = 61u64;
                        if nat_eq(a1388, a1389) {
                            let v9: u64 = f193(v0.clone(), {
                                let a1390 = v8;
                                let a1391 = 1u64;
                                nat_add(a1390, a1391)?
                            })?;
                            let v10: u64 = f129(v0.clone(), v9)?;
                            let a1392 = v10;
                            let a1393 = 34u64;
                            if if nat_eq(a1392, a1393) {
                                true
                            } else {
                                let a1394 = v10;
                                let a1395 = 39u64;
                                nat_eq(a1394, a1395)
                            } {
                                let m1398 = f200(v0.clone(), {
                                    let a1396 = v9;
                                    let a1397 = 1u64;
                                    nat_add(a1396, a1397)?
                                }, v4, Bytes::lit(&[]), true, v10)?;
                                match m1398 {
                                    Ok(v12) => {
                                        let m1400 = f199(v0, {
                                            let (_, h1399) = v12;
                                            h1399
                                        }, v4, List::cons(v7.clone(), v3))?;
                                        match m1400 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C1(v7, {
                                                    let (h1401, _) = v12;
                                                    h1401
                                                }, std::rc::Rc::new({
                                                    let (h1402, _) = v14;
                                                    h1402
                                                })), {
                                                    let (_, h1403) = v14;
                                                    h1403
                                                })))
                                            }
                                            Err(v13) => {
                                                Ok(f197(v13))
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(f197(v11))
                                    }
                                }
                            } else {
                                Ok(f197(f125(Str::lit("an attribute value must be quoted (XML 1.0 [10])"), v9)?))
                            }
                        } else {
                            Ok(f197(f125(Str::lit("an attribute needs = and a value (XML 1.0 [25], [41])"), v8)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f200(v0: Bytes, p1443: u64, p1444: u64, p1445: Bytes, v4: bool, v5: u64) -> R<Result<(Bytes, u64), Bytes>> {
    let m1442 = (p1443, (p1444, p1445));
    {
        let mut m1442 = m1442;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1442;
                let m1404 = v2;
                if m1404 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f280(f125(Str::lit("the input ends inside a text run"), v1)?)))
                } else {
                    let v6 = m1404 - 1;
                    let v7: u64 = f129(v0.clone(), v1)?;
                    if if v4 {
                        let a1405 = v7;
                        let a1406 = v5;
                        nat_eq(a1405, a1406)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, {
                            let a1407 = v1;
                            let a1408 = 1u64;
                            nat_add(a1407, a1408)?
                        }))))
                    } else {
                        let a1409 = v7;
                        let a1410 = 256u64;
                        if nat_eq(a1409, a1410) {
                            if v4 {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f280(f125(Str::lit("an attribute value is not closed (XML 1.0 [10])"), v1)?)))
                            } else {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                            }
                        } else {
                            let a1411 = v7;
                            let a1412 = 60u64;
                            if nat_eq(a1411, a1412) {
                                if v4 {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f280(f125(Str::lit("< is not allowed in an attribute value (XML 1.0 [10], WFC: No < in Attribute Values)"), v1)?)))
                                } else {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                                }
                            } else {
                                let a1413 = v7;
                                let a1414 = 38u64;
                                if nat_eq(a1413, a1414) {
                                    let m1415 = f281(v0.clone(), v1)?;
                                    match m1415 {
                                        Ok(v9) => {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let (_, h1416) = v9;
                                                h1416
                                            }, (v6, {
                                                let a1418 = v3;
                                                let a1419 = {
                                                    let (h1417, _) = v9;
                                                    h1417
                                                };
                                                append_bytes(a1418, a1419)
                                            })))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f280(v8)))
                                        }
                                    }
                                } else {
                                    if v4 {
                                        let a1420 = v7;
                                        let a1421 = 9u64;
                                        if if nat_eq(a1420, a1421) {
                                            true
                                        } else {
                                            let a1422 = v7;
                                            let a1423 = 10u64;
                                            nat_eq(a1422, a1423)
                                        } {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let a1424 = v1;
                                                let a1425 = 1u64;
                                                nat_add(a1424, a1425)?
                                            }, (v6, {
                                                let a1426 = v3;
                                                let a1427 = f8(Str::lit(" "));
                                                append_bytes(a1426, a1427)
                                            })))
                                        } else {
                                            let v10: u64 = f282(v0.clone(), true, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v10, (v6, {
                                                let a1430 = v3;
                                                let a1431 = f45(v0.clone(), v1, {
                                                    let a1428 = v10;
                                                    let a1429 = v1;
                                                    nat_sub(a1428, a1429)
                                                });
                                                append_bytes(a1430, a1431)
                                            })))
                                        }
                                    } else {
                                        let a1432 = v7;
                                        let a1433 = 93u64;
                                        if nat_eq(a1432, a1433) {
                                            if f128(v0.clone(), v1, Str::lit("]]>")) {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f280(f125(Str::lit("]]> is not allowed in character data (XML 1.0 [14])"), v1)?)))
                                            } else {
                                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                    let a1434 = v1;
                                                    let a1435 = 1u64;
                                                    nat_add(a1434, a1435)?
                                                }, (v6, {
                                                    let a1436 = v3;
                                                    let a1437 = f8(Str::lit("]"));
                                                    append_bytes(a1436, a1437)
                                                })))
                                            }
                                        } else {
                                            let v11: u64 = f282(v0.clone(), false, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v11, (v6, {
                                                let a1440 = v3;
                                                let a1441 = f45(v0.clone(), v1, {
                                                    let a1438 = v11;
                                                    let a1439 = v1;
                                                    nat_sub(a1438, a1439)
                                                });
                                                append_bytes(a1440, a1441)
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
                Ok(next) => { m1442 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f201(v0: Bytes) -> Result<List<(Bytes, Bytes)>, Bytes> {
    Err::<List<(Bytes, Bytes)>, Bytes>(v0)
}

pub fn f202() -> Bytes {
    f8(Str::lit("http://www.w3.org/XML/1998/namespace"))
}

pub fn f203() -> Bytes {
    f8(Str::lit("http://www.w3.org/2000/xmlns/"))
}

pub fn f204(v0: Bytes, v1: Bytes) -> bool {
    let m1450 = {
        let a1447 = v0;
        let a1448 = 0u64;
        let a1449 = {
            let a1446 = v1.clone();
            length_bytes(a1446)
        };
        slice_bytes(a1447, a1448, a1449)
    };
    match m1450 {
        None => {
            false
        }
        Some(v2) => {
            let a1451 = v2;
            let a1452 = v1;
            equal(a1451, a1452)
        }
    }
}

pub fn f205(v0: Bytes) -> R<Bytes> {
    let m1453 = f191(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1453 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v1) => {
            Ok(f45(v0, 0u64, v1))
        }
    }
}

pub fn f206(p1458: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1457 = p1458;
    {
        let mut m1457 = m1457;
        loop {
            let step = {
                let v0 = m1457;
                let m1454 = v0;
                match m1454.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h1455, _) = v2;
                            h1455
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h1456) = v2;
                                h1456
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1457 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f207(v0: Bytes) -> R<Bytes> {
    let m1459 = f191(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1459 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(f45(v0.clone(), {
                let a1460 = v1;
                let a1461 = 1u64;
                nat_add(a1460, a1461)?
            }, {
                let a1465 = {
                    let a1462 = v0;
                    length_bytes(a1462)
                };
                let a1466 = {
                    let a1463 = v1;
                    let a1464 = 1u64;
                    nat_add(a1463, a1464)?
                };
                nat_sub(a1465, a1466)
            }))
        }
    }
}

pub fn f208() -> Bytes {
    f8(Str::lit("http://www.w3.org/2001/XMLSchema"))
}

pub fn f209(v0: Bytes) -> Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes> {
    Err::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v0)
}

pub fn f210(v0: Bytes, v1: Adt18, v2: List<(Bytes, Bytes)>, v3: Bytes) -> R<Result<Adt24, Bytes>> {
    let v4: Bytes = f211(v0.clone())?;
    let m1467 = f283(v1, v2)?;
    match m1467 {
        Ok(v6) => {
            let m1468 = v6;
            match m1468 {
                None => {
                    Ok(f284(f285(f8(Str::lit("a simpleType must have a restriction, list or union child (XSD 1.1 Part 2, section 4.1.2)")), v0)))
                }
                Some(v7) => {
                    let v8: Bytes = match v7.clone() {
                        Adt31::C0(h1469, _, _, _) => {
                            h1469
                        }
                    };
                    let v9: Adt18 = match v7.clone() {
                        Adt31::C0(_, h1470, _, _) => {
                            h1470
                        }
                    };
                    let v10: Adt18 = match v7.clone() {
                        Adt31::C0(_, _, h1471, _) => {
                            h1471
                        }
                    };
                    let v11: List<(Bytes, Bytes)> = match v7 {
                        Adt31::C0(_, _, _, h1472) => {
                            h1472
                        }
                    };
                    if f32(v8.clone(), f8(Str::lit("union"))) {
                        Ok(f284(f285(f8(Str::lit("a union is not translated, because a literal can be valid for more than one member type, so no LexLean inductive keeps its members disjoint (XSD 1.1 Part 2, section 2.4.1.3)")), v0)))
                    } else {
                        if f32(v8, f8(Str::lit("list"))) {
                            let m1473 = f135(v9, f8(Str::lit("itemType")));
                            match m1473 {
                                None => {
                                    Ok(f284(f285(f8(Str::lit("a list with an anonymous item type is not translated yet")), v0)))
                                }
                                Some(v12) => {
                                    let m1474 = f286(v11, v3, v12)?;
                                    match m1474 {
                                        Ok(v14) => {
                                            if f204(match v14.clone() {
                                                Adt24::C0(h1475, _) => {
                                                    h1475
                                                }
                                            }, f8(Str::lit("(List "))) {
                                                Ok(f284(f285(f8(Str::lit("the item type of a list must not itself be a list (XSD 1.1 Part 2, section 4.1.5)")), v0)))
                                            } else {
                                                Ok(Ok::<Adt24, Bytes>(Adt24::C0(f270(v4, f7(List::cons(f8(Str::lit("(List ")), List::cons(match v14.clone() {
                                                    Adt24::C0(h1476, _) => {
                                                        h1476
                                                    }
                                                }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))), match v14 {
                                                    Adt24::C0(_, h1477) => {
                                                        h1477
                                                    }
                                                })))
                                            }
                                        }
                                        Err(v13) => {
                                            Ok(f284(f285(v13, v0)))
                                        }
                                    }
                                }
                            }
                        } else {
                            let m1478 = f135(v9, f8(Str::lit("base")));
                            match m1478 {
                                None => {
                                    Ok(f284(f285(f8(Str::lit("a restriction with an anonymous base type is not translated yet")), v0)))
                                }
                                Some(v15) => {
                                    let m1479 = f286(v11.clone(), v3, v15)?;
                                    match m1479 {
                                        Ok(v17) => {
                                            let m1480 = f287(v10, v11)?;
                                            match m1480 {
                                                Ok(v19) => {
                                                    let v20: List<Adt2> = f288(v19, List::<Bytes>::nil())?;
                                                    let m1481 = v20.clone();
                                                    match m1481.uncons() {
                                                        None => {
                                                            Ok(Ok::<Adt24, Bytes>(Adt24::C0(f270(v4, match v17.clone() {
                                                                Adt24::C0(h1482, _) => {
                                                                    h1482
                                                                }
                                                            }), match v17 {
                                                                Adt24::C0(_, h1483) => {
                                                                    h1483
                                                                }
                                                            })))
                                                        }
                                                        Some((_, _)) => {
                                                            let m1484 = f262(f258(f289(v20.clone()))?, v20, List::<Bytes>::nil())?;
                                                            match m1484 {
                                                                Ok(v24) => {
                                                                    Ok(Ok::<Adt24, Bytes>(Adt24::C0(f264(v4, v24), List::<Bytes>::nil())))
                                                                }
                                                                Err(v23) => {
                                                                    Ok(f284(f285(v23, v0)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Err(v18) => {
                                                    Ok(f284(f285(v18, v0)))
                                                }
                                            }
                                        }
                                        Err(v16) => {
                                            Ok(f284(f285(v16, v0)))
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
            Ok(f284(f285(v5, v0)))
        }
    }
}

pub fn f211(v0: Bytes) -> R<Bytes> {
    f290(v0.clone(), 0u64, {
        let a1485 = v0;
        length_bytes(a1485)
    }, true, Bytes::lit(&[]))
}

pub fn f212(p1488: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m1487 = p1488;
    {
        let mut m1487 = m1487;
        loop {
            let step = {
                let v0 = m1487;
                let m1486 = v0;
                match m1486.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f173(v2.clone(), v1.clone()) {
                            Ok::<List<Bytes>, Option<Bytes>>(v3)
                        } else {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1487 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f213(v0: Bytes) -> Result<List<Bytes>, Bytes> {
    Err::<List<Bytes>, Bytes>(v0)
}

pub fn f214(v0: List<Bytes>, v1: Option<Bytes>) -> R<List<(Bytes, Bytes)>> {
    let m1489 = v0;
    match m1489.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, v3)) => {
            if f32(v2.clone(), f8(Str::lit("[[authority]]"))) {
                f214(v3, None::<Bytes>)
            } else {
                let m1490 = f291(v2.clone(), f8(Str::lit("url = ")))?;
                match m1490 {
                    None => {
                        let m1491 = f291(v2, f8(Str::lit("artifact = ")))?;
                        match m1491 {
                            None => {
                                f214(v3, v1)
                            }
                            Some(v5) => {
                                let m1492 = v1.clone();
                                match m1492 {
                                    None => {
                                        f214(v3, v1)
                                    }
                                    Some(v6) => {
                                        Ok(List::cons((v6, v5), f214(v3, v1)?))
                                    }
                                }
                            }
                        }
                    }
                    Some(v4) => {
                        f214(v3, Some(v4))
                    }
                }
            }
        }
    }
}

pub fn f215(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a1493 = v0.clone();
        length_bytes(a1493)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h1494, _) = f60(Fn0::F292(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h1494
    };
    let v3: u64 = {
        let (h1495, _) = {
            let (_, h1496) = v2.clone();
            h1496
        };
        h1495
    };
    let v4: List<Bytes> = {
        let (_, h1497) = {
            let (_, h1498) = v2;
            h1498
        };
        h1497
    };
    let a1499 = v3;
    let a1500 = v1;
    if nat_lt(a1499, a1500) {
        Ok(f62(List::cons(f45(v0, v3, {
            let a1501 = v1;
            let a1502 = v3;
            nat_sub(a1501, a1502)
        }), v4)))
    } else {
        Ok(f62(v4))
    }
}

pub fn f216(v0: List<Adt20>) -> R<Result<(u64, Bytes), Bytes>> {
    let m1503 = f293(v0.clone())?;
    match m1503 {
        Ok(v2) => {
            let v3: Adt25 = Adt25::C0(v2.clone(), f294(v0.clone()));
            let m1504 = f295(f296(v0)?, v3, v2)?;
            match m1504 {
                Ok(v5) => {
                    let m1505 = f297(f298(v5), List::<Adt28>::nil());
                    match m1505 {
                        Ok(v7) => {
                            let v8: List<Bytes> = f299(v7.clone());
                            let m1506 = f300(v7.clone(), v8.clone());
                            match m1506 {
                                None => {
                                    let v10: List<(Bytes, List<Bytes>)> = f301(v7.clone(), v8.clone(), f302(v7.clone())?);
                                    let m1512 = f303({
                                        let a1510 = {
                                            let a1509 = v8.clone();
                                            length_list(a1509)
                                        };
                                        let a1511 = 1u64;
                                        nat_add(a1510, a1511)?
                                    }, v7.clone(), f304(v7, f305(v10, v8.clone(), v8, List::<Bytes>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil());
                                    match m1512 {
                                        Ok(v12) => {
                                            Ok(Ok::<(u64, Bytes), Bytes>(({
                                                let a1513 = v12.clone();
                                                length_list(a1513)
                                            }, f117(v12, f8(Str::lit("\u{a}"))))))
                                        }
                                        Err(v11) => {
                                            Ok(Err::<(u64, Bytes), Bytes>(v11))
                                        }
                                    }
                                }
                                Some(v9) => {
                                    Ok(Err::<(u64, Bytes), Bytes>(f220(f8(Str::lit("the type ")), {
                                        let (h1507, _) = v9;
                                        h1507
                                    }, f220(f8(Str::lit(" uses ")), {
                                        let (_, h1508) = v9;
                                        h1508
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

pub fn f217(p1516: List<Adt20>, p1517: List<Adt20>) -> List<Adt20> {
    let m1515 = (p1516, p1517);
    {
        let mut m1515 = m1515;
        loop {
            let step = {
                let (v0, v1) = m1515;
                let m1514 = v0;
                match m1514.uncons() {
                    None => {
                        Err::<(List<Adt20>, List<Adt20>), List<Adt20>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt20>, List<Adt20>), List<Adt20>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1515 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f218(v0: List<Adt20>) -> List<Bytes> {
    let m1518 = v0;
    match m1518.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt20::C0(h1519, _, _, _, _, _) => {
                    h1519
                }
            }, f218(v2))
        }
    }
}

pub fn f219(p1524: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m1523 = p1524;
    {
        let mut m1523 = m1523;
        loop {
            let step = {
                let v0 = m1523;
                let m1520 = v0;
                match m1520.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h1521, _) = v2;
                            h1521
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h1522) = v2;
                                h1522
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1523 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f220(v0: Bytes, v1: Bytes, v2: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(v1, List::cons(v2, List::<Bytes>::nil()))))
}

pub fn f221(v0: Bytes, v1: Bytes, v2: Option<Bytes>) -> R<Result<Adt20, Bytes>> {
    let m1525 = f77(v1.clone())?;
    match m1525 {
        Ok(v4) => {
            let m1526 = v4;
            match m1526 {
                Adt18::C0(v5, r1531, r1532) => {
                    let v6 = (*r1531).clone();
                    let v7 = (*r1532).clone();
                    let m1527 = f132(v6.clone(), List::<(Bytes, Bytes)>::nil());
                    match m1527 {
                        Ok(v9) => {
                            let m1528 = f133(v9.clone(), v5)?;
                            match m1528 {
                                Ok(v11) => {
                                    if f134(v11, Str::lit("schema")) {
                                        Ok(Ok::<Adt20, Bytes>(Adt20::C0(v0, v2, {
                                            let m1529 = f135(v6, f8(Str::lit("targetNamespace")));
                                            match m1529 {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some(v12) => {
                                                    v12
                                                }
                                            }
                                        }, v9, v7, {
                                            let a1530 = v1;
                                            length_bytes(a1530)
                                        })))
                                    } else {
                                        Ok(Err::<Adt20, Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(": the root element must be xs:schema in the XML Schema namespace (XSD 1.1 Part 1, section 3.17.2)")), List::<Bytes>::nil())))))
                                    }
                                }
                                Err(v10) => {
                                    Ok(Err::<Adt20, Bytes>(v10))
                                }
                            }
                        }
                        Err(v8) => {
                            Ok(Err::<Adt20, Bytes>(v8))
                        }
                    }
                }
                Adt18::C1(_, _, _) => {
                    Ok(Err::<Adt20, Bytes>(f8(Str::lit("the document has no root element"))))
                }
                Adt18::C2(_) => {
                    Ok(Err::<Adt20, Bytes>(f8(Str::lit("the document has no root element"))))
                }
                Adt18::C3(_, _) => {
                    Ok(Err::<Adt20, Bytes>(f8(Str::lit("the document has no root element"))))
                }
                Adt18::C4 => {
                    Ok(Err::<Adt20, Bytes>(f8(Str::lit("the document has no root element"))))
                }
            }
        }
        Err(v3) => {
            Ok(Err::<Adt20, Bytes>(f220(v0, f8(Str::lit(" is not well-formed XML: ")), v3)))
        }
    }
}

pub fn f222(p1537: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1536 = p1537;
    {
        let mut m1536 = m1536;
        loop {
            let step = {
                let v0 = m1536;
                let m1533 = v0;
                match m1533.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (_, h1534) = v2;
                            h1534
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (h1535, _) = v2;
                                h1535
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
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

pub fn f223(v0: Adt20, v1: Bytes, v2: Bytes) -> Option<Bytes> {
    if f32(v1.clone(), f8(Str::lit("root"))) {
        None::<Bytes>
    } else {
        if f32(match v0.clone() {
            Adt20::C0(_, _, h1538, _, _, _) => {
                h1538
            }
        }, v2.clone()) {
            None::<Bytes>
        } else {
            Some(f7(List::cons(match v0.clone() {
                Adt20::C0(h1539, _, _, _, _, _) => {
                    h1539
                }
            }, List::cons(f8(Str::lit(" has the target namespace ")), List::cons(match v0 {
                Adt20::C0(_, _, h1540, _, _, _) => {
                    h1540
                }
            }, List::cons(f8(Str::lit(", but the xs:")), List::cons(v1.clone(), List::cons(f8(Str::lit(" that names it requires ")), List::cons(v2, List::cons(if f32(v1, f8(Str::lit("include"))) {
                f8(Str::lit("; an included document without a target namespace is not adopted into the including one (XSD 1.1 Part 1, section 4.2.3)"))
            } else {
                f8(Str::lit(" (XSD 1.1 Part 1, section 4.2.6.2)"))
            }, List::<Bytes>::nil()))))))))))
        }
    }
}

pub fn f224(v0: Adt18, v1: List<(Bytes, Bytes)>) -> R<Result<List<Adt27>, Bytes>> {
    let m1541 = v0;
    match m1541 {
        Adt18::C0(_, _, _) => {
            Ok(Ok::<List<Adt27>, Bytes>(List::<Adt27>::nil()))
        }
        Adt18::C1(_, _, _) => {
            Ok(Ok::<List<Adt27>, Bytes>(List::<Adt27>::nil()))
        }
        Adt18::C2(_) => {
            Ok(Ok::<List<Adt27>, Bytes>(List::<Adt27>::nil()))
        }
        Adt18::C3(r1552, r1553) => {
            let v2 = (*r1552).clone();
            let v3 = (*r1553).clone();
            let m1542 = f224(v3, v1.clone())?;
            match m1542 {
                Ok(v5) => {
                    let m1543 = v2;
                    match m1543 {
                        Adt18::C0(v6, r1550, r1551) => {
                            let v7 = (*r1550).clone();
                            let v8 = (*r1551).clone();
                            let m1544 = f132(v7.clone(), v1);
                            match m1544 {
                                Ok(v10) => {
                                    let m1545 = f133(v10.clone(), v6)?;
                                    match m1545 {
                                        Ok(v12) => {
                                            if if f32({
                                                let (h1546, _) = v12.clone();
                                                h1546
                                            }, f208()) {
                                                f306(f32({
                                                    let (_, h1547) = v12.clone();
                                                    h1547
                                                }, f8(Str::lit("annotation"))))
                                            } else {
                                                false
                                            } {
                                                Ok(Ok::<List<Adt27>, Bytes>(List::cons(Adt27::C0({
                                                    let (h1548, _) = v12;
                                                    h1548
                                                }, {
                                                    let (_, h1549) = v12;
                                                    h1549
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
                        Adt18::C1(_, _, _) => {
                            Ok(Ok::<List<Adt27>, Bytes>(v5))
                        }
                        Adt18::C2(_) => {
                            Ok(Ok::<List<Adt27>, Bytes>(v5))
                        }
                        Adt18::C3(_, _) => {
                            Ok(Ok::<List<Adt27>, Bytes>(v5))
                        }
                        Adt18::C4 => {
                            Ok(Ok::<List<Adt27>, Bytes>(v5))
                        }
                    }
                }
                Err(v4) => {
                    Ok(Err::<List<Adt27>, Bytes>(v4))
                }
            }
        }
        Adt18::C4 => {
            Ok(Ok::<List<Adt27>, Bytes>(List::<Adt27>::nil()))
        }
    }
}

pub fn f225(v0: List<Adt27>, v1: Adt20, v2: List<(Bytes, Bytes)>) -> R<Result<List<(Bytes, (Bytes, Bytes))>, Bytes>> {
    let m1554 = v0;
    match m1554.uncons() {
        None => {
            Ok(Ok::<List<(Bytes, (Bytes, Bytes))>, Bytes>(List::<(Bytes, (Bytes, Bytes))>::nil()))
        }
        Some((v3, v4)) => {
            let m1555 = f225(v4, v1.clone(), v2.clone())?;
            match m1555 {
                Ok(v6) => {
                    if if f307(v3.clone(), Str::lit("redefine")) {
                        true
                    } else {
                        if f307(v3.clone(), Str::lit("override")) {
                            true
                        } else {
                            f307(v3.clone(), Str::lit("defaultOpenContent"))
                        }
                    } {
                        Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                            Adt20::C0(h1556, _, _, _, _, _) => {
                                h1556
                            }
                        }, List::cons(f8(Str::lit(": xs:")), List::cons(match v3 {
                            Adt27::C0(_, h1557, _, _, _) => {
                                h1557
                            }
                        }, List::cons(f8(Str::lit(" is not translated, because it changes components that other schema documents define (XSD 1.1 Part 1, sections 4.2.4, 4.2.5 and 3.4.2.3)")), List::<Bytes>::nil())))))))
                    } else {
                        if if f307(v3.clone(), Str::lit("import")) {
                            true
                        } else {
                            f307(v3.clone(), Str::lit("include"))
                        } {
                            let m1558 = f308(v3.clone(), Str::lit("schemaLocation"));
                            match m1558 {
                                None => {
                                    Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                                        Adt20::C0(h1559, _, _, _, _, _) => {
                                            h1559
                                        }
                                    }, List::cons(f8(Str::lit(": an xs:")), List::cons(match v3 {
                                        Adt27::C0(_, h1560, _, _, _) => {
                                            h1560
                                        }
                                    }, List::cons(f8(Str::lit(" with no schemaLocation names no document whose components could be read (XSD 1.1 Part 1, section 4.2.6.2)")), List::<Bytes>::nil())))))))
                                }
                                Some(v7) => {
                                    let m1562 = f309(match v1.clone() {
                                        Adt20::C0(_, h1561, _, _, _, _) => {
                                            h1561
                                        }
                                    }, v7)?;
                                    match m1562 {
                                        Ok(v9) => {
                                            let m1564 = f310(v2, v9.clone());
                                            match m1564 {
                                                None => {
                                                    Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                                                        Adt20::C0(h1565, _, _, _, _, _) => {
                                                            h1565
                                                        }
                                                    }, List::cons(f8(Str::lit(": the schema document ")), List::cons(v9, List::cons(f8(Str::lit(" is not a bound authority, so its components cannot be read; stage its binding first (XSD 1.1 Part 1, section 4.2.6.2)")), List::<Bytes>::nil())))))))
                                                }
                                                Some(v10) => {
                                                    Ok(Ok::<List<(Bytes, (Bytes, Bytes))>, Bytes>(List::cons((v10, (match v3.clone() {
                                                        Adt27::C0(_, h1566, _, _, _) => {
                                                            h1566
                                                        }
                                                    }, if f307(v3.clone(), Str::lit("include")) {
                                                        match v1 {
                                                            Adt20::C0(_, _, h1567, _, _, _) => {
                                                                h1567
                                                            }
                                                        }
                                                    } else {
                                                        f311(v3, Str::lit("namespace"), Str::lit(""))
                                                    })), v6)))
                                                }
                                            }
                                        }
                                        Err(v8) => {
                                            Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f220(match v1 {
                                                Adt20::C0(h1563, _, _, _, _, _) => {
                                                    h1563
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

pub fn f226(v0: List<(Bytes, (Bytes, Bytes))>, v1: List<(Bytes, (Bytes, Bytes))>) -> List<(Bytes, (Bytes, Bytes))> {
    let m1568 = v0;
    match m1568.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f226(v3, v1))
        }
    }
}

pub fn f227(v0: List<(Bytes, Option<Bytes>)>, v1: List<(Bytes, Option<Bytes>)>) -> List<(Bytes, Option<Bytes>)> {
    let m1569 = v0;
    match m1569.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f227(v3, v1))
        }
    }
}

pub fn f228(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f312(v0, v1)
}

pub fn f229(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f313(v0, v1)
}

pub fn f230(v0: Bytes, v1: u64) -> R<bool> {
    if if if f128(v0.clone(), v1, Str::lit("---")) {
        true
    } else {
        f128(v0.clone(), v1, Str::lit("..."))
    } {
        if f314(f232(v0.clone(), {
            let a1570 = v1;
            let a1571 = 3u64;
            nat_add(a1570, a1571)?
        })?) {
            true
        } else {
            f315(f232(v0.clone(), {
                let a1572 = v1;
                let a1573 = 3u64;
                nat_add(a1572, a1573)?
            })?)
        }
    } else {
        false
    } {
        let a1574 = f316(v0, v1)?;
        let a1575 = 0u64;
        Ok(nat_eq(a1574, a1575))
    } else {
        Ok(false)
    }
}

pub fn f231(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f317(v0.clone(), v1)?;
    let v3: u64 = f232(v0.clone(), v2)?;
    if if f315(v3) {
        true
    } else {
        let a1576 = v3;
        let a1577 = 35u64;
        if nat_eq(a1576, a1577) {
            let a1578 = v1;
            let a1579 = v2;
            nat_lt(a1578, a1579)
        } else {
            false
        }
    } {
        Ok(Ok::<u64, Bytes>(f318(v0, v2)?))
    } else {
        Ok(Err::<u64, Bytes>(f125(Str::lit("only white space and a comment may follow a node on its line (YAML 1.2.2 [77])"), v2)?))
    }
}

pub fn f232(v0: Bytes, v1: u64) -> R<u64> {
    f94(v0, v1)
}

pub fn f233(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: List<Bytes>) -> R<Adt22> {
    let m1580 = v3;
    if m1580 == 0 {
        Ok(Adt22::C1(f125(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v8 = m1580 - 1;
        let a1581 = v4;
        let a1582 = 0u64;
        if nat_eq(a1581, a1582) {
            let v9: u64 = f319(v0.clone(), v1)?;
            let v10: u64 = {
                let a1583 = v9;
                let a1584 = v1;
                nat_sub(a1583, a1584)
            };
            let a1585 = f232(v0.clone(), v9)?;
            let a1586 = 9u64;
            if nat_eq(a1585, a1586) {
                let v11: u64 = f317(v0.clone(), v9)?;
                if if f320(v0.clone(), v11)? {
                    true
                } else {
                    f321(v0.clone(), v11)?
                } {
                    Ok(Adt22::C1(f125(Str::lit("a tab cannot indent a block node (YAML 1.2.2 [63])"), v11)?))
                } else {
                    let m1587 = f322(v0.clone(), v11)?;
                    match m1587 {
                        None => {
                            f323(v0, v11, v2)
                        }
                        Some(v12) => {
                            Ok(Adt22::C1(v12))
                        }
                    }
                }
            } else {
                if f320(v0.clone(), v9)? {
                    Ok(f324(true, f233(v0, v9, v2, v8, 2u64, false, v10, List::<Bytes>::nil())?))
                } else {
                    let m1588 = f322(v0.clone(), v9)?;
                    match m1588 {
                        None => {
                            if f321(v0.clone(), v9)? {
                                Ok(f324(false, f233(v0, v9, v2, v8, 3u64, false, v10, List::<Bytes>::nil())?))
                            } else {
                                f323(v0, v9, v2)
                            }
                        }
                        Some(v13) => {
                            Ok(Adt22::C1(v13))
                        }
                    }
                }
            }
        } else {
            let a1589 = v4;
            let a1590 = 1u64;
            if nat_eq(a1589, a1590) {
                let v14: u64 = f317(v0.clone(), v1)?;
                let v15: u64 = f232(v0.clone(), v14)?;
                if if f315(v15) {
                    true
                } else {
                    let a1591 = v15;
                    let a1592 = 35u64;
                    if nat_eq(a1591, a1592) {
                        let a1593 = v1;
                        let a1594 = v14;
                        nat_lt(a1593, a1594)
                    } else {
                        false
                    }
                } {
                    let v16: u64 = f145(v0.clone(), f318(v0.clone(), v14)?)?;
                    let a1596 = {
                        let a1595 = v0.clone();
                        length_bytes(a1595)
                    };
                    let a1597 = v16;
                    if if nat_le(a1596, a1597) {
                        true
                    } else {
                        f230(v0.clone(), v16)?
                    } {
                        Ok(Adt22::C0(Adt2::C0, v16))
                    } else {
                        let v17: u64 = {
                            let a1598 = f319(v0.clone(), v16)?;
                            let a1599 = v16;
                            nat_sub(a1598, a1599)
                        };
                        let a1600 = v2;
                        let a1601 = v17;
                        if nat_le(a1600, a1601) {
                            f233(v0, v16, v2, v8, 0u64, false, 0u64, List::<Bytes>::nil())
                        } else {
                            if if if f234(v5) {
                                let a1602 = v17;
                                let a1603 = v6;
                                nat_eq(a1602, a1603)
                            } else {
                                false
                            } {
                                f320(v0.clone(), {
                                    let a1604 = v16;
                                    let a1605 = v17;
                                    nat_add(a1604, a1605)?
                                })?
                            } else {
                                false
                            } {
                                Ok(f324(true, f233(v0, {
                                    let a1606 = v16;
                                    let a1607 = v17;
                                    nat_add(a1606, a1607)?
                                }, v2, v8, 2u64, false, v17, List::<Bytes>::nil())?))
                            } else {
                                Ok(Adt22::C0(Adt2::C0, v16))
                            }
                        }
                    }
                } else {
                    if if v5 {
                        f320(v0.clone(), v14)?
                    } else {
                        false
                    } {
                        let a1610 = f325(f326(v0.clone(), v1, v14), 9u64)?;
                        let a1611 = {
                            let a1608 = v14;
                            let a1609 = v1;
                            nat_sub(a1608, a1609)
                        };
                        if nat_lt(a1610, a1611) {
                            Ok(Adt22::C1(f125(Str::lit("a tab cannot indent a block node (YAML 1.2.2 [63])"), v14)?))
                        } else {
                            Ok(f324(true, f233(v0.clone(), v14, v2, v8, 2u64, false, f316(v0, v14)?, List::<Bytes>::nil())?))
                        }
                    } else {
                        let m1612 = f322(v0.clone(), v14)?;
                        match m1612 {
                            None => {
                                if if v5 {
                                    f321(v0.clone(), v14)?
                                } else {
                                    false
                                } {
                                    Ok(f324(false, f233(v0.clone(), v14, v2, v8, 3u64, false, f316(v0, v14)?, List::<Bytes>::nil())?))
                                } else {
                                    f323(v0, v14, v2)
                                }
                            }
                            Some(v18) => {
                                Ok(Adt22::C1(v18))
                            }
                        }
                    }
                }
            } else {
                let a1613 = v4;
                let a1614 = 2u64;
                if nat_eq(a1613, a1614) {
                    let m1619 = f233(v0.clone(), {
                        let a1615 = v1;
                        let a1616 = 1u64;
                        nat_add(a1615, a1616)?
                    }, {
                        let a1617 = v6;
                        let a1618 = 1u64;
                        nat_add(a1617, a1618)?
                    }, v8, 1u64, true, v6, List::<Bytes>::nil())?;
                    match m1619 {
                        Adt22::C0(v20, v21) => {
                            let v22: u64 = f145(v0.clone(), v21)?;
                            let v23: u64 = {
                                let a1620 = f319(v0.clone(), v22)?;
                                let a1621 = v22;
                                nat_sub(a1620, a1621)
                            };
                            let a1623 = {
                                let a1622 = v0.clone();
                                length_bytes(a1622)
                            };
                            let a1624 = v22;
                            if nat_le(a1623, a1624) {
                                Ok(Adt22::C0(Adt2::C9(std::rc::Rc::new(v20), std::rc::Rc::new(Adt2::C8)), v22))
                            } else {
                                let a1625 = v23;
                                let a1626 = v6;
                                if if nat_eq(a1625, a1626) {
                                    f320(v0.clone(), {
                                        let a1627 = v22;
                                        let a1628 = v23;
                                        nat_add(a1627, a1628)?
                                    })?
                                } else {
                                    false
                                } {
                                    let m1631 = f233(v0, {
                                        let a1629 = v22;
                                        let a1630 = v23;
                                        nat_add(a1629, a1630)?
                                    }, v2, v8, 2u64, false, v6, List::<Bytes>::nil())?;
                                    match m1631 {
                                        Adt22::C0(v25, v26) => {
                                            Ok(Adt22::C0(Adt2::C9(std::rc::Rc::new(v20), std::rc::Rc::new(v25)), v26))
                                        }
                                        Adt22::C1(v24) => {
                                            Ok(Adt22::C1(v24))
                                        }
                                    }
                                } else {
                                    let a1632 = v6;
                                    let a1633 = v23;
                                    if nat_lt(a1632, a1633) {
                                        Ok(Adt22::C1(f125(Str::lit("a line is indented more than its block sequence entry allows (YAML 1.2.2 [183])"), {
                                            let a1634 = v22;
                                            let a1635 = v23;
                                            nat_add(a1634, a1635)?
                                        })?))
                                    } else {
                                        Ok(Adt22::C0(Adt2::C9(std::rc::Rc::new(v20), std::rc::Rc::new(Adt2::C8)), v22))
                                    }
                                }
                            }
                        }
                        Adt22::C1(v19) => {
                            Ok(Adt22::C1(v19))
                        }
                    }
                } else {
                    let m1636 = f322(v0.clone(), v1)?;
                    match m1636 {
                        None => {
                            let m1637 = f327(v0.clone(), v1)?;
                            match m1637 {
                                None => {
                                    Ok(Adt22::C1(f125(Str::lit("a block mapping entry must be an implicit key and : (YAML 1.2.2 [188])"), v1)?))
                                }
                                Some(v28) => {
                                    let v29: Bytes = f328(v0.clone(), v1)?;
                                    if f278(v29.clone(), v7.clone()) {
                                        Ok(Adt22::C1(f125(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                    } else {
                                        let m1642 = f233(v0.clone(), {
                                            let a1638 = v28;
                                            let a1639 = 1u64;
                                            nat_add(a1638, a1639)?
                                        }, {
                                            let a1640 = v6;
                                            let a1641 = 1u64;
                                            nat_add(a1640, a1641)?
                                        }, v8, 1u64, false, v6, List::<Bytes>::nil())?;
                                        match m1642 {
                                            Adt22::C0(v31, v32) => {
                                                let v33: u64 = f145(v0.clone(), v32)?;
                                                let v34: u64 = {
                                                    let a1643 = f319(v0.clone(), v33)?;
                                                    let a1644 = v33;
                                                    nat_sub(a1643, a1644)
                                                };
                                                let a1646 = {
                                                    let a1645 = v0.clone();
                                                    length_bytes(a1645)
                                                };
                                                let a1647 = v33;
                                                if nat_le(a1646, a1647) {
                                                    Ok(Adt22::C0(Adt2::C10(v29, std::rc::Rc::new(v31), std::rc::Rc::new(Adt2::C8)), v33))
                                                } else {
                                                    let a1648 = v34;
                                                    let a1649 = v6;
                                                    if if if nat_eq(a1648, a1649) {
                                                        f234(f320(v0.clone(), {
                                                            let a1650 = v33;
                                                            let a1651 = v34;
                                                            nat_add(a1650, a1651)?
                                                        })?)
                                                    } else {
                                                        false
                                                    } {
                                                        f234(f230(v0.clone(), v33)?)
                                                    } else {
                                                        false
                                                    } {
                                                        let m1654 = f233(v0, {
                                                            let a1652 = v33;
                                                            let a1653 = v34;
                                                            nat_add(a1652, a1653)?
                                                        }, v2, v8, 3u64, false, v6, List::cons(v29.clone(), v7))?;
                                                        match m1654 {
                                                            Adt22::C0(v36, v37) => {
                                                                Ok(Adt22::C0(Adt2::C10(v29, std::rc::Rc::new(v31), std::rc::Rc::new(v36)), v37))
                                                            }
                                                            Adt22::C1(v35) => {
                                                                Ok(Adt22::C1(v35))
                                                            }
                                                        }
                                                    } else {
                                                        let a1655 = v6;
                                                        let a1656 = v34;
                                                        if nat_lt(a1655, a1656) {
                                                            Ok(Adt22::C1(f125(Str::lit("a line is indented more than its block mapping entry allows (YAML 1.2.2 [187])"), {
                                                                let a1657 = v33;
                                                                let a1658 = v34;
                                                                nat_add(a1657, a1658)?
                                                            })?))
                                                        } else {
                                                            Ok(Adt22::C0(Adt2::C10(v29, std::rc::Rc::new(v31), std::rc::Rc::new(Adt2::C8)), v33))
                                                        }
                                                    }
                                                }
                                            }
                                            Adt22::C1(v30) => {
                                                Ok(Adt22::C1(v30))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v27) => {
                            Ok(Adt22::C1(v27))
                        }
                    }
                }
            }
        }
    }
}

pub fn f234(v0: bool) -> bool {
    !v0
}

pub fn f235(v0: Adt2) -> R<Bytes> {
    let m1659 = v0;
    match m1659 {
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
            f329(v4, v5)
        }
        Adt2::C4(v6, v7, v8) => {
            f330(v6, v7, v8)
        }
        Adt2::C5(v9) => {
            f331(v9)
        }
        Adt2::C6(r1660) => {
            let v10 = (*r1660).clone();
            Ok(f332(f333(v10)?))
        }
        Adt2::C7(r1661) => {
            let v11 = (*r1661).clone();
            f334(f335(v11)?)
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

pub fn f236(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f94(v0.clone(), {
        let a1662 = v1;
        let a1663 = 1u64;
        nat_add(a1662, a1663)?
    })?;
    let v4: Bytes = {
        let a1664 = v3;
        let a1665 = 117u64;
        if nat_eq(a1664, a1665) {
            f336(v0, v1)?
        } else {
            f337(v3)
        }
    };
    let a1666 = v4.clone();
    let a1667 = Bytes::lit(&[]);
    if equal(a1666, a1667) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a1668 = v3;
            let a1669 = 117u64;
            if nat_eq(a1668, a1669) {
                let a1670 = v1;
                let a1671 = 6u64;
                nat_add(a1670, a1671)?
            } else {
                let a1672 = v1;
                let a1673 = 2u64;
                nat_add(a1672, a1673)?
            }
        }, ({
            let a1674 = v2;
            let a1675 = v4;
            append_bytes(a1674, a1675)
        }, 0u64))))
    }
}

pub fn f237(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f338(v0, v1)
}

pub fn f238(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f339(v0, v1)
}

pub fn f239(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1676 = v0.clone();
        length_bytes(a1676)
    };
    let a1677 = 0u64;
    let a1678 = v1;
    if nat_lt(a1677, a1678) {
        let a1679 = f340(v0.clone())?;
        let a1680 = v1;
        if nat_eq(a1679, a1680) {
            let a1681 = 1u64;
            let a1682 = v1;
            if nat_lt(a1681, a1682) {
                let m1685 = {
                    let a1683 = v0;
                    let a1684 = 0u64;
                    index_bytes(a1683, a1684)
                };
                match m1685 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1686 = v2;
                        let a1687 = 48u8;
                        Ok(!equal(a1686, a1687))
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

pub fn f240(v0: Fn6, p1692: u64, p1693: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1691 = (p1692, p1693);
    {
        let mut m1691 = m1691;
        loop {
            let step = {
                let (v1, v2) = m1691;
                let m1688 = v1;
                if m1688 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1688 - 1;
                    let m1690 = {
                        let c1689: Fn6 = v0.clone();
                        c1689.apply(v2)?
                    };
                    match m1690 {
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
                Ok(next) => { m1691 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f241(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f341(v0, v1)
}

pub fn f242(v0: Bytes) -> R<u64> {
    let (h1694, _) = f188(Fn5::F342(v0.clone()), {
        let a1696 = {
            let a1695 = v0;
            length_bytes(a1695)
        };
        let a1697 = 1u64;
        nat_add(a1696, a1697)?
    }, 0u64)?;
    Ok(h1694)
}

pub fn f243(v0: u64) -> Bytes {
    let m1698 = v0;
    if m1698 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1698 - 1;
        let a1699 = f8(Str::lit("0"));
        let a1700 = f243(v1);
        append_bytes(a1699, a1700)
    }
}

pub fn f244(v0: u32) -> Bytes {
    let a1703 = {
        let a1701 = v0;
        let a1702 = 8u32;
        fixed_u32::bit_and(a1701, a1702)
    };
    let a1704 = 0u32;
    if equal(a1703, a1704) {
        let a1707 = {
            let a1705 = v0;
            let a1706 = 4u32;
            fixed_u32::bit_and(a1705, a1706)
        };
        let a1708 = 0u32;
        if equal(a1707, a1708) {
            let a1711 = {
                let a1709 = v0;
                let a1710 = 2u32;
                fixed_u32::bit_and(a1709, a1710)
            };
            let a1712 = 0u32;
            if equal(a1711, a1712) {
                let a1715 = {
                    let a1713 = v0;
                    let a1714 = 1u32;
                    fixed_u32::bit_and(a1713, a1714)
                };
                let a1716 = 0u32;
                if equal(a1715, a1716) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a1719 = {
                    let a1717 = v0;
                    let a1718 = 1u32;
                    fixed_u32::bit_and(a1717, a1718)
                };
                let a1720 = 0u32;
                if equal(a1719, a1720) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a1723 = {
                let a1721 = v0;
                let a1722 = 2u32;
                fixed_u32::bit_and(a1721, a1722)
            };
            let a1724 = 0u32;
            if equal(a1723, a1724) {
                let a1727 = {
                    let a1725 = v0;
                    let a1726 = 1u32;
                    fixed_u32::bit_and(a1725, a1726)
                };
                let a1728 = 0u32;
                if equal(a1727, a1728) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a1731 = {
                    let a1729 = v0;
                    let a1730 = 1u32;
                    fixed_u32::bit_and(a1729, a1730)
                };
                let a1732 = 0u32;
                if equal(a1731, a1732) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a1735 = {
            let a1733 = v0;
            let a1734 = 4u32;
            fixed_u32::bit_and(a1733, a1734)
        };
        let a1736 = 0u32;
        if equal(a1735, a1736) {
            let a1739 = {
                let a1737 = v0;
                let a1738 = 2u32;
                fixed_u32::bit_and(a1737, a1738)
            };
            let a1740 = 0u32;
            if equal(a1739, a1740) {
                let a1743 = {
                    let a1741 = v0;
                    let a1742 = 1u32;
                    fixed_u32::bit_and(a1741, a1742)
                };
                let a1744 = 0u32;
                if equal(a1743, a1744) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a1747 = {
                    let a1745 = v0;
                    let a1746 = 1u32;
                    fixed_u32::bit_and(a1745, a1746)
                };
                let a1748 = 0u32;
                if equal(a1747, a1748) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a1751 = {
                let a1749 = v0;
                let a1750 = 2u32;
                fixed_u32::bit_and(a1749, a1750)
            };
            let a1752 = 0u32;
            if equal(a1751, a1752) {
                let a1755 = {
                    let a1753 = v0;
                    let a1754 = 1u32;
                    fixed_u32::bit_and(a1753, a1754)
                };
                let a1756 = 0u32;
                if equal(a1755, a1756) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a1759 = {
                    let a1757 = v0;
                    let a1758 = 1u32;
                    fixed_u32::bit_and(a1757, a1758)
                };
                let a1760 = 0u32;
                if equal(a1759, a1760) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f245(v0: u32, v1: u32) -> u32 {
    let m1763 = {
        let a1761 = v0;
        let a1762 = v1;
        fixed_u32::shift_right(a1761, a1762)
    };
    match m1763 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f246(v0: Adt21, v1: Adt29) -> R<Adt21> {
    let v2: Adt30 = f343(Fn7::F344, 64u64, Adt30::C0(match v0.clone() {
        Adt21::C0(h1764, _, _, _, _, _, _, _) => {
            h1764
        }
    }, match v0.clone() {
        Adt21::C0(_, h1765, _, _, _, _, _, _) => {
            h1765
        }
    }, match v0.clone() {
        Adt21::C0(_, _, h1766, _, _, _, _, _) => {
            h1766
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, h1767, _, _, _, _) => {
            h1767
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, h1768, _, _, _) => {
            h1768
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, h1769, _, _) => {
            h1769
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, h1770, _) => {
            h1770
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, _, h1771) => {
            h1771
        }
    }, v1, 0u64))?;
    Ok(Adt21::C0(f345(match v0.clone() {
        Adt21::C0(h1772, _, _, _, _, _, _, _) => {
            h1772
        }
    }, match v2.clone() {
        Adt30::C0(h1773, _, _, _, _, _, _, _, _, _) => {
            h1773
        }
    }), f345(match v0.clone() {
        Adt21::C0(_, h1774, _, _, _, _, _, _) => {
            h1774
        }
    }, match v2.clone() {
        Adt30::C0(_, h1775, _, _, _, _, _, _, _, _) => {
            h1775
        }
    }), f345(match v0.clone() {
        Adt21::C0(_, _, h1776, _, _, _, _, _) => {
            h1776
        }
    }, match v2.clone() {
        Adt30::C0(_, _, h1777, _, _, _, _, _, _, _) => {
            h1777
        }
    }), f345(match v0.clone() {
        Adt21::C0(_, _, _, h1778, _, _, _, _) => {
            h1778
        }
    }, match v2.clone() {
        Adt30::C0(_, _, _, h1779, _, _, _, _, _, _) => {
            h1779
        }
    }), f345(match v0.clone() {
        Adt21::C0(_, _, _, _, h1780, _, _, _) => {
            h1780
        }
    }, match v2.clone() {
        Adt30::C0(_, _, _, _, h1781, _, _, _, _, _) => {
            h1781
        }
    }), f345(match v0.clone() {
        Adt21::C0(_, _, _, _, _, h1782, _, _) => {
            h1782
        }
    }, match v2.clone() {
        Adt30::C0(_, _, _, _, _, h1783, _, _, _, _) => {
            h1783
        }
    }), f345(match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, h1784, _) => {
            h1784
        }
    }, match v2.clone() {
        Adt30::C0(_, _, _, _, _, _, h1785, _, _, _) => {
            h1785
        }
    }), f345(match v0 {
        Adt21::C0(_, _, _, _, _, _, _, h1786) => {
            h1786
        }
    }, match v2 {
        Adt30::C0(_, _, _, _, _, _, _, h1787, _, _) => {
            h1787
        }
    })))
}

pub fn f247(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt29> {
    Ok(Adt29::C0(f346(v0.clone(), v1, v2, v3)?, f346(v0.clone(), v1, v2, {
        let a1788 = v3;
        let a1789 = 4u64;
        nat_add(a1788, a1789)?
    })?, f346(v0.clone(), v1, v2, {
        let a1790 = v3;
        let a1791 = 8u64;
        nat_add(a1790, a1791)?
    })?, f346(v0.clone(), v1, v2, {
        let a1792 = v3;
        let a1793 = 12u64;
        nat_add(a1792, a1793)?
    })?, f346(v0.clone(), v1, v2, {
        let a1794 = v3;
        let a1795 = 16u64;
        nat_add(a1794, a1795)?
    })?, f346(v0.clone(), v1, v2, {
        let a1796 = v3;
        let a1797 = 20u64;
        nat_add(a1796, a1797)?
    })?, f346(v0.clone(), v1, v2, {
        let a1798 = v3;
        let a1799 = 24u64;
        nat_add(a1798, a1799)?
    })?, f346(v0.clone(), v1, v2, {
        let a1800 = v3;
        let a1801 = 28u64;
        nat_add(a1800, a1801)?
    })?, f346(v0.clone(), v1, v2, {
        let a1802 = v3;
        let a1803 = 32u64;
        nat_add(a1802, a1803)?
    })?, f346(v0.clone(), v1, v2, {
        let a1804 = v3;
        let a1805 = 36u64;
        nat_add(a1804, a1805)?
    })?, f346(v0.clone(), v1, v2, {
        let a1806 = v3;
        let a1807 = 40u64;
        nat_add(a1806, a1807)?
    })?, f346(v0.clone(), v1, v2, {
        let a1808 = v3;
        let a1809 = 44u64;
        nat_add(a1808, a1809)?
    })?, f346(v0.clone(), v1, v2, {
        let a1810 = v3;
        let a1811 = 48u64;
        nat_add(a1810, a1811)?
    })?, f346(v0.clone(), v1, v2, {
        let a1812 = v3;
        let a1813 = 52u64;
        nat_add(a1812, a1813)?
    })?, f346(v0.clone(), v1, v2, {
        let a1814 = v3;
        let a1815 = 56u64;
        nat_add(a1814, a1815)?
    })?, f346(v0, v1, v2, {
        let a1816 = v3;
        let a1817 = 60u64;
        nat_add(a1816, a1817)?
    })?))
}

pub fn f248(p1872: u64, v1: Bytes, p1873: u64, p1874: List<Adt23>) -> R<Result<List<Adt23>, Bytes>> {
    let m1871 = (p1872, (p1873, p1874));
    {
        let mut m1871 = m1871;
        loop {
            let step = {
                let (v0, (v2, v3)) = m1871;
                let m1818 = v0;
                if m1818 == 0 {
                    Err::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>(Ok(Err::<List<Adt23>, Bytes>(f347(Str::lit("the input has more tokens than bytes"), v2)?)))
                } else {
                    let v4 = m1818 - 1;
                    let v5: u64 = f129(v1.clone(), v2)?;
                    let a1819 = v5;
                    let a1820 = 256u64;
                    if nat_eq(a1819, a1820) {
                        Err::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>(Ok(Ok::<List<Adt23>, Bytes>(f348(v3, List::<Adt23>::nil()))))
                    } else {
                        if f190(v5)? {
                            Ok::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>((v4, ({
                                let a1821 = v2;
                                let a1822 = 1u64;
                                nat_add(a1821, a1822)?
                            }, v3)))
                        } else {
                            if f128(v1.clone(), v2, Str::lit("//")) {
                                let m1823 = f191(v1.clone(), Bytes::lit(&[0x0a]), v2)?;
                                match m1823 {
                                    None => {
                                        Err::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>(Ok(Ok::<List<Adt23>, Bytes>(f348(v3, List::<Adt23>::nil()))))
                                    }
                                    Some(v6) => {
                                        Ok::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>((v4, ({
                                            let a1824 = v6;
                                            let a1825 = 1u64;
                                            nat_add(a1824, a1825)?
                                        }, v3)))
                                    }
                                }
                            } else {
                                if f128(v1.clone(), v2, Str::lit("/*")) {
                                    let m1829 = f191(v1.clone(), {
                                        let a1826 = Str::lit("*/");
                                        utf8_encode(a1826)
                                    }, {
                                        let a1827 = v2;
                                        let a1828 = 2u64;
                                        nat_add(a1827, a1828)?
                                    })?;
                                    match m1829 {
                                        None => {
                                            Err::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>(Ok(Err::<List<Adt23>, Bytes>(f347(Str::lit("a comment is not closed (Web IDL, IDL grammar: comment)"), v2)?)))
                                        }
                                        Some(v7) => {
                                            Ok::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>((v4, ({
                                                let a1830 = v7;
                                                let a1831 = 2u64;
                                                nat_add(a1830, a1831)?
                                            }, v3)))
                                        }
                                    }
                                } else {
                                    let a1832 = v5;
                                    let a1833 = 34u64;
                                    if nat_eq(a1832, a1833) {
                                        let m1836 = f191(v1.clone(), Bytes::lit(&[0x22]), {
                                            let a1834 = v2;
                                            let a1835 = 1u64;
                                            nat_add(a1834, a1835)?
                                        })?;
                                        match m1836 {
                                            None => {
                                                Err::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>(Ok(Err::<List<Adt23>, Bytes>(f347(Str::lit("a string is not closed (Web IDL, IDL grammar: string)"), v2)?)))
                                            }
                                            Some(v8) => {
                                                Ok::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>((v4, ({
                                                    let a1837 = v8;
                                                    let a1838 = 1u64;
                                                    nat_add(a1837, a1838)?
                                                }, List::cons(Adt23::C0(3u64, f45(v1.clone(), {
                                                    let a1839 = v2;
                                                    let a1840 = 1u64;
                                                    nat_add(a1839, a1840)?
                                                }, {
                                                    let a1843 = v8;
                                                    let a1844 = {
                                                        let a1841 = v2;
                                                        let a1842 = 1u64;
                                                        nat_add(a1841, a1842)?
                                                    };
                                                    nat_sub(a1843, a1844)
                                                })), v3))))
                                            }
                                        }
                                    } else {
                                        let v9: u64 = {
                                            let a1845 = v5;
                                            let a1846 = 95u64;
                                            if if nat_eq(a1845, a1846) {
                                                true
                                            } else {
                                                let a1847 = v5;
                                                let a1848 = 45u64;
                                                nat_eq(a1847, a1848)
                                            } {
                                                let a1849 = v2;
                                                let a1850 = 1u64;
                                                nat_add(a1849, a1850)?
                                            } else {
                                                v2
                                            }
                                        };
                                        if f349(f129(v1.clone(), v9)?) {
                                            let v10: u64 = f350(v1.clone(), 3u64, {
                                                let a1851 = v9;
                                                let a1852 = 1u64;
                                                nat_add(a1851, a1852)?
                                            })?;
                                            Ok::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>((v4, (v10, List::cons(Adt23::C0(0u64, f45(v1.clone(), v2, {
                                                let a1853 = v10;
                                                let a1854 = v2;
                                                nat_sub(a1853, a1854)
                                            })), v3))))
                                        } else {
                                            let v11: u64 = {
                                                let a1855 = v5;
                                                let a1856 = 45u64;
                                                if nat_eq(a1855, a1856) {
                                                    let a1857 = v2;
                                                    let a1858 = 1u64;
                                                    nat_add(a1857, a1858)?
                                                } else {
                                                    v2
                                                }
                                            };
                                            let v12: u64 = f351(v1.clone(), v11)?;
                                            let v13: u64 = f352(v1.clone(), v11)?;
                                            let a1859 = v12;
                                            let a1860 = v13;
                                            if nat_lt(a1859, a1860) {
                                                Ok::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>((v4, (v13, List::cons(Adt23::C0(2u64, f45(v1.clone(), v2, {
                                                    let a1861 = v13;
                                                    let a1862 = v2;
                                                    nat_sub(a1861, a1862)
                                                })), v3))))
                                            } else {
                                                let a1863 = v11;
                                                let a1864 = v12;
                                                if nat_lt(a1863, a1864) {
                                                    Ok::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>((v4, (v12, List::cons(Adt23::C0(1u64, f45(v1.clone(), v2, {
                                                        let a1865 = v12;
                                                        let a1866 = v2;
                                                        nat_sub(a1865, a1866)
                                                    })), v3))))
                                                } else {
                                                    if f128(v1.clone(), v2, Str::lit("...")) {
                                                        Ok::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>((v4, ({
                                                            let a1867 = v2;
                                                            let a1868 = 3u64;
                                                            nat_add(a1867, a1868)?
                                                        }, List::cons(Adt23::C0(4u64, f8(Str::lit("..."))), v3))))
                                                    } else {
                                                        Ok::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>((v4, ({
                                                            let a1869 = v2;
                                                            let a1870 = 1u64;
                                                            nat_add(a1869, a1870)?
                                                        }, List::cons(Adt23::C0(4u64, f45(v1.clone(), v2, 1u64)), v3))))
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
                Ok(next) => { m1871 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f249(p1877: List<Adt14>, p1878: List<Adt14>) -> List<Adt14> {
    let m1876 = (p1877, p1878);
    {
        let mut m1876 = m1876;
        loop {
            let step = {
                let (v0, v1) = m1876;
                let m1875 = v0;
                match m1875.uncons() {
                    None => {
                        Err::<(List<Adt14>, List<Adt14>), List<Adt14>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt14>, List<Adt14>), List<Adt14>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1876 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f250(v0: List<Adt23>) -> R<Result<List<Adt23>, Bytes>> {
    if f353(v0.clone(), Str::lit("[")) {
        Ok(f354({
            let a1880 = {
                let a1879 = v0.clone();
                length_list(a1879)
            };
            let a1881 = 1u64;
            nat_add(a1880, a1881)?
        }, f355(v0), List::cons(f8(Str::lit("]")), List::<Bytes>::nil()), false))
    } else {
        Ok(Ok::<List<Adt23>, Bytes>(v0))
    }
}

pub fn f251(v0: u64, v1: List<Adt23>) -> R<Result<(Adt14, List<Adt23>), Bytes>> {
    if f353(v1.clone(), Str::lit("interface")) {
        if f353(f355(v1.clone()), Str::lit("mixin")) {
            f356(v0, 1u64, f355(f355(v1)), false)
        } else {
            f356(v0, 0u64, f355(v1), true)
        }
    } else {
        if f353(v1.clone(), Str::lit("partial")) {
            let v2: List<Adt23> = f355(v1);
            if f353(v2.clone(), Str::lit("interface")) {
                if f353(f355(v2.clone()), Str::lit("mixin")) {
                    f356(v0, 6u64, f355(f355(v2)), false)
                } else {
                    f356(v0, 5u64, f355(v2), false)
                }
            } else {
                if f353(v2.clone(), Str::lit("dictionary")) {
                    f356(v0, 7u64, f355(v2), false)
                } else {
                    if f353(v2.clone(), Str::lit("namespace")) {
                        f357(f355(v2), f8(Str::lit("a namespace holds operations and attributes of no object, so it is behavior, not data (Web IDL 2.6)")))
                    } else {
                        Ok(Err::<(Adt14, List<Adt23>), Bytes>(f358(Str::lit("interface, dictionary or namespace"), v2)))
                    }
                }
            }
        } else {
            if f353(v1.clone(), Str::lit("dictionary")) {
                f356(v0, 2u64, f355(v1), true)
            } else {
                if f353(v1.clone(), Str::lit("enum")) {
                    let m1882 = f359(f355(v1));
                    match m1882 {
                        Ok(v4) => {
                            let m1883 = f360(f361(v4.clone()), Str::lit("{"));
                            match m1883 {
                                Ok(v6) => {
                                    let m1884 = f362(v0, v6, List::<Bytes>::nil());
                                    match m1884 {
                                        Ok(v8) => {
                                            Ok(Ok::<(Adt14, List<Adt23>), Bytes>((Adt14::C0(3u64, f363(v4), Bytes::lit(&[]), List::<Adt15>::nil(), {
                                                let (h1885, _) = v8;
                                                h1885
                                            }, f364(Bytes::lit(&[]))), {
                                                let (_, h1886) = v8;
                                                h1886
                                            })))
                                        }
                                        Err(v7) => {
                                            Ok(Err::<(Adt14, List<Adt23>), Bytes>(v7))
                                        }
                                    }
                                }
                                Err(v5) => {
                                    Ok(Err::<(Adt14, List<Adt23>), Bytes>(v5))
                                }
                            }
                        }
                        Err(v3) => {
                            Ok(Err::<(Adt14, List<Adt23>), Bytes>(v3))
                        }
                    }
                } else {
                    if f353(v1.clone(), Str::lit("typedef")) {
                        let m1887 = f365(v0, f355(v1), 0u64)?;
                        match m1887 {
                            Ok(v10) => {
                                let m1888 = f359(f366(v10.clone()));
                                match m1888 {
                                    Ok(v12) => {
                                        let m1889 = f360(f361(v12.clone()), Str::lit(";"));
                                        match m1889 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt14, List<Adt23>), Bytes>((Adt14::C0(4u64, f363(v12), Bytes::lit(&[]), List::<Adt15>::nil(), List::<Bytes>::nil(), f367(v10)), v14)))
                                            }
                                            Err(v13) => {
                                                Ok(Err::<(Adt14, List<Adt23>), Bytes>(v13))
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(Err::<(Adt14, List<Adt23>), Bytes>(v11))
                                    }
                                }
                            }
                            Err(v9) => {
                                Ok(Err::<(Adt14, List<Adt23>), Bytes>(v9))
                            }
                        }
                    } else {
                        if f353(v1.clone(), Str::lit("callback")) {
                            if f353(f355(v1.clone()), Str::lit("interface")) {
                                f357(f355(f355(v1)), f8(Str::lit("a callback interface is implemented by user code, so it is behavior, not data (Web IDL 2.4)")))
                            } else {
                                f357(f355(v1), f8(Str::lit("a callback function is behavior, not data (Web IDL 2.10)")))
                            }
                        } else {
                            if f353(v1.clone(), Str::lit("namespace")) {
                                f357(f355(v1), f8(Str::lit("a namespace holds operations and attributes of no object, so it is behavior, not data (Web IDL 2.6)")))
                            } else {
                                let m1890 = f359(v1);
                                match m1890 {
                                    Ok(v16) => {
                                        let m1891 = f360(f361(v16.clone()), Str::lit("includes"));
                                        match m1891 {
                                            Ok(v18) => {
                                                let m1892 = f359(v18);
                                                match m1892 {
                                                    Ok(v20) => {
                                                        let m1893 = f360(f361(v20.clone()), Str::lit(";"));
                                                        match m1893 {
                                                            Ok(v22) => {
                                                                Ok(Ok::<(Adt14, List<Adt23>), Bytes>((Adt14::C0(8u64, f363(v16), f363(v20), List::<Adt15>::nil(), List::<Bytes>::nil(), f364(Bytes::lit(&[]))), v22)))
                                                            }
                                                            Err(v21) => {
                                                                Ok(Err::<(Adt14, List<Adt23>), Bytes>(v21))
                                                            }
                                                        }
                                                    }
                                                    Err(v19) => {
                                                        Ok(Err::<(Adt14, List<Adt23>), Bytes>(v19))
                                                    }
                                                }
                                            }
                                            Err(v17) => {
                                                Ok(Err::<(Adt14, List<Adt23>), Bytes>(v17))
                                            }
                                        }
                                    }
                                    Err(v15) => {
                                        Ok(Err::<(Adt14, List<Adt23>), Bytes>(v15))
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

pub fn f252(p1897: List<Bytes>, v1: List<Adt17>) -> Bytes {
    let m1896 = p1897;
    {
        let mut m1896 = m1896;
        loop {
            let step = {
                let v0 = m1896;
                let m1894 = v0;
                match m1894.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1895 = f174(v1.clone(), v2.clone());
                        match m1895 {
                            None => {
                                Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("refers to ")), List::cons(v2, List::cons(f8(Str::lit(", which no input defines (Web IDL 2.13.22 to 2.13.26)")), List::<Bytes>::nil())))))
                            }
                            Some(v4) => {
                                if f32(f175(v4.clone()), Bytes::lit(&[])) {
                                    Ok::<List<Bytes>, Bytes>(v3)
                                } else {
                                    Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("refers to ")), List::cons(v2, List::cons(f8(Str::lit(", which is not translated: ")), List::cons(f175(v4), List::<Bytes>::nil()))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1896 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f253(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f211(v0)?;
    if f173(v1.clone(), f368()) {
        f369(v1)
    } else {
        Ok(v1)
    }
}

pub fn f254(p1903: List<Adt14>, v1: List<Adt14>, v2: Bytes) -> Option<Bytes> {
    let m1902 = p1903;
    {
        let mut m1902 = m1902;
        loop {
            let step = {
                let v0 = m1902;
                let m1898 = v0;
                match m1898.uncons() {
                    None => {
                        Err::<List<Adt14>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v3, v4)) => {
                        let a1899 = f170(v3.clone());
                        let a1900 = 8u64;
                        if if nat_eq(a1899, a1900) {
                            f32(f172(v3.clone()), v2.clone())
                        } else {
                            false
                        } {
                            let m1901 = f181(v1.clone(), 1u64, f183(v3.clone()));
                            match m1901 {
                                None => {
                                    Err::<List<Adt14>, Option<Bytes>>(Some(f183(v3)))
                                }
                                Some(_) => {
                                    Ok::<List<Adt14>, Option<Bytes>>(v4)
                                }
                            }
                        } else {
                            Ok::<List<Adt14>, Option<Bytes>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1902 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f255(v0: u64, v1: List<Adt14>, v2: Adt14) -> R<Result<List<Adt15>, Bytes>> {
    let m1904 = v0;
    if m1904 == 0 {
        Ok(Err::<List<Adt15>, Bytes>(f7(List::cons(f8(Str::lit("the inheritance of ")), List::cons(f172(v2), List::cons(f8(Str::lit(" is cyclic (Web IDL 2.2 and 2.7: an interface or dictionary must not inherit from itself)")), List::<Bytes>::nil()))))))
    } else {
        let v3 = m1904 - 1;
        let v4: List<Adt15> = {
            let a1905 = f170(v2.clone());
            let a1906 = 2u64;
            if nat_eq(a1905, a1906) {
                f370(f371(f372(v2.clone()), f373(v1.clone(), 7u64, f172(v2.clone()))))?
            } else {
                f371(f372(v2.clone()), f371(f373(v1.clone(), 5u64, f172(v2.clone())), f374(v1.clone(), v1.clone(), f172(v2.clone()))))
            }
        };
        if f32(f183(v2.clone()), Bytes::lit(&[])) {
            Ok(Ok::<List<Adt15>, Bytes>(v4))
        } else {
            let m1907 = f181(v1.clone(), f170(v2.clone()), f183(v2.clone()));
            match m1907 {
                None => {
                    Ok(Err::<List<Adt15>, Bytes>(f7(List::cons(f8(Str::lit("inherits ")), List::cons(f183(v2.clone()), List::cons({
                        let a1908 = f170(v2);
                        let a1909 = 2u64;
                        if nat_eq(a1908, a1909) {
                            f8(Str::lit(", which no input defines as a dictionary (Web IDL 2.7)"))
                        } else {
                            f8(Str::lit(", which no input defines as an interface (Web IDL 2.2)"))
                        }
                    }, List::<Bytes>::nil()))))))
                }
                Some(v5) => {
                    let m1910 = f255(v3, v1, v5)?;
                    match m1910 {
                        Ok(v7) => {
                            Ok(Ok::<List<Adt15>, Bytes>(f371(v7, v4)))
                        }
                        Err(v6) => {
                            Ok(Err::<List<Adt15>, Bytes>(v6))
                        }
                    }
                }
            }
        }
    }
}

pub fn f256(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: mk, fields: []}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))
}

pub fn f257(v0: bool, v1: List<Adt15>) -> R<Result<(List<Bytes>, List<Bytes>), Bytes>> {
    let m1911 = v1;
    match m1911.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::<Bytes>::nil(), List::<Bytes>::nil())))
        }
        Some((v2, v3)) => {
            let v4: Adt16 = f375(v2.clone());
            if !f32(f266(v4.clone()), Bytes::lit(&[])) {
                Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons(f376(v2), List::cons(f8(Str::lit(": ")), List::cons(f266(v4), List::<Bytes>::nil())))))))
            } else {
                let m1912 = f377(v0, f376(v2.clone()))?;
                match m1912 {
                    None => {
                        Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons(f376(v2), List::cons(f8(Str::lit(" has no field name")), List::<Bytes>::nil()))))))
                    }
                    Some(v5) => {
                        if f173(v5.clone(), f378(v0, f259(v3.clone()))?) {
                            Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("two of its members derive the field name ")), List::cons(v5, List::cons(f8(Str::lit(" (Web IDL 2.2 and 2.7: the members of one definition have distinct identifiers)")), List::<Bytes>::nil()))))))
                        } else {
                            let m1913 = f257(v0, v3)?;
                            match m1913 {
                                Ok(v7) => {
                                    Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v5, List::cons(f8(Str::lit(", type: ")), List::cons({
                                        let a1914 = f379(v2);
                                        let a1915 = 1u64;
                                        if nat_eq(a1914, a1915) {
                                            f7(List::cons(f8(Str::lit("(Option ")), List::cons(f268(v4.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                        } else {
                                            f268(v4.clone())
                                        }
                                    }, List::cons(f8(Str::lit("}")), List::<Bytes>::nil())))))), {
                                        let (h1916, _) = v7;
                                        h1916
                                    }), f176(f269(v4), {
                                        let (_, h1917) = v7;
                                        h1917
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

pub fn f258(p1921: List<Bytes>) -> R<bool> {
    let m1920 = p1921;
    {
        let mut m1920 = m1920;
        loop {
            let step = {
                let v0 = m1920;
                let m1918 = v0;
                match m1918.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m1919 = f380(v1)?;
                        match m1919 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f173(v3, f378(false, v2.clone())?) {
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
                Ok(next) => { m1920 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f259(v0: List<Adt15>) -> List<Bytes> {
    let m1922 = v0;
    match m1922.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f376(v1), f259(v2))
        }
    }
}

pub fn f260(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}")), List::cons(f117(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f261(v0: Adt14) -> List<Bytes> {
    let m1923 = v0;
    match m1923 {
        Adt14::C0(_, _, _, _, v5, _) => {
            v5
        }
    }
}

pub fn f262(v0: bool, v1: List<Adt2>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1924 = v1;
    match m1924.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m1925 = f381(v0, v3.clone())?;
            match m1925 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f148(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f173(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m1926 = f262(v0, v4, List::cons(v5.clone(), v2))?;
                        match m1926 {
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

pub fn f263(v0: List<Bytes>) -> List<Adt2> {
    let m1927 = v0;
    match m1927.uncons() {
        None => {
            List::<Adt2>::nil()
        }
        Some((v1, v2)) => {
            List::cons(Adt2::C5(v1), f263(v2))
        }
    }
}

pub fn f264(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f117(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f265(v0: Adt14) -> Adt16 {
    let m1928 = v0;
    match m1928 {
        Adt14::C0(_, _, _, _, _, v6) => {
            v6
        }
    }
}

pub fn f266(v0: Adt16) -> Bytes {
    let m1929 = v0;
    match m1929 {
        Adt16::C0(_, _, _, v4, _) => {
            v4
        }
    }
}

pub fn f267(v0: Adt16) -> u64 {
    let m1930 = v0;
    match m1930 {
        Adt16::C0(v1, _, _, _, _) => {
            v1
        }
    }
}

pub fn f268(v0: Adt16) -> Bytes {
    let m1931 = v0;
    match m1931 {
        Adt16::C0(_, v2, _, _, _) => {
            v2
        }
    }
}

pub fn f269(v0: Adt16) -> List<Bytes> {
    let m1932 = v0;
    match m1932 {
        Adt16::C0(_, _, v3, _, _) => {
            v3
        }
    }
}

pub fn f270(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f271(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h1933, _) = v2;
        h1933
    };
    let a1934 = v3;
    let a1935 = 0u64;
    if nat_eq(a1934, a1935) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a1936 = v3;
            let a1937 = v1;
            let a1938 = 0u64;
            nat_quot(a1936, a1937, a1938)
        }, {
            let a1943 = f184(v0, {
                let a1939 = v3;
                let a1940 = v1;
                let a1941 = 0u64;
                nat_rem(a1939, a1940, a1941)
            });
            let a1944 = {
                let (_, h1942) = v2;
                h1942
            };
            append_bytes(a1943, a1944)
        }))
    }
}

pub fn f272(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h1945, _) = v1;
        h1945
    };
    let a1947 = {
        let a1946 = v0.clone();
        length_bytes(a1946)
    };
    let a1948 = v2;
    if nat_le(a1947, a1948) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let a1949 = f129(v0.clone(), v2)?;
        let a1950 = 13u64;
        if nat_eq(a1949, a1950) {
            Ok(Some(({
                let a1953 = f129(v0, {
                    let a1951 = v2;
                    let a1952 = 1u64;
                    nat_add(a1951, a1952)?
                })?;
                let a1954 = 10u64;
                if nat_eq(a1953, a1954) {
                    let a1955 = v2;
                    let a1956 = 2u64;
                    nat_add(a1955, a1956)?
                } else {
                    let a1957 = v2;
                    let a1958 = 1u64;
                    nat_add(a1957, a1958)?
                }
            }, {
                let a1960 = {
                    let (_, h1959) = v1;
                    h1959
                };
                let a1961 = f8(Str::lit("\u{a}"));
                append_bytes(a1960, a1961)
            })))
        } else {
            let v4: u64 = {
                let m1962 = f191(v0.clone(), f8(Str::lit("\u{d}")), v2)?;
                match m1962 {
                    None => {
                        let a1963 = v0.clone();
                        length_bytes(a1963)
                    }
                    Some(v3) => {
                        v3
                    }
                }
            };
            Ok(Some((v4, {
                let a1967 = {
                    let (_, h1964) = v1;
                    h1964
                };
                let a1968 = f45(v0, v2, {
                    let a1965 = v4;
                    let a1966 = v2;
                    nat_sub(a1965, a1966)
                });
                append_bytes(a1967, a1968)
            })))
        }
    }
}

pub fn f273(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1969 = f382(v0, v1)?;
    match m1969 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f383({
                let (h1970, _) = v2;
                h1970
            })? {
                Ok(Some({
                    let a1972 = v1;
                    let a1973 = {
                        let (_, h1971) = v2;
                        h1971
                    };
                    nat_add(a1972, a1973)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f274(v0: u64, v1: Bytes) -> R<bool> {
    f384(v0, v1.clone(), 0u64, {
        let a1975 = {
            let a1974 = v1;
            length_bytes(a1974)
        };
        let a1976 = 3u64;
        let a1977 = 0u64;
        nat_quot(a1975, a1976, a1977)
    }, true)
}

pub fn f275() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{0}\u{0} "))
}

pub fn f276(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f385(v1, v0, v2)
}

pub fn f277(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f386(v0, v1)
}

pub fn f278(v0: Bytes, p1980: List<Bytes>) -> bool {
    let m1979 = p1980;
    {
        let mut m1979 = m1979;
        loop {
            let step = {
                let v1 = m1979;
                let m1978 = v1;
                match m1978.uncons() {
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
                Ok(next) => { m1979 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f279(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f387(v0, v1, v2)
}

pub fn f280(v0: Bytes) -> Result<(Bytes, u64), Bytes> {
    Err::<(Bytes, u64), Bytes>(v0)
}

pub fn f281(v0: Bytes, v1: u64) -> R<Result<(Bytes, u64), Bytes>> {
    if f128(v0.clone(), v1, Str::lit("&#")) {
        let v2: bool = {
            let a1983 = f129(v0.clone(), {
                let a1981 = v1;
                let a1982 = 2u64;
                nat_add(a1981, a1982)?
            })?;
            let a1984 = 120u64;
            nat_eq(a1983, a1984)
        };
        let v3: u64 = if v2 {
            let a1985 = v1;
            let a1986 = 3u64;
            nat_add(a1985, a1986)?
        } else {
            let a1987 = v1;
            let a1988 = 2u64;
            nat_add(a1987, a1988)?
        };
        let v4: (u64, u64) = f388(v0.clone(), v2, v3)?;
        let a1990 = {
            let (h1989, _) = v4;
            h1989
        };
        let a1991 = v3;
        if if nat_eq(a1990, a1991) {
            true
        } else {
            let a1993 = f129(v0, {
                let (h1992, _) = v4;
                h1992
            })?;
            let a1994 = 59u64;
            !nat_eq(a1993, a1994)
        } {
            Ok(f280(f125(Str::lit("a character reference must be &#digits; or &#xhex; (XML 1.0 [66])"), v1)?))
        } else {
            if f383({
                let (_, h1995) = v4;
                h1995
            })? {
                Ok(Ok::<(Bytes, u64), Bytes>((f389({
                    let (_, h1996) = v4;
                    h1996
                })?, {
                    let a1998 = {
                        let (h1997, _) = v4;
                        h1997
                    };
                    let a1999 = 1u64;
                    nat_add(a1998, a1999)?
                })))
            } else {
                Ok(f280(f125(Str::lit("a character reference must name a Char (XML 1.0 [66] WFC: Legal Character)"), v1)?))
            }
        }
    } else {
        if f128(v0.clone(), v1, Str::lit("&lt;")) {
            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("<")), {
                let a2000 = v1;
                let a2001 = 4u64;
                nat_add(a2000, a2001)?
            })))
        } else {
            if f128(v0.clone(), v1, Str::lit("&gt;")) {
                Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit(">")), {
                    let a2002 = v1;
                    let a2003 = 4u64;
                    nat_add(a2002, a2003)?
                })))
            } else {
                if f128(v0.clone(), v1, Str::lit("&amp;")) {
                    Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("&")), {
                        let a2004 = v1;
                        let a2005 = 5u64;
                        nat_add(a2004, a2005)?
                    })))
                } else {
                    if f128(v0.clone(), v1, Str::lit("&apos;")) {
                        Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("'")), {
                            let a2006 = v1;
                            let a2007 = 6u64;
                            nat_add(a2006, a2007)?
                        })))
                    } else {
                        if f128(v0, v1, Str::lit("&quot;")) {
                            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("\"")), {
                                let a2008 = v1;
                                let a2009 = 6u64;
                                nat_add(a2008, a2009)?
                            })))
                        } else {
                            Ok(f280(f125(Str::lit("an entity reference names an entity that is not declared, and only the five predefined entities are (XML 1.0 section 4.1, WFC: Entity Declared)"), v1)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f282(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<u64> {
    let (h2010, _) = f188(Fn5::F390(v1, v2, v0.clone()), {
        let a2012 = {
            let a2011 = v0;
            length_bytes(a2011)
        };
        let a2013 = 1u64;
        nat_add(a2012, a2013)?
    }, v3)?;
    Ok(h2010)
}

pub fn f283(p2026: Adt18, v1: List<(Bytes, Bytes)>) -> R<Result<Option<Adt31>, Bytes>> {
    let m2025 = p2026;
    {
        let mut m2025 = m2025;
        loop {
            let step = {
                let v0 = m2025;
                let m2014 = v0;
                match m2014 {
                    Adt18::C0(_, _, _) => {
                        Err::<Adt18, R<Result<Option<Adt31>, Bytes>>>(Ok(Ok::<Option<Adt31>, Bytes>(None::<Adt31>)))
                    }
                    Adt18::C1(_, _, _) => {
                        Err::<Adt18, R<Result<Option<Adt31>, Bytes>>>(Ok(Ok::<Option<Adt31>, Bytes>(None::<Adt31>)))
                    }
                    Adt18::C2(_) => {
                        Err::<Adt18, R<Result<Option<Adt31>, Bytes>>>(Ok(Ok::<Option<Adt31>, Bytes>(None::<Adt31>)))
                    }
                    Adt18::C3(r2023, r2024) => {
                        let v2 = (*r2023).clone();
                        let v3 = (*r2024).clone();
                        let m2015 = v2;
                        match m2015 {
                            Adt18::C0(v4, r2021, r2022) => {
                                let v5 = (*r2021).clone();
                                let v6 = (*r2022).clone();
                                let m2016 = f132(v5.clone(), v1.clone());
                                match m2016 {
                                    Ok(v8) => {
                                        let m2017 = f133(v8.clone(), v4)?;
                                        match m2017 {
                                            Ok(v10) => {
                                                if f278({
                                                    let (_, h2018) = v10.clone();
                                                    h2018
                                                }, List::cons(f8(Str::lit("restriction")), List::cons(f8(Str::lit("list")), List::cons(f8(Str::lit("union")), List::<Bytes>::nil())))) {
                                                    if f32({
                                                        let (h2019, _) = v10;
                                                        h2019
                                                    }, f208()) {
                                                        Err::<Adt18, R<Result<Option<Adt31>, Bytes>>>(Ok(Ok::<Option<Adt31>, Bytes>(Some(Adt31::C0({
                                                            let (_, h2020) = v10;
                                                            h2020
                                                        }, v5, v6, v8)))))
                                                    } else {
                                                        Ok::<Adt18, R<Result<Option<Adt31>, Bytes>>>(v3)
                                                    }
                                                } else {
                                                    Ok::<Adt18, R<Result<Option<Adt31>, Bytes>>>(v3)
                                                }
                                            }
                                            Err(v9) => {
                                                Err::<Adt18, R<Result<Option<Adt31>, Bytes>>>(Ok(f391(v9)))
                                            }
                                        }
                                    }
                                    Err(v7) => {
                                        Err::<Adt18, R<Result<Option<Adt31>, Bytes>>>(Ok(f391(v7)))
                                    }
                                }
                            }
                            Adt18::C1(_, _, _) => {
                                Ok::<Adt18, R<Result<Option<Adt31>, Bytes>>>(v3)
                            }
                            Adt18::C2(_) => {
                                Ok::<Adt18, R<Result<Option<Adt31>, Bytes>>>(v3)
                            }
                            Adt18::C3(_, _) => {
                                Ok::<Adt18, R<Result<Option<Adt31>, Bytes>>>(v3)
                            }
                            Adt18::C4 => {
                                Ok::<Adt18, R<Result<Option<Adt31>, Bytes>>>(v3)
                            }
                        }
                    }
                    Adt18::C4 => {
                        Err::<Adt18, R<Result<Option<Adt31>, Bytes>>>(Ok(Ok::<Option<Adt31>, Bytes>(None::<Adt31>)))
                    }
                }
            };
            match step {
                Ok(next) => { m2025 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f284(v0: Bytes) -> Result<Adt24, Bytes> {
    Err::<Adt24, Bytes>(v0)
}

pub fn f285(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit(" at simpleType ")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f286(v0: List<(Bytes, Bytes)>, v1: Bytes, v2: Bytes) -> R<Result<Adt24, Bytes>> {
    let m2027 = f133(v0, v2.clone())?;
    match m2027 {
        Ok(v4) => {
            if f32({
                let (h2028, _) = v4.clone();
                h2028
            }, f208()) {
                let m2030 = f392({
                    let (_, h2029) = v4;
                    h2029
                }, 16u64)?;
                match m2030 {
                    Ok(v6) => {
                        Ok(Ok::<Adt24, Bytes>(Adt24::C0(v6, List::<Bytes>::nil())))
                    }
                    Err(v5) => {
                        Ok(f284(v5))
                    }
                }
            } else {
                if f32({
                    let (h2031, _) = v4.clone();
                    h2031
                }, v1) {
                    let v7: Bytes = f211({
                        let (_, h2032) = v4;
                        h2032
                    })?;
                    Ok(Ok::<Adt24, Bytes>(Adt24::C0(v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                } else {
                    Ok(f284(f7(List::cons(f8(Str::lit("the type ")), List::cons(v2, List::cons(f8(Str::lit(" is in the namespace ")), List::cons({
                        let (h2033, _) = v4;
                        h2033
                    }, List::cons(f8(Str::lit(", which is neither XML Schema's nor this document's target namespace; imported schema documents are not read yet")), List::<Bytes>::nil()))))))))
                }
            }
        }
        Err(v3) => {
            Ok(f284(v3))
        }
    }
}

pub fn f287(v0: Adt18, v1: List<(Bytes, Bytes)>) -> R<Result<List<Adt2>, Bytes>> {
    let m2034 = v0;
    match m2034 {
        Adt18::C0(_, _, _) => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
        Adt18::C1(_, _, _) => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
        Adt18::C2(_) => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
        Adt18::C3(r2041, r2042) => {
            let v2 = (*r2041).clone();
            let v3 = (*r2042).clone();
            let m2035 = f287(v3, v1.clone())?;
            match m2035 {
                Ok(v5) => {
                    let m2036 = v2;
                    match m2036 {
                        Adt18::C0(v6, r2040, _) => {
                            let v7 = (*r2040).clone();
                            let m2037 = f132(v7.clone(), v1);
                            match m2037 {
                                Ok(v10) => {
                                    let m2038 = f133(v10, v6)?;
                                    match m2038 {
                                        Ok(v12) => {
                                            if f134(v12, Str::lit("enumeration")) {
                                                let m2039 = f135(v7, f8(Str::lit("value")));
                                                match m2039 {
                                                    None => {
                                                        Ok(f393(f8(Str::lit("an enumeration facet must have a value attribute (XSD 1.1 Part 2, section 4.3.5.2)"))))
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
                                            Ok(f393(v11))
                                        }
                                    }
                                }
                                Err(v9) => {
                                    Ok(f393(v9))
                                }
                            }
                        }
                        Adt18::C1(_, _, _) => {
                            Ok(Ok::<List<Adt2>, Bytes>(v5))
                        }
                        Adt18::C2(_) => {
                            Ok(Ok::<List<Adt2>, Bytes>(v5))
                        }
                        Adt18::C3(_, _) => {
                            Ok(Ok::<List<Adt2>, Bytes>(v5))
                        }
                        Adt18::C4 => {
                            Ok(Ok::<List<Adt2>, Bytes>(v5))
                        }
                    }
                }
                Err(v4) => {
                    Ok(f393(v4))
                }
            }
        }
        Adt18::C4 => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
    }
}

pub fn f288(v0: List<Adt2>, v1: List<Bytes>) -> R<List<Adt2>> {
    let m2043 = v0;
    match m2043.uncons() {
        None => {
            Ok(List::<Adt2>::nil())
        }
        Some((v2, v3)) => {
            let v4: Bytes = f148(v2.clone())?;
            if f173(v4.clone(), v1.clone()) {
                f288(v3, v1)
            } else {
                Ok(List::cons(v2, f288(v3, List::cons(v4, v1))?))
            }
        }
    }
}

pub fn f289(v0: List<Adt2>) -> List<Bytes> {
    let m2044 = v0;
    match m2044.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2045 = f394(Some(v1));
            match m2045 {
                None => {
                    f289(v2)
                }
                Some(v3) => {
                    List::cons(v3, f289(v2))
                }
            }
        }
    }
}

pub fn f290(v0: Bytes, p2066: u64, p2067: u64, p2068: bool, p2069: Bytes) -> R<Bytes> {
    let m2065 = (p2066, (p2067, (p2068, p2069)));
    {
        let mut m2065 = m2065;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m2065;
                let m2046 = v2;
                if m2046 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m2046 - 1;
                    let m2049 = {
                        let a2047 = v0.clone();
                        let a2048 = v1;
                        index_bytes(a2047, a2048)
                    };
                    match m2049 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f56(v6)?;
                            if f395(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a2050 = v1;
                                    let a2051 = 1u64;
                                    nat_add(a2050, a2051)?
                                }, (v5, (true, v4))))
                            } else {
                                if f96(f396(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a2052 = v1;
                                        let a2053 = 1u64;
                                        nat_add(a2052, a2053)?
                                    }, (v5, (false, {
                                        let a2057 = v4;
                                        let a2058 = f45({
                                            let a2054 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a2054)
                                        }, {
                                            let a2055 = v7;
                                            let a2056 = 97u64;
                                            nat_sub(a2055, a2056)
                                        }, 1u64);
                                        append_bytes(a2057, a2058)
                                    }))))
                                } else {
                                    if f153(f396(v7), f153(f397(v7), f398(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a2059 = v1;
                                            let a2060 = 1u64;
                                            nat_add(a2059, a2060)?
                                        }, (v5, (false, {
                                            let a2061 = v4;
                                            let a2062 = f45(v0.clone(), v1, 1u64);
                                            append_bytes(a2061, a2062)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a2063 = v1;
                                            let a2064 = 1u64;
                                            nat_add(a2063, a2064)?
                                        }, (v5, (v3, v4))))
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

pub fn f291(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f204(v0.clone(), v1) {
        let m2070 = f43(v0, 34u8)?;
        match m2070.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m2071 = v3;
                match m2071.uncons() {
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

pub fn f292(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f399(v0, v1)
}

pub fn f293(v0: List<Adt20>) -> R<Result<List<Adt26>, Bytes>> {
    let m2072 = v0;
    match m2072.uncons() {
        None => {
            Ok(Ok::<List<Adt26>, Bytes>(List::<Adt26>::nil()))
        }
        Some((v1, v2)) => {
            let m2075 = f224(match v1.clone() {
                Adt20::C0(_, _, _, _, h2073, _) => {
                    h2073
                }
            }, match v1.clone() {
                Adt20::C0(_, _, _, h2074, _, _) => {
                    h2074
                }
            })?;
            match m2075 {
                Ok(v4) => {
                    let m2076 = f293(v2)?;
                    match m2076 {
                        Ok(v6) => {
                            Ok(Ok::<List<Adt26>, Bytes>(f400(f401(v4, match v1 {
                                Adt20::C0(_, _, h2077, _, _, _) => {
                                    h2077
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

pub fn f294(v0: List<Adt20>) -> List<Bytes> {
    let m2078 = v0;
    match m2078.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt20::C0(_, _, h2079, _, _, _) => {
                    h2079
                }
            }, f294(v2))
        }
    }
}

pub fn f295(v0: u64, v1: Adt25, v2: List<Adt26>) -> R<Result<List<Adt28>, Bytes>> {
    let m2080 = v2;
    match m2080.uncons() {
        None => {
            Ok(Ok::<List<Adt28>, Bytes>(List::<Adt28>::nil()))
        }
        Some((v3, v4)) => {
            let m2081 = f402(v0, v1.clone(), v3)?;
            match m2081 {
                Ok(v6) => {
                    let m2082 = f295(v0, v1, v4)?;
                    match m2082 {
                        Ok(v8) => {
                            Ok(Ok::<List<Adt28>, Bytes>(f403(v6, v8)))
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

pub fn f296(v0: List<Adt20>) -> R<u64> {
    let m2083 = v0;
    match m2083.uncons() {
        None => {
            Ok(64u64)
        }
        Some((v1, v2)) => {
            let a2085 = match v1 {
                Adt20::C0(_, _, _, _, _, h2084) => {
                    h2084
                }
            };
            let a2086 = f296(v2)?;
            nat_add(a2085, a2086)
        }
    }
}

pub fn f297(p2094: List<Adt28>, p2095: List<Adt28>) -> Result<List<Adt28>, Bytes> {
    let m2093 = (p2094, p2095);
    {
        let mut m2093 = m2093;
        loop {
            let step = {
                let (v0, v1) = m2093;
                let m2087 = v0;
                match m2087.uncons() {
                    None => {
                        Err::<(List<Adt28>, List<Adt28>), Result<List<Adt28>, Bytes>>(Ok::<List<Adt28>, Bytes>(f404(v1, List::<Adt28>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m2089 = f405(v1.clone(), match v2.clone() {
                            Adt28::C0(h2088, _, _, _) => {
                                h2088
                            }
                        });
                        match m2089 {
                            None => {
                                Ok::<(List<Adt28>, List<Adt28>), Result<List<Adt28>, Bytes>>((v3, List::cons(v2, v1)))
                            }
                            Some(v4) => {
                                if f32(match v4 {
                                    Adt28::C0(_, _, h2090, _) => {
                                        h2090
                                    }
                                }, match v2.clone() {
                                    Adt28::C0(_, _, h2091, _) => {
                                        h2091
                                    }
                                }) {
                                    Ok::<(List<Adt28>, List<Adt28>), Result<List<Adt28>, Bytes>>((v3, v1))
                                } else {
                                    Err::<(List<Adt28>, List<Adt28>), Result<List<Adt28>, Bytes>>(Err::<List<Adt28>, Bytes>(f220(f8(Str::lit("two components derive the type name ")), match v2 {
                                        Adt28::C0(h2092, _, _, _) => {
                                            h2092
                                        }
                                    }, f8(Str::lit(", so their LexLean declarations would collide")))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2093 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f298(v0: List<Adt28>) -> List<Adt28> {
    if if f173(f406(), f407(v0.clone())) {
        let m2096 = f405(v0.clone(), f406());
        match m2096 {
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
        List::cons(Adt28::C0(f406(), List::<Bytes>::nil(), f264(f406(), f408()), f408()), v0)
    } else {
        v0
    }
}

pub fn f299(v0: List<Adt28>) -> List<Bytes> {
    let m2097 = v0;
    match m2097.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt28::C0(h2098, _, _, _) => {
                    h2098
                }
            }, f299(v2))
        }
    }
}

pub fn f300(p2104: List<Adt28>, v1: List<Bytes>) -> Option<(Bytes, Bytes)> {
    let m2103 = p2104;
    {
        let mut m2103 = m2103;
        loop {
            let step = {
                let v0 = m2103;
                let m2099 = v0;
                match m2099.uncons() {
                    None => {
                        Err::<List<Adt28>, Option<(Bytes, Bytes)>>(None::<(Bytes, Bytes)>)
                    }
                    Some((v2, v3)) => {
                        let m2101 = f212(match v2.clone() {
                            Adt28::C0(_, h2100, _, _) => {
                                h2100
                            }
                        }, v1.clone());
                        match m2101 {
                            None => {
                                Ok::<List<Adt28>, Option<(Bytes, Bytes)>>(v3)
                            }
                            Some(v4) => {
                                Err::<List<Adt28>, Option<(Bytes, Bytes)>>(Some((match v2 {
                                    Adt28::C0(h2102, _, _, _) => {
                                        h2102
                                    }
                                }, v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2103 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f301(v0: List<Adt28>, v1: List<Bytes>, v2: u64) -> List<(Bytes, List<Bytes>)> {
    let m2105 = v1;
    match m2105.uncons() {
        None => {
            List::<(Bytes, List<Bytes>)>::nil()
        }
        Some((v3, v4)) => {
            List::cons((v3.clone(), f409(v2, v0.clone(), f410(v0.clone(), v3), List::<Bytes>::nil())), f301(v0, v4, v2))
        }
    }
}

pub fn f302(v0: List<Adt28>) -> R<u64> {
    let m2106 = v0;
    match m2106.uncons() {
        None => {
            Ok(1u64)
        }
        Some((v1, v2)) => {
            let a2111 = {
                let a2109 = {
                    let a2108 = match v1 {
                        Adt28::C0(_, h2107, _, _) => {
                            h2107
                        }
                    };
                    length_list(a2108)
                };
                let a2110 = 1u64;
                nat_add(a2109, a2110)?
            };
            let a2112 = f302(v2)?;
            nat_add(a2111, a2112)
        }
    }
}

pub fn f303(p2122: u64, v1: List<Adt28>, p2123: List<(List<Bytes>, List<Bytes>)>, p2124: List<Bytes>, p2125: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m2121 = (p2122, (p2123, (p2124, p2125)));
    {
        let mut m2121 = m2121;
        loop {
            let step = {
                let (v0, (v2, (v3, v4))) = m2121;
                let m2113 = v0;
                if m2113 == 0 {
                    let m2114 = v2;
                    match m2114.uncons() {
                        None => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v4))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(f8(Str::lit("the declarations could not be ordered so that each follows the types it uses"))))
                        }
                    }
                } else {
                    let v7 = m2113 - 1;
                    let m2115 = v2.clone();
                    match m2115.uncons() {
                        None => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v4))
                        }
                        Some((_, _)) => {
                            let m2116 = f411(v2, v3.clone());
                            match m2116 {
                                None => {
                                    Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(f8(Str::lit("the declarations could not be ordered so that each follows the types it uses"))))
                                }
                                Some(v10) => {
                                    let m2118 = f412(v1.clone(), {
                                        let (h2117, _) = v10.clone();
                                        h2117
                                    });
                                    match m2118 {
                                        Ok(v12) => {
                                            Ok::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>((v7, ({
                                                let (_, h2119) = v10;
                                                h2119
                                            }, (f413({
                                                let (h2120, _) = v10;
                                                h2120
                                            }, v3), f413(v4, v12)))))
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
                Ok(next) => { m2121 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f304(v0: List<Adt28>, v1: List<List<Bytes>>) -> List<(List<Bytes>, List<Bytes>)> {
    let m2126 = v1;
    match m2126.uncons() {
        None => {
            List::<(List<Bytes>, List<Bytes>)>::nil()
        }
        Some((v2, v3)) => {
            List::cons((v2.clone(), f414(v0.clone(), v2.clone(), v2)), f304(v0, v3))
        }
    }
}

pub fn f305(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> List<List<Bytes>> {
    let m2127 = v2;
    match m2127.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((v4, v5)) => {
            if f173(v4.clone(), v3.clone()) {
                f305(v0, v1, v5, v3)
            } else {
                let v6: List<Bytes> = f415(v0.clone(), v1.clone(), v4);
                List::cons(v6.clone(), f305(v0, v1, v5, f413(v6, v3)))
            }
        }
    }
}

pub fn f306(v0: bool) -> bool {
    !v0
}

pub fn f307(v0: Adt27, v1: Str) -> bool {
    f32(match v0 {
        Adt27::C0(_, h2128, _, _, _) => {
            h2128
        }
    }, f8(v1))
}

pub fn f308(v0: Adt27, v1: Str) -> Option<Bytes> {
    f135(match v0 {
        Adt27::C0(_, _, h2129, _, _) => {
            h2129
        }
    }, f8(v1))
}

pub fn f309(v0: Option<Bytes>, v1: Bytes) -> R<Result<Bytes, Bytes>> {
    let m2130 = f191(v1.clone(), f8(Str::lit("://")), 0u64)?;
    match m2130 {
        None => {
            let v3: Bytes = f220(f8(Str::lit("the relative schema location ")), v1.clone(), f8(Str::lit(" is not resolved, because only a bare file name relative to a bound document's URL is merged (RFC 3986, section 5.2.3)")));
            if if f204(v1.clone(), f8(Str::lit("/"))) {
                true
            } else {
                let m2131 = f191(v1.clone(), f8(Str::lit("..")), 0u64)?;
                match m2131 {
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
                let m2132 = v0;
                match m2132 {
                    None => {
                        Ok(Err::<Bytes, Bytes>(v3))
                    }
                    Some(v5) => {
                        let m2134 = f416(v5.clone(), 0u64, {
                            let a2133 = v5.clone();
                            length_bytes(a2133)
                        }, None::<u64>)?;
                        match m2134 {
                            None => {
                                Ok(Err::<Bytes, Bytes>(v3))
                            }
                            Some(v6) => {
                                Ok(Ok::<Bytes, Bytes>({
                                    let a2137 = f45(v5, 0u64, {
                                        let a2135 = v6;
                                        let a2136 = 1u64;
                                        nat_add(a2135, a2136)?
                                    });
                                    let a2138 = v1;
                                    append_bytes(a2137, a2138)
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

pub fn f310(p2143: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m2142 = p2143;
    {
        let mut m2142 = m2142;
        loop {
            let step = {
                let v0 = m2142;
                let m2139 = v0;
                match m2139.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h2140, _) = v2;
                            h2140
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h2141) = v2;
                                h2141
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2142 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f311(v0: Adt27, v1: Str, v2: Str) -> Bytes {
    let m2144 = f308(v0, v1);
    match m2144 {
        None => {
            f8(v2)
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f312(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2145 = f382(v0, v1)?;
    match m2145 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f417({
                let (h2146, _) = v2;
                h2146
            }) {
                Ok(Some({
                    let a2148 = v1;
                    let a2149 = {
                        let (_, h2147) = v2;
                        h2147
                    };
                    nat_add(a2148, a2149)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f313(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2151 = v1;
    let a2152 = {
        let a2150 = v0.clone();
        length_bytes(a2150)
    };
    if if nat_lt(a2151, a2152) {
        f418(v0.clone(), v1)?
    } else {
        false
    } {
        Ok(Some(f318(v0, v1)?))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f314(v0: u64) -> bool {
    let a2153 = v0;
    let a2154 = 32u64;
    if nat_eq(a2153, a2154) {
        true
    } else {
        let a2155 = v0;
        let a2156 = 9u64;
        nat_eq(a2155, a2156)
    }
}

pub fn f315(v0: u64) -> bool {
    let a2157 = v0;
    let a2158 = 10u64;
    if nat_eq(a2157, a2158) {
        true
    } else {
        let a2159 = v0;
        let a2160 = 256u64;
        nat_eq(a2159, a2160)
    }
}

pub fn f316(v0: Bytes, v1: u64) -> R<u64> {
    let a2161 = v1;
    let a2162 = f419(v0, v1)?;
    Ok(nat_sub(a2161, a2162))
}

pub fn f317(v0: Bytes, v1: u64) -> R<u64> {
    let (h2163, _) = f188(Fn5::F420(v0.clone()), {
        let a2165 = {
            let a2164 = v0;
            length_bytes(a2164)
        };
        let a2166 = 1u64;
        nat_add(a2165, a2166)?
    }, v1)?;
    Ok(h2163)
}

pub fn f318(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f421(v0.clone(), v1)?;
    let a2168 = v2;
    let a2169 = {
        let a2167 = v0;
        length_bytes(a2167)
    };
    if nat_lt(a2168, a2169) {
        let a2170 = v2;
        let a2171 = 1u64;
        nat_add(a2170, a2171)
    } else {
        Ok(v2)
    }
}

pub fn f319(v0: Bytes, v1: u64) -> R<u64> {
    let (h2172, _) = f188(Fn5::F422(v0.clone()), {
        let a2174 = {
            let a2173 = v0;
            length_bytes(a2173)
        };
        let a2175 = 1u64;
        nat_add(a2174, a2175)?
    }, v1)?;
    Ok(h2172)
}

pub fn f320(v0: Bytes, v1: u64) -> R<bool> {
    let a2176 = f232(v0.clone(), v1)?;
    let a2177 = 45u64;
    if nat_eq(a2176, a2177) {
        if f314(f232(v0.clone(), {
            let a2178 = v1;
            let a2179 = 1u64;
            nat_add(a2178, a2179)?
        })?) {
            Ok(true)
        } else {
            Ok(f315(f232(v0, {
                let a2180 = v1;
                let a2181 = 1u64;
                nat_add(a2180, a2181)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f321(v0: Bytes, v1: u64) -> R<bool> {
    let m2182 = f327(v0, v1)?;
    match m2182 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f322(v0: Bytes, v1: u64) -> R<Option<Bytes>> {
    let v2: u64 = f232(v0.clone(), v1)?;
    let v3: bool = f234(f423(f232(v0, {
        let a2183 = v1;
        let a2184 = 1u64;
        nat_add(a2183, a2184)?
    })?, true));
    let a2185 = v2;
    let a2186 = 58u64;
    if if nat_eq(a2185, a2186) {
        v3
    } else {
        false
    } {
        Ok(Some(f125(Str::lit("a null key is not read: Studio.Value has no null (YAML 1.2.2 section 10.3.2)"), v1)?))
    } else {
        let a2187 = v2;
        let a2188 = 38u64;
        if nat_eq(a2187, a2188) {
            Ok(Some(f125(Str::lit("an anchor is not read (YAML 1.2.2 [101])"), v1)?))
        } else {
            let a2189 = v2;
            let a2190 = 42u64;
            if nat_eq(a2189, a2190) {
                Ok(Some(f125(Str::lit("an alias is not read (YAML 1.2.2 [104])"), v1)?))
            } else {
                let a2191 = v2;
                let a2192 = 33u64;
                if nat_eq(a2191, a2192) {
                    Ok(Some(f125(Str::lit("a tag is not read (YAML 1.2.2 [97])"), v1)?))
                } else {
                    let a2193 = v2;
                    let a2194 = 63u64;
                    if if nat_eq(a2193, a2194) {
                        v3
                    } else {
                        false
                    } {
                        Ok(Some(f125(Str::lit("an explicit key is not read (YAML 1.2.2 [143], [189])"), v1)?))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f323(v0: Bytes, v1: u64, v2: u64) -> R<Adt22> {
    let v3: u64 = f232(v0.clone(), v1)?;
    let a2195 = v3;
    let a2196 = 124u64;
    if if nat_eq(a2195, a2196) {
        true
    } else {
        let a2197 = v3;
        let a2198 = 62u64;
        nat_eq(a2197, a2198)
    } {
        f424(v0, v1, v2)
    } else {
        let a2199 = v3;
        let a2200 = 34u64;
        if if nat_eq(a2199, a2200) {
            true
        } else {
            let a2201 = v3;
            let a2202 = 39u64;
            nat_eq(a2201, a2202)
        } {
            let m2203 = f425(v0.clone(), v1, v2, false)?;
            match m2203 {
                Adt34::C0(v5, v6) => {
                    let m2204 = f231(v0, v6)?;
                    match m2204 {
                        Ok(v8) => {
                            Ok(Adt22::C0(Adt2::C5(v5), v8))
                        }
                        Err(v7) => {
                            Ok(Adt22::C1(v7))
                        }
                    }
                }
                Adt34::C1(v4) => {
                    Ok(Adt22::C1(v4))
                }
            }
        } else {
            let a2205 = v3;
            let a2206 = 91u64;
            if if nat_eq(a2205, a2206) {
                true
            } else {
                let a2207 = v3;
                let a2208 = 123u64;
                nat_eq(a2207, a2208)
            } {
                let m2212 = f426(v0.clone(), v1, v2, {
                    let a2210 = {
                        let a2209 = v0.clone();
                        length_bytes(a2209)
                    };
                    let a2211 = 2u64;
                    nat_add(a2210, a2211)?
                }, 0u64, List::<Bytes>::nil())?;
                match m2212 {
                    Adt22::C0(v10, v11) => {
                        let a2213 = f232(v0.clone(), f317(v0.clone(), v11)?)?;
                        let a2214 = 58u64;
                        if nat_eq(a2213, a2214) {
                            Ok(Adt22::C1(f125(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [154])"), v1)?))
                        } else {
                            let m2215 = f231(v0, v11)?;
                            match m2215 {
                                Ok(v13) => {
                                    Ok(Adt22::C0(v10, v13))
                                }
                                Err(v12) => {
                                    Ok(Adt22::C1(v12))
                                }
                            }
                        }
                    }
                    Adt22::C1(v9) => {
                        Ok(Adt22::C1(v9))
                    }
                }
            } else {
                let m2216 = f322(v0.clone(), v1)?;
                match m2216 {
                    None => {
                        if f234(f427(v0.clone(), v1, false)?) {
                            Ok(Adt22::C1(f125(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                        } else {
                            let v15: u64 = f428(v0.clone(), v1, false)?;
                            let m2220 = f429(v0.clone(), v15, v2, {
                                let a2218 = {
                                    let a2217 = v0.clone();
                                    length_bytes(a2217)
                                };
                                let a2219 = 2u64;
                                nat_add(a2218, a2219)?
                            }, f326(v0, v1, v15))?;
                            match m2220 {
                                Adt34::C0(v17, v18) => {
                                    f430(v17, v1, v18)
                                }
                                Adt34::C1(v16) => {
                                    Ok(Adt22::C1(v16))
                                }
                            }
                        }
                    }
                    Some(v14) => {
                        Ok(Adt22::C1(v14))
                    }
                }
            }
        }
    }
}

pub fn f324(v0: bool, v1: Adt22) -> Adt22 {
    let m2221 = v1;
    match m2221 {
        Adt22::C0(v3, v4) => {
            Adt22::C0(if v0 {
                Adt2::C6(std::rc::Rc::new(v3))
            } else {
                Adt2::C7(std::rc::Rc::new(v3))
            }, v4)
        }
        Adt22::C1(v2) => {
            Adt22::C1(v2)
        }
    }
}

pub fn f325(v0: Bytes, v1: u64) -> R<u64> {
    let (h2222, _) = f188(Fn5::F431(v0.clone(), v1), {
        let a2224 = {
            let a2223 = v0;
            length_bytes(a2223)
        };
        let a2225 = 1u64;
        nat_add(a2224, a2225)?
    }, 0u64)?;
    Ok(h2222)
}

pub fn f326(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    f45(v0, v1, {
        let a2226 = v2;
        let a2227 = v1;
        nat_sub(a2226, a2227)
    })
}

pub fn f327(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f232(v0.clone(), v1)?;
    let a2228 = v2;
    let a2229 = 34u64;
    if if nat_eq(a2228, a2229) {
        true
    } else {
        let a2230 = v2;
        let a2231 = 39u64;
        nat_eq(a2230, a2231)
    } {
        let m2232 = f425(v0.clone(), v1, 0u64, true)?;
        match m2232 {
            Adt34::C0(_, v5) => {
                let v6: u64 = f317(v0.clone(), v5)?;
                if f432(v0, v6)? {
                    Ok(Some(v6))
                } else {
                    Ok(None::<u64>)
                }
            }
            Adt34::C1(_) => {
                Ok(None::<u64>)
            }
        }
    } else {
        if f427(v0.clone(), v1, false)? {
            let v7: u64 = f317(v0.clone(), f428(v0.clone(), v1, false)?)?;
            if f432(v0, v7)? {
                Ok(Some(v7))
            } else {
                Ok(None::<u64>)
            }
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f328(v0: Bytes, v1: u64) -> R<Bytes> {
    let v2: u64 = f232(v0.clone(), v1)?;
    let a2233 = v2;
    let a2234 = 34u64;
    if if nat_eq(a2233, a2234) {
        true
    } else {
        let a2235 = v2;
        let a2236 = 39u64;
        nat_eq(a2235, a2236)
    } {
        let m2237 = f425(v0, v1, 0u64, true)?;
        match m2237 {
            Adt34::C0(v4, _) => {
                Ok(v4)
            }
            Adt34::C1(_) => {
                Ok(Bytes::lit(&[]))
            }
        }
    } else {
        Ok(f326(v0.clone(), v1, f428(v0, v1, false)?))
    }
}

pub fn f329(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a2238 = 0u64;
        let a2239 = v1;
        nat_lt(a2238, a2239)
    } else {
        false
    } {
        let a2240 = Bytes::lit(&[0x2d]);
        let a2241 = f76(v1)?;
        Ok(append_bytes(a2240, a2241))
    } else {
        f76(v1)
    }
}

pub fn f330(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f76(v1)?;
    let v4: u64 = {
        let a2242 = v3.clone();
        length_bytes(a2242)
    };
    let v5: Bytes = {
        let a2243 = v2;
        let a2244 = v4;
        if nat_lt(a2243, a2244) {
            f7(List::cons(f45(v3.clone(), 0u64, {
                let a2245 = v4;
                let a2246 = v2;
                nat_sub(a2245, a2246)
            }), List::cons(f8(Str::lit(".")), List::cons(f45(v3, {
                let a2247 = v4;
                let a2248 = v2;
                nat_sub(a2247, a2248)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f243({
                let a2249 = v2;
                let a2250 = v4;
                nat_sub(a2249, a2250)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a2251 = f8(Str::lit("-"));
        let a2252 = v5;
        Ok(append_bytes(a2251, a2252))
    } else {
        Ok(v5)
    }
}

pub fn f331(v0: Bytes) -> R<Bytes> {
    let a2257 = f150(v0.clone(), 0u64, {
        let a2254 = {
            let a2253 = v0.clone();
            length_bytes(a2253)
        };
        let a2255 = 1u64;
        nat_add(a2254, a2255)?
    })?;
    let a2258 = {
        let a2256 = v0.clone();
        length_bytes(a2256)
    };
    if nat_eq(a2257, a2258) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        let a2264 = {
            let a2262 = Bytes::lit(&[0x22]);
            let a2263 = f433(v0.clone(), 0u64, {
                let a2260 = {
                    let a2259 = v0;
                    length_bytes(a2259)
                };
                let a2261 = 1u64;
                nat_add(a2260, a2261)?
            }, Bytes::lit(&[]))?;
            append_bytes(a2262, a2263)
        };
        let a2265 = Bytes::lit(&[0x22]);
        Ok(append_bytes(a2264, a2265))
    }
}

pub fn f332(v0: List<Bytes>) -> Bytes {
    let a2268 = {
        let a2266 = Bytes::lit(&[0x5b]);
        let a2267 = f46(v0, Bytes::lit(&[0x2c]));
        append_bytes(a2266, a2267)
    };
    let a2269 = Bytes::lit(&[0x5d]);
    append_bytes(a2268, a2269)
}

pub fn f333(v0: Adt2) -> R<List<Bytes>> {
    let m2270 = v0;
    match m2270 {
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
        Adt2::C9(r2271, r2272) => {
            let v12 = (*r2271).clone();
            let v13 = (*r2272).clone();
            Ok(List::cons(f235(v12)?, f333(v13)?))
        }
        Adt2::C10(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f334(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f434(Fn8::F435, List::<(Bytes, Bytes)>::nil(), f436(v0)?);
    let v2: List<Bytes> = f437(Fn9::F438, List::<Bytes>::nil(), v1)?;
    let a2275 = {
        let a2273 = Bytes::lit(&[0x7b]);
        let a2274 = f46(v2, Bytes::lit(&[0x2c]));
        append_bytes(a2273, a2274)
    };
    let a2276 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a2275, a2276))
}

pub fn f335(v0: Adt2) -> R<List<(Bytes, Bytes)>> {
    let m2277 = v0;
    match m2277 {
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
        Adt2::C10(v14, r2278, r2279) => {
            let v15 = (*r2278).clone();
            let v16 = (*r2279).clone();
            Ok(List::cons((v14, f235(v15)?), f335(v16)?))
        }
    }
}

pub fn f336(v0: Bytes, v1: u64) -> R<Bytes> {
    let a2282 = f45(v0.clone(), {
        let a2280 = v1;
        let a2281 = 2u64;
        nat_add(a2280, a2281)?
    }, 2u64);
    let a2283 = Bytes::lit(&[0x30, 0x30]);
    if equal(a2282, a2283) {
        let m2286 = f439(f45(v0, {
            let a2284 = v1;
            let a2285 = 4u64;
            nat_add(a2284, a2285)?
        }, 2u64))?;
        match m2286 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f94(v2.clone(), 0u64)?;
                let a2287 = v3;
                let a2288 = 32u64;
                if nat_lt(a2287, a2288) {
                    if f440(v3) {
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

pub fn f337(v0: u64) -> Bytes {
    let a2289 = v0;
    let a2290 = 34u64;
    if nat_eq(a2289, a2290) {
        Bytes::lit(&[0x22])
    } else {
        let a2291 = v0;
        let a2292 = 92u64;
        if nat_eq(a2291, a2292) {
            Bytes::lit(&[0x5c])
        } else {
            let a2293 = v0;
            let a2294 = 98u64;
            if nat_eq(a2293, a2294) {
                Bytes::lit(&[0x08])
            } else {
                let a2295 = v0;
                let a2296 = 102u64;
                if nat_eq(a2295, a2296) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a2297 = v0;
                    let a2298 = 110u64;
                    if nat_eq(a2297, a2298) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a2299 = v0;
                        let a2300 = 114u64;
                        if nat_eq(a2299, a2300) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a2301 = v0;
                            let a2302 = 116u64;
                            if nat_eq(a2301, a2302) {
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

pub fn f338(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f94(v0, v1)?;
    let a2303 = 47u64;
    let a2304 = v2;
    if if nat_lt(a2303, a2304) {
        let a2305 = v2;
        let a2306 = 58u64;
        nat_lt(a2305, a2306)
    } else {
        false
    } {
        Ok(Some({
            let a2307 = v1;
            let a2308 = 1u64;
            nat_add(a2307, a2308)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f339(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2309 = f94(v0, v1)?;
    let a2310 = 48u64;
    if nat_eq(a2309, a2310) {
        Ok(Some({
            let a2311 = v1;
            let a2312 = 1u64;
            nat_add(a2311, a2312)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f340(v0: Bytes) -> R<u64> {
    let (h2313, _) = f188(Fn5::F441(v0.clone()), {
        let a2314 = v0;
        length_bytes(a2314)
    }, 0u64)?;
    Ok(h2313)
}

pub fn f341(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m2318 = {
        let a2316 = v0;
        let a2317 = {
            let (h2315, _) = v1;
            h2315
        };
        index_bytes(a2316, a2317)
    };
    match m2318 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2320 = {
                    let (h2319, _) = v1;
                    h2319
                };
                let a2321 = 1u64;
                nat_add(a2320, a2321)?
            }, {
                let a2325 = {
                    let a2323 = {
                        let (_, h2322) = v1;
                        h2322
                    };
                    let a2324 = 10u64;
                    nat_mul(a2323, a2324)?
                };
                let a2326 = f442(v2)?;
                nat_add(a2325, a2326)?
            })))
        }
    }
}

pub fn f342(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f443(v0, v1)
}

pub fn f343(v0: Fn7, p2330: u64, p2331: Adt30) -> R<Adt30> {
    let m2329 = (p2330, p2331);
    {
        let mut m2329 = m2329;
        loop {
            let step = {
                let (v1, v2) = m2329;
                let m2327 = v1;
                if m2327 == 0 {
                    Err::<(u64, Adt30), R<Adt30>>(Ok(v2))
                } else {
                    let v3 = m2327 - 1;
                    Ok::<(u64, Adt30), R<Adt30>>((v3, {
                        let c2328: Fn7 = v0.clone();
                        c2328.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2329 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f344(v0: Adt30) -> R<Adt30> {
    f444(v0)
}

pub fn f345(v0: u32, v1: u32) -> u32 {
    let m2334 = {
        let a2332 = f445(v0);
        let a2333 = f445(v1);
        fixed_u64::checked_add(a2332, a2333)
    };
    match m2334 {
        None => {
            0u32
        }
        Some(v2) => {
            f446(v2)
        }
    }
}

pub fn f346(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a2337 = {
        let a2335 = v3;
        let a2336 = 8u64;
        nat_add(a2335, a2336)?
    };
    let a2338 = v2;
    if nat_eq(a2337, a2338) {
        Ok(f447({
            let a2341 = {
                let a2339 = v1;
                let a2340 = 8u64;
                nat_mul(a2339, a2340)?
            };
            let a2342 = 4294967296u64;
            let a2343 = 0u64;
            nat_quot(a2341, a2342, a2343)
        }))
    } else {
        let a2346 = {
            let a2344 = v3;
            let a2345 = 4u64;
            nat_add(a2344, a2345)?
        };
        let a2347 = v2;
        if nat_eq(a2346, a2347) {
            Ok(f447({
                let a2350 = {
                    let a2348 = v1;
                    let a2349 = 8u64;
                    nat_mul(a2348, a2349)?
                };
                let a2351 = 4294967296u64;
                let a2352 = 0u64;
                nat_rem(a2350, a2351, a2352)
            }))
        } else {
            Ok(f448(f449(v0.clone(), v1, v3), f449(v0.clone(), v1, {
                let a2353 = v3;
                let a2354 = 1u64;
                nat_add(a2353, a2354)?
            }), f449(v0.clone(), v1, {
                let a2355 = v3;
                let a2356 = 2u64;
                nat_add(a2355, a2356)?
            }), f449(v0, v1, {
                let a2357 = v3;
                let a2358 = 3u64;
                nat_add(a2357, a2358)?
            })))
        }
    }
}

pub fn f347(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a2359 = v0;
        utf8_encode(a2359)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f76(v1)?, List::<Bytes>::nil())))))
}

pub fn f348(p2362: List<Adt23>, p2363: List<Adt23>) -> List<Adt23> {
    let m2361 = (p2362, p2363);
    {
        let mut m2361 = m2361;
        loop {
            let step = {
                let (v0, v1) = m2361;
                let m2360 = v0;
                match m2360.uncons() {
                    None => {
                        Err::<(List<Adt23>, List<Adt23>), List<Adt23>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt23>, List<Adt23>), List<Adt23>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2361 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f349(v0: u64) -> bool {
    let a2364 = 65u64;
    let a2365 = v0;
    if if nat_le(a2364, a2365) {
        let a2366 = v0;
        let a2367 = 90u64;
        nat_le(a2366, a2367)
    } else {
        false
    } {
        true
    } else {
        let a2368 = 97u64;
        let a2369 = v0;
        if nat_le(a2368, a2369) {
            let a2370 = v0;
            let a2371 = 122u64;
            nat_le(a2370, a2371)
        } else {
            false
        }
    }
}

pub fn f350(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h2372, _) = f188(Fn5::F450(v1, v0.clone()), {
        let a2374 = {
            let a2373 = v0;
            length_bytes(a2373)
        };
        let a2375 = 1u64;
        nat_add(a2374, a2375)?
    }, v2)?;
    Ok(h2372)
}

pub fn f351(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f129(v0.clone(), v1)?;
    let a2376 = v2;
    let a2377 = 48u64;
    if nat_eq(a2376, a2377) {
        let v3: u64 = f129(v0.clone(), {
            let a2378 = v1;
            let a2379 = 1u64;
            nat_add(a2378, a2379)?
        })?;
        let a2380 = v3;
        let a2381 = 88u64;
        if if if nat_eq(a2380, a2381) {
            true
        } else {
            let a2382 = v3;
            let a2383 = 120u64;
            nat_eq(a2382, a2383)
        } {
            f451(f129(v0.clone(), {
                let a2384 = v1;
                let a2385 = 2u64;
                nat_add(a2384, a2385)?
            })?)
        } else {
            false
        } {
            f350(v0, 1u64, {
                let a2386 = v1;
                let a2387 = 2u64;
                nat_add(a2386, a2387)?
            })
        } else {
            f350(v0, 2u64, {
                let a2388 = v1;
                let a2389 = 1u64;
                nat_add(a2388, a2389)?
            })
        }
    } else {
        if f452(v2) {
            f350(v0, 0u64, v1)
        } else {
            Ok(v1)
        }
    }
}

pub fn f352(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f350(v0.clone(), 0u64, v1)?;
    let a2390 = f129(v0.clone(), v2)?;
    let a2391 = 46u64;
    if nat_eq(a2390, a2391) {
        let v3: u64 = f350(v0.clone(), 0u64, {
            let a2392 = v2;
            let a2393 = 1u64;
            nat_add(a2392, a2393)?
        })?;
        let a2394 = v1;
        let a2395 = v2;
        if if nat_lt(a2394, a2395) {
            true
        } else {
            let a2398 = {
                let a2396 = v2;
                let a2397 = 1u64;
                nat_add(a2396, a2397)?
            };
            let a2399 = v3;
            nat_lt(a2398, a2399)
        } {
            f453(v0, v3)
        } else {
            Ok(v1)
        }
    } else {
        let a2400 = v1;
        let a2401 = v2;
        if nat_lt(a2400, a2401) {
            let v4: u64 = f453(v0, v2)?;
            let a2402 = v2;
            let a2403 = v4;
            if nat_lt(a2402, a2403) {
                Ok(v4)
            } else {
                Ok(v1)
            }
        } else {
            Ok(v1)
        }
    }
}

pub fn f353(v0: List<Adt23>, v1: Str) -> bool {
    let m2404 = v0;
    match m2404.uncons() {
        None => {
            false
        }
        Some((v2, _)) => {
            f454(v2, {
                let a2405 = v1;
                utf8_encode(a2405)
            })
        }
    }
}

pub fn f354(p2418: u64, p2419: List<Adt23>, p2420: List<Bytes>, v3: bool) -> Result<List<Adt23>, Bytes> {
    let m2417 = (p2418, (p2419, p2420));
    {
        let mut m2417 = m2417;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2417;
                let m2406 = v0;
                if m2406 == 0 {
                    Err::<(u64, (List<Adt23>, List<Bytes>)), Result<List<Adt23>, Bytes>>(Err::<List<Adt23>, Bytes>(f8(Str::lit("brackets nest deeper than the input is long"))))
                } else {
                    let v4 = m2406 - 1;
                    let m2407 = v1;
                    match m2407.uncons() {
                        None => {
                            Err::<(u64, (List<Adt23>, List<Bytes>)), Result<List<Adt23>, Bytes>>(Err::<List<Adt23>, Bytes>(f8(Str::lit("the input ends inside brackets or before the ; that closes a member (Web IDL, IDL grammar)"))))
                        }
                        Some((v5, v6)) => {
                            let a2408 = f455(v5.clone());
                            let a2409 = 3u64;
                            if nat_eq(a2408, a2409) {
                                Ok::<(u64, (List<Adt23>, List<Bytes>)), Result<List<Adt23>, Bytes>>((v4, (v6, v2)))
                            } else {
                                let v7: Bytes = f456(v5);
                                let v8: Bytes = f457(v7.clone());
                                if !f32(v8.clone(), Bytes::lit(&[])) {
                                    Ok::<(u64, (List<Adt23>, List<Bytes>)), Result<List<Adt23>, Bytes>>((v4, (v6, List::cons(v8, v2))))
                                } else {
                                    if f458(v7.clone()) {
                                        let m2410 = v2;
                                        match m2410.uncons() {
                                            None => {
                                                Err::<(u64, (List<Adt23>, List<Bytes>)), Result<List<Adt23>, Bytes>>(Err::<List<Adt23>, Bytes>(f7(List::cons(f8(Str::lit("the closing bracket ")), List::cons(v7, List::cons(f8(Str::lit(" has no opening bracket (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
                                            }
                                            Some((v9, v10)) => {
                                                if f32(v9.clone(), v7.clone()) {
                                                    if if !v3 {
                                                        let a2412 = {
                                                            let a2411 = v10.clone();
                                                            length_list(a2411)
                                                        };
                                                        let a2413 = 0u64;
                                                        nat_eq(a2412, a2413)
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
                                                let a2415 = {
                                                    let a2414 = v2.clone();
                                                    length_list(a2414)
                                                };
                                                let a2416 = 0u64;
                                                nat_eq(a2415, a2416)
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
                Ok(next) => { m2417 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f355(v0: List<Adt23>) -> List<Adt23> {
    let m2421 = v0;
    match m2421.uncons() {
        None => {
            List::<Adt23>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f356(v0: u64, v1: u64, v2: List<Adt23>, v3: bool) -> R<Result<(Adt14, List<Adt23>), Bytes>> {
    let m2422 = f359(v2);
    match m2422 {
        Ok(v5) => {
            let v6: bool = if v3 {
                f353(f361(v5.clone()), Str::lit(":"))
            } else {
                false
            };
            let m2423 = if v6 {
                f359(f355(f361(v5.clone())))
            } else {
                Ok::<Adt32, Bytes>(Adt32::C0(Bytes::lit(&[]), f361(v5.clone())))
            };
            match m2423 {
                Ok(v8) => {
                    let m2424 = f360(f361(v8.clone()), Str::lit("{"));
                    match m2424 {
                        Ok(v10) => {
                            let m2429 = {
                                let a2425 = v1;
                                let a2426 = 2u64;
                                if if nat_eq(a2425, a2426) {
                                    true
                                } else {
                                    let a2427 = v1;
                                    let a2428 = 7u64;
                                    nat_eq(a2427, a2428)
                                } {
                                    f459(v0, v10, List::<Adt15>::nil())?
                                } else {
                                    f460(v0, v10, List::<Adt15>::nil())?
                                }
                            };
                            match m2429 {
                                Ok(v12) => {
                                    Ok(Ok::<(Adt14, List<Adt23>), Bytes>((Adt14::C0(v1, f363(v5), f363(v8), {
                                        let (h2430, _) = v12;
                                        h2430
                                    }, List::<Bytes>::nil(), f364(Bytes::lit(&[]))), {
                                        let (_, h2431) = v12;
                                        h2431
                                    })))
                                }
                                Err(v11) => {
                                    Ok(Err::<(Adt14, List<Adt23>), Bytes>(v11))
                                }
                            }
                        }
                        Err(v9) => {
                            Ok(Err::<(Adt14, List<Adt23>), Bytes>(v9))
                        }
                    }
                }
                Err(v7) => {
                    Ok(Err::<(Adt14, List<Adt23>), Bytes>(v7))
                }
            }
        }
        Err(v4) => {
            Ok(Err::<(Adt14, List<Adt23>), Bytes>(v4))
        }
    }
}

pub fn f357(v0: List<Adt23>, v1: Bytes) -> R<Result<(Adt14, List<Adt23>), Bytes>> {
    let m2432 = f359(v0);
    match m2432 {
        Ok(v3) => {
            let m2433 = f461(f361(v3.clone()))?;
            match m2433 {
                Ok(v5) => {
                    Ok(Ok::<(Adt14, List<Adt23>), Bytes>((Adt14::C0(9u64, f363(v3), Bytes::lit(&[]), List::<Adt15>::nil(), List::cons(v1, List::<Bytes>::nil()), f364(Bytes::lit(&[]))), v5)))
                }
                Err(v4) => {
                    Ok(Err::<(Adt14, List<Adt23>), Bytes>(v4))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<(Adt14, List<Adt23>), Bytes>(v2))
        }
    }
}

pub fn f358(v0: Str, v1: List<Adt23>) -> Bytes {
    f7(List::cons(f8(Str::lit("expected ")), List::cons({
        let a2434 = v0;
        utf8_encode(a2434)
    }, List::cons(f8(Str::lit(" but found ")), List::cons(f462(v1), List::cons(f8(Str::lit(" (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
}

pub fn f359(v0: List<Adt23>) -> Result<Adt32, Bytes> {
    let m2435 = v0.clone();
    match m2435.uncons() {
        None => {
            Err::<Adt32, Bytes>(f358(Str::lit("an identifier"), v0))
        }
        Some((v1, v2)) => {
            let a2436 = f455(v1.clone());
            let a2437 = 0u64;
            if nat_eq(a2436, a2437) {
                Ok::<Adt32, Bytes>(Adt32::C0(f463(f456(v1)), v2))
            } else {
                Err::<Adt32, Bytes>(f358(Str::lit("an identifier"), v0))
            }
        }
    }
}

pub fn f360(v0: List<Adt23>, v1: Str) -> Result<List<Adt23>, Bytes> {
    if f353(v0.clone(), v1.clone()) {
        Ok::<List<Adt23>, Bytes>(f355(v0))
    } else {
        Err::<List<Adt23>, Bytes>(f358(v1, v0))
    }
}

pub fn f361(v0: Adt32) -> List<Adt23> {
    let m2438 = v0;
    match m2438 {
        Adt32::C0(_, v2) => {
            v2
        }
    }
}

pub fn f362(p2446: u64, p2447: List<Adt23>, p2448: List<Bytes>) -> Result<(List<Bytes>, List<Adt23>), Bytes> {
    let m2445 = (p2446, (p2447, p2448));
    {
        let mut m2445 = m2445;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2445;
                let m2439 = v0;
                if m2439 == 0 {
                    Err::<(u64, (List<Adt23>, List<Bytes>)), Result<(List<Bytes>, List<Adt23>), Bytes>>(Err::<(List<Bytes>, List<Adt23>), Bytes>(f8(Str::lit("an enumeration has more values than the input has tokens"))))
                } else {
                    let v3 = m2439 - 1;
                    let m2440 = v1.clone();
                    match m2440.uncons() {
                        None => {
                            Err::<(u64, (List<Adt23>, List<Bytes>)), Result<(List<Bytes>, List<Adt23>), Bytes>>(Err::<(List<Bytes>, List<Adt23>), Bytes>(f358(Str::lit("a string"), v1)))
                        }
                        Some((v4, v5)) => {
                            let a2441 = f455(v4.clone());
                            let a2442 = 3u64;
                            if !nat_eq(a2441, a2442) {
                                Err::<(u64, (List<Adt23>, List<Bytes>)), Result<(List<Bytes>, List<Adt23>), Bytes>>(Err::<(List<Bytes>, List<Adt23>), Bytes>(f358(Str::lit("a string"), v1)))
                            } else {
                                if f353(v5.clone(), Str::lit(",")) {
                                    if f353(f355(v5.clone()), Str::lit("}")) {
                                        let m2443 = f360(f355(f355(v5)), Str::lit(";"));
                                        match m2443 {
                                            Ok(v7) => {
                                                Err::<(u64, (List<Adt23>, List<Bytes>)), Result<(List<Bytes>, List<Adt23>), Bytes>>(Ok::<(List<Bytes>, List<Adt23>), Bytes>((f62(List::cons(f456(v4), v2)), v7)))
                                            }
                                            Err(v6) => {
                                                Err::<(u64, (List<Adt23>, List<Bytes>)), Result<(List<Bytes>, List<Adt23>), Bytes>>(Err::<(List<Bytes>, List<Adt23>), Bytes>(v6))
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (List<Adt23>, List<Bytes>)), Result<(List<Bytes>, List<Adt23>), Bytes>>((v3, (f355(v5), List::cons(f456(v4), v2))))
                                    }
                                } else {
                                    if f353(v5.clone(), Str::lit("}")) {
                                        let m2444 = f360(f355(v5), Str::lit(";"));
                                        match m2444 {
                                            Ok(v9) => {
                                                Err::<(u64, (List<Adt23>, List<Bytes>)), Result<(List<Bytes>, List<Adt23>), Bytes>>(Ok::<(List<Bytes>, List<Adt23>), Bytes>((f62(List::cons(f456(v4), v2)), v9)))
                                            }
                                            Err(v8) => {
                                                Err::<(u64, (List<Adt23>, List<Bytes>)), Result<(List<Bytes>, List<Adt23>), Bytes>>(Err::<(List<Bytes>, List<Adt23>), Bytes>(v8))
                                            }
                                        }
                                    } else {
                                        Err::<(u64, (List<Adt23>, List<Bytes>)), Result<(List<Bytes>, List<Adt23>), Bytes>>(Err::<(List<Bytes>, List<Adt23>), Bytes>(f358(Str::lit(", or }"), v5)))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2445 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f363(v0: Adt32) -> Bytes {
    let m2449 = v0;
    match m2449 {
        Adt32::C0(v1, _) => {
            v1
        }
    }
}

pub fn f364(v0: Bytes) -> Adt16 {
    Adt16::C0(0u64, v0, List::<Bytes>::nil(), Bytes::lit(&[]), Bytes::lit(&[]))
}

pub fn f365(v0: u64, v1: List<Adt23>, v2: u64) -> R<Result<Adt33, Bytes>> {
    let m2450 = v0;
    if m2450 == 0 {
        Ok(Err::<Adt33, Bytes>(f8(Str::lit("a type nests deeper than the input is long"))))
    } else {
        let v3 = m2450 - 1;
        let a2451 = v2;
        let a2452 = 0u64;
        if nat_eq(a2451, a2452) {
            let m2453 = f250(v1)?;
            match m2453 {
                Ok(v5) => {
                    if f353(v5.clone(), Str::lit("(")) {
                        let m2454 = f365(v3, f355(v5), 1u64)?;
                        match m2454 {
                            Ok(v7) => {
                                if f353(f366(v7.clone()), Str::lit("?")) {
                                    Ok(Ok::<Adt33, Bytes>(Adt33::C0(Adt16::C0(0u64, Bytes::lit(&[]), f269(f367(v7.clone())), f8(Str::lit("a nullable union type is not translated yet, because the typedef identifier names only the union (Web IDL 2.13.32)")), Bytes::lit(&[])), f355(f366(v7)))))
                                } else {
                                    Ok(Ok::<Adt33, Bytes>(Adt33::C0(f367(v7.clone()), f366(v7))))
                                }
                            }
                            Err(v6) => {
                                Ok(Err::<Adt33, Bytes>(v6))
                            }
                        }
                    } else {
                        let m2455 = v5.clone();
                        match m2455.uncons() {
                            None => {
                                Ok(Err::<Adt33, Bytes>(f358(Str::lit("a type"), v5)))
                            }
                            Some((v8, v9)) => {
                                let v10: Bytes = f456(v8.clone());
                                let a2456 = f455(v8);
                                let a2457 = 0u64;
                                if !nat_eq(a2456, a2457) {
                                    Ok(Err::<Adt33, Bytes>(f358(Str::lit("a type"), v5)))
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
                                        let m2458 = f360(v9, Str::lit("<"));
                                        match m2458 {
                                            Ok(v12) => {
                                                let m2459 = f365(v3, v12, 0u64)?;
                                                match m2459 {
                                                    Ok(v14) => {
                                                        let m2460 = f360(f366(v14.clone()), Str::lit(">"));
                                                        match m2460 {
                                                            Ok(v16) => {
                                                                Ok(f464(f465(Str::lit("List"), f367(v14)), v16))
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
                                            let m2461 = f360(v9, Str::lit("<"));
                                            match m2461 {
                                                Ok(v18) => {
                                                    let m2462 = f365(v3, v18, 0u64)?;
                                                    match m2462 {
                                                        Ok(v20) => {
                                                            let m2463 = f360(f366(v20), Str::lit(">"));
                                                            match m2463 {
                                                                Ok(v22) => {
                                                                    Ok(f464(f466(f8(Str::lit("async_sequence types are not translated, because an async sequence is produced over time and is not a value (Web IDL 2.13.29)"))), v22))
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
                                                let m2464 = f360(v9, Str::lit("<"));
                                                match m2464 {
                                                    Ok(v24) => {
                                                        let m2465 = f365(v3, v24, 0u64)?;
                                                        match m2465 {
                                                            Ok(v26) => {
                                                                let m2466 = f360(f366(v26), Str::lit(">"));
                                                                match m2466 {
                                                                    Ok(v28) => {
                                                                        Ok(f464(f466(f8(Str::lit("Promise types are not translated, because a promise is a deferred result, not a value (Web IDL 2.13.31)"))), v28))
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
                                                    let m2467 = f360(v9, Str::lit("<"));
                                                    match m2467 {
                                                        Ok(v30) => {
                                                            let m2468 = v30.clone();
                                                            match m2468.uncons() {
                                                                None => {
                                                                    Ok(Err::<Adt33, Bytes>(f358(Str::lit("a string type"), v30)))
                                                                }
                                                                Some((v31, v32)) => {
                                                                    if if f32(f456(v31.clone()), f8(Str::lit("DOMString"))) {
                                                                        true
                                                                    } else {
                                                                        if f32(f456(v31.clone()), f8(Str::lit("USVString"))) {
                                                                            true
                                                                        } else {
                                                                            f32(f456(v31.clone()), f8(Str::lit("ByteString")))
                                                                        }
                                                                    } {
                                                                        let m2469 = f360(v32, Str::lit(","));
                                                                        match m2469 {
                                                                            Ok(v34) => {
                                                                                let m2470 = f365(v3, v34, 0u64)?;
                                                                                match m2470 {
                                                                                    Ok(v36) => {
                                                                                        let m2471 = f360(f366(v36.clone()), Str::lit(">"));
                                                                                        match m2471 {
                                                                                            Ok(v38) => {
                                                                                                let m2472 = f467(f456(v31));
                                                                                                match m2472 {
                                                                                                    None => {
                                                                                                        Ok(Err::<Adt33, Bytes>(f358(Str::lit("a string type"), v30)))
                                                                                                    }
                                                                                                    Some(v39) => {
                                                                                                        let v40: Adt16 = f468(f367(v36));
                                                                                                        Ok(f464(Adt16::C0(0u64, f7(List::cons(f8(Str::lit("(List (")), List::cons(f268(v39), List::cons(f8(Str::lit(" * ")), List::cons(f268(v40.clone()), List::cons(f8(Str::lit("))")), List::<Bytes>::nil())))))), f269(v40.clone()), f266(v40), Bytes::lit(&[])), v38))
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
                                                                        Ok(Err::<Adt33, Bytes>(f358(Str::lit("DOMString, USVString or ByteString"), v30)))
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
                                                        if f353(v9.clone(), Str::lit("short")) {
                                                            Ok(f464(f364(f8(Str::lit("Nat"))), f355(v9)))
                                                        } else {
                                                            if f353(v9.clone(), Str::lit("long")) {
                                                                let v41: List<Adt23> = f355(v9);
                                                                Ok(f464(f364(f8(Str::lit("Nat"))), if f353(v41.clone(), Str::lit("long")) {
                                                                    f355(v41)
                                                                } else {
                                                                    v41
                                                                }))
                                                            } else {
                                                                Ok(Err::<Adt33, Bytes>(f358(Str::lit("short or long"), v9)))
                                                            }
                                                        }
                                                    } else {
                                                        if f32(v10.clone(), f8(Str::lit("short"))) {
                                                            Ok(f464(f364(f8(Str::lit("Int"))), v9))
                                                        } else {
                                                            if f32(v10.clone(), f8(Str::lit("long"))) {
                                                                Ok(f464(f364(f8(Str::lit("Int"))), if f353(v9.clone(), Str::lit("long")) {
                                                                    f355(v9)
                                                                } else {
                                                                    v9
                                                                }))
                                                            } else {
                                                                if f32(v10.clone(), f8(Str::lit("unrestricted"))) {
                                                                    if if f353(v9.clone(), Str::lit("float")) {
                                                                        true
                                                                    } else {
                                                                        f353(v9.clone(), Str::lit("double"))
                                                                    } {
                                                                        Ok(f464(f466(f8(Str::lit("unrestricted float and unrestricted double are not translated, because its value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.13 and 2.13.15)"))), f355(v9)))
                                                                    } else {
                                                                        Ok(Err::<Adt33, Bytes>(f358(Str::lit("float or double"), v9)))
                                                                    }
                                                                } else {
                                                                    let m2473 = f467(v10.clone());
                                                                    match m2473 {
                                                                        None => {
                                                                            let v42: Bytes = f463(v10);
                                                                            Ok(f464(Adt16::C0(0u64, f253(v42.clone())?, List::cons(v42.clone(), List::<Bytes>::nil()), Bytes::lit(&[]), v42), v9))
                                                                        }
                                                                        Some(v43) => {
                                                                            Ok(f464(v43, v9))
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
            let m2474 = f365(v3, v1, 0u64)?;
            match m2474 {
                Ok(v45) => {
                    let v46: Adt16 = f367(v45.clone());
                    let v48: Adt16 = {
                        let a2475 = f267(v46.clone());
                        let a2476 = 0u64;
                        if nat_eq(a2475, a2476) {
                            if f32(f266(v46.clone()), Bytes::lit(&[])) {
                                if f32(f469(v46.clone()), Bytes::lit(&[])) {
                                    Adt16::C0(1u64, Bytes::lit(&[]), f269(v46), f8(Str::lit("a union member type that is not a named definition gives its constructor no name, so the union is not translated yet (Web IDL 2.13.32)")), Bytes::lit(&[]))
                                } else {
                                    let m2477 = f377(false, f469(v46.clone()))?;
                                    match m2477 {
                                        None => {
                                            Adt16::C0(1u64, Bytes::lit(&[]), f269(v46.clone()), f7(List::cons(f8(Str::lit("the union member ")), List::cons(f469(v46), List::cons(f8(Str::lit(" has no constructor name")), List::<Bytes>::nil())))), Bytes::lit(&[]))
                                        }
                                        Some(v47) => {
                                            Adt16::C0(1u64, f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v47, List::cons(f8(Str::lit(", fields: [")), List::cons(f268(v46.clone()), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f269(v46), Bytes::lit(&[]), Bytes::lit(&[]))
                                        }
                                    }
                                }
                            } else {
                                Adt16::C0(1u64, Bytes::lit(&[]), f269(v46.clone()), f266(v46), Bytes::lit(&[]))
                            }
                        } else {
                            v46
                        }
                    };
                    if f353(f366(v45.clone()), Str::lit("or")) {
                        let m2478 = f365(v3, f355(f366(v45)), 2u64)?;
                        match m2478 {
                            Ok(v50) => {
                                Ok(Ok::<Adt33, Bytes>(Adt33::C0(Adt16::C0(1u64, if f32(f268(v48.clone()), Bytes::lit(&[])) {
                                    f268(f367(v50.clone()))
                                } else {
                                    if f32(f268(f367(v50.clone())), Bytes::lit(&[])) {
                                        f268(v48.clone())
                                    } else {
                                        f7(List::cons(f268(v48.clone()), List::cons(f8(Str::lit(",\u{a}")), List::cons(f268(f367(v50.clone())), List::<Bytes>::nil()))))
                                    }
                                }, f176(f269(v48.clone()), f269(f367(v50.clone()))), if f32(f266(v48.clone()), Bytes::lit(&[])) {
                                    f266(f367(v50.clone()))
                                } else {
                                    f266(v48)
                                }, Bytes::lit(&[])), f366(v50))))
                            }
                            Err(v49) => {
                                Ok(Err::<Adt33, Bytes>(v49))
                            }
                        }
                    } else {
                        let a2479 = v2;
                        let a2480 = 2u64;
                        if if nat_eq(a2479, a2480) {
                            f353(f366(v45.clone()), Str::lit(")"))
                        } else {
                            false
                        } {
                            Ok(Ok::<Adt33, Bytes>(Adt33::C0(v48, f355(f366(v45)))))
                        } else {
                            Ok(Err::<Adt33, Bytes>({
                                let a2481 = v2;
                                let a2482 = 1u64;
                                if nat_eq(a2481, a2482) {
                                    f358(Str::lit("or"), f366(v45))
                                } else {
                                    f358(Str::lit("or or )"), f366(v45))
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

pub fn f366(v0: Adt33) -> List<Adt23> {
    let m2483 = v0;
    match m2483 {
        Adt33::C0(_, v2) => {
            v2
        }
    }
}

pub fn f367(v0: Adt33) -> Adt16 {
    let m2484 = v0;
    match m2484 {
        Adt33::C0(v1, _) => {
            v1
        }
    }
}

pub fn f368() -> List<Bytes> {
    List::cons(f8(Str::lit("Bool")), List::cons(f8(Str::lit("ContractViolation")), List::cons(f8(Str::lit("List")), List::cons(f8(Str::lit("Nat")), List::cons(f8(Str::lit("Option")), List::cons(f8(Str::lit("Prod")), List::cons(f8(Str::lit("ReasoningFailure")), List::cons(f8(Str::lit("Result")), List::<Bytes>::nil()))))))))
}

pub fn f369(v0: Bytes) -> R<Bytes> {
    let m2487 = {
        let a2485 = v0.clone();
        let a2486 = 0u64;
        index_bytes(a2485, a2486)
    };
    match m2487 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            let a2493 = {
                let a2488 = f8(Str::lit("u"));
                let a2489 = f470(8u64, f56(v1)?, 4u64, Bytes::lit(&[]));
                append_bytes(a2488, a2489)
            };
            let a2494 = f45(v0.clone(), 1u64, {
                let a2491 = {
                    let a2490 = v0;
                    length_bytes(a2490)
                };
                let a2492 = 1u64;
                nat_sub(a2491, a2492)
            });
            Ok(append_bytes(a2493, a2494))
        }
    }
}

pub fn f370(v0: List<Adt15>) -> R<List<Adt15>> {
    let m2495 = v0;
    match m2495.uncons() {
        None => {
            Ok(List::<Adt15>::nil())
        }
        Some((v1, v2)) => {
            f471(v1, f370(v2)?)
        }
    }
}

pub fn f371(v0: List<Adt15>, v1: List<Adt15>) -> List<Adt15> {
    let m2496 = v0;
    match m2496.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f371(v3, v1))
        }
    }
}

pub fn f372(v0: Adt14) -> List<Adt15> {
    let m2497 = v0;
    match m2497 {
        Adt14::C0(_, _, _, v4, _, _) => {
            v4
        }
    }
}

pub fn f373(v0: List<Adt14>, v1: u64, v2: Bytes) -> List<Adt15> {
    let m2498 = v0;
    match m2498.uncons() {
        None => {
            List::<Adt15>::nil()
        }
        Some((v3, v4)) => {
            let a2499 = f170(v3.clone());
            let a2500 = v1;
            if if nat_eq(a2499, a2500) {
                f32(f172(v3.clone()), v2.clone())
            } else {
                false
            } {
                f371(f372(v3), f373(v4, v1, v2))
            } else {
                f373(v4, v1, v2)
            }
        }
    }
}

pub fn f374(v0: List<Adt14>, v1: List<Adt14>, v2: Bytes) -> List<Adt15> {
    let m2501 = v0;
    match m2501.uncons() {
        None => {
            List::<Adt15>::nil()
        }
        Some((v3, v4)) => {
            let a2502 = f170(v3.clone());
            let a2503 = 8u64;
            if if nat_eq(a2502, a2503) {
                f32(f172(v3.clone()), v2.clone())
            } else {
                false
            } {
                f371(f472(v1.clone(), f183(v3)), f374(v4, v1, v2))
            } else {
                f374(v4, v1, v2)
            }
        }
    }
}

pub fn f375(v0: Adt15) -> Adt16 {
    let m2504 = v0;
    match m2504 {
        Adt15::C0(_, v2, _) => {
            v2
        }
    }
}

pub fn f376(v0: Adt15) -> Bytes {
    let m2505 = v0;
    match m2505 {
        Adt15::C0(v1, _, _) => {
            v1
        }
    }
}

pub fn f377(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f473(v1)
    } else {
        f380(v1)
    }
}

pub fn f378(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m2506 = v1;
    match m2506.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m2507 = f377(v0, v2)?;
            match m2507 {
                None => {
                    f378(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f378(v0, v3)?))
                }
            }
        }
    }
}

pub fn f379(v0: Adt15) -> u64 {
    let m2508 = v0;
    match m2508 {
        Adt15::C0(_, _, v3) => {
            v3
        }
    }
}

pub fn f380(v0: Bytes) -> R<Option<Bytes>> {
    let m2509 = f474(v0)?;
    match m2509 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            Ok(Some(f475(v1)?))
        }
    }
}

pub fn f381(v0: bool, v1: Adt2) -> R<Option<Bytes>> {
    let m2510 = v1;
    match m2510 {
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
                let a2521 = if f96(v7, {
                    let a2519 = 0u64;
                    let a2520 = v8;
                    nat_lt(a2519, a2520)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a2522 = f329(false, v8)?;
                append_bytes(a2521, a2522)
            }))
        }
        Adt2::C4(v3, v4, v5) => {
            let m2513 = f473({
                let a2511 = f8(Str::lit("n"));
                let a2512 = f330(false, v4, v5)?;
                append_bytes(a2511, a2512)
            })?;
            match m2513 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a2517 = f8(Str::lit("nMinus"));
                        let a2518 = f45(v6.clone(), 1u64, {
                            let a2515 = {
                                let a2514 = v6;
                                length_bytes(a2514)
                            };
                            let a2516 = 1u64;
                            nat_sub(a2515, a2516)
                        });
                        append_bytes(a2517, a2518)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt2::C5(v2) => {
            f377(v0, v2)
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

pub fn f382(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m2525 = {
        let a2523 = v0.clone();
        let a2524 = v1;
        index_bytes(a2523, a2524)
    };
    match m2525 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f56(v2)?;
            let a2526 = v3;
            let a2527 = 128u64;
            if nat_lt(a2526, a2527) {
                Ok(Some((v3, 1u64)))
            } else {
                if f96({
                    let a2528 = 194u64;
                    let a2529 = v3;
                    nat_le(a2528, a2529)
                }, {
                    let a2530 = v3;
                    let a2531 = 223u64;
                    nat_le(a2530, a2531)
                }) {
                    let m2534 = f476(v0, {
                        let a2532 = v1;
                        let a2533 = 1u64;
                        nat_add(a2532, a2533)?
                    })?;
                    match m2534 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f477({
                                let a2539 = {
                                    let a2537 = {
                                        let a2535 = v3;
                                        let a2536 = 192u64;
                                        nat_sub(a2535, a2536)
                                    };
                                    let a2538 = 64u64;
                                    nat_mul(a2537, a2538)?
                                };
                                let a2540 = v4;
                                nat_add(a2539, a2540)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f96({
                        let a2541 = 224u64;
                        let a2542 = v3;
                        nat_le(a2541, a2542)
                    }, {
                        let a2543 = v3;
                        let a2544 = 239u64;
                        nat_le(a2543, a2544)
                    }) {
                        let m2547 = f476(v0.clone(), {
                            let a2545 = v1;
                            let a2546 = 1u64;
                            nat_add(a2545, a2546)?
                        })?;
                        match m2547 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m2550 = f476(v0, {
                                    let a2548 = v1;
                                    let a2549 = 2u64;
                                    nat_add(a2548, a2549)?
                                })?;
                                match m2550 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f477({
                                            let a2559 = {
                                                let a2553 = {
                                                    let a2551 = v3;
                                                    let a2552 = 224u64;
                                                    nat_sub(a2551, a2552)
                                                };
                                                let a2554 = 4096u64;
                                                nat_mul(a2553, a2554)?
                                            };
                                            let a2560 = {
                                                let a2557 = {
                                                    let a2555 = v5;
                                                    let a2556 = 64u64;
                                                    nat_mul(a2555, a2556)?
                                                };
                                                let a2558 = v6;
                                                nat_add(a2557, a2558)?
                                            };
                                            nat_add(a2559, a2560)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f96({
                            let a2561 = 240u64;
                            let a2562 = v3;
                            nat_le(a2561, a2562)
                        }, {
                            let a2563 = v3;
                            let a2564 = 244u64;
                            nat_le(a2563, a2564)
                        }) {
                            let m2567 = f476(v0.clone(), {
                                let a2565 = v1;
                                let a2566 = 1u64;
                                nat_add(a2565, a2566)?
                            })?;
                            match m2567 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m2570 = f476(v0.clone(), {
                                        let a2568 = v1;
                                        let a2569 = 2u64;
                                        nat_add(a2568, a2569)?
                                    })?;
                                    match m2570 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m2573 = f476(v0, {
                                                let a2571 = v1;
                                                let a2572 = 3u64;
                                                nat_add(a2571, a2572)?
                                            })?;
                                            match m2573 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f477({
                                                        let a2586 = {
                                                            let a2576 = {
                                                                let a2574 = v3;
                                                                let a2575 = 240u64;
                                                                nat_sub(a2574, a2575)
                                                            };
                                                            let a2577 = 262144u64;
                                                            nat_mul(a2576, a2577)?
                                                        };
                                                        let a2587 = {
                                                            let a2584 = {
                                                                let a2578 = v7;
                                                                let a2579 = 4096u64;
                                                                nat_mul(a2578, a2579)?
                                                            };
                                                            let a2585 = {
                                                                let a2582 = {
                                                                    let a2580 = v8;
                                                                    let a2581 = 64u64;
                                                                    nat_mul(a2580, a2581)?
                                                                };
                                                                let a2583 = v9;
                                                                nat_add(a2582, a2583)?
                                                            };
                                                            nat_add(a2584, a2585)?
                                                        };
                                                        nat_add(a2586, a2587)?
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

pub fn f383(v0: u64) -> R<bool> {
    f274(v0, f478())
}

pub fn f384(v0: u64, v1: Bytes, p2598: u64, p2599: u64, p2600: bool) -> R<bool> {
    let m2597 = (p2598, (p2599, p2600));
    {
        let mut m2597 = m2597;
        loop {
            let step = {
                let (v2, (v3, v4)) = m2597;
                let m2588 = v3;
                if m2588 == 0 {
                    Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                } else {
                    let v5 = m2588 - 1;
                    let v6: u64 = f479(v1.clone(), v2)?;
                    if v4 {
                        let a2589 = v0;
                        let a2590 = v6;
                        if nat_lt(a2589, a2590) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a2591 = v2;
                                let a2592 = 1u64;
                                nat_add(a2591, a2592)?
                            }, (v5, false)))
                        }
                    } else {
                        let a2593 = v0;
                        let a2594 = v6;
                        if nat_le(a2593, a2594) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(true))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a2595 = v2;
                                let a2596 = 1u64;
                                nat_add(a2595, a2596)?
                            }, (v5, true)))
                        }
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

pub fn f385(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    if f121(v0.clone(), v1, v2) {
        Ok(None::<u64>)
    } else {
        let a2602 = v2;
        let a2603 = {
            let a2601 = v0;
            length_bytes(a2601)
        };
        if nat_lt(a2602, a2603) {
            Ok(Some({
                let a2604 = v2;
                let a2605 = 1u64;
                nat_add(a2604, a2605)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f386(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f190(f129(v0, v1)?)? {
        Ok(Some({
            let a2606 = v1;
            let a2607 = 1u64;
            nat_add(a2606, a2607)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f387(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let m2608 = f382(v0, v2)?;
    match m2608 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            let a2609 = v2;
            let a2610 = v1;
            if if nat_eq(a2609, a2610) {
                f480({
                    let (h2611, _) = v3;
                    h2611
                })?
            } else {
                f481({
                    let (h2612, _) = v3;
                    h2612
                })?
            } {
                Ok(Some({
                    let a2614 = v2;
                    let a2615 = {
                        let (_, h2613) = v3;
                        h2613
                    };
                    nat_add(a2614, a2615)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f388(v0: Bytes, v1: bool, v2: u64) -> R<(u64, u64)> {
    let (h2616, _) = f240(Fn6::F482(v1, v0.clone()), {
        let a2618 = {
            let a2617 = v0;
            length_bytes(a2617)
        };
        let a2619 = 1u64;
        nat_add(a2618, a2619)?
    }, (v2, 0u64))?;
    Ok(h2616)
}

pub fn f389(v0: u64) -> R<Bytes> {
    let a2620 = v0;
    let a2621 = 128u64;
    if nat_lt(a2620, a2621) {
        Ok(f483(f484(), v0))
    } else {
        let a2622 = v0;
        let a2623 = 2048u64;
        if nat_lt(a2622, a2623) {
            let a2631 = f483(f485(), {
                let a2629 = {
                    let a2627 = {
                        let a2624 = v0;
                        let a2625 = 64u64;
                        let a2626 = 0u64;
                        nat_quot(a2624, a2625, a2626)
                    };
                    let a2628 = 2u64;
                    nat_sub(a2627, a2628)
                };
                let a2630 = 2u64;
                nat_mul(a2629, a2630)?
            });
            let a2632 = f486(v0)?;
            Ok(append_bytes(a2631, a2632))
        } else {
            let a2633 = v0;
            let a2634 = 65536u64;
            if nat_lt(a2633, a2634) {
                let a2645 = f483(f487(), {
                    let a2638 = {
                        let a2635 = v0;
                        let a2636 = 4096u64;
                        let a2637 = 0u64;
                        nat_quot(a2635, a2636, a2637)
                    };
                    let a2639 = 3u64;
                    nat_mul(a2638, a2639)?
                });
                let a2646 = {
                    let a2643 = f486({
                        let a2640 = v0;
                        let a2641 = 64u64;
                        let a2642 = 0u64;
                        nat_quot(a2640, a2641, a2642)
                    })?;
                    let a2644 = f486(v0)?;
                    append_bytes(a2643, a2644)
                };
                Ok(append_bytes(a2645, a2646))
            } else {
                let a2660 = f483(Bytes::lit(&[0xf0, 0xf1, 0xf2, 0xf3, 0xf4]), {
                    let a2647 = v0;
                    let a2648 = 262144u64;
                    let a2649 = 0u64;
                    nat_quot(a2647, a2648, a2649)
                });
                let a2661 = {
                    let a2658 = f486({
                        let a2650 = v0;
                        let a2651 = 4096u64;
                        let a2652 = 0u64;
                        nat_quot(a2650, a2651, a2652)
                    })?;
                    let a2659 = {
                        let a2656 = f486({
                            let a2653 = v0;
                            let a2654 = 64u64;
                            let a2655 = 0u64;
                            nat_quot(a2653, a2654, a2655)
                        })?;
                        let a2657 = f486(v0)?;
                        append_bytes(a2656, a2657)
                    };
                    append_bytes(a2658, a2659)
                };
                Ok(append_bytes(a2660, a2661))
            }
        }
    }
}

pub fn f390(v0: bool, v1: u64, v2: Bytes, v3: u64) -> R<Option<u64>> {
    f488(v2, v0, v1, v3)
}

pub fn f391(v0: Bytes) -> Result<Option<Adt31>, Bytes> {
    Err::<Option<Adt31>, Bytes>(v0)
}

pub fn f392(p2665: Bytes, p2666: u64) -> R<Result<Bytes, Bytes>> {
    let m2664 = (p2665, p2666);
    {
        let mut m2664 = m2664;
        loop {
            let step = {
                let (v0, v1) = m2664;
                let m2662 = v1;
                if m2662 == 0 {
                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f489(f8(Str::lit("the built-in datatype hierarchy is deeper than XSD 1.1 Part 2 section 3 defines")))))
                } else {
                    let v2 = m2662 - 1;
                    if f278(v0.clone(), List::cons(f8(Str::lit("NMTOKENS")), List::cons(f8(Str::lit("IDREFS")), List::cons(f8(Str::lit("ENTITIES")), List::<Bytes>::nil())))) {
                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("(List Bytes)")))))
                    } else {
                        if f32(v0.clone(), f8(Str::lit("nonNegativeInteger"))) {
                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Nat")))))
                        } else {
                            if f32(v0.clone(), f8(Str::lit("integer"))) {
                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Int")))))
                            } else {
                                if f32(v0.clone(), f8(Str::lit("decimal"))) {
                                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f490())))
                                } else {
                                    if f32(v0.clone(), f8(Str::lit("boolean"))) {
                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Bool")))))
                                    } else {
                                        if f278(v0.clone(), List::cons(f8(Str::lit("float")), List::cons(f8(Str::lit("double")), List::<Bytes>::nil()))) {
                                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f489(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because its value space holds positive and negative zero, the infinities and NaN, which no exact LexLean number type holds (XSD 1.1 Part 2, sections 3.3.4 and 3.3.5)")), List::<Bytes>::nil())))))))
                                        } else {
                                            if f278(v0.clone(), List::cons(f8(Str::lit("anySimpleType")), List::cons(f8(Str::lit("anyAtomicType")), List::<Bytes>::nil()))) {
                                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f489(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because it is the union of every simple value space and has no one LexLean type (XSD 1.1 Part 2, sections 3.2.1 and 3.2.2)")), List::<Bytes>::nil())))))))
                                            } else {
                                                let m2663 = f491(v0.clone())?;
                                                match m2663 {
                                                    None => {
                                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f489(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not a built-in datatype of XSD 1.1 Part 2")), List::<Bytes>::nil())))))))
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
                Ok(next) => { m2664 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f393(v0: Bytes) -> Result<List<Adt2>, Bytes> {
    Err::<List<Adt2>, Bytes>(v0)
}

pub fn f394(v0: Option<Adt2>) -> Option<Bytes> {
    let m2667 = v0;
    match m2667 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m2668 = v1;
            match m2668 {
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

pub fn f395(v0: u64) -> bool {
    f153({
        let a2669 = v0;
        let a2670 = 95u64;
        nat_eq(a2669, a2670)
    }, f153({
        let a2671 = v0;
        let a2672 = 45u64;
        nat_eq(a2671, a2672)
    }, f153({
        let a2673 = v0;
        let a2674 = 32u64;
        nat_eq(a2673, a2674)
    }, f153({
        let a2675 = v0;
        let a2676 = 46u64;
        nat_eq(a2675, a2676)
    }, {
        let a2677 = v0;
        let a2678 = 47u64;
        nat_eq(a2677, a2678)
    }))))
}

pub fn f396(v0: u64) -> bool {
    f96({
        let a2679 = 97u64;
        let a2680 = v0;
        nat_le(a2679, a2680)
    }, {
        let a2681 = v0;
        let a2682 = 122u64;
        nat_le(a2681, a2682)
    })
}

pub fn f397(v0: u64) -> bool {
    f96({
        let a2683 = 65u64;
        let a2684 = v0;
        nat_le(a2683, a2684)
    }, {
        let a2685 = v0;
        let a2686 = 90u64;
        nat_le(a2685, a2686)
    })
}

pub fn f398(v0: u64) -> bool {
    f96({
        let a2687 = 48u64;
        let a2688 = v0;
        nat_le(a2687, a2688)
    }, {
        let a2689 = v0;
        let a2690 = 57u64;
        nat_le(a2689, a2690)
    })
}

pub fn f399(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h2691, _) = v1.clone();
        h2691
    };
    let v3: u64 = {
        let (h2692, _) = {
            let (_, h2693) = v1.clone();
            h2693
        };
        h2692
    };
    let v4: List<Bytes> = {
        let (_, h2694) = {
            let (_, h2695) = v1;
            h2695
        };
        h2694
    };
    let m2696 = f158(v0.clone(), v2);
    match m2696 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a2697 = v5;
            let a2698 = 10u8;
            if equal(a2697, a2698) {
                Ok(Some(({
                    let a2699 = v2;
                    let a2700 = 1u64;
                    nat_add(a2699, a2700)?
                }, ({
                    let a2701 = v2;
                    let a2702 = 1u64;
                    nat_add(a2701, a2702)?
                }, List::cons(f492(f45(v0, v3, {
                    let a2703 = v2;
                    let a2704 = v3;
                    nat_sub(a2703, a2704)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a2705 = v2;
                    let a2706 = 1u64;
                    nat_add(a2705, a2706)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f400(v0: List<Adt26>, v1: List<Adt26>) -> List<Adt26> {
    let m2707 = v0;
    match m2707.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f400(v3, v1))
        }
    }
}

pub fn f401(v0: List<Adt27>, v1: Bytes) -> List<Adt26> {
    let m2708 = v0;
    match m2708.uncons() {
        None => {
            List::<Adt26>::nil()
        }
        Some((v2, v3)) => {
            if f173(match v2.clone() {
                Adt27::C0(_, h2709, _, _, _) => {
                    h2709
                }
            }, f493()) {
                let m2710 = f308(v2.clone(), Str::lit("name"));
                match m2710 {
                    None => {
                        f401(v3, v1)
                    }
                    Some(v4) => {
                        List::cons(Adt26::C0(match v2.clone() {
                            Adt27::C0(_, h2711, _, _, _) => {
                                h2711
                            }
                        }, v1.clone(), v4, v2), f401(v3, v1))
                    }
                }
            } else {
                f401(v3, v1)
            }
        }
    }
}

pub fn f402(v0: u64, v1: Adt25, v2: Adt26) -> R<Result<List<Adt28>, Bytes>> {
    let v3: Bytes = f211(match v2.clone() {
        Adt26::C0(_, _, h2712, _) => {
            h2712
        }
    })?;
    let v4: Adt27 = match v2.clone() {
        Adt26::C0(_, _, _, h2713) => {
            h2713
        }
    };
    let v5: Bytes = match v2.clone() {
        Adt26::C0(_, h2714, _, _) => {
            h2714
        }
    };
    let v6: Bytes = match v2.clone() {
        Adt26::C0(h2715, _, _, _) => {
            h2715
        }
    };
    if f32(v6.clone(), f8(Str::lit("complexType"))) {
        f494(v0, v1, v2)
    } else {
        if f32(v6.clone(), f8(Str::lit("simpleType"))) {
            let m2716 = f495(v3, v4, v5)?;
            match m2716 {
                Ok(v8) => {
                    Ok(Ok::<List<Adt28>, Bytes>(List::cons(v8, List::<Adt28>::nil())))
                }
                Err(v7) => {
                    Ok(Err::<List<Adt28>, Bytes>(v7))
                }
            }
        } else {
            if f32(v6.clone(), f8(Str::lit("element"))) {
                let m2717 = f308(v4.clone(), Str::lit("type"));
                match m2717 {
                    None => {
                        let m2718 = f496(v0, v1, v3, v5, v4)?;
                        match m2718 {
                            Ok(v11) => {
                                Ok(Ok::<List<Adt28>, Bytes>(match v11 {
                                    Adt35::C0(_, h2719) => {
                                        h2719
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
                    let m2720 = f308(v4.clone(), Str::lit("type"));
                    match m2720 {
                        None => {
                            let m2721 = f497(v1, v5, v3, v4)?;
                            match m2721 {
                                Ok(v14) => {
                                    Ok(Ok::<List<Adt28>, Bytes>(match v14 {
                                        Adt35::C0(_, h2722) => {
                                            h2722
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
                        let m2723 = f498(v4)?;
                        match m2723 {
                            Ok(v16) => {
                                let m2724 = f499(v16, List::cons(f8(Str::lit("sequence")), List::cons(f8(Str::lit("choice")), List::cons(f8(Str::lit("all")), List::<Bytes>::nil()))));
                                match m2724 {
                                    None => {
                                        Ok(Err::<List<Adt28>, Bytes>(f500(f8(Str::lit("a model group definition must have a sequence, choice or all child (XSD 1.1 Part 1, section 3.7.2)")), v3)))
                                    }
                                    Some(v17) => {
                                        f501(v0, v1, v3, v5, v17)
                                    }
                                }
                            }
                            Err(v15) => {
                                Ok(Err::<List<Adt28>, Bytes>(v15))
                            }
                        }
                    } else {
                        let m2725 = f498(v4)?;
                        match m2725 {
                            Ok(v19) => {
                                let m2726 = f502(v0, v1, v3, v5, v19, f503())?;
                                match m2726 {
                                    Ok(v21) => {
                                        Ok(Ok::<List<Adt28>, Bytes>(match v21 {
                                            Adt36::C0(_, h2727, _, _, _) => {
                                                h2727
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

pub fn f403(v0: List<Adt28>, v1: List<Adt28>) -> List<Adt28> {
    let m2728 = v0;
    match m2728.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f403(v3, v1))
        }
    }
}

pub fn f404(p2731: List<Adt28>, p2732: List<Adt28>) -> List<Adt28> {
    let m2730 = (p2731, p2732);
    {
        let mut m2730 = m2730;
        loop {
            let step = {
                let (v0, v1) = m2730;
                let m2729 = v0;
                match m2729.uncons() {
                    None => {
                        Err::<(List<Adt28>, List<Adt28>), List<Adt28>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt28>, List<Adt28>), List<Adt28>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2730 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f405(p2736: List<Adt28>, v1: Bytes) -> Option<Adt28> {
    let m2735 = p2736;
    {
        let mut m2735 = m2735;
        loop {
            let step = {
                let v0 = m2735;
                let m2733 = v0;
                match m2733.uncons() {
                    None => {
                        Err::<List<Adt28>, Option<Adt28>>(None::<Adt28>)
                    }
                    Some((v2, v3)) => {
                        if f32(match v2.clone() {
                            Adt28::C0(h2734, _, _, _) => {
                                h2734
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
                Ok(next) => { m2735 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f406() -> Bytes {
    f8(Str::lit("XmlNode"))
}

pub fn f407(v0: List<Adt28>) -> List<Bytes> {
    let m2737 = v0;
    match m2737.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f413(match v1 {
                Adt28::C0(_, h2738, _, _) => {
                    h2738
                }
            }, f407(v2))
        }
    }
}

pub fn f408() -> List<Bytes> {
    List::cons(f8(Str::lit("    {name: element, fields: [Bytes, Bytes, (List ((Bytes * Bytes) * Bytes)), (List XmlNode)]}")), List::cons(f8(Str::lit("    {name: text, fields: [Bytes]}")), List::<Bytes>::nil()))
}

pub fn f409(p2742: u64, v1: List<Adt28>, p2743: List<Bytes>, p2744: List<Bytes>) -> List<Bytes> {
    let m2741 = (p2742, (p2743, p2744));
    {
        let mut m2741 = m2741;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2741;
                let m2739 = v0;
                if m2739 == 0 {
                    Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v3)
                } else {
                    let v4 = m2739 - 1;
                    let m2740 = v2;
                    match m2740.uncons() {
                        None => {
                            Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v3)
                        }
                        Some((v5, v6)) => {
                            if f173(v5.clone(), v3.clone()) {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v3)))
                            } else {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (f413(f410(v1.clone(), v5.clone()), v6), List::cons(v5, v3))))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2741 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f410(v0: List<Adt28>, v1: Bytes) -> List<Bytes> {
    let m2745 = f405(v0, v1);
    match m2745 {
        None => {
            List::<Bytes>::nil()
        }
        Some(v2) => {
            match v2 {
                Adt28::C0(_, h2746, _, _) => {
                    h2746
                }
            }
        }
    }
}

pub fn f411(v0: List<(List<Bytes>, List<Bytes>)>, v1: List<Bytes>) -> Option<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)> {
    let m2747 = v0;
    match m2747.uncons() {
        None => {
            None::<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)>
        }
        Some((v2, v3)) => {
            let m2749 = f212({
                let (_, h2748) = v2.clone();
                h2748
            }, v1.clone());
            match m2749 {
                None => {
                    Some(({
                        let (h2750, _) = v2;
                        h2750
                    }, v3))
                }
                Some(_) => {
                    let m2751 = f411(v3, v1);
                    match m2751 {
                        None => {
                            None::<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h2752, _) = v5;
                                h2752
                            }, List::cons(v2, {
                                let (_, h2753) = v5;
                                h2753
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f412(v0: List<Adt28>, v1: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m2754 = v1.clone();
    match m2754.uncons() {
        None => {
            Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m2755 = v3;
            match m2755.uncons() {
                None => {
                    let m2758 = f405(v0, v2.clone());
                    match m2758 {
                        None => {
                            Err::<List<Bytes>, Bytes>(f505(v2))
                        }
                        Some(v6) => {
                            if f173(v2.clone(), match v6.clone() {
                                Adt28::C0(_, h2759, _, _) => {
                                    h2759
                                }
                            }) {
                                let m2761 = match v6.clone() {
                                    Adt28::C0(_, _, _, h2760) => {
                                        h2760
                                    }
                                };
                                match m2761.uncons() {
                                    None => {
                                        Err::<List<Bytes>, Bytes>(f505(v2))
                                    }
                                    Some((_, _)) => {
                                        Ok::<List<Bytes>, Bytes>(List::cons(f264(v2, match v6 {
                                            Adt28::C0(_, _, _, h2762) => {
                                                h2762
                                            }
                                        }), List::<Bytes>::nil()))
                                    }
                                }
                            } else {
                                Ok::<List<Bytes>, Bytes>(List::cons(match v6 {
                                    Adt28::C0(_, _, h2763, _) => {
                                        h2763
                                    }
                                }, List::<Bytes>::nil()))
                            }
                        }
                    }
                }
                Some((_, _)) => {
                    f504(v0, v1, {
                        let a2756 = v2;
                        let a2757 = f8(Str::lit("Group"));
                        append_bytes(a2756, a2757)
                    })
                }
            }
        }
    }
}

pub fn f413(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m2764 = v0;
    match m2764.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f413(v3, v1))
        }
    }
}

pub fn f414(v0: List<Adt28>, v1: List<Bytes>, v2: List<Bytes>) -> List<Bytes> {
    let m2765 = v1;
    match m2765.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            f413(f506(f410(v0.clone(), v3), v2.clone()), f414(v0, v4, v2))
        }
    }
}

pub fn f415(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>, v2: Bytes) -> List<Bytes> {
    let m2766 = v1;
    match m2766.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if f32(v3.clone(), v2.clone()) {
                true
            } else {
                if f173(v3.clone(), f507(v0.clone(), v2.clone())) {
                    f173(v2.clone(), f507(v0.clone(), v3.clone()))
                } else {
                    false
                }
            } {
                List::cons(v3, f415(v0, v4, v2))
            } else {
                f415(v0, v4, v2)
            }
        }
    }
}

pub fn f416(v0: Bytes, p2775: u64, p2776: u64, p2777: Option<u64>) -> R<Option<u64>> {
    let m2774 = (p2775, (p2776, p2777));
    {
        let mut m2774 = m2774;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2774;
                let m2767 = v2;
                if m2767 == 0 {
                    Err::<(u64, (u64, Option<u64>)), R<Option<u64>>>(Ok(v3))
                } else {
                    let v4 = m2767 - 1;
                    let v5: u64 = f129(v0.clone(), v1)?;
                    let a2768 = v5;
                    let a2769 = 256u64;
                    if nat_eq(a2768, a2769) {
                        Err::<(u64, (u64, Option<u64>)), R<Option<u64>>>(Ok(v3))
                    } else {
                        Ok::<(u64, (u64, Option<u64>)), R<Option<u64>>>(({
                            let a2770 = v1;
                            let a2771 = 1u64;
                            nat_add(a2770, a2771)?
                        }, (v4, {
                            let a2772 = v5;
                            let a2773 = 47u64;
                            if nat_eq(a2772, a2773) {
                                Some(v1)
                            } else {
                                v3
                            }
                        })))
                    }
                }
            };
            match step {
                Ok(next) => { m2774 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f417(v0: u64) -> bool {
    let a2778 = v0;
    let a2779 = 9u64;
    if if if nat_eq(a2778, a2779) {
        true
    } else {
        let a2780 = v0;
        let a2781 = 10u64;
        nat_eq(a2780, a2781)
    } {
        true
    } else {
        let a2782 = v0;
        let a2783 = 13u64;
        if nat_eq(a2782, a2783) {
            true
        } else {
            let a2784 = 32u64;
            let a2785 = v0;
            if nat_le(a2784, a2785) {
                let a2786 = v0;
                let a2787 = 126u64;
                nat_le(a2786, a2787)
            } else {
                false
            }
        }
    } {
        true
    } else {
        let a2788 = v0;
        let a2789 = 133u64;
        if if nat_eq(a2788, a2789) {
            true
        } else {
            let a2790 = 160u64;
            let a2791 = v0;
            if nat_le(a2790, a2791) {
                let a2792 = v0;
                let a2793 = 55295u64;
                nat_le(a2792, a2793)
            } else {
                false
            }
        } {
            true
        } else {
            let a2794 = 57344u64;
            let a2795 = v0;
            if if nat_le(a2794, a2795) {
                let a2796 = v0;
                let a2797 = 65533u64;
                nat_le(a2796, a2797)
            } else {
                false
            } {
                true
            } else {
                let a2798 = 65536u64;
                let a2799 = v0;
                if nat_le(a2798, a2799) {
                    let a2800 = v0;
                    let a2801 = 1114111u64;
                    nat_le(a2800, a2801)
                } else {
                    false
                }
            }
        }
    }
}

pub fn f418(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f317(v0.clone(), v1)?;
    if f315(f232(v0.clone(), v2)?) {
        Ok(true)
    } else {
        let a2802 = f232(v0, v2)?;
        let a2803 = 35u64;
        Ok(nat_eq(a2802, a2803))
    }
}

pub fn f419(v0: Bytes, v1: u64) -> R<u64> {
    let (h2804, _) = f188(Fn5::F508(v0), {
        let a2805 = v1;
        let a2806 = 1u64;
        nat_add(a2805, a2806)?
    }, v1)?;
    Ok(h2804)
}

pub fn f420(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f509(v0, v1)
}

pub fn f421(v0: Bytes, v1: u64) -> R<u64> {
    let (h2807, _) = f188(Fn5::F510(v0.clone()), {
        let a2809 = {
            let a2808 = v0;
            length_bytes(a2808)
        };
        let a2810 = 1u64;
        nat_add(a2809, a2810)?
    }, v1)?;
    Ok(h2807)
}

pub fn f422(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f511(v0, v1)
}

pub fn f423(v0: u64, v1: bool) -> bool {
    f234(if if f314(v0) {
        true
    } else {
        f315(v0)
    } {
        true
    } else {
        if v1 {
            f512(v0)
        } else {
            false
        }
    })
}

pub fn f424(v0: Bytes, v1: u64, v2: u64) -> R<Adt22> {
    let v3: bool = {
        let a2811 = f232(v0.clone(), v1)?;
        let a2812 = 62u64;
        nat_eq(a2811, a2812)
    };
    let v4: u64 = f232(v0.clone(), {
        let a2813 = v1;
        let a2814 = 1u64;
        nat_add(a2813, a2814)?
    })?;
    let v5: u64 = f232(v0.clone(), {
        let a2815 = v1;
        let a2816 = 2u64;
        nat_add(a2815, a2816)?
    })?;
    let v6: u64 = f513(v4);
    let v7: u64 = f514(v4);
    let v8: bool = {
        let a2817 = 0u64;
        let a2818 = v6;
        if nat_lt(a2817, a2818) {
            true
        } else {
            let a2819 = 0u64;
            let a2820 = v7;
            nat_lt(a2819, a2820)
        }
    };
    let v9: u64 = {
        let a2821 = 0u64;
        let a2822 = v7;
        if nat_lt(a2821, a2822) {
            f513(v5)
        } else {
            0u64
        }
    };
    let v10: u64 = {
        let a2823 = 0u64;
        let a2824 = v6;
        if nat_lt(a2823, a2824) {
            f514(v5)
        } else {
            0u64
        }
    };
    let v11: u64 = {
        let a2825 = v6;
        let a2826 = v9;
        nat_add(a2825, a2826)?
    };
    let v12: u64 = {
        let a2827 = v7;
        let a2828 = v10;
        nat_add(a2827, a2828)?
    };
    let v13: u64 = {
        let a2837 = {
            let a2829 = v1;
            let a2830 = 1u64;
            nat_add(a2829, a2830)?
        };
        let a2838 = {
            let a2835 = if v8 {
                1u64
            } else {
                0u64
            };
            let a2836 = {
                let a2831 = 0u64;
                let a2832 = v9;
                if if nat_lt(a2831, a2832) {
                    true
                } else {
                    let a2833 = 0u64;
                    let a2834 = v10;
                    nat_lt(a2833, a2834)
                } {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a2835, a2836)?
        };
        nat_add(a2837, a2838)?
    };
    let m2839 = f231(v0.clone(), v13)?;
    match m2839 {
        Ok(v15) => {
            let a2840 = 0u64;
            let a2841 = v12;
            if nat_lt(a2840, a2841) {
                f515(v0, v15, {
                    let a2844 = {
                        let a2842 = v2;
                        let a2843 = v12;
                        nat_add(a2842, a2843)?
                    };
                    let a2845 = 1u64;
                    nat_sub(a2844, a2845)
                }, v3, v11)
            } else {
                let v16: (u64, u64) = {
                    let (h2846, _) = f240(Fn6::F516(v0.clone()), {
                        let a2848 = {
                            let a2847 = v0.clone();
                            length_bytes(a2847)
                        };
                        let a2849 = 1u64;
                        nat_add(a2848, a2849)?
                    }, (v15, 0u64))?;
                    h2846
                };
                let v17: u64 = f319(v0.clone(), {
                    let (h2850, _) = v16;
                    h2850
                })?;
                let a2851 = f232(v0.clone(), v17)?;
                let a2852 = 256u64;
                if nat_eq(a2851, a2852) {
                    Ok(Adt22::C0(Adt2::C5({
                        let a2853 = v11;
                        let a2854 = 2u64;
                        if nat_eq(a2853, a2854) {
                            f517(f518(v0.clone(), v15)?)
                        } else {
                            Bytes::lit(&[])
                        }
                    }), {
                        let a2855 = v0;
                        length_bytes(a2855)
                    }))
                } else {
                    let v18: u64 = f519({
                        let a2857 = v17;
                        let a2858 = {
                            let (h2856, _) = v16;
                            h2856
                        };
                        nat_sub(a2857, a2858)
                    }, v2);
                    let a2860 = v18;
                    let a2861 = {
                        let (_, h2859) = v16;
                        h2859
                    };
                    if nat_lt(a2860, a2861) {
                        Ok(Adt22::C1(f125(Str::lit("a leading empty line of a block scalar holds more spaces than its first content line (YAML 1.2.2 section 8.1.1.1)"), v15)?))
                    } else {
                        f515(v0, v15, v18, v3, v11)
                    }
                }
            }
        }
        Err(v14) => {
            Ok(Adt22::C1(v14))
        }
    }
}

pub fn f425(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Adt34> {
    let a2862 = f232(v0.clone(), v1)?;
    let a2863 = 34u64;
    if nat_eq(a2862, a2863) {
        f520(v0.clone(), {
            let a2864 = v1;
            let a2865 = 1u64;
            nat_add(a2864, a2865)?
        }, v2, v3, {
            let a2867 = {
                let a2866 = v0;
                length_bytes(a2866)
            };
            let a2868 = 2u64;
            nat_add(a2867, a2868)?
        }, Bytes::lit(&[]))
    } else {
        f521(v0.clone(), {
            let a2869 = v1;
            let a2870 = 1u64;
            nat_add(a2869, a2870)?
        }, v2, v3, {
            let a2872 = {
                let a2871 = v0;
                length_bytes(a2871)
            };
            let a2873 = 2u64;
            nat_add(a2872, a2873)?
        }, Bytes::lit(&[]))
    }
}

pub fn f426(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: List<Bytes>) -> R<Adt22> {
    let m2874 = v3;
    if m2874 == 0 {
        Ok(Adt22::C1(f125(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v6 = m2874 - 1;
        let a2875 = v4;
        let a2876 = 1u64;
        if nat_eq(a2875, a2876) {
            let m2877 = f426(v0.clone(), v1, v2, v6, 0u64, List::<Bytes>::nil())?;
            match m2877 {
                Adt22::C0(v8, v9) => {
                    let m2878 = f522(v0.clone(), v9, v2)?;
                    match m2878 {
                        Ok(v11) => {
                            let v12: u64 = f232(v0.clone(), v11)?;
                            let a2879 = v12;
                            let a2880 = 58u64;
                            if nat_eq(a2879, a2880) {
                                Ok(Adt22::C1(f125(Str::lit("a single-pair mapping inside a flow sequence is not read (YAML 1.2.2 [150])"), v11)?))
                            } else {
                                let a2881 = v12;
                                let a2882 = 93u64;
                                if nat_eq(a2881, a2882) {
                                    Ok(Adt22::C0(Adt2::C9(std::rc::Rc::new(v8), std::rc::Rc::new(Adt2::C8)), {
                                        let a2883 = v11;
                                        let a2884 = 1u64;
                                        nat_add(a2883, a2884)?
                                    }))
                                } else {
                                    let a2885 = v12;
                                    let a2886 = 44u64;
                                    if nat_eq(a2885, a2886) {
                                        let m2889 = f522(v0.clone(), {
                                            let a2887 = v11;
                                            let a2888 = 1u64;
                                            nat_add(a2887, a2888)?
                                        }, v2)?;
                                        match m2889 {
                                            Ok(v14) => {
                                                let a2890 = f232(v0.clone(), v14)?;
                                                let a2891 = 93u64;
                                                if nat_eq(a2890, a2891) {
                                                    Ok(Adt22::C0(Adt2::C9(std::rc::Rc::new(v8), std::rc::Rc::new(Adt2::C8)), {
                                                        let a2892 = v14;
                                                        let a2893 = 1u64;
                                                        nat_add(a2892, a2893)?
                                                    }))
                                                } else {
                                                    let m2894 = f426(v0, v14, v2, v6, 1u64, List::<Bytes>::nil())?;
                                                    match m2894 {
                                                        Adt22::C0(v16, v17) => {
                                                            Ok(Adt22::C0(Adt2::C9(std::rc::Rc::new(v8), std::rc::Rc::new(v16)), v17))
                                                        }
                                                        Adt22::C1(v15) => {
                                                            Ok(Adt22::C1(v15))
                                                        }
                                                    }
                                                }
                                            }
                                            Err(v13) => {
                                                Ok(Adt22::C1(v13))
                                            }
                                        }
                                    } else {
                                        Ok(Adt22::C1(f125(Str::lit("a flow sequence entry must be followed by , or ] (YAML 1.2.2 [137])"), v11)?))
                                    }
                                }
                            }
                        }
                        Err(v10) => {
                            Ok(Adt22::C1(v10))
                        }
                    }
                }
                Adt22::C1(v7) => {
                    Ok(Adt22::C1(v7))
                }
            }
        } else {
            let a2895 = v4;
            let a2896 = 2u64;
            if nat_eq(a2895, a2896) {
                let m2897 = f523(v0.clone(), v1, v2)?;
                match m2897 {
                    Adt34::C0(v19, v20) => {
                        let m2898 = f522(v0.clone(), v20, v2)?;
                        match m2898 {
                            Ok(v22) => {
                                let m2912 = {
                                    let a2899 = f232(v0.clone(), v22)?;
                                    let a2900 = 44u64;
                                    if if nat_eq(a2899, a2900) {
                                        true
                                    } else {
                                        let a2901 = f232(v0.clone(), v22)?;
                                        let a2902 = 125u64;
                                        nat_eq(a2901, a2902)
                                    } {
                                        Adt22::C0(Adt2::C0, v22)
                                    } else {
                                        if f234({
                                            let a2903 = f232(v0.clone(), v22)?;
                                            let a2904 = 58u64;
                                            nat_eq(a2903, a2904)
                                        }) {
                                            Adt22::C1(f125(Str::lit("a flow mapping entry must be a key, then : and its value, or a key alone (YAML 1.2.2 [142], [146])"), v22)?)
                                        } else {
                                            let m2907 = f522(v0.clone(), {
                                                let a2905 = v22;
                                                let a2906 = 1u64;
                                                nat_add(a2905, a2906)?
                                            }, v2)?;
                                            match m2907 {
                                                Ok(v24) => {
                                                    let a2908 = f232(v0.clone(), v24)?;
                                                    let a2909 = 44u64;
                                                    if if nat_eq(a2908, a2909) {
                                                        true
                                                    } else {
                                                        let a2910 = f232(v0.clone(), v24)?;
                                                        let a2911 = 125u64;
                                                        nat_eq(a2910, a2911)
                                                    } {
                                                        Adt22::C0(Adt2::C0, v24)
                                                    } else {
                                                        f426(v0.clone(), v24, v2, v6, 0u64, List::<Bytes>::nil())?
                                                    }
                                                }
                                                Err(v23) => {
                                                    Adt22::C1(v23)
                                                }
                                            }
                                        }
                                    }
                                };
                                match m2912 {
                                    Adt22::C0(v26, v27) => {
                                        if f278(v19.clone(), v5.clone()) {
                                            Ok(Adt22::C1(f125(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                        } else {
                                            let m2913 = f522(v0.clone(), v27, v2)?;
                                            match m2913 {
                                                Ok(v29) => {
                                                    let v30: u64 = f232(v0.clone(), v29)?;
                                                    let a2914 = v30;
                                                    let a2915 = 125u64;
                                                    if nat_eq(a2914, a2915) {
                                                        Ok(Adt22::C0(Adt2::C10(v19, std::rc::Rc::new(v26), std::rc::Rc::new(Adt2::C8)), {
                                                            let a2916 = v29;
                                                            let a2917 = 1u64;
                                                            nat_add(a2916, a2917)?
                                                        }))
                                                    } else {
                                                        let a2918 = v30;
                                                        let a2919 = 44u64;
                                                        if nat_eq(a2918, a2919) {
                                                            let m2922 = f522(v0.clone(), {
                                                                let a2920 = v29;
                                                                let a2921 = 1u64;
                                                                nat_add(a2920, a2921)?
                                                            }, v2)?;
                                                            match m2922 {
                                                                Ok(v32) => {
                                                                    let a2923 = f232(v0.clone(), v32)?;
                                                                    let a2924 = 125u64;
                                                                    if nat_eq(a2923, a2924) {
                                                                        Ok(Adt22::C0(Adt2::C10(v19, std::rc::Rc::new(v26), std::rc::Rc::new(Adt2::C8)), {
                                                                            let a2925 = v32;
                                                                            let a2926 = 1u64;
                                                                            nat_add(a2925, a2926)?
                                                                        }))
                                                                    } else {
                                                                        let m2927 = f426(v0, v32, v2, v6, 2u64, List::cons(v19.clone(), v5))?;
                                                                        match m2927 {
                                                                            Adt22::C0(v34, v35) => {
                                                                                Ok(Adt22::C0(Adt2::C10(v19, std::rc::Rc::new(v26), std::rc::Rc::new(v34)), v35))
                                                                            }
                                                                            Adt22::C1(v33) => {
                                                                                Ok(Adt22::C1(v33))
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                Err(v31) => {
                                                                    Ok(Adt22::C1(v31))
                                                                }
                                                            }
                                                        } else {
                                                            Ok(Adt22::C1(f125(Str::lit("a flow mapping entry must be followed by , or } (YAML 1.2.2 [140])"), v29)?))
                                                        }
                                                    }
                                                }
                                                Err(v28) => {
                                                    Ok(Adt22::C1(v28))
                                                }
                                            }
                                        }
                                    }
                                    Adt22::C1(v25) => {
                                        Ok(Adt22::C1(v25))
                                    }
                                }
                            }
                            Err(v21) => {
                                Ok(Adt22::C1(v21))
                            }
                        }
                    }
                    Adt34::C1(v18) => {
                        Ok(Adt22::C1(v18))
                    }
                }
            } else {
                let v36: u64 = f232(v0.clone(), v1)?;
                let a2928 = v36;
                let a2929 = 91u64;
                if if nat_eq(a2928, a2929) {
                    true
                } else {
                    let a2930 = v36;
                    let a2931 = 123u64;
                    nat_eq(a2930, a2931)
                } {
                    let m2934 = f522(v0.clone(), {
                        let a2932 = v1;
                        let a2933 = 1u64;
                        nat_add(a2932, a2933)?
                    }, v2)?;
                    match m2934 {
                        Ok(v38) => {
                            let a2937 = f232(v0.clone(), v38)?;
                            let a2938 = {
                                let a2935 = v36;
                                let a2936 = 2u64;
                                nat_add(a2935, a2936)?
                            };
                            if nat_eq(a2937, a2938) {
                                Ok(Adt22::C0({
                                    let a2939 = v36;
                                    let a2940 = 91u64;
                                    if nat_eq(a2939, a2940) {
                                        Adt2::C6(std::rc::Rc::new(Adt2::C8))
                                    } else {
                                        Adt2::C7(std::rc::Rc::new(Adt2::C8))
                                    }
                                }, {
                                    let a2941 = v38;
                                    let a2942 = 1u64;
                                    nat_add(a2941, a2942)?
                                }))
                            } else {
                                let m2945 = f426(v0, v38, v2, v6, {
                                    let a2943 = v36;
                                    let a2944 = 91u64;
                                    if nat_eq(a2943, a2944) {
                                        1u64
                                    } else {
                                        2u64
                                    }
                                }, List::<Bytes>::nil())?;
                                match m2945 {
                                    Adt22::C0(v40, v41) => {
                                        Ok(Adt22::C0({
                                            let a2946 = v36;
                                            let a2947 = 91u64;
                                            if nat_eq(a2946, a2947) {
                                                Adt2::C6(std::rc::Rc::new(v40))
                                            } else {
                                                Adt2::C7(std::rc::Rc::new(v40))
                                            }
                                        }, v41))
                                    }
                                    Adt22::C1(v39) => {
                                        Ok(Adt22::C1(v39))
                                    }
                                }
                            }
                        }
                        Err(v37) => {
                            Ok(Adt22::C1(v37))
                        }
                    }
                } else {
                    let a2948 = v36;
                    let a2949 = 34u64;
                    if if nat_eq(a2948, a2949) {
                        true
                    } else {
                        let a2950 = v36;
                        let a2951 = 39u64;
                        nat_eq(a2950, a2951)
                    } {
                        let m2952 = f425(v0, v1, v2, false)?;
                        match m2952 {
                            Adt34::C0(v43, v44) => {
                                Ok(Adt22::C0(Adt2::C5(v43), v44))
                            }
                            Adt34::C1(v42) => {
                                Ok(Adt22::C1(v42))
                            }
                        }
                    } else {
                        let m2953 = f322(v0.clone(), v1)?;
                        match m2953 {
                            None => {
                                if f234(f427(v0.clone(), v1, true)?) {
                                    Ok(Adt22::C1(f125(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                                } else {
                                    let v46: u64 = f428(v0.clone(), v1, true)?;
                                    let v47: u64 = f317(v0.clone(), v46)?;
                                    let m2956 = {
                                        let a2954 = f232(v0.clone(), v47)?;
                                        let a2955 = 10u64;
                                        if nat_eq(a2954, a2955) {
                                            f522(v0.clone(), v47, v2)?
                                        } else {
                                            Ok::<u64, Bytes>(v47)
                                        }
                                    };
                                    match m2956 {
                                        Ok(v49) => {
                                            let a2957 = f232(v0.clone(), v47)?;
                                            let a2958 = 10u64;
                                            if if if nat_eq(a2957, a2958) {
                                                f234(f512(f232(v0.clone(), v49)?))
                                            } else {
                                                false
                                            } {
                                                f234({
                                                    let a2959 = f232(v0.clone(), v49)?;
                                                    let a2960 = 58u64;
                                                    nat_eq(a2959, a2960)
                                                })
                                            } else {
                                                false
                                            } {
                                                Ok(Adt22::C1(f125(Str::lit("a multi-line plain scalar inside a flow collection is not read (YAML 1.2.2 [135])"), v47)?))
                                            } else {
                                                f430(f326(v0, v1, v46), v1, v46)
                                            }
                                        }
                                        Err(v48) => {
                                            Ok(Adt22::C1(v48))
                                        }
                                    }
                                }
                            }
                            Some(v45) => {
                                Ok(Adt22::C1(v45))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f427(v0: Bytes, v1: u64, v2: bool) -> R<bool> {
    let v3: u64 = f232(v0.clone(), v1)?;
    if if f314(v3) {
        true
    } else {
        f315(v3)
    } {
        Ok(false)
    } else {
        let a2961 = v3;
        let a2962 = 45u64;
        if if if nat_eq(a2961, a2962) {
            true
        } else {
            let a2963 = v3;
            let a2964 = 63u64;
            nat_eq(a2963, a2964)
        } {
            true
        } else {
            let a2965 = v3;
            let a2966 = 58u64;
            nat_eq(a2965, a2966)
        } {
            Ok(f423(f232(v0, {
                let a2967 = v1;
                let a2968 = 1u64;
                nat_add(a2967, a2968)?
            })?, v2))
        } else {
            Ok(f234(f524(v3)?))
        }
    }
}

pub fn f428(v0: Bytes, v1: u64, v2: bool) -> R<u64> {
    let (_, h2969) = {
        let (h2970, _) = f240(Fn6::F525(v2, v0.clone()), {
            let a2972 = {
                let a2971 = v0;
                length_bytes(a2971)
            };
            let a2973 = 1u64;
            nat_add(a2972, a2973)?
        }, (v1, v1))?;
        h2970
    };
    Ok(h2969)
}

pub fn f429(v0: Bytes, p2998: u64, v2: u64, p2999: u64, p3000: Bytes) -> R<Adt34> {
    let m2997 = (p2998, (p2999, p3000));
    {
        let mut m2997 = m2997;
        loop {
            let step = {
                let (v1, (v3, v4)) = m2997;
                let m2974 = v3;
                if m2974 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C1(f125(Str::lit("a plain scalar has more lines than the input"), v1)?)))
                } else {
                    let v5 = m2974 - 1;
                    let v6: u64 = f317(v0.clone(), v1)?;
                    let v7: u64 = f232(v0.clone(), v6)?;
                    let a2975 = v7;
                    let a2976 = 58u64;
                    if nat_eq(a2975, a2976) {
                        Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C1(f526(v6)?)))
                    } else {
                        let a2977 = v7;
                        let a2978 = 35u64;
                        if if nat_eq(a2977, a2978) {
                            true
                        } else {
                            let a2979 = v7;
                            let a2980 = 256u64;
                            nat_eq(a2979, a2980)
                        } {
                            Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C0(v4, f318(v0.clone(), v6)?)))
                        } else {
                            let v8: (u64, u64) = f527(v0.clone(), {
                                let a2981 = v6;
                                let a2982 = 1u64;
                                nat_add(a2981, a2982)?
                            })?;
                            let v9: u64 = {
                                let (h2983, _) = v8;
                                h2983
                            };
                            let v10: u64 = f319(v0.clone(), v9)?;
                            let v11: u64 = f317(v0.clone(), v10)?;
                            let v12: u64 = f232(v0.clone(), v11)?;
                            let a2984 = v12;
                            let a2985 = 256u64;
                            if if if if nat_eq(a2984, a2985) {
                                true
                            } else {
                                let a2988 = {
                                    let a2986 = v10;
                                    let a2987 = v9;
                                    nat_sub(a2986, a2987)
                                };
                                let a2989 = v2;
                                nat_lt(a2988, a2989)
                            } {
                                true
                            } else {
                                let a2990 = v12;
                                let a2991 = 35u64;
                                nat_eq(a2990, a2991)
                            } {
                                true
                            } else {
                                f230(v0.clone(), v9)?
                            } {
                                Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C0(v4, {
                                    let a2992 = v6;
                                    let a2993 = 1u64;
                                    nat_add(a2992, a2993)?
                                })))
                            } else {
                                let v13: u64 = f428(v0.clone(), v11, false)?;
                                let a2994 = v13;
                                let a2995 = v11;
                                if nat_eq(a2994, a2995) {
                                    Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C1(f526(v11)?)))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Adt34>>((v13, (v5, f7(List::cons(v4, List::cons(f528({
                                        let (_, h2996) = v8;
                                        h2996
                                    }), List::cons(f326(v0.clone(), v11, v13), List::<Bytes>::nil())))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2997 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f430(v0: Bytes, v1: u64, v2: u64) -> R<Adt22> {
    let a3002 = {
        let a3001 = v0.clone();
        length_bytes(a3001)
    };
    let a3003 = 0u64;
    if if nat_eq(a3002, a3003) {
        true
    } else {
        f278(v0.clone(), List::cons(f8(Str::lit("~")), List::cons(f8(Str::lit("null")), List::cons(f8(Str::lit("Null")), List::cons(f8(Str::lit("NULL")), List::<Bytes>::nil())))))
    } {
        Ok(Adt22::C0(Adt2::C0, v2))
    } else {
        if f278(v0.clone(), List::cons(f8(Str::lit("true")), List::cons(f8(Str::lit("True")), List::cons(f8(Str::lit("TRUE")), List::<Bytes>::nil())))) {
            Ok(Adt22::C0(Adt2::C2(true), v2))
        } else {
            if f278(v0.clone(), List::cons(f8(Str::lit("false")), List::cons(f8(Str::lit("False")), List::cons(f8(Str::lit("FALSE")), List::<Bytes>::nil())))) {
                Ok(Adt22::C0(Adt2::C2(false), v2))
            } else {
                if f278(v0.clone(), List::cons(f8(Str::lit(".inf")), List::cons(f8(Str::lit(".Inf")), List::cons(f8(Str::lit(".INF")), List::cons(f8(Str::lit("+.inf")), List::cons(f8(Str::lit("+.Inf")), List::cons(f8(Str::lit("+.INF")), List::cons(f8(Str::lit("-.inf")), List::cons(f8(Str::lit("-.Inf")), List::cons(f8(Str::lit("-.INF")), List::cons(f8(Str::lit(".nan")), List::cons(f8(Str::lit(".NaN")), List::cons(f8(Str::lit(".NAN")), List::<Bytes>::nil()))))))))))))) {
                    Ok(Adt22::C1(f125(Str::lit("an infinite or not-a-number float is not read: Studio.Value has none (YAML 1.2.2 section 10.3.2)"), v1)?))
                } else {
                    if if f128(v0.clone(), 0u64, Str::lit("0o")) {
                        let a3005 = 2u64;
                        let a3006 = {
                            let a3004 = v0.clone();
                            length_bytes(a3004)
                        };
                        nat_lt(a3005, a3006)
                    } else {
                        false
                    } {
                        f529(v0, 8u64, v1, v2)
                    } else {
                        if if f128(v0.clone(), 0u64, Str::lit("0x")) {
                            let a3008 = 2u64;
                            let a3009 = {
                                let a3007 = v0.clone();
                                length_bytes(a3007)
                            };
                            nat_lt(a3008, a3009)
                        } else {
                            false
                        } {
                            f529(v0, 16u64, v1, v2)
                        } else {
                            let m3010 = f530(v0.clone())?;
                            match m3010 {
                                None => {
                                    Ok(Adt22::C0(Adt2::C5(v0), v2))
                                }
                                Some(v3) => {
                                    let m3011 = f97(v3.clone(), 0u64)?;
                                    match m3011 {
                                        None => {
                                            Ok(Adt22::C1(f531(v1)?))
                                        }
                                        Some(v4) => {
                                            let a3014 = {
                                                let (h3012, _) = v4;
                                                h3012
                                            };
                                            let a3015 = {
                                                let a3013 = v3;
                                                length_bytes(a3013)
                                            };
                                            if nat_eq(a3014, a3015) {
                                                Ok(Adt22::C0({
                                                    let (_, h3016) = v4;
                                                    h3016
                                                }, v2))
                                            } else {
                                                Ok(Adt22::C1(f531(v1)?))
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

pub fn f431(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f532(v0, v1, v2)
}

pub fn f432(v0: Bytes, v1: u64) -> R<bool> {
    let a3017 = f232(v0.clone(), v1)?;
    let a3018 = 58u64;
    if nat_eq(a3017, a3018) {
        if f314(f232(v0.clone(), {
            let a3019 = v1;
            let a3020 = 1u64;
            nat_add(a3019, a3020)?
        })?) {
            Ok(true)
        } else {
            Ok(f315(f232(v0, {
                let a3021 = v1;
                let a3022 = 1u64;
                nat_add(a3021, a3022)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f433(v0: Bytes, p3041: u64, p3042: u64, p3043: Bytes) -> R<Bytes> {
    let m3040 = (p3041, (p3042, p3043));
    {
        let mut m3040 = m3040;
        loop {
            let step = {
                let (v1, (v2, v3)) = m3040;
                let m3023 = v2;
                if m3023 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m3023 - 1;
                    let v5: u64 = f150(v0.clone(), v1, {
                        let a3027 = {
                            let a3025 = {
                                let a3024 = v0.clone();
                                length_bytes(a3024)
                            };
                            let a3026 = v1;
                            nat_sub(a3025, a3026)
                        };
                        let a3028 = 1u64;
                        nat_add(a3027, a3028)?
                    })?;
                    let v6: Bytes = {
                        let a3031 = v3;
                        let a3032 = f45(v0.clone(), v1, {
                            let a3029 = v5;
                            let a3030 = v1;
                            nat_sub(a3029, a3030)
                        });
                        append_bytes(a3031, a3032)
                    };
                    let m3035 = {
                        let a3033 = v0.clone();
                        let a3034 = v5;
                        index_bytes(a3033, a3034)
                    };
                    match m3035 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v6))
                        }
                        Some(v7) => {
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                let a3036 = v5;
                                let a3037 = 1u64;
                                nat_add(a3036, a3037)?
                            }, (v4, {
                                let a3038 = v6;
                                let a3039 = f533(v0.clone(), v5, v7)?;
                                append_bytes(a3038, a3039)
                            })))
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

pub fn f434(v0: Fn8, p3047: List<(Bytes, Bytes)>, p3048: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m3046 = (p3047, p3048);
    {
        let mut m3046 = m3046;
        loop {
            let step = {
                let (v1, v2) = m3046;
                let m3044 = v2;
                match m3044.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c3045: Fn8 = v0.clone();
                            c3045.apply(v1, v3)
                        }, v4))
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

pub fn f435(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f534(v0, v1)
}

pub fn f436(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f535(v0)
}

pub fn f437(v0: Fn9, p3052: List<Bytes>, p3053: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m3051 = (p3052, p3053);
    {
        let mut m3051 = m3051;
        loop {
            let step = {
                let (v1, v2) = m3051;
                let m3049 = v2;
                match m3049.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c3050: Fn9 = v0.clone();
                            c3050.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3051 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f438(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f536(v1)?, v0))
}

pub fn f439(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a3054 = v0.clone();
        length_bytes(a3054)
    };
    let a3058 = {
        let a3055 = v1;
        let a3056 = 2u64;
        let a3057 = 0u64;
        nat_rem(a3055, a3056, a3057)
    };
    let a3059 = 0u64;
    if nat_eq(a3058, a3059) {
        let (_, h3060) = {
            let (h3061, _) = f537(Fn10::F538(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h3061
        };
        Ok(h3060)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f440(v0: u64) -> bool {
    let a3062 = v0;
    let a3063 = 8u64;
    if nat_eq(a3062, a3063) {
        true
    } else {
        let a3064 = v0;
        let a3065 = 9u64;
        if nat_eq(a3064, a3065) {
            true
        } else {
            let a3066 = v0;
            let a3067 = 10u64;
            if nat_eq(a3066, a3067) {
                true
            } else {
                let a3068 = v0;
                let a3069 = 12u64;
                if nat_eq(a3068, a3069) {
                    true
                } else {
                    let a3070 = v0;
                    let a3071 = 13u64;
                    nat_eq(a3070, a3071)
                }
            }
        }
    }
}

pub fn f441(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3074 = {
        let a3072 = v0;
        let a3073 = v1;
        index_bytes(a3072, a3073)
    };
    match m3074 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f539(v2)? {
                Ok(Some({
                    let a3075 = v1;
                    let a3076 = 1u64;
                    nat_add(a3075, a3076)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f442(v0: u8) -> R<u64> {
    let a3123 = {
        let v1: u8 = v0;
        let a3121 = {
            let a3097 = {
                let a3085 = {
                    let a3079 = {
                        let a3077 = v1;
                        let a3078 = 128u8;
                        fixed_u8::bit_and(a3077, a3078)
                    };
                    let a3080 = 128u8;
                    if equal(a3079, a3080) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3086 = {
                    let a3083 = {
                        let a3081 = v1;
                        let a3082 = 64u8;
                        fixed_u8::bit_and(a3081, a3082)
                    };
                    let a3084 = 64u8;
                    if equal(a3083, a3084) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3085, a3086)?
            };
            let a3098 = {
                let a3095 = {
                    let a3089 = {
                        let a3087 = v1;
                        let a3088 = 32u8;
                        fixed_u8::bit_and(a3087, a3088)
                    };
                    let a3090 = 32u8;
                    if equal(a3089, a3090) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3096 = {
                    let a3093 = {
                        let a3091 = v1;
                        let a3092 = 16u8;
                        fixed_u8::bit_and(a3091, a3092)
                    };
                    let a3094 = 16u8;
                    if equal(a3093, a3094) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3095, a3096)?
            };
            nat_add(a3097, a3098)?
        };
        let a3122 = {
            let a3119 = {
                let a3107 = {
                    let a3101 = {
                        let a3099 = v1;
                        let a3100 = 8u8;
                        fixed_u8::bit_and(a3099, a3100)
                    };
                    let a3102 = 8u8;
                    if equal(a3101, a3102) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3108 = {
                    let a3105 = {
                        let a3103 = v1;
                        let a3104 = 4u8;
                        fixed_u8::bit_and(a3103, a3104)
                    };
                    let a3106 = 4u8;
                    if equal(a3105, a3106) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3107, a3108)?
            };
            let a3120 = {
                let a3117 = {
                    let a3111 = {
                        let a3109 = v1;
                        let a3110 = 2u8;
                        fixed_u8::bit_and(a3109, a3110)
                    };
                    let a3112 = 2u8;
                    if equal(a3111, a3112) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3118 = {
                    let a3115 = {
                        let a3113 = v1;
                        let a3114 = 1u8;
                        fixed_u8::bit_and(a3113, a3114)
                    };
                    let a3116 = 1u8;
                    if equal(a3115, a3116) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3117, a3118)?
            };
            nat_add(a3119, a3120)?
        };
        nat_add(a3121, a3122)?
    };
    let a3124 = 48u64;
    Ok(nat_sub(a3123, a3124))
}

pub fn f443(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a3125 = v0.clone();
        length_bytes(a3125)
    };
    let a3126 = v1;
    let a3127 = v2;
    if if nat_lt(a3126, a3127) {
        let a3132 = f94(v0, {
            let a3130 = v2;
            let a3131 = {
                let a3128 = v1;
                let a3129 = 1u64;
                nat_add(a3128, a3129)?
            };
            nat_sub(a3130, a3131)
        })?;
        let a3133 = 48u64;
        nat_eq(a3132, a3133)
    } else {
        false
    } {
        Ok(Some({
            let a3134 = v1;
            let a3135 = 1u64;
            nat_add(a3134, a3135)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f444(v0: Adt30) -> R<Adt30> {
    let v1: u32 = f540(match v0.clone() {
        Adt30::C0(_, _, _, _, _, _, _, _, _, h3136) => {
            h3136
        }
    });
    let v2: u32 = f345(match v0.clone() {
        Adt30::C0(_, _, _, _, _, _, _, h3137, _, _) => {
            h3137
        }
    }, f345(f541(match v0.clone() {
        Adt30::C0(_, _, _, _, h3138, _, _, _, _, _) => {
            h3138
        }
    }), f345(f542(match v0.clone() {
        Adt30::C0(_, _, _, _, h3139, _, _, _, _, _) => {
            h3139
        }
    }, match v0.clone() {
        Adt30::C0(_, _, _, _, _, h3140, _, _, _, _) => {
            h3140
        }
    }, match v0.clone() {
        Adt30::C0(_, _, _, _, _, _, h3141, _, _, _) => {
            h3141
        }
    }), f345(v1, {
        let m3144 = match v0.clone() {
            Adt30::C0(_, _, _, _, _, _, _, _, h3143, _) => {
                h3143
            }
        };
        match m3144 {
            Adt29::C0(h3142, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3142
            }
        }
    }))));
    let v3: u32 = f345(f543(match v0.clone() {
        Adt30::C0(h3145, _, _, _, _, _, _, _, _, _) => {
            h3145
        }
    }), f544(match v0.clone() {
        Adt30::C0(h3146, _, _, _, _, _, _, _, _, _) => {
            h3146
        }
    }, match v0.clone() {
        Adt30::C0(_, h3147, _, _, _, _, _, _, _, _) => {
            h3147
        }
    }, match v0.clone() {
        Adt30::C0(_, _, h3148, _, _, _, _, _, _, _) => {
            h3148
        }
    }));
    Ok(Adt30::C0(f345(v2, v3), match v0.clone() {
        Adt30::C0(h3149, _, _, _, _, _, _, _, _, _) => {
            h3149
        }
    }, match v0.clone() {
        Adt30::C0(_, h3150, _, _, _, _, _, _, _, _) => {
            h3150
        }
    }, match v0.clone() {
        Adt30::C0(_, _, h3151, _, _, _, _, _, _, _) => {
            h3151
        }
    }, f345(match v0.clone() {
        Adt30::C0(_, _, _, h3152, _, _, _, _, _, _) => {
            h3152
        }
    }, v2), match v0.clone() {
        Adt30::C0(_, _, _, _, h3153, _, _, _, _, _) => {
            h3153
        }
    }, match v0.clone() {
        Adt30::C0(_, _, _, _, _, h3154, _, _, _, _) => {
            h3154
        }
    }, match v0.clone() {
        Adt30::C0(_, _, _, _, _, _, h3155, _, _, _) => {
            h3155
        }
    }, f545(match v0.clone() {
        Adt30::C0(_, _, _, _, _, _, _, _, h3156, _) => {
            h3156
        }
    }), {
        let a3158 = match v0 {
            Adt30::C0(_, _, _, _, _, _, _, _, _, h3157) => {
                h3157
            }
        };
        let a3159 = 1u64;
        nat_add(a3158, a3159)?
    }))
}

pub fn f445(v0: u32) -> u64 {
    let m3161 = {
        let a3160 = v0;
        fixed_u64::convert(i128::from(a3160))
    };
    match m3161 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f446(v0: u64) -> u32 {
    let m3165 = {
        let a3164 = {
            let a3162 = v0;
            let a3163 = 4294967295u64;
            fixed_u64::bit_and(a3162, a3163)
        };
        fixed_u32::convert(i128::from(a3164))
    };
    match m3165 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f447(v0: u64) -> u32 {
    let (_, h3166) = {
        let (_, h3167) = f546(Fn11::F547, 32u64, (v0, (1u32, 0u32)));
        h3167
    };
    h3166
}

pub fn f448(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a3172 = f548(v0, 24u32);
    let a3173 = {
        let a3170 = f548(v1, 16u32);
        let a3171 = {
            let a3168 = f548(v2, 8u32);
            let a3169 = v3;
            fixed_u32::bit_or(a3168, a3169)
        };
        fixed_u32::bit_or(a3170, a3171)
    };
    fixed_u32::bit_or(a3172, a3173)
}

pub fn f449(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a3174 = v2;
    let a3175 = v1;
    if nat_lt(a3174, a3175) {
        let m3178 = {
            let a3176 = v0;
            let a3177 = v2;
            index_bytes(a3176, a3177)
        };
        match m3178 {
            None => {
                0u32
            }
            Some(v3) => {
                let m3180 = {
                    let a3179 = v3;
                    fixed_u32::convert(i128::from(a3179))
                };
                match m3180 {
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
        let a3181 = v2;
        let a3182 = v1;
        if nat_eq(a3181, a3182) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f450(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f549(v1, v0, v2)
}

pub fn f451(v0: u64) -> bool {
    if f452(v0) {
        true
    } else {
        let a3183 = 65u64;
        let a3184 = v0;
        if if nat_le(a3183, a3184) {
            let a3185 = v0;
            let a3186 = 70u64;
            nat_le(a3185, a3186)
        } else {
            false
        } {
            true
        } else {
            let a3187 = 97u64;
            let a3188 = v0;
            if nat_le(a3187, a3188) {
                let a3189 = v0;
                let a3190 = 102u64;
                nat_le(a3189, a3190)
            } else {
                false
            }
        }
    }
}

pub fn f452(v0: u64) -> bool {
    let a3191 = 48u64;
    let a3192 = v0;
    if nat_le(a3191, a3192) {
        let a3193 = v0;
        let a3194 = 57u64;
        nat_le(a3193, a3194)
    } else {
        false
    }
}

pub fn f453(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f129(v0.clone(), v1)?;
    let a3195 = v2;
    let a3196 = 69u64;
    if if nat_eq(a3195, a3196) {
        true
    } else {
        let a3197 = v2;
        let a3198 = 101u64;
        nat_eq(a3197, a3198)
    } {
        let v3: u64 = f129(v0.clone(), {
            let a3199 = v1;
            let a3200 = 1u64;
            nat_add(a3199, a3200)?
        })?;
        let v4: u64 = {
            let a3201 = v3;
            let a3202 = 43u64;
            if if nat_eq(a3201, a3202) {
                true
            } else {
                let a3203 = v3;
                let a3204 = 45u64;
                nat_eq(a3203, a3204)
            } {
                let a3205 = v1;
                let a3206 = 2u64;
                nat_add(a3205, a3206)?
            } else {
                let a3207 = v1;
                let a3208 = 1u64;
                nat_add(a3207, a3208)?
            }
        };
        let v5: u64 = f350(v0, 0u64, v4)?;
        let a3209 = v4;
        let a3210 = v5;
        if nat_lt(a3209, a3210) {
            Ok(v5)
        } else {
            Ok(v1)
        }
    } else {
        Ok(v1)
    }
}

pub fn f454(v0: Adt23, v1: Bytes) -> bool {
    let a3211 = f455(v0.clone());
    let a3212 = 3u64;
    if !nat_eq(a3211, a3212) {
        f32(f456(v0), v1)
    } else {
        false
    }
}

pub fn f455(v0: Adt23) -> u64 {
    let m3213 = v0;
    match m3213 {
        Adt23::C0(v1, _) => {
            v1
        }
    }
}

pub fn f456(v0: Adt23) -> Bytes {
    let m3214 = v0;
    match m3214 {
        Adt23::C0(_, v2) => {
            v2
        }
    }
}

pub fn f457(v0: Bytes) -> Bytes {
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

pub fn f458(v0: Bytes) -> bool {
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

pub fn f459(p3223: u64, p3224: List<Adt23>, p3225: List<Adt15>) -> R<Result<(List<Adt15>, List<Adt23>), Bytes>> {
    let m3222 = (p3223, (p3224, p3225));
    {
        let mut m3222 = m3222;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3222;
                let m3215 = v0;
                if m3215 == 0 {
                    Err::<(u64, (List<Adt23>, List<Adt15>)), R<Result<(List<Adt15>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt15>, List<Adt23>), Bytes>(f8(Str::lit("a dictionary has more members than the input has tokens")))))
                } else {
                    let v3 = m3215 - 1;
                    if f353(v1.clone(), Str::lit("}")) {
                        let m3216 = f360(f355(v1), Str::lit(";"));
                        match m3216 {
                            Ok(v5) => {
                                Err::<(u64, (List<Adt23>, List<Adt15>)), R<Result<(List<Adt15>, List<Adt23>), Bytes>>>(Ok(Ok::<(List<Adt15>, List<Adt23>), Bytes>((f550(v2, List::<Adt15>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<Adt23>, List<Adt15>)), R<Result<(List<Adt15>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt15>, List<Adt23>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m3217 = f250(v1)?;
                        match m3217 {
                            Ok(v7) => {
                                let v8: bool = f353(v7.clone(), Str::lit("required"));
                                let m3218 = f365(v3, if v8 {
                                    f355(v7)
                                } else {
                                    v7
                                }, 0u64)?;
                                match m3218 {
                                    Ok(v10) => {
                                        let m3219 = f359(f366(v10.clone()));
                                        match m3219 {
                                            Ok(v12) => {
                                                if if f353(f361(v12.clone()), Str::lit("=")) {
                                                    !v8
                                                } else {
                                                    false
                                                } {
                                                    let m3220 = f461(f355(f361(v12.clone())))?;
                                                    match m3220 {
                                                        Ok(v14) => {
                                                            Ok::<(u64, (List<Adt23>, List<Adt15>)), R<Result<(List<Adt15>, List<Adt23>), Bytes>>>((v3, (v14, List::cons(Adt15::C0(f363(v12), f468(f367(v10)), 0u64), v2))))
                                                        }
                                                        Err(v13) => {
                                                            Err::<(u64, (List<Adt23>, List<Adt15>)), R<Result<(List<Adt15>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt15>, List<Adt23>), Bytes>(v13)))
                                                        }
                                                    }
                                                } else {
                                                    let m3221 = f360(f361(v12.clone()), Str::lit(";"));
                                                    match m3221 {
                                                        Ok(v16) => {
                                                            Ok::<(u64, (List<Adt23>, List<Adt15>)), R<Result<(List<Adt15>, List<Adt23>), Bytes>>>((v3, (v16, List::cons(Adt15::C0(f363(v12), f468(f367(v10)), if v8 {
                                                                0u64
                                                            } else {
                                                                1u64
                                                            }), v2))))
                                                        }
                                                        Err(v15) => {
                                                            Err::<(u64, (List<Adt23>, List<Adt15>)), R<Result<(List<Adt15>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt15>, List<Adt23>), Bytes>(v15)))
                                                        }
                                                    }
                                                }
                                            }
                                            Err(v11) => {
                                                Err::<(u64, (List<Adt23>, List<Adt15>)), R<Result<(List<Adt15>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt15>, List<Adt23>), Bytes>(v11)))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Err::<(u64, (List<Adt23>, List<Adt15>)), R<Result<(List<Adt15>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt15>, List<Adt23>), Bytes>(v9)))
                                    }
                                }
                            }
                            Err(v6) => {
                                Err::<(u64, (List<Adt23>, List<Adt15>)), R<Result<(List<Adt15>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt15>, List<Adt23>), Bytes>(v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3222 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f460(p3234: u64, p3235: List<Adt23>, p3236: List<Adt15>) -> R<Result<(List<Adt15>, List<Adt23>), Bytes>> {
    let m3233 = (p3234, (p3235, p3236));
    {
        let mut m3233 = m3233;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3233;
                let m3226 = v0;
                if m3226 == 0 {
                    Err::<(u64, (List<Adt23>, List<Adt15>)), R<Result<(List<Adt15>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt15>, List<Adt23>), Bytes>(f8(Str::lit("an interface has more members than the input has tokens")))))
                } else {
                    let v3 = m3226 - 1;
                    if f353(v1.clone(), Str::lit("}")) {
                        let m3227 = f360(f355(v1), Str::lit(";"));
                        match m3227 {
                            Ok(v5) => {
                                Err::<(u64, (List<Adt23>, List<Adt15>)), R<Result<(List<Adt15>, List<Adt23>), Bytes>>>(Ok(Ok::<(List<Adt15>, List<Adt23>), Bytes>((f550(v2, List::<Adt15>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<Adt23>, List<Adt15>)), R<Result<(List<Adt15>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt15>, List<Adt23>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m3228 = f250(v1)?;
                        match m3228 {
                            Ok(v7) => {
                                let v8: List<Adt23> = if if f353(v7.clone(), Str::lit("stringifier")) {
                                    true
                                } else {
                                    f353(v7.clone(), Str::lit("inherit"))
                                } {
                                    f355(v7.clone())
                                } else {
                                    v7.clone()
                                };
                                let v9: List<Adt23> = if f353(v8.clone(), Str::lit("readonly")) {
                                    f355(v8)
                                } else {
                                    v8
                                };
                                if f353(v9.clone(), Str::lit("attribute")) {
                                    let m3229 = f365(v3, f355(v9), 0u64)?;
                                    match m3229 {
                                        Ok(v11) => {
                                            let m3230 = f359(f366(v11.clone()));
                                            match m3230 {
                                                Ok(v13) => {
                                                    let m3231 = f360(f361(v13.clone()), Str::lit(";"));
                                                    match m3231 {
                                                        Ok(v15) => {
                                                            Ok::<(u64, (List<Adt23>, List<Adt15>)), R<Result<(List<Adt15>, List<Adt23>), Bytes>>>((v3, (v15, List::cons(Adt15::C0(f363(v13), f468(f367(v11)), 0u64), v2))))
                                                        }
                                                        Err(v14) => {
                                                            Err::<(u64, (List<Adt23>, List<Adt15>)), R<Result<(List<Adt15>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt15>, List<Adt23>), Bytes>(v14)))
                                                        }
                                                    }
                                                }
                                                Err(v12) => {
                                                    Err::<(u64, (List<Adt23>, List<Adt15>)), R<Result<(List<Adt15>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt15>, List<Adt23>), Bytes>(v12)))
                                                }
                                            }
                                        }
                                        Err(v10) => {
                                            Err::<(u64, (List<Adt23>, List<Adt15>)), R<Result<(List<Adt15>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt15>, List<Adt23>), Bytes>(v10)))
                                        }
                                    }
                                } else {
                                    let m3232 = f461(v7)?;
                                    match m3232 {
                                        Ok(v17) => {
                                            Ok::<(u64, (List<Adt23>, List<Adt15>)), R<Result<(List<Adt15>, List<Adt23>), Bytes>>>((v3, (v17, v2)))
                                        }
                                        Err(v16) => {
                                            Err::<(u64, (List<Adt23>, List<Adt15>)), R<Result<(List<Adt15>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt15>, List<Adt23>), Bytes>(v16)))
                                        }
                                    }
                                }
                            }
                            Err(v6) => {
                                Err::<(u64, (List<Adt23>, List<Adt15>)), R<Result<(List<Adt15>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt15>, List<Adt23>), Bytes>(v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3233 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f461(v0: List<Adt23>) -> R<Result<List<Adt23>, Bytes>> {
    Ok(f354({
        let a3238 = {
            let a3237 = v0.clone();
            length_list(a3237)
        };
        let a3239 = 1u64;
        nat_add(a3238, a3239)?
    }, v0, List::<Bytes>::nil(), true))
}

pub fn f462(v0: List<Adt23>) -> Bytes {
    let m3240 = v0;
    match m3240.uncons() {
        None => {
            f8(Str::lit("the end of the input"))
        }
        Some((v1, _)) => {
            f7(List::cons(f8(Str::lit("'")), List::cons(f456(v1), List::cons(f8(Str::lit("'")), List::<Bytes>::nil()))))
        }
    }
}

pub fn f463(v0: Bytes) -> Bytes {
    if f204(v0.clone(), f8(Str::lit("_"))) {
        f45(v0.clone(), 1u64, {
            let a3242 = {
                let a3241 = v0;
                length_bytes(a3241)
            };
            let a3243 = 1u64;
            nat_sub(a3242, a3243)
        })
    } else {
        v0
    }
}

pub fn f464(v0: Adt16, v1: List<Adt23>) -> Result<Adt33, Bytes> {
    if f353(v1.clone(), Str::lit("?")) {
        Ok::<Adt33, Bytes>(Adt33::C0(f465(Str::lit("Option"), v0), f355(v1)))
    } else {
        Ok::<Adt33, Bytes>(Adt33::C0(v0, v1))
    }
}

pub fn f465(v0: Str, v1: Adt16) -> Adt16 {
    let v2: Adt16 = f468(v1);
    Adt16::C0(0u64, f7(List::cons(f8(Str::lit("(")), List::cons({
        let a3244 = v0;
        utf8_encode(a3244)
    }, List::cons(f8(Str::lit(" ")), List::cons(f268(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), f269(v2.clone()), f266(v2), Bytes::lit(&[]))
}

pub fn f466(v0: Bytes) -> Adt16 {
    Adt16::C0(0u64, Bytes::lit(&[]), List::<Bytes>::nil(), v0, Bytes::lit(&[]))
}

pub fn f467(v0: Bytes) -> Option<Adt16> {
    f551(v0, f552())
}

pub fn f468(v0: Adt16) -> Adt16 {
    let a3245 = f267(v0.clone());
    let a3246 = 0u64;
    if nat_eq(a3245, a3246) {
        v0
    } else {
        Adt16::C0(0u64, Bytes::lit(&[]), f269(v0.clone()), if f32(f266(v0.clone()), Bytes::lit(&[])) {
            f553()
        } else {
            f266(v0)
        }, Bytes::lit(&[]))
    }
}

pub fn f469(v0: Adt16) -> Bytes {
    let m3247 = v0;
    match m3247 {
        Adt16::C0(_, _, _, _, v5) => {
            v5
        }
    }
}

pub fn f470(p3264: u64, p3265: u64, p3266: u64, p3267: Bytes) -> Bytes {
    let m3263 = (p3264, (p3265, (p3266, p3267)));
    {
        let mut m3263 = m3263;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m3263;
                let m3248 = v0;
                if m3248 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m3248 - 1;
                    if f96({
                        let a3249 = v1;
                        let a3250 = 0u64;
                        nat_eq(a3249, a3250)
                    }, {
                        let a3251 = v2;
                        let a3252 = 0u64;
                        nat_eq(a3251, a3252)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a3253 = v1;
                            let a3254 = 16u64;
                            let a3255 = 0u64;
                            nat_quot(a3253, a3254, a3255)
                        }, ({
                            let a3256 = v2;
                            let a3257 = 1u64;
                            nat_sub(a3256, a3257)
                        }, {
                            let a3261 = f554({
                                let a3258 = v1;
                                let a3259 = 16u64;
                                let a3260 = 0u64;
                                nat_rem(a3258, a3259, a3260)
                            });
                            let a3262 = v3;
                            append_bytes(a3261, a3262)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m3263 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f471(v0: Adt15, v1: List<Adt15>) -> R<List<Adt15>> {
    let m3268 = v1;
    match m3268.uncons() {
        None => {
            Ok(List::cons(v0, List::<Adt15>::nil()))
        }
        Some((v2, v3)) => {
            if f555({
                let a3270 = {
                    let a3269 = f376(v0.clone());
                    length_bytes(a3269)
                };
                let a3271 = 1u64;
                nat_add(a3270, a3271)?
            }, f376(v0.clone()), f376(v2.clone()), 0u64)? {
                Ok(List::cons(v0, List::cons(v2, v3)))
            } else {
                Ok(List::cons(v2, f471(v0, v3)?))
            }
        }
    }
}

pub fn f472(v0: List<Adt14>, v1: Bytes) -> List<Adt15> {
    let m3272 = f181(v0.clone(), 1u64, v1.clone());
    match m3272 {
        None => {
            f373(v0, 6u64, v1)
        }
        Some(v2) => {
            f371(f372(v2), f373(v0, 6u64, v1))
        }
    }
}

pub fn f473(v0: Bytes) -> R<Option<Bytes>> {
    let m3274 = f556({
        let a3273 = v0.clone();
        length_bytes(a3273)
    }, v0.clone(), 0u64, Bytes::lit(&[]))?;
    match m3274 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a3276 = {
                let a3275 = v1.clone();
                length_bytes(a3275)
            };
            let a3277 = 0u64;
            if nat_eq(a3276, a3277) {
                let a3279 = {
                    let a3278 = v0;
                    length_bytes(a3278)
                };
                let a3280 = 0u64;
                if nat_eq(a3279, a3280) {
                    Ok(Some(f8(Str::lit("u"))))
                } else {
                    Ok(None::<Bytes>)
                }
            } else {
                Ok(Some(f475(v1)?))
            }
        }
    }
}

pub fn f474(v0: Bytes) -> R<Option<Bytes>> {
    if f557({
        let a3281 = v0.clone();
        length_bytes(a3281)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f558(v0)?;
        if f559(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m3284 = {
                let a3282 = v1.clone();
                let a3283 = 0u64;
                index_bytes(a3282, a3283)
            };
            match m3284 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f398(f56(v2)?) {
                        Ok(Some({
                            let a3285 = f8(Str::lit("n"));
                            let a3286 = v1;
                            append_bytes(a3285, a3286)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f560({
            let a3287 = v0.clone();
            length_bytes(a3287)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f475(v0: Bytes) -> R<Bytes> {
    if f173(v0.clone(), f561()) {
        f369(v0)
    } else {
        Ok(v0)
    }
}

pub fn f476(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3290 = {
        let a3288 = v0;
        let a3289 = v1;
        index_bytes(a3288, a3289)
    };
    match m3290 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f56(v2)?;
            if f96({
                let a3291 = 128u64;
                let a3292 = v3;
                nat_le(a3291, a3292)
            }, {
                let a3293 = v3;
                let a3294 = 191u64;
                nat_le(a3293, a3294)
            }) {
                Ok(Some({
                    let a3295 = v3;
                    let a3296 = 128u64;
                    nat_sub(a3295, a3296)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f477(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f96({
        let a3297 = v1;
        let a3298 = v0;
        nat_le(a3297, a3298)
    }, f96({
        let a3299 = v0;
        let a3300 = v2;
        nat_le(a3299, a3300)
    }, f153({
        let a3301 = v0;
        let a3302 = 55296u64;
        nat_lt(a3301, a3302)
    }, {
        let a3303 = 57343u64;
        let a3304 = v0;
        nat_lt(a3303, a3304)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f478() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{3}/\u{7f}\u{3}@\u{0}\u{3}\u{7f}}\u{4}\u{0}\u{0}C\u{7f}\u{7f}"))
}

pub fn f479(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a3305 = v1;
        let a3306 = 3u64;
        nat_mul(a3305, a3306)?
    };
    let a3317 = {
        let a3313 = {
            let a3307 = f94(v0.clone(), v2)?;
            let a3308 = 16384u64;
            nat_mul(a3307, a3308)?
        };
        let a3314 = {
            let a3311 = f94(v0.clone(), {
                let a3309 = v2;
                let a3310 = 1u64;
                nat_add(a3309, a3310)?
            })?;
            let a3312 = 128u64;
            nat_mul(a3311, a3312)?
        };
        nat_add(a3313, a3314)?
    };
    let a3318 = f94(v0, {
        let a3315 = v2;
        let a3316 = 2u64;
        nat_add(a3315, a3316)?
    })?;
    nat_add(a3317, a3318)
}

pub fn f480(v0: u64) -> R<bool> {
    f274(v0, f562())
}

pub fn f481(v0: u64) -> R<bool> {
    f274(v0, f563())
}

pub fn f482(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f564(v1, v0, v2)
}

pub fn f483(v0: Bytes, v1: u64) -> Bytes {
    f45(v0, v1, 1u64)
}

pub fn f484() -> Bytes {
    f8(Str::lit("\u{0}\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\u{9}\u{a}\u{b}\u{c}\u{d}\u{e}\u{f}\u{10}\u{11}\u{12}\u{13}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{1b}\u{1c}\u{1d}\u{1e}\u{1f} !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~\u{7f}"))
}

pub fn f485() -> Bytes {
    f8(Str::lit("\u{80}\u{c0}\u{100}\u{140}\u{180}\u{1c0}\u{200}\u{240}\u{280}\u{2c0}\u{300}\u{340}\u{380}\u{3c0}\u{400}\u{440}\u{480}\u{4c0}\u{500}\u{540}\u{580}\u{5c0}\u{600}\u{640}\u{680}\u{6c0}\u{700}\u{740}\u{780}\u{7c0}"))
}

pub fn f486(v0: u64) -> R<Bytes> {
    Ok(f483(f565(), {
        let a3324 = {
            let a3322 = {
                let a3319 = v0;
                let a3320 = 64u64;
                let a3321 = 0u64;
                nat_rem(a3319, a3320, a3321)
            };
            let a3323 = 2u64;
            nat_mul(a3322, a3323)?
        };
        let a3325 = 1u64;
        nat_add(a3324, a3325)?
    }))
}

pub fn f487() -> Bytes {
    f8(Str::lit("\u{800}\u{1000}\u{2000}\u{3000}\u{4000}\u{5000}\u{6000}\u{7000}\u{8000}\u{9000}\u{a000}\u{b000}\u{c000}\u{d000}\u{e000}\u{f000}"))
}

pub fn f488(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<Option<u64>> {
    if f566(f129(v0, v3)?, v1, v2)? {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3326 = v3;
            let a3327 = 1u64;
            nat_add(a3326, a3327)?
        }))
    }
}

pub fn f489(v0: Bytes) -> Result<Bytes, Bytes> {
    Err::<Bytes, Bytes>(v0)
}

pub fn f490() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f491(v0: Bytes) -> R<Option<Bytes>> {
    let m3328 = f191(v0.clone(), f8(Str::lit(" ")), 0u64)?;
    match m3328 {
        None => {
            let m3329 = f191(v0.clone(), f8(Str::lit(":")), 0u64)?;
            match m3329 {
                None => {
                    let v3: Bytes = f7(List::cons(f8(Str::lit(" ")), List::cons(v0, List::cons(f8(Str::lit(":")), List::<Bytes>::nil()))));
                    let m3330 = f191(f567(), v3.clone(), 0u64)?;
                    match m3330 {
                        None => {
                            Ok(None::<Bytes>)
                        }
                        Some(v4) => {
                            let v5: u64 = {
                                let a3332 = v4;
                                let a3333 = {
                                    let a3331 = v3;
                                    length_bytes(a3331)
                                };
                                nat_add(a3332, a3333)?
                            };
                            let m3334 = f191(f567(), f8(Str::lit(" ")), v5)?;
                            match m3334 {
                                None => {
                                    Ok(None::<Bytes>)
                                }
                                Some(v6) => {
                                    Ok(Some(f45(f567(), v5, {
                                        let a3335 = v6;
                                        let a3336 = v5;
                                        nat_sub(a3335, a3336)
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

pub fn f492(v0: Bytes) -> Bytes {
    if f40(v0.clone(), Bytes::lit(&[0x0d])) {
        f45(v0.clone(), 0u64, {
            let a3338 = {
                let a3337 = v0;
                length_bytes(a3337)
            };
            let a3339 = 1u64;
            nat_sub(a3338, a3339)
        })
    } else {
        v0
    }
}

pub fn f493() -> List<Bytes> {
    List::cons(f8(Str::lit("complexType")), List::cons(f8(Str::lit("simpleType")), List::cons(f8(Str::lit("element")), List::cons(f8(Str::lit("attribute")), List::cons(f8(Str::lit("attributeGroup")), List::cons(f8(Str::lit("group")), List::<Bytes>::nil()))))))
}

pub fn f494(v0: u64, v1: Adt25, v2: Adt26) -> R<Result<List<Adt28>, Bytes>> {
    let v3: Bytes = f211(match v2.clone() {
        Adt26::C0(_, _, h3340, _) => {
            h3340
        }
    })?;
    let v4: (Bytes, Bytes) = (match v2.clone() {
        Adt26::C0(_, h3341, _, _) => {
            h3341
        }
    }, match v2.clone() {
        Adt26::C0(_, _, h3342, _) => {
            h3342
        }
    });
    let v5: bool = f568(match v2.clone() {
        Adt26::C0(_, _, _, h3343) => {
            h3343
        }
    }, Str::lit("abstract"), Str::lit("true"));
    let m3345 = f569(match v1.clone() {
        Adt25::C0(h3344, _) => {
            h3344
        }
    }, v4, f306(v5))?;
    match m3345 {
        Ok(v7) => {
            if v5 {
                let m3346 = v7.clone();
                match m3346.uncons() {
                    None => {
                        Ok(Err::<List<Adt28>, Bytes>(f500(f8(Str::lit("an abstract complex type with no type derived from it in this schema set admits no instance (XSD 1.1 Part 1, sections 3.4.1 and 3.3.4.2)")), v3)))
                    }
                    Some((_, _)) => {
                        let m3347 = f570(v3, v7.clone(), f571(v7.clone())?, f571(v7)?)?;
                        match m3347 {
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
                let m3348 = v7.clone();
                match m3348.uncons() {
                    None => {
                        f572(v0, v1, v3, match v2.clone() {
                            Adt26::C0(_, h3349, _, _) => {
                                h3349
                            }
                        }, match v2 {
                            Adt26::C0(_, _, _, h3350) => {
                                h3350
                            }
                        })
                    }
                    Some((_, _)) => {
                        let v14: Bytes = f573(v3.clone());
                        let m3353 = f572(v0, v1, v14.clone(), match v2.clone() {
                            Adt26::C0(_, h3351, _, _) => {
                                h3351
                            }
                        }, match v2.clone() {
                            Adt26::C0(_, _, _, h3352) => {
                                h3352
                            }
                        })?;
                        match m3353 {
                            Ok(v16) => {
                                let m3355 = f570(v3, List::cons(match v2 {
                                    Adt26::C0(_, _, h3354, _) => {
                                        h3354
                                    }
                                }, v7.clone()), List::cons(v14.clone(), f571(v7.clone())?), List::cons(v14, f571(v7)?))?;
                                match m3355 {
                                    Ok(v18) => {
                                        Ok(Ok::<List<Adt28>, Bytes>(f403(v16, List::cons(v18, List::<Adt28>::nil()))))
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

pub fn f495(v0: Bytes, v1: Adt27, v2: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3358 = f210(v0.clone(), match v1.clone() {
        Adt27::C0(_, _, _, h3356, _) => {
            h3356
        }
    }, match v1 {
        Adt27::C0(_, _, _, _, h3357) => {
            h3357
        }
    }, v2)?;
    match m3358 {
        Ok(v4) => {
            Ok(Ok::<Adt28, Bytes>(Adt28::C0(f211(v0)?, match v4.clone() {
                Adt24::C0(_, h3359) => {
                    h3359
                }
            }, match v4 {
                Adt24::C0(h3360, _) => {
                    h3360
                }
            }, List::<Bytes>::nil())))
        }
        Err(v3) => {
            Ok(Err::<Adt28, Bytes>(v3))
        }
    }
}

pub fn f496(v0: u64, v1: Adt25, v2: Bytes, v3: Bytes, v4: Adt27) -> R<Result<Adt35, Bytes>> {
    let m3361 = v0;
    if m3361 == 0 {
        Ok(Err::<Adt35, Bytes>(f500(f8(Str::lit("the element declarations nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m3361 - 1;
        let m3362 = f308(v4.clone(), Str::lit("type"));
        match m3362 {
            None => {
                let m3366 = f498(v4.clone())?;
                match m3366 {
                    Ok(v10) => {
                        let m3367 = f499(v10, List::cons(f8(Str::lit("complexType")), List::cons(f8(Str::lit("simpleType")), List::<Bytes>::nil())));
                        match m3367 {
                            None => {
                                Ok(Ok::<Adt35, Bytes>(Adt35::C0(Adt24::C0(f575(v4, f406()), List::cons(f406(), List::<Bytes>::nil())), List::<Adt28>::nil())))
                            }
                            Some(v11) => {
                                if f307(v11.clone(), Str::lit("complexType")) {
                                    let m3368 = f572(v5, v1, v2.clone(), v3, v11)?;
                                    match m3368 {
                                        Ok(v13) => {
                                            Ok(Ok::<Adt35, Bytes>(Adt35::C0(Adt24::C0(f575(v4, v2.clone()), List::cons(v2, List::<Bytes>::nil())), v13)))
                                        }
                                        Err(v12) => {
                                            Ok(Err::<Adt35, Bytes>(v12))
                                        }
                                    }
                                } else {
                                    let m3369 = f495(v2.clone(), v11, v3)?;
                                    match m3369 {
                                        Ok(v15) => {
                                            Ok(Ok::<Adt35, Bytes>(Adt35::C0(Adt24::C0(f575(v4, v2.clone()), List::cons(v2, List::<Bytes>::nil())), List::cons(v15, List::<Adt28>::nil()))))
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
                let m3363 = f574(v1, v4.clone(), v6)?;
                match m3363 {
                    Ok(v8) => {
                        Ok(Ok::<Adt35, Bytes>(Adt35::C0(Adt24::C0(f575(v4, match v8.clone() {
                            Adt24::C0(h3364, _) => {
                                h3364
                            }
                        }), match v8 {
                            Adt24::C0(_, h3365) => {
                                h3365
                            }
                        }), List::<Adt28>::nil())))
                    }
                    Err(v7) => {
                        Ok(Err::<Adt35, Bytes>(f500(v7, v2)))
                    }
                }
            }
        }
    }
}

pub fn f497(v0: Adt25, v1: Bytes, v2: Bytes, v3: Adt27) -> R<Result<Adt35, Bytes>> {
    let m3370 = f308(v3.clone(), Str::lit("type"));
    match m3370 {
        None => {
            let m3372 = f498(v3)?;
            match m3372 {
                Ok(v8) => {
                    let m3373 = f499(v8, List::cons(f8(Str::lit("simpleType")), List::<Bytes>::nil()));
                    match m3373 {
                        None => {
                            Ok(Err::<Adt35, Bytes>(f500(f8(Str::lit("an attribute declaration with no type has the type xs:anySimpleType, which is not translated, because it is the union of every simple value space (XSD 1.1 Part 1, section 3.2.2.1; Part 2, section 3.2.1)")), v2)))
                        }
                        Some(v9) => {
                            let m3374 = f495(v2.clone(), v9, v1)?;
                            match m3374 {
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
            let m3371 = f574(v0, v3, v4)?;
            match m3371 {
                Ok(v6) => {
                    Ok(Ok::<Adt35, Bytes>(Adt35::C0(v6, List::<Adt28>::nil())))
                }
                Err(v5) => {
                    Ok(Err::<Adt35, Bytes>(f500(v5, v2)))
                }
            }
        }
    }
}

pub fn f498(v0: Adt27) -> R<Result<List<Adt27>, Bytes>> {
    f224(match v0.clone() {
        Adt27::C0(_, _, _, h3375, _) => {
            h3375
        }
    }, match v0 {
        Adt27::C0(_, _, _, _, h3376) => {
            h3376
        }
    })
}

pub fn f499(p3380: List<Adt27>, v1: List<Bytes>) -> Option<Adt27> {
    let m3379 = p3380;
    {
        let mut m3379 = m3379;
        loop {
            let step = {
                let v0 = m3379;
                let m3377 = v0;
                match m3377.uncons() {
                    None => {
                        Err::<List<Adt27>, Option<Adt27>>(None::<Adt27>)
                    }
                    Some((v2, v3)) => {
                        if f173(match v2.clone() {
                            Adt27::C0(_, h3378, _, _, _) => {
                                h3378
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
                Ok(next) => { m3379 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f500(v0: Bytes, v1: Bytes) -> Bytes {
    f220(v0, f8(Str::lit(" at ")), v1)
}

pub fn f501(v0: u64, v1: Adt25, v2: Bytes, v3: Bytes, v4: Adt27) -> R<Result<List<Adt28>, Bytes>> {
    let m3381 = v0;
    if m3381 == 0 {
        Ok(Err::<List<Adt28>, Bytes>(f500(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m3381 - 1;
        let m3382 = f498(v4.clone())?;
        match m3382 {
            Ok(v7) => {
                if f307(v4, Str::lit("choice")) {
                    let m3383 = f576(v5, v1, v2.clone(), v3, false, v7, f503())?;
                    match m3383 {
                        Ok(v9) => {
                            let m3387 = f570(v2, f577(match v9.clone() {
                                Adt36::C0(h3384, _, _, _, _) => {
                                    h3384
                                }
                            }), f578(match v9.clone() {
                                Adt36::C0(h3385, _, _, _, _) => {
                                    h3385
                                }
                            }), f579(match v9.clone() {
                                Adt36::C0(h3386, _, _, _, _) => {
                                    h3386
                                }
                            }))?;
                            match m3387 {
                                Ok(v11) => {
                                    Ok(Ok::<List<Adt28>, Bytes>(f403(match v9 {
                                        Adt36::C0(_, h3388, _, _, _) => {
                                            h3388
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
                    let m3389 = f576(v5, v1, v2.clone(), v3, true, v7, f503())?;
                    match m3389 {
                        Ok(v13) => {
                            let m3391 = f580(v2, match v13.clone() {
                                Adt36::C0(h3390, _, _, _, _) => {
                                    h3390
                                }
                            })?;
                            match m3391 {
                                Ok(v15) => {
                                    Ok(Ok::<List<Adt28>, Bytes>(f403(match v13 {
                                        Adt36::C0(_, h3392, _, _, _) => {
                                            h3392
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

pub fn f502(v0: u64, v1: Adt25, v2: Bytes, v3: Bytes, v4: List<Adt27>, v5: Adt36) -> R<Result<Adt36, Bytes>> {
    let m3393 = v0;
    if m3393 == 0 {
        Ok(Err::<Adt36, Bytes>(f500(f8(Str::lit("the attribute group references nest deeper than the schema set has components, so they refer to each other in a cycle (XSD 1.1 Part 1, section 3.6.3)")), v2)))
    } else {
        let v6 = m3393 - 1;
        let m3394 = v4;
        match m3394.uncons() {
            None => {
                Ok(Ok::<Adt36, Bytes>(v5))
            }
            Some((v7, v8)) => {
                if f307(v7.clone(), Str::lit("attribute")) {
                    if f568(v7.clone(), Str::lit("use"), Str::lit("prohibited")) {
                        f502(v6, v1, v2, v3, v8, v5)
                    } else {
                        let v9: bool = f568(v7.clone(), Str::lit("use"), Str::lit("required"));
                        let m3395 = f308(v7.clone(), Str::lit("ref"));
                        match m3395 {
                            None => {
                                let m3405 = f308(v7.clone(), Str::lit("name"));
                                match m3405 {
                                    None => {
                                        Ok(Err::<Adt36, Bytes>(f500(f8(Str::lit("a local attribute declaration must have a name or a ref (XSD 1.1 Part 1, section 3.2.3)")), v2)))
                                    }
                                    Some(v16) => {
                                        let m3408 = f497(v1.clone(), v3.clone(), {
                                            let a3406 = v2.clone();
                                            let a3407 = f211(v16.clone())?;
                                            append_bytes(a3406, a3407)
                                        }, v7)?;
                                        match m3408 {
                                            Ok(v18) => {
                                                f502(v6, v1, v2, v3, v8, f582(v5, v16, v18, v9))
                                            }
                                            Err(v17) => {
                                                Ok(Err::<Adt36, Bytes>(v17))
                                            }
                                        }
                                    }
                                }
                            }
                            Some(v10) => {
                                let m3397 = f133(match v7 {
                                    Adt27::C0(_, _, _, _, h3396) => {
                                        h3396
                                    }
                                }, v10.clone())?;
                                match m3397 {
                                    Ok(v12) => {
                                        if f32({
                                            let (h3398, _) = v12.clone();
                                            h3398
                                        }, f202()) {
                                            Ok(Err::<Adt36, Bytes>(f500(f220(f8(Str::lit("the attribute ")), v10, f8(Str::lit(" is declared by the schema document for the XML namespace, which this schema set does not import (XSD 1.1 Part 1, sections 3.2.2.3 and 4.2.6)"))), v2)))
                                        } else {
                                            let m3399 = f581(v1.clone(), Str::lit("attribute"), v12.clone());
                                            match m3399 {
                                                None => {
                                                    Ok(Err::<Adt36, Bytes>(f500(f220(f8(Str::lit("the attribute ")), v10, f8(Str::lit(" is not declared by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                                }
                                                Some(v13) => {
                                                    let m3403 = f497(v1.clone(), match v13.clone() {
                                                        Adt26::C0(_, h3400, _, _) => {
                                                            h3400
                                                        }
                                                    }, f211({
                                                        let (_, h3401) = v12.clone();
                                                        h3401
                                                    })?, match v13 {
                                                        Adt26::C0(_, _, _, h3402) => {
                                                            h3402
                                                        }
                                                    })?;
                                                    match m3403 {
                                                        Ok(v15) => {
                                                            f502(v6, v1, v2, v3, v8, f582(v5, {
                                                                let (_, h3404) = v12;
                                                                h3404
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
                                        Ok(Err::<Adt36, Bytes>(f500(v11, v2)))
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if f307(v7.clone(), Str::lit("attributeGroup")) {
                        let m3409 = f308(v7.clone(), Str::lit("ref"));
                        match m3409 {
                            None => {
                                Ok(Err::<Adt36, Bytes>(f500(f8(Str::lit("an attribute group reference must have a ref (XSD 1.1 Part 1, section 3.6.2)")), v2)))
                            }
                            Some(v19) => {
                                let m3411 = f133(match v7 {
                                    Adt27::C0(_, _, _, _, h3410) => {
                                        h3410
                                    }
                                }, v19.clone())?;
                                match m3411 {
                                    Ok(v21) => {
                                        let m3412 = f581(v1.clone(), Str::lit("attributeGroup"), v21.clone());
                                        match m3412 {
                                            None => {
                                                Ok(Err::<Adt36, Bytes>(f500(f220(f8(Str::lit("the attribute group ")), v19, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                            }
                                            Some(v22) => {
                                                let m3414 = f498(match v22.clone() {
                                                    Adt26::C0(_, _, _, h3413) => {
                                                        h3413
                                                    }
                                                })?;
                                                match m3414 {
                                                    Ok(v24) => {
                                                        let m3417 = f502(v6, v1.clone(), f211({
                                                            let (_, h3415) = v21;
                                                            h3415
                                                        })?, match v22 {
                                                            Adt26::C0(_, h3416, _, _) => {
                                                                h3416
                                                            }
                                                        }, v24, v5)?;
                                                        match m3417 {
                                                            Ok(v26) => {
                                                                f502(v6, v1, v2, v3, v8, v26)
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
                                        Ok(Err::<Adt36, Bytes>(f500(v20, v2)))
                                    }
                                }
                            }
                        }
                    } else {
                        if f307(v7, Str::lit("anyAttribute")) {
                            f502(v6, v1, v2, v3, v8, f583(v5, Adt37::C0(f8(Str::lit("anyAttribute")), f584(), List::<Bytes>::nil())))
                        } else {
                            f502(v6, v1, v2, v3, v8, v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f503() -> Adt36 {
    Adt36::C0(List::<Adt37>::nil(), List::<Adt28>::nil(), 0u64, 0u64, 0u64)
}

pub fn f504(v0: List<Adt28>, v1: List<Bytes>, v2: Bytes) -> Result<List<Bytes>, Bytes> {
    let m3418 = v1;
    match m3418.uncons() {
        None => {
            Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let m3419 = f405(v0.clone(), v3.clone());
            match m3419 {
                None => {
                    Err::<List<Bytes>, Bytes>(f505(v3))
                }
                Some(v5) => {
                    let m3421 = match v5.clone() {
                        Adt28::C0(_, _, _, h3420) => {
                            h3420
                        }
                    };
                    match m3421.uncons() {
                        None => {
                            Err::<List<Bytes>, Bytes>(f505(v3))
                        }
                        Some((_, _)) => {
                            let m3422 = f504(v0, v4, v2.clone());
                            match m3422 {
                                Ok(v9) => {
                                    Ok::<List<Bytes>, Bytes>(List::cons(f585(v3, match v5 {
                                        Adt28::C0(_, _, _, h3423) => {
                                            h3423
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

pub fn f505(v0: Bytes) -> Bytes {
    f500(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)")), v0)
}

pub fn f506(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m3424 = v0;
    match m3424.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f173(v2.clone(), v1.clone()) {
                f506(v3, v1)
            } else {
                List::cons(v2, f506(v3, v1))
            }
        }
    }
}

pub fn f507(p3429: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m3428 = p3429;
    {
        let mut m3428 = m3428;
        loop {
            let step = {
                let v0 = m3428;
                let m3425 = v0;
                match m3425.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h3426, _) = v2;
                            h3426
                        }, v1.clone()) {
                            let (_, h3427) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h3427)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3428 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f508(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f586(v0, v1)
}

pub fn f509(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f314(f232(v0, v1)?) {
        Ok(Some({
            let a3430 = v1;
            let a3431 = 1u64;
            nat_add(a3430, a3431)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f510(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f587(v0, v1)
}

pub fn f511(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a3432 = f232(v0, v1)?;
    let a3433 = 32u64;
    if nat_eq(a3432, a3433) {
        Ok(Some({
            let a3434 = v1;
            let a3435 = 1u64;
            nat_add(a3434, a3435)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f512(v0: u64) -> bool {
    let a3436 = v0;
    let a3437 = 44u64;
    if if if nat_eq(a3436, a3437) {
        true
    } else {
        let a3438 = v0;
        let a3439 = 91u64;
        nat_eq(a3438, a3439)
    } {
        true
    } else {
        let a3440 = v0;
        let a3441 = 93u64;
        if nat_eq(a3440, a3441) {
            true
        } else {
            let a3442 = v0;
            let a3443 = 123u64;
            nat_eq(a3442, a3443)
        }
    } {
        true
    } else {
        let a3444 = v0;
        let a3445 = 125u64;
        nat_eq(a3444, a3445)
    }
}

pub fn f513(v0: u64) -> u64 {
    let a3446 = v0;
    let a3447 = 45u64;
    if nat_eq(a3446, a3447) {
        1u64
    } else {
        let a3448 = v0;
        let a3449 = 43u64;
        if nat_eq(a3448, a3449) {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f514(v0: u64) -> u64 {
    let a3450 = 49u64;
    let a3451 = v0;
    if if nat_le(a3450, a3451) {
        let a3452 = v0;
        let a3453 = 57u64;
        nat_le(a3452, a3453)
    } else {
        false
    } {
        let a3454 = v0;
        let a3455 = 48u64;
        nat_sub(a3454, a3455)
    } else {
        0u64
    }
}

pub fn f515(v0: Bytes, v1: u64, v2: u64, v3: bool, v4: u64) -> R<Adt22> {
    let m3456 = f588(v0.clone(), v1, v2)?;
    match m3456 {
        Adt38::C0(v6, v7) => {
            let v8: u64 = v7;
            let v9: u64 = f589(v0.clone(), v8, v2)?;
            if f590(v0.clone(), v8, v2)? {
                let m3460 = f591(v0.clone(), v8, v2, v3, {
                    let a3458 = {
                        let a3457 = v0;
                        length_bytes(a3457)
                    };
                    let a3459 = 2u64;
                    nat_add(a3458, a3459)?
                }, Bytes::lit(&[]), v6)?;
                match m3460 {
                    Adt39::C0(v11, v12, v13) => {
                        Ok(Adt22::C0(Adt2::C5(f7(List::cons(v11, List::cons({
                            let a3461 = v4;
                            let a3462 = 1u64;
                            if nat_eq(a3461, a3462) {
                                Bytes::lit(&[])
                            } else {
                                Bytes::lit(&[0x0a])
                            }
                        }, List::cons({
                            let a3463 = v4;
                            let a3464 = 2u64;
                            if nat_eq(a3463, a3464) {
                                f517(v12)
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
                    let a3465 = v4;
                    let a3466 = 2u64;
                    if nat_eq(a3465, a3466) {
                        f517(v6)
                    } else {
                        Bytes::lit(&[])
                    }
                }), {
                    let a3467 = f232(v0, v9)?;
                    let a3468 = 256u64;
                    if nat_eq(a3467, a3468) {
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

pub fn f516(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f592(v0, v1)
}

pub fn f517(v0: u64) -> Bytes {
    let m3469 = v0;
    if m3469 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m3469 - 1;
        let a3470 = Bytes::lit(&[0x0a]);
        let a3471 = f517(v1);
        append_bytes(a3470, a3471)
    }
}

pub fn f518(v0: Bytes, v1: u64) -> R<u64> {
    let (_, h3472) = {
        let (h3473, _) = f240(Fn6::F593(v0.clone()), {
            let a3475 = {
                let a3474 = v0;
                length_bytes(a3474)
            };
            let a3476 = 1u64;
            nat_add(a3475, a3476)?
        }, (v1, 0u64))?;
        h3473
    };
    Ok(h3472)
}

pub fn f519(v0: u64, v1: u64) -> u64 {
    let a3477 = v0;
    let a3478 = v1;
    if nat_lt(a3477, a3478) {
        v1
    } else {
        v0
    }
}

pub fn f520(v0: Bytes, p3537: u64, v2: u64, v3: bool, p3538: u64, p3539: Bytes) -> R<Adt34> {
    let m3536 = (p3537, (p3538, p3539));
    {
        let mut m3536 = m3536;
        loop {
            let step = {
                let (v1, (v4, v5)) = m3536;
                let m3479 = v4;
                if m3479 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C1(f125(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                } else {
                    let v6 = m3479 - 1;
                    let v7: u64 = f232(v0.clone(), v1)?;
                    let a3480 = v7;
                    let a3481 = 34u64;
                    if nat_eq(a3480, a3481) {
                        Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C0(v5, {
                            let a3482 = v1;
                            let a3483 = 1u64;
                            nat_add(a3482, a3483)?
                        })))
                    } else {
                        let a3484 = v7;
                        let a3485 = 256u64;
                        if nat_eq(a3484, a3485) {
                            Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C1(f125(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                        } else {
                            let a3486 = v7;
                            let a3487 = 92u64;
                            if nat_eq(a3486, a3487) {
                                let v8: u64 = f232(v0.clone(), {
                                    let a3488 = v1;
                                    let a3489 = 1u64;
                                    nat_add(a3488, a3489)?
                                })?;
                                let m3490 = f594(v8)?;
                                match m3490 {
                                    None => {
                                        let a3495 = v8;
                                        let a3496 = 10u64;
                                        if nat_eq(a3495, a3496) {
                                            if v3 {
                                                Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C1(f595(v1)?)))
                                            } else {
                                                let m3499 = f596(v0.clone(), {
                                                    let a3497 = v1;
                                                    let a3498 = 1u64;
                                                    nat_add(a3497, a3498)?
                                                }, v2, v6, 0u64)?;
                                                match m3499 {
                                                    Adt38::C0(v11, v12) => {
                                                        Ok::<(u64, (u64, Bytes)), R<Adt34>>((v12, (v6, {
                                                            let a3500 = v5;
                                                            let a3501 = f517(v11);
                                                            append_bytes(a3500, a3501)
                                                        })))
                                                    }
                                                    Adt38::C1(v10) => {
                                                        Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C1(v10)))
                                                    }
                                                }
                                            }
                                        } else {
                                            let v13: u64 = {
                                                let a3502 = v8;
                                                let a3503 = 120u64;
                                                if nat_eq(a3502, a3503) {
                                                    2u64
                                                } else {
                                                    let a3504 = v8;
                                                    let a3505 = 117u64;
                                                    if nat_eq(a3504, a3505) {
                                                        4u64
                                                    } else {
                                                        let a3506 = v8;
                                                        let a3507 = 85u64;
                                                        if nat_eq(a3506, a3507) {
                                                            8u64
                                                        } else {
                                                            0u64
                                                        }
                                                    }
                                                }
                                            };
                                            let a3508 = v13;
                                            let a3509 = 0u64;
                                            if nat_eq(a3508, a3509) {
                                                Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C1(f125(Str::lit("an escape must be one of YAML 1.2.2 [42] to [62]"), v1)?)))
                                            } else {
                                                let m3512 = f597(v0.clone(), {
                                                    let a3510 = v1;
                                                    let a3511 = 2u64;
                                                    nat_add(a3510, a3511)?
                                                }, v13, 0u64)?;
                                                match m3512 {
                                                    None => {
                                                        Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C1(f125(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                    }
                                                    Some(v14) => {
                                                        let a3513 = 1114111u64;
                                                        let a3514 = v14;
                                                        if if nat_lt(a3513, a3514) {
                                                            true
                                                        } else {
                                                            let a3515 = 55296u64;
                                                            let a3516 = v14;
                                                            if nat_le(a3515, a3516) {
                                                                let a3517 = v14;
                                                                let a3518 = 57343u64;
                                                                nat_le(a3517, a3518)
                                                            } else {
                                                                false
                                                            }
                                                        } {
                                                            Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C1(f125(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                        } else {
                                                            Ok::<(u64, (u64, Bytes)), R<Adt34>>(({
                                                                let a3521 = {
                                                                    let a3519 = v1;
                                                                    let a3520 = 2u64;
                                                                    nat_add(a3519, a3520)?
                                                                };
                                                                let a3522 = v13;
                                                                nat_add(a3521, a3522)?
                                                            }, (v6, {
                                                                let a3523 = v5;
                                                                let a3524 = f389(v14)?;
                                                                append_bytes(a3523, a3524)
                                                            })))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Some(v9) => {
                                        Ok::<(u64, (u64, Bytes)), R<Adt34>>(({
                                            let a3491 = v1;
                                            let a3492 = 2u64;
                                            nat_add(a3491, a3492)?
                                        }, (v6, {
                                            let a3493 = v5;
                                            let a3494 = f389(v9)?;
                                            append_bytes(a3493, a3494)
                                        })))
                                    }
                                }
                            } else {
                                if if f314(v7) {
                                    true
                                } else {
                                    let a3525 = v7;
                                    let a3526 = 10u64;
                                    nat_eq(a3525, a3526)
                                } {
                                    let v15: u64 = f317(v0.clone(), v1)?;
                                    let a3527 = f232(v0.clone(), v15)?;
                                    let a3528 = 10u64;
                                    if nat_eq(a3527, a3528) {
                                        if v3 {
                                            Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C1(f595(v1)?)))
                                        } else {
                                            let m3529 = f596(v0.clone(), v15, v2, v6, 0u64)?;
                                            match m3529 {
                                                Adt38::C0(v17, v18) => {
                                                    Ok::<(u64, (u64, Bytes)), R<Adt34>>((v18, (v6, {
                                                        let a3530 = v5;
                                                        let a3531 = f528(v17);
                                                        append_bytes(a3530, a3531)
                                                    })))
                                                }
                                                Adt38::C1(v16) => {
                                                    Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C1(v16)))
                                                }
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (u64, Bytes)), R<Adt34>>((v15, (v6, {
                                            let a3532 = v5;
                                            let a3533 = f326(v0.clone(), v1, v15);
                                            append_bytes(a3532, a3533)
                                        })))
                                    }
                                } else {
                                    let v19: u64 = f598(v0.clone(), v1)?;
                                    Ok::<(u64, (u64, Bytes)), R<Adt34>>((v19, (v6, {
                                        let a3534 = v5;
                                        let a3535 = f326(v0.clone(), v1, v19);
                                        append_bytes(a3534, a3535)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3536 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f521(v0: Bytes, p3567: u64, v2: u64, v3: bool, p3568: u64, p3569: Bytes) -> R<Adt34> {
    let m3566 = (p3567, (p3568, p3569));
    {
        let mut m3566 = m3566;
        loop {
            let step = {
                let (v1, (v4, v5)) = m3566;
                let m3540 = v4;
                if m3540 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C1(f125(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                } else {
                    let v6 = m3540 - 1;
                    let v7: u64 = f232(v0.clone(), v1)?;
                    let a3541 = v7;
                    let a3542 = 39u64;
                    if nat_eq(a3541, a3542) {
                        let a3545 = f232(v0.clone(), {
                            let a3543 = v1;
                            let a3544 = 1u64;
                            nat_add(a3543, a3544)?
                        })?;
                        let a3546 = 39u64;
                        if nat_eq(a3545, a3546) {
                            Ok::<(u64, (u64, Bytes)), R<Adt34>>(({
                                let a3547 = v1;
                                let a3548 = 2u64;
                                nat_add(a3547, a3548)?
                            }, (v6, {
                                let a3549 = v5;
                                let a3550 = Bytes::lit(&[0x27]);
                                append_bytes(a3549, a3550)
                            })))
                        } else {
                            Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C0(v5, {
                                let a3551 = v1;
                                let a3552 = 1u64;
                                nat_add(a3551, a3552)?
                            })))
                        }
                    } else {
                        let a3553 = v7;
                        let a3554 = 256u64;
                        if nat_eq(a3553, a3554) {
                            Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C1(f125(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                        } else {
                            if if f314(v7) {
                                true
                            } else {
                                let a3555 = v7;
                                let a3556 = 10u64;
                                nat_eq(a3555, a3556)
                            } {
                                let v8: u64 = f317(v0.clone(), v1)?;
                                let a3557 = f232(v0.clone(), v8)?;
                                let a3558 = 10u64;
                                if nat_eq(a3557, a3558) {
                                    if v3 {
                                        Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C1(f595(v1)?)))
                                    } else {
                                        let m3559 = f596(v0.clone(), v8, v2, v6, 0u64)?;
                                        match m3559 {
                                            Adt38::C0(v10, v11) => {
                                                Ok::<(u64, (u64, Bytes)), R<Adt34>>((v11, (v6, {
                                                    let a3560 = v5;
                                                    let a3561 = f528(v10);
                                                    append_bytes(a3560, a3561)
                                                })))
                                            }
                                            Adt38::C1(v9) => {
                                                Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C1(v9)))
                                            }
                                        }
                                    }
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Adt34>>((v8, (v6, {
                                        let a3562 = v5;
                                        let a3563 = f326(v0.clone(), v1, v8);
                                        append_bytes(a3562, a3563)
                                    })))
                                }
                            } else {
                                let v12: u64 = f599(v0.clone(), v1)?;
                                Ok::<(u64, (u64, Bytes)), R<Adt34>>((v12, (v6, {
                                    let a3564 = v5;
                                    let a3565 = f326(v0.clone(), v1, v12);
                                    append_bytes(a3564, a3565)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3566 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f522(v0: Bytes, v1: u64, v2: u64) -> R<Result<u64, Bytes>> {
    let v3: (u64, bool) = {
        let (h3570, _) = f600(Fn12::F601(v2, v0.clone()), {
            let a3572 = {
                let a3571 = v0.clone();
                length_bytes(a3571)
            };
            let a3573 = 1u64;
            nat_add(a3572, a3573)?
        }, (v1, false))?;
        h3570
    };
    let (_, h3574) = v3;
    if h3574 {
        if f230(v0, {
            let (h3575, _) = v3;
            h3575
        })? {
            Ok(Err::<u64, Bytes>(f125(Str::lit("a document marker cannot appear inside a flow collection (YAML 1.2.2 [206])"), {
                let (h3576, _) = v3;
                h3576
            })?))
        } else {
            Ok(Err::<u64, Bytes>(f125(Str::lit("a line of a flow collection is indented less than its node (YAML 1.2.2 [80])"), {
                let (h3577, _) = v3;
                h3577
            })?))
        }
    } else {
        Ok(Ok::<u64, Bytes>({
            let (h3578, _) = v3;
            h3578
        }))
    }
}

pub fn f523(v0: Bytes, v1: u64, v2: u64) -> R<Adt34> {
    let v3: u64 = f232(v0.clone(), v1)?;
    let a3579 = v3;
    let a3580 = 34u64;
    if if nat_eq(a3579, a3580) {
        true
    } else {
        let a3581 = v3;
        let a3582 = 39u64;
        nat_eq(a3581, a3582)
    } {
        f425(v0, v1, v2, false)
    } else {
        let m3583 = f322(v0.clone(), v1)?;
        match m3583 {
            None => {
                let a3584 = v3;
                let a3585 = 91u64;
                if if nat_eq(a3584, a3585) {
                    true
                } else {
                    let a3586 = v3;
                    let a3587 = 123u64;
                    nat_eq(a3586, a3587)
                } {
                    Ok(Adt34::C1(f125(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [142])"), v1)?))
                } else {
                    if f427(v0.clone(), v1, true)? {
                        let v5: u64 = f428(v0.clone(), v1, true)?;
                        let v6: u64 = f317(v0.clone(), v5)?;
                        let a3588 = f232(v0.clone(), v6)?;
                        let a3589 = 10u64;
                        if nat_eq(a3588, a3589) {
                            let m3590 = f522(v0.clone(), v6, v2)?;
                            match m3590 {
                                Ok(v8) => {
                                    let v9: u64 = f232(v0.clone(), v8)?;
                                    let a3591 = v9;
                                    let a3592 = 58u64;
                                    if if if nat_eq(a3591, a3592) {
                                        true
                                    } else {
                                        let a3593 = v9;
                                        let a3594 = 44u64;
                                        nat_eq(a3593, a3594)
                                    } {
                                        true
                                    } else {
                                        let a3595 = v9;
                                        let a3596 = 125u64;
                                        nat_eq(a3595, a3596)
                                    } {
                                        Ok(Adt34::C0(f326(v0, v1, v5), v5))
                                    } else {
                                        Ok(Adt34::C1(f125(Str::lit("a multi-line plain scalar inside a flow collection is not read (YAML 1.2.2 [135])"), v6)?))
                                    }
                                }
                                Err(v7) => {
                                    Ok(Adt34::C1(v7))
                                }
                            }
                        } else {
                            Ok(Adt34::C0(f326(v0, v1, v5), v5))
                        }
                    } else {
                        Ok(Adt34::C1(f125(Str::lit("a flow mapping key must be a scalar (YAML 1.2.2 [142])"), v1)?))
                    }
                }
            }
            Some(v4) => {
                Ok(Adt34::C1(v4))
            }
        }
    }
}

pub fn f524(v0: u64) -> R<bool> {
    let a3597 = f325(f8(Str::lit(",[]{}-?:#&*!|>'\"@`")), v0)?;
    let a3598 = 18u64;
    if nat_lt(a3597, a3598) {
        Ok(true)
    } else {
        let a3599 = v0;
        let a3600 = 37u64;
        Ok(nat_eq(a3599, a3600))
    }
}

pub fn f525(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f602(v1, v0, v2)
}

pub fn f526(v0: u64) -> R<Bytes> {
    f125(Str::lit("a mapping value is not allowed here: a plain scalar holds ': ' (YAML 1.2.2 [130])"), v0)
}

pub fn f527(v0: Bytes, v1: u64) -> R<(u64, u64)> {
    let (h3601, _) = f240(Fn6::F603(v0.clone()), {
        let a3603 = {
            let a3602 = v0;
            length_bytes(a3602)
        };
        let a3604 = 1u64;
        nat_add(a3603, a3604)?
    }, (v1, 0u64))?;
    Ok(h3601)
}

pub fn f528(v0: u64) -> Bytes {
    let a3605 = v0;
    let a3606 = 0u64;
    if nat_eq(a3605, a3606) {
        Bytes::lit(&[0x20])
    } else {
        f517(v0)
    }
}

pub fn f529(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt22> {
    let v4: (u64, u64) = f604(v0.clone(), v1, 2u64)?;
    let a3609 = {
        let (h3607, _) = v4;
        h3607
    };
    let a3610 = {
        let a3608 = v0.clone();
        length_bytes(a3608)
    };
    if nat_eq(a3609, a3610) {
        if f157(false, {
            let (_, h3611) = v4;
            h3611
        }) {
            Ok(Adt22::C0(Adt2::C3(false, {
                let (_, h3612) = v4;
                h3612
            }), v3))
        } else {
            Ok(Adt22::C1(f531(v2)?))
        }
    } else {
        Ok(Adt22::C0(Adt2::C5(v0), v3))
    }
}

pub fn f530(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a3613 = v0.clone();
        length_bytes(a3613)
    };
    let v2: u64 = f232(v0.clone(), 0u64)?;
    let v3: u64 = {
        let a3614 = v2;
        let a3615 = 43u64;
        if if nat_eq(a3614, a3615) {
            true
        } else {
            let a3616 = v2;
            let a3617 = 45u64;
            nat_eq(a3616, a3617)
        } {
            1u64
        } else {
            0u64
        }
    };
    let v4: u64 = f152(v0.clone(), v3)?;
    let v5: bool = {
        let a3618 = f232(v0.clone(), v4)?;
        let a3619 = 46u64;
        nat_eq(a3618, a3619)
    };
    let v6: u64 = if v5 {
        let a3620 = v4;
        let a3621 = 1u64;
        nat_add(a3620, a3621)?
    } else {
        v4
    };
    let v7: u64 = if v5 {
        f152(v0.clone(), v6)?
    } else {
        v4
    };
    let v8: u64 = f232(v0.clone(), v7)?;
    let v9: bool = {
        let a3622 = v8;
        let a3623 = 101u64;
        if nat_eq(a3622, a3623) {
            true
        } else {
            let a3624 = v8;
            let a3625 = 69u64;
            nat_eq(a3624, a3625)
        }
    };
    let v10: u64 = f232(v0.clone(), {
        let a3626 = v7;
        let a3627 = 1u64;
        nat_add(a3626, a3627)?
    })?;
    let v11: bool = if v9 {
        let a3628 = v10;
        let a3629 = 43u64;
        if nat_eq(a3628, a3629) {
            true
        } else {
            let a3630 = v10;
            let a3631 = 45u64;
            nat_eq(a3630, a3631)
        }
    } else {
        false
    };
    let v12: u64 = if v9 {
        if v11 {
            let a3632 = v7;
            let a3633 = 2u64;
            nat_add(a3632, a3633)?
        } else {
            let a3634 = v7;
            let a3635 = 1u64;
            nat_add(a3634, a3635)?
        }
    } else {
        v7
    };
    let v13: u64 = if v9 {
        f152(v0.clone(), v12)?
    } else {
        v7
    };
    let v14: Bytes = f326(v0.clone(), v3, v4);
    let v15: u64 = f154(v14.clone())?;
    let a3636 = v13;
    let a3637 = v1;
    if if if nat_eq(a3636, a3637) {
        let a3638 = v3;
        let a3639 = v4;
        if nat_lt(a3638, a3639) {
            true
        } else {
            if v5 {
                let a3640 = v6;
                let a3641 = v7;
                nat_lt(a3640, a3641)
            } else {
                false
            }
        }
    } else {
        false
    } {
        if f234(v9) {
            true
        } else {
            let a3642 = v12;
            let a3643 = v13;
            nat_lt(a3642, a3643)
        }
    } else {
        false
    } {
        Ok(Some(f7(List::cons({
            let a3644 = v2;
            let a3645 = 45u64;
            if nat_eq(a3644, a3645) {
                Bytes::lit(&[0x2d])
            } else {
                Bytes::lit(&[])
            }
        }, List::cons({
            let a3647 = v15;
            let a3648 = {
                let a3646 = v14.clone();
                length_bytes(a3646)
            };
            if nat_eq(a3647, a3648) {
                Bytes::lit(&[0x30])
            } else {
                f326(v14.clone(), v15, {
                    let a3649 = v14;
                    length_bytes(a3649)
                })
            }
        }, List::cons({
            let a3650 = v6;
            let a3651 = v7;
            if nat_lt(a3650, a3651) {
                let a3652 = Bytes::lit(&[0x2e]);
                let a3653 = f326(v0.clone(), v6, v7);
                append_bytes(a3652, a3653)
            } else {
                Bytes::lit(&[])
            }
        }, List::cons(if v9 {
            let a3656 = if if v11 {
                let a3654 = v10;
                let a3655 = 45u64;
                nat_eq(a3654, a3655)
            } else {
                false
            } {
                Bytes::lit(&[0x65, 0x2d])
            } else {
                Bytes::lit(&[0x65])
            };
            let a3657 = f326(v0, v12, v13);
            append_bytes(a3656, a3657)
        } else {
            Bytes::lit(&[])
        }, List::<Bytes>::nil())))))))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f531(v0: u64) -> R<Bytes> {
    f125(Str::lit("a number outside the range Studio.Value holds is not read (YAML 1.2.2 section 10.3.2)"), v0)
}

pub fn f532(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a3659 = v2;
    let a3660 = {
        let a3658 = v0.clone();
        length_bytes(a3658)
    };
    if if nat_lt(a3659, a3660) {
        f234({
            let a3661 = f232(v0, v2)?;
            let a3662 = v1;
            nat_eq(a3661, a3662)
        })
    } else {
        false
    } {
        Ok(Some({
            let a3663 = v2;
            let a3664 = 1u64;
            nat_add(a3663, a3664)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f533(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a3709 = {
            let a3685 = {
                let a3673 = {
                    let a3667 = {
                        let a3665 = v3;
                        let a3666 = 128u8;
                        fixed_u8::bit_and(a3665, a3666)
                    };
                    let a3668 = 128u8;
                    if equal(a3667, a3668) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3674 = {
                    let a3671 = {
                        let a3669 = v3;
                        let a3670 = 64u8;
                        fixed_u8::bit_and(a3669, a3670)
                    };
                    let a3672 = 64u8;
                    if equal(a3671, a3672) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3673, a3674)?
            };
            let a3686 = {
                let a3683 = {
                    let a3677 = {
                        let a3675 = v3;
                        let a3676 = 32u8;
                        fixed_u8::bit_and(a3675, a3676)
                    };
                    let a3678 = 32u8;
                    if equal(a3677, a3678) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3684 = {
                    let a3681 = {
                        let a3679 = v3;
                        let a3680 = 16u8;
                        fixed_u8::bit_and(a3679, a3680)
                    };
                    let a3682 = 16u8;
                    if equal(a3681, a3682) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3683, a3684)?
            };
            nat_add(a3685, a3686)?
        };
        let a3710 = {
            let a3707 = {
                let a3695 = {
                    let a3689 = {
                        let a3687 = v3;
                        let a3688 = 8u8;
                        fixed_u8::bit_and(a3687, a3688)
                    };
                    let a3690 = 8u8;
                    if equal(a3689, a3690) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3696 = {
                    let a3693 = {
                        let a3691 = v3;
                        let a3692 = 4u8;
                        fixed_u8::bit_and(a3691, a3692)
                    };
                    let a3694 = 4u8;
                    if equal(a3693, a3694) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3695, a3696)?
            };
            let a3708 = {
                let a3705 = {
                    let a3699 = {
                        let a3697 = v3;
                        let a3698 = 2u8;
                        fixed_u8::bit_and(a3697, a3698)
                    };
                    let a3700 = 2u8;
                    if equal(a3699, a3700) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3706 = {
                    let a3703 = {
                        let a3701 = v3;
                        let a3702 = 1u8;
                        fixed_u8::bit_and(a3701, a3702)
                    };
                    let a3704 = 1u8;
                    if equal(a3703, a3704) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3705, a3706)?
            };
            nat_add(a3707, a3708)?
        };
        nat_add(a3709, a3710)?
    };
    let a3711 = v4;
    let a3712 = 34u64;
    if nat_eq(a3711, a3712) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a3713 = v4;
        let a3714 = 92u64;
        if nat_eq(a3713, a3714) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a3715 = v4;
            let a3716 = 8u64;
            if nat_eq(a3715, a3716) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a3717 = v4;
                let a3718 = 12u64;
                if nat_eq(a3717, a3718) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a3719 = v4;
                    let a3720 = 10u64;
                    if nat_eq(a3719, a3720) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a3721 = v4;
                        let a3722 = 13u64;
                        if nat_eq(a3721, a3722) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a3723 = v4;
                            let a3724 = 9u64;
                            if nat_eq(a3723, a3724) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a3725 = v4;
                                let a3726 = 32u64;
                                if nat_lt(a3725, a3726) {
                                    let a3727 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a3728 = f605(v2)?;
                                    Ok(append_bytes(a3727, a3728))
                                } else {
                                    Ok(f45(v0, v1, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f534(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m3729 = v0.clone();
    match m3729.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a3732 = {
                let (h3730, _) = v2;
                h3730
            };
            let a3733 = {
                let (h3731, _) = v1.clone();
                h3731
            };
            if equal(a3732, a3733) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f535(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f606(f607(Fn13::F608, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m3738 = {
        let (h3734, _) = f609(Fn14::F610, {
            let a3736 = {
                let a3735 = v0;
                length_list(a3735)
            };
            let a3737 = 1u64;
            nat_add(a3736, a3737)?
        }, v1)?;
        h3734
    };
    match m3738.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f536(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a3743 = {
        let a3740 = f331({
            let (h3739, _) = v0;
            h3739
        })?;
        let a3741 = Bytes::lit(&[0x3a]);
        append_bytes(a3740, a3741)
    };
    let a3744 = {
        let (_, h3742) = v0;
        h3742
    };
    Ok(append_bytes(a3743, a3744))
}

pub fn f537(v0: Fn10, p3749: u64, p3750: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m3748 = (p3749, p3750);
    {
        let mut m3748 = m3748;
        loop {
            let step = {
                let (v1, v2) = m3748;
                let m3745 = v1;
                if m3745 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3745 - 1;
                    let m3747 = {
                        let c3746: Fn10 = v0.clone();
                        c3746.apply(v2.clone())?
                    };
                    match m3747 {
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
                Ok(next) => { m3748 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f538(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f611(v0, v1)
}

pub fn f539(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3795 = {
            let a3771 = {
                let a3759 = {
                    let a3753 = {
                        let a3751 = v1;
                        let a3752 = 128u8;
                        fixed_u8::bit_and(a3751, a3752)
                    };
                    let a3754 = 128u8;
                    if equal(a3753, a3754) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3760 = {
                    let a3757 = {
                        let a3755 = v1;
                        let a3756 = 64u8;
                        fixed_u8::bit_and(a3755, a3756)
                    };
                    let a3758 = 64u8;
                    if equal(a3757, a3758) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3759, a3760)?
            };
            let a3772 = {
                let a3769 = {
                    let a3763 = {
                        let a3761 = v1;
                        let a3762 = 32u8;
                        fixed_u8::bit_and(a3761, a3762)
                    };
                    let a3764 = 32u8;
                    if equal(a3763, a3764) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3770 = {
                    let a3767 = {
                        let a3765 = v1;
                        let a3766 = 16u8;
                        fixed_u8::bit_and(a3765, a3766)
                    };
                    let a3768 = 16u8;
                    if equal(a3767, a3768) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3769, a3770)?
            };
            nat_add(a3771, a3772)?
        };
        let a3796 = {
            let a3793 = {
                let a3781 = {
                    let a3775 = {
                        let a3773 = v1;
                        let a3774 = 8u8;
                        fixed_u8::bit_and(a3773, a3774)
                    };
                    let a3776 = 8u8;
                    if equal(a3775, a3776) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3782 = {
                    let a3779 = {
                        let a3777 = v1;
                        let a3778 = 4u8;
                        fixed_u8::bit_and(a3777, a3778)
                    };
                    let a3780 = 4u8;
                    if equal(a3779, a3780) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3781, a3782)?
            };
            let a3794 = {
                let a3791 = {
                    let a3785 = {
                        let a3783 = v1;
                        let a3784 = 2u8;
                        fixed_u8::bit_and(a3783, a3784)
                    };
                    let a3786 = 2u8;
                    if equal(a3785, a3786) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3792 = {
                    let a3789 = {
                        let a3787 = v1;
                        let a3788 = 1u8;
                        fixed_u8::bit_and(a3787, a3788)
                    };
                    let a3790 = 1u8;
                    if equal(a3789, a3790) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3791, a3792)?
            };
            nat_add(a3793, a3794)?
        };
        nat_add(a3795, a3796)?
    };
    let a3797 = 48u64;
    let a3798 = v2;
    if nat_le(a3797, a3798) {
        let a3799 = v2;
        let a3800 = 57u64;
        Ok(nat_le(a3799, a3800))
    } else {
        Ok(false)
    }
}

pub fn f540(v0: u64) -> u32 {
    let a3801 = v0;
    let a3802 = 32u64;
    if nat_lt(a3801, a3802) {
        let a3803 = v0;
        let a3804 = 16u64;
        if nat_lt(a3803, a3804) {
            let a3805 = v0;
            let a3806 = 8u64;
            if nat_lt(a3805, a3806) {
                let a3807 = v0;
                let a3808 = 4u64;
                if nat_lt(a3807, a3808) {
                    let a3809 = v0;
                    let a3810 = 2u64;
                    if nat_lt(a3809, a3810) {
                        let a3811 = v0;
                        let a3812 = 1u64;
                        if nat_lt(a3811, a3812) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a3813 = v0;
                        let a3814 = 3u64;
                        if nat_lt(a3813, a3814) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a3815 = v0;
                    let a3816 = 6u64;
                    if nat_lt(a3815, a3816) {
                        let a3817 = v0;
                        let a3818 = 5u64;
                        if nat_lt(a3817, a3818) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a3819 = v0;
                        let a3820 = 7u64;
                        if nat_lt(a3819, a3820) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a3821 = v0;
                let a3822 = 12u64;
                if nat_lt(a3821, a3822) {
                    let a3823 = v0;
                    let a3824 = 10u64;
                    if nat_lt(a3823, a3824) {
                        let a3825 = v0;
                        let a3826 = 9u64;
                        if nat_lt(a3825, a3826) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a3827 = v0;
                        let a3828 = 11u64;
                        if nat_lt(a3827, a3828) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a3829 = v0;
                    let a3830 = 14u64;
                    if nat_lt(a3829, a3830) {
                        let a3831 = v0;
                        let a3832 = 13u64;
                        if nat_lt(a3831, a3832) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a3833 = v0;
                        let a3834 = 15u64;
                        if nat_lt(a3833, a3834) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a3835 = v0;
            let a3836 = 24u64;
            if nat_lt(a3835, a3836) {
                let a3837 = v0;
                let a3838 = 20u64;
                if nat_lt(a3837, a3838) {
                    let a3839 = v0;
                    let a3840 = 18u64;
                    if nat_lt(a3839, a3840) {
                        let a3841 = v0;
                        let a3842 = 17u64;
                        if nat_lt(a3841, a3842) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a3843 = v0;
                        let a3844 = 19u64;
                        if nat_lt(a3843, a3844) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a3845 = v0;
                    let a3846 = 22u64;
                    if nat_lt(a3845, a3846) {
                        let a3847 = v0;
                        let a3848 = 21u64;
                        if nat_lt(a3847, a3848) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a3849 = v0;
                        let a3850 = 23u64;
                        if nat_lt(a3849, a3850) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a3851 = v0;
                let a3852 = 28u64;
                if nat_lt(a3851, a3852) {
                    let a3853 = v0;
                    let a3854 = 26u64;
                    if nat_lt(a3853, a3854) {
                        let a3855 = v0;
                        let a3856 = 25u64;
                        if nat_lt(a3855, a3856) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a3857 = v0;
                        let a3858 = 27u64;
                        if nat_lt(a3857, a3858) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a3859 = v0;
                    let a3860 = 30u64;
                    if nat_lt(a3859, a3860) {
                        let a3861 = v0;
                        let a3862 = 29u64;
                        if nat_lt(a3861, a3862) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a3863 = v0;
                        let a3864 = 31u64;
                        if nat_lt(a3863, a3864) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a3865 = v0;
        let a3866 = 48u64;
        if nat_lt(a3865, a3866) {
            let a3867 = v0;
            let a3868 = 40u64;
            if nat_lt(a3867, a3868) {
                let a3869 = v0;
                let a3870 = 36u64;
                if nat_lt(a3869, a3870) {
                    let a3871 = v0;
                    let a3872 = 34u64;
                    if nat_lt(a3871, a3872) {
                        let a3873 = v0;
                        let a3874 = 33u64;
                        if nat_lt(a3873, a3874) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a3875 = v0;
                        let a3876 = 35u64;
                        if nat_lt(a3875, a3876) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a3877 = v0;
                    let a3878 = 38u64;
                    if nat_lt(a3877, a3878) {
                        let a3879 = v0;
                        let a3880 = 37u64;
                        if nat_lt(a3879, a3880) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a3881 = v0;
                        let a3882 = 39u64;
                        if nat_lt(a3881, a3882) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a3883 = v0;
                let a3884 = 44u64;
                if nat_lt(a3883, a3884) {
                    let a3885 = v0;
                    let a3886 = 42u64;
                    if nat_lt(a3885, a3886) {
                        let a3887 = v0;
                        let a3888 = 41u64;
                        if nat_lt(a3887, a3888) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a3889 = v0;
                        let a3890 = 43u64;
                        if nat_lt(a3889, a3890) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a3891 = v0;
                    let a3892 = 46u64;
                    if nat_lt(a3891, a3892) {
                        let a3893 = v0;
                        let a3894 = 45u64;
                        if nat_lt(a3893, a3894) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a3895 = v0;
                        let a3896 = 47u64;
                        if nat_lt(a3895, a3896) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a3897 = v0;
            let a3898 = 56u64;
            if nat_lt(a3897, a3898) {
                let a3899 = v0;
                let a3900 = 52u64;
                if nat_lt(a3899, a3900) {
                    let a3901 = v0;
                    let a3902 = 50u64;
                    if nat_lt(a3901, a3902) {
                        let a3903 = v0;
                        let a3904 = 49u64;
                        if nat_lt(a3903, a3904) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a3905 = v0;
                        let a3906 = 51u64;
                        if nat_lt(a3905, a3906) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a3907 = v0;
                    let a3908 = 54u64;
                    if nat_lt(a3907, a3908) {
                        let a3909 = v0;
                        let a3910 = 53u64;
                        if nat_lt(a3909, a3910) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a3911 = v0;
                        let a3912 = 55u64;
                        if nat_lt(a3911, a3912) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a3913 = v0;
                let a3914 = 60u64;
                if nat_lt(a3913, a3914) {
                    let a3915 = v0;
                    let a3916 = 58u64;
                    if nat_lt(a3915, a3916) {
                        let a3917 = v0;
                        let a3918 = 57u64;
                        if nat_lt(a3917, a3918) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a3919 = v0;
                        let a3920 = 59u64;
                        if nat_lt(a3919, a3920) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a3921 = v0;
                    let a3922 = 62u64;
                    if nat_lt(a3921, a3922) {
                        let a3923 = v0;
                        let a3924 = 61u64;
                        if nat_lt(a3923, a3924) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a3925 = v0;
                        let a3926 = 63u64;
                        if nat_lt(a3925, a3926) {
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

pub fn f541(v0: u32) -> u32 {
    let a3929 = {
        let a3927 = f612(v0, 6u32, 26u32);
        let a3928 = f612(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a3927, a3928)
    };
    let a3930 = f612(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a3929, a3930)
}

pub fn f542(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3936 = {
        let a3931 = v0;
        let a3932 = v1;
        fixed_u32::bit_and(a3931, a3932)
    };
    let a3937 = {
        let a3934 = {
            let a3933 = v0;
            fixed_u32::bit_not(a3933)
        };
        let a3935 = v2;
        fixed_u32::bit_and(a3934, a3935)
    };
    fixed_u32::bit_xor(a3936, a3937)
}

pub fn f543(v0: u32) -> u32 {
    let a3940 = {
        let a3938 = f612(v0, 2u32, 30u32);
        let a3939 = f612(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a3938, a3939)
    };
    let a3941 = f612(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a3940, a3941)
}

pub fn f544(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3950 = {
        let a3946 = {
            let a3942 = v0;
            let a3943 = v1;
            fixed_u32::bit_and(a3942, a3943)
        };
        let a3947 = {
            let a3944 = v0;
            let a3945 = v2;
            fixed_u32::bit_and(a3944, a3945)
        };
        fixed_u32::bit_xor(a3946, a3947)
    };
    let a3951 = {
        let a3948 = v1;
        let a3949 = v2;
        fixed_u32::bit_and(a3948, a3949)
    };
    fixed_u32::bit_xor(a3950, a3951)
}

pub fn f545(v0: Adt29) -> Adt29 {
    Adt29::C0(match v0.clone() {
        Adt29::C0(_, h3952, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3952
        }
    }, match v0.clone() {
        Adt29::C0(_, _, h3953, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3953
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, h3954, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3954
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, h3955, _, _, _, _, _, _, _, _, _, _, _) => {
            h3955
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, h3956, _, _, _, _, _, _, _, _, _, _) => {
            h3956
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, h3957, _, _, _, _, _, _, _, _, _) => {
            h3957
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, h3958, _, _, _, _, _, _, _, _) => {
            h3958
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, h3959, _, _, _, _, _, _, _) => {
            h3959
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, _, h3960, _, _, _, _, _, _) => {
            h3960
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, _, _, h3961, _, _, _, _, _) => {
            h3961
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, _, _, _, h3962, _, _, _, _) => {
            h3962
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3963, _, _, _) => {
            h3963
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3964, _, _) => {
            h3964
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3965, _) => {
            h3965
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3966) => {
            h3966
        }
    }, f345(f613(match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3967, _) => {
            h3967
        }
    }), f345(match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, _, h3968, _, _, _, _, _, _) => {
            h3968
        }
    }, f345(f614(match v0.clone() {
        Adt29::C0(_, h3969, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3969
        }
    }), match v0 {
        Adt29::C0(h3970, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3970
        }
    }))))
}

pub fn f546(v0: Fn11, p3974: u64, p3975: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m3973 = (p3974, p3975);
    {
        let mut m3973 = m3973;
        loop {
            let step = {
                let (v1, v2) = m3973;
                let m3971 = v1;
                if m3971 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m3971 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c3972: Fn11 = v0.clone();
                        c3972.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m3973 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f547(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h3976, _) = v0;
        h3976
    };
    let v2: u32 = {
        let (h3977, _) = {
            let (_, h3978) = v0;
            h3978
        };
        h3977
    };
    let v3: u32 = {
        let (_, h3979) = {
            let (_, h3980) = v0;
            h3980
        };
        h3979
    };
    ({
        let a3981 = v1;
        let a3982 = 2u64;
        let a3983 = 0u64;
        nat_quot(a3981, a3982, a3983)
    }, (f548(v2, 1u32), {
        let a3987 = {
            let a3984 = v1;
            let a3985 = 2u64;
            let a3986 = 0u64;
            nat_rem(a3984, a3985, a3986)
        };
        let a3988 = 1u64;
        if nat_eq(a3987, a3988) {
            let a3989 = v3;
            let a3990 = v2;
            fixed_u32::bit_or(a3989, a3990)
        } else {
            v3
        }
    }))
}

pub fn f548(v0: u32, v1: u32) -> u32 {
    let m3993 = {
        let a3991 = v0;
        let a3992 = v1;
        fixed_u32::shift_left(a3991, a3992)
    };
    match m3993 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f549(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    if f615(v1, f129(v0, v2)?) {
        Ok(Some({
            let a3994 = v2;
            let a3995 = 1u64;
            nat_add(a3994, a3995)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f550(p3998: List<Adt15>, p3999: List<Adt15>) -> List<Adt15> {
    let m3997 = (p3998, p3999);
    {
        let mut m3997 = m3997;
        loop {
            let step = {
                let (v0, v1) = m3997;
                let m3996 = v0;
                match m3996.uncons() {
                    None => {
                        Err::<(List<Adt15>, List<Adt15>), List<Adt15>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt15>, List<Adt15>), List<Adt15>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3997 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f551(v0: Bytes, p4004: List<(Bytes, Adt16)>) -> Option<Adt16> {
    let m4003 = p4004;
    {
        let mut m4003 = m4003;
        loop {
            let step = {
                let v1 = m4003;
                let m4000 = v1;
                match m4000.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt16)>, Option<Adt16>>(None::<Adt16>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h4001, _) = v2;
                            h4001
                        }, v0.clone()) {
                            Err::<List<(Bytes, Adt16)>, Option<Adt16>>(Some({
                                let (_, h4002) = v2;
                                h4002
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt16)>, Option<Adt16>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4003 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f552() -> List<(Bytes, Adt16)> {
    List::cons((f8(Str::lit("boolean")), f364(f8(Str::lit("Bool")))), List::cons((f8(Str::lit("byte")), f364(f8(Str::lit("Int")))), List::cons((f8(Str::lit("octet")), f364(f8(Str::lit("Nat")))), List::cons((f8(Str::lit("bigint")), f364(f8(Str::lit("Int")))), List::cons((f8(Str::lit("float")), f364(f490())), List::cons((f8(Str::lit("double")), f364(f490())), List::cons((f8(Str::lit("DOMString")), f364(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("USVString")), f364(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ByteString")), f364(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ArrayBuffer")), f364(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("SharedArrayBuffer")), f364(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("DataView")), f364(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("Int8Array")), f364(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int16Array")), f364(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int32Array")), f364(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("BigInt64Array")), f364(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Uint8Array")), f364(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint16Array")), f364(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint32Array")), f364(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint8ClampedArray")), f364(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("BigUint64Array")), f364(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Float16Array")), f466(f8(Str::lit("Float16Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("Float32Array")), f466(f8(Str::lit("Float32Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("Float64Array")), f466(f8(Str::lit("Float64Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("any")), f466(f8(Str::lit("any is not translated, because it is the union of every Web IDL type and has no one LexLean type (Web IDL 2.13.1)")))), List::cons((f8(Str::lit("object")), f466(f8(Str::lit("object is not translated, because its values are references to JavaScript objects of any shape, which no LexLean type holds (Web IDL 2.13.20)")))), List::cons((f8(Str::lit("symbol")), f466(f8(Str::lit("symbol is not translated, because a symbol value has identity but no content that a LexLean type holds (Web IDL 2.13.21)")))), List::cons((f8(Str::lit("undefined")), f466(f8(Str::lit("undefined is not translated, because it is the type of the absence of a value (Web IDL 2.13.2)")))), List::<(Bytes, Adt16)>::nil()))))))))))))))))))))))))))))
}

pub fn f553() -> Bytes {
    f8(Str::lit("a union type is translated only as the whole body of a typedef, whose identifier names its inductive (Web IDL 2.11 and 2.13.32)"))
}

pub fn f554(v0: u64) -> Bytes {
    f45({
        let a4005 = Str::lit("0123456789ABCDEF");
        utf8_encode(a4005)
    }, v0, 1u64)
}

pub fn f555(p4016: u64, v1: Bytes, v2: Bytes, p4017: u64) -> R<bool> {
    let m4015 = (p4016, p4017);
    {
        let mut m4015 = m4015;
        loop {
            let step = {
                let (v0, v3) = m4015;
                let m4006 = v0;
                if m4006 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(false))
                } else {
                    let v4 = m4006 - 1;
                    let v5: u64 = f616(v1.clone(), v3)?;
                    let v6: u64 = f616(v2.clone(), v3)?;
                    let a4007 = v5;
                    let a4008 = v6;
                    if nat_lt(a4007, a4008) {
                        Err::<(u64, u64), R<bool>>(Ok(true))
                    } else {
                        let a4009 = v6;
                        let a4010 = v5;
                        if nat_lt(a4009, a4010) {
                            Err::<(u64, u64), R<bool>>(Ok(false))
                        } else {
                            let a4011 = v5;
                            let a4012 = 0u64;
                            if nat_eq(a4011, a4012) {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            } else {
                                Ok::<(u64, u64), R<bool>>((v4, {
                                    let a4013 = v3;
                                    let a4014 = 1u64;
                                    nat_add(a4013, a4014)?
                                }))
                            }
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

pub fn f556(p4030: u64, v1: Bytes, p4031: u64, p4032: Bytes) -> R<Option<Bytes>> {
    let m4029 = (p4030, (p4031, p4032));
    {
        let mut m4029 = m4029;
        loop {
            let step = {
                let (v0, (v2, v3)) = m4029;
                let m4018 = v0;
                if m4018 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m4018 - 1;
                    let a4020 = {
                        let a4019 = v1.clone();
                        length_bytes(a4019)
                    };
                    let a4021 = v2;
                    if nat_le(a4020, a4021) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m4022 = f382(v1.clone(), v2)?;
                        match m4022 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a4024 = v2;
                                    let a4025 = {
                                        let (_, h4023) = v5;
                                        h4023
                                    };
                                    nat_add(a4024, a4025)?
                                }, {
                                    let a4027 = v3;
                                    let a4028 = f617(v1.clone(), v2, {
                                        let (h4026, _) = v5;
                                        h4026
                                    });
                                    append_bytes(a4027, a4028)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4029 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f557(p4042: u64, v1: Bytes, p4043: u64) -> R<bool> {
    let m4041 = (p4042, p4043);
    {
        let mut m4041 = m4041;
        loop {
            let step = {
                let (v0, v2) = m4041;
                let m4033 = v0;
                if m4033 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m4033 - 1;
                    let m4036 = {
                        let a4034 = v1.clone();
                        let a4035 = v2;
                        index_bytes(a4034, a4035)
                    };
                    match m4036 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a4037 = f56(v4)?;
                            let a4038 = 128u64;
                            if nat_lt(a4037, a4038) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a4039 = v2;
                                    let a4040 = 1u64;
                                    nat_add(a4039, a4040)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4041 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f558(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f290(v0.clone(), 0u64, {
        let a4044 = v0;
        length_bytes(a4044)
    }, false, Bytes::lit(&[]))?;
    let m4047 = {
        let a4045 = v1.clone();
        let a4046 = 0u64;
        index_bytes(a4045, a4046)
    };
    match m4047 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f56(v2)?;
            if f397(v3) {
                let a4054 = f45({
                    let a4048 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a4048)
                }, {
                    let a4049 = v3;
                    let a4050 = 65u64;
                    nat_sub(a4049, a4050)
                }, 1u64);
                let a4055 = f45(v1.clone(), 1u64, {
                    let a4052 = {
                        let a4051 = v1;
                        length_bytes(a4051)
                    };
                    let a4053 = 1u64;
                    nat_sub(a4052, a4053)
                });
                Ok(append_bytes(a4054, a4055))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f559(v0: Bytes) -> R<bool> {
    let m4058 = {
        let a4056 = v0;
        let a4057 = 0u64;
        index_bytes(a4056, a4057)
    };
    match m4058 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f153(f396(f56(v1)?), f397(f56(v1)?)))
        }
    }
}

pub fn f560(p4071: u64, v1: Bytes, p4072: u64, p4073: Bytes) -> R<Option<Bytes>> {
    let m4070 = (p4071, (p4072, p4073));
    {
        let mut m4070 = m4070;
        loop {
            let step = {
                let (v0, (v2, v3)) = m4070;
                let m4059 = v0;
                if m4059 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m4059 - 1;
                    let a4061 = {
                        let a4060 = v1.clone();
                        length_bytes(a4060)
                    };
                    let a4062 = v2;
                    if nat_le(a4061, a4062) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m4063 = f382(v1.clone(), v2)?;
                        match m4063 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a4065 = v2;
                                    let a4066 = {
                                        let (_, h4064) = v5;
                                        h4064
                                    };
                                    nat_add(a4065, a4066)?
                                }, {
                                    let a4068 = v3;
                                    let a4069 = f618({
                                        let (h4067, _) = v5;
                                        h4067
                                    });
                                    append_bytes(a4068, a4069)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4070 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f561() -> List<Bytes> {
    List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("termination_by")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("admit")), List::cons(f8(Str::lit("below")), List::cons(f8(Str::lit("brecOn")), List::cons(f8(Str::lit("casesOn")), List::cons(f8(Str::lit("ctorElim")), List::cons(f8(Str::lit("ctorElimType")), List::cons(f8(Str::lit("ctorIdx")), List::cons(f8(Str::lit("noConfusion")), List::cons(f8(Str::lit("noConfusionType")), List::cons(f8(Str::lit("rec")), List::cons(f8(Str::lit("recOn")), List::cons(f8(Str::lit("toCtorIdx")), List::<Bytes>::nil())))))))))))))))))))
}

pub fn f562() -> Bytes {
    f8(Str::lit("\u{0}\u{0}:\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{5}\u{7f}\u{0}\u{6}p\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f563() -> Bytes {
    f8(Str::lit("\u{0}\u{0}-\u{0}\u{0}.\u{0}\u{0}0\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}7\u{0}\u{1}7\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@?\u{0}@@\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f564(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f129(v0, {
        let (h4074, _) = v2;
        h4074
    })?;
    let a4076 = 1114111u64;
    let a4077 = {
        let (_, h4075) = v2;
        h4075
    };
    if nat_lt(a4076, a4077) {
        Ok(None::<(u64, u64)>)
    } else {
        let v4: u64 = f619(v3, v1);
        let a4078 = v4;
        let a4079 = 16u64;
        if nat_eq(a4078, a4079) {
            Ok(None::<(u64, u64)>)
        } else {
            Ok(Some(({
                let a4081 = {
                    let (h4080, _) = v2;
                    h4080
                };
                let a4082 = 1u64;
                nat_add(a4081, a4082)?
            }, {
                let a4086 = {
                    let a4084 = {
                        let (_, h4083) = v2;
                        h4083
                    };
                    let a4085 = if v1 {
                        16u64
                    } else {
                        10u64
                    };
                    nat_mul(a4084, a4085)?
                };
                let a4087 = v4;
                nat_add(a4086, a4087)?
            })))
        }
    }
}

pub fn f565() -> Bytes {
    f8(Str::lit("\u{80}\u{81}\u{82}\u{83}\u{84}\u{85}\u{86}\u{87}\u{88}\u{89}\u{8a}\u{8b}\u{8c}\u{8d}\u{8e}\u{8f}\u{90}\u{91}\u{92}\u{93}\u{94}\u{95}\u{96}\u{97}\u{98}\u{99}\u{9a}\u{9b}\u{9c}\u{9d}\u{9e}\u{9f}\u{a0}\u{a1}\u{a2}\u{a3}\u{a4}\u{a5}\u{a6}\u{a7}\u{a8}\u{a9}\u{aa}\u{ab}\u{ac}\u{ad}\u{ae}\u{af}\u{b0}\u{b1}\u{b2}\u{b3}\u{b4}\u{b5}\u{b6}\u{b7}\u{b8}\u{b9}\u{ba}\u{bb}\u{bc}\u{bd}\u{be}\u{bf}"))
}

pub fn f566(v0: u64, v1: bool, v2: u64) -> R<bool> {
    if v1 {
        let a4088 = v0;
        let a4089 = v2;
        if nat_eq(a4088, a4089) {
            Ok(true)
        } else {
            f274(v0, f620())
        }
    } else {
        f274(v0, f621())
    }
}

pub fn f567() -> Bytes {
    f8(Str::lit(" string:anyAtomicType boolean:anyAtomicType decimal:anyAtomicType float:anyAtomicType double:anyAtomicType duration:anyAtomicType dateTime:anyAtomicType time:anyAtomicType date:anyAtomicType gYearMonth:anyAtomicType gYear:anyAtomicType gMonthDay:anyAtomicType gDay:anyAtomicType gMonth:anyAtomicType hexBinary:anyAtomicType base64Binary:anyAtomicType anyURI:anyAtomicType QName:anyAtomicType NOTATION:anyAtomicType normalizedString:string token:normalizedString language:token NMTOKEN:token Name:token NCName:Name ID:NCName IDREF:NCName ENTITY:NCName integer:decimal nonPositiveInteger:integer long:integer nonNegativeInteger:integer negativeInteger:nonPositiveInteger int:long short:int byte:short unsignedLong:nonNegativeInteger positiveInteger:nonNegativeInteger unsignedInt:unsignedLong unsignedShort:unsignedInt unsignedByte:unsignedShort yearMonthDuration:duration dayTimeDuration:duration dateTimeStamp:dateTime NMTOKENS:anySimpleType IDREFS:anySimpleType ENTITIES:anySimpleType anyAtomicType:anySimpleType "))
}

pub fn f568(v0: Adt27, v1: Str, v2: Str) -> bool {
    let m4090 = f308(v0, v1);
    match m4090 {
        None => {
            false
        }
        Some(v3) => {
            f32(v3, f8(v2))
        }
    }
}

pub fn f569(v0: List<Adt26>, v1: (Bytes, Bytes), v2: bool) -> R<Result<List<Bytes>, Bytes>> {
    let m4091 = v0;
    match m4091.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m4092 = f569(v4, v1.clone(), v2)?;
            match m4092 {
                Ok(v6) => {
                    if f32(match v3.clone() {
                        Adt26::C0(h4093, _, _, _) => {
                            h4093
                        }
                    }, f8(Str::lit("complexType"))) {
                        let m4094 = f622(v3.clone())?;
                        match m4094 {
                            Ok(v8) => {
                                let m4095 = v8;
                                match m4095 {
                                    None => {
                                        Ok(Ok::<List<Bytes>, Bytes>(v6))
                                    }
                                    Some(v9) => {
                                        if if f32({
                                            let (h4096, _) = {
                                                let (_, h4097) = v9.clone();
                                                h4097
                                            };
                                            h4096
                                        }, {
                                            let (h4098, _) = v1;
                                            h4098
                                        }) {
                                            if f32({
                                                let (_, h4099) = {
                                                    let (_, h4100) = v9;
                                                    h4100
                                                };
                                                h4099
                                            }, {
                                                let (_, h4101) = v1;
                                                h4101
                                            }) {
                                                if f306(v2) {
                                                    true
                                                } else {
                                                    f307({
                                                        let (h4102, _) = v9;
                                                        h4102
                                                    }, Str::lit("extension"))
                                                }
                                            } else {
                                                false
                                            }
                                        } else {
                                            false
                                        } {
                                            Ok(Ok::<List<Bytes>, Bytes>(List::cons(match v3 {
                                                Adt26::C0(_, _, h4103, _) => {
                                                    h4103
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

pub fn f570(v0: Bytes, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<Adt28, Bytes>> {
    let m4104 = v1.clone();
    match m4104.uncons() {
        None => {
            Ok(Err::<Adt28, Bytes>(f500(f8(Str::lit("a choice with no particle that can occur matches no element sequence, so it has no LexLean constructor (XSD 1.1 Part 1, section 3.8.4.3)")), v0)))
        }
        Some((_, _)) => {
            let m4105 = f623(v1)?;
            match m4105 {
                Ok(v7) => {
                    let v8: List<Bytes> = f624(v7, v2);
                    Ok(Ok::<Adt28, Bytes>(Adt28::C0(v0.clone(), v3, f264(v0, v8.clone()), v8)))
                }
                Err(v6) => {
                    Ok(Err::<Adt28, Bytes>(f500(v6, v0)))
                }
            }
        }
    }
}

pub fn f571(v0: List<Bytes>) -> R<List<Bytes>> {
    let m4106 = v0;
    match m4106.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f211(v1)?, f571(v2)?))
        }
    }
}

pub fn f572(v0: u64, v1: Adt25, v2: Bytes, v3: Bytes, v4: Adt27) -> R<Result<List<Adt28>, Bytes>> {
    let m4107 = v0;
    if m4107 == 0 {
        Ok(Err::<List<Adt28>, Bytes>(f500(f8(Str::lit("the complex types nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m4107 - 1;
        let m4108 = f625(v5, v1, v2.clone(), v3, v4, f503())?;
        match m4108 {
            Ok(v7) => {
                let m4110 = f580(v2, match v7.clone() {
                    Adt36::C0(h4109, _, _, _, _) => {
                        h4109
                    }
                })?;
                match m4110 {
                    Ok(v9) => {
                        Ok(Ok::<List<Adt28>, Bytes>(f403(match v7 {
                            Adt36::C0(_, h4111, _, _, _) => {
                                h4111
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

pub fn f573(v0: Bytes) -> Bytes {
    let a4112 = v0;
    let a4113 = f8(Str::lit("Content"));
    append_bytes(a4112, a4113)
}

pub fn f574(v0: Adt25, v1: Adt27, v2: Bytes) -> R<Result<Adt24, Bytes>> {
    let m4115 = f133(match v1 {
        Adt27::C0(_, _, _, _, h4114) => {
            h4114
        }
    }, v2.clone())?;
    match m4115 {
        Ok(v4) => {
            if f32({
                let (h4116, _) = v4.clone();
                h4116
            }, f208()) {
                if f32({
                    let (_, h4117) = v4.clone();
                    h4117
                }, f8(Str::lit("anyType"))) {
                    Ok(Ok::<Adt24, Bytes>(Adt24::C0(f406(), List::cons(f406(), List::<Bytes>::nil()))))
                } else {
                    let m4119 = f392({
                        let (_, h4118) = v4;
                        h4118
                    }, 16u64)?;
                    match m4119 {
                        Ok(v6) => {
                            Ok(Ok::<Adt24, Bytes>(Adt24::C0(v6, List::<Bytes>::nil())))
                        }
                        Err(v5) => {
                            Ok(Err::<Adt24, Bytes>(v5))
                        }
                    }
                }
            } else {
                if f173({
                    let (h4120, _) = v4.clone();
                    h4120
                }, match v0.clone() {
                    Adt25::C0(_, h4121) => {
                        h4121
                    }
                }) {
                    let v7: Bytes = f211({
                        let (_, h4122) = v4.clone();
                        h4122
                    })?;
                    let m4123 = f581(v0.clone(), Str::lit("complexType"), v4.clone());
                    match m4123 {
                        None => {
                            let m4124 = f581(v0, Str::lit("simpleType"), v4);
                            match m4124 {
                                None => {
                                    Ok(Err::<Adt24, Bytes>(f220(f8(Str::lit("the type ")), v2, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)")))))
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
                    Ok(Err::<Adt24, Bytes>(f220(f8(Str::lit("the type ")), v2, f220(f8(Str::lit(" is in the namespace ")), {
                        let (h4125, _) = v4;
                        h4125
                    }, f8(Str::lit(", which no schema document of this set targets (XSD 1.1 Part 1, sections 3.17.6.2 and 4.2.6)"))))))
                }
            }
        }
        Err(v3) => {
            Ok(Err::<Adt24, Bytes>(v3))
        }
    }
}

pub fn f575(v0: Adt27, v1: Bytes) -> Bytes {
    if f568(v0, Str::lit("nillable"), Str::lit("true")) {
        f626(Str::lit("Option"), v1)
    } else {
        v1
    }
}

pub fn f576(p4130: u64, v1: Adt25, v2: Bytes, v3: Bytes, v4: bool, p4131: List<Adt27>, p4132: Adt36) -> R<Result<Adt36, Bytes>> {
    let m4129 = (p4130, (p4131, p4132));
    {
        let mut m4129 = m4129;
        loop {
            let step = {
                let (v0, (v5, v6)) = m4129;
                let m4126 = v0;
                if m4126 == 0 {
                    Err::<(u64, (List<Adt27>, Adt36)), R<Result<Adt36, Bytes>>>(Ok(Err::<Adt36, Bytes>(f500(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2.clone()))))
                } else {
                    let v7 = m4126 - 1;
                    let m4127 = v5;
                    match m4127.uncons() {
                        None => {
                            Err::<(u64, (List<Adt27>, Adt36)), R<Result<Adt36, Bytes>>>(Ok(Ok::<Adt36, Bytes>(v6)))
                        }
                        Some((v8, v9)) => {
                            let m4128 = f627(v7, v1.clone(), v2.clone(), v3.clone(), v4, v8, v6)?;
                            match m4128 {
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
                Ok(next) => { m4129 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f577(v0: List<Adt37>) -> List<Bytes> {
    let m4133 = v0;
    match m4133.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt37::C0(h4134, _, _) => {
                    h4134
                }
            }, f577(v2))
        }
    }
}

pub fn f578(v0: List<Adt37>) -> List<Bytes> {
    let m4135 = v0;
    match m4135.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt37::C0(_, h4136, _) => {
                    h4136
                }
            }, f578(v2))
        }
    }
}

pub fn f579(v0: List<Adt37>) -> List<Bytes> {
    let m4137 = v0;
    match m4137.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f413(match v1 {
                Adt37::C0(_, _, h4138) => {
                    h4138
                }
            }, f579(v2))
        }
    }
}

pub fn f580(v0: Bytes, v1: List<Adt37>) -> R<Result<Adt28, Bytes>> {
    let m4139 = f623(f577(v1.clone()))?;
    match m4139 {
        Ok(v3) => {
            let v4: List<Bytes> = f578(v1.clone());
            let v7: Bytes = {
                let m4140 = v1.clone();
                match m4140.uncons() {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some((_, _)) => {
                        f220(f8(Str::lit("\u{a}")), f117(f628(v3, v4.clone()), f8(Str::lit(",\u{a}"))), f8(Str::lit("\u{a}  ")))
                    }
                }
            };
            Ok(Ok::<Adt28, Bytes>(Adt28::C0(v0.clone(), f579(v1), f220(f8(Str::lit("structure{\u{a}  name: ")), v0, f220(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), v7, f8(Str::lit("]\u{a}}\u{a}")))), List::cons(f220(f8(Str::lit("    {name: mk, fields: [")), f117(v4, f8(Str::lit(", "))), f8(Str::lit("]}"))), List::<Bytes>::nil()))))
        }
        Err(v2) => {
            Ok(Err::<Adt28, Bytes>(f500(v2, v0)))
        }
    }
}

pub fn f581(v0: Adt25, v1: Str, v2: (Bytes, Bytes)) -> Option<Adt26> {
    f629(match v0 {
        Adt25::C0(h4141, _) => {
            h4141
        }
    }, f8(v1), {
        let (h4142, _) = v2;
        h4142
    }, {
        let (_, h4143) = v2;
        h4143
    })
}

pub fn f582(v0: Adt36, v1: Bytes, v2: Adt35, v3: bool) -> Adt36 {
    f630(f583(v0, Adt37::C0(v1, if v3 {
        let m4146 = match v2.clone() {
            Adt35::C0(h4145, _) => {
                h4145
            }
        };
        match m4146 {
            Adt24::C0(h4144, _) => {
                h4144
            }
        }
    } else {
        f626(Str::lit("Option"), {
            let m4149 = match v2.clone() {
                Adt35::C0(h4148, _) => {
                    h4148
                }
            };
            match m4149 {
                Adt24::C0(h4147, _) => {
                    h4147
                }
            }
        })
    }, {
        let m4152 = match v2.clone() {
            Adt35::C0(h4151, _) => {
                h4151
            }
        };
        match m4152 {
            Adt24::C0(_, h4150) => {
                h4150
            }
        }
    })), match v2 {
        Adt35::C0(_, h4153) => {
            h4153
        }
    })
}

pub fn f583(v0: Adt36, v1: Adt37) -> Adt36 {
    Adt36::C0(f631(match v0.clone() {
        Adt36::C0(h4154, _, _, _, _) => {
            h4154
        }
    }, List::cons(v1, List::<Adt37>::nil())), match v0.clone() {
        Adt36::C0(_, h4155, _, _, _) => {
            h4155
        }
    }, match v0.clone() {
        Adt36::C0(_, _, h4156, _, _) => {
            h4156
        }
    }, match v0.clone() {
        Adt36::C0(_, _, _, h4157, _) => {
            h4157
        }
    }, match v0 {
        Adt36::C0(_, _, _, _, h4158) => {
            h4158
        }
    })
}

pub fn f584() -> Bytes {
    f8(Str::lit("(List ((Bytes * Bytes) * Bytes))"))
}

pub fn f585(v0: Bytes, v1: List<Bytes>, v2: Bytes) -> Bytes {
    f220(f8(Str::lit("inductive{\u{a}  name: ")), v0, f220(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), f117(v1, f8(Str::lit(",\u{a}"))), f220(f8(Str::lit("\u{a}  ],\u{a}  mutual: ")), v2, f8(Str::lit("\u{a}}\u{a}")))))
}

pub fn f586(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a4159 = 0u64;
    let a4160 = v1;
    if if nat_lt(a4159, a4160) {
        f234({
            let a4163 = f232(v0, {
                let a4161 = v1;
                let a4162 = 1u64;
                nat_sub(a4161, a4162)
            })?;
            let a4164 = 10u64;
            nat_eq(a4163, a4164)
        })
    } else {
        false
    } {
        Ok(Some({
            let a4165 = v1;
            let a4166 = 1u64;
            nat_sub(a4165, a4166)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f587(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f315(f232(v0, v1)?) {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a4167 = v1;
            let a4168 = 1u64;
            nat_add(a4167, a4168)?
        }))
    }
}

pub fn f588(v0: Bytes, v1: u64, v2: u64) -> R<Adt38> {
    let v3: (u64, u64) = {
        let (h4169, _) = f240(Fn6::F632(v2, v0.clone()), {
            let a4171 = {
                let a4170 = v0.clone();
                length_bytes(a4170)
            };
            let a4172 = 1u64;
            nat_add(a4171, a4172)?
        }, (v1, 0u64))?;
        h4169
    };
    let v4: u64 = f589(v0.clone(), {
        let (h4173, _) = v3;
        h4173
    }, v2)?;
    let a4174 = f232(v0, v4)?;
    let a4175 = 9u64;
    if if nat_eq(a4174, a4175) {
        let a4179 = {
            let a4177 = v4;
            let a4178 = {
                let (h4176, _) = v3;
                h4176
            };
            nat_sub(a4177, a4178)
        };
        let a4180 = v2;
        nat_lt(a4179, a4180)
    } else {
        false
    } {
        Ok(Adt38::C1(f125(Str::lit("a tab cannot indent a block scalar line (YAML 1.2.2 [63], [70])"), v4)?))
    } else {
        Ok(Adt38::C0({
            let (_, h4181) = v3;
            h4181
        }, {
            let (h4182, _) = v3;
            h4182
        }))
    }
}

pub fn f589(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h4183, _) = f188(Fn5::F633(v2, v0.clone(), v1), {
        let a4185 = {
            let a4184 = v0;
            length_bytes(a4184)
        };
        let a4186 = 1u64;
        nat_add(a4185, a4186)?
    }, v1)?;
    Ok(h4183)
}

pub fn f590(v0: Bytes, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = f589(v0.clone(), v1, v2)?;
    let a4189 = {
        let a4187 = v3;
        let a4188 = v1;
        nat_sub(a4187, a4188)
    };
    let a4190 = v2;
    if nat_eq(a4189, a4190) {
        Ok(f234({
            let a4191 = f232(v0, v3)?;
            let a4192 = 256u64;
            nat_eq(a4191, a4192)
        }))
    } else {
        Ok(false)
    }
}

pub fn f591(v0: Bytes, p4202: u64, v2: u64, v3: bool, p4203: u64, p4204: Bytes, p4205: u64) -> R<Adt39> {
    let m4201 = (p4202, (p4203, (p4204, p4205)));
    {
        let mut m4201 = m4201;
        loop {
            let step = {
                let (v1, (v4, (v5, v6))) = m4201;
                let m4193 = v4;
                if m4193 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<Adt39>>(Ok(Adt39::C1(f125(Str::lit("a block scalar has more lines than the input"), v1)?)))
                } else {
                    let v7 = m4193 - 1;
                    let v8: u64 = f589(v0.clone(), v1, v2)?;
                    let v9: bool = f234(f314(f232(v0.clone(), v8)?));
                    let v10: u64 = f421(v0.clone(), v8)?;
                    let v11: Bytes = f7(List::cons(v5, List::cons(f517(v6), List::cons(f326(v0.clone(), v8, v10), List::<Bytes>::nil()))));
                    let m4194 = f588(v0.clone(), f318(v0.clone(), v10)?, v2)?;
                    match m4194 {
                        Adt38::C0(v13, v14) => {
                            let v15: u64 = v14;
                            let v16: u64 = f589(v0.clone(), v15, v2)?;
                            if f590(v0.clone(), v15, v2)? {
                                Ok::<(u64, (u64, (Bytes, u64))), R<Adt39>>((v15, (v7, ({
                                    let a4197 = v11;
                                    let a4198 = if if if v3 {
                                        v9
                                    } else {
                                        false
                                    } {
                                        f234(f314(f232(v0.clone(), v16)?))
                                    } else {
                                        false
                                    } {
                                        let a4195 = v13;
                                        let a4196 = 0u64;
                                        if nat_eq(a4195, a4196) {
                                            Bytes::lit(&[0x20])
                                        } else {
                                            Bytes::lit(&[])
                                        }
                                    } else {
                                        Bytes::lit(&[0x0a])
                                    };
                                    append_bytes(a4197, a4198)
                                }, v13))))
                            } else {
                                Err::<(u64, (u64, (Bytes, u64))), R<Adt39>>(Ok(Adt39::C0(v11, v13, {
                                    let a4199 = f232(v0.clone(), v16)?;
                                    let a4200 = 256u64;
                                    if nat_eq(a4199, a4200) {
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
                Ok(next) => { m4201 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f592(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f319(v0.clone(), {
        let (h4206, _) = v1;
        h4206
    })?;
    let a4207 = f232(v0, v2)?;
    let a4208 = 10u64;
    if nat_eq(a4207, a4208) {
        Ok(Some(({
            let a4209 = v2;
            let a4210 = 1u64;
            nat_add(a4209, a4210)?
        }, f519({
            let (_, h4211) = v1;
            h4211
        }, {
            let a4213 = v2;
            let a4214 = {
                let (h4212, _) = v1;
                h4212
            };
            nat_sub(a4213, a4214)
        }))))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f593(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f634(v0, v1)
}

pub fn f594(v0: u64) -> R<Option<u64>> {
    let v1: u64 = f325(f8(Str::lit("abtnvfr")), v0)?;
    let a4215 = v1;
    let a4216 = 7u64;
    if nat_lt(a4215, a4216) {
        Ok(Some({
            let a4217 = v1;
            let a4218 = 7u64;
            nat_add(a4217, a4218)?
        }))
    } else {
        let a4219 = v0;
        let a4220 = 48u64;
        if nat_eq(a4219, a4220) {
            Ok(Some(0u64))
        } else {
            let a4221 = v0;
            let a4222 = 9u64;
            if nat_eq(a4221, a4222) {
                Ok(Some(9u64))
            } else {
                let a4223 = v0;
                let a4224 = 101u64;
                if nat_eq(a4223, a4224) {
                    Ok(Some(27u64))
                } else {
                    let a4225 = v0;
                    let a4226 = 78u64;
                    if nat_eq(a4225, a4226) {
                        Ok(Some(133u64))
                    } else {
                        let a4227 = v0;
                        let a4228 = 95u64;
                        if nat_eq(a4227, a4228) {
                            Ok(Some(160u64))
                        } else {
                            let a4229 = v0;
                            let a4230 = 76u64;
                            if nat_eq(a4229, a4230) {
                                Ok(Some(8232u64))
                            } else {
                                let a4231 = v0;
                                let a4232 = 80u64;
                                if nat_eq(a4231, a4232) {
                                    Ok(Some(8233u64))
                                } else {
                                    let a4233 = f325(f8(Str::lit(" \"/\\")), v0)?;
                                    let a4234 = 4u64;
                                    if nat_lt(a4233, a4234) {
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

pub fn f595(v0: u64) -> R<Bytes> {
    f125(Str::lit("an implicit key must be on one line (YAML 1.2.2 [154])"), v0)
}

pub fn f596(v0: Bytes, p4249: u64, v2: u64, p4250: u64, p4251: u64) -> R<Adt38> {
    let m4248 = (p4249, (p4250, p4251));
    {
        let mut m4248 = m4248;
        loop {
            let step = {
                let (v1, (v3, v4)) = m4248;
                let m4235 = v3;
                if m4235 == 0 {
                    Err::<(u64, (u64, u64)), R<Adt38>>(Ok(Adt38::C1(f125(Str::lit("the input ends inside a scalar"), v1)?)))
                } else {
                    let v5 = m4235 - 1;
                    let v6: u64 = {
                        let a4236 = v1;
                        let a4237 = 1u64;
                        nat_add(a4236, a4237)?
                    };
                    let v7: u64 = f319(v0.clone(), v6)?;
                    let v8: u64 = f317(v0.clone(), v7)?;
                    let v9: u64 = f232(v0.clone(), v8)?;
                    if f230(v0.clone(), v6)? {
                        Err::<(u64, (u64, u64)), R<Adt38>>(Ok(Adt38::C1(f125(Str::lit("a document marker cannot appear inside a scalar (YAML 1.2.2 [206])"), v6)?)))
                    } else {
                        let a4238 = v9;
                        let a4239 = 10u64;
                        if nat_eq(a4238, a4239) {
                            Ok::<(u64, (u64, u64)), R<Adt38>>((v8, (v5, {
                                let a4240 = v4;
                                let a4241 = 1u64;
                                nat_add(a4240, a4241)?
                            })))
                        } else {
                            let a4242 = v9;
                            let a4243 = 256u64;
                            if nat_eq(a4242, a4243) {
                                Err::<(u64, (u64, u64)), R<Adt38>>(Ok(Adt38::C1(f125(Str::lit("the input ends inside a quoted scalar (YAML 1.2.2 [109], [120])"), v8)?)))
                            } else {
                                let a4246 = {
                                    let a4244 = v7;
                                    let a4245 = v6;
                                    nat_sub(a4244, a4245)
                                };
                                let a4247 = v2;
                                if nat_lt(a4246, a4247) {
                                    Err::<(u64, (u64, u64)), R<Adt38>>(Ok(Adt38::C1(f125(Str::lit("a continuation line of a scalar is indented less than its node (YAML 1.2.2 [69])"), v7)?)))
                                } else {
                                    Err::<(u64, (u64, u64)), R<Adt38>>(Ok(Adt38::C0(v4, v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4248 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f597(v0: Bytes, p4261: u64, p4262: u64, p4263: u64) -> R<Option<u64>> {
    let m4260 = (p4261, (p4262, p4263));
    {
        let mut m4260 = m4260;
        loop {
            let step = {
                let (v1, (v2, v3)) = m4260;
                let m4252 = v2;
                if m4252 == 0 {
                    Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(Some(v3)))
                } else {
                    let v4 = m4252 - 1;
                    let m4253 = f635(f232(v0.clone(), v1)?);
                    match m4253 {
                        None => {
                            Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(None::<u64>))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, u64)), R<Option<u64>>>(({
                                let a4254 = v1;
                                let a4255 = 1u64;
                                nat_add(a4254, a4255)?
                            }, (v4, {
                                let a4258 = {
                                    let a4256 = v3;
                                    let a4257 = 16u64;
                                    nat_mul(a4256, a4257)?
                                };
                                let a4259 = v5;
                                nat_add(a4258, a4259)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4260 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f598(v0: Bytes, v1: u64) -> R<u64> {
    let (h4264, _) = f188(Fn5::F636(v0.clone()), {
        let a4266 = {
            let a4265 = v0;
            length_bytes(a4265)
        };
        let a4267 = 1u64;
        nat_add(a4266, a4267)?
    }, v1)?;
    Ok(h4264)
}

pub fn f599(v0: Bytes, v1: u64) -> R<u64> {
    let (h4268, _) = f188(Fn5::F637(v0.clone()), {
        let a4270 = {
            let a4269 = v0;
            length_bytes(a4269)
        };
        let a4271 = 1u64;
        nat_add(a4270, a4271)?
    }, v1)?;
    Ok(h4268)
}

pub fn f600(v0: Fn12, p4276: u64, p4277: (u64, bool)) -> R<((u64, bool), bool)> {
    let m4275 = (p4276, p4277);
    {
        let mut m4275 = m4275;
        loop {
            let step = {
                let (v1, v2) = m4275;
                let m4272 = v1;
                if m4272 == 0 {
                    Err::<(u64, (u64, bool)), R<((u64, bool), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4272 - 1;
                    let m4274 = {
                        let c4273: Fn12 = v0.clone();
                        c4273.apply(v2)?
                    };
                    match m4274 {
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
                Ok(next) => { m4275 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f601(v0: u64, v1: Bytes, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    f638(v1, v0, v2)
}

pub fn f602(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = {
        let (h4278, _) = v2;
        h4278
    };
    let v4: u64 = f232(v0.clone(), v3)?;
    if f315(v4) {
        Ok(None::<(u64, u64)>)
    } else {
        if f314(v4) {
            Ok(Some(({
                let a4279 = v3;
                let a4280 = 1u64;
                nat_add(a4279, a4280)?
            }, {
                let (_, h4281) = v2;
                h4281
            })))
        } else {
            let a4282 = v4;
            let a4283 = 58u64;
            if if nat_eq(a4282, a4283) {
                f234(f423(f232(v0.clone(), {
                    let a4284 = v3;
                    let a4285 = 1u64;
                    nat_add(a4284, a4285)?
                })?, v1))
            } else {
                false
            } {
                Ok(None::<(u64, u64)>)
            } else {
                let a4286 = v4;
                let a4287 = 35u64;
                if if nat_eq(a4286, a4287) {
                    f314(f232(v0, {
                        let a4288 = v3;
                        let a4289 = 1u64;
                        nat_sub(a4288, a4289)
                    })?)
                } else {
                    false
                } {
                    Ok(None::<(u64, u64)>)
                } else {
                    if if v1 {
                        f512(v4)
                    } else {
                        false
                    } {
                        Ok(None::<(u64, u64)>)
                    } else {
                        Ok(Some(({
                            let a4290 = v3;
                            let a4291 = 1u64;
                            nat_add(a4290, a4291)?
                        }, {
                            let a4292 = v3;
                            let a4293 = 1u64;
                            nat_add(a4292, a4293)?
                        })))
                    }
                }
            }
        }
    }
}

pub fn f603(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f639(v0, v1)
}

pub fn f604(v0: Bytes, v1: u64, v2: u64) -> R<(u64, u64)> {
    let (h4294, _) = f240(Fn6::F640(v1, v0.clone()), {
        let a4296 = {
            let a4295 = v0;
            length_bytes(a4295)
        };
        let a4297 = 1u64;
        nat_add(a4296, a4297)?
    }, (v2, 0u64))?;
    Ok(h4294)
}

pub fn f605(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4342 = {
            let a4318 = {
                let a4306 = {
                    let a4300 = {
                        let a4298 = v1;
                        let a4299 = 128u8;
                        fixed_u8::bit_and(a4298, a4299)
                    };
                    let a4301 = 128u8;
                    if equal(a4300, a4301) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4307 = {
                    let a4304 = {
                        let a4302 = v1;
                        let a4303 = 64u8;
                        fixed_u8::bit_and(a4302, a4303)
                    };
                    let a4305 = 64u8;
                    if equal(a4304, a4305) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4306, a4307)?
            };
            let a4319 = {
                let a4316 = {
                    let a4310 = {
                        let a4308 = v1;
                        let a4309 = 32u8;
                        fixed_u8::bit_and(a4308, a4309)
                    };
                    let a4311 = 32u8;
                    if equal(a4310, a4311) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4317 = {
                    let a4314 = {
                        let a4312 = v1;
                        let a4313 = 16u8;
                        fixed_u8::bit_and(a4312, a4313)
                    };
                    let a4315 = 16u8;
                    if equal(a4314, a4315) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4316, a4317)?
            };
            nat_add(a4318, a4319)?
        };
        let a4343 = {
            let a4340 = {
                let a4328 = {
                    let a4322 = {
                        let a4320 = v1;
                        let a4321 = 8u8;
                        fixed_u8::bit_and(a4320, a4321)
                    };
                    let a4323 = 8u8;
                    if equal(a4322, a4323) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4329 = {
                    let a4326 = {
                        let a4324 = v1;
                        let a4325 = 4u8;
                        fixed_u8::bit_and(a4324, a4325)
                    };
                    let a4327 = 4u8;
                    if equal(a4326, a4327) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4328, a4329)?
            };
            let a4341 = {
                let a4338 = {
                    let a4332 = {
                        let a4330 = v1;
                        let a4331 = 2u8;
                        fixed_u8::bit_and(a4330, a4331)
                    };
                    let a4333 = 2u8;
                    if equal(a4332, a4333) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4339 = {
                    let a4336 = {
                        let a4334 = v1;
                        let a4335 = 1u8;
                        fixed_u8::bit_and(a4334, a4335)
                    };
                    let a4337 = 1u8;
                    if equal(a4336, a4337) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4338, a4339)?
            };
            nat_add(a4340, a4341)?
        };
        nat_add(a4342, a4343)?
    };
    let a4350 = f184(f641(), {
        let a4344 = v2;
        let a4345 = 16u64;
        let a4346 = 0u64;
        nat_quot(a4344, a4345, a4346)
    });
    let a4351 = f184(f641(), {
        let a4347 = v2;
        let a4348 = 16u64;
        let a4349 = 0u64;
        nat_rem(a4347, a4348, a4349)
    });
    Ok(append_bytes(a4350, a4351))
}

pub fn f606(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f642(Fn15::F643, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f607(v0: Fn13, p4355: List<List<(Bytes, Bytes)>>, p4356: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m4354 = (p4355, p4356);
    {
        let mut m4354 = m4354;
        loop {
            let step = {
                let (v1, v2) = m4354;
                let m4352 = v2;
                match m4352.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c4353: Fn13 = v0.clone();
                            c4353.apply(v1, v3)
                        }, v4))
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

pub fn f608(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f609(v0: Fn14, p4361: u64, p4362: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m4360 = (p4361, p4362);
    {
        let mut m4360 = m4360;
        loop {
            let step = {
                let (v1, v2) = m4360;
                let m4357 = v1;
                if m4357 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4357 - 1;
                    let m4359 = {
                        let c4358: Fn14 = v0.clone();
                        c4358.apply(v2.clone())?
                    };
                    match m4359 {
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
                Ok(next) => { m4360 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f610(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f644(v0)
}

pub fn f611(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h4363, _) = v1;
        h4363
    };
    let m4365 = {
        let (_, h4364) = v1;
        h4364
    };
    match m4365 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m4368 = {
                let a4366 = v0.clone();
                let a4367 = v2;
                index_bytes(a4366, a4367)
            };
            match m4368 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m4373 = {
                        let a4371 = v0;
                        let a4372 = {
                            let a4369 = v2;
                            let a4370 = 1u64;
                            nat_add(a4369, a4370)?
                        };
                        index_bytes(a4371, a4372)
                    };
                    match m4373 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f645(v4)? {
                                f645(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a4374 = v2;
                                    let a4375 = 2u64;
                                    nat_add(a4374, a4375)?
                                }, Some({
                                    let a4380 = v3;
                                    let a4381 = f184(f646(), {
                                        let a4378 = {
                                            let a4376 = f647(v4)?;
                                            let a4377 = 16u64;
                                            nat_mul(a4376, a4377)?
                                        };
                                        let a4379 = f647(v5)?;
                                        nat_add(a4378, a4379)?
                                    });
                                    append_bytes(a4380, a4381)
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

pub fn f612(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4382 = f245(v0, v1);
    let a4383 = f548(v0, v2);
    fixed_u32::bit_or(a4382, a4383)
}

pub fn f613(v0: u32) -> u32 {
    let a4386 = {
        let a4384 = f612(v0, 17u32, 15u32);
        let a4385 = f612(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a4384, a4385)
    };
    let a4387 = f245(v0, 10u32);
    fixed_u32::bit_xor(a4386, a4387)
}

pub fn f614(v0: u32) -> u32 {
    let a4390 = {
        let a4388 = f612(v0, 7u32, 25u32);
        let a4389 = f612(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a4388, a4389)
    };
    let a4391 = f245(v0, 3u32);
    fixed_u32::bit_xor(a4390, a4391)
}

pub fn f615(v0: u64, v1: u64) -> bool {
    let a4392 = v0;
    let a4393 = 0u64;
    if nat_eq(a4392, a4393) {
        f452(v1)
    } else {
        let a4394 = v0;
        let a4395 = 1u64;
        if nat_eq(a4394, a4395) {
            f451(v1)
        } else {
            let a4396 = v0;
            let a4397 = 2u64;
            if nat_eq(a4396, a4397) {
                f648(v1)
            } else {
                f649(v1)
            }
        }
    }
}

pub fn f616(v0: Bytes, v1: u64) -> R<u64> {
    let a4399 = v1;
    let a4400 = {
        let a4398 = v0.clone();
        length_bytes(a4398)
    };
    if nat_lt(a4399, a4400) {
        let a4401 = f129(v0, v1)?;
        let a4402 = 1u64;
        nat_add(a4401, a4402)
    } else {
        Ok(0u64)
    }
}

pub fn f617(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f153(f96(f153(f396(v2), f397(v2)), f96({
        let a4403 = v2;
        let a4404 = 117u64;
        !nat_eq(a4403, a4404)
    }, {
        let a4405 = v2;
        let a4406 = 85u64;
        !nat_eq(a4405, a4406)
    })), f96(f398(v2), {
        let a4407 = 0u64;
        let a4408 = v1;
        nat_lt(a4407, a4408)
    })) {
        f45(v0, v1, 1u64)
    } else {
        let a4409 = v2;
        let a4410 = 65536u64;
        if nat_lt(a4409, a4410) {
            let a4411 = f8(Str::lit("u"));
            let a4412 = f470(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a4411, a4412)
        } else {
            let a4413 = f8(Str::lit("U"));
            let a4414 = f470(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a4413, a4414)
        }
    }
}

pub fn f618(v0: u64) -> Bytes {
    let a4415 = f8(Str::lit("u"));
    let a4416 = f470(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a4415, a4416)
}

pub fn f619(v0: u64, v1: bool) -> u64 {
    let a4417 = v0;
    let a4418 = 48u64;
    if nat_lt(a4417, a4418) {
        16u64
    } else {
        let a4419 = v0;
        let a4420 = 57u64;
        if nat_le(a4419, a4420) {
            let a4421 = v0;
            let a4422 = 48u64;
            nat_sub(a4421, a4422)
        } else {
            if v1 {
                let a4423 = v0;
                let a4424 = 65u64;
                if nat_lt(a4423, a4424) {
                    16u64
                } else {
                    let a4425 = v0;
                    let a4426 = 70u64;
                    if nat_le(a4425, a4426) {
                        let a4427 = v0;
                        let a4428 = 55u64;
                        nat_sub(a4427, a4428)
                    } else {
                        let a4429 = v0;
                        let a4430 = 97u64;
                        if nat_lt(a4429, a4430) {
                            16u64
                        } else {
                            let a4431 = v0;
                            let a4432 = 102u64;
                            if nat_le(a4431, a4432) {
                                let a4433 = v0;
                                let a4434 = 87u64;
                                nat_sub(a4433, a4434)
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

pub fn f620() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f621() -> Bytes {
    f8(Str::lit("\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{0}]\u{0}\u{0}]\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f622(v0: Adt26) -> R<Result<Option<(Adt27, (Bytes, Bytes))>, Bytes>> {
    let m4436 = f498(match v0 {
        Adt26::C0(_, _, _, h4435) => {
            h4435
        }
    })?;
    match m4436 {
        Ok(v2) => {
            let m4437 = f499(v2, List::cons(f8(Str::lit("simpleContent")), List::cons(f8(Str::lit("complexContent")), List::<Bytes>::nil())));
            match m4437 {
                None => {
                    Ok(Ok::<Option<(Adt27, (Bytes, Bytes))>, Bytes>(None::<(Adt27, (Bytes, Bytes))>))
                }
                Some(v3) => {
                    let m4438 = f498(v3)?;
                    match m4438 {
                        Ok(v5) => {
                            let m4439 = f499(v5, List::cons(f8(Str::lit("extension")), List::cons(f8(Str::lit("restriction")), List::<Bytes>::nil())));
                            match m4439 {
                                None => {
                                    Ok(Ok::<Option<(Adt27, (Bytes, Bytes))>, Bytes>(None::<(Adt27, (Bytes, Bytes))>))
                                }
                                Some(v6) => {
                                    let m4440 = f308(v6.clone(), Str::lit("base"));
                                    match m4440 {
                                        None => {
                                            Ok(Ok::<Option<(Adt27, (Bytes, Bytes))>, Bytes>(None::<(Adt27, (Bytes, Bytes))>))
                                        }
                                        Some(v7) => {
                                            let m4442 = f133(match v6.clone() {
                                                Adt27::C0(_, _, _, _, h4441) => {
                                                    h4441
                                                }
                                            }, v7)?;
                                            match m4442 {
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

pub fn f623(v0: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f650(f258(v0.clone())?, v0, List::<Bytes>::nil())
}

pub fn f624(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m4443 = v0;
    match m4443.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m4444 = v1;
            match m4444.uncons() {
                None => {
                    List::<Bytes>::nil()
                }
                Some((v4, v5)) => {
                    List::cons(f220(f8(Str::lit("    {name: ")), v2, f220(f8(Str::lit(", fields: [")), v4, f8(Str::lit("]}")))), f624(v3, v5))
                }
            }
        }
    }
}

pub fn f625(v0: u64, v1: Adt25, v2: Bytes, v3: Bytes, v4: Adt27, v5: Adt36) -> R<Result<Adt36, Bytes>> {
    let m4445 = v0;
    if m4445 == 0 {
        Ok(Err::<Adt36, Bytes>(f500(f8(Str::lit("the type derivations nest deeper than the schema documents have bytes, so the types derive from each other in a cycle (XSD 1.1 Part 1, section 3.4.6.1)")), v2)))
    } else {
        let v6 = m4445 - 1;
        if f568(v4.clone(), Str::lit("mixed"), Str::lit("true")) {
            Ok(Err::<Adt36, Bytes>(f500(f8(Str::lit("a mixed content type interleaves character data with its elements, which a LexLean field list does not keep in order, so it is not translated (XSD 1.1 Part 1, section 3.4.2.3)")), v2)))
        } else {
            let m4446 = f498(v4)?;
            match m4446 {
                Ok(v8) => {
                    let m4447 = f499(v8.clone(), List::cons(f8(Str::lit("openContent")), List::cons(f8(Str::lit("assert")), List::<Bytes>::nil())));
                    match m4447 {
                        None => {
                            let m4449 = f499(v8.clone(), List::cons(f8(Str::lit("simpleContent")), List::cons(f8(Str::lit("complexContent")), List::<Bytes>::nil())));
                            match m4449 {
                                None => {
                                    let m4450 = f576(v6, v1.clone(), v2.clone(), v3.clone(), true, v8.clone(), v5)?;
                                    match m4450 {
                                        Ok(v11) => {
                                            f502(v6, v1, v2, v3, v8, v11)
                                        }
                                        Err(v10) => {
                                            Ok(Err::<Adt36, Bytes>(v10))
                                        }
                                    }
                                }
                                Some(v12) => {
                                    if f568(v12.clone(), Str::lit("mixed"), Str::lit("true")) {
                                        Ok(Err::<Adt36, Bytes>(f500(f8(Str::lit("a mixed content type interleaves character data with its elements, which a LexLean field list does not keep in order, so it is not translated (XSD 1.1 Part 1, section 3.4.2.3)")), v2)))
                                    } else {
                                        let m4451 = f498(v12.clone())?;
                                        match m4451 {
                                            Ok(v14) => {
                                                let m4452 = f499(v14, List::cons(f8(Str::lit("extension")), List::cons(f8(Str::lit("restriction")), List::<Bytes>::nil())));
                                                match m4452 {
                                                    None => {
                                                        Ok(Err::<Adt36, Bytes>(f500(f8(Str::lit("simpleContent and complexContent must have an extension or a restriction child (XSD 1.1 Part 1, sections 3.4.2.2 and 3.4.2.3)")), v2)))
                                                    }
                                                    Some(v15) => {
                                                        let m4453 = f308(v15.clone(), Str::lit("base"));
                                                        match m4453 {
                                                            None => {
                                                                Ok(Err::<Adt36, Bytes>(f500(f8(Str::lit("an extension or restriction must name its base type (XSD 1.1 Part 1, section 3.4.3)")), v2)))
                                                            }
                                                            Some(v16) => {
                                                                let m4455 = f133(match v15.clone() {
                                                                    Adt27::C0(_, _, _, _, h4454) => {
                                                                        h4454
                                                                    }
                                                                }, v16.clone())?;
                                                                match m4455 {
                                                                    Ok(v18) => {
                                                                        let m4456 = f498(v15.clone())?;
                                                                        match m4456 {
                                                                            Ok(v20) => {
                                                                                let v21: bool = if f32({
                                                                                    let (h4457, _) = v18.clone();
                                                                                    h4457
                                                                                }, f208()) {
                                                                                    f32({
                                                                                        let (_, h4458) = v18.clone();
                                                                                        h4458
                                                                                    }, f8(Str::lit("anyType")))
                                                                                } else {
                                                                                    false
                                                                                };
                                                                                if if f307(v15.clone(), Str::lit("restriction")) {
                                                                                    f306(v21)
                                                                                } else {
                                                                                    false
                                                                                } {
                                                                                    Ok(Err::<Adt36, Bytes>(f500(f220(f8(Str::lit("a restriction of the complex type ")), v16, f8(Str::lit(" is not translated, because the attribute uses it inherits and the content it narrows are not read yet (XSD 1.1 Part 1, sections 3.4.2.2 and 3.4.2.3)"))), v2)))
                                                                                } else {
                                                                                    let m4473 = if if v21 {
                                                                                        true
                                                                                    } else {
                                                                                        f32(match v12 {
                                                                                            Adt27::C0(_, h4459, _, _, _) => {
                                                                                                h4459
                                                                                            }
                                                                                        }, f8(Str::lit("complexContent")))
                                                                                    } {
                                                                                        if v21 {
                                                                                            Ok::<Adt36, Bytes>(v5)
                                                                                        } else {
                                                                                            let m4460 = f581(v1.clone(), Str::lit("complexType"), v18.clone());
                                                                                            match m4460 {
                                                                                                None => {
                                                                                                    Err::<Adt36, Bytes>(f500(f220(f8(Str::lit("the base type ")), v16, f8(Str::lit(" of a complex content extension is not a complex type of this schema set (XSD 1.1 Part 1, section 3.4.2.3)"))), v2.clone()))
                                                                                                }
                                                                                                Some(v22) => {
                                                                                                    let m4464 = f625(v6, v1.clone(), f211({
                                                                                                        let (_, h4461) = v18;
                                                                                                        h4461
                                                                                                    })?, match v22.clone() {
                                                                                                        Adt26::C0(_, h4462, _, _) => {
                                                                                                            h4462
                                                                                                        }
                                                                                                    }, match v22 {
                                                                                                        Adt26::C0(_, _, _, h4463) => {
                                                                                                            h4463
                                                                                                        }
                                                                                                    }, v5)?;
                                                                                                    match m4464 {
                                                                                                        Ok(v24) => {
                                                                                                            Ok::<Adt36, Bytes>(f651(v24))
                                                                                                        }
                                                                                                        Err(v23) => {
                                                                                                            Err::<Adt36, Bytes>(v23)
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        let m4465 = f581(v1.clone(), Str::lit("complexType"), v18.clone());
                                                                                        match m4465 {
                                                                                            None => {
                                                                                                let m4470 = f574(v1.clone(), v15, v16)?;
                                                                                                match m4470 {
                                                                                                    Ok(v29) => {
                                                                                                        Ok::<Adt36, Bytes>(f583(v5, Adt37::C0(f8(Str::lit("value")), match v29.clone() {
                                                                                                            Adt24::C0(h4471, _) => {
                                                                                                                h4471
                                                                                                            }
                                                                                                        }, match v29 {
                                                                                                            Adt24::C0(_, h4472) => {
                                                                                                                h4472
                                                                                                            }
                                                                                                        })))
                                                                                                    }
                                                                                                    Err(v28) => {
                                                                                                        Err::<Adt36, Bytes>(f500(v28, v2.clone()))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            Some(v25) => {
                                                                                                let m4469 = f625(v6, v1.clone(), f211({
                                                                                                    let (_, h4466) = v18;
                                                                                                    h4466
                                                                                                })?, match v25.clone() {
                                                                                                    Adt26::C0(_, h4467, _, _) => {
                                                                                                        h4467
                                                                                                    }
                                                                                                }, match v25 {
                                                                                                    Adt26::C0(_, _, _, h4468) => {
                                                                                                        h4468
                                                                                                    }
                                                                                                }, v5)?;
                                                                                                match m4469 {
                                                                                                    Ok(v27) => {
                                                                                                        Ok::<Adt36, Bytes>(f651(v27))
                                                                                                    }
                                                                                                    Err(v26) => {
                                                                                                        Err::<Adt36, Bytes>(v26)
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    };
                                                                                    match m4473 {
                                                                                        Ok(v31) => {
                                                                                            let m4474 = f576(v6, v1.clone(), v2.clone(), v3.clone(), true, v20.clone(), v31)?;
                                                                                            match m4474 {
                                                                                                Ok(v33) => {
                                                                                                    f502(v6, v1, v2, v3, v20, v33)
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
                                                                        Ok(Err::<Adt36, Bytes>(f500(v17, v2)))
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
                            Ok(Err::<Adt36, Bytes>(f500(f220(f8(Str::lit("xs:")), match v9 {
                                Adt27::C0(_, h4448, _, _, _) => {
                                    h4448
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

pub fn f626(v0: Str, v1: Bytes) -> Bytes {
    f220(f8(Str::lit("(")), f8(v0), f220(f8(Str::lit(" ")), v1, f8(Str::lit(")"))))
}

pub fn f627(v0: u64, v1: Adt25, v2: Bytes, v3: Bytes, v4: bool, v5: Adt27, v6: Adt36) -> R<Result<Adt36, Bytes>> {
    let m4475 = v0;
    if m4475 == 0 {
        Ok(Err::<Adt36, Bytes>(f500(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v7 = m4475 - 1;
        if f307(v5.clone(), Str::lit("element")) {
            let m4476 = f652(v7, v1, v2, v3, v5.clone())?;
            match m4476 {
                Ok(v9) => {
                    let m4482 = f653(v5, {
                        let m4481 = {
                            let m4480 = match v9.clone() {
                                Adt40::C0(_, h4479) => {
                                    h4479
                                }
                            };
                            match m4480 {
                                Adt35::C0(h4478, _) => {
                                    h4478
                                }
                            }
                        };
                        match m4481 {
                            Adt24::C0(h4477, _) => {
                                h4477
                            }
                        }
                    });
                    match m4482 {
                        None => {
                            Ok(Ok::<Adt36, Bytes>(v6))
                        }
                        Some(v10) => {
                            Ok(Ok::<Adt36, Bytes>(f630(f583(v6, Adt37::C0(match v9.clone() {
                                Adt40::C0(h4483, _) => {
                                    h4483
                                }
                            }, v10, {
                                let m4488 = {
                                    let m4487 = match v9.clone() {
                                        Adt40::C0(_, h4486) => {
                                            h4486
                                        }
                                    };
                                    match m4487 {
                                        Adt35::C0(h4485, _) => {
                                            h4485
                                        }
                                    }
                                };
                                match m4488 {
                                    Adt24::C0(_, h4484) => {
                                        h4484
                                    }
                                }
                            })), {
                                let m4491 = match v9 {
                                    Adt40::C0(_, h4490) => {
                                        h4490
                                    }
                                };
                                match m4491 {
                                    Adt35::C0(_, h4489) => {
                                        h4489
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
            if f307(v5.clone(), Str::lit("any")) {
                let m4492 = f653(v5, f406());
                match m4492 {
                    None => {
                        Ok(Ok::<Adt36, Bytes>(v6))
                    }
                    Some(v11) => {
                        Ok(Ok::<Adt36, Bytes>(f583(v6, Adt37::C0(f8(Str::lit("any")), v11, List::cons(f406(), List::<Bytes>::nil())))))
                    }
                }
            } else {
                if f307(v5.clone(), Str::lit("group")) {
                    let m4493 = f308(v5.clone(), Str::lit("ref"));
                    match m4493 {
                        None => {
                            Ok(Err::<Adt36, Bytes>(f500(f8(Str::lit("a model group reference must have a ref (XSD 1.1 Part 1, section 3.7.2)")), v2)))
                        }
                        Some(v12) => {
                            let m4495 = f133(match v5.clone() {
                                Adt27::C0(_, _, _, _, h4494) => {
                                    h4494
                                }
                            }, v12.clone())?;
                            match m4495 {
                                Ok(v14) => {
                                    let m4496 = f581(v1, Str::lit("group"), v14.clone());
                                    match m4496 {
                                        None => {
                                            Ok(Err::<Adt36, Bytes>(f500(f220(f8(Str::lit("the model group ")), v12, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                        }
                                        Some(_) => {
                                            let v16: Bytes = f211({
                                                let (_, h4497) = v14.clone();
                                                h4497
                                            })?;
                                            let m4498 = f653(v5, v16.clone());
                                            match m4498 {
                                                None => {
                                                    Ok(Ok::<Adt36, Bytes>(v6))
                                                }
                                                Some(v17) => {
                                                    Ok(Ok::<Adt36, Bytes>(f583(v6, Adt37::C0({
                                                        let (_, h4499) = v14;
                                                        h4499
                                                    }, v17, List::cons(v16, List::<Bytes>::nil())))))
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(v13) => {
                                    Ok(Err::<Adt36, Bytes>(f500(v13, v2)))
                                }
                            }
                        }
                    }
                } else {
                    if if f307(v5.clone(), Str::lit("sequence")) {
                        true
                    } else {
                        if f307(v5.clone(), Str::lit("choice")) {
                            true
                        } else {
                            f307(v5.clone(), Str::lit("all"))
                        }
                    } {
                        if if v4 {
                            if f654(v5.clone()) {
                                f306(f307(v5.clone(), Str::lit("choice")))
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m4500 = f498(v5)?;
                            match m4500 {
                                Ok(v19) => {
                                    f576(v7, v1, v2, v3, true, v19, v6)
                                }
                                Err(v18) => {
                                    Ok(Err::<Adt36, Bytes>(v18))
                                }
                            }
                        } else {
                            let v20: Bytes = match v5.clone() {
                                Adt27::C0(_, h4501, _, _, _) => {
                                    h4501
                                }
                            };
                            let v21: u64 = f655(v6.clone(), v20.clone())?;
                            let v22: Bytes = {
                                let a4502 = v21;
                                let a4503 = 1u64;
                                if nat_eq(a4502, a4503) {
                                    Bytes::lit(&[])
                                } else {
                                    f76(v21)?
                                }
                            };
                            let v23: Bytes = f220(v2, f211(v20.clone())?, v22.clone());
                            let m4504 = f501(v7, v1, v23.clone(), v3, v5.clone())?;
                            match m4504 {
                                Ok(v25) => {
                                    let m4505 = f653(v5, v23.clone());
                                    match m4505 {
                                        None => {
                                            Ok(Ok::<Adt36, Bytes>(f656(v6, v20)?))
                                        }
                                        Some(v26) => {
                                            Ok(Ok::<Adt36, Bytes>(f630(f583(f656(v6, v20.clone())?, Adt37::C0({
                                                let a4506 = v20;
                                                let a4507 = v22;
                                                append_bytes(a4506, a4507)
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

pub fn f628(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m4508 = v0;
    match m4508.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m4509 = v1;
            match m4509.uncons() {
                None => {
                    List::<Bytes>::nil()
                }
                Some((v4, v5)) => {
                    List::cons(f220(f8(Str::lit("    {name: ")), v2, f220(f8(Str::lit(", type: ")), v4, f8(Str::lit("}")))), f628(v3, v5))
                }
            }
        }
    }
}

pub fn f629(p4515: List<Adt26>, v1: Bytes, v2: Bytes, v3: Bytes) -> Option<Adt26> {
    let m4514 = p4515;
    {
        let mut m4514 = m4514;
        loop {
            let step = {
                let v0 = m4514;
                let m4510 = v0;
                match m4510.uncons() {
                    None => {
                        Err::<List<Adt26>, Option<Adt26>>(None::<Adt26>)
                    }
                    Some((v4, v5)) => {
                        if if f32(match v4.clone() {
                            Adt26::C0(h4511, _, _, _) => {
                                h4511
                            }
                        }, v1.clone()) {
                            if f32(match v4.clone() {
                                Adt26::C0(_, h4512, _, _) => {
                                    h4512
                                }
                            }, v2.clone()) {
                                f32(match v4.clone() {
                                    Adt26::C0(_, _, h4513, _) => {
                                        h4513
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
                Ok(next) => { m4514 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f630(v0: Adt36, v1: List<Adt28>) -> Adt36 {
    Adt36::C0(match v0.clone() {
        Adt36::C0(h4516, _, _, _, _) => {
            h4516
        }
    }, f403(match v0.clone() {
        Adt36::C0(_, h4517, _, _, _) => {
            h4517
        }
    }, v1), match v0.clone() {
        Adt36::C0(_, _, h4518, _, _) => {
            h4518
        }
    }, match v0.clone() {
        Adt36::C0(_, _, _, h4519, _) => {
            h4519
        }
    }, match v0 {
        Adt36::C0(_, _, _, _, h4520) => {
            h4520
        }
    })
}

pub fn f631(v0: List<Adt37>, v1: List<Adt37>) -> List<Adt37> {
    let m4521 = v0;
    match m4521.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f631(v3, v1))
        }
    }
}

pub fn f632(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f657(v1, v0, v2)
}

pub fn f633(v0: u64, v1: Bytes, v2: u64, v3: u64) -> R<Option<u64>> {
    f658(v1, v2, v0, v3)
}

pub fn f634(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let a4524 = {
        let (h4522, _) = v1;
        h4522
    };
    let a4525 = {
        let a4523 = v0.clone();
        length_bytes(a4523)
    };
    if nat_lt(a4524, a4525) {
        Ok(Some((f318(v0, {
            let (h4526, _) = v1;
            h4526
        })?, {
            let a4528 = {
                let (_, h4527) = v1;
                h4527
            };
            let a4529 = 1u64;
            nat_add(a4528, a4529)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f635(v0: u64) -> Option<u64> {
    let a4530 = 48u64;
    let a4531 = v0;
    if if nat_le(a4530, a4531) {
        let a4532 = v0;
        let a4533 = 57u64;
        nat_le(a4532, a4533)
    } else {
        false
    } {
        Some({
            let a4534 = v0;
            let a4535 = 48u64;
            nat_sub(a4534, a4535)
        })
    } else {
        let a4536 = 97u64;
        let a4537 = v0;
        if if nat_le(a4536, a4537) {
            let a4538 = v0;
            let a4539 = 102u64;
            nat_le(a4538, a4539)
        } else {
            false
        } {
            Some({
                let a4540 = v0;
                let a4541 = 87u64;
                nat_sub(a4540, a4541)
            })
        } else {
            let a4542 = 65u64;
            let a4543 = v0;
            if if nat_le(a4542, a4543) {
                let a4544 = v0;
                let a4545 = 70u64;
                nat_le(a4544, a4545)
            } else {
                false
            } {
                Some({
                    let a4546 = v0;
                    let a4547 = 55u64;
                    nat_sub(a4546, a4547)
                })
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f636(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f659(v0, v1)
}

pub fn f637(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f660(v0, v1)
}

pub fn f638(v0: Bytes, v1: u64, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    let v3: u64 = {
        let (h4548, _) = v2;
        h4548
    };
    let v4: u64 = f232(v0.clone(), v3)?;
    let (_, h4549) = v2;
    if h4549 {
        Ok(None::<(u64, bool)>)
    } else {
        if f314(v4) {
            Ok(Some(({
                let a4550 = v3;
                let a4551 = 1u64;
                nat_add(a4550, a4551)?
            }, false)))
        } else {
            let a4552 = v4;
            let a4553 = 35u64;
            if if nat_eq(a4552, a4553) {
                let a4554 = v3;
                let a4555 = 0u64;
                if if nat_eq(a4554, a4555) {
                    true
                } else {
                    f314(f232(v0.clone(), {
                        let a4556 = v3;
                        let a4557 = 1u64;
                        nat_sub(a4556, a4557)
                    })?)
                } {
                    true
                } else {
                    let a4560 = f232(v0.clone(), {
                        let a4558 = v3;
                        let a4559 = 1u64;
                        nat_sub(a4558, a4559)
                    })?;
                    let a4561 = 10u64;
                    nat_eq(a4560, a4561)
                }
            } else {
                false
            } {
                Ok(Some((f421(v0, v3)?, false)))
            } else {
                let a4562 = v4;
                let a4563 = 10u64;
                if nat_eq(a4562, a4563) {
                    let v5: u64 = {
                        let a4564 = v3;
                        let a4565 = 1u64;
                        nat_add(a4564, a4565)?
                    };
                    let v6: u64 = f319(v0.clone(), v5)?;
                    let v7: u64 = f317(v0.clone(), v6)?;
                    if if f230(v0.clone(), v5)? {
                        true
                    } else {
                        let a4568 = {
                            let a4566 = v6;
                            let a4567 = v5;
                            nat_sub(a4566, a4567)
                        };
                        let a4569 = v1;
                        if if nat_lt(a4568, a4569) {
                            f234(f315(f232(v0.clone(), v7)?))
                        } else {
                            false
                        } {
                            f234({
                                let a4570 = f232(v0, v7)?;
                                let a4571 = 35u64;
                                nat_eq(a4570, a4571)
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

pub fn f639(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f317(v0.clone(), {
        let (h4572, _) = v1;
        h4572
    })?;
    let a4573 = f232(v0, v2)?;
    let a4574 = 10u64;
    if nat_eq(a4573, a4574) {
        Ok(Some(({
            let a4575 = v2;
            let a4576 = 1u64;
            nat_add(a4575, a4576)?
        }, {
            let a4578 = {
                let (_, h4577) = v1;
                h4577
            };
            let a4579 = 1u64;
            nat_add(a4578, a4579)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f640(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f661(v1, v0, v2)
}

pub fn f641() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f642(v0: Fn15, p4583: List<List<(Bytes, Bytes)>>, p4584: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m4582 = (p4583, p4584);
    {
        let mut m4582 = m4582;
        loop {
            let step = {
                let (v1, v2) = m4582;
                let m4580 = v2;
                match m4580.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c4581: Fn15 = v0.clone();
                            c4581.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m4582 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f643(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f644(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a4586 = {
        let a4585 = v0.clone();
        length_list(a4585)
    };
    let a4587 = 1u64;
    if nat_le(a4586, a4587) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f662(v0)?))
    }
}

pub fn f645(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4632 = {
            let a4608 = {
                let a4596 = {
                    let a4590 = {
                        let a4588 = v1;
                        let a4589 = 128u8;
                        fixed_u8::bit_and(a4588, a4589)
                    };
                    let a4591 = 128u8;
                    if equal(a4590, a4591) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4597 = {
                    let a4594 = {
                        let a4592 = v1;
                        let a4593 = 64u8;
                        fixed_u8::bit_and(a4592, a4593)
                    };
                    let a4595 = 64u8;
                    if equal(a4594, a4595) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4596, a4597)?
            };
            let a4609 = {
                let a4606 = {
                    let a4600 = {
                        let a4598 = v1;
                        let a4599 = 32u8;
                        fixed_u8::bit_and(a4598, a4599)
                    };
                    let a4601 = 32u8;
                    if equal(a4600, a4601) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4607 = {
                    let a4604 = {
                        let a4602 = v1;
                        let a4603 = 16u8;
                        fixed_u8::bit_and(a4602, a4603)
                    };
                    let a4605 = 16u8;
                    if equal(a4604, a4605) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4606, a4607)?
            };
            nat_add(a4608, a4609)?
        };
        let a4633 = {
            let a4630 = {
                let a4618 = {
                    let a4612 = {
                        let a4610 = v1;
                        let a4611 = 8u8;
                        fixed_u8::bit_and(a4610, a4611)
                    };
                    let a4613 = 8u8;
                    if equal(a4612, a4613) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4619 = {
                    let a4616 = {
                        let a4614 = v1;
                        let a4615 = 4u8;
                        fixed_u8::bit_and(a4614, a4615)
                    };
                    let a4617 = 4u8;
                    if equal(a4616, a4617) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4618, a4619)?
            };
            let a4631 = {
                let a4628 = {
                    let a4622 = {
                        let a4620 = v1;
                        let a4621 = 2u8;
                        fixed_u8::bit_and(a4620, a4621)
                    };
                    let a4623 = 2u8;
                    if equal(a4622, a4623) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4629 = {
                    let a4626 = {
                        let a4624 = v1;
                        let a4625 = 1u8;
                        fixed_u8::bit_and(a4624, a4625)
                    };
                    let a4627 = 1u8;
                    if equal(a4626, a4627) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4628, a4629)?
            };
            nat_add(a4630, a4631)?
        };
        nat_add(a4632, a4633)?
    };
    let a4634 = 48u64;
    let a4635 = v2;
    if if nat_le(a4634, a4635) {
        let a4636 = v2;
        let a4637 = 57u64;
        nat_le(a4636, a4637)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a4638 = 97u64;
        let a4639 = v2;
        if nat_le(a4638, a4639) {
            let a4640 = v2;
            let a4641 = 102u64;
            Ok(nat_le(a4640, a4641))
        } else {
            Ok(false)
        }
    }
}

pub fn f646() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f647(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4686 = {
            let a4662 = {
                let a4650 = {
                    let a4644 = {
                        let a4642 = v1;
                        let a4643 = 128u8;
                        fixed_u8::bit_and(a4642, a4643)
                    };
                    let a4645 = 128u8;
                    if equal(a4644, a4645) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4651 = {
                    let a4648 = {
                        let a4646 = v1;
                        let a4647 = 64u8;
                        fixed_u8::bit_and(a4646, a4647)
                    };
                    let a4649 = 64u8;
                    if equal(a4648, a4649) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4650, a4651)?
            };
            let a4663 = {
                let a4660 = {
                    let a4654 = {
                        let a4652 = v1;
                        let a4653 = 32u8;
                        fixed_u8::bit_and(a4652, a4653)
                    };
                    let a4655 = 32u8;
                    if equal(a4654, a4655) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4661 = {
                    let a4658 = {
                        let a4656 = v1;
                        let a4657 = 16u8;
                        fixed_u8::bit_and(a4656, a4657)
                    };
                    let a4659 = 16u8;
                    if equal(a4658, a4659) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4660, a4661)?
            };
            nat_add(a4662, a4663)?
        };
        let a4687 = {
            let a4684 = {
                let a4672 = {
                    let a4666 = {
                        let a4664 = v1;
                        let a4665 = 8u8;
                        fixed_u8::bit_and(a4664, a4665)
                    };
                    let a4667 = 8u8;
                    if equal(a4666, a4667) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4673 = {
                    let a4670 = {
                        let a4668 = v1;
                        let a4669 = 4u8;
                        fixed_u8::bit_and(a4668, a4669)
                    };
                    let a4671 = 4u8;
                    if equal(a4670, a4671) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4672, a4673)?
            };
            let a4685 = {
                let a4682 = {
                    let a4676 = {
                        let a4674 = v1;
                        let a4675 = 2u8;
                        fixed_u8::bit_and(a4674, a4675)
                    };
                    let a4677 = 2u8;
                    if equal(a4676, a4677) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4683 = {
                    let a4680 = {
                        let a4678 = v1;
                        let a4679 = 1u8;
                        fixed_u8::bit_and(a4678, a4679)
                    };
                    let a4681 = 1u8;
                    if equal(a4680, a4681) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4682, a4683)?
            };
            nat_add(a4684, a4685)?
        };
        nat_add(a4686, a4687)?
    };
    let a4688 = v2;
    let a4689 = 57u64;
    if nat_le(a4688, a4689) {
        let a4690 = v2;
        let a4691 = 48u64;
        Ok(nat_sub(a4690, a4691))
    } else {
        let a4692 = v2;
        let a4693 = 87u64;
        Ok(nat_sub(a4692, a4693))
    }
}

pub fn f648(v0: u64) -> bool {
    let a4694 = 48u64;
    let a4695 = v0;
    if nat_le(a4694, a4695) {
        let a4696 = v0;
        let a4697 = 55u64;
        nat_le(a4696, a4697)
    } else {
        false
    }
}

pub fn f649(v0: u64) -> bool {
    if f349(v0) {
        true
    } else {
        if f452(v0) {
            true
        } else {
            let a4698 = v0;
            let a4699 = 95u64;
            if nat_eq(a4698, a4699) {
                true
            } else {
                let a4700 = v0;
                let a4701 = 45u64;
                nat_eq(a4700, a4701)
            }
        }
    }
}

pub fn f650(v0: bool, v1: List<Bytes>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m4702 = v1;
    match m4702.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m4703 = f377(v0, v3.clone())?;
            match m4703 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f220(f8(Str::lit("the component name ")), v3, f8(Str::lit(" derives no LexLean name")))))
                }
                Some(v5) => {
                    if f173(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f220(f8(Str::lit("two components derive the name ")), v5, f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n; when those names are not distinct, every name keeps its letters and digits and writes each other character as a C11 universal character name)")))))
                    } else {
                        let m4704 = f650(v0, v4, List::cons(v5.clone(), v2))?;
                        match m4704 {
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

pub fn f651(v0: Adt36) -> Adt36 {
    Adt36::C0(match v0.clone() {
        Adt36::C0(h4705, _, _, _, _) => {
            h4705
        }
    }, match v0 {
        Adt36::C0(_, h4706, _, _, _) => {
            h4706
        }
    }, 0u64, 0u64, 0u64)
}

pub fn f652(v0: u64, v1: Adt25, v2: Bytes, v3: Bytes, v4: Adt27) -> R<Result<Adt40, Bytes>> {
    let m4707 = v0;
    if m4707 == 0 {
        Ok(Err::<Adt40, Bytes>(f500(f8(Str::lit("the element declarations nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m4707 - 1;
        let m4708 = f308(v4.clone(), Str::lit("ref"));
        match m4708 {
            None => {
                let m4719 = f308(v4.clone(), Str::lit("name"));
                match m4719 {
                    None => {
                        Ok(Err::<Adt40, Bytes>(f500(f8(Str::lit("a local element declaration must have a name or a ref (XSD 1.1 Part 1, section 3.3.3)")), v2)))
                    }
                    Some(v12) => {
                        let m4722 = f496(v5, v1, {
                            let a4720 = v2;
                            let a4721 = f211(v12.clone())?;
                            append_bytes(a4720, a4721)
                        }, v3, v4)?;
                        match m4722 {
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
                let m4710 = f133(match v4 {
                    Adt27::C0(_, _, _, _, h4709) => {
                        h4709
                    }
                }, v6.clone())?;
                match m4710 {
                    Ok(v8) => {
                        let m4711 = f581(v1.clone(), Str::lit("element"), v8.clone());
                        match m4711 {
                            None => {
                                Ok(Err::<Adt40, Bytes>(f500(f220(f8(Str::lit("the element ")), v6, f8(Str::lit(" is not declared by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                            }
                            Some(v9) => {
                                if if f568(match v9.clone() {
                                    Adt26::C0(_, _, _, h4712) => {
                                        h4712
                                    }
                                }, Str::lit("abstract"), Str::lit("true")) {
                                    true
                                } else {
                                    f663(match v1.clone() {
                                        Adt25::C0(h4713, _) => {
                                            h4713
                                        }
                                    }, v8.clone())?
                                } {
                                    Ok(Err::<Adt40, Bytes>(f500(f220(f8(Str::lit("the element ")), v6, f8(Str::lit(" heads a substitution group, so its particle also admits the members of that group, which is not translated (XSD 1.1 Part 1, section 3.3.6.4)"))), v2)))
                                } else {
                                    let m4717 = f496(v5, v1, f211({
                                        let (_, h4714) = v8.clone();
                                        h4714
                                    })?, match v9.clone() {
                                        Adt26::C0(_, h4715, _, _) => {
                                            h4715
                                        }
                                    }, match v9 {
                                        Adt26::C0(_, _, _, h4716) => {
                                            h4716
                                        }
                                    })?;
                                    match m4717 {
                                        Ok(v11) => {
                                            Ok(Ok::<Adt40, Bytes>(Adt40::C0({
                                                let (_, h4718) = v8;
                                                h4718
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
                        Ok(Err::<Adt40, Bytes>(f500(v7, v2)))
                    }
                }
            }
        }
    }
}

pub fn f653(v0: Adt27, v1: Bytes) -> Option<Bytes> {
    let v2: Bytes = f311(v0.clone(), Str::lit("maxOccurs"), Str::lit("1"));
    if f32(v2.clone(), f8(Str::lit("0"))) {
        None::<Bytes>
    } else {
        if f32(v2, f8(Str::lit("1"))) {
            if f568(v0, Str::lit("minOccurs"), Str::lit("0")) {
                Some(f626(Str::lit("Option"), v1))
            } else {
                Some(v1)
            }
        } else {
            Some(f626(Str::lit("List"), v1))
        }
    }
}

pub fn f654(v0: Adt27) -> bool {
    if f32(f311(v0.clone(), Str::lit("minOccurs"), Str::lit("1")), f8(Str::lit("1"))) {
        f32(f311(v0, Str::lit("maxOccurs"), Str::lit("1")), f8(Str::lit("1")))
    } else {
        false
    }
}

pub fn f655(v0: Adt36, v1: Bytes) -> R<u64> {
    if f32(v1.clone(), f8(Str::lit("sequence"))) {
        let a4724 = match v0 {
            Adt36::C0(_, _, h4723, _, _) => {
                h4723
            }
        };
        let a4725 = 1u64;
        nat_add(a4724, a4725)
    } else {
        if f32(v1, f8(Str::lit("choice"))) {
            let a4727 = match v0 {
                Adt36::C0(_, _, _, h4726, _) => {
                    h4726
                }
            };
            let a4728 = 1u64;
            nat_add(a4727, a4728)
        } else {
            let a4730 = match v0 {
                Adt36::C0(_, _, _, _, h4729) => {
                    h4729
                }
            };
            let a4731 = 1u64;
            nat_add(a4730, a4731)
        }
    }
}

pub fn f656(v0: Adt36, v1: Bytes) -> R<Adt36> {
    if f32(v1.clone(), f8(Str::lit("sequence"))) {
        Ok(Adt36::C0(match v0.clone() {
            Adt36::C0(h4732, _, _, _, _) => {
                h4732
            }
        }, match v0.clone() {
            Adt36::C0(_, h4733, _, _, _) => {
                h4733
            }
        }, {
            let a4735 = match v0.clone() {
                Adt36::C0(_, _, h4734, _, _) => {
                    h4734
                }
            };
            let a4736 = 1u64;
            nat_add(a4735, a4736)?
        }, match v0.clone() {
            Adt36::C0(_, _, _, h4737, _) => {
                h4737
            }
        }, match v0 {
            Adt36::C0(_, _, _, _, h4738) => {
                h4738
            }
        }))
    } else {
        if f32(v1, f8(Str::lit("choice"))) {
            Ok(Adt36::C0(match v0.clone() {
                Adt36::C0(h4739, _, _, _, _) => {
                    h4739
                }
            }, match v0.clone() {
                Adt36::C0(_, h4740, _, _, _) => {
                    h4740
                }
            }, match v0.clone() {
                Adt36::C0(_, _, h4741, _, _) => {
                    h4741
                }
            }, {
                let a4743 = match v0.clone() {
                    Adt36::C0(_, _, _, h4742, _) => {
                        h4742
                    }
                };
                let a4744 = 1u64;
                nat_add(a4743, a4744)?
            }, match v0 {
                Adt36::C0(_, _, _, _, h4745) => {
                    h4745
                }
            }))
        } else {
            Ok(Adt36::C0(match v0.clone() {
                Adt36::C0(h4746, _, _, _, _) => {
                    h4746
                }
            }, match v0.clone() {
                Adt36::C0(_, h4747, _, _, _) => {
                    h4747
                }
            }, match v0.clone() {
                Adt36::C0(_, _, h4748, _, _) => {
                    h4748
                }
            }, match v0.clone() {
                Adt36::C0(_, _, _, h4749, _) => {
                    h4749
                }
            }, {
                let a4751 = match v0 {
                    Adt36::C0(_, _, _, _, h4750) => {
                        h4750
                    }
                };
                let a4752 = 1u64;
                nat_add(a4751, a4752)?
            }))
        }
    }
}

pub fn f657(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f589(v0.clone(), {
        let (h4753, _) = v2;
        h4753
    }, v1)?;
    let a4754 = f232(v0, v3)?;
    let a4755 = 10u64;
    if nat_eq(a4754, a4755) {
        Ok(Some(({
            let a4756 = v3;
            let a4757 = 1u64;
            nat_add(a4756, a4757)?
        }, {
            let a4759 = {
                let (_, h4758) = v2;
                h4758
            };
            let a4760 = 1u64;
            nat_add(a4759, a4760)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f658(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a4761 = f232(v0, v3)?;
    let a4762 = 32u64;
    if if nat_eq(a4761, a4762) {
        let a4765 = {
            let a4763 = v3;
            let a4764 = v1;
            nat_sub(a4763, a4764)
        };
        let a4766 = v2;
        nat_lt(a4765, a4766)
    } else {
        false
    } {
        Ok(Some({
            let a4767 = v3;
            let a4768 = 1u64;
            nat_add(a4767, a4768)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f659(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f232(v0, v1)?;
    let a4769 = v2;
    let a4770 = 34u64;
    if if if if nat_eq(a4769, a4770) {
        true
    } else {
        let a4771 = v2;
        let a4772 = 92u64;
        nat_eq(a4771, a4772)
    } {
        true
    } else {
        f314(v2)
    } {
        true
    } else {
        f315(v2)
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a4773 = v1;
            let a4774 = 1u64;
            nat_add(a4773, a4774)?
        }))
    }
}

pub fn f660(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f232(v0, v1)?;
    let a4775 = v2;
    let a4776 = 39u64;
    if if if nat_eq(a4775, a4776) {
        true
    } else {
        f314(v2)
    } {
        true
    } else {
        f315(v2)
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a4777 = v1;
            let a4778 = 1u64;
            nat_add(a4777, a4778)?
        }))
    }
}

pub fn f661(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m4780 = f635(f232(v0, {
        let (h4779, _) = v2;
        h4779
    })?);
    match m4780 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v3) => {
            let a4781 = v3;
            let a4782 = v1;
            if nat_lt(a4781, a4782) {
                Ok(Some(({
                    let a4784 = {
                        let (h4783, _) = v2;
                        h4783
                    };
                    let a4785 = 1u64;
                    nat_add(a4784, a4785)?
                }, {
                    let a4789 = {
                        let a4787 = {
                            let (_, h4786) = v2;
                            h4786
                        };
                        let a4788 = v1;
                        nat_mul(a4787, a4788)?
                    };
                    let a4790 = v3;
                    nat_add(a4789, a4790)?
                })))
            } else {
                Ok(None::<(u64, u64)>)
            }
        }
    }
}

pub fn f662(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f606({
        let (_, h4791) = {
            let (h4792, _) = f664(Fn16::F665, {
                let a4794 = {
                    let a4793 = v0.clone();
                    length_list(a4793)
                };
                let a4795 = 1u64;
                nat_add(a4794, a4795)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h4792
        };
        h4791
    }))
}

pub fn f663(p4804: List<Adt26>, v1: (Bytes, Bytes)) -> R<bool> {
    let m4803 = p4804;
    {
        let mut m4803 = m4803;
        loop {
            let step = {
                let v0 = m4803;
                let m4796 = v0;
                match m4796.uncons() {
                    None => {
                        Err::<List<Adt26>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if if f32(match v2.clone() {
                            Adt26::C0(h4797, _, _, _) => {
                                h4797
                            }
                        }, f8(Str::lit("element"))) {
                            let m4799 = f308(match v2.clone() {
                                Adt26::C0(_, _, _, h4798) => {
                                    h4798
                                }
                            }, Str::lit("substitutionGroup"));
                            match m4799 {
                                None => {
                                    false
                                }
                                Some(v4) => {
                                    f666({
                                        let m4802 = match v2 {
                                            Adt26::C0(_, _, _, h4801) => {
                                                h4801
                                            }
                                        };
                                        match m4802 {
                                            Adt27::C0(_, _, _, _, h4800) => {
                                                h4800
                                            }
                                        }
                                    }, f667(v4)?, v1.clone())?
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
                Ok(next) => { m4803 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f664(v0: Fn16, p4809: u64, p4810: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m4808 = (p4809, p4810);
    {
        let mut m4808 = m4808;
        loop {
            let step = {
                let (v1, v2) = m4808;
                let m4805 = v1;
                if m4805 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4805 - 1;
                    let m4807 = {
                        let c4806: Fn16 = v0.clone();
                        c4806.apply(v2.clone())?
                    };
                    match m4807 {
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
                Ok(next) => { m4808 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f665(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f668(v0)
}

pub fn f666(v0: List<(Bytes, Bytes)>, p4818: List<Bytes>, v2: (Bytes, Bytes)) -> R<bool> {
    let m4817 = p4818;
    {
        let mut m4817 = m4817;
        loop {
            let step = {
                let v1 = m4817;
                let m4811 = v1;
                match m4811.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v3, v4)) => {
                        let m4812 = f133(v0.clone(), v3)?;
                        match m4812 {
                            Ok(v6) => {
                                if if f32({
                                    let (h4813, _) = v6;
                                    h4813
                                }, {
                                    let (h4814, _) = v2.clone();
                                    h4814
                                }) {
                                    f32({
                                        let (_, h4815) = v6;
                                        h4815
                                    }, {
                                        let (_, h4816) = v2.clone();
                                        h4816
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
                Ok(next) => { m4817 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f667(v0: Bytes) -> R<List<Bytes>> {
    f43(v0, 32u8)
}

pub fn f668(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h4819, _) = v0;
        h4819
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h4820) = v0;
        h4820
    };
    let m4821 = v1;
    match m4821.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m4822 = v4;
            match m4822.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f669(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f669(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a4827 = {
            let a4825 = {
                let a4823 = v0.clone();
                length_list(a4823)
            };
            let a4826 = {
                let a4824 = v1.clone();
                length_list(a4824)
            };
            nat_add(a4825, a4826)?
        };
        let a4828 = 1u64;
        nat_add(a4827, a4828)?
    };
    Ok(f670({
        let (_, h4829) = {
            let (_, h4830) = {
                let (h4831, _) = f671(Fn17::F672, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h4831
            };
            h4830
        };
        h4829
    }))
}

pub fn f670(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f434(Fn8::F673, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f671(v0: Fn17, p4836: u64, p4837: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m4835 = (p4836, p4837);
    {
        let mut m4835 = m4835;
        loop {
            let step = {
                let (v1, v2) = m4835;
                let m4832 = v1;
                if m4832 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m4832 - 1;
                    let m4834 = {
                        let c4833: Fn17 = v0.clone();
                        c4833.apply(v2.clone())
                    };
                    match m4834 {
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
                Ok(next) => { m4835 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f672(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f674(v0)
}

pub fn f673(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f674(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h4838, _) = v0.clone();
        h4838
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h4839, _) = {
            let (_, h4840) = v0.clone();
            h4840
        };
        h4839
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h4841) = {
            let (_, h4842) = v0;
            h4842
        };
        h4841
    };
    let m4843 = v1.clone();
    match m4843.uncons() {
        None => {
            let m4844 = v2;
            match m4844.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m4845 = v2.clone();
            match m4845.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f675({
                        let (h4846, _) = v8.clone();
                        h4846
                    }, {
                        let (h4847, _) = v6.clone();
                        h4847
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

pub fn f675(v0: Bytes, v1: Bytes) -> bool {
    let a4852 = {
        let a4848 = v0;
        let a4849 = v1;
        compare_bytes(a4848, a4849)
    };
    let a4853 = {
        let a4850 = Bytes::lit(&[]);
        let a4851 = Bytes::lit(&[0x01]);
        compare_bytes(a4850, a4851)
    };
    equal(a4852, a4853)
}

pub fn run(p0: Option<Adt0>, p1: Adt3) -> R<(Adt0, Adt10)> {
    f0(p0, p1)
}
