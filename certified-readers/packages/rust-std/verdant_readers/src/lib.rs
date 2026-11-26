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
    C0(List<(Bytes, Option<Bytes>)>, List<(Bytes, List<Bytes>)>, List<Bytes>, List<(Bytes, Option<Adt2>)>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(bool),
    C1(bool, u64),
    C2(bool, u64, u64),
    C3(Bytes),
    C4(std::rc::Rc<Adt2>),
    C5(std::rc::Rc<Adt2>),
    C6,
    C7(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C8(Bytes, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
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
    C0(u64, Bytes, Bytes, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<Bytes>, (u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(Bytes, std::rc::Rc<Adt14>, std::rc::Rc<Adt14>),
    C1(Bytes, Bytes, std::rc::Rc<Adt14>),
    C2(Bytes),
    C3(std::rc::Rc<Adt14>, std::rc::Rc<Adt14>),
    C4,
}

#[derive(Clone)]
pub enum Adt15 {
    C0(Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(Bytes, Adt14, Adt14, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Fn0 {
    F51(u8, Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F51(k0, k1) => f51(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F54(Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn1::F54(k0) => f54(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F81,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn2::F81 => f81(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F152(u64, Bytes),
    F153(Bytes),
    F363(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn3::F152(k0, k1) => Ok(f152(*k0, k1.clone(), p0)),
            Fn3::F153(k0) => f153(k0.clone(), p0),
            Fn3::F363(k0) => f363(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F155(Bytes),
    F180(Bytes),
    F181(Bytes),
    F208(Bytes),
    F209(Bytes),
    F244(Bytes, Bytes),
    F245(Bytes),
    F246(Bytes, u64),
    F261(Bytes),
    F262(Bytes),
    F285(Bytes),
    F327(bool, u64, Bytes),
    F338(Bytes),
    F362(Bytes),
    F368(Bytes),
    F371(u64, Bytes),
    F455(u64, Bytes, u64),
    F458(Bytes),
    F459(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn4::F155(k0) => f155(k0.clone(), p0),
            Fn4::F180(k0) => f180(k0.clone(), p0),
            Fn4::F181(k0) => f181(k0.clone(), p0),
            Fn4::F208(k0) => f208(k0.clone(), p0),
            Fn4::F209(k0) => f209(k0.clone(), p0),
            Fn4::F244(k0, k1) => f244(k0.clone(), k1.clone(), p0),
            Fn4::F245(k0) => f245(k0.clone(), p0),
            Fn4::F246(k0, k1) => f246(k0.clone(), *k1, p0),
            Fn4::F261(k0) => f261(k0.clone(), p0),
            Fn4::F262(k0) => f262(k0.clone(), p0),
            Fn4::F285(k0) => f285(k0.clone(), p0),
            Fn4::F327(k0, k1, k2) => f327(*k0, *k1, k2.clone(), p0),
            Fn4::F338(k0) => f338(k0.clone(), p0),
            Fn4::F362(k0) => f362(k0.clone(), p0),
            Fn4::F368(k0) => f368(k0.clone(), p0),
            Fn4::F371(k0, k1) => f371(*k0, k1.clone(), p0),
            Fn4::F455(k0, k1, k2) => f455(*k0, k1.clone(), *k2, p0),
            Fn4::F458(k0) => f458(k0.clone(), p0),
            Fn4::F459(k0) => f459(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F207(Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
        match self {
            Fn5::F207(k0) => f207(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F212(Bytes),
    F345(Bytes),
    F354(bool, Bytes),
    F402(bool, Bytes),
    F418(Bytes),
    F428(Bytes),
    F454(u64, Bytes),
    F462(u64, Bytes),
}

impl Fn6 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn6::F212(k0) => f212(k0.clone(), p0),
            Fn6::F345(k0) => f345(k0.clone(), p0),
            Fn6::F354(k0, k1) => f354(*k0, k1.clone(), p0),
            Fn6::F402(k0, k1) => f402(*k0, k1.clone(), p0),
            Fn6::F418(k0) => f418(k0.clone(), p0),
            Fn6::F428(k0) => f428(k0.clone(), p0),
            Fn6::F454(k0, k1) => f454(*k0, k1.clone(), p0),
            Fn6::F462(k0, k1) => f462(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F276,
    F500,
}

impl Fn7 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn7::F276 => f276(p0, p1),
            Fn7::F500 => f500(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F279,
}

impl Fn8 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn8::F279 => f279(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F426(u64, Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, bool)) -> R<Option<(u64, bool)>> {
        match self {
            Fn9::F426(k0, k1) => f426(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F434,
}

impl Fn10 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn10::F434 => f434(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F436,
}

impl Fn11 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn11::F436 => f436(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F465,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn12::F465 => f465(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F484(Bytes),
}

impl Fn13 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn13::F484(k0) => f484(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F490,
}

impl Fn14 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn14::F490 => f490(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F499,
}

impl Fn15 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn15::F499 => f499(p0),
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
    Adt1::C0(List::<(Bytes, Option<Bytes>)>::nil(), List::<(Bytes, List<Bytes>)>::nil(), List::<Bytes>::nil(), List::<(Bytes, Option<Adt2>)>::nil())
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
                Ok(List::cons(f33(v4)?, f23(v0, {
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
    Ok(Adt1::C0(List::cons((v1.clone(), v2.clone()), match v0.clone() {
        Adt1::C0(h154, _, _, _) => {
            h154
        }
    }), match v0.clone() {
        Adt1::C0(_, h155, _, _) => {
            h155
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h156, _) => {
            h156
        }
    }, if f34(v1.clone(), f8(Str::lit(".json"))) {
        List::cons((v1, f35({
            let m157 = v2;
            match m157 {
                None => {
                    Bytes::lit(&[])
                }
                Some(v3) => {
                    v3
                }
            }
        })?), match v0 {
            Adt1::C0(_, _, _, h158) => {
                h158
            }
        })
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
    let v1: List<Bytes> = f36(v0.clone(), 47u8)?;
    let v2: Bytes = f37(v1);
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
        Ok(f38(v0.clone(), 0u64, {
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
    f7(List::cons(f8(Str::lit("usage: verdant_studio ")), List::cons(f39(f40(), f8(Str::lit(" | "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))
}

pub fn f32(v0: Bytes, v1: Bytes, v2: List<Bytes>, v3: Adt1) -> R<Adt12> {
    if f41(v0.clone(), f8(Str::lit("read-webidl"))) {
        f42(v3, v1, v2)
    } else {
        if f41(v0.clone(), f8(Str::lit("read-xml"))) {
            f43(v3, v1)
        } else {
            if f41(v0.clone(), f8(Str::lit("read-xsd"))) {
                f44(v3, v1)
            } else {
                if f41(v0, f8(Str::lit("read-yaml"))) {
                    f45(v3, v1)
                } else {
                    Ok(Adt12::C4(f31()))
                }
            }
        }
    }
}

pub fn f33(v0: u8) -> R<Str> {
    let v1: u64 = f46(v0)?;
    let m190 = {
        let a188 = f47({
            let a182 = v1;
            let a183 = 16u64;
            let a184 = 0u64;
            nat_quot(a182, a183, a184)
        });
        let a189 = {
            let a185 = v1;
            let a186 = 16u64;
            let a187 = 0u64;
            nat_rem(a185, a186, a187)
        };
        index_list(a188, a189)
    };
    match m190 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f34(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a191 = v0.clone();
        length_bytes(a191)
    };
    let v3: u64 = {
        let a192 = v1.clone();
        length_bytes(a192)
    };
    let a193 = v3;
    let a194 = v2;
    if nat_le(a193, a194) {
        let m200 = {
            let a197 = v0;
            let a198 = {
                let a195 = v2;
                let a196 = v3;
                nat_sub(a195, a196)
            };
            let a199 = v3;
            slice_bytes(a197, a198, a199)
        };
        match m200 {
            None => {
                false
            }
            Some(v4) => {
                let a201 = v4;
                let a202 = v1;
                equal(a201, a202)
            }
        }
    } else {
        false
    }
}

pub fn f35(v0: Bytes) -> R<Option<Adt2>> {
    let v1: Bytes = f48(v0)?;
    let m206 = f49(v1.clone(), 0u64, {
        let a204 = {
            let a203 = v1.clone();
            length_bytes(a203)
        };
        let a205 = 2u64;
        nat_add(a204, a205)?
    }, 0u64)?;
    match m206 {
        None => {
            Ok(None::<Adt2>)
        }
        Some(v2) => {
            let a209 = {
                let (_, h207) = v2;
                h207
            };
            let a210 = {
                let a208 = v1;
                length_bytes(a208)
            };
            if nat_eq(a209, a210) {
                Ok(Some({
                    let (h211, _) = v2;
                    h211
                }))
            } else {
                Ok(None::<Adt2>)
            }
        }
    }
}

pub fn f36(v0: Bytes, v1: u8) -> R<List<Bytes>> {
    let v2: u64 = {
        let a212 = v0.clone();
        length_bytes(a212)
    };
    let v3: (u64, (u64, List<Bytes>)) = {
        let (h213, _) = f50(Fn0::F51(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h213
    };
    let v4: u64 = {
        let (h214, _) = {
            let (_, h215) = v3.clone();
            h215
        };
        h214
    };
    Ok(f52(List::cons(f38(v0, v4, {
        let a216 = v2;
        let a217 = v4;
        nat_sub(a216, a217)
    }), {
        let (_, h218) = {
            let (_, h219) = v3;
            h219
        };
        h218
    })))
}

pub fn f37(p223: List<Bytes>) -> Bytes {
    let m222 = p223;
    {
        let mut m222 = m222;
        loop {
            let step = {
                let v0 = m222;
                let m220 = v0;
                match m220.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v1, v2)) => {
                        let m221 = v2.clone();
                        match m221.uncons() {
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
                Ok(next) => { m222 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f38(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m227 = {
        let a224 = v0;
        let a225 = v1;
        let a226 = v2;
        slice_bytes(a224, a225, a226)
    };
    match m227 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f39(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m228 = f53(Fn1::F54(v1), None::<Bytes>, v0);
    match m228 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f40() -> List<Bytes> {
    List::cons(f8(Str::lit("admit JSONL")), List::cons(f8(Str::lit("admit-encoding STAGED")), List::cons(f8(Str::lit("agenda")), List::cons(f8(Str::lit("build")), List::cons(f8(Str::lit("check")), List::cons(f8(Str::lit("ingest-authority BINDING")), List::cons(f8(Str::lit("ingest-index CSV")), List::cons(f8(Str::lit("merge-gate RULES")), List::cons(f8(Str::lit("next [SKIP...]")), List::cons(f8(Str::lit("propose-encoding SCHEMA NAME CITATION...")), List::cons(f8(Str::lit("publish")), List::cons(f8(Str::lit("read-webidl FILE [IMPORT...]")), List::cons(f8(Str::lit("read-xml FILE")), List::cons(f8(Str::lit("read-xsd FILE")), List::cons(f8(Str::lit("read-yaml FILE")), List::cons(f8(Str::lit("record-decision JSONL")), List::cons(f8(Str::lit("stage-admissions JSONL")), List::cons(f8(Str::lit("stage-bindings JSONL")), List::cons(f8(Str::lit("stage-decisions JSONL")), List::cons(f8(Str::lit("stage-encodings JSONL")), List::cons(f8(Str::lit("stage-engine")), List::cons(f8(Str::lit("stage-mappings JSONL")), List::cons(f8(Str::lit("stage-matrix JSONL")), List::<Bytes>::nil())))))))))))))))))))))))
}

pub fn f41(v0: Bytes, v1: Bytes) -> bool {
    let a229 = v0;
    let a230 = v1;
    equal(a229, a230)
}

pub fn f42(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<Adt12> {
    let m231 = f55(v0.clone(), List::cons(v1.clone(), v2.clone()));
    match m231.uncons() {
        None => {
            let m232 = f56(f57(v0.clone(), v1), f58(v0, v2))?;
            match m232 {
                Ok(v6) => {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("-- ")), List::cons(f59({
                        let (h233, _) = v6;
                        h233
                    })?, List::cons(f8(Str::lit(" Web IDL definitions translated\u{a}")), List::cons({
                        let (_, h234) = v6;
                        h234
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

pub fn f43(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m235 = f55(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m235.uncons() {
        None => {
            let m236 = f60(f57(v0, v1))?;
            match m236 {
                Ok(v5) => {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("well-formed XML: root ")), List::cons(f61(v5.clone()), List::cons(f8(Str::lit(", ")), List::cons(f59(f62(v5)?)?, List::cons(f8(Str::lit(" elements\u{a}")), List::<Bytes>::nil()))))))))
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

pub fn f44(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m237 = f55(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m237.uncons() {
        None => {
            let m238 = f63(f57(v0, v1))?;
            match m238 {
                Ok(v5) => {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("-- ")), List::cons(f59({
                        let (h239, _) = v5;
                        h239
                    })?, List::cons(f8(Str::lit(" simple types\u{a}")), List::cons({
                        let (_, h240) = v5;
                        h240
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

pub fn f45(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m241 = f55(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m241.uncons() {
        None => {
            let m242 = f64(f57(v0, v1))?;
            match m242 {
                Ok(v5) => {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), {
                        let a243 = f65(v5)?;
                        let a244 = f8(Str::lit("\u{a}"));
                        append_bytes(a243, a244)
                    }))
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

pub fn f46(v0: u8) -> R<u64> {
    let v1: u8 = v0;
    let a289 = {
        let a265 = {
            let a253 = {
                let a247 = {
                    let a245 = v1;
                    let a246 = 128u8;
                    fixed_u8::bit_and(a245, a246)
                };
                let a248 = 128u8;
                if equal(a247, a248) {
                    128u64
                } else {
                    0u64
                }
            };
            let a254 = {
                let a251 = {
                    let a249 = v1;
                    let a250 = 64u8;
                    fixed_u8::bit_and(a249, a250)
                };
                let a252 = 64u8;
                if equal(a251, a252) {
                    64u64
                } else {
                    0u64
                }
            };
            nat_add(a253, a254)?
        };
        let a266 = {
            let a263 = {
                let a257 = {
                    let a255 = v1;
                    let a256 = 32u8;
                    fixed_u8::bit_and(a255, a256)
                };
                let a258 = 32u8;
                if equal(a257, a258) {
                    32u64
                } else {
                    0u64
                }
            };
            let a264 = {
                let a261 = {
                    let a259 = v1;
                    let a260 = 16u8;
                    fixed_u8::bit_and(a259, a260)
                };
                let a262 = 16u8;
                if equal(a261, a262) {
                    16u64
                } else {
                    0u64
                }
            };
            nat_add(a263, a264)?
        };
        nat_add(a265, a266)?
    };
    let a290 = {
        let a287 = {
            let a275 = {
                let a269 = {
                    let a267 = v1;
                    let a268 = 8u8;
                    fixed_u8::bit_and(a267, a268)
                };
                let a270 = 8u8;
                if equal(a269, a270) {
                    8u64
                } else {
                    0u64
                }
            };
            let a276 = {
                let a273 = {
                    let a271 = v1;
                    let a272 = 4u8;
                    fixed_u8::bit_and(a271, a272)
                };
                let a274 = 4u8;
                if equal(a273, a274) {
                    4u64
                } else {
                    0u64
                }
            };
            nat_add(a275, a276)?
        };
        let a288 = {
            let a285 = {
                let a279 = {
                    let a277 = v1;
                    let a278 = 2u8;
                    fixed_u8::bit_and(a277, a278)
                };
                let a280 = 2u8;
                if equal(a279, a280) {
                    2u64
                } else {
                    0u64
                }
            };
            let a286 = {
                let a283 = {
                    let a281 = v1;
                    let a282 = 1u8;
                    fixed_u8::bit_and(a281, a282)
                };
                let a284 = 1u8;
                if equal(a283, a284) {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a285, a286)?
        };
        nat_add(a287, a288)?
    };
    nat_add(a289, a290)
}

pub fn f47(v0: u64) -> List<Str> {
    let a291 = v0;
    let a292 = 0u64;
    if nat_eq(a291, a292) {
        f66()
    } else {
        let a293 = v0;
        let a294 = 1u64;
        if nat_eq(a293, a294) {
            f67()
        } else {
            let a295 = v0;
            let a296 = 2u64;
            if nat_eq(a295, a296) {
                f68()
            } else {
                let a297 = v0;
                let a298 = 3u64;
                if nat_eq(a297, a298) {
                    f69()
                } else {
                    let a299 = v0;
                    let a300 = 4u64;
                    if nat_eq(a299, a300) {
                        f70()
                    } else {
                        let a301 = v0;
                        let a302 = 5u64;
                        if nat_eq(a301, a302) {
                            f71()
                        } else {
                            let a303 = v0;
                            let a304 = 6u64;
                            if nat_eq(a303, a304) {
                                f72()
                            } else {
                                let a305 = v0;
                                let a306 = 7u64;
                                if nat_eq(a305, a306) {
                                    f73()
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

pub fn f48(v0: Bytes) -> R<Bytes> {
    f74(v0.clone(), 0u64, {
        let a307 = v0;
        length_bytes(a307)
    }, false, false, 0u64, Bytes::lit(&[]))
}

pub fn f49(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt2, u64)>> {
    let m308 = v2;
    if m308 == 0 {
        Ok(None::<(Adt2, u64)>)
    } else {
        let v4 = m308 - 1;
        let v5: u64 = f75(v0.clone(), v1)?;
        let a309 = v3;
        let a310 = 1u64;
        if nat_eq(a309, a310) {
            let m311 = f49(v0.clone(), v1, v4, 0u64)?;
            match m311 {
                None => {
                    Ok(None::<(Adt2, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f75(v0.clone(), {
                        let (_, h312) = v6.clone();
                        h312
                    })?;
                    let a313 = v7;
                    let a314 = 44u64;
                    if nat_eq(a313, a314) {
                        let m318 = f49(v0, {
                            let a316 = {
                                let (_, h315) = v6;
                                h315
                            };
                            let a317 = 1u64;
                            nat_add(a316, a317)?
                        }, v4, 1u64)?;
                        match m318 {
                            None => {
                                Ok(None::<(Adt2, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt2::C7(std::rc::Rc::new({
                                    let (h319, _) = v6;
                                    h319
                                }), std::rc::Rc::new({
                                    let (h320, _) = v8;
                                    h320
                                })), {
                                    let (_, h321) = v8;
                                    h321
                                })))
                            }
                        }
                    } else {
                        let a322 = v7;
                        let a323 = 93u64;
                        if nat_eq(a322, a323) {
                            Ok(Some((Adt2::C7(std::rc::Rc::new({
                                let (h324, _) = v6;
                                h324
                            }), std::rc::Rc::new(Adt2::C6)), {
                                let a326 = {
                                    let (_, h325) = v6;
                                    h325
                                };
                                let a327 = 1u64;
                                nat_add(a326, a327)?
                            })))
                        } else {
                            Ok(None::<(Adt2, u64)>)
                        }
                    }
                }
            }
        } else {
            let a328 = v3;
            let a329 = 2u64;
            if nat_eq(a328, a329) {
                let m332 = f76(v0.clone(), {
                    let a330 = v1;
                    let a331 = 1u64;
                    nat_add(a330, a331)?
                })?;
                match m332 {
                    None => {
                        Ok(None::<(Adt2, u64)>)
                    }
                    Some(v9) => {
                        if f77({
                            let a333 = v5;
                            let a334 = 34u64;
                            nat_eq(a333, a334)
                        }, {
                            let a336 = f75(v0.clone(), {
                                let (h335, _) = v9.clone();
                                h335
                            })?;
                            let a337 = 58u64;
                            nat_eq(a336, a337)
                        }) {
                            let m341 = f49(v0.clone(), {
                                let a339 = {
                                    let (h338, _) = v9;
                                    h338
                                };
                                let a340 = 1u64;
                                nat_add(a339, a340)?
                            }, v4, 0u64)?;
                            match m341 {
                                None => {
                                    Ok(None::<(Adt2, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f75(v0.clone(), {
                                        let (_, h342) = v10.clone();
                                        h342
                                    })?;
                                    let a343 = v11;
                                    let a344 = 44u64;
                                    if nat_eq(a343, a344) {
                                        let m348 = f49(v0, {
                                            let a346 = {
                                                let (_, h345) = v10;
                                                h345
                                            };
                                            let a347 = 1u64;
                                            nat_add(a346, a347)?
                                        }, v4, 2u64)?;
                                        match m348 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt2::C8({
                                                    let (_, h349) = v9;
                                                    h349
                                                }, std::rc::Rc::new({
                                                    let (h350, _) = v10;
                                                    h350
                                                }), std::rc::Rc::new({
                                                    let (h351, _) = v12;
                                                    h351
                                                })), {
                                                    let (_, h352) = v12;
                                                    h352
                                                })))
                                            }
                                        }
                                    } else {
                                        let a353 = v11;
                                        let a354 = 125u64;
                                        if nat_eq(a353, a354) {
                                            Ok(Some((Adt2::C8({
                                                let (_, h355) = v9;
                                                h355
                                            }, std::rc::Rc::new({
                                                let (h356, _) = v10;
                                                h356
                                            }), std::rc::Rc::new(Adt2::C6)), {
                                                let a358 = {
                                                    let (_, h357) = v10;
                                                    h357
                                                };
                                                let a359 = 1u64;
                                                nat_add(a358, a359)?
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
                let a360 = v5;
                let a361 = 34u64;
                if nat_eq(a360, a361) {
                    let m364 = f76(v0, {
                        let a362 = v1;
                        let a363 = 1u64;
                        nat_add(a362, a363)?
                    })?;
                    match m364 {
                        None => {
                            Ok(None::<(Adt2, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt2::C3({
                                let (_, h365) = v13;
                                h365
                            }), {
                                let (h366, _) = v13;
                                h366
                            })))
                        }
                    }
                } else {
                    if f41(f38(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                        Ok(Some((Adt2::C0(true), {
                            let a367 = v1;
                            let a368 = 4u64;
                            nat_add(a367, a368)?
                        })))
                    } else {
                        if f41(f38(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                            Ok(Some((Adt2::C0(false), {
                                let a369 = v1;
                                let a370 = 5u64;
                                nat_add(a369, a370)?
                            })))
                        } else {
                            let a371 = v5;
                            let a372 = 91u64;
                            if nat_eq(a371, a372) {
                                let a375 = f75(v0.clone(), {
                                    let a373 = v1;
                                    let a374 = 1u64;
                                    nat_add(a373, a374)?
                                })?;
                                let a376 = 93u64;
                                if nat_eq(a375, a376) {
                                    Ok(Some((Adt2::C4(std::rc::Rc::new(Adt2::C6)), {
                                        let a377 = v1;
                                        let a378 = 2u64;
                                        nat_add(a377, a378)?
                                    })))
                                } else {
                                    let m381 = f49(v0, {
                                        let a379 = v1;
                                        let a380 = 1u64;
                                        nat_add(a379, a380)?
                                    }, v4, 1u64)?;
                                    match m381 {
                                        None => {
                                            Ok(None::<(Adt2, u64)>)
                                        }
                                        Some(v14) => {
                                            Ok(Some((Adt2::C4(std::rc::Rc::new({
                                                let (h382, _) = v14;
                                                h382
                                            })), {
                                                let (_, h383) = v14;
                                                h383
                                            })))
                                        }
                                    }
                                }
                            } else {
                                let a384 = v5;
                                let a385 = 123u64;
                                if nat_eq(a384, a385) {
                                    let a388 = f75(v0.clone(), {
                                        let a386 = v1;
                                        let a387 = 1u64;
                                        nat_add(a386, a387)?
                                    })?;
                                    let a389 = 125u64;
                                    if nat_eq(a388, a389) {
                                        Ok(Some((Adt2::C5(std::rc::Rc::new(Adt2::C6)), {
                                            let a390 = v1;
                                            let a391 = 2u64;
                                            nat_add(a390, a391)?
                                        })))
                                    } else {
                                        let m394 = f49(v0, {
                                            let a392 = v1;
                                            let a393 = 1u64;
                                            nat_add(a392, a393)?
                                        }, v4, 2u64)?;
                                        match m394 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v15) => {
                                                Ok(Some((Adt2::C5(std::rc::Rc::new({
                                                    let (h395, _) = v15;
                                                    h395
                                                })), {
                                                    let (_, h396) = v15;
                                                    h396
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let m397 = f78(v0, v1)?;
                                    match m397 {
                                        None => {
                                            Ok(None::<(Adt2, u64)>)
                                        }
                                        Some(v16) => {
                                            Ok(Some(({
                                                let (_, h398) = v16;
                                                h398
                                            }, {
                                                let (h399, _) = v16;
                                                h399
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

pub fn f50(v0: Fn0, p404: u64, p405: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m403 = (p404, p405);
    {
        let mut m403 = m403;
        loop {
            let step = {
                let (v1, v2) = m403;
                let m400 = v1;
                if m400 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m400 - 1;
                    let m402 = {
                        let c401: Fn0 = v0.clone();
                        c401.apply(v2.clone())?
                    };
                    match m402 {
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
                Ok(next) => { m403 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f51(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f79(v1, v0, v2)
}

pub fn f52(v0: List<Bytes>) -> List<Bytes> {
    f80(Fn2::F81, List::<Bytes>::nil(), v0)
}

pub fn f53(v0: Fn1, p409: Option<Bytes>, p410: List<Bytes>) -> Option<Bytes> {
    let m408 = (p409, p410);
    {
        let mut m408 = m408;
        loop {
            let step = {
                let (v1, v2) = m408;
                let m406 = v2;
                match m406.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c407: Fn1 = v0.clone();
                            c407.apply(v1, v3)
                        }, v4))
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

pub fn f54(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m411 = v1;
    match m411 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a414 = {
                    let a412 = v3;
                    let a413 = v0;
                    append_bytes(a412, a413)
                };
                let a415 = v2;
                append_bytes(a414, a415)
            })
        }
    }
}

pub fn f55(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m416 = v1;
    match m416.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m418 = f82(match v0.clone() {
                Adt1::C0(h417, _, _, _) => {
                    h417
                }
            }, v2.clone());
            match m418 {
                None => {
                    List::cons(v2, f55(v0, v3))
                }
                Some(_) => {
                    f55(v0, v3)
                }
            }
        }
    }
}

pub fn f56(v0: Bytes, v1: List<Bytes>) -> R<Result<(u64, Bytes), Bytes>> {
    let m419 = f83(v0)?;
    match m419 {
        Ok(v3) => {
            let m420 = f84(v1)?;
            match m420 {
                Ok(v5) => {
                    let v6: List<Adt13> = f85(v3.clone(), v5);
                    let v7: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))> = f86({
                        let a422 = {
                            let a421 = v6.clone();
                            length_list(a421)
                        };
                        let a423 = 1u64;
                        nat_add(a422, a423)?
                    }, f87(v6.clone(), v6.clone())?)?;
                    let v8: List<Bytes> = f88({
                        let a425 = {
                            let a424 = v7.clone();
                            length_list(a424)
                        };
                        let a426 = 1u64;
                        nat_add(a425, a426)?
                    }, f89(v3.clone()), List::<Bytes>::nil(), v7.clone());
                    let v9: List<(Bytes, (List<Bytes>, Bytes))> = f90(v7.clone(), v8);
                    let v10: (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>) = f91({
                        let a428 = {
                            let a427 = v9.clone();
                            length_list(a427)
                        };
                        let a429 = 1u64;
                        nat_add(a428, a429)?
                    }, v9, List::<Bytes>::nil(), List::<Bytes>::nil());
                    Ok(Ok::<(u64, Bytes), Bytes>(({
                        let a431 = {
                            let (h430, _) = v10.clone();
                            h430
                        };
                        length_list(a431)
                    }, f7(List::cons(f7(f92(v3, v7, v6)), List::cons(f7(f93({
                        let (_, h432) = v10;
                        h432
                    })), List::cons(f8(Str::lit("-- operations, constants and the other members that are not attributes are behavior, not data, and are not translated (Web IDL 2.5)\u{a}")), List::cons(f94(f95({
                        let (h433, _) = v10;
                        h433
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

pub fn f57(v0: Adt1, v1: Bytes) -> Bytes {
    let m435 = f82(match v0 {
        Adt1::C0(h434, _, _, _) => {
            h434
        }
    }, v1);
    match m435 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m436 = v2;
            match m436 {
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

pub fn f58(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m437 = v1;
    match m437.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f57(v0.clone(), v2), f58(v0, v3))
        }
    }
}

pub fn f59(v0: u64) -> R<Bytes> {
    f96(f97(), 10u64, v0)
}

pub fn f60(v0: Bytes) -> R<Result<Adt14, Bytes>> {
    let v1: Bytes = if f98(v0.clone(), f8(Str::lit("\u{feff}")), 0u64) {
        f38(v0.clone(), 3u64, {
            let a439 = {
                let a438 = v0;
                length_bytes(a438)
            };
            let a440 = 3u64;
            nat_sub(a439, a440)
        })
    } else {
        v0
    };
    let v2: Bytes = f99(v1)?;
    let v3: u64 = {
        let a441 = v2.clone();
        length_bytes(a441)
    };
    let v4: u64 = f100(v2.clone())?;
    let a442 = v4;
    let a443 = v3;
    if nat_lt(a442, a443) {
        Ok(f101(f102(Str::lit("the input is not a sequence of Chars in UTF-8 (XML 1.0 [2], section 4.3.3)"), v4)?))
    } else {
        let m444 = f103(v2.clone())?;
        match m444 {
            Ok(v6) => {
                let m447 = f104(v2.clone(), v6, {
                    let a445 = v3;
                    let a446 = 2u64;
                    nat_add(a445, a446)?
                })?;
                match m447 {
                    Ok(v8) => {
                        if f105(v2.clone(), v8, Str::lit("<!DOCTYPE")) {
                            Ok(f101(f102(Str::lit("a document type declaration is not read; Verdant Studio reads documents that have none (XML 1.0 [28])"), v8)?))
                        } else {
                            let a448 = f106(v2.clone(), v8)?;
                            let a449 = 60u64;
                            if nat_eq(a448, a449) {
                                let m452 = f107(v2.clone(), v8, {
                                    let a450 = v3;
                                    let a451 = 2u64;
                                    nat_add(a450, a451)?
                                }, 0u64)?;
                                match m452 {
                                    Ok(v10) => {
                                        let m456 = f104(v2, {
                                            let (_, h453) = v10;
                                            h453
                                        }, {
                                            let a454 = v3;
                                            let a455 = 2u64;
                                            nat_add(a454, a455)?
                                        })?;
                                        match m456 {
                                            Ok(v12) => {
                                                let a457 = v12;
                                                let a458 = v3;
                                                if nat_eq(a457, a458) {
                                                    Ok(Ok::<Adt14, Bytes>({
                                                        let (h459, _) = v10;
                                                        h459
                                                    }))
                                                } else {
                                                    Ok(f101(f102(Str::lit("only comments, processing instructions and white space may follow the root element (XML 1.0 [1], [27])"), v12)?))
                                                }
                                            }
                                            Err(v11) => {
                                                Ok(f101(v11))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Ok(f101(v9))
                                    }
                                }
                            } else {
                                Ok(f101(f102(Str::lit("a document must have exactly one root element (XML 1.0 [1])"), v8)?))
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(f101(v7))
                    }
                }
            }
            Err(v5) => {
                Ok(f101(v5))
            }
        }
    }
}

pub fn f61(v0: Adt14) -> Bytes {
    let m460 = v0;
    match m460 {
        Adt14::C0(v1, _, _) => {
            v1
        }
        Adt14::C1(_, _, _) => {
            Bytes::lit(&[])
        }
        Adt14::C2(_) => {
            Bytes::lit(&[])
        }
        Adt14::C3(_, _) => {
            Bytes::lit(&[])
        }
        Adt14::C4 => {
            Bytes::lit(&[])
        }
    }
}

pub fn f62(v0: Adt14) -> R<u64> {
    let m461 = v0;
    match m461 {
        Adt14::C0(_, _, r466) => {
            let v3 = (*r466).clone();
            let a462 = 1u64;
            let a463 = f62(v3)?;
            nat_add(a462, a463)
        }
        Adt14::C1(_, _, _) => {
            Ok(0u64)
        }
        Adt14::C2(_) => {
            Ok(0u64)
        }
        Adt14::C3(r467, r468) => {
            let v8 = (*r467).clone();
            let v9 = (*r468).clone();
            let a464 = f62(v8)?;
            let a465 = f62(v9)?;
            nat_add(a464, a465)
        }
        Adt14::C4 => {
            Ok(0u64)
        }
    }
}

pub fn f63(v0: Bytes) -> R<Result<(u64, Bytes), Bytes>> {
    let m469 = f60(v0)?;
    match m469 {
        Ok(v2) => {
            let m470 = v2;
            match m470 {
                Adt14::C0(v3, r482, r483) => {
                    let v4 = (*r482).clone();
                    let v5 = (*r483).clone();
                    let m471 = f109(v4.clone(), List::<(Bytes, Bytes)>::nil());
                    match m471 {
                        Ok(v7) => {
                            let m472 = f110(v7.clone(), v3)?;
                            match m472 {
                                Ok(v9) => {
                                    if f111(v9, Str::lit("schema")) {
                                        let v11: Bytes = {
                                            let m473 = f112(v4, f8(Str::lit("targetNamespace")));
                                            match m473 {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some(v10) => {
                                                    v10
                                                }
                                            }
                                        };
                                        let m474 = f113(v5, v7, v11)?;
                                        match m474 {
                                            Ok(v13) => {
                                                let v14: List<Bytes> = f114(v13.clone());
                                                let m475 = f115(v14.clone());
                                                match m475 {
                                                    None => {
                                                        let m476 = f116(v13.clone(), v14.clone());
                                                        match m476 {
                                                            None => {
                                                                let m480 = f117({
                                                                    let a478 = {
                                                                        let a477 = v14.clone();
                                                                        length_list(a477)
                                                                    };
                                                                    let a479 = 1u64;
                                                                    nat_add(a478, a479)?
                                                                }, v13, List::<Bytes>::nil(), List::<Bytes>::nil());
                                                                match m480 {
                                                                    Ok(v18) => {
                                                                        Ok(Ok::<(u64, Bytes), Bytes>(({
                                                                            let a481 = v14;
                                                                            length_list(a481)
                                                                        }, f94(f95(v18, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))))))
                                                                    }
                                                                    Err(v17) => {
                                                                        Ok(f108(v17))
                                                                    }
                                                                }
                                                            }
                                                            Some(v16) => {
                                                                Ok(f108(f7(List::cons(f8(Str::lit("the type ")), List::cons(v16, List::cons(f8(Str::lit(" is not a simple type of this schema document")), List::<Bytes>::nil()))))))
                                                            }
                                                        }
                                                    }
                                                    Some(v15) => {
                                                        Ok(f108(f7(List::cons(f8(Str::lit("two simple types derive the type name ")), List::cons(v15, List::<Bytes>::nil())))))
                                                    }
                                                }
                                            }
                                            Err(v12) => {
                                                Ok(f108(v12))
                                            }
                                        }
                                    } else {
                                        Ok(f108(f8(Str::lit("the root element must be xs:schema in the XML Schema namespace (XSD 1.1 Part 1, section 3.17.2)"))))
                                    }
                                }
                                Err(v8) => {
                                    Ok(f108(v8))
                                }
                            }
                        }
                        Err(v6) => {
                            Ok(f108(v6))
                        }
                    }
                }
                Adt14::C1(_, _, _) => {
                    Ok(f108(f8(Str::lit("the document has no root element"))))
                }
                Adt14::C2(_) => {
                    Ok(f108(f8(Str::lit("the document has no root element"))))
                }
                Adt14::C3(_, _) => {
                    Ok(f108(f8(Str::lit("the document has no root element"))))
                }
                Adt14::C4 => {
                    Ok(f108(f8(Str::lit("the document has no root element"))))
                }
            }
        }
        Err(v1) => {
            Ok(f108(f7(List::cons(f8(Str::lit("not well-formed XML: ")), List::cons(v1, List::<Bytes>::nil())))))
        }
    }
}

pub fn f64(v0: Bytes) -> R<Result<Adt2, Bytes>> {
    let v1: Bytes = if f98(v0.clone(), Bytes::lit(&[0xef, 0xbb, 0xbf]), 0u64) {
        f38(v0.clone(), 3u64, {
            let a485 = {
                let a484 = v0;
                length_bytes(a484)
            };
            let a486 = 3u64;
            nat_sub(a485, a486)
        })
    } else {
        v0
    };
    let v2: Bytes = f99(v1)?;
    let v3: u64 = {
        let a487 = v2.clone();
        length_bytes(a487)
    };
    let v4: u64 = f118(v2.clone())?;
    let v5: u64 = {
        let a488 = v3;
        let a489 = 2u64;
        nat_add(a488, a489)?
    };
    let v6: u64 = f119(v2.clone(), 0u64)?;
    let a490 = v4;
    let a491 = v3;
    if nat_lt(a490, a491) {
        Ok(Err::<Adt2, Bytes>(f102(Str::lit("the input is not a sequence of printable characters in UTF-8 (YAML 1.2.2 [1], section 5.2)"), v4)?))
    } else {
        let a492 = v6;
        let a493 = v3;
        if if if nat_lt(a492, a493) {
            f105(v2.clone(), v6, Str::lit("..."))
        } else {
            false
        } {
            f120(v2.clone(), v6)?
        } else {
            false
        } {
            Ok(Err::<Adt2, Bytes>(f102(Str::lit("a stream with no document is not read (YAML 1.2.2 [211])"), v6)?))
        } else {
            let a494 = f121(v2.clone(), v6)?;
            let a495 = 37u64;
            if nat_eq(a494, a495) {
                Ok(Err::<Adt2, Bytes>(f102(Str::lit("a directive is not read (YAML 1.2.2 [82])"), v6)?))
            } else {
                let a496 = v3;
                let a497 = v6;
                if nat_le(a496, a497) {
                    Ok(Err::<Adt2, Bytes>(f102(Str::lit("a stream with no document is not read (YAML 1.2.2 [211])"), v6)?))
                } else {
                    let m500 = if if f105(v2.clone(), v6, Str::lit("---")) {
                        f120(v2.clone(), v6)?
                    } else {
                        false
                    } {
                        f122(v2.clone(), {
                            let a498 = v6;
                            let a499 = 3u64;
                            nat_add(a498, a499)?
                        }, 0u64, v5, 1u64, false, 0u64, List::<Bytes>::nil())?
                    } else {
                        f122(v2.clone(), v6, 0u64, v5, 0u64, false, 0u64, List::<Bytes>::nil())?
                    };
                    match m500 {
                        Ok(v8) => {
                            let m502 = f123(v2, {
                                let (_, h501) = v8;
                                h501
                            })?;
                            match m502 {
                                Ok(_) => {
                                    Ok(Ok::<Adt2, Bytes>({
                                        let (h503, _) = v8;
                                        h503
                                    }))
                                }
                                Err(v9) => {
                                    Ok(Err::<Adt2, Bytes>(v9))
                                }
                            }
                        }
                        Err(v7) => {
                            Ok(Err::<Adt2, Bytes>(v7))
                        }
                    }
                }
            }
        }
    }
}

pub fn f65(v0: Adt2) -> R<Bytes> {
    f124(v0)
}

pub fn f66() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{9}"), List::cons(Str::lit("\u{a}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{d}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f67() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f68() -> List<Str> {
    List::cons(Str::lit(" "), List::cons(Str::lit("!"), List::cons(Str::lit("\""), List::cons(Str::lit("#"), List::cons(Str::lit("$"), List::cons(Str::lit("%"), List::cons(Str::lit("&"), List::cons(Str::lit("'"), List::cons(Str::lit("("), List::cons(Str::lit(")"), List::cons(Str::lit("*"), List::cons(Str::lit("+"), List::cons(Str::lit(","), List::cons(Str::lit("-"), List::cons(Str::lit("."), List::cons(Str::lit("/"), List::<Str>::nil()))))))))))))))))
}

pub fn f69() -> List<Str> {
    List::cons(Str::lit("0"), List::cons(Str::lit("1"), List::cons(Str::lit("2"), List::cons(Str::lit("3"), List::cons(Str::lit("4"), List::cons(Str::lit("5"), List::cons(Str::lit("6"), List::cons(Str::lit("7"), List::cons(Str::lit("8"), List::cons(Str::lit("9"), List::cons(Str::lit(":"), List::cons(Str::lit(";"), List::cons(Str::lit("<"), List::cons(Str::lit("="), List::cons(Str::lit(">"), List::cons(Str::lit("?"), List::<Str>::nil()))))))))))))))))
}

pub fn f70() -> List<Str> {
    List::cons(Str::lit("@"), List::cons(Str::lit("A"), List::cons(Str::lit("B"), List::cons(Str::lit("C"), List::cons(Str::lit("D"), List::cons(Str::lit("E"), List::cons(Str::lit("F"), List::cons(Str::lit("G"), List::cons(Str::lit("H"), List::cons(Str::lit("I"), List::cons(Str::lit("J"), List::cons(Str::lit("K"), List::cons(Str::lit("L"), List::cons(Str::lit("M"), List::cons(Str::lit("N"), List::cons(Str::lit("O"), List::<Str>::nil()))))))))))))))))
}

pub fn f71() -> List<Str> {
    List::cons(Str::lit("P"), List::cons(Str::lit("Q"), List::cons(Str::lit("R"), List::cons(Str::lit("S"), List::cons(Str::lit("T"), List::cons(Str::lit("U"), List::cons(Str::lit("V"), List::cons(Str::lit("W"), List::cons(Str::lit("X"), List::cons(Str::lit("Y"), List::cons(Str::lit("Z"), List::cons(Str::lit("["), List::cons(Str::lit("\\"), List::cons(Str::lit("]"), List::cons(Str::lit("^"), List::cons(Str::lit("_"), List::<Str>::nil()))))))))))))))))
}

pub fn f72() -> List<Str> {
    List::cons(Str::lit("`"), List::cons(Str::lit("a"), List::cons(Str::lit("b"), List::cons(Str::lit("c"), List::cons(Str::lit("d"), List::cons(Str::lit("e"), List::cons(Str::lit("f"), List::cons(Str::lit("g"), List::cons(Str::lit("h"), List::cons(Str::lit("i"), List::cons(Str::lit("j"), List::cons(Str::lit("k"), List::cons(Str::lit("l"), List::cons(Str::lit("m"), List::cons(Str::lit("n"), List::cons(Str::lit("o"), List::<Str>::nil()))))))))))))))))
}

pub fn f73() -> List<Str> {
    List::cons(Str::lit("p"), List::cons(Str::lit("q"), List::cons(Str::lit("r"), List::cons(Str::lit("s"), List::cons(Str::lit("t"), List::cons(Str::lit("u"), List::cons(Str::lit("v"), List::cons(Str::lit("w"), List::cons(Str::lit("x"), List::cons(Str::lit("y"), List::cons(Str::lit("z"), List::cons(Str::lit("{"), List::cons(Str::lit("|"), List::cons(Str::lit("}"), List::cons(Str::lit("~"), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f74(v0: Bytes, p539: u64, p540: u64, p541: bool, p542: bool, p543: u64, p544: Bytes) -> R<Bytes> {
    let m538 = (p539, (p540, (p541, (p542, (p543, p544)))));
    {
        let mut m538 = m538;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, (v5, v6))))) = m538;
                let m504 = v2;
                if m504 == 0 {
                    Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f125(v0.clone(), v5, v1, v6)))
                } else {
                    let v7 = m504 - 1;
                    let m507 = {
                        let a505 = v0.clone();
                        let a506 = v1;
                        index_bytes(a505, a506)
                    };
                    match m507 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f125(v0.clone(), v5, v1, v6)))
                        }
                        Some(v8) => {
                            let v9: u64 = f46(v8)?;
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                    let a508 = v1;
                                    let a509 = 1u64;
                                    nat_add(a508, a509)?
                                }, (v7, (if v4 {
                                    true
                                } else {
                                    let a510 = v9;
                                    let a511 = 34u64;
                                    !nat_eq(a510, a511)
                                }, (if v4 {
                                    false
                                } else {
                                    let a512 = v9;
                                    let a513 = 92u64;
                                    nat_eq(a512, a513)
                                }, (v5, v6))))))
                            } else {
                                let a514 = v9;
                                let a515 = 32u64;
                                if if if if nat_eq(a514, a515) {
                                    true
                                } else {
                                    let a516 = v9;
                                    let a517 = 9u64;
                                    nat_eq(a516, a517)
                                } {
                                    true
                                } else {
                                    let a518 = v9;
                                    let a519 = 10u64;
                                    nat_eq(a518, a519)
                                } {
                                    true
                                } else {
                                    let a520 = v9;
                                    let a521 = 13u64;
                                    nat_eq(a520, a521)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a522 = v1;
                                        let a523 = 1u64;
                                        nat_add(a522, a523)?
                                    }, (v7, (false, (false, ({
                                        let a524 = v1;
                                        let a525 = 1u64;
                                        nat_add(a524, a525)?
                                    }, f125(v0.clone(), v5, v1, v6)))))))
                                } else {
                                    let a526 = v9;
                                    let a527 = 110u64;
                                    if if nat_eq(a526, a527) {
                                        f41(f38(v0.clone(), v1, 4u64), f8(Str::lit("null")))
                                    } else {
                                        false
                                    } {
                                        Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                            let a528 = v1;
                                            let a529 = 4u64;
                                            nat_add(a528, a529)?
                                        }, (v7, (false, (false, ({
                                            let a530 = v1;
                                            let a531 = 4u64;
                                            nat_add(a530, a531)?
                                        }, {
                                            let a532 = f125(v0.clone(), v5, v1, v6);
                                            let a533 = f8(Str::lit("[]"));
                                            append_bytes(a532, a533)
                                        }))))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                            let a534 = v1;
                                            let a535 = 1u64;
                                            nat_add(a534, a535)?
                                        }, (v7, ({
                                            let a536 = v9;
                                            let a537 = 34u64;
                                            nat_eq(a536, a537)
                                        }, (false, (v5, v6))))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m538 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f75(v0: Bytes, v1: u64) -> R<u64> {
    let m547 = {
        let a545 = v0;
        let a546 = v1;
        index_bytes(a545, a546)
    };
    match m547 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a592 = {
                let a568 = {
                    let a556 = {
                        let a550 = {
                            let a548 = v3;
                            let a549 = 128u8;
                            fixed_u8::bit_and(a548, a549)
                        };
                        let a551 = 128u8;
                        if equal(a550, a551) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a557 = {
                        let a554 = {
                            let a552 = v3;
                            let a553 = 64u8;
                            fixed_u8::bit_and(a552, a553)
                        };
                        let a555 = 64u8;
                        if equal(a554, a555) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a556, a557)?
                };
                let a569 = {
                    let a566 = {
                        let a560 = {
                            let a558 = v3;
                            let a559 = 32u8;
                            fixed_u8::bit_and(a558, a559)
                        };
                        let a561 = 32u8;
                        if equal(a560, a561) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a567 = {
                        let a564 = {
                            let a562 = v3;
                            let a563 = 16u8;
                            fixed_u8::bit_and(a562, a563)
                        };
                        let a565 = 16u8;
                        if equal(a564, a565) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a566, a567)?
                };
                nat_add(a568, a569)?
            };
            let a593 = {
                let a590 = {
                    let a578 = {
                        let a572 = {
                            let a570 = v3;
                            let a571 = 8u8;
                            fixed_u8::bit_and(a570, a571)
                        };
                        let a573 = 8u8;
                        if equal(a572, a573) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a579 = {
                        let a576 = {
                            let a574 = v3;
                            let a575 = 4u8;
                            fixed_u8::bit_and(a574, a575)
                        };
                        let a577 = 4u8;
                        if equal(a576, a577) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a578, a579)?
                };
                let a591 = {
                    let a588 = {
                        let a582 = {
                            let a580 = v3;
                            let a581 = 2u8;
                            fixed_u8::bit_and(a580, a581)
                        };
                        let a583 = 2u8;
                        if equal(a582, a583) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a589 = {
                        let a586 = {
                            let a584 = v3;
                            let a585 = 1u8;
                            fixed_u8::bit_and(a584, a585)
                        };
                        let a587 = 1u8;
                        if equal(a586, a587) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a588, a589)?
                };
                nat_add(a590, a591)?
            };
            nat_add(a592, a593)
        }
    }
}

pub fn f76(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = f126(v0.clone(), v1, {
        let a595 = {
            let a594 = v0.clone();
            length_bytes(a594)
        };
        let a596 = 1u64;
        nat_add(a595, a596)?
    })?;
    let a597 = f75(v0.clone(), v2)?;
    let a598 = 34u64;
    if nat_eq(a597, a598) {
        Ok(Some(({
            let a599 = v2;
            let a600 = 1u64;
            nat_add(a599, a600)?
        }, f38(v0, v1, {
            let a601 = v2;
            let a602 = v1;
            nat_sub(a601, a602)
        }))))
    } else {
        f127(v0, v1)
    }
}

pub fn f77(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f78(v0: Bytes, v1: u64) -> R<Option<(u64, Adt2)>> {
    let v2: bool = {
        let a603 = f75(v0.clone(), v1)?;
        let a604 = 45u64;
        nat_eq(a603, a604)
    };
    let v3: u64 = if v2 {
        let a605 = v1;
        let a606 = 1u64;
        nat_add(a605, a606)?
    } else {
        v1
    };
    let v4: u64 = f128(v0.clone(), v3)?;
    let v5: u64 = {
        let a607 = v4;
        let a608 = v3;
        nat_sub(a607, a608)
    };
    let v6: bool = {
        let a609 = f75(v0.clone(), v4)?;
        let a610 = 46u64;
        nat_eq(a609, a610)
    };
    let v7: u64 = if v6 {
        let a611 = v4;
        let a612 = 1u64;
        nat_add(a611, a612)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f128(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f75(v0.clone(), v8)?;
    let v10: bool = {
        let a613 = v9;
        let a614 = 101u64;
        if nat_eq(a613, a614) {
            true
        } else {
            let a615 = v9;
            let a616 = 69u64;
            nat_eq(a615, a616)
        }
    };
    let v11: u64 = f75(v0.clone(), {
        let a617 = v8;
        let a618 = 1u64;
        nat_add(a617, a618)?
    })?;
    let v12: bool = f77(v10, f129({
        let a619 = v11;
        let a620 = 43u64;
        nat_eq(a619, a620)
    }, {
        let a621 = v11;
        let a622 = 45u64;
        nat_eq(a621, a622)
    }));
    let v13: u64 = if v10 {
        if v12 {
            let a623 = v8;
            let a624 = 2u64;
            nat_add(a623, a624)?
        } else {
            let a625 = v8;
            let a626 = 1u64;
            nat_add(a625, a626)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f128(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f38(v0.clone(), v13, {
        let a627 = v14;
        let a628 = v13;
        nat_sub(a627, a628)
    });
    let v16: u64 = f130(v15.clone())?;
    let v17: Bytes = f38(v15.clone(), v16, {
        let a630 = {
            let a629 = v15;
            length_bytes(a629)
        };
        let a631 = v16;
        nat_sub(a630, a631)
    });
    let a632 = v5;
    let a633 = 0u64;
    if if nat_eq(a632, a633) {
        true
    } else {
        if f77({
            let a634 = 1u64;
            let a635 = v5;
            nat_lt(a634, a635)
        }, {
            let a636 = f75(v0.clone(), v3)?;
            let a637 = 48u64;
            nat_eq(a636, a637)
        }) {
            true
        } else {
            if f77(v6, {
                let a638 = v8;
                let a639 = v7;
                nat_eq(a638, a639)
            }) {
                true
            } else {
                f129(if v10 {
                    let a640 = v14;
                    let a641 = v13;
                    nat_eq(a640, a641)
                } else {
                    false
                }, {
                    let a643 = 18u64;
                    let a644 = {
                        let a642 = v17.clone();
                        length_bytes(a642)
                    };
                    nat_lt(a643, a644)
                })
            }
        }
    } {
        Ok(None::<(u64, Adt2)>)
    } else {
        let m648 = {
            let a646 = {
                let a645 = v17.clone();
                length_bytes(a645)
            };
            let a647 = 0u64;
            if nat_eq(a646, a647) {
                Some(0u64)
            } else {
                f131(v17)?
            }
        };
        match m648 {
            None => {
                Ok(None::<(u64, Adt2)>)
            }
            Some(v18) => {
                let m653 = f132(f38(v0.clone(), v3, v5), f38(v0, v7, {
                    let a649 = v8;
                    let a650 = v7;
                    nat_sub(a649, a650)
                }), f77(v12, {
                    let a651 = v11;
                    let a652 = 45u64;
                    nat_eq(a651, a652)
                }), v18)?;
                match m653 {
                    None => {
                        Ok(None::<(u64, Adt2)>)
                    }
                    Some(v19) => {
                        let a656 = 19u64;
                        let a657 = {
                            let a655 = {
                                let (h654, _) = v19.clone();
                                h654
                            };
                            length_bytes(a655)
                        };
                        if nat_lt(a656, a657) {
                            Ok(None::<(u64, Adt2)>)
                        } else {
                            let m659 = f131({
                                let (h658, _) = v19.clone();
                                h658
                            })?;
                            match m659 {
                                None => {
                                    Ok(None::<(u64, Adt2)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f77(v2, {
                                        let a660 = 0u64;
                                        let a661 = v20;
                                        nat_lt(a660, a661)
                                    });
                                    if f133(v21, v20) {
                                        Ok(Some((v14, {
                                            let a663 = {
                                                let (_, h662) = v19.clone();
                                                h662
                                            };
                                            let a664 = 0u64;
                                            if nat_eq(a663, a664) {
                                                Adt2::C1(v21, v20)
                                            } else {
                                                Adt2::C2(v21, v20, {
                                                    let (_, h665) = v19;
                                                    h665
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

pub fn f79(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h666, _) = v2.clone();
        h666
    };
    let v4: u64 = {
        let (h667, _) = {
            let (_, h668) = v2.clone();
            h668
        };
        h667
    };
    let v5: List<Bytes> = {
        let (_, h669) = {
            let (_, h670) = v2;
            h670
        };
        h669
    };
    let m671 = f134(v0.clone(), v3);
    match m671 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a672 = v6;
            let a673 = v1;
            if equal(a672, a673) {
                Ok(Some(({
                    let a674 = v3;
                    let a675 = 1u64;
                    nat_add(a674, a675)?
                }, ({
                    let a676 = v3;
                    let a677 = 1u64;
                    nat_add(a676, a677)?
                }, List::cons(f38(v0, v4, {
                    let a678 = v3;
                    let a679 = v4;
                    nat_sub(a678, a679)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a680 = v3;
                    let a681 = 1u64;
                    nat_add(a680, a681)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f80(v0: Fn2, p685: List<Bytes>, p686: List<Bytes>) -> List<Bytes> {
    let m684 = (p685, p686);
    {
        let mut m684 = m684;
        loop {
            let step = {
                let (v1, v2) = m684;
                let m682 = v2;
                match m682.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c683: Fn2 = v0.clone();
                            c683.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m684 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f81(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f82(p691: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m690 = p691;
    {
        let mut m690 = m690;
        loop {
            let step = {
                let v0 = m690;
                let m687 = v0;
                match m687.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f41({
                            let (h688, _) = v2;
                            h688
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h689) = v2;
                                h689
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m690 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f83(v0: Bytes) -> R<Result<List<Adt13>, Bytes>> {
    let m692 = f135(v0)?;
    match m692 {
        Ok(v2) => {
            f136({
                let a694 = {
                    let a693 = v2.clone();
                    length_list(a693)
                };
                let a695 = 1u64;
                nat_add(a694, a695)?
            }, v2, List::<Adt13>::nil())
        }
        Err(v1) => {
            Ok(Err::<List<Adt13>, Bytes>(v1))
        }
    }
}

pub fn f84(v0: List<Bytes>) -> R<Result<List<Adt13>, Bytes>> {
    let m696 = v0;
    match m696.uncons() {
        None => {
            Ok(Ok::<List<Adt13>, Bytes>(List::<Adt13>::nil()))
        }
        Some((v1, v2)) => {
            let m697 = f83(v1)?;
            match m697 {
                Ok(v4) => {
                    let m698 = f84(v2)?;
                    match m698 {
                        Ok(v6) => {
                            Ok(Ok::<List<Adt13>, Bytes>(f85(v4, v6)))
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

pub fn f85(v0: List<Adt13>, v1: List<Adt13>) -> List<Adt13> {
    let m699 = v0;
    match m699.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f85(v3, v1))
        }
    }
}

pub fn f86(p704: u64, p705: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>) -> R<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>> {
    let m703 = (p704, p705);
    {
        let mut m703 = m703;
        loop {
            let step = {
                let (v0, v1) = m703;
                let m700 = v0;
                if m700 == 0 {
                    Err::<(u64, List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>), R<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>>>(Ok(v1))
                } else {
                    let v2 = m700 - 1;
                    let v3: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))> = f137(v1.clone(), v1.clone());
                    let a701 = f138(v3.clone())?;
                    let a702 = f138(v1.clone())?;
                    if nat_eq(a701, a702) {
                        Err::<(u64, List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>), R<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>>>(Ok(v1))
                    } else {
                        Ok::<(u64, List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>), R<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>>>((v2, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m703 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f87(v0: List<Adt13>, v1: List<Adt13>) -> R<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>> {
    let m706 = v0;
    match m706.uncons() {
        None => {
            Ok(List::<(Bytes, (Bytes, (Bytes, List<Bytes>)))>::nil())
        }
        Some((v2, v3)) => {
            if f139(f140(v2.clone())) {
                let v6: (Bytes, (Bytes, (Bytes, List<Bytes>))) = {
                    let m707 = f141(v1.clone(), v2.clone())?;
                    match m707 {
                        Ok(v5) => {
                            (f142(v2), (Bytes::lit(&[]), ({
                                let (h708, _) = v5;
                                h708
                            }, {
                                let (_, h709) = v5;
                                h709
                            })))
                        }
                        Err(v4) => {
                            (f142(v2), (v4, (Bytes::lit(&[]), List::<Bytes>::nil())))
                        }
                    }
                };
                Ok(List::cons(v6, f87(v3, v1)?))
            } else {
                f87(v3, v1)
            }
        }
    }
}

pub fn f88(p719: u64, p720: List<Bytes>, p721: List<Bytes>, v3: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>) -> List<Bytes> {
    let m718 = (p719, (p720, p721));
    {
        let mut m718 = m718;
        loop {
            let step = {
                let (v0, (v1, v2)) = m718;
                let m710 = v0;
                if m710 == 0 {
                    Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v2)
                } else {
                    let v4 = m710 - 1;
                    let m711 = v1;
                    match m711.uncons() {
                        None => {
                            Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v2)
                        }
                        Some((v5, v6)) => {
                            if f143(v5.clone(), v2.clone()) {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v2)))
                            } else {
                                let m712 = f144(v3.clone(), v5.clone());
                                match m712 {
                                    None => {
                                        Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v2)))
                                    }
                                    Some(v7) => {
                                        if f41({
                                            let (h713, _) = {
                                                let (_, h714) = v7.clone();
                                                h714
                                            };
                                            h713
                                        }, Bytes::lit(&[])) {
                                            Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (f145({
                                                let (_, h715) = {
                                                    let (_, h716) = {
                                                        let (_, h717) = v7;
                                                        h717
                                                    };
                                                    h716
                                                };
                                                h715
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
                Ok(next) => { m718 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f89(v0: List<Adt13>) -> List<Bytes> {
    let m722 = v0;
    match m722.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let v3: u64 = f140(v1.clone());
            let a723 = v3;
            let a724 = 0u64;
            if if nat_eq(a723, a724) {
                true
            } else {
                let a725 = v3;
                let a726 = 2u64;
                if nat_eq(a725, a726) {
                    true
                } else {
                    let a727 = v3;
                    let a728 = 3u64;
                    if nat_eq(a727, a728) {
                        true
                    } else {
                        let a729 = v3;
                        let a730 = 4u64;
                        if nat_eq(a729, a730) {
                            true
                        } else {
                            let a731 = v3;
                            let a732 = 5u64;
                            if nat_eq(a731, a732) {
                                true
                            } else {
                                let a733 = v3;
                                let a734 = 7u64;
                                if nat_eq(a733, a734) {
                                    true
                                } else {
                                    let a735 = v3;
                                    let a736 = 8u64;
                                    nat_eq(a735, a736)
                                }
                            }
                        }
                    }
                }
            } {
                List::cons(f142(v1), f89(v2))
            } else {
                f89(v2)
            }
        }
    }
}

pub fn f90(v0: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>, v1: List<Bytes>) -> List<(Bytes, (List<Bytes>, Bytes))> {
    let m737 = v0;
    match m737.uncons() {
        None => {
            List::<(Bytes, (List<Bytes>, Bytes))>::nil()
        }
        Some((v2, v3)) => {
            if if f143({
                let (h738, _) = v2.clone();
                h738
            }, v1.clone()) {
                f41({
                    let (h739, _) = {
                        let (_, h740) = v2.clone();
                        h740
                    };
                    h739
                }, Bytes::lit(&[]))
            } else {
                false
            } {
                List::cons(({
                    let (h741, _) = v2.clone();
                    h741
                }, ({
                    let (_, h742) = {
                        let (_, h743) = {
                            let (_, h744) = v2.clone();
                            h744
                        };
                        h743
                    };
                    h742
                }, {
                    let (h745, _) = {
                        let (_, h746) = {
                            let (_, h747) = v2;
                            h747
                        };
                        h746
                    };
                    h745
                })), f90(v3, v1))
            } else {
                f90(v3, v1)
            }
        }
    }
}

pub fn f91(p756: u64, p757: List<(Bytes, (List<Bytes>, Bytes))>, p758: List<Bytes>, p759: List<Bytes>) -> (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>) {
    let m755 = (p756, (p757, (p758, p759)));
    {
        let mut m755 = m755;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m755;
                let m748 = v0;
                if m748 == 0 {
                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v3, v1))
                } else {
                    let v4 = m748 - 1;
                    let m749 = f146(v1.clone(), v2.clone());
                    match m749 {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v3, v1))
                        }
                        Some(v5) => {
                            Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v4, ({
                                let (_, h750) = v5.clone();
                                h750
                            }, (List::cons({
                                let (h751, _) = {
                                    let (h752, _) = v5.clone();
                                    h752
                                };
                                h751
                            }, v2), List::cons({
                                let (_, h753) = {
                                    let (h754, _) = v5;
                                    h754
                                };
                                h753
                            }, v3)))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m755 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f92(v0: List<Adt13>, v1: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>, v2: List<Adt13>) -> List<Bytes> {
    let m760 = v0;
    match m760.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            let v5: u64 = f140(v3.clone());
            let a761 = v5;
            let a762 = 5u64;
            if if nat_eq(a761, a762) {
                true
            } else {
                let a763 = v5;
                let a764 = 7u64;
                nat_eq(a763, a764)
            } {
                let m767 = f147(v2.clone(), {
                    let a765 = v5;
                    let a766 = 5u64;
                    if nat_eq(a765, a766) {
                        0u64
                    } else {
                        2u64
                    }
                }, f142(v3.clone()));
                match m767 {
                    None => {
                        List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f148(v5), List::cons(f142(v3.clone()), List::cons(f8(Str::lit(": no input defines ")), List::cons(f142(v3), List::cons(f8(Str::lit(", so its members join no definition (Web IDL 2.2 and 2.7)\u{a}")), List::<Bytes>::nil()))))))), f92(v4, v1, v2))
                    }
                    Some(_) => {
                        f92(v4, v1, v2)
                    }
                }
            } else {
                let a768 = v5;
                let a769 = 8u64;
                if nat_eq(a768, a769) {
                    let m770 = f147(v2.clone(), 0u64, f142(v3.clone()));
                    match m770 {
                        None => {
                            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f142(v3.clone()), List::cons(f8(Str::lit(" includes ")), List::cons(f149(v3.clone()), List::cons(f8(Str::lit(": no input defines the interface ")), List::cons(f142(v3), List::cons(f8(Str::lit(" (Web IDL 2.3)\u{a}")), List::<Bytes>::nil())))))))), f92(v4, v1, v2))
                        }
                        Some(_) => {
                            f92(v4, v1, v2)
                        }
                    }
                } else {
                    if f139(v5) {
                        let m771 = f144(v1.clone(), f142(v3.clone()));
                        match m771 {
                            None => {
                                f92(v4, v1, v2)
                            }
                            Some(v8) => {
                                if f41({
                                    let (h772, _) = {
                                        let (_, h773) = v8.clone();
                                        h773
                                    };
                                    h772
                                }, Bytes::lit(&[])) {
                                    f92(v4, v1, v2)
                                } else {
                                    List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f142(v3), List::cons(f8(Str::lit(": ")), List::cons({
                                        let (h774, _) = {
                                            let (_, h775) = v8;
                                            h775
                                        };
                                        h774
                                    }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), f92(v4, v1, v2))
                                }
                            }
                        }
                    } else {
                        f92(v4, v1, v2)
                    }
                }
            }
        }
    }
}

pub fn f93(v0: List<(Bytes, (List<Bytes>, Bytes))>) -> List<Bytes> {
    let m776 = v0;
    match m776.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons({
                let (h777, _) = v1;
                h777
            }, List::cons(f8(Str::lit(": it refers back to itself through its members, and the translator emits non-recursive declarations only\u{a}")), List::<Bytes>::nil())))), f93(v2))
        }
    }
}

pub fn f94(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m778 = v0;
    match m778.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m779 = v3.clone();
            match m779.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a782 = v2;
                    let a783 = {
                        let a780 = v1.clone();
                        let a781 = f94(v3, v1);
                        append_bytes(a780, a781)
                    };
                    append_bytes(a782, a783)
                }
            }
        }
    }
}

pub fn f95(p786: List<Bytes>, p787: List<Bytes>) -> List<Bytes> {
    let m785 = (p786, p787);
    {
        let mut m785 = m785;
        loop {
            let step = {
                let (v0, v1) = m785;
                let m784 = v0;
                match m784.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m785 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f96(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a788 = v2;
    let a789 = 0u64;
    if nat_eq(a788, a789) {
        Ok(f150(v0, 0u64))
    } else {
        let (_, h790) = {
            let (h791, _) = f151(Fn3::F152(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h791
        };
        Ok(h790)
    }
}

pub fn f97() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f98(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m796 = {
        let a793 = v0;
        let a794 = v2;
        let a795 = {
            let a792 = v1.clone();
            length_bytes(a792)
        };
        slice_bytes(a793, a794, a795)
    };
    match m796 {
        None => {
            false
        }
        Some(v3) => {
            let a797 = v3;
            let a798 = v1;
            equal(a797, a798)
        }
    }
}

pub fn f99(v0: Bytes) -> R<Bytes> {
    let (_, h799) = {
        let (h800, _) = f151(Fn3::F153(v0.clone()), {
            let a802 = {
                let a801 = v0;
                length_bytes(a801)
            };
            let a803 = 1u64;
            nat_add(a802, a803)?
        }, (0u64, Bytes::lit(&[])))?;
        h800
    };
    Ok(h799)
}

pub fn f100(v0: Bytes) -> R<u64> {
    let (h804, _) = f154(Fn4::F155(v0.clone()), {
        let a806 = {
            let a805 = v0;
            length_bytes(a805)
        };
        let a807 = 1u64;
        nat_add(a806, a807)?
    }, 0u64)?;
    Ok(h804)
}

pub fn f101(v0: Bytes) -> Result<Adt14, Bytes> {
    Err::<Adt14, Bytes>(v0)
}

pub fn f102(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a808 = v0;
        utf8_encode(a808)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f59(v1)?, List::<Bytes>::nil())))))
}

pub fn f103(v0: Bytes) -> R<Result<u64, Bytes>> {
    if if f105(v0.clone(), 0u64, Str::lit("<?xml")) {
        f156(f106(v0.clone(), 5u64)?)?
    } else {
        false
    } {
        let m810 = f157(v0.clone(), {
            let a809 = Str::lit("?>");
            utf8_encode(a809)
        }, 5u64)?;
        match m810 {
            None => {
                Ok(f158(f102(Str::lit("the XML declaration is not closed (XML 1.0 [23])"), 0u64)?))
            }
            Some(v1) => {
                let m812 = f157(f38(v0.clone(), 0u64, v1), {
                    let a811 = Str::lit("encoding");
                    utf8_encode(a811)
                }, 5u64)?;
                match m812 {
                    None => {
                        Ok(Ok::<u64, Bytes>({
                            let a813 = v1;
                            let a814 = 2u64;
                            nat_add(a813, a814)?
                        }))
                    }
                    Some(v2) => {
                        let v3: u64 = f159(v0.clone(), {
                            let a815 = v2;
                            let a816 = 8u64;
                            nat_add(a815, a816)?
                        })?;
                        let v4: u64 = f159(v0.clone(), {
                            let a817 = v3;
                            let a818 = 1u64;
                            nat_add(a817, a818)?
                        })?;
                        let _: u64 = f106(v0.clone(), v4)?;
                        let v7: u64 = {
                            let m821 = f157(v0.clone(), f38(v0.clone(), v4, 1u64), {
                                let a819 = v4;
                                let a820 = 1u64;
                                nat_add(a819, a820)?
                            })?;
                            match m821 {
                                None => {
                                    v1
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        if f160(f38(v0, {
                            let a822 = v4;
                            let a823 = 1u64;
                            nat_add(a822, a823)?
                        }, {
                            let a826 = v7;
                            let a827 = {
                                let a824 = v4;
                                let a825 = 1u64;
                                nat_add(a824, a825)?
                            };
                            nat_sub(a826, a827)
                        })) {
                            Ok(Ok::<u64, Bytes>({
                                let a828 = v1;
                                let a829 = 2u64;
                                nat_add(a828, a829)?
                            }))
                        } else {
                            Ok(f158(f102(Str::lit("only the UTF-8 encoding is read (XML 1.0 section 4.3.3)"), 0u64)?))
                        }
                    }
                }
            }
        }
    } else {
        Ok(Ok::<u64, Bytes>(0u64))
    }
}

pub fn f104(v0: Bytes, p834: u64, p835: u64) -> R<Result<u64, Bytes>> {
    let m833 = (p834, p835);
    {
        let mut m833 = m833;
        loop {
            let step = {
                let (v1, v2) = m833;
                let m830 = v2;
                if m830 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f158(f102(Str::lit("the prolog is longer than the input"), v1)?)))
                } else {
                    let v3 = m830 - 1;
                    let v4: u64 = f159(v0.clone(), v1)?;
                    if f105(v0.clone(), v4, Str::lit("<!--")) {
                        let m831 = f161(v0.clone(), v4)?;
                        match m831 {
                            Ok(v6) => {
                                Ok::<(u64, u64), R<Result<u64, Bytes>>>((v6, v3))
                            }
                            Err(v5) => {
                                Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f158(v5)))
                            }
                        }
                    } else {
                        if f105(v0.clone(), v4, Str::lit("<?")) {
                            let m832 = f162(v0.clone(), v4)?;
                            match m832 {
                                Ok(v8) => {
                                    Ok::<(u64, u64), R<Result<u64, Bytes>>>((v8, v3))
                                }
                                Err(v7) => {
                                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f158(v7)))
                                }
                            }
                        } else {
                            Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Ok::<u64, Bytes>(v4)))
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

pub fn f105(v0: Bytes, v1: u64, v2: Str) -> bool {
    f98(v0, {
        let a836 = v2;
        utf8_encode(a836)
    }, v1)
}

pub fn f106(v0: Bytes, v1: u64) -> R<u64> {
    f75(v0, v1)
}

pub fn f107(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Result<(Adt14, u64), Bytes>> {
    let m837 = v2;
    if m837 == 0 {
        Ok(f163(f102(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v4 = m837 - 1;
        let a838 = v3;
        let a839 = 0u64;
        if nat_eq(a838, a839) {
            let v5: u64 = f164(v0.clone(), {
                let a840 = v1;
                let a841 = 1u64;
                nat_add(a840, a841)?
            })?;
            let a844 = v5;
            let a845 = {
                let a842 = v1;
                let a843 = 1u64;
                nat_add(a842, a843)?
            };
            if nat_eq(a844, a845) {
                Ok(f163(f102(Str::lit("an element must start with a Name (XML 1.0 [40])"), v1)?))
            } else {
                let v6: Bytes = f38(v0.clone(), {
                    let a846 = v1;
                    let a847 = 1u64;
                    nat_add(a846, a847)?
                }, {
                    let a850 = v5;
                    let a851 = {
                        let a848 = v1;
                        let a849 = 1u64;
                        nat_add(a848, a849)?
                    };
                    nat_sub(a850, a851)
                });
                let m852 = f165(v0.clone(), v5, v4, List::<Bytes>::nil())?;
                match m852 {
                    Ok(v8) => {
                        if f105(v0.clone(), {
                            let (_, h853) = v8.clone();
                            h853
                        }, Str::lit("/>")) {
                            Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C0(v6, std::rc::Rc::new({
                                let (h854, _) = v8;
                                h854
                            }), std::rc::Rc::new(Adt14::C4)), {
                                let a856 = {
                                    let (_, h855) = v8;
                                    h855
                                };
                                let a857 = 2u64;
                                nat_add(a856, a857)?
                            })))
                        } else {
                            let m861 = f107(v0.clone(), {
                                let a859 = {
                                    let (_, h858) = v8;
                                    h858
                                };
                                let a860 = 1u64;
                                nat_add(a859, a860)?
                            }, v4, 1u64)?;
                            match m861 {
                                Ok(v10) => {
                                    let v11: u64 = {
                                        let a863 = {
                                            let (_, h862) = v10.clone();
                                            h862
                                        };
                                        let a864 = 2u64;
                                        nat_add(a863, a864)?
                                    };
                                    let v12: u64 = f164(v0.clone(), v11)?;
                                    if f41(f38(v0.clone(), v11, {
                                        let a865 = v12;
                                        let a866 = v11;
                                        nat_sub(a865, a866)
                                    }), v6.clone()) {
                                        let v13: u64 = f159(v0.clone(), v12)?;
                                        let a867 = f106(v0, v13)?;
                                        let a868 = 62u64;
                                        if nat_eq(a867, a868) {
                                            Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C0(v6, std::rc::Rc::new({
                                                let (h869, _) = v8;
                                                h869
                                            }), std::rc::Rc::new({
                                                let (h870, _) = v10;
                                                h870
                                            })), {
                                                let a871 = v13;
                                                let a872 = 1u64;
                                                nat_add(a871, a872)?
                                            })))
                                        } else {
                                            Ok(f163(f102(Str::lit("an end tag must close with > (XML 1.0 [42])"), v13)?))
                                        }
                                    } else {
                                        Ok(f163(f102(Str::lit("an end tag must name the element its start tag opened (XML 1.0 WFC: Element Type Match)"), {
                                            let (_, h873) = v10;
                                            h873
                                        })?))
                                    }
                                }
                                Err(v9) => {
                                    Ok(f163(v9))
                                }
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(f163(v7))
                    }
                }
            }
        } else {
            let a874 = f106(v0.clone(), v1)?;
            let a875 = 256u64;
            if nat_eq(a874, a875) {
                Ok(f163(f102(Str::lit("the input ends before an element is closed (XML 1.0 [39])"), v1)?))
            } else {
                if f105(v0.clone(), v1, Str::lit("</")) {
                    Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C4, v1)))
                } else {
                    if f105(v0.clone(), v1, Str::lit("<!--")) {
                        let m876 = f161(v0.clone(), v1)?;
                        match m876 {
                            Ok(v15) => {
                                f107(v0, v15, v4, 1u64)
                            }
                            Err(v14) => {
                                Ok(f163(v14))
                            }
                        }
                    } else {
                        if f105(v0.clone(), v1, Str::lit("<![CDATA[")) {
                            let m880 = f157(v0.clone(), {
                                let a877 = Str::lit("]]>");
                                utf8_encode(a877)
                            }, {
                                let a878 = v1;
                                let a879 = 9u64;
                                nat_add(a878, a879)?
                            })?;
                            match m880 {
                                None => {
                                    Ok(f163(f102(Str::lit("a CDATA section is not closed (XML 1.0 [18])"), v1)?))
                                }
                                Some(v16) => {
                                    let m883 = f107(v0.clone(), {
                                        let a881 = v16;
                                        let a882 = 3u64;
                                        nat_add(a881, a882)?
                                    }, v4, 1u64)?;
                                    match m883 {
                                        Ok(v18) => {
                                            Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C3(std::rc::Rc::new(Adt14::C2(f38(v0, {
                                                let a884 = v1;
                                                let a885 = 9u64;
                                                nat_add(a884, a885)?
                                            }, {
                                                let a888 = v16;
                                                let a889 = {
                                                    let a886 = v1;
                                                    let a887 = 9u64;
                                                    nat_add(a886, a887)?
                                                };
                                                nat_sub(a888, a889)
                                            }))), std::rc::Rc::new({
                                                let (h890, _) = v18;
                                                h890
                                            })), {
                                                let (_, h891) = v18;
                                                h891
                                            })))
                                        }
                                        Err(v17) => {
                                            Ok(f163(v17))
                                        }
                                    }
                                }
                            }
                        } else {
                            if f105(v0.clone(), v1, Str::lit("<?")) {
                                let m892 = f162(v0.clone(), v1)?;
                                match m892 {
                                    Ok(v20) => {
                                        f107(v0, v20, v4, 1u64)
                                    }
                                    Err(v19) => {
                                        Ok(f163(v19))
                                    }
                                }
                            } else {
                                if f105(v0.clone(), v1, Str::lit("<!")) {
                                    Ok(f163(f102(Str::lit("a markup declaration is not allowed in content (XML 1.0 [43])"), v1)?))
                                } else {
                                    let a893 = f106(v0.clone(), v1)?;
                                    let a894 = 60u64;
                                    if nat_eq(a893, a894) {
                                        let m895 = f107(v0.clone(), v1, v4, 0u64)?;
                                        match m895 {
                                            Ok(v22) => {
                                                let m897 = f107(v0, {
                                                    let (_, h896) = v22;
                                                    h896
                                                }, v4, 1u64)?;
                                                match m897 {
                                                    Ok(v24) => {
                                                        Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C3(std::rc::Rc::new({
                                                            let (h898, _) = v22;
                                                            h898
                                                        }), std::rc::Rc::new({
                                                            let (h899, _) = v24;
                                                            h899
                                                        })), {
                                                            let (_, h900) = v24;
                                                            h900
                                                        })))
                                                    }
                                                    Err(v23) => {
                                                        Ok(f163(v23))
                                                    }
                                                }
                                            }
                                            Err(v21) => {
                                                Ok(f163(v21))
                                            }
                                        }
                                    } else {
                                        let m901 = f166(v0.clone(), v1, v4, Bytes::lit(&[]), false, 0u64)?;
                                        match m901 {
                                            Ok(v26) => {
                                                let m903 = f107(v0, {
                                                    let (_, h902) = v26;
                                                    h902
                                                }, v4, 1u64)?;
                                                match m903 {
                                                    Ok(v28) => {
                                                        Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C3(std::rc::Rc::new(Adt14::C2({
                                                            let (h904, _) = v26;
                                                            h904
                                                        })), std::rc::Rc::new({
                                                            let (h905, _) = v28;
                                                            h905
                                                        })), {
                                                            let (_, h906) = v28;
                                                            h906
                                                        })))
                                                    }
                                                    Err(v27) => {
                                                        Ok(f163(v27))
                                                    }
                                                }
                                            }
                                            Err(v25) => {
                                                Ok(f163(v25))
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

pub fn f108(v0: Bytes) -> Result<(u64, Bytes), Bytes> {
    Err::<(u64, Bytes), Bytes>(v0)
}

pub fn f109(v0: Adt14, v1: List<(Bytes, Bytes)>) -> Result<List<(Bytes, Bytes)>, Bytes> {
    let m907 = v0;
    match m907 {
        Adt14::C0(_, _, _) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt14::C1(v2, v3, r912) => {
            let v4 = (*r912).clone();
            let m908 = f109(v4, v1);
            match m908 {
                Ok(v6) => {
                    if f41(v2.clone(), f8(Str::lit("xmlns"))) {
                        if if f41(v3.clone(), f168()) {
                            true
                        } else {
                            f41(v3.clone(), f169())
                        } {
                            f167(f8(Str::lit("the default namespace must not be the xml or xmlns namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Bytes>(List::cons((Bytes::lit(&[]), v3), v6))
                        }
                    } else {
                        if f170(v2.clone(), f8(Str::lit("xmlns:"))) {
                            let v7: Bytes = f38(v2.clone(), 6u64, {
                                let a910 = {
                                    let a909 = v2;
                                    length_bytes(a909)
                                };
                                let a911 = 6u64;
                                nat_sub(a910, a911)
                            });
                            if f41(v7.clone(), f8(Str::lit("xmlns"))) {
                                f167(f8(Str::lit("the prefix xmlns must not be declared (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                            } else {
                                if if f41(v7.clone(), f8(Str::lit("xml"))) {
                                    !f41(v3.clone(), f168())
                                } else {
                                    false
                                } {
                                    f167(f8(Str::lit("the prefix xml may be bound only to its own namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                } else {
                                    if if if f41(v3.clone(), f168()) {
                                        true
                                    } else {
                                        f41(v3.clone(), f169())
                                    } {
                                        !f41(v7.clone(), f8(Str::lit("xml")))
                                    } else {
                                        false
                                    } {
                                        f167(f8(Str::lit("the xml and xmlns namespace names must not be bound to another prefix (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                    } else {
                                        if f41(v3.clone(), Bytes::lit(&[])) {
                                            f167(f8(Str::lit("a prefix must not be undeclared (Namespaces in XML 1.0, section 3, NSC: No Prefix Undeclaring)")))
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
                    f167(v5)
                }
            }
        }
        Adt14::C2(_) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt14::C3(_, _) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt14::C4 => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
    }
}

pub fn f110(v0: List<(Bytes, Bytes)>, v1: Bytes) -> R<Result<(Bytes, Bytes), Bytes>> {
    let v2: Bytes = f171(v1.clone())?;
    if f41(v2.clone(), Bytes::lit(&[])) {
        let m913 = f172(v0, Bytes::lit(&[]));
        match m913 {
            None => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((Bytes::lit(&[]), v1)))
            }
            Some(v3) => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((v3, v1)))
            }
        }
    } else {
        if f41(v2.clone(), f8(Str::lit("xml"))) {
            Ok(Ok::<(Bytes, Bytes), Bytes>((f168(), f173(v1)?)))
        } else {
            let m914 = f172(v0, v2.clone());
            match m914 {
                None => {
                    Ok(Err::<(Bytes, Bytes), Bytes>(f7(List::cons(f8(Str::lit("the prefix ")), List::cons(v2, List::cons(f8(Str::lit(" is not declared (Namespaces in XML 1.0, section 5, NSC: Prefix Declared)")), List::<Bytes>::nil()))))))
                }
                Some(v4) => {
                    Ok(Ok::<(Bytes, Bytes), Bytes>((v4, f173(v1)?)))
                }
            }
        }
    }
}

pub fn f111(v0: (Bytes, Bytes), v1: Str) -> bool {
    if f41({
        let (h915, _) = v0;
        h915
    }, f174()) {
        f41({
            let (_, h916) = v0;
            h916
        }, {
            let a917 = v1;
            utf8_encode(a917)
        })
    } else {
        false
    }
}

pub fn f112(p921: Adt14, v1: Bytes) -> Option<Bytes> {
    let m920 = p921;
    {
        let mut m920 = m920;
        loop {
            let step = {
                let v0 = m920;
                let m918 = v0;
                match m918 {
                    Adt14::C0(_, _, _) => {
                        Err::<Adt14, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt14::C1(v2, v3, r919) => {
                        let v4 = (*r919).clone();
                        if f41(v2, v1.clone()) {
                            Err::<Adt14, Option<Bytes>>(Some(v3))
                        } else {
                            Ok::<Adt14, Option<Bytes>>(v4)
                        }
                    }
                    Adt14::C2(_) => {
                        Err::<Adt14, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt14::C3(_, _) => {
                        Err::<Adt14, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt14::C4 => {
                        Err::<Adt14, Option<Bytes>>(None::<Bytes>)
                    }
                }
            };
            match step {
                Ok(next) => { m920 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f113(v0: Adt14, v1: List<(Bytes, Bytes)>, v2: Bytes) -> R<Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>> {
    let m922 = v0;
    match m922 {
        Adt14::C0(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt14::C1(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt14::C2(_) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt14::C3(r933, r934) => {
            let v3 = (*r933).clone();
            let v4 = (*r934).clone();
            let m923 = f113(v4, v1.clone(), v2.clone())?;
            match m923 {
                Ok(v6) => {
                    let m924 = v3;
                    match m924 {
                        Adt14::C0(v7, r931, r932) => {
                            let v8 = (*r931).clone();
                            let v9 = (*r932).clone();
                            let m925 = f109(v8.clone(), v1);
                            match m925 {
                                Ok(v11) => {
                                    let m926 = f110(v11.clone(), v7)?;
                                    match m926 {
                                        Ok(v13) => {
                                            if f111(v13, Str::lit("simpleType")) {
                                                let m927 = f112(v8, f8(Str::lit("name")));
                                                match m927 {
                                                    None => {
                                                        Ok(f175(f8(Str::lit("a top-level simpleType must have a name (XSD 1.1 Part 2, section 4.1.2)"))))
                                                    }
                                                    Some(v14) => {
                                                        let m928 = f176(v14.clone(), v9, v11, v2)?;
                                                        match m928 {
                                                            Ok(v16) => {
                                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::cons((f177(v14)?, (match v16.clone() {
                                                                    Adt15::C0(_, h929) => {
                                                                        h929
                                                                    }
                                                                }, match v16 {
                                                                    Adt15::C0(h930, _) => {
                                                                        h930
                                                                    }
                                                                })), v6)))
                                                            }
                                                            Err(v15) => {
                                                                Ok(f175(v15))
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                                            }
                                        }
                                        Err(v12) => {
                                            Ok(f175(v12))
                                        }
                                    }
                                }
                                Err(v10) => {
                                    Ok(f175(v10))
                                }
                            }
                        }
                        Adt14::C1(_, _, _) => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                        Adt14::C2(_) => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                        Adt14::C3(_, _) => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                        Adt14::C4 => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                    }
                }
                Err(v5) => {
                    Ok(f175(v5))
                }
            }
        }
        Adt14::C4 => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
    }
}

pub fn f114(v0: List<(Bytes, (List<Bytes>, Bytes))>) -> List<Bytes> {
    let m935 = v0;
    match m935.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h936, _) = v1;
                h936
            }, f114(v2))
        }
    }
}

pub fn f115(p939: List<Bytes>) -> Option<Bytes> {
    let m938 = p939;
    {
        let mut m938 = m938;
        loop {
            let step = {
                let v0 = m938;
                let m937 = v0;
                match m937.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f143(v1.clone(), v2.clone()) {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v1))
                        } else {
                            Ok::<List<Bytes>, Option<Bytes>>(v2)
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

pub fn f116(p945: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<Bytes> {
    let m944 = p945;
    {
        let mut m944 = m944;
        loop {
            let step = {
                let v0 = m944;
                let m940 = v0;
                match m940.uncons() {
                    None => {
                        Err::<List<(Bytes, (List<Bytes>, Bytes))>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        let m943 = f178({
                            let (h941, _) = {
                                let (_, h942) = v2;
                                h942
                            };
                            h941
                        }, v1.clone());
                        match m943 {
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
                Ok(next) => { m944 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f117(p956: u64, p957: List<(Bytes, (List<Bytes>, Bytes))>, p958: List<Bytes>, p959: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m955 = (p956, (p957, (p958, p959)));
    {
        let mut m955 = m955;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m955;
                let m946 = v0;
                if m946 == 0 {
                    let m947 = v1;
                    match m947.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(f179(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                        }
                    }
                } else {
                    let v6 = m946 - 1;
                    let m948 = v1.clone();
                    match m948.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            let m949 = f146(v1, v2.clone());
                            match m949 {
                                None => {
                                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(f179(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                                }
                                Some(v9) => {
                                    Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>((v6, ({
                                        let (_, h950) = v9.clone();
                                        h950
                                    }, (List::cons({
                                        let (h951, _) = {
                                            let (h952, _) = v9.clone();
                                            h952
                                        };
                                        h951
                                    }, v2), List::cons({
                                        let (_, h953) = {
                                            let (h954, _) = v9;
                                            h954
                                        };
                                        h953
                                    }, v3)))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m955 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f118(v0: Bytes) -> R<u64> {
    let (h960, _) = f154(Fn4::F180(v0.clone()), {
        let a962 = {
            let a961 = v0;
            length_bytes(a961)
        };
        let a963 = 1u64;
        nat_add(a962, a963)?
    }, 0u64)?;
    Ok(h960)
}

pub fn f119(v0: Bytes, v1: u64) -> R<u64> {
    let (h964, _) = f154(Fn4::F181(v0.clone()), {
        let a966 = {
            let a965 = v0;
            length_bytes(a965)
        };
        let a967 = 1u64;
        nat_add(a966, a967)?
    }, v1)?;
    Ok(h964)
}

pub fn f120(v0: Bytes, v1: u64) -> R<bool> {
    if if if f105(v0.clone(), v1, Str::lit("---")) {
        true
    } else {
        f105(v0.clone(), v1, Str::lit("..."))
    } {
        if f182(f121(v0.clone(), {
            let a968 = v1;
            let a969 = 3u64;
            nat_add(a968, a969)?
        })?) {
            true
        } else {
            f183(f121(v0.clone(), {
                let a970 = v1;
                let a971 = 3u64;
                nat_add(a970, a971)?
            })?)
        }
    } else {
        false
    } {
        let a972 = f184(v0, v1)?;
        let a973 = 0u64;
        Ok(nat_eq(a972, a973))
    } else {
        Ok(false)
    }
}

pub fn f121(v0: Bytes, v1: u64) -> R<u64> {
    f75(v0, v1)
}

pub fn f122(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: List<Bytes>) -> R<Result<(Adt2, u64), Bytes>> {
    let m974 = v3;
    if m974 == 0 {
        Ok(Err::<(Adt2, u64), Bytes>(f102(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v8 = m974 - 1;
        let a975 = v4;
        let a976 = 0u64;
        if nat_eq(a975, a976) {
            let v9: u64 = f185(v0.clone(), v1)?;
            let v10: u64 = {
                let a977 = v9;
                let a978 = v1;
                nat_sub(a977, a978)
            };
            let a979 = f121(v0.clone(), v9)?;
            let a980 = 9u64;
            if nat_eq(a979, a980) {
                let v11: u64 = f186(v0.clone(), v9)?;
                if if f187(v0.clone(), v11)? {
                    true
                } else {
                    f188(v0.clone(), v11)?
                } {
                    Ok(Err::<(Adt2, u64), Bytes>(f102(Str::lit("a tab cannot indent a block node (YAML 1.2.2 [63])"), v11)?))
                } else {
                    let m981 = f189(v0.clone(), v11)?;
                    match m981 {
                        None => {
                            f190(v0, v11, v2)
                        }
                        Some(v12) => {
                            Ok(Err::<(Adt2, u64), Bytes>(v12))
                        }
                    }
                }
            } else {
                if f187(v0.clone(), v9)? {
                    Ok(f191(true, f122(v0, v9, v2, v8, 2u64, false, v10, List::<Bytes>::nil())?))
                } else {
                    let m982 = f189(v0.clone(), v9)?;
                    match m982 {
                        None => {
                            if f188(v0.clone(), v9)? {
                                Ok(f191(false, f122(v0, v9, v2, v8, 3u64, false, v10, List::<Bytes>::nil())?))
                            } else {
                                f190(v0, v9, v2)
                            }
                        }
                        Some(v13) => {
                            Ok(Err::<(Adt2, u64), Bytes>(v13))
                        }
                    }
                }
            }
        } else {
            let a983 = v4;
            let a984 = 1u64;
            if nat_eq(a983, a984) {
                let v14: u64 = f186(v0.clone(), v1)?;
                let v15: u64 = f121(v0.clone(), v14)?;
                if if f183(v15) {
                    true
                } else {
                    let a985 = v15;
                    let a986 = 35u64;
                    if nat_eq(a985, a986) {
                        let a987 = v1;
                        let a988 = v14;
                        nat_lt(a987, a988)
                    } else {
                        false
                    }
                } {
                    let v16: u64 = f119(v0.clone(), f192(v0.clone(), v14)?)?;
                    let a990 = {
                        let a989 = v0.clone();
                        length_bytes(a989)
                    };
                    let a991 = v16;
                    if if nat_le(a990, a991) {
                        true
                    } else {
                        f120(v0.clone(), v16)?
                    } {
                        Ok(Err::<(Adt2, u64), Bytes>(f193(v14)?))
                    } else {
                        let v17: u64 = {
                            let a992 = f185(v0.clone(), v16)?;
                            let a993 = v16;
                            nat_sub(a992, a993)
                        };
                        let a994 = v2;
                        let a995 = v17;
                        if nat_le(a994, a995) {
                            f122(v0, v16, v2, v8, 0u64, false, 0u64, List::<Bytes>::nil())
                        } else {
                            if if if f194(v5) {
                                let a996 = v17;
                                let a997 = v6;
                                nat_eq(a996, a997)
                            } else {
                                false
                            } {
                                f187(v0.clone(), {
                                    let a998 = v16;
                                    let a999 = v17;
                                    nat_add(a998, a999)?
                                })?
                            } else {
                                false
                            } {
                                Ok(f191(true, f122(v0, {
                                    let a1000 = v16;
                                    let a1001 = v17;
                                    nat_add(a1000, a1001)?
                                }, v2, v8, 2u64, false, v17, List::<Bytes>::nil())?))
                            } else {
                                Ok(Err::<(Adt2, u64), Bytes>(f193(v14)?))
                            }
                        }
                    }
                } else {
                    if if v5 {
                        f187(v0.clone(), v14)?
                    } else {
                        false
                    } {
                        Ok(f191(true, f122(v0.clone(), v14, v2, v8, 2u64, false, f184(v0, v14)?, List::<Bytes>::nil())?))
                    } else {
                        let m1002 = f189(v0.clone(), v14)?;
                        match m1002 {
                            None => {
                                if if v5 {
                                    f188(v0.clone(), v14)?
                                } else {
                                    false
                                } {
                                    Ok(f191(false, f122(v0.clone(), v14, v2, v8, 3u64, false, f184(v0, v14)?, List::<Bytes>::nil())?))
                                } else {
                                    f190(v0, v14, v2)
                                }
                            }
                            Some(v18) => {
                                Ok(Err::<(Adt2, u64), Bytes>(v18))
                            }
                        }
                    }
                }
            } else {
                let a1003 = v4;
                let a1004 = 2u64;
                if nat_eq(a1003, a1004) {
                    let m1009 = f122(v0.clone(), {
                        let a1005 = v1;
                        let a1006 = 1u64;
                        nat_add(a1005, a1006)?
                    }, {
                        let a1007 = v6;
                        let a1008 = 1u64;
                        nat_add(a1007, a1008)?
                    }, v8, 1u64, true, v6, List::<Bytes>::nil())?;
                    match m1009 {
                        Ok(v20) => {
                            let v21: u64 = f119(v0.clone(), {
                                let (_, h1010) = v20;
                                h1010
                            })?;
                            let v22: u64 = {
                                let a1011 = f185(v0.clone(), v21)?;
                                let a1012 = v21;
                                nat_sub(a1011, a1012)
                            };
                            let a1014 = {
                                let a1013 = v0.clone();
                                length_bytes(a1013)
                            };
                            let a1015 = v21;
                            if nat_le(a1014, a1015) {
                                Ok(Ok::<(Adt2, u64), Bytes>((Adt2::C7(std::rc::Rc::new({
                                    let (h1016, _) = v20;
                                    h1016
                                }), std::rc::Rc::new(Adt2::C6)), v21)))
                            } else {
                                let a1017 = v22;
                                let a1018 = v6;
                                if if nat_eq(a1017, a1018) {
                                    f187(v0.clone(), {
                                        let a1019 = v21;
                                        let a1020 = v22;
                                        nat_add(a1019, a1020)?
                                    })?
                                } else {
                                    false
                                } {
                                    let m1023 = f122(v0, {
                                        let a1021 = v21;
                                        let a1022 = v22;
                                        nat_add(a1021, a1022)?
                                    }, v2, v8, 2u64, false, v6, List::<Bytes>::nil())?;
                                    match m1023 {
                                        Ok(v24) => {
                                            Ok(Ok::<(Adt2, u64), Bytes>((Adt2::C7(std::rc::Rc::new({
                                                let (h1024, _) = v20;
                                                h1024
                                            }), std::rc::Rc::new({
                                                let (h1025, _) = v24;
                                                h1025
                                            })), {
                                                let (_, h1026) = v24;
                                                h1026
                                            })))
                                        }
                                        Err(v23) => {
                                            Ok(Err::<(Adt2, u64), Bytes>(v23))
                                        }
                                    }
                                } else {
                                    let a1027 = v6;
                                    let a1028 = v22;
                                    if nat_lt(a1027, a1028) {
                                        Ok(Err::<(Adt2, u64), Bytes>(f102(Str::lit("a line is indented more than its block sequence entry allows (YAML 1.2.2 [183])"), {
                                            let a1029 = v21;
                                            let a1030 = v22;
                                            nat_add(a1029, a1030)?
                                        })?))
                                    } else {
                                        Ok(Ok::<(Adt2, u64), Bytes>((Adt2::C7(std::rc::Rc::new({
                                            let (h1031, _) = v20;
                                            h1031
                                        }), std::rc::Rc::new(Adt2::C6)), v21)))
                                    }
                                }
                            }
                        }
                        Err(v19) => {
                            Ok(Err::<(Adt2, u64), Bytes>(v19))
                        }
                    }
                } else {
                    let m1032 = f189(v0.clone(), v1)?;
                    match m1032 {
                        None => {
                            let m1033 = f195(v0.clone(), v1)?;
                            match m1033 {
                                None => {
                                    Ok(Err::<(Adt2, u64), Bytes>(f102(Str::lit("a block mapping entry must be an implicit key and : (YAML 1.2.2 [188])"), v1)?))
                                }
                                Some(v26) => {
                                    let v27: Bytes = f196(v0.clone(), v1)?;
                                    if f197(v27.clone(), v7.clone()) {
                                        Ok(Err::<(Adt2, u64), Bytes>(f102(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                    } else {
                                        let m1038 = f122(v0.clone(), {
                                            let a1034 = v26;
                                            let a1035 = 1u64;
                                            nat_add(a1034, a1035)?
                                        }, {
                                            let a1036 = v6;
                                            let a1037 = 1u64;
                                            nat_add(a1036, a1037)?
                                        }, v8, 1u64, false, v6, List::<Bytes>::nil())?;
                                        match m1038 {
                                            Ok(v29) => {
                                                let v30: u64 = f119(v0.clone(), {
                                                    let (_, h1039) = v29;
                                                    h1039
                                                })?;
                                                let v31: u64 = {
                                                    let a1040 = f185(v0.clone(), v30)?;
                                                    let a1041 = v30;
                                                    nat_sub(a1040, a1041)
                                                };
                                                let a1043 = {
                                                    let a1042 = v0.clone();
                                                    length_bytes(a1042)
                                                };
                                                let a1044 = v30;
                                                if nat_le(a1043, a1044) {
                                                    Ok(Ok::<(Adt2, u64), Bytes>((Adt2::C8(v27, std::rc::Rc::new({
                                                        let (h1045, _) = v29;
                                                        h1045
                                                    }), std::rc::Rc::new(Adt2::C6)), v30)))
                                                } else {
                                                    let a1046 = v31;
                                                    let a1047 = v6;
                                                    if if if nat_eq(a1046, a1047) {
                                                        f194(f187(v0.clone(), {
                                                            let a1048 = v30;
                                                            let a1049 = v31;
                                                            nat_add(a1048, a1049)?
                                                        })?)
                                                    } else {
                                                        false
                                                    } {
                                                        f194(f120(v0.clone(), v30)?)
                                                    } else {
                                                        false
                                                    } {
                                                        let m1052 = f122(v0, {
                                                            let a1050 = v30;
                                                            let a1051 = v31;
                                                            nat_add(a1050, a1051)?
                                                        }, v2, v8, 3u64, false, v6, List::cons(v27.clone(), v7))?;
                                                        match m1052 {
                                                            Ok(v33) => {
                                                                Ok(Ok::<(Adt2, u64), Bytes>((Adt2::C8(v27, std::rc::Rc::new({
                                                                    let (h1053, _) = v29;
                                                                    h1053
                                                                }), std::rc::Rc::new({
                                                                    let (h1054, _) = v33;
                                                                    h1054
                                                                })), {
                                                                    let (_, h1055) = v33;
                                                                    h1055
                                                                })))
                                                            }
                                                            Err(v32) => {
                                                                Ok(Err::<(Adt2, u64), Bytes>(v32))
                                                            }
                                                        }
                                                    } else {
                                                        let a1056 = v6;
                                                        let a1057 = v31;
                                                        if nat_lt(a1056, a1057) {
                                                            Ok(Err::<(Adt2, u64), Bytes>(f102(Str::lit("a line is indented more than its block mapping entry allows (YAML 1.2.2 [187])"), {
                                                                let a1058 = v30;
                                                                let a1059 = v31;
                                                                nat_add(a1058, a1059)?
                                                            })?))
                                                        } else {
                                                            Ok(Ok::<(Adt2, u64), Bytes>((Adt2::C8(v27, std::rc::Rc::new({
                                                                let (h1060, _) = v29;
                                                                h1060
                                                            }), std::rc::Rc::new(Adt2::C6)), v30)))
                                                        }
                                                    }
                                                }
                                            }
                                            Err(v28) => {
                                                Ok(Err::<(Adt2, u64), Bytes>(v28))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v25) => {
                            Ok(Err::<(Adt2, u64), Bytes>(v25))
                        }
                    }
                }
            }
        }
    }
}

pub fn f123(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f119(v0.clone(), v1)?;
    let v3: u64 = {
        let a1061 = v0.clone();
        length_bytes(a1061)
    };
    let a1062 = v2;
    let a1063 = v3;
    if if nat_lt(a1062, a1063) {
        if f105(v0.clone(), v2, Str::lit("...")) {
            f120(v0.clone(), v2)?
        } else {
            false
        }
    } else {
        false
    } {
        let m1066 = f198(v0.clone(), {
            let a1064 = v2;
            let a1065 = 3u64;
            nat_add(a1064, a1065)?
        })?;
        match m1066 {
            Ok(v5) => {
                let v6: u64 = f119(v0, v5)?;
                let a1067 = v6;
                let a1068 = v3;
                if nat_lt(a1067, a1068) {
                    Ok(Err::<u64, Bytes>(f102(Str::lit("a stream of more than one document is not read (YAML 1.2.2 [211])"), v6)?))
                } else {
                    Ok(Ok::<u64, Bytes>(v6))
                }
            }
            Err(v4) => {
                Ok(Err::<u64, Bytes>(v4))
            }
        }
    } else {
        let a1069 = v2;
        let a1070 = v3;
        if if nat_lt(a1069, a1070) {
            f120(v0, v2)?
        } else {
            false
        } {
            Ok(Err::<u64, Bytes>(f102(Str::lit("a stream of more than one document is not read (YAML 1.2.2 [211])"), v2)?))
        } else {
            let a1071 = v2;
            let a1072 = v3;
            if nat_lt(a1071, a1072) {
                Ok(Err::<u64, Bytes>(f102(Str::lit("only comments may follow the root node (YAML 1.2.2 [207])"), v2)?))
            } else {
                Ok(Ok::<u64, Bytes>(v2))
            }
        }
    }
}

pub fn f124(v0: Adt2) -> R<Bytes> {
    let m1073 = v0;
    match m1073 {
        Adt2::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt2::C1(v2, v3) => {
            f199(v2, v3)
        }
        Adt2::C2(v4, v5, v6) => {
            f200(v4, v5, v6)
        }
        Adt2::C3(v7) => {
            f201(v7)
        }
        Adt2::C4(r1074) => {
            let v8 = (*r1074).clone();
            Ok(f202(f203(v8)?))
        }
        Adt2::C5(r1075) => {
            let v9 = (*r1075).clone();
            f204(f205(v9)?)
        }
        Adt2::C6 => {
            Ok(Bytes::lit(&[]))
        }
        Adt2::C7(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt2::C8(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f125(v0: Bytes, v1: u64, v2: u64, v3: Bytes) -> Bytes {
    let a1078 = v3;
    let a1079 = f38(v0, v1, {
        let a1076 = v2;
        let a1077 = v1;
        nat_sub(a1076, a1077)
    });
    append_bytes(a1078, a1079)
}

pub fn f126(v0: Bytes, p1092: u64, p1093: u64) -> R<u64> {
    let m1091 = (p1092, p1093);
    {
        let mut m1091 = m1091;
        loop {
            let step = {
                let (v1, v2) = m1091;
                let m1080 = v2;
                if m1080 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m1080 - 1;
                    let v4: u64 = f75(v0.clone(), v1)?;
                    let a1081 = v4;
                    let a1082 = 34u64;
                    if nat_eq(a1081, a1082) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a1083 = v4;
                        let a1084 = 92u64;
                        if nat_eq(a1083, a1084) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a1085 = v4;
                            let a1086 = 32u64;
                            if nat_lt(a1085, a1086) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a1087 = v4;
                                let a1088 = 256u64;
                                if nat_eq(a1087, a1088) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a1089 = v1;
                                        let a1090 = 1u64;
                                        nat_add(a1089, a1090)?
                                    }, v3))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1091 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f127(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: ((u64, (Bytes, u64)), bool) = f206(Fn5::F207(v0.clone()), {
        let a1095 = {
            let a1094 = v0;
            length_bytes(a1094)
        };
        let a1096 = 1u64;
        nat_add(a1095, a1096)?
    }, (v1, (Bytes::lit(&[]), 0u64)))?;
    let a1100 = {
        let (_, h1097) = {
            let (_, h1098) = {
                let (h1099, _) = v2.clone();
                h1099
            };
            h1098
        };
        h1097
    };
    let a1101 = 1u64;
    if nat_eq(a1100, a1101) {
        Ok(Some(({
            let (h1102, _) = {
                let (h1103, _) = v2.clone();
                h1103
            };
            h1102
        }, {
            let (h1104, _) = {
                let (_, h1105) = {
                    let (h1106, _) = v2;
                    h1106
                };
                h1105
            };
            h1104
        })))
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f128(v0: Bytes, v1: u64) -> R<u64> {
    let (h1107, _) = f154(Fn4::F208(v0.clone()), {
        let a1109 = {
            let a1108 = v0;
            length_bytes(a1108)
        };
        let a1110 = 1u64;
        nat_add(a1109, a1110)?
    }, v1)?;
    Ok(h1107)
}

pub fn f129(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f130(v0: Bytes) -> R<u64> {
    let (h1111, _) = f154(Fn4::F209(v0.clone()), {
        let a1113 = {
            let a1112 = v0;
            length_bytes(a1112)
        };
        let a1114 = 1u64;
        nat_add(a1113, a1114)?
    }, 0u64)?;
    Ok(h1111)
}

pub fn f131(v0: Bytes) -> R<Option<u64>> {
    if f210(v0.clone())? {
        Ok(Some({
            let (_, h1115) = {
                let (h1116, _) = f211(Fn6::F212(v0.clone()), {
                    let a1117 = v0;
                    length_bytes(a1117)
                }, (0u64, 0u64))?;
                h1116
            };
            h1115
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f132(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f38(v1.clone(), 0u64, {
        let a1119 = {
            let a1118 = v1.clone();
            length_bytes(a1118)
        };
        let a1120 = f213(v1)?;
        nat_sub(a1119, a1120)
    });
    let v5: Bytes = {
        let a1121 = v0;
        let a1122 = v4.clone();
        append_bytes(a1121, a1122)
    };
    let v6: u64 = f130(v5.clone())?;
    let v7: Bytes = f38(v5.clone(), v6, {
        let a1124 = {
            let a1123 = v5;
            length_bytes(a1123)
        };
        let a1125 = v6;
        nat_sub(a1124, a1125)
    });
    let v8: u64 = {
        let a1126 = v4;
        length_bytes(a1126)
    };
    let v9: u64 = {
        let a1127 = v7.clone();
        length_bytes(a1127)
    };
    let a1128 = v9;
    let a1129 = 0u64;
    if nat_eq(a1128, a1129) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f129(v2, {
            let a1130 = v3;
            let a1131 = v8;
            nat_lt(a1130, a1131)
        }) {
            let v10: u64 = if v2 {
                let a1132 = v3;
                let a1133 = v8;
                nat_add(a1132, a1133)?
            } else {
                let a1134 = v8;
                let a1135 = v3;
                nat_sub(a1134, a1135)
            };
            let v11: u64 = f213(v7.clone())?;
            let a1136 = v10;
            let a1137 = v11;
            if nat_le(a1136, a1137) {
                Ok(Some((f38(v7, 0u64, {
                    let a1138 = v9;
                    let a1139 = v10;
                    nat_sub(a1138, a1139)
                }), 0u64)))
            } else {
                let a1142 = {
                    let a1140 = v10;
                    let a1141 = v11;
                    nat_sub(a1140, a1141)
                };
                let a1143 = 19u64;
                if nat_le(a1142, a1143) {
                    Ok(Some((f38(v7, 0u64, {
                        let a1144 = v9;
                        let a1145 = v11;
                        nat_sub(a1144, a1145)
                    }), {
                        let a1146 = v10;
                        let a1147 = v11;
                        nat_sub(a1146, a1147)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a1148 = v3;
                let a1149 = v8;
                nat_sub(a1148, a1149)
            };
            let a1152 = {
                let a1150 = v9;
                let a1151 = v12;
                nat_add(a1150, a1151)?
            };
            let a1153 = 19u64;
            if nat_le(a1152, a1153) {
                Ok(Some(({
                    let a1154 = v7;
                    let a1155 = f214(v12);
                    append_bytes(a1154, a1155)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f133(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1156 = 0u64;
        let a1157 = v1;
        if nat_lt(a1156, a1157) {
            let a1158 = v1;
            let a1159 = 9223372036854775808u64;
            nat_le(a1158, a1159)
        } else {
            false
        }
    } else {
        let a1160 = v1;
        let a1161 = 9223372036854775807u64;
        nat_le(a1160, a1161)
    }
}

pub fn f134(v0: Bytes, v1: u64) -> Option<u8> {
    let a1162 = v0;
    let a1163 = v1;
    index_bytes(a1162, a1163)
}

pub fn f135(v0: Bytes) -> R<Result<List<(u64, Bytes)>, Bytes>> {
    f215({
        let a1165 = {
            let a1164 = v0.clone();
            length_bytes(a1164)
        };
        let a1166 = 1u64;
        nat_add(a1165, a1166)?
    }, v0, 0u64, List::<(u64, Bytes)>::nil())
}

pub fn f136(p1174: u64, p1175: List<(u64, Bytes)>, p1176: List<Adt13>) -> R<Result<List<Adt13>, Bytes>> {
    let m1173 = (p1174, (p1175, p1176));
    {
        let mut m1173 = m1173;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1173;
                let m1167 = v0;
                if m1167 == 0 {
                    Err::<(u64, (List<(u64, Bytes)>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>(Ok(Err::<List<Adt13>, Bytes>(f8(Str::lit("the input has more definitions than tokens")))))
                } else {
                    let v3 = m1167 - 1;
                    let m1168 = v1.clone();
                    match m1168.uncons() {
                        None => {
                            Err::<(u64, (List<(u64, Bytes)>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>(Ok(Ok::<List<Adt13>, Bytes>(f216(v2, List::<Adt13>::nil()))))
                        }
                        Some((_, _)) => {
                            let m1169 = f217(v1)?;
                            match m1169 {
                                Ok(v7) => {
                                    let m1170 = f218(v3, v7)?;
                                    match m1170 {
                                        Ok(v9) => {
                                            Ok::<(u64, (List<(u64, Bytes)>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>((v3, ({
                                                let (_, h1171) = v9;
                                                h1171
                                            }, List::cons({
                                                let (h1172, _) = v9;
                                                h1172
                                            }, v2))))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (List<(u64, Bytes)>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>(Ok(Err::<List<Adt13>, Bytes>(v8)))
                                        }
                                    }
                                }
                                Err(v6) => {
                                    Err::<(u64, (List<(u64, Bytes)>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>(Ok(Err::<List<Adt13>, Bytes>(v6)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1173 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f137(v0: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>, v1: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>) -> List<(Bytes, (Bytes, (Bytes, List<Bytes>)))> {
    let m1177 = v0;
    match m1177.uncons() {
        None => {
            List::<(Bytes, (Bytes, (Bytes, List<Bytes>)))>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = if f41({
                let (h1178, _) = {
                    let (_, h1179) = v2.clone();
                    h1179
                };
                h1178
            }, Bytes::lit(&[])) {
                f219({
                    let (_, h1180) = {
                        let (_, h1181) = {
                            let (_, h1182) = v2.clone();
                            h1182
                        };
                        h1181
                    };
                    h1180
                }, v1.clone())
            } else {
                let (h1183, _) = {
                    let (_, h1184) = v2.clone();
                    h1184
                };
                h1183
            };
            List::cons(({
                let (h1185, _) = v2;
                h1185
            }, (v4, {
                let (_, h1186) = {
                    let (_, h1187) = v2;
                    h1187
                };
                h1186
            })), f137(v3, v1))
        }
    }
}

pub fn f138(v0: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>) -> R<u64> {
    let m1188 = v0;
    match m1188.uncons() {
        None => {
            Ok(0u64)
        }
        Some((v1, v2)) => {
            if f41({
                let (h1189, _) = {
                    let (_, h1190) = v1;
                    h1190
                };
                h1189
            }, Bytes::lit(&[])) {
                f138(v2)
            } else {
                let a1191 = f138(v2)?;
                let a1192 = 1u64;
                nat_add(a1191, a1192)
            }
        }
    }
}

pub fn f139(v0: u64) -> bool {
    let a1193 = v0;
    let a1194 = 0u64;
    if nat_eq(a1193, a1194) {
        true
    } else {
        let a1195 = v0;
        let a1196 = 2u64;
        if nat_eq(a1195, a1196) {
            true
        } else {
            let a1197 = v0;
            let a1198 = 3u64;
            if nat_eq(a1197, a1198) {
                true
            } else {
                let a1199 = v0;
                let a1200 = 4u64;
                if nat_eq(a1199, a1200) {
                    true
                } else {
                    let a1201 = v0;
                    let a1202 = 9u64;
                    nat_eq(a1201, a1202)
                }
            }
        }
    }
}

pub fn f140(v0: Adt13) -> u64 {
    let m1203 = v0;
    match m1203 {
        Adt13::C0(v1, _, _, _, _, _) => {
            v1
        }
    }
}

pub fn f141(v0: List<Adt13>, v1: Adt13) -> R<Result<(Bytes, List<Bytes>), Bytes>> {
    let v2: u64 = f140(v1.clone());
    let v3: Bytes = f177(f142(v1.clone()))?;
    if if f220(v3.clone()) {
        f139(v2)
    } else {
        false
    } {
        Ok(Err::<(Bytes, List<Bytes>), Bytes>(f221(v3)))
    } else {
        let a1204 = v2;
        let a1205 = 0u64;
        if if nat_eq(a1204, a1205) {
            true
        } else {
            let a1206 = v2;
            let a1207 = 2u64;
            nat_eq(a1206, a1207)
        } {
            let m1208 = f222(v0.clone(), v0.clone(), f142(v1.clone()));
            match m1208 {
                None => {
                    let m1212 = f223({
                        let a1210 = {
                            let a1209 = v0.clone();
                            length_list(a1209)
                        };
                        let a1211 = 1u64;
                        nat_add(a1210, a1211)?
                    }, v0, v1)?;
                    match m1212 {
                        Ok(v5) => {
                            let a1214 = {
                                let a1213 = v5.clone();
                                length_list(a1213)
                            };
                            let a1215 = 0u64;
                            if nat_eq(a1214, a1215) {
                                Ok(Err::<(Bytes, List<Bytes>), Bytes>(f8(Str::lit("it has no data members, and LexLean refuses a structure with no fields"))))
                            } else {
                                let m1216 = f224(f225(f226(v5.clone()))?, v5)?;
                                match m1216 {
                                    Ok(v7) => {
                                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f227(v3, {
                                            let (h1217, _) = v7;
                                            h1217
                                        }), {
                                            let (_, h1218) = v7;
                                            h1218
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
            let a1219 = v2;
            let a1220 = 3u64;
            if nat_eq(a1219, a1220) {
                let m1221 = f115(f228(v1.clone()));
                match m1221 {
                    None => {
                        let m1222 = f229(f225(f228(v1.clone()))?, f228(v1.clone()))?;
                        match m1222 {
                            None => {
                                let m1223 = f230(f225(f228(v1.clone()))?, f231(f228(v1)), List::<Bytes>::nil())?;
                                match m1223 {
                                    Ok(v10) => {
                                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f232(v3, v10), List::<Bytes>::nil())))
                                    }
                                    Err(v9) => {
                                        Ok(Err::<(Bytes, List<Bytes>), Bytes>(v9))
                                    }
                                }
                            }
                            Some(v11) => {
                                Ok(Err::<(Bytes, List<Bytes>), Bytes>(f221(v11)))
                            }
                        }
                    }
                    Some(v12) => {
                        Ok(Err::<(Bytes, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("the enumeration value \"")), List::cons(v12, List::cons(f8(Str::lit("\" appears twice (Web IDL 2.9: the values must be distinct)")), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                let a1224 = v2;
                let a1225 = 4u64;
                if nat_eq(a1224, a1225) {
                    let v13: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) = f233(v1);
                    if !f41(f234(v13.clone()), Bytes::lit(&[])) {
                        Ok(Err::<(Bytes, List<Bytes>), Bytes>(f234(v13)))
                    } else {
                        let a1226 = f235(v13.clone());
                        let a1227 = 1u64;
                        if nat_eq(a1226, a1227) {
                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f232(v3, List::cons(f236(v13.clone()), List::<Bytes>::nil())), f237(v13))))
                        } else {
                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f238(v3, f236(v13.clone())), f237(v13))))
                        }
                    }
                } else {
                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f7(f228(v1))))
                }
            }
        }
    }
}

pub fn f142(v0: Adt13) -> Bytes {
    let m1228 = v0;
    match m1228 {
        Adt13::C0(_, v2, _, _, _, _) => {
            v2
        }
    }
}

pub fn f143(v0: Bytes, p1231: List<Bytes>) -> bool {
    let m1230 = p1231;
    {
        let mut m1230 = m1230;
        loop {
            let step = {
                let v1 = m1230;
                let m1229 = v1;
                match m1229.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f41(v2, v0.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1230 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f144(p1235: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>, v1: Bytes) -> Option<(Bytes, (Bytes, (Bytes, List<Bytes>)))> {
    let m1234 = p1235;
    {
        let mut m1234 = m1234;
        loop {
            let step = {
                let v0 = m1234;
                let m1232 = v0;
                match m1232.uncons() {
                    None => {
                        Err::<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>, Option<(Bytes, (Bytes, (Bytes, List<Bytes>)))>>(None::<(Bytes, (Bytes, (Bytes, List<Bytes>)))>)
                    }
                    Some((v2, v3)) => {
                        if f41({
                            let (h1233, _) = v2.clone();
                            h1233
                        }, v1.clone()) {
                            Err::<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>, Option<(Bytes, (Bytes, (Bytes, List<Bytes>)))>>(Some(v2))
                        } else {
                            Ok::<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>, Option<(Bytes, (Bytes, (Bytes, List<Bytes>)))>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1234 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f145(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m1236 = v0;
    match m1236.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f145(v3, v1))
        }
    }
}

pub fn f146(v0: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)> {
    let m1237 = v0;
    match m1237.uncons() {
        None => {
            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
        }
        Some((v2, v3)) => {
            let m1240 = f178({
                let (h1238, _) = {
                    let (_, h1239) = v2.clone();
                    h1239
                };
                h1238
            }, v1.clone());
            match m1240 {
                None => {
                    Some((({
                        let (h1241, _) = v2;
                        h1241
                    }, {
                        let (_, h1242) = {
                            let (_, h1243) = v2;
                            h1243
                        };
                        h1242
                    }), v3))
                }
                Some(_) => {
                    let m1244 = f146(v3, v1);
                    match m1244 {
                        None => {
                            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h1245, _) = v5;
                                h1245
                            }, List::cons(v2, {
                                let (_, h1246) = v5;
                                h1246
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f147(p1251: List<Adt13>, v1: u64, v2: Bytes) -> Option<Adt13> {
    let m1250 = p1251;
    {
        let mut m1250 = m1250;
        loop {
            let step = {
                let v0 = m1250;
                let m1247 = v0;
                match m1247.uncons() {
                    None => {
                        Err::<List<Adt13>, Option<Adt13>>(None::<Adt13>)
                    }
                    Some((v3, v4)) => {
                        let a1248 = f140(v3.clone());
                        let a1249 = v1;
                        if if nat_eq(a1248, a1249) {
                            f41(f142(v3.clone()), v2.clone())
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
                Ok(next) => { m1250 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f148(v0: u64) -> Bytes {
    let a1252 = v0;
    let a1253 = 5u64;
    if nat_eq(a1252, a1253) {
        f8(Str::lit("partial interface "))
    } else {
        let a1254 = v0;
        let a1255 = 7u64;
        if nat_eq(a1254, a1255) {
            f8(Str::lit("partial dictionary "))
        } else {
            f8(Str::lit(""))
        }
    }
}

pub fn f149(v0: Adt13) -> Bytes {
    let m1256 = v0;
    match m1256 {
        Adt13::C0(_, _, v3, _, _, _) => {
            v3
        }
    }
}

pub fn f150(v0: Bytes, v1: u64) -> Bytes {
    let m1260 = {
        let a1257 = v0;
        let a1258 = v1;
        let a1259 = 1u64;
        slice_bytes(a1257, a1258, a1259)
    };
    match m1260 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f151(v0: Fn3, p1265: u64, p1266: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1264 = (p1265, p1266);
    {
        let mut m1264 = m1264;
        loop {
            let step = {
                let (v1, v2) = m1264;
                let m1261 = v1;
                if m1261 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1261 - 1;
                    let m1263 = {
                        let c1262: Fn3 = v0.clone();
                        c1262.apply(v2.clone())?
                    };
                    match m1263 {
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
                Ok(next) => { m1264 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f152(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f239(v1, v0, v2)
}

pub fn f153(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f240(v0, v1)
}

pub fn f154(v0: Fn4, p1271: u64, p1272: u64) -> R<(u64, bool)> {
    let m1270 = (p1271, p1272);
    {
        let mut m1270 = m1270;
        loop {
            let step = {
                let (v1, v2) = m1270;
                let m1267 = v1;
                if m1267 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1267 - 1;
                    let m1269 = {
                        let c1268: Fn4 = v0.clone();
                        c1268.apply(v2)?
                    };
                    match m1269 {
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
                Ok(next) => { m1270 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f155(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f241(v0, v1)
}

pub fn f156(v0: u64) -> R<bool> {
    f242(v0, f243())
}

pub fn f157(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let v3: u64 = {
        let (h1273, _) = f154(Fn4::F244(v1.clone(), v0.clone()), {
            let a1275 = {
                let a1274 = v0.clone();
                length_bytes(a1274)
            };
            let a1276 = 1u64;
            nat_add(a1275, a1276)?
        }, v2)?;
        h1273
    };
    if f98(v0, v1, v3) {
        Ok(Some(v3))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f158(v0: Bytes) -> Result<u64, Bytes> {
    Err::<u64, Bytes>(v0)
}

pub fn f159(v0: Bytes, v1: u64) -> R<u64> {
    let (h1277, _) = f154(Fn4::F245(v0.clone()), {
        let a1279 = {
            let a1278 = v0;
            length_bytes(a1278)
        };
        let a1280 = 1u64;
        nat_add(a1279, a1280)?
    }, v1)?;
    Ok(h1277)
}

pub fn f160(v0: Bytes) -> bool {
    f197(v0, List::cons(f8(Str::lit("UTF-8")), List::cons(f8(Str::lit("UTf-8")), List::cons(f8(Str::lit("UtF-8")), List::cons(f8(Str::lit("Utf-8")), List::cons(f8(Str::lit("uTF-8")), List::cons(f8(Str::lit("uTf-8")), List::cons(f8(Str::lit("utF-8")), List::cons(f8(Str::lit("utf-8")), List::<Bytes>::nil())))))))))
}

pub fn f161(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let m1284 = f157(v0.clone(), {
        let a1281 = Str::lit("--");
        utf8_encode(a1281)
    }, {
        let a1282 = v1;
        let a1283 = 4u64;
        nat_add(a1282, a1283)?
    })?;
    match m1284 {
        None => {
            Ok(f158(f102(Str::lit("a comment is not closed (XML 1.0 [15])"), v1)?))
        }
        Some(v2) => {
            let a1287 = f106(v0, {
                let a1285 = v2;
                let a1286 = 2u64;
                nat_add(a1285, a1286)?
            })?;
            let a1288 = 62u64;
            if nat_eq(a1287, a1288) {
                Ok(Ok::<u64, Bytes>({
                    let a1289 = v2;
                    let a1290 = 3u64;
                    nat_add(a1289, a1290)?
                }))
            } else {
                Ok(f158(f102(Str::lit("-- is not allowed inside a comment (XML 1.0 [15])"), v2)?))
            }
        }
    }
}

pub fn f162(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f164(v0.clone(), {
        let a1291 = v1;
        let a1292 = 2u64;
        nat_add(a1291, a1292)?
    })?;
    let a1295 = v2;
    let a1296 = {
        let a1293 = v1;
        let a1294 = 2u64;
        nat_add(a1293, a1294)?
    };
    if nat_eq(a1295, a1296) {
        Ok(f158(f102(Str::lit("a processing instruction must start with a Name (XML 1.0 [16])"), v1)?))
    } else {
        let a1299 = v2;
        let a1300 = {
            let a1297 = v1;
            let a1298 = 5u64;
            nat_add(a1297, a1298)?
        };
        if if nat_eq(a1299, a1300) {
            f197(f38(v0.clone(), {
                let a1301 = v1;
                let a1302 = 2u64;
                nat_add(a1301, a1302)?
            }, 3u64), List::cons(f8(Str::lit("XML")), List::cons(f8(Str::lit("XMl")), List::cons(f8(Str::lit("XmL")), List::cons(f8(Str::lit("Xml")), List::cons(f8(Str::lit("xML")), List::cons(f8(Str::lit("xMl")), List::cons(f8(Str::lit("xmL")), List::cons(f8(Str::lit("xml")), List::<Bytes>::nil())))))))))
        } else {
            false
        } {
            Ok(f158(f102(Str::lit("the target xml is reserved; an XML declaration may appear only at the very start (XML 1.0 [17], [22])"), v1)?))
        } else {
            let m1304 = f157(v0, {
                let a1303 = Str::lit("?>");
                utf8_encode(a1303)
            }, v2)?;
            match m1304 {
                None => {
                    Ok(f158(f102(Str::lit("a processing instruction is not closed (XML 1.0 [16])"), v1)?))
                }
                Some(v3) => {
                    Ok(Ok::<u64, Bytes>({
                        let a1305 = v3;
                        let a1306 = 2u64;
                        nat_add(a1305, a1306)?
                    }))
                }
            }
        }
    }
}

pub fn f163(v0: Bytes) -> Result<(Adt14, u64), Bytes> {
    Err::<(Adt14, u64), Bytes>(v0)
}

pub fn f164(v0: Bytes, v1: u64) -> R<u64> {
    let (h1307, _) = f154(Fn4::F246(v0.clone(), v1), {
        let a1309 = {
            let a1308 = v0;
            length_bytes(a1308)
        };
        let a1310 = 1u64;
        nat_add(a1309, a1310)?
    }, v1)?;
    Ok(h1307)
}

pub fn f165(v0: Bytes, v1: u64, v2: u64, v3: List<Bytes>) -> R<Result<(Adt14, u64), Bytes>> {
    let m1311 = v2;
    if m1311 == 0 {
        Ok(f163(f102(Str::lit("a tag has more attributes than the input has bytes"), v1)?))
    } else {
        let v4 = m1311 - 1;
        let v5: u64 = f159(v0.clone(), v1)?;
        let a1312 = f106(v0.clone(), v5)?;
        let a1313 = 62u64;
        if if nat_eq(a1312, a1313) {
            true
        } else {
            f105(v0.clone(), v5, Str::lit("/>"))
        } {
            Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C4, v5)))
        } else {
            let a1314 = v5;
            let a1315 = v1;
            if nat_eq(a1314, a1315) {
                Ok(f163(f102(Str::lit("attributes must be separated by white space (XML 1.0 [40])"), v1)?))
            } else {
                let v6: u64 = f164(v0.clone(), v5)?;
                let a1316 = v6;
                let a1317 = v5;
                if nat_eq(a1316, a1317) {
                    Ok(f163(f102(Str::lit("an attribute must start with a Name (XML 1.0 [41])"), v5)?))
                } else {
                    let v7: Bytes = f38(v0.clone(), v5, {
                        let a1318 = v6;
                        let a1319 = v5;
                        nat_sub(a1318, a1319)
                    });
                    if f197(v7.clone(), v3.clone()) {
                        Ok(f163(f102(Str::lit("an attribute name appears twice in one tag (XML 1.0 WFC: Unique Att Spec)"), v5)?))
                    } else {
                        let v8: u64 = f159(v0.clone(), v6)?;
                        let a1320 = f106(v0.clone(), v8)?;
                        let a1321 = 61u64;
                        if nat_eq(a1320, a1321) {
                            let v9: u64 = f159(v0.clone(), {
                                let a1322 = v8;
                                let a1323 = 1u64;
                                nat_add(a1322, a1323)?
                            })?;
                            let v10: u64 = f106(v0.clone(), v9)?;
                            let a1324 = v10;
                            let a1325 = 34u64;
                            if if nat_eq(a1324, a1325) {
                                true
                            } else {
                                let a1326 = v10;
                                let a1327 = 39u64;
                                nat_eq(a1326, a1327)
                            } {
                                let m1330 = f166(v0.clone(), {
                                    let a1328 = v9;
                                    let a1329 = 1u64;
                                    nat_add(a1328, a1329)?
                                }, v4, Bytes::lit(&[]), true, v10)?;
                                match m1330 {
                                    Ok(v12) => {
                                        let m1332 = f165(v0, {
                                            let (_, h1331) = v12;
                                            h1331
                                        }, v4, List::cons(v7.clone(), v3))?;
                                        match m1332 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C1(v7, {
                                                    let (h1333, _) = v12;
                                                    h1333
                                                }, std::rc::Rc::new({
                                                    let (h1334, _) = v14;
                                                    h1334
                                                })), {
                                                    let (_, h1335) = v14;
                                                    h1335
                                                })))
                                            }
                                            Err(v13) => {
                                                Ok(f163(v13))
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(f163(v11))
                                    }
                                }
                            } else {
                                Ok(f163(f102(Str::lit("an attribute value must be quoted (XML 1.0 [10])"), v9)?))
                            }
                        } else {
                            Ok(f163(f102(Str::lit("an attribute needs = and a value (XML 1.0 [25], [41])"), v8)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f166(v0: Bytes, p1375: u64, p1376: u64, p1377: Bytes, v4: bool, v5: u64) -> R<Result<(Bytes, u64), Bytes>> {
    let m1374 = (p1375, (p1376, p1377));
    {
        let mut m1374 = m1374;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1374;
                let m1336 = v2;
                if m1336 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f247(f102(Str::lit("the input ends inside a text run"), v1)?)))
                } else {
                    let v6 = m1336 - 1;
                    let v7: u64 = f106(v0.clone(), v1)?;
                    if if v4 {
                        let a1337 = v7;
                        let a1338 = v5;
                        nat_eq(a1337, a1338)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, {
                            let a1339 = v1;
                            let a1340 = 1u64;
                            nat_add(a1339, a1340)?
                        }))))
                    } else {
                        let a1341 = v7;
                        let a1342 = 256u64;
                        if nat_eq(a1341, a1342) {
                            if v4 {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f247(f102(Str::lit("an attribute value is not closed (XML 1.0 [10])"), v1)?)))
                            } else {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                            }
                        } else {
                            let a1343 = v7;
                            let a1344 = 60u64;
                            if nat_eq(a1343, a1344) {
                                if v4 {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f247(f102(Str::lit("< is not allowed in an attribute value (XML 1.0 [10], WFC: No < in Attribute Values)"), v1)?)))
                                } else {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                                }
                            } else {
                                let a1345 = v7;
                                let a1346 = 38u64;
                                if nat_eq(a1345, a1346) {
                                    let m1347 = f248(v0.clone(), v1)?;
                                    match m1347 {
                                        Ok(v9) => {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let (_, h1348) = v9;
                                                h1348
                                            }, (v6, {
                                                let a1350 = v3;
                                                let a1351 = {
                                                    let (h1349, _) = v9;
                                                    h1349
                                                };
                                                append_bytes(a1350, a1351)
                                            })))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f247(v8)))
                                        }
                                    }
                                } else {
                                    if v4 {
                                        let a1352 = v7;
                                        let a1353 = 9u64;
                                        if if nat_eq(a1352, a1353) {
                                            true
                                        } else {
                                            let a1354 = v7;
                                            let a1355 = 10u64;
                                            nat_eq(a1354, a1355)
                                        } {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let a1356 = v1;
                                                let a1357 = 1u64;
                                                nat_add(a1356, a1357)?
                                            }, (v6, {
                                                let a1358 = v3;
                                                let a1359 = f8(Str::lit(" "));
                                                append_bytes(a1358, a1359)
                                            })))
                                        } else {
                                            let v10: u64 = f249(v0.clone(), true, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v10, (v6, {
                                                let a1362 = v3;
                                                let a1363 = f38(v0.clone(), v1, {
                                                    let a1360 = v10;
                                                    let a1361 = v1;
                                                    nat_sub(a1360, a1361)
                                                });
                                                append_bytes(a1362, a1363)
                                            })))
                                        }
                                    } else {
                                        let a1364 = v7;
                                        let a1365 = 93u64;
                                        if nat_eq(a1364, a1365) {
                                            if f105(v0.clone(), v1, Str::lit("]]>")) {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f247(f102(Str::lit("]]> is not allowed in character data (XML 1.0 [14])"), v1)?)))
                                            } else {
                                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                    let a1366 = v1;
                                                    let a1367 = 1u64;
                                                    nat_add(a1366, a1367)?
                                                }, (v6, {
                                                    let a1368 = v3;
                                                    let a1369 = f8(Str::lit("]"));
                                                    append_bytes(a1368, a1369)
                                                })))
                                            }
                                        } else {
                                            let v11: u64 = f249(v0.clone(), false, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v11, (v6, {
                                                let a1372 = v3;
                                                let a1373 = f38(v0.clone(), v1, {
                                                    let a1370 = v11;
                                                    let a1371 = v1;
                                                    nat_sub(a1370, a1371)
                                                });
                                                append_bytes(a1372, a1373)
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
                Ok(next) => { m1374 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f167(v0: Bytes) -> Result<List<(Bytes, Bytes)>, Bytes> {
    Err::<List<(Bytes, Bytes)>, Bytes>(v0)
}

pub fn f168() -> Bytes {
    f8(Str::lit("http://www.w3.org/XML/1998/namespace"))
}

pub fn f169() -> Bytes {
    f8(Str::lit("http://www.w3.org/2000/xmlns/"))
}

pub fn f170(v0: Bytes, v1: Bytes) -> bool {
    let m1382 = {
        let a1379 = v0;
        let a1380 = 0u64;
        let a1381 = {
            let a1378 = v1.clone();
            length_bytes(a1378)
        };
        slice_bytes(a1379, a1380, a1381)
    };
    match m1382 {
        None => {
            false
        }
        Some(v2) => {
            let a1383 = v2;
            let a1384 = v1;
            equal(a1383, a1384)
        }
    }
}

pub fn f171(v0: Bytes) -> R<Bytes> {
    let m1385 = f157(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1385 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v1) => {
            Ok(f38(v0, 0u64, v1))
        }
    }
}

pub fn f172(p1390: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1389 = p1390;
    {
        let mut m1389 = m1389;
        loop {
            let step = {
                let v0 = m1389;
                let m1386 = v0;
                match m1386.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f41({
                            let (h1387, _) = v2;
                            h1387
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h1388) = v2;
                                h1388
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1389 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f173(v0: Bytes) -> R<Bytes> {
    let m1391 = f157(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1391 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(f38(v0.clone(), {
                let a1392 = v1;
                let a1393 = 1u64;
                nat_add(a1392, a1393)?
            }, {
                let a1397 = {
                    let a1394 = v0;
                    length_bytes(a1394)
                };
                let a1398 = {
                    let a1395 = v1;
                    let a1396 = 1u64;
                    nat_add(a1395, a1396)?
                };
                nat_sub(a1397, a1398)
            }))
        }
    }
}

pub fn f174() -> Bytes {
    f8(Str::lit("http://www.w3.org/2001/XMLSchema"))
}

pub fn f175(v0: Bytes) -> Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes> {
    Err::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v0)
}

pub fn f176(v0: Bytes, v1: Adt14, v2: List<(Bytes, Bytes)>, v3: Bytes) -> R<Result<Adt15, Bytes>> {
    let v4: Bytes = f177(v0.clone())?;
    let m1399 = f250(v1, v2)?;
    match m1399 {
        Ok(v6) => {
            let m1400 = v6;
            match m1400 {
                None => {
                    Ok(f251(f252(f8(Str::lit("a simpleType must have a restriction, list or union child (XSD 1.1 Part 2, section 4.1.2)")), v0)))
                }
                Some(v7) => {
                    let v8: Bytes = match v7.clone() {
                        Adt16::C0(h1401, _, _, _) => {
                            h1401
                        }
                    };
                    let v9: Adt14 = match v7.clone() {
                        Adt16::C0(_, h1402, _, _) => {
                            h1402
                        }
                    };
                    let v10: Adt14 = match v7.clone() {
                        Adt16::C0(_, _, h1403, _) => {
                            h1403
                        }
                    };
                    let v11: List<(Bytes, Bytes)> = match v7 {
                        Adt16::C0(_, _, _, h1404) => {
                            h1404
                        }
                    };
                    if f41(v8.clone(), f8(Str::lit("union"))) {
                        Ok(f251(f252(f8(Str::lit("a union is not translated, because a literal can be valid for more than one member type, so no LexLean inductive keeps its members disjoint (XSD 1.1 Part 2, section 2.4.1.3)")), v0)))
                    } else {
                        if f41(v8, f8(Str::lit("list"))) {
                            let m1405 = f112(v9, f8(Str::lit("itemType")));
                            match m1405 {
                                None => {
                                    Ok(f251(f252(f8(Str::lit("a list with an anonymous item type is not translated yet")), v0)))
                                }
                                Some(v12) => {
                                    let m1406 = f253(v11, v3, v12)?;
                                    match m1406 {
                                        Ok(v14) => {
                                            if f170(match v14.clone() {
                                                Adt15::C0(h1407, _) => {
                                                    h1407
                                                }
                                            }, f8(Str::lit("(List "))) {
                                                Ok(f251(f252(f8(Str::lit("the item type of a list must not itself be a list (XSD 1.1 Part 2, section 4.1.5)")), v0)))
                                            } else {
                                                Ok(Ok::<Adt15, Bytes>(Adt15::C0(f238(v4, f7(List::cons(f8(Str::lit("(List ")), List::cons(match v14.clone() {
                                                    Adt15::C0(h1408, _) => {
                                                        h1408
                                                    }
                                                }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))), match v14 {
                                                    Adt15::C0(_, h1409) => {
                                                        h1409
                                                    }
                                                })))
                                            }
                                        }
                                        Err(v13) => {
                                            Ok(f251(f252(v13, v0)))
                                        }
                                    }
                                }
                            }
                        } else {
                            let m1410 = f112(v9, f8(Str::lit("base")));
                            match m1410 {
                                None => {
                                    Ok(f251(f252(f8(Str::lit("a restriction with an anonymous base type is not translated yet")), v0)))
                                }
                                Some(v15) => {
                                    let m1411 = f253(v11.clone(), v3, v15)?;
                                    match m1411 {
                                        Ok(v17) => {
                                            let m1412 = f254(v10, v11)?;
                                            match m1412 {
                                                Ok(v19) => {
                                                    let v20: List<Adt2> = f255(v19, List::<Bytes>::nil())?;
                                                    let m1413 = v20.clone();
                                                    match m1413.uncons() {
                                                        None => {
                                                            Ok(Ok::<Adt15, Bytes>(Adt15::C0(f238(v4, match v17.clone() {
                                                                Adt15::C0(h1414, _) => {
                                                                    h1414
                                                                }
                                                            }), match v17 {
                                                                Adt15::C0(_, h1415) => {
                                                                    h1415
                                                                }
                                                            })))
                                                        }
                                                        Some((_, _)) => {
                                                            let m1416 = f230(f225(f256(v20.clone()))?, v20, List::<Bytes>::nil())?;
                                                            match m1416 {
                                                                Ok(v24) => {
                                                                    Ok(Ok::<Adt15, Bytes>(Adt15::C0(f232(v4, v24), List::<Bytes>::nil())))
                                                                }
                                                                Err(v23) => {
                                                                    Ok(f251(f252(v23, v0)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Err(v18) => {
                                                    Ok(f251(f252(v18, v0)))
                                                }
                                            }
                                        }
                                        Err(v16) => {
                                            Ok(f251(f252(v16, v0)))
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
            Ok(f251(f252(v5, v0)))
        }
    }
}

pub fn f177(v0: Bytes) -> R<Bytes> {
    f257(v0.clone(), 0u64, {
        let a1417 = v0;
        length_bytes(a1417)
    }, true, Bytes::lit(&[]))
}

pub fn f178(p1420: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m1419 = p1420;
    {
        let mut m1419 = m1419;
        loop {
            let step = {
                let v0 = m1419;
                let m1418 = v0;
                match m1418.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f143(v2.clone(), v1.clone()) {
                            Ok::<List<Bytes>, Option<Bytes>>(v3)
                        } else {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1419 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f179(v0: Bytes) -> Result<List<Bytes>, Bytes> {
    Err::<List<Bytes>, Bytes>(v0)
}

pub fn f180(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f258(v0, v1)
}

pub fn f181(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f259(v0, v1)
}

pub fn f182(v0: u64) -> bool {
    let a1421 = v0;
    let a1422 = 32u64;
    if nat_eq(a1421, a1422) {
        true
    } else {
        let a1423 = v0;
        let a1424 = 9u64;
        nat_eq(a1423, a1424)
    }
}

pub fn f183(v0: u64) -> bool {
    let a1425 = v0;
    let a1426 = 10u64;
    if nat_eq(a1425, a1426) {
        true
    } else {
        let a1427 = v0;
        let a1428 = 256u64;
        nat_eq(a1427, a1428)
    }
}

pub fn f184(v0: Bytes, v1: u64) -> R<u64> {
    let a1429 = v1;
    let a1430 = f260(v0, v1)?;
    Ok(nat_sub(a1429, a1430))
}

pub fn f185(v0: Bytes, v1: u64) -> R<u64> {
    let (h1431, _) = f154(Fn4::F261(v0.clone()), {
        let a1433 = {
            let a1432 = v0;
            length_bytes(a1432)
        };
        let a1434 = 1u64;
        nat_add(a1433, a1434)?
    }, v1)?;
    Ok(h1431)
}

pub fn f186(v0: Bytes, v1: u64) -> R<u64> {
    let (h1435, _) = f154(Fn4::F262(v0.clone()), {
        let a1437 = {
            let a1436 = v0;
            length_bytes(a1436)
        };
        let a1438 = 1u64;
        nat_add(a1437, a1438)?
    }, v1)?;
    Ok(h1435)
}

pub fn f187(v0: Bytes, v1: u64) -> R<bool> {
    let a1439 = f121(v0.clone(), v1)?;
    let a1440 = 45u64;
    if nat_eq(a1439, a1440) {
        if f182(f121(v0.clone(), {
            let a1441 = v1;
            let a1442 = 1u64;
            nat_add(a1441, a1442)?
        })?) {
            Ok(true)
        } else {
            Ok(f183(f121(v0, {
                let a1443 = v1;
                let a1444 = 1u64;
                nat_add(a1443, a1444)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f188(v0: Bytes, v1: u64) -> R<bool> {
    let m1445 = f195(v0, v1)?;
    match m1445 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f189(v0: Bytes, v1: u64) -> R<Option<Bytes>> {
    let v2: u64 = f121(v0.clone(), v1)?;
    let v3: bool = f194(f263(f121(v0, {
        let a1446 = v1;
        let a1447 = 1u64;
        nat_add(a1446, a1447)?
    })?, true));
    let a1448 = v2;
    let a1449 = 58u64;
    if if nat_eq(a1448, a1449) {
        v3
    } else {
        false
    } {
        Ok(Some(f102(Str::lit("a null key is not read: Studio.Value has no null (YAML 1.2.2 section 10.3.2)"), v1)?))
    } else {
        let a1450 = v2;
        let a1451 = 38u64;
        if nat_eq(a1450, a1451) {
            Ok(Some(f102(Str::lit("an anchor is not read (YAML 1.2.2 [101])"), v1)?))
        } else {
            let a1452 = v2;
            let a1453 = 42u64;
            if nat_eq(a1452, a1453) {
                Ok(Some(f102(Str::lit("an alias is not read (YAML 1.2.2 [104])"), v1)?))
            } else {
                let a1454 = v2;
                let a1455 = 33u64;
                if nat_eq(a1454, a1455) {
                    Ok(Some(f102(Str::lit("a tag is not read (YAML 1.2.2 [97])"), v1)?))
                } else {
                    let a1456 = v2;
                    let a1457 = 63u64;
                    if if nat_eq(a1456, a1457) {
                        v3
                    } else {
                        false
                    } {
                        Ok(Some(f102(Str::lit("an explicit key is not read (YAML 1.2.2 [143], [189])"), v1)?))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f190(v0: Bytes, v1: u64, v2: u64) -> R<Result<(Adt2, u64), Bytes>> {
    let v3: u64 = f121(v0.clone(), v1)?;
    let a1458 = v3;
    let a1459 = 124u64;
    if if nat_eq(a1458, a1459) {
        true
    } else {
        let a1460 = v3;
        let a1461 = 62u64;
        nat_eq(a1460, a1461)
    } {
        f264(v0, v1, v2)
    } else {
        let a1462 = v3;
        let a1463 = 34u64;
        if if nat_eq(a1462, a1463) {
            true
        } else {
            let a1464 = v3;
            let a1465 = 39u64;
            nat_eq(a1464, a1465)
        } {
            let m1466 = f265(v0.clone(), v1, v2, false)?;
            match m1466 {
                Ok(v5) => {
                    let m1468 = f198(v0, {
                        let (_, h1467) = v5;
                        h1467
                    })?;
                    match m1468 {
                        Ok(v7) => {
                            Ok(Ok::<(Adt2, u64), Bytes>((Adt2::C3({
                                let (h1469, _) = v5;
                                h1469
                            }), v7)))
                        }
                        Err(v6) => {
                            Ok(Err::<(Adt2, u64), Bytes>(v6))
                        }
                    }
                }
                Err(v4) => {
                    Ok(Err::<(Adt2, u64), Bytes>(v4))
                }
            }
        } else {
            let a1470 = v3;
            let a1471 = 91u64;
            if if nat_eq(a1470, a1471) {
                true
            } else {
                let a1472 = v3;
                let a1473 = 123u64;
                nat_eq(a1472, a1473)
            } {
                let m1477 = f266(v0.clone(), v1, v2, {
                    let a1475 = {
                        let a1474 = v0.clone();
                        length_bytes(a1474)
                    };
                    let a1476 = 2u64;
                    nat_add(a1475, a1476)?
                }, 0u64, List::<Bytes>::nil())?;
                match m1477 {
                    Ok(v9) => {
                        let a1479 = f121(v0.clone(), f186(v0.clone(), {
                            let (_, h1478) = v9.clone();
                            h1478
                        })?)?;
                        let a1480 = 58u64;
                        if nat_eq(a1479, a1480) {
                            Ok(Err::<(Adt2, u64), Bytes>(f102(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [154])"), v1)?))
                        } else {
                            let m1482 = f198(v0, {
                                let (_, h1481) = v9;
                                h1481
                            })?;
                            match m1482 {
                                Ok(v11) => {
                                    Ok(Ok::<(Adt2, u64), Bytes>(({
                                        let (h1483, _) = v9;
                                        h1483
                                    }, v11)))
                                }
                                Err(v10) => {
                                    Ok(Err::<(Adt2, u64), Bytes>(v10))
                                }
                            }
                        }
                    }
                    Err(v8) => {
                        Ok(Err::<(Adt2, u64), Bytes>(v8))
                    }
                }
            } else {
                let m1484 = f189(v0.clone(), v1)?;
                match m1484 {
                    None => {
                        if f194(f267(v0.clone(), v1, false)?) {
                            Ok(Err::<(Adt2, u64), Bytes>(f102(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                        } else {
                            let v13: u64 = f268(v0.clone(), v1, false)?;
                            let m1488 = f269(v0.clone(), v13, v2, {
                                let a1486 = {
                                    let a1485 = v0.clone();
                                    length_bytes(a1485)
                                };
                                let a1487 = 2u64;
                                nat_add(a1486, a1487)?
                            }, f270(v0, v1, v13))?;
                            match m1488 {
                                Ok(v15) => {
                                    let m1490 = f271({
                                        let (h1489, _) = v15;
                                        h1489
                                    }, v1)?;
                                    match m1490 {
                                        Ok(v17) => {
                                            Ok(Ok::<(Adt2, u64), Bytes>((v17, {
                                                let (_, h1491) = v15;
                                                h1491
                                            })))
                                        }
                                        Err(v16) => {
                                            Ok(Err::<(Adt2, u64), Bytes>(v16))
                                        }
                                    }
                                }
                                Err(v14) => {
                                    Ok(Err::<(Adt2, u64), Bytes>(v14))
                                }
                            }
                        }
                    }
                    Some(v12) => {
                        Ok(Err::<(Adt2, u64), Bytes>(v12))
                    }
                }
            }
        }
    }
}

pub fn f191(v0: bool, v1: Result<(Adt2, u64), Bytes>) -> Result<(Adt2, u64), Bytes> {
    let m1492 = v1;
    match m1492 {
        Ok(v3) => {
            Ok::<(Adt2, u64), Bytes>((if v0 {
                Adt2::C4(std::rc::Rc::new({
                    let (h1493, _) = v3;
                    h1493
                }))
            } else {
                Adt2::C5(std::rc::Rc::new({
                    let (h1494, _) = v3;
                    h1494
                }))
            }, {
                let (_, h1495) = v3;
                h1495
            }))
        }
        Err(v2) => {
            Err::<(Adt2, u64), Bytes>(v2)
        }
    }
}

pub fn f192(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f272(v0.clone(), v1)?;
    let a1497 = v2;
    let a1498 = {
        let a1496 = v0;
        length_bytes(a1496)
    };
    if nat_lt(a1497, a1498) {
        let a1499 = v2;
        let a1500 = 1u64;
        nat_add(a1499, a1500)
    } else {
        Ok(v2)
    }
}

pub fn f193(v0: u64) -> R<Bytes> {
    f102(Str::lit("a null node is not read: Studio.Value has no null (YAML 1.2.2 section 10.3.2)"), v0)
}

pub fn f194(v0: bool) -> bool {
    !v0
}

pub fn f195(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f121(v0.clone(), v1)?;
    let a1501 = v2;
    let a1502 = 34u64;
    if if nat_eq(a1501, a1502) {
        true
    } else {
        let a1503 = v2;
        let a1504 = 39u64;
        nat_eq(a1503, a1504)
    } {
        let m1505 = f265(v0.clone(), v1, 0u64, true)?;
        match m1505 {
            Ok(v4) => {
                let v5: u64 = f186(v0.clone(), {
                    let (_, h1506) = v4;
                    h1506
                })?;
                if f273(v0, v5)? {
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
        if f267(v0.clone(), v1, false)? {
            let v6: u64 = f186(v0.clone(), f268(v0.clone(), v1, false)?)?;
            if f273(v0, v6)? {
                Ok(Some(v6))
            } else {
                Ok(None::<u64>)
            }
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f196(v0: Bytes, v1: u64) -> R<Bytes> {
    let v2: u64 = f121(v0.clone(), v1)?;
    let a1507 = v2;
    let a1508 = 34u64;
    if if nat_eq(a1507, a1508) {
        true
    } else {
        let a1509 = v2;
        let a1510 = 39u64;
        nat_eq(a1509, a1510)
    } {
        let m1511 = f265(v0, v1, 0u64, true)?;
        match m1511 {
            Ok(v4) => {
                let (h1512, _) = v4;
                Ok(h1512)
            }
            Err(_) => {
                Ok(Bytes::lit(&[]))
            }
        }
    } else {
        Ok(f270(v0.clone(), v1, f268(v0, v1, false)?))
    }
}

pub fn f197(v0: Bytes, p1515: List<Bytes>) -> bool {
    let m1514 = p1515;
    {
        let mut m1514 = m1514;
        loop {
            let step = {
                let v1 = m1514;
                let m1513 = v1;
                match m1513.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f41(v2, v0.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1514 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f198(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f186(v0.clone(), v1)?;
    let v3: u64 = f121(v0.clone(), v2)?;
    if if f183(v3) {
        true
    } else {
        let a1516 = v3;
        let a1517 = 35u64;
        if nat_eq(a1516, a1517) {
            let a1518 = v1;
            let a1519 = v2;
            nat_lt(a1518, a1519)
        } else {
            false
        }
    } {
        Ok(Ok::<u64, Bytes>(f192(v0, v2)?))
    } else {
        Ok(Err::<u64, Bytes>(f102(Str::lit("only white space and a comment may follow a node on its line (YAML 1.2.2 [77])"), v2)?))
    }
}

pub fn f199(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1520 = 0u64;
        let a1521 = v1;
        nat_lt(a1520, a1521)
    } else {
        false
    } {
        let a1522 = Bytes::lit(&[0x2d]);
        let a1523 = f59(v1)?;
        Ok(append_bytes(a1522, a1523))
    } else {
        f59(v1)
    }
}

pub fn f200(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f59(v1)?;
    let v4: u64 = {
        let a1524 = v3.clone();
        length_bytes(a1524)
    };
    let v5: Bytes = {
        let a1525 = v2;
        let a1526 = v4;
        if nat_lt(a1525, a1526) {
            f7(List::cons(f38(v3.clone(), 0u64, {
                let a1527 = v4;
                let a1528 = v2;
                nat_sub(a1527, a1528)
            }), List::cons(f8(Str::lit(".")), List::cons(f38(v3, {
                let a1529 = v4;
                let a1530 = v2;
                nat_sub(a1529, a1530)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f214({
                let a1531 = v2;
                let a1532 = v4;
                nat_sub(a1531, a1532)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1533 = f8(Str::lit("-"));
        let a1534 = v5;
        Ok(append_bytes(a1533, a1534))
    } else {
        Ok(v5)
    }
}

pub fn f201(v0: Bytes) -> R<Bytes> {
    let a1539 = f126(v0.clone(), 0u64, {
        let a1536 = {
            let a1535 = v0.clone();
            length_bytes(a1535)
        };
        let a1537 = 1u64;
        nat_add(a1536, a1537)?
    })?;
    let a1540 = {
        let a1538 = v0.clone();
        length_bytes(a1538)
    };
    if nat_eq(a1539, a1540) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        f274(v0)
    }
}

pub fn f202(v0: List<Bytes>) -> Bytes {
    let a1543 = {
        let a1541 = Bytes::lit(&[0x5b]);
        let a1542 = f39(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1541, a1542)
    };
    let a1544 = Bytes::lit(&[0x5d]);
    append_bytes(a1543, a1544)
}

pub fn f203(v0: Adt2) -> R<List<Bytes>> {
    let m1545 = v0;
    match m1545 {
        Adt2::C0(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C2(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C3(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C4(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C5(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C6 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C7(r1546, r1547) => {
            let v10 = (*r1546).clone();
            let v11 = (*r1547).clone();
            Ok(List::cons(f124(v10)?, f203(v11)?))
        }
        Adt2::C8(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f204(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f275(Fn7::F276, List::<(Bytes, Bytes)>::nil(), f277(v0)?);
    let v2: List<Bytes> = f278(Fn8::F279, List::<Bytes>::nil(), v1)?;
    let a1550 = {
        let a1548 = Bytes::lit(&[0x7b]);
        let a1549 = f39(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1548, a1549)
    };
    let a1551 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1550, a1551))
}

pub fn f205(v0: Adt2) -> R<List<(Bytes, Bytes)>> {
    let m1552 = v0;
    match m1552 {
        Adt2::C0(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C2(_, _, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C3(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C4(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C5(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C6 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C7(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C8(v12, r1553, r1554) => {
            let v13 = (*r1553).clone();
            let v14 = (*r1554).clone();
            Ok(List::cons((v12, f124(v13)?), f205(v14)?))
        }
    }
}

pub fn f206(v0: Fn5, p1559: u64, p1560: (u64, (Bytes, u64))) -> R<((u64, (Bytes, u64)), bool)> {
    let m1558 = (p1559, p1560);
    {
        let mut m1558 = m1558;
        loop {
            let step = {
                let (v1, v2) = m1558;
                let m1555 = v1;
                if m1555 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1555 - 1;
                    let m1557 = {
                        let c1556: Fn5 = v0.clone();
                        c1556.apply(v2.clone())?
                    };
                    match m1557 {
                        None => {
                            Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1558 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f207(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    f280(v0, v1)
}

pub fn f208(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f281(v0, v1)
}

pub fn f209(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f282(v0, v1)
}

pub fn f210(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1561 = v0.clone();
        length_bytes(a1561)
    };
    let a1562 = 0u64;
    let a1563 = v1;
    if nat_lt(a1562, a1563) {
        let a1564 = f283(v0.clone())?;
        let a1565 = v1;
        if nat_eq(a1564, a1565) {
            let a1566 = 1u64;
            let a1567 = v1;
            if nat_lt(a1566, a1567) {
                let m1570 = {
                    let a1568 = v0;
                    let a1569 = 0u64;
                    index_bytes(a1568, a1569)
                };
                match m1570 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1571 = v2;
                        let a1572 = 48u8;
                        Ok(!equal(a1571, a1572))
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

pub fn f211(v0: Fn6, p1577: u64, p1578: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1576 = (p1577, p1578);
    {
        let mut m1576 = m1576;
        loop {
            let step = {
                let (v1, v2) = m1576;
                let m1573 = v1;
                if m1573 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1573 - 1;
                    let m1575 = {
                        let c1574: Fn6 = v0.clone();
                        c1574.apply(v2)?
                    };
                    match m1575 {
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
                Ok(next) => { m1576 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f212(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f284(v0, v1)
}

pub fn f213(v0: Bytes) -> R<u64> {
    let (h1579, _) = f154(Fn4::F285(v0.clone()), {
        let a1581 = {
            let a1580 = v0;
            length_bytes(a1580)
        };
        let a1582 = 1u64;
        nat_add(a1581, a1582)?
    }, 0u64)?;
    Ok(h1579)
}

pub fn f214(v0: u64) -> Bytes {
    let m1583 = v0;
    if m1583 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1583 - 1;
        let a1584 = f8(Str::lit("0"));
        let a1585 = f214(v1);
        append_bytes(a1584, a1585)
    }
}

pub fn f215(p1640: u64, v1: Bytes, p1641: u64, p1642: List<(u64, Bytes)>) -> R<Result<List<(u64, Bytes)>, Bytes>> {
    let m1639 = (p1640, (p1641, p1642));
    {
        let mut m1639 = m1639;
        loop {
            let step = {
                let (v0, (v2, v3)) = m1639;
                let m1586 = v0;
                if m1586 == 0 {
                    Err::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>(Ok(Err::<List<(u64, Bytes)>, Bytes>(f286(Str::lit("the input has more tokens than bytes"), v2)?)))
                } else {
                    let v4 = m1586 - 1;
                    let v5: u64 = f106(v1.clone(), v2)?;
                    let a1587 = v5;
                    let a1588 = 256u64;
                    if nat_eq(a1587, a1588) {
                        Err::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>(Ok(Ok::<List<(u64, Bytes)>, Bytes>(f287(v3, List::<(u64, Bytes)>::nil()))))
                    } else {
                        if f156(v5)? {
                            Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, ({
                                let a1589 = v2;
                                let a1590 = 1u64;
                                nat_add(a1589, a1590)?
                            }, v3)))
                        } else {
                            if f105(v1.clone(), v2, Str::lit("//")) {
                                let m1591 = f157(v1.clone(), Bytes::lit(&[0x0a]), v2)?;
                                match m1591 {
                                    None => {
                                        Err::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>(Ok(Ok::<List<(u64, Bytes)>, Bytes>(f287(v3, List::<(u64, Bytes)>::nil()))))
                                    }
                                    Some(v6) => {
                                        Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, ({
                                            let a1592 = v6;
                                            let a1593 = 1u64;
                                            nat_add(a1592, a1593)?
                                        }, v3)))
                                    }
                                }
                            } else {
                                if f105(v1.clone(), v2, Str::lit("/*")) {
                                    let m1597 = f157(v1.clone(), {
                                        let a1594 = Str::lit("*/");
                                        utf8_encode(a1594)
                                    }, {
                                        let a1595 = v2;
                                        let a1596 = 2u64;
                                        nat_add(a1595, a1596)?
                                    })?;
                                    match m1597 {
                                        None => {
                                            Err::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>(Ok(Err::<List<(u64, Bytes)>, Bytes>(f286(Str::lit("a comment is not closed (Web IDL, IDL grammar: comment)"), v2)?)))
                                        }
                                        Some(v7) => {
                                            Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, ({
                                                let a1598 = v7;
                                                let a1599 = 2u64;
                                                nat_add(a1598, a1599)?
                                            }, v3)))
                                        }
                                    }
                                } else {
                                    let a1600 = v5;
                                    let a1601 = 34u64;
                                    if nat_eq(a1600, a1601) {
                                        let m1604 = f157(v1.clone(), Bytes::lit(&[0x22]), {
                                            let a1602 = v2;
                                            let a1603 = 1u64;
                                            nat_add(a1602, a1603)?
                                        })?;
                                        match m1604 {
                                            None => {
                                                Err::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>(Ok(Err::<List<(u64, Bytes)>, Bytes>(f286(Str::lit("a string is not closed (Web IDL, IDL grammar: string)"), v2)?)))
                                            }
                                            Some(v8) => {
                                                Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, ({
                                                    let a1605 = v8;
                                                    let a1606 = 1u64;
                                                    nat_add(a1605, a1606)?
                                                }, List::cons((3u64, f38(v1.clone(), {
                                                    let a1607 = v2;
                                                    let a1608 = 1u64;
                                                    nat_add(a1607, a1608)?
                                                }, {
                                                    let a1611 = v8;
                                                    let a1612 = {
                                                        let a1609 = v2;
                                                        let a1610 = 1u64;
                                                        nat_add(a1609, a1610)?
                                                    };
                                                    nat_sub(a1611, a1612)
                                                })), v3))))
                                            }
                                        }
                                    } else {
                                        let v9: u64 = {
                                            let a1613 = v5;
                                            let a1614 = 95u64;
                                            if if nat_eq(a1613, a1614) {
                                                true
                                            } else {
                                                let a1615 = v5;
                                                let a1616 = 45u64;
                                                nat_eq(a1615, a1616)
                                            } {
                                                let a1617 = v2;
                                                let a1618 = 1u64;
                                                nat_add(a1617, a1618)?
                                            } else {
                                                v2
                                            }
                                        };
                                        if f288(f106(v1.clone(), v9)?) {
                                            let v10: u64 = f289(v1.clone(), 3u64, {
                                                let a1619 = v9;
                                                let a1620 = 1u64;
                                                nat_add(a1619, a1620)?
                                            })?;
                                            Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, (v10, List::cons((0u64, f38(v1.clone(), v2, {
                                                let a1621 = v10;
                                                let a1622 = v2;
                                                nat_sub(a1621, a1622)
                                            })), v3))))
                                        } else {
                                            let v11: u64 = {
                                                let a1623 = v5;
                                                let a1624 = 45u64;
                                                if nat_eq(a1623, a1624) {
                                                    let a1625 = v2;
                                                    let a1626 = 1u64;
                                                    nat_add(a1625, a1626)?
                                                } else {
                                                    v2
                                                }
                                            };
                                            let v12: u64 = f290(v1.clone(), v11)?;
                                            let v13: u64 = f291(v1.clone(), v11)?;
                                            let a1627 = v12;
                                            let a1628 = v13;
                                            if nat_lt(a1627, a1628) {
                                                Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, (v13, List::cons((2u64, f38(v1.clone(), v2, {
                                                    let a1629 = v13;
                                                    let a1630 = v2;
                                                    nat_sub(a1629, a1630)
                                                })), v3))))
                                            } else {
                                                let a1631 = v11;
                                                let a1632 = v12;
                                                if nat_lt(a1631, a1632) {
                                                    Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, (v12, List::cons((1u64, f38(v1.clone(), v2, {
                                                        let a1633 = v12;
                                                        let a1634 = v2;
                                                        nat_sub(a1633, a1634)
                                                    })), v3))))
                                                } else {
                                                    if f105(v1.clone(), v2, Str::lit("...")) {
                                                        Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, ({
                                                            let a1635 = v2;
                                                            let a1636 = 3u64;
                                                            nat_add(a1635, a1636)?
                                                        }, List::cons((4u64, f8(Str::lit("..."))), v3))))
                                                    } else {
                                                        Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, ({
                                                            let a1637 = v2;
                                                            let a1638 = 1u64;
                                                            nat_add(a1637, a1638)?
                                                        }, List::cons((4u64, f38(v1.clone(), v2, 1u64)), v3))))
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
                Ok(next) => { m1639 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f216(p1645: List<Adt13>, p1646: List<Adt13>) -> List<Adt13> {
    let m1644 = (p1645, p1646);
    {
        let mut m1644 = m1644;
        loop {
            let step = {
                let (v0, v1) = m1644;
                let m1643 = v0;
                match m1643.uncons() {
                    None => {
                        Err::<(List<Adt13>, List<Adt13>), List<Adt13>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt13>, List<Adt13>), List<Adt13>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1644 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f217(v0: List<(u64, Bytes)>) -> R<Result<List<(u64, Bytes)>, Bytes>> {
    if f292(v0.clone(), Str::lit("[")) {
        Ok(f293({
            let a1648 = {
                let a1647 = v0.clone();
                length_list(a1647)
            };
            let a1649 = 1u64;
            nat_add(a1648, a1649)?
        }, f294(v0), List::cons(f8(Str::lit("]")), List::<Bytes>::nil()), false))
    } else {
        Ok(Ok::<List<(u64, Bytes)>, Bytes>(v0))
    }
}

pub fn f218(v0: u64, v1: List<(u64, Bytes)>) -> R<Result<(Adt13, List<(u64, Bytes)>), Bytes>> {
    if f292(v1.clone(), Str::lit("interface")) {
        if f292(f294(v1.clone()), Str::lit("mixin")) {
            f295(v0, 1u64, f294(f294(v1)), false)
        } else {
            f295(v0, 0u64, f294(v1), true)
        }
    } else {
        if f292(v1.clone(), Str::lit("partial")) {
            let v2: List<(u64, Bytes)> = f294(v1);
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
                        Ok(Err::<(Adt13, List<(u64, Bytes)>), Bytes>(f297(Str::lit("interface, dictionary or namespace"), v2)))
                    }
                }
            }
        } else {
            if f292(v1.clone(), Str::lit("dictionary")) {
                f295(v0, 2u64, f294(v1), true)
            } else {
                if f292(v1.clone(), Str::lit("enum")) {
                    let m1650 = f298(f294(v1));
                    match m1650 {
                        Ok(v4) => {
                            let m1652 = f299({
                                let (_, h1651) = v4;
                                h1651
                            }, Str::lit("{"));
                            match m1652 {
                                Ok(v6) => {
                                    let m1653 = f300(v0, v6, List::<Bytes>::nil());
                                    match m1653 {
                                        Ok(v8) => {
                                            Ok(Ok::<(Adt13, List<(u64, Bytes)>), Bytes>((Adt13::C0(3u64, {
                                                let (h1654, _) = v4;
                                                h1654
                                            }, Bytes::lit(&[]), List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil(), {
                                                let (h1655, _) = v8;
                                                h1655
                                            }, f301(Bytes::lit(&[]))), {
                                                let (_, h1656) = v8;
                                                h1656
                                            })))
                                        }
                                        Err(v7) => {
                                            Ok(Err::<(Adt13, List<(u64, Bytes)>), Bytes>(v7))
                                        }
                                    }
                                }
                                Err(v5) => {
                                    Ok(Err::<(Adt13, List<(u64, Bytes)>), Bytes>(v5))
                                }
                            }
                        }
                        Err(v3) => {
                            Ok(Err::<(Adt13, List<(u64, Bytes)>), Bytes>(v3))
                        }
                    }
                } else {
                    if f292(v1.clone(), Str::lit("typedef")) {
                        let m1657 = f302(v0, f294(v1), 0u64)?;
                        match m1657 {
                            Ok(v10) => {
                                let m1659 = f298({
                                    let (_, h1658) = v10;
                                    h1658
                                });
                                match m1659 {
                                    Ok(v12) => {
                                        let m1661 = f299({
                                            let (_, h1660) = v12;
                                            h1660
                                        }, Str::lit(";"));
                                        match m1661 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt13, List<(u64, Bytes)>), Bytes>((Adt13::C0(4u64, {
                                                    let (h1662, _) = v12;
                                                    h1662
                                                }, Bytes::lit(&[]), List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil(), List::<Bytes>::nil(), {
                                                    let (h1663, _) = v10;
                                                    h1663
                                                }), v14)))
                                            }
                                            Err(v13) => {
                                                Ok(Err::<(Adt13, List<(u64, Bytes)>), Bytes>(v13))
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(Err::<(Adt13, List<(u64, Bytes)>), Bytes>(v11))
                                    }
                                }
                            }
                            Err(v9) => {
                                Ok(Err::<(Adt13, List<(u64, Bytes)>), Bytes>(v9))
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
                                let m1664 = f298(v1);
                                match m1664 {
                                    Ok(v16) => {
                                        let m1666 = f299({
                                            let (_, h1665) = v16;
                                            h1665
                                        }, Str::lit("includes"));
                                        match m1666 {
                                            Ok(v18) => {
                                                let m1667 = f298(v18);
                                                match m1667 {
                                                    Ok(v20) => {
                                                        let m1669 = f299({
                                                            let (_, h1668) = v20;
                                                            h1668
                                                        }, Str::lit(";"));
                                                        match m1669 {
                                                            Ok(v22) => {
                                                                Ok(Ok::<(Adt13, List<(u64, Bytes)>), Bytes>((Adt13::C0(8u64, {
                                                                    let (h1670, _) = v16;
                                                                    h1670
                                                                }, {
                                                                    let (h1671, _) = v20;
                                                                    h1671
                                                                }, List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil(), List::<Bytes>::nil(), f301(Bytes::lit(&[]))), v22)))
                                                            }
                                                            Err(v21) => {
                                                                Ok(Err::<(Adt13, List<(u64, Bytes)>), Bytes>(v21))
                                                            }
                                                        }
                                                    }
                                                    Err(v19) => {
                                                        Ok(Err::<(Adt13, List<(u64, Bytes)>), Bytes>(v19))
                                                    }
                                                }
                                            }
                                            Err(v17) => {
                                                Ok(Err::<(Adt13, List<(u64, Bytes)>), Bytes>(v17))
                                            }
                                        }
                                    }
                                    Err(v15) => {
                                        Ok(Err::<(Adt13, List<(u64, Bytes)>), Bytes>(v15))
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

pub fn f219(p1679: List<Bytes>, v1: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>) -> Bytes {
    let m1678 = p1679;
    {
        let mut m1678 = m1678;
        loop {
            let step = {
                let v0 = m1678;
                let m1672 = v0;
                match m1672.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1673 = f144(v1.clone(), v2.clone());
                        match m1673 {
                            None => {
                                Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("refers to ")), List::cons(v2, List::cons(f8(Str::lit(", which no input defines (Web IDL 2.13.22 to 2.13.26)")), List::<Bytes>::nil())))))
                            }
                            Some(v4) => {
                                if f41({
                                    let (h1674, _) = {
                                        let (_, h1675) = v4.clone();
                                        h1675
                                    };
                                    h1674
                                }, Bytes::lit(&[])) {
                                    Ok::<List<Bytes>, Bytes>(v3)
                                } else {
                                    Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("refers to ")), List::cons(v2, List::cons(f8(Str::lit(", which is not translated: ")), List::cons({
                                        let (h1676, _) = {
                                            let (_, h1677) = v4;
                                            h1677
                                        };
                                        h1676
                                    }, List::<Bytes>::nil()))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1678 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f220(v0: Bytes) -> bool {
    if f143(v0.clone(), f303()) {
        true
    } else {
        if f143(v0.clone(), f304()) {
            true
        } else {
            if f143(v0.clone(), f305()) {
                true
            } else {
                if f143(v0.clone(), f306()) {
                    true
                } else {
                    if f143(v0.clone(), f307()) {
                        true
                    } else {
                        if f143(v0.clone(), f308()) {
                            true
                        } else {
                            f143(v0, f309())
                        }
                    }
                }
            }
        }
    }
}

pub fn f221(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("the derived name ")), List::cons(v0, List::cons(f8(Str::lit(" is a reserved token of Lean 4.32.2, which LexLean 0.4.3 refuses as a name (lexlean 0.4.3 src/backend/lean_tokens.rs, SPEC.md 17.8)")), List::<Bytes>::nil()))))
}

pub fn f222(p1685: List<Adt13>, v1: List<Adt13>, v2: Bytes) -> Option<Bytes> {
    let m1684 = p1685;
    {
        let mut m1684 = m1684;
        loop {
            let step = {
                let v0 = m1684;
                let m1680 = v0;
                match m1680.uncons() {
                    None => {
                        Err::<List<Adt13>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v3, v4)) => {
                        let a1681 = f140(v3.clone());
                        let a1682 = 8u64;
                        if if nat_eq(a1681, a1682) {
                            f41(f142(v3.clone()), v2.clone())
                        } else {
                            false
                        } {
                            let m1683 = f147(v1.clone(), 1u64, f149(v3.clone()));
                            match m1683 {
                                None => {
                                    Err::<List<Adt13>, Option<Bytes>>(Some(f149(v3)))
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
                Ok(next) => { m1684 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f223(v0: u64, v1: List<Adt13>, v2: Adt13) -> R<Result<List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, Bytes>> {
    let m1686 = v0;
    if m1686 == 0 {
        Ok(Err::<List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, Bytes>(f7(List::cons(f8(Str::lit("the inheritance of ")), List::cons(f142(v2), List::cons(f8(Str::lit(" is cyclic (Web IDL 2.2 and 2.7: an interface or dictionary must not inherit from itself)")), List::<Bytes>::nil()))))))
    } else {
        let v3 = m1686 - 1;
        let v4: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))> = {
            let a1687 = f140(v2.clone());
            let a1688 = 2u64;
            if nat_eq(a1687, a1688) {
                f310(f311(f312(v2.clone()), f313(v1.clone(), 7u64, f142(v2.clone()))))?
            } else {
                f311(f312(v2.clone()), f311(f313(v1.clone(), 5u64, f142(v2.clone())), f314(v1.clone(), v1.clone(), f142(v2.clone()))))
            }
        };
        if f41(f149(v2.clone()), Bytes::lit(&[])) {
            Ok(Ok::<List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, Bytes>(v4))
        } else {
            let m1689 = f147(v1.clone(), f140(v2.clone()), f149(v2.clone()));
            match m1689 {
                None => {
                    Ok(Err::<List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, Bytes>(f7(List::cons(f8(Str::lit("inherits ")), List::cons(f149(v2.clone()), List::cons({
                        let a1690 = f140(v2);
                        let a1691 = 2u64;
                        if nat_eq(a1690, a1691) {
                            f8(Str::lit(", which no input defines as a dictionary (Web IDL 2.7)"))
                        } else {
                            f8(Str::lit(", which no input defines as an interface (Web IDL 2.2)"))
                        }
                    }, List::<Bytes>::nil()))))))
                }
                Some(v5) => {
                    let m1692 = f223(v3, v1, v5)?;
                    match m1692 {
                        Ok(v7) => {
                            Ok(Ok::<List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, Bytes>(f311(v7, v4)))
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

pub fn f224(v0: bool, v1: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> R<Result<(List<Bytes>, List<Bytes>), Bytes>> {
    let m1693 = v1;
    match m1693.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::<Bytes>::nil(), List::<Bytes>::nil())))
        }
        Some((v2, v3)) => {
            let v4: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) = {
                let (h1694, _) = {
                    let (_, h1695) = v2.clone();
                    h1695
                };
                h1694
            };
            if !f41(f234(v4.clone()), Bytes::lit(&[])) {
                Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons({
                    let (h1696, _) = v2;
                    h1696
                }, List::cons(f8(Str::lit(": ")), List::cons(f234(v4), List::<Bytes>::nil())))))))
            } else {
                let m1698 = f315(v0, {
                    let (h1697, _) = v2.clone();
                    h1697
                })?;
                match m1698 {
                    None => {
                        Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons({
                            let (h1699, _) = v2;
                            h1699
                        }, List::cons(f8(Str::lit(" has no field name")), List::<Bytes>::nil()))))))
                    }
                    Some(v5) => {
                        if f220(v5.clone()) {
                            Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f221(v5)))
                        } else {
                            if f143(v5.clone(), f316(v0, f226(v3.clone()))?) {
                                Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("two of its members derive the field name ")), List::cons(v5, List::cons(f8(Str::lit(" (Web IDL 2.2 and 2.7: the members of one definition have distinct identifiers)")), List::<Bytes>::nil()))))))
                            } else {
                                let m1700 = f224(v0, v3)?;
                                match m1700 {
                                    Ok(v7) => {
                                        Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v5, List::cons(f8(Str::lit(", type: ")), List::cons({
                                            let a1703 = {
                                                let (_, h1701) = {
                                                    let (_, h1702) = v2;
                                                    h1702
                                                };
                                                h1701
                                            };
                                            let a1704 = 1u64;
                                            if nat_eq(a1703, a1704) {
                                                f7(List::cons(f8(Str::lit("(Option ")), List::cons(f236(v4.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                            } else {
                                                f236(v4.clone())
                                            }
                                        }, List::cons(f8(Str::lit("}")), List::<Bytes>::nil())))))), {
                                            let (h1705, _) = v7;
                                            h1705
                                        }), f145(f237(v4), {
                                            let (_, h1706) = v7;
                                            h1706
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

pub fn f225(p1710: List<Bytes>) -> R<bool> {
    let m1709 = p1710;
    {
        let mut m1709 = m1709;
        loop {
            let step = {
                let v0 = m1709;
                let m1707 = v0;
                match m1707.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m1708 = f317(v1)?;
                        match m1708 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f143(v3, f316(false, v2.clone())?) {
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
                Ok(next) => { m1709 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f226(v0: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> List<Bytes> {
    let m1711 = v0;
    match m1711.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h1712, _) = v1;
                h1712
            }, f226(v2))
        }
    }
}

pub fn f227(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}")), List::cons(f94(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f228(v0: Adt13) -> List<Bytes> {
    let m1713 = v0;
    match m1713 {
        Adt13::C0(_, _, _, _, v5, _) => {
            v5
        }
    }
}

pub fn f229(v0: bool, p1717: List<Bytes>) -> R<Option<Bytes>> {
    let m1716 = p1717;
    {
        let mut m1716 = m1716;
        loop {
            let step = {
                let v1 = m1716;
                let m1714 = v1;
                match m1714.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m1715 = f315(v0, v2)?;
                        match m1715 {
                            None => {
                                Ok::<List<Bytes>, R<Option<Bytes>>>(v3)
                            }
                            Some(v4) => {
                                if f220(v4.clone()) {
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
                Ok(next) => { m1716 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f230(v0: bool, v1: List<Adt2>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1718 = v1;
    match m1718.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m1719 = f318(v0, v3.clone())?;
            match m1719 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f65(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f143(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m1720 = f230(v0, v4, List::cons(v5.clone(), v2))?;
                        match m1720 {
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

pub fn f231(v0: List<Bytes>) -> List<Adt2> {
    let m1721 = v0;
    match m1721.uncons() {
        None => {
            List::<Adt2>::nil()
        }
        Some((v1, v2)) => {
            List::cons(Adt2::C3(v1), f231(v2))
        }
    }
}

pub fn f232(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f94(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f233(v0: Adt13) -> (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) {
    let m1722 = v0;
    match m1722 {
        Adt13::C0(_, _, _, _, _, v6) => {
            v6
        }
    }
}

pub fn f234(v0: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))) -> Bytes {
    let (h1723, _) = {
        let (_, h1724) = {
            let (_, h1725) = {
                let (_, h1726) = v0;
                h1726
            };
            h1725
        };
        h1724
    };
    h1723
}

pub fn f235(v0: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))) -> u64 {
    let (h1727, _) = v0;
    h1727
}

pub fn f236(v0: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))) -> Bytes {
    let (h1728, _) = {
        let (_, h1729) = v0;
        h1729
    };
    h1728
}

pub fn f237(v0: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))) -> List<Bytes> {
    let (h1730, _) = {
        let (_, h1731) = {
            let (_, h1732) = v0;
            h1732
        };
        h1731
    };
    h1730
}

pub fn f238(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f239(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h1733, _) = v2;
        h1733
    };
    let a1734 = v3;
    let a1735 = 0u64;
    if nat_eq(a1734, a1735) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a1736 = v3;
            let a1737 = v1;
            let a1738 = 0u64;
            nat_quot(a1736, a1737, a1738)
        }, {
            let a1743 = f150(v0, {
                let a1739 = v3;
                let a1740 = v1;
                let a1741 = 0u64;
                nat_rem(a1739, a1740, a1741)
            });
            let a1744 = {
                let (_, h1742) = v2;
                h1742
            };
            append_bytes(a1743, a1744)
        }))
    }
}

pub fn f240(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h1745, _) = v1;
        h1745
    };
    let a1747 = {
        let a1746 = v0.clone();
        length_bytes(a1746)
    };
    let a1748 = v2;
    if nat_le(a1747, a1748) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let a1749 = f106(v0.clone(), v2)?;
        let a1750 = 13u64;
        if nat_eq(a1749, a1750) {
            Ok(Some(({
                let a1753 = f106(v0, {
                    let a1751 = v2;
                    let a1752 = 1u64;
                    nat_add(a1751, a1752)?
                })?;
                let a1754 = 10u64;
                if nat_eq(a1753, a1754) {
                    let a1755 = v2;
                    let a1756 = 2u64;
                    nat_add(a1755, a1756)?
                } else {
                    let a1757 = v2;
                    let a1758 = 1u64;
                    nat_add(a1757, a1758)?
                }
            }, {
                let a1760 = {
                    let (_, h1759) = v1;
                    h1759
                };
                let a1761 = f8(Str::lit("\u{a}"));
                append_bytes(a1760, a1761)
            })))
        } else {
            let v4: u64 = {
                let m1762 = f157(v0.clone(), f8(Str::lit("\u{d}")), v2)?;
                match m1762 {
                    None => {
                        let a1763 = v0.clone();
                        length_bytes(a1763)
                    }
                    Some(v3) => {
                        v3
                    }
                }
            };
            Ok(Some((v4, {
                let a1767 = {
                    let (_, h1764) = v1;
                    h1764
                };
                let a1768 = f38(v0, v2, {
                    let a1765 = v4;
                    let a1766 = v2;
                    nat_sub(a1765, a1766)
                });
                append_bytes(a1767, a1768)
            })))
        }
    }
}

pub fn f241(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1769 = f319(v0, v1)?;
    match m1769 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f320({
                let (h1770, _) = v2;
                h1770
            })? {
                Ok(Some({
                    let a1772 = v1;
                    let a1773 = {
                        let (_, h1771) = v2;
                        h1771
                    };
                    nat_add(a1772, a1773)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f242(v0: u64, v1: Bytes) -> R<bool> {
    f321(v0, v1.clone(), 0u64, {
        let a1775 = {
            let a1774 = v1;
            length_bytes(a1774)
        };
        let a1776 = 3u64;
        let a1777 = 0u64;
        nat_quot(a1775, a1776, a1777)
    }, true)
}

pub fn f243() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{0}\u{0} "))
}

pub fn f244(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f322(v1, v0, v2)
}

pub fn f245(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f323(v0, v1)
}

pub fn f246(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f324(v0, v1, v2)
}

pub fn f247(v0: Bytes) -> Result<(Bytes, u64), Bytes> {
    Err::<(Bytes, u64), Bytes>(v0)
}

pub fn f248(v0: Bytes, v1: u64) -> R<Result<(Bytes, u64), Bytes>> {
    if f105(v0.clone(), v1, Str::lit("&#")) {
        let v2: bool = {
            let a1780 = f106(v0.clone(), {
                let a1778 = v1;
                let a1779 = 2u64;
                nat_add(a1778, a1779)?
            })?;
            let a1781 = 120u64;
            nat_eq(a1780, a1781)
        };
        let v3: u64 = if v2 {
            let a1782 = v1;
            let a1783 = 3u64;
            nat_add(a1782, a1783)?
        } else {
            let a1784 = v1;
            let a1785 = 2u64;
            nat_add(a1784, a1785)?
        };
        let v4: (u64, u64) = f325(v0.clone(), v2, v3)?;
        let a1787 = {
            let (h1786, _) = v4;
            h1786
        };
        let a1788 = v3;
        if if nat_eq(a1787, a1788) {
            true
        } else {
            let a1790 = f106(v0, {
                let (h1789, _) = v4;
                h1789
            })?;
            let a1791 = 59u64;
            !nat_eq(a1790, a1791)
        } {
            Ok(f247(f102(Str::lit("a character reference must be &#digits; or &#xhex; (XML 1.0 [66])"), v1)?))
        } else {
            if f320({
                let (_, h1792) = v4;
                h1792
            })? {
                Ok(Ok::<(Bytes, u64), Bytes>((f326({
                    let (_, h1793) = v4;
                    h1793
                })?, {
                    let a1795 = {
                        let (h1794, _) = v4;
                        h1794
                    };
                    let a1796 = 1u64;
                    nat_add(a1795, a1796)?
                })))
            } else {
                Ok(f247(f102(Str::lit("a character reference must name a Char (XML 1.0 [66] WFC: Legal Character)"), v1)?))
            }
        }
    } else {
        if f105(v0.clone(), v1, Str::lit("&lt;")) {
            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("<")), {
                let a1797 = v1;
                let a1798 = 4u64;
                nat_add(a1797, a1798)?
            })))
        } else {
            if f105(v0.clone(), v1, Str::lit("&gt;")) {
                Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit(">")), {
                    let a1799 = v1;
                    let a1800 = 4u64;
                    nat_add(a1799, a1800)?
                })))
            } else {
                if f105(v0.clone(), v1, Str::lit("&amp;")) {
                    Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("&")), {
                        let a1801 = v1;
                        let a1802 = 5u64;
                        nat_add(a1801, a1802)?
                    })))
                } else {
                    if f105(v0.clone(), v1, Str::lit("&apos;")) {
                        Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("'")), {
                            let a1803 = v1;
                            let a1804 = 6u64;
                            nat_add(a1803, a1804)?
                        })))
                    } else {
                        if f105(v0, v1, Str::lit("&quot;")) {
                            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("\"")), {
                                let a1805 = v1;
                                let a1806 = 6u64;
                                nat_add(a1805, a1806)?
                            })))
                        } else {
                            Ok(f247(f102(Str::lit("an entity reference names an entity that is not declared, and only the five predefined entities are (XML 1.0 section 4.1, WFC: Entity Declared)"), v1)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f249(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<u64> {
    let (h1807, _) = f154(Fn4::F327(v1, v2, v0.clone()), {
        let a1809 = {
            let a1808 = v0;
            length_bytes(a1808)
        };
        let a1810 = 1u64;
        nat_add(a1809, a1810)?
    }, v3)?;
    Ok(h1807)
}

pub fn f250(p1823: Adt14, v1: List<(Bytes, Bytes)>) -> R<Result<Option<Adt16>, Bytes>> {
    let m1822 = p1823;
    {
        let mut m1822 = m1822;
        loop {
            let step = {
                let v0 = m1822;
                let m1811 = v0;
                match m1811 {
                    Adt14::C0(_, _, _) => {
                        Err::<Adt14, R<Result<Option<Adt16>, Bytes>>>(Ok(Ok::<Option<Adt16>, Bytes>(None::<Adt16>)))
                    }
                    Adt14::C1(_, _, _) => {
                        Err::<Adt14, R<Result<Option<Adt16>, Bytes>>>(Ok(Ok::<Option<Adt16>, Bytes>(None::<Adt16>)))
                    }
                    Adt14::C2(_) => {
                        Err::<Adt14, R<Result<Option<Adt16>, Bytes>>>(Ok(Ok::<Option<Adt16>, Bytes>(None::<Adt16>)))
                    }
                    Adt14::C3(r1820, r1821) => {
                        let v2 = (*r1820).clone();
                        let v3 = (*r1821).clone();
                        let m1812 = v2;
                        match m1812 {
                            Adt14::C0(v4, r1818, r1819) => {
                                let v5 = (*r1818).clone();
                                let v6 = (*r1819).clone();
                                let m1813 = f109(v5.clone(), v1.clone());
                                match m1813 {
                                    Ok(v8) => {
                                        let m1814 = f110(v8.clone(), v4)?;
                                        match m1814 {
                                            Ok(v10) => {
                                                if f197({
                                                    let (_, h1815) = v10.clone();
                                                    h1815
                                                }, List::cons(f8(Str::lit("restriction")), List::cons(f8(Str::lit("list")), List::cons(f8(Str::lit("union")), List::<Bytes>::nil())))) {
                                                    if f41({
                                                        let (h1816, _) = v10;
                                                        h1816
                                                    }, f174()) {
                                                        Err::<Adt14, R<Result<Option<Adt16>, Bytes>>>(Ok(Ok::<Option<Adt16>, Bytes>(Some(Adt16::C0({
                                                            let (_, h1817) = v10;
                                                            h1817
                                                        }, v5, v6, v8)))))
                                                    } else {
                                                        Ok::<Adt14, R<Result<Option<Adt16>, Bytes>>>(v3)
                                                    }
                                                } else {
                                                    Ok::<Adt14, R<Result<Option<Adt16>, Bytes>>>(v3)
                                                }
                                            }
                                            Err(v9) => {
                                                Err::<Adt14, R<Result<Option<Adt16>, Bytes>>>(Ok(f328(v9)))
                                            }
                                        }
                                    }
                                    Err(v7) => {
                                        Err::<Adt14, R<Result<Option<Adt16>, Bytes>>>(Ok(f328(v7)))
                                    }
                                }
                            }
                            Adt14::C1(_, _, _) => {
                                Ok::<Adt14, R<Result<Option<Adt16>, Bytes>>>(v3)
                            }
                            Adt14::C2(_) => {
                                Ok::<Adt14, R<Result<Option<Adt16>, Bytes>>>(v3)
                            }
                            Adt14::C3(_, _) => {
                                Ok::<Adt14, R<Result<Option<Adt16>, Bytes>>>(v3)
                            }
                            Adt14::C4 => {
                                Ok::<Adt14, R<Result<Option<Adt16>, Bytes>>>(v3)
                            }
                        }
                    }
                    Adt14::C4 => {
                        Err::<Adt14, R<Result<Option<Adt16>, Bytes>>>(Ok(Ok::<Option<Adt16>, Bytes>(None::<Adt16>)))
                    }
                }
            };
            match step {
                Ok(next) => { m1822 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f251(v0: Bytes) -> Result<Adt15, Bytes> {
    Err::<Adt15, Bytes>(v0)
}

pub fn f252(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit(" at simpleType ")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f253(v0: List<(Bytes, Bytes)>, v1: Bytes, v2: Bytes) -> R<Result<Adt15, Bytes>> {
    let m1824 = f110(v0, v2.clone())?;
    match m1824 {
        Ok(v4) => {
            if f41({
                let (h1825, _) = v4.clone();
                h1825
            }, f174()) {
                let m1827 = f329({
                    let (_, h1826) = v4;
                    h1826
                }, 16u64)?;
                match m1827 {
                    Ok(v6) => {
                        Ok(Ok::<Adt15, Bytes>(Adt15::C0(v6, List::<Bytes>::nil())))
                    }
                    Err(v5) => {
                        Ok(f251(v5))
                    }
                }
            } else {
                if f41({
                    let (h1828, _) = v4.clone();
                    h1828
                }, v1) {
                    let v7: Bytes = f177({
                        let (_, h1829) = v4;
                        h1829
                    })?;
                    Ok(Ok::<Adt15, Bytes>(Adt15::C0(v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                } else {
                    Ok(f251(f7(List::cons(f8(Str::lit("the type ")), List::cons(v2, List::cons(f8(Str::lit(" is in the namespace ")), List::cons({
                        let (h1830, _) = v4;
                        h1830
                    }, List::cons(f8(Str::lit(", which is neither XML Schema's nor this document's target namespace; imported schema documents are not read yet")), List::<Bytes>::nil()))))))))
                }
            }
        }
        Err(v3) => {
            Ok(f251(v3))
        }
    }
}

pub fn f254(v0: Adt14, v1: List<(Bytes, Bytes)>) -> R<Result<List<Adt2>, Bytes>> {
    let m1831 = v0;
    match m1831 {
        Adt14::C0(_, _, _) => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
        Adt14::C1(_, _, _) => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
        Adt14::C2(_) => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
        Adt14::C3(r1838, r1839) => {
            let v2 = (*r1838).clone();
            let v3 = (*r1839).clone();
            let m1832 = f254(v3, v1.clone())?;
            match m1832 {
                Ok(v5) => {
                    let m1833 = v2;
                    match m1833 {
                        Adt14::C0(v6, r1837, _) => {
                            let v7 = (*r1837).clone();
                            let m1834 = f109(v7.clone(), v1);
                            match m1834 {
                                Ok(v10) => {
                                    let m1835 = f110(v10, v6)?;
                                    match m1835 {
                                        Ok(v12) => {
                                            if f111(v12, Str::lit("enumeration")) {
                                                let m1836 = f112(v7, f8(Str::lit("value")));
                                                match m1836 {
                                                    None => {
                                                        Ok(f330(f8(Str::lit("an enumeration facet must have a value attribute (XSD 1.1 Part 2, section 4.3.5.2)"))))
                                                    }
                                                    Some(v13) => {
                                                        Ok(Ok::<List<Adt2>, Bytes>(List::cons(Adt2::C3(v13), v5)))
                                                    }
                                                }
                                            } else {
                                                Ok(Ok::<List<Adt2>, Bytes>(v5))
                                            }
                                        }
                                        Err(v11) => {
                                            Ok(f330(v11))
                                        }
                                    }
                                }
                                Err(v9) => {
                                    Ok(f330(v9))
                                }
                            }
                        }
                        Adt14::C1(_, _, _) => {
                            Ok(Ok::<List<Adt2>, Bytes>(v5))
                        }
                        Adt14::C2(_) => {
                            Ok(Ok::<List<Adt2>, Bytes>(v5))
                        }
                        Adt14::C3(_, _) => {
                            Ok(Ok::<List<Adt2>, Bytes>(v5))
                        }
                        Adt14::C4 => {
                            Ok(Ok::<List<Adt2>, Bytes>(v5))
                        }
                    }
                }
                Err(v4) => {
                    Ok(f330(v4))
                }
            }
        }
        Adt14::C4 => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
    }
}

pub fn f255(v0: List<Adt2>, v1: List<Bytes>) -> R<List<Adt2>> {
    let m1840 = v0;
    match m1840.uncons() {
        None => {
            Ok(List::<Adt2>::nil())
        }
        Some((v2, v3)) => {
            let v4: Bytes = f65(v2.clone())?;
            if f143(v4.clone(), v1.clone()) {
                f255(v3, v1)
            } else {
                Ok(List::cons(v2, f255(v3, List::cons(v4, v1))?))
            }
        }
    }
}

pub fn f256(v0: List<Adt2>) -> List<Bytes> {
    let m1841 = v0;
    match m1841.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1842 = f331(Some(v1));
            match m1842 {
                None => {
                    f256(v2)
                }
                Some(v3) => {
                    List::cons(v3, f256(v2))
                }
            }
        }
    }
}

pub fn f257(v0: Bytes, p1863: u64, p1864: u64, p1865: bool, p1866: Bytes) -> R<Bytes> {
    let m1862 = (p1863, (p1864, (p1865, p1866)));
    {
        let mut m1862 = m1862;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1862;
                let m1843 = v2;
                if m1843 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1843 - 1;
                    let m1846 = {
                        let a1844 = v0.clone();
                        let a1845 = v1;
                        index_bytes(a1844, a1845)
                    };
                    match m1846 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f46(v6)?;
                            if f332(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1847 = v1;
                                    let a1848 = 1u64;
                                    nat_add(a1847, a1848)?
                                }, (v5, (true, v4))))
                            } else {
                                if f77(f333(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1849 = v1;
                                        let a1850 = 1u64;
                                        nat_add(a1849, a1850)?
                                    }, (v5, (false, {
                                        let a1854 = v4;
                                        let a1855 = f38({
                                            let a1851 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1851)
                                        }, {
                                            let a1852 = v7;
                                            let a1853 = 97u64;
                                            nat_sub(a1852, a1853)
                                        }, 1u64);
                                        append_bytes(a1854, a1855)
                                    }))))
                                } else {
                                    if f129(f333(v7), f129(f334(v7), f335(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1856 = v1;
                                            let a1857 = 1u64;
                                            nat_add(a1856, a1857)?
                                        }, (v5, (false, {
                                            let a1858 = v4;
                                            let a1859 = f38(v0.clone(), v1, 1u64);
                                            append_bytes(a1858, a1859)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1860 = v1;
                                            let a1861 = 1u64;
                                            nat_add(a1860, a1861)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1862 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f258(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1867 = f319(v0, v1)?;
    match m1867 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f336({
                let (h1868, _) = v2;
                h1868
            }) {
                Ok(Some({
                    let a1870 = v1;
                    let a1871 = {
                        let (_, h1869) = v2;
                        h1869
                    };
                    nat_add(a1870, a1871)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f259(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1873 = v1;
    let a1874 = {
        let a1872 = v0.clone();
        length_bytes(a1872)
    };
    if if nat_lt(a1873, a1874) {
        f337(v0.clone(), v1)?
    } else {
        false
    } {
        Ok(Some(f192(v0, v1)?))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f260(v0: Bytes, v1: u64) -> R<u64> {
    let (h1875, _) = f154(Fn4::F338(v0), {
        let a1876 = v1;
        let a1877 = 1u64;
        nat_add(a1876, a1877)?
    }, v1)?;
    Ok(h1875)
}

pub fn f261(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f339(v0, v1)
}

pub fn f262(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f340(v0, v1)
}

pub fn f263(v0: u64, v1: bool) -> bool {
    f194(if if f182(v0) {
        true
    } else {
        f183(v0)
    } {
        true
    } else {
        if v1 {
            f341(v0)
        } else {
            false
        }
    })
}

pub fn f264(v0: Bytes, v1: u64, v2: u64) -> R<Result<(Adt2, u64), Bytes>> {
    let v3: bool = {
        let a1878 = f121(v0.clone(), v1)?;
        let a1879 = 62u64;
        nat_eq(a1878, a1879)
    };
    let v4: u64 = f121(v0.clone(), {
        let a1880 = v1;
        let a1881 = 1u64;
        nat_add(a1880, a1881)?
    })?;
    let v5: u64 = f121(v0.clone(), {
        let a1882 = v1;
        let a1883 = 2u64;
        nat_add(a1882, a1883)?
    })?;
    let v6: u64 = f342(v4);
    let v7: u64 = f343(v4);
    let v8: bool = {
        let a1884 = 0u64;
        let a1885 = v6;
        if nat_lt(a1884, a1885) {
            true
        } else {
            let a1886 = 0u64;
            let a1887 = v7;
            nat_lt(a1886, a1887)
        }
    };
    let v9: u64 = {
        let a1888 = 0u64;
        let a1889 = v7;
        if nat_lt(a1888, a1889) {
            f342(v5)
        } else {
            0u64
        }
    };
    let v10: u64 = {
        let a1890 = 0u64;
        let a1891 = v6;
        if nat_lt(a1890, a1891) {
            f343(v5)
        } else {
            0u64
        }
    };
    let v11: u64 = {
        let a1892 = v6;
        let a1893 = v9;
        nat_add(a1892, a1893)?
    };
    let v12: u64 = {
        let a1894 = v7;
        let a1895 = v10;
        nat_add(a1894, a1895)?
    };
    let v13: u64 = {
        let a1904 = {
            let a1896 = v1;
            let a1897 = 1u64;
            nat_add(a1896, a1897)?
        };
        let a1905 = {
            let a1902 = if v8 {
                1u64
            } else {
                0u64
            };
            let a1903 = {
                let a1898 = 0u64;
                let a1899 = v9;
                if if nat_lt(a1898, a1899) {
                    true
                } else {
                    let a1900 = 0u64;
                    let a1901 = v10;
                    nat_lt(a1900, a1901)
                } {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a1902, a1903)?
        };
        nat_add(a1904, a1905)?
    };
    let m1906 = f198(v0.clone(), v13)?;
    match m1906 {
        Ok(v15) => {
            let a1907 = 0u64;
            let a1908 = v12;
            if nat_lt(a1907, a1908) {
                f344(v0, v15, {
                    let a1911 = {
                        let a1909 = v2;
                        let a1910 = v12;
                        nat_add(a1909, a1910)?
                    };
                    let a1912 = 1u64;
                    nat_sub(a1911, a1912)
                }, v3, v11)
            } else {
                let v16: (u64, u64) = {
                    let (h1913, _) = f211(Fn6::F345(v0.clone()), {
                        let a1915 = {
                            let a1914 = v0.clone();
                            length_bytes(a1914)
                        };
                        let a1916 = 1u64;
                        nat_add(a1915, a1916)?
                    }, (v15, 0u64))?;
                    h1913
                };
                let v17: u64 = f185(v0.clone(), {
                    let (h1917, _) = v16;
                    h1917
                })?;
                let a1918 = f121(v0.clone(), v17)?;
                let a1919 = 256u64;
                if nat_eq(a1918, a1919) {
                    Ok(Ok::<(Adt2, u64), Bytes>((Adt2::C3({
                        let a1920 = v11;
                        let a1921 = 2u64;
                        if nat_eq(a1920, a1921) {
                            f346(f347(v0.clone(), v15)?)
                        } else {
                            Bytes::lit(&[])
                        }
                    }), {
                        let a1922 = v0;
                        length_bytes(a1922)
                    })))
                } else {
                    let v18: u64 = f348({
                        let a1924 = v17;
                        let a1925 = {
                            let (h1923, _) = v16;
                            h1923
                        };
                        nat_sub(a1924, a1925)
                    }, v2);
                    let a1927 = v18;
                    let a1928 = {
                        let (_, h1926) = v16;
                        h1926
                    };
                    if nat_lt(a1927, a1928) {
                        Ok(Err::<(Adt2, u64), Bytes>(f102(Str::lit("a leading empty line of a block scalar holds more spaces than its first content line (YAML 1.2.2 section 8.1.1.1)"), v15)?))
                    } else {
                        f344(v0, v15, v18, v3, v11)
                    }
                }
            }
        }
        Err(v14) => {
            Ok(Err::<(Adt2, u64), Bytes>(v14))
        }
    }
}

pub fn f265(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Result<(Bytes, u64), Bytes>> {
    let a1929 = f121(v0.clone(), v1)?;
    let a1930 = 34u64;
    if nat_eq(a1929, a1930) {
        f349(v0.clone(), {
            let a1931 = v1;
            let a1932 = 1u64;
            nat_add(a1931, a1932)?
        }, v2, v3, {
            let a1934 = {
                let a1933 = v0;
                length_bytes(a1933)
            };
            let a1935 = 2u64;
            nat_add(a1934, a1935)?
        }, Bytes::lit(&[]))
    } else {
        f350(v0.clone(), {
            let a1936 = v1;
            let a1937 = 1u64;
            nat_add(a1936, a1937)?
        }, v2, v3, {
            let a1939 = {
                let a1938 = v0;
                length_bytes(a1938)
            };
            let a1940 = 2u64;
            nat_add(a1939, a1940)?
        }, Bytes::lit(&[]))
    }
}

pub fn f266(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: List<Bytes>) -> R<Result<(Adt2, u64), Bytes>> {
    let m1941 = v3;
    if m1941 == 0 {
        Ok(Err::<(Adt2, u64), Bytes>(f102(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v6 = m1941 - 1;
        let a1942 = v4;
        let a1943 = 1u64;
        if nat_eq(a1942, a1943) {
            let m1944 = f266(v0.clone(), v1, v2, v6, 0u64, List::<Bytes>::nil())?;
            match m1944 {
                Ok(v8) => {
                    let m1946 = f351(v0.clone(), {
                        let (_, h1945) = v8;
                        h1945
                    }, v2)?;
                    match m1946 {
                        Ok(v10) => {
                            let v11: u64 = f121(v0.clone(), v10)?;
                            let a1947 = v11;
                            let a1948 = 58u64;
                            if nat_eq(a1947, a1948) {
                                Ok(Err::<(Adt2, u64), Bytes>(f102(Str::lit("a single-pair mapping inside a flow sequence is not read (YAML 1.2.2 [150])"), v10)?))
                            } else {
                                let a1949 = v11;
                                let a1950 = 93u64;
                                if nat_eq(a1949, a1950) {
                                    Ok(Ok::<(Adt2, u64), Bytes>((Adt2::C7(std::rc::Rc::new({
                                        let (h1951, _) = v8;
                                        h1951
                                    }), std::rc::Rc::new(Adt2::C6)), {
                                        let a1952 = v10;
                                        let a1953 = 1u64;
                                        nat_add(a1952, a1953)?
                                    })))
                                } else {
                                    let a1954 = v11;
                                    let a1955 = 44u64;
                                    if nat_eq(a1954, a1955) {
                                        let m1958 = f351(v0.clone(), {
                                            let a1956 = v10;
                                            let a1957 = 1u64;
                                            nat_add(a1956, a1957)?
                                        }, v2)?;
                                        match m1958 {
                                            Ok(v13) => {
                                                let a1959 = f121(v0.clone(), v13)?;
                                                let a1960 = 93u64;
                                                if nat_eq(a1959, a1960) {
                                                    Ok(Ok::<(Adt2, u64), Bytes>((Adt2::C7(std::rc::Rc::new({
                                                        let (h1961, _) = v8;
                                                        h1961
                                                    }), std::rc::Rc::new(Adt2::C6)), {
                                                        let a1962 = v13;
                                                        let a1963 = 1u64;
                                                        nat_add(a1962, a1963)?
                                                    })))
                                                } else {
                                                    let m1964 = f266(v0, v13, v2, v6, 1u64, List::<Bytes>::nil())?;
                                                    match m1964 {
                                                        Ok(v15) => {
                                                            Ok(Ok::<(Adt2, u64), Bytes>((Adt2::C7(std::rc::Rc::new({
                                                                let (h1965, _) = v8;
                                                                h1965
                                                            }), std::rc::Rc::new({
                                                                let (h1966, _) = v15;
                                                                h1966
                                                            })), {
                                                                let (_, h1967) = v15;
                                                                h1967
                                                            })))
                                                        }
                                                        Err(v14) => {
                                                            Ok(Err::<(Adt2, u64), Bytes>(v14))
                                                        }
                                                    }
                                                }
                                            }
                                            Err(v12) => {
                                                Ok(Err::<(Adt2, u64), Bytes>(v12))
                                            }
                                        }
                                    } else {
                                        Ok(Err::<(Adt2, u64), Bytes>(f102(Str::lit("a flow sequence entry must be followed by , or ] (YAML 1.2.2 [137])"), v10)?))
                                    }
                                }
                            }
                        }
                        Err(v9) => {
                            Ok(Err::<(Adt2, u64), Bytes>(v9))
                        }
                    }
                }
                Err(v7) => {
                    Ok(Err::<(Adt2, u64), Bytes>(v7))
                }
            }
        } else {
            let a1968 = v4;
            let a1969 = 2u64;
            if nat_eq(a1968, a1969) {
                let m1970 = f352(v0.clone(), v1, v2)?;
                match m1970 {
                    Ok(v17) => {
                        let m1972 = f351(v0.clone(), {
                            let (_, h1971) = v17.clone();
                            h1971
                        }, v2)?;
                        match m1972 {
                            Ok(v19) => {
                                if f194({
                                    let a1973 = f121(v0.clone(), v19)?;
                                    let a1974 = 58u64;
                                    nat_eq(a1973, a1974)
                                }) {
                                    Ok(Err::<(Adt2, u64), Bytes>(f102(Str::lit("a flow mapping entry needs : and a value; a null value is not read (YAML 1.2.2 [142])"), v19)?))
                                } else {
                                    let m1977 = f351(v0.clone(), {
                                        let a1975 = v19;
                                        let a1976 = 1u64;
                                        nat_add(a1975, a1976)?
                                    }, v2)?;
                                    match m1977 {
                                        Ok(v21) => {
                                            let a1978 = f121(v0.clone(), v21)?;
                                            let a1979 = 44u64;
                                            if if nat_eq(a1978, a1979) {
                                                true
                                            } else {
                                                let a1980 = f121(v0.clone(), v21)?;
                                                let a1981 = 125u64;
                                                nat_eq(a1980, a1981)
                                            } {
                                                Ok(Err::<(Adt2, u64), Bytes>(f193(v21)?))
                                            } else {
                                                let m1982 = f266(v0.clone(), v21, v2, v6, 0u64, List::<Bytes>::nil())?;
                                                match m1982 {
                                                    Ok(v23) => {
                                                        if f197({
                                                            let (h1983, _) = v17.clone();
                                                            h1983
                                                        }, v5.clone()) {
                                                            Ok(Err::<(Adt2, u64), Bytes>(f102(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                                        } else {
                                                            let m1985 = f351(v0.clone(), {
                                                                let (_, h1984) = v23;
                                                                h1984
                                                            }, v2)?;
                                                            match m1985 {
                                                                Ok(v25) => {
                                                                    let v26: u64 = f121(v0.clone(), v25)?;
                                                                    let a1986 = v26;
                                                                    let a1987 = 125u64;
                                                                    if nat_eq(a1986, a1987) {
                                                                        Ok(Ok::<(Adt2, u64), Bytes>((Adt2::C8({
                                                                            let (h1988, _) = v17;
                                                                            h1988
                                                                        }, std::rc::Rc::new({
                                                                            let (h1989, _) = v23;
                                                                            h1989
                                                                        }), std::rc::Rc::new(Adt2::C6)), {
                                                                            let a1990 = v25;
                                                                            let a1991 = 1u64;
                                                                            nat_add(a1990, a1991)?
                                                                        })))
                                                                    } else {
                                                                        let a1992 = v26;
                                                                        let a1993 = 44u64;
                                                                        if nat_eq(a1992, a1993) {
                                                                            let m1996 = f351(v0.clone(), {
                                                                                let a1994 = v25;
                                                                                let a1995 = 1u64;
                                                                                nat_add(a1994, a1995)?
                                                                            }, v2)?;
                                                                            match m1996 {
                                                                                Ok(v28) => {
                                                                                    let a1997 = f121(v0.clone(), v28)?;
                                                                                    let a1998 = 125u64;
                                                                                    if nat_eq(a1997, a1998) {
                                                                                        Ok(Ok::<(Adt2, u64), Bytes>((Adt2::C8({
                                                                                            let (h1999, _) = v17;
                                                                                            h1999
                                                                                        }, std::rc::Rc::new({
                                                                                            let (h2000, _) = v23;
                                                                                            h2000
                                                                                        }), std::rc::Rc::new(Adt2::C6)), {
                                                                                            let a2001 = v28;
                                                                                            let a2002 = 1u64;
                                                                                            nat_add(a2001, a2002)?
                                                                                        })))
                                                                                    } else {
                                                                                        let m2004 = f266(v0, v28, v2, v6, 2u64, List::cons({
                                                                                            let (h2003, _) = v17.clone();
                                                                                            h2003
                                                                                        }, v5))?;
                                                                                        match m2004 {
                                                                                            Ok(v30) => {
                                                                                                Ok(Ok::<(Adt2, u64), Bytes>((Adt2::C8({
                                                                                                    let (h2005, _) = v17;
                                                                                                    h2005
                                                                                                }, std::rc::Rc::new({
                                                                                                    let (h2006, _) = v23;
                                                                                                    h2006
                                                                                                }), std::rc::Rc::new({
                                                                                                    let (h2007, _) = v30;
                                                                                                    h2007
                                                                                                })), {
                                                                                                    let (_, h2008) = v30;
                                                                                                    h2008
                                                                                                })))
                                                                                            }
                                                                                            Err(v29) => {
                                                                                                Ok(Err::<(Adt2, u64), Bytes>(v29))
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                Err(v27) => {
                                                                                    Ok(Err::<(Adt2, u64), Bytes>(v27))
                                                                                }
                                                                            }
                                                                        } else {
                                                                            Ok(Err::<(Adt2, u64), Bytes>(f102(Str::lit("a flow mapping entry must be followed by , or } (YAML 1.2.2 [140])"), v25)?))
                                                                        }
                                                                    }
                                                                }
                                                                Err(v24) => {
                                                                    Ok(Err::<(Adt2, u64), Bytes>(v24))
                                                                }
                                                            }
                                                        }
                                                    }
                                                    Err(v22) => {
                                                        Ok(Err::<(Adt2, u64), Bytes>(v22))
                                                    }
                                                }
                                            }
                                        }
                                        Err(v20) => {
                                            Ok(Err::<(Adt2, u64), Bytes>(v20))
                                        }
                                    }
                                }
                            }
                            Err(v18) => {
                                Ok(Err::<(Adt2, u64), Bytes>(v18))
                            }
                        }
                    }
                    Err(v16) => {
                        Ok(Err::<(Adt2, u64), Bytes>(v16))
                    }
                }
            } else {
                let v31: u64 = f121(v0.clone(), v1)?;
                let a2009 = v31;
                let a2010 = 91u64;
                if if nat_eq(a2009, a2010) {
                    true
                } else {
                    let a2011 = v31;
                    let a2012 = 123u64;
                    nat_eq(a2011, a2012)
                } {
                    let m2015 = f351(v0.clone(), {
                        let a2013 = v1;
                        let a2014 = 1u64;
                        nat_add(a2013, a2014)?
                    }, v2)?;
                    match m2015 {
                        Ok(v33) => {
                            let a2018 = f121(v0.clone(), v33)?;
                            let a2019 = {
                                let a2016 = v31;
                                let a2017 = 2u64;
                                nat_add(a2016, a2017)?
                            };
                            if nat_eq(a2018, a2019) {
                                Ok(Ok::<(Adt2, u64), Bytes>(({
                                    let a2020 = v31;
                                    let a2021 = 91u64;
                                    if nat_eq(a2020, a2021) {
                                        Adt2::C4(std::rc::Rc::new(Adt2::C6))
                                    } else {
                                        Adt2::C5(std::rc::Rc::new(Adt2::C6))
                                    }
                                }, {
                                    let a2022 = v33;
                                    let a2023 = 1u64;
                                    nat_add(a2022, a2023)?
                                })))
                            } else {
                                let m2026 = f266(v0, v33, v2, v6, {
                                    let a2024 = v31;
                                    let a2025 = 91u64;
                                    if nat_eq(a2024, a2025) {
                                        1u64
                                    } else {
                                        2u64
                                    }
                                }, List::<Bytes>::nil())?;
                                match m2026 {
                                    Ok(v35) => {
                                        Ok(Ok::<(Adt2, u64), Bytes>(({
                                            let a2027 = v31;
                                            let a2028 = 91u64;
                                            if nat_eq(a2027, a2028) {
                                                Adt2::C4(std::rc::Rc::new({
                                                    let (h2029, _) = v35;
                                                    h2029
                                                }))
                                            } else {
                                                Adt2::C5(std::rc::Rc::new({
                                                    let (h2030, _) = v35;
                                                    h2030
                                                }))
                                            }
                                        }, {
                                            let (_, h2031) = v35;
                                            h2031
                                        })))
                                    }
                                    Err(v34) => {
                                        Ok(Err::<(Adt2, u64), Bytes>(v34))
                                    }
                                }
                            }
                        }
                        Err(v32) => {
                            Ok(Err::<(Adt2, u64), Bytes>(v32))
                        }
                    }
                } else {
                    let a2032 = v31;
                    let a2033 = 34u64;
                    if if nat_eq(a2032, a2033) {
                        true
                    } else {
                        let a2034 = v31;
                        let a2035 = 39u64;
                        nat_eq(a2034, a2035)
                    } {
                        let m2036 = f265(v0, v1, v2, false)?;
                        match m2036 {
                            Ok(v37) => {
                                Ok(Ok::<(Adt2, u64), Bytes>((Adt2::C3({
                                    let (h2037, _) = v37;
                                    h2037
                                }), {
                                    let (_, h2038) = v37;
                                    h2038
                                })))
                            }
                            Err(v36) => {
                                Ok(Err::<(Adt2, u64), Bytes>(v36))
                            }
                        }
                    } else {
                        let m2039 = f189(v0.clone(), v1)?;
                        match m2039 {
                            None => {
                                if f194(f267(v0.clone(), v1, true)?) {
                                    Ok(Err::<(Adt2, u64), Bytes>(f102(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                                } else {
                                    let v39: u64 = f268(v0.clone(), v1, true)?;
                                    let v40: u64 = f186(v0.clone(), v39)?;
                                    let m2042 = {
                                        let a2040 = f121(v0.clone(), v40)?;
                                        let a2041 = 10u64;
                                        if nat_eq(a2040, a2041) {
                                            f351(v0.clone(), v40, v2)?
                                        } else {
                                            Ok::<u64, Bytes>(v40)
                                        }
                                    };
                                    match m2042 {
                                        Ok(v42) => {
                                            let a2043 = f121(v0.clone(), v40)?;
                                            let a2044 = 10u64;
                                            if if if nat_eq(a2043, a2044) {
                                                f194(f341(f121(v0.clone(), v42)?))
                                            } else {
                                                false
                                            } {
                                                f194({
                                                    let a2045 = f121(v0.clone(), v42)?;
                                                    let a2046 = 58u64;
                                                    nat_eq(a2045, a2046)
                                                })
                                            } else {
                                                false
                                            } {
                                                Ok(Err::<(Adt2, u64), Bytes>(f102(Str::lit("a multi-line plain scalar inside a flow collection is not read (YAML 1.2.2 [135])"), v40)?))
                                            } else {
                                                let m2047 = f271(f270(v0, v1, v39), v1)?;
                                                match m2047 {
                                                    Ok(v44) => {
                                                        Ok(Ok::<(Adt2, u64), Bytes>((v44, v39)))
                                                    }
                                                    Err(v43) => {
                                                        Ok(Err::<(Adt2, u64), Bytes>(v43))
                                                    }
                                                }
                                            }
                                        }
                                        Err(v41) => {
                                            Ok(Err::<(Adt2, u64), Bytes>(v41))
                                        }
                                    }
                                }
                            }
                            Some(v38) => {
                                Ok(Err::<(Adt2, u64), Bytes>(v38))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f267(v0: Bytes, v1: u64, v2: bool) -> R<bool> {
    let v3: u64 = f121(v0.clone(), v1)?;
    if if f182(v3) {
        true
    } else {
        f183(v3)
    } {
        Ok(false)
    } else {
        let a2048 = v3;
        let a2049 = 45u64;
        if if if nat_eq(a2048, a2049) {
            true
        } else {
            let a2050 = v3;
            let a2051 = 63u64;
            nat_eq(a2050, a2051)
        } {
            true
        } else {
            let a2052 = v3;
            let a2053 = 58u64;
            nat_eq(a2052, a2053)
        } {
            Ok(f263(f121(v0, {
                let a2054 = v1;
                let a2055 = 1u64;
                nat_add(a2054, a2055)?
            })?, v2))
        } else {
            Ok(f194(f353(v3)))
        }
    }
}

pub fn f268(v0: Bytes, v1: u64, v2: bool) -> R<u64> {
    let (_, h2056) = {
        let (h2057, _) = f211(Fn6::F354(v2, v0.clone()), {
            let a2059 = {
                let a2058 = v0;
                length_bytes(a2058)
            };
            let a2060 = 1u64;
            nat_add(a2059, a2060)?
        }, (v1, v1))?;
        h2057
    };
    Ok(h2056)
}

pub fn f269(v0: Bytes, p2085: u64, v2: u64, p2086: u64, p2087: Bytes) -> R<Result<(Bytes, u64), Bytes>> {
    let m2084 = (p2085, (p2086, p2087));
    {
        let mut m2084 = m2084;
        loop {
            let step = {
                let (v1, (v3, v4)) = m2084;
                let m2061 = v3;
                if m2061 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f102(Str::lit("a plain scalar has more lines than the input"), v1)?)))
                } else {
                    let v5 = m2061 - 1;
                    let v6: u64 = f186(v0.clone(), v1)?;
                    let v7: u64 = f121(v0.clone(), v6)?;
                    let a2062 = v7;
                    let a2063 = 58u64;
                    if nat_eq(a2062, a2063) {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f355(v6)?)))
                    } else {
                        let a2064 = v7;
                        let a2065 = 35u64;
                        if if nat_eq(a2064, a2065) {
                            true
                        } else {
                            let a2066 = v7;
                            let a2067 = 256u64;
                            nat_eq(a2066, a2067)
                        } {
                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v4, f192(v0.clone(), v6)?))))
                        } else {
                            let v8: (u64, u64) = f356(v0.clone(), {
                                let a2068 = v6;
                                let a2069 = 1u64;
                                nat_add(a2068, a2069)?
                            })?;
                            let v9: u64 = {
                                let (h2070, _) = v8;
                                h2070
                            };
                            let v10: u64 = f185(v0.clone(), v9)?;
                            let v11: u64 = f186(v0.clone(), v10)?;
                            let v12: u64 = f121(v0.clone(), v11)?;
                            let a2071 = v12;
                            let a2072 = 256u64;
                            if if if if nat_eq(a2071, a2072) {
                                true
                            } else {
                                let a2075 = {
                                    let a2073 = v10;
                                    let a2074 = v9;
                                    nat_sub(a2073, a2074)
                                };
                                let a2076 = v2;
                                nat_lt(a2075, a2076)
                            } {
                                true
                            } else {
                                let a2077 = v12;
                                let a2078 = 35u64;
                                nat_eq(a2077, a2078)
                            } {
                                true
                            } else {
                                f120(v0.clone(), v9)?
                            } {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v4, {
                                    let a2079 = v6;
                                    let a2080 = 1u64;
                                    nat_add(a2079, a2080)?
                                }))))
                            } else {
                                let v13: u64 = f268(v0.clone(), v11, false)?;
                                let a2081 = v13;
                                let a2082 = v11;
                                if nat_eq(a2081, a2082) {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f355(v11)?)))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v13, (v5, f7(List::cons(v4, List::cons(f357({
                                        let (_, h2083) = v8;
                                        h2083
                                    }), List::cons(f270(v0.clone(), v11, v13), List::<Bytes>::nil())))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2084 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f270(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    f38(v0, v1, {
        let a2088 = v2;
        let a2089 = v1;
        nat_sub(a2088, a2089)
    })
}

pub fn f271(v0: Bytes, v1: u64) -> R<Result<Adt2, Bytes>> {
    let a2091 = {
        let a2090 = v0.clone();
        length_bytes(a2090)
    };
    let a2092 = 0u64;
    if if if if nat_eq(a2091, a2092) {
        true
    } else {
        f358(v0.clone(), Str::lit("~"))
    } {
        true
    } else {
        if f358(v0.clone(), Str::lit("null")) {
            true
        } else {
            f358(v0.clone(), Str::lit("Null"))
        }
    } {
        true
    } else {
        f358(v0.clone(), Str::lit("NULL"))
    } {
        Ok(Err::<Adt2, Bytes>(f193(v1)?))
    } else {
        if if if f358(v0.clone(), Str::lit("true")) {
            true
        } else {
            f358(v0.clone(), Str::lit("True"))
        } {
            true
        } else {
            f358(v0.clone(), Str::lit("TRUE"))
        } {
            Ok(Ok::<Adt2, Bytes>(Adt2::C0(true)))
        } else {
            if if if f358(v0.clone(), Str::lit("false")) {
                true
            } else {
                f358(v0.clone(), Str::lit("False"))
            } {
                true
            } else {
                f358(v0.clone(), Str::lit("FALSE"))
            } {
                Ok(Ok::<Adt2, Bytes>(Adt2::C0(false)))
            } else {
                if if if if if f358(v0.clone(), Str::lit(".inf")) {
                    true
                } else {
                    f358(v0.clone(), Str::lit(".Inf"))
                } {
                    true
                } else {
                    if f358(v0.clone(), Str::lit(".INF")) {
                        true
                    } else {
                        f358(v0.clone(), Str::lit("+.inf"))
                    }
                } {
                    true
                } else {
                    if if f358(v0.clone(), Str::lit("+.Inf")) {
                        true
                    } else {
                        f358(v0.clone(), Str::lit("+.INF"))
                    } {
                        true
                    } else {
                        if f358(v0.clone(), Str::lit("-.inf")) {
                            true
                        } else {
                            f358(v0.clone(), Str::lit("-.Inf"))
                        }
                    }
                } {
                    true
                } else {
                    if if f358(v0.clone(), Str::lit("-.INF")) {
                        true
                    } else {
                        f358(v0.clone(), Str::lit(".nan"))
                    } {
                        true
                    } else {
                        if f358(v0.clone(), Str::lit(".NaN")) {
                            true
                        } else {
                            f358(v0.clone(), Str::lit(".NAN"))
                        }
                    }
                } {
                    Ok(Err::<Adt2, Bytes>(f102(Str::lit("an infinite or not-a-number float is not read: Studio.Value has none (YAML 1.2.2 section 10.3.2)"), v1)?))
                } else {
                    if if f105(v0.clone(), 0u64, Str::lit("0o")) {
                        let a2094 = 2u64;
                        let a2095 = {
                            let a2093 = v0.clone();
                            length_bytes(a2093)
                        };
                        nat_lt(a2094, a2095)
                    } else {
                        false
                    } {
                        f359(v0, 8u64, v1)
                    } else {
                        if if f105(v0.clone(), 0u64, Str::lit("0x")) {
                            let a2097 = 2u64;
                            let a2098 = {
                                let a2096 = v0.clone();
                                length_bytes(a2096)
                            };
                            nat_lt(a2097, a2098)
                        } else {
                            false
                        } {
                            f359(v0, 16u64, v1)
                        } else {
                            let m2099 = f360(v0.clone())?;
                            match m2099 {
                                None => {
                                    Ok(Ok::<Adt2, Bytes>(Adt2::C3(v0)))
                                }
                                Some(v2) => {
                                    let m2100 = f78(v2.clone(), 0u64)?;
                                    match m2100 {
                                        None => {
                                            Ok(Err::<Adt2, Bytes>(f361(v1)?))
                                        }
                                        Some(v3) => {
                                            let a2103 = {
                                                let (h2101, _) = v3;
                                                h2101
                                            };
                                            let a2104 = {
                                                let a2102 = v2;
                                                length_bytes(a2102)
                                            };
                                            if nat_eq(a2103, a2104) {
                                                Ok(Ok::<Adt2, Bytes>({
                                                    let (_, h2105) = v3;
                                                    h2105
                                                }))
                                            } else {
                                                Ok(Err::<Adt2, Bytes>(f361(v1)?))
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

pub fn f272(v0: Bytes, v1: u64) -> R<u64> {
    let (h2106, _) = f154(Fn4::F362(v0.clone()), {
        let a2108 = {
            let a2107 = v0;
            length_bytes(a2107)
        };
        let a2109 = 1u64;
        nat_add(a2108, a2109)?
    }, v1)?;
    Ok(h2106)
}

pub fn f273(v0: Bytes, v1: u64) -> R<bool> {
    let a2110 = f121(v0.clone(), v1)?;
    let a2111 = 58u64;
    if nat_eq(a2110, a2111) {
        if f182(f121(v0.clone(), {
            let a2112 = v1;
            let a2113 = 1u64;
            nat_add(a2112, a2113)?
        })?) {
            Ok(true)
        } else {
            Ok(f183(f121(v0, {
                let a2114 = v1;
                let a2115 = 1u64;
                nat_add(a2114, a2115)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f274(v0: Bytes) -> R<Bytes> {
    let a2121 = {
        let a2119 = Bytes::lit(&[0x22]);
        let a2120 = {
            let (_, h2116) = {
                let (h2117, _) = f151(Fn3::F363(v0.clone()), {
                    let a2118 = v0;
                    length_bytes(a2118)
                }, (0u64, Bytes::lit(&[])))?;
                h2117
            };
            h2116
        };
        append_bytes(a2119, a2120)
    };
    let a2122 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a2121, a2122))
}

pub fn f275(v0: Fn7, p2126: List<(Bytes, Bytes)>, p2127: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m2125 = (p2126, p2127);
    {
        let mut m2125 = m2125;
        loop {
            let step = {
                let (v1, v2) = m2125;
                let m2123 = v2;
                match m2123.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c2124: Fn7 = v0.clone();
                            c2124.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2125 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f276(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f364(v0, v1)
}

pub fn f277(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f365(v0)
}

pub fn f278(v0: Fn8, p2131: List<Bytes>, p2132: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m2130 = (p2131, p2132);
    {
        let mut m2130 = m2130;
        loop {
            let step = {
                let (v1, v2) = m2130;
                let m2128 = v2;
                match m2128.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c2129: Fn8 = v0.clone();
                            c2129.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2130 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f279(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f366(v1)?, v0))
}

pub fn f280(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    let v2: u64 = {
        let (h2133, _) = v1.clone();
        h2133
    };
    let v3: Bytes = {
        let (h2134, _) = {
            let (_, h2135) = v1.clone();
            h2135
        };
        h2134
    };
    let a2138 = {
        let (_, h2136) = {
            let (_, h2137) = v1;
            h2137
        };
        h2136
    };
    let a2139 = 0u64;
    if nat_eq(a2138, a2139) {
        let v4: u64 = f75(v0.clone(), v2)?;
        let a2140 = v4;
        let a2141 = 34u64;
        if nat_eq(a2140, a2141) {
            Ok(Some(({
                let a2142 = v2;
                let a2143 = 1u64;
                nat_add(a2142, a2143)?
            }, (v3, 1u64))))
        } else {
            let a2144 = v4;
            let a2145 = 92u64;
            if nat_eq(a2144, a2145) {
                f367(v0, v2, v3)
            } else {
                let a2146 = v4;
                let a2147 = 32u64;
                if if nat_lt(a2146, a2147) {
                    true
                } else {
                    let a2148 = v4;
                    let a2149 = 256u64;
                    nat_eq(a2148, a2149)
                } {
                    Ok(Some((v2, (v3, 2u64))))
                } else {
                    Ok(Some(({
                        let a2150 = v2;
                        let a2151 = 1u64;
                        nat_add(a2150, a2151)?
                    }, ({
                        let a2152 = v3;
                        let a2153 = f38(v0, v2, 1u64);
                        append_bytes(a2152, a2153)
                    }, 0u64))))
                }
            }
        }
    } else {
        Ok(None::<(u64, (Bytes, u64))>)
    }
}

pub fn f281(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f75(v0, v1)?;
    let a2154 = 47u64;
    let a2155 = v2;
    if if nat_lt(a2154, a2155) {
        let a2156 = v2;
        let a2157 = 58u64;
        nat_lt(a2156, a2157)
    } else {
        false
    } {
        Ok(Some({
            let a2158 = v1;
            let a2159 = 1u64;
            nat_add(a2158, a2159)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f282(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2160 = f75(v0, v1)?;
    let a2161 = 48u64;
    if nat_eq(a2160, a2161) {
        Ok(Some({
            let a2162 = v1;
            let a2163 = 1u64;
            nat_add(a2162, a2163)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f283(v0: Bytes) -> R<u64> {
    let (h2164, _) = f154(Fn4::F368(v0.clone()), {
        let a2165 = v0;
        length_bytes(a2165)
    }, 0u64)?;
    Ok(h2164)
}

pub fn f284(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m2169 = {
        let a2167 = v0;
        let a2168 = {
            let (h2166, _) = v1;
            h2166
        };
        index_bytes(a2167, a2168)
    };
    match m2169 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2171 = {
                    let (h2170, _) = v1;
                    h2170
                };
                let a2172 = 1u64;
                nat_add(a2171, a2172)?
            }, {
                let a2176 = {
                    let a2174 = {
                        let (_, h2173) = v1;
                        h2173
                    };
                    let a2175 = 10u64;
                    nat_mul(a2174, a2175)?
                };
                let a2177 = f369(v2)?;
                nat_add(a2176, a2177)?
            })))
        }
    }
}

pub fn f285(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f370(v0, v1)
}

pub fn f286(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a2178 = v0;
        utf8_encode(a2178)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f59(v1)?, List::<Bytes>::nil())))))
}

pub fn f287(p2181: List<(u64, Bytes)>, p2182: List<(u64, Bytes)>) -> List<(u64, Bytes)> {
    let m2180 = (p2181, p2182);
    {
        let mut m2180 = m2180;
        loop {
            let step = {
                let (v0, v1) = m2180;
                let m2179 = v0;
                match m2179.uncons() {
                    None => {
                        Err::<(List<(u64, Bytes)>, List<(u64, Bytes)>), List<(u64, Bytes)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(u64, Bytes)>, List<(u64, Bytes)>), List<(u64, Bytes)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2180 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f288(v0: u64) -> bool {
    let a2183 = 65u64;
    let a2184 = v0;
    if if nat_le(a2183, a2184) {
        let a2185 = v0;
        let a2186 = 90u64;
        nat_le(a2185, a2186)
    } else {
        false
    } {
        true
    } else {
        let a2187 = 97u64;
        let a2188 = v0;
        if nat_le(a2187, a2188) {
            let a2189 = v0;
            let a2190 = 122u64;
            nat_le(a2189, a2190)
        } else {
            false
        }
    }
}

pub fn f289(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h2191, _) = f154(Fn4::F371(v1, v0.clone()), {
        let a2193 = {
            let a2192 = v0;
            length_bytes(a2192)
        };
        let a2194 = 1u64;
        nat_add(a2193, a2194)?
    }, v2)?;
    Ok(h2191)
}

pub fn f290(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f106(v0.clone(), v1)?;
    let a2195 = v2;
    let a2196 = 48u64;
    if nat_eq(a2195, a2196) {
        let v3: u64 = f106(v0.clone(), {
            let a2197 = v1;
            let a2198 = 1u64;
            nat_add(a2197, a2198)?
        })?;
        let a2199 = v3;
        let a2200 = 88u64;
        if if if nat_eq(a2199, a2200) {
            true
        } else {
            let a2201 = v3;
            let a2202 = 120u64;
            nat_eq(a2201, a2202)
        } {
            f372(f106(v0.clone(), {
                let a2203 = v1;
                let a2204 = 2u64;
                nat_add(a2203, a2204)?
            })?)
        } else {
            false
        } {
            f289(v0, 1u64, {
                let a2205 = v1;
                let a2206 = 2u64;
                nat_add(a2205, a2206)?
            })
        } else {
            f289(v0, 2u64, {
                let a2207 = v1;
                let a2208 = 1u64;
                nat_add(a2207, a2208)?
            })
        }
    } else {
        if f373(v2) {
            f289(v0, 0u64, v1)
        } else {
            Ok(v1)
        }
    }
}

pub fn f291(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f289(v0.clone(), 0u64, v1)?;
    let a2209 = f106(v0.clone(), v2)?;
    let a2210 = 46u64;
    if nat_eq(a2209, a2210) {
        let v3: u64 = f289(v0.clone(), 0u64, {
            let a2211 = v2;
            let a2212 = 1u64;
            nat_add(a2211, a2212)?
        })?;
        let a2213 = v1;
        let a2214 = v2;
        if if nat_lt(a2213, a2214) {
            true
        } else {
            let a2217 = {
                let a2215 = v2;
                let a2216 = 1u64;
                nat_add(a2215, a2216)?
            };
            let a2218 = v3;
            nat_lt(a2217, a2218)
        } {
            f374(v0, v3)
        } else {
            Ok(v1)
        }
    } else {
        let a2219 = v1;
        let a2220 = v2;
        if nat_lt(a2219, a2220) {
            let v4: u64 = f374(v0, v2)?;
            let a2221 = v2;
            let a2222 = v4;
            if nat_lt(a2221, a2222) {
                Ok(v4)
            } else {
                Ok(v1)
            }
        } else {
            Ok(v1)
        }
    }
}

pub fn f292(v0: List<(u64, Bytes)>, v1: Str) -> bool {
    let m2223 = v0;
    match m2223.uncons() {
        None => {
            false
        }
        Some((v2, _)) => {
            f375(v2, {
                let a2224 = v1;
                utf8_encode(a2224)
            })
        }
    }
}

pub fn f293(p2239: u64, p2240: List<(u64, Bytes)>, p2241: List<Bytes>, v3: bool) -> Result<List<(u64, Bytes)>, Bytes> {
    let m2238 = (p2239, (p2240, p2241));
    {
        let mut m2238 = m2238;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2238;
                let m2225 = v0;
                if m2225 == 0 {
                    Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<List<(u64, Bytes)>, Bytes>>(Err::<List<(u64, Bytes)>, Bytes>(f8(Str::lit("brackets nest deeper than the input is long"))))
                } else {
                    let v4 = m2225 - 1;
                    let m2226 = v1;
                    match m2226.uncons() {
                        None => {
                            Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<List<(u64, Bytes)>, Bytes>>(Err::<List<(u64, Bytes)>, Bytes>(f8(Str::lit("the input ends inside brackets or before the ; that closes a member (Web IDL, IDL grammar)"))))
                        }
                        Some((v5, v6)) => {
                            let a2228 = {
                                let (h2227, _) = v5;
                                h2227
                            };
                            let a2229 = 3u64;
                            if nat_eq(a2228, a2229) {
                                Ok::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<List<(u64, Bytes)>, Bytes>>((v4, (v6, v2)))
                            } else {
                                let v7: Bytes = {
                                    let (_, h2230) = v5;
                                    h2230
                                };
                                let v8: Bytes = f376(v7.clone());
                                if !f41(v8.clone(), Bytes::lit(&[])) {
                                    Ok::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<List<(u64, Bytes)>, Bytes>>((v4, (v6, List::cons(v8, v2))))
                                } else {
                                    if f377(v7.clone()) {
                                        let m2231 = v2;
                                        match m2231.uncons() {
                                            None => {
                                                Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<List<(u64, Bytes)>, Bytes>>(Err::<List<(u64, Bytes)>, Bytes>(f7(List::cons(f8(Str::lit("the closing bracket ")), List::cons(v7, List::cons(f8(Str::lit(" has no opening bracket (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
                                            }
                                            Some((v9, v10)) => {
                                                if f41(v9.clone(), v7.clone()) {
                                                    if if !v3 {
                                                        let a2233 = {
                                                            let a2232 = v10.clone();
                                                            length_list(a2232)
                                                        };
                                                        let a2234 = 0u64;
                                                        nat_eq(a2233, a2234)
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
                                            if f41(v7, f8(Str::lit(";"))) {
                                                let a2236 = {
                                                    let a2235 = v2.clone();
                                                    length_list(a2235)
                                                };
                                                let a2237 = 0u64;
                                                nat_eq(a2236, a2237)
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
                Ok(next) => { m2238 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f294(v0: List<(u64, Bytes)>) -> List<(u64, Bytes)> {
    let m2242 = v0;
    match m2242.uncons() {
        None => {
            List::<(u64, Bytes)>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f295(v0: u64, v1: u64, v2: List<(u64, Bytes)>, v3: bool) -> R<Result<(Adt13, List<(u64, Bytes)>), Bytes>> {
    let m2243 = f298(v2);
    match m2243 {
        Ok(v5) => {
            let v6: bool = if v3 {
                f292({
                    let (_, h2244) = v5.clone();
                    h2244
                }, Str::lit(":"))
            } else {
                false
            };
            let m2247 = if v6 {
                f298(f294({
                    let (_, h2245) = v5;
                    h2245
                }))
            } else {
                Ok::<(Bytes, List<(u64, Bytes)>), Bytes>((Bytes::lit(&[]), {
                    let (_, h2246) = v5;
                    h2246
                }))
            };
            match m2247 {
                Ok(v8) => {
                    let m2249 = f299({
                        let (_, h2248) = v8;
                        h2248
                    }, Str::lit("{"));
                    match m2249 {
                        Ok(v10) => {
                            let m2254 = {
                                let a2250 = v1;
                                let a2251 = 2u64;
                                if if nat_eq(a2250, a2251) {
                                    true
                                } else {
                                    let a2252 = v1;
                                    let a2253 = 7u64;
                                    nat_eq(a2252, a2253)
                                } {
                                    f378(v0, v10, List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil())?
                                } else {
                                    f379(v0, v10, List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil())?
                                }
                            };
                            match m2254 {
                                Ok(v12) => {
                                    Ok(Ok::<(Adt13, List<(u64, Bytes)>), Bytes>((Adt13::C0(v1, {
                                        let (h2255, _) = v5;
                                        h2255
                                    }, {
                                        let (h2256, _) = v8;
                                        h2256
                                    }, {
                                        let (h2257, _) = v12;
                                        h2257
                                    }, List::<Bytes>::nil(), f301(Bytes::lit(&[]))), {
                                        let (_, h2258) = v12;
                                        h2258
                                    })))
                                }
                                Err(v11) => {
                                    Ok(Err::<(Adt13, List<(u64, Bytes)>), Bytes>(v11))
                                }
                            }
                        }
                        Err(v9) => {
                            Ok(Err::<(Adt13, List<(u64, Bytes)>), Bytes>(v9))
                        }
                    }
                }
                Err(v7) => {
                    Ok(Err::<(Adt13, List<(u64, Bytes)>), Bytes>(v7))
                }
            }
        }
        Err(v4) => {
            Ok(Err::<(Adt13, List<(u64, Bytes)>), Bytes>(v4))
        }
    }
}

pub fn f296(v0: List<(u64, Bytes)>, v1: Bytes) -> R<Result<(Adt13, List<(u64, Bytes)>), Bytes>> {
    let m2259 = f298(v0);
    match m2259 {
        Ok(v3) => {
            let m2261 = f380({
                let (_, h2260) = v3;
                h2260
            })?;
            match m2261 {
                Ok(v5) => {
                    Ok(Ok::<(Adt13, List<(u64, Bytes)>), Bytes>((Adt13::C0(9u64, {
                        let (h2262, _) = v3;
                        h2262
                    }, Bytes::lit(&[]), List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil(), List::cons(v1, List::<Bytes>::nil()), f301(Bytes::lit(&[]))), v5)))
                }
                Err(v4) => {
                    Ok(Err::<(Adt13, List<(u64, Bytes)>), Bytes>(v4))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<(Adt13, List<(u64, Bytes)>), Bytes>(v2))
        }
    }
}

pub fn f297(v0: Str, v1: List<(u64, Bytes)>) -> Bytes {
    f7(List::cons(f8(Str::lit("expected ")), List::cons({
        let a2263 = v0;
        utf8_encode(a2263)
    }, List::cons(f8(Str::lit(" but found ")), List::cons(f381(v1), List::cons(f8(Str::lit(" (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
}

pub fn f298(v0: List<(u64, Bytes)>) -> Result<(Bytes, List<(u64, Bytes)>), Bytes> {
    let m2264 = v0.clone();
    match m2264.uncons() {
        None => {
            Err::<(Bytes, List<(u64, Bytes)>), Bytes>(f297(Str::lit("an identifier"), v0))
        }
        Some((v1, v2)) => {
            let a2266 = {
                let (h2265, _) = v1;
                h2265
            };
            let a2267 = 0u64;
            if nat_eq(a2266, a2267) {
                Ok::<(Bytes, List<(u64, Bytes)>), Bytes>((f382({
                    let (_, h2268) = v1;
                    h2268
                }), v2))
            } else {
                Err::<(Bytes, List<(u64, Bytes)>), Bytes>(f297(Str::lit("an identifier"), v0))
            }
        }
    }
}

pub fn f299(v0: List<(u64, Bytes)>, v1: Str) -> Result<List<(u64, Bytes)>, Bytes> {
    if f292(v0.clone(), v1.clone()) {
        Ok::<List<(u64, Bytes)>, Bytes>(f294(v0))
    } else {
        Err::<List<(u64, Bytes)>, Bytes>(f297(v1, v0))
    }
}

pub fn f300(p2280: u64, p2281: List<(u64, Bytes)>, p2282: List<Bytes>) -> Result<(List<Bytes>, List<(u64, Bytes)>), Bytes> {
    let m2279 = (p2280, (p2281, p2282));
    {
        let mut m2279 = m2279;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2279;
                let m2269 = v0;
                if m2269 == 0 {
                    Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Err::<(List<Bytes>, List<(u64, Bytes)>), Bytes>(f8(Str::lit("an enumeration has more values than the input has tokens"))))
                } else {
                    let v3 = m2269 - 1;
                    let m2270 = v1.clone();
                    match m2270.uncons() {
                        None => {
                            Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Err::<(List<Bytes>, List<(u64, Bytes)>), Bytes>(f297(Str::lit("a string"), v1)))
                        }
                        Some((v4, v5)) => {
                            let a2272 = {
                                let (h2271, _) = v4;
                                h2271
                            };
                            let a2273 = 3u64;
                            if !nat_eq(a2272, a2273) {
                                Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Err::<(List<Bytes>, List<(u64, Bytes)>), Bytes>(f297(Str::lit("a string"), v1)))
                            } else {
                                if f292(v5.clone(), Str::lit(",")) {
                                    if f292(f294(v5.clone()), Str::lit("}")) {
                                        let m2274 = f299(f294(f294(v5)), Str::lit(";"));
                                        match m2274 {
                                            Ok(v7) => {
                                                Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Ok::<(List<Bytes>, List<(u64, Bytes)>), Bytes>((f52(List::cons({
                                                    let (_, h2275) = v4;
                                                    h2275
                                                }, v2)), v7)))
                                            }
                                            Err(v6) => {
                                                Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Err::<(List<Bytes>, List<(u64, Bytes)>), Bytes>(v6))
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>((v3, (f294(v5), List::cons({
                                            let (_, h2276) = v4;
                                            h2276
                                        }, v2))))
                                    }
                                } else {
                                    if f292(v5.clone(), Str::lit("}")) {
                                        let m2277 = f299(f294(v5), Str::lit(";"));
                                        match m2277 {
                                            Ok(v9) => {
                                                Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Ok::<(List<Bytes>, List<(u64, Bytes)>), Bytes>((f52(List::cons({
                                                    let (_, h2278) = v4;
                                                    h2278
                                                }, v2)), v9)))
                                            }
                                            Err(v8) => {
                                                Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Err::<(List<Bytes>, List<(u64, Bytes)>), Bytes>(v8))
                                            }
                                        }
                                    } else {
                                        Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Err::<(List<Bytes>, List<(u64, Bytes)>), Bytes>(f297(Str::lit(", or }"), v5)))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2279 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f301(v0: Bytes) -> (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) {
    (0u64, (v0, (List::<Bytes>::nil(), (Bytes::lit(&[]), Bytes::lit(&[])))))
}

pub fn f302(v0: u64, v1: List<(u64, Bytes)>, v2: u64) -> R<Result<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>> {
    let m2283 = v0;
    if m2283 == 0 {
        Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f8(Str::lit("a type nests deeper than the input is long"))))
    } else {
        let v3 = m2283 - 1;
        let a2284 = v2;
        let a2285 = 0u64;
        if nat_eq(a2284, a2285) {
            let m2286 = f217(v1)?;
            match m2286 {
                Ok(v5) => {
                    if f292(v5.clone(), Str::lit("(")) {
                        let m2287 = f302(v3, f294(v5), 1u64)?;
                        match m2287 {
                            Ok(v7) => {
                                if f292({
                                    let (_, h2288) = v7.clone();
                                    h2288
                                }, Str::lit("?")) {
                                    Ok(Ok::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(((0u64, (Bytes::lit(&[]), (f237({
                                        let (h2289, _) = v7;
                                        h2289
                                    }), (f8(Str::lit("a nullable union type is not translated yet, because the typedef identifier names only the union (Web IDL 2.13.32)")), Bytes::lit(&[]))))), f294({
                                        let (_, h2290) = v7;
                                        h2290
                                    }))))
                                } else {
                                    Ok(Ok::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(({
                                        let (h2291, _) = v7;
                                        h2291
                                    }, {
                                        let (_, h2292) = v7;
                                        h2292
                                    })))
                                }
                            }
                            Err(v6) => {
                                Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(v6))
                            }
                        }
                    } else {
                        let m2293 = v5.clone();
                        match m2293.uncons() {
                            None => {
                                Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f297(Str::lit("a type"), v5)))
                            }
                            Some((v8, v9)) => {
                                let v10: Bytes = {
                                    let (_, h2294) = v8;
                                    h2294
                                };
                                let a2296 = {
                                    let (h2295, _) = v8;
                                    h2295
                                };
                                let a2297 = 0u64;
                                if !nat_eq(a2296, a2297) {
                                    Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f297(Str::lit("a type"), v5)))
                                } else {
                                    if if f41(v10.clone(), f8(Str::lit("sequence"))) {
                                        true
                                    } else {
                                        if f41(v10.clone(), f8(Str::lit("FrozenArray"))) {
                                            true
                                        } else {
                                            f41(v10.clone(), f8(Str::lit("ObservableArray")))
                                        }
                                    } {
                                        let m2298 = f299(v9, Str::lit("<"));
                                        match m2298 {
                                            Ok(v12) => {
                                                let m2299 = f302(v3, v12, 0u64)?;
                                                match m2299 {
                                                    Ok(v14) => {
                                                        let m2301 = f299({
                                                            let (_, h2300) = v14;
                                                            h2300
                                                        }, Str::lit(">"));
                                                        match m2301 {
                                                            Ok(v16) => {
                                                                Ok(f383(f384(Str::lit("List"), {
                                                                    let (h2302, _) = v14;
                                                                    h2302
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
                                        if f41(v10.clone(), f8(Str::lit("async_sequence"))) {
                                            let m2303 = f299(v9, Str::lit("<"));
                                            match m2303 {
                                                Ok(v18) => {
                                                    let m2304 = f302(v3, v18, 0u64)?;
                                                    match m2304 {
                                                        Ok(v20) => {
                                                            let m2306 = f299({
                                                                let (_, h2305) = v20;
                                                                h2305
                                                            }, Str::lit(">"));
                                                            match m2306 {
                                                                Ok(v22) => {
                                                                    Ok(f383(f385(f8(Str::lit("async_sequence types are not translated, because an async sequence is produced over time and is not a value (Web IDL 2.13.29)"))), v22))
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
                                            if f41(v10.clone(), f8(Str::lit("Promise"))) {
                                                let m2307 = f299(v9, Str::lit("<"));
                                                match m2307 {
                                                    Ok(v24) => {
                                                        let m2308 = f302(v3, v24, 0u64)?;
                                                        match m2308 {
                                                            Ok(v26) => {
                                                                let m2310 = f299({
                                                                    let (_, h2309) = v26;
                                                                    h2309
                                                                }, Str::lit(">"));
                                                                match m2310 {
                                                                    Ok(v28) => {
                                                                        Ok(f383(f385(f8(Str::lit("Promise types are not translated, because a promise is a deferred result, not a value (Web IDL 2.13.31)"))), v28))
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
                                                if f41(v10.clone(), f8(Str::lit("record"))) {
                                                    let m2311 = f299(v9, Str::lit("<"));
                                                    match m2311 {
                                                        Ok(v30) => {
                                                            let m2312 = v30.clone();
                                                            match m2312.uncons() {
                                                                None => {
                                                                    Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f297(Str::lit("a string type"), v30)))
                                                                }
                                                                Some((v31, v32)) => {
                                                                    if if f41({
                                                                        let (_, h2313) = v31.clone();
                                                                        h2313
                                                                    }, f8(Str::lit("DOMString"))) {
                                                                        true
                                                                    } else {
                                                                        if f41({
                                                                            let (_, h2314) = v31.clone();
                                                                            h2314
                                                                        }, f8(Str::lit("USVString"))) {
                                                                            true
                                                                        } else {
                                                                            f41({
                                                                                let (_, h2315) = v31.clone();
                                                                                h2315
                                                                            }, f8(Str::lit("ByteString")))
                                                                        }
                                                                    } {
                                                                        let m2316 = f299(v32, Str::lit(","));
                                                                        match m2316 {
                                                                            Ok(v34) => {
                                                                                let m2317 = f302(v3, v34, 0u64)?;
                                                                                match m2317 {
                                                                                    Ok(v36) => {
                                                                                        let m2319 = f299({
                                                                                            let (_, h2318) = v36;
                                                                                            h2318
                                                                                        }, Str::lit(">"));
                                                                                        match m2319 {
                                                                                            Ok(v38) => {
                                                                                                let m2321 = f386({
                                                                                                    let (_, h2320) = v31;
                                                                                                    h2320
                                                                                                });
                                                                                                match m2321 {
                                                                                                    None => {
                                                                                                        Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f297(Str::lit("a string type"), v30)))
                                                                                                    }
                                                                                                    Some(v39) => {
                                                                                                        let v40: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) = f387({
                                                                                                            let (h2322, _) = v36;
                                                                                                            h2322
                                                                                                        });
                                                                                                        Ok(f383((0u64, (f7(List::cons(f8(Str::lit("(List (")), List::cons(f236(v39), List::cons(f8(Str::lit(" * ")), List::cons(f236(v40.clone()), List::cons(f8(Str::lit("))")), List::<Bytes>::nil())))))), (f237(v40.clone()), (f234(v40), Bytes::lit(&[]))))), v38))
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
                                                                        Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f297(Str::lit("DOMString, USVString or ByteString"), v30)))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Err(v29) => {
                                                            Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(v29))
                                                        }
                                                    }
                                                } else {
                                                    if f41(v10.clone(), f8(Str::lit("unsigned"))) {
                                                        if f292(v9.clone(), Str::lit("short")) {
                                                            Ok(f383(f301(f8(Str::lit("Nat"))), f294(v9)))
                                                        } else {
                                                            if f292(v9.clone(), Str::lit("long")) {
                                                                let v41: List<(u64, Bytes)> = f294(v9);
                                                                Ok(f383(f301(f8(Str::lit("Nat"))), if f292(v41.clone(), Str::lit("long")) {
                                                                    f294(v41)
                                                                } else {
                                                                    v41
                                                                }))
                                                            } else {
                                                                Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f297(Str::lit("short or long"), v9)))
                                                            }
                                                        }
                                                    } else {
                                                        if f41(v10.clone(), f8(Str::lit("short"))) {
                                                            Ok(f383(f301(f8(Str::lit("Int"))), v9))
                                                        } else {
                                                            if f41(v10.clone(), f8(Str::lit("long"))) {
                                                                Ok(f383(f301(f8(Str::lit("Int"))), if f292(v9.clone(), Str::lit("long")) {
                                                                    f294(v9)
                                                                } else {
                                                                    v9
                                                                }))
                                                            } else {
                                                                if f41(v10.clone(), f8(Str::lit("unrestricted"))) {
                                                                    if if f292(v9.clone(), Str::lit("float")) {
                                                                        true
                                                                    } else {
                                                                        f292(v9.clone(), Str::lit("double"))
                                                                    } {
                                                                        Ok(f383(f385(f8(Str::lit("unrestricted float and unrestricted double are not translated, because its value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.13 and 2.13.15)"))), f294(v9)))
                                                                    } else {
                                                                        Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f297(Str::lit("float or double"), v9)))
                                                                    }
                                                                } else {
                                                                    let m2323 = f386(v10.clone());
                                                                    match m2323 {
                                                                        None => {
                                                                            let v42: Bytes = f382(v10);
                                                                            Ok(f383((0u64, (f177(v42.clone())?, (List::cons(v42.clone(), List::<Bytes>::nil()), (Bytes::lit(&[]), v42)))), v9))
                                                                        }
                                                                        Some(v43) => {
                                                                            Ok(f383(v43, v9))
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
            let m2324 = f302(v3, v1, 0u64)?;
            match m2324 {
                Ok(v45) => {
                    let v46: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) = {
                        let (h2325, _) = v45.clone();
                        h2325
                    };
                    let v48: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) = {
                        let a2326 = f235(v46.clone());
                        let a2327 = 0u64;
                        if nat_eq(a2326, a2327) {
                            if f41(f234(v46.clone()), Bytes::lit(&[])) {
                                if f41(f388(v46.clone()), Bytes::lit(&[])) {
                                    (1u64, (Bytes::lit(&[]), (f237(v46), (f8(Str::lit("a union member type that is not a named definition gives its constructor no name, so the union is not translated yet (Web IDL 2.13.32)")), Bytes::lit(&[])))))
                                } else {
                                    let m2328 = f315(false, f388(v46.clone()))?;
                                    match m2328 {
                                        None => {
                                            (1u64, (Bytes::lit(&[]), (f237(v46.clone()), (f7(List::cons(f8(Str::lit("the union member ")), List::cons(f388(v46), List::cons(f8(Str::lit(" has no constructor name")), List::<Bytes>::nil())))), Bytes::lit(&[])))))
                                        }
                                        Some(v47) => {
                                            if f220(v47.clone()) {
                                                (1u64, (Bytes::lit(&[]), (f237(v46), (f221(v47), Bytes::lit(&[])))))
                                            } else {
                                                (1u64, (f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v47, List::cons(f8(Str::lit(", fields: [")), List::cons(f236(v46.clone()), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), (f237(v46), (Bytes::lit(&[]), Bytes::lit(&[])))))
                                            }
                                        }
                                    }
                                }
                            } else {
                                (1u64, (Bytes::lit(&[]), (f237(v46.clone()), (f234(v46), Bytes::lit(&[])))))
                            }
                        } else {
                            v46
                        }
                    };
                    if f292({
                        let (_, h2329) = v45.clone();
                        h2329
                    }, Str::lit("or")) {
                        let m2331 = f302(v3, f294({
                            let (_, h2330) = v45;
                            h2330
                        }), 2u64)?;
                        match m2331 {
                            Ok(v50) => {
                                Ok(Ok::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(((1u64, (if f41(f236(v48.clone()), Bytes::lit(&[])) {
                                    f236({
                                        let (h2332, _) = v50.clone();
                                        h2332
                                    })
                                } else {
                                    if f41(f236({
                                        let (h2333, _) = v50.clone();
                                        h2333
                                    }), Bytes::lit(&[])) {
                                        f236(v48.clone())
                                    } else {
                                        f7(List::cons(f236(v48.clone()), List::cons(f8(Str::lit(",\u{a}")), List::cons(f236({
                                            let (h2334, _) = v50.clone();
                                            h2334
                                        }), List::<Bytes>::nil()))))
                                    }
                                }, (f145(f237(v48.clone()), f237({
                                    let (h2335, _) = v50.clone();
                                    h2335
                                })), (if f41(f234(v48.clone()), Bytes::lit(&[])) {
                                    f234({
                                        let (h2336, _) = v50;
                                        h2336
                                    })
                                } else {
                                    f234(v48)
                                }, Bytes::lit(&[]))))), {
                                    let (_, h2337) = v50;
                                    h2337
                                })))
                            }
                            Err(v49) => {
                                Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(v49))
                            }
                        }
                    } else {
                        let a2338 = v2;
                        let a2339 = 2u64;
                        if if nat_eq(a2338, a2339) {
                            f292({
                                let (_, h2340) = v45.clone();
                                h2340
                            }, Str::lit(")"))
                        } else {
                            false
                        } {
                            Ok(Ok::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>((v48, f294({
                                let (_, h2341) = v45;
                                h2341
                            }))))
                        } else {
                            Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>({
                                let a2342 = v2;
                                let a2343 = 1u64;
                                if nat_eq(a2342, a2343) {
                                    f297(Str::lit("or"), {
                                        let (_, h2344) = v45;
                                        h2344
                                    })
                                } else {
                                    f297(Str::lit("or or )"), {
                                        let (_, h2345) = v45;
                                        h2345
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

pub fn f303() -> List<Bytes> {
    List::cons(f8(Str::lit("Prop")), List::cons(f8(Str::lit("Sort")), List::cons(f8(Str::lit("StateRefT")), List::cons(f8(Str::lit("Type")), List::cons(f8(Str::lit("abbrev")), List::cons(f8(Str::lit("at")), List::cons(f8(Str::lit("attribute")), List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("bif")), List::cons(f8(Str::lit("break")), List::cons(f8(Str::lit("by")), List::cons(f8(Str::lit("calc")), List::cons(f8(Str::lit("catch")), List::cons(f8(Str::lit("class")), List::cons(f8(Str::lit("coinductive")), List::cons(f8(Str::lit("continue")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f304() -> List<Bytes> {
    List::cons(f8(Str::lit("def")), List::cons(f8(Str::lit("deriving")), List::cons(f8(Str::lit("do")), List::cons(f8(Str::lit("dsimproc")), List::cons(f8(Str::lit("elab")), List::cons(f8(Str::lit("else")), List::cons(f8(Str::lit("end")), List::cons(f8(Str::lit("example")), List::cons(f8(Str::lit("exists")), List::cons(f8(Str::lit("export")), List::cons(f8(Str::lit("extends")), List::cons(f8(Str::lit("finally")), List::cons(f8(Str::lit("for")), List::cons(f8(Str::lit("forall")), List::cons(f8(Str::lit("from")), List::cons(f8(Str::lit("fun")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f305() -> List<Bytes> {
    List::cons(f8(Str::lit("generalizing")), List::cons(f8(Str::lit("have")), List::cons(f8(Str::lit("haveI")), List::cons(f8(Str::lit("hiding")), List::cons(f8(Str::lit("idbg")), List::cons(f8(Str::lit("if")), List::cons(f8(Str::lit("import")), List::cons(f8(Str::lit("in")), List::cons(f8(Str::lit("include")), List::cons(f8(Str::lit("inductive")), List::cons(f8(Str::lit("inferInstanceAs")), List::cons(f8(Str::lit("infix")), List::cons(f8(Str::lit("infixl")), List::cons(f8(Str::lit("infixr")), List::cons(f8(Str::lit("initialize")), List::cons(f8(Str::lit("instance")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f306() -> List<Bytes> {
    List::cons(f8(Str::lit("let")), List::cons(f8(Str::lit("letI")), List::cons(f8(Str::lit("local")), List::cons(f8(Str::lit("logNamedError")), List::cons(f8(Str::lit("logNamedErrorAt")), List::cons(f8(Str::lit("logNamedWarning")), List::cons(f8(Str::lit("logNamedWarningAt")), List::cons(f8(Str::lit("macro")), List::cons(f8(Str::lit("match")), List::cons(f8(Str::lit("matches")), List::cons(f8(Str::lit("meta")), List::cons(f8(Str::lit("mut")), List::cons(f8(Str::lit("mutual")), List::cons(f8(Str::lit("namespace")), List::cons(f8(Str::lit("nofun")), List::cons(f8(Str::lit("nomatch")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f307() -> List<Bytes> {
    List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("nonrec")), List::cons(f8(Str::lit("notation")), List::cons(f8(Str::lit("omit")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("open")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("postfix")), List::cons(f8(Str::lit("prefix")), List::cons(f8(Str::lit("private")), List::cons(f8(Str::lit("protected")), List::cons(f8(Str::lit("public")), List::cons(f8(Str::lit("renaming")), List::cons(f8(Str::lit("repeat")), List::cons(f8(Str::lit("reprove")), List::cons(f8(Str::lit("return")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f308() -> List<Bytes> {
    List::cons(f8(Str::lit("scoped")), List::cons(f8(Str::lit("seal")), List::cons(f8(Str::lit("section")), List::cons(f8(Str::lit("show")), List::cons(f8(Str::lit("simproc")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("structure")), List::cons(f8(Str::lit("suffices")), List::cons(f8(Str::lit("syntax")), List::cons(f8(Str::lit("then")), List::cons(f8(Str::lit("theorem")), List::cons(f8(Str::lit("throwError")), List::cons(f8(Str::lit("throwErrorAt")), List::cons(f8(Str::lit("throwNamedError")), List::cons(f8(Str::lit("throwNamedErrorAt")), List::cons(f8(Str::lit("try")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f309() -> List<Bytes> {
    List::cons(f8(Str::lit("universe")), List::cons(f8(Str::lit("unless")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("unseal")), List::cons(f8(Str::lit("until")), List::cons(f8(Str::lit("using")), List::cons(f8(Str::lit("variable")), List::cons(f8(Str::lit("where")), List::cons(f8(Str::lit("while")), List::cons(f8(Str::lit("with")), List::cons(f8(Str::lit("reprove")), List::cons(f8(Str::lit("throwError")), List::cons(f8(Str::lit("throwErrorAt")), List::<Bytes>::nil())))))))))))))
}

pub fn f310(v0: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> R<List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>> {
    let m2346 = v0;
    match m2346.uncons() {
        None => {
            Ok(List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil())
        }
        Some((v1, v2)) => {
            f389(v1, f310(v2)?)
        }
    }
}

pub fn f311(v0: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, v1: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))> {
    let m2347 = v0;
    match m2347.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f311(v3, v1))
        }
    }
}

pub fn f312(v0: Adt13) -> List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))> {
    let m2348 = v0;
    match m2348 {
        Adt13::C0(_, _, _, v4, _, _) => {
            v4
        }
    }
}

pub fn f313(v0: List<Adt13>, v1: u64, v2: Bytes) -> List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))> {
    let m2349 = v0;
    match m2349.uncons() {
        None => {
            List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil()
        }
        Some((v3, v4)) => {
            let a2350 = f140(v3.clone());
            let a2351 = v1;
            if if nat_eq(a2350, a2351) {
                f41(f142(v3.clone()), v2.clone())
            } else {
                false
            } {
                f311(f312(v3), f313(v4, v1, v2))
            } else {
                f313(v4, v1, v2)
            }
        }
    }
}

pub fn f314(v0: List<Adt13>, v1: List<Adt13>, v2: Bytes) -> List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))> {
    let m2352 = v0;
    match m2352.uncons() {
        None => {
            List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil()
        }
        Some((v3, v4)) => {
            let a2353 = f140(v3.clone());
            let a2354 = 8u64;
            if if nat_eq(a2353, a2354) {
                f41(f142(v3.clone()), v2.clone())
            } else {
                false
            } {
                f311(f390(v1.clone(), f149(v3)), f314(v4, v1, v2))
            } else {
                f314(v4, v1, v2)
            }
        }
    }
}

pub fn f315(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f391(v1)
    } else {
        f317(v1)
    }
}

pub fn f316(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m2355 = v1;
    match m2355.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m2356 = f315(v0, v2)?;
            match m2356 {
                None => {
                    f316(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f316(v0, v3)?))
                }
            }
        }
    }
}

pub fn f317(v0: Bytes) -> R<Option<Bytes>> {
    if f392({
        let a2357 = v0.clone();
        length_bytes(a2357)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f393(v0)?;
        if f394(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m2360 = {
                let a2358 = v1.clone();
                let a2359 = 0u64;
                index_bytes(a2358, a2359)
            };
            match m2360 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f335(f46(v2)?) {
                        Ok(Some({
                            let a2361 = f8(Str::lit("n"));
                            let a2362 = v1;
                            append_bytes(a2361, a2362)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f395({
            let a2363 = v0.clone();
            length_bytes(a2363)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f318(v0: bool, v1: Adt2) -> R<Option<Bytes>> {
    let m2364 = v1;
    match m2364 {
        Adt2::C0(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C1(v7, v8) => {
            Ok(Some({
                let a2375 = if f77(v7, {
                    let a2373 = 0u64;
                    let a2374 = v8;
                    nat_lt(a2373, a2374)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a2376 = f199(false, v8)?;
                append_bytes(a2375, a2376)
            }))
        }
        Adt2::C2(v3, v4, v5) => {
            let m2367 = f391({
                let a2365 = f8(Str::lit("n"));
                let a2366 = f200(false, v4, v5)?;
                append_bytes(a2365, a2366)
            })?;
            match m2367 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a2371 = f8(Str::lit("nMinus"));
                        let a2372 = f38(v6.clone(), 1u64, {
                            let a2369 = {
                                let a2368 = v6;
                                length_bytes(a2368)
                            };
                            let a2370 = 1u64;
                            nat_sub(a2369, a2370)
                        });
                        append_bytes(a2371, a2372)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt2::C3(v2) => {
            f315(v0, v2)
        }
        Adt2::C4(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C5(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C6 => {
            Ok(None::<Bytes>)
        }
        Adt2::C7(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt2::C8(_, _, _) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f319(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m2379 = {
        let a2377 = v0.clone();
        let a2378 = v1;
        index_bytes(a2377, a2378)
    };
    match m2379 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f46(v2)?;
            let a2380 = v3;
            let a2381 = 128u64;
            if nat_lt(a2380, a2381) {
                Ok(Some((v3, 1u64)))
            } else {
                if f77({
                    let a2382 = 194u64;
                    let a2383 = v3;
                    nat_le(a2382, a2383)
                }, {
                    let a2384 = v3;
                    let a2385 = 223u64;
                    nat_le(a2384, a2385)
                }) {
                    let m2388 = f396(v0, {
                        let a2386 = v1;
                        let a2387 = 1u64;
                        nat_add(a2386, a2387)?
                    })?;
                    match m2388 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f397({
                                let a2393 = {
                                    let a2391 = {
                                        let a2389 = v3;
                                        let a2390 = 192u64;
                                        nat_sub(a2389, a2390)
                                    };
                                    let a2392 = 64u64;
                                    nat_mul(a2391, a2392)?
                                };
                                let a2394 = v4;
                                nat_add(a2393, a2394)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f77({
                        let a2395 = 224u64;
                        let a2396 = v3;
                        nat_le(a2395, a2396)
                    }, {
                        let a2397 = v3;
                        let a2398 = 239u64;
                        nat_le(a2397, a2398)
                    }) {
                        let m2401 = f396(v0.clone(), {
                            let a2399 = v1;
                            let a2400 = 1u64;
                            nat_add(a2399, a2400)?
                        })?;
                        match m2401 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m2404 = f396(v0, {
                                    let a2402 = v1;
                                    let a2403 = 2u64;
                                    nat_add(a2402, a2403)?
                                })?;
                                match m2404 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f397({
                                            let a2413 = {
                                                let a2407 = {
                                                    let a2405 = v3;
                                                    let a2406 = 224u64;
                                                    nat_sub(a2405, a2406)
                                                };
                                                let a2408 = 4096u64;
                                                nat_mul(a2407, a2408)?
                                            };
                                            let a2414 = {
                                                let a2411 = {
                                                    let a2409 = v5;
                                                    let a2410 = 64u64;
                                                    nat_mul(a2409, a2410)?
                                                };
                                                let a2412 = v6;
                                                nat_add(a2411, a2412)?
                                            };
                                            nat_add(a2413, a2414)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f77({
                            let a2415 = 240u64;
                            let a2416 = v3;
                            nat_le(a2415, a2416)
                        }, {
                            let a2417 = v3;
                            let a2418 = 244u64;
                            nat_le(a2417, a2418)
                        }) {
                            let m2421 = f396(v0.clone(), {
                                let a2419 = v1;
                                let a2420 = 1u64;
                                nat_add(a2419, a2420)?
                            })?;
                            match m2421 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m2424 = f396(v0.clone(), {
                                        let a2422 = v1;
                                        let a2423 = 2u64;
                                        nat_add(a2422, a2423)?
                                    })?;
                                    match m2424 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m2427 = f396(v0, {
                                                let a2425 = v1;
                                                let a2426 = 3u64;
                                                nat_add(a2425, a2426)?
                                            })?;
                                            match m2427 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f397({
                                                        let a2440 = {
                                                            let a2430 = {
                                                                let a2428 = v3;
                                                                let a2429 = 240u64;
                                                                nat_sub(a2428, a2429)
                                                            };
                                                            let a2431 = 262144u64;
                                                            nat_mul(a2430, a2431)?
                                                        };
                                                        let a2441 = {
                                                            let a2438 = {
                                                                let a2432 = v7;
                                                                let a2433 = 4096u64;
                                                                nat_mul(a2432, a2433)?
                                                            };
                                                            let a2439 = {
                                                                let a2436 = {
                                                                    let a2434 = v8;
                                                                    let a2435 = 64u64;
                                                                    nat_mul(a2434, a2435)?
                                                                };
                                                                let a2437 = v9;
                                                                nat_add(a2436, a2437)?
                                                            };
                                                            nat_add(a2438, a2439)?
                                                        };
                                                        nat_add(a2440, a2441)?
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

pub fn f320(v0: u64) -> R<bool> {
    f242(v0, f398())
}

pub fn f321(v0: u64, v1: Bytes, p2452: u64, p2453: u64, p2454: bool) -> R<bool> {
    let m2451 = (p2452, (p2453, p2454));
    {
        let mut m2451 = m2451;
        loop {
            let step = {
                let (v2, (v3, v4)) = m2451;
                let m2442 = v3;
                if m2442 == 0 {
                    Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                } else {
                    let v5 = m2442 - 1;
                    let v6: u64 = f399(v1.clone(), v2)?;
                    if v4 {
                        let a2443 = v0;
                        let a2444 = v6;
                        if nat_lt(a2443, a2444) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a2445 = v2;
                                let a2446 = 1u64;
                                nat_add(a2445, a2446)?
                            }, (v5, false)))
                        }
                    } else {
                        let a2447 = v0;
                        let a2448 = v6;
                        if nat_le(a2447, a2448) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(true))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a2449 = v2;
                                let a2450 = 1u64;
                                nat_add(a2449, a2450)?
                            }, (v5, true)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2451 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f322(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    if f98(v0.clone(), v1, v2) {
        Ok(None::<u64>)
    } else {
        let a2456 = v2;
        let a2457 = {
            let a2455 = v0;
            length_bytes(a2455)
        };
        if nat_lt(a2456, a2457) {
            Ok(Some({
                let a2458 = v2;
                let a2459 = 1u64;
                nat_add(a2458, a2459)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f323(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f156(f106(v0, v1)?)? {
        Ok(Some({
            let a2460 = v1;
            let a2461 = 1u64;
            nat_add(a2460, a2461)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f324(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let m2462 = f319(v0, v2)?;
    match m2462 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            let a2463 = v2;
            let a2464 = v1;
            if if nat_eq(a2463, a2464) {
                f400({
                    let (h2465, _) = v3;
                    h2465
                })?
            } else {
                f401({
                    let (h2466, _) = v3;
                    h2466
                })?
            } {
                Ok(Some({
                    let a2468 = v2;
                    let a2469 = {
                        let (_, h2467) = v3;
                        h2467
                    };
                    nat_add(a2468, a2469)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f325(v0: Bytes, v1: bool, v2: u64) -> R<(u64, u64)> {
    let (h2470, _) = f211(Fn6::F402(v1, v0.clone()), {
        let a2472 = {
            let a2471 = v0;
            length_bytes(a2471)
        };
        let a2473 = 1u64;
        nat_add(a2472, a2473)?
    }, (v2, 0u64))?;
    Ok(h2470)
}

pub fn f326(v0: u64) -> R<Bytes> {
    let a2474 = v0;
    let a2475 = 128u64;
    if nat_lt(a2474, a2475) {
        Ok(f403(f404(), v0))
    } else {
        let a2476 = v0;
        let a2477 = 2048u64;
        if nat_lt(a2476, a2477) {
            let a2485 = f403(f405(), {
                let a2483 = {
                    let a2481 = {
                        let a2478 = v0;
                        let a2479 = 64u64;
                        let a2480 = 0u64;
                        nat_quot(a2478, a2479, a2480)
                    };
                    let a2482 = 2u64;
                    nat_sub(a2481, a2482)
                };
                let a2484 = 2u64;
                nat_mul(a2483, a2484)?
            });
            let a2486 = f406(v0)?;
            Ok(append_bytes(a2485, a2486))
        } else {
            let a2487 = v0;
            let a2488 = 65536u64;
            if nat_lt(a2487, a2488) {
                let a2499 = f403(f407(), {
                    let a2492 = {
                        let a2489 = v0;
                        let a2490 = 4096u64;
                        let a2491 = 0u64;
                        nat_quot(a2489, a2490, a2491)
                    };
                    let a2493 = 3u64;
                    nat_mul(a2492, a2493)?
                });
                let a2500 = {
                    let a2497 = f406({
                        let a2494 = v0;
                        let a2495 = 64u64;
                        let a2496 = 0u64;
                        nat_quot(a2494, a2495, a2496)
                    })?;
                    let a2498 = f406(v0)?;
                    append_bytes(a2497, a2498)
                };
                Ok(append_bytes(a2499, a2500))
            } else {
                let a2514 = f403(Bytes::lit(&[0xf0, 0xf1, 0xf2, 0xf3, 0xf4]), {
                    let a2501 = v0;
                    let a2502 = 262144u64;
                    let a2503 = 0u64;
                    nat_quot(a2501, a2502, a2503)
                });
                let a2515 = {
                    let a2512 = f406({
                        let a2504 = v0;
                        let a2505 = 4096u64;
                        let a2506 = 0u64;
                        nat_quot(a2504, a2505, a2506)
                    })?;
                    let a2513 = {
                        let a2510 = f406({
                            let a2507 = v0;
                            let a2508 = 64u64;
                            let a2509 = 0u64;
                            nat_quot(a2507, a2508, a2509)
                        })?;
                        let a2511 = f406(v0)?;
                        append_bytes(a2510, a2511)
                    };
                    append_bytes(a2512, a2513)
                };
                Ok(append_bytes(a2514, a2515))
            }
        }
    }
}

pub fn f327(v0: bool, v1: u64, v2: Bytes, v3: u64) -> R<Option<u64>> {
    f408(v2, v0, v1, v3)
}

pub fn f328(v0: Bytes) -> Result<Option<Adt16>, Bytes> {
    Err::<Option<Adt16>, Bytes>(v0)
}

pub fn f329(p2519: Bytes, p2520: u64) -> R<Result<Bytes, Bytes>> {
    let m2518 = (p2519, p2520);
    {
        let mut m2518 = m2518;
        loop {
            let step = {
                let (v0, v1) = m2518;
                let m2516 = v1;
                if m2516 == 0 {
                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f409(f8(Str::lit("the built-in datatype hierarchy is deeper than XSD 1.1 Part 2 section 3 defines")))))
                } else {
                    let v2 = m2516 - 1;
                    if f197(v0.clone(), List::cons(f8(Str::lit("NMTOKENS")), List::cons(f8(Str::lit("IDREFS")), List::cons(f8(Str::lit("ENTITIES")), List::<Bytes>::nil())))) {
                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("(List Bytes)")))))
                    } else {
                        if f41(v0.clone(), f8(Str::lit("nonNegativeInteger"))) {
                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Nat")))))
                        } else {
                            if f41(v0.clone(), f8(Str::lit("integer"))) {
                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Int")))))
                            } else {
                                if f41(v0.clone(), f8(Str::lit("decimal"))) {
                                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f410())))
                                } else {
                                    if f41(v0.clone(), f8(Str::lit("boolean"))) {
                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Bool")))))
                                    } else {
                                        if f197(v0.clone(), List::cons(f8(Str::lit("float")), List::cons(f8(Str::lit("double")), List::<Bytes>::nil()))) {
                                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f409(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because its value space holds positive and negative zero, the infinities and NaN, which no exact LexLean number type holds (XSD 1.1 Part 2, sections 3.3.4 and 3.3.5)")), List::<Bytes>::nil())))))))
                                        } else {
                                            if f197(v0.clone(), List::cons(f8(Str::lit("anySimpleType")), List::cons(f8(Str::lit("anyAtomicType")), List::<Bytes>::nil()))) {
                                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f409(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because it is the union of every simple value space and has no one LexLean type (XSD 1.1 Part 2, sections 3.2.1 and 3.2.2)")), List::<Bytes>::nil())))))))
                                            } else {
                                                let m2517 = f411(v0.clone())?;
                                                match m2517 {
                                                    None => {
                                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f409(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not a built-in datatype of XSD 1.1 Part 2")), List::<Bytes>::nil())))))))
                                                    }
                                                    Some(v3) => {
                                                        if f41(v3.clone(), f8(Str::lit("anyAtomicType"))) {
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
                Ok(next) => { m2518 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f330(v0: Bytes) -> Result<List<Adt2>, Bytes> {
    Err::<List<Adt2>, Bytes>(v0)
}

pub fn f331(v0: Option<Adt2>) -> Option<Bytes> {
    let m2521 = v0;
    match m2521 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m2522 = v1;
            match m2522 {
                Adt2::C0(_) => {
                    None::<Bytes>
                }
                Adt2::C1(_, _) => {
                    None::<Bytes>
                }
                Adt2::C2(_, _, _) => {
                    None::<Bytes>
                }
                Adt2::C3(v2) => {
                    Some(v2)
                }
                Adt2::C4(_) => {
                    None::<Bytes>
                }
                Adt2::C5(_) => {
                    None::<Bytes>
                }
                Adt2::C6 => {
                    None::<Bytes>
                }
                Adt2::C7(_, _) => {
                    None::<Bytes>
                }
                Adt2::C8(_, _, _) => {
                    None::<Bytes>
                }
            }
        }
    }
}

pub fn f332(v0: u64) -> bool {
    f129({
        let a2523 = v0;
        let a2524 = 95u64;
        nat_eq(a2523, a2524)
    }, f129({
        let a2525 = v0;
        let a2526 = 45u64;
        nat_eq(a2525, a2526)
    }, f129({
        let a2527 = v0;
        let a2528 = 32u64;
        nat_eq(a2527, a2528)
    }, f129({
        let a2529 = v0;
        let a2530 = 46u64;
        nat_eq(a2529, a2530)
    }, {
        let a2531 = v0;
        let a2532 = 47u64;
        nat_eq(a2531, a2532)
    }))))
}

pub fn f333(v0: u64) -> bool {
    f77({
        let a2533 = 97u64;
        let a2534 = v0;
        nat_le(a2533, a2534)
    }, {
        let a2535 = v0;
        let a2536 = 122u64;
        nat_le(a2535, a2536)
    })
}

pub fn f334(v0: u64) -> bool {
    f77({
        let a2537 = 65u64;
        let a2538 = v0;
        nat_le(a2537, a2538)
    }, {
        let a2539 = v0;
        let a2540 = 90u64;
        nat_le(a2539, a2540)
    })
}

pub fn f335(v0: u64) -> bool {
    f77({
        let a2541 = 48u64;
        let a2542 = v0;
        nat_le(a2541, a2542)
    }, {
        let a2543 = v0;
        let a2544 = 57u64;
        nat_le(a2543, a2544)
    })
}

pub fn f336(v0: u64) -> bool {
    let a2545 = v0;
    let a2546 = 9u64;
    if if if nat_eq(a2545, a2546) {
        true
    } else {
        let a2547 = v0;
        let a2548 = 10u64;
        nat_eq(a2547, a2548)
    } {
        true
    } else {
        let a2549 = v0;
        let a2550 = 13u64;
        if nat_eq(a2549, a2550) {
            true
        } else {
            let a2551 = 32u64;
            let a2552 = v0;
            if nat_le(a2551, a2552) {
                let a2553 = v0;
                let a2554 = 126u64;
                nat_le(a2553, a2554)
            } else {
                false
            }
        }
    } {
        true
    } else {
        let a2555 = v0;
        let a2556 = 133u64;
        if if nat_eq(a2555, a2556) {
            true
        } else {
            let a2557 = 160u64;
            let a2558 = v0;
            if nat_le(a2557, a2558) {
                let a2559 = v0;
                let a2560 = 55295u64;
                nat_le(a2559, a2560)
            } else {
                false
            }
        } {
            true
        } else {
            let a2561 = 57344u64;
            let a2562 = v0;
            if if nat_le(a2561, a2562) {
                let a2563 = v0;
                let a2564 = 65533u64;
                nat_le(a2563, a2564)
            } else {
                false
            } {
                true
            } else {
                let a2565 = 65536u64;
                let a2566 = v0;
                if nat_le(a2565, a2566) {
                    let a2567 = v0;
                    let a2568 = 1114111u64;
                    nat_le(a2567, a2568)
                } else {
                    false
                }
            }
        }
    }
}

pub fn f337(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f186(v0.clone(), v1)?;
    if f183(f121(v0.clone(), v2)?) {
        Ok(true)
    } else {
        let a2569 = f121(v0, v2)?;
        let a2570 = 35u64;
        Ok(nat_eq(a2569, a2570))
    }
}

pub fn f338(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f412(v0, v1)
}

pub fn f339(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2571 = f121(v0, v1)?;
    let a2572 = 32u64;
    if nat_eq(a2571, a2572) {
        Ok(Some({
            let a2573 = v1;
            let a2574 = 1u64;
            nat_add(a2573, a2574)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f340(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f182(f121(v0, v1)?) {
        Ok(Some({
            let a2575 = v1;
            let a2576 = 1u64;
            nat_add(a2575, a2576)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f341(v0: u64) -> bool {
    let a2577 = v0;
    let a2578 = 44u64;
    if if if nat_eq(a2577, a2578) {
        true
    } else {
        let a2579 = v0;
        let a2580 = 91u64;
        nat_eq(a2579, a2580)
    } {
        true
    } else {
        let a2581 = v0;
        let a2582 = 93u64;
        if nat_eq(a2581, a2582) {
            true
        } else {
            let a2583 = v0;
            let a2584 = 123u64;
            nat_eq(a2583, a2584)
        }
    } {
        true
    } else {
        let a2585 = v0;
        let a2586 = 125u64;
        nat_eq(a2585, a2586)
    }
}

pub fn f342(v0: u64) -> u64 {
    let a2587 = v0;
    let a2588 = 45u64;
    if nat_eq(a2587, a2588) {
        1u64
    } else {
        let a2589 = v0;
        let a2590 = 43u64;
        if nat_eq(a2589, a2590) {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f343(v0: u64) -> u64 {
    let a2591 = 49u64;
    let a2592 = v0;
    if if nat_le(a2591, a2592) {
        let a2593 = v0;
        let a2594 = 57u64;
        nat_le(a2593, a2594)
    } else {
        false
    } {
        let a2595 = v0;
        let a2596 = 48u64;
        nat_sub(a2595, a2596)
    } else {
        0u64
    }
}

pub fn f344(v0: Bytes, v1: u64, v2: u64, v3: bool, v4: u64) -> R<Result<(Adt2, u64), Bytes>> {
    let m2597 = f413(v0.clone(), v1, v2)?;
    match m2597 {
        Ok(v6) => {
            let v7: u64 = {
                let (_, h2598) = v6;
                h2598
            };
            let v8: u64 = f414(v0.clone(), v7, v2)?;
            if f415(v0.clone(), v7, v2)? {
                let m2603 = f416(v0.clone(), v7, v2, v3, {
                    let a2600 = {
                        let a2599 = v0;
                        length_bytes(a2599)
                    };
                    let a2601 = 2u64;
                    nat_add(a2600, a2601)?
                }, Bytes::lit(&[]), {
                    let (h2602, _) = v6;
                    h2602
                })?;
                match m2603 {
                    Ok(v10) => {
                        Ok(Ok::<(Adt2, u64), Bytes>((Adt2::C3(f7(List::cons({
                            let (h2604, _) = v10.clone();
                            h2604
                        }, List::cons({
                            let a2605 = v4;
                            let a2606 = 1u64;
                            if nat_eq(a2605, a2606) {
                                Bytes::lit(&[])
                            } else {
                                Bytes::lit(&[0x0a])
                            }
                        }, List::cons({
                            let a2607 = v4;
                            let a2608 = 2u64;
                            if nat_eq(a2607, a2608) {
                                f346({
                                    let (h2609, _) = {
                                        let (_, h2610) = v10.clone();
                                        h2610
                                    };
                                    h2609
                                })
                            } else {
                                Bytes::lit(&[])
                            }
                        }, List::<Bytes>::nil()))))), {
                            let (_, h2611) = {
                                let (_, h2612) = v10;
                                h2612
                            };
                            h2611
                        })))
                    }
                    Err(v9) => {
                        Ok(Err::<(Adt2, u64), Bytes>(v9))
                    }
                }
            } else {
                Ok(Ok::<(Adt2, u64), Bytes>((Adt2::C3({
                    let a2613 = v4;
                    let a2614 = 2u64;
                    if nat_eq(a2613, a2614) {
                        f346({
                            let (h2615, _) = v6;
                            h2615
                        })
                    } else {
                        Bytes::lit(&[])
                    }
                }), {
                    let a2616 = f121(v0, v8)?;
                    let a2617 = 256u64;
                    if nat_eq(a2616, a2617) {
                        v8
                    } else {
                        v7
                    }
                })))
            }
        }
        Err(v5) => {
            Ok(Err::<(Adt2, u64), Bytes>(v5))
        }
    }
}

pub fn f345(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f417(v0, v1)
}

pub fn f346(v0: u64) -> Bytes {
    let m2618 = v0;
    if m2618 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2618 - 1;
        let a2619 = Bytes::lit(&[0x0a]);
        let a2620 = f346(v1);
        append_bytes(a2619, a2620)
    }
}

pub fn f347(v0: Bytes, v1: u64) -> R<u64> {
    let (_, h2621) = {
        let (h2622, _) = f211(Fn6::F418(v0.clone()), {
            let a2624 = {
                let a2623 = v0;
                length_bytes(a2623)
            };
            let a2625 = 1u64;
            nat_add(a2624, a2625)?
        }, (v1, 0u64))?;
        h2622
    };
    Ok(h2621)
}

pub fn f348(v0: u64, v1: u64) -> u64 {
    let a2626 = v0;
    let a2627 = v1;
    if nat_lt(a2626, a2627) {
        v1
    } else {
        v0
    }
}

pub fn f349(v0: Bytes, p2690: u64, v2: u64, v3: bool, p2691: u64, p2692: Bytes) -> R<Result<(Bytes, u64), Bytes>> {
    let m2689 = (p2690, (p2691, p2692));
    {
        let mut m2689 = m2689;
        loop {
            let step = {
                let (v1, (v4, v5)) = m2689;
                let m2628 = v4;
                if m2628 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f102(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                } else {
                    let v6 = m2628 - 1;
                    let v7: u64 = f121(v0.clone(), v1)?;
                    let a2629 = v7;
                    let a2630 = 34u64;
                    if nat_eq(a2629, a2630) {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v5, {
                            let a2631 = v1;
                            let a2632 = 1u64;
                            nat_add(a2631, a2632)?
                        }))))
                    } else {
                        let a2633 = v7;
                        let a2634 = 256u64;
                        if nat_eq(a2633, a2634) {
                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f102(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                        } else {
                            let a2635 = v7;
                            let a2636 = 92u64;
                            if nat_eq(a2635, a2636) {
                                let v8: u64 = f121(v0.clone(), {
                                    let a2637 = v1;
                                    let a2638 = 1u64;
                                    nat_add(a2637, a2638)?
                                })?;
                                let m2639 = f419(v8);
                                match m2639 {
                                    None => {
                                        let a2644 = v8;
                                        let a2645 = 10u64;
                                        if nat_eq(a2644, a2645) {
                                            if v3 {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f420(v1)?)))
                                            } else {
                                                let m2648 = f421(v0.clone(), {
                                                    let a2646 = v1;
                                                    let a2647 = 1u64;
                                                    nat_add(a2646, a2647)?
                                                }, v2, v6, 0u64)?;
                                                match m2648 {
                                                    Ok(v11) => {
                                                        Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                            let (_, h2649) = v11;
                                                            h2649
                                                        }, (v6, {
                                                            let a2651 = v5;
                                                            let a2652 = f346({
                                                                let (h2650, _) = v11;
                                                                h2650
                                                            });
                                                            append_bytes(a2651, a2652)
                                                        })))
                                                    }
                                                    Err(v10) => {
                                                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(v10)))
                                                    }
                                                }
                                            }
                                        } else {
                                            let v12: u64 = {
                                                let a2653 = v8;
                                                let a2654 = 120u64;
                                                if nat_eq(a2653, a2654) {
                                                    2u64
                                                } else {
                                                    let a2655 = v8;
                                                    let a2656 = 117u64;
                                                    if nat_eq(a2655, a2656) {
                                                        4u64
                                                    } else {
                                                        let a2657 = v8;
                                                        let a2658 = 85u64;
                                                        if nat_eq(a2657, a2658) {
                                                            8u64
                                                        } else {
                                                            0u64
                                                        }
                                                    }
                                                }
                                            };
                                            let a2659 = v12;
                                            let a2660 = 0u64;
                                            if nat_eq(a2659, a2660) {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f102(Str::lit("an escape must be one of YAML 1.2.2 [42] to [62]"), v1)?)))
                                            } else {
                                                let m2663 = f422(v0.clone(), {
                                                    let a2661 = v1;
                                                    let a2662 = 2u64;
                                                    nat_add(a2661, a2662)?
                                                }, v12, 0u64)?;
                                                match m2663 {
                                                    None => {
                                                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f102(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                    }
                                                    Some(v13) => {
                                                        let a2664 = 1114111u64;
                                                        let a2665 = v13;
                                                        if if nat_lt(a2664, a2665) {
                                                            true
                                                        } else {
                                                            let a2666 = 55296u64;
                                                            let a2667 = v13;
                                                            if nat_le(a2666, a2667) {
                                                                let a2668 = v13;
                                                                let a2669 = 57343u64;
                                                                nat_le(a2668, a2669)
                                                            } else {
                                                                false
                                                            }
                                                        } {
                                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f102(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                        } else {
                                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                                let a2672 = {
                                                                    let a2670 = v1;
                                                                    let a2671 = 2u64;
                                                                    nat_add(a2670, a2671)?
                                                                };
                                                                let a2673 = v12;
                                                                nat_add(a2672, a2673)?
                                                            }, (v6, {
                                                                let a2674 = v5;
                                                                let a2675 = f326(v13)?;
                                                                append_bytes(a2674, a2675)
                                                            })))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Some(v9) => {
                                        Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                            let a2640 = v1;
                                            let a2641 = 2u64;
                                            nat_add(a2640, a2641)?
                                        }, (v6, {
                                            let a2642 = v5;
                                            let a2643 = v9;
                                            append_bytes(a2642, a2643)
                                        })))
                                    }
                                }
                            } else {
                                if if f182(v7) {
                                    true
                                } else {
                                    let a2676 = v7;
                                    let a2677 = 10u64;
                                    nat_eq(a2676, a2677)
                                } {
                                    let v14: u64 = f186(v0.clone(), v1)?;
                                    let a2678 = f121(v0.clone(), v14)?;
                                    let a2679 = 10u64;
                                    if nat_eq(a2678, a2679) {
                                        if v3 {
                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f420(v1)?)))
                                        } else {
                                            let m2680 = f421(v0.clone(), v14, v2, v6, 0u64)?;
                                            match m2680 {
                                                Ok(v16) => {
                                                    Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                        let (_, h2681) = v16;
                                                        h2681
                                                    }, (v6, {
                                                        let a2683 = v5;
                                                        let a2684 = f357({
                                                            let (h2682, _) = v16;
                                                            h2682
                                                        });
                                                        append_bytes(a2683, a2684)
                                                    })))
                                                }
                                                Err(v15) => {
                                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(v15)))
                                                }
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v14, (v6, {
                                            let a2685 = v5;
                                            let a2686 = f270(v0.clone(), v1, v14);
                                            append_bytes(a2685, a2686)
                                        })))
                                    }
                                } else {
                                    let v17: u64 = f423(v0.clone(), v1)?;
                                    Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v17, (v6, {
                                        let a2687 = v5;
                                        let a2688 = f270(v0.clone(), v1, v17);
                                        append_bytes(a2687, a2688)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2689 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f350(v0: Bytes, p2722: u64, v2: u64, v3: bool, p2723: u64, p2724: Bytes) -> R<Result<(Bytes, u64), Bytes>> {
    let m2721 = (p2722, (p2723, p2724));
    {
        let mut m2721 = m2721;
        loop {
            let step = {
                let (v1, (v4, v5)) = m2721;
                let m2693 = v4;
                if m2693 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f102(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                } else {
                    let v6 = m2693 - 1;
                    let v7: u64 = f121(v0.clone(), v1)?;
                    let a2694 = v7;
                    let a2695 = 39u64;
                    if nat_eq(a2694, a2695) {
                        let a2698 = f121(v0.clone(), {
                            let a2696 = v1;
                            let a2697 = 1u64;
                            nat_add(a2696, a2697)?
                        })?;
                        let a2699 = 39u64;
                        if nat_eq(a2698, a2699) {
                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                let a2700 = v1;
                                let a2701 = 2u64;
                                nat_add(a2700, a2701)?
                            }, (v6, {
                                let a2702 = v5;
                                let a2703 = Bytes::lit(&[0x27]);
                                append_bytes(a2702, a2703)
                            })))
                        } else {
                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v5, {
                                let a2704 = v1;
                                let a2705 = 1u64;
                                nat_add(a2704, a2705)?
                            }))))
                        }
                    } else {
                        let a2706 = v7;
                        let a2707 = 256u64;
                        if nat_eq(a2706, a2707) {
                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f102(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                        } else {
                            if if f182(v7) {
                                true
                            } else {
                                let a2708 = v7;
                                let a2709 = 10u64;
                                nat_eq(a2708, a2709)
                            } {
                                let v8: u64 = f186(v0.clone(), v1)?;
                                let a2710 = f121(v0.clone(), v8)?;
                                let a2711 = 10u64;
                                if nat_eq(a2710, a2711) {
                                    if v3 {
                                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f420(v1)?)))
                                    } else {
                                        let m2712 = f421(v0.clone(), v8, v2, v6, 0u64)?;
                                        match m2712 {
                                            Ok(v10) => {
                                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                    let (_, h2713) = v10;
                                                    h2713
                                                }, (v6, {
                                                    let a2715 = v5;
                                                    let a2716 = f357({
                                                        let (h2714, _) = v10;
                                                        h2714
                                                    });
                                                    append_bytes(a2715, a2716)
                                                })))
                                            }
                                            Err(v9) => {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(v9)))
                                            }
                                        }
                                    }
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v8, (v6, {
                                        let a2717 = v5;
                                        let a2718 = f270(v0.clone(), v1, v8);
                                        append_bytes(a2717, a2718)
                                    })))
                                }
                            } else {
                                let v11: u64 = f424(v0.clone(), v1)?;
                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v11, (v6, {
                                    let a2719 = v5;
                                    let a2720 = f270(v0.clone(), v1, v11);
                                    append_bytes(a2719, a2720)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2721 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f351(v0: Bytes, v1: u64, v2: u64) -> R<Result<u64, Bytes>> {
    let v3: (u64, bool) = {
        let (h2725, _) = f425(Fn9::F426(v2, v0.clone()), {
            let a2727 = {
                let a2726 = v0.clone();
                length_bytes(a2726)
            };
            let a2728 = 1u64;
            nat_add(a2727, a2728)?
        }, (v1, false))?;
        h2725
    };
    let (_, h2729) = v3;
    if h2729 {
        if f120(v0, {
            let (h2730, _) = v3;
            h2730
        })? {
            Ok(Err::<u64, Bytes>(f102(Str::lit("a document marker cannot appear inside a flow collection (YAML 1.2.2 [206])"), {
                let (h2731, _) = v3;
                h2731
            })?))
        } else {
            Ok(Err::<u64, Bytes>(f102(Str::lit("a line of a flow collection is indented less than its node (YAML 1.2.2 [80])"), {
                let (h2732, _) = v3;
                h2732
            })?))
        }
    } else {
        Ok(Ok::<u64, Bytes>({
            let (h2733, _) = v3;
            h2733
        }))
    }
}

pub fn f352(v0: Bytes, v1: u64, v2: u64) -> R<Result<(Bytes, u64), Bytes>> {
    let v3: u64 = f121(v0.clone(), v1)?;
    let a2734 = v3;
    let a2735 = 34u64;
    if if nat_eq(a2734, a2735) {
        true
    } else {
        let a2736 = v3;
        let a2737 = 39u64;
        nat_eq(a2736, a2737)
    } {
        f265(v0, v1, v2, false)
    } else {
        let m2738 = f189(v0.clone(), v1)?;
        match m2738 {
            None => {
                let a2739 = v3;
                let a2740 = 91u64;
                if if nat_eq(a2739, a2740) {
                    true
                } else {
                    let a2741 = v3;
                    let a2742 = 123u64;
                    nat_eq(a2741, a2742)
                } {
                    Ok(Err::<(Bytes, u64), Bytes>(f102(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [142])"), v1)?))
                } else {
                    if f267(v0.clone(), v1, true)? {
                        let v5: u64 = f268(v0.clone(), v1, true)?;
                        Ok(Ok::<(Bytes, u64), Bytes>((f270(v0, v1, v5), v5)))
                    } else {
                        Ok(Err::<(Bytes, u64), Bytes>(f102(Str::lit("a flow mapping key must be a scalar (YAML 1.2.2 [142])"), v1)?))
                    }
                }
            }
            Some(v4) => {
                Ok(Err::<(Bytes, u64), Bytes>(v4))
            }
        }
    }
}

pub fn f353(v0: u64) -> bool {
    if f341(v0) {
        true
    } else {
        let a2743 = v0;
        let a2744 = 45u64;
        if if if if nat_eq(a2743, a2744) {
            true
        } else {
            let a2745 = v0;
            let a2746 = 63u64;
            nat_eq(a2745, a2746)
        } {
            true
        } else {
            let a2747 = v0;
            let a2748 = 58u64;
            if nat_eq(a2747, a2748) {
                true
            } else {
                let a2749 = v0;
                let a2750 = 35u64;
                nat_eq(a2749, a2750)
            }
        } {
            true
        } else {
            let a2751 = v0;
            let a2752 = 38u64;
            if if nat_eq(a2751, a2752) {
                true
            } else {
                let a2753 = v0;
                let a2754 = 42u64;
                nat_eq(a2753, a2754)
            } {
                true
            } else {
                let a2755 = v0;
                let a2756 = 33u64;
                if nat_eq(a2755, a2756) {
                    true
                } else {
                    let a2757 = v0;
                    let a2758 = 124u64;
                    nat_eq(a2757, a2758)
                }
            }
        } {
            true
        } else {
            let a2759 = v0;
            let a2760 = 62u64;
            if if if nat_eq(a2759, a2760) {
                true
            } else {
                let a2761 = v0;
                let a2762 = 39u64;
                nat_eq(a2761, a2762)
            } {
                true
            } else {
                let a2763 = v0;
                let a2764 = 34u64;
                if nat_eq(a2763, a2764) {
                    true
                } else {
                    let a2765 = v0;
                    let a2766 = 37u64;
                    nat_eq(a2765, a2766)
                }
            } {
                true
            } else {
                let a2767 = v0;
                let a2768 = 64u64;
                if nat_eq(a2767, a2768) {
                    true
                } else {
                    let a2769 = v0;
                    let a2770 = 96u64;
                    nat_eq(a2769, a2770)
                }
            }
        }
    }
}

pub fn f354(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f427(v1, v0, v2)
}

pub fn f355(v0: u64) -> R<Bytes> {
    f102(Str::lit("a mapping value is not allowed here: a plain scalar holds ': ' (YAML 1.2.2 [130])"), v0)
}

pub fn f356(v0: Bytes, v1: u64) -> R<(u64, u64)> {
    let (h2771, _) = f211(Fn6::F428(v0.clone()), {
        let a2773 = {
            let a2772 = v0;
            length_bytes(a2772)
        };
        let a2774 = 1u64;
        nat_add(a2773, a2774)?
    }, (v1, 0u64))?;
    Ok(h2771)
}

pub fn f357(v0: u64) -> Bytes {
    let a2775 = v0;
    let a2776 = 0u64;
    if nat_eq(a2775, a2776) {
        Bytes::lit(&[0x20])
    } else {
        f346(v0)
    }
}

pub fn f358(v0: Bytes, v1: Str) -> bool {
    f41(v0, f8(v1))
}

pub fn f359(v0: Bytes, v1: u64, v2: u64) -> R<Result<Adt2, Bytes>> {
    let v3: (u64, u64) = f429(v0.clone(), v1, 2u64)?;
    let a2779 = {
        let (h2777, _) = v3;
        h2777
    };
    let a2780 = {
        let a2778 = v0.clone();
        length_bytes(a2778)
    };
    if nat_eq(a2779, a2780) {
        if f133(false, {
            let (_, h2781) = v3;
            h2781
        }) {
            Ok(Ok::<Adt2, Bytes>(Adt2::C1(false, {
                let (_, h2782) = v3;
                h2782
            })))
        } else {
            Ok(Err::<Adt2, Bytes>(f361(v2)?))
        }
    } else {
        Ok(Ok::<Adt2, Bytes>(Adt2::C3(v0)))
    }
}

pub fn f360(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2783 = v0.clone();
        length_bytes(a2783)
    };
    let v2: u64 = f121(v0.clone(), 0u64)?;
    let v3: u64 = {
        let a2784 = v2;
        let a2785 = 43u64;
        if if nat_eq(a2784, a2785) {
            true
        } else {
            let a2786 = v2;
            let a2787 = 45u64;
            nat_eq(a2786, a2787)
        } {
            1u64
        } else {
            0u64
        }
    };
    let v4: u64 = f128(v0.clone(), v3)?;
    let v5: bool = {
        let a2788 = f121(v0.clone(), v4)?;
        let a2789 = 46u64;
        nat_eq(a2788, a2789)
    };
    let v6: u64 = if v5 {
        let a2790 = v4;
        let a2791 = 1u64;
        nat_add(a2790, a2791)?
    } else {
        v4
    };
    let v7: u64 = if v5 {
        f128(v0.clone(), v6)?
    } else {
        v4
    };
    let v8: u64 = f121(v0.clone(), v7)?;
    let v9: bool = {
        let a2792 = v8;
        let a2793 = 101u64;
        if nat_eq(a2792, a2793) {
            true
        } else {
            let a2794 = v8;
            let a2795 = 69u64;
            nat_eq(a2794, a2795)
        }
    };
    let v10: u64 = f121(v0.clone(), {
        let a2796 = v7;
        let a2797 = 1u64;
        nat_add(a2796, a2797)?
    })?;
    let v11: bool = if v9 {
        let a2798 = v10;
        let a2799 = 43u64;
        if nat_eq(a2798, a2799) {
            true
        } else {
            let a2800 = v10;
            let a2801 = 45u64;
            nat_eq(a2800, a2801)
        }
    } else {
        false
    };
    let v12: u64 = if v9 {
        if v11 {
            let a2802 = v7;
            let a2803 = 2u64;
            nat_add(a2802, a2803)?
        } else {
            let a2804 = v7;
            let a2805 = 1u64;
            nat_add(a2804, a2805)?
        }
    } else {
        v7
    };
    let v13: u64 = if v9 {
        f128(v0.clone(), v12)?
    } else {
        v7
    };
    let v14: Bytes = f270(v0.clone(), v3, v4);
    let v15: u64 = f130(v14.clone())?;
    let a2806 = v13;
    let a2807 = v1;
    if if if nat_eq(a2806, a2807) {
        let a2808 = v3;
        let a2809 = v4;
        if nat_lt(a2808, a2809) {
            true
        } else {
            if v5 {
                let a2810 = v6;
                let a2811 = v7;
                nat_lt(a2810, a2811)
            } else {
                false
            }
        }
    } else {
        false
    } {
        if f194(v9) {
            true
        } else {
            let a2812 = v12;
            let a2813 = v13;
            nat_lt(a2812, a2813)
        }
    } else {
        false
    } {
        Ok(Some(f7(List::cons({
            let a2814 = v2;
            let a2815 = 45u64;
            if nat_eq(a2814, a2815) {
                Bytes::lit(&[0x2d])
            } else {
                Bytes::lit(&[])
            }
        }, List::cons({
            let a2817 = v15;
            let a2818 = {
                let a2816 = v14.clone();
                length_bytes(a2816)
            };
            if nat_eq(a2817, a2818) {
                Bytes::lit(&[0x30])
            } else {
                f270(v14.clone(), v15, {
                    let a2819 = v14;
                    length_bytes(a2819)
                })
            }
        }, List::cons({
            let a2820 = v6;
            let a2821 = v7;
            if nat_lt(a2820, a2821) {
                let a2822 = Bytes::lit(&[0x2e]);
                let a2823 = f270(v0.clone(), v6, v7);
                append_bytes(a2822, a2823)
            } else {
                Bytes::lit(&[])
            }
        }, List::cons(if v9 {
            let a2826 = if if v11 {
                let a2824 = v10;
                let a2825 = 45u64;
                nat_eq(a2824, a2825)
            } else {
                false
            } {
                Bytes::lit(&[0x65, 0x2d])
            } else {
                Bytes::lit(&[0x65])
            };
            let a2827 = f270(v0, v12, v13);
            append_bytes(a2826, a2827)
        } else {
            Bytes::lit(&[])
        }, List::<Bytes>::nil())))))))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f361(v0: u64) -> R<Bytes> {
    f102(Str::lit("a number outside the range Studio.Value holds is not read (YAML 1.2.2 section 10.3.2)"), v0)
}

pub fn f362(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f430(v0, v1)
}

pub fn f363(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f431(v0, v1)
}

pub fn f364(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m2828 = v0.clone();
    match m2828.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a2831 = {
                let (h2829, _) = v2;
                h2829
            };
            let a2832 = {
                let (h2830, _) = v1.clone();
                h2830
            };
            if equal(a2831, a2832) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f365(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f432(f433(Fn10::F434, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m2837 = {
        let (h2833, _) = f435(Fn11::F436, {
            let a2835 = {
                let a2834 = v0;
                length_list(a2834)
            };
            let a2836 = 1u64;
            nat_add(a2835, a2836)?
        }, v1)?;
        h2833
    };
    match m2837.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f366(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2842 = {
        let a2839 = f274({
            let (h2838, _) = v0;
            h2838
        })?;
        let a2840 = Bytes::lit(&[0x3a]);
        append_bytes(a2839, a2840)
    };
    let a2843 = {
        let (_, h2841) = v0;
        h2841
    };
    Ok(append_bytes(a2842, a2843))
}

pub fn f367(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f75(v0.clone(), {
        let a2844 = v1;
        let a2845 = 1u64;
        nat_add(a2844, a2845)?
    })?;
    let v4: Bytes = {
        let a2846 = v3;
        let a2847 = 117u64;
        if nat_eq(a2846, a2847) {
            f437(v0, v1)?
        } else {
            f438(v3)
        }
    };
    let a2848 = v4.clone();
    let a2849 = Bytes::lit(&[]);
    if equal(a2848, a2849) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a2850 = v3;
            let a2851 = 117u64;
            if nat_eq(a2850, a2851) {
                let a2852 = v1;
                let a2853 = 6u64;
                nat_add(a2852, a2853)?
            } else {
                let a2854 = v1;
                let a2855 = 2u64;
                nat_add(a2854, a2855)?
            }
        }, ({
            let a2856 = v2;
            let a2857 = v4;
            append_bytes(a2856, a2857)
        }, 0u64))))
    }
}

pub fn f368(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2860 = {
        let a2858 = v0;
        let a2859 = v1;
        index_bytes(a2858, a2859)
    };
    match m2860 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f439(v2)? {
                Ok(Some({
                    let a2861 = v1;
                    let a2862 = 1u64;
                    nat_add(a2861, a2862)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f369(v0: u8) -> R<u64> {
    let a2909 = {
        let v1: u8 = v0;
        let a2907 = {
            let a2883 = {
                let a2871 = {
                    let a2865 = {
                        let a2863 = v1;
                        let a2864 = 128u8;
                        fixed_u8::bit_and(a2863, a2864)
                    };
                    let a2866 = 128u8;
                    if equal(a2865, a2866) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2872 = {
                    let a2869 = {
                        let a2867 = v1;
                        let a2868 = 64u8;
                        fixed_u8::bit_and(a2867, a2868)
                    };
                    let a2870 = 64u8;
                    if equal(a2869, a2870) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2871, a2872)?
            };
            let a2884 = {
                let a2881 = {
                    let a2875 = {
                        let a2873 = v1;
                        let a2874 = 32u8;
                        fixed_u8::bit_and(a2873, a2874)
                    };
                    let a2876 = 32u8;
                    if equal(a2875, a2876) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2882 = {
                    let a2879 = {
                        let a2877 = v1;
                        let a2878 = 16u8;
                        fixed_u8::bit_and(a2877, a2878)
                    };
                    let a2880 = 16u8;
                    if equal(a2879, a2880) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2881, a2882)?
            };
            nat_add(a2883, a2884)?
        };
        let a2908 = {
            let a2905 = {
                let a2893 = {
                    let a2887 = {
                        let a2885 = v1;
                        let a2886 = 8u8;
                        fixed_u8::bit_and(a2885, a2886)
                    };
                    let a2888 = 8u8;
                    if equal(a2887, a2888) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2894 = {
                    let a2891 = {
                        let a2889 = v1;
                        let a2890 = 4u8;
                        fixed_u8::bit_and(a2889, a2890)
                    };
                    let a2892 = 4u8;
                    if equal(a2891, a2892) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2893, a2894)?
            };
            let a2906 = {
                let a2903 = {
                    let a2897 = {
                        let a2895 = v1;
                        let a2896 = 2u8;
                        fixed_u8::bit_and(a2895, a2896)
                    };
                    let a2898 = 2u8;
                    if equal(a2897, a2898) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2904 = {
                    let a2901 = {
                        let a2899 = v1;
                        let a2900 = 1u8;
                        fixed_u8::bit_and(a2899, a2900)
                    };
                    let a2902 = 1u8;
                    if equal(a2901, a2902) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2903, a2904)?
            };
            nat_add(a2905, a2906)?
        };
        nat_add(a2907, a2908)?
    };
    let a2910 = 48u64;
    Ok(nat_sub(a2909, a2910))
}

pub fn f370(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a2911 = v0.clone();
        length_bytes(a2911)
    };
    let a2912 = v1;
    let a2913 = v2;
    if if nat_lt(a2912, a2913) {
        let a2918 = f75(v0, {
            let a2916 = v2;
            let a2917 = {
                let a2914 = v1;
                let a2915 = 1u64;
                nat_add(a2914, a2915)?
            };
            nat_sub(a2916, a2917)
        })?;
        let a2919 = 48u64;
        nat_eq(a2918, a2919)
    } else {
        false
    } {
        Ok(Some({
            let a2920 = v1;
            let a2921 = 1u64;
            nat_add(a2920, a2921)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f371(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f440(v1, v0, v2)
}

pub fn f372(v0: u64) -> bool {
    if f373(v0) {
        true
    } else {
        let a2922 = 65u64;
        let a2923 = v0;
        if if nat_le(a2922, a2923) {
            let a2924 = v0;
            let a2925 = 70u64;
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
                let a2929 = 102u64;
                nat_le(a2928, a2929)
            } else {
                false
            }
        }
    }
}

pub fn f373(v0: u64) -> bool {
    let a2930 = 48u64;
    let a2931 = v0;
    if nat_le(a2930, a2931) {
        let a2932 = v0;
        let a2933 = 57u64;
        nat_le(a2932, a2933)
    } else {
        false
    }
}

pub fn f374(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f106(v0.clone(), v1)?;
    let a2934 = v2;
    let a2935 = 69u64;
    if if nat_eq(a2934, a2935) {
        true
    } else {
        let a2936 = v2;
        let a2937 = 101u64;
        nat_eq(a2936, a2937)
    } {
        let v3: u64 = f106(v0.clone(), {
            let a2938 = v1;
            let a2939 = 1u64;
            nat_add(a2938, a2939)?
        })?;
        let v4: u64 = {
            let a2940 = v3;
            let a2941 = 43u64;
            if if nat_eq(a2940, a2941) {
                true
            } else {
                let a2942 = v3;
                let a2943 = 45u64;
                nat_eq(a2942, a2943)
            } {
                let a2944 = v1;
                let a2945 = 2u64;
                nat_add(a2944, a2945)?
            } else {
                let a2946 = v1;
                let a2947 = 1u64;
                nat_add(a2946, a2947)?
            }
        };
        let v5: u64 = f289(v0, 0u64, v4)?;
        let a2948 = v4;
        let a2949 = v5;
        if nat_lt(a2948, a2949) {
            Ok(v5)
        } else {
            Ok(v1)
        }
    } else {
        Ok(v1)
    }
}

pub fn f375(v0: (u64, Bytes), v1: Bytes) -> bool {
    let a2951 = {
        let (h2950, _) = v0;
        h2950
    };
    let a2952 = 3u64;
    if !nat_eq(a2951, a2952) {
        f41({
            let (_, h2953) = v0;
            h2953
        }, v1)
    } else {
        false
    }
}

pub fn f376(v0: Bytes) -> Bytes {
    if f41(v0.clone(), f8(Str::lit("("))) {
        f8(Str::lit(")"))
    } else {
        if f41(v0.clone(), f8(Str::lit("["))) {
            f8(Str::lit("]"))
        } else {
            if f41(v0.clone(), f8(Str::lit("{"))) {
                f8(Str::lit("}"))
            } else {
                if f41(v0, f8(Str::lit("<"))) {
                    f8(Str::lit(">"))
                } else {
                    Bytes::lit(&[])
                }
            }
        }
    }
}

pub fn f377(v0: Bytes) -> bool {
    if f41(v0.clone(), f8(Str::lit(")"))) {
        true
    } else {
        if f41(v0.clone(), f8(Str::lit("]"))) {
            true
        } else {
            if f41(v0.clone(), f8(Str::lit("}"))) {
                true
            } else {
                f41(v0, f8(Str::lit(">")))
            }
        }
    }
}

pub fn f378(p2970: u64, p2971: List<(u64, Bytes)>, p2972: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>> {
    let m2969 = (p2970, (p2971, p2972));
    {
        let mut m2969 = m2969;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2969;
                let m2954 = v0;
                if m2954 == 0 {
                    Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>(f8(Str::lit("a dictionary has more members than the input has tokens")))))
                } else {
                    let v3 = m2954 - 1;
                    if f292(v1.clone(), Str::lit("}")) {
                        let m2955 = f299(f294(v1), Str::lit(";"));
                        match m2955 {
                            Ok(v5) => {
                                Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Ok::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>((f441(v2, List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m2956 = f217(v1)?;
                        match m2956 {
                            Ok(v7) => {
                                let v8: bool = f292(v7.clone(), Str::lit("required"));
                                let m2957 = f302(v3, if v8 {
                                    f294(v7)
                                } else {
                                    v7
                                }, 0u64)?;
                                match m2957 {
                                    Ok(v10) => {
                                        let m2959 = f298({
                                            let (_, h2958) = v10;
                                            h2958
                                        });
                                        match m2959 {
                                            Ok(v12) => {
                                                if if f292({
                                                    let (_, h2960) = v12.clone();
                                                    h2960
                                                }, Str::lit("=")) {
                                                    !v8
                                                } else {
                                                    false
                                                } {
                                                    let m2962 = f380(f294({
                                                        let (_, h2961) = v12;
                                                        h2961
                                                    }))?;
                                                    match m2962 {
                                                        Ok(v14) => {
                                                            Ok::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>((v3, (v14, List::cons(({
                                                                let (h2963, _) = v12;
                                                                h2963
                                                            }, (f387({
                                                                let (h2964, _) = v10;
                                                                h2964
                                                            }), 0u64)), v2))))
                                                        }
                                                        Err(v13) => {
                                                            Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>(v13)))
                                                        }
                                                    }
                                                } else {
                                                    let m2966 = f299({
                                                        let (_, h2965) = v12;
                                                        h2965
                                                    }, Str::lit(";"));
                                                    match m2966 {
                                                        Ok(v16) => {
                                                            Ok::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>((v3, (v16, List::cons(({
                                                                let (h2967, _) = v12;
                                                                h2967
                                                            }, (f387({
                                                                let (h2968, _) = v10;
                                                                h2968
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
                Ok(next) => { m2969 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f379(p2985: u64, p2986: List<(u64, Bytes)>, p2987: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>> {
    let m2984 = (p2985, (p2986, p2987));
    {
        let mut m2984 = m2984;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2984;
                let m2973 = v0;
                if m2973 == 0 {
                    Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>(f8(Str::lit("an interface has more members than the input has tokens")))))
                } else {
                    let v3 = m2973 - 1;
                    if f292(v1.clone(), Str::lit("}")) {
                        let m2974 = f299(f294(v1), Str::lit(";"));
                        match m2974 {
                            Ok(v5) => {
                                Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Ok::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>((f441(v2, List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m2975 = f217(v1)?;
                        match m2975 {
                            Ok(v7) => {
                                let v8: List<(u64, Bytes)> = if if f292(v7.clone(), Str::lit("stringifier")) {
                                    true
                                } else {
                                    f292(v7.clone(), Str::lit("inherit"))
                                } {
                                    f294(v7.clone())
                                } else {
                                    v7.clone()
                                };
                                let v9: List<(u64, Bytes)> = if f292(v8.clone(), Str::lit("readonly")) {
                                    f294(v8)
                                } else {
                                    v8
                                };
                                if f292(v9.clone(), Str::lit("attribute")) {
                                    let m2976 = f302(v3, f294(v9), 0u64)?;
                                    match m2976 {
                                        Ok(v11) => {
                                            let m2978 = f298({
                                                let (_, h2977) = v11;
                                                h2977
                                            });
                                            match m2978 {
                                                Ok(v13) => {
                                                    let m2980 = f299({
                                                        let (_, h2979) = v13;
                                                        h2979
                                                    }, Str::lit(";"));
                                                    match m2980 {
                                                        Ok(v15) => {
                                                            Ok::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>((v3, (v15, List::cons(({
                                                                let (h2981, _) = v13;
                                                                h2981
                                                            }, (f387({
                                                                let (h2982, _) = v11;
                                                                h2982
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
                                    let m2983 = f380(v7)?;
                                    match m2983 {
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
                Ok(next) => { m2984 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f380(v0: List<(u64, Bytes)>) -> R<Result<List<(u64, Bytes)>, Bytes>> {
    Ok(f293({
        let a2989 = {
            let a2988 = v0.clone();
            length_list(a2988)
        };
        let a2990 = 1u64;
        nat_add(a2989, a2990)?
    }, v0, List::<Bytes>::nil(), true))
}

pub fn f381(v0: List<(u64, Bytes)>) -> Bytes {
    let m2991 = v0;
    match m2991.uncons() {
        None => {
            f8(Str::lit("the end of the input"))
        }
        Some((v1, _)) => {
            f7(List::cons(f8(Str::lit("'")), List::cons({
                let (_, h2992) = v1;
                h2992
            }, List::cons(f8(Str::lit("'")), List::<Bytes>::nil()))))
        }
    }
}

pub fn f382(v0: Bytes) -> Bytes {
    if f170(v0.clone(), f8(Str::lit("_"))) {
        f38(v0.clone(), 1u64, {
            let a2994 = {
                let a2993 = v0;
                length_bytes(a2993)
            };
            let a2995 = 1u64;
            nat_sub(a2994, a2995)
        })
    } else {
        v0
    }
}

pub fn f383(v0: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), v1: List<(u64, Bytes)>) -> Result<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes> {
    if f292(v1.clone(), Str::lit("?")) {
        Ok::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>((f384(Str::lit("Option"), v0), f294(v1)))
    } else {
        Ok::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>((v0, v1))
    }
}

pub fn f384(v0: Str, v1: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))) -> (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) {
    let v2: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) = f387(v1);
    (0u64, (f7(List::cons(f8(Str::lit("(")), List::cons({
        let a2996 = v0;
        utf8_encode(a2996)
    }, List::cons(f8(Str::lit(" ")), List::cons(f236(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), (f237(v2.clone()), (f234(v2), Bytes::lit(&[])))))
}

pub fn f385(v0: Bytes) -> (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) {
    (0u64, (Bytes::lit(&[]), (List::<Bytes>::nil(), (v0, Bytes::lit(&[])))))
}

pub fn f386(v0: Bytes) -> Option<(u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))> {
    let m2997 = f442(v0, f443());
    match m2997 {
        None => {
            None::<(u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))>
        }
        Some(v1) => {
            Some({
                let (h2998, _) = v1;
                if h2998 {
                    f301({
                        let (_, h2999) = v1;
                        h2999
                    })
                } else {
                    f385({
                        let (_, h3000) = v1;
                        h3000
                    })
                }
            })
        }
    }
}

pub fn f387(v0: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))) -> (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) {
    let a3001 = f235(v0.clone());
    let a3002 = 0u64;
    if nat_eq(a3001, a3002) {
        v0
    } else {
        (0u64, (Bytes::lit(&[]), (f237(v0.clone()), (if f41(f234(v0.clone()), Bytes::lit(&[])) {
            f444()
        } else {
            f234(v0)
        }, Bytes::lit(&[])))))
    }
}

pub fn f388(v0: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))) -> Bytes {
    let (_, h3003) = {
        let (_, h3004) = {
            let (_, h3005) = {
                let (_, h3006) = v0;
                h3006
            };
            h3005
        };
        h3004
    };
    h3003
}

pub fn f389(v0: (Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64)), v1: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> R<List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>> {
    let m3007 = v1;
    match m3007.uncons() {
        None => {
            Ok(List::cons(v0, List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil()))
        }
        Some((v2, v3)) => {
            if f445({
                let a3010 = {
                    let a3009 = {
                        let (h3008, _) = v0.clone();
                        h3008
                    };
                    length_bytes(a3009)
                };
                let a3011 = 1u64;
                nat_add(a3010, a3011)?
            }, {
                let (h3012, _) = v0.clone();
                h3012
            }, {
                let (h3013, _) = v2.clone();
                h3013
            }, 0u64)? {
                Ok(List::cons(v0, List::cons(v2, v3)))
            } else {
                Ok(List::cons(v2, f389(v0, v3)?))
            }
        }
    }
}

pub fn f390(v0: List<Adt13>, v1: Bytes) -> List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))> {
    let m3014 = f147(v0.clone(), 1u64, v1.clone());
    match m3014 {
        None => {
            f313(v0, 6u64, v1)
        }
        Some(v2) => {
            f311(f312(v2), f313(v0, 6u64, v1))
        }
    }
}

pub fn f391(v0: Bytes) -> R<Option<Bytes>> {
    let m3016 = f446({
        let a3015 = v0.clone();
        length_bytes(a3015)
    }, v0, 0u64, Bytes::lit(&[]))?;
    match m3016 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a3018 = {
                let a3017 = v1.clone();
                length_bytes(a3017)
            };
            let a3019 = 0u64;
            if nat_eq(a3018, a3019) {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(v1))
            }
        }
    }
}

pub fn f392(p3029: u64, v1: Bytes, p3030: u64) -> R<bool> {
    let m3028 = (p3029, p3030);
    {
        let mut m3028 = m3028;
        loop {
            let step = {
                let (v0, v2) = m3028;
                let m3020 = v0;
                if m3020 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m3020 - 1;
                    let m3023 = {
                        let a3021 = v1.clone();
                        let a3022 = v2;
                        index_bytes(a3021, a3022)
                    };
                    match m3023 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a3024 = f46(v4)?;
                            let a3025 = 128u64;
                            if nat_lt(a3024, a3025) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a3026 = v2;
                                    let a3027 = 1u64;
                                    nat_add(a3026, a3027)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3028 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f393(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f257(v0.clone(), 0u64, {
        let a3031 = v0;
        length_bytes(a3031)
    }, false, Bytes::lit(&[]))?;
    let m3034 = {
        let a3032 = v1.clone();
        let a3033 = 0u64;
        index_bytes(a3032, a3033)
    };
    match m3034 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f46(v2)?;
            if f334(v3) {
                let a3041 = f38({
                    let a3035 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a3035)
                }, {
                    let a3036 = v3;
                    let a3037 = 65u64;
                    nat_sub(a3036, a3037)
                }, 1u64);
                let a3042 = f38(v1.clone(), 1u64, {
                    let a3039 = {
                        let a3038 = v1;
                        length_bytes(a3038)
                    };
                    let a3040 = 1u64;
                    nat_sub(a3039, a3040)
                });
                Ok(append_bytes(a3041, a3042))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f394(v0: Bytes) -> R<bool> {
    let m3045 = {
        let a3043 = v0;
        let a3044 = 0u64;
        index_bytes(a3043, a3044)
    };
    match m3045 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f129(f333(f46(v1)?), f334(f46(v1)?)))
        }
    }
}

pub fn f395(p3058: u64, v1: Bytes, p3059: u64, p3060: Bytes) -> R<Option<Bytes>> {
    let m3057 = (p3058, (p3059, p3060));
    {
        let mut m3057 = m3057;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3057;
                let m3046 = v0;
                if m3046 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3046 - 1;
                    let a3048 = {
                        let a3047 = v1.clone();
                        length_bytes(a3047)
                    };
                    let a3049 = v2;
                    if nat_le(a3048, a3049) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3050 = f319(v1.clone(), v2)?;
                        match m3050 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3052 = v2;
                                    let a3053 = {
                                        let (_, h3051) = v5;
                                        h3051
                                    };
                                    nat_add(a3052, a3053)?
                                }, {
                                    let a3055 = v3;
                                    let a3056 = f447({
                                        let (h3054, _) = v5;
                                        h3054
                                    });
                                    append_bytes(a3055, a3056)
                                })))
                            }
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

pub fn f396(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3063 = {
        let a3061 = v0;
        let a3062 = v1;
        index_bytes(a3061, a3062)
    };
    match m3063 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f46(v2)?;
            if f77({
                let a3064 = 128u64;
                let a3065 = v3;
                nat_le(a3064, a3065)
            }, {
                let a3066 = v3;
                let a3067 = 191u64;
                nat_le(a3066, a3067)
            }) {
                Ok(Some({
                    let a3068 = v3;
                    let a3069 = 128u64;
                    nat_sub(a3068, a3069)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f397(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f77({
        let a3070 = v1;
        let a3071 = v0;
        nat_le(a3070, a3071)
    }, f77({
        let a3072 = v0;
        let a3073 = v2;
        nat_le(a3072, a3073)
    }, f129({
        let a3074 = v0;
        let a3075 = 55296u64;
        nat_lt(a3074, a3075)
    }, {
        let a3076 = 57343u64;
        let a3077 = v0;
        nat_lt(a3076, a3077)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f398() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{3}/\u{7f}\u{3}@\u{0}\u{3}\u{7f}}\u{4}\u{0}\u{0}C\u{7f}\u{7f}"))
}

pub fn f399(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a3078 = v1;
        let a3079 = 3u64;
        nat_mul(a3078, a3079)?
    };
    let a3090 = {
        let a3086 = {
            let a3080 = f75(v0.clone(), v2)?;
            let a3081 = 16384u64;
            nat_mul(a3080, a3081)?
        };
        let a3087 = {
            let a3084 = f75(v0.clone(), {
                let a3082 = v2;
                let a3083 = 1u64;
                nat_add(a3082, a3083)?
            })?;
            let a3085 = 128u64;
            nat_mul(a3084, a3085)?
        };
        nat_add(a3086, a3087)?
    };
    let a3091 = f75(v0, {
        let a3088 = v2;
        let a3089 = 2u64;
        nat_add(a3088, a3089)?
    })?;
    nat_add(a3090, a3091)
}

pub fn f400(v0: u64) -> R<bool> {
    f242(v0, f448())
}

pub fn f401(v0: u64) -> R<bool> {
    f242(v0, f449())
}

pub fn f402(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f450(v1, v0, v2)
}

pub fn f403(v0: Bytes, v1: u64) -> Bytes {
    f38(v0, v1, 1u64)
}

pub fn f404() -> Bytes {
    f8(Str::lit("\u{0}\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\u{9}\u{a}\u{b}\u{c}\u{d}\u{e}\u{f}\u{10}\u{11}\u{12}\u{13}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{1b}\u{1c}\u{1d}\u{1e}\u{1f} !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~\u{7f}"))
}

pub fn f405() -> Bytes {
    f8(Str::lit("\u{80}\u{c0}\u{100}\u{140}\u{180}\u{1c0}\u{200}\u{240}\u{280}\u{2c0}\u{300}\u{340}\u{380}\u{3c0}\u{400}\u{440}\u{480}\u{4c0}\u{500}\u{540}\u{580}\u{5c0}\u{600}\u{640}\u{680}\u{6c0}\u{700}\u{740}\u{780}\u{7c0}"))
}

pub fn f406(v0: u64) -> R<Bytes> {
    Ok(f403(f451(), {
        let a3097 = {
            let a3095 = {
                let a3092 = v0;
                let a3093 = 64u64;
                let a3094 = 0u64;
                nat_rem(a3092, a3093, a3094)
            };
            let a3096 = 2u64;
            nat_mul(a3095, a3096)?
        };
        let a3098 = 1u64;
        nat_add(a3097, a3098)?
    }))
}

pub fn f407() -> Bytes {
    f8(Str::lit("\u{800}\u{1000}\u{2000}\u{3000}\u{4000}\u{5000}\u{6000}\u{7000}\u{8000}\u{9000}\u{a000}\u{b000}\u{c000}\u{d000}\u{e000}\u{f000}"))
}

pub fn f408(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<Option<u64>> {
    if f452(f106(v0, v3)?, v1, v2)? {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3099 = v3;
            let a3100 = 1u64;
            nat_add(a3099, a3100)?
        }))
    }
}

pub fn f409(v0: Bytes) -> Result<Bytes, Bytes> {
    Err::<Bytes, Bytes>(v0)
}

pub fn f410() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f411(v0: Bytes) -> R<Option<Bytes>> {
    let m3101 = f157(v0.clone(), f8(Str::lit(" ")), 0u64)?;
    match m3101 {
        None => {
            let m3102 = f157(v0.clone(), f8(Str::lit(":")), 0u64)?;
            match m3102 {
                None => {
                    let v3: Bytes = f7(List::cons(f8(Str::lit(" ")), List::cons(v0, List::cons(f8(Str::lit(":")), List::<Bytes>::nil()))));
                    let m3103 = f157(f453(), v3.clone(), 0u64)?;
                    match m3103 {
                        None => {
                            Ok(None::<Bytes>)
                        }
                        Some(v4) => {
                            let v5: u64 = {
                                let a3105 = v4;
                                let a3106 = {
                                    let a3104 = v3;
                                    length_bytes(a3104)
                                };
                                nat_add(a3105, a3106)?
                            };
                            let m3107 = f157(f453(), f8(Str::lit(" ")), v5)?;
                            match m3107 {
                                None => {
                                    Ok(None::<Bytes>)
                                }
                                Some(v6) => {
                                    Ok(Some(f38(f453(), v5, {
                                        let a3108 = v6;
                                        let a3109 = v5;
                                        nat_sub(a3108, a3109)
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

pub fn f412(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a3110 = 0u64;
    let a3111 = v1;
    if if nat_lt(a3110, a3111) {
        f194({
            let a3114 = f121(v0, {
                let a3112 = v1;
                let a3113 = 1u64;
                nat_sub(a3112, a3113)
            })?;
            let a3115 = 10u64;
            nat_eq(a3114, a3115)
        })
    } else {
        false
    } {
        Ok(Some({
            let a3116 = v1;
            let a3117 = 1u64;
            nat_sub(a3116, a3117)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f413(v0: Bytes, v1: u64, v2: u64) -> R<Result<(u64, u64), Bytes>> {
    let v3: (u64, u64) = {
        let (h3118, _) = f211(Fn6::F454(v2, v0.clone()), {
            let a3120 = {
                let a3119 = v0.clone();
                length_bytes(a3119)
            };
            let a3121 = 1u64;
            nat_add(a3120, a3121)?
        }, (v1, 0u64))?;
        h3118
    };
    let v4: u64 = f414(v0.clone(), {
        let (h3122, _) = v3;
        h3122
    }, v2)?;
    let a3123 = f121(v0, v4)?;
    let a3124 = 9u64;
    if if nat_eq(a3123, a3124) {
        let a3128 = {
            let a3126 = v4;
            let a3127 = {
                let (h3125, _) = v3;
                h3125
            };
            nat_sub(a3126, a3127)
        };
        let a3129 = v2;
        nat_lt(a3128, a3129)
    } else {
        false
    } {
        Ok(Err::<(u64, u64), Bytes>(f102(Str::lit("a tab cannot indent a block scalar line (YAML 1.2.2 [63], [70])"), v4)?))
    } else {
        Ok(Ok::<(u64, u64), Bytes>(({
            let (_, h3130) = v3;
            h3130
        }, {
            let (h3131, _) = v3;
            h3131
        })))
    }
}

pub fn f414(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h3132, _) = f154(Fn4::F455(v2, v0.clone(), v1), {
        let a3134 = {
            let a3133 = v0;
            length_bytes(a3133)
        };
        let a3135 = 1u64;
        nat_add(a3134, a3135)?
    }, v1)?;
    Ok(h3132)
}

pub fn f415(v0: Bytes, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = f414(v0.clone(), v1, v2)?;
    let a3138 = {
        let a3136 = v3;
        let a3137 = v1;
        nat_sub(a3136, a3137)
    };
    let a3139 = v2;
    if nat_eq(a3138, a3139) {
        Ok(f194({
            let a3140 = f121(v0, v3)?;
            let a3141 = 256u64;
            nat_eq(a3140, a3141)
        }))
    } else {
        Ok(false)
    }
}

pub fn f416(v0: Bytes, p3155: u64, v2: u64, v3: bool, p3156: u64, p3157: Bytes, p3158: u64) -> R<Result<(Bytes, (u64, u64)), Bytes>> {
    let m3154 = (p3155, (p3156, (p3157, p3158)));
    {
        let mut m3154 = m3154;
        loop {
            let step = {
                let (v1, (v4, (v5, v6))) = m3154;
                let m3142 = v4;
                if m3142 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<Result<(Bytes, (u64, u64)), Bytes>>>(Ok(Err::<(Bytes, (u64, u64)), Bytes>(f102(Str::lit("a block scalar has more lines than the input"), v1)?)))
                } else {
                    let v7 = m3142 - 1;
                    let v8: u64 = f414(v0.clone(), v1, v2)?;
                    let v9: bool = f194(f182(f121(v0.clone(), v8)?));
                    let v10: u64 = f272(v0.clone(), v8)?;
                    let v11: Bytes = f7(List::cons(v5, List::cons(f346(v6), List::cons(f270(v0.clone(), v8, v10), List::<Bytes>::nil()))));
                    let m3143 = f413(v0.clone(), f192(v0.clone(), v10)?, v2)?;
                    match m3143 {
                        Ok(v13) => {
                            let v14: u64 = {
                                let (_, h3144) = v13;
                                h3144
                            };
                            let v15: u64 = f414(v0.clone(), v14, v2)?;
                            if f415(v0.clone(), v14, v2)? {
                                Ok::<(u64, (u64, (Bytes, u64))), R<Result<(Bytes, (u64, u64)), Bytes>>>((v14, (v7, ({
                                    let a3148 = v11;
                                    let a3149 = if if if v3 {
                                        v9
                                    } else {
                                        false
                                    } {
                                        f194(f182(f121(v0.clone(), v15)?))
                                    } else {
                                        false
                                    } {
                                        let a3146 = {
                                            let (h3145, _) = v13;
                                            h3145
                                        };
                                        let a3147 = 0u64;
                                        if nat_eq(a3146, a3147) {
                                            Bytes::lit(&[0x20])
                                        } else {
                                            Bytes::lit(&[])
                                        }
                                    } else {
                                        Bytes::lit(&[0x0a])
                                    };
                                    append_bytes(a3148, a3149)
                                }, {
                                    let (h3150, _) = v13;
                                    h3150
                                }))))
                            } else {
                                Err::<(u64, (u64, (Bytes, u64))), R<Result<(Bytes, (u64, u64)), Bytes>>>(Ok(Ok::<(Bytes, (u64, u64)), Bytes>((v11, ({
                                    let (h3151, _) = v13;
                                    h3151
                                }, {
                                    let a3152 = f121(v0.clone(), v15)?;
                                    let a3153 = 256u64;
                                    if nat_eq(a3152, a3153) {
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
                Ok(next) => { m3154 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f417(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f185(v0.clone(), {
        let (h3159, _) = v1;
        h3159
    })?;
    let a3160 = f121(v0, v2)?;
    let a3161 = 10u64;
    if nat_eq(a3160, a3161) {
        Ok(Some(({
            let a3162 = v2;
            let a3163 = 1u64;
            nat_add(a3162, a3163)?
        }, f348({
            let (_, h3164) = v1;
            h3164
        }, {
            let a3166 = v2;
            let a3167 = {
                let (h3165, _) = v1;
                h3165
            };
            nat_sub(a3166, a3167)
        }))))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f418(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f456(v0, v1)
}

pub fn f419(v0: u64) -> Option<Bytes> {
    let a3168 = v0;
    let a3169 = 48u64;
    if nat_eq(a3168, a3169) {
        Some(Bytes::lit(&[0x00]))
    } else {
        let a3170 = v0;
        let a3171 = 97u64;
        if nat_eq(a3170, a3171) {
            Some(Bytes::lit(&[0x07]))
        } else {
            let a3172 = v0;
            let a3173 = 98u64;
            if nat_eq(a3172, a3173) {
                Some(Bytes::lit(&[0x08]))
            } else {
                let a3174 = v0;
                let a3175 = 116u64;
                if if nat_eq(a3174, a3175) {
                    true
                } else {
                    let a3176 = v0;
                    let a3177 = 9u64;
                    nat_eq(a3176, a3177)
                } {
                    Some(Bytes::lit(&[0x09]))
                } else {
                    let a3178 = v0;
                    let a3179 = 110u64;
                    if nat_eq(a3178, a3179) {
                        Some(Bytes::lit(&[0x0a]))
                    } else {
                        let a3180 = v0;
                        let a3181 = 118u64;
                        if nat_eq(a3180, a3181) {
                            Some(Bytes::lit(&[0x0b]))
                        } else {
                            let a3182 = v0;
                            let a3183 = 102u64;
                            if nat_eq(a3182, a3183) {
                                Some(Bytes::lit(&[0x0c]))
                            } else {
                                let a3184 = v0;
                                let a3185 = 114u64;
                                if nat_eq(a3184, a3185) {
                                    Some(Bytes::lit(&[0x0d]))
                                } else {
                                    let a3186 = v0;
                                    let a3187 = 101u64;
                                    if nat_eq(a3186, a3187) {
                                        Some(Bytes::lit(&[0x1b]))
                                    } else {
                                        let a3188 = v0;
                                        let a3189 = 32u64;
                                        if nat_eq(a3188, a3189) {
                                            Some(Bytes::lit(&[0x20]))
                                        } else {
                                            let a3190 = v0;
                                            let a3191 = 34u64;
                                            if nat_eq(a3190, a3191) {
                                                Some(Bytes::lit(&[0x22]))
                                            } else {
                                                let a3192 = v0;
                                                let a3193 = 47u64;
                                                if nat_eq(a3192, a3193) {
                                                    Some(Bytes::lit(&[0x2f]))
                                                } else {
                                                    let a3194 = v0;
                                                    let a3195 = 92u64;
                                                    if nat_eq(a3194, a3195) {
                                                        Some(Bytes::lit(&[0x5c]))
                                                    } else {
                                                        let a3196 = v0;
                                                        let a3197 = 78u64;
                                                        if nat_eq(a3196, a3197) {
                                                            Some(Bytes::lit(&[0xc2, 0x85]))
                                                        } else {
                                                            let a3198 = v0;
                                                            let a3199 = 95u64;
                                                            if nat_eq(a3198, a3199) {
                                                                Some(Bytes::lit(&[0xc2, 0xa0]))
                                                            } else {
                                                                let a3200 = v0;
                                                                let a3201 = 76u64;
                                                                if nat_eq(a3200, a3201) {
                                                                    Some(Bytes::lit(&[0xe2, 0x80, 0xa8]))
                                                                } else {
                                                                    let a3202 = v0;
                                                                    let a3203 = 80u64;
                                                                    if nat_eq(a3202, a3203) {
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

pub fn f420(v0: u64) -> R<Bytes> {
    f102(Str::lit("an implicit key must be on one line (YAML 1.2.2 [154])"), v0)
}

pub fn f421(v0: Bytes, p3218: u64, v2: u64, p3219: u64, p3220: u64) -> R<Result<(u64, u64), Bytes>> {
    let m3217 = (p3218, (p3219, p3220));
    {
        let mut m3217 = m3217;
        loop {
            let step = {
                let (v1, (v3, v4)) = m3217;
                let m3204 = v3;
                if m3204 == 0 {
                    Err::<(u64, (u64, u64)), R<Result<(u64, u64), Bytes>>>(Ok(Err::<(u64, u64), Bytes>(f102(Str::lit("the input ends inside a scalar"), v1)?)))
                } else {
                    let v5 = m3204 - 1;
                    let v6: u64 = {
                        let a3205 = v1;
                        let a3206 = 1u64;
                        nat_add(a3205, a3206)?
                    };
                    let v7: u64 = f185(v0.clone(), v6)?;
                    let v8: u64 = f186(v0.clone(), v7)?;
                    let v9: u64 = f121(v0.clone(), v8)?;
                    if f120(v0.clone(), v6)? {
                        Err::<(u64, (u64, u64)), R<Result<(u64, u64), Bytes>>>(Ok(Err::<(u64, u64), Bytes>(f102(Str::lit("a document marker cannot appear inside a scalar (YAML 1.2.2 [206])"), v6)?)))
                    } else {
                        let a3207 = v9;
                        let a3208 = 10u64;
                        if nat_eq(a3207, a3208) {
                            Ok::<(u64, (u64, u64)), R<Result<(u64, u64), Bytes>>>((v8, (v5, {
                                let a3209 = v4;
                                let a3210 = 1u64;
                                nat_add(a3209, a3210)?
                            })))
                        } else {
                            let a3211 = v9;
                            let a3212 = 256u64;
                            if nat_eq(a3211, a3212) {
                                Err::<(u64, (u64, u64)), R<Result<(u64, u64), Bytes>>>(Ok(Err::<(u64, u64), Bytes>(f102(Str::lit("the input ends inside a quoted scalar (YAML 1.2.2 [109], [120])"), v8)?)))
                            } else {
                                let a3215 = {
                                    let a3213 = v7;
                                    let a3214 = v6;
                                    nat_sub(a3213, a3214)
                                };
                                let a3216 = v2;
                                if nat_lt(a3215, a3216) {
                                    Err::<(u64, (u64, u64)), R<Result<(u64, u64), Bytes>>>(Ok(Err::<(u64, u64), Bytes>(f102(Str::lit("a continuation line of a scalar is indented less than its node (YAML 1.2.2 [69])"), v7)?)))
                                } else {
                                    Err::<(u64, (u64, u64)), R<Result<(u64, u64), Bytes>>>(Ok(Ok::<(u64, u64), Bytes>((v4, v8))))
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

pub fn f422(v0: Bytes, p3230: u64, p3231: u64, p3232: u64) -> R<Option<u64>> {
    let m3229 = (p3230, (p3231, p3232));
    {
        let mut m3229 = m3229;
        loop {
            let step = {
                let (v1, (v2, v3)) = m3229;
                let m3221 = v2;
                if m3221 == 0 {
                    Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3221 - 1;
                    let m3222 = f457(f121(v0.clone(), v1)?);
                    match m3222 {
                        None => {
                            Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(None::<u64>))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, u64)), R<Option<u64>>>(({
                                let a3223 = v1;
                                let a3224 = 1u64;
                                nat_add(a3223, a3224)?
                            }, (v4, {
                                let a3227 = {
                                    let a3225 = v3;
                                    let a3226 = 16u64;
                                    nat_mul(a3225, a3226)?
                                };
                                let a3228 = v5;
                                nat_add(a3227, a3228)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3229 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f423(v0: Bytes, v1: u64) -> R<u64> {
    let (h3233, _) = f154(Fn4::F458(v0.clone()), {
        let a3235 = {
            let a3234 = v0;
            length_bytes(a3234)
        };
        let a3236 = 1u64;
        nat_add(a3235, a3236)?
    }, v1)?;
    Ok(h3233)
}

pub fn f424(v0: Bytes, v1: u64) -> R<u64> {
    let (h3237, _) = f154(Fn4::F459(v0.clone()), {
        let a3239 = {
            let a3238 = v0;
            length_bytes(a3238)
        };
        let a3240 = 1u64;
        nat_add(a3239, a3240)?
    }, v1)?;
    Ok(h3237)
}

pub fn f425(v0: Fn9, p3245: u64, p3246: (u64, bool)) -> R<((u64, bool), bool)> {
    let m3244 = (p3245, p3246);
    {
        let mut m3244 = m3244;
        loop {
            let step = {
                let (v1, v2) = m3244;
                let m3241 = v1;
                if m3241 == 0 {
                    Err::<(u64, (u64, bool)), R<((u64, bool), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3241 - 1;
                    let m3243 = {
                        let c3242: Fn9 = v0.clone();
                        c3242.apply(v2)?
                    };
                    match m3243 {
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
                Ok(next) => { m3244 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f426(v0: u64, v1: Bytes, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    f460(v1, v0, v2)
}

pub fn f427(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = {
        let (h3247, _) = v2;
        h3247
    };
    let v4: u64 = f121(v0.clone(), v3)?;
    if f183(v4) {
        Ok(None::<(u64, u64)>)
    } else {
        if f182(v4) {
            Ok(Some(({
                let a3248 = v3;
                let a3249 = 1u64;
                nat_add(a3248, a3249)?
            }, {
                let (_, h3250) = v2;
                h3250
            })))
        } else {
            let a3251 = v4;
            let a3252 = 58u64;
            if if nat_eq(a3251, a3252) {
                f194(f263(f121(v0.clone(), {
                    let a3253 = v3;
                    let a3254 = 1u64;
                    nat_add(a3253, a3254)?
                })?, v1))
            } else {
                false
            } {
                Ok(None::<(u64, u64)>)
            } else {
                let a3255 = v4;
                let a3256 = 35u64;
                if if nat_eq(a3255, a3256) {
                    f182(f121(v0, {
                        let a3257 = v3;
                        let a3258 = 1u64;
                        nat_sub(a3257, a3258)
                    })?)
                } else {
                    false
                } {
                    Ok(None::<(u64, u64)>)
                } else {
                    if if v1 {
                        f341(v4)
                    } else {
                        false
                    } {
                        Ok(None::<(u64, u64)>)
                    } else {
                        Ok(Some(({
                            let a3259 = v3;
                            let a3260 = 1u64;
                            nat_add(a3259, a3260)?
                        }, {
                            let a3261 = v3;
                            let a3262 = 1u64;
                            nat_add(a3261, a3262)?
                        })))
                    }
                }
            }
        }
    }
}

pub fn f428(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f461(v0, v1)
}

pub fn f429(v0: Bytes, v1: u64, v2: u64) -> R<(u64, u64)> {
    let (h3263, _) = f211(Fn6::F462(v1, v0.clone()), {
        let a3265 = {
            let a3264 = v0;
            length_bytes(a3264)
        };
        let a3266 = 1u64;
        nat_add(a3265, a3266)?
    }, (v2, 0u64))?;
    Ok(h3263)
}

pub fn f430(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f183(f121(v0, v1)?) {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3267 = v1;
            let a3268 = 1u64;
            nat_add(a3267, a3268)?
        }))
    }
}

pub fn f431(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m3272 = {
        let a3270 = v0.clone();
        let a3271 = {
            let (h3269, _) = v1.clone();
            h3269
        };
        index_bytes(a3270, a3271)
    };
    match m3272 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a3274 = {
                    let (h3273, _) = v1.clone();
                    h3273
                };
                let a3275 = 1u64;
                nat_add(a3274, a3275)?
            }, {
                let a3278 = {
                    let (_, h3276) = v1;
                    h3276
                };
                let a3279 = f463(v0, {
                    let (h3277, _) = v1;
                    h3277
                }, v2)?;
                append_bytes(a3278, a3279)
            })))
        }
    }
}

pub fn f432(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f464(Fn12::F465, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f433(v0: Fn10, p3283: List<List<(Bytes, Bytes)>>, p3284: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m3282 = (p3283, p3284);
    {
        let mut m3282 = m3282;
        loop {
            let step = {
                let (v1, v2) = m3282;
                let m3280 = v2;
                match m3280.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c3281: Fn10 = v0.clone();
                            c3281.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3282 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f434(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f435(v0: Fn11, p3289: u64, p3290: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m3288 = (p3289, p3290);
    {
        let mut m3288 = m3288;
        loop {
            let step = {
                let (v1, v2) = m3288;
                let m3285 = v1;
                if m3285 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3285 - 1;
                    let m3287 = {
                        let c3286: Fn11 = v0.clone();
                        c3286.apply(v2.clone())?
                    };
                    match m3287 {
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
                Ok(next) => { m3288 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f436(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f466(v0)
}

pub fn f437(v0: Bytes, v1: u64) -> R<Bytes> {
    let a3293 = f38(v0.clone(), {
        let a3291 = v1;
        let a3292 = 2u64;
        nat_add(a3291, a3292)?
    }, 2u64);
    let a3294 = Bytes::lit(&[0x30, 0x30]);
    if equal(a3293, a3294) {
        let m3297 = f467(f38(v0, {
            let a3295 = v1;
            let a3296 = 4u64;
            nat_add(a3295, a3296)?
        }, 2u64))?;
        match m3297 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f75(v2.clone(), 0u64)?;
                let a3298 = v3;
                let a3299 = 32u64;
                if nat_lt(a3298, a3299) {
                    if f468(v3) {
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

pub fn f438(v0: u64) -> Bytes {
    let a3300 = v0;
    let a3301 = 34u64;
    if nat_eq(a3300, a3301) {
        Bytes::lit(&[0x22])
    } else {
        let a3302 = v0;
        let a3303 = 92u64;
        if nat_eq(a3302, a3303) {
            Bytes::lit(&[0x5c])
        } else {
            let a3304 = v0;
            let a3305 = 98u64;
            if nat_eq(a3304, a3305) {
                Bytes::lit(&[0x08])
            } else {
                let a3306 = v0;
                let a3307 = 102u64;
                if nat_eq(a3306, a3307) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a3308 = v0;
                    let a3309 = 110u64;
                    if nat_eq(a3308, a3309) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a3310 = v0;
                        let a3311 = 114u64;
                        if nat_eq(a3310, a3311) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a3312 = v0;
                            let a3313 = 116u64;
                            if nat_eq(a3312, a3313) {
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

pub fn f439(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3358 = {
            let a3334 = {
                let a3322 = {
                    let a3316 = {
                        let a3314 = v1;
                        let a3315 = 128u8;
                        fixed_u8::bit_and(a3314, a3315)
                    };
                    let a3317 = 128u8;
                    if equal(a3316, a3317) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3323 = {
                    let a3320 = {
                        let a3318 = v1;
                        let a3319 = 64u8;
                        fixed_u8::bit_and(a3318, a3319)
                    };
                    let a3321 = 64u8;
                    if equal(a3320, a3321) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3322, a3323)?
            };
            let a3335 = {
                let a3332 = {
                    let a3326 = {
                        let a3324 = v1;
                        let a3325 = 32u8;
                        fixed_u8::bit_and(a3324, a3325)
                    };
                    let a3327 = 32u8;
                    if equal(a3326, a3327) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3333 = {
                    let a3330 = {
                        let a3328 = v1;
                        let a3329 = 16u8;
                        fixed_u8::bit_and(a3328, a3329)
                    };
                    let a3331 = 16u8;
                    if equal(a3330, a3331) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3332, a3333)?
            };
            nat_add(a3334, a3335)?
        };
        let a3359 = {
            let a3356 = {
                let a3344 = {
                    let a3338 = {
                        let a3336 = v1;
                        let a3337 = 8u8;
                        fixed_u8::bit_and(a3336, a3337)
                    };
                    let a3339 = 8u8;
                    if equal(a3338, a3339) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3345 = {
                    let a3342 = {
                        let a3340 = v1;
                        let a3341 = 4u8;
                        fixed_u8::bit_and(a3340, a3341)
                    };
                    let a3343 = 4u8;
                    if equal(a3342, a3343) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3344, a3345)?
            };
            let a3357 = {
                let a3354 = {
                    let a3348 = {
                        let a3346 = v1;
                        let a3347 = 2u8;
                        fixed_u8::bit_and(a3346, a3347)
                    };
                    let a3349 = 2u8;
                    if equal(a3348, a3349) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3355 = {
                    let a3352 = {
                        let a3350 = v1;
                        let a3351 = 1u8;
                        fixed_u8::bit_and(a3350, a3351)
                    };
                    let a3353 = 1u8;
                    if equal(a3352, a3353) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3354, a3355)?
            };
            nat_add(a3356, a3357)?
        };
        nat_add(a3358, a3359)?
    };
    let a3360 = 48u64;
    let a3361 = v2;
    if nat_le(a3360, a3361) {
        let a3362 = v2;
        let a3363 = 57u64;
        Ok(nat_le(a3362, a3363))
    } else {
        Ok(false)
    }
}

pub fn f440(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    if f469(v1, f106(v0, v2)?) {
        Ok(Some({
            let a3364 = v2;
            let a3365 = 1u64;
            nat_add(a3364, a3365)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f441(p3368: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, p3369: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))> {
    let m3367 = (p3368, p3369);
    {
        let mut m3367 = m3367;
        loop {
            let step = {
                let (v0, v1) = m3367;
                let m3366 = v0;
                match m3366.uncons() {
                    None => {
                        Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>), List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>), List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3367 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f442(v0: Bytes, p3374: List<(Bytes, (bool, Bytes))>) -> Option<(bool, Bytes)> {
    let m3373 = p3374;
    {
        let mut m3373 = m3373;
        loop {
            let step = {
                let v1 = m3373;
                let m3370 = v1;
                match m3370.uncons() {
                    None => {
                        Err::<List<(Bytes, (bool, Bytes))>, Option<(bool, Bytes)>>(None::<(bool, Bytes)>)
                    }
                    Some((v2, v3)) => {
                        if f41(v0.clone(), {
                            let (h3371, _) = v2;
                            h3371
                        }) {
                            Err::<List<(Bytes, (bool, Bytes))>, Option<(bool, Bytes)>>(Some({
                                let (_, h3372) = v2;
                                h3372
                            }))
                        } else {
                            Ok::<List<(Bytes, (bool, Bytes))>, Option<(bool, Bytes)>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3373 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f443() -> List<(Bytes, (bool, Bytes))> {
    List::cons((f8(Str::lit("boolean")), (true, f8(Str::lit("Bool")))), List::cons((f8(Str::lit("byte")), (true, f8(Str::lit("Int")))), List::cons((f8(Str::lit("octet")), (true, f8(Str::lit("Nat")))), List::cons((f8(Str::lit("bigint")), (true, f8(Str::lit("Int")))), List::cons((f8(Str::lit("float")), (true, f410())), List::cons((f8(Str::lit("double")), (true, f410())), List::cons((f8(Str::lit("DOMString")), (true, f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("USVString")), (true, f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ByteString")), (true, f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ArrayBuffer")), (true, f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("SharedArrayBuffer")), (true, f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("DataView")), (true, f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("Int8Array")), (true, f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int16Array")), (true, f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int32Array")), (true, f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("BigInt64Array")), (true, f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Uint8Array")), (true, f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint16Array")), (true, f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint32Array")), (true, f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint8ClampedArray")), (true, f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("BigUint64Array")), (true, f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Float16Array")), (false, f8(Str::lit("Float16Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("Float32Array")), (false, f8(Str::lit("Float32Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("Float64Array")), (false, f8(Str::lit("Float64Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("any")), (false, f8(Str::lit("any is not translated, because it is the union of every Web IDL type and has no one LexLean type (Web IDL 2.13.1)")))), List::cons((f8(Str::lit("object")), (false, f8(Str::lit("object is not translated, because its values are references to JavaScript objects of any shape, which no LexLean type holds (Web IDL 2.13.20)")))), List::cons((f8(Str::lit("symbol")), (false, f8(Str::lit("symbol is not translated, because a symbol value has identity but no content that a LexLean type holds (Web IDL 2.13.21)")))), List::cons((f8(Str::lit("undefined")), (false, f8(Str::lit("undefined is not translated, because it is the type of the absence of a value (Web IDL 2.13.2)")))), List::<(Bytes, (bool, Bytes))>::nil()))))))))))))))))))))))))))))
}

pub fn f444() -> Bytes {
    f8(Str::lit("a union type is translated only as the whole body of a typedef, whose identifier names its inductive (Web IDL 2.11 and 2.13.32)"))
}

pub fn f445(p3385: u64, v1: Bytes, v2: Bytes, p3386: u64) -> R<bool> {
    let m3384 = (p3385, p3386);
    {
        let mut m3384 = m3384;
        loop {
            let step = {
                let (v0, v3) = m3384;
                let m3375 = v0;
                if m3375 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(false))
                } else {
                    let v4 = m3375 - 1;
                    let v5: u64 = f470(v1.clone(), v3)?;
                    let v6: u64 = f470(v2.clone(), v3)?;
                    let a3376 = v5;
                    let a3377 = v6;
                    if nat_lt(a3376, a3377) {
                        Err::<(u64, u64), R<bool>>(Ok(true))
                    } else {
                        let a3378 = v6;
                        let a3379 = v5;
                        if nat_lt(a3378, a3379) {
                            Err::<(u64, u64), R<bool>>(Ok(false))
                        } else {
                            let a3380 = v5;
                            let a3381 = 0u64;
                            if nat_eq(a3380, a3381) {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            } else {
                                Ok::<(u64, u64), R<bool>>((v4, {
                                    let a3382 = v3;
                                    let a3383 = 1u64;
                                    nat_add(a3382, a3383)?
                                }))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3384 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f446(p3399: u64, v1: Bytes, p3400: u64, p3401: Bytes) -> R<Option<Bytes>> {
    let m3398 = (p3399, (p3400, p3401));
    {
        let mut m3398 = m3398;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3398;
                let m3387 = v0;
                if m3387 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3387 - 1;
                    let a3389 = {
                        let a3388 = v1.clone();
                        length_bytes(a3388)
                    };
                    let a3390 = v2;
                    if nat_le(a3389, a3390) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3391 = f319(v1.clone(), v2)?;
                        match m3391 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3393 = v2;
                                    let a3394 = {
                                        let (_, h3392) = v5;
                                        h3392
                                    };
                                    nat_add(a3393, a3394)?
                                }, {
                                    let a3396 = v3;
                                    let a3397 = f471(v1.clone(), v2, {
                                        let (h3395, _) = v5;
                                        h3395
                                    });
                                    append_bytes(a3396, a3397)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3398 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f447(v0: u64) -> Bytes {
    let a3402 = f8(Str::lit("u"));
    let a3403 = f472(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a3402, a3403)
}

pub fn f448() -> Bytes {
    f8(Str::lit("\u{0}\u{0}:\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{5}\u{7f}\u{0}\u{6}p\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f449() -> Bytes {
    f8(Str::lit("\u{0}\u{0}-\u{0}\u{0}.\u{0}\u{0}0\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}7\u{0}\u{1}7\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@?\u{0}@@\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f450(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f106(v0, {
        let (h3404, _) = v2;
        h3404
    })?;
    let a3406 = 1114111u64;
    let a3407 = {
        let (_, h3405) = v2;
        h3405
    };
    if nat_lt(a3406, a3407) {
        Ok(None::<(u64, u64)>)
    } else {
        let v4: u64 = f473(v3, v1);
        let a3408 = v4;
        let a3409 = 16u64;
        if nat_eq(a3408, a3409) {
            Ok(None::<(u64, u64)>)
        } else {
            Ok(Some(({
                let a3411 = {
                    let (h3410, _) = v2;
                    h3410
                };
                let a3412 = 1u64;
                nat_add(a3411, a3412)?
            }, {
                let a3416 = {
                    let a3414 = {
                        let (_, h3413) = v2;
                        h3413
                    };
                    let a3415 = if v1 {
                        16u64
                    } else {
                        10u64
                    };
                    nat_mul(a3414, a3415)?
                };
                let a3417 = v4;
                nat_add(a3416, a3417)?
            })))
        }
    }
}

pub fn f451() -> Bytes {
    f8(Str::lit("\u{80}\u{81}\u{82}\u{83}\u{84}\u{85}\u{86}\u{87}\u{88}\u{89}\u{8a}\u{8b}\u{8c}\u{8d}\u{8e}\u{8f}\u{90}\u{91}\u{92}\u{93}\u{94}\u{95}\u{96}\u{97}\u{98}\u{99}\u{9a}\u{9b}\u{9c}\u{9d}\u{9e}\u{9f}\u{a0}\u{a1}\u{a2}\u{a3}\u{a4}\u{a5}\u{a6}\u{a7}\u{a8}\u{a9}\u{aa}\u{ab}\u{ac}\u{ad}\u{ae}\u{af}\u{b0}\u{b1}\u{b2}\u{b3}\u{b4}\u{b5}\u{b6}\u{b7}\u{b8}\u{b9}\u{ba}\u{bb}\u{bc}\u{bd}\u{be}\u{bf}"))
}

pub fn f452(v0: u64, v1: bool, v2: u64) -> R<bool> {
    if v1 {
        let a3418 = v0;
        let a3419 = v2;
        if nat_eq(a3418, a3419) {
            Ok(true)
        } else {
            f242(v0, f474())
        }
    } else {
        f242(v0, f475())
    }
}

pub fn f453() -> Bytes {
    f8(Str::lit(" string:anyAtomicType boolean:anyAtomicType decimal:anyAtomicType float:anyAtomicType double:anyAtomicType duration:anyAtomicType dateTime:anyAtomicType time:anyAtomicType date:anyAtomicType gYearMonth:anyAtomicType gYear:anyAtomicType gMonthDay:anyAtomicType gDay:anyAtomicType gMonth:anyAtomicType hexBinary:anyAtomicType base64Binary:anyAtomicType anyURI:anyAtomicType QName:anyAtomicType NOTATION:anyAtomicType normalizedString:string token:normalizedString language:token NMTOKEN:token Name:token NCName:Name ID:NCName IDREF:NCName ENTITY:NCName integer:decimal nonPositiveInteger:integer long:integer nonNegativeInteger:integer negativeInteger:nonPositiveInteger int:long short:int byte:short unsignedLong:nonNegativeInteger positiveInteger:nonNegativeInteger unsignedInt:unsignedLong unsignedShort:unsignedInt unsignedByte:unsignedShort yearMonthDuration:duration dayTimeDuration:duration dateTimeStamp:dateTime NMTOKENS:anySimpleType IDREFS:anySimpleType ENTITIES:anySimpleType anyAtomicType:anySimpleType "))
}

pub fn f454(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f476(v1, v0, v2)
}

pub fn f455(v0: u64, v1: Bytes, v2: u64, v3: u64) -> R<Option<u64>> {
    f477(v1, v2, v0, v3)
}

pub fn f456(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let a3422 = {
        let (h3420, _) = v1;
        h3420
    };
    let a3423 = {
        let a3421 = v0.clone();
        length_bytes(a3421)
    };
    if nat_lt(a3422, a3423) {
        Ok(Some((f192(v0, {
            let (h3424, _) = v1;
            h3424
        })?, {
            let a3426 = {
                let (_, h3425) = v1;
                h3425
            };
            let a3427 = 1u64;
            nat_add(a3426, a3427)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f457(v0: u64) -> Option<u64> {
    let a3428 = 48u64;
    let a3429 = v0;
    if if nat_le(a3428, a3429) {
        let a3430 = v0;
        let a3431 = 57u64;
        nat_le(a3430, a3431)
    } else {
        false
    } {
        Some({
            let a3432 = v0;
            let a3433 = 48u64;
            nat_sub(a3432, a3433)
        })
    } else {
        let a3434 = 97u64;
        let a3435 = v0;
        if if nat_le(a3434, a3435) {
            let a3436 = v0;
            let a3437 = 102u64;
            nat_le(a3436, a3437)
        } else {
            false
        } {
            Some({
                let a3438 = v0;
                let a3439 = 87u64;
                nat_sub(a3438, a3439)
            })
        } else {
            let a3440 = 65u64;
            let a3441 = v0;
            if if nat_le(a3440, a3441) {
                let a3442 = v0;
                let a3443 = 70u64;
                nat_le(a3442, a3443)
            } else {
                false
            } {
                Some({
                    let a3444 = v0;
                    let a3445 = 55u64;
                    nat_sub(a3444, a3445)
                })
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f458(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f478(v0, v1)
}

pub fn f459(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f479(v0, v1)
}

pub fn f460(v0: Bytes, v1: u64, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    let v3: u64 = {
        let (h3446, _) = v2;
        h3446
    };
    let v4: u64 = f121(v0.clone(), v3)?;
    let (_, h3447) = v2;
    if h3447 {
        Ok(None::<(u64, bool)>)
    } else {
        if f182(v4) {
            Ok(Some(({
                let a3448 = v3;
                let a3449 = 1u64;
                nat_add(a3448, a3449)?
            }, false)))
        } else {
            let a3450 = v4;
            let a3451 = 35u64;
            if if nat_eq(a3450, a3451) {
                let a3452 = v3;
                let a3453 = 0u64;
                if if nat_eq(a3452, a3453) {
                    true
                } else {
                    f182(f121(v0.clone(), {
                        let a3454 = v3;
                        let a3455 = 1u64;
                        nat_sub(a3454, a3455)
                    })?)
                } {
                    true
                } else {
                    let a3458 = f121(v0.clone(), {
                        let a3456 = v3;
                        let a3457 = 1u64;
                        nat_sub(a3456, a3457)
                    })?;
                    let a3459 = 10u64;
                    nat_eq(a3458, a3459)
                }
            } else {
                false
            } {
                Ok(Some((f272(v0, v3)?, false)))
            } else {
                let a3460 = v4;
                let a3461 = 10u64;
                if nat_eq(a3460, a3461) {
                    let v5: u64 = {
                        let a3462 = v3;
                        let a3463 = 1u64;
                        nat_add(a3462, a3463)?
                    };
                    let v6: u64 = f185(v0.clone(), v5)?;
                    let v7: u64 = f186(v0.clone(), v6)?;
                    if if f120(v0.clone(), v5)? {
                        true
                    } else {
                        let a3466 = {
                            let a3464 = v6;
                            let a3465 = v5;
                            nat_sub(a3464, a3465)
                        };
                        let a3467 = v1;
                        if if nat_lt(a3466, a3467) {
                            f194(f183(f121(v0.clone(), v7)?))
                        } else {
                            false
                        } {
                            f194({
                                let a3468 = f121(v0, v7)?;
                                let a3469 = 35u64;
                                nat_eq(a3468, a3469)
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

pub fn f461(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f186(v0.clone(), {
        let (h3470, _) = v1;
        h3470
    })?;
    let a3471 = f121(v0, v2)?;
    let a3472 = 10u64;
    if nat_eq(a3471, a3472) {
        Ok(Some(({
            let a3473 = v2;
            let a3474 = 1u64;
            nat_add(a3473, a3474)?
        }, {
            let a3476 = {
                let (_, h3475) = v1;
                h3475
            };
            let a3477 = 1u64;
            nat_add(a3476, a3477)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f462(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f480(v1, v0, v2)
}

pub fn f463(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a3522 = {
            let a3498 = {
                let a3486 = {
                    let a3480 = {
                        let a3478 = v3;
                        let a3479 = 128u8;
                        fixed_u8::bit_and(a3478, a3479)
                    };
                    let a3481 = 128u8;
                    if equal(a3480, a3481) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3487 = {
                    let a3484 = {
                        let a3482 = v3;
                        let a3483 = 64u8;
                        fixed_u8::bit_and(a3482, a3483)
                    };
                    let a3485 = 64u8;
                    if equal(a3484, a3485) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3486, a3487)?
            };
            let a3499 = {
                let a3496 = {
                    let a3490 = {
                        let a3488 = v3;
                        let a3489 = 32u8;
                        fixed_u8::bit_and(a3488, a3489)
                    };
                    let a3491 = 32u8;
                    if equal(a3490, a3491) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3497 = {
                    let a3494 = {
                        let a3492 = v3;
                        let a3493 = 16u8;
                        fixed_u8::bit_and(a3492, a3493)
                    };
                    let a3495 = 16u8;
                    if equal(a3494, a3495) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3496, a3497)?
            };
            nat_add(a3498, a3499)?
        };
        let a3523 = {
            let a3520 = {
                let a3508 = {
                    let a3502 = {
                        let a3500 = v3;
                        let a3501 = 8u8;
                        fixed_u8::bit_and(a3500, a3501)
                    };
                    let a3503 = 8u8;
                    if equal(a3502, a3503) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3509 = {
                    let a3506 = {
                        let a3504 = v3;
                        let a3505 = 4u8;
                        fixed_u8::bit_and(a3504, a3505)
                    };
                    let a3507 = 4u8;
                    if equal(a3506, a3507) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3508, a3509)?
            };
            let a3521 = {
                let a3518 = {
                    let a3512 = {
                        let a3510 = v3;
                        let a3511 = 2u8;
                        fixed_u8::bit_and(a3510, a3511)
                    };
                    let a3513 = 2u8;
                    if equal(a3512, a3513) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3519 = {
                    let a3516 = {
                        let a3514 = v3;
                        let a3515 = 1u8;
                        fixed_u8::bit_and(a3514, a3515)
                    };
                    let a3517 = 1u8;
                    if equal(a3516, a3517) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3518, a3519)?
            };
            nat_add(a3520, a3521)?
        };
        nat_add(a3522, a3523)?
    };
    let a3524 = v4;
    let a3525 = 34u64;
    if nat_eq(a3524, a3525) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a3526 = v4;
        let a3527 = 92u64;
        if nat_eq(a3526, a3527) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a3528 = v4;
            let a3529 = 8u64;
            if nat_eq(a3528, a3529) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a3530 = v4;
                let a3531 = 12u64;
                if nat_eq(a3530, a3531) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a3532 = v4;
                    let a3533 = 10u64;
                    if nat_eq(a3532, a3533) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a3534 = v4;
                        let a3535 = 13u64;
                        if nat_eq(a3534, a3535) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a3536 = v4;
                            let a3537 = 9u64;
                            if nat_eq(a3536, a3537) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a3538 = v4;
                                let a3539 = 32u64;
                                if nat_lt(a3538, a3539) {
                                    let a3540 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a3541 = f481(v2)?;
                                    Ok(append_bytes(a3540, a3541))
                                } else {
                                    Ok(f38(v0, v1, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f464(v0: Fn12, p3545: List<List<(Bytes, Bytes)>>, p3546: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m3544 = (p3545, p3546);
    {
        let mut m3544 = m3544;
        loop {
            let step = {
                let (v1, v2) = m3544;
                let m3542 = v2;
                match m3542.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c3543: Fn12 = v0.clone();
                            c3543.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3544 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f465(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f466(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a3548 = {
        let a3547 = v0.clone();
        length_list(a3547)
    };
    let a3549 = 1u64;
    if nat_le(a3548, a3549) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f482(v0)?))
    }
}

pub fn f467(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a3550 = v0.clone();
        length_bytes(a3550)
    };
    let a3554 = {
        let a3551 = v1;
        let a3552 = 2u64;
        let a3553 = 0u64;
        nat_rem(a3551, a3552, a3553)
    };
    let a3555 = 0u64;
    if nat_eq(a3554, a3555) {
        let (_, h3556) = {
            let (h3557, _) = f483(Fn13::F484(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h3557
        };
        Ok(h3556)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f468(v0: u64) -> bool {
    let a3558 = v0;
    let a3559 = 8u64;
    if nat_eq(a3558, a3559) {
        true
    } else {
        let a3560 = v0;
        let a3561 = 9u64;
        if nat_eq(a3560, a3561) {
            true
        } else {
            let a3562 = v0;
            let a3563 = 10u64;
            if nat_eq(a3562, a3563) {
                true
            } else {
                let a3564 = v0;
                let a3565 = 12u64;
                if nat_eq(a3564, a3565) {
                    true
                } else {
                    let a3566 = v0;
                    let a3567 = 13u64;
                    nat_eq(a3566, a3567)
                }
            }
        }
    }
}

pub fn f469(v0: u64, v1: u64) -> bool {
    let a3568 = v0;
    let a3569 = 0u64;
    if nat_eq(a3568, a3569) {
        f373(v1)
    } else {
        let a3570 = v0;
        let a3571 = 1u64;
        if nat_eq(a3570, a3571) {
            f372(v1)
        } else {
            let a3572 = v0;
            let a3573 = 2u64;
            if nat_eq(a3572, a3573) {
                f485(v1)
            } else {
                f486(v1)
            }
        }
    }
}

pub fn f470(v0: Bytes, v1: u64) -> R<u64> {
    let a3575 = v1;
    let a3576 = {
        let a3574 = v0.clone();
        length_bytes(a3574)
    };
    if nat_lt(a3575, a3576) {
        let a3577 = f106(v0, v1)?;
        let a3578 = 1u64;
        nat_add(a3577, a3578)
    } else {
        Ok(0u64)
    }
}

pub fn f471(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f129(f77(f129(f333(v2), f334(v2)), f77({
        let a3579 = v2;
        let a3580 = 117u64;
        !nat_eq(a3579, a3580)
    }, {
        let a3581 = v2;
        let a3582 = 85u64;
        !nat_eq(a3581, a3582)
    })), f77(f335(v2), {
        let a3583 = 0u64;
        let a3584 = v1;
        nat_lt(a3583, a3584)
    })) {
        f38(v0, v1, 1u64)
    } else {
        let a3585 = v2;
        let a3586 = 65536u64;
        if nat_lt(a3585, a3586) {
            let a3587 = f8(Str::lit("u"));
            let a3588 = f472(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a3587, a3588)
        } else {
            let a3589 = f8(Str::lit("U"));
            let a3590 = f472(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a3589, a3590)
        }
    }
}

pub fn f472(p3607: u64, p3608: u64, p3609: u64, p3610: Bytes) -> Bytes {
    let m3606 = (p3607, (p3608, (p3609, p3610)));
    {
        let mut m3606 = m3606;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m3606;
                let m3591 = v0;
                if m3591 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m3591 - 1;
                    if f77({
                        let a3592 = v1;
                        let a3593 = 0u64;
                        nat_eq(a3592, a3593)
                    }, {
                        let a3594 = v2;
                        let a3595 = 0u64;
                        nat_eq(a3594, a3595)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a3596 = v1;
                            let a3597 = 16u64;
                            let a3598 = 0u64;
                            nat_quot(a3596, a3597, a3598)
                        }, ({
                            let a3599 = v2;
                            let a3600 = 1u64;
                            nat_sub(a3599, a3600)
                        }, {
                            let a3604 = f487({
                                let a3601 = v1;
                                let a3602 = 16u64;
                                let a3603 = 0u64;
                                nat_rem(a3601, a3602, a3603)
                            });
                            let a3605 = v3;
                            append_bytes(a3604, a3605)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m3606 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f473(v0: u64, v1: bool) -> u64 {
    let a3611 = v0;
    let a3612 = 48u64;
    if nat_lt(a3611, a3612) {
        16u64
    } else {
        let a3613 = v0;
        let a3614 = 57u64;
        if nat_le(a3613, a3614) {
            let a3615 = v0;
            let a3616 = 48u64;
            nat_sub(a3615, a3616)
        } else {
            if v1 {
                let a3617 = v0;
                let a3618 = 65u64;
                if nat_lt(a3617, a3618) {
                    16u64
                } else {
                    let a3619 = v0;
                    let a3620 = 70u64;
                    if nat_le(a3619, a3620) {
                        let a3621 = v0;
                        let a3622 = 55u64;
                        nat_sub(a3621, a3622)
                    } else {
                        let a3623 = v0;
                        let a3624 = 97u64;
                        if nat_lt(a3623, a3624) {
                            16u64
                        } else {
                            let a3625 = v0;
                            let a3626 = 102u64;
                            if nat_le(a3625, a3626) {
                                let a3627 = v0;
                                let a3628 = 87u64;
                                nat_sub(a3627, a3628)
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

pub fn f474() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f475() -> Bytes {
    f8(Str::lit("\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{0}]\u{0}\u{0}]\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f476(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f414(v0.clone(), {
        let (h3629, _) = v2;
        h3629
    }, v1)?;
    let a3630 = f121(v0, v3)?;
    let a3631 = 10u64;
    if nat_eq(a3630, a3631) {
        Ok(Some(({
            let a3632 = v3;
            let a3633 = 1u64;
            nat_add(a3632, a3633)?
        }, {
            let a3635 = {
                let (_, h3634) = v2;
                h3634
            };
            let a3636 = 1u64;
            nat_add(a3635, a3636)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f477(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a3637 = f121(v0, v3)?;
    let a3638 = 32u64;
    if if nat_eq(a3637, a3638) {
        let a3641 = {
            let a3639 = v3;
            let a3640 = v1;
            nat_sub(a3639, a3640)
        };
        let a3642 = v2;
        nat_lt(a3641, a3642)
    } else {
        false
    } {
        Ok(Some({
            let a3643 = v3;
            let a3644 = 1u64;
            nat_add(a3643, a3644)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f478(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f121(v0, v1)?;
    let a3645 = v2;
    let a3646 = 34u64;
    if if if if nat_eq(a3645, a3646) {
        true
    } else {
        let a3647 = v2;
        let a3648 = 92u64;
        nat_eq(a3647, a3648)
    } {
        true
    } else {
        f182(v2)
    } {
        true
    } else {
        f183(v2)
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3649 = v1;
            let a3650 = 1u64;
            nat_add(a3649, a3650)?
        }))
    }
}

pub fn f479(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f121(v0, v1)?;
    let a3651 = v2;
    let a3652 = 39u64;
    if if if nat_eq(a3651, a3652) {
        true
    } else {
        f182(v2)
    } {
        true
    } else {
        f183(v2)
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3653 = v1;
            let a3654 = 1u64;
            nat_add(a3653, a3654)?
        }))
    }
}

pub fn f480(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m3656 = f457(f121(v0, {
        let (h3655, _) = v2;
        h3655
    })?);
    match m3656 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v3) => {
            let a3657 = v3;
            let a3658 = v1;
            if nat_lt(a3657, a3658) {
                Ok(Some(({
                    let a3660 = {
                        let (h3659, _) = v2;
                        h3659
                    };
                    let a3661 = 1u64;
                    nat_add(a3660, a3661)?
                }, {
                    let a3665 = {
                        let a3663 = {
                            let (_, h3662) = v2;
                            h3662
                        };
                        let a3664 = v1;
                        nat_mul(a3663, a3664)?
                    };
                    let a3666 = v3;
                    nat_add(a3665, a3666)?
                })))
            } else {
                Ok(None::<(u64, u64)>)
            }
        }
    }
}

pub fn f481(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3711 = {
            let a3687 = {
                let a3675 = {
                    let a3669 = {
                        let a3667 = v1;
                        let a3668 = 128u8;
                        fixed_u8::bit_and(a3667, a3668)
                    };
                    let a3670 = 128u8;
                    if equal(a3669, a3670) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3676 = {
                    let a3673 = {
                        let a3671 = v1;
                        let a3672 = 64u8;
                        fixed_u8::bit_and(a3671, a3672)
                    };
                    let a3674 = 64u8;
                    if equal(a3673, a3674) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3675, a3676)?
            };
            let a3688 = {
                let a3685 = {
                    let a3679 = {
                        let a3677 = v1;
                        let a3678 = 32u8;
                        fixed_u8::bit_and(a3677, a3678)
                    };
                    let a3680 = 32u8;
                    if equal(a3679, a3680) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3686 = {
                    let a3683 = {
                        let a3681 = v1;
                        let a3682 = 16u8;
                        fixed_u8::bit_and(a3681, a3682)
                    };
                    let a3684 = 16u8;
                    if equal(a3683, a3684) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3685, a3686)?
            };
            nat_add(a3687, a3688)?
        };
        let a3712 = {
            let a3709 = {
                let a3697 = {
                    let a3691 = {
                        let a3689 = v1;
                        let a3690 = 8u8;
                        fixed_u8::bit_and(a3689, a3690)
                    };
                    let a3692 = 8u8;
                    if equal(a3691, a3692) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3698 = {
                    let a3695 = {
                        let a3693 = v1;
                        let a3694 = 4u8;
                        fixed_u8::bit_and(a3693, a3694)
                    };
                    let a3696 = 4u8;
                    if equal(a3695, a3696) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3697, a3698)?
            };
            let a3710 = {
                let a3707 = {
                    let a3701 = {
                        let a3699 = v1;
                        let a3700 = 2u8;
                        fixed_u8::bit_and(a3699, a3700)
                    };
                    let a3702 = 2u8;
                    if equal(a3701, a3702) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3708 = {
                    let a3705 = {
                        let a3703 = v1;
                        let a3704 = 1u8;
                        fixed_u8::bit_and(a3703, a3704)
                    };
                    let a3706 = 1u8;
                    if equal(a3705, a3706) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3707, a3708)?
            };
            nat_add(a3709, a3710)?
        };
        nat_add(a3711, a3712)?
    };
    let a3719 = f150(f488(), {
        let a3713 = v2;
        let a3714 = 16u64;
        let a3715 = 0u64;
        nat_quot(a3713, a3714, a3715)
    });
    let a3720 = f150(f488(), {
        let a3716 = v2;
        let a3717 = 16u64;
        let a3718 = 0u64;
        nat_rem(a3716, a3717, a3718)
    });
    Ok(append_bytes(a3719, a3720))
}

pub fn f482(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f432({
        let (_, h3721) = {
            let (h3722, _) = f489(Fn14::F490, {
                let a3724 = {
                    let a3723 = v0.clone();
                    length_list(a3723)
                };
                let a3725 = 1u64;
                nat_add(a3724, a3725)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h3722
        };
        h3721
    }))
}

pub fn f483(v0: Fn13, p3730: u64, p3731: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m3729 = (p3730, p3731);
    {
        let mut m3729 = m3729;
        loop {
            let step = {
                let (v1, v2) = m3729;
                let m3726 = v1;
                if m3726 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3726 - 1;
                    let m3728 = {
                        let c3727: Fn13 = v0.clone();
                        c3727.apply(v2.clone())?
                    };
                    match m3728 {
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
                Ok(next) => { m3729 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f484(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f491(v0, v1)
}

pub fn f485(v0: u64) -> bool {
    let a3732 = 48u64;
    let a3733 = v0;
    if nat_le(a3732, a3733) {
        let a3734 = v0;
        let a3735 = 55u64;
        nat_le(a3734, a3735)
    } else {
        false
    }
}

pub fn f486(v0: u64) -> bool {
    if f288(v0) {
        true
    } else {
        if f373(v0) {
            true
        } else {
            let a3736 = v0;
            let a3737 = 95u64;
            if nat_eq(a3736, a3737) {
                true
            } else {
                let a3738 = v0;
                let a3739 = 45u64;
                nat_eq(a3738, a3739)
            }
        }
    }
}

pub fn f487(v0: u64) -> Bytes {
    f38({
        let a3740 = Str::lit("0123456789ABCDEF");
        utf8_encode(a3740)
    }, v0, 1u64)
}

pub fn f488() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f489(v0: Fn14, p3745: u64, p3746: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m3744 = (p3745, p3746);
    {
        let mut m3744 = m3744;
        loop {
            let step = {
                let (v1, v2) = m3744;
                let m3741 = v1;
                if m3741 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3741 - 1;
                    let m3743 = {
                        let c3742: Fn14 = v0.clone();
                        c3742.apply(v2.clone())?
                    };
                    match m3743 {
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
                Ok(next) => { m3744 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f490(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f492(v0)
}

pub fn f491(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h3747, _) = v1;
        h3747
    };
    let m3749 = {
        let (_, h3748) = v1;
        h3748
    };
    match m3749 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m3752 = {
                let a3750 = v0.clone();
                let a3751 = v2;
                index_bytes(a3750, a3751)
            };
            match m3752 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m3757 = {
                        let a3755 = v0;
                        let a3756 = {
                            let a3753 = v2;
                            let a3754 = 1u64;
                            nat_add(a3753, a3754)?
                        };
                        index_bytes(a3755, a3756)
                    };
                    match m3757 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f493(v4)? {
                                f493(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a3758 = v2;
                                    let a3759 = 2u64;
                                    nat_add(a3758, a3759)?
                                }, Some({
                                    let a3764 = v3;
                                    let a3765 = f150(f494(), {
                                        let a3762 = {
                                            let a3760 = f495(v4)?;
                                            let a3761 = 16u64;
                                            nat_mul(a3760, a3761)?
                                        };
                                        let a3763 = f495(v5)?;
                                        nat_add(a3762, a3763)?
                                    });
                                    append_bytes(a3764, a3765)
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

pub fn f492(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h3766, _) = v0;
        h3766
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h3767) = v0;
        h3767
    };
    let m3768 = v1;
    match m3768.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m3769 = v4;
            match m3769.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f496(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f493(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3814 = {
            let a3790 = {
                let a3778 = {
                    let a3772 = {
                        let a3770 = v1;
                        let a3771 = 128u8;
                        fixed_u8::bit_and(a3770, a3771)
                    };
                    let a3773 = 128u8;
                    if equal(a3772, a3773) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3779 = {
                    let a3776 = {
                        let a3774 = v1;
                        let a3775 = 64u8;
                        fixed_u8::bit_and(a3774, a3775)
                    };
                    let a3777 = 64u8;
                    if equal(a3776, a3777) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3778, a3779)?
            };
            let a3791 = {
                let a3788 = {
                    let a3782 = {
                        let a3780 = v1;
                        let a3781 = 32u8;
                        fixed_u8::bit_and(a3780, a3781)
                    };
                    let a3783 = 32u8;
                    if equal(a3782, a3783) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3789 = {
                    let a3786 = {
                        let a3784 = v1;
                        let a3785 = 16u8;
                        fixed_u8::bit_and(a3784, a3785)
                    };
                    let a3787 = 16u8;
                    if equal(a3786, a3787) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3788, a3789)?
            };
            nat_add(a3790, a3791)?
        };
        let a3815 = {
            let a3812 = {
                let a3800 = {
                    let a3794 = {
                        let a3792 = v1;
                        let a3793 = 8u8;
                        fixed_u8::bit_and(a3792, a3793)
                    };
                    let a3795 = 8u8;
                    if equal(a3794, a3795) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3801 = {
                    let a3798 = {
                        let a3796 = v1;
                        let a3797 = 4u8;
                        fixed_u8::bit_and(a3796, a3797)
                    };
                    let a3799 = 4u8;
                    if equal(a3798, a3799) {
                        4u64
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
                        let a3803 = 2u8;
                        fixed_u8::bit_and(a3802, a3803)
                    };
                    let a3805 = 2u8;
                    if equal(a3804, a3805) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3811 = {
                    let a3808 = {
                        let a3806 = v1;
                        let a3807 = 1u8;
                        fixed_u8::bit_and(a3806, a3807)
                    };
                    let a3809 = 1u8;
                    if equal(a3808, a3809) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3810, a3811)?
            };
            nat_add(a3812, a3813)?
        };
        nat_add(a3814, a3815)?
    };
    let a3816 = 48u64;
    let a3817 = v2;
    if if nat_le(a3816, a3817) {
        let a3818 = v2;
        let a3819 = 57u64;
        nat_le(a3818, a3819)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a3820 = 97u64;
        let a3821 = v2;
        if nat_le(a3820, a3821) {
            let a3822 = v2;
            let a3823 = 102u64;
            Ok(nat_le(a3822, a3823))
        } else {
            Ok(false)
        }
    }
}

pub fn f494() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f495(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3868 = {
            let a3844 = {
                let a3832 = {
                    let a3826 = {
                        let a3824 = v1;
                        let a3825 = 128u8;
                        fixed_u8::bit_and(a3824, a3825)
                    };
                    let a3827 = 128u8;
                    if equal(a3826, a3827) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3833 = {
                    let a3830 = {
                        let a3828 = v1;
                        let a3829 = 64u8;
                        fixed_u8::bit_and(a3828, a3829)
                    };
                    let a3831 = 64u8;
                    if equal(a3830, a3831) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3832, a3833)?
            };
            let a3845 = {
                let a3842 = {
                    let a3836 = {
                        let a3834 = v1;
                        let a3835 = 32u8;
                        fixed_u8::bit_and(a3834, a3835)
                    };
                    let a3837 = 32u8;
                    if equal(a3836, a3837) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3843 = {
                    let a3840 = {
                        let a3838 = v1;
                        let a3839 = 16u8;
                        fixed_u8::bit_and(a3838, a3839)
                    };
                    let a3841 = 16u8;
                    if equal(a3840, a3841) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3842, a3843)?
            };
            nat_add(a3844, a3845)?
        };
        let a3869 = {
            let a3866 = {
                let a3854 = {
                    let a3848 = {
                        let a3846 = v1;
                        let a3847 = 8u8;
                        fixed_u8::bit_and(a3846, a3847)
                    };
                    let a3849 = 8u8;
                    if equal(a3848, a3849) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3855 = {
                    let a3852 = {
                        let a3850 = v1;
                        let a3851 = 4u8;
                        fixed_u8::bit_and(a3850, a3851)
                    };
                    let a3853 = 4u8;
                    if equal(a3852, a3853) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3854, a3855)?
            };
            let a3867 = {
                let a3864 = {
                    let a3858 = {
                        let a3856 = v1;
                        let a3857 = 2u8;
                        fixed_u8::bit_and(a3856, a3857)
                    };
                    let a3859 = 2u8;
                    if equal(a3858, a3859) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3865 = {
                    let a3862 = {
                        let a3860 = v1;
                        let a3861 = 1u8;
                        fixed_u8::bit_and(a3860, a3861)
                    };
                    let a3863 = 1u8;
                    if equal(a3862, a3863) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3864, a3865)?
            };
            nat_add(a3866, a3867)?
        };
        nat_add(a3868, a3869)?
    };
    let a3870 = v2;
    let a3871 = 57u64;
    if nat_le(a3870, a3871) {
        let a3872 = v2;
        let a3873 = 48u64;
        Ok(nat_sub(a3872, a3873))
    } else {
        let a3874 = v2;
        let a3875 = 87u64;
        Ok(nat_sub(a3874, a3875))
    }
}

pub fn f496(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3880 = {
            let a3878 = {
                let a3876 = v0.clone();
                length_list(a3876)
            };
            let a3879 = {
                let a3877 = v1.clone();
                length_list(a3877)
            };
            nat_add(a3878, a3879)?
        };
        let a3881 = 1u64;
        nat_add(a3880, a3881)?
    };
    Ok(f497({
        let (_, h3882) = {
            let (_, h3883) = {
                let (h3884, _) = f498(Fn15::F499, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3884
            };
            h3883
        };
        h3882
    }))
}

pub fn f497(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f275(Fn7::F500, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f498(v0: Fn15, p3889: u64, p3890: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3888 = (p3889, p3890);
    {
        let mut m3888 = m3888;
        loop {
            let step = {
                let (v1, v2) = m3888;
                let m3885 = v1;
                if m3885 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3885 - 1;
                    let m3887 = {
                        let c3886: Fn15 = v0.clone();
                        c3886.apply(v2.clone())
                    };
                    match m3887 {
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
                Ok(next) => { m3888 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f499(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f501(v0)
}

pub fn f500(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f501(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h3891, _) = v0.clone();
        h3891
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h3892, _) = {
            let (_, h3893) = v0.clone();
            h3893
        };
        h3892
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h3894) = {
            let (_, h3895) = v0;
            h3895
        };
        h3894
    };
    let m3896 = v1.clone();
    match m3896.uncons() {
        None => {
            let m3897 = v2;
            match m3897.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3898 = v2.clone();
            match m3898.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f502({
                        let (h3899, _) = v8.clone();
                        h3899
                    }, {
                        let (h3900, _) = v6.clone();
                        h3900
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

pub fn f502(v0: Bytes, v1: Bytes) -> bool {
    let a3905 = {
        let a3901 = v0;
        let a3902 = v1;
        compare_bytes(a3901, a3902)
    };
    let a3906 = {
        let a3903 = Bytes::lit(&[]);
        let a3904 = Bytes::lit(&[0x01]);
        compare_bytes(a3903, a3904)
    };
    equal(a3905, a3906)
}

pub fn run(p0: Option<Adt0>, p1: Adt3) -> R<(Adt0, Adt10)> {
    f0(p0, p1)
}
