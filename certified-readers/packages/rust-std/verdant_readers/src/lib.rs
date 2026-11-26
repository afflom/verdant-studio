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
    C0(bool),
    C1(bool, u64),
    C2(bool, u64, u64),
    C3(Bytes),
    C4(std::rc::Rc<Adt17>),
    C5(std::rc::Rc<Adt17>),
    C6,
    C7(std::rc::Rc<Adt17>, std::rc::Rc<Adt17>),
    C8(Bytes, std::rc::Rc<Adt17>, std::rc::Rc<Adt17>),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(Adt17, u64),
    C1(Bytes),
}

#[derive(Clone)]
pub enum Adt19 {
    C0(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt20 {
    C0(Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt21 {
    C0(Bytes, Adt16, Adt16, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Adt22 {
    C0(Bytes, u64),
    C1(Bytes),
}

#[derive(Clone)]
pub enum Adt23 {
    C0(Bytes, List<Adt19>),
}

#[derive(Clone)]
pub enum Adt24 {
    C0(Adt14, List<Adt19>),
}

#[derive(Clone)]
pub enum Adt25 {
    C0(u64, u64),
    C1(Bytes),
}

#[derive(Clone)]
pub enum Adt26 {
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
    F126(u64, Bytes),
    F127(Bytes),
    F335(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn3::F126(k0, k1) => Ok(f126(*k0, k1.clone(), p0)),
            Fn3::F127(k0) => f127(k0.clone(), p0),
            Fn3::F335(k0) => f335(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F129(Bytes),
    F155(Bytes),
    F156(Bytes),
    F210(Bytes, Bytes),
    F211(Bytes),
    F212(Bytes, u64),
    F227(Bytes),
    F228(Bytes),
    F296(bool, u64, Bytes),
    F309(Bytes),
    F334(Bytes),
    F339(u64, Bytes),
    F431(u64, Bytes, u64),
    F434(Bytes),
    F435(Bytes),
    F437(Bytes, u64),
    F440(Bytes),
    F441(Bytes),
    F467(Bytes),
    F473(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn4::F129(k0) => f129(k0.clone(), p0),
            Fn4::F155(k0) => f155(k0.clone(), p0),
            Fn4::F156(k0) => f156(k0.clone(), p0),
            Fn4::F210(k0, k1) => f210(k0.clone(), k1.clone(), p0),
            Fn4::F211(k0) => f211(k0.clone(), p0),
            Fn4::F212(k0, k1) => f212(k0.clone(), *k1, p0),
            Fn4::F227(k0) => f227(k0.clone(), p0),
            Fn4::F228(k0) => f228(k0.clone(), p0),
            Fn4::F296(k0, k1, k2) => f296(*k0, *k1, k2.clone(), p0),
            Fn4::F309(k0) => f309(k0.clone(), p0),
            Fn4::F334(k0) => f334(k0.clone(), p0),
            Fn4::F339(k0, k1) => f339(*k0, k1.clone(), p0),
            Fn4::F431(k0, k1, k2) => f431(*k0, k1.clone(), *k2, p0),
            Fn4::F434(k0) => f434(k0.clone(), p0),
            Fn4::F435(k0) => f435(k0.clone(), p0),
            Fn4::F437(k0, k1) => f437(k0.clone(), *k1, p0),
            Fn4::F440(k0) => f440(k0.clone(), p0),
            Fn4::F441(k0) => f441(k0.clone(), p0),
            Fn4::F467(k0) => f467(k0.clone(), p0),
            Fn4::F473(k0) => f473(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F244,
    F485,
}

impl Fn5 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn5::F244 => f244(p0, p1),
            Fn5::F485 => f485(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F247,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn6::F247 => f247(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F317(Bytes),
    F326(bool, Bytes),
    F370(bool, Bytes),
    F386(Bytes),
    F397(Bytes),
    F430(u64, Bytes),
    F439(u64, Bytes),
    F443(Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn7::F317(k0) => f317(k0.clone(), p0),
            Fn7::F326(k0, k1) => f326(*k0, k1.clone(), p0),
            Fn7::F370(k0, k1) => f370(*k0, k1.clone(), p0),
            Fn7::F386(k0) => f386(k0.clone(), p0),
            Fn7::F397(k0) => f397(k0.clone(), p0),
            Fn7::F430(k0, k1) => f430(*k0, k1.clone(), p0),
            Fn7::F439(k0, k1) => f439(*k0, k1.clone(), p0),
            Fn7::F443(k0) => f443(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F394(u64, Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: (u64, bool)) -> R<Option<(u64, bool)>> {
        match self {
            Fn8::F394(k0, k1) => f394(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F408,
}

impl Fn9 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn9::F408 => f408(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F410,
}

impl Fn10 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn10::F410 => f410(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F447,
}

impl Fn11 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn11::F447 => f447(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F478,
}

impl Fn12 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn12::F478 => f478(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F484,
}

impl Fn13 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn13::F484 => f484(p0),
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
                    let v7: List<Adt15> = f65({
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

pub fn f52(v0: Bytes) -> R<Result<Adt16, Bytes>> {
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
                                                    Ok(Ok::<Adt16, Bytes>({
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

pub fn f53(v0: Adt16) -> Bytes {
    let m319 = v0;
    match m319 {
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

pub fn f54(v0: Adt16) -> R<u64> {
    let m320 = v0;
    match m320 {
        Adt16::C0(_, _, r325) => {
            let v3 = (*r325).clone();
            let a321 = 1u64;
            let a322 = f54(v3)?;
            nat_add(a321, a322)
        }
        Adt16::C1(_, _, _) => {
            Ok(0u64)
        }
        Adt16::C2(_) => {
            Ok(0u64)
        }
        Adt16::C3(r326, r327) => {
            let v8 = (*r326).clone();
            let v9 = (*r327).clone();
            let a323 = f54(v8)?;
            let a324 = f54(v9)?;
            nat_add(a323, a324)
        }
        Adt16::C4 => {
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
                Adt16::C0(v3, r341, r342) => {
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
                Adt16::C1(_, _, _) => {
                    Ok(f87(f8(Str::lit("the document has no root element"))))
                }
                Adt16::C2(_) => {
                    Ok(f87(f8(Str::lit("the document has no root element"))))
                }
                Adt16::C3(_, _) => {
                    Ok(f87(f8(Str::lit("the document has no root element"))))
                }
                Adt16::C4 => {
                    Ok(f87(f8(Str::lit("the document has no root element"))))
                }
            }
        }
        Err(v1) => {
            Ok(f87(f7(List::cons(f8(Str::lit("not well-formed XML: ")), List::cons(v1, List::<Bytes>::nil())))))
        }
    }
}

pub fn f56(v0: Bytes) -> R<Result<Adt17, Bytes>> {
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
        Ok(Err::<Adt17, Bytes>(f81(Str::lit("the input is not a sequence of printable characters in UTF-8 (YAML 1.2.2 [1], section 5.2)"), v4)?))
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
            Ok(Err::<Adt17, Bytes>(f81(Str::lit("a stream with no document is not read (YAML 1.2.2 [211])"), v6)?))
        } else {
            let a353 = f100(v2.clone(), v6)?;
            let a354 = 37u64;
            if nat_eq(a353, a354) {
                Ok(Err::<Adt17, Bytes>(f81(Str::lit("a directive is not read (YAML 1.2.2 [82])"), v6)?))
            } else {
                let a355 = v3;
                let a356 = v6;
                if nat_le(a355, a356) {
                    Ok(Err::<Adt17, Bytes>(f81(Str::lit("a stream with no document is not read (YAML 1.2.2 [211])"), v6)?))
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
                        Adt18::C0(v8, v9) => {
                            let m360 = f102(v2, v9)?;
                            match m360 {
                                Ok(_) => {
                                    Ok(Ok::<Adt17, Bytes>(v8))
                                }
                                Err(v10) => {
                                    Ok(Err::<Adt17, Bytes>(v10))
                                }
                            }
                        }
                        Adt18::C1(v7) => {
                            Ok(Err::<Adt17, Bytes>(v7))
                        }
                    }
                }
            }
        }
    }
}

pub fn f57(v0: Adt17) -> R<Bytes> {
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

pub fn f65(p399: u64, p400: List<Adt15>) -> R<List<Adt15>> {
    let m398 = (p399, p400);
    {
        let mut m398 = m398;
        loop {
            let step = {
                let (v0, v1) = m398;
                let m395 = v0;
                if m395 == 0 {
                    Err::<(u64, List<Adt15>), R<List<Adt15>>>(Ok(v1))
                } else {
                    let v2 = m395 - 1;
                    let v3: List<Adt15> = f107(v1.clone(), v1.clone());
                    let a396 = f108(v3.clone())?;
                    let a397 = f108(v1.clone())?;
                    if nat_eq(a396, a397) {
                        Err::<(u64, List<Adt15>), R<List<Adt15>>>(Ok(v1))
                    } else {
                        Ok::<(u64, List<Adt15>), R<List<Adt15>>>((v2, v3))
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

pub fn f66(v0: List<Adt12>, v1: List<Adt12>) -> R<List<Adt15>> {
    let m401 = v0;
    match m401.uncons() {
        None => {
            Ok(List::<Adt15>::nil())
        }
        Some((v2, v3)) => {
            if f109(f110(v2.clone())) {
                let v6: Adt15 = {
                    let m402 = f111(v1.clone(), v2.clone())?;
                    match m402 {
                        Ok(v5) => {
                            Adt15::C0(f112(v2), Bytes::lit(&[]), {
                                let (h403, _) = v5;
                                h403
                            }, {
                                let (_, h404) = v5;
                                h404
                            })
                        }
                        Err(v4) => {
                            Adt15::C0(f112(v2), v4, Bytes::lit(&[]), List::<Bytes>::nil())
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

pub fn f67(p409: u64, p410: List<Bytes>, p411: List<Bytes>, v3: List<Adt15>) -> List<Bytes> {
    let m408 = (p409, (p410, p411));
    {
        let mut m408 = m408;
        loop {
            let step = {
                let (v0, (v1, v2)) = m408;
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
                                        if f35(f115(v7.clone()), Bytes::lit(&[])) {
                                            Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (f116(f117(v7), v6), List::cons(v5, v2))))
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
                Ok(next) => { m408 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f68(v0: List<Adt12>) -> List<Bytes> {
    let m412 = v0;
    match m412.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let v3: u64 = f110(v1.clone());
            let a413 = v3;
            let a414 = 0u64;
            if if nat_eq(a413, a414) {
                true
            } else {
                let a415 = v3;
                let a416 = 2u64;
                if nat_eq(a415, a416) {
                    true
                } else {
                    let a417 = v3;
                    let a418 = 3u64;
                    if nat_eq(a417, a418) {
                        true
                    } else {
                        let a419 = v3;
                        let a420 = 4u64;
                        if nat_eq(a419, a420) {
                            true
                        } else {
                            let a421 = v3;
                            let a422 = 5u64;
                            if nat_eq(a421, a422) {
                                true
                            } else {
                                let a423 = v3;
                                let a424 = 7u64;
                                if nat_eq(a423, a424) {
                                    true
                                } else {
                                    let a425 = v3;
                                    let a426 = 8u64;
                                    nat_eq(a425, a426)
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

pub fn f69(v0: List<Adt15>, v1: List<Bytes>) -> List<(Bytes, (List<Bytes>, Bytes))> {
    let m427 = v0;
    match m427.uncons() {
        None => {
            List::<(Bytes, (List<Bytes>, Bytes))>::nil()
        }
        Some((v2, v3)) => {
            if if f113(f118(v2.clone()), v1.clone()) {
                f35(f115(v2.clone()), Bytes::lit(&[]))
            } else {
                false
            } {
                List::cons((f118(v2.clone()), (f117(v2.clone()), f119(v2))), f69(v3, v1))
            } else {
                f69(v3, v1)
            }
        }
    }
}

pub fn f70(p436: u64, p437: List<(Bytes, (List<Bytes>, Bytes))>, p438: List<Bytes>, p439: List<Bytes>) -> (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>) {
    let m435 = (p436, (p437, (p438, p439)));
    {
        let mut m435 = m435;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m435;
                let m428 = v0;
                if m428 == 0 {
                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v3, v1))
                } else {
                    let v4 = m428 - 1;
                    let m429 = f120(v1.clone(), v2.clone());
                    match m429 {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v3, v1))
                        }
                        Some(v5) => {
                            Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v4, ({
                                let (_, h430) = v5.clone();
                                h430
                            }, (List::cons({
                                let (h431, _) = {
                                    let (h432, _) = v5.clone();
                                    h432
                                };
                                h431
                            }, v2), List::cons({
                                let (_, h433) = {
                                    let (h434, _) = v5;
                                    h434
                                };
                                h433
                            }, v3)))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m435 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f71(v0: List<Adt12>, v1: List<Adt15>, v2: List<Adt12>) -> List<Bytes> {
    let m440 = v0;
    match m440.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            let v5: u64 = f110(v3.clone());
            let a441 = v5;
            let a442 = 5u64;
            if if nat_eq(a441, a442) {
                true
            } else {
                let a443 = v5;
                let a444 = 7u64;
                nat_eq(a443, a444)
            } {
                let m447 = f121(v2.clone(), {
                    let a445 = v5;
                    let a446 = 5u64;
                    if nat_eq(a445, a446) {
                        0u64
                    } else {
                        2u64
                    }
                }, f112(v3.clone()));
                match m447 {
                    None => {
                        List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f122(v5), List::cons(f112(v3.clone()), List::cons(f8(Str::lit(": no input defines ")), List::cons(f112(v3), List::cons(f8(Str::lit(", so its members join no definition (Web IDL 2.2 and 2.7)\u{a}")), List::<Bytes>::nil()))))))), f71(v4, v1, v2))
                    }
                    Some(_) => {
                        f71(v4, v1, v2)
                    }
                }
            } else {
                let a448 = v5;
                let a449 = 8u64;
                if nat_eq(a448, a449) {
                    let m450 = f121(v2.clone(), 0u64, f112(v3.clone()));
                    match m450 {
                        None => {
                            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f112(v3.clone()), List::cons(f8(Str::lit(" includes ")), List::cons(f123(v3.clone()), List::cons(f8(Str::lit(": no input defines the interface ")), List::cons(f112(v3), List::cons(f8(Str::lit(" (Web IDL 2.3)\u{a}")), List::<Bytes>::nil())))))))), f71(v4, v1, v2))
                        }
                        Some(_) => {
                            f71(v4, v1, v2)
                        }
                    }
                } else {
                    if f109(v5) {
                        let m451 = f114(v1.clone(), f112(v3.clone()));
                        match m451 {
                            None => {
                                f71(v4, v1, v2)
                            }
                            Some(v8) => {
                                if f35(f115(v8.clone()), Bytes::lit(&[])) {
                                    f71(v4, v1, v2)
                                } else {
                                    List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f112(v3), List::cons(f8(Str::lit(": ")), List::cons(f115(v8), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), f71(v4, v1, v2))
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
    let m452 = v0;
    match m452.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons({
                let (h453, _) = v1;
                h453
            }, List::cons(f8(Str::lit(": it refers back to itself through its members, and the translator emits non-recursive declarations only\u{a}")), List::<Bytes>::nil())))), f72(v2))
        }
    }
}

pub fn f73(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m454 = v0;
    match m454.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m455 = v3.clone();
            match m455.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a458 = v2;
                    let a459 = {
                        let a456 = v1.clone();
                        let a457 = f73(v3, v1);
                        append_bytes(a456, a457)
                    };
                    append_bytes(a458, a459)
                }
            }
        }
    }
}

pub fn f74(p462: List<Bytes>, p463: List<Bytes>) -> List<Bytes> {
    let m461 = (p462, p463);
    {
        let mut m461 = m461;
        loop {
            let step = {
                let (v0, v1) = m461;
                let m460 = v0;
                match m460.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
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

pub fn f75(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a464 = v2;
    let a465 = 0u64;
    if nat_eq(a464, a465) {
        Ok(f124(v0, 0u64))
    } else {
        let (_, h466) = {
            let (h467, _) = f125(Fn3::F126(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h467
        };
        Ok(h466)
    }
}

pub fn f76() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f77(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m472 = {
        let a469 = v0;
        let a470 = v2;
        let a471 = {
            let a468 = v1.clone();
            length_bytes(a468)
        };
        slice_bytes(a469, a470, a471)
    };
    match m472 {
        None => {
            false
        }
        Some(v3) => {
            let a473 = v3;
            let a474 = v1;
            equal(a473, a474)
        }
    }
}

pub fn f78(v0: Bytes) -> R<Bytes> {
    let (_, h475) = {
        let (h476, _) = f125(Fn3::F127(v0.clone()), {
            let a478 = {
                let a477 = v0;
                length_bytes(a477)
            };
            let a479 = 1u64;
            nat_add(a478, a479)?
        }, (0u64, Bytes::lit(&[])))?;
        h476
    };
    Ok(h475)
}

pub fn f79(v0: Bytes) -> R<u64> {
    let (h480, _) = f128(Fn4::F129(v0.clone()), {
        let a482 = {
            let a481 = v0;
            length_bytes(a481)
        };
        let a483 = 1u64;
        nat_add(a482, a483)?
    }, 0u64)?;
    Ok(h480)
}

pub fn f80(v0: Bytes) -> Result<Adt16, Bytes> {
    Err::<Adt16, Bytes>(v0)
}

pub fn f81(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a484 = v0;
        utf8_encode(a484)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f51(v1)?, List::<Bytes>::nil())))))
}

pub fn f82(v0: Bytes) -> R<Result<u64, Bytes>> {
    if if f84(v0.clone(), 0u64, Str::lit("<?xml")) {
        f130(f85(v0.clone(), 5u64)?)?
    } else {
        false
    } {
        let m486 = f131(v0.clone(), {
            let a485 = Str::lit("?>");
            utf8_encode(a485)
        }, 5u64)?;
        match m486 {
            None => {
                Ok(f132(f81(Str::lit("the XML declaration is not closed (XML 1.0 [23])"), 0u64)?))
            }
            Some(v1) => {
                let m488 = f131(f32(v0.clone(), 0u64, v1), {
                    let a487 = Str::lit("encoding");
                    utf8_encode(a487)
                }, 5u64)?;
                match m488 {
                    None => {
                        Ok(Ok::<u64, Bytes>({
                            let a489 = v1;
                            let a490 = 2u64;
                            nat_add(a489, a490)?
                        }))
                    }
                    Some(v2) => {
                        let v3: u64 = f133(v0.clone(), {
                            let a491 = v2;
                            let a492 = 8u64;
                            nat_add(a491, a492)?
                        })?;
                        let v4: u64 = f133(v0.clone(), {
                            let a493 = v3;
                            let a494 = 1u64;
                            nat_add(a493, a494)?
                        })?;
                        let _: u64 = f85(v0.clone(), v4)?;
                        let v7: u64 = {
                            let m497 = f131(v0.clone(), f32(v0.clone(), v4, 1u64), {
                                let a495 = v4;
                                let a496 = 1u64;
                                nat_add(a495, a496)?
                            })?;
                            match m497 {
                                None => {
                                    v1
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        if f134(f32(v0, {
                            let a498 = v4;
                            let a499 = 1u64;
                            nat_add(a498, a499)?
                        }, {
                            let a502 = v7;
                            let a503 = {
                                let a500 = v4;
                                let a501 = 1u64;
                                nat_add(a500, a501)?
                            };
                            nat_sub(a502, a503)
                        })) {
                            Ok(Ok::<u64, Bytes>({
                                let a504 = v1;
                                let a505 = 2u64;
                                nat_add(a504, a505)?
                            }))
                        } else {
                            Ok(f132(f81(Str::lit("only the UTF-8 encoding is read (XML 1.0 section 4.3.3)"), 0u64)?))
                        }
                    }
                }
            }
        }
    } else {
        Ok(Ok::<u64, Bytes>(0u64))
    }
}

pub fn f83(v0: Bytes, p510: u64, p511: u64) -> R<Result<u64, Bytes>> {
    let m509 = (p510, p511);
    {
        let mut m509 = m509;
        loop {
            let step = {
                let (v1, v2) = m509;
                let m506 = v2;
                if m506 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f132(f81(Str::lit("the prolog is longer than the input"), v1)?)))
                } else {
                    let v3 = m506 - 1;
                    let v4: u64 = f133(v0.clone(), v1)?;
                    if f84(v0.clone(), v4, Str::lit("<!--")) {
                        let m507 = f135(v0.clone(), v4)?;
                        match m507 {
                            Ok(v6) => {
                                Ok::<(u64, u64), R<Result<u64, Bytes>>>((v6, v3))
                            }
                            Err(v5) => {
                                Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f132(v5)))
                            }
                        }
                    } else {
                        if f84(v0.clone(), v4, Str::lit("<?")) {
                            let m508 = f136(v0.clone(), v4)?;
                            match m508 {
                                Ok(v8) => {
                                    Ok::<(u64, u64), R<Result<u64, Bytes>>>((v8, v3))
                                }
                                Err(v7) => {
                                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f132(v7)))
                                }
                            }
                        } else {
                            Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Ok::<u64, Bytes>(v4)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m509 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f84(v0: Bytes, v1: u64, v2: Str) -> bool {
    f77(v0, {
        let a512 = v2;
        utf8_encode(a512)
    }, v1)
}

pub fn f85(v0: Bytes, v1: u64) -> R<u64> {
    f137(v0, v1)
}

pub fn f86(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Result<(Adt16, u64), Bytes>> {
    let m513 = v2;
    if m513 == 0 {
        Ok(f138(f81(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v4 = m513 - 1;
        let a514 = v3;
        let a515 = 0u64;
        if nat_eq(a514, a515) {
            let v5: u64 = f139(v0.clone(), {
                let a516 = v1;
                let a517 = 1u64;
                nat_add(a516, a517)?
            })?;
            let a520 = v5;
            let a521 = {
                let a518 = v1;
                let a519 = 1u64;
                nat_add(a518, a519)?
            };
            if nat_eq(a520, a521) {
                Ok(f138(f81(Str::lit("an element must start with a Name (XML 1.0 [40])"), v1)?))
            } else {
                let v6: Bytes = f32(v0.clone(), {
                    let a522 = v1;
                    let a523 = 1u64;
                    nat_add(a522, a523)?
                }, {
                    let a526 = v5;
                    let a527 = {
                        let a524 = v1;
                        let a525 = 1u64;
                        nat_add(a524, a525)?
                    };
                    nat_sub(a526, a527)
                });
                let m528 = f140(v0.clone(), v5, v4, List::<Bytes>::nil())?;
                match m528 {
                    Ok(v8) => {
                        if f84(v0.clone(), {
                            let (_, h529) = v8.clone();
                            h529
                        }, Str::lit("/>")) {
                            Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C0(v6, std::rc::Rc::new({
                                let (h530, _) = v8;
                                h530
                            }), std::rc::Rc::new(Adt16::C4)), {
                                let a532 = {
                                    let (_, h531) = v8;
                                    h531
                                };
                                let a533 = 2u64;
                                nat_add(a532, a533)?
                            })))
                        } else {
                            let m537 = f86(v0.clone(), {
                                let a535 = {
                                    let (_, h534) = v8;
                                    h534
                                };
                                let a536 = 1u64;
                                nat_add(a535, a536)?
                            }, v4, 1u64)?;
                            match m537 {
                                Ok(v10) => {
                                    let v11: u64 = {
                                        let a539 = {
                                            let (_, h538) = v10.clone();
                                            h538
                                        };
                                        let a540 = 2u64;
                                        nat_add(a539, a540)?
                                    };
                                    let v12: u64 = f139(v0.clone(), v11)?;
                                    if f35(f32(v0.clone(), v11, {
                                        let a541 = v12;
                                        let a542 = v11;
                                        nat_sub(a541, a542)
                                    }), v6.clone()) {
                                        let v13: u64 = f133(v0.clone(), v12)?;
                                        let a543 = f85(v0, v13)?;
                                        let a544 = 62u64;
                                        if nat_eq(a543, a544) {
                                            Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C0(v6, std::rc::Rc::new({
                                                let (h545, _) = v8;
                                                h545
                                            }), std::rc::Rc::new({
                                                let (h546, _) = v10;
                                                h546
                                            })), {
                                                let a547 = v13;
                                                let a548 = 1u64;
                                                nat_add(a547, a548)?
                                            })))
                                        } else {
                                            Ok(f138(f81(Str::lit("an end tag must close with > (XML 1.0 [42])"), v13)?))
                                        }
                                    } else {
                                        Ok(f138(f81(Str::lit("an end tag must name the element its start tag opened (XML 1.0 WFC: Element Type Match)"), {
                                            let (_, h549) = v10;
                                            h549
                                        })?))
                                    }
                                }
                                Err(v9) => {
                                    Ok(f138(v9))
                                }
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(f138(v7))
                    }
                }
            }
        } else {
            let a550 = f85(v0.clone(), v1)?;
            let a551 = 256u64;
            if nat_eq(a550, a551) {
                Ok(f138(f81(Str::lit("the input ends before an element is closed (XML 1.0 [39])"), v1)?))
            } else {
                if f84(v0.clone(), v1, Str::lit("</")) {
                    Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C4, v1)))
                } else {
                    if f84(v0.clone(), v1, Str::lit("<!--")) {
                        let m552 = f135(v0.clone(), v1)?;
                        match m552 {
                            Ok(v15) => {
                                f86(v0, v15, v4, 1u64)
                            }
                            Err(v14) => {
                                Ok(f138(v14))
                            }
                        }
                    } else {
                        if f84(v0.clone(), v1, Str::lit("<![CDATA[")) {
                            let m556 = f131(v0.clone(), {
                                let a553 = Str::lit("]]>");
                                utf8_encode(a553)
                            }, {
                                let a554 = v1;
                                let a555 = 9u64;
                                nat_add(a554, a555)?
                            })?;
                            match m556 {
                                None => {
                                    Ok(f138(f81(Str::lit("a CDATA section is not closed (XML 1.0 [18])"), v1)?))
                                }
                                Some(v16) => {
                                    let m559 = f86(v0.clone(), {
                                        let a557 = v16;
                                        let a558 = 3u64;
                                        nat_add(a557, a558)?
                                    }, v4, 1u64)?;
                                    match m559 {
                                        Ok(v18) => {
                                            Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C3(std::rc::Rc::new(Adt16::C2(f32(v0, {
                                                let a560 = v1;
                                                let a561 = 9u64;
                                                nat_add(a560, a561)?
                                            }, {
                                                let a564 = v16;
                                                let a565 = {
                                                    let a562 = v1;
                                                    let a563 = 9u64;
                                                    nat_add(a562, a563)?
                                                };
                                                nat_sub(a564, a565)
                                            }))), std::rc::Rc::new({
                                                let (h566, _) = v18;
                                                h566
                                            })), {
                                                let (_, h567) = v18;
                                                h567
                                            })))
                                        }
                                        Err(v17) => {
                                            Ok(f138(v17))
                                        }
                                    }
                                }
                            }
                        } else {
                            if f84(v0.clone(), v1, Str::lit("<?")) {
                                let m568 = f136(v0.clone(), v1)?;
                                match m568 {
                                    Ok(v20) => {
                                        f86(v0, v20, v4, 1u64)
                                    }
                                    Err(v19) => {
                                        Ok(f138(v19))
                                    }
                                }
                            } else {
                                if f84(v0.clone(), v1, Str::lit("<!")) {
                                    Ok(f138(f81(Str::lit("a markup declaration is not allowed in content (XML 1.0 [43])"), v1)?))
                                } else {
                                    let a569 = f85(v0.clone(), v1)?;
                                    let a570 = 60u64;
                                    if nat_eq(a569, a570) {
                                        let m571 = f86(v0.clone(), v1, v4, 0u64)?;
                                        match m571 {
                                            Ok(v22) => {
                                                let m573 = f86(v0, {
                                                    let (_, h572) = v22;
                                                    h572
                                                }, v4, 1u64)?;
                                                match m573 {
                                                    Ok(v24) => {
                                                        Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C3(std::rc::Rc::new({
                                                            let (h574, _) = v22;
                                                            h574
                                                        }), std::rc::Rc::new({
                                                            let (h575, _) = v24;
                                                            h575
                                                        })), {
                                                            let (_, h576) = v24;
                                                            h576
                                                        })))
                                                    }
                                                    Err(v23) => {
                                                        Ok(f138(v23))
                                                    }
                                                }
                                            }
                                            Err(v21) => {
                                                Ok(f138(v21))
                                            }
                                        }
                                    } else {
                                        let m577 = f141(v0.clone(), v1, v4, Bytes::lit(&[]), false, 0u64)?;
                                        match m577 {
                                            Ok(v26) => {
                                                let m579 = f86(v0, {
                                                    let (_, h578) = v26;
                                                    h578
                                                }, v4, 1u64)?;
                                                match m579 {
                                                    Ok(v28) => {
                                                        Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C3(std::rc::Rc::new(Adt16::C2({
                                                            let (h580, _) = v26;
                                                            h580
                                                        })), std::rc::Rc::new({
                                                            let (h581, _) = v28;
                                                            h581
                                                        })), {
                                                            let (_, h582) = v28;
                                                            h582
                                                        })))
                                                    }
                                                    Err(v27) => {
                                                        Ok(f138(v27))
                                                    }
                                                }
                                            }
                                            Err(v25) => {
                                                Ok(f138(v25))
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

pub fn f88(v0: Adt16, v1: List<(Bytes, Bytes)>) -> Result<List<(Bytes, Bytes)>, Bytes> {
    let m583 = v0;
    match m583 {
        Adt16::C0(_, _, _) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt16::C1(v2, v3, r588) => {
            let v4 = (*r588).clone();
            let m584 = f88(v4, v1);
            match m584 {
                Ok(v6) => {
                    if f35(v2.clone(), f8(Str::lit("xmlns"))) {
                        if if f35(v3.clone(), f143()) {
                            true
                        } else {
                            f35(v3.clone(), f144())
                        } {
                            f142(f8(Str::lit("the default namespace must not be the xml or xmlns namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Bytes>(List::cons((Bytes::lit(&[]), v3), v6))
                        }
                    } else {
                        if f145(v2.clone(), f8(Str::lit("xmlns:"))) {
                            let v7: Bytes = f32(v2.clone(), 6u64, {
                                let a586 = {
                                    let a585 = v2;
                                    length_bytes(a585)
                                };
                                let a587 = 6u64;
                                nat_sub(a586, a587)
                            });
                            if f35(v7.clone(), f8(Str::lit("xmlns"))) {
                                f142(f8(Str::lit("the prefix xmlns must not be declared (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                            } else {
                                if if f35(v7.clone(), f8(Str::lit("xml"))) {
                                    !f35(v3.clone(), f143())
                                } else {
                                    false
                                } {
                                    f142(f8(Str::lit("the prefix xml may be bound only to its own namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                } else {
                                    if if if f35(v3.clone(), f143()) {
                                        true
                                    } else {
                                        f35(v3.clone(), f144())
                                    } {
                                        !f35(v7.clone(), f8(Str::lit("xml")))
                                    } else {
                                        false
                                    } {
                                        f142(f8(Str::lit("the xml and xmlns namespace names must not be bound to another prefix (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                    } else {
                                        if f35(v3.clone(), Bytes::lit(&[])) {
                                            f142(f8(Str::lit("a prefix must not be undeclared (Namespaces in XML 1.0, section 3, NSC: No Prefix Undeclaring)")))
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
                    f142(v5)
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

pub fn f89(v0: List<(Bytes, Bytes)>, v1: Bytes) -> R<Result<(Bytes, Bytes), Bytes>> {
    let v2: Bytes = f146(v1.clone())?;
    if f35(v2.clone(), Bytes::lit(&[])) {
        let m589 = f147(v0, Bytes::lit(&[]));
        match m589 {
            None => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((Bytes::lit(&[]), v1)))
            }
            Some(v3) => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((v3, v1)))
            }
        }
    } else {
        if f35(v2.clone(), f8(Str::lit("xml"))) {
            Ok(Ok::<(Bytes, Bytes), Bytes>((f143(), f148(v1)?)))
        } else {
            let m590 = f147(v0, v2.clone());
            match m590 {
                None => {
                    Ok(Err::<(Bytes, Bytes), Bytes>(f7(List::cons(f8(Str::lit("the prefix ")), List::cons(v2, List::cons(f8(Str::lit(" is not declared (Namespaces in XML 1.0, section 5, NSC: Prefix Declared)")), List::<Bytes>::nil()))))))
                }
                Some(v4) => {
                    Ok(Ok::<(Bytes, Bytes), Bytes>((v4, f148(v1)?)))
                }
            }
        }
    }
}

pub fn f90(v0: (Bytes, Bytes), v1: Str) -> bool {
    if f35({
        let (h591, _) = v0;
        h591
    }, f149()) {
        f35({
            let (_, h592) = v0;
            h592
        }, {
            let a593 = v1;
            utf8_encode(a593)
        })
    } else {
        false
    }
}

pub fn f91(p597: Adt16, v1: Bytes) -> Option<Bytes> {
    let m596 = p597;
    {
        let mut m596 = m596;
        loop {
            let step = {
                let v0 = m596;
                let m594 = v0;
                match m594 {
                    Adt16::C0(_, _, _) => {
                        Err::<Adt16, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt16::C1(v2, v3, r595) => {
                        let v4 = (*r595).clone();
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
                Ok(next) => { m596 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f92(v0: Adt16, v1: List<(Bytes, Bytes)>, v2: Bytes) -> R<Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>> {
    let m598 = v0;
    match m598 {
        Adt16::C0(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt16::C1(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt16::C2(_) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt16::C3(r609, r610) => {
            let v3 = (*r609).clone();
            let v4 = (*r610).clone();
            let m599 = f92(v4, v1.clone(), v2.clone())?;
            match m599 {
                Ok(v6) => {
                    let m600 = v3;
                    match m600 {
                        Adt16::C0(v7, r607, r608) => {
                            let v8 = (*r607).clone();
                            let v9 = (*r608).clone();
                            let m601 = f88(v8.clone(), v1);
                            match m601 {
                                Ok(v11) => {
                                    let m602 = f89(v11.clone(), v7)?;
                                    match m602 {
                                        Ok(v13) => {
                                            if f90(v13, Str::lit("simpleType")) {
                                                let m603 = f91(v8, f8(Str::lit("name")));
                                                match m603 {
                                                    None => {
                                                        Ok(f150(f8(Str::lit("a top-level simpleType must have a name (XSD 1.1 Part 2, section 4.1.2)"))))
                                                    }
                                                    Some(v14) => {
                                                        let m604 = f151(v14.clone(), v9, v11, v2)?;
                                                        match m604 {
                                                            Ok(v16) => {
                                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::cons((f152(v14)?, (match v16.clone() {
                                                                    Adt20::C0(_, h605) => {
                                                                        h605
                                                                    }
                                                                }, match v16 {
                                                                    Adt20::C0(h606, _) => {
                                                                        h606
                                                                    }
                                                                })), v6)))
                                                            }
                                                            Err(v15) => {
                                                                Ok(f150(v15))
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                                            }
                                        }
                                        Err(v12) => {
                                            Ok(f150(v12))
                                        }
                                    }
                                }
                                Err(v10) => {
                                    Ok(f150(v10))
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
                    Ok(f150(v5))
                }
            }
        }
        Adt16::C4 => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
    }
}

pub fn f93(v0: List<(Bytes, (List<Bytes>, Bytes))>) -> List<Bytes> {
    let m611 = v0;
    match m611.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h612, _) = v1;
                h612
            }, f93(v2))
        }
    }
}

pub fn f94(p615: List<Bytes>) -> Option<Bytes> {
    let m614 = p615;
    {
        let mut m614 = m614;
        loop {
            let step = {
                let v0 = m614;
                let m613 = v0;
                match m613.uncons() {
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
                Ok(next) => { m614 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f95(p621: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<Bytes> {
    let m620 = p621;
    {
        let mut m620 = m620;
        loop {
            let step = {
                let v0 = m620;
                let m616 = v0;
                match m616.uncons() {
                    None => {
                        Err::<List<(Bytes, (List<Bytes>, Bytes))>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        let m619 = f153({
                            let (h617, _) = {
                                let (_, h618) = v2;
                                h618
                            };
                            h617
                        }, v1.clone());
                        match m619 {
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
                Ok(next) => { m620 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f96(p632: u64, p633: List<(Bytes, (List<Bytes>, Bytes))>, p634: List<Bytes>, p635: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m631 = (p632, (p633, (p634, p635)));
    {
        let mut m631 = m631;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m631;
                let m622 = v0;
                if m622 == 0 {
                    let m623 = v1;
                    match m623.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(f154(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                        }
                    }
                } else {
                    let v6 = m622 - 1;
                    let m624 = v1.clone();
                    match m624.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            let m625 = f120(v1, v2.clone());
                            match m625 {
                                None => {
                                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(f154(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                                }
                                Some(v9) => {
                                    Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>((v6, ({
                                        let (_, h626) = v9.clone();
                                        h626
                                    }, (List::cons({
                                        let (h627, _) = {
                                            let (h628, _) = v9.clone();
                                            h628
                                        };
                                        h627
                                    }, v2), List::cons({
                                        let (_, h629) = {
                                            let (h630, _) = v9;
                                            h630
                                        };
                                        h629
                                    }, v3)))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m631 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f97(v0: Bytes) -> R<u64> {
    let (h636, _) = f128(Fn4::F155(v0.clone()), {
        let a638 = {
            let a637 = v0;
            length_bytes(a637)
        };
        let a639 = 1u64;
        nat_add(a638, a639)?
    }, 0u64)?;
    Ok(h636)
}

pub fn f98(v0: Bytes, v1: u64) -> R<u64> {
    let (h640, _) = f128(Fn4::F156(v0.clone()), {
        let a642 = {
            let a641 = v0;
            length_bytes(a641)
        };
        let a643 = 1u64;
        nat_add(a642, a643)?
    }, v1)?;
    Ok(h640)
}

pub fn f99(v0: Bytes, v1: u64) -> R<bool> {
    if if if f84(v0.clone(), v1, Str::lit("---")) {
        true
    } else {
        f84(v0.clone(), v1, Str::lit("..."))
    } {
        if f157(f100(v0.clone(), {
            let a644 = v1;
            let a645 = 3u64;
            nat_add(a644, a645)?
        })?) {
            true
        } else {
            f158(f100(v0.clone(), {
                let a646 = v1;
                let a647 = 3u64;
                nat_add(a646, a647)?
            })?)
        }
    } else {
        false
    } {
        let a648 = f159(v0, v1)?;
        let a649 = 0u64;
        Ok(nat_eq(a648, a649))
    } else {
        Ok(false)
    }
}

pub fn f100(v0: Bytes, v1: u64) -> R<u64> {
    f137(v0, v1)
}

pub fn f101(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: List<Bytes>) -> R<Adt18> {
    let m650 = v3;
    if m650 == 0 {
        Ok(Adt18::C1(f81(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v8 = m650 - 1;
        let a651 = v4;
        let a652 = 0u64;
        if nat_eq(a651, a652) {
            let v9: u64 = f160(v0.clone(), v1)?;
            let v10: u64 = {
                let a653 = v9;
                let a654 = v1;
                nat_sub(a653, a654)
            };
            let a655 = f100(v0.clone(), v9)?;
            let a656 = 9u64;
            if nat_eq(a655, a656) {
                let v11: u64 = f161(v0.clone(), v9)?;
                if if f162(v0.clone(), v11)? {
                    true
                } else {
                    f163(v0.clone(), v11)?
                } {
                    Ok(Adt18::C1(f81(Str::lit("a tab cannot indent a block node (YAML 1.2.2 [63])"), v11)?))
                } else {
                    let m657 = f164(v0.clone(), v11)?;
                    match m657 {
                        None => {
                            f165(v0, v11, v2)
                        }
                        Some(v12) => {
                            Ok(Adt18::C1(v12))
                        }
                    }
                }
            } else {
                if f162(v0.clone(), v9)? {
                    Ok(f166(true, f101(v0, v9, v2, v8, 2u64, false, v10, List::<Bytes>::nil())?))
                } else {
                    let m658 = f164(v0.clone(), v9)?;
                    match m658 {
                        None => {
                            if f163(v0.clone(), v9)? {
                                Ok(f166(false, f101(v0, v9, v2, v8, 3u64, false, v10, List::<Bytes>::nil())?))
                            } else {
                                f165(v0, v9, v2)
                            }
                        }
                        Some(v13) => {
                            Ok(Adt18::C1(v13))
                        }
                    }
                }
            }
        } else {
            let a659 = v4;
            let a660 = 1u64;
            if nat_eq(a659, a660) {
                let v14: u64 = f161(v0.clone(), v1)?;
                let v15: u64 = f100(v0.clone(), v14)?;
                if if f158(v15) {
                    true
                } else {
                    let a661 = v15;
                    let a662 = 35u64;
                    if nat_eq(a661, a662) {
                        let a663 = v1;
                        let a664 = v14;
                        nat_lt(a663, a664)
                    } else {
                        false
                    }
                } {
                    let v16: u64 = f98(v0.clone(), f167(v0.clone(), v14)?)?;
                    let a666 = {
                        let a665 = v0.clone();
                        length_bytes(a665)
                    };
                    let a667 = v16;
                    if if nat_le(a666, a667) {
                        true
                    } else {
                        f99(v0.clone(), v16)?
                    } {
                        Ok(Adt18::C1(f168(v14)?))
                    } else {
                        let v17: u64 = {
                            let a668 = f160(v0.clone(), v16)?;
                            let a669 = v16;
                            nat_sub(a668, a669)
                        };
                        let a670 = v2;
                        let a671 = v17;
                        if nat_le(a670, a671) {
                            f101(v0, v16, v2, v8, 0u64, false, 0u64, List::<Bytes>::nil())
                        } else {
                            if if if f169(v5) {
                                let a672 = v17;
                                let a673 = v6;
                                nat_eq(a672, a673)
                            } else {
                                false
                            } {
                                f162(v0.clone(), {
                                    let a674 = v16;
                                    let a675 = v17;
                                    nat_add(a674, a675)?
                                })?
                            } else {
                                false
                            } {
                                Ok(f166(true, f101(v0, {
                                    let a676 = v16;
                                    let a677 = v17;
                                    nat_add(a676, a677)?
                                }, v2, v8, 2u64, false, v17, List::<Bytes>::nil())?))
                            } else {
                                Ok(Adt18::C1(f168(v14)?))
                            }
                        }
                    }
                } else {
                    if if v5 {
                        f162(v0.clone(), v14)?
                    } else {
                        false
                    } {
                        Ok(f166(true, f101(v0.clone(), v14, v2, v8, 2u64, false, f159(v0, v14)?, List::<Bytes>::nil())?))
                    } else {
                        let m678 = f164(v0.clone(), v14)?;
                        match m678 {
                            None => {
                                if if v5 {
                                    f163(v0.clone(), v14)?
                                } else {
                                    false
                                } {
                                    Ok(f166(false, f101(v0.clone(), v14, v2, v8, 3u64, false, f159(v0, v14)?, List::<Bytes>::nil())?))
                                } else {
                                    f165(v0, v14, v2)
                                }
                            }
                            Some(v18) => {
                                Ok(Adt18::C1(v18))
                            }
                        }
                    }
                }
            } else {
                let a679 = v4;
                let a680 = 2u64;
                if nat_eq(a679, a680) {
                    let m685 = f101(v0.clone(), {
                        let a681 = v1;
                        let a682 = 1u64;
                        nat_add(a681, a682)?
                    }, {
                        let a683 = v6;
                        let a684 = 1u64;
                        nat_add(a683, a684)?
                    }, v8, 1u64, true, v6, List::<Bytes>::nil())?;
                    match m685 {
                        Adt18::C0(v20, v21) => {
                            let v22: u64 = f98(v0.clone(), v21)?;
                            let v23: u64 = {
                                let a686 = f160(v0.clone(), v22)?;
                                let a687 = v22;
                                nat_sub(a686, a687)
                            };
                            let a689 = {
                                let a688 = v0.clone();
                                length_bytes(a688)
                            };
                            let a690 = v22;
                            if nat_le(a689, a690) {
                                Ok(Adt18::C0(Adt17::C7(std::rc::Rc::new(v20), std::rc::Rc::new(Adt17::C6)), v22))
                            } else {
                                let a691 = v23;
                                let a692 = v6;
                                if if nat_eq(a691, a692) {
                                    f162(v0.clone(), {
                                        let a693 = v22;
                                        let a694 = v23;
                                        nat_add(a693, a694)?
                                    })?
                                } else {
                                    false
                                } {
                                    let m697 = f101(v0, {
                                        let a695 = v22;
                                        let a696 = v23;
                                        nat_add(a695, a696)?
                                    }, v2, v8, 2u64, false, v6, List::<Bytes>::nil())?;
                                    match m697 {
                                        Adt18::C0(v25, v26) => {
                                            Ok(Adt18::C0(Adt17::C7(std::rc::Rc::new(v20), std::rc::Rc::new(v25)), v26))
                                        }
                                        Adt18::C1(v24) => {
                                            Ok(Adt18::C1(v24))
                                        }
                                    }
                                } else {
                                    let a698 = v6;
                                    let a699 = v23;
                                    if nat_lt(a698, a699) {
                                        Ok(Adt18::C1(f81(Str::lit("a line is indented more than its block sequence entry allows (YAML 1.2.2 [183])"), {
                                            let a700 = v22;
                                            let a701 = v23;
                                            nat_add(a700, a701)?
                                        })?))
                                    } else {
                                        Ok(Adt18::C0(Adt17::C7(std::rc::Rc::new(v20), std::rc::Rc::new(Adt17::C6)), v22))
                                    }
                                }
                            }
                        }
                        Adt18::C1(v19) => {
                            Ok(Adt18::C1(v19))
                        }
                    }
                } else {
                    let m702 = f164(v0.clone(), v1)?;
                    match m702 {
                        None => {
                            let m703 = f170(v0.clone(), v1)?;
                            match m703 {
                                None => {
                                    Ok(Adt18::C1(f81(Str::lit("a block mapping entry must be an implicit key and : (YAML 1.2.2 [188])"), v1)?))
                                }
                                Some(v28) => {
                                    let v29: Bytes = f171(v0.clone(), v1)?;
                                    if f172(v29.clone(), v7.clone()) {
                                        Ok(Adt18::C1(f81(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                    } else {
                                        let m708 = f101(v0.clone(), {
                                            let a704 = v28;
                                            let a705 = 1u64;
                                            nat_add(a704, a705)?
                                        }, {
                                            let a706 = v6;
                                            let a707 = 1u64;
                                            nat_add(a706, a707)?
                                        }, v8, 1u64, false, v6, List::<Bytes>::nil())?;
                                        match m708 {
                                            Adt18::C0(v31, v32) => {
                                                let v33: u64 = f98(v0.clone(), v32)?;
                                                let v34: u64 = {
                                                    let a709 = f160(v0.clone(), v33)?;
                                                    let a710 = v33;
                                                    nat_sub(a709, a710)
                                                };
                                                let a712 = {
                                                    let a711 = v0.clone();
                                                    length_bytes(a711)
                                                };
                                                let a713 = v33;
                                                if nat_le(a712, a713) {
                                                    Ok(Adt18::C0(Adt17::C8(v29, std::rc::Rc::new(v31), std::rc::Rc::new(Adt17::C6)), v33))
                                                } else {
                                                    let a714 = v34;
                                                    let a715 = v6;
                                                    if if if nat_eq(a714, a715) {
                                                        f169(f162(v0.clone(), {
                                                            let a716 = v33;
                                                            let a717 = v34;
                                                            nat_add(a716, a717)?
                                                        })?)
                                                    } else {
                                                        false
                                                    } {
                                                        f169(f99(v0.clone(), v33)?)
                                                    } else {
                                                        false
                                                    } {
                                                        let m720 = f101(v0, {
                                                            let a718 = v33;
                                                            let a719 = v34;
                                                            nat_add(a718, a719)?
                                                        }, v2, v8, 3u64, false, v6, List::cons(v29.clone(), v7))?;
                                                        match m720 {
                                                            Adt18::C0(v36, v37) => {
                                                                Ok(Adt18::C0(Adt17::C8(v29, std::rc::Rc::new(v31), std::rc::Rc::new(v36)), v37))
                                                            }
                                                            Adt18::C1(v35) => {
                                                                Ok(Adt18::C1(v35))
                                                            }
                                                        }
                                                    } else {
                                                        let a721 = v6;
                                                        let a722 = v34;
                                                        if nat_lt(a721, a722) {
                                                            Ok(Adt18::C1(f81(Str::lit("a line is indented more than its block mapping entry allows (YAML 1.2.2 [187])"), {
                                                                let a723 = v33;
                                                                let a724 = v34;
                                                                nat_add(a723, a724)?
                                                            })?))
                                                        } else {
                                                            Ok(Adt18::C0(Adt17::C8(v29, std::rc::Rc::new(v31), std::rc::Rc::new(Adt17::C6)), v33))
                                                        }
                                                    }
                                                }
                                            }
                                            Adt18::C1(v30) => {
                                                Ok(Adt18::C1(v30))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v27) => {
                            Ok(Adt18::C1(v27))
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
        let a725 = v0.clone();
        length_bytes(a725)
    };
    let a726 = v2;
    let a727 = v3;
    if if nat_lt(a726, a727) {
        if f84(v0.clone(), v2, Str::lit("...")) {
            f99(v0.clone(), v2)?
        } else {
            false
        }
    } else {
        false
    } {
        let m730 = f173(v0.clone(), {
            let a728 = v2;
            let a729 = 3u64;
            nat_add(a728, a729)?
        })?;
        match m730 {
            Ok(v5) => {
                let v6: u64 = f98(v0, v5)?;
                let a731 = v6;
                let a732 = v3;
                if nat_lt(a731, a732) {
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
        let a733 = v2;
        let a734 = v3;
        if if nat_lt(a733, a734) {
            f99(v0, v2)?
        } else {
            false
        } {
            Ok(Err::<u64, Bytes>(f81(Str::lit("a stream of more than one document is not read (YAML 1.2.2 [211])"), v2)?))
        } else {
            let a735 = v2;
            let a736 = v3;
            if nat_lt(a735, a736) {
                Ok(Err::<u64, Bytes>(f81(Str::lit("only comments may follow the root node (YAML 1.2.2 [207])"), v2)?))
            } else {
                Ok(Ok::<u64, Bytes>(v2))
            }
        }
    }
}

pub fn f103(v0: Adt17) -> R<Bytes> {
    let m737 = v0;
    match m737 {
        Adt17::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt17::C1(v2, v3) => {
            f174(v2, v3)
        }
        Adt17::C2(v4, v5, v6) => {
            f175(v4, v5, v6)
        }
        Adt17::C3(v7) => {
            f176(v7)
        }
        Adt17::C4(r738) => {
            let v8 = (*r738).clone();
            Ok(f177(f178(v8)?))
        }
        Adt17::C5(r739) => {
            let v9 = (*r739).clone();
            f179(f180(v9)?)
        }
        Adt17::C6 => {
            Ok(Bytes::lit(&[]))
        }
        Adt17::C7(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt17::C8(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f104(v0: Bytes, v1: u64) -> Option<u8> {
    let a740 = v0;
    let a741 = v1;
    index_bytes(a740, a741)
}

pub fn f105(v0: Bytes) -> R<Result<List<Adt19>, Bytes>> {
    f181({
        let a743 = {
            let a742 = v0.clone();
            length_bytes(a742)
        };
        let a744 = 1u64;
        nat_add(a743, a744)?
    }, v0, 0u64, List::<Adt19>::nil())
}

pub fn f106(p752: u64, p753: List<Adt19>, p754: List<Adt12>) -> R<Result<List<Adt12>, Bytes>> {
    let m751 = (p752, (p753, p754));
    {
        let mut m751 = m751;
        loop {
            let step = {
                let (v0, (v1, v2)) = m751;
                let m745 = v0;
                if m745 == 0 {
                    Err::<(u64, (List<Adt19>, List<Adt12>)), R<Result<List<Adt12>, Bytes>>>(Ok(Err::<List<Adt12>, Bytes>(f8(Str::lit("the input has more definitions than tokens")))))
                } else {
                    let v3 = m745 - 1;
                    let m746 = v1.clone();
                    match m746.uncons() {
                        None => {
                            Err::<(u64, (List<Adt19>, List<Adt12>)), R<Result<List<Adt12>, Bytes>>>(Ok(Ok::<List<Adt12>, Bytes>(f182(v2, List::<Adt12>::nil()))))
                        }
                        Some((_, _)) => {
                            let m747 = f183(v1)?;
                            match m747 {
                                Ok(v7) => {
                                    let m748 = f184(v3, v7)?;
                                    match m748 {
                                        Ok(v9) => {
                                            Ok::<(u64, (List<Adt19>, List<Adt12>)), R<Result<List<Adt12>, Bytes>>>((v3, ({
                                                let (_, h749) = v9;
                                                h749
                                            }, List::cons({
                                                let (h750, _) = v9;
                                                h750
                                            }, v2))))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (List<Adt19>, List<Adt12>)), R<Result<List<Adt12>, Bytes>>>(Ok(Err::<List<Adt12>, Bytes>(v8)))
                                        }
                                    }
                                }
                                Err(v6) => {
                                    Err::<(u64, (List<Adt19>, List<Adt12>)), R<Result<List<Adt12>, Bytes>>>(Ok(Err::<List<Adt12>, Bytes>(v6)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m751 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f107(v0: List<Adt15>, v1: List<Adt15>) -> List<Adt15> {
    let m755 = v0;
    match m755.uncons() {
        None => {
            List::<Adt15>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = if f35(f115(v2.clone()), Bytes::lit(&[])) {
                f185(f117(v2.clone()), v1.clone())
            } else {
                f115(v2.clone())
            };
            List::cons(Adt15::C0(f118(v2.clone()), v4, f119(v2.clone()), f117(v2)), f107(v3, v1))
        }
    }
}

pub fn f108(v0: List<Adt15>) -> R<u64> {
    let m756 = v0;
    match m756.uncons() {
        None => {
            Ok(0u64)
        }
        Some((v1, v2)) => {
            if f35(f115(v1), Bytes::lit(&[])) {
                f108(v2)
            } else {
                let a757 = f108(v2)?;
                let a758 = 1u64;
                nat_add(a757, a758)
            }
        }
    }
}

pub fn f109(v0: u64) -> bool {
    let a759 = v0;
    let a760 = 0u64;
    if nat_eq(a759, a760) {
        true
    } else {
        let a761 = v0;
        let a762 = 2u64;
        if nat_eq(a761, a762) {
            true
        } else {
            let a763 = v0;
            let a764 = 3u64;
            if nat_eq(a763, a764) {
                true
            } else {
                let a765 = v0;
                let a766 = 4u64;
                if nat_eq(a765, a766) {
                    true
                } else {
                    let a767 = v0;
                    let a768 = 9u64;
                    nat_eq(a767, a768)
                }
            }
        }
    }
}

pub fn f110(v0: Adt12) -> u64 {
    let m769 = v0;
    match m769 {
        Adt12::C0(v1, _, _, _, _, _) => {
            v1
        }
    }
}

pub fn f111(v0: List<Adt12>, v1: Adt12) -> R<Result<(Bytes, List<Bytes>), Bytes>> {
    let v2: u64 = f110(v1.clone());
    let v3: Bytes = f152(f112(v1.clone()))?;
    if if f186(v3.clone()) {
        f109(v2)
    } else {
        false
    } {
        Ok(Err::<(Bytes, List<Bytes>), Bytes>(f187(v3)))
    } else {
        let a770 = v2;
        let a771 = 0u64;
        if if nat_eq(a770, a771) {
            true
        } else {
            let a772 = v2;
            let a773 = 2u64;
            nat_eq(a772, a773)
        } {
            let m774 = f188(v0.clone(), v0.clone(), f112(v1.clone()));
            match m774 {
                None => {
                    let m778 = f189({
                        let a776 = {
                            let a775 = v0.clone();
                            length_list(a775)
                        };
                        let a777 = 1u64;
                        nat_add(a776, a777)?
                    }, v0, v1)?;
                    match m778 {
                        Ok(v5) => {
                            let a780 = {
                                let a779 = v5.clone();
                                length_list(a779)
                            };
                            let a781 = 0u64;
                            if nat_eq(a780, a781) {
                                Ok(Err::<(Bytes, List<Bytes>), Bytes>(f8(Str::lit("it has no data members, and LexLean refuses a structure with no fields"))))
                            } else {
                                let m782 = f190(f191(f192(v5.clone()))?, v5)?;
                                match m782 {
                                    Ok(v7) => {
                                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f193(v3, {
                                            let (h783, _) = v7;
                                            h783
                                        }), {
                                            let (_, h784) = v7;
                                            h784
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
            let a785 = v2;
            let a786 = 3u64;
            if nat_eq(a785, a786) {
                let m787 = f94(f194(v1.clone()));
                match m787 {
                    None => {
                        let m788 = f195(f191(f194(v1.clone()))?, f194(v1.clone()))?;
                        match m788 {
                            None => {
                                let m789 = f196(f191(f194(v1.clone()))?, f197(f194(v1)), List::<Bytes>::nil())?;
                                match m789 {
                                    Ok(v10) => {
                                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f198(v3, v10), List::<Bytes>::nil())))
                                    }
                                    Err(v9) => {
                                        Ok(Err::<(Bytes, List<Bytes>), Bytes>(v9))
                                    }
                                }
                            }
                            Some(v11) => {
                                Ok(Err::<(Bytes, List<Bytes>), Bytes>(f187(v11)))
                            }
                        }
                    }
                    Some(v12) => {
                        Ok(Err::<(Bytes, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("the enumeration value \"")), List::cons(v12, List::cons(f8(Str::lit("\" appears twice (Web IDL 2.9: the values must be distinct)")), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                let a790 = v2;
                let a791 = 4u64;
                if nat_eq(a790, a791) {
                    let v13: Adt14 = f199(v1);
                    if !f35(f200(v13.clone()), Bytes::lit(&[])) {
                        Ok(Err::<(Bytes, List<Bytes>), Bytes>(f200(v13)))
                    } else {
                        let a792 = f201(v13.clone());
                        let a793 = 1u64;
                        if nat_eq(a792, a793) {
                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f198(v3, List::cons(f202(v13.clone()), List::<Bytes>::nil())), f203(v13))))
                        } else {
                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f204(v3, f202(v13.clone())), f203(v13))))
                        }
                    }
                } else {
                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f7(f194(v1))))
                }
            }
        }
    }
}

pub fn f112(v0: Adt12) -> Bytes {
    let m794 = v0;
    match m794 {
        Adt12::C0(_, v2, _, _, _, _) => {
            v2
        }
    }
}

pub fn f113(v0: Bytes, p797: List<Bytes>) -> bool {
    let m796 = p797;
    {
        let mut m796 = m796;
        loop {
            let step = {
                let v1 = m796;
                let m795 = v1;
                match m795.uncons() {
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
                Ok(next) => { m796 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f114(p800: List<Adt15>, v1: Bytes) -> Option<Adt15> {
    let m799 = p800;
    {
        let mut m799 = m799;
        loop {
            let step = {
                let v0 = m799;
                let m798 = v0;
                match m798.uncons() {
                    None => {
                        Err::<List<Adt15>, Option<Adt15>>(None::<Adt15>)
                    }
                    Some((v2, v3)) => {
                        if f35(f118(v2.clone()), v1.clone()) {
                            Err::<List<Adt15>, Option<Adt15>>(Some(v2))
                        } else {
                            Ok::<List<Adt15>, Option<Adt15>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m799 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f115(v0: Adt15) -> Bytes {
    let m801 = v0;
    match m801 {
        Adt15::C0(_, v2, _, _) => {
            v2
        }
    }
}

pub fn f116(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m802 = v0;
    match m802.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f116(v3, v1))
        }
    }
}

pub fn f117(v0: Adt15) -> List<Bytes> {
    let m803 = v0;
    match m803 {
        Adt15::C0(_, _, _, v4) => {
            v4
        }
    }
}

pub fn f118(v0: Adt15) -> Bytes {
    let m804 = v0;
    match m804 {
        Adt15::C0(v1, _, _, _) => {
            v1
        }
    }
}

pub fn f119(v0: Adt15) -> Bytes {
    let m805 = v0;
    match m805 {
        Adt15::C0(_, _, v3, _) => {
            v3
        }
    }
}

pub fn f120(v0: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)> {
    let m806 = v0;
    match m806.uncons() {
        None => {
            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
        }
        Some((v2, v3)) => {
            let m809 = f153({
                let (h807, _) = {
                    let (_, h808) = v2.clone();
                    h808
                };
                h807
            }, v1.clone());
            match m809 {
                None => {
                    Some((({
                        let (h810, _) = v2;
                        h810
                    }, {
                        let (_, h811) = {
                            let (_, h812) = v2;
                            h812
                        };
                        h811
                    }), v3))
                }
                Some(_) => {
                    let m813 = f120(v3, v1);
                    match m813 {
                        None => {
                            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h814, _) = v5;
                                h814
                            }, List::cons(v2, {
                                let (_, h815) = v5;
                                h815
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f121(p820: List<Adt12>, v1: u64, v2: Bytes) -> Option<Adt12> {
    let m819 = p820;
    {
        let mut m819 = m819;
        loop {
            let step = {
                let v0 = m819;
                let m816 = v0;
                match m816.uncons() {
                    None => {
                        Err::<List<Adt12>, Option<Adt12>>(None::<Adt12>)
                    }
                    Some((v3, v4)) => {
                        let a817 = f110(v3.clone());
                        let a818 = v1;
                        if if nat_eq(a817, a818) {
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
                Ok(next) => { m819 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f122(v0: u64) -> Bytes {
    let a821 = v0;
    let a822 = 5u64;
    if nat_eq(a821, a822) {
        f8(Str::lit("partial interface "))
    } else {
        let a823 = v0;
        let a824 = 7u64;
        if nat_eq(a823, a824) {
            f8(Str::lit("partial dictionary "))
        } else {
            f8(Str::lit(""))
        }
    }
}

pub fn f123(v0: Adt12) -> Bytes {
    let m825 = v0;
    match m825 {
        Adt12::C0(_, _, v3, _, _, _) => {
            v3
        }
    }
}

pub fn f124(v0: Bytes, v1: u64) -> Bytes {
    let m829 = {
        let a826 = v0;
        let a827 = v1;
        let a828 = 1u64;
        slice_bytes(a826, a827, a828)
    };
    match m829 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f125(v0: Fn3, p834: u64, p835: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m833 = (p834, p835);
    {
        let mut m833 = m833;
        loop {
            let step = {
                let (v1, v2) = m833;
                let m830 = v1;
                if m830 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m830 - 1;
                    let m832 = {
                        let c831: Fn3 = v0.clone();
                        c831.apply(v2.clone())?
                    };
                    match m832 {
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
                Ok(next) => { m833 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f126(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f205(v1, v0, v2)
}

pub fn f127(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f206(v0, v1)
}

pub fn f128(v0: Fn4, p840: u64, p841: u64) -> R<(u64, bool)> {
    let m839 = (p840, p841);
    {
        let mut m839 = m839;
        loop {
            let step = {
                let (v1, v2) = m839;
                let m836 = v1;
                if m836 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m836 - 1;
                    let m838 = {
                        let c837: Fn4 = v0.clone();
                        c837.apply(v2)?
                    };
                    match m838 {
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
                Ok(next) => { m839 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f129(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f207(v0, v1)
}

pub fn f130(v0: u64) -> R<bool> {
    f208(v0, f209())
}

pub fn f131(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let v3: u64 = {
        let (h842, _) = f128(Fn4::F210(v1.clone(), v0.clone()), {
            let a844 = {
                let a843 = v0.clone();
                length_bytes(a843)
            };
            let a845 = 1u64;
            nat_add(a844, a845)?
        }, v2)?;
        h842
    };
    if f77(v0, v1, v3) {
        Ok(Some(v3))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f132(v0: Bytes) -> Result<u64, Bytes> {
    Err::<u64, Bytes>(v0)
}

pub fn f133(v0: Bytes, v1: u64) -> R<u64> {
    let (h846, _) = f128(Fn4::F211(v0.clone()), {
        let a848 = {
            let a847 = v0;
            length_bytes(a847)
        };
        let a849 = 1u64;
        nat_add(a848, a849)?
    }, v1)?;
    Ok(h846)
}

pub fn f134(v0: Bytes) -> bool {
    f172(v0, List::cons(f8(Str::lit("UTF-8")), List::cons(f8(Str::lit("UTf-8")), List::cons(f8(Str::lit("UtF-8")), List::cons(f8(Str::lit("Utf-8")), List::cons(f8(Str::lit("uTF-8")), List::cons(f8(Str::lit("uTf-8")), List::cons(f8(Str::lit("utF-8")), List::cons(f8(Str::lit("utf-8")), List::<Bytes>::nil())))))))))
}

pub fn f135(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let m853 = f131(v0.clone(), {
        let a850 = Str::lit("--");
        utf8_encode(a850)
    }, {
        let a851 = v1;
        let a852 = 4u64;
        nat_add(a851, a852)?
    })?;
    match m853 {
        None => {
            Ok(f132(f81(Str::lit("a comment is not closed (XML 1.0 [15])"), v1)?))
        }
        Some(v2) => {
            let a856 = f85(v0, {
                let a854 = v2;
                let a855 = 2u64;
                nat_add(a854, a855)?
            })?;
            let a857 = 62u64;
            if nat_eq(a856, a857) {
                Ok(Ok::<u64, Bytes>({
                    let a858 = v2;
                    let a859 = 3u64;
                    nat_add(a858, a859)?
                }))
            } else {
                Ok(f132(f81(Str::lit("-- is not allowed inside a comment (XML 1.0 [15])"), v2)?))
            }
        }
    }
}

pub fn f136(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f139(v0.clone(), {
        let a860 = v1;
        let a861 = 2u64;
        nat_add(a860, a861)?
    })?;
    let a864 = v2;
    let a865 = {
        let a862 = v1;
        let a863 = 2u64;
        nat_add(a862, a863)?
    };
    if nat_eq(a864, a865) {
        Ok(f132(f81(Str::lit("a processing instruction must start with a Name (XML 1.0 [16])"), v1)?))
    } else {
        let a868 = v2;
        let a869 = {
            let a866 = v1;
            let a867 = 5u64;
            nat_add(a866, a867)?
        };
        if if nat_eq(a868, a869) {
            f172(f32(v0.clone(), {
                let a870 = v1;
                let a871 = 2u64;
                nat_add(a870, a871)?
            }, 3u64), List::cons(f8(Str::lit("XML")), List::cons(f8(Str::lit("XMl")), List::cons(f8(Str::lit("XmL")), List::cons(f8(Str::lit("Xml")), List::cons(f8(Str::lit("xML")), List::cons(f8(Str::lit("xMl")), List::cons(f8(Str::lit("xmL")), List::cons(f8(Str::lit("xml")), List::<Bytes>::nil())))))))))
        } else {
            false
        } {
            Ok(f132(f81(Str::lit("the target xml is reserved; an XML declaration may appear only at the very start (XML 1.0 [17], [22])"), v1)?))
        } else {
            let m873 = f131(v0, {
                let a872 = Str::lit("?>");
                utf8_encode(a872)
            }, v2)?;
            match m873 {
                None => {
                    Ok(f132(f81(Str::lit("a processing instruction is not closed (XML 1.0 [16])"), v1)?))
                }
                Some(v3) => {
                    Ok(Ok::<u64, Bytes>({
                        let a874 = v3;
                        let a875 = 2u64;
                        nat_add(a874, a875)?
                    }))
                }
            }
        }
    }
}

pub fn f137(v0: Bytes, v1: u64) -> R<u64> {
    let m878 = {
        let a876 = v0;
        let a877 = v1;
        index_bytes(a876, a877)
    };
    match m878 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a923 = {
                let a899 = {
                    let a887 = {
                        let a881 = {
                            let a879 = v3;
                            let a880 = 128u8;
                            fixed_u8::bit_and(a879, a880)
                        };
                        let a882 = 128u8;
                        if equal(a881, a882) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a888 = {
                        let a885 = {
                            let a883 = v3;
                            let a884 = 64u8;
                            fixed_u8::bit_and(a883, a884)
                        };
                        let a886 = 64u8;
                        if equal(a885, a886) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a887, a888)?
                };
                let a900 = {
                    let a897 = {
                        let a891 = {
                            let a889 = v3;
                            let a890 = 32u8;
                            fixed_u8::bit_and(a889, a890)
                        };
                        let a892 = 32u8;
                        if equal(a891, a892) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a898 = {
                        let a895 = {
                            let a893 = v3;
                            let a894 = 16u8;
                            fixed_u8::bit_and(a893, a894)
                        };
                        let a896 = 16u8;
                        if equal(a895, a896) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a897, a898)?
                };
                nat_add(a899, a900)?
            };
            let a924 = {
                let a921 = {
                    let a909 = {
                        let a903 = {
                            let a901 = v3;
                            let a902 = 8u8;
                            fixed_u8::bit_and(a901, a902)
                        };
                        let a904 = 8u8;
                        if equal(a903, a904) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a910 = {
                        let a907 = {
                            let a905 = v3;
                            let a906 = 4u8;
                            fixed_u8::bit_and(a905, a906)
                        };
                        let a908 = 4u8;
                        if equal(a907, a908) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a909, a910)?
                };
                let a922 = {
                    let a919 = {
                        let a913 = {
                            let a911 = v3;
                            let a912 = 2u8;
                            fixed_u8::bit_and(a911, a912)
                        };
                        let a914 = 2u8;
                        if equal(a913, a914) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a920 = {
                        let a917 = {
                            let a915 = v3;
                            let a916 = 1u8;
                            fixed_u8::bit_and(a915, a916)
                        };
                        let a918 = 1u8;
                        if equal(a917, a918) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a919, a920)?
                };
                nat_add(a921, a922)?
            };
            nat_add(a923, a924)
        }
    }
}

pub fn f138(v0: Bytes) -> Result<(Adt16, u64), Bytes> {
    Err::<(Adt16, u64), Bytes>(v0)
}

pub fn f139(v0: Bytes, v1: u64) -> R<u64> {
    let (h925, _) = f128(Fn4::F212(v0.clone(), v1), {
        let a927 = {
            let a926 = v0;
            length_bytes(a926)
        };
        let a928 = 1u64;
        nat_add(a927, a928)?
    }, v1)?;
    Ok(h925)
}

pub fn f140(v0: Bytes, v1: u64, v2: u64, v3: List<Bytes>) -> R<Result<(Adt16, u64), Bytes>> {
    let m929 = v2;
    if m929 == 0 {
        Ok(f138(f81(Str::lit("a tag has more attributes than the input has bytes"), v1)?))
    } else {
        let v4 = m929 - 1;
        let v5: u64 = f133(v0.clone(), v1)?;
        let a930 = f85(v0.clone(), v5)?;
        let a931 = 62u64;
        if if nat_eq(a930, a931) {
            true
        } else {
            f84(v0.clone(), v5, Str::lit("/>"))
        } {
            Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C4, v5)))
        } else {
            let a932 = v5;
            let a933 = v1;
            if nat_eq(a932, a933) {
                Ok(f138(f81(Str::lit("attributes must be separated by white space (XML 1.0 [40])"), v1)?))
            } else {
                let v6: u64 = f139(v0.clone(), v5)?;
                let a934 = v6;
                let a935 = v5;
                if nat_eq(a934, a935) {
                    Ok(f138(f81(Str::lit("an attribute must start with a Name (XML 1.0 [41])"), v5)?))
                } else {
                    let v7: Bytes = f32(v0.clone(), v5, {
                        let a936 = v6;
                        let a937 = v5;
                        nat_sub(a936, a937)
                    });
                    if f172(v7.clone(), v3.clone()) {
                        Ok(f138(f81(Str::lit("an attribute name appears twice in one tag (XML 1.0 WFC: Unique Att Spec)"), v5)?))
                    } else {
                        let v8: u64 = f133(v0.clone(), v6)?;
                        let a938 = f85(v0.clone(), v8)?;
                        let a939 = 61u64;
                        if nat_eq(a938, a939) {
                            let v9: u64 = f133(v0.clone(), {
                                let a940 = v8;
                                let a941 = 1u64;
                                nat_add(a940, a941)?
                            })?;
                            let v10: u64 = f85(v0.clone(), v9)?;
                            let a942 = v10;
                            let a943 = 34u64;
                            if if nat_eq(a942, a943) {
                                true
                            } else {
                                let a944 = v10;
                                let a945 = 39u64;
                                nat_eq(a944, a945)
                            } {
                                let m948 = f141(v0.clone(), {
                                    let a946 = v9;
                                    let a947 = 1u64;
                                    nat_add(a946, a947)?
                                }, v4, Bytes::lit(&[]), true, v10)?;
                                match m948 {
                                    Ok(v12) => {
                                        let m950 = f140(v0, {
                                            let (_, h949) = v12;
                                            h949
                                        }, v4, List::cons(v7.clone(), v3))?;
                                        match m950 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C1(v7, {
                                                    let (h951, _) = v12;
                                                    h951
                                                }, std::rc::Rc::new({
                                                    let (h952, _) = v14;
                                                    h952
                                                })), {
                                                    let (_, h953) = v14;
                                                    h953
                                                })))
                                            }
                                            Err(v13) => {
                                                Ok(f138(v13))
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(f138(v11))
                                    }
                                }
                            } else {
                                Ok(f138(f81(Str::lit("an attribute value must be quoted (XML 1.0 [10])"), v9)?))
                            }
                        } else {
                            Ok(f138(f81(Str::lit("an attribute needs = and a value (XML 1.0 [25], [41])"), v8)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f141(v0: Bytes, p993: u64, p994: u64, p995: Bytes, v4: bool, v5: u64) -> R<Result<(Bytes, u64), Bytes>> {
    let m992 = (p993, (p994, p995));
    {
        let mut m992 = m992;
        loop {
            let step = {
                let (v1, (v2, v3)) = m992;
                let m954 = v2;
                if m954 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f213(f81(Str::lit("the input ends inside a text run"), v1)?)))
                } else {
                    let v6 = m954 - 1;
                    let v7: u64 = f85(v0.clone(), v1)?;
                    if if v4 {
                        let a955 = v7;
                        let a956 = v5;
                        nat_eq(a955, a956)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, {
                            let a957 = v1;
                            let a958 = 1u64;
                            nat_add(a957, a958)?
                        }))))
                    } else {
                        let a959 = v7;
                        let a960 = 256u64;
                        if nat_eq(a959, a960) {
                            if v4 {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f213(f81(Str::lit("an attribute value is not closed (XML 1.0 [10])"), v1)?)))
                            } else {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                            }
                        } else {
                            let a961 = v7;
                            let a962 = 60u64;
                            if nat_eq(a961, a962) {
                                if v4 {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f213(f81(Str::lit("< is not allowed in an attribute value (XML 1.0 [10], WFC: No < in Attribute Values)"), v1)?)))
                                } else {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                                }
                            } else {
                                let a963 = v7;
                                let a964 = 38u64;
                                if nat_eq(a963, a964) {
                                    let m965 = f214(v0.clone(), v1)?;
                                    match m965 {
                                        Ok(v9) => {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let (_, h966) = v9;
                                                h966
                                            }, (v6, {
                                                let a968 = v3;
                                                let a969 = {
                                                    let (h967, _) = v9;
                                                    h967
                                                };
                                                append_bytes(a968, a969)
                                            })))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f213(v8)))
                                        }
                                    }
                                } else {
                                    if v4 {
                                        let a970 = v7;
                                        let a971 = 9u64;
                                        if if nat_eq(a970, a971) {
                                            true
                                        } else {
                                            let a972 = v7;
                                            let a973 = 10u64;
                                            nat_eq(a972, a973)
                                        } {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let a974 = v1;
                                                let a975 = 1u64;
                                                nat_add(a974, a975)?
                                            }, (v6, {
                                                let a976 = v3;
                                                let a977 = f8(Str::lit(" "));
                                                append_bytes(a976, a977)
                                            })))
                                        } else {
                                            let v10: u64 = f215(v0.clone(), true, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v10, (v6, {
                                                let a980 = v3;
                                                let a981 = f32(v0.clone(), v1, {
                                                    let a978 = v10;
                                                    let a979 = v1;
                                                    nat_sub(a978, a979)
                                                });
                                                append_bytes(a980, a981)
                                            })))
                                        }
                                    } else {
                                        let a982 = v7;
                                        let a983 = 93u64;
                                        if nat_eq(a982, a983) {
                                            if f84(v0.clone(), v1, Str::lit("]]>")) {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f213(f81(Str::lit("]]> is not allowed in character data (XML 1.0 [14])"), v1)?)))
                                            } else {
                                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                    let a984 = v1;
                                                    let a985 = 1u64;
                                                    nat_add(a984, a985)?
                                                }, (v6, {
                                                    let a986 = v3;
                                                    let a987 = f8(Str::lit("]"));
                                                    append_bytes(a986, a987)
                                                })))
                                            }
                                        } else {
                                            let v11: u64 = f215(v0.clone(), false, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v11, (v6, {
                                                let a990 = v3;
                                                let a991 = f32(v0.clone(), v1, {
                                                    let a988 = v11;
                                                    let a989 = v1;
                                                    nat_sub(a988, a989)
                                                });
                                                append_bytes(a990, a991)
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
                Ok(next) => { m992 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f142(v0: Bytes) -> Result<List<(Bytes, Bytes)>, Bytes> {
    Err::<List<(Bytes, Bytes)>, Bytes>(v0)
}

pub fn f143() -> Bytes {
    f8(Str::lit("http://www.w3.org/XML/1998/namespace"))
}

pub fn f144() -> Bytes {
    f8(Str::lit("http://www.w3.org/2000/xmlns/"))
}

pub fn f145(v0: Bytes, v1: Bytes) -> bool {
    let m1000 = {
        let a997 = v0;
        let a998 = 0u64;
        let a999 = {
            let a996 = v1.clone();
            length_bytes(a996)
        };
        slice_bytes(a997, a998, a999)
    };
    match m1000 {
        None => {
            false
        }
        Some(v2) => {
            let a1001 = v2;
            let a1002 = v1;
            equal(a1001, a1002)
        }
    }
}

pub fn f146(v0: Bytes) -> R<Bytes> {
    let m1003 = f131(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1003 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v1) => {
            Ok(f32(v0, 0u64, v1))
        }
    }
}

pub fn f147(p1008: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1007 = p1008;
    {
        let mut m1007 = m1007;
        loop {
            let step = {
                let v0 = m1007;
                let m1004 = v0;
                match m1004.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h1005, _) = v2;
                            h1005
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h1006) = v2;
                                h1006
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1007 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f148(v0: Bytes) -> R<Bytes> {
    let m1009 = f131(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1009 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(f32(v0.clone(), {
                let a1010 = v1;
                let a1011 = 1u64;
                nat_add(a1010, a1011)?
            }, {
                let a1015 = {
                    let a1012 = v0;
                    length_bytes(a1012)
                };
                let a1016 = {
                    let a1013 = v1;
                    let a1014 = 1u64;
                    nat_add(a1013, a1014)?
                };
                nat_sub(a1015, a1016)
            }))
        }
    }
}

pub fn f149() -> Bytes {
    f8(Str::lit("http://www.w3.org/2001/XMLSchema"))
}

pub fn f150(v0: Bytes) -> Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes> {
    Err::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v0)
}

pub fn f151(v0: Bytes, v1: Adt16, v2: List<(Bytes, Bytes)>, v3: Bytes) -> R<Result<Adt20, Bytes>> {
    let v4: Bytes = f152(v0.clone())?;
    let m1017 = f216(v1, v2)?;
    match m1017 {
        Ok(v6) => {
            let m1018 = v6;
            match m1018 {
                None => {
                    Ok(f217(f218(f8(Str::lit("a simpleType must have a restriction, list or union child (XSD 1.1 Part 2, section 4.1.2)")), v0)))
                }
                Some(v7) => {
                    let v8: Bytes = match v7.clone() {
                        Adt21::C0(h1019, _, _, _) => {
                            h1019
                        }
                    };
                    let v9: Adt16 = match v7.clone() {
                        Adt21::C0(_, h1020, _, _) => {
                            h1020
                        }
                    };
                    let v10: Adt16 = match v7.clone() {
                        Adt21::C0(_, _, h1021, _) => {
                            h1021
                        }
                    };
                    let v11: List<(Bytes, Bytes)> = match v7 {
                        Adt21::C0(_, _, _, h1022) => {
                            h1022
                        }
                    };
                    if f35(v8.clone(), f8(Str::lit("union"))) {
                        Ok(f217(f218(f8(Str::lit("a union is not translated, because a literal can be valid for more than one member type, so no LexLean inductive keeps its members disjoint (XSD 1.1 Part 2, section 2.4.1.3)")), v0)))
                    } else {
                        if f35(v8, f8(Str::lit("list"))) {
                            let m1023 = f91(v9, f8(Str::lit("itemType")));
                            match m1023 {
                                None => {
                                    Ok(f217(f218(f8(Str::lit("a list with an anonymous item type is not translated yet")), v0)))
                                }
                                Some(v12) => {
                                    let m1024 = f219(v11, v3, v12)?;
                                    match m1024 {
                                        Ok(v14) => {
                                            if f145(match v14.clone() {
                                                Adt20::C0(h1025, _) => {
                                                    h1025
                                                }
                                            }, f8(Str::lit("(List "))) {
                                                Ok(f217(f218(f8(Str::lit("the item type of a list must not itself be a list (XSD 1.1 Part 2, section 4.1.5)")), v0)))
                                            } else {
                                                Ok(Ok::<Adt20, Bytes>(Adt20::C0(f204(v4, f7(List::cons(f8(Str::lit("(List ")), List::cons(match v14.clone() {
                                                    Adt20::C0(h1026, _) => {
                                                        h1026
                                                    }
                                                }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))), match v14 {
                                                    Adt20::C0(_, h1027) => {
                                                        h1027
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
                            let m1028 = f91(v9, f8(Str::lit("base")));
                            match m1028 {
                                None => {
                                    Ok(f217(f218(f8(Str::lit("a restriction with an anonymous base type is not translated yet")), v0)))
                                }
                                Some(v15) => {
                                    let m1029 = f219(v11.clone(), v3, v15)?;
                                    match m1029 {
                                        Ok(v17) => {
                                            let m1030 = f220(v10, v11)?;
                                            match m1030 {
                                                Ok(v19) => {
                                                    let v20: List<Adt17> = f221(v19, List::<Bytes>::nil())?;
                                                    let m1031 = v20.clone();
                                                    match m1031.uncons() {
                                                        None => {
                                                            Ok(Ok::<Adt20, Bytes>(Adt20::C0(f204(v4, match v17.clone() {
                                                                Adt20::C0(h1032, _) => {
                                                                    h1032
                                                                }
                                                            }), match v17 {
                                                                Adt20::C0(_, h1033) => {
                                                                    h1033
                                                                }
                                                            })))
                                                        }
                                                        Some((_, _)) => {
                                                            let m1034 = f196(f191(f222(v20.clone()))?, v20, List::<Bytes>::nil())?;
                                                            match m1034 {
                                                                Ok(v24) => {
                                                                    Ok(Ok::<Adt20, Bytes>(Adt20::C0(f198(v4, v24), List::<Bytes>::nil())))
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

pub fn f152(v0: Bytes) -> R<Bytes> {
    f223(v0.clone(), 0u64, {
        let a1035 = v0;
        length_bytes(a1035)
    }, true, Bytes::lit(&[]))
}

pub fn f153(p1038: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m1037 = p1038;
    {
        let mut m1037 = m1037;
        loop {
            let step = {
                let v0 = m1037;
                let m1036 = v0;
                match m1036.uncons() {
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
                Ok(next) => { m1037 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f154(v0: Bytes) -> Result<List<Bytes>, Bytes> {
    Err::<List<Bytes>, Bytes>(v0)
}

pub fn f155(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f224(v0, v1)
}

pub fn f156(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f225(v0, v1)
}

pub fn f157(v0: u64) -> bool {
    let a1039 = v0;
    let a1040 = 32u64;
    if nat_eq(a1039, a1040) {
        true
    } else {
        let a1041 = v0;
        let a1042 = 9u64;
        nat_eq(a1041, a1042)
    }
}

pub fn f158(v0: u64) -> bool {
    let a1043 = v0;
    let a1044 = 10u64;
    if nat_eq(a1043, a1044) {
        true
    } else {
        let a1045 = v0;
        let a1046 = 256u64;
        nat_eq(a1045, a1046)
    }
}

pub fn f159(v0: Bytes, v1: u64) -> R<u64> {
    let a1047 = v1;
    let a1048 = f226(v0, v1)?;
    Ok(nat_sub(a1047, a1048))
}

pub fn f160(v0: Bytes, v1: u64) -> R<u64> {
    let (h1049, _) = f128(Fn4::F227(v0.clone()), {
        let a1051 = {
            let a1050 = v0;
            length_bytes(a1050)
        };
        let a1052 = 1u64;
        nat_add(a1051, a1052)?
    }, v1)?;
    Ok(h1049)
}

pub fn f161(v0: Bytes, v1: u64) -> R<u64> {
    let (h1053, _) = f128(Fn4::F228(v0.clone()), {
        let a1055 = {
            let a1054 = v0;
            length_bytes(a1054)
        };
        let a1056 = 1u64;
        nat_add(a1055, a1056)?
    }, v1)?;
    Ok(h1053)
}

pub fn f162(v0: Bytes, v1: u64) -> R<bool> {
    let a1057 = f100(v0.clone(), v1)?;
    let a1058 = 45u64;
    if nat_eq(a1057, a1058) {
        if f157(f100(v0.clone(), {
            let a1059 = v1;
            let a1060 = 1u64;
            nat_add(a1059, a1060)?
        })?) {
            Ok(true)
        } else {
            Ok(f158(f100(v0, {
                let a1061 = v1;
                let a1062 = 1u64;
                nat_add(a1061, a1062)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f163(v0: Bytes, v1: u64) -> R<bool> {
    let m1063 = f170(v0, v1)?;
    match m1063 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f164(v0: Bytes, v1: u64) -> R<Option<Bytes>> {
    let v2: u64 = f100(v0.clone(), v1)?;
    let v3: bool = f169(f229(f100(v0, {
        let a1064 = v1;
        let a1065 = 1u64;
        nat_add(a1064, a1065)?
    })?, true));
    let a1066 = v2;
    let a1067 = 58u64;
    if if nat_eq(a1066, a1067) {
        v3
    } else {
        false
    } {
        Ok(Some(f81(Str::lit("a null key is not read: Studio.Value has no null (YAML 1.2.2 section 10.3.2)"), v1)?))
    } else {
        let a1068 = v2;
        let a1069 = 38u64;
        if nat_eq(a1068, a1069) {
            Ok(Some(f81(Str::lit("an anchor is not read (YAML 1.2.2 [101])"), v1)?))
        } else {
            let a1070 = v2;
            let a1071 = 42u64;
            if nat_eq(a1070, a1071) {
                Ok(Some(f81(Str::lit("an alias is not read (YAML 1.2.2 [104])"), v1)?))
            } else {
                let a1072 = v2;
                let a1073 = 33u64;
                if nat_eq(a1072, a1073) {
                    Ok(Some(f81(Str::lit("a tag is not read (YAML 1.2.2 [97])"), v1)?))
                } else {
                    let a1074 = v2;
                    let a1075 = 63u64;
                    if if nat_eq(a1074, a1075) {
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

pub fn f165(v0: Bytes, v1: u64, v2: u64) -> R<Adt18> {
    let v3: u64 = f100(v0.clone(), v1)?;
    let a1076 = v3;
    let a1077 = 124u64;
    if if nat_eq(a1076, a1077) {
        true
    } else {
        let a1078 = v3;
        let a1079 = 62u64;
        nat_eq(a1078, a1079)
    } {
        f230(v0, v1, v2)
    } else {
        let a1080 = v3;
        let a1081 = 34u64;
        if if nat_eq(a1080, a1081) {
            true
        } else {
            let a1082 = v3;
            let a1083 = 39u64;
            nat_eq(a1082, a1083)
        } {
            let m1084 = f231(v0.clone(), v1, v2, false)?;
            match m1084 {
                Adt22::C0(v5, v6) => {
                    let m1085 = f173(v0, v6)?;
                    match m1085 {
                        Ok(v8) => {
                            Ok(Adt18::C0(Adt17::C3(v5), v8))
                        }
                        Err(v7) => {
                            Ok(Adt18::C1(v7))
                        }
                    }
                }
                Adt22::C1(v4) => {
                    Ok(Adt18::C1(v4))
                }
            }
        } else {
            let a1086 = v3;
            let a1087 = 91u64;
            if if nat_eq(a1086, a1087) {
                true
            } else {
                let a1088 = v3;
                let a1089 = 123u64;
                nat_eq(a1088, a1089)
            } {
                let m1093 = f232(v0.clone(), v1, v2, {
                    let a1091 = {
                        let a1090 = v0.clone();
                        length_bytes(a1090)
                    };
                    let a1092 = 2u64;
                    nat_add(a1091, a1092)?
                }, 0u64, List::<Bytes>::nil())?;
                match m1093 {
                    Adt18::C0(v10, v11) => {
                        let a1094 = f100(v0.clone(), f161(v0.clone(), v11)?)?;
                        let a1095 = 58u64;
                        if nat_eq(a1094, a1095) {
                            Ok(Adt18::C1(f81(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [154])"), v1)?))
                        } else {
                            let m1096 = f173(v0, v11)?;
                            match m1096 {
                                Ok(v13) => {
                                    Ok(Adt18::C0(v10, v13))
                                }
                                Err(v12) => {
                                    Ok(Adt18::C1(v12))
                                }
                            }
                        }
                    }
                    Adt18::C1(v9) => {
                        Ok(Adt18::C1(v9))
                    }
                }
            } else {
                let m1097 = f164(v0.clone(), v1)?;
                match m1097 {
                    None => {
                        if f169(f233(v0.clone(), v1, false)?) {
                            Ok(Adt18::C1(f81(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                        } else {
                            let v15: u64 = f234(v0.clone(), v1, false)?;
                            let m1101 = f235(v0.clone(), v15, v2, {
                                let a1099 = {
                                    let a1098 = v0.clone();
                                    length_bytes(a1098)
                                };
                                let a1100 = 2u64;
                                nat_add(a1099, a1100)?
                            }, f236(v0, v1, v15))?;
                            match m1101 {
                                Adt22::C0(v17, v18) => {
                                    f237(v17, v1, v18)
                                }
                                Adt22::C1(v16) => {
                                    Ok(Adt18::C1(v16))
                                }
                            }
                        }
                    }
                    Some(v14) => {
                        Ok(Adt18::C1(v14))
                    }
                }
            }
        }
    }
}

pub fn f166(v0: bool, v1: Adt18) -> Adt18 {
    let m1102 = v1;
    match m1102 {
        Adt18::C0(v3, v4) => {
            Adt18::C0(if v0 {
                Adt17::C4(std::rc::Rc::new(v3))
            } else {
                Adt17::C5(std::rc::Rc::new(v3))
            }, v4)
        }
        Adt18::C1(v2) => {
            Adt18::C1(v2)
        }
    }
}

pub fn f167(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f238(v0.clone(), v1)?;
    let a1104 = v2;
    let a1105 = {
        let a1103 = v0;
        length_bytes(a1103)
    };
    if nat_lt(a1104, a1105) {
        let a1106 = v2;
        let a1107 = 1u64;
        nat_add(a1106, a1107)
    } else {
        Ok(v2)
    }
}

pub fn f168(v0: u64) -> R<Bytes> {
    f81(Str::lit("a null node is not read: Studio.Value has no null (YAML 1.2.2 section 10.3.2)"), v0)
}

pub fn f169(v0: bool) -> bool {
    !v0
}

pub fn f170(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f100(v0.clone(), v1)?;
    let a1108 = v2;
    let a1109 = 34u64;
    if if nat_eq(a1108, a1109) {
        true
    } else {
        let a1110 = v2;
        let a1111 = 39u64;
        nat_eq(a1110, a1111)
    } {
        let m1112 = f231(v0.clone(), v1, 0u64, true)?;
        match m1112 {
            Adt22::C0(_, v5) => {
                let v6: u64 = f161(v0.clone(), v5)?;
                if f239(v0, v6)? {
                    Ok(Some(v6))
                } else {
                    Ok(None::<u64>)
                }
            }
            Adt22::C1(_) => {
                Ok(None::<u64>)
            }
        }
    } else {
        if f233(v0.clone(), v1, false)? {
            let v7: u64 = f161(v0.clone(), f234(v0.clone(), v1, false)?)?;
            if f239(v0, v7)? {
                Ok(Some(v7))
            } else {
                Ok(None::<u64>)
            }
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f171(v0: Bytes, v1: u64) -> R<Bytes> {
    let v2: u64 = f100(v0.clone(), v1)?;
    let a1113 = v2;
    let a1114 = 34u64;
    if if nat_eq(a1113, a1114) {
        true
    } else {
        let a1115 = v2;
        let a1116 = 39u64;
        nat_eq(a1115, a1116)
    } {
        let m1117 = f231(v0, v1, 0u64, true)?;
        match m1117 {
            Adt22::C0(v4, _) => {
                Ok(v4)
            }
            Adt22::C1(_) => {
                Ok(Bytes::lit(&[]))
            }
        }
    } else {
        Ok(f236(v0.clone(), v1, f234(v0, v1, false)?))
    }
}

pub fn f172(v0: Bytes, p1120: List<Bytes>) -> bool {
    let m1119 = p1120;
    {
        let mut m1119 = m1119;
        loop {
            let step = {
                let v1 = m1119;
                let m1118 = v1;
                match m1118.uncons() {
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
                Ok(next) => { m1119 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f173(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f161(v0.clone(), v1)?;
    let v3: u64 = f100(v0.clone(), v2)?;
    if if f158(v3) {
        true
    } else {
        let a1121 = v3;
        let a1122 = 35u64;
        if nat_eq(a1121, a1122) {
            let a1123 = v1;
            let a1124 = v2;
            nat_lt(a1123, a1124)
        } else {
            false
        }
    } {
        Ok(Ok::<u64, Bytes>(f167(v0, v2)?))
    } else {
        Ok(Err::<u64, Bytes>(f81(Str::lit("only white space and a comment may follow a node on its line (YAML 1.2.2 [77])"), v2)?))
    }
}

pub fn f174(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1125 = 0u64;
        let a1126 = v1;
        nat_lt(a1125, a1126)
    } else {
        false
    } {
        let a1127 = Bytes::lit(&[0x2d]);
        let a1128 = f51(v1)?;
        Ok(append_bytes(a1127, a1128))
    } else {
        f51(v1)
    }
}

pub fn f175(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f51(v1)?;
    let v4: u64 = {
        let a1129 = v3.clone();
        length_bytes(a1129)
    };
    let v5: Bytes = {
        let a1130 = v2;
        let a1131 = v4;
        if nat_lt(a1130, a1131) {
            f7(List::cons(f32(v3.clone(), 0u64, {
                let a1132 = v4;
                let a1133 = v2;
                nat_sub(a1132, a1133)
            }), List::cons(f8(Str::lit(".")), List::cons(f32(v3, {
                let a1134 = v4;
                let a1135 = v2;
                nat_sub(a1134, a1135)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f240({
                let a1136 = v2;
                let a1137 = v4;
                nat_sub(a1136, a1137)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1138 = f8(Str::lit("-"));
        let a1139 = v5;
        Ok(append_bytes(a1138, a1139))
    } else {
        Ok(v5)
    }
}

pub fn f176(v0: Bytes) -> R<Bytes> {
    let a1144 = f241(v0.clone(), 0u64, {
        let a1141 = {
            let a1140 = v0.clone();
            length_bytes(a1140)
        };
        let a1142 = 1u64;
        nat_add(a1141, a1142)?
    })?;
    let a1145 = {
        let a1143 = v0.clone();
        length_bytes(a1143)
    };
    if nat_eq(a1144, a1145) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        f242(v0)
    }
}

pub fn f177(v0: List<Bytes>) -> Bytes {
    let a1148 = {
        let a1146 = Bytes::lit(&[0x5b]);
        let a1147 = f33(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1146, a1147)
    };
    let a1149 = Bytes::lit(&[0x5d]);
    append_bytes(a1148, a1149)
}

pub fn f178(v0: Adt17) -> R<List<Bytes>> {
    let m1150 = v0;
    match m1150 {
        Adt17::C0(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt17::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt17::C2(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt17::C3(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt17::C4(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt17::C5(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt17::C6 => {
            Ok(List::<Bytes>::nil())
        }
        Adt17::C7(r1151, r1152) => {
            let v10 = (*r1151).clone();
            let v11 = (*r1152).clone();
            Ok(List::cons(f103(v10)?, f178(v11)?))
        }
        Adt17::C8(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f179(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f243(Fn5::F244, List::<(Bytes, Bytes)>::nil(), f245(v0)?);
    let v2: List<Bytes> = f246(Fn6::F247, List::<Bytes>::nil(), v1)?;
    let a1155 = {
        let a1153 = Bytes::lit(&[0x7b]);
        let a1154 = f33(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1153, a1154)
    };
    let a1156 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1155, a1156))
}

pub fn f180(v0: Adt17) -> R<List<(Bytes, Bytes)>> {
    let m1157 = v0;
    match m1157 {
        Adt17::C0(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt17::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt17::C2(_, _, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt17::C3(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt17::C4(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt17::C5(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt17::C6 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt17::C7(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt17::C8(v12, r1158, r1159) => {
            let v13 = (*r1158).clone();
            let v14 = (*r1159).clone();
            Ok(List::cons((v12, f103(v13)?), f180(v14)?))
        }
    }
}

pub fn f181(p1214: u64, v1: Bytes, p1215: u64, p1216: List<Adt19>) -> R<Result<List<Adt19>, Bytes>> {
    let m1213 = (p1214, (p1215, p1216));
    {
        let mut m1213 = m1213;
        loop {
            let step = {
                let (v0, (v2, v3)) = m1213;
                let m1160 = v0;
                if m1160 == 0 {
                    Err::<(u64, (u64, List<Adt19>)), R<Result<List<Adt19>, Bytes>>>(Ok(Err::<List<Adt19>, Bytes>(f248(Str::lit("the input has more tokens than bytes"), v2)?)))
                } else {
                    let v4 = m1160 - 1;
                    let v5: u64 = f85(v1.clone(), v2)?;
                    let a1161 = v5;
                    let a1162 = 256u64;
                    if nat_eq(a1161, a1162) {
                        Err::<(u64, (u64, List<Adt19>)), R<Result<List<Adt19>, Bytes>>>(Ok(Ok::<List<Adt19>, Bytes>(f249(v3, List::<Adt19>::nil()))))
                    } else {
                        if f130(v5)? {
                            Ok::<(u64, (u64, List<Adt19>)), R<Result<List<Adt19>, Bytes>>>((v4, ({
                                let a1163 = v2;
                                let a1164 = 1u64;
                                nat_add(a1163, a1164)?
                            }, v3)))
                        } else {
                            if f84(v1.clone(), v2, Str::lit("//")) {
                                let m1165 = f131(v1.clone(), Bytes::lit(&[0x0a]), v2)?;
                                match m1165 {
                                    None => {
                                        Err::<(u64, (u64, List<Adt19>)), R<Result<List<Adt19>, Bytes>>>(Ok(Ok::<List<Adt19>, Bytes>(f249(v3, List::<Adt19>::nil()))))
                                    }
                                    Some(v6) => {
                                        Ok::<(u64, (u64, List<Adt19>)), R<Result<List<Adt19>, Bytes>>>((v4, ({
                                            let a1166 = v6;
                                            let a1167 = 1u64;
                                            nat_add(a1166, a1167)?
                                        }, v3)))
                                    }
                                }
                            } else {
                                if f84(v1.clone(), v2, Str::lit("/*")) {
                                    let m1171 = f131(v1.clone(), {
                                        let a1168 = Str::lit("*/");
                                        utf8_encode(a1168)
                                    }, {
                                        let a1169 = v2;
                                        let a1170 = 2u64;
                                        nat_add(a1169, a1170)?
                                    })?;
                                    match m1171 {
                                        None => {
                                            Err::<(u64, (u64, List<Adt19>)), R<Result<List<Adt19>, Bytes>>>(Ok(Err::<List<Adt19>, Bytes>(f248(Str::lit("a comment is not closed (Web IDL, IDL grammar: comment)"), v2)?)))
                                        }
                                        Some(v7) => {
                                            Ok::<(u64, (u64, List<Adt19>)), R<Result<List<Adt19>, Bytes>>>((v4, ({
                                                let a1172 = v7;
                                                let a1173 = 2u64;
                                                nat_add(a1172, a1173)?
                                            }, v3)))
                                        }
                                    }
                                } else {
                                    let a1174 = v5;
                                    let a1175 = 34u64;
                                    if nat_eq(a1174, a1175) {
                                        let m1178 = f131(v1.clone(), Bytes::lit(&[0x22]), {
                                            let a1176 = v2;
                                            let a1177 = 1u64;
                                            nat_add(a1176, a1177)?
                                        })?;
                                        match m1178 {
                                            None => {
                                                Err::<(u64, (u64, List<Adt19>)), R<Result<List<Adt19>, Bytes>>>(Ok(Err::<List<Adt19>, Bytes>(f248(Str::lit("a string is not closed (Web IDL, IDL grammar: string)"), v2)?)))
                                            }
                                            Some(v8) => {
                                                Ok::<(u64, (u64, List<Adt19>)), R<Result<List<Adt19>, Bytes>>>((v4, ({
                                                    let a1179 = v8;
                                                    let a1180 = 1u64;
                                                    nat_add(a1179, a1180)?
                                                }, List::cons(Adt19::C0(3u64, f32(v1.clone(), {
                                                    let a1181 = v2;
                                                    let a1182 = 1u64;
                                                    nat_add(a1181, a1182)?
                                                }, {
                                                    let a1185 = v8;
                                                    let a1186 = {
                                                        let a1183 = v2;
                                                        let a1184 = 1u64;
                                                        nat_add(a1183, a1184)?
                                                    };
                                                    nat_sub(a1185, a1186)
                                                })), v3))))
                                            }
                                        }
                                    } else {
                                        let v9: u64 = {
                                            let a1187 = v5;
                                            let a1188 = 95u64;
                                            if if nat_eq(a1187, a1188) {
                                                true
                                            } else {
                                                let a1189 = v5;
                                                let a1190 = 45u64;
                                                nat_eq(a1189, a1190)
                                            } {
                                                let a1191 = v2;
                                                let a1192 = 1u64;
                                                nat_add(a1191, a1192)?
                                            } else {
                                                v2
                                            }
                                        };
                                        if f250(f85(v1.clone(), v9)?) {
                                            let v10: u64 = f251(v1.clone(), 3u64, {
                                                let a1193 = v9;
                                                let a1194 = 1u64;
                                                nat_add(a1193, a1194)?
                                            })?;
                                            Ok::<(u64, (u64, List<Adt19>)), R<Result<List<Adt19>, Bytes>>>((v4, (v10, List::cons(Adt19::C0(0u64, f32(v1.clone(), v2, {
                                                let a1195 = v10;
                                                let a1196 = v2;
                                                nat_sub(a1195, a1196)
                                            })), v3))))
                                        } else {
                                            let v11: u64 = {
                                                let a1197 = v5;
                                                let a1198 = 45u64;
                                                if nat_eq(a1197, a1198) {
                                                    let a1199 = v2;
                                                    let a1200 = 1u64;
                                                    nat_add(a1199, a1200)?
                                                } else {
                                                    v2
                                                }
                                            };
                                            let v12: u64 = f252(v1.clone(), v11)?;
                                            let v13: u64 = f253(v1.clone(), v11)?;
                                            let a1201 = v12;
                                            let a1202 = v13;
                                            if nat_lt(a1201, a1202) {
                                                Ok::<(u64, (u64, List<Adt19>)), R<Result<List<Adt19>, Bytes>>>((v4, (v13, List::cons(Adt19::C0(2u64, f32(v1.clone(), v2, {
                                                    let a1203 = v13;
                                                    let a1204 = v2;
                                                    nat_sub(a1203, a1204)
                                                })), v3))))
                                            } else {
                                                let a1205 = v11;
                                                let a1206 = v12;
                                                if nat_lt(a1205, a1206) {
                                                    Ok::<(u64, (u64, List<Adt19>)), R<Result<List<Adt19>, Bytes>>>((v4, (v12, List::cons(Adt19::C0(1u64, f32(v1.clone(), v2, {
                                                        let a1207 = v12;
                                                        let a1208 = v2;
                                                        nat_sub(a1207, a1208)
                                                    })), v3))))
                                                } else {
                                                    if f84(v1.clone(), v2, Str::lit("...")) {
                                                        Ok::<(u64, (u64, List<Adt19>)), R<Result<List<Adt19>, Bytes>>>((v4, ({
                                                            let a1209 = v2;
                                                            let a1210 = 3u64;
                                                            nat_add(a1209, a1210)?
                                                        }, List::cons(Adt19::C0(4u64, f8(Str::lit("..."))), v3))))
                                                    } else {
                                                        Ok::<(u64, (u64, List<Adt19>)), R<Result<List<Adt19>, Bytes>>>((v4, ({
                                                            let a1211 = v2;
                                                            let a1212 = 1u64;
                                                            nat_add(a1211, a1212)?
                                                        }, List::cons(Adt19::C0(4u64, f32(v1.clone(), v2, 1u64)), v3))))
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
                Ok(next) => { m1213 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f182(p1219: List<Adt12>, p1220: List<Adt12>) -> List<Adt12> {
    let m1218 = (p1219, p1220);
    {
        let mut m1218 = m1218;
        loop {
            let step = {
                let (v0, v1) = m1218;
                let m1217 = v0;
                match m1217.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1218 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f183(v0: List<Adt19>) -> R<Result<List<Adt19>, Bytes>> {
    if f254(v0.clone(), Str::lit("[")) {
        Ok(f255({
            let a1222 = {
                let a1221 = v0.clone();
                length_list(a1221)
            };
            let a1223 = 1u64;
            nat_add(a1222, a1223)?
        }, f256(v0), List::cons(f8(Str::lit("]")), List::<Bytes>::nil()), false))
    } else {
        Ok(Ok::<List<Adt19>, Bytes>(v0))
    }
}

pub fn f184(v0: u64, v1: List<Adt19>) -> R<Result<(Adt12, List<Adt19>), Bytes>> {
    if f254(v1.clone(), Str::lit("interface")) {
        if f254(f256(v1.clone()), Str::lit("mixin")) {
            f257(v0, 1u64, f256(f256(v1)), false)
        } else {
            f257(v0, 0u64, f256(v1), true)
        }
    } else {
        if f254(v1.clone(), Str::lit("partial")) {
            let v2: List<Adt19> = f256(v1);
            if f254(v2.clone(), Str::lit("interface")) {
                if f254(f256(v2.clone()), Str::lit("mixin")) {
                    f257(v0, 6u64, f256(f256(v2)), false)
                } else {
                    f257(v0, 5u64, f256(v2), false)
                }
            } else {
                if f254(v2.clone(), Str::lit("dictionary")) {
                    f257(v0, 7u64, f256(v2), false)
                } else {
                    if f254(v2.clone(), Str::lit("namespace")) {
                        f258(f256(v2), f8(Str::lit("a namespace holds operations and attributes of no object, so it is behavior, not data (Web IDL 2.6)")))
                    } else {
                        Ok(Err::<(Adt12, List<Adt19>), Bytes>(f259(Str::lit("interface, dictionary or namespace"), v2)))
                    }
                }
            }
        } else {
            if f254(v1.clone(), Str::lit("dictionary")) {
                f257(v0, 2u64, f256(v1), true)
            } else {
                if f254(v1.clone(), Str::lit("enum")) {
                    let m1224 = f260(f256(v1));
                    match m1224 {
                        Ok(v4) => {
                            let m1225 = f261(f262(v4.clone()), Str::lit("{"));
                            match m1225 {
                                Ok(v6) => {
                                    let m1226 = f263(v0, v6, List::<Bytes>::nil());
                                    match m1226 {
                                        Ok(v8) => {
                                            Ok(Ok::<(Adt12, List<Adt19>), Bytes>((Adt12::C0(3u64, f264(v4), Bytes::lit(&[]), List::<Adt13>::nil(), {
                                                let (h1227, _) = v8;
                                                h1227
                                            }, f265(Bytes::lit(&[]))), {
                                                let (_, h1228) = v8;
                                                h1228
                                            })))
                                        }
                                        Err(v7) => {
                                            Ok(Err::<(Adt12, List<Adt19>), Bytes>(v7))
                                        }
                                    }
                                }
                                Err(v5) => {
                                    Ok(Err::<(Adt12, List<Adt19>), Bytes>(v5))
                                }
                            }
                        }
                        Err(v3) => {
                            Ok(Err::<(Adt12, List<Adt19>), Bytes>(v3))
                        }
                    }
                } else {
                    if f254(v1.clone(), Str::lit("typedef")) {
                        let m1229 = f266(v0, f256(v1), 0u64)?;
                        match m1229 {
                            Ok(v10) => {
                                let m1230 = f260(f267(v10.clone()));
                                match m1230 {
                                    Ok(v12) => {
                                        let m1231 = f261(f262(v12.clone()), Str::lit(";"));
                                        match m1231 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt12, List<Adt19>), Bytes>((Adt12::C0(4u64, f264(v12), Bytes::lit(&[]), List::<Adt13>::nil(), List::<Bytes>::nil(), f268(v10)), v14)))
                                            }
                                            Err(v13) => {
                                                Ok(Err::<(Adt12, List<Adt19>), Bytes>(v13))
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(Err::<(Adt12, List<Adt19>), Bytes>(v11))
                                    }
                                }
                            }
                            Err(v9) => {
                                Ok(Err::<(Adt12, List<Adt19>), Bytes>(v9))
                            }
                        }
                    } else {
                        if f254(v1.clone(), Str::lit("callback")) {
                            if f254(f256(v1.clone()), Str::lit("interface")) {
                                f258(f256(f256(v1)), f8(Str::lit("a callback interface is implemented by user code, so it is behavior, not data (Web IDL 2.4)")))
                            } else {
                                f258(f256(v1), f8(Str::lit("a callback function is behavior, not data (Web IDL 2.10)")))
                            }
                        } else {
                            if f254(v1.clone(), Str::lit("namespace")) {
                                f258(f256(v1), f8(Str::lit("a namespace holds operations and attributes of no object, so it is behavior, not data (Web IDL 2.6)")))
                            } else {
                                let m1232 = f260(v1);
                                match m1232 {
                                    Ok(v16) => {
                                        let m1233 = f261(f262(v16.clone()), Str::lit("includes"));
                                        match m1233 {
                                            Ok(v18) => {
                                                let m1234 = f260(v18);
                                                match m1234 {
                                                    Ok(v20) => {
                                                        let m1235 = f261(f262(v20.clone()), Str::lit(";"));
                                                        match m1235 {
                                                            Ok(v22) => {
                                                                Ok(Ok::<(Adt12, List<Adt19>), Bytes>((Adt12::C0(8u64, f264(v16), f264(v20), List::<Adt13>::nil(), List::<Bytes>::nil(), f265(Bytes::lit(&[]))), v22)))
                                                            }
                                                            Err(v21) => {
                                                                Ok(Err::<(Adt12, List<Adt19>), Bytes>(v21))
                                                            }
                                                        }
                                                    }
                                                    Err(v19) => {
                                                        Ok(Err::<(Adt12, List<Adt19>), Bytes>(v19))
                                                    }
                                                }
                                            }
                                            Err(v17) => {
                                                Ok(Err::<(Adt12, List<Adt19>), Bytes>(v17))
                                            }
                                        }
                                    }
                                    Err(v15) => {
                                        Ok(Err::<(Adt12, List<Adt19>), Bytes>(v15))
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

pub fn f185(p1239: List<Bytes>, v1: List<Adt15>) -> Bytes {
    let m1238 = p1239;
    {
        let mut m1238 = m1238;
        loop {
            let step = {
                let v0 = m1238;
                let m1236 = v0;
                match m1236.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1237 = f114(v1.clone(), v2.clone());
                        match m1237 {
                            None => {
                                Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("refers to ")), List::cons(v2, List::cons(f8(Str::lit(", which no input defines (Web IDL 2.13.22 to 2.13.26)")), List::<Bytes>::nil())))))
                            }
                            Some(v4) => {
                                if f35(f115(v4.clone()), Bytes::lit(&[])) {
                                    Ok::<List<Bytes>, Bytes>(v3)
                                } else {
                                    Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("refers to ")), List::cons(v2, List::cons(f8(Str::lit(", which is not translated: ")), List::cons(f115(v4), List::<Bytes>::nil()))))))
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

pub fn f186(v0: Bytes) -> bool {
    if f113(v0.clone(), f269()) {
        true
    } else {
        if f113(v0.clone(), f270()) {
            true
        } else {
            if f113(v0.clone(), f271()) {
                true
            } else {
                if f113(v0.clone(), f272()) {
                    true
                } else {
                    if f113(v0.clone(), f273()) {
                        true
                    } else {
                        if f113(v0.clone(), f274()) {
                            true
                        } else {
                            f113(v0, f275())
                        }
                    }
                }
            }
        }
    }
}

pub fn f187(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("the derived name ")), List::cons(v0, List::cons(f8(Str::lit(" is a reserved token of Lean 4.32.2, which LexLean 0.4.3 refuses as a name (lexlean 0.4.3 src/backend/lean_tokens.rs, SPEC.md 17.8)")), List::<Bytes>::nil()))))
}

pub fn f188(p1245: List<Adt12>, v1: List<Adt12>, v2: Bytes) -> Option<Bytes> {
    let m1244 = p1245;
    {
        let mut m1244 = m1244;
        loop {
            let step = {
                let v0 = m1244;
                let m1240 = v0;
                match m1240.uncons() {
                    None => {
                        Err::<List<Adt12>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v3, v4)) => {
                        let a1241 = f110(v3.clone());
                        let a1242 = 8u64;
                        if if nat_eq(a1241, a1242) {
                            f35(f112(v3.clone()), v2.clone())
                        } else {
                            false
                        } {
                            let m1243 = f121(v1.clone(), 1u64, f123(v3.clone()));
                            match m1243 {
                                None => {
                                    Err::<List<Adt12>, Option<Bytes>>(Some(f123(v3)))
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
                Ok(next) => { m1244 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f189(v0: u64, v1: List<Adt12>, v2: Adt12) -> R<Result<List<Adt13>, Bytes>> {
    let m1246 = v0;
    if m1246 == 0 {
        Ok(Err::<List<Adt13>, Bytes>(f7(List::cons(f8(Str::lit("the inheritance of ")), List::cons(f112(v2), List::cons(f8(Str::lit(" is cyclic (Web IDL 2.2 and 2.7: an interface or dictionary must not inherit from itself)")), List::<Bytes>::nil()))))))
    } else {
        let v3 = m1246 - 1;
        let v4: List<Adt13> = {
            let a1247 = f110(v2.clone());
            let a1248 = 2u64;
            if nat_eq(a1247, a1248) {
                f276(f277(f278(v2.clone()), f279(v1.clone(), 7u64, f112(v2.clone()))))?
            } else {
                f277(f278(v2.clone()), f277(f279(v1.clone(), 5u64, f112(v2.clone())), f280(v1.clone(), v1.clone(), f112(v2.clone()))))
            }
        };
        if f35(f123(v2.clone()), Bytes::lit(&[])) {
            Ok(Ok::<List<Adt13>, Bytes>(v4))
        } else {
            let m1249 = f121(v1.clone(), f110(v2.clone()), f123(v2.clone()));
            match m1249 {
                None => {
                    Ok(Err::<List<Adt13>, Bytes>(f7(List::cons(f8(Str::lit("inherits ")), List::cons(f123(v2.clone()), List::cons({
                        let a1250 = f110(v2);
                        let a1251 = 2u64;
                        if nat_eq(a1250, a1251) {
                            f8(Str::lit(", which no input defines as a dictionary (Web IDL 2.7)"))
                        } else {
                            f8(Str::lit(", which no input defines as an interface (Web IDL 2.2)"))
                        }
                    }, List::<Bytes>::nil()))))))
                }
                Some(v5) => {
                    let m1252 = f189(v3, v1, v5)?;
                    match m1252 {
                        Ok(v7) => {
                            Ok(Ok::<List<Adt13>, Bytes>(f277(v7, v4)))
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

pub fn f190(v0: bool, v1: List<Adt13>) -> R<Result<(List<Bytes>, List<Bytes>), Bytes>> {
    let m1253 = v1;
    match m1253.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::<Bytes>::nil(), List::<Bytes>::nil())))
        }
        Some((v2, v3)) => {
            let v4: Adt14 = f281(v2.clone());
            if !f35(f200(v4.clone()), Bytes::lit(&[])) {
                Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons(f282(v2), List::cons(f8(Str::lit(": ")), List::cons(f200(v4), List::<Bytes>::nil())))))))
            } else {
                let m1254 = f283(v0, f282(v2.clone()))?;
                match m1254 {
                    None => {
                        Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons(f282(v2), List::cons(f8(Str::lit(" has no field name")), List::<Bytes>::nil()))))))
                    }
                    Some(v5) => {
                        if f186(v5.clone()) {
                            Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f187(v5)))
                        } else {
                            if f113(v5.clone(), f284(v0, f192(v3.clone()))?) {
                                Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("two of its members derive the field name ")), List::cons(v5, List::cons(f8(Str::lit(" (Web IDL 2.2 and 2.7: the members of one definition have distinct identifiers)")), List::<Bytes>::nil()))))))
                            } else {
                                let m1255 = f190(v0, v3)?;
                                match m1255 {
                                    Ok(v7) => {
                                        Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v5, List::cons(f8(Str::lit(", type: ")), List::cons({
                                            let a1256 = f285(v2);
                                            let a1257 = 1u64;
                                            if nat_eq(a1256, a1257) {
                                                f7(List::cons(f8(Str::lit("(Option ")), List::cons(f202(v4.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                            } else {
                                                f202(v4.clone())
                                            }
                                        }, List::cons(f8(Str::lit("}")), List::<Bytes>::nil())))))), {
                                            let (h1258, _) = v7;
                                            h1258
                                        }), f116(f203(v4), {
                                            let (_, h1259) = v7;
                                            h1259
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

pub fn f191(p1263: List<Bytes>) -> R<bool> {
    let m1262 = p1263;
    {
        let mut m1262 = m1262;
        loop {
            let step = {
                let v0 = m1262;
                let m1260 = v0;
                match m1260.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m1261 = f286(v1)?;
                        match m1261 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f113(v3, f284(false, v2.clone())?) {
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
                Ok(next) => { m1262 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f192(v0: List<Adt13>) -> List<Bytes> {
    let m1264 = v0;
    match m1264.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f282(v1), f192(v2))
        }
    }
}

pub fn f193(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}")), List::cons(f73(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f194(v0: Adt12) -> List<Bytes> {
    let m1265 = v0;
    match m1265 {
        Adt12::C0(_, _, _, _, v5, _) => {
            v5
        }
    }
}

pub fn f195(v0: bool, p1269: List<Bytes>) -> R<Option<Bytes>> {
    let m1268 = p1269;
    {
        let mut m1268 = m1268;
        loop {
            let step = {
                let v1 = m1268;
                let m1266 = v1;
                match m1266.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m1267 = f283(v0, v2)?;
                        match m1267 {
                            None => {
                                Ok::<List<Bytes>, R<Option<Bytes>>>(v3)
                            }
                            Some(v4) => {
                                if f186(v4.clone()) {
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
                Ok(next) => { m1268 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f196(v0: bool, v1: List<Adt17>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1270 = v1;
    match m1270.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m1271 = f287(v0, v3.clone())?;
            match m1271 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f57(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f113(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m1272 = f196(v0, v4, List::cons(v5.clone(), v2))?;
                        match m1272 {
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

pub fn f197(v0: List<Bytes>) -> List<Adt17> {
    let m1273 = v0;
    match m1273.uncons() {
        None => {
            List::<Adt17>::nil()
        }
        Some((v1, v2)) => {
            List::cons(Adt17::C3(v1), f197(v2))
        }
    }
}

pub fn f198(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f73(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f199(v0: Adt12) -> Adt14 {
    let m1274 = v0;
    match m1274 {
        Adt12::C0(_, _, _, _, _, v6) => {
            v6
        }
    }
}

pub fn f200(v0: Adt14) -> Bytes {
    let m1275 = v0;
    match m1275 {
        Adt14::C0(_, _, _, v4, _) => {
            v4
        }
    }
}

pub fn f201(v0: Adt14) -> u64 {
    let m1276 = v0;
    match m1276 {
        Adt14::C0(v1, _, _, _, _) => {
            v1
        }
    }
}

pub fn f202(v0: Adt14) -> Bytes {
    let m1277 = v0;
    match m1277 {
        Adt14::C0(_, v2, _, _, _) => {
            v2
        }
    }
}

pub fn f203(v0: Adt14) -> List<Bytes> {
    let m1278 = v0;
    match m1278 {
        Adt14::C0(_, _, v3, _, _) => {
            v3
        }
    }
}

pub fn f204(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f205(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h1279, _) = v2;
        h1279
    };
    let a1280 = v3;
    let a1281 = 0u64;
    if nat_eq(a1280, a1281) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a1282 = v3;
            let a1283 = v1;
            let a1284 = 0u64;
            nat_quot(a1282, a1283, a1284)
        }, {
            let a1289 = f124(v0, {
                let a1285 = v3;
                let a1286 = v1;
                let a1287 = 0u64;
                nat_rem(a1285, a1286, a1287)
            });
            let a1290 = {
                let (_, h1288) = v2;
                h1288
            };
            append_bytes(a1289, a1290)
        }))
    }
}

pub fn f206(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h1291, _) = v1;
        h1291
    };
    let a1293 = {
        let a1292 = v0.clone();
        length_bytes(a1292)
    };
    let a1294 = v2;
    if nat_le(a1293, a1294) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let a1295 = f85(v0.clone(), v2)?;
        let a1296 = 13u64;
        if nat_eq(a1295, a1296) {
            Ok(Some(({
                let a1299 = f85(v0, {
                    let a1297 = v2;
                    let a1298 = 1u64;
                    nat_add(a1297, a1298)?
                })?;
                let a1300 = 10u64;
                if nat_eq(a1299, a1300) {
                    let a1301 = v2;
                    let a1302 = 2u64;
                    nat_add(a1301, a1302)?
                } else {
                    let a1303 = v2;
                    let a1304 = 1u64;
                    nat_add(a1303, a1304)?
                }
            }, {
                let a1306 = {
                    let (_, h1305) = v1;
                    h1305
                };
                let a1307 = f8(Str::lit("\u{a}"));
                append_bytes(a1306, a1307)
            })))
        } else {
            let v4: u64 = {
                let m1308 = f131(v0.clone(), f8(Str::lit("\u{d}")), v2)?;
                match m1308 {
                    None => {
                        let a1309 = v0.clone();
                        length_bytes(a1309)
                    }
                    Some(v3) => {
                        v3
                    }
                }
            };
            Ok(Some((v4, {
                let a1313 = {
                    let (_, h1310) = v1;
                    h1310
                };
                let a1314 = f32(v0, v2, {
                    let a1311 = v4;
                    let a1312 = v2;
                    nat_sub(a1311, a1312)
                });
                append_bytes(a1313, a1314)
            })))
        }
    }
}

pub fn f207(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1315 = f288(v0, v1)?;
    match m1315 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f289({
                let (h1316, _) = v2;
                h1316
            })? {
                Ok(Some({
                    let a1318 = v1;
                    let a1319 = {
                        let (_, h1317) = v2;
                        h1317
                    };
                    nat_add(a1318, a1319)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f208(v0: u64, v1: Bytes) -> R<bool> {
    f290(v0, v1.clone(), 0u64, {
        let a1321 = {
            let a1320 = v1;
            length_bytes(a1320)
        };
        let a1322 = 3u64;
        let a1323 = 0u64;
        nat_quot(a1321, a1322, a1323)
    }, true)
}

pub fn f209() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{0}\u{0} "))
}

pub fn f210(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f291(v1, v0, v2)
}

pub fn f211(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f292(v0, v1)
}

pub fn f212(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f293(v0, v1, v2)
}

pub fn f213(v0: Bytes) -> Result<(Bytes, u64), Bytes> {
    Err::<(Bytes, u64), Bytes>(v0)
}

pub fn f214(v0: Bytes, v1: u64) -> R<Result<(Bytes, u64), Bytes>> {
    if f84(v0.clone(), v1, Str::lit("&#")) {
        let v2: bool = {
            let a1326 = f85(v0.clone(), {
                let a1324 = v1;
                let a1325 = 2u64;
                nat_add(a1324, a1325)?
            })?;
            let a1327 = 120u64;
            nat_eq(a1326, a1327)
        };
        let v3: u64 = if v2 {
            let a1328 = v1;
            let a1329 = 3u64;
            nat_add(a1328, a1329)?
        } else {
            let a1330 = v1;
            let a1331 = 2u64;
            nat_add(a1330, a1331)?
        };
        let v4: (u64, u64) = f294(v0.clone(), v2, v3)?;
        let a1333 = {
            let (h1332, _) = v4;
            h1332
        };
        let a1334 = v3;
        if if nat_eq(a1333, a1334) {
            true
        } else {
            let a1336 = f85(v0, {
                let (h1335, _) = v4;
                h1335
            })?;
            let a1337 = 59u64;
            !nat_eq(a1336, a1337)
        } {
            Ok(f213(f81(Str::lit("a character reference must be &#digits; or &#xhex; (XML 1.0 [66])"), v1)?))
        } else {
            if f289({
                let (_, h1338) = v4;
                h1338
            })? {
                Ok(Ok::<(Bytes, u64), Bytes>((f295({
                    let (_, h1339) = v4;
                    h1339
                })?, {
                    let a1341 = {
                        let (h1340, _) = v4;
                        h1340
                    };
                    let a1342 = 1u64;
                    nat_add(a1341, a1342)?
                })))
            } else {
                Ok(f213(f81(Str::lit("a character reference must name a Char (XML 1.0 [66] WFC: Legal Character)"), v1)?))
            }
        }
    } else {
        if f84(v0.clone(), v1, Str::lit("&lt;")) {
            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("<")), {
                let a1343 = v1;
                let a1344 = 4u64;
                nat_add(a1343, a1344)?
            })))
        } else {
            if f84(v0.clone(), v1, Str::lit("&gt;")) {
                Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit(">")), {
                    let a1345 = v1;
                    let a1346 = 4u64;
                    nat_add(a1345, a1346)?
                })))
            } else {
                if f84(v0.clone(), v1, Str::lit("&amp;")) {
                    Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("&")), {
                        let a1347 = v1;
                        let a1348 = 5u64;
                        nat_add(a1347, a1348)?
                    })))
                } else {
                    if f84(v0.clone(), v1, Str::lit("&apos;")) {
                        Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("'")), {
                            let a1349 = v1;
                            let a1350 = 6u64;
                            nat_add(a1349, a1350)?
                        })))
                    } else {
                        if f84(v0, v1, Str::lit("&quot;")) {
                            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("\"")), {
                                let a1351 = v1;
                                let a1352 = 6u64;
                                nat_add(a1351, a1352)?
                            })))
                        } else {
                            Ok(f213(f81(Str::lit("an entity reference names an entity that is not declared, and only the five predefined entities are (XML 1.0 section 4.1, WFC: Entity Declared)"), v1)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f215(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<u64> {
    let (h1353, _) = f128(Fn4::F296(v1, v2, v0.clone()), {
        let a1355 = {
            let a1354 = v0;
            length_bytes(a1354)
        };
        let a1356 = 1u64;
        nat_add(a1355, a1356)?
    }, v3)?;
    Ok(h1353)
}

pub fn f216(p1369: Adt16, v1: List<(Bytes, Bytes)>) -> R<Result<Option<Adt21>, Bytes>> {
    let m1368 = p1369;
    {
        let mut m1368 = m1368;
        loop {
            let step = {
                let v0 = m1368;
                let m1357 = v0;
                match m1357 {
                    Adt16::C0(_, _, _) => {
                        Err::<Adt16, R<Result<Option<Adt21>, Bytes>>>(Ok(Ok::<Option<Adt21>, Bytes>(None::<Adt21>)))
                    }
                    Adt16::C1(_, _, _) => {
                        Err::<Adt16, R<Result<Option<Adt21>, Bytes>>>(Ok(Ok::<Option<Adt21>, Bytes>(None::<Adt21>)))
                    }
                    Adt16::C2(_) => {
                        Err::<Adt16, R<Result<Option<Adt21>, Bytes>>>(Ok(Ok::<Option<Adt21>, Bytes>(None::<Adt21>)))
                    }
                    Adt16::C3(r1366, r1367) => {
                        let v2 = (*r1366).clone();
                        let v3 = (*r1367).clone();
                        let m1358 = v2;
                        match m1358 {
                            Adt16::C0(v4, r1364, r1365) => {
                                let v5 = (*r1364).clone();
                                let v6 = (*r1365).clone();
                                let m1359 = f88(v5.clone(), v1.clone());
                                match m1359 {
                                    Ok(v8) => {
                                        let m1360 = f89(v8.clone(), v4)?;
                                        match m1360 {
                                            Ok(v10) => {
                                                if f172({
                                                    let (_, h1361) = v10.clone();
                                                    h1361
                                                }, List::cons(f8(Str::lit("restriction")), List::cons(f8(Str::lit("list")), List::cons(f8(Str::lit("union")), List::<Bytes>::nil())))) {
                                                    if f35({
                                                        let (h1362, _) = v10;
                                                        h1362
                                                    }, f149()) {
                                                        Err::<Adt16, R<Result<Option<Adt21>, Bytes>>>(Ok(Ok::<Option<Adt21>, Bytes>(Some(Adt21::C0({
                                                            let (_, h1363) = v10;
                                                            h1363
                                                        }, v5, v6, v8)))))
                                                    } else {
                                                        Ok::<Adt16, R<Result<Option<Adt21>, Bytes>>>(v3)
                                                    }
                                                } else {
                                                    Ok::<Adt16, R<Result<Option<Adt21>, Bytes>>>(v3)
                                                }
                                            }
                                            Err(v9) => {
                                                Err::<Adt16, R<Result<Option<Adt21>, Bytes>>>(Ok(f297(v9)))
                                            }
                                        }
                                    }
                                    Err(v7) => {
                                        Err::<Adt16, R<Result<Option<Adt21>, Bytes>>>(Ok(f297(v7)))
                                    }
                                }
                            }
                            Adt16::C1(_, _, _) => {
                                Ok::<Adt16, R<Result<Option<Adt21>, Bytes>>>(v3)
                            }
                            Adt16::C2(_) => {
                                Ok::<Adt16, R<Result<Option<Adt21>, Bytes>>>(v3)
                            }
                            Adt16::C3(_, _) => {
                                Ok::<Adt16, R<Result<Option<Adt21>, Bytes>>>(v3)
                            }
                            Adt16::C4 => {
                                Ok::<Adt16, R<Result<Option<Adt21>, Bytes>>>(v3)
                            }
                        }
                    }
                    Adt16::C4 => {
                        Err::<Adt16, R<Result<Option<Adt21>, Bytes>>>(Ok(Ok::<Option<Adt21>, Bytes>(None::<Adt21>)))
                    }
                }
            };
            match step {
                Ok(next) => { m1368 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f217(v0: Bytes) -> Result<Adt20, Bytes> {
    Err::<Adt20, Bytes>(v0)
}

pub fn f218(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit(" at simpleType ")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f219(v0: List<(Bytes, Bytes)>, v1: Bytes, v2: Bytes) -> R<Result<Adt20, Bytes>> {
    let m1370 = f89(v0, v2.clone())?;
    match m1370 {
        Ok(v4) => {
            if f35({
                let (h1371, _) = v4.clone();
                h1371
            }, f149()) {
                let m1373 = f298({
                    let (_, h1372) = v4;
                    h1372
                }, 16u64)?;
                match m1373 {
                    Ok(v6) => {
                        Ok(Ok::<Adt20, Bytes>(Adt20::C0(v6, List::<Bytes>::nil())))
                    }
                    Err(v5) => {
                        Ok(f217(v5))
                    }
                }
            } else {
                if f35({
                    let (h1374, _) = v4.clone();
                    h1374
                }, v1) {
                    let v7: Bytes = f152({
                        let (_, h1375) = v4;
                        h1375
                    })?;
                    Ok(Ok::<Adt20, Bytes>(Adt20::C0(v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                } else {
                    Ok(f217(f7(List::cons(f8(Str::lit("the type ")), List::cons(v2, List::cons(f8(Str::lit(" is in the namespace ")), List::cons({
                        let (h1376, _) = v4;
                        h1376
                    }, List::cons(f8(Str::lit(", which is neither XML Schema's nor this document's target namespace; imported schema documents are not read yet")), List::<Bytes>::nil()))))))))
                }
            }
        }
        Err(v3) => {
            Ok(f217(v3))
        }
    }
}

pub fn f220(v0: Adt16, v1: List<(Bytes, Bytes)>) -> R<Result<List<Adt17>, Bytes>> {
    let m1377 = v0;
    match m1377 {
        Adt16::C0(_, _, _) => {
            Ok(Ok::<List<Adt17>, Bytes>(List::<Adt17>::nil()))
        }
        Adt16::C1(_, _, _) => {
            Ok(Ok::<List<Adt17>, Bytes>(List::<Adt17>::nil()))
        }
        Adt16::C2(_) => {
            Ok(Ok::<List<Adt17>, Bytes>(List::<Adt17>::nil()))
        }
        Adt16::C3(r1384, r1385) => {
            let v2 = (*r1384).clone();
            let v3 = (*r1385).clone();
            let m1378 = f220(v3, v1.clone())?;
            match m1378 {
                Ok(v5) => {
                    let m1379 = v2;
                    match m1379 {
                        Adt16::C0(v6, r1383, _) => {
                            let v7 = (*r1383).clone();
                            let m1380 = f88(v7.clone(), v1);
                            match m1380 {
                                Ok(v10) => {
                                    let m1381 = f89(v10, v6)?;
                                    match m1381 {
                                        Ok(v12) => {
                                            if f90(v12, Str::lit("enumeration")) {
                                                let m1382 = f91(v7, f8(Str::lit("value")));
                                                match m1382 {
                                                    None => {
                                                        Ok(f299(f8(Str::lit("an enumeration facet must have a value attribute (XSD 1.1 Part 2, section 4.3.5.2)"))))
                                                    }
                                                    Some(v13) => {
                                                        Ok(Ok::<List<Adt17>, Bytes>(List::cons(Adt17::C3(v13), v5)))
                                                    }
                                                }
                                            } else {
                                                Ok(Ok::<List<Adt17>, Bytes>(v5))
                                            }
                                        }
                                        Err(v11) => {
                                            Ok(f299(v11))
                                        }
                                    }
                                }
                                Err(v9) => {
                                    Ok(f299(v9))
                                }
                            }
                        }
                        Adt16::C1(_, _, _) => {
                            Ok(Ok::<List<Adt17>, Bytes>(v5))
                        }
                        Adt16::C2(_) => {
                            Ok(Ok::<List<Adt17>, Bytes>(v5))
                        }
                        Adt16::C3(_, _) => {
                            Ok(Ok::<List<Adt17>, Bytes>(v5))
                        }
                        Adt16::C4 => {
                            Ok(Ok::<List<Adt17>, Bytes>(v5))
                        }
                    }
                }
                Err(v4) => {
                    Ok(f299(v4))
                }
            }
        }
        Adt16::C4 => {
            Ok(Ok::<List<Adt17>, Bytes>(List::<Adt17>::nil()))
        }
    }
}

pub fn f221(v0: List<Adt17>, v1: List<Bytes>) -> R<List<Adt17>> {
    let m1386 = v0;
    match m1386.uncons() {
        None => {
            Ok(List::<Adt17>::nil())
        }
        Some((v2, v3)) => {
            let v4: Bytes = f57(v2.clone())?;
            if f113(v4.clone(), v1.clone()) {
                f221(v3, v1)
            } else {
                Ok(List::cons(v2, f221(v3, List::cons(v4, v1))?))
            }
        }
    }
}

pub fn f222(v0: List<Adt17>) -> List<Bytes> {
    let m1387 = v0;
    match m1387.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1388 = f300(Some(v1));
            match m1388 {
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

pub fn f223(v0: Bytes, p1409: u64, p1410: u64, p1411: bool, p1412: Bytes) -> R<Bytes> {
    let m1408 = (p1409, (p1410, (p1411, p1412)));
    {
        let mut m1408 = m1408;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1408;
                let m1389 = v2;
                if m1389 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1389 - 1;
                    let m1392 = {
                        let a1390 = v0.clone();
                        let a1391 = v1;
                        index_bytes(a1390, a1391)
                    };
                    match m1392 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f41(v6)?;
                            if f301(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1393 = v1;
                                    let a1394 = 1u64;
                                    nat_add(a1393, a1394)?
                                }, (v5, (true, v4))))
                            } else {
                                if f302(f303(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1395 = v1;
                                        let a1396 = 1u64;
                                        nat_add(a1395, a1396)?
                                    }, (v5, (false, {
                                        let a1400 = v4;
                                        let a1401 = f32({
                                            let a1397 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1397)
                                        }, {
                                            let a1398 = v7;
                                            let a1399 = 97u64;
                                            nat_sub(a1398, a1399)
                                        }, 1u64);
                                        append_bytes(a1400, a1401)
                                    }))))
                                } else {
                                    if f304(f303(v7), f304(f305(v7), f306(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1402 = v1;
                                            let a1403 = 1u64;
                                            nat_add(a1402, a1403)?
                                        }, (v5, (false, {
                                            let a1404 = v4;
                                            let a1405 = f32(v0.clone(), v1, 1u64);
                                            append_bytes(a1404, a1405)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1406 = v1;
                                            let a1407 = 1u64;
                                            nat_add(a1406, a1407)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1408 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f224(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1413 = f288(v0, v1)?;
    match m1413 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f307({
                let (h1414, _) = v2;
                h1414
            }) {
                Ok(Some({
                    let a1416 = v1;
                    let a1417 = {
                        let (_, h1415) = v2;
                        h1415
                    };
                    nat_add(a1416, a1417)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f225(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1419 = v1;
    let a1420 = {
        let a1418 = v0.clone();
        length_bytes(a1418)
    };
    if if nat_lt(a1419, a1420) {
        f308(v0.clone(), v1)?
    } else {
        false
    } {
        Ok(Some(f167(v0, v1)?))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f226(v0: Bytes, v1: u64) -> R<u64> {
    let (h1421, _) = f128(Fn4::F309(v0), {
        let a1422 = v1;
        let a1423 = 1u64;
        nat_add(a1422, a1423)?
    }, v1)?;
    Ok(h1421)
}

pub fn f227(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f310(v0, v1)
}

pub fn f228(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f311(v0, v1)
}

pub fn f229(v0: u64, v1: bool) -> bool {
    f169(if if f157(v0) {
        true
    } else {
        f158(v0)
    } {
        true
    } else {
        if v1 {
            f312(v0)
        } else {
            false
        }
    })
}

pub fn f230(v0: Bytes, v1: u64, v2: u64) -> R<Adt18> {
    let v3: bool = {
        let a1424 = f100(v0.clone(), v1)?;
        let a1425 = 62u64;
        nat_eq(a1424, a1425)
    };
    let v4: u64 = f100(v0.clone(), {
        let a1426 = v1;
        let a1427 = 1u64;
        nat_add(a1426, a1427)?
    })?;
    let v5: u64 = f100(v0.clone(), {
        let a1428 = v1;
        let a1429 = 2u64;
        nat_add(a1428, a1429)?
    })?;
    let v6: u64 = f313(v4);
    let v7: u64 = f314(v4);
    let v8: bool = {
        let a1430 = 0u64;
        let a1431 = v6;
        if nat_lt(a1430, a1431) {
            true
        } else {
            let a1432 = 0u64;
            let a1433 = v7;
            nat_lt(a1432, a1433)
        }
    };
    let v9: u64 = {
        let a1434 = 0u64;
        let a1435 = v7;
        if nat_lt(a1434, a1435) {
            f313(v5)
        } else {
            0u64
        }
    };
    let v10: u64 = {
        let a1436 = 0u64;
        let a1437 = v6;
        if nat_lt(a1436, a1437) {
            f314(v5)
        } else {
            0u64
        }
    };
    let v11: u64 = {
        let a1438 = v6;
        let a1439 = v9;
        nat_add(a1438, a1439)?
    };
    let v12: u64 = {
        let a1440 = v7;
        let a1441 = v10;
        nat_add(a1440, a1441)?
    };
    let v13: u64 = {
        let a1450 = {
            let a1442 = v1;
            let a1443 = 1u64;
            nat_add(a1442, a1443)?
        };
        let a1451 = {
            let a1448 = if v8 {
                1u64
            } else {
                0u64
            };
            let a1449 = {
                let a1444 = 0u64;
                let a1445 = v9;
                if if nat_lt(a1444, a1445) {
                    true
                } else {
                    let a1446 = 0u64;
                    let a1447 = v10;
                    nat_lt(a1446, a1447)
                } {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a1448, a1449)?
        };
        nat_add(a1450, a1451)?
    };
    let m1452 = f173(v0.clone(), v13)?;
    match m1452 {
        Ok(v15) => {
            let a1453 = 0u64;
            let a1454 = v12;
            if nat_lt(a1453, a1454) {
                f315(v0, v15, {
                    let a1457 = {
                        let a1455 = v2;
                        let a1456 = v12;
                        nat_add(a1455, a1456)?
                    };
                    let a1458 = 1u64;
                    nat_sub(a1457, a1458)
                }, v3, v11)
            } else {
                let v16: (u64, u64) = {
                    let (h1459, _) = f316(Fn7::F317(v0.clone()), {
                        let a1461 = {
                            let a1460 = v0.clone();
                            length_bytes(a1460)
                        };
                        let a1462 = 1u64;
                        nat_add(a1461, a1462)?
                    }, (v15, 0u64))?;
                    h1459
                };
                let v17: u64 = f160(v0.clone(), {
                    let (h1463, _) = v16;
                    h1463
                })?;
                let a1464 = f100(v0.clone(), v17)?;
                let a1465 = 256u64;
                if nat_eq(a1464, a1465) {
                    Ok(Adt18::C0(Adt17::C3({
                        let a1466 = v11;
                        let a1467 = 2u64;
                        if nat_eq(a1466, a1467) {
                            f318(f319(v0.clone(), v15)?)
                        } else {
                            Bytes::lit(&[])
                        }
                    }), {
                        let a1468 = v0;
                        length_bytes(a1468)
                    }))
                } else {
                    let v18: u64 = f320({
                        let a1470 = v17;
                        let a1471 = {
                            let (h1469, _) = v16;
                            h1469
                        };
                        nat_sub(a1470, a1471)
                    }, v2);
                    let a1473 = v18;
                    let a1474 = {
                        let (_, h1472) = v16;
                        h1472
                    };
                    if nat_lt(a1473, a1474) {
                        Ok(Adt18::C1(f81(Str::lit("a leading empty line of a block scalar holds more spaces than its first content line (YAML 1.2.2 section 8.1.1.1)"), v15)?))
                    } else {
                        f315(v0, v15, v18, v3, v11)
                    }
                }
            }
        }
        Err(v14) => {
            Ok(Adt18::C1(v14))
        }
    }
}

pub fn f231(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Adt22> {
    let a1475 = f100(v0.clone(), v1)?;
    let a1476 = 34u64;
    if nat_eq(a1475, a1476) {
        f321(v0.clone(), {
            let a1477 = v1;
            let a1478 = 1u64;
            nat_add(a1477, a1478)?
        }, v2, v3, {
            let a1480 = {
                let a1479 = v0;
                length_bytes(a1479)
            };
            let a1481 = 2u64;
            nat_add(a1480, a1481)?
        }, Bytes::lit(&[]))
    } else {
        f322(v0.clone(), {
            let a1482 = v1;
            let a1483 = 1u64;
            nat_add(a1482, a1483)?
        }, v2, v3, {
            let a1485 = {
                let a1484 = v0;
                length_bytes(a1484)
            };
            let a1486 = 2u64;
            nat_add(a1485, a1486)?
        }, Bytes::lit(&[]))
    }
}

pub fn f232(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: List<Bytes>) -> R<Adt18> {
    let m1487 = v3;
    if m1487 == 0 {
        Ok(Adt18::C1(f81(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v6 = m1487 - 1;
        let a1488 = v4;
        let a1489 = 1u64;
        if nat_eq(a1488, a1489) {
            let m1490 = f232(v0.clone(), v1, v2, v6, 0u64, List::<Bytes>::nil())?;
            match m1490 {
                Adt18::C0(v8, v9) => {
                    let m1491 = f323(v0.clone(), v9, v2)?;
                    match m1491 {
                        Ok(v11) => {
                            let v12: u64 = f100(v0.clone(), v11)?;
                            let a1492 = v12;
                            let a1493 = 58u64;
                            if nat_eq(a1492, a1493) {
                                Ok(Adt18::C1(f81(Str::lit("a single-pair mapping inside a flow sequence is not read (YAML 1.2.2 [150])"), v11)?))
                            } else {
                                let a1494 = v12;
                                let a1495 = 93u64;
                                if nat_eq(a1494, a1495) {
                                    Ok(Adt18::C0(Adt17::C7(std::rc::Rc::new(v8), std::rc::Rc::new(Adt17::C6)), {
                                        let a1496 = v11;
                                        let a1497 = 1u64;
                                        nat_add(a1496, a1497)?
                                    }))
                                } else {
                                    let a1498 = v12;
                                    let a1499 = 44u64;
                                    if nat_eq(a1498, a1499) {
                                        let m1502 = f323(v0.clone(), {
                                            let a1500 = v11;
                                            let a1501 = 1u64;
                                            nat_add(a1500, a1501)?
                                        }, v2)?;
                                        match m1502 {
                                            Ok(v14) => {
                                                let a1503 = f100(v0.clone(), v14)?;
                                                let a1504 = 93u64;
                                                if nat_eq(a1503, a1504) {
                                                    Ok(Adt18::C0(Adt17::C7(std::rc::Rc::new(v8), std::rc::Rc::new(Adt17::C6)), {
                                                        let a1505 = v14;
                                                        let a1506 = 1u64;
                                                        nat_add(a1505, a1506)?
                                                    }))
                                                } else {
                                                    let m1507 = f232(v0, v14, v2, v6, 1u64, List::<Bytes>::nil())?;
                                                    match m1507 {
                                                        Adt18::C0(v16, v17) => {
                                                            Ok(Adt18::C0(Adt17::C7(std::rc::Rc::new(v8), std::rc::Rc::new(v16)), v17))
                                                        }
                                                        Adt18::C1(v15) => {
                                                            Ok(Adt18::C1(v15))
                                                        }
                                                    }
                                                }
                                            }
                                            Err(v13) => {
                                                Ok(Adt18::C1(v13))
                                            }
                                        }
                                    } else {
                                        Ok(Adt18::C1(f81(Str::lit("a flow sequence entry must be followed by , or ] (YAML 1.2.2 [137])"), v11)?))
                                    }
                                }
                            }
                        }
                        Err(v10) => {
                            Ok(Adt18::C1(v10))
                        }
                    }
                }
                Adt18::C1(v7) => {
                    Ok(Adt18::C1(v7))
                }
            }
        } else {
            let a1508 = v4;
            let a1509 = 2u64;
            if nat_eq(a1508, a1509) {
                let m1510 = f324(v0.clone(), v1, v2)?;
                match m1510 {
                    Adt22::C0(v19, v20) => {
                        let m1511 = f323(v0.clone(), v20, v2)?;
                        match m1511 {
                            Ok(v22) => {
                                if f169({
                                    let a1512 = f100(v0.clone(), v22)?;
                                    let a1513 = 58u64;
                                    nat_eq(a1512, a1513)
                                }) {
                                    Ok(Adt18::C1(f81(Str::lit("a flow mapping entry needs : and a value; a null value is not read (YAML 1.2.2 [142])"), v22)?))
                                } else {
                                    let m1516 = f323(v0.clone(), {
                                        let a1514 = v22;
                                        let a1515 = 1u64;
                                        nat_add(a1514, a1515)?
                                    }, v2)?;
                                    match m1516 {
                                        Ok(v24) => {
                                            let a1517 = f100(v0.clone(), v24)?;
                                            let a1518 = 44u64;
                                            if if nat_eq(a1517, a1518) {
                                                true
                                            } else {
                                                let a1519 = f100(v0.clone(), v24)?;
                                                let a1520 = 125u64;
                                                nat_eq(a1519, a1520)
                                            } {
                                                Ok(Adt18::C1(f168(v24)?))
                                            } else {
                                                let m1521 = f232(v0.clone(), v24, v2, v6, 0u64, List::<Bytes>::nil())?;
                                                match m1521 {
                                                    Adt18::C0(v26, v27) => {
                                                        if f172(v19.clone(), v5.clone()) {
                                                            Ok(Adt18::C1(f81(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                                        } else {
                                                            let m1522 = f323(v0.clone(), v27, v2)?;
                                                            match m1522 {
                                                                Ok(v29) => {
                                                                    let v30: u64 = f100(v0.clone(), v29)?;
                                                                    let a1523 = v30;
                                                                    let a1524 = 125u64;
                                                                    if nat_eq(a1523, a1524) {
                                                                        Ok(Adt18::C0(Adt17::C8(v19, std::rc::Rc::new(v26), std::rc::Rc::new(Adt17::C6)), {
                                                                            let a1525 = v29;
                                                                            let a1526 = 1u64;
                                                                            nat_add(a1525, a1526)?
                                                                        }))
                                                                    } else {
                                                                        let a1527 = v30;
                                                                        let a1528 = 44u64;
                                                                        if nat_eq(a1527, a1528) {
                                                                            let m1531 = f323(v0.clone(), {
                                                                                let a1529 = v29;
                                                                                let a1530 = 1u64;
                                                                                nat_add(a1529, a1530)?
                                                                            }, v2)?;
                                                                            match m1531 {
                                                                                Ok(v32) => {
                                                                                    let a1532 = f100(v0.clone(), v32)?;
                                                                                    let a1533 = 125u64;
                                                                                    if nat_eq(a1532, a1533) {
                                                                                        Ok(Adt18::C0(Adt17::C8(v19, std::rc::Rc::new(v26), std::rc::Rc::new(Adt17::C6)), {
                                                                                            let a1534 = v32;
                                                                                            let a1535 = 1u64;
                                                                                            nat_add(a1534, a1535)?
                                                                                        }))
                                                                                    } else {
                                                                                        let m1536 = f232(v0, v32, v2, v6, 2u64, List::cons(v19.clone(), v5))?;
                                                                                        match m1536 {
                                                                                            Adt18::C0(v34, v35) => {
                                                                                                Ok(Adt18::C0(Adt17::C8(v19, std::rc::Rc::new(v26), std::rc::Rc::new(v34)), v35))
                                                                                            }
                                                                                            Adt18::C1(v33) => {
                                                                                                Ok(Adt18::C1(v33))
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                Err(v31) => {
                                                                                    Ok(Adt18::C1(v31))
                                                                                }
                                                                            }
                                                                        } else {
                                                                            Ok(Adt18::C1(f81(Str::lit("a flow mapping entry must be followed by , or } (YAML 1.2.2 [140])"), v29)?))
                                                                        }
                                                                    }
                                                                }
                                                                Err(v28) => {
                                                                    Ok(Adt18::C1(v28))
                                                                }
                                                            }
                                                        }
                                                    }
                                                    Adt18::C1(v25) => {
                                                        Ok(Adt18::C1(v25))
                                                    }
                                                }
                                            }
                                        }
                                        Err(v23) => {
                                            Ok(Adt18::C1(v23))
                                        }
                                    }
                                }
                            }
                            Err(v21) => {
                                Ok(Adt18::C1(v21))
                            }
                        }
                    }
                    Adt22::C1(v18) => {
                        Ok(Adt18::C1(v18))
                    }
                }
            } else {
                let v36: u64 = f100(v0.clone(), v1)?;
                let a1537 = v36;
                let a1538 = 91u64;
                if if nat_eq(a1537, a1538) {
                    true
                } else {
                    let a1539 = v36;
                    let a1540 = 123u64;
                    nat_eq(a1539, a1540)
                } {
                    let m1543 = f323(v0.clone(), {
                        let a1541 = v1;
                        let a1542 = 1u64;
                        nat_add(a1541, a1542)?
                    }, v2)?;
                    match m1543 {
                        Ok(v38) => {
                            let a1546 = f100(v0.clone(), v38)?;
                            let a1547 = {
                                let a1544 = v36;
                                let a1545 = 2u64;
                                nat_add(a1544, a1545)?
                            };
                            if nat_eq(a1546, a1547) {
                                Ok(Adt18::C0({
                                    let a1548 = v36;
                                    let a1549 = 91u64;
                                    if nat_eq(a1548, a1549) {
                                        Adt17::C4(std::rc::Rc::new(Adt17::C6))
                                    } else {
                                        Adt17::C5(std::rc::Rc::new(Adt17::C6))
                                    }
                                }, {
                                    let a1550 = v38;
                                    let a1551 = 1u64;
                                    nat_add(a1550, a1551)?
                                }))
                            } else {
                                let m1554 = f232(v0, v38, v2, v6, {
                                    let a1552 = v36;
                                    let a1553 = 91u64;
                                    if nat_eq(a1552, a1553) {
                                        1u64
                                    } else {
                                        2u64
                                    }
                                }, List::<Bytes>::nil())?;
                                match m1554 {
                                    Adt18::C0(v40, v41) => {
                                        Ok(Adt18::C0({
                                            let a1555 = v36;
                                            let a1556 = 91u64;
                                            if nat_eq(a1555, a1556) {
                                                Adt17::C4(std::rc::Rc::new(v40))
                                            } else {
                                                Adt17::C5(std::rc::Rc::new(v40))
                                            }
                                        }, v41))
                                    }
                                    Adt18::C1(v39) => {
                                        Ok(Adt18::C1(v39))
                                    }
                                }
                            }
                        }
                        Err(v37) => {
                            Ok(Adt18::C1(v37))
                        }
                    }
                } else {
                    let a1557 = v36;
                    let a1558 = 34u64;
                    if if nat_eq(a1557, a1558) {
                        true
                    } else {
                        let a1559 = v36;
                        let a1560 = 39u64;
                        nat_eq(a1559, a1560)
                    } {
                        let m1561 = f231(v0, v1, v2, false)?;
                        match m1561 {
                            Adt22::C0(v43, v44) => {
                                Ok(Adt18::C0(Adt17::C3(v43), v44))
                            }
                            Adt22::C1(v42) => {
                                Ok(Adt18::C1(v42))
                            }
                        }
                    } else {
                        let m1562 = f164(v0.clone(), v1)?;
                        match m1562 {
                            None => {
                                if f169(f233(v0.clone(), v1, true)?) {
                                    Ok(Adt18::C1(f81(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                                } else {
                                    let v46: u64 = f234(v0.clone(), v1, true)?;
                                    let v47: u64 = f161(v0.clone(), v46)?;
                                    let m1565 = {
                                        let a1563 = f100(v0.clone(), v47)?;
                                        let a1564 = 10u64;
                                        if nat_eq(a1563, a1564) {
                                            f323(v0.clone(), v47, v2)?
                                        } else {
                                            Ok::<u64, Bytes>(v47)
                                        }
                                    };
                                    match m1565 {
                                        Ok(v49) => {
                                            let a1566 = f100(v0.clone(), v47)?;
                                            let a1567 = 10u64;
                                            if if if nat_eq(a1566, a1567) {
                                                f169(f312(f100(v0.clone(), v49)?))
                                            } else {
                                                false
                                            } {
                                                f169({
                                                    let a1568 = f100(v0.clone(), v49)?;
                                                    let a1569 = 58u64;
                                                    nat_eq(a1568, a1569)
                                                })
                                            } else {
                                                false
                                            } {
                                                Ok(Adt18::C1(f81(Str::lit("a multi-line plain scalar inside a flow collection is not read (YAML 1.2.2 [135])"), v47)?))
                                            } else {
                                                f237(f236(v0, v1, v46), v1, v46)
                                            }
                                        }
                                        Err(v48) => {
                                            Ok(Adt18::C1(v48))
                                        }
                                    }
                                }
                            }
                            Some(v45) => {
                                Ok(Adt18::C1(v45))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f233(v0: Bytes, v1: u64, v2: bool) -> R<bool> {
    let v3: u64 = f100(v0.clone(), v1)?;
    if if f157(v3) {
        true
    } else {
        f158(v3)
    } {
        Ok(false)
    } else {
        let a1570 = v3;
        let a1571 = 45u64;
        if if if nat_eq(a1570, a1571) {
            true
        } else {
            let a1572 = v3;
            let a1573 = 63u64;
            nat_eq(a1572, a1573)
        } {
            true
        } else {
            let a1574 = v3;
            let a1575 = 58u64;
            nat_eq(a1574, a1575)
        } {
            Ok(f229(f100(v0, {
                let a1576 = v1;
                let a1577 = 1u64;
                nat_add(a1576, a1577)?
            })?, v2))
        } else {
            Ok(f169(f325(v3)?))
        }
    }
}

pub fn f234(v0: Bytes, v1: u64, v2: bool) -> R<u64> {
    let (_, h1578) = {
        let (h1579, _) = f316(Fn7::F326(v2, v0.clone()), {
            let a1581 = {
                let a1580 = v0;
                length_bytes(a1580)
            };
            let a1582 = 1u64;
            nat_add(a1581, a1582)?
        }, (v1, v1))?;
        h1579
    };
    Ok(h1578)
}

pub fn f235(v0: Bytes, p1607: u64, v2: u64, p1608: u64, p1609: Bytes) -> R<Adt22> {
    let m1606 = (p1607, (p1608, p1609));
    {
        let mut m1606 = m1606;
        loop {
            let step = {
                let (v1, (v3, v4)) = m1606;
                let m1583 = v3;
                if m1583 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C1(f81(Str::lit("a plain scalar has more lines than the input"), v1)?)))
                } else {
                    let v5 = m1583 - 1;
                    let v6: u64 = f161(v0.clone(), v1)?;
                    let v7: u64 = f100(v0.clone(), v6)?;
                    let a1584 = v7;
                    let a1585 = 58u64;
                    if nat_eq(a1584, a1585) {
                        Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C1(f327(v6)?)))
                    } else {
                        let a1586 = v7;
                        let a1587 = 35u64;
                        if if nat_eq(a1586, a1587) {
                            true
                        } else {
                            let a1588 = v7;
                            let a1589 = 256u64;
                            nat_eq(a1588, a1589)
                        } {
                            Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C0(v4, f167(v0.clone(), v6)?)))
                        } else {
                            let v8: (u64, u64) = f328(v0.clone(), {
                                let a1590 = v6;
                                let a1591 = 1u64;
                                nat_add(a1590, a1591)?
                            })?;
                            let v9: u64 = {
                                let (h1592, _) = v8;
                                h1592
                            };
                            let v10: u64 = f160(v0.clone(), v9)?;
                            let v11: u64 = f161(v0.clone(), v10)?;
                            let v12: u64 = f100(v0.clone(), v11)?;
                            let a1593 = v12;
                            let a1594 = 256u64;
                            if if if if nat_eq(a1593, a1594) {
                                true
                            } else {
                                let a1597 = {
                                    let a1595 = v10;
                                    let a1596 = v9;
                                    nat_sub(a1595, a1596)
                                };
                                let a1598 = v2;
                                nat_lt(a1597, a1598)
                            } {
                                true
                            } else {
                                let a1599 = v12;
                                let a1600 = 35u64;
                                nat_eq(a1599, a1600)
                            } {
                                true
                            } else {
                                f99(v0.clone(), v9)?
                            } {
                                Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C0(v4, {
                                    let a1601 = v6;
                                    let a1602 = 1u64;
                                    nat_add(a1601, a1602)?
                                })))
                            } else {
                                let v13: u64 = f234(v0.clone(), v11, false)?;
                                let a1603 = v13;
                                let a1604 = v11;
                                if nat_eq(a1603, a1604) {
                                    Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C1(f327(v11)?)))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Adt22>>((v13, (v5, f7(List::cons(v4, List::cons(f329({
                                        let (_, h1605) = v8;
                                        h1605
                                    }), List::cons(f236(v0.clone(), v11, v13), List::<Bytes>::nil())))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1606 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f236(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    f32(v0, v1, {
        let a1610 = v2;
        let a1611 = v1;
        nat_sub(a1610, a1611)
    })
}

pub fn f237(v0: Bytes, v1: u64, v2: u64) -> R<Adt18> {
    let a1613 = {
        let a1612 = v0.clone();
        length_bytes(a1612)
    };
    let a1614 = 0u64;
    if if nat_eq(a1613, a1614) {
        true
    } else {
        f172(v0.clone(), List::cons(f8(Str::lit("~")), List::cons(f8(Str::lit("null")), List::cons(f8(Str::lit("Null")), List::cons(f8(Str::lit("NULL")), List::<Bytes>::nil())))))
    } {
        Ok(Adt18::C1(f168(v1)?))
    } else {
        if f172(v0.clone(), List::cons(f8(Str::lit("true")), List::cons(f8(Str::lit("True")), List::cons(f8(Str::lit("TRUE")), List::<Bytes>::nil())))) {
            Ok(Adt18::C0(Adt17::C0(true), v2))
        } else {
            if f172(v0.clone(), List::cons(f8(Str::lit("false")), List::cons(f8(Str::lit("False")), List::cons(f8(Str::lit("FALSE")), List::<Bytes>::nil())))) {
                Ok(Adt18::C0(Adt17::C0(false), v2))
            } else {
                if f172(v0.clone(), List::cons(f8(Str::lit(".inf")), List::cons(f8(Str::lit(".Inf")), List::cons(f8(Str::lit(".INF")), List::cons(f8(Str::lit("+.inf")), List::cons(f8(Str::lit("+.Inf")), List::cons(f8(Str::lit("+.INF")), List::cons(f8(Str::lit("-.inf")), List::cons(f8(Str::lit("-.Inf")), List::cons(f8(Str::lit("-.INF")), List::cons(f8(Str::lit(".nan")), List::cons(f8(Str::lit(".NaN")), List::cons(f8(Str::lit(".NAN")), List::<Bytes>::nil()))))))))))))) {
                    Ok(Adt18::C1(f81(Str::lit("an infinite or not-a-number float is not read: Studio.Value has none (YAML 1.2.2 section 10.3.2)"), v1)?))
                } else {
                    if if f84(v0.clone(), 0u64, Str::lit("0o")) {
                        let a1616 = 2u64;
                        let a1617 = {
                            let a1615 = v0.clone();
                            length_bytes(a1615)
                        };
                        nat_lt(a1616, a1617)
                    } else {
                        false
                    } {
                        f330(v0, 8u64, v1, v2)
                    } else {
                        if if f84(v0.clone(), 0u64, Str::lit("0x")) {
                            let a1619 = 2u64;
                            let a1620 = {
                                let a1618 = v0.clone();
                                length_bytes(a1618)
                            };
                            nat_lt(a1619, a1620)
                        } else {
                            false
                        } {
                            f330(v0, 16u64, v1, v2)
                        } else {
                            let m1621 = f331(v0.clone())?;
                            match m1621 {
                                None => {
                                    Ok(Adt18::C0(Adt17::C3(v0), v2))
                                }
                                Some(v3) => {
                                    let m1622 = f332(v3.clone(), 0u64)?;
                                    match m1622 {
                                        None => {
                                            Ok(Adt18::C1(f333(v1)?))
                                        }
                                        Some(v4) => {
                                            let a1625 = {
                                                let (h1623, _) = v4;
                                                h1623
                                            };
                                            let a1626 = {
                                                let a1624 = v3;
                                                length_bytes(a1624)
                                            };
                                            if nat_eq(a1625, a1626) {
                                                Ok(Adt18::C0({
                                                    let (_, h1627) = v4;
                                                    h1627
                                                }, v2))
                                            } else {
                                                Ok(Adt18::C1(f333(v1)?))
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

pub fn f238(v0: Bytes, v1: u64) -> R<u64> {
    let (h1628, _) = f128(Fn4::F334(v0.clone()), {
        let a1630 = {
            let a1629 = v0;
            length_bytes(a1629)
        };
        let a1631 = 1u64;
        nat_add(a1630, a1631)?
    }, v1)?;
    Ok(h1628)
}

pub fn f239(v0: Bytes, v1: u64) -> R<bool> {
    let a1632 = f100(v0.clone(), v1)?;
    let a1633 = 58u64;
    if nat_eq(a1632, a1633) {
        if f157(f100(v0.clone(), {
            let a1634 = v1;
            let a1635 = 1u64;
            nat_add(a1634, a1635)?
        })?) {
            Ok(true)
        } else {
            Ok(f158(f100(v0, {
                let a1636 = v1;
                let a1637 = 1u64;
                nat_add(a1636, a1637)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f240(v0: u64) -> Bytes {
    let m1638 = v0;
    if m1638 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1638 - 1;
        let a1639 = f8(Str::lit("0"));
        let a1640 = f240(v1);
        append_bytes(a1639, a1640)
    }
}

pub fn f241(v0: Bytes, p1653: u64, p1654: u64) -> R<u64> {
    let m1652 = (p1653, p1654);
    {
        let mut m1652 = m1652;
        loop {
            let step = {
                let (v1, v2) = m1652;
                let m1641 = v2;
                if m1641 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m1641 - 1;
                    let v4: u64 = f137(v0.clone(), v1)?;
                    let a1642 = v4;
                    let a1643 = 34u64;
                    if nat_eq(a1642, a1643) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a1644 = v4;
                        let a1645 = 92u64;
                        if nat_eq(a1644, a1645) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a1646 = v4;
                            let a1647 = 32u64;
                            if nat_lt(a1646, a1647) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a1648 = v4;
                                let a1649 = 256u64;
                                if nat_eq(a1648, a1649) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a1650 = v1;
                                        let a1651 = 1u64;
                                        nat_add(a1650, a1651)?
                                    }, v3))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1652 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f242(v0: Bytes) -> R<Bytes> {
    let a1660 = {
        let a1658 = Bytes::lit(&[0x22]);
        let a1659 = {
            let (_, h1655) = {
                let (h1656, _) = f125(Fn3::F335(v0.clone()), {
                    let a1657 = v0;
                    length_bytes(a1657)
                }, (0u64, Bytes::lit(&[])))?;
                h1656
            };
            h1655
        };
        append_bytes(a1658, a1659)
    };
    let a1661 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a1660, a1661))
}

pub fn f243(v0: Fn5, p1665: List<(Bytes, Bytes)>, p1666: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1664 = (p1665, p1666);
    {
        let mut m1664 = m1664;
        loop {
            let step = {
                let (v1, v2) = m1664;
                let m1662 = v2;
                match m1662.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1663: Fn5 = v0.clone();
                            c1663.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1664 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f244(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f336(v0, v1)
}

pub fn f245(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f337(v0)
}

pub fn f246(v0: Fn6, p1670: List<Bytes>, p1671: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1669 = (p1670, p1671);
    {
        let mut m1669 = m1669;
        loop {
            let step = {
                let (v1, v2) = m1669;
                let m1667 = v2;
                match m1667.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1668: Fn6 = v0.clone();
                            c1668.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1669 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f247(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f338(v1)?, v0))
}

pub fn f248(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a1672 = v0;
        utf8_encode(a1672)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f51(v1)?, List::<Bytes>::nil())))))
}

pub fn f249(p1675: List<Adt19>, p1676: List<Adt19>) -> List<Adt19> {
    let m1674 = (p1675, p1676);
    {
        let mut m1674 = m1674;
        loop {
            let step = {
                let (v0, v1) = m1674;
                let m1673 = v0;
                match m1673.uncons() {
                    None => {
                        Err::<(List<Adt19>, List<Adt19>), List<Adt19>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt19>, List<Adt19>), List<Adt19>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1674 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f250(v0: u64) -> bool {
    let a1677 = 65u64;
    let a1678 = v0;
    if if nat_le(a1677, a1678) {
        let a1679 = v0;
        let a1680 = 90u64;
        nat_le(a1679, a1680)
    } else {
        false
    } {
        true
    } else {
        let a1681 = 97u64;
        let a1682 = v0;
        if nat_le(a1681, a1682) {
            let a1683 = v0;
            let a1684 = 122u64;
            nat_le(a1683, a1684)
        } else {
            false
        }
    }
}

pub fn f251(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h1685, _) = f128(Fn4::F339(v1, v0.clone()), {
        let a1687 = {
            let a1686 = v0;
            length_bytes(a1686)
        };
        let a1688 = 1u64;
        nat_add(a1687, a1688)?
    }, v2)?;
    Ok(h1685)
}

pub fn f252(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f85(v0.clone(), v1)?;
    let a1689 = v2;
    let a1690 = 48u64;
    if nat_eq(a1689, a1690) {
        let v3: u64 = f85(v0.clone(), {
            let a1691 = v1;
            let a1692 = 1u64;
            nat_add(a1691, a1692)?
        })?;
        let a1693 = v3;
        let a1694 = 88u64;
        if if if nat_eq(a1693, a1694) {
            true
        } else {
            let a1695 = v3;
            let a1696 = 120u64;
            nat_eq(a1695, a1696)
        } {
            f340(f85(v0.clone(), {
                let a1697 = v1;
                let a1698 = 2u64;
                nat_add(a1697, a1698)?
            })?)
        } else {
            false
        } {
            f251(v0, 1u64, {
                let a1699 = v1;
                let a1700 = 2u64;
                nat_add(a1699, a1700)?
            })
        } else {
            f251(v0, 2u64, {
                let a1701 = v1;
                let a1702 = 1u64;
                nat_add(a1701, a1702)?
            })
        }
    } else {
        if f341(v2) {
            f251(v0, 0u64, v1)
        } else {
            Ok(v1)
        }
    }
}

pub fn f253(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f251(v0.clone(), 0u64, v1)?;
    let a1703 = f85(v0.clone(), v2)?;
    let a1704 = 46u64;
    if nat_eq(a1703, a1704) {
        let v3: u64 = f251(v0.clone(), 0u64, {
            let a1705 = v2;
            let a1706 = 1u64;
            nat_add(a1705, a1706)?
        })?;
        let a1707 = v1;
        let a1708 = v2;
        if if nat_lt(a1707, a1708) {
            true
        } else {
            let a1711 = {
                let a1709 = v2;
                let a1710 = 1u64;
                nat_add(a1709, a1710)?
            };
            let a1712 = v3;
            nat_lt(a1711, a1712)
        } {
            f342(v0, v3)
        } else {
            Ok(v1)
        }
    } else {
        let a1713 = v1;
        let a1714 = v2;
        if nat_lt(a1713, a1714) {
            let v4: u64 = f342(v0, v2)?;
            let a1715 = v2;
            let a1716 = v4;
            if nat_lt(a1715, a1716) {
                Ok(v4)
            } else {
                Ok(v1)
            }
        } else {
            Ok(v1)
        }
    }
}

pub fn f254(v0: List<Adt19>, v1: Str) -> bool {
    let m1717 = v0;
    match m1717.uncons() {
        None => {
            false
        }
        Some((v2, _)) => {
            f343(v2, {
                let a1718 = v1;
                utf8_encode(a1718)
            })
        }
    }
}

pub fn f255(p1731: u64, p1732: List<Adt19>, p1733: List<Bytes>, v3: bool) -> Result<List<Adt19>, Bytes> {
    let m1730 = (p1731, (p1732, p1733));
    {
        let mut m1730 = m1730;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1730;
                let m1719 = v0;
                if m1719 == 0 {
                    Err::<(u64, (List<Adt19>, List<Bytes>)), Result<List<Adt19>, Bytes>>(Err::<List<Adt19>, Bytes>(f8(Str::lit("brackets nest deeper than the input is long"))))
                } else {
                    let v4 = m1719 - 1;
                    let m1720 = v1;
                    match m1720.uncons() {
                        None => {
                            Err::<(u64, (List<Adt19>, List<Bytes>)), Result<List<Adt19>, Bytes>>(Err::<List<Adt19>, Bytes>(f8(Str::lit("the input ends inside brackets or before the ; that closes a member (Web IDL, IDL grammar)"))))
                        }
                        Some((v5, v6)) => {
                            let a1721 = f344(v5.clone());
                            let a1722 = 3u64;
                            if nat_eq(a1721, a1722) {
                                Ok::<(u64, (List<Adt19>, List<Bytes>)), Result<List<Adt19>, Bytes>>((v4, (v6, v2)))
                            } else {
                                let v7: Bytes = f345(v5);
                                let v8: Bytes = f346(v7.clone());
                                if !f35(v8.clone(), Bytes::lit(&[])) {
                                    Ok::<(u64, (List<Adt19>, List<Bytes>)), Result<List<Adt19>, Bytes>>((v4, (v6, List::cons(v8, v2))))
                                } else {
                                    if f347(v7.clone()) {
                                        let m1723 = v2;
                                        match m1723.uncons() {
                                            None => {
                                                Err::<(u64, (List<Adt19>, List<Bytes>)), Result<List<Adt19>, Bytes>>(Err::<List<Adt19>, Bytes>(f7(List::cons(f8(Str::lit("the closing bracket ")), List::cons(v7, List::cons(f8(Str::lit(" has no opening bracket (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
                                            }
                                            Some((v9, v10)) => {
                                                if f35(v9.clone(), v7.clone()) {
                                                    if if !v3 {
                                                        let a1725 = {
                                                            let a1724 = v10.clone();
                                                            length_list(a1724)
                                                        };
                                                        let a1726 = 0u64;
                                                        nat_eq(a1725, a1726)
                                                    } else {
                                                        false
                                                    } {
                                                        Err::<(u64, (List<Adt19>, List<Bytes>)), Result<List<Adt19>, Bytes>>(Ok::<List<Adt19>, Bytes>(v6))
                                                    } else {
                                                        Ok::<(u64, (List<Adt19>, List<Bytes>)), Result<List<Adt19>, Bytes>>((v4, (v6, v10)))
                                                    }
                                                } else {
                                                    Err::<(u64, (List<Adt19>, List<Bytes>)), Result<List<Adt19>, Bytes>>(Err::<List<Adt19>, Bytes>(f7(List::cons(f8(Str::lit("the closing bracket ")), List::cons(v7, List::cons(f8(Str::lit(" does not match the opening bracket, which needs ")), List::cons(v9, List::cons(f8(Str::lit(" (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))))
                                                }
                                            }
                                        }
                                    } else {
                                        if if v3 {
                                            if f35(v7, f8(Str::lit(";"))) {
                                                let a1728 = {
                                                    let a1727 = v2.clone();
                                                    length_list(a1727)
                                                };
                                                let a1729 = 0u64;
                                                nat_eq(a1728, a1729)
                                            } else {
                                                false
                                            }
                                        } else {
                                            false
                                        } {
                                            Err::<(u64, (List<Adt19>, List<Bytes>)), Result<List<Adt19>, Bytes>>(Ok::<List<Adt19>, Bytes>(v6))
                                        } else {
                                            Ok::<(u64, (List<Adt19>, List<Bytes>)), Result<List<Adt19>, Bytes>>((v4, (v6, v2)))
                                        }
                                    }
                                }
                            }
                        }
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

pub fn f256(v0: List<Adt19>) -> List<Adt19> {
    let m1734 = v0;
    match m1734.uncons() {
        None => {
            List::<Adt19>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f257(v0: u64, v1: u64, v2: List<Adt19>, v3: bool) -> R<Result<(Adt12, List<Adt19>), Bytes>> {
    let m1735 = f260(v2);
    match m1735 {
        Ok(v5) => {
            let v6: bool = if v3 {
                f254(f262(v5.clone()), Str::lit(":"))
            } else {
                false
            };
            let m1736 = if v6 {
                f260(f256(f262(v5.clone())))
            } else {
                Ok::<Adt23, Bytes>(Adt23::C0(Bytes::lit(&[]), f262(v5.clone())))
            };
            match m1736 {
                Ok(v8) => {
                    let m1737 = f261(f262(v8.clone()), Str::lit("{"));
                    match m1737 {
                        Ok(v10) => {
                            let m1742 = {
                                let a1738 = v1;
                                let a1739 = 2u64;
                                if if nat_eq(a1738, a1739) {
                                    true
                                } else {
                                    let a1740 = v1;
                                    let a1741 = 7u64;
                                    nat_eq(a1740, a1741)
                                } {
                                    f348(v0, v10, List::<Adt13>::nil())?
                                } else {
                                    f349(v0, v10, List::<Adt13>::nil())?
                                }
                            };
                            match m1742 {
                                Ok(v12) => {
                                    Ok(Ok::<(Adt12, List<Adt19>), Bytes>((Adt12::C0(v1, f264(v5), f264(v8), {
                                        let (h1743, _) = v12;
                                        h1743
                                    }, List::<Bytes>::nil(), f265(Bytes::lit(&[]))), {
                                        let (_, h1744) = v12;
                                        h1744
                                    })))
                                }
                                Err(v11) => {
                                    Ok(Err::<(Adt12, List<Adt19>), Bytes>(v11))
                                }
                            }
                        }
                        Err(v9) => {
                            Ok(Err::<(Adt12, List<Adt19>), Bytes>(v9))
                        }
                    }
                }
                Err(v7) => {
                    Ok(Err::<(Adt12, List<Adt19>), Bytes>(v7))
                }
            }
        }
        Err(v4) => {
            Ok(Err::<(Adt12, List<Adt19>), Bytes>(v4))
        }
    }
}

pub fn f258(v0: List<Adt19>, v1: Bytes) -> R<Result<(Adt12, List<Adt19>), Bytes>> {
    let m1745 = f260(v0);
    match m1745 {
        Ok(v3) => {
            let m1746 = f350(f262(v3.clone()))?;
            match m1746 {
                Ok(v5) => {
                    Ok(Ok::<(Adt12, List<Adt19>), Bytes>((Adt12::C0(9u64, f264(v3), Bytes::lit(&[]), List::<Adt13>::nil(), List::cons(v1, List::<Bytes>::nil()), f265(Bytes::lit(&[]))), v5)))
                }
                Err(v4) => {
                    Ok(Err::<(Adt12, List<Adt19>), Bytes>(v4))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<(Adt12, List<Adt19>), Bytes>(v2))
        }
    }
}

pub fn f259(v0: Str, v1: List<Adt19>) -> Bytes {
    f7(List::cons(f8(Str::lit("expected ")), List::cons({
        let a1747 = v0;
        utf8_encode(a1747)
    }, List::cons(f8(Str::lit(" but found ")), List::cons(f351(v1), List::cons(f8(Str::lit(" (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
}

pub fn f260(v0: List<Adt19>) -> Result<Adt23, Bytes> {
    let m1748 = v0.clone();
    match m1748.uncons() {
        None => {
            Err::<Adt23, Bytes>(f259(Str::lit("an identifier"), v0))
        }
        Some((v1, v2)) => {
            let a1749 = f344(v1.clone());
            let a1750 = 0u64;
            if nat_eq(a1749, a1750) {
                Ok::<Adt23, Bytes>(Adt23::C0(f352(f345(v1)), v2))
            } else {
                Err::<Adt23, Bytes>(f259(Str::lit("an identifier"), v0))
            }
        }
    }
}

pub fn f261(v0: List<Adt19>, v1: Str) -> Result<List<Adt19>, Bytes> {
    if f254(v0.clone(), v1.clone()) {
        Ok::<List<Adt19>, Bytes>(f256(v0))
    } else {
        Err::<List<Adt19>, Bytes>(f259(v1, v0))
    }
}

pub fn f262(v0: Adt23) -> List<Adt19> {
    let m1751 = v0;
    match m1751 {
        Adt23::C0(_, v2) => {
            v2
        }
    }
}

pub fn f263(p1759: u64, p1760: List<Adt19>, p1761: List<Bytes>) -> Result<(List<Bytes>, List<Adt19>), Bytes> {
    let m1758 = (p1759, (p1760, p1761));
    {
        let mut m1758 = m1758;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1758;
                let m1752 = v0;
                if m1752 == 0 {
                    Err::<(u64, (List<Adt19>, List<Bytes>)), Result<(List<Bytes>, List<Adt19>), Bytes>>(Err::<(List<Bytes>, List<Adt19>), Bytes>(f8(Str::lit("an enumeration has more values than the input has tokens"))))
                } else {
                    let v3 = m1752 - 1;
                    let m1753 = v1.clone();
                    match m1753.uncons() {
                        None => {
                            Err::<(u64, (List<Adt19>, List<Bytes>)), Result<(List<Bytes>, List<Adt19>), Bytes>>(Err::<(List<Bytes>, List<Adt19>), Bytes>(f259(Str::lit("a string"), v1)))
                        }
                        Some((v4, v5)) => {
                            let a1754 = f344(v4.clone());
                            let a1755 = 3u64;
                            if !nat_eq(a1754, a1755) {
                                Err::<(u64, (List<Adt19>, List<Bytes>)), Result<(List<Bytes>, List<Adt19>), Bytes>>(Err::<(List<Bytes>, List<Adt19>), Bytes>(f259(Str::lit("a string"), v1)))
                            } else {
                                if f254(v5.clone(), Str::lit(",")) {
                                    if f254(f256(v5.clone()), Str::lit("}")) {
                                        let m1756 = f261(f256(f256(v5)), Str::lit(";"));
                                        match m1756 {
                                            Ok(v7) => {
                                                Err::<(u64, (List<Adt19>, List<Bytes>)), Result<(List<Bytes>, List<Adt19>), Bytes>>(Ok::<(List<Bytes>, List<Adt19>), Bytes>((f44(List::cons(f345(v4), v2)), v7)))
                                            }
                                            Err(v6) => {
                                                Err::<(u64, (List<Adt19>, List<Bytes>)), Result<(List<Bytes>, List<Adt19>), Bytes>>(Err::<(List<Bytes>, List<Adt19>), Bytes>(v6))
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (List<Adt19>, List<Bytes>)), Result<(List<Bytes>, List<Adt19>), Bytes>>((v3, (f256(v5), List::cons(f345(v4), v2))))
                                    }
                                } else {
                                    if f254(v5.clone(), Str::lit("}")) {
                                        let m1757 = f261(f256(v5), Str::lit(";"));
                                        match m1757 {
                                            Ok(v9) => {
                                                Err::<(u64, (List<Adt19>, List<Bytes>)), Result<(List<Bytes>, List<Adt19>), Bytes>>(Ok::<(List<Bytes>, List<Adt19>), Bytes>((f44(List::cons(f345(v4), v2)), v9)))
                                            }
                                            Err(v8) => {
                                                Err::<(u64, (List<Adt19>, List<Bytes>)), Result<(List<Bytes>, List<Adt19>), Bytes>>(Err::<(List<Bytes>, List<Adt19>), Bytes>(v8))
                                            }
                                        }
                                    } else {
                                        Err::<(u64, (List<Adt19>, List<Bytes>)), Result<(List<Bytes>, List<Adt19>), Bytes>>(Err::<(List<Bytes>, List<Adt19>), Bytes>(f259(Str::lit(", or }"), v5)))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1758 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f264(v0: Adt23) -> Bytes {
    let m1762 = v0;
    match m1762 {
        Adt23::C0(v1, _) => {
            v1
        }
    }
}

pub fn f265(v0: Bytes) -> Adt14 {
    Adt14::C0(0u64, v0, List::<Bytes>::nil(), Bytes::lit(&[]), Bytes::lit(&[]))
}

pub fn f266(v0: u64, v1: List<Adt19>, v2: u64) -> R<Result<Adt24, Bytes>> {
    let m1763 = v0;
    if m1763 == 0 {
        Ok(Err::<Adt24, Bytes>(f8(Str::lit("a type nests deeper than the input is long"))))
    } else {
        let v3 = m1763 - 1;
        let a1764 = v2;
        let a1765 = 0u64;
        if nat_eq(a1764, a1765) {
            let m1766 = f183(v1)?;
            match m1766 {
                Ok(v5) => {
                    if f254(v5.clone(), Str::lit("(")) {
                        let m1767 = f266(v3, f256(v5), 1u64)?;
                        match m1767 {
                            Ok(v7) => {
                                if f254(f267(v7.clone()), Str::lit("?")) {
                                    Ok(Ok::<Adt24, Bytes>(Adt24::C0(Adt14::C0(0u64, Bytes::lit(&[]), f203(f268(v7.clone())), f8(Str::lit("a nullable union type is not translated yet, because the typedef identifier names only the union (Web IDL 2.13.32)")), Bytes::lit(&[])), f256(f267(v7)))))
                                } else {
                                    Ok(Ok::<Adt24, Bytes>(Adt24::C0(f268(v7.clone()), f267(v7))))
                                }
                            }
                            Err(v6) => {
                                Ok(Err::<Adt24, Bytes>(v6))
                            }
                        }
                    } else {
                        let m1768 = v5.clone();
                        match m1768.uncons() {
                            None => {
                                Ok(Err::<Adt24, Bytes>(f259(Str::lit("a type"), v5)))
                            }
                            Some((v8, v9)) => {
                                let v10: Bytes = f345(v8.clone());
                                let a1769 = f344(v8);
                                let a1770 = 0u64;
                                if !nat_eq(a1769, a1770) {
                                    Ok(Err::<Adt24, Bytes>(f259(Str::lit("a type"), v5)))
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
                                        let m1771 = f261(v9, Str::lit("<"));
                                        match m1771 {
                                            Ok(v12) => {
                                                let m1772 = f266(v3, v12, 0u64)?;
                                                match m1772 {
                                                    Ok(v14) => {
                                                        let m1773 = f261(f267(v14.clone()), Str::lit(">"));
                                                        match m1773 {
                                                            Ok(v16) => {
                                                                Ok(f353(f354(Str::lit("List"), f268(v14)), v16))
                                                            }
                                                            Err(v15) => {
                                                                Ok(Err::<Adt24, Bytes>(v15))
                                                            }
                                                        }
                                                    }
                                                    Err(v13) => {
                                                        Ok(Err::<Adt24, Bytes>(v13))
                                                    }
                                                }
                                            }
                                            Err(v11) => {
                                                Ok(Err::<Adt24, Bytes>(v11))
                                            }
                                        }
                                    } else {
                                        if f35(v10.clone(), f8(Str::lit("async_sequence"))) {
                                            let m1774 = f261(v9, Str::lit("<"));
                                            match m1774 {
                                                Ok(v18) => {
                                                    let m1775 = f266(v3, v18, 0u64)?;
                                                    match m1775 {
                                                        Ok(v20) => {
                                                            let m1776 = f261(f267(v20), Str::lit(">"));
                                                            match m1776 {
                                                                Ok(v22) => {
                                                                    Ok(f353(f355(f8(Str::lit("async_sequence types are not translated, because an async sequence is produced over time and is not a value (Web IDL 2.13.29)"))), v22))
                                                                }
                                                                Err(v21) => {
                                                                    Ok(Err::<Adt24, Bytes>(v21))
                                                                }
                                                            }
                                                        }
                                                        Err(v19) => {
                                                            Ok(Err::<Adt24, Bytes>(v19))
                                                        }
                                                    }
                                                }
                                                Err(v17) => {
                                                    Ok(Err::<Adt24, Bytes>(v17))
                                                }
                                            }
                                        } else {
                                            if f35(v10.clone(), f8(Str::lit("Promise"))) {
                                                let m1777 = f261(v9, Str::lit("<"));
                                                match m1777 {
                                                    Ok(v24) => {
                                                        let m1778 = f266(v3, v24, 0u64)?;
                                                        match m1778 {
                                                            Ok(v26) => {
                                                                let m1779 = f261(f267(v26), Str::lit(">"));
                                                                match m1779 {
                                                                    Ok(v28) => {
                                                                        Ok(f353(f355(f8(Str::lit("Promise types are not translated, because a promise is a deferred result, not a value (Web IDL 2.13.31)"))), v28))
                                                                    }
                                                                    Err(v27) => {
                                                                        Ok(Err::<Adt24, Bytes>(v27))
                                                                    }
                                                                }
                                                            }
                                                            Err(v25) => {
                                                                Ok(Err::<Adt24, Bytes>(v25))
                                                            }
                                                        }
                                                    }
                                                    Err(v23) => {
                                                        Ok(Err::<Adt24, Bytes>(v23))
                                                    }
                                                }
                                            } else {
                                                if f35(v10.clone(), f8(Str::lit("record"))) {
                                                    let m1780 = f261(v9, Str::lit("<"));
                                                    match m1780 {
                                                        Ok(v30) => {
                                                            let m1781 = v30.clone();
                                                            match m1781.uncons() {
                                                                None => {
                                                                    Ok(Err::<Adt24, Bytes>(f259(Str::lit("a string type"), v30)))
                                                                }
                                                                Some((v31, v32)) => {
                                                                    if if f35(f345(v31.clone()), f8(Str::lit("DOMString"))) {
                                                                        true
                                                                    } else {
                                                                        if f35(f345(v31.clone()), f8(Str::lit("USVString"))) {
                                                                            true
                                                                        } else {
                                                                            f35(f345(v31.clone()), f8(Str::lit("ByteString")))
                                                                        }
                                                                    } {
                                                                        let m1782 = f261(v32, Str::lit(","));
                                                                        match m1782 {
                                                                            Ok(v34) => {
                                                                                let m1783 = f266(v3, v34, 0u64)?;
                                                                                match m1783 {
                                                                                    Ok(v36) => {
                                                                                        let m1784 = f261(f267(v36.clone()), Str::lit(">"));
                                                                                        match m1784 {
                                                                                            Ok(v38) => {
                                                                                                let m1785 = f356(f345(v31));
                                                                                                match m1785 {
                                                                                                    None => {
                                                                                                        Ok(Err::<Adt24, Bytes>(f259(Str::lit("a string type"), v30)))
                                                                                                    }
                                                                                                    Some(v39) => {
                                                                                                        let v40: Adt14 = f357(f268(v36));
                                                                                                        Ok(f353(Adt14::C0(0u64, f7(List::cons(f8(Str::lit("(List (")), List::cons(f202(v39), List::cons(f8(Str::lit(" * ")), List::cons(f202(v40.clone()), List::cons(f8(Str::lit("))")), List::<Bytes>::nil())))))), f203(v40.clone()), f200(v40), Bytes::lit(&[])), v38))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            Err(v37) => {
                                                                                                Ok(Err::<Adt24, Bytes>(v37))
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    Err(v35) => {
                                                                                        Ok(Err::<Adt24, Bytes>(v35))
                                                                                    }
                                                                                }
                                                                            }
                                                                            Err(v33) => {
                                                                                Ok(Err::<Adt24, Bytes>(v33))
                                                                            }
                                                                        }
                                                                    } else {
                                                                        Ok(Err::<Adt24, Bytes>(f259(Str::lit("DOMString, USVString or ByteString"), v30)))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Err(v29) => {
                                                            Ok(Err::<Adt24, Bytes>(v29))
                                                        }
                                                    }
                                                } else {
                                                    if f35(v10.clone(), f8(Str::lit("unsigned"))) {
                                                        if f254(v9.clone(), Str::lit("short")) {
                                                            Ok(f353(f265(f8(Str::lit("Nat"))), f256(v9)))
                                                        } else {
                                                            if f254(v9.clone(), Str::lit("long")) {
                                                                let v41: List<Adt19> = f256(v9);
                                                                Ok(f353(f265(f8(Str::lit("Nat"))), if f254(v41.clone(), Str::lit("long")) {
                                                                    f256(v41)
                                                                } else {
                                                                    v41
                                                                }))
                                                            } else {
                                                                Ok(Err::<Adt24, Bytes>(f259(Str::lit("short or long"), v9)))
                                                            }
                                                        }
                                                    } else {
                                                        if f35(v10.clone(), f8(Str::lit("short"))) {
                                                            Ok(f353(f265(f8(Str::lit("Int"))), v9))
                                                        } else {
                                                            if f35(v10.clone(), f8(Str::lit("long"))) {
                                                                Ok(f353(f265(f8(Str::lit("Int"))), if f254(v9.clone(), Str::lit("long")) {
                                                                    f256(v9)
                                                                } else {
                                                                    v9
                                                                }))
                                                            } else {
                                                                if f35(v10.clone(), f8(Str::lit("unrestricted"))) {
                                                                    if if f254(v9.clone(), Str::lit("float")) {
                                                                        true
                                                                    } else {
                                                                        f254(v9.clone(), Str::lit("double"))
                                                                    } {
                                                                        Ok(f353(f355(f8(Str::lit("unrestricted float and unrestricted double are not translated, because its value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.13 and 2.13.15)"))), f256(v9)))
                                                                    } else {
                                                                        Ok(Err::<Adt24, Bytes>(f259(Str::lit("float or double"), v9)))
                                                                    }
                                                                } else {
                                                                    let m1786 = f356(v10.clone());
                                                                    match m1786 {
                                                                        None => {
                                                                            let v42: Bytes = f352(v10);
                                                                            Ok(f353(Adt14::C0(0u64, f152(v42.clone())?, List::cons(v42.clone(), List::<Bytes>::nil()), Bytes::lit(&[]), v42), v9))
                                                                        }
                                                                        Some(v43) => {
                                                                            Ok(f353(v43, v9))
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
                    Ok(Err::<Adt24, Bytes>(v4))
                }
            }
        } else {
            let m1787 = f266(v3, v1, 0u64)?;
            match m1787 {
                Ok(v45) => {
                    let v46: Adt14 = f268(v45.clone());
                    let v48: Adt14 = {
                        let a1788 = f201(v46.clone());
                        let a1789 = 0u64;
                        if nat_eq(a1788, a1789) {
                            if f35(f200(v46.clone()), Bytes::lit(&[])) {
                                if f35(f358(v46.clone()), Bytes::lit(&[])) {
                                    Adt14::C0(1u64, Bytes::lit(&[]), f203(v46), f8(Str::lit("a union member type that is not a named definition gives its constructor no name, so the union is not translated yet (Web IDL 2.13.32)")), Bytes::lit(&[]))
                                } else {
                                    let m1790 = f283(false, f358(v46.clone()))?;
                                    match m1790 {
                                        None => {
                                            Adt14::C0(1u64, Bytes::lit(&[]), f203(v46.clone()), f7(List::cons(f8(Str::lit("the union member ")), List::cons(f358(v46), List::cons(f8(Str::lit(" has no constructor name")), List::<Bytes>::nil())))), Bytes::lit(&[]))
                                        }
                                        Some(v47) => {
                                            if f186(v47.clone()) {
                                                Adt14::C0(1u64, Bytes::lit(&[]), f203(v46), f187(v47), Bytes::lit(&[]))
                                            } else {
                                                Adt14::C0(1u64, f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v47, List::cons(f8(Str::lit(", fields: [")), List::cons(f202(v46.clone()), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f203(v46), Bytes::lit(&[]), Bytes::lit(&[]))
                                            }
                                        }
                                    }
                                }
                            } else {
                                Adt14::C0(1u64, Bytes::lit(&[]), f203(v46.clone()), f200(v46), Bytes::lit(&[]))
                            }
                        } else {
                            v46
                        }
                    };
                    if f254(f267(v45.clone()), Str::lit("or")) {
                        let m1791 = f266(v3, f256(f267(v45)), 2u64)?;
                        match m1791 {
                            Ok(v50) => {
                                Ok(Ok::<Adt24, Bytes>(Adt24::C0(Adt14::C0(1u64, if f35(f202(v48.clone()), Bytes::lit(&[])) {
                                    f202(f268(v50.clone()))
                                } else {
                                    if f35(f202(f268(v50.clone())), Bytes::lit(&[])) {
                                        f202(v48.clone())
                                    } else {
                                        f7(List::cons(f202(v48.clone()), List::cons(f8(Str::lit(",\u{a}")), List::cons(f202(f268(v50.clone())), List::<Bytes>::nil()))))
                                    }
                                }, f116(f203(v48.clone()), f203(f268(v50.clone()))), if f35(f200(v48.clone()), Bytes::lit(&[])) {
                                    f200(f268(v50.clone()))
                                } else {
                                    f200(v48)
                                }, Bytes::lit(&[])), f267(v50))))
                            }
                            Err(v49) => {
                                Ok(Err::<Adt24, Bytes>(v49))
                            }
                        }
                    } else {
                        let a1792 = v2;
                        let a1793 = 2u64;
                        if if nat_eq(a1792, a1793) {
                            f254(f267(v45.clone()), Str::lit(")"))
                        } else {
                            false
                        } {
                            Ok(Ok::<Adt24, Bytes>(Adt24::C0(v48, f256(f267(v45)))))
                        } else {
                            Ok(Err::<Adt24, Bytes>({
                                let a1794 = v2;
                                let a1795 = 1u64;
                                if nat_eq(a1794, a1795) {
                                    f259(Str::lit("or"), f267(v45))
                                } else {
                                    f259(Str::lit("or or )"), f267(v45))
                                }
                            }))
                        }
                    }
                }
                Err(v44) => {
                    Ok(Err::<Adt24, Bytes>(v44))
                }
            }
        }
    }
}

pub fn f267(v0: Adt24) -> List<Adt19> {
    let m1796 = v0;
    match m1796 {
        Adt24::C0(_, v2) => {
            v2
        }
    }
}

pub fn f268(v0: Adt24) -> Adt14 {
    let m1797 = v0;
    match m1797 {
        Adt24::C0(v1, _) => {
            v1
        }
    }
}

pub fn f269() -> List<Bytes> {
    List::cons(f8(Str::lit("Prop")), List::cons(f8(Str::lit("Sort")), List::cons(f8(Str::lit("StateRefT")), List::cons(f8(Str::lit("Type")), List::cons(f8(Str::lit("abbrev")), List::cons(f8(Str::lit("at")), List::cons(f8(Str::lit("attribute")), List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("bif")), List::cons(f8(Str::lit("break")), List::cons(f8(Str::lit("by")), List::cons(f8(Str::lit("calc")), List::cons(f8(Str::lit("catch")), List::cons(f8(Str::lit("class")), List::cons(f8(Str::lit("coinductive")), List::cons(f8(Str::lit("continue")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f270() -> List<Bytes> {
    List::cons(f8(Str::lit("def")), List::cons(f8(Str::lit("deriving")), List::cons(f8(Str::lit("do")), List::cons(f8(Str::lit("dsimproc")), List::cons(f8(Str::lit("elab")), List::cons(f8(Str::lit("else")), List::cons(f8(Str::lit("end")), List::cons(f8(Str::lit("example")), List::cons(f8(Str::lit("exists")), List::cons(f8(Str::lit("export")), List::cons(f8(Str::lit("extends")), List::cons(f8(Str::lit("finally")), List::cons(f8(Str::lit("for")), List::cons(f8(Str::lit("forall")), List::cons(f8(Str::lit("from")), List::cons(f8(Str::lit("fun")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f271() -> List<Bytes> {
    List::cons(f8(Str::lit("generalizing")), List::cons(f8(Str::lit("have")), List::cons(f8(Str::lit("haveI")), List::cons(f8(Str::lit("hiding")), List::cons(f8(Str::lit("idbg")), List::cons(f8(Str::lit("if")), List::cons(f8(Str::lit("import")), List::cons(f8(Str::lit("in")), List::cons(f8(Str::lit("include")), List::cons(f8(Str::lit("inductive")), List::cons(f8(Str::lit("inferInstanceAs")), List::cons(f8(Str::lit("infix")), List::cons(f8(Str::lit("infixl")), List::cons(f8(Str::lit("infixr")), List::cons(f8(Str::lit("initialize")), List::cons(f8(Str::lit("instance")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f272() -> List<Bytes> {
    List::cons(f8(Str::lit("let")), List::cons(f8(Str::lit("letI")), List::cons(f8(Str::lit("local")), List::cons(f8(Str::lit("logNamedError")), List::cons(f8(Str::lit("logNamedErrorAt")), List::cons(f8(Str::lit("logNamedWarning")), List::cons(f8(Str::lit("logNamedWarningAt")), List::cons(f8(Str::lit("macro")), List::cons(f8(Str::lit("match")), List::cons(f8(Str::lit("matches")), List::cons(f8(Str::lit("meta")), List::cons(f8(Str::lit("mut")), List::cons(f8(Str::lit("mutual")), List::cons(f8(Str::lit("namespace")), List::cons(f8(Str::lit("nofun")), List::cons(f8(Str::lit("nomatch")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f273() -> List<Bytes> {
    List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("nonrec")), List::cons(f8(Str::lit("notation")), List::cons(f8(Str::lit("omit")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("open")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("postfix")), List::cons(f8(Str::lit("prefix")), List::cons(f8(Str::lit("private")), List::cons(f8(Str::lit("protected")), List::cons(f8(Str::lit("public")), List::cons(f8(Str::lit("renaming")), List::cons(f8(Str::lit("repeat")), List::cons(f8(Str::lit("reprove")), List::cons(f8(Str::lit("return")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f274() -> List<Bytes> {
    List::cons(f8(Str::lit("scoped")), List::cons(f8(Str::lit("seal")), List::cons(f8(Str::lit("section")), List::cons(f8(Str::lit("show")), List::cons(f8(Str::lit("simproc")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("structure")), List::cons(f8(Str::lit("suffices")), List::cons(f8(Str::lit("syntax")), List::cons(f8(Str::lit("then")), List::cons(f8(Str::lit("theorem")), List::cons(f8(Str::lit("throwError")), List::cons(f8(Str::lit("throwErrorAt")), List::cons(f8(Str::lit("throwNamedError")), List::cons(f8(Str::lit("throwNamedErrorAt")), List::cons(f8(Str::lit("try")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f275() -> List<Bytes> {
    List::cons(f8(Str::lit("universe")), List::cons(f8(Str::lit("unless")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("unseal")), List::cons(f8(Str::lit("until")), List::cons(f8(Str::lit("using")), List::cons(f8(Str::lit("variable")), List::cons(f8(Str::lit("where")), List::cons(f8(Str::lit("while")), List::cons(f8(Str::lit("with")), List::cons(f8(Str::lit("reprove")), List::cons(f8(Str::lit("throwError")), List::cons(f8(Str::lit("throwErrorAt")), List::<Bytes>::nil())))))))))))))
}

pub fn f276(v0: List<Adt13>) -> R<List<Adt13>> {
    let m1798 = v0;
    match m1798.uncons() {
        None => {
            Ok(List::<Adt13>::nil())
        }
        Some((v1, v2)) => {
            f359(v1, f276(v2)?)
        }
    }
}

pub fn f277(v0: List<Adt13>, v1: List<Adt13>) -> List<Adt13> {
    let m1799 = v0;
    match m1799.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f277(v3, v1))
        }
    }
}

pub fn f278(v0: Adt12) -> List<Adt13> {
    let m1800 = v0;
    match m1800 {
        Adt12::C0(_, _, _, v4, _, _) => {
            v4
        }
    }
}

pub fn f279(v0: List<Adt12>, v1: u64, v2: Bytes) -> List<Adt13> {
    let m1801 = v0;
    match m1801.uncons() {
        None => {
            List::<Adt13>::nil()
        }
        Some((v3, v4)) => {
            let a1802 = f110(v3.clone());
            let a1803 = v1;
            if if nat_eq(a1802, a1803) {
                f35(f112(v3.clone()), v2.clone())
            } else {
                false
            } {
                f277(f278(v3), f279(v4, v1, v2))
            } else {
                f279(v4, v1, v2)
            }
        }
    }
}

pub fn f280(v0: List<Adt12>, v1: List<Adt12>, v2: Bytes) -> List<Adt13> {
    let m1804 = v0;
    match m1804.uncons() {
        None => {
            List::<Adt13>::nil()
        }
        Some((v3, v4)) => {
            let a1805 = f110(v3.clone());
            let a1806 = 8u64;
            if if nat_eq(a1805, a1806) {
                f35(f112(v3.clone()), v2.clone())
            } else {
                false
            } {
                f277(f360(v1.clone(), f123(v3)), f280(v4, v1, v2))
            } else {
                f280(v4, v1, v2)
            }
        }
    }
}

pub fn f281(v0: Adt13) -> Adt14 {
    let m1807 = v0;
    match m1807 {
        Adt13::C0(_, v2, _) => {
            v2
        }
    }
}

pub fn f282(v0: Adt13) -> Bytes {
    let m1808 = v0;
    match m1808 {
        Adt13::C0(v1, _, _) => {
            v1
        }
    }
}

pub fn f283(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f361(v1)
    } else {
        f286(v1)
    }
}

pub fn f284(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1809 = v1;
    match m1809.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m1810 = f283(v0, v2)?;
            match m1810 {
                None => {
                    f284(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f284(v0, v3)?))
                }
            }
        }
    }
}

pub fn f285(v0: Adt13) -> u64 {
    let m1811 = v0;
    match m1811 {
        Adt13::C0(_, _, v3) => {
            v3
        }
    }
}

pub fn f286(v0: Bytes) -> R<Option<Bytes>> {
    let m1812 = f362(v0)?;
    match m1812 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            Ok(Some(f363(v1)?))
        }
    }
}

pub fn f287(v0: bool, v1: Adt17) -> R<Option<Bytes>> {
    let m1813 = v1;
    match m1813 {
        Adt17::C0(_) => {
            Ok(None::<Bytes>)
        }
        Adt17::C1(v7, v8) => {
            Ok(Some({
                let a1824 = if f302(v7, {
                    let a1822 = 0u64;
                    let a1823 = v8;
                    nat_lt(a1822, a1823)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a1825 = f174(false, v8)?;
                append_bytes(a1824, a1825)
            }))
        }
        Adt17::C2(v3, v4, v5) => {
            let m1816 = f361({
                let a1814 = f8(Str::lit("n"));
                let a1815 = f175(false, v4, v5)?;
                append_bytes(a1814, a1815)
            })?;
            match m1816 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a1820 = f8(Str::lit("nMinus"));
                        let a1821 = f32(v6.clone(), 1u64, {
                            let a1818 = {
                                let a1817 = v6;
                                length_bytes(a1817)
                            };
                            let a1819 = 1u64;
                            nat_sub(a1818, a1819)
                        });
                        append_bytes(a1820, a1821)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt17::C3(v2) => {
            f283(v0, v2)
        }
        Adt17::C4(_) => {
            Ok(None::<Bytes>)
        }
        Adt17::C5(_) => {
            Ok(None::<Bytes>)
        }
        Adt17::C6 => {
            Ok(None::<Bytes>)
        }
        Adt17::C7(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt17::C8(_, _, _) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f288(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m1828 = {
        let a1826 = v0.clone();
        let a1827 = v1;
        index_bytes(a1826, a1827)
    };
    match m1828 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f41(v2)?;
            let a1829 = v3;
            let a1830 = 128u64;
            if nat_lt(a1829, a1830) {
                Ok(Some((v3, 1u64)))
            } else {
                if f302({
                    let a1831 = 194u64;
                    let a1832 = v3;
                    nat_le(a1831, a1832)
                }, {
                    let a1833 = v3;
                    let a1834 = 223u64;
                    nat_le(a1833, a1834)
                }) {
                    let m1837 = f364(v0, {
                        let a1835 = v1;
                        let a1836 = 1u64;
                        nat_add(a1835, a1836)?
                    })?;
                    match m1837 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f365({
                                let a1842 = {
                                    let a1840 = {
                                        let a1838 = v3;
                                        let a1839 = 192u64;
                                        nat_sub(a1838, a1839)
                                    };
                                    let a1841 = 64u64;
                                    nat_mul(a1840, a1841)?
                                };
                                let a1843 = v4;
                                nat_add(a1842, a1843)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f302({
                        let a1844 = 224u64;
                        let a1845 = v3;
                        nat_le(a1844, a1845)
                    }, {
                        let a1846 = v3;
                        let a1847 = 239u64;
                        nat_le(a1846, a1847)
                    }) {
                        let m1850 = f364(v0.clone(), {
                            let a1848 = v1;
                            let a1849 = 1u64;
                            nat_add(a1848, a1849)?
                        })?;
                        match m1850 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m1853 = f364(v0, {
                                    let a1851 = v1;
                                    let a1852 = 2u64;
                                    nat_add(a1851, a1852)?
                                })?;
                                match m1853 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f365({
                                            let a1862 = {
                                                let a1856 = {
                                                    let a1854 = v3;
                                                    let a1855 = 224u64;
                                                    nat_sub(a1854, a1855)
                                                };
                                                let a1857 = 4096u64;
                                                nat_mul(a1856, a1857)?
                                            };
                                            let a1863 = {
                                                let a1860 = {
                                                    let a1858 = v5;
                                                    let a1859 = 64u64;
                                                    nat_mul(a1858, a1859)?
                                                };
                                                let a1861 = v6;
                                                nat_add(a1860, a1861)?
                                            };
                                            nat_add(a1862, a1863)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f302({
                            let a1864 = 240u64;
                            let a1865 = v3;
                            nat_le(a1864, a1865)
                        }, {
                            let a1866 = v3;
                            let a1867 = 244u64;
                            nat_le(a1866, a1867)
                        }) {
                            let m1870 = f364(v0.clone(), {
                                let a1868 = v1;
                                let a1869 = 1u64;
                                nat_add(a1868, a1869)?
                            })?;
                            match m1870 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m1873 = f364(v0.clone(), {
                                        let a1871 = v1;
                                        let a1872 = 2u64;
                                        nat_add(a1871, a1872)?
                                    })?;
                                    match m1873 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m1876 = f364(v0, {
                                                let a1874 = v1;
                                                let a1875 = 3u64;
                                                nat_add(a1874, a1875)?
                                            })?;
                                            match m1876 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f365({
                                                        let a1889 = {
                                                            let a1879 = {
                                                                let a1877 = v3;
                                                                let a1878 = 240u64;
                                                                nat_sub(a1877, a1878)
                                                            };
                                                            let a1880 = 262144u64;
                                                            nat_mul(a1879, a1880)?
                                                        };
                                                        let a1890 = {
                                                            let a1887 = {
                                                                let a1881 = v7;
                                                                let a1882 = 4096u64;
                                                                nat_mul(a1881, a1882)?
                                                            };
                                                            let a1888 = {
                                                                let a1885 = {
                                                                    let a1883 = v8;
                                                                    let a1884 = 64u64;
                                                                    nat_mul(a1883, a1884)?
                                                                };
                                                                let a1886 = v9;
                                                                nat_add(a1885, a1886)?
                                                            };
                                                            nat_add(a1887, a1888)?
                                                        };
                                                        nat_add(a1889, a1890)?
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

pub fn f289(v0: u64) -> R<bool> {
    f208(v0, f366())
}

pub fn f290(v0: u64, v1: Bytes, p1901: u64, p1902: u64, p1903: bool) -> R<bool> {
    let m1900 = (p1901, (p1902, p1903));
    {
        let mut m1900 = m1900;
        loop {
            let step = {
                let (v2, (v3, v4)) = m1900;
                let m1891 = v3;
                if m1891 == 0 {
                    Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                } else {
                    let v5 = m1891 - 1;
                    let v6: u64 = f367(v1.clone(), v2)?;
                    if v4 {
                        let a1892 = v0;
                        let a1893 = v6;
                        if nat_lt(a1892, a1893) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a1894 = v2;
                                let a1895 = 1u64;
                                nat_add(a1894, a1895)?
                            }, (v5, false)))
                        }
                    } else {
                        let a1896 = v0;
                        let a1897 = v6;
                        if nat_le(a1896, a1897) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(true))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a1898 = v2;
                                let a1899 = 1u64;
                                nat_add(a1898, a1899)?
                            }, (v5, true)))
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

pub fn f291(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    if f77(v0.clone(), v1, v2) {
        Ok(None::<u64>)
    } else {
        let a1905 = v2;
        let a1906 = {
            let a1904 = v0;
            length_bytes(a1904)
        };
        if nat_lt(a1905, a1906) {
            Ok(Some({
                let a1907 = v2;
                let a1908 = 1u64;
                nat_add(a1907, a1908)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f292(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f130(f85(v0, v1)?)? {
        Ok(Some({
            let a1909 = v1;
            let a1910 = 1u64;
            nat_add(a1909, a1910)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f293(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let m1911 = f288(v0, v2)?;
    match m1911 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            let a1912 = v2;
            let a1913 = v1;
            if if nat_eq(a1912, a1913) {
                f368({
                    let (h1914, _) = v3;
                    h1914
                })?
            } else {
                f369({
                    let (h1915, _) = v3;
                    h1915
                })?
            } {
                Ok(Some({
                    let a1917 = v2;
                    let a1918 = {
                        let (_, h1916) = v3;
                        h1916
                    };
                    nat_add(a1917, a1918)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f294(v0: Bytes, v1: bool, v2: u64) -> R<(u64, u64)> {
    let (h1919, _) = f316(Fn7::F370(v1, v0.clone()), {
        let a1921 = {
            let a1920 = v0;
            length_bytes(a1920)
        };
        let a1922 = 1u64;
        nat_add(a1921, a1922)?
    }, (v2, 0u64))?;
    Ok(h1919)
}

pub fn f295(v0: u64) -> R<Bytes> {
    let a1923 = v0;
    let a1924 = 128u64;
    if nat_lt(a1923, a1924) {
        Ok(f371(f372(), v0))
    } else {
        let a1925 = v0;
        let a1926 = 2048u64;
        if nat_lt(a1925, a1926) {
            let a1934 = f371(f373(), {
                let a1932 = {
                    let a1930 = {
                        let a1927 = v0;
                        let a1928 = 64u64;
                        let a1929 = 0u64;
                        nat_quot(a1927, a1928, a1929)
                    };
                    let a1931 = 2u64;
                    nat_sub(a1930, a1931)
                };
                let a1933 = 2u64;
                nat_mul(a1932, a1933)?
            });
            let a1935 = f374(v0)?;
            Ok(append_bytes(a1934, a1935))
        } else {
            let a1936 = v0;
            let a1937 = 65536u64;
            if nat_lt(a1936, a1937) {
                let a1948 = f371(f375(), {
                    let a1941 = {
                        let a1938 = v0;
                        let a1939 = 4096u64;
                        let a1940 = 0u64;
                        nat_quot(a1938, a1939, a1940)
                    };
                    let a1942 = 3u64;
                    nat_mul(a1941, a1942)?
                });
                let a1949 = {
                    let a1946 = f374({
                        let a1943 = v0;
                        let a1944 = 64u64;
                        let a1945 = 0u64;
                        nat_quot(a1943, a1944, a1945)
                    })?;
                    let a1947 = f374(v0)?;
                    append_bytes(a1946, a1947)
                };
                Ok(append_bytes(a1948, a1949))
            } else {
                let a1963 = f371(Bytes::lit(&[0xf0, 0xf1, 0xf2, 0xf3, 0xf4]), {
                    let a1950 = v0;
                    let a1951 = 262144u64;
                    let a1952 = 0u64;
                    nat_quot(a1950, a1951, a1952)
                });
                let a1964 = {
                    let a1961 = f374({
                        let a1953 = v0;
                        let a1954 = 4096u64;
                        let a1955 = 0u64;
                        nat_quot(a1953, a1954, a1955)
                    })?;
                    let a1962 = {
                        let a1959 = f374({
                            let a1956 = v0;
                            let a1957 = 64u64;
                            let a1958 = 0u64;
                            nat_quot(a1956, a1957, a1958)
                        })?;
                        let a1960 = f374(v0)?;
                        append_bytes(a1959, a1960)
                    };
                    append_bytes(a1961, a1962)
                };
                Ok(append_bytes(a1963, a1964))
            }
        }
    }
}

pub fn f296(v0: bool, v1: u64, v2: Bytes, v3: u64) -> R<Option<u64>> {
    f376(v2, v0, v1, v3)
}

pub fn f297(v0: Bytes) -> Result<Option<Adt21>, Bytes> {
    Err::<Option<Adt21>, Bytes>(v0)
}

pub fn f298(p1968: Bytes, p1969: u64) -> R<Result<Bytes, Bytes>> {
    let m1967 = (p1968, p1969);
    {
        let mut m1967 = m1967;
        loop {
            let step = {
                let (v0, v1) = m1967;
                let m1965 = v1;
                if m1965 == 0 {
                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f377(f8(Str::lit("the built-in datatype hierarchy is deeper than XSD 1.1 Part 2 section 3 defines")))))
                } else {
                    let v2 = m1965 - 1;
                    if f172(v0.clone(), List::cons(f8(Str::lit("NMTOKENS")), List::cons(f8(Str::lit("IDREFS")), List::cons(f8(Str::lit("ENTITIES")), List::<Bytes>::nil())))) {
                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("(List Bytes)")))))
                    } else {
                        if f35(v0.clone(), f8(Str::lit("nonNegativeInteger"))) {
                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Nat")))))
                        } else {
                            if f35(v0.clone(), f8(Str::lit("integer"))) {
                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Int")))))
                            } else {
                                if f35(v0.clone(), f8(Str::lit("decimal"))) {
                                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f378())))
                                } else {
                                    if f35(v0.clone(), f8(Str::lit("boolean"))) {
                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Bool")))))
                                    } else {
                                        if f172(v0.clone(), List::cons(f8(Str::lit("float")), List::cons(f8(Str::lit("double")), List::<Bytes>::nil()))) {
                                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f377(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because its value space holds positive and negative zero, the infinities and NaN, which no exact LexLean number type holds (XSD 1.1 Part 2, sections 3.3.4 and 3.3.5)")), List::<Bytes>::nil())))))))
                                        } else {
                                            if f172(v0.clone(), List::cons(f8(Str::lit("anySimpleType")), List::cons(f8(Str::lit("anyAtomicType")), List::<Bytes>::nil()))) {
                                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f377(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because it is the union of every simple value space and has no one LexLean type (XSD 1.1 Part 2, sections 3.2.1 and 3.2.2)")), List::<Bytes>::nil())))))))
                                            } else {
                                                let m1966 = f379(v0.clone())?;
                                                match m1966 {
                                                    None => {
                                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f377(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not a built-in datatype of XSD 1.1 Part 2")), List::<Bytes>::nil())))))))
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
                Ok(next) => { m1967 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f299(v0: Bytes) -> Result<List<Adt17>, Bytes> {
    Err::<List<Adt17>, Bytes>(v0)
}

pub fn f300(v0: Option<Adt17>) -> Option<Bytes> {
    let m1970 = v0;
    match m1970 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m1971 = v1;
            match m1971 {
                Adt17::C0(_) => {
                    None::<Bytes>
                }
                Adt17::C1(_, _) => {
                    None::<Bytes>
                }
                Adt17::C2(_, _, _) => {
                    None::<Bytes>
                }
                Adt17::C3(v2) => {
                    Some(v2)
                }
                Adt17::C4(_) => {
                    None::<Bytes>
                }
                Adt17::C5(_) => {
                    None::<Bytes>
                }
                Adt17::C6 => {
                    None::<Bytes>
                }
                Adt17::C7(_, _) => {
                    None::<Bytes>
                }
                Adt17::C8(_, _, _) => {
                    None::<Bytes>
                }
            }
        }
    }
}

pub fn f301(v0: u64) -> bool {
    f304({
        let a1972 = v0;
        let a1973 = 95u64;
        nat_eq(a1972, a1973)
    }, f304({
        let a1974 = v0;
        let a1975 = 45u64;
        nat_eq(a1974, a1975)
    }, f304({
        let a1976 = v0;
        let a1977 = 32u64;
        nat_eq(a1976, a1977)
    }, f304({
        let a1978 = v0;
        let a1979 = 46u64;
        nat_eq(a1978, a1979)
    }, {
        let a1980 = v0;
        let a1981 = 47u64;
        nat_eq(a1980, a1981)
    }))))
}

pub fn f302(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f303(v0: u64) -> bool {
    f302({
        let a1982 = 97u64;
        let a1983 = v0;
        nat_le(a1982, a1983)
    }, {
        let a1984 = v0;
        let a1985 = 122u64;
        nat_le(a1984, a1985)
    })
}

pub fn f304(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f305(v0: u64) -> bool {
    f302({
        let a1986 = 65u64;
        let a1987 = v0;
        nat_le(a1986, a1987)
    }, {
        let a1988 = v0;
        let a1989 = 90u64;
        nat_le(a1988, a1989)
    })
}

pub fn f306(v0: u64) -> bool {
    f302({
        let a1990 = 48u64;
        let a1991 = v0;
        nat_le(a1990, a1991)
    }, {
        let a1992 = v0;
        let a1993 = 57u64;
        nat_le(a1992, a1993)
    })
}

pub fn f307(v0: u64) -> bool {
    let a1994 = v0;
    let a1995 = 9u64;
    if if if nat_eq(a1994, a1995) {
        true
    } else {
        let a1996 = v0;
        let a1997 = 10u64;
        nat_eq(a1996, a1997)
    } {
        true
    } else {
        let a1998 = v0;
        let a1999 = 13u64;
        if nat_eq(a1998, a1999) {
            true
        } else {
            let a2000 = 32u64;
            let a2001 = v0;
            if nat_le(a2000, a2001) {
                let a2002 = v0;
                let a2003 = 126u64;
                nat_le(a2002, a2003)
            } else {
                false
            }
        }
    } {
        true
    } else {
        let a2004 = v0;
        let a2005 = 133u64;
        if if nat_eq(a2004, a2005) {
            true
        } else {
            let a2006 = 160u64;
            let a2007 = v0;
            if nat_le(a2006, a2007) {
                let a2008 = v0;
                let a2009 = 55295u64;
                nat_le(a2008, a2009)
            } else {
                false
            }
        } {
            true
        } else {
            let a2010 = 57344u64;
            let a2011 = v0;
            if if nat_le(a2010, a2011) {
                let a2012 = v0;
                let a2013 = 65533u64;
                nat_le(a2012, a2013)
            } else {
                false
            } {
                true
            } else {
                let a2014 = 65536u64;
                let a2015 = v0;
                if nat_le(a2014, a2015) {
                    let a2016 = v0;
                    let a2017 = 1114111u64;
                    nat_le(a2016, a2017)
                } else {
                    false
                }
            }
        }
    }
}

pub fn f308(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f161(v0.clone(), v1)?;
    if f158(f100(v0.clone(), v2)?) {
        Ok(true)
    } else {
        let a2018 = f100(v0, v2)?;
        let a2019 = 35u64;
        Ok(nat_eq(a2018, a2019))
    }
}

pub fn f309(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f380(v0, v1)
}

pub fn f310(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2020 = f100(v0, v1)?;
    let a2021 = 32u64;
    if nat_eq(a2020, a2021) {
        Ok(Some({
            let a2022 = v1;
            let a2023 = 1u64;
            nat_add(a2022, a2023)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f311(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f157(f100(v0, v1)?) {
        Ok(Some({
            let a2024 = v1;
            let a2025 = 1u64;
            nat_add(a2024, a2025)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f312(v0: u64) -> bool {
    let a2026 = v0;
    let a2027 = 44u64;
    if if if nat_eq(a2026, a2027) {
        true
    } else {
        let a2028 = v0;
        let a2029 = 91u64;
        nat_eq(a2028, a2029)
    } {
        true
    } else {
        let a2030 = v0;
        let a2031 = 93u64;
        if nat_eq(a2030, a2031) {
            true
        } else {
            let a2032 = v0;
            let a2033 = 123u64;
            nat_eq(a2032, a2033)
        }
    } {
        true
    } else {
        let a2034 = v0;
        let a2035 = 125u64;
        nat_eq(a2034, a2035)
    }
}

pub fn f313(v0: u64) -> u64 {
    let a2036 = v0;
    let a2037 = 45u64;
    if nat_eq(a2036, a2037) {
        1u64
    } else {
        let a2038 = v0;
        let a2039 = 43u64;
        if nat_eq(a2038, a2039) {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f314(v0: u64) -> u64 {
    let a2040 = 49u64;
    let a2041 = v0;
    if if nat_le(a2040, a2041) {
        let a2042 = v0;
        let a2043 = 57u64;
        nat_le(a2042, a2043)
    } else {
        false
    } {
        let a2044 = v0;
        let a2045 = 48u64;
        nat_sub(a2044, a2045)
    } else {
        0u64
    }
}

pub fn f315(v0: Bytes, v1: u64, v2: u64, v3: bool, v4: u64) -> R<Adt18> {
    let m2046 = f381(v0.clone(), v1, v2)?;
    match m2046 {
        Adt25::C0(v6, v7) => {
            let v8: u64 = v7;
            let v9: u64 = f382(v0.clone(), v8, v2)?;
            if f383(v0.clone(), v8, v2)? {
                let m2050 = f384(v0.clone(), v8, v2, v3, {
                    let a2048 = {
                        let a2047 = v0;
                        length_bytes(a2047)
                    };
                    let a2049 = 2u64;
                    nat_add(a2048, a2049)?
                }, Bytes::lit(&[]), v6)?;
                match m2050 {
                    Adt26::C0(v11, v12, v13) => {
                        Ok(Adt18::C0(Adt17::C3(f7(List::cons(v11, List::cons({
                            let a2051 = v4;
                            let a2052 = 1u64;
                            if nat_eq(a2051, a2052) {
                                Bytes::lit(&[])
                            } else {
                                Bytes::lit(&[0x0a])
                            }
                        }, List::cons({
                            let a2053 = v4;
                            let a2054 = 2u64;
                            if nat_eq(a2053, a2054) {
                                f318(v12)
                            } else {
                                Bytes::lit(&[])
                            }
                        }, List::<Bytes>::nil()))))), v13))
                    }
                    Adt26::C1(v10) => {
                        Ok(Adt18::C1(v10))
                    }
                }
            } else {
                Ok(Adt18::C0(Adt17::C3({
                    let a2055 = v4;
                    let a2056 = 2u64;
                    if nat_eq(a2055, a2056) {
                        f318(v6)
                    } else {
                        Bytes::lit(&[])
                    }
                }), {
                    let a2057 = f100(v0, v9)?;
                    let a2058 = 256u64;
                    if nat_eq(a2057, a2058) {
                        v9
                    } else {
                        v8
                    }
                }))
            }
        }
        Adt25::C1(v5) => {
            Ok(Adt18::C1(v5))
        }
    }
}

pub fn f316(v0: Fn7, p2063: u64, p2064: (u64, u64)) -> R<((u64, u64), bool)> {
    let m2062 = (p2063, p2064);
    {
        let mut m2062 = m2062;
        loop {
            let step = {
                let (v1, v2) = m2062;
                let m2059 = v1;
                if m2059 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2059 - 1;
                    let m2061 = {
                        let c2060: Fn7 = v0.clone();
                        c2060.apply(v2)?
                    };
                    match m2061 {
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
                Ok(next) => { m2062 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f317(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f385(v0, v1)
}

pub fn f318(v0: u64) -> Bytes {
    let m2065 = v0;
    if m2065 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2065 - 1;
        let a2066 = Bytes::lit(&[0x0a]);
        let a2067 = f318(v1);
        append_bytes(a2066, a2067)
    }
}

pub fn f319(v0: Bytes, v1: u64) -> R<u64> {
    let (_, h2068) = {
        let (h2069, _) = f316(Fn7::F386(v0.clone()), {
            let a2071 = {
                let a2070 = v0;
                length_bytes(a2070)
            };
            let a2072 = 1u64;
            nat_add(a2071, a2072)?
        }, (v1, 0u64))?;
        h2069
    };
    Ok(h2068)
}

pub fn f320(v0: u64, v1: u64) -> u64 {
    let a2073 = v0;
    let a2074 = v1;
    if nat_lt(a2073, a2074) {
        v1
    } else {
        v0
    }
}

pub fn f321(v0: Bytes, p2133: u64, v2: u64, v3: bool, p2134: u64, p2135: Bytes) -> R<Adt22> {
    let m2132 = (p2133, (p2134, p2135));
    {
        let mut m2132 = m2132;
        loop {
            let step = {
                let (v1, (v4, v5)) = m2132;
                let m2075 = v4;
                if m2075 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C1(f81(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                } else {
                    let v6 = m2075 - 1;
                    let v7: u64 = f100(v0.clone(), v1)?;
                    let a2076 = v7;
                    let a2077 = 34u64;
                    if nat_eq(a2076, a2077) {
                        Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C0(v5, {
                            let a2078 = v1;
                            let a2079 = 1u64;
                            nat_add(a2078, a2079)?
                        })))
                    } else {
                        let a2080 = v7;
                        let a2081 = 256u64;
                        if nat_eq(a2080, a2081) {
                            Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C1(f81(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                        } else {
                            let a2082 = v7;
                            let a2083 = 92u64;
                            if nat_eq(a2082, a2083) {
                                let v8: u64 = f100(v0.clone(), {
                                    let a2084 = v1;
                                    let a2085 = 1u64;
                                    nat_add(a2084, a2085)?
                                })?;
                                let m2086 = f387(v8)?;
                                match m2086 {
                                    None => {
                                        let a2091 = v8;
                                        let a2092 = 10u64;
                                        if nat_eq(a2091, a2092) {
                                            if v3 {
                                                Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C1(f388(v1)?)))
                                            } else {
                                                let m2095 = f389(v0.clone(), {
                                                    let a2093 = v1;
                                                    let a2094 = 1u64;
                                                    nat_add(a2093, a2094)?
                                                }, v2, v6, 0u64)?;
                                                match m2095 {
                                                    Adt25::C0(v11, v12) => {
                                                        Ok::<(u64, (u64, Bytes)), R<Adt22>>((v12, (v6, {
                                                            let a2096 = v5;
                                                            let a2097 = f318(v11);
                                                            append_bytes(a2096, a2097)
                                                        })))
                                                    }
                                                    Adt25::C1(v10) => {
                                                        Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C1(v10)))
                                                    }
                                                }
                                            }
                                        } else {
                                            let v13: u64 = {
                                                let a2098 = v8;
                                                let a2099 = 120u64;
                                                if nat_eq(a2098, a2099) {
                                                    2u64
                                                } else {
                                                    let a2100 = v8;
                                                    let a2101 = 117u64;
                                                    if nat_eq(a2100, a2101) {
                                                        4u64
                                                    } else {
                                                        let a2102 = v8;
                                                        let a2103 = 85u64;
                                                        if nat_eq(a2102, a2103) {
                                                            8u64
                                                        } else {
                                                            0u64
                                                        }
                                                    }
                                                }
                                            };
                                            let a2104 = v13;
                                            let a2105 = 0u64;
                                            if nat_eq(a2104, a2105) {
                                                Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C1(f81(Str::lit("an escape must be one of YAML 1.2.2 [42] to [62]"), v1)?)))
                                            } else {
                                                let m2108 = f390(v0.clone(), {
                                                    let a2106 = v1;
                                                    let a2107 = 2u64;
                                                    nat_add(a2106, a2107)?
                                                }, v13, 0u64)?;
                                                match m2108 {
                                                    None => {
                                                        Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C1(f81(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                    }
                                                    Some(v14) => {
                                                        let a2109 = 1114111u64;
                                                        let a2110 = v14;
                                                        if if nat_lt(a2109, a2110) {
                                                            true
                                                        } else {
                                                            let a2111 = 55296u64;
                                                            let a2112 = v14;
                                                            if nat_le(a2111, a2112) {
                                                                let a2113 = v14;
                                                                let a2114 = 57343u64;
                                                                nat_le(a2113, a2114)
                                                            } else {
                                                                false
                                                            }
                                                        } {
                                                            Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C1(f81(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                        } else {
                                                            Ok::<(u64, (u64, Bytes)), R<Adt22>>(({
                                                                let a2117 = {
                                                                    let a2115 = v1;
                                                                    let a2116 = 2u64;
                                                                    nat_add(a2115, a2116)?
                                                                };
                                                                let a2118 = v13;
                                                                nat_add(a2117, a2118)?
                                                            }, (v6, {
                                                                let a2119 = v5;
                                                                let a2120 = f295(v14)?;
                                                                append_bytes(a2119, a2120)
                                                            })))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Some(v9) => {
                                        Ok::<(u64, (u64, Bytes)), R<Adt22>>(({
                                            let a2087 = v1;
                                            let a2088 = 2u64;
                                            nat_add(a2087, a2088)?
                                        }, (v6, {
                                            let a2089 = v5;
                                            let a2090 = f295(v9)?;
                                            append_bytes(a2089, a2090)
                                        })))
                                    }
                                }
                            } else {
                                if if f157(v7) {
                                    true
                                } else {
                                    let a2121 = v7;
                                    let a2122 = 10u64;
                                    nat_eq(a2121, a2122)
                                } {
                                    let v15: u64 = f161(v0.clone(), v1)?;
                                    let a2123 = f100(v0.clone(), v15)?;
                                    let a2124 = 10u64;
                                    if nat_eq(a2123, a2124) {
                                        if v3 {
                                            Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C1(f388(v1)?)))
                                        } else {
                                            let m2125 = f389(v0.clone(), v15, v2, v6, 0u64)?;
                                            match m2125 {
                                                Adt25::C0(v17, v18) => {
                                                    Ok::<(u64, (u64, Bytes)), R<Adt22>>((v18, (v6, {
                                                        let a2126 = v5;
                                                        let a2127 = f329(v17);
                                                        append_bytes(a2126, a2127)
                                                    })))
                                                }
                                                Adt25::C1(v16) => {
                                                    Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C1(v16)))
                                                }
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (u64, Bytes)), R<Adt22>>((v15, (v6, {
                                            let a2128 = v5;
                                            let a2129 = f236(v0.clone(), v1, v15);
                                            append_bytes(a2128, a2129)
                                        })))
                                    }
                                } else {
                                    let v19: u64 = f391(v0.clone(), v1)?;
                                    Ok::<(u64, (u64, Bytes)), R<Adt22>>((v19, (v6, {
                                        let a2130 = v5;
                                        let a2131 = f236(v0.clone(), v1, v19);
                                        append_bytes(a2130, a2131)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2132 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f322(v0: Bytes, p2163: u64, v2: u64, v3: bool, p2164: u64, p2165: Bytes) -> R<Adt22> {
    let m2162 = (p2163, (p2164, p2165));
    {
        let mut m2162 = m2162;
        loop {
            let step = {
                let (v1, (v4, v5)) = m2162;
                let m2136 = v4;
                if m2136 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C1(f81(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                } else {
                    let v6 = m2136 - 1;
                    let v7: u64 = f100(v0.clone(), v1)?;
                    let a2137 = v7;
                    let a2138 = 39u64;
                    if nat_eq(a2137, a2138) {
                        let a2141 = f100(v0.clone(), {
                            let a2139 = v1;
                            let a2140 = 1u64;
                            nat_add(a2139, a2140)?
                        })?;
                        let a2142 = 39u64;
                        if nat_eq(a2141, a2142) {
                            Ok::<(u64, (u64, Bytes)), R<Adt22>>(({
                                let a2143 = v1;
                                let a2144 = 2u64;
                                nat_add(a2143, a2144)?
                            }, (v6, {
                                let a2145 = v5;
                                let a2146 = Bytes::lit(&[0x27]);
                                append_bytes(a2145, a2146)
                            })))
                        } else {
                            Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C0(v5, {
                                let a2147 = v1;
                                let a2148 = 1u64;
                                nat_add(a2147, a2148)?
                            })))
                        }
                    } else {
                        let a2149 = v7;
                        let a2150 = 256u64;
                        if nat_eq(a2149, a2150) {
                            Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C1(f81(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                        } else {
                            if if f157(v7) {
                                true
                            } else {
                                let a2151 = v7;
                                let a2152 = 10u64;
                                nat_eq(a2151, a2152)
                            } {
                                let v8: u64 = f161(v0.clone(), v1)?;
                                let a2153 = f100(v0.clone(), v8)?;
                                let a2154 = 10u64;
                                if nat_eq(a2153, a2154) {
                                    if v3 {
                                        Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C1(f388(v1)?)))
                                    } else {
                                        let m2155 = f389(v0.clone(), v8, v2, v6, 0u64)?;
                                        match m2155 {
                                            Adt25::C0(v10, v11) => {
                                                Ok::<(u64, (u64, Bytes)), R<Adt22>>((v11, (v6, {
                                                    let a2156 = v5;
                                                    let a2157 = f329(v10);
                                                    append_bytes(a2156, a2157)
                                                })))
                                            }
                                            Adt25::C1(v9) => {
                                                Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C1(v9)))
                                            }
                                        }
                                    }
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Adt22>>((v8, (v6, {
                                        let a2158 = v5;
                                        let a2159 = f236(v0.clone(), v1, v8);
                                        append_bytes(a2158, a2159)
                                    })))
                                }
                            } else {
                                let v12: u64 = f392(v0.clone(), v1)?;
                                Ok::<(u64, (u64, Bytes)), R<Adt22>>((v12, (v6, {
                                    let a2160 = v5;
                                    let a2161 = f236(v0.clone(), v1, v12);
                                    append_bytes(a2160, a2161)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2162 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f323(v0: Bytes, v1: u64, v2: u64) -> R<Result<u64, Bytes>> {
    let v3: (u64, bool) = {
        let (h2166, _) = f393(Fn8::F394(v2, v0.clone()), {
            let a2168 = {
                let a2167 = v0.clone();
                length_bytes(a2167)
            };
            let a2169 = 1u64;
            nat_add(a2168, a2169)?
        }, (v1, false))?;
        h2166
    };
    let (_, h2170) = v3;
    if h2170 {
        if f99(v0, {
            let (h2171, _) = v3;
            h2171
        })? {
            Ok(Err::<u64, Bytes>(f81(Str::lit("a document marker cannot appear inside a flow collection (YAML 1.2.2 [206])"), {
                let (h2172, _) = v3;
                h2172
            })?))
        } else {
            Ok(Err::<u64, Bytes>(f81(Str::lit("a line of a flow collection is indented less than its node (YAML 1.2.2 [80])"), {
                let (h2173, _) = v3;
                h2173
            })?))
        }
    } else {
        Ok(Ok::<u64, Bytes>({
            let (h2174, _) = v3;
            h2174
        }))
    }
}

pub fn f324(v0: Bytes, v1: u64, v2: u64) -> R<Adt22> {
    let v3: u64 = f100(v0.clone(), v1)?;
    let a2175 = v3;
    let a2176 = 34u64;
    if if nat_eq(a2175, a2176) {
        true
    } else {
        let a2177 = v3;
        let a2178 = 39u64;
        nat_eq(a2177, a2178)
    } {
        f231(v0, v1, v2, false)
    } else {
        let m2179 = f164(v0.clone(), v1)?;
        match m2179 {
            None => {
                let a2180 = v3;
                let a2181 = 91u64;
                if if nat_eq(a2180, a2181) {
                    true
                } else {
                    let a2182 = v3;
                    let a2183 = 123u64;
                    nat_eq(a2182, a2183)
                } {
                    Ok(Adt22::C1(f81(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [142])"), v1)?))
                } else {
                    if f233(v0.clone(), v1, true)? {
                        let v5: u64 = f234(v0.clone(), v1, true)?;
                        Ok(Adt22::C0(f236(v0, v1, v5), v5))
                    } else {
                        Ok(Adt22::C1(f81(Str::lit("a flow mapping key must be a scalar (YAML 1.2.2 [142])"), v1)?))
                    }
                }
            }
            Some(v4) => {
                Ok(Adt22::C1(v4))
            }
        }
    }
}

pub fn f325(v0: u64) -> R<bool> {
    let a2184 = f395(f8(Str::lit(",[]{}-?:#&*!|>'\"@`")), v0)?;
    let a2185 = 18u64;
    if nat_lt(a2184, a2185) {
        Ok(true)
    } else {
        let a2186 = v0;
        let a2187 = 37u64;
        Ok(nat_eq(a2186, a2187))
    }
}

pub fn f326(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f396(v1, v0, v2)
}

pub fn f327(v0: u64) -> R<Bytes> {
    f81(Str::lit("a mapping value is not allowed here: a plain scalar holds ': ' (YAML 1.2.2 [130])"), v0)
}

pub fn f328(v0: Bytes, v1: u64) -> R<(u64, u64)> {
    let (h2188, _) = f316(Fn7::F397(v0.clone()), {
        let a2190 = {
            let a2189 = v0;
            length_bytes(a2189)
        };
        let a2191 = 1u64;
        nat_add(a2190, a2191)?
    }, (v1, 0u64))?;
    Ok(h2188)
}

pub fn f329(v0: u64) -> Bytes {
    let a2192 = v0;
    let a2193 = 0u64;
    if nat_eq(a2192, a2193) {
        Bytes::lit(&[0x20])
    } else {
        f318(v0)
    }
}

pub fn f330(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt18> {
    let v4: (u64, u64) = f398(v0.clone(), v1, 2u64)?;
    let a2196 = {
        let (h2194, _) = v4;
        h2194
    };
    let a2197 = {
        let a2195 = v0.clone();
        length_bytes(a2195)
    };
    if nat_eq(a2196, a2197) {
        if f399(false, {
            let (_, h2198) = v4;
            h2198
        }) {
            Ok(Adt18::C0(Adt17::C1(false, {
                let (_, h2199) = v4;
                h2199
            }), v3))
        } else {
            Ok(Adt18::C1(f333(v2)?))
        }
    } else {
        Ok(Adt18::C0(Adt17::C3(v0), v3))
    }
}

pub fn f331(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2200 = v0.clone();
        length_bytes(a2200)
    };
    let v2: u64 = f100(v0.clone(), 0u64)?;
    let v3: u64 = {
        let a2201 = v2;
        let a2202 = 43u64;
        if if nat_eq(a2201, a2202) {
            true
        } else {
            let a2203 = v2;
            let a2204 = 45u64;
            nat_eq(a2203, a2204)
        } {
            1u64
        } else {
            0u64
        }
    };
    let v4: u64 = f400(v0.clone(), v3)?;
    let v5: bool = {
        let a2205 = f100(v0.clone(), v4)?;
        let a2206 = 46u64;
        nat_eq(a2205, a2206)
    };
    let v6: u64 = if v5 {
        let a2207 = v4;
        let a2208 = 1u64;
        nat_add(a2207, a2208)?
    } else {
        v4
    };
    let v7: u64 = if v5 {
        f400(v0.clone(), v6)?
    } else {
        v4
    };
    let v8: u64 = f100(v0.clone(), v7)?;
    let v9: bool = {
        let a2209 = v8;
        let a2210 = 101u64;
        if nat_eq(a2209, a2210) {
            true
        } else {
            let a2211 = v8;
            let a2212 = 69u64;
            nat_eq(a2211, a2212)
        }
    };
    let v10: u64 = f100(v0.clone(), {
        let a2213 = v7;
        let a2214 = 1u64;
        nat_add(a2213, a2214)?
    })?;
    let v11: bool = if v9 {
        let a2215 = v10;
        let a2216 = 43u64;
        if nat_eq(a2215, a2216) {
            true
        } else {
            let a2217 = v10;
            let a2218 = 45u64;
            nat_eq(a2217, a2218)
        }
    } else {
        false
    };
    let v12: u64 = if v9 {
        if v11 {
            let a2219 = v7;
            let a2220 = 2u64;
            nat_add(a2219, a2220)?
        } else {
            let a2221 = v7;
            let a2222 = 1u64;
            nat_add(a2221, a2222)?
        }
    } else {
        v7
    };
    let v13: u64 = if v9 {
        f400(v0.clone(), v12)?
    } else {
        v7
    };
    let v14: Bytes = f236(v0.clone(), v3, v4);
    let v15: u64 = f401(v14.clone())?;
    let a2223 = v13;
    let a2224 = v1;
    if if if nat_eq(a2223, a2224) {
        let a2225 = v3;
        let a2226 = v4;
        if nat_lt(a2225, a2226) {
            true
        } else {
            if v5 {
                let a2227 = v6;
                let a2228 = v7;
                nat_lt(a2227, a2228)
            } else {
                false
            }
        }
    } else {
        false
    } {
        if f169(v9) {
            true
        } else {
            let a2229 = v12;
            let a2230 = v13;
            nat_lt(a2229, a2230)
        }
    } else {
        false
    } {
        Ok(Some(f7(List::cons({
            let a2231 = v2;
            let a2232 = 45u64;
            if nat_eq(a2231, a2232) {
                Bytes::lit(&[0x2d])
            } else {
                Bytes::lit(&[])
            }
        }, List::cons({
            let a2234 = v15;
            let a2235 = {
                let a2233 = v14.clone();
                length_bytes(a2233)
            };
            if nat_eq(a2234, a2235) {
                Bytes::lit(&[0x30])
            } else {
                f236(v14.clone(), v15, {
                    let a2236 = v14;
                    length_bytes(a2236)
                })
            }
        }, List::cons({
            let a2237 = v6;
            let a2238 = v7;
            if nat_lt(a2237, a2238) {
                let a2239 = Bytes::lit(&[0x2e]);
                let a2240 = f236(v0.clone(), v6, v7);
                append_bytes(a2239, a2240)
            } else {
                Bytes::lit(&[])
            }
        }, List::cons(if v9 {
            let a2243 = if if v11 {
                let a2241 = v10;
                let a2242 = 45u64;
                nat_eq(a2241, a2242)
            } else {
                false
            } {
                Bytes::lit(&[0x65, 0x2d])
            } else {
                Bytes::lit(&[0x65])
            };
            let a2244 = f236(v0, v12, v13);
            append_bytes(a2243, a2244)
        } else {
            Bytes::lit(&[])
        }, List::<Bytes>::nil())))))))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f332(v0: Bytes, v1: u64) -> R<Option<(u64, Adt17)>> {
    let v2: bool = {
        let a2245 = f137(v0.clone(), v1)?;
        let a2246 = 45u64;
        nat_eq(a2245, a2246)
    };
    let v3: u64 = if v2 {
        let a2247 = v1;
        let a2248 = 1u64;
        nat_add(a2247, a2248)?
    } else {
        v1
    };
    let v4: u64 = f400(v0.clone(), v3)?;
    let v5: u64 = {
        let a2249 = v4;
        let a2250 = v3;
        nat_sub(a2249, a2250)
    };
    let v6: bool = {
        let a2251 = f137(v0.clone(), v4)?;
        let a2252 = 46u64;
        nat_eq(a2251, a2252)
    };
    let v7: u64 = if v6 {
        let a2253 = v4;
        let a2254 = 1u64;
        nat_add(a2253, a2254)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f400(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f137(v0.clone(), v8)?;
    let v10: bool = {
        let a2255 = v9;
        let a2256 = 101u64;
        if nat_eq(a2255, a2256) {
            true
        } else {
            let a2257 = v9;
            let a2258 = 69u64;
            nat_eq(a2257, a2258)
        }
    };
    let v11: u64 = f137(v0.clone(), {
        let a2259 = v8;
        let a2260 = 1u64;
        nat_add(a2259, a2260)?
    })?;
    let v12: bool = f302(v10, f304({
        let a2261 = v11;
        let a2262 = 43u64;
        nat_eq(a2261, a2262)
    }, {
        let a2263 = v11;
        let a2264 = 45u64;
        nat_eq(a2263, a2264)
    }));
    let v13: u64 = if v10 {
        if v12 {
            let a2265 = v8;
            let a2266 = 2u64;
            nat_add(a2265, a2266)?
        } else {
            let a2267 = v8;
            let a2268 = 1u64;
            nat_add(a2267, a2268)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f400(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f32(v0.clone(), v13, {
        let a2269 = v14;
        let a2270 = v13;
        nat_sub(a2269, a2270)
    });
    let v16: u64 = f401(v15.clone())?;
    let v17: Bytes = f32(v15.clone(), v16, {
        let a2272 = {
            let a2271 = v15;
            length_bytes(a2271)
        };
        let a2273 = v16;
        nat_sub(a2272, a2273)
    });
    let a2274 = v5;
    let a2275 = 0u64;
    if if nat_eq(a2274, a2275) {
        true
    } else {
        if f302({
            let a2276 = 1u64;
            let a2277 = v5;
            nat_lt(a2276, a2277)
        }, {
            let a2278 = f137(v0.clone(), v3)?;
            let a2279 = 48u64;
            nat_eq(a2278, a2279)
        }) {
            true
        } else {
            if f302(v6, {
                let a2280 = v8;
                let a2281 = v7;
                nat_eq(a2280, a2281)
            }) {
                true
            } else {
                f304(if v10 {
                    let a2282 = v14;
                    let a2283 = v13;
                    nat_eq(a2282, a2283)
                } else {
                    false
                }, {
                    let a2285 = 18u64;
                    let a2286 = {
                        let a2284 = v17.clone();
                        length_bytes(a2284)
                    };
                    nat_lt(a2285, a2286)
                })
            }
        }
    } {
        Ok(None::<(u64, Adt17)>)
    } else {
        let m2290 = {
            let a2288 = {
                let a2287 = v17.clone();
                length_bytes(a2287)
            };
            let a2289 = 0u64;
            if nat_eq(a2288, a2289) {
                Some(0u64)
            } else {
                f402(v17)?
            }
        };
        match m2290 {
            None => {
                Ok(None::<(u64, Adt17)>)
            }
            Some(v18) => {
                let m2295 = f403(f32(v0.clone(), v3, v5), f32(v0, v7, {
                    let a2291 = v8;
                    let a2292 = v7;
                    nat_sub(a2291, a2292)
                }), f302(v12, {
                    let a2293 = v11;
                    let a2294 = 45u64;
                    nat_eq(a2293, a2294)
                }), v18)?;
                match m2295 {
                    None => {
                        Ok(None::<(u64, Adt17)>)
                    }
                    Some(v19) => {
                        let a2298 = 19u64;
                        let a2299 = {
                            let a2297 = {
                                let (h2296, _) = v19.clone();
                                h2296
                            };
                            length_bytes(a2297)
                        };
                        if nat_lt(a2298, a2299) {
                            Ok(None::<(u64, Adt17)>)
                        } else {
                            let m2301 = f402({
                                let (h2300, _) = v19.clone();
                                h2300
                            })?;
                            match m2301 {
                                None => {
                                    Ok(None::<(u64, Adt17)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f302(v2, {
                                        let a2302 = 0u64;
                                        let a2303 = v20;
                                        nat_lt(a2302, a2303)
                                    });
                                    if f399(v21, v20) {
                                        Ok(Some((v14, {
                                            let a2305 = {
                                                let (_, h2304) = v19.clone();
                                                h2304
                                            };
                                            let a2306 = 0u64;
                                            if nat_eq(a2305, a2306) {
                                                Adt17::C1(v21, v20)
                                            } else {
                                                Adt17::C2(v21, v20, {
                                                    let (_, h2307) = v19;
                                                    h2307
                                                })
                                            }
                                        })))
                                    } else {
                                        Ok(None::<(u64, Adt17)>)
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

pub fn f333(v0: u64) -> R<Bytes> {
    f81(Str::lit("a number outside the range Studio.Value holds is not read (YAML 1.2.2 section 10.3.2)"), v0)
}

pub fn f334(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f404(v0, v1)
}

pub fn f335(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f405(v0, v1)
}

pub fn f336(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m2308 = v0.clone();
    match m2308.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a2311 = {
                let (h2309, _) = v2;
                h2309
            };
            let a2312 = {
                let (h2310, _) = v1.clone();
                h2310
            };
            if equal(a2311, a2312) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f337(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f406(f407(Fn9::F408, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m2317 = {
        let (h2313, _) = f409(Fn10::F410, {
            let a2315 = {
                let a2314 = v0;
                length_list(a2314)
            };
            let a2316 = 1u64;
            nat_add(a2315, a2316)?
        }, v1)?;
        h2313
    };
    match m2317.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f338(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2322 = {
        let a2319 = f242({
            let (h2318, _) = v0;
            h2318
        })?;
        let a2320 = Bytes::lit(&[0x3a]);
        append_bytes(a2319, a2320)
    };
    let a2323 = {
        let (_, h2321) = v0;
        h2321
    };
    Ok(append_bytes(a2322, a2323))
}

pub fn f339(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f411(v1, v0, v2)
}

pub fn f340(v0: u64) -> bool {
    if f341(v0) {
        true
    } else {
        let a2324 = 65u64;
        let a2325 = v0;
        if if nat_le(a2324, a2325) {
            let a2326 = v0;
            let a2327 = 70u64;
            nat_le(a2326, a2327)
        } else {
            false
        } {
            true
        } else {
            let a2328 = 97u64;
            let a2329 = v0;
            if nat_le(a2328, a2329) {
                let a2330 = v0;
                let a2331 = 102u64;
                nat_le(a2330, a2331)
            } else {
                false
            }
        }
    }
}

pub fn f341(v0: u64) -> bool {
    let a2332 = 48u64;
    let a2333 = v0;
    if nat_le(a2332, a2333) {
        let a2334 = v0;
        let a2335 = 57u64;
        nat_le(a2334, a2335)
    } else {
        false
    }
}

pub fn f342(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f85(v0.clone(), v1)?;
    let a2336 = v2;
    let a2337 = 69u64;
    if if nat_eq(a2336, a2337) {
        true
    } else {
        let a2338 = v2;
        let a2339 = 101u64;
        nat_eq(a2338, a2339)
    } {
        let v3: u64 = f85(v0.clone(), {
            let a2340 = v1;
            let a2341 = 1u64;
            nat_add(a2340, a2341)?
        })?;
        let v4: u64 = {
            let a2342 = v3;
            let a2343 = 43u64;
            if if nat_eq(a2342, a2343) {
                true
            } else {
                let a2344 = v3;
                let a2345 = 45u64;
                nat_eq(a2344, a2345)
            } {
                let a2346 = v1;
                let a2347 = 2u64;
                nat_add(a2346, a2347)?
            } else {
                let a2348 = v1;
                let a2349 = 1u64;
                nat_add(a2348, a2349)?
            }
        };
        let v5: u64 = f251(v0, 0u64, v4)?;
        let a2350 = v4;
        let a2351 = v5;
        if nat_lt(a2350, a2351) {
            Ok(v5)
        } else {
            Ok(v1)
        }
    } else {
        Ok(v1)
    }
}

pub fn f343(v0: Adt19, v1: Bytes) -> bool {
    let a2352 = f344(v0.clone());
    let a2353 = 3u64;
    if !nat_eq(a2352, a2353) {
        f35(f345(v0), v1)
    } else {
        false
    }
}

pub fn f344(v0: Adt19) -> u64 {
    let m2354 = v0;
    match m2354 {
        Adt19::C0(v1, _) => {
            v1
        }
    }
}

pub fn f345(v0: Adt19) -> Bytes {
    let m2355 = v0;
    match m2355 {
        Adt19::C0(_, v2) => {
            v2
        }
    }
}

pub fn f346(v0: Bytes) -> Bytes {
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

pub fn f347(v0: Bytes) -> bool {
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

pub fn f348(p2364: u64, p2365: List<Adt19>, p2366: List<Adt13>) -> R<Result<(List<Adt13>, List<Adt19>), Bytes>> {
    let m2363 = (p2364, (p2365, p2366));
    {
        let mut m2363 = m2363;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2363;
                let m2356 = v0;
                if m2356 == 0 {
                    Err::<(u64, (List<Adt19>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt19>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt19>), Bytes>(f8(Str::lit("a dictionary has more members than the input has tokens")))))
                } else {
                    let v3 = m2356 - 1;
                    if f254(v1.clone(), Str::lit("}")) {
                        let m2357 = f261(f256(v1), Str::lit(";"));
                        match m2357 {
                            Ok(v5) => {
                                Err::<(u64, (List<Adt19>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt19>), Bytes>>>(Ok(Ok::<(List<Adt13>, List<Adt19>), Bytes>((f412(v2, List::<Adt13>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<Adt19>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt19>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt19>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m2358 = f183(v1)?;
                        match m2358 {
                            Ok(v7) => {
                                let v8: bool = f254(v7.clone(), Str::lit("required"));
                                let m2359 = f266(v3, if v8 {
                                    f256(v7)
                                } else {
                                    v7
                                }, 0u64)?;
                                match m2359 {
                                    Ok(v10) => {
                                        let m2360 = f260(f267(v10.clone()));
                                        match m2360 {
                                            Ok(v12) => {
                                                if if f254(f262(v12.clone()), Str::lit("=")) {
                                                    !v8
                                                } else {
                                                    false
                                                } {
                                                    let m2361 = f350(f256(f262(v12.clone())))?;
                                                    match m2361 {
                                                        Ok(v14) => {
                                                            Ok::<(u64, (List<Adt19>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt19>), Bytes>>>((v3, (v14, List::cons(Adt13::C0(f264(v12), f357(f268(v10)), 0u64), v2))))
                                                        }
                                                        Err(v13) => {
                                                            Err::<(u64, (List<Adt19>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt19>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt19>), Bytes>(v13)))
                                                        }
                                                    }
                                                } else {
                                                    let m2362 = f261(f262(v12.clone()), Str::lit(";"));
                                                    match m2362 {
                                                        Ok(v16) => {
                                                            Ok::<(u64, (List<Adt19>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt19>), Bytes>>>((v3, (v16, List::cons(Adt13::C0(f264(v12), f357(f268(v10)), if v8 {
                                                                0u64
                                                            } else {
                                                                1u64
                                                            }), v2))))
                                                        }
                                                        Err(v15) => {
                                                            Err::<(u64, (List<Adt19>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt19>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt19>), Bytes>(v15)))
                                                        }
                                                    }
                                                }
                                            }
                                            Err(v11) => {
                                                Err::<(u64, (List<Adt19>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt19>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt19>), Bytes>(v11)))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Err::<(u64, (List<Adt19>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt19>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt19>), Bytes>(v9)))
                                    }
                                }
                            }
                            Err(v6) => {
                                Err::<(u64, (List<Adt19>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt19>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt19>), Bytes>(v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2363 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f349(p2375: u64, p2376: List<Adt19>, p2377: List<Adt13>) -> R<Result<(List<Adt13>, List<Adt19>), Bytes>> {
    let m2374 = (p2375, (p2376, p2377));
    {
        let mut m2374 = m2374;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2374;
                let m2367 = v0;
                if m2367 == 0 {
                    Err::<(u64, (List<Adt19>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt19>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt19>), Bytes>(f8(Str::lit("an interface has more members than the input has tokens")))))
                } else {
                    let v3 = m2367 - 1;
                    if f254(v1.clone(), Str::lit("}")) {
                        let m2368 = f261(f256(v1), Str::lit(";"));
                        match m2368 {
                            Ok(v5) => {
                                Err::<(u64, (List<Adt19>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt19>), Bytes>>>(Ok(Ok::<(List<Adt13>, List<Adt19>), Bytes>((f412(v2, List::<Adt13>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<Adt19>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt19>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt19>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m2369 = f183(v1)?;
                        match m2369 {
                            Ok(v7) => {
                                let v8: List<Adt19> = if if f254(v7.clone(), Str::lit("stringifier")) {
                                    true
                                } else {
                                    f254(v7.clone(), Str::lit("inherit"))
                                } {
                                    f256(v7.clone())
                                } else {
                                    v7.clone()
                                };
                                let v9: List<Adt19> = if f254(v8.clone(), Str::lit("readonly")) {
                                    f256(v8)
                                } else {
                                    v8
                                };
                                if f254(v9.clone(), Str::lit("attribute")) {
                                    let m2370 = f266(v3, f256(v9), 0u64)?;
                                    match m2370 {
                                        Ok(v11) => {
                                            let m2371 = f260(f267(v11.clone()));
                                            match m2371 {
                                                Ok(v13) => {
                                                    let m2372 = f261(f262(v13.clone()), Str::lit(";"));
                                                    match m2372 {
                                                        Ok(v15) => {
                                                            Ok::<(u64, (List<Adt19>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt19>), Bytes>>>((v3, (v15, List::cons(Adt13::C0(f264(v13), f357(f268(v11)), 0u64), v2))))
                                                        }
                                                        Err(v14) => {
                                                            Err::<(u64, (List<Adt19>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt19>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt19>), Bytes>(v14)))
                                                        }
                                                    }
                                                }
                                                Err(v12) => {
                                                    Err::<(u64, (List<Adt19>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt19>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt19>), Bytes>(v12)))
                                                }
                                            }
                                        }
                                        Err(v10) => {
                                            Err::<(u64, (List<Adt19>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt19>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt19>), Bytes>(v10)))
                                        }
                                    }
                                } else {
                                    let m2373 = f350(v7)?;
                                    match m2373 {
                                        Ok(v17) => {
                                            Ok::<(u64, (List<Adt19>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt19>), Bytes>>>((v3, (v17, v2)))
                                        }
                                        Err(v16) => {
                                            Err::<(u64, (List<Adt19>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt19>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt19>), Bytes>(v16)))
                                        }
                                    }
                                }
                            }
                            Err(v6) => {
                                Err::<(u64, (List<Adt19>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt19>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt19>), Bytes>(v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2374 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f350(v0: List<Adt19>) -> R<Result<List<Adt19>, Bytes>> {
    Ok(f255({
        let a2379 = {
            let a2378 = v0.clone();
            length_list(a2378)
        };
        let a2380 = 1u64;
        nat_add(a2379, a2380)?
    }, v0, List::<Bytes>::nil(), true))
}

pub fn f351(v0: List<Adt19>) -> Bytes {
    let m2381 = v0;
    match m2381.uncons() {
        None => {
            f8(Str::lit("the end of the input"))
        }
        Some((v1, _)) => {
            f7(List::cons(f8(Str::lit("'")), List::cons(f345(v1), List::cons(f8(Str::lit("'")), List::<Bytes>::nil()))))
        }
    }
}

pub fn f352(v0: Bytes) -> Bytes {
    if f145(v0.clone(), f8(Str::lit("_"))) {
        f32(v0.clone(), 1u64, {
            let a2383 = {
                let a2382 = v0;
                length_bytes(a2382)
            };
            let a2384 = 1u64;
            nat_sub(a2383, a2384)
        })
    } else {
        v0
    }
}

pub fn f353(v0: Adt14, v1: List<Adt19>) -> Result<Adt24, Bytes> {
    if f254(v1.clone(), Str::lit("?")) {
        Ok::<Adt24, Bytes>(Adt24::C0(f354(Str::lit("Option"), v0), f256(v1)))
    } else {
        Ok::<Adt24, Bytes>(Adt24::C0(v0, v1))
    }
}

pub fn f354(v0: Str, v1: Adt14) -> Adt14 {
    let v2: Adt14 = f357(v1);
    Adt14::C0(0u64, f7(List::cons(f8(Str::lit("(")), List::cons({
        let a2385 = v0;
        utf8_encode(a2385)
    }, List::cons(f8(Str::lit(" ")), List::cons(f202(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), f203(v2.clone()), f200(v2), Bytes::lit(&[]))
}

pub fn f355(v0: Bytes) -> Adt14 {
    Adt14::C0(0u64, Bytes::lit(&[]), List::<Bytes>::nil(), v0, Bytes::lit(&[]))
}

pub fn f356(v0: Bytes) -> Option<Adt14> {
    f413(v0, f414())
}

pub fn f357(v0: Adt14) -> Adt14 {
    let a2386 = f201(v0.clone());
    let a2387 = 0u64;
    if nat_eq(a2386, a2387) {
        v0
    } else {
        Adt14::C0(0u64, Bytes::lit(&[]), f203(v0.clone()), if f35(f200(v0.clone()), Bytes::lit(&[])) {
            f415()
        } else {
            f200(v0)
        }, Bytes::lit(&[]))
    }
}

pub fn f358(v0: Adt14) -> Bytes {
    let m2388 = v0;
    match m2388 {
        Adt14::C0(_, _, _, _, v5) => {
            v5
        }
    }
}

pub fn f359(v0: Adt13, v1: List<Adt13>) -> R<List<Adt13>> {
    let m2389 = v1;
    match m2389.uncons() {
        None => {
            Ok(List::cons(v0, List::<Adt13>::nil()))
        }
        Some((v2, v3)) => {
            if f416({
                let a2391 = {
                    let a2390 = f282(v0.clone());
                    length_bytes(a2390)
                };
                let a2392 = 1u64;
                nat_add(a2391, a2392)?
            }, f282(v0.clone()), f282(v2.clone()), 0u64)? {
                Ok(List::cons(v0, List::cons(v2, v3)))
            } else {
                Ok(List::cons(v2, f359(v0, v3)?))
            }
        }
    }
}

pub fn f360(v0: List<Adt12>, v1: Bytes) -> List<Adt13> {
    let m2393 = f121(v0.clone(), 1u64, v1.clone());
    match m2393 {
        None => {
            f279(v0, 6u64, v1)
        }
        Some(v2) => {
            f277(f278(v2), f279(v0, 6u64, v1))
        }
    }
}

pub fn f361(v0: Bytes) -> R<Option<Bytes>> {
    let m2395 = f417({
        let a2394 = v0.clone();
        length_bytes(a2394)
    }, v0.clone(), 0u64, Bytes::lit(&[]))?;
    match m2395 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a2397 = {
                let a2396 = v1.clone();
                length_bytes(a2396)
            };
            let a2398 = 0u64;
            if nat_eq(a2397, a2398) {
                let a2400 = {
                    let a2399 = v0;
                    length_bytes(a2399)
                };
                let a2401 = 0u64;
                if nat_eq(a2400, a2401) {
                    Ok(Some(f8(Str::lit("u"))))
                } else {
                    Ok(None::<Bytes>)
                }
            } else {
                Ok(Some(f363(v1)?))
            }
        }
    }
}

pub fn f362(v0: Bytes) -> R<Option<Bytes>> {
    if f418({
        let a2402 = v0.clone();
        length_bytes(a2402)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f419(v0)?;
        if f420(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m2405 = {
                let a2403 = v1.clone();
                let a2404 = 0u64;
                index_bytes(a2403, a2404)
            };
            match m2405 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f306(f41(v2)?) {
                        Ok(Some({
                            let a2406 = f8(Str::lit("n"));
                            let a2407 = v1;
                            append_bytes(a2406, a2407)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f421({
            let a2408 = v0.clone();
            length_bytes(a2408)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f363(v0: Bytes) -> R<Bytes> {
    if f113(v0.clone(), f422()) {
        f423(v0)
    } else {
        Ok(v0)
    }
}

pub fn f364(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2411 = {
        let a2409 = v0;
        let a2410 = v1;
        index_bytes(a2409, a2410)
    };
    match m2411 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f41(v2)?;
            if f302({
                let a2412 = 128u64;
                let a2413 = v3;
                nat_le(a2412, a2413)
            }, {
                let a2414 = v3;
                let a2415 = 191u64;
                nat_le(a2414, a2415)
            }) {
                Ok(Some({
                    let a2416 = v3;
                    let a2417 = 128u64;
                    nat_sub(a2416, a2417)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f365(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f302({
        let a2418 = v1;
        let a2419 = v0;
        nat_le(a2418, a2419)
    }, f302({
        let a2420 = v0;
        let a2421 = v2;
        nat_le(a2420, a2421)
    }, f304({
        let a2422 = v0;
        let a2423 = 55296u64;
        nat_lt(a2422, a2423)
    }, {
        let a2424 = 57343u64;
        let a2425 = v0;
        nat_lt(a2424, a2425)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f366() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{3}/\u{7f}\u{3}@\u{0}\u{3}\u{7f}}\u{4}\u{0}\u{0}C\u{7f}\u{7f}"))
}

pub fn f367(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a2426 = v1;
        let a2427 = 3u64;
        nat_mul(a2426, a2427)?
    };
    let a2438 = {
        let a2434 = {
            let a2428 = f137(v0.clone(), v2)?;
            let a2429 = 16384u64;
            nat_mul(a2428, a2429)?
        };
        let a2435 = {
            let a2432 = f137(v0.clone(), {
                let a2430 = v2;
                let a2431 = 1u64;
                nat_add(a2430, a2431)?
            })?;
            let a2433 = 128u64;
            nat_mul(a2432, a2433)?
        };
        nat_add(a2434, a2435)?
    };
    let a2439 = f137(v0, {
        let a2436 = v2;
        let a2437 = 2u64;
        nat_add(a2436, a2437)?
    })?;
    nat_add(a2438, a2439)
}

pub fn f368(v0: u64) -> R<bool> {
    f208(v0, f424())
}

pub fn f369(v0: u64) -> R<bool> {
    f208(v0, f425())
}

pub fn f370(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f426(v1, v0, v2)
}

pub fn f371(v0: Bytes, v1: u64) -> Bytes {
    f32(v0, v1, 1u64)
}

pub fn f372() -> Bytes {
    f8(Str::lit("\u{0}\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\u{9}\u{a}\u{b}\u{c}\u{d}\u{e}\u{f}\u{10}\u{11}\u{12}\u{13}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{1b}\u{1c}\u{1d}\u{1e}\u{1f} !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~\u{7f}"))
}

pub fn f373() -> Bytes {
    f8(Str::lit("\u{80}\u{c0}\u{100}\u{140}\u{180}\u{1c0}\u{200}\u{240}\u{280}\u{2c0}\u{300}\u{340}\u{380}\u{3c0}\u{400}\u{440}\u{480}\u{4c0}\u{500}\u{540}\u{580}\u{5c0}\u{600}\u{640}\u{680}\u{6c0}\u{700}\u{740}\u{780}\u{7c0}"))
}

pub fn f374(v0: u64) -> R<Bytes> {
    Ok(f371(f427(), {
        let a2445 = {
            let a2443 = {
                let a2440 = v0;
                let a2441 = 64u64;
                let a2442 = 0u64;
                nat_rem(a2440, a2441, a2442)
            };
            let a2444 = 2u64;
            nat_mul(a2443, a2444)?
        };
        let a2446 = 1u64;
        nat_add(a2445, a2446)?
    }))
}

pub fn f375() -> Bytes {
    f8(Str::lit("\u{800}\u{1000}\u{2000}\u{3000}\u{4000}\u{5000}\u{6000}\u{7000}\u{8000}\u{9000}\u{a000}\u{b000}\u{c000}\u{d000}\u{e000}\u{f000}"))
}

pub fn f376(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<Option<u64>> {
    if f428(f85(v0, v3)?, v1, v2)? {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a2447 = v3;
            let a2448 = 1u64;
            nat_add(a2447, a2448)?
        }))
    }
}

pub fn f377(v0: Bytes) -> Result<Bytes, Bytes> {
    Err::<Bytes, Bytes>(v0)
}

pub fn f378() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f379(v0: Bytes) -> R<Option<Bytes>> {
    let m2449 = f131(v0.clone(), f8(Str::lit(" ")), 0u64)?;
    match m2449 {
        None => {
            let m2450 = f131(v0.clone(), f8(Str::lit(":")), 0u64)?;
            match m2450 {
                None => {
                    let v3: Bytes = f7(List::cons(f8(Str::lit(" ")), List::cons(v0, List::cons(f8(Str::lit(":")), List::<Bytes>::nil()))));
                    let m2451 = f131(f429(), v3.clone(), 0u64)?;
                    match m2451 {
                        None => {
                            Ok(None::<Bytes>)
                        }
                        Some(v4) => {
                            let v5: u64 = {
                                let a2453 = v4;
                                let a2454 = {
                                    let a2452 = v3;
                                    length_bytes(a2452)
                                };
                                nat_add(a2453, a2454)?
                            };
                            let m2455 = f131(f429(), f8(Str::lit(" ")), v5)?;
                            match m2455 {
                                None => {
                                    Ok(None::<Bytes>)
                                }
                                Some(v6) => {
                                    Ok(Some(f32(f429(), v5, {
                                        let a2456 = v6;
                                        let a2457 = v5;
                                        nat_sub(a2456, a2457)
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

pub fn f380(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2458 = 0u64;
    let a2459 = v1;
    if if nat_lt(a2458, a2459) {
        f169({
            let a2462 = f100(v0, {
                let a2460 = v1;
                let a2461 = 1u64;
                nat_sub(a2460, a2461)
            })?;
            let a2463 = 10u64;
            nat_eq(a2462, a2463)
        })
    } else {
        false
    } {
        Ok(Some({
            let a2464 = v1;
            let a2465 = 1u64;
            nat_sub(a2464, a2465)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f381(v0: Bytes, v1: u64, v2: u64) -> R<Adt25> {
    let v3: (u64, u64) = {
        let (h2466, _) = f316(Fn7::F430(v2, v0.clone()), {
            let a2468 = {
                let a2467 = v0.clone();
                length_bytes(a2467)
            };
            let a2469 = 1u64;
            nat_add(a2468, a2469)?
        }, (v1, 0u64))?;
        h2466
    };
    let v4: u64 = f382(v0.clone(), {
        let (h2470, _) = v3;
        h2470
    }, v2)?;
    let a2471 = f100(v0, v4)?;
    let a2472 = 9u64;
    if if nat_eq(a2471, a2472) {
        let a2476 = {
            let a2474 = v4;
            let a2475 = {
                let (h2473, _) = v3;
                h2473
            };
            nat_sub(a2474, a2475)
        };
        let a2477 = v2;
        nat_lt(a2476, a2477)
    } else {
        false
    } {
        Ok(Adt25::C1(f81(Str::lit("a tab cannot indent a block scalar line (YAML 1.2.2 [63], [70])"), v4)?))
    } else {
        Ok(Adt25::C0({
            let (_, h2478) = v3;
            h2478
        }, {
            let (h2479, _) = v3;
            h2479
        }))
    }
}

pub fn f382(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h2480, _) = f128(Fn4::F431(v2, v0.clone(), v1), {
        let a2482 = {
            let a2481 = v0;
            length_bytes(a2481)
        };
        let a2483 = 1u64;
        nat_add(a2482, a2483)?
    }, v1)?;
    Ok(h2480)
}

pub fn f383(v0: Bytes, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = f382(v0.clone(), v1, v2)?;
    let a2486 = {
        let a2484 = v3;
        let a2485 = v1;
        nat_sub(a2484, a2485)
    };
    let a2487 = v2;
    if nat_eq(a2486, a2487) {
        Ok(f169({
            let a2488 = f100(v0, v3)?;
            let a2489 = 256u64;
            nat_eq(a2488, a2489)
        }))
    } else {
        Ok(false)
    }
}

pub fn f384(v0: Bytes, p2499: u64, v2: u64, v3: bool, p2500: u64, p2501: Bytes, p2502: u64) -> R<Adt26> {
    let m2498 = (p2499, (p2500, (p2501, p2502)));
    {
        let mut m2498 = m2498;
        loop {
            let step = {
                let (v1, (v4, (v5, v6))) = m2498;
                let m2490 = v4;
                if m2490 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<Adt26>>(Ok(Adt26::C1(f81(Str::lit("a block scalar has more lines than the input"), v1)?)))
                } else {
                    let v7 = m2490 - 1;
                    let v8: u64 = f382(v0.clone(), v1, v2)?;
                    let v9: bool = f169(f157(f100(v0.clone(), v8)?));
                    let v10: u64 = f238(v0.clone(), v8)?;
                    let v11: Bytes = f7(List::cons(v5, List::cons(f318(v6), List::cons(f236(v0.clone(), v8, v10), List::<Bytes>::nil()))));
                    let m2491 = f381(v0.clone(), f167(v0.clone(), v10)?, v2)?;
                    match m2491 {
                        Adt25::C0(v13, v14) => {
                            let v15: u64 = v14;
                            let v16: u64 = f382(v0.clone(), v15, v2)?;
                            if f383(v0.clone(), v15, v2)? {
                                Ok::<(u64, (u64, (Bytes, u64))), R<Adt26>>((v15, (v7, ({
                                    let a2494 = v11;
                                    let a2495 = if if if v3 {
                                        v9
                                    } else {
                                        false
                                    } {
                                        f169(f157(f100(v0.clone(), v16)?))
                                    } else {
                                        false
                                    } {
                                        let a2492 = v13;
                                        let a2493 = 0u64;
                                        if nat_eq(a2492, a2493) {
                                            Bytes::lit(&[0x20])
                                        } else {
                                            Bytes::lit(&[])
                                        }
                                    } else {
                                        Bytes::lit(&[0x0a])
                                    };
                                    append_bytes(a2494, a2495)
                                }, v13))))
                            } else {
                                Err::<(u64, (u64, (Bytes, u64))), R<Adt26>>(Ok(Adt26::C0(v11, v13, {
                                    let a2496 = f100(v0.clone(), v16)?;
                                    let a2497 = 256u64;
                                    if nat_eq(a2496, a2497) {
                                        v16
                                    } else {
                                        v15
                                    }
                                })))
                            }
                        }
                        Adt25::C1(v12) => {
                            Err::<(u64, (u64, (Bytes, u64))), R<Adt26>>(Ok(Adt26::C1(v12)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2498 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f385(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f160(v0.clone(), {
        let (h2503, _) = v1;
        h2503
    })?;
    let a2504 = f100(v0, v2)?;
    let a2505 = 10u64;
    if nat_eq(a2504, a2505) {
        Ok(Some(({
            let a2506 = v2;
            let a2507 = 1u64;
            nat_add(a2506, a2507)?
        }, f320({
            let (_, h2508) = v1;
            h2508
        }, {
            let a2510 = v2;
            let a2511 = {
                let (h2509, _) = v1;
                h2509
            };
            nat_sub(a2510, a2511)
        }))))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f386(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f432(v0, v1)
}

pub fn f387(v0: u64) -> R<Option<u64>> {
    let v1: u64 = f395(f8(Str::lit("abtnvfr")), v0)?;
    let a2512 = v1;
    let a2513 = 7u64;
    if nat_lt(a2512, a2513) {
        Ok(Some({
            let a2514 = v1;
            let a2515 = 7u64;
            nat_add(a2514, a2515)?
        }))
    } else {
        let a2516 = v0;
        let a2517 = 48u64;
        if nat_eq(a2516, a2517) {
            Ok(Some(0u64))
        } else {
            let a2518 = v0;
            let a2519 = 9u64;
            if nat_eq(a2518, a2519) {
                Ok(Some(9u64))
            } else {
                let a2520 = v0;
                let a2521 = 101u64;
                if nat_eq(a2520, a2521) {
                    Ok(Some(27u64))
                } else {
                    let a2522 = v0;
                    let a2523 = 78u64;
                    if nat_eq(a2522, a2523) {
                        Ok(Some(133u64))
                    } else {
                        let a2524 = v0;
                        let a2525 = 95u64;
                        if nat_eq(a2524, a2525) {
                            Ok(Some(160u64))
                        } else {
                            let a2526 = v0;
                            let a2527 = 76u64;
                            if nat_eq(a2526, a2527) {
                                Ok(Some(8232u64))
                            } else {
                                let a2528 = v0;
                                let a2529 = 80u64;
                                if nat_eq(a2528, a2529) {
                                    Ok(Some(8233u64))
                                } else {
                                    let a2530 = f395(f8(Str::lit(" \"/\\")), v0)?;
                                    let a2531 = 4u64;
                                    if nat_lt(a2530, a2531) {
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

pub fn f388(v0: u64) -> R<Bytes> {
    f81(Str::lit("an implicit key must be on one line (YAML 1.2.2 [154])"), v0)
}

pub fn f389(v0: Bytes, p2546: u64, v2: u64, p2547: u64, p2548: u64) -> R<Adt25> {
    let m2545 = (p2546, (p2547, p2548));
    {
        let mut m2545 = m2545;
        loop {
            let step = {
                let (v1, (v3, v4)) = m2545;
                let m2532 = v3;
                if m2532 == 0 {
                    Err::<(u64, (u64, u64)), R<Adt25>>(Ok(Adt25::C1(f81(Str::lit("the input ends inside a scalar"), v1)?)))
                } else {
                    let v5 = m2532 - 1;
                    let v6: u64 = {
                        let a2533 = v1;
                        let a2534 = 1u64;
                        nat_add(a2533, a2534)?
                    };
                    let v7: u64 = f160(v0.clone(), v6)?;
                    let v8: u64 = f161(v0.clone(), v7)?;
                    let v9: u64 = f100(v0.clone(), v8)?;
                    if f99(v0.clone(), v6)? {
                        Err::<(u64, (u64, u64)), R<Adt25>>(Ok(Adt25::C1(f81(Str::lit("a document marker cannot appear inside a scalar (YAML 1.2.2 [206])"), v6)?)))
                    } else {
                        let a2535 = v9;
                        let a2536 = 10u64;
                        if nat_eq(a2535, a2536) {
                            Ok::<(u64, (u64, u64)), R<Adt25>>((v8, (v5, {
                                let a2537 = v4;
                                let a2538 = 1u64;
                                nat_add(a2537, a2538)?
                            })))
                        } else {
                            let a2539 = v9;
                            let a2540 = 256u64;
                            if nat_eq(a2539, a2540) {
                                Err::<(u64, (u64, u64)), R<Adt25>>(Ok(Adt25::C1(f81(Str::lit("the input ends inside a quoted scalar (YAML 1.2.2 [109], [120])"), v8)?)))
                            } else {
                                let a2543 = {
                                    let a2541 = v7;
                                    let a2542 = v6;
                                    nat_sub(a2541, a2542)
                                };
                                let a2544 = v2;
                                if nat_lt(a2543, a2544) {
                                    Err::<(u64, (u64, u64)), R<Adt25>>(Ok(Adt25::C1(f81(Str::lit("a continuation line of a scalar is indented less than its node (YAML 1.2.2 [69])"), v7)?)))
                                } else {
                                    Err::<(u64, (u64, u64)), R<Adt25>>(Ok(Adt25::C0(v4, v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2545 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f390(v0: Bytes, p2558: u64, p2559: u64, p2560: u64) -> R<Option<u64>> {
    let m2557 = (p2558, (p2559, p2560));
    {
        let mut m2557 = m2557;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2557;
                let m2549 = v2;
                if m2549 == 0 {
                    Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(Some(v3)))
                } else {
                    let v4 = m2549 - 1;
                    let m2550 = f433(f100(v0.clone(), v1)?);
                    match m2550 {
                        None => {
                            Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(None::<u64>))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, u64)), R<Option<u64>>>(({
                                let a2551 = v1;
                                let a2552 = 1u64;
                                nat_add(a2551, a2552)?
                            }, (v4, {
                                let a2555 = {
                                    let a2553 = v3;
                                    let a2554 = 16u64;
                                    nat_mul(a2553, a2554)?
                                };
                                let a2556 = v5;
                                nat_add(a2555, a2556)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2557 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f391(v0: Bytes, v1: u64) -> R<u64> {
    let (h2561, _) = f128(Fn4::F434(v0.clone()), {
        let a2563 = {
            let a2562 = v0;
            length_bytes(a2562)
        };
        let a2564 = 1u64;
        nat_add(a2563, a2564)?
    }, v1)?;
    Ok(h2561)
}

pub fn f392(v0: Bytes, v1: u64) -> R<u64> {
    let (h2565, _) = f128(Fn4::F435(v0.clone()), {
        let a2567 = {
            let a2566 = v0;
            length_bytes(a2566)
        };
        let a2568 = 1u64;
        nat_add(a2567, a2568)?
    }, v1)?;
    Ok(h2565)
}

pub fn f393(v0: Fn8, p2573: u64, p2574: (u64, bool)) -> R<((u64, bool), bool)> {
    let m2572 = (p2573, p2574);
    {
        let mut m2572 = m2572;
        loop {
            let step = {
                let (v1, v2) = m2572;
                let m2569 = v1;
                if m2569 == 0 {
                    Err::<(u64, (u64, bool)), R<((u64, bool), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2569 - 1;
                    let m2571 = {
                        let c2570: Fn8 = v0.clone();
                        c2570.apply(v2)?
                    };
                    match m2571 {
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
                Ok(next) => { m2572 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f394(v0: u64, v1: Bytes, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    f436(v1, v0, v2)
}

pub fn f395(v0: Bytes, v1: u64) -> R<u64> {
    let (h2575, _) = f128(Fn4::F437(v0.clone(), v1), {
        let a2577 = {
            let a2576 = v0;
            length_bytes(a2576)
        };
        let a2578 = 1u64;
        nat_add(a2577, a2578)?
    }, 0u64)?;
    Ok(h2575)
}

pub fn f396(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = {
        let (h2579, _) = v2;
        h2579
    };
    let v4: u64 = f100(v0.clone(), v3)?;
    if f158(v4) {
        Ok(None::<(u64, u64)>)
    } else {
        if f157(v4) {
            Ok(Some(({
                let a2580 = v3;
                let a2581 = 1u64;
                nat_add(a2580, a2581)?
            }, {
                let (_, h2582) = v2;
                h2582
            })))
        } else {
            let a2583 = v4;
            let a2584 = 58u64;
            if if nat_eq(a2583, a2584) {
                f169(f229(f100(v0.clone(), {
                    let a2585 = v3;
                    let a2586 = 1u64;
                    nat_add(a2585, a2586)?
                })?, v1))
            } else {
                false
            } {
                Ok(None::<(u64, u64)>)
            } else {
                let a2587 = v4;
                let a2588 = 35u64;
                if if nat_eq(a2587, a2588) {
                    f157(f100(v0, {
                        let a2589 = v3;
                        let a2590 = 1u64;
                        nat_sub(a2589, a2590)
                    })?)
                } else {
                    false
                } {
                    Ok(None::<(u64, u64)>)
                } else {
                    if if v1 {
                        f312(v4)
                    } else {
                        false
                    } {
                        Ok(None::<(u64, u64)>)
                    } else {
                        Ok(Some(({
                            let a2591 = v3;
                            let a2592 = 1u64;
                            nat_add(a2591, a2592)?
                        }, {
                            let a2593 = v3;
                            let a2594 = 1u64;
                            nat_add(a2593, a2594)?
                        })))
                    }
                }
            }
        }
    }
}

pub fn f397(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f438(v0, v1)
}

pub fn f398(v0: Bytes, v1: u64, v2: u64) -> R<(u64, u64)> {
    let (h2595, _) = f316(Fn7::F439(v1, v0.clone()), {
        let a2597 = {
            let a2596 = v0;
            length_bytes(a2596)
        };
        let a2598 = 1u64;
        nat_add(a2597, a2598)?
    }, (v2, 0u64))?;
    Ok(h2595)
}

pub fn f399(v0: bool, v1: u64) -> bool {
    if v0 {
        let a2599 = 0u64;
        let a2600 = v1;
        if nat_lt(a2599, a2600) {
            let a2601 = v1;
            let a2602 = 9223372036854775808u64;
            nat_le(a2601, a2602)
        } else {
            false
        }
    } else {
        let a2603 = v1;
        let a2604 = 9223372036854775807u64;
        nat_le(a2603, a2604)
    }
}

pub fn f400(v0: Bytes, v1: u64) -> R<u64> {
    let (h2605, _) = f128(Fn4::F440(v0.clone()), {
        let a2607 = {
            let a2606 = v0;
            length_bytes(a2606)
        };
        let a2608 = 1u64;
        nat_add(a2607, a2608)?
    }, v1)?;
    Ok(h2605)
}

pub fn f401(v0: Bytes) -> R<u64> {
    let (h2609, _) = f128(Fn4::F441(v0.clone()), {
        let a2611 = {
            let a2610 = v0;
            length_bytes(a2610)
        };
        let a2612 = 1u64;
        nat_add(a2611, a2612)?
    }, 0u64)?;
    Ok(h2609)
}

pub fn f402(v0: Bytes) -> R<Option<u64>> {
    if f442(v0.clone())? {
        Ok(Some({
            let (_, h2613) = {
                let (h2614, _) = f316(Fn7::F443(v0.clone()), {
                    let a2615 = v0;
                    length_bytes(a2615)
                }, (0u64, 0u64))?;
                h2614
            };
            h2613
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f403(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f32(v1.clone(), 0u64, {
        let a2617 = {
            let a2616 = v1.clone();
            length_bytes(a2616)
        };
        let a2618 = f444(v1)?;
        nat_sub(a2617, a2618)
    });
    let v5: Bytes = {
        let a2619 = v0;
        let a2620 = v4.clone();
        append_bytes(a2619, a2620)
    };
    let v6: u64 = f401(v5.clone())?;
    let v7: Bytes = f32(v5.clone(), v6, {
        let a2622 = {
            let a2621 = v5;
            length_bytes(a2621)
        };
        let a2623 = v6;
        nat_sub(a2622, a2623)
    });
    let v8: u64 = {
        let a2624 = v4;
        length_bytes(a2624)
    };
    let v9: u64 = {
        let a2625 = v7.clone();
        length_bytes(a2625)
    };
    let a2626 = v9;
    let a2627 = 0u64;
    if nat_eq(a2626, a2627) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f304(v2, {
            let a2628 = v3;
            let a2629 = v8;
            nat_lt(a2628, a2629)
        }) {
            let v10: u64 = if v2 {
                let a2630 = v3;
                let a2631 = v8;
                nat_add(a2630, a2631)?
            } else {
                let a2632 = v8;
                let a2633 = v3;
                nat_sub(a2632, a2633)
            };
            let v11: u64 = f444(v7.clone())?;
            let a2634 = v10;
            let a2635 = v11;
            if nat_le(a2634, a2635) {
                Ok(Some((f32(v7, 0u64, {
                    let a2636 = v9;
                    let a2637 = v10;
                    nat_sub(a2636, a2637)
                }), 0u64)))
            } else {
                let a2640 = {
                    let a2638 = v10;
                    let a2639 = v11;
                    nat_sub(a2638, a2639)
                };
                let a2641 = 19u64;
                if nat_le(a2640, a2641) {
                    Ok(Some((f32(v7, 0u64, {
                        let a2642 = v9;
                        let a2643 = v11;
                        nat_sub(a2642, a2643)
                    }), {
                        let a2644 = v10;
                        let a2645 = v11;
                        nat_sub(a2644, a2645)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a2646 = v3;
                let a2647 = v8;
                nat_sub(a2646, a2647)
            };
            let a2650 = {
                let a2648 = v9;
                let a2649 = v12;
                nat_add(a2648, a2649)?
            };
            let a2651 = 19u64;
            if nat_le(a2650, a2651) {
                Ok(Some(({
                    let a2652 = v7;
                    let a2653 = f240(v12);
                    append_bytes(a2652, a2653)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f404(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f158(f100(v0, v1)?) {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a2654 = v1;
            let a2655 = 1u64;
            nat_add(a2654, a2655)?
        }))
    }
}

pub fn f405(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m2659 = {
        let a2657 = v0.clone();
        let a2658 = {
            let (h2656, _) = v1.clone();
            h2656
        };
        index_bytes(a2657, a2658)
    };
    match m2659 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2661 = {
                    let (h2660, _) = v1.clone();
                    h2660
                };
                let a2662 = 1u64;
                nat_add(a2661, a2662)?
            }, {
                let a2665 = {
                    let (_, h2663) = v1;
                    h2663
                };
                let a2666 = f445(v0, {
                    let (h2664, _) = v1;
                    h2664
                }, v2)?;
                append_bytes(a2665, a2666)
            })))
        }
    }
}

pub fn f406(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f446(Fn11::F447, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f407(v0: Fn9, p2670: List<List<(Bytes, Bytes)>>, p2671: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m2669 = (p2670, p2671);
    {
        let mut m2669 = m2669;
        loop {
            let step = {
                let (v1, v2) = m2669;
                let m2667 = v2;
                match m2667.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c2668: Fn9 = v0.clone();
                            c2668.apply(v1, v3)
                        }, v4))
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

pub fn f408(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f409(v0: Fn10, p2676: u64, p2677: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2675 = (p2676, p2677);
    {
        let mut m2675 = m2675;
        loop {
            let step = {
                let (v1, v2) = m2675;
                let m2672 = v1;
                if m2672 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2672 - 1;
                    let m2674 = {
                        let c2673: Fn10 = v0.clone();
                        c2673.apply(v2.clone())?
                    };
                    match m2674 {
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
                Ok(next) => { m2675 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f410(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f448(v0)
}

pub fn f411(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    if f449(v1, f85(v0, v2)?) {
        Ok(Some({
            let a2678 = v2;
            let a2679 = 1u64;
            nat_add(a2678, a2679)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f412(p2682: List<Adt13>, p2683: List<Adt13>) -> List<Adt13> {
    let m2681 = (p2682, p2683);
    {
        let mut m2681 = m2681;
        loop {
            let step = {
                let (v0, v1) = m2681;
                let m2680 = v0;
                match m2680.uncons() {
                    None => {
                        Err::<(List<Adt13>, List<Adt13>), List<Adt13>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt13>, List<Adt13>), List<Adt13>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2681 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f413(v0: Bytes, p2688: List<(Bytes, Adt14)>) -> Option<Adt14> {
    let m2687 = p2688;
    {
        let mut m2687 = m2687;
        loop {
            let step = {
                let v1 = m2687;
                let m2684 = v1;
                match m2684.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt14)>, Option<Adt14>>(None::<Adt14>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h2685, _) = v2;
                            h2685
                        }, v0.clone()) {
                            Err::<List<(Bytes, Adt14)>, Option<Adt14>>(Some({
                                let (_, h2686) = v2;
                                h2686
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt14)>, Option<Adt14>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2687 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f414() -> List<(Bytes, Adt14)> {
    List::cons((f8(Str::lit("boolean")), f265(f8(Str::lit("Bool")))), List::cons((f8(Str::lit("byte")), f265(f8(Str::lit("Int")))), List::cons((f8(Str::lit("octet")), f265(f8(Str::lit("Nat")))), List::cons((f8(Str::lit("bigint")), f265(f8(Str::lit("Int")))), List::cons((f8(Str::lit("float")), f265(f378())), List::cons((f8(Str::lit("double")), f265(f378())), List::cons((f8(Str::lit("DOMString")), f265(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("USVString")), f265(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ByteString")), f265(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ArrayBuffer")), f265(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("SharedArrayBuffer")), f265(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("DataView")), f265(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("Int8Array")), f265(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int16Array")), f265(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int32Array")), f265(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("BigInt64Array")), f265(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Uint8Array")), f265(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint16Array")), f265(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint32Array")), f265(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint8ClampedArray")), f265(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("BigUint64Array")), f265(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Float16Array")), f355(f8(Str::lit("Float16Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("Float32Array")), f355(f8(Str::lit("Float32Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("Float64Array")), f355(f8(Str::lit("Float64Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("any")), f355(f8(Str::lit("any is not translated, because it is the union of every Web IDL type and has no one LexLean type (Web IDL 2.13.1)")))), List::cons((f8(Str::lit("object")), f355(f8(Str::lit("object is not translated, because its values are references to JavaScript objects of any shape, which no LexLean type holds (Web IDL 2.13.20)")))), List::cons((f8(Str::lit("symbol")), f355(f8(Str::lit("symbol is not translated, because a symbol value has identity but no content that a LexLean type holds (Web IDL 2.13.21)")))), List::cons((f8(Str::lit("undefined")), f355(f8(Str::lit("undefined is not translated, because it is the type of the absence of a value (Web IDL 2.13.2)")))), List::<(Bytes, Adt14)>::nil()))))))))))))))))))))))))))))
}

pub fn f415() -> Bytes {
    f8(Str::lit("a union type is translated only as the whole body of a typedef, whose identifier names its inductive (Web IDL 2.11 and 2.13.32)"))
}

pub fn f416(p2699: u64, v1: Bytes, v2: Bytes, p2700: u64) -> R<bool> {
    let m2698 = (p2699, p2700);
    {
        let mut m2698 = m2698;
        loop {
            let step = {
                let (v0, v3) = m2698;
                let m2689 = v0;
                if m2689 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(false))
                } else {
                    let v4 = m2689 - 1;
                    let v5: u64 = f450(v1.clone(), v3)?;
                    let v6: u64 = f450(v2.clone(), v3)?;
                    let a2690 = v5;
                    let a2691 = v6;
                    if nat_lt(a2690, a2691) {
                        Err::<(u64, u64), R<bool>>(Ok(true))
                    } else {
                        let a2692 = v6;
                        let a2693 = v5;
                        if nat_lt(a2692, a2693) {
                            Err::<(u64, u64), R<bool>>(Ok(false))
                        } else {
                            let a2694 = v5;
                            let a2695 = 0u64;
                            if nat_eq(a2694, a2695) {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            } else {
                                Ok::<(u64, u64), R<bool>>((v4, {
                                    let a2696 = v3;
                                    let a2697 = 1u64;
                                    nat_add(a2696, a2697)?
                                }))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2698 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f417(p2713: u64, v1: Bytes, p2714: u64, p2715: Bytes) -> R<Option<Bytes>> {
    let m2712 = (p2713, (p2714, p2715));
    {
        let mut m2712 = m2712;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2712;
                let m2701 = v0;
                if m2701 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m2701 - 1;
                    let a2703 = {
                        let a2702 = v1.clone();
                        length_bytes(a2702)
                    };
                    let a2704 = v2;
                    if nat_le(a2703, a2704) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m2705 = f288(v1.clone(), v2)?;
                        match m2705 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a2707 = v2;
                                    let a2708 = {
                                        let (_, h2706) = v5;
                                        h2706
                                    };
                                    nat_add(a2707, a2708)?
                                }, {
                                    let a2710 = v3;
                                    let a2711 = f451(v1.clone(), v2, {
                                        let (h2709, _) = v5;
                                        h2709
                                    });
                                    append_bytes(a2710, a2711)
                                })))
                            }
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

pub fn f418(p2725: u64, v1: Bytes, p2726: u64) -> R<bool> {
    let m2724 = (p2725, p2726);
    {
        let mut m2724 = m2724;
        loop {
            let step = {
                let (v0, v2) = m2724;
                let m2716 = v0;
                if m2716 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m2716 - 1;
                    let m2719 = {
                        let a2717 = v1.clone();
                        let a2718 = v2;
                        index_bytes(a2717, a2718)
                    };
                    match m2719 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a2720 = f41(v4)?;
                            let a2721 = 128u64;
                            if nat_lt(a2720, a2721) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a2722 = v2;
                                    let a2723 = 1u64;
                                    nat_add(a2722, a2723)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2724 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f419(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f223(v0.clone(), 0u64, {
        let a2727 = v0;
        length_bytes(a2727)
    }, false, Bytes::lit(&[]))?;
    let m2730 = {
        let a2728 = v1.clone();
        let a2729 = 0u64;
        index_bytes(a2728, a2729)
    };
    match m2730 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f41(v2)?;
            if f305(v3) {
                let a2737 = f32({
                    let a2731 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a2731)
                }, {
                    let a2732 = v3;
                    let a2733 = 65u64;
                    nat_sub(a2732, a2733)
                }, 1u64);
                let a2738 = f32(v1.clone(), 1u64, {
                    let a2735 = {
                        let a2734 = v1;
                        length_bytes(a2734)
                    };
                    let a2736 = 1u64;
                    nat_sub(a2735, a2736)
                });
                Ok(append_bytes(a2737, a2738))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f420(v0: Bytes) -> R<bool> {
    let m2741 = {
        let a2739 = v0;
        let a2740 = 0u64;
        index_bytes(a2739, a2740)
    };
    match m2741 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f304(f303(f41(v1)?), f305(f41(v1)?)))
        }
    }
}

pub fn f421(p2754: u64, v1: Bytes, p2755: u64, p2756: Bytes) -> R<Option<Bytes>> {
    let m2753 = (p2754, (p2755, p2756));
    {
        let mut m2753 = m2753;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2753;
                let m2742 = v0;
                if m2742 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m2742 - 1;
                    let a2744 = {
                        let a2743 = v1.clone();
                        length_bytes(a2743)
                    };
                    let a2745 = v2;
                    if nat_le(a2744, a2745) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m2746 = f288(v1.clone(), v2)?;
                        match m2746 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a2748 = v2;
                                    let a2749 = {
                                        let (_, h2747) = v5;
                                        h2747
                                    };
                                    nat_add(a2748, a2749)?
                                }, {
                                    let a2751 = v3;
                                    let a2752 = f452({
                                        let (h2750, _) = v5;
                                        h2750
                                    });
                                    append_bytes(a2751, a2752)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2753 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f422() -> List<Bytes> {
    List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("termination_by")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("admit")), List::cons(f8(Str::lit("below")), List::cons(f8(Str::lit("brecOn")), List::cons(f8(Str::lit("casesOn")), List::cons(f8(Str::lit("ctorElim")), List::cons(f8(Str::lit("ctorElimType")), List::cons(f8(Str::lit("ctorIdx")), List::cons(f8(Str::lit("noConfusion")), List::cons(f8(Str::lit("noConfusionType")), List::cons(f8(Str::lit("rec")), List::cons(f8(Str::lit("recOn")), List::cons(f8(Str::lit("toCtorIdx")), List::<Bytes>::nil())))))))))))))))))))
}

pub fn f423(v0: Bytes) -> R<Bytes> {
    let m2759 = {
        let a2757 = v0.clone();
        let a2758 = 0u64;
        index_bytes(a2757, a2758)
    };
    match m2759 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            let a2765 = {
                let a2760 = f8(Str::lit("u"));
                let a2761 = f453(8u64, f41(v1)?, 4u64, Bytes::lit(&[]));
                append_bytes(a2760, a2761)
            };
            let a2766 = f32(v0.clone(), 1u64, {
                let a2763 = {
                    let a2762 = v0;
                    length_bytes(a2762)
                };
                let a2764 = 1u64;
                nat_sub(a2763, a2764)
            });
            Ok(append_bytes(a2765, a2766))
        }
    }
}

pub fn f424() -> Bytes {
    f8(Str::lit("\u{0}\u{0}:\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{5}\u{7f}\u{0}\u{6}p\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f425() -> Bytes {
    f8(Str::lit("\u{0}\u{0}-\u{0}\u{0}.\u{0}\u{0}0\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}7\u{0}\u{1}7\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@?\u{0}@@\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f426(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f85(v0, {
        let (h2767, _) = v2;
        h2767
    })?;
    let a2769 = 1114111u64;
    let a2770 = {
        let (_, h2768) = v2;
        h2768
    };
    if nat_lt(a2769, a2770) {
        Ok(None::<(u64, u64)>)
    } else {
        let v4: u64 = f454(v3, v1);
        let a2771 = v4;
        let a2772 = 16u64;
        if nat_eq(a2771, a2772) {
            Ok(None::<(u64, u64)>)
        } else {
            Ok(Some(({
                let a2774 = {
                    let (h2773, _) = v2;
                    h2773
                };
                let a2775 = 1u64;
                nat_add(a2774, a2775)?
            }, {
                let a2779 = {
                    let a2777 = {
                        let (_, h2776) = v2;
                        h2776
                    };
                    let a2778 = if v1 {
                        16u64
                    } else {
                        10u64
                    };
                    nat_mul(a2777, a2778)?
                };
                let a2780 = v4;
                nat_add(a2779, a2780)?
            })))
        }
    }
}

pub fn f427() -> Bytes {
    f8(Str::lit("\u{80}\u{81}\u{82}\u{83}\u{84}\u{85}\u{86}\u{87}\u{88}\u{89}\u{8a}\u{8b}\u{8c}\u{8d}\u{8e}\u{8f}\u{90}\u{91}\u{92}\u{93}\u{94}\u{95}\u{96}\u{97}\u{98}\u{99}\u{9a}\u{9b}\u{9c}\u{9d}\u{9e}\u{9f}\u{a0}\u{a1}\u{a2}\u{a3}\u{a4}\u{a5}\u{a6}\u{a7}\u{a8}\u{a9}\u{aa}\u{ab}\u{ac}\u{ad}\u{ae}\u{af}\u{b0}\u{b1}\u{b2}\u{b3}\u{b4}\u{b5}\u{b6}\u{b7}\u{b8}\u{b9}\u{ba}\u{bb}\u{bc}\u{bd}\u{be}\u{bf}"))
}

pub fn f428(v0: u64, v1: bool, v2: u64) -> R<bool> {
    if v1 {
        let a2781 = v0;
        let a2782 = v2;
        if nat_eq(a2781, a2782) {
            Ok(true)
        } else {
            f208(v0, f455())
        }
    } else {
        f208(v0, f456())
    }
}

pub fn f429() -> Bytes {
    f8(Str::lit(" string:anyAtomicType boolean:anyAtomicType decimal:anyAtomicType float:anyAtomicType double:anyAtomicType duration:anyAtomicType dateTime:anyAtomicType time:anyAtomicType date:anyAtomicType gYearMonth:anyAtomicType gYear:anyAtomicType gMonthDay:anyAtomicType gDay:anyAtomicType gMonth:anyAtomicType hexBinary:anyAtomicType base64Binary:anyAtomicType anyURI:anyAtomicType QName:anyAtomicType NOTATION:anyAtomicType normalizedString:string token:normalizedString language:token NMTOKEN:token Name:token NCName:Name ID:NCName IDREF:NCName ENTITY:NCName integer:decimal nonPositiveInteger:integer long:integer nonNegativeInteger:integer negativeInteger:nonPositiveInteger int:long short:int byte:short unsignedLong:nonNegativeInteger positiveInteger:nonNegativeInteger unsignedInt:unsignedLong unsignedShort:unsignedInt unsignedByte:unsignedShort yearMonthDuration:duration dayTimeDuration:duration dateTimeStamp:dateTime NMTOKENS:anySimpleType IDREFS:anySimpleType ENTITIES:anySimpleType anyAtomicType:anySimpleType "))
}

pub fn f430(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f457(v1, v0, v2)
}

pub fn f431(v0: u64, v1: Bytes, v2: u64, v3: u64) -> R<Option<u64>> {
    f458(v1, v2, v0, v3)
}

pub fn f432(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let a2785 = {
        let (h2783, _) = v1;
        h2783
    };
    let a2786 = {
        let a2784 = v0.clone();
        length_bytes(a2784)
    };
    if nat_lt(a2785, a2786) {
        Ok(Some((f167(v0, {
            let (h2787, _) = v1;
            h2787
        })?, {
            let a2789 = {
                let (_, h2788) = v1;
                h2788
            };
            let a2790 = 1u64;
            nat_add(a2789, a2790)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f433(v0: u64) -> Option<u64> {
    let a2791 = 48u64;
    let a2792 = v0;
    if if nat_le(a2791, a2792) {
        let a2793 = v0;
        let a2794 = 57u64;
        nat_le(a2793, a2794)
    } else {
        false
    } {
        Some({
            let a2795 = v0;
            let a2796 = 48u64;
            nat_sub(a2795, a2796)
        })
    } else {
        let a2797 = 97u64;
        let a2798 = v0;
        if if nat_le(a2797, a2798) {
            let a2799 = v0;
            let a2800 = 102u64;
            nat_le(a2799, a2800)
        } else {
            false
        } {
            Some({
                let a2801 = v0;
                let a2802 = 87u64;
                nat_sub(a2801, a2802)
            })
        } else {
            let a2803 = 65u64;
            let a2804 = v0;
            if if nat_le(a2803, a2804) {
                let a2805 = v0;
                let a2806 = 70u64;
                nat_le(a2805, a2806)
            } else {
                false
            } {
                Some({
                    let a2807 = v0;
                    let a2808 = 55u64;
                    nat_sub(a2807, a2808)
                })
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f434(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f459(v0, v1)
}

pub fn f435(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f460(v0, v1)
}

pub fn f436(v0: Bytes, v1: u64, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    let v3: u64 = {
        let (h2809, _) = v2;
        h2809
    };
    let v4: u64 = f100(v0.clone(), v3)?;
    let (_, h2810) = v2;
    if h2810 {
        Ok(None::<(u64, bool)>)
    } else {
        if f157(v4) {
            Ok(Some(({
                let a2811 = v3;
                let a2812 = 1u64;
                nat_add(a2811, a2812)?
            }, false)))
        } else {
            let a2813 = v4;
            let a2814 = 35u64;
            if if nat_eq(a2813, a2814) {
                let a2815 = v3;
                let a2816 = 0u64;
                if if nat_eq(a2815, a2816) {
                    true
                } else {
                    f157(f100(v0.clone(), {
                        let a2817 = v3;
                        let a2818 = 1u64;
                        nat_sub(a2817, a2818)
                    })?)
                } {
                    true
                } else {
                    let a2821 = f100(v0.clone(), {
                        let a2819 = v3;
                        let a2820 = 1u64;
                        nat_sub(a2819, a2820)
                    })?;
                    let a2822 = 10u64;
                    nat_eq(a2821, a2822)
                }
            } else {
                false
            } {
                Ok(Some((f238(v0, v3)?, false)))
            } else {
                let a2823 = v4;
                let a2824 = 10u64;
                if nat_eq(a2823, a2824) {
                    let v5: u64 = {
                        let a2825 = v3;
                        let a2826 = 1u64;
                        nat_add(a2825, a2826)?
                    };
                    let v6: u64 = f160(v0.clone(), v5)?;
                    let v7: u64 = f161(v0.clone(), v6)?;
                    if if f99(v0.clone(), v5)? {
                        true
                    } else {
                        let a2829 = {
                            let a2827 = v6;
                            let a2828 = v5;
                            nat_sub(a2827, a2828)
                        };
                        let a2830 = v1;
                        if if nat_lt(a2829, a2830) {
                            f169(f158(f100(v0.clone(), v7)?))
                        } else {
                            false
                        } {
                            f169({
                                let a2831 = f100(v0, v7)?;
                                let a2832 = 35u64;
                                nat_eq(a2831, a2832)
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

pub fn f437(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f461(v0, v1, v2)
}

pub fn f438(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f161(v0.clone(), {
        let (h2833, _) = v1;
        h2833
    })?;
    let a2834 = f100(v0, v2)?;
    let a2835 = 10u64;
    if nat_eq(a2834, a2835) {
        Ok(Some(({
            let a2836 = v2;
            let a2837 = 1u64;
            nat_add(a2836, a2837)?
        }, {
            let a2839 = {
                let (_, h2838) = v1;
                h2838
            };
            let a2840 = 1u64;
            nat_add(a2839, a2840)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f439(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f462(v1, v0, v2)
}

pub fn f440(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f463(v0, v1)
}

pub fn f441(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f464(v0, v1)
}

pub fn f442(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a2841 = v0.clone();
        length_bytes(a2841)
    };
    let a2842 = 0u64;
    let a2843 = v1;
    if nat_lt(a2842, a2843) {
        let a2844 = f465(v0.clone())?;
        let a2845 = v1;
        if nat_eq(a2844, a2845) {
            let a2846 = 1u64;
            let a2847 = v1;
            if nat_lt(a2846, a2847) {
                let m2850 = {
                    let a2848 = v0;
                    let a2849 = 0u64;
                    index_bytes(a2848, a2849)
                };
                match m2850 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a2851 = v2;
                        let a2852 = 48u8;
                        Ok(!equal(a2851, a2852))
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

pub fn f443(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f466(v0, v1)
}

pub fn f444(v0: Bytes) -> R<u64> {
    let (h2853, _) = f128(Fn4::F467(v0.clone()), {
        let a2855 = {
            let a2854 = v0;
            length_bytes(a2854)
        };
        let a2856 = 1u64;
        nat_add(a2855, a2856)?
    }, 0u64)?;
    Ok(h2853)
}

pub fn f445(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a2901 = {
            let a2877 = {
                let a2865 = {
                    let a2859 = {
                        let a2857 = v3;
                        let a2858 = 128u8;
                        fixed_u8::bit_and(a2857, a2858)
                    };
                    let a2860 = 128u8;
                    if equal(a2859, a2860) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2866 = {
                    let a2863 = {
                        let a2861 = v3;
                        let a2862 = 64u8;
                        fixed_u8::bit_and(a2861, a2862)
                    };
                    let a2864 = 64u8;
                    if equal(a2863, a2864) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2865, a2866)?
            };
            let a2878 = {
                let a2875 = {
                    let a2869 = {
                        let a2867 = v3;
                        let a2868 = 32u8;
                        fixed_u8::bit_and(a2867, a2868)
                    };
                    let a2870 = 32u8;
                    if equal(a2869, a2870) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2876 = {
                    let a2873 = {
                        let a2871 = v3;
                        let a2872 = 16u8;
                        fixed_u8::bit_and(a2871, a2872)
                    };
                    let a2874 = 16u8;
                    if equal(a2873, a2874) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2875, a2876)?
            };
            nat_add(a2877, a2878)?
        };
        let a2902 = {
            let a2899 = {
                let a2887 = {
                    let a2881 = {
                        let a2879 = v3;
                        let a2880 = 8u8;
                        fixed_u8::bit_and(a2879, a2880)
                    };
                    let a2882 = 8u8;
                    if equal(a2881, a2882) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2888 = {
                    let a2885 = {
                        let a2883 = v3;
                        let a2884 = 4u8;
                        fixed_u8::bit_and(a2883, a2884)
                    };
                    let a2886 = 4u8;
                    if equal(a2885, a2886) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2887, a2888)?
            };
            let a2900 = {
                let a2897 = {
                    let a2891 = {
                        let a2889 = v3;
                        let a2890 = 2u8;
                        fixed_u8::bit_and(a2889, a2890)
                    };
                    let a2892 = 2u8;
                    if equal(a2891, a2892) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2898 = {
                    let a2895 = {
                        let a2893 = v3;
                        let a2894 = 1u8;
                        fixed_u8::bit_and(a2893, a2894)
                    };
                    let a2896 = 1u8;
                    if equal(a2895, a2896) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2897, a2898)?
            };
            nat_add(a2899, a2900)?
        };
        nat_add(a2901, a2902)?
    };
    let a2903 = v4;
    let a2904 = 34u64;
    if nat_eq(a2903, a2904) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a2905 = v4;
        let a2906 = 92u64;
        if nat_eq(a2905, a2906) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a2907 = v4;
            let a2908 = 8u64;
            if nat_eq(a2907, a2908) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a2909 = v4;
                let a2910 = 12u64;
                if nat_eq(a2909, a2910) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a2911 = v4;
                    let a2912 = 10u64;
                    if nat_eq(a2911, a2912) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a2913 = v4;
                        let a2914 = 13u64;
                        if nat_eq(a2913, a2914) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a2915 = v4;
                            let a2916 = 9u64;
                            if nat_eq(a2915, a2916) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a2917 = v4;
                                let a2918 = 32u64;
                                if nat_lt(a2917, a2918) {
                                    let a2919 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a2920 = f468(v2)?;
                                    Ok(append_bytes(a2919, a2920))
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

pub fn f446(v0: Fn11, p2924: List<List<(Bytes, Bytes)>>, p2925: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2923 = (p2924, p2925);
    {
        let mut m2923 = m2923;
        loop {
            let step = {
                let (v1, v2) = m2923;
                let m2921 = v2;
                match m2921.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2922: Fn11 = v0.clone();
                            c2922.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2923 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f447(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f448(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2927 = {
        let a2926 = v0.clone();
        length_list(a2926)
    };
    let a2928 = 1u64;
    if nat_le(a2927, a2928) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f469(v0)?))
    }
}

pub fn f449(v0: u64, v1: u64) -> bool {
    let a2929 = v0;
    let a2930 = 0u64;
    if nat_eq(a2929, a2930) {
        f341(v1)
    } else {
        let a2931 = v0;
        let a2932 = 1u64;
        if nat_eq(a2931, a2932) {
            f340(v1)
        } else {
            let a2933 = v0;
            let a2934 = 2u64;
            if nat_eq(a2933, a2934) {
                f470(v1)
            } else {
                f471(v1)
            }
        }
    }
}

pub fn f450(v0: Bytes, v1: u64) -> R<u64> {
    let a2936 = v1;
    let a2937 = {
        let a2935 = v0.clone();
        length_bytes(a2935)
    };
    if nat_lt(a2936, a2937) {
        let a2938 = f85(v0, v1)?;
        let a2939 = 1u64;
        nat_add(a2938, a2939)
    } else {
        Ok(0u64)
    }
}

pub fn f451(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f304(f302(f304(f303(v2), f305(v2)), f302({
        let a2940 = v2;
        let a2941 = 117u64;
        !nat_eq(a2940, a2941)
    }, {
        let a2942 = v2;
        let a2943 = 85u64;
        !nat_eq(a2942, a2943)
    })), f302(f306(v2), {
        let a2944 = 0u64;
        let a2945 = v1;
        nat_lt(a2944, a2945)
    })) {
        f32(v0, v1, 1u64)
    } else {
        let a2946 = v2;
        let a2947 = 65536u64;
        if nat_lt(a2946, a2947) {
            let a2948 = f8(Str::lit("u"));
            let a2949 = f453(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a2948, a2949)
        } else {
            let a2950 = f8(Str::lit("U"));
            let a2951 = f453(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a2950, a2951)
        }
    }
}

pub fn f452(v0: u64) -> Bytes {
    let a2952 = f8(Str::lit("u"));
    let a2953 = f453(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a2952, a2953)
}

pub fn f453(p2970: u64, p2971: u64, p2972: u64, p2973: Bytes) -> Bytes {
    let m2969 = (p2970, (p2971, (p2972, p2973)));
    {
        let mut m2969 = m2969;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m2969;
                let m2954 = v0;
                if m2954 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m2954 - 1;
                    if f302({
                        let a2955 = v1;
                        let a2956 = 0u64;
                        nat_eq(a2955, a2956)
                    }, {
                        let a2957 = v2;
                        let a2958 = 0u64;
                        nat_eq(a2957, a2958)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a2959 = v1;
                            let a2960 = 16u64;
                            let a2961 = 0u64;
                            nat_quot(a2959, a2960, a2961)
                        }, ({
                            let a2962 = v2;
                            let a2963 = 1u64;
                            nat_sub(a2962, a2963)
                        }, {
                            let a2967 = f472({
                                let a2964 = v1;
                                let a2965 = 16u64;
                                let a2966 = 0u64;
                                nat_rem(a2964, a2965, a2966)
                            });
                            let a2968 = v3;
                            append_bytes(a2967, a2968)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m2969 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f454(v0: u64, v1: bool) -> u64 {
    let a2974 = v0;
    let a2975 = 48u64;
    if nat_lt(a2974, a2975) {
        16u64
    } else {
        let a2976 = v0;
        let a2977 = 57u64;
        if nat_le(a2976, a2977) {
            let a2978 = v0;
            let a2979 = 48u64;
            nat_sub(a2978, a2979)
        } else {
            if v1 {
                let a2980 = v0;
                let a2981 = 65u64;
                if nat_lt(a2980, a2981) {
                    16u64
                } else {
                    let a2982 = v0;
                    let a2983 = 70u64;
                    if nat_le(a2982, a2983) {
                        let a2984 = v0;
                        let a2985 = 55u64;
                        nat_sub(a2984, a2985)
                    } else {
                        let a2986 = v0;
                        let a2987 = 97u64;
                        if nat_lt(a2986, a2987) {
                            16u64
                        } else {
                            let a2988 = v0;
                            let a2989 = 102u64;
                            if nat_le(a2988, a2989) {
                                let a2990 = v0;
                                let a2991 = 87u64;
                                nat_sub(a2990, a2991)
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

pub fn f455() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f456() -> Bytes {
    f8(Str::lit("\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{0}]\u{0}\u{0}]\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f457(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f382(v0.clone(), {
        let (h2992, _) = v2;
        h2992
    }, v1)?;
    let a2993 = f100(v0, v3)?;
    let a2994 = 10u64;
    if nat_eq(a2993, a2994) {
        Ok(Some(({
            let a2995 = v3;
            let a2996 = 1u64;
            nat_add(a2995, a2996)?
        }, {
            let a2998 = {
                let (_, h2997) = v2;
                h2997
            };
            let a2999 = 1u64;
            nat_add(a2998, a2999)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f458(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a3000 = f100(v0, v3)?;
    let a3001 = 32u64;
    if if nat_eq(a3000, a3001) {
        let a3004 = {
            let a3002 = v3;
            let a3003 = v1;
            nat_sub(a3002, a3003)
        };
        let a3005 = v2;
        nat_lt(a3004, a3005)
    } else {
        false
    } {
        Ok(Some({
            let a3006 = v3;
            let a3007 = 1u64;
            nat_add(a3006, a3007)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f459(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f100(v0, v1)?;
    let a3008 = v2;
    let a3009 = 34u64;
    if if if if nat_eq(a3008, a3009) {
        true
    } else {
        let a3010 = v2;
        let a3011 = 92u64;
        nat_eq(a3010, a3011)
    } {
        true
    } else {
        f157(v2)
    } {
        true
    } else {
        f158(v2)
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3012 = v1;
            let a3013 = 1u64;
            nat_add(a3012, a3013)?
        }))
    }
}

pub fn f460(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f100(v0, v1)?;
    let a3014 = v2;
    let a3015 = 39u64;
    if if if nat_eq(a3014, a3015) {
        true
    } else {
        f157(v2)
    } {
        true
    } else {
        f158(v2)
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3016 = v1;
            let a3017 = 1u64;
            nat_add(a3016, a3017)?
        }))
    }
}

pub fn f461(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a3019 = v2;
    let a3020 = {
        let a3018 = v0.clone();
        length_bytes(a3018)
    };
    if if nat_lt(a3019, a3020) {
        f169({
            let a3021 = f100(v0, v2)?;
            let a3022 = v1;
            nat_eq(a3021, a3022)
        })
    } else {
        false
    } {
        Ok(Some({
            let a3023 = v2;
            let a3024 = 1u64;
            nat_add(a3023, a3024)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f462(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m3026 = f433(f100(v0, {
        let (h3025, _) = v2;
        h3025
    })?);
    match m3026 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v3) => {
            let a3027 = v3;
            let a3028 = v1;
            if nat_lt(a3027, a3028) {
                Ok(Some(({
                    let a3030 = {
                        let (h3029, _) = v2;
                        h3029
                    };
                    let a3031 = 1u64;
                    nat_add(a3030, a3031)?
                }, {
                    let a3035 = {
                        let a3033 = {
                            let (_, h3032) = v2;
                            h3032
                        };
                        let a3034 = v1;
                        nat_mul(a3033, a3034)?
                    };
                    let a3036 = v3;
                    nat_add(a3035, a3036)?
                })))
            } else {
                Ok(None::<(u64, u64)>)
            }
        }
    }
}

pub fn f463(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f137(v0, v1)?;
    let a3037 = 47u64;
    let a3038 = v2;
    if if nat_lt(a3037, a3038) {
        let a3039 = v2;
        let a3040 = 58u64;
        nat_lt(a3039, a3040)
    } else {
        false
    } {
        Ok(Some({
            let a3041 = v1;
            let a3042 = 1u64;
            nat_add(a3041, a3042)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f464(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a3043 = f137(v0, v1)?;
    let a3044 = 48u64;
    if nat_eq(a3043, a3044) {
        Ok(Some({
            let a3045 = v1;
            let a3046 = 1u64;
            nat_add(a3045, a3046)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f465(v0: Bytes) -> R<u64> {
    let (h3047, _) = f128(Fn4::F473(v0.clone()), {
        let a3048 = v0;
        length_bytes(a3048)
    }, 0u64)?;
    Ok(h3047)
}

pub fn f466(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m3052 = {
        let a3050 = v0;
        let a3051 = {
            let (h3049, _) = v1;
            h3049
        };
        index_bytes(a3050, a3051)
    };
    match m3052 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a3054 = {
                    let (h3053, _) = v1;
                    h3053
                };
                let a3055 = 1u64;
                nat_add(a3054, a3055)?
            }, {
                let a3059 = {
                    let a3057 = {
                        let (_, h3056) = v1;
                        h3056
                    };
                    let a3058 = 10u64;
                    nat_mul(a3057, a3058)?
                };
                let a3060 = f474(v2)?;
                nat_add(a3059, a3060)?
            })))
        }
    }
}

pub fn f467(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f475(v0, v1)
}

pub fn f468(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3105 = {
            let a3081 = {
                let a3069 = {
                    let a3063 = {
                        let a3061 = v1;
                        let a3062 = 128u8;
                        fixed_u8::bit_and(a3061, a3062)
                    };
                    let a3064 = 128u8;
                    if equal(a3063, a3064) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3070 = {
                    let a3067 = {
                        let a3065 = v1;
                        let a3066 = 64u8;
                        fixed_u8::bit_and(a3065, a3066)
                    };
                    let a3068 = 64u8;
                    if equal(a3067, a3068) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3069, a3070)?
            };
            let a3082 = {
                let a3079 = {
                    let a3073 = {
                        let a3071 = v1;
                        let a3072 = 32u8;
                        fixed_u8::bit_and(a3071, a3072)
                    };
                    let a3074 = 32u8;
                    if equal(a3073, a3074) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3080 = {
                    let a3077 = {
                        let a3075 = v1;
                        let a3076 = 16u8;
                        fixed_u8::bit_and(a3075, a3076)
                    };
                    let a3078 = 16u8;
                    if equal(a3077, a3078) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3079, a3080)?
            };
            nat_add(a3081, a3082)?
        };
        let a3106 = {
            let a3103 = {
                let a3091 = {
                    let a3085 = {
                        let a3083 = v1;
                        let a3084 = 8u8;
                        fixed_u8::bit_and(a3083, a3084)
                    };
                    let a3086 = 8u8;
                    if equal(a3085, a3086) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3092 = {
                    let a3089 = {
                        let a3087 = v1;
                        let a3088 = 4u8;
                        fixed_u8::bit_and(a3087, a3088)
                    };
                    let a3090 = 4u8;
                    if equal(a3089, a3090) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3091, a3092)?
            };
            let a3104 = {
                let a3101 = {
                    let a3095 = {
                        let a3093 = v1;
                        let a3094 = 2u8;
                        fixed_u8::bit_and(a3093, a3094)
                    };
                    let a3096 = 2u8;
                    if equal(a3095, a3096) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3102 = {
                    let a3099 = {
                        let a3097 = v1;
                        let a3098 = 1u8;
                        fixed_u8::bit_and(a3097, a3098)
                    };
                    let a3100 = 1u8;
                    if equal(a3099, a3100) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3101, a3102)?
            };
            nat_add(a3103, a3104)?
        };
        nat_add(a3105, a3106)?
    };
    let a3113 = f124(f476(), {
        let a3107 = v2;
        let a3108 = 16u64;
        let a3109 = 0u64;
        nat_quot(a3107, a3108, a3109)
    });
    let a3114 = f124(f476(), {
        let a3110 = v2;
        let a3111 = 16u64;
        let a3112 = 0u64;
        nat_rem(a3110, a3111, a3112)
    });
    Ok(append_bytes(a3113, a3114))
}

pub fn f469(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f406({
        let (_, h3115) = {
            let (h3116, _) = f477(Fn12::F478, {
                let a3118 = {
                    let a3117 = v0.clone();
                    length_list(a3117)
                };
                let a3119 = 1u64;
                nat_add(a3118, a3119)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h3116
        };
        h3115
    }))
}

pub fn f470(v0: u64) -> bool {
    let a3120 = 48u64;
    let a3121 = v0;
    if nat_le(a3120, a3121) {
        let a3122 = v0;
        let a3123 = 55u64;
        nat_le(a3122, a3123)
    } else {
        false
    }
}

pub fn f471(v0: u64) -> bool {
    if f250(v0) {
        true
    } else {
        if f341(v0) {
            true
        } else {
            let a3124 = v0;
            let a3125 = 95u64;
            if nat_eq(a3124, a3125) {
                true
            } else {
                let a3126 = v0;
                let a3127 = 45u64;
                nat_eq(a3126, a3127)
            }
        }
    }
}

pub fn f472(v0: u64) -> Bytes {
    f32({
        let a3128 = Str::lit("0123456789ABCDEF");
        utf8_encode(a3128)
    }, v0, 1u64)
}

pub fn f473(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3131 = {
        let a3129 = v0;
        let a3130 = v1;
        index_bytes(a3129, a3130)
    };
    match m3131 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f479(v2)? {
                Ok(Some({
                    let a3132 = v1;
                    let a3133 = 1u64;
                    nat_add(a3132, a3133)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f474(v0: u8) -> R<u64> {
    let a3180 = {
        let v1: u8 = v0;
        let a3178 = {
            let a3154 = {
                let a3142 = {
                    let a3136 = {
                        let a3134 = v1;
                        let a3135 = 128u8;
                        fixed_u8::bit_and(a3134, a3135)
                    };
                    let a3137 = 128u8;
                    if equal(a3136, a3137) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3143 = {
                    let a3140 = {
                        let a3138 = v1;
                        let a3139 = 64u8;
                        fixed_u8::bit_and(a3138, a3139)
                    };
                    let a3141 = 64u8;
                    if equal(a3140, a3141) {
                        64u64
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
                        let a3145 = 32u8;
                        fixed_u8::bit_and(a3144, a3145)
                    };
                    let a3147 = 32u8;
                    if equal(a3146, a3147) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3153 = {
                    let a3150 = {
                        let a3148 = v1;
                        let a3149 = 16u8;
                        fixed_u8::bit_and(a3148, a3149)
                    };
                    let a3151 = 16u8;
                    if equal(a3150, a3151) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3152, a3153)?
            };
            nat_add(a3154, a3155)?
        };
        let a3179 = {
            let a3176 = {
                let a3164 = {
                    let a3158 = {
                        let a3156 = v1;
                        let a3157 = 8u8;
                        fixed_u8::bit_and(a3156, a3157)
                    };
                    let a3159 = 8u8;
                    if equal(a3158, a3159) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3165 = {
                    let a3162 = {
                        let a3160 = v1;
                        let a3161 = 4u8;
                        fixed_u8::bit_and(a3160, a3161)
                    };
                    let a3163 = 4u8;
                    if equal(a3162, a3163) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3164, a3165)?
            };
            let a3177 = {
                let a3174 = {
                    let a3168 = {
                        let a3166 = v1;
                        let a3167 = 2u8;
                        fixed_u8::bit_and(a3166, a3167)
                    };
                    let a3169 = 2u8;
                    if equal(a3168, a3169) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3175 = {
                    let a3172 = {
                        let a3170 = v1;
                        let a3171 = 1u8;
                        fixed_u8::bit_and(a3170, a3171)
                    };
                    let a3173 = 1u8;
                    if equal(a3172, a3173) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3174, a3175)?
            };
            nat_add(a3176, a3177)?
        };
        nat_add(a3178, a3179)?
    };
    let a3181 = 48u64;
    Ok(nat_sub(a3180, a3181))
}

pub fn f475(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a3182 = v0.clone();
        length_bytes(a3182)
    };
    let a3183 = v1;
    let a3184 = v2;
    if if nat_lt(a3183, a3184) {
        let a3189 = f137(v0, {
            let a3187 = v2;
            let a3188 = {
                let a3185 = v1;
                let a3186 = 1u64;
                nat_add(a3185, a3186)?
            };
            nat_sub(a3187, a3188)
        })?;
        let a3190 = 48u64;
        nat_eq(a3189, a3190)
    } else {
        false
    } {
        Ok(Some({
            let a3191 = v1;
            let a3192 = 1u64;
            nat_add(a3191, a3192)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f476() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f477(v0: Fn12, p3197: u64, p3198: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m3196 = (p3197, p3198);
    {
        let mut m3196 = m3196;
        loop {
            let step = {
                let (v1, v2) = m3196;
                let m3193 = v1;
                if m3193 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3193 - 1;
                    let m3195 = {
                        let c3194: Fn12 = v0.clone();
                        c3194.apply(v2.clone())?
                    };
                    match m3195 {
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
                Ok(next) => { m3196 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f478(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f480(v0)
}

pub fn f479(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3243 = {
            let a3219 = {
                let a3207 = {
                    let a3201 = {
                        let a3199 = v1;
                        let a3200 = 128u8;
                        fixed_u8::bit_and(a3199, a3200)
                    };
                    let a3202 = 128u8;
                    if equal(a3201, a3202) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3208 = {
                    let a3205 = {
                        let a3203 = v1;
                        let a3204 = 64u8;
                        fixed_u8::bit_and(a3203, a3204)
                    };
                    let a3206 = 64u8;
                    if equal(a3205, a3206) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3207, a3208)?
            };
            let a3220 = {
                let a3217 = {
                    let a3211 = {
                        let a3209 = v1;
                        let a3210 = 32u8;
                        fixed_u8::bit_and(a3209, a3210)
                    };
                    let a3212 = 32u8;
                    if equal(a3211, a3212) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3218 = {
                    let a3215 = {
                        let a3213 = v1;
                        let a3214 = 16u8;
                        fixed_u8::bit_and(a3213, a3214)
                    };
                    let a3216 = 16u8;
                    if equal(a3215, a3216) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3217, a3218)?
            };
            nat_add(a3219, a3220)?
        };
        let a3244 = {
            let a3241 = {
                let a3229 = {
                    let a3223 = {
                        let a3221 = v1;
                        let a3222 = 8u8;
                        fixed_u8::bit_and(a3221, a3222)
                    };
                    let a3224 = 8u8;
                    if equal(a3223, a3224) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3230 = {
                    let a3227 = {
                        let a3225 = v1;
                        let a3226 = 4u8;
                        fixed_u8::bit_and(a3225, a3226)
                    };
                    let a3228 = 4u8;
                    if equal(a3227, a3228) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3229, a3230)?
            };
            let a3242 = {
                let a3239 = {
                    let a3233 = {
                        let a3231 = v1;
                        let a3232 = 2u8;
                        fixed_u8::bit_and(a3231, a3232)
                    };
                    let a3234 = 2u8;
                    if equal(a3233, a3234) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3240 = {
                    let a3237 = {
                        let a3235 = v1;
                        let a3236 = 1u8;
                        fixed_u8::bit_and(a3235, a3236)
                    };
                    let a3238 = 1u8;
                    if equal(a3237, a3238) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3239, a3240)?
            };
            nat_add(a3241, a3242)?
        };
        nat_add(a3243, a3244)?
    };
    let a3245 = 48u64;
    let a3246 = v2;
    if nat_le(a3245, a3246) {
        let a3247 = v2;
        let a3248 = 57u64;
        Ok(nat_le(a3247, a3248))
    } else {
        Ok(false)
    }
}

pub fn f480(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h3249, _) = v0;
        h3249
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h3250) = v0;
        h3250
    };
    let m3251 = v1;
    match m3251.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m3252 = v4;
            match m3252.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f481(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f481(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3257 = {
            let a3255 = {
                let a3253 = v0.clone();
                length_list(a3253)
            };
            let a3256 = {
                let a3254 = v1.clone();
                length_list(a3254)
            };
            nat_add(a3255, a3256)?
        };
        let a3258 = 1u64;
        nat_add(a3257, a3258)?
    };
    Ok(f482({
        let (_, h3259) = {
            let (_, h3260) = {
                let (h3261, _) = f483(Fn13::F484, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3261
            };
            h3260
        };
        h3259
    }))
}

pub fn f482(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f243(Fn5::F485, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f483(v0: Fn13, p3266: u64, p3267: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3265 = (p3266, p3267);
    {
        let mut m3265 = m3265;
        loop {
            let step = {
                let (v1, v2) = m3265;
                let m3262 = v1;
                if m3262 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3262 - 1;
                    let m3264 = {
                        let c3263: Fn13 = v0.clone();
                        c3263.apply(v2.clone())
                    };
                    match m3264 {
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
                Ok(next) => { m3265 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f484(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f486(v0)
}

pub fn f485(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f486(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h3268, _) = v0.clone();
        h3268
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h3269, _) = {
            let (_, h3270) = v0.clone();
            h3270
        };
        h3269
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h3271) = {
            let (_, h3272) = v0;
            h3272
        };
        h3271
    };
    let m3273 = v1.clone();
    match m3273.uncons() {
        None => {
            let m3274 = v2;
            match m3274.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3275 = v2.clone();
            match m3275.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f487({
                        let (h3276, _) = v8.clone();
                        h3276
                    }, {
                        let (h3277, _) = v6.clone();
                        h3277
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

pub fn f487(v0: Bytes, v1: Bytes) -> bool {
    let a3282 = {
        let a3278 = v0;
        let a3279 = v1;
        compare_bytes(a3278, a3279)
    };
    let a3283 = {
        let a3280 = Bytes::lit(&[]);
        let a3281 = Bytes::lit(&[0x01]);
        compare_bytes(a3280, a3281)
    };
    equal(a3282, a3283)
}

pub fn run(p0: Option<Adt0>, p1: Adt2) -> R<(Adt0, Adt9)> {
    f0(p0, p1)
}
