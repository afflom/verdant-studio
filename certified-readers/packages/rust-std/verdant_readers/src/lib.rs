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
    F225(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F44(k0, k1) => f44(*k0, k1.clone(), p0),
            Fn0::F225(k0) => f225(k0.clone(), p0),
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
    F451(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn3::F128(k0, k1) => Ok(f128(*k0, k1.clone(), p0)),
            Fn3::F129(k0) => f129(k0.clone(), p0),
            Fn3::F451(k0) => f451(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F131(Bytes),
    F170(Bytes),
    F171(Bytes),
    F209(Bytes, Bytes),
    F210(Bytes),
    F212(Bytes, u64),
    F315(bool, u64, Bytes),
    F346(Bytes),
    F347(Bytes),
    F348(Bytes),
    F368(u64, Bytes),
    F450(Bytes),
    F542(u64, Bytes, u64),
    F545(Bytes),
    F546(Bytes),
    F548(Bytes, u64),
    F551(Bytes),
    F552(Bytes),
    F580(Bytes),
    F584(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn4::F131(k0) => f131(k0.clone(), p0),
            Fn4::F170(k0) => f170(k0.clone(), p0),
            Fn4::F171(k0) => f171(k0.clone(), p0),
            Fn4::F209(k0, k1) => f209(k0.clone(), k1.clone(), p0),
            Fn4::F210(k0) => f210(k0.clone(), p0),
            Fn4::F212(k0, k1) => f212(k0.clone(), *k1, p0),
            Fn4::F315(k0, k1, k2) => f315(*k0, *k1, k2.clone(), p0),
            Fn4::F346(k0) => f346(k0.clone(), p0),
            Fn4::F347(k0) => f347(k0.clone(), p0),
            Fn4::F348(k0) => f348(k0.clone(), p0),
            Fn4::F368(k0, k1) => f368(*k0, k1.clone(), p0),
            Fn4::F450(k0) => f450(k0.clone(), p0),
            Fn4::F542(k0, k1, k2) => f542(*k0, k1.clone(), *k2, p0),
            Fn4::F545(k0) => f545(k0.clone(), p0),
            Fn4::F546(k0) => f546(k0.clone(), p0),
            Fn4::F548(k0, k1) => f548(k0.clone(), *k1, p0),
            Fn4::F551(k0) => f551(k0.clone(), p0),
            Fn4::F552(k0) => f552(k0.clone(), p0),
            Fn4::F580(k0) => f580(k0.clone(), p0),
            Fn4::F584(k0) => f584(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F364,
    F598,
}

impl Fn5 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn5::F364 => f364(p0, p1),
            Fn5::F598 => f598(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F367,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn6::F367 => f367(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F400(bool, Bytes),
    F433(Bytes),
    F442(bool, Bytes),
    F498(Bytes),
    F509(Bytes),
    F541(u64, Bytes),
    F550(u64, Bytes),
    F554(Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn7::F400(k0, k1) => f400(*k0, k1.clone(), p0),
            Fn7::F433(k0) => f433(k0.clone(), p0),
            Fn7::F442(k0, k1) => f442(*k0, k1.clone(), p0),
            Fn7::F498(k0) => f498(k0.clone(), p0),
            Fn7::F509(k0) => f509(k0.clone(), p0),
            Fn7::F541(k0, k1) => f541(*k0, k1.clone(), p0),
            Fn7::F550(k0, k1) => f550(*k0, k1.clone(), p0),
            Fn7::F554(k0) => f554(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F506(u64, Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: (u64, bool)) -> R<Option<(u64, bool)>> {
        match self {
            Fn8::F506(k0, k1) => f506(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F520,
}

impl Fn9 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn9::F520 => f520(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F522,
}

impl Fn10 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn10::F522 => f522(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F558,
}

impl Fn11 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn11::F558 => f558(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F589,
}

impl Fn12 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn12::F589 => f589(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F597,
}

impl Fn13 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn13::F597 => f597(p0),
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
    let v3: Bytes = f154(f114(v1.clone()))?;
    if if f185(v3.clone()) {
        f111(v2)
    } else {
        false
    } {
        Ok(Err::<(Bytes, List<Bytes>), Bytes>(f186(v3)))
    } else {
        let a720 = v2;
        let a721 = 0u64;
        if if nat_eq(a720, a721) {
            true
        } else {
            let a722 = v2;
            let a723 = 2u64;
            nat_eq(a722, a723)
        } {
            let m724 = f187(v0.clone(), v0.clone(), f114(v1.clone()));
            match m724 {
                None => {
                    let m728 = f188({
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
                                Ok(Err::<(Bytes, List<Bytes>), Bytes>(f8(Str::lit("it has no data members, and LexLean refuses a structure with no fields"))))
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
                        let m738 = f194(f190(f193(v1.clone()))?, f193(v1.clone()))?;
                        match m738 {
                            None => {
                                let m739 = f195(f190(f193(v1.clone()))?, f196(f193(v1)), List::<Bytes>::nil())?;
                                match m739 {
                                    Ok(v10) => {
                                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f197(v3, v10), List::<Bytes>::nil())))
                                    }
                                    Err(v9) => {
                                        Ok(Err::<(Bytes, List<Bytes>), Bytes>(v9))
                                    }
                                }
                            }
                            Some(v11) => {
                                Ok(Err::<(Bytes, List<Bytes>), Bytes>(f186(v11)))
                            }
                        }
                    }
                    Some(v12) => {
                        Ok(Err::<(Bytes, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("the enumeration value \"")), List::cons(v12, List::cons(f8(Str::lit("\" appears twice (Web IDL 2.9: the values must be distinct)")), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                let a740 = v2;
                let a741 = 4u64;
                if nat_eq(a740, a741) {
                    let v13: Adt14 = f198(v1);
                    if !f35(f199(v13.clone()), Bytes::lit(&[])) {
                        Ok(Err::<(Bytes, List<Bytes>), Bytes>(f199(v13)))
                    } else {
                        let a742 = f200(v13.clone());
                        let a743 = 1u64;
                        if nat_eq(a742, a743) {
                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f197(v3, List::cons(f201(v13.clone()), List::<Bytes>::nil())), f202(v13))))
                        } else {
                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f203(v3, f201(v13.clone())), f202(v13))))
                        }
                    }
                } else {
                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f7(f193(v1))))
                }
            }
        }
    }
}

pub fn f114(v0: Adt12) -> Bytes {
    let m744 = v0;
    match m744 {
        Adt12::C0(_, v2, _, _, _, _) => {
            v2
        }
    }
}

pub fn f115(v0: Bytes, p747: List<Bytes>) -> bool {
    let m746 = p747;
    {
        let mut m746 = m746;
        loop {
            let step = {
                let v1 = m746;
                let m745 = v1;
                match m745.uncons() {
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
                Ok(next) => { m746 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f116(p750: List<Adt15>, v1: Bytes) -> Option<Adt15> {
    let m749 = p750;
    {
        let mut m749 = m749;
        loop {
            let step = {
                let v0 = m749;
                let m748 = v0;
                match m748.uncons() {
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
                Ok(next) => { m749 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f117(v0: Adt15) -> Bytes {
    let m751 = v0;
    match m751 {
        Adt15::C0(_, v2, _, _) => {
            v2
        }
    }
}

pub fn f118(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m752 = v0;
    match m752.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f118(v3, v1))
        }
    }
}

pub fn f119(v0: Adt15) -> List<Bytes> {
    let m753 = v0;
    match m753 {
        Adt15::C0(_, _, _, v4) => {
            v4
        }
    }
}

pub fn f120(v0: Adt15) -> Bytes {
    let m754 = v0;
    match m754 {
        Adt15::C0(v1, _, _, _) => {
            v1
        }
    }
}

pub fn f121(v0: Adt15) -> Bytes {
    let m755 = v0;
    match m755 {
        Adt15::C0(_, _, v3, _) => {
            v3
        }
    }
}

pub fn f122(v0: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)> {
    let m756 = v0;
    match m756.uncons() {
        None => {
            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
        }
        Some((v2, v3)) => {
            let m759 = f155({
                let (h757, _) = {
                    let (_, h758) = v2.clone();
                    h758
                };
                h757
            }, v1.clone());
            match m759 {
                None => {
                    Some((({
                        let (h760, _) = v2;
                        h760
                    }, {
                        let (_, h761) = {
                            let (_, h762) = v2;
                            h762
                        };
                        h761
                    }), v3))
                }
                Some(_) => {
                    let m763 = f122(v3, v1);
                    match m763 {
                        None => {
                            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h764, _) = v5;
                                h764
                            }, List::cons(v2, {
                                let (_, h765) = v5;
                                h765
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f123(p770: List<Adt12>, v1: u64, v2: Bytes) -> Option<Adt12> {
    let m769 = p770;
    {
        let mut m769 = m769;
        loop {
            let step = {
                let v0 = m769;
                let m766 = v0;
                match m766.uncons() {
                    None => {
                        Err::<List<Adt12>, Option<Adt12>>(None::<Adt12>)
                    }
                    Some((v3, v4)) => {
                        let a767 = f112(v3.clone());
                        let a768 = v1;
                        if if nat_eq(a767, a768) {
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
                Ok(next) => { m769 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f124(v0: u64) -> Bytes {
    let a771 = v0;
    let a772 = 5u64;
    if nat_eq(a771, a772) {
        f8(Str::lit("partial interface "))
    } else {
        let a773 = v0;
        let a774 = 7u64;
        if nat_eq(a773, a774) {
            f8(Str::lit("partial dictionary "))
        } else {
            f8(Str::lit(""))
        }
    }
}

pub fn f125(v0: Adt12) -> Bytes {
    let m775 = v0;
    match m775 {
        Adt12::C0(_, _, v3, _, _, _) => {
            v3
        }
    }
}

pub fn f126(v0: Bytes, v1: u64) -> Bytes {
    let m779 = {
        let a776 = v0;
        let a777 = v1;
        let a778 = 1u64;
        slice_bytes(a776, a777, a778)
    };
    match m779 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f127(v0: Fn3, p784: u64, p785: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m783 = (p784, p785);
    {
        let mut m783 = m783;
        loop {
            let step = {
                let (v1, v2) = m783;
                let m780 = v1;
                if m780 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m780 - 1;
                    let m782 = {
                        let c781: Fn3 = v0.clone();
                        c781.apply(v2.clone())?
                    };
                    match m782 {
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
                Ok(next) => { m783 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f128(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f204(v1, v0, v2)
}

pub fn f129(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f205(v0, v1)
}

pub fn f130(v0: Fn4, p790: u64, p791: u64) -> R<(u64, bool)> {
    let m789 = (p790, p791);
    {
        let mut m789 = m789;
        loop {
            let step = {
                let (v1, v2) = m789;
                let m786 = v1;
                if m786 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m786 - 1;
                    let m788 = {
                        let c787: Fn4 = v0.clone();
                        c787.apply(v2)?
                    };
                    match m788 {
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
                Ok(next) => { m789 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f131(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f206(v0, v1)
}

pub fn f132(v0: u64) -> R<bool> {
    f207(v0, f208())
}

pub fn f133(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let v3: u64 = {
        let (h792, _) = f130(Fn4::F209(v1.clone(), v0.clone()), {
            let a794 = {
                let a793 = v0.clone();
                length_bytes(a793)
            };
            let a795 = 1u64;
            nat_add(a794, a795)?
        }, v2)?;
        h792
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
    let (h796, _) = f130(Fn4::F210(v0.clone()), {
        let a798 = {
            let a797 = v0;
            length_bytes(a797)
        };
        let a799 = 1u64;
        nat_add(a798, a799)?
    }, v1)?;
    Ok(h796)
}

pub fn f136(v0: Bytes) -> bool {
    f211(v0, List::cons(f8(Str::lit("UTF-8")), List::cons(f8(Str::lit("UTf-8")), List::cons(f8(Str::lit("UtF-8")), List::cons(f8(Str::lit("Utf-8")), List::cons(f8(Str::lit("uTF-8")), List::cons(f8(Str::lit("uTf-8")), List::cons(f8(Str::lit("utF-8")), List::cons(f8(Str::lit("utf-8")), List::<Bytes>::nil())))))))))
}

pub fn f137(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let m803 = f133(v0.clone(), {
        let a800 = Str::lit("--");
        utf8_encode(a800)
    }, {
        let a801 = v1;
        let a802 = 4u64;
        nat_add(a801, a802)?
    })?;
    match m803 {
        None => {
            Ok(f134(f83(Str::lit("a comment is not closed (XML 1.0 [15])"), v1)?))
        }
        Some(v2) => {
            let a806 = f87(v0, {
                let a804 = v2;
                let a805 = 2u64;
                nat_add(a804, a805)?
            })?;
            let a807 = 62u64;
            if nat_eq(a806, a807) {
                Ok(Ok::<u64, Bytes>({
                    let a808 = v2;
                    let a809 = 3u64;
                    nat_add(a808, a809)?
                }))
            } else {
                Ok(f134(f83(Str::lit("-- is not allowed inside a comment (XML 1.0 [15])"), v2)?))
            }
        }
    }
}

pub fn f138(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f141(v0.clone(), {
        let a810 = v1;
        let a811 = 2u64;
        nat_add(a810, a811)?
    })?;
    let a814 = v2;
    let a815 = {
        let a812 = v1;
        let a813 = 2u64;
        nat_add(a812, a813)?
    };
    if nat_eq(a814, a815) {
        Ok(f134(f83(Str::lit("a processing instruction must start with a Name (XML 1.0 [16])"), v1)?))
    } else {
        let a818 = v2;
        let a819 = {
            let a816 = v1;
            let a817 = 5u64;
            nat_add(a816, a817)?
        };
        if if nat_eq(a818, a819) {
            f211(f32(v0.clone(), {
                let a820 = v1;
                let a821 = 2u64;
                nat_add(a820, a821)?
            }, 3u64), List::cons(f8(Str::lit("XML")), List::cons(f8(Str::lit("XMl")), List::cons(f8(Str::lit("XmL")), List::cons(f8(Str::lit("Xml")), List::cons(f8(Str::lit("xML")), List::cons(f8(Str::lit("xMl")), List::cons(f8(Str::lit("xmL")), List::cons(f8(Str::lit("xml")), List::<Bytes>::nil())))))))))
        } else {
            false
        } {
            Ok(f134(f83(Str::lit("the target xml is reserved; an XML declaration may appear only at the very start (XML 1.0 [17], [22])"), v1)?))
        } else {
            let m823 = f133(v0, {
                let a822 = Str::lit("?>");
                utf8_encode(a822)
            }, v2)?;
            match m823 {
                None => {
                    Ok(f134(f83(Str::lit("a processing instruction is not closed (XML 1.0 [16])"), v1)?))
                }
                Some(v3) => {
                    Ok(Ok::<u64, Bytes>({
                        let a824 = v3;
                        let a825 = 2u64;
                        nat_add(a824, a825)?
                    }))
                }
            }
        }
    }
}

pub fn f139(v0: Bytes, v1: u64) -> R<u64> {
    let m828 = {
        let a826 = v0;
        let a827 = v1;
        index_bytes(a826, a827)
    };
    match m828 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a873 = {
                let a849 = {
                    let a837 = {
                        let a831 = {
                            let a829 = v3;
                            let a830 = 128u8;
                            fixed_u8::bit_and(a829, a830)
                        };
                        let a832 = 128u8;
                        if equal(a831, a832) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a838 = {
                        let a835 = {
                            let a833 = v3;
                            let a834 = 64u8;
                            fixed_u8::bit_and(a833, a834)
                        };
                        let a836 = 64u8;
                        if equal(a835, a836) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a837, a838)?
                };
                let a850 = {
                    let a847 = {
                        let a841 = {
                            let a839 = v3;
                            let a840 = 32u8;
                            fixed_u8::bit_and(a839, a840)
                        };
                        let a842 = 32u8;
                        if equal(a841, a842) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a848 = {
                        let a845 = {
                            let a843 = v3;
                            let a844 = 16u8;
                            fixed_u8::bit_and(a843, a844)
                        };
                        let a846 = 16u8;
                        if equal(a845, a846) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a847, a848)?
                };
                nat_add(a849, a850)?
            };
            let a874 = {
                let a871 = {
                    let a859 = {
                        let a853 = {
                            let a851 = v3;
                            let a852 = 8u8;
                            fixed_u8::bit_and(a851, a852)
                        };
                        let a854 = 8u8;
                        if equal(a853, a854) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a860 = {
                        let a857 = {
                            let a855 = v3;
                            let a856 = 4u8;
                            fixed_u8::bit_and(a855, a856)
                        };
                        let a858 = 4u8;
                        if equal(a857, a858) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a859, a860)?
                };
                let a872 = {
                    let a869 = {
                        let a863 = {
                            let a861 = v3;
                            let a862 = 2u8;
                            fixed_u8::bit_and(a861, a862)
                        };
                        let a864 = 2u8;
                        if equal(a863, a864) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a870 = {
                        let a867 = {
                            let a865 = v3;
                            let a866 = 1u8;
                            fixed_u8::bit_and(a865, a866)
                        };
                        let a868 = 1u8;
                        if equal(a867, a868) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a869, a870)?
                };
                nat_add(a871, a872)?
            };
            nat_add(a873, a874)
        }
    }
}

pub fn f140(v0: Bytes) -> Result<(Adt16, u64), Bytes> {
    Err::<(Adt16, u64), Bytes>(v0)
}

pub fn f141(v0: Bytes, v1: u64) -> R<u64> {
    let (h875, _) = f130(Fn4::F212(v0.clone(), v1), {
        let a877 = {
            let a876 = v0;
            length_bytes(a876)
        };
        let a878 = 1u64;
        nat_add(a877, a878)?
    }, v1)?;
    Ok(h875)
}

pub fn f142(v0: Bytes, v1: u64, v2: u64, v3: List<Bytes>) -> R<Result<(Adt16, u64), Bytes>> {
    let m879 = v2;
    if m879 == 0 {
        Ok(f140(f83(Str::lit("a tag has more attributes than the input has bytes"), v1)?))
    } else {
        let v4 = m879 - 1;
        let v5: u64 = f135(v0.clone(), v1)?;
        let a880 = f87(v0.clone(), v5)?;
        let a881 = 62u64;
        if if nat_eq(a880, a881) {
            true
        } else {
            f86(v0.clone(), v5, Str::lit("/>"))
        } {
            Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C4, v5)))
        } else {
            let a882 = v5;
            let a883 = v1;
            if nat_eq(a882, a883) {
                Ok(f140(f83(Str::lit("attributes must be separated by white space (XML 1.0 [40])"), v1)?))
            } else {
                let v6: u64 = f141(v0.clone(), v5)?;
                let a884 = v6;
                let a885 = v5;
                if nat_eq(a884, a885) {
                    Ok(f140(f83(Str::lit("an attribute must start with a Name (XML 1.0 [41])"), v5)?))
                } else {
                    let v7: Bytes = f32(v0.clone(), v5, {
                        let a886 = v6;
                        let a887 = v5;
                        nat_sub(a886, a887)
                    });
                    if f211(v7.clone(), v3.clone()) {
                        Ok(f140(f83(Str::lit("an attribute name appears twice in one tag (XML 1.0 WFC: Unique Att Spec)"), v5)?))
                    } else {
                        let v8: u64 = f135(v0.clone(), v6)?;
                        let a888 = f87(v0.clone(), v8)?;
                        let a889 = 61u64;
                        if nat_eq(a888, a889) {
                            let v9: u64 = f135(v0.clone(), {
                                let a890 = v8;
                                let a891 = 1u64;
                                nat_add(a890, a891)?
                            })?;
                            let v10: u64 = f87(v0.clone(), v9)?;
                            let a892 = v10;
                            let a893 = 34u64;
                            if if nat_eq(a892, a893) {
                                true
                            } else {
                                let a894 = v10;
                                let a895 = 39u64;
                                nat_eq(a894, a895)
                            } {
                                let m898 = f143(v0.clone(), {
                                    let a896 = v9;
                                    let a897 = 1u64;
                                    nat_add(a896, a897)?
                                }, v4, Bytes::lit(&[]), true, v10)?;
                                match m898 {
                                    Ok(v12) => {
                                        let m900 = f142(v0, {
                                            let (_, h899) = v12;
                                            h899
                                        }, v4, List::cons(v7.clone(), v3))?;
                                        match m900 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C1(v7, {
                                                    let (h901, _) = v12;
                                                    h901
                                                }, std::rc::Rc::new({
                                                    let (h902, _) = v14;
                                                    h902
                                                })), {
                                                    let (_, h903) = v14;
                                                    h903
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

pub fn f143(v0: Bytes, p943: u64, p944: u64, p945: Bytes, v4: bool, v5: u64) -> R<Result<(Bytes, u64), Bytes>> {
    let m942 = (p943, (p944, p945));
    {
        let mut m942 = m942;
        loop {
            let step = {
                let (v1, (v2, v3)) = m942;
                let m904 = v2;
                if m904 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f213(f83(Str::lit("the input ends inside a text run"), v1)?)))
                } else {
                    let v6 = m904 - 1;
                    let v7: u64 = f87(v0.clone(), v1)?;
                    if if v4 {
                        let a905 = v7;
                        let a906 = v5;
                        nat_eq(a905, a906)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, {
                            let a907 = v1;
                            let a908 = 1u64;
                            nat_add(a907, a908)?
                        }))))
                    } else {
                        let a909 = v7;
                        let a910 = 256u64;
                        if nat_eq(a909, a910) {
                            if v4 {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f213(f83(Str::lit("an attribute value is not closed (XML 1.0 [10])"), v1)?)))
                            } else {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                            }
                        } else {
                            let a911 = v7;
                            let a912 = 60u64;
                            if nat_eq(a911, a912) {
                                if v4 {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f213(f83(Str::lit("< is not allowed in an attribute value (XML 1.0 [10], WFC: No < in Attribute Values)"), v1)?)))
                                } else {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                                }
                            } else {
                                let a913 = v7;
                                let a914 = 38u64;
                                if nat_eq(a913, a914) {
                                    let m915 = f214(v0.clone(), v1)?;
                                    match m915 {
                                        Ok(v9) => {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let (_, h916) = v9;
                                                h916
                                            }, (v6, {
                                                let a918 = v3;
                                                let a919 = {
                                                    let (h917, _) = v9;
                                                    h917
                                                };
                                                append_bytes(a918, a919)
                                            })))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f213(v8)))
                                        }
                                    }
                                } else {
                                    if v4 {
                                        let a920 = v7;
                                        let a921 = 9u64;
                                        if if nat_eq(a920, a921) {
                                            true
                                        } else {
                                            let a922 = v7;
                                            let a923 = 10u64;
                                            nat_eq(a922, a923)
                                        } {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let a924 = v1;
                                                let a925 = 1u64;
                                                nat_add(a924, a925)?
                                            }, (v6, {
                                                let a926 = v3;
                                                let a927 = f8(Str::lit(" "));
                                                append_bytes(a926, a927)
                                            })))
                                        } else {
                                            let v10: u64 = f215(v0.clone(), true, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v10, (v6, {
                                                let a930 = v3;
                                                let a931 = f32(v0.clone(), v1, {
                                                    let a928 = v10;
                                                    let a929 = v1;
                                                    nat_sub(a928, a929)
                                                });
                                                append_bytes(a930, a931)
                                            })))
                                        }
                                    } else {
                                        let a932 = v7;
                                        let a933 = 93u64;
                                        if nat_eq(a932, a933) {
                                            if f86(v0.clone(), v1, Str::lit("]]>")) {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f213(f83(Str::lit("]]> is not allowed in character data (XML 1.0 [14])"), v1)?)))
                                            } else {
                                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                    let a934 = v1;
                                                    let a935 = 1u64;
                                                    nat_add(a934, a935)?
                                                }, (v6, {
                                                    let a936 = v3;
                                                    let a937 = f8(Str::lit("]"));
                                                    append_bytes(a936, a937)
                                                })))
                                            }
                                        } else {
                                            let v11: u64 = f215(v0.clone(), false, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v11, (v6, {
                                                let a940 = v3;
                                                let a941 = f32(v0.clone(), v1, {
                                                    let a938 = v11;
                                                    let a939 = v1;
                                                    nat_sub(a938, a939)
                                                });
                                                append_bytes(a940, a941)
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
                Ok(next) => { m942 = next; }
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
    let m950 = {
        let a947 = v0;
        let a948 = 0u64;
        let a949 = {
            let a946 = v1.clone();
            length_bytes(a946)
        };
        slice_bytes(a947, a948, a949)
    };
    match m950 {
        None => {
            false
        }
        Some(v2) => {
            let a951 = v2;
            let a952 = v1;
            equal(a951, a952)
        }
    }
}

pub fn f148(v0: Bytes) -> R<Bytes> {
    let m953 = f133(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m953 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v1) => {
            Ok(f32(v0, 0u64, v1))
        }
    }
}

pub fn f149(p958: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m957 = p958;
    {
        let mut m957 = m957;
        loop {
            let step = {
                let v0 = m957;
                let m954 = v0;
                match m954.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h955, _) = v2;
                            h955
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h956) = v2;
                                h956
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m957 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f150(v0: Bytes) -> R<Bytes> {
    let m959 = f133(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m959 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(f32(v0.clone(), {
                let a960 = v1;
                let a961 = 1u64;
                nat_add(a960, a961)?
            }, {
                let a965 = {
                    let a962 = v0;
                    length_bytes(a962)
                };
                let a966 = {
                    let a963 = v1;
                    let a964 = 1u64;
                    nat_add(a963, a964)?
                };
                nat_sub(a965, a966)
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
    let m967 = f216(v1, v2)?;
    match m967 {
        Ok(v6) => {
            let m968 = v6;
            match m968 {
                None => {
                    Ok(f217(f218(f8(Str::lit("a simpleType must have a restriction, list or union child (XSD 1.1 Part 2, section 4.1.2)")), v0)))
                }
                Some(v7) => {
                    let v8: Bytes = match v7.clone() {
                        Adt27::C0(h969, _, _, _) => {
                            h969
                        }
                    };
                    let v9: Adt16 = match v7.clone() {
                        Adt27::C0(_, h970, _, _) => {
                            h970
                        }
                    };
                    let v10: Adt16 = match v7.clone() {
                        Adt27::C0(_, _, h971, _) => {
                            h971
                        }
                    };
                    let v11: List<(Bytes, Bytes)> = match v7 {
                        Adt27::C0(_, _, _, h972) => {
                            h972
                        }
                    };
                    if f35(v8.clone(), f8(Str::lit("union"))) {
                        Ok(f217(f218(f8(Str::lit("a union is not translated, because a literal can be valid for more than one member type, so no LexLean inductive keeps its members disjoint (XSD 1.1 Part 2, section 2.4.1.3)")), v0)))
                    } else {
                        if f35(v8, f8(Str::lit("list"))) {
                            let m973 = f93(v9, f8(Str::lit("itemType")));
                            match m973 {
                                None => {
                                    Ok(f217(f218(f8(Str::lit("a list with an anonymous item type is not translated yet")), v0)))
                                }
                                Some(v12) => {
                                    let m974 = f219(v11, v3, v12)?;
                                    match m974 {
                                        Ok(v14) => {
                                            if f147(match v14.clone() {
                                                Adt22::C0(h975, _) => {
                                                    h975
                                                }
                                            }, f8(Str::lit("(List "))) {
                                                Ok(f217(f218(f8(Str::lit("the item type of a list must not itself be a list (XSD 1.1 Part 2, section 4.1.5)")), v0)))
                                            } else {
                                                Ok(Ok::<Adt22, Bytes>(Adt22::C0(f203(v4, f7(List::cons(f8(Str::lit("(List ")), List::cons(match v14.clone() {
                                                    Adt22::C0(h976, _) => {
                                                        h976
                                                    }
                                                }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))), match v14 {
                                                    Adt22::C0(_, h977) => {
                                                        h977
                                                    }
                                                })))
                                            }
                                        }
                                        Err(v13) => {
                                            Ok(f217(f218(v13, v0)))
                                        }
                                    }
                                }
                            }
                        } else {
                            let m978 = f93(v9, f8(Str::lit("base")));
                            match m978 {
                                None => {
                                    Ok(f217(f218(f8(Str::lit("a restriction with an anonymous base type is not translated yet")), v0)))
                                }
                                Some(v15) => {
                                    let m979 = f219(v11.clone(), v3, v15)?;
                                    match m979 {
                                        Ok(v17) => {
                                            let m980 = f220(v10, v11)?;
                                            match m980 {
                                                Ok(v19) => {
                                                    let v20: List<Adt19> = f221(v19, List::<Bytes>::nil())?;
                                                    let m981 = v20.clone();
                                                    match m981.uncons() {
                                                        None => {
                                                            Ok(Ok::<Adt22, Bytes>(Adt22::C0(f203(v4, match v17.clone() {
                                                                Adt22::C0(h982, _) => {
                                                                    h982
                                                                }
                                                            }), match v17 {
                                                                Adt22::C0(_, h983) => {
                                                                    h983
                                                                }
                                                            })))
                                                        }
                                                        Some((_, _)) => {
                                                            let m984 = f195(f190(f222(v20.clone()))?, v20, List::<Bytes>::nil())?;
                                                            match m984 {
                                                                Ok(v24) => {
                                                                    Ok(Ok::<Adt22, Bytes>(Adt22::C0(f197(v4, v24), List::<Bytes>::nil())))
                                                                }
                                                                Err(v23) => {
                                                                    Ok(f217(f218(v23, v0)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Err(v18) => {
                                                    Ok(f217(f218(v18, v0)))
                                                }
                                            }
                                        }
                                        Err(v16) => {
                                            Ok(f217(f218(v16, v0)))
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
            Ok(f217(f218(v5, v0)))
        }
    }
}

pub fn f154(v0: Bytes) -> R<Bytes> {
    f223(v0.clone(), 0u64, {
        let a985 = v0;
        length_bytes(a985)
    }, true, Bytes::lit(&[]))
}

pub fn f155(p988: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m987 = p988;
    {
        let mut m987 = m987;
        loop {
            let step = {
                let v0 = m987;
                let m986 = v0;
                match m986.uncons() {
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
                Ok(next) => { m987 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f156(v0: Bytes) -> Result<List<Bytes>, Bytes> {
    Err::<List<Bytes>, Bytes>(v0)
}

pub fn f157(v0: List<Bytes>, v1: Option<Bytes>) -> R<List<(Bytes, Bytes)>> {
    let m989 = v0;
    match m989.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, v3)) => {
            if f35(v2.clone(), f8(Str::lit("[[authority]]"))) {
                f157(v3, None::<Bytes>)
            } else {
                let m990 = f224(v2.clone(), f8(Str::lit("url = ")))?;
                match m990 {
                    None => {
                        let m991 = f224(v2, f8(Str::lit("artifact = ")))?;
                        match m991 {
                            None => {
                                f157(v3, v1)
                            }
                            Some(v5) => {
                                let m992 = v1.clone();
                                match m992 {
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
        let a993 = v0.clone();
        length_bytes(a993)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h994, _) = f43(Fn0::F225(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h994
    };
    let v3: u64 = {
        let (h995, _) = {
            let (_, h996) = v2.clone();
            h996
        };
        h995
    };
    let v4: List<Bytes> = {
        let (_, h997) = {
            let (_, h998) = v2;
            h998
        };
        h997
    };
    let a999 = v3;
    let a1000 = v1;
    if nat_lt(a999, a1000) {
        Ok(f45(List::cons(f32(v0, v3, {
            let a1001 = v1;
            let a1002 = v3;
            nat_sub(a1001, a1002)
        }), v4)))
    } else {
        Ok(f45(v4))
    }
}

pub fn f159(v0: List<Adt18>) -> R<Result<(u64, Bytes), Bytes>> {
    let m1003 = f226(v0.clone())?;
    match m1003 {
        Ok(v2) => {
            let v3: Adt23 = Adt23::C0(v2.clone(), f227(v0.clone()));
            let m1004 = f228(f229(v0)?, v3, v2)?;
            match m1004 {
                Ok(v5) => {
                    let m1005 = f230(f231(v5), List::<Adt26>::nil());
                    match m1005 {
                        Ok(v7) => {
                            let v8: List<Bytes> = f232(v7.clone());
                            let m1006 = f233(v7.clone(), v8.clone());
                            match m1006 {
                                None => {
                                    let v10: List<(Bytes, List<Bytes>)> = f234(v7.clone(), v8.clone(), f235(v7.clone())?);
                                    let m1012 = f236({
                                        let a1010 = {
                                            let a1009 = v8.clone();
                                            length_list(a1009)
                                        };
                                        let a1011 = 1u64;
                                        nat_add(a1010, a1011)?
                                    }, v7.clone(), f237(v7, f238(v10, v8.clone(), v8, List::<Bytes>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil());
                                    match m1012 {
                                        Ok(v12) => {
                                            Ok(Ok::<(u64, Bytes), Bytes>(({
                                                let a1013 = v12.clone();
                                                length_list(a1013)
                                            }, f75(v12, f8(Str::lit("\u{a}"))))))
                                        }
                                        Err(v11) => {
                                            Ok(Err::<(u64, Bytes), Bytes>(v11))
                                        }
                                    }
                                }
                                Some(v9) => {
                                    Ok(Err::<(u64, Bytes), Bytes>(f163(f8(Str::lit("the type ")), {
                                        let (h1007, _) = v9;
                                        h1007
                                    }, f163(f8(Str::lit(" uses ")), {
                                        let (_, h1008) = v9;
                                        h1008
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

pub fn f160(p1016: List<Adt18>, p1017: List<Adt18>) -> List<Adt18> {
    let m1015 = (p1016, p1017);
    {
        let mut m1015 = m1015;
        loop {
            let step = {
                let (v0, v1) = m1015;
                let m1014 = v0;
                match m1014.uncons() {
                    None => {
                        Err::<(List<Adt18>, List<Adt18>), List<Adt18>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt18>, List<Adt18>), List<Adt18>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1015 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f161(v0: List<Adt18>) -> List<Bytes> {
    let m1018 = v0;
    match m1018.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt18::C0(h1019, _, _, _, _, _) => {
                    h1019
                }
            }, f161(v2))
        }
    }
}

pub fn f162(p1024: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m1023 = p1024;
    {
        let mut m1023 = m1023;
        loop {
            let step = {
                let v0 = m1023;
                let m1020 = v0;
                match m1020.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h1021, _) = v2;
                            h1021
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h1022) = v2;
                                h1022
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
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

pub fn f163(v0: Bytes, v1: Bytes, v2: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(v1, List::cons(v2, List::<Bytes>::nil()))))
}

pub fn f164(v0: Bytes, v1: Bytes, v2: Option<Bytes>) -> R<Result<Adt18, Bytes>> {
    let m1025 = f53(v1.clone())?;
    match m1025 {
        Ok(v4) => {
            let m1026 = v4;
            match m1026 {
                Adt16::C0(v5, r1031, r1032) => {
                    let v6 = (*r1031).clone();
                    let v7 = (*r1032).clone();
                    let m1027 = f90(v6.clone(), List::<(Bytes, Bytes)>::nil());
                    match m1027 {
                        Ok(v9) => {
                            let m1028 = f91(v9.clone(), v5)?;
                            match m1028 {
                                Ok(v11) => {
                                    if f92(v11, Str::lit("schema")) {
                                        Ok(Ok::<Adt18, Bytes>(Adt18::C0(v0, v2, {
                                            let m1029 = f93(v6, f8(Str::lit("targetNamespace")));
                                            match m1029 {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some(v12) => {
                                                    v12
                                                }
                                            }
                                        }, v9, v7, {
                                            let a1030 = v1;
                                            length_bytes(a1030)
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

pub fn f165(p1037: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1036 = p1037;
    {
        let mut m1036 = m1036;
        loop {
            let step = {
                let v0 = m1036;
                let m1033 = v0;
                match m1033.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (_, h1034) = v2;
                            h1034
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (h1035, _) = v2;
                                h1035
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1036 = next; }
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
            Adt18::C0(_, _, h1038, _, _, _) => {
                h1038
            }
        }, v2.clone()) {
            None::<Bytes>
        } else {
            Some(f7(List::cons(match v0.clone() {
                Adt18::C0(h1039, _, _, _, _, _) => {
                    h1039
                }
            }, List::cons(f8(Str::lit(" has the target namespace ")), List::cons(match v0 {
                Adt18::C0(_, _, h1040, _, _, _) => {
                    h1040
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
    let m1041 = v0;
    match m1041 {
        Adt16::C0(_, _, _) => {
            Ok(Ok::<List<Adt25>, Bytes>(List::<Adt25>::nil()))
        }
        Adt16::C1(_, _, _) => {
            Ok(Ok::<List<Adt25>, Bytes>(List::<Adt25>::nil()))
        }
        Adt16::C2(_) => {
            Ok(Ok::<List<Adt25>, Bytes>(List::<Adt25>::nil()))
        }
        Adt16::C3(r1052, r1053) => {
            let v2 = (*r1052).clone();
            let v3 = (*r1053).clone();
            let m1042 = f167(v3, v1.clone())?;
            match m1042 {
                Ok(v5) => {
                    let m1043 = v2;
                    match m1043 {
                        Adt16::C0(v6, r1050, r1051) => {
                            let v7 = (*r1050).clone();
                            let v8 = (*r1051).clone();
                            let m1044 = f90(v7.clone(), v1);
                            match m1044 {
                                Ok(v10) => {
                                    let m1045 = f91(v10.clone(), v6)?;
                                    match m1045 {
                                        Ok(v12) => {
                                            if if f35({
                                                let (h1046, _) = v12.clone();
                                                h1046
                                            }, f151()) {
                                                f239(f35({
                                                    let (_, h1047) = v12.clone();
                                                    h1047
                                                }, f8(Str::lit("annotation"))))
                                            } else {
                                                false
                                            } {
                                                Ok(Ok::<List<Adt25>, Bytes>(List::cons(Adt25::C0({
                                                    let (h1048, _) = v12;
                                                    h1048
                                                }, {
                                                    let (_, h1049) = v12;
                                                    h1049
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
    let m1054 = v0;
    match m1054.uncons() {
        None => {
            Ok(Ok::<List<(Bytes, (Bytes, Bytes))>, Bytes>(List::<(Bytes, (Bytes, Bytes))>::nil()))
        }
        Some((v3, v4)) => {
            let m1055 = f168(v4, v1.clone(), v2.clone())?;
            match m1055 {
                Ok(v6) => {
                    if if f240(v3.clone(), Str::lit("redefine")) {
                        true
                    } else {
                        if f240(v3.clone(), Str::lit("override")) {
                            true
                        } else {
                            f240(v3.clone(), Str::lit("defaultOpenContent"))
                        }
                    } {
                        Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                            Adt18::C0(h1056, _, _, _, _, _) => {
                                h1056
                            }
                        }, List::cons(f8(Str::lit(": xs:")), List::cons(match v3 {
                            Adt25::C0(_, h1057, _, _, _) => {
                                h1057
                            }
                        }, List::cons(f8(Str::lit(" is not translated, because it changes components that other schema documents define (XSD 1.1 Part 1, sections 4.2.4, 4.2.5 and 3.4.2.3)")), List::<Bytes>::nil())))))))
                    } else {
                        if if f240(v3.clone(), Str::lit("import")) {
                            true
                        } else {
                            f240(v3.clone(), Str::lit("include"))
                        } {
                            let m1058 = f241(v3.clone(), Str::lit("schemaLocation"));
                            match m1058 {
                                None => {
                                    Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                                        Adt18::C0(h1059, _, _, _, _, _) => {
                                            h1059
                                        }
                                    }, List::cons(f8(Str::lit(": an xs:")), List::cons(match v3 {
                                        Adt25::C0(_, h1060, _, _, _) => {
                                            h1060
                                        }
                                    }, List::cons(f8(Str::lit(" with no schemaLocation names no document whose components could be read (XSD 1.1 Part 1, section 4.2.6.2)")), List::<Bytes>::nil())))))))
                                }
                                Some(v7) => {
                                    let m1062 = f242(match v1.clone() {
                                        Adt18::C0(_, h1061, _, _, _, _) => {
                                            h1061
                                        }
                                    }, v7)?;
                                    match m1062 {
                                        Ok(v9) => {
                                            let m1064 = f243(v2, v9.clone());
                                            match m1064 {
                                                None => {
                                                    Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                                                        Adt18::C0(h1065, _, _, _, _, _) => {
                                                            h1065
                                                        }
                                                    }, List::cons(f8(Str::lit(": the schema document ")), List::cons(v9, List::cons(f8(Str::lit(" is not a bound authority, so its components cannot be read; stage its binding first (XSD 1.1 Part 1, section 4.2.6.2)")), List::<Bytes>::nil())))))))
                                                }
                                                Some(v10) => {
                                                    Ok(Ok::<List<(Bytes, (Bytes, Bytes))>, Bytes>(List::cons((v10, (match v3.clone() {
                                                        Adt25::C0(_, h1066, _, _, _) => {
                                                            h1066
                                                        }
                                                    }, if f240(v3.clone(), Str::lit("include")) {
                                                        match v1 {
                                                            Adt18::C0(_, _, h1067, _, _, _) => {
                                                                h1067
                                                            }
                                                        }
                                                    } else {
                                                        f244(v3, Str::lit("namespace"), Str::lit(""))
                                                    })), v6)))
                                                }
                                            }
                                        }
                                        Err(v8) => {
                                            Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f163(match v1 {
                                                Adt18::C0(h1063, _, _, _, _, _) => {
                                                    h1063
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
    let m1068 = v0;
    match m1068.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f169(v3, v1))
        }
    }
}

pub fn f170(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f245(v0, v1)
}

pub fn f171(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f246(v0, v1)
}

pub fn f172(v0: u64) -> bool {
    let a1069 = v0;
    let a1070 = 32u64;
    if nat_eq(a1069, a1070) {
        true
    } else {
        let a1071 = v0;
        let a1072 = 9u64;
        nat_eq(a1071, a1072)
    }
}

pub fn f173(v0: Bytes, v1: u64) -> R<u64> {
    f139(v0, v1)
}

pub fn f174(v0: u64) -> bool {
    let a1073 = v0;
    let a1074 = 10u64;
    if nat_eq(a1073, a1074) {
        true
    } else {
        let a1075 = v0;
        let a1076 = 256u64;
        nat_eq(a1075, a1076)
    }
}

pub fn f175(v0: Bytes, v1: u64) -> R<u64> {
    let a1077 = v1;
    let a1078 = f247(v0, v1)?;
    Ok(nat_sub(a1077, a1078))
}

pub fn f176(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: List<Bytes>) -> R<Adt20> {
    let m1079 = v3;
    if m1079 == 0 {
        Ok(Adt20::C1(f83(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v8 = m1079 - 1;
        let a1080 = v4;
        let a1081 = 0u64;
        if nat_eq(a1080, a1081) {
            let v9: u64 = f248(v0.clone(), v1)?;
            let v10: u64 = {
                let a1082 = v9;
                let a1083 = v1;
                nat_sub(a1082, a1083)
            };
            let a1084 = f173(v0.clone(), v9)?;
            let a1085 = 9u64;
            if nat_eq(a1084, a1085) {
                let v11: u64 = f249(v0.clone(), v9)?;
                if if f250(v0.clone(), v11)? {
                    true
                } else {
                    f251(v0.clone(), v11)?
                } {
                    Ok(Adt20::C1(f83(Str::lit("a tab cannot indent a block node (YAML 1.2.2 [63])"), v11)?))
                } else {
                    let m1086 = f252(v0.clone(), v11)?;
                    match m1086 {
                        None => {
                            f253(v0, v11, v2)
                        }
                        Some(v12) => {
                            Ok(Adt20::C1(v12))
                        }
                    }
                }
            } else {
                if f250(v0.clone(), v9)? {
                    Ok(f254(true, f176(v0, v9, v2, v8, 2u64, false, v10, List::<Bytes>::nil())?))
                } else {
                    let m1087 = f252(v0.clone(), v9)?;
                    match m1087 {
                        None => {
                            if f251(v0.clone(), v9)? {
                                Ok(f254(false, f176(v0, v9, v2, v8, 3u64, false, v10, List::<Bytes>::nil())?))
                            } else {
                                f253(v0, v9, v2)
                            }
                        }
                        Some(v13) => {
                            Ok(Adt20::C1(v13))
                        }
                    }
                }
            }
        } else {
            let a1088 = v4;
            let a1089 = 1u64;
            if nat_eq(a1088, a1089) {
                let v14: u64 = f249(v0.clone(), v1)?;
                let v15: u64 = f173(v0.clone(), v14)?;
                if if f174(v15) {
                    true
                } else {
                    let a1090 = v15;
                    let a1091 = 35u64;
                    if nat_eq(a1090, a1091) {
                        let a1092 = v1;
                        let a1093 = v14;
                        nat_lt(a1092, a1093)
                    } else {
                        false
                    }
                } {
                    let v16: u64 = f102(v0.clone(), f255(v0.clone(), v14)?)?;
                    let a1095 = {
                        let a1094 = v0.clone();
                        length_bytes(a1094)
                    };
                    let a1096 = v16;
                    if if nat_le(a1095, a1096) {
                        true
                    } else {
                        f103(v0.clone(), v16)?
                    } {
                        Ok(Adt20::C1(f256(v14)?))
                    } else {
                        let v17: u64 = {
                            let a1097 = f248(v0.clone(), v16)?;
                            let a1098 = v16;
                            nat_sub(a1097, a1098)
                        };
                        let a1099 = v2;
                        let a1100 = v17;
                        if nat_le(a1099, a1100) {
                            f176(v0, v16, v2, v8, 0u64, false, 0u64, List::<Bytes>::nil())
                        } else {
                            if if if f178(v5) {
                                let a1101 = v17;
                                let a1102 = v6;
                                nat_eq(a1101, a1102)
                            } else {
                                false
                            } {
                                f250(v0.clone(), {
                                    let a1103 = v16;
                                    let a1104 = v17;
                                    nat_add(a1103, a1104)?
                                })?
                            } else {
                                false
                            } {
                                Ok(f254(true, f176(v0, {
                                    let a1105 = v16;
                                    let a1106 = v17;
                                    nat_add(a1105, a1106)?
                                }, v2, v8, 2u64, false, v17, List::<Bytes>::nil())?))
                            } else {
                                Ok(Adt20::C1(f256(v14)?))
                            }
                        }
                    }
                } else {
                    if if v5 {
                        f250(v0.clone(), v14)?
                    } else {
                        false
                    } {
                        Ok(f254(true, f176(v0.clone(), v14, v2, v8, 2u64, false, f175(v0, v14)?, List::<Bytes>::nil())?))
                    } else {
                        let m1107 = f252(v0.clone(), v14)?;
                        match m1107 {
                            None => {
                                if if v5 {
                                    f251(v0.clone(), v14)?
                                } else {
                                    false
                                } {
                                    Ok(f254(false, f176(v0.clone(), v14, v2, v8, 3u64, false, f175(v0, v14)?, List::<Bytes>::nil())?))
                                } else {
                                    f253(v0, v14, v2)
                                }
                            }
                            Some(v18) => {
                                Ok(Adt20::C1(v18))
                            }
                        }
                    }
                }
            } else {
                let a1108 = v4;
                let a1109 = 2u64;
                if nat_eq(a1108, a1109) {
                    let m1114 = f176(v0.clone(), {
                        let a1110 = v1;
                        let a1111 = 1u64;
                        nat_add(a1110, a1111)?
                    }, {
                        let a1112 = v6;
                        let a1113 = 1u64;
                        nat_add(a1112, a1113)?
                    }, v8, 1u64, true, v6, List::<Bytes>::nil())?;
                    match m1114 {
                        Adt20::C0(v20, v21) => {
                            let v22: u64 = f102(v0.clone(), v21)?;
                            let v23: u64 = {
                                let a1115 = f248(v0.clone(), v22)?;
                                let a1116 = v22;
                                nat_sub(a1115, a1116)
                            };
                            let a1118 = {
                                let a1117 = v0.clone();
                                length_bytes(a1117)
                            };
                            let a1119 = v22;
                            if nat_le(a1118, a1119) {
                                Ok(Adt20::C0(Adt19::C9(std::rc::Rc::new(v20), std::rc::Rc::new(Adt19::C8)), v22))
                            } else {
                                let a1120 = v23;
                                let a1121 = v6;
                                if if nat_eq(a1120, a1121) {
                                    f250(v0.clone(), {
                                        let a1122 = v22;
                                        let a1123 = v23;
                                        nat_add(a1122, a1123)?
                                    })?
                                } else {
                                    false
                                } {
                                    let m1126 = f176(v0, {
                                        let a1124 = v22;
                                        let a1125 = v23;
                                        nat_add(a1124, a1125)?
                                    }, v2, v8, 2u64, false, v6, List::<Bytes>::nil())?;
                                    match m1126 {
                                        Adt20::C0(v25, v26) => {
                                            Ok(Adt20::C0(Adt19::C9(std::rc::Rc::new(v20), std::rc::Rc::new(v25)), v26))
                                        }
                                        Adt20::C1(v24) => {
                                            Ok(Adt20::C1(v24))
                                        }
                                    }
                                } else {
                                    let a1127 = v6;
                                    let a1128 = v23;
                                    if nat_lt(a1127, a1128) {
                                        Ok(Adt20::C1(f83(Str::lit("a line is indented more than its block sequence entry allows (YAML 1.2.2 [183])"), {
                                            let a1129 = v22;
                                            let a1130 = v23;
                                            nat_add(a1129, a1130)?
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
                    let m1131 = f252(v0.clone(), v1)?;
                    match m1131 {
                        None => {
                            let m1132 = f257(v0.clone(), v1)?;
                            match m1132 {
                                None => {
                                    Ok(Adt20::C1(f83(Str::lit("a block mapping entry must be an implicit key and : (YAML 1.2.2 [188])"), v1)?))
                                }
                                Some(v28) => {
                                    let v29: Bytes = f258(v0.clone(), v1)?;
                                    if f211(v29.clone(), v7.clone()) {
                                        Ok(Adt20::C1(f83(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                    } else {
                                        let m1137 = f176(v0.clone(), {
                                            let a1133 = v28;
                                            let a1134 = 1u64;
                                            nat_add(a1133, a1134)?
                                        }, {
                                            let a1135 = v6;
                                            let a1136 = 1u64;
                                            nat_add(a1135, a1136)?
                                        }, v8, 1u64, false, v6, List::<Bytes>::nil())?;
                                        match m1137 {
                                            Adt20::C0(v31, v32) => {
                                                let v33: u64 = f102(v0.clone(), v32)?;
                                                let v34: u64 = {
                                                    let a1138 = f248(v0.clone(), v33)?;
                                                    let a1139 = v33;
                                                    nat_sub(a1138, a1139)
                                                };
                                                let a1141 = {
                                                    let a1140 = v0.clone();
                                                    length_bytes(a1140)
                                                };
                                                let a1142 = v33;
                                                if nat_le(a1141, a1142) {
                                                    Ok(Adt20::C0(Adt19::C10(v29, std::rc::Rc::new(v31), std::rc::Rc::new(Adt19::C8)), v33))
                                                } else {
                                                    let a1143 = v34;
                                                    let a1144 = v6;
                                                    if if if nat_eq(a1143, a1144) {
                                                        f178(f250(v0.clone(), {
                                                            let a1145 = v33;
                                                            let a1146 = v34;
                                                            nat_add(a1145, a1146)?
                                                        })?)
                                                    } else {
                                                        false
                                                    } {
                                                        f178(f103(v0.clone(), v33)?)
                                                    } else {
                                                        false
                                                    } {
                                                        let m1149 = f176(v0, {
                                                            let a1147 = v33;
                                                            let a1148 = v34;
                                                            nat_add(a1147, a1148)?
                                                        }, v2, v8, 3u64, false, v6, List::cons(v29.clone(), v7))?;
                                                        match m1149 {
                                                            Adt20::C0(v36, v37) => {
                                                                Ok(Adt20::C0(Adt19::C10(v29, std::rc::Rc::new(v31), std::rc::Rc::new(v36)), v37))
                                                            }
                                                            Adt20::C1(v35) => {
                                                                Ok(Adt20::C1(v35))
                                                            }
                                                        }
                                                    } else {
                                                        let a1150 = v6;
                                                        let a1151 = v34;
                                                        if nat_lt(a1150, a1151) {
                                                            Ok(Adt20::C1(f83(Str::lit("a line is indented more than its block mapping entry allows (YAML 1.2.2 [187])"), {
                                                                let a1152 = v33;
                                                                let a1153 = v34;
                                                                nat_add(a1152, a1153)?
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

pub fn f177(v0: Bytes, p1162: u64, p1163: u64) -> R<Result<u64, Bytes>> {
    let m1161 = (p1162, p1163);
    {
        let mut m1161 = m1161;
        loop {
            let step = {
                let (v1, v2) = m1161;
                let m1154 = v2;
                if m1154 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Ok::<u64, Bytes>(v1)))
                } else {
                    let v3 = m1154 - 1;
                    let a1156 = v1;
                    let a1157 = {
                        let a1155 = v0.clone();
                        length_bytes(a1155)
                    };
                    if if nat_lt(a1156, a1157) {
                        if f86(v0.clone(), v1, Str::lit("...")) {
                            f103(v0.clone(), v1)?
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        let m1160 = f259(v0.clone(), {
                            let a1158 = v1;
                            let a1159 = 3u64;
                            nat_add(a1158, a1159)?
                        })?;
                        match m1160 {
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
                Ok(next) => { m1161 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f178(v0: bool) -> bool {
    !v0
}

pub fn f179(v0: Adt19) -> R<Bytes> {
    let m1164 = v0;
    match m1164 {
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
            f260(v4, v5)
        }
        Adt19::C4(v6, v7, v8) => {
            f261(v6, v7, v8)
        }
        Adt19::C5(v9) => {
            f262(v9)
        }
        Adt19::C6(r1165) => {
            let v10 = (*r1165).clone();
            Ok(f263(f264(v10)?))
        }
        Adt19::C7(r1166) => {
            let v11 = (*r1166).clone();
            f265(f266(v11)?)
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

pub fn f180(p1221: u64, v1: Bytes, p1222: u64, p1223: List<Adt21>) -> R<Result<List<Adt21>, Bytes>> {
    let m1220 = (p1221, (p1222, p1223));
    {
        let mut m1220 = m1220;
        loop {
            let step = {
                let (v0, (v2, v3)) = m1220;
                let m1167 = v0;
                if m1167 == 0 {
                    Err::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>(Ok(Err::<List<Adt21>, Bytes>(f267(Str::lit("the input has more tokens than bytes"), v2)?)))
                } else {
                    let v4 = m1167 - 1;
                    let v5: u64 = f87(v1.clone(), v2)?;
                    let a1168 = v5;
                    let a1169 = 256u64;
                    if nat_eq(a1168, a1169) {
                        Err::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>(Ok(Ok::<List<Adt21>, Bytes>(f268(v3, List::<Adt21>::nil()))))
                    } else {
                        if f132(v5)? {
                            Ok::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>((v4, ({
                                let a1170 = v2;
                                let a1171 = 1u64;
                                nat_add(a1170, a1171)?
                            }, v3)))
                        } else {
                            if f86(v1.clone(), v2, Str::lit("//")) {
                                let m1172 = f133(v1.clone(), Bytes::lit(&[0x0a]), v2)?;
                                match m1172 {
                                    None => {
                                        Err::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>(Ok(Ok::<List<Adt21>, Bytes>(f268(v3, List::<Adt21>::nil()))))
                                    }
                                    Some(v6) => {
                                        Ok::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>((v4, ({
                                            let a1173 = v6;
                                            let a1174 = 1u64;
                                            nat_add(a1173, a1174)?
                                        }, v3)))
                                    }
                                }
                            } else {
                                if f86(v1.clone(), v2, Str::lit("/*")) {
                                    let m1178 = f133(v1.clone(), {
                                        let a1175 = Str::lit("*/");
                                        utf8_encode(a1175)
                                    }, {
                                        let a1176 = v2;
                                        let a1177 = 2u64;
                                        nat_add(a1176, a1177)?
                                    })?;
                                    match m1178 {
                                        None => {
                                            Err::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>(Ok(Err::<List<Adt21>, Bytes>(f267(Str::lit("a comment is not closed (Web IDL, IDL grammar: comment)"), v2)?)))
                                        }
                                        Some(v7) => {
                                            Ok::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>((v4, ({
                                                let a1179 = v7;
                                                let a1180 = 2u64;
                                                nat_add(a1179, a1180)?
                                            }, v3)))
                                        }
                                    }
                                } else {
                                    let a1181 = v5;
                                    let a1182 = 34u64;
                                    if nat_eq(a1181, a1182) {
                                        let m1185 = f133(v1.clone(), Bytes::lit(&[0x22]), {
                                            let a1183 = v2;
                                            let a1184 = 1u64;
                                            nat_add(a1183, a1184)?
                                        })?;
                                        match m1185 {
                                            None => {
                                                Err::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>(Ok(Err::<List<Adt21>, Bytes>(f267(Str::lit("a string is not closed (Web IDL, IDL grammar: string)"), v2)?)))
                                            }
                                            Some(v8) => {
                                                Ok::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>((v4, ({
                                                    let a1186 = v8;
                                                    let a1187 = 1u64;
                                                    nat_add(a1186, a1187)?
                                                }, List::cons(Adt21::C0(3u64, f32(v1.clone(), {
                                                    let a1188 = v2;
                                                    let a1189 = 1u64;
                                                    nat_add(a1188, a1189)?
                                                }, {
                                                    let a1192 = v8;
                                                    let a1193 = {
                                                        let a1190 = v2;
                                                        let a1191 = 1u64;
                                                        nat_add(a1190, a1191)?
                                                    };
                                                    nat_sub(a1192, a1193)
                                                })), v3))))
                                            }
                                        }
                                    } else {
                                        let v9: u64 = {
                                            let a1194 = v5;
                                            let a1195 = 95u64;
                                            if if nat_eq(a1194, a1195) {
                                                true
                                            } else {
                                                let a1196 = v5;
                                                let a1197 = 45u64;
                                                nat_eq(a1196, a1197)
                                            } {
                                                let a1198 = v2;
                                                let a1199 = 1u64;
                                                nat_add(a1198, a1199)?
                                            } else {
                                                v2
                                            }
                                        };
                                        if f269(f87(v1.clone(), v9)?) {
                                            let v10: u64 = f270(v1.clone(), 3u64, {
                                                let a1200 = v9;
                                                let a1201 = 1u64;
                                                nat_add(a1200, a1201)?
                                            })?;
                                            Ok::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>((v4, (v10, List::cons(Adt21::C0(0u64, f32(v1.clone(), v2, {
                                                let a1202 = v10;
                                                let a1203 = v2;
                                                nat_sub(a1202, a1203)
                                            })), v3))))
                                        } else {
                                            let v11: u64 = {
                                                let a1204 = v5;
                                                let a1205 = 45u64;
                                                if nat_eq(a1204, a1205) {
                                                    let a1206 = v2;
                                                    let a1207 = 1u64;
                                                    nat_add(a1206, a1207)?
                                                } else {
                                                    v2
                                                }
                                            };
                                            let v12: u64 = f271(v1.clone(), v11)?;
                                            let v13: u64 = f272(v1.clone(), v11)?;
                                            let a1208 = v12;
                                            let a1209 = v13;
                                            if nat_lt(a1208, a1209) {
                                                Ok::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>((v4, (v13, List::cons(Adt21::C0(2u64, f32(v1.clone(), v2, {
                                                    let a1210 = v13;
                                                    let a1211 = v2;
                                                    nat_sub(a1210, a1211)
                                                })), v3))))
                                            } else {
                                                let a1212 = v11;
                                                let a1213 = v12;
                                                if nat_lt(a1212, a1213) {
                                                    Ok::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>((v4, (v12, List::cons(Adt21::C0(1u64, f32(v1.clone(), v2, {
                                                        let a1214 = v12;
                                                        let a1215 = v2;
                                                        nat_sub(a1214, a1215)
                                                    })), v3))))
                                                } else {
                                                    if f86(v1.clone(), v2, Str::lit("...")) {
                                                        Ok::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>((v4, ({
                                                            let a1216 = v2;
                                                            let a1217 = 3u64;
                                                            nat_add(a1216, a1217)?
                                                        }, List::cons(Adt21::C0(4u64, f8(Str::lit("..."))), v3))))
                                                    } else {
                                                        Ok::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>((v4, ({
                                                            let a1218 = v2;
                                                            let a1219 = 1u64;
                                                            nat_add(a1218, a1219)?
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
                Ok(next) => { m1220 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f181(p1226: List<Adt12>, p1227: List<Adt12>) -> List<Adt12> {
    let m1225 = (p1226, p1227);
    {
        let mut m1225 = m1225;
        loop {
            let step = {
                let (v0, v1) = m1225;
                let m1224 = v0;
                match m1224.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1225 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f182(v0: List<Adt21>) -> R<Result<List<Adt21>, Bytes>> {
    if f273(v0.clone(), Str::lit("[")) {
        Ok(f274({
            let a1229 = {
                let a1228 = v0.clone();
                length_list(a1228)
            };
            let a1230 = 1u64;
            nat_add(a1229, a1230)?
        }, f275(v0), List::cons(f8(Str::lit("]")), List::<Bytes>::nil()), false))
    } else {
        Ok(Ok::<List<Adt21>, Bytes>(v0))
    }
}

pub fn f183(v0: u64, v1: List<Adt21>) -> R<Result<(Adt12, List<Adt21>), Bytes>> {
    if f273(v1.clone(), Str::lit("interface")) {
        if f273(f275(v1.clone()), Str::lit("mixin")) {
            f276(v0, 1u64, f275(f275(v1)), false)
        } else {
            f276(v0, 0u64, f275(v1), true)
        }
    } else {
        if f273(v1.clone(), Str::lit("partial")) {
            let v2: List<Adt21> = f275(v1);
            if f273(v2.clone(), Str::lit("interface")) {
                if f273(f275(v2.clone()), Str::lit("mixin")) {
                    f276(v0, 6u64, f275(f275(v2)), false)
                } else {
                    f276(v0, 5u64, f275(v2), false)
                }
            } else {
                if f273(v2.clone(), Str::lit("dictionary")) {
                    f276(v0, 7u64, f275(v2), false)
                } else {
                    if f273(v2.clone(), Str::lit("namespace")) {
                        f277(f275(v2), f8(Str::lit("a namespace holds operations and attributes of no object, so it is behavior, not data (Web IDL 2.6)")))
                    } else {
                        Ok(Err::<(Adt12, List<Adt21>), Bytes>(f278(Str::lit("interface, dictionary or namespace"), v2)))
                    }
                }
            }
        } else {
            if f273(v1.clone(), Str::lit("dictionary")) {
                f276(v0, 2u64, f275(v1), true)
            } else {
                if f273(v1.clone(), Str::lit("enum")) {
                    let m1231 = f279(f275(v1));
                    match m1231 {
                        Ok(v4) => {
                            let m1232 = f280(f281(v4.clone()), Str::lit("{"));
                            match m1232 {
                                Ok(v6) => {
                                    let m1233 = f282(v0, v6, List::<Bytes>::nil());
                                    match m1233 {
                                        Ok(v8) => {
                                            Ok(Ok::<(Adt12, List<Adt21>), Bytes>((Adt12::C0(3u64, f283(v4), Bytes::lit(&[]), List::<Adt13>::nil(), {
                                                let (h1234, _) = v8;
                                                h1234
                                            }, f284(Bytes::lit(&[]))), {
                                                let (_, h1235) = v8;
                                                h1235
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
                    if f273(v1.clone(), Str::lit("typedef")) {
                        let m1236 = f285(v0, f275(v1), 0u64)?;
                        match m1236 {
                            Ok(v10) => {
                                let m1237 = f279(f286(v10.clone()));
                                match m1237 {
                                    Ok(v12) => {
                                        let m1238 = f280(f281(v12.clone()), Str::lit(";"));
                                        match m1238 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt12, List<Adt21>), Bytes>((Adt12::C0(4u64, f283(v12), Bytes::lit(&[]), List::<Adt13>::nil(), List::<Bytes>::nil(), f287(v10)), v14)))
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
                        if f273(v1.clone(), Str::lit("callback")) {
                            if f273(f275(v1.clone()), Str::lit("interface")) {
                                f277(f275(f275(v1)), f8(Str::lit("a callback interface is implemented by user code, so it is behavior, not data (Web IDL 2.4)")))
                            } else {
                                f277(f275(v1), f8(Str::lit("a callback function is behavior, not data (Web IDL 2.10)")))
                            }
                        } else {
                            if f273(v1.clone(), Str::lit("namespace")) {
                                f277(f275(v1), f8(Str::lit("a namespace holds operations and attributes of no object, so it is behavior, not data (Web IDL 2.6)")))
                            } else {
                                let m1239 = f279(v1);
                                match m1239 {
                                    Ok(v16) => {
                                        let m1240 = f280(f281(v16.clone()), Str::lit("includes"));
                                        match m1240 {
                                            Ok(v18) => {
                                                let m1241 = f279(v18);
                                                match m1241 {
                                                    Ok(v20) => {
                                                        let m1242 = f280(f281(v20.clone()), Str::lit(";"));
                                                        match m1242 {
                                                            Ok(v22) => {
                                                                Ok(Ok::<(Adt12, List<Adt21>), Bytes>((Adt12::C0(8u64, f283(v16), f283(v20), List::<Adt13>::nil(), List::<Bytes>::nil(), f284(Bytes::lit(&[]))), v22)))
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

pub fn f184(p1246: List<Bytes>, v1: List<Adt15>) -> Bytes {
    let m1245 = p1246;
    {
        let mut m1245 = m1245;
        loop {
            let step = {
                let v0 = m1245;
                let m1243 = v0;
                match m1243.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1244 = f116(v1.clone(), v2.clone());
                        match m1244 {
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
                Ok(next) => { m1245 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f185(v0: Bytes) -> bool {
    if f115(v0.clone(), f288()) {
        true
    } else {
        if f115(v0.clone(), f289()) {
            true
        } else {
            if f115(v0.clone(), f290()) {
                true
            } else {
                if f115(v0.clone(), f291()) {
                    true
                } else {
                    if f115(v0.clone(), f292()) {
                        true
                    } else {
                        if f115(v0.clone(), f293()) {
                            true
                        } else {
                            f115(v0, f294())
                        }
                    }
                }
            }
        }
    }
}

pub fn f186(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("the derived name ")), List::cons(v0, List::cons(f8(Str::lit(" is a reserved token of Lean 4.32.2, which LexLean 0.4.3 refuses as a name (lexlean 0.4.3 src/backend/lean_tokens.rs, SPEC.md 17.8)")), List::<Bytes>::nil()))))
}

pub fn f187(p1252: List<Adt12>, v1: List<Adt12>, v2: Bytes) -> Option<Bytes> {
    let m1251 = p1252;
    {
        let mut m1251 = m1251;
        loop {
            let step = {
                let v0 = m1251;
                let m1247 = v0;
                match m1247.uncons() {
                    None => {
                        Err::<List<Adt12>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v3, v4)) => {
                        let a1248 = f112(v3.clone());
                        let a1249 = 8u64;
                        if if nat_eq(a1248, a1249) {
                            f35(f114(v3.clone()), v2.clone())
                        } else {
                            false
                        } {
                            let m1250 = f123(v1.clone(), 1u64, f125(v3.clone()));
                            match m1250 {
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
                Ok(next) => { m1251 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f188(v0: u64, v1: List<Adt12>, v2: Adt12) -> R<Result<List<Adt13>, Bytes>> {
    let m1253 = v0;
    if m1253 == 0 {
        Ok(Err::<List<Adt13>, Bytes>(f7(List::cons(f8(Str::lit("the inheritance of ")), List::cons(f114(v2), List::cons(f8(Str::lit(" is cyclic (Web IDL 2.2 and 2.7: an interface or dictionary must not inherit from itself)")), List::<Bytes>::nil()))))))
    } else {
        let v3 = m1253 - 1;
        let v4: List<Adt13> = {
            let a1254 = f112(v2.clone());
            let a1255 = 2u64;
            if nat_eq(a1254, a1255) {
                f295(f296(f297(v2.clone()), f298(v1.clone(), 7u64, f114(v2.clone()))))?
            } else {
                f296(f297(v2.clone()), f296(f298(v1.clone(), 5u64, f114(v2.clone())), f299(v1.clone(), v1.clone(), f114(v2.clone()))))
            }
        };
        if f35(f125(v2.clone()), Bytes::lit(&[])) {
            Ok(Ok::<List<Adt13>, Bytes>(v4))
        } else {
            let m1256 = f123(v1.clone(), f112(v2.clone()), f125(v2.clone()));
            match m1256 {
                None => {
                    Ok(Err::<List<Adt13>, Bytes>(f7(List::cons(f8(Str::lit("inherits ")), List::cons(f125(v2.clone()), List::cons({
                        let a1257 = f112(v2);
                        let a1258 = 2u64;
                        if nat_eq(a1257, a1258) {
                            f8(Str::lit(", which no input defines as a dictionary (Web IDL 2.7)"))
                        } else {
                            f8(Str::lit(", which no input defines as an interface (Web IDL 2.2)"))
                        }
                    }, List::<Bytes>::nil()))))))
                }
                Some(v5) => {
                    let m1259 = f188(v3, v1, v5)?;
                    match m1259 {
                        Ok(v7) => {
                            Ok(Ok::<List<Adt13>, Bytes>(f296(v7, v4)))
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

pub fn f189(v0: bool, v1: List<Adt13>) -> R<Result<(List<Bytes>, List<Bytes>), Bytes>> {
    let m1260 = v1;
    match m1260.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::<Bytes>::nil(), List::<Bytes>::nil())))
        }
        Some((v2, v3)) => {
            let v4: Adt14 = f300(v2.clone());
            if !f35(f199(v4.clone()), Bytes::lit(&[])) {
                Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons(f301(v2), List::cons(f8(Str::lit(": ")), List::cons(f199(v4), List::<Bytes>::nil())))))))
            } else {
                let m1261 = f302(v0, f301(v2.clone()))?;
                match m1261 {
                    None => {
                        Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons(f301(v2), List::cons(f8(Str::lit(" has no field name")), List::<Bytes>::nil()))))))
                    }
                    Some(v5) => {
                        if f185(v5.clone()) {
                            Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f186(v5)))
                        } else {
                            if f115(v5.clone(), f303(v0, f191(v3.clone()))?) {
                                Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("two of its members derive the field name ")), List::cons(v5, List::cons(f8(Str::lit(" (Web IDL 2.2 and 2.7: the members of one definition have distinct identifiers)")), List::<Bytes>::nil()))))))
                            } else {
                                let m1262 = f189(v0, v3)?;
                                match m1262 {
                                    Ok(v7) => {
                                        Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v5, List::cons(f8(Str::lit(", type: ")), List::cons({
                                            let a1263 = f304(v2);
                                            let a1264 = 1u64;
                                            if nat_eq(a1263, a1264) {
                                                f7(List::cons(f8(Str::lit("(Option ")), List::cons(f201(v4.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                            } else {
                                                f201(v4.clone())
                                            }
                                        }, List::cons(f8(Str::lit("}")), List::<Bytes>::nil())))))), {
                                            let (h1265, _) = v7;
                                            h1265
                                        }), f118(f202(v4), {
                                            let (_, h1266) = v7;
                                            h1266
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

pub fn f190(p1270: List<Bytes>) -> R<bool> {
    let m1269 = p1270;
    {
        let mut m1269 = m1269;
        loop {
            let step = {
                let v0 = m1269;
                let m1267 = v0;
                match m1267.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m1268 = f305(v1)?;
                        match m1268 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f115(v3, f303(false, v2.clone())?) {
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
                Ok(next) => { m1269 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f191(v0: List<Adt13>) -> List<Bytes> {
    let m1271 = v0;
    match m1271.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f301(v1), f191(v2))
        }
    }
}

pub fn f192(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}")), List::cons(f75(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f193(v0: Adt12) -> List<Bytes> {
    let m1272 = v0;
    match m1272 {
        Adt12::C0(_, _, _, _, v5, _) => {
            v5
        }
    }
}

pub fn f194(v0: bool, p1276: List<Bytes>) -> R<Option<Bytes>> {
    let m1275 = p1276;
    {
        let mut m1275 = m1275;
        loop {
            let step = {
                let v1 = m1275;
                let m1273 = v1;
                match m1273.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m1274 = f302(v0, v2)?;
                        match m1274 {
                            None => {
                                Ok::<List<Bytes>, R<Option<Bytes>>>(v3)
                            }
                            Some(v4) => {
                                if f185(v4.clone()) {
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
                Ok(next) => { m1275 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f195(v0: bool, v1: List<Adt19>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1277 = v1;
    match m1277.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m1278 = f306(v0, v3.clone())?;
            match m1278 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f105(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f115(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m1279 = f195(v0, v4, List::cons(v5.clone(), v2))?;
                        match m1279 {
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

pub fn f196(v0: List<Bytes>) -> List<Adt19> {
    let m1280 = v0;
    match m1280.uncons() {
        None => {
            List::<Adt19>::nil()
        }
        Some((v1, v2)) => {
            List::cons(Adt19::C5(v1), f196(v2))
        }
    }
}

pub fn f197(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f75(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f198(v0: Adt12) -> Adt14 {
    let m1281 = v0;
    match m1281 {
        Adt12::C0(_, _, _, _, _, v6) => {
            v6
        }
    }
}

pub fn f199(v0: Adt14) -> Bytes {
    let m1282 = v0;
    match m1282 {
        Adt14::C0(_, _, _, v4, _) => {
            v4
        }
    }
}

pub fn f200(v0: Adt14) -> u64 {
    let m1283 = v0;
    match m1283 {
        Adt14::C0(v1, _, _, _, _) => {
            v1
        }
    }
}

pub fn f201(v0: Adt14) -> Bytes {
    let m1284 = v0;
    match m1284 {
        Adt14::C0(_, v2, _, _, _) => {
            v2
        }
    }
}

pub fn f202(v0: Adt14) -> List<Bytes> {
    let m1285 = v0;
    match m1285 {
        Adt14::C0(_, _, v3, _, _) => {
            v3
        }
    }
}

pub fn f203(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f204(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h1286, _) = v2;
        h1286
    };
    let a1287 = v3;
    let a1288 = 0u64;
    if nat_eq(a1287, a1288) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a1289 = v3;
            let a1290 = v1;
            let a1291 = 0u64;
            nat_quot(a1289, a1290, a1291)
        }, {
            let a1296 = f126(v0, {
                let a1292 = v3;
                let a1293 = v1;
                let a1294 = 0u64;
                nat_rem(a1292, a1293, a1294)
            });
            let a1297 = {
                let (_, h1295) = v2;
                h1295
            };
            append_bytes(a1296, a1297)
        }))
    }
}

pub fn f205(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h1298, _) = v1;
        h1298
    };
    let a1300 = {
        let a1299 = v0.clone();
        length_bytes(a1299)
    };
    let a1301 = v2;
    if nat_le(a1300, a1301) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let a1302 = f87(v0.clone(), v2)?;
        let a1303 = 13u64;
        if nat_eq(a1302, a1303) {
            Ok(Some(({
                let a1306 = f87(v0, {
                    let a1304 = v2;
                    let a1305 = 1u64;
                    nat_add(a1304, a1305)?
                })?;
                let a1307 = 10u64;
                if nat_eq(a1306, a1307) {
                    let a1308 = v2;
                    let a1309 = 2u64;
                    nat_add(a1308, a1309)?
                } else {
                    let a1310 = v2;
                    let a1311 = 1u64;
                    nat_add(a1310, a1311)?
                }
            }, {
                let a1313 = {
                    let (_, h1312) = v1;
                    h1312
                };
                let a1314 = f8(Str::lit("\u{a}"));
                append_bytes(a1313, a1314)
            })))
        } else {
            let v4: u64 = {
                let m1315 = f133(v0.clone(), f8(Str::lit("\u{d}")), v2)?;
                match m1315 {
                    None => {
                        let a1316 = v0.clone();
                        length_bytes(a1316)
                    }
                    Some(v3) => {
                        v3
                    }
                }
            };
            Ok(Some((v4, {
                let a1320 = {
                    let (_, h1317) = v1;
                    h1317
                };
                let a1321 = f32(v0, v2, {
                    let a1318 = v4;
                    let a1319 = v2;
                    nat_sub(a1318, a1319)
                });
                append_bytes(a1320, a1321)
            })))
        }
    }
}

pub fn f206(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1322 = f307(v0, v1)?;
    match m1322 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f308({
                let (h1323, _) = v2;
                h1323
            })? {
                Ok(Some({
                    let a1325 = v1;
                    let a1326 = {
                        let (_, h1324) = v2;
                        h1324
                    };
                    nat_add(a1325, a1326)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f207(v0: u64, v1: Bytes) -> R<bool> {
    f309(v0, v1.clone(), 0u64, {
        let a1328 = {
            let a1327 = v1;
            length_bytes(a1327)
        };
        let a1329 = 3u64;
        let a1330 = 0u64;
        nat_quot(a1328, a1329, a1330)
    }, true)
}

pub fn f208() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{0}\u{0} "))
}

pub fn f209(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f310(v1, v0, v2)
}

pub fn f210(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f311(v0, v1)
}

pub fn f211(v0: Bytes, p1333: List<Bytes>) -> bool {
    let m1332 = p1333;
    {
        let mut m1332 = m1332;
        loop {
            let step = {
                let v1 = m1332;
                let m1331 = v1;
                match m1331.uncons() {
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
                Ok(next) => { m1332 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f212(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f312(v0, v1, v2)
}

pub fn f213(v0: Bytes) -> Result<(Bytes, u64), Bytes> {
    Err::<(Bytes, u64), Bytes>(v0)
}

pub fn f214(v0: Bytes, v1: u64) -> R<Result<(Bytes, u64), Bytes>> {
    if f86(v0.clone(), v1, Str::lit("&#")) {
        let v2: bool = {
            let a1336 = f87(v0.clone(), {
                let a1334 = v1;
                let a1335 = 2u64;
                nat_add(a1334, a1335)?
            })?;
            let a1337 = 120u64;
            nat_eq(a1336, a1337)
        };
        let v3: u64 = if v2 {
            let a1338 = v1;
            let a1339 = 3u64;
            nat_add(a1338, a1339)?
        } else {
            let a1340 = v1;
            let a1341 = 2u64;
            nat_add(a1340, a1341)?
        };
        let v4: (u64, u64) = f313(v0.clone(), v2, v3)?;
        let a1343 = {
            let (h1342, _) = v4;
            h1342
        };
        let a1344 = v3;
        if if nat_eq(a1343, a1344) {
            true
        } else {
            let a1346 = f87(v0, {
                let (h1345, _) = v4;
                h1345
            })?;
            let a1347 = 59u64;
            !nat_eq(a1346, a1347)
        } {
            Ok(f213(f83(Str::lit("a character reference must be &#digits; or &#xhex; (XML 1.0 [66])"), v1)?))
        } else {
            if f308({
                let (_, h1348) = v4;
                h1348
            })? {
                Ok(Ok::<(Bytes, u64), Bytes>((f314({
                    let (_, h1349) = v4;
                    h1349
                })?, {
                    let a1351 = {
                        let (h1350, _) = v4;
                        h1350
                    };
                    let a1352 = 1u64;
                    nat_add(a1351, a1352)?
                })))
            } else {
                Ok(f213(f83(Str::lit("a character reference must name a Char (XML 1.0 [66] WFC: Legal Character)"), v1)?))
            }
        }
    } else {
        if f86(v0.clone(), v1, Str::lit("&lt;")) {
            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("<")), {
                let a1353 = v1;
                let a1354 = 4u64;
                nat_add(a1353, a1354)?
            })))
        } else {
            if f86(v0.clone(), v1, Str::lit("&gt;")) {
                Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit(">")), {
                    let a1355 = v1;
                    let a1356 = 4u64;
                    nat_add(a1355, a1356)?
                })))
            } else {
                if f86(v0.clone(), v1, Str::lit("&amp;")) {
                    Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("&")), {
                        let a1357 = v1;
                        let a1358 = 5u64;
                        nat_add(a1357, a1358)?
                    })))
                } else {
                    if f86(v0.clone(), v1, Str::lit("&apos;")) {
                        Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("'")), {
                            let a1359 = v1;
                            let a1360 = 6u64;
                            nat_add(a1359, a1360)?
                        })))
                    } else {
                        if f86(v0, v1, Str::lit("&quot;")) {
                            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("\"")), {
                                let a1361 = v1;
                                let a1362 = 6u64;
                                nat_add(a1361, a1362)?
                            })))
                        } else {
                            Ok(f213(f83(Str::lit("an entity reference names an entity that is not declared, and only the five predefined entities are (XML 1.0 section 4.1, WFC: Entity Declared)"), v1)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f215(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<u64> {
    let (h1363, _) = f130(Fn4::F315(v1, v2, v0.clone()), {
        let a1365 = {
            let a1364 = v0;
            length_bytes(a1364)
        };
        let a1366 = 1u64;
        nat_add(a1365, a1366)?
    }, v3)?;
    Ok(h1363)
}

pub fn f216(p1379: Adt16, v1: List<(Bytes, Bytes)>) -> R<Result<Option<Adt27>, Bytes>> {
    let m1378 = p1379;
    {
        let mut m1378 = m1378;
        loop {
            let step = {
                let v0 = m1378;
                let m1367 = v0;
                match m1367 {
                    Adt16::C0(_, _, _) => {
                        Err::<Adt16, R<Result<Option<Adt27>, Bytes>>>(Ok(Ok::<Option<Adt27>, Bytes>(None::<Adt27>)))
                    }
                    Adt16::C1(_, _, _) => {
                        Err::<Adt16, R<Result<Option<Adt27>, Bytes>>>(Ok(Ok::<Option<Adt27>, Bytes>(None::<Adt27>)))
                    }
                    Adt16::C2(_) => {
                        Err::<Adt16, R<Result<Option<Adt27>, Bytes>>>(Ok(Ok::<Option<Adt27>, Bytes>(None::<Adt27>)))
                    }
                    Adt16::C3(r1376, r1377) => {
                        let v2 = (*r1376).clone();
                        let v3 = (*r1377).clone();
                        let m1368 = v2;
                        match m1368 {
                            Adt16::C0(v4, r1374, r1375) => {
                                let v5 = (*r1374).clone();
                                let v6 = (*r1375).clone();
                                let m1369 = f90(v5.clone(), v1.clone());
                                match m1369 {
                                    Ok(v8) => {
                                        let m1370 = f91(v8.clone(), v4)?;
                                        match m1370 {
                                            Ok(v10) => {
                                                if f211({
                                                    let (_, h1371) = v10.clone();
                                                    h1371
                                                }, List::cons(f8(Str::lit("restriction")), List::cons(f8(Str::lit("list")), List::cons(f8(Str::lit("union")), List::<Bytes>::nil())))) {
                                                    if f35({
                                                        let (h1372, _) = v10;
                                                        h1372
                                                    }, f151()) {
                                                        Err::<Adt16, R<Result<Option<Adt27>, Bytes>>>(Ok(Ok::<Option<Adt27>, Bytes>(Some(Adt27::C0({
                                                            let (_, h1373) = v10;
                                                            h1373
                                                        }, v5, v6, v8)))))
                                                    } else {
                                                        Ok::<Adt16, R<Result<Option<Adt27>, Bytes>>>(v3)
                                                    }
                                                } else {
                                                    Ok::<Adt16, R<Result<Option<Adt27>, Bytes>>>(v3)
                                                }
                                            }
                                            Err(v9) => {
                                                Err::<Adt16, R<Result<Option<Adt27>, Bytes>>>(Ok(f316(v9)))
                                            }
                                        }
                                    }
                                    Err(v7) => {
                                        Err::<Adt16, R<Result<Option<Adt27>, Bytes>>>(Ok(f316(v7)))
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
                Ok(next) => { m1378 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f217(v0: Bytes) -> Result<Adt22, Bytes> {
    Err::<Adt22, Bytes>(v0)
}

pub fn f218(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit(" at simpleType ")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f219(v0: List<(Bytes, Bytes)>, v1: Bytes, v2: Bytes) -> R<Result<Adt22, Bytes>> {
    let m1380 = f91(v0, v2.clone())?;
    match m1380 {
        Ok(v4) => {
            if f35({
                let (h1381, _) = v4.clone();
                h1381
            }, f151()) {
                let m1383 = f317({
                    let (_, h1382) = v4;
                    h1382
                }, 16u64)?;
                match m1383 {
                    Ok(v6) => {
                        Ok(Ok::<Adt22, Bytes>(Adt22::C0(v6, List::<Bytes>::nil())))
                    }
                    Err(v5) => {
                        Ok(f217(v5))
                    }
                }
            } else {
                if f35({
                    let (h1384, _) = v4.clone();
                    h1384
                }, v1) {
                    let v7: Bytes = f154({
                        let (_, h1385) = v4;
                        h1385
                    })?;
                    Ok(Ok::<Adt22, Bytes>(Adt22::C0(v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                } else {
                    Ok(f217(f7(List::cons(f8(Str::lit("the type ")), List::cons(v2, List::cons(f8(Str::lit(" is in the namespace ")), List::cons({
                        let (h1386, _) = v4;
                        h1386
                    }, List::cons(f8(Str::lit(", which is neither XML Schema's nor this document's target namespace; imported schema documents are not read yet")), List::<Bytes>::nil()))))))))
                }
            }
        }
        Err(v3) => {
            Ok(f217(v3))
        }
    }
}

pub fn f220(v0: Adt16, v1: List<(Bytes, Bytes)>) -> R<Result<List<Adt19>, Bytes>> {
    let m1387 = v0;
    match m1387 {
        Adt16::C0(_, _, _) => {
            Ok(Ok::<List<Adt19>, Bytes>(List::<Adt19>::nil()))
        }
        Adt16::C1(_, _, _) => {
            Ok(Ok::<List<Adt19>, Bytes>(List::<Adt19>::nil()))
        }
        Adt16::C2(_) => {
            Ok(Ok::<List<Adt19>, Bytes>(List::<Adt19>::nil()))
        }
        Adt16::C3(r1394, r1395) => {
            let v2 = (*r1394).clone();
            let v3 = (*r1395).clone();
            let m1388 = f220(v3, v1.clone())?;
            match m1388 {
                Ok(v5) => {
                    let m1389 = v2;
                    match m1389 {
                        Adt16::C0(v6, r1393, _) => {
                            let v7 = (*r1393).clone();
                            let m1390 = f90(v7.clone(), v1);
                            match m1390 {
                                Ok(v10) => {
                                    let m1391 = f91(v10, v6)?;
                                    match m1391 {
                                        Ok(v12) => {
                                            if f92(v12, Str::lit("enumeration")) {
                                                let m1392 = f93(v7, f8(Str::lit("value")));
                                                match m1392 {
                                                    None => {
                                                        Ok(f318(f8(Str::lit("an enumeration facet must have a value attribute (XSD 1.1 Part 2, section 4.3.5.2)"))))
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
                                            Ok(f318(v11))
                                        }
                                    }
                                }
                                Err(v9) => {
                                    Ok(f318(v9))
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
                    Ok(f318(v4))
                }
            }
        }
        Adt16::C4 => {
            Ok(Ok::<List<Adt19>, Bytes>(List::<Adt19>::nil()))
        }
    }
}

pub fn f221(v0: List<Adt19>, v1: List<Bytes>) -> R<List<Adt19>> {
    let m1396 = v0;
    match m1396.uncons() {
        None => {
            Ok(List::<Adt19>::nil())
        }
        Some((v2, v3)) => {
            let v4: Bytes = f105(v2.clone())?;
            if f115(v4.clone(), v1.clone()) {
                f221(v3, v1)
            } else {
                Ok(List::cons(v2, f221(v3, List::cons(v4, v1))?))
            }
        }
    }
}

pub fn f222(v0: List<Adt19>) -> List<Bytes> {
    let m1397 = v0;
    match m1397.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1398 = f319(Some(v1));
            match m1398 {
                None => {
                    f222(v2)
                }
                Some(v3) => {
                    List::cons(v3, f222(v2))
                }
            }
        }
    }
}

pub fn f223(v0: Bytes, p1419: u64, p1420: u64, p1421: bool, p1422: Bytes) -> R<Bytes> {
    let m1418 = (p1419, (p1420, (p1421, p1422)));
    {
        let mut m1418 = m1418;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1418;
                let m1399 = v2;
                if m1399 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1399 - 1;
                    let m1402 = {
                        let a1400 = v0.clone();
                        let a1401 = v1;
                        index_bytes(a1400, a1401)
                    };
                    match m1402 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f42(v6)?;
                            if f320(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1403 = v1;
                                    let a1404 = 1u64;
                                    nat_add(a1403, a1404)?
                                }, (v5, (true, v4))))
                            } else {
                                if f321(f322(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1405 = v1;
                                        let a1406 = 1u64;
                                        nat_add(a1405, a1406)?
                                    }, (v5, (false, {
                                        let a1410 = v4;
                                        let a1411 = f32({
                                            let a1407 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1407)
                                        }, {
                                            let a1408 = v7;
                                            let a1409 = 97u64;
                                            nat_sub(a1408, a1409)
                                        }, 1u64);
                                        append_bytes(a1410, a1411)
                                    }))))
                                } else {
                                    if f323(f322(v7), f323(f324(v7), f325(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1412 = v1;
                                            let a1413 = 1u64;
                                            nat_add(a1412, a1413)?
                                        }, (v5, (false, {
                                            let a1414 = v4;
                                            let a1415 = f32(v0.clone(), v1, 1u64);
                                            append_bytes(a1414, a1415)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1416 = v1;
                                            let a1417 = 1u64;
                                            nat_add(a1416, a1417)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1418 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f224(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f147(v0.clone(), v1) {
        let m1423 = f30(v0, 34u8)?;
        match m1423.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m1424 = v3;
                match m1424.uncons() {
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

pub fn f225(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f326(v0, v1)
}

pub fn f226(v0: List<Adt18>) -> R<Result<List<Adt24>, Bytes>> {
    let m1425 = v0;
    match m1425.uncons() {
        None => {
            Ok(Ok::<List<Adt24>, Bytes>(List::<Adt24>::nil()))
        }
        Some((v1, v2)) => {
            let m1428 = f167(match v1.clone() {
                Adt18::C0(_, _, _, _, h1426, _) => {
                    h1426
                }
            }, match v1.clone() {
                Adt18::C0(_, _, _, h1427, _, _) => {
                    h1427
                }
            })?;
            match m1428 {
                Ok(v4) => {
                    let m1429 = f226(v2)?;
                    match m1429 {
                        Ok(v6) => {
                            Ok(Ok::<List<Adt24>, Bytes>(f327(f328(v4, match v1 {
                                Adt18::C0(_, _, h1430, _, _, _) => {
                                    h1430
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

pub fn f227(v0: List<Adt18>) -> List<Bytes> {
    let m1431 = v0;
    match m1431.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt18::C0(_, _, h1432, _, _, _) => {
                    h1432
                }
            }, f227(v2))
        }
    }
}

pub fn f228(v0: u64, v1: Adt23, v2: List<Adt24>) -> R<Result<List<Adt26>, Bytes>> {
    let m1433 = v2;
    match m1433.uncons() {
        None => {
            Ok(Ok::<List<Adt26>, Bytes>(List::<Adt26>::nil()))
        }
        Some((v3, v4)) => {
            let m1434 = f329(v0, v1.clone(), v3)?;
            match m1434 {
                Ok(v6) => {
                    let m1435 = f228(v0, v1, v4)?;
                    match m1435 {
                        Ok(v8) => {
                            Ok(Ok::<List<Adt26>, Bytes>(f330(v6, v8)))
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

pub fn f229(v0: List<Adt18>) -> R<u64> {
    let m1436 = v0;
    match m1436.uncons() {
        None => {
            Ok(64u64)
        }
        Some((v1, v2)) => {
            let a1438 = match v1 {
                Adt18::C0(_, _, _, _, _, h1437) => {
                    h1437
                }
            };
            let a1439 = f229(v2)?;
            nat_add(a1438, a1439)
        }
    }
}

pub fn f230(p1447: List<Adt26>, p1448: List<Adt26>) -> Result<List<Adt26>, Bytes> {
    let m1446 = (p1447, p1448);
    {
        let mut m1446 = m1446;
        loop {
            let step = {
                let (v0, v1) = m1446;
                let m1440 = v0;
                match m1440.uncons() {
                    None => {
                        Err::<(List<Adt26>, List<Adt26>), Result<List<Adt26>, Bytes>>(Ok::<List<Adt26>, Bytes>(f331(v1, List::<Adt26>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m1442 = f332(v1.clone(), match v2.clone() {
                            Adt26::C0(h1441, _, _, _) => {
                                h1441
                            }
                        });
                        match m1442 {
                            None => {
                                Ok::<(List<Adt26>, List<Adt26>), Result<List<Adt26>, Bytes>>((v3, List::cons(v2, v1)))
                            }
                            Some(v4) => {
                                if f35(match v4 {
                                    Adt26::C0(_, _, h1443, _) => {
                                        h1443
                                    }
                                }, match v2.clone() {
                                    Adt26::C0(_, _, h1444, _) => {
                                        h1444
                                    }
                                }) {
                                    Ok::<(List<Adt26>, List<Adt26>), Result<List<Adt26>, Bytes>>((v3, v1))
                                } else {
                                    Err::<(List<Adt26>, List<Adt26>), Result<List<Adt26>, Bytes>>(Err::<List<Adt26>, Bytes>(f163(f8(Str::lit("two components derive the type name ")), match v2 {
                                        Adt26::C0(h1445, _, _, _) => {
                                            h1445
                                        }
                                    }, f8(Str::lit(", so their LexLean declarations would collide")))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1446 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f231(v0: List<Adt26>) -> List<Adt26> {
    if if f115(f333(), f334(v0.clone())) {
        let m1449 = f332(v0.clone(), f333());
        match m1449 {
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
        List::cons(Adt26::C0(f333(), List::<Bytes>::nil(), f197(f333(), f335()), f335()), v0)
    } else {
        v0
    }
}

pub fn f232(v0: List<Adt26>) -> List<Bytes> {
    let m1450 = v0;
    match m1450.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt26::C0(h1451, _, _, _) => {
                    h1451
                }
            }, f232(v2))
        }
    }
}

pub fn f233(p1457: List<Adt26>, v1: List<Bytes>) -> Option<(Bytes, Bytes)> {
    let m1456 = p1457;
    {
        let mut m1456 = m1456;
        loop {
            let step = {
                let v0 = m1456;
                let m1452 = v0;
                match m1452.uncons() {
                    None => {
                        Err::<List<Adt26>, Option<(Bytes, Bytes)>>(None::<(Bytes, Bytes)>)
                    }
                    Some((v2, v3)) => {
                        let m1454 = f155(match v2.clone() {
                            Adt26::C0(_, h1453, _, _) => {
                                h1453
                            }
                        }, v1.clone());
                        match m1454 {
                            None => {
                                Ok::<List<Adt26>, Option<(Bytes, Bytes)>>(v3)
                            }
                            Some(v4) => {
                                Err::<List<Adt26>, Option<(Bytes, Bytes)>>(Some((match v2 {
                                    Adt26::C0(h1455, _, _, _) => {
                                        h1455
                                    }
                                }, v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1456 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f234(v0: List<Adt26>, v1: List<Bytes>, v2: u64) -> List<(Bytes, List<Bytes>)> {
    let m1458 = v1;
    match m1458.uncons() {
        None => {
            List::<(Bytes, List<Bytes>)>::nil()
        }
        Some((v3, v4)) => {
            List::cons((v3.clone(), f336(v2, v0.clone(), f337(v0.clone(), v3), List::<Bytes>::nil())), f234(v0, v4, v2))
        }
    }
}

pub fn f235(v0: List<Adt26>) -> R<u64> {
    let m1459 = v0;
    match m1459.uncons() {
        None => {
            Ok(1u64)
        }
        Some((v1, v2)) => {
            let a1464 = {
                let a1462 = {
                    let a1461 = match v1 {
                        Adt26::C0(_, h1460, _, _) => {
                            h1460
                        }
                    };
                    length_list(a1461)
                };
                let a1463 = 1u64;
                nat_add(a1462, a1463)?
            };
            let a1465 = f235(v2)?;
            nat_add(a1464, a1465)
        }
    }
}

pub fn f236(p1475: u64, v1: List<Adt26>, p1476: List<(List<Bytes>, List<Bytes>)>, p1477: List<Bytes>, p1478: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m1474 = (p1475, (p1476, (p1477, p1478)));
    {
        let mut m1474 = m1474;
        loop {
            let step = {
                let (v0, (v2, (v3, v4))) = m1474;
                let m1466 = v0;
                if m1466 == 0 {
                    let m1467 = v2;
                    match m1467.uncons() {
                        None => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v4))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(f8(Str::lit("the declarations could not be ordered so that each follows the types it uses"))))
                        }
                    }
                } else {
                    let v7 = m1466 - 1;
                    let m1468 = v2.clone();
                    match m1468.uncons() {
                        None => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v4))
                        }
                        Some((_, _)) => {
                            let m1469 = f338(v2, v3.clone());
                            match m1469 {
                                None => {
                                    Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(f8(Str::lit("the declarations could not be ordered so that each follows the types it uses"))))
                                }
                                Some(v10) => {
                                    let m1471 = f339(v1.clone(), {
                                        let (h1470, _) = v10.clone();
                                        h1470
                                    });
                                    match m1471 {
                                        Ok(v12) => {
                                            Ok::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>((v7, ({
                                                let (_, h1472) = v10;
                                                h1472
                                            }, (f340({
                                                let (h1473, _) = v10;
                                                h1473
                                            }, v3), f340(v4, v12)))))
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
                Ok(next) => { m1474 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f237(v0: List<Adt26>, v1: List<List<Bytes>>) -> List<(List<Bytes>, List<Bytes>)> {
    let m1479 = v1;
    match m1479.uncons() {
        None => {
            List::<(List<Bytes>, List<Bytes>)>::nil()
        }
        Some((v2, v3)) => {
            List::cons((v2.clone(), f341(v0.clone(), v2.clone(), v2)), f237(v0, v3))
        }
    }
}

pub fn f238(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> List<List<Bytes>> {
    let m1480 = v2;
    match m1480.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((v4, v5)) => {
            if f115(v4.clone(), v3.clone()) {
                f238(v0, v1, v5, v3)
            } else {
                let v6: List<Bytes> = f342(v0.clone(), v1.clone(), v4);
                List::cons(v6.clone(), f238(v0, v1, v5, f340(v6, v3)))
            }
        }
    }
}

pub fn f239(v0: bool) -> bool {
    !v0
}

pub fn f240(v0: Adt25, v1: Str) -> bool {
    f35(match v0 {
        Adt25::C0(_, h1481, _, _, _) => {
            h1481
        }
    }, f8(v1))
}

pub fn f241(v0: Adt25, v1: Str) -> Option<Bytes> {
    f93(match v0 {
        Adt25::C0(_, _, h1482, _, _) => {
            h1482
        }
    }, f8(v1))
}

pub fn f242(v0: Option<Bytes>, v1: Bytes) -> R<Result<Bytes, Bytes>> {
    let m1483 = f133(v1.clone(), f8(Str::lit("://")), 0u64)?;
    match m1483 {
        None => {
            let v3: Bytes = f163(f8(Str::lit("the relative schema location ")), v1.clone(), f8(Str::lit(" is not resolved, because only a bare file name relative to a bound document's URL is merged (RFC 3986, section 5.2.3)")));
            if if f147(v1.clone(), f8(Str::lit("/"))) {
                true
            } else {
                let m1484 = f133(v1.clone(), f8(Str::lit("..")), 0u64)?;
                match m1484 {
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
                let m1485 = v0;
                match m1485 {
                    None => {
                        Ok(Err::<Bytes, Bytes>(v3))
                    }
                    Some(v5) => {
                        let m1487 = f343(v5.clone(), 0u64, {
                            let a1486 = v5.clone();
                            length_bytes(a1486)
                        }, None::<u64>)?;
                        match m1487 {
                            None => {
                                Ok(Err::<Bytes, Bytes>(v3))
                            }
                            Some(v6) => {
                                Ok(Ok::<Bytes, Bytes>({
                                    let a1490 = f32(v5, 0u64, {
                                        let a1488 = v6;
                                        let a1489 = 1u64;
                                        nat_add(a1488, a1489)?
                                    });
                                    let a1491 = v1;
                                    append_bytes(a1490, a1491)
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

pub fn f243(p1496: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1495 = p1496;
    {
        let mut m1495 = m1495;
        loop {
            let step = {
                let v0 = m1495;
                let m1492 = v0;
                match m1492.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h1493, _) = v2;
                            h1493
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h1494) = v2;
                                h1494
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1495 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f244(v0: Adt25, v1: Str, v2: Str) -> Bytes {
    let m1497 = f241(v0, v1);
    match m1497 {
        None => {
            f8(v2)
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f245(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1498 = f307(v0, v1)?;
    match m1498 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f344({
                let (h1499, _) = v2;
                h1499
            }) {
                Ok(Some({
                    let a1501 = v1;
                    let a1502 = {
                        let (_, h1500) = v2;
                        h1500
                    };
                    nat_add(a1501, a1502)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f246(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1504 = v1;
    let a1505 = {
        let a1503 = v0.clone();
        length_bytes(a1503)
    };
    if if nat_lt(a1504, a1505) {
        f345(v0.clone(), v1)?
    } else {
        false
    } {
        Ok(Some(f255(v0, v1)?))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f247(v0: Bytes, v1: u64) -> R<u64> {
    let (h1506, _) = f130(Fn4::F346(v0), {
        let a1507 = v1;
        let a1508 = 1u64;
        nat_add(a1507, a1508)?
    }, v1)?;
    Ok(h1506)
}

pub fn f248(v0: Bytes, v1: u64) -> R<u64> {
    let (h1509, _) = f130(Fn4::F347(v0.clone()), {
        let a1511 = {
            let a1510 = v0;
            length_bytes(a1510)
        };
        let a1512 = 1u64;
        nat_add(a1511, a1512)?
    }, v1)?;
    Ok(h1509)
}

pub fn f249(v0: Bytes, v1: u64) -> R<u64> {
    let (h1513, _) = f130(Fn4::F348(v0.clone()), {
        let a1515 = {
            let a1514 = v0;
            length_bytes(a1514)
        };
        let a1516 = 1u64;
        nat_add(a1515, a1516)?
    }, v1)?;
    Ok(h1513)
}

pub fn f250(v0: Bytes, v1: u64) -> R<bool> {
    let a1517 = f173(v0.clone(), v1)?;
    let a1518 = 45u64;
    if nat_eq(a1517, a1518) {
        if f172(f173(v0.clone(), {
            let a1519 = v1;
            let a1520 = 1u64;
            nat_add(a1519, a1520)?
        })?) {
            Ok(true)
        } else {
            Ok(f174(f173(v0, {
                let a1521 = v1;
                let a1522 = 1u64;
                nat_add(a1521, a1522)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f251(v0: Bytes, v1: u64) -> R<bool> {
    let m1523 = f257(v0, v1)?;
    match m1523 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f252(v0: Bytes, v1: u64) -> R<Option<Bytes>> {
    let v2: u64 = f173(v0.clone(), v1)?;
    let v3: bool = f178(f349(f173(v0, {
        let a1524 = v1;
        let a1525 = 1u64;
        nat_add(a1524, a1525)?
    })?, true));
    let a1526 = v2;
    let a1527 = 58u64;
    if if nat_eq(a1526, a1527) {
        v3
    } else {
        false
    } {
        Ok(Some(f83(Str::lit("a null key is not read: Studio.Value has no null (YAML 1.2.2 section 10.3.2)"), v1)?))
    } else {
        let a1528 = v2;
        let a1529 = 38u64;
        if nat_eq(a1528, a1529) {
            Ok(Some(f83(Str::lit("an anchor is not read (YAML 1.2.2 [101])"), v1)?))
        } else {
            let a1530 = v2;
            let a1531 = 42u64;
            if nat_eq(a1530, a1531) {
                Ok(Some(f83(Str::lit("an alias is not read (YAML 1.2.2 [104])"), v1)?))
            } else {
                let a1532 = v2;
                let a1533 = 33u64;
                if nat_eq(a1532, a1533) {
                    Ok(Some(f83(Str::lit("a tag is not read (YAML 1.2.2 [97])"), v1)?))
                } else {
                    let a1534 = v2;
                    let a1535 = 63u64;
                    if if nat_eq(a1534, a1535) {
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

pub fn f253(v0: Bytes, v1: u64, v2: u64) -> R<Adt20> {
    let v3: u64 = f173(v0.clone(), v1)?;
    let a1536 = v3;
    let a1537 = 124u64;
    if if nat_eq(a1536, a1537) {
        true
    } else {
        let a1538 = v3;
        let a1539 = 62u64;
        nat_eq(a1538, a1539)
    } {
        f350(v0, v1, v2)
    } else {
        let a1540 = v3;
        let a1541 = 34u64;
        if if nat_eq(a1540, a1541) {
            true
        } else {
            let a1542 = v3;
            let a1543 = 39u64;
            nat_eq(a1542, a1543)
        } {
            let m1544 = f351(v0.clone(), v1, v2, false)?;
            match m1544 {
                Adt30::C0(v5, v6) => {
                    let m1545 = f259(v0, v6)?;
                    match m1545 {
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
            let a1546 = v3;
            let a1547 = 91u64;
            if if nat_eq(a1546, a1547) {
                true
            } else {
                let a1548 = v3;
                let a1549 = 123u64;
                nat_eq(a1548, a1549)
            } {
                let m1553 = f352(v0.clone(), v1, v2, {
                    let a1551 = {
                        let a1550 = v0.clone();
                        length_bytes(a1550)
                    };
                    let a1552 = 2u64;
                    nat_add(a1551, a1552)?
                }, 0u64, List::<Bytes>::nil())?;
                match m1553 {
                    Adt20::C0(v10, v11) => {
                        let a1554 = f173(v0.clone(), f249(v0.clone(), v11)?)?;
                        let a1555 = 58u64;
                        if nat_eq(a1554, a1555) {
                            Ok(Adt20::C1(f83(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [154])"), v1)?))
                        } else {
                            let m1556 = f259(v0, v11)?;
                            match m1556 {
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
                let m1557 = f252(v0.clone(), v1)?;
                match m1557 {
                    None => {
                        if f178(f353(v0.clone(), v1, false)?) {
                            Ok(Adt20::C1(f83(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                        } else {
                            let v15: u64 = f354(v0.clone(), v1, false)?;
                            let m1561 = f355(v0.clone(), v15, v2, {
                                let a1559 = {
                                    let a1558 = v0.clone();
                                    length_bytes(a1558)
                                };
                                let a1560 = 2u64;
                                nat_add(a1559, a1560)?
                            }, f356(v0, v1, v15))?;
                            match m1561 {
                                Adt30::C0(v17, v18) => {
                                    f357(v17, v1, v18)
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

pub fn f254(v0: bool, v1: Adt20) -> Adt20 {
    let m1562 = v1;
    match m1562 {
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

pub fn f255(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f358(v0.clone(), v1)?;
    let a1564 = v2;
    let a1565 = {
        let a1563 = v0;
        length_bytes(a1563)
    };
    if nat_lt(a1564, a1565) {
        let a1566 = v2;
        let a1567 = 1u64;
        nat_add(a1566, a1567)
    } else {
        Ok(v2)
    }
}

pub fn f256(v0: u64) -> R<Bytes> {
    f83(Str::lit("a null node is not read: Studio.Value has no null (YAML 1.2.2 section 10.3.2)"), v0)
}

pub fn f257(v0: Bytes, v1: u64) -> R<Option<u64>> {
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
        let m1572 = f351(v0.clone(), v1, 0u64, true)?;
        match m1572 {
            Adt30::C0(_, v5) => {
                let v6: u64 = f249(v0.clone(), v5)?;
                if f359(v0, v6)? {
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
        if f353(v0.clone(), v1, false)? {
            let v7: u64 = f249(v0.clone(), f354(v0.clone(), v1, false)?)?;
            if f359(v0, v7)? {
                Ok(Some(v7))
            } else {
                Ok(None::<u64>)
            }
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f258(v0: Bytes, v1: u64) -> R<Bytes> {
    let v2: u64 = f173(v0.clone(), v1)?;
    let a1573 = v2;
    let a1574 = 34u64;
    if if nat_eq(a1573, a1574) {
        true
    } else {
        let a1575 = v2;
        let a1576 = 39u64;
        nat_eq(a1575, a1576)
    } {
        let m1577 = f351(v0, v1, 0u64, true)?;
        match m1577 {
            Adt30::C0(v4, _) => {
                Ok(v4)
            }
            Adt30::C1(_) => {
                Ok(Bytes::lit(&[]))
            }
        }
    } else {
        Ok(f356(v0.clone(), v1, f354(v0, v1, false)?))
    }
}

pub fn f259(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f249(v0.clone(), v1)?;
    let v3: u64 = f173(v0.clone(), v2)?;
    if if f174(v3) {
        true
    } else {
        let a1578 = v3;
        let a1579 = 35u64;
        if nat_eq(a1578, a1579) {
            let a1580 = v1;
            let a1581 = v2;
            nat_lt(a1580, a1581)
        } else {
            false
        }
    } {
        Ok(Ok::<u64, Bytes>(f255(v0, v2)?))
    } else {
        Ok(Err::<u64, Bytes>(f83(Str::lit("only white space and a comment may follow a node on its line (YAML 1.2.2 [77])"), v2)?))
    }
}

pub fn f260(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1582 = 0u64;
        let a1583 = v1;
        nat_lt(a1582, a1583)
    } else {
        false
    } {
        let a1584 = Bytes::lit(&[0x2d]);
        let a1585 = f52(v1)?;
        Ok(append_bytes(a1584, a1585))
    } else {
        f52(v1)
    }
}

pub fn f261(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f52(v1)?;
    let v4: u64 = {
        let a1586 = v3.clone();
        length_bytes(a1586)
    };
    let v5: Bytes = {
        let a1587 = v2;
        let a1588 = v4;
        if nat_lt(a1587, a1588) {
            f7(List::cons(f32(v3.clone(), 0u64, {
                let a1589 = v4;
                let a1590 = v2;
                nat_sub(a1589, a1590)
            }), List::cons(f8(Str::lit(".")), List::cons(f32(v3, {
                let a1591 = v4;
                let a1592 = v2;
                nat_sub(a1591, a1592)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f360({
                let a1593 = v2;
                let a1594 = v4;
                nat_sub(a1593, a1594)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1595 = f8(Str::lit("-"));
        let a1596 = v5;
        Ok(append_bytes(a1595, a1596))
    } else {
        Ok(v5)
    }
}

pub fn f262(v0: Bytes) -> R<Bytes> {
    let a1601 = f361(v0.clone(), 0u64, {
        let a1598 = {
            let a1597 = v0.clone();
            length_bytes(a1597)
        };
        let a1599 = 1u64;
        nat_add(a1598, a1599)?
    })?;
    let a1602 = {
        let a1600 = v0.clone();
        length_bytes(a1600)
    };
    if nat_eq(a1601, a1602) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        f362(v0)
    }
}

pub fn f263(v0: List<Bytes>) -> Bytes {
    let a1605 = {
        let a1603 = Bytes::lit(&[0x5b]);
        let a1604 = f33(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1603, a1604)
    };
    let a1606 = Bytes::lit(&[0x5d]);
    append_bytes(a1605, a1606)
}

pub fn f264(v0: Adt19) -> R<List<Bytes>> {
    let m1607 = v0;
    match m1607 {
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
        Adt19::C9(r1608, r1609) => {
            let v12 = (*r1608).clone();
            let v13 = (*r1609).clone();
            Ok(List::cons(f179(v12)?, f264(v13)?))
        }
        Adt19::C10(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f265(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f363(Fn5::F364, List::<(Bytes, Bytes)>::nil(), f365(v0)?);
    let v2: List<Bytes> = f366(Fn6::F367, List::<Bytes>::nil(), v1)?;
    let a1612 = {
        let a1610 = Bytes::lit(&[0x7b]);
        let a1611 = f33(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1610, a1611)
    };
    let a1613 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1612, a1613))
}

pub fn f266(v0: Adt19) -> R<List<(Bytes, Bytes)>> {
    let m1614 = v0;
    match m1614 {
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
        Adt19::C10(v14, r1615, r1616) => {
            let v15 = (*r1615).clone();
            let v16 = (*r1616).clone();
            Ok(List::cons((v14, f179(v15)?), f266(v16)?))
        }
    }
}

pub fn f267(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a1617 = v0;
        utf8_encode(a1617)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f52(v1)?, List::<Bytes>::nil())))))
}

pub fn f268(p1620: List<Adt21>, p1621: List<Adt21>) -> List<Adt21> {
    let m1619 = (p1620, p1621);
    {
        let mut m1619 = m1619;
        loop {
            let step = {
                let (v0, v1) = m1619;
                let m1618 = v0;
                match m1618.uncons() {
                    None => {
                        Err::<(List<Adt21>, List<Adt21>), List<Adt21>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt21>, List<Adt21>), List<Adt21>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1619 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f269(v0: u64) -> bool {
    let a1622 = 65u64;
    let a1623 = v0;
    if if nat_le(a1622, a1623) {
        let a1624 = v0;
        let a1625 = 90u64;
        nat_le(a1624, a1625)
    } else {
        false
    } {
        true
    } else {
        let a1626 = 97u64;
        let a1627 = v0;
        if nat_le(a1626, a1627) {
            let a1628 = v0;
            let a1629 = 122u64;
            nat_le(a1628, a1629)
        } else {
            false
        }
    }
}

pub fn f270(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h1630, _) = f130(Fn4::F368(v1, v0.clone()), {
        let a1632 = {
            let a1631 = v0;
            length_bytes(a1631)
        };
        let a1633 = 1u64;
        nat_add(a1632, a1633)?
    }, v2)?;
    Ok(h1630)
}

pub fn f271(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f87(v0.clone(), v1)?;
    let a1634 = v2;
    let a1635 = 48u64;
    if nat_eq(a1634, a1635) {
        let v3: u64 = f87(v0.clone(), {
            let a1636 = v1;
            let a1637 = 1u64;
            nat_add(a1636, a1637)?
        })?;
        let a1638 = v3;
        let a1639 = 88u64;
        if if if nat_eq(a1638, a1639) {
            true
        } else {
            let a1640 = v3;
            let a1641 = 120u64;
            nat_eq(a1640, a1641)
        } {
            f369(f87(v0.clone(), {
                let a1642 = v1;
                let a1643 = 2u64;
                nat_add(a1642, a1643)?
            })?)
        } else {
            false
        } {
            f270(v0, 1u64, {
                let a1644 = v1;
                let a1645 = 2u64;
                nat_add(a1644, a1645)?
            })
        } else {
            f270(v0, 2u64, {
                let a1646 = v1;
                let a1647 = 1u64;
                nat_add(a1646, a1647)?
            })
        }
    } else {
        if f370(v2) {
            f270(v0, 0u64, v1)
        } else {
            Ok(v1)
        }
    }
}

pub fn f272(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f270(v0.clone(), 0u64, v1)?;
    let a1648 = f87(v0.clone(), v2)?;
    let a1649 = 46u64;
    if nat_eq(a1648, a1649) {
        let v3: u64 = f270(v0.clone(), 0u64, {
            let a1650 = v2;
            let a1651 = 1u64;
            nat_add(a1650, a1651)?
        })?;
        let a1652 = v1;
        let a1653 = v2;
        if if nat_lt(a1652, a1653) {
            true
        } else {
            let a1656 = {
                let a1654 = v2;
                let a1655 = 1u64;
                nat_add(a1654, a1655)?
            };
            let a1657 = v3;
            nat_lt(a1656, a1657)
        } {
            f371(v0, v3)
        } else {
            Ok(v1)
        }
    } else {
        let a1658 = v1;
        let a1659 = v2;
        if nat_lt(a1658, a1659) {
            let v4: u64 = f371(v0, v2)?;
            let a1660 = v2;
            let a1661 = v4;
            if nat_lt(a1660, a1661) {
                Ok(v4)
            } else {
                Ok(v1)
            }
        } else {
            Ok(v1)
        }
    }
}

pub fn f273(v0: List<Adt21>, v1: Str) -> bool {
    let m1662 = v0;
    match m1662.uncons() {
        None => {
            false
        }
        Some((v2, _)) => {
            f372(v2, {
                let a1663 = v1;
                utf8_encode(a1663)
            })
        }
    }
}

pub fn f274(p1676: u64, p1677: List<Adt21>, p1678: List<Bytes>, v3: bool) -> Result<List<Adt21>, Bytes> {
    let m1675 = (p1676, (p1677, p1678));
    {
        let mut m1675 = m1675;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1675;
                let m1664 = v0;
                if m1664 == 0 {
                    Err::<(u64, (List<Adt21>, List<Bytes>)), Result<List<Adt21>, Bytes>>(Err::<List<Adt21>, Bytes>(f8(Str::lit("brackets nest deeper than the input is long"))))
                } else {
                    let v4 = m1664 - 1;
                    let m1665 = v1;
                    match m1665.uncons() {
                        None => {
                            Err::<(u64, (List<Adt21>, List<Bytes>)), Result<List<Adt21>, Bytes>>(Err::<List<Adt21>, Bytes>(f8(Str::lit("the input ends inside brackets or before the ; that closes a member (Web IDL, IDL grammar)"))))
                        }
                        Some((v5, v6)) => {
                            let a1666 = f373(v5.clone());
                            let a1667 = 3u64;
                            if nat_eq(a1666, a1667) {
                                Ok::<(u64, (List<Adt21>, List<Bytes>)), Result<List<Adt21>, Bytes>>((v4, (v6, v2)))
                            } else {
                                let v7: Bytes = f374(v5);
                                let v8: Bytes = f375(v7.clone());
                                if !f35(v8.clone(), Bytes::lit(&[])) {
                                    Ok::<(u64, (List<Adt21>, List<Bytes>)), Result<List<Adt21>, Bytes>>((v4, (v6, List::cons(v8, v2))))
                                } else {
                                    if f376(v7.clone()) {
                                        let m1668 = v2;
                                        match m1668.uncons() {
                                            None => {
                                                Err::<(u64, (List<Adt21>, List<Bytes>)), Result<List<Adt21>, Bytes>>(Err::<List<Adt21>, Bytes>(f7(List::cons(f8(Str::lit("the closing bracket ")), List::cons(v7, List::cons(f8(Str::lit(" has no opening bracket (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
                                            }
                                            Some((v9, v10)) => {
                                                if f35(v9.clone(), v7.clone()) {
                                                    if if !v3 {
                                                        let a1670 = {
                                                            let a1669 = v10.clone();
                                                            length_list(a1669)
                                                        };
                                                        let a1671 = 0u64;
                                                        nat_eq(a1670, a1671)
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
                                                let a1673 = {
                                                    let a1672 = v2.clone();
                                                    length_list(a1672)
                                                };
                                                let a1674 = 0u64;
                                                nat_eq(a1673, a1674)
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
                Ok(next) => { m1675 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f275(v0: List<Adt21>) -> List<Adt21> {
    let m1679 = v0;
    match m1679.uncons() {
        None => {
            List::<Adt21>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f276(v0: u64, v1: u64, v2: List<Adt21>, v3: bool) -> R<Result<(Adt12, List<Adt21>), Bytes>> {
    let m1680 = f279(v2);
    match m1680 {
        Ok(v5) => {
            let v6: bool = if v3 {
                f273(f281(v5.clone()), Str::lit(":"))
            } else {
                false
            };
            let m1681 = if v6 {
                f279(f275(f281(v5.clone())))
            } else {
                Ok::<Adt28, Bytes>(Adt28::C0(Bytes::lit(&[]), f281(v5.clone())))
            };
            match m1681 {
                Ok(v8) => {
                    let m1682 = f280(f281(v8.clone()), Str::lit("{"));
                    match m1682 {
                        Ok(v10) => {
                            let m1687 = {
                                let a1683 = v1;
                                let a1684 = 2u64;
                                if if nat_eq(a1683, a1684) {
                                    true
                                } else {
                                    let a1685 = v1;
                                    let a1686 = 7u64;
                                    nat_eq(a1685, a1686)
                                } {
                                    f377(v0, v10, List::<Adt13>::nil())?
                                } else {
                                    f378(v0, v10, List::<Adt13>::nil())?
                                }
                            };
                            match m1687 {
                                Ok(v12) => {
                                    Ok(Ok::<(Adt12, List<Adt21>), Bytes>((Adt12::C0(v1, f283(v5), f283(v8), {
                                        let (h1688, _) = v12;
                                        h1688
                                    }, List::<Bytes>::nil(), f284(Bytes::lit(&[]))), {
                                        let (_, h1689) = v12;
                                        h1689
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

pub fn f277(v0: List<Adt21>, v1: Bytes) -> R<Result<(Adt12, List<Adt21>), Bytes>> {
    let m1690 = f279(v0);
    match m1690 {
        Ok(v3) => {
            let m1691 = f379(f281(v3.clone()))?;
            match m1691 {
                Ok(v5) => {
                    Ok(Ok::<(Adt12, List<Adt21>), Bytes>((Adt12::C0(9u64, f283(v3), Bytes::lit(&[]), List::<Adt13>::nil(), List::cons(v1, List::<Bytes>::nil()), f284(Bytes::lit(&[]))), v5)))
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

pub fn f278(v0: Str, v1: List<Adt21>) -> Bytes {
    f7(List::cons(f8(Str::lit("expected ")), List::cons({
        let a1692 = v0;
        utf8_encode(a1692)
    }, List::cons(f8(Str::lit(" but found ")), List::cons(f380(v1), List::cons(f8(Str::lit(" (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
}

pub fn f279(v0: List<Adt21>) -> Result<Adt28, Bytes> {
    let m1693 = v0.clone();
    match m1693.uncons() {
        None => {
            Err::<Adt28, Bytes>(f278(Str::lit("an identifier"), v0))
        }
        Some((v1, v2)) => {
            let a1694 = f373(v1.clone());
            let a1695 = 0u64;
            if nat_eq(a1694, a1695) {
                Ok::<Adt28, Bytes>(Adt28::C0(f381(f374(v1)), v2))
            } else {
                Err::<Adt28, Bytes>(f278(Str::lit("an identifier"), v0))
            }
        }
    }
}

pub fn f280(v0: List<Adt21>, v1: Str) -> Result<List<Adt21>, Bytes> {
    if f273(v0.clone(), v1.clone()) {
        Ok::<List<Adt21>, Bytes>(f275(v0))
    } else {
        Err::<List<Adt21>, Bytes>(f278(v1, v0))
    }
}

pub fn f281(v0: Adt28) -> List<Adt21> {
    let m1696 = v0;
    match m1696 {
        Adt28::C0(_, v2) => {
            v2
        }
    }
}

pub fn f282(p1704: u64, p1705: List<Adt21>, p1706: List<Bytes>) -> Result<(List<Bytes>, List<Adt21>), Bytes> {
    let m1703 = (p1704, (p1705, p1706));
    {
        let mut m1703 = m1703;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1703;
                let m1697 = v0;
                if m1697 == 0 {
                    Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Err::<(List<Bytes>, List<Adt21>), Bytes>(f8(Str::lit("an enumeration has more values than the input has tokens"))))
                } else {
                    let v3 = m1697 - 1;
                    let m1698 = v1.clone();
                    match m1698.uncons() {
                        None => {
                            Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Err::<(List<Bytes>, List<Adt21>), Bytes>(f278(Str::lit("a string"), v1)))
                        }
                        Some((v4, v5)) => {
                            let a1699 = f373(v4.clone());
                            let a1700 = 3u64;
                            if !nat_eq(a1699, a1700) {
                                Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Err::<(List<Bytes>, List<Adt21>), Bytes>(f278(Str::lit("a string"), v1)))
                            } else {
                                if f273(v5.clone(), Str::lit(",")) {
                                    if f273(f275(v5.clone()), Str::lit("}")) {
                                        let m1701 = f280(f275(f275(v5)), Str::lit(";"));
                                        match m1701 {
                                            Ok(v7) => {
                                                Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Ok::<(List<Bytes>, List<Adt21>), Bytes>((f45(List::cons(f374(v4), v2)), v7)))
                                            }
                                            Err(v6) => {
                                                Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Err::<(List<Bytes>, List<Adt21>), Bytes>(v6))
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>((v3, (f275(v5), List::cons(f374(v4), v2))))
                                    }
                                } else {
                                    if f273(v5.clone(), Str::lit("}")) {
                                        let m1702 = f280(f275(v5), Str::lit(";"));
                                        match m1702 {
                                            Ok(v9) => {
                                                Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Ok::<(List<Bytes>, List<Adt21>), Bytes>((f45(List::cons(f374(v4), v2)), v9)))
                                            }
                                            Err(v8) => {
                                                Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Err::<(List<Bytes>, List<Adt21>), Bytes>(v8))
                                            }
                                        }
                                    } else {
                                        Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Err::<(List<Bytes>, List<Adt21>), Bytes>(f278(Str::lit(", or }"), v5)))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1703 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f283(v0: Adt28) -> Bytes {
    let m1707 = v0;
    match m1707 {
        Adt28::C0(v1, _) => {
            v1
        }
    }
}

pub fn f284(v0: Bytes) -> Adt14 {
    Adt14::C0(0u64, v0, List::<Bytes>::nil(), Bytes::lit(&[]), Bytes::lit(&[]))
}

pub fn f285(v0: u64, v1: List<Adt21>, v2: u64) -> R<Result<Adt29, Bytes>> {
    let m1708 = v0;
    if m1708 == 0 {
        Ok(Err::<Adt29, Bytes>(f8(Str::lit("a type nests deeper than the input is long"))))
    } else {
        let v3 = m1708 - 1;
        let a1709 = v2;
        let a1710 = 0u64;
        if nat_eq(a1709, a1710) {
            let m1711 = f182(v1)?;
            match m1711 {
                Ok(v5) => {
                    if f273(v5.clone(), Str::lit("(")) {
                        let m1712 = f285(v3, f275(v5), 1u64)?;
                        match m1712 {
                            Ok(v7) => {
                                if f273(f286(v7.clone()), Str::lit("?")) {
                                    Ok(Ok::<Adt29, Bytes>(Adt29::C0(Adt14::C0(0u64, Bytes::lit(&[]), f202(f287(v7.clone())), f8(Str::lit("a nullable union type is not translated yet, because the typedef identifier names only the union (Web IDL 2.13.32)")), Bytes::lit(&[])), f275(f286(v7)))))
                                } else {
                                    Ok(Ok::<Adt29, Bytes>(Adt29::C0(f287(v7.clone()), f286(v7))))
                                }
                            }
                            Err(v6) => {
                                Ok(Err::<Adt29, Bytes>(v6))
                            }
                        }
                    } else {
                        let m1713 = v5.clone();
                        match m1713.uncons() {
                            None => {
                                Ok(Err::<Adt29, Bytes>(f278(Str::lit("a type"), v5)))
                            }
                            Some((v8, v9)) => {
                                let v10: Bytes = f374(v8.clone());
                                let a1714 = f373(v8);
                                let a1715 = 0u64;
                                if !nat_eq(a1714, a1715) {
                                    Ok(Err::<Adt29, Bytes>(f278(Str::lit("a type"), v5)))
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
                                        let m1716 = f280(v9, Str::lit("<"));
                                        match m1716 {
                                            Ok(v12) => {
                                                let m1717 = f285(v3, v12, 0u64)?;
                                                match m1717 {
                                                    Ok(v14) => {
                                                        let m1718 = f280(f286(v14.clone()), Str::lit(">"));
                                                        match m1718 {
                                                            Ok(v16) => {
                                                                Ok(f382(f383(Str::lit("List"), f287(v14)), v16))
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
                                            let m1719 = f280(v9, Str::lit("<"));
                                            match m1719 {
                                                Ok(v18) => {
                                                    let m1720 = f285(v3, v18, 0u64)?;
                                                    match m1720 {
                                                        Ok(v20) => {
                                                            let m1721 = f280(f286(v20), Str::lit(">"));
                                                            match m1721 {
                                                                Ok(v22) => {
                                                                    Ok(f382(f384(f8(Str::lit("async_sequence types are not translated, because an async sequence is produced over time and is not a value (Web IDL 2.13.29)"))), v22))
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
                                                let m1722 = f280(v9, Str::lit("<"));
                                                match m1722 {
                                                    Ok(v24) => {
                                                        let m1723 = f285(v3, v24, 0u64)?;
                                                        match m1723 {
                                                            Ok(v26) => {
                                                                let m1724 = f280(f286(v26), Str::lit(">"));
                                                                match m1724 {
                                                                    Ok(v28) => {
                                                                        Ok(f382(f384(f8(Str::lit("Promise types are not translated, because a promise is a deferred result, not a value (Web IDL 2.13.31)"))), v28))
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
                                                    let m1725 = f280(v9, Str::lit("<"));
                                                    match m1725 {
                                                        Ok(v30) => {
                                                            let m1726 = v30.clone();
                                                            match m1726.uncons() {
                                                                None => {
                                                                    Ok(Err::<Adt29, Bytes>(f278(Str::lit("a string type"), v30)))
                                                                }
                                                                Some((v31, v32)) => {
                                                                    if if f35(f374(v31.clone()), f8(Str::lit("DOMString"))) {
                                                                        true
                                                                    } else {
                                                                        if f35(f374(v31.clone()), f8(Str::lit("USVString"))) {
                                                                            true
                                                                        } else {
                                                                            f35(f374(v31.clone()), f8(Str::lit("ByteString")))
                                                                        }
                                                                    } {
                                                                        let m1727 = f280(v32, Str::lit(","));
                                                                        match m1727 {
                                                                            Ok(v34) => {
                                                                                let m1728 = f285(v3, v34, 0u64)?;
                                                                                match m1728 {
                                                                                    Ok(v36) => {
                                                                                        let m1729 = f280(f286(v36.clone()), Str::lit(">"));
                                                                                        match m1729 {
                                                                                            Ok(v38) => {
                                                                                                let m1730 = f385(f374(v31));
                                                                                                match m1730 {
                                                                                                    None => {
                                                                                                        Ok(Err::<Adt29, Bytes>(f278(Str::lit("a string type"), v30)))
                                                                                                    }
                                                                                                    Some(v39) => {
                                                                                                        let v40: Adt14 = f386(f287(v36));
                                                                                                        Ok(f382(Adt14::C0(0u64, f7(List::cons(f8(Str::lit("(List (")), List::cons(f201(v39), List::cons(f8(Str::lit(" * ")), List::cons(f201(v40.clone()), List::cons(f8(Str::lit("))")), List::<Bytes>::nil())))))), f202(v40.clone()), f199(v40), Bytes::lit(&[])), v38))
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
                                                                        Ok(Err::<Adt29, Bytes>(f278(Str::lit("DOMString, USVString or ByteString"), v30)))
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
                                                        if f273(v9.clone(), Str::lit("short")) {
                                                            Ok(f382(f284(f8(Str::lit("Nat"))), f275(v9)))
                                                        } else {
                                                            if f273(v9.clone(), Str::lit("long")) {
                                                                let v41: List<Adt21> = f275(v9);
                                                                Ok(f382(f284(f8(Str::lit("Nat"))), if f273(v41.clone(), Str::lit("long")) {
                                                                    f275(v41)
                                                                } else {
                                                                    v41
                                                                }))
                                                            } else {
                                                                Ok(Err::<Adt29, Bytes>(f278(Str::lit("short or long"), v9)))
                                                            }
                                                        }
                                                    } else {
                                                        if f35(v10.clone(), f8(Str::lit("short"))) {
                                                            Ok(f382(f284(f8(Str::lit("Int"))), v9))
                                                        } else {
                                                            if f35(v10.clone(), f8(Str::lit("long"))) {
                                                                Ok(f382(f284(f8(Str::lit("Int"))), if f273(v9.clone(), Str::lit("long")) {
                                                                    f275(v9)
                                                                } else {
                                                                    v9
                                                                }))
                                                            } else {
                                                                if f35(v10.clone(), f8(Str::lit("unrestricted"))) {
                                                                    if if f273(v9.clone(), Str::lit("float")) {
                                                                        true
                                                                    } else {
                                                                        f273(v9.clone(), Str::lit("double"))
                                                                    } {
                                                                        Ok(f382(f384(f8(Str::lit("unrestricted float and unrestricted double are not translated, because its value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.13 and 2.13.15)"))), f275(v9)))
                                                                    } else {
                                                                        Ok(Err::<Adt29, Bytes>(f278(Str::lit("float or double"), v9)))
                                                                    }
                                                                } else {
                                                                    let m1731 = f385(v10.clone());
                                                                    match m1731 {
                                                                        None => {
                                                                            let v42: Bytes = f381(v10);
                                                                            Ok(f382(Adt14::C0(0u64, f154(v42.clone())?, List::cons(v42.clone(), List::<Bytes>::nil()), Bytes::lit(&[]), v42), v9))
                                                                        }
                                                                        Some(v43) => {
                                                                            Ok(f382(v43, v9))
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
            let m1732 = f285(v3, v1, 0u64)?;
            match m1732 {
                Ok(v45) => {
                    let v46: Adt14 = f287(v45.clone());
                    let v48: Adt14 = {
                        let a1733 = f200(v46.clone());
                        let a1734 = 0u64;
                        if nat_eq(a1733, a1734) {
                            if f35(f199(v46.clone()), Bytes::lit(&[])) {
                                if f35(f387(v46.clone()), Bytes::lit(&[])) {
                                    Adt14::C0(1u64, Bytes::lit(&[]), f202(v46), f8(Str::lit("a union member type that is not a named definition gives its constructor no name, so the union is not translated yet (Web IDL 2.13.32)")), Bytes::lit(&[]))
                                } else {
                                    let m1735 = f302(false, f387(v46.clone()))?;
                                    match m1735 {
                                        None => {
                                            Adt14::C0(1u64, Bytes::lit(&[]), f202(v46.clone()), f7(List::cons(f8(Str::lit("the union member ")), List::cons(f387(v46), List::cons(f8(Str::lit(" has no constructor name")), List::<Bytes>::nil())))), Bytes::lit(&[]))
                                        }
                                        Some(v47) => {
                                            if f185(v47.clone()) {
                                                Adt14::C0(1u64, Bytes::lit(&[]), f202(v46), f186(v47), Bytes::lit(&[]))
                                            } else {
                                                Adt14::C0(1u64, f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v47, List::cons(f8(Str::lit(", fields: [")), List::cons(f201(v46.clone()), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f202(v46), Bytes::lit(&[]), Bytes::lit(&[]))
                                            }
                                        }
                                    }
                                }
                            } else {
                                Adt14::C0(1u64, Bytes::lit(&[]), f202(v46.clone()), f199(v46), Bytes::lit(&[]))
                            }
                        } else {
                            v46
                        }
                    };
                    if f273(f286(v45.clone()), Str::lit("or")) {
                        let m1736 = f285(v3, f275(f286(v45)), 2u64)?;
                        match m1736 {
                            Ok(v50) => {
                                Ok(Ok::<Adt29, Bytes>(Adt29::C0(Adt14::C0(1u64, if f35(f201(v48.clone()), Bytes::lit(&[])) {
                                    f201(f287(v50.clone()))
                                } else {
                                    if f35(f201(f287(v50.clone())), Bytes::lit(&[])) {
                                        f201(v48.clone())
                                    } else {
                                        f7(List::cons(f201(v48.clone()), List::cons(f8(Str::lit(",\u{a}")), List::cons(f201(f287(v50.clone())), List::<Bytes>::nil()))))
                                    }
                                }, f118(f202(v48.clone()), f202(f287(v50.clone()))), if f35(f199(v48.clone()), Bytes::lit(&[])) {
                                    f199(f287(v50.clone()))
                                } else {
                                    f199(v48)
                                }, Bytes::lit(&[])), f286(v50))))
                            }
                            Err(v49) => {
                                Ok(Err::<Adt29, Bytes>(v49))
                            }
                        }
                    } else {
                        let a1737 = v2;
                        let a1738 = 2u64;
                        if if nat_eq(a1737, a1738) {
                            f273(f286(v45.clone()), Str::lit(")"))
                        } else {
                            false
                        } {
                            Ok(Ok::<Adt29, Bytes>(Adt29::C0(v48, f275(f286(v45)))))
                        } else {
                            Ok(Err::<Adt29, Bytes>({
                                let a1739 = v2;
                                let a1740 = 1u64;
                                if nat_eq(a1739, a1740) {
                                    f278(Str::lit("or"), f286(v45))
                                } else {
                                    f278(Str::lit("or or )"), f286(v45))
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

pub fn f286(v0: Adt29) -> List<Adt21> {
    let m1741 = v0;
    match m1741 {
        Adt29::C0(_, v2) => {
            v2
        }
    }
}

pub fn f287(v0: Adt29) -> Adt14 {
    let m1742 = v0;
    match m1742 {
        Adt29::C0(v1, _) => {
            v1
        }
    }
}

pub fn f288() -> List<Bytes> {
    List::cons(f8(Str::lit("Prop")), List::cons(f8(Str::lit("Sort")), List::cons(f8(Str::lit("StateRefT")), List::cons(f8(Str::lit("Type")), List::cons(f8(Str::lit("abbrev")), List::cons(f8(Str::lit("at")), List::cons(f8(Str::lit("attribute")), List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("bif")), List::cons(f8(Str::lit("break")), List::cons(f8(Str::lit("by")), List::cons(f8(Str::lit("calc")), List::cons(f8(Str::lit("catch")), List::cons(f8(Str::lit("class")), List::cons(f8(Str::lit("coinductive")), List::cons(f8(Str::lit("continue")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f289() -> List<Bytes> {
    List::cons(f8(Str::lit("def")), List::cons(f8(Str::lit("deriving")), List::cons(f8(Str::lit("do")), List::cons(f8(Str::lit("dsimproc")), List::cons(f8(Str::lit("elab")), List::cons(f8(Str::lit("else")), List::cons(f8(Str::lit("end")), List::cons(f8(Str::lit("example")), List::cons(f8(Str::lit("exists")), List::cons(f8(Str::lit("export")), List::cons(f8(Str::lit("extends")), List::cons(f8(Str::lit("finally")), List::cons(f8(Str::lit("for")), List::cons(f8(Str::lit("forall")), List::cons(f8(Str::lit("from")), List::cons(f8(Str::lit("fun")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f290() -> List<Bytes> {
    List::cons(f8(Str::lit("generalizing")), List::cons(f8(Str::lit("have")), List::cons(f8(Str::lit("haveI")), List::cons(f8(Str::lit("hiding")), List::cons(f8(Str::lit("idbg")), List::cons(f8(Str::lit("if")), List::cons(f8(Str::lit("import")), List::cons(f8(Str::lit("in")), List::cons(f8(Str::lit("include")), List::cons(f8(Str::lit("inductive")), List::cons(f8(Str::lit("inferInstanceAs")), List::cons(f8(Str::lit("infix")), List::cons(f8(Str::lit("infixl")), List::cons(f8(Str::lit("infixr")), List::cons(f8(Str::lit("initialize")), List::cons(f8(Str::lit("instance")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f291() -> List<Bytes> {
    List::cons(f8(Str::lit("let")), List::cons(f8(Str::lit("letI")), List::cons(f8(Str::lit("local")), List::cons(f8(Str::lit("logNamedError")), List::cons(f8(Str::lit("logNamedErrorAt")), List::cons(f8(Str::lit("logNamedWarning")), List::cons(f8(Str::lit("logNamedWarningAt")), List::cons(f8(Str::lit("macro")), List::cons(f8(Str::lit("match")), List::cons(f8(Str::lit("matches")), List::cons(f8(Str::lit("meta")), List::cons(f8(Str::lit("mut")), List::cons(f8(Str::lit("mutual")), List::cons(f8(Str::lit("namespace")), List::cons(f8(Str::lit("nofun")), List::cons(f8(Str::lit("nomatch")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f292() -> List<Bytes> {
    List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("nonrec")), List::cons(f8(Str::lit("notation")), List::cons(f8(Str::lit("omit")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("open")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("postfix")), List::cons(f8(Str::lit("prefix")), List::cons(f8(Str::lit("private")), List::cons(f8(Str::lit("protected")), List::cons(f8(Str::lit("public")), List::cons(f8(Str::lit("renaming")), List::cons(f8(Str::lit("repeat")), List::cons(f8(Str::lit("reprove")), List::cons(f8(Str::lit("return")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f293() -> List<Bytes> {
    List::cons(f8(Str::lit("scoped")), List::cons(f8(Str::lit("seal")), List::cons(f8(Str::lit("section")), List::cons(f8(Str::lit("show")), List::cons(f8(Str::lit("simproc")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("structure")), List::cons(f8(Str::lit("suffices")), List::cons(f8(Str::lit("syntax")), List::cons(f8(Str::lit("then")), List::cons(f8(Str::lit("theorem")), List::cons(f8(Str::lit("throwError")), List::cons(f8(Str::lit("throwErrorAt")), List::cons(f8(Str::lit("throwNamedError")), List::cons(f8(Str::lit("throwNamedErrorAt")), List::cons(f8(Str::lit("try")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f294() -> List<Bytes> {
    List::cons(f8(Str::lit("universe")), List::cons(f8(Str::lit("unless")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("unseal")), List::cons(f8(Str::lit("until")), List::cons(f8(Str::lit("using")), List::cons(f8(Str::lit("variable")), List::cons(f8(Str::lit("where")), List::cons(f8(Str::lit("while")), List::cons(f8(Str::lit("with")), List::cons(f8(Str::lit("reprove")), List::cons(f8(Str::lit("throwError")), List::cons(f8(Str::lit("throwErrorAt")), List::<Bytes>::nil())))))))))))))
}

pub fn f295(v0: List<Adt13>) -> R<List<Adt13>> {
    let m1743 = v0;
    match m1743.uncons() {
        None => {
            Ok(List::<Adt13>::nil())
        }
        Some((v1, v2)) => {
            f388(v1, f295(v2)?)
        }
    }
}

pub fn f296(v0: List<Adt13>, v1: List<Adt13>) -> List<Adt13> {
    let m1744 = v0;
    match m1744.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f296(v3, v1))
        }
    }
}

pub fn f297(v0: Adt12) -> List<Adt13> {
    let m1745 = v0;
    match m1745 {
        Adt12::C0(_, _, _, v4, _, _) => {
            v4
        }
    }
}

pub fn f298(v0: List<Adt12>, v1: u64, v2: Bytes) -> List<Adt13> {
    let m1746 = v0;
    match m1746.uncons() {
        None => {
            List::<Adt13>::nil()
        }
        Some((v3, v4)) => {
            let a1747 = f112(v3.clone());
            let a1748 = v1;
            if if nat_eq(a1747, a1748) {
                f35(f114(v3.clone()), v2.clone())
            } else {
                false
            } {
                f296(f297(v3), f298(v4, v1, v2))
            } else {
                f298(v4, v1, v2)
            }
        }
    }
}

pub fn f299(v0: List<Adt12>, v1: List<Adt12>, v2: Bytes) -> List<Adt13> {
    let m1749 = v0;
    match m1749.uncons() {
        None => {
            List::<Adt13>::nil()
        }
        Some((v3, v4)) => {
            let a1750 = f112(v3.clone());
            let a1751 = 8u64;
            if if nat_eq(a1750, a1751) {
                f35(f114(v3.clone()), v2.clone())
            } else {
                false
            } {
                f296(f389(v1.clone(), f125(v3)), f299(v4, v1, v2))
            } else {
                f299(v4, v1, v2)
            }
        }
    }
}

pub fn f300(v0: Adt13) -> Adt14 {
    let m1752 = v0;
    match m1752 {
        Adt13::C0(_, v2, _) => {
            v2
        }
    }
}

pub fn f301(v0: Adt13) -> Bytes {
    let m1753 = v0;
    match m1753 {
        Adt13::C0(v1, _, _) => {
            v1
        }
    }
}

pub fn f302(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f390(v1)
    } else {
        f305(v1)
    }
}

pub fn f303(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1754 = v1;
    match m1754.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m1755 = f302(v0, v2)?;
            match m1755 {
                None => {
                    f303(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f303(v0, v3)?))
                }
            }
        }
    }
}

pub fn f304(v0: Adt13) -> u64 {
    let m1756 = v0;
    match m1756 {
        Adt13::C0(_, _, v3) => {
            v3
        }
    }
}

pub fn f305(v0: Bytes) -> R<Option<Bytes>> {
    let m1757 = f391(v0)?;
    match m1757 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            Ok(Some(f392(v1)?))
        }
    }
}

pub fn f306(v0: bool, v1: Adt19) -> R<Option<Bytes>> {
    let m1758 = v1;
    match m1758 {
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
                let a1769 = if f321(v7, {
                    let a1767 = 0u64;
                    let a1768 = v8;
                    nat_lt(a1767, a1768)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a1770 = f260(false, v8)?;
                append_bytes(a1769, a1770)
            }))
        }
        Adt19::C4(v3, v4, v5) => {
            let m1761 = f390({
                let a1759 = f8(Str::lit("n"));
                let a1760 = f261(false, v4, v5)?;
                append_bytes(a1759, a1760)
            })?;
            match m1761 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a1765 = f8(Str::lit("nMinus"));
                        let a1766 = f32(v6.clone(), 1u64, {
                            let a1763 = {
                                let a1762 = v6;
                                length_bytes(a1762)
                            };
                            let a1764 = 1u64;
                            nat_sub(a1763, a1764)
                        });
                        append_bytes(a1765, a1766)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt19::C5(v2) => {
            f302(v0, v2)
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

pub fn f307(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m1773 = {
        let a1771 = v0.clone();
        let a1772 = v1;
        index_bytes(a1771, a1772)
    };
    match m1773 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f42(v2)?;
            let a1774 = v3;
            let a1775 = 128u64;
            if nat_lt(a1774, a1775) {
                Ok(Some((v3, 1u64)))
            } else {
                if f321({
                    let a1776 = 194u64;
                    let a1777 = v3;
                    nat_le(a1776, a1777)
                }, {
                    let a1778 = v3;
                    let a1779 = 223u64;
                    nat_le(a1778, a1779)
                }) {
                    let m1782 = f393(v0, {
                        let a1780 = v1;
                        let a1781 = 1u64;
                        nat_add(a1780, a1781)?
                    })?;
                    match m1782 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f394({
                                let a1787 = {
                                    let a1785 = {
                                        let a1783 = v3;
                                        let a1784 = 192u64;
                                        nat_sub(a1783, a1784)
                                    };
                                    let a1786 = 64u64;
                                    nat_mul(a1785, a1786)?
                                };
                                let a1788 = v4;
                                nat_add(a1787, a1788)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f321({
                        let a1789 = 224u64;
                        let a1790 = v3;
                        nat_le(a1789, a1790)
                    }, {
                        let a1791 = v3;
                        let a1792 = 239u64;
                        nat_le(a1791, a1792)
                    }) {
                        let m1795 = f393(v0.clone(), {
                            let a1793 = v1;
                            let a1794 = 1u64;
                            nat_add(a1793, a1794)?
                        })?;
                        match m1795 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m1798 = f393(v0, {
                                    let a1796 = v1;
                                    let a1797 = 2u64;
                                    nat_add(a1796, a1797)?
                                })?;
                                match m1798 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f394({
                                            let a1807 = {
                                                let a1801 = {
                                                    let a1799 = v3;
                                                    let a1800 = 224u64;
                                                    nat_sub(a1799, a1800)
                                                };
                                                let a1802 = 4096u64;
                                                nat_mul(a1801, a1802)?
                                            };
                                            let a1808 = {
                                                let a1805 = {
                                                    let a1803 = v5;
                                                    let a1804 = 64u64;
                                                    nat_mul(a1803, a1804)?
                                                };
                                                let a1806 = v6;
                                                nat_add(a1805, a1806)?
                                            };
                                            nat_add(a1807, a1808)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f321({
                            let a1809 = 240u64;
                            let a1810 = v3;
                            nat_le(a1809, a1810)
                        }, {
                            let a1811 = v3;
                            let a1812 = 244u64;
                            nat_le(a1811, a1812)
                        }) {
                            let m1815 = f393(v0.clone(), {
                                let a1813 = v1;
                                let a1814 = 1u64;
                                nat_add(a1813, a1814)?
                            })?;
                            match m1815 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m1818 = f393(v0.clone(), {
                                        let a1816 = v1;
                                        let a1817 = 2u64;
                                        nat_add(a1816, a1817)?
                                    })?;
                                    match m1818 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m1821 = f393(v0, {
                                                let a1819 = v1;
                                                let a1820 = 3u64;
                                                nat_add(a1819, a1820)?
                                            })?;
                                            match m1821 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f394({
                                                        let a1834 = {
                                                            let a1824 = {
                                                                let a1822 = v3;
                                                                let a1823 = 240u64;
                                                                nat_sub(a1822, a1823)
                                                            };
                                                            let a1825 = 262144u64;
                                                            nat_mul(a1824, a1825)?
                                                        };
                                                        let a1835 = {
                                                            let a1832 = {
                                                                let a1826 = v7;
                                                                let a1827 = 4096u64;
                                                                nat_mul(a1826, a1827)?
                                                            };
                                                            let a1833 = {
                                                                let a1830 = {
                                                                    let a1828 = v8;
                                                                    let a1829 = 64u64;
                                                                    nat_mul(a1828, a1829)?
                                                                };
                                                                let a1831 = v9;
                                                                nat_add(a1830, a1831)?
                                                            };
                                                            nat_add(a1832, a1833)?
                                                        };
                                                        nat_add(a1834, a1835)?
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

pub fn f308(v0: u64) -> R<bool> {
    f207(v0, f395())
}

pub fn f309(v0: u64, v1: Bytes, p1846: u64, p1847: u64, p1848: bool) -> R<bool> {
    let m1845 = (p1846, (p1847, p1848));
    {
        let mut m1845 = m1845;
        loop {
            let step = {
                let (v2, (v3, v4)) = m1845;
                let m1836 = v3;
                if m1836 == 0 {
                    Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                } else {
                    let v5 = m1836 - 1;
                    let v6: u64 = f396(v1.clone(), v2)?;
                    if v4 {
                        let a1837 = v0;
                        let a1838 = v6;
                        if nat_lt(a1837, a1838) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a1839 = v2;
                                let a1840 = 1u64;
                                nat_add(a1839, a1840)?
                            }, (v5, false)))
                        }
                    } else {
                        let a1841 = v0;
                        let a1842 = v6;
                        if nat_le(a1841, a1842) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(true))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a1843 = v2;
                                let a1844 = 1u64;
                                nat_add(a1843, a1844)?
                            }, (v5, true)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1845 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f310(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    if f79(v0.clone(), v1, v2) {
        Ok(None::<u64>)
    } else {
        let a1850 = v2;
        let a1851 = {
            let a1849 = v0;
            length_bytes(a1849)
        };
        if nat_lt(a1850, a1851) {
            Ok(Some({
                let a1852 = v2;
                let a1853 = 1u64;
                nat_add(a1852, a1853)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f311(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f132(f87(v0, v1)?)? {
        Ok(Some({
            let a1854 = v1;
            let a1855 = 1u64;
            nat_add(a1854, a1855)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f312(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let m1856 = f307(v0, v2)?;
    match m1856 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            let a1857 = v2;
            let a1858 = v1;
            if if nat_eq(a1857, a1858) {
                f397({
                    let (h1859, _) = v3;
                    h1859
                })?
            } else {
                f398({
                    let (h1860, _) = v3;
                    h1860
                })?
            } {
                Ok(Some({
                    let a1862 = v2;
                    let a1863 = {
                        let (_, h1861) = v3;
                        h1861
                    };
                    nat_add(a1862, a1863)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f313(v0: Bytes, v1: bool, v2: u64) -> R<(u64, u64)> {
    let (h1864, _) = f399(Fn7::F400(v1, v0.clone()), {
        let a1866 = {
            let a1865 = v0;
            length_bytes(a1865)
        };
        let a1867 = 1u64;
        nat_add(a1866, a1867)?
    }, (v2, 0u64))?;
    Ok(h1864)
}

pub fn f314(v0: u64) -> R<Bytes> {
    let a1868 = v0;
    let a1869 = 128u64;
    if nat_lt(a1868, a1869) {
        Ok(f401(f402(), v0))
    } else {
        let a1870 = v0;
        let a1871 = 2048u64;
        if nat_lt(a1870, a1871) {
            let a1879 = f401(f403(), {
                let a1877 = {
                    let a1875 = {
                        let a1872 = v0;
                        let a1873 = 64u64;
                        let a1874 = 0u64;
                        nat_quot(a1872, a1873, a1874)
                    };
                    let a1876 = 2u64;
                    nat_sub(a1875, a1876)
                };
                let a1878 = 2u64;
                nat_mul(a1877, a1878)?
            });
            let a1880 = f404(v0)?;
            Ok(append_bytes(a1879, a1880))
        } else {
            let a1881 = v0;
            let a1882 = 65536u64;
            if nat_lt(a1881, a1882) {
                let a1893 = f401(f405(), {
                    let a1886 = {
                        let a1883 = v0;
                        let a1884 = 4096u64;
                        let a1885 = 0u64;
                        nat_quot(a1883, a1884, a1885)
                    };
                    let a1887 = 3u64;
                    nat_mul(a1886, a1887)?
                });
                let a1894 = {
                    let a1891 = f404({
                        let a1888 = v0;
                        let a1889 = 64u64;
                        let a1890 = 0u64;
                        nat_quot(a1888, a1889, a1890)
                    })?;
                    let a1892 = f404(v0)?;
                    append_bytes(a1891, a1892)
                };
                Ok(append_bytes(a1893, a1894))
            } else {
                let a1908 = f401(Bytes::lit(&[0xf0, 0xf1, 0xf2, 0xf3, 0xf4]), {
                    let a1895 = v0;
                    let a1896 = 262144u64;
                    let a1897 = 0u64;
                    nat_quot(a1895, a1896, a1897)
                });
                let a1909 = {
                    let a1906 = f404({
                        let a1898 = v0;
                        let a1899 = 4096u64;
                        let a1900 = 0u64;
                        nat_quot(a1898, a1899, a1900)
                    })?;
                    let a1907 = {
                        let a1904 = f404({
                            let a1901 = v0;
                            let a1902 = 64u64;
                            let a1903 = 0u64;
                            nat_quot(a1901, a1902, a1903)
                        })?;
                        let a1905 = f404(v0)?;
                        append_bytes(a1904, a1905)
                    };
                    append_bytes(a1906, a1907)
                };
                Ok(append_bytes(a1908, a1909))
            }
        }
    }
}

pub fn f315(v0: bool, v1: u64, v2: Bytes, v3: u64) -> R<Option<u64>> {
    f406(v2, v0, v1, v3)
}

pub fn f316(v0: Bytes) -> Result<Option<Adt27>, Bytes> {
    Err::<Option<Adt27>, Bytes>(v0)
}

pub fn f317(p1913: Bytes, p1914: u64) -> R<Result<Bytes, Bytes>> {
    let m1912 = (p1913, p1914);
    {
        let mut m1912 = m1912;
        loop {
            let step = {
                let (v0, v1) = m1912;
                let m1910 = v1;
                if m1910 == 0 {
                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f407(f8(Str::lit("the built-in datatype hierarchy is deeper than XSD 1.1 Part 2 section 3 defines")))))
                } else {
                    let v2 = m1910 - 1;
                    if f211(v0.clone(), List::cons(f8(Str::lit("NMTOKENS")), List::cons(f8(Str::lit("IDREFS")), List::cons(f8(Str::lit("ENTITIES")), List::<Bytes>::nil())))) {
                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("(List Bytes)")))))
                    } else {
                        if f35(v0.clone(), f8(Str::lit("nonNegativeInteger"))) {
                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Nat")))))
                        } else {
                            if f35(v0.clone(), f8(Str::lit("integer"))) {
                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Int")))))
                            } else {
                                if f35(v0.clone(), f8(Str::lit("decimal"))) {
                                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f408())))
                                } else {
                                    if f35(v0.clone(), f8(Str::lit("boolean"))) {
                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Bool")))))
                                    } else {
                                        if f211(v0.clone(), List::cons(f8(Str::lit("float")), List::cons(f8(Str::lit("double")), List::<Bytes>::nil()))) {
                                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f407(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because its value space holds positive and negative zero, the infinities and NaN, which no exact LexLean number type holds (XSD 1.1 Part 2, sections 3.3.4 and 3.3.5)")), List::<Bytes>::nil())))))))
                                        } else {
                                            if f211(v0.clone(), List::cons(f8(Str::lit("anySimpleType")), List::cons(f8(Str::lit("anyAtomicType")), List::<Bytes>::nil()))) {
                                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f407(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because it is the union of every simple value space and has no one LexLean type (XSD 1.1 Part 2, sections 3.2.1 and 3.2.2)")), List::<Bytes>::nil())))))))
                                            } else {
                                                let m1911 = f409(v0.clone())?;
                                                match m1911 {
                                                    None => {
                                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f407(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not a built-in datatype of XSD 1.1 Part 2")), List::<Bytes>::nil())))))))
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
                Ok(next) => { m1912 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f318(v0: Bytes) -> Result<List<Adt19>, Bytes> {
    Err::<List<Adt19>, Bytes>(v0)
}

pub fn f319(v0: Option<Adt19>) -> Option<Bytes> {
    let m1915 = v0;
    match m1915 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m1916 = v1;
            match m1916 {
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

pub fn f320(v0: u64) -> bool {
    f323({
        let a1917 = v0;
        let a1918 = 95u64;
        nat_eq(a1917, a1918)
    }, f323({
        let a1919 = v0;
        let a1920 = 45u64;
        nat_eq(a1919, a1920)
    }, f323({
        let a1921 = v0;
        let a1922 = 32u64;
        nat_eq(a1921, a1922)
    }, f323({
        let a1923 = v0;
        let a1924 = 46u64;
        nat_eq(a1923, a1924)
    }, {
        let a1925 = v0;
        let a1926 = 47u64;
        nat_eq(a1925, a1926)
    }))))
}

pub fn f321(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f322(v0: u64) -> bool {
    f321({
        let a1927 = 97u64;
        let a1928 = v0;
        nat_le(a1927, a1928)
    }, {
        let a1929 = v0;
        let a1930 = 122u64;
        nat_le(a1929, a1930)
    })
}

pub fn f323(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f324(v0: u64) -> bool {
    f321({
        let a1931 = 65u64;
        let a1932 = v0;
        nat_le(a1931, a1932)
    }, {
        let a1933 = v0;
        let a1934 = 90u64;
        nat_le(a1933, a1934)
    })
}

pub fn f325(v0: u64) -> bool {
    f321({
        let a1935 = 48u64;
        let a1936 = v0;
        nat_le(a1935, a1936)
    }, {
        let a1937 = v0;
        let a1938 = 57u64;
        nat_le(a1937, a1938)
    })
}

pub fn f326(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h1939, _) = v1.clone();
        h1939
    };
    let v3: u64 = {
        let (h1940, _) = {
            let (_, h1941) = v1.clone();
            h1941
        };
        h1940
    };
    let v4: List<Bytes> = {
        let (_, h1942) = {
            let (_, h1943) = v1;
            h1943
        };
        h1942
    };
    let m1944 = f106(v0.clone(), v2);
    match m1944 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a1945 = v5;
            let a1946 = 10u8;
            if equal(a1945, a1946) {
                Ok(Some(({
                    let a1947 = v2;
                    let a1948 = 1u64;
                    nat_add(a1947, a1948)?
                }, ({
                    let a1949 = v2;
                    let a1950 = 1u64;
                    nat_add(a1949, a1950)?
                }, List::cons(f410(f32(v0, v3, {
                    let a1951 = v2;
                    let a1952 = v3;
                    nat_sub(a1951, a1952)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a1953 = v2;
                    let a1954 = 1u64;
                    nat_add(a1953, a1954)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f327(v0: List<Adt24>, v1: List<Adt24>) -> List<Adt24> {
    let m1955 = v0;
    match m1955.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f327(v3, v1))
        }
    }
}

pub fn f328(v0: List<Adt25>, v1: Bytes) -> List<Adt24> {
    let m1956 = v0;
    match m1956.uncons() {
        None => {
            List::<Adt24>::nil()
        }
        Some((v2, v3)) => {
            if f115(match v2.clone() {
                Adt25::C0(_, h1957, _, _, _) => {
                    h1957
                }
            }, f411()) {
                let m1958 = f241(v2.clone(), Str::lit("name"));
                match m1958 {
                    None => {
                        f328(v3, v1)
                    }
                    Some(v4) => {
                        List::cons(Adt24::C0(match v2.clone() {
                            Adt25::C0(_, h1959, _, _, _) => {
                                h1959
                            }
                        }, v1.clone(), v4, v2), f328(v3, v1))
                    }
                }
            } else {
                f328(v3, v1)
            }
        }
    }
}

pub fn f329(v0: u64, v1: Adt23, v2: Adt24) -> R<Result<List<Adt26>, Bytes>> {
    let v3: Bytes = f154(match v2.clone() {
        Adt24::C0(_, _, h1960, _) => {
            h1960
        }
    })?;
    let v4: Adt25 = match v2.clone() {
        Adt24::C0(_, _, _, h1961) => {
            h1961
        }
    };
    let v5: Bytes = match v2.clone() {
        Adt24::C0(_, h1962, _, _) => {
            h1962
        }
    };
    let v6: Bytes = match v2.clone() {
        Adt24::C0(h1963, _, _, _) => {
            h1963
        }
    };
    if f35(v6.clone(), f8(Str::lit("complexType"))) {
        f412(v0, v1, v2)
    } else {
        if f35(v6.clone(), f8(Str::lit("simpleType"))) {
            let m1964 = f413(v3, v4, v5)?;
            match m1964 {
                Ok(v8) => {
                    Ok(Ok::<List<Adt26>, Bytes>(List::cons(v8, List::<Adt26>::nil())))
                }
                Err(v7) => {
                    Ok(Err::<List<Adt26>, Bytes>(v7))
                }
            }
        } else {
            if f35(v6.clone(), f8(Str::lit("element"))) {
                let m1965 = f241(v4.clone(), Str::lit("type"));
                match m1965 {
                    None => {
                        let m1966 = f414(v0, v1, v3, v5, v4)?;
                        match m1966 {
                            Ok(v11) => {
                                Ok(Ok::<List<Adt26>, Bytes>(match v11 {
                                    Adt31::C0(_, h1967) => {
                                        h1967
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
                    let m1968 = f241(v4.clone(), Str::lit("type"));
                    match m1968 {
                        None => {
                            let m1969 = f415(v1, v5, v3, v4)?;
                            match m1969 {
                                Ok(v14) => {
                                    Ok(Ok::<List<Adt26>, Bytes>(match v14 {
                                        Adt31::C0(_, h1970) => {
                                            h1970
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
                        let m1971 = f416(v4)?;
                        match m1971 {
                            Ok(v16) => {
                                let m1972 = f417(v16, List::cons(f8(Str::lit("sequence")), List::cons(f8(Str::lit("choice")), List::cons(f8(Str::lit("all")), List::<Bytes>::nil()))));
                                match m1972 {
                                    None => {
                                        Ok(Err::<List<Adt26>, Bytes>(f418(f8(Str::lit("a model group definition must have a sequence, choice or all child (XSD 1.1 Part 1, section 3.7.2)")), v3)))
                                    }
                                    Some(v17) => {
                                        f419(v0, v1, v3, v5, v17)
                                    }
                                }
                            }
                            Err(v15) => {
                                Ok(Err::<List<Adt26>, Bytes>(v15))
                            }
                        }
                    } else {
                        let m1973 = f416(v4)?;
                        match m1973 {
                            Ok(v19) => {
                                let m1974 = f420(v0, v1, v3, v5, v19, f421())?;
                                match m1974 {
                                    Ok(v21) => {
                                        Ok(Ok::<List<Adt26>, Bytes>(match v21 {
                                            Adt32::C0(_, h1975, _, _, _) => {
                                                h1975
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

pub fn f330(v0: List<Adt26>, v1: List<Adt26>) -> List<Adt26> {
    let m1976 = v0;
    match m1976.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f330(v3, v1))
        }
    }
}

pub fn f331(p1979: List<Adt26>, p1980: List<Adt26>) -> List<Adt26> {
    let m1978 = (p1979, p1980);
    {
        let mut m1978 = m1978;
        loop {
            let step = {
                let (v0, v1) = m1978;
                let m1977 = v0;
                match m1977.uncons() {
                    None => {
                        Err::<(List<Adt26>, List<Adt26>), List<Adt26>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt26>, List<Adt26>), List<Adt26>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1978 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f332(p1984: List<Adt26>, v1: Bytes) -> Option<Adt26> {
    let m1983 = p1984;
    {
        let mut m1983 = m1983;
        loop {
            let step = {
                let v0 = m1983;
                let m1981 = v0;
                match m1981.uncons() {
                    None => {
                        Err::<List<Adt26>, Option<Adt26>>(None::<Adt26>)
                    }
                    Some((v2, v3)) => {
                        if f35(match v2.clone() {
                            Adt26::C0(h1982, _, _, _) => {
                                h1982
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
                Ok(next) => { m1983 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f333() -> Bytes {
    f8(Str::lit("XmlNode"))
}

pub fn f334(v0: List<Adt26>) -> List<Bytes> {
    let m1985 = v0;
    match m1985.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f340(match v1 {
                Adt26::C0(_, h1986, _, _) => {
                    h1986
                }
            }, f334(v2))
        }
    }
}

pub fn f335() -> List<Bytes> {
    List::cons(f8(Str::lit("    {name: element, fields: [Bytes, Bytes, (List ((Bytes * Bytes) * Bytes)), (List XmlNode)]}")), List::cons(f8(Str::lit("    {name: text, fields: [Bytes]}")), List::<Bytes>::nil()))
}

pub fn f336(p1990: u64, v1: List<Adt26>, p1991: List<Bytes>, p1992: List<Bytes>) -> List<Bytes> {
    let m1989 = (p1990, (p1991, p1992));
    {
        let mut m1989 = m1989;
        loop {
            let step = {
                let (v0, (v2, v3)) = m1989;
                let m1987 = v0;
                if m1987 == 0 {
                    Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v3)
                } else {
                    let v4 = m1987 - 1;
                    let m1988 = v2;
                    match m1988.uncons() {
                        None => {
                            Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v3)
                        }
                        Some((v5, v6)) => {
                            if f115(v5.clone(), v3.clone()) {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v3)))
                            } else {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (f340(f337(v1.clone(), v5.clone()), v6), List::cons(v5, v3))))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1989 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f337(v0: List<Adt26>, v1: Bytes) -> List<Bytes> {
    let m1993 = f332(v0, v1);
    match m1993 {
        None => {
            List::<Bytes>::nil()
        }
        Some(v2) => {
            match v2 {
                Adt26::C0(_, h1994, _, _) => {
                    h1994
                }
            }
        }
    }
}

pub fn f338(v0: List<(List<Bytes>, List<Bytes>)>, v1: List<Bytes>) -> Option<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)> {
    let m1995 = v0;
    match m1995.uncons() {
        None => {
            None::<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)>
        }
        Some((v2, v3)) => {
            let m1997 = f155({
                let (_, h1996) = v2.clone();
                h1996
            }, v1.clone());
            match m1997 {
                None => {
                    Some(({
                        let (h1998, _) = v2;
                        h1998
                    }, v3))
                }
                Some(_) => {
                    let m1999 = f338(v3, v1);
                    match m1999 {
                        None => {
                            None::<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h2000, _) = v5;
                                h2000
                            }, List::cons(v2, {
                                let (_, h2001) = v5;
                                h2001
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f339(v0: List<Adt26>, v1: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m2002 = v1.clone();
    match m2002.uncons() {
        None => {
            Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m2003 = v3;
            match m2003.uncons() {
                None => {
                    let m2006 = f332(v0, v2.clone());
                    match m2006 {
                        None => {
                            Err::<List<Bytes>, Bytes>(f423(v2))
                        }
                        Some(v6) => {
                            if f115(v2.clone(), match v6.clone() {
                                Adt26::C0(_, h2007, _, _) => {
                                    h2007
                                }
                            }) {
                                let m2009 = match v6.clone() {
                                    Adt26::C0(_, _, _, h2008) => {
                                        h2008
                                    }
                                };
                                match m2009.uncons() {
                                    None => {
                                        Err::<List<Bytes>, Bytes>(f423(v2))
                                    }
                                    Some((_, _)) => {
                                        Ok::<List<Bytes>, Bytes>(List::cons(f197(v2, match v6 {
                                            Adt26::C0(_, _, _, h2010) => {
                                                h2010
                                            }
                                        }), List::<Bytes>::nil()))
                                    }
                                }
                            } else {
                                Ok::<List<Bytes>, Bytes>(List::cons(match v6 {
                                    Adt26::C0(_, _, h2011, _) => {
                                        h2011
                                    }
                                }, List::<Bytes>::nil()))
                            }
                        }
                    }
                }
                Some((_, _)) => {
                    f422(v0, v1, {
                        let a2004 = v2;
                        let a2005 = f8(Str::lit("Group"));
                        append_bytes(a2004, a2005)
                    })
                }
            }
        }
    }
}

pub fn f340(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m2012 = v0;
    match m2012.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f340(v3, v1))
        }
    }
}

pub fn f341(v0: List<Adt26>, v1: List<Bytes>, v2: List<Bytes>) -> List<Bytes> {
    let m2013 = v1;
    match m2013.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            f340(f424(f337(v0.clone(), v3), v2.clone()), f341(v0, v4, v2))
        }
    }
}

pub fn f342(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>, v2: Bytes) -> List<Bytes> {
    let m2014 = v1;
    match m2014.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if f35(v3.clone(), v2.clone()) {
                true
            } else {
                if f115(v3.clone(), f425(v0.clone(), v2.clone())) {
                    f115(v2.clone(), f425(v0.clone(), v3.clone()))
                } else {
                    false
                }
            } {
                List::cons(v3, f342(v0, v4, v2))
            } else {
                f342(v0, v4, v2)
            }
        }
    }
}

pub fn f343(v0: Bytes, p2023: u64, p2024: u64, p2025: Option<u64>) -> R<Option<u64>> {
    let m2022 = (p2023, (p2024, p2025));
    {
        let mut m2022 = m2022;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2022;
                let m2015 = v2;
                if m2015 == 0 {
                    Err::<(u64, (u64, Option<u64>)), R<Option<u64>>>(Ok(v3))
                } else {
                    let v4 = m2015 - 1;
                    let v5: u64 = f87(v0.clone(), v1)?;
                    let a2016 = v5;
                    let a2017 = 256u64;
                    if nat_eq(a2016, a2017) {
                        Err::<(u64, (u64, Option<u64>)), R<Option<u64>>>(Ok(v3))
                    } else {
                        Ok::<(u64, (u64, Option<u64>)), R<Option<u64>>>(({
                            let a2018 = v1;
                            let a2019 = 1u64;
                            nat_add(a2018, a2019)?
                        }, (v4, {
                            let a2020 = v5;
                            let a2021 = 47u64;
                            if nat_eq(a2020, a2021) {
                                Some(v1)
                            } else {
                                v3
                            }
                        })))
                    }
                }
            };
            match step {
                Ok(next) => { m2022 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f344(v0: u64) -> bool {
    let a2026 = v0;
    let a2027 = 9u64;
    if if if nat_eq(a2026, a2027) {
        true
    } else {
        let a2028 = v0;
        let a2029 = 10u64;
        nat_eq(a2028, a2029)
    } {
        true
    } else {
        let a2030 = v0;
        let a2031 = 13u64;
        if nat_eq(a2030, a2031) {
            true
        } else {
            let a2032 = 32u64;
            let a2033 = v0;
            if nat_le(a2032, a2033) {
                let a2034 = v0;
                let a2035 = 126u64;
                nat_le(a2034, a2035)
            } else {
                false
            }
        }
    } {
        true
    } else {
        let a2036 = v0;
        let a2037 = 133u64;
        if if nat_eq(a2036, a2037) {
            true
        } else {
            let a2038 = 160u64;
            let a2039 = v0;
            if nat_le(a2038, a2039) {
                let a2040 = v0;
                let a2041 = 55295u64;
                nat_le(a2040, a2041)
            } else {
                false
            }
        } {
            true
        } else {
            let a2042 = 57344u64;
            let a2043 = v0;
            if if nat_le(a2042, a2043) {
                let a2044 = v0;
                let a2045 = 65533u64;
                nat_le(a2044, a2045)
            } else {
                false
            } {
                true
            } else {
                let a2046 = 65536u64;
                let a2047 = v0;
                if nat_le(a2046, a2047) {
                    let a2048 = v0;
                    let a2049 = 1114111u64;
                    nat_le(a2048, a2049)
                } else {
                    false
                }
            }
        }
    }
}

pub fn f345(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f249(v0.clone(), v1)?;
    if f174(f173(v0.clone(), v2)?) {
        Ok(true)
    } else {
        let a2050 = f173(v0, v2)?;
        let a2051 = 35u64;
        Ok(nat_eq(a2050, a2051))
    }
}

pub fn f346(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f426(v0, v1)
}

pub fn f347(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f427(v0, v1)
}

pub fn f348(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f428(v0, v1)
}

pub fn f349(v0: u64, v1: bool) -> bool {
    f178(if if f172(v0) {
        true
    } else {
        f174(v0)
    } {
        true
    } else {
        if v1 {
            f429(v0)
        } else {
            false
        }
    })
}

pub fn f350(v0: Bytes, v1: u64, v2: u64) -> R<Adt20> {
    let v3: bool = {
        let a2052 = f173(v0.clone(), v1)?;
        let a2053 = 62u64;
        nat_eq(a2052, a2053)
    };
    let v4: u64 = f173(v0.clone(), {
        let a2054 = v1;
        let a2055 = 1u64;
        nat_add(a2054, a2055)?
    })?;
    let v5: u64 = f173(v0.clone(), {
        let a2056 = v1;
        let a2057 = 2u64;
        nat_add(a2056, a2057)?
    })?;
    let v6: u64 = f430(v4);
    let v7: u64 = f431(v4);
    let v8: bool = {
        let a2058 = 0u64;
        let a2059 = v6;
        if nat_lt(a2058, a2059) {
            true
        } else {
            let a2060 = 0u64;
            let a2061 = v7;
            nat_lt(a2060, a2061)
        }
    };
    let v9: u64 = {
        let a2062 = 0u64;
        let a2063 = v7;
        if nat_lt(a2062, a2063) {
            f430(v5)
        } else {
            0u64
        }
    };
    let v10: u64 = {
        let a2064 = 0u64;
        let a2065 = v6;
        if nat_lt(a2064, a2065) {
            f431(v5)
        } else {
            0u64
        }
    };
    let v11: u64 = {
        let a2066 = v6;
        let a2067 = v9;
        nat_add(a2066, a2067)?
    };
    let v12: u64 = {
        let a2068 = v7;
        let a2069 = v10;
        nat_add(a2068, a2069)?
    };
    let v13: u64 = {
        let a2078 = {
            let a2070 = v1;
            let a2071 = 1u64;
            nat_add(a2070, a2071)?
        };
        let a2079 = {
            let a2076 = if v8 {
                1u64
            } else {
                0u64
            };
            let a2077 = {
                let a2072 = 0u64;
                let a2073 = v9;
                if if nat_lt(a2072, a2073) {
                    true
                } else {
                    let a2074 = 0u64;
                    let a2075 = v10;
                    nat_lt(a2074, a2075)
                } {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a2076, a2077)?
        };
        nat_add(a2078, a2079)?
    };
    let m2080 = f259(v0.clone(), v13)?;
    match m2080 {
        Ok(v15) => {
            let a2081 = 0u64;
            let a2082 = v12;
            if nat_lt(a2081, a2082) {
                f432(v0, v15, {
                    let a2085 = {
                        let a2083 = v2;
                        let a2084 = v12;
                        nat_add(a2083, a2084)?
                    };
                    let a2086 = 1u64;
                    nat_sub(a2085, a2086)
                }, v3, v11)
            } else {
                let v16: (u64, u64) = {
                    let (h2087, _) = f399(Fn7::F433(v0.clone()), {
                        let a2089 = {
                            let a2088 = v0.clone();
                            length_bytes(a2088)
                        };
                        let a2090 = 1u64;
                        nat_add(a2089, a2090)?
                    }, (v15, 0u64))?;
                    h2087
                };
                let v17: u64 = f248(v0.clone(), {
                    let (h2091, _) = v16;
                    h2091
                })?;
                let a2092 = f173(v0.clone(), v17)?;
                let a2093 = 256u64;
                if nat_eq(a2092, a2093) {
                    Ok(Adt20::C0(Adt19::C5({
                        let a2094 = v11;
                        let a2095 = 2u64;
                        if nat_eq(a2094, a2095) {
                            f434(f435(v0.clone(), v15)?)
                        } else {
                            Bytes::lit(&[])
                        }
                    }), {
                        let a2096 = v0;
                        length_bytes(a2096)
                    }))
                } else {
                    let v18: u64 = f436({
                        let a2098 = v17;
                        let a2099 = {
                            let (h2097, _) = v16;
                            h2097
                        };
                        nat_sub(a2098, a2099)
                    }, v2);
                    let a2101 = v18;
                    let a2102 = {
                        let (_, h2100) = v16;
                        h2100
                    };
                    if nat_lt(a2101, a2102) {
                        Ok(Adt20::C1(f83(Str::lit("a leading empty line of a block scalar holds more spaces than its first content line (YAML 1.2.2 section 8.1.1.1)"), v15)?))
                    } else {
                        f432(v0, v15, v18, v3, v11)
                    }
                }
            }
        }
        Err(v14) => {
            Ok(Adt20::C1(v14))
        }
    }
}

pub fn f351(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Adt30> {
    let a2103 = f173(v0.clone(), v1)?;
    let a2104 = 34u64;
    if nat_eq(a2103, a2104) {
        f437(v0.clone(), {
            let a2105 = v1;
            let a2106 = 1u64;
            nat_add(a2105, a2106)?
        }, v2, v3, {
            let a2108 = {
                let a2107 = v0;
                length_bytes(a2107)
            };
            let a2109 = 2u64;
            nat_add(a2108, a2109)?
        }, Bytes::lit(&[]))
    } else {
        f438(v0.clone(), {
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
    }
}

pub fn f352(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: List<Bytes>) -> R<Adt20> {
    let m2115 = v3;
    if m2115 == 0 {
        Ok(Adt20::C1(f83(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v6 = m2115 - 1;
        let a2116 = v4;
        let a2117 = 1u64;
        if nat_eq(a2116, a2117) {
            let m2118 = f352(v0.clone(), v1, v2, v6, 0u64, List::<Bytes>::nil())?;
            match m2118 {
                Adt20::C0(v8, v9) => {
                    let m2119 = f439(v0.clone(), v9, v2)?;
                    match m2119 {
                        Ok(v11) => {
                            let v12: u64 = f173(v0.clone(), v11)?;
                            let a2120 = v12;
                            let a2121 = 58u64;
                            if nat_eq(a2120, a2121) {
                                Ok(Adt20::C1(f83(Str::lit("a single-pair mapping inside a flow sequence is not read (YAML 1.2.2 [150])"), v11)?))
                            } else {
                                let a2122 = v12;
                                let a2123 = 93u64;
                                if nat_eq(a2122, a2123) {
                                    Ok(Adt20::C0(Adt19::C9(std::rc::Rc::new(v8), std::rc::Rc::new(Adt19::C8)), {
                                        let a2124 = v11;
                                        let a2125 = 1u64;
                                        nat_add(a2124, a2125)?
                                    }))
                                } else {
                                    let a2126 = v12;
                                    let a2127 = 44u64;
                                    if nat_eq(a2126, a2127) {
                                        let m2130 = f439(v0.clone(), {
                                            let a2128 = v11;
                                            let a2129 = 1u64;
                                            nat_add(a2128, a2129)?
                                        }, v2)?;
                                        match m2130 {
                                            Ok(v14) => {
                                                let a2131 = f173(v0.clone(), v14)?;
                                                let a2132 = 93u64;
                                                if nat_eq(a2131, a2132) {
                                                    Ok(Adt20::C0(Adt19::C9(std::rc::Rc::new(v8), std::rc::Rc::new(Adt19::C8)), {
                                                        let a2133 = v14;
                                                        let a2134 = 1u64;
                                                        nat_add(a2133, a2134)?
                                                    }))
                                                } else {
                                                    let m2135 = f352(v0, v14, v2, v6, 1u64, List::<Bytes>::nil())?;
                                                    match m2135 {
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
            let a2136 = v4;
            let a2137 = 2u64;
            if nat_eq(a2136, a2137) {
                let m2138 = f440(v0.clone(), v1, v2)?;
                match m2138 {
                    Adt30::C0(v19, v20) => {
                        let m2139 = f439(v0.clone(), v20, v2)?;
                        match m2139 {
                            Ok(v22) => {
                                if f178({
                                    let a2140 = f173(v0.clone(), v22)?;
                                    let a2141 = 58u64;
                                    nat_eq(a2140, a2141)
                                }) {
                                    Ok(Adt20::C1(f83(Str::lit("a flow mapping entry needs : and a value; a null value is not read (YAML 1.2.2 [142])"), v22)?))
                                } else {
                                    let m2144 = f439(v0.clone(), {
                                        let a2142 = v22;
                                        let a2143 = 1u64;
                                        nat_add(a2142, a2143)?
                                    }, v2)?;
                                    match m2144 {
                                        Ok(v24) => {
                                            let a2145 = f173(v0.clone(), v24)?;
                                            let a2146 = 44u64;
                                            if if nat_eq(a2145, a2146) {
                                                true
                                            } else {
                                                let a2147 = f173(v0.clone(), v24)?;
                                                let a2148 = 125u64;
                                                nat_eq(a2147, a2148)
                                            } {
                                                Ok(Adt20::C1(f256(v24)?))
                                            } else {
                                                let m2149 = f352(v0.clone(), v24, v2, v6, 0u64, List::<Bytes>::nil())?;
                                                match m2149 {
                                                    Adt20::C0(v26, v27) => {
                                                        if f211(v19.clone(), v5.clone()) {
                                                            Ok(Adt20::C1(f83(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                                        } else {
                                                            let m2150 = f439(v0.clone(), v27, v2)?;
                                                            match m2150 {
                                                                Ok(v29) => {
                                                                    let v30: u64 = f173(v0.clone(), v29)?;
                                                                    let a2151 = v30;
                                                                    let a2152 = 125u64;
                                                                    if nat_eq(a2151, a2152) {
                                                                        Ok(Adt20::C0(Adt19::C10(v19, std::rc::Rc::new(v26), std::rc::Rc::new(Adt19::C8)), {
                                                                            let a2153 = v29;
                                                                            let a2154 = 1u64;
                                                                            nat_add(a2153, a2154)?
                                                                        }))
                                                                    } else {
                                                                        let a2155 = v30;
                                                                        let a2156 = 44u64;
                                                                        if nat_eq(a2155, a2156) {
                                                                            let m2159 = f439(v0.clone(), {
                                                                                let a2157 = v29;
                                                                                let a2158 = 1u64;
                                                                                nat_add(a2157, a2158)?
                                                                            }, v2)?;
                                                                            match m2159 {
                                                                                Ok(v32) => {
                                                                                    let a2160 = f173(v0.clone(), v32)?;
                                                                                    let a2161 = 125u64;
                                                                                    if nat_eq(a2160, a2161) {
                                                                                        Ok(Adt20::C0(Adt19::C10(v19, std::rc::Rc::new(v26), std::rc::Rc::new(Adt19::C8)), {
                                                                                            let a2162 = v32;
                                                                                            let a2163 = 1u64;
                                                                                            nat_add(a2162, a2163)?
                                                                                        }))
                                                                                    } else {
                                                                                        let m2164 = f352(v0, v32, v2, v6, 2u64, List::cons(v19.clone(), v5))?;
                                                                                        match m2164 {
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
                let a2165 = v36;
                let a2166 = 91u64;
                if if nat_eq(a2165, a2166) {
                    true
                } else {
                    let a2167 = v36;
                    let a2168 = 123u64;
                    nat_eq(a2167, a2168)
                } {
                    let m2171 = f439(v0.clone(), {
                        let a2169 = v1;
                        let a2170 = 1u64;
                        nat_add(a2169, a2170)?
                    }, v2)?;
                    match m2171 {
                        Ok(v38) => {
                            let a2174 = f173(v0.clone(), v38)?;
                            let a2175 = {
                                let a2172 = v36;
                                let a2173 = 2u64;
                                nat_add(a2172, a2173)?
                            };
                            if nat_eq(a2174, a2175) {
                                Ok(Adt20::C0({
                                    let a2176 = v36;
                                    let a2177 = 91u64;
                                    if nat_eq(a2176, a2177) {
                                        Adt19::C6(std::rc::Rc::new(Adt19::C8))
                                    } else {
                                        Adt19::C7(std::rc::Rc::new(Adt19::C8))
                                    }
                                }, {
                                    let a2178 = v38;
                                    let a2179 = 1u64;
                                    nat_add(a2178, a2179)?
                                }))
                            } else {
                                let m2182 = f352(v0, v38, v2, v6, {
                                    let a2180 = v36;
                                    let a2181 = 91u64;
                                    if nat_eq(a2180, a2181) {
                                        1u64
                                    } else {
                                        2u64
                                    }
                                }, List::<Bytes>::nil())?;
                                match m2182 {
                                    Adt20::C0(v40, v41) => {
                                        Ok(Adt20::C0({
                                            let a2183 = v36;
                                            let a2184 = 91u64;
                                            if nat_eq(a2183, a2184) {
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
                    let a2185 = v36;
                    let a2186 = 34u64;
                    if if nat_eq(a2185, a2186) {
                        true
                    } else {
                        let a2187 = v36;
                        let a2188 = 39u64;
                        nat_eq(a2187, a2188)
                    } {
                        let m2189 = f351(v0, v1, v2, false)?;
                        match m2189 {
                            Adt30::C0(v43, v44) => {
                                Ok(Adt20::C0(Adt19::C5(v43), v44))
                            }
                            Adt30::C1(v42) => {
                                Ok(Adt20::C1(v42))
                            }
                        }
                    } else {
                        let m2190 = f252(v0.clone(), v1)?;
                        match m2190 {
                            None => {
                                if f178(f353(v0.clone(), v1, true)?) {
                                    Ok(Adt20::C1(f83(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                                } else {
                                    let v46: u64 = f354(v0.clone(), v1, true)?;
                                    let v47: u64 = f249(v0.clone(), v46)?;
                                    let m2193 = {
                                        let a2191 = f173(v0.clone(), v47)?;
                                        let a2192 = 10u64;
                                        if nat_eq(a2191, a2192) {
                                            f439(v0.clone(), v47, v2)?
                                        } else {
                                            Ok::<u64, Bytes>(v47)
                                        }
                                    };
                                    match m2193 {
                                        Ok(v49) => {
                                            let a2194 = f173(v0.clone(), v47)?;
                                            let a2195 = 10u64;
                                            if if if nat_eq(a2194, a2195) {
                                                f178(f429(f173(v0.clone(), v49)?))
                                            } else {
                                                false
                                            } {
                                                f178({
                                                    let a2196 = f173(v0.clone(), v49)?;
                                                    let a2197 = 58u64;
                                                    nat_eq(a2196, a2197)
                                                })
                                            } else {
                                                false
                                            } {
                                                Ok(Adt20::C1(f83(Str::lit("a multi-line plain scalar inside a flow collection is not read (YAML 1.2.2 [135])"), v47)?))
                                            } else {
                                                f357(f356(v0, v1, v46), v1, v46)
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

pub fn f353(v0: Bytes, v1: u64, v2: bool) -> R<bool> {
    let v3: u64 = f173(v0.clone(), v1)?;
    if if f172(v3) {
        true
    } else {
        f174(v3)
    } {
        Ok(false)
    } else {
        let a2198 = v3;
        let a2199 = 45u64;
        if if if nat_eq(a2198, a2199) {
            true
        } else {
            let a2200 = v3;
            let a2201 = 63u64;
            nat_eq(a2200, a2201)
        } {
            true
        } else {
            let a2202 = v3;
            let a2203 = 58u64;
            nat_eq(a2202, a2203)
        } {
            Ok(f349(f173(v0, {
                let a2204 = v1;
                let a2205 = 1u64;
                nat_add(a2204, a2205)?
            })?, v2))
        } else {
            Ok(f178(f441(v3)?))
        }
    }
}

pub fn f354(v0: Bytes, v1: u64, v2: bool) -> R<u64> {
    let (_, h2206) = {
        let (h2207, _) = f399(Fn7::F442(v2, v0.clone()), {
            let a2209 = {
                let a2208 = v0;
                length_bytes(a2208)
            };
            let a2210 = 1u64;
            nat_add(a2209, a2210)?
        }, (v1, v1))?;
        h2207
    };
    Ok(h2206)
}

pub fn f355(v0: Bytes, p2235: u64, v2: u64, p2236: u64, p2237: Bytes) -> R<Adt30> {
    let m2234 = (p2235, (p2236, p2237));
    {
        let mut m2234 = m2234;
        loop {
            let step = {
                let (v1, (v3, v4)) = m2234;
                let m2211 = v3;
                if m2211 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f83(Str::lit("a plain scalar has more lines than the input"), v1)?)))
                } else {
                    let v5 = m2211 - 1;
                    let v6: u64 = f249(v0.clone(), v1)?;
                    let v7: u64 = f173(v0.clone(), v6)?;
                    let a2212 = v7;
                    let a2213 = 58u64;
                    if nat_eq(a2212, a2213) {
                        Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f443(v6)?)))
                    } else {
                        let a2214 = v7;
                        let a2215 = 35u64;
                        if if nat_eq(a2214, a2215) {
                            true
                        } else {
                            let a2216 = v7;
                            let a2217 = 256u64;
                            nat_eq(a2216, a2217)
                        } {
                            Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C0(v4, f255(v0.clone(), v6)?)))
                        } else {
                            let v8: (u64, u64) = f444(v0.clone(), {
                                let a2218 = v6;
                                let a2219 = 1u64;
                                nat_add(a2218, a2219)?
                            })?;
                            let v9: u64 = {
                                let (h2220, _) = v8;
                                h2220
                            };
                            let v10: u64 = f248(v0.clone(), v9)?;
                            let v11: u64 = f249(v0.clone(), v10)?;
                            let v12: u64 = f173(v0.clone(), v11)?;
                            let a2221 = v12;
                            let a2222 = 256u64;
                            if if if if nat_eq(a2221, a2222) {
                                true
                            } else {
                                let a2225 = {
                                    let a2223 = v10;
                                    let a2224 = v9;
                                    nat_sub(a2223, a2224)
                                };
                                let a2226 = v2;
                                nat_lt(a2225, a2226)
                            } {
                                true
                            } else {
                                let a2227 = v12;
                                let a2228 = 35u64;
                                nat_eq(a2227, a2228)
                            } {
                                true
                            } else {
                                f103(v0.clone(), v9)?
                            } {
                                Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C0(v4, {
                                    let a2229 = v6;
                                    let a2230 = 1u64;
                                    nat_add(a2229, a2230)?
                                })))
                            } else {
                                let v13: u64 = f354(v0.clone(), v11, false)?;
                                let a2231 = v13;
                                let a2232 = v11;
                                if nat_eq(a2231, a2232) {
                                    Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f443(v11)?)))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Adt30>>((v13, (v5, f7(List::cons(v4, List::cons(f445({
                                        let (_, h2233) = v8;
                                        h2233
                                    }), List::cons(f356(v0.clone(), v11, v13), List::<Bytes>::nil())))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2234 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f356(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    f32(v0, v1, {
        let a2238 = v2;
        let a2239 = v1;
        nat_sub(a2238, a2239)
    })
}

pub fn f357(v0: Bytes, v1: u64, v2: u64) -> R<Adt20> {
    let a2241 = {
        let a2240 = v0.clone();
        length_bytes(a2240)
    };
    let a2242 = 0u64;
    if if nat_eq(a2241, a2242) {
        true
    } else {
        f211(v0.clone(), List::cons(f8(Str::lit("~")), List::cons(f8(Str::lit("null")), List::cons(f8(Str::lit("Null")), List::cons(f8(Str::lit("NULL")), List::<Bytes>::nil())))))
    } {
        Ok(Adt20::C1(f256(v1)?))
    } else {
        if f211(v0.clone(), List::cons(f8(Str::lit("true")), List::cons(f8(Str::lit("True")), List::cons(f8(Str::lit("TRUE")), List::<Bytes>::nil())))) {
            Ok(Adt20::C0(Adt19::C2(true), v2))
        } else {
            if f211(v0.clone(), List::cons(f8(Str::lit("false")), List::cons(f8(Str::lit("False")), List::cons(f8(Str::lit("FALSE")), List::<Bytes>::nil())))) {
                Ok(Adt20::C0(Adt19::C2(false), v2))
            } else {
                if f211(v0.clone(), List::cons(f8(Str::lit(".inf")), List::cons(f8(Str::lit(".Inf")), List::cons(f8(Str::lit(".INF")), List::cons(f8(Str::lit("+.inf")), List::cons(f8(Str::lit("+.Inf")), List::cons(f8(Str::lit("+.INF")), List::cons(f8(Str::lit("-.inf")), List::cons(f8(Str::lit("-.Inf")), List::cons(f8(Str::lit("-.INF")), List::cons(f8(Str::lit(".nan")), List::cons(f8(Str::lit(".NaN")), List::cons(f8(Str::lit(".NAN")), List::<Bytes>::nil()))))))))))))) {
                    Ok(Adt20::C1(f83(Str::lit("an infinite or not-a-number float is not read: Studio.Value has none (YAML 1.2.2 section 10.3.2)"), v1)?))
                } else {
                    if if f86(v0.clone(), 0u64, Str::lit("0o")) {
                        let a2244 = 2u64;
                        let a2245 = {
                            let a2243 = v0.clone();
                            length_bytes(a2243)
                        };
                        nat_lt(a2244, a2245)
                    } else {
                        false
                    } {
                        f446(v0, 8u64, v1, v2)
                    } else {
                        if if f86(v0.clone(), 0u64, Str::lit("0x")) {
                            let a2247 = 2u64;
                            let a2248 = {
                                let a2246 = v0.clone();
                                length_bytes(a2246)
                            };
                            nat_lt(a2247, a2248)
                        } else {
                            false
                        } {
                            f446(v0, 16u64, v1, v2)
                        } else {
                            let m2249 = f447(v0.clone())?;
                            match m2249 {
                                None => {
                                    Ok(Adt20::C0(Adt19::C5(v0), v2))
                                }
                                Some(v3) => {
                                    let m2250 = f448(v3.clone(), 0u64)?;
                                    match m2250 {
                                        None => {
                                            Ok(Adt20::C1(f449(v1)?))
                                        }
                                        Some(v4) => {
                                            let a2253 = {
                                                let (h2251, _) = v4;
                                                h2251
                                            };
                                            let a2254 = {
                                                let a2252 = v3;
                                                length_bytes(a2252)
                                            };
                                            if nat_eq(a2253, a2254) {
                                                Ok(Adt20::C0({
                                                    let (_, h2255) = v4;
                                                    h2255
                                                }, v2))
                                            } else {
                                                Ok(Adt20::C1(f449(v1)?))
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

pub fn f358(v0: Bytes, v1: u64) -> R<u64> {
    let (h2256, _) = f130(Fn4::F450(v0.clone()), {
        let a2258 = {
            let a2257 = v0;
            length_bytes(a2257)
        };
        let a2259 = 1u64;
        nat_add(a2258, a2259)?
    }, v1)?;
    Ok(h2256)
}

pub fn f359(v0: Bytes, v1: u64) -> R<bool> {
    let a2260 = f173(v0.clone(), v1)?;
    let a2261 = 58u64;
    if nat_eq(a2260, a2261) {
        if f172(f173(v0.clone(), {
            let a2262 = v1;
            let a2263 = 1u64;
            nat_add(a2262, a2263)?
        })?) {
            Ok(true)
        } else {
            Ok(f174(f173(v0, {
                let a2264 = v1;
                let a2265 = 1u64;
                nat_add(a2264, a2265)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f360(v0: u64) -> Bytes {
    let m2266 = v0;
    if m2266 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2266 - 1;
        let a2267 = f8(Str::lit("0"));
        let a2268 = f360(v1);
        append_bytes(a2267, a2268)
    }
}

pub fn f361(v0: Bytes, p2281: u64, p2282: u64) -> R<u64> {
    let m2280 = (p2281, p2282);
    {
        let mut m2280 = m2280;
        loop {
            let step = {
                let (v1, v2) = m2280;
                let m2269 = v2;
                if m2269 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m2269 - 1;
                    let v4: u64 = f139(v0.clone(), v1)?;
                    let a2270 = v4;
                    let a2271 = 34u64;
                    if nat_eq(a2270, a2271) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a2272 = v4;
                        let a2273 = 92u64;
                        if nat_eq(a2272, a2273) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a2274 = v4;
                            let a2275 = 32u64;
                            if nat_lt(a2274, a2275) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a2276 = v4;
                                let a2277 = 256u64;
                                if nat_eq(a2276, a2277) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a2278 = v1;
                                        let a2279 = 1u64;
                                        nat_add(a2278, a2279)?
                                    }, v3))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2280 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f362(v0: Bytes) -> R<Bytes> {
    let a2288 = {
        let a2286 = Bytes::lit(&[0x22]);
        let a2287 = {
            let (_, h2283) = {
                let (h2284, _) = f127(Fn3::F451(v0.clone()), {
                    let a2285 = v0;
                    length_bytes(a2285)
                }, (0u64, Bytes::lit(&[])))?;
                h2284
            };
            h2283
        };
        append_bytes(a2286, a2287)
    };
    let a2289 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a2288, a2289))
}

pub fn f363(v0: Fn5, p2293: List<(Bytes, Bytes)>, p2294: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m2292 = (p2293, p2294);
    {
        let mut m2292 = m2292;
        loop {
            let step = {
                let (v1, v2) = m2292;
                let m2290 = v2;
                match m2290.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c2291: Fn5 = v0.clone();
                            c2291.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2292 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f364(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f452(v0, v1)
}

pub fn f365(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f453(v0)
}

pub fn f366(v0: Fn6, p2298: List<Bytes>, p2299: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m2297 = (p2298, p2299);
    {
        let mut m2297 = m2297;
        loop {
            let step = {
                let (v1, v2) = m2297;
                let m2295 = v2;
                match m2295.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c2296: Fn6 = v0.clone();
                            c2296.apply(v1, v3)?
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

pub fn f367(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f454(v1)?, v0))
}

pub fn f368(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f455(v1, v0, v2)
}

pub fn f369(v0: u64) -> bool {
    if f370(v0) {
        true
    } else {
        let a2300 = 65u64;
        let a2301 = v0;
        if if nat_le(a2300, a2301) {
            let a2302 = v0;
            let a2303 = 70u64;
            nat_le(a2302, a2303)
        } else {
            false
        } {
            true
        } else {
            let a2304 = 97u64;
            let a2305 = v0;
            if nat_le(a2304, a2305) {
                let a2306 = v0;
                let a2307 = 102u64;
                nat_le(a2306, a2307)
            } else {
                false
            }
        }
    }
}

pub fn f370(v0: u64) -> bool {
    let a2308 = 48u64;
    let a2309 = v0;
    if nat_le(a2308, a2309) {
        let a2310 = v0;
        let a2311 = 57u64;
        nat_le(a2310, a2311)
    } else {
        false
    }
}

pub fn f371(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f87(v0.clone(), v1)?;
    let a2312 = v2;
    let a2313 = 69u64;
    if if nat_eq(a2312, a2313) {
        true
    } else {
        let a2314 = v2;
        let a2315 = 101u64;
        nat_eq(a2314, a2315)
    } {
        let v3: u64 = f87(v0.clone(), {
            let a2316 = v1;
            let a2317 = 1u64;
            nat_add(a2316, a2317)?
        })?;
        let v4: u64 = {
            let a2318 = v3;
            let a2319 = 43u64;
            if if nat_eq(a2318, a2319) {
                true
            } else {
                let a2320 = v3;
                let a2321 = 45u64;
                nat_eq(a2320, a2321)
            } {
                let a2322 = v1;
                let a2323 = 2u64;
                nat_add(a2322, a2323)?
            } else {
                let a2324 = v1;
                let a2325 = 1u64;
                nat_add(a2324, a2325)?
            }
        };
        let v5: u64 = f270(v0, 0u64, v4)?;
        let a2326 = v4;
        let a2327 = v5;
        if nat_lt(a2326, a2327) {
            Ok(v5)
        } else {
            Ok(v1)
        }
    } else {
        Ok(v1)
    }
}

pub fn f372(v0: Adt21, v1: Bytes) -> bool {
    let a2328 = f373(v0.clone());
    let a2329 = 3u64;
    if !nat_eq(a2328, a2329) {
        f35(f374(v0), v1)
    } else {
        false
    }
}

pub fn f373(v0: Adt21) -> u64 {
    let m2330 = v0;
    match m2330 {
        Adt21::C0(v1, _) => {
            v1
        }
    }
}

pub fn f374(v0: Adt21) -> Bytes {
    let m2331 = v0;
    match m2331 {
        Adt21::C0(_, v2) => {
            v2
        }
    }
}

pub fn f375(v0: Bytes) -> Bytes {
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

pub fn f376(v0: Bytes) -> bool {
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

pub fn f377(p2340: u64, p2341: List<Adt21>, p2342: List<Adt13>) -> R<Result<(List<Adt13>, List<Adt21>), Bytes>> {
    let m2339 = (p2340, (p2341, p2342));
    {
        let mut m2339 = m2339;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2339;
                let m2332 = v0;
                if m2332 == 0 {
                    Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt21>), Bytes>(f8(Str::lit("a dictionary has more members than the input has tokens")))))
                } else {
                    let v3 = m2332 - 1;
                    if f273(v1.clone(), Str::lit("}")) {
                        let m2333 = f280(f275(v1), Str::lit(";"));
                        match m2333 {
                            Ok(v5) => {
                                Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>(Ok(Ok::<(List<Adt13>, List<Adt21>), Bytes>((f456(v2, List::<Adt13>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt21>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m2334 = f182(v1)?;
                        match m2334 {
                            Ok(v7) => {
                                let v8: bool = f273(v7.clone(), Str::lit("required"));
                                let m2335 = f285(v3, if v8 {
                                    f275(v7)
                                } else {
                                    v7
                                }, 0u64)?;
                                match m2335 {
                                    Ok(v10) => {
                                        let m2336 = f279(f286(v10.clone()));
                                        match m2336 {
                                            Ok(v12) => {
                                                if if f273(f281(v12.clone()), Str::lit("=")) {
                                                    !v8
                                                } else {
                                                    false
                                                } {
                                                    let m2337 = f379(f275(f281(v12.clone())))?;
                                                    match m2337 {
                                                        Ok(v14) => {
                                                            Ok::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>((v3, (v14, List::cons(Adt13::C0(f283(v12), f386(f287(v10)), 0u64), v2))))
                                                        }
                                                        Err(v13) => {
                                                            Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt21>), Bytes>(v13)))
                                                        }
                                                    }
                                                } else {
                                                    let m2338 = f280(f281(v12.clone()), Str::lit(";"));
                                                    match m2338 {
                                                        Ok(v16) => {
                                                            Ok::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>((v3, (v16, List::cons(Adt13::C0(f283(v12), f386(f287(v10)), if v8 {
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
                Ok(next) => { m2339 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f378(p2351: u64, p2352: List<Adt21>, p2353: List<Adt13>) -> R<Result<(List<Adt13>, List<Adt21>), Bytes>> {
    let m2350 = (p2351, (p2352, p2353));
    {
        let mut m2350 = m2350;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2350;
                let m2343 = v0;
                if m2343 == 0 {
                    Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt21>), Bytes>(f8(Str::lit("an interface has more members than the input has tokens")))))
                } else {
                    let v3 = m2343 - 1;
                    if f273(v1.clone(), Str::lit("}")) {
                        let m2344 = f280(f275(v1), Str::lit(";"));
                        match m2344 {
                            Ok(v5) => {
                                Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>(Ok(Ok::<(List<Adt13>, List<Adt21>), Bytes>((f456(v2, List::<Adt13>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt21>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m2345 = f182(v1)?;
                        match m2345 {
                            Ok(v7) => {
                                let v8: List<Adt21> = if if f273(v7.clone(), Str::lit("stringifier")) {
                                    true
                                } else {
                                    f273(v7.clone(), Str::lit("inherit"))
                                } {
                                    f275(v7.clone())
                                } else {
                                    v7.clone()
                                };
                                let v9: List<Adt21> = if f273(v8.clone(), Str::lit("readonly")) {
                                    f275(v8)
                                } else {
                                    v8
                                };
                                if f273(v9.clone(), Str::lit("attribute")) {
                                    let m2346 = f285(v3, f275(v9), 0u64)?;
                                    match m2346 {
                                        Ok(v11) => {
                                            let m2347 = f279(f286(v11.clone()));
                                            match m2347 {
                                                Ok(v13) => {
                                                    let m2348 = f280(f281(v13.clone()), Str::lit(";"));
                                                    match m2348 {
                                                        Ok(v15) => {
                                                            Ok::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>((v3, (v15, List::cons(Adt13::C0(f283(v13), f386(f287(v11)), 0u64), v2))))
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
                                    let m2349 = f379(v7)?;
                                    match m2349 {
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
                Ok(next) => { m2350 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f379(v0: List<Adt21>) -> R<Result<List<Adt21>, Bytes>> {
    Ok(f274({
        let a2355 = {
            let a2354 = v0.clone();
            length_list(a2354)
        };
        let a2356 = 1u64;
        nat_add(a2355, a2356)?
    }, v0, List::<Bytes>::nil(), true))
}

pub fn f380(v0: List<Adt21>) -> Bytes {
    let m2357 = v0;
    match m2357.uncons() {
        None => {
            f8(Str::lit("the end of the input"))
        }
        Some((v1, _)) => {
            f7(List::cons(f8(Str::lit("'")), List::cons(f374(v1), List::cons(f8(Str::lit("'")), List::<Bytes>::nil()))))
        }
    }
}

pub fn f381(v0: Bytes) -> Bytes {
    if f147(v0.clone(), f8(Str::lit("_"))) {
        f32(v0.clone(), 1u64, {
            let a2359 = {
                let a2358 = v0;
                length_bytes(a2358)
            };
            let a2360 = 1u64;
            nat_sub(a2359, a2360)
        })
    } else {
        v0
    }
}

pub fn f382(v0: Adt14, v1: List<Adt21>) -> Result<Adt29, Bytes> {
    if f273(v1.clone(), Str::lit("?")) {
        Ok::<Adt29, Bytes>(Adt29::C0(f383(Str::lit("Option"), v0), f275(v1)))
    } else {
        Ok::<Adt29, Bytes>(Adt29::C0(v0, v1))
    }
}

pub fn f383(v0: Str, v1: Adt14) -> Adt14 {
    let v2: Adt14 = f386(v1);
    Adt14::C0(0u64, f7(List::cons(f8(Str::lit("(")), List::cons({
        let a2361 = v0;
        utf8_encode(a2361)
    }, List::cons(f8(Str::lit(" ")), List::cons(f201(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), f202(v2.clone()), f199(v2), Bytes::lit(&[]))
}

pub fn f384(v0: Bytes) -> Adt14 {
    Adt14::C0(0u64, Bytes::lit(&[]), List::<Bytes>::nil(), v0, Bytes::lit(&[]))
}

pub fn f385(v0: Bytes) -> Option<Adt14> {
    f457(v0, f458())
}

pub fn f386(v0: Adt14) -> Adt14 {
    let a2362 = f200(v0.clone());
    let a2363 = 0u64;
    if nat_eq(a2362, a2363) {
        v0
    } else {
        Adt14::C0(0u64, Bytes::lit(&[]), f202(v0.clone()), if f35(f199(v0.clone()), Bytes::lit(&[])) {
            f459()
        } else {
            f199(v0)
        }, Bytes::lit(&[]))
    }
}

pub fn f387(v0: Adt14) -> Bytes {
    let m2364 = v0;
    match m2364 {
        Adt14::C0(_, _, _, _, v5) => {
            v5
        }
    }
}

pub fn f388(v0: Adt13, v1: List<Adt13>) -> R<List<Adt13>> {
    let m2365 = v1;
    match m2365.uncons() {
        None => {
            Ok(List::cons(v0, List::<Adt13>::nil()))
        }
        Some((v2, v3)) => {
            if f460({
                let a2367 = {
                    let a2366 = f301(v0.clone());
                    length_bytes(a2366)
                };
                let a2368 = 1u64;
                nat_add(a2367, a2368)?
            }, f301(v0.clone()), f301(v2.clone()), 0u64)? {
                Ok(List::cons(v0, List::cons(v2, v3)))
            } else {
                Ok(List::cons(v2, f388(v0, v3)?))
            }
        }
    }
}

pub fn f389(v0: List<Adt12>, v1: Bytes) -> List<Adt13> {
    let m2369 = f123(v0.clone(), 1u64, v1.clone());
    match m2369 {
        None => {
            f298(v0, 6u64, v1)
        }
        Some(v2) => {
            f296(f297(v2), f298(v0, 6u64, v1))
        }
    }
}

pub fn f390(v0: Bytes) -> R<Option<Bytes>> {
    let m2371 = f461({
        let a2370 = v0.clone();
        length_bytes(a2370)
    }, v0.clone(), 0u64, Bytes::lit(&[]))?;
    match m2371 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a2373 = {
                let a2372 = v1.clone();
                length_bytes(a2372)
            };
            let a2374 = 0u64;
            if nat_eq(a2373, a2374) {
                let a2376 = {
                    let a2375 = v0;
                    length_bytes(a2375)
                };
                let a2377 = 0u64;
                if nat_eq(a2376, a2377) {
                    Ok(Some(f8(Str::lit("u"))))
                } else {
                    Ok(None::<Bytes>)
                }
            } else {
                Ok(Some(f392(v1)?))
            }
        }
    }
}

pub fn f391(v0: Bytes) -> R<Option<Bytes>> {
    if f462({
        let a2378 = v0.clone();
        length_bytes(a2378)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f463(v0)?;
        if f464(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m2381 = {
                let a2379 = v1.clone();
                let a2380 = 0u64;
                index_bytes(a2379, a2380)
            };
            match m2381 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f325(f42(v2)?) {
                        Ok(Some({
                            let a2382 = f8(Str::lit("n"));
                            let a2383 = v1;
                            append_bytes(a2382, a2383)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f465({
            let a2384 = v0.clone();
            length_bytes(a2384)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f392(v0: Bytes) -> R<Bytes> {
    if f115(v0.clone(), f466()) {
        f467(v0)
    } else {
        Ok(v0)
    }
}

pub fn f393(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2387 = {
        let a2385 = v0;
        let a2386 = v1;
        index_bytes(a2385, a2386)
    };
    match m2387 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f42(v2)?;
            if f321({
                let a2388 = 128u64;
                let a2389 = v3;
                nat_le(a2388, a2389)
            }, {
                let a2390 = v3;
                let a2391 = 191u64;
                nat_le(a2390, a2391)
            }) {
                Ok(Some({
                    let a2392 = v3;
                    let a2393 = 128u64;
                    nat_sub(a2392, a2393)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f394(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f321({
        let a2394 = v1;
        let a2395 = v0;
        nat_le(a2394, a2395)
    }, f321({
        let a2396 = v0;
        let a2397 = v2;
        nat_le(a2396, a2397)
    }, f323({
        let a2398 = v0;
        let a2399 = 55296u64;
        nat_lt(a2398, a2399)
    }, {
        let a2400 = 57343u64;
        let a2401 = v0;
        nat_lt(a2400, a2401)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f395() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{3}/\u{7f}\u{3}@\u{0}\u{3}\u{7f}}\u{4}\u{0}\u{0}C\u{7f}\u{7f}"))
}

pub fn f396(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a2402 = v1;
        let a2403 = 3u64;
        nat_mul(a2402, a2403)?
    };
    let a2414 = {
        let a2410 = {
            let a2404 = f139(v0.clone(), v2)?;
            let a2405 = 16384u64;
            nat_mul(a2404, a2405)?
        };
        let a2411 = {
            let a2408 = f139(v0.clone(), {
                let a2406 = v2;
                let a2407 = 1u64;
                nat_add(a2406, a2407)?
            })?;
            let a2409 = 128u64;
            nat_mul(a2408, a2409)?
        };
        nat_add(a2410, a2411)?
    };
    let a2415 = f139(v0, {
        let a2412 = v2;
        let a2413 = 2u64;
        nat_add(a2412, a2413)?
    })?;
    nat_add(a2414, a2415)
}

pub fn f397(v0: u64) -> R<bool> {
    f207(v0, f468())
}

pub fn f398(v0: u64) -> R<bool> {
    f207(v0, f469())
}

pub fn f399(v0: Fn7, p2420: u64, p2421: (u64, u64)) -> R<((u64, u64), bool)> {
    let m2419 = (p2420, p2421);
    {
        let mut m2419 = m2419;
        loop {
            let step = {
                let (v1, v2) = m2419;
                let m2416 = v1;
                if m2416 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2416 - 1;
                    let m2418 = {
                        let c2417: Fn7 = v0.clone();
                        c2417.apply(v2)?
                    };
                    match m2418 {
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
                Ok(next) => { m2419 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f400(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f470(v1, v0, v2)
}

pub fn f401(v0: Bytes, v1: u64) -> Bytes {
    f32(v0, v1, 1u64)
}

pub fn f402() -> Bytes {
    f8(Str::lit("\u{0}\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\u{9}\u{a}\u{b}\u{c}\u{d}\u{e}\u{f}\u{10}\u{11}\u{12}\u{13}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{1b}\u{1c}\u{1d}\u{1e}\u{1f} !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~\u{7f}"))
}

pub fn f403() -> Bytes {
    f8(Str::lit("\u{80}\u{c0}\u{100}\u{140}\u{180}\u{1c0}\u{200}\u{240}\u{280}\u{2c0}\u{300}\u{340}\u{380}\u{3c0}\u{400}\u{440}\u{480}\u{4c0}\u{500}\u{540}\u{580}\u{5c0}\u{600}\u{640}\u{680}\u{6c0}\u{700}\u{740}\u{780}\u{7c0}"))
}

pub fn f404(v0: u64) -> R<Bytes> {
    Ok(f401(f471(), {
        let a2427 = {
            let a2425 = {
                let a2422 = v0;
                let a2423 = 64u64;
                let a2424 = 0u64;
                nat_rem(a2422, a2423, a2424)
            };
            let a2426 = 2u64;
            nat_mul(a2425, a2426)?
        };
        let a2428 = 1u64;
        nat_add(a2427, a2428)?
    }))
}

pub fn f405() -> Bytes {
    f8(Str::lit("\u{800}\u{1000}\u{2000}\u{3000}\u{4000}\u{5000}\u{6000}\u{7000}\u{8000}\u{9000}\u{a000}\u{b000}\u{c000}\u{d000}\u{e000}\u{f000}"))
}

pub fn f406(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<Option<u64>> {
    if f472(f87(v0, v3)?, v1, v2)? {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a2429 = v3;
            let a2430 = 1u64;
            nat_add(a2429, a2430)?
        }))
    }
}

pub fn f407(v0: Bytes) -> Result<Bytes, Bytes> {
    Err::<Bytes, Bytes>(v0)
}

pub fn f408() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f409(v0: Bytes) -> R<Option<Bytes>> {
    let m2431 = f133(v0.clone(), f8(Str::lit(" ")), 0u64)?;
    match m2431 {
        None => {
            let m2432 = f133(v0.clone(), f8(Str::lit(":")), 0u64)?;
            match m2432 {
                None => {
                    let v3: Bytes = f7(List::cons(f8(Str::lit(" ")), List::cons(v0, List::cons(f8(Str::lit(":")), List::<Bytes>::nil()))));
                    let m2433 = f133(f473(), v3.clone(), 0u64)?;
                    match m2433 {
                        None => {
                            Ok(None::<Bytes>)
                        }
                        Some(v4) => {
                            let v5: u64 = {
                                let a2435 = v4;
                                let a2436 = {
                                    let a2434 = v3;
                                    length_bytes(a2434)
                                };
                                nat_add(a2435, a2436)?
                            };
                            let m2437 = f133(f473(), f8(Str::lit(" ")), v5)?;
                            match m2437 {
                                None => {
                                    Ok(None::<Bytes>)
                                }
                                Some(v6) => {
                                    Ok(Some(f32(f473(), v5, {
                                        let a2438 = v6;
                                        let a2439 = v5;
                                        nat_sub(a2438, a2439)
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

pub fn f410(v0: Bytes) -> Bytes {
    if f474(v0.clone(), Bytes::lit(&[0x0d])) {
        f32(v0.clone(), 0u64, {
            let a2441 = {
                let a2440 = v0;
                length_bytes(a2440)
            };
            let a2442 = 1u64;
            nat_sub(a2441, a2442)
        })
    } else {
        v0
    }
}

pub fn f411() -> List<Bytes> {
    List::cons(f8(Str::lit("complexType")), List::cons(f8(Str::lit("simpleType")), List::cons(f8(Str::lit("element")), List::cons(f8(Str::lit("attribute")), List::cons(f8(Str::lit("attributeGroup")), List::cons(f8(Str::lit("group")), List::<Bytes>::nil()))))))
}

pub fn f412(v0: u64, v1: Adt23, v2: Adt24) -> R<Result<List<Adt26>, Bytes>> {
    let v3: Bytes = f154(match v2.clone() {
        Adt24::C0(_, _, h2443, _) => {
            h2443
        }
    })?;
    let v4: (Bytes, Bytes) = (match v2.clone() {
        Adt24::C0(_, h2444, _, _) => {
            h2444
        }
    }, match v2.clone() {
        Adt24::C0(_, _, h2445, _) => {
            h2445
        }
    });
    let v5: bool = f475(match v2.clone() {
        Adt24::C0(_, _, _, h2446) => {
            h2446
        }
    }, Str::lit("abstract"), Str::lit("true"));
    let m2448 = f476(match v1.clone() {
        Adt23::C0(h2447, _) => {
            h2447
        }
    }, v4, f239(v5))?;
    match m2448 {
        Ok(v7) => {
            if v5 {
                let m2449 = v7.clone();
                match m2449.uncons() {
                    None => {
                        Ok(Err::<List<Adt26>, Bytes>(f418(f8(Str::lit("an abstract complex type with no type derived from it in this schema set admits no instance (XSD 1.1 Part 1, sections 3.4.1 and 3.3.4.2)")), v3)))
                    }
                    Some((_, _)) => {
                        let m2450 = f477(v3, v7.clone(), f478(v7.clone())?, f478(v7)?)?;
                        match m2450 {
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
                let m2451 = v7.clone();
                match m2451.uncons() {
                    None => {
                        f479(v0, v1, v3, match v2.clone() {
                            Adt24::C0(_, h2452, _, _) => {
                                h2452
                            }
                        }, match v2 {
                            Adt24::C0(_, _, _, h2453) => {
                                h2453
                            }
                        })
                    }
                    Some((_, _)) => {
                        let v14: Bytes = f480(v3.clone());
                        let m2456 = f479(v0, v1, v14.clone(), match v2.clone() {
                            Adt24::C0(_, h2454, _, _) => {
                                h2454
                            }
                        }, match v2.clone() {
                            Adt24::C0(_, _, _, h2455) => {
                                h2455
                            }
                        })?;
                        match m2456 {
                            Ok(v16) => {
                                let m2458 = f477(v3, List::cons(match v2 {
                                    Adt24::C0(_, _, h2457, _) => {
                                        h2457
                                    }
                                }, v7.clone()), List::cons(v14.clone(), f478(v7.clone())?), List::cons(v14, f478(v7)?))?;
                                match m2458 {
                                    Ok(v18) => {
                                        Ok(Ok::<List<Adt26>, Bytes>(f330(v16, List::cons(v18, List::<Adt26>::nil()))))
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

pub fn f413(v0: Bytes, v1: Adt25, v2: Bytes) -> R<Result<Adt26, Bytes>> {
    let m2461 = f153(v0.clone(), match v1.clone() {
        Adt25::C0(_, _, _, h2459, _) => {
            h2459
        }
    }, match v1 {
        Adt25::C0(_, _, _, _, h2460) => {
            h2460
        }
    }, v2)?;
    match m2461 {
        Ok(v4) => {
            Ok(Ok::<Adt26, Bytes>(Adt26::C0(f154(v0)?, match v4.clone() {
                Adt22::C0(_, h2462) => {
                    h2462
                }
            }, match v4 {
                Adt22::C0(h2463, _) => {
                    h2463
                }
            }, List::<Bytes>::nil())))
        }
        Err(v3) => {
            Ok(Err::<Adt26, Bytes>(v3))
        }
    }
}

pub fn f414(v0: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: Adt25) -> R<Result<Adt31, Bytes>> {
    let m2464 = v0;
    if m2464 == 0 {
        Ok(Err::<Adt31, Bytes>(f418(f8(Str::lit("the element declarations nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m2464 - 1;
        let m2465 = f241(v4.clone(), Str::lit("type"));
        match m2465 {
            None => {
                let m2469 = f416(v4.clone())?;
                match m2469 {
                    Ok(v10) => {
                        let m2470 = f417(v10, List::cons(f8(Str::lit("complexType")), List::cons(f8(Str::lit("simpleType")), List::<Bytes>::nil())));
                        match m2470 {
                            None => {
                                Ok(Ok::<Adt31, Bytes>(Adt31::C0(Adt22::C0(f482(v4, f333()), List::cons(f333(), List::<Bytes>::nil())), List::<Adt26>::nil())))
                            }
                            Some(v11) => {
                                if f240(v11.clone(), Str::lit("complexType")) {
                                    let m2471 = f479(v5, v1, v2.clone(), v3, v11)?;
                                    match m2471 {
                                        Ok(v13) => {
                                            Ok(Ok::<Adt31, Bytes>(Adt31::C0(Adt22::C0(f482(v4, v2.clone()), List::cons(v2, List::<Bytes>::nil())), v13)))
                                        }
                                        Err(v12) => {
                                            Ok(Err::<Adt31, Bytes>(v12))
                                        }
                                    }
                                } else {
                                    let m2472 = f413(v2.clone(), v11, v3)?;
                                    match m2472 {
                                        Ok(v15) => {
                                            Ok(Ok::<Adt31, Bytes>(Adt31::C0(Adt22::C0(f482(v4, v2.clone()), List::cons(v2, List::<Bytes>::nil())), List::cons(v15, List::<Adt26>::nil()))))
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
                let m2466 = f481(v1, v4.clone(), v6)?;
                match m2466 {
                    Ok(v8) => {
                        Ok(Ok::<Adt31, Bytes>(Adt31::C0(Adt22::C0(f482(v4, match v8.clone() {
                            Adt22::C0(h2467, _) => {
                                h2467
                            }
                        }), match v8 {
                            Adt22::C0(_, h2468) => {
                                h2468
                            }
                        }), List::<Adt26>::nil())))
                    }
                    Err(v7) => {
                        Ok(Err::<Adt31, Bytes>(f418(v7, v2)))
                    }
                }
            }
        }
    }
}

pub fn f415(v0: Adt23, v1: Bytes, v2: Bytes, v3: Adt25) -> R<Result<Adt31, Bytes>> {
    let m2473 = f241(v3.clone(), Str::lit("type"));
    match m2473 {
        None => {
            let m2475 = f416(v3)?;
            match m2475 {
                Ok(v8) => {
                    let m2476 = f417(v8, List::cons(f8(Str::lit("simpleType")), List::<Bytes>::nil()));
                    match m2476 {
                        None => {
                            Ok(Err::<Adt31, Bytes>(f418(f8(Str::lit("an attribute declaration with no type has the type xs:anySimpleType, which is not translated, because it is the union of every simple value space (XSD 1.1 Part 1, section 3.2.2.1; Part 2, section 3.2.1)")), v2)))
                        }
                        Some(v9) => {
                            let m2477 = f413(v2.clone(), v9, v1)?;
                            match m2477 {
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
            let m2474 = f481(v0, v3, v4)?;
            match m2474 {
                Ok(v6) => {
                    Ok(Ok::<Adt31, Bytes>(Adt31::C0(v6, List::<Adt26>::nil())))
                }
                Err(v5) => {
                    Ok(Err::<Adt31, Bytes>(f418(v5, v2)))
                }
            }
        }
    }
}

pub fn f416(v0: Adt25) -> R<Result<List<Adt25>, Bytes>> {
    f167(match v0.clone() {
        Adt25::C0(_, _, _, h2478, _) => {
            h2478
        }
    }, match v0 {
        Adt25::C0(_, _, _, _, h2479) => {
            h2479
        }
    })
}

pub fn f417(p2483: List<Adt25>, v1: List<Bytes>) -> Option<Adt25> {
    let m2482 = p2483;
    {
        let mut m2482 = m2482;
        loop {
            let step = {
                let v0 = m2482;
                let m2480 = v0;
                match m2480.uncons() {
                    None => {
                        Err::<List<Adt25>, Option<Adt25>>(None::<Adt25>)
                    }
                    Some((v2, v3)) => {
                        if f115(match v2.clone() {
                            Adt25::C0(_, h2481, _, _, _) => {
                                h2481
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
                Ok(next) => { m2482 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f418(v0: Bytes, v1: Bytes) -> Bytes {
    f163(v0, f8(Str::lit(" at ")), v1)
}

pub fn f419(v0: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: Adt25) -> R<Result<List<Adt26>, Bytes>> {
    let m2484 = v0;
    if m2484 == 0 {
        Ok(Err::<List<Adt26>, Bytes>(f418(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m2484 - 1;
        let m2485 = f416(v4.clone())?;
        match m2485 {
            Ok(v7) => {
                if f240(v4, Str::lit("choice")) {
                    let m2486 = f483(v5, v1, v2.clone(), v3, false, v7, f421())?;
                    match m2486 {
                        Ok(v9) => {
                            let m2490 = f477(v2, f484(match v9.clone() {
                                Adt32::C0(h2487, _, _, _, _) => {
                                    h2487
                                }
                            }), f485(match v9.clone() {
                                Adt32::C0(h2488, _, _, _, _) => {
                                    h2488
                                }
                            }), f486(match v9.clone() {
                                Adt32::C0(h2489, _, _, _, _) => {
                                    h2489
                                }
                            }))?;
                            match m2490 {
                                Ok(v11) => {
                                    Ok(Ok::<List<Adt26>, Bytes>(f330(match v9 {
                                        Adt32::C0(_, h2491, _, _, _) => {
                                            h2491
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
                    let m2492 = f483(v5, v1, v2.clone(), v3, true, v7, f421())?;
                    match m2492 {
                        Ok(v13) => {
                            let m2494 = f487(v2, match v13.clone() {
                                Adt32::C0(h2493, _, _, _, _) => {
                                    h2493
                                }
                            })?;
                            match m2494 {
                                Ok(v15) => {
                                    Ok(Ok::<List<Adt26>, Bytes>(f330(match v13 {
                                        Adt32::C0(_, h2495, _, _, _) => {
                                            h2495
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

pub fn f420(v0: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: List<Adt25>, v5: Adt32) -> R<Result<Adt32, Bytes>> {
    let m2496 = v0;
    if m2496 == 0 {
        Ok(Err::<Adt32, Bytes>(f418(f8(Str::lit("the attribute group references nest deeper than the schema set has components, so they refer to each other in a cycle (XSD 1.1 Part 1, section 3.6.3)")), v2)))
    } else {
        let v6 = m2496 - 1;
        let m2497 = v4;
        match m2497.uncons() {
            None => {
                Ok(Ok::<Adt32, Bytes>(v5))
            }
            Some((v7, v8)) => {
                if f240(v7.clone(), Str::lit("attribute")) {
                    if f475(v7.clone(), Str::lit("use"), Str::lit("prohibited")) {
                        f420(v6, v1, v2, v3, v8, v5)
                    } else {
                        let v9: bool = f475(v7.clone(), Str::lit("use"), Str::lit("required"));
                        let m2498 = f241(v7.clone(), Str::lit("ref"));
                        match m2498 {
                            None => {
                                let m2508 = f241(v7.clone(), Str::lit("name"));
                                match m2508 {
                                    None => {
                                        Ok(Err::<Adt32, Bytes>(f418(f8(Str::lit("a local attribute declaration must have a name or a ref (XSD 1.1 Part 1, section 3.2.3)")), v2)))
                                    }
                                    Some(v16) => {
                                        let m2511 = f415(v1.clone(), v3.clone(), {
                                            let a2509 = v2.clone();
                                            let a2510 = f154(v16.clone())?;
                                            append_bytes(a2509, a2510)
                                        }, v7)?;
                                        match m2511 {
                                            Ok(v18) => {
                                                f420(v6, v1, v2, v3, v8, f489(v5, v16, v18, v9))
                                            }
                                            Err(v17) => {
                                                Ok(Err::<Adt32, Bytes>(v17))
                                            }
                                        }
                                    }
                                }
                            }
                            Some(v10) => {
                                let m2500 = f91(match v7 {
                                    Adt25::C0(_, _, _, _, h2499) => {
                                        h2499
                                    }
                                }, v10.clone())?;
                                match m2500 {
                                    Ok(v12) => {
                                        if f35({
                                            let (h2501, _) = v12.clone();
                                            h2501
                                        }, f145()) {
                                            Ok(Err::<Adt32, Bytes>(f418(f163(f8(Str::lit("the attribute ")), v10, f8(Str::lit(" is declared by the schema document for the XML namespace, which this schema set does not import (XSD 1.1 Part 1, sections 3.2.2.3 and 4.2.6)"))), v2)))
                                        } else {
                                            let m2502 = f488(v1.clone(), Str::lit("attribute"), v12.clone());
                                            match m2502 {
                                                None => {
                                                    Ok(Err::<Adt32, Bytes>(f418(f163(f8(Str::lit("the attribute ")), v10, f8(Str::lit(" is not declared by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                                }
                                                Some(v13) => {
                                                    let m2506 = f415(v1.clone(), match v13.clone() {
                                                        Adt24::C0(_, h2503, _, _) => {
                                                            h2503
                                                        }
                                                    }, f154({
                                                        let (_, h2504) = v12.clone();
                                                        h2504
                                                    })?, match v13 {
                                                        Adt24::C0(_, _, _, h2505) => {
                                                            h2505
                                                        }
                                                    })?;
                                                    match m2506 {
                                                        Ok(v15) => {
                                                            f420(v6, v1, v2, v3, v8, f489(v5, {
                                                                let (_, h2507) = v12;
                                                                h2507
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
                                        Ok(Err::<Adt32, Bytes>(f418(v11, v2)))
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if f240(v7.clone(), Str::lit("attributeGroup")) {
                        let m2512 = f241(v7.clone(), Str::lit("ref"));
                        match m2512 {
                            None => {
                                Ok(Err::<Adt32, Bytes>(f418(f8(Str::lit("an attribute group reference must have a ref (XSD 1.1 Part 1, section 3.6.2)")), v2)))
                            }
                            Some(v19) => {
                                let m2514 = f91(match v7 {
                                    Adt25::C0(_, _, _, _, h2513) => {
                                        h2513
                                    }
                                }, v19.clone())?;
                                match m2514 {
                                    Ok(v21) => {
                                        let m2515 = f488(v1.clone(), Str::lit("attributeGroup"), v21.clone());
                                        match m2515 {
                                            None => {
                                                Ok(Err::<Adt32, Bytes>(f418(f163(f8(Str::lit("the attribute group ")), v19, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                            }
                                            Some(v22) => {
                                                let m2517 = f416(match v22.clone() {
                                                    Adt24::C0(_, _, _, h2516) => {
                                                        h2516
                                                    }
                                                })?;
                                                match m2517 {
                                                    Ok(v24) => {
                                                        let m2520 = f420(v6, v1.clone(), f154({
                                                            let (_, h2518) = v21;
                                                            h2518
                                                        })?, match v22 {
                                                            Adt24::C0(_, h2519, _, _) => {
                                                                h2519
                                                            }
                                                        }, v24, v5)?;
                                                        match m2520 {
                                                            Ok(v26) => {
                                                                f420(v6, v1, v2, v3, v8, v26)
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
                                        Ok(Err::<Adt32, Bytes>(f418(v20, v2)))
                                    }
                                }
                            }
                        }
                    } else {
                        if f240(v7, Str::lit("anyAttribute")) {
                            f420(v6, v1, v2, v3, v8, f490(v5, Adt33::C0(f8(Str::lit("anyAttribute")), f491(), List::<Bytes>::nil())))
                        } else {
                            f420(v6, v1, v2, v3, v8, v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f421() -> Adt32 {
    Adt32::C0(List::<Adt33>::nil(), List::<Adt26>::nil(), 0u64, 0u64, 0u64)
}

pub fn f422(v0: List<Adt26>, v1: List<Bytes>, v2: Bytes) -> Result<List<Bytes>, Bytes> {
    let m2521 = v1;
    match m2521.uncons() {
        None => {
            Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let m2522 = f332(v0.clone(), v3.clone());
            match m2522 {
                None => {
                    Err::<List<Bytes>, Bytes>(f423(v3))
                }
                Some(v5) => {
                    let m2524 = match v5.clone() {
                        Adt26::C0(_, _, _, h2523) => {
                            h2523
                        }
                    };
                    match m2524.uncons() {
                        None => {
                            Err::<List<Bytes>, Bytes>(f423(v3))
                        }
                        Some((_, _)) => {
                            let m2525 = f422(v0, v4, v2.clone());
                            match m2525 {
                                Ok(v9) => {
                                    Ok::<List<Bytes>, Bytes>(List::cons(f492(v3, match v5 {
                                        Adt26::C0(_, _, _, h2526) => {
                                            h2526
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

pub fn f423(v0: Bytes) -> Bytes {
    f418(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)")), v0)
}

pub fn f424(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m2527 = v0;
    match m2527.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f115(v2.clone(), v1.clone()) {
                f424(v3, v1)
            } else {
                List::cons(v2, f424(v3, v1))
            }
        }
    }
}

pub fn f425(p2532: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m2531 = p2532;
    {
        let mut m2531 = m2531;
        loop {
            let step = {
                let v0 = m2531;
                let m2528 = v0;
                match m2528.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h2529, _) = v2;
                            h2529
                        }, v1.clone()) {
                            let (_, h2530) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h2530)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
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

pub fn f426(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2533 = 0u64;
    let a2534 = v1;
    if if nat_lt(a2533, a2534) {
        f178({
            let a2537 = f173(v0, {
                let a2535 = v1;
                let a2536 = 1u64;
                nat_sub(a2535, a2536)
            })?;
            let a2538 = 10u64;
            nat_eq(a2537, a2538)
        })
    } else {
        false
    } {
        Ok(Some({
            let a2539 = v1;
            let a2540 = 1u64;
            nat_sub(a2539, a2540)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f427(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2541 = f173(v0, v1)?;
    let a2542 = 32u64;
    if nat_eq(a2541, a2542) {
        Ok(Some({
            let a2543 = v1;
            let a2544 = 1u64;
            nat_add(a2543, a2544)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f428(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f172(f173(v0, v1)?) {
        Ok(Some({
            let a2545 = v1;
            let a2546 = 1u64;
            nat_add(a2545, a2546)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f429(v0: u64) -> bool {
    let a2547 = v0;
    let a2548 = 44u64;
    if if if nat_eq(a2547, a2548) {
        true
    } else {
        let a2549 = v0;
        let a2550 = 91u64;
        nat_eq(a2549, a2550)
    } {
        true
    } else {
        let a2551 = v0;
        let a2552 = 93u64;
        if nat_eq(a2551, a2552) {
            true
        } else {
            let a2553 = v0;
            let a2554 = 123u64;
            nat_eq(a2553, a2554)
        }
    } {
        true
    } else {
        let a2555 = v0;
        let a2556 = 125u64;
        nat_eq(a2555, a2556)
    }
}

pub fn f430(v0: u64) -> u64 {
    let a2557 = v0;
    let a2558 = 45u64;
    if nat_eq(a2557, a2558) {
        1u64
    } else {
        let a2559 = v0;
        let a2560 = 43u64;
        if nat_eq(a2559, a2560) {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f431(v0: u64) -> u64 {
    let a2561 = 49u64;
    let a2562 = v0;
    if if nat_le(a2561, a2562) {
        let a2563 = v0;
        let a2564 = 57u64;
        nat_le(a2563, a2564)
    } else {
        false
    } {
        let a2565 = v0;
        let a2566 = 48u64;
        nat_sub(a2565, a2566)
    } else {
        0u64
    }
}

pub fn f432(v0: Bytes, v1: u64, v2: u64, v3: bool, v4: u64) -> R<Adt20> {
    let m2567 = f493(v0.clone(), v1, v2)?;
    match m2567 {
        Adt34::C0(v6, v7) => {
            let v8: u64 = v7;
            let v9: u64 = f494(v0.clone(), v8, v2)?;
            if f495(v0.clone(), v8, v2)? {
                let m2571 = f496(v0.clone(), v8, v2, v3, {
                    let a2569 = {
                        let a2568 = v0;
                        length_bytes(a2568)
                    };
                    let a2570 = 2u64;
                    nat_add(a2569, a2570)?
                }, Bytes::lit(&[]), v6)?;
                match m2571 {
                    Adt35::C0(v11, v12, v13) => {
                        Ok(Adt20::C0(Adt19::C5(f7(List::cons(v11, List::cons({
                            let a2572 = v4;
                            let a2573 = 1u64;
                            if nat_eq(a2572, a2573) {
                                Bytes::lit(&[])
                            } else {
                                Bytes::lit(&[0x0a])
                            }
                        }, List::cons({
                            let a2574 = v4;
                            let a2575 = 2u64;
                            if nat_eq(a2574, a2575) {
                                f434(v12)
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
                    let a2576 = v4;
                    let a2577 = 2u64;
                    if nat_eq(a2576, a2577) {
                        f434(v6)
                    } else {
                        Bytes::lit(&[])
                    }
                }), {
                    let a2578 = f173(v0, v9)?;
                    let a2579 = 256u64;
                    if nat_eq(a2578, a2579) {
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

pub fn f433(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f497(v0, v1)
}

pub fn f434(v0: u64) -> Bytes {
    let m2580 = v0;
    if m2580 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2580 - 1;
        let a2581 = Bytes::lit(&[0x0a]);
        let a2582 = f434(v1);
        append_bytes(a2581, a2582)
    }
}

pub fn f435(v0: Bytes, v1: u64) -> R<u64> {
    let (_, h2583) = {
        let (h2584, _) = f399(Fn7::F498(v0.clone()), {
            let a2586 = {
                let a2585 = v0;
                length_bytes(a2585)
            };
            let a2587 = 1u64;
            nat_add(a2586, a2587)?
        }, (v1, 0u64))?;
        h2584
    };
    Ok(h2583)
}

pub fn f436(v0: u64, v1: u64) -> u64 {
    let a2588 = v0;
    let a2589 = v1;
    if nat_lt(a2588, a2589) {
        v1
    } else {
        v0
    }
}

pub fn f437(v0: Bytes, p2648: u64, v2: u64, v3: bool, p2649: u64, p2650: Bytes) -> R<Adt30> {
    let m2647 = (p2648, (p2649, p2650));
    {
        let mut m2647 = m2647;
        loop {
            let step = {
                let (v1, (v4, v5)) = m2647;
                let m2590 = v4;
                if m2590 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f83(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                } else {
                    let v6 = m2590 - 1;
                    let v7: u64 = f173(v0.clone(), v1)?;
                    let a2591 = v7;
                    let a2592 = 34u64;
                    if nat_eq(a2591, a2592) {
                        Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C0(v5, {
                            let a2593 = v1;
                            let a2594 = 1u64;
                            nat_add(a2593, a2594)?
                        })))
                    } else {
                        let a2595 = v7;
                        let a2596 = 256u64;
                        if nat_eq(a2595, a2596) {
                            Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f83(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                        } else {
                            let a2597 = v7;
                            let a2598 = 92u64;
                            if nat_eq(a2597, a2598) {
                                let v8: u64 = f173(v0.clone(), {
                                    let a2599 = v1;
                                    let a2600 = 1u64;
                                    nat_add(a2599, a2600)?
                                })?;
                                let m2601 = f499(v8)?;
                                match m2601 {
                                    None => {
                                        let a2606 = v8;
                                        let a2607 = 10u64;
                                        if nat_eq(a2606, a2607) {
                                            if v3 {
                                                Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f500(v1)?)))
                                            } else {
                                                let m2610 = f501(v0.clone(), {
                                                    let a2608 = v1;
                                                    let a2609 = 1u64;
                                                    nat_add(a2608, a2609)?
                                                }, v2, v6, 0u64)?;
                                                match m2610 {
                                                    Adt34::C0(v11, v12) => {
                                                        Ok::<(u64, (u64, Bytes)), R<Adt30>>((v12, (v6, {
                                                            let a2611 = v5;
                                                            let a2612 = f434(v11);
                                                            append_bytes(a2611, a2612)
                                                        })))
                                                    }
                                                    Adt34::C1(v10) => {
                                                        Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(v10)))
                                                    }
                                                }
                                            }
                                        } else {
                                            let v13: u64 = {
                                                let a2613 = v8;
                                                let a2614 = 120u64;
                                                if nat_eq(a2613, a2614) {
                                                    2u64
                                                } else {
                                                    let a2615 = v8;
                                                    let a2616 = 117u64;
                                                    if nat_eq(a2615, a2616) {
                                                        4u64
                                                    } else {
                                                        let a2617 = v8;
                                                        let a2618 = 85u64;
                                                        if nat_eq(a2617, a2618) {
                                                            8u64
                                                        } else {
                                                            0u64
                                                        }
                                                    }
                                                }
                                            };
                                            let a2619 = v13;
                                            let a2620 = 0u64;
                                            if nat_eq(a2619, a2620) {
                                                Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f83(Str::lit("an escape must be one of YAML 1.2.2 [42] to [62]"), v1)?)))
                                            } else {
                                                let m2623 = f502(v0.clone(), {
                                                    let a2621 = v1;
                                                    let a2622 = 2u64;
                                                    nat_add(a2621, a2622)?
                                                }, v13, 0u64)?;
                                                match m2623 {
                                                    None => {
                                                        Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f83(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                    }
                                                    Some(v14) => {
                                                        let a2624 = 1114111u64;
                                                        let a2625 = v14;
                                                        if if nat_lt(a2624, a2625) {
                                                            true
                                                        } else {
                                                            let a2626 = 55296u64;
                                                            let a2627 = v14;
                                                            if nat_le(a2626, a2627) {
                                                                let a2628 = v14;
                                                                let a2629 = 57343u64;
                                                                nat_le(a2628, a2629)
                                                            } else {
                                                                false
                                                            }
                                                        } {
                                                            Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f83(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                        } else {
                                                            Ok::<(u64, (u64, Bytes)), R<Adt30>>(({
                                                                let a2632 = {
                                                                    let a2630 = v1;
                                                                    let a2631 = 2u64;
                                                                    nat_add(a2630, a2631)?
                                                                };
                                                                let a2633 = v13;
                                                                nat_add(a2632, a2633)?
                                                            }, (v6, {
                                                                let a2634 = v5;
                                                                let a2635 = f314(v14)?;
                                                                append_bytes(a2634, a2635)
                                                            })))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Some(v9) => {
                                        Ok::<(u64, (u64, Bytes)), R<Adt30>>(({
                                            let a2602 = v1;
                                            let a2603 = 2u64;
                                            nat_add(a2602, a2603)?
                                        }, (v6, {
                                            let a2604 = v5;
                                            let a2605 = f314(v9)?;
                                            append_bytes(a2604, a2605)
                                        })))
                                    }
                                }
                            } else {
                                if if f172(v7) {
                                    true
                                } else {
                                    let a2636 = v7;
                                    let a2637 = 10u64;
                                    nat_eq(a2636, a2637)
                                } {
                                    let v15: u64 = f249(v0.clone(), v1)?;
                                    let a2638 = f173(v0.clone(), v15)?;
                                    let a2639 = 10u64;
                                    if nat_eq(a2638, a2639) {
                                        if v3 {
                                            Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f500(v1)?)))
                                        } else {
                                            let m2640 = f501(v0.clone(), v15, v2, v6, 0u64)?;
                                            match m2640 {
                                                Adt34::C0(v17, v18) => {
                                                    Ok::<(u64, (u64, Bytes)), R<Adt30>>((v18, (v6, {
                                                        let a2641 = v5;
                                                        let a2642 = f445(v17);
                                                        append_bytes(a2641, a2642)
                                                    })))
                                                }
                                                Adt34::C1(v16) => {
                                                    Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(v16)))
                                                }
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (u64, Bytes)), R<Adt30>>((v15, (v6, {
                                            let a2643 = v5;
                                            let a2644 = f356(v0.clone(), v1, v15);
                                            append_bytes(a2643, a2644)
                                        })))
                                    }
                                } else {
                                    let v19: u64 = f503(v0.clone(), v1)?;
                                    Ok::<(u64, (u64, Bytes)), R<Adt30>>((v19, (v6, {
                                        let a2645 = v5;
                                        let a2646 = f356(v0.clone(), v1, v19);
                                        append_bytes(a2645, a2646)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2647 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f438(v0: Bytes, p2678: u64, v2: u64, v3: bool, p2679: u64, p2680: Bytes) -> R<Adt30> {
    let m2677 = (p2678, (p2679, p2680));
    {
        let mut m2677 = m2677;
        loop {
            let step = {
                let (v1, (v4, v5)) = m2677;
                let m2651 = v4;
                if m2651 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f83(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                } else {
                    let v6 = m2651 - 1;
                    let v7: u64 = f173(v0.clone(), v1)?;
                    let a2652 = v7;
                    let a2653 = 39u64;
                    if nat_eq(a2652, a2653) {
                        let a2656 = f173(v0.clone(), {
                            let a2654 = v1;
                            let a2655 = 1u64;
                            nat_add(a2654, a2655)?
                        })?;
                        let a2657 = 39u64;
                        if nat_eq(a2656, a2657) {
                            Ok::<(u64, (u64, Bytes)), R<Adt30>>(({
                                let a2658 = v1;
                                let a2659 = 2u64;
                                nat_add(a2658, a2659)?
                            }, (v6, {
                                let a2660 = v5;
                                let a2661 = Bytes::lit(&[0x27]);
                                append_bytes(a2660, a2661)
                            })))
                        } else {
                            Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C0(v5, {
                                let a2662 = v1;
                                let a2663 = 1u64;
                                nat_add(a2662, a2663)?
                            })))
                        }
                    } else {
                        let a2664 = v7;
                        let a2665 = 256u64;
                        if nat_eq(a2664, a2665) {
                            Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f83(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                        } else {
                            if if f172(v7) {
                                true
                            } else {
                                let a2666 = v7;
                                let a2667 = 10u64;
                                nat_eq(a2666, a2667)
                            } {
                                let v8: u64 = f249(v0.clone(), v1)?;
                                let a2668 = f173(v0.clone(), v8)?;
                                let a2669 = 10u64;
                                if nat_eq(a2668, a2669) {
                                    if v3 {
                                        Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f500(v1)?)))
                                    } else {
                                        let m2670 = f501(v0.clone(), v8, v2, v6, 0u64)?;
                                        match m2670 {
                                            Adt34::C0(v10, v11) => {
                                                Ok::<(u64, (u64, Bytes)), R<Adt30>>((v11, (v6, {
                                                    let a2671 = v5;
                                                    let a2672 = f445(v10);
                                                    append_bytes(a2671, a2672)
                                                })))
                                            }
                                            Adt34::C1(v9) => {
                                                Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(v9)))
                                            }
                                        }
                                    }
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Adt30>>((v8, (v6, {
                                        let a2673 = v5;
                                        let a2674 = f356(v0.clone(), v1, v8);
                                        append_bytes(a2673, a2674)
                                    })))
                                }
                            } else {
                                let v12: u64 = f504(v0.clone(), v1)?;
                                Ok::<(u64, (u64, Bytes)), R<Adt30>>((v12, (v6, {
                                    let a2675 = v5;
                                    let a2676 = f356(v0.clone(), v1, v12);
                                    append_bytes(a2675, a2676)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2677 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f439(v0: Bytes, v1: u64, v2: u64) -> R<Result<u64, Bytes>> {
    let v3: (u64, bool) = {
        let (h2681, _) = f505(Fn8::F506(v2, v0.clone()), {
            let a2683 = {
                let a2682 = v0.clone();
                length_bytes(a2682)
            };
            let a2684 = 1u64;
            nat_add(a2683, a2684)?
        }, (v1, false))?;
        h2681
    };
    let (_, h2685) = v3;
    if h2685 {
        if f103(v0, {
            let (h2686, _) = v3;
            h2686
        })? {
            Ok(Err::<u64, Bytes>(f83(Str::lit("a document marker cannot appear inside a flow collection (YAML 1.2.2 [206])"), {
                let (h2687, _) = v3;
                h2687
            })?))
        } else {
            Ok(Err::<u64, Bytes>(f83(Str::lit("a line of a flow collection is indented less than its node (YAML 1.2.2 [80])"), {
                let (h2688, _) = v3;
                h2688
            })?))
        }
    } else {
        Ok(Ok::<u64, Bytes>({
            let (h2689, _) = v3;
            h2689
        }))
    }
}

pub fn f440(v0: Bytes, v1: u64, v2: u64) -> R<Adt30> {
    let v3: u64 = f173(v0.clone(), v1)?;
    let a2690 = v3;
    let a2691 = 34u64;
    if if nat_eq(a2690, a2691) {
        true
    } else {
        let a2692 = v3;
        let a2693 = 39u64;
        nat_eq(a2692, a2693)
    } {
        f351(v0, v1, v2, false)
    } else {
        let m2694 = f252(v0.clone(), v1)?;
        match m2694 {
            None => {
                let a2695 = v3;
                let a2696 = 91u64;
                if if nat_eq(a2695, a2696) {
                    true
                } else {
                    let a2697 = v3;
                    let a2698 = 123u64;
                    nat_eq(a2697, a2698)
                } {
                    Ok(Adt30::C1(f83(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [142])"), v1)?))
                } else {
                    if f353(v0.clone(), v1, true)? {
                        let v5: u64 = f354(v0.clone(), v1, true)?;
                        Ok(Adt30::C0(f356(v0, v1, v5), v5))
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

pub fn f441(v0: u64) -> R<bool> {
    let a2699 = f507(f8(Str::lit(",[]{}-?:#&*!|>'\"@`")), v0)?;
    let a2700 = 18u64;
    if nat_lt(a2699, a2700) {
        Ok(true)
    } else {
        let a2701 = v0;
        let a2702 = 37u64;
        Ok(nat_eq(a2701, a2702))
    }
}

pub fn f442(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f508(v1, v0, v2)
}

pub fn f443(v0: u64) -> R<Bytes> {
    f83(Str::lit("a mapping value is not allowed here: a plain scalar holds ': ' (YAML 1.2.2 [130])"), v0)
}

pub fn f444(v0: Bytes, v1: u64) -> R<(u64, u64)> {
    let (h2703, _) = f399(Fn7::F509(v0.clone()), {
        let a2705 = {
            let a2704 = v0;
            length_bytes(a2704)
        };
        let a2706 = 1u64;
        nat_add(a2705, a2706)?
    }, (v1, 0u64))?;
    Ok(h2703)
}

pub fn f445(v0: u64) -> Bytes {
    let a2707 = v0;
    let a2708 = 0u64;
    if nat_eq(a2707, a2708) {
        Bytes::lit(&[0x20])
    } else {
        f434(v0)
    }
}

pub fn f446(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt20> {
    let v4: (u64, u64) = f510(v0.clone(), v1, 2u64)?;
    let a2711 = {
        let (h2709, _) = v4;
        h2709
    };
    let a2712 = {
        let a2710 = v0.clone();
        length_bytes(a2710)
    };
    if nat_eq(a2711, a2712) {
        if f511(false, {
            let (_, h2713) = v4;
            h2713
        }) {
            Ok(Adt20::C0(Adt19::C3(false, {
                let (_, h2714) = v4;
                h2714
            }), v3))
        } else {
            Ok(Adt20::C1(f449(v2)?))
        }
    } else {
        Ok(Adt20::C0(Adt19::C5(v0), v3))
    }
}

pub fn f447(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2715 = v0.clone();
        length_bytes(a2715)
    };
    let v2: u64 = f173(v0.clone(), 0u64)?;
    let v3: u64 = {
        let a2716 = v2;
        let a2717 = 43u64;
        if if nat_eq(a2716, a2717) {
            true
        } else {
            let a2718 = v2;
            let a2719 = 45u64;
            nat_eq(a2718, a2719)
        } {
            1u64
        } else {
            0u64
        }
    };
    let v4: u64 = f512(v0.clone(), v3)?;
    let v5: bool = {
        let a2720 = f173(v0.clone(), v4)?;
        let a2721 = 46u64;
        nat_eq(a2720, a2721)
    };
    let v6: u64 = if v5 {
        let a2722 = v4;
        let a2723 = 1u64;
        nat_add(a2722, a2723)?
    } else {
        v4
    };
    let v7: u64 = if v5 {
        f512(v0.clone(), v6)?
    } else {
        v4
    };
    let v8: u64 = f173(v0.clone(), v7)?;
    let v9: bool = {
        let a2724 = v8;
        let a2725 = 101u64;
        if nat_eq(a2724, a2725) {
            true
        } else {
            let a2726 = v8;
            let a2727 = 69u64;
            nat_eq(a2726, a2727)
        }
    };
    let v10: u64 = f173(v0.clone(), {
        let a2728 = v7;
        let a2729 = 1u64;
        nat_add(a2728, a2729)?
    })?;
    let v11: bool = if v9 {
        let a2730 = v10;
        let a2731 = 43u64;
        if nat_eq(a2730, a2731) {
            true
        } else {
            let a2732 = v10;
            let a2733 = 45u64;
            nat_eq(a2732, a2733)
        }
    } else {
        false
    };
    let v12: u64 = if v9 {
        if v11 {
            let a2734 = v7;
            let a2735 = 2u64;
            nat_add(a2734, a2735)?
        } else {
            let a2736 = v7;
            let a2737 = 1u64;
            nat_add(a2736, a2737)?
        }
    } else {
        v7
    };
    let v13: u64 = if v9 {
        f512(v0.clone(), v12)?
    } else {
        v7
    };
    let v14: Bytes = f356(v0.clone(), v3, v4);
    let v15: u64 = f513(v14.clone())?;
    let a2738 = v13;
    let a2739 = v1;
    if if if nat_eq(a2738, a2739) {
        let a2740 = v3;
        let a2741 = v4;
        if nat_lt(a2740, a2741) {
            true
        } else {
            if v5 {
                let a2742 = v6;
                let a2743 = v7;
                nat_lt(a2742, a2743)
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
            let a2744 = v12;
            let a2745 = v13;
            nat_lt(a2744, a2745)
        }
    } else {
        false
    } {
        Ok(Some(f7(List::cons({
            let a2746 = v2;
            let a2747 = 45u64;
            if nat_eq(a2746, a2747) {
                Bytes::lit(&[0x2d])
            } else {
                Bytes::lit(&[])
            }
        }, List::cons({
            let a2749 = v15;
            let a2750 = {
                let a2748 = v14.clone();
                length_bytes(a2748)
            };
            if nat_eq(a2749, a2750) {
                Bytes::lit(&[0x30])
            } else {
                f356(v14.clone(), v15, {
                    let a2751 = v14;
                    length_bytes(a2751)
                })
            }
        }, List::cons({
            let a2752 = v6;
            let a2753 = v7;
            if nat_lt(a2752, a2753) {
                let a2754 = Bytes::lit(&[0x2e]);
                let a2755 = f356(v0.clone(), v6, v7);
                append_bytes(a2754, a2755)
            } else {
                Bytes::lit(&[])
            }
        }, List::cons(if v9 {
            let a2758 = if if v11 {
                let a2756 = v10;
                let a2757 = 45u64;
                nat_eq(a2756, a2757)
            } else {
                false
            } {
                Bytes::lit(&[0x65, 0x2d])
            } else {
                Bytes::lit(&[0x65])
            };
            let a2759 = f356(v0, v12, v13);
            append_bytes(a2758, a2759)
        } else {
            Bytes::lit(&[])
        }, List::<Bytes>::nil())))))))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f448(v0: Bytes, v1: u64) -> R<Option<(u64, Adt19)>> {
    let v2: bool = {
        let a2760 = f139(v0.clone(), v1)?;
        let a2761 = 45u64;
        nat_eq(a2760, a2761)
    };
    let v3: u64 = if v2 {
        let a2762 = v1;
        let a2763 = 1u64;
        nat_add(a2762, a2763)?
    } else {
        v1
    };
    let v4: u64 = f512(v0.clone(), v3)?;
    let v5: u64 = {
        let a2764 = v4;
        let a2765 = v3;
        nat_sub(a2764, a2765)
    };
    let v6: bool = {
        let a2766 = f139(v0.clone(), v4)?;
        let a2767 = 46u64;
        nat_eq(a2766, a2767)
    };
    let v7: u64 = if v6 {
        let a2768 = v4;
        let a2769 = 1u64;
        nat_add(a2768, a2769)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f512(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f139(v0.clone(), v8)?;
    let v10: bool = {
        let a2770 = v9;
        let a2771 = 101u64;
        if nat_eq(a2770, a2771) {
            true
        } else {
            let a2772 = v9;
            let a2773 = 69u64;
            nat_eq(a2772, a2773)
        }
    };
    let v11: u64 = f139(v0.clone(), {
        let a2774 = v8;
        let a2775 = 1u64;
        nat_add(a2774, a2775)?
    })?;
    let v12: bool = f321(v10, f323({
        let a2776 = v11;
        let a2777 = 43u64;
        nat_eq(a2776, a2777)
    }, {
        let a2778 = v11;
        let a2779 = 45u64;
        nat_eq(a2778, a2779)
    }));
    let v13: u64 = if v10 {
        if v12 {
            let a2780 = v8;
            let a2781 = 2u64;
            nat_add(a2780, a2781)?
        } else {
            let a2782 = v8;
            let a2783 = 1u64;
            nat_add(a2782, a2783)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f512(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f32(v0.clone(), v13, {
        let a2784 = v14;
        let a2785 = v13;
        nat_sub(a2784, a2785)
    });
    let v16: u64 = f513(v15.clone())?;
    let v17: Bytes = f32(v15.clone(), v16, {
        let a2787 = {
            let a2786 = v15;
            length_bytes(a2786)
        };
        let a2788 = v16;
        nat_sub(a2787, a2788)
    });
    let a2789 = v5;
    let a2790 = 0u64;
    if if nat_eq(a2789, a2790) {
        true
    } else {
        if f321({
            let a2791 = 1u64;
            let a2792 = v5;
            nat_lt(a2791, a2792)
        }, {
            let a2793 = f139(v0.clone(), v3)?;
            let a2794 = 48u64;
            nat_eq(a2793, a2794)
        }) {
            true
        } else {
            if f321(v6, {
                let a2795 = v8;
                let a2796 = v7;
                nat_eq(a2795, a2796)
            }) {
                true
            } else {
                f323(if v10 {
                    let a2797 = v14;
                    let a2798 = v13;
                    nat_eq(a2797, a2798)
                } else {
                    false
                }, {
                    let a2800 = 18u64;
                    let a2801 = {
                        let a2799 = v17.clone();
                        length_bytes(a2799)
                    };
                    nat_lt(a2800, a2801)
                })
            }
        }
    } {
        Ok(None::<(u64, Adt19)>)
    } else {
        let m2805 = {
            let a2803 = {
                let a2802 = v17.clone();
                length_bytes(a2802)
            };
            let a2804 = 0u64;
            if nat_eq(a2803, a2804) {
                Some(0u64)
            } else {
                f514(v17)?
            }
        };
        match m2805 {
            None => {
                Ok(None::<(u64, Adt19)>)
            }
            Some(v18) => {
                let m2810 = f515(f32(v0.clone(), v3, v5), f32(v0, v7, {
                    let a2806 = v8;
                    let a2807 = v7;
                    nat_sub(a2806, a2807)
                }), f321(v12, {
                    let a2808 = v11;
                    let a2809 = 45u64;
                    nat_eq(a2808, a2809)
                }), v18)?;
                match m2810 {
                    None => {
                        Ok(None::<(u64, Adt19)>)
                    }
                    Some(v19) => {
                        let a2813 = 19u64;
                        let a2814 = {
                            let a2812 = {
                                let (h2811, _) = v19.clone();
                                h2811
                            };
                            length_bytes(a2812)
                        };
                        if nat_lt(a2813, a2814) {
                            Ok(None::<(u64, Adt19)>)
                        } else {
                            let m2816 = f514({
                                let (h2815, _) = v19.clone();
                                h2815
                            })?;
                            match m2816 {
                                None => {
                                    Ok(None::<(u64, Adt19)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f321(v2, {
                                        let a2817 = 0u64;
                                        let a2818 = v20;
                                        nat_lt(a2817, a2818)
                                    });
                                    if f511(v21, v20) {
                                        Ok(Some((v14, {
                                            let a2820 = {
                                                let (_, h2819) = v19.clone();
                                                h2819
                                            };
                                            let a2821 = 0u64;
                                            if nat_eq(a2820, a2821) {
                                                Adt19::C3(v21, v20)
                                            } else {
                                                Adt19::C4(v21, v20, {
                                                    let (_, h2822) = v19;
                                                    h2822
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

pub fn f449(v0: u64) -> R<Bytes> {
    f83(Str::lit("a number outside the range Studio.Value holds is not read (YAML 1.2.2 section 10.3.2)"), v0)
}

pub fn f450(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f516(v0, v1)
}

pub fn f451(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f517(v0, v1)
}

pub fn f452(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m2823 = v0.clone();
    match m2823.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a2826 = {
                let (h2824, _) = v2;
                h2824
            };
            let a2827 = {
                let (h2825, _) = v1.clone();
                h2825
            };
            if equal(a2826, a2827) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f453(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f518(f519(Fn9::F520, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m2832 = {
        let (h2828, _) = f521(Fn10::F522, {
            let a2830 = {
                let a2829 = v0;
                length_list(a2829)
            };
            let a2831 = 1u64;
            nat_add(a2830, a2831)?
        }, v1)?;
        h2828
    };
    match m2832.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f454(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2837 = {
        let a2834 = f362({
            let (h2833, _) = v0;
            h2833
        })?;
        let a2835 = Bytes::lit(&[0x3a]);
        append_bytes(a2834, a2835)
    };
    let a2838 = {
        let (_, h2836) = v0;
        h2836
    };
    Ok(append_bytes(a2837, a2838))
}

pub fn f455(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    if f523(v1, f87(v0, v2)?) {
        Ok(Some({
            let a2839 = v2;
            let a2840 = 1u64;
            nat_add(a2839, a2840)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f456(p2843: List<Adt13>, p2844: List<Adt13>) -> List<Adt13> {
    let m2842 = (p2843, p2844);
    {
        let mut m2842 = m2842;
        loop {
            let step = {
                let (v0, v1) = m2842;
                let m2841 = v0;
                match m2841.uncons() {
                    None => {
                        Err::<(List<Adt13>, List<Adt13>), List<Adt13>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt13>, List<Adt13>), List<Adt13>>((v3, List::cons(v2, v1)))
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

pub fn f457(v0: Bytes, p2849: List<(Bytes, Adt14)>) -> Option<Adt14> {
    let m2848 = p2849;
    {
        let mut m2848 = m2848;
        loop {
            let step = {
                let v1 = m2848;
                let m2845 = v1;
                match m2845.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt14)>, Option<Adt14>>(None::<Adt14>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h2846, _) = v2;
                            h2846
                        }, v0.clone()) {
                            Err::<List<(Bytes, Adt14)>, Option<Adt14>>(Some({
                                let (_, h2847) = v2;
                                h2847
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt14)>, Option<Adt14>>(v3)
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

pub fn f458() -> List<(Bytes, Adt14)> {
    List::cons((f8(Str::lit("boolean")), f284(f8(Str::lit("Bool")))), List::cons((f8(Str::lit("byte")), f284(f8(Str::lit("Int")))), List::cons((f8(Str::lit("octet")), f284(f8(Str::lit("Nat")))), List::cons((f8(Str::lit("bigint")), f284(f8(Str::lit("Int")))), List::cons((f8(Str::lit("float")), f284(f408())), List::cons((f8(Str::lit("double")), f284(f408())), List::cons((f8(Str::lit("DOMString")), f284(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("USVString")), f284(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ByteString")), f284(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ArrayBuffer")), f284(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("SharedArrayBuffer")), f284(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("DataView")), f284(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("Int8Array")), f284(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int16Array")), f284(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int32Array")), f284(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("BigInt64Array")), f284(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Uint8Array")), f284(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint16Array")), f284(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint32Array")), f284(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint8ClampedArray")), f284(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("BigUint64Array")), f284(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Float16Array")), f384(f8(Str::lit("Float16Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("Float32Array")), f384(f8(Str::lit("Float32Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("Float64Array")), f384(f8(Str::lit("Float64Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("any")), f384(f8(Str::lit("any is not translated, because it is the union of every Web IDL type and has no one LexLean type (Web IDL 2.13.1)")))), List::cons((f8(Str::lit("object")), f384(f8(Str::lit("object is not translated, because its values are references to JavaScript objects of any shape, which no LexLean type holds (Web IDL 2.13.20)")))), List::cons((f8(Str::lit("symbol")), f384(f8(Str::lit("symbol is not translated, because a symbol value has identity but no content that a LexLean type holds (Web IDL 2.13.21)")))), List::cons((f8(Str::lit("undefined")), f384(f8(Str::lit("undefined is not translated, because it is the type of the absence of a value (Web IDL 2.13.2)")))), List::<(Bytes, Adt14)>::nil()))))))))))))))))))))))))))))
}

pub fn f459() -> Bytes {
    f8(Str::lit("a union type is translated only as the whole body of a typedef, whose identifier names its inductive (Web IDL 2.11 and 2.13.32)"))
}

pub fn f460(p2860: u64, v1: Bytes, v2: Bytes, p2861: u64) -> R<bool> {
    let m2859 = (p2860, p2861);
    {
        let mut m2859 = m2859;
        loop {
            let step = {
                let (v0, v3) = m2859;
                let m2850 = v0;
                if m2850 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(false))
                } else {
                    let v4 = m2850 - 1;
                    let v5: u64 = f524(v1.clone(), v3)?;
                    let v6: u64 = f524(v2.clone(), v3)?;
                    let a2851 = v5;
                    let a2852 = v6;
                    if nat_lt(a2851, a2852) {
                        Err::<(u64, u64), R<bool>>(Ok(true))
                    } else {
                        let a2853 = v6;
                        let a2854 = v5;
                        if nat_lt(a2853, a2854) {
                            Err::<(u64, u64), R<bool>>(Ok(false))
                        } else {
                            let a2855 = v5;
                            let a2856 = 0u64;
                            if nat_eq(a2855, a2856) {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            } else {
                                Ok::<(u64, u64), R<bool>>((v4, {
                                    let a2857 = v3;
                                    let a2858 = 1u64;
                                    nat_add(a2857, a2858)?
                                }))
                            }
                        }
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

pub fn f461(p2874: u64, v1: Bytes, p2875: u64, p2876: Bytes) -> R<Option<Bytes>> {
    let m2873 = (p2874, (p2875, p2876));
    {
        let mut m2873 = m2873;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2873;
                let m2862 = v0;
                if m2862 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m2862 - 1;
                    let a2864 = {
                        let a2863 = v1.clone();
                        length_bytes(a2863)
                    };
                    let a2865 = v2;
                    if nat_le(a2864, a2865) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m2866 = f307(v1.clone(), v2)?;
                        match m2866 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a2868 = v2;
                                    let a2869 = {
                                        let (_, h2867) = v5;
                                        h2867
                                    };
                                    nat_add(a2868, a2869)?
                                }, {
                                    let a2871 = v3;
                                    let a2872 = f525(v1.clone(), v2, {
                                        let (h2870, _) = v5;
                                        h2870
                                    });
                                    append_bytes(a2871, a2872)
                                })))
                            }
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

pub fn f462(p2886: u64, v1: Bytes, p2887: u64) -> R<bool> {
    let m2885 = (p2886, p2887);
    {
        let mut m2885 = m2885;
        loop {
            let step = {
                let (v0, v2) = m2885;
                let m2877 = v0;
                if m2877 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m2877 - 1;
                    let m2880 = {
                        let a2878 = v1.clone();
                        let a2879 = v2;
                        index_bytes(a2878, a2879)
                    };
                    match m2880 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a2881 = f42(v4)?;
                            let a2882 = 128u64;
                            if nat_lt(a2881, a2882) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a2883 = v2;
                                    let a2884 = 1u64;
                                    nat_add(a2883, a2884)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
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

pub fn f463(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f223(v0.clone(), 0u64, {
        let a2888 = v0;
        length_bytes(a2888)
    }, false, Bytes::lit(&[]))?;
    let m2891 = {
        let a2889 = v1.clone();
        let a2890 = 0u64;
        index_bytes(a2889, a2890)
    };
    match m2891 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f42(v2)?;
            if f324(v3) {
                let a2898 = f32({
                    let a2892 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a2892)
                }, {
                    let a2893 = v3;
                    let a2894 = 65u64;
                    nat_sub(a2893, a2894)
                }, 1u64);
                let a2899 = f32(v1.clone(), 1u64, {
                    let a2896 = {
                        let a2895 = v1;
                        length_bytes(a2895)
                    };
                    let a2897 = 1u64;
                    nat_sub(a2896, a2897)
                });
                Ok(append_bytes(a2898, a2899))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f464(v0: Bytes) -> R<bool> {
    let m2902 = {
        let a2900 = v0;
        let a2901 = 0u64;
        index_bytes(a2900, a2901)
    };
    match m2902 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f323(f322(f42(v1)?), f324(f42(v1)?)))
        }
    }
}

pub fn f465(p2915: u64, v1: Bytes, p2916: u64, p2917: Bytes) -> R<Option<Bytes>> {
    let m2914 = (p2915, (p2916, p2917));
    {
        let mut m2914 = m2914;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2914;
                let m2903 = v0;
                if m2903 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m2903 - 1;
                    let a2905 = {
                        let a2904 = v1.clone();
                        length_bytes(a2904)
                    };
                    let a2906 = v2;
                    if nat_le(a2905, a2906) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m2907 = f307(v1.clone(), v2)?;
                        match m2907 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a2909 = v2;
                                    let a2910 = {
                                        let (_, h2908) = v5;
                                        h2908
                                    };
                                    nat_add(a2909, a2910)?
                                }, {
                                    let a2912 = v3;
                                    let a2913 = f526({
                                        let (h2911, _) = v5;
                                        h2911
                                    });
                                    append_bytes(a2912, a2913)
                                })))
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

pub fn f466() -> List<Bytes> {
    List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("termination_by")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("admit")), List::cons(f8(Str::lit("below")), List::cons(f8(Str::lit("brecOn")), List::cons(f8(Str::lit("casesOn")), List::cons(f8(Str::lit("ctorElim")), List::cons(f8(Str::lit("ctorElimType")), List::cons(f8(Str::lit("ctorIdx")), List::cons(f8(Str::lit("noConfusion")), List::cons(f8(Str::lit("noConfusionType")), List::cons(f8(Str::lit("rec")), List::cons(f8(Str::lit("recOn")), List::cons(f8(Str::lit("toCtorIdx")), List::<Bytes>::nil())))))))))))))))))))
}

pub fn f467(v0: Bytes) -> R<Bytes> {
    let m2920 = {
        let a2918 = v0.clone();
        let a2919 = 0u64;
        index_bytes(a2918, a2919)
    };
    match m2920 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            let a2926 = {
                let a2921 = f8(Str::lit("u"));
                let a2922 = f527(8u64, f42(v1)?, 4u64, Bytes::lit(&[]));
                append_bytes(a2921, a2922)
            };
            let a2927 = f32(v0.clone(), 1u64, {
                let a2924 = {
                    let a2923 = v0;
                    length_bytes(a2923)
                };
                let a2925 = 1u64;
                nat_sub(a2924, a2925)
            });
            Ok(append_bytes(a2926, a2927))
        }
    }
}

pub fn f468() -> Bytes {
    f8(Str::lit("\u{0}\u{0}:\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{5}\u{7f}\u{0}\u{6}p\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f469() -> Bytes {
    f8(Str::lit("\u{0}\u{0}-\u{0}\u{0}.\u{0}\u{0}0\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}7\u{0}\u{1}7\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@?\u{0}@@\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f470(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f87(v0, {
        let (h2928, _) = v2;
        h2928
    })?;
    let a2930 = 1114111u64;
    let a2931 = {
        let (_, h2929) = v2;
        h2929
    };
    if nat_lt(a2930, a2931) {
        Ok(None::<(u64, u64)>)
    } else {
        let v4: u64 = f528(v3, v1);
        let a2932 = v4;
        let a2933 = 16u64;
        if nat_eq(a2932, a2933) {
            Ok(None::<(u64, u64)>)
        } else {
            Ok(Some(({
                let a2935 = {
                    let (h2934, _) = v2;
                    h2934
                };
                let a2936 = 1u64;
                nat_add(a2935, a2936)?
            }, {
                let a2940 = {
                    let a2938 = {
                        let (_, h2937) = v2;
                        h2937
                    };
                    let a2939 = if v1 {
                        16u64
                    } else {
                        10u64
                    };
                    nat_mul(a2938, a2939)?
                };
                let a2941 = v4;
                nat_add(a2940, a2941)?
            })))
        }
    }
}

pub fn f471() -> Bytes {
    f8(Str::lit("\u{80}\u{81}\u{82}\u{83}\u{84}\u{85}\u{86}\u{87}\u{88}\u{89}\u{8a}\u{8b}\u{8c}\u{8d}\u{8e}\u{8f}\u{90}\u{91}\u{92}\u{93}\u{94}\u{95}\u{96}\u{97}\u{98}\u{99}\u{9a}\u{9b}\u{9c}\u{9d}\u{9e}\u{9f}\u{a0}\u{a1}\u{a2}\u{a3}\u{a4}\u{a5}\u{a6}\u{a7}\u{a8}\u{a9}\u{aa}\u{ab}\u{ac}\u{ad}\u{ae}\u{af}\u{b0}\u{b1}\u{b2}\u{b3}\u{b4}\u{b5}\u{b6}\u{b7}\u{b8}\u{b9}\u{ba}\u{bb}\u{bc}\u{bd}\u{be}\u{bf}"))
}

pub fn f472(v0: u64, v1: bool, v2: u64) -> R<bool> {
    if v1 {
        let a2942 = v0;
        let a2943 = v2;
        if nat_eq(a2942, a2943) {
            Ok(true)
        } else {
            f207(v0, f529())
        }
    } else {
        f207(v0, f530())
    }
}

pub fn f473() -> Bytes {
    f8(Str::lit(" string:anyAtomicType boolean:anyAtomicType decimal:anyAtomicType float:anyAtomicType double:anyAtomicType duration:anyAtomicType dateTime:anyAtomicType time:anyAtomicType date:anyAtomicType gYearMonth:anyAtomicType gYear:anyAtomicType gMonthDay:anyAtomicType gDay:anyAtomicType gMonth:anyAtomicType hexBinary:anyAtomicType base64Binary:anyAtomicType anyURI:anyAtomicType QName:anyAtomicType NOTATION:anyAtomicType normalizedString:string token:normalizedString language:token NMTOKEN:token Name:token NCName:Name ID:NCName IDREF:NCName ENTITY:NCName integer:decimal nonPositiveInteger:integer long:integer nonNegativeInteger:integer negativeInteger:nonPositiveInteger int:long short:int byte:short unsignedLong:nonNegativeInteger positiveInteger:nonNegativeInteger unsignedInt:unsignedLong unsignedShort:unsignedInt unsignedByte:unsignedShort yearMonthDuration:duration dayTimeDuration:duration dateTimeStamp:dateTime NMTOKENS:anySimpleType IDREFS:anySimpleType ENTITIES:anySimpleType anyAtomicType:anySimpleType "))
}

pub fn f474(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a2944 = v0.clone();
        length_bytes(a2944)
    };
    let v3: u64 = {
        let a2945 = v1.clone();
        length_bytes(a2945)
    };
    let a2946 = v3;
    let a2947 = v2;
    if nat_le(a2946, a2947) {
        let m2953 = {
            let a2950 = v0;
            let a2951 = {
                let a2948 = v2;
                let a2949 = v3;
                nat_sub(a2948, a2949)
            };
            let a2952 = v3;
            slice_bytes(a2950, a2951, a2952)
        };
        match m2953 {
            None => {
                false
            }
            Some(v4) => {
                let a2954 = v4;
                let a2955 = v1;
                equal(a2954, a2955)
            }
        }
    } else {
        false
    }
}

pub fn f475(v0: Adt25, v1: Str, v2: Str) -> bool {
    let m2956 = f241(v0, v1);
    match m2956 {
        None => {
            false
        }
        Some(v3) => {
            f35(v3, f8(v2))
        }
    }
}

pub fn f476(v0: List<Adt24>, v1: (Bytes, Bytes), v2: bool) -> R<Result<List<Bytes>, Bytes>> {
    let m2957 = v0;
    match m2957.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m2958 = f476(v4, v1.clone(), v2)?;
            match m2958 {
                Ok(v6) => {
                    if f35(match v3.clone() {
                        Adt24::C0(h2959, _, _, _) => {
                            h2959
                        }
                    }, f8(Str::lit("complexType"))) {
                        let m2960 = f531(v3.clone())?;
                        match m2960 {
                            Ok(v8) => {
                                let m2961 = v8;
                                match m2961 {
                                    None => {
                                        Ok(Ok::<List<Bytes>, Bytes>(v6))
                                    }
                                    Some(v9) => {
                                        if if f35({
                                            let (h2962, _) = {
                                                let (_, h2963) = v9.clone();
                                                h2963
                                            };
                                            h2962
                                        }, {
                                            let (h2964, _) = v1;
                                            h2964
                                        }) {
                                            if f35({
                                                let (_, h2965) = {
                                                    let (_, h2966) = v9;
                                                    h2966
                                                };
                                                h2965
                                            }, {
                                                let (_, h2967) = v1;
                                                h2967
                                            }) {
                                                if f239(v2) {
                                                    true
                                                } else {
                                                    f240({
                                                        let (h2968, _) = v9;
                                                        h2968
                                                    }, Str::lit("extension"))
                                                }
                                            } else {
                                                false
                                            }
                                        } else {
                                            false
                                        } {
                                            Ok(Ok::<List<Bytes>, Bytes>(List::cons(match v3 {
                                                Adt24::C0(_, _, h2969, _) => {
                                                    h2969
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

pub fn f477(v0: Bytes, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<Adt26, Bytes>> {
    let m2970 = v1.clone();
    match m2970.uncons() {
        None => {
            Ok(Err::<Adt26, Bytes>(f418(f8(Str::lit("a choice with no particle that can occur matches no element sequence, so it has no LexLean constructor (XSD 1.1 Part 1, section 3.8.4.3)")), v0)))
        }
        Some((_, _)) => {
            let m2971 = f532(v1)?;
            match m2971 {
                Ok(v7) => {
                    let v8: List<Bytes> = f533(v7, v2);
                    Ok(Ok::<Adt26, Bytes>(Adt26::C0(v0.clone(), v3, f197(v0, v8.clone()), v8)))
                }
                Err(v6) => {
                    Ok(Err::<Adt26, Bytes>(f418(v6, v0)))
                }
            }
        }
    }
}

pub fn f478(v0: List<Bytes>) -> R<List<Bytes>> {
    let m2972 = v0;
    match m2972.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f154(v1)?, f478(v2)?))
        }
    }
}

pub fn f479(v0: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: Adt25) -> R<Result<List<Adt26>, Bytes>> {
    let m2973 = v0;
    if m2973 == 0 {
        Ok(Err::<List<Adt26>, Bytes>(f418(f8(Str::lit("the complex types nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m2973 - 1;
        let m2974 = f534(v5, v1, v2.clone(), v3, v4, f421())?;
        match m2974 {
            Ok(v7) => {
                let m2976 = f487(v2, match v7.clone() {
                    Adt32::C0(h2975, _, _, _, _) => {
                        h2975
                    }
                })?;
                match m2976 {
                    Ok(v9) => {
                        Ok(Ok::<List<Adt26>, Bytes>(f330(match v7 {
                            Adt32::C0(_, h2977, _, _, _) => {
                                h2977
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

pub fn f480(v0: Bytes) -> Bytes {
    let a2978 = v0;
    let a2979 = f8(Str::lit("Content"));
    append_bytes(a2978, a2979)
}

pub fn f481(v0: Adt23, v1: Adt25, v2: Bytes) -> R<Result<Adt22, Bytes>> {
    let m2981 = f91(match v1 {
        Adt25::C0(_, _, _, _, h2980) => {
            h2980
        }
    }, v2.clone())?;
    match m2981 {
        Ok(v4) => {
            if f35({
                let (h2982, _) = v4.clone();
                h2982
            }, f151()) {
                if f35({
                    let (_, h2983) = v4.clone();
                    h2983
                }, f8(Str::lit("anyType"))) {
                    Ok(Ok::<Adt22, Bytes>(Adt22::C0(f333(), List::cons(f333(), List::<Bytes>::nil()))))
                } else {
                    let m2985 = f317({
                        let (_, h2984) = v4;
                        h2984
                    }, 16u64)?;
                    match m2985 {
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
                    let (h2986, _) = v4.clone();
                    h2986
                }, match v0.clone() {
                    Adt23::C0(_, h2987) => {
                        h2987
                    }
                }) {
                    let v7: Bytes = f154({
                        let (_, h2988) = v4.clone();
                        h2988
                    })?;
                    let m2989 = f488(v0.clone(), Str::lit("complexType"), v4.clone());
                    match m2989 {
                        None => {
                            let m2990 = f488(v0, Str::lit("simpleType"), v4);
                            match m2990 {
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
                        let (h2991, _) = v4;
                        h2991
                    }, f8(Str::lit(", which no schema document of this set targets (XSD 1.1 Part 1, sections 3.17.6.2 and 4.2.6)"))))))
                }
            }
        }
        Err(v3) => {
            Ok(Err::<Adt22, Bytes>(v3))
        }
    }
}

pub fn f482(v0: Adt25, v1: Bytes) -> Bytes {
    if f475(v0, Str::lit("nillable"), Str::lit("true")) {
        f535(Str::lit("Option"), v1)
    } else {
        v1
    }
}

pub fn f483(p2996: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: bool, p2997: List<Adt25>, p2998: Adt32) -> R<Result<Adt32, Bytes>> {
    let m2995 = (p2996, (p2997, p2998));
    {
        let mut m2995 = m2995;
        loop {
            let step = {
                let (v0, (v5, v6)) = m2995;
                let m2992 = v0;
                if m2992 == 0 {
                    Err::<(u64, (List<Adt25>, Adt32)), R<Result<Adt32, Bytes>>>(Ok(Err::<Adt32, Bytes>(f418(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2.clone()))))
                } else {
                    let v7 = m2992 - 1;
                    let m2993 = v5;
                    match m2993.uncons() {
                        None => {
                            Err::<(u64, (List<Adt25>, Adt32)), R<Result<Adt32, Bytes>>>(Ok(Ok::<Adt32, Bytes>(v6)))
                        }
                        Some((v8, v9)) => {
                            let m2994 = f536(v7, v1.clone(), v2.clone(), v3.clone(), v4, v8, v6)?;
                            match m2994 {
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
                Ok(next) => { m2995 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f484(v0: List<Adt33>) -> List<Bytes> {
    let m2999 = v0;
    match m2999.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt33::C0(h3000, _, _) => {
                    h3000
                }
            }, f484(v2))
        }
    }
}

pub fn f485(v0: List<Adt33>) -> List<Bytes> {
    let m3001 = v0;
    match m3001.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt33::C0(_, h3002, _) => {
                    h3002
                }
            }, f485(v2))
        }
    }
}

pub fn f486(v0: List<Adt33>) -> List<Bytes> {
    let m3003 = v0;
    match m3003.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f340(match v1 {
                Adt33::C0(_, _, h3004) => {
                    h3004
                }
            }, f486(v2))
        }
    }
}

pub fn f487(v0: Bytes, v1: List<Adt33>) -> R<Result<Adt26, Bytes>> {
    let m3005 = f532(f484(v1.clone()))?;
    match m3005 {
        Ok(v3) => {
            let v4: List<Bytes> = f485(v1.clone());
            let v7: Bytes = {
                let m3006 = v1.clone();
                match m3006.uncons() {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some((_, _)) => {
                        f163(f8(Str::lit("\u{a}")), f75(f537(v3, v4.clone()), f8(Str::lit(",\u{a}"))), f8(Str::lit("\u{a}  ")))
                    }
                }
            };
            Ok(Ok::<Adt26, Bytes>(Adt26::C0(v0.clone(), f486(v1), f163(f8(Str::lit("structure{\u{a}  name: ")), v0, f163(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), v7, f8(Str::lit("]\u{a}}\u{a}")))), List::cons(f163(f8(Str::lit("    {name: mk, fields: [")), f75(v4, f8(Str::lit(", "))), f8(Str::lit("]}"))), List::<Bytes>::nil()))))
        }
        Err(v2) => {
            Ok(Err::<Adt26, Bytes>(f418(v2, v0)))
        }
    }
}

pub fn f488(v0: Adt23, v1: Str, v2: (Bytes, Bytes)) -> Option<Adt24> {
    f538(match v0 {
        Adt23::C0(h3007, _) => {
            h3007
        }
    }, f8(v1), {
        let (h3008, _) = v2;
        h3008
    }, {
        let (_, h3009) = v2;
        h3009
    })
}

pub fn f489(v0: Adt32, v1: Bytes, v2: Adt31, v3: bool) -> Adt32 {
    f539(f490(v0, Adt33::C0(v1, if v3 {
        let m3012 = match v2.clone() {
            Adt31::C0(h3011, _) => {
                h3011
            }
        };
        match m3012 {
            Adt22::C0(h3010, _) => {
                h3010
            }
        }
    } else {
        f535(Str::lit("Option"), {
            let m3015 = match v2.clone() {
                Adt31::C0(h3014, _) => {
                    h3014
                }
            };
            match m3015 {
                Adt22::C0(h3013, _) => {
                    h3013
                }
            }
        })
    }, {
        let m3018 = match v2.clone() {
            Adt31::C0(h3017, _) => {
                h3017
            }
        };
        match m3018 {
            Adt22::C0(_, h3016) => {
                h3016
            }
        }
    })), match v2 {
        Adt31::C0(_, h3019) => {
            h3019
        }
    })
}

pub fn f490(v0: Adt32, v1: Adt33) -> Adt32 {
    Adt32::C0(f540(match v0.clone() {
        Adt32::C0(h3020, _, _, _, _) => {
            h3020
        }
    }, List::cons(v1, List::<Adt33>::nil())), match v0.clone() {
        Adt32::C0(_, h3021, _, _, _) => {
            h3021
        }
    }, match v0.clone() {
        Adt32::C0(_, _, h3022, _, _) => {
            h3022
        }
    }, match v0.clone() {
        Adt32::C0(_, _, _, h3023, _) => {
            h3023
        }
    }, match v0 {
        Adt32::C0(_, _, _, _, h3024) => {
            h3024
        }
    })
}

pub fn f491() -> Bytes {
    f8(Str::lit("(List ((Bytes * Bytes) * Bytes))"))
}

pub fn f492(v0: Bytes, v1: List<Bytes>, v2: Bytes) -> Bytes {
    f163(f8(Str::lit("inductive{\u{a}  name: ")), v0, f163(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), f75(v1, f8(Str::lit(",\u{a}"))), f163(f8(Str::lit("\u{a}  ],\u{a}  mutual: ")), v2, f8(Str::lit("\u{a}}\u{a}")))))
}

pub fn f493(v0: Bytes, v1: u64, v2: u64) -> R<Adt34> {
    let v3: (u64, u64) = {
        let (h3025, _) = f399(Fn7::F541(v2, v0.clone()), {
            let a3027 = {
                let a3026 = v0.clone();
                length_bytes(a3026)
            };
            let a3028 = 1u64;
            nat_add(a3027, a3028)?
        }, (v1, 0u64))?;
        h3025
    };
    let v4: u64 = f494(v0.clone(), {
        let (h3029, _) = v3;
        h3029
    }, v2)?;
    let a3030 = f173(v0, v4)?;
    let a3031 = 9u64;
    if if nat_eq(a3030, a3031) {
        let a3035 = {
            let a3033 = v4;
            let a3034 = {
                let (h3032, _) = v3;
                h3032
            };
            nat_sub(a3033, a3034)
        };
        let a3036 = v2;
        nat_lt(a3035, a3036)
    } else {
        false
    } {
        Ok(Adt34::C1(f83(Str::lit("a tab cannot indent a block scalar line (YAML 1.2.2 [63], [70])"), v4)?))
    } else {
        Ok(Adt34::C0({
            let (_, h3037) = v3;
            h3037
        }, {
            let (h3038, _) = v3;
            h3038
        }))
    }
}

pub fn f494(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h3039, _) = f130(Fn4::F542(v2, v0.clone(), v1), {
        let a3041 = {
            let a3040 = v0;
            length_bytes(a3040)
        };
        let a3042 = 1u64;
        nat_add(a3041, a3042)?
    }, v1)?;
    Ok(h3039)
}

pub fn f495(v0: Bytes, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = f494(v0.clone(), v1, v2)?;
    let a3045 = {
        let a3043 = v3;
        let a3044 = v1;
        nat_sub(a3043, a3044)
    };
    let a3046 = v2;
    if nat_eq(a3045, a3046) {
        Ok(f178({
            let a3047 = f173(v0, v3)?;
            let a3048 = 256u64;
            nat_eq(a3047, a3048)
        }))
    } else {
        Ok(false)
    }
}

pub fn f496(v0: Bytes, p3058: u64, v2: u64, v3: bool, p3059: u64, p3060: Bytes, p3061: u64) -> R<Adt35> {
    let m3057 = (p3058, (p3059, (p3060, p3061)));
    {
        let mut m3057 = m3057;
        loop {
            let step = {
                let (v1, (v4, (v5, v6))) = m3057;
                let m3049 = v4;
                if m3049 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<Adt35>>(Ok(Adt35::C1(f83(Str::lit("a block scalar has more lines than the input"), v1)?)))
                } else {
                    let v7 = m3049 - 1;
                    let v8: u64 = f494(v0.clone(), v1, v2)?;
                    let v9: bool = f178(f172(f173(v0.clone(), v8)?));
                    let v10: u64 = f358(v0.clone(), v8)?;
                    let v11: Bytes = f7(List::cons(v5, List::cons(f434(v6), List::cons(f356(v0.clone(), v8, v10), List::<Bytes>::nil()))));
                    let m3050 = f493(v0.clone(), f255(v0.clone(), v10)?, v2)?;
                    match m3050 {
                        Adt34::C0(v13, v14) => {
                            let v15: u64 = v14;
                            let v16: u64 = f494(v0.clone(), v15, v2)?;
                            if f495(v0.clone(), v15, v2)? {
                                Ok::<(u64, (u64, (Bytes, u64))), R<Adt35>>((v15, (v7, ({
                                    let a3053 = v11;
                                    let a3054 = if if if v3 {
                                        v9
                                    } else {
                                        false
                                    } {
                                        f178(f172(f173(v0.clone(), v16)?))
                                    } else {
                                        false
                                    } {
                                        let a3051 = v13;
                                        let a3052 = 0u64;
                                        if nat_eq(a3051, a3052) {
                                            Bytes::lit(&[0x20])
                                        } else {
                                            Bytes::lit(&[])
                                        }
                                    } else {
                                        Bytes::lit(&[0x0a])
                                    };
                                    append_bytes(a3053, a3054)
                                }, v13))))
                            } else {
                                Err::<(u64, (u64, (Bytes, u64))), R<Adt35>>(Ok(Adt35::C0(v11, v13, {
                                    let a3055 = f173(v0.clone(), v16)?;
                                    let a3056 = 256u64;
                                    if nat_eq(a3055, a3056) {
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
                Ok(next) => { m3057 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f497(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f248(v0.clone(), {
        let (h3062, _) = v1;
        h3062
    })?;
    let a3063 = f173(v0, v2)?;
    let a3064 = 10u64;
    if nat_eq(a3063, a3064) {
        Ok(Some(({
            let a3065 = v2;
            let a3066 = 1u64;
            nat_add(a3065, a3066)?
        }, f436({
            let (_, h3067) = v1;
            h3067
        }, {
            let a3069 = v2;
            let a3070 = {
                let (h3068, _) = v1;
                h3068
            };
            nat_sub(a3069, a3070)
        }))))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f498(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f543(v0, v1)
}

pub fn f499(v0: u64) -> R<Option<u64>> {
    let v1: u64 = f507(f8(Str::lit("abtnvfr")), v0)?;
    let a3071 = v1;
    let a3072 = 7u64;
    if nat_lt(a3071, a3072) {
        Ok(Some({
            let a3073 = v1;
            let a3074 = 7u64;
            nat_add(a3073, a3074)?
        }))
    } else {
        let a3075 = v0;
        let a3076 = 48u64;
        if nat_eq(a3075, a3076) {
            Ok(Some(0u64))
        } else {
            let a3077 = v0;
            let a3078 = 9u64;
            if nat_eq(a3077, a3078) {
                Ok(Some(9u64))
            } else {
                let a3079 = v0;
                let a3080 = 101u64;
                if nat_eq(a3079, a3080) {
                    Ok(Some(27u64))
                } else {
                    let a3081 = v0;
                    let a3082 = 78u64;
                    if nat_eq(a3081, a3082) {
                        Ok(Some(133u64))
                    } else {
                        let a3083 = v0;
                        let a3084 = 95u64;
                        if nat_eq(a3083, a3084) {
                            Ok(Some(160u64))
                        } else {
                            let a3085 = v0;
                            let a3086 = 76u64;
                            if nat_eq(a3085, a3086) {
                                Ok(Some(8232u64))
                            } else {
                                let a3087 = v0;
                                let a3088 = 80u64;
                                if nat_eq(a3087, a3088) {
                                    Ok(Some(8233u64))
                                } else {
                                    let a3089 = f507(f8(Str::lit(" \"/\\")), v0)?;
                                    let a3090 = 4u64;
                                    if nat_lt(a3089, a3090) {
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

pub fn f500(v0: u64) -> R<Bytes> {
    f83(Str::lit("an implicit key must be on one line (YAML 1.2.2 [154])"), v0)
}

pub fn f501(v0: Bytes, p3105: u64, v2: u64, p3106: u64, p3107: u64) -> R<Adt34> {
    let m3104 = (p3105, (p3106, p3107));
    {
        let mut m3104 = m3104;
        loop {
            let step = {
                let (v1, (v3, v4)) = m3104;
                let m3091 = v3;
                if m3091 == 0 {
                    Err::<(u64, (u64, u64)), R<Adt34>>(Ok(Adt34::C1(f83(Str::lit("the input ends inside a scalar"), v1)?)))
                } else {
                    let v5 = m3091 - 1;
                    let v6: u64 = {
                        let a3092 = v1;
                        let a3093 = 1u64;
                        nat_add(a3092, a3093)?
                    };
                    let v7: u64 = f248(v0.clone(), v6)?;
                    let v8: u64 = f249(v0.clone(), v7)?;
                    let v9: u64 = f173(v0.clone(), v8)?;
                    if f103(v0.clone(), v6)? {
                        Err::<(u64, (u64, u64)), R<Adt34>>(Ok(Adt34::C1(f83(Str::lit("a document marker cannot appear inside a scalar (YAML 1.2.2 [206])"), v6)?)))
                    } else {
                        let a3094 = v9;
                        let a3095 = 10u64;
                        if nat_eq(a3094, a3095) {
                            Ok::<(u64, (u64, u64)), R<Adt34>>((v8, (v5, {
                                let a3096 = v4;
                                let a3097 = 1u64;
                                nat_add(a3096, a3097)?
                            })))
                        } else {
                            let a3098 = v9;
                            let a3099 = 256u64;
                            if nat_eq(a3098, a3099) {
                                Err::<(u64, (u64, u64)), R<Adt34>>(Ok(Adt34::C1(f83(Str::lit("the input ends inside a quoted scalar (YAML 1.2.2 [109], [120])"), v8)?)))
                            } else {
                                let a3102 = {
                                    let a3100 = v7;
                                    let a3101 = v6;
                                    nat_sub(a3100, a3101)
                                };
                                let a3103 = v2;
                                if nat_lt(a3102, a3103) {
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
                Ok(next) => { m3104 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f502(v0: Bytes, p3117: u64, p3118: u64, p3119: u64) -> R<Option<u64>> {
    let m3116 = (p3117, (p3118, p3119));
    {
        let mut m3116 = m3116;
        loop {
            let step = {
                let (v1, (v2, v3)) = m3116;
                let m3108 = v2;
                if m3108 == 0 {
                    Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3108 - 1;
                    let m3109 = f544(f173(v0.clone(), v1)?);
                    match m3109 {
                        None => {
                            Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(None::<u64>))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, u64)), R<Option<u64>>>(({
                                let a3110 = v1;
                                let a3111 = 1u64;
                                nat_add(a3110, a3111)?
                            }, (v4, {
                                let a3114 = {
                                    let a3112 = v3;
                                    let a3113 = 16u64;
                                    nat_mul(a3112, a3113)?
                                };
                                let a3115 = v5;
                                nat_add(a3114, a3115)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3116 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f503(v0: Bytes, v1: u64) -> R<u64> {
    let (h3120, _) = f130(Fn4::F545(v0.clone()), {
        let a3122 = {
            let a3121 = v0;
            length_bytes(a3121)
        };
        let a3123 = 1u64;
        nat_add(a3122, a3123)?
    }, v1)?;
    Ok(h3120)
}

pub fn f504(v0: Bytes, v1: u64) -> R<u64> {
    let (h3124, _) = f130(Fn4::F546(v0.clone()), {
        let a3126 = {
            let a3125 = v0;
            length_bytes(a3125)
        };
        let a3127 = 1u64;
        nat_add(a3126, a3127)?
    }, v1)?;
    Ok(h3124)
}

pub fn f505(v0: Fn8, p3132: u64, p3133: (u64, bool)) -> R<((u64, bool), bool)> {
    let m3131 = (p3132, p3133);
    {
        let mut m3131 = m3131;
        loop {
            let step = {
                let (v1, v2) = m3131;
                let m3128 = v1;
                if m3128 == 0 {
                    Err::<(u64, (u64, bool)), R<((u64, bool), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3128 - 1;
                    let m3130 = {
                        let c3129: Fn8 = v0.clone();
                        c3129.apply(v2)?
                    };
                    match m3130 {
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
                Ok(next) => { m3131 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f506(v0: u64, v1: Bytes, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    f547(v1, v0, v2)
}

pub fn f507(v0: Bytes, v1: u64) -> R<u64> {
    let (h3134, _) = f130(Fn4::F548(v0.clone(), v1), {
        let a3136 = {
            let a3135 = v0;
            length_bytes(a3135)
        };
        let a3137 = 1u64;
        nat_add(a3136, a3137)?
    }, 0u64)?;
    Ok(h3134)
}

pub fn f508(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = {
        let (h3138, _) = v2;
        h3138
    };
    let v4: u64 = f173(v0.clone(), v3)?;
    if f174(v4) {
        Ok(None::<(u64, u64)>)
    } else {
        if f172(v4) {
            Ok(Some(({
                let a3139 = v3;
                let a3140 = 1u64;
                nat_add(a3139, a3140)?
            }, {
                let (_, h3141) = v2;
                h3141
            })))
        } else {
            let a3142 = v4;
            let a3143 = 58u64;
            if if nat_eq(a3142, a3143) {
                f178(f349(f173(v0.clone(), {
                    let a3144 = v3;
                    let a3145 = 1u64;
                    nat_add(a3144, a3145)?
                })?, v1))
            } else {
                false
            } {
                Ok(None::<(u64, u64)>)
            } else {
                let a3146 = v4;
                let a3147 = 35u64;
                if if nat_eq(a3146, a3147) {
                    f172(f173(v0, {
                        let a3148 = v3;
                        let a3149 = 1u64;
                        nat_sub(a3148, a3149)
                    })?)
                } else {
                    false
                } {
                    Ok(None::<(u64, u64)>)
                } else {
                    if if v1 {
                        f429(v4)
                    } else {
                        false
                    } {
                        Ok(None::<(u64, u64)>)
                    } else {
                        Ok(Some(({
                            let a3150 = v3;
                            let a3151 = 1u64;
                            nat_add(a3150, a3151)?
                        }, {
                            let a3152 = v3;
                            let a3153 = 1u64;
                            nat_add(a3152, a3153)?
                        })))
                    }
                }
            }
        }
    }
}

pub fn f509(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f549(v0, v1)
}

pub fn f510(v0: Bytes, v1: u64, v2: u64) -> R<(u64, u64)> {
    let (h3154, _) = f399(Fn7::F550(v1, v0.clone()), {
        let a3156 = {
            let a3155 = v0;
            length_bytes(a3155)
        };
        let a3157 = 1u64;
        nat_add(a3156, a3157)?
    }, (v2, 0u64))?;
    Ok(h3154)
}

pub fn f511(v0: bool, v1: u64) -> bool {
    if v0 {
        let a3158 = 0u64;
        let a3159 = v1;
        if nat_lt(a3158, a3159) {
            let a3160 = v1;
            let a3161 = 9223372036854775808u64;
            nat_le(a3160, a3161)
        } else {
            false
        }
    } else {
        let a3162 = v1;
        let a3163 = 9223372036854775807u64;
        nat_le(a3162, a3163)
    }
}

pub fn f512(v0: Bytes, v1: u64) -> R<u64> {
    let (h3164, _) = f130(Fn4::F551(v0.clone()), {
        let a3166 = {
            let a3165 = v0;
            length_bytes(a3165)
        };
        let a3167 = 1u64;
        nat_add(a3166, a3167)?
    }, v1)?;
    Ok(h3164)
}

pub fn f513(v0: Bytes) -> R<u64> {
    let (h3168, _) = f130(Fn4::F552(v0.clone()), {
        let a3170 = {
            let a3169 = v0;
            length_bytes(a3169)
        };
        let a3171 = 1u64;
        nat_add(a3170, a3171)?
    }, 0u64)?;
    Ok(h3168)
}

pub fn f514(v0: Bytes) -> R<Option<u64>> {
    if f553(v0.clone())? {
        Ok(Some({
            let (_, h3172) = {
                let (h3173, _) = f399(Fn7::F554(v0.clone()), {
                    let a3174 = v0;
                    length_bytes(a3174)
                }, (0u64, 0u64))?;
                h3173
            };
            h3172
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f515(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f32(v1.clone(), 0u64, {
        let a3176 = {
            let a3175 = v1.clone();
            length_bytes(a3175)
        };
        let a3177 = f555(v1)?;
        nat_sub(a3176, a3177)
    });
    let v5: Bytes = {
        let a3178 = v0;
        let a3179 = v4.clone();
        append_bytes(a3178, a3179)
    };
    let v6: u64 = f513(v5.clone())?;
    let v7: Bytes = f32(v5.clone(), v6, {
        let a3181 = {
            let a3180 = v5;
            length_bytes(a3180)
        };
        let a3182 = v6;
        nat_sub(a3181, a3182)
    });
    let v8: u64 = {
        let a3183 = v4;
        length_bytes(a3183)
    };
    let v9: u64 = {
        let a3184 = v7.clone();
        length_bytes(a3184)
    };
    let a3185 = v9;
    let a3186 = 0u64;
    if nat_eq(a3185, a3186) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f323(v2, {
            let a3187 = v3;
            let a3188 = v8;
            nat_lt(a3187, a3188)
        }) {
            let v10: u64 = if v2 {
                let a3189 = v3;
                let a3190 = v8;
                nat_add(a3189, a3190)?
            } else {
                let a3191 = v8;
                let a3192 = v3;
                nat_sub(a3191, a3192)
            };
            let v11: u64 = f555(v7.clone())?;
            let a3193 = v10;
            let a3194 = v11;
            if nat_le(a3193, a3194) {
                Ok(Some((f32(v7, 0u64, {
                    let a3195 = v9;
                    let a3196 = v10;
                    nat_sub(a3195, a3196)
                }), 0u64)))
            } else {
                let a3199 = {
                    let a3197 = v10;
                    let a3198 = v11;
                    nat_sub(a3197, a3198)
                };
                let a3200 = 19u64;
                if nat_le(a3199, a3200) {
                    Ok(Some((f32(v7, 0u64, {
                        let a3201 = v9;
                        let a3202 = v11;
                        nat_sub(a3201, a3202)
                    }), {
                        let a3203 = v10;
                        let a3204 = v11;
                        nat_sub(a3203, a3204)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a3205 = v3;
                let a3206 = v8;
                nat_sub(a3205, a3206)
            };
            let a3209 = {
                let a3207 = v9;
                let a3208 = v12;
                nat_add(a3207, a3208)?
            };
            let a3210 = 19u64;
            if nat_le(a3209, a3210) {
                Ok(Some(({
                    let a3211 = v7;
                    let a3212 = f360(v12);
                    append_bytes(a3211, a3212)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f516(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f174(f173(v0, v1)?) {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3213 = v1;
            let a3214 = 1u64;
            nat_add(a3213, a3214)?
        }))
    }
}

pub fn f517(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m3218 = {
        let a3216 = v0.clone();
        let a3217 = {
            let (h3215, _) = v1.clone();
            h3215
        };
        index_bytes(a3216, a3217)
    };
    match m3218 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a3220 = {
                    let (h3219, _) = v1.clone();
                    h3219
                };
                let a3221 = 1u64;
                nat_add(a3220, a3221)?
            }, {
                let a3224 = {
                    let (_, h3222) = v1;
                    h3222
                };
                let a3225 = f556(v0, {
                    let (h3223, _) = v1;
                    h3223
                }, v2)?;
                append_bytes(a3224, a3225)
            })))
        }
    }
}

pub fn f518(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f557(Fn11::F558, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f519(v0: Fn9, p3229: List<List<(Bytes, Bytes)>>, p3230: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m3228 = (p3229, p3230);
    {
        let mut m3228 = m3228;
        loop {
            let step = {
                let (v1, v2) = m3228;
                let m3226 = v2;
                match m3226.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c3227: Fn9 = v0.clone();
                            c3227.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3228 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f520(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f521(v0: Fn10, p3235: u64, p3236: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m3234 = (p3235, p3236);
    {
        let mut m3234 = m3234;
        loop {
            let step = {
                let (v1, v2) = m3234;
                let m3231 = v1;
                if m3231 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3231 - 1;
                    let m3233 = {
                        let c3232: Fn10 = v0.clone();
                        c3232.apply(v2.clone())?
                    };
                    match m3233 {
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
                Ok(next) => { m3234 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f522(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f559(v0)
}

pub fn f523(v0: u64, v1: u64) -> bool {
    let a3237 = v0;
    let a3238 = 0u64;
    if nat_eq(a3237, a3238) {
        f370(v1)
    } else {
        let a3239 = v0;
        let a3240 = 1u64;
        if nat_eq(a3239, a3240) {
            f369(v1)
        } else {
            let a3241 = v0;
            let a3242 = 2u64;
            if nat_eq(a3241, a3242) {
                f560(v1)
            } else {
                f561(v1)
            }
        }
    }
}

pub fn f524(v0: Bytes, v1: u64) -> R<u64> {
    let a3244 = v1;
    let a3245 = {
        let a3243 = v0.clone();
        length_bytes(a3243)
    };
    if nat_lt(a3244, a3245) {
        let a3246 = f87(v0, v1)?;
        let a3247 = 1u64;
        nat_add(a3246, a3247)
    } else {
        Ok(0u64)
    }
}

pub fn f525(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f323(f321(f323(f322(v2), f324(v2)), f321({
        let a3248 = v2;
        let a3249 = 117u64;
        !nat_eq(a3248, a3249)
    }, {
        let a3250 = v2;
        let a3251 = 85u64;
        !nat_eq(a3250, a3251)
    })), f321(f325(v2), {
        let a3252 = 0u64;
        let a3253 = v1;
        nat_lt(a3252, a3253)
    })) {
        f32(v0, v1, 1u64)
    } else {
        let a3254 = v2;
        let a3255 = 65536u64;
        if nat_lt(a3254, a3255) {
            let a3256 = f8(Str::lit("u"));
            let a3257 = f527(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a3256, a3257)
        } else {
            let a3258 = f8(Str::lit("U"));
            let a3259 = f527(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a3258, a3259)
        }
    }
}

pub fn f526(v0: u64) -> Bytes {
    let a3260 = f8(Str::lit("u"));
    let a3261 = f527(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a3260, a3261)
}

pub fn f527(p3278: u64, p3279: u64, p3280: u64, p3281: Bytes) -> Bytes {
    let m3277 = (p3278, (p3279, (p3280, p3281)));
    {
        let mut m3277 = m3277;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m3277;
                let m3262 = v0;
                if m3262 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m3262 - 1;
                    if f321({
                        let a3263 = v1;
                        let a3264 = 0u64;
                        nat_eq(a3263, a3264)
                    }, {
                        let a3265 = v2;
                        let a3266 = 0u64;
                        nat_eq(a3265, a3266)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a3267 = v1;
                            let a3268 = 16u64;
                            let a3269 = 0u64;
                            nat_quot(a3267, a3268, a3269)
                        }, ({
                            let a3270 = v2;
                            let a3271 = 1u64;
                            nat_sub(a3270, a3271)
                        }, {
                            let a3275 = f562({
                                let a3272 = v1;
                                let a3273 = 16u64;
                                let a3274 = 0u64;
                                nat_rem(a3272, a3273, a3274)
                            });
                            let a3276 = v3;
                            append_bytes(a3275, a3276)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m3277 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f528(v0: u64, v1: bool) -> u64 {
    let a3282 = v0;
    let a3283 = 48u64;
    if nat_lt(a3282, a3283) {
        16u64
    } else {
        let a3284 = v0;
        let a3285 = 57u64;
        if nat_le(a3284, a3285) {
            let a3286 = v0;
            let a3287 = 48u64;
            nat_sub(a3286, a3287)
        } else {
            if v1 {
                let a3288 = v0;
                let a3289 = 65u64;
                if nat_lt(a3288, a3289) {
                    16u64
                } else {
                    let a3290 = v0;
                    let a3291 = 70u64;
                    if nat_le(a3290, a3291) {
                        let a3292 = v0;
                        let a3293 = 55u64;
                        nat_sub(a3292, a3293)
                    } else {
                        let a3294 = v0;
                        let a3295 = 97u64;
                        if nat_lt(a3294, a3295) {
                            16u64
                        } else {
                            let a3296 = v0;
                            let a3297 = 102u64;
                            if nat_le(a3296, a3297) {
                                let a3298 = v0;
                                let a3299 = 87u64;
                                nat_sub(a3298, a3299)
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

pub fn f529() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f530() -> Bytes {
    f8(Str::lit("\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{0}]\u{0}\u{0}]\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f531(v0: Adt24) -> R<Result<Option<(Adt25, (Bytes, Bytes))>, Bytes>> {
    let m3301 = f416(match v0 {
        Adt24::C0(_, _, _, h3300) => {
            h3300
        }
    })?;
    match m3301 {
        Ok(v2) => {
            let m3302 = f417(v2, List::cons(f8(Str::lit("simpleContent")), List::cons(f8(Str::lit("complexContent")), List::<Bytes>::nil())));
            match m3302 {
                None => {
                    Ok(Ok::<Option<(Adt25, (Bytes, Bytes))>, Bytes>(None::<(Adt25, (Bytes, Bytes))>))
                }
                Some(v3) => {
                    let m3303 = f416(v3)?;
                    match m3303 {
                        Ok(v5) => {
                            let m3304 = f417(v5, List::cons(f8(Str::lit("extension")), List::cons(f8(Str::lit("restriction")), List::<Bytes>::nil())));
                            match m3304 {
                                None => {
                                    Ok(Ok::<Option<(Adt25, (Bytes, Bytes))>, Bytes>(None::<(Adt25, (Bytes, Bytes))>))
                                }
                                Some(v6) => {
                                    let m3305 = f241(v6.clone(), Str::lit("base"));
                                    match m3305 {
                                        None => {
                                            Ok(Ok::<Option<(Adt25, (Bytes, Bytes))>, Bytes>(None::<(Adt25, (Bytes, Bytes))>))
                                        }
                                        Some(v7) => {
                                            let m3307 = f91(match v6.clone() {
                                                Adt25::C0(_, _, _, _, h3306) => {
                                                    h3306
                                                }
                                            }, v7)?;
                                            match m3307 {
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

pub fn f532(v0: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f563(f190(v0.clone())?, v0, List::<Bytes>::nil())
}

pub fn f533(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m3308 = v0;
    match m3308.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m3309 = v1;
            match m3309.uncons() {
                None => {
                    List::<Bytes>::nil()
                }
                Some((v4, v5)) => {
                    List::cons(f163(f8(Str::lit("    {name: ")), v2, f163(f8(Str::lit(", fields: [")), v4, f8(Str::lit("]}")))), f533(v3, v5))
                }
            }
        }
    }
}

pub fn f534(v0: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: Adt25, v5: Adt32) -> R<Result<Adt32, Bytes>> {
    let m3310 = v0;
    if m3310 == 0 {
        Ok(Err::<Adt32, Bytes>(f418(f8(Str::lit("the type derivations nest deeper than the schema documents have bytes, so the types derive from each other in a cycle (XSD 1.1 Part 1, section 3.4.6.1)")), v2)))
    } else {
        let v6 = m3310 - 1;
        if f475(v4.clone(), Str::lit("mixed"), Str::lit("true")) {
            Ok(Err::<Adt32, Bytes>(f418(f8(Str::lit("a mixed content type interleaves character data with its elements, which a LexLean field list does not keep in order, so it is not translated (XSD 1.1 Part 1, section 3.4.2.3)")), v2)))
        } else {
            let m3311 = f416(v4)?;
            match m3311 {
                Ok(v8) => {
                    let m3312 = f417(v8.clone(), List::cons(f8(Str::lit("openContent")), List::cons(f8(Str::lit("assert")), List::<Bytes>::nil())));
                    match m3312 {
                        None => {
                            let m3314 = f417(v8.clone(), List::cons(f8(Str::lit("simpleContent")), List::cons(f8(Str::lit("complexContent")), List::<Bytes>::nil())));
                            match m3314 {
                                None => {
                                    let m3315 = f483(v6, v1.clone(), v2.clone(), v3.clone(), true, v8.clone(), v5)?;
                                    match m3315 {
                                        Ok(v11) => {
                                            f420(v6, v1, v2, v3, v8, v11)
                                        }
                                        Err(v10) => {
                                            Ok(Err::<Adt32, Bytes>(v10))
                                        }
                                    }
                                }
                                Some(v12) => {
                                    if f475(v12.clone(), Str::lit("mixed"), Str::lit("true")) {
                                        Ok(Err::<Adt32, Bytes>(f418(f8(Str::lit("a mixed content type interleaves character data with its elements, which a LexLean field list does not keep in order, so it is not translated (XSD 1.1 Part 1, section 3.4.2.3)")), v2)))
                                    } else {
                                        let m3316 = f416(v12.clone())?;
                                        match m3316 {
                                            Ok(v14) => {
                                                let m3317 = f417(v14, List::cons(f8(Str::lit("extension")), List::cons(f8(Str::lit("restriction")), List::<Bytes>::nil())));
                                                match m3317 {
                                                    None => {
                                                        Ok(Err::<Adt32, Bytes>(f418(f8(Str::lit("simpleContent and complexContent must have an extension or a restriction child (XSD 1.1 Part 1, sections 3.4.2.2 and 3.4.2.3)")), v2)))
                                                    }
                                                    Some(v15) => {
                                                        let m3318 = f241(v15.clone(), Str::lit("base"));
                                                        match m3318 {
                                                            None => {
                                                                Ok(Err::<Adt32, Bytes>(f418(f8(Str::lit("an extension or restriction must name its base type (XSD 1.1 Part 1, section 3.4.3)")), v2)))
                                                            }
                                                            Some(v16) => {
                                                                let m3320 = f91(match v15.clone() {
                                                                    Adt25::C0(_, _, _, _, h3319) => {
                                                                        h3319
                                                                    }
                                                                }, v16.clone())?;
                                                                match m3320 {
                                                                    Ok(v18) => {
                                                                        let m3321 = f416(v15.clone())?;
                                                                        match m3321 {
                                                                            Ok(v20) => {
                                                                                let v21: bool = if f35({
                                                                                    let (h3322, _) = v18.clone();
                                                                                    h3322
                                                                                }, f151()) {
                                                                                    f35({
                                                                                        let (_, h3323) = v18.clone();
                                                                                        h3323
                                                                                    }, f8(Str::lit("anyType")))
                                                                                } else {
                                                                                    false
                                                                                };
                                                                                if if f240(v15.clone(), Str::lit("restriction")) {
                                                                                    f239(v21)
                                                                                } else {
                                                                                    false
                                                                                } {
                                                                                    Ok(Err::<Adt32, Bytes>(f418(f163(f8(Str::lit("a restriction of the complex type ")), v16, f8(Str::lit(" is not translated, because the attribute uses it inherits and the content it narrows are not read yet (XSD 1.1 Part 1, sections 3.4.2.2 and 3.4.2.3)"))), v2)))
                                                                                } else {
                                                                                    let m3338 = if if v21 {
                                                                                        true
                                                                                    } else {
                                                                                        f35(match v12 {
                                                                                            Adt25::C0(_, h3324, _, _, _) => {
                                                                                                h3324
                                                                                            }
                                                                                        }, f8(Str::lit("complexContent")))
                                                                                    } {
                                                                                        if v21 {
                                                                                            Ok::<Adt32, Bytes>(v5)
                                                                                        } else {
                                                                                            let m3325 = f488(v1.clone(), Str::lit("complexType"), v18.clone());
                                                                                            match m3325 {
                                                                                                None => {
                                                                                                    Err::<Adt32, Bytes>(f418(f163(f8(Str::lit("the base type ")), v16, f8(Str::lit(" of a complex content extension is not a complex type of this schema set (XSD 1.1 Part 1, section 3.4.2.3)"))), v2.clone()))
                                                                                                }
                                                                                                Some(v22) => {
                                                                                                    let m3329 = f534(v6, v1.clone(), f154({
                                                                                                        let (_, h3326) = v18;
                                                                                                        h3326
                                                                                                    })?, match v22.clone() {
                                                                                                        Adt24::C0(_, h3327, _, _) => {
                                                                                                            h3327
                                                                                                        }
                                                                                                    }, match v22 {
                                                                                                        Adt24::C0(_, _, _, h3328) => {
                                                                                                            h3328
                                                                                                        }
                                                                                                    }, v5)?;
                                                                                                    match m3329 {
                                                                                                        Ok(v24) => {
                                                                                                            Ok::<Adt32, Bytes>(f564(v24))
                                                                                                        }
                                                                                                        Err(v23) => {
                                                                                                            Err::<Adt32, Bytes>(v23)
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        let m3330 = f488(v1.clone(), Str::lit("complexType"), v18.clone());
                                                                                        match m3330 {
                                                                                            None => {
                                                                                                let m3335 = f481(v1.clone(), v15, v16)?;
                                                                                                match m3335 {
                                                                                                    Ok(v29) => {
                                                                                                        Ok::<Adt32, Bytes>(f490(v5, Adt33::C0(f8(Str::lit("value")), match v29.clone() {
                                                                                                            Adt22::C0(h3336, _) => {
                                                                                                                h3336
                                                                                                            }
                                                                                                        }, match v29 {
                                                                                                            Adt22::C0(_, h3337) => {
                                                                                                                h3337
                                                                                                            }
                                                                                                        })))
                                                                                                    }
                                                                                                    Err(v28) => {
                                                                                                        Err::<Adt32, Bytes>(f418(v28, v2.clone()))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            Some(v25) => {
                                                                                                let m3334 = f534(v6, v1.clone(), f154({
                                                                                                    let (_, h3331) = v18;
                                                                                                    h3331
                                                                                                })?, match v25.clone() {
                                                                                                    Adt24::C0(_, h3332, _, _) => {
                                                                                                        h3332
                                                                                                    }
                                                                                                }, match v25 {
                                                                                                    Adt24::C0(_, _, _, h3333) => {
                                                                                                        h3333
                                                                                                    }
                                                                                                }, v5)?;
                                                                                                match m3334 {
                                                                                                    Ok(v27) => {
                                                                                                        Ok::<Adt32, Bytes>(f564(v27))
                                                                                                    }
                                                                                                    Err(v26) => {
                                                                                                        Err::<Adt32, Bytes>(v26)
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    };
                                                                                    match m3338 {
                                                                                        Ok(v31) => {
                                                                                            let m3339 = f483(v6, v1.clone(), v2.clone(), v3.clone(), true, v20.clone(), v31)?;
                                                                                            match m3339 {
                                                                                                Ok(v33) => {
                                                                                                    f420(v6, v1, v2, v3, v20, v33)
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
                                                                        Ok(Err::<Adt32, Bytes>(f418(v17, v2)))
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
                            Ok(Err::<Adt32, Bytes>(f418(f163(f8(Str::lit("xs:")), match v9 {
                                Adt25::C0(_, h3313, _, _, _) => {
                                    h3313
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

pub fn f535(v0: Str, v1: Bytes) -> Bytes {
    f163(f8(Str::lit("(")), f8(v0), f163(f8(Str::lit(" ")), v1, f8(Str::lit(")"))))
}

pub fn f536(v0: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: bool, v5: Adt25, v6: Adt32) -> R<Result<Adt32, Bytes>> {
    let m3340 = v0;
    if m3340 == 0 {
        Ok(Err::<Adt32, Bytes>(f418(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v7 = m3340 - 1;
        if f240(v5.clone(), Str::lit("element")) {
            let m3341 = f565(v7, v1, v2, v3, v5.clone())?;
            match m3341 {
                Ok(v9) => {
                    let m3347 = f566(v5, {
                        let m3346 = {
                            let m3345 = match v9.clone() {
                                Adt36::C0(_, h3344) => {
                                    h3344
                                }
                            };
                            match m3345 {
                                Adt31::C0(h3343, _) => {
                                    h3343
                                }
                            }
                        };
                        match m3346 {
                            Adt22::C0(h3342, _) => {
                                h3342
                            }
                        }
                    });
                    match m3347 {
                        None => {
                            Ok(Ok::<Adt32, Bytes>(v6))
                        }
                        Some(v10) => {
                            Ok(Ok::<Adt32, Bytes>(f539(f490(v6, Adt33::C0(match v9.clone() {
                                Adt36::C0(h3348, _) => {
                                    h3348
                                }
                            }, v10, {
                                let m3353 = {
                                    let m3352 = match v9.clone() {
                                        Adt36::C0(_, h3351) => {
                                            h3351
                                        }
                                    };
                                    match m3352 {
                                        Adt31::C0(h3350, _) => {
                                            h3350
                                        }
                                    }
                                };
                                match m3353 {
                                    Adt22::C0(_, h3349) => {
                                        h3349
                                    }
                                }
                            })), {
                                let m3356 = match v9 {
                                    Adt36::C0(_, h3355) => {
                                        h3355
                                    }
                                };
                                match m3356 {
                                    Adt31::C0(_, h3354) => {
                                        h3354
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
            if f240(v5.clone(), Str::lit("any")) {
                let m3357 = f566(v5, f333());
                match m3357 {
                    None => {
                        Ok(Ok::<Adt32, Bytes>(v6))
                    }
                    Some(v11) => {
                        Ok(Ok::<Adt32, Bytes>(f490(v6, Adt33::C0(f8(Str::lit("any")), v11, List::cons(f333(), List::<Bytes>::nil())))))
                    }
                }
            } else {
                if f240(v5.clone(), Str::lit("group")) {
                    let m3358 = f241(v5.clone(), Str::lit("ref"));
                    match m3358 {
                        None => {
                            Ok(Err::<Adt32, Bytes>(f418(f8(Str::lit("a model group reference must have a ref (XSD 1.1 Part 1, section 3.7.2)")), v2)))
                        }
                        Some(v12) => {
                            let m3360 = f91(match v5.clone() {
                                Adt25::C0(_, _, _, _, h3359) => {
                                    h3359
                                }
                            }, v12.clone())?;
                            match m3360 {
                                Ok(v14) => {
                                    let m3361 = f488(v1, Str::lit("group"), v14.clone());
                                    match m3361 {
                                        None => {
                                            Ok(Err::<Adt32, Bytes>(f418(f163(f8(Str::lit("the model group ")), v12, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                        }
                                        Some(_) => {
                                            let v16: Bytes = f154({
                                                let (_, h3362) = v14.clone();
                                                h3362
                                            })?;
                                            let m3363 = f566(v5, v16.clone());
                                            match m3363 {
                                                None => {
                                                    Ok(Ok::<Adt32, Bytes>(v6))
                                                }
                                                Some(v17) => {
                                                    Ok(Ok::<Adt32, Bytes>(f490(v6, Adt33::C0({
                                                        let (_, h3364) = v14;
                                                        h3364
                                                    }, v17, List::cons(v16, List::<Bytes>::nil())))))
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(v13) => {
                                    Ok(Err::<Adt32, Bytes>(f418(v13, v2)))
                                }
                            }
                        }
                    }
                } else {
                    if if f240(v5.clone(), Str::lit("sequence")) {
                        true
                    } else {
                        if f240(v5.clone(), Str::lit("choice")) {
                            true
                        } else {
                            f240(v5.clone(), Str::lit("all"))
                        }
                    } {
                        if if v4 {
                            if f567(v5.clone()) {
                                f239(f240(v5.clone(), Str::lit("choice")))
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m3365 = f416(v5)?;
                            match m3365 {
                                Ok(v19) => {
                                    f483(v7, v1, v2, v3, true, v19, v6)
                                }
                                Err(v18) => {
                                    Ok(Err::<Adt32, Bytes>(v18))
                                }
                            }
                        } else {
                            let v20: Bytes = match v5.clone() {
                                Adt25::C0(_, h3366, _, _, _) => {
                                    h3366
                                }
                            };
                            let v21: u64 = f568(v6.clone(), v20.clone())?;
                            let v22: Bytes = {
                                let a3367 = v21;
                                let a3368 = 1u64;
                                if nat_eq(a3367, a3368) {
                                    Bytes::lit(&[])
                                } else {
                                    f52(v21)?
                                }
                            };
                            let v23: Bytes = f163(v2, f154(v20.clone())?, v22.clone());
                            let m3369 = f419(v7, v1, v23.clone(), v3, v5.clone())?;
                            match m3369 {
                                Ok(v25) => {
                                    let m3370 = f566(v5, v23.clone());
                                    match m3370 {
                                        None => {
                                            Ok(Ok::<Adt32, Bytes>(f569(v6, v20)?))
                                        }
                                        Some(v26) => {
                                            Ok(Ok::<Adt32, Bytes>(f539(f490(f569(v6, v20.clone())?, Adt33::C0({
                                                let a3371 = v20;
                                                let a3372 = v22;
                                                append_bytes(a3371, a3372)
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

pub fn f537(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m3373 = v0;
    match m3373.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m3374 = v1;
            match m3374.uncons() {
                None => {
                    List::<Bytes>::nil()
                }
                Some((v4, v5)) => {
                    List::cons(f163(f8(Str::lit("    {name: ")), v2, f163(f8(Str::lit(", type: ")), v4, f8(Str::lit("}")))), f537(v3, v5))
                }
            }
        }
    }
}

pub fn f538(p3380: List<Adt24>, v1: Bytes, v2: Bytes, v3: Bytes) -> Option<Adt24> {
    let m3379 = p3380;
    {
        let mut m3379 = m3379;
        loop {
            let step = {
                let v0 = m3379;
                let m3375 = v0;
                match m3375.uncons() {
                    None => {
                        Err::<List<Adt24>, Option<Adt24>>(None::<Adt24>)
                    }
                    Some((v4, v5)) => {
                        if if f35(match v4.clone() {
                            Adt24::C0(h3376, _, _, _) => {
                                h3376
                            }
                        }, v1.clone()) {
                            if f35(match v4.clone() {
                                Adt24::C0(_, h3377, _, _) => {
                                    h3377
                                }
                            }, v2.clone()) {
                                f35(match v4.clone() {
                                    Adt24::C0(_, _, h3378, _) => {
                                        h3378
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
                Ok(next) => { m3379 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f539(v0: Adt32, v1: List<Adt26>) -> Adt32 {
    Adt32::C0(match v0.clone() {
        Adt32::C0(h3381, _, _, _, _) => {
            h3381
        }
    }, f330(match v0.clone() {
        Adt32::C0(_, h3382, _, _, _) => {
            h3382
        }
    }, v1), match v0.clone() {
        Adt32::C0(_, _, h3383, _, _) => {
            h3383
        }
    }, match v0.clone() {
        Adt32::C0(_, _, _, h3384, _) => {
            h3384
        }
    }, match v0 {
        Adt32::C0(_, _, _, _, h3385) => {
            h3385
        }
    })
}

pub fn f540(v0: List<Adt33>, v1: List<Adt33>) -> List<Adt33> {
    let m3386 = v0;
    match m3386.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f540(v3, v1))
        }
    }
}

pub fn f541(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f570(v1, v0, v2)
}

pub fn f542(v0: u64, v1: Bytes, v2: u64, v3: u64) -> R<Option<u64>> {
    f571(v1, v2, v0, v3)
}

pub fn f543(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let a3389 = {
        let (h3387, _) = v1;
        h3387
    };
    let a3390 = {
        let a3388 = v0.clone();
        length_bytes(a3388)
    };
    if nat_lt(a3389, a3390) {
        Ok(Some((f255(v0, {
            let (h3391, _) = v1;
            h3391
        })?, {
            let a3393 = {
                let (_, h3392) = v1;
                h3392
            };
            let a3394 = 1u64;
            nat_add(a3393, a3394)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f544(v0: u64) -> Option<u64> {
    let a3395 = 48u64;
    let a3396 = v0;
    if if nat_le(a3395, a3396) {
        let a3397 = v0;
        let a3398 = 57u64;
        nat_le(a3397, a3398)
    } else {
        false
    } {
        Some({
            let a3399 = v0;
            let a3400 = 48u64;
            nat_sub(a3399, a3400)
        })
    } else {
        let a3401 = 97u64;
        let a3402 = v0;
        if if nat_le(a3401, a3402) {
            let a3403 = v0;
            let a3404 = 102u64;
            nat_le(a3403, a3404)
        } else {
            false
        } {
            Some({
                let a3405 = v0;
                let a3406 = 87u64;
                nat_sub(a3405, a3406)
            })
        } else {
            let a3407 = 65u64;
            let a3408 = v0;
            if if nat_le(a3407, a3408) {
                let a3409 = v0;
                let a3410 = 70u64;
                nat_le(a3409, a3410)
            } else {
                false
            } {
                Some({
                    let a3411 = v0;
                    let a3412 = 55u64;
                    nat_sub(a3411, a3412)
                })
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f545(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f572(v0, v1)
}

pub fn f546(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f573(v0, v1)
}

pub fn f547(v0: Bytes, v1: u64, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    let v3: u64 = {
        let (h3413, _) = v2;
        h3413
    };
    let v4: u64 = f173(v0.clone(), v3)?;
    let (_, h3414) = v2;
    if h3414 {
        Ok(None::<(u64, bool)>)
    } else {
        if f172(v4) {
            Ok(Some(({
                let a3415 = v3;
                let a3416 = 1u64;
                nat_add(a3415, a3416)?
            }, false)))
        } else {
            let a3417 = v4;
            let a3418 = 35u64;
            if if nat_eq(a3417, a3418) {
                let a3419 = v3;
                let a3420 = 0u64;
                if if nat_eq(a3419, a3420) {
                    true
                } else {
                    f172(f173(v0.clone(), {
                        let a3421 = v3;
                        let a3422 = 1u64;
                        nat_sub(a3421, a3422)
                    })?)
                } {
                    true
                } else {
                    let a3425 = f173(v0.clone(), {
                        let a3423 = v3;
                        let a3424 = 1u64;
                        nat_sub(a3423, a3424)
                    })?;
                    let a3426 = 10u64;
                    nat_eq(a3425, a3426)
                }
            } else {
                false
            } {
                Ok(Some((f358(v0, v3)?, false)))
            } else {
                let a3427 = v4;
                let a3428 = 10u64;
                if nat_eq(a3427, a3428) {
                    let v5: u64 = {
                        let a3429 = v3;
                        let a3430 = 1u64;
                        nat_add(a3429, a3430)?
                    };
                    let v6: u64 = f248(v0.clone(), v5)?;
                    let v7: u64 = f249(v0.clone(), v6)?;
                    if if f103(v0.clone(), v5)? {
                        true
                    } else {
                        let a3433 = {
                            let a3431 = v6;
                            let a3432 = v5;
                            nat_sub(a3431, a3432)
                        };
                        let a3434 = v1;
                        if if nat_lt(a3433, a3434) {
                            f178(f174(f173(v0.clone(), v7)?))
                        } else {
                            false
                        } {
                            f178({
                                let a3435 = f173(v0, v7)?;
                                let a3436 = 35u64;
                                nat_eq(a3435, a3436)
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

pub fn f548(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f574(v0, v1, v2)
}

pub fn f549(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f249(v0.clone(), {
        let (h3437, _) = v1;
        h3437
    })?;
    let a3438 = f173(v0, v2)?;
    let a3439 = 10u64;
    if nat_eq(a3438, a3439) {
        Ok(Some(({
            let a3440 = v2;
            let a3441 = 1u64;
            nat_add(a3440, a3441)?
        }, {
            let a3443 = {
                let (_, h3442) = v1;
                h3442
            };
            let a3444 = 1u64;
            nat_add(a3443, a3444)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f550(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f575(v1, v0, v2)
}

pub fn f551(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f576(v0, v1)
}

pub fn f552(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f577(v0, v1)
}

pub fn f553(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a3445 = v0.clone();
        length_bytes(a3445)
    };
    let a3446 = 0u64;
    let a3447 = v1;
    if nat_lt(a3446, a3447) {
        let a3448 = f578(v0.clone())?;
        let a3449 = v1;
        if nat_eq(a3448, a3449) {
            let a3450 = 1u64;
            let a3451 = v1;
            if nat_lt(a3450, a3451) {
                let m3454 = {
                    let a3452 = v0;
                    let a3453 = 0u64;
                    index_bytes(a3452, a3453)
                };
                match m3454 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a3455 = v2;
                        let a3456 = 48u8;
                        Ok(!equal(a3455, a3456))
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

pub fn f554(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f579(v0, v1)
}

pub fn f555(v0: Bytes) -> R<u64> {
    let (h3457, _) = f130(Fn4::F580(v0.clone()), {
        let a3459 = {
            let a3458 = v0;
            length_bytes(a3458)
        };
        let a3460 = 1u64;
        nat_add(a3459, a3460)?
    }, 0u64)?;
    Ok(h3457)
}

pub fn f556(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a3505 = {
            let a3481 = {
                let a3469 = {
                    let a3463 = {
                        let a3461 = v3;
                        let a3462 = 128u8;
                        fixed_u8::bit_and(a3461, a3462)
                    };
                    let a3464 = 128u8;
                    if equal(a3463, a3464) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3470 = {
                    let a3467 = {
                        let a3465 = v3;
                        let a3466 = 64u8;
                        fixed_u8::bit_and(a3465, a3466)
                    };
                    let a3468 = 64u8;
                    if equal(a3467, a3468) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3469, a3470)?
            };
            let a3482 = {
                let a3479 = {
                    let a3473 = {
                        let a3471 = v3;
                        let a3472 = 32u8;
                        fixed_u8::bit_and(a3471, a3472)
                    };
                    let a3474 = 32u8;
                    if equal(a3473, a3474) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3480 = {
                    let a3477 = {
                        let a3475 = v3;
                        let a3476 = 16u8;
                        fixed_u8::bit_and(a3475, a3476)
                    };
                    let a3478 = 16u8;
                    if equal(a3477, a3478) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3479, a3480)?
            };
            nat_add(a3481, a3482)?
        };
        let a3506 = {
            let a3503 = {
                let a3491 = {
                    let a3485 = {
                        let a3483 = v3;
                        let a3484 = 8u8;
                        fixed_u8::bit_and(a3483, a3484)
                    };
                    let a3486 = 8u8;
                    if equal(a3485, a3486) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3492 = {
                    let a3489 = {
                        let a3487 = v3;
                        let a3488 = 4u8;
                        fixed_u8::bit_and(a3487, a3488)
                    };
                    let a3490 = 4u8;
                    if equal(a3489, a3490) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3491, a3492)?
            };
            let a3504 = {
                let a3501 = {
                    let a3495 = {
                        let a3493 = v3;
                        let a3494 = 2u8;
                        fixed_u8::bit_and(a3493, a3494)
                    };
                    let a3496 = 2u8;
                    if equal(a3495, a3496) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3502 = {
                    let a3499 = {
                        let a3497 = v3;
                        let a3498 = 1u8;
                        fixed_u8::bit_and(a3497, a3498)
                    };
                    let a3500 = 1u8;
                    if equal(a3499, a3500) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3501, a3502)?
            };
            nat_add(a3503, a3504)?
        };
        nat_add(a3505, a3506)?
    };
    let a3507 = v4;
    let a3508 = 34u64;
    if nat_eq(a3507, a3508) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a3509 = v4;
        let a3510 = 92u64;
        if nat_eq(a3509, a3510) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a3511 = v4;
            let a3512 = 8u64;
            if nat_eq(a3511, a3512) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a3513 = v4;
                let a3514 = 12u64;
                if nat_eq(a3513, a3514) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a3515 = v4;
                    let a3516 = 10u64;
                    if nat_eq(a3515, a3516) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a3517 = v4;
                        let a3518 = 13u64;
                        if nat_eq(a3517, a3518) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a3519 = v4;
                            let a3520 = 9u64;
                            if nat_eq(a3519, a3520) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a3521 = v4;
                                let a3522 = 32u64;
                                if nat_lt(a3521, a3522) {
                                    let a3523 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a3524 = f581(v2)?;
                                    Ok(append_bytes(a3523, a3524))
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

pub fn f557(v0: Fn11, p3528: List<List<(Bytes, Bytes)>>, p3529: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m3527 = (p3528, p3529);
    {
        let mut m3527 = m3527;
        loop {
            let step = {
                let (v1, v2) = m3527;
                let m3525 = v2;
                match m3525.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c3526: Fn11 = v0.clone();
                            c3526.apply(v1, v3)
                        }, v4))
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

pub fn f558(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f559(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a3531 = {
        let a3530 = v0.clone();
        length_list(a3530)
    };
    let a3532 = 1u64;
    if nat_le(a3531, a3532) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f582(v0)?))
    }
}

pub fn f560(v0: u64) -> bool {
    let a3533 = 48u64;
    let a3534 = v0;
    if nat_le(a3533, a3534) {
        let a3535 = v0;
        let a3536 = 55u64;
        nat_le(a3535, a3536)
    } else {
        false
    }
}

pub fn f561(v0: u64) -> bool {
    if f269(v0) {
        true
    } else {
        if f370(v0) {
            true
        } else {
            let a3537 = v0;
            let a3538 = 95u64;
            if nat_eq(a3537, a3538) {
                true
            } else {
                let a3539 = v0;
                let a3540 = 45u64;
                nat_eq(a3539, a3540)
            }
        }
    }
}

pub fn f562(v0: u64) -> Bytes {
    f32({
        let a3541 = Str::lit("0123456789ABCDEF");
        utf8_encode(a3541)
    }, v0, 1u64)
}

pub fn f563(v0: bool, v1: List<Bytes>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m3542 = v1;
    match m3542.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m3543 = f302(v0, v3.clone())?;
            match m3543 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f163(f8(Str::lit("the component name ")), v3, f8(Str::lit(" derives no LexLean name")))))
                }
                Some(v5) => {
                    if f115(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f163(f8(Str::lit("two components derive the name ")), v5, f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n; when those names are not distinct, every name keeps its letters and digits and writes each other character as a C11 universal character name)")))))
                    } else {
                        let m3544 = f563(v0, v4, List::cons(v5.clone(), v2))?;
                        match m3544 {
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

pub fn f564(v0: Adt32) -> Adt32 {
    Adt32::C0(match v0.clone() {
        Adt32::C0(h3545, _, _, _, _) => {
            h3545
        }
    }, match v0 {
        Adt32::C0(_, h3546, _, _, _) => {
            h3546
        }
    }, 0u64, 0u64, 0u64)
}

pub fn f565(v0: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: Adt25) -> R<Result<Adt36, Bytes>> {
    let m3547 = v0;
    if m3547 == 0 {
        Ok(Err::<Adt36, Bytes>(f418(f8(Str::lit("the element declarations nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m3547 - 1;
        let m3548 = f241(v4.clone(), Str::lit("ref"));
        match m3548 {
            None => {
                let m3559 = f241(v4.clone(), Str::lit("name"));
                match m3559 {
                    None => {
                        Ok(Err::<Adt36, Bytes>(f418(f8(Str::lit("a local element declaration must have a name or a ref (XSD 1.1 Part 1, section 3.3.3)")), v2)))
                    }
                    Some(v12) => {
                        let m3562 = f414(v5, v1, {
                            let a3560 = v2;
                            let a3561 = f154(v12.clone())?;
                            append_bytes(a3560, a3561)
                        }, v3, v4)?;
                        match m3562 {
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
                let m3550 = f91(match v4 {
                    Adt25::C0(_, _, _, _, h3549) => {
                        h3549
                    }
                }, v6.clone())?;
                match m3550 {
                    Ok(v8) => {
                        let m3551 = f488(v1.clone(), Str::lit("element"), v8.clone());
                        match m3551 {
                            None => {
                                Ok(Err::<Adt36, Bytes>(f418(f163(f8(Str::lit("the element ")), v6, f8(Str::lit(" is not declared by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                            }
                            Some(v9) => {
                                if if f475(match v9.clone() {
                                    Adt24::C0(_, _, _, h3552) => {
                                        h3552
                                    }
                                }, Str::lit("abstract"), Str::lit("true")) {
                                    true
                                } else {
                                    f583(match v1.clone() {
                                        Adt23::C0(h3553, _) => {
                                            h3553
                                        }
                                    }, v8.clone())?
                                } {
                                    Ok(Err::<Adt36, Bytes>(f418(f163(f8(Str::lit("the element ")), v6, f8(Str::lit(" heads a substitution group, so its particle also admits the members of that group, which is not translated (XSD 1.1 Part 1, section 3.3.6.4)"))), v2)))
                                } else {
                                    let m3557 = f414(v5, v1, f154({
                                        let (_, h3554) = v8.clone();
                                        h3554
                                    })?, match v9.clone() {
                                        Adt24::C0(_, h3555, _, _) => {
                                            h3555
                                        }
                                    }, match v9 {
                                        Adt24::C0(_, _, _, h3556) => {
                                            h3556
                                        }
                                    })?;
                                    match m3557 {
                                        Ok(v11) => {
                                            Ok(Ok::<Adt36, Bytes>(Adt36::C0({
                                                let (_, h3558) = v8;
                                                h3558
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
                        Ok(Err::<Adt36, Bytes>(f418(v7, v2)))
                    }
                }
            }
        }
    }
}

pub fn f566(v0: Adt25, v1: Bytes) -> Option<Bytes> {
    let v2: Bytes = f244(v0.clone(), Str::lit("maxOccurs"), Str::lit("1"));
    if f35(v2.clone(), f8(Str::lit("0"))) {
        None::<Bytes>
    } else {
        if f35(v2, f8(Str::lit("1"))) {
            if f475(v0, Str::lit("minOccurs"), Str::lit("0")) {
                Some(f535(Str::lit("Option"), v1))
            } else {
                Some(v1)
            }
        } else {
            Some(f535(Str::lit("List"), v1))
        }
    }
}

pub fn f567(v0: Adt25) -> bool {
    if f35(f244(v0.clone(), Str::lit("minOccurs"), Str::lit("1")), f8(Str::lit("1"))) {
        f35(f244(v0, Str::lit("maxOccurs"), Str::lit("1")), f8(Str::lit("1")))
    } else {
        false
    }
}

pub fn f568(v0: Adt32, v1: Bytes) -> R<u64> {
    if f35(v1.clone(), f8(Str::lit("sequence"))) {
        let a3564 = match v0 {
            Adt32::C0(_, _, h3563, _, _) => {
                h3563
            }
        };
        let a3565 = 1u64;
        nat_add(a3564, a3565)
    } else {
        if f35(v1, f8(Str::lit("choice"))) {
            let a3567 = match v0 {
                Adt32::C0(_, _, _, h3566, _) => {
                    h3566
                }
            };
            let a3568 = 1u64;
            nat_add(a3567, a3568)
        } else {
            let a3570 = match v0 {
                Adt32::C0(_, _, _, _, h3569) => {
                    h3569
                }
            };
            let a3571 = 1u64;
            nat_add(a3570, a3571)
        }
    }
}

pub fn f569(v0: Adt32, v1: Bytes) -> R<Adt32> {
    if f35(v1.clone(), f8(Str::lit("sequence"))) {
        Ok(Adt32::C0(match v0.clone() {
            Adt32::C0(h3572, _, _, _, _) => {
                h3572
            }
        }, match v0.clone() {
            Adt32::C0(_, h3573, _, _, _) => {
                h3573
            }
        }, {
            let a3575 = match v0.clone() {
                Adt32::C0(_, _, h3574, _, _) => {
                    h3574
                }
            };
            let a3576 = 1u64;
            nat_add(a3575, a3576)?
        }, match v0.clone() {
            Adt32::C0(_, _, _, h3577, _) => {
                h3577
            }
        }, match v0 {
            Adt32::C0(_, _, _, _, h3578) => {
                h3578
            }
        }))
    } else {
        if f35(v1, f8(Str::lit("choice"))) {
            Ok(Adt32::C0(match v0.clone() {
                Adt32::C0(h3579, _, _, _, _) => {
                    h3579
                }
            }, match v0.clone() {
                Adt32::C0(_, h3580, _, _, _) => {
                    h3580
                }
            }, match v0.clone() {
                Adt32::C0(_, _, h3581, _, _) => {
                    h3581
                }
            }, {
                let a3583 = match v0.clone() {
                    Adt32::C0(_, _, _, h3582, _) => {
                        h3582
                    }
                };
                let a3584 = 1u64;
                nat_add(a3583, a3584)?
            }, match v0 {
                Adt32::C0(_, _, _, _, h3585) => {
                    h3585
                }
            }))
        } else {
            Ok(Adt32::C0(match v0.clone() {
                Adt32::C0(h3586, _, _, _, _) => {
                    h3586
                }
            }, match v0.clone() {
                Adt32::C0(_, h3587, _, _, _) => {
                    h3587
                }
            }, match v0.clone() {
                Adt32::C0(_, _, h3588, _, _) => {
                    h3588
                }
            }, match v0.clone() {
                Adt32::C0(_, _, _, h3589, _) => {
                    h3589
                }
            }, {
                let a3591 = match v0 {
                    Adt32::C0(_, _, _, _, h3590) => {
                        h3590
                    }
                };
                let a3592 = 1u64;
                nat_add(a3591, a3592)?
            }))
        }
    }
}

pub fn f570(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f494(v0.clone(), {
        let (h3593, _) = v2;
        h3593
    }, v1)?;
    let a3594 = f173(v0, v3)?;
    let a3595 = 10u64;
    if nat_eq(a3594, a3595) {
        Ok(Some(({
            let a3596 = v3;
            let a3597 = 1u64;
            nat_add(a3596, a3597)?
        }, {
            let a3599 = {
                let (_, h3598) = v2;
                h3598
            };
            let a3600 = 1u64;
            nat_add(a3599, a3600)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f571(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a3601 = f173(v0, v3)?;
    let a3602 = 32u64;
    if if nat_eq(a3601, a3602) {
        let a3605 = {
            let a3603 = v3;
            let a3604 = v1;
            nat_sub(a3603, a3604)
        };
        let a3606 = v2;
        nat_lt(a3605, a3606)
    } else {
        false
    } {
        Ok(Some({
            let a3607 = v3;
            let a3608 = 1u64;
            nat_add(a3607, a3608)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f572(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f173(v0, v1)?;
    let a3609 = v2;
    let a3610 = 34u64;
    if if if if nat_eq(a3609, a3610) {
        true
    } else {
        let a3611 = v2;
        let a3612 = 92u64;
        nat_eq(a3611, a3612)
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
            let a3613 = v1;
            let a3614 = 1u64;
            nat_add(a3613, a3614)?
        }))
    }
}

pub fn f573(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f173(v0, v1)?;
    let a3615 = v2;
    let a3616 = 39u64;
    if if if nat_eq(a3615, a3616) {
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
            let a3617 = v1;
            let a3618 = 1u64;
            nat_add(a3617, a3618)?
        }))
    }
}

pub fn f574(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a3620 = v2;
    let a3621 = {
        let a3619 = v0.clone();
        length_bytes(a3619)
    };
    if if nat_lt(a3620, a3621) {
        f178({
            let a3622 = f173(v0, v2)?;
            let a3623 = v1;
            nat_eq(a3622, a3623)
        })
    } else {
        false
    } {
        Ok(Some({
            let a3624 = v2;
            let a3625 = 1u64;
            nat_add(a3624, a3625)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f575(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m3627 = f544(f173(v0, {
        let (h3626, _) = v2;
        h3626
    })?);
    match m3627 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v3) => {
            let a3628 = v3;
            let a3629 = v1;
            if nat_lt(a3628, a3629) {
                Ok(Some(({
                    let a3631 = {
                        let (h3630, _) = v2;
                        h3630
                    };
                    let a3632 = 1u64;
                    nat_add(a3631, a3632)?
                }, {
                    let a3636 = {
                        let a3634 = {
                            let (_, h3633) = v2;
                            h3633
                        };
                        let a3635 = v1;
                        nat_mul(a3634, a3635)?
                    };
                    let a3637 = v3;
                    nat_add(a3636, a3637)?
                })))
            } else {
                Ok(None::<(u64, u64)>)
            }
        }
    }
}

pub fn f576(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f139(v0, v1)?;
    let a3638 = 47u64;
    let a3639 = v2;
    if if nat_lt(a3638, a3639) {
        let a3640 = v2;
        let a3641 = 58u64;
        nat_lt(a3640, a3641)
    } else {
        false
    } {
        Ok(Some({
            let a3642 = v1;
            let a3643 = 1u64;
            nat_add(a3642, a3643)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f577(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a3644 = f139(v0, v1)?;
    let a3645 = 48u64;
    if nat_eq(a3644, a3645) {
        Ok(Some({
            let a3646 = v1;
            let a3647 = 1u64;
            nat_add(a3646, a3647)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f578(v0: Bytes) -> R<u64> {
    let (h3648, _) = f130(Fn4::F584(v0.clone()), {
        let a3649 = v0;
        length_bytes(a3649)
    }, 0u64)?;
    Ok(h3648)
}

pub fn f579(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m3653 = {
        let a3651 = v0;
        let a3652 = {
            let (h3650, _) = v1;
            h3650
        };
        index_bytes(a3651, a3652)
    };
    match m3653 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a3655 = {
                    let (h3654, _) = v1;
                    h3654
                };
                let a3656 = 1u64;
                nat_add(a3655, a3656)?
            }, {
                let a3660 = {
                    let a3658 = {
                        let (_, h3657) = v1;
                        h3657
                    };
                    let a3659 = 10u64;
                    nat_mul(a3658, a3659)?
                };
                let a3661 = f585(v2)?;
                nat_add(a3660, a3661)?
            })))
        }
    }
}

pub fn f580(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f586(v0, v1)
}

pub fn f581(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3706 = {
            let a3682 = {
                let a3670 = {
                    let a3664 = {
                        let a3662 = v1;
                        let a3663 = 128u8;
                        fixed_u8::bit_and(a3662, a3663)
                    };
                    let a3665 = 128u8;
                    if equal(a3664, a3665) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3671 = {
                    let a3668 = {
                        let a3666 = v1;
                        let a3667 = 64u8;
                        fixed_u8::bit_and(a3666, a3667)
                    };
                    let a3669 = 64u8;
                    if equal(a3668, a3669) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3670, a3671)?
            };
            let a3683 = {
                let a3680 = {
                    let a3674 = {
                        let a3672 = v1;
                        let a3673 = 32u8;
                        fixed_u8::bit_and(a3672, a3673)
                    };
                    let a3675 = 32u8;
                    if equal(a3674, a3675) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3681 = {
                    let a3678 = {
                        let a3676 = v1;
                        let a3677 = 16u8;
                        fixed_u8::bit_and(a3676, a3677)
                    };
                    let a3679 = 16u8;
                    if equal(a3678, a3679) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3680, a3681)?
            };
            nat_add(a3682, a3683)?
        };
        let a3707 = {
            let a3704 = {
                let a3692 = {
                    let a3686 = {
                        let a3684 = v1;
                        let a3685 = 8u8;
                        fixed_u8::bit_and(a3684, a3685)
                    };
                    let a3687 = 8u8;
                    if equal(a3686, a3687) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3693 = {
                    let a3690 = {
                        let a3688 = v1;
                        let a3689 = 4u8;
                        fixed_u8::bit_and(a3688, a3689)
                    };
                    let a3691 = 4u8;
                    if equal(a3690, a3691) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3692, a3693)?
            };
            let a3705 = {
                let a3702 = {
                    let a3696 = {
                        let a3694 = v1;
                        let a3695 = 2u8;
                        fixed_u8::bit_and(a3694, a3695)
                    };
                    let a3697 = 2u8;
                    if equal(a3696, a3697) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3703 = {
                    let a3700 = {
                        let a3698 = v1;
                        let a3699 = 1u8;
                        fixed_u8::bit_and(a3698, a3699)
                    };
                    let a3701 = 1u8;
                    if equal(a3700, a3701) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3702, a3703)?
            };
            nat_add(a3704, a3705)?
        };
        nat_add(a3706, a3707)?
    };
    let a3714 = f126(f587(), {
        let a3708 = v2;
        let a3709 = 16u64;
        let a3710 = 0u64;
        nat_quot(a3708, a3709, a3710)
    });
    let a3715 = f126(f587(), {
        let a3711 = v2;
        let a3712 = 16u64;
        let a3713 = 0u64;
        nat_rem(a3711, a3712, a3713)
    });
    Ok(append_bytes(a3714, a3715))
}

pub fn f582(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f518({
        let (_, h3716) = {
            let (h3717, _) = f588(Fn12::F589, {
                let a3719 = {
                    let a3718 = v0.clone();
                    length_list(a3718)
                };
                let a3720 = 1u64;
                nat_add(a3719, a3720)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h3717
        };
        h3716
    }))
}

pub fn f583(p3729: List<Adt24>, v1: (Bytes, Bytes)) -> R<bool> {
    let m3728 = p3729;
    {
        let mut m3728 = m3728;
        loop {
            let step = {
                let v0 = m3728;
                let m3721 = v0;
                match m3721.uncons() {
                    None => {
                        Err::<List<Adt24>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if if f35(match v2.clone() {
                            Adt24::C0(h3722, _, _, _) => {
                                h3722
                            }
                        }, f8(Str::lit("element"))) {
                            let m3724 = f241(match v2.clone() {
                                Adt24::C0(_, _, _, h3723) => {
                                    h3723
                                }
                            }, Str::lit("substitutionGroup"));
                            match m3724 {
                                None => {
                                    false
                                }
                                Some(v4) => {
                                    f590({
                                        let m3727 = match v2 {
                                            Adt24::C0(_, _, _, h3726) => {
                                                h3726
                                            }
                                        };
                                        match m3727 {
                                            Adt25::C0(_, _, _, _, h3725) => {
                                                h3725
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
                Ok(next) => { m3728 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f584(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3732 = {
        let a3730 = v0;
        let a3731 = v1;
        index_bytes(a3730, a3731)
    };
    match m3732 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f592(v2)? {
                Ok(Some({
                    let a3733 = v1;
                    let a3734 = 1u64;
                    nat_add(a3733, a3734)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f585(v0: u8) -> R<u64> {
    let a3781 = {
        let v1: u8 = v0;
        let a3779 = {
            let a3755 = {
                let a3743 = {
                    let a3737 = {
                        let a3735 = v1;
                        let a3736 = 128u8;
                        fixed_u8::bit_and(a3735, a3736)
                    };
                    let a3738 = 128u8;
                    if equal(a3737, a3738) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3744 = {
                    let a3741 = {
                        let a3739 = v1;
                        let a3740 = 64u8;
                        fixed_u8::bit_and(a3739, a3740)
                    };
                    let a3742 = 64u8;
                    if equal(a3741, a3742) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3743, a3744)?
            };
            let a3756 = {
                let a3753 = {
                    let a3747 = {
                        let a3745 = v1;
                        let a3746 = 32u8;
                        fixed_u8::bit_and(a3745, a3746)
                    };
                    let a3748 = 32u8;
                    if equal(a3747, a3748) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3754 = {
                    let a3751 = {
                        let a3749 = v1;
                        let a3750 = 16u8;
                        fixed_u8::bit_and(a3749, a3750)
                    };
                    let a3752 = 16u8;
                    if equal(a3751, a3752) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3753, a3754)?
            };
            nat_add(a3755, a3756)?
        };
        let a3780 = {
            let a3777 = {
                let a3765 = {
                    let a3759 = {
                        let a3757 = v1;
                        let a3758 = 8u8;
                        fixed_u8::bit_and(a3757, a3758)
                    };
                    let a3760 = 8u8;
                    if equal(a3759, a3760) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3766 = {
                    let a3763 = {
                        let a3761 = v1;
                        let a3762 = 4u8;
                        fixed_u8::bit_and(a3761, a3762)
                    };
                    let a3764 = 4u8;
                    if equal(a3763, a3764) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3765, a3766)?
            };
            let a3778 = {
                let a3775 = {
                    let a3769 = {
                        let a3767 = v1;
                        let a3768 = 2u8;
                        fixed_u8::bit_and(a3767, a3768)
                    };
                    let a3770 = 2u8;
                    if equal(a3769, a3770) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3776 = {
                    let a3773 = {
                        let a3771 = v1;
                        let a3772 = 1u8;
                        fixed_u8::bit_and(a3771, a3772)
                    };
                    let a3774 = 1u8;
                    if equal(a3773, a3774) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3775, a3776)?
            };
            nat_add(a3777, a3778)?
        };
        nat_add(a3779, a3780)?
    };
    let a3782 = 48u64;
    Ok(nat_sub(a3781, a3782))
}

pub fn f586(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a3783 = v0.clone();
        length_bytes(a3783)
    };
    let a3784 = v1;
    let a3785 = v2;
    if if nat_lt(a3784, a3785) {
        let a3790 = f139(v0, {
            let a3788 = v2;
            let a3789 = {
                let a3786 = v1;
                let a3787 = 1u64;
                nat_add(a3786, a3787)?
            };
            nat_sub(a3788, a3789)
        })?;
        let a3791 = 48u64;
        nat_eq(a3790, a3791)
    } else {
        false
    } {
        Ok(Some({
            let a3792 = v1;
            let a3793 = 1u64;
            nat_add(a3792, a3793)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f587() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f588(v0: Fn12, p3798: u64, p3799: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m3797 = (p3798, p3799);
    {
        let mut m3797 = m3797;
        loop {
            let step = {
                let (v1, v2) = m3797;
                let m3794 = v1;
                if m3794 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3794 - 1;
                    let m3796 = {
                        let c3795: Fn12 = v0.clone();
                        c3795.apply(v2.clone())?
                    };
                    match m3796 {
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
                Ok(next) => { m3797 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f589(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f593(v0)
}

pub fn f590(v0: List<(Bytes, Bytes)>, p3807: List<Bytes>, v2: (Bytes, Bytes)) -> R<bool> {
    let m3806 = p3807;
    {
        let mut m3806 = m3806;
        loop {
            let step = {
                let v1 = m3806;
                let m3800 = v1;
                match m3800.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v3, v4)) => {
                        let m3801 = f91(v0.clone(), v3)?;
                        match m3801 {
                            Ok(v6) => {
                                if if f35({
                                    let (h3802, _) = v6;
                                    h3802
                                }, {
                                    let (h3803, _) = v2.clone();
                                    h3803
                                }) {
                                    f35({
                                        let (_, h3804) = v6;
                                        h3804
                                    }, {
                                        let (_, h3805) = v2.clone();
                                        h3805
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
                Ok(next) => { m3806 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f591(v0: Bytes) -> R<List<Bytes>> {
    f30(v0, 32u8)
}

pub fn f592(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3852 = {
            let a3828 = {
                let a3816 = {
                    let a3810 = {
                        let a3808 = v1;
                        let a3809 = 128u8;
                        fixed_u8::bit_and(a3808, a3809)
                    };
                    let a3811 = 128u8;
                    if equal(a3810, a3811) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3817 = {
                    let a3814 = {
                        let a3812 = v1;
                        let a3813 = 64u8;
                        fixed_u8::bit_and(a3812, a3813)
                    };
                    let a3815 = 64u8;
                    if equal(a3814, a3815) {
                        64u64
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
                        let a3819 = 32u8;
                        fixed_u8::bit_and(a3818, a3819)
                    };
                    let a3821 = 32u8;
                    if equal(a3820, a3821) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3827 = {
                    let a3824 = {
                        let a3822 = v1;
                        let a3823 = 16u8;
                        fixed_u8::bit_and(a3822, a3823)
                    };
                    let a3825 = 16u8;
                    if equal(a3824, a3825) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3826, a3827)?
            };
            nat_add(a3828, a3829)?
        };
        let a3853 = {
            let a3850 = {
                let a3838 = {
                    let a3832 = {
                        let a3830 = v1;
                        let a3831 = 8u8;
                        fixed_u8::bit_and(a3830, a3831)
                    };
                    let a3833 = 8u8;
                    if equal(a3832, a3833) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3839 = {
                    let a3836 = {
                        let a3834 = v1;
                        let a3835 = 4u8;
                        fixed_u8::bit_and(a3834, a3835)
                    };
                    let a3837 = 4u8;
                    if equal(a3836, a3837) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3838, a3839)?
            };
            let a3851 = {
                let a3848 = {
                    let a3842 = {
                        let a3840 = v1;
                        let a3841 = 2u8;
                        fixed_u8::bit_and(a3840, a3841)
                    };
                    let a3843 = 2u8;
                    if equal(a3842, a3843) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3849 = {
                    let a3846 = {
                        let a3844 = v1;
                        let a3845 = 1u8;
                        fixed_u8::bit_and(a3844, a3845)
                    };
                    let a3847 = 1u8;
                    if equal(a3846, a3847) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3848, a3849)?
            };
            nat_add(a3850, a3851)?
        };
        nat_add(a3852, a3853)?
    };
    let a3854 = 48u64;
    let a3855 = v2;
    if nat_le(a3854, a3855) {
        let a3856 = v2;
        let a3857 = 57u64;
        Ok(nat_le(a3856, a3857))
    } else {
        Ok(false)
    }
}

pub fn f593(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h3858, _) = v0;
        h3858
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h3859) = v0;
        h3859
    };
    let m3860 = v1;
    match m3860.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m3861 = v4;
            match m3861.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f594(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f594(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3866 = {
            let a3864 = {
                let a3862 = v0.clone();
                length_list(a3862)
            };
            let a3865 = {
                let a3863 = v1.clone();
                length_list(a3863)
            };
            nat_add(a3864, a3865)?
        };
        let a3867 = 1u64;
        nat_add(a3866, a3867)?
    };
    Ok(f595({
        let (_, h3868) = {
            let (_, h3869) = {
                let (h3870, _) = f596(Fn13::F597, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3870
            };
            h3869
        };
        h3868
    }))
}

pub fn f595(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f363(Fn5::F598, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f596(v0: Fn13, p3875: u64, p3876: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3874 = (p3875, p3876);
    {
        let mut m3874 = m3874;
        loop {
            let step = {
                let (v1, v2) = m3874;
                let m3871 = v1;
                if m3871 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3871 - 1;
                    let m3873 = {
                        let c3872: Fn13 = v0.clone();
                        c3872.apply(v2.clone())
                    };
                    match m3873 {
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
                Ok(next) => { m3874 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f597(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f599(v0)
}

pub fn f598(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f599(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h3877, _) = v0.clone();
        h3877
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h3878, _) = {
            let (_, h3879) = v0.clone();
            h3879
        };
        h3878
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h3880) = {
            let (_, h3881) = v0;
            h3881
        };
        h3880
    };
    let m3882 = v1.clone();
    match m3882.uncons() {
        None => {
            let m3883 = v2;
            match m3883.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3884 = v2.clone();
            match m3884.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f600({
                        let (h3885, _) = v8.clone();
                        h3885
                    }, {
                        let (h3886, _) = v6.clone();
                        h3886
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

pub fn f600(v0: Bytes, v1: Bytes) -> bool {
    let a3891 = {
        let a3887 = v0;
        let a3888 = v1;
        compare_bytes(a3887, a3888)
    };
    let a3892 = {
        let a3889 = Bytes::lit(&[]);
        let a3890 = Bytes::lit(&[0x01]);
        compare_bytes(a3889, a3890)
    };
    equal(a3891, a3892)
}

pub fn run(p0: Option<Adt0>, p1: Adt2) -> R<(Adt0, Adt9)> {
    f0(p0, p1)
}
