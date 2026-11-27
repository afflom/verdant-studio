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
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(u64),
    C1(Bytes),
    C2(Bytes),
}

#[derive(Clone)]
pub enum Adt19 {
    C0(Adt2, u64),
    C1(Bytes),
}

#[derive(Clone)]
pub enum Adt20 {
    C0(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt21 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt22 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, Adt21, u64),
}

#[derive(Clone)]
pub enum Adt23 {
    C0(Bytes, u64),
    C1(Bytes),
}

#[derive(Clone)]
pub enum Adt24 {
    C0(Bytes, List<Adt20>),
}

#[derive(Clone)]
pub enum Adt25 {
    C0(Adt15, List<Adt20>),
}

#[derive(Clone)]
pub enum Adt26 {
    C0(u64, u64),
    C1(Bytes),
}

#[derive(Clone)]
pub enum Adt27 {
    C0(Bytes, u64, u64),
    C1(Bytes),
}

#[derive(Clone)]
pub enum Fn0 {
    F65(u8, Bytes),
    F233(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F65(k0, k1) => f65(*k0, k1.clone(), p0),
            Fn0::F233(k0) => f233(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F68(Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn1::F68(k0) => f68(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F109,
    F235,
    F318,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn2::F109 => f109(p0, p1),
            Fn2::F235 => f235(p0, p1),
            Fn2::F318 => f318(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F163(Bytes, u64, u64),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Adt17)) -> R<(u64, Adt17)> {
        match self {
            Fn3::F163(k0, k1, k2) => f163(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F175(Bytes),
    F186(u64, Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn4::F175(k0) => f175(k0.clone(), p0),
            Fn4::F186(k0, k1) => Ok(f186(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F177(Bytes),
    F178(Bytes),
    F208(Bytes),
    F209(Bytes),
    F237(Bytes, u64, Bytes, u64),
    F296(Bytes),
    F329(Bytes),
    F331(Bytes),
    F343(Bytes, u64),
    F384(Bytes),
    F414(Bytes, Bytes),
    F417(Bytes),
    F419(Bytes),
    F445(Bytes, bool),
    F450(u64, Bytes),
    F554(u64, Bytes, u64),
    F563(Bytes),
    F564(Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn5::F177(k0) => f177(k0.clone(), p0),
            Fn5::F178(k0) => f178(k0.clone(), p0),
            Fn5::F208(k0) => f208(k0.clone(), p0),
            Fn5::F209(k0) => f209(k0.clone(), p0),
            Fn5::F237(k0, k1, k2, k3) => f237(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn5::F296(k0) => f296(k0.clone(), p0),
            Fn5::F329(k0) => f329(k0.clone(), p0),
            Fn5::F331(k0) => f331(k0.clone(), p0),
            Fn5::F343(k0, k1) => f343(k0.clone(), *k1, p0),
            Fn5::F384(k0) => f384(k0.clone(), p0),
            Fn5::F414(k0, k1) => f414(k0.clone(), k1.clone(), p0),
            Fn5::F417(k0) => f417(k0.clone(), p0),
            Fn5::F419(k0) => f419(k0.clone(), p0),
            Fn5::F445(k0, k1) => f445(k0.clone(), *k1, p0),
            Fn5::F450(k0, k1) => f450(*k0, k1.clone(), p0),
            Fn5::F554(k0, k1, k2) => f554(*k0, k1.clone(), *k2, p0),
            Fn5::F563(k0) => f563(k0.clone(), p0),
            Fn5::F564(k0) => f564(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F212(Bytes),
    F425(Bytes),
    F441(bool, Bytes),
    F512(Bytes),
    F527(Bytes),
    F553(u64, Bytes),
    F556(u64, Bytes),
}

impl Fn6 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn6::F212(k0) => f212(k0.clone(), p0),
            Fn6::F425(k0) => f425(k0.clone(), p0),
            Fn6::F441(k0, k1) => f441(*k0, k1.clone(), p0),
            Fn6::F512(k0) => f512(k0.clone(), p0),
            Fn6::F527(k0) => f527(k0.clone(), p0),
            Fn6::F553(k0, k1) => f553(*k0, k1.clone(), p0),
            Fn6::F556(k0, k1) => f556(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F298,
}

impl Fn7 {
    pub fn apply(&self, p0: Adt22) -> R<Adt22> {
        match self {
            Fn7::F298 => f298(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F312,
    F600,
}

impl Fn8 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn8::F312 => f312(p0, p1),
            Fn8::F600 => f600(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F315,
}

impl Fn9 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn9::F315 => f315(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F405,
}

impl Fn10 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn10::F405 => f405(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F408,
}

impl Fn11 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn11::F408 => f408(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F410,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn12::F410 => f410(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F413,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn13::F413 => f413(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F478(Bytes),
}

impl Fn14 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn14::F478(k0) => f478(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F487,
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn15::F487 => f487(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F496,
}

impl Fn16 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn16::F496 => f496(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F498,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn17::F498 => f498(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F500,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn18::F500 => f500(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F503,
}

impl Fn19 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn19::F503 => f503(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F524(u64, Bytes),
}

impl Fn20 {
    pub fn apply(&self, p0: (u64, bool)) -> R<Option<(u64, bool)>> {
        match self {
            Fn20::F524(k0, k1) => f524(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F550,
}

impl Fn21 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn21::F550 => f550(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F579,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn22::F579 => f579(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F589,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn23::F589 => f589(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F595,
}

impl Fn24 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn24::F595 => f595(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F598,
}

impl Fn25 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn25::F598 => f598(p0),
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
            if f32(v0.clone(), f8(Str::lit("read-yaml"))) {
                f58(v3, v1)
            } else {
                if f59(v0) {
                    Ok(Adt12::C4(f8(Str::lit("read-xml, read-xsd and read-xsd-components run in verdant_xml, the command of the XML readers project (readers/xml)\u{a}"))))
                } else {
                    Ok(Adt12::C4(f31()))
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
    let v1: u64 = f60(v0)?;
    let m199 = {
        let a197 = f61({
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
    let v1: Bytes = f62(v0)?;
    let m229 = f63(v1.clone(), 0u64, {
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
        let (h236, _) = f64(Fn0::F65(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h236
    };
    let v4: u64 = {
        let (h237, _) = {
            let (_, h238) = v3.clone();
            h238
        };
        h237
    };
    Ok(f66(List::cons(f46(v0, v4, {
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
    let m251 = f67(Fn1::F68(v1), None::<Bytes>, v0);
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
    List::cons(f8(Str::lit("admit JSONL")), List::cons(f8(Str::lit("admit-encoding STAGED")), List::cons(f8(Str::lit("admit-registry AUTHORITY ROW CONSTRUCT|none read-OP|check-yaml ARTIFACT")), List::cons(f8(Str::lit("admit-translated FILE NAME read-OP ARG... CITATION...")), List::cons(f8(Str::lit("agenda")), List::cons(f8(Str::lit("build")), List::cons(f8(Str::lit("check")), List::cons(f8(Str::lit("check-yaml FILE")), List::cons(f8(Str::lit("ingest-authority BINDING")), List::cons(f8(Str::lit("ingest-index CSV")), List::cons(f8(Str::lit("merge-gate RULES")), List::cons(f8(Str::lit("next [SKIP...]")), List::cons(f8(Str::lit("propose-encoding SCHEMA NAME CITATION...")), List::cons(f8(Str::lit("publish")), List::cons(f8(Str::lit("read-admit NAME read-OP FILE ARG... CITATION...")), List::cons(f8(Str::lit("read-webidl FILE [IMPORT...]")), List::cons(f8(Str::lit("read-xml FILE")), List::cons(f8(Str::lit("read-xsd FILE")), List::cons(f8(Str::lit("read-xsd-components FILE")), List::cons(f8(Str::lit("read-yaml FILE")), List::cons(f8(Str::lit("record-decision JSONL")), List::cons(f8(Str::lit("record-error REFUSAL TIME RUN OPERATION...")), List::cons(f8(Str::lit("stage-admissions JSONL")), List::cons(f8(Str::lit("stage-bindings JSONL")), List::cons(f8(Str::lit("stage-decisions JSONL")), List::cons(f8(Str::lit("stage-encodings JSONL")), List::cons(f8(Str::lit("stage-engine")), List::cons(f8(Str::lit("stage-mappings JSONL")), List::cons(f8(Str::lit("stage-matrix JSONL")), List::cons(f8(Str::lit("stage-registries JSONL")), List::cons(f8(Str::lit("verify-translations ADMISSIONS")), List::<Bytes>::nil())))))))))))))))))))))))))))))))
}

pub fn f49(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m252 = v1;
    match m252.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m253 = f69(f70(v0.clone(), v2.clone()), v2.clone());
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
                        let m255 = f71(v2.clone(), f8(Str::lit("translation")));
                        match m255 {
                            None => {
                                Ok::<List<Adt2>, R<Adt12>>(v3)
                            }
                            Some(v4) => {
                                let v6: Bytes = {
                                    let m256 = f72(f71(v2, f8(Str::lit("construct"))));
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
                                    let m257 = f72(f71(v4.clone(), f8(Str::lit("output_sha256"))));
                                    match m257 {
                                        None => {
                                            Bytes::lit(&[])
                                        }
                                        Some(v7) => {
                                            v7
                                        }
                                    }
                                };
                                let m258 = f73(f74(f71(v4, f8(Str::lit("command")))));
                                match m258.uncons() {
                                    None => {
                                        Err::<List<Adt2>, R<Adt12>>(Ok(Adt12::C4(f7(List::cons(v6, List::cons(f8(Str::lit(" records no reader command\u{a}")), List::<Bytes>::nil()))))))
                                    }
                                    Some((v9, v10)) => {
                                        if f59(v9.clone()) {
                                            Ok::<List<Adt2>, R<Adt12>>(v3)
                                        } else {
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
                                                            if f32(f75(v20.clone())?, v8.clone()) {
                                                                Ok::<List<Adt2>, R<Adt12>>(v3)
                                                            } else {
                                                                Err::<List<Adt2>, R<Adt12>>(Ok(Adt12::C4(f7(List::cons(v6, List::cons(f8(Str::lit(": its reader command now prints output with sha256 ")), List::cons(f75(v20)?, List::cons(f8(Str::lit(", and the admission records ")), List::cons(v8, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))))
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
    Ok(f74(f76(v0, v1)?))
}

pub fn f52(v0: List<Bytes>) -> List<Bytes> {
    let m263 = v0;
    match m263.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f77(v1.clone()) {
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
    let m271 = f78(v4);
    match m271 {
        None => {
            Ok(Adt12::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
        }
        Some(v5) => {
            let m272 = f79(f80(List::cons(Adt2::C8(std::rc::Rc::new(v5.clone())), List::<Adt2>::nil())), f81(f51(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m272 {
                None => {
                    f83(v0, v2.clone(), Adt2::C11(f8(Str::lit("translation")), std::rc::Rc::new(Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("command")), std::rc::Rc::new(Adt2::C7(std::rc::Rc::new(f84(v3)))), std::rc::Rc::new(Adt2::C11(f8(Str::lit("output_sha256")), std::rc::Rc::new(Adt2::C6(f75(v1.clone())?)), std::rc::Rc::new(Adt2::C9))))))), std::rc::Rc::new(v5)), f85(v2, f86(v1)?)?, f8(Str::lit("Encoding entered through admit-translated from the verdant_readers command recorded in its admission.")), List::<(Bytes, Bytes)>::nil())
                }
                Some(v6) => {
                    Ok(f82(v6))
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
                        if f77(v1.clone()) {
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
            let m277 = f87(f88(v0, v1))?;
            match m277 {
                Ok(v5) => {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f89(f90(v5)?)?, List::cons(f8(Str::lit(" documents\u{a}")), List::<Bytes>::nil())))))
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
            let m279 = f91(f88(v0.clone(), v1), f92(v0, v2))?;
            match m279 {
                Ok(v6) => {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("-- ")), List::cons(f89({
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
            let m283 = f87(f88(v0, v1))?;
            match m283 {
                Ok(v5) => {
                    let m284 = f93(v5)?;
                    match m284 {
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

pub fn f59(v0: Bytes) -> bool {
    if f32(v0.clone(), f8(Str::lit("read-xml"))) {
        true
    } else {
        if f32(v0.clone(), f8(Str::lit("read-xsd"))) {
            true
        } else {
            f32(v0, f8(Str::lit("read-xsd-components")))
        }
    }
}

pub fn f60(v0: u8) -> R<u64> {
    let v1: u8 = v0;
    let a329 = {
        let a305 = {
            let a293 = {
                let a287 = {
                    let a285 = v1;
                    let a286 = 128u8;
                    fixed_u8::bit_and(a285, a286)
                };
                let a288 = 128u8;
                if equal(a287, a288) {
                    128u64
                } else {
                    0u64
                }
            };
            let a294 = {
                let a291 = {
                    let a289 = v1;
                    let a290 = 64u8;
                    fixed_u8::bit_and(a289, a290)
                };
                let a292 = 64u8;
                if equal(a291, a292) {
                    64u64
                } else {
                    0u64
                }
            };
            nat_add(a293, a294)?
        };
        let a306 = {
            let a303 = {
                let a297 = {
                    let a295 = v1;
                    let a296 = 32u8;
                    fixed_u8::bit_and(a295, a296)
                };
                let a298 = 32u8;
                if equal(a297, a298) {
                    32u64
                } else {
                    0u64
                }
            };
            let a304 = {
                let a301 = {
                    let a299 = v1;
                    let a300 = 16u8;
                    fixed_u8::bit_and(a299, a300)
                };
                let a302 = 16u8;
                if equal(a301, a302) {
                    16u64
                } else {
                    0u64
                }
            };
            nat_add(a303, a304)?
        };
        nat_add(a305, a306)?
    };
    let a330 = {
        let a327 = {
            let a315 = {
                let a309 = {
                    let a307 = v1;
                    let a308 = 8u8;
                    fixed_u8::bit_and(a307, a308)
                };
                let a310 = 8u8;
                if equal(a309, a310) {
                    8u64
                } else {
                    0u64
                }
            };
            let a316 = {
                let a313 = {
                    let a311 = v1;
                    let a312 = 4u8;
                    fixed_u8::bit_and(a311, a312)
                };
                let a314 = 4u8;
                if equal(a313, a314) {
                    4u64
                } else {
                    0u64
                }
            };
            nat_add(a315, a316)?
        };
        let a328 = {
            let a325 = {
                let a319 = {
                    let a317 = v1;
                    let a318 = 2u8;
                    fixed_u8::bit_and(a317, a318)
                };
                let a320 = 2u8;
                if equal(a319, a320) {
                    2u64
                } else {
                    0u64
                }
            };
            let a326 = {
                let a323 = {
                    let a321 = v1;
                    let a322 = 1u8;
                    fixed_u8::bit_and(a321, a322)
                };
                let a324 = 1u8;
                if equal(a323, a324) {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a325, a326)?
        };
        nat_add(a327, a328)?
    };
    nat_add(a329, a330)
}

pub fn f61(v0: u64) -> List<Str> {
    let a331 = v0;
    let a332 = 0u64;
    if nat_eq(a331, a332) {
        f94()
    } else {
        let a333 = v0;
        let a334 = 1u64;
        if nat_eq(a333, a334) {
            f95()
        } else {
            let a335 = v0;
            let a336 = 2u64;
            if nat_eq(a335, a336) {
                f96()
            } else {
                let a337 = v0;
                let a338 = 3u64;
                if nat_eq(a337, a338) {
                    f97()
                } else {
                    let a339 = v0;
                    let a340 = 4u64;
                    if nat_eq(a339, a340) {
                        f98()
                    } else {
                        let a341 = v0;
                        let a342 = 5u64;
                        if nat_eq(a341, a342) {
                            f99()
                        } else {
                            let a343 = v0;
                            let a344 = 6u64;
                            if nat_eq(a343, a344) {
                                f100()
                            } else {
                                let a345 = v0;
                                let a346 = 7u64;
                                if nat_eq(a345, a346) {
                                    f101()
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

pub fn f62(v0: Bytes) -> R<Bytes> {
    f102(v0.clone(), 0u64, {
        let a347 = v0;
        length_bytes(a347)
    }, false, false, 0u64, Bytes::lit(&[]))
}

pub fn f63(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt2, u64)>> {
    let m348 = v2;
    if m348 == 0 {
        Ok(None::<(Adt2, u64)>)
    } else {
        let v4 = m348 - 1;
        let v5: u64 = f103(v0.clone(), v1)?;
        let a349 = v3;
        let a350 = 1u64;
        if nat_eq(a349, a350) {
            let m351 = f63(v0.clone(), v1, v4, 0u64)?;
            match m351 {
                None => {
                    Ok(None::<(Adt2, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f103(v0.clone(), {
                        let (_, h352) = v6.clone();
                        h352
                    })?;
                    let a353 = v7;
                    let a354 = 44u64;
                    if nat_eq(a353, a354) {
                        let m358 = f63(v0, {
                            let a356 = {
                                let (_, h355) = v6;
                                h355
                            };
                            let a357 = 1u64;
                            nat_add(a356, a357)?
                        }, v4, 1u64)?;
                        match m358 {
                            None => {
                                Ok(None::<(Adt2, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt2::C10(std::rc::Rc::new({
                                    let (h359, _) = v6;
                                    h359
                                }), std::rc::Rc::new({
                                    let (h360, _) = v8;
                                    h360
                                })), {
                                    let (_, h361) = v8;
                                    h361
                                })))
                            }
                        }
                    } else {
                        let a362 = v7;
                        let a363 = 93u64;
                        if nat_eq(a362, a363) {
                            Ok(Some((Adt2::C10(std::rc::Rc::new({
                                let (h364, _) = v6;
                                h364
                            }), std::rc::Rc::new(Adt2::C9)), {
                                let a366 = {
                                    let (_, h365) = v6;
                                    h365
                                };
                                let a367 = 1u64;
                                nat_add(a366, a367)?
                            })))
                        } else {
                            Ok(None::<(Adt2, u64)>)
                        }
                    }
                }
            }
        } else {
            let a368 = v3;
            let a369 = 2u64;
            if nat_eq(a368, a369) {
                let m372 = f104(v0.clone(), {
                    let a370 = v1;
                    let a371 = 1u64;
                    nat_add(a370, a371)?
                })?;
                match m372 {
                    None => {
                        Ok(None::<(Adt2, u64)>)
                    }
                    Some(v9) => {
                        if f105({
                            let a373 = v5;
                            let a374 = 34u64;
                            nat_eq(a373, a374)
                        }, {
                            let a376 = f103(v0.clone(), {
                                let (h375, _) = v9.clone();
                                h375
                            })?;
                            let a377 = 58u64;
                            nat_eq(a376, a377)
                        }) {
                            let m381 = f63(v0.clone(), {
                                let a379 = {
                                    let (h378, _) = v9;
                                    h378
                                };
                                let a380 = 1u64;
                                nat_add(a379, a380)?
                            }, v4, 0u64)?;
                            match m381 {
                                None => {
                                    Ok(None::<(Adt2, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f103(v0.clone(), {
                                        let (_, h382) = v10.clone();
                                        h382
                                    })?;
                                    let a383 = v11;
                                    let a384 = 44u64;
                                    if nat_eq(a383, a384) {
                                        let m388 = f63(v0, {
                                            let a386 = {
                                                let (_, h385) = v10;
                                                h385
                                            };
                                            let a387 = 1u64;
                                            nat_add(a386, a387)?
                                        }, v4, 2u64)?;
                                        match m388 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt2::C11({
                                                    let (_, h389) = v9;
                                                    h389
                                                }, std::rc::Rc::new({
                                                    let (h390, _) = v10;
                                                    h390
                                                }), std::rc::Rc::new({
                                                    let (h391, _) = v12;
                                                    h391
                                                })), {
                                                    let (_, h392) = v12;
                                                    h392
                                                })))
                                            }
                                        }
                                    } else {
                                        let a393 = v11;
                                        let a394 = 125u64;
                                        if nat_eq(a393, a394) {
                                            Ok(Some((Adt2::C11({
                                                let (_, h395) = v9;
                                                h395
                                            }, std::rc::Rc::new({
                                                let (h396, _) = v10;
                                                h396
                                            }), std::rc::Rc::new(Adt2::C9)), {
                                                let a398 = {
                                                    let (_, h397) = v10;
                                                    h397
                                                };
                                                let a399 = 1u64;
                                                nat_add(a398, a399)?
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
                let a400 = v5;
                let a401 = 34u64;
                if nat_eq(a400, a401) {
                    let m404 = f104(v0, {
                        let a402 = v1;
                        let a403 = 1u64;
                        nat_add(a402, a403)?
                    })?;
                    match m404 {
                        None => {
                            Ok(None::<(Adt2, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt2::C6({
                                let (_, h405) = v13;
                                h405
                            }), {
                                let (h406, _) = v13;
                                h406
                            })))
                        }
                    }
                } else {
                    if f32(f46(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                        Ok(Some((Adt2::C0, {
                            let a407 = v1;
                            let a408 = 4u64;
                            nat_add(a407, a408)?
                        })))
                    } else {
                        if f32(f46(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                            Ok(Some((Adt2::C2(true), {
                                let a409 = v1;
                                let a410 = 4u64;
                                nat_add(a409, a410)?
                            })))
                        } else {
                            if f32(f46(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                                Ok(Some((Adt2::C2(false), {
                                    let a411 = v1;
                                    let a412 = 5u64;
                                    nat_add(a411, a412)?
                                })))
                            } else {
                                let a413 = v5;
                                let a414 = 91u64;
                                if nat_eq(a413, a414) {
                                    let a417 = f103(v0.clone(), {
                                        let a415 = v1;
                                        let a416 = 1u64;
                                        nat_add(a415, a416)?
                                    })?;
                                    let a418 = 93u64;
                                    if nat_eq(a417, a418) {
                                        Ok(Some((Adt2::C7(std::rc::Rc::new(Adt2::C9)), {
                                            let a419 = v1;
                                            let a420 = 2u64;
                                            nat_add(a419, a420)?
                                        })))
                                    } else {
                                        let m423 = f63(v0, {
                                            let a421 = v1;
                                            let a422 = 1u64;
                                            nat_add(a421, a422)?
                                        }, v4, 1u64)?;
                                        match m423 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v14) => {
                                                Ok(Some((Adt2::C7(std::rc::Rc::new({
                                                    let (h424, _) = v14;
                                                    h424
                                                })), {
                                                    let (_, h425) = v14;
                                                    h425
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let a426 = v5;
                                    let a427 = 123u64;
                                    if nat_eq(a426, a427) {
                                        let a430 = f103(v0.clone(), {
                                            let a428 = v1;
                                            let a429 = 1u64;
                                            nat_add(a428, a429)?
                                        })?;
                                        let a431 = 125u64;
                                        if nat_eq(a430, a431) {
                                            Ok(Some((Adt2::C8(std::rc::Rc::new(Adt2::C9)), {
                                                let a432 = v1;
                                                let a433 = 2u64;
                                                nat_add(a432, a433)?
                                            })))
                                        } else {
                                            let m436 = f63(v0, {
                                                let a434 = v1;
                                                let a435 = 1u64;
                                                nat_add(a434, a435)?
                                            }, v4, 2u64)?;
                                            match m436 {
                                                None => {
                                                    Ok(None::<(Adt2, u64)>)
                                                }
                                                Some(v15) => {
                                                    Ok(Some((Adt2::C8(std::rc::Rc::new({
                                                        let (h437, _) = v15;
                                                        h437
                                                    })), {
                                                        let (_, h438) = v15;
                                                        h438
                                                    })))
                                                }
                                            }
                                        }
                                    } else {
                                        let m439 = f106(v0, v1)?;
                                        match m439 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v16) => {
                                                Ok(Some(({
                                                    let (_, h440) = v16;
                                                    h440
                                                }, {
                                                    let (h441, _) = v16;
                                                    h441
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

pub fn f64(v0: Fn0, p446: u64, p447: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m445 = (p446, p447);
    {
        let mut m445 = m445;
        loop {
            let step = {
                let (v1, v2) = m445;
                let m442 = v1;
                if m442 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m442 - 1;
                    let m444 = {
                        let c443: Fn0 = v0.clone();
                        c443.apply(v2.clone())?
                    };
                    match m444 {
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
                Ok(next) => { m445 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f65(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f107(v1, v0, v2)
}

pub fn f66(v0: List<Bytes>) -> List<Bytes> {
    f108(Fn2::F109, List::<Bytes>::nil(), v0)
}

pub fn f67(v0: Fn1, p451: Option<Bytes>, p452: List<Bytes>) -> Option<Bytes> {
    let m450 = (p451, p452);
    {
        let mut m450 = m450;
        loop {
            let step = {
                let (v1, v2) = m450;
                let m448 = v2;
                match m448.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c449: Fn1 = v0.clone();
                            c449.apply(v1, v3)
                        }, v4))
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

pub fn f68(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m453 = v1;
    match m453 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a456 = {
                    let a454 = v3;
                    let a455 = v0;
                    append_bytes(a454, a455)
                };
                let a457 = v2;
                append_bytes(a456, a457)
            })
        }
    }
}

pub fn f69(p462: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m461 = p462;
    {
        let mut m461 = m461;
        loop {
            let step = {
                let v0 = m461;
                let m458 = v0;
                match m458.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h459, _) = v2;
                            h459
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h460) = v2;
                                h460
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m461 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f70(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Bytes>)> {
    let m466 = {
        let a464 = match v0 {
            Adt1::C0(h463, _, _, _) => {
                h463
            }
        };
        let a465 = f40(v1);
        index_list(a464, a465)
    };
    match m466 {
        None => {
            List::<(Bytes, Option<Bytes>)>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f71(p472: Adt2, v1: Bytes) -> Option<Adt2> {
    let m471 = p472;
    {
        let mut m471 = m471;
        loop {
            let step = {
                let v0 = m471;
                let m467 = v0;
                match m467 {
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
                    Adt2::C8(r468) => {
                        let v5 = (*r468).clone();
                        Ok::<Adt2, Option<Adt2>>(v5)
                    }
                    Adt2::C9 => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C10(_, _) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C11(v2, r469, r470) => {
                        let v3 = (*r469).clone();
                        let v4 = (*r470).clone();
                        if f32(v2, v1.clone()) {
                            Err::<Adt2, Option<Adt2>>(Some(v3))
                        } else {
                            Ok::<Adt2, Option<Adt2>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m471 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f72(v0: Option<Adt2>) -> Option<Bytes> {
    let m473 = v0;
    match m473 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m474 = v1;
            match m474 {
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

pub fn f73(v0: List<Adt2>) -> List<Bytes> {
    let m475 = v0;
    match m475.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m476 = f72(Some(v1));
            match m476 {
                None => {
                    f73(v2)
                }
                Some(v3) => {
                    List::cons(v3, f73(v2))
                }
            }
        }
    }
}

pub fn f74(v0: Option<Adt2>) -> List<Adt2> {
    let m477 = v0;
    match m477 {
        None => {
            List::<Adt2>::nil()
        }
        Some(v1) => {
            f110(v1)
        }
    }
}

pub fn f75(v0: Bytes) -> R<Bytes> {
    Ok(f111(f112(v0)?))
}

pub fn f76(v0: Adt1, v1: Bytes) -> R<Option<Adt2>> {
    f113(v0, v1)
}

pub fn f77(v0: Bytes) -> bool {
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

pub fn f78(v0: List<Bytes>) -> Option<Adt2> {
    let m478 = v0;
    match m478.uncons() {
        None => {
            None::<Adt2>
        }
        Some((v1, v2)) => {
            if f32(v1.clone(), f8(Str::lit("req"))) {
                let m479 = v2;
                match m479.uncons() {
                    None => {
                        None::<Adt2>
                    }
                    Some((v3, _)) => {
                        Some(Adt2::C11(f8(Str::lit("req")), std::rc::Rc::new(Adt2::C6(v3)), std::rc::Rc::new(Adt2::C9)))
                    }
                }
            } else {
                if f32(v1.clone(), f8(Str::lit("std"))) {
                    let m480 = v2;
                    match m480.uncons() {
                        None => {
                            None::<Adt2>
                        }
                        Some((v5, v6)) => {
                            Some(Adt2::C11(f8(Str::lit("std")), std::rc::Rc::new(Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("authority")), std::rc::Rc::new(Adt2::C6(v5)), std::rc::Rc::new({
                                let m481 = v6;
                                match m481.uncons() {
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
                    if f32(v1, f8(Str::lit("der"))) {
                        let m482 = v2;
                        match m482.uncons() {
                            None => {
                                None::<Adt2>
                            }
                            Some((v9, v10)) => {
                                let m483 = v10;
                                match m483.uncons() {
                                    None => {
                                        None::<Adt2>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt2::C11(f8(Str::lit("der")), std::rc::Rc::new(Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("check")), std::rc::Rc::new(Adt2::C6(v11)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("inputs")), std::rc::Rc::new(Adt2::C7(std::rc::Rc::new(f84(v12)))), std::rc::Rc::new(Adt2::C11(f8(Str::lit("rule")), std::rc::Rc::new(Adt2::C6(v9)), std::rc::Rc::new(Adt2::C9))))))))), std::rc::Rc::new(Adt2::C9)))
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

pub fn f79(p486: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m485 = p486;
    {
        let mut m485 = m485;
        loop {
            let step = {
                let v0 = m485;
                let m484 = v0;
                match m484.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f114(v2.clone(), v1.clone()) {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v2))
                        } else {
                            Ok::<List<Bytes>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m485 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f80(v0: List<Adt2>) -> List<Bytes> {
    let m487 = v0;
    match m487.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m488 = f71(v1, f8(Str::lit("std")));
            match m488 {
                None => {
                    f80(v2)
                }
                Some(v3) => {
                    List::cons(f115(v3, f8(Str::lit("row"))), f80(v2))
                }
            }
        }
    }
}

pub fn f81(v0: List<Adt2>) -> List<Bytes> {
    let m489 = v0;
    match m489.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f116(f73(f74(f71(v1, f8(Str::lit("defers"))))), f81(v2))
        }
    }
}

pub fn f82(v0: Bytes) -> Adt12 {
    Adt12::C4(f7(List::cons(f8(Str::lit("index row ")), List::cons(v0, List::cons(f8(Str::lit(" is deferred by an owner decision, so nothing that cites it is admitted\u{a}")), List::<Bytes>::nil())))))
}

pub fn f83(v0: Adt1, v1: Bytes, v2: Adt2, v3: Bytes, v4: Bytes, v5: List<(Bytes, Bytes)>) -> R<Adt12> {
    let v6: Bytes = {
        let a490 = f8(Str::lit("Catalog."));
        let a491 = f117(v1.clone())?;
        append_bytes(a490, a491)
    };
    let v7: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f117(v1)?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
    Ok(Adt12::C3(f118(v5, List::cons((v7.clone(), v3), List::cons((f8(Str::lit("model/admissions.json")), f119(Adt2::C7(std::rc::Rc::new(f120(f121(f51(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v6.clone(), Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("construct")), std::rc::Rc::new(Adt2::C6(v6.clone())), std::rc::Rc::new(v2)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f119(Adt2::C7(std::rc::Rc::new(f120(f121(f51(v0.clone(), f8(Str::lit("model/strata.json")))?, f8(Str::lit("path")), v7.clone(), Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("path")), std::rc::Rc::new(Adt2::C6(v7.clone())), std::rc::Rc::new(Adt2::C11(f8(Str::lit("role")), std::rc::Rc::new(Adt2::C6(v4)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("stratum")), std::rc::Rc::new(Adt2::C3(false, 0u64)), std::rc::Rc::new(Adt2::C9)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
        let a495 = f122(f88(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f46(v7.clone(), 4u64, {
            let a493 = {
                let a492 = v7;
                length_bytes(a492)
            };
            let a494 = 4u64;
            nat_sub(a493, a494)
        }))?;
        let a496 = f8(Str::lit("\u{a}"));
        append_bytes(a495, a496)
    }), List::cons(f123(v0, List::cons(v6.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v6, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f84(v0: List<Bytes>) -> Adt2 {
    let m497 = v0;
    match m497.uncons() {
        None => {
            Adt2::C9
        }
        Some((v1, v2)) => {
            Adt2::C10(std::rc::Rc::new(Adt2::C6(v1)), std::rc::Rc::new(f84(v2)))
        }
    }
}

pub fn f85(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\\begin{lexlean}{Catalog.")), List::cons(f117(v0)?, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(v1, List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil())))))))
}

pub fn f86(v0: Bytes) -> R<Bytes> {
    f124({
        let a499 = {
            let a498 = v0.clone();
            length_bytes(a498)
        };
        let a500 = 1u64;
        nat_add(a499, a500)?
    }, v0)
}

pub fn f87(v0: Bytes) -> R<Result<List<Adt2>, Bytes>> {
    let v1: Bytes = if f125(v0.clone(), Bytes::lit(&[0xef, 0xbb, 0xbf]), 0u64) {
        f46(v0.clone(), 3u64, {
            let a502 = {
                let a501 = v0;
                length_bytes(a501)
            };
            let a503 = 3u64;
            nat_sub(a502, a503)
        })
    } else {
        v0
    };
    let v2: Bytes = f126(v1)?;
    let v3: u64 = {
        let a504 = v2.clone();
        length_bytes(a504)
    };
    let v4: u64 = f127(v2.clone())?;
    let v5: u64 = f128(v2.clone(), 0u64)?;
    let a505 = v4;
    let a506 = v3;
    if nat_lt(a505, a506) {
        Ok(Err::<List<Adt2>, Bytes>(f129(Str::lit("the input is not a sequence of printable characters in UTF-8 (YAML 1.2.2 [1], section 5.2)"), v4)?))
    } else {
        let m509 = f130(v2.clone(), v5, {
            let a507 = v3;
            let a508 = 1u64;
            nat_add(a507, a508)?
        })?;
        match m509 {
            Ok(v7) => {
                let a510 = v3;
                let a511 = v7;
                if nat_le(a510, a511) {
                    Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
                } else {
                    f131(v2, v7, true, {
                        let a512 = v3;
                        let a513 = 1u64;
                        nat_add(a512, a513)?
                    })
                }
            }
            Err(v6) => {
                Ok(Err::<List<Adt2>, Bytes>(v6))
            }
        }
    }
}

pub fn f88(v0: Adt1, v1: Bytes) -> Bytes {
    let m514 = f69(f70(v0, v1.clone()), v1);
    match m514 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m515 = v2;
            match m515 {
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

pub fn f89(v0: u64) -> R<Bytes> {
    f132(f133(), 10u64, v0)
}

pub fn f90(v0: List<Adt2>) -> R<u64> {
    let m516 = v0;
    match m516.uncons() {
        None => {
            Ok(0u64)
        }
        Some((_, v2)) => {
            let a517 = f90(v2)?;
            let a518 = 1u64;
            nat_add(a517, a518)
        }
    }
}

pub fn f91(v0: Bytes, v1: List<Bytes>) -> R<Result<(u64, Bytes), Bytes>> {
    let m519 = f134(v0)?;
    match m519 {
        Ok(v3) => {
            let m520 = f135(v1)?;
            match m520 {
                Ok(v5) => {
                    let v6: List<Adt13> = f136(v3.clone(), v5);
                    let v7: List<Adt16> = f137({
                        let a522 = {
                            let a521 = v6.clone();
                            length_list(a521)
                        };
                        let a523 = 2u64;
                        nat_add(a522, a523)?
                    }, List::cons(Adt16::C0(f138(), Bytes::lit(&[]), f139(false), List::<Bytes>::nil()), List::cons(Adt16::C0(f140(), Bytes::lit(&[]), f139(true), List::<Bytes>::nil()), f141(v6.clone(), v6.clone())?)))?;
                    let v8: List<Bytes> = f142({
                        let a525 = {
                            let a524 = v7.clone();
                            length_list(a524)
                        };
                        let a526 = 1u64;
                        nat_add(a525, a526)?
                    }, f143(v3.clone()), List::<Bytes>::nil(), v7.clone());
                    let v9: List<(Bytes, (List<Bytes>, Bytes))> = f144(v7.clone(), v8);
                    let v10: (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>) = f145({
                        let a528 = {
                            let a527 = v9.clone();
                            length_list(a527)
                        };
                        let a529 = 1u64;
                        nat_add(a528, a529)?
                    }, v9, List::<Bytes>::nil(), List::<Bytes>::nil());
                    Ok(Ok::<(u64, Bytes), Bytes>(({
                        let a531 = {
                            let (h530, _) = v10.clone();
                            h530
                        };
                        length_list(a531)
                    }, f7(List::cons(f7(f146(v3, v7, v6)), List::cons(f7(f147({
                        let (_, h532) = v10;
                        h532
                    })), List::cons(f8(Str::lit("-- operations, constants and the other members that are not attributes are behavior, not data, and are not translated (Web IDL 2.5)\u{a}")), List::cons(f148(f149({
                        let (h533, _) = v10;
                        h533
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

pub fn f92(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m534 = v1;
    match m534.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f88(v0.clone(), v2), f92(v0, v3))
        }
    }
}

pub fn f93(v0: List<Adt2>) -> R<Result<Bytes, Bytes>> {
    let m535 = v0;
    match m535.uncons() {
        None => {
            Ok(Ok::<Bytes, Bytes>(Bytes::lit(&[])))
        }
        Some((v1, v2)) => {
            let m536 = f150(v1)?;
            match m536 {
                Ok(v4) => {
                    let m537 = f93(v2)?;
                    match m537 {
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

pub fn f94() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{9}"), List::cons(Str::lit("\u{a}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{d}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f95() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f96() -> List<Str> {
    List::cons(Str::lit(" "), List::cons(Str::lit("!"), List::cons(Str::lit("\""), List::cons(Str::lit("#"), List::cons(Str::lit("$"), List::cons(Str::lit("%"), List::cons(Str::lit("&"), List::cons(Str::lit("'"), List::cons(Str::lit("("), List::cons(Str::lit(")"), List::cons(Str::lit("*"), List::cons(Str::lit("+"), List::cons(Str::lit(","), List::cons(Str::lit("-"), List::cons(Str::lit("."), List::cons(Str::lit("/"), List::<Str>::nil()))))))))))))))))
}

pub fn f97() -> List<Str> {
    List::cons(Str::lit("0"), List::cons(Str::lit("1"), List::cons(Str::lit("2"), List::cons(Str::lit("3"), List::cons(Str::lit("4"), List::cons(Str::lit("5"), List::cons(Str::lit("6"), List::cons(Str::lit("7"), List::cons(Str::lit("8"), List::cons(Str::lit("9"), List::cons(Str::lit(":"), List::cons(Str::lit(";"), List::cons(Str::lit("<"), List::cons(Str::lit("="), List::cons(Str::lit(">"), List::cons(Str::lit("?"), List::<Str>::nil()))))))))))))))))
}

pub fn f98() -> List<Str> {
    List::cons(Str::lit("@"), List::cons(Str::lit("A"), List::cons(Str::lit("B"), List::cons(Str::lit("C"), List::cons(Str::lit("D"), List::cons(Str::lit("E"), List::cons(Str::lit("F"), List::cons(Str::lit("G"), List::cons(Str::lit("H"), List::cons(Str::lit("I"), List::cons(Str::lit("J"), List::cons(Str::lit("K"), List::cons(Str::lit("L"), List::cons(Str::lit("M"), List::cons(Str::lit("N"), List::cons(Str::lit("O"), List::<Str>::nil()))))))))))))))))
}

pub fn f99() -> List<Str> {
    List::cons(Str::lit("P"), List::cons(Str::lit("Q"), List::cons(Str::lit("R"), List::cons(Str::lit("S"), List::cons(Str::lit("T"), List::cons(Str::lit("U"), List::cons(Str::lit("V"), List::cons(Str::lit("W"), List::cons(Str::lit("X"), List::cons(Str::lit("Y"), List::cons(Str::lit("Z"), List::cons(Str::lit("["), List::cons(Str::lit("\\"), List::cons(Str::lit("]"), List::cons(Str::lit("^"), List::cons(Str::lit("_"), List::<Str>::nil()))))))))))))))))
}

pub fn f100() -> List<Str> {
    List::cons(Str::lit("`"), List::cons(Str::lit("a"), List::cons(Str::lit("b"), List::cons(Str::lit("c"), List::cons(Str::lit("d"), List::cons(Str::lit("e"), List::cons(Str::lit("f"), List::cons(Str::lit("g"), List::cons(Str::lit("h"), List::cons(Str::lit("i"), List::cons(Str::lit("j"), List::cons(Str::lit("k"), List::cons(Str::lit("l"), List::cons(Str::lit("m"), List::cons(Str::lit("n"), List::cons(Str::lit("o"), List::<Str>::nil()))))))))))))))))
}

pub fn f101() -> List<Str> {
    List::cons(Str::lit("p"), List::cons(Str::lit("q"), List::cons(Str::lit("r"), List::cons(Str::lit("s"), List::cons(Str::lit("t"), List::cons(Str::lit("u"), List::cons(Str::lit("v"), List::cons(Str::lit("w"), List::cons(Str::lit("x"), List::cons(Str::lit("y"), List::cons(Str::lit("z"), List::cons(Str::lit("{"), List::cons(Str::lit("|"), List::cons(Str::lit("}"), List::cons(Str::lit("~"), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f102(v0: Bytes, p565: u64, p566: u64, p567: bool, p568: bool, p569: u64, p570: Bytes) -> R<Bytes> {
    let m564 = (p565, (p566, (p567, (p568, (p569, p570)))));
    {
        let mut m564 = m564;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, (v5, v6))))) = m564;
                let m538 = v2;
                if m538 == 0 {
                    Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f151(v0.clone(), v5, v1, v6)))
                } else {
                    let v7 = m538 - 1;
                    let m541 = {
                        let a539 = v0.clone();
                        let a540 = v1;
                        index_bytes(a539, a540)
                    };
                    match m541 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f151(v0.clone(), v5, v1, v6)))
                        }
                        Some(v8) => {
                            let v9: u64 = f60(v8)?;
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                    let a542 = v1;
                                    let a543 = 1u64;
                                    nat_add(a542, a543)?
                                }, (v7, (if v4 {
                                    true
                                } else {
                                    let a544 = v9;
                                    let a545 = 34u64;
                                    !nat_eq(a544, a545)
                                }, (if v4 {
                                    false
                                } else {
                                    let a546 = v9;
                                    let a547 = 92u64;
                                    nat_eq(a546, a547)
                                }, (v5, v6))))))
                            } else {
                                let a548 = v9;
                                let a549 = 32u64;
                                if if if if nat_eq(a548, a549) {
                                    true
                                } else {
                                    let a550 = v9;
                                    let a551 = 9u64;
                                    nat_eq(a550, a551)
                                } {
                                    true
                                } else {
                                    let a552 = v9;
                                    let a553 = 10u64;
                                    nat_eq(a552, a553)
                                } {
                                    true
                                } else {
                                    let a554 = v9;
                                    let a555 = 13u64;
                                    nat_eq(a554, a555)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a556 = v1;
                                        let a557 = 1u64;
                                        nat_add(a556, a557)?
                                    }, (v7, (false, (false, ({
                                        let a558 = v1;
                                        let a559 = 1u64;
                                        nat_add(a558, a559)?
                                    }, f151(v0.clone(), v5, v1, v6)))))))
                                } else {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a560 = v1;
                                        let a561 = 1u64;
                                        nat_add(a560, a561)?
                                    }, (v7, ({
                                        let a562 = v9;
                                        let a563 = 34u64;
                                        nat_eq(a562, a563)
                                    }, (false, (v5, v6))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m564 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f103(v0: Bytes, v1: u64) -> R<u64> {
    let m573 = {
        let a571 = v0;
        let a572 = v1;
        index_bytes(a571, a572)
    };
    match m573 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a618 = {
                let a594 = {
                    let a582 = {
                        let a576 = {
                            let a574 = v3;
                            let a575 = 128u8;
                            fixed_u8::bit_and(a574, a575)
                        };
                        let a577 = 128u8;
                        if equal(a576, a577) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a583 = {
                        let a580 = {
                            let a578 = v3;
                            let a579 = 64u8;
                            fixed_u8::bit_and(a578, a579)
                        };
                        let a581 = 64u8;
                        if equal(a580, a581) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a582, a583)?
                };
                let a595 = {
                    let a592 = {
                        let a586 = {
                            let a584 = v3;
                            let a585 = 32u8;
                            fixed_u8::bit_and(a584, a585)
                        };
                        let a587 = 32u8;
                        if equal(a586, a587) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a593 = {
                        let a590 = {
                            let a588 = v3;
                            let a589 = 16u8;
                            fixed_u8::bit_and(a588, a589)
                        };
                        let a591 = 16u8;
                        if equal(a590, a591) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a592, a593)?
                };
                nat_add(a594, a595)?
            };
            let a619 = {
                let a616 = {
                    let a604 = {
                        let a598 = {
                            let a596 = v3;
                            let a597 = 8u8;
                            fixed_u8::bit_and(a596, a597)
                        };
                        let a599 = 8u8;
                        if equal(a598, a599) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a605 = {
                        let a602 = {
                            let a600 = v3;
                            let a601 = 4u8;
                            fixed_u8::bit_and(a600, a601)
                        };
                        let a603 = 4u8;
                        if equal(a602, a603) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a604, a605)?
                };
                let a617 = {
                    let a614 = {
                        let a608 = {
                            let a606 = v3;
                            let a607 = 2u8;
                            fixed_u8::bit_and(a606, a607)
                        };
                        let a609 = 2u8;
                        if equal(a608, a609) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a615 = {
                        let a612 = {
                            let a610 = v3;
                            let a611 = 1u8;
                            fixed_u8::bit_and(a610, a611)
                        };
                        let a613 = 1u8;
                        if equal(a612, a613) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a614, a615)?
                };
                nat_add(a616, a617)?
            };
            nat_add(a618, a619)
        }
    }
}

pub fn f104(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = f152(v0.clone(), v1, {
        let a621 = {
            let a620 = v0.clone();
            length_bytes(a620)
        };
        let a622 = 1u64;
        nat_add(a621, a622)?
    })?;
    let a623 = f103(v0.clone(), v2)?;
    let a624 = 34u64;
    if nat_eq(a623, a624) {
        Ok(Some(({
            let a625 = v2;
            let a626 = 1u64;
            nat_add(a625, a626)?
        }, f46(v0, v1, {
            let a627 = v2;
            let a628 = v1;
            nat_sub(a627, a628)
        }))))
    } else {
        f153(v0.clone(), v1, {
            let a630 = {
                let a629 = v0;
                length_bytes(a629)
            };
            let a631 = 1u64;
            nat_add(a630, a631)?
        }, Bytes::lit(&[]))
    }
}

pub fn f105(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f106(v0: Bytes, v1: u64) -> R<Option<(u64, Adt2)>> {
    let v2: bool = {
        let a632 = f103(v0.clone(), v1)?;
        let a633 = 45u64;
        nat_eq(a632, a633)
    };
    let v3: u64 = if v2 {
        let a634 = v1;
        let a635 = 1u64;
        nat_add(a634, a635)?
    } else {
        v1
    };
    let v4: u64 = f154(v0.clone(), v3)?;
    let v5: u64 = {
        let a636 = v4;
        let a637 = v3;
        nat_sub(a636, a637)
    };
    let v6: bool = {
        let a638 = f103(v0.clone(), v4)?;
        let a639 = 46u64;
        nat_eq(a638, a639)
    };
    let v7: u64 = if v6 {
        let a640 = v4;
        let a641 = 1u64;
        nat_add(a640, a641)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f154(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f103(v0.clone(), v8)?;
    let v10: bool = {
        let a642 = v9;
        let a643 = 101u64;
        if nat_eq(a642, a643) {
            true
        } else {
            let a644 = v9;
            let a645 = 69u64;
            nat_eq(a644, a645)
        }
    };
    let v11: u64 = f103(v0.clone(), {
        let a646 = v8;
        let a647 = 1u64;
        nat_add(a646, a647)?
    })?;
    let v12: bool = f105(v10, f155({
        let a648 = v11;
        let a649 = 43u64;
        nat_eq(a648, a649)
    }, {
        let a650 = v11;
        let a651 = 45u64;
        nat_eq(a650, a651)
    }));
    let v13: u64 = if v10 {
        if v12 {
            let a652 = v8;
            let a653 = 2u64;
            nat_add(a652, a653)?
        } else {
            let a654 = v8;
            let a655 = 1u64;
            nat_add(a654, a655)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f154(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f46(v0.clone(), v13, {
        let a656 = v14;
        let a657 = v13;
        nat_sub(a656, a657)
    });
    let v16: u64 = f156(v15.clone())?;
    let v17: Bytes = f46(v15.clone(), v16, {
        let a659 = {
            let a658 = v15;
            length_bytes(a658)
        };
        let a660 = v16;
        nat_sub(a659, a660)
    });
    let a661 = v5;
    let a662 = 0u64;
    if if nat_eq(a661, a662) {
        true
    } else {
        if f105({
            let a663 = 1u64;
            let a664 = v5;
            nat_lt(a663, a664)
        }, {
            let a665 = f103(v0.clone(), v3)?;
            let a666 = 48u64;
            nat_eq(a665, a666)
        }) {
            true
        } else {
            if f105(v6, {
                let a667 = v8;
                let a668 = v7;
                nat_eq(a667, a668)
            }) {
                true
            } else {
                f155(if v10 {
                    let a669 = v14;
                    let a670 = v13;
                    nat_eq(a669, a670)
                } else {
                    false
                }, {
                    let a672 = 18u64;
                    let a673 = {
                        let a671 = v17.clone();
                        length_bytes(a671)
                    };
                    nat_lt(a672, a673)
                })
            }
        }
    } {
        Ok(None::<(u64, Adt2)>)
    } else {
        let m677 = {
            let a675 = {
                let a674 = v17.clone();
                length_bytes(a674)
            };
            let a676 = 0u64;
            if nat_eq(a675, a676) {
                Some(0u64)
            } else {
                f157(v17)?
            }
        };
        match m677 {
            None => {
                Ok(None::<(u64, Adt2)>)
            }
            Some(v18) => {
                let m682 = f158(f46(v0.clone(), v3, v5), f46(v0, v7, {
                    let a678 = v8;
                    let a679 = v7;
                    nat_sub(a678, a679)
                }), f105(v12, {
                    let a680 = v11;
                    let a681 = 45u64;
                    nat_eq(a680, a681)
                }), v18)?;
                match m682 {
                    None => {
                        Ok(None::<(u64, Adt2)>)
                    }
                    Some(v19) => {
                        let a685 = 19u64;
                        let a686 = {
                            let a684 = {
                                let (h683, _) = v19.clone();
                                h683
                            };
                            length_bytes(a684)
                        };
                        if nat_lt(a685, a686) {
                            Ok(None::<(u64, Adt2)>)
                        } else {
                            let m688 = f157({
                                let (h687, _) = v19.clone();
                                h687
                            })?;
                            match m688 {
                                None => {
                                    Ok(None::<(u64, Adt2)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f105(v2, {
                                        let a689 = 0u64;
                                        let a690 = v20;
                                        nat_lt(a689, a690)
                                    });
                                    if f159(v21, v20) {
                                        Ok(Some((v14, {
                                            let a692 = {
                                                let (_, h691) = v19.clone();
                                                h691
                                            };
                                            let a693 = 0u64;
                                            if nat_eq(a692, a693) {
                                                Adt2::C3(v21, v20)
                                            } else {
                                                Adt2::C4(v21, v20, {
                                                    let (_, h694) = v19;
                                                    h694
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

pub fn f107(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h695, _) = v2.clone();
        h695
    };
    let v4: u64 = {
        let (h696, _) = {
            let (_, h697) = v2.clone();
            h697
        };
        h696
    };
    let v5: List<Bytes> = {
        let (_, h698) = {
            let (_, h699) = v2;
            h699
        };
        h698
    };
    let m700 = f160(v0.clone(), v3);
    match m700 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a701 = v6;
            let a702 = v1;
            if equal(a701, a702) {
                Ok(Some(({
                    let a703 = v3;
                    let a704 = 1u64;
                    nat_add(a703, a704)?
                }, ({
                    let a705 = v3;
                    let a706 = 1u64;
                    nat_add(a705, a706)?
                }, List::cons(f46(v0, v4, {
                    let a707 = v3;
                    let a708 = v4;
                    nat_sub(a707, a708)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a709 = v3;
                    let a710 = 1u64;
                    nat_add(a709, a710)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f108(v0: Fn2, p714: List<Bytes>, p715: List<Bytes>) -> List<Bytes> {
    let m713 = (p714, p715);
    {
        let mut m713 = m713;
        loop {
            let step = {
                let (v1, v2) = m713;
                let m711 = v2;
                match m711.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c712: Fn2 = v0.clone();
                            c712.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m713 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f109(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f110(v0: Adt2) -> List<Adt2> {
    let m716 = v0;
    match m716 {
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
        Adt2::C7(r717) => {
            let v3 = (*r717).clone();
            f110(v3)
        }
        Adt2::C8(_) => {
            List::<Adt2>::nil()
        }
        Adt2::C9 => {
            List::<Adt2>::nil()
        }
        Adt2::C10(r718, r719) => {
            let v1 = (*r718).clone();
            let v2 = (*r719).clone();
            List::cons(v1, f110(v2))
        }
        Adt2::C11(_, _, _) => {
            List::<Adt2>::nil()
        }
    }
}

pub fn f111(v0: Adt17) -> Bytes {
    let a740 = f161(match v0.clone() {
        Adt17::C0(h720, _, _, _, _, _, _, _) => {
            h720
        }
    });
    let a741 = {
        let a738 = f161(match v0.clone() {
            Adt17::C0(_, h721, _, _, _, _, _, _) => {
                h721
            }
        });
        let a739 = {
            let a736 = f161(match v0.clone() {
                Adt17::C0(_, _, h722, _, _, _, _, _) => {
                    h722
                }
            });
            let a737 = {
                let a734 = f161(match v0.clone() {
                    Adt17::C0(_, _, _, h723, _, _, _, _) => {
                        h723
                    }
                });
                let a735 = {
                    let a732 = f161(match v0.clone() {
                        Adt17::C0(_, _, _, _, h724, _, _, _) => {
                            h724
                        }
                    });
                    let a733 = {
                        let a730 = f161(match v0.clone() {
                            Adt17::C0(_, _, _, _, _, h725, _, _) => {
                                h725
                            }
                        });
                        let a731 = {
                            let a728 = f161(match v0.clone() {
                                Adt17::C0(_, _, _, _, _, _, h726, _) => {
                                    h726
                                }
                            });
                            let a729 = f161(match v0 {
                                Adt17::C0(_, _, _, _, _, _, _, h727) => {
                                    h727
                                }
                            });
                            append_bytes(a728, a729)
                        };
                        append_bytes(a730, a731)
                    };
                    append_bytes(a732, a733)
                };
                append_bytes(a734, a735)
            };
            append_bytes(a736, a737)
        };
        append_bytes(a738, a739)
    };
    append_bytes(a740, a741)
}

pub fn f112(v0: Bytes) -> R<Adt17> {
    let v1: u64 = {
        let a742 = v0.clone();
        length_bytes(a742)
    };
    let v2: u64 = {
        let a748 = {
            let a745 = {
                let a743 = v1;
                let a744 = 72u64;
                nat_add(a743, a744)?
            };
            let a746 = 64u64;
            let a747 = 0u64;
            nat_quot(a745, a746, a747)
        };
        let a749 = 64u64;
        nat_mul(a748, a749)?
    };
    let (_, h750) = f162(Fn3::F163(v0, v1, v2), {
        let a751 = v2;
        let a752 = 64u64;
        let a753 = 0u64;
        nat_quot(a751, a752, a753)
    }, (0u64, f164()))?;
    Ok(h750)
}

pub fn f113(v0: Adt1, v1: Bytes) -> R<Option<Adt2>> {
    let m754 = f165(f166(v0.clone(), v1.clone()), v1.clone());
    match m754 {
        None => {
            f43(f88(v0, v1))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f114(v0: Bytes, p757: List<Bytes>) -> bool {
    let m756 = p757;
    {
        let mut m756 = m756;
        loop {
            let step = {
                let v1 = m756;
                let m755 = v1;
                match m755.uncons() {
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
                Ok(next) => { m756 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f115(v0: Adt2, v1: Bytes) -> Bytes {
    let m758 = f72(f71(v0, v1));
    match m758 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f116(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m759 = v0;
    match m759.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f116(v3, v1))
        }
    }
}

pub fn f117(v0: Bytes) -> R<Bytes> {
    f167(v0.clone(), 0u64, {
        let a760 = v0;
        length_bytes(a760)
    }, true, Bytes::lit(&[]))
}

pub fn f118(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m761 = v0;
    match m761.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f118(v3, v1))
        }
    }
}

pub fn f119(v0: Adt2) -> R<Bytes> {
    let a762 = f168(v0)?;
    let a763 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a762, a763))
}

pub fn f120(v0: List<Adt2>) -> Adt2 {
    let m764 = v0;
    match m764.uncons() {
        None => {
            Adt2::C9
        }
        Some((v1, v2)) => {
            Adt2::C10(std::rc::Rc::new(v1), std::rc::Rc::new(f120(v2)))
        }
    }
}

pub fn f121(v0: List<Adt2>, v1: Bytes, v2: Bytes, v3: Adt2) -> List<Adt2> {
    let m765 = v0;
    match m765.uncons() {
        None => {
            List::cons(v3, List::<Adt2>::nil())
        }
        Some((v4, v5)) => {
            if f32(f115(v4.clone(), v1.clone()), v2.clone()) {
                List::cons(v3, f169(v5, v1, v2))
            } else {
                List::cons(v4, f121(v5, v1, v2, v3))
            }
        }
    }
}

pub fn f122(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f148(f170(f171(v0)?, v1)?, f8(Str::lit("\u{a}"))))
}

pub fn f123(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f119(Adt2::C7(std::rc::Rc::new(f84(f172(f116(v1, f73(f51(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f124(p776: u64, p777: Bytes) -> R<Bytes> {
    let m775 = (p776, p777);
    {
        let mut m775 = m775;
        loop {
            let step = {
                let (v0, v1) = m775;
                let m766 = v0;
                if m766 == 0 {
                    Err::<(u64, Bytes), R<Bytes>>(Ok(v1))
                } else {
                    let v2 = m766 - 1;
                    if f53(v1.clone(), f8(Str::lit("-- "))) {
                        let m767 = f173(v1.clone(), f8(Str::lit("\u{a}")))?;
                        match m767 {
                            None => {
                                Err::<(u64, Bytes), R<Bytes>>(Ok(v1))
                            }
                            Some(v3) => {
                                Ok::<(u64, Bytes), R<Bytes>>((v2, f46(v1.clone(), {
                                    let a768 = v3;
                                    let a769 = 1u64;
                                    nat_add(a768, a769)?
                                }, {
                                    let a773 = {
                                        let a770 = v1;
                                        length_bytes(a770)
                                    };
                                    let a774 = {
                                        let a771 = v3;
                                        let a772 = 1u64;
                                        nat_add(a771, a772)?
                                    };
                                    nat_sub(a773, a774)
                                })))
                            }
                        }
                    } else {
                        Err::<(u64, Bytes), R<Bytes>>(Ok(v1))
                    }
                }
            };
            match step {
                Ok(next) => { m775 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f125(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m782 = {
        let a779 = v0;
        let a780 = v2;
        let a781 = {
            let a778 = v1.clone();
            length_bytes(a778)
        };
        slice_bytes(a779, a780, a781)
    };
    match m782 {
        None => {
            false
        }
        Some(v3) => {
            let a783 = v3;
            let a784 = v1;
            equal(a783, a784)
        }
    }
}

pub fn f126(v0: Bytes) -> R<Bytes> {
    let (_, h785) = {
        let (h786, _) = f174(Fn4::F175(v0.clone()), {
            let a788 = {
                let a787 = v0;
                length_bytes(a787)
            };
            let a789 = 1u64;
            nat_add(a788, a789)?
        }, (0u64, Bytes::lit(&[])))?;
        h786
    };
    Ok(h785)
}

pub fn f127(v0: Bytes) -> R<u64> {
    let (h790, _) = f176(Fn5::F177(v0.clone()), {
        let a792 = {
            let a791 = v0;
            length_bytes(a791)
        };
        let a793 = 1u64;
        nat_add(a792, a793)?
    }, 0u64)?;
    Ok(h790)
}

pub fn f128(v0: Bytes, v1: u64) -> R<u64> {
    let (h794, _) = f176(Fn5::F178(v0.clone()), {
        let a796 = {
            let a795 = v0;
            length_bytes(a795)
        };
        let a797 = 1u64;
        nat_add(a796, a797)?
    }, v1)?;
    Ok(h794)
}

pub fn f129(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a798 = v0;
        utf8_encode(a798)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f89(v1)?, List::<Bytes>::nil())))))
}

pub fn f130(v0: Bytes, p807: u64, p808: u64) -> R<Result<u64, Bytes>> {
    let m806 = (p807, p808);
    {
        let mut m806 = m806;
        loop {
            let step = {
                let (v1, v2) = m806;
                let m799 = v2;
                if m799 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Ok::<u64, Bytes>(v1)))
                } else {
                    let v3 = m799 - 1;
                    let a801 = v1;
                    let a802 = {
                        let a800 = v0.clone();
                        length_bytes(a800)
                    };
                    if if nat_lt(a801, a802) {
                        if f179(v0.clone(), v1, Str::lit("...")) {
                            f180(v0.clone(), v1)?
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        let m805 = f181(v0.clone(), {
                            let a803 = v1;
                            let a804 = 3u64;
                            nat_add(a803, a804)?
                        })?;
                        match m805 {
                            Ok(v5) => {
                                Ok::<(u64, u64), R<Result<u64, Bytes>>>((f128(v0.clone(), v5)?, v3))
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
                Ok(next) => { m806 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f131(v0: Bytes, v1: u64, v2: bool, v3: u64) -> R<Result<List<Adt2>, Bytes>> {
    let m809 = v3;
    if m809 == 0 {
        Ok(Err::<List<Adt2>, Bytes>(f129(Str::lit("the stream holds more documents than it is long"), v1)?))
    } else {
        let v4 = m809 - 1;
        let v5: u64 = {
            let a810 = v0.clone();
            length_bytes(a810)
        };
        let a811 = f182(v0.clone(), v1)?;
        let a812 = 37u64;
        if nat_eq(a811, a812) {
            Ok(Err::<List<Adt2>, Bytes>(f129(Str::lit("a directive is not read (YAML 1.2.2 [82])"), v1)?))
        } else {
            let m819 = if if f179(v0.clone(), v1, Str::lit("---")) {
                f180(v0.clone(), v1)?
            } else {
                false
            } {
                f183(v0.clone(), {
                    let a813 = v1;
                    let a814 = 3u64;
                    nat_add(a813, a814)?
                }, 0u64, {
                    let a815 = v5;
                    let a816 = 2u64;
                    nat_add(a815, a816)?
                }, 1u64, false, 0u64, List::<Bytes>::nil(), Adt18::C0(0u64))?
            } else {
                if v2 {
                    f183(v0.clone(), v1, 0u64, {
                        let a817 = v5;
                        let a818 = 2u64;
                        nat_add(a817, a818)?
                    }, 0u64, false, 0u64, List::<Bytes>::nil(), Adt18::C0(0u64))?
                } else {
                    Adt19::C1(f129(Str::lit("only comments, a document end marker or a document start marker may follow a document (YAML 1.2.2 [207], [211])"), v1)?)
                }
            };
            match m819 {
                Adt19::C0(v7, v8) => {
                    let v9: u64 = f128(v0.clone(), v8)?;
                    let m822 = f130(v0.clone(), v9, {
                        let a820 = v5;
                        let a821 = 1u64;
                        nat_add(a820, a821)?
                    })?;
                    match m822 {
                        Ok(v11) => {
                            let a823 = v5;
                            let a824 = v11;
                            if nat_le(a823, a824) {
                                Ok(Ok::<List<Adt2>, Bytes>(List::cons(v7, List::<Adt2>::nil())))
                            } else {
                                let m827 = f131(v0, v11, f184({
                                    let a825 = v11;
                                    let a826 = v9;
                                    nat_eq(a825, a826)
                                }), v4)?;
                                match m827 {
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
                Adt19::C1(v6) => {
                    Ok(Err::<List<Adt2>, Bytes>(v6))
                }
            }
        }
    }
}

pub fn f132(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a828 = v2;
    let a829 = 0u64;
    if nat_eq(a828, a829) {
        Ok(f185(v0, 0u64))
    } else {
        let (_, h830) = {
            let (h831, _) = f174(Fn4::F186(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h831
        };
        Ok(h830)
    }
}

pub fn f133() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f134(v0: Bytes) -> R<Result<List<Adt13>, Bytes>> {
    let m832 = f187(v0)?;
    match m832 {
        Ok(v2) => {
            f188({
                let a834 = {
                    let a833 = v2.clone();
                    length_list(a833)
                };
                let a835 = 1u64;
                nat_add(a834, a835)?
            }, v2, List::<Adt13>::nil())
        }
        Err(v1) => {
            Ok(Err::<List<Adt13>, Bytes>(v1))
        }
    }
}

pub fn f135(v0: List<Bytes>) -> R<Result<List<Adt13>, Bytes>> {
    let m836 = v0;
    match m836.uncons() {
        None => {
            Ok(Ok::<List<Adt13>, Bytes>(List::<Adt13>::nil()))
        }
        Some((v1, v2)) => {
            let m837 = f134(v1)?;
            match m837 {
                Ok(v4) => {
                    let m838 = f135(v2)?;
                    match m838 {
                        Ok(v6) => {
                            Ok(Ok::<List<Adt13>, Bytes>(f136(v4, v6)))
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

pub fn f136(v0: List<Adt13>, v1: List<Adt13>) -> List<Adt13> {
    let m839 = v0;
    match m839.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f136(v3, v1))
        }
    }
}

pub fn f137(p844: u64, p845: List<Adt16>) -> R<List<Adt16>> {
    let m843 = (p844, p845);
    {
        let mut m843 = m843;
        loop {
            let step = {
                let (v0, v1) = m843;
                let m840 = v0;
                if m840 == 0 {
                    Err::<(u64, List<Adt16>), R<List<Adt16>>>(Ok(v1))
                } else {
                    let v2 = m840 - 1;
                    let v3: List<Adt16> = f189(v1.clone(), v1.clone());
                    let a841 = f190(v3.clone())?;
                    let a842 = f190(v1.clone())?;
                    if nat_eq(a841, a842) {
                        Err::<(u64, List<Adt16>), R<List<Adt16>>>(Ok(v1))
                    } else {
                        Ok::<(u64, List<Adt16>), R<List<Adt16>>>((v2, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m843 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f138() -> Bytes {
    f8(Str::lit("Binary32"))
}

pub fn f139(v0: bool) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(if v0 {
        f140()
    } else {
        f138()
    }, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: bits, type: Nat}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))
}

pub fn f140() -> Bytes {
    f8(Str::lit("Binary64"))
}

pub fn f141(v0: List<Adt13>, v1: List<Adt13>) -> R<List<Adt16>> {
    let m846 = v0;
    match m846.uncons() {
        None => {
            Ok(List::<Adt16>::nil())
        }
        Some((v2, v3)) => {
            if f191(f192(v2.clone())) {
                let v6: Adt16 = {
                    let m847 = f193(v1.clone(), v2.clone())?;
                    match m847 {
                        Ok(v5) => {
                            Adt16::C0(f194(v2), Bytes::lit(&[]), {
                                let (h848, _) = v5;
                                h848
                            }, {
                                let (_, h849) = v5;
                                h849
                            })
                        }
                        Err(v4) => {
                            Adt16::C0(f194(v2), v4, Bytes::lit(&[]), List::<Bytes>::nil())
                        }
                    }
                };
                Ok(List::cons(v6, f141(v3, v1)?))
            } else {
                f141(v3, v1)
            }
        }
    }
}

pub fn f142(p854: u64, p855: List<Bytes>, p856: List<Bytes>, v3: List<Adt16>) -> List<Bytes> {
    let m853 = (p854, (p855, p856));
    {
        let mut m853 = m853;
        loop {
            let step = {
                let (v0, (v1, v2)) = m853;
                let m850 = v0;
                if m850 == 0 {
                    Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v2)
                } else {
                    let v4 = m850 - 1;
                    let m851 = v1;
                    match m851.uncons() {
                        None => {
                            Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v2)
                        }
                        Some((v5, v6)) => {
                            if f114(v5.clone(), v2.clone()) {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v2)))
                            } else {
                                let m852 = f195(v3.clone(), v5.clone());
                                match m852 {
                                    None => {
                                        Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v2)))
                                    }
                                    Some(v7) => {
                                        if f32(f196(v7.clone()), Bytes::lit(&[])) {
                                            Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (f197(f198(v7), v6), List::cons(v5, v2))))
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
                Ok(next) => { m853 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f143(v0: List<Adt13>) -> List<Bytes> {
    let m857 = v0;
    match m857.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let v3: u64 = f192(v1.clone());
            let a858 = v3;
            let a859 = 0u64;
            if if nat_eq(a858, a859) {
                true
            } else {
                let a860 = v3;
                let a861 = 2u64;
                if nat_eq(a860, a861) {
                    true
                } else {
                    let a862 = v3;
                    let a863 = 3u64;
                    if nat_eq(a862, a863) {
                        true
                    } else {
                        let a864 = v3;
                        let a865 = 4u64;
                        if nat_eq(a864, a865) {
                            true
                        } else {
                            let a866 = v3;
                            let a867 = 5u64;
                            if nat_eq(a866, a867) {
                                true
                            } else {
                                let a868 = v3;
                                let a869 = 7u64;
                                if nat_eq(a868, a869) {
                                    true
                                } else {
                                    let a870 = v3;
                                    let a871 = 8u64;
                                    nat_eq(a870, a871)
                                }
                            }
                        }
                    }
                }
            } {
                List::cons(f194(v1), f143(v2))
            } else {
                f143(v2)
            }
        }
    }
}

pub fn f144(v0: List<Adt16>, v1: List<Bytes>) -> List<(Bytes, (List<Bytes>, Bytes))> {
    let m872 = v0;
    match m872.uncons() {
        None => {
            List::<(Bytes, (List<Bytes>, Bytes))>::nil()
        }
        Some((v2, v3)) => {
            if if f114(f199(v2.clone()), v1.clone()) {
                f32(f196(v2.clone()), Bytes::lit(&[]))
            } else {
                false
            } {
                List::cons((f199(v2.clone()), (f198(v2.clone()), f200(v2))), f144(v3, v1))
            } else {
                f144(v3, v1)
            }
        }
    }
}

pub fn f145(p881: u64, p882: List<(Bytes, (List<Bytes>, Bytes))>, p883: List<Bytes>, p884: List<Bytes>) -> (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>) {
    let m880 = (p881, (p882, (p883, p884)));
    {
        let mut m880 = m880;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m880;
                let m873 = v0;
                if m873 == 0 {
                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v3, v1))
                } else {
                    let v4 = m873 - 1;
                    let m874 = f201(v1.clone(), v2.clone());
                    match m874 {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v3, v1))
                        }
                        Some(v5) => {
                            Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v4, ({
                                let (_, h875) = v5.clone();
                                h875
                            }, (List::cons({
                                let (h876, _) = {
                                    let (h877, _) = v5.clone();
                                    h877
                                };
                                h876
                            }, v2), List::cons({
                                let (_, h878) = {
                                    let (h879, _) = v5;
                                    h879
                                };
                                h878
                            }, v3)))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m880 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f146(v0: List<Adt13>, v1: List<Adt16>, v2: List<Adt13>) -> List<Bytes> {
    let m885 = v0;
    match m885.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            let v5: u64 = f192(v3.clone());
            let a886 = v5;
            let a887 = 5u64;
            if if nat_eq(a886, a887) {
                true
            } else {
                let a888 = v5;
                let a889 = 7u64;
                nat_eq(a888, a889)
            } {
                let m892 = f202(v2.clone(), {
                    let a890 = v5;
                    let a891 = 5u64;
                    if nat_eq(a890, a891) {
                        0u64
                    } else {
                        2u64
                    }
                }, f194(v3.clone()));
                match m892 {
                    None => {
                        List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f203(v5), List::cons(f194(v3.clone()), List::cons(f8(Str::lit(": no input defines ")), List::cons(f194(v3), List::cons(f8(Str::lit(", so its members join no definition (Web IDL 2.2 and 2.7)\u{a}")), List::<Bytes>::nil()))))))), f146(v4, v1, v2))
                    }
                    Some(_) => {
                        f146(v4, v1, v2)
                    }
                }
            } else {
                let a893 = v5;
                let a894 = 8u64;
                if nat_eq(a893, a894) {
                    let m895 = f202(v2.clone(), 0u64, f194(v3.clone()));
                    match m895 {
                        None => {
                            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f194(v3.clone()), List::cons(f8(Str::lit(" includes ")), List::cons(f204(v3.clone()), List::cons(f8(Str::lit(": no input defines the interface ")), List::cons(f194(v3), List::cons(f8(Str::lit(" (Web IDL 2.3)\u{a}")), List::<Bytes>::nil())))))))), f146(v4, v1, v2))
                        }
                        Some(_) => {
                            f146(v4, v1, v2)
                        }
                    }
                } else {
                    if f191(v5) {
                        let m896 = f195(v1.clone(), f194(v3.clone()));
                        match m896 {
                            None => {
                                f146(v4, v1, v2)
                            }
                            Some(v8) => {
                                if f32(f196(v8.clone()), Bytes::lit(&[])) {
                                    f146(v4, v1, v2)
                                } else {
                                    List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f194(v3), List::cons(f8(Str::lit(": ")), List::cons(f196(v8), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), f146(v4, v1, v2))
                                }
                            }
                        }
                    } else {
                        f146(v4, v1, v2)
                    }
                }
            }
        }
    }
}

pub fn f147(v0: List<(Bytes, (List<Bytes>, Bytes))>) -> List<Bytes> {
    let m897 = v0;
    match m897.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons({
                let (h898, _) = v1;
                h898
            }, List::cons(f8(Str::lit(": it refers back to itself through its members, and the translator emits non-recursive declarations only\u{a}")), List::<Bytes>::nil())))), f147(v2))
        }
    }
}

pub fn f148(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m899 = v0;
    match m899.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m900 = v3.clone();
            match m900.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a903 = v2;
                    let a904 = {
                        let a901 = v1.clone();
                        let a902 = f148(v3, v1);
                        append_bytes(a901, a902)
                    };
                    append_bytes(a903, a904)
                }
            }
        }
    }
}

pub fn f149(p907: List<Bytes>, p908: List<Bytes>) -> List<Bytes> {
    let m906 = (p907, p908);
    {
        let mut m906 = m906;
        loop {
            let step = {
                let (v0, v1) = m906;
                let m905 = v0;
                match m905.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m906 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f150(v0: Adt2) -> R<Result<Bytes, Bytes>> {
    let m909 = f205(v0.clone());
    match m909 {
        None => {
            let m910 = f206(v0.clone())?;
            match m910 {
                None => {
                    Ok(Ok::<Bytes, Bytes>(f168(v0)?))
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

pub fn f151(v0: Bytes, v1: u64, v2: u64, v3: Bytes) -> Bytes {
    let a913 = v3;
    let a914 = f46(v0, v1, {
        let a911 = v2;
        let a912 = v1;
        nat_sub(a911, a912)
    });
    append_bytes(a913, a914)
}

pub fn f152(v0: Bytes, p927: u64, p928: u64) -> R<u64> {
    let m926 = (p927, p928);
    {
        let mut m926 = m926;
        loop {
            let step = {
                let (v1, v2) = m926;
                let m915 = v2;
                if m915 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m915 - 1;
                    let v4: u64 = f103(v0.clone(), v1)?;
                    let a916 = v4;
                    let a917 = 34u64;
                    if nat_eq(a916, a917) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a918 = v4;
                        let a919 = 92u64;
                        if nat_eq(a918, a919) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a920 = v4;
                            let a921 = 32u64;
                            if nat_lt(a920, a921) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a922 = v4;
                                let a923 = 256u64;
                                if nat_eq(a922, a923) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a924 = v1;
                                        let a925 = 1u64;
                                        nat_add(a924, a925)?
                                    }, v3))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m926 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f153(v0: Bytes, p954: u64, p955: u64, p956: Bytes) -> R<Option<(u64, Bytes)>> {
    let m953 = (p954, (p955, p956));
    {
        let mut m953 = m953;
        loop {
            let step = {
                let (v1, (v2, v3)) = m953;
                let m929 = v2;
                if m929 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                } else {
                    let v4 = m929 - 1;
                    let v5: u64 = f152(v0.clone(), v1, {
                        let a933 = {
                            let a931 = {
                                let a930 = v0.clone();
                                length_bytes(a930)
                            };
                            let a932 = v1;
                            nat_sub(a931, a932)
                        };
                        let a934 = 1u64;
                        nat_add(a933, a934)?
                    })?;
                    let v6: Bytes = {
                        let a937 = v3;
                        let a938 = f46(v0.clone(), v1, {
                            let a935 = v5;
                            let a936 = v1;
                            nat_sub(a935, a936)
                        });
                        append_bytes(a937, a938)
                    };
                    let v7: u64 = f103(v0.clone(), v5)?;
                    let a939 = v7;
                    let a940 = 34u64;
                    if nat_eq(a939, a940) {
                        Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(Some(({
                            let a941 = v5;
                            let a942 = 1u64;
                            nat_add(a941, a942)?
                        }, v6))))
                    } else {
                        let a943 = v7;
                        let a944 = 92u64;
                        if nat_eq(a943, a944) {
                            let m945 = f207(v0.clone(), v5, v6)?;
                            match m945 {
                                None => {
                                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                                }
                                Some(v8) => {
                                    let a948 = {
                                        let (_, h946) = {
                                            let (_, h947) = v8.clone();
                                            h947
                                        };
                                        h946
                                    };
                                    let a949 = 0u64;
                                    if nat_eq(a948, a949) {
                                        Ok::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(({
                                            let (h950, _) = v8;
                                            h950
                                        }, (v4, {
                                            let (h951, _) = {
                                                let (_, h952) = v8;
                                                h952
                                            };
                                            h951
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
                Ok(next) => { m953 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f154(v0: Bytes, v1: u64) -> R<u64> {
    let (h957, _) = f176(Fn5::F208(v0.clone()), {
        let a959 = {
            let a958 = v0;
            length_bytes(a958)
        };
        let a960 = 1u64;
        nat_add(a959, a960)?
    }, v1)?;
    Ok(h957)
}

pub fn f155(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f156(v0: Bytes) -> R<u64> {
    let (h961, _) = f176(Fn5::F209(v0.clone()), {
        let a963 = {
            let a962 = v0;
            length_bytes(a962)
        };
        let a964 = 1u64;
        nat_add(a963, a964)?
    }, 0u64)?;
    Ok(h961)
}

pub fn f157(v0: Bytes) -> R<Option<u64>> {
    if f210(v0.clone())? {
        Ok(Some({
            let (_, h965) = {
                let (h966, _) = f211(Fn6::F212(v0.clone()), {
                    let a967 = v0;
                    length_bytes(a967)
                }, (0u64, 0u64))?;
                h966
            };
            h965
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f158(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f46(v1.clone(), 0u64, {
        let a969 = {
            let a968 = v1.clone();
            length_bytes(a968)
        };
        let a970 = f213(v1)?;
        nat_sub(a969, a970)
    });
    let v5: Bytes = {
        let a971 = v0;
        let a972 = v4.clone();
        append_bytes(a971, a972)
    };
    let v6: u64 = f156(v5.clone())?;
    let v7: Bytes = f46(v5.clone(), v6, {
        let a974 = {
            let a973 = v5;
            length_bytes(a973)
        };
        let a975 = v6;
        nat_sub(a974, a975)
    });
    let v8: u64 = {
        let a976 = v4;
        length_bytes(a976)
    };
    let v9: u64 = {
        let a977 = v7.clone();
        length_bytes(a977)
    };
    let a978 = v9;
    let a979 = 0u64;
    if nat_eq(a978, a979) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f155(v2, {
            let a980 = v3;
            let a981 = v8;
            nat_lt(a980, a981)
        }) {
            let v10: u64 = if v2 {
                let a982 = v3;
                let a983 = v8;
                nat_add(a982, a983)?
            } else {
                let a984 = v8;
                let a985 = v3;
                nat_sub(a984, a985)
            };
            let v11: u64 = f213(v7.clone())?;
            let a986 = v10;
            let a987 = v11;
            if nat_le(a986, a987) {
                Ok(Some((f46(v7, 0u64, {
                    let a988 = v9;
                    let a989 = v10;
                    nat_sub(a988, a989)
                }), 0u64)))
            } else {
                let a992 = {
                    let a990 = v10;
                    let a991 = v11;
                    nat_sub(a990, a991)
                };
                let a993 = 19u64;
                if nat_le(a992, a993) {
                    Ok(Some((f46(v7, 0u64, {
                        let a994 = v9;
                        let a995 = v11;
                        nat_sub(a994, a995)
                    }), {
                        let a996 = v10;
                        let a997 = v11;
                        nat_sub(a996, a997)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a998 = v3;
                let a999 = v8;
                nat_sub(a998, a999)
            };
            let a1002 = {
                let a1000 = v9;
                let a1001 = v12;
                nat_add(a1000, a1001)?
            };
            let a1003 = 19u64;
            if nat_le(a1002, a1003) {
                Ok(Some(({
                    let a1004 = v7;
                    let a1005 = f214(v12);
                    append_bytes(a1004, a1005)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f159(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1006 = 0u64;
        let a1007 = v1;
        if nat_lt(a1006, a1007) {
            let a1008 = v1;
            let a1009 = 9223372036854775808u64;
            nat_le(a1008, a1009)
        } else {
            false
        }
    } else {
        let a1010 = v1;
        let a1011 = 9223372036854775807u64;
        nat_le(a1010, a1011)
    }
}

pub fn f160(v0: Bytes, v1: u64) -> Option<u8> {
    let a1012 = v0;
    let a1013 = v1;
    index_bytes(a1012, a1013)
}

pub fn f161(v0: u32) -> Bytes {
    let a1042 = f215({
        let a1014 = f216(v0, 28u32);
        let a1015 = 15u32;
        fixed_u32::bit_and(a1014, a1015)
    });
    let a1043 = {
        let a1040 = f215({
            let a1016 = f216(v0, 24u32);
            let a1017 = 15u32;
            fixed_u32::bit_and(a1016, a1017)
        });
        let a1041 = {
            let a1038 = f215({
                let a1018 = f216(v0, 20u32);
                let a1019 = 15u32;
                fixed_u32::bit_and(a1018, a1019)
            });
            let a1039 = {
                let a1036 = f215({
                    let a1020 = f216(v0, 16u32);
                    let a1021 = 15u32;
                    fixed_u32::bit_and(a1020, a1021)
                });
                let a1037 = {
                    let a1034 = f215({
                        let a1022 = f216(v0, 12u32);
                        let a1023 = 15u32;
                        fixed_u32::bit_and(a1022, a1023)
                    });
                    let a1035 = {
                        let a1032 = f215({
                            let a1024 = f216(v0, 8u32);
                            let a1025 = 15u32;
                            fixed_u32::bit_and(a1024, a1025)
                        });
                        let a1033 = {
                            let a1030 = f215({
                                let a1026 = f216(v0, 4u32);
                                let a1027 = 15u32;
                                fixed_u32::bit_and(a1026, a1027)
                            });
                            let a1031 = f215({
                                let a1028 = v0;
                                let a1029 = 15u32;
                                fixed_u32::bit_and(a1028, a1029)
                            });
                            append_bytes(a1030, a1031)
                        };
                        append_bytes(a1032, a1033)
                    };
                    append_bytes(a1034, a1035)
                };
                append_bytes(a1036, a1037)
            };
            append_bytes(a1038, a1039)
        };
        append_bytes(a1040, a1041)
    };
    append_bytes(a1042, a1043)
}

pub fn f162(v0: Fn3, p1047: u64, p1048: (u64, Adt17)) -> R<(u64, Adt17)> {
    let m1046 = (p1047, p1048);
    {
        let mut m1046 = m1046;
        loop {
            let step = {
                let (v1, v2) = m1046;
                let m1044 = v1;
                if m1044 == 0 {
                    Err::<(u64, (u64, Adt17)), R<(u64, Adt17)>>(Ok(v2))
                } else {
                    let v3 = m1044 - 1;
                    Ok::<(u64, (u64, Adt17)), R<(u64, Adt17)>>((v3, {
                        let c1045: Fn3 = v0.clone();
                        c1045.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1046 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f163(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt17)) -> R<(u64, Adt17)> {
    Ok(({
        let a1050 = {
            let (h1049, _) = v3.clone();
            h1049
        };
        let a1051 = 64u64;
        nat_add(a1050, a1051)?
    }, f217({
        let (_, h1052) = v3;
        h1052
    }, f218(v0, v1, v2, {
        let (h1053, _) = v3;
        h1053
    })?)?))
}

pub fn f164() -> Adt17 {
    Adt17::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f165(p1058: List<(Bytes, Option<Adt2>)>, v1: Bytes) -> Option<Option<Adt2>> {
    let m1057 = p1058;
    {
        let mut m1057 = m1057;
        loop {
            let step = {
                let v0 = m1057;
                let m1054 = v0;
                match m1054.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(None::<Option<Adt2>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h1055, _) = v2;
                            h1055
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(Some({
                                let (_, h1056) = v2;
                                h1056
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1057 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f166(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Adt2>)> {
    let m1062 = {
        let a1060 = match v0 {
            Adt1::C0(_, _, _, h1059) => {
                h1059
            }
        };
        let a1061 = f40(v1);
        index_list(a1060, a1061)
    };
    match m1062 {
        None => {
            List::<(Bytes, Option<Adt2>)>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f167(v0: Bytes, p1083: u64, p1084: u64, p1085: bool, p1086: Bytes) -> R<Bytes> {
    let m1082 = (p1083, (p1084, (p1085, p1086)));
    {
        let mut m1082 = m1082;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1082;
                let m1063 = v2;
                if m1063 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1063 - 1;
                    let m1066 = {
                        let a1064 = v0.clone();
                        let a1065 = v1;
                        index_bytes(a1064, a1065)
                    };
                    match m1066 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f60(v6)?;
                            if f219(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1067 = v1;
                                    let a1068 = 1u64;
                                    nat_add(a1067, a1068)?
                                }, (v5, (true, v4))))
                            } else {
                                if f105(f220(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1069 = v1;
                                        let a1070 = 1u64;
                                        nat_add(a1069, a1070)?
                                    }, (v5, (false, {
                                        let a1074 = v4;
                                        let a1075 = f46({
                                            let a1071 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1071)
                                        }, {
                                            let a1072 = v7;
                                            let a1073 = 97u64;
                                            nat_sub(a1072, a1073)
                                        }, 1u64);
                                        append_bytes(a1074, a1075)
                                    }))))
                                } else {
                                    if f155(f220(v7), f155(f221(v7), f222(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1076 = v1;
                                            let a1077 = 1u64;
                                            nat_add(a1076, a1077)?
                                        }, (v5, (false, {
                                            let a1078 = v4;
                                            let a1079 = f46(v0.clone(), v1, 1u64);
                                            append_bytes(a1078, a1079)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1080 = v1;
                                            let a1081 = 1u64;
                                            nat_add(a1080, a1081)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
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

pub fn f168(v0: Adt2) -> R<Bytes> {
    let m1087 = v0;
    match m1087 {
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
            f223(v4, v5)
        }
        Adt2::C4(v6, v7, v8) => {
            f224(v6, v7, v8)
        }
        Adt2::C5(v9, v10) => {
            f225(v9, v10)
        }
        Adt2::C6(v11) => {
            f226(v11)
        }
        Adt2::C7(r1088) => {
            let v12 = (*r1088).clone();
            Ok(f227(f228(v12)?))
        }
        Adt2::C8(r1089) => {
            let v13 = (*r1089).clone();
            f229(f230(v13)?)
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

pub fn f169(v0: List<Adt2>, v1: Bytes, v2: Bytes) -> List<Adt2> {
    let m1090 = v0;
    match m1090.uncons() {
        None => {
            List::<Adt2>::nil()
        }
        Some((v3, v4)) => {
            if f32(f115(v3.clone(), v1.clone()), v2.clone()) {
                f169(v4, v1, v2)
            } else {
                List::cons(v3, f169(v4, v1, v2))
            }
        }
    }
}

pub fn f170(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m1091 = v0;
    match m1091.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f53(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f148(f231(f172(List::cons(v1, f232(f44(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f170(v3, v1)?))
            }
        }
    }
}

pub fn f171(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a1092 = v0.clone();
        length_bytes(a1092)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h1093, _) = f64(Fn0::F233(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h1093
    };
    let v3: u64 = {
        let (h1094, _) = {
            let (_, h1095) = v2.clone();
            h1095
        };
        h1094
    };
    let v4: List<Bytes> = {
        let (_, h1096) = {
            let (_, h1097) = v2;
            h1097
        };
        h1096
    };
    let a1098 = v3;
    let a1099 = v1;
    if nat_lt(a1098, a1099) {
        Ok(f66(List::cons(f46(v0, v3, {
            let a1100 = v1;
            let a1101 = v3;
            nat_sub(a1100, a1101)
        }), v4)))
    } else {
        Ok(f66(v4))
    }
}

pub fn f172(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f234(f108(Fn2::F235, List::<Bytes>::nil(), f236(v0)?)))
}

pub fn f173(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a1102 = v0.clone();
        length_bytes(a1102)
    };
    let v3: u64 = {
        let a1103 = v1.clone();
        length_bytes(a1103)
    };
    let v4: u64 = {
        let (h1104, _) = f176(Fn5::F237(v1.clone(), v3, v0.clone(), v2), {
            let a1105 = v2;
            let a1106 = 1u64;
            nat_add(a1105, a1106)?
        }, 0u64)?;
        h1104
    };
    if f125(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f174(v0: Fn4, p1111: u64, p1112: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1110 = (p1111, p1112);
    {
        let mut m1110 = m1110;
        loop {
            let step = {
                let (v1, v2) = m1110;
                let m1107 = v1;
                if m1107 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1107 - 1;
                    let m1109 = {
                        let c1108: Fn4 = v0.clone();
                        c1108.apply(v2.clone())?
                    };
                    match m1109 {
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
                Ok(next) => { m1110 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f175(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f238(v0, v1)
}

pub fn f176(v0: Fn5, p1117: u64, p1118: u64) -> R<(u64, bool)> {
    let m1116 = (p1117, p1118);
    {
        let mut m1116 = m1116;
        loop {
            let step = {
                let (v1, v2) = m1116;
                let m1113 = v1;
                if m1113 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1113 - 1;
                    let m1115 = {
                        let c1114: Fn5 = v0.clone();
                        c1114.apply(v2)?
                    };
                    match m1115 {
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
                Ok(next) => { m1116 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f177(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f239(v0, v1)
}

pub fn f178(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f240(v0, v1)
}

pub fn f179(v0: Bytes, v1: u64, v2: Str) -> bool {
    f125(v0, {
        let a1119 = v2;
        utf8_encode(a1119)
    }, v1)
}

pub fn f180(v0: Bytes, v1: u64) -> R<bool> {
    if if if f179(v0.clone(), v1, Str::lit("---")) {
        true
    } else {
        f179(v0.clone(), v1, Str::lit("..."))
    } {
        if f241(f182(v0.clone(), {
            let a1120 = v1;
            let a1121 = 3u64;
            nat_add(a1120, a1121)?
        })?) {
            true
        } else {
            f242(f182(v0.clone(), {
                let a1122 = v1;
                let a1123 = 3u64;
                nat_add(a1122, a1123)?
            })?)
        }
    } else {
        false
    } {
        let a1124 = f243(v0, v1)?;
        let a1125 = 0u64;
        Ok(nat_eq(a1124, a1125))
    } else {
        Ok(false)
    }
}

pub fn f181(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f244(v0.clone(), v1)?;
    let v3: u64 = f182(v0.clone(), v2)?;
    if if f242(v3) {
        true
    } else {
        let a1126 = v3;
        let a1127 = 35u64;
        if nat_eq(a1126, a1127) {
            let a1128 = v1;
            let a1129 = v2;
            nat_lt(a1128, a1129)
        } else {
            false
        }
    } {
        Ok(Ok::<u64, Bytes>(f245(v0, v2)?))
    } else {
        Ok(Err::<u64, Bytes>(f129(Str::lit("only white space and a comment may follow a node on its line (YAML 1.2.2 [77])"), v2)?))
    }
}

pub fn f182(v0: Bytes, v1: u64) -> R<u64> {
    f103(v0, v1)
}

pub fn f183(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: List<Bytes>, v8: Adt18) -> R<Adt19> {
    let m1130 = v3;
    if m1130 == 0 {
        Ok(Adt19::C1(f129(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v9 = m1130 - 1;
        let a1131 = v4;
        let a1132 = 0u64;
        if nat_eq(a1131, a1132) {
            let v10: u64 = f246(v0.clone(), v1)?;
            let v11: u64 = {
                let a1133 = v10;
                let a1134 = v1;
                nat_sub(a1133, a1134)
            };
            let a1135 = f182(v0.clone(), v10)?;
            let a1136 = 9u64;
            if nat_eq(a1135, a1136) {
                let v12: u64 = f244(v0.clone(), v10)?;
                if if f247(v0.clone(), v12)? {
                    true
                } else {
                    f248(v0.clone(), v12)?
                } {
                    Ok(Adt19::C1(f129(Str::lit("a tab cannot indent a block node (YAML 1.2.2 [63])"), v12)?))
                } else {
                    let m1137 = f249(v0.clone(), v12)?;
                    match m1137 {
                        None => {
                            f250(v0, v12, v2, v8)
                        }
                        Some(v13) => {
                            Ok(Adt19::C1(v13))
                        }
                    }
                }
            } else {
                let a1138 = f182(v0.clone(), v10)?;
                let a1139 = 33u64;
                if if nat_eq(a1138, a1139) {
                    f184(f248(v0.clone(), v10)?)
                } else {
                    false
                } {
                    if f251(v8) {
                        Ok(Adt19::C1(f129(Str::lit("a node holds at most one tag (YAML 1.2.2 [96])"), v10)?))
                    } else {
                        let v14: Adt18 = f252(v0.clone(), v10)?;
                        let m1140 = f253(v14.clone());
                        match m1140 {
                            None => {
                                f183(v0.clone(), f254(v0.clone(), v10)?, v2, v9, 1u64, false, {
                                    let a1142 = {
                                        let a1141 = v0;
                                        length_bytes(a1141)
                                    };
                                    let a1143 = 1u64;
                                    nat_add(a1142, a1143)?
                                }, List::<Bytes>::nil(), v14)
                            }
                            Some(v15) => {
                                Ok(Adt19::C1(v15))
                            }
                        }
                    }
                } else {
                    if f247(v0.clone(), v10)? {
                        f255(v8, true, v10, f183(v0, v10, v2, v9, 2u64, false, v11, List::<Bytes>::nil(), Adt18::C0(0u64))?)
                    } else {
                        let m1146 = {
                            let a1144 = f182(v0.clone(), v10)?;
                            let a1145 = 33u64;
                            if nat_eq(a1144, a1145) {
                                None::<Bytes>
                            } else {
                                f249(v0.clone(), v10)?
                            }
                        };
                        match m1146 {
                            None => {
                                if f248(v0.clone(), v10)? {
                                    f255(v8, false, v10, f183(v0, v10, v2, v9, 3u64, false, v11, List::<Bytes>::nil(), Adt18::C0(0u64))?)
                                } else {
                                    f250(v0, v10, v2, v8)
                                }
                            }
                            Some(v16) => {
                                Ok(Adt19::C1(v16))
                            }
                        }
                    }
                }
            }
        } else {
            let a1147 = v4;
            let a1148 = 1u64;
            if nat_eq(a1147, a1148) {
                let v17: u64 = f244(v0.clone(), v1)?;
                let v18: u64 = f182(v0.clone(), v17)?;
                if if f242(v18) {
                    true
                } else {
                    let a1149 = v18;
                    let a1150 = 35u64;
                    if nat_eq(a1149, a1150) {
                        let a1151 = v1;
                        let a1152 = v17;
                        nat_lt(a1151, a1152)
                    } else {
                        false
                    }
                } {
                    let v19: u64 = f128(v0.clone(), f245(v0.clone(), v17)?)?;
                    let a1154 = {
                        let a1153 = v0.clone();
                        length_bytes(a1153)
                    };
                    let a1155 = v19;
                    if if nat_le(a1154, a1155) {
                        true
                    } else {
                        f180(v0.clone(), v19)?
                    } {
                        f256(v8, v17, v19)
                    } else {
                        let v20: u64 = {
                            let a1156 = f246(v0.clone(), v19)?;
                            let a1157 = v19;
                            nat_sub(a1156, a1157)
                        };
                        let a1158 = v2;
                        let a1159 = v20;
                        if nat_le(a1158, a1159) {
                            f183(v0, v19, v2, v9, 0u64, false, 0u64, List::<Bytes>::nil(), v8)
                        } else {
                            if if if f184(v5) {
                                let a1160 = v20;
                                let a1161 = v6;
                                nat_eq(a1160, a1161)
                            } else {
                                false
                            } {
                                f247(v0.clone(), {
                                    let a1162 = v19;
                                    let a1163 = v20;
                                    nat_add(a1162, a1163)?
                                })?
                            } else {
                                false
                            } {
                                f255(v8, true, v17, f183(v0, {
                                    let a1164 = v19;
                                    let a1165 = v20;
                                    nat_add(a1164, a1165)?
                                }, v2, v9, 2u64, false, v20, List::<Bytes>::nil(), Adt18::C0(0u64))?)
                            } else {
                                f256(v8, v17, v19)
                            }
                        }
                    }
                } else {
                    let a1166 = v18;
                    let a1167 = 33u64;
                    if if nat_eq(a1166, a1167) {
                        f251(v8.clone())
                    } else {
                        false
                    } {
                        Ok(Adt19::C1(f129(Str::lit("a node holds at most one tag (YAML 1.2.2 [96])"), v17)?))
                    } else {
                        let a1168 = v18;
                        let a1169 = 33u64;
                        if if nat_eq(a1168, a1169) {
                            f184(if v5 {
                                f248(v0.clone(), v17)?
                            } else {
                                false
                            })
                        } else {
                            false
                        } {
                            let v21: Adt18 = f252(v0.clone(), v17)?;
                            let m1170 = f253(v21.clone());
                            match m1170 {
                                None => {
                                    f183(v0.clone(), f254(v0, v17)?, v2, v9, 1u64, v5, v6, List::<Bytes>::nil(), v21)
                                }
                                Some(v22) => {
                                    Ok(Adt19::C1(v22))
                                }
                            }
                        } else {
                            if if v5 {
                                f247(v0.clone(), v17)?
                            } else {
                                false
                            } {
                                let a1173 = f257(f258(v0.clone(), v1, v17), 9u64)?;
                                let a1174 = {
                                    let a1171 = v17;
                                    let a1172 = v1;
                                    nat_sub(a1171, a1172)
                                };
                                if nat_lt(a1173, a1174) {
                                    Ok(Adt19::C1(f129(Str::lit("a tab cannot indent a block node (YAML 1.2.2 [63])"), v17)?))
                                } else {
                                    f255(v8, true, v17, f183(v0.clone(), v17, v2, v9, 2u64, false, f243(v0, v17)?, List::<Bytes>::nil(), Adt18::C0(0u64))?)
                                }
                            } else {
                                let m1177 = {
                                    let a1175 = v18;
                                    let a1176 = 33u64;
                                    if nat_eq(a1175, a1176) {
                                        None::<Bytes>
                                    } else {
                                        f249(v0.clone(), v17)?
                                    }
                                };
                                match m1177 {
                                    None => {
                                        if if v5 {
                                            f248(v0.clone(), v17)?
                                        } else {
                                            false
                                        } {
                                            f255(v8, false, v17, f183(v0.clone(), v17, v2, v9, 3u64, false, f243(v0, v17)?, List::<Bytes>::nil(), Adt18::C0(0u64))?)
                                        } else {
                                            f250(v0, v17, v2, v8)
                                        }
                                    }
                                    Some(v23) => {
                                        Ok(Adt19::C1(v23))
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                let a1178 = v4;
                let a1179 = 2u64;
                if nat_eq(a1178, a1179) {
                    let m1184 = f183(v0.clone(), {
                        let a1180 = v1;
                        let a1181 = 1u64;
                        nat_add(a1180, a1181)?
                    }, {
                        let a1182 = v6;
                        let a1183 = 1u64;
                        nat_add(a1182, a1183)?
                    }, v9, 1u64, true, v6, List::<Bytes>::nil(), Adt18::C0(0u64))?;
                    match m1184 {
                        Adt19::C0(v25, v26) => {
                            let v27: u64 = f128(v0.clone(), v26)?;
                            let v28: u64 = {
                                let a1185 = f246(v0.clone(), v27)?;
                                let a1186 = v27;
                                nat_sub(a1185, a1186)
                            };
                            let a1188 = {
                                let a1187 = v0.clone();
                                length_bytes(a1187)
                            };
                            let a1189 = v27;
                            if nat_le(a1188, a1189) {
                                Ok(Adt19::C0(Adt2::C10(std::rc::Rc::new(v25), std::rc::Rc::new(Adt2::C9)), v27))
                            } else {
                                let a1190 = v28;
                                let a1191 = v6;
                                if if nat_eq(a1190, a1191) {
                                    f247(v0.clone(), {
                                        let a1192 = v27;
                                        let a1193 = v28;
                                        nat_add(a1192, a1193)?
                                    })?
                                } else {
                                    false
                                } {
                                    let m1196 = f183(v0, {
                                        let a1194 = v27;
                                        let a1195 = v28;
                                        nat_add(a1194, a1195)?
                                    }, v2, v9, 2u64, false, v6, List::<Bytes>::nil(), Adt18::C0(0u64))?;
                                    match m1196 {
                                        Adt19::C0(v30, v31) => {
                                            Ok(Adt19::C0(Adt2::C10(std::rc::Rc::new(v25), std::rc::Rc::new(v30)), v31))
                                        }
                                        Adt19::C1(v29) => {
                                            Ok(Adt19::C1(v29))
                                        }
                                    }
                                } else {
                                    let a1197 = v6;
                                    let a1198 = v28;
                                    if nat_lt(a1197, a1198) {
                                        Ok(Adt19::C1(f129(Str::lit("a line is indented more than its block sequence entry allows (YAML 1.2.2 [183])"), {
                                            let a1199 = v27;
                                            let a1200 = v28;
                                            nat_add(a1199, a1200)?
                                        })?))
                                    } else {
                                        Ok(Adt19::C0(Adt2::C10(std::rc::Rc::new(v25), std::rc::Rc::new(Adt2::C9)), v27))
                                    }
                                }
                            }
                        }
                        Adt19::C1(v24) => {
                            Ok(Adt19::C1(v24))
                        }
                    }
                } else {
                    let m1201 = f259(v0.clone(), v1)?;
                    match m1201 {
                        Ok(v33) => {
                            let m1202 = f249(v0.clone(), v33)?;
                            match m1202 {
                                None => {
                                    let m1203 = f260(v0.clone(), v33)?;
                                    match m1203 {
                                        None => {
                                            Ok(Adt19::C1(f129(Str::lit("a block mapping entry must be an implicit key and : (YAML 1.2.2 [188])"), v33)?))
                                        }
                                        Some(v35) => {
                                            let v36: Bytes = f261(v0.clone(), v33)?;
                                            if f262(v36.clone(), v7.clone()) {
                                                Ok(Adt19::C1(f129(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                            } else {
                                                let m1208 = f183(v0.clone(), {
                                                    let a1204 = v35;
                                                    let a1205 = 1u64;
                                                    nat_add(a1204, a1205)?
                                                }, {
                                                    let a1206 = v6;
                                                    let a1207 = 1u64;
                                                    nat_add(a1206, a1207)?
                                                }, v9, 1u64, false, v6, List::<Bytes>::nil(), Adt18::C0(0u64))?;
                                                match m1208 {
                                                    Adt19::C0(v38, v39) => {
                                                        let v40: u64 = f128(v0.clone(), v39)?;
                                                        let v41: u64 = {
                                                            let a1209 = f246(v0.clone(), v40)?;
                                                            let a1210 = v40;
                                                            nat_sub(a1209, a1210)
                                                        };
                                                        let a1212 = {
                                                            let a1211 = v0.clone();
                                                            length_bytes(a1211)
                                                        };
                                                        let a1213 = v40;
                                                        if nat_le(a1212, a1213) {
                                                            Ok(Adt19::C0(Adt2::C11(v36, std::rc::Rc::new(v38), std::rc::Rc::new(Adt2::C9)), v40))
                                                        } else {
                                                            let a1214 = v41;
                                                            let a1215 = v6;
                                                            if if if nat_eq(a1214, a1215) {
                                                                f184(f247(v0.clone(), {
                                                                    let a1216 = v40;
                                                                    let a1217 = v41;
                                                                    nat_add(a1216, a1217)?
                                                                })?)
                                                            } else {
                                                                false
                                                            } {
                                                                f184(f180(v0.clone(), v40)?)
                                                            } else {
                                                                false
                                                            } {
                                                                let m1220 = f183(v0, {
                                                                    let a1218 = v40;
                                                                    let a1219 = v41;
                                                                    nat_add(a1218, a1219)?
                                                                }, v2, v9, 3u64, false, v6, List::cons(v36.clone(), v7), Adt18::C0(0u64))?;
                                                                match m1220 {
                                                                    Adt19::C0(v43, v44) => {
                                                                        Ok(Adt19::C0(Adt2::C11(v36, std::rc::Rc::new(v38), std::rc::Rc::new(v43)), v44))
                                                                    }
                                                                    Adt19::C1(v42) => {
                                                                        Ok(Adt19::C1(v42))
                                                                    }
                                                                }
                                                            } else {
                                                                let a1221 = v6;
                                                                let a1222 = v41;
                                                                if nat_lt(a1221, a1222) {
                                                                    Ok(Adt19::C1(f129(Str::lit("a line is indented more than its block mapping entry allows (YAML 1.2.2 [187])"), {
                                                                        let a1223 = v40;
                                                                        let a1224 = v41;
                                                                        nat_add(a1223, a1224)?
                                                                    })?))
                                                                } else {
                                                                    Ok(Adt19::C0(Adt2::C11(v36, std::rc::Rc::new(v38), std::rc::Rc::new(Adt2::C9)), v40))
                                                                }
                                                            }
                                                        }
                                                    }
                                                    Adt19::C1(v37) => {
                                                        Ok(Adt19::C1(v37))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                Some(v34) => {
                                    Ok(Adt19::C1(v34))
                                }
                            }
                        }
                        Err(v32) => {
                            Ok(Adt19::C1(v32))
                        }
                    }
                }
            }
        }
    }
}

pub fn f184(v0: bool) -> bool {
    !v0
}

pub fn f185(v0: Bytes, v1: u64) -> Bytes {
    let m1228 = {
        let a1225 = v0;
        let a1226 = v1;
        let a1227 = 1u64;
        slice_bytes(a1225, a1226, a1227)
    };
    match m1228 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f186(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f263(v1, v0, v2)
}

pub fn f187(v0: Bytes) -> R<Result<List<Adt20>, Bytes>> {
    f264({
        let a1230 = {
            let a1229 = v0.clone();
            length_bytes(a1229)
        };
        let a1231 = 1u64;
        nat_add(a1230, a1231)?
    }, v0, 0u64, List::<Adt20>::nil())
}

pub fn f188(p1239: u64, p1240: List<Adt20>, p1241: List<Adt13>) -> R<Result<List<Adt13>, Bytes>> {
    let m1238 = (p1239, (p1240, p1241));
    {
        let mut m1238 = m1238;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1238;
                let m1232 = v0;
                if m1232 == 0 {
                    Err::<(u64, (List<Adt20>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>(Ok(Err::<List<Adt13>, Bytes>(f8(Str::lit("the input has more definitions than tokens")))))
                } else {
                    let v3 = m1232 - 1;
                    let m1233 = v1.clone();
                    match m1233.uncons() {
                        None => {
                            Err::<(u64, (List<Adt20>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>(Ok(Ok::<List<Adt13>, Bytes>(f265(v2, List::<Adt13>::nil()))))
                        }
                        Some((_, _)) => {
                            let m1234 = f266(v1)?;
                            match m1234 {
                                Ok(v7) => {
                                    let m1235 = f267(v3, v7)?;
                                    match m1235 {
                                        Ok(v9) => {
                                            Ok::<(u64, (List<Adt20>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>((v3, ({
                                                let (_, h1236) = v9;
                                                h1236
                                            }, List::cons({
                                                let (h1237, _) = v9;
                                                h1237
                                            }, v2))))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (List<Adt20>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>(Ok(Err::<List<Adt13>, Bytes>(v8)))
                                        }
                                    }
                                }
                                Err(v6) => {
                                    Err::<(u64, (List<Adt20>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>(Ok(Err::<List<Adt13>, Bytes>(v6)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1238 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f189(v0: List<Adt16>, v1: List<Adt16>) -> List<Adt16> {
    let m1242 = v0;
    match m1242.uncons() {
        None => {
            List::<Adt16>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = if f32(f196(v2.clone()), Bytes::lit(&[])) {
                f268(f198(v2.clone()), v1.clone())
            } else {
                f196(v2.clone())
            };
            List::cons(Adt16::C0(f199(v2.clone()), v4, f200(v2.clone()), f198(v2)), f189(v3, v1))
        }
    }
}

pub fn f190(v0: List<Adt16>) -> R<u64> {
    let m1243 = v0;
    match m1243.uncons() {
        None => {
            Ok(0u64)
        }
        Some((v1, v2)) => {
            if f32(f196(v1), Bytes::lit(&[])) {
                f190(v2)
            } else {
                let a1244 = f190(v2)?;
                let a1245 = 1u64;
                nat_add(a1244, a1245)
            }
        }
    }
}

pub fn f191(v0: u64) -> bool {
    let a1246 = v0;
    let a1247 = 0u64;
    if nat_eq(a1246, a1247) {
        true
    } else {
        let a1248 = v0;
        let a1249 = 2u64;
        if nat_eq(a1248, a1249) {
            true
        } else {
            let a1250 = v0;
            let a1251 = 3u64;
            if nat_eq(a1250, a1251) {
                true
            } else {
                let a1252 = v0;
                let a1253 = 4u64;
                if nat_eq(a1252, a1253) {
                    true
                } else {
                    let a1254 = v0;
                    let a1255 = 9u64;
                    nat_eq(a1254, a1255)
                }
            }
        }
    }
}

pub fn f192(v0: Adt13) -> u64 {
    let m1256 = v0;
    match m1256 {
        Adt13::C0(v1, _, _, _, _, _) => {
            v1
        }
    }
}

pub fn f193(v0: List<Adt13>, v1: Adt13) -> R<Result<(Bytes, List<Bytes>), Bytes>> {
    let v2: u64 = f192(v1.clone());
    let v3: Bytes = f269(f194(v1.clone()))?;
    let a1257 = v2;
    let a1258 = 0u64;
    if if nat_eq(a1257, a1258) {
        true
    } else {
        let a1259 = v2;
        let a1260 = 2u64;
        nat_eq(a1259, a1260)
    } {
        let m1261 = f270(v0.clone(), v0.clone(), f194(v1.clone()));
        match m1261 {
            None => {
                let m1265 = f271({
                    let a1263 = {
                        let a1262 = v0.clone();
                        length_list(a1262)
                    };
                    let a1264 = 1u64;
                    nat_add(a1263, a1264)?
                }, v0, v1)?;
                match m1265 {
                    Ok(v5) => {
                        let a1267 = {
                            let a1266 = v5.clone();
                            length_list(a1266)
                        };
                        let a1268 = 0u64;
                        if nat_eq(a1267, a1268) {
                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f272(v3), List::<Bytes>::nil())))
                        } else {
                            let m1269 = f273(f274(f275(v5.clone()))?, v5)?;
                            match m1269 {
                                Ok(v7) => {
                                    Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f276(v3, {
                                        let (h1270, _) = v7;
                                        h1270
                                    }), {
                                        let (_, h1271) = v7;
                                        h1271
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
        let a1272 = v2;
        let a1273 = 3u64;
        if nat_eq(a1272, a1273) {
            let m1274 = f277(f278(v1.clone()));
            match m1274 {
                None => {
                    let m1275 = f279(f274(f278(v1.clone()))?, f280(f278(v1)), List::<Bytes>::nil())?;
                    match m1275 {
                        Ok(v10) => {
                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f281(v3, v10), List::<Bytes>::nil())))
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
            let a1276 = v2;
            let a1277 = 4u64;
            if nat_eq(a1276, a1277) {
                let v12: Adt15 = f282(v1);
                if !f32(f283(v12.clone()), Bytes::lit(&[])) {
                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f283(v12)))
                } else {
                    let a1278 = f284(v12.clone());
                    let a1279 = 1u64;
                    if nat_eq(a1278, a1279) {
                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f281(v3, List::cons(f285(v12.clone()), List::<Bytes>::nil())), f286(v12))))
                    } else {
                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f287(v3, f285(v12.clone())), f286(v12))))
                    }
                }
            } else {
                Ok(Err::<(Bytes, List<Bytes>), Bytes>(f7(f278(v1))))
            }
        }
    }
}

pub fn f194(v0: Adt13) -> Bytes {
    let m1280 = v0;
    match m1280 {
        Adt13::C0(_, v2, _, _, _, _) => {
            v2
        }
    }
}

pub fn f195(p1283: List<Adt16>, v1: Bytes) -> Option<Adt16> {
    let m1282 = p1283;
    {
        let mut m1282 = m1282;
        loop {
            let step = {
                let v0 = m1282;
                let m1281 = v0;
                match m1281.uncons() {
                    None => {
                        Err::<List<Adt16>, Option<Adt16>>(None::<Adt16>)
                    }
                    Some((v2, v3)) => {
                        if f32(f199(v2.clone()), v1.clone()) {
                            Err::<List<Adt16>, Option<Adt16>>(Some(v2))
                        } else {
                            Ok::<List<Adt16>, Option<Adt16>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1282 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f196(v0: Adt16) -> Bytes {
    let m1284 = v0;
    match m1284 {
        Adt16::C0(_, v2, _, _) => {
            v2
        }
    }
}

pub fn f197(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m1285 = v0;
    match m1285.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f197(v3, v1))
        }
    }
}

pub fn f198(v0: Adt16) -> List<Bytes> {
    let m1286 = v0;
    match m1286 {
        Adt16::C0(_, _, _, v4) => {
            v4
        }
    }
}

pub fn f199(v0: Adt16) -> Bytes {
    let m1287 = v0;
    match m1287 {
        Adt16::C0(v1, _, _, _) => {
            v1
        }
    }
}

pub fn f200(v0: Adt16) -> Bytes {
    let m1288 = v0;
    match m1288 {
        Adt16::C0(_, _, v3, _) => {
            v3
        }
    }
}

pub fn f201(v0: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)> {
    let m1289 = v0;
    match m1289.uncons() {
        None => {
            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
        }
        Some((v2, v3)) => {
            let m1292 = f288({
                let (h1290, _) = {
                    let (_, h1291) = v2.clone();
                    h1291
                };
                h1290
            }, v1.clone());
            match m1292 {
                None => {
                    Some((({
                        let (h1293, _) = v2;
                        h1293
                    }, {
                        let (_, h1294) = {
                            let (_, h1295) = v2;
                            h1295
                        };
                        h1294
                    }), v3))
                }
                Some(_) => {
                    let m1296 = f201(v3, v1);
                    match m1296 {
                        None => {
                            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h1297, _) = v5;
                                h1297
                            }, List::cons(v2, {
                                let (_, h1298) = v5;
                                h1298
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f202(p1303: List<Adt13>, v1: u64, v2: Bytes) -> Option<Adt13> {
    let m1302 = p1303;
    {
        let mut m1302 = m1302;
        loop {
            let step = {
                let v0 = m1302;
                let m1299 = v0;
                match m1299.uncons() {
                    None => {
                        Err::<List<Adt13>, Option<Adt13>>(None::<Adt13>)
                    }
                    Some((v3, v4)) => {
                        let a1300 = f192(v3.clone());
                        let a1301 = v1;
                        if if nat_eq(a1300, a1301) {
                            f32(f194(v3.clone()), v2.clone())
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
                Ok(next) => { m1302 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f203(v0: u64) -> Bytes {
    let a1304 = v0;
    let a1305 = 5u64;
    if nat_eq(a1304, a1305) {
        f8(Str::lit("partial interface "))
    } else {
        let a1306 = v0;
        let a1307 = 7u64;
        if nat_eq(a1306, a1307) {
            f8(Str::lit("partial dictionary "))
        } else {
            f8(Str::lit(""))
        }
    }
}

pub fn f204(v0: Adt13) -> Bytes {
    let m1308 = v0;
    match m1308 {
        Adt13::C0(_, _, v3, _, _, _) => {
            v3
        }
    }
}

pub fn f205(v0: Adt2) -> Option<Bytes> {
    let m1309 = v0;
    match m1309 {
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
        Adt2::C7(r1312) => {
            let v3 = (*r1312).clone();
            f205(v3)
        }
        Adt2::C8(r1313) => {
            let v4 = (*r1313).clone();
            f205(v4)
        }
        Adt2::C9 => {
            None::<Bytes>
        }
        Adt2::C10(r1314, r1315) => {
            let v5 = (*r1314).clone();
            let v6 = (*r1315).clone();
            let m1310 = f205(v5);
            match m1310 {
                None => {
                    f205(v6)
                }
                Some(v7) => {
                    Some(v7)
                }
            }
        }
        Adt2::C11(_, r1316, r1317) => {
            let v9 = (*r1316).clone();
            let v10 = (*r1317).clone();
            let m1311 = f205(v9);
            match m1311 {
                None => {
                    f205(v10)
                }
                Some(v11) => {
                    Some(v11)
                }
            }
        }
    }
}

pub fn f206(v0: Adt2) -> R<Option<Bytes>> {
    let m1318 = v0;
    match m1318 {
        Adt2::C0 => {
            Ok(None::<Bytes>)
        }
        Adt2::C1(_, r1321) => {
            let v2 = (*r1321).clone();
            f206(v2)
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
            if f289(v18, v19)? {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(f225(v18, v19)?))
            }
        }
        Adt2::C6(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C7(r1322) => {
            let v3 = (*r1322).clone();
            f206(v3)
        }
        Adt2::C8(r1323) => {
            let v4 = (*r1323).clone();
            f206(v4)
        }
        Adt2::C9 => {
            Ok(None::<Bytes>)
        }
        Adt2::C10(r1324, r1325) => {
            let v5 = (*r1324).clone();
            let v6 = (*r1325).clone();
            let m1319 = f206(v5)?;
            match m1319 {
                None => {
                    f206(v6)
                }
                Some(v7) => {
                    Ok(Some(v7))
                }
            }
        }
        Adt2::C11(_, r1326, r1327) => {
            let v9 = (*r1326).clone();
            let v10 = (*r1327).clone();
            let m1320 = f206(v9)?;
            match m1320 {
                None => {
                    f206(v10)
                }
                Some(v11) => {
                    Ok(Some(v11))
                }
            }
        }
    }
}

pub fn f207(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f103(v0.clone(), {
        let a1328 = v1;
        let a1329 = 1u64;
        nat_add(a1328, a1329)?
    })?;
    let v4: Bytes = {
        let a1330 = v3;
        let a1331 = 117u64;
        if nat_eq(a1330, a1331) {
            f290(v0, v1)?
        } else {
            f291(v3)
        }
    };
    let a1332 = v4.clone();
    let a1333 = Bytes::lit(&[]);
    if equal(a1332, a1333) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a1334 = v3;
            let a1335 = 117u64;
            if nat_eq(a1334, a1335) {
                let a1336 = v1;
                let a1337 = 6u64;
                nat_add(a1336, a1337)?
            } else {
                let a1338 = v1;
                let a1339 = 2u64;
                nat_add(a1338, a1339)?
            }
        }, ({
            let a1340 = v2;
            let a1341 = v4;
            append_bytes(a1340, a1341)
        }, 0u64))))
    }
}

pub fn f208(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f292(v0, v1)
}

pub fn f209(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f293(v0, v1)
}

pub fn f210(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1342 = v0.clone();
        length_bytes(a1342)
    };
    let a1343 = 0u64;
    let a1344 = v1;
    if nat_lt(a1343, a1344) {
        let a1345 = f294(v0.clone())?;
        let a1346 = v1;
        if nat_eq(a1345, a1346) {
            let a1347 = 1u64;
            let a1348 = v1;
            if nat_lt(a1347, a1348) {
                let m1351 = {
                    let a1349 = v0;
                    let a1350 = 0u64;
                    index_bytes(a1349, a1350)
                };
                match m1351 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1352 = v2;
                        let a1353 = 48u8;
                        Ok(!equal(a1352, a1353))
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

pub fn f211(v0: Fn6, p1358: u64, p1359: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1357 = (p1358, p1359);
    {
        let mut m1357 = m1357;
        loop {
            let step = {
                let (v1, v2) = m1357;
                let m1354 = v1;
                if m1354 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1354 - 1;
                    let m1356 = {
                        let c1355: Fn6 = v0.clone();
                        c1355.apply(v2)?
                    };
                    match m1356 {
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
                Ok(next) => { m1357 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f212(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f295(v0, v1)
}

pub fn f213(v0: Bytes) -> R<u64> {
    let (h1360, _) = f176(Fn5::F296(v0.clone()), {
        let a1362 = {
            let a1361 = v0;
            length_bytes(a1361)
        };
        let a1363 = 1u64;
        nat_add(a1362, a1363)?
    }, 0u64)?;
    Ok(h1360)
}

pub fn f214(v0: u64) -> Bytes {
    let m1364 = v0;
    if m1364 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1364 - 1;
        let a1365 = f8(Str::lit("0"));
        let a1366 = f214(v1);
        append_bytes(a1365, a1366)
    }
}

pub fn f215(v0: u32) -> Bytes {
    let a1369 = {
        let a1367 = v0;
        let a1368 = 8u32;
        fixed_u32::bit_and(a1367, a1368)
    };
    let a1370 = 0u32;
    if equal(a1369, a1370) {
        let a1373 = {
            let a1371 = v0;
            let a1372 = 4u32;
            fixed_u32::bit_and(a1371, a1372)
        };
        let a1374 = 0u32;
        if equal(a1373, a1374) {
            let a1377 = {
                let a1375 = v0;
                let a1376 = 2u32;
                fixed_u32::bit_and(a1375, a1376)
            };
            let a1378 = 0u32;
            if equal(a1377, a1378) {
                let a1381 = {
                    let a1379 = v0;
                    let a1380 = 1u32;
                    fixed_u32::bit_and(a1379, a1380)
                };
                let a1382 = 0u32;
                if equal(a1381, a1382) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a1385 = {
                    let a1383 = v0;
                    let a1384 = 1u32;
                    fixed_u32::bit_and(a1383, a1384)
                };
                let a1386 = 0u32;
                if equal(a1385, a1386) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a1389 = {
                let a1387 = v0;
                let a1388 = 2u32;
                fixed_u32::bit_and(a1387, a1388)
            };
            let a1390 = 0u32;
            if equal(a1389, a1390) {
                let a1393 = {
                    let a1391 = v0;
                    let a1392 = 1u32;
                    fixed_u32::bit_and(a1391, a1392)
                };
                let a1394 = 0u32;
                if equal(a1393, a1394) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a1397 = {
                    let a1395 = v0;
                    let a1396 = 1u32;
                    fixed_u32::bit_and(a1395, a1396)
                };
                let a1398 = 0u32;
                if equal(a1397, a1398) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a1401 = {
            let a1399 = v0;
            let a1400 = 4u32;
            fixed_u32::bit_and(a1399, a1400)
        };
        let a1402 = 0u32;
        if equal(a1401, a1402) {
            let a1405 = {
                let a1403 = v0;
                let a1404 = 2u32;
                fixed_u32::bit_and(a1403, a1404)
            };
            let a1406 = 0u32;
            if equal(a1405, a1406) {
                let a1409 = {
                    let a1407 = v0;
                    let a1408 = 1u32;
                    fixed_u32::bit_and(a1407, a1408)
                };
                let a1410 = 0u32;
                if equal(a1409, a1410) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a1413 = {
                    let a1411 = v0;
                    let a1412 = 1u32;
                    fixed_u32::bit_and(a1411, a1412)
                };
                let a1414 = 0u32;
                if equal(a1413, a1414) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a1417 = {
                let a1415 = v0;
                let a1416 = 2u32;
                fixed_u32::bit_and(a1415, a1416)
            };
            let a1418 = 0u32;
            if equal(a1417, a1418) {
                let a1421 = {
                    let a1419 = v0;
                    let a1420 = 1u32;
                    fixed_u32::bit_and(a1419, a1420)
                };
                let a1422 = 0u32;
                if equal(a1421, a1422) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a1425 = {
                    let a1423 = v0;
                    let a1424 = 1u32;
                    fixed_u32::bit_and(a1423, a1424)
                };
                let a1426 = 0u32;
                if equal(a1425, a1426) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f216(v0: u32, v1: u32) -> u32 {
    let m1429 = {
        let a1427 = v0;
        let a1428 = v1;
        fixed_u32::shift_right(a1427, a1428)
    };
    match m1429 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f217(v0: Adt17, v1: Adt21) -> R<Adt17> {
    let v2: Adt22 = f297(Fn7::F298, 64u64, Adt22::C0(match v0.clone() {
        Adt17::C0(h1430, _, _, _, _, _, _, _) => {
            h1430
        }
    }, match v0.clone() {
        Adt17::C0(_, h1431, _, _, _, _, _, _) => {
            h1431
        }
    }, match v0.clone() {
        Adt17::C0(_, _, h1432, _, _, _, _, _) => {
            h1432
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, h1433, _, _, _, _) => {
            h1433
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, h1434, _, _, _) => {
            h1434
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, h1435, _, _) => {
            h1435
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, h1436, _) => {
            h1436
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, _, h1437) => {
            h1437
        }
    }, v1, 0u64))?;
    Ok(Adt17::C0(f299(match v0.clone() {
        Adt17::C0(h1438, _, _, _, _, _, _, _) => {
            h1438
        }
    }, match v2.clone() {
        Adt22::C0(h1439, _, _, _, _, _, _, _, _, _) => {
            h1439
        }
    }), f299(match v0.clone() {
        Adt17::C0(_, h1440, _, _, _, _, _, _) => {
            h1440
        }
    }, match v2.clone() {
        Adt22::C0(_, h1441, _, _, _, _, _, _, _, _) => {
            h1441
        }
    }), f299(match v0.clone() {
        Adt17::C0(_, _, h1442, _, _, _, _, _) => {
            h1442
        }
    }, match v2.clone() {
        Adt22::C0(_, _, h1443, _, _, _, _, _, _, _) => {
            h1443
        }
    }), f299(match v0.clone() {
        Adt17::C0(_, _, _, h1444, _, _, _, _) => {
            h1444
        }
    }, match v2.clone() {
        Adt22::C0(_, _, _, h1445, _, _, _, _, _, _) => {
            h1445
        }
    }), f299(match v0.clone() {
        Adt17::C0(_, _, _, _, h1446, _, _, _) => {
            h1446
        }
    }, match v2.clone() {
        Adt22::C0(_, _, _, _, h1447, _, _, _, _, _) => {
            h1447
        }
    }), f299(match v0.clone() {
        Adt17::C0(_, _, _, _, _, h1448, _, _) => {
            h1448
        }
    }, match v2.clone() {
        Adt22::C0(_, _, _, _, _, h1449, _, _, _, _) => {
            h1449
        }
    }), f299(match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, h1450, _) => {
            h1450
        }
    }, match v2.clone() {
        Adt22::C0(_, _, _, _, _, _, h1451, _, _, _) => {
            h1451
        }
    }), f299(match v0 {
        Adt17::C0(_, _, _, _, _, _, _, h1452) => {
            h1452
        }
    }, match v2 {
        Adt22::C0(_, _, _, _, _, _, _, h1453, _, _) => {
            h1453
        }
    })))
}

pub fn f218(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt21> {
    Ok(Adt21::C0(f300(v0.clone(), v1, v2, v3)?, f300(v0.clone(), v1, v2, {
        let a1454 = v3;
        let a1455 = 4u64;
        nat_add(a1454, a1455)?
    })?, f300(v0.clone(), v1, v2, {
        let a1456 = v3;
        let a1457 = 8u64;
        nat_add(a1456, a1457)?
    })?, f300(v0.clone(), v1, v2, {
        let a1458 = v3;
        let a1459 = 12u64;
        nat_add(a1458, a1459)?
    })?, f300(v0.clone(), v1, v2, {
        let a1460 = v3;
        let a1461 = 16u64;
        nat_add(a1460, a1461)?
    })?, f300(v0.clone(), v1, v2, {
        let a1462 = v3;
        let a1463 = 20u64;
        nat_add(a1462, a1463)?
    })?, f300(v0.clone(), v1, v2, {
        let a1464 = v3;
        let a1465 = 24u64;
        nat_add(a1464, a1465)?
    })?, f300(v0.clone(), v1, v2, {
        let a1466 = v3;
        let a1467 = 28u64;
        nat_add(a1466, a1467)?
    })?, f300(v0.clone(), v1, v2, {
        let a1468 = v3;
        let a1469 = 32u64;
        nat_add(a1468, a1469)?
    })?, f300(v0.clone(), v1, v2, {
        let a1470 = v3;
        let a1471 = 36u64;
        nat_add(a1470, a1471)?
    })?, f300(v0.clone(), v1, v2, {
        let a1472 = v3;
        let a1473 = 40u64;
        nat_add(a1472, a1473)?
    })?, f300(v0.clone(), v1, v2, {
        let a1474 = v3;
        let a1475 = 44u64;
        nat_add(a1474, a1475)?
    })?, f300(v0.clone(), v1, v2, {
        let a1476 = v3;
        let a1477 = 48u64;
        nat_add(a1476, a1477)?
    })?, f300(v0.clone(), v1, v2, {
        let a1478 = v3;
        let a1479 = 52u64;
        nat_add(a1478, a1479)?
    })?, f300(v0.clone(), v1, v2, {
        let a1480 = v3;
        let a1481 = 56u64;
        nat_add(a1480, a1481)?
    })?, f300(v0, v1, v2, {
        let a1482 = v3;
        let a1483 = 60u64;
        nat_add(a1482, a1483)?
    })?))
}

pub fn f219(v0: u64) -> bool {
    f155({
        let a1484 = v0;
        let a1485 = 95u64;
        nat_eq(a1484, a1485)
    }, f155({
        let a1486 = v0;
        let a1487 = 45u64;
        nat_eq(a1486, a1487)
    }, f155({
        let a1488 = v0;
        let a1489 = 32u64;
        nat_eq(a1488, a1489)
    }, f155({
        let a1490 = v0;
        let a1491 = 46u64;
        nat_eq(a1490, a1491)
    }, {
        let a1492 = v0;
        let a1493 = 47u64;
        nat_eq(a1492, a1493)
    }))))
}

pub fn f220(v0: u64) -> bool {
    f105({
        let a1494 = 97u64;
        let a1495 = v0;
        nat_le(a1494, a1495)
    }, {
        let a1496 = v0;
        let a1497 = 122u64;
        nat_le(a1496, a1497)
    })
}

pub fn f221(v0: u64) -> bool {
    f105({
        let a1498 = 65u64;
        let a1499 = v0;
        nat_le(a1498, a1499)
    }, {
        let a1500 = v0;
        let a1501 = 90u64;
        nat_le(a1500, a1501)
    })
}

pub fn f222(v0: u64) -> bool {
    f105({
        let a1502 = 48u64;
        let a1503 = v0;
        nat_le(a1502, a1503)
    }, {
        let a1504 = v0;
        let a1505 = 57u64;
        nat_le(a1504, a1505)
    })
}

pub fn f223(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1506 = 0u64;
        let a1507 = v1;
        nat_lt(a1506, a1507)
    } else {
        false
    } {
        let a1508 = Bytes::lit(&[0x2d]);
        let a1509 = f89(v1)?;
        Ok(append_bytes(a1508, a1509))
    } else {
        f89(v1)
    }
}

pub fn f224(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f89(v1)?;
    let v4: u64 = {
        let a1510 = v3.clone();
        length_bytes(a1510)
    };
    let v5: Bytes = {
        let a1511 = v2;
        let a1512 = v4;
        if nat_lt(a1511, a1512) {
            f7(List::cons(f46(v3.clone(), 0u64, {
                let a1513 = v4;
                let a1514 = v2;
                nat_sub(a1513, a1514)
            }), List::cons(f8(Str::lit(".")), List::cons(f46(v3, {
                let a1515 = v4;
                let a1516 = v2;
                nat_sub(a1515, a1516)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f214({
                let a1517 = v2;
                let a1518 = v4;
                nat_sub(a1517, a1518)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1519 = f8(Str::lit("-"));
        let a1520 = v5;
        Ok(append_bytes(a1519, a1520))
    } else {
        Ok(v5)
    }
}

pub fn f225(v0: bool, v1: u64) -> R<Bytes> {
    let v2: u64 = f301(v0)?;
    let v3: bool = {
        let a1521 = f302(v0)?;
        let a1522 = v1;
        nat_le(a1521, a1522)
    };
    let v4: u64 = {
        let a1526 = {
            let a1523 = v1;
            let a1524 = v2;
            let a1525 = 0u64;
            nat_quot(a1523, a1524, a1525)
        };
        let a1527 = f303(f304(v0))?;
        let a1528 = 0u64;
        nat_rem(a1526, a1527, a1528)
    };
    let v5: u64 = {
        let a1529 = v1;
        let a1530 = v2;
        let a1531 = 0u64;
        nat_rem(a1529, a1530, a1531)
    };
    let a1532 = v4;
    let a1533 = f305(v0)?;
    if nat_eq(a1532, a1533) {
        let a1534 = v5;
        let a1535 = 0u64;
        if nat_eq(a1534, a1535) {
            Ok(f306(v3, f8(Str::lit("inf"))))
        } else {
            Ok(f306(v3, {
                let a1539 = {
                    let a1536 = v2;
                    let a1537 = 2u64;
                    let a1538 = 0u64;
                    nat_quot(a1536, a1537, a1538)
                };
                let a1540 = v5;
                if nat_le(a1539, a1540) {
                    f8(Str::lit("nan"))
                } else {
                    f8(Str::lit("snan"))
                }
            }))
        }
    } else {
        if f105({
            let a1541 = v4;
            let a1542 = 0u64;
            nat_eq(a1541, a1542)
        }, {
            let a1543 = v5;
            let a1544 = 0u64;
            nat_eq(a1543, a1544)
        }) {
            Ok(f306(v3, Bytes::lit(&[0x30])))
        } else {
            let v6: u64 = {
                let a1545 = v4;
                let a1546 = 0u64;
                if nat_eq(a1545, a1546) {
                    v5
                } else {
                    let a1547 = v2;
                    let a1548 = v5;
                    nat_add(a1547, a1548)?
                }
            };
            let v7: u64 = {
                let a1549 = v4;
                let a1550 = 0u64;
                if nat_eq(a1549, a1550) {
                    1u64
                } else {
                    v4
                }
            };
            let v8: u64 = {
                let a1557 = {
                    let a1553 = {
                        let a1551 = v7;
                        let a1552 = 2000u64;
                        nat_add(a1551, a1552)?
                    };
                    let a1554 = f307(v0);
                    nat_sub(a1553, a1554)
                };
                let a1558 = {
                    let a1555 = f308(v0);
                    let a1556 = 1u64;
                    nat_sub(a1555, a1556)
                };
                nat_sub(a1557, a1558)
            };
            Ok(f306(v3, {
                let a1559 = 2000u64;
                let a1560 = v8;
                if nat_le(a1559, a1560) {
                    f309(v6, true, {
                        let a1561 = v8;
                        let a1562 = 2000u64;
                        nat_sub(a1561, a1562)
                    })?
                } else {
                    f309(v6, false, {
                        let a1563 = 2000u64;
                        let a1564 = v8;
                        nat_sub(a1563, a1564)
                    })?
                }
            }))
        }
    }
}

pub fn f226(v0: Bytes) -> R<Bytes> {
    let a1569 = f152(v0.clone(), 0u64, {
        let a1566 = {
            let a1565 = v0.clone();
            length_bytes(a1565)
        };
        let a1567 = 1u64;
        nat_add(a1566, a1567)?
    })?;
    let a1570 = {
        let a1568 = v0.clone();
        length_bytes(a1568)
    };
    if nat_eq(a1569, a1570) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        let a1576 = {
            let a1574 = Bytes::lit(&[0x22]);
            let a1575 = f310(v0.clone(), 0u64, {
                let a1572 = {
                    let a1571 = v0;
                    length_bytes(a1571)
                };
                let a1573 = 1u64;
                nat_add(a1572, a1573)?
            }, Bytes::lit(&[]))?;
            append_bytes(a1574, a1575)
        };
        let a1577 = Bytes::lit(&[0x22]);
        Ok(append_bytes(a1576, a1577))
    }
}

pub fn f227(v0: List<Bytes>) -> Bytes {
    let a1580 = {
        let a1578 = Bytes::lit(&[0x5b]);
        let a1579 = f47(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1578, a1579)
    };
    let a1581 = Bytes::lit(&[0x5d]);
    append_bytes(a1580, a1581)
}

pub fn f228(v0: Adt2) -> R<List<Bytes>> {
    let m1582 = v0;
    match m1582 {
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
        Adt2::C10(r1583, r1584) => {
            let v14 = (*r1583).clone();
            let v15 = (*r1584).clone();
            Ok(List::cons(f168(v14)?, f228(v15)?))
        }
        Adt2::C11(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f229(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f311(Fn8::F312, List::<(Bytes, Bytes)>::nil(), f313(v0)?);
    let v2: List<Bytes> = f314(Fn9::F315, List::<Bytes>::nil(), v1)?;
    let a1587 = {
        let a1585 = Bytes::lit(&[0x7b]);
        let a1586 = f47(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1585, a1586)
    };
    let a1588 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1587, a1588))
}

pub fn f230(v0: Adt2) -> R<List<(Bytes, Bytes)>> {
    let m1589 = v0;
    match m1589 {
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
        Adt2::C11(v16, r1590, r1591) => {
            let v17 = (*r1590).clone();
            let v18 = (*r1591).clone();
            Ok(List::cons((v16, f168(v17)?), f230(v18)?))
        }
    }
}

pub fn f231(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1592 = v0;
    match m1592.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f316(v1)?, f231(v2)?))
        }
    }
}

pub fn f232(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m1593 = v0;
    match m1593.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f232(v3, false))
            } else {
                f232(v3, true)
            }
        }
    }
}

pub fn f233(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f317(v0, v1)
}

pub fn f234(v0: List<Bytes>) -> List<Bytes> {
    f108(Fn2::F318, List::<Bytes>::nil(), v0)
}

pub fn f235(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f319(v0, v1)
}

pub fn f236(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f320(f321(f322(v0))?))
}

pub fn f237(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f125(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1596 = {
            let a1594 = v4;
            let a1595 = v1;
            nat_add(a1594, a1595)?
        };
        let a1597 = v3;
        if nat_lt(a1596, a1597) {
            Ok(Some({
                let a1598 = v4;
                let a1599 = 1u64;
                nat_add(a1598, a1599)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f238(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h1600, _) = v1;
        h1600
    };
    let a1602 = {
        let a1601 = v0.clone();
        length_bytes(a1601)
    };
    let a1603 = v2;
    if nat_le(a1602, a1603) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let a1604 = f323(v0.clone(), v2)?;
        let a1605 = 13u64;
        if nat_eq(a1604, a1605) {
            Ok(Some(({
                let a1608 = f323(v0, {
                    let a1606 = v2;
                    let a1607 = 1u64;
                    nat_add(a1606, a1607)?
                })?;
                let a1609 = 10u64;
                if nat_eq(a1608, a1609) {
                    let a1610 = v2;
                    let a1611 = 2u64;
                    nat_add(a1610, a1611)?
                } else {
                    let a1612 = v2;
                    let a1613 = 1u64;
                    nat_add(a1612, a1613)?
                }
            }, {
                let a1615 = {
                    let (_, h1614) = v1;
                    h1614
                };
                let a1616 = f8(Str::lit("\u{a}"));
                append_bytes(a1615, a1616)
            })))
        } else {
            let v4: u64 = {
                let m1617 = f324(v0.clone(), f8(Str::lit("\u{d}")), v2)?;
                match m1617 {
                    None => {
                        let a1618 = v0.clone();
                        length_bytes(a1618)
                    }
                    Some(v3) => {
                        v3
                    }
                }
            };
            Ok(Some((v4, {
                let a1622 = {
                    let (_, h1619) = v1;
                    h1619
                };
                let a1623 = f46(v0, v2, {
                    let a1620 = v4;
                    let a1621 = v2;
                    nat_sub(a1620, a1621)
                });
                append_bytes(a1622, a1623)
            })))
        }
    }
}

pub fn f239(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1624 = f325(v0, v1)?;
    match m1624 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f326({
                let (h1625, _) = v2;
                h1625
            }) {
                Ok(Some({
                    let a1627 = v1;
                    let a1628 = {
                        let (_, h1626) = v2;
                        h1626
                    };
                    nat_add(a1627, a1628)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f240(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1630 = v1;
    let a1631 = {
        let a1629 = v0.clone();
        length_bytes(a1629)
    };
    if if nat_lt(a1630, a1631) {
        f327(v0.clone(), v1)?
    } else {
        false
    } {
        Ok(Some(f245(v0, v1)?))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f241(v0: u64) -> bool {
    let a1632 = v0;
    let a1633 = 32u64;
    if nat_eq(a1632, a1633) {
        true
    } else {
        let a1634 = v0;
        let a1635 = 9u64;
        nat_eq(a1634, a1635)
    }
}

pub fn f242(v0: u64) -> bool {
    let a1636 = v0;
    let a1637 = 10u64;
    if nat_eq(a1636, a1637) {
        true
    } else {
        let a1638 = v0;
        let a1639 = 256u64;
        nat_eq(a1638, a1639)
    }
}

pub fn f243(v0: Bytes, v1: u64) -> R<u64> {
    let a1640 = v1;
    let a1641 = f328(v0, v1)?;
    Ok(nat_sub(a1640, a1641))
}

pub fn f244(v0: Bytes, v1: u64) -> R<u64> {
    let (h1642, _) = f176(Fn5::F329(v0.clone()), {
        let a1644 = {
            let a1643 = v0;
            length_bytes(a1643)
        };
        let a1645 = 1u64;
        nat_add(a1644, a1645)?
    }, v1)?;
    Ok(h1642)
}

pub fn f245(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f330(v0.clone(), v1)?;
    let a1647 = v2;
    let a1648 = {
        let a1646 = v0;
        length_bytes(a1646)
    };
    if nat_lt(a1647, a1648) {
        let a1649 = v2;
        let a1650 = 1u64;
        nat_add(a1649, a1650)
    } else {
        Ok(v2)
    }
}

pub fn f246(v0: Bytes, v1: u64) -> R<u64> {
    let (h1651, _) = f176(Fn5::F331(v0.clone()), {
        let a1653 = {
            let a1652 = v0;
            length_bytes(a1652)
        };
        let a1654 = 1u64;
        nat_add(a1653, a1654)?
    }, v1)?;
    Ok(h1651)
}

pub fn f247(v0: Bytes, v1: u64) -> R<bool> {
    let a1655 = f182(v0.clone(), v1)?;
    let a1656 = 45u64;
    if nat_eq(a1655, a1656) {
        if f241(f182(v0.clone(), {
            let a1657 = v1;
            let a1658 = 1u64;
            nat_add(a1657, a1658)?
        })?) {
            Ok(true)
        } else {
            Ok(f242(f182(v0, {
                let a1659 = v1;
                let a1660 = 1u64;
                nat_add(a1659, a1660)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f248(v0: Bytes, v1: u64) -> R<bool> {
    let m1661 = f260(v0, v1)?;
    match m1661 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f249(v0: Bytes, v1: u64) -> R<Option<Bytes>> {
    let v2: u64 = f182(v0.clone(), v1)?;
    let v3: bool = f184(f332(f182(v0, {
        let a1662 = v1;
        let a1663 = 1u64;
        nat_add(a1662, a1663)?
    })?, true));
    let a1664 = v2;
    let a1665 = 58u64;
    if if nat_eq(a1664, a1665) {
        v3
    } else {
        false
    } {
        Ok(Some(f129(Str::lit("a null key is not read: Studio.Value keys are strings (YAML 1.2.2 section 10.3.2)"), v1)?))
    } else {
        let a1666 = v2;
        let a1667 = 38u64;
        if nat_eq(a1666, a1667) {
            Ok(Some(f129(Str::lit("an anchor is not read (YAML 1.2.2 [101])"), v1)?))
        } else {
            let a1668 = v2;
            let a1669 = 42u64;
            if nat_eq(a1668, a1669) {
                Ok(Some(f129(Str::lit("an alias is not read (YAML 1.2.2 [104])"), v1)?))
            } else {
                let a1670 = v2;
                let a1671 = 33u64;
                if nat_eq(a1670, a1671) {
                    Ok(Some(f129(Str::lit("a tag is not read (YAML 1.2.2 [97])"), v1)?))
                } else {
                    let a1672 = v2;
                    let a1673 = 63u64;
                    if if nat_eq(a1672, a1673) {
                        v3
                    } else {
                        false
                    } {
                        Ok(Some(f129(Str::lit("an explicit key is not read (YAML 1.2.2 [143], [189])"), v1)?))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f250(v0: Bytes, v1: u64, v2: u64, v3: Adt18) -> R<Adt19> {
    let v4: u64 = f182(v0.clone(), v1)?;
    let a1674 = v4;
    let a1675 = 124u64;
    if if nat_eq(a1674, a1675) {
        true
    } else {
        let a1676 = v4;
        let a1677 = 62u64;
        nat_eq(a1676, a1677)
    } {
        let m1678 = f333(v0, v1, v2)?;
        match m1678 {
            Adt19::C0(v6, v7) => {
                let m1679 = f72(Some(v6.clone()));
                match m1679 {
                    None => {
                        Ok(Adt19::C0(v6, v7))
                    }
                    Some(v8) => {
                        f334(v3, v8, true, v1, v7)
                    }
                }
            }
            Adt19::C1(v5) => {
                Ok(Adt19::C1(v5))
            }
        }
    } else {
        let a1680 = v4;
        let a1681 = 34u64;
        if if nat_eq(a1680, a1681) {
            true
        } else {
            let a1682 = v4;
            let a1683 = 39u64;
            nat_eq(a1682, a1683)
        } {
            let m1684 = f335(v0.clone(), v1, v2, false)?;
            match m1684 {
                Adt23::C0(v10, v11) => {
                    let m1685 = f181(v0, v11)?;
                    match m1685 {
                        Ok(v13) => {
                            f334(v3, v10, true, v1, v13)
                        }
                        Err(v12) => {
                            Ok(Adt19::C1(v12))
                        }
                    }
                }
                Adt23::C1(v9) => {
                    Ok(Adt19::C1(v9))
                }
            }
        } else {
            let a1686 = v4;
            let a1687 = 91u64;
            if if nat_eq(a1686, a1687) {
                true
            } else {
                let a1688 = v4;
                let a1689 = 123u64;
                nat_eq(a1688, a1689)
            } {
                let m1693 = f336(v0.clone(), v1, v2, {
                    let a1691 = {
                        let a1690 = v0.clone();
                        length_bytes(a1690)
                    };
                    let a1692 = 2u64;
                    nat_add(a1691, a1692)?
                }, 0u64, List::<Bytes>::nil(), v3)?;
                match m1693 {
                    Adt19::C0(v15, v16) => {
                        let a1694 = f182(v0.clone(), f244(v0.clone(), v16)?)?;
                        let a1695 = 58u64;
                        if nat_eq(a1694, a1695) {
                            Ok(Adt19::C1(f129(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [154])"), v1)?))
                        } else {
                            let m1696 = f181(v0, v16)?;
                            match m1696 {
                                Ok(v18) => {
                                    Ok(Adt19::C0(v15, v18))
                                }
                                Err(v17) => {
                                    Ok(Adt19::C1(v17))
                                }
                            }
                        }
                    }
                    Adt19::C1(v14) => {
                        Ok(Adt19::C1(v14))
                    }
                }
            } else {
                let m1697 = f249(v0.clone(), v1)?;
                match m1697 {
                    None => {
                        if f184(f337(v0.clone(), v1, false)?) {
                            Ok(Adt19::C1(f129(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                        } else {
                            let v20: u64 = f338(v0.clone(), v1, false)?;
                            let m1701 = f339(v0.clone(), v20, v2, {
                                let a1699 = {
                                    let a1698 = v0.clone();
                                    length_bytes(a1698)
                                };
                                let a1700 = 2u64;
                                nat_add(a1699, a1700)?
                            }, f258(v0, v1, v20))?;
                            match m1701 {
                                Adt23::C0(v22, v23) => {
                                    f334(v3, v22, false, v1, v23)
                                }
                                Adt23::C1(v21) => {
                                    Ok(Adt19::C1(v21))
                                }
                            }
                        }
                    }
                    Some(v19) => {
                        Ok(Adt19::C1(v19))
                    }
                }
            }
        }
    }
}

pub fn f251(v0: Adt18) -> bool {
    let m1702 = v0;
    match m1702 {
        Adt18::C0(v2) => {
            let a1703 = 0u64;
            let a1704 = v2;
            nat_lt(a1703, a1704)
        }
        Adt18::C1(_) => {
            true
        }
        Adt18::C2(_) => {
            true
        }
    }
}

pub fn f252(v0: Bytes, v1: u64) -> R<Adt18> {
    let v2: u64 = f340(v0.clone(), v1)?;
    let a1707 = f182(v0.clone(), {
        let a1705 = v1;
        let a1706 = 1u64;
        nat_add(a1705, a1706)?
    })?;
    let a1708 = 60u64;
    if nat_eq(a1707, a1708) {
        let a1709 = f182(v0.clone(), v2)?;
        let a1710 = 62u64;
        if nat_eq(a1709, a1710) {
            let v3: Bytes = f258(v0, {
                let a1711 = v1;
                let a1712 = 2u64;
                nat_add(a1711, a1712)?
            }, v2);
            if f179(v3.clone(), 0u64, Str::lit("tag:yaml.org,2002:")) {
                f341(f258(v3.clone(), 18u64, {
                    let a1713 = v3;
                    length_bytes(a1713)
                }), v1)
            } else {
                Ok(Adt18::C1(v3))
            }
        } else {
            Ok(Adt18::C2(f129(Str::lit("a verbatim tag must end with > (YAML 1.2.2 [98])"), v1)?))
        }
    } else {
        let a1716 = v2;
        let a1717 = {
            let a1714 = v1;
            let a1715 = 1u64;
            nat_add(a1714, a1715)?
        };
        if nat_eq(a1716, a1717) {
            Ok(Adt18::C0(8u64))
        } else {
            let a1720 = f182(v0.clone(), {
                let a1718 = v1;
                let a1719 = 1u64;
                nat_add(a1718, a1719)?
            })?;
            let a1721 = 33u64;
            if nat_eq(a1720, a1721) {
                f341(f258(v0, {
                    let a1722 = v1;
                    let a1723 = 2u64;
                    nat_add(a1722, a1723)?
                }, v2), v1)
            } else {
                let a1730 = f257(f258(v0.clone(), {
                    let a1724 = v1;
                    let a1725 = 1u64;
                    nat_add(a1724, a1725)?
                }, v2), 33u64)?;
                let a1731 = {
                    let a1728 = v2;
                    let a1729 = {
                        let a1726 = v1;
                        let a1727 = 1u64;
                        nat_add(a1726, a1727)?
                    };
                    nat_sub(a1728, a1729)
                };
                if nat_lt(a1730, a1731) {
                    Ok(Adt18::C2(f129(Str::lit("a named tag handle is not read: it needs a TAG directive (YAML 1.2.2 [92], [82])"), v1)?))
                } else {
                    Ok(Adt18::C1(f258(v0, v1, v2)))
                }
            }
        }
    }
}

pub fn f253(v0: Adt18) -> Option<Bytes> {
    let m1732 = v0;
    match m1732 {
        Adt18::C0(_) => {
            None::<Bytes>
        }
        Adt18::C1(_) => {
            None::<Bytes>
        }
        Adt18::C2(v1) => {
            Some(v1)
        }
    }
}

pub fn f254(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f340(v0.clone(), v1)?;
    let a1735 = f182(v0.clone(), {
        let a1733 = v1;
        let a1734 = 1u64;
        nat_add(a1733, a1734)?
    })?;
    let a1736 = 60u64;
    if if nat_eq(a1735, a1736) {
        let a1737 = f182(v0, v2)?;
        let a1738 = 62u64;
        nat_eq(a1737, a1738)
    } else {
        false
    } {
        let a1739 = v2;
        let a1740 = 1u64;
        nat_add(a1739, a1740)
    } else {
        Ok(v2)
    }
}

pub fn f255(v0: Adt18, v1: bool, v2: u64, v3: Adt19) -> R<Adt19> {
    let m1741 = v3;
    match m1741 {
        Adt19::C0(v5, v6) => {
            let v7: Adt2 = if v1 {
                Adt2::C7(std::rc::Rc::new(v5))
            } else {
                Adt2::C8(std::rc::Rc::new(v5))
            };
            let m1742 = v0;
            match m1742 {
                Adt18::C0(v10) => {
                    let a1743 = v10;
                    let a1744 = 0u64;
                    if if if nat_eq(a1743, a1744) {
                        true
                    } else {
                        let a1745 = v10;
                        let a1746 = 8u64;
                        nat_eq(a1745, a1746)
                    } {
                        true
                    } else {
                        let a1747 = v10;
                        let a1748 = if v1 {
                            6u64
                        } else {
                            7u64
                        };
                        nat_eq(a1747, a1748)
                    } {
                        Ok(Adt19::C0(v7, v6))
                    } else {
                        Ok(Adt19::C1(f342(v2)?))
                    }
                }
                Adt18::C1(v9) => {
                    Ok(Adt19::C0(Adt2::C1(v9, std::rc::Rc::new(v7)), v6))
                }
                Adt18::C2(v8) => {
                    Ok(Adt19::C1(v8))
                }
            }
        }
        Adt19::C1(v4) => {
            Ok(Adt19::C1(v4))
        }
    }
}

pub fn f256(v0: Adt18, v1: u64, v2: u64) -> R<Adt19> {
    let m1749 = v0;
    match m1749 {
        Adt18::C0(v5) => {
            let a1750 = v5;
            let a1751 = 1u64;
            if nat_eq(a1750, a1751) {
                Ok(Adt19::C0(Adt2::C6(Bytes::lit(&[])), v2))
            } else {
                let a1752 = v5;
                let a1753 = 8u64;
                if nat_eq(a1752, a1753) {
                    Ok(Adt19::C1(f129(Str::lit("an empty node with the non-specific tag ! is not read (YAML 1.2.2 [100])"), v1)?))
                } else {
                    let a1754 = v5;
                    let a1755 = 0u64;
                    if if nat_eq(a1754, a1755) {
                        true
                    } else {
                        let a1756 = v5;
                        let a1757 = 5u64;
                        nat_eq(a1756, a1757)
                    } {
                        Ok(Adt19::C0(Adt2::C0, v2))
                    } else {
                        Ok(Adt19::C1(f342(v1)?))
                    }
                }
            }
        }
        Adt18::C1(v4) => {
            Ok(Adt19::C0(Adt2::C1(v4, std::rc::Rc::new(Adt2::C0)), v2))
        }
        Adt18::C2(v3) => {
            Ok(Adt19::C1(v3))
        }
    }
}

pub fn f257(v0: Bytes, v1: u64) -> R<u64> {
    let (h1758, _) = f176(Fn5::F343(v0.clone(), v1), {
        let a1760 = {
            let a1759 = v0;
            length_bytes(a1759)
        };
        let a1761 = 1u64;
        nat_add(a1760, a1761)?
    }, 0u64)?;
    Ok(h1758)
}

pub fn f258(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    f46(v0, v1, {
        let a1762 = v2;
        let a1763 = v1;
        nat_sub(a1762, a1763)
    })
}

pub fn f259(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let a1764 = f182(v0.clone(), v1)?;
    let a1765 = 33u64;
    if nat_eq(a1764, a1765) {
        let m1766 = f252(v0.clone(), v1)?;
        match m1766 {
            Adt18::C0(v4) => {
                let a1767 = v4;
                let a1768 = 1u64;
                if if nat_eq(a1767, a1768) {
                    true
                } else {
                    let a1769 = v4;
                    let a1770 = 8u64;
                    nat_eq(a1769, a1770)
                } {
                    Ok(Ok::<u64, Bytes>(f244(v0.clone(), f254(v0, v1)?)?))
                } else {
                    Ok(Err::<u64, Bytes>(f129(Str::lit("a mapping key tagged other than !!str is not read: Studio.Value keys are strings (YAML 1.2.2 section 10.3.2)"), v1)?))
                }
            }
            Adt18::C1(_) => {
                Ok(Err::<u64, Bytes>(f129(Str::lit("a mapping key tagged other than !!str is not read: Studio.Value keys are strings (YAML 1.2.2 section 10.3.2)"), v1)?))
            }
            Adt18::C2(v2) => {
                Ok(Err::<u64, Bytes>(v2))
            }
        }
    } else {
        Ok(Ok::<u64, Bytes>(v1))
    }
}

pub fn f260(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a1771 = f182(v0.clone(), v1)?;
        let a1772 = 33u64;
        if nat_eq(a1771, a1772) {
            f244(v0.clone(), f254(v0.clone(), v1)?)?
        } else {
            v1
        }
    };
    let v3: u64 = f182(v0.clone(), v2)?;
    let a1773 = v3;
    let a1774 = 34u64;
    if if nat_eq(a1773, a1774) {
        true
    } else {
        let a1775 = v3;
        let a1776 = 39u64;
        nat_eq(a1775, a1776)
    } {
        let m1777 = f335(v0.clone(), v2, 0u64, true)?;
        match m1777 {
            Adt23::C0(_, v6) => {
                let v7: u64 = f244(v0.clone(), v6)?;
                if f344(v0, v7)? {
                    Ok(Some(v7))
                } else {
                    Ok(None::<u64>)
                }
            }
            Adt23::C1(_) => {
                Ok(None::<u64>)
            }
        }
    } else {
        if f337(v0.clone(), v2, false)? {
            let v8: u64 = f244(v0.clone(), f338(v0.clone(), v2, false)?)?;
            if f344(v0, v8)? {
                Ok(Some(v8))
            } else {
                Ok(None::<u64>)
            }
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f261(v0: Bytes, v1: u64) -> R<Bytes> {
    let v2: u64 = f182(v0.clone(), v1)?;
    let a1778 = v2;
    let a1779 = 34u64;
    if if nat_eq(a1778, a1779) {
        true
    } else {
        let a1780 = v2;
        let a1781 = 39u64;
        nat_eq(a1780, a1781)
    } {
        let m1782 = f335(v0, v1, 0u64, true)?;
        match m1782 {
            Adt23::C0(v4, _) => {
                Ok(v4)
            }
            Adt23::C1(_) => {
                Ok(Bytes::lit(&[]))
            }
        }
    } else {
        Ok(f258(v0.clone(), v1, f338(v0, v1, false)?))
    }
}

pub fn f262(v0: Bytes, p1785: List<Bytes>) -> bool {
    let m1784 = p1785;
    {
        let mut m1784 = m1784;
        loop {
            let step = {
                let v1 = m1784;
                let m1783 = v1;
                match m1783.uncons() {
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
                Ok(next) => { m1784 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f263(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h1786, _) = v2;
        h1786
    };
    let a1787 = v3;
    let a1788 = 0u64;
    if nat_eq(a1787, a1788) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a1789 = v3;
            let a1790 = v1;
            let a1791 = 0u64;
            nat_quot(a1789, a1790, a1791)
        }, {
            let a1796 = f185(v0, {
                let a1792 = v3;
                let a1793 = v1;
                let a1794 = 0u64;
                nat_rem(a1792, a1793, a1794)
            });
            let a1797 = {
                let (_, h1795) = v2;
                h1795
            };
            append_bytes(a1796, a1797)
        }))
    }
}

pub fn f264(p1852: u64, v1: Bytes, p1853: u64, p1854: List<Adt20>) -> R<Result<List<Adt20>, Bytes>> {
    let m1851 = (p1852, (p1853, p1854));
    {
        let mut m1851 = m1851;
        loop {
            let step = {
                let (v0, (v2, v3)) = m1851;
                let m1798 = v0;
                if m1798 == 0 {
                    Err::<(u64, (u64, List<Adt20>)), R<Result<List<Adt20>, Bytes>>>(Ok(Err::<List<Adt20>, Bytes>(f345(Str::lit("the input has more tokens than bytes"), v2)?)))
                } else {
                    let v4 = m1798 - 1;
                    let v5: u64 = f323(v1.clone(), v2)?;
                    let a1799 = v5;
                    let a1800 = 256u64;
                    if nat_eq(a1799, a1800) {
                        Err::<(u64, (u64, List<Adt20>)), R<Result<List<Adt20>, Bytes>>>(Ok(Ok::<List<Adt20>, Bytes>(f346(v3, List::<Adt20>::nil()))))
                    } else {
                        if f347(v5)? {
                            Ok::<(u64, (u64, List<Adt20>)), R<Result<List<Adt20>, Bytes>>>((v4, ({
                                let a1801 = v2;
                                let a1802 = 1u64;
                                nat_add(a1801, a1802)?
                            }, v3)))
                        } else {
                            if f179(v1.clone(), v2, Str::lit("//")) {
                                let m1803 = f324(v1.clone(), Bytes::lit(&[0x0a]), v2)?;
                                match m1803 {
                                    None => {
                                        Err::<(u64, (u64, List<Adt20>)), R<Result<List<Adt20>, Bytes>>>(Ok(Ok::<List<Adt20>, Bytes>(f346(v3, List::<Adt20>::nil()))))
                                    }
                                    Some(v6) => {
                                        Ok::<(u64, (u64, List<Adt20>)), R<Result<List<Adt20>, Bytes>>>((v4, ({
                                            let a1804 = v6;
                                            let a1805 = 1u64;
                                            nat_add(a1804, a1805)?
                                        }, v3)))
                                    }
                                }
                            } else {
                                if f179(v1.clone(), v2, Str::lit("/*")) {
                                    let m1809 = f324(v1.clone(), {
                                        let a1806 = Str::lit("*/");
                                        utf8_encode(a1806)
                                    }, {
                                        let a1807 = v2;
                                        let a1808 = 2u64;
                                        nat_add(a1807, a1808)?
                                    })?;
                                    match m1809 {
                                        None => {
                                            Err::<(u64, (u64, List<Adt20>)), R<Result<List<Adt20>, Bytes>>>(Ok(Err::<List<Adt20>, Bytes>(f345(Str::lit("a comment is not closed (Web IDL, IDL grammar: comment)"), v2)?)))
                                        }
                                        Some(v7) => {
                                            Ok::<(u64, (u64, List<Adt20>)), R<Result<List<Adt20>, Bytes>>>((v4, ({
                                                let a1810 = v7;
                                                let a1811 = 2u64;
                                                nat_add(a1810, a1811)?
                                            }, v3)))
                                        }
                                    }
                                } else {
                                    let a1812 = v5;
                                    let a1813 = 34u64;
                                    if nat_eq(a1812, a1813) {
                                        let m1816 = f324(v1.clone(), Bytes::lit(&[0x22]), {
                                            let a1814 = v2;
                                            let a1815 = 1u64;
                                            nat_add(a1814, a1815)?
                                        })?;
                                        match m1816 {
                                            None => {
                                                Err::<(u64, (u64, List<Adt20>)), R<Result<List<Adt20>, Bytes>>>(Ok(Err::<List<Adt20>, Bytes>(f345(Str::lit("a string is not closed (Web IDL, IDL grammar: string)"), v2)?)))
                                            }
                                            Some(v8) => {
                                                Ok::<(u64, (u64, List<Adt20>)), R<Result<List<Adt20>, Bytes>>>((v4, ({
                                                    let a1817 = v8;
                                                    let a1818 = 1u64;
                                                    nat_add(a1817, a1818)?
                                                }, List::cons(Adt20::C0(3u64, f46(v1.clone(), {
                                                    let a1819 = v2;
                                                    let a1820 = 1u64;
                                                    nat_add(a1819, a1820)?
                                                }, {
                                                    let a1823 = v8;
                                                    let a1824 = {
                                                        let a1821 = v2;
                                                        let a1822 = 1u64;
                                                        nat_add(a1821, a1822)?
                                                    };
                                                    nat_sub(a1823, a1824)
                                                })), v3))))
                                            }
                                        }
                                    } else {
                                        let v9: u64 = {
                                            let a1825 = v5;
                                            let a1826 = 95u64;
                                            if if nat_eq(a1825, a1826) {
                                                true
                                            } else {
                                                let a1827 = v5;
                                                let a1828 = 45u64;
                                                nat_eq(a1827, a1828)
                                            } {
                                                let a1829 = v2;
                                                let a1830 = 1u64;
                                                nat_add(a1829, a1830)?
                                            } else {
                                                v2
                                            }
                                        };
                                        if f348(f323(v1.clone(), v9)?) {
                                            let v10: u64 = f349(v1.clone(), 3u64, {
                                                let a1831 = v9;
                                                let a1832 = 1u64;
                                                nat_add(a1831, a1832)?
                                            })?;
                                            Ok::<(u64, (u64, List<Adt20>)), R<Result<List<Adt20>, Bytes>>>((v4, (v10, List::cons(Adt20::C0(0u64, f46(v1.clone(), v2, {
                                                let a1833 = v10;
                                                let a1834 = v2;
                                                nat_sub(a1833, a1834)
                                            })), v3))))
                                        } else {
                                            let v11: u64 = {
                                                let a1835 = v5;
                                                let a1836 = 45u64;
                                                if nat_eq(a1835, a1836) {
                                                    let a1837 = v2;
                                                    let a1838 = 1u64;
                                                    nat_add(a1837, a1838)?
                                                } else {
                                                    v2
                                                }
                                            };
                                            let v12: u64 = f350(v1.clone(), v11)?;
                                            let v13: u64 = f351(v1.clone(), v11)?;
                                            let a1839 = v12;
                                            let a1840 = v13;
                                            if nat_lt(a1839, a1840) {
                                                Ok::<(u64, (u64, List<Adt20>)), R<Result<List<Adt20>, Bytes>>>((v4, (v13, List::cons(Adt20::C0(2u64, f46(v1.clone(), v2, {
                                                    let a1841 = v13;
                                                    let a1842 = v2;
                                                    nat_sub(a1841, a1842)
                                                })), v3))))
                                            } else {
                                                let a1843 = v11;
                                                let a1844 = v12;
                                                if nat_lt(a1843, a1844) {
                                                    Ok::<(u64, (u64, List<Adt20>)), R<Result<List<Adt20>, Bytes>>>((v4, (v12, List::cons(Adt20::C0(1u64, f46(v1.clone(), v2, {
                                                        let a1845 = v12;
                                                        let a1846 = v2;
                                                        nat_sub(a1845, a1846)
                                                    })), v3))))
                                                } else {
                                                    if f179(v1.clone(), v2, Str::lit("...")) {
                                                        Ok::<(u64, (u64, List<Adt20>)), R<Result<List<Adt20>, Bytes>>>((v4, ({
                                                            let a1847 = v2;
                                                            let a1848 = 3u64;
                                                            nat_add(a1847, a1848)?
                                                        }, List::cons(Adt20::C0(4u64, f8(Str::lit("..."))), v3))))
                                                    } else {
                                                        Ok::<(u64, (u64, List<Adt20>)), R<Result<List<Adt20>, Bytes>>>((v4, ({
                                                            let a1849 = v2;
                                                            let a1850 = 1u64;
                                                            nat_add(a1849, a1850)?
                                                        }, List::cons(Adt20::C0(4u64, f46(v1.clone(), v2, 1u64)), v3))))
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
                Ok(next) => { m1851 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f265(p1857: List<Adt13>, p1858: List<Adt13>) -> List<Adt13> {
    let m1856 = (p1857, p1858);
    {
        let mut m1856 = m1856;
        loop {
            let step = {
                let (v0, v1) = m1856;
                let m1855 = v0;
                match m1855.uncons() {
                    None => {
                        Err::<(List<Adt13>, List<Adt13>), List<Adt13>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt13>, List<Adt13>), List<Adt13>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1856 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f266(v0: List<Adt20>) -> R<Result<List<Adt20>, Bytes>> {
    if f352(v0.clone(), Str::lit("[")) {
        Ok(f353({
            let a1860 = {
                let a1859 = v0.clone();
                length_list(a1859)
            };
            let a1861 = 1u64;
            nat_add(a1860, a1861)?
        }, f354(v0), List::cons(f8(Str::lit("]")), List::<Bytes>::nil()), false))
    } else {
        Ok(Ok::<List<Adt20>, Bytes>(v0))
    }
}

pub fn f267(v0: u64, v1: List<Adt20>) -> R<Result<(Adt13, List<Adt20>), Bytes>> {
    if f352(v1.clone(), Str::lit("interface")) {
        if f352(f354(v1.clone()), Str::lit("mixin")) {
            f355(v0, 1u64, f354(f354(v1)), false)
        } else {
            f355(v0, 0u64, f354(v1), true)
        }
    } else {
        if f352(v1.clone(), Str::lit("partial")) {
            let v2: List<Adt20> = f354(v1);
            if f352(v2.clone(), Str::lit("interface")) {
                if f352(f354(v2.clone()), Str::lit("mixin")) {
                    f355(v0, 6u64, f354(f354(v2)), false)
                } else {
                    f355(v0, 5u64, f354(v2), false)
                }
            } else {
                if f352(v2.clone(), Str::lit("dictionary")) {
                    f355(v0, 7u64, f354(v2), false)
                } else {
                    if f352(v2.clone(), Str::lit("namespace")) {
                        f356(f354(v2), f8(Str::lit("a namespace holds operations and attributes of no object, so it is behavior, not data (Web IDL 2.6)")))
                    } else {
                        Ok(Err::<(Adt13, List<Adt20>), Bytes>(f357(Str::lit("interface, dictionary or namespace"), v2)))
                    }
                }
            }
        } else {
            if f352(v1.clone(), Str::lit("dictionary")) {
                f355(v0, 2u64, f354(v1), true)
            } else {
                if f352(v1.clone(), Str::lit("enum")) {
                    let m1862 = f358(f354(v1));
                    match m1862 {
                        Ok(v4) => {
                            let m1863 = f359(f360(v4.clone()), Str::lit("{"));
                            match m1863 {
                                Ok(v6) => {
                                    let m1864 = f361(v0, v6, List::<Bytes>::nil());
                                    match m1864 {
                                        Ok(v8) => {
                                            Ok(Ok::<(Adt13, List<Adt20>), Bytes>((Adt13::C0(3u64, f362(v4), Bytes::lit(&[]), List::<Adt14>::nil(), {
                                                let (h1865, _) = v8;
                                                h1865
                                            }, f363(Bytes::lit(&[]))), {
                                                let (_, h1866) = v8;
                                                h1866
                                            })))
                                        }
                                        Err(v7) => {
                                            Ok(Err::<(Adt13, List<Adt20>), Bytes>(v7))
                                        }
                                    }
                                }
                                Err(v5) => {
                                    Ok(Err::<(Adt13, List<Adt20>), Bytes>(v5))
                                }
                            }
                        }
                        Err(v3) => {
                            Ok(Err::<(Adt13, List<Adt20>), Bytes>(v3))
                        }
                    }
                } else {
                    if f352(v1.clone(), Str::lit("typedef")) {
                        let m1867 = f364(v0, f354(v1), 0u64)?;
                        match m1867 {
                            Ok(v10) => {
                                let m1868 = f358(f365(v10.clone()));
                                match m1868 {
                                    Ok(v12) => {
                                        let m1869 = f359(f360(v12.clone()), Str::lit(";"));
                                        match m1869 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt13, List<Adt20>), Bytes>((Adt13::C0(4u64, f362(v12), Bytes::lit(&[]), List::<Adt14>::nil(), List::<Bytes>::nil(), f366(v10)), v14)))
                                            }
                                            Err(v13) => {
                                                Ok(Err::<(Adt13, List<Adt20>), Bytes>(v13))
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(Err::<(Adt13, List<Adt20>), Bytes>(v11))
                                    }
                                }
                            }
                            Err(v9) => {
                                Ok(Err::<(Adt13, List<Adt20>), Bytes>(v9))
                            }
                        }
                    } else {
                        if f352(v1.clone(), Str::lit("callback")) {
                            if f352(f354(v1.clone()), Str::lit("interface")) {
                                f356(f354(f354(v1)), f8(Str::lit("a callback interface is implemented by user code, so it is behavior, not data (Web IDL 2.4)")))
                            } else {
                                f356(f354(v1), f8(Str::lit("a callback function is behavior, not data (Web IDL 2.10)")))
                            }
                        } else {
                            if f352(v1.clone(), Str::lit("namespace")) {
                                f356(f354(v1), f8(Str::lit("a namespace holds operations and attributes of no object, so it is behavior, not data (Web IDL 2.6)")))
                            } else {
                                let m1870 = f358(v1);
                                match m1870 {
                                    Ok(v16) => {
                                        let m1871 = f359(f360(v16.clone()), Str::lit("includes"));
                                        match m1871 {
                                            Ok(v18) => {
                                                let m1872 = f358(v18);
                                                match m1872 {
                                                    Ok(v20) => {
                                                        let m1873 = f359(f360(v20.clone()), Str::lit(";"));
                                                        match m1873 {
                                                            Ok(v22) => {
                                                                Ok(Ok::<(Adt13, List<Adt20>), Bytes>((Adt13::C0(8u64, f362(v16), f362(v20), List::<Adt14>::nil(), List::<Bytes>::nil(), f363(Bytes::lit(&[]))), v22)))
                                                            }
                                                            Err(v21) => {
                                                                Ok(Err::<(Adt13, List<Adt20>), Bytes>(v21))
                                                            }
                                                        }
                                                    }
                                                    Err(v19) => {
                                                        Ok(Err::<(Adt13, List<Adt20>), Bytes>(v19))
                                                    }
                                                }
                                            }
                                            Err(v17) => {
                                                Ok(Err::<(Adt13, List<Adt20>), Bytes>(v17))
                                            }
                                        }
                                    }
                                    Err(v15) => {
                                        Ok(Err::<(Adt13, List<Adt20>), Bytes>(v15))
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

pub fn f268(p1877: List<Bytes>, v1: List<Adt16>) -> Bytes {
    let m1876 = p1877;
    {
        let mut m1876 = m1876;
        loop {
            let step = {
                let v0 = m1876;
                let m1874 = v0;
                match m1874.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1875 = f195(v1.clone(), v2.clone());
                        match m1875 {
                            None => {
                                Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("refers to ")), List::cons(v2, List::cons(f8(Str::lit(", which no input defines (Web IDL 2.13.22 to 2.13.26)")), List::<Bytes>::nil())))))
                            }
                            Some(v4) => {
                                if f32(f196(v4.clone()), Bytes::lit(&[])) {
                                    Ok::<List<Bytes>, Bytes>(v3)
                                } else {
                                    Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("refers to ")), List::cons(v2, List::cons(f8(Str::lit(", which is not translated: ")), List::cons(f196(v4), List::<Bytes>::nil()))))))
                                }
                            }
                        }
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

pub fn f269(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f117(v0)?;
    if f114(v1.clone(), f367()) {
        f368(v1)
    } else {
        Ok(v1)
    }
}

pub fn f270(p1883: List<Adt13>, v1: List<Adt13>, v2: Bytes) -> Option<Bytes> {
    let m1882 = p1883;
    {
        let mut m1882 = m1882;
        loop {
            let step = {
                let v0 = m1882;
                let m1878 = v0;
                match m1878.uncons() {
                    None => {
                        Err::<List<Adt13>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v3, v4)) => {
                        let a1879 = f192(v3.clone());
                        let a1880 = 8u64;
                        if if nat_eq(a1879, a1880) {
                            f32(f194(v3.clone()), v2.clone())
                        } else {
                            false
                        } {
                            let m1881 = f202(v1.clone(), 1u64, f204(v3.clone()));
                            match m1881 {
                                None => {
                                    Err::<List<Adt13>, Option<Bytes>>(Some(f204(v3)))
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
                Ok(next) => { m1882 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f271(v0: u64, v1: List<Adt13>, v2: Adt13) -> R<Result<List<Adt14>, Bytes>> {
    let m1884 = v0;
    if m1884 == 0 {
        Ok(Err::<List<Adt14>, Bytes>(f7(List::cons(f8(Str::lit("the inheritance of ")), List::cons(f194(v2), List::cons(f8(Str::lit(" is cyclic (Web IDL 2.2 and 2.7: an interface or dictionary must not inherit from itself)")), List::<Bytes>::nil()))))))
    } else {
        let v3 = m1884 - 1;
        let v4: List<Adt14> = {
            let a1885 = f192(v2.clone());
            let a1886 = 2u64;
            if nat_eq(a1885, a1886) {
                f369(f370(f371(v2.clone()), f372(v1.clone(), 7u64, f194(v2.clone()))))?
            } else {
                f370(f371(v2.clone()), f370(f372(v1.clone(), 5u64, f194(v2.clone())), f373(v1.clone(), v1.clone(), f194(v2.clone()))))
            }
        };
        if f32(f204(v2.clone()), Bytes::lit(&[])) {
            Ok(Ok::<List<Adt14>, Bytes>(v4))
        } else {
            let m1887 = f202(v1.clone(), f192(v2.clone()), f204(v2.clone()));
            match m1887 {
                None => {
                    Ok(Err::<List<Adt14>, Bytes>(f7(List::cons(f8(Str::lit("inherits ")), List::cons(f204(v2.clone()), List::cons({
                        let a1888 = f192(v2);
                        let a1889 = 2u64;
                        if nat_eq(a1888, a1889) {
                            f8(Str::lit(", which no input defines as a dictionary (Web IDL 2.7)"))
                        } else {
                            f8(Str::lit(", which no input defines as an interface (Web IDL 2.2)"))
                        }
                    }, List::<Bytes>::nil()))))))
                }
                Some(v5) => {
                    let m1890 = f271(v3, v1, v5)?;
                    match m1890 {
                        Ok(v7) => {
                            Ok(Ok::<List<Adt14>, Bytes>(f370(v7, v4)))
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

pub fn f272(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: mk, fields: []}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))
}

pub fn f273(v0: bool, v1: List<Adt14>) -> R<Result<(List<Bytes>, List<Bytes>), Bytes>> {
    let m1891 = v1;
    match m1891.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::<Bytes>::nil(), List::<Bytes>::nil())))
        }
        Some((v2, v3)) => {
            let v4: Adt15 = f374(v2.clone());
            if !f32(f283(v4.clone()), Bytes::lit(&[])) {
                Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons(f375(v2), List::cons(f8(Str::lit(": ")), List::cons(f283(v4), List::<Bytes>::nil())))))))
            } else {
                let m1892 = f376(v0, f375(v2.clone()))?;
                match m1892 {
                    None => {
                        Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons(f375(v2), List::cons(f8(Str::lit(" has no field name")), List::<Bytes>::nil()))))))
                    }
                    Some(v5) => {
                        if f114(v5.clone(), f377(v0, f275(v3.clone()))?) {
                            Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("two of its members derive the field name ")), List::cons(v5, List::cons(f8(Str::lit(" (Web IDL 2.2 and 2.7: the members of one definition have distinct identifiers)")), List::<Bytes>::nil()))))))
                        } else {
                            let m1893 = f273(v0, v3)?;
                            match m1893 {
                                Ok(v7) => {
                                    Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v5, List::cons(f8(Str::lit(", type: ")), List::cons({
                                        let a1894 = f378(v2);
                                        let a1895 = 1u64;
                                        if nat_eq(a1894, a1895) {
                                            f7(List::cons(f8(Str::lit("(Option ")), List::cons(f285(v4.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                        } else {
                                            f285(v4.clone())
                                        }
                                    }, List::cons(f8(Str::lit("}")), List::<Bytes>::nil())))))), {
                                        let (h1896, _) = v7;
                                        h1896
                                    }), f197(f286(v4), {
                                        let (_, h1897) = v7;
                                        h1897
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

pub fn f274(p1901: List<Bytes>) -> R<bool> {
    let m1900 = p1901;
    {
        let mut m1900 = m1900;
        loop {
            let step = {
                let v0 = m1900;
                let m1898 = v0;
                match m1898.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m1899 = f379(v1)?;
                        match m1899 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f114(v3, f377(false, v2.clone())?) {
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
                Ok(next) => { m1900 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f275(v0: List<Adt14>) -> List<Bytes> {
    let m1902 = v0;
    match m1902.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f375(v1), f275(v2))
        }
    }
}

pub fn f276(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}")), List::cons(f148(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f277(p1905: List<Bytes>) -> Option<Bytes> {
    let m1904 = p1905;
    {
        let mut m1904 = m1904;
        loop {
            let step = {
                let v0 = m1904;
                let m1903 = v0;
                match m1903.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f114(v1.clone(), v2.clone()) {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v1))
                        } else {
                            Ok::<List<Bytes>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1904 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f278(v0: Adt13) -> List<Bytes> {
    let m1906 = v0;
    match m1906 {
        Adt13::C0(_, _, _, _, v5, _) => {
            v5
        }
    }
}

pub fn f279(v0: bool, v1: List<Adt2>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1907 = v1;
    match m1907.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m1908 = f380(v0, v3.clone())?;
            match m1908 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f381(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f114(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m1909 = f279(v0, v4, List::cons(v5.clone(), v2))?;
                        match m1909 {
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

pub fn f280(v0: List<Bytes>) -> List<Adt2> {
    let m1910 = v0;
    match m1910.uncons() {
        None => {
            List::<Adt2>::nil()
        }
        Some((v1, v2)) => {
            List::cons(Adt2::C6(v1), f280(v2))
        }
    }
}

pub fn f281(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f148(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f282(v0: Adt13) -> Adt15 {
    let m1911 = v0;
    match m1911 {
        Adt13::C0(_, _, _, _, _, v6) => {
            v6
        }
    }
}

pub fn f283(v0: Adt15) -> Bytes {
    let m1912 = v0;
    match m1912 {
        Adt15::C0(_, _, _, v4, _) => {
            v4
        }
    }
}

pub fn f284(v0: Adt15) -> u64 {
    let m1913 = v0;
    match m1913 {
        Adt15::C0(v1, _, _, _, _) => {
            v1
        }
    }
}

pub fn f285(v0: Adt15) -> Bytes {
    let m1914 = v0;
    match m1914 {
        Adt15::C0(_, v2, _, _, _) => {
            v2
        }
    }
}

pub fn f286(v0: Adt15) -> List<Bytes> {
    let m1915 = v0;
    match m1915 {
        Adt15::C0(_, _, v3, _, _) => {
            v3
        }
    }
}

pub fn f287(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f288(p1918: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m1917 = p1918;
    {
        let mut m1917 = m1917;
        loop {
            let step = {
                let v0 = m1917;
                let m1916 = v0;
                match m1916.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f114(v2.clone(), v1.clone()) {
                            Ok::<List<Bytes>, Option<Bytes>>(v3)
                        } else {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v2))
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

pub fn f289(v0: bool, v1: u64) -> R<bool> {
    let a1925 = {
        let a1922 = {
            let a1919 = v1;
            let a1920 = f301(v0)?;
            let a1921 = 0u64;
            nat_quot(a1919, a1920, a1921)
        };
        let a1923 = f303(f304(v0))?;
        let a1924 = 0u64;
        nat_rem(a1922, a1923, a1924)
    };
    let a1926 = f305(v0)?;
    Ok(!nat_eq(a1925, a1926))
}

pub fn f290(v0: Bytes, v1: u64) -> R<Bytes> {
    let a1929 = f46(v0.clone(), {
        let a1927 = v1;
        let a1928 = 2u64;
        nat_add(a1927, a1928)?
    }, 2u64);
    let a1930 = Bytes::lit(&[0x30, 0x30]);
    if equal(a1929, a1930) {
        let m1933 = f382(f46(v0, {
            let a1931 = v1;
            let a1932 = 4u64;
            nat_add(a1931, a1932)?
        }, 2u64))?;
        match m1933 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f103(v2.clone(), 0u64)?;
                let a1934 = v3;
                let a1935 = 32u64;
                if nat_lt(a1934, a1935) {
                    if f383(v3) {
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

pub fn f291(v0: u64) -> Bytes {
    let a1936 = v0;
    let a1937 = 34u64;
    if nat_eq(a1936, a1937) {
        Bytes::lit(&[0x22])
    } else {
        let a1938 = v0;
        let a1939 = 92u64;
        if nat_eq(a1938, a1939) {
            Bytes::lit(&[0x5c])
        } else {
            let a1940 = v0;
            let a1941 = 98u64;
            if nat_eq(a1940, a1941) {
                Bytes::lit(&[0x08])
            } else {
                let a1942 = v0;
                let a1943 = 102u64;
                if nat_eq(a1942, a1943) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a1944 = v0;
                    let a1945 = 110u64;
                    if nat_eq(a1944, a1945) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a1946 = v0;
                        let a1947 = 114u64;
                        if nat_eq(a1946, a1947) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a1948 = v0;
                            let a1949 = 116u64;
                            if nat_eq(a1948, a1949) {
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

pub fn f292(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f103(v0, v1)?;
    let a1950 = 47u64;
    let a1951 = v2;
    if if nat_lt(a1950, a1951) {
        let a1952 = v2;
        let a1953 = 58u64;
        nat_lt(a1952, a1953)
    } else {
        false
    } {
        Ok(Some({
            let a1954 = v1;
            let a1955 = 1u64;
            nat_add(a1954, a1955)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f293(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1956 = f103(v0, v1)?;
    let a1957 = 48u64;
    if nat_eq(a1956, a1957) {
        Ok(Some({
            let a1958 = v1;
            let a1959 = 1u64;
            nat_add(a1958, a1959)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f294(v0: Bytes) -> R<u64> {
    let (h1960, _) = f176(Fn5::F384(v0.clone()), {
        let a1961 = v0;
        length_bytes(a1961)
    }, 0u64)?;
    Ok(h1960)
}

pub fn f295(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m1965 = {
        let a1963 = v0;
        let a1964 = {
            let (h1962, _) = v1;
            h1962
        };
        index_bytes(a1963, a1964)
    };
    match m1965 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1967 = {
                    let (h1966, _) = v1;
                    h1966
                };
                let a1968 = 1u64;
                nat_add(a1967, a1968)?
            }, {
                let a1972 = {
                    let a1970 = {
                        let (_, h1969) = v1;
                        h1969
                    };
                    let a1971 = 10u64;
                    nat_mul(a1970, a1971)?
                };
                let a1973 = f385(v2)?;
                nat_add(a1972, a1973)?
            })))
        }
    }
}

pub fn f296(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f386(v0, v1)
}

pub fn f297(v0: Fn7, p1977: u64, p1978: Adt22) -> R<Adt22> {
    let m1976 = (p1977, p1978);
    {
        let mut m1976 = m1976;
        loop {
            let step = {
                let (v1, v2) = m1976;
                let m1974 = v1;
                if m1974 == 0 {
                    Err::<(u64, Adt22), R<Adt22>>(Ok(v2))
                } else {
                    let v3 = m1974 - 1;
                    Ok::<(u64, Adt22), R<Adt22>>((v3, {
                        let c1975: Fn7 = v0.clone();
                        c1975.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1976 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f298(v0: Adt22) -> R<Adt22> {
    f387(v0)
}

pub fn f299(v0: u32, v1: u32) -> u32 {
    let m1981 = {
        let a1979 = f388(v0);
        let a1980 = f388(v1);
        fixed_u64::checked_add(a1979, a1980)
    };
    match m1981 {
        None => {
            0u32
        }
        Some(v2) => {
            f389(v2)
        }
    }
}

pub fn f300(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a1984 = {
        let a1982 = v3;
        let a1983 = 8u64;
        nat_add(a1982, a1983)?
    };
    let a1985 = v2;
    if nat_eq(a1984, a1985) {
        Ok(f390({
            let a1988 = {
                let a1986 = v1;
                let a1987 = 8u64;
                nat_mul(a1986, a1987)?
            };
            let a1989 = 4294967296u64;
            let a1990 = 0u64;
            nat_quot(a1988, a1989, a1990)
        }))
    } else {
        let a1993 = {
            let a1991 = v3;
            let a1992 = 4u64;
            nat_add(a1991, a1992)?
        };
        let a1994 = v2;
        if nat_eq(a1993, a1994) {
            Ok(f390({
                let a1997 = {
                    let a1995 = v1;
                    let a1996 = 8u64;
                    nat_mul(a1995, a1996)?
                };
                let a1998 = 4294967296u64;
                let a1999 = 0u64;
                nat_rem(a1997, a1998, a1999)
            }))
        } else {
            Ok(f391(f392(v0.clone(), v1, v3), f392(v0.clone(), v1, {
                let a2000 = v3;
                let a2001 = 1u64;
                nat_add(a2000, a2001)?
            }), f392(v0.clone(), v1, {
                let a2002 = v3;
                let a2003 = 2u64;
                nat_add(a2002, a2003)?
            }), f392(v0, v1, {
                let a2004 = v3;
                let a2005 = 3u64;
                nat_add(a2004, a2005)?
            })))
        }
    }
}

pub fn f301(v0: bool) -> R<u64> {
    f303({
        let a2006 = f308(v0);
        let a2007 = 1u64;
        nat_sub(a2006, a2007)
    })
}

pub fn f302(v0: bool) -> R<u64> {
    f303(if v0 {
        63u64
    } else {
        31u64
    })
}

pub fn f303(v0: u64) -> R<u64> {
    let m2008 = v0;
    if m2008 == 0 {
        Ok(1u64)
    } else {
        let v1 = m2008 - 1;
        let a2009 = 2u64;
        let a2010 = f303(v1)?;
        nat_mul(a2009, a2010)
    }
}

pub fn f304(v0: bool) -> u64 {
    if v0 {
        11u64
    } else {
        8u64
    }
}

pub fn f305(v0: bool) -> R<u64> {
    let a2011 = f303(f304(v0))?;
    let a2012 = 1u64;
    Ok(nat_sub(a2011, a2012))
}

pub fn f306(v0: bool, v1: Bytes) -> Bytes {
    if v0 {
        let a2013 = Bytes::lit(&[0x2d]);
        let a2014 = v1;
        append_bytes(a2013, a2014)
    } else {
        v1
    }
}

pub fn f307(v0: bool) -> u64 {
    if v0 {
        1023u64
    } else {
        127u64
    }
}

pub fn f308(v0: bool) -> u64 {
    if v0 {
        53u64
    } else {
        24u64
    }
}

pub fn f309(v0: u64, v1: bool, v2: u64) -> R<Bytes> {
    if v1 {
        f393(f394(v2, f395(3u64, v0), 2u64)?)
    } else {
        let v3: Bytes = f393(f394(v2, f395(3u64, v0), 5u64)?)?;
        let v4: u64 = {
            let a2015 = v3.clone();
            length_bytes(a2015)
        };
        let v5: Bytes = {
            let a2016 = v2;
            let a2017 = v4;
            if nat_lt(a2016, a2017) {
                f46(v3.clone(), 0u64, {
                    let a2018 = v4;
                    let a2019 = v2;
                    nat_sub(a2018, a2019)
                })
            } else {
                Bytes::lit(&[0x30])
            }
        };
        let v6: Bytes = f396({
            let a2020 = v2;
            let a2021 = v4;
            if nat_lt(a2020, a2021) {
                f46(v3, {
                    let a2022 = v4;
                    let a2023 = v2;
                    nat_sub(a2022, a2023)
                }, v2)
            } else {
                let a2026 = f397({
                    let a2024 = v2;
                    let a2025 = v4;
                    nat_sub(a2024, a2025)
                });
                let a2027 = v3;
                append_bytes(a2026, a2027)
            }
        }, {
            let a2028 = v2;
            let a2029 = 1u64;
            nat_add(a2028, a2029)?
        })?;
        let a2031 = {
            let a2030 = v6.clone();
            length_bytes(a2030)
        };
        let a2032 = 0u64;
        if nat_eq(a2031, a2032) {
            Ok(v5)
        } else {
            Ok(f7(List::cons(v5, List::cons(Bytes::lit(&[0x2e]), List::cons(v6, List::<Bytes>::nil())))))
        }
    }
}

pub fn f310(v0: Bytes, p2051: u64, p2052: u64, p2053: Bytes) -> R<Bytes> {
    let m2050 = (p2051, (p2052, p2053));
    {
        let mut m2050 = m2050;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2050;
                let m2033 = v2;
                if m2033 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m2033 - 1;
                    let v5: u64 = f152(v0.clone(), v1, {
                        let a2037 = {
                            let a2035 = {
                                let a2034 = v0.clone();
                                length_bytes(a2034)
                            };
                            let a2036 = v1;
                            nat_sub(a2035, a2036)
                        };
                        let a2038 = 1u64;
                        nat_add(a2037, a2038)?
                    })?;
                    let v6: Bytes = {
                        let a2041 = v3;
                        let a2042 = f46(v0.clone(), v1, {
                            let a2039 = v5;
                            let a2040 = v1;
                            nat_sub(a2039, a2040)
                        });
                        append_bytes(a2041, a2042)
                    };
                    let m2045 = {
                        let a2043 = v0.clone();
                        let a2044 = v5;
                        index_bytes(a2043, a2044)
                    };
                    match m2045 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v6))
                        }
                        Some(v7) => {
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                let a2046 = v5;
                                let a2047 = 1u64;
                                nat_add(a2046, a2047)?
                            }, (v4, {
                                let a2048 = v6;
                                let a2049 = f398(v0.clone(), v5, v7)?;
                                append_bytes(a2048, a2049)
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2050 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f311(v0: Fn8, p2057: List<(Bytes, Bytes)>, p2058: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m2056 = (p2057, p2058);
    {
        let mut m2056 = m2056;
        loop {
            let step = {
                let (v1, v2) = m2056;
                let m2054 = v2;
                match m2054.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c2055: Fn8 = v0.clone();
                            c2055.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2056 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f312(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f399(v0, v1)
}

pub fn f313(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f400(v0)
}

pub fn f314(v0: Fn9, p2062: List<Bytes>, p2063: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m2061 = (p2062, p2063);
    {
        let mut m2061 = m2061;
        loop {
            let step = {
                let (v1, v2) = m2061;
                let m2059 = v2;
                match m2059.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c2060: Fn9 = v0.clone();
                            c2060.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2061 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f315(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f401(v1)?, v0))
}

pub fn f316(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(f402(v0.clone(), 0u64, {
        let a2064 = v0;
        length_bytes(a2064)
    }, Bytes::lit(&[]))?, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
}

pub fn f317(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h2065, _) = v1.clone();
        h2065
    };
    let v3: u64 = {
        let (h2066, _) = {
            let (_, h2067) = v1.clone();
            h2067
        };
        h2066
    };
    let v4: List<Bytes> = {
        let (_, h2068) = {
            let (_, h2069) = v1;
            h2069
        };
        h2068
    };
    let m2070 = f160(v0.clone(), v2);
    match m2070 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a2071 = v5;
            let a2072 = 10u8;
            if equal(a2071, a2072) {
                Ok(Some(({
                    let a2073 = v2;
                    let a2074 = 1u64;
                    nat_add(a2073, a2074)?
                }, ({
                    let a2075 = v2;
                    let a2076 = 1u64;
                    nat_add(a2075, a2076)?
                }, List::cons(f403(f46(v0, v3, {
                    let a2077 = v2;
                    let a2078 = v3;
                    nat_sub(a2077, a2078)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a2079 = v2;
                    let a2080 = 1u64;
                    nat_add(a2079, a2080)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f318(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f319(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m2081 = v0.clone();
    match m2081.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a2082 = v2;
            let a2083 = v1.clone();
            if equal(a2082, a2083) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f320(v0: List<(Bytes, ())>) -> List<Bytes> {
    f234(f404(Fn10::F405, List::<Bytes>::nil(), v0))
}

pub fn f321(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f406(f407(Fn11::F408, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m2088 = {
        let (h2084, _) = f409(Fn12::F410, {
            let a2086 = {
                let a2085 = v0;
                length_list(a2085)
            };
            let a2087 = 1u64;
            nat_add(a2086, a2087)?
        }, v1)?;
        h2084
    };
    match m2088.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f322(v0: List<Bytes>) -> List<(Bytes, ())> {
    f411(f412(Fn13::F413, List::<(Bytes, ())>::nil(), v0))
}

pub fn f323(v0: Bytes, v1: u64) -> R<u64> {
    f103(v0, v1)
}

pub fn f324(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let v3: u64 = {
        let (h2089, _) = f176(Fn5::F414(v1.clone(), v0.clone()), {
            let a2091 = {
                let a2090 = v0.clone();
                length_bytes(a2090)
            };
            let a2092 = 1u64;
            nat_add(a2091, a2092)?
        }, v2)?;
        h2089
    };
    if f125(v0, v1, v3) {
        Ok(Some(v3))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f325(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m2095 = {
        let a2093 = v0.clone();
        let a2094 = v1;
        index_bytes(a2093, a2094)
    };
    match m2095 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f60(v2)?;
            let a2096 = v3;
            let a2097 = 128u64;
            if nat_lt(a2096, a2097) {
                Ok(Some((v3, 1u64)))
            } else {
                if f105({
                    let a2098 = 194u64;
                    let a2099 = v3;
                    nat_le(a2098, a2099)
                }, {
                    let a2100 = v3;
                    let a2101 = 223u64;
                    nat_le(a2100, a2101)
                }) {
                    let m2104 = f415(v0, {
                        let a2102 = v1;
                        let a2103 = 1u64;
                        nat_add(a2102, a2103)?
                    })?;
                    match m2104 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f416({
                                let a2109 = {
                                    let a2107 = {
                                        let a2105 = v3;
                                        let a2106 = 192u64;
                                        nat_sub(a2105, a2106)
                                    };
                                    let a2108 = 64u64;
                                    nat_mul(a2107, a2108)?
                                };
                                let a2110 = v4;
                                nat_add(a2109, a2110)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f105({
                        let a2111 = 224u64;
                        let a2112 = v3;
                        nat_le(a2111, a2112)
                    }, {
                        let a2113 = v3;
                        let a2114 = 239u64;
                        nat_le(a2113, a2114)
                    }) {
                        let m2117 = f415(v0.clone(), {
                            let a2115 = v1;
                            let a2116 = 1u64;
                            nat_add(a2115, a2116)?
                        })?;
                        match m2117 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m2120 = f415(v0, {
                                    let a2118 = v1;
                                    let a2119 = 2u64;
                                    nat_add(a2118, a2119)?
                                })?;
                                match m2120 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f416({
                                            let a2129 = {
                                                let a2123 = {
                                                    let a2121 = v3;
                                                    let a2122 = 224u64;
                                                    nat_sub(a2121, a2122)
                                                };
                                                let a2124 = 4096u64;
                                                nat_mul(a2123, a2124)?
                                            };
                                            let a2130 = {
                                                let a2127 = {
                                                    let a2125 = v5;
                                                    let a2126 = 64u64;
                                                    nat_mul(a2125, a2126)?
                                                };
                                                let a2128 = v6;
                                                nat_add(a2127, a2128)?
                                            };
                                            nat_add(a2129, a2130)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f105({
                            let a2131 = 240u64;
                            let a2132 = v3;
                            nat_le(a2131, a2132)
                        }, {
                            let a2133 = v3;
                            let a2134 = 244u64;
                            nat_le(a2133, a2134)
                        }) {
                            let m2137 = f415(v0.clone(), {
                                let a2135 = v1;
                                let a2136 = 1u64;
                                nat_add(a2135, a2136)?
                            })?;
                            match m2137 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m2140 = f415(v0.clone(), {
                                        let a2138 = v1;
                                        let a2139 = 2u64;
                                        nat_add(a2138, a2139)?
                                    })?;
                                    match m2140 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m2143 = f415(v0, {
                                                let a2141 = v1;
                                                let a2142 = 3u64;
                                                nat_add(a2141, a2142)?
                                            })?;
                                            match m2143 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f416({
                                                        let a2156 = {
                                                            let a2146 = {
                                                                let a2144 = v3;
                                                                let a2145 = 240u64;
                                                                nat_sub(a2144, a2145)
                                                            };
                                                            let a2147 = 262144u64;
                                                            nat_mul(a2146, a2147)?
                                                        };
                                                        let a2157 = {
                                                            let a2154 = {
                                                                let a2148 = v7;
                                                                let a2149 = 4096u64;
                                                                nat_mul(a2148, a2149)?
                                                            };
                                                            let a2155 = {
                                                                let a2152 = {
                                                                    let a2150 = v8;
                                                                    let a2151 = 64u64;
                                                                    nat_mul(a2150, a2151)?
                                                                };
                                                                let a2153 = v9;
                                                                nat_add(a2152, a2153)?
                                                            };
                                                            nat_add(a2154, a2155)?
                                                        };
                                                        nat_add(a2156, a2157)?
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

pub fn f326(v0: u64) -> bool {
    let a2158 = v0;
    let a2159 = 9u64;
    if if if nat_eq(a2158, a2159) {
        true
    } else {
        let a2160 = v0;
        let a2161 = 10u64;
        nat_eq(a2160, a2161)
    } {
        true
    } else {
        let a2162 = v0;
        let a2163 = 13u64;
        if nat_eq(a2162, a2163) {
            true
        } else {
            let a2164 = 32u64;
            let a2165 = v0;
            if nat_le(a2164, a2165) {
                let a2166 = v0;
                let a2167 = 126u64;
                nat_le(a2166, a2167)
            } else {
                false
            }
        }
    } {
        true
    } else {
        let a2168 = v0;
        let a2169 = 133u64;
        if if nat_eq(a2168, a2169) {
            true
        } else {
            let a2170 = 160u64;
            let a2171 = v0;
            if nat_le(a2170, a2171) {
                let a2172 = v0;
                let a2173 = 55295u64;
                nat_le(a2172, a2173)
            } else {
                false
            }
        } {
            true
        } else {
            let a2174 = 57344u64;
            let a2175 = v0;
            if if nat_le(a2174, a2175) {
                let a2176 = v0;
                let a2177 = 65533u64;
                nat_le(a2176, a2177)
            } else {
                false
            } {
                true
            } else {
                let a2178 = 65536u64;
                let a2179 = v0;
                if nat_le(a2178, a2179) {
                    let a2180 = v0;
                    let a2181 = 1114111u64;
                    nat_le(a2180, a2181)
                } else {
                    false
                }
            }
        }
    }
}

pub fn f327(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f244(v0.clone(), v1)?;
    if f242(f182(v0.clone(), v2)?) {
        Ok(true)
    } else {
        let a2182 = f182(v0, v2)?;
        let a2183 = 35u64;
        Ok(nat_eq(a2182, a2183))
    }
}

pub fn f328(v0: Bytes, v1: u64) -> R<u64> {
    let (h2184, _) = f176(Fn5::F417(v0), {
        let a2185 = v1;
        let a2186 = 1u64;
        nat_add(a2185, a2186)?
    }, v1)?;
    Ok(h2184)
}

pub fn f329(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f418(v0, v1)
}

pub fn f330(v0: Bytes, v1: u64) -> R<u64> {
    let (h2187, _) = f176(Fn5::F419(v0.clone()), {
        let a2189 = {
            let a2188 = v0;
            length_bytes(a2188)
        };
        let a2190 = 1u64;
        nat_add(a2189, a2190)?
    }, v1)?;
    Ok(h2187)
}

pub fn f331(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f420(v0, v1)
}

pub fn f332(v0: u64, v1: bool) -> bool {
    f184(if if f241(v0) {
        true
    } else {
        f242(v0)
    } {
        true
    } else {
        if v1 {
            f421(v0)
        } else {
            false
        }
    })
}

pub fn f333(v0: Bytes, v1: u64, v2: u64) -> R<Adt19> {
    let v3: bool = {
        let a2191 = f182(v0.clone(), v1)?;
        let a2192 = 62u64;
        nat_eq(a2191, a2192)
    };
    let v4: u64 = f182(v0.clone(), {
        let a2193 = v1;
        let a2194 = 1u64;
        nat_add(a2193, a2194)?
    })?;
    let v5: u64 = f182(v0.clone(), {
        let a2195 = v1;
        let a2196 = 2u64;
        nat_add(a2195, a2196)?
    })?;
    let v6: u64 = f422(v4);
    let v7: u64 = f423(v4);
    let v8: bool = {
        let a2197 = 0u64;
        let a2198 = v6;
        if nat_lt(a2197, a2198) {
            true
        } else {
            let a2199 = 0u64;
            let a2200 = v7;
            nat_lt(a2199, a2200)
        }
    };
    let v9: u64 = {
        let a2201 = 0u64;
        let a2202 = v7;
        if nat_lt(a2201, a2202) {
            f422(v5)
        } else {
            0u64
        }
    };
    let v10: u64 = {
        let a2203 = 0u64;
        let a2204 = v6;
        if nat_lt(a2203, a2204) {
            f423(v5)
        } else {
            0u64
        }
    };
    let v11: u64 = {
        let a2205 = v6;
        let a2206 = v9;
        nat_add(a2205, a2206)?
    };
    let v12: u64 = {
        let a2207 = v7;
        let a2208 = v10;
        nat_add(a2207, a2208)?
    };
    let v13: u64 = {
        let a2217 = {
            let a2209 = v1;
            let a2210 = 1u64;
            nat_add(a2209, a2210)?
        };
        let a2218 = {
            let a2215 = if v8 {
                1u64
            } else {
                0u64
            };
            let a2216 = {
                let a2211 = 0u64;
                let a2212 = v9;
                if if nat_lt(a2211, a2212) {
                    true
                } else {
                    let a2213 = 0u64;
                    let a2214 = v10;
                    nat_lt(a2213, a2214)
                } {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a2215, a2216)?
        };
        nat_add(a2217, a2218)?
    };
    let m2219 = f181(v0.clone(), v13)?;
    match m2219 {
        Ok(v15) => {
            let a2220 = 0u64;
            let a2221 = v12;
            if nat_lt(a2220, a2221) {
                f424(v0, v15, {
                    let a2224 = {
                        let a2222 = v2;
                        let a2223 = v12;
                        nat_add(a2222, a2223)?
                    };
                    let a2225 = 1u64;
                    nat_sub(a2224, a2225)
                }, v3, v11)
            } else {
                let v16: (u64, u64) = {
                    let (h2226, _) = f211(Fn6::F425(v0.clone()), {
                        let a2228 = {
                            let a2227 = v0.clone();
                            length_bytes(a2227)
                        };
                        let a2229 = 1u64;
                        nat_add(a2228, a2229)?
                    }, (v15, 0u64))?;
                    h2226
                };
                let v17: u64 = f246(v0.clone(), {
                    let (h2230, _) = v16;
                    h2230
                })?;
                let a2231 = f182(v0.clone(), v17)?;
                let a2232 = 256u64;
                if nat_eq(a2231, a2232) {
                    Ok(Adt19::C0(Adt2::C6({
                        let a2233 = v11;
                        let a2234 = 2u64;
                        if nat_eq(a2233, a2234) {
                            f426(f427(v0.clone(), v15)?)
                        } else {
                            Bytes::lit(&[])
                        }
                    }), {
                        let a2235 = v0;
                        length_bytes(a2235)
                    }))
                } else {
                    let v18: u64 = f428({
                        let a2237 = v17;
                        let a2238 = {
                            let (h2236, _) = v16;
                            h2236
                        };
                        nat_sub(a2237, a2238)
                    }, v2);
                    let a2240 = v18;
                    let a2241 = {
                        let (_, h2239) = v16;
                        h2239
                    };
                    if nat_lt(a2240, a2241) {
                        Ok(Adt19::C1(f129(Str::lit("a leading empty line of a block scalar holds more spaces than its first content line (YAML 1.2.2 section 8.1.1.1)"), v15)?))
                    } else {
                        f424(v0, v15, v18, v3, v11)
                    }
                }
            }
        }
        Err(v14) => {
            Ok(Adt19::C1(v14))
        }
    }
}

pub fn f334(v0: Adt18, v1: Bytes, v2: bool, v3: u64, v4: u64) -> R<Adt19> {
    let m2242 = v0;
    match m2242 {
        Adt18::C0(v7) => {
            let a2243 = v7;
            let a2244 = 1u64;
            if if if nat_eq(a2243, a2244) {
                true
            } else {
                let a2245 = v7;
                let a2246 = 8u64;
                nat_eq(a2245, a2246)
            } {
                true
            } else {
                let a2247 = v7;
                let a2248 = 0u64;
                if nat_eq(a2247, a2248) {
                    v2
                } else {
                    false
                }
            } {
                Ok(Adt19::C0(Adt2::C6(v1), v4))
            } else {
                let a2249 = v7;
                let a2250 = 0u64;
                if nat_eq(a2249, a2250) {
                    f429(v1, v3, v4)
                } else {
                    let a2251 = v7;
                    let a2252 = 5u64;
                    if nat_eq(a2251, a2252) {
                        if f430(v1) {
                            Ok(Adt19::C0(Adt2::C0, v4))
                        } else {
                            Ok(Adt19::C1(f342(v3)?))
                        }
                    } else {
                        let a2253 = v7;
                        let a2254 = 2u64;
                        if if nat_eq(a2253, a2254) {
                            f431(v1.clone())?
                        } else {
                            let a2255 = v7;
                            let a2256 = 3u64;
                            if nat_eq(a2255, a2256) {
                                let m2257 = f432(v1.clone())?;
                                match m2257 {
                                    None => {
                                        f433(v1.clone())
                                    }
                                    Some(_) => {
                                        true
                                    }
                                }
                            } else {
                                let a2258 = v7;
                                let a2259 = 4u64;
                                if nat_eq(a2258, a2259) {
                                    if f434(v1.clone()) {
                                        true
                                    } else {
                                        f435(v1.clone())
                                    }
                                } else {
                                    false
                                }
                            }
                        } {
                            f429(v1, v3, v4)
                        } else {
                            Ok(Adt19::C1(f342(v3)?))
                        }
                    }
                }
            }
        }
        Adt18::C1(v6) => {
            Ok(Adt19::C0(Adt2::C1(v6, std::rc::Rc::new(Adt2::C6(v1))), v4))
        }
        Adt18::C2(v5) => {
            Ok(Adt19::C1(v5))
        }
    }
}

pub fn f335(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Adt23> {
    let a2260 = f182(v0.clone(), v1)?;
    let a2261 = 34u64;
    if nat_eq(a2260, a2261) {
        f436(v0.clone(), {
            let a2262 = v1;
            let a2263 = 1u64;
            nat_add(a2262, a2263)?
        }, v2, v3, {
            let a2265 = {
                let a2264 = v0;
                length_bytes(a2264)
            };
            let a2266 = 2u64;
            nat_add(a2265, a2266)?
        }, Bytes::lit(&[]))
    } else {
        f437(v0.clone(), {
            let a2267 = v1;
            let a2268 = 1u64;
            nat_add(a2267, a2268)?
        }, v2, v3, {
            let a2270 = {
                let a2269 = v0;
                length_bytes(a2269)
            };
            let a2271 = 2u64;
            nat_add(a2270, a2271)?
        }, Bytes::lit(&[]))
    }
}

pub fn f336(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: List<Bytes>, v6: Adt18) -> R<Adt19> {
    let m2272 = v3;
    if m2272 == 0 {
        Ok(Adt19::C1(f129(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v7 = m2272 - 1;
        let a2273 = v4;
        let a2274 = 1u64;
        if nat_eq(a2273, a2274) {
            let m2275 = f336(v0.clone(), v1, v2, v7, 0u64, List::<Bytes>::nil(), Adt18::C0(0u64))?;
            match m2275 {
                Adt19::C0(v9, v10) => {
                    let m2276 = f438(v0.clone(), v10, v2)?;
                    match m2276 {
                        Ok(v12) => {
                            let v13: u64 = f182(v0.clone(), v12)?;
                            let a2277 = v13;
                            let a2278 = 58u64;
                            if nat_eq(a2277, a2278) {
                                Ok(Adt19::C1(f129(Str::lit("a single-pair mapping inside a flow sequence is not read (YAML 1.2.2 [150])"), v12)?))
                            } else {
                                let a2279 = v13;
                                let a2280 = 93u64;
                                if nat_eq(a2279, a2280) {
                                    Ok(Adt19::C0(Adt2::C10(std::rc::Rc::new(v9), std::rc::Rc::new(Adt2::C9)), {
                                        let a2281 = v12;
                                        let a2282 = 1u64;
                                        nat_add(a2281, a2282)?
                                    }))
                                } else {
                                    let a2283 = v13;
                                    let a2284 = 44u64;
                                    if nat_eq(a2283, a2284) {
                                        let m2287 = f438(v0.clone(), {
                                            let a2285 = v12;
                                            let a2286 = 1u64;
                                            nat_add(a2285, a2286)?
                                        }, v2)?;
                                        match m2287 {
                                            Ok(v15) => {
                                                let a2288 = f182(v0.clone(), v15)?;
                                                let a2289 = 93u64;
                                                if nat_eq(a2288, a2289) {
                                                    Ok(Adt19::C0(Adt2::C10(std::rc::Rc::new(v9), std::rc::Rc::new(Adt2::C9)), {
                                                        let a2290 = v15;
                                                        let a2291 = 1u64;
                                                        nat_add(a2290, a2291)?
                                                    }))
                                                } else {
                                                    let m2292 = f336(v0, v15, v2, v7, 1u64, List::<Bytes>::nil(), Adt18::C0(0u64))?;
                                                    match m2292 {
                                                        Adt19::C0(v17, v18) => {
                                                            Ok(Adt19::C0(Adt2::C10(std::rc::Rc::new(v9), std::rc::Rc::new(v17)), v18))
                                                        }
                                                        Adt19::C1(v16) => {
                                                            Ok(Adt19::C1(v16))
                                                        }
                                                    }
                                                }
                                            }
                                            Err(v14) => {
                                                Ok(Adt19::C1(v14))
                                            }
                                        }
                                    } else {
                                        Ok(Adt19::C1(f129(Str::lit("a flow sequence entry must be followed by , or ] (YAML 1.2.2 [137])"), v12)?))
                                    }
                                }
                            }
                        }
                        Err(v11) => {
                            Ok(Adt19::C1(v11))
                        }
                    }
                }
                Adt19::C1(v8) => {
                    Ok(Adt19::C1(v8))
                }
            }
        } else {
            let a2293 = v4;
            let a2294 = 2u64;
            if nat_eq(a2293, a2294) {
                let m2295 = f439(v0.clone(), v1, v2)?;
                match m2295 {
                    Adt23::C0(v20, v21) => {
                        let m2296 = f438(v0.clone(), v21, v2)?;
                        match m2296 {
                            Ok(v23) => {
                                let m2310 = {
                                    let a2297 = f182(v0.clone(), v23)?;
                                    let a2298 = 44u64;
                                    if if nat_eq(a2297, a2298) {
                                        true
                                    } else {
                                        let a2299 = f182(v0.clone(), v23)?;
                                        let a2300 = 125u64;
                                        nat_eq(a2299, a2300)
                                    } {
                                        Adt19::C0(Adt2::C0, v23)
                                    } else {
                                        if f184({
                                            let a2301 = f182(v0.clone(), v23)?;
                                            let a2302 = 58u64;
                                            nat_eq(a2301, a2302)
                                        }) {
                                            Adt19::C1(f129(Str::lit("a flow mapping entry must be a key, then : and its value, or a key alone (YAML 1.2.2 [142], [146])"), v23)?)
                                        } else {
                                            let m2305 = f438(v0.clone(), {
                                                let a2303 = v23;
                                                let a2304 = 1u64;
                                                nat_add(a2303, a2304)?
                                            }, v2)?;
                                            match m2305 {
                                                Ok(v25) => {
                                                    let a2306 = f182(v0.clone(), v25)?;
                                                    let a2307 = 44u64;
                                                    if if nat_eq(a2306, a2307) {
                                                        true
                                                    } else {
                                                        let a2308 = f182(v0.clone(), v25)?;
                                                        let a2309 = 125u64;
                                                        nat_eq(a2308, a2309)
                                                    } {
                                                        Adt19::C0(Adt2::C0, v25)
                                                    } else {
                                                        f336(v0.clone(), v25, v2, v7, 0u64, List::<Bytes>::nil(), Adt18::C0(0u64))?
                                                    }
                                                }
                                                Err(v24) => {
                                                    Adt19::C1(v24)
                                                }
                                            }
                                        }
                                    }
                                };
                                match m2310 {
                                    Adt19::C0(v27, v28) => {
                                        if f262(v20.clone(), v5.clone()) {
                                            Ok(Adt19::C1(f129(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                        } else {
                                            let m2311 = f438(v0.clone(), v28, v2)?;
                                            match m2311 {
                                                Ok(v30) => {
                                                    let v31: u64 = f182(v0.clone(), v30)?;
                                                    let a2312 = v31;
                                                    let a2313 = 125u64;
                                                    if nat_eq(a2312, a2313) {
                                                        Ok(Adt19::C0(Adt2::C11(v20, std::rc::Rc::new(v27), std::rc::Rc::new(Adt2::C9)), {
                                                            let a2314 = v30;
                                                            let a2315 = 1u64;
                                                            nat_add(a2314, a2315)?
                                                        }))
                                                    } else {
                                                        let a2316 = v31;
                                                        let a2317 = 44u64;
                                                        if nat_eq(a2316, a2317) {
                                                            let m2320 = f438(v0.clone(), {
                                                                let a2318 = v30;
                                                                let a2319 = 1u64;
                                                                nat_add(a2318, a2319)?
                                                            }, v2)?;
                                                            match m2320 {
                                                                Ok(v33) => {
                                                                    let a2321 = f182(v0.clone(), v33)?;
                                                                    let a2322 = 125u64;
                                                                    if nat_eq(a2321, a2322) {
                                                                        Ok(Adt19::C0(Adt2::C11(v20, std::rc::Rc::new(v27), std::rc::Rc::new(Adt2::C9)), {
                                                                            let a2323 = v33;
                                                                            let a2324 = 1u64;
                                                                            nat_add(a2323, a2324)?
                                                                        }))
                                                                    } else {
                                                                        let m2325 = f336(v0, v33, v2, v7, 2u64, List::cons(v20.clone(), v5), Adt18::C0(0u64))?;
                                                                        match m2325 {
                                                                            Adt19::C0(v35, v36) => {
                                                                                Ok(Adt19::C0(Adt2::C11(v20, std::rc::Rc::new(v27), std::rc::Rc::new(v35)), v36))
                                                                            }
                                                                            Adt19::C1(v34) => {
                                                                                Ok(Adt19::C1(v34))
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                Err(v32) => {
                                                                    Ok(Adt19::C1(v32))
                                                                }
                                                            }
                                                        } else {
                                                            Ok(Adt19::C1(f129(Str::lit("a flow mapping entry must be followed by , or } (YAML 1.2.2 [140])"), v30)?))
                                                        }
                                                    }
                                                }
                                                Err(v29) => {
                                                    Ok(Adt19::C1(v29))
                                                }
                                            }
                                        }
                                    }
                                    Adt19::C1(v26) => {
                                        Ok(Adt19::C1(v26))
                                    }
                                }
                            }
                            Err(v22) => {
                                Ok(Adt19::C1(v22))
                            }
                        }
                    }
                    Adt23::C1(v19) => {
                        Ok(Adt19::C1(v19))
                    }
                }
            } else {
                let v37: u64 = f182(v0.clone(), v1)?;
                let a2326 = v37;
                let a2327 = 33u64;
                if nat_eq(a2326, a2327) {
                    if f251(v6) {
                        Ok(Adt19::C1(f129(Str::lit("a node holds at most one tag (YAML 1.2.2 [96])"), v1)?))
                    } else {
                        let v38: Adt18 = f252(v0.clone(), v1)?;
                        let m2328 = f253(v38.clone());
                        match m2328 {
                            None => {
                                let m2329 = f438(v0.clone(), f254(v0.clone(), v1)?, v2)?;
                                match m2329 {
                                    Ok(v41) => {
                                        let a2330 = f257(f8(Str::lit(",]}:")), f182(v0.clone(), v41)?)?;
                                        let a2331 = 4u64;
                                        if nat_lt(a2330, a2331) {
                                            f256(v38, v41, v41)
                                        } else {
                                            f336(v0, v41, v2, v7, 0u64, List::<Bytes>::nil(), v38)
                                        }
                                    }
                                    Err(v40) => {
                                        Ok(Adt19::C1(v40))
                                    }
                                }
                            }
                            Some(v39) => {
                                Ok(Adt19::C1(v39))
                            }
                        }
                    }
                } else {
                    let a2332 = v37;
                    let a2333 = 91u64;
                    if if nat_eq(a2332, a2333) {
                        true
                    } else {
                        let a2334 = v37;
                        let a2335 = 123u64;
                        nat_eq(a2334, a2335)
                    } {
                        let m2338 = f438(v0.clone(), {
                            let a2336 = v1;
                            let a2337 = 1u64;
                            nat_add(a2336, a2337)?
                        }, v2)?;
                        match m2338 {
                            Ok(v43) => {
                                f255(v6, {
                                    let a2339 = v37;
                                    let a2340 = 91u64;
                                    nat_eq(a2339, a2340)
                                }, v1, {
                                    let a2343 = f182(v0.clone(), v43)?;
                                    let a2344 = {
                                        let a2341 = v37;
                                        let a2342 = 2u64;
                                        nat_add(a2341, a2342)?
                                    };
                                    if nat_eq(a2343, a2344) {
                                        Adt19::C0(Adt2::C9, {
                                            let a2345 = v43;
                                            let a2346 = 1u64;
                                            nat_add(a2345, a2346)?
                                        })
                                    } else {
                                        f336(v0, v43, v2, v7, {
                                            let a2347 = v37;
                                            let a2348 = 91u64;
                                            if nat_eq(a2347, a2348) {
                                                1u64
                                            } else {
                                                2u64
                                            }
                                        }, List::<Bytes>::nil(), Adt18::C0(0u64))?
                                    }
                                })
                            }
                            Err(v42) => {
                                Ok(Adt19::C1(v42))
                            }
                        }
                    } else {
                        let a2349 = v37;
                        let a2350 = 34u64;
                        if if nat_eq(a2349, a2350) {
                            true
                        } else {
                            let a2351 = v37;
                            let a2352 = 39u64;
                            nat_eq(a2351, a2352)
                        } {
                            let m2353 = f335(v0, v1, v2, false)?;
                            match m2353 {
                                Adt23::C0(v45, v46) => {
                                    f334(v6, v45, true, v1, v46)
                                }
                                Adt23::C1(v44) => {
                                    Ok(Adt19::C1(v44))
                                }
                            }
                        } else {
                            let m2354 = f249(v0.clone(), v1)?;
                            match m2354 {
                                None => {
                                    if f184(f337(v0.clone(), v1, true)?) {
                                        Ok(Adt19::C1(f129(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                                    } else {
                                        let v48: u64 = f338(v0.clone(), v1, true)?;
                                        let v49: u64 = f244(v0.clone(), v48)?;
                                        let m2357 = {
                                            let a2355 = f182(v0.clone(), v49)?;
                                            let a2356 = 10u64;
                                            if nat_eq(a2355, a2356) {
                                                f438(v0.clone(), v49, v2)?
                                            } else {
                                                Ok::<u64, Bytes>(v49)
                                            }
                                        };
                                        match m2357 {
                                            Ok(v51) => {
                                                let a2358 = f182(v0.clone(), v49)?;
                                                let a2359 = 10u64;
                                                if if if nat_eq(a2358, a2359) {
                                                    f184(f421(f182(v0.clone(), v51)?))
                                                } else {
                                                    false
                                                } {
                                                    f184({
                                                        let a2360 = f182(v0.clone(), v51)?;
                                                        let a2361 = 58u64;
                                                        nat_eq(a2360, a2361)
                                                    })
                                                } else {
                                                    false
                                                } {
                                                    Ok(Adt19::C1(f129(Str::lit("a multi-line plain scalar inside a flow collection is not read (YAML 1.2.2 [135])"), v49)?))
                                                } else {
                                                    f334(v6, f258(v0, v1, v48), false, v1, v48)
                                                }
                                            }
                                            Err(v50) => {
                                                Ok(Adt19::C1(v50))
                                            }
                                        }
                                    }
                                }
                                Some(v47) => {
                                    Ok(Adt19::C1(v47))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f337(v0: Bytes, v1: u64, v2: bool) -> R<bool> {
    let v3: u64 = f182(v0.clone(), v1)?;
    if if f241(v3) {
        true
    } else {
        f242(v3)
    } {
        Ok(false)
    } else {
        let a2362 = v3;
        let a2363 = 45u64;
        if if if nat_eq(a2362, a2363) {
            true
        } else {
            let a2364 = v3;
            let a2365 = 63u64;
            nat_eq(a2364, a2365)
        } {
            true
        } else {
            let a2366 = v3;
            let a2367 = 58u64;
            nat_eq(a2366, a2367)
        } {
            Ok(f332(f182(v0, {
                let a2368 = v1;
                let a2369 = 1u64;
                nat_add(a2368, a2369)?
            })?, v2))
        } else {
            Ok(f184(f440(v3)?))
        }
    }
}

pub fn f338(v0: Bytes, v1: u64, v2: bool) -> R<u64> {
    let (_, h2370) = {
        let (h2371, _) = f211(Fn6::F441(v2, v0.clone()), {
            let a2373 = {
                let a2372 = v0;
                length_bytes(a2372)
            };
            let a2374 = 1u64;
            nat_add(a2373, a2374)?
        }, (v1, v1))?;
        h2371
    };
    Ok(h2370)
}

pub fn f339(v0: Bytes, p2399: u64, v2: u64, p2400: u64, p2401: Bytes) -> R<Adt23> {
    let m2398 = (p2399, (p2400, p2401));
    {
        let mut m2398 = m2398;
        loop {
            let step = {
                let (v1, (v3, v4)) = m2398;
                let m2375 = v3;
                if m2375 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C1(f129(Str::lit("a plain scalar has more lines than the input"), v1)?)))
                } else {
                    let v5 = m2375 - 1;
                    let v6: u64 = f244(v0.clone(), v1)?;
                    let v7: u64 = f182(v0.clone(), v6)?;
                    let a2376 = v7;
                    let a2377 = 58u64;
                    if nat_eq(a2376, a2377) {
                        Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C1(f442(v6)?)))
                    } else {
                        let a2378 = v7;
                        let a2379 = 35u64;
                        if if nat_eq(a2378, a2379) {
                            true
                        } else {
                            let a2380 = v7;
                            let a2381 = 256u64;
                            nat_eq(a2380, a2381)
                        } {
                            Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C0(v4, f245(v0.clone(), v6)?)))
                        } else {
                            let v8: (u64, u64) = f443(v0.clone(), {
                                let a2382 = v6;
                                let a2383 = 1u64;
                                nat_add(a2382, a2383)?
                            })?;
                            let v9: u64 = {
                                let (h2384, _) = v8;
                                h2384
                            };
                            let v10: u64 = f246(v0.clone(), v9)?;
                            let v11: u64 = f244(v0.clone(), v10)?;
                            let v12: u64 = f182(v0.clone(), v11)?;
                            let a2385 = v12;
                            let a2386 = 256u64;
                            if if if if nat_eq(a2385, a2386) {
                                true
                            } else {
                                let a2389 = {
                                    let a2387 = v10;
                                    let a2388 = v9;
                                    nat_sub(a2387, a2388)
                                };
                                let a2390 = v2;
                                nat_lt(a2389, a2390)
                            } {
                                true
                            } else {
                                let a2391 = v12;
                                let a2392 = 35u64;
                                nat_eq(a2391, a2392)
                            } {
                                true
                            } else {
                                f180(v0.clone(), v9)?
                            } {
                                Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C0(v4, {
                                    let a2393 = v6;
                                    let a2394 = 1u64;
                                    nat_add(a2393, a2394)?
                                })))
                            } else {
                                let v13: u64 = f338(v0.clone(), v11, false)?;
                                let a2395 = v13;
                                let a2396 = v11;
                                if nat_eq(a2395, a2396) {
                                    Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C1(f442(v11)?)))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Adt23>>((v13, (v5, f7(List::cons(v4, List::cons(f444({
                                        let (_, h2397) = v8;
                                        h2397
                                    }), List::cons(f258(v0.clone(), v11, v13), List::<Bytes>::nil())))))))
                                }
                            }
                        }
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

pub fn f340(v0: Bytes, v1: u64) -> R<u64> {
    let v2: bool = {
        let a2404 = f182(v0.clone(), {
            let a2402 = v1;
            let a2403 = 1u64;
            nat_add(a2402, a2403)?
        })?;
        let a2405 = 60u64;
        nat_eq(a2404, a2405)
    };
    let (h2406, _) = f176(Fn5::F445(v0.clone(), v2), {
        let a2408 = {
            let a2407 = v0;
            length_bytes(a2407)
        };
        let a2409 = 1u64;
        nat_add(a2408, a2409)?
    }, if v2 {
        let a2410 = v1;
        let a2411 = 2u64;
        nat_add(a2410, a2411)?
    } else {
        let a2412 = v1;
        let a2413 = 1u64;
        nat_add(a2412, a2413)?
    })?;
    Ok(h2406)
}

pub fn f341(v0: Bytes, v1: u64) -> R<Adt18> {
    let v2: u64 = f446(v0, List::cons(f8(Str::lit("str")), List::cons(f8(Str::lit("int")), List::cons(f8(Str::lit("float")), List::cons(f8(Str::lit("bool")), List::cons(f8(Str::lit("null")), List::cons(f8(Str::lit("seq")), List::cons(f8(Str::lit("map")), List::<Bytes>::nil()))))))), 1u64)?;
    let a2414 = v2;
    let a2415 = 0u64;
    if nat_eq(a2414, a2415) {
        Ok(Adt18::C2(f129(Str::lit("a tag outside the YAML 1.2.2 core schema is not read (YAML 1.2.2 section 10.3)"), v1)?))
    } else {
        Ok(Adt18::C0(v2))
    }
}

pub fn f342(v0: u64) -> R<Bytes> {
    f129(Str::lit("a node's content does not match its tag (YAML 1.2.2 section 10.3.2)"), v0)
}

pub fn f343(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f447(v0, v1, v2)
}

pub fn f344(v0: Bytes, v1: u64) -> R<bool> {
    let a2416 = f182(v0.clone(), v1)?;
    let a2417 = 58u64;
    if nat_eq(a2416, a2417) {
        if f241(f182(v0.clone(), {
            let a2418 = v1;
            let a2419 = 1u64;
            nat_add(a2418, a2419)?
        })?) {
            Ok(true)
        } else {
            Ok(f242(f182(v0, {
                let a2420 = v1;
                let a2421 = 1u64;
                nat_add(a2420, a2421)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f345(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a2422 = v0;
        utf8_encode(a2422)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f89(v1)?, List::<Bytes>::nil())))))
}

pub fn f346(p2425: List<Adt20>, p2426: List<Adt20>) -> List<Adt20> {
    let m2424 = (p2425, p2426);
    {
        let mut m2424 = m2424;
        loop {
            let step = {
                let (v0, v1) = m2424;
                let m2423 = v0;
                match m2423.uncons() {
                    None => {
                        Err::<(List<Adt20>, List<Adt20>), List<Adt20>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt20>, List<Adt20>), List<Adt20>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2424 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f347(v0: u64) -> R<bool> {
    f448(v0, f449())
}

pub fn f348(v0: u64) -> bool {
    let a2427 = 65u64;
    let a2428 = v0;
    if if nat_le(a2427, a2428) {
        let a2429 = v0;
        let a2430 = 90u64;
        nat_le(a2429, a2430)
    } else {
        false
    } {
        true
    } else {
        let a2431 = 97u64;
        let a2432 = v0;
        if nat_le(a2431, a2432) {
            let a2433 = v0;
            let a2434 = 122u64;
            nat_le(a2433, a2434)
        } else {
            false
        }
    }
}

pub fn f349(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h2435, _) = f176(Fn5::F450(v1, v0.clone()), {
        let a2437 = {
            let a2436 = v0;
            length_bytes(a2436)
        };
        let a2438 = 1u64;
        nat_add(a2437, a2438)?
    }, v2)?;
    Ok(h2435)
}

pub fn f350(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f323(v0.clone(), v1)?;
    let a2439 = v2;
    let a2440 = 48u64;
    if nat_eq(a2439, a2440) {
        let v3: u64 = f323(v0.clone(), {
            let a2441 = v1;
            let a2442 = 1u64;
            nat_add(a2441, a2442)?
        })?;
        let a2443 = v3;
        let a2444 = 88u64;
        if if if nat_eq(a2443, a2444) {
            true
        } else {
            let a2445 = v3;
            let a2446 = 120u64;
            nat_eq(a2445, a2446)
        } {
            f451(f323(v0.clone(), {
                let a2447 = v1;
                let a2448 = 2u64;
                nat_add(a2447, a2448)?
            })?)
        } else {
            false
        } {
            f349(v0, 1u64, {
                let a2449 = v1;
                let a2450 = 2u64;
                nat_add(a2449, a2450)?
            })
        } else {
            f349(v0, 2u64, {
                let a2451 = v1;
                let a2452 = 1u64;
                nat_add(a2451, a2452)?
            })
        }
    } else {
        if f452(v2) {
            f349(v0, 0u64, v1)
        } else {
            Ok(v1)
        }
    }
}

pub fn f351(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f349(v0.clone(), 0u64, v1)?;
    let a2453 = f323(v0.clone(), v2)?;
    let a2454 = 46u64;
    if nat_eq(a2453, a2454) {
        let v3: u64 = f349(v0.clone(), 0u64, {
            let a2455 = v2;
            let a2456 = 1u64;
            nat_add(a2455, a2456)?
        })?;
        let a2457 = v1;
        let a2458 = v2;
        if if nat_lt(a2457, a2458) {
            true
        } else {
            let a2461 = {
                let a2459 = v2;
                let a2460 = 1u64;
                nat_add(a2459, a2460)?
            };
            let a2462 = v3;
            nat_lt(a2461, a2462)
        } {
            f453(v0, v3)
        } else {
            Ok(v1)
        }
    } else {
        let a2463 = v1;
        let a2464 = v2;
        if nat_lt(a2463, a2464) {
            let v4: u64 = f453(v0, v2)?;
            let a2465 = v2;
            let a2466 = v4;
            if nat_lt(a2465, a2466) {
                Ok(v4)
            } else {
                Ok(v1)
            }
        } else {
            Ok(v1)
        }
    }
}

pub fn f352(v0: List<Adt20>, v1: Str) -> bool {
    let m2467 = v0;
    match m2467.uncons() {
        None => {
            false
        }
        Some((v2, _)) => {
            f454(v2, {
                let a2468 = v1;
                utf8_encode(a2468)
            })
        }
    }
}

pub fn f353(p2481: u64, p2482: List<Adt20>, p2483: List<Bytes>, v3: bool) -> Result<List<Adt20>, Bytes> {
    let m2480 = (p2481, (p2482, p2483));
    {
        let mut m2480 = m2480;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2480;
                let m2469 = v0;
                if m2469 == 0 {
                    Err::<(u64, (List<Adt20>, List<Bytes>)), Result<List<Adt20>, Bytes>>(Err::<List<Adt20>, Bytes>(f8(Str::lit("brackets nest deeper than the input is long"))))
                } else {
                    let v4 = m2469 - 1;
                    let m2470 = v1;
                    match m2470.uncons() {
                        None => {
                            Err::<(u64, (List<Adt20>, List<Bytes>)), Result<List<Adt20>, Bytes>>(Err::<List<Adt20>, Bytes>(f8(Str::lit("the input ends inside brackets or before the ; that closes a member (Web IDL, IDL grammar)"))))
                        }
                        Some((v5, v6)) => {
                            let a2471 = f455(v5.clone());
                            let a2472 = 3u64;
                            if nat_eq(a2471, a2472) {
                                Ok::<(u64, (List<Adt20>, List<Bytes>)), Result<List<Adt20>, Bytes>>((v4, (v6, v2)))
                            } else {
                                let v7: Bytes = f456(v5);
                                let v8: Bytes = f457(v7.clone());
                                if !f32(v8.clone(), Bytes::lit(&[])) {
                                    Ok::<(u64, (List<Adt20>, List<Bytes>)), Result<List<Adt20>, Bytes>>((v4, (v6, List::cons(v8, v2))))
                                } else {
                                    if f458(v7.clone()) {
                                        let m2473 = v2;
                                        match m2473.uncons() {
                                            None => {
                                                Err::<(u64, (List<Adt20>, List<Bytes>)), Result<List<Adt20>, Bytes>>(Err::<List<Adt20>, Bytes>(f7(List::cons(f8(Str::lit("the closing bracket ")), List::cons(v7, List::cons(f8(Str::lit(" has no opening bracket (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
                                            }
                                            Some((v9, v10)) => {
                                                if f32(v9.clone(), v7.clone()) {
                                                    if if !v3 {
                                                        let a2475 = {
                                                            let a2474 = v10.clone();
                                                            length_list(a2474)
                                                        };
                                                        let a2476 = 0u64;
                                                        nat_eq(a2475, a2476)
                                                    } else {
                                                        false
                                                    } {
                                                        Err::<(u64, (List<Adt20>, List<Bytes>)), Result<List<Adt20>, Bytes>>(Ok::<List<Adt20>, Bytes>(v6))
                                                    } else {
                                                        Ok::<(u64, (List<Adt20>, List<Bytes>)), Result<List<Adt20>, Bytes>>((v4, (v6, v10)))
                                                    }
                                                } else {
                                                    Err::<(u64, (List<Adt20>, List<Bytes>)), Result<List<Adt20>, Bytes>>(Err::<List<Adt20>, Bytes>(f7(List::cons(f8(Str::lit("the closing bracket ")), List::cons(v7, List::cons(f8(Str::lit(" does not match the opening bracket, which needs ")), List::cons(v9, List::cons(f8(Str::lit(" (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))))
                                                }
                                            }
                                        }
                                    } else {
                                        if if v3 {
                                            if f32(v7, f8(Str::lit(";"))) {
                                                let a2478 = {
                                                    let a2477 = v2.clone();
                                                    length_list(a2477)
                                                };
                                                let a2479 = 0u64;
                                                nat_eq(a2478, a2479)
                                            } else {
                                                false
                                            }
                                        } else {
                                            false
                                        } {
                                            Err::<(u64, (List<Adt20>, List<Bytes>)), Result<List<Adt20>, Bytes>>(Ok::<List<Adt20>, Bytes>(v6))
                                        } else {
                                            Ok::<(u64, (List<Adt20>, List<Bytes>)), Result<List<Adt20>, Bytes>>((v4, (v6, v2)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2480 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f354(v0: List<Adt20>) -> List<Adt20> {
    let m2484 = v0;
    match m2484.uncons() {
        None => {
            List::<Adt20>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f355(v0: u64, v1: u64, v2: List<Adt20>, v3: bool) -> R<Result<(Adt13, List<Adt20>), Bytes>> {
    let m2485 = f358(v2);
    match m2485 {
        Ok(v5) => {
            let v6: bool = if v3 {
                f352(f360(v5.clone()), Str::lit(":"))
            } else {
                false
            };
            let m2486 = if v6 {
                f358(f354(f360(v5.clone())))
            } else {
                Ok::<Adt24, Bytes>(Adt24::C0(Bytes::lit(&[]), f360(v5.clone())))
            };
            match m2486 {
                Ok(v8) => {
                    let m2487 = f359(f360(v8.clone()), Str::lit("{"));
                    match m2487 {
                        Ok(v10) => {
                            let m2492 = {
                                let a2488 = v1;
                                let a2489 = 2u64;
                                if if nat_eq(a2488, a2489) {
                                    true
                                } else {
                                    let a2490 = v1;
                                    let a2491 = 7u64;
                                    nat_eq(a2490, a2491)
                                } {
                                    f459(v0, v10, List::<Adt14>::nil())?
                                } else {
                                    f460(v0, v10, List::<Adt14>::nil())?
                                }
                            };
                            match m2492 {
                                Ok(v12) => {
                                    Ok(Ok::<(Adt13, List<Adt20>), Bytes>((Adt13::C0(v1, f362(v5), f362(v8), {
                                        let (h2493, _) = v12;
                                        h2493
                                    }, List::<Bytes>::nil(), f363(Bytes::lit(&[]))), {
                                        let (_, h2494) = v12;
                                        h2494
                                    })))
                                }
                                Err(v11) => {
                                    Ok(Err::<(Adt13, List<Adt20>), Bytes>(v11))
                                }
                            }
                        }
                        Err(v9) => {
                            Ok(Err::<(Adt13, List<Adt20>), Bytes>(v9))
                        }
                    }
                }
                Err(v7) => {
                    Ok(Err::<(Adt13, List<Adt20>), Bytes>(v7))
                }
            }
        }
        Err(v4) => {
            Ok(Err::<(Adt13, List<Adt20>), Bytes>(v4))
        }
    }
}

pub fn f356(v0: List<Adt20>, v1: Bytes) -> R<Result<(Adt13, List<Adt20>), Bytes>> {
    let m2495 = f358(v0);
    match m2495 {
        Ok(v3) => {
            let m2496 = f461(f360(v3.clone()))?;
            match m2496 {
                Ok(v5) => {
                    Ok(Ok::<(Adt13, List<Adt20>), Bytes>((Adt13::C0(9u64, f362(v3), Bytes::lit(&[]), List::<Adt14>::nil(), List::cons(v1, List::<Bytes>::nil()), f363(Bytes::lit(&[]))), v5)))
                }
                Err(v4) => {
                    Ok(Err::<(Adt13, List<Adt20>), Bytes>(v4))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<(Adt13, List<Adt20>), Bytes>(v2))
        }
    }
}

pub fn f357(v0: Str, v1: List<Adt20>) -> Bytes {
    f7(List::cons(f8(Str::lit("expected ")), List::cons({
        let a2497 = v0;
        utf8_encode(a2497)
    }, List::cons(f8(Str::lit(" but found ")), List::cons(f462(v1), List::cons(f8(Str::lit(" (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
}

pub fn f358(v0: List<Adt20>) -> Result<Adt24, Bytes> {
    let m2498 = v0.clone();
    match m2498.uncons() {
        None => {
            Err::<Adt24, Bytes>(f357(Str::lit("an identifier"), v0))
        }
        Some((v1, v2)) => {
            let a2499 = f455(v1.clone());
            let a2500 = 0u64;
            if nat_eq(a2499, a2500) {
                Ok::<Adt24, Bytes>(Adt24::C0(f463(f456(v1)), v2))
            } else {
                Err::<Adt24, Bytes>(f357(Str::lit("an identifier"), v0))
            }
        }
    }
}

pub fn f359(v0: List<Adt20>, v1: Str) -> Result<List<Adt20>, Bytes> {
    if f352(v0.clone(), v1.clone()) {
        Ok::<List<Adt20>, Bytes>(f354(v0))
    } else {
        Err::<List<Adt20>, Bytes>(f357(v1, v0))
    }
}

pub fn f360(v0: Adt24) -> List<Adt20> {
    let m2501 = v0;
    match m2501 {
        Adt24::C0(_, v2) => {
            v2
        }
    }
}

pub fn f361(p2509: u64, p2510: List<Adt20>, p2511: List<Bytes>) -> Result<(List<Bytes>, List<Adt20>), Bytes> {
    let m2508 = (p2509, (p2510, p2511));
    {
        let mut m2508 = m2508;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2508;
                let m2502 = v0;
                if m2502 == 0 {
                    Err::<(u64, (List<Adt20>, List<Bytes>)), Result<(List<Bytes>, List<Adt20>), Bytes>>(Err::<(List<Bytes>, List<Adt20>), Bytes>(f8(Str::lit("an enumeration has more values than the input has tokens"))))
                } else {
                    let v3 = m2502 - 1;
                    let m2503 = v1.clone();
                    match m2503.uncons() {
                        None => {
                            Err::<(u64, (List<Adt20>, List<Bytes>)), Result<(List<Bytes>, List<Adt20>), Bytes>>(Err::<(List<Bytes>, List<Adt20>), Bytes>(f357(Str::lit("a string"), v1)))
                        }
                        Some((v4, v5)) => {
                            let a2504 = f455(v4.clone());
                            let a2505 = 3u64;
                            if !nat_eq(a2504, a2505) {
                                Err::<(u64, (List<Adt20>, List<Bytes>)), Result<(List<Bytes>, List<Adt20>), Bytes>>(Err::<(List<Bytes>, List<Adt20>), Bytes>(f357(Str::lit("a string"), v1)))
                            } else {
                                if f352(v5.clone(), Str::lit(",")) {
                                    if f352(f354(v5.clone()), Str::lit("}")) {
                                        let m2506 = f359(f354(f354(v5)), Str::lit(";"));
                                        match m2506 {
                                            Ok(v7) => {
                                                Err::<(u64, (List<Adt20>, List<Bytes>)), Result<(List<Bytes>, List<Adt20>), Bytes>>(Ok::<(List<Bytes>, List<Adt20>), Bytes>((f66(List::cons(f456(v4), v2)), v7)))
                                            }
                                            Err(v6) => {
                                                Err::<(u64, (List<Adt20>, List<Bytes>)), Result<(List<Bytes>, List<Adt20>), Bytes>>(Err::<(List<Bytes>, List<Adt20>), Bytes>(v6))
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (List<Adt20>, List<Bytes>)), Result<(List<Bytes>, List<Adt20>), Bytes>>((v3, (f354(v5), List::cons(f456(v4), v2))))
                                    }
                                } else {
                                    if f352(v5.clone(), Str::lit("}")) {
                                        let m2507 = f359(f354(v5), Str::lit(";"));
                                        match m2507 {
                                            Ok(v9) => {
                                                Err::<(u64, (List<Adt20>, List<Bytes>)), Result<(List<Bytes>, List<Adt20>), Bytes>>(Ok::<(List<Bytes>, List<Adt20>), Bytes>((f66(List::cons(f456(v4), v2)), v9)))
                                            }
                                            Err(v8) => {
                                                Err::<(u64, (List<Adt20>, List<Bytes>)), Result<(List<Bytes>, List<Adt20>), Bytes>>(Err::<(List<Bytes>, List<Adt20>), Bytes>(v8))
                                            }
                                        }
                                    } else {
                                        Err::<(u64, (List<Adt20>, List<Bytes>)), Result<(List<Bytes>, List<Adt20>), Bytes>>(Err::<(List<Bytes>, List<Adt20>), Bytes>(f357(Str::lit(", or }"), v5)))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2508 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f362(v0: Adt24) -> Bytes {
    let m2512 = v0;
    match m2512 {
        Adt24::C0(v1, _) => {
            v1
        }
    }
}

pub fn f363(v0: Bytes) -> Adt15 {
    Adt15::C0(0u64, v0, List::<Bytes>::nil(), Bytes::lit(&[]), Bytes::lit(&[]))
}

pub fn f364(v0: u64, v1: List<Adt20>, v2: u64) -> R<Result<Adt25, Bytes>> {
    let m2513 = v0;
    if m2513 == 0 {
        Ok(Err::<Adt25, Bytes>(f8(Str::lit("a type nests deeper than the input is long"))))
    } else {
        let v3 = m2513 - 1;
        let a2514 = v2;
        let a2515 = 0u64;
        if nat_eq(a2514, a2515) {
            let m2516 = f266(v1)?;
            match m2516 {
                Ok(v5) => {
                    if f352(v5.clone(), Str::lit("(")) {
                        let m2517 = f364(v3, f354(v5), 1u64)?;
                        match m2517 {
                            Ok(v7) => {
                                if f352(f365(v7.clone()), Str::lit("?")) {
                                    Ok(Ok::<Adt25, Bytes>(Adt25::C0(Adt15::C0(0u64, Bytes::lit(&[]), f286(f366(v7.clone())), f8(Str::lit("a nullable union type is not translated yet, because the typedef identifier names only the union (Web IDL 2.13.32)")), Bytes::lit(&[])), f354(f365(v7)))))
                                } else {
                                    Ok(Ok::<Adt25, Bytes>(Adt25::C0(f366(v7.clone()), f365(v7))))
                                }
                            }
                            Err(v6) => {
                                Ok(Err::<Adt25, Bytes>(v6))
                            }
                        }
                    } else {
                        let m2518 = v5.clone();
                        match m2518.uncons() {
                            None => {
                                Ok(Err::<Adt25, Bytes>(f357(Str::lit("a type"), v5)))
                            }
                            Some((v8, v9)) => {
                                let v10: Bytes = f456(v8.clone());
                                let a2519 = f455(v8);
                                let a2520 = 0u64;
                                if !nat_eq(a2519, a2520) {
                                    Ok(Err::<Adt25, Bytes>(f357(Str::lit("a type"), v5)))
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
                                        let m2521 = f359(v9, Str::lit("<"));
                                        match m2521 {
                                            Ok(v12) => {
                                                let m2522 = f364(v3, v12, 0u64)?;
                                                match m2522 {
                                                    Ok(v14) => {
                                                        let m2523 = f359(f365(v14.clone()), Str::lit(">"));
                                                        match m2523 {
                                                            Ok(v16) => {
                                                                Ok(f464(f465(Str::lit("List"), f366(v14)), v16))
                                                            }
                                                            Err(v15) => {
                                                                Ok(Err::<Adt25, Bytes>(v15))
                                                            }
                                                        }
                                                    }
                                                    Err(v13) => {
                                                        Ok(Err::<Adt25, Bytes>(v13))
                                                    }
                                                }
                                            }
                                            Err(v11) => {
                                                Ok(Err::<Adt25, Bytes>(v11))
                                            }
                                        }
                                    } else {
                                        if f32(v10.clone(), f8(Str::lit("async_sequence"))) {
                                            let m2524 = f359(v9, Str::lit("<"));
                                            match m2524 {
                                                Ok(v18) => {
                                                    let m2525 = f364(v3, v18, 0u64)?;
                                                    match m2525 {
                                                        Ok(v20) => {
                                                            let m2526 = f359(f365(v20), Str::lit(">"));
                                                            match m2526 {
                                                                Ok(v22) => {
                                                                    Ok(f464(f466(f8(Str::lit("async_sequence types are not translated, because an async sequence is produced over time and is not a value (Web IDL 2.13.29)"))), v22))
                                                                }
                                                                Err(v21) => {
                                                                    Ok(Err::<Adt25, Bytes>(v21))
                                                                }
                                                            }
                                                        }
                                                        Err(v19) => {
                                                            Ok(Err::<Adt25, Bytes>(v19))
                                                        }
                                                    }
                                                }
                                                Err(v17) => {
                                                    Ok(Err::<Adt25, Bytes>(v17))
                                                }
                                            }
                                        } else {
                                            if f32(v10.clone(), f8(Str::lit("Promise"))) {
                                                let m2527 = f359(v9, Str::lit("<"));
                                                match m2527 {
                                                    Ok(v24) => {
                                                        let m2528 = f364(v3, v24, 0u64)?;
                                                        match m2528 {
                                                            Ok(v26) => {
                                                                let m2529 = f359(f365(v26), Str::lit(">"));
                                                                match m2529 {
                                                                    Ok(v28) => {
                                                                        Ok(f464(f466(f8(Str::lit("Promise types are not translated, because a promise is a deferred result, not a value (Web IDL 2.13.31)"))), v28))
                                                                    }
                                                                    Err(v27) => {
                                                                        Ok(Err::<Adt25, Bytes>(v27))
                                                                    }
                                                                }
                                                            }
                                                            Err(v25) => {
                                                                Ok(Err::<Adt25, Bytes>(v25))
                                                            }
                                                        }
                                                    }
                                                    Err(v23) => {
                                                        Ok(Err::<Adt25, Bytes>(v23))
                                                    }
                                                }
                                            } else {
                                                if f32(v10.clone(), f8(Str::lit("record"))) {
                                                    let m2530 = f359(v9, Str::lit("<"));
                                                    match m2530 {
                                                        Ok(v30) => {
                                                            let m2531 = v30.clone();
                                                            match m2531.uncons() {
                                                                None => {
                                                                    Ok(Err::<Adt25, Bytes>(f357(Str::lit("a string type"), v30)))
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
                                                                        let m2532 = f359(v32, Str::lit(","));
                                                                        match m2532 {
                                                                            Ok(v34) => {
                                                                                let m2533 = f364(v3, v34, 0u64)?;
                                                                                match m2533 {
                                                                                    Ok(v36) => {
                                                                                        let m2534 = f359(f365(v36.clone()), Str::lit(">"));
                                                                                        match m2534 {
                                                                                            Ok(v38) => {
                                                                                                let m2535 = f467(f456(v31));
                                                                                                match m2535 {
                                                                                                    None => {
                                                                                                        Ok(Err::<Adt25, Bytes>(f357(Str::lit("a string type"), v30)))
                                                                                                    }
                                                                                                    Some(v39) => {
                                                                                                        let v40: Adt15 = f468(f366(v36));
                                                                                                        Ok(f464(Adt15::C0(0u64, f7(List::cons(f8(Str::lit("(List (")), List::cons(f285(v39), List::cons(f8(Str::lit(" * ")), List::cons(f285(v40.clone()), List::cons(f8(Str::lit("))")), List::<Bytes>::nil())))))), f286(v40.clone()), f283(v40), Bytes::lit(&[])), v38))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            Err(v37) => {
                                                                                                Ok(Err::<Adt25, Bytes>(v37))
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    Err(v35) => {
                                                                                        Ok(Err::<Adt25, Bytes>(v35))
                                                                                    }
                                                                                }
                                                                            }
                                                                            Err(v33) => {
                                                                                Ok(Err::<Adt25, Bytes>(v33))
                                                                            }
                                                                        }
                                                                    } else {
                                                                        Ok(Err::<Adt25, Bytes>(f357(Str::lit("DOMString, USVString or ByteString"), v30)))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Err(v29) => {
                                                            Ok(Err::<Adt25, Bytes>(v29))
                                                        }
                                                    }
                                                } else {
                                                    if f32(v10.clone(), f8(Str::lit("unsigned"))) {
                                                        if f352(v9.clone(), Str::lit("short")) {
                                                            Ok(f464(f363(f8(Str::lit("Nat"))), f354(v9)))
                                                        } else {
                                                            if f352(v9.clone(), Str::lit("long")) {
                                                                let v41: List<Adt20> = f354(v9);
                                                                Ok(f464(f363(f8(Str::lit("Nat"))), if f352(v41.clone(), Str::lit("long")) {
                                                                    f354(v41)
                                                                } else {
                                                                    v41
                                                                }))
                                                            } else {
                                                                Ok(Err::<Adt25, Bytes>(f357(Str::lit("short or long"), v9)))
                                                            }
                                                        }
                                                    } else {
                                                        if f32(v10.clone(), f8(Str::lit("short"))) {
                                                            Ok(f464(f363(f8(Str::lit("Int"))), v9))
                                                        } else {
                                                            if f32(v10.clone(), f8(Str::lit("long"))) {
                                                                Ok(f464(f363(f8(Str::lit("Int"))), if f352(v9.clone(), Str::lit("long")) {
                                                                    f354(v9)
                                                                } else {
                                                                    v9
                                                                }))
                                                            } else {
                                                                if f32(v10.clone(), f8(Str::lit("unrestricted"))) {
                                                                    if f352(v9.clone(), Str::lit("float")) {
                                                                        Ok(f464(f469(false), f354(v9)))
                                                                    } else {
                                                                        if f352(v9.clone(), Str::lit("double")) {
                                                                            Ok(f464(f469(true), f354(v9)))
                                                                        } else {
                                                                            Ok(Err::<Adt25, Bytes>(f357(Str::lit("float or double"), v9)))
                                                                        }
                                                                    }
                                                                } else {
                                                                    let m2536 = f467(v10.clone());
                                                                    match m2536 {
                                                                        None => {
                                                                            let v42: Bytes = f463(v10);
                                                                            Ok(f464(Adt15::C0(0u64, f269(v42.clone())?, List::cons(v42.clone(), List::<Bytes>::nil()), Bytes::lit(&[]), v42), v9))
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
                    Ok(Err::<Adt25, Bytes>(v4))
                }
            }
        } else {
            let m2537 = f364(v3, v1, 0u64)?;
            match m2537 {
                Ok(v45) => {
                    let v46: Adt15 = f366(v45.clone());
                    let v48: Adt15 = {
                        let a2538 = f284(v46.clone());
                        let a2539 = 0u64;
                        if nat_eq(a2538, a2539) {
                            if f32(f283(v46.clone()), Bytes::lit(&[])) {
                                if f32(f470(v46.clone()), Bytes::lit(&[])) {
                                    Adt15::C0(1u64, Bytes::lit(&[]), f286(v46), f8(Str::lit("a union member type that is not a named definition gives its constructor no name, so the union is not translated yet (Web IDL 2.13.32)")), Bytes::lit(&[]))
                                } else {
                                    let m2540 = f376(false, f470(v46.clone()))?;
                                    match m2540 {
                                        None => {
                                            Adt15::C0(1u64, Bytes::lit(&[]), f286(v46.clone()), f7(List::cons(f8(Str::lit("the union member ")), List::cons(f470(v46), List::cons(f8(Str::lit(" has no constructor name")), List::<Bytes>::nil())))), Bytes::lit(&[]))
                                        }
                                        Some(v47) => {
                                            Adt15::C0(1u64, f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v47, List::cons(f8(Str::lit(", fields: [")), List::cons(f285(v46.clone()), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f286(v46), Bytes::lit(&[]), Bytes::lit(&[]))
                                        }
                                    }
                                }
                            } else {
                                Adt15::C0(1u64, Bytes::lit(&[]), f286(v46.clone()), f283(v46), Bytes::lit(&[]))
                            }
                        } else {
                            v46
                        }
                    };
                    if f352(f365(v45.clone()), Str::lit("or")) {
                        let m2541 = f364(v3, f354(f365(v45)), 2u64)?;
                        match m2541 {
                            Ok(v50) => {
                                Ok(Ok::<Adt25, Bytes>(Adt25::C0(Adt15::C0(1u64, if f32(f285(v48.clone()), Bytes::lit(&[])) {
                                    f285(f366(v50.clone()))
                                } else {
                                    if f32(f285(f366(v50.clone())), Bytes::lit(&[])) {
                                        f285(v48.clone())
                                    } else {
                                        f7(List::cons(f285(v48.clone()), List::cons(f8(Str::lit(",\u{a}")), List::cons(f285(f366(v50.clone())), List::<Bytes>::nil()))))
                                    }
                                }, f197(f286(v48.clone()), f286(f366(v50.clone()))), if f32(f283(v48.clone()), Bytes::lit(&[])) {
                                    f283(f366(v50.clone()))
                                } else {
                                    f283(v48)
                                }, Bytes::lit(&[])), f365(v50))))
                            }
                            Err(v49) => {
                                Ok(Err::<Adt25, Bytes>(v49))
                            }
                        }
                    } else {
                        let a2542 = v2;
                        let a2543 = 2u64;
                        if if nat_eq(a2542, a2543) {
                            f352(f365(v45.clone()), Str::lit(")"))
                        } else {
                            false
                        } {
                            Ok(Ok::<Adt25, Bytes>(Adt25::C0(v48, f354(f365(v45)))))
                        } else {
                            Ok(Err::<Adt25, Bytes>({
                                let a2544 = v2;
                                let a2545 = 1u64;
                                if nat_eq(a2544, a2545) {
                                    f357(Str::lit("or"), f365(v45))
                                } else {
                                    f357(Str::lit("or or )"), f365(v45))
                                }
                            }))
                        }
                    }
                }
                Err(v44) => {
                    Ok(Err::<Adt25, Bytes>(v44))
                }
            }
        }
    }
}

pub fn f365(v0: Adt25) -> List<Adt20> {
    let m2546 = v0;
    match m2546 {
        Adt25::C0(_, v2) => {
            v2
        }
    }
}

pub fn f366(v0: Adt25) -> Adt15 {
    let m2547 = v0;
    match m2547 {
        Adt25::C0(v1, _) => {
            v1
        }
    }
}

pub fn f367() -> List<Bytes> {
    List::cons(f8(Str::lit("Bool")), List::cons(f8(Str::lit("ContractViolation")), List::cons(f8(Str::lit("List")), List::cons(f8(Str::lit("Nat")), List::cons(f8(Str::lit("Option")), List::cons(f8(Str::lit("Prod")), List::cons(f8(Str::lit("ReasoningFailure")), List::cons(f8(Str::lit("Result")), List::<Bytes>::nil()))))))))
}

pub fn f368(v0: Bytes) -> R<Bytes> {
    let m2550 = {
        let a2548 = v0.clone();
        let a2549 = 0u64;
        index_bytes(a2548, a2549)
    };
    match m2550 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            let a2556 = {
                let a2551 = f8(Str::lit("u"));
                let a2552 = f471(8u64, f60(v1)?, 4u64, Bytes::lit(&[]));
                append_bytes(a2551, a2552)
            };
            let a2557 = f46(v0.clone(), 1u64, {
                let a2554 = {
                    let a2553 = v0;
                    length_bytes(a2553)
                };
                let a2555 = 1u64;
                nat_sub(a2554, a2555)
            });
            Ok(append_bytes(a2556, a2557))
        }
    }
}

pub fn f369(v0: List<Adt14>) -> R<List<Adt14>> {
    let m2558 = v0;
    match m2558.uncons() {
        None => {
            Ok(List::<Adt14>::nil())
        }
        Some((v1, v2)) => {
            f472(v1, f369(v2)?)
        }
    }
}

pub fn f370(v0: List<Adt14>, v1: List<Adt14>) -> List<Adt14> {
    let m2559 = v0;
    match m2559.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f370(v3, v1))
        }
    }
}

pub fn f371(v0: Adt13) -> List<Adt14> {
    let m2560 = v0;
    match m2560 {
        Adt13::C0(_, _, _, v4, _, _) => {
            v4
        }
    }
}

pub fn f372(v0: List<Adt13>, v1: u64, v2: Bytes) -> List<Adt14> {
    let m2561 = v0;
    match m2561.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v3, v4)) => {
            let a2562 = f192(v3.clone());
            let a2563 = v1;
            if if nat_eq(a2562, a2563) {
                f32(f194(v3.clone()), v2.clone())
            } else {
                false
            } {
                f370(f371(v3), f372(v4, v1, v2))
            } else {
                f372(v4, v1, v2)
            }
        }
    }
}

pub fn f373(v0: List<Adt13>, v1: List<Adt13>, v2: Bytes) -> List<Adt14> {
    let m2564 = v0;
    match m2564.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v3, v4)) => {
            let a2565 = f192(v3.clone());
            let a2566 = 8u64;
            if if nat_eq(a2565, a2566) {
                f32(f194(v3.clone()), v2.clone())
            } else {
                false
            } {
                f370(f473(v1.clone(), f204(v3)), f373(v4, v1, v2))
            } else {
                f373(v4, v1, v2)
            }
        }
    }
}

pub fn f374(v0: Adt14) -> Adt15 {
    let m2567 = v0;
    match m2567 {
        Adt14::C0(_, v2, _) => {
            v2
        }
    }
}

pub fn f375(v0: Adt14) -> Bytes {
    let m2568 = v0;
    match m2568 {
        Adt14::C0(v1, _, _) => {
            v1
        }
    }
}

pub fn f376(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f474(v1)
    } else {
        f379(v1)
    }
}

pub fn f377(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m2569 = v1;
    match m2569.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m2570 = f376(v0, v2)?;
            match m2570 {
                None => {
                    f377(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f377(v0, v3)?))
                }
            }
        }
    }
}

pub fn f378(v0: Adt14) -> u64 {
    let m2571 = v0;
    match m2571 {
        Adt14::C0(_, _, v3) => {
            v3
        }
    }
}

pub fn f379(v0: Bytes) -> R<Option<Bytes>> {
    let m2572 = f475(v0)?;
    match m2572 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            Ok(Some(f476(v1)?))
        }
    }
}

pub fn f380(v0: bool, v1: Adt2) -> R<Option<Bytes>> {
    let m2573 = v1;
    match m2573 {
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
                let a2584 = if f105(v9, {
                    let a2582 = 0u64;
                    let a2583 = v10;
                    nat_lt(a2582, a2583)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a2585 = f223(false, v10)?;
                append_bytes(a2584, a2585)
            }))
        }
        Adt2::C4(v3, v4, v5) => {
            let m2576 = f474({
                let a2574 = f8(Str::lit("n"));
                let a2575 = f224(false, v4, v5)?;
                append_bytes(a2574, a2575)
            })?;
            match m2576 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a2580 = f8(Str::lit("nMinus"));
                        let a2581 = f46(v6.clone(), 1u64, {
                            let a2578 = {
                                let a2577 = v6;
                                length_bytes(a2577)
                            };
                            let a2579 = 1u64;
                            nat_sub(a2578, a2579)
                        });
                        append_bytes(a2580, a2581)
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
            f376(v0, v2)
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

pub fn f381(v0: Adt2) -> R<Bytes> {
    f168(v0)
}

pub fn f382(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2586 = v0.clone();
        length_bytes(a2586)
    };
    let a2590 = {
        let a2587 = v1;
        let a2588 = 2u64;
        let a2589 = 0u64;
        nat_rem(a2587, a2588, a2589)
    };
    let a2591 = 0u64;
    if nat_eq(a2590, a2591) {
        let (_, h2592) = {
            let (h2593, _) = f477(Fn14::F478(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2593
        };
        Ok(h2592)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f383(v0: u64) -> bool {
    let a2594 = v0;
    let a2595 = 8u64;
    if nat_eq(a2594, a2595) {
        true
    } else {
        let a2596 = v0;
        let a2597 = 9u64;
        if nat_eq(a2596, a2597) {
            true
        } else {
            let a2598 = v0;
            let a2599 = 10u64;
            if nat_eq(a2598, a2599) {
                true
            } else {
                let a2600 = v0;
                let a2601 = 12u64;
                if nat_eq(a2600, a2601) {
                    true
                } else {
                    let a2602 = v0;
                    let a2603 = 13u64;
                    nat_eq(a2602, a2603)
                }
            }
        }
    }
}

pub fn f384(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2606 = {
        let a2604 = v0;
        let a2605 = v1;
        index_bytes(a2604, a2605)
    };
    match m2606 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f479(v2)? {
                Ok(Some({
                    let a2607 = v1;
                    let a2608 = 1u64;
                    nat_add(a2607, a2608)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f385(v0: u8) -> R<u64> {
    let a2655 = {
        let v1: u8 = v0;
        let a2653 = {
            let a2629 = {
                let a2617 = {
                    let a2611 = {
                        let a2609 = v1;
                        let a2610 = 128u8;
                        fixed_u8::bit_and(a2609, a2610)
                    };
                    let a2612 = 128u8;
                    if equal(a2611, a2612) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2618 = {
                    let a2615 = {
                        let a2613 = v1;
                        let a2614 = 64u8;
                        fixed_u8::bit_and(a2613, a2614)
                    };
                    let a2616 = 64u8;
                    if equal(a2615, a2616) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2617, a2618)?
            };
            let a2630 = {
                let a2627 = {
                    let a2621 = {
                        let a2619 = v1;
                        let a2620 = 32u8;
                        fixed_u8::bit_and(a2619, a2620)
                    };
                    let a2622 = 32u8;
                    if equal(a2621, a2622) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2628 = {
                    let a2625 = {
                        let a2623 = v1;
                        let a2624 = 16u8;
                        fixed_u8::bit_and(a2623, a2624)
                    };
                    let a2626 = 16u8;
                    if equal(a2625, a2626) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2627, a2628)?
            };
            nat_add(a2629, a2630)?
        };
        let a2654 = {
            let a2651 = {
                let a2639 = {
                    let a2633 = {
                        let a2631 = v1;
                        let a2632 = 8u8;
                        fixed_u8::bit_and(a2631, a2632)
                    };
                    let a2634 = 8u8;
                    if equal(a2633, a2634) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2640 = {
                    let a2637 = {
                        let a2635 = v1;
                        let a2636 = 4u8;
                        fixed_u8::bit_and(a2635, a2636)
                    };
                    let a2638 = 4u8;
                    if equal(a2637, a2638) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2639, a2640)?
            };
            let a2652 = {
                let a2649 = {
                    let a2643 = {
                        let a2641 = v1;
                        let a2642 = 2u8;
                        fixed_u8::bit_and(a2641, a2642)
                    };
                    let a2644 = 2u8;
                    if equal(a2643, a2644) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2650 = {
                    let a2647 = {
                        let a2645 = v1;
                        let a2646 = 1u8;
                        fixed_u8::bit_and(a2645, a2646)
                    };
                    let a2648 = 1u8;
                    if equal(a2647, a2648) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2649, a2650)?
            };
            nat_add(a2651, a2652)?
        };
        nat_add(a2653, a2654)?
    };
    let a2656 = 48u64;
    Ok(nat_sub(a2655, a2656))
}

pub fn f386(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a2657 = v0.clone();
        length_bytes(a2657)
    };
    let a2658 = v1;
    let a2659 = v2;
    if if nat_lt(a2658, a2659) {
        let a2664 = f103(v0, {
            let a2662 = v2;
            let a2663 = {
                let a2660 = v1;
                let a2661 = 1u64;
                nat_add(a2660, a2661)?
            };
            nat_sub(a2662, a2663)
        })?;
        let a2665 = 48u64;
        nat_eq(a2664, a2665)
    } else {
        false
    } {
        Ok(Some({
            let a2666 = v1;
            let a2667 = 1u64;
            nat_add(a2666, a2667)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f387(v0: Adt22) -> R<Adt22> {
    let v1: u32 = f480(match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, h2668) => {
            h2668
        }
    });
    let v2: u32 = f299(match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, h2669, _, _) => {
            h2669
        }
    }, f299(f481(match v0.clone() {
        Adt22::C0(_, _, _, _, h2670, _, _, _, _, _) => {
            h2670
        }
    }), f299(f482(match v0.clone() {
        Adt22::C0(_, _, _, _, h2671, _, _, _, _, _) => {
            h2671
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, h2672, _, _, _, _) => {
            h2672
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, h2673, _, _, _) => {
            h2673
        }
    }), f299(v1, {
        let m2676 = match v0.clone() {
            Adt22::C0(_, _, _, _, _, _, _, _, h2675, _) => {
                h2675
            }
        };
        match m2676 {
            Adt21::C0(h2674, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2674
            }
        }
    }))));
    let v3: u32 = f299(f483(match v0.clone() {
        Adt22::C0(h2677, _, _, _, _, _, _, _, _, _) => {
            h2677
        }
    }), f484(match v0.clone() {
        Adt22::C0(h2678, _, _, _, _, _, _, _, _, _) => {
            h2678
        }
    }, match v0.clone() {
        Adt22::C0(_, h2679, _, _, _, _, _, _, _, _) => {
            h2679
        }
    }, match v0.clone() {
        Adt22::C0(_, _, h2680, _, _, _, _, _, _, _) => {
            h2680
        }
    }));
    Ok(Adt22::C0(f299(v2, v3), match v0.clone() {
        Adt22::C0(h2681, _, _, _, _, _, _, _, _, _) => {
            h2681
        }
    }, match v0.clone() {
        Adt22::C0(_, h2682, _, _, _, _, _, _, _, _) => {
            h2682
        }
    }, match v0.clone() {
        Adt22::C0(_, _, h2683, _, _, _, _, _, _, _) => {
            h2683
        }
    }, f299(match v0.clone() {
        Adt22::C0(_, _, _, h2684, _, _, _, _, _, _) => {
            h2684
        }
    }, v2), match v0.clone() {
        Adt22::C0(_, _, _, _, h2685, _, _, _, _, _) => {
            h2685
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, h2686, _, _, _, _) => {
            h2686
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, h2687, _, _, _) => {
            h2687
        }
    }, f485(match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, h2688, _) => {
            h2688
        }
    }), {
        let a2690 = match v0 {
            Adt22::C0(_, _, _, _, _, _, _, _, _, h2689) => {
                h2689
            }
        };
        let a2691 = 1u64;
        nat_add(a2690, a2691)?
    }))
}

pub fn f388(v0: u32) -> u64 {
    let m2693 = {
        let a2692 = v0;
        fixed_u64::convert(i128::from(a2692))
    };
    match m2693 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f389(v0: u64) -> u32 {
    let m2697 = {
        let a2696 = {
            let a2694 = v0;
            let a2695 = 4294967295u64;
            fixed_u64::bit_and(a2694, a2695)
        };
        fixed_u32::convert(i128::from(a2696))
    };
    match m2697 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f390(v0: u64) -> u32 {
    let (_, h2698) = {
        let (_, h2699) = f486(Fn15::F487, 32u64, (v0, (1u32, 0u32)));
        h2699
    };
    h2698
}

pub fn f391(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a2704 = f488(v0, 24u32);
    let a2705 = {
        let a2702 = f488(v1, 16u32);
        let a2703 = {
            let a2700 = f488(v2, 8u32);
            let a2701 = v3;
            fixed_u32::bit_or(a2700, a2701)
        };
        fixed_u32::bit_or(a2702, a2703)
    };
    fixed_u32::bit_or(a2704, a2705)
}

pub fn f392(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a2706 = v2;
    let a2707 = v1;
    if nat_lt(a2706, a2707) {
        let m2710 = {
            let a2708 = v0;
            let a2709 = v2;
            index_bytes(a2708, a2709)
        };
        match m2710 {
            None => {
                0u32
            }
            Some(v3) => {
                let m2712 = {
                    let a2711 = v3;
                    fixed_u32::convert(i128::from(a2711))
                };
                match m2712 {
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
        let a2713 = v2;
        let a2714 = v1;
        if nat_eq(a2713, a2714) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f393(v0: List<u64>) -> R<Bytes> {
    let m2715 = f489(v0, List::<u64>::nil());
    match m2715.uncons() {
        None => {
            Ok(Bytes::lit(&[0x30]))
        }
        Some((v1, v2)) => {
            let a2716 = f89(v1)?;
            let a2717 = f490(v2)?;
            Ok(append_bytes(a2716, a2717))
        }
    }
}

pub fn f394(p2720: u64, p2721: List<u64>, v2: u64) -> R<List<u64>> {
    let m2719 = (p2720, p2721);
    {
        let mut m2719 = m2719;
        loop {
            let step = {
                let (v0, v1) = m2719;
                let m2718 = v0;
                if m2718 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v1))
                } else {
                    let v3 = m2718 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, f491(v1, v2, 0u64)?))
                }
            };
            match step {
                Ok(next) => { m2719 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f395(v0: u64, v1: u64) -> List<u64> {
    let m2722 = v0;
    if m2722 == 0 {
        List::<u64>::nil()
    } else {
        let v2 = m2722 - 1;
        let a2723 = v1;
        let a2724 = 0u64;
        if nat_eq(a2723, a2724) {
            List::<u64>::nil()
        } else {
            List::cons({
                let a2725 = v1;
                let a2726 = f492();
                let a2727 = 0u64;
                nat_rem(a2725, a2726, a2727)
            }, f395(v2, {
                let a2728 = v1;
                let a2729 = f492();
                let a2730 = 0u64;
                nat_quot(a2728, a2729, a2730)
            }))
        }
    }
}

pub fn f396(p2741: Bytes, p2742: u64) -> R<Bytes> {
    let m2740 = (p2741, p2742);
    {
        let mut m2740 = m2740;
        loop {
            let step = {
                let (v0, v1) = m2740;
                let m2731 = v1;
                if m2731 == 0 {
                    Err::<(Bytes, u64), R<Bytes>>(Ok(v0))
                } else {
                    let v2 = m2731 - 1;
                    let a2735 = f103(v0.clone(), {
                        let a2733 = {
                            let a2732 = v0.clone();
                            length_bytes(a2732)
                        };
                        let a2734 = 1u64;
                        nat_sub(a2733, a2734)
                    })?;
                    let a2736 = 48u64;
                    if nat_eq(a2735, a2736) {
                        Ok::<(Bytes, u64), R<Bytes>>((f46(v0.clone(), 0u64, {
                            let a2738 = {
                                let a2737 = v0;
                                length_bytes(a2737)
                            };
                            let a2739 = 1u64;
                            nat_sub(a2738, a2739)
                        }), v2))
                    } else {
                        Err::<(Bytes, u64), R<Bytes>>(Ok(v0))
                    }
                }
            };
            match step {
                Ok(next) => { m2740 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f397(v0: u64) -> Bytes {
    let m2743 = v0;
    if m2743 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2743 - 1;
        let a2744 = Bytes::lit(&[0x30]);
        let a2745 = f397(v1);
        append_bytes(a2744, a2745)
    }
}

pub fn f398(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a2790 = {
            let a2766 = {
                let a2754 = {
                    let a2748 = {
                        let a2746 = v3;
                        let a2747 = 128u8;
                        fixed_u8::bit_and(a2746, a2747)
                    };
                    let a2749 = 128u8;
                    if equal(a2748, a2749) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2755 = {
                    let a2752 = {
                        let a2750 = v3;
                        let a2751 = 64u8;
                        fixed_u8::bit_and(a2750, a2751)
                    };
                    let a2753 = 64u8;
                    if equal(a2752, a2753) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2754, a2755)?
            };
            let a2767 = {
                let a2764 = {
                    let a2758 = {
                        let a2756 = v3;
                        let a2757 = 32u8;
                        fixed_u8::bit_and(a2756, a2757)
                    };
                    let a2759 = 32u8;
                    if equal(a2758, a2759) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2765 = {
                    let a2762 = {
                        let a2760 = v3;
                        let a2761 = 16u8;
                        fixed_u8::bit_and(a2760, a2761)
                    };
                    let a2763 = 16u8;
                    if equal(a2762, a2763) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2764, a2765)?
            };
            nat_add(a2766, a2767)?
        };
        let a2791 = {
            let a2788 = {
                let a2776 = {
                    let a2770 = {
                        let a2768 = v3;
                        let a2769 = 8u8;
                        fixed_u8::bit_and(a2768, a2769)
                    };
                    let a2771 = 8u8;
                    if equal(a2770, a2771) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2777 = {
                    let a2774 = {
                        let a2772 = v3;
                        let a2773 = 4u8;
                        fixed_u8::bit_and(a2772, a2773)
                    };
                    let a2775 = 4u8;
                    if equal(a2774, a2775) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2776, a2777)?
            };
            let a2789 = {
                let a2786 = {
                    let a2780 = {
                        let a2778 = v3;
                        let a2779 = 2u8;
                        fixed_u8::bit_and(a2778, a2779)
                    };
                    let a2781 = 2u8;
                    if equal(a2780, a2781) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2787 = {
                    let a2784 = {
                        let a2782 = v3;
                        let a2783 = 1u8;
                        fixed_u8::bit_and(a2782, a2783)
                    };
                    let a2785 = 1u8;
                    if equal(a2784, a2785) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2786, a2787)?
            };
            nat_add(a2788, a2789)?
        };
        nat_add(a2790, a2791)?
    };
    let a2792 = v4;
    let a2793 = 34u64;
    if nat_eq(a2792, a2793) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a2794 = v4;
        let a2795 = 92u64;
        if nat_eq(a2794, a2795) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a2796 = v4;
            let a2797 = 8u64;
            if nat_eq(a2796, a2797) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a2798 = v4;
                let a2799 = 12u64;
                if nat_eq(a2798, a2799) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a2800 = v4;
                    let a2801 = 10u64;
                    if nat_eq(a2800, a2801) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a2802 = v4;
                        let a2803 = 13u64;
                        if nat_eq(a2802, a2803) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a2804 = v4;
                            let a2805 = 9u64;
                            if nat_eq(a2804, a2805) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a2806 = v4;
                                let a2807 = 32u64;
                                if nat_lt(a2806, a2807) {
                                    let a2808 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a2809 = f493(v2)?;
                                    Ok(append_bytes(a2808, a2809))
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

pub fn f399(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m2810 = v0.clone();
    match m2810.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a2813 = {
                let (h2811, _) = v2;
                h2811
            };
            let a2814 = {
                let (h2812, _) = v1.clone();
                h2812
            };
            if equal(a2813, a2814) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f400(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f494(f495(Fn16::F496, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m2819 = {
        let (h2815, _) = f497(Fn17::F498, {
            let a2817 = {
                let a2816 = v0;
                length_list(a2816)
            };
            let a2818 = 1u64;
            nat_add(a2817, a2818)?
        }, v1)?;
        h2815
    };
    match m2819.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f401(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2824 = {
        let a2821 = f226({
            let (h2820, _) = v0;
            h2820
        })?;
        let a2822 = Bytes::lit(&[0x3a]);
        append_bytes(a2821, a2822)
    };
    let a2825 = {
        let (_, h2823) = v0;
        h2823
    };
    Ok(append_bytes(a2824, a2825))
}

pub fn f402(v0: Bytes, p2851: u64, p2852: u64, p2853: Bytes) -> R<Bytes> {
    let m2850 = (p2851, (p2852, p2853));
    {
        let mut m2850 = m2850;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2850;
                let m2826 = v2;
                if m2826 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m2826 - 1;
                    let m2829 = {
                        let a2827 = v0.clone();
                        let a2828 = v1;
                        index_bytes(a2827, a2828)
                    };
                    match m2829 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v6: u64 = f60(v5)?;
                            if f155({
                                let a2830 = v6;
                                let a2831 = 34u64;
                                nat_eq(a2830, a2831)
                            }, {
                                let a2832 = v6;
                                let a2833 = 92u64;
                                nat_eq(a2832, a2833)
                            }) {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a2834 = v1;
                                    let a2835 = 1u64;
                                    nat_add(a2834, a2835)?
                                }, (v4, {
                                    let a2838 = v3;
                                    let a2839 = {
                                        let a2836 = f8(Str::lit("\\"));
                                        let a2837 = f46(v0.clone(), v1, 1u64);
                                        append_bytes(a2836, a2837)
                                    };
                                    append_bytes(a2838, a2839)
                                })))
                            } else {
                                let a2840 = v6;
                                let a2841 = 10u64;
                                if nat_eq(a2840, a2841) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a2842 = v1;
                                        let a2843 = 1u64;
                                        nat_add(a2842, a2843)?
                                    }, (v4, {
                                        let a2844 = v3;
                                        let a2845 = f8(Str::lit("\\n"));
                                        append_bytes(a2844, a2845)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a2846 = v1;
                                        let a2847 = 1u64;
                                        nat_add(a2846, a2847)?
                                    }, (v4, {
                                        let a2848 = v3;
                                        let a2849 = f46(v0.clone(), v1, 1u64);
                                        append_bytes(a2848, a2849)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2850 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f403(v0: Bytes) -> Bytes {
    if f41(v0.clone(), Bytes::lit(&[0x0d])) {
        f46(v0.clone(), 0u64, {
            let a2855 = {
                let a2854 = v0;
                length_bytes(a2854)
            };
            let a2856 = 1u64;
            nat_sub(a2855, a2856)
        })
    } else {
        v0
    }
}

pub fn f404(v0: Fn10, p2860: List<Bytes>, p2861: List<(Bytes, ())>) -> List<Bytes> {
    let m2859 = (p2860, p2861);
    {
        let mut m2859 = m2859;
        loop {
            let step = {
                let (v1, v2) = m2859;
                let m2857 = v2;
                match m2857.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c2858: Fn10 = v0.clone();
                            c2858.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2859 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f405(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h2862, _) = v1;
        h2862
    }, v0)
}

pub fn f406(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f499(Fn18::F500, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f407(v0: Fn11, p2866: List<List<(Bytes, ())>>, p2867: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m2865 = (p2866, p2867);
    {
        let mut m2865 = m2865;
        loop {
            let step = {
                let (v1, v2) = m2865;
                let m2863 = v2;
                match m2863.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c2864: Fn11 = v0.clone();
                            c2864.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2865 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f408(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f409(v0: Fn12, p2872: u64, p2873: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m2871 = (p2872, p2873);
    {
        let mut m2871 = m2871;
        loop {
            let step = {
                let (v1, v2) = m2871;
                let m2868 = v1;
                if m2868 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2868 - 1;
                    let m2870 = {
                        let c2869: Fn12 = v0.clone();
                        c2869.apply(v2.clone())?
                    };
                    match m2870 {
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
                Ok(next) => { m2871 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f410(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f501(v0)
}

pub fn f411(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f502(Fn19::F503, List::<(Bytes, ())>::nil(), v0)
}

pub fn f412(v0: Fn13, p2877: List<(Bytes, ())>, p2878: List<Bytes>) -> List<(Bytes, ())> {
    let m2876 = (p2877, p2878);
    {
        let mut m2876 = m2876;
        loop {
            let step = {
                let (v1, v2) = m2876;
                let m2874 = v2;
                match m2874.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c2875: Fn13 = v0.clone();
                            c2875.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2876 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f413(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f414(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f504(v1, v0, v2)
}

pub fn f415(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2881 = {
        let a2879 = v0;
        let a2880 = v1;
        index_bytes(a2879, a2880)
    };
    match m2881 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f60(v2)?;
            if f105({
                let a2882 = 128u64;
                let a2883 = v3;
                nat_le(a2882, a2883)
            }, {
                let a2884 = v3;
                let a2885 = 191u64;
                nat_le(a2884, a2885)
            }) {
                Ok(Some({
                    let a2886 = v3;
                    let a2887 = 128u64;
                    nat_sub(a2886, a2887)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f416(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f105({
        let a2888 = v1;
        let a2889 = v0;
        nat_le(a2888, a2889)
    }, f105({
        let a2890 = v0;
        let a2891 = v2;
        nat_le(a2890, a2891)
    }, f155({
        let a2892 = v0;
        let a2893 = 55296u64;
        nat_lt(a2892, a2893)
    }, {
        let a2894 = 57343u64;
        let a2895 = v0;
        nat_lt(a2894, a2895)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f417(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f505(v0, v1)
}

pub fn f418(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f241(f182(v0, v1)?) {
        Ok(Some({
            let a2896 = v1;
            let a2897 = 1u64;
            nat_add(a2896, a2897)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f419(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f506(v0, v1)
}

pub fn f420(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2898 = f182(v0, v1)?;
    let a2899 = 32u64;
    if nat_eq(a2898, a2899) {
        Ok(Some({
            let a2900 = v1;
            let a2901 = 1u64;
            nat_add(a2900, a2901)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f421(v0: u64) -> bool {
    let a2902 = v0;
    let a2903 = 44u64;
    if if if nat_eq(a2902, a2903) {
        true
    } else {
        let a2904 = v0;
        let a2905 = 91u64;
        nat_eq(a2904, a2905)
    } {
        true
    } else {
        let a2906 = v0;
        let a2907 = 93u64;
        if nat_eq(a2906, a2907) {
            true
        } else {
            let a2908 = v0;
            let a2909 = 123u64;
            nat_eq(a2908, a2909)
        }
    } {
        true
    } else {
        let a2910 = v0;
        let a2911 = 125u64;
        nat_eq(a2910, a2911)
    }
}

pub fn f422(v0: u64) -> u64 {
    let a2912 = v0;
    let a2913 = 45u64;
    if nat_eq(a2912, a2913) {
        1u64
    } else {
        let a2914 = v0;
        let a2915 = 43u64;
        if nat_eq(a2914, a2915) {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f423(v0: u64) -> u64 {
    let a2916 = 49u64;
    let a2917 = v0;
    if if nat_le(a2916, a2917) {
        let a2918 = v0;
        let a2919 = 57u64;
        nat_le(a2918, a2919)
    } else {
        false
    } {
        let a2920 = v0;
        let a2921 = 48u64;
        nat_sub(a2920, a2921)
    } else {
        0u64
    }
}

pub fn f424(v0: Bytes, v1: u64, v2: u64, v3: bool, v4: u64) -> R<Adt19> {
    let m2922 = f507(v0.clone(), v1, v2)?;
    match m2922 {
        Adt26::C0(v6, v7) => {
            let v8: u64 = v7;
            let v9: u64 = f508(v0.clone(), v8, v2)?;
            if f509(v0.clone(), v8, v2)? {
                let m2926 = f510(v0.clone(), v8, v2, v3, {
                    let a2924 = {
                        let a2923 = v0;
                        length_bytes(a2923)
                    };
                    let a2925 = 2u64;
                    nat_add(a2924, a2925)?
                }, Bytes::lit(&[]), v6)?;
                match m2926 {
                    Adt27::C0(v11, v12, v13) => {
                        Ok(Adt19::C0(Adt2::C6(f7(List::cons(v11, List::cons({
                            let a2927 = v4;
                            let a2928 = 1u64;
                            if nat_eq(a2927, a2928) {
                                Bytes::lit(&[])
                            } else {
                                Bytes::lit(&[0x0a])
                            }
                        }, List::cons({
                            let a2929 = v4;
                            let a2930 = 2u64;
                            if nat_eq(a2929, a2930) {
                                f426(v12)
                            } else {
                                Bytes::lit(&[])
                            }
                        }, List::<Bytes>::nil()))))), v13))
                    }
                    Adt27::C1(v10) => {
                        Ok(Adt19::C1(v10))
                    }
                }
            } else {
                Ok(Adt19::C0(Adt2::C6({
                    let a2931 = v4;
                    let a2932 = 2u64;
                    if nat_eq(a2931, a2932) {
                        f426(v6)
                    } else {
                        Bytes::lit(&[])
                    }
                }), {
                    let a2933 = f182(v0, v9)?;
                    let a2934 = 256u64;
                    if nat_eq(a2933, a2934) {
                        v9
                    } else {
                        v8
                    }
                }))
            }
        }
        Adt26::C1(v5) => {
            Ok(Adt19::C1(v5))
        }
    }
}

pub fn f425(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f511(v0, v1)
}

pub fn f426(v0: u64) -> Bytes {
    let m2935 = v0;
    if m2935 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2935 - 1;
        let a2936 = Bytes::lit(&[0x0a]);
        let a2937 = f426(v1);
        append_bytes(a2936, a2937)
    }
}

pub fn f427(v0: Bytes, v1: u64) -> R<u64> {
    let (_, h2938) = {
        let (h2939, _) = f211(Fn6::F512(v0.clone()), {
            let a2941 = {
                let a2940 = v0;
                length_bytes(a2940)
            };
            let a2942 = 1u64;
            nat_add(a2941, a2942)?
        }, (v1, 0u64))?;
        h2939
    };
    Ok(h2938)
}

pub fn f428(v0: u64, v1: u64) -> u64 {
    let a2943 = v0;
    let a2944 = v1;
    if nat_lt(a2943, a2944) {
        v1
    } else {
        v0
    }
}

pub fn f429(v0: Bytes, v1: u64, v2: u64) -> R<Adt19> {
    if f430(v0.clone()) {
        Ok(Adt19::C0(Adt2::C0, v2))
    } else {
        if f434(v0.clone()) {
            Ok(Adt19::C0(Adt2::C2(true), v2))
        } else {
            if f435(v0.clone()) {
                Ok(Adt19::C0(Adt2::C2(false), v2))
            } else {
                if f433(v0.clone()) {
                    Ok(Adt19::C1(f129(Str::lit("an infinite or not-a-number float is not read: Studio.Value has none (YAML 1.2.2 section 10.3.2)"), v1)?))
                } else {
                    if if f179(v0.clone(), 0u64, Str::lit("0o")) {
                        let a2946 = 2u64;
                        let a2947 = {
                            let a2945 = v0.clone();
                            length_bytes(a2945)
                        };
                        nat_lt(a2946, a2947)
                    } else {
                        false
                    } {
                        f513(v0, 8u64, v1, v2)
                    } else {
                        if if f179(v0.clone(), 0u64, Str::lit("0x")) {
                            let a2949 = 2u64;
                            let a2950 = {
                                let a2948 = v0.clone();
                                length_bytes(a2948)
                            };
                            nat_lt(a2949, a2950)
                        } else {
                            false
                        } {
                            f513(v0, 16u64, v1, v2)
                        } else {
                            let m2951 = f432(v0.clone())?;
                            match m2951 {
                                None => {
                                    Ok(Adt19::C0(Adt2::C6(v0), v2))
                                }
                                Some(v3) => {
                                    let m2952 = f106(v3.clone(), 0u64)?;
                                    match m2952 {
                                        None => {
                                            Ok(Adt19::C1(f514(v1)?))
                                        }
                                        Some(v4) => {
                                            let a2955 = {
                                                let (h2953, _) = v4;
                                                h2953
                                            };
                                            let a2956 = {
                                                let a2954 = v3;
                                                length_bytes(a2954)
                                            };
                                            if nat_eq(a2955, a2956) {
                                                Ok(Adt19::C0({
                                                    let (_, h2957) = v4;
                                                    h2957
                                                }, v2))
                                            } else {
                                                Ok(Adt19::C1(f514(v1)?))
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

pub fn f430(v0: Bytes) -> bool {
    let a2959 = {
        let a2958 = v0.clone();
        length_bytes(a2958)
    };
    let a2960 = 0u64;
    if nat_eq(a2959, a2960) {
        true
    } else {
        f262(v0, List::cons(f8(Str::lit("~")), List::cons(f8(Str::lit("null")), List::cons(f8(Str::lit("Null")), List::cons(f8(Str::lit("NULL")), List::<Bytes>::nil())))))
    }
}

pub fn f431(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a2961 = v0.clone();
        length_bytes(a2961)
    };
    let v2: u64 = f182(v0.clone(), 0u64)?;
    let v3: u64 = {
        let a2962 = v2;
        let a2963 = 43u64;
        if if nat_eq(a2962, a2963) {
            true
        } else {
            let a2964 = v2;
            let a2965 = 45u64;
            nat_eq(a2964, a2965)
        } {
            1u64
        } else {
            0u64
        }
    };
    let a2966 = v3;
    let a2967 = v1;
    if if nat_lt(a2966, a2967) {
        let a2968 = f154(v0.clone(), v3)?;
        let a2969 = v1;
        nat_eq(a2968, a2969)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a2970 = 2u64;
        let a2971 = v1;
        if nat_lt(a2970, a2971) {
            if if f179(v0.clone(), 0u64, Str::lit("0o")) {
                let a2973 = {
                    let (h2972, _) = f515(v0.clone(), 8u64, 2u64)?;
                    h2972
                };
                let a2974 = v1;
                nat_eq(a2973, a2974)
            } else {
                false
            } {
                Ok(true)
            } else {
                if f179(v0.clone(), 0u64, Str::lit("0x")) {
                    let a2976 = {
                        let (h2975, _) = f515(v0, 16u64, 2u64)?;
                        h2975
                    };
                    let a2977 = v1;
                    Ok(nat_eq(a2976, a2977))
                } else {
                    Ok(false)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f432(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2978 = v0.clone();
        length_bytes(a2978)
    };
    let v2: u64 = f182(v0.clone(), 0u64)?;
    let v3: u64 = {
        let a2979 = v2;
        let a2980 = 43u64;
        if if nat_eq(a2979, a2980) {
            true
        } else {
            let a2981 = v2;
            let a2982 = 45u64;
            nat_eq(a2981, a2982)
        } {
            1u64
        } else {
            0u64
        }
    };
    let v4: u64 = f154(v0.clone(), v3)?;
    let v5: bool = {
        let a2983 = f182(v0.clone(), v4)?;
        let a2984 = 46u64;
        nat_eq(a2983, a2984)
    };
    let v6: u64 = if v5 {
        let a2985 = v4;
        let a2986 = 1u64;
        nat_add(a2985, a2986)?
    } else {
        v4
    };
    let v7: u64 = if v5 {
        f154(v0.clone(), v6)?
    } else {
        v4
    };
    let v8: u64 = f182(v0.clone(), v7)?;
    let v9: bool = {
        let a2987 = v8;
        let a2988 = 101u64;
        if nat_eq(a2987, a2988) {
            true
        } else {
            let a2989 = v8;
            let a2990 = 69u64;
            nat_eq(a2989, a2990)
        }
    };
    let v10: u64 = f182(v0.clone(), {
        let a2991 = v7;
        let a2992 = 1u64;
        nat_add(a2991, a2992)?
    })?;
    let v11: bool = if v9 {
        let a2993 = v10;
        let a2994 = 43u64;
        if nat_eq(a2993, a2994) {
            true
        } else {
            let a2995 = v10;
            let a2996 = 45u64;
            nat_eq(a2995, a2996)
        }
    } else {
        false
    };
    let v12: u64 = if v9 {
        if v11 {
            let a2997 = v7;
            let a2998 = 2u64;
            nat_add(a2997, a2998)?
        } else {
            let a2999 = v7;
            let a3000 = 1u64;
            nat_add(a2999, a3000)?
        }
    } else {
        v7
    };
    let v13: u64 = if v9 {
        f154(v0.clone(), v12)?
    } else {
        v7
    };
    let v14: Bytes = f258(v0.clone(), v3, v4);
    let v15: u64 = f156(v14.clone())?;
    let a3001 = v13;
    let a3002 = v1;
    if if if nat_eq(a3001, a3002) {
        let a3003 = v3;
        let a3004 = v4;
        if nat_lt(a3003, a3004) {
            true
        } else {
            if v5 {
                let a3005 = v6;
                let a3006 = v7;
                nat_lt(a3005, a3006)
            } else {
                false
            }
        }
    } else {
        false
    } {
        if f184(v9) {
            true
        } else {
            let a3007 = v12;
            let a3008 = v13;
            nat_lt(a3007, a3008)
        }
    } else {
        false
    } {
        Ok(Some(f7(List::cons({
            let a3009 = v2;
            let a3010 = 45u64;
            if nat_eq(a3009, a3010) {
                Bytes::lit(&[0x2d])
            } else {
                Bytes::lit(&[])
            }
        }, List::cons({
            let a3012 = v15;
            let a3013 = {
                let a3011 = v14.clone();
                length_bytes(a3011)
            };
            if nat_eq(a3012, a3013) {
                Bytes::lit(&[0x30])
            } else {
                f258(v14.clone(), v15, {
                    let a3014 = v14;
                    length_bytes(a3014)
                })
            }
        }, List::cons({
            let a3015 = v6;
            let a3016 = v7;
            if nat_lt(a3015, a3016) {
                let a3017 = Bytes::lit(&[0x2e]);
                let a3018 = f258(v0.clone(), v6, v7);
                append_bytes(a3017, a3018)
            } else {
                Bytes::lit(&[])
            }
        }, List::cons(if v9 {
            let a3021 = if if v11 {
                let a3019 = v10;
                let a3020 = 45u64;
                nat_eq(a3019, a3020)
            } else {
                false
            } {
                Bytes::lit(&[0x65, 0x2d])
            } else {
                Bytes::lit(&[0x65])
            };
            let a3022 = f258(v0, v12, v13);
            append_bytes(a3021, a3022)
        } else {
            Bytes::lit(&[])
        }, List::<Bytes>::nil())))))))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f433(v0: Bytes) -> bool {
    f262(v0, List::cons(f8(Str::lit(".inf")), List::cons(f8(Str::lit(".Inf")), List::cons(f8(Str::lit(".INF")), List::cons(f8(Str::lit("+.inf")), List::cons(f8(Str::lit("+.Inf")), List::cons(f8(Str::lit("+.INF")), List::cons(f8(Str::lit("-.inf")), List::cons(f8(Str::lit("-.Inf")), List::cons(f8(Str::lit("-.INF")), List::cons(f8(Str::lit(".nan")), List::cons(f8(Str::lit(".NaN")), List::cons(f8(Str::lit(".NAN")), List::<Bytes>::nil())))))))))))))
}

pub fn f434(v0: Bytes) -> bool {
    f262(v0, List::cons(f8(Str::lit("true")), List::cons(f8(Str::lit("True")), List::cons(f8(Str::lit("TRUE")), List::<Bytes>::nil()))))
}

pub fn f435(v0: Bytes) -> bool {
    f262(v0, List::cons(f8(Str::lit("false")), List::cons(f8(Str::lit("False")), List::cons(f8(Str::lit("FALSE")), List::<Bytes>::nil()))))
}

pub fn f436(v0: Bytes, p3081: u64, v2: u64, v3: bool, p3082: u64, p3083: Bytes) -> R<Adt23> {
    let m3080 = (p3081, (p3082, p3083));
    {
        let mut m3080 = m3080;
        loop {
            let step = {
                let (v1, (v4, v5)) = m3080;
                let m3023 = v4;
                if m3023 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C1(f129(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                } else {
                    let v6 = m3023 - 1;
                    let v7: u64 = f182(v0.clone(), v1)?;
                    let a3024 = v7;
                    let a3025 = 34u64;
                    if nat_eq(a3024, a3025) {
                        Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C0(v5, {
                            let a3026 = v1;
                            let a3027 = 1u64;
                            nat_add(a3026, a3027)?
                        })))
                    } else {
                        let a3028 = v7;
                        let a3029 = 256u64;
                        if nat_eq(a3028, a3029) {
                            Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C1(f129(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                        } else {
                            let a3030 = v7;
                            let a3031 = 92u64;
                            if nat_eq(a3030, a3031) {
                                let v8: u64 = f182(v0.clone(), {
                                    let a3032 = v1;
                                    let a3033 = 1u64;
                                    nat_add(a3032, a3033)?
                                })?;
                                let m3034 = f516(v8)?;
                                match m3034 {
                                    None => {
                                        let a3039 = v8;
                                        let a3040 = 10u64;
                                        if nat_eq(a3039, a3040) {
                                            if v3 {
                                                Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C1(f518(v1)?)))
                                            } else {
                                                let m3043 = f519(v0.clone(), {
                                                    let a3041 = v1;
                                                    let a3042 = 1u64;
                                                    nat_add(a3041, a3042)?
                                                }, v2, v6, 0u64)?;
                                                match m3043 {
                                                    Adt26::C0(v11, v12) => {
                                                        Ok::<(u64, (u64, Bytes)), R<Adt23>>((v12, (v6, {
                                                            let a3044 = v5;
                                                            let a3045 = f426(v11);
                                                            append_bytes(a3044, a3045)
                                                        })))
                                                    }
                                                    Adt26::C1(v10) => {
                                                        Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C1(v10)))
                                                    }
                                                }
                                            }
                                        } else {
                                            let v13: u64 = {
                                                let a3046 = v8;
                                                let a3047 = 120u64;
                                                if nat_eq(a3046, a3047) {
                                                    2u64
                                                } else {
                                                    let a3048 = v8;
                                                    let a3049 = 117u64;
                                                    if nat_eq(a3048, a3049) {
                                                        4u64
                                                    } else {
                                                        let a3050 = v8;
                                                        let a3051 = 85u64;
                                                        if nat_eq(a3050, a3051) {
                                                            8u64
                                                        } else {
                                                            0u64
                                                        }
                                                    }
                                                }
                                            };
                                            let a3052 = v13;
                                            let a3053 = 0u64;
                                            if nat_eq(a3052, a3053) {
                                                Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C1(f129(Str::lit("an escape must be one of YAML 1.2.2 [42] to [62]"), v1)?)))
                                            } else {
                                                let m3056 = f520(v0.clone(), {
                                                    let a3054 = v1;
                                                    let a3055 = 2u64;
                                                    nat_add(a3054, a3055)?
                                                }, v13, 0u64)?;
                                                match m3056 {
                                                    None => {
                                                        Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C1(f129(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                    }
                                                    Some(v14) => {
                                                        let a3057 = 1114111u64;
                                                        let a3058 = v14;
                                                        if if nat_lt(a3057, a3058) {
                                                            true
                                                        } else {
                                                            let a3059 = 55296u64;
                                                            let a3060 = v14;
                                                            if nat_le(a3059, a3060) {
                                                                let a3061 = v14;
                                                                let a3062 = 57343u64;
                                                                nat_le(a3061, a3062)
                                                            } else {
                                                                false
                                                            }
                                                        } {
                                                            Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C1(f129(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                        } else {
                                                            Ok::<(u64, (u64, Bytes)), R<Adt23>>(({
                                                                let a3065 = {
                                                                    let a3063 = v1;
                                                                    let a3064 = 2u64;
                                                                    nat_add(a3063, a3064)?
                                                                };
                                                                let a3066 = v13;
                                                                nat_add(a3065, a3066)?
                                                            }, (v6, {
                                                                let a3067 = v5;
                                                                let a3068 = f517(v14)?;
                                                                append_bytes(a3067, a3068)
                                                            })))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Some(v9) => {
                                        Ok::<(u64, (u64, Bytes)), R<Adt23>>(({
                                            let a3035 = v1;
                                            let a3036 = 2u64;
                                            nat_add(a3035, a3036)?
                                        }, (v6, {
                                            let a3037 = v5;
                                            let a3038 = f517(v9)?;
                                            append_bytes(a3037, a3038)
                                        })))
                                    }
                                }
                            } else {
                                if if f241(v7) {
                                    true
                                } else {
                                    let a3069 = v7;
                                    let a3070 = 10u64;
                                    nat_eq(a3069, a3070)
                                } {
                                    let v15: u64 = f244(v0.clone(), v1)?;
                                    let a3071 = f182(v0.clone(), v15)?;
                                    let a3072 = 10u64;
                                    if nat_eq(a3071, a3072) {
                                        if v3 {
                                            Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C1(f518(v1)?)))
                                        } else {
                                            let m3073 = f519(v0.clone(), v15, v2, v6, 0u64)?;
                                            match m3073 {
                                                Adt26::C0(v17, v18) => {
                                                    Ok::<(u64, (u64, Bytes)), R<Adt23>>((v18, (v6, {
                                                        let a3074 = v5;
                                                        let a3075 = f444(v17);
                                                        append_bytes(a3074, a3075)
                                                    })))
                                                }
                                                Adt26::C1(v16) => {
                                                    Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C1(v16)))
                                                }
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (u64, Bytes)), R<Adt23>>((v15, (v6, {
                                            let a3076 = v5;
                                            let a3077 = f258(v0.clone(), v1, v15);
                                            append_bytes(a3076, a3077)
                                        })))
                                    }
                                } else {
                                    let v19: u64 = f521(v0.clone(), v1)?;
                                    Ok::<(u64, (u64, Bytes)), R<Adt23>>((v19, (v6, {
                                        let a3078 = v5;
                                        let a3079 = f258(v0.clone(), v1, v19);
                                        append_bytes(a3078, a3079)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3080 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f437(v0: Bytes, p3111: u64, v2: u64, v3: bool, p3112: u64, p3113: Bytes) -> R<Adt23> {
    let m3110 = (p3111, (p3112, p3113));
    {
        let mut m3110 = m3110;
        loop {
            let step = {
                let (v1, (v4, v5)) = m3110;
                let m3084 = v4;
                if m3084 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C1(f129(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                } else {
                    let v6 = m3084 - 1;
                    let v7: u64 = f182(v0.clone(), v1)?;
                    let a3085 = v7;
                    let a3086 = 39u64;
                    if nat_eq(a3085, a3086) {
                        let a3089 = f182(v0.clone(), {
                            let a3087 = v1;
                            let a3088 = 1u64;
                            nat_add(a3087, a3088)?
                        })?;
                        let a3090 = 39u64;
                        if nat_eq(a3089, a3090) {
                            Ok::<(u64, (u64, Bytes)), R<Adt23>>(({
                                let a3091 = v1;
                                let a3092 = 2u64;
                                nat_add(a3091, a3092)?
                            }, (v6, {
                                let a3093 = v5;
                                let a3094 = Bytes::lit(&[0x27]);
                                append_bytes(a3093, a3094)
                            })))
                        } else {
                            Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C0(v5, {
                                let a3095 = v1;
                                let a3096 = 1u64;
                                nat_add(a3095, a3096)?
                            })))
                        }
                    } else {
                        let a3097 = v7;
                        let a3098 = 256u64;
                        if nat_eq(a3097, a3098) {
                            Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C1(f129(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                        } else {
                            if if f241(v7) {
                                true
                            } else {
                                let a3099 = v7;
                                let a3100 = 10u64;
                                nat_eq(a3099, a3100)
                            } {
                                let v8: u64 = f244(v0.clone(), v1)?;
                                let a3101 = f182(v0.clone(), v8)?;
                                let a3102 = 10u64;
                                if nat_eq(a3101, a3102) {
                                    if v3 {
                                        Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C1(f518(v1)?)))
                                    } else {
                                        let m3103 = f519(v0.clone(), v8, v2, v6, 0u64)?;
                                        match m3103 {
                                            Adt26::C0(v10, v11) => {
                                                Ok::<(u64, (u64, Bytes)), R<Adt23>>((v11, (v6, {
                                                    let a3104 = v5;
                                                    let a3105 = f444(v10);
                                                    append_bytes(a3104, a3105)
                                                })))
                                            }
                                            Adt26::C1(v9) => {
                                                Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C1(v9)))
                                            }
                                        }
                                    }
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Adt23>>((v8, (v6, {
                                        let a3106 = v5;
                                        let a3107 = f258(v0.clone(), v1, v8);
                                        append_bytes(a3106, a3107)
                                    })))
                                }
                            } else {
                                let v12: u64 = f522(v0.clone(), v1)?;
                                Ok::<(u64, (u64, Bytes)), R<Adt23>>((v12, (v6, {
                                    let a3108 = v5;
                                    let a3109 = f258(v0.clone(), v1, v12);
                                    append_bytes(a3108, a3109)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3110 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f438(v0: Bytes, v1: u64, v2: u64) -> R<Result<u64, Bytes>> {
    let v3: (u64, bool) = {
        let (h3114, _) = f523(Fn20::F524(v2, v0.clone()), {
            let a3116 = {
                let a3115 = v0.clone();
                length_bytes(a3115)
            };
            let a3117 = 1u64;
            nat_add(a3116, a3117)?
        }, (v1, false))?;
        h3114
    };
    let (_, h3118) = v3;
    if h3118 {
        if f180(v0, {
            let (h3119, _) = v3;
            h3119
        })? {
            Ok(Err::<u64, Bytes>(f129(Str::lit("a document marker cannot appear inside a flow collection (YAML 1.2.2 [206])"), {
                let (h3120, _) = v3;
                h3120
            })?))
        } else {
            Ok(Err::<u64, Bytes>(f129(Str::lit("a line of a flow collection is indented less than its node (YAML 1.2.2 [80])"), {
                let (h3121, _) = v3;
                h3121
            })?))
        }
    } else {
        Ok(Ok::<u64, Bytes>({
            let (h3122, _) = v3;
            h3122
        }))
    }
}

pub fn f439(v0: Bytes, v1: u64, v2: u64) -> R<Adt23> {
    let a3123 = f182(v0.clone(), v1)?;
    let a3124 = 33u64;
    if nat_eq(a3123, a3124) {
        let m3125 = f259(v0.clone(), v1)?;
        match m3125 {
            Ok(v4) => {
                let m3126 = f438(v0.clone(), v4, v2)?;
                match m3126 {
                    Ok(v6) => {
                        let a3127 = f182(v0.clone(), v6)?;
                        let a3128 = 58u64;
                        if nat_eq(a3127, a3128) {
                            let a3131 = f254(v0, v1)?;
                            let a3132 = {
                                let a3129 = v1;
                                let a3130 = 1u64;
                                nat_add(a3129, a3130)?
                            };
                            if nat_eq(a3131, a3132) {
                                Ok(Adt23::C1(f129(Str::lit("an empty node with the non-specific tag ! is not read (YAML 1.2.2 [100])"), v1)?))
                            } else {
                                Ok(Adt23::C0(Bytes::lit(&[]), v6))
                            }
                        } else {
                            f525(v0, v6, v2)
                        }
                    }
                    Err(v5) => {
                        Ok(Adt23::C1(v5))
                    }
                }
            }
            Err(v3) => {
                Ok(Adt23::C1(v3))
            }
        }
    } else {
        f525(v0, v1, v2)
    }
}

pub fn f440(v0: u64) -> R<bool> {
    let a3133 = f257(f8(Str::lit(",[]{}-?:#&*!|>'\"@`")), v0)?;
    let a3134 = 18u64;
    if nat_lt(a3133, a3134) {
        Ok(true)
    } else {
        let a3135 = v0;
        let a3136 = 37u64;
        Ok(nat_eq(a3135, a3136))
    }
}

pub fn f441(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f526(v1, v0, v2)
}

pub fn f442(v0: u64) -> R<Bytes> {
    f129(Str::lit("a mapping value is not allowed here: a plain scalar holds ': ' (YAML 1.2.2 [130])"), v0)
}

pub fn f443(v0: Bytes, v1: u64) -> R<(u64, u64)> {
    let (h3137, _) = f211(Fn6::F527(v0.clone()), {
        let a3139 = {
            let a3138 = v0;
            length_bytes(a3138)
        };
        let a3140 = 1u64;
        nat_add(a3139, a3140)?
    }, (v1, 0u64))?;
    Ok(h3137)
}

pub fn f444(v0: u64) -> Bytes {
    let a3141 = v0;
    let a3142 = 0u64;
    if nat_eq(a3141, a3142) {
        Bytes::lit(&[0x20])
    } else {
        f426(v0)
    }
}

pub fn f445(v0: Bytes, v1: bool, v2: u64) -> R<Option<u64>> {
    f528(v0, v1, v2)
}

pub fn f446(v0: Bytes, p3147: List<Bytes>, p3148: u64) -> R<u64> {
    let m3146 = (p3147, p3148);
    {
        let mut m3146 = m3146;
        loop {
            let step = {
                let (v1, v2) = m3146;
                let m3143 = v1;
                match m3143.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<u64>>(Ok(0u64))
                    }
                    Some((v3, v4)) => {
                        if f32(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<u64>>(Ok(v2))
                        } else {
                            Ok::<(List<Bytes>, u64), R<u64>>((v4, {
                                let a3144 = v2;
                                let a3145 = 1u64;
                                nat_add(a3144, a3145)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3146 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f447(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a3150 = v2;
    let a3151 = {
        let a3149 = v0.clone();
        length_bytes(a3149)
    };
    if if nat_lt(a3150, a3151) {
        f184({
            let a3152 = f182(v0, v2)?;
            let a3153 = v1;
            nat_eq(a3152, a3153)
        })
    } else {
        false
    } {
        Ok(Some({
            let a3154 = v2;
            let a3155 = 1u64;
            nat_add(a3154, a3155)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f448(v0: u64, v1: Bytes) -> R<bool> {
    f529(v0, v1.clone(), 0u64, {
        let a3157 = {
            let a3156 = v1;
            length_bytes(a3156)
        };
        let a3158 = 3u64;
        let a3159 = 0u64;
        nat_quot(a3157, a3158, a3159)
    }, true)
}

pub fn f449() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{0}\u{0} "))
}

pub fn f450(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f530(v1, v0, v2)
}

pub fn f451(v0: u64) -> bool {
    if f452(v0) {
        true
    } else {
        let a3160 = 65u64;
        let a3161 = v0;
        if if nat_le(a3160, a3161) {
            let a3162 = v0;
            let a3163 = 70u64;
            nat_le(a3162, a3163)
        } else {
            false
        } {
            true
        } else {
            let a3164 = 97u64;
            let a3165 = v0;
            if nat_le(a3164, a3165) {
                let a3166 = v0;
                let a3167 = 102u64;
                nat_le(a3166, a3167)
            } else {
                false
            }
        }
    }
}

pub fn f452(v0: u64) -> bool {
    let a3168 = 48u64;
    let a3169 = v0;
    if nat_le(a3168, a3169) {
        let a3170 = v0;
        let a3171 = 57u64;
        nat_le(a3170, a3171)
    } else {
        false
    }
}

pub fn f453(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f323(v0.clone(), v1)?;
    let a3172 = v2;
    let a3173 = 69u64;
    if if nat_eq(a3172, a3173) {
        true
    } else {
        let a3174 = v2;
        let a3175 = 101u64;
        nat_eq(a3174, a3175)
    } {
        let v3: u64 = f323(v0.clone(), {
            let a3176 = v1;
            let a3177 = 1u64;
            nat_add(a3176, a3177)?
        })?;
        let v4: u64 = {
            let a3178 = v3;
            let a3179 = 43u64;
            if if nat_eq(a3178, a3179) {
                true
            } else {
                let a3180 = v3;
                let a3181 = 45u64;
                nat_eq(a3180, a3181)
            } {
                let a3182 = v1;
                let a3183 = 2u64;
                nat_add(a3182, a3183)?
            } else {
                let a3184 = v1;
                let a3185 = 1u64;
                nat_add(a3184, a3185)?
            }
        };
        let v5: u64 = f349(v0, 0u64, v4)?;
        let a3186 = v4;
        let a3187 = v5;
        if nat_lt(a3186, a3187) {
            Ok(v5)
        } else {
            Ok(v1)
        }
    } else {
        Ok(v1)
    }
}

pub fn f454(v0: Adt20, v1: Bytes) -> bool {
    let a3188 = f455(v0.clone());
    let a3189 = 3u64;
    if !nat_eq(a3188, a3189) {
        f32(f456(v0), v1)
    } else {
        false
    }
}

pub fn f455(v0: Adt20) -> u64 {
    let m3190 = v0;
    match m3190 {
        Adt20::C0(v1, _) => {
            v1
        }
    }
}

pub fn f456(v0: Adt20) -> Bytes {
    let m3191 = v0;
    match m3191 {
        Adt20::C0(_, v2) => {
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

pub fn f459(p3200: u64, p3201: List<Adt20>, p3202: List<Adt14>) -> R<Result<(List<Adt14>, List<Adt20>), Bytes>> {
    let m3199 = (p3200, (p3201, p3202));
    {
        let mut m3199 = m3199;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3199;
                let m3192 = v0;
                if m3192 == 0 {
                    Err::<(u64, (List<Adt20>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt20>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt20>), Bytes>(f8(Str::lit("a dictionary has more members than the input has tokens")))))
                } else {
                    let v3 = m3192 - 1;
                    if f352(v1.clone(), Str::lit("}")) {
                        let m3193 = f359(f354(v1), Str::lit(";"));
                        match m3193 {
                            Ok(v5) => {
                                Err::<(u64, (List<Adt20>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt20>), Bytes>>>(Ok(Ok::<(List<Adt14>, List<Adt20>), Bytes>((f531(v2, List::<Adt14>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<Adt20>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt20>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt20>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m3194 = f266(v1)?;
                        match m3194 {
                            Ok(v7) => {
                                let v8: bool = f352(v7.clone(), Str::lit("required"));
                                let m3195 = f364(v3, if v8 {
                                    f354(v7)
                                } else {
                                    v7
                                }, 0u64)?;
                                match m3195 {
                                    Ok(v10) => {
                                        let m3196 = f358(f365(v10.clone()));
                                        match m3196 {
                                            Ok(v12) => {
                                                if if f352(f360(v12.clone()), Str::lit("=")) {
                                                    !v8
                                                } else {
                                                    false
                                                } {
                                                    let m3197 = f461(f354(f360(v12.clone())))?;
                                                    match m3197 {
                                                        Ok(v14) => {
                                                            Ok::<(u64, (List<Adt20>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt20>), Bytes>>>((v3, (v14, List::cons(Adt14::C0(f362(v12), f468(f366(v10)), 0u64), v2))))
                                                        }
                                                        Err(v13) => {
                                                            Err::<(u64, (List<Adt20>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt20>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt20>), Bytes>(v13)))
                                                        }
                                                    }
                                                } else {
                                                    let m3198 = f359(f360(v12.clone()), Str::lit(";"));
                                                    match m3198 {
                                                        Ok(v16) => {
                                                            Ok::<(u64, (List<Adt20>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt20>), Bytes>>>((v3, (v16, List::cons(Adt14::C0(f362(v12), f468(f366(v10)), if v8 {
                                                                0u64
                                                            } else {
                                                                1u64
                                                            }), v2))))
                                                        }
                                                        Err(v15) => {
                                                            Err::<(u64, (List<Adt20>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt20>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt20>), Bytes>(v15)))
                                                        }
                                                    }
                                                }
                                            }
                                            Err(v11) => {
                                                Err::<(u64, (List<Adt20>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt20>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt20>), Bytes>(v11)))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Err::<(u64, (List<Adt20>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt20>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt20>), Bytes>(v9)))
                                    }
                                }
                            }
                            Err(v6) => {
                                Err::<(u64, (List<Adt20>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt20>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt20>), Bytes>(v6)))
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

pub fn f460(p3211: u64, p3212: List<Adt20>, p3213: List<Adt14>) -> R<Result<(List<Adt14>, List<Adt20>), Bytes>> {
    let m3210 = (p3211, (p3212, p3213));
    {
        let mut m3210 = m3210;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3210;
                let m3203 = v0;
                if m3203 == 0 {
                    Err::<(u64, (List<Adt20>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt20>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt20>), Bytes>(f8(Str::lit("an interface has more members than the input has tokens")))))
                } else {
                    let v3 = m3203 - 1;
                    if f352(v1.clone(), Str::lit("}")) {
                        let m3204 = f359(f354(v1), Str::lit(";"));
                        match m3204 {
                            Ok(v5) => {
                                Err::<(u64, (List<Adt20>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt20>), Bytes>>>(Ok(Ok::<(List<Adt14>, List<Adt20>), Bytes>((f531(v2, List::<Adt14>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<Adt20>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt20>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt20>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m3205 = f266(v1)?;
                        match m3205 {
                            Ok(v7) => {
                                let v8: List<Adt20> = if if f352(v7.clone(), Str::lit("stringifier")) {
                                    true
                                } else {
                                    f352(v7.clone(), Str::lit("inherit"))
                                } {
                                    f354(v7.clone())
                                } else {
                                    v7.clone()
                                };
                                let v9: List<Adt20> = if f352(v8.clone(), Str::lit("readonly")) {
                                    f354(v8)
                                } else {
                                    v8
                                };
                                if f352(v9.clone(), Str::lit("attribute")) {
                                    let m3206 = f364(v3, f354(v9), 0u64)?;
                                    match m3206 {
                                        Ok(v11) => {
                                            let m3207 = f358(f365(v11.clone()));
                                            match m3207 {
                                                Ok(v13) => {
                                                    let m3208 = f359(f360(v13.clone()), Str::lit(";"));
                                                    match m3208 {
                                                        Ok(v15) => {
                                                            Ok::<(u64, (List<Adt20>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt20>), Bytes>>>((v3, (v15, List::cons(Adt14::C0(f362(v13), f468(f366(v11)), 0u64), v2))))
                                                        }
                                                        Err(v14) => {
                                                            Err::<(u64, (List<Adt20>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt20>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt20>), Bytes>(v14)))
                                                        }
                                                    }
                                                }
                                                Err(v12) => {
                                                    Err::<(u64, (List<Adt20>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt20>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt20>), Bytes>(v12)))
                                                }
                                            }
                                        }
                                        Err(v10) => {
                                            Err::<(u64, (List<Adt20>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt20>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt20>), Bytes>(v10)))
                                        }
                                    }
                                } else {
                                    let m3209 = f461(v7)?;
                                    match m3209 {
                                        Ok(v17) => {
                                            Ok::<(u64, (List<Adt20>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt20>), Bytes>>>((v3, (v17, v2)))
                                        }
                                        Err(v16) => {
                                            Err::<(u64, (List<Adt20>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt20>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt20>), Bytes>(v16)))
                                        }
                                    }
                                }
                            }
                            Err(v6) => {
                                Err::<(u64, (List<Adt20>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt20>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt20>), Bytes>(v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3210 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f461(v0: List<Adt20>) -> R<Result<List<Adt20>, Bytes>> {
    Ok(f353({
        let a3215 = {
            let a3214 = v0.clone();
            length_list(a3214)
        };
        let a3216 = 1u64;
        nat_add(a3215, a3216)?
    }, v0, List::<Bytes>::nil(), true))
}

pub fn f462(v0: List<Adt20>) -> Bytes {
    let m3217 = v0;
    match m3217.uncons() {
        None => {
            f8(Str::lit("the end of the input"))
        }
        Some((v1, _)) => {
            f7(List::cons(f8(Str::lit("'")), List::cons(f456(v1), List::cons(f8(Str::lit("'")), List::<Bytes>::nil()))))
        }
    }
}

pub fn f463(v0: Bytes) -> Bytes {
    if f53(v0.clone(), f8(Str::lit("_"))) {
        f46(v0.clone(), 1u64, {
            let a3219 = {
                let a3218 = v0;
                length_bytes(a3218)
            };
            let a3220 = 1u64;
            nat_sub(a3219, a3220)
        })
    } else {
        v0
    }
}

pub fn f464(v0: Adt15, v1: List<Adt20>) -> Result<Adt25, Bytes> {
    if f352(v1.clone(), Str::lit("?")) {
        Ok::<Adt25, Bytes>(Adt25::C0(f465(Str::lit("Option"), v0), f354(v1)))
    } else {
        Ok::<Adt25, Bytes>(Adt25::C0(v0, v1))
    }
}

pub fn f465(v0: Str, v1: Adt15) -> Adt15 {
    let v2: Adt15 = f468(v1);
    Adt15::C0(0u64, f7(List::cons(f8(Str::lit("(")), List::cons({
        let a3221 = v0;
        utf8_encode(a3221)
    }, List::cons(f8(Str::lit(" ")), List::cons(f285(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), f286(v2.clone()), f283(v2), Bytes::lit(&[]))
}

pub fn f466(v0: Bytes) -> Adt15 {
    Adt15::C0(0u64, Bytes::lit(&[]), List::<Bytes>::nil(), v0, Bytes::lit(&[]))
}

pub fn f467(v0: Bytes) -> Option<Adt15> {
    f532(v0, f533())
}

pub fn f468(v0: Adt15) -> Adt15 {
    let a3222 = f284(v0.clone());
    let a3223 = 0u64;
    if nat_eq(a3222, a3223) {
        v0
    } else {
        Adt15::C0(0u64, Bytes::lit(&[]), f286(v0.clone()), if f32(f283(v0.clone()), Bytes::lit(&[])) {
            f534()
        } else {
            f283(v0)
        }, Bytes::lit(&[]))
    }
}

pub fn f469(v0: bool) -> Adt15 {
    let v1: Bytes = if v0 {
        f140()
    } else {
        f138()
    };
    Adt15::C0(0u64, v1.clone(), List::cons(v1.clone(), List::<Bytes>::nil()), Bytes::lit(&[]), v1)
}

pub fn f470(v0: Adt15) -> Bytes {
    let m3224 = v0;
    match m3224 {
        Adt15::C0(_, _, _, _, v5) => {
            v5
        }
    }
}

pub fn f471(p3241: u64, p3242: u64, p3243: u64, p3244: Bytes) -> Bytes {
    let m3240 = (p3241, (p3242, (p3243, p3244)));
    {
        let mut m3240 = m3240;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m3240;
                let m3225 = v0;
                if m3225 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m3225 - 1;
                    if f105({
                        let a3226 = v1;
                        let a3227 = 0u64;
                        nat_eq(a3226, a3227)
                    }, {
                        let a3228 = v2;
                        let a3229 = 0u64;
                        nat_eq(a3228, a3229)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a3230 = v1;
                            let a3231 = 16u64;
                            let a3232 = 0u64;
                            nat_quot(a3230, a3231, a3232)
                        }, ({
                            let a3233 = v2;
                            let a3234 = 1u64;
                            nat_sub(a3233, a3234)
                        }, {
                            let a3238 = f535({
                                let a3235 = v1;
                                let a3236 = 16u64;
                                let a3237 = 0u64;
                                nat_rem(a3235, a3236, a3237)
                            });
                            let a3239 = v3;
                            append_bytes(a3238, a3239)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m3240 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f472(v0: Adt14, v1: List<Adt14>) -> R<List<Adt14>> {
    let m3245 = v1;
    match m3245.uncons() {
        None => {
            Ok(List::cons(v0, List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            if f536({
                let a3247 = {
                    let a3246 = f375(v0.clone());
                    length_bytes(a3246)
                };
                let a3248 = 1u64;
                nat_add(a3247, a3248)?
            }, f375(v0.clone()), f375(v2.clone()), 0u64)? {
                Ok(List::cons(v0, List::cons(v2, v3)))
            } else {
                Ok(List::cons(v2, f472(v0, v3)?))
            }
        }
    }
}

pub fn f473(v0: List<Adt13>, v1: Bytes) -> List<Adt14> {
    let m3249 = f202(v0.clone(), 1u64, v1.clone());
    match m3249 {
        None => {
            f372(v0, 6u64, v1)
        }
        Some(v2) => {
            f370(f371(v2), f372(v0, 6u64, v1))
        }
    }
}

pub fn f474(v0: Bytes) -> R<Option<Bytes>> {
    let m3251 = f537({
        let a3250 = v0.clone();
        length_bytes(a3250)
    }, v0.clone(), 0u64, Bytes::lit(&[]))?;
    match m3251 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a3253 = {
                let a3252 = v1.clone();
                length_bytes(a3252)
            };
            let a3254 = 0u64;
            if nat_eq(a3253, a3254) {
                let a3256 = {
                    let a3255 = v0;
                    length_bytes(a3255)
                };
                let a3257 = 0u64;
                if nat_eq(a3256, a3257) {
                    Ok(Some(f8(Str::lit("u"))))
                } else {
                    Ok(None::<Bytes>)
                }
            } else {
                Ok(Some(f476(v1)?))
            }
        }
    }
}

pub fn f475(v0: Bytes) -> R<Option<Bytes>> {
    if f538({
        let a3258 = v0.clone();
        length_bytes(a3258)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f539(v0)?;
        if f540(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m3261 = {
                let a3259 = v1.clone();
                let a3260 = 0u64;
                index_bytes(a3259, a3260)
            };
            match m3261 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f222(f60(v2)?) {
                        Ok(Some({
                            let a3262 = f8(Str::lit("n"));
                            let a3263 = v1;
                            append_bytes(a3262, a3263)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f541({
            let a3264 = v0.clone();
            length_bytes(a3264)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f476(v0: Bytes) -> R<Bytes> {
    if f114(v0.clone(), f542()) {
        f368(v0)
    } else {
        Ok(v0)
    }
}

pub fn f477(v0: Fn14, p3269: u64, p3270: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m3268 = (p3269, p3270);
    {
        let mut m3268 = m3268;
        loop {
            let step = {
                let (v1, v2) = m3268;
                let m3265 = v1;
                if m3265 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3265 - 1;
                    let m3267 = {
                        let c3266: Fn14 = v0.clone();
                        c3266.apply(v2.clone())?
                    };
                    match m3267 {
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
                Ok(next) => { m3268 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f478(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f543(v0, v1)
}

pub fn f479(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3315 = {
            let a3291 = {
                let a3279 = {
                    let a3273 = {
                        let a3271 = v1;
                        let a3272 = 128u8;
                        fixed_u8::bit_and(a3271, a3272)
                    };
                    let a3274 = 128u8;
                    if equal(a3273, a3274) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3280 = {
                    let a3277 = {
                        let a3275 = v1;
                        let a3276 = 64u8;
                        fixed_u8::bit_and(a3275, a3276)
                    };
                    let a3278 = 64u8;
                    if equal(a3277, a3278) {
                        64u64
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
                        let a3282 = 32u8;
                        fixed_u8::bit_and(a3281, a3282)
                    };
                    let a3284 = 32u8;
                    if equal(a3283, a3284) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3290 = {
                    let a3287 = {
                        let a3285 = v1;
                        let a3286 = 16u8;
                        fixed_u8::bit_and(a3285, a3286)
                    };
                    let a3288 = 16u8;
                    if equal(a3287, a3288) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3289, a3290)?
            };
            nat_add(a3291, a3292)?
        };
        let a3316 = {
            let a3313 = {
                let a3301 = {
                    let a3295 = {
                        let a3293 = v1;
                        let a3294 = 8u8;
                        fixed_u8::bit_and(a3293, a3294)
                    };
                    let a3296 = 8u8;
                    if equal(a3295, a3296) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3302 = {
                    let a3299 = {
                        let a3297 = v1;
                        let a3298 = 4u8;
                        fixed_u8::bit_and(a3297, a3298)
                    };
                    let a3300 = 4u8;
                    if equal(a3299, a3300) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3301, a3302)?
            };
            let a3314 = {
                let a3311 = {
                    let a3305 = {
                        let a3303 = v1;
                        let a3304 = 2u8;
                        fixed_u8::bit_and(a3303, a3304)
                    };
                    let a3306 = 2u8;
                    if equal(a3305, a3306) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3312 = {
                    let a3309 = {
                        let a3307 = v1;
                        let a3308 = 1u8;
                        fixed_u8::bit_and(a3307, a3308)
                    };
                    let a3310 = 1u8;
                    if equal(a3309, a3310) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3311, a3312)?
            };
            nat_add(a3313, a3314)?
        };
        nat_add(a3315, a3316)?
    };
    let a3317 = 48u64;
    let a3318 = v2;
    if nat_le(a3317, a3318) {
        let a3319 = v2;
        let a3320 = 57u64;
        Ok(nat_le(a3319, a3320))
    } else {
        Ok(false)
    }
}

pub fn f480(v0: u64) -> u32 {
    let a3321 = v0;
    let a3322 = 32u64;
    if nat_lt(a3321, a3322) {
        let a3323 = v0;
        let a3324 = 16u64;
        if nat_lt(a3323, a3324) {
            let a3325 = v0;
            let a3326 = 8u64;
            if nat_lt(a3325, a3326) {
                let a3327 = v0;
                let a3328 = 4u64;
                if nat_lt(a3327, a3328) {
                    let a3329 = v0;
                    let a3330 = 2u64;
                    if nat_lt(a3329, a3330) {
                        let a3331 = v0;
                        let a3332 = 1u64;
                        if nat_lt(a3331, a3332) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a3333 = v0;
                        let a3334 = 3u64;
                        if nat_lt(a3333, a3334) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a3335 = v0;
                    let a3336 = 6u64;
                    if nat_lt(a3335, a3336) {
                        let a3337 = v0;
                        let a3338 = 5u64;
                        if nat_lt(a3337, a3338) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a3339 = v0;
                        let a3340 = 7u64;
                        if nat_lt(a3339, a3340) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a3341 = v0;
                let a3342 = 12u64;
                if nat_lt(a3341, a3342) {
                    let a3343 = v0;
                    let a3344 = 10u64;
                    if nat_lt(a3343, a3344) {
                        let a3345 = v0;
                        let a3346 = 9u64;
                        if nat_lt(a3345, a3346) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a3347 = v0;
                        let a3348 = 11u64;
                        if nat_lt(a3347, a3348) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a3349 = v0;
                    let a3350 = 14u64;
                    if nat_lt(a3349, a3350) {
                        let a3351 = v0;
                        let a3352 = 13u64;
                        if nat_lt(a3351, a3352) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a3353 = v0;
                        let a3354 = 15u64;
                        if nat_lt(a3353, a3354) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a3355 = v0;
            let a3356 = 24u64;
            if nat_lt(a3355, a3356) {
                let a3357 = v0;
                let a3358 = 20u64;
                if nat_lt(a3357, a3358) {
                    let a3359 = v0;
                    let a3360 = 18u64;
                    if nat_lt(a3359, a3360) {
                        let a3361 = v0;
                        let a3362 = 17u64;
                        if nat_lt(a3361, a3362) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a3363 = v0;
                        let a3364 = 19u64;
                        if nat_lt(a3363, a3364) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a3365 = v0;
                    let a3366 = 22u64;
                    if nat_lt(a3365, a3366) {
                        let a3367 = v0;
                        let a3368 = 21u64;
                        if nat_lt(a3367, a3368) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a3369 = v0;
                        let a3370 = 23u64;
                        if nat_lt(a3369, a3370) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a3371 = v0;
                let a3372 = 28u64;
                if nat_lt(a3371, a3372) {
                    let a3373 = v0;
                    let a3374 = 26u64;
                    if nat_lt(a3373, a3374) {
                        let a3375 = v0;
                        let a3376 = 25u64;
                        if nat_lt(a3375, a3376) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a3377 = v0;
                        let a3378 = 27u64;
                        if nat_lt(a3377, a3378) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a3379 = v0;
                    let a3380 = 30u64;
                    if nat_lt(a3379, a3380) {
                        let a3381 = v0;
                        let a3382 = 29u64;
                        if nat_lt(a3381, a3382) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a3383 = v0;
                        let a3384 = 31u64;
                        if nat_lt(a3383, a3384) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a3385 = v0;
        let a3386 = 48u64;
        if nat_lt(a3385, a3386) {
            let a3387 = v0;
            let a3388 = 40u64;
            if nat_lt(a3387, a3388) {
                let a3389 = v0;
                let a3390 = 36u64;
                if nat_lt(a3389, a3390) {
                    let a3391 = v0;
                    let a3392 = 34u64;
                    if nat_lt(a3391, a3392) {
                        let a3393 = v0;
                        let a3394 = 33u64;
                        if nat_lt(a3393, a3394) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a3395 = v0;
                        let a3396 = 35u64;
                        if nat_lt(a3395, a3396) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a3397 = v0;
                    let a3398 = 38u64;
                    if nat_lt(a3397, a3398) {
                        let a3399 = v0;
                        let a3400 = 37u64;
                        if nat_lt(a3399, a3400) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a3401 = v0;
                        let a3402 = 39u64;
                        if nat_lt(a3401, a3402) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a3403 = v0;
                let a3404 = 44u64;
                if nat_lt(a3403, a3404) {
                    let a3405 = v0;
                    let a3406 = 42u64;
                    if nat_lt(a3405, a3406) {
                        let a3407 = v0;
                        let a3408 = 41u64;
                        if nat_lt(a3407, a3408) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a3409 = v0;
                        let a3410 = 43u64;
                        if nat_lt(a3409, a3410) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a3411 = v0;
                    let a3412 = 46u64;
                    if nat_lt(a3411, a3412) {
                        let a3413 = v0;
                        let a3414 = 45u64;
                        if nat_lt(a3413, a3414) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a3415 = v0;
                        let a3416 = 47u64;
                        if nat_lt(a3415, a3416) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a3417 = v0;
            let a3418 = 56u64;
            if nat_lt(a3417, a3418) {
                let a3419 = v0;
                let a3420 = 52u64;
                if nat_lt(a3419, a3420) {
                    let a3421 = v0;
                    let a3422 = 50u64;
                    if nat_lt(a3421, a3422) {
                        let a3423 = v0;
                        let a3424 = 49u64;
                        if nat_lt(a3423, a3424) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a3425 = v0;
                        let a3426 = 51u64;
                        if nat_lt(a3425, a3426) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a3427 = v0;
                    let a3428 = 54u64;
                    if nat_lt(a3427, a3428) {
                        let a3429 = v0;
                        let a3430 = 53u64;
                        if nat_lt(a3429, a3430) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a3431 = v0;
                        let a3432 = 55u64;
                        if nat_lt(a3431, a3432) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a3433 = v0;
                let a3434 = 60u64;
                if nat_lt(a3433, a3434) {
                    let a3435 = v0;
                    let a3436 = 58u64;
                    if nat_lt(a3435, a3436) {
                        let a3437 = v0;
                        let a3438 = 57u64;
                        if nat_lt(a3437, a3438) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a3439 = v0;
                        let a3440 = 59u64;
                        if nat_lt(a3439, a3440) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a3441 = v0;
                    let a3442 = 62u64;
                    if nat_lt(a3441, a3442) {
                        let a3443 = v0;
                        let a3444 = 61u64;
                        if nat_lt(a3443, a3444) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a3445 = v0;
                        let a3446 = 63u64;
                        if nat_lt(a3445, a3446) {
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

pub fn f481(v0: u32) -> u32 {
    let a3449 = {
        let a3447 = f544(v0, 6u32, 26u32);
        let a3448 = f544(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a3447, a3448)
    };
    let a3450 = f544(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a3449, a3450)
}

pub fn f482(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3456 = {
        let a3451 = v0;
        let a3452 = v1;
        fixed_u32::bit_and(a3451, a3452)
    };
    let a3457 = {
        let a3454 = {
            let a3453 = v0;
            fixed_u32::bit_not(a3453)
        };
        let a3455 = v2;
        fixed_u32::bit_and(a3454, a3455)
    };
    fixed_u32::bit_xor(a3456, a3457)
}

pub fn f483(v0: u32) -> u32 {
    let a3460 = {
        let a3458 = f544(v0, 2u32, 30u32);
        let a3459 = f544(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a3458, a3459)
    };
    let a3461 = f544(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a3460, a3461)
}

pub fn f484(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3470 = {
        let a3466 = {
            let a3462 = v0;
            let a3463 = v1;
            fixed_u32::bit_and(a3462, a3463)
        };
        let a3467 = {
            let a3464 = v0;
            let a3465 = v2;
            fixed_u32::bit_and(a3464, a3465)
        };
        fixed_u32::bit_xor(a3466, a3467)
    };
    let a3471 = {
        let a3468 = v1;
        let a3469 = v2;
        fixed_u32::bit_and(a3468, a3469)
    };
    fixed_u32::bit_xor(a3470, a3471)
}

pub fn f485(v0: Adt21) -> Adt21 {
    Adt21::C0(match v0.clone() {
        Adt21::C0(_, h3472, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3472
        }
    }, match v0.clone() {
        Adt21::C0(_, _, h3473, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3473
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, h3474, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3474
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, h3475, _, _, _, _, _, _, _, _, _, _, _) => {
            h3475
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, h3476, _, _, _, _, _, _, _, _, _, _) => {
            h3476
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, h3477, _, _, _, _, _, _, _, _, _) => {
            h3477
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, _, h3478, _, _, _, _, _, _, _, _) => {
            h3478
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, _, _, h3479, _, _, _, _, _, _, _) => {
            h3479
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, _, _, _, h3480, _, _, _, _, _, _) => {
            h3480
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, _, _, _, _, h3481, _, _, _, _, _) => {
            h3481
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, _, _, _, _, _, h3482, _, _, _, _) => {
            h3482
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3483, _, _, _) => {
            h3483
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3484, _, _) => {
            h3484
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3485, _) => {
            h3485
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3486) => {
            h3486
        }
    }, f299(f545(match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3487, _) => {
            h3487
        }
    }), f299(match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, _, _, _, h3488, _, _, _, _, _, _) => {
            h3488
        }
    }, f299(f546(match v0.clone() {
        Adt21::C0(_, h3489, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3489
        }
    }), match v0 {
        Adt21::C0(h3490, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3490
        }
    }))))
}

pub fn f486(v0: Fn15, p3494: u64, p3495: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m3493 = (p3494, p3495);
    {
        let mut m3493 = m3493;
        loop {
            let step = {
                let (v1, v2) = m3493;
                let m3491 = v1;
                if m3491 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m3491 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c3492: Fn15 = v0.clone();
                        c3492.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m3493 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f487(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h3496, _) = v0;
        h3496
    };
    let v2: u32 = {
        let (h3497, _) = {
            let (_, h3498) = v0;
            h3498
        };
        h3497
    };
    let v3: u32 = {
        let (_, h3499) = {
            let (_, h3500) = v0;
            h3500
        };
        h3499
    };
    ({
        let a3501 = v1;
        let a3502 = 2u64;
        let a3503 = 0u64;
        nat_quot(a3501, a3502, a3503)
    }, (f488(v2, 1u32), {
        let a3507 = {
            let a3504 = v1;
            let a3505 = 2u64;
            let a3506 = 0u64;
            nat_rem(a3504, a3505, a3506)
        };
        let a3508 = 1u64;
        if nat_eq(a3507, a3508) {
            let a3509 = v3;
            let a3510 = v2;
            fixed_u32::bit_or(a3509, a3510)
        } else {
            v3
        }
    }))
}

pub fn f488(v0: u32, v1: u32) -> u32 {
    let m3513 = {
        let a3511 = v0;
        let a3512 = v1;
        fixed_u32::shift_left(a3511, a3512)
    };
    match m3513 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f489(p3516: List<u64>, p3517: List<u64>) -> List<u64> {
    let m3515 = (p3516, p3517);
    {
        let mut m3515 = m3515;
        loop {
            let step = {
                let (v0, v1) = m3515;
                let m3514 = v0;
                match m3514.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3515 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f490(v0: List<u64>) -> R<Bytes> {
    let m3518 = v0;
    match m3518.uncons() {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some((v1, v2)) => {
            let a3519 = f547(v1)?;
            let a3520 = f490(v2)?;
            Ok(append_bytes(a3519, a3520))
        }
    }
}

pub fn f491(v0: List<u64>, v1: u64, v2: u64) -> R<List<u64>> {
    let m3521 = v0;
    match m3521.uncons() {
        None => {
            let a3522 = v2;
            let a3523 = 0u64;
            if nat_eq(a3522, a3523) {
                Ok(List::<u64>::nil())
            } else {
                Ok(List::cons(v2, List::<u64>::nil()))
            }
        }
        Some((v3, v4)) => {
            let v5: u64 = {
                let a3526 = {
                    let a3524 = v3;
                    let a3525 = v1;
                    nat_mul(a3524, a3525)?
                };
                let a3527 = v2;
                nat_add(a3526, a3527)?
            };
            Ok(List::cons({
                let a3528 = v5;
                let a3529 = f492();
                let a3530 = 0u64;
                nat_rem(a3528, a3529, a3530)
            }, f491(v4, v1, {
                let a3531 = v5;
                let a3532 = f492();
                let a3533 = 0u64;
                nat_quot(a3531, a3532, a3533)
            })?))
        }
    }
}

pub fn f492() -> u64 {
    1000000000u64
}

pub fn f493(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3578 = {
            let a3554 = {
                let a3542 = {
                    let a3536 = {
                        let a3534 = v1;
                        let a3535 = 128u8;
                        fixed_u8::bit_and(a3534, a3535)
                    };
                    let a3537 = 128u8;
                    if equal(a3536, a3537) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3543 = {
                    let a3540 = {
                        let a3538 = v1;
                        let a3539 = 64u8;
                        fixed_u8::bit_and(a3538, a3539)
                    };
                    let a3541 = 64u8;
                    if equal(a3540, a3541) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3542, a3543)?
            };
            let a3555 = {
                let a3552 = {
                    let a3546 = {
                        let a3544 = v1;
                        let a3545 = 32u8;
                        fixed_u8::bit_and(a3544, a3545)
                    };
                    let a3547 = 32u8;
                    if equal(a3546, a3547) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3553 = {
                    let a3550 = {
                        let a3548 = v1;
                        let a3549 = 16u8;
                        fixed_u8::bit_and(a3548, a3549)
                    };
                    let a3551 = 16u8;
                    if equal(a3550, a3551) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3552, a3553)?
            };
            nat_add(a3554, a3555)?
        };
        let a3579 = {
            let a3576 = {
                let a3564 = {
                    let a3558 = {
                        let a3556 = v1;
                        let a3557 = 8u8;
                        fixed_u8::bit_and(a3556, a3557)
                    };
                    let a3559 = 8u8;
                    if equal(a3558, a3559) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3565 = {
                    let a3562 = {
                        let a3560 = v1;
                        let a3561 = 4u8;
                        fixed_u8::bit_and(a3560, a3561)
                    };
                    let a3563 = 4u8;
                    if equal(a3562, a3563) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3564, a3565)?
            };
            let a3577 = {
                let a3574 = {
                    let a3568 = {
                        let a3566 = v1;
                        let a3567 = 2u8;
                        fixed_u8::bit_and(a3566, a3567)
                    };
                    let a3569 = 2u8;
                    if equal(a3568, a3569) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3575 = {
                    let a3572 = {
                        let a3570 = v1;
                        let a3571 = 1u8;
                        fixed_u8::bit_and(a3570, a3571)
                    };
                    let a3573 = 1u8;
                    if equal(a3572, a3573) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3574, a3575)?
            };
            nat_add(a3576, a3577)?
        };
        nat_add(a3578, a3579)?
    };
    let a3586 = f185(f548(), {
        let a3580 = v2;
        let a3581 = 16u64;
        let a3582 = 0u64;
        nat_quot(a3580, a3581, a3582)
    });
    let a3587 = f185(f548(), {
        let a3583 = v2;
        let a3584 = 16u64;
        let a3585 = 0u64;
        nat_rem(a3583, a3584, a3585)
    });
    Ok(append_bytes(a3586, a3587))
}

pub fn f494(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f549(Fn21::F550, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f495(v0: Fn16, p3591: List<List<(Bytes, Bytes)>>, p3592: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m3590 = (p3591, p3592);
    {
        let mut m3590 = m3590;
        loop {
            let step = {
                let (v1, v2) = m3590;
                let m3588 = v2;
                match m3588.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c3589: Fn16 = v0.clone();
                            c3589.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3590 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f496(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f497(v0: Fn17, p3597: u64, p3598: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m3596 = (p3597, p3598);
    {
        let mut m3596 = m3596;
        loop {
            let step = {
                let (v1, v2) = m3596;
                let m3593 = v1;
                if m3593 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3593 - 1;
                    let m3595 = {
                        let c3594: Fn17 = v0.clone();
                        c3594.apply(v2.clone())?
                    };
                    match m3595 {
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
                Ok(next) => { m3596 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f498(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f551(v0)
}

pub fn f499(v0: Fn18, p3602: List<List<(Bytes, ())>>, p3603: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m3601 = (p3602, p3603);
    {
        let mut m3601 = m3601;
        loop {
            let step = {
                let (v1, v2) = m3601;
                let m3599 = v2;
                match m3599.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c3600: Fn18 = v0.clone();
                            c3600.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3601 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f500(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f501(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a3605 = {
        let a3604 = v0.clone();
        length_list(a3604)
    };
    let a3606 = 1u64;
    if nat_le(a3605, a3606) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f552(v0)?))
    }
}

pub fn f502(v0: Fn19, p3610: List<(Bytes, ())>, p3611: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m3609 = (p3610, p3611);
    {
        let mut m3609 = m3609;
        loop {
            let step = {
                let (v1, v2) = m3609;
                let m3607 = v2;
                match m3607.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c3608: Fn19 = v0.clone();
                            c3608.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3609 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f503(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f504(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    if f125(v0.clone(), v1, v2) {
        Ok(None::<u64>)
    } else {
        let a3613 = v2;
        let a3614 = {
            let a3612 = v0;
            length_bytes(a3612)
        };
        if nat_lt(a3613, a3614) {
            Ok(Some({
                let a3615 = v2;
                let a3616 = 1u64;
                nat_add(a3615, a3616)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f505(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a3617 = 0u64;
    let a3618 = v1;
    if if nat_lt(a3617, a3618) {
        f184({
            let a3621 = f182(v0, {
                let a3619 = v1;
                let a3620 = 1u64;
                nat_sub(a3619, a3620)
            })?;
            let a3622 = 10u64;
            nat_eq(a3621, a3622)
        })
    } else {
        false
    } {
        Ok(Some({
            let a3623 = v1;
            let a3624 = 1u64;
            nat_sub(a3623, a3624)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f506(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f242(f182(v0, v1)?) {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3625 = v1;
            let a3626 = 1u64;
            nat_add(a3625, a3626)?
        }))
    }
}

pub fn f507(v0: Bytes, v1: u64, v2: u64) -> R<Adt26> {
    let v3: (u64, u64) = {
        let (h3627, _) = f211(Fn6::F553(v2, v0.clone()), {
            let a3629 = {
                let a3628 = v0.clone();
                length_bytes(a3628)
            };
            let a3630 = 1u64;
            nat_add(a3629, a3630)?
        }, (v1, 0u64))?;
        h3627
    };
    let v4: u64 = f508(v0.clone(), {
        let (h3631, _) = v3;
        h3631
    }, v2)?;
    let a3632 = f182(v0, v4)?;
    let a3633 = 9u64;
    if if nat_eq(a3632, a3633) {
        let a3637 = {
            let a3635 = v4;
            let a3636 = {
                let (h3634, _) = v3;
                h3634
            };
            nat_sub(a3635, a3636)
        };
        let a3638 = v2;
        nat_lt(a3637, a3638)
    } else {
        false
    } {
        Ok(Adt26::C1(f129(Str::lit("a tab cannot indent a block scalar line (YAML 1.2.2 [63], [70])"), v4)?))
    } else {
        Ok(Adt26::C0({
            let (_, h3639) = v3;
            h3639
        }, {
            let (h3640, _) = v3;
            h3640
        }))
    }
}

pub fn f508(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h3641, _) = f176(Fn5::F554(v2, v0.clone(), v1), {
        let a3643 = {
            let a3642 = v0;
            length_bytes(a3642)
        };
        let a3644 = 1u64;
        nat_add(a3643, a3644)?
    }, v1)?;
    Ok(h3641)
}

pub fn f509(v0: Bytes, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = f508(v0.clone(), v1, v2)?;
    let a3647 = {
        let a3645 = v3;
        let a3646 = v1;
        nat_sub(a3645, a3646)
    };
    let a3648 = v2;
    if nat_eq(a3647, a3648) {
        Ok(f184({
            let a3649 = f182(v0, v3)?;
            let a3650 = 256u64;
            nat_eq(a3649, a3650)
        }))
    } else {
        Ok(false)
    }
}

pub fn f510(v0: Bytes, p3660: u64, v2: u64, v3: bool, p3661: u64, p3662: Bytes, p3663: u64) -> R<Adt27> {
    let m3659 = (p3660, (p3661, (p3662, p3663)));
    {
        let mut m3659 = m3659;
        loop {
            let step = {
                let (v1, (v4, (v5, v6))) = m3659;
                let m3651 = v4;
                if m3651 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<Adt27>>(Ok(Adt27::C1(f129(Str::lit("a block scalar has more lines than the input"), v1)?)))
                } else {
                    let v7 = m3651 - 1;
                    let v8: u64 = f508(v0.clone(), v1, v2)?;
                    let v9: bool = f184(f241(f182(v0.clone(), v8)?));
                    let v10: u64 = f330(v0.clone(), v8)?;
                    let v11: Bytes = f7(List::cons(v5, List::cons(f426(v6), List::cons(f258(v0.clone(), v8, v10), List::<Bytes>::nil()))));
                    let m3652 = f507(v0.clone(), f245(v0.clone(), v10)?, v2)?;
                    match m3652 {
                        Adt26::C0(v13, v14) => {
                            let v15: u64 = v14;
                            let v16: u64 = f508(v0.clone(), v15, v2)?;
                            if f509(v0.clone(), v15, v2)? {
                                Ok::<(u64, (u64, (Bytes, u64))), R<Adt27>>((v15, (v7, ({
                                    let a3655 = v11;
                                    let a3656 = if if if v3 {
                                        v9
                                    } else {
                                        false
                                    } {
                                        f184(f241(f182(v0.clone(), v16)?))
                                    } else {
                                        false
                                    } {
                                        let a3653 = v13;
                                        let a3654 = 0u64;
                                        if nat_eq(a3653, a3654) {
                                            Bytes::lit(&[0x20])
                                        } else {
                                            Bytes::lit(&[])
                                        }
                                    } else {
                                        Bytes::lit(&[0x0a])
                                    };
                                    append_bytes(a3655, a3656)
                                }, v13))))
                            } else {
                                Err::<(u64, (u64, (Bytes, u64))), R<Adt27>>(Ok(Adt27::C0(v11, v13, {
                                    let a3657 = f182(v0.clone(), v16)?;
                                    let a3658 = 256u64;
                                    if nat_eq(a3657, a3658) {
                                        v16
                                    } else {
                                        v15
                                    }
                                })))
                            }
                        }
                        Adt26::C1(v12) => {
                            Err::<(u64, (u64, (Bytes, u64))), R<Adt27>>(Ok(Adt27::C1(v12)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3659 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f511(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f246(v0.clone(), {
        let (h3664, _) = v1;
        h3664
    })?;
    let a3665 = f182(v0, v2)?;
    let a3666 = 10u64;
    if nat_eq(a3665, a3666) {
        Ok(Some(({
            let a3667 = v2;
            let a3668 = 1u64;
            nat_add(a3667, a3668)?
        }, f428({
            let (_, h3669) = v1;
            h3669
        }, {
            let a3671 = v2;
            let a3672 = {
                let (h3670, _) = v1;
                h3670
            };
            nat_sub(a3671, a3672)
        }))))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f512(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f555(v0, v1)
}

pub fn f513(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt19> {
    let v4: (u64, u64) = f515(v0.clone(), v1, 2u64)?;
    let a3675 = {
        let (h3673, _) = v4;
        h3673
    };
    let a3676 = {
        let a3674 = v0.clone();
        length_bytes(a3674)
    };
    if nat_eq(a3675, a3676) {
        if f159(false, {
            let (_, h3677) = v4;
            h3677
        }) {
            Ok(Adt19::C0(Adt2::C3(false, {
                let (_, h3678) = v4;
                h3678
            }), v3))
        } else {
            Ok(Adt19::C1(f514(v2)?))
        }
    } else {
        Ok(Adt19::C0(Adt2::C6(v0), v3))
    }
}

pub fn f514(v0: u64) -> R<Bytes> {
    f129(Str::lit("a number outside the range Studio.Value holds is not read (YAML 1.2.2 section 10.3.2)"), v0)
}

pub fn f515(v0: Bytes, v1: u64, v2: u64) -> R<(u64, u64)> {
    let (h3679, _) = f211(Fn6::F556(v1, v0.clone()), {
        let a3681 = {
            let a3680 = v0;
            length_bytes(a3680)
        };
        let a3682 = 1u64;
        nat_add(a3681, a3682)?
    }, (v2, 0u64))?;
    Ok(h3679)
}

pub fn f516(v0: u64) -> R<Option<u64>> {
    let v1: u64 = f257(f8(Str::lit("abtnvfr")), v0)?;
    let a3683 = v1;
    let a3684 = 7u64;
    if nat_lt(a3683, a3684) {
        Ok(Some({
            let a3685 = v1;
            let a3686 = 7u64;
            nat_add(a3685, a3686)?
        }))
    } else {
        let a3687 = v0;
        let a3688 = 48u64;
        if nat_eq(a3687, a3688) {
            Ok(Some(0u64))
        } else {
            let a3689 = v0;
            let a3690 = 9u64;
            if nat_eq(a3689, a3690) {
                Ok(Some(9u64))
            } else {
                let a3691 = v0;
                let a3692 = 101u64;
                if nat_eq(a3691, a3692) {
                    Ok(Some(27u64))
                } else {
                    let a3693 = v0;
                    let a3694 = 78u64;
                    if nat_eq(a3693, a3694) {
                        Ok(Some(133u64))
                    } else {
                        let a3695 = v0;
                        let a3696 = 95u64;
                        if nat_eq(a3695, a3696) {
                            Ok(Some(160u64))
                        } else {
                            let a3697 = v0;
                            let a3698 = 76u64;
                            if nat_eq(a3697, a3698) {
                                Ok(Some(8232u64))
                            } else {
                                let a3699 = v0;
                                let a3700 = 80u64;
                                if nat_eq(a3699, a3700) {
                                    Ok(Some(8233u64))
                                } else {
                                    let a3701 = f257(f8(Str::lit(" \"/\\")), v0)?;
                                    let a3702 = 4u64;
                                    if nat_lt(a3701, a3702) {
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

pub fn f517(v0: u64) -> R<Bytes> {
    let a3703 = v0;
    let a3704 = 128u64;
    if nat_lt(a3703, a3704) {
        Ok(f557(f558(), v0))
    } else {
        let a3705 = v0;
        let a3706 = 2048u64;
        if nat_lt(a3705, a3706) {
            let a3714 = f557(f559(), {
                let a3712 = {
                    let a3710 = {
                        let a3707 = v0;
                        let a3708 = 64u64;
                        let a3709 = 0u64;
                        nat_quot(a3707, a3708, a3709)
                    };
                    let a3711 = 2u64;
                    nat_sub(a3710, a3711)
                };
                let a3713 = 2u64;
                nat_mul(a3712, a3713)?
            });
            let a3715 = f560(v0)?;
            Ok(append_bytes(a3714, a3715))
        } else {
            let a3716 = v0;
            let a3717 = 65536u64;
            if nat_lt(a3716, a3717) {
                let a3728 = f557(f561(), {
                    let a3721 = {
                        let a3718 = v0;
                        let a3719 = 4096u64;
                        let a3720 = 0u64;
                        nat_quot(a3718, a3719, a3720)
                    };
                    let a3722 = 3u64;
                    nat_mul(a3721, a3722)?
                });
                let a3729 = {
                    let a3726 = f560({
                        let a3723 = v0;
                        let a3724 = 64u64;
                        let a3725 = 0u64;
                        nat_quot(a3723, a3724, a3725)
                    })?;
                    let a3727 = f560(v0)?;
                    append_bytes(a3726, a3727)
                };
                Ok(append_bytes(a3728, a3729))
            } else {
                let a3743 = f557(Bytes::lit(&[0xf0, 0xf1, 0xf2, 0xf3, 0xf4]), {
                    let a3730 = v0;
                    let a3731 = 262144u64;
                    let a3732 = 0u64;
                    nat_quot(a3730, a3731, a3732)
                });
                let a3744 = {
                    let a3741 = f560({
                        let a3733 = v0;
                        let a3734 = 4096u64;
                        let a3735 = 0u64;
                        nat_quot(a3733, a3734, a3735)
                    })?;
                    let a3742 = {
                        let a3739 = f560({
                            let a3736 = v0;
                            let a3737 = 64u64;
                            let a3738 = 0u64;
                            nat_quot(a3736, a3737, a3738)
                        })?;
                        let a3740 = f560(v0)?;
                        append_bytes(a3739, a3740)
                    };
                    append_bytes(a3741, a3742)
                };
                Ok(append_bytes(a3743, a3744))
            }
        }
    }
}

pub fn f518(v0: u64) -> R<Bytes> {
    f129(Str::lit("an implicit key must be on one line (YAML 1.2.2 [154])"), v0)
}

pub fn f519(v0: Bytes, p3759: u64, v2: u64, p3760: u64, p3761: u64) -> R<Adt26> {
    let m3758 = (p3759, (p3760, p3761));
    {
        let mut m3758 = m3758;
        loop {
            let step = {
                let (v1, (v3, v4)) = m3758;
                let m3745 = v3;
                if m3745 == 0 {
                    Err::<(u64, (u64, u64)), R<Adt26>>(Ok(Adt26::C1(f129(Str::lit("the input ends inside a scalar"), v1)?)))
                } else {
                    let v5 = m3745 - 1;
                    let v6: u64 = {
                        let a3746 = v1;
                        let a3747 = 1u64;
                        nat_add(a3746, a3747)?
                    };
                    let v7: u64 = f246(v0.clone(), v6)?;
                    let v8: u64 = f244(v0.clone(), v7)?;
                    let v9: u64 = f182(v0.clone(), v8)?;
                    if f180(v0.clone(), v6)? {
                        Err::<(u64, (u64, u64)), R<Adt26>>(Ok(Adt26::C1(f129(Str::lit("a document marker cannot appear inside a scalar (YAML 1.2.2 [206])"), v6)?)))
                    } else {
                        let a3748 = v9;
                        let a3749 = 10u64;
                        if nat_eq(a3748, a3749) {
                            Ok::<(u64, (u64, u64)), R<Adt26>>((v8, (v5, {
                                let a3750 = v4;
                                let a3751 = 1u64;
                                nat_add(a3750, a3751)?
                            })))
                        } else {
                            let a3752 = v9;
                            let a3753 = 256u64;
                            if nat_eq(a3752, a3753) {
                                Err::<(u64, (u64, u64)), R<Adt26>>(Ok(Adt26::C1(f129(Str::lit("the input ends inside a quoted scalar (YAML 1.2.2 [109], [120])"), v8)?)))
                            } else {
                                let a3756 = {
                                    let a3754 = v7;
                                    let a3755 = v6;
                                    nat_sub(a3754, a3755)
                                };
                                let a3757 = v2;
                                if nat_lt(a3756, a3757) {
                                    Err::<(u64, (u64, u64)), R<Adt26>>(Ok(Adt26::C1(f129(Str::lit("a continuation line of a scalar is indented less than its node (YAML 1.2.2 [69])"), v7)?)))
                                } else {
                                    Err::<(u64, (u64, u64)), R<Adt26>>(Ok(Adt26::C0(v4, v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3758 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f520(v0: Bytes, p3771: u64, p3772: u64, p3773: u64) -> R<Option<u64>> {
    let m3770 = (p3771, (p3772, p3773));
    {
        let mut m3770 = m3770;
        loop {
            let step = {
                let (v1, (v2, v3)) = m3770;
                let m3762 = v2;
                if m3762 == 0 {
                    Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3762 - 1;
                    let m3763 = f562(f182(v0.clone(), v1)?);
                    match m3763 {
                        None => {
                            Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(None::<u64>))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, u64)), R<Option<u64>>>(({
                                let a3764 = v1;
                                let a3765 = 1u64;
                                nat_add(a3764, a3765)?
                            }, (v4, {
                                let a3768 = {
                                    let a3766 = v3;
                                    let a3767 = 16u64;
                                    nat_mul(a3766, a3767)?
                                };
                                let a3769 = v5;
                                nat_add(a3768, a3769)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3770 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f521(v0: Bytes, v1: u64) -> R<u64> {
    let (h3774, _) = f176(Fn5::F563(v0.clone()), {
        let a3776 = {
            let a3775 = v0;
            length_bytes(a3775)
        };
        let a3777 = 1u64;
        nat_add(a3776, a3777)?
    }, v1)?;
    Ok(h3774)
}

pub fn f522(v0: Bytes, v1: u64) -> R<u64> {
    let (h3778, _) = f176(Fn5::F564(v0.clone()), {
        let a3780 = {
            let a3779 = v0;
            length_bytes(a3779)
        };
        let a3781 = 1u64;
        nat_add(a3780, a3781)?
    }, v1)?;
    Ok(h3778)
}

pub fn f523(v0: Fn20, p3786: u64, p3787: (u64, bool)) -> R<((u64, bool), bool)> {
    let m3785 = (p3786, p3787);
    {
        let mut m3785 = m3785;
        loop {
            let step = {
                let (v1, v2) = m3785;
                let m3782 = v1;
                if m3782 == 0 {
                    Err::<(u64, (u64, bool)), R<((u64, bool), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3782 - 1;
                    let m3784 = {
                        let c3783: Fn20 = v0.clone();
                        c3783.apply(v2)?
                    };
                    match m3784 {
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
                Ok(next) => { m3785 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f524(v0: u64, v1: Bytes, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    f565(v1, v0, v2)
}

pub fn f525(v0: Bytes, v1: u64, v2: u64) -> R<Adt23> {
    let v3: u64 = f182(v0.clone(), v1)?;
    let a3788 = v3;
    let a3789 = 34u64;
    if if nat_eq(a3788, a3789) {
        true
    } else {
        let a3790 = v3;
        let a3791 = 39u64;
        nat_eq(a3790, a3791)
    } {
        f335(v0, v1, v2, false)
    } else {
        let m3792 = f249(v0.clone(), v1)?;
        match m3792 {
            None => {
                let a3793 = v3;
                let a3794 = 91u64;
                if if nat_eq(a3793, a3794) {
                    true
                } else {
                    let a3795 = v3;
                    let a3796 = 123u64;
                    nat_eq(a3795, a3796)
                } {
                    Ok(Adt23::C1(f129(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [142])"), v1)?))
                } else {
                    if f337(v0.clone(), v1, true)? {
                        let v5: u64 = f338(v0.clone(), v1, true)?;
                        let v6: u64 = f244(v0.clone(), v5)?;
                        let a3797 = f182(v0.clone(), v6)?;
                        let a3798 = 10u64;
                        if nat_eq(a3797, a3798) {
                            let m3799 = f438(v0.clone(), v6, v2)?;
                            match m3799 {
                                Ok(v8) => {
                                    let v9: u64 = f182(v0.clone(), v8)?;
                                    let a3800 = v9;
                                    let a3801 = 58u64;
                                    if if if nat_eq(a3800, a3801) {
                                        true
                                    } else {
                                        let a3802 = v9;
                                        let a3803 = 44u64;
                                        nat_eq(a3802, a3803)
                                    } {
                                        true
                                    } else {
                                        let a3804 = v9;
                                        let a3805 = 125u64;
                                        nat_eq(a3804, a3805)
                                    } {
                                        Ok(Adt23::C0(f258(v0, v1, v5), v5))
                                    } else {
                                        Ok(Adt23::C1(f129(Str::lit("a multi-line plain scalar inside a flow collection is not read (YAML 1.2.2 [135])"), v6)?))
                                    }
                                }
                                Err(v7) => {
                                    Ok(Adt23::C1(v7))
                                }
                            }
                        } else {
                            Ok(Adt23::C0(f258(v0, v1, v5), v5))
                        }
                    } else {
                        Ok(Adt23::C1(f129(Str::lit("a flow mapping key must be a scalar (YAML 1.2.2 [142])"), v1)?))
                    }
                }
            }
            Some(v4) => {
                Ok(Adt23::C1(v4))
            }
        }
    }
}

pub fn f526(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = {
        let (h3806, _) = v2;
        h3806
    };
    let v4: u64 = f182(v0.clone(), v3)?;
    if f242(v4) {
        Ok(None::<(u64, u64)>)
    } else {
        if f241(v4) {
            Ok(Some(({
                let a3807 = v3;
                let a3808 = 1u64;
                nat_add(a3807, a3808)?
            }, {
                let (_, h3809) = v2;
                h3809
            })))
        } else {
            let a3810 = v4;
            let a3811 = 58u64;
            if if nat_eq(a3810, a3811) {
                f184(f332(f182(v0.clone(), {
                    let a3812 = v3;
                    let a3813 = 1u64;
                    nat_add(a3812, a3813)?
                })?, v1))
            } else {
                false
            } {
                Ok(None::<(u64, u64)>)
            } else {
                let a3814 = v4;
                let a3815 = 35u64;
                if if nat_eq(a3814, a3815) {
                    f241(f182(v0, {
                        let a3816 = v3;
                        let a3817 = 1u64;
                        nat_sub(a3816, a3817)
                    })?)
                } else {
                    false
                } {
                    Ok(None::<(u64, u64)>)
                } else {
                    if if v1 {
                        f421(v4)
                    } else {
                        false
                    } {
                        Ok(None::<(u64, u64)>)
                    } else {
                        Ok(Some(({
                            let a3818 = v3;
                            let a3819 = 1u64;
                            nat_add(a3818, a3819)?
                        }, {
                            let a3820 = v3;
                            let a3821 = 1u64;
                            nat_add(a3820, a3821)?
                        })))
                    }
                }
            }
        }
    }
}

pub fn f527(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f566(v0, v1)
}

pub fn f528(v0: Bytes, v1: bool, v2: u64) -> R<Option<u64>> {
    let v3: u64 = f182(v0, v2)?;
    if if if f241(v3) {
        true
    } else {
        f242(v3)
    } {
        true
    } else {
        if v1 {
            let a3822 = v3;
            let a3823 = 62u64;
            nat_eq(a3822, a3823)
        } else {
            f421(v3)
        }
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3824 = v2;
            let a3825 = 1u64;
            nat_add(a3824, a3825)?
        }))
    }
}

pub fn f529(v0: u64, v1: Bytes, p3836: u64, p3837: u64, p3838: bool) -> R<bool> {
    let m3835 = (p3836, (p3837, p3838));
    {
        let mut m3835 = m3835;
        loop {
            let step = {
                let (v2, (v3, v4)) = m3835;
                let m3826 = v3;
                if m3826 == 0 {
                    Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                } else {
                    let v5 = m3826 - 1;
                    let v6: u64 = f567(v1.clone(), v2)?;
                    if v4 {
                        let a3827 = v0;
                        let a3828 = v6;
                        if nat_lt(a3827, a3828) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a3829 = v2;
                                let a3830 = 1u64;
                                nat_add(a3829, a3830)?
                            }, (v5, false)))
                        }
                    } else {
                        let a3831 = v0;
                        let a3832 = v6;
                        if nat_le(a3831, a3832) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(true))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a3833 = v2;
                                let a3834 = 1u64;
                                nat_add(a3833, a3834)?
                            }, (v5, true)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3835 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f530(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    if f568(v1, f323(v0, v2)?) {
        Ok(Some({
            let a3839 = v2;
            let a3840 = 1u64;
            nat_add(a3839, a3840)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f531(p3843: List<Adt14>, p3844: List<Adt14>) -> List<Adt14> {
    let m3842 = (p3843, p3844);
    {
        let mut m3842 = m3842;
        loop {
            let step = {
                let (v0, v1) = m3842;
                let m3841 = v0;
                match m3841.uncons() {
                    None => {
                        Err::<(List<Adt14>, List<Adt14>), List<Adt14>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt14>, List<Adt14>), List<Adt14>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3842 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f532(v0: Bytes, p3849: List<(Bytes, Adt15)>) -> Option<Adt15> {
    let m3848 = p3849;
    {
        let mut m3848 = m3848;
        loop {
            let step = {
                let v1 = m3848;
                let m3845 = v1;
                match m3845.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt15)>, Option<Adt15>>(None::<Adt15>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h3846, _) = v2;
                            h3846
                        }, v0.clone()) {
                            Err::<List<(Bytes, Adt15)>, Option<Adt15>>(Some({
                                let (_, h3847) = v2;
                                h3847
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt15)>, Option<Adt15>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3848 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f533() -> List<(Bytes, Adt15)> {
    List::cons((f8(Str::lit("boolean")), f363(f8(Str::lit("Bool")))), List::cons((f8(Str::lit("byte")), f363(f8(Str::lit("Int")))), List::cons((f8(Str::lit("octet")), f363(f8(Str::lit("Nat")))), List::cons((f8(Str::lit("bigint")), f363(f8(Str::lit("Int")))), List::cons((f8(Str::lit("float")), f363(f569())), List::cons((f8(Str::lit("double")), f363(f569())), List::cons((f8(Str::lit("DOMString")), f363(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("USVString")), f363(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ByteString")), f363(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ArrayBuffer")), f363(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("SharedArrayBuffer")), f363(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("DataView")), f363(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("Int8Array")), f363(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int16Array")), f363(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int32Array")), f363(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("BigInt64Array")), f363(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Uint8Array")), f363(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint16Array")), f363(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint32Array")), f363(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint8ClampedArray")), f363(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("BigUint64Array")), f363(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Float16Array")), f466(f8(Str::lit("Float16Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("Float32Array")), f570(false)), List::cons((f8(Str::lit("Float64Array")), f570(true)), List::cons((f8(Str::lit("any")), f466(f8(Str::lit("any is not translated, because it is the union of every Web IDL type and has no one LexLean type (Web IDL 2.13.1)")))), List::cons((f8(Str::lit("object")), f466(f8(Str::lit("object is not translated, because its values are references to JavaScript objects of any shape, which no LexLean type holds (Web IDL 2.13.20)")))), List::cons((f8(Str::lit("symbol")), f466(f8(Str::lit("symbol is not translated, because a symbol value has identity but no content that a LexLean type holds (Web IDL 2.13.21)")))), List::cons((f8(Str::lit("undefined")), f466(f8(Str::lit("undefined is not translated, because it is the type of the absence of a value (Web IDL 2.13.2)")))), List::<(Bytes, Adt15)>::nil()))))))))))))))))))))))))))))
}

pub fn f534() -> Bytes {
    f8(Str::lit("a union type is translated only as the whole body of a typedef, whose identifier names its inductive (Web IDL 2.11 and 2.13.32)"))
}

pub fn f535(v0: u64) -> Bytes {
    f46({
        let a3850 = Str::lit("0123456789ABCDEF");
        utf8_encode(a3850)
    }, v0, 1u64)
}

pub fn f536(p3861: u64, v1: Bytes, v2: Bytes, p3862: u64) -> R<bool> {
    let m3860 = (p3861, p3862);
    {
        let mut m3860 = m3860;
        loop {
            let step = {
                let (v0, v3) = m3860;
                let m3851 = v0;
                if m3851 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(false))
                } else {
                    let v4 = m3851 - 1;
                    let v5: u64 = f571(v1.clone(), v3)?;
                    let v6: u64 = f571(v2.clone(), v3)?;
                    let a3852 = v5;
                    let a3853 = v6;
                    if nat_lt(a3852, a3853) {
                        Err::<(u64, u64), R<bool>>(Ok(true))
                    } else {
                        let a3854 = v6;
                        let a3855 = v5;
                        if nat_lt(a3854, a3855) {
                            Err::<(u64, u64), R<bool>>(Ok(false))
                        } else {
                            let a3856 = v5;
                            let a3857 = 0u64;
                            if nat_eq(a3856, a3857) {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            } else {
                                Ok::<(u64, u64), R<bool>>((v4, {
                                    let a3858 = v3;
                                    let a3859 = 1u64;
                                    nat_add(a3858, a3859)?
                                }))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3860 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f537(p3875: u64, v1: Bytes, p3876: u64, p3877: Bytes) -> R<Option<Bytes>> {
    let m3874 = (p3875, (p3876, p3877));
    {
        let mut m3874 = m3874;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3874;
                let m3863 = v0;
                if m3863 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3863 - 1;
                    let a3865 = {
                        let a3864 = v1.clone();
                        length_bytes(a3864)
                    };
                    let a3866 = v2;
                    if nat_le(a3865, a3866) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3867 = f325(v1.clone(), v2)?;
                        match m3867 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3869 = v2;
                                    let a3870 = {
                                        let (_, h3868) = v5;
                                        h3868
                                    };
                                    nat_add(a3869, a3870)?
                                }, {
                                    let a3872 = v3;
                                    let a3873 = f572(v1.clone(), v2, {
                                        let (h3871, _) = v5;
                                        h3871
                                    });
                                    append_bytes(a3872, a3873)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3874 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f538(p3887: u64, v1: Bytes, p3888: u64) -> R<bool> {
    let m3886 = (p3887, p3888);
    {
        let mut m3886 = m3886;
        loop {
            let step = {
                let (v0, v2) = m3886;
                let m3878 = v0;
                if m3878 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m3878 - 1;
                    let m3881 = {
                        let a3879 = v1.clone();
                        let a3880 = v2;
                        index_bytes(a3879, a3880)
                    };
                    match m3881 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a3882 = f60(v4)?;
                            let a3883 = 128u64;
                            if nat_lt(a3882, a3883) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a3884 = v2;
                                    let a3885 = 1u64;
                                    nat_add(a3884, a3885)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3886 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f539(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f167(v0.clone(), 0u64, {
        let a3889 = v0;
        length_bytes(a3889)
    }, false, Bytes::lit(&[]))?;
    let m3892 = {
        let a3890 = v1.clone();
        let a3891 = 0u64;
        index_bytes(a3890, a3891)
    };
    match m3892 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f60(v2)?;
            if f221(v3) {
                let a3899 = f46({
                    let a3893 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a3893)
                }, {
                    let a3894 = v3;
                    let a3895 = 65u64;
                    nat_sub(a3894, a3895)
                }, 1u64);
                let a3900 = f46(v1.clone(), 1u64, {
                    let a3897 = {
                        let a3896 = v1;
                        length_bytes(a3896)
                    };
                    let a3898 = 1u64;
                    nat_sub(a3897, a3898)
                });
                Ok(append_bytes(a3899, a3900))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f540(v0: Bytes) -> R<bool> {
    let m3903 = {
        let a3901 = v0;
        let a3902 = 0u64;
        index_bytes(a3901, a3902)
    };
    match m3903 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f155(f220(f60(v1)?), f221(f60(v1)?)))
        }
    }
}

pub fn f541(p3916: u64, v1: Bytes, p3917: u64, p3918: Bytes) -> R<Option<Bytes>> {
    let m3915 = (p3916, (p3917, p3918));
    {
        let mut m3915 = m3915;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3915;
                let m3904 = v0;
                if m3904 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3904 - 1;
                    let a3906 = {
                        let a3905 = v1.clone();
                        length_bytes(a3905)
                    };
                    let a3907 = v2;
                    if nat_le(a3906, a3907) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3908 = f325(v1.clone(), v2)?;
                        match m3908 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3910 = v2;
                                    let a3911 = {
                                        let (_, h3909) = v5;
                                        h3909
                                    };
                                    nat_add(a3910, a3911)?
                                }, {
                                    let a3913 = v3;
                                    let a3914 = f573({
                                        let (h3912, _) = v5;
                                        h3912
                                    });
                                    append_bytes(a3913, a3914)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3915 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f542() -> List<Bytes> {
    List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("termination_by")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("admit")), List::cons(f8(Str::lit("below")), List::cons(f8(Str::lit("brecOn")), List::cons(f8(Str::lit("casesOn")), List::cons(f8(Str::lit("ctorElim")), List::cons(f8(Str::lit("ctorElimType")), List::cons(f8(Str::lit("ctorIdx")), List::cons(f8(Str::lit("noConfusion")), List::cons(f8(Str::lit("noConfusionType")), List::cons(f8(Str::lit("rec")), List::cons(f8(Str::lit("recOn")), List::cons(f8(Str::lit("toCtorIdx")), List::<Bytes>::nil())))))))))))))))))))
}

pub fn f543(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h3919, _) = v1;
        h3919
    };
    let m3921 = {
        let (_, h3920) = v1;
        h3920
    };
    match m3921 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m3924 = {
                let a3922 = v0.clone();
                let a3923 = v2;
                index_bytes(a3922, a3923)
            };
            match m3924 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m3929 = {
                        let a3927 = v0;
                        let a3928 = {
                            let a3925 = v2;
                            let a3926 = 1u64;
                            nat_add(a3925, a3926)?
                        };
                        index_bytes(a3927, a3928)
                    };
                    match m3929 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f574(v4)? {
                                f574(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a3930 = v2;
                                    let a3931 = 2u64;
                                    nat_add(a3930, a3931)?
                                }, Some({
                                    let a3936 = v3;
                                    let a3937 = f185(f575(), {
                                        let a3934 = {
                                            let a3932 = f576(v4)?;
                                            let a3933 = 16u64;
                                            nat_mul(a3932, a3933)?
                                        };
                                        let a3935 = f576(v5)?;
                                        nat_add(a3934, a3935)?
                                    });
                                    append_bytes(a3936, a3937)
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

pub fn f544(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3938 = f216(v0, v1);
    let a3939 = f488(v0, v2);
    fixed_u32::bit_or(a3938, a3939)
}

pub fn f545(v0: u32) -> u32 {
    let a3942 = {
        let a3940 = f544(v0, 17u32, 15u32);
        let a3941 = f544(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a3940, a3941)
    };
    let a3943 = f216(v0, 10u32);
    fixed_u32::bit_xor(a3942, a3943)
}

pub fn f546(v0: u32) -> u32 {
    let a3946 = {
        let a3944 = f544(v0, 7u32, 25u32);
        let a3945 = f544(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a3944, a3945)
    };
    let a3947 = f216(v0, 3u32);
    fixed_u32::bit_xor(a3946, a3947)
}

pub fn f547(v0: u64) -> R<Bytes> {
    let v1: Bytes = f89(v0)?;
    let a3951 = f397({
        let a3949 = 9u64;
        let a3950 = {
            let a3948 = v1.clone();
            length_bytes(a3948)
        };
        nat_sub(a3949, a3950)
    });
    let a3952 = v1;
    Ok(append_bytes(a3951, a3952))
}

pub fn f548() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f549(v0: Fn21, p3956: List<List<(Bytes, Bytes)>>, p3957: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m3955 = (p3956, p3957);
    {
        let mut m3955 = m3955;
        loop {
            let step = {
                let (v1, v2) = m3955;
                let m3953 = v2;
                match m3953.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c3954: Fn21 = v0.clone();
                            c3954.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3955 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f550(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f551(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a3959 = {
        let a3958 = v0.clone();
        length_list(a3958)
    };
    let a3960 = 1u64;
    if nat_le(a3959, a3960) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f577(v0)?))
    }
}

pub fn f552(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f406({
        let (_, h3961) = {
            let (h3962, _) = f578(Fn22::F579, {
                let a3964 = {
                    let a3963 = v0.clone();
                    length_list(a3963)
                };
                let a3965 = 1u64;
                nat_add(a3964, a3965)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h3962
        };
        h3961
    }))
}

pub fn f553(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f580(v1, v0, v2)
}

pub fn f554(v0: u64, v1: Bytes, v2: u64, v3: u64) -> R<Option<u64>> {
    f581(v1, v2, v0, v3)
}

pub fn f555(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let a3968 = {
        let (h3966, _) = v1;
        h3966
    };
    let a3969 = {
        let a3967 = v0.clone();
        length_bytes(a3967)
    };
    if nat_lt(a3968, a3969) {
        Ok(Some((f245(v0, {
            let (h3970, _) = v1;
            h3970
        })?, {
            let a3972 = {
                let (_, h3971) = v1;
                h3971
            };
            let a3973 = 1u64;
            nat_add(a3972, a3973)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f556(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f582(v1, v0, v2)
}

pub fn f557(v0: Bytes, v1: u64) -> Bytes {
    f46(v0, v1, 1u64)
}

pub fn f558() -> Bytes {
    f8(Str::lit("\u{0}\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\u{9}\u{a}\u{b}\u{c}\u{d}\u{e}\u{f}\u{10}\u{11}\u{12}\u{13}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{1b}\u{1c}\u{1d}\u{1e}\u{1f} !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~\u{7f}"))
}

pub fn f559() -> Bytes {
    f8(Str::lit("\u{80}\u{c0}\u{100}\u{140}\u{180}\u{1c0}\u{200}\u{240}\u{280}\u{2c0}\u{300}\u{340}\u{380}\u{3c0}\u{400}\u{440}\u{480}\u{4c0}\u{500}\u{540}\u{580}\u{5c0}\u{600}\u{640}\u{680}\u{6c0}\u{700}\u{740}\u{780}\u{7c0}"))
}

pub fn f560(v0: u64) -> R<Bytes> {
    Ok(f557(f583(), {
        let a3979 = {
            let a3977 = {
                let a3974 = v0;
                let a3975 = 64u64;
                let a3976 = 0u64;
                nat_rem(a3974, a3975, a3976)
            };
            let a3978 = 2u64;
            nat_mul(a3977, a3978)?
        };
        let a3980 = 1u64;
        nat_add(a3979, a3980)?
    }))
}

pub fn f561() -> Bytes {
    f8(Str::lit("\u{800}\u{1000}\u{2000}\u{3000}\u{4000}\u{5000}\u{6000}\u{7000}\u{8000}\u{9000}\u{a000}\u{b000}\u{c000}\u{d000}\u{e000}\u{f000}"))
}

pub fn f562(v0: u64) -> Option<u64> {
    let a3981 = 48u64;
    let a3982 = v0;
    if if nat_le(a3981, a3982) {
        let a3983 = v0;
        let a3984 = 57u64;
        nat_le(a3983, a3984)
    } else {
        false
    } {
        Some({
            let a3985 = v0;
            let a3986 = 48u64;
            nat_sub(a3985, a3986)
        })
    } else {
        let a3987 = 97u64;
        let a3988 = v0;
        if if nat_le(a3987, a3988) {
            let a3989 = v0;
            let a3990 = 102u64;
            nat_le(a3989, a3990)
        } else {
            false
        } {
            Some({
                let a3991 = v0;
                let a3992 = 87u64;
                nat_sub(a3991, a3992)
            })
        } else {
            let a3993 = 65u64;
            let a3994 = v0;
            if if nat_le(a3993, a3994) {
                let a3995 = v0;
                let a3996 = 70u64;
                nat_le(a3995, a3996)
            } else {
                false
            } {
                Some({
                    let a3997 = v0;
                    let a3998 = 55u64;
                    nat_sub(a3997, a3998)
                })
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f563(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f584(v0, v1)
}

pub fn f564(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f585(v0, v1)
}

pub fn f565(v0: Bytes, v1: u64, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    let v3: u64 = {
        let (h3999, _) = v2;
        h3999
    };
    let v4: u64 = f182(v0.clone(), v3)?;
    let (_, h4000) = v2;
    if h4000 {
        Ok(None::<(u64, bool)>)
    } else {
        if f241(v4) {
            Ok(Some(({
                let a4001 = v3;
                let a4002 = 1u64;
                nat_add(a4001, a4002)?
            }, false)))
        } else {
            let a4003 = v4;
            let a4004 = 35u64;
            if if nat_eq(a4003, a4004) {
                let a4005 = v3;
                let a4006 = 0u64;
                if if nat_eq(a4005, a4006) {
                    true
                } else {
                    f241(f182(v0.clone(), {
                        let a4007 = v3;
                        let a4008 = 1u64;
                        nat_sub(a4007, a4008)
                    })?)
                } {
                    true
                } else {
                    let a4011 = f182(v0.clone(), {
                        let a4009 = v3;
                        let a4010 = 1u64;
                        nat_sub(a4009, a4010)
                    })?;
                    let a4012 = 10u64;
                    nat_eq(a4011, a4012)
                }
            } else {
                false
            } {
                Ok(Some((f330(v0, v3)?, false)))
            } else {
                let a4013 = v4;
                let a4014 = 10u64;
                if nat_eq(a4013, a4014) {
                    let v5: u64 = {
                        let a4015 = v3;
                        let a4016 = 1u64;
                        nat_add(a4015, a4016)?
                    };
                    let v6: u64 = f246(v0.clone(), v5)?;
                    let v7: u64 = f244(v0.clone(), v6)?;
                    if if f180(v0.clone(), v5)? {
                        true
                    } else {
                        let a4019 = {
                            let a4017 = v6;
                            let a4018 = v5;
                            nat_sub(a4017, a4018)
                        };
                        let a4020 = v1;
                        if if nat_lt(a4019, a4020) {
                            f184(f242(f182(v0.clone(), v7)?))
                        } else {
                            false
                        } {
                            f184({
                                let a4021 = f182(v0, v7)?;
                                let a4022 = 35u64;
                                nat_eq(a4021, a4022)
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

pub fn f566(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f244(v0.clone(), {
        let (h4023, _) = v1;
        h4023
    })?;
    let a4024 = f182(v0, v2)?;
    let a4025 = 10u64;
    if nat_eq(a4024, a4025) {
        Ok(Some(({
            let a4026 = v2;
            let a4027 = 1u64;
            nat_add(a4026, a4027)?
        }, {
            let a4029 = {
                let (_, h4028) = v1;
                h4028
            };
            let a4030 = 1u64;
            nat_add(a4029, a4030)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f567(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a4031 = v1;
        let a4032 = 3u64;
        nat_mul(a4031, a4032)?
    };
    let a4043 = {
        let a4039 = {
            let a4033 = f103(v0.clone(), v2)?;
            let a4034 = 16384u64;
            nat_mul(a4033, a4034)?
        };
        let a4040 = {
            let a4037 = f103(v0.clone(), {
                let a4035 = v2;
                let a4036 = 1u64;
                nat_add(a4035, a4036)?
            })?;
            let a4038 = 128u64;
            nat_mul(a4037, a4038)?
        };
        nat_add(a4039, a4040)?
    };
    let a4044 = f103(v0, {
        let a4041 = v2;
        let a4042 = 2u64;
        nat_add(a4041, a4042)?
    })?;
    nat_add(a4043, a4044)
}

pub fn f568(v0: u64, v1: u64) -> bool {
    let a4045 = v0;
    let a4046 = 0u64;
    if nat_eq(a4045, a4046) {
        f452(v1)
    } else {
        let a4047 = v0;
        let a4048 = 1u64;
        if nat_eq(a4047, a4048) {
            f451(v1)
        } else {
            let a4049 = v0;
            let a4050 = 2u64;
            if nat_eq(a4049, a4050) {
                f586(v1)
            } else {
                f587(v1)
            }
        }
    }
}

pub fn f569() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f570(v0: bool) -> Adt15 {
    let v1: Bytes = if v0 {
        f140()
    } else {
        f138()
    };
    Adt15::C0(0u64, f7(List::cons(f8(Str::lit("(List ")), List::cons(v1.clone(), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))), List::cons(v1, List::<Bytes>::nil()), Bytes::lit(&[]), Bytes::lit(&[]))
}

pub fn f571(v0: Bytes, v1: u64) -> R<u64> {
    let a4052 = v1;
    let a4053 = {
        let a4051 = v0.clone();
        length_bytes(a4051)
    };
    if nat_lt(a4052, a4053) {
        let a4054 = f323(v0, v1)?;
        let a4055 = 1u64;
        nat_add(a4054, a4055)
    } else {
        Ok(0u64)
    }
}

pub fn f572(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f155(f105(f155(f220(v2), f221(v2)), f105({
        let a4056 = v2;
        let a4057 = 117u64;
        !nat_eq(a4056, a4057)
    }, {
        let a4058 = v2;
        let a4059 = 85u64;
        !nat_eq(a4058, a4059)
    })), f105(f222(v2), {
        let a4060 = 0u64;
        let a4061 = v1;
        nat_lt(a4060, a4061)
    })) {
        f46(v0, v1, 1u64)
    } else {
        let a4062 = v2;
        let a4063 = 65536u64;
        if nat_lt(a4062, a4063) {
            let a4064 = f8(Str::lit("u"));
            let a4065 = f471(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a4064, a4065)
        } else {
            let a4066 = f8(Str::lit("U"));
            let a4067 = f471(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a4066, a4067)
        }
    }
}

pub fn f573(v0: u64) -> Bytes {
    let a4068 = f8(Str::lit("u"));
    let a4069 = f471(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a4068, a4069)
}

pub fn f574(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4114 = {
            let a4090 = {
                let a4078 = {
                    let a4072 = {
                        let a4070 = v1;
                        let a4071 = 128u8;
                        fixed_u8::bit_and(a4070, a4071)
                    };
                    let a4073 = 128u8;
                    if equal(a4072, a4073) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4079 = {
                    let a4076 = {
                        let a4074 = v1;
                        let a4075 = 64u8;
                        fixed_u8::bit_and(a4074, a4075)
                    };
                    let a4077 = 64u8;
                    if equal(a4076, a4077) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4078, a4079)?
            };
            let a4091 = {
                let a4088 = {
                    let a4082 = {
                        let a4080 = v1;
                        let a4081 = 32u8;
                        fixed_u8::bit_and(a4080, a4081)
                    };
                    let a4083 = 32u8;
                    if equal(a4082, a4083) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4089 = {
                    let a4086 = {
                        let a4084 = v1;
                        let a4085 = 16u8;
                        fixed_u8::bit_and(a4084, a4085)
                    };
                    let a4087 = 16u8;
                    if equal(a4086, a4087) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4088, a4089)?
            };
            nat_add(a4090, a4091)?
        };
        let a4115 = {
            let a4112 = {
                let a4100 = {
                    let a4094 = {
                        let a4092 = v1;
                        let a4093 = 8u8;
                        fixed_u8::bit_and(a4092, a4093)
                    };
                    let a4095 = 8u8;
                    if equal(a4094, a4095) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4101 = {
                    let a4098 = {
                        let a4096 = v1;
                        let a4097 = 4u8;
                        fixed_u8::bit_and(a4096, a4097)
                    };
                    let a4099 = 4u8;
                    if equal(a4098, a4099) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4100, a4101)?
            };
            let a4113 = {
                let a4110 = {
                    let a4104 = {
                        let a4102 = v1;
                        let a4103 = 2u8;
                        fixed_u8::bit_and(a4102, a4103)
                    };
                    let a4105 = 2u8;
                    if equal(a4104, a4105) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4111 = {
                    let a4108 = {
                        let a4106 = v1;
                        let a4107 = 1u8;
                        fixed_u8::bit_and(a4106, a4107)
                    };
                    let a4109 = 1u8;
                    if equal(a4108, a4109) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4110, a4111)?
            };
            nat_add(a4112, a4113)?
        };
        nat_add(a4114, a4115)?
    };
    let a4116 = 48u64;
    let a4117 = v2;
    if if nat_le(a4116, a4117) {
        let a4118 = v2;
        let a4119 = 57u64;
        nat_le(a4118, a4119)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a4120 = 97u64;
        let a4121 = v2;
        if nat_le(a4120, a4121) {
            let a4122 = v2;
            let a4123 = 102u64;
            Ok(nat_le(a4122, a4123))
        } else {
            Ok(false)
        }
    }
}

pub fn f575() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f576(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4168 = {
            let a4144 = {
                let a4132 = {
                    let a4126 = {
                        let a4124 = v1;
                        let a4125 = 128u8;
                        fixed_u8::bit_and(a4124, a4125)
                    };
                    let a4127 = 128u8;
                    if equal(a4126, a4127) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4133 = {
                    let a4130 = {
                        let a4128 = v1;
                        let a4129 = 64u8;
                        fixed_u8::bit_and(a4128, a4129)
                    };
                    let a4131 = 64u8;
                    if equal(a4130, a4131) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4132, a4133)?
            };
            let a4145 = {
                let a4142 = {
                    let a4136 = {
                        let a4134 = v1;
                        let a4135 = 32u8;
                        fixed_u8::bit_and(a4134, a4135)
                    };
                    let a4137 = 32u8;
                    if equal(a4136, a4137) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4143 = {
                    let a4140 = {
                        let a4138 = v1;
                        let a4139 = 16u8;
                        fixed_u8::bit_and(a4138, a4139)
                    };
                    let a4141 = 16u8;
                    if equal(a4140, a4141) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4142, a4143)?
            };
            nat_add(a4144, a4145)?
        };
        let a4169 = {
            let a4166 = {
                let a4154 = {
                    let a4148 = {
                        let a4146 = v1;
                        let a4147 = 8u8;
                        fixed_u8::bit_and(a4146, a4147)
                    };
                    let a4149 = 8u8;
                    if equal(a4148, a4149) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4155 = {
                    let a4152 = {
                        let a4150 = v1;
                        let a4151 = 4u8;
                        fixed_u8::bit_and(a4150, a4151)
                    };
                    let a4153 = 4u8;
                    if equal(a4152, a4153) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4154, a4155)?
            };
            let a4167 = {
                let a4164 = {
                    let a4158 = {
                        let a4156 = v1;
                        let a4157 = 2u8;
                        fixed_u8::bit_and(a4156, a4157)
                    };
                    let a4159 = 2u8;
                    if equal(a4158, a4159) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4165 = {
                    let a4162 = {
                        let a4160 = v1;
                        let a4161 = 1u8;
                        fixed_u8::bit_and(a4160, a4161)
                    };
                    let a4163 = 1u8;
                    if equal(a4162, a4163) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4164, a4165)?
            };
            nat_add(a4166, a4167)?
        };
        nat_add(a4168, a4169)?
    };
    let a4170 = v2;
    let a4171 = 57u64;
    if nat_le(a4170, a4171) {
        let a4172 = v2;
        let a4173 = 48u64;
        Ok(nat_sub(a4172, a4173))
    } else {
        let a4174 = v2;
        let a4175 = 87u64;
        Ok(nat_sub(a4174, a4175))
    }
}

pub fn f577(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f494({
        let (_, h4176) = {
            let (h4177, _) = f588(Fn23::F589, {
                let a4179 = {
                    let a4178 = v0.clone();
                    length_list(a4178)
                };
                let a4180 = 1u64;
                nat_add(a4179, a4180)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h4177
        };
        h4176
    }))
}

pub fn f578(v0: Fn22, p4185: u64, p4186: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m4184 = (p4185, p4186);
    {
        let mut m4184 = m4184;
        loop {
            let step = {
                let (v1, v2) = m4184;
                let m4181 = v1;
                if m4181 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4181 - 1;
                    let m4183 = {
                        let c4182: Fn22 = v0.clone();
                        c4182.apply(v2.clone())?
                    };
                    match m4183 {
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
                Ok(next) => { m4184 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f579(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f590(v0)
}

pub fn f580(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f508(v0.clone(), {
        let (h4187, _) = v2;
        h4187
    }, v1)?;
    let a4188 = f182(v0, v3)?;
    let a4189 = 10u64;
    if nat_eq(a4188, a4189) {
        Ok(Some(({
            let a4190 = v3;
            let a4191 = 1u64;
            nat_add(a4190, a4191)?
        }, {
            let a4193 = {
                let (_, h4192) = v2;
                h4192
            };
            let a4194 = 1u64;
            nat_add(a4193, a4194)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f581(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a4195 = f182(v0, v3)?;
    let a4196 = 32u64;
    if if nat_eq(a4195, a4196) {
        let a4199 = {
            let a4197 = v3;
            let a4198 = v1;
            nat_sub(a4197, a4198)
        };
        let a4200 = v2;
        nat_lt(a4199, a4200)
    } else {
        false
    } {
        Ok(Some({
            let a4201 = v3;
            let a4202 = 1u64;
            nat_add(a4201, a4202)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f582(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m4204 = f562(f182(v0, {
        let (h4203, _) = v2;
        h4203
    })?);
    match m4204 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v3) => {
            let a4205 = v3;
            let a4206 = v1;
            if nat_lt(a4205, a4206) {
                Ok(Some(({
                    let a4208 = {
                        let (h4207, _) = v2;
                        h4207
                    };
                    let a4209 = 1u64;
                    nat_add(a4208, a4209)?
                }, {
                    let a4213 = {
                        let a4211 = {
                            let (_, h4210) = v2;
                            h4210
                        };
                        let a4212 = v1;
                        nat_mul(a4211, a4212)?
                    };
                    let a4214 = v3;
                    nat_add(a4213, a4214)?
                })))
            } else {
                Ok(None::<(u64, u64)>)
            }
        }
    }
}

pub fn f583() -> Bytes {
    f8(Str::lit("\u{80}\u{81}\u{82}\u{83}\u{84}\u{85}\u{86}\u{87}\u{88}\u{89}\u{8a}\u{8b}\u{8c}\u{8d}\u{8e}\u{8f}\u{90}\u{91}\u{92}\u{93}\u{94}\u{95}\u{96}\u{97}\u{98}\u{99}\u{9a}\u{9b}\u{9c}\u{9d}\u{9e}\u{9f}\u{a0}\u{a1}\u{a2}\u{a3}\u{a4}\u{a5}\u{a6}\u{a7}\u{a8}\u{a9}\u{aa}\u{ab}\u{ac}\u{ad}\u{ae}\u{af}\u{b0}\u{b1}\u{b2}\u{b3}\u{b4}\u{b5}\u{b6}\u{b7}\u{b8}\u{b9}\u{ba}\u{bb}\u{bc}\u{bd}\u{be}\u{bf}"))
}

pub fn f584(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f182(v0, v1)?;
    let a4215 = v2;
    let a4216 = 34u64;
    if if if if nat_eq(a4215, a4216) {
        true
    } else {
        let a4217 = v2;
        let a4218 = 92u64;
        nat_eq(a4217, a4218)
    } {
        true
    } else {
        f241(v2)
    } {
        true
    } else {
        f242(v2)
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a4219 = v1;
            let a4220 = 1u64;
            nat_add(a4219, a4220)?
        }))
    }
}

pub fn f585(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f182(v0, v1)?;
    let a4221 = v2;
    let a4222 = 39u64;
    if if if nat_eq(a4221, a4222) {
        true
    } else {
        f241(v2)
    } {
        true
    } else {
        f242(v2)
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a4223 = v1;
            let a4224 = 1u64;
            nat_add(a4223, a4224)?
        }))
    }
}

pub fn f586(v0: u64) -> bool {
    let a4225 = 48u64;
    let a4226 = v0;
    if nat_le(a4225, a4226) {
        let a4227 = v0;
        let a4228 = 55u64;
        nat_le(a4227, a4228)
    } else {
        false
    }
}

pub fn f587(v0: u64) -> bool {
    if f348(v0) {
        true
    } else {
        if f452(v0) {
            true
        } else {
            let a4229 = v0;
            let a4230 = 95u64;
            if nat_eq(a4229, a4230) {
                true
            } else {
                let a4231 = v0;
                let a4232 = 45u64;
                nat_eq(a4231, a4232)
            }
        }
    }
}

pub fn f588(v0: Fn23, p4237: u64, p4238: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m4236 = (p4237, p4238);
    {
        let mut m4236 = m4236;
        loop {
            let step = {
                let (v1, v2) = m4236;
                let m4233 = v1;
                if m4233 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4233 - 1;
                    let m4235 = {
                        let c4234: Fn23 = v0.clone();
                        c4234.apply(v2.clone())?
                    };
                    match m4235 {
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
                Ok(next) => { m4236 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f589(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f591(v0)
}

pub fn f590(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h4239, _) = v0;
        h4239
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h4240) = v0;
        h4240
    };
    let m4241 = v1;
    match m4241.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m4242 = v4;
            match m4242.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f592(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f591(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h4243, _) = v0;
        h4243
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h4244) = v0;
        h4244
    };
    let m4245 = v1;
    match m4245.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m4246 = v4;
            match m4246.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f593(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f592(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a4251 = {
            let a4249 = {
                let a4247 = v0.clone();
                length_list(a4247)
            };
            let a4250 = {
                let a4248 = v1.clone();
                length_list(a4248)
            };
            nat_add(a4249, a4250)?
        };
        let a4252 = 1u64;
        nat_add(a4251, a4252)?
    };
    Ok(f411({
        let (_, h4253) = {
            let (_, h4254) = {
                let (h4255, _) = f594(Fn24::F595, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h4255
            };
            h4254
        };
        h4253
    }))
}

pub fn f593(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a4260 = {
            let a4258 = {
                let a4256 = v0.clone();
                length_list(a4256)
            };
            let a4259 = {
                let a4257 = v1.clone();
                length_list(a4257)
            };
            nat_add(a4258, a4259)?
        };
        let a4261 = 1u64;
        nat_add(a4260, a4261)?
    };
    Ok(f596({
        let (_, h4262) = {
            let (_, h4263) = {
                let (h4264, _) = f597(Fn25::F598, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h4264
            };
            h4263
        };
        h4262
    }))
}

pub fn f594(v0: Fn24, p4269: u64, p4270: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m4268 = (p4269, p4270);
    {
        let mut m4268 = m4268;
        loop {
            let step = {
                let (v1, v2) = m4268;
                let m4265 = v1;
                if m4265 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m4265 - 1;
                    let m4267 = {
                        let c4266: Fn24 = v0.clone();
                        c4266.apply(v2.clone())
                    };
                    match m4267 {
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
                Ok(next) => { m4268 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f595(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f599(v0)
}

pub fn f596(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f311(Fn8::F600, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f597(v0: Fn25, p4275: u64, p4276: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m4274 = (p4275, p4276);
    {
        let mut m4274 = m4274;
        loop {
            let step = {
                let (v1, v2) = m4274;
                let m4271 = v1;
                if m4271 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m4271 - 1;
                    let m4273 = {
                        let c4272: Fn25 = v0.clone();
                        c4272.apply(v2.clone())
                    };
                    match m4273 {
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
                Ok(next) => { m4274 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f598(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f601(v0)
}

pub fn f599(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h4277, _) = v0.clone();
        h4277
    };
    let v2: List<(Bytes, ())> = {
        let (h4278, _) = {
            let (_, h4279) = v0.clone();
            h4279
        };
        h4278
    };
    let v3: List<(Bytes, ())> = {
        let (_, h4280) = {
            let (_, h4281) = v0;
            h4281
        };
        h4280
    };
    let m4282 = v1.clone();
    match m4282.uncons() {
        None => {
            let m4283 = v2;
            match m4283.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m4284 = v2.clone();
            match m4284.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f602({
                        let (h4285, _) = v8.clone();
                        h4285
                    }, {
                        let (h4286, _) = v6.clone();
                        h4286
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

pub fn f600(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f601(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h4287, _) = v0.clone();
        h4287
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h4288, _) = {
            let (_, h4289) = v0.clone();
            h4289
        };
        h4288
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h4290) = {
            let (_, h4291) = v0;
            h4291
        };
        h4290
    };
    let m4292 = v1.clone();
    match m4292.uncons() {
        None => {
            let m4293 = v2;
            match m4293.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m4294 = v2.clone();
            match m4294.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f602({
                        let (h4295, _) = v8.clone();
                        h4295
                    }, {
                        let (h4296, _) = v6.clone();
                        h4296
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

pub fn f602(v0: Bytes, v1: Bytes) -> bool {
    let a4301 = {
        let a4297 = v0;
        let a4298 = v1;
        compare_bytes(a4297, a4298)
    };
    let a4302 = {
        let a4299 = Bytes::lit(&[]);
        let a4300 = Bytes::lit(&[0x01]);
        compare_bytes(a4299, a4300)
    };
    equal(a4301, a4302)
}

pub fn run(p0: Option<Adt0>, p1: Adt3) -> R<(Adt0, Adt10)> {
    f0(p0, p1)
}
