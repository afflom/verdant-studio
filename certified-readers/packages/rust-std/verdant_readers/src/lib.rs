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
    C0(Adt2, u64),
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
    C0(Bytes, Bytes, Adt17, Adt17, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Adt26 {
    C0(Bytes, List<Bytes>, Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt27 {
    C0(Bytes, Adt17, Adt17, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Adt28 {
    C0(Bytes, List<Adt21>),
}

#[derive(Clone)]
pub enum Adt29 {
    C0(Adt15, List<Adt21>),
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
    F57(u8, Bytes),
    F268(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F57(k0, k1) => f57(*k0, k1.clone(), p0),
            Fn0::F268(k0) => f268(k0.clone(), p0),
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
    F217(Bytes),
    F218(Bytes),
    F252(Bytes, Bytes),
    F253(Bytes),
    F255(Bytes, u64),
    F318(Bytes),
    F362(bool, u64, Bytes),
    F392(Bytes),
    F394(Bytes),
    F403(Bytes, u64),
    F413(Bytes),
    F416(u64, Bytes),
    F474(Bytes),
    F476(Bytes),
    F587(u64, Bytes, u64),
    F590(Bytes),
    F591(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn4::F169(k0) => f169(k0.clone(), p0),
            Fn4::F208(k0) => f208(k0.clone(), p0),
            Fn4::F209(k0) => f209(k0.clone(), p0),
            Fn4::F217(k0) => f217(k0.clone(), p0),
            Fn4::F218(k0) => f218(k0.clone(), p0),
            Fn4::F252(k0, k1) => f252(k0.clone(), k1.clone(), p0),
            Fn4::F253(k0) => f253(k0.clone(), p0),
            Fn4::F255(k0, k1) => f255(k0.clone(), *k1, p0),
            Fn4::F318(k0) => f318(k0.clone(), p0),
            Fn4::F362(k0, k1, k2) => f362(*k0, *k1, k2.clone(), p0),
            Fn4::F392(k0) => f392(k0.clone(), p0),
            Fn4::F394(k0) => f394(k0.clone(), p0),
            Fn4::F403(k0, k1) => f403(k0.clone(), *k1, p0),
            Fn4::F413(k0) => f413(k0.clone(), p0),
            Fn4::F416(k0, k1) => f416(*k0, k1.clone(), p0),
            Fn4::F474(k0) => f474(k0.clone(), p0),
            Fn4::F476(k0) => f476(k0.clone(), p0),
            Fn4::F587(k0, k1, k2) => f587(*k0, k1.clone(), *k2, p0),
            Fn4::F590(k0) => f590(k0.clone(), p0),
            Fn4::F591(k0) => f591(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F221(Bytes),
    F448(bool, Bytes),
    F482(Bytes),
    F491(bool, Bytes),
    F550(Bytes),
    F560(Bytes),
    F586(u64, Bytes),
    F594(u64, Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn5::F221(k0) => f221(k0.clone(), p0),
            Fn5::F448(k0, k1) => f448(*k0, k1.clone(), p0),
            Fn5::F482(k0) => f482(k0.clone(), p0),
            Fn5::F491(k0, k1) => f491(*k0, k1.clone(), p0),
            Fn5::F550(k0) => f550(k0.clone(), p0),
            Fn5::F560(k0) => f560(k0.clone(), p0),
            Fn5::F586(k0, k1) => f586(*k0, k1.clone(), p0),
            Fn5::F594(k0, k1) => f594(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F407,
    F627,
}

impl Fn6 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn6::F407 => f407(p0, p1),
            Fn6::F627 => f627(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F410,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn7::F410 => f410(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F504(Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn8::F504(k0) => f504(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F558(u64, Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, bool)) -> R<Option<(u64, bool)>> {
        match self {
            Fn9::F558(k0, k1) => f558(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F565,
}

impl Fn10 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn10::F565 => f565(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F567,
}

impl Fn11 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn11::F567 => f567(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F597,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn12::F597 => f597(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F619,
}

impl Fn13 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn13::F619 => f619(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F626,
}

impl Fn14 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn14::F626 => f626(p0),
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
    List::cons(f8(Str::lit("admit JSONL")), List::cons(f8(Str::lit("admit-encoding STAGED")), List::cons(f8(Str::lit("agenda")), List::cons(f8(Str::lit("build")), List::cons(f8(Str::lit("check")), List::cons(f8(Str::lit("ingest-authority BINDING")), List::cons(f8(Str::lit("ingest-index CSV")), List::cons(f8(Str::lit("merge-gate RULES")), List::cons(f8(Str::lit("next [SKIP...]")), List::cons(f8(Str::lit("propose-encoding SCHEMA NAME CITATION...")), List::cons(f8(Str::lit("publish")), List::cons(f8(Str::lit("read-webidl FILE [IMPORT...]")), List::cons(f8(Str::lit("read-xml FILE")), List::cons(f8(Str::lit("read-xsd FILE")), List::cons(f8(Str::lit("read-xsd-components FILE")), List::cons(f8(Str::lit("read-yaml FILE")), List::cons(f8(Str::lit("record-decision JSONL")), List::cons(f8(Str::lit("record-error REFUSAL TIME RUN OPERATION...")), List::cons(f8(Str::lit("stage-admissions JSONL")), List::cons(f8(Str::lit("stage-bindings JSONL")), List::cons(f8(Str::lit("stage-decisions JSONL")), List::cons(f8(Str::lit("stage-encodings JSONL")), List::cons(f8(Str::lit("stage-engine")), List::cons(f8(Str::lit("stage-mappings JSONL")), List::cons(f8(Str::lit("stage-matrix JSONL")), List::<Bytes>::nil())))))))))))))))))))))))))
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
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f73(v5)?))
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
    let a305 = {
        let a281 = {
            let a269 = {
                let a263 = {
                    let a261 = v1;
                    let a262 = 128u8;
                    fixed_u8::bit_and(a261, a262)
                };
                let a264 = 128u8;
                if equal(a263, a264) {
                    128u64
                } else {
                    0u64
                }
            };
            let a270 = {
                let a267 = {
                    let a265 = v1;
                    let a266 = 64u8;
                    fixed_u8::bit_and(a265, a266)
                };
                let a268 = 64u8;
                if equal(a267, a268) {
                    64u64
                } else {
                    0u64
                }
            };
            nat_add(a269, a270)?
        };
        let a282 = {
            let a279 = {
                let a273 = {
                    let a271 = v1;
                    let a272 = 32u8;
                    fixed_u8::bit_and(a271, a272)
                };
                let a274 = 32u8;
                if equal(a273, a274) {
                    32u64
                } else {
                    0u64
                }
            };
            let a280 = {
                let a277 = {
                    let a275 = v1;
                    let a276 = 16u8;
                    fixed_u8::bit_and(a275, a276)
                };
                let a278 = 16u8;
                if equal(a277, a278) {
                    16u64
                } else {
                    0u64
                }
            };
            nat_add(a279, a280)?
        };
        nat_add(a281, a282)?
    };
    let a306 = {
        let a303 = {
            let a291 = {
                let a285 = {
                    let a283 = v1;
                    let a284 = 8u8;
                    fixed_u8::bit_and(a283, a284)
                };
                let a286 = 8u8;
                if equal(a285, a286) {
                    8u64
                } else {
                    0u64
                }
            };
            let a292 = {
                let a289 = {
                    let a287 = v1;
                    let a288 = 4u8;
                    fixed_u8::bit_and(a287, a288)
                };
                let a290 = 4u8;
                if equal(a289, a290) {
                    4u64
                } else {
                    0u64
                }
            };
            nat_add(a291, a292)?
        };
        let a304 = {
            let a301 = {
                let a295 = {
                    let a293 = v1;
                    let a294 = 2u8;
                    fixed_u8::bit_and(a293, a294)
                };
                let a296 = 2u8;
                if equal(a295, a296) {
                    2u64
                } else {
                    0u64
                }
            };
            let a302 = {
                let a299 = {
                    let a297 = v1;
                    let a298 = 1u8;
                    fixed_u8::bit_and(a297, a298)
                };
                let a300 = 1u8;
                if equal(a299, a300) {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a301, a302)?
        };
        nat_add(a303, a304)?
    };
    nat_add(a305, a306)
}

pub fn f53(v0: u64) -> List<Str> {
    let a307 = v0;
    let a308 = 0u64;
    if nat_eq(a307, a308) {
        f74()
    } else {
        let a309 = v0;
        let a310 = 1u64;
        if nat_eq(a309, a310) {
            f75()
        } else {
            let a311 = v0;
            let a312 = 2u64;
            if nat_eq(a311, a312) {
                f76()
            } else {
                let a313 = v0;
                let a314 = 3u64;
                if nat_eq(a313, a314) {
                    f77()
                } else {
                    let a315 = v0;
                    let a316 = 4u64;
                    if nat_eq(a315, a316) {
                        f78()
                    } else {
                        let a317 = v0;
                        let a318 = 5u64;
                        if nat_eq(a317, a318) {
                            f79()
                        } else {
                            let a319 = v0;
                            let a320 = 6u64;
                            if nat_eq(a319, a320) {
                                f80()
                            } else {
                                let a321 = v0;
                                let a322 = 7u64;
                                if nat_eq(a321, a322) {
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
        let a323 = v0;
        length_bytes(a323)
    }, false, false, 0u64, Bytes::lit(&[]))
}

pub fn f55(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt2, u64)>> {
    let m324 = v2;
    if m324 == 0 {
        Ok(None::<(Adt2, u64)>)
    } else {
        let v4 = m324 - 1;
        let v5: u64 = f83(v0.clone(), v1)?;
        let a325 = v3;
        let a326 = 1u64;
        if nat_eq(a325, a326) {
            let m327 = f55(v0.clone(), v1, v4, 0u64)?;
            match m327 {
                None => {
                    Ok(None::<(Adt2, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f83(v0.clone(), {
                        let (_, h328) = v6.clone();
                        h328
                    })?;
                    let a329 = v7;
                    let a330 = 44u64;
                    if nat_eq(a329, a330) {
                        let m334 = f55(v0, {
                            let a332 = {
                                let (_, h331) = v6;
                                h331
                            };
                            let a333 = 1u64;
                            nat_add(a332, a333)?
                        }, v4, 1u64)?;
                        match m334 {
                            None => {
                                Ok(None::<(Adt2, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt2::C9(std::rc::Rc::new({
                                    let (h335, _) = v6;
                                    h335
                                }), std::rc::Rc::new({
                                    let (h336, _) = v8;
                                    h336
                                })), {
                                    let (_, h337) = v8;
                                    h337
                                })))
                            }
                        }
                    } else {
                        let a338 = v7;
                        let a339 = 93u64;
                        if nat_eq(a338, a339) {
                            Ok(Some((Adt2::C9(std::rc::Rc::new({
                                let (h340, _) = v6;
                                h340
                            }), std::rc::Rc::new(Adt2::C8)), {
                                let a342 = {
                                    let (_, h341) = v6;
                                    h341
                                };
                                let a343 = 1u64;
                                nat_add(a342, a343)?
                            })))
                        } else {
                            Ok(None::<(Adt2, u64)>)
                        }
                    }
                }
            }
        } else {
            let a344 = v3;
            let a345 = 2u64;
            if nat_eq(a344, a345) {
                let m348 = f84(v0.clone(), {
                    let a346 = v1;
                    let a347 = 1u64;
                    nat_add(a346, a347)?
                })?;
                match m348 {
                    None => {
                        Ok(None::<(Adt2, u64)>)
                    }
                    Some(v9) => {
                        if f85({
                            let a349 = v5;
                            let a350 = 34u64;
                            nat_eq(a349, a350)
                        }, {
                            let a352 = f83(v0.clone(), {
                                let (h351, _) = v9.clone();
                                h351
                            })?;
                            let a353 = 58u64;
                            nat_eq(a352, a353)
                        }) {
                            let m357 = f55(v0.clone(), {
                                let a355 = {
                                    let (h354, _) = v9;
                                    h354
                                };
                                let a356 = 1u64;
                                nat_add(a355, a356)?
                            }, v4, 0u64)?;
                            match m357 {
                                None => {
                                    Ok(None::<(Adt2, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f83(v0.clone(), {
                                        let (_, h358) = v10.clone();
                                        h358
                                    })?;
                                    let a359 = v11;
                                    let a360 = 44u64;
                                    if nat_eq(a359, a360) {
                                        let m364 = f55(v0, {
                                            let a362 = {
                                                let (_, h361) = v10;
                                                h361
                                            };
                                            let a363 = 1u64;
                                            nat_add(a362, a363)?
                                        }, v4, 2u64)?;
                                        match m364 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt2::C10({
                                                    let (_, h365) = v9;
                                                    h365
                                                }, std::rc::Rc::new({
                                                    let (h366, _) = v10;
                                                    h366
                                                }), std::rc::Rc::new({
                                                    let (h367, _) = v12;
                                                    h367
                                                })), {
                                                    let (_, h368) = v12;
                                                    h368
                                                })))
                                            }
                                        }
                                    } else {
                                        let a369 = v11;
                                        let a370 = 125u64;
                                        if nat_eq(a369, a370) {
                                            Ok(Some((Adt2::C10({
                                                let (_, h371) = v9;
                                                h371
                                            }, std::rc::Rc::new({
                                                let (h372, _) = v10;
                                                h372
                                            }), std::rc::Rc::new(Adt2::C8)), {
                                                let a374 = {
                                                    let (_, h373) = v10;
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
                            Ok(None::<(Adt2, u64)>)
                        }
                    }
                }
            } else {
                let a376 = v5;
                let a377 = 34u64;
                if nat_eq(a376, a377) {
                    let m380 = f84(v0, {
                        let a378 = v1;
                        let a379 = 1u64;
                        nat_add(a378, a379)?
                    })?;
                    match m380 {
                        None => {
                            Ok(None::<(Adt2, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt2::C5({
                                let (_, h381) = v13;
                                h381
                            }), {
                                let (h382, _) = v13;
                                h382
                            })))
                        }
                    }
                } else {
                    if f46(f43(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                        Ok(Some((Adt2::C0, {
                            let a383 = v1;
                            let a384 = 4u64;
                            nat_add(a383, a384)?
                        })))
                    } else {
                        if f46(f43(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                            Ok(Some((Adt2::C2(true), {
                                let a385 = v1;
                                let a386 = 4u64;
                                nat_add(a385, a386)?
                            })))
                        } else {
                            if f46(f43(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                                Ok(Some((Adt2::C2(false), {
                                    let a387 = v1;
                                    let a388 = 5u64;
                                    nat_add(a387, a388)?
                                })))
                            } else {
                                let a389 = v5;
                                let a390 = 91u64;
                                if nat_eq(a389, a390) {
                                    let a393 = f83(v0.clone(), {
                                        let a391 = v1;
                                        let a392 = 1u64;
                                        nat_add(a391, a392)?
                                    })?;
                                    let a394 = 93u64;
                                    if nat_eq(a393, a394) {
                                        Ok(Some((Adt2::C6(std::rc::Rc::new(Adt2::C8)), {
                                            let a395 = v1;
                                            let a396 = 2u64;
                                            nat_add(a395, a396)?
                                        })))
                                    } else {
                                        let m399 = f55(v0, {
                                            let a397 = v1;
                                            let a398 = 1u64;
                                            nat_add(a397, a398)?
                                        }, v4, 1u64)?;
                                        match m399 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v14) => {
                                                Ok(Some((Adt2::C6(std::rc::Rc::new({
                                                    let (h400, _) = v14;
                                                    h400
                                                })), {
                                                    let (_, h401) = v14;
                                                    h401
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let a402 = v5;
                                    let a403 = 123u64;
                                    if nat_eq(a402, a403) {
                                        let a406 = f83(v0.clone(), {
                                            let a404 = v1;
                                            let a405 = 1u64;
                                            nat_add(a404, a405)?
                                        })?;
                                        let a407 = 125u64;
                                        if nat_eq(a406, a407) {
                                            Ok(Some((Adt2::C7(std::rc::Rc::new(Adt2::C8)), {
                                                let a408 = v1;
                                                let a409 = 2u64;
                                                nat_add(a408, a409)?
                                            })))
                                        } else {
                                            let m412 = f55(v0, {
                                                let a410 = v1;
                                                let a411 = 1u64;
                                                nat_add(a410, a411)?
                                            }, v4, 2u64)?;
                                            match m412 {
                                                None => {
                                                    Ok(None::<(Adt2, u64)>)
                                                }
                                                Some(v15) => {
                                                    Ok(Some((Adt2::C7(std::rc::Rc::new({
                                                        let (h413, _) = v15;
                                                        h413
                                                    })), {
                                                        let (_, h414) = v15;
                                                        h414
                                                    })))
                                                }
                                            }
                                        }
                                    } else {
                                        let m415 = f86(v0, v1)?;
                                        match m415 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v16) => {
                                                Ok(Some(({
                                                    let (_, h416) = v16;
                                                    h416
                                                }, {
                                                    let (h417, _) = v16;
                                                    h417
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

pub fn f56(v0: Fn0, p422: u64, p423: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m421 = (p422, p423);
    {
        let mut m421 = m421;
        loop {
            let step = {
                let (v1, v2) = m421;
                let m418 = v1;
                if m418 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m418 - 1;
                    let m420 = {
                        let c419: Fn0 = v0.clone();
                        c419.apply(v2.clone())?
                    };
                    match m420 {
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
                Ok(next) => { m421 = next; }
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

pub fn f59(v0: Fn1, p427: Option<Bytes>, p428: List<Bytes>) -> Option<Bytes> {
    let m426 = (p427, p428);
    {
        let mut m426 = m426;
        loop {
            let step = {
                let (v1, v2) = m426;
                let m424 = v2;
                match m424.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c425: Fn1 = v0.clone();
                            c425.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m426 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f60(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m429 = v1;
    match m429 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a432 = {
                    let a430 = v3;
                    let a431 = v0;
                    append_bytes(a430, a431)
                };
                let a433 = v2;
                append_bytes(a432, a433)
            })
        }
    }
}

pub fn f61(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m434 = v1;
    match m434.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m435 = f90(f91(v0.clone(), v2.clone()), v2.clone());
            match m435 {
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
    let m436 = f92(v0)?;
    match m436 {
        Ok(v3) => {
            let m437 = f93(v1)?;
            match m437 {
                Ok(v5) => {
                    let v6: List<Adt13> = f94(v3.clone(), v5);
                    let v7: List<Adt16> = f95({
                        let a439 = {
                            let a438 = v6.clone();
                            length_list(a438)
                        };
                        let a440 = 1u64;
                        nat_add(a439, a440)?
                    }, f96(v6.clone(), v6.clone())?)?;
                    let v8: List<Bytes> = f97({
                        let a442 = {
                            let a441 = v7.clone();
                            length_list(a441)
                        };
                        let a443 = 1u64;
                        nat_add(a442, a443)?
                    }, f98(v3.clone()), List::<Bytes>::nil(), v7.clone());
                    let v9: List<(Bytes, (List<Bytes>, Bytes))> = f99(v7.clone(), v8);
                    let v10: (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>) = f100({
                        let a445 = {
                            let a444 = v9.clone();
                            length_list(a444)
                        };
                        let a446 = 1u64;
                        nat_add(a445, a446)?
                    }, v9, List::<Bytes>::nil(), List::<Bytes>::nil());
                    Ok(Ok::<(u64, Bytes), Bytes>(({
                        let a448 = {
                            let (h447, _) = v10.clone();
                            h447
                        };
                        length_list(a448)
                    }, f7(List::cons(f7(f101(v3, v7, v6)), List::cons(f7(f102({
                        let (_, h449) = v10;
                        h449
                    })), List::cons(f8(Str::lit("-- operations, constants and the other members that are not attributes are behavior, not data, and are not translated (Web IDL 2.5)\u{a}")), List::cons(f103(f104({
                        let (h450, _) = v10;
                        h450
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
    let m451 = f90(f91(v0, v1.clone()), v1);
    match m451 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m452 = v2;
            match m452 {
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
    let m453 = v1;
    match m453.uncons() {
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
            let a455 = {
                let a454 = v0;
                length_bytes(a454)
            };
            let a456 = 3u64;
            nat_sub(a455, a456)
        })
    } else {
        v0
    };
    let v2: Bytes = f108(v1)?;
    let v3: u64 = {
        let a457 = v2.clone();
        length_bytes(a457)
    };
    let v4: u64 = f109(v2.clone())?;
    let a458 = v4;
    let a459 = v3;
    if nat_lt(a458, a459) {
        Ok(f110(f111(Str::lit("the input is not a sequence of Chars in UTF-8 (XML 1.0 [2], section 4.3.3)"), v4)?))
    } else {
        let m460 = f112(v2.clone())?;
        match m460 {
            Ok(v6) => {
                let m463 = f113(v2.clone(), v6, {
                    let a461 = v3;
                    let a462 = 2u64;
                    nat_add(a461, a462)?
                })?;
                match m463 {
                    Ok(v8) => {
                        if f114(v2.clone(), v8, Str::lit("<!DOCTYPE")) {
                            Ok(f110(f111(Str::lit("a document type declaration is not read; Verdant Studio reads documents that have none (XML 1.0 [28])"), v8)?))
                        } else {
                            let a464 = f115(v2.clone(), v8)?;
                            let a465 = 60u64;
                            if nat_eq(a464, a465) {
                                let m468 = f116(v2.clone(), v8, {
                                    let a466 = v3;
                                    let a467 = 2u64;
                                    nat_add(a466, a467)?
                                }, 0u64)?;
                                match m468 {
                                    Ok(v10) => {
                                        let m472 = f113(v2, {
                                            let (_, h469) = v10;
                                            h469
                                        }, {
                                            let a470 = v3;
                                            let a471 = 2u64;
                                            nat_add(a470, a471)?
                                        })?;
                                        match m472 {
                                            Ok(v12) => {
                                                let a473 = v12;
                                                let a474 = v3;
                                                if nat_eq(a473, a474) {
                                                    Ok(Ok::<Adt17, Bytes>({
                                                        let (h475, _) = v10;
                                                        h475
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
    let m476 = v0;
    match m476 {
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
    let m477 = v0;
    match m477 {
        Adt17::C0(_, _, r482) => {
            let v3 = (*r482).clone();
            let a478 = 1u64;
            let a479 = f68(v3)?;
            nat_add(a478, a479)
        }
        Adt17::C1(_, _, _) => {
            Ok(0u64)
        }
        Adt17::C2(_) => {
            Ok(0u64)
        }
        Adt17::C3(r483, r484) => {
            let v8 = (*r483).clone();
            let v9 = (*r484).clone();
            let a480 = f68(v8)?;
            let a481 = f68(v9)?;
            nat_add(a480, a481)
        }
        Adt17::C4 => {
            Ok(0u64)
        }
    }
}

pub fn f69(v0: Bytes) -> R<Result<(u64, Bytes), Bytes>> {
    let m485 = f66(v0)?;
    match m485 {
        Ok(v2) => {
            let m486 = v2;
            match m486 {
                Adt17::C0(v3, r498, r499) => {
                    let v4 = (*r498).clone();
                    let v5 = (*r499).clone();
                    let m487 = f118(v4.clone(), List::<(Bytes, Bytes)>::nil());
                    match m487 {
                        Ok(v7) => {
                            let m488 = f119(v7.clone(), v3)?;
                            match m488 {
                                Ok(v9) => {
                                    if f120(v9, Str::lit("schema")) {
                                        let v11: Bytes = {
                                            let m489 = f121(v4, f8(Str::lit("targetNamespace")));
                                            match m489 {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some(v10) => {
                                                    v10
                                                }
                                            }
                                        };
                                        let m490 = f122(v5, v7, v11)?;
                                        match m490 {
                                            Ok(v13) => {
                                                let v14: List<Bytes> = f123(v13.clone());
                                                let m491 = f124(v14.clone());
                                                match m491 {
                                                    None => {
                                                        let m492 = f125(v13.clone(), v14.clone());
                                                        match m492 {
                                                            None => {
                                                                let m496 = f126({
                                                                    let a494 = {
                                                                        let a493 = v14.clone();
                                                                        length_list(a493)
                                                                    };
                                                                    let a495 = 1u64;
                                                                    nat_add(a494, a495)?
                                                                }, v13, List::<Bytes>::nil(), List::<Bytes>::nil());
                                                                match m496 {
                                                                    Ok(v18) => {
                                                                        Ok(Ok::<(u64, Bytes), Bytes>(({
                                                                            let a497 = v14;
                                                                            length_list(a497)
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
        let a501 = {
            let a500 = v3.clone();
            length_list(a500)
        };
        let a502 = 2u64;
        nat_add(a501, a502)?
    }, v0, v3, List::cons((v2, (f8(Str::lit("root")), Bytes::lit(&[]))), List::<(Bytes, (Bytes, Bytes))>::nil()), List::<Adt19>::nil(), List::<Bytes>::nil())
}

pub fn f71(v0: Adt1) -> List<(Bytes, Option<Bytes>)> {
    f129(match v0 {
        Adt1::C0(h503, _, _, _) => {
            h503
        }
    })
}

pub fn f72(v0: Bytes) -> R<Result<List<Adt2>, Bytes>> {
    let v1: Bytes = if f107(v0.clone(), Bytes::lit(&[0xef, 0xbb, 0xbf]), 0u64) {
        f43(v0.clone(), 3u64, {
            let a505 = {
                let a504 = v0;
                length_bytes(a504)
            };
            let a506 = 3u64;
            nat_sub(a505, a506)
        })
    } else {
        v0
    };
    let v2: Bytes = f108(v1)?;
    let v3: u64 = {
        let a507 = v2.clone();
        length_bytes(a507)
    };
    let v4: u64 = f130(v2.clone())?;
    let v5: u64 = f131(v2.clone(), 0u64)?;
    let a508 = v4;
    let a509 = v3;
    if nat_lt(a508, a509) {
        Ok(Err::<List<Adt2>, Bytes>(f111(Str::lit("the input is not a sequence of printable characters in UTF-8 (YAML 1.2.2 [1], section 5.2)"), v4)?))
    } else {
        let m512 = f132(v2.clone(), v5, {
            let a510 = v3;
            let a511 = 1u64;
            nat_add(a510, a511)?
        })?;
        match m512 {
            Ok(v7) => {
                let a513 = v3;
                let a514 = v7;
                if nat_le(a513, a514) {
                    Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
                } else {
                    f133(v2, v7, true, {
                        let a515 = v3;
                        let a516 = 1u64;
                        nat_add(a515, a516)?
                    })
                }
            }
            Err(v6) => {
                Ok(Err::<List<Adt2>, Bytes>(v6))
            }
        }
    }
}

pub fn f73(v0: List<Adt2>) -> R<Bytes> {
    let m517 = v0;
    match m517.uncons() {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some((v1, v2)) => {
            Ok(f7(List::cons(f134(v1)?, List::cons(f8(Str::lit("\u{a}")), List::cons(f73(v2)?, List::<Bytes>::nil())))))
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

pub fn f82(v0: Bytes, p545: u64, p546: u64, p547: bool, p548: bool, p549: u64, p550: Bytes) -> R<Bytes> {
    let m544 = (p545, (p546, (p547, (p548, (p549, p550)))));
    {
        let mut m544 = m544;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, (v5, v6))))) = m544;
                let m518 = v2;
                if m518 == 0 {
                    Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f135(v0.clone(), v5, v1, v6)))
                } else {
                    let v7 = m518 - 1;
                    let m521 = {
                        let a519 = v0.clone();
                        let a520 = v1;
                        index_bytes(a519, a520)
                    };
                    match m521 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f135(v0.clone(), v5, v1, v6)))
                        }
                        Some(v8) => {
                            let v9: u64 = f52(v8)?;
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                    let a522 = v1;
                                    let a523 = 1u64;
                                    nat_add(a522, a523)?
                                }, (v7, (if v4 {
                                    true
                                } else {
                                    let a524 = v9;
                                    let a525 = 34u64;
                                    !nat_eq(a524, a525)
                                }, (if v4 {
                                    false
                                } else {
                                    let a526 = v9;
                                    let a527 = 92u64;
                                    nat_eq(a526, a527)
                                }, (v5, v6))))))
                            } else {
                                let a528 = v9;
                                let a529 = 32u64;
                                if if if if nat_eq(a528, a529) {
                                    true
                                } else {
                                    let a530 = v9;
                                    let a531 = 9u64;
                                    nat_eq(a530, a531)
                                } {
                                    true
                                } else {
                                    let a532 = v9;
                                    let a533 = 10u64;
                                    nat_eq(a532, a533)
                                } {
                                    true
                                } else {
                                    let a534 = v9;
                                    let a535 = 13u64;
                                    nat_eq(a534, a535)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a536 = v1;
                                        let a537 = 1u64;
                                        nat_add(a536, a537)?
                                    }, (v7, (false, (false, ({
                                        let a538 = v1;
                                        let a539 = 1u64;
                                        nat_add(a538, a539)?
                                    }, f135(v0.clone(), v5, v1, v6)))))))
                                } else {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a540 = v1;
                                        let a541 = 1u64;
                                        nat_add(a540, a541)?
                                    }, (v7, ({
                                        let a542 = v9;
                                        let a543 = 34u64;
                                        nat_eq(a542, a543)
                                    }, (false, (v5, v6))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m544 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f83(v0: Bytes, v1: u64) -> R<u64> {
    let m553 = {
        let a551 = v0;
        let a552 = v1;
        index_bytes(a551, a552)
    };
    match m553 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a598 = {
                let a574 = {
                    let a562 = {
                        let a556 = {
                            let a554 = v3;
                            let a555 = 128u8;
                            fixed_u8::bit_and(a554, a555)
                        };
                        let a557 = 128u8;
                        if equal(a556, a557) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a563 = {
                        let a560 = {
                            let a558 = v3;
                            let a559 = 64u8;
                            fixed_u8::bit_and(a558, a559)
                        };
                        let a561 = 64u8;
                        if equal(a560, a561) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a562, a563)?
                };
                let a575 = {
                    let a572 = {
                        let a566 = {
                            let a564 = v3;
                            let a565 = 32u8;
                            fixed_u8::bit_and(a564, a565)
                        };
                        let a567 = 32u8;
                        if equal(a566, a567) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a573 = {
                        let a570 = {
                            let a568 = v3;
                            let a569 = 16u8;
                            fixed_u8::bit_and(a568, a569)
                        };
                        let a571 = 16u8;
                        if equal(a570, a571) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a572, a573)?
                };
                nat_add(a574, a575)?
            };
            let a599 = {
                let a596 = {
                    let a584 = {
                        let a578 = {
                            let a576 = v3;
                            let a577 = 8u8;
                            fixed_u8::bit_and(a576, a577)
                        };
                        let a579 = 8u8;
                        if equal(a578, a579) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a585 = {
                        let a582 = {
                            let a580 = v3;
                            let a581 = 4u8;
                            fixed_u8::bit_and(a580, a581)
                        };
                        let a583 = 4u8;
                        if equal(a582, a583) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a584, a585)?
                };
                let a597 = {
                    let a594 = {
                        let a588 = {
                            let a586 = v3;
                            let a587 = 2u8;
                            fixed_u8::bit_and(a586, a587)
                        };
                        let a589 = 2u8;
                        if equal(a588, a589) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a595 = {
                        let a592 = {
                            let a590 = v3;
                            let a591 = 1u8;
                            fixed_u8::bit_and(a590, a591)
                        };
                        let a593 = 1u8;
                        if equal(a592, a593) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a594, a595)?
                };
                nat_add(a596, a597)?
            };
            nat_add(a598, a599)
        }
    }
}

pub fn f84(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = f136(v0.clone(), v1, {
        let a601 = {
            let a600 = v0.clone();
            length_bytes(a600)
        };
        let a602 = 1u64;
        nat_add(a601, a602)?
    })?;
    let a603 = f83(v0.clone(), v2)?;
    let a604 = 34u64;
    if nat_eq(a603, a604) {
        Ok(Some(({
            let a605 = v2;
            let a606 = 1u64;
            nat_add(a605, a606)?
        }, f43(v0, v1, {
            let a607 = v2;
            let a608 = v1;
            nat_sub(a607, a608)
        }))))
    } else {
        f137(v0.clone(), v1, {
            let a610 = {
                let a609 = v0;
                length_bytes(a609)
            };
            let a611 = 1u64;
            nat_add(a610, a611)?
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
        let a612 = f83(v0.clone(), v1)?;
        let a613 = 45u64;
        nat_eq(a612, a613)
    };
    let v3: u64 = if v2 {
        let a614 = v1;
        let a615 = 1u64;
        nat_add(a614, a615)?
    } else {
        v1
    };
    let v4: u64 = f138(v0.clone(), v3)?;
    let v5: u64 = {
        let a616 = v4;
        let a617 = v3;
        nat_sub(a616, a617)
    };
    let v6: bool = {
        let a618 = f83(v0.clone(), v4)?;
        let a619 = 46u64;
        nat_eq(a618, a619)
    };
    let v7: u64 = if v6 {
        let a620 = v4;
        let a621 = 1u64;
        nat_add(a620, a621)?
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
        let a622 = v9;
        let a623 = 101u64;
        if nat_eq(a622, a623) {
            true
        } else {
            let a624 = v9;
            let a625 = 69u64;
            nat_eq(a624, a625)
        }
    };
    let v11: u64 = f83(v0.clone(), {
        let a626 = v8;
        let a627 = 1u64;
        nat_add(a626, a627)?
    })?;
    let v12: bool = f85(v10, f139({
        let a628 = v11;
        let a629 = 43u64;
        nat_eq(a628, a629)
    }, {
        let a630 = v11;
        let a631 = 45u64;
        nat_eq(a630, a631)
    }));
    let v13: u64 = if v10 {
        if v12 {
            let a632 = v8;
            let a633 = 2u64;
            nat_add(a632, a633)?
        } else {
            let a634 = v8;
            let a635 = 1u64;
            nat_add(a634, a635)?
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
        let a636 = v14;
        let a637 = v13;
        nat_sub(a636, a637)
    });
    let v16: u64 = f140(v15.clone())?;
    let v17: Bytes = f43(v15.clone(), v16, {
        let a639 = {
            let a638 = v15;
            length_bytes(a638)
        };
        let a640 = v16;
        nat_sub(a639, a640)
    });
    let a641 = v5;
    let a642 = 0u64;
    if if nat_eq(a641, a642) {
        true
    } else {
        if f85({
            let a643 = 1u64;
            let a644 = v5;
            nat_lt(a643, a644)
        }, {
            let a645 = f83(v0.clone(), v3)?;
            let a646 = 48u64;
            nat_eq(a645, a646)
        }) {
            true
        } else {
            if f85(v6, {
                let a647 = v8;
                let a648 = v7;
                nat_eq(a647, a648)
            }) {
                true
            } else {
                f139(if v10 {
                    let a649 = v14;
                    let a650 = v13;
                    nat_eq(a649, a650)
                } else {
                    false
                }, {
                    let a652 = 18u64;
                    let a653 = {
                        let a651 = v17.clone();
                        length_bytes(a651)
                    };
                    nat_lt(a652, a653)
                })
            }
        }
    } {
        Ok(None::<(u64, Adt2)>)
    } else {
        let m657 = {
            let a655 = {
                let a654 = v17.clone();
                length_bytes(a654)
            };
            let a656 = 0u64;
            if nat_eq(a655, a656) {
                Some(0u64)
            } else {
                f141(v17)?
            }
        };
        match m657 {
            None => {
                Ok(None::<(u64, Adt2)>)
            }
            Some(v18) => {
                let m662 = f142(f43(v0.clone(), v3, v5), f43(v0, v7, {
                    let a658 = v8;
                    let a659 = v7;
                    nat_sub(a658, a659)
                }), f85(v12, {
                    let a660 = v11;
                    let a661 = 45u64;
                    nat_eq(a660, a661)
                }), v18)?;
                match m662 {
                    None => {
                        Ok(None::<(u64, Adt2)>)
                    }
                    Some(v19) => {
                        let a665 = 19u64;
                        let a666 = {
                            let a664 = {
                                let (h663, _) = v19.clone();
                                h663
                            };
                            length_bytes(a664)
                        };
                        if nat_lt(a665, a666) {
                            Ok(None::<(u64, Adt2)>)
                        } else {
                            let m668 = f141({
                                let (h667, _) = v19.clone();
                                h667
                            })?;
                            match m668 {
                                None => {
                                    Ok(None::<(u64, Adt2)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f85(v2, {
                                        let a669 = 0u64;
                                        let a670 = v20;
                                        nat_lt(a669, a670)
                                    });
                                    if f143(v21, v20) {
                                        Ok(Some((v14, {
                                            let a672 = {
                                                let (_, h671) = v19.clone();
                                                h671
                                            };
                                            let a673 = 0u64;
                                            if nat_eq(a672, a673) {
                                                Adt2::C3(v21, v20)
                                            } else {
                                                Adt2::C4(v21, v20, {
                                                    let (_, h674) = v19;
                                                    h674
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
        let (h675, _) = v2.clone();
        h675
    };
    let v4: u64 = {
        let (h676, _) = {
            let (_, h677) = v2.clone();
            h677
        };
        h676
    };
    let v5: List<Bytes> = {
        let (_, h678) = {
            let (_, h679) = v2;
            h679
        };
        h678
    };
    let m680 = f144(v0.clone(), v3);
    match m680 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a681 = v6;
            let a682 = v1;
            if equal(a681, a682) {
                Ok(Some(({
                    let a683 = v3;
                    let a684 = 1u64;
                    nat_add(a683, a684)?
                }, ({
                    let a685 = v3;
                    let a686 = 1u64;
                    nat_add(a685, a686)?
                }, List::cons(f43(v0, v4, {
                    let a687 = v3;
                    let a688 = v4;
                    nat_sub(a687, a688)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a689 = v3;
                    let a690 = 1u64;
                    nat_add(a689, a690)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f88(v0: Fn2, p694: List<Bytes>, p695: List<Bytes>) -> List<Bytes> {
    let m693 = (p694, p695);
    {
        let mut m693 = m693;
        loop {
            let step = {
                let (v1, v2) = m693;
                let m691 = v2;
                match m691.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c692: Fn2 = v0.clone();
                            c692.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m693 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f89(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f90(p700: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m699 = p700;
    {
        let mut m699 = m699;
        loop {
            let step = {
                let v0 = m699;
                let m696 = v0;
                match m696.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f46({
                            let (h697, _) = v2;
                            h697
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h698) = v2;
                                h698
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m699 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f91(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Bytes>)> {
    let m704 = {
        let a702 = match v0 {
            Adt1::C0(h701, _, _, _) => {
                h701
            }
        };
        let a703 = f37(v1);
        index_list(a702, a703)
    };
    match m704 {
        None => {
            List::<(Bytes, Option<Bytes>)>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f92(v0: Bytes) -> R<Result<List<Adt13>, Bytes>> {
    let m705 = f145(v0)?;
    match m705 {
        Ok(v2) => {
            f146({
                let a707 = {
                    let a706 = v2.clone();
                    length_list(a706)
                };
                let a708 = 1u64;
                nat_add(a707, a708)?
            }, v2, List::<Adt13>::nil())
        }
        Err(v1) => {
            Ok(Err::<List<Adt13>, Bytes>(v1))
        }
    }
}

pub fn f93(v0: List<Bytes>) -> R<Result<List<Adt13>, Bytes>> {
    let m709 = v0;
    match m709.uncons() {
        None => {
            Ok(Ok::<List<Adt13>, Bytes>(List::<Adt13>::nil()))
        }
        Some((v1, v2)) => {
            let m710 = f92(v1)?;
            match m710 {
                Ok(v4) => {
                    let m711 = f93(v2)?;
                    match m711 {
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
    let m712 = v0;
    match m712.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f94(v3, v1))
        }
    }
}

pub fn f95(p717: u64, p718: List<Adt16>) -> R<List<Adt16>> {
    let m716 = (p717, p718);
    {
        let mut m716 = m716;
        loop {
            let step = {
                let (v0, v1) = m716;
                let m713 = v0;
                if m713 == 0 {
                    Err::<(u64, List<Adt16>), R<List<Adt16>>>(Ok(v1))
                } else {
                    let v2 = m713 - 1;
                    let v3: List<Adt16> = f147(v1.clone(), v1.clone());
                    let a714 = f148(v3.clone())?;
                    let a715 = f148(v1.clone())?;
                    if nat_eq(a714, a715) {
                        Err::<(u64, List<Adt16>), R<List<Adt16>>>(Ok(v1))
                    } else {
                        Ok::<(u64, List<Adt16>), R<List<Adt16>>>((v2, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m716 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f96(v0: List<Adt13>, v1: List<Adt13>) -> R<List<Adt16>> {
    let m719 = v0;
    match m719.uncons() {
        None => {
            Ok(List::<Adt16>::nil())
        }
        Some((v2, v3)) => {
            if f149(f150(v2.clone())) {
                let v6: Adt16 = {
                    let m720 = f151(v1.clone(), v2.clone())?;
                    match m720 {
                        Ok(v5) => {
                            Adt16::C0(f152(v2), Bytes::lit(&[]), {
                                let (h721, _) = v5;
                                h721
                            }, {
                                let (_, h722) = v5;
                                h722
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

pub fn f97(p727: u64, p728: List<Bytes>, p729: List<Bytes>, v3: List<Adt16>) -> List<Bytes> {
    let m726 = (p727, (p728, p729));
    {
        let mut m726 = m726;
        loop {
            let step = {
                let (v0, (v1, v2)) = m726;
                let m723 = v0;
                if m723 == 0 {
                    Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v2)
                } else {
                    let v4 = m723 - 1;
                    let m724 = v1;
                    match m724.uncons() {
                        None => {
                            Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v2)
                        }
                        Some((v5, v6)) => {
                            if f153(v5.clone(), v2.clone()) {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v2)))
                            } else {
                                let m725 = f154(v3.clone(), v5.clone());
                                match m725 {
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
                Ok(next) => { m726 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f98(v0: List<Adt13>) -> List<Bytes> {
    let m730 = v0;
    match m730.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let v3: u64 = f150(v1.clone());
            let a731 = v3;
            let a732 = 0u64;
            if if nat_eq(a731, a732) {
                true
            } else {
                let a733 = v3;
                let a734 = 2u64;
                if nat_eq(a733, a734) {
                    true
                } else {
                    let a735 = v3;
                    let a736 = 3u64;
                    if nat_eq(a735, a736) {
                        true
                    } else {
                        let a737 = v3;
                        let a738 = 4u64;
                        if nat_eq(a737, a738) {
                            true
                        } else {
                            let a739 = v3;
                            let a740 = 5u64;
                            if nat_eq(a739, a740) {
                                true
                            } else {
                                let a741 = v3;
                                let a742 = 7u64;
                                if nat_eq(a741, a742) {
                                    true
                                } else {
                                    let a743 = v3;
                                    let a744 = 8u64;
                                    nat_eq(a743, a744)
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
    let m745 = v0;
    match m745.uncons() {
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

pub fn f100(p754: u64, p755: List<(Bytes, (List<Bytes>, Bytes))>, p756: List<Bytes>, p757: List<Bytes>) -> (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>) {
    let m753 = (p754, (p755, (p756, p757)));
    {
        let mut m753 = m753;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m753;
                let m746 = v0;
                if m746 == 0 {
                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v3, v1))
                } else {
                    let v4 = m746 - 1;
                    let m747 = f160(v1.clone(), v2.clone());
                    match m747 {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v3, v1))
                        }
                        Some(v5) => {
                            Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v4, ({
                                let (_, h748) = v5.clone();
                                h748
                            }, (List::cons({
                                let (h749, _) = {
                                    let (h750, _) = v5.clone();
                                    h750
                                };
                                h749
                            }, v2), List::cons({
                                let (_, h751) = {
                                    let (h752, _) = v5;
                                    h752
                                };
                                h751
                            }, v3)))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m753 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f101(v0: List<Adt13>, v1: List<Adt16>, v2: List<Adt13>) -> List<Bytes> {
    let m758 = v0;
    match m758.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            let v5: u64 = f150(v3.clone());
            let a759 = v5;
            let a760 = 5u64;
            if if nat_eq(a759, a760) {
                true
            } else {
                let a761 = v5;
                let a762 = 7u64;
                nat_eq(a761, a762)
            } {
                let m765 = f161(v2.clone(), {
                    let a763 = v5;
                    let a764 = 5u64;
                    if nat_eq(a763, a764) {
                        0u64
                    } else {
                        2u64
                    }
                }, f152(v3.clone()));
                match m765 {
                    None => {
                        List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f162(v5), List::cons(f152(v3.clone()), List::cons(f8(Str::lit(": no input defines ")), List::cons(f152(v3), List::cons(f8(Str::lit(", so its members join no definition (Web IDL 2.2 and 2.7)\u{a}")), List::<Bytes>::nil()))))))), f101(v4, v1, v2))
                    }
                    Some(_) => {
                        f101(v4, v1, v2)
                    }
                }
            } else {
                let a766 = v5;
                let a767 = 8u64;
                if nat_eq(a766, a767) {
                    let m768 = f161(v2.clone(), 0u64, f152(v3.clone()));
                    match m768 {
                        None => {
                            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f152(v3.clone()), List::cons(f8(Str::lit(" includes ")), List::cons(f163(v3.clone()), List::cons(f8(Str::lit(": no input defines the interface ")), List::cons(f152(v3), List::cons(f8(Str::lit(" (Web IDL 2.3)\u{a}")), List::<Bytes>::nil())))))))), f101(v4, v1, v2))
                        }
                        Some(_) => {
                            f101(v4, v1, v2)
                        }
                    }
                } else {
                    if f149(v5) {
                        let m769 = f154(v1.clone(), f152(v3.clone()));
                        match m769 {
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
    let m770 = v0;
    match m770.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons({
                let (h771, _) = v1;
                h771
            }, List::cons(f8(Str::lit(": it refers back to itself through its members, and the translator emits non-recursive declarations only\u{a}")), List::<Bytes>::nil())))), f102(v2))
        }
    }
}

pub fn f103(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m772 = v0;
    match m772.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m773 = v3.clone();
            match m773.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a776 = v2;
                    let a777 = {
                        let a774 = v1.clone();
                        let a775 = f103(v3, v1);
                        append_bytes(a774, a775)
                    };
                    append_bytes(a776, a777)
                }
            }
        }
    }
}

pub fn f104(p780: List<Bytes>, p781: List<Bytes>) -> List<Bytes> {
    let m779 = (p780, p781);
    {
        let mut m779 = m779;
        loop {
            let step = {
                let (v0, v1) = m779;
                let m778 = v0;
                match m778.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m779 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f105(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a782 = v2;
    let a783 = 0u64;
    if nat_eq(a782, a783) {
        Ok(f164(v0, 0u64))
    } else {
        let (_, h784) = {
            let (h785, _) = f165(Fn3::F166(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h785
        };
        Ok(h784)
    }
}

pub fn f106() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f107(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m790 = {
        let a787 = v0;
        let a788 = v2;
        let a789 = {
            let a786 = v1.clone();
            length_bytes(a786)
        };
        slice_bytes(a787, a788, a789)
    };
    match m790 {
        None => {
            false
        }
        Some(v3) => {
            let a791 = v3;
            let a792 = v1;
            equal(a791, a792)
        }
    }
}

pub fn f108(v0: Bytes) -> R<Bytes> {
    let (_, h793) = {
        let (h794, _) = f165(Fn3::F167(v0.clone()), {
            let a796 = {
                let a795 = v0;
                length_bytes(a795)
            };
            let a797 = 1u64;
            nat_add(a796, a797)?
        }, (0u64, Bytes::lit(&[])))?;
        h794
    };
    Ok(h793)
}

pub fn f109(v0: Bytes) -> R<u64> {
    let (h798, _) = f168(Fn4::F169(v0.clone()), {
        let a800 = {
            let a799 = v0;
            length_bytes(a799)
        };
        let a801 = 1u64;
        nat_add(a800, a801)?
    }, 0u64)?;
    Ok(h798)
}

pub fn f110(v0: Bytes) -> Result<Adt17, Bytes> {
    Err::<Adt17, Bytes>(v0)
}

pub fn f111(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a802 = v0;
        utf8_encode(a802)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f65(v1)?, List::<Bytes>::nil())))))
}

pub fn f112(v0: Bytes) -> R<Result<u64, Bytes>> {
    if if f114(v0.clone(), 0u64, Str::lit("<?xml")) {
        f170(f115(v0.clone(), 5u64)?)?
    } else {
        false
    } {
        let m804 = f171(v0.clone(), {
            let a803 = Str::lit("?>");
            utf8_encode(a803)
        }, 5u64)?;
        match m804 {
            None => {
                Ok(f172(f111(Str::lit("the XML declaration is not closed (XML 1.0 [23])"), 0u64)?))
            }
            Some(v1) => {
                let m806 = f171(f43(v0.clone(), 0u64, v1), {
                    let a805 = Str::lit("encoding");
                    utf8_encode(a805)
                }, 5u64)?;
                match m806 {
                    None => {
                        Ok(Ok::<u64, Bytes>({
                            let a807 = v1;
                            let a808 = 2u64;
                            nat_add(a807, a808)?
                        }))
                    }
                    Some(v2) => {
                        let v3: u64 = f173(v0.clone(), {
                            let a809 = v2;
                            let a810 = 8u64;
                            nat_add(a809, a810)?
                        })?;
                        let v4: u64 = f173(v0.clone(), {
                            let a811 = v3;
                            let a812 = 1u64;
                            nat_add(a811, a812)?
                        })?;
                        let _: u64 = f115(v0.clone(), v4)?;
                        let v7: u64 = {
                            let m815 = f171(v0.clone(), f43(v0.clone(), v4, 1u64), {
                                let a813 = v4;
                                let a814 = 1u64;
                                nat_add(a813, a814)?
                            })?;
                            match m815 {
                                None => {
                                    v1
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        if f174(f43(v0, {
                            let a816 = v4;
                            let a817 = 1u64;
                            nat_add(a816, a817)?
                        }, {
                            let a820 = v7;
                            let a821 = {
                                let a818 = v4;
                                let a819 = 1u64;
                                nat_add(a818, a819)?
                            };
                            nat_sub(a820, a821)
                        })) {
                            Ok(Ok::<u64, Bytes>({
                                let a822 = v1;
                                let a823 = 2u64;
                                nat_add(a822, a823)?
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

pub fn f113(v0: Bytes, p828: u64, p829: u64) -> R<Result<u64, Bytes>> {
    let m827 = (p828, p829);
    {
        let mut m827 = m827;
        loop {
            let step = {
                let (v1, v2) = m827;
                let m824 = v2;
                if m824 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f172(f111(Str::lit("the prolog is longer than the input"), v1)?)))
                } else {
                    let v3 = m824 - 1;
                    let v4: u64 = f173(v0.clone(), v1)?;
                    if f114(v0.clone(), v4, Str::lit("<!--")) {
                        let m825 = f175(v0.clone(), v4)?;
                        match m825 {
                            Ok(v6) => {
                                Ok::<(u64, u64), R<Result<u64, Bytes>>>((v6, v3))
                            }
                            Err(v5) => {
                                Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f172(v5)))
                            }
                        }
                    } else {
                        if f114(v0.clone(), v4, Str::lit("<?")) {
                            let m826 = f176(v0.clone(), v4)?;
                            match m826 {
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
                Ok(next) => { m827 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f114(v0: Bytes, v1: u64, v2: Str) -> bool {
    f107(v0, {
        let a830 = v2;
        utf8_encode(a830)
    }, v1)
}

pub fn f115(v0: Bytes, v1: u64) -> R<u64> {
    f83(v0, v1)
}

pub fn f116(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Result<(Adt17, u64), Bytes>> {
    let m831 = v2;
    if m831 == 0 {
        Ok(f177(f111(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v4 = m831 - 1;
        let a832 = v3;
        let a833 = 0u64;
        if nat_eq(a832, a833) {
            let v5: u64 = f178(v0.clone(), {
                let a834 = v1;
                let a835 = 1u64;
                nat_add(a834, a835)?
            })?;
            let a838 = v5;
            let a839 = {
                let a836 = v1;
                let a837 = 1u64;
                nat_add(a836, a837)?
            };
            if nat_eq(a838, a839) {
                Ok(f177(f111(Str::lit("an element must start with a Name (XML 1.0 [40])"), v1)?))
            } else {
                let v6: Bytes = f43(v0.clone(), {
                    let a840 = v1;
                    let a841 = 1u64;
                    nat_add(a840, a841)?
                }, {
                    let a844 = v5;
                    let a845 = {
                        let a842 = v1;
                        let a843 = 1u64;
                        nat_add(a842, a843)?
                    };
                    nat_sub(a844, a845)
                });
                let m846 = f179(v0.clone(), v5, v4, List::<Bytes>::nil())?;
                match m846 {
                    Ok(v8) => {
                        if f114(v0.clone(), {
                            let (_, h847) = v8.clone();
                            h847
                        }, Str::lit("/>")) {
                            Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C0(v6, std::rc::Rc::new({
                                let (h848, _) = v8;
                                h848
                            }), std::rc::Rc::new(Adt17::C4)), {
                                let a850 = {
                                    let (_, h849) = v8;
                                    h849
                                };
                                let a851 = 2u64;
                                nat_add(a850, a851)?
                            })))
                        } else {
                            let m855 = f116(v0.clone(), {
                                let a853 = {
                                    let (_, h852) = v8;
                                    h852
                                };
                                let a854 = 1u64;
                                nat_add(a853, a854)?
                            }, v4, 1u64)?;
                            match m855 {
                                Ok(v10) => {
                                    let v11: u64 = {
                                        let a857 = {
                                            let (_, h856) = v10.clone();
                                            h856
                                        };
                                        let a858 = 2u64;
                                        nat_add(a857, a858)?
                                    };
                                    let v12: u64 = f178(v0.clone(), v11)?;
                                    if f46(f43(v0.clone(), v11, {
                                        let a859 = v12;
                                        let a860 = v11;
                                        nat_sub(a859, a860)
                                    }), v6.clone()) {
                                        let v13: u64 = f173(v0.clone(), v12)?;
                                        let a861 = f115(v0, v13)?;
                                        let a862 = 62u64;
                                        if nat_eq(a861, a862) {
                                            Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C0(v6, std::rc::Rc::new({
                                                let (h863, _) = v8;
                                                h863
                                            }), std::rc::Rc::new({
                                                let (h864, _) = v10;
                                                h864
                                            })), {
                                                let a865 = v13;
                                                let a866 = 1u64;
                                                nat_add(a865, a866)?
                                            })))
                                        } else {
                                            Ok(f177(f111(Str::lit("an end tag must close with > (XML 1.0 [42])"), v13)?))
                                        }
                                    } else {
                                        Ok(f177(f111(Str::lit("an end tag must name the element its start tag opened (XML 1.0 WFC: Element Type Match)"), {
                                            let (_, h867) = v10;
                                            h867
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
            let a868 = f115(v0.clone(), v1)?;
            let a869 = 256u64;
            if nat_eq(a868, a869) {
                Ok(f177(f111(Str::lit("the input ends before an element is closed (XML 1.0 [39])"), v1)?))
            } else {
                if f114(v0.clone(), v1, Str::lit("</")) {
                    Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C4, v1)))
                } else {
                    if f114(v0.clone(), v1, Str::lit("<!--")) {
                        let m870 = f175(v0.clone(), v1)?;
                        match m870 {
                            Ok(v15) => {
                                f116(v0, v15, v4, 1u64)
                            }
                            Err(v14) => {
                                Ok(f177(v14))
                            }
                        }
                    } else {
                        if f114(v0.clone(), v1, Str::lit("<![CDATA[")) {
                            let m874 = f171(v0.clone(), {
                                let a871 = Str::lit("]]>");
                                utf8_encode(a871)
                            }, {
                                let a872 = v1;
                                let a873 = 9u64;
                                nat_add(a872, a873)?
                            })?;
                            match m874 {
                                None => {
                                    Ok(f177(f111(Str::lit("a CDATA section is not closed (XML 1.0 [18])"), v1)?))
                                }
                                Some(v16) => {
                                    let m877 = f116(v0.clone(), {
                                        let a875 = v16;
                                        let a876 = 3u64;
                                        nat_add(a875, a876)?
                                    }, v4, 1u64)?;
                                    match m877 {
                                        Ok(v18) => {
                                            Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C3(std::rc::Rc::new(Adt17::C2(f43(v0, {
                                                let a878 = v1;
                                                let a879 = 9u64;
                                                nat_add(a878, a879)?
                                            }, {
                                                let a882 = v16;
                                                let a883 = {
                                                    let a880 = v1;
                                                    let a881 = 9u64;
                                                    nat_add(a880, a881)?
                                                };
                                                nat_sub(a882, a883)
                                            }))), std::rc::Rc::new({
                                                let (h884, _) = v18;
                                                h884
                                            })), {
                                                let (_, h885) = v18;
                                                h885
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
                                let m886 = f176(v0.clone(), v1)?;
                                match m886 {
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
                                    let a887 = f115(v0.clone(), v1)?;
                                    let a888 = 60u64;
                                    if nat_eq(a887, a888) {
                                        let m889 = f116(v0.clone(), v1, v4, 0u64)?;
                                        match m889 {
                                            Ok(v22) => {
                                                let m891 = f116(v0, {
                                                    let (_, h890) = v22;
                                                    h890
                                                }, v4, 1u64)?;
                                                match m891 {
                                                    Ok(v24) => {
                                                        Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C3(std::rc::Rc::new({
                                                            let (h892, _) = v22;
                                                            h892
                                                        }), std::rc::Rc::new({
                                                            let (h893, _) = v24;
                                                            h893
                                                        })), {
                                                            let (_, h894) = v24;
                                                            h894
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
                                        let m895 = f180(v0.clone(), v1, v4, Bytes::lit(&[]), false, 0u64)?;
                                        match m895 {
                                            Ok(v26) => {
                                                let m897 = f116(v0, {
                                                    let (_, h896) = v26;
                                                    h896
                                                }, v4, 1u64)?;
                                                match m897 {
                                                    Ok(v28) => {
                                                        Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C3(std::rc::Rc::new(Adt17::C2({
                                                            let (h898, _) = v26;
                                                            h898
                                                        })), std::rc::Rc::new({
                                                            let (h899, _) = v28;
                                                            h899
                                                        })), {
                                                            let (_, h900) = v28;
                                                            h900
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
    let m901 = v0;
    match m901 {
        Adt17::C0(_, _, _) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt17::C1(v2, v3, r906) => {
            let v4 = (*r906).clone();
            let m902 = f118(v4, v1);
            match m902 {
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
                                let a904 = {
                                    let a903 = v2;
                                    length_bytes(a903)
                                };
                                let a905 = 6u64;
                                nat_sub(a904, a905)
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
        let m907 = f186(v0, Bytes::lit(&[]));
        match m907 {
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
            let m908 = f186(v0, v2.clone());
            match m908 {
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
        let (h909, _) = v0;
        h909
    }, f188()) {
        f46({
            let (_, h910) = v0;
            h910
        }, {
            let a911 = v1;
            utf8_encode(a911)
        })
    } else {
        false
    }
}

pub fn f121(p915: Adt17, v1: Bytes) -> Option<Bytes> {
    let m914 = p915;
    {
        let mut m914 = m914;
        loop {
            let step = {
                let v0 = m914;
                let m912 = v0;
                match m912 {
                    Adt17::C0(_, _, _) => {
                        Err::<Adt17, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt17::C1(v2, v3, r913) => {
                        let v4 = (*r913).clone();
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
                Ok(next) => { m914 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f122(v0: Adt17, v1: List<(Bytes, Bytes)>, v2: Bytes) -> R<Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>> {
    let m916 = v0;
    match m916 {
        Adt17::C0(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt17::C2(_) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt17::C3(r927, r928) => {
            let v3 = (*r927).clone();
            let v4 = (*r928).clone();
            let m917 = f122(v4, v1.clone(), v2.clone())?;
            match m917 {
                Ok(v6) => {
                    let m918 = v3;
                    match m918 {
                        Adt17::C0(v7, r925, r926) => {
                            let v8 = (*r925).clone();
                            let v9 = (*r926).clone();
                            let m919 = f118(v8.clone(), v1);
                            match m919 {
                                Ok(v11) => {
                                    let m920 = f119(v11.clone(), v7)?;
                                    match m920 {
                                        Ok(v13) => {
                                            if f120(v13, Str::lit("simpleType")) {
                                                let m921 = f121(v8, f8(Str::lit("name")));
                                                match m921 {
                                                    None => {
                                                        Ok(f189(f8(Str::lit("a top-level simpleType must have a name (XSD 1.1 Part 2, section 4.1.2)"))))
                                                    }
                                                    Some(v14) => {
                                                        let m922 = f190(v14.clone(), v9, v11, v2)?;
                                                        match m922 {
                                                            Ok(v16) => {
                                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::cons((f191(v14)?, (match v16.clone() {
                                                                    Adt22::C0(_, h923) => {
                                                                        h923
                                                                    }
                                                                }, match v16 {
                                                                    Adt22::C0(h924, _) => {
                                                                        h924
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
    let m929 = v0;
    match m929.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h930, _) = v1;
                h930
            }, f123(v2))
        }
    }
}

pub fn f124(p933: List<Bytes>) -> Option<Bytes> {
    let m932 = p933;
    {
        let mut m932 = m932;
        loop {
            let step = {
                let v0 = m932;
                let m931 = v0;
                match m931.uncons() {
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
                Ok(next) => { m932 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f125(p939: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<Bytes> {
    let m938 = p939;
    {
        let mut m938 = m938;
        loop {
            let step = {
                let v0 = m938;
                let m934 = v0;
                match m934.uncons() {
                    None => {
                        Err::<List<(Bytes, (List<Bytes>, Bytes))>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        let m937 = f192({
                            let (h935, _) = {
                                let (_, h936) = v2;
                                h936
                            };
                            h935
                        }, v1.clone());
                        match m937 {
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
                Ok(next) => { m938 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f126(p950: u64, p951: List<(Bytes, (List<Bytes>, Bytes))>, p952: List<Bytes>, p953: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m949 = (p950, (p951, (p952, p953)));
    {
        let mut m949 = m949;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m949;
                let m940 = v0;
                if m940 == 0 {
                    let m941 = v1;
                    match m941.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(f193(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                        }
                    }
                } else {
                    let v6 = m940 - 1;
                    let m942 = v1.clone();
                    match m942.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            let m943 = f160(v1, v2.clone());
                            match m943 {
                                None => {
                                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(f193(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                                }
                                Some(v9) => {
                                    Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>((v6, ({
                                        let (_, h944) = v9.clone();
                                        h944
                                    }, (List::cons({
                                        let (h945, _) = {
                                            let (h946, _) = v9.clone();
                                            h946
                                        };
                                        h945
                                    }, v2), List::cons({
                                        let (_, h947) = {
                                            let (h948, _) = v9;
                                            h948
                                        };
                                        h947
                                    }, v3)))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m949 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f127(v0: Bytes) -> R<List<(Bytes, Bytes)>> {
    f194(f195(v0)?, None::<Bytes>)
}

pub fn f128(p974: u64, v1: List<(Bytes, Option<Bytes>)>, v2: List<(Bytes, Bytes)>, p975: List<(Bytes, (Bytes, Bytes))>, p976: List<Adt19>, p977: List<Bytes>) -> R<Adt18> {
    let m973 = (p974, (p975, (p976, p977)));
    {
        let mut m973 = m973;
        loop {
            let step = {
                let (v0, (v3, (v4, v5))) = m973;
                let m954 = v0;
                if m954 == 0 {
                    Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>(Ok(Adt18::C1(f8(Str::lit("the schema documents import and include more documents than the bound authorities hold")))))
                } else {
                    let v6 = m954 - 1;
                    let m955 = v3;
                    match m955.uncons() {
                        None => {
                            let m956 = v5.clone();
                            match m956.uncons() {
                                None => {
                                    let m957 = f196(f197(v4, List::<Adt19>::nil()))?;
                                    match m957 {
                                        Ok(v8) => {
                                            Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>(Ok(Adt18::C2({
                                                let (h958, _) = v8;
                                                h958
                                            }, {
                                                let (_, h959) = v8;
                                                h959
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
                                let (h960, _) = v11.clone();
                                h960
                            };
                            if if f153(v13.clone(), f198(v4.clone())) {
                                true
                            } else {
                                f153(v13.clone(), v5.clone())
                            } {
                                Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>((v6, (v12, (v4, v5))))
                            } else {
                                let m961 = f199(v1.clone(), v13.clone());
                                match m961 {
                                    None => {
                                        Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>((v6, (v12, (v4, List::cons(v13, v5)))))
                                    }
                                    Some(v14) => {
                                        let m962 = v14;
                                        match m962 {
                                            None => {
                                                Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>(Ok(Adt18::C1(f200(f8(Str::lit("the schema document ")), v13, f8(Str::lit(" is not in the repository"))))))
                                            }
                                            Some(v15) => {
                                                let m963 = f201(v13.clone(), v15, f202(v2.clone(), v13))?;
                                                match m963 {
                                                    Ok(v17) => {
                                                        let m968 = f203(v17.clone(), {
                                                            let (h964, _) = {
                                                                let (_, h965) = v11.clone();
                                                                h965
                                                            };
                                                            h964
                                                        }, {
                                                            let (_, h966) = {
                                                                let (_, h967) = v11;
                                                                h967
                                                            };
                                                            h966
                                                        });
                                                        match m968 {
                                                            None => {
                                                                let m971 = f204(match v17.clone() {
                                                                    Adt19::C0(_, _, _, _, h969, _) => {
                                                                        h969
                                                                    }
                                                                }, match v17.clone() {
                                                                    Adt19::C0(_, _, _, h970, _, _) => {
                                                                        h970
                                                                    }
                                                                })?;
                                                                match m971 {
                                                                    Ok(v20) => {
                                                                        let m972 = f205(v20, v17.clone(), v2.clone())?;
                                                                        match m972 {
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
                Ok(next) => { m973 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f129(v0: List<List<(Bytes, Option<Bytes>)>>) -> List<(Bytes, Option<Bytes>)> {
    let m978 = v0;
    match m978.uncons() {
        None => {
            List::<(Bytes, Option<Bytes>)>::nil()
        }
        Some((v1, v2)) => {
            f207(v1, f129(v2))
        }
    }
}

pub fn f130(v0: Bytes) -> R<u64> {
    let (h979, _) = f168(Fn4::F208(v0.clone()), {
        let a981 = {
            let a980 = v0;
            length_bytes(a980)
        };
        let a982 = 1u64;
        nat_add(a981, a982)?
    }, 0u64)?;
    Ok(h979)
}

pub fn f131(v0: Bytes, v1: u64) -> R<u64> {
    let (h983, _) = f168(Fn4::F209(v0.clone()), {
        let a985 = {
            let a984 = v0;
            length_bytes(a984)
        };
        let a986 = 1u64;
        nat_add(a985, a986)?
    }, v1)?;
    Ok(h983)
}

pub fn f132(v0: Bytes, p995: u64, p996: u64) -> R<Result<u64, Bytes>> {
    let m994 = (p995, p996);
    {
        let mut m994 = m994;
        loop {
            let step = {
                let (v1, v2) = m994;
                let m987 = v2;
                if m987 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Ok::<u64, Bytes>(v1)))
                } else {
                    let v3 = m987 - 1;
                    let a989 = v1;
                    let a990 = {
                        let a988 = v0.clone();
                        length_bytes(a988)
                    };
                    if if nat_lt(a989, a990) {
                        if f114(v0.clone(), v1, Str::lit("...")) {
                            f210(v0.clone(), v1)?
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        let m993 = f211(v0.clone(), {
                            let a991 = v1;
                            let a992 = 3u64;
                            nat_add(a991, a992)?
                        })?;
                        match m993 {
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
                Ok(next) => { m994 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f133(v0: Bytes, v1: u64, v2: bool, v3: u64) -> R<Result<List<Adt2>, Bytes>> {
    let m997 = v3;
    if m997 == 0 {
        Ok(Err::<List<Adt2>, Bytes>(f111(Str::lit("the stream holds more documents than it is long"), v1)?))
    } else {
        let v4 = m997 - 1;
        let v5: u64 = {
            let a998 = v0.clone();
            length_bytes(a998)
        };
        let a999 = f212(v0.clone(), v1)?;
        let a1000 = 37u64;
        if nat_eq(a999, a1000) {
            Ok(Err::<List<Adt2>, Bytes>(f111(Str::lit("a directive is not read (YAML 1.2.2 [82])"), v1)?))
        } else {
            let m1007 = if if f114(v0.clone(), v1, Str::lit("---")) {
                f210(v0.clone(), v1)?
            } else {
                false
            } {
                f213(v0.clone(), {
                    let a1001 = v1;
                    let a1002 = 3u64;
                    nat_add(a1001, a1002)?
                }, 0u64, {
                    let a1003 = v5;
                    let a1004 = 2u64;
                    nat_add(a1003, a1004)?
                }, 1u64, false, 0u64, List::<Bytes>::nil())?
            } else {
                if v2 {
                    f213(v0.clone(), v1, 0u64, {
                        let a1005 = v5;
                        let a1006 = 2u64;
                        nat_add(a1005, a1006)?
                    }, 0u64, false, 0u64, List::<Bytes>::nil())?
                } else {
                    Adt20::C1(f111(Str::lit("only comments, a document end marker or a document start marker may follow a document (YAML 1.2.2 [207], [211])"), v1)?)
                }
            };
            match m1007 {
                Adt20::C0(v7, v8) => {
                    let v9: u64 = f131(v0.clone(), v8)?;
                    let m1010 = f132(v0.clone(), v9, {
                        let a1008 = v5;
                        let a1009 = 1u64;
                        nat_add(a1008, a1009)?
                    })?;
                    match m1010 {
                        Ok(v11) => {
                            let a1011 = v5;
                            let a1012 = v11;
                            if nat_le(a1011, a1012) {
                                Ok(Ok::<List<Adt2>, Bytes>(List::cons(v7, List::<Adt2>::nil())))
                            } else {
                                let m1015 = f133(v0, v11, f214({
                                    let a1013 = v11;
                                    let a1014 = v9;
                                    nat_eq(a1013, a1014)
                                }), v4)?;
                                match m1015 {
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
                Adt20::C1(v6) => {
                    Ok(Err::<List<Adt2>, Bytes>(v6))
                }
            }
        }
    }
}

pub fn f134(v0: Adt2) -> R<Bytes> {
    f215(v0)
}

pub fn f135(v0: Bytes, v1: u64, v2: u64, v3: Bytes) -> Bytes {
    let a1018 = v3;
    let a1019 = f43(v0, v1, {
        let a1016 = v2;
        let a1017 = v1;
        nat_sub(a1016, a1017)
    });
    append_bytes(a1018, a1019)
}

pub fn f136(v0: Bytes, p1032: u64, p1033: u64) -> R<u64> {
    let m1031 = (p1032, p1033);
    {
        let mut m1031 = m1031;
        loop {
            let step = {
                let (v1, v2) = m1031;
                let m1020 = v2;
                if m1020 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m1020 - 1;
                    let v4: u64 = f83(v0.clone(), v1)?;
                    let a1021 = v4;
                    let a1022 = 34u64;
                    if nat_eq(a1021, a1022) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a1023 = v4;
                        let a1024 = 92u64;
                        if nat_eq(a1023, a1024) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a1025 = v4;
                            let a1026 = 32u64;
                            if nat_lt(a1025, a1026) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a1027 = v4;
                                let a1028 = 256u64;
                                if nat_eq(a1027, a1028) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a1029 = v1;
                                        let a1030 = 1u64;
                                        nat_add(a1029, a1030)?
                                    }, v3))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1031 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f137(v0: Bytes, p1059: u64, p1060: u64, p1061: Bytes) -> R<Option<(u64, Bytes)>> {
    let m1058 = (p1059, (p1060, p1061));
    {
        let mut m1058 = m1058;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1058;
                let m1034 = v2;
                if m1034 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                } else {
                    let v4 = m1034 - 1;
                    let v5: u64 = f136(v0.clone(), v1, {
                        let a1038 = {
                            let a1036 = {
                                let a1035 = v0.clone();
                                length_bytes(a1035)
                            };
                            let a1037 = v1;
                            nat_sub(a1036, a1037)
                        };
                        let a1039 = 1u64;
                        nat_add(a1038, a1039)?
                    })?;
                    let v6: Bytes = {
                        let a1042 = v3;
                        let a1043 = f43(v0.clone(), v1, {
                            let a1040 = v5;
                            let a1041 = v1;
                            nat_sub(a1040, a1041)
                        });
                        append_bytes(a1042, a1043)
                    };
                    let v7: u64 = f83(v0.clone(), v5)?;
                    let a1044 = v7;
                    let a1045 = 34u64;
                    if nat_eq(a1044, a1045) {
                        Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(Some(({
                            let a1046 = v5;
                            let a1047 = 1u64;
                            nat_add(a1046, a1047)?
                        }, v6))))
                    } else {
                        let a1048 = v7;
                        let a1049 = 92u64;
                        if nat_eq(a1048, a1049) {
                            let m1050 = f216(v0.clone(), v5, v6)?;
                            match m1050 {
                                None => {
                                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                                }
                                Some(v8) => {
                                    let a1053 = {
                                        let (_, h1051) = {
                                            let (_, h1052) = v8.clone();
                                            h1052
                                        };
                                        h1051
                                    };
                                    let a1054 = 0u64;
                                    if nat_eq(a1053, a1054) {
                                        Ok::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(({
                                            let (h1055, _) = v8;
                                            h1055
                                        }, (v4, {
                                            let (h1056, _) = {
                                                let (_, h1057) = v8;
                                                h1057
                                            };
                                            h1056
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
                Ok(next) => { m1058 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f138(v0: Bytes, v1: u64) -> R<u64> {
    let (h1062, _) = f168(Fn4::F217(v0.clone()), {
        let a1064 = {
            let a1063 = v0;
            length_bytes(a1063)
        };
        let a1065 = 1u64;
        nat_add(a1064, a1065)?
    }, v1)?;
    Ok(h1062)
}

pub fn f139(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f140(v0: Bytes) -> R<u64> {
    let (h1066, _) = f168(Fn4::F218(v0.clone()), {
        let a1068 = {
            let a1067 = v0;
            length_bytes(a1067)
        };
        let a1069 = 1u64;
        nat_add(a1068, a1069)?
    }, 0u64)?;
    Ok(h1066)
}

pub fn f141(v0: Bytes) -> R<Option<u64>> {
    if f219(v0.clone())? {
        Ok(Some({
            let (_, h1070) = {
                let (h1071, _) = f220(Fn5::F221(v0.clone()), {
                    let a1072 = v0;
                    length_bytes(a1072)
                }, (0u64, 0u64))?;
                h1071
            };
            h1070
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f142(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f43(v1.clone(), 0u64, {
        let a1074 = {
            let a1073 = v1.clone();
            length_bytes(a1073)
        };
        let a1075 = f222(v1)?;
        nat_sub(a1074, a1075)
    });
    let v5: Bytes = {
        let a1076 = v0;
        let a1077 = v4.clone();
        append_bytes(a1076, a1077)
    };
    let v6: u64 = f140(v5.clone())?;
    let v7: Bytes = f43(v5.clone(), v6, {
        let a1079 = {
            let a1078 = v5;
            length_bytes(a1078)
        };
        let a1080 = v6;
        nat_sub(a1079, a1080)
    });
    let v8: u64 = {
        let a1081 = v4;
        length_bytes(a1081)
    };
    let v9: u64 = {
        let a1082 = v7.clone();
        length_bytes(a1082)
    };
    let a1083 = v9;
    let a1084 = 0u64;
    if nat_eq(a1083, a1084) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f139(v2, {
            let a1085 = v3;
            let a1086 = v8;
            nat_lt(a1085, a1086)
        }) {
            let v10: u64 = if v2 {
                let a1087 = v3;
                let a1088 = v8;
                nat_add(a1087, a1088)?
            } else {
                let a1089 = v8;
                let a1090 = v3;
                nat_sub(a1089, a1090)
            };
            let v11: u64 = f222(v7.clone())?;
            let a1091 = v10;
            let a1092 = v11;
            if nat_le(a1091, a1092) {
                Ok(Some((f43(v7, 0u64, {
                    let a1093 = v9;
                    let a1094 = v10;
                    nat_sub(a1093, a1094)
                }), 0u64)))
            } else {
                let a1097 = {
                    let a1095 = v10;
                    let a1096 = v11;
                    nat_sub(a1095, a1096)
                };
                let a1098 = 19u64;
                if nat_le(a1097, a1098) {
                    Ok(Some((f43(v7, 0u64, {
                        let a1099 = v9;
                        let a1100 = v11;
                        nat_sub(a1099, a1100)
                    }), {
                        let a1101 = v10;
                        let a1102 = v11;
                        nat_sub(a1101, a1102)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a1103 = v3;
                let a1104 = v8;
                nat_sub(a1103, a1104)
            };
            let a1107 = {
                let a1105 = v9;
                let a1106 = v12;
                nat_add(a1105, a1106)?
            };
            let a1108 = 19u64;
            if nat_le(a1107, a1108) {
                Ok(Some(({
                    let a1109 = v7;
                    let a1110 = f223(v12);
                    append_bytes(a1109, a1110)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f143(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1111 = 0u64;
        let a1112 = v1;
        if nat_lt(a1111, a1112) {
            let a1113 = v1;
            let a1114 = 9223372036854775808u64;
            nat_le(a1113, a1114)
        } else {
            false
        }
    } else {
        let a1115 = v1;
        let a1116 = 9223372036854775807u64;
        nat_le(a1115, a1116)
    }
}

pub fn f144(v0: Bytes, v1: u64) -> Option<u8> {
    let a1117 = v0;
    let a1118 = v1;
    index_bytes(a1117, a1118)
}

pub fn f145(v0: Bytes) -> R<Result<List<Adt21>, Bytes>> {
    f224({
        let a1120 = {
            let a1119 = v0.clone();
            length_bytes(a1119)
        };
        let a1121 = 1u64;
        nat_add(a1120, a1121)?
    }, v0, 0u64, List::<Adt21>::nil())
}

pub fn f146(p1129: u64, p1130: List<Adt21>, p1131: List<Adt13>) -> R<Result<List<Adt13>, Bytes>> {
    let m1128 = (p1129, (p1130, p1131));
    {
        let mut m1128 = m1128;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1128;
                let m1122 = v0;
                if m1122 == 0 {
                    Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>(Ok(Err::<List<Adt13>, Bytes>(f8(Str::lit("the input has more definitions than tokens")))))
                } else {
                    let v3 = m1122 - 1;
                    let m1123 = v1.clone();
                    match m1123.uncons() {
                        None => {
                            Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>(Ok(Ok::<List<Adt13>, Bytes>(f225(v2, List::<Adt13>::nil()))))
                        }
                        Some((_, _)) => {
                            let m1124 = f226(v1)?;
                            match m1124 {
                                Ok(v7) => {
                                    let m1125 = f227(v3, v7)?;
                                    match m1125 {
                                        Ok(v9) => {
                                            Ok::<(u64, (List<Adt21>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>((v3, ({
                                                let (_, h1126) = v9;
                                                h1126
                                            }, List::cons({
                                                let (h1127, _) = v9;
                                                h1127
                                            }, v2))))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>(Ok(Err::<List<Adt13>, Bytes>(v8)))
                                        }
                                    }
                                }
                                Err(v6) => {
                                    Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>(Ok(Err::<List<Adt13>, Bytes>(v6)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1128 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f147(v0: List<Adt16>, v1: List<Adt16>) -> List<Adt16> {
    let m1132 = v0;
    match m1132.uncons() {
        None => {
            List::<Adt16>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = if f46(f155(v2.clone()), Bytes::lit(&[])) {
                f228(f157(v2.clone()), v1.clone())
            } else {
                f155(v2.clone())
            };
            List::cons(Adt16::C0(f158(v2.clone()), v4, f159(v2.clone()), f157(v2)), f147(v3, v1))
        }
    }
}

pub fn f148(v0: List<Adt16>) -> R<u64> {
    let m1133 = v0;
    match m1133.uncons() {
        None => {
            Ok(0u64)
        }
        Some((v1, v2)) => {
            if f46(f155(v1), Bytes::lit(&[])) {
                f148(v2)
            } else {
                let a1134 = f148(v2)?;
                let a1135 = 1u64;
                nat_add(a1134, a1135)
            }
        }
    }
}

pub fn f149(v0: u64) -> bool {
    let a1136 = v0;
    let a1137 = 0u64;
    if nat_eq(a1136, a1137) {
        true
    } else {
        let a1138 = v0;
        let a1139 = 2u64;
        if nat_eq(a1138, a1139) {
            true
        } else {
            let a1140 = v0;
            let a1141 = 3u64;
            if nat_eq(a1140, a1141) {
                true
            } else {
                let a1142 = v0;
                let a1143 = 4u64;
                if nat_eq(a1142, a1143) {
                    true
                } else {
                    let a1144 = v0;
                    let a1145 = 9u64;
                    nat_eq(a1144, a1145)
                }
            }
        }
    }
}

pub fn f150(v0: Adt13) -> u64 {
    let m1146 = v0;
    match m1146 {
        Adt13::C0(v1, _, _, _, _, _) => {
            v1
        }
    }
}

pub fn f151(v0: List<Adt13>, v1: Adt13) -> R<Result<(Bytes, List<Bytes>), Bytes>> {
    let v2: u64 = f150(v1.clone());
    let v3: Bytes = f229(f152(v1.clone()))?;
    let a1147 = v2;
    let a1148 = 0u64;
    if if nat_eq(a1147, a1148) {
        true
    } else {
        let a1149 = v2;
        let a1150 = 2u64;
        nat_eq(a1149, a1150)
    } {
        let m1151 = f230(v0.clone(), v0.clone(), f152(v1.clone()));
        match m1151 {
            None => {
                let m1155 = f231({
                    let a1153 = {
                        let a1152 = v0.clone();
                        length_list(a1152)
                    };
                    let a1154 = 1u64;
                    nat_add(a1153, a1154)?
                }, v0, v1)?;
                match m1155 {
                    Ok(v5) => {
                        let a1157 = {
                            let a1156 = v5.clone();
                            length_list(a1156)
                        };
                        let a1158 = 0u64;
                        if nat_eq(a1157, a1158) {
                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f232(v3), List::<Bytes>::nil())))
                        } else {
                            let m1159 = f233(f234(f235(v5.clone()))?, v5)?;
                            match m1159 {
                                Ok(v7) => {
                                    Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f236(v3, {
                                        let (h1160, _) = v7;
                                        h1160
                                    }), {
                                        let (_, h1161) = v7;
                                        h1161
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
        let a1162 = v2;
        let a1163 = 3u64;
        if nat_eq(a1162, a1163) {
            let m1164 = f124(f237(v1.clone()));
            match m1164 {
                None => {
                    let m1165 = f238(f234(f237(v1.clone()))?, f239(f237(v1)), List::<Bytes>::nil())?;
                    match m1165 {
                        Ok(v10) => {
                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f240(v3, v10), List::<Bytes>::nil())))
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
            let a1166 = v2;
            let a1167 = 4u64;
            if nat_eq(a1166, a1167) {
                let v12: Adt15 = f241(v1);
                if !f46(f242(v12.clone()), Bytes::lit(&[])) {
                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f242(v12)))
                } else {
                    let a1168 = f243(v12.clone());
                    let a1169 = 1u64;
                    if nat_eq(a1168, a1169) {
                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f240(v3, List::cons(f244(v12.clone()), List::<Bytes>::nil())), f245(v12))))
                    } else {
                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f246(v3, f244(v12.clone())), f245(v12))))
                    }
                }
            } else {
                Ok(Err::<(Bytes, List<Bytes>), Bytes>(f7(f237(v1))))
            }
        }
    }
}

pub fn f152(v0: Adt13) -> Bytes {
    let m1170 = v0;
    match m1170 {
        Adt13::C0(_, v2, _, _, _, _) => {
            v2
        }
    }
}

pub fn f153(v0: Bytes, p1173: List<Bytes>) -> bool {
    let m1172 = p1173;
    {
        let mut m1172 = m1172;
        loop {
            let step = {
                let v1 = m1172;
                let m1171 = v1;
                match m1171.uncons() {
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
                Ok(next) => { m1172 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f154(p1176: List<Adt16>, v1: Bytes) -> Option<Adt16> {
    let m1175 = p1176;
    {
        let mut m1175 = m1175;
        loop {
            let step = {
                let v0 = m1175;
                let m1174 = v0;
                match m1174.uncons() {
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
                Ok(next) => { m1175 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f155(v0: Adt16) -> Bytes {
    let m1177 = v0;
    match m1177 {
        Adt16::C0(_, v2, _, _) => {
            v2
        }
    }
}

pub fn f156(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m1178 = v0;
    match m1178.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f156(v3, v1))
        }
    }
}

pub fn f157(v0: Adt16) -> List<Bytes> {
    let m1179 = v0;
    match m1179 {
        Adt16::C0(_, _, _, v4) => {
            v4
        }
    }
}

pub fn f158(v0: Adt16) -> Bytes {
    let m1180 = v0;
    match m1180 {
        Adt16::C0(v1, _, _, _) => {
            v1
        }
    }
}

pub fn f159(v0: Adt16) -> Bytes {
    let m1181 = v0;
    match m1181 {
        Adt16::C0(_, _, v3, _) => {
            v3
        }
    }
}

pub fn f160(v0: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)> {
    let m1182 = v0;
    match m1182.uncons() {
        None => {
            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
        }
        Some((v2, v3)) => {
            let m1185 = f192({
                let (h1183, _) = {
                    let (_, h1184) = v2.clone();
                    h1184
                };
                h1183
            }, v1.clone());
            match m1185 {
                None => {
                    Some((({
                        let (h1186, _) = v2;
                        h1186
                    }, {
                        let (_, h1187) = {
                            let (_, h1188) = v2;
                            h1188
                        };
                        h1187
                    }), v3))
                }
                Some(_) => {
                    let m1189 = f160(v3, v1);
                    match m1189 {
                        None => {
                            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h1190, _) = v5;
                                h1190
                            }, List::cons(v2, {
                                let (_, h1191) = v5;
                                h1191
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f161(p1196: List<Adt13>, v1: u64, v2: Bytes) -> Option<Adt13> {
    let m1195 = p1196;
    {
        let mut m1195 = m1195;
        loop {
            let step = {
                let v0 = m1195;
                let m1192 = v0;
                match m1192.uncons() {
                    None => {
                        Err::<List<Adt13>, Option<Adt13>>(None::<Adt13>)
                    }
                    Some((v3, v4)) => {
                        let a1193 = f150(v3.clone());
                        let a1194 = v1;
                        if if nat_eq(a1193, a1194) {
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
                Ok(next) => { m1195 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f162(v0: u64) -> Bytes {
    let a1197 = v0;
    let a1198 = 5u64;
    if nat_eq(a1197, a1198) {
        f8(Str::lit("partial interface "))
    } else {
        let a1199 = v0;
        let a1200 = 7u64;
        if nat_eq(a1199, a1200) {
            f8(Str::lit("partial dictionary "))
        } else {
            f8(Str::lit(""))
        }
    }
}

pub fn f163(v0: Adt13) -> Bytes {
    let m1201 = v0;
    match m1201 {
        Adt13::C0(_, _, v3, _, _, _) => {
            v3
        }
    }
}

pub fn f164(v0: Bytes, v1: u64) -> Bytes {
    let m1205 = {
        let a1202 = v0;
        let a1203 = v1;
        let a1204 = 1u64;
        slice_bytes(a1202, a1203, a1204)
    };
    match m1205 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f165(v0: Fn3, p1210: u64, p1211: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1209 = (p1210, p1211);
    {
        let mut m1209 = m1209;
        loop {
            let step = {
                let (v1, v2) = m1209;
                let m1206 = v1;
                if m1206 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1206 - 1;
                    let m1208 = {
                        let c1207: Fn3 = v0.clone();
                        c1207.apply(v2.clone())?
                    };
                    match m1208 {
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
                Ok(next) => { m1209 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f166(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f247(v1, v0, v2)
}

pub fn f167(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f248(v0, v1)
}

pub fn f168(v0: Fn4, p1216: u64, p1217: u64) -> R<(u64, bool)> {
    let m1215 = (p1216, p1217);
    {
        let mut m1215 = m1215;
        loop {
            let step = {
                let (v1, v2) = m1215;
                let m1212 = v1;
                if m1212 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1212 - 1;
                    let m1214 = {
                        let c1213: Fn4 = v0.clone();
                        c1213.apply(v2)?
                    };
                    match m1214 {
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
                Ok(next) => { m1215 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f169(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f249(v0, v1)
}

pub fn f170(v0: u64) -> R<bool> {
    f250(v0, f251())
}

pub fn f171(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let v3: u64 = {
        let (h1218, _) = f168(Fn4::F252(v1.clone(), v0.clone()), {
            let a1220 = {
                let a1219 = v0.clone();
                length_bytes(a1219)
            };
            let a1221 = 1u64;
            nat_add(a1220, a1221)?
        }, v2)?;
        h1218
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
    let (h1222, _) = f168(Fn4::F253(v0.clone()), {
        let a1224 = {
            let a1223 = v0;
            length_bytes(a1223)
        };
        let a1225 = 1u64;
        nat_add(a1224, a1225)?
    }, v1)?;
    Ok(h1222)
}

pub fn f174(v0: Bytes) -> bool {
    f254(v0, List::cons(f8(Str::lit("UTF-8")), List::cons(f8(Str::lit("UTf-8")), List::cons(f8(Str::lit("UtF-8")), List::cons(f8(Str::lit("Utf-8")), List::cons(f8(Str::lit("uTF-8")), List::cons(f8(Str::lit("uTf-8")), List::cons(f8(Str::lit("utF-8")), List::cons(f8(Str::lit("utf-8")), List::<Bytes>::nil())))))))))
}

pub fn f175(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let m1229 = f171(v0.clone(), {
        let a1226 = Str::lit("--");
        utf8_encode(a1226)
    }, {
        let a1227 = v1;
        let a1228 = 4u64;
        nat_add(a1227, a1228)?
    })?;
    match m1229 {
        None => {
            Ok(f172(f111(Str::lit("a comment is not closed (XML 1.0 [15])"), v1)?))
        }
        Some(v2) => {
            let a1232 = f115(v0, {
                let a1230 = v2;
                let a1231 = 2u64;
                nat_add(a1230, a1231)?
            })?;
            let a1233 = 62u64;
            if nat_eq(a1232, a1233) {
                Ok(Ok::<u64, Bytes>({
                    let a1234 = v2;
                    let a1235 = 3u64;
                    nat_add(a1234, a1235)?
                }))
            } else {
                Ok(f172(f111(Str::lit("-- is not allowed inside a comment (XML 1.0 [15])"), v2)?))
            }
        }
    }
}

pub fn f176(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f178(v0.clone(), {
        let a1236 = v1;
        let a1237 = 2u64;
        nat_add(a1236, a1237)?
    })?;
    let a1240 = v2;
    let a1241 = {
        let a1238 = v1;
        let a1239 = 2u64;
        nat_add(a1238, a1239)?
    };
    if nat_eq(a1240, a1241) {
        Ok(f172(f111(Str::lit("a processing instruction must start with a Name (XML 1.0 [16])"), v1)?))
    } else {
        let a1244 = v2;
        let a1245 = {
            let a1242 = v1;
            let a1243 = 5u64;
            nat_add(a1242, a1243)?
        };
        if if nat_eq(a1244, a1245) {
            f254(f43(v0.clone(), {
                let a1246 = v1;
                let a1247 = 2u64;
                nat_add(a1246, a1247)?
            }, 3u64), List::cons(f8(Str::lit("XML")), List::cons(f8(Str::lit("XMl")), List::cons(f8(Str::lit("XmL")), List::cons(f8(Str::lit("Xml")), List::cons(f8(Str::lit("xML")), List::cons(f8(Str::lit("xMl")), List::cons(f8(Str::lit("xmL")), List::cons(f8(Str::lit("xml")), List::<Bytes>::nil())))))))))
        } else {
            false
        } {
            Ok(f172(f111(Str::lit("the target xml is reserved; an XML declaration may appear only at the very start (XML 1.0 [17], [22])"), v1)?))
        } else {
            let m1249 = f171(v0, {
                let a1248 = Str::lit("?>");
                utf8_encode(a1248)
            }, v2)?;
            match m1249 {
                None => {
                    Ok(f172(f111(Str::lit("a processing instruction is not closed (XML 1.0 [16])"), v1)?))
                }
                Some(v3) => {
                    Ok(Ok::<u64, Bytes>({
                        let a1250 = v3;
                        let a1251 = 2u64;
                        nat_add(a1250, a1251)?
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
    let (h1252, _) = f168(Fn4::F255(v0.clone(), v1), {
        let a1254 = {
            let a1253 = v0;
            length_bytes(a1253)
        };
        let a1255 = 1u64;
        nat_add(a1254, a1255)?
    }, v1)?;
    Ok(h1252)
}

pub fn f179(v0: Bytes, v1: u64, v2: u64, v3: List<Bytes>) -> R<Result<(Adt17, u64), Bytes>> {
    let m1256 = v2;
    if m1256 == 0 {
        Ok(f177(f111(Str::lit("a tag has more attributes than the input has bytes"), v1)?))
    } else {
        let v4 = m1256 - 1;
        let v5: u64 = f173(v0.clone(), v1)?;
        let a1257 = f115(v0.clone(), v5)?;
        let a1258 = 62u64;
        if if nat_eq(a1257, a1258) {
            true
        } else {
            f114(v0.clone(), v5, Str::lit("/>"))
        } {
            Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C4, v5)))
        } else {
            let a1259 = v5;
            let a1260 = v1;
            if nat_eq(a1259, a1260) {
                Ok(f177(f111(Str::lit("attributes must be separated by white space (XML 1.0 [40])"), v1)?))
            } else {
                let v6: u64 = f178(v0.clone(), v5)?;
                let a1261 = v6;
                let a1262 = v5;
                if nat_eq(a1261, a1262) {
                    Ok(f177(f111(Str::lit("an attribute must start with a Name (XML 1.0 [41])"), v5)?))
                } else {
                    let v7: Bytes = f43(v0.clone(), v5, {
                        let a1263 = v6;
                        let a1264 = v5;
                        nat_sub(a1263, a1264)
                    });
                    if f254(v7.clone(), v3.clone()) {
                        Ok(f177(f111(Str::lit("an attribute name appears twice in one tag (XML 1.0 WFC: Unique Att Spec)"), v5)?))
                    } else {
                        let v8: u64 = f173(v0.clone(), v6)?;
                        let a1265 = f115(v0.clone(), v8)?;
                        let a1266 = 61u64;
                        if nat_eq(a1265, a1266) {
                            let v9: u64 = f173(v0.clone(), {
                                let a1267 = v8;
                                let a1268 = 1u64;
                                nat_add(a1267, a1268)?
                            })?;
                            let v10: u64 = f115(v0.clone(), v9)?;
                            let a1269 = v10;
                            let a1270 = 34u64;
                            if if nat_eq(a1269, a1270) {
                                true
                            } else {
                                let a1271 = v10;
                                let a1272 = 39u64;
                                nat_eq(a1271, a1272)
                            } {
                                let m1275 = f180(v0.clone(), {
                                    let a1273 = v9;
                                    let a1274 = 1u64;
                                    nat_add(a1273, a1274)?
                                }, v4, Bytes::lit(&[]), true, v10)?;
                                match m1275 {
                                    Ok(v12) => {
                                        let m1277 = f179(v0, {
                                            let (_, h1276) = v12;
                                            h1276
                                        }, v4, List::cons(v7.clone(), v3))?;
                                        match m1277 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C1(v7, {
                                                    let (h1278, _) = v12;
                                                    h1278
                                                }, std::rc::Rc::new({
                                                    let (h1279, _) = v14;
                                                    h1279
                                                })), {
                                                    let (_, h1280) = v14;
                                                    h1280
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

pub fn f180(v0: Bytes, p1320: u64, p1321: u64, p1322: Bytes, v4: bool, v5: u64) -> R<Result<(Bytes, u64), Bytes>> {
    let m1319 = (p1320, (p1321, p1322));
    {
        let mut m1319 = m1319;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1319;
                let m1281 = v2;
                if m1281 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f256(f111(Str::lit("the input ends inside a text run"), v1)?)))
                } else {
                    let v6 = m1281 - 1;
                    let v7: u64 = f115(v0.clone(), v1)?;
                    if if v4 {
                        let a1282 = v7;
                        let a1283 = v5;
                        nat_eq(a1282, a1283)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, {
                            let a1284 = v1;
                            let a1285 = 1u64;
                            nat_add(a1284, a1285)?
                        }))))
                    } else {
                        let a1286 = v7;
                        let a1287 = 256u64;
                        if nat_eq(a1286, a1287) {
                            if v4 {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f256(f111(Str::lit("an attribute value is not closed (XML 1.0 [10])"), v1)?)))
                            } else {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                            }
                        } else {
                            let a1288 = v7;
                            let a1289 = 60u64;
                            if nat_eq(a1288, a1289) {
                                if v4 {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f256(f111(Str::lit("< is not allowed in an attribute value (XML 1.0 [10], WFC: No < in Attribute Values)"), v1)?)))
                                } else {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                                }
                            } else {
                                let a1290 = v7;
                                let a1291 = 38u64;
                                if nat_eq(a1290, a1291) {
                                    let m1292 = f257(v0.clone(), v1)?;
                                    match m1292 {
                                        Ok(v9) => {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let (_, h1293) = v9;
                                                h1293
                                            }, (v6, {
                                                let a1295 = v3;
                                                let a1296 = {
                                                    let (h1294, _) = v9;
                                                    h1294
                                                };
                                                append_bytes(a1295, a1296)
                                            })))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f256(v8)))
                                        }
                                    }
                                } else {
                                    if v4 {
                                        let a1297 = v7;
                                        let a1298 = 9u64;
                                        if if nat_eq(a1297, a1298) {
                                            true
                                        } else {
                                            let a1299 = v7;
                                            let a1300 = 10u64;
                                            nat_eq(a1299, a1300)
                                        } {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let a1301 = v1;
                                                let a1302 = 1u64;
                                                nat_add(a1301, a1302)?
                                            }, (v6, {
                                                let a1303 = v3;
                                                let a1304 = f8(Str::lit(" "));
                                                append_bytes(a1303, a1304)
                                            })))
                                        } else {
                                            let v10: u64 = f258(v0.clone(), true, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v10, (v6, {
                                                let a1307 = v3;
                                                let a1308 = f43(v0.clone(), v1, {
                                                    let a1305 = v10;
                                                    let a1306 = v1;
                                                    nat_sub(a1305, a1306)
                                                });
                                                append_bytes(a1307, a1308)
                                            })))
                                        }
                                    } else {
                                        let a1309 = v7;
                                        let a1310 = 93u64;
                                        if nat_eq(a1309, a1310) {
                                            if f114(v0.clone(), v1, Str::lit("]]>")) {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f256(f111(Str::lit("]]> is not allowed in character data (XML 1.0 [14])"), v1)?)))
                                            } else {
                                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                    let a1311 = v1;
                                                    let a1312 = 1u64;
                                                    nat_add(a1311, a1312)?
                                                }, (v6, {
                                                    let a1313 = v3;
                                                    let a1314 = f8(Str::lit("]"));
                                                    append_bytes(a1313, a1314)
                                                })))
                                            }
                                        } else {
                                            let v11: u64 = f258(v0.clone(), false, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v11, (v6, {
                                                let a1317 = v3;
                                                let a1318 = f43(v0.clone(), v1, {
                                                    let a1315 = v11;
                                                    let a1316 = v1;
                                                    nat_sub(a1315, a1316)
                                                });
                                                append_bytes(a1317, a1318)
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
                Ok(next) => { m1319 = next; }
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
    let m1327 = {
        let a1324 = v0;
        let a1325 = 0u64;
        let a1326 = {
            let a1323 = v1.clone();
            length_bytes(a1323)
        };
        slice_bytes(a1324, a1325, a1326)
    };
    match m1327 {
        None => {
            false
        }
        Some(v2) => {
            let a1328 = v2;
            let a1329 = v1;
            equal(a1328, a1329)
        }
    }
}

pub fn f185(v0: Bytes) -> R<Bytes> {
    let m1330 = f171(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1330 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v1) => {
            Ok(f43(v0, 0u64, v1))
        }
    }
}

pub fn f186(p1335: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1334 = p1335;
    {
        let mut m1334 = m1334;
        loop {
            let step = {
                let v0 = m1334;
                let m1331 = v0;
                match m1331.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f46({
                            let (h1332, _) = v2;
                            h1332
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h1333) = v2;
                                h1333
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1334 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f187(v0: Bytes) -> R<Bytes> {
    let m1336 = f171(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1336 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(f43(v0.clone(), {
                let a1337 = v1;
                let a1338 = 1u64;
                nat_add(a1337, a1338)?
            }, {
                let a1342 = {
                    let a1339 = v0;
                    length_bytes(a1339)
                };
                let a1343 = {
                    let a1340 = v1;
                    let a1341 = 1u64;
                    nat_add(a1340, a1341)?
                };
                nat_sub(a1342, a1343)
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

pub fn f190(v0: Bytes, v1: Adt17, v2: List<(Bytes, Bytes)>, v3: Bytes) -> R<Result<Adt22, Bytes>> {
    let v4: Bytes = f191(v0.clone())?;
    let m1344 = f259(v1, v2)?;
    match m1344 {
        Ok(v6) => {
            let m1345 = v6;
            match m1345 {
                None => {
                    Ok(f260(f261(f8(Str::lit("a simpleType must have a restriction, list or union child (XSD 1.1 Part 2, section 4.1.2)")), v0)))
                }
                Some(v7) => {
                    let v8: Bytes = match v7.clone() {
                        Adt27::C0(h1346, _, _, _) => {
                            h1346
                        }
                    };
                    let v9: Adt17 = match v7.clone() {
                        Adt27::C0(_, h1347, _, _) => {
                            h1347
                        }
                    };
                    let v10: Adt17 = match v7.clone() {
                        Adt27::C0(_, _, h1348, _) => {
                            h1348
                        }
                    };
                    let v11: List<(Bytes, Bytes)> = match v7 {
                        Adt27::C0(_, _, _, h1349) => {
                            h1349
                        }
                    };
                    if f46(v8.clone(), f8(Str::lit("union"))) {
                        Ok(f260(f261(f8(Str::lit("a union is not translated, because a literal can be valid for more than one member type, so no LexLean inductive keeps its members disjoint (XSD 1.1 Part 2, section 2.4.1.3)")), v0)))
                    } else {
                        if f46(v8, f8(Str::lit("list"))) {
                            let m1350 = f121(v9, f8(Str::lit("itemType")));
                            match m1350 {
                                None => {
                                    Ok(f260(f261(f8(Str::lit("a list with an anonymous item type is not translated yet")), v0)))
                                }
                                Some(v12) => {
                                    let m1351 = f262(v11, v3, v12)?;
                                    match m1351 {
                                        Ok(v14) => {
                                            if f184(match v14.clone() {
                                                Adt22::C0(h1352, _) => {
                                                    h1352
                                                }
                                            }, f8(Str::lit("(List "))) {
                                                Ok(f260(f261(f8(Str::lit("the item type of a list must not itself be a list (XSD 1.1 Part 2, section 4.1.5)")), v0)))
                                            } else {
                                                Ok(Ok::<Adt22, Bytes>(Adt22::C0(f246(v4, f7(List::cons(f8(Str::lit("(List ")), List::cons(match v14.clone() {
                                                    Adt22::C0(h1353, _) => {
                                                        h1353
                                                    }
                                                }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))), match v14 {
                                                    Adt22::C0(_, h1354) => {
                                                        h1354
                                                    }
                                                })))
                                            }
                                        }
                                        Err(v13) => {
                                            Ok(f260(f261(v13, v0)))
                                        }
                                    }
                                }
                            }
                        } else {
                            let m1355 = f121(v9, f8(Str::lit("base")));
                            match m1355 {
                                None => {
                                    Ok(f260(f261(f8(Str::lit("a restriction with an anonymous base type is not translated yet")), v0)))
                                }
                                Some(v15) => {
                                    let m1356 = f262(v11.clone(), v3, v15)?;
                                    match m1356 {
                                        Ok(v17) => {
                                            let m1357 = f263(v10, v11)?;
                                            match m1357 {
                                                Ok(v19) => {
                                                    let v20: List<Adt2> = f264(v19, List::<Bytes>::nil())?;
                                                    let m1358 = v20.clone();
                                                    match m1358.uncons() {
                                                        None => {
                                                            Ok(Ok::<Adt22, Bytes>(Adt22::C0(f246(v4, match v17.clone() {
                                                                Adt22::C0(h1359, _) => {
                                                                    h1359
                                                                }
                                                            }), match v17 {
                                                                Adt22::C0(_, h1360) => {
                                                                    h1360
                                                                }
                                                            })))
                                                        }
                                                        Some((_, _)) => {
                                                            let m1361 = f238(f234(f265(v20.clone()))?, v20, List::<Bytes>::nil())?;
                                                            match m1361 {
                                                                Ok(v24) => {
                                                                    Ok(Ok::<Adt22, Bytes>(Adt22::C0(f240(v4, v24), List::<Bytes>::nil())))
                                                                }
                                                                Err(v23) => {
                                                                    Ok(f260(f261(v23, v0)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Err(v18) => {
                                                    Ok(f260(f261(v18, v0)))
                                                }
                                            }
                                        }
                                        Err(v16) => {
                                            Ok(f260(f261(v16, v0)))
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
            Ok(f260(f261(v5, v0)))
        }
    }
}

pub fn f191(v0: Bytes) -> R<Bytes> {
    f266(v0.clone(), 0u64, {
        let a1362 = v0;
        length_bytes(a1362)
    }, true, Bytes::lit(&[]))
}

pub fn f192(p1365: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m1364 = p1365;
    {
        let mut m1364 = m1364;
        loop {
            let step = {
                let v0 = m1364;
                let m1363 = v0;
                match m1363.uncons() {
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
                Ok(next) => { m1364 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f193(v0: Bytes) -> Result<List<Bytes>, Bytes> {
    Err::<List<Bytes>, Bytes>(v0)
}

pub fn f194(v0: List<Bytes>, v1: Option<Bytes>) -> R<List<(Bytes, Bytes)>> {
    let m1366 = v0;
    match m1366.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, v3)) => {
            if f46(v2.clone(), f8(Str::lit("[[authority]]"))) {
                f194(v3, None::<Bytes>)
            } else {
                let m1367 = f267(v2.clone(), f8(Str::lit("url = ")))?;
                match m1367 {
                    None => {
                        let m1368 = f267(v2, f8(Str::lit("artifact = ")))?;
                        match m1368 {
                            None => {
                                f194(v3, v1)
                            }
                            Some(v5) => {
                                let m1369 = v1.clone();
                                match m1369 {
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
        let a1370 = v0.clone();
        length_bytes(a1370)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h1371, _) = f56(Fn0::F268(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h1371
    };
    let v3: u64 = {
        let (h1372, _) = {
            let (_, h1373) = v2.clone();
            h1373
        };
        h1372
    };
    let v4: List<Bytes> = {
        let (_, h1374) = {
            let (_, h1375) = v2;
            h1375
        };
        h1374
    };
    let a1376 = v3;
    let a1377 = v1;
    if nat_lt(a1376, a1377) {
        Ok(f58(List::cons(f43(v0, v3, {
            let a1378 = v1;
            let a1379 = v3;
            nat_sub(a1378, a1379)
        }), v4)))
    } else {
        Ok(f58(v4))
    }
}

pub fn f196(v0: List<Adt19>) -> R<Result<(u64, Bytes), Bytes>> {
    let m1380 = f269(v0.clone())?;
    match m1380 {
        Ok(v2) => {
            let v3: Adt23 = Adt23::C0(v2.clone(), f270(v0.clone()));
            let m1381 = f271(f272(v0)?, v3, v2)?;
            match m1381 {
                Ok(v5) => {
                    let m1382 = f273(f274(v5), List::<Adt26>::nil());
                    match m1382 {
                        Ok(v7) => {
                            let v8: List<Bytes> = f275(v7.clone());
                            let m1383 = f276(v7.clone(), v8.clone());
                            match m1383 {
                                None => {
                                    let v10: List<(Bytes, List<Bytes>)> = f277(v7.clone(), v8.clone(), f278(v7.clone())?);
                                    let m1389 = f279({
                                        let a1387 = {
                                            let a1386 = v8.clone();
                                            length_list(a1386)
                                        };
                                        let a1388 = 1u64;
                                        nat_add(a1387, a1388)?
                                    }, v7.clone(), f280(v7, f281(v10, v8.clone(), v8, List::<Bytes>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil());
                                    match m1389 {
                                        Ok(v12) => {
                                            Ok(Ok::<(u64, Bytes), Bytes>(({
                                                let a1390 = v12.clone();
                                                length_list(a1390)
                                            }, f103(v12, f8(Str::lit("\u{a}"))))))
                                        }
                                        Err(v11) => {
                                            Ok(Err::<(u64, Bytes), Bytes>(v11))
                                        }
                                    }
                                }
                                Some(v9) => {
                                    Ok(Err::<(u64, Bytes), Bytes>(f200(f8(Str::lit("the type ")), {
                                        let (h1384, _) = v9;
                                        h1384
                                    }, f200(f8(Str::lit(" uses ")), {
                                        let (_, h1385) = v9;
                                        h1385
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

pub fn f197(p1393: List<Adt19>, p1394: List<Adt19>) -> List<Adt19> {
    let m1392 = (p1393, p1394);
    {
        let mut m1392 = m1392;
        loop {
            let step = {
                let (v0, v1) = m1392;
                let m1391 = v0;
                match m1391.uncons() {
                    None => {
                        Err::<(List<Adt19>, List<Adt19>), List<Adt19>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt19>, List<Adt19>), List<Adt19>>((v3, List::cons(v2, v1)))
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

pub fn f198(v0: List<Adt19>) -> List<Bytes> {
    let m1395 = v0;
    match m1395.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt19::C0(h1396, _, _, _, _, _) => {
                    h1396
                }
            }, f198(v2))
        }
    }
}

pub fn f199(p1401: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m1400 = p1401;
    {
        let mut m1400 = m1400;
        loop {
            let step = {
                let v0 = m1400;
                let m1397 = v0;
                match m1397.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f46({
                            let (h1398, _) = v2;
                            h1398
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h1399) = v2;
                                h1399
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1400 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f200(v0: Bytes, v1: Bytes, v2: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(v1, List::cons(v2, List::<Bytes>::nil()))))
}

pub fn f201(v0: Bytes, v1: Bytes, v2: Option<Bytes>) -> R<Result<Adt19, Bytes>> {
    let m1402 = f66(v1.clone())?;
    match m1402 {
        Ok(v4) => {
            let m1403 = v4;
            match m1403 {
                Adt17::C0(v5, r1408, r1409) => {
                    let v6 = (*r1408).clone();
                    let v7 = (*r1409).clone();
                    let m1404 = f118(v6.clone(), List::<(Bytes, Bytes)>::nil());
                    match m1404 {
                        Ok(v9) => {
                            let m1405 = f119(v9.clone(), v5)?;
                            match m1405 {
                                Ok(v11) => {
                                    if f120(v11, Str::lit("schema")) {
                                        Ok(Ok::<Adt19, Bytes>(Adt19::C0(v0, v2, {
                                            let m1406 = f121(v6, f8(Str::lit("targetNamespace")));
                                            match m1406 {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some(v12) => {
                                                    v12
                                                }
                                            }
                                        }, v9, v7, {
                                            let a1407 = v1;
                                            length_bytes(a1407)
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

pub fn f202(p1414: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1413 = p1414;
    {
        let mut m1413 = m1413;
        loop {
            let step = {
                let v0 = m1413;
                let m1410 = v0;
                match m1410.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f46({
                            let (_, h1411) = v2;
                            h1411
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (h1412, _) = v2;
                                h1412
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
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

pub fn f203(v0: Adt19, v1: Bytes, v2: Bytes) -> Option<Bytes> {
    if f46(v1.clone(), f8(Str::lit("root"))) {
        None::<Bytes>
    } else {
        if f46(match v0.clone() {
            Adt19::C0(_, _, h1415, _, _, _) => {
                h1415
            }
        }, v2.clone()) {
            None::<Bytes>
        } else {
            Some(f7(List::cons(match v0.clone() {
                Adt19::C0(h1416, _, _, _, _, _) => {
                    h1416
                }
            }, List::cons(f8(Str::lit(" has the target namespace ")), List::cons(match v0 {
                Adt19::C0(_, _, h1417, _, _, _) => {
                    h1417
                }
            }, List::cons(f8(Str::lit(", but the xs:")), List::cons(v1.clone(), List::cons(f8(Str::lit(" that names it requires ")), List::cons(v2, List::cons(if f46(v1, f8(Str::lit("include"))) {
                f8(Str::lit("; an included document without a target namespace is not adopted into the including one (XSD 1.1 Part 1, section 4.2.3)"))
            } else {
                f8(Str::lit(" (XSD 1.1 Part 1, section 4.2.6.2)"))
            }, List::<Bytes>::nil()))))))))))
        }
    }
}

pub fn f204(v0: Adt17, v1: List<(Bytes, Bytes)>) -> R<Result<List<Adt25>, Bytes>> {
    let m1418 = v0;
    match m1418 {
        Adt17::C0(_, _, _) => {
            Ok(Ok::<List<Adt25>, Bytes>(List::<Adt25>::nil()))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<List<Adt25>, Bytes>(List::<Adt25>::nil()))
        }
        Adt17::C2(_) => {
            Ok(Ok::<List<Adt25>, Bytes>(List::<Adt25>::nil()))
        }
        Adt17::C3(r1429, r1430) => {
            let v2 = (*r1429).clone();
            let v3 = (*r1430).clone();
            let m1419 = f204(v3, v1.clone())?;
            match m1419 {
                Ok(v5) => {
                    let m1420 = v2;
                    match m1420 {
                        Adt17::C0(v6, r1427, r1428) => {
                            let v7 = (*r1427).clone();
                            let v8 = (*r1428).clone();
                            let m1421 = f118(v7.clone(), v1);
                            match m1421 {
                                Ok(v10) => {
                                    let m1422 = f119(v10.clone(), v6)?;
                                    match m1422 {
                                        Ok(v12) => {
                                            if if f46({
                                                let (h1423, _) = v12.clone();
                                                h1423
                                            }, f188()) {
                                                f282(f46({
                                                    let (_, h1424) = v12.clone();
                                                    h1424
                                                }, f8(Str::lit("annotation"))))
                                            } else {
                                                false
                                            } {
                                                Ok(Ok::<List<Adt25>, Bytes>(List::cons(Adt25::C0({
                                                    let (h1425, _) = v12;
                                                    h1425
                                                }, {
                                                    let (_, h1426) = v12;
                                                    h1426
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
                        Adt17::C1(_, _, _) => {
                            Ok(Ok::<List<Adt25>, Bytes>(v5))
                        }
                        Adt17::C2(_) => {
                            Ok(Ok::<List<Adt25>, Bytes>(v5))
                        }
                        Adt17::C3(_, _) => {
                            Ok(Ok::<List<Adt25>, Bytes>(v5))
                        }
                        Adt17::C4 => {
                            Ok(Ok::<List<Adt25>, Bytes>(v5))
                        }
                    }
                }
                Err(v4) => {
                    Ok(Err::<List<Adt25>, Bytes>(v4))
                }
            }
        }
        Adt17::C4 => {
            Ok(Ok::<List<Adt25>, Bytes>(List::<Adt25>::nil()))
        }
    }
}

pub fn f205(v0: List<Adt25>, v1: Adt19, v2: List<(Bytes, Bytes)>) -> R<Result<List<(Bytes, (Bytes, Bytes))>, Bytes>> {
    let m1431 = v0;
    match m1431.uncons() {
        None => {
            Ok(Ok::<List<(Bytes, (Bytes, Bytes))>, Bytes>(List::<(Bytes, (Bytes, Bytes))>::nil()))
        }
        Some((v3, v4)) => {
            let m1432 = f205(v4, v1.clone(), v2.clone())?;
            match m1432 {
                Ok(v6) => {
                    if if f283(v3.clone(), Str::lit("redefine")) {
                        true
                    } else {
                        if f283(v3.clone(), Str::lit("override")) {
                            true
                        } else {
                            f283(v3.clone(), Str::lit("defaultOpenContent"))
                        }
                    } {
                        Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                            Adt19::C0(h1433, _, _, _, _, _) => {
                                h1433
                            }
                        }, List::cons(f8(Str::lit(": xs:")), List::cons(match v3 {
                            Adt25::C0(_, h1434, _, _, _) => {
                                h1434
                            }
                        }, List::cons(f8(Str::lit(" is not translated, because it changes components that other schema documents define (XSD 1.1 Part 1, sections 4.2.4, 4.2.5 and 3.4.2.3)")), List::<Bytes>::nil())))))))
                    } else {
                        if if f283(v3.clone(), Str::lit("import")) {
                            true
                        } else {
                            f283(v3.clone(), Str::lit("include"))
                        } {
                            let m1435 = f284(v3.clone(), Str::lit("schemaLocation"));
                            match m1435 {
                                None => {
                                    Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                                        Adt19::C0(h1436, _, _, _, _, _) => {
                                            h1436
                                        }
                                    }, List::cons(f8(Str::lit(": an xs:")), List::cons(match v3 {
                                        Adt25::C0(_, h1437, _, _, _) => {
                                            h1437
                                        }
                                    }, List::cons(f8(Str::lit(" with no schemaLocation names no document whose components could be read (XSD 1.1 Part 1, section 4.2.6.2)")), List::<Bytes>::nil())))))))
                                }
                                Some(v7) => {
                                    let m1439 = f285(match v1.clone() {
                                        Adt19::C0(_, h1438, _, _, _, _) => {
                                            h1438
                                        }
                                    }, v7)?;
                                    match m1439 {
                                        Ok(v9) => {
                                            let m1441 = f286(v2, v9.clone());
                                            match m1441 {
                                                None => {
                                                    Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                                                        Adt19::C0(h1442, _, _, _, _, _) => {
                                                            h1442
                                                        }
                                                    }, List::cons(f8(Str::lit(": the schema document ")), List::cons(v9, List::cons(f8(Str::lit(" is not a bound authority, so its components cannot be read; stage its binding first (XSD 1.1 Part 1, section 4.2.6.2)")), List::<Bytes>::nil())))))))
                                                }
                                                Some(v10) => {
                                                    Ok(Ok::<List<(Bytes, (Bytes, Bytes))>, Bytes>(List::cons((v10, (match v3.clone() {
                                                        Adt25::C0(_, h1443, _, _, _) => {
                                                            h1443
                                                        }
                                                    }, if f283(v3.clone(), Str::lit("include")) {
                                                        match v1 {
                                                            Adt19::C0(_, _, h1444, _, _, _) => {
                                                                h1444
                                                            }
                                                        }
                                                    } else {
                                                        f287(v3, Str::lit("namespace"), Str::lit(""))
                                                    })), v6)))
                                                }
                                            }
                                        }
                                        Err(v8) => {
                                            Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f200(match v1 {
                                                Adt19::C0(h1440, _, _, _, _, _) => {
                                                    h1440
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
    let m1445 = v0;
    match m1445.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f206(v3, v1))
        }
    }
}

pub fn f207(v0: List<(Bytes, Option<Bytes>)>, v1: List<(Bytes, Option<Bytes>)>) -> List<(Bytes, Option<Bytes>)> {
    let m1446 = v0;
    match m1446.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f207(v3, v1))
        }
    }
}

pub fn f208(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f288(v0, v1)
}

pub fn f209(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f289(v0, v1)
}

pub fn f210(v0: Bytes, v1: u64) -> R<bool> {
    if if if f114(v0.clone(), v1, Str::lit("---")) {
        true
    } else {
        f114(v0.clone(), v1, Str::lit("..."))
    } {
        if f290(f212(v0.clone(), {
            let a1447 = v1;
            let a1448 = 3u64;
            nat_add(a1447, a1448)?
        })?) {
            true
        } else {
            f291(f212(v0.clone(), {
                let a1449 = v1;
                let a1450 = 3u64;
                nat_add(a1449, a1450)?
            })?)
        }
    } else {
        false
    } {
        let a1451 = f292(v0, v1)?;
        let a1452 = 0u64;
        Ok(nat_eq(a1451, a1452))
    } else {
        Ok(false)
    }
}

pub fn f211(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f293(v0.clone(), v1)?;
    let v3: u64 = f212(v0.clone(), v2)?;
    if if f291(v3) {
        true
    } else {
        let a1453 = v3;
        let a1454 = 35u64;
        if nat_eq(a1453, a1454) {
            let a1455 = v1;
            let a1456 = v2;
            nat_lt(a1455, a1456)
        } else {
            false
        }
    } {
        Ok(Ok::<u64, Bytes>(f294(v0, v2)?))
    } else {
        Ok(Err::<u64, Bytes>(f111(Str::lit("only white space and a comment may follow a node on its line (YAML 1.2.2 [77])"), v2)?))
    }
}

pub fn f212(v0: Bytes, v1: u64) -> R<u64> {
    f83(v0, v1)
}

pub fn f213(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: List<Bytes>) -> R<Adt20> {
    let m1457 = v3;
    if m1457 == 0 {
        Ok(Adt20::C1(f111(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v8 = m1457 - 1;
        let a1458 = v4;
        let a1459 = 0u64;
        if nat_eq(a1458, a1459) {
            let v9: u64 = f295(v0.clone(), v1)?;
            let v10: u64 = {
                let a1460 = v9;
                let a1461 = v1;
                nat_sub(a1460, a1461)
            };
            let a1462 = f212(v0.clone(), v9)?;
            let a1463 = 9u64;
            if nat_eq(a1462, a1463) {
                let v11: u64 = f293(v0.clone(), v9)?;
                if if f296(v0.clone(), v11)? {
                    true
                } else {
                    f297(v0.clone(), v11)?
                } {
                    Ok(Adt20::C1(f111(Str::lit("a tab cannot indent a block node (YAML 1.2.2 [63])"), v11)?))
                } else {
                    let m1464 = f298(v0.clone(), v11)?;
                    match m1464 {
                        None => {
                            f299(v0, v11, v2)
                        }
                        Some(v12) => {
                            Ok(Adt20::C1(v12))
                        }
                    }
                }
            } else {
                if f296(v0.clone(), v9)? {
                    Ok(f300(true, f213(v0, v9, v2, v8, 2u64, false, v10, List::<Bytes>::nil())?))
                } else {
                    let m1465 = f298(v0.clone(), v9)?;
                    match m1465 {
                        None => {
                            if f297(v0.clone(), v9)? {
                                Ok(f300(false, f213(v0, v9, v2, v8, 3u64, false, v10, List::<Bytes>::nil())?))
                            } else {
                                f299(v0, v9, v2)
                            }
                        }
                        Some(v13) => {
                            Ok(Adt20::C1(v13))
                        }
                    }
                }
            }
        } else {
            let a1466 = v4;
            let a1467 = 1u64;
            if nat_eq(a1466, a1467) {
                let v14: u64 = f293(v0.clone(), v1)?;
                let v15: u64 = f212(v0.clone(), v14)?;
                if if f291(v15) {
                    true
                } else {
                    let a1468 = v15;
                    let a1469 = 35u64;
                    if nat_eq(a1468, a1469) {
                        let a1470 = v1;
                        let a1471 = v14;
                        nat_lt(a1470, a1471)
                    } else {
                        false
                    }
                } {
                    let v16: u64 = f131(v0.clone(), f294(v0.clone(), v14)?)?;
                    let a1473 = {
                        let a1472 = v0.clone();
                        length_bytes(a1472)
                    };
                    let a1474 = v16;
                    if if nat_le(a1473, a1474) {
                        true
                    } else {
                        f210(v0.clone(), v16)?
                    } {
                        Ok(Adt20::C0(Adt2::C0, v16))
                    } else {
                        let v17: u64 = {
                            let a1475 = f295(v0.clone(), v16)?;
                            let a1476 = v16;
                            nat_sub(a1475, a1476)
                        };
                        let a1477 = v2;
                        let a1478 = v17;
                        if nat_le(a1477, a1478) {
                            f213(v0, v16, v2, v8, 0u64, false, 0u64, List::<Bytes>::nil())
                        } else {
                            if if if f214(v5) {
                                let a1479 = v17;
                                let a1480 = v6;
                                nat_eq(a1479, a1480)
                            } else {
                                false
                            } {
                                f296(v0.clone(), {
                                    let a1481 = v16;
                                    let a1482 = v17;
                                    nat_add(a1481, a1482)?
                                })?
                            } else {
                                false
                            } {
                                Ok(f300(true, f213(v0, {
                                    let a1483 = v16;
                                    let a1484 = v17;
                                    nat_add(a1483, a1484)?
                                }, v2, v8, 2u64, false, v17, List::<Bytes>::nil())?))
                            } else {
                                Ok(Adt20::C0(Adt2::C0, v16))
                            }
                        }
                    }
                } else {
                    if if v5 {
                        f296(v0.clone(), v14)?
                    } else {
                        false
                    } {
                        let a1487 = f301(f302(v0.clone(), v1, v14), 9u64)?;
                        let a1488 = {
                            let a1485 = v14;
                            let a1486 = v1;
                            nat_sub(a1485, a1486)
                        };
                        if nat_lt(a1487, a1488) {
                            Ok(Adt20::C1(f111(Str::lit("a tab cannot indent a block node (YAML 1.2.2 [63])"), v14)?))
                        } else {
                            Ok(f300(true, f213(v0.clone(), v14, v2, v8, 2u64, false, f292(v0, v14)?, List::<Bytes>::nil())?))
                        }
                    } else {
                        let m1489 = f298(v0.clone(), v14)?;
                        match m1489 {
                            None => {
                                if if v5 {
                                    f297(v0.clone(), v14)?
                                } else {
                                    false
                                } {
                                    Ok(f300(false, f213(v0.clone(), v14, v2, v8, 3u64, false, f292(v0, v14)?, List::<Bytes>::nil())?))
                                } else {
                                    f299(v0, v14, v2)
                                }
                            }
                            Some(v18) => {
                                Ok(Adt20::C1(v18))
                            }
                        }
                    }
                }
            } else {
                let a1490 = v4;
                let a1491 = 2u64;
                if nat_eq(a1490, a1491) {
                    let m1496 = f213(v0.clone(), {
                        let a1492 = v1;
                        let a1493 = 1u64;
                        nat_add(a1492, a1493)?
                    }, {
                        let a1494 = v6;
                        let a1495 = 1u64;
                        nat_add(a1494, a1495)?
                    }, v8, 1u64, true, v6, List::<Bytes>::nil())?;
                    match m1496 {
                        Adt20::C0(v20, v21) => {
                            let v22: u64 = f131(v0.clone(), v21)?;
                            let v23: u64 = {
                                let a1497 = f295(v0.clone(), v22)?;
                                let a1498 = v22;
                                nat_sub(a1497, a1498)
                            };
                            let a1500 = {
                                let a1499 = v0.clone();
                                length_bytes(a1499)
                            };
                            let a1501 = v22;
                            if nat_le(a1500, a1501) {
                                Ok(Adt20::C0(Adt2::C9(std::rc::Rc::new(v20), std::rc::Rc::new(Adt2::C8)), v22))
                            } else {
                                let a1502 = v23;
                                let a1503 = v6;
                                if if nat_eq(a1502, a1503) {
                                    f296(v0.clone(), {
                                        let a1504 = v22;
                                        let a1505 = v23;
                                        nat_add(a1504, a1505)?
                                    })?
                                } else {
                                    false
                                } {
                                    let m1508 = f213(v0, {
                                        let a1506 = v22;
                                        let a1507 = v23;
                                        nat_add(a1506, a1507)?
                                    }, v2, v8, 2u64, false, v6, List::<Bytes>::nil())?;
                                    match m1508 {
                                        Adt20::C0(v25, v26) => {
                                            Ok(Adt20::C0(Adt2::C9(std::rc::Rc::new(v20), std::rc::Rc::new(v25)), v26))
                                        }
                                        Adt20::C1(v24) => {
                                            Ok(Adt20::C1(v24))
                                        }
                                    }
                                } else {
                                    let a1509 = v6;
                                    let a1510 = v23;
                                    if nat_lt(a1509, a1510) {
                                        Ok(Adt20::C1(f111(Str::lit("a line is indented more than its block sequence entry allows (YAML 1.2.2 [183])"), {
                                            let a1511 = v22;
                                            let a1512 = v23;
                                            nat_add(a1511, a1512)?
                                        })?))
                                    } else {
                                        Ok(Adt20::C0(Adt2::C9(std::rc::Rc::new(v20), std::rc::Rc::new(Adt2::C8)), v22))
                                    }
                                }
                            }
                        }
                        Adt20::C1(v19) => {
                            Ok(Adt20::C1(v19))
                        }
                    }
                } else {
                    let m1513 = f298(v0.clone(), v1)?;
                    match m1513 {
                        None => {
                            let m1514 = f303(v0.clone(), v1)?;
                            match m1514 {
                                None => {
                                    Ok(Adt20::C1(f111(Str::lit("a block mapping entry must be an implicit key and : (YAML 1.2.2 [188])"), v1)?))
                                }
                                Some(v28) => {
                                    let v29: Bytes = f304(v0.clone(), v1)?;
                                    if f254(v29.clone(), v7.clone()) {
                                        Ok(Adt20::C1(f111(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                    } else {
                                        let m1519 = f213(v0.clone(), {
                                            let a1515 = v28;
                                            let a1516 = 1u64;
                                            nat_add(a1515, a1516)?
                                        }, {
                                            let a1517 = v6;
                                            let a1518 = 1u64;
                                            nat_add(a1517, a1518)?
                                        }, v8, 1u64, false, v6, List::<Bytes>::nil())?;
                                        match m1519 {
                                            Adt20::C0(v31, v32) => {
                                                let v33: u64 = f131(v0.clone(), v32)?;
                                                let v34: u64 = {
                                                    let a1520 = f295(v0.clone(), v33)?;
                                                    let a1521 = v33;
                                                    nat_sub(a1520, a1521)
                                                };
                                                let a1523 = {
                                                    let a1522 = v0.clone();
                                                    length_bytes(a1522)
                                                };
                                                let a1524 = v33;
                                                if nat_le(a1523, a1524) {
                                                    Ok(Adt20::C0(Adt2::C10(v29, std::rc::Rc::new(v31), std::rc::Rc::new(Adt2::C8)), v33))
                                                } else {
                                                    let a1525 = v34;
                                                    let a1526 = v6;
                                                    if if if nat_eq(a1525, a1526) {
                                                        f214(f296(v0.clone(), {
                                                            let a1527 = v33;
                                                            let a1528 = v34;
                                                            nat_add(a1527, a1528)?
                                                        })?)
                                                    } else {
                                                        false
                                                    } {
                                                        f214(f210(v0.clone(), v33)?)
                                                    } else {
                                                        false
                                                    } {
                                                        let m1531 = f213(v0, {
                                                            let a1529 = v33;
                                                            let a1530 = v34;
                                                            nat_add(a1529, a1530)?
                                                        }, v2, v8, 3u64, false, v6, List::cons(v29.clone(), v7))?;
                                                        match m1531 {
                                                            Adt20::C0(v36, v37) => {
                                                                Ok(Adt20::C0(Adt2::C10(v29, std::rc::Rc::new(v31), std::rc::Rc::new(v36)), v37))
                                                            }
                                                            Adt20::C1(v35) => {
                                                                Ok(Adt20::C1(v35))
                                                            }
                                                        }
                                                    } else {
                                                        let a1532 = v6;
                                                        let a1533 = v34;
                                                        if nat_lt(a1532, a1533) {
                                                            Ok(Adt20::C1(f111(Str::lit("a line is indented more than its block mapping entry allows (YAML 1.2.2 [187])"), {
                                                                let a1534 = v33;
                                                                let a1535 = v34;
                                                                nat_add(a1534, a1535)?
                                                            })?))
                                                        } else {
                                                            Ok(Adt20::C0(Adt2::C10(v29, std::rc::Rc::new(v31), std::rc::Rc::new(Adt2::C8)), v33))
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

pub fn f214(v0: bool) -> bool {
    !v0
}

pub fn f215(v0: Adt2) -> R<Bytes> {
    let m1536 = v0;
    match m1536 {
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
            f305(v4, v5)
        }
        Adt2::C4(v6, v7, v8) => {
            f306(v6, v7, v8)
        }
        Adt2::C5(v9) => {
            f307(v9)
        }
        Adt2::C6(r1537) => {
            let v10 = (*r1537).clone();
            Ok(f308(f309(v10)?))
        }
        Adt2::C7(r1538) => {
            let v11 = (*r1538).clone();
            f310(f311(v11)?)
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

pub fn f216(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f83(v0.clone(), {
        let a1539 = v1;
        let a1540 = 1u64;
        nat_add(a1539, a1540)?
    })?;
    let v4: Bytes = {
        let a1541 = v3;
        let a1542 = 117u64;
        if nat_eq(a1541, a1542) {
            f312(v0, v1)?
        } else {
            f313(v3)
        }
    };
    let a1543 = v4.clone();
    let a1544 = Bytes::lit(&[]);
    if equal(a1543, a1544) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a1545 = v3;
            let a1546 = 117u64;
            if nat_eq(a1545, a1546) {
                let a1547 = v1;
                let a1548 = 6u64;
                nat_add(a1547, a1548)?
            } else {
                let a1549 = v1;
                let a1550 = 2u64;
                nat_add(a1549, a1550)?
            }
        }, ({
            let a1551 = v2;
            let a1552 = v4;
            append_bytes(a1551, a1552)
        }, 0u64))))
    }
}

pub fn f217(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f314(v0, v1)
}

pub fn f218(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f315(v0, v1)
}

pub fn f219(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1553 = v0.clone();
        length_bytes(a1553)
    };
    let a1554 = 0u64;
    let a1555 = v1;
    if nat_lt(a1554, a1555) {
        let a1556 = f316(v0.clone())?;
        let a1557 = v1;
        if nat_eq(a1556, a1557) {
            let a1558 = 1u64;
            let a1559 = v1;
            if nat_lt(a1558, a1559) {
                let m1562 = {
                    let a1560 = v0;
                    let a1561 = 0u64;
                    index_bytes(a1560, a1561)
                };
                match m1562 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1563 = v2;
                        let a1564 = 48u8;
                        Ok(!equal(a1563, a1564))
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

pub fn f220(v0: Fn5, p1569: u64, p1570: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1568 = (p1569, p1570);
    {
        let mut m1568 = m1568;
        loop {
            let step = {
                let (v1, v2) = m1568;
                let m1565 = v1;
                if m1565 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1565 - 1;
                    let m1567 = {
                        let c1566: Fn5 = v0.clone();
                        c1566.apply(v2)?
                    };
                    match m1567 {
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
                Ok(next) => { m1568 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f221(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f317(v0, v1)
}

pub fn f222(v0: Bytes) -> R<u64> {
    let (h1571, _) = f168(Fn4::F318(v0.clone()), {
        let a1573 = {
            let a1572 = v0;
            length_bytes(a1572)
        };
        let a1574 = 1u64;
        nat_add(a1573, a1574)?
    }, 0u64)?;
    Ok(h1571)
}

pub fn f223(v0: u64) -> Bytes {
    let m1575 = v0;
    if m1575 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1575 - 1;
        let a1576 = f8(Str::lit("0"));
        let a1577 = f223(v1);
        append_bytes(a1576, a1577)
    }
}

pub fn f224(p1632: u64, v1: Bytes, p1633: u64, p1634: List<Adt21>) -> R<Result<List<Adt21>, Bytes>> {
    let m1631 = (p1632, (p1633, p1634));
    {
        let mut m1631 = m1631;
        loop {
            let step = {
                let (v0, (v2, v3)) = m1631;
                let m1578 = v0;
                if m1578 == 0 {
                    Err::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>(Ok(Err::<List<Adt21>, Bytes>(f319(Str::lit("the input has more tokens than bytes"), v2)?)))
                } else {
                    let v4 = m1578 - 1;
                    let v5: u64 = f115(v1.clone(), v2)?;
                    let a1579 = v5;
                    let a1580 = 256u64;
                    if nat_eq(a1579, a1580) {
                        Err::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>(Ok(Ok::<List<Adt21>, Bytes>(f320(v3, List::<Adt21>::nil()))))
                    } else {
                        if f170(v5)? {
                            Ok::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>((v4, ({
                                let a1581 = v2;
                                let a1582 = 1u64;
                                nat_add(a1581, a1582)?
                            }, v3)))
                        } else {
                            if f114(v1.clone(), v2, Str::lit("//")) {
                                let m1583 = f171(v1.clone(), Bytes::lit(&[0x0a]), v2)?;
                                match m1583 {
                                    None => {
                                        Err::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>(Ok(Ok::<List<Adt21>, Bytes>(f320(v3, List::<Adt21>::nil()))))
                                    }
                                    Some(v6) => {
                                        Ok::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>((v4, ({
                                            let a1584 = v6;
                                            let a1585 = 1u64;
                                            nat_add(a1584, a1585)?
                                        }, v3)))
                                    }
                                }
                            } else {
                                if f114(v1.clone(), v2, Str::lit("/*")) {
                                    let m1589 = f171(v1.clone(), {
                                        let a1586 = Str::lit("*/");
                                        utf8_encode(a1586)
                                    }, {
                                        let a1587 = v2;
                                        let a1588 = 2u64;
                                        nat_add(a1587, a1588)?
                                    })?;
                                    match m1589 {
                                        None => {
                                            Err::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>(Ok(Err::<List<Adt21>, Bytes>(f319(Str::lit("a comment is not closed (Web IDL, IDL grammar: comment)"), v2)?)))
                                        }
                                        Some(v7) => {
                                            Ok::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>((v4, ({
                                                let a1590 = v7;
                                                let a1591 = 2u64;
                                                nat_add(a1590, a1591)?
                                            }, v3)))
                                        }
                                    }
                                } else {
                                    let a1592 = v5;
                                    let a1593 = 34u64;
                                    if nat_eq(a1592, a1593) {
                                        let m1596 = f171(v1.clone(), Bytes::lit(&[0x22]), {
                                            let a1594 = v2;
                                            let a1595 = 1u64;
                                            nat_add(a1594, a1595)?
                                        })?;
                                        match m1596 {
                                            None => {
                                                Err::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>(Ok(Err::<List<Adt21>, Bytes>(f319(Str::lit("a string is not closed (Web IDL, IDL grammar: string)"), v2)?)))
                                            }
                                            Some(v8) => {
                                                Ok::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>((v4, ({
                                                    let a1597 = v8;
                                                    let a1598 = 1u64;
                                                    nat_add(a1597, a1598)?
                                                }, List::cons(Adt21::C0(3u64, f43(v1.clone(), {
                                                    let a1599 = v2;
                                                    let a1600 = 1u64;
                                                    nat_add(a1599, a1600)?
                                                }, {
                                                    let a1603 = v8;
                                                    let a1604 = {
                                                        let a1601 = v2;
                                                        let a1602 = 1u64;
                                                        nat_add(a1601, a1602)?
                                                    };
                                                    nat_sub(a1603, a1604)
                                                })), v3))))
                                            }
                                        }
                                    } else {
                                        let v9: u64 = {
                                            let a1605 = v5;
                                            let a1606 = 95u64;
                                            if if nat_eq(a1605, a1606) {
                                                true
                                            } else {
                                                let a1607 = v5;
                                                let a1608 = 45u64;
                                                nat_eq(a1607, a1608)
                                            } {
                                                let a1609 = v2;
                                                let a1610 = 1u64;
                                                nat_add(a1609, a1610)?
                                            } else {
                                                v2
                                            }
                                        };
                                        if f321(f115(v1.clone(), v9)?) {
                                            let v10: u64 = f322(v1.clone(), 3u64, {
                                                let a1611 = v9;
                                                let a1612 = 1u64;
                                                nat_add(a1611, a1612)?
                                            })?;
                                            Ok::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>((v4, (v10, List::cons(Adt21::C0(0u64, f43(v1.clone(), v2, {
                                                let a1613 = v10;
                                                let a1614 = v2;
                                                nat_sub(a1613, a1614)
                                            })), v3))))
                                        } else {
                                            let v11: u64 = {
                                                let a1615 = v5;
                                                let a1616 = 45u64;
                                                if nat_eq(a1615, a1616) {
                                                    let a1617 = v2;
                                                    let a1618 = 1u64;
                                                    nat_add(a1617, a1618)?
                                                } else {
                                                    v2
                                                }
                                            };
                                            let v12: u64 = f323(v1.clone(), v11)?;
                                            let v13: u64 = f324(v1.clone(), v11)?;
                                            let a1619 = v12;
                                            let a1620 = v13;
                                            if nat_lt(a1619, a1620) {
                                                Ok::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>((v4, (v13, List::cons(Adt21::C0(2u64, f43(v1.clone(), v2, {
                                                    let a1621 = v13;
                                                    let a1622 = v2;
                                                    nat_sub(a1621, a1622)
                                                })), v3))))
                                            } else {
                                                let a1623 = v11;
                                                let a1624 = v12;
                                                if nat_lt(a1623, a1624) {
                                                    Ok::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>((v4, (v12, List::cons(Adt21::C0(1u64, f43(v1.clone(), v2, {
                                                        let a1625 = v12;
                                                        let a1626 = v2;
                                                        nat_sub(a1625, a1626)
                                                    })), v3))))
                                                } else {
                                                    if f114(v1.clone(), v2, Str::lit("...")) {
                                                        Ok::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>((v4, ({
                                                            let a1627 = v2;
                                                            let a1628 = 3u64;
                                                            nat_add(a1627, a1628)?
                                                        }, List::cons(Adt21::C0(4u64, f8(Str::lit("..."))), v3))))
                                                    } else {
                                                        Ok::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>((v4, ({
                                                            let a1629 = v2;
                                                            let a1630 = 1u64;
                                                            nat_add(a1629, a1630)?
                                                        }, List::cons(Adt21::C0(4u64, f43(v1.clone(), v2, 1u64)), v3))))
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
                Ok(next) => { m1631 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f225(p1637: List<Adt13>, p1638: List<Adt13>) -> List<Adt13> {
    let m1636 = (p1637, p1638);
    {
        let mut m1636 = m1636;
        loop {
            let step = {
                let (v0, v1) = m1636;
                let m1635 = v0;
                match m1635.uncons() {
                    None => {
                        Err::<(List<Adt13>, List<Adt13>), List<Adt13>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt13>, List<Adt13>), List<Adt13>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1636 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f226(v0: List<Adt21>) -> R<Result<List<Adt21>, Bytes>> {
    if f325(v0.clone(), Str::lit("[")) {
        Ok(f326({
            let a1640 = {
                let a1639 = v0.clone();
                length_list(a1639)
            };
            let a1641 = 1u64;
            nat_add(a1640, a1641)?
        }, f327(v0), List::cons(f8(Str::lit("]")), List::<Bytes>::nil()), false))
    } else {
        Ok(Ok::<List<Adt21>, Bytes>(v0))
    }
}

pub fn f227(v0: u64, v1: List<Adt21>) -> R<Result<(Adt13, List<Adt21>), Bytes>> {
    if f325(v1.clone(), Str::lit("interface")) {
        if f325(f327(v1.clone()), Str::lit("mixin")) {
            f328(v0, 1u64, f327(f327(v1)), false)
        } else {
            f328(v0, 0u64, f327(v1), true)
        }
    } else {
        if f325(v1.clone(), Str::lit("partial")) {
            let v2: List<Adt21> = f327(v1);
            if f325(v2.clone(), Str::lit("interface")) {
                if f325(f327(v2.clone()), Str::lit("mixin")) {
                    f328(v0, 6u64, f327(f327(v2)), false)
                } else {
                    f328(v0, 5u64, f327(v2), false)
                }
            } else {
                if f325(v2.clone(), Str::lit("dictionary")) {
                    f328(v0, 7u64, f327(v2), false)
                } else {
                    if f325(v2.clone(), Str::lit("namespace")) {
                        f329(f327(v2), f8(Str::lit("a namespace holds operations and attributes of no object, so it is behavior, not data (Web IDL 2.6)")))
                    } else {
                        Ok(Err::<(Adt13, List<Adt21>), Bytes>(f330(Str::lit("interface, dictionary or namespace"), v2)))
                    }
                }
            }
        } else {
            if f325(v1.clone(), Str::lit("dictionary")) {
                f328(v0, 2u64, f327(v1), true)
            } else {
                if f325(v1.clone(), Str::lit("enum")) {
                    let m1642 = f331(f327(v1));
                    match m1642 {
                        Ok(v4) => {
                            let m1643 = f332(f333(v4.clone()), Str::lit("{"));
                            match m1643 {
                                Ok(v6) => {
                                    let m1644 = f334(v0, v6, List::<Bytes>::nil());
                                    match m1644 {
                                        Ok(v8) => {
                                            Ok(Ok::<(Adt13, List<Adt21>), Bytes>((Adt13::C0(3u64, f335(v4), Bytes::lit(&[]), List::<Adt14>::nil(), {
                                                let (h1645, _) = v8;
                                                h1645
                                            }, f336(Bytes::lit(&[]))), {
                                                let (_, h1646) = v8;
                                                h1646
                                            })))
                                        }
                                        Err(v7) => {
                                            Ok(Err::<(Adt13, List<Adt21>), Bytes>(v7))
                                        }
                                    }
                                }
                                Err(v5) => {
                                    Ok(Err::<(Adt13, List<Adt21>), Bytes>(v5))
                                }
                            }
                        }
                        Err(v3) => {
                            Ok(Err::<(Adt13, List<Adt21>), Bytes>(v3))
                        }
                    }
                } else {
                    if f325(v1.clone(), Str::lit("typedef")) {
                        let m1647 = f337(v0, f327(v1), 0u64)?;
                        match m1647 {
                            Ok(v10) => {
                                let m1648 = f331(f338(v10.clone()));
                                match m1648 {
                                    Ok(v12) => {
                                        let m1649 = f332(f333(v12.clone()), Str::lit(";"));
                                        match m1649 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt13, List<Adt21>), Bytes>((Adt13::C0(4u64, f335(v12), Bytes::lit(&[]), List::<Adt14>::nil(), List::<Bytes>::nil(), f339(v10)), v14)))
                                            }
                                            Err(v13) => {
                                                Ok(Err::<(Adt13, List<Adt21>), Bytes>(v13))
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(Err::<(Adt13, List<Adt21>), Bytes>(v11))
                                    }
                                }
                            }
                            Err(v9) => {
                                Ok(Err::<(Adt13, List<Adt21>), Bytes>(v9))
                            }
                        }
                    } else {
                        if f325(v1.clone(), Str::lit("callback")) {
                            if f325(f327(v1.clone()), Str::lit("interface")) {
                                f329(f327(f327(v1)), f8(Str::lit("a callback interface is implemented by user code, so it is behavior, not data (Web IDL 2.4)")))
                            } else {
                                f329(f327(v1), f8(Str::lit("a callback function is behavior, not data (Web IDL 2.10)")))
                            }
                        } else {
                            if f325(v1.clone(), Str::lit("namespace")) {
                                f329(f327(v1), f8(Str::lit("a namespace holds operations and attributes of no object, so it is behavior, not data (Web IDL 2.6)")))
                            } else {
                                let m1650 = f331(v1);
                                match m1650 {
                                    Ok(v16) => {
                                        let m1651 = f332(f333(v16.clone()), Str::lit("includes"));
                                        match m1651 {
                                            Ok(v18) => {
                                                let m1652 = f331(v18);
                                                match m1652 {
                                                    Ok(v20) => {
                                                        let m1653 = f332(f333(v20.clone()), Str::lit(";"));
                                                        match m1653 {
                                                            Ok(v22) => {
                                                                Ok(Ok::<(Adt13, List<Adt21>), Bytes>((Adt13::C0(8u64, f335(v16), f335(v20), List::<Adt14>::nil(), List::<Bytes>::nil(), f336(Bytes::lit(&[]))), v22)))
                                                            }
                                                            Err(v21) => {
                                                                Ok(Err::<(Adt13, List<Adt21>), Bytes>(v21))
                                                            }
                                                        }
                                                    }
                                                    Err(v19) => {
                                                        Ok(Err::<(Adt13, List<Adt21>), Bytes>(v19))
                                                    }
                                                }
                                            }
                                            Err(v17) => {
                                                Ok(Err::<(Adt13, List<Adt21>), Bytes>(v17))
                                            }
                                        }
                                    }
                                    Err(v15) => {
                                        Ok(Err::<(Adt13, List<Adt21>), Bytes>(v15))
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

pub fn f228(p1657: List<Bytes>, v1: List<Adt16>) -> Bytes {
    let m1656 = p1657;
    {
        let mut m1656 = m1656;
        loop {
            let step = {
                let v0 = m1656;
                let m1654 = v0;
                match m1654.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1655 = f154(v1.clone(), v2.clone());
                        match m1655 {
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
                Ok(next) => { m1656 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f229(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f191(v0)?;
    if f153(v1.clone(), f340()) {
        f341(v1)
    } else {
        Ok(v1)
    }
}

pub fn f230(p1663: List<Adt13>, v1: List<Adt13>, v2: Bytes) -> Option<Bytes> {
    let m1662 = p1663;
    {
        let mut m1662 = m1662;
        loop {
            let step = {
                let v0 = m1662;
                let m1658 = v0;
                match m1658.uncons() {
                    None => {
                        Err::<List<Adt13>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v3, v4)) => {
                        let a1659 = f150(v3.clone());
                        let a1660 = 8u64;
                        if if nat_eq(a1659, a1660) {
                            f46(f152(v3.clone()), v2.clone())
                        } else {
                            false
                        } {
                            let m1661 = f161(v1.clone(), 1u64, f163(v3.clone()));
                            match m1661 {
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
                Ok(next) => { m1662 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f231(v0: u64, v1: List<Adt13>, v2: Adt13) -> R<Result<List<Adt14>, Bytes>> {
    let m1664 = v0;
    if m1664 == 0 {
        Ok(Err::<List<Adt14>, Bytes>(f7(List::cons(f8(Str::lit("the inheritance of ")), List::cons(f152(v2), List::cons(f8(Str::lit(" is cyclic (Web IDL 2.2 and 2.7: an interface or dictionary must not inherit from itself)")), List::<Bytes>::nil()))))))
    } else {
        let v3 = m1664 - 1;
        let v4: List<Adt14> = {
            let a1665 = f150(v2.clone());
            let a1666 = 2u64;
            if nat_eq(a1665, a1666) {
                f342(f343(f344(v2.clone()), f345(v1.clone(), 7u64, f152(v2.clone()))))?
            } else {
                f343(f344(v2.clone()), f343(f345(v1.clone(), 5u64, f152(v2.clone())), f346(v1.clone(), v1.clone(), f152(v2.clone()))))
            }
        };
        if f46(f163(v2.clone()), Bytes::lit(&[])) {
            Ok(Ok::<List<Adt14>, Bytes>(v4))
        } else {
            let m1667 = f161(v1.clone(), f150(v2.clone()), f163(v2.clone()));
            match m1667 {
                None => {
                    Ok(Err::<List<Adt14>, Bytes>(f7(List::cons(f8(Str::lit("inherits ")), List::cons(f163(v2.clone()), List::cons({
                        let a1668 = f150(v2);
                        let a1669 = 2u64;
                        if nat_eq(a1668, a1669) {
                            f8(Str::lit(", which no input defines as a dictionary (Web IDL 2.7)"))
                        } else {
                            f8(Str::lit(", which no input defines as an interface (Web IDL 2.2)"))
                        }
                    }, List::<Bytes>::nil()))))))
                }
                Some(v5) => {
                    let m1670 = f231(v3, v1, v5)?;
                    match m1670 {
                        Ok(v7) => {
                            Ok(Ok::<List<Adt14>, Bytes>(f343(v7, v4)))
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

pub fn f232(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: mk, fields: []}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))
}

pub fn f233(v0: bool, v1: List<Adt14>) -> R<Result<(List<Bytes>, List<Bytes>), Bytes>> {
    let m1671 = v1;
    match m1671.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::<Bytes>::nil(), List::<Bytes>::nil())))
        }
        Some((v2, v3)) => {
            let v4: Adt15 = f347(v2.clone());
            if !f46(f242(v4.clone()), Bytes::lit(&[])) {
                Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons(f348(v2), List::cons(f8(Str::lit(": ")), List::cons(f242(v4), List::<Bytes>::nil())))))))
            } else {
                let m1672 = f349(v0, f348(v2.clone()))?;
                match m1672 {
                    None => {
                        Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons(f348(v2), List::cons(f8(Str::lit(" has no field name")), List::<Bytes>::nil()))))))
                    }
                    Some(v5) => {
                        if f153(v5.clone(), f350(v0, f235(v3.clone()))?) {
                            Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("two of its members derive the field name ")), List::cons(v5, List::cons(f8(Str::lit(" (Web IDL 2.2 and 2.7: the members of one definition have distinct identifiers)")), List::<Bytes>::nil()))))))
                        } else {
                            let m1673 = f233(v0, v3)?;
                            match m1673 {
                                Ok(v7) => {
                                    Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v5, List::cons(f8(Str::lit(", type: ")), List::cons({
                                        let a1674 = f351(v2);
                                        let a1675 = 1u64;
                                        if nat_eq(a1674, a1675) {
                                            f7(List::cons(f8(Str::lit("(Option ")), List::cons(f244(v4.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                        } else {
                                            f244(v4.clone())
                                        }
                                    }, List::cons(f8(Str::lit("}")), List::<Bytes>::nil())))))), {
                                        let (h1676, _) = v7;
                                        h1676
                                    }), f156(f245(v4), {
                                        let (_, h1677) = v7;
                                        h1677
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

pub fn f234(p1681: List<Bytes>) -> R<bool> {
    let m1680 = p1681;
    {
        let mut m1680 = m1680;
        loop {
            let step = {
                let v0 = m1680;
                let m1678 = v0;
                match m1678.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m1679 = f352(v1)?;
                        match m1679 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f153(v3, f350(false, v2.clone())?) {
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
                Ok(next) => { m1680 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f235(v0: List<Adt14>) -> List<Bytes> {
    let m1682 = v0;
    match m1682.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f348(v1), f235(v2))
        }
    }
}

pub fn f236(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}")), List::cons(f103(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f237(v0: Adt13) -> List<Bytes> {
    let m1683 = v0;
    match m1683 {
        Adt13::C0(_, _, _, _, v5, _) => {
            v5
        }
    }
}

pub fn f238(v0: bool, v1: List<Adt2>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1684 = v1;
    match m1684.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m1685 = f353(v0, v3.clone())?;
            match m1685 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f134(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f153(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m1686 = f238(v0, v4, List::cons(v5.clone(), v2))?;
                        match m1686 {
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

pub fn f239(v0: List<Bytes>) -> List<Adt2> {
    let m1687 = v0;
    match m1687.uncons() {
        None => {
            List::<Adt2>::nil()
        }
        Some((v1, v2)) => {
            List::cons(Adt2::C5(v1), f239(v2))
        }
    }
}

pub fn f240(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f103(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f241(v0: Adt13) -> Adt15 {
    let m1688 = v0;
    match m1688 {
        Adt13::C0(_, _, _, _, _, v6) => {
            v6
        }
    }
}

pub fn f242(v0: Adt15) -> Bytes {
    let m1689 = v0;
    match m1689 {
        Adt15::C0(_, _, _, v4, _) => {
            v4
        }
    }
}

pub fn f243(v0: Adt15) -> u64 {
    let m1690 = v0;
    match m1690 {
        Adt15::C0(v1, _, _, _, _) => {
            v1
        }
    }
}

pub fn f244(v0: Adt15) -> Bytes {
    let m1691 = v0;
    match m1691 {
        Adt15::C0(_, v2, _, _, _) => {
            v2
        }
    }
}

pub fn f245(v0: Adt15) -> List<Bytes> {
    let m1692 = v0;
    match m1692 {
        Adt15::C0(_, _, v3, _, _) => {
            v3
        }
    }
}

pub fn f246(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f247(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h1693, _) = v2;
        h1693
    };
    let a1694 = v3;
    let a1695 = 0u64;
    if nat_eq(a1694, a1695) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a1696 = v3;
            let a1697 = v1;
            let a1698 = 0u64;
            nat_quot(a1696, a1697, a1698)
        }, {
            let a1703 = f164(v0, {
                let a1699 = v3;
                let a1700 = v1;
                let a1701 = 0u64;
                nat_rem(a1699, a1700, a1701)
            });
            let a1704 = {
                let (_, h1702) = v2;
                h1702
            };
            append_bytes(a1703, a1704)
        }))
    }
}

pub fn f248(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h1705, _) = v1;
        h1705
    };
    let a1707 = {
        let a1706 = v0.clone();
        length_bytes(a1706)
    };
    let a1708 = v2;
    if nat_le(a1707, a1708) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let a1709 = f115(v0.clone(), v2)?;
        let a1710 = 13u64;
        if nat_eq(a1709, a1710) {
            Ok(Some(({
                let a1713 = f115(v0, {
                    let a1711 = v2;
                    let a1712 = 1u64;
                    nat_add(a1711, a1712)?
                })?;
                let a1714 = 10u64;
                if nat_eq(a1713, a1714) {
                    let a1715 = v2;
                    let a1716 = 2u64;
                    nat_add(a1715, a1716)?
                } else {
                    let a1717 = v2;
                    let a1718 = 1u64;
                    nat_add(a1717, a1718)?
                }
            }, {
                let a1720 = {
                    let (_, h1719) = v1;
                    h1719
                };
                let a1721 = f8(Str::lit("\u{a}"));
                append_bytes(a1720, a1721)
            })))
        } else {
            let v4: u64 = {
                let m1722 = f171(v0.clone(), f8(Str::lit("\u{d}")), v2)?;
                match m1722 {
                    None => {
                        let a1723 = v0.clone();
                        length_bytes(a1723)
                    }
                    Some(v3) => {
                        v3
                    }
                }
            };
            Ok(Some((v4, {
                let a1727 = {
                    let (_, h1724) = v1;
                    h1724
                };
                let a1728 = f43(v0, v2, {
                    let a1725 = v4;
                    let a1726 = v2;
                    nat_sub(a1725, a1726)
                });
                append_bytes(a1727, a1728)
            })))
        }
    }
}

pub fn f249(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1729 = f354(v0, v1)?;
    match m1729 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f355({
                let (h1730, _) = v2;
                h1730
            })? {
                Ok(Some({
                    let a1732 = v1;
                    let a1733 = {
                        let (_, h1731) = v2;
                        h1731
                    };
                    nat_add(a1732, a1733)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f250(v0: u64, v1: Bytes) -> R<bool> {
    f356(v0, v1.clone(), 0u64, {
        let a1735 = {
            let a1734 = v1;
            length_bytes(a1734)
        };
        let a1736 = 3u64;
        let a1737 = 0u64;
        nat_quot(a1735, a1736, a1737)
    }, true)
}

pub fn f251() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{0}\u{0} "))
}

pub fn f252(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f357(v1, v0, v2)
}

pub fn f253(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f358(v0, v1)
}

pub fn f254(v0: Bytes, p1740: List<Bytes>) -> bool {
    let m1739 = p1740;
    {
        let mut m1739 = m1739;
        loop {
            let step = {
                let v1 = m1739;
                let m1738 = v1;
                match m1738.uncons() {
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
                Ok(next) => { m1739 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f255(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f359(v0, v1, v2)
}

pub fn f256(v0: Bytes) -> Result<(Bytes, u64), Bytes> {
    Err::<(Bytes, u64), Bytes>(v0)
}

pub fn f257(v0: Bytes, v1: u64) -> R<Result<(Bytes, u64), Bytes>> {
    if f114(v0.clone(), v1, Str::lit("&#")) {
        let v2: bool = {
            let a1743 = f115(v0.clone(), {
                let a1741 = v1;
                let a1742 = 2u64;
                nat_add(a1741, a1742)?
            })?;
            let a1744 = 120u64;
            nat_eq(a1743, a1744)
        };
        let v3: u64 = if v2 {
            let a1745 = v1;
            let a1746 = 3u64;
            nat_add(a1745, a1746)?
        } else {
            let a1747 = v1;
            let a1748 = 2u64;
            nat_add(a1747, a1748)?
        };
        let v4: (u64, u64) = f360(v0.clone(), v2, v3)?;
        let a1750 = {
            let (h1749, _) = v4;
            h1749
        };
        let a1751 = v3;
        if if nat_eq(a1750, a1751) {
            true
        } else {
            let a1753 = f115(v0, {
                let (h1752, _) = v4;
                h1752
            })?;
            let a1754 = 59u64;
            !nat_eq(a1753, a1754)
        } {
            Ok(f256(f111(Str::lit("a character reference must be &#digits; or &#xhex; (XML 1.0 [66])"), v1)?))
        } else {
            if f355({
                let (_, h1755) = v4;
                h1755
            })? {
                Ok(Ok::<(Bytes, u64), Bytes>((f361({
                    let (_, h1756) = v4;
                    h1756
                })?, {
                    let a1758 = {
                        let (h1757, _) = v4;
                        h1757
                    };
                    let a1759 = 1u64;
                    nat_add(a1758, a1759)?
                })))
            } else {
                Ok(f256(f111(Str::lit("a character reference must name a Char (XML 1.0 [66] WFC: Legal Character)"), v1)?))
            }
        }
    } else {
        if f114(v0.clone(), v1, Str::lit("&lt;")) {
            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("<")), {
                let a1760 = v1;
                let a1761 = 4u64;
                nat_add(a1760, a1761)?
            })))
        } else {
            if f114(v0.clone(), v1, Str::lit("&gt;")) {
                Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit(">")), {
                    let a1762 = v1;
                    let a1763 = 4u64;
                    nat_add(a1762, a1763)?
                })))
            } else {
                if f114(v0.clone(), v1, Str::lit("&amp;")) {
                    Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("&")), {
                        let a1764 = v1;
                        let a1765 = 5u64;
                        nat_add(a1764, a1765)?
                    })))
                } else {
                    if f114(v0.clone(), v1, Str::lit("&apos;")) {
                        Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("'")), {
                            let a1766 = v1;
                            let a1767 = 6u64;
                            nat_add(a1766, a1767)?
                        })))
                    } else {
                        if f114(v0, v1, Str::lit("&quot;")) {
                            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("\"")), {
                                let a1768 = v1;
                                let a1769 = 6u64;
                                nat_add(a1768, a1769)?
                            })))
                        } else {
                            Ok(f256(f111(Str::lit("an entity reference names an entity that is not declared, and only the five predefined entities are (XML 1.0 section 4.1, WFC: Entity Declared)"), v1)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f258(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<u64> {
    let (h1770, _) = f168(Fn4::F362(v1, v2, v0.clone()), {
        let a1772 = {
            let a1771 = v0;
            length_bytes(a1771)
        };
        let a1773 = 1u64;
        nat_add(a1772, a1773)?
    }, v3)?;
    Ok(h1770)
}

pub fn f259(p1786: Adt17, v1: List<(Bytes, Bytes)>) -> R<Result<Option<Adt27>, Bytes>> {
    let m1785 = p1786;
    {
        let mut m1785 = m1785;
        loop {
            let step = {
                let v0 = m1785;
                let m1774 = v0;
                match m1774 {
                    Adt17::C0(_, _, _) => {
                        Err::<Adt17, R<Result<Option<Adt27>, Bytes>>>(Ok(Ok::<Option<Adt27>, Bytes>(None::<Adt27>)))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<Adt17, R<Result<Option<Adt27>, Bytes>>>(Ok(Ok::<Option<Adt27>, Bytes>(None::<Adt27>)))
                    }
                    Adt17::C2(_) => {
                        Err::<Adt17, R<Result<Option<Adt27>, Bytes>>>(Ok(Ok::<Option<Adt27>, Bytes>(None::<Adt27>)))
                    }
                    Adt17::C3(r1783, r1784) => {
                        let v2 = (*r1783).clone();
                        let v3 = (*r1784).clone();
                        let m1775 = v2;
                        match m1775 {
                            Adt17::C0(v4, r1781, r1782) => {
                                let v5 = (*r1781).clone();
                                let v6 = (*r1782).clone();
                                let m1776 = f118(v5.clone(), v1.clone());
                                match m1776 {
                                    Ok(v8) => {
                                        let m1777 = f119(v8.clone(), v4)?;
                                        match m1777 {
                                            Ok(v10) => {
                                                if f254({
                                                    let (_, h1778) = v10.clone();
                                                    h1778
                                                }, List::cons(f8(Str::lit("restriction")), List::cons(f8(Str::lit("list")), List::cons(f8(Str::lit("union")), List::<Bytes>::nil())))) {
                                                    if f46({
                                                        let (h1779, _) = v10;
                                                        h1779
                                                    }, f188()) {
                                                        Err::<Adt17, R<Result<Option<Adt27>, Bytes>>>(Ok(Ok::<Option<Adt27>, Bytes>(Some(Adt27::C0({
                                                            let (_, h1780) = v10;
                                                            h1780
                                                        }, v5, v6, v8)))))
                                                    } else {
                                                        Ok::<Adt17, R<Result<Option<Adt27>, Bytes>>>(v3)
                                                    }
                                                } else {
                                                    Ok::<Adt17, R<Result<Option<Adt27>, Bytes>>>(v3)
                                                }
                                            }
                                            Err(v9) => {
                                                Err::<Adt17, R<Result<Option<Adt27>, Bytes>>>(Ok(f363(v9)))
                                            }
                                        }
                                    }
                                    Err(v7) => {
                                        Err::<Adt17, R<Result<Option<Adt27>, Bytes>>>(Ok(f363(v7)))
                                    }
                                }
                            }
                            Adt17::C1(_, _, _) => {
                                Ok::<Adt17, R<Result<Option<Adt27>, Bytes>>>(v3)
                            }
                            Adt17::C2(_) => {
                                Ok::<Adt17, R<Result<Option<Adt27>, Bytes>>>(v3)
                            }
                            Adt17::C3(_, _) => {
                                Ok::<Adt17, R<Result<Option<Adt27>, Bytes>>>(v3)
                            }
                            Adt17::C4 => {
                                Ok::<Adt17, R<Result<Option<Adt27>, Bytes>>>(v3)
                            }
                        }
                    }
                    Adt17::C4 => {
                        Err::<Adt17, R<Result<Option<Adt27>, Bytes>>>(Ok(Ok::<Option<Adt27>, Bytes>(None::<Adt27>)))
                    }
                }
            };
            match step {
                Ok(next) => { m1785 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f260(v0: Bytes) -> Result<Adt22, Bytes> {
    Err::<Adt22, Bytes>(v0)
}

pub fn f261(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit(" at simpleType ")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f262(v0: List<(Bytes, Bytes)>, v1: Bytes, v2: Bytes) -> R<Result<Adt22, Bytes>> {
    let m1787 = f119(v0, v2.clone())?;
    match m1787 {
        Ok(v4) => {
            if f46({
                let (h1788, _) = v4.clone();
                h1788
            }, f188()) {
                let m1790 = f364({
                    let (_, h1789) = v4;
                    h1789
                }, 16u64)?;
                match m1790 {
                    Ok(v6) => {
                        Ok(Ok::<Adt22, Bytes>(Adt22::C0(v6, List::<Bytes>::nil())))
                    }
                    Err(v5) => {
                        Ok(f260(v5))
                    }
                }
            } else {
                if f46({
                    let (h1791, _) = v4.clone();
                    h1791
                }, v1) {
                    let v7: Bytes = f191({
                        let (_, h1792) = v4;
                        h1792
                    })?;
                    Ok(Ok::<Adt22, Bytes>(Adt22::C0(v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                } else {
                    Ok(f260(f7(List::cons(f8(Str::lit("the type ")), List::cons(v2, List::cons(f8(Str::lit(" is in the namespace ")), List::cons({
                        let (h1793, _) = v4;
                        h1793
                    }, List::cons(f8(Str::lit(", which is neither XML Schema's nor this document's target namespace; imported schema documents are not read yet")), List::<Bytes>::nil()))))))))
                }
            }
        }
        Err(v3) => {
            Ok(f260(v3))
        }
    }
}

pub fn f263(v0: Adt17, v1: List<(Bytes, Bytes)>) -> R<Result<List<Adt2>, Bytes>> {
    let m1794 = v0;
    match m1794 {
        Adt17::C0(_, _, _) => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
        Adt17::C2(_) => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
        Adt17::C3(r1801, r1802) => {
            let v2 = (*r1801).clone();
            let v3 = (*r1802).clone();
            let m1795 = f263(v3, v1.clone())?;
            match m1795 {
                Ok(v5) => {
                    let m1796 = v2;
                    match m1796 {
                        Adt17::C0(v6, r1800, _) => {
                            let v7 = (*r1800).clone();
                            let m1797 = f118(v7.clone(), v1);
                            match m1797 {
                                Ok(v10) => {
                                    let m1798 = f119(v10, v6)?;
                                    match m1798 {
                                        Ok(v12) => {
                                            if f120(v12, Str::lit("enumeration")) {
                                                let m1799 = f121(v7, f8(Str::lit("value")));
                                                match m1799 {
                                                    None => {
                                                        Ok(f365(f8(Str::lit("an enumeration facet must have a value attribute (XSD 1.1 Part 2, section 4.3.5.2)"))))
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
                                            Ok(f365(v11))
                                        }
                                    }
                                }
                                Err(v9) => {
                                    Ok(f365(v9))
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
                    Ok(f365(v4))
                }
            }
        }
        Adt17::C4 => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
    }
}

pub fn f264(v0: List<Adt2>, v1: List<Bytes>) -> R<List<Adt2>> {
    let m1803 = v0;
    match m1803.uncons() {
        None => {
            Ok(List::<Adt2>::nil())
        }
        Some((v2, v3)) => {
            let v4: Bytes = f134(v2.clone())?;
            if f153(v4.clone(), v1.clone()) {
                f264(v3, v1)
            } else {
                Ok(List::cons(v2, f264(v3, List::cons(v4, v1))?))
            }
        }
    }
}

pub fn f265(v0: List<Adt2>) -> List<Bytes> {
    let m1804 = v0;
    match m1804.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1805 = f366(Some(v1));
            match m1805 {
                None => {
                    f265(v2)
                }
                Some(v3) => {
                    List::cons(v3, f265(v2))
                }
            }
        }
    }
}

pub fn f266(v0: Bytes, p1826: u64, p1827: u64, p1828: bool, p1829: Bytes) -> R<Bytes> {
    let m1825 = (p1826, (p1827, (p1828, p1829)));
    {
        let mut m1825 = m1825;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1825;
                let m1806 = v2;
                if m1806 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1806 - 1;
                    let m1809 = {
                        let a1807 = v0.clone();
                        let a1808 = v1;
                        index_bytes(a1807, a1808)
                    };
                    match m1809 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f52(v6)?;
                            if f367(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1810 = v1;
                                    let a1811 = 1u64;
                                    nat_add(a1810, a1811)?
                                }, (v5, (true, v4))))
                            } else {
                                if f85(f368(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1812 = v1;
                                        let a1813 = 1u64;
                                        nat_add(a1812, a1813)?
                                    }, (v5, (false, {
                                        let a1817 = v4;
                                        let a1818 = f43({
                                            let a1814 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1814)
                                        }, {
                                            let a1815 = v7;
                                            let a1816 = 97u64;
                                            nat_sub(a1815, a1816)
                                        }, 1u64);
                                        append_bytes(a1817, a1818)
                                    }))))
                                } else {
                                    if f139(f368(v7), f139(f369(v7), f370(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1819 = v1;
                                            let a1820 = 1u64;
                                            nat_add(a1819, a1820)?
                                        }, (v5, (false, {
                                            let a1821 = v4;
                                            let a1822 = f43(v0.clone(), v1, 1u64);
                                            append_bytes(a1821, a1822)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1823 = v1;
                                            let a1824 = 1u64;
                                            nat_add(a1823, a1824)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1825 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f267(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f184(v0.clone(), v1) {
        let m1830 = f41(v0, 34u8)?;
        match m1830.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m1831 = v3;
                match m1831.uncons() {
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

pub fn f268(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f371(v0, v1)
}

pub fn f269(v0: List<Adt19>) -> R<Result<List<Adt24>, Bytes>> {
    let m1832 = v0;
    match m1832.uncons() {
        None => {
            Ok(Ok::<List<Adt24>, Bytes>(List::<Adt24>::nil()))
        }
        Some((v1, v2)) => {
            let m1835 = f204(match v1.clone() {
                Adt19::C0(_, _, _, _, h1833, _) => {
                    h1833
                }
            }, match v1.clone() {
                Adt19::C0(_, _, _, h1834, _, _) => {
                    h1834
                }
            })?;
            match m1835 {
                Ok(v4) => {
                    let m1836 = f269(v2)?;
                    match m1836 {
                        Ok(v6) => {
                            Ok(Ok::<List<Adt24>, Bytes>(f372(f373(v4, match v1 {
                                Adt19::C0(_, _, h1837, _, _, _) => {
                                    h1837
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

pub fn f270(v0: List<Adt19>) -> List<Bytes> {
    let m1838 = v0;
    match m1838.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt19::C0(_, _, h1839, _, _, _) => {
                    h1839
                }
            }, f270(v2))
        }
    }
}

pub fn f271(v0: u64, v1: Adt23, v2: List<Adt24>) -> R<Result<List<Adt26>, Bytes>> {
    let m1840 = v2;
    match m1840.uncons() {
        None => {
            Ok(Ok::<List<Adt26>, Bytes>(List::<Adt26>::nil()))
        }
        Some((v3, v4)) => {
            let m1841 = f374(v0, v1.clone(), v3)?;
            match m1841 {
                Ok(v6) => {
                    let m1842 = f271(v0, v1, v4)?;
                    match m1842 {
                        Ok(v8) => {
                            Ok(Ok::<List<Adt26>, Bytes>(f375(v6, v8)))
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

pub fn f272(v0: List<Adt19>) -> R<u64> {
    let m1843 = v0;
    match m1843.uncons() {
        None => {
            Ok(64u64)
        }
        Some((v1, v2)) => {
            let a1845 = match v1 {
                Adt19::C0(_, _, _, _, _, h1844) => {
                    h1844
                }
            };
            let a1846 = f272(v2)?;
            nat_add(a1845, a1846)
        }
    }
}

pub fn f273(p1854: List<Adt26>, p1855: List<Adt26>) -> Result<List<Adt26>, Bytes> {
    let m1853 = (p1854, p1855);
    {
        let mut m1853 = m1853;
        loop {
            let step = {
                let (v0, v1) = m1853;
                let m1847 = v0;
                match m1847.uncons() {
                    None => {
                        Err::<(List<Adt26>, List<Adt26>), Result<List<Adt26>, Bytes>>(Ok::<List<Adt26>, Bytes>(f376(v1, List::<Adt26>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m1849 = f377(v1.clone(), match v2.clone() {
                            Adt26::C0(h1848, _, _, _) => {
                                h1848
                            }
                        });
                        match m1849 {
                            None => {
                                Ok::<(List<Adt26>, List<Adt26>), Result<List<Adt26>, Bytes>>((v3, List::cons(v2, v1)))
                            }
                            Some(v4) => {
                                if f46(match v4 {
                                    Adt26::C0(_, _, h1850, _) => {
                                        h1850
                                    }
                                }, match v2.clone() {
                                    Adt26::C0(_, _, h1851, _) => {
                                        h1851
                                    }
                                }) {
                                    Ok::<(List<Adt26>, List<Adt26>), Result<List<Adt26>, Bytes>>((v3, v1))
                                } else {
                                    Err::<(List<Adt26>, List<Adt26>), Result<List<Adt26>, Bytes>>(Err::<List<Adt26>, Bytes>(f200(f8(Str::lit("two components derive the type name ")), match v2 {
                                        Adt26::C0(h1852, _, _, _) => {
                                            h1852
                                        }
                                    }, f8(Str::lit(", so their LexLean declarations would collide")))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1853 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f274(v0: List<Adt26>) -> List<Adt26> {
    if if f153(f378(), f379(v0.clone())) {
        let m1856 = f377(v0.clone(), f378());
        match m1856 {
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
        List::cons(Adt26::C0(f378(), List::<Bytes>::nil(), f240(f378(), f380()), f380()), v0)
    } else {
        v0
    }
}

pub fn f275(v0: List<Adt26>) -> List<Bytes> {
    let m1857 = v0;
    match m1857.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt26::C0(h1858, _, _, _) => {
                    h1858
                }
            }, f275(v2))
        }
    }
}

pub fn f276(p1864: List<Adt26>, v1: List<Bytes>) -> Option<(Bytes, Bytes)> {
    let m1863 = p1864;
    {
        let mut m1863 = m1863;
        loop {
            let step = {
                let v0 = m1863;
                let m1859 = v0;
                match m1859.uncons() {
                    None => {
                        Err::<List<Adt26>, Option<(Bytes, Bytes)>>(None::<(Bytes, Bytes)>)
                    }
                    Some((v2, v3)) => {
                        let m1861 = f192(match v2.clone() {
                            Adt26::C0(_, h1860, _, _) => {
                                h1860
                            }
                        }, v1.clone());
                        match m1861 {
                            None => {
                                Ok::<List<Adt26>, Option<(Bytes, Bytes)>>(v3)
                            }
                            Some(v4) => {
                                Err::<List<Adt26>, Option<(Bytes, Bytes)>>(Some((match v2 {
                                    Adt26::C0(h1862, _, _, _) => {
                                        h1862
                                    }
                                }, v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1863 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f277(v0: List<Adt26>, v1: List<Bytes>, v2: u64) -> List<(Bytes, List<Bytes>)> {
    let m1865 = v1;
    match m1865.uncons() {
        None => {
            List::<(Bytes, List<Bytes>)>::nil()
        }
        Some((v3, v4)) => {
            List::cons((v3.clone(), f381(v2, v0.clone(), f382(v0.clone(), v3), List::<Bytes>::nil())), f277(v0, v4, v2))
        }
    }
}

pub fn f278(v0: List<Adt26>) -> R<u64> {
    let m1866 = v0;
    match m1866.uncons() {
        None => {
            Ok(1u64)
        }
        Some((v1, v2)) => {
            let a1871 = {
                let a1869 = {
                    let a1868 = match v1 {
                        Adt26::C0(_, h1867, _, _) => {
                            h1867
                        }
                    };
                    length_list(a1868)
                };
                let a1870 = 1u64;
                nat_add(a1869, a1870)?
            };
            let a1872 = f278(v2)?;
            nat_add(a1871, a1872)
        }
    }
}

pub fn f279(p1882: u64, v1: List<Adt26>, p1883: List<(List<Bytes>, List<Bytes>)>, p1884: List<Bytes>, p1885: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m1881 = (p1882, (p1883, (p1884, p1885)));
    {
        let mut m1881 = m1881;
        loop {
            let step = {
                let (v0, (v2, (v3, v4))) = m1881;
                let m1873 = v0;
                if m1873 == 0 {
                    let m1874 = v2;
                    match m1874.uncons() {
                        None => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v4))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(f8(Str::lit("the declarations could not be ordered so that each follows the types it uses"))))
                        }
                    }
                } else {
                    let v7 = m1873 - 1;
                    let m1875 = v2.clone();
                    match m1875.uncons() {
                        None => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v4))
                        }
                        Some((_, _)) => {
                            let m1876 = f383(v2, v3.clone());
                            match m1876 {
                                None => {
                                    Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(f8(Str::lit("the declarations could not be ordered so that each follows the types it uses"))))
                                }
                                Some(v10) => {
                                    let m1878 = f384(v1.clone(), {
                                        let (h1877, _) = v10.clone();
                                        h1877
                                    });
                                    match m1878 {
                                        Ok(v12) => {
                                            Ok::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>((v7, ({
                                                let (_, h1879) = v10;
                                                h1879
                                            }, (f385({
                                                let (h1880, _) = v10;
                                                h1880
                                            }, v3), f385(v4, v12)))))
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
                Ok(next) => { m1881 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f280(v0: List<Adt26>, v1: List<List<Bytes>>) -> List<(List<Bytes>, List<Bytes>)> {
    let m1886 = v1;
    match m1886.uncons() {
        None => {
            List::<(List<Bytes>, List<Bytes>)>::nil()
        }
        Some((v2, v3)) => {
            List::cons((v2.clone(), f386(v0.clone(), v2.clone(), v2)), f280(v0, v3))
        }
    }
}

pub fn f281(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> List<List<Bytes>> {
    let m1887 = v2;
    match m1887.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((v4, v5)) => {
            if f153(v4.clone(), v3.clone()) {
                f281(v0, v1, v5, v3)
            } else {
                let v6: List<Bytes> = f387(v0.clone(), v1.clone(), v4);
                List::cons(v6.clone(), f281(v0, v1, v5, f385(v6, v3)))
            }
        }
    }
}

pub fn f282(v0: bool) -> bool {
    !v0
}

pub fn f283(v0: Adt25, v1: Str) -> bool {
    f46(match v0 {
        Adt25::C0(_, h1888, _, _, _) => {
            h1888
        }
    }, f8(v1))
}

pub fn f284(v0: Adt25, v1: Str) -> Option<Bytes> {
    f121(match v0 {
        Adt25::C0(_, _, h1889, _, _) => {
            h1889
        }
    }, f8(v1))
}

pub fn f285(v0: Option<Bytes>, v1: Bytes) -> R<Result<Bytes, Bytes>> {
    let m1890 = f171(v1.clone(), f8(Str::lit("://")), 0u64)?;
    match m1890 {
        None => {
            let v3: Bytes = f200(f8(Str::lit("the relative schema location ")), v1.clone(), f8(Str::lit(" is not resolved, because only a bare file name relative to a bound document's URL is merged (RFC 3986, section 5.2.3)")));
            if if f184(v1.clone(), f8(Str::lit("/"))) {
                true
            } else {
                let m1891 = f171(v1.clone(), f8(Str::lit("..")), 0u64)?;
                match m1891 {
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
                let m1892 = v0;
                match m1892 {
                    None => {
                        Ok(Err::<Bytes, Bytes>(v3))
                    }
                    Some(v5) => {
                        let m1894 = f388(v5.clone(), 0u64, {
                            let a1893 = v5.clone();
                            length_bytes(a1893)
                        }, None::<u64>)?;
                        match m1894 {
                            None => {
                                Ok(Err::<Bytes, Bytes>(v3))
                            }
                            Some(v6) => {
                                Ok(Ok::<Bytes, Bytes>({
                                    let a1897 = f43(v5, 0u64, {
                                        let a1895 = v6;
                                        let a1896 = 1u64;
                                        nat_add(a1895, a1896)?
                                    });
                                    let a1898 = v1;
                                    append_bytes(a1897, a1898)
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

pub fn f286(p1903: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1902 = p1903;
    {
        let mut m1902 = m1902;
        loop {
            let step = {
                let v0 = m1902;
                let m1899 = v0;
                match m1899.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f46({
                            let (h1900, _) = v2;
                            h1900
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h1901) = v2;
                                h1901
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
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

pub fn f287(v0: Adt25, v1: Str, v2: Str) -> Bytes {
    let m1904 = f284(v0, v1);
    match m1904 {
        None => {
            f8(v2)
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f288(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1905 = f354(v0, v1)?;
    match m1905 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f389({
                let (h1906, _) = v2;
                h1906
            }) {
                Ok(Some({
                    let a1908 = v1;
                    let a1909 = {
                        let (_, h1907) = v2;
                        h1907
                    };
                    nat_add(a1908, a1909)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f289(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1911 = v1;
    let a1912 = {
        let a1910 = v0.clone();
        length_bytes(a1910)
    };
    if if nat_lt(a1911, a1912) {
        f390(v0.clone(), v1)?
    } else {
        false
    } {
        Ok(Some(f294(v0, v1)?))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f290(v0: u64) -> bool {
    let a1913 = v0;
    let a1914 = 32u64;
    if nat_eq(a1913, a1914) {
        true
    } else {
        let a1915 = v0;
        let a1916 = 9u64;
        nat_eq(a1915, a1916)
    }
}

pub fn f291(v0: u64) -> bool {
    let a1917 = v0;
    let a1918 = 10u64;
    if nat_eq(a1917, a1918) {
        true
    } else {
        let a1919 = v0;
        let a1920 = 256u64;
        nat_eq(a1919, a1920)
    }
}

pub fn f292(v0: Bytes, v1: u64) -> R<u64> {
    let a1921 = v1;
    let a1922 = f391(v0, v1)?;
    Ok(nat_sub(a1921, a1922))
}

pub fn f293(v0: Bytes, v1: u64) -> R<u64> {
    let (h1923, _) = f168(Fn4::F392(v0.clone()), {
        let a1925 = {
            let a1924 = v0;
            length_bytes(a1924)
        };
        let a1926 = 1u64;
        nat_add(a1925, a1926)?
    }, v1)?;
    Ok(h1923)
}

pub fn f294(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f393(v0.clone(), v1)?;
    let a1928 = v2;
    let a1929 = {
        let a1927 = v0;
        length_bytes(a1927)
    };
    if nat_lt(a1928, a1929) {
        let a1930 = v2;
        let a1931 = 1u64;
        nat_add(a1930, a1931)
    } else {
        Ok(v2)
    }
}

pub fn f295(v0: Bytes, v1: u64) -> R<u64> {
    let (h1932, _) = f168(Fn4::F394(v0.clone()), {
        let a1934 = {
            let a1933 = v0;
            length_bytes(a1933)
        };
        let a1935 = 1u64;
        nat_add(a1934, a1935)?
    }, v1)?;
    Ok(h1932)
}

pub fn f296(v0: Bytes, v1: u64) -> R<bool> {
    let a1936 = f212(v0.clone(), v1)?;
    let a1937 = 45u64;
    if nat_eq(a1936, a1937) {
        if f290(f212(v0.clone(), {
            let a1938 = v1;
            let a1939 = 1u64;
            nat_add(a1938, a1939)?
        })?) {
            Ok(true)
        } else {
            Ok(f291(f212(v0, {
                let a1940 = v1;
                let a1941 = 1u64;
                nat_add(a1940, a1941)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f297(v0: Bytes, v1: u64) -> R<bool> {
    let m1942 = f303(v0, v1)?;
    match m1942 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f298(v0: Bytes, v1: u64) -> R<Option<Bytes>> {
    let v2: u64 = f212(v0.clone(), v1)?;
    let v3: bool = f214(f395(f212(v0, {
        let a1943 = v1;
        let a1944 = 1u64;
        nat_add(a1943, a1944)?
    })?, true));
    let a1945 = v2;
    let a1946 = 58u64;
    if if nat_eq(a1945, a1946) {
        v3
    } else {
        false
    } {
        Ok(Some(f111(Str::lit("a null key is not read: Studio.Value has no null (YAML 1.2.2 section 10.3.2)"), v1)?))
    } else {
        let a1947 = v2;
        let a1948 = 38u64;
        if nat_eq(a1947, a1948) {
            Ok(Some(f111(Str::lit("an anchor is not read (YAML 1.2.2 [101])"), v1)?))
        } else {
            let a1949 = v2;
            let a1950 = 42u64;
            if nat_eq(a1949, a1950) {
                Ok(Some(f111(Str::lit("an alias is not read (YAML 1.2.2 [104])"), v1)?))
            } else {
                let a1951 = v2;
                let a1952 = 33u64;
                if nat_eq(a1951, a1952) {
                    Ok(Some(f111(Str::lit("a tag is not read (YAML 1.2.2 [97])"), v1)?))
                } else {
                    let a1953 = v2;
                    let a1954 = 63u64;
                    if if nat_eq(a1953, a1954) {
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

pub fn f299(v0: Bytes, v1: u64, v2: u64) -> R<Adt20> {
    let v3: u64 = f212(v0.clone(), v1)?;
    let a1955 = v3;
    let a1956 = 124u64;
    if if nat_eq(a1955, a1956) {
        true
    } else {
        let a1957 = v3;
        let a1958 = 62u64;
        nat_eq(a1957, a1958)
    } {
        f396(v0, v1, v2)
    } else {
        let a1959 = v3;
        let a1960 = 34u64;
        if if nat_eq(a1959, a1960) {
            true
        } else {
            let a1961 = v3;
            let a1962 = 39u64;
            nat_eq(a1961, a1962)
        } {
            let m1963 = f397(v0.clone(), v1, v2, false)?;
            match m1963 {
                Adt30::C0(v5, v6) => {
                    let m1964 = f211(v0, v6)?;
                    match m1964 {
                        Ok(v8) => {
                            Ok(Adt20::C0(Adt2::C5(v5), v8))
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
            let a1965 = v3;
            let a1966 = 91u64;
            if if nat_eq(a1965, a1966) {
                true
            } else {
                let a1967 = v3;
                let a1968 = 123u64;
                nat_eq(a1967, a1968)
            } {
                let m1972 = f398(v0.clone(), v1, v2, {
                    let a1970 = {
                        let a1969 = v0.clone();
                        length_bytes(a1969)
                    };
                    let a1971 = 2u64;
                    nat_add(a1970, a1971)?
                }, 0u64, List::<Bytes>::nil())?;
                match m1972 {
                    Adt20::C0(v10, v11) => {
                        let a1973 = f212(v0.clone(), f293(v0.clone(), v11)?)?;
                        let a1974 = 58u64;
                        if nat_eq(a1973, a1974) {
                            Ok(Adt20::C1(f111(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [154])"), v1)?))
                        } else {
                            let m1975 = f211(v0, v11)?;
                            match m1975 {
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
                let m1976 = f298(v0.clone(), v1)?;
                match m1976 {
                    None => {
                        if f214(f399(v0.clone(), v1, false)?) {
                            Ok(Adt20::C1(f111(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                        } else {
                            let v15: u64 = f400(v0.clone(), v1, false)?;
                            let m1980 = f401(v0.clone(), v15, v2, {
                                let a1978 = {
                                    let a1977 = v0.clone();
                                    length_bytes(a1977)
                                };
                                let a1979 = 2u64;
                                nat_add(a1978, a1979)?
                            }, f302(v0, v1, v15))?;
                            match m1980 {
                                Adt30::C0(v17, v18) => {
                                    f402(v17, v1, v18)
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

pub fn f300(v0: bool, v1: Adt20) -> Adt20 {
    let m1981 = v1;
    match m1981 {
        Adt20::C0(v3, v4) => {
            Adt20::C0(if v0 {
                Adt2::C6(std::rc::Rc::new(v3))
            } else {
                Adt2::C7(std::rc::Rc::new(v3))
            }, v4)
        }
        Adt20::C1(v2) => {
            Adt20::C1(v2)
        }
    }
}

pub fn f301(v0: Bytes, v1: u64) -> R<u64> {
    let (h1982, _) = f168(Fn4::F403(v0.clone(), v1), {
        let a1984 = {
            let a1983 = v0;
            length_bytes(a1983)
        };
        let a1985 = 1u64;
        nat_add(a1984, a1985)?
    }, 0u64)?;
    Ok(h1982)
}

pub fn f302(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    f43(v0, v1, {
        let a1986 = v2;
        let a1987 = v1;
        nat_sub(a1986, a1987)
    })
}

pub fn f303(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f212(v0.clone(), v1)?;
    let a1988 = v2;
    let a1989 = 34u64;
    if if nat_eq(a1988, a1989) {
        true
    } else {
        let a1990 = v2;
        let a1991 = 39u64;
        nat_eq(a1990, a1991)
    } {
        let m1992 = f397(v0.clone(), v1, 0u64, true)?;
        match m1992 {
            Adt30::C0(_, v5) => {
                let v6: u64 = f293(v0.clone(), v5)?;
                if f404(v0, v6)? {
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
        if f399(v0.clone(), v1, false)? {
            let v7: u64 = f293(v0.clone(), f400(v0.clone(), v1, false)?)?;
            if f404(v0, v7)? {
                Ok(Some(v7))
            } else {
                Ok(None::<u64>)
            }
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f304(v0: Bytes, v1: u64) -> R<Bytes> {
    let v2: u64 = f212(v0.clone(), v1)?;
    let a1993 = v2;
    let a1994 = 34u64;
    if if nat_eq(a1993, a1994) {
        true
    } else {
        let a1995 = v2;
        let a1996 = 39u64;
        nat_eq(a1995, a1996)
    } {
        let m1997 = f397(v0, v1, 0u64, true)?;
        match m1997 {
            Adt30::C0(v4, _) => {
                Ok(v4)
            }
            Adt30::C1(_) => {
                Ok(Bytes::lit(&[]))
            }
        }
    } else {
        Ok(f302(v0.clone(), v1, f400(v0, v1, false)?))
    }
}

pub fn f305(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1998 = 0u64;
        let a1999 = v1;
        nat_lt(a1998, a1999)
    } else {
        false
    } {
        let a2000 = Bytes::lit(&[0x2d]);
        let a2001 = f65(v1)?;
        Ok(append_bytes(a2000, a2001))
    } else {
        f65(v1)
    }
}

pub fn f306(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f65(v1)?;
    let v4: u64 = {
        let a2002 = v3.clone();
        length_bytes(a2002)
    };
    let v5: Bytes = {
        let a2003 = v2;
        let a2004 = v4;
        if nat_lt(a2003, a2004) {
            f7(List::cons(f43(v3.clone(), 0u64, {
                let a2005 = v4;
                let a2006 = v2;
                nat_sub(a2005, a2006)
            }), List::cons(f8(Str::lit(".")), List::cons(f43(v3, {
                let a2007 = v4;
                let a2008 = v2;
                nat_sub(a2007, a2008)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f223({
                let a2009 = v2;
                let a2010 = v4;
                nat_sub(a2009, a2010)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a2011 = f8(Str::lit("-"));
        let a2012 = v5;
        Ok(append_bytes(a2011, a2012))
    } else {
        Ok(v5)
    }
}

pub fn f307(v0: Bytes) -> R<Bytes> {
    let a2017 = f136(v0.clone(), 0u64, {
        let a2014 = {
            let a2013 = v0.clone();
            length_bytes(a2013)
        };
        let a2015 = 1u64;
        nat_add(a2014, a2015)?
    })?;
    let a2018 = {
        let a2016 = v0.clone();
        length_bytes(a2016)
    };
    if nat_eq(a2017, a2018) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        let a2024 = {
            let a2022 = Bytes::lit(&[0x22]);
            let a2023 = f405(v0.clone(), 0u64, {
                let a2020 = {
                    let a2019 = v0;
                    length_bytes(a2019)
                };
                let a2021 = 1u64;
                nat_add(a2020, a2021)?
            }, Bytes::lit(&[]))?;
            append_bytes(a2022, a2023)
        };
        let a2025 = Bytes::lit(&[0x22]);
        Ok(append_bytes(a2024, a2025))
    }
}

pub fn f308(v0: List<Bytes>) -> Bytes {
    let a2028 = {
        let a2026 = Bytes::lit(&[0x5b]);
        let a2027 = f44(v0, Bytes::lit(&[0x2c]));
        append_bytes(a2026, a2027)
    };
    let a2029 = Bytes::lit(&[0x5d]);
    append_bytes(a2028, a2029)
}

pub fn f309(v0: Adt2) -> R<List<Bytes>> {
    let m2030 = v0;
    match m2030 {
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
        Adt2::C9(r2031, r2032) => {
            let v12 = (*r2031).clone();
            let v13 = (*r2032).clone();
            Ok(List::cons(f215(v12)?, f309(v13)?))
        }
        Adt2::C10(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f310(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f406(Fn6::F407, List::<(Bytes, Bytes)>::nil(), f408(v0)?);
    let v2: List<Bytes> = f409(Fn7::F410, List::<Bytes>::nil(), v1)?;
    let a2035 = {
        let a2033 = Bytes::lit(&[0x7b]);
        let a2034 = f44(v2, Bytes::lit(&[0x2c]));
        append_bytes(a2033, a2034)
    };
    let a2036 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a2035, a2036))
}

pub fn f311(v0: Adt2) -> R<List<(Bytes, Bytes)>> {
    let m2037 = v0;
    match m2037 {
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
        Adt2::C10(v14, r2038, r2039) => {
            let v15 = (*r2038).clone();
            let v16 = (*r2039).clone();
            Ok(List::cons((v14, f215(v15)?), f311(v16)?))
        }
    }
}

pub fn f312(v0: Bytes, v1: u64) -> R<Bytes> {
    let a2042 = f43(v0.clone(), {
        let a2040 = v1;
        let a2041 = 2u64;
        nat_add(a2040, a2041)?
    }, 2u64);
    let a2043 = Bytes::lit(&[0x30, 0x30]);
    if equal(a2042, a2043) {
        let m2046 = f411(f43(v0, {
            let a2044 = v1;
            let a2045 = 4u64;
            nat_add(a2044, a2045)?
        }, 2u64))?;
        match m2046 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f83(v2.clone(), 0u64)?;
                let a2047 = v3;
                let a2048 = 32u64;
                if nat_lt(a2047, a2048) {
                    if f412(v3) {
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

pub fn f313(v0: u64) -> Bytes {
    let a2049 = v0;
    let a2050 = 34u64;
    if nat_eq(a2049, a2050) {
        Bytes::lit(&[0x22])
    } else {
        let a2051 = v0;
        let a2052 = 92u64;
        if nat_eq(a2051, a2052) {
            Bytes::lit(&[0x5c])
        } else {
            let a2053 = v0;
            let a2054 = 98u64;
            if nat_eq(a2053, a2054) {
                Bytes::lit(&[0x08])
            } else {
                let a2055 = v0;
                let a2056 = 102u64;
                if nat_eq(a2055, a2056) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a2057 = v0;
                    let a2058 = 110u64;
                    if nat_eq(a2057, a2058) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a2059 = v0;
                        let a2060 = 114u64;
                        if nat_eq(a2059, a2060) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a2061 = v0;
                            let a2062 = 116u64;
                            if nat_eq(a2061, a2062) {
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

pub fn f314(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f83(v0, v1)?;
    let a2063 = 47u64;
    let a2064 = v2;
    if if nat_lt(a2063, a2064) {
        let a2065 = v2;
        let a2066 = 58u64;
        nat_lt(a2065, a2066)
    } else {
        false
    } {
        Ok(Some({
            let a2067 = v1;
            let a2068 = 1u64;
            nat_add(a2067, a2068)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f315(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2069 = f83(v0, v1)?;
    let a2070 = 48u64;
    if nat_eq(a2069, a2070) {
        Ok(Some({
            let a2071 = v1;
            let a2072 = 1u64;
            nat_add(a2071, a2072)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f316(v0: Bytes) -> R<u64> {
    let (h2073, _) = f168(Fn4::F413(v0.clone()), {
        let a2074 = v0;
        length_bytes(a2074)
    }, 0u64)?;
    Ok(h2073)
}

pub fn f317(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m2078 = {
        let a2076 = v0;
        let a2077 = {
            let (h2075, _) = v1;
            h2075
        };
        index_bytes(a2076, a2077)
    };
    match m2078 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2080 = {
                    let (h2079, _) = v1;
                    h2079
                };
                let a2081 = 1u64;
                nat_add(a2080, a2081)?
            }, {
                let a2085 = {
                    let a2083 = {
                        let (_, h2082) = v1;
                        h2082
                    };
                    let a2084 = 10u64;
                    nat_mul(a2083, a2084)?
                };
                let a2086 = f414(v2)?;
                nat_add(a2085, a2086)?
            })))
        }
    }
}

pub fn f318(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f415(v0, v1)
}

pub fn f319(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a2087 = v0;
        utf8_encode(a2087)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f65(v1)?, List::<Bytes>::nil())))))
}

pub fn f320(p2090: List<Adt21>, p2091: List<Adt21>) -> List<Adt21> {
    let m2089 = (p2090, p2091);
    {
        let mut m2089 = m2089;
        loop {
            let step = {
                let (v0, v1) = m2089;
                let m2088 = v0;
                match m2088.uncons() {
                    None => {
                        Err::<(List<Adt21>, List<Adt21>), List<Adt21>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt21>, List<Adt21>), List<Adt21>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2089 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f321(v0: u64) -> bool {
    let a2092 = 65u64;
    let a2093 = v0;
    if if nat_le(a2092, a2093) {
        let a2094 = v0;
        let a2095 = 90u64;
        nat_le(a2094, a2095)
    } else {
        false
    } {
        true
    } else {
        let a2096 = 97u64;
        let a2097 = v0;
        if nat_le(a2096, a2097) {
            let a2098 = v0;
            let a2099 = 122u64;
            nat_le(a2098, a2099)
        } else {
            false
        }
    }
}

pub fn f322(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h2100, _) = f168(Fn4::F416(v1, v0.clone()), {
        let a2102 = {
            let a2101 = v0;
            length_bytes(a2101)
        };
        let a2103 = 1u64;
        nat_add(a2102, a2103)?
    }, v2)?;
    Ok(h2100)
}

pub fn f323(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f115(v0.clone(), v1)?;
    let a2104 = v2;
    let a2105 = 48u64;
    if nat_eq(a2104, a2105) {
        let v3: u64 = f115(v0.clone(), {
            let a2106 = v1;
            let a2107 = 1u64;
            nat_add(a2106, a2107)?
        })?;
        let a2108 = v3;
        let a2109 = 88u64;
        if if if nat_eq(a2108, a2109) {
            true
        } else {
            let a2110 = v3;
            let a2111 = 120u64;
            nat_eq(a2110, a2111)
        } {
            f417(f115(v0.clone(), {
                let a2112 = v1;
                let a2113 = 2u64;
                nat_add(a2112, a2113)?
            })?)
        } else {
            false
        } {
            f322(v0, 1u64, {
                let a2114 = v1;
                let a2115 = 2u64;
                nat_add(a2114, a2115)?
            })
        } else {
            f322(v0, 2u64, {
                let a2116 = v1;
                let a2117 = 1u64;
                nat_add(a2116, a2117)?
            })
        }
    } else {
        if f418(v2) {
            f322(v0, 0u64, v1)
        } else {
            Ok(v1)
        }
    }
}

pub fn f324(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f322(v0.clone(), 0u64, v1)?;
    let a2118 = f115(v0.clone(), v2)?;
    let a2119 = 46u64;
    if nat_eq(a2118, a2119) {
        let v3: u64 = f322(v0.clone(), 0u64, {
            let a2120 = v2;
            let a2121 = 1u64;
            nat_add(a2120, a2121)?
        })?;
        let a2122 = v1;
        let a2123 = v2;
        if if nat_lt(a2122, a2123) {
            true
        } else {
            let a2126 = {
                let a2124 = v2;
                let a2125 = 1u64;
                nat_add(a2124, a2125)?
            };
            let a2127 = v3;
            nat_lt(a2126, a2127)
        } {
            f419(v0, v3)
        } else {
            Ok(v1)
        }
    } else {
        let a2128 = v1;
        let a2129 = v2;
        if nat_lt(a2128, a2129) {
            let v4: u64 = f419(v0, v2)?;
            let a2130 = v2;
            let a2131 = v4;
            if nat_lt(a2130, a2131) {
                Ok(v4)
            } else {
                Ok(v1)
            }
        } else {
            Ok(v1)
        }
    }
}

pub fn f325(v0: List<Adt21>, v1: Str) -> bool {
    let m2132 = v0;
    match m2132.uncons() {
        None => {
            false
        }
        Some((v2, _)) => {
            f420(v2, {
                let a2133 = v1;
                utf8_encode(a2133)
            })
        }
    }
}

pub fn f326(p2146: u64, p2147: List<Adt21>, p2148: List<Bytes>, v3: bool) -> Result<List<Adt21>, Bytes> {
    let m2145 = (p2146, (p2147, p2148));
    {
        let mut m2145 = m2145;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2145;
                let m2134 = v0;
                if m2134 == 0 {
                    Err::<(u64, (List<Adt21>, List<Bytes>)), Result<List<Adt21>, Bytes>>(Err::<List<Adt21>, Bytes>(f8(Str::lit("brackets nest deeper than the input is long"))))
                } else {
                    let v4 = m2134 - 1;
                    let m2135 = v1;
                    match m2135.uncons() {
                        None => {
                            Err::<(u64, (List<Adt21>, List<Bytes>)), Result<List<Adt21>, Bytes>>(Err::<List<Adt21>, Bytes>(f8(Str::lit("the input ends inside brackets or before the ; that closes a member (Web IDL, IDL grammar)"))))
                        }
                        Some((v5, v6)) => {
                            let a2136 = f421(v5.clone());
                            let a2137 = 3u64;
                            if nat_eq(a2136, a2137) {
                                Ok::<(u64, (List<Adt21>, List<Bytes>)), Result<List<Adt21>, Bytes>>((v4, (v6, v2)))
                            } else {
                                let v7: Bytes = f422(v5);
                                let v8: Bytes = f423(v7.clone());
                                if !f46(v8.clone(), Bytes::lit(&[])) {
                                    Ok::<(u64, (List<Adt21>, List<Bytes>)), Result<List<Adt21>, Bytes>>((v4, (v6, List::cons(v8, v2))))
                                } else {
                                    if f424(v7.clone()) {
                                        let m2138 = v2;
                                        match m2138.uncons() {
                                            None => {
                                                Err::<(u64, (List<Adt21>, List<Bytes>)), Result<List<Adt21>, Bytes>>(Err::<List<Adt21>, Bytes>(f7(List::cons(f8(Str::lit("the closing bracket ")), List::cons(v7, List::cons(f8(Str::lit(" has no opening bracket (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
                                            }
                                            Some((v9, v10)) => {
                                                if f46(v9.clone(), v7.clone()) {
                                                    if if !v3 {
                                                        let a2140 = {
                                                            let a2139 = v10.clone();
                                                            length_list(a2139)
                                                        };
                                                        let a2141 = 0u64;
                                                        nat_eq(a2140, a2141)
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
                                            if f46(v7, f8(Str::lit(";"))) {
                                                let a2143 = {
                                                    let a2142 = v2.clone();
                                                    length_list(a2142)
                                                };
                                                let a2144 = 0u64;
                                                nat_eq(a2143, a2144)
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
                Ok(next) => { m2145 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f327(v0: List<Adt21>) -> List<Adt21> {
    let m2149 = v0;
    match m2149.uncons() {
        None => {
            List::<Adt21>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f328(v0: u64, v1: u64, v2: List<Adt21>, v3: bool) -> R<Result<(Adt13, List<Adt21>), Bytes>> {
    let m2150 = f331(v2);
    match m2150 {
        Ok(v5) => {
            let v6: bool = if v3 {
                f325(f333(v5.clone()), Str::lit(":"))
            } else {
                false
            };
            let m2151 = if v6 {
                f331(f327(f333(v5.clone())))
            } else {
                Ok::<Adt28, Bytes>(Adt28::C0(Bytes::lit(&[]), f333(v5.clone())))
            };
            match m2151 {
                Ok(v8) => {
                    let m2152 = f332(f333(v8.clone()), Str::lit("{"));
                    match m2152 {
                        Ok(v10) => {
                            let m2157 = {
                                let a2153 = v1;
                                let a2154 = 2u64;
                                if if nat_eq(a2153, a2154) {
                                    true
                                } else {
                                    let a2155 = v1;
                                    let a2156 = 7u64;
                                    nat_eq(a2155, a2156)
                                } {
                                    f425(v0, v10, List::<Adt14>::nil())?
                                } else {
                                    f426(v0, v10, List::<Adt14>::nil())?
                                }
                            };
                            match m2157 {
                                Ok(v12) => {
                                    Ok(Ok::<(Adt13, List<Adt21>), Bytes>((Adt13::C0(v1, f335(v5), f335(v8), {
                                        let (h2158, _) = v12;
                                        h2158
                                    }, List::<Bytes>::nil(), f336(Bytes::lit(&[]))), {
                                        let (_, h2159) = v12;
                                        h2159
                                    })))
                                }
                                Err(v11) => {
                                    Ok(Err::<(Adt13, List<Adt21>), Bytes>(v11))
                                }
                            }
                        }
                        Err(v9) => {
                            Ok(Err::<(Adt13, List<Adt21>), Bytes>(v9))
                        }
                    }
                }
                Err(v7) => {
                    Ok(Err::<(Adt13, List<Adt21>), Bytes>(v7))
                }
            }
        }
        Err(v4) => {
            Ok(Err::<(Adt13, List<Adt21>), Bytes>(v4))
        }
    }
}

pub fn f329(v0: List<Adt21>, v1: Bytes) -> R<Result<(Adt13, List<Adt21>), Bytes>> {
    let m2160 = f331(v0);
    match m2160 {
        Ok(v3) => {
            let m2161 = f427(f333(v3.clone()))?;
            match m2161 {
                Ok(v5) => {
                    Ok(Ok::<(Adt13, List<Adt21>), Bytes>((Adt13::C0(9u64, f335(v3), Bytes::lit(&[]), List::<Adt14>::nil(), List::cons(v1, List::<Bytes>::nil()), f336(Bytes::lit(&[]))), v5)))
                }
                Err(v4) => {
                    Ok(Err::<(Adt13, List<Adt21>), Bytes>(v4))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<(Adt13, List<Adt21>), Bytes>(v2))
        }
    }
}

pub fn f330(v0: Str, v1: List<Adt21>) -> Bytes {
    f7(List::cons(f8(Str::lit("expected ")), List::cons({
        let a2162 = v0;
        utf8_encode(a2162)
    }, List::cons(f8(Str::lit(" but found ")), List::cons(f428(v1), List::cons(f8(Str::lit(" (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
}

pub fn f331(v0: List<Adt21>) -> Result<Adt28, Bytes> {
    let m2163 = v0.clone();
    match m2163.uncons() {
        None => {
            Err::<Adt28, Bytes>(f330(Str::lit("an identifier"), v0))
        }
        Some((v1, v2)) => {
            let a2164 = f421(v1.clone());
            let a2165 = 0u64;
            if nat_eq(a2164, a2165) {
                Ok::<Adt28, Bytes>(Adt28::C0(f429(f422(v1)), v2))
            } else {
                Err::<Adt28, Bytes>(f330(Str::lit("an identifier"), v0))
            }
        }
    }
}

pub fn f332(v0: List<Adt21>, v1: Str) -> Result<List<Adt21>, Bytes> {
    if f325(v0.clone(), v1.clone()) {
        Ok::<List<Adt21>, Bytes>(f327(v0))
    } else {
        Err::<List<Adt21>, Bytes>(f330(v1, v0))
    }
}

pub fn f333(v0: Adt28) -> List<Adt21> {
    let m2166 = v0;
    match m2166 {
        Adt28::C0(_, v2) => {
            v2
        }
    }
}

pub fn f334(p2174: u64, p2175: List<Adt21>, p2176: List<Bytes>) -> Result<(List<Bytes>, List<Adt21>), Bytes> {
    let m2173 = (p2174, (p2175, p2176));
    {
        let mut m2173 = m2173;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2173;
                let m2167 = v0;
                if m2167 == 0 {
                    Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Err::<(List<Bytes>, List<Adt21>), Bytes>(f8(Str::lit("an enumeration has more values than the input has tokens"))))
                } else {
                    let v3 = m2167 - 1;
                    let m2168 = v1.clone();
                    match m2168.uncons() {
                        None => {
                            Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Err::<(List<Bytes>, List<Adt21>), Bytes>(f330(Str::lit("a string"), v1)))
                        }
                        Some((v4, v5)) => {
                            let a2169 = f421(v4.clone());
                            let a2170 = 3u64;
                            if !nat_eq(a2169, a2170) {
                                Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Err::<(List<Bytes>, List<Adt21>), Bytes>(f330(Str::lit("a string"), v1)))
                            } else {
                                if f325(v5.clone(), Str::lit(",")) {
                                    if f325(f327(v5.clone()), Str::lit("}")) {
                                        let m2171 = f332(f327(f327(v5)), Str::lit(";"));
                                        match m2171 {
                                            Ok(v7) => {
                                                Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Ok::<(List<Bytes>, List<Adt21>), Bytes>((f58(List::cons(f422(v4), v2)), v7)))
                                            }
                                            Err(v6) => {
                                                Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Err::<(List<Bytes>, List<Adt21>), Bytes>(v6))
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>((v3, (f327(v5), List::cons(f422(v4), v2))))
                                    }
                                } else {
                                    if f325(v5.clone(), Str::lit("}")) {
                                        let m2172 = f332(f327(v5), Str::lit(";"));
                                        match m2172 {
                                            Ok(v9) => {
                                                Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Ok::<(List<Bytes>, List<Adt21>), Bytes>((f58(List::cons(f422(v4), v2)), v9)))
                                            }
                                            Err(v8) => {
                                                Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Err::<(List<Bytes>, List<Adt21>), Bytes>(v8))
                                            }
                                        }
                                    } else {
                                        Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Err::<(List<Bytes>, List<Adt21>), Bytes>(f330(Str::lit(", or }"), v5)))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2173 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f335(v0: Adt28) -> Bytes {
    let m2177 = v0;
    match m2177 {
        Adt28::C0(v1, _) => {
            v1
        }
    }
}

pub fn f336(v0: Bytes) -> Adt15 {
    Adt15::C0(0u64, v0, List::<Bytes>::nil(), Bytes::lit(&[]), Bytes::lit(&[]))
}

pub fn f337(v0: u64, v1: List<Adt21>, v2: u64) -> R<Result<Adt29, Bytes>> {
    let m2178 = v0;
    if m2178 == 0 {
        Ok(Err::<Adt29, Bytes>(f8(Str::lit("a type nests deeper than the input is long"))))
    } else {
        let v3 = m2178 - 1;
        let a2179 = v2;
        let a2180 = 0u64;
        if nat_eq(a2179, a2180) {
            let m2181 = f226(v1)?;
            match m2181 {
                Ok(v5) => {
                    if f325(v5.clone(), Str::lit("(")) {
                        let m2182 = f337(v3, f327(v5), 1u64)?;
                        match m2182 {
                            Ok(v7) => {
                                if f325(f338(v7.clone()), Str::lit("?")) {
                                    Ok(Ok::<Adt29, Bytes>(Adt29::C0(Adt15::C0(0u64, Bytes::lit(&[]), f245(f339(v7.clone())), f8(Str::lit("a nullable union type is not translated yet, because the typedef identifier names only the union (Web IDL 2.13.32)")), Bytes::lit(&[])), f327(f338(v7)))))
                                } else {
                                    Ok(Ok::<Adt29, Bytes>(Adt29::C0(f339(v7.clone()), f338(v7))))
                                }
                            }
                            Err(v6) => {
                                Ok(Err::<Adt29, Bytes>(v6))
                            }
                        }
                    } else {
                        let m2183 = v5.clone();
                        match m2183.uncons() {
                            None => {
                                Ok(Err::<Adt29, Bytes>(f330(Str::lit("a type"), v5)))
                            }
                            Some((v8, v9)) => {
                                let v10: Bytes = f422(v8.clone());
                                let a2184 = f421(v8);
                                let a2185 = 0u64;
                                if !nat_eq(a2184, a2185) {
                                    Ok(Err::<Adt29, Bytes>(f330(Str::lit("a type"), v5)))
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
                                        let m2186 = f332(v9, Str::lit("<"));
                                        match m2186 {
                                            Ok(v12) => {
                                                let m2187 = f337(v3, v12, 0u64)?;
                                                match m2187 {
                                                    Ok(v14) => {
                                                        let m2188 = f332(f338(v14.clone()), Str::lit(">"));
                                                        match m2188 {
                                                            Ok(v16) => {
                                                                Ok(f430(f431(Str::lit("List"), f339(v14)), v16))
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
                                        if f46(v10.clone(), f8(Str::lit("async_sequence"))) {
                                            let m2189 = f332(v9, Str::lit("<"));
                                            match m2189 {
                                                Ok(v18) => {
                                                    let m2190 = f337(v3, v18, 0u64)?;
                                                    match m2190 {
                                                        Ok(v20) => {
                                                            let m2191 = f332(f338(v20), Str::lit(">"));
                                                            match m2191 {
                                                                Ok(v22) => {
                                                                    Ok(f430(f432(f8(Str::lit("async_sequence types are not translated, because an async sequence is produced over time and is not a value (Web IDL 2.13.29)"))), v22))
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
                                            if f46(v10.clone(), f8(Str::lit("Promise"))) {
                                                let m2192 = f332(v9, Str::lit("<"));
                                                match m2192 {
                                                    Ok(v24) => {
                                                        let m2193 = f337(v3, v24, 0u64)?;
                                                        match m2193 {
                                                            Ok(v26) => {
                                                                let m2194 = f332(f338(v26), Str::lit(">"));
                                                                match m2194 {
                                                                    Ok(v28) => {
                                                                        Ok(f430(f432(f8(Str::lit("Promise types are not translated, because a promise is a deferred result, not a value (Web IDL 2.13.31)"))), v28))
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
                                                if f46(v10.clone(), f8(Str::lit("record"))) {
                                                    let m2195 = f332(v9, Str::lit("<"));
                                                    match m2195 {
                                                        Ok(v30) => {
                                                            let m2196 = v30.clone();
                                                            match m2196.uncons() {
                                                                None => {
                                                                    Ok(Err::<Adt29, Bytes>(f330(Str::lit("a string type"), v30)))
                                                                }
                                                                Some((v31, v32)) => {
                                                                    if if f46(f422(v31.clone()), f8(Str::lit("DOMString"))) {
                                                                        true
                                                                    } else {
                                                                        if f46(f422(v31.clone()), f8(Str::lit("USVString"))) {
                                                                            true
                                                                        } else {
                                                                            f46(f422(v31.clone()), f8(Str::lit("ByteString")))
                                                                        }
                                                                    } {
                                                                        let m2197 = f332(v32, Str::lit(","));
                                                                        match m2197 {
                                                                            Ok(v34) => {
                                                                                let m2198 = f337(v3, v34, 0u64)?;
                                                                                match m2198 {
                                                                                    Ok(v36) => {
                                                                                        let m2199 = f332(f338(v36.clone()), Str::lit(">"));
                                                                                        match m2199 {
                                                                                            Ok(v38) => {
                                                                                                let m2200 = f433(f422(v31));
                                                                                                match m2200 {
                                                                                                    None => {
                                                                                                        Ok(Err::<Adt29, Bytes>(f330(Str::lit("a string type"), v30)))
                                                                                                    }
                                                                                                    Some(v39) => {
                                                                                                        let v40: Adt15 = f434(f339(v36));
                                                                                                        Ok(f430(Adt15::C0(0u64, f7(List::cons(f8(Str::lit("(List (")), List::cons(f244(v39), List::cons(f8(Str::lit(" * ")), List::cons(f244(v40.clone()), List::cons(f8(Str::lit("))")), List::<Bytes>::nil())))))), f245(v40.clone()), f242(v40), Bytes::lit(&[])), v38))
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
                                                                        Ok(Err::<Adt29, Bytes>(f330(Str::lit("DOMString, USVString or ByteString"), v30)))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Err(v29) => {
                                                            Ok(Err::<Adt29, Bytes>(v29))
                                                        }
                                                    }
                                                } else {
                                                    if f46(v10.clone(), f8(Str::lit("unsigned"))) {
                                                        if f325(v9.clone(), Str::lit("short")) {
                                                            Ok(f430(f336(f8(Str::lit("Nat"))), f327(v9)))
                                                        } else {
                                                            if f325(v9.clone(), Str::lit("long")) {
                                                                let v41: List<Adt21> = f327(v9);
                                                                Ok(f430(f336(f8(Str::lit("Nat"))), if f325(v41.clone(), Str::lit("long")) {
                                                                    f327(v41)
                                                                } else {
                                                                    v41
                                                                }))
                                                            } else {
                                                                Ok(Err::<Adt29, Bytes>(f330(Str::lit("short or long"), v9)))
                                                            }
                                                        }
                                                    } else {
                                                        if f46(v10.clone(), f8(Str::lit("short"))) {
                                                            Ok(f430(f336(f8(Str::lit("Int"))), v9))
                                                        } else {
                                                            if f46(v10.clone(), f8(Str::lit("long"))) {
                                                                Ok(f430(f336(f8(Str::lit("Int"))), if f325(v9.clone(), Str::lit("long")) {
                                                                    f327(v9)
                                                                } else {
                                                                    v9
                                                                }))
                                                            } else {
                                                                if f46(v10.clone(), f8(Str::lit("unrestricted"))) {
                                                                    if if f325(v9.clone(), Str::lit("float")) {
                                                                        true
                                                                    } else {
                                                                        f325(v9.clone(), Str::lit("double"))
                                                                    } {
                                                                        Ok(f430(f432(f8(Str::lit("unrestricted float and unrestricted double are not translated, because its value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.13 and 2.13.15)"))), f327(v9)))
                                                                    } else {
                                                                        Ok(Err::<Adt29, Bytes>(f330(Str::lit("float or double"), v9)))
                                                                    }
                                                                } else {
                                                                    let m2201 = f433(v10.clone());
                                                                    match m2201 {
                                                                        None => {
                                                                            let v42: Bytes = f429(v10);
                                                                            Ok(f430(Adt15::C0(0u64, f229(v42.clone())?, List::cons(v42.clone(), List::<Bytes>::nil()), Bytes::lit(&[]), v42), v9))
                                                                        }
                                                                        Some(v43) => {
                                                                            Ok(f430(v43, v9))
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
            let m2202 = f337(v3, v1, 0u64)?;
            match m2202 {
                Ok(v45) => {
                    let v46: Adt15 = f339(v45.clone());
                    let v48: Adt15 = {
                        let a2203 = f243(v46.clone());
                        let a2204 = 0u64;
                        if nat_eq(a2203, a2204) {
                            if f46(f242(v46.clone()), Bytes::lit(&[])) {
                                if f46(f435(v46.clone()), Bytes::lit(&[])) {
                                    Adt15::C0(1u64, Bytes::lit(&[]), f245(v46), f8(Str::lit("a union member type that is not a named definition gives its constructor no name, so the union is not translated yet (Web IDL 2.13.32)")), Bytes::lit(&[]))
                                } else {
                                    let m2205 = f349(false, f435(v46.clone()))?;
                                    match m2205 {
                                        None => {
                                            Adt15::C0(1u64, Bytes::lit(&[]), f245(v46.clone()), f7(List::cons(f8(Str::lit("the union member ")), List::cons(f435(v46), List::cons(f8(Str::lit(" has no constructor name")), List::<Bytes>::nil())))), Bytes::lit(&[]))
                                        }
                                        Some(v47) => {
                                            Adt15::C0(1u64, f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v47, List::cons(f8(Str::lit(", fields: [")), List::cons(f244(v46.clone()), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f245(v46), Bytes::lit(&[]), Bytes::lit(&[]))
                                        }
                                    }
                                }
                            } else {
                                Adt15::C0(1u64, Bytes::lit(&[]), f245(v46.clone()), f242(v46), Bytes::lit(&[]))
                            }
                        } else {
                            v46
                        }
                    };
                    if f325(f338(v45.clone()), Str::lit("or")) {
                        let m2206 = f337(v3, f327(f338(v45)), 2u64)?;
                        match m2206 {
                            Ok(v50) => {
                                Ok(Ok::<Adt29, Bytes>(Adt29::C0(Adt15::C0(1u64, if f46(f244(v48.clone()), Bytes::lit(&[])) {
                                    f244(f339(v50.clone()))
                                } else {
                                    if f46(f244(f339(v50.clone())), Bytes::lit(&[])) {
                                        f244(v48.clone())
                                    } else {
                                        f7(List::cons(f244(v48.clone()), List::cons(f8(Str::lit(",\u{a}")), List::cons(f244(f339(v50.clone())), List::<Bytes>::nil()))))
                                    }
                                }, f156(f245(v48.clone()), f245(f339(v50.clone()))), if f46(f242(v48.clone()), Bytes::lit(&[])) {
                                    f242(f339(v50.clone()))
                                } else {
                                    f242(v48)
                                }, Bytes::lit(&[])), f338(v50))))
                            }
                            Err(v49) => {
                                Ok(Err::<Adt29, Bytes>(v49))
                            }
                        }
                    } else {
                        let a2207 = v2;
                        let a2208 = 2u64;
                        if if nat_eq(a2207, a2208) {
                            f325(f338(v45.clone()), Str::lit(")"))
                        } else {
                            false
                        } {
                            Ok(Ok::<Adt29, Bytes>(Adt29::C0(v48, f327(f338(v45)))))
                        } else {
                            Ok(Err::<Adt29, Bytes>({
                                let a2209 = v2;
                                let a2210 = 1u64;
                                if nat_eq(a2209, a2210) {
                                    f330(Str::lit("or"), f338(v45))
                                } else {
                                    f330(Str::lit("or or )"), f338(v45))
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

pub fn f338(v0: Adt29) -> List<Adt21> {
    let m2211 = v0;
    match m2211 {
        Adt29::C0(_, v2) => {
            v2
        }
    }
}

pub fn f339(v0: Adt29) -> Adt15 {
    let m2212 = v0;
    match m2212 {
        Adt29::C0(v1, _) => {
            v1
        }
    }
}

pub fn f340() -> List<Bytes> {
    List::cons(f8(Str::lit("Bool")), List::cons(f8(Str::lit("ContractViolation")), List::cons(f8(Str::lit("List")), List::cons(f8(Str::lit("Nat")), List::cons(f8(Str::lit("Option")), List::cons(f8(Str::lit("Prod")), List::cons(f8(Str::lit("ReasoningFailure")), List::cons(f8(Str::lit("Result")), List::<Bytes>::nil()))))))))
}

pub fn f341(v0: Bytes) -> R<Bytes> {
    let m2215 = {
        let a2213 = v0.clone();
        let a2214 = 0u64;
        index_bytes(a2213, a2214)
    };
    match m2215 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            let a2221 = {
                let a2216 = f8(Str::lit("u"));
                let a2217 = f436(8u64, f52(v1)?, 4u64, Bytes::lit(&[]));
                append_bytes(a2216, a2217)
            };
            let a2222 = f43(v0.clone(), 1u64, {
                let a2219 = {
                    let a2218 = v0;
                    length_bytes(a2218)
                };
                let a2220 = 1u64;
                nat_sub(a2219, a2220)
            });
            Ok(append_bytes(a2221, a2222))
        }
    }
}

pub fn f342(v0: List<Adt14>) -> R<List<Adt14>> {
    let m2223 = v0;
    match m2223.uncons() {
        None => {
            Ok(List::<Adt14>::nil())
        }
        Some((v1, v2)) => {
            f437(v1, f342(v2)?)
        }
    }
}

pub fn f343(v0: List<Adt14>, v1: List<Adt14>) -> List<Adt14> {
    let m2224 = v0;
    match m2224.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f343(v3, v1))
        }
    }
}

pub fn f344(v0: Adt13) -> List<Adt14> {
    let m2225 = v0;
    match m2225 {
        Adt13::C0(_, _, _, v4, _, _) => {
            v4
        }
    }
}

pub fn f345(v0: List<Adt13>, v1: u64, v2: Bytes) -> List<Adt14> {
    let m2226 = v0;
    match m2226.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v3, v4)) => {
            let a2227 = f150(v3.clone());
            let a2228 = v1;
            if if nat_eq(a2227, a2228) {
                f46(f152(v3.clone()), v2.clone())
            } else {
                false
            } {
                f343(f344(v3), f345(v4, v1, v2))
            } else {
                f345(v4, v1, v2)
            }
        }
    }
}

pub fn f346(v0: List<Adt13>, v1: List<Adt13>, v2: Bytes) -> List<Adt14> {
    let m2229 = v0;
    match m2229.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v3, v4)) => {
            let a2230 = f150(v3.clone());
            let a2231 = 8u64;
            if if nat_eq(a2230, a2231) {
                f46(f152(v3.clone()), v2.clone())
            } else {
                false
            } {
                f343(f438(v1.clone(), f163(v3)), f346(v4, v1, v2))
            } else {
                f346(v4, v1, v2)
            }
        }
    }
}

pub fn f347(v0: Adt14) -> Adt15 {
    let m2232 = v0;
    match m2232 {
        Adt14::C0(_, v2, _) => {
            v2
        }
    }
}

pub fn f348(v0: Adt14) -> Bytes {
    let m2233 = v0;
    match m2233 {
        Adt14::C0(v1, _, _) => {
            v1
        }
    }
}

pub fn f349(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f439(v1)
    } else {
        f352(v1)
    }
}

pub fn f350(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m2234 = v1;
    match m2234.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m2235 = f349(v0, v2)?;
            match m2235 {
                None => {
                    f350(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f350(v0, v3)?))
                }
            }
        }
    }
}

pub fn f351(v0: Adt14) -> u64 {
    let m2236 = v0;
    match m2236 {
        Adt14::C0(_, _, v3) => {
            v3
        }
    }
}

pub fn f352(v0: Bytes) -> R<Option<Bytes>> {
    let m2237 = f440(v0)?;
    match m2237 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            Ok(Some(f441(v1)?))
        }
    }
}

pub fn f353(v0: bool, v1: Adt2) -> R<Option<Bytes>> {
    let m2238 = v1;
    match m2238 {
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
                let a2249 = if f85(v7, {
                    let a2247 = 0u64;
                    let a2248 = v8;
                    nat_lt(a2247, a2248)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a2250 = f305(false, v8)?;
                append_bytes(a2249, a2250)
            }))
        }
        Adt2::C4(v3, v4, v5) => {
            let m2241 = f439({
                let a2239 = f8(Str::lit("n"));
                let a2240 = f306(false, v4, v5)?;
                append_bytes(a2239, a2240)
            })?;
            match m2241 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a2245 = f8(Str::lit("nMinus"));
                        let a2246 = f43(v6.clone(), 1u64, {
                            let a2243 = {
                                let a2242 = v6;
                                length_bytes(a2242)
                            };
                            let a2244 = 1u64;
                            nat_sub(a2243, a2244)
                        });
                        append_bytes(a2245, a2246)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt2::C5(v2) => {
            f349(v0, v2)
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

pub fn f354(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m2253 = {
        let a2251 = v0.clone();
        let a2252 = v1;
        index_bytes(a2251, a2252)
    };
    match m2253 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f52(v2)?;
            let a2254 = v3;
            let a2255 = 128u64;
            if nat_lt(a2254, a2255) {
                Ok(Some((v3, 1u64)))
            } else {
                if f85({
                    let a2256 = 194u64;
                    let a2257 = v3;
                    nat_le(a2256, a2257)
                }, {
                    let a2258 = v3;
                    let a2259 = 223u64;
                    nat_le(a2258, a2259)
                }) {
                    let m2262 = f442(v0, {
                        let a2260 = v1;
                        let a2261 = 1u64;
                        nat_add(a2260, a2261)?
                    })?;
                    match m2262 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f443({
                                let a2267 = {
                                    let a2265 = {
                                        let a2263 = v3;
                                        let a2264 = 192u64;
                                        nat_sub(a2263, a2264)
                                    };
                                    let a2266 = 64u64;
                                    nat_mul(a2265, a2266)?
                                };
                                let a2268 = v4;
                                nat_add(a2267, a2268)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f85({
                        let a2269 = 224u64;
                        let a2270 = v3;
                        nat_le(a2269, a2270)
                    }, {
                        let a2271 = v3;
                        let a2272 = 239u64;
                        nat_le(a2271, a2272)
                    }) {
                        let m2275 = f442(v0.clone(), {
                            let a2273 = v1;
                            let a2274 = 1u64;
                            nat_add(a2273, a2274)?
                        })?;
                        match m2275 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m2278 = f442(v0, {
                                    let a2276 = v1;
                                    let a2277 = 2u64;
                                    nat_add(a2276, a2277)?
                                })?;
                                match m2278 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f443({
                                            let a2287 = {
                                                let a2281 = {
                                                    let a2279 = v3;
                                                    let a2280 = 224u64;
                                                    nat_sub(a2279, a2280)
                                                };
                                                let a2282 = 4096u64;
                                                nat_mul(a2281, a2282)?
                                            };
                                            let a2288 = {
                                                let a2285 = {
                                                    let a2283 = v5;
                                                    let a2284 = 64u64;
                                                    nat_mul(a2283, a2284)?
                                                };
                                                let a2286 = v6;
                                                nat_add(a2285, a2286)?
                                            };
                                            nat_add(a2287, a2288)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f85({
                            let a2289 = 240u64;
                            let a2290 = v3;
                            nat_le(a2289, a2290)
                        }, {
                            let a2291 = v3;
                            let a2292 = 244u64;
                            nat_le(a2291, a2292)
                        }) {
                            let m2295 = f442(v0.clone(), {
                                let a2293 = v1;
                                let a2294 = 1u64;
                                nat_add(a2293, a2294)?
                            })?;
                            match m2295 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m2298 = f442(v0.clone(), {
                                        let a2296 = v1;
                                        let a2297 = 2u64;
                                        nat_add(a2296, a2297)?
                                    })?;
                                    match m2298 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m2301 = f442(v0, {
                                                let a2299 = v1;
                                                let a2300 = 3u64;
                                                nat_add(a2299, a2300)?
                                            })?;
                                            match m2301 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f443({
                                                        let a2314 = {
                                                            let a2304 = {
                                                                let a2302 = v3;
                                                                let a2303 = 240u64;
                                                                nat_sub(a2302, a2303)
                                                            };
                                                            let a2305 = 262144u64;
                                                            nat_mul(a2304, a2305)?
                                                        };
                                                        let a2315 = {
                                                            let a2312 = {
                                                                let a2306 = v7;
                                                                let a2307 = 4096u64;
                                                                nat_mul(a2306, a2307)?
                                                            };
                                                            let a2313 = {
                                                                let a2310 = {
                                                                    let a2308 = v8;
                                                                    let a2309 = 64u64;
                                                                    nat_mul(a2308, a2309)?
                                                                };
                                                                let a2311 = v9;
                                                                nat_add(a2310, a2311)?
                                                            };
                                                            nat_add(a2312, a2313)?
                                                        };
                                                        nat_add(a2314, a2315)?
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

pub fn f355(v0: u64) -> R<bool> {
    f250(v0, f444())
}

pub fn f356(v0: u64, v1: Bytes, p2326: u64, p2327: u64, p2328: bool) -> R<bool> {
    let m2325 = (p2326, (p2327, p2328));
    {
        let mut m2325 = m2325;
        loop {
            let step = {
                let (v2, (v3, v4)) = m2325;
                let m2316 = v3;
                if m2316 == 0 {
                    Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                } else {
                    let v5 = m2316 - 1;
                    let v6: u64 = f445(v1.clone(), v2)?;
                    if v4 {
                        let a2317 = v0;
                        let a2318 = v6;
                        if nat_lt(a2317, a2318) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a2319 = v2;
                                let a2320 = 1u64;
                                nat_add(a2319, a2320)?
                            }, (v5, false)))
                        }
                    } else {
                        let a2321 = v0;
                        let a2322 = v6;
                        if nat_le(a2321, a2322) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(true))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a2323 = v2;
                                let a2324 = 1u64;
                                nat_add(a2323, a2324)?
                            }, (v5, true)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2325 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f357(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    if f107(v0.clone(), v1, v2) {
        Ok(None::<u64>)
    } else {
        let a2330 = v2;
        let a2331 = {
            let a2329 = v0;
            length_bytes(a2329)
        };
        if nat_lt(a2330, a2331) {
            Ok(Some({
                let a2332 = v2;
                let a2333 = 1u64;
                nat_add(a2332, a2333)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f358(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f170(f115(v0, v1)?)? {
        Ok(Some({
            let a2334 = v1;
            let a2335 = 1u64;
            nat_add(a2334, a2335)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f359(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let m2336 = f354(v0, v2)?;
    match m2336 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            let a2337 = v2;
            let a2338 = v1;
            if if nat_eq(a2337, a2338) {
                f446({
                    let (h2339, _) = v3;
                    h2339
                })?
            } else {
                f447({
                    let (h2340, _) = v3;
                    h2340
                })?
            } {
                Ok(Some({
                    let a2342 = v2;
                    let a2343 = {
                        let (_, h2341) = v3;
                        h2341
                    };
                    nat_add(a2342, a2343)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f360(v0: Bytes, v1: bool, v2: u64) -> R<(u64, u64)> {
    let (h2344, _) = f220(Fn5::F448(v1, v0.clone()), {
        let a2346 = {
            let a2345 = v0;
            length_bytes(a2345)
        };
        let a2347 = 1u64;
        nat_add(a2346, a2347)?
    }, (v2, 0u64))?;
    Ok(h2344)
}

pub fn f361(v0: u64) -> R<Bytes> {
    let a2348 = v0;
    let a2349 = 128u64;
    if nat_lt(a2348, a2349) {
        Ok(f449(f450(), v0))
    } else {
        let a2350 = v0;
        let a2351 = 2048u64;
        if nat_lt(a2350, a2351) {
            let a2359 = f449(f451(), {
                let a2357 = {
                    let a2355 = {
                        let a2352 = v0;
                        let a2353 = 64u64;
                        let a2354 = 0u64;
                        nat_quot(a2352, a2353, a2354)
                    };
                    let a2356 = 2u64;
                    nat_sub(a2355, a2356)
                };
                let a2358 = 2u64;
                nat_mul(a2357, a2358)?
            });
            let a2360 = f452(v0)?;
            Ok(append_bytes(a2359, a2360))
        } else {
            let a2361 = v0;
            let a2362 = 65536u64;
            if nat_lt(a2361, a2362) {
                let a2373 = f449(f453(), {
                    let a2366 = {
                        let a2363 = v0;
                        let a2364 = 4096u64;
                        let a2365 = 0u64;
                        nat_quot(a2363, a2364, a2365)
                    };
                    let a2367 = 3u64;
                    nat_mul(a2366, a2367)?
                });
                let a2374 = {
                    let a2371 = f452({
                        let a2368 = v0;
                        let a2369 = 64u64;
                        let a2370 = 0u64;
                        nat_quot(a2368, a2369, a2370)
                    })?;
                    let a2372 = f452(v0)?;
                    append_bytes(a2371, a2372)
                };
                Ok(append_bytes(a2373, a2374))
            } else {
                let a2388 = f449(Bytes::lit(&[0xf0, 0xf1, 0xf2, 0xf3, 0xf4]), {
                    let a2375 = v0;
                    let a2376 = 262144u64;
                    let a2377 = 0u64;
                    nat_quot(a2375, a2376, a2377)
                });
                let a2389 = {
                    let a2386 = f452({
                        let a2378 = v0;
                        let a2379 = 4096u64;
                        let a2380 = 0u64;
                        nat_quot(a2378, a2379, a2380)
                    })?;
                    let a2387 = {
                        let a2384 = f452({
                            let a2381 = v0;
                            let a2382 = 64u64;
                            let a2383 = 0u64;
                            nat_quot(a2381, a2382, a2383)
                        })?;
                        let a2385 = f452(v0)?;
                        append_bytes(a2384, a2385)
                    };
                    append_bytes(a2386, a2387)
                };
                Ok(append_bytes(a2388, a2389))
            }
        }
    }
}

pub fn f362(v0: bool, v1: u64, v2: Bytes, v3: u64) -> R<Option<u64>> {
    f454(v2, v0, v1, v3)
}

pub fn f363(v0: Bytes) -> Result<Option<Adt27>, Bytes> {
    Err::<Option<Adt27>, Bytes>(v0)
}

pub fn f364(p2393: Bytes, p2394: u64) -> R<Result<Bytes, Bytes>> {
    let m2392 = (p2393, p2394);
    {
        let mut m2392 = m2392;
        loop {
            let step = {
                let (v0, v1) = m2392;
                let m2390 = v1;
                if m2390 == 0 {
                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f455(f8(Str::lit("the built-in datatype hierarchy is deeper than XSD 1.1 Part 2 section 3 defines")))))
                } else {
                    let v2 = m2390 - 1;
                    if f254(v0.clone(), List::cons(f8(Str::lit("NMTOKENS")), List::cons(f8(Str::lit("IDREFS")), List::cons(f8(Str::lit("ENTITIES")), List::<Bytes>::nil())))) {
                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("(List Bytes)")))))
                    } else {
                        if f46(v0.clone(), f8(Str::lit("nonNegativeInteger"))) {
                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Nat")))))
                        } else {
                            if f46(v0.clone(), f8(Str::lit("integer"))) {
                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Int")))))
                            } else {
                                if f46(v0.clone(), f8(Str::lit("decimal"))) {
                                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f456())))
                                } else {
                                    if f46(v0.clone(), f8(Str::lit("boolean"))) {
                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Bool")))))
                                    } else {
                                        if f254(v0.clone(), List::cons(f8(Str::lit("float")), List::cons(f8(Str::lit("double")), List::<Bytes>::nil()))) {
                                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f455(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because its value space holds positive and negative zero, the infinities and NaN, which no exact LexLean number type holds (XSD 1.1 Part 2, sections 3.3.4 and 3.3.5)")), List::<Bytes>::nil())))))))
                                        } else {
                                            if f254(v0.clone(), List::cons(f8(Str::lit("anySimpleType")), List::cons(f8(Str::lit("anyAtomicType")), List::<Bytes>::nil()))) {
                                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f455(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because it is the union of every simple value space and has no one LexLean type (XSD 1.1 Part 2, sections 3.2.1 and 3.2.2)")), List::<Bytes>::nil())))))))
                                            } else {
                                                let m2391 = f457(v0.clone())?;
                                                match m2391 {
                                                    None => {
                                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f455(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not a built-in datatype of XSD 1.1 Part 2")), List::<Bytes>::nil())))))))
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
                Ok(next) => { m2392 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f365(v0: Bytes) -> Result<List<Adt2>, Bytes> {
    Err::<List<Adt2>, Bytes>(v0)
}

pub fn f366(v0: Option<Adt2>) -> Option<Bytes> {
    let m2395 = v0;
    match m2395 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m2396 = v1;
            match m2396 {
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

pub fn f367(v0: u64) -> bool {
    f139({
        let a2397 = v0;
        let a2398 = 95u64;
        nat_eq(a2397, a2398)
    }, f139({
        let a2399 = v0;
        let a2400 = 45u64;
        nat_eq(a2399, a2400)
    }, f139({
        let a2401 = v0;
        let a2402 = 32u64;
        nat_eq(a2401, a2402)
    }, f139({
        let a2403 = v0;
        let a2404 = 46u64;
        nat_eq(a2403, a2404)
    }, {
        let a2405 = v0;
        let a2406 = 47u64;
        nat_eq(a2405, a2406)
    }))))
}

pub fn f368(v0: u64) -> bool {
    f85({
        let a2407 = 97u64;
        let a2408 = v0;
        nat_le(a2407, a2408)
    }, {
        let a2409 = v0;
        let a2410 = 122u64;
        nat_le(a2409, a2410)
    })
}

pub fn f369(v0: u64) -> bool {
    f85({
        let a2411 = 65u64;
        let a2412 = v0;
        nat_le(a2411, a2412)
    }, {
        let a2413 = v0;
        let a2414 = 90u64;
        nat_le(a2413, a2414)
    })
}

pub fn f370(v0: u64) -> bool {
    f85({
        let a2415 = 48u64;
        let a2416 = v0;
        nat_le(a2415, a2416)
    }, {
        let a2417 = v0;
        let a2418 = 57u64;
        nat_le(a2417, a2418)
    })
}

pub fn f371(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h2419, _) = v1.clone();
        h2419
    };
    let v3: u64 = {
        let (h2420, _) = {
            let (_, h2421) = v1.clone();
            h2421
        };
        h2420
    };
    let v4: List<Bytes> = {
        let (_, h2422) = {
            let (_, h2423) = v1;
            h2423
        };
        h2422
    };
    let m2424 = f144(v0.clone(), v2);
    match m2424 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a2425 = v5;
            let a2426 = 10u8;
            if equal(a2425, a2426) {
                Ok(Some(({
                    let a2427 = v2;
                    let a2428 = 1u64;
                    nat_add(a2427, a2428)?
                }, ({
                    let a2429 = v2;
                    let a2430 = 1u64;
                    nat_add(a2429, a2430)?
                }, List::cons(f458(f43(v0, v3, {
                    let a2431 = v2;
                    let a2432 = v3;
                    nat_sub(a2431, a2432)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a2433 = v2;
                    let a2434 = 1u64;
                    nat_add(a2433, a2434)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f372(v0: List<Adt24>, v1: List<Adt24>) -> List<Adt24> {
    let m2435 = v0;
    match m2435.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f372(v3, v1))
        }
    }
}

pub fn f373(v0: List<Adt25>, v1: Bytes) -> List<Adt24> {
    let m2436 = v0;
    match m2436.uncons() {
        None => {
            List::<Adt24>::nil()
        }
        Some((v2, v3)) => {
            if f153(match v2.clone() {
                Adt25::C0(_, h2437, _, _, _) => {
                    h2437
                }
            }, f459()) {
                let m2438 = f284(v2.clone(), Str::lit("name"));
                match m2438 {
                    None => {
                        f373(v3, v1)
                    }
                    Some(v4) => {
                        List::cons(Adt24::C0(match v2.clone() {
                            Adt25::C0(_, h2439, _, _, _) => {
                                h2439
                            }
                        }, v1.clone(), v4, v2), f373(v3, v1))
                    }
                }
            } else {
                f373(v3, v1)
            }
        }
    }
}

pub fn f374(v0: u64, v1: Adt23, v2: Adt24) -> R<Result<List<Adt26>, Bytes>> {
    let v3: Bytes = f191(match v2.clone() {
        Adt24::C0(_, _, h2440, _) => {
            h2440
        }
    })?;
    let v4: Adt25 = match v2.clone() {
        Adt24::C0(_, _, _, h2441) => {
            h2441
        }
    };
    let v5: Bytes = match v2.clone() {
        Adt24::C0(_, h2442, _, _) => {
            h2442
        }
    };
    let v6: Bytes = match v2.clone() {
        Adt24::C0(h2443, _, _, _) => {
            h2443
        }
    };
    if f46(v6.clone(), f8(Str::lit("complexType"))) {
        f460(v0, v1, v2)
    } else {
        if f46(v6.clone(), f8(Str::lit("simpleType"))) {
            let m2444 = f461(v3, v4, v5)?;
            match m2444 {
                Ok(v8) => {
                    Ok(Ok::<List<Adt26>, Bytes>(List::cons(v8, List::<Adt26>::nil())))
                }
                Err(v7) => {
                    Ok(Err::<List<Adt26>, Bytes>(v7))
                }
            }
        } else {
            if f46(v6.clone(), f8(Str::lit("element"))) {
                let m2445 = f284(v4.clone(), Str::lit("type"));
                match m2445 {
                    None => {
                        let m2446 = f462(v0, v1, v3, v5, v4)?;
                        match m2446 {
                            Ok(v11) => {
                                Ok(Ok::<List<Adt26>, Bytes>(match v11 {
                                    Adt31::C0(_, h2447) => {
                                        h2447
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
                if f46(v6.clone(), f8(Str::lit("attribute"))) {
                    let m2448 = f284(v4.clone(), Str::lit("type"));
                    match m2448 {
                        None => {
                            let m2449 = f463(v1, v5, v3, v4)?;
                            match m2449 {
                                Ok(v14) => {
                                    Ok(Ok::<List<Adt26>, Bytes>(match v14 {
                                        Adt31::C0(_, h2450) => {
                                            h2450
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
                    if f46(v6, f8(Str::lit("group"))) {
                        let m2451 = f464(v4)?;
                        match m2451 {
                            Ok(v16) => {
                                let m2452 = f465(v16, List::cons(f8(Str::lit("sequence")), List::cons(f8(Str::lit("choice")), List::cons(f8(Str::lit("all")), List::<Bytes>::nil()))));
                                match m2452 {
                                    None => {
                                        Ok(Err::<List<Adt26>, Bytes>(f466(f8(Str::lit("a model group definition must have a sequence, choice or all child (XSD 1.1 Part 1, section 3.7.2)")), v3)))
                                    }
                                    Some(v17) => {
                                        f467(v0, v1, v3, v5, v17)
                                    }
                                }
                            }
                            Err(v15) => {
                                Ok(Err::<List<Adt26>, Bytes>(v15))
                            }
                        }
                    } else {
                        let m2453 = f464(v4)?;
                        match m2453 {
                            Ok(v19) => {
                                let m2454 = f468(v0, v1, v3, v5, v19, f469())?;
                                match m2454 {
                                    Ok(v21) => {
                                        Ok(Ok::<List<Adt26>, Bytes>(match v21 {
                                            Adt32::C0(_, h2455, _, _, _) => {
                                                h2455
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

pub fn f375(v0: List<Adt26>, v1: List<Adt26>) -> List<Adt26> {
    let m2456 = v0;
    match m2456.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f375(v3, v1))
        }
    }
}

pub fn f376(p2459: List<Adt26>, p2460: List<Adt26>) -> List<Adt26> {
    let m2458 = (p2459, p2460);
    {
        let mut m2458 = m2458;
        loop {
            let step = {
                let (v0, v1) = m2458;
                let m2457 = v0;
                match m2457.uncons() {
                    None => {
                        Err::<(List<Adt26>, List<Adt26>), List<Adt26>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt26>, List<Adt26>), List<Adt26>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2458 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f377(p2464: List<Adt26>, v1: Bytes) -> Option<Adt26> {
    let m2463 = p2464;
    {
        let mut m2463 = m2463;
        loop {
            let step = {
                let v0 = m2463;
                let m2461 = v0;
                match m2461.uncons() {
                    None => {
                        Err::<List<Adt26>, Option<Adt26>>(None::<Adt26>)
                    }
                    Some((v2, v3)) => {
                        if f46(match v2.clone() {
                            Adt26::C0(h2462, _, _, _) => {
                                h2462
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
                Ok(next) => { m2463 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f378() -> Bytes {
    f8(Str::lit("XmlNode"))
}

pub fn f379(v0: List<Adt26>) -> List<Bytes> {
    let m2465 = v0;
    match m2465.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f385(match v1 {
                Adt26::C0(_, h2466, _, _) => {
                    h2466
                }
            }, f379(v2))
        }
    }
}

pub fn f380() -> List<Bytes> {
    List::cons(f8(Str::lit("    {name: element, fields: [Bytes, Bytes, (List ((Bytes * Bytes) * Bytes)), (List XmlNode)]}")), List::cons(f8(Str::lit("    {name: text, fields: [Bytes]}")), List::<Bytes>::nil()))
}

pub fn f381(p2470: u64, v1: List<Adt26>, p2471: List<Bytes>, p2472: List<Bytes>) -> List<Bytes> {
    let m2469 = (p2470, (p2471, p2472));
    {
        let mut m2469 = m2469;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2469;
                let m2467 = v0;
                if m2467 == 0 {
                    Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v3)
                } else {
                    let v4 = m2467 - 1;
                    let m2468 = v2;
                    match m2468.uncons() {
                        None => {
                            Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v3)
                        }
                        Some((v5, v6)) => {
                            if f153(v5.clone(), v3.clone()) {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v3)))
                            } else {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (f385(f382(v1.clone(), v5.clone()), v6), List::cons(v5, v3))))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2469 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f382(v0: List<Adt26>, v1: Bytes) -> List<Bytes> {
    let m2473 = f377(v0, v1);
    match m2473 {
        None => {
            List::<Bytes>::nil()
        }
        Some(v2) => {
            match v2 {
                Adt26::C0(_, h2474, _, _) => {
                    h2474
                }
            }
        }
    }
}

pub fn f383(v0: List<(List<Bytes>, List<Bytes>)>, v1: List<Bytes>) -> Option<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)> {
    let m2475 = v0;
    match m2475.uncons() {
        None => {
            None::<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)>
        }
        Some((v2, v3)) => {
            let m2477 = f192({
                let (_, h2476) = v2.clone();
                h2476
            }, v1.clone());
            match m2477 {
                None => {
                    Some(({
                        let (h2478, _) = v2;
                        h2478
                    }, v3))
                }
                Some(_) => {
                    let m2479 = f383(v3, v1);
                    match m2479 {
                        None => {
                            None::<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h2480, _) = v5;
                                h2480
                            }, List::cons(v2, {
                                let (_, h2481) = v5;
                                h2481
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f384(v0: List<Adt26>, v1: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m2482 = v1.clone();
    match m2482.uncons() {
        None => {
            Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m2483 = v3;
            match m2483.uncons() {
                None => {
                    let m2486 = f377(v0, v2.clone());
                    match m2486 {
                        None => {
                            Err::<List<Bytes>, Bytes>(f471(v2))
                        }
                        Some(v6) => {
                            if f153(v2.clone(), match v6.clone() {
                                Adt26::C0(_, h2487, _, _) => {
                                    h2487
                                }
                            }) {
                                let m2489 = match v6.clone() {
                                    Adt26::C0(_, _, _, h2488) => {
                                        h2488
                                    }
                                };
                                match m2489.uncons() {
                                    None => {
                                        Err::<List<Bytes>, Bytes>(f471(v2))
                                    }
                                    Some((_, _)) => {
                                        Ok::<List<Bytes>, Bytes>(List::cons(f240(v2, match v6 {
                                            Adt26::C0(_, _, _, h2490) => {
                                                h2490
                                            }
                                        }), List::<Bytes>::nil()))
                                    }
                                }
                            } else {
                                Ok::<List<Bytes>, Bytes>(List::cons(match v6 {
                                    Adt26::C0(_, _, h2491, _) => {
                                        h2491
                                    }
                                }, List::<Bytes>::nil()))
                            }
                        }
                    }
                }
                Some((_, _)) => {
                    f470(v0, v1, {
                        let a2484 = v2;
                        let a2485 = f8(Str::lit("Group"));
                        append_bytes(a2484, a2485)
                    })
                }
            }
        }
    }
}

pub fn f385(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m2492 = v0;
    match m2492.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f385(v3, v1))
        }
    }
}

pub fn f386(v0: List<Adt26>, v1: List<Bytes>, v2: List<Bytes>) -> List<Bytes> {
    let m2493 = v1;
    match m2493.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            f385(f472(f382(v0.clone(), v3), v2.clone()), f386(v0, v4, v2))
        }
    }
}

pub fn f387(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>, v2: Bytes) -> List<Bytes> {
    let m2494 = v1;
    match m2494.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if f46(v3.clone(), v2.clone()) {
                true
            } else {
                if f153(v3.clone(), f473(v0.clone(), v2.clone())) {
                    f153(v2.clone(), f473(v0.clone(), v3.clone()))
                } else {
                    false
                }
            } {
                List::cons(v3, f387(v0, v4, v2))
            } else {
                f387(v0, v4, v2)
            }
        }
    }
}

pub fn f388(v0: Bytes, p2503: u64, p2504: u64, p2505: Option<u64>) -> R<Option<u64>> {
    let m2502 = (p2503, (p2504, p2505));
    {
        let mut m2502 = m2502;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2502;
                let m2495 = v2;
                if m2495 == 0 {
                    Err::<(u64, (u64, Option<u64>)), R<Option<u64>>>(Ok(v3))
                } else {
                    let v4 = m2495 - 1;
                    let v5: u64 = f115(v0.clone(), v1)?;
                    let a2496 = v5;
                    let a2497 = 256u64;
                    if nat_eq(a2496, a2497) {
                        Err::<(u64, (u64, Option<u64>)), R<Option<u64>>>(Ok(v3))
                    } else {
                        Ok::<(u64, (u64, Option<u64>)), R<Option<u64>>>(({
                            let a2498 = v1;
                            let a2499 = 1u64;
                            nat_add(a2498, a2499)?
                        }, (v4, {
                            let a2500 = v5;
                            let a2501 = 47u64;
                            if nat_eq(a2500, a2501) {
                                Some(v1)
                            } else {
                                v3
                            }
                        })))
                    }
                }
            };
            match step {
                Ok(next) => { m2502 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f389(v0: u64) -> bool {
    let a2506 = v0;
    let a2507 = 9u64;
    if if if nat_eq(a2506, a2507) {
        true
    } else {
        let a2508 = v0;
        let a2509 = 10u64;
        nat_eq(a2508, a2509)
    } {
        true
    } else {
        let a2510 = v0;
        let a2511 = 13u64;
        if nat_eq(a2510, a2511) {
            true
        } else {
            let a2512 = 32u64;
            let a2513 = v0;
            if nat_le(a2512, a2513) {
                let a2514 = v0;
                let a2515 = 126u64;
                nat_le(a2514, a2515)
            } else {
                false
            }
        }
    } {
        true
    } else {
        let a2516 = v0;
        let a2517 = 133u64;
        if if nat_eq(a2516, a2517) {
            true
        } else {
            let a2518 = 160u64;
            let a2519 = v0;
            if nat_le(a2518, a2519) {
                let a2520 = v0;
                let a2521 = 55295u64;
                nat_le(a2520, a2521)
            } else {
                false
            }
        } {
            true
        } else {
            let a2522 = 57344u64;
            let a2523 = v0;
            if if nat_le(a2522, a2523) {
                let a2524 = v0;
                let a2525 = 65533u64;
                nat_le(a2524, a2525)
            } else {
                false
            } {
                true
            } else {
                let a2526 = 65536u64;
                let a2527 = v0;
                if nat_le(a2526, a2527) {
                    let a2528 = v0;
                    let a2529 = 1114111u64;
                    nat_le(a2528, a2529)
                } else {
                    false
                }
            }
        }
    }
}

pub fn f390(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f293(v0.clone(), v1)?;
    if f291(f212(v0.clone(), v2)?) {
        Ok(true)
    } else {
        let a2530 = f212(v0, v2)?;
        let a2531 = 35u64;
        Ok(nat_eq(a2530, a2531))
    }
}

pub fn f391(v0: Bytes, v1: u64) -> R<u64> {
    let (h2532, _) = f168(Fn4::F474(v0), {
        let a2533 = v1;
        let a2534 = 1u64;
        nat_add(a2533, a2534)?
    }, v1)?;
    Ok(h2532)
}

pub fn f392(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f475(v0, v1)
}

pub fn f393(v0: Bytes, v1: u64) -> R<u64> {
    let (h2535, _) = f168(Fn4::F476(v0.clone()), {
        let a2537 = {
            let a2536 = v0;
            length_bytes(a2536)
        };
        let a2538 = 1u64;
        nat_add(a2537, a2538)?
    }, v1)?;
    Ok(h2535)
}

pub fn f394(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f477(v0, v1)
}

pub fn f395(v0: u64, v1: bool) -> bool {
    f214(if if f290(v0) {
        true
    } else {
        f291(v0)
    } {
        true
    } else {
        if v1 {
            f478(v0)
        } else {
            false
        }
    })
}

pub fn f396(v0: Bytes, v1: u64, v2: u64) -> R<Adt20> {
    let v3: bool = {
        let a2539 = f212(v0.clone(), v1)?;
        let a2540 = 62u64;
        nat_eq(a2539, a2540)
    };
    let v4: u64 = f212(v0.clone(), {
        let a2541 = v1;
        let a2542 = 1u64;
        nat_add(a2541, a2542)?
    })?;
    let v5: u64 = f212(v0.clone(), {
        let a2543 = v1;
        let a2544 = 2u64;
        nat_add(a2543, a2544)?
    })?;
    let v6: u64 = f479(v4);
    let v7: u64 = f480(v4);
    let v8: bool = {
        let a2545 = 0u64;
        let a2546 = v6;
        if nat_lt(a2545, a2546) {
            true
        } else {
            let a2547 = 0u64;
            let a2548 = v7;
            nat_lt(a2547, a2548)
        }
    };
    let v9: u64 = {
        let a2549 = 0u64;
        let a2550 = v7;
        if nat_lt(a2549, a2550) {
            f479(v5)
        } else {
            0u64
        }
    };
    let v10: u64 = {
        let a2551 = 0u64;
        let a2552 = v6;
        if nat_lt(a2551, a2552) {
            f480(v5)
        } else {
            0u64
        }
    };
    let v11: u64 = {
        let a2553 = v6;
        let a2554 = v9;
        nat_add(a2553, a2554)?
    };
    let v12: u64 = {
        let a2555 = v7;
        let a2556 = v10;
        nat_add(a2555, a2556)?
    };
    let v13: u64 = {
        let a2565 = {
            let a2557 = v1;
            let a2558 = 1u64;
            nat_add(a2557, a2558)?
        };
        let a2566 = {
            let a2563 = if v8 {
                1u64
            } else {
                0u64
            };
            let a2564 = {
                let a2559 = 0u64;
                let a2560 = v9;
                if if nat_lt(a2559, a2560) {
                    true
                } else {
                    let a2561 = 0u64;
                    let a2562 = v10;
                    nat_lt(a2561, a2562)
                } {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a2563, a2564)?
        };
        nat_add(a2565, a2566)?
    };
    let m2567 = f211(v0.clone(), v13)?;
    match m2567 {
        Ok(v15) => {
            let a2568 = 0u64;
            let a2569 = v12;
            if nat_lt(a2568, a2569) {
                f481(v0, v15, {
                    let a2572 = {
                        let a2570 = v2;
                        let a2571 = v12;
                        nat_add(a2570, a2571)?
                    };
                    let a2573 = 1u64;
                    nat_sub(a2572, a2573)
                }, v3, v11)
            } else {
                let v16: (u64, u64) = {
                    let (h2574, _) = f220(Fn5::F482(v0.clone()), {
                        let a2576 = {
                            let a2575 = v0.clone();
                            length_bytes(a2575)
                        };
                        let a2577 = 1u64;
                        nat_add(a2576, a2577)?
                    }, (v15, 0u64))?;
                    h2574
                };
                let v17: u64 = f295(v0.clone(), {
                    let (h2578, _) = v16;
                    h2578
                })?;
                let a2579 = f212(v0.clone(), v17)?;
                let a2580 = 256u64;
                if nat_eq(a2579, a2580) {
                    Ok(Adt20::C0(Adt2::C5({
                        let a2581 = v11;
                        let a2582 = 2u64;
                        if nat_eq(a2581, a2582) {
                            f483(f484(v0.clone(), v15)?)
                        } else {
                            Bytes::lit(&[])
                        }
                    }), {
                        let a2583 = v0;
                        length_bytes(a2583)
                    }))
                } else {
                    let v18: u64 = f485({
                        let a2585 = v17;
                        let a2586 = {
                            let (h2584, _) = v16;
                            h2584
                        };
                        nat_sub(a2585, a2586)
                    }, v2);
                    let a2588 = v18;
                    let a2589 = {
                        let (_, h2587) = v16;
                        h2587
                    };
                    if nat_lt(a2588, a2589) {
                        Ok(Adt20::C1(f111(Str::lit("a leading empty line of a block scalar holds more spaces than its first content line (YAML 1.2.2 section 8.1.1.1)"), v15)?))
                    } else {
                        f481(v0, v15, v18, v3, v11)
                    }
                }
            }
        }
        Err(v14) => {
            Ok(Adt20::C1(v14))
        }
    }
}

pub fn f397(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Adt30> {
    let a2590 = f212(v0.clone(), v1)?;
    let a2591 = 34u64;
    if nat_eq(a2590, a2591) {
        f486(v0.clone(), {
            let a2592 = v1;
            let a2593 = 1u64;
            nat_add(a2592, a2593)?
        }, v2, v3, {
            let a2595 = {
                let a2594 = v0;
                length_bytes(a2594)
            };
            let a2596 = 2u64;
            nat_add(a2595, a2596)?
        }, Bytes::lit(&[]))
    } else {
        f487(v0.clone(), {
            let a2597 = v1;
            let a2598 = 1u64;
            nat_add(a2597, a2598)?
        }, v2, v3, {
            let a2600 = {
                let a2599 = v0;
                length_bytes(a2599)
            };
            let a2601 = 2u64;
            nat_add(a2600, a2601)?
        }, Bytes::lit(&[]))
    }
}

pub fn f398(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: List<Bytes>) -> R<Adt20> {
    let m2602 = v3;
    if m2602 == 0 {
        Ok(Adt20::C1(f111(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v6 = m2602 - 1;
        let a2603 = v4;
        let a2604 = 1u64;
        if nat_eq(a2603, a2604) {
            let m2605 = f398(v0.clone(), v1, v2, v6, 0u64, List::<Bytes>::nil())?;
            match m2605 {
                Adt20::C0(v8, v9) => {
                    let m2606 = f488(v0.clone(), v9, v2)?;
                    match m2606 {
                        Ok(v11) => {
                            let v12: u64 = f212(v0.clone(), v11)?;
                            let a2607 = v12;
                            let a2608 = 58u64;
                            if nat_eq(a2607, a2608) {
                                Ok(Adt20::C1(f111(Str::lit("a single-pair mapping inside a flow sequence is not read (YAML 1.2.2 [150])"), v11)?))
                            } else {
                                let a2609 = v12;
                                let a2610 = 93u64;
                                if nat_eq(a2609, a2610) {
                                    Ok(Adt20::C0(Adt2::C9(std::rc::Rc::new(v8), std::rc::Rc::new(Adt2::C8)), {
                                        let a2611 = v11;
                                        let a2612 = 1u64;
                                        nat_add(a2611, a2612)?
                                    }))
                                } else {
                                    let a2613 = v12;
                                    let a2614 = 44u64;
                                    if nat_eq(a2613, a2614) {
                                        let m2617 = f488(v0.clone(), {
                                            let a2615 = v11;
                                            let a2616 = 1u64;
                                            nat_add(a2615, a2616)?
                                        }, v2)?;
                                        match m2617 {
                                            Ok(v14) => {
                                                let a2618 = f212(v0.clone(), v14)?;
                                                let a2619 = 93u64;
                                                if nat_eq(a2618, a2619) {
                                                    Ok(Adt20::C0(Adt2::C9(std::rc::Rc::new(v8), std::rc::Rc::new(Adt2::C8)), {
                                                        let a2620 = v14;
                                                        let a2621 = 1u64;
                                                        nat_add(a2620, a2621)?
                                                    }))
                                                } else {
                                                    let m2622 = f398(v0, v14, v2, v6, 1u64, List::<Bytes>::nil())?;
                                                    match m2622 {
                                                        Adt20::C0(v16, v17) => {
                                                            Ok(Adt20::C0(Adt2::C9(std::rc::Rc::new(v8), std::rc::Rc::new(v16)), v17))
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
                                        Ok(Adt20::C1(f111(Str::lit("a flow sequence entry must be followed by , or ] (YAML 1.2.2 [137])"), v11)?))
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
            let a2623 = v4;
            let a2624 = 2u64;
            if nat_eq(a2623, a2624) {
                let m2625 = f489(v0.clone(), v1, v2)?;
                match m2625 {
                    Adt30::C0(v19, v20) => {
                        let m2626 = f488(v0.clone(), v20, v2)?;
                        match m2626 {
                            Ok(v22) => {
                                let m2640 = {
                                    let a2627 = f212(v0.clone(), v22)?;
                                    let a2628 = 44u64;
                                    if if nat_eq(a2627, a2628) {
                                        true
                                    } else {
                                        let a2629 = f212(v0.clone(), v22)?;
                                        let a2630 = 125u64;
                                        nat_eq(a2629, a2630)
                                    } {
                                        Adt20::C0(Adt2::C0, v22)
                                    } else {
                                        if f214({
                                            let a2631 = f212(v0.clone(), v22)?;
                                            let a2632 = 58u64;
                                            nat_eq(a2631, a2632)
                                        }) {
                                            Adt20::C1(f111(Str::lit("a flow mapping entry must be a key, then : and its value, or a key alone (YAML 1.2.2 [142], [146])"), v22)?)
                                        } else {
                                            let m2635 = f488(v0.clone(), {
                                                let a2633 = v22;
                                                let a2634 = 1u64;
                                                nat_add(a2633, a2634)?
                                            }, v2)?;
                                            match m2635 {
                                                Ok(v24) => {
                                                    let a2636 = f212(v0.clone(), v24)?;
                                                    let a2637 = 44u64;
                                                    if if nat_eq(a2636, a2637) {
                                                        true
                                                    } else {
                                                        let a2638 = f212(v0.clone(), v24)?;
                                                        let a2639 = 125u64;
                                                        nat_eq(a2638, a2639)
                                                    } {
                                                        Adt20::C0(Adt2::C0, v24)
                                                    } else {
                                                        f398(v0.clone(), v24, v2, v6, 0u64, List::<Bytes>::nil())?
                                                    }
                                                }
                                                Err(v23) => {
                                                    Adt20::C1(v23)
                                                }
                                            }
                                        }
                                    }
                                };
                                match m2640 {
                                    Adt20::C0(v26, v27) => {
                                        if f254(v19.clone(), v5.clone()) {
                                            Ok(Adt20::C1(f111(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                        } else {
                                            let m2641 = f488(v0.clone(), v27, v2)?;
                                            match m2641 {
                                                Ok(v29) => {
                                                    let v30: u64 = f212(v0.clone(), v29)?;
                                                    let a2642 = v30;
                                                    let a2643 = 125u64;
                                                    if nat_eq(a2642, a2643) {
                                                        Ok(Adt20::C0(Adt2::C10(v19, std::rc::Rc::new(v26), std::rc::Rc::new(Adt2::C8)), {
                                                            let a2644 = v29;
                                                            let a2645 = 1u64;
                                                            nat_add(a2644, a2645)?
                                                        }))
                                                    } else {
                                                        let a2646 = v30;
                                                        let a2647 = 44u64;
                                                        if nat_eq(a2646, a2647) {
                                                            let m2650 = f488(v0.clone(), {
                                                                let a2648 = v29;
                                                                let a2649 = 1u64;
                                                                nat_add(a2648, a2649)?
                                                            }, v2)?;
                                                            match m2650 {
                                                                Ok(v32) => {
                                                                    let a2651 = f212(v0.clone(), v32)?;
                                                                    let a2652 = 125u64;
                                                                    if nat_eq(a2651, a2652) {
                                                                        Ok(Adt20::C0(Adt2::C10(v19, std::rc::Rc::new(v26), std::rc::Rc::new(Adt2::C8)), {
                                                                            let a2653 = v32;
                                                                            let a2654 = 1u64;
                                                                            nat_add(a2653, a2654)?
                                                                        }))
                                                                    } else {
                                                                        let m2655 = f398(v0, v32, v2, v6, 2u64, List::cons(v19.clone(), v5))?;
                                                                        match m2655 {
                                                                            Adt20::C0(v34, v35) => {
                                                                                Ok(Adt20::C0(Adt2::C10(v19, std::rc::Rc::new(v26), std::rc::Rc::new(v34)), v35))
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
                                                            Ok(Adt20::C1(f111(Str::lit("a flow mapping entry must be followed by , or } (YAML 1.2.2 [140])"), v29)?))
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
                let v36: u64 = f212(v0.clone(), v1)?;
                let a2656 = v36;
                let a2657 = 91u64;
                if if nat_eq(a2656, a2657) {
                    true
                } else {
                    let a2658 = v36;
                    let a2659 = 123u64;
                    nat_eq(a2658, a2659)
                } {
                    let m2662 = f488(v0.clone(), {
                        let a2660 = v1;
                        let a2661 = 1u64;
                        nat_add(a2660, a2661)?
                    }, v2)?;
                    match m2662 {
                        Ok(v38) => {
                            let a2665 = f212(v0.clone(), v38)?;
                            let a2666 = {
                                let a2663 = v36;
                                let a2664 = 2u64;
                                nat_add(a2663, a2664)?
                            };
                            if nat_eq(a2665, a2666) {
                                Ok(Adt20::C0({
                                    let a2667 = v36;
                                    let a2668 = 91u64;
                                    if nat_eq(a2667, a2668) {
                                        Adt2::C6(std::rc::Rc::new(Adt2::C8))
                                    } else {
                                        Adt2::C7(std::rc::Rc::new(Adt2::C8))
                                    }
                                }, {
                                    let a2669 = v38;
                                    let a2670 = 1u64;
                                    nat_add(a2669, a2670)?
                                }))
                            } else {
                                let m2673 = f398(v0, v38, v2, v6, {
                                    let a2671 = v36;
                                    let a2672 = 91u64;
                                    if nat_eq(a2671, a2672) {
                                        1u64
                                    } else {
                                        2u64
                                    }
                                }, List::<Bytes>::nil())?;
                                match m2673 {
                                    Adt20::C0(v40, v41) => {
                                        Ok(Adt20::C0({
                                            let a2674 = v36;
                                            let a2675 = 91u64;
                                            if nat_eq(a2674, a2675) {
                                                Adt2::C6(std::rc::Rc::new(v40))
                                            } else {
                                                Adt2::C7(std::rc::Rc::new(v40))
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
                    let a2676 = v36;
                    let a2677 = 34u64;
                    if if nat_eq(a2676, a2677) {
                        true
                    } else {
                        let a2678 = v36;
                        let a2679 = 39u64;
                        nat_eq(a2678, a2679)
                    } {
                        let m2680 = f397(v0, v1, v2, false)?;
                        match m2680 {
                            Adt30::C0(v43, v44) => {
                                Ok(Adt20::C0(Adt2::C5(v43), v44))
                            }
                            Adt30::C1(v42) => {
                                Ok(Adt20::C1(v42))
                            }
                        }
                    } else {
                        let m2681 = f298(v0.clone(), v1)?;
                        match m2681 {
                            None => {
                                if f214(f399(v0.clone(), v1, true)?) {
                                    Ok(Adt20::C1(f111(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                                } else {
                                    let v46: u64 = f400(v0.clone(), v1, true)?;
                                    let v47: u64 = f293(v0.clone(), v46)?;
                                    let m2684 = {
                                        let a2682 = f212(v0.clone(), v47)?;
                                        let a2683 = 10u64;
                                        if nat_eq(a2682, a2683) {
                                            f488(v0.clone(), v47, v2)?
                                        } else {
                                            Ok::<u64, Bytes>(v47)
                                        }
                                    };
                                    match m2684 {
                                        Ok(v49) => {
                                            let a2685 = f212(v0.clone(), v47)?;
                                            let a2686 = 10u64;
                                            if if if nat_eq(a2685, a2686) {
                                                f214(f478(f212(v0.clone(), v49)?))
                                            } else {
                                                false
                                            } {
                                                f214({
                                                    let a2687 = f212(v0.clone(), v49)?;
                                                    let a2688 = 58u64;
                                                    nat_eq(a2687, a2688)
                                                })
                                            } else {
                                                false
                                            } {
                                                Ok(Adt20::C1(f111(Str::lit("a multi-line plain scalar inside a flow collection is not read (YAML 1.2.2 [135])"), v47)?))
                                            } else {
                                                f402(f302(v0, v1, v46), v1, v46)
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

pub fn f399(v0: Bytes, v1: u64, v2: bool) -> R<bool> {
    let v3: u64 = f212(v0.clone(), v1)?;
    if if f290(v3) {
        true
    } else {
        f291(v3)
    } {
        Ok(false)
    } else {
        let a2689 = v3;
        let a2690 = 45u64;
        if if if nat_eq(a2689, a2690) {
            true
        } else {
            let a2691 = v3;
            let a2692 = 63u64;
            nat_eq(a2691, a2692)
        } {
            true
        } else {
            let a2693 = v3;
            let a2694 = 58u64;
            nat_eq(a2693, a2694)
        } {
            Ok(f395(f212(v0, {
                let a2695 = v1;
                let a2696 = 1u64;
                nat_add(a2695, a2696)?
            })?, v2))
        } else {
            Ok(f214(f490(v3)?))
        }
    }
}

pub fn f400(v0: Bytes, v1: u64, v2: bool) -> R<u64> {
    let (_, h2697) = {
        let (h2698, _) = f220(Fn5::F491(v2, v0.clone()), {
            let a2700 = {
                let a2699 = v0;
                length_bytes(a2699)
            };
            let a2701 = 1u64;
            nat_add(a2700, a2701)?
        }, (v1, v1))?;
        h2698
    };
    Ok(h2697)
}

pub fn f401(v0: Bytes, p2726: u64, v2: u64, p2727: u64, p2728: Bytes) -> R<Adt30> {
    let m2725 = (p2726, (p2727, p2728));
    {
        let mut m2725 = m2725;
        loop {
            let step = {
                let (v1, (v3, v4)) = m2725;
                let m2702 = v3;
                if m2702 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f111(Str::lit("a plain scalar has more lines than the input"), v1)?)))
                } else {
                    let v5 = m2702 - 1;
                    let v6: u64 = f293(v0.clone(), v1)?;
                    let v7: u64 = f212(v0.clone(), v6)?;
                    let a2703 = v7;
                    let a2704 = 58u64;
                    if nat_eq(a2703, a2704) {
                        Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f492(v6)?)))
                    } else {
                        let a2705 = v7;
                        let a2706 = 35u64;
                        if if nat_eq(a2705, a2706) {
                            true
                        } else {
                            let a2707 = v7;
                            let a2708 = 256u64;
                            nat_eq(a2707, a2708)
                        } {
                            Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C0(v4, f294(v0.clone(), v6)?)))
                        } else {
                            let v8: (u64, u64) = f493(v0.clone(), {
                                let a2709 = v6;
                                let a2710 = 1u64;
                                nat_add(a2709, a2710)?
                            })?;
                            let v9: u64 = {
                                let (h2711, _) = v8;
                                h2711
                            };
                            let v10: u64 = f295(v0.clone(), v9)?;
                            let v11: u64 = f293(v0.clone(), v10)?;
                            let v12: u64 = f212(v0.clone(), v11)?;
                            let a2712 = v12;
                            let a2713 = 256u64;
                            if if if if nat_eq(a2712, a2713) {
                                true
                            } else {
                                let a2716 = {
                                    let a2714 = v10;
                                    let a2715 = v9;
                                    nat_sub(a2714, a2715)
                                };
                                let a2717 = v2;
                                nat_lt(a2716, a2717)
                            } {
                                true
                            } else {
                                let a2718 = v12;
                                let a2719 = 35u64;
                                nat_eq(a2718, a2719)
                            } {
                                true
                            } else {
                                f210(v0.clone(), v9)?
                            } {
                                Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C0(v4, {
                                    let a2720 = v6;
                                    let a2721 = 1u64;
                                    nat_add(a2720, a2721)?
                                })))
                            } else {
                                let v13: u64 = f400(v0.clone(), v11, false)?;
                                let a2722 = v13;
                                let a2723 = v11;
                                if nat_eq(a2722, a2723) {
                                    Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f492(v11)?)))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Adt30>>((v13, (v5, f7(List::cons(v4, List::cons(f494({
                                        let (_, h2724) = v8;
                                        h2724
                                    }), List::cons(f302(v0.clone(), v11, v13), List::<Bytes>::nil())))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2725 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f402(v0: Bytes, v1: u64, v2: u64) -> R<Adt20> {
    let a2730 = {
        let a2729 = v0.clone();
        length_bytes(a2729)
    };
    let a2731 = 0u64;
    if if nat_eq(a2730, a2731) {
        true
    } else {
        f254(v0.clone(), List::cons(f8(Str::lit("~")), List::cons(f8(Str::lit("null")), List::cons(f8(Str::lit("Null")), List::cons(f8(Str::lit("NULL")), List::<Bytes>::nil())))))
    } {
        Ok(Adt20::C0(Adt2::C0, v2))
    } else {
        if f254(v0.clone(), List::cons(f8(Str::lit("true")), List::cons(f8(Str::lit("True")), List::cons(f8(Str::lit("TRUE")), List::<Bytes>::nil())))) {
            Ok(Adt20::C0(Adt2::C2(true), v2))
        } else {
            if f254(v0.clone(), List::cons(f8(Str::lit("false")), List::cons(f8(Str::lit("False")), List::cons(f8(Str::lit("FALSE")), List::<Bytes>::nil())))) {
                Ok(Adt20::C0(Adt2::C2(false), v2))
            } else {
                if f254(v0.clone(), List::cons(f8(Str::lit(".inf")), List::cons(f8(Str::lit(".Inf")), List::cons(f8(Str::lit(".INF")), List::cons(f8(Str::lit("+.inf")), List::cons(f8(Str::lit("+.Inf")), List::cons(f8(Str::lit("+.INF")), List::cons(f8(Str::lit("-.inf")), List::cons(f8(Str::lit("-.Inf")), List::cons(f8(Str::lit("-.INF")), List::cons(f8(Str::lit(".nan")), List::cons(f8(Str::lit(".NaN")), List::cons(f8(Str::lit(".NAN")), List::<Bytes>::nil()))))))))))))) {
                    Ok(Adt20::C1(f111(Str::lit("an infinite or not-a-number float is not read: Studio.Value has none (YAML 1.2.2 section 10.3.2)"), v1)?))
                } else {
                    if if f114(v0.clone(), 0u64, Str::lit("0o")) {
                        let a2733 = 2u64;
                        let a2734 = {
                            let a2732 = v0.clone();
                            length_bytes(a2732)
                        };
                        nat_lt(a2733, a2734)
                    } else {
                        false
                    } {
                        f495(v0, 8u64, v1, v2)
                    } else {
                        if if f114(v0.clone(), 0u64, Str::lit("0x")) {
                            let a2736 = 2u64;
                            let a2737 = {
                                let a2735 = v0.clone();
                                length_bytes(a2735)
                            };
                            nat_lt(a2736, a2737)
                        } else {
                            false
                        } {
                            f495(v0, 16u64, v1, v2)
                        } else {
                            let m2738 = f496(v0.clone())?;
                            match m2738 {
                                None => {
                                    Ok(Adt20::C0(Adt2::C5(v0), v2))
                                }
                                Some(v3) => {
                                    let m2739 = f86(v3.clone(), 0u64)?;
                                    match m2739 {
                                        None => {
                                            Ok(Adt20::C1(f497(v1)?))
                                        }
                                        Some(v4) => {
                                            let a2742 = {
                                                let (h2740, _) = v4;
                                                h2740
                                            };
                                            let a2743 = {
                                                let a2741 = v3;
                                                length_bytes(a2741)
                                            };
                                            if nat_eq(a2742, a2743) {
                                                Ok(Adt20::C0({
                                                    let (_, h2744) = v4;
                                                    h2744
                                                }, v2))
                                            } else {
                                                Ok(Adt20::C1(f497(v1)?))
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

pub fn f403(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f498(v0, v1, v2)
}

pub fn f404(v0: Bytes, v1: u64) -> R<bool> {
    let a2745 = f212(v0.clone(), v1)?;
    let a2746 = 58u64;
    if nat_eq(a2745, a2746) {
        if f290(f212(v0.clone(), {
            let a2747 = v1;
            let a2748 = 1u64;
            nat_add(a2747, a2748)?
        })?) {
            Ok(true)
        } else {
            Ok(f291(f212(v0, {
                let a2749 = v1;
                let a2750 = 1u64;
                nat_add(a2749, a2750)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f405(v0: Bytes, p2769: u64, p2770: u64, p2771: Bytes) -> R<Bytes> {
    let m2768 = (p2769, (p2770, p2771));
    {
        let mut m2768 = m2768;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2768;
                let m2751 = v2;
                if m2751 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m2751 - 1;
                    let v5: u64 = f136(v0.clone(), v1, {
                        let a2755 = {
                            let a2753 = {
                                let a2752 = v0.clone();
                                length_bytes(a2752)
                            };
                            let a2754 = v1;
                            nat_sub(a2753, a2754)
                        };
                        let a2756 = 1u64;
                        nat_add(a2755, a2756)?
                    })?;
                    let v6: Bytes = {
                        let a2759 = v3;
                        let a2760 = f43(v0.clone(), v1, {
                            let a2757 = v5;
                            let a2758 = v1;
                            nat_sub(a2757, a2758)
                        });
                        append_bytes(a2759, a2760)
                    };
                    let m2763 = {
                        let a2761 = v0.clone();
                        let a2762 = v5;
                        index_bytes(a2761, a2762)
                    };
                    match m2763 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v6))
                        }
                        Some(v7) => {
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                let a2764 = v5;
                                let a2765 = 1u64;
                                nat_add(a2764, a2765)?
                            }, (v4, {
                                let a2766 = v6;
                                let a2767 = f499(v0.clone(), v5, v7)?;
                                append_bytes(a2766, a2767)
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2768 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f406(v0: Fn6, p2775: List<(Bytes, Bytes)>, p2776: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m2774 = (p2775, p2776);
    {
        let mut m2774 = m2774;
        loop {
            let step = {
                let (v1, v2) = m2774;
                let m2772 = v2;
                match m2772.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c2773: Fn6 = v0.clone();
                            c2773.apply(v1, v3)
                        }, v4))
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

pub fn f407(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f500(v0, v1)
}

pub fn f408(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f501(v0)
}

pub fn f409(v0: Fn7, p2780: List<Bytes>, p2781: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m2779 = (p2780, p2781);
    {
        let mut m2779 = m2779;
        loop {
            let step = {
                let (v1, v2) = m2779;
                let m2777 = v2;
                match m2777.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c2778: Fn7 = v0.clone();
                            c2778.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2779 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f410(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f502(v1)?, v0))
}

pub fn f411(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2782 = v0.clone();
        length_bytes(a2782)
    };
    let a2786 = {
        let a2783 = v1;
        let a2784 = 2u64;
        let a2785 = 0u64;
        nat_rem(a2783, a2784, a2785)
    };
    let a2787 = 0u64;
    if nat_eq(a2786, a2787) {
        let (_, h2788) = {
            let (h2789, _) = f503(Fn8::F504(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2789
        };
        Ok(h2788)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f412(v0: u64) -> bool {
    let a2790 = v0;
    let a2791 = 8u64;
    if nat_eq(a2790, a2791) {
        true
    } else {
        let a2792 = v0;
        let a2793 = 9u64;
        if nat_eq(a2792, a2793) {
            true
        } else {
            let a2794 = v0;
            let a2795 = 10u64;
            if nat_eq(a2794, a2795) {
                true
            } else {
                let a2796 = v0;
                let a2797 = 12u64;
                if nat_eq(a2796, a2797) {
                    true
                } else {
                    let a2798 = v0;
                    let a2799 = 13u64;
                    nat_eq(a2798, a2799)
                }
            }
        }
    }
}

pub fn f413(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2802 = {
        let a2800 = v0;
        let a2801 = v1;
        index_bytes(a2800, a2801)
    };
    match m2802 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f505(v2)? {
                Ok(Some({
                    let a2803 = v1;
                    let a2804 = 1u64;
                    nat_add(a2803, a2804)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f414(v0: u8) -> R<u64> {
    let a2851 = {
        let v1: u8 = v0;
        let a2849 = {
            let a2825 = {
                let a2813 = {
                    let a2807 = {
                        let a2805 = v1;
                        let a2806 = 128u8;
                        fixed_u8::bit_and(a2805, a2806)
                    };
                    let a2808 = 128u8;
                    if equal(a2807, a2808) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2814 = {
                    let a2811 = {
                        let a2809 = v1;
                        let a2810 = 64u8;
                        fixed_u8::bit_and(a2809, a2810)
                    };
                    let a2812 = 64u8;
                    if equal(a2811, a2812) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2813, a2814)?
            };
            let a2826 = {
                let a2823 = {
                    let a2817 = {
                        let a2815 = v1;
                        let a2816 = 32u8;
                        fixed_u8::bit_and(a2815, a2816)
                    };
                    let a2818 = 32u8;
                    if equal(a2817, a2818) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2824 = {
                    let a2821 = {
                        let a2819 = v1;
                        let a2820 = 16u8;
                        fixed_u8::bit_and(a2819, a2820)
                    };
                    let a2822 = 16u8;
                    if equal(a2821, a2822) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2823, a2824)?
            };
            nat_add(a2825, a2826)?
        };
        let a2850 = {
            let a2847 = {
                let a2835 = {
                    let a2829 = {
                        let a2827 = v1;
                        let a2828 = 8u8;
                        fixed_u8::bit_and(a2827, a2828)
                    };
                    let a2830 = 8u8;
                    if equal(a2829, a2830) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2836 = {
                    let a2833 = {
                        let a2831 = v1;
                        let a2832 = 4u8;
                        fixed_u8::bit_and(a2831, a2832)
                    };
                    let a2834 = 4u8;
                    if equal(a2833, a2834) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2835, a2836)?
            };
            let a2848 = {
                let a2845 = {
                    let a2839 = {
                        let a2837 = v1;
                        let a2838 = 2u8;
                        fixed_u8::bit_and(a2837, a2838)
                    };
                    let a2840 = 2u8;
                    if equal(a2839, a2840) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2846 = {
                    let a2843 = {
                        let a2841 = v1;
                        let a2842 = 1u8;
                        fixed_u8::bit_and(a2841, a2842)
                    };
                    let a2844 = 1u8;
                    if equal(a2843, a2844) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2845, a2846)?
            };
            nat_add(a2847, a2848)?
        };
        nat_add(a2849, a2850)?
    };
    let a2852 = 48u64;
    Ok(nat_sub(a2851, a2852))
}

pub fn f415(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a2853 = v0.clone();
        length_bytes(a2853)
    };
    let a2854 = v1;
    let a2855 = v2;
    if if nat_lt(a2854, a2855) {
        let a2860 = f83(v0, {
            let a2858 = v2;
            let a2859 = {
                let a2856 = v1;
                let a2857 = 1u64;
                nat_add(a2856, a2857)?
            };
            nat_sub(a2858, a2859)
        })?;
        let a2861 = 48u64;
        nat_eq(a2860, a2861)
    } else {
        false
    } {
        Ok(Some({
            let a2862 = v1;
            let a2863 = 1u64;
            nat_add(a2862, a2863)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f416(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f506(v1, v0, v2)
}

pub fn f417(v0: u64) -> bool {
    if f418(v0) {
        true
    } else {
        let a2864 = 65u64;
        let a2865 = v0;
        if if nat_le(a2864, a2865) {
            let a2866 = v0;
            let a2867 = 70u64;
            nat_le(a2866, a2867)
        } else {
            false
        } {
            true
        } else {
            let a2868 = 97u64;
            let a2869 = v0;
            if nat_le(a2868, a2869) {
                let a2870 = v0;
                let a2871 = 102u64;
                nat_le(a2870, a2871)
            } else {
                false
            }
        }
    }
}

pub fn f418(v0: u64) -> bool {
    let a2872 = 48u64;
    let a2873 = v0;
    if nat_le(a2872, a2873) {
        let a2874 = v0;
        let a2875 = 57u64;
        nat_le(a2874, a2875)
    } else {
        false
    }
}

pub fn f419(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f115(v0.clone(), v1)?;
    let a2876 = v2;
    let a2877 = 69u64;
    if if nat_eq(a2876, a2877) {
        true
    } else {
        let a2878 = v2;
        let a2879 = 101u64;
        nat_eq(a2878, a2879)
    } {
        let v3: u64 = f115(v0.clone(), {
            let a2880 = v1;
            let a2881 = 1u64;
            nat_add(a2880, a2881)?
        })?;
        let v4: u64 = {
            let a2882 = v3;
            let a2883 = 43u64;
            if if nat_eq(a2882, a2883) {
                true
            } else {
                let a2884 = v3;
                let a2885 = 45u64;
                nat_eq(a2884, a2885)
            } {
                let a2886 = v1;
                let a2887 = 2u64;
                nat_add(a2886, a2887)?
            } else {
                let a2888 = v1;
                let a2889 = 1u64;
                nat_add(a2888, a2889)?
            }
        };
        let v5: u64 = f322(v0, 0u64, v4)?;
        let a2890 = v4;
        let a2891 = v5;
        if nat_lt(a2890, a2891) {
            Ok(v5)
        } else {
            Ok(v1)
        }
    } else {
        Ok(v1)
    }
}

pub fn f420(v0: Adt21, v1: Bytes) -> bool {
    let a2892 = f421(v0.clone());
    let a2893 = 3u64;
    if !nat_eq(a2892, a2893) {
        f46(f422(v0), v1)
    } else {
        false
    }
}

pub fn f421(v0: Adt21) -> u64 {
    let m2894 = v0;
    match m2894 {
        Adt21::C0(v1, _) => {
            v1
        }
    }
}

pub fn f422(v0: Adt21) -> Bytes {
    let m2895 = v0;
    match m2895 {
        Adt21::C0(_, v2) => {
            v2
        }
    }
}

pub fn f423(v0: Bytes) -> Bytes {
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

pub fn f424(v0: Bytes) -> bool {
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

pub fn f425(p2904: u64, p2905: List<Adt21>, p2906: List<Adt14>) -> R<Result<(List<Adt14>, List<Adt21>), Bytes>> {
    let m2903 = (p2904, (p2905, p2906));
    {
        let mut m2903 = m2903;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2903;
                let m2896 = v0;
                if m2896 == 0 {
                    Err::<(u64, (List<Adt21>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt21>), Bytes>(f8(Str::lit("a dictionary has more members than the input has tokens")))))
                } else {
                    let v3 = m2896 - 1;
                    if f325(v1.clone(), Str::lit("}")) {
                        let m2897 = f332(f327(v1), Str::lit(";"));
                        match m2897 {
                            Ok(v5) => {
                                Err::<(u64, (List<Adt21>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt21>), Bytes>>>(Ok(Ok::<(List<Adt14>, List<Adt21>), Bytes>((f507(v2, List::<Adt14>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<Adt21>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt21>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m2898 = f226(v1)?;
                        match m2898 {
                            Ok(v7) => {
                                let v8: bool = f325(v7.clone(), Str::lit("required"));
                                let m2899 = f337(v3, if v8 {
                                    f327(v7)
                                } else {
                                    v7
                                }, 0u64)?;
                                match m2899 {
                                    Ok(v10) => {
                                        let m2900 = f331(f338(v10.clone()));
                                        match m2900 {
                                            Ok(v12) => {
                                                if if f325(f333(v12.clone()), Str::lit("=")) {
                                                    !v8
                                                } else {
                                                    false
                                                } {
                                                    let m2901 = f427(f327(f333(v12.clone())))?;
                                                    match m2901 {
                                                        Ok(v14) => {
                                                            Ok::<(u64, (List<Adt21>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt21>), Bytes>>>((v3, (v14, List::cons(Adt14::C0(f335(v12), f434(f339(v10)), 0u64), v2))))
                                                        }
                                                        Err(v13) => {
                                                            Err::<(u64, (List<Adt21>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt21>), Bytes>(v13)))
                                                        }
                                                    }
                                                } else {
                                                    let m2902 = f332(f333(v12.clone()), Str::lit(";"));
                                                    match m2902 {
                                                        Ok(v16) => {
                                                            Ok::<(u64, (List<Adt21>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt21>), Bytes>>>((v3, (v16, List::cons(Adt14::C0(f335(v12), f434(f339(v10)), if v8 {
                                                                0u64
                                                            } else {
                                                                1u64
                                                            }), v2))))
                                                        }
                                                        Err(v15) => {
                                                            Err::<(u64, (List<Adt21>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt21>), Bytes>(v15)))
                                                        }
                                                    }
                                                }
                                            }
                                            Err(v11) => {
                                                Err::<(u64, (List<Adt21>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt21>), Bytes>(v11)))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Err::<(u64, (List<Adt21>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt21>), Bytes>(v9)))
                                    }
                                }
                            }
                            Err(v6) => {
                                Err::<(u64, (List<Adt21>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt21>), Bytes>(v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2903 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f426(p2915: u64, p2916: List<Adt21>, p2917: List<Adt14>) -> R<Result<(List<Adt14>, List<Adt21>), Bytes>> {
    let m2914 = (p2915, (p2916, p2917));
    {
        let mut m2914 = m2914;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2914;
                let m2907 = v0;
                if m2907 == 0 {
                    Err::<(u64, (List<Adt21>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt21>), Bytes>(f8(Str::lit("an interface has more members than the input has tokens")))))
                } else {
                    let v3 = m2907 - 1;
                    if f325(v1.clone(), Str::lit("}")) {
                        let m2908 = f332(f327(v1), Str::lit(";"));
                        match m2908 {
                            Ok(v5) => {
                                Err::<(u64, (List<Adt21>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt21>), Bytes>>>(Ok(Ok::<(List<Adt14>, List<Adt21>), Bytes>((f507(v2, List::<Adt14>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<Adt21>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt21>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m2909 = f226(v1)?;
                        match m2909 {
                            Ok(v7) => {
                                let v8: List<Adt21> = if if f325(v7.clone(), Str::lit("stringifier")) {
                                    true
                                } else {
                                    f325(v7.clone(), Str::lit("inherit"))
                                } {
                                    f327(v7.clone())
                                } else {
                                    v7.clone()
                                };
                                let v9: List<Adt21> = if f325(v8.clone(), Str::lit("readonly")) {
                                    f327(v8)
                                } else {
                                    v8
                                };
                                if f325(v9.clone(), Str::lit("attribute")) {
                                    let m2910 = f337(v3, f327(v9), 0u64)?;
                                    match m2910 {
                                        Ok(v11) => {
                                            let m2911 = f331(f338(v11.clone()));
                                            match m2911 {
                                                Ok(v13) => {
                                                    let m2912 = f332(f333(v13.clone()), Str::lit(";"));
                                                    match m2912 {
                                                        Ok(v15) => {
                                                            Ok::<(u64, (List<Adt21>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt21>), Bytes>>>((v3, (v15, List::cons(Adt14::C0(f335(v13), f434(f339(v11)), 0u64), v2))))
                                                        }
                                                        Err(v14) => {
                                                            Err::<(u64, (List<Adt21>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt21>), Bytes>(v14)))
                                                        }
                                                    }
                                                }
                                                Err(v12) => {
                                                    Err::<(u64, (List<Adt21>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt21>), Bytes>(v12)))
                                                }
                                            }
                                        }
                                        Err(v10) => {
                                            Err::<(u64, (List<Adt21>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt21>), Bytes>(v10)))
                                        }
                                    }
                                } else {
                                    let m2913 = f427(v7)?;
                                    match m2913 {
                                        Ok(v17) => {
                                            Ok::<(u64, (List<Adt21>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt21>), Bytes>>>((v3, (v17, v2)))
                                        }
                                        Err(v16) => {
                                            Err::<(u64, (List<Adt21>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt21>), Bytes>(v16)))
                                        }
                                    }
                                }
                            }
                            Err(v6) => {
                                Err::<(u64, (List<Adt21>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt21>), Bytes>(v6)))
                            }
                        }
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

pub fn f427(v0: List<Adt21>) -> R<Result<List<Adt21>, Bytes>> {
    Ok(f326({
        let a2919 = {
            let a2918 = v0.clone();
            length_list(a2918)
        };
        let a2920 = 1u64;
        nat_add(a2919, a2920)?
    }, v0, List::<Bytes>::nil(), true))
}

pub fn f428(v0: List<Adt21>) -> Bytes {
    let m2921 = v0;
    match m2921.uncons() {
        None => {
            f8(Str::lit("the end of the input"))
        }
        Some((v1, _)) => {
            f7(List::cons(f8(Str::lit("'")), List::cons(f422(v1), List::cons(f8(Str::lit("'")), List::<Bytes>::nil()))))
        }
    }
}

pub fn f429(v0: Bytes) -> Bytes {
    if f184(v0.clone(), f8(Str::lit("_"))) {
        f43(v0.clone(), 1u64, {
            let a2923 = {
                let a2922 = v0;
                length_bytes(a2922)
            };
            let a2924 = 1u64;
            nat_sub(a2923, a2924)
        })
    } else {
        v0
    }
}

pub fn f430(v0: Adt15, v1: List<Adt21>) -> Result<Adt29, Bytes> {
    if f325(v1.clone(), Str::lit("?")) {
        Ok::<Adt29, Bytes>(Adt29::C0(f431(Str::lit("Option"), v0), f327(v1)))
    } else {
        Ok::<Adt29, Bytes>(Adt29::C0(v0, v1))
    }
}

pub fn f431(v0: Str, v1: Adt15) -> Adt15 {
    let v2: Adt15 = f434(v1);
    Adt15::C0(0u64, f7(List::cons(f8(Str::lit("(")), List::cons({
        let a2925 = v0;
        utf8_encode(a2925)
    }, List::cons(f8(Str::lit(" ")), List::cons(f244(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), f245(v2.clone()), f242(v2), Bytes::lit(&[]))
}

pub fn f432(v0: Bytes) -> Adt15 {
    Adt15::C0(0u64, Bytes::lit(&[]), List::<Bytes>::nil(), v0, Bytes::lit(&[]))
}

pub fn f433(v0: Bytes) -> Option<Adt15> {
    f508(v0, f509())
}

pub fn f434(v0: Adt15) -> Adt15 {
    let a2926 = f243(v0.clone());
    let a2927 = 0u64;
    if nat_eq(a2926, a2927) {
        v0
    } else {
        Adt15::C0(0u64, Bytes::lit(&[]), f245(v0.clone()), if f46(f242(v0.clone()), Bytes::lit(&[])) {
            f510()
        } else {
            f242(v0)
        }, Bytes::lit(&[]))
    }
}

pub fn f435(v0: Adt15) -> Bytes {
    let m2928 = v0;
    match m2928 {
        Adt15::C0(_, _, _, _, v5) => {
            v5
        }
    }
}

pub fn f436(p2945: u64, p2946: u64, p2947: u64, p2948: Bytes) -> Bytes {
    let m2944 = (p2945, (p2946, (p2947, p2948)));
    {
        let mut m2944 = m2944;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m2944;
                let m2929 = v0;
                if m2929 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m2929 - 1;
                    if f85({
                        let a2930 = v1;
                        let a2931 = 0u64;
                        nat_eq(a2930, a2931)
                    }, {
                        let a2932 = v2;
                        let a2933 = 0u64;
                        nat_eq(a2932, a2933)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a2934 = v1;
                            let a2935 = 16u64;
                            let a2936 = 0u64;
                            nat_quot(a2934, a2935, a2936)
                        }, ({
                            let a2937 = v2;
                            let a2938 = 1u64;
                            nat_sub(a2937, a2938)
                        }, {
                            let a2942 = f511({
                                let a2939 = v1;
                                let a2940 = 16u64;
                                let a2941 = 0u64;
                                nat_rem(a2939, a2940, a2941)
                            });
                            let a2943 = v3;
                            append_bytes(a2942, a2943)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m2944 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f437(v0: Adt14, v1: List<Adt14>) -> R<List<Adt14>> {
    let m2949 = v1;
    match m2949.uncons() {
        None => {
            Ok(List::cons(v0, List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            if f512({
                let a2951 = {
                    let a2950 = f348(v0.clone());
                    length_bytes(a2950)
                };
                let a2952 = 1u64;
                nat_add(a2951, a2952)?
            }, f348(v0.clone()), f348(v2.clone()), 0u64)? {
                Ok(List::cons(v0, List::cons(v2, v3)))
            } else {
                Ok(List::cons(v2, f437(v0, v3)?))
            }
        }
    }
}

pub fn f438(v0: List<Adt13>, v1: Bytes) -> List<Adt14> {
    let m2953 = f161(v0.clone(), 1u64, v1.clone());
    match m2953 {
        None => {
            f345(v0, 6u64, v1)
        }
        Some(v2) => {
            f343(f344(v2), f345(v0, 6u64, v1))
        }
    }
}

pub fn f439(v0: Bytes) -> R<Option<Bytes>> {
    let m2955 = f513({
        let a2954 = v0.clone();
        length_bytes(a2954)
    }, v0.clone(), 0u64, Bytes::lit(&[]))?;
    match m2955 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a2957 = {
                let a2956 = v1.clone();
                length_bytes(a2956)
            };
            let a2958 = 0u64;
            if nat_eq(a2957, a2958) {
                let a2960 = {
                    let a2959 = v0;
                    length_bytes(a2959)
                };
                let a2961 = 0u64;
                if nat_eq(a2960, a2961) {
                    Ok(Some(f8(Str::lit("u"))))
                } else {
                    Ok(None::<Bytes>)
                }
            } else {
                Ok(Some(f441(v1)?))
            }
        }
    }
}

pub fn f440(v0: Bytes) -> R<Option<Bytes>> {
    if f514({
        let a2962 = v0.clone();
        length_bytes(a2962)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f515(v0)?;
        if f516(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m2965 = {
                let a2963 = v1.clone();
                let a2964 = 0u64;
                index_bytes(a2963, a2964)
            };
            match m2965 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f370(f52(v2)?) {
                        Ok(Some({
                            let a2966 = f8(Str::lit("n"));
                            let a2967 = v1;
                            append_bytes(a2966, a2967)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f517({
            let a2968 = v0.clone();
            length_bytes(a2968)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f441(v0: Bytes) -> R<Bytes> {
    if f153(v0.clone(), f518()) {
        f341(v0)
    } else {
        Ok(v0)
    }
}

pub fn f442(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2971 = {
        let a2969 = v0;
        let a2970 = v1;
        index_bytes(a2969, a2970)
    };
    match m2971 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f52(v2)?;
            if f85({
                let a2972 = 128u64;
                let a2973 = v3;
                nat_le(a2972, a2973)
            }, {
                let a2974 = v3;
                let a2975 = 191u64;
                nat_le(a2974, a2975)
            }) {
                Ok(Some({
                    let a2976 = v3;
                    let a2977 = 128u64;
                    nat_sub(a2976, a2977)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f443(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f85({
        let a2978 = v1;
        let a2979 = v0;
        nat_le(a2978, a2979)
    }, f85({
        let a2980 = v0;
        let a2981 = v2;
        nat_le(a2980, a2981)
    }, f139({
        let a2982 = v0;
        let a2983 = 55296u64;
        nat_lt(a2982, a2983)
    }, {
        let a2984 = 57343u64;
        let a2985 = v0;
        nat_lt(a2984, a2985)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f444() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{3}/\u{7f}\u{3}@\u{0}\u{3}\u{7f}}\u{4}\u{0}\u{0}C\u{7f}\u{7f}"))
}

pub fn f445(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a2986 = v1;
        let a2987 = 3u64;
        nat_mul(a2986, a2987)?
    };
    let a2998 = {
        let a2994 = {
            let a2988 = f83(v0.clone(), v2)?;
            let a2989 = 16384u64;
            nat_mul(a2988, a2989)?
        };
        let a2995 = {
            let a2992 = f83(v0.clone(), {
                let a2990 = v2;
                let a2991 = 1u64;
                nat_add(a2990, a2991)?
            })?;
            let a2993 = 128u64;
            nat_mul(a2992, a2993)?
        };
        nat_add(a2994, a2995)?
    };
    let a2999 = f83(v0, {
        let a2996 = v2;
        let a2997 = 2u64;
        nat_add(a2996, a2997)?
    })?;
    nat_add(a2998, a2999)
}

pub fn f446(v0: u64) -> R<bool> {
    f250(v0, f519())
}

pub fn f447(v0: u64) -> R<bool> {
    f250(v0, f520())
}

pub fn f448(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f521(v1, v0, v2)
}

pub fn f449(v0: Bytes, v1: u64) -> Bytes {
    f43(v0, v1, 1u64)
}

pub fn f450() -> Bytes {
    f8(Str::lit("\u{0}\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\u{9}\u{a}\u{b}\u{c}\u{d}\u{e}\u{f}\u{10}\u{11}\u{12}\u{13}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{1b}\u{1c}\u{1d}\u{1e}\u{1f} !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~\u{7f}"))
}

pub fn f451() -> Bytes {
    f8(Str::lit("\u{80}\u{c0}\u{100}\u{140}\u{180}\u{1c0}\u{200}\u{240}\u{280}\u{2c0}\u{300}\u{340}\u{380}\u{3c0}\u{400}\u{440}\u{480}\u{4c0}\u{500}\u{540}\u{580}\u{5c0}\u{600}\u{640}\u{680}\u{6c0}\u{700}\u{740}\u{780}\u{7c0}"))
}

pub fn f452(v0: u64) -> R<Bytes> {
    Ok(f449(f522(), {
        let a3005 = {
            let a3003 = {
                let a3000 = v0;
                let a3001 = 64u64;
                let a3002 = 0u64;
                nat_rem(a3000, a3001, a3002)
            };
            let a3004 = 2u64;
            nat_mul(a3003, a3004)?
        };
        let a3006 = 1u64;
        nat_add(a3005, a3006)?
    }))
}

pub fn f453() -> Bytes {
    f8(Str::lit("\u{800}\u{1000}\u{2000}\u{3000}\u{4000}\u{5000}\u{6000}\u{7000}\u{8000}\u{9000}\u{a000}\u{b000}\u{c000}\u{d000}\u{e000}\u{f000}"))
}

pub fn f454(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<Option<u64>> {
    if f523(f115(v0, v3)?, v1, v2)? {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3007 = v3;
            let a3008 = 1u64;
            nat_add(a3007, a3008)?
        }))
    }
}

pub fn f455(v0: Bytes) -> Result<Bytes, Bytes> {
    Err::<Bytes, Bytes>(v0)
}

pub fn f456() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f457(v0: Bytes) -> R<Option<Bytes>> {
    let m3009 = f171(v0.clone(), f8(Str::lit(" ")), 0u64)?;
    match m3009 {
        None => {
            let m3010 = f171(v0.clone(), f8(Str::lit(":")), 0u64)?;
            match m3010 {
                None => {
                    let v3: Bytes = f7(List::cons(f8(Str::lit(" ")), List::cons(v0, List::cons(f8(Str::lit(":")), List::<Bytes>::nil()))));
                    let m3011 = f171(f524(), v3.clone(), 0u64)?;
                    match m3011 {
                        None => {
                            Ok(None::<Bytes>)
                        }
                        Some(v4) => {
                            let v5: u64 = {
                                let a3013 = v4;
                                let a3014 = {
                                    let a3012 = v3;
                                    length_bytes(a3012)
                                };
                                nat_add(a3013, a3014)?
                            };
                            let m3015 = f171(f524(), f8(Str::lit(" ")), v5)?;
                            match m3015 {
                                None => {
                                    Ok(None::<Bytes>)
                                }
                                Some(v6) => {
                                    Ok(Some(f43(f524(), v5, {
                                        let a3016 = v6;
                                        let a3017 = v5;
                                        nat_sub(a3016, a3017)
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

pub fn f458(v0: Bytes) -> Bytes {
    if f38(v0.clone(), Bytes::lit(&[0x0d])) {
        f43(v0.clone(), 0u64, {
            let a3019 = {
                let a3018 = v0;
                length_bytes(a3018)
            };
            let a3020 = 1u64;
            nat_sub(a3019, a3020)
        })
    } else {
        v0
    }
}

pub fn f459() -> List<Bytes> {
    List::cons(f8(Str::lit("complexType")), List::cons(f8(Str::lit("simpleType")), List::cons(f8(Str::lit("element")), List::cons(f8(Str::lit("attribute")), List::cons(f8(Str::lit("attributeGroup")), List::cons(f8(Str::lit("group")), List::<Bytes>::nil()))))))
}

pub fn f460(v0: u64, v1: Adt23, v2: Adt24) -> R<Result<List<Adt26>, Bytes>> {
    let v3: Bytes = f191(match v2.clone() {
        Adt24::C0(_, _, h3021, _) => {
            h3021
        }
    })?;
    let v4: (Bytes, Bytes) = (match v2.clone() {
        Adt24::C0(_, h3022, _, _) => {
            h3022
        }
    }, match v2.clone() {
        Adt24::C0(_, _, h3023, _) => {
            h3023
        }
    });
    let v5: bool = f525(match v2.clone() {
        Adt24::C0(_, _, _, h3024) => {
            h3024
        }
    }, Str::lit("abstract"), Str::lit("true"));
    let m3026 = f526(match v1.clone() {
        Adt23::C0(h3025, _) => {
            h3025
        }
    }, v4, f282(v5))?;
    match m3026 {
        Ok(v7) => {
            if v5 {
                let m3027 = v7.clone();
                match m3027.uncons() {
                    None => {
                        Ok(Err::<List<Adt26>, Bytes>(f466(f8(Str::lit("an abstract complex type with no type derived from it in this schema set admits no instance (XSD 1.1 Part 1, sections 3.4.1 and 3.3.4.2)")), v3)))
                    }
                    Some((_, _)) => {
                        let m3028 = f527(v3, v7.clone(), f528(v7.clone())?, f528(v7)?)?;
                        match m3028 {
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
                let m3029 = v7.clone();
                match m3029.uncons() {
                    None => {
                        f529(v0, v1, v3, match v2.clone() {
                            Adt24::C0(_, h3030, _, _) => {
                                h3030
                            }
                        }, match v2 {
                            Adt24::C0(_, _, _, h3031) => {
                                h3031
                            }
                        })
                    }
                    Some((_, _)) => {
                        let v14: Bytes = f530(v3.clone());
                        let m3034 = f529(v0, v1, v14.clone(), match v2.clone() {
                            Adt24::C0(_, h3032, _, _) => {
                                h3032
                            }
                        }, match v2.clone() {
                            Adt24::C0(_, _, _, h3033) => {
                                h3033
                            }
                        })?;
                        match m3034 {
                            Ok(v16) => {
                                let m3036 = f527(v3, List::cons(match v2 {
                                    Adt24::C0(_, _, h3035, _) => {
                                        h3035
                                    }
                                }, v7.clone()), List::cons(v14.clone(), f528(v7.clone())?), List::cons(v14, f528(v7)?))?;
                                match m3036 {
                                    Ok(v18) => {
                                        Ok(Ok::<List<Adt26>, Bytes>(f375(v16, List::cons(v18, List::<Adt26>::nil()))))
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

pub fn f461(v0: Bytes, v1: Adt25, v2: Bytes) -> R<Result<Adt26, Bytes>> {
    let m3039 = f190(v0.clone(), match v1.clone() {
        Adt25::C0(_, _, _, h3037, _) => {
            h3037
        }
    }, match v1 {
        Adt25::C0(_, _, _, _, h3038) => {
            h3038
        }
    }, v2)?;
    match m3039 {
        Ok(v4) => {
            Ok(Ok::<Adt26, Bytes>(Adt26::C0(f191(v0)?, match v4.clone() {
                Adt22::C0(_, h3040) => {
                    h3040
                }
            }, match v4 {
                Adt22::C0(h3041, _) => {
                    h3041
                }
            }, List::<Bytes>::nil())))
        }
        Err(v3) => {
            Ok(Err::<Adt26, Bytes>(v3))
        }
    }
}

pub fn f462(v0: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: Adt25) -> R<Result<Adt31, Bytes>> {
    let m3042 = v0;
    if m3042 == 0 {
        Ok(Err::<Adt31, Bytes>(f466(f8(Str::lit("the element declarations nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m3042 - 1;
        let m3043 = f284(v4.clone(), Str::lit("type"));
        match m3043 {
            None => {
                let m3047 = f464(v4.clone())?;
                match m3047 {
                    Ok(v10) => {
                        let m3048 = f465(v10, List::cons(f8(Str::lit("complexType")), List::cons(f8(Str::lit("simpleType")), List::<Bytes>::nil())));
                        match m3048 {
                            None => {
                                Ok(Ok::<Adt31, Bytes>(Adt31::C0(Adt22::C0(f532(v4, f378()), List::cons(f378(), List::<Bytes>::nil())), List::<Adt26>::nil())))
                            }
                            Some(v11) => {
                                if f283(v11.clone(), Str::lit("complexType")) {
                                    let m3049 = f529(v5, v1, v2.clone(), v3, v11)?;
                                    match m3049 {
                                        Ok(v13) => {
                                            Ok(Ok::<Adt31, Bytes>(Adt31::C0(Adt22::C0(f532(v4, v2.clone()), List::cons(v2, List::<Bytes>::nil())), v13)))
                                        }
                                        Err(v12) => {
                                            Ok(Err::<Adt31, Bytes>(v12))
                                        }
                                    }
                                } else {
                                    let m3050 = f461(v2.clone(), v11, v3)?;
                                    match m3050 {
                                        Ok(v15) => {
                                            Ok(Ok::<Adt31, Bytes>(Adt31::C0(Adt22::C0(f532(v4, v2.clone()), List::cons(v2, List::<Bytes>::nil())), List::cons(v15, List::<Adt26>::nil()))))
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
                let m3044 = f531(v1, v4.clone(), v6)?;
                match m3044 {
                    Ok(v8) => {
                        Ok(Ok::<Adt31, Bytes>(Adt31::C0(Adt22::C0(f532(v4, match v8.clone() {
                            Adt22::C0(h3045, _) => {
                                h3045
                            }
                        }), match v8 {
                            Adt22::C0(_, h3046) => {
                                h3046
                            }
                        }), List::<Adt26>::nil())))
                    }
                    Err(v7) => {
                        Ok(Err::<Adt31, Bytes>(f466(v7, v2)))
                    }
                }
            }
        }
    }
}

pub fn f463(v0: Adt23, v1: Bytes, v2: Bytes, v3: Adt25) -> R<Result<Adt31, Bytes>> {
    let m3051 = f284(v3.clone(), Str::lit("type"));
    match m3051 {
        None => {
            let m3053 = f464(v3)?;
            match m3053 {
                Ok(v8) => {
                    let m3054 = f465(v8, List::cons(f8(Str::lit("simpleType")), List::<Bytes>::nil()));
                    match m3054 {
                        None => {
                            Ok(Err::<Adt31, Bytes>(f466(f8(Str::lit("an attribute declaration with no type has the type xs:anySimpleType, which is not translated, because it is the union of every simple value space (XSD 1.1 Part 1, section 3.2.2.1; Part 2, section 3.2.1)")), v2)))
                        }
                        Some(v9) => {
                            let m3055 = f461(v2.clone(), v9, v1)?;
                            match m3055 {
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
            let m3052 = f531(v0, v3, v4)?;
            match m3052 {
                Ok(v6) => {
                    Ok(Ok::<Adt31, Bytes>(Adt31::C0(v6, List::<Adt26>::nil())))
                }
                Err(v5) => {
                    Ok(Err::<Adt31, Bytes>(f466(v5, v2)))
                }
            }
        }
    }
}

pub fn f464(v0: Adt25) -> R<Result<List<Adt25>, Bytes>> {
    f204(match v0.clone() {
        Adt25::C0(_, _, _, h3056, _) => {
            h3056
        }
    }, match v0 {
        Adt25::C0(_, _, _, _, h3057) => {
            h3057
        }
    })
}

pub fn f465(p3061: List<Adt25>, v1: List<Bytes>) -> Option<Adt25> {
    let m3060 = p3061;
    {
        let mut m3060 = m3060;
        loop {
            let step = {
                let v0 = m3060;
                let m3058 = v0;
                match m3058.uncons() {
                    None => {
                        Err::<List<Adt25>, Option<Adt25>>(None::<Adt25>)
                    }
                    Some((v2, v3)) => {
                        if f153(match v2.clone() {
                            Adt25::C0(_, h3059, _, _, _) => {
                                h3059
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
                Ok(next) => { m3060 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f466(v0: Bytes, v1: Bytes) -> Bytes {
    f200(v0, f8(Str::lit(" at ")), v1)
}

pub fn f467(v0: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: Adt25) -> R<Result<List<Adt26>, Bytes>> {
    let m3062 = v0;
    if m3062 == 0 {
        Ok(Err::<List<Adt26>, Bytes>(f466(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m3062 - 1;
        let m3063 = f464(v4.clone())?;
        match m3063 {
            Ok(v7) => {
                if f283(v4, Str::lit("choice")) {
                    let m3064 = f533(v5, v1, v2.clone(), v3, false, v7, f469())?;
                    match m3064 {
                        Ok(v9) => {
                            let m3068 = f527(v2, f534(match v9.clone() {
                                Adt32::C0(h3065, _, _, _, _) => {
                                    h3065
                                }
                            }), f535(match v9.clone() {
                                Adt32::C0(h3066, _, _, _, _) => {
                                    h3066
                                }
                            }), f536(match v9.clone() {
                                Adt32::C0(h3067, _, _, _, _) => {
                                    h3067
                                }
                            }))?;
                            match m3068 {
                                Ok(v11) => {
                                    Ok(Ok::<List<Adt26>, Bytes>(f375(match v9 {
                                        Adt32::C0(_, h3069, _, _, _) => {
                                            h3069
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
                    let m3070 = f533(v5, v1, v2.clone(), v3, true, v7, f469())?;
                    match m3070 {
                        Ok(v13) => {
                            let m3072 = f537(v2, match v13.clone() {
                                Adt32::C0(h3071, _, _, _, _) => {
                                    h3071
                                }
                            })?;
                            match m3072 {
                                Ok(v15) => {
                                    Ok(Ok::<List<Adt26>, Bytes>(f375(match v13 {
                                        Adt32::C0(_, h3073, _, _, _) => {
                                            h3073
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

pub fn f468(v0: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: List<Adt25>, v5: Adt32) -> R<Result<Adt32, Bytes>> {
    let m3074 = v0;
    if m3074 == 0 {
        Ok(Err::<Adt32, Bytes>(f466(f8(Str::lit("the attribute group references nest deeper than the schema set has components, so they refer to each other in a cycle (XSD 1.1 Part 1, section 3.6.3)")), v2)))
    } else {
        let v6 = m3074 - 1;
        let m3075 = v4;
        match m3075.uncons() {
            None => {
                Ok(Ok::<Adt32, Bytes>(v5))
            }
            Some((v7, v8)) => {
                if f283(v7.clone(), Str::lit("attribute")) {
                    if f525(v7.clone(), Str::lit("use"), Str::lit("prohibited")) {
                        f468(v6, v1, v2, v3, v8, v5)
                    } else {
                        let v9: bool = f525(v7.clone(), Str::lit("use"), Str::lit("required"));
                        let m3076 = f284(v7.clone(), Str::lit("ref"));
                        match m3076 {
                            None => {
                                let m3086 = f284(v7.clone(), Str::lit("name"));
                                match m3086 {
                                    None => {
                                        Ok(Err::<Adt32, Bytes>(f466(f8(Str::lit("a local attribute declaration must have a name or a ref (XSD 1.1 Part 1, section 3.2.3)")), v2)))
                                    }
                                    Some(v16) => {
                                        let m3089 = f463(v1.clone(), v3.clone(), {
                                            let a3087 = v2.clone();
                                            let a3088 = f191(v16.clone())?;
                                            append_bytes(a3087, a3088)
                                        }, v7)?;
                                        match m3089 {
                                            Ok(v18) => {
                                                f468(v6, v1, v2, v3, v8, f539(v5, v16, v18, v9))
                                            }
                                            Err(v17) => {
                                                Ok(Err::<Adt32, Bytes>(v17))
                                            }
                                        }
                                    }
                                }
                            }
                            Some(v10) => {
                                let m3078 = f119(match v7 {
                                    Adt25::C0(_, _, _, _, h3077) => {
                                        h3077
                                    }
                                }, v10.clone())?;
                                match m3078 {
                                    Ok(v12) => {
                                        if f46({
                                            let (h3079, _) = v12.clone();
                                            h3079
                                        }, f182()) {
                                            Ok(Err::<Adt32, Bytes>(f466(f200(f8(Str::lit("the attribute ")), v10, f8(Str::lit(" is declared by the schema document for the XML namespace, which this schema set does not import (XSD 1.1 Part 1, sections 3.2.2.3 and 4.2.6)"))), v2)))
                                        } else {
                                            let m3080 = f538(v1.clone(), Str::lit("attribute"), v12.clone());
                                            match m3080 {
                                                None => {
                                                    Ok(Err::<Adt32, Bytes>(f466(f200(f8(Str::lit("the attribute ")), v10, f8(Str::lit(" is not declared by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                                }
                                                Some(v13) => {
                                                    let m3084 = f463(v1.clone(), match v13.clone() {
                                                        Adt24::C0(_, h3081, _, _) => {
                                                            h3081
                                                        }
                                                    }, f191({
                                                        let (_, h3082) = v12.clone();
                                                        h3082
                                                    })?, match v13 {
                                                        Adt24::C0(_, _, _, h3083) => {
                                                            h3083
                                                        }
                                                    })?;
                                                    match m3084 {
                                                        Ok(v15) => {
                                                            f468(v6, v1, v2, v3, v8, f539(v5, {
                                                                let (_, h3085) = v12;
                                                                h3085
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
                                        Ok(Err::<Adt32, Bytes>(f466(v11, v2)))
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if f283(v7.clone(), Str::lit("attributeGroup")) {
                        let m3090 = f284(v7.clone(), Str::lit("ref"));
                        match m3090 {
                            None => {
                                Ok(Err::<Adt32, Bytes>(f466(f8(Str::lit("an attribute group reference must have a ref (XSD 1.1 Part 1, section 3.6.2)")), v2)))
                            }
                            Some(v19) => {
                                let m3092 = f119(match v7 {
                                    Adt25::C0(_, _, _, _, h3091) => {
                                        h3091
                                    }
                                }, v19.clone())?;
                                match m3092 {
                                    Ok(v21) => {
                                        let m3093 = f538(v1.clone(), Str::lit("attributeGroup"), v21.clone());
                                        match m3093 {
                                            None => {
                                                Ok(Err::<Adt32, Bytes>(f466(f200(f8(Str::lit("the attribute group ")), v19, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                            }
                                            Some(v22) => {
                                                let m3095 = f464(match v22.clone() {
                                                    Adt24::C0(_, _, _, h3094) => {
                                                        h3094
                                                    }
                                                })?;
                                                match m3095 {
                                                    Ok(v24) => {
                                                        let m3098 = f468(v6, v1.clone(), f191({
                                                            let (_, h3096) = v21;
                                                            h3096
                                                        })?, match v22 {
                                                            Adt24::C0(_, h3097, _, _) => {
                                                                h3097
                                                            }
                                                        }, v24, v5)?;
                                                        match m3098 {
                                                            Ok(v26) => {
                                                                f468(v6, v1, v2, v3, v8, v26)
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
                                        Ok(Err::<Adt32, Bytes>(f466(v20, v2)))
                                    }
                                }
                            }
                        }
                    } else {
                        if f283(v7, Str::lit("anyAttribute")) {
                            f468(v6, v1, v2, v3, v8, f540(v5, Adt33::C0(f8(Str::lit("anyAttribute")), f541(), List::<Bytes>::nil())))
                        } else {
                            f468(v6, v1, v2, v3, v8, v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f469() -> Adt32 {
    Adt32::C0(List::<Adt33>::nil(), List::<Adt26>::nil(), 0u64, 0u64, 0u64)
}

pub fn f470(v0: List<Adt26>, v1: List<Bytes>, v2: Bytes) -> Result<List<Bytes>, Bytes> {
    let m3099 = v1;
    match m3099.uncons() {
        None => {
            Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let m3100 = f377(v0.clone(), v3.clone());
            match m3100 {
                None => {
                    Err::<List<Bytes>, Bytes>(f471(v3))
                }
                Some(v5) => {
                    let m3102 = match v5.clone() {
                        Adt26::C0(_, _, _, h3101) => {
                            h3101
                        }
                    };
                    match m3102.uncons() {
                        None => {
                            Err::<List<Bytes>, Bytes>(f471(v3))
                        }
                        Some((_, _)) => {
                            let m3103 = f470(v0, v4, v2.clone());
                            match m3103 {
                                Ok(v9) => {
                                    Ok::<List<Bytes>, Bytes>(List::cons(f542(v3, match v5 {
                                        Adt26::C0(_, _, _, h3104) => {
                                            h3104
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

pub fn f471(v0: Bytes) -> Bytes {
    f466(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)")), v0)
}

pub fn f472(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m3105 = v0;
    match m3105.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f153(v2.clone(), v1.clone()) {
                f472(v3, v1)
            } else {
                List::cons(v2, f472(v3, v1))
            }
        }
    }
}

pub fn f473(p3110: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m3109 = p3110;
    {
        let mut m3109 = m3109;
        loop {
            let step = {
                let v0 = m3109;
                let m3106 = v0;
                match m3106.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f46({
                            let (h3107, _) = v2;
                            h3107
                        }, v1.clone()) {
                            let (_, h3108) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h3108)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3109 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f474(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f543(v0, v1)
}

pub fn f475(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f290(f212(v0, v1)?) {
        Ok(Some({
            let a3111 = v1;
            let a3112 = 1u64;
            nat_add(a3111, a3112)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f476(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f544(v0, v1)
}

pub fn f477(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a3113 = f212(v0, v1)?;
    let a3114 = 32u64;
    if nat_eq(a3113, a3114) {
        Ok(Some({
            let a3115 = v1;
            let a3116 = 1u64;
            nat_add(a3115, a3116)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f478(v0: u64) -> bool {
    let a3117 = v0;
    let a3118 = 44u64;
    if if if nat_eq(a3117, a3118) {
        true
    } else {
        let a3119 = v0;
        let a3120 = 91u64;
        nat_eq(a3119, a3120)
    } {
        true
    } else {
        let a3121 = v0;
        let a3122 = 93u64;
        if nat_eq(a3121, a3122) {
            true
        } else {
            let a3123 = v0;
            let a3124 = 123u64;
            nat_eq(a3123, a3124)
        }
    } {
        true
    } else {
        let a3125 = v0;
        let a3126 = 125u64;
        nat_eq(a3125, a3126)
    }
}

pub fn f479(v0: u64) -> u64 {
    let a3127 = v0;
    let a3128 = 45u64;
    if nat_eq(a3127, a3128) {
        1u64
    } else {
        let a3129 = v0;
        let a3130 = 43u64;
        if nat_eq(a3129, a3130) {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f480(v0: u64) -> u64 {
    let a3131 = 49u64;
    let a3132 = v0;
    if if nat_le(a3131, a3132) {
        let a3133 = v0;
        let a3134 = 57u64;
        nat_le(a3133, a3134)
    } else {
        false
    } {
        let a3135 = v0;
        let a3136 = 48u64;
        nat_sub(a3135, a3136)
    } else {
        0u64
    }
}

pub fn f481(v0: Bytes, v1: u64, v2: u64, v3: bool, v4: u64) -> R<Adt20> {
    let m3137 = f545(v0.clone(), v1, v2)?;
    match m3137 {
        Adt34::C0(v6, v7) => {
            let v8: u64 = v7;
            let v9: u64 = f546(v0.clone(), v8, v2)?;
            if f547(v0.clone(), v8, v2)? {
                let m3141 = f548(v0.clone(), v8, v2, v3, {
                    let a3139 = {
                        let a3138 = v0;
                        length_bytes(a3138)
                    };
                    let a3140 = 2u64;
                    nat_add(a3139, a3140)?
                }, Bytes::lit(&[]), v6)?;
                match m3141 {
                    Adt35::C0(v11, v12, v13) => {
                        Ok(Adt20::C0(Adt2::C5(f7(List::cons(v11, List::cons({
                            let a3142 = v4;
                            let a3143 = 1u64;
                            if nat_eq(a3142, a3143) {
                                Bytes::lit(&[])
                            } else {
                                Bytes::lit(&[0x0a])
                            }
                        }, List::cons({
                            let a3144 = v4;
                            let a3145 = 2u64;
                            if nat_eq(a3144, a3145) {
                                f483(v12)
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
                Ok(Adt20::C0(Adt2::C5({
                    let a3146 = v4;
                    let a3147 = 2u64;
                    if nat_eq(a3146, a3147) {
                        f483(v6)
                    } else {
                        Bytes::lit(&[])
                    }
                }), {
                    let a3148 = f212(v0, v9)?;
                    let a3149 = 256u64;
                    if nat_eq(a3148, a3149) {
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

pub fn f482(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f549(v0, v1)
}

pub fn f483(v0: u64) -> Bytes {
    let m3150 = v0;
    if m3150 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m3150 - 1;
        let a3151 = Bytes::lit(&[0x0a]);
        let a3152 = f483(v1);
        append_bytes(a3151, a3152)
    }
}

pub fn f484(v0: Bytes, v1: u64) -> R<u64> {
    let (_, h3153) = {
        let (h3154, _) = f220(Fn5::F550(v0.clone()), {
            let a3156 = {
                let a3155 = v0;
                length_bytes(a3155)
            };
            let a3157 = 1u64;
            nat_add(a3156, a3157)?
        }, (v1, 0u64))?;
        h3154
    };
    Ok(h3153)
}

pub fn f485(v0: u64, v1: u64) -> u64 {
    let a3158 = v0;
    let a3159 = v1;
    if nat_lt(a3158, a3159) {
        v1
    } else {
        v0
    }
}

pub fn f486(v0: Bytes, p3218: u64, v2: u64, v3: bool, p3219: u64, p3220: Bytes) -> R<Adt30> {
    let m3217 = (p3218, (p3219, p3220));
    {
        let mut m3217 = m3217;
        loop {
            let step = {
                let (v1, (v4, v5)) = m3217;
                let m3160 = v4;
                if m3160 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f111(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                } else {
                    let v6 = m3160 - 1;
                    let v7: u64 = f212(v0.clone(), v1)?;
                    let a3161 = v7;
                    let a3162 = 34u64;
                    if nat_eq(a3161, a3162) {
                        Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C0(v5, {
                            let a3163 = v1;
                            let a3164 = 1u64;
                            nat_add(a3163, a3164)?
                        })))
                    } else {
                        let a3165 = v7;
                        let a3166 = 256u64;
                        if nat_eq(a3165, a3166) {
                            Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f111(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                        } else {
                            let a3167 = v7;
                            let a3168 = 92u64;
                            if nat_eq(a3167, a3168) {
                                let v8: u64 = f212(v0.clone(), {
                                    let a3169 = v1;
                                    let a3170 = 1u64;
                                    nat_add(a3169, a3170)?
                                })?;
                                let m3171 = f551(v8)?;
                                match m3171 {
                                    None => {
                                        let a3176 = v8;
                                        let a3177 = 10u64;
                                        if nat_eq(a3176, a3177) {
                                            if v3 {
                                                Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f552(v1)?)))
                                            } else {
                                                let m3180 = f553(v0.clone(), {
                                                    let a3178 = v1;
                                                    let a3179 = 1u64;
                                                    nat_add(a3178, a3179)?
                                                }, v2, v6, 0u64)?;
                                                match m3180 {
                                                    Adt34::C0(v11, v12) => {
                                                        Ok::<(u64, (u64, Bytes)), R<Adt30>>((v12, (v6, {
                                                            let a3181 = v5;
                                                            let a3182 = f483(v11);
                                                            append_bytes(a3181, a3182)
                                                        })))
                                                    }
                                                    Adt34::C1(v10) => {
                                                        Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(v10)))
                                                    }
                                                }
                                            }
                                        } else {
                                            let v13: u64 = {
                                                let a3183 = v8;
                                                let a3184 = 120u64;
                                                if nat_eq(a3183, a3184) {
                                                    2u64
                                                } else {
                                                    let a3185 = v8;
                                                    let a3186 = 117u64;
                                                    if nat_eq(a3185, a3186) {
                                                        4u64
                                                    } else {
                                                        let a3187 = v8;
                                                        let a3188 = 85u64;
                                                        if nat_eq(a3187, a3188) {
                                                            8u64
                                                        } else {
                                                            0u64
                                                        }
                                                    }
                                                }
                                            };
                                            let a3189 = v13;
                                            let a3190 = 0u64;
                                            if nat_eq(a3189, a3190) {
                                                Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f111(Str::lit("an escape must be one of YAML 1.2.2 [42] to [62]"), v1)?)))
                                            } else {
                                                let m3193 = f554(v0.clone(), {
                                                    let a3191 = v1;
                                                    let a3192 = 2u64;
                                                    nat_add(a3191, a3192)?
                                                }, v13, 0u64)?;
                                                match m3193 {
                                                    None => {
                                                        Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f111(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                    }
                                                    Some(v14) => {
                                                        let a3194 = 1114111u64;
                                                        let a3195 = v14;
                                                        if if nat_lt(a3194, a3195) {
                                                            true
                                                        } else {
                                                            let a3196 = 55296u64;
                                                            let a3197 = v14;
                                                            if nat_le(a3196, a3197) {
                                                                let a3198 = v14;
                                                                let a3199 = 57343u64;
                                                                nat_le(a3198, a3199)
                                                            } else {
                                                                false
                                                            }
                                                        } {
                                                            Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f111(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                        } else {
                                                            Ok::<(u64, (u64, Bytes)), R<Adt30>>(({
                                                                let a3202 = {
                                                                    let a3200 = v1;
                                                                    let a3201 = 2u64;
                                                                    nat_add(a3200, a3201)?
                                                                };
                                                                let a3203 = v13;
                                                                nat_add(a3202, a3203)?
                                                            }, (v6, {
                                                                let a3204 = v5;
                                                                let a3205 = f361(v14)?;
                                                                append_bytes(a3204, a3205)
                                                            })))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Some(v9) => {
                                        Ok::<(u64, (u64, Bytes)), R<Adt30>>(({
                                            let a3172 = v1;
                                            let a3173 = 2u64;
                                            nat_add(a3172, a3173)?
                                        }, (v6, {
                                            let a3174 = v5;
                                            let a3175 = f361(v9)?;
                                            append_bytes(a3174, a3175)
                                        })))
                                    }
                                }
                            } else {
                                if if f290(v7) {
                                    true
                                } else {
                                    let a3206 = v7;
                                    let a3207 = 10u64;
                                    nat_eq(a3206, a3207)
                                } {
                                    let v15: u64 = f293(v0.clone(), v1)?;
                                    let a3208 = f212(v0.clone(), v15)?;
                                    let a3209 = 10u64;
                                    if nat_eq(a3208, a3209) {
                                        if v3 {
                                            Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f552(v1)?)))
                                        } else {
                                            let m3210 = f553(v0.clone(), v15, v2, v6, 0u64)?;
                                            match m3210 {
                                                Adt34::C0(v17, v18) => {
                                                    Ok::<(u64, (u64, Bytes)), R<Adt30>>((v18, (v6, {
                                                        let a3211 = v5;
                                                        let a3212 = f494(v17);
                                                        append_bytes(a3211, a3212)
                                                    })))
                                                }
                                                Adt34::C1(v16) => {
                                                    Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(v16)))
                                                }
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (u64, Bytes)), R<Adt30>>((v15, (v6, {
                                            let a3213 = v5;
                                            let a3214 = f302(v0.clone(), v1, v15);
                                            append_bytes(a3213, a3214)
                                        })))
                                    }
                                } else {
                                    let v19: u64 = f555(v0.clone(), v1)?;
                                    Ok::<(u64, (u64, Bytes)), R<Adt30>>((v19, (v6, {
                                        let a3215 = v5;
                                        let a3216 = f302(v0.clone(), v1, v19);
                                        append_bytes(a3215, a3216)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3217 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f487(v0: Bytes, p3248: u64, v2: u64, v3: bool, p3249: u64, p3250: Bytes) -> R<Adt30> {
    let m3247 = (p3248, (p3249, p3250));
    {
        let mut m3247 = m3247;
        loop {
            let step = {
                let (v1, (v4, v5)) = m3247;
                let m3221 = v4;
                if m3221 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f111(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                } else {
                    let v6 = m3221 - 1;
                    let v7: u64 = f212(v0.clone(), v1)?;
                    let a3222 = v7;
                    let a3223 = 39u64;
                    if nat_eq(a3222, a3223) {
                        let a3226 = f212(v0.clone(), {
                            let a3224 = v1;
                            let a3225 = 1u64;
                            nat_add(a3224, a3225)?
                        })?;
                        let a3227 = 39u64;
                        if nat_eq(a3226, a3227) {
                            Ok::<(u64, (u64, Bytes)), R<Adt30>>(({
                                let a3228 = v1;
                                let a3229 = 2u64;
                                nat_add(a3228, a3229)?
                            }, (v6, {
                                let a3230 = v5;
                                let a3231 = Bytes::lit(&[0x27]);
                                append_bytes(a3230, a3231)
                            })))
                        } else {
                            Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C0(v5, {
                                let a3232 = v1;
                                let a3233 = 1u64;
                                nat_add(a3232, a3233)?
                            })))
                        }
                    } else {
                        let a3234 = v7;
                        let a3235 = 256u64;
                        if nat_eq(a3234, a3235) {
                            Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f111(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                        } else {
                            if if f290(v7) {
                                true
                            } else {
                                let a3236 = v7;
                                let a3237 = 10u64;
                                nat_eq(a3236, a3237)
                            } {
                                let v8: u64 = f293(v0.clone(), v1)?;
                                let a3238 = f212(v0.clone(), v8)?;
                                let a3239 = 10u64;
                                if nat_eq(a3238, a3239) {
                                    if v3 {
                                        Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f552(v1)?)))
                                    } else {
                                        let m3240 = f553(v0.clone(), v8, v2, v6, 0u64)?;
                                        match m3240 {
                                            Adt34::C0(v10, v11) => {
                                                Ok::<(u64, (u64, Bytes)), R<Adt30>>((v11, (v6, {
                                                    let a3241 = v5;
                                                    let a3242 = f494(v10);
                                                    append_bytes(a3241, a3242)
                                                })))
                                            }
                                            Adt34::C1(v9) => {
                                                Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(v9)))
                                            }
                                        }
                                    }
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Adt30>>((v8, (v6, {
                                        let a3243 = v5;
                                        let a3244 = f302(v0.clone(), v1, v8);
                                        append_bytes(a3243, a3244)
                                    })))
                                }
                            } else {
                                let v12: u64 = f556(v0.clone(), v1)?;
                                Ok::<(u64, (u64, Bytes)), R<Adt30>>((v12, (v6, {
                                    let a3245 = v5;
                                    let a3246 = f302(v0.clone(), v1, v12);
                                    append_bytes(a3245, a3246)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3247 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f488(v0: Bytes, v1: u64, v2: u64) -> R<Result<u64, Bytes>> {
    let v3: (u64, bool) = {
        let (h3251, _) = f557(Fn9::F558(v2, v0.clone()), {
            let a3253 = {
                let a3252 = v0.clone();
                length_bytes(a3252)
            };
            let a3254 = 1u64;
            nat_add(a3253, a3254)?
        }, (v1, false))?;
        h3251
    };
    let (_, h3255) = v3;
    if h3255 {
        if f210(v0, {
            let (h3256, _) = v3;
            h3256
        })? {
            Ok(Err::<u64, Bytes>(f111(Str::lit("a document marker cannot appear inside a flow collection (YAML 1.2.2 [206])"), {
                let (h3257, _) = v3;
                h3257
            })?))
        } else {
            Ok(Err::<u64, Bytes>(f111(Str::lit("a line of a flow collection is indented less than its node (YAML 1.2.2 [80])"), {
                let (h3258, _) = v3;
                h3258
            })?))
        }
    } else {
        Ok(Ok::<u64, Bytes>({
            let (h3259, _) = v3;
            h3259
        }))
    }
}

pub fn f489(v0: Bytes, v1: u64, v2: u64) -> R<Adt30> {
    let v3: u64 = f212(v0.clone(), v1)?;
    let a3260 = v3;
    let a3261 = 34u64;
    if if nat_eq(a3260, a3261) {
        true
    } else {
        let a3262 = v3;
        let a3263 = 39u64;
        nat_eq(a3262, a3263)
    } {
        f397(v0, v1, v2, false)
    } else {
        let m3264 = f298(v0.clone(), v1)?;
        match m3264 {
            None => {
                let a3265 = v3;
                let a3266 = 91u64;
                if if nat_eq(a3265, a3266) {
                    true
                } else {
                    let a3267 = v3;
                    let a3268 = 123u64;
                    nat_eq(a3267, a3268)
                } {
                    Ok(Adt30::C1(f111(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [142])"), v1)?))
                } else {
                    if f399(v0.clone(), v1, true)? {
                        let v5: u64 = f400(v0.clone(), v1, true)?;
                        let v6: u64 = f293(v0.clone(), v5)?;
                        let a3269 = f212(v0.clone(), v6)?;
                        let a3270 = 10u64;
                        if nat_eq(a3269, a3270) {
                            let m3271 = f488(v0.clone(), v6, v2)?;
                            match m3271 {
                                Ok(v8) => {
                                    let v9: u64 = f212(v0.clone(), v8)?;
                                    let a3272 = v9;
                                    let a3273 = 58u64;
                                    if if if nat_eq(a3272, a3273) {
                                        true
                                    } else {
                                        let a3274 = v9;
                                        let a3275 = 44u64;
                                        nat_eq(a3274, a3275)
                                    } {
                                        true
                                    } else {
                                        let a3276 = v9;
                                        let a3277 = 125u64;
                                        nat_eq(a3276, a3277)
                                    } {
                                        Ok(Adt30::C0(f302(v0, v1, v5), v5))
                                    } else {
                                        Ok(Adt30::C1(f111(Str::lit("a multi-line plain scalar inside a flow collection is not read (YAML 1.2.2 [135])"), v6)?))
                                    }
                                }
                                Err(v7) => {
                                    Ok(Adt30::C1(v7))
                                }
                            }
                        } else {
                            Ok(Adt30::C0(f302(v0, v1, v5), v5))
                        }
                    } else {
                        Ok(Adt30::C1(f111(Str::lit("a flow mapping key must be a scalar (YAML 1.2.2 [142])"), v1)?))
                    }
                }
            }
            Some(v4) => {
                Ok(Adt30::C1(v4))
            }
        }
    }
}

pub fn f490(v0: u64) -> R<bool> {
    let a3278 = f301(f8(Str::lit(",[]{}-?:#&*!|>'\"@`")), v0)?;
    let a3279 = 18u64;
    if nat_lt(a3278, a3279) {
        Ok(true)
    } else {
        let a3280 = v0;
        let a3281 = 37u64;
        Ok(nat_eq(a3280, a3281))
    }
}

pub fn f491(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f559(v1, v0, v2)
}

pub fn f492(v0: u64) -> R<Bytes> {
    f111(Str::lit("a mapping value is not allowed here: a plain scalar holds ': ' (YAML 1.2.2 [130])"), v0)
}

pub fn f493(v0: Bytes, v1: u64) -> R<(u64, u64)> {
    let (h3282, _) = f220(Fn5::F560(v0.clone()), {
        let a3284 = {
            let a3283 = v0;
            length_bytes(a3283)
        };
        let a3285 = 1u64;
        nat_add(a3284, a3285)?
    }, (v1, 0u64))?;
    Ok(h3282)
}

pub fn f494(v0: u64) -> Bytes {
    let a3286 = v0;
    let a3287 = 0u64;
    if nat_eq(a3286, a3287) {
        Bytes::lit(&[0x20])
    } else {
        f483(v0)
    }
}

pub fn f495(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt20> {
    let v4: (u64, u64) = f561(v0.clone(), v1, 2u64)?;
    let a3290 = {
        let (h3288, _) = v4;
        h3288
    };
    let a3291 = {
        let a3289 = v0.clone();
        length_bytes(a3289)
    };
    if nat_eq(a3290, a3291) {
        if f143(false, {
            let (_, h3292) = v4;
            h3292
        }) {
            Ok(Adt20::C0(Adt2::C3(false, {
                let (_, h3293) = v4;
                h3293
            }), v3))
        } else {
            Ok(Adt20::C1(f497(v2)?))
        }
    } else {
        Ok(Adt20::C0(Adt2::C5(v0), v3))
    }
}

pub fn f496(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a3294 = v0.clone();
        length_bytes(a3294)
    };
    let v2: u64 = f212(v0.clone(), 0u64)?;
    let v3: u64 = {
        let a3295 = v2;
        let a3296 = 43u64;
        if if nat_eq(a3295, a3296) {
            true
        } else {
            let a3297 = v2;
            let a3298 = 45u64;
            nat_eq(a3297, a3298)
        } {
            1u64
        } else {
            0u64
        }
    };
    let v4: u64 = f138(v0.clone(), v3)?;
    let v5: bool = {
        let a3299 = f212(v0.clone(), v4)?;
        let a3300 = 46u64;
        nat_eq(a3299, a3300)
    };
    let v6: u64 = if v5 {
        let a3301 = v4;
        let a3302 = 1u64;
        nat_add(a3301, a3302)?
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
        let a3303 = v8;
        let a3304 = 101u64;
        if nat_eq(a3303, a3304) {
            true
        } else {
            let a3305 = v8;
            let a3306 = 69u64;
            nat_eq(a3305, a3306)
        }
    };
    let v10: u64 = f212(v0.clone(), {
        let a3307 = v7;
        let a3308 = 1u64;
        nat_add(a3307, a3308)?
    })?;
    let v11: bool = if v9 {
        let a3309 = v10;
        let a3310 = 43u64;
        if nat_eq(a3309, a3310) {
            true
        } else {
            let a3311 = v10;
            let a3312 = 45u64;
            nat_eq(a3311, a3312)
        }
    } else {
        false
    };
    let v12: u64 = if v9 {
        if v11 {
            let a3313 = v7;
            let a3314 = 2u64;
            nat_add(a3313, a3314)?
        } else {
            let a3315 = v7;
            let a3316 = 1u64;
            nat_add(a3315, a3316)?
        }
    } else {
        v7
    };
    let v13: u64 = if v9 {
        f138(v0.clone(), v12)?
    } else {
        v7
    };
    let v14: Bytes = f302(v0.clone(), v3, v4);
    let v15: u64 = f140(v14.clone())?;
    let a3317 = v13;
    let a3318 = v1;
    if if if nat_eq(a3317, a3318) {
        let a3319 = v3;
        let a3320 = v4;
        if nat_lt(a3319, a3320) {
            true
        } else {
            if v5 {
                let a3321 = v6;
                let a3322 = v7;
                nat_lt(a3321, a3322)
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
            let a3323 = v12;
            let a3324 = v13;
            nat_lt(a3323, a3324)
        }
    } else {
        false
    } {
        Ok(Some(f7(List::cons({
            let a3325 = v2;
            let a3326 = 45u64;
            if nat_eq(a3325, a3326) {
                Bytes::lit(&[0x2d])
            } else {
                Bytes::lit(&[])
            }
        }, List::cons({
            let a3328 = v15;
            let a3329 = {
                let a3327 = v14.clone();
                length_bytes(a3327)
            };
            if nat_eq(a3328, a3329) {
                Bytes::lit(&[0x30])
            } else {
                f302(v14.clone(), v15, {
                    let a3330 = v14;
                    length_bytes(a3330)
                })
            }
        }, List::cons({
            let a3331 = v6;
            let a3332 = v7;
            if nat_lt(a3331, a3332) {
                let a3333 = Bytes::lit(&[0x2e]);
                let a3334 = f302(v0.clone(), v6, v7);
                append_bytes(a3333, a3334)
            } else {
                Bytes::lit(&[])
            }
        }, List::cons(if v9 {
            let a3337 = if if v11 {
                let a3335 = v10;
                let a3336 = 45u64;
                nat_eq(a3335, a3336)
            } else {
                false
            } {
                Bytes::lit(&[0x65, 0x2d])
            } else {
                Bytes::lit(&[0x65])
            };
            let a3338 = f302(v0, v12, v13);
            append_bytes(a3337, a3338)
        } else {
            Bytes::lit(&[])
        }, List::<Bytes>::nil())))))))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f497(v0: u64) -> R<Bytes> {
    f111(Str::lit("a number outside the range Studio.Value holds is not read (YAML 1.2.2 section 10.3.2)"), v0)
}

pub fn f498(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a3340 = v2;
    let a3341 = {
        let a3339 = v0.clone();
        length_bytes(a3339)
    };
    if if nat_lt(a3340, a3341) {
        f214({
            let a3342 = f212(v0, v2)?;
            let a3343 = v1;
            nat_eq(a3342, a3343)
        })
    } else {
        false
    } {
        Ok(Some({
            let a3344 = v2;
            let a3345 = 1u64;
            nat_add(a3344, a3345)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f499(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a3390 = {
            let a3366 = {
                let a3354 = {
                    let a3348 = {
                        let a3346 = v3;
                        let a3347 = 128u8;
                        fixed_u8::bit_and(a3346, a3347)
                    };
                    let a3349 = 128u8;
                    if equal(a3348, a3349) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3355 = {
                    let a3352 = {
                        let a3350 = v3;
                        let a3351 = 64u8;
                        fixed_u8::bit_and(a3350, a3351)
                    };
                    let a3353 = 64u8;
                    if equal(a3352, a3353) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3354, a3355)?
            };
            let a3367 = {
                let a3364 = {
                    let a3358 = {
                        let a3356 = v3;
                        let a3357 = 32u8;
                        fixed_u8::bit_and(a3356, a3357)
                    };
                    let a3359 = 32u8;
                    if equal(a3358, a3359) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3365 = {
                    let a3362 = {
                        let a3360 = v3;
                        let a3361 = 16u8;
                        fixed_u8::bit_and(a3360, a3361)
                    };
                    let a3363 = 16u8;
                    if equal(a3362, a3363) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3364, a3365)?
            };
            nat_add(a3366, a3367)?
        };
        let a3391 = {
            let a3388 = {
                let a3376 = {
                    let a3370 = {
                        let a3368 = v3;
                        let a3369 = 8u8;
                        fixed_u8::bit_and(a3368, a3369)
                    };
                    let a3371 = 8u8;
                    if equal(a3370, a3371) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3377 = {
                    let a3374 = {
                        let a3372 = v3;
                        let a3373 = 4u8;
                        fixed_u8::bit_and(a3372, a3373)
                    };
                    let a3375 = 4u8;
                    if equal(a3374, a3375) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3376, a3377)?
            };
            let a3389 = {
                let a3386 = {
                    let a3380 = {
                        let a3378 = v3;
                        let a3379 = 2u8;
                        fixed_u8::bit_and(a3378, a3379)
                    };
                    let a3381 = 2u8;
                    if equal(a3380, a3381) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3387 = {
                    let a3384 = {
                        let a3382 = v3;
                        let a3383 = 1u8;
                        fixed_u8::bit_and(a3382, a3383)
                    };
                    let a3385 = 1u8;
                    if equal(a3384, a3385) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3386, a3387)?
            };
            nat_add(a3388, a3389)?
        };
        nat_add(a3390, a3391)?
    };
    let a3392 = v4;
    let a3393 = 34u64;
    if nat_eq(a3392, a3393) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a3394 = v4;
        let a3395 = 92u64;
        if nat_eq(a3394, a3395) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a3396 = v4;
            let a3397 = 8u64;
            if nat_eq(a3396, a3397) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a3398 = v4;
                let a3399 = 12u64;
                if nat_eq(a3398, a3399) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a3400 = v4;
                    let a3401 = 10u64;
                    if nat_eq(a3400, a3401) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a3402 = v4;
                        let a3403 = 13u64;
                        if nat_eq(a3402, a3403) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a3404 = v4;
                            let a3405 = 9u64;
                            if nat_eq(a3404, a3405) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a3406 = v4;
                                let a3407 = 32u64;
                                if nat_lt(a3406, a3407) {
                                    let a3408 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a3409 = f562(v2)?;
                                    Ok(append_bytes(a3408, a3409))
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

pub fn f500(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m3410 = v0.clone();
    match m3410.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a3413 = {
                let (h3411, _) = v2;
                h3411
            };
            let a3414 = {
                let (h3412, _) = v1.clone();
                h3412
            };
            if equal(a3413, a3414) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f501(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f563(f564(Fn10::F565, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m3419 = {
        let (h3415, _) = f566(Fn11::F567, {
            let a3417 = {
                let a3416 = v0;
                length_list(a3416)
            };
            let a3418 = 1u64;
            nat_add(a3417, a3418)?
        }, v1)?;
        h3415
    };
    match m3419.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f502(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a3424 = {
        let a3421 = f307({
            let (h3420, _) = v0;
            h3420
        })?;
        let a3422 = Bytes::lit(&[0x3a]);
        append_bytes(a3421, a3422)
    };
    let a3425 = {
        let (_, h3423) = v0;
        h3423
    };
    Ok(append_bytes(a3424, a3425))
}

pub fn f503(v0: Fn8, p3430: u64, p3431: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m3429 = (p3430, p3431);
    {
        let mut m3429 = m3429;
        loop {
            let step = {
                let (v1, v2) = m3429;
                let m3426 = v1;
                if m3426 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3426 - 1;
                    let m3428 = {
                        let c3427: Fn8 = v0.clone();
                        c3427.apply(v2.clone())?
                    };
                    match m3428 {
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
                Ok(next) => { m3429 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f504(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f568(v0, v1)
}

pub fn f505(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3476 = {
            let a3452 = {
                let a3440 = {
                    let a3434 = {
                        let a3432 = v1;
                        let a3433 = 128u8;
                        fixed_u8::bit_and(a3432, a3433)
                    };
                    let a3435 = 128u8;
                    if equal(a3434, a3435) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3441 = {
                    let a3438 = {
                        let a3436 = v1;
                        let a3437 = 64u8;
                        fixed_u8::bit_and(a3436, a3437)
                    };
                    let a3439 = 64u8;
                    if equal(a3438, a3439) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3440, a3441)?
            };
            let a3453 = {
                let a3450 = {
                    let a3444 = {
                        let a3442 = v1;
                        let a3443 = 32u8;
                        fixed_u8::bit_and(a3442, a3443)
                    };
                    let a3445 = 32u8;
                    if equal(a3444, a3445) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3451 = {
                    let a3448 = {
                        let a3446 = v1;
                        let a3447 = 16u8;
                        fixed_u8::bit_and(a3446, a3447)
                    };
                    let a3449 = 16u8;
                    if equal(a3448, a3449) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3450, a3451)?
            };
            nat_add(a3452, a3453)?
        };
        let a3477 = {
            let a3474 = {
                let a3462 = {
                    let a3456 = {
                        let a3454 = v1;
                        let a3455 = 8u8;
                        fixed_u8::bit_and(a3454, a3455)
                    };
                    let a3457 = 8u8;
                    if equal(a3456, a3457) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3463 = {
                    let a3460 = {
                        let a3458 = v1;
                        let a3459 = 4u8;
                        fixed_u8::bit_and(a3458, a3459)
                    };
                    let a3461 = 4u8;
                    if equal(a3460, a3461) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3462, a3463)?
            };
            let a3475 = {
                let a3472 = {
                    let a3466 = {
                        let a3464 = v1;
                        let a3465 = 2u8;
                        fixed_u8::bit_and(a3464, a3465)
                    };
                    let a3467 = 2u8;
                    if equal(a3466, a3467) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3473 = {
                    let a3470 = {
                        let a3468 = v1;
                        let a3469 = 1u8;
                        fixed_u8::bit_and(a3468, a3469)
                    };
                    let a3471 = 1u8;
                    if equal(a3470, a3471) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3472, a3473)?
            };
            nat_add(a3474, a3475)?
        };
        nat_add(a3476, a3477)?
    };
    let a3478 = 48u64;
    let a3479 = v2;
    if nat_le(a3478, a3479) {
        let a3480 = v2;
        let a3481 = 57u64;
        Ok(nat_le(a3480, a3481))
    } else {
        Ok(false)
    }
}

pub fn f506(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    if f569(v1, f115(v0, v2)?) {
        Ok(Some({
            let a3482 = v2;
            let a3483 = 1u64;
            nat_add(a3482, a3483)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f507(p3486: List<Adt14>, p3487: List<Adt14>) -> List<Adt14> {
    let m3485 = (p3486, p3487);
    {
        let mut m3485 = m3485;
        loop {
            let step = {
                let (v0, v1) = m3485;
                let m3484 = v0;
                match m3484.uncons() {
                    None => {
                        Err::<(List<Adt14>, List<Adt14>), List<Adt14>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt14>, List<Adt14>), List<Adt14>>((v3, List::cons(v2, v1)))
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

pub fn f508(v0: Bytes, p3492: List<(Bytes, Adt15)>) -> Option<Adt15> {
    let m3491 = p3492;
    {
        let mut m3491 = m3491;
        loop {
            let step = {
                let v1 = m3491;
                let m3488 = v1;
                match m3488.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt15)>, Option<Adt15>>(None::<Adt15>)
                    }
                    Some((v2, v3)) => {
                        if f46({
                            let (h3489, _) = v2;
                            h3489
                        }, v0.clone()) {
                            Err::<List<(Bytes, Adt15)>, Option<Adt15>>(Some({
                                let (_, h3490) = v2;
                                h3490
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt15)>, Option<Adt15>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3491 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f509() -> List<(Bytes, Adt15)> {
    List::cons((f8(Str::lit("boolean")), f336(f8(Str::lit("Bool")))), List::cons((f8(Str::lit("byte")), f336(f8(Str::lit("Int")))), List::cons((f8(Str::lit("octet")), f336(f8(Str::lit("Nat")))), List::cons((f8(Str::lit("bigint")), f336(f8(Str::lit("Int")))), List::cons((f8(Str::lit("float")), f336(f456())), List::cons((f8(Str::lit("double")), f336(f456())), List::cons((f8(Str::lit("DOMString")), f336(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("USVString")), f336(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ByteString")), f336(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ArrayBuffer")), f336(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("SharedArrayBuffer")), f336(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("DataView")), f336(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("Int8Array")), f336(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int16Array")), f336(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int32Array")), f336(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("BigInt64Array")), f336(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Uint8Array")), f336(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint16Array")), f336(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint32Array")), f336(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint8ClampedArray")), f336(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("BigUint64Array")), f336(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Float16Array")), f432(f8(Str::lit("Float16Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("Float32Array")), f432(f8(Str::lit("Float32Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("Float64Array")), f432(f8(Str::lit("Float64Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("any")), f432(f8(Str::lit("any is not translated, because it is the union of every Web IDL type and has no one LexLean type (Web IDL 2.13.1)")))), List::cons((f8(Str::lit("object")), f432(f8(Str::lit("object is not translated, because its values are references to JavaScript objects of any shape, which no LexLean type holds (Web IDL 2.13.20)")))), List::cons((f8(Str::lit("symbol")), f432(f8(Str::lit("symbol is not translated, because a symbol value has identity but no content that a LexLean type holds (Web IDL 2.13.21)")))), List::cons((f8(Str::lit("undefined")), f432(f8(Str::lit("undefined is not translated, because it is the type of the absence of a value (Web IDL 2.13.2)")))), List::<(Bytes, Adt15)>::nil()))))))))))))))))))))))))))))
}

pub fn f510() -> Bytes {
    f8(Str::lit("a union type is translated only as the whole body of a typedef, whose identifier names its inductive (Web IDL 2.11 and 2.13.32)"))
}

pub fn f511(v0: u64) -> Bytes {
    f43({
        let a3493 = Str::lit("0123456789ABCDEF");
        utf8_encode(a3493)
    }, v0, 1u64)
}

pub fn f512(p3504: u64, v1: Bytes, v2: Bytes, p3505: u64) -> R<bool> {
    let m3503 = (p3504, p3505);
    {
        let mut m3503 = m3503;
        loop {
            let step = {
                let (v0, v3) = m3503;
                let m3494 = v0;
                if m3494 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(false))
                } else {
                    let v4 = m3494 - 1;
                    let v5: u64 = f570(v1.clone(), v3)?;
                    let v6: u64 = f570(v2.clone(), v3)?;
                    let a3495 = v5;
                    let a3496 = v6;
                    if nat_lt(a3495, a3496) {
                        Err::<(u64, u64), R<bool>>(Ok(true))
                    } else {
                        let a3497 = v6;
                        let a3498 = v5;
                        if nat_lt(a3497, a3498) {
                            Err::<(u64, u64), R<bool>>(Ok(false))
                        } else {
                            let a3499 = v5;
                            let a3500 = 0u64;
                            if nat_eq(a3499, a3500) {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            } else {
                                Ok::<(u64, u64), R<bool>>((v4, {
                                    let a3501 = v3;
                                    let a3502 = 1u64;
                                    nat_add(a3501, a3502)?
                                }))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3503 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f513(p3518: u64, v1: Bytes, p3519: u64, p3520: Bytes) -> R<Option<Bytes>> {
    let m3517 = (p3518, (p3519, p3520));
    {
        let mut m3517 = m3517;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3517;
                let m3506 = v0;
                if m3506 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3506 - 1;
                    let a3508 = {
                        let a3507 = v1.clone();
                        length_bytes(a3507)
                    };
                    let a3509 = v2;
                    if nat_le(a3508, a3509) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3510 = f354(v1.clone(), v2)?;
                        match m3510 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3512 = v2;
                                    let a3513 = {
                                        let (_, h3511) = v5;
                                        h3511
                                    };
                                    nat_add(a3512, a3513)?
                                }, {
                                    let a3515 = v3;
                                    let a3516 = f571(v1.clone(), v2, {
                                        let (h3514, _) = v5;
                                        h3514
                                    });
                                    append_bytes(a3515, a3516)
                                })))
                            }
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

pub fn f514(p3530: u64, v1: Bytes, p3531: u64) -> R<bool> {
    let m3529 = (p3530, p3531);
    {
        let mut m3529 = m3529;
        loop {
            let step = {
                let (v0, v2) = m3529;
                let m3521 = v0;
                if m3521 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m3521 - 1;
                    let m3524 = {
                        let a3522 = v1.clone();
                        let a3523 = v2;
                        index_bytes(a3522, a3523)
                    };
                    match m3524 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a3525 = f52(v4)?;
                            let a3526 = 128u64;
                            if nat_lt(a3525, a3526) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a3527 = v2;
                                    let a3528 = 1u64;
                                    nat_add(a3527, a3528)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3529 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f515(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f266(v0.clone(), 0u64, {
        let a3532 = v0;
        length_bytes(a3532)
    }, false, Bytes::lit(&[]))?;
    let m3535 = {
        let a3533 = v1.clone();
        let a3534 = 0u64;
        index_bytes(a3533, a3534)
    };
    match m3535 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f52(v2)?;
            if f369(v3) {
                let a3542 = f43({
                    let a3536 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a3536)
                }, {
                    let a3537 = v3;
                    let a3538 = 65u64;
                    nat_sub(a3537, a3538)
                }, 1u64);
                let a3543 = f43(v1.clone(), 1u64, {
                    let a3540 = {
                        let a3539 = v1;
                        length_bytes(a3539)
                    };
                    let a3541 = 1u64;
                    nat_sub(a3540, a3541)
                });
                Ok(append_bytes(a3542, a3543))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f516(v0: Bytes) -> R<bool> {
    let m3546 = {
        let a3544 = v0;
        let a3545 = 0u64;
        index_bytes(a3544, a3545)
    };
    match m3546 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f139(f368(f52(v1)?), f369(f52(v1)?)))
        }
    }
}

pub fn f517(p3559: u64, v1: Bytes, p3560: u64, p3561: Bytes) -> R<Option<Bytes>> {
    let m3558 = (p3559, (p3560, p3561));
    {
        let mut m3558 = m3558;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3558;
                let m3547 = v0;
                if m3547 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3547 - 1;
                    let a3549 = {
                        let a3548 = v1.clone();
                        length_bytes(a3548)
                    };
                    let a3550 = v2;
                    if nat_le(a3549, a3550) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3551 = f354(v1.clone(), v2)?;
                        match m3551 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3553 = v2;
                                    let a3554 = {
                                        let (_, h3552) = v5;
                                        h3552
                                    };
                                    nat_add(a3553, a3554)?
                                }, {
                                    let a3556 = v3;
                                    let a3557 = f572({
                                        let (h3555, _) = v5;
                                        h3555
                                    });
                                    append_bytes(a3556, a3557)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3558 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f518() -> List<Bytes> {
    List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("termination_by")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("admit")), List::cons(f8(Str::lit("below")), List::cons(f8(Str::lit("brecOn")), List::cons(f8(Str::lit("casesOn")), List::cons(f8(Str::lit("ctorElim")), List::cons(f8(Str::lit("ctorElimType")), List::cons(f8(Str::lit("ctorIdx")), List::cons(f8(Str::lit("noConfusion")), List::cons(f8(Str::lit("noConfusionType")), List::cons(f8(Str::lit("rec")), List::cons(f8(Str::lit("recOn")), List::cons(f8(Str::lit("toCtorIdx")), List::<Bytes>::nil())))))))))))))))))))
}

pub fn f519() -> Bytes {
    f8(Str::lit("\u{0}\u{0}:\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{5}\u{7f}\u{0}\u{6}p\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f520() -> Bytes {
    f8(Str::lit("\u{0}\u{0}-\u{0}\u{0}.\u{0}\u{0}0\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}7\u{0}\u{1}7\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@?\u{0}@@\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f521(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f115(v0, {
        let (h3562, _) = v2;
        h3562
    })?;
    let a3564 = 1114111u64;
    let a3565 = {
        let (_, h3563) = v2;
        h3563
    };
    if nat_lt(a3564, a3565) {
        Ok(None::<(u64, u64)>)
    } else {
        let v4: u64 = f573(v3, v1);
        let a3566 = v4;
        let a3567 = 16u64;
        if nat_eq(a3566, a3567) {
            Ok(None::<(u64, u64)>)
        } else {
            Ok(Some(({
                let a3569 = {
                    let (h3568, _) = v2;
                    h3568
                };
                let a3570 = 1u64;
                nat_add(a3569, a3570)?
            }, {
                let a3574 = {
                    let a3572 = {
                        let (_, h3571) = v2;
                        h3571
                    };
                    let a3573 = if v1 {
                        16u64
                    } else {
                        10u64
                    };
                    nat_mul(a3572, a3573)?
                };
                let a3575 = v4;
                nat_add(a3574, a3575)?
            })))
        }
    }
}

pub fn f522() -> Bytes {
    f8(Str::lit("\u{80}\u{81}\u{82}\u{83}\u{84}\u{85}\u{86}\u{87}\u{88}\u{89}\u{8a}\u{8b}\u{8c}\u{8d}\u{8e}\u{8f}\u{90}\u{91}\u{92}\u{93}\u{94}\u{95}\u{96}\u{97}\u{98}\u{99}\u{9a}\u{9b}\u{9c}\u{9d}\u{9e}\u{9f}\u{a0}\u{a1}\u{a2}\u{a3}\u{a4}\u{a5}\u{a6}\u{a7}\u{a8}\u{a9}\u{aa}\u{ab}\u{ac}\u{ad}\u{ae}\u{af}\u{b0}\u{b1}\u{b2}\u{b3}\u{b4}\u{b5}\u{b6}\u{b7}\u{b8}\u{b9}\u{ba}\u{bb}\u{bc}\u{bd}\u{be}\u{bf}"))
}

pub fn f523(v0: u64, v1: bool, v2: u64) -> R<bool> {
    if v1 {
        let a3576 = v0;
        let a3577 = v2;
        if nat_eq(a3576, a3577) {
            Ok(true)
        } else {
            f250(v0, f574())
        }
    } else {
        f250(v0, f575())
    }
}

pub fn f524() -> Bytes {
    f8(Str::lit(" string:anyAtomicType boolean:anyAtomicType decimal:anyAtomicType float:anyAtomicType double:anyAtomicType duration:anyAtomicType dateTime:anyAtomicType time:anyAtomicType date:anyAtomicType gYearMonth:anyAtomicType gYear:anyAtomicType gMonthDay:anyAtomicType gDay:anyAtomicType gMonth:anyAtomicType hexBinary:anyAtomicType base64Binary:anyAtomicType anyURI:anyAtomicType QName:anyAtomicType NOTATION:anyAtomicType normalizedString:string token:normalizedString language:token NMTOKEN:token Name:token NCName:Name ID:NCName IDREF:NCName ENTITY:NCName integer:decimal nonPositiveInteger:integer long:integer nonNegativeInteger:integer negativeInteger:nonPositiveInteger int:long short:int byte:short unsignedLong:nonNegativeInteger positiveInteger:nonNegativeInteger unsignedInt:unsignedLong unsignedShort:unsignedInt unsignedByte:unsignedShort yearMonthDuration:duration dayTimeDuration:duration dateTimeStamp:dateTime NMTOKENS:anySimpleType IDREFS:anySimpleType ENTITIES:anySimpleType anyAtomicType:anySimpleType "))
}

pub fn f525(v0: Adt25, v1: Str, v2: Str) -> bool {
    let m3578 = f284(v0, v1);
    match m3578 {
        None => {
            false
        }
        Some(v3) => {
            f46(v3, f8(v2))
        }
    }
}

pub fn f526(v0: List<Adt24>, v1: (Bytes, Bytes), v2: bool) -> R<Result<List<Bytes>, Bytes>> {
    let m3579 = v0;
    match m3579.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m3580 = f526(v4, v1.clone(), v2)?;
            match m3580 {
                Ok(v6) => {
                    if f46(match v3.clone() {
                        Adt24::C0(h3581, _, _, _) => {
                            h3581
                        }
                    }, f8(Str::lit("complexType"))) {
                        let m3582 = f576(v3.clone())?;
                        match m3582 {
                            Ok(v8) => {
                                let m3583 = v8;
                                match m3583 {
                                    None => {
                                        Ok(Ok::<List<Bytes>, Bytes>(v6))
                                    }
                                    Some(v9) => {
                                        if if f46({
                                            let (h3584, _) = {
                                                let (_, h3585) = v9.clone();
                                                h3585
                                            };
                                            h3584
                                        }, {
                                            let (h3586, _) = v1;
                                            h3586
                                        }) {
                                            if f46({
                                                let (_, h3587) = {
                                                    let (_, h3588) = v9;
                                                    h3588
                                                };
                                                h3587
                                            }, {
                                                let (_, h3589) = v1;
                                                h3589
                                            }) {
                                                if f282(v2) {
                                                    true
                                                } else {
                                                    f283({
                                                        let (h3590, _) = v9;
                                                        h3590
                                                    }, Str::lit("extension"))
                                                }
                                            } else {
                                                false
                                            }
                                        } else {
                                            false
                                        } {
                                            Ok(Ok::<List<Bytes>, Bytes>(List::cons(match v3 {
                                                Adt24::C0(_, _, h3591, _) => {
                                                    h3591
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

pub fn f527(v0: Bytes, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<Adt26, Bytes>> {
    let m3592 = v1.clone();
    match m3592.uncons() {
        None => {
            Ok(Err::<Adt26, Bytes>(f466(f8(Str::lit("a choice with no particle that can occur matches no element sequence, so it has no LexLean constructor (XSD 1.1 Part 1, section 3.8.4.3)")), v0)))
        }
        Some((_, _)) => {
            let m3593 = f577(v1)?;
            match m3593 {
                Ok(v7) => {
                    let v8: List<Bytes> = f578(v7, v2);
                    Ok(Ok::<Adt26, Bytes>(Adt26::C0(v0.clone(), v3, f240(v0, v8.clone()), v8)))
                }
                Err(v6) => {
                    Ok(Err::<Adt26, Bytes>(f466(v6, v0)))
                }
            }
        }
    }
}

pub fn f528(v0: List<Bytes>) -> R<List<Bytes>> {
    let m3594 = v0;
    match m3594.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f191(v1)?, f528(v2)?))
        }
    }
}

pub fn f529(v0: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: Adt25) -> R<Result<List<Adt26>, Bytes>> {
    let m3595 = v0;
    if m3595 == 0 {
        Ok(Err::<List<Adt26>, Bytes>(f466(f8(Str::lit("the complex types nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m3595 - 1;
        let m3596 = f579(v5, v1, v2.clone(), v3, v4, f469())?;
        match m3596 {
            Ok(v7) => {
                let m3598 = f537(v2, match v7.clone() {
                    Adt32::C0(h3597, _, _, _, _) => {
                        h3597
                    }
                })?;
                match m3598 {
                    Ok(v9) => {
                        Ok(Ok::<List<Adt26>, Bytes>(f375(match v7 {
                            Adt32::C0(_, h3599, _, _, _) => {
                                h3599
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

pub fn f530(v0: Bytes) -> Bytes {
    let a3600 = v0;
    let a3601 = f8(Str::lit("Content"));
    append_bytes(a3600, a3601)
}

pub fn f531(v0: Adt23, v1: Adt25, v2: Bytes) -> R<Result<Adt22, Bytes>> {
    let m3603 = f119(match v1 {
        Adt25::C0(_, _, _, _, h3602) => {
            h3602
        }
    }, v2.clone())?;
    match m3603 {
        Ok(v4) => {
            if f46({
                let (h3604, _) = v4.clone();
                h3604
            }, f188()) {
                if f46({
                    let (_, h3605) = v4.clone();
                    h3605
                }, f8(Str::lit("anyType"))) {
                    Ok(Ok::<Adt22, Bytes>(Adt22::C0(f378(), List::cons(f378(), List::<Bytes>::nil()))))
                } else {
                    let m3607 = f364({
                        let (_, h3606) = v4;
                        h3606
                    }, 16u64)?;
                    match m3607 {
                        Ok(v6) => {
                            Ok(Ok::<Adt22, Bytes>(Adt22::C0(v6, List::<Bytes>::nil())))
                        }
                        Err(v5) => {
                            Ok(Err::<Adt22, Bytes>(v5))
                        }
                    }
                }
            } else {
                if f153({
                    let (h3608, _) = v4.clone();
                    h3608
                }, match v0.clone() {
                    Adt23::C0(_, h3609) => {
                        h3609
                    }
                }) {
                    let v7: Bytes = f191({
                        let (_, h3610) = v4.clone();
                        h3610
                    })?;
                    let m3611 = f538(v0.clone(), Str::lit("complexType"), v4.clone());
                    match m3611 {
                        None => {
                            let m3612 = f538(v0, Str::lit("simpleType"), v4);
                            match m3612 {
                                None => {
                                    Ok(Err::<Adt22, Bytes>(f200(f8(Str::lit("the type ")), v2, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)")))))
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
                    Ok(Err::<Adt22, Bytes>(f200(f8(Str::lit("the type ")), v2, f200(f8(Str::lit(" is in the namespace ")), {
                        let (h3613, _) = v4;
                        h3613
                    }, f8(Str::lit(", which no schema document of this set targets (XSD 1.1 Part 1, sections 3.17.6.2 and 4.2.6)"))))))
                }
            }
        }
        Err(v3) => {
            Ok(Err::<Adt22, Bytes>(v3))
        }
    }
}

pub fn f532(v0: Adt25, v1: Bytes) -> Bytes {
    if f525(v0, Str::lit("nillable"), Str::lit("true")) {
        f580(Str::lit("Option"), v1)
    } else {
        v1
    }
}

pub fn f533(p3618: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: bool, p3619: List<Adt25>, p3620: Adt32) -> R<Result<Adt32, Bytes>> {
    let m3617 = (p3618, (p3619, p3620));
    {
        let mut m3617 = m3617;
        loop {
            let step = {
                let (v0, (v5, v6)) = m3617;
                let m3614 = v0;
                if m3614 == 0 {
                    Err::<(u64, (List<Adt25>, Adt32)), R<Result<Adt32, Bytes>>>(Ok(Err::<Adt32, Bytes>(f466(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2.clone()))))
                } else {
                    let v7 = m3614 - 1;
                    let m3615 = v5;
                    match m3615.uncons() {
                        None => {
                            Err::<(u64, (List<Adt25>, Adt32)), R<Result<Adt32, Bytes>>>(Ok(Ok::<Adt32, Bytes>(v6)))
                        }
                        Some((v8, v9)) => {
                            let m3616 = f581(v7, v1.clone(), v2.clone(), v3.clone(), v4, v8, v6)?;
                            match m3616 {
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
                Ok(next) => { m3617 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f534(v0: List<Adt33>) -> List<Bytes> {
    let m3621 = v0;
    match m3621.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt33::C0(h3622, _, _) => {
                    h3622
                }
            }, f534(v2))
        }
    }
}

pub fn f535(v0: List<Adt33>) -> List<Bytes> {
    let m3623 = v0;
    match m3623.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt33::C0(_, h3624, _) => {
                    h3624
                }
            }, f535(v2))
        }
    }
}

pub fn f536(v0: List<Adt33>) -> List<Bytes> {
    let m3625 = v0;
    match m3625.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f385(match v1 {
                Adt33::C0(_, _, h3626) => {
                    h3626
                }
            }, f536(v2))
        }
    }
}

pub fn f537(v0: Bytes, v1: List<Adt33>) -> R<Result<Adt26, Bytes>> {
    let m3627 = f577(f534(v1.clone()))?;
    match m3627 {
        Ok(v3) => {
            let v4: List<Bytes> = f535(v1.clone());
            let v7: Bytes = {
                let m3628 = v1.clone();
                match m3628.uncons() {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some((_, _)) => {
                        f200(f8(Str::lit("\u{a}")), f103(f582(v3, v4.clone()), f8(Str::lit(",\u{a}"))), f8(Str::lit("\u{a}  ")))
                    }
                }
            };
            Ok(Ok::<Adt26, Bytes>(Adt26::C0(v0.clone(), f536(v1), f200(f8(Str::lit("structure{\u{a}  name: ")), v0, f200(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), v7, f8(Str::lit("]\u{a}}\u{a}")))), List::cons(f200(f8(Str::lit("    {name: mk, fields: [")), f103(v4, f8(Str::lit(", "))), f8(Str::lit("]}"))), List::<Bytes>::nil()))))
        }
        Err(v2) => {
            Ok(Err::<Adt26, Bytes>(f466(v2, v0)))
        }
    }
}

pub fn f538(v0: Adt23, v1: Str, v2: (Bytes, Bytes)) -> Option<Adt24> {
    f583(match v0 {
        Adt23::C0(h3629, _) => {
            h3629
        }
    }, f8(v1), {
        let (h3630, _) = v2;
        h3630
    }, {
        let (_, h3631) = v2;
        h3631
    })
}

pub fn f539(v0: Adt32, v1: Bytes, v2: Adt31, v3: bool) -> Adt32 {
    f584(f540(v0, Adt33::C0(v1, if v3 {
        let m3634 = match v2.clone() {
            Adt31::C0(h3633, _) => {
                h3633
            }
        };
        match m3634 {
            Adt22::C0(h3632, _) => {
                h3632
            }
        }
    } else {
        f580(Str::lit("Option"), {
            let m3637 = match v2.clone() {
                Adt31::C0(h3636, _) => {
                    h3636
                }
            };
            match m3637 {
                Adt22::C0(h3635, _) => {
                    h3635
                }
            }
        })
    }, {
        let m3640 = match v2.clone() {
            Adt31::C0(h3639, _) => {
                h3639
            }
        };
        match m3640 {
            Adt22::C0(_, h3638) => {
                h3638
            }
        }
    })), match v2 {
        Adt31::C0(_, h3641) => {
            h3641
        }
    })
}

pub fn f540(v0: Adt32, v1: Adt33) -> Adt32 {
    Adt32::C0(f585(match v0.clone() {
        Adt32::C0(h3642, _, _, _, _) => {
            h3642
        }
    }, List::cons(v1, List::<Adt33>::nil())), match v0.clone() {
        Adt32::C0(_, h3643, _, _, _) => {
            h3643
        }
    }, match v0.clone() {
        Adt32::C0(_, _, h3644, _, _) => {
            h3644
        }
    }, match v0.clone() {
        Adt32::C0(_, _, _, h3645, _) => {
            h3645
        }
    }, match v0 {
        Adt32::C0(_, _, _, _, h3646) => {
            h3646
        }
    })
}

pub fn f541() -> Bytes {
    f8(Str::lit("(List ((Bytes * Bytes) * Bytes))"))
}

pub fn f542(v0: Bytes, v1: List<Bytes>, v2: Bytes) -> Bytes {
    f200(f8(Str::lit("inductive{\u{a}  name: ")), v0, f200(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), f103(v1, f8(Str::lit(",\u{a}"))), f200(f8(Str::lit("\u{a}  ],\u{a}  mutual: ")), v2, f8(Str::lit("\u{a}}\u{a}")))))
}

pub fn f543(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a3647 = 0u64;
    let a3648 = v1;
    if if nat_lt(a3647, a3648) {
        f214({
            let a3651 = f212(v0, {
                let a3649 = v1;
                let a3650 = 1u64;
                nat_sub(a3649, a3650)
            })?;
            let a3652 = 10u64;
            nat_eq(a3651, a3652)
        })
    } else {
        false
    } {
        Ok(Some({
            let a3653 = v1;
            let a3654 = 1u64;
            nat_sub(a3653, a3654)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f544(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f291(f212(v0, v1)?) {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3655 = v1;
            let a3656 = 1u64;
            nat_add(a3655, a3656)?
        }))
    }
}

pub fn f545(v0: Bytes, v1: u64, v2: u64) -> R<Adt34> {
    let v3: (u64, u64) = {
        let (h3657, _) = f220(Fn5::F586(v2, v0.clone()), {
            let a3659 = {
                let a3658 = v0.clone();
                length_bytes(a3658)
            };
            let a3660 = 1u64;
            nat_add(a3659, a3660)?
        }, (v1, 0u64))?;
        h3657
    };
    let v4: u64 = f546(v0.clone(), {
        let (h3661, _) = v3;
        h3661
    }, v2)?;
    let a3662 = f212(v0, v4)?;
    let a3663 = 9u64;
    if if nat_eq(a3662, a3663) {
        let a3667 = {
            let a3665 = v4;
            let a3666 = {
                let (h3664, _) = v3;
                h3664
            };
            nat_sub(a3665, a3666)
        };
        let a3668 = v2;
        nat_lt(a3667, a3668)
    } else {
        false
    } {
        Ok(Adt34::C1(f111(Str::lit("a tab cannot indent a block scalar line (YAML 1.2.2 [63], [70])"), v4)?))
    } else {
        Ok(Adt34::C0({
            let (_, h3669) = v3;
            h3669
        }, {
            let (h3670, _) = v3;
            h3670
        }))
    }
}

pub fn f546(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h3671, _) = f168(Fn4::F587(v2, v0.clone(), v1), {
        let a3673 = {
            let a3672 = v0;
            length_bytes(a3672)
        };
        let a3674 = 1u64;
        nat_add(a3673, a3674)?
    }, v1)?;
    Ok(h3671)
}

pub fn f547(v0: Bytes, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = f546(v0.clone(), v1, v2)?;
    let a3677 = {
        let a3675 = v3;
        let a3676 = v1;
        nat_sub(a3675, a3676)
    };
    let a3678 = v2;
    if nat_eq(a3677, a3678) {
        Ok(f214({
            let a3679 = f212(v0, v3)?;
            let a3680 = 256u64;
            nat_eq(a3679, a3680)
        }))
    } else {
        Ok(false)
    }
}

pub fn f548(v0: Bytes, p3690: u64, v2: u64, v3: bool, p3691: u64, p3692: Bytes, p3693: u64) -> R<Adt35> {
    let m3689 = (p3690, (p3691, (p3692, p3693)));
    {
        let mut m3689 = m3689;
        loop {
            let step = {
                let (v1, (v4, (v5, v6))) = m3689;
                let m3681 = v4;
                if m3681 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<Adt35>>(Ok(Adt35::C1(f111(Str::lit("a block scalar has more lines than the input"), v1)?)))
                } else {
                    let v7 = m3681 - 1;
                    let v8: u64 = f546(v0.clone(), v1, v2)?;
                    let v9: bool = f214(f290(f212(v0.clone(), v8)?));
                    let v10: u64 = f393(v0.clone(), v8)?;
                    let v11: Bytes = f7(List::cons(v5, List::cons(f483(v6), List::cons(f302(v0.clone(), v8, v10), List::<Bytes>::nil()))));
                    let m3682 = f545(v0.clone(), f294(v0.clone(), v10)?, v2)?;
                    match m3682 {
                        Adt34::C0(v13, v14) => {
                            let v15: u64 = v14;
                            let v16: u64 = f546(v0.clone(), v15, v2)?;
                            if f547(v0.clone(), v15, v2)? {
                                Ok::<(u64, (u64, (Bytes, u64))), R<Adt35>>((v15, (v7, ({
                                    let a3685 = v11;
                                    let a3686 = if if if v3 {
                                        v9
                                    } else {
                                        false
                                    } {
                                        f214(f290(f212(v0.clone(), v16)?))
                                    } else {
                                        false
                                    } {
                                        let a3683 = v13;
                                        let a3684 = 0u64;
                                        if nat_eq(a3683, a3684) {
                                            Bytes::lit(&[0x20])
                                        } else {
                                            Bytes::lit(&[])
                                        }
                                    } else {
                                        Bytes::lit(&[0x0a])
                                    };
                                    append_bytes(a3685, a3686)
                                }, v13))))
                            } else {
                                Err::<(u64, (u64, (Bytes, u64))), R<Adt35>>(Ok(Adt35::C0(v11, v13, {
                                    let a3687 = f212(v0.clone(), v16)?;
                                    let a3688 = 256u64;
                                    if nat_eq(a3687, a3688) {
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
                Ok(next) => { m3689 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f549(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f295(v0.clone(), {
        let (h3694, _) = v1;
        h3694
    })?;
    let a3695 = f212(v0, v2)?;
    let a3696 = 10u64;
    if nat_eq(a3695, a3696) {
        Ok(Some(({
            let a3697 = v2;
            let a3698 = 1u64;
            nat_add(a3697, a3698)?
        }, f485({
            let (_, h3699) = v1;
            h3699
        }, {
            let a3701 = v2;
            let a3702 = {
                let (h3700, _) = v1;
                h3700
            };
            nat_sub(a3701, a3702)
        }))))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f550(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f588(v0, v1)
}

pub fn f551(v0: u64) -> R<Option<u64>> {
    let v1: u64 = f301(f8(Str::lit("abtnvfr")), v0)?;
    let a3703 = v1;
    let a3704 = 7u64;
    if nat_lt(a3703, a3704) {
        Ok(Some({
            let a3705 = v1;
            let a3706 = 7u64;
            nat_add(a3705, a3706)?
        }))
    } else {
        let a3707 = v0;
        let a3708 = 48u64;
        if nat_eq(a3707, a3708) {
            Ok(Some(0u64))
        } else {
            let a3709 = v0;
            let a3710 = 9u64;
            if nat_eq(a3709, a3710) {
                Ok(Some(9u64))
            } else {
                let a3711 = v0;
                let a3712 = 101u64;
                if nat_eq(a3711, a3712) {
                    Ok(Some(27u64))
                } else {
                    let a3713 = v0;
                    let a3714 = 78u64;
                    if nat_eq(a3713, a3714) {
                        Ok(Some(133u64))
                    } else {
                        let a3715 = v0;
                        let a3716 = 95u64;
                        if nat_eq(a3715, a3716) {
                            Ok(Some(160u64))
                        } else {
                            let a3717 = v0;
                            let a3718 = 76u64;
                            if nat_eq(a3717, a3718) {
                                Ok(Some(8232u64))
                            } else {
                                let a3719 = v0;
                                let a3720 = 80u64;
                                if nat_eq(a3719, a3720) {
                                    Ok(Some(8233u64))
                                } else {
                                    let a3721 = f301(f8(Str::lit(" \"/\\")), v0)?;
                                    let a3722 = 4u64;
                                    if nat_lt(a3721, a3722) {
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

pub fn f552(v0: u64) -> R<Bytes> {
    f111(Str::lit("an implicit key must be on one line (YAML 1.2.2 [154])"), v0)
}

pub fn f553(v0: Bytes, p3737: u64, v2: u64, p3738: u64, p3739: u64) -> R<Adt34> {
    let m3736 = (p3737, (p3738, p3739));
    {
        let mut m3736 = m3736;
        loop {
            let step = {
                let (v1, (v3, v4)) = m3736;
                let m3723 = v3;
                if m3723 == 0 {
                    Err::<(u64, (u64, u64)), R<Adt34>>(Ok(Adt34::C1(f111(Str::lit("the input ends inside a scalar"), v1)?)))
                } else {
                    let v5 = m3723 - 1;
                    let v6: u64 = {
                        let a3724 = v1;
                        let a3725 = 1u64;
                        nat_add(a3724, a3725)?
                    };
                    let v7: u64 = f295(v0.clone(), v6)?;
                    let v8: u64 = f293(v0.clone(), v7)?;
                    let v9: u64 = f212(v0.clone(), v8)?;
                    if f210(v0.clone(), v6)? {
                        Err::<(u64, (u64, u64)), R<Adt34>>(Ok(Adt34::C1(f111(Str::lit("a document marker cannot appear inside a scalar (YAML 1.2.2 [206])"), v6)?)))
                    } else {
                        let a3726 = v9;
                        let a3727 = 10u64;
                        if nat_eq(a3726, a3727) {
                            Ok::<(u64, (u64, u64)), R<Adt34>>((v8, (v5, {
                                let a3728 = v4;
                                let a3729 = 1u64;
                                nat_add(a3728, a3729)?
                            })))
                        } else {
                            let a3730 = v9;
                            let a3731 = 256u64;
                            if nat_eq(a3730, a3731) {
                                Err::<(u64, (u64, u64)), R<Adt34>>(Ok(Adt34::C1(f111(Str::lit("the input ends inside a quoted scalar (YAML 1.2.2 [109], [120])"), v8)?)))
                            } else {
                                let a3734 = {
                                    let a3732 = v7;
                                    let a3733 = v6;
                                    nat_sub(a3732, a3733)
                                };
                                let a3735 = v2;
                                if nat_lt(a3734, a3735) {
                                    Err::<(u64, (u64, u64)), R<Adt34>>(Ok(Adt34::C1(f111(Str::lit("a continuation line of a scalar is indented less than its node (YAML 1.2.2 [69])"), v7)?)))
                                } else {
                                    Err::<(u64, (u64, u64)), R<Adt34>>(Ok(Adt34::C0(v4, v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3736 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f554(v0: Bytes, p3749: u64, p3750: u64, p3751: u64) -> R<Option<u64>> {
    let m3748 = (p3749, (p3750, p3751));
    {
        let mut m3748 = m3748;
        loop {
            let step = {
                let (v1, (v2, v3)) = m3748;
                let m3740 = v2;
                if m3740 == 0 {
                    Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3740 - 1;
                    let m3741 = f589(f212(v0.clone(), v1)?);
                    match m3741 {
                        None => {
                            Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(None::<u64>))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, u64)), R<Option<u64>>>(({
                                let a3742 = v1;
                                let a3743 = 1u64;
                                nat_add(a3742, a3743)?
                            }, (v4, {
                                let a3746 = {
                                    let a3744 = v3;
                                    let a3745 = 16u64;
                                    nat_mul(a3744, a3745)?
                                };
                                let a3747 = v5;
                                nat_add(a3746, a3747)?
                            })))
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

pub fn f555(v0: Bytes, v1: u64) -> R<u64> {
    let (h3752, _) = f168(Fn4::F590(v0.clone()), {
        let a3754 = {
            let a3753 = v0;
            length_bytes(a3753)
        };
        let a3755 = 1u64;
        nat_add(a3754, a3755)?
    }, v1)?;
    Ok(h3752)
}

pub fn f556(v0: Bytes, v1: u64) -> R<u64> {
    let (h3756, _) = f168(Fn4::F591(v0.clone()), {
        let a3758 = {
            let a3757 = v0;
            length_bytes(a3757)
        };
        let a3759 = 1u64;
        nat_add(a3758, a3759)?
    }, v1)?;
    Ok(h3756)
}

pub fn f557(v0: Fn9, p3764: u64, p3765: (u64, bool)) -> R<((u64, bool), bool)> {
    let m3763 = (p3764, p3765);
    {
        let mut m3763 = m3763;
        loop {
            let step = {
                let (v1, v2) = m3763;
                let m3760 = v1;
                if m3760 == 0 {
                    Err::<(u64, (u64, bool)), R<((u64, bool), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3760 - 1;
                    let m3762 = {
                        let c3761: Fn9 = v0.clone();
                        c3761.apply(v2)?
                    };
                    match m3762 {
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
                Ok(next) => { m3763 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f558(v0: u64, v1: Bytes, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    f592(v1, v0, v2)
}

pub fn f559(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = {
        let (h3766, _) = v2;
        h3766
    };
    let v4: u64 = f212(v0.clone(), v3)?;
    if f291(v4) {
        Ok(None::<(u64, u64)>)
    } else {
        if f290(v4) {
            Ok(Some(({
                let a3767 = v3;
                let a3768 = 1u64;
                nat_add(a3767, a3768)?
            }, {
                let (_, h3769) = v2;
                h3769
            })))
        } else {
            let a3770 = v4;
            let a3771 = 58u64;
            if if nat_eq(a3770, a3771) {
                f214(f395(f212(v0.clone(), {
                    let a3772 = v3;
                    let a3773 = 1u64;
                    nat_add(a3772, a3773)?
                })?, v1))
            } else {
                false
            } {
                Ok(None::<(u64, u64)>)
            } else {
                let a3774 = v4;
                let a3775 = 35u64;
                if if nat_eq(a3774, a3775) {
                    f290(f212(v0, {
                        let a3776 = v3;
                        let a3777 = 1u64;
                        nat_sub(a3776, a3777)
                    })?)
                } else {
                    false
                } {
                    Ok(None::<(u64, u64)>)
                } else {
                    if if v1 {
                        f478(v4)
                    } else {
                        false
                    } {
                        Ok(None::<(u64, u64)>)
                    } else {
                        Ok(Some(({
                            let a3778 = v3;
                            let a3779 = 1u64;
                            nat_add(a3778, a3779)?
                        }, {
                            let a3780 = v3;
                            let a3781 = 1u64;
                            nat_add(a3780, a3781)?
                        })))
                    }
                }
            }
        }
    }
}

pub fn f560(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f593(v0, v1)
}

pub fn f561(v0: Bytes, v1: u64, v2: u64) -> R<(u64, u64)> {
    let (h3782, _) = f220(Fn5::F594(v1, v0.clone()), {
        let a3784 = {
            let a3783 = v0;
            length_bytes(a3783)
        };
        let a3785 = 1u64;
        nat_add(a3784, a3785)?
    }, (v2, 0u64))?;
    Ok(h3782)
}

pub fn f562(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3830 = {
            let a3806 = {
                let a3794 = {
                    let a3788 = {
                        let a3786 = v1;
                        let a3787 = 128u8;
                        fixed_u8::bit_and(a3786, a3787)
                    };
                    let a3789 = 128u8;
                    if equal(a3788, a3789) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3795 = {
                    let a3792 = {
                        let a3790 = v1;
                        let a3791 = 64u8;
                        fixed_u8::bit_and(a3790, a3791)
                    };
                    let a3793 = 64u8;
                    if equal(a3792, a3793) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3794, a3795)?
            };
            let a3807 = {
                let a3804 = {
                    let a3798 = {
                        let a3796 = v1;
                        let a3797 = 32u8;
                        fixed_u8::bit_and(a3796, a3797)
                    };
                    let a3799 = 32u8;
                    if equal(a3798, a3799) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3805 = {
                    let a3802 = {
                        let a3800 = v1;
                        let a3801 = 16u8;
                        fixed_u8::bit_and(a3800, a3801)
                    };
                    let a3803 = 16u8;
                    if equal(a3802, a3803) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3804, a3805)?
            };
            nat_add(a3806, a3807)?
        };
        let a3831 = {
            let a3828 = {
                let a3816 = {
                    let a3810 = {
                        let a3808 = v1;
                        let a3809 = 8u8;
                        fixed_u8::bit_and(a3808, a3809)
                    };
                    let a3811 = 8u8;
                    if equal(a3810, a3811) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3817 = {
                    let a3814 = {
                        let a3812 = v1;
                        let a3813 = 4u8;
                        fixed_u8::bit_and(a3812, a3813)
                    };
                    let a3815 = 4u8;
                    if equal(a3814, a3815) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3816, a3817)?
            };
            let a3829 = {
                let a3826 = {
                    let a3820 = {
                        let a3818 = v1;
                        let a3819 = 2u8;
                        fixed_u8::bit_and(a3818, a3819)
                    };
                    let a3821 = 2u8;
                    if equal(a3820, a3821) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3827 = {
                    let a3824 = {
                        let a3822 = v1;
                        let a3823 = 1u8;
                        fixed_u8::bit_and(a3822, a3823)
                    };
                    let a3825 = 1u8;
                    if equal(a3824, a3825) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3826, a3827)?
            };
            nat_add(a3828, a3829)?
        };
        nat_add(a3830, a3831)?
    };
    let a3838 = f164(f595(), {
        let a3832 = v2;
        let a3833 = 16u64;
        let a3834 = 0u64;
        nat_quot(a3832, a3833, a3834)
    });
    let a3839 = f164(f595(), {
        let a3835 = v2;
        let a3836 = 16u64;
        let a3837 = 0u64;
        nat_rem(a3835, a3836, a3837)
    });
    Ok(append_bytes(a3838, a3839))
}

pub fn f563(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f596(Fn12::F597, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f564(v0: Fn10, p3843: List<List<(Bytes, Bytes)>>, p3844: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m3842 = (p3843, p3844);
    {
        let mut m3842 = m3842;
        loop {
            let step = {
                let (v1, v2) = m3842;
                let m3840 = v2;
                match m3840.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c3841: Fn10 = v0.clone();
                            c3841.apply(v1, v3)
                        }, v4))
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

pub fn f565(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f566(v0: Fn11, p3849: u64, p3850: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m3848 = (p3849, p3850);
    {
        let mut m3848 = m3848;
        loop {
            let step = {
                let (v1, v2) = m3848;
                let m3845 = v1;
                if m3845 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3845 - 1;
                    let m3847 = {
                        let c3846: Fn11 = v0.clone();
                        c3846.apply(v2.clone())?
                    };
                    match m3847 {
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
                Ok(next) => { m3848 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f567(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f598(v0)
}

pub fn f568(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h3851, _) = v1;
        h3851
    };
    let m3853 = {
        let (_, h3852) = v1;
        h3852
    };
    match m3853 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m3856 = {
                let a3854 = v0.clone();
                let a3855 = v2;
                index_bytes(a3854, a3855)
            };
            match m3856 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m3861 = {
                        let a3859 = v0;
                        let a3860 = {
                            let a3857 = v2;
                            let a3858 = 1u64;
                            nat_add(a3857, a3858)?
                        };
                        index_bytes(a3859, a3860)
                    };
                    match m3861 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f599(v4)? {
                                f599(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a3862 = v2;
                                    let a3863 = 2u64;
                                    nat_add(a3862, a3863)?
                                }, Some({
                                    let a3868 = v3;
                                    let a3869 = f164(f600(), {
                                        let a3866 = {
                                            let a3864 = f601(v4)?;
                                            let a3865 = 16u64;
                                            nat_mul(a3864, a3865)?
                                        };
                                        let a3867 = f601(v5)?;
                                        nat_add(a3866, a3867)?
                                    });
                                    append_bytes(a3868, a3869)
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

pub fn f569(v0: u64, v1: u64) -> bool {
    let a3870 = v0;
    let a3871 = 0u64;
    if nat_eq(a3870, a3871) {
        f418(v1)
    } else {
        let a3872 = v0;
        let a3873 = 1u64;
        if nat_eq(a3872, a3873) {
            f417(v1)
        } else {
            let a3874 = v0;
            let a3875 = 2u64;
            if nat_eq(a3874, a3875) {
                f602(v1)
            } else {
                f603(v1)
            }
        }
    }
}

pub fn f570(v0: Bytes, v1: u64) -> R<u64> {
    let a3877 = v1;
    let a3878 = {
        let a3876 = v0.clone();
        length_bytes(a3876)
    };
    if nat_lt(a3877, a3878) {
        let a3879 = f115(v0, v1)?;
        let a3880 = 1u64;
        nat_add(a3879, a3880)
    } else {
        Ok(0u64)
    }
}

pub fn f571(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f139(f85(f139(f368(v2), f369(v2)), f85({
        let a3881 = v2;
        let a3882 = 117u64;
        !nat_eq(a3881, a3882)
    }, {
        let a3883 = v2;
        let a3884 = 85u64;
        !nat_eq(a3883, a3884)
    })), f85(f370(v2), {
        let a3885 = 0u64;
        let a3886 = v1;
        nat_lt(a3885, a3886)
    })) {
        f43(v0, v1, 1u64)
    } else {
        let a3887 = v2;
        let a3888 = 65536u64;
        if nat_lt(a3887, a3888) {
            let a3889 = f8(Str::lit("u"));
            let a3890 = f436(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a3889, a3890)
        } else {
            let a3891 = f8(Str::lit("U"));
            let a3892 = f436(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a3891, a3892)
        }
    }
}

pub fn f572(v0: u64) -> Bytes {
    let a3893 = f8(Str::lit("u"));
    let a3894 = f436(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a3893, a3894)
}

pub fn f573(v0: u64, v1: bool) -> u64 {
    let a3895 = v0;
    let a3896 = 48u64;
    if nat_lt(a3895, a3896) {
        16u64
    } else {
        let a3897 = v0;
        let a3898 = 57u64;
        if nat_le(a3897, a3898) {
            let a3899 = v0;
            let a3900 = 48u64;
            nat_sub(a3899, a3900)
        } else {
            if v1 {
                let a3901 = v0;
                let a3902 = 65u64;
                if nat_lt(a3901, a3902) {
                    16u64
                } else {
                    let a3903 = v0;
                    let a3904 = 70u64;
                    if nat_le(a3903, a3904) {
                        let a3905 = v0;
                        let a3906 = 55u64;
                        nat_sub(a3905, a3906)
                    } else {
                        let a3907 = v0;
                        let a3908 = 97u64;
                        if nat_lt(a3907, a3908) {
                            16u64
                        } else {
                            let a3909 = v0;
                            let a3910 = 102u64;
                            if nat_le(a3909, a3910) {
                                let a3911 = v0;
                                let a3912 = 87u64;
                                nat_sub(a3911, a3912)
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

pub fn f574() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f575() -> Bytes {
    f8(Str::lit("\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{0}]\u{0}\u{0}]\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f576(v0: Adt24) -> R<Result<Option<(Adt25, (Bytes, Bytes))>, Bytes>> {
    let m3914 = f464(match v0 {
        Adt24::C0(_, _, _, h3913) => {
            h3913
        }
    })?;
    match m3914 {
        Ok(v2) => {
            let m3915 = f465(v2, List::cons(f8(Str::lit("simpleContent")), List::cons(f8(Str::lit("complexContent")), List::<Bytes>::nil())));
            match m3915 {
                None => {
                    Ok(Ok::<Option<(Adt25, (Bytes, Bytes))>, Bytes>(None::<(Adt25, (Bytes, Bytes))>))
                }
                Some(v3) => {
                    let m3916 = f464(v3)?;
                    match m3916 {
                        Ok(v5) => {
                            let m3917 = f465(v5, List::cons(f8(Str::lit("extension")), List::cons(f8(Str::lit("restriction")), List::<Bytes>::nil())));
                            match m3917 {
                                None => {
                                    Ok(Ok::<Option<(Adt25, (Bytes, Bytes))>, Bytes>(None::<(Adt25, (Bytes, Bytes))>))
                                }
                                Some(v6) => {
                                    let m3918 = f284(v6.clone(), Str::lit("base"));
                                    match m3918 {
                                        None => {
                                            Ok(Ok::<Option<(Adt25, (Bytes, Bytes))>, Bytes>(None::<(Adt25, (Bytes, Bytes))>))
                                        }
                                        Some(v7) => {
                                            let m3920 = f119(match v6.clone() {
                                                Adt25::C0(_, _, _, _, h3919) => {
                                                    h3919
                                                }
                                            }, v7)?;
                                            match m3920 {
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

pub fn f577(v0: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f604(f234(v0.clone())?, v0, List::<Bytes>::nil())
}

pub fn f578(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m3921 = v0;
    match m3921.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m3922 = v1;
            match m3922.uncons() {
                None => {
                    List::<Bytes>::nil()
                }
                Some((v4, v5)) => {
                    List::cons(f200(f8(Str::lit("    {name: ")), v2, f200(f8(Str::lit(", fields: [")), v4, f8(Str::lit("]}")))), f578(v3, v5))
                }
            }
        }
    }
}

pub fn f579(v0: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: Adt25, v5: Adt32) -> R<Result<Adt32, Bytes>> {
    let m3923 = v0;
    if m3923 == 0 {
        Ok(Err::<Adt32, Bytes>(f466(f8(Str::lit("the type derivations nest deeper than the schema documents have bytes, so the types derive from each other in a cycle (XSD 1.1 Part 1, section 3.4.6.1)")), v2)))
    } else {
        let v6 = m3923 - 1;
        if f525(v4.clone(), Str::lit("mixed"), Str::lit("true")) {
            Ok(Err::<Adt32, Bytes>(f466(f8(Str::lit("a mixed content type interleaves character data with its elements, which a LexLean field list does not keep in order, so it is not translated (XSD 1.1 Part 1, section 3.4.2.3)")), v2)))
        } else {
            let m3924 = f464(v4)?;
            match m3924 {
                Ok(v8) => {
                    let m3925 = f465(v8.clone(), List::cons(f8(Str::lit("openContent")), List::cons(f8(Str::lit("assert")), List::<Bytes>::nil())));
                    match m3925 {
                        None => {
                            let m3927 = f465(v8.clone(), List::cons(f8(Str::lit("simpleContent")), List::cons(f8(Str::lit("complexContent")), List::<Bytes>::nil())));
                            match m3927 {
                                None => {
                                    let m3928 = f533(v6, v1.clone(), v2.clone(), v3.clone(), true, v8.clone(), v5)?;
                                    match m3928 {
                                        Ok(v11) => {
                                            f468(v6, v1, v2, v3, v8, v11)
                                        }
                                        Err(v10) => {
                                            Ok(Err::<Adt32, Bytes>(v10))
                                        }
                                    }
                                }
                                Some(v12) => {
                                    if f525(v12.clone(), Str::lit("mixed"), Str::lit("true")) {
                                        Ok(Err::<Adt32, Bytes>(f466(f8(Str::lit("a mixed content type interleaves character data with its elements, which a LexLean field list does not keep in order, so it is not translated (XSD 1.1 Part 1, section 3.4.2.3)")), v2)))
                                    } else {
                                        let m3929 = f464(v12.clone())?;
                                        match m3929 {
                                            Ok(v14) => {
                                                let m3930 = f465(v14, List::cons(f8(Str::lit("extension")), List::cons(f8(Str::lit("restriction")), List::<Bytes>::nil())));
                                                match m3930 {
                                                    None => {
                                                        Ok(Err::<Adt32, Bytes>(f466(f8(Str::lit("simpleContent and complexContent must have an extension or a restriction child (XSD 1.1 Part 1, sections 3.4.2.2 and 3.4.2.3)")), v2)))
                                                    }
                                                    Some(v15) => {
                                                        let m3931 = f284(v15.clone(), Str::lit("base"));
                                                        match m3931 {
                                                            None => {
                                                                Ok(Err::<Adt32, Bytes>(f466(f8(Str::lit("an extension or restriction must name its base type (XSD 1.1 Part 1, section 3.4.3)")), v2)))
                                                            }
                                                            Some(v16) => {
                                                                let m3933 = f119(match v15.clone() {
                                                                    Adt25::C0(_, _, _, _, h3932) => {
                                                                        h3932
                                                                    }
                                                                }, v16.clone())?;
                                                                match m3933 {
                                                                    Ok(v18) => {
                                                                        let m3934 = f464(v15.clone())?;
                                                                        match m3934 {
                                                                            Ok(v20) => {
                                                                                let v21: bool = if f46({
                                                                                    let (h3935, _) = v18.clone();
                                                                                    h3935
                                                                                }, f188()) {
                                                                                    f46({
                                                                                        let (_, h3936) = v18.clone();
                                                                                        h3936
                                                                                    }, f8(Str::lit("anyType")))
                                                                                } else {
                                                                                    false
                                                                                };
                                                                                if if f283(v15.clone(), Str::lit("restriction")) {
                                                                                    f282(v21)
                                                                                } else {
                                                                                    false
                                                                                } {
                                                                                    Ok(Err::<Adt32, Bytes>(f466(f200(f8(Str::lit("a restriction of the complex type ")), v16, f8(Str::lit(" is not translated, because the attribute uses it inherits and the content it narrows are not read yet (XSD 1.1 Part 1, sections 3.4.2.2 and 3.4.2.3)"))), v2)))
                                                                                } else {
                                                                                    let m3951 = if if v21 {
                                                                                        true
                                                                                    } else {
                                                                                        f46(match v12 {
                                                                                            Adt25::C0(_, h3937, _, _, _) => {
                                                                                                h3937
                                                                                            }
                                                                                        }, f8(Str::lit("complexContent")))
                                                                                    } {
                                                                                        if v21 {
                                                                                            Ok::<Adt32, Bytes>(v5)
                                                                                        } else {
                                                                                            let m3938 = f538(v1.clone(), Str::lit("complexType"), v18.clone());
                                                                                            match m3938 {
                                                                                                None => {
                                                                                                    Err::<Adt32, Bytes>(f466(f200(f8(Str::lit("the base type ")), v16, f8(Str::lit(" of a complex content extension is not a complex type of this schema set (XSD 1.1 Part 1, section 3.4.2.3)"))), v2.clone()))
                                                                                                }
                                                                                                Some(v22) => {
                                                                                                    let m3942 = f579(v6, v1.clone(), f191({
                                                                                                        let (_, h3939) = v18;
                                                                                                        h3939
                                                                                                    })?, match v22.clone() {
                                                                                                        Adt24::C0(_, h3940, _, _) => {
                                                                                                            h3940
                                                                                                        }
                                                                                                    }, match v22 {
                                                                                                        Adt24::C0(_, _, _, h3941) => {
                                                                                                            h3941
                                                                                                        }
                                                                                                    }, v5)?;
                                                                                                    match m3942 {
                                                                                                        Ok(v24) => {
                                                                                                            Ok::<Adt32, Bytes>(f605(v24))
                                                                                                        }
                                                                                                        Err(v23) => {
                                                                                                            Err::<Adt32, Bytes>(v23)
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        let m3943 = f538(v1.clone(), Str::lit("complexType"), v18.clone());
                                                                                        match m3943 {
                                                                                            None => {
                                                                                                let m3948 = f531(v1.clone(), v15, v16)?;
                                                                                                match m3948 {
                                                                                                    Ok(v29) => {
                                                                                                        Ok::<Adt32, Bytes>(f540(v5, Adt33::C0(f8(Str::lit("value")), match v29.clone() {
                                                                                                            Adt22::C0(h3949, _) => {
                                                                                                                h3949
                                                                                                            }
                                                                                                        }, match v29 {
                                                                                                            Adt22::C0(_, h3950) => {
                                                                                                                h3950
                                                                                                            }
                                                                                                        })))
                                                                                                    }
                                                                                                    Err(v28) => {
                                                                                                        Err::<Adt32, Bytes>(f466(v28, v2.clone()))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            Some(v25) => {
                                                                                                let m3947 = f579(v6, v1.clone(), f191({
                                                                                                    let (_, h3944) = v18;
                                                                                                    h3944
                                                                                                })?, match v25.clone() {
                                                                                                    Adt24::C0(_, h3945, _, _) => {
                                                                                                        h3945
                                                                                                    }
                                                                                                }, match v25 {
                                                                                                    Adt24::C0(_, _, _, h3946) => {
                                                                                                        h3946
                                                                                                    }
                                                                                                }, v5)?;
                                                                                                match m3947 {
                                                                                                    Ok(v27) => {
                                                                                                        Ok::<Adt32, Bytes>(f605(v27))
                                                                                                    }
                                                                                                    Err(v26) => {
                                                                                                        Err::<Adt32, Bytes>(v26)
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    };
                                                                                    match m3951 {
                                                                                        Ok(v31) => {
                                                                                            let m3952 = f533(v6, v1.clone(), v2.clone(), v3.clone(), true, v20.clone(), v31)?;
                                                                                            match m3952 {
                                                                                                Ok(v33) => {
                                                                                                    f468(v6, v1, v2, v3, v20, v33)
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
                                                                        Ok(Err::<Adt32, Bytes>(f466(v17, v2)))
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
                            Ok(Err::<Adt32, Bytes>(f466(f200(f8(Str::lit("xs:")), match v9 {
                                Adt25::C0(_, h3926, _, _, _) => {
                                    h3926
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

pub fn f580(v0: Str, v1: Bytes) -> Bytes {
    f200(f8(Str::lit("(")), f8(v0), f200(f8(Str::lit(" ")), v1, f8(Str::lit(")"))))
}

pub fn f581(v0: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: bool, v5: Adt25, v6: Adt32) -> R<Result<Adt32, Bytes>> {
    let m3953 = v0;
    if m3953 == 0 {
        Ok(Err::<Adt32, Bytes>(f466(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v7 = m3953 - 1;
        if f283(v5.clone(), Str::lit("element")) {
            let m3954 = f606(v7, v1, v2, v3, v5.clone())?;
            match m3954 {
                Ok(v9) => {
                    let m3960 = f607(v5, {
                        let m3959 = {
                            let m3958 = match v9.clone() {
                                Adt36::C0(_, h3957) => {
                                    h3957
                                }
                            };
                            match m3958 {
                                Adt31::C0(h3956, _) => {
                                    h3956
                                }
                            }
                        };
                        match m3959 {
                            Adt22::C0(h3955, _) => {
                                h3955
                            }
                        }
                    });
                    match m3960 {
                        None => {
                            Ok(Ok::<Adt32, Bytes>(v6))
                        }
                        Some(v10) => {
                            Ok(Ok::<Adt32, Bytes>(f584(f540(v6, Adt33::C0(match v9.clone() {
                                Adt36::C0(h3961, _) => {
                                    h3961
                                }
                            }, v10, {
                                let m3966 = {
                                    let m3965 = match v9.clone() {
                                        Adt36::C0(_, h3964) => {
                                            h3964
                                        }
                                    };
                                    match m3965 {
                                        Adt31::C0(h3963, _) => {
                                            h3963
                                        }
                                    }
                                };
                                match m3966 {
                                    Adt22::C0(_, h3962) => {
                                        h3962
                                    }
                                }
                            })), {
                                let m3969 = match v9 {
                                    Adt36::C0(_, h3968) => {
                                        h3968
                                    }
                                };
                                match m3969 {
                                    Adt31::C0(_, h3967) => {
                                        h3967
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
            if f283(v5.clone(), Str::lit("any")) {
                let m3970 = f607(v5, f378());
                match m3970 {
                    None => {
                        Ok(Ok::<Adt32, Bytes>(v6))
                    }
                    Some(v11) => {
                        Ok(Ok::<Adt32, Bytes>(f540(v6, Adt33::C0(f8(Str::lit("any")), v11, List::cons(f378(), List::<Bytes>::nil())))))
                    }
                }
            } else {
                if f283(v5.clone(), Str::lit("group")) {
                    let m3971 = f284(v5.clone(), Str::lit("ref"));
                    match m3971 {
                        None => {
                            Ok(Err::<Adt32, Bytes>(f466(f8(Str::lit("a model group reference must have a ref (XSD 1.1 Part 1, section 3.7.2)")), v2)))
                        }
                        Some(v12) => {
                            let m3973 = f119(match v5.clone() {
                                Adt25::C0(_, _, _, _, h3972) => {
                                    h3972
                                }
                            }, v12.clone())?;
                            match m3973 {
                                Ok(v14) => {
                                    let m3974 = f538(v1, Str::lit("group"), v14.clone());
                                    match m3974 {
                                        None => {
                                            Ok(Err::<Adt32, Bytes>(f466(f200(f8(Str::lit("the model group ")), v12, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                        }
                                        Some(_) => {
                                            let v16: Bytes = f191({
                                                let (_, h3975) = v14.clone();
                                                h3975
                                            })?;
                                            let m3976 = f607(v5, v16.clone());
                                            match m3976 {
                                                None => {
                                                    Ok(Ok::<Adt32, Bytes>(v6))
                                                }
                                                Some(v17) => {
                                                    Ok(Ok::<Adt32, Bytes>(f540(v6, Adt33::C0({
                                                        let (_, h3977) = v14;
                                                        h3977
                                                    }, v17, List::cons(v16, List::<Bytes>::nil())))))
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(v13) => {
                                    Ok(Err::<Adt32, Bytes>(f466(v13, v2)))
                                }
                            }
                        }
                    }
                } else {
                    if if f283(v5.clone(), Str::lit("sequence")) {
                        true
                    } else {
                        if f283(v5.clone(), Str::lit("choice")) {
                            true
                        } else {
                            f283(v5.clone(), Str::lit("all"))
                        }
                    } {
                        if if v4 {
                            if f608(v5.clone()) {
                                f282(f283(v5.clone(), Str::lit("choice")))
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m3978 = f464(v5)?;
                            match m3978 {
                                Ok(v19) => {
                                    f533(v7, v1, v2, v3, true, v19, v6)
                                }
                                Err(v18) => {
                                    Ok(Err::<Adt32, Bytes>(v18))
                                }
                            }
                        } else {
                            let v20: Bytes = match v5.clone() {
                                Adt25::C0(_, h3979, _, _, _) => {
                                    h3979
                                }
                            };
                            let v21: u64 = f609(v6.clone(), v20.clone())?;
                            let v22: Bytes = {
                                let a3980 = v21;
                                let a3981 = 1u64;
                                if nat_eq(a3980, a3981) {
                                    Bytes::lit(&[])
                                } else {
                                    f65(v21)?
                                }
                            };
                            let v23: Bytes = f200(v2, f191(v20.clone())?, v22.clone());
                            let m3982 = f467(v7, v1, v23.clone(), v3, v5.clone())?;
                            match m3982 {
                                Ok(v25) => {
                                    let m3983 = f607(v5, v23.clone());
                                    match m3983 {
                                        None => {
                                            Ok(Ok::<Adt32, Bytes>(f610(v6, v20)?))
                                        }
                                        Some(v26) => {
                                            Ok(Ok::<Adt32, Bytes>(f584(f540(f610(v6, v20.clone())?, Adt33::C0({
                                                let a3984 = v20;
                                                let a3985 = v22;
                                                append_bytes(a3984, a3985)
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

pub fn f582(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m3986 = v0;
    match m3986.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m3987 = v1;
            match m3987.uncons() {
                None => {
                    List::<Bytes>::nil()
                }
                Some((v4, v5)) => {
                    List::cons(f200(f8(Str::lit("    {name: ")), v2, f200(f8(Str::lit(", type: ")), v4, f8(Str::lit("}")))), f582(v3, v5))
                }
            }
        }
    }
}

pub fn f583(p3993: List<Adt24>, v1: Bytes, v2: Bytes, v3: Bytes) -> Option<Adt24> {
    let m3992 = p3993;
    {
        let mut m3992 = m3992;
        loop {
            let step = {
                let v0 = m3992;
                let m3988 = v0;
                match m3988.uncons() {
                    None => {
                        Err::<List<Adt24>, Option<Adt24>>(None::<Adt24>)
                    }
                    Some((v4, v5)) => {
                        if if f46(match v4.clone() {
                            Adt24::C0(h3989, _, _, _) => {
                                h3989
                            }
                        }, v1.clone()) {
                            if f46(match v4.clone() {
                                Adt24::C0(_, h3990, _, _) => {
                                    h3990
                                }
                            }, v2.clone()) {
                                f46(match v4.clone() {
                                    Adt24::C0(_, _, h3991, _) => {
                                        h3991
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
                Ok(next) => { m3992 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f584(v0: Adt32, v1: List<Adt26>) -> Adt32 {
    Adt32::C0(match v0.clone() {
        Adt32::C0(h3994, _, _, _, _) => {
            h3994
        }
    }, f375(match v0.clone() {
        Adt32::C0(_, h3995, _, _, _) => {
            h3995
        }
    }, v1), match v0.clone() {
        Adt32::C0(_, _, h3996, _, _) => {
            h3996
        }
    }, match v0.clone() {
        Adt32::C0(_, _, _, h3997, _) => {
            h3997
        }
    }, match v0 {
        Adt32::C0(_, _, _, _, h3998) => {
            h3998
        }
    })
}

pub fn f585(v0: List<Adt33>, v1: List<Adt33>) -> List<Adt33> {
    let m3999 = v0;
    match m3999.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f585(v3, v1))
        }
    }
}

pub fn f586(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f611(v1, v0, v2)
}

pub fn f587(v0: u64, v1: Bytes, v2: u64, v3: u64) -> R<Option<u64>> {
    f612(v1, v2, v0, v3)
}

pub fn f588(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let a4002 = {
        let (h4000, _) = v1;
        h4000
    };
    let a4003 = {
        let a4001 = v0.clone();
        length_bytes(a4001)
    };
    if nat_lt(a4002, a4003) {
        Ok(Some((f294(v0, {
            let (h4004, _) = v1;
            h4004
        })?, {
            let a4006 = {
                let (_, h4005) = v1;
                h4005
            };
            let a4007 = 1u64;
            nat_add(a4006, a4007)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f589(v0: u64) -> Option<u64> {
    let a4008 = 48u64;
    let a4009 = v0;
    if if nat_le(a4008, a4009) {
        let a4010 = v0;
        let a4011 = 57u64;
        nat_le(a4010, a4011)
    } else {
        false
    } {
        Some({
            let a4012 = v0;
            let a4013 = 48u64;
            nat_sub(a4012, a4013)
        })
    } else {
        let a4014 = 97u64;
        let a4015 = v0;
        if if nat_le(a4014, a4015) {
            let a4016 = v0;
            let a4017 = 102u64;
            nat_le(a4016, a4017)
        } else {
            false
        } {
            Some({
                let a4018 = v0;
                let a4019 = 87u64;
                nat_sub(a4018, a4019)
            })
        } else {
            let a4020 = 65u64;
            let a4021 = v0;
            if if nat_le(a4020, a4021) {
                let a4022 = v0;
                let a4023 = 70u64;
                nat_le(a4022, a4023)
            } else {
                false
            } {
                Some({
                    let a4024 = v0;
                    let a4025 = 55u64;
                    nat_sub(a4024, a4025)
                })
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f590(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f613(v0, v1)
}

pub fn f591(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f614(v0, v1)
}

pub fn f592(v0: Bytes, v1: u64, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    let v3: u64 = {
        let (h4026, _) = v2;
        h4026
    };
    let v4: u64 = f212(v0.clone(), v3)?;
    let (_, h4027) = v2;
    if h4027 {
        Ok(None::<(u64, bool)>)
    } else {
        if f290(v4) {
            Ok(Some(({
                let a4028 = v3;
                let a4029 = 1u64;
                nat_add(a4028, a4029)?
            }, false)))
        } else {
            let a4030 = v4;
            let a4031 = 35u64;
            if if nat_eq(a4030, a4031) {
                let a4032 = v3;
                let a4033 = 0u64;
                if if nat_eq(a4032, a4033) {
                    true
                } else {
                    f290(f212(v0.clone(), {
                        let a4034 = v3;
                        let a4035 = 1u64;
                        nat_sub(a4034, a4035)
                    })?)
                } {
                    true
                } else {
                    let a4038 = f212(v0.clone(), {
                        let a4036 = v3;
                        let a4037 = 1u64;
                        nat_sub(a4036, a4037)
                    })?;
                    let a4039 = 10u64;
                    nat_eq(a4038, a4039)
                }
            } else {
                false
            } {
                Ok(Some((f393(v0, v3)?, false)))
            } else {
                let a4040 = v4;
                let a4041 = 10u64;
                if nat_eq(a4040, a4041) {
                    let v5: u64 = {
                        let a4042 = v3;
                        let a4043 = 1u64;
                        nat_add(a4042, a4043)?
                    };
                    let v6: u64 = f295(v0.clone(), v5)?;
                    let v7: u64 = f293(v0.clone(), v6)?;
                    if if f210(v0.clone(), v5)? {
                        true
                    } else {
                        let a4046 = {
                            let a4044 = v6;
                            let a4045 = v5;
                            nat_sub(a4044, a4045)
                        };
                        let a4047 = v1;
                        if if nat_lt(a4046, a4047) {
                            f214(f291(f212(v0.clone(), v7)?))
                        } else {
                            false
                        } {
                            f214({
                                let a4048 = f212(v0, v7)?;
                                let a4049 = 35u64;
                                nat_eq(a4048, a4049)
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

pub fn f593(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f293(v0.clone(), {
        let (h4050, _) = v1;
        h4050
    })?;
    let a4051 = f212(v0, v2)?;
    let a4052 = 10u64;
    if nat_eq(a4051, a4052) {
        Ok(Some(({
            let a4053 = v2;
            let a4054 = 1u64;
            nat_add(a4053, a4054)?
        }, {
            let a4056 = {
                let (_, h4055) = v1;
                h4055
            };
            let a4057 = 1u64;
            nat_add(a4056, a4057)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f594(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f615(v1, v0, v2)
}

pub fn f595() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f596(v0: Fn12, p4061: List<List<(Bytes, Bytes)>>, p4062: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m4060 = (p4061, p4062);
    {
        let mut m4060 = m4060;
        loop {
            let step = {
                let (v1, v2) = m4060;
                let m4058 = v2;
                match m4058.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c4059: Fn12 = v0.clone();
                            c4059.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m4060 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f597(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f598(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a4064 = {
        let a4063 = v0.clone();
        length_list(a4063)
    };
    let a4065 = 1u64;
    if nat_le(a4064, a4065) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f616(v0)?))
    }
}

pub fn f599(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4110 = {
            let a4086 = {
                let a4074 = {
                    let a4068 = {
                        let a4066 = v1;
                        let a4067 = 128u8;
                        fixed_u8::bit_and(a4066, a4067)
                    };
                    let a4069 = 128u8;
                    if equal(a4068, a4069) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4075 = {
                    let a4072 = {
                        let a4070 = v1;
                        let a4071 = 64u8;
                        fixed_u8::bit_and(a4070, a4071)
                    };
                    let a4073 = 64u8;
                    if equal(a4072, a4073) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4074, a4075)?
            };
            let a4087 = {
                let a4084 = {
                    let a4078 = {
                        let a4076 = v1;
                        let a4077 = 32u8;
                        fixed_u8::bit_and(a4076, a4077)
                    };
                    let a4079 = 32u8;
                    if equal(a4078, a4079) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4085 = {
                    let a4082 = {
                        let a4080 = v1;
                        let a4081 = 16u8;
                        fixed_u8::bit_and(a4080, a4081)
                    };
                    let a4083 = 16u8;
                    if equal(a4082, a4083) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4084, a4085)?
            };
            nat_add(a4086, a4087)?
        };
        let a4111 = {
            let a4108 = {
                let a4096 = {
                    let a4090 = {
                        let a4088 = v1;
                        let a4089 = 8u8;
                        fixed_u8::bit_and(a4088, a4089)
                    };
                    let a4091 = 8u8;
                    if equal(a4090, a4091) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4097 = {
                    let a4094 = {
                        let a4092 = v1;
                        let a4093 = 4u8;
                        fixed_u8::bit_and(a4092, a4093)
                    };
                    let a4095 = 4u8;
                    if equal(a4094, a4095) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4096, a4097)?
            };
            let a4109 = {
                let a4106 = {
                    let a4100 = {
                        let a4098 = v1;
                        let a4099 = 2u8;
                        fixed_u8::bit_and(a4098, a4099)
                    };
                    let a4101 = 2u8;
                    if equal(a4100, a4101) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4107 = {
                    let a4104 = {
                        let a4102 = v1;
                        let a4103 = 1u8;
                        fixed_u8::bit_and(a4102, a4103)
                    };
                    let a4105 = 1u8;
                    if equal(a4104, a4105) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4106, a4107)?
            };
            nat_add(a4108, a4109)?
        };
        nat_add(a4110, a4111)?
    };
    let a4112 = 48u64;
    let a4113 = v2;
    if if nat_le(a4112, a4113) {
        let a4114 = v2;
        let a4115 = 57u64;
        nat_le(a4114, a4115)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a4116 = 97u64;
        let a4117 = v2;
        if nat_le(a4116, a4117) {
            let a4118 = v2;
            let a4119 = 102u64;
            Ok(nat_le(a4118, a4119))
        } else {
            Ok(false)
        }
    }
}

pub fn f600() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f601(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4164 = {
            let a4140 = {
                let a4128 = {
                    let a4122 = {
                        let a4120 = v1;
                        let a4121 = 128u8;
                        fixed_u8::bit_and(a4120, a4121)
                    };
                    let a4123 = 128u8;
                    if equal(a4122, a4123) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4129 = {
                    let a4126 = {
                        let a4124 = v1;
                        let a4125 = 64u8;
                        fixed_u8::bit_and(a4124, a4125)
                    };
                    let a4127 = 64u8;
                    if equal(a4126, a4127) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4128, a4129)?
            };
            let a4141 = {
                let a4138 = {
                    let a4132 = {
                        let a4130 = v1;
                        let a4131 = 32u8;
                        fixed_u8::bit_and(a4130, a4131)
                    };
                    let a4133 = 32u8;
                    if equal(a4132, a4133) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4139 = {
                    let a4136 = {
                        let a4134 = v1;
                        let a4135 = 16u8;
                        fixed_u8::bit_and(a4134, a4135)
                    };
                    let a4137 = 16u8;
                    if equal(a4136, a4137) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4138, a4139)?
            };
            nat_add(a4140, a4141)?
        };
        let a4165 = {
            let a4162 = {
                let a4150 = {
                    let a4144 = {
                        let a4142 = v1;
                        let a4143 = 8u8;
                        fixed_u8::bit_and(a4142, a4143)
                    };
                    let a4145 = 8u8;
                    if equal(a4144, a4145) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4151 = {
                    let a4148 = {
                        let a4146 = v1;
                        let a4147 = 4u8;
                        fixed_u8::bit_and(a4146, a4147)
                    };
                    let a4149 = 4u8;
                    if equal(a4148, a4149) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4150, a4151)?
            };
            let a4163 = {
                let a4160 = {
                    let a4154 = {
                        let a4152 = v1;
                        let a4153 = 2u8;
                        fixed_u8::bit_and(a4152, a4153)
                    };
                    let a4155 = 2u8;
                    if equal(a4154, a4155) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4161 = {
                    let a4158 = {
                        let a4156 = v1;
                        let a4157 = 1u8;
                        fixed_u8::bit_and(a4156, a4157)
                    };
                    let a4159 = 1u8;
                    if equal(a4158, a4159) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4160, a4161)?
            };
            nat_add(a4162, a4163)?
        };
        nat_add(a4164, a4165)?
    };
    let a4166 = v2;
    let a4167 = 57u64;
    if nat_le(a4166, a4167) {
        let a4168 = v2;
        let a4169 = 48u64;
        Ok(nat_sub(a4168, a4169))
    } else {
        let a4170 = v2;
        let a4171 = 87u64;
        Ok(nat_sub(a4170, a4171))
    }
}

pub fn f602(v0: u64) -> bool {
    let a4172 = 48u64;
    let a4173 = v0;
    if nat_le(a4172, a4173) {
        let a4174 = v0;
        let a4175 = 55u64;
        nat_le(a4174, a4175)
    } else {
        false
    }
}

pub fn f603(v0: u64) -> bool {
    if f321(v0) {
        true
    } else {
        if f418(v0) {
            true
        } else {
            let a4176 = v0;
            let a4177 = 95u64;
            if nat_eq(a4176, a4177) {
                true
            } else {
                let a4178 = v0;
                let a4179 = 45u64;
                nat_eq(a4178, a4179)
            }
        }
    }
}

pub fn f604(v0: bool, v1: List<Bytes>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m4180 = v1;
    match m4180.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m4181 = f349(v0, v3.clone())?;
            match m4181 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f200(f8(Str::lit("the component name ")), v3, f8(Str::lit(" derives no LexLean name")))))
                }
                Some(v5) => {
                    if f153(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f200(f8(Str::lit("two components derive the name ")), v5, f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n; when those names are not distinct, every name keeps its letters and digits and writes each other character as a C11 universal character name)")))))
                    } else {
                        let m4182 = f604(v0, v4, List::cons(v5.clone(), v2))?;
                        match m4182 {
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

pub fn f605(v0: Adt32) -> Adt32 {
    Adt32::C0(match v0.clone() {
        Adt32::C0(h4183, _, _, _, _) => {
            h4183
        }
    }, match v0 {
        Adt32::C0(_, h4184, _, _, _) => {
            h4184
        }
    }, 0u64, 0u64, 0u64)
}

pub fn f606(v0: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: Adt25) -> R<Result<Adt36, Bytes>> {
    let m4185 = v0;
    if m4185 == 0 {
        Ok(Err::<Adt36, Bytes>(f466(f8(Str::lit("the element declarations nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m4185 - 1;
        let m4186 = f284(v4.clone(), Str::lit("ref"));
        match m4186 {
            None => {
                let m4197 = f284(v4.clone(), Str::lit("name"));
                match m4197 {
                    None => {
                        Ok(Err::<Adt36, Bytes>(f466(f8(Str::lit("a local element declaration must have a name or a ref (XSD 1.1 Part 1, section 3.3.3)")), v2)))
                    }
                    Some(v12) => {
                        let m4200 = f462(v5, v1, {
                            let a4198 = v2;
                            let a4199 = f191(v12.clone())?;
                            append_bytes(a4198, a4199)
                        }, v3, v4)?;
                        match m4200 {
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
                let m4188 = f119(match v4 {
                    Adt25::C0(_, _, _, _, h4187) => {
                        h4187
                    }
                }, v6.clone())?;
                match m4188 {
                    Ok(v8) => {
                        let m4189 = f538(v1.clone(), Str::lit("element"), v8.clone());
                        match m4189 {
                            None => {
                                Ok(Err::<Adt36, Bytes>(f466(f200(f8(Str::lit("the element ")), v6, f8(Str::lit(" is not declared by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                            }
                            Some(v9) => {
                                if if f525(match v9.clone() {
                                    Adt24::C0(_, _, _, h4190) => {
                                        h4190
                                    }
                                }, Str::lit("abstract"), Str::lit("true")) {
                                    true
                                } else {
                                    f617(match v1.clone() {
                                        Adt23::C0(h4191, _) => {
                                            h4191
                                        }
                                    }, v8.clone())?
                                } {
                                    Ok(Err::<Adt36, Bytes>(f466(f200(f8(Str::lit("the element ")), v6, f8(Str::lit(" heads a substitution group, so its particle also admits the members of that group, which is not translated (XSD 1.1 Part 1, section 3.3.6.4)"))), v2)))
                                } else {
                                    let m4195 = f462(v5, v1, f191({
                                        let (_, h4192) = v8.clone();
                                        h4192
                                    })?, match v9.clone() {
                                        Adt24::C0(_, h4193, _, _) => {
                                            h4193
                                        }
                                    }, match v9 {
                                        Adt24::C0(_, _, _, h4194) => {
                                            h4194
                                        }
                                    })?;
                                    match m4195 {
                                        Ok(v11) => {
                                            Ok(Ok::<Adt36, Bytes>(Adt36::C0({
                                                let (_, h4196) = v8;
                                                h4196
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
                        Ok(Err::<Adt36, Bytes>(f466(v7, v2)))
                    }
                }
            }
        }
    }
}

pub fn f607(v0: Adt25, v1: Bytes) -> Option<Bytes> {
    let v2: Bytes = f287(v0.clone(), Str::lit("maxOccurs"), Str::lit("1"));
    if f46(v2.clone(), f8(Str::lit("0"))) {
        None::<Bytes>
    } else {
        if f46(v2, f8(Str::lit("1"))) {
            if f525(v0, Str::lit("minOccurs"), Str::lit("0")) {
                Some(f580(Str::lit("Option"), v1))
            } else {
                Some(v1)
            }
        } else {
            Some(f580(Str::lit("List"), v1))
        }
    }
}

pub fn f608(v0: Adt25) -> bool {
    if f46(f287(v0.clone(), Str::lit("minOccurs"), Str::lit("1")), f8(Str::lit("1"))) {
        f46(f287(v0, Str::lit("maxOccurs"), Str::lit("1")), f8(Str::lit("1")))
    } else {
        false
    }
}

pub fn f609(v0: Adt32, v1: Bytes) -> R<u64> {
    if f46(v1.clone(), f8(Str::lit("sequence"))) {
        let a4202 = match v0 {
            Adt32::C0(_, _, h4201, _, _) => {
                h4201
            }
        };
        let a4203 = 1u64;
        nat_add(a4202, a4203)
    } else {
        if f46(v1, f8(Str::lit("choice"))) {
            let a4205 = match v0 {
                Adt32::C0(_, _, _, h4204, _) => {
                    h4204
                }
            };
            let a4206 = 1u64;
            nat_add(a4205, a4206)
        } else {
            let a4208 = match v0 {
                Adt32::C0(_, _, _, _, h4207) => {
                    h4207
                }
            };
            let a4209 = 1u64;
            nat_add(a4208, a4209)
        }
    }
}

pub fn f610(v0: Adt32, v1: Bytes) -> R<Adt32> {
    if f46(v1.clone(), f8(Str::lit("sequence"))) {
        Ok(Adt32::C0(match v0.clone() {
            Adt32::C0(h4210, _, _, _, _) => {
                h4210
            }
        }, match v0.clone() {
            Adt32::C0(_, h4211, _, _, _) => {
                h4211
            }
        }, {
            let a4213 = match v0.clone() {
                Adt32::C0(_, _, h4212, _, _) => {
                    h4212
                }
            };
            let a4214 = 1u64;
            nat_add(a4213, a4214)?
        }, match v0.clone() {
            Adt32::C0(_, _, _, h4215, _) => {
                h4215
            }
        }, match v0 {
            Adt32::C0(_, _, _, _, h4216) => {
                h4216
            }
        }))
    } else {
        if f46(v1, f8(Str::lit("choice"))) {
            Ok(Adt32::C0(match v0.clone() {
                Adt32::C0(h4217, _, _, _, _) => {
                    h4217
                }
            }, match v0.clone() {
                Adt32::C0(_, h4218, _, _, _) => {
                    h4218
                }
            }, match v0.clone() {
                Adt32::C0(_, _, h4219, _, _) => {
                    h4219
                }
            }, {
                let a4221 = match v0.clone() {
                    Adt32::C0(_, _, _, h4220, _) => {
                        h4220
                    }
                };
                let a4222 = 1u64;
                nat_add(a4221, a4222)?
            }, match v0 {
                Adt32::C0(_, _, _, _, h4223) => {
                    h4223
                }
            }))
        } else {
            Ok(Adt32::C0(match v0.clone() {
                Adt32::C0(h4224, _, _, _, _) => {
                    h4224
                }
            }, match v0.clone() {
                Adt32::C0(_, h4225, _, _, _) => {
                    h4225
                }
            }, match v0.clone() {
                Adt32::C0(_, _, h4226, _, _) => {
                    h4226
                }
            }, match v0.clone() {
                Adt32::C0(_, _, _, h4227, _) => {
                    h4227
                }
            }, {
                let a4229 = match v0 {
                    Adt32::C0(_, _, _, _, h4228) => {
                        h4228
                    }
                };
                let a4230 = 1u64;
                nat_add(a4229, a4230)?
            }))
        }
    }
}

pub fn f611(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f546(v0.clone(), {
        let (h4231, _) = v2;
        h4231
    }, v1)?;
    let a4232 = f212(v0, v3)?;
    let a4233 = 10u64;
    if nat_eq(a4232, a4233) {
        Ok(Some(({
            let a4234 = v3;
            let a4235 = 1u64;
            nat_add(a4234, a4235)?
        }, {
            let a4237 = {
                let (_, h4236) = v2;
                h4236
            };
            let a4238 = 1u64;
            nat_add(a4237, a4238)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f612(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a4239 = f212(v0, v3)?;
    let a4240 = 32u64;
    if if nat_eq(a4239, a4240) {
        let a4243 = {
            let a4241 = v3;
            let a4242 = v1;
            nat_sub(a4241, a4242)
        };
        let a4244 = v2;
        nat_lt(a4243, a4244)
    } else {
        false
    } {
        Ok(Some({
            let a4245 = v3;
            let a4246 = 1u64;
            nat_add(a4245, a4246)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f613(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f212(v0, v1)?;
    let a4247 = v2;
    let a4248 = 34u64;
    if if if if nat_eq(a4247, a4248) {
        true
    } else {
        let a4249 = v2;
        let a4250 = 92u64;
        nat_eq(a4249, a4250)
    } {
        true
    } else {
        f290(v2)
    } {
        true
    } else {
        f291(v2)
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a4251 = v1;
            let a4252 = 1u64;
            nat_add(a4251, a4252)?
        }))
    }
}

pub fn f614(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f212(v0, v1)?;
    let a4253 = v2;
    let a4254 = 39u64;
    if if if nat_eq(a4253, a4254) {
        true
    } else {
        f290(v2)
    } {
        true
    } else {
        f291(v2)
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a4255 = v1;
            let a4256 = 1u64;
            nat_add(a4255, a4256)?
        }))
    }
}

pub fn f615(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m4258 = f589(f212(v0, {
        let (h4257, _) = v2;
        h4257
    })?);
    match m4258 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v3) => {
            let a4259 = v3;
            let a4260 = v1;
            if nat_lt(a4259, a4260) {
                Ok(Some(({
                    let a4262 = {
                        let (h4261, _) = v2;
                        h4261
                    };
                    let a4263 = 1u64;
                    nat_add(a4262, a4263)?
                }, {
                    let a4267 = {
                        let a4265 = {
                            let (_, h4264) = v2;
                            h4264
                        };
                        let a4266 = v1;
                        nat_mul(a4265, a4266)?
                    };
                    let a4268 = v3;
                    nat_add(a4267, a4268)?
                })))
            } else {
                Ok(None::<(u64, u64)>)
            }
        }
    }
}

pub fn f616(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f563({
        let (_, h4269) = {
            let (h4270, _) = f618(Fn13::F619, {
                let a4272 = {
                    let a4271 = v0.clone();
                    length_list(a4271)
                };
                let a4273 = 1u64;
                nat_add(a4272, a4273)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h4270
        };
        h4269
    }))
}

pub fn f617(p4282: List<Adt24>, v1: (Bytes, Bytes)) -> R<bool> {
    let m4281 = p4282;
    {
        let mut m4281 = m4281;
        loop {
            let step = {
                let v0 = m4281;
                let m4274 = v0;
                match m4274.uncons() {
                    None => {
                        Err::<List<Adt24>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if if f46(match v2.clone() {
                            Adt24::C0(h4275, _, _, _) => {
                                h4275
                            }
                        }, f8(Str::lit("element"))) {
                            let m4277 = f284(match v2.clone() {
                                Adt24::C0(_, _, _, h4276) => {
                                    h4276
                                }
                            }, Str::lit("substitutionGroup"));
                            match m4277 {
                                None => {
                                    false
                                }
                                Some(v4) => {
                                    f620({
                                        let m4280 = match v2 {
                                            Adt24::C0(_, _, _, h4279) => {
                                                h4279
                                            }
                                        };
                                        match m4280 {
                                            Adt25::C0(_, _, _, _, h4278) => {
                                                h4278
                                            }
                                        }
                                    }, f621(v4)?, v1.clone())?
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
                Ok(next) => { m4281 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f618(v0: Fn13, p4287: u64, p4288: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m4286 = (p4287, p4288);
    {
        let mut m4286 = m4286;
        loop {
            let step = {
                let (v1, v2) = m4286;
                let m4283 = v1;
                if m4283 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4283 - 1;
                    let m4285 = {
                        let c4284: Fn13 = v0.clone();
                        c4284.apply(v2.clone())?
                    };
                    match m4285 {
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
                Ok(next) => { m4286 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f619(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f622(v0)
}

pub fn f620(v0: List<(Bytes, Bytes)>, p4296: List<Bytes>, v2: (Bytes, Bytes)) -> R<bool> {
    let m4295 = p4296;
    {
        let mut m4295 = m4295;
        loop {
            let step = {
                let v1 = m4295;
                let m4289 = v1;
                match m4289.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v3, v4)) => {
                        let m4290 = f119(v0.clone(), v3)?;
                        match m4290 {
                            Ok(v6) => {
                                if if f46({
                                    let (h4291, _) = v6;
                                    h4291
                                }, {
                                    let (h4292, _) = v2.clone();
                                    h4292
                                }) {
                                    f46({
                                        let (_, h4293) = v6;
                                        h4293
                                    }, {
                                        let (_, h4294) = v2.clone();
                                        h4294
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
                Ok(next) => { m4295 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f621(v0: Bytes) -> R<List<Bytes>> {
    f41(v0, 32u8)
}

pub fn f622(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h4297, _) = v0;
        h4297
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h4298) = v0;
        h4298
    };
    let m4299 = v1;
    match m4299.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m4300 = v4;
            match m4300.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f623(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f623(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a4305 = {
            let a4303 = {
                let a4301 = v0.clone();
                length_list(a4301)
            };
            let a4304 = {
                let a4302 = v1.clone();
                length_list(a4302)
            };
            nat_add(a4303, a4304)?
        };
        let a4306 = 1u64;
        nat_add(a4305, a4306)?
    };
    Ok(f624({
        let (_, h4307) = {
            let (_, h4308) = {
                let (h4309, _) = f625(Fn14::F626, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h4309
            };
            h4308
        };
        h4307
    }))
}

pub fn f624(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f406(Fn6::F627, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f625(v0: Fn14, p4314: u64, p4315: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m4313 = (p4314, p4315);
    {
        let mut m4313 = m4313;
        loop {
            let step = {
                let (v1, v2) = m4313;
                let m4310 = v1;
                if m4310 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m4310 - 1;
                    let m4312 = {
                        let c4311: Fn14 = v0.clone();
                        c4311.apply(v2.clone())
                    };
                    match m4312 {
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
                Ok(next) => { m4313 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f626(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f628(v0)
}

pub fn f627(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f628(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h4316, _) = v0.clone();
        h4316
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h4317, _) = {
            let (_, h4318) = v0.clone();
            h4318
        };
        h4317
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h4319) = {
            let (_, h4320) = v0;
            h4320
        };
        h4319
    };
    let m4321 = v1.clone();
    match m4321.uncons() {
        None => {
            let m4322 = v2;
            match m4322.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m4323 = v2.clone();
            match m4323.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f629({
                        let (h4324, _) = v8.clone();
                        h4324
                    }, {
                        let (h4325, _) = v6.clone();
                        h4325
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

pub fn f629(v0: Bytes, v1: Bytes) -> bool {
    let a4330 = {
        let a4326 = v0;
        let a4327 = v1;
        compare_bytes(a4326, a4327)
    };
    let a4331 = {
        let a4328 = Bytes::lit(&[]);
        let a4329 = Bytes::lit(&[0x01]);
        compare_bytes(a4328, a4329)
    };
    equal(a4330, a4331)
}

pub fn run(p0: Option<Adt0>, p1: Adt3) -> R<(Adt0, Adt10)> {
    f0(p0, p1)
}
