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
    C0(Adt2, u64),
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
    C0(Bytes, Adt17, Adt17, List<(Bytes, Bytes)>),
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
    C0(Adt15, List<Adt19>),
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
    F56(u8, Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F56(k0, k1) => f56(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F59(Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn1::F59(k0) => f59(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F86,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn2::F86 => f86(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F162(u64, Bytes),
    F163(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn3::F162(k0, k1) => Ok(f162(*k0, k1.clone(), p0)),
            Fn3::F163(k0) => f163(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F165(Bytes),
    F190(Bytes),
    F191(Bytes),
    F217(Bytes),
    F218(Bytes),
    F253(Bytes, Bytes),
    F254(Bytes),
    F255(Bytes, u64),
    F270(Bytes),
    F271(Bytes),
    F295(Bytes),
    F344(bool, u64, Bytes),
    F355(Bytes),
    F378(Bytes),
    F385(Bytes),
    F388(u64, Bytes),
    F478(u64, Bytes, u64),
    F481(Bytes),
    F482(Bytes),
    F484(Bytes, u64),
}

impl Fn4 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn4::F165(k0) => f165(k0.clone(), p0),
            Fn4::F190(k0) => f190(k0.clone(), p0),
            Fn4::F191(k0) => f191(k0.clone(), p0),
            Fn4::F217(k0) => f217(k0.clone(), p0),
            Fn4::F218(k0) => f218(k0.clone(), p0),
            Fn4::F253(k0, k1) => f253(k0.clone(), k1.clone(), p0),
            Fn4::F254(k0) => f254(k0.clone(), p0),
            Fn4::F255(k0, k1) => f255(k0.clone(), *k1, p0),
            Fn4::F270(k0) => f270(k0.clone(), p0),
            Fn4::F271(k0) => f271(k0.clone(), p0),
            Fn4::F295(k0) => f295(k0.clone(), p0),
            Fn4::F344(k0, k1, k2) => f344(*k0, *k1, k2.clone(), p0),
            Fn4::F355(k0) => f355(k0.clone(), p0),
            Fn4::F378(k0) => f378(k0.clone(), p0),
            Fn4::F385(k0) => f385(k0.clone(), p0),
            Fn4::F388(k0, k1) => f388(*k0, k1.clone(), p0),
            Fn4::F478(k0, k1, k2) => f478(*k0, k1.clone(), *k2, p0),
            Fn4::F481(k0) => f481(k0.clone(), p0),
            Fn4::F482(k0) => f482(k0.clone(), p0),
            Fn4::F484(k0, k1) => f484(k0.clone(), *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F221(Bytes),
    F362(Bytes),
    F371(bool, Bytes),
    F419(bool, Bytes),
    F435(Bytes),
    F446(Bytes),
    F477(u64, Bytes),
    F486(u64, Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn5::F221(k0) => f221(k0.clone(), p0),
            Fn5::F362(k0) => f362(k0.clone(), p0),
            Fn5::F371(k0, k1) => f371(*k0, k1.clone(), p0),
            Fn5::F419(k0, k1) => f419(*k0, k1.clone(), p0),
            Fn5::F435(k0) => f435(k0.clone(), p0),
            Fn5::F446(k0) => f446(k0.clone(), p0),
            Fn5::F477(k0, k1) => f477(*k0, k1.clone(), p0),
            Fn5::F486(k0, k1) => f486(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F285,
    F520,
}

impl Fn6 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn6::F285 => f285(p0, p1),
            Fn6::F520 => f520(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F288,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn7::F288 => f288(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F443(u64, Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: (u64, bool)) -> R<Option<(u64, bool)>> {
        match self {
            Fn8::F443(k0, k1) => f443(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F452,
}

impl Fn9 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn9::F452 => f452(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F454,
}

impl Fn10 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn10::F454 => f454(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F456(Bytes),
}

impl Fn11 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn11::F456(k0) => f456(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F489,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn12::F489 => f489(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F514,
}

impl Fn13 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn13::F514 => f514(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F519,
}

impl Fn14 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn14::F519 => f519(p0),
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
                if f46(v0, f8(Str::lit("read-yaml"))) {
                    f50(v3, v1)
                } else {
                    Ok(Adt12::C4(f31()))
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
    let v1: u64 = f51(v0)?;
    let m192 = {
        let a190 = f52({
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
    let v1: Bytes = f53(v0)?;
    let m222 = f54(v1.clone(), 0u64, {
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
        let (h229, _) = f55(Fn0::F56(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h229
    };
    let v4: u64 = {
        let (h230, _) = {
            let (_, h231) = v3.clone();
            h231
        };
        h230
    };
    Ok(f57(List::cons(f43(v0, v4, {
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
    let m244 = f58(Fn1::F59(v1), None::<Bytes>, v0);
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
    List::cons(f8(Str::lit("admit JSONL")), List::cons(f8(Str::lit("admit-encoding STAGED")), List::cons(f8(Str::lit("agenda")), List::cons(f8(Str::lit("build")), List::cons(f8(Str::lit("check")), List::cons(f8(Str::lit("ingest-authority BINDING")), List::cons(f8(Str::lit("ingest-index CSV")), List::cons(f8(Str::lit("merge-gate RULES")), List::cons(f8(Str::lit("next [SKIP...]")), List::cons(f8(Str::lit("propose-encoding SCHEMA NAME CITATION...")), List::cons(f8(Str::lit("publish")), List::cons(f8(Str::lit("read-webidl FILE [IMPORT...]")), List::cons(f8(Str::lit("read-xml FILE")), List::cons(f8(Str::lit("read-xsd FILE")), List::cons(f8(Str::lit("read-yaml FILE")), List::cons(f8(Str::lit("record-decision JSONL")), List::cons(f8(Str::lit("stage-admissions JSONL")), List::cons(f8(Str::lit("stage-bindings JSONL")), List::cons(f8(Str::lit("stage-decisions JSONL")), List::cons(f8(Str::lit("stage-encodings JSONL")), List::cons(f8(Str::lit("stage-engine")), List::cons(f8(Str::lit("stage-mappings JSONL")), List::cons(f8(Str::lit("stage-matrix JSONL")), List::<Bytes>::nil())))))))))))))))))))))))
}

pub fn f46(v0: Bytes, v1: Bytes) -> bool {
    let a245 = v0;
    let a246 = v1;
    equal(a245, a246)
}

pub fn f47(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<Adt12> {
    let m247 = f60(v0.clone(), List::cons(v1.clone(), v2.clone()));
    match m247.uncons() {
        None => {
            let m248 = f61(f62(v0.clone(), v1), f63(v0, v2))?;
            match m248 {
                Ok(v6) => {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("-- ")), List::cons(f64({
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
    let m251 = f60(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m251.uncons() {
        None => {
            let m252 = f65(f62(v0, v1))?;
            match m252 {
                Ok(v5) => {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("well-formed XML: root ")), List::cons(f66(v5.clone()), List::cons(f8(Str::lit(", ")), List::cons(f64(f67(v5)?)?, List::cons(f8(Str::lit(" elements\u{a}")), List::<Bytes>::nil()))))))))
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
    let m253 = f60(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m253.uncons() {
        None => {
            let m254 = f68(f62(v0, v1))?;
            match m254 {
                Ok(v5) => {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("-- ")), List::cons(f64({
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
    let m257 = f60(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m257.uncons() {
        None => {
            let m258 = f69(f62(v0, v1))?;
            match m258 {
                Ok(v5) => {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), {
                        let a259 = f70(v5)?;
                        let a260 = f8(Str::lit("\u{a}"));
                        append_bytes(a259, a260)
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

pub fn f51(v0: u8) -> R<u64> {
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

pub fn f52(v0: u64) -> List<Str> {
    let a307 = v0;
    let a308 = 0u64;
    if nat_eq(a307, a308) {
        f71()
    } else {
        let a309 = v0;
        let a310 = 1u64;
        if nat_eq(a309, a310) {
            f72()
        } else {
            let a311 = v0;
            let a312 = 2u64;
            if nat_eq(a311, a312) {
                f73()
            } else {
                let a313 = v0;
                let a314 = 3u64;
                if nat_eq(a313, a314) {
                    f74()
                } else {
                    let a315 = v0;
                    let a316 = 4u64;
                    if nat_eq(a315, a316) {
                        f75()
                    } else {
                        let a317 = v0;
                        let a318 = 5u64;
                        if nat_eq(a317, a318) {
                            f76()
                        } else {
                            let a319 = v0;
                            let a320 = 6u64;
                            if nat_eq(a319, a320) {
                                f77()
                            } else {
                                let a321 = v0;
                                let a322 = 7u64;
                                if nat_eq(a321, a322) {
                                    f78()
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

pub fn f53(v0: Bytes) -> R<Bytes> {
    f79(v0.clone(), 0u64, {
        let a323 = v0;
        length_bytes(a323)
    }, false, false, 0u64, Bytes::lit(&[]))
}

pub fn f54(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt2, u64)>> {
    let m324 = v2;
    if m324 == 0 {
        Ok(None::<(Adt2, u64)>)
    } else {
        let v4 = m324 - 1;
        let v5: u64 = f80(v0.clone(), v1)?;
        let a325 = v3;
        let a326 = 1u64;
        if nat_eq(a325, a326) {
            let m327 = f54(v0.clone(), v1, v4, 0u64)?;
            match m327 {
                None => {
                    Ok(None::<(Adt2, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f80(v0.clone(), {
                        let (_, h328) = v6.clone();
                        h328
                    })?;
                    let a329 = v7;
                    let a330 = 44u64;
                    if nat_eq(a329, a330) {
                        let m334 = f54(v0, {
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
                                Ok(Some((Adt2::C7(std::rc::Rc::new({
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
                            Ok(Some((Adt2::C7(std::rc::Rc::new({
                                let (h340, _) = v6;
                                h340
                            }), std::rc::Rc::new(Adt2::C6)), {
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
                let m348 = f81(v0.clone(), {
                    let a346 = v1;
                    let a347 = 1u64;
                    nat_add(a346, a347)?
                })?;
                match m348 {
                    None => {
                        Ok(None::<(Adt2, u64)>)
                    }
                    Some(v9) => {
                        if f82({
                            let a349 = v5;
                            let a350 = 34u64;
                            nat_eq(a349, a350)
                        }, {
                            let a352 = f80(v0.clone(), {
                                let (h351, _) = v9.clone();
                                h351
                            })?;
                            let a353 = 58u64;
                            nat_eq(a352, a353)
                        }) {
                            let m357 = f54(v0.clone(), {
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
                                    let v11: u64 = f80(v0.clone(), {
                                        let (_, h358) = v10.clone();
                                        h358
                                    })?;
                                    let a359 = v11;
                                    let a360 = 44u64;
                                    if nat_eq(a359, a360) {
                                        let m364 = f54(v0, {
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
                                                Ok(Some((Adt2::C8({
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
                                            Ok(Some((Adt2::C8({
                                                let (_, h371) = v9;
                                                h371
                                            }, std::rc::Rc::new({
                                                let (h372, _) = v10;
                                                h372
                                            }), std::rc::Rc::new(Adt2::C6)), {
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
                    let m380 = f81(v0, {
                        let a378 = v1;
                        let a379 = 1u64;
                        nat_add(a378, a379)?
                    })?;
                    match m380 {
                        None => {
                            Ok(None::<(Adt2, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt2::C3({
                                let (_, h381) = v13;
                                h381
                            }), {
                                let (h382, _) = v13;
                                h382
                            })))
                        }
                    }
                } else {
                    if f46(f43(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                        Ok(Some((Adt2::C0(true), {
                            let a383 = v1;
                            let a384 = 4u64;
                            nat_add(a383, a384)?
                        })))
                    } else {
                        if f46(f43(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                            Ok(Some((Adt2::C0(false), {
                                let a385 = v1;
                                let a386 = 5u64;
                                nat_add(a385, a386)?
                            })))
                        } else {
                            let a387 = v5;
                            let a388 = 91u64;
                            if nat_eq(a387, a388) {
                                let a391 = f80(v0.clone(), {
                                    let a389 = v1;
                                    let a390 = 1u64;
                                    nat_add(a389, a390)?
                                })?;
                                let a392 = 93u64;
                                if nat_eq(a391, a392) {
                                    Ok(Some((Adt2::C4(std::rc::Rc::new(Adt2::C6)), {
                                        let a393 = v1;
                                        let a394 = 2u64;
                                        nat_add(a393, a394)?
                                    })))
                                } else {
                                    let m397 = f54(v0, {
                                        let a395 = v1;
                                        let a396 = 1u64;
                                        nat_add(a395, a396)?
                                    }, v4, 1u64)?;
                                    match m397 {
                                        None => {
                                            Ok(None::<(Adt2, u64)>)
                                        }
                                        Some(v14) => {
                                            Ok(Some((Adt2::C4(std::rc::Rc::new({
                                                let (h398, _) = v14;
                                                h398
                                            })), {
                                                let (_, h399) = v14;
                                                h399
                                            })))
                                        }
                                    }
                                }
                            } else {
                                let a400 = v5;
                                let a401 = 123u64;
                                if nat_eq(a400, a401) {
                                    let a404 = f80(v0.clone(), {
                                        let a402 = v1;
                                        let a403 = 1u64;
                                        nat_add(a402, a403)?
                                    })?;
                                    let a405 = 125u64;
                                    if nat_eq(a404, a405) {
                                        Ok(Some((Adt2::C5(std::rc::Rc::new(Adt2::C6)), {
                                            let a406 = v1;
                                            let a407 = 2u64;
                                            nat_add(a406, a407)?
                                        })))
                                    } else {
                                        let m410 = f54(v0, {
                                            let a408 = v1;
                                            let a409 = 1u64;
                                            nat_add(a408, a409)?
                                        }, v4, 2u64)?;
                                        match m410 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v15) => {
                                                Ok(Some((Adt2::C5(std::rc::Rc::new({
                                                    let (h411, _) = v15;
                                                    h411
                                                })), {
                                                    let (_, h412) = v15;
                                                    h412
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let m413 = f83(v0, v1)?;
                                    match m413 {
                                        None => {
                                            Ok(None::<(Adt2, u64)>)
                                        }
                                        Some(v16) => {
                                            Ok(Some(({
                                                let (_, h414) = v16;
                                                h414
                                            }, {
                                                let (h415, _) = v16;
                                                h415
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

pub fn f55(v0: Fn0, p420: u64, p421: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m419 = (p420, p421);
    {
        let mut m419 = m419;
        loop {
            let step = {
                let (v1, v2) = m419;
                let m416 = v1;
                if m416 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m416 - 1;
                    let m418 = {
                        let c417: Fn0 = v0.clone();
                        c417.apply(v2.clone())?
                    };
                    match m418 {
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
                Ok(next) => { m419 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f56(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f84(v1, v0, v2)
}

pub fn f57(v0: List<Bytes>) -> List<Bytes> {
    f85(Fn2::F86, List::<Bytes>::nil(), v0)
}

pub fn f58(v0: Fn1, p425: Option<Bytes>, p426: List<Bytes>) -> Option<Bytes> {
    let m424 = (p425, p426);
    {
        let mut m424 = m424;
        loop {
            let step = {
                let (v1, v2) = m424;
                let m422 = v2;
                match m422.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c423: Fn1 = v0.clone();
                            c423.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m424 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f59(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m427 = v1;
    match m427 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a430 = {
                    let a428 = v3;
                    let a429 = v0;
                    append_bytes(a428, a429)
                };
                let a431 = v2;
                append_bytes(a430, a431)
            })
        }
    }
}

pub fn f60(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m432 = v1;
    match m432.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m433 = f87(f88(v0.clone(), v2.clone()), v2.clone());
            match m433 {
                None => {
                    List::cons(v2, f60(v0, v3))
                }
                Some(_) => {
                    f60(v0, v3)
                }
            }
        }
    }
}

pub fn f61(v0: Bytes, v1: List<Bytes>) -> R<Result<(u64, Bytes), Bytes>> {
    let m434 = f89(v0)?;
    match m434 {
        Ok(v3) => {
            let m435 = f90(v1)?;
            match m435 {
                Ok(v5) => {
                    let v6: List<Adt13> = f91(v3.clone(), v5);
                    let v7: List<Adt16> = f92({
                        let a437 = {
                            let a436 = v6.clone();
                            length_list(a436)
                        };
                        let a438 = 1u64;
                        nat_add(a437, a438)?
                    }, f93(v6.clone(), v6.clone())?)?;
                    let v8: List<Bytes> = f94({
                        let a440 = {
                            let a439 = v7.clone();
                            length_list(a439)
                        };
                        let a441 = 1u64;
                        nat_add(a440, a441)?
                    }, f95(v3.clone()), List::<Bytes>::nil(), v7.clone());
                    let v9: List<(Bytes, (List<Bytes>, Bytes))> = f96(v7.clone(), v8);
                    let v10: (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>) = f97({
                        let a443 = {
                            let a442 = v9.clone();
                            length_list(a442)
                        };
                        let a444 = 1u64;
                        nat_add(a443, a444)?
                    }, v9, List::<Bytes>::nil(), List::<Bytes>::nil());
                    Ok(Ok::<(u64, Bytes), Bytes>(({
                        let a446 = {
                            let (h445, _) = v10.clone();
                            h445
                        };
                        length_list(a446)
                    }, f7(List::cons(f7(f98(v3, v7, v6)), List::cons(f7(f99({
                        let (_, h447) = v10;
                        h447
                    })), List::cons(f8(Str::lit("-- operations, constants and the other members that are not attributes are behavior, not data, and are not translated (Web IDL 2.5)\u{a}")), List::cons(f100(f101({
                        let (h448, _) = v10;
                        h448
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

pub fn f62(v0: Adt1, v1: Bytes) -> Bytes {
    let m449 = f87(f88(v0, v1.clone()), v1);
    match m449 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m450 = v2;
            match m450 {
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

pub fn f63(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m451 = v1;
    match m451.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f62(v0.clone(), v2), f63(v0, v3))
        }
    }
}

pub fn f64(v0: u64) -> R<Bytes> {
    f102(f103(), 10u64, v0)
}

pub fn f65(v0: Bytes) -> R<Result<Adt17, Bytes>> {
    let v1: Bytes = if f104(v0.clone(), f8(Str::lit("\u{feff}")), 0u64) {
        f43(v0.clone(), 3u64, {
            let a453 = {
                let a452 = v0;
                length_bytes(a452)
            };
            let a454 = 3u64;
            nat_sub(a453, a454)
        })
    } else {
        v0
    };
    let v2: Bytes = f105(v1)?;
    let v3: u64 = {
        let a455 = v2.clone();
        length_bytes(a455)
    };
    let v4: u64 = f106(v2.clone())?;
    let a456 = v4;
    let a457 = v3;
    if nat_lt(a456, a457) {
        Ok(f107(f108(Str::lit("the input is not a sequence of Chars in UTF-8 (XML 1.0 [2], section 4.3.3)"), v4)?))
    } else {
        let m458 = f109(v2.clone())?;
        match m458 {
            Ok(v6) => {
                let m461 = f110(v2.clone(), v6, {
                    let a459 = v3;
                    let a460 = 2u64;
                    nat_add(a459, a460)?
                })?;
                match m461 {
                    Ok(v8) => {
                        if f111(v2.clone(), v8, Str::lit("<!DOCTYPE")) {
                            Ok(f107(f108(Str::lit("a document type declaration is not read; Verdant Studio reads documents that have none (XML 1.0 [28])"), v8)?))
                        } else {
                            let a462 = f112(v2.clone(), v8)?;
                            let a463 = 60u64;
                            if nat_eq(a462, a463) {
                                let m466 = f113(v2.clone(), v8, {
                                    let a464 = v3;
                                    let a465 = 2u64;
                                    nat_add(a464, a465)?
                                }, 0u64)?;
                                match m466 {
                                    Ok(v10) => {
                                        let m470 = f110(v2, {
                                            let (_, h467) = v10;
                                            h467
                                        }, {
                                            let a468 = v3;
                                            let a469 = 2u64;
                                            nat_add(a468, a469)?
                                        })?;
                                        match m470 {
                                            Ok(v12) => {
                                                let a471 = v12;
                                                let a472 = v3;
                                                if nat_eq(a471, a472) {
                                                    Ok(Ok::<Adt17, Bytes>({
                                                        let (h473, _) = v10;
                                                        h473
                                                    }))
                                                } else {
                                                    Ok(f107(f108(Str::lit("only comments, processing instructions and white space may follow the root element (XML 1.0 [1], [27])"), v12)?))
                                                }
                                            }
                                            Err(v11) => {
                                                Ok(f107(v11))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Ok(f107(v9))
                                    }
                                }
                            } else {
                                Ok(f107(f108(Str::lit("a document must have exactly one root element (XML 1.0 [1])"), v8)?))
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(f107(v7))
                    }
                }
            }
            Err(v5) => {
                Ok(f107(v5))
            }
        }
    }
}

pub fn f66(v0: Adt17) -> Bytes {
    let m474 = v0;
    match m474 {
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

pub fn f67(v0: Adt17) -> R<u64> {
    let m475 = v0;
    match m475 {
        Adt17::C0(_, _, r480) => {
            let v3 = (*r480).clone();
            let a476 = 1u64;
            let a477 = f67(v3)?;
            nat_add(a476, a477)
        }
        Adt17::C1(_, _, _) => {
            Ok(0u64)
        }
        Adt17::C2(_) => {
            Ok(0u64)
        }
        Adt17::C3(r481, r482) => {
            let v8 = (*r481).clone();
            let v9 = (*r482).clone();
            let a478 = f67(v8)?;
            let a479 = f67(v9)?;
            nat_add(a478, a479)
        }
        Adt17::C4 => {
            Ok(0u64)
        }
    }
}

pub fn f68(v0: Bytes) -> R<Result<(u64, Bytes), Bytes>> {
    let m483 = f65(v0)?;
    match m483 {
        Ok(v2) => {
            let m484 = v2;
            match m484 {
                Adt17::C0(v3, r496, r497) => {
                    let v4 = (*r496).clone();
                    let v5 = (*r497).clone();
                    let m485 = f115(v4.clone(), List::<(Bytes, Bytes)>::nil());
                    match m485 {
                        Ok(v7) => {
                            let m486 = f116(v7.clone(), v3)?;
                            match m486 {
                                Ok(v9) => {
                                    if f117(v9, Str::lit("schema")) {
                                        let v11: Bytes = {
                                            let m487 = f118(v4, f8(Str::lit("targetNamespace")));
                                            match m487 {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some(v10) => {
                                                    v10
                                                }
                                            }
                                        };
                                        let m488 = f119(v5, v7, v11)?;
                                        match m488 {
                                            Ok(v13) => {
                                                let v14: List<Bytes> = f120(v13.clone());
                                                let m489 = f121(v14.clone());
                                                match m489 {
                                                    None => {
                                                        let m490 = f122(v13.clone(), v14.clone());
                                                        match m490 {
                                                            None => {
                                                                let m494 = f123({
                                                                    let a492 = {
                                                                        let a491 = v14.clone();
                                                                        length_list(a491)
                                                                    };
                                                                    let a493 = 1u64;
                                                                    nat_add(a492, a493)?
                                                                }, v13, List::<Bytes>::nil(), List::<Bytes>::nil());
                                                                match m494 {
                                                                    Ok(v18) => {
                                                                        Ok(Ok::<(u64, Bytes), Bytes>(({
                                                                            let a495 = v14;
                                                                            length_list(a495)
                                                                        }, f100(f101(v18, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))))))
                                                                    }
                                                                    Err(v17) => {
                                                                        Ok(f114(v17))
                                                                    }
                                                                }
                                                            }
                                                            Some(v16) => {
                                                                Ok(f114(f7(List::cons(f8(Str::lit("the type ")), List::cons(v16, List::cons(f8(Str::lit(" is not a simple type of this schema document")), List::<Bytes>::nil()))))))
                                                            }
                                                        }
                                                    }
                                                    Some(v15) => {
                                                        Ok(f114(f7(List::cons(f8(Str::lit("two simple types derive the type name ")), List::cons(v15, List::<Bytes>::nil())))))
                                                    }
                                                }
                                            }
                                            Err(v12) => {
                                                Ok(f114(v12))
                                            }
                                        }
                                    } else {
                                        Ok(f114(f8(Str::lit("the root element must be xs:schema in the XML Schema namespace (XSD 1.1 Part 1, section 3.17.2)"))))
                                    }
                                }
                                Err(v8) => {
                                    Ok(f114(v8))
                                }
                            }
                        }
                        Err(v6) => {
                            Ok(f114(v6))
                        }
                    }
                }
                Adt17::C1(_, _, _) => {
                    Ok(f114(f8(Str::lit("the document has no root element"))))
                }
                Adt17::C2(_) => {
                    Ok(f114(f8(Str::lit("the document has no root element"))))
                }
                Adt17::C3(_, _) => {
                    Ok(f114(f8(Str::lit("the document has no root element"))))
                }
                Adt17::C4 => {
                    Ok(f114(f8(Str::lit("the document has no root element"))))
                }
            }
        }
        Err(v1) => {
            Ok(f114(f7(List::cons(f8(Str::lit("not well-formed XML: ")), List::cons(v1, List::<Bytes>::nil())))))
        }
    }
}

pub fn f69(v0: Bytes) -> R<Result<Adt2, Bytes>> {
    let v1: Bytes = if f104(v0.clone(), Bytes::lit(&[0xef, 0xbb, 0xbf]), 0u64) {
        f43(v0.clone(), 3u64, {
            let a499 = {
                let a498 = v0;
                length_bytes(a498)
            };
            let a500 = 3u64;
            nat_sub(a499, a500)
        })
    } else {
        v0
    };
    let v2: Bytes = f105(v1)?;
    let v3: u64 = {
        let a501 = v2.clone();
        length_bytes(a501)
    };
    let v4: u64 = f124(v2.clone())?;
    let v5: u64 = {
        let a502 = v3;
        let a503 = 2u64;
        nat_add(a502, a503)?
    };
    let v6: u64 = f125(v2.clone(), 0u64)?;
    let a504 = v4;
    let a505 = v3;
    if nat_lt(a504, a505) {
        Ok(Err::<Adt2, Bytes>(f108(Str::lit("the input is not a sequence of printable characters in UTF-8 (YAML 1.2.2 [1], section 5.2)"), v4)?))
    } else {
        let a506 = v6;
        let a507 = v3;
        if if if nat_lt(a506, a507) {
            f111(v2.clone(), v6, Str::lit("..."))
        } else {
            false
        } {
            f126(v2.clone(), v6)?
        } else {
            false
        } {
            Ok(Err::<Adt2, Bytes>(f108(Str::lit("a stream with no document is not read (YAML 1.2.2 [211])"), v6)?))
        } else {
            let a508 = f127(v2.clone(), v6)?;
            let a509 = 37u64;
            if nat_eq(a508, a509) {
                Ok(Err::<Adt2, Bytes>(f108(Str::lit("a directive is not read (YAML 1.2.2 [82])"), v6)?))
            } else {
                let a510 = v3;
                let a511 = v6;
                if nat_le(a510, a511) {
                    Ok(Err::<Adt2, Bytes>(f108(Str::lit("a stream with no document is not read (YAML 1.2.2 [211])"), v6)?))
                } else {
                    let m514 = if if f111(v2.clone(), v6, Str::lit("---")) {
                        f126(v2.clone(), v6)?
                    } else {
                        false
                    } {
                        f128(v2.clone(), {
                            let a512 = v6;
                            let a513 = 3u64;
                            nat_add(a512, a513)?
                        }, 0u64, v5, 1u64, false, 0u64, List::<Bytes>::nil())?
                    } else {
                        f128(v2.clone(), v6, 0u64, v5, 0u64, false, 0u64, List::<Bytes>::nil())?
                    };
                    match m514 {
                        Adt18::C0(v8, v9) => {
                            let m515 = f129(v2, v9)?;
                            match m515 {
                                Ok(_) => {
                                    Ok(Ok::<Adt2, Bytes>(v8))
                                }
                                Err(v10) => {
                                    Ok(Err::<Adt2, Bytes>(v10))
                                }
                            }
                        }
                        Adt18::C1(v7) => {
                            Ok(Err::<Adt2, Bytes>(v7))
                        }
                    }
                }
            }
        }
    }
}

pub fn f70(v0: Adt2) -> R<Bytes> {
    f130(v0)
}

pub fn f71() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{9}"), List::cons(Str::lit("\u{a}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{d}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f72() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f73() -> List<Str> {
    List::cons(Str::lit(" "), List::cons(Str::lit("!"), List::cons(Str::lit("\""), List::cons(Str::lit("#"), List::cons(Str::lit("$"), List::cons(Str::lit("%"), List::cons(Str::lit("&"), List::cons(Str::lit("'"), List::cons(Str::lit("("), List::cons(Str::lit(")"), List::cons(Str::lit("*"), List::cons(Str::lit("+"), List::cons(Str::lit(","), List::cons(Str::lit("-"), List::cons(Str::lit("."), List::cons(Str::lit("/"), List::<Str>::nil()))))))))))))))))
}

pub fn f74() -> List<Str> {
    List::cons(Str::lit("0"), List::cons(Str::lit("1"), List::cons(Str::lit("2"), List::cons(Str::lit("3"), List::cons(Str::lit("4"), List::cons(Str::lit("5"), List::cons(Str::lit("6"), List::cons(Str::lit("7"), List::cons(Str::lit("8"), List::cons(Str::lit("9"), List::cons(Str::lit(":"), List::cons(Str::lit(";"), List::cons(Str::lit("<"), List::cons(Str::lit("="), List::cons(Str::lit(">"), List::cons(Str::lit("?"), List::<Str>::nil()))))))))))))))))
}

pub fn f75() -> List<Str> {
    List::cons(Str::lit("@"), List::cons(Str::lit("A"), List::cons(Str::lit("B"), List::cons(Str::lit("C"), List::cons(Str::lit("D"), List::cons(Str::lit("E"), List::cons(Str::lit("F"), List::cons(Str::lit("G"), List::cons(Str::lit("H"), List::cons(Str::lit("I"), List::cons(Str::lit("J"), List::cons(Str::lit("K"), List::cons(Str::lit("L"), List::cons(Str::lit("M"), List::cons(Str::lit("N"), List::cons(Str::lit("O"), List::<Str>::nil()))))))))))))))))
}

pub fn f76() -> List<Str> {
    List::cons(Str::lit("P"), List::cons(Str::lit("Q"), List::cons(Str::lit("R"), List::cons(Str::lit("S"), List::cons(Str::lit("T"), List::cons(Str::lit("U"), List::cons(Str::lit("V"), List::cons(Str::lit("W"), List::cons(Str::lit("X"), List::cons(Str::lit("Y"), List::cons(Str::lit("Z"), List::cons(Str::lit("["), List::cons(Str::lit("\\"), List::cons(Str::lit("]"), List::cons(Str::lit("^"), List::cons(Str::lit("_"), List::<Str>::nil()))))))))))))))))
}

pub fn f77() -> List<Str> {
    List::cons(Str::lit("`"), List::cons(Str::lit("a"), List::cons(Str::lit("b"), List::cons(Str::lit("c"), List::cons(Str::lit("d"), List::cons(Str::lit("e"), List::cons(Str::lit("f"), List::cons(Str::lit("g"), List::cons(Str::lit("h"), List::cons(Str::lit("i"), List::cons(Str::lit("j"), List::cons(Str::lit("k"), List::cons(Str::lit("l"), List::cons(Str::lit("m"), List::cons(Str::lit("n"), List::cons(Str::lit("o"), List::<Str>::nil()))))))))))))))))
}

pub fn f78() -> List<Str> {
    List::cons(Str::lit("p"), List::cons(Str::lit("q"), List::cons(Str::lit("r"), List::cons(Str::lit("s"), List::cons(Str::lit("t"), List::cons(Str::lit("u"), List::cons(Str::lit("v"), List::cons(Str::lit("w"), List::cons(Str::lit("x"), List::cons(Str::lit("y"), List::cons(Str::lit("z"), List::cons(Str::lit("{"), List::cons(Str::lit("|"), List::cons(Str::lit("}"), List::cons(Str::lit("~"), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f79(v0: Bytes, p551: u64, p552: u64, p553: bool, p554: bool, p555: u64, p556: Bytes) -> R<Bytes> {
    let m550 = (p551, (p552, (p553, (p554, (p555, p556)))));
    {
        let mut m550 = m550;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, (v5, v6))))) = m550;
                let m516 = v2;
                if m516 == 0 {
                    Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f131(v0.clone(), v5, v1, v6)))
                } else {
                    let v7 = m516 - 1;
                    let m519 = {
                        let a517 = v0.clone();
                        let a518 = v1;
                        index_bytes(a517, a518)
                    };
                    match m519 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f131(v0.clone(), v5, v1, v6)))
                        }
                        Some(v8) => {
                            let v9: u64 = f51(v8)?;
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                    let a520 = v1;
                                    let a521 = 1u64;
                                    nat_add(a520, a521)?
                                }, (v7, (if v4 {
                                    true
                                } else {
                                    let a522 = v9;
                                    let a523 = 34u64;
                                    !nat_eq(a522, a523)
                                }, (if v4 {
                                    false
                                } else {
                                    let a524 = v9;
                                    let a525 = 92u64;
                                    nat_eq(a524, a525)
                                }, (v5, v6))))))
                            } else {
                                let a526 = v9;
                                let a527 = 32u64;
                                if if if if nat_eq(a526, a527) {
                                    true
                                } else {
                                    let a528 = v9;
                                    let a529 = 9u64;
                                    nat_eq(a528, a529)
                                } {
                                    true
                                } else {
                                    let a530 = v9;
                                    let a531 = 10u64;
                                    nat_eq(a530, a531)
                                } {
                                    true
                                } else {
                                    let a532 = v9;
                                    let a533 = 13u64;
                                    nat_eq(a532, a533)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a534 = v1;
                                        let a535 = 1u64;
                                        nat_add(a534, a535)?
                                    }, (v7, (false, (false, ({
                                        let a536 = v1;
                                        let a537 = 1u64;
                                        nat_add(a536, a537)?
                                    }, f131(v0.clone(), v5, v1, v6)))))))
                                } else {
                                    let a538 = v9;
                                    let a539 = 110u64;
                                    if if nat_eq(a538, a539) {
                                        f46(f43(v0.clone(), v1, 4u64), f8(Str::lit("null")))
                                    } else {
                                        false
                                    } {
                                        Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                            let a540 = v1;
                                            let a541 = 4u64;
                                            nat_add(a540, a541)?
                                        }, (v7, (false, (false, ({
                                            let a542 = v1;
                                            let a543 = 4u64;
                                            nat_add(a542, a543)?
                                        }, {
                                            let a544 = f131(v0.clone(), v5, v1, v6);
                                            let a545 = f8(Str::lit("[]"));
                                            append_bytes(a544, a545)
                                        }))))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                            let a546 = v1;
                                            let a547 = 1u64;
                                            nat_add(a546, a547)?
                                        }, (v7, ({
                                            let a548 = v9;
                                            let a549 = 34u64;
                                            nat_eq(a548, a549)
                                        }, (false, (v5, v6))))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m550 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f80(v0: Bytes, v1: u64) -> R<u64> {
    let m559 = {
        let a557 = v0;
        let a558 = v1;
        index_bytes(a557, a558)
    };
    match m559 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a604 = {
                let a580 = {
                    let a568 = {
                        let a562 = {
                            let a560 = v3;
                            let a561 = 128u8;
                            fixed_u8::bit_and(a560, a561)
                        };
                        let a563 = 128u8;
                        if equal(a562, a563) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a569 = {
                        let a566 = {
                            let a564 = v3;
                            let a565 = 64u8;
                            fixed_u8::bit_and(a564, a565)
                        };
                        let a567 = 64u8;
                        if equal(a566, a567) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a568, a569)?
                };
                let a581 = {
                    let a578 = {
                        let a572 = {
                            let a570 = v3;
                            let a571 = 32u8;
                            fixed_u8::bit_and(a570, a571)
                        };
                        let a573 = 32u8;
                        if equal(a572, a573) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a579 = {
                        let a576 = {
                            let a574 = v3;
                            let a575 = 16u8;
                            fixed_u8::bit_and(a574, a575)
                        };
                        let a577 = 16u8;
                        if equal(a576, a577) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a578, a579)?
                };
                nat_add(a580, a581)?
            };
            let a605 = {
                let a602 = {
                    let a590 = {
                        let a584 = {
                            let a582 = v3;
                            let a583 = 8u8;
                            fixed_u8::bit_and(a582, a583)
                        };
                        let a585 = 8u8;
                        if equal(a584, a585) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a591 = {
                        let a588 = {
                            let a586 = v3;
                            let a587 = 4u8;
                            fixed_u8::bit_and(a586, a587)
                        };
                        let a589 = 4u8;
                        if equal(a588, a589) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a590, a591)?
                };
                let a603 = {
                    let a600 = {
                        let a594 = {
                            let a592 = v3;
                            let a593 = 2u8;
                            fixed_u8::bit_and(a592, a593)
                        };
                        let a595 = 2u8;
                        if equal(a594, a595) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a601 = {
                        let a598 = {
                            let a596 = v3;
                            let a597 = 1u8;
                            fixed_u8::bit_and(a596, a597)
                        };
                        let a599 = 1u8;
                        if equal(a598, a599) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a600, a601)?
                };
                nat_add(a602, a603)?
            };
            nat_add(a604, a605)
        }
    }
}

pub fn f81(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = f132(v0.clone(), v1, {
        let a607 = {
            let a606 = v0.clone();
            length_bytes(a606)
        };
        let a608 = 1u64;
        nat_add(a607, a608)?
    })?;
    let a609 = f80(v0.clone(), v2)?;
    let a610 = 34u64;
    if nat_eq(a609, a610) {
        Ok(Some(({
            let a611 = v2;
            let a612 = 1u64;
            nat_add(a611, a612)?
        }, f43(v0, v1, {
            let a613 = v2;
            let a614 = v1;
            nat_sub(a613, a614)
        }))))
    } else {
        f133(v0.clone(), v1, {
            let a616 = {
                let a615 = v0;
                length_bytes(a615)
            };
            let a617 = 1u64;
            nat_add(a616, a617)?
        }, Bytes::lit(&[]))
    }
}

pub fn f82(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f83(v0: Bytes, v1: u64) -> R<Option<(u64, Adt2)>> {
    let v2: bool = {
        let a618 = f80(v0.clone(), v1)?;
        let a619 = 45u64;
        nat_eq(a618, a619)
    };
    let v3: u64 = if v2 {
        let a620 = v1;
        let a621 = 1u64;
        nat_add(a620, a621)?
    } else {
        v1
    };
    let v4: u64 = f134(v0.clone(), v3)?;
    let v5: u64 = {
        let a622 = v4;
        let a623 = v3;
        nat_sub(a622, a623)
    };
    let v6: bool = {
        let a624 = f80(v0.clone(), v4)?;
        let a625 = 46u64;
        nat_eq(a624, a625)
    };
    let v7: u64 = if v6 {
        let a626 = v4;
        let a627 = 1u64;
        nat_add(a626, a627)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f134(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f80(v0.clone(), v8)?;
    let v10: bool = {
        let a628 = v9;
        let a629 = 101u64;
        if nat_eq(a628, a629) {
            true
        } else {
            let a630 = v9;
            let a631 = 69u64;
            nat_eq(a630, a631)
        }
    };
    let v11: u64 = f80(v0.clone(), {
        let a632 = v8;
        let a633 = 1u64;
        nat_add(a632, a633)?
    })?;
    let v12: bool = f82(v10, f135({
        let a634 = v11;
        let a635 = 43u64;
        nat_eq(a634, a635)
    }, {
        let a636 = v11;
        let a637 = 45u64;
        nat_eq(a636, a637)
    }));
    let v13: u64 = if v10 {
        if v12 {
            let a638 = v8;
            let a639 = 2u64;
            nat_add(a638, a639)?
        } else {
            let a640 = v8;
            let a641 = 1u64;
            nat_add(a640, a641)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f134(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f43(v0.clone(), v13, {
        let a642 = v14;
        let a643 = v13;
        nat_sub(a642, a643)
    });
    let v16: u64 = f136(v15.clone())?;
    let v17: Bytes = f43(v15.clone(), v16, {
        let a645 = {
            let a644 = v15;
            length_bytes(a644)
        };
        let a646 = v16;
        nat_sub(a645, a646)
    });
    let a647 = v5;
    let a648 = 0u64;
    if if nat_eq(a647, a648) {
        true
    } else {
        if f82({
            let a649 = 1u64;
            let a650 = v5;
            nat_lt(a649, a650)
        }, {
            let a651 = f80(v0.clone(), v3)?;
            let a652 = 48u64;
            nat_eq(a651, a652)
        }) {
            true
        } else {
            if f82(v6, {
                let a653 = v8;
                let a654 = v7;
                nat_eq(a653, a654)
            }) {
                true
            } else {
                f135(if v10 {
                    let a655 = v14;
                    let a656 = v13;
                    nat_eq(a655, a656)
                } else {
                    false
                }, {
                    let a658 = 18u64;
                    let a659 = {
                        let a657 = v17.clone();
                        length_bytes(a657)
                    };
                    nat_lt(a658, a659)
                })
            }
        }
    } {
        Ok(None::<(u64, Adt2)>)
    } else {
        let m663 = {
            let a661 = {
                let a660 = v17.clone();
                length_bytes(a660)
            };
            let a662 = 0u64;
            if nat_eq(a661, a662) {
                Some(0u64)
            } else {
                f137(v17)?
            }
        };
        match m663 {
            None => {
                Ok(None::<(u64, Adt2)>)
            }
            Some(v18) => {
                let m668 = f138(f43(v0.clone(), v3, v5), f43(v0, v7, {
                    let a664 = v8;
                    let a665 = v7;
                    nat_sub(a664, a665)
                }), f82(v12, {
                    let a666 = v11;
                    let a667 = 45u64;
                    nat_eq(a666, a667)
                }), v18)?;
                match m668 {
                    None => {
                        Ok(None::<(u64, Adt2)>)
                    }
                    Some(v19) => {
                        let a671 = 19u64;
                        let a672 = {
                            let a670 = {
                                let (h669, _) = v19.clone();
                                h669
                            };
                            length_bytes(a670)
                        };
                        if nat_lt(a671, a672) {
                            Ok(None::<(u64, Adt2)>)
                        } else {
                            let m674 = f137({
                                let (h673, _) = v19.clone();
                                h673
                            })?;
                            match m674 {
                                None => {
                                    Ok(None::<(u64, Adt2)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f82(v2, {
                                        let a675 = 0u64;
                                        let a676 = v20;
                                        nat_lt(a675, a676)
                                    });
                                    if f139(v21, v20) {
                                        Ok(Some((v14, {
                                            let a678 = {
                                                let (_, h677) = v19.clone();
                                                h677
                                            };
                                            let a679 = 0u64;
                                            if nat_eq(a678, a679) {
                                                Adt2::C1(v21, v20)
                                            } else {
                                                Adt2::C2(v21, v20, {
                                                    let (_, h680) = v19;
                                                    h680
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

pub fn f84(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h681, _) = v2.clone();
        h681
    };
    let v4: u64 = {
        let (h682, _) = {
            let (_, h683) = v2.clone();
            h683
        };
        h682
    };
    let v5: List<Bytes> = {
        let (_, h684) = {
            let (_, h685) = v2;
            h685
        };
        h684
    };
    let m686 = f140(v0.clone(), v3);
    match m686 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a687 = v6;
            let a688 = v1;
            if equal(a687, a688) {
                Ok(Some(({
                    let a689 = v3;
                    let a690 = 1u64;
                    nat_add(a689, a690)?
                }, ({
                    let a691 = v3;
                    let a692 = 1u64;
                    nat_add(a691, a692)?
                }, List::cons(f43(v0, v4, {
                    let a693 = v3;
                    let a694 = v4;
                    nat_sub(a693, a694)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a695 = v3;
                    let a696 = 1u64;
                    nat_add(a695, a696)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f85(v0: Fn2, p700: List<Bytes>, p701: List<Bytes>) -> List<Bytes> {
    let m699 = (p700, p701);
    {
        let mut m699 = m699;
        loop {
            let step = {
                let (v1, v2) = m699;
                let m697 = v2;
                match m697.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c698: Fn2 = v0.clone();
                            c698.apply(v1, v3)
                        }, v4))
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

pub fn f86(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f87(p706: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m705 = p706;
    {
        let mut m705 = m705;
        loop {
            let step = {
                let v0 = m705;
                let m702 = v0;
                match m702.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f46({
                            let (h703, _) = v2;
                            h703
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h704) = v2;
                                h704
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m705 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f88(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Bytes>)> {
    let m710 = {
        let a708 = match v0 {
            Adt1::C0(h707, _, _, _) => {
                h707
            }
        };
        let a709 = f37(v1);
        index_list(a708, a709)
    };
    match m710 {
        None => {
            List::<(Bytes, Option<Bytes>)>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f89(v0: Bytes) -> R<Result<List<Adt13>, Bytes>> {
    let m711 = f141(v0)?;
    match m711 {
        Ok(v2) => {
            f142({
                let a713 = {
                    let a712 = v2.clone();
                    length_list(a712)
                };
                let a714 = 1u64;
                nat_add(a713, a714)?
            }, v2, List::<Adt13>::nil())
        }
        Err(v1) => {
            Ok(Err::<List<Adt13>, Bytes>(v1))
        }
    }
}

pub fn f90(v0: List<Bytes>) -> R<Result<List<Adt13>, Bytes>> {
    let m715 = v0;
    match m715.uncons() {
        None => {
            Ok(Ok::<List<Adt13>, Bytes>(List::<Adt13>::nil()))
        }
        Some((v1, v2)) => {
            let m716 = f89(v1)?;
            match m716 {
                Ok(v4) => {
                    let m717 = f90(v2)?;
                    match m717 {
                        Ok(v6) => {
                            Ok(Ok::<List<Adt13>, Bytes>(f91(v4, v6)))
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

pub fn f91(v0: List<Adt13>, v1: List<Adt13>) -> List<Adt13> {
    let m718 = v0;
    match m718.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f91(v3, v1))
        }
    }
}

pub fn f92(p723: u64, p724: List<Adt16>) -> R<List<Adt16>> {
    let m722 = (p723, p724);
    {
        let mut m722 = m722;
        loop {
            let step = {
                let (v0, v1) = m722;
                let m719 = v0;
                if m719 == 0 {
                    Err::<(u64, List<Adt16>), R<List<Adt16>>>(Ok(v1))
                } else {
                    let v2 = m719 - 1;
                    let v3: List<Adt16> = f143(v1.clone(), v1.clone());
                    let a720 = f144(v3.clone())?;
                    let a721 = f144(v1.clone())?;
                    if nat_eq(a720, a721) {
                        Err::<(u64, List<Adt16>), R<List<Adt16>>>(Ok(v1))
                    } else {
                        Ok::<(u64, List<Adt16>), R<List<Adt16>>>((v2, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m722 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f93(v0: List<Adt13>, v1: List<Adt13>) -> R<List<Adt16>> {
    let m725 = v0;
    match m725.uncons() {
        None => {
            Ok(List::<Adt16>::nil())
        }
        Some((v2, v3)) => {
            if f145(f146(v2.clone())) {
                let v6: Adt16 = {
                    let m726 = f147(v1.clone(), v2.clone())?;
                    match m726 {
                        Ok(v5) => {
                            Adt16::C0(f148(v2), Bytes::lit(&[]), {
                                let (h727, _) = v5;
                                h727
                            }, {
                                let (_, h728) = v5;
                                h728
                            })
                        }
                        Err(v4) => {
                            Adt16::C0(f148(v2), v4, Bytes::lit(&[]), List::<Bytes>::nil())
                        }
                    }
                };
                Ok(List::cons(v6, f93(v3, v1)?))
            } else {
                f93(v3, v1)
            }
        }
    }
}

pub fn f94(p733: u64, p734: List<Bytes>, p735: List<Bytes>, v3: List<Adt16>) -> List<Bytes> {
    let m732 = (p733, (p734, p735));
    {
        let mut m732 = m732;
        loop {
            let step = {
                let (v0, (v1, v2)) = m732;
                let m729 = v0;
                if m729 == 0 {
                    Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v2)
                } else {
                    let v4 = m729 - 1;
                    let m730 = v1;
                    match m730.uncons() {
                        None => {
                            Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v2)
                        }
                        Some((v5, v6)) => {
                            if f149(v5.clone(), v2.clone()) {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v2)))
                            } else {
                                let m731 = f150(v3.clone(), v5.clone());
                                match m731 {
                                    None => {
                                        Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v2)))
                                    }
                                    Some(v7) => {
                                        if f46(f151(v7.clone()), Bytes::lit(&[])) {
                                            Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (f152(f153(v7), v6), List::cons(v5, v2))))
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
                Ok(next) => { m732 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f95(v0: List<Adt13>) -> List<Bytes> {
    let m736 = v0;
    match m736.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let v3: u64 = f146(v1.clone());
            let a737 = v3;
            let a738 = 0u64;
            if if nat_eq(a737, a738) {
                true
            } else {
                let a739 = v3;
                let a740 = 2u64;
                if nat_eq(a739, a740) {
                    true
                } else {
                    let a741 = v3;
                    let a742 = 3u64;
                    if nat_eq(a741, a742) {
                        true
                    } else {
                        let a743 = v3;
                        let a744 = 4u64;
                        if nat_eq(a743, a744) {
                            true
                        } else {
                            let a745 = v3;
                            let a746 = 5u64;
                            if nat_eq(a745, a746) {
                                true
                            } else {
                                let a747 = v3;
                                let a748 = 7u64;
                                if nat_eq(a747, a748) {
                                    true
                                } else {
                                    let a749 = v3;
                                    let a750 = 8u64;
                                    nat_eq(a749, a750)
                                }
                            }
                        }
                    }
                }
            } {
                List::cons(f148(v1), f95(v2))
            } else {
                f95(v2)
            }
        }
    }
}

pub fn f96(v0: List<Adt16>, v1: List<Bytes>) -> List<(Bytes, (List<Bytes>, Bytes))> {
    let m751 = v0;
    match m751.uncons() {
        None => {
            List::<(Bytes, (List<Bytes>, Bytes))>::nil()
        }
        Some((v2, v3)) => {
            if if f149(f154(v2.clone()), v1.clone()) {
                f46(f151(v2.clone()), Bytes::lit(&[]))
            } else {
                false
            } {
                List::cons((f154(v2.clone()), (f153(v2.clone()), f155(v2))), f96(v3, v1))
            } else {
                f96(v3, v1)
            }
        }
    }
}

pub fn f97(p760: u64, p761: List<(Bytes, (List<Bytes>, Bytes))>, p762: List<Bytes>, p763: List<Bytes>) -> (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>) {
    let m759 = (p760, (p761, (p762, p763)));
    {
        let mut m759 = m759;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m759;
                let m752 = v0;
                if m752 == 0 {
                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v3, v1))
                } else {
                    let v4 = m752 - 1;
                    let m753 = f156(v1.clone(), v2.clone());
                    match m753 {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v3, v1))
                        }
                        Some(v5) => {
                            Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v4, ({
                                let (_, h754) = v5.clone();
                                h754
                            }, (List::cons({
                                let (h755, _) = {
                                    let (h756, _) = v5.clone();
                                    h756
                                };
                                h755
                            }, v2), List::cons({
                                let (_, h757) = {
                                    let (h758, _) = v5;
                                    h758
                                };
                                h757
                            }, v3)))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m759 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f98(v0: List<Adt13>, v1: List<Adt16>, v2: List<Adt13>) -> List<Bytes> {
    let m764 = v0;
    match m764.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            let v5: u64 = f146(v3.clone());
            let a765 = v5;
            let a766 = 5u64;
            if if nat_eq(a765, a766) {
                true
            } else {
                let a767 = v5;
                let a768 = 7u64;
                nat_eq(a767, a768)
            } {
                let m771 = f157(v2.clone(), {
                    let a769 = v5;
                    let a770 = 5u64;
                    if nat_eq(a769, a770) {
                        0u64
                    } else {
                        2u64
                    }
                }, f148(v3.clone()));
                match m771 {
                    None => {
                        List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f158(v5), List::cons(f148(v3.clone()), List::cons(f8(Str::lit(": no input defines ")), List::cons(f148(v3), List::cons(f8(Str::lit(", so its members join no definition (Web IDL 2.2 and 2.7)\u{a}")), List::<Bytes>::nil()))))))), f98(v4, v1, v2))
                    }
                    Some(_) => {
                        f98(v4, v1, v2)
                    }
                }
            } else {
                let a772 = v5;
                let a773 = 8u64;
                if nat_eq(a772, a773) {
                    let m774 = f157(v2.clone(), 0u64, f148(v3.clone()));
                    match m774 {
                        None => {
                            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f148(v3.clone()), List::cons(f8(Str::lit(" includes ")), List::cons(f159(v3.clone()), List::cons(f8(Str::lit(": no input defines the interface ")), List::cons(f148(v3), List::cons(f8(Str::lit(" (Web IDL 2.3)\u{a}")), List::<Bytes>::nil())))))))), f98(v4, v1, v2))
                        }
                        Some(_) => {
                            f98(v4, v1, v2)
                        }
                    }
                } else {
                    if f145(v5) {
                        let m775 = f150(v1.clone(), f148(v3.clone()));
                        match m775 {
                            None => {
                                f98(v4, v1, v2)
                            }
                            Some(v8) => {
                                if f46(f151(v8.clone()), Bytes::lit(&[])) {
                                    f98(v4, v1, v2)
                                } else {
                                    List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f148(v3), List::cons(f8(Str::lit(": ")), List::cons(f151(v8), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), f98(v4, v1, v2))
                                }
                            }
                        }
                    } else {
                        f98(v4, v1, v2)
                    }
                }
            }
        }
    }
}

pub fn f99(v0: List<(Bytes, (List<Bytes>, Bytes))>) -> List<Bytes> {
    let m776 = v0;
    match m776.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons({
                let (h777, _) = v1;
                h777
            }, List::cons(f8(Str::lit(": it refers back to itself through its members, and the translator emits non-recursive declarations only\u{a}")), List::<Bytes>::nil())))), f99(v2))
        }
    }
}

pub fn f100(v0: List<Bytes>, v1: Bytes) -> Bytes {
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
                        let a781 = f100(v3, v1);
                        append_bytes(a780, a781)
                    };
                    append_bytes(a782, a783)
                }
            }
        }
    }
}

pub fn f101(p786: List<Bytes>, p787: List<Bytes>) -> List<Bytes> {
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

pub fn f102(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a788 = v2;
    let a789 = 0u64;
    if nat_eq(a788, a789) {
        Ok(f160(v0, 0u64))
    } else {
        let (_, h790) = {
            let (h791, _) = f161(Fn3::F162(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h791
        };
        Ok(h790)
    }
}

pub fn f103() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f104(v0: Bytes, v1: Bytes, v2: u64) -> bool {
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

pub fn f105(v0: Bytes) -> R<Bytes> {
    let (_, h799) = {
        let (h800, _) = f161(Fn3::F163(v0.clone()), {
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

pub fn f106(v0: Bytes) -> R<u64> {
    let (h804, _) = f164(Fn4::F165(v0.clone()), {
        let a806 = {
            let a805 = v0;
            length_bytes(a805)
        };
        let a807 = 1u64;
        nat_add(a806, a807)?
    }, 0u64)?;
    Ok(h804)
}

pub fn f107(v0: Bytes) -> Result<Adt17, Bytes> {
    Err::<Adt17, Bytes>(v0)
}

pub fn f108(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a808 = v0;
        utf8_encode(a808)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f64(v1)?, List::<Bytes>::nil())))))
}

pub fn f109(v0: Bytes) -> R<Result<u64, Bytes>> {
    if if f111(v0.clone(), 0u64, Str::lit("<?xml")) {
        f166(f112(v0.clone(), 5u64)?)?
    } else {
        false
    } {
        let m810 = f167(v0.clone(), {
            let a809 = Str::lit("?>");
            utf8_encode(a809)
        }, 5u64)?;
        match m810 {
            None => {
                Ok(f168(f108(Str::lit("the XML declaration is not closed (XML 1.0 [23])"), 0u64)?))
            }
            Some(v1) => {
                let m812 = f167(f43(v0.clone(), 0u64, v1), {
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
                        let v3: u64 = f169(v0.clone(), {
                            let a815 = v2;
                            let a816 = 8u64;
                            nat_add(a815, a816)?
                        })?;
                        let v4: u64 = f169(v0.clone(), {
                            let a817 = v3;
                            let a818 = 1u64;
                            nat_add(a817, a818)?
                        })?;
                        let _: u64 = f112(v0.clone(), v4)?;
                        let v7: u64 = {
                            let m821 = f167(v0.clone(), f43(v0.clone(), v4, 1u64), {
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
                        if f170(f43(v0, {
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
                            Ok(f168(f108(Str::lit("only the UTF-8 encoding is read (XML 1.0 section 4.3.3)"), 0u64)?))
                        }
                    }
                }
            }
        }
    } else {
        Ok(Ok::<u64, Bytes>(0u64))
    }
}

pub fn f110(v0: Bytes, p834: u64, p835: u64) -> R<Result<u64, Bytes>> {
    let m833 = (p834, p835);
    {
        let mut m833 = m833;
        loop {
            let step = {
                let (v1, v2) = m833;
                let m830 = v2;
                if m830 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f168(f108(Str::lit("the prolog is longer than the input"), v1)?)))
                } else {
                    let v3 = m830 - 1;
                    let v4: u64 = f169(v0.clone(), v1)?;
                    if f111(v0.clone(), v4, Str::lit("<!--")) {
                        let m831 = f171(v0.clone(), v4)?;
                        match m831 {
                            Ok(v6) => {
                                Ok::<(u64, u64), R<Result<u64, Bytes>>>((v6, v3))
                            }
                            Err(v5) => {
                                Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f168(v5)))
                            }
                        }
                    } else {
                        if f111(v0.clone(), v4, Str::lit("<?")) {
                            let m832 = f172(v0.clone(), v4)?;
                            match m832 {
                                Ok(v8) => {
                                    Ok::<(u64, u64), R<Result<u64, Bytes>>>((v8, v3))
                                }
                                Err(v7) => {
                                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f168(v7)))
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

pub fn f111(v0: Bytes, v1: u64, v2: Str) -> bool {
    f104(v0, {
        let a836 = v2;
        utf8_encode(a836)
    }, v1)
}

pub fn f112(v0: Bytes, v1: u64) -> R<u64> {
    f80(v0, v1)
}

pub fn f113(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Result<(Adt17, u64), Bytes>> {
    let m837 = v2;
    if m837 == 0 {
        Ok(f173(f108(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v4 = m837 - 1;
        let a838 = v3;
        let a839 = 0u64;
        if nat_eq(a838, a839) {
            let v5: u64 = f174(v0.clone(), {
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
                Ok(f173(f108(Str::lit("an element must start with a Name (XML 1.0 [40])"), v1)?))
            } else {
                let v6: Bytes = f43(v0.clone(), {
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
                let m852 = f175(v0.clone(), v5, v4, List::<Bytes>::nil())?;
                match m852 {
                    Ok(v8) => {
                        if f111(v0.clone(), {
                            let (_, h853) = v8.clone();
                            h853
                        }, Str::lit("/>")) {
                            Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C0(v6, std::rc::Rc::new({
                                let (h854, _) = v8;
                                h854
                            }), std::rc::Rc::new(Adt17::C4)), {
                                let a856 = {
                                    let (_, h855) = v8;
                                    h855
                                };
                                let a857 = 2u64;
                                nat_add(a856, a857)?
                            })))
                        } else {
                            let m861 = f113(v0.clone(), {
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
                                    let v12: u64 = f174(v0.clone(), v11)?;
                                    if f46(f43(v0.clone(), v11, {
                                        let a865 = v12;
                                        let a866 = v11;
                                        nat_sub(a865, a866)
                                    }), v6.clone()) {
                                        let v13: u64 = f169(v0.clone(), v12)?;
                                        let a867 = f112(v0, v13)?;
                                        let a868 = 62u64;
                                        if nat_eq(a867, a868) {
                                            Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C0(v6, std::rc::Rc::new({
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
                                            Ok(f173(f108(Str::lit("an end tag must close with > (XML 1.0 [42])"), v13)?))
                                        }
                                    } else {
                                        Ok(f173(f108(Str::lit("an end tag must name the element its start tag opened (XML 1.0 WFC: Element Type Match)"), {
                                            let (_, h873) = v10;
                                            h873
                                        })?))
                                    }
                                }
                                Err(v9) => {
                                    Ok(f173(v9))
                                }
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(f173(v7))
                    }
                }
            }
        } else {
            let a874 = f112(v0.clone(), v1)?;
            let a875 = 256u64;
            if nat_eq(a874, a875) {
                Ok(f173(f108(Str::lit("the input ends before an element is closed (XML 1.0 [39])"), v1)?))
            } else {
                if f111(v0.clone(), v1, Str::lit("</")) {
                    Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C4, v1)))
                } else {
                    if f111(v0.clone(), v1, Str::lit("<!--")) {
                        let m876 = f171(v0.clone(), v1)?;
                        match m876 {
                            Ok(v15) => {
                                f113(v0, v15, v4, 1u64)
                            }
                            Err(v14) => {
                                Ok(f173(v14))
                            }
                        }
                    } else {
                        if f111(v0.clone(), v1, Str::lit("<![CDATA[")) {
                            let m880 = f167(v0.clone(), {
                                let a877 = Str::lit("]]>");
                                utf8_encode(a877)
                            }, {
                                let a878 = v1;
                                let a879 = 9u64;
                                nat_add(a878, a879)?
                            })?;
                            match m880 {
                                None => {
                                    Ok(f173(f108(Str::lit("a CDATA section is not closed (XML 1.0 [18])"), v1)?))
                                }
                                Some(v16) => {
                                    let m883 = f113(v0.clone(), {
                                        let a881 = v16;
                                        let a882 = 3u64;
                                        nat_add(a881, a882)?
                                    }, v4, 1u64)?;
                                    match m883 {
                                        Ok(v18) => {
                                            Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C3(std::rc::Rc::new(Adt17::C2(f43(v0, {
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
                                            Ok(f173(v17))
                                        }
                                    }
                                }
                            }
                        } else {
                            if f111(v0.clone(), v1, Str::lit("<?")) {
                                let m892 = f172(v0.clone(), v1)?;
                                match m892 {
                                    Ok(v20) => {
                                        f113(v0, v20, v4, 1u64)
                                    }
                                    Err(v19) => {
                                        Ok(f173(v19))
                                    }
                                }
                            } else {
                                if f111(v0.clone(), v1, Str::lit("<!")) {
                                    Ok(f173(f108(Str::lit("a markup declaration is not allowed in content (XML 1.0 [43])"), v1)?))
                                } else {
                                    let a893 = f112(v0.clone(), v1)?;
                                    let a894 = 60u64;
                                    if nat_eq(a893, a894) {
                                        let m895 = f113(v0.clone(), v1, v4, 0u64)?;
                                        match m895 {
                                            Ok(v22) => {
                                                let m897 = f113(v0, {
                                                    let (_, h896) = v22;
                                                    h896
                                                }, v4, 1u64)?;
                                                match m897 {
                                                    Ok(v24) => {
                                                        Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C3(std::rc::Rc::new({
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
                                                        Ok(f173(v23))
                                                    }
                                                }
                                            }
                                            Err(v21) => {
                                                Ok(f173(v21))
                                            }
                                        }
                                    } else {
                                        let m901 = f176(v0.clone(), v1, v4, Bytes::lit(&[]), false, 0u64)?;
                                        match m901 {
                                            Ok(v26) => {
                                                let m903 = f113(v0, {
                                                    let (_, h902) = v26;
                                                    h902
                                                }, v4, 1u64)?;
                                                match m903 {
                                                    Ok(v28) => {
                                                        Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C3(std::rc::Rc::new(Adt17::C2({
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
                                                        Ok(f173(v27))
                                                    }
                                                }
                                            }
                                            Err(v25) => {
                                                Ok(f173(v25))
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

pub fn f114(v0: Bytes) -> Result<(u64, Bytes), Bytes> {
    Err::<(u64, Bytes), Bytes>(v0)
}

pub fn f115(v0: Adt17, v1: List<(Bytes, Bytes)>) -> Result<List<(Bytes, Bytes)>, Bytes> {
    let m907 = v0;
    match m907 {
        Adt17::C0(_, _, _) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt17::C1(v2, v3, r912) => {
            let v4 = (*r912).clone();
            let m908 = f115(v4, v1);
            match m908 {
                Ok(v6) => {
                    if f46(v2.clone(), f8(Str::lit("xmlns"))) {
                        if if f46(v3.clone(), f178()) {
                            true
                        } else {
                            f46(v3.clone(), f179())
                        } {
                            f177(f8(Str::lit("the default namespace must not be the xml or xmlns namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Bytes>(List::cons((Bytes::lit(&[]), v3), v6))
                        }
                    } else {
                        if f180(v2.clone(), f8(Str::lit("xmlns:"))) {
                            let v7: Bytes = f43(v2.clone(), 6u64, {
                                let a910 = {
                                    let a909 = v2;
                                    length_bytes(a909)
                                };
                                let a911 = 6u64;
                                nat_sub(a910, a911)
                            });
                            if f46(v7.clone(), f8(Str::lit("xmlns"))) {
                                f177(f8(Str::lit("the prefix xmlns must not be declared (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                            } else {
                                if if f46(v7.clone(), f8(Str::lit("xml"))) {
                                    !f46(v3.clone(), f178())
                                } else {
                                    false
                                } {
                                    f177(f8(Str::lit("the prefix xml may be bound only to its own namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                } else {
                                    if if if f46(v3.clone(), f178()) {
                                        true
                                    } else {
                                        f46(v3.clone(), f179())
                                    } {
                                        !f46(v7.clone(), f8(Str::lit("xml")))
                                    } else {
                                        false
                                    } {
                                        f177(f8(Str::lit("the xml and xmlns namespace names must not be bound to another prefix (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                    } else {
                                        if f46(v3.clone(), Bytes::lit(&[])) {
                                            f177(f8(Str::lit("a prefix must not be undeclared (Namespaces in XML 1.0, section 3, NSC: No Prefix Undeclaring)")))
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
                    f177(v5)
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

pub fn f116(v0: List<(Bytes, Bytes)>, v1: Bytes) -> R<Result<(Bytes, Bytes), Bytes>> {
    let v2: Bytes = f181(v1.clone())?;
    if f46(v2.clone(), Bytes::lit(&[])) {
        let m913 = f182(v0, Bytes::lit(&[]));
        match m913 {
            None => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((Bytes::lit(&[]), v1)))
            }
            Some(v3) => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((v3, v1)))
            }
        }
    } else {
        if f46(v2.clone(), f8(Str::lit("xml"))) {
            Ok(Ok::<(Bytes, Bytes), Bytes>((f178(), f183(v1)?)))
        } else {
            let m914 = f182(v0, v2.clone());
            match m914 {
                None => {
                    Ok(Err::<(Bytes, Bytes), Bytes>(f7(List::cons(f8(Str::lit("the prefix ")), List::cons(v2, List::cons(f8(Str::lit(" is not declared (Namespaces in XML 1.0, section 5, NSC: Prefix Declared)")), List::<Bytes>::nil()))))))
                }
                Some(v4) => {
                    Ok(Ok::<(Bytes, Bytes), Bytes>((v4, f183(v1)?)))
                }
            }
        }
    }
}

pub fn f117(v0: (Bytes, Bytes), v1: Str) -> bool {
    if f46({
        let (h915, _) = v0;
        h915
    }, f184()) {
        f46({
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

pub fn f118(p921: Adt17, v1: Bytes) -> Option<Bytes> {
    let m920 = p921;
    {
        let mut m920 = m920;
        loop {
            let step = {
                let v0 = m920;
                let m918 = v0;
                match m918 {
                    Adt17::C0(_, _, _) => {
                        Err::<Adt17, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt17::C1(v2, v3, r919) => {
                        let v4 = (*r919).clone();
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
                Ok(next) => { m920 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f119(v0: Adt17, v1: List<(Bytes, Bytes)>, v2: Bytes) -> R<Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>> {
    let m922 = v0;
    match m922 {
        Adt17::C0(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt17::C2(_) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt17::C3(r933, r934) => {
            let v3 = (*r933).clone();
            let v4 = (*r934).clone();
            let m923 = f119(v4, v1.clone(), v2.clone())?;
            match m923 {
                Ok(v6) => {
                    let m924 = v3;
                    match m924 {
                        Adt17::C0(v7, r931, r932) => {
                            let v8 = (*r931).clone();
                            let v9 = (*r932).clone();
                            let m925 = f115(v8.clone(), v1);
                            match m925 {
                                Ok(v11) => {
                                    let m926 = f116(v11.clone(), v7)?;
                                    match m926 {
                                        Ok(v13) => {
                                            if f117(v13, Str::lit("simpleType")) {
                                                let m927 = f118(v8, f8(Str::lit("name")));
                                                match m927 {
                                                    None => {
                                                        Ok(f185(f8(Str::lit("a top-level simpleType must have a name (XSD 1.1 Part 2, section 4.1.2)"))))
                                                    }
                                                    Some(v14) => {
                                                        let m928 = f186(v14.clone(), v9, v11, v2)?;
                                                        match m928 {
                                                            Ok(v16) => {
                                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::cons((f187(v14)?, (match v16.clone() {
                                                                    Adt20::C0(_, h929) => {
                                                                        h929
                                                                    }
                                                                }, match v16 {
                                                                    Adt20::C0(h930, _) => {
                                                                        h930
                                                                    }
                                                                })), v6)))
                                                            }
                                                            Err(v15) => {
                                                                Ok(f185(v15))
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                                            }
                                        }
                                        Err(v12) => {
                                            Ok(f185(v12))
                                        }
                                    }
                                }
                                Err(v10) => {
                                    Ok(f185(v10))
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
                    Ok(f185(v5))
                }
            }
        }
        Adt17::C4 => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
    }
}

pub fn f120(v0: List<(Bytes, (List<Bytes>, Bytes))>) -> List<Bytes> {
    let m935 = v0;
    match m935.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h936, _) = v1;
                h936
            }, f120(v2))
        }
    }
}

pub fn f121(p939: List<Bytes>) -> Option<Bytes> {
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
                        if f149(v1.clone(), v2.clone()) {
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

pub fn f122(p945: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<Bytes> {
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
                        let m943 = f188({
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

pub fn f123(p956: u64, p957: List<(Bytes, (List<Bytes>, Bytes))>, p958: List<Bytes>, p959: List<Bytes>) -> Result<List<Bytes>, Bytes> {
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
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(f189(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
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
                            let m949 = f156(v1, v2.clone());
                            match m949 {
                                None => {
                                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(f189(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
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

pub fn f124(v0: Bytes) -> R<u64> {
    let (h960, _) = f164(Fn4::F190(v0.clone()), {
        let a962 = {
            let a961 = v0;
            length_bytes(a961)
        };
        let a963 = 1u64;
        nat_add(a962, a963)?
    }, 0u64)?;
    Ok(h960)
}

pub fn f125(v0: Bytes, v1: u64) -> R<u64> {
    let (h964, _) = f164(Fn4::F191(v0.clone()), {
        let a966 = {
            let a965 = v0;
            length_bytes(a965)
        };
        let a967 = 1u64;
        nat_add(a966, a967)?
    }, v1)?;
    Ok(h964)
}

pub fn f126(v0: Bytes, v1: u64) -> R<bool> {
    if if if f111(v0.clone(), v1, Str::lit("---")) {
        true
    } else {
        f111(v0.clone(), v1, Str::lit("..."))
    } {
        if f192(f127(v0.clone(), {
            let a968 = v1;
            let a969 = 3u64;
            nat_add(a968, a969)?
        })?) {
            true
        } else {
            f193(f127(v0.clone(), {
                let a970 = v1;
                let a971 = 3u64;
                nat_add(a970, a971)?
            })?)
        }
    } else {
        false
    } {
        let a972 = f194(v0, v1)?;
        let a973 = 0u64;
        Ok(nat_eq(a972, a973))
    } else {
        Ok(false)
    }
}

pub fn f127(v0: Bytes, v1: u64) -> R<u64> {
    f80(v0, v1)
}

pub fn f128(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: List<Bytes>) -> R<Adt18> {
    let m974 = v3;
    if m974 == 0 {
        Ok(Adt18::C1(f108(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v8 = m974 - 1;
        let a975 = v4;
        let a976 = 0u64;
        if nat_eq(a975, a976) {
            let v9: u64 = f195(v0.clone(), v1)?;
            let v10: u64 = {
                let a977 = v9;
                let a978 = v1;
                nat_sub(a977, a978)
            };
            let a979 = f127(v0.clone(), v9)?;
            let a980 = 9u64;
            if nat_eq(a979, a980) {
                let v11: u64 = f196(v0.clone(), v9)?;
                if if f197(v0.clone(), v11)? {
                    true
                } else {
                    f198(v0.clone(), v11)?
                } {
                    Ok(Adt18::C1(f108(Str::lit("a tab cannot indent a block node (YAML 1.2.2 [63])"), v11)?))
                } else {
                    let m981 = f199(v0.clone(), v11)?;
                    match m981 {
                        None => {
                            f200(v0, v11, v2)
                        }
                        Some(v12) => {
                            Ok(Adt18::C1(v12))
                        }
                    }
                }
            } else {
                if f197(v0.clone(), v9)? {
                    Ok(f201(true, f128(v0, v9, v2, v8, 2u64, false, v10, List::<Bytes>::nil())?))
                } else {
                    let m982 = f199(v0.clone(), v9)?;
                    match m982 {
                        None => {
                            if f198(v0.clone(), v9)? {
                                Ok(f201(false, f128(v0, v9, v2, v8, 3u64, false, v10, List::<Bytes>::nil())?))
                            } else {
                                f200(v0, v9, v2)
                            }
                        }
                        Some(v13) => {
                            Ok(Adt18::C1(v13))
                        }
                    }
                }
            }
        } else {
            let a983 = v4;
            let a984 = 1u64;
            if nat_eq(a983, a984) {
                let v14: u64 = f196(v0.clone(), v1)?;
                let v15: u64 = f127(v0.clone(), v14)?;
                if if f193(v15) {
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
                    let v16: u64 = f125(v0.clone(), f202(v0.clone(), v14)?)?;
                    let a990 = {
                        let a989 = v0.clone();
                        length_bytes(a989)
                    };
                    let a991 = v16;
                    if if nat_le(a990, a991) {
                        true
                    } else {
                        f126(v0.clone(), v16)?
                    } {
                        Ok(Adt18::C1(f203(v14)?))
                    } else {
                        let v17: u64 = {
                            let a992 = f195(v0.clone(), v16)?;
                            let a993 = v16;
                            nat_sub(a992, a993)
                        };
                        let a994 = v2;
                        let a995 = v17;
                        if nat_le(a994, a995) {
                            f128(v0, v16, v2, v8, 0u64, false, 0u64, List::<Bytes>::nil())
                        } else {
                            if if if f204(v5) {
                                let a996 = v17;
                                let a997 = v6;
                                nat_eq(a996, a997)
                            } else {
                                false
                            } {
                                f197(v0.clone(), {
                                    let a998 = v16;
                                    let a999 = v17;
                                    nat_add(a998, a999)?
                                })?
                            } else {
                                false
                            } {
                                Ok(f201(true, f128(v0, {
                                    let a1000 = v16;
                                    let a1001 = v17;
                                    nat_add(a1000, a1001)?
                                }, v2, v8, 2u64, false, v17, List::<Bytes>::nil())?))
                            } else {
                                Ok(Adt18::C1(f203(v14)?))
                            }
                        }
                    }
                } else {
                    if if v5 {
                        f197(v0.clone(), v14)?
                    } else {
                        false
                    } {
                        Ok(f201(true, f128(v0.clone(), v14, v2, v8, 2u64, false, f194(v0, v14)?, List::<Bytes>::nil())?))
                    } else {
                        let m1002 = f199(v0.clone(), v14)?;
                        match m1002 {
                            None => {
                                if if v5 {
                                    f198(v0.clone(), v14)?
                                } else {
                                    false
                                } {
                                    Ok(f201(false, f128(v0.clone(), v14, v2, v8, 3u64, false, f194(v0, v14)?, List::<Bytes>::nil())?))
                                } else {
                                    f200(v0, v14, v2)
                                }
                            }
                            Some(v18) => {
                                Ok(Adt18::C1(v18))
                            }
                        }
                    }
                }
            } else {
                let a1003 = v4;
                let a1004 = 2u64;
                if nat_eq(a1003, a1004) {
                    let m1009 = f128(v0.clone(), {
                        let a1005 = v1;
                        let a1006 = 1u64;
                        nat_add(a1005, a1006)?
                    }, {
                        let a1007 = v6;
                        let a1008 = 1u64;
                        nat_add(a1007, a1008)?
                    }, v8, 1u64, true, v6, List::<Bytes>::nil())?;
                    match m1009 {
                        Adt18::C0(v20, v21) => {
                            let v22: u64 = f125(v0.clone(), v21)?;
                            let v23: u64 = {
                                let a1010 = f195(v0.clone(), v22)?;
                                let a1011 = v22;
                                nat_sub(a1010, a1011)
                            };
                            let a1013 = {
                                let a1012 = v0.clone();
                                length_bytes(a1012)
                            };
                            let a1014 = v22;
                            if nat_le(a1013, a1014) {
                                Ok(Adt18::C0(Adt2::C7(std::rc::Rc::new(v20), std::rc::Rc::new(Adt2::C6)), v22))
                            } else {
                                let a1015 = v23;
                                let a1016 = v6;
                                if if nat_eq(a1015, a1016) {
                                    f197(v0.clone(), {
                                        let a1017 = v22;
                                        let a1018 = v23;
                                        nat_add(a1017, a1018)?
                                    })?
                                } else {
                                    false
                                } {
                                    let m1021 = f128(v0, {
                                        let a1019 = v22;
                                        let a1020 = v23;
                                        nat_add(a1019, a1020)?
                                    }, v2, v8, 2u64, false, v6, List::<Bytes>::nil())?;
                                    match m1021 {
                                        Adt18::C0(v25, v26) => {
                                            Ok(Adt18::C0(Adt2::C7(std::rc::Rc::new(v20), std::rc::Rc::new(v25)), v26))
                                        }
                                        Adt18::C1(v24) => {
                                            Ok(Adt18::C1(v24))
                                        }
                                    }
                                } else {
                                    let a1022 = v6;
                                    let a1023 = v23;
                                    if nat_lt(a1022, a1023) {
                                        Ok(Adt18::C1(f108(Str::lit("a line is indented more than its block sequence entry allows (YAML 1.2.2 [183])"), {
                                            let a1024 = v22;
                                            let a1025 = v23;
                                            nat_add(a1024, a1025)?
                                        })?))
                                    } else {
                                        Ok(Adt18::C0(Adt2::C7(std::rc::Rc::new(v20), std::rc::Rc::new(Adt2::C6)), v22))
                                    }
                                }
                            }
                        }
                        Adt18::C1(v19) => {
                            Ok(Adt18::C1(v19))
                        }
                    }
                } else {
                    let m1026 = f199(v0.clone(), v1)?;
                    match m1026 {
                        None => {
                            let m1027 = f205(v0.clone(), v1)?;
                            match m1027 {
                                None => {
                                    Ok(Adt18::C1(f108(Str::lit("a block mapping entry must be an implicit key and : (YAML 1.2.2 [188])"), v1)?))
                                }
                                Some(v28) => {
                                    let v29: Bytes = f206(v0.clone(), v1)?;
                                    if f207(v29.clone(), v7.clone()) {
                                        Ok(Adt18::C1(f108(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                    } else {
                                        let m1032 = f128(v0.clone(), {
                                            let a1028 = v28;
                                            let a1029 = 1u64;
                                            nat_add(a1028, a1029)?
                                        }, {
                                            let a1030 = v6;
                                            let a1031 = 1u64;
                                            nat_add(a1030, a1031)?
                                        }, v8, 1u64, false, v6, List::<Bytes>::nil())?;
                                        match m1032 {
                                            Adt18::C0(v31, v32) => {
                                                let v33: u64 = f125(v0.clone(), v32)?;
                                                let v34: u64 = {
                                                    let a1033 = f195(v0.clone(), v33)?;
                                                    let a1034 = v33;
                                                    nat_sub(a1033, a1034)
                                                };
                                                let a1036 = {
                                                    let a1035 = v0.clone();
                                                    length_bytes(a1035)
                                                };
                                                let a1037 = v33;
                                                if nat_le(a1036, a1037) {
                                                    Ok(Adt18::C0(Adt2::C8(v29, std::rc::Rc::new(v31), std::rc::Rc::new(Adt2::C6)), v33))
                                                } else {
                                                    let a1038 = v34;
                                                    let a1039 = v6;
                                                    if if if nat_eq(a1038, a1039) {
                                                        f204(f197(v0.clone(), {
                                                            let a1040 = v33;
                                                            let a1041 = v34;
                                                            nat_add(a1040, a1041)?
                                                        })?)
                                                    } else {
                                                        false
                                                    } {
                                                        f204(f126(v0.clone(), v33)?)
                                                    } else {
                                                        false
                                                    } {
                                                        let m1044 = f128(v0, {
                                                            let a1042 = v33;
                                                            let a1043 = v34;
                                                            nat_add(a1042, a1043)?
                                                        }, v2, v8, 3u64, false, v6, List::cons(v29.clone(), v7))?;
                                                        match m1044 {
                                                            Adt18::C0(v36, v37) => {
                                                                Ok(Adt18::C0(Adt2::C8(v29, std::rc::Rc::new(v31), std::rc::Rc::new(v36)), v37))
                                                            }
                                                            Adt18::C1(v35) => {
                                                                Ok(Adt18::C1(v35))
                                                            }
                                                        }
                                                    } else {
                                                        let a1045 = v6;
                                                        let a1046 = v34;
                                                        if nat_lt(a1045, a1046) {
                                                            Ok(Adt18::C1(f108(Str::lit("a line is indented more than its block mapping entry allows (YAML 1.2.2 [187])"), {
                                                                let a1047 = v33;
                                                                let a1048 = v34;
                                                                nat_add(a1047, a1048)?
                                                            })?))
                                                        } else {
                                                            Ok(Adt18::C0(Adt2::C8(v29, std::rc::Rc::new(v31), std::rc::Rc::new(Adt2::C6)), v33))
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

pub fn f129(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f125(v0.clone(), v1)?;
    let v3: u64 = {
        let a1049 = v0.clone();
        length_bytes(a1049)
    };
    let a1050 = v2;
    let a1051 = v3;
    if if nat_lt(a1050, a1051) {
        if f111(v0.clone(), v2, Str::lit("...")) {
            f126(v0.clone(), v2)?
        } else {
            false
        }
    } else {
        false
    } {
        let m1054 = f208(v0.clone(), {
            let a1052 = v2;
            let a1053 = 3u64;
            nat_add(a1052, a1053)?
        })?;
        match m1054 {
            Ok(v5) => {
                let v6: u64 = f125(v0, v5)?;
                let a1055 = v6;
                let a1056 = v3;
                if nat_lt(a1055, a1056) {
                    Ok(Err::<u64, Bytes>(f108(Str::lit("a stream of more than one document is not read (YAML 1.2.2 [211])"), v6)?))
                } else {
                    Ok(Ok::<u64, Bytes>(v6))
                }
            }
            Err(v4) => {
                Ok(Err::<u64, Bytes>(v4))
            }
        }
    } else {
        let a1057 = v2;
        let a1058 = v3;
        if if nat_lt(a1057, a1058) {
            f126(v0, v2)?
        } else {
            false
        } {
            Ok(Err::<u64, Bytes>(f108(Str::lit("a stream of more than one document is not read (YAML 1.2.2 [211])"), v2)?))
        } else {
            let a1059 = v2;
            let a1060 = v3;
            if nat_lt(a1059, a1060) {
                Ok(Err::<u64, Bytes>(f108(Str::lit("only comments may follow the root node (YAML 1.2.2 [207])"), v2)?))
            } else {
                Ok(Ok::<u64, Bytes>(v2))
            }
        }
    }
}

pub fn f130(v0: Adt2) -> R<Bytes> {
    let m1061 = v0;
    match m1061 {
        Adt2::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt2::C1(v2, v3) => {
            f209(v2, v3)
        }
        Adt2::C2(v4, v5, v6) => {
            f210(v4, v5, v6)
        }
        Adt2::C3(v7) => {
            f211(v7)
        }
        Adt2::C4(r1062) => {
            let v8 = (*r1062).clone();
            Ok(f212(f213(v8)?))
        }
        Adt2::C5(r1063) => {
            let v9 = (*r1063).clone();
            f214(f215(v9)?)
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

pub fn f131(v0: Bytes, v1: u64, v2: u64, v3: Bytes) -> Bytes {
    let a1066 = v3;
    let a1067 = f43(v0, v1, {
        let a1064 = v2;
        let a1065 = v1;
        nat_sub(a1064, a1065)
    });
    append_bytes(a1066, a1067)
}

pub fn f132(v0: Bytes, p1080: u64, p1081: u64) -> R<u64> {
    let m1079 = (p1080, p1081);
    {
        let mut m1079 = m1079;
        loop {
            let step = {
                let (v1, v2) = m1079;
                let m1068 = v2;
                if m1068 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m1068 - 1;
                    let v4: u64 = f80(v0.clone(), v1)?;
                    let a1069 = v4;
                    let a1070 = 34u64;
                    if nat_eq(a1069, a1070) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a1071 = v4;
                        let a1072 = 92u64;
                        if nat_eq(a1071, a1072) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a1073 = v4;
                            let a1074 = 32u64;
                            if nat_lt(a1073, a1074) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a1075 = v4;
                                let a1076 = 256u64;
                                if nat_eq(a1075, a1076) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a1077 = v1;
                                        let a1078 = 1u64;
                                        nat_add(a1077, a1078)?
                                    }, v3))
                                }
                            }
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

pub fn f133(v0: Bytes, p1107: u64, p1108: u64, p1109: Bytes) -> R<Option<(u64, Bytes)>> {
    let m1106 = (p1107, (p1108, p1109));
    {
        let mut m1106 = m1106;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1106;
                let m1082 = v2;
                if m1082 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                } else {
                    let v4 = m1082 - 1;
                    let v5: u64 = f132(v0.clone(), v1, {
                        let a1086 = {
                            let a1084 = {
                                let a1083 = v0.clone();
                                length_bytes(a1083)
                            };
                            let a1085 = v1;
                            nat_sub(a1084, a1085)
                        };
                        let a1087 = 1u64;
                        nat_add(a1086, a1087)?
                    })?;
                    let v6: Bytes = {
                        let a1090 = v3;
                        let a1091 = f43(v0.clone(), v1, {
                            let a1088 = v5;
                            let a1089 = v1;
                            nat_sub(a1088, a1089)
                        });
                        append_bytes(a1090, a1091)
                    };
                    let v7: u64 = f80(v0.clone(), v5)?;
                    let a1092 = v7;
                    let a1093 = 34u64;
                    if nat_eq(a1092, a1093) {
                        Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(Some(({
                            let a1094 = v5;
                            let a1095 = 1u64;
                            nat_add(a1094, a1095)?
                        }, v6))))
                    } else {
                        let a1096 = v7;
                        let a1097 = 92u64;
                        if nat_eq(a1096, a1097) {
                            let m1098 = f216(v0.clone(), v5, v6)?;
                            match m1098 {
                                None => {
                                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                                }
                                Some(v8) => {
                                    let a1101 = {
                                        let (_, h1099) = {
                                            let (_, h1100) = v8.clone();
                                            h1100
                                        };
                                        h1099
                                    };
                                    let a1102 = 0u64;
                                    if nat_eq(a1101, a1102) {
                                        Ok::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(({
                                            let (h1103, _) = v8;
                                            h1103
                                        }, (v4, {
                                            let (h1104, _) = {
                                                let (_, h1105) = v8;
                                                h1105
                                            };
                                            h1104
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
                Ok(next) => { m1106 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f134(v0: Bytes, v1: u64) -> R<u64> {
    let (h1110, _) = f164(Fn4::F217(v0.clone()), {
        let a1112 = {
            let a1111 = v0;
            length_bytes(a1111)
        };
        let a1113 = 1u64;
        nat_add(a1112, a1113)?
    }, v1)?;
    Ok(h1110)
}

pub fn f135(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f136(v0: Bytes) -> R<u64> {
    let (h1114, _) = f164(Fn4::F218(v0.clone()), {
        let a1116 = {
            let a1115 = v0;
            length_bytes(a1115)
        };
        let a1117 = 1u64;
        nat_add(a1116, a1117)?
    }, 0u64)?;
    Ok(h1114)
}

pub fn f137(v0: Bytes) -> R<Option<u64>> {
    if f219(v0.clone())? {
        Ok(Some({
            let (_, h1118) = {
                let (h1119, _) = f220(Fn5::F221(v0.clone()), {
                    let a1120 = v0;
                    length_bytes(a1120)
                }, (0u64, 0u64))?;
                h1119
            };
            h1118
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f138(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f43(v1.clone(), 0u64, {
        let a1122 = {
            let a1121 = v1.clone();
            length_bytes(a1121)
        };
        let a1123 = f222(v1)?;
        nat_sub(a1122, a1123)
    });
    let v5: Bytes = {
        let a1124 = v0;
        let a1125 = v4.clone();
        append_bytes(a1124, a1125)
    };
    let v6: u64 = f136(v5.clone())?;
    let v7: Bytes = f43(v5.clone(), v6, {
        let a1127 = {
            let a1126 = v5;
            length_bytes(a1126)
        };
        let a1128 = v6;
        nat_sub(a1127, a1128)
    });
    let v8: u64 = {
        let a1129 = v4;
        length_bytes(a1129)
    };
    let v9: u64 = {
        let a1130 = v7.clone();
        length_bytes(a1130)
    };
    let a1131 = v9;
    let a1132 = 0u64;
    if nat_eq(a1131, a1132) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f135(v2, {
            let a1133 = v3;
            let a1134 = v8;
            nat_lt(a1133, a1134)
        }) {
            let v10: u64 = if v2 {
                let a1135 = v3;
                let a1136 = v8;
                nat_add(a1135, a1136)?
            } else {
                let a1137 = v8;
                let a1138 = v3;
                nat_sub(a1137, a1138)
            };
            let v11: u64 = f222(v7.clone())?;
            let a1139 = v10;
            let a1140 = v11;
            if nat_le(a1139, a1140) {
                Ok(Some((f43(v7, 0u64, {
                    let a1141 = v9;
                    let a1142 = v10;
                    nat_sub(a1141, a1142)
                }), 0u64)))
            } else {
                let a1145 = {
                    let a1143 = v10;
                    let a1144 = v11;
                    nat_sub(a1143, a1144)
                };
                let a1146 = 19u64;
                if nat_le(a1145, a1146) {
                    Ok(Some((f43(v7, 0u64, {
                        let a1147 = v9;
                        let a1148 = v11;
                        nat_sub(a1147, a1148)
                    }), {
                        let a1149 = v10;
                        let a1150 = v11;
                        nat_sub(a1149, a1150)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a1151 = v3;
                let a1152 = v8;
                nat_sub(a1151, a1152)
            };
            let a1155 = {
                let a1153 = v9;
                let a1154 = v12;
                nat_add(a1153, a1154)?
            };
            let a1156 = 19u64;
            if nat_le(a1155, a1156) {
                Ok(Some(({
                    let a1157 = v7;
                    let a1158 = f223(v12);
                    append_bytes(a1157, a1158)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f139(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1159 = 0u64;
        let a1160 = v1;
        if nat_lt(a1159, a1160) {
            let a1161 = v1;
            let a1162 = 9223372036854775808u64;
            nat_le(a1161, a1162)
        } else {
            false
        }
    } else {
        let a1163 = v1;
        let a1164 = 9223372036854775807u64;
        nat_le(a1163, a1164)
    }
}

pub fn f140(v0: Bytes, v1: u64) -> Option<u8> {
    let a1165 = v0;
    let a1166 = v1;
    index_bytes(a1165, a1166)
}

pub fn f141(v0: Bytes) -> R<Result<List<Adt19>, Bytes>> {
    f224({
        let a1168 = {
            let a1167 = v0.clone();
            length_bytes(a1167)
        };
        let a1169 = 1u64;
        nat_add(a1168, a1169)?
    }, v0, 0u64, List::<Adt19>::nil())
}

pub fn f142(p1177: u64, p1178: List<Adt19>, p1179: List<Adt13>) -> R<Result<List<Adt13>, Bytes>> {
    let m1176 = (p1177, (p1178, p1179));
    {
        let mut m1176 = m1176;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1176;
                let m1170 = v0;
                if m1170 == 0 {
                    Err::<(u64, (List<Adt19>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>(Ok(Err::<List<Adt13>, Bytes>(f8(Str::lit("the input has more definitions than tokens")))))
                } else {
                    let v3 = m1170 - 1;
                    let m1171 = v1.clone();
                    match m1171.uncons() {
                        None => {
                            Err::<(u64, (List<Adt19>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>(Ok(Ok::<List<Adt13>, Bytes>(f225(v2, List::<Adt13>::nil()))))
                        }
                        Some((_, _)) => {
                            let m1172 = f226(v1)?;
                            match m1172 {
                                Ok(v7) => {
                                    let m1173 = f227(v3, v7)?;
                                    match m1173 {
                                        Ok(v9) => {
                                            Ok::<(u64, (List<Adt19>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>((v3, ({
                                                let (_, h1174) = v9;
                                                h1174
                                            }, List::cons({
                                                let (h1175, _) = v9;
                                                h1175
                                            }, v2))))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (List<Adt19>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>(Ok(Err::<List<Adt13>, Bytes>(v8)))
                                        }
                                    }
                                }
                                Err(v6) => {
                                    Err::<(u64, (List<Adt19>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>(Ok(Err::<List<Adt13>, Bytes>(v6)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1176 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f143(v0: List<Adt16>, v1: List<Adt16>) -> List<Adt16> {
    let m1180 = v0;
    match m1180.uncons() {
        None => {
            List::<Adt16>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = if f46(f151(v2.clone()), Bytes::lit(&[])) {
                f228(f153(v2.clone()), v1.clone())
            } else {
                f151(v2.clone())
            };
            List::cons(Adt16::C0(f154(v2.clone()), v4, f155(v2.clone()), f153(v2)), f143(v3, v1))
        }
    }
}

pub fn f144(v0: List<Adt16>) -> R<u64> {
    let m1181 = v0;
    match m1181.uncons() {
        None => {
            Ok(0u64)
        }
        Some((v1, v2)) => {
            if f46(f151(v1), Bytes::lit(&[])) {
                f144(v2)
            } else {
                let a1182 = f144(v2)?;
                let a1183 = 1u64;
                nat_add(a1182, a1183)
            }
        }
    }
}

pub fn f145(v0: u64) -> bool {
    let a1184 = v0;
    let a1185 = 0u64;
    if nat_eq(a1184, a1185) {
        true
    } else {
        let a1186 = v0;
        let a1187 = 2u64;
        if nat_eq(a1186, a1187) {
            true
        } else {
            let a1188 = v0;
            let a1189 = 3u64;
            if nat_eq(a1188, a1189) {
                true
            } else {
                let a1190 = v0;
                let a1191 = 4u64;
                if nat_eq(a1190, a1191) {
                    true
                } else {
                    let a1192 = v0;
                    let a1193 = 9u64;
                    nat_eq(a1192, a1193)
                }
            }
        }
    }
}

pub fn f146(v0: Adt13) -> u64 {
    let m1194 = v0;
    match m1194 {
        Adt13::C0(v1, _, _, _, _, _) => {
            v1
        }
    }
}

pub fn f147(v0: List<Adt13>, v1: Adt13) -> R<Result<(Bytes, List<Bytes>), Bytes>> {
    let v2: u64 = f146(v1.clone());
    let v3: Bytes = f187(f148(v1.clone()))?;
    if if f229(v3.clone()) {
        f145(v2)
    } else {
        false
    } {
        Ok(Err::<(Bytes, List<Bytes>), Bytes>(f230(v3)))
    } else {
        let a1195 = v2;
        let a1196 = 0u64;
        if if nat_eq(a1195, a1196) {
            true
        } else {
            let a1197 = v2;
            let a1198 = 2u64;
            nat_eq(a1197, a1198)
        } {
            let m1199 = f231(v0.clone(), v0.clone(), f148(v1.clone()));
            match m1199 {
                None => {
                    let m1203 = f232({
                        let a1201 = {
                            let a1200 = v0.clone();
                            length_list(a1200)
                        };
                        let a1202 = 1u64;
                        nat_add(a1201, a1202)?
                    }, v0, v1)?;
                    match m1203 {
                        Ok(v5) => {
                            let a1205 = {
                                let a1204 = v5.clone();
                                length_list(a1204)
                            };
                            let a1206 = 0u64;
                            if nat_eq(a1205, a1206) {
                                Ok(Err::<(Bytes, List<Bytes>), Bytes>(f8(Str::lit("it has no data members, and LexLean refuses a structure with no fields"))))
                            } else {
                                let m1207 = f233(f234(f235(v5.clone()))?, v5)?;
                                match m1207 {
                                    Ok(v7) => {
                                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f236(v3, {
                                            let (h1208, _) = v7;
                                            h1208
                                        }), {
                                            let (_, h1209) = v7;
                                            h1209
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
            let a1210 = v2;
            let a1211 = 3u64;
            if nat_eq(a1210, a1211) {
                let m1212 = f121(f237(v1.clone()));
                match m1212 {
                    None => {
                        let m1213 = f238(f234(f237(v1.clone()))?, f237(v1.clone()))?;
                        match m1213 {
                            None => {
                                let m1214 = f239(f234(f237(v1.clone()))?, f240(f237(v1)), List::<Bytes>::nil())?;
                                match m1214 {
                                    Ok(v10) => {
                                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f241(v3, v10), List::<Bytes>::nil())))
                                    }
                                    Err(v9) => {
                                        Ok(Err::<(Bytes, List<Bytes>), Bytes>(v9))
                                    }
                                }
                            }
                            Some(v11) => {
                                Ok(Err::<(Bytes, List<Bytes>), Bytes>(f230(v11)))
                            }
                        }
                    }
                    Some(v12) => {
                        Ok(Err::<(Bytes, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("the enumeration value \"")), List::cons(v12, List::cons(f8(Str::lit("\" appears twice (Web IDL 2.9: the values must be distinct)")), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                let a1215 = v2;
                let a1216 = 4u64;
                if nat_eq(a1215, a1216) {
                    let v13: Adt15 = f242(v1);
                    if !f46(f243(v13.clone()), Bytes::lit(&[])) {
                        Ok(Err::<(Bytes, List<Bytes>), Bytes>(f243(v13)))
                    } else {
                        let a1217 = f244(v13.clone());
                        let a1218 = 1u64;
                        if nat_eq(a1217, a1218) {
                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f241(v3, List::cons(f245(v13.clone()), List::<Bytes>::nil())), f246(v13))))
                        } else {
                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f247(v3, f245(v13.clone())), f246(v13))))
                        }
                    }
                } else {
                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f7(f237(v1))))
                }
            }
        }
    }
}

pub fn f148(v0: Adt13) -> Bytes {
    let m1219 = v0;
    match m1219 {
        Adt13::C0(_, v2, _, _, _, _) => {
            v2
        }
    }
}

pub fn f149(v0: Bytes, p1222: List<Bytes>) -> bool {
    let m1221 = p1222;
    {
        let mut m1221 = m1221;
        loop {
            let step = {
                let v1 = m1221;
                let m1220 = v1;
                match m1220.uncons() {
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
                Ok(next) => { m1221 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f150(p1225: List<Adt16>, v1: Bytes) -> Option<Adt16> {
    let m1224 = p1225;
    {
        let mut m1224 = m1224;
        loop {
            let step = {
                let v0 = m1224;
                let m1223 = v0;
                match m1223.uncons() {
                    None => {
                        Err::<List<Adt16>, Option<Adt16>>(None::<Adt16>)
                    }
                    Some((v2, v3)) => {
                        if f46(f154(v2.clone()), v1.clone()) {
                            Err::<List<Adt16>, Option<Adt16>>(Some(v2))
                        } else {
                            Ok::<List<Adt16>, Option<Adt16>>(v3)
                        }
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

pub fn f151(v0: Adt16) -> Bytes {
    let m1226 = v0;
    match m1226 {
        Adt16::C0(_, v2, _, _) => {
            v2
        }
    }
}

pub fn f152(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m1227 = v0;
    match m1227.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f152(v3, v1))
        }
    }
}

pub fn f153(v0: Adt16) -> List<Bytes> {
    let m1228 = v0;
    match m1228 {
        Adt16::C0(_, _, _, v4) => {
            v4
        }
    }
}

pub fn f154(v0: Adt16) -> Bytes {
    let m1229 = v0;
    match m1229 {
        Adt16::C0(v1, _, _, _) => {
            v1
        }
    }
}

pub fn f155(v0: Adt16) -> Bytes {
    let m1230 = v0;
    match m1230 {
        Adt16::C0(_, _, v3, _) => {
            v3
        }
    }
}

pub fn f156(v0: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)> {
    let m1231 = v0;
    match m1231.uncons() {
        None => {
            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
        }
        Some((v2, v3)) => {
            let m1234 = f188({
                let (h1232, _) = {
                    let (_, h1233) = v2.clone();
                    h1233
                };
                h1232
            }, v1.clone());
            match m1234 {
                None => {
                    Some((({
                        let (h1235, _) = v2;
                        h1235
                    }, {
                        let (_, h1236) = {
                            let (_, h1237) = v2;
                            h1237
                        };
                        h1236
                    }), v3))
                }
                Some(_) => {
                    let m1238 = f156(v3, v1);
                    match m1238 {
                        None => {
                            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h1239, _) = v5;
                                h1239
                            }, List::cons(v2, {
                                let (_, h1240) = v5;
                                h1240
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f157(p1245: List<Adt13>, v1: u64, v2: Bytes) -> Option<Adt13> {
    let m1244 = p1245;
    {
        let mut m1244 = m1244;
        loop {
            let step = {
                let v0 = m1244;
                let m1241 = v0;
                match m1241.uncons() {
                    None => {
                        Err::<List<Adt13>, Option<Adt13>>(None::<Adt13>)
                    }
                    Some((v3, v4)) => {
                        let a1242 = f146(v3.clone());
                        let a1243 = v1;
                        if if nat_eq(a1242, a1243) {
                            f46(f148(v3.clone()), v2.clone())
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
                Ok(next) => { m1244 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f158(v0: u64) -> Bytes {
    let a1246 = v0;
    let a1247 = 5u64;
    if nat_eq(a1246, a1247) {
        f8(Str::lit("partial interface "))
    } else {
        let a1248 = v0;
        let a1249 = 7u64;
        if nat_eq(a1248, a1249) {
            f8(Str::lit("partial dictionary "))
        } else {
            f8(Str::lit(""))
        }
    }
}

pub fn f159(v0: Adt13) -> Bytes {
    let m1250 = v0;
    match m1250 {
        Adt13::C0(_, _, v3, _, _, _) => {
            v3
        }
    }
}

pub fn f160(v0: Bytes, v1: u64) -> Bytes {
    let m1254 = {
        let a1251 = v0;
        let a1252 = v1;
        let a1253 = 1u64;
        slice_bytes(a1251, a1252, a1253)
    };
    match m1254 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f161(v0: Fn3, p1259: u64, p1260: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1258 = (p1259, p1260);
    {
        let mut m1258 = m1258;
        loop {
            let step = {
                let (v1, v2) = m1258;
                let m1255 = v1;
                if m1255 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1255 - 1;
                    let m1257 = {
                        let c1256: Fn3 = v0.clone();
                        c1256.apply(v2.clone())?
                    };
                    match m1257 {
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
                Ok(next) => { m1258 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f162(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f248(v1, v0, v2)
}

pub fn f163(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f249(v0, v1)
}

pub fn f164(v0: Fn4, p1265: u64, p1266: u64) -> R<(u64, bool)> {
    let m1264 = (p1265, p1266);
    {
        let mut m1264 = m1264;
        loop {
            let step = {
                let (v1, v2) = m1264;
                let m1261 = v1;
                if m1261 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1261 - 1;
                    let m1263 = {
                        let c1262: Fn4 = v0.clone();
                        c1262.apply(v2)?
                    };
                    match m1263 {
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
                Ok(next) => { m1264 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f165(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f250(v0, v1)
}

pub fn f166(v0: u64) -> R<bool> {
    f251(v0, f252())
}

pub fn f167(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let v3: u64 = {
        let (h1267, _) = f164(Fn4::F253(v1.clone(), v0.clone()), {
            let a1269 = {
                let a1268 = v0.clone();
                length_bytes(a1268)
            };
            let a1270 = 1u64;
            nat_add(a1269, a1270)?
        }, v2)?;
        h1267
    };
    if f104(v0, v1, v3) {
        Ok(Some(v3))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f168(v0: Bytes) -> Result<u64, Bytes> {
    Err::<u64, Bytes>(v0)
}

pub fn f169(v0: Bytes, v1: u64) -> R<u64> {
    let (h1271, _) = f164(Fn4::F254(v0.clone()), {
        let a1273 = {
            let a1272 = v0;
            length_bytes(a1272)
        };
        let a1274 = 1u64;
        nat_add(a1273, a1274)?
    }, v1)?;
    Ok(h1271)
}

pub fn f170(v0: Bytes) -> bool {
    f207(v0, List::cons(f8(Str::lit("UTF-8")), List::cons(f8(Str::lit("UTf-8")), List::cons(f8(Str::lit("UtF-8")), List::cons(f8(Str::lit("Utf-8")), List::cons(f8(Str::lit("uTF-8")), List::cons(f8(Str::lit("uTf-8")), List::cons(f8(Str::lit("utF-8")), List::cons(f8(Str::lit("utf-8")), List::<Bytes>::nil())))))))))
}

pub fn f171(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let m1278 = f167(v0.clone(), {
        let a1275 = Str::lit("--");
        utf8_encode(a1275)
    }, {
        let a1276 = v1;
        let a1277 = 4u64;
        nat_add(a1276, a1277)?
    })?;
    match m1278 {
        None => {
            Ok(f168(f108(Str::lit("a comment is not closed (XML 1.0 [15])"), v1)?))
        }
        Some(v2) => {
            let a1281 = f112(v0, {
                let a1279 = v2;
                let a1280 = 2u64;
                nat_add(a1279, a1280)?
            })?;
            let a1282 = 62u64;
            if nat_eq(a1281, a1282) {
                Ok(Ok::<u64, Bytes>({
                    let a1283 = v2;
                    let a1284 = 3u64;
                    nat_add(a1283, a1284)?
                }))
            } else {
                Ok(f168(f108(Str::lit("-- is not allowed inside a comment (XML 1.0 [15])"), v2)?))
            }
        }
    }
}

pub fn f172(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f174(v0.clone(), {
        let a1285 = v1;
        let a1286 = 2u64;
        nat_add(a1285, a1286)?
    })?;
    let a1289 = v2;
    let a1290 = {
        let a1287 = v1;
        let a1288 = 2u64;
        nat_add(a1287, a1288)?
    };
    if nat_eq(a1289, a1290) {
        Ok(f168(f108(Str::lit("a processing instruction must start with a Name (XML 1.0 [16])"), v1)?))
    } else {
        let a1293 = v2;
        let a1294 = {
            let a1291 = v1;
            let a1292 = 5u64;
            nat_add(a1291, a1292)?
        };
        if if nat_eq(a1293, a1294) {
            f207(f43(v0.clone(), {
                let a1295 = v1;
                let a1296 = 2u64;
                nat_add(a1295, a1296)?
            }, 3u64), List::cons(f8(Str::lit("XML")), List::cons(f8(Str::lit("XMl")), List::cons(f8(Str::lit("XmL")), List::cons(f8(Str::lit("Xml")), List::cons(f8(Str::lit("xML")), List::cons(f8(Str::lit("xMl")), List::cons(f8(Str::lit("xmL")), List::cons(f8(Str::lit("xml")), List::<Bytes>::nil())))))))))
        } else {
            false
        } {
            Ok(f168(f108(Str::lit("the target xml is reserved; an XML declaration may appear only at the very start (XML 1.0 [17], [22])"), v1)?))
        } else {
            let m1298 = f167(v0, {
                let a1297 = Str::lit("?>");
                utf8_encode(a1297)
            }, v2)?;
            match m1298 {
                None => {
                    Ok(f168(f108(Str::lit("a processing instruction is not closed (XML 1.0 [16])"), v1)?))
                }
                Some(v3) => {
                    Ok(Ok::<u64, Bytes>({
                        let a1299 = v3;
                        let a1300 = 2u64;
                        nat_add(a1299, a1300)?
                    }))
                }
            }
        }
    }
}

pub fn f173(v0: Bytes) -> Result<(Adt17, u64), Bytes> {
    Err::<(Adt17, u64), Bytes>(v0)
}

pub fn f174(v0: Bytes, v1: u64) -> R<u64> {
    let (h1301, _) = f164(Fn4::F255(v0.clone(), v1), {
        let a1303 = {
            let a1302 = v0;
            length_bytes(a1302)
        };
        let a1304 = 1u64;
        nat_add(a1303, a1304)?
    }, v1)?;
    Ok(h1301)
}

pub fn f175(v0: Bytes, v1: u64, v2: u64, v3: List<Bytes>) -> R<Result<(Adt17, u64), Bytes>> {
    let m1305 = v2;
    if m1305 == 0 {
        Ok(f173(f108(Str::lit("a tag has more attributes than the input has bytes"), v1)?))
    } else {
        let v4 = m1305 - 1;
        let v5: u64 = f169(v0.clone(), v1)?;
        let a1306 = f112(v0.clone(), v5)?;
        let a1307 = 62u64;
        if if nat_eq(a1306, a1307) {
            true
        } else {
            f111(v0.clone(), v5, Str::lit("/>"))
        } {
            Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C4, v5)))
        } else {
            let a1308 = v5;
            let a1309 = v1;
            if nat_eq(a1308, a1309) {
                Ok(f173(f108(Str::lit("attributes must be separated by white space (XML 1.0 [40])"), v1)?))
            } else {
                let v6: u64 = f174(v0.clone(), v5)?;
                let a1310 = v6;
                let a1311 = v5;
                if nat_eq(a1310, a1311) {
                    Ok(f173(f108(Str::lit("an attribute must start with a Name (XML 1.0 [41])"), v5)?))
                } else {
                    let v7: Bytes = f43(v0.clone(), v5, {
                        let a1312 = v6;
                        let a1313 = v5;
                        nat_sub(a1312, a1313)
                    });
                    if f207(v7.clone(), v3.clone()) {
                        Ok(f173(f108(Str::lit("an attribute name appears twice in one tag (XML 1.0 WFC: Unique Att Spec)"), v5)?))
                    } else {
                        let v8: u64 = f169(v0.clone(), v6)?;
                        let a1314 = f112(v0.clone(), v8)?;
                        let a1315 = 61u64;
                        if nat_eq(a1314, a1315) {
                            let v9: u64 = f169(v0.clone(), {
                                let a1316 = v8;
                                let a1317 = 1u64;
                                nat_add(a1316, a1317)?
                            })?;
                            let v10: u64 = f112(v0.clone(), v9)?;
                            let a1318 = v10;
                            let a1319 = 34u64;
                            if if nat_eq(a1318, a1319) {
                                true
                            } else {
                                let a1320 = v10;
                                let a1321 = 39u64;
                                nat_eq(a1320, a1321)
                            } {
                                let m1324 = f176(v0.clone(), {
                                    let a1322 = v9;
                                    let a1323 = 1u64;
                                    nat_add(a1322, a1323)?
                                }, v4, Bytes::lit(&[]), true, v10)?;
                                match m1324 {
                                    Ok(v12) => {
                                        let m1326 = f175(v0, {
                                            let (_, h1325) = v12;
                                            h1325
                                        }, v4, List::cons(v7.clone(), v3))?;
                                        match m1326 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C1(v7, {
                                                    let (h1327, _) = v12;
                                                    h1327
                                                }, std::rc::Rc::new({
                                                    let (h1328, _) = v14;
                                                    h1328
                                                })), {
                                                    let (_, h1329) = v14;
                                                    h1329
                                                })))
                                            }
                                            Err(v13) => {
                                                Ok(f173(v13))
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(f173(v11))
                                    }
                                }
                            } else {
                                Ok(f173(f108(Str::lit("an attribute value must be quoted (XML 1.0 [10])"), v9)?))
                            }
                        } else {
                            Ok(f173(f108(Str::lit("an attribute needs = and a value (XML 1.0 [25], [41])"), v8)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f176(v0: Bytes, p1369: u64, p1370: u64, p1371: Bytes, v4: bool, v5: u64) -> R<Result<(Bytes, u64), Bytes>> {
    let m1368 = (p1369, (p1370, p1371));
    {
        let mut m1368 = m1368;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1368;
                let m1330 = v2;
                if m1330 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f256(f108(Str::lit("the input ends inside a text run"), v1)?)))
                } else {
                    let v6 = m1330 - 1;
                    let v7: u64 = f112(v0.clone(), v1)?;
                    if if v4 {
                        let a1331 = v7;
                        let a1332 = v5;
                        nat_eq(a1331, a1332)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, {
                            let a1333 = v1;
                            let a1334 = 1u64;
                            nat_add(a1333, a1334)?
                        }))))
                    } else {
                        let a1335 = v7;
                        let a1336 = 256u64;
                        if nat_eq(a1335, a1336) {
                            if v4 {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f256(f108(Str::lit("an attribute value is not closed (XML 1.0 [10])"), v1)?)))
                            } else {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                            }
                        } else {
                            let a1337 = v7;
                            let a1338 = 60u64;
                            if nat_eq(a1337, a1338) {
                                if v4 {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f256(f108(Str::lit("< is not allowed in an attribute value (XML 1.0 [10], WFC: No < in Attribute Values)"), v1)?)))
                                } else {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                                }
                            } else {
                                let a1339 = v7;
                                let a1340 = 38u64;
                                if nat_eq(a1339, a1340) {
                                    let m1341 = f257(v0.clone(), v1)?;
                                    match m1341 {
                                        Ok(v9) => {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let (_, h1342) = v9;
                                                h1342
                                            }, (v6, {
                                                let a1344 = v3;
                                                let a1345 = {
                                                    let (h1343, _) = v9;
                                                    h1343
                                                };
                                                append_bytes(a1344, a1345)
                                            })))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f256(v8)))
                                        }
                                    }
                                } else {
                                    if v4 {
                                        let a1346 = v7;
                                        let a1347 = 9u64;
                                        if if nat_eq(a1346, a1347) {
                                            true
                                        } else {
                                            let a1348 = v7;
                                            let a1349 = 10u64;
                                            nat_eq(a1348, a1349)
                                        } {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let a1350 = v1;
                                                let a1351 = 1u64;
                                                nat_add(a1350, a1351)?
                                            }, (v6, {
                                                let a1352 = v3;
                                                let a1353 = f8(Str::lit(" "));
                                                append_bytes(a1352, a1353)
                                            })))
                                        } else {
                                            let v10: u64 = f258(v0.clone(), true, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v10, (v6, {
                                                let a1356 = v3;
                                                let a1357 = f43(v0.clone(), v1, {
                                                    let a1354 = v10;
                                                    let a1355 = v1;
                                                    nat_sub(a1354, a1355)
                                                });
                                                append_bytes(a1356, a1357)
                                            })))
                                        }
                                    } else {
                                        let a1358 = v7;
                                        let a1359 = 93u64;
                                        if nat_eq(a1358, a1359) {
                                            if f111(v0.clone(), v1, Str::lit("]]>")) {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f256(f108(Str::lit("]]> is not allowed in character data (XML 1.0 [14])"), v1)?)))
                                            } else {
                                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                    let a1360 = v1;
                                                    let a1361 = 1u64;
                                                    nat_add(a1360, a1361)?
                                                }, (v6, {
                                                    let a1362 = v3;
                                                    let a1363 = f8(Str::lit("]"));
                                                    append_bytes(a1362, a1363)
                                                })))
                                            }
                                        } else {
                                            let v11: u64 = f258(v0.clone(), false, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v11, (v6, {
                                                let a1366 = v3;
                                                let a1367 = f43(v0.clone(), v1, {
                                                    let a1364 = v11;
                                                    let a1365 = v1;
                                                    nat_sub(a1364, a1365)
                                                });
                                                append_bytes(a1366, a1367)
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
                Ok(next) => { m1368 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f177(v0: Bytes) -> Result<List<(Bytes, Bytes)>, Bytes> {
    Err::<List<(Bytes, Bytes)>, Bytes>(v0)
}

pub fn f178() -> Bytes {
    f8(Str::lit("http://www.w3.org/XML/1998/namespace"))
}

pub fn f179() -> Bytes {
    f8(Str::lit("http://www.w3.org/2000/xmlns/"))
}

pub fn f180(v0: Bytes, v1: Bytes) -> bool {
    let m1376 = {
        let a1373 = v0;
        let a1374 = 0u64;
        let a1375 = {
            let a1372 = v1.clone();
            length_bytes(a1372)
        };
        slice_bytes(a1373, a1374, a1375)
    };
    match m1376 {
        None => {
            false
        }
        Some(v2) => {
            let a1377 = v2;
            let a1378 = v1;
            equal(a1377, a1378)
        }
    }
}

pub fn f181(v0: Bytes) -> R<Bytes> {
    let m1379 = f167(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1379 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v1) => {
            Ok(f43(v0, 0u64, v1))
        }
    }
}

pub fn f182(p1384: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1383 = p1384;
    {
        let mut m1383 = m1383;
        loop {
            let step = {
                let v0 = m1383;
                let m1380 = v0;
                match m1380.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f46({
                            let (h1381, _) = v2;
                            h1381
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h1382) = v2;
                                h1382
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1383 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f183(v0: Bytes) -> R<Bytes> {
    let m1385 = f167(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1385 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(f43(v0.clone(), {
                let a1386 = v1;
                let a1387 = 1u64;
                nat_add(a1386, a1387)?
            }, {
                let a1391 = {
                    let a1388 = v0;
                    length_bytes(a1388)
                };
                let a1392 = {
                    let a1389 = v1;
                    let a1390 = 1u64;
                    nat_add(a1389, a1390)?
                };
                nat_sub(a1391, a1392)
            }))
        }
    }
}

pub fn f184() -> Bytes {
    f8(Str::lit("http://www.w3.org/2001/XMLSchema"))
}

pub fn f185(v0: Bytes) -> Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes> {
    Err::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v0)
}

pub fn f186(v0: Bytes, v1: Adt17, v2: List<(Bytes, Bytes)>, v3: Bytes) -> R<Result<Adt20, Bytes>> {
    let v4: Bytes = f187(v0.clone())?;
    let m1393 = f259(v1, v2)?;
    match m1393 {
        Ok(v6) => {
            let m1394 = v6;
            match m1394 {
                None => {
                    Ok(f260(f261(f8(Str::lit("a simpleType must have a restriction, list or union child (XSD 1.1 Part 2, section 4.1.2)")), v0)))
                }
                Some(v7) => {
                    let v8: Bytes = match v7.clone() {
                        Adt21::C0(h1395, _, _, _) => {
                            h1395
                        }
                    };
                    let v9: Adt17 = match v7.clone() {
                        Adt21::C0(_, h1396, _, _) => {
                            h1396
                        }
                    };
                    let v10: Adt17 = match v7.clone() {
                        Adt21::C0(_, _, h1397, _) => {
                            h1397
                        }
                    };
                    let v11: List<(Bytes, Bytes)> = match v7 {
                        Adt21::C0(_, _, _, h1398) => {
                            h1398
                        }
                    };
                    if f46(v8.clone(), f8(Str::lit("union"))) {
                        Ok(f260(f261(f8(Str::lit("a union is not translated, because a literal can be valid for more than one member type, so no LexLean inductive keeps its members disjoint (XSD 1.1 Part 2, section 2.4.1.3)")), v0)))
                    } else {
                        if f46(v8, f8(Str::lit("list"))) {
                            let m1399 = f118(v9, f8(Str::lit("itemType")));
                            match m1399 {
                                None => {
                                    Ok(f260(f261(f8(Str::lit("a list with an anonymous item type is not translated yet")), v0)))
                                }
                                Some(v12) => {
                                    let m1400 = f262(v11, v3, v12)?;
                                    match m1400 {
                                        Ok(v14) => {
                                            if f180(match v14.clone() {
                                                Adt20::C0(h1401, _) => {
                                                    h1401
                                                }
                                            }, f8(Str::lit("(List "))) {
                                                Ok(f260(f261(f8(Str::lit("the item type of a list must not itself be a list (XSD 1.1 Part 2, section 4.1.5)")), v0)))
                                            } else {
                                                Ok(Ok::<Adt20, Bytes>(Adt20::C0(f247(v4, f7(List::cons(f8(Str::lit("(List ")), List::cons(match v14.clone() {
                                                    Adt20::C0(h1402, _) => {
                                                        h1402
                                                    }
                                                }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))), match v14 {
                                                    Adt20::C0(_, h1403) => {
                                                        h1403
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
                            let m1404 = f118(v9, f8(Str::lit("base")));
                            match m1404 {
                                None => {
                                    Ok(f260(f261(f8(Str::lit("a restriction with an anonymous base type is not translated yet")), v0)))
                                }
                                Some(v15) => {
                                    let m1405 = f262(v11.clone(), v3, v15)?;
                                    match m1405 {
                                        Ok(v17) => {
                                            let m1406 = f263(v10, v11)?;
                                            match m1406 {
                                                Ok(v19) => {
                                                    let v20: List<Adt2> = f264(v19, List::<Bytes>::nil())?;
                                                    let m1407 = v20.clone();
                                                    match m1407.uncons() {
                                                        None => {
                                                            Ok(Ok::<Adt20, Bytes>(Adt20::C0(f247(v4, match v17.clone() {
                                                                Adt20::C0(h1408, _) => {
                                                                    h1408
                                                                }
                                                            }), match v17 {
                                                                Adt20::C0(_, h1409) => {
                                                                    h1409
                                                                }
                                                            })))
                                                        }
                                                        Some((_, _)) => {
                                                            let m1410 = f239(f234(f265(v20.clone()))?, v20, List::<Bytes>::nil())?;
                                                            match m1410 {
                                                                Ok(v24) => {
                                                                    Ok(Ok::<Adt20, Bytes>(Adt20::C0(f241(v4, v24), List::<Bytes>::nil())))
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

pub fn f187(v0: Bytes) -> R<Bytes> {
    f266(v0.clone(), 0u64, {
        let a1411 = v0;
        length_bytes(a1411)
    }, true, Bytes::lit(&[]))
}

pub fn f188(p1414: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m1413 = p1414;
    {
        let mut m1413 = m1413;
        loop {
            let step = {
                let v0 = m1413;
                let m1412 = v0;
                match m1412.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f149(v2.clone(), v1.clone()) {
                            Ok::<List<Bytes>, Option<Bytes>>(v3)
                        } else {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v2))
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

pub fn f189(v0: Bytes) -> Result<List<Bytes>, Bytes> {
    Err::<List<Bytes>, Bytes>(v0)
}

pub fn f190(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f267(v0, v1)
}

pub fn f191(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f268(v0, v1)
}

pub fn f192(v0: u64) -> bool {
    let a1415 = v0;
    let a1416 = 32u64;
    if nat_eq(a1415, a1416) {
        true
    } else {
        let a1417 = v0;
        let a1418 = 9u64;
        nat_eq(a1417, a1418)
    }
}

pub fn f193(v0: u64) -> bool {
    let a1419 = v0;
    let a1420 = 10u64;
    if nat_eq(a1419, a1420) {
        true
    } else {
        let a1421 = v0;
        let a1422 = 256u64;
        nat_eq(a1421, a1422)
    }
}

pub fn f194(v0: Bytes, v1: u64) -> R<u64> {
    let a1423 = v1;
    let a1424 = f269(v0, v1)?;
    Ok(nat_sub(a1423, a1424))
}

pub fn f195(v0: Bytes, v1: u64) -> R<u64> {
    let (h1425, _) = f164(Fn4::F270(v0.clone()), {
        let a1427 = {
            let a1426 = v0;
            length_bytes(a1426)
        };
        let a1428 = 1u64;
        nat_add(a1427, a1428)?
    }, v1)?;
    Ok(h1425)
}

pub fn f196(v0: Bytes, v1: u64) -> R<u64> {
    let (h1429, _) = f164(Fn4::F271(v0.clone()), {
        let a1431 = {
            let a1430 = v0;
            length_bytes(a1430)
        };
        let a1432 = 1u64;
        nat_add(a1431, a1432)?
    }, v1)?;
    Ok(h1429)
}

pub fn f197(v0: Bytes, v1: u64) -> R<bool> {
    let a1433 = f127(v0.clone(), v1)?;
    let a1434 = 45u64;
    if nat_eq(a1433, a1434) {
        if f192(f127(v0.clone(), {
            let a1435 = v1;
            let a1436 = 1u64;
            nat_add(a1435, a1436)?
        })?) {
            Ok(true)
        } else {
            Ok(f193(f127(v0, {
                let a1437 = v1;
                let a1438 = 1u64;
                nat_add(a1437, a1438)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f198(v0: Bytes, v1: u64) -> R<bool> {
    let m1439 = f205(v0, v1)?;
    match m1439 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f199(v0: Bytes, v1: u64) -> R<Option<Bytes>> {
    let v2: u64 = f127(v0.clone(), v1)?;
    let v3: bool = f204(f272(f127(v0, {
        let a1440 = v1;
        let a1441 = 1u64;
        nat_add(a1440, a1441)?
    })?, true));
    let a1442 = v2;
    let a1443 = 58u64;
    if if nat_eq(a1442, a1443) {
        v3
    } else {
        false
    } {
        Ok(Some(f108(Str::lit("a null key is not read: Studio.Value has no null (YAML 1.2.2 section 10.3.2)"), v1)?))
    } else {
        let a1444 = v2;
        let a1445 = 38u64;
        if nat_eq(a1444, a1445) {
            Ok(Some(f108(Str::lit("an anchor is not read (YAML 1.2.2 [101])"), v1)?))
        } else {
            let a1446 = v2;
            let a1447 = 42u64;
            if nat_eq(a1446, a1447) {
                Ok(Some(f108(Str::lit("an alias is not read (YAML 1.2.2 [104])"), v1)?))
            } else {
                let a1448 = v2;
                let a1449 = 33u64;
                if nat_eq(a1448, a1449) {
                    Ok(Some(f108(Str::lit("a tag is not read (YAML 1.2.2 [97])"), v1)?))
                } else {
                    let a1450 = v2;
                    let a1451 = 63u64;
                    if if nat_eq(a1450, a1451) {
                        v3
                    } else {
                        false
                    } {
                        Ok(Some(f108(Str::lit("an explicit key is not read (YAML 1.2.2 [143], [189])"), v1)?))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f200(v0: Bytes, v1: u64, v2: u64) -> R<Adt18> {
    let v3: u64 = f127(v0.clone(), v1)?;
    let a1452 = v3;
    let a1453 = 124u64;
    if if nat_eq(a1452, a1453) {
        true
    } else {
        let a1454 = v3;
        let a1455 = 62u64;
        nat_eq(a1454, a1455)
    } {
        f273(v0, v1, v2)
    } else {
        let a1456 = v3;
        let a1457 = 34u64;
        if if nat_eq(a1456, a1457) {
            true
        } else {
            let a1458 = v3;
            let a1459 = 39u64;
            nat_eq(a1458, a1459)
        } {
            let m1460 = f274(v0.clone(), v1, v2, false)?;
            match m1460 {
                Adt22::C0(v5, v6) => {
                    let m1461 = f208(v0, v6)?;
                    match m1461 {
                        Ok(v8) => {
                            Ok(Adt18::C0(Adt2::C3(v5), v8))
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
            let a1462 = v3;
            let a1463 = 91u64;
            if if nat_eq(a1462, a1463) {
                true
            } else {
                let a1464 = v3;
                let a1465 = 123u64;
                nat_eq(a1464, a1465)
            } {
                let m1469 = f275(v0.clone(), v1, v2, {
                    let a1467 = {
                        let a1466 = v0.clone();
                        length_bytes(a1466)
                    };
                    let a1468 = 2u64;
                    nat_add(a1467, a1468)?
                }, 0u64, List::<Bytes>::nil())?;
                match m1469 {
                    Adt18::C0(v10, v11) => {
                        let a1470 = f127(v0.clone(), f196(v0.clone(), v11)?)?;
                        let a1471 = 58u64;
                        if nat_eq(a1470, a1471) {
                            Ok(Adt18::C1(f108(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [154])"), v1)?))
                        } else {
                            let m1472 = f208(v0, v11)?;
                            match m1472 {
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
                let m1473 = f199(v0.clone(), v1)?;
                match m1473 {
                    None => {
                        if f204(f276(v0.clone(), v1, false)?) {
                            Ok(Adt18::C1(f108(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                        } else {
                            let v15: u64 = f277(v0.clone(), v1, false)?;
                            let m1477 = f278(v0.clone(), v15, v2, {
                                let a1475 = {
                                    let a1474 = v0.clone();
                                    length_bytes(a1474)
                                };
                                let a1476 = 2u64;
                                nat_add(a1475, a1476)?
                            }, f279(v0, v1, v15))?;
                            match m1477 {
                                Adt22::C0(v17, v18) => {
                                    f280(v17, v1, v18)
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

pub fn f201(v0: bool, v1: Adt18) -> Adt18 {
    let m1478 = v1;
    match m1478 {
        Adt18::C0(v3, v4) => {
            Adt18::C0(if v0 {
                Adt2::C4(std::rc::Rc::new(v3))
            } else {
                Adt2::C5(std::rc::Rc::new(v3))
            }, v4)
        }
        Adt18::C1(v2) => {
            Adt18::C1(v2)
        }
    }
}

pub fn f202(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f281(v0.clone(), v1)?;
    let a1480 = v2;
    let a1481 = {
        let a1479 = v0;
        length_bytes(a1479)
    };
    if nat_lt(a1480, a1481) {
        let a1482 = v2;
        let a1483 = 1u64;
        nat_add(a1482, a1483)
    } else {
        Ok(v2)
    }
}

pub fn f203(v0: u64) -> R<Bytes> {
    f108(Str::lit("a null node is not read: Studio.Value has no null (YAML 1.2.2 section 10.3.2)"), v0)
}

pub fn f204(v0: bool) -> bool {
    !v0
}

pub fn f205(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f127(v0.clone(), v1)?;
    let a1484 = v2;
    let a1485 = 34u64;
    if if nat_eq(a1484, a1485) {
        true
    } else {
        let a1486 = v2;
        let a1487 = 39u64;
        nat_eq(a1486, a1487)
    } {
        let m1488 = f274(v0.clone(), v1, 0u64, true)?;
        match m1488 {
            Adt22::C0(_, v5) => {
                let v6: u64 = f196(v0.clone(), v5)?;
                if f282(v0, v6)? {
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
        if f276(v0.clone(), v1, false)? {
            let v7: u64 = f196(v0.clone(), f277(v0.clone(), v1, false)?)?;
            if f282(v0, v7)? {
                Ok(Some(v7))
            } else {
                Ok(None::<u64>)
            }
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f206(v0: Bytes, v1: u64) -> R<Bytes> {
    let v2: u64 = f127(v0.clone(), v1)?;
    let a1489 = v2;
    let a1490 = 34u64;
    if if nat_eq(a1489, a1490) {
        true
    } else {
        let a1491 = v2;
        let a1492 = 39u64;
        nat_eq(a1491, a1492)
    } {
        let m1493 = f274(v0, v1, 0u64, true)?;
        match m1493 {
            Adt22::C0(v4, _) => {
                Ok(v4)
            }
            Adt22::C1(_) => {
                Ok(Bytes::lit(&[]))
            }
        }
    } else {
        Ok(f279(v0.clone(), v1, f277(v0, v1, false)?))
    }
}

pub fn f207(v0: Bytes, p1496: List<Bytes>) -> bool {
    let m1495 = p1496;
    {
        let mut m1495 = m1495;
        loop {
            let step = {
                let v1 = m1495;
                let m1494 = v1;
                match m1494.uncons() {
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
                Ok(next) => { m1495 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f208(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f196(v0.clone(), v1)?;
    let v3: u64 = f127(v0.clone(), v2)?;
    if if f193(v3) {
        true
    } else {
        let a1497 = v3;
        let a1498 = 35u64;
        if nat_eq(a1497, a1498) {
            let a1499 = v1;
            let a1500 = v2;
            nat_lt(a1499, a1500)
        } else {
            false
        }
    } {
        Ok(Ok::<u64, Bytes>(f202(v0, v2)?))
    } else {
        Ok(Err::<u64, Bytes>(f108(Str::lit("only white space and a comment may follow a node on its line (YAML 1.2.2 [77])"), v2)?))
    }
}

pub fn f209(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1501 = 0u64;
        let a1502 = v1;
        nat_lt(a1501, a1502)
    } else {
        false
    } {
        let a1503 = Bytes::lit(&[0x2d]);
        let a1504 = f64(v1)?;
        Ok(append_bytes(a1503, a1504))
    } else {
        f64(v1)
    }
}

pub fn f210(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f64(v1)?;
    let v4: u64 = {
        let a1505 = v3.clone();
        length_bytes(a1505)
    };
    let v5: Bytes = {
        let a1506 = v2;
        let a1507 = v4;
        if nat_lt(a1506, a1507) {
            f7(List::cons(f43(v3.clone(), 0u64, {
                let a1508 = v4;
                let a1509 = v2;
                nat_sub(a1508, a1509)
            }), List::cons(f8(Str::lit(".")), List::cons(f43(v3, {
                let a1510 = v4;
                let a1511 = v2;
                nat_sub(a1510, a1511)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f223({
                let a1512 = v2;
                let a1513 = v4;
                nat_sub(a1512, a1513)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1514 = f8(Str::lit("-"));
        let a1515 = v5;
        Ok(append_bytes(a1514, a1515))
    } else {
        Ok(v5)
    }
}

pub fn f211(v0: Bytes) -> R<Bytes> {
    let a1520 = f132(v0.clone(), 0u64, {
        let a1517 = {
            let a1516 = v0.clone();
            length_bytes(a1516)
        };
        let a1518 = 1u64;
        nat_add(a1517, a1518)?
    })?;
    let a1521 = {
        let a1519 = v0.clone();
        length_bytes(a1519)
    };
    if nat_eq(a1520, a1521) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        let a1527 = {
            let a1525 = Bytes::lit(&[0x22]);
            let a1526 = f283(v0.clone(), 0u64, {
                let a1523 = {
                    let a1522 = v0;
                    length_bytes(a1522)
                };
                let a1524 = 1u64;
                nat_add(a1523, a1524)?
            }, Bytes::lit(&[]))?;
            append_bytes(a1525, a1526)
        };
        let a1528 = Bytes::lit(&[0x22]);
        Ok(append_bytes(a1527, a1528))
    }
}

pub fn f212(v0: List<Bytes>) -> Bytes {
    let a1531 = {
        let a1529 = Bytes::lit(&[0x5b]);
        let a1530 = f44(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1529, a1530)
    };
    let a1532 = Bytes::lit(&[0x5d]);
    append_bytes(a1531, a1532)
}

pub fn f213(v0: Adt2) -> R<List<Bytes>> {
    let m1533 = v0;
    match m1533 {
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
        Adt2::C7(r1534, r1535) => {
            let v10 = (*r1534).clone();
            let v11 = (*r1535).clone();
            Ok(List::cons(f130(v10)?, f213(v11)?))
        }
        Adt2::C8(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f214(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f284(Fn6::F285, List::<(Bytes, Bytes)>::nil(), f286(v0)?);
    let v2: List<Bytes> = f287(Fn7::F288, List::<Bytes>::nil(), v1)?;
    let a1538 = {
        let a1536 = Bytes::lit(&[0x7b]);
        let a1537 = f44(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1536, a1537)
    };
    let a1539 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1538, a1539))
}

pub fn f215(v0: Adt2) -> R<List<(Bytes, Bytes)>> {
    let m1540 = v0;
    match m1540 {
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
        Adt2::C8(v12, r1541, r1542) => {
            let v13 = (*r1541).clone();
            let v14 = (*r1542).clone();
            Ok(List::cons((v12, f130(v13)?), f215(v14)?))
        }
    }
}

pub fn f216(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f80(v0.clone(), {
        let a1543 = v1;
        let a1544 = 1u64;
        nat_add(a1543, a1544)?
    })?;
    let v4: Bytes = {
        let a1545 = v3;
        let a1546 = 117u64;
        if nat_eq(a1545, a1546) {
            f289(v0, v1)?
        } else {
            f290(v3)
        }
    };
    let a1547 = v4.clone();
    let a1548 = Bytes::lit(&[]);
    if equal(a1547, a1548) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a1549 = v3;
            let a1550 = 117u64;
            if nat_eq(a1549, a1550) {
                let a1551 = v1;
                let a1552 = 6u64;
                nat_add(a1551, a1552)?
            } else {
                let a1553 = v1;
                let a1554 = 2u64;
                nat_add(a1553, a1554)?
            }
        }, ({
            let a1555 = v2;
            let a1556 = v4;
            append_bytes(a1555, a1556)
        }, 0u64))))
    }
}

pub fn f217(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f291(v0, v1)
}

pub fn f218(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f292(v0, v1)
}

pub fn f219(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1557 = v0.clone();
        length_bytes(a1557)
    };
    let a1558 = 0u64;
    let a1559 = v1;
    if nat_lt(a1558, a1559) {
        let a1560 = f293(v0.clone())?;
        let a1561 = v1;
        if nat_eq(a1560, a1561) {
            let a1562 = 1u64;
            let a1563 = v1;
            if nat_lt(a1562, a1563) {
                let m1566 = {
                    let a1564 = v0;
                    let a1565 = 0u64;
                    index_bytes(a1564, a1565)
                };
                match m1566 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1567 = v2;
                        let a1568 = 48u8;
                        Ok(!equal(a1567, a1568))
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

pub fn f220(v0: Fn5, p1573: u64, p1574: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1572 = (p1573, p1574);
    {
        let mut m1572 = m1572;
        loop {
            let step = {
                let (v1, v2) = m1572;
                let m1569 = v1;
                if m1569 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1569 - 1;
                    let m1571 = {
                        let c1570: Fn5 = v0.clone();
                        c1570.apply(v2)?
                    };
                    match m1571 {
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
                Ok(next) => { m1572 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f221(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f294(v0, v1)
}

pub fn f222(v0: Bytes) -> R<u64> {
    let (h1575, _) = f164(Fn4::F295(v0.clone()), {
        let a1577 = {
            let a1576 = v0;
            length_bytes(a1576)
        };
        let a1578 = 1u64;
        nat_add(a1577, a1578)?
    }, 0u64)?;
    Ok(h1575)
}

pub fn f223(v0: u64) -> Bytes {
    let m1579 = v0;
    if m1579 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1579 - 1;
        let a1580 = f8(Str::lit("0"));
        let a1581 = f223(v1);
        append_bytes(a1580, a1581)
    }
}

pub fn f224(p1636: u64, v1: Bytes, p1637: u64, p1638: List<Adt19>) -> R<Result<List<Adt19>, Bytes>> {
    let m1635 = (p1636, (p1637, p1638));
    {
        let mut m1635 = m1635;
        loop {
            let step = {
                let (v0, (v2, v3)) = m1635;
                let m1582 = v0;
                if m1582 == 0 {
                    Err::<(u64, (u64, List<Adt19>)), R<Result<List<Adt19>, Bytes>>>(Ok(Err::<List<Adt19>, Bytes>(f296(Str::lit("the input has more tokens than bytes"), v2)?)))
                } else {
                    let v4 = m1582 - 1;
                    let v5: u64 = f112(v1.clone(), v2)?;
                    let a1583 = v5;
                    let a1584 = 256u64;
                    if nat_eq(a1583, a1584) {
                        Err::<(u64, (u64, List<Adt19>)), R<Result<List<Adt19>, Bytes>>>(Ok(Ok::<List<Adt19>, Bytes>(f297(v3, List::<Adt19>::nil()))))
                    } else {
                        if f166(v5)? {
                            Ok::<(u64, (u64, List<Adt19>)), R<Result<List<Adt19>, Bytes>>>((v4, ({
                                let a1585 = v2;
                                let a1586 = 1u64;
                                nat_add(a1585, a1586)?
                            }, v3)))
                        } else {
                            if f111(v1.clone(), v2, Str::lit("//")) {
                                let m1587 = f167(v1.clone(), Bytes::lit(&[0x0a]), v2)?;
                                match m1587 {
                                    None => {
                                        Err::<(u64, (u64, List<Adt19>)), R<Result<List<Adt19>, Bytes>>>(Ok(Ok::<List<Adt19>, Bytes>(f297(v3, List::<Adt19>::nil()))))
                                    }
                                    Some(v6) => {
                                        Ok::<(u64, (u64, List<Adt19>)), R<Result<List<Adt19>, Bytes>>>((v4, ({
                                            let a1588 = v6;
                                            let a1589 = 1u64;
                                            nat_add(a1588, a1589)?
                                        }, v3)))
                                    }
                                }
                            } else {
                                if f111(v1.clone(), v2, Str::lit("/*")) {
                                    let m1593 = f167(v1.clone(), {
                                        let a1590 = Str::lit("*/");
                                        utf8_encode(a1590)
                                    }, {
                                        let a1591 = v2;
                                        let a1592 = 2u64;
                                        nat_add(a1591, a1592)?
                                    })?;
                                    match m1593 {
                                        None => {
                                            Err::<(u64, (u64, List<Adt19>)), R<Result<List<Adt19>, Bytes>>>(Ok(Err::<List<Adt19>, Bytes>(f296(Str::lit("a comment is not closed (Web IDL, IDL grammar: comment)"), v2)?)))
                                        }
                                        Some(v7) => {
                                            Ok::<(u64, (u64, List<Adt19>)), R<Result<List<Adt19>, Bytes>>>((v4, ({
                                                let a1594 = v7;
                                                let a1595 = 2u64;
                                                nat_add(a1594, a1595)?
                                            }, v3)))
                                        }
                                    }
                                } else {
                                    let a1596 = v5;
                                    let a1597 = 34u64;
                                    if nat_eq(a1596, a1597) {
                                        let m1600 = f167(v1.clone(), Bytes::lit(&[0x22]), {
                                            let a1598 = v2;
                                            let a1599 = 1u64;
                                            nat_add(a1598, a1599)?
                                        })?;
                                        match m1600 {
                                            None => {
                                                Err::<(u64, (u64, List<Adt19>)), R<Result<List<Adt19>, Bytes>>>(Ok(Err::<List<Adt19>, Bytes>(f296(Str::lit("a string is not closed (Web IDL, IDL grammar: string)"), v2)?)))
                                            }
                                            Some(v8) => {
                                                Ok::<(u64, (u64, List<Adt19>)), R<Result<List<Adt19>, Bytes>>>((v4, ({
                                                    let a1601 = v8;
                                                    let a1602 = 1u64;
                                                    nat_add(a1601, a1602)?
                                                }, List::cons(Adt19::C0(3u64, f43(v1.clone(), {
                                                    let a1603 = v2;
                                                    let a1604 = 1u64;
                                                    nat_add(a1603, a1604)?
                                                }, {
                                                    let a1607 = v8;
                                                    let a1608 = {
                                                        let a1605 = v2;
                                                        let a1606 = 1u64;
                                                        nat_add(a1605, a1606)?
                                                    };
                                                    nat_sub(a1607, a1608)
                                                })), v3))))
                                            }
                                        }
                                    } else {
                                        let v9: u64 = {
                                            let a1609 = v5;
                                            let a1610 = 95u64;
                                            if if nat_eq(a1609, a1610) {
                                                true
                                            } else {
                                                let a1611 = v5;
                                                let a1612 = 45u64;
                                                nat_eq(a1611, a1612)
                                            } {
                                                let a1613 = v2;
                                                let a1614 = 1u64;
                                                nat_add(a1613, a1614)?
                                            } else {
                                                v2
                                            }
                                        };
                                        if f298(f112(v1.clone(), v9)?) {
                                            let v10: u64 = f299(v1.clone(), 3u64, {
                                                let a1615 = v9;
                                                let a1616 = 1u64;
                                                nat_add(a1615, a1616)?
                                            })?;
                                            Ok::<(u64, (u64, List<Adt19>)), R<Result<List<Adt19>, Bytes>>>((v4, (v10, List::cons(Adt19::C0(0u64, f43(v1.clone(), v2, {
                                                let a1617 = v10;
                                                let a1618 = v2;
                                                nat_sub(a1617, a1618)
                                            })), v3))))
                                        } else {
                                            let v11: u64 = {
                                                let a1619 = v5;
                                                let a1620 = 45u64;
                                                if nat_eq(a1619, a1620) {
                                                    let a1621 = v2;
                                                    let a1622 = 1u64;
                                                    nat_add(a1621, a1622)?
                                                } else {
                                                    v2
                                                }
                                            };
                                            let v12: u64 = f300(v1.clone(), v11)?;
                                            let v13: u64 = f301(v1.clone(), v11)?;
                                            let a1623 = v12;
                                            let a1624 = v13;
                                            if nat_lt(a1623, a1624) {
                                                Ok::<(u64, (u64, List<Adt19>)), R<Result<List<Adt19>, Bytes>>>((v4, (v13, List::cons(Adt19::C0(2u64, f43(v1.clone(), v2, {
                                                    let a1625 = v13;
                                                    let a1626 = v2;
                                                    nat_sub(a1625, a1626)
                                                })), v3))))
                                            } else {
                                                let a1627 = v11;
                                                let a1628 = v12;
                                                if nat_lt(a1627, a1628) {
                                                    Ok::<(u64, (u64, List<Adt19>)), R<Result<List<Adt19>, Bytes>>>((v4, (v12, List::cons(Adt19::C0(1u64, f43(v1.clone(), v2, {
                                                        let a1629 = v12;
                                                        let a1630 = v2;
                                                        nat_sub(a1629, a1630)
                                                    })), v3))))
                                                } else {
                                                    if f111(v1.clone(), v2, Str::lit("...")) {
                                                        Ok::<(u64, (u64, List<Adt19>)), R<Result<List<Adt19>, Bytes>>>((v4, ({
                                                            let a1631 = v2;
                                                            let a1632 = 3u64;
                                                            nat_add(a1631, a1632)?
                                                        }, List::cons(Adt19::C0(4u64, f8(Str::lit("..."))), v3))))
                                                    } else {
                                                        Ok::<(u64, (u64, List<Adt19>)), R<Result<List<Adt19>, Bytes>>>((v4, ({
                                                            let a1633 = v2;
                                                            let a1634 = 1u64;
                                                            nat_add(a1633, a1634)?
                                                        }, List::cons(Adt19::C0(4u64, f43(v1.clone(), v2, 1u64)), v3))))
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
                Ok(next) => { m1635 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f225(p1641: List<Adt13>, p1642: List<Adt13>) -> List<Adt13> {
    let m1640 = (p1641, p1642);
    {
        let mut m1640 = m1640;
        loop {
            let step = {
                let (v0, v1) = m1640;
                let m1639 = v0;
                match m1639.uncons() {
                    None => {
                        Err::<(List<Adt13>, List<Adt13>), List<Adt13>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt13>, List<Adt13>), List<Adt13>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1640 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f226(v0: List<Adt19>) -> R<Result<List<Adt19>, Bytes>> {
    if f302(v0.clone(), Str::lit("[")) {
        Ok(f303({
            let a1644 = {
                let a1643 = v0.clone();
                length_list(a1643)
            };
            let a1645 = 1u64;
            nat_add(a1644, a1645)?
        }, f304(v0), List::cons(f8(Str::lit("]")), List::<Bytes>::nil()), false))
    } else {
        Ok(Ok::<List<Adt19>, Bytes>(v0))
    }
}

pub fn f227(v0: u64, v1: List<Adt19>) -> R<Result<(Adt13, List<Adt19>), Bytes>> {
    if f302(v1.clone(), Str::lit("interface")) {
        if f302(f304(v1.clone()), Str::lit("mixin")) {
            f305(v0, 1u64, f304(f304(v1)), false)
        } else {
            f305(v0, 0u64, f304(v1), true)
        }
    } else {
        if f302(v1.clone(), Str::lit("partial")) {
            let v2: List<Adt19> = f304(v1);
            if f302(v2.clone(), Str::lit("interface")) {
                if f302(f304(v2.clone()), Str::lit("mixin")) {
                    f305(v0, 6u64, f304(f304(v2)), false)
                } else {
                    f305(v0, 5u64, f304(v2), false)
                }
            } else {
                if f302(v2.clone(), Str::lit("dictionary")) {
                    f305(v0, 7u64, f304(v2), false)
                } else {
                    if f302(v2.clone(), Str::lit("namespace")) {
                        f306(f304(v2), f8(Str::lit("a namespace holds operations and attributes of no object, so it is behavior, not data (Web IDL 2.6)")))
                    } else {
                        Ok(Err::<(Adt13, List<Adt19>), Bytes>(f307(Str::lit("interface, dictionary or namespace"), v2)))
                    }
                }
            }
        } else {
            if f302(v1.clone(), Str::lit("dictionary")) {
                f305(v0, 2u64, f304(v1), true)
            } else {
                if f302(v1.clone(), Str::lit("enum")) {
                    let m1646 = f308(f304(v1));
                    match m1646 {
                        Ok(v4) => {
                            let m1647 = f309(f310(v4.clone()), Str::lit("{"));
                            match m1647 {
                                Ok(v6) => {
                                    let m1648 = f311(v0, v6, List::<Bytes>::nil());
                                    match m1648 {
                                        Ok(v8) => {
                                            Ok(Ok::<(Adt13, List<Adt19>), Bytes>((Adt13::C0(3u64, f312(v4), Bytes::lit(&[]), List::<Adt14>::nil(), {
                                                let (h1649, _) = v8;
                                                h1649
                                            }, f313(Bytes::lit(&[]))), {
                                                let (_, h1650) = v8;
                                                h1650
                                            })))
                                        }
                                        Err(v7) => {
                                            Ok(Err::<(Adt13, List<Adt19>), Bytes>(v7))
                                        }
                                    }
                                }
                                Err(v5) => {
                                    Ok(Err::<(Adt13, List<Adt19>), Bytes>(v5))
                                }
                            }
                        }
                        Err(v3) => {
                            Ok(Err::<(Adt13, List<Adt19>), Bytes>(v3))
                        }
                    }
                } else {
                    if f302(v1.clone(), Str::lit("typedef")) {
                        let m1651 = f314(v0, f304(v1), 0u64)?;
                        match m1651 {
                            Ok(v10) => {
                                let m1652 = f308(f315(v10.clone()));
                                match m1652 {
                                    Ok(v12) => {
                                        let m1653 = f309(f310(v12.clone()), Str::lit(";"));
                                        match m1653 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt13, List<Adt19>), Bytes>((Adt13::C0(4u64, f312(v12), Bytes::lit(&[]), List::<Adt14>::nil(), List::<Bytes>::nil(), f316(v10)), v14)))
                                            }
                                            Err(v13) => {
                                                Ok(Err::<(Adt13, List<Adt19>), Bytes>(v13))
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(Err::<(Adt13, List<Adt19>), Bytes>(v11))
                                    }
                                }
                            }
                            Err(v9) => {
                                Ok(Err::<(Adt13, List<Adt19>), Bytes>(v9))
                            }
                        }
                    } else {
                        if f302(v1.clone(), Str::lit("callback")) {
                            if f302(f304(v1.clone()), Str::lit("interface")) {
                                f306(f304(f304(v1)), f8(Str::lit("a callback interface is implemented by user code, so it is behavior, not data (Web IDL 2.4)")))
                            } else {
                                f306(f304(v1), f8(Str::lit("a callback function is behavior, not data (Web IDL 2.10)")))
                            }
                        } else {
                            if f302(v1.clone(), Str::lit("namespace")) {
                                f306(f304(v1), f8(Str::lit("a namespace holds operations and attributes of no object, so it is behavior, not data (Web IDL 2.6)")))
                            } else {
                                let m1654 = f308(v1);
                                match m1654 {
                                    Ok(v16) => {
                                        let m1655 = f309(f310(v16.clone()), Str::lit("includes"));
                                        match m1655 {
                                            Ok(v18) => {
                                                let m1656 = f308(v18);
                                                match m1656 {
                                                    Ok(v20) => {
                                                        let m1657 = f309(f310(v20.clone()), Str::lit(";"));
                                                        match m1657 {
                                                            Ok(v22) => {
                                                                Ok(Ok::<(Adt13, List<Adt19>), Bytes>((Adt13::C0(8u64, f312(v16), f312(v20), List::<Adt14>::nil(), List::<Bytes>::nil(), f313(Bytes::lit(&[]))), v22)))
                                                            }
                                                            Err(v21) => {
                                                                Ok(Err::<(Adt13, List<Adt19>), Bytes>(v21))
                                                            }
                                                        }
                                                    }
                                                    Err(v19) => {
                                                        Ok(Err::<(Adt13, List<Adt19>), Bytes>(v19))
                                                    }
                                                }
                                            }
                                            Err(v17) => {
                                                Ok(Err::<(Adt13, List<Adt19>), Bytes>(v17))
                                            }
                                        }
                                    }
                                    Err(v15) => {
                                        Ok(Err::<(Adt13, List<Adt19>), Bytes>(v15))
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

pub fn f228(p1661: List<Bytes>, v1: List<Adt16>) -> Bytes {
    let m1660 = p1661;
    {
        let mut m1660 = m1660;
        loop {
            let step = {
                let v0 = m1660;
                let m1658 = v0;
                match m1658.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1659 = f150(v1.clone(), v2.clone());
                        match m1659 {
                            None => {
                                Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("refers to ")), List::cons(v2, List::cons(f8(Str::lit(", which no input defines (Web IDL 2.13.22 to 2.13.26)")), List::<Bytes>::nil())))))
                            }
                            Some(v4) => {
                                if f46(f151(v4.clone()), Bytes::lit(&[])) {
                                    Ok::<List<Bytes>, Bytes>(v3)
                                } else {
                                    Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("refers to ")), List::cons(v2, List::cons(f8(Str::lit(", which is not translated: ")), List::cons(f151(v4), List::<Bytes>::nil()))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1660 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f229(v0: Bytes) -> bool {
    if f149(v0.clone(), f317()) {
        true
    } else {
        if f149(v0.clone(), f318()) {
            true
        } else {
            if f149(v0.clone(), f319()) {
                true
            } else {
                if f149(v0.clone(), f320()) {
                    true
                } else {
                    if f149(v0.clone(), f321()) {
                        true
                    } else {
                        if f149(v0.clone(), f322()) {
                            true
                        } else {
                            f149(v0, f323())
                        }
                    }
                }
            }
        }
    }
}

pub fn f230(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("the derived name ")), List::cons(v0, List::cons(f8(Str::lit(" is a reserved token of Lean 4.32.2, which LexLean 0.4.3 refuses as a name (lexlean 0.4.3 src/backend/lean_tokens.rs, SPEC.md 17.8)")), List::<Bytes>::nil()))))
}

pub fn f231(p1667: List<Adt13>, v1: List<Adt13>, v2: Bytes) -> Option<Bytes> {
    let m1666 = p1667;
    {
        let mut m1666 = m1666;
        loop {
            let step = {
                let v0 = m1666;
                let m1662 = v0;
                match m1662.uncons() {
                    None => {
                        Err::<List<Adt13>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v3, v4)) => {
                        let a1663 = f146(v3.clone());
                        let a1664 = 8u64;
                        if if nat_eq(a1663, a1664) {
                            f46(f148(v3.clone()), v2.clone())
                        } else {
                            false
                        } {
                            let m1665 = f157(v1.clone(), 1u64, f159(v3.clone()));
                            match m1665 {
                                None => {
                                    Err::<List<Adt13>, Option<Bytes>>(Some(f159(v3)))
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
                Ok(next) => { m1666 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f232(v0: u64, v1: List<Adt13>, v2: Adt13) -> R<Result<List<Adt14>, Bytes>> {
    let m1668 = v0;
    if m1668 == 0 {
        Ok(Err::<List<Adt14>, Bytes>(f7(List::cons(f8(Str::lit("the inheritance of ")), List::cons(f148(v2), List::cons(f8(Str::lit(" is cyclic (Web IDL 2.2 and 2.7: an interface or dictionary must not inherit from itself)")), List::<Bytes>::nil()))))))
    } else {
        let v3 = m1668 - 1;
        let v4: List<Adt14> = {
            let a1669 = f146(v2.clone());
            let a1670 = 2u64;
            if nat_eq(a1669, a1670) {
                f324(f325(f326(v2.clone()), f327(v1.clone(), 7u64, f148(v2.clone()))))?
            } else {
                f325(f326(v2.clone()), f325(f327(v1.clone(), 5u64, f148(v2.clone())), f328(v1.clone(), v1.clone(), f148(v2.clone()))))
            }
        };
        if f46(f159(v2.clone()), Bytes::lit(&[])) {
            Ok(Ok::<List<Adt14>, Bytes>(v4))
        } else {
            let m1671 = f157(v1.clone(), f146(v2.clone()), f159(v2.clone()));
            match m1671 {
                None => {
                    Ok(Err::<List<Adt14>, Bytes>(f7(List::cons(f8(Str::lit("inherits ")), List::cons(f159(v2.clone()), List::cons({
                        let a1672 = f146(v2);
                        let a1673 = 2u64;
                        if nat_eq(a1672, a1673) {
                            f8(Str::lit(", which no input defines as a dictionary (Web IDL 2.7)"))
                        } else {
                            f8(Str::lit(", which no input defines as an interface (Web IDL 2.2)"))
                        }
                    }, List::<Bytes>::nil()))))))
                }
                Some(v5) => {
                    let m1674 = f232(v3, v1, v5)?;
                    match m1674 {
                        Ok(v7) => {
                            Ok(Ok::<List<Adt14>, Bytes>(f325(v7, v4)))
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

pub fn f233(v0: bool, v1: List<Adt14>) -> R<Result<(List<Bytes>, List<Bytes>), Bytes>> {
    let m1675 = v1;
    match m1675.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::<Bytes>::nil(), List::<Bytes>::nil())))
        }
        Some((v2, v3)) => {
            let v4: Adt15 = f329(v2.clone());
            if !f46(f243(v4.clone()), Bytes::lit(&[])) {
                Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons(f330(v2), List::cons(f8(Str::lit(": ")), List::cons(f243(v4), List::<Bytes>::nil())))))))
            } else {
                let m1676 = f331(v0, f330(v2.clone()))?;
                match m1676 {
                    None => {
                        Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons(f330(v2), List::cons(f8(Str::lit(" has no field name")), List::<Bytes>::nil()))))))
                    }
                    Some(v5) => {
                        if f229(v5.clone()) {
                            Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f230(v5)))
                        } else {
                            if f149(v5.clone(), f332(v0, f235(v3.clone()))?) {
                                Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("two of its members derive the field name ")), List::cons(v5, List::cons(f8(Str::lit(" (Web IDL 2.2 and 2.7: the members of one definition have distinct identifiers)")), List::<Bytes>::nil()))))))
                            } else {
                                let m1677 = f233(v0, v3)?;
                                match m1677 {
                                    Ok(v7) => {
                                        Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v5, List::cons(f8(Str::lit(", type: ")), List::cons({
                                            let a1678 = f333(v2);
                                            let a1679 = 1u64;
                                            if nat_eq(a1678, a1679) {
                                                f7(List::cons(f8(Str::lit("(Option ")), List::cons(f245(v4.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                            } else {
                                                f245(v4.clone())
                                            }
                                        }, List::cons(f8(Str::lit("}")), List::<Bytes>::nil())))))), {
                                            let (h1680, _) = v7;
                                            h1680
                                        }), f152(f246(v4), {
                                            let (_, h1681) = v7;
                                            h1681
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

pub fn f234(p1685: List<Bytes>) -> R<bool> {
    let m1684 = p1685;
    {
        let mut m1684 = m1684;
        loop {
            let step = {
                let v0 = m1684;
                let m1682 = v0;
                match m1682.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m1683 = f334(v1)?;
                        match m1683 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f149(v3, f332(false, v2.clone())?) {
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
                Ok(next) => { m1684 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f235(v0: List<Adt14>) -> List<Bytes> {
    let m1686 = v0;
    match m1686.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f330(v1), f235(v2))
        }
    }
}

pub fn f236(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}")), List::cons(f100(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f237(v0: Adt13) -> List<Bytes> {
    let m1687 = v0;
    match m1687 {
        Adt13::C0(_, _, _, _, v5, _) => {
            v5
        }
    }
}

pub fn f238(v0: bool, p1691: List<Bytes>) -> R<Option<Bytes>> {
    let m1690 = p1691;
    {
        let mut m1690 = m1690;
        loop {
            let step = {
                let v1 = m1690;
                let m1688 = v1;
                match m1688.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m1689 = f331(v0, v2)?;
                        match m1689 {
                            None => {
                                Ok::<List<Bytes>, R<Option<Bytes>>>(v3)
                            }
                            Some(v4) => {
                                if f229(v4.clone()) {
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
                Ok(next) => { m1690 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f239(v0: bool, v1: List<Adt2>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1692 = v1;
    match m1692.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m1693 = f335(v0, v3.clone())?;
            match m1693 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f70(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f149(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m1694 = f239(v0, v4, List::cons(v5.clone(), v2))?;
                        match m1694 {
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

pub fn f240(v0: List<Bytes>) -> List<Adt2> {
    let m1695 = v0;
    match m1695.uncons() {
        None => {
            List::<Adt2>::nil()
        }
        Some((v1, v2)) => {
            List::cons(Adt2::C3(v1), f240(v2))
        }
    }
}

pub fn f241(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f100(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f242(v0: Adt13) -> Adt15 {
    let m1696 = v0;
    match m1696 {
        Adt13::C0(_, _, _, _, _, v6) => {
            v6
        }
    }
}

pub fn f243(v0: Adt15) -> Bytes {
    let m1697 = v0;
    match m1697 {
        Adt15::C0(_, _, _, v4, _) => {
            v4
        }
    }
}

pub fn f244(v0: Adt15) -> u64 {
    let m1698 = v0;
    match m1698 {
        Adt15::C0(v1, _, _, _, _) => {
            v1
        }
    }
}

pub fn f245(v0: Adt15) -> Bytes {
    let m1699 = v0;
    match m1699 {
        Adt15::C0(_, v2, _, _, _) => {
            v2
        }
    }
}

pub fn f246(v0: Adt15) -> List<Bytes> {
    let m1700 = v0;
    match m1700 {
        Adt15::C0(_, _, v3, _, _) => {
            v3
        }
    }
}

pub fn f247(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f248(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h1701, _) = v2;
        h1701
    };
    let a1702 = v3;
    let a1703 = 0u64;
    if nat_eq(a1702, a1703) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a1704 = v3;
            let a1705 = v1;
            let a1706 = 0u64;
            nat_quot(a1704, a1705, a1706)
        }, {
            let a1711 = f160(v0, {
                let a1707 = v3;
                let a1708 = v1;
                let a1709 = 0u64;
                nat_rem(a1707, a1708, a1709)
            });
            let a1712 = {
                let (_, h1710) = v2;
                h1710
            };
            append_bytes(a1711, a1712)
        }))
    }
}

pub fn f249(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h1713, _) = v1;
        h1713
    };
    let a1715 = {
        let a1714 = v0.clone();
        length_bytes(a1714)
    };
    let a1716 = v2;
    if nat_le(a1715, a1716) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let a1717 = f112(v0.clone(), v2)?;
        let a1718 = 13u64;
        if nat_eq(a1717, a1718) {
            Ok(Some(({
                let a1721 = f112(v0, {
                    let a1719 = v2;
                    let a1720 = 1u64;
                    nat_add(a1719, a1720)?
                })?;
                let a1722 = 10u64;
                if nat_eq(a1721, a1722) {
                    let a1723 = v2;
                    let a1724 = 2u64;
                    nat_add(a1723, a1724)?
                } else {
                    let a1725 = v2;
                    let a1726 = 1u64;
                    nat_add(a1725, a1726)?
                }
            }, {
                let a1728 = {
                    let (_, h1727) = v1;
                    h1727
                };
                let a1729 = f8(Str::lit("\u{a}"));
                append_bytes(a1728, a1729)
            })))
        } else {
            let v4: u64 = {
                let m1730 = f167(v0.clone(), f8(Str::lit("\u{d}")), v2)?;
                match m1730 {
                    None => {
                        let a1731 = v0.clone();
                        length_bytes(a1731)
                    }
                    Some(v3) => {
                        v3
                    }
                }
            };
            Ok(Some((v4, {
                let a1735 = {
                    let (_, h1732) = v1;
                    h1732
                };
                let a1736 = f43(v0, v2, {
                    let a1733 = v4;
                    let a1734 = v2;
                    nat_sub(a1733, a1734)
                });
                append_bytes(a1735, a1736)
            })))
        }
    }
}

pub fn f250(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1737 = f336(v0, v1)?;
    match m1737 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f337({
                let (h1738, _) = v2;
                h1738
            })? {
                Ok(Some({
                    let a1740 = v1;
                    let a1741 = {
                        let (_, h1739) = v2;
                        h1739
                    };
                    nat_add(a1740, a1741)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f251(v0: u64, v1: Bytes) -> R<bool> {
    f338(v0, v1.clone(), 0u64, {
        let a1743 = {
            let a1742 = v1;
            length_bytes(a1742)
        };
        let a1744 = 3u64;
        let a1745 = 0u64;
        nat_quot(a1743, a1744, a1745)
    }, true)
}

pub fn f252() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{0}\u{0} "))
}

pub fn f253(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f339(v1, v0, v2)
}

pub fn f254(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f340(v0, v1)
}

pub fn f255(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f341(v0, v1, v2)
}

pub fn f256(v0: Bytes) -> Result<(Bytes, u64), Bytes> {
    Err::<(Bytes, u64), Bytes>(v0)
}

pub fn f257(v0: Bytes, v1: u64) -> R<Result<(Bytes, u64), Bytes>> {
    if f111(v0.clone(), v1, Str::lit("&#")) {
        let v2: bool = {
            let a1748 = f112(v0.clone(), {
                let a1746 = v1;
                let a1747 = 2u64;
                nat_add(a1746, a1747)?
            })?;
            let a1749 = 120u64;
            nat_eq(a1748, a1749)
        };
        let v3: u64 = if v2 {
            let a1750 = v1;
            let a1751 = 3u64;
            nat_add(a1750, a1751)?
        } else {
            let a1752 = v1;
            let a1753 = 2u64;
            nat_add(a1752, a1753)?
        };
        let v4: (u64, u64) = f342(v0.clone(), v2, v3)?;
        let a1755 = {
            let (h1754, _) = v4;
            h1754
        };
        let a1756 = v3;
        if if nat_eq(a1755, a1756) {
            true
        } else {
            let a1758 = f112(v0, {
                let (h1757, _) = v4;
                h1757
            })?;
            let a1759 = 59u64;
            !nat_eq(a1758, a1759)
        } {
            Ok(f256(f108(Str::lit("a character reference must be &#digits; or &#xhex; (XML 1.0 [66])"), v1)?))
        } else {
            if f337({
                let (_, h1760) = v4;
                h1760
            })? {
                Ok(Ok::<(Bytes, u64), Bytes>((f343({
                    let (_, h1761) = v4;
                    h1761
                })?, {
                    let a1763 = {
                        let (h1762, _) = v4;
                        h1762
                    };
                    let a1764 = 1u64;
                    nat_add(a1763, a1764)?
                })))
            } else {
                Ok(f256(f108(Str::lit("a character reference must name a Char (XML 1.0 [66] WFC: Legal Character)"), v1)?))
            }
        }
    } else {
        if f111(v0.clone(), v1, Str::lit("&lt;")) {
            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("<")), {
                let a1765 = v1;
                let a1766 = 4u64;
                nat_add(a1765, a1766)?
            })))
        } else {
            if f111(v0.clone(), v1, Str::lit("&gt;")) {
                Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit(">")), {
                    let a1767 = v1;
                    let a1768 = 4u64;
                    nat_add(a1767, a1768)?
                })))
            } else {
                if f111(v0.clone(), v1, Str::lit("&amp;")) {
                    Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("&")), {
                        let a1769 = v1;
                        let a1770 = 5u64;
                        nat_add(a1769, a1770)?
                    })))
                } else {
                    if f111(v0.clone(), v1, Str::lit("&apos;")) {
                        Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("'")), {
                            let a1771 = v1;
                            let a1772 = 6u64;
                            nat_add(a1771, a1772)?
                        })))
                    } else {
                        if f111(v0, v1, Str::lit("&quot;")) {
                            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("\"")), {
                                let a1773 = v1;
                                let a1774 = 6u64;
                                nat_add(a1773, a1774)?
                            })))
                        } else {
                            Ok(f256(f108(Str::lit("an entity reference names an entity that is not declared, and only the five predefined entities are (XML 1.0 section 4.1, WFC: Entity Declared)"), v1)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f258(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<u64> {
    let (h1775, _) = f164(Fn4::F344(v1, v2, v0.clone()), {
        let a1777 = {
            let a1776 = v0;
            length_bytes(a1776)
        };
        let a1778 = 1u64;
        nat_add(a1777, a1778)?
    }, v3)?;
    Ok(h1775)
}

pub fn f259(p1791: Adt17, v1: List<(Bytes, Bytes)>) -> R<Result<Option<Adt21>, Bytes>> {
    let m1790 = p1791;
    {
        let mut m1790 = m1790;
        loop {
            let step = {
                let v0 = m1790;
                let m1779 = v0;
                match m1779 {
                    Adt17::C0(_, _, _) => {
                        Err::<Adt17, R<Result<Option<Adt21>, Bytes>>>(Ok(Ok::<Option<Adt21>, Bytes>(None::<Adt21>)))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<Adt17, R<Result<Option<Adt21>, Bytes>>>(Ok(Ok::<Option<Adt21>, Bytes>(None::<Adt21>)))
                    }
                    Adt17::C2(_) => {
                        Err::<Adt17, R<Result<Option<Adt21>, Bytes>>>(Ok(Ok::<Option<Adt21>, Bytes>(None::<Adt21>)))
                    }
                    Adt17::C3(r1788, r1789) => {
                        let v2 = (*r1788).clone();
                        let v3 = (*r1789).clone();
                        let m1780 = v2;
                        match m1780 {
                            Adt17::C0(v4, r1786, r1787) => {
                                let v5 = (*r1786).clone();
                                let v6 = (*r1787).clone();
                                let m1781 = f115(v5.clone(), v1.clone());
                                match m1781 {
                                    Ok(v8) => {
                                        let m1782 = f116(v8.clone(), v4)?;
                                        match m1782 {
                                            Ok(v10) => {
                                                if f207({
                                                    let (_, h1783) = v10.clone();
                                                    h1783
                                                }, List::cons(f8(Str::lit("restriction")), List::cons(f8(Str::lit("list")), List::cons(f8(Str::lit("union")), List::<Bytes>::nil())))) {
                                                    if f46({
                                                        let (h1784, _) = v10;
                                                        h1784
                                                    }, f184()) {
                                                        Err::<Adt17, R<Result<Option<Adt21>, Bytes>>>(Ok(Ok::<Option<Adt21>, Bytes>(Some(Adt21::C0({
                                                            let (_, h1785) = v10;
                                                            h1785
                                                        }, v5, v6, v8)))))
                                                    } else {
                                                        Ok::<Adt17, R<Result<Option<Adt21>, Bytes>>>(v3)
                                                    }
                                                } else {
                                                    Ok::<Adt17, R<Result<Option<Adt21>, Bytes>>>(v3)
                                                }
                                            }
                                            Err(v9) => {
                                                Err::<Adt17, R<Result<Option<Adt21>, Bytes>>>(Ok(f345(v9)))
                                            }
                                        }
                                    }
                                    Err(v7) => {
                                        Err::<Adt17, R<Result<Option<Adt21>, Bytes>>>(Ok(f345(v7)))
                                    }
                                }
                            }
                            Adt17::C1(_, _, _) => {
                                Ok::<Adt17, R<Result<Option<Adt21>, Bytes>>>(v3)
                            }
                            Adt17::C2(_) => {
                                Ok::<Adt17, R<Result<Option<Adt21>, Bytes>>>(v3)
                            }
                            Adt17::C3(_, _) => {
                                Ok::<Adt17, R<Result<Option<Adt21>, Bytes>>>(v3)
                            }
                            Adt17::C4 => {
                                Ok::<Adt17, R<Result<Option<Adt21>, Bytes>>>(v3)
                            }
                        }
                    }
                    Adt17::C4 => {
                        Err::<Adt17, R<Result<Option<Adt21>, Bytes>>>(Ok(Ok::<Option<Adt21>, Bytes>(None::<Adt21>)))
                    }
                }
            };
            match step {
                Ok(next) => { m1790 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f260(v0: Bytes) -> Result<Adt20, Bytes> {
    Err::<Adt20, Bytes>(v0)
}

pub fn f261(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit(" at simpleType ")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f262(v0: List<(Bytes, Bytes)>, v1: Bytes, v2: Bytes) -> R<Result<Adt20, Bytes>> {
    let m1792 = f116(v0, v2.clone())?;
    match m1792 {
        Ok(v4) => {
            if f46({
                let (h1793, _) = v4.clone();
                h1793
            }, f184()) {
                let m1795 = f346({
                    let (_, h1794) = v4;
                    h1794
                }, 16u64)?;
                match m1795 {
                    Ok(v6) => {
                        Ok(Ok::<Adt20, Bytes>(Adt20::C0(v6, List::<Bytes>::nil())))
                    }
                    Err(v5) => {
                        Ok(f260(v5))
                    }
                }
            } else {
                if f46({
                    let (h1796, _) = v4.clone();
                    h1796
                }, v1) {
                    let v7: Bytes = f187({
                        let (_, h1797) = v4;
                        h1797
                    })?;
                    Ok(Ok::<Adt20, Bytes>(Adt20::C0(v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                } else {
                    Ok(f260(f7(List::cons(f8(Str::lit("the type ")), List::cons(v2, List::cons(f8(Str::lit(" is in the namespace ")), List::cons({
                        let (h1798, _) = v4;
                        h1798
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
    let m1799 = v0;
    match m1799 {
        Adt17::C0(_, _, _) => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
        Adt17::C2(_) => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
        Adt17::C3(r1806, r1807) => {
            let v2 = (*r1806).clone();
            let v3 = (*r1807).clone();
            let m1800 = f263(v3, v1.clone())?;
            match m1800 {
                Ok(v5) => {
                    let m1801 = v2;
                    match m1801 {
                        Adt17::C0(v6, r1805, _) => {
                            let v7 = (*r1805).clone();
                            let m1802 = f115(v7.clone(), v1);
                            match m1802 {
                                Ok(v10) => {
                                    let m1803 = f116(v10, v6)?;
                                    match m1803 {
                                        Ok(v12) => {
                                            if f117(v12, Str::lit("enumeration")) {
                                                let m1804 = f118(v7, f8(Str::lit("value")));
                                                match m1804 {
                                                    None => {
                                                        Ok(f347(f8(Str::lit("an enumeration facet must have a value attribute (XSD 1.1 Part 2, section 4.3.5.2)"))))
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
                                            Ok(f347(v11))
                                        }
                                    }
                                }
                                Err(v9) => {
                                    Ok(f347(v9))
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
                    Ok(f347(v4))
                }
            }
        }
        Adt17::C4 => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
    }
}

pub fn f264(v0: List<Adt2>, v1: List<Bytes>) -> R<List<Adt2>> {
    let m1808 = v0;
    match m1808.uncons() {
        None => {
            Ok(List::<Adt2>::nil())
        }
        Some((v2, v3)) => {
            let v4: Bytes = f70(v2.clone())?;
            if f149(v4.clone(), v1.clone()) {
                f264(v3, v1)
            } else {
                Ok(List::cons(v2, f264(v3, List::cons(v4, v1))?))
            }
        }
    }
}

pub fn f265(v0: List<Adt2>) -> List<Bytes> {
    let m1809 = v0;
    match m1809.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1810 = f348(Some(v1));
            match m1810 {
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

pub fn f266(v0: Bytes, p1831: u64, p1832: u64, p1833: bool, p1834: Bytes) -> R<Bytes> {
    let m1830 = (p1831, (p1832, (p1833, p1834)));
    {
        let mut m1830 = m1830;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1830;
                let m1811 = v2;
                if m1811 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1811 - 1;
                    let m1814 = {
                        let a1812 = v0.clone();
                        let a1813 = v1;
                        index_bytes(a1812, a1813)
                    };
                    match m1814 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f51(v6)?;
                            if f349(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1815 = v1;
                                    let a1816 = 1u64;
                                    nat_add(a1815, a1816)?
                                }, (v5, (true, v4))))
                            } else {
                                if f82(f350(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1817 = v1;
                                        let a1818 = 1u64;
                                        nat_add(a1817, a1818)?
                                    }, (v5, (false, {
                                        let a1822 = v4;
                                        let a1823 = f43({
                                            let a1819 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1819)
                                        }, {
                                            let a1820 = v7;
                                            let a1821 = 97u64;
                                            nat_sub(a1820, a1821)
                                        }, 1u64);
                                        append_bytes(a1822, a1823)
                                    }))))
                                } else {
                                    if f135(f350(v7), f135(f351(v7), f352(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1824 = v1;
                                            let a1825 = 1u64;
                                            nat_add(a1824, a1825)?
                                        }, (v5, (false, {
                                            let a1826 = v4;
                                            let a1827 = f43(v0.clone(), v1, 1u64);
                                            append_bytes(a1826, a1827)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1828 = v1;
                                            let a1829 = 1u64;
                                            nat_add(a1828, a1829)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1830 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f267(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1835 = f336(v0, v1)?;
    match m1835 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f353({
                let (h1836, _) = v2;
                h1836
            }) {
                Ok(Some({
                    let a1838 = v1;
                    let a1839 = {
                        let (_, h1837) = v2;
                        h1837
                    };
                    nat_add(a1838, a1839)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f268(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1841 = v1;
    let a1842 = {
        let a1840 = v0.clone();
        length_bytes(a1840)
    };
    if if nat_lt(a1841, a1842) {
        f354(v0.clone(), v1)?
    } else {
        false
    } {
        Ok(Some(f202(v0, v1)?))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f269(v0: Bytes, v1: u64) -> R<u64> {
    let (h1843, _) = f164(Fn4::F355(v0), {
        let a1844 = v1;
        let a1845 = 1u64;
        nat_add(a1844, a1845)?
    }, v1)?;
    Ok(h1843)
}

pub fn f270(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f356(v0, v1)
}

pub fn f271(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f357(v0, v1)
}

pub fn f272(v0: u64, v1: bool) -> bool {
    f204(if if f192(v0) {
        true
    } else {
        f193(v0)
    } {
        true
    } else {
        if v1 {
            f358(v0)
        } else {
            false
        }
    })
}

pub fn f273(v0: Bytes, v1: u64, v2: u64) -> R<Adt18> {
    let v3: bool = {
        let a1846 = f127(v0.clone(), v1)?;
        let a1847 = 62u64;
        nat_eq(a1846, a1847)
    };
    let v4: u64 = f127(v0.clone(), {
        let a1848 = v1;
        let a1849 = 1u64;
        nat_add(a1848, a1849)?
    })?;
    let v5: u64 = f127(v0.clone(), {
        let a1850 = v1;
        let a1851 = 2u64;
        nat_add(a1850, a1851)?
    })?;
    let v6: u64 = f359(v4);
    let v7: u64 = f360(v4);
    let v8: bool = {
        let a1852 = 0u64;
        let a1853 = v6;
        if nat_lt(a1852, a1853) {
            true
        } else {
            let a1854 = 0u64;
            let a1855 = v7;
            nat_lt(a1854, a1855)
        }
    };
    let v9: u64 = {
        let a1856 = 0u64;
        let a1857 = v7;
        if nat_lt(a1856, a1857) {
            f359(v5)
        } else {
            0u64
        }
    };
    let v10: u64 = {
        let a1858 = 0u64;
        let a1859 = v6;
        if nat_lt(a1858, a1859) {
            f360(v5)
        } else {
            0u64
        }
    };
    let v11: u64 = {
        let a1860 = v6;
        let a1861 = v9;
        nat_add(a1860, a1861)?
    };
    let v12: u64 = {
        let a1862 = v7;
        let a1863 = v10;
        nat_add(a1862, a1863)?
    };
    let v13: u64 = {
        let a1872 = {
            let a1864 = v1;
            let a1865 = 1u64;
            nat_add(a1864, a1865)?
        };
        let a1873 = {
            let a1870 = if v8 {
                1u64
            } else {
                0u64
            };
            let a1871 = {
                let a1866 = 0u64;
                let a1867 = v9;
                if if nat_lt(a1866, a1867) {
                    true
                } else {
                    let a1868 = 0u64;
                    let a1869 = v10;
                    nat_lt(a1868, a1869)
                } {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a1870, a1871)?
        };
        nat_add(a1872, a1873)?
    };
    let m1874 = f208(v0.clone(), v13)?;
    match m1874 {
        Ok(v15) => {
            let a1875 = 0u64;
            let a1876 = v12;
            if nat_lt(a1875, a1876) {
                f361(v0, v15, {
                    let a1879 = {
                        let a1877 = v2;
                        let a1878 = v12;
                        nat_add(a1877, a1878)?
                    };
                    let a1880 = 1u64;
                    nat_sub(a1879, a1880)
                }, v3, v11)
            } else {
                let v16: (u64, u64) = {
                    let (h1881, _) = f220(Fn5::F362(v0.clone()), {
                        let a1883 = {
                            let a1882 = v0.clone();
                            length_bytes(a1882)
                        };
                        let a1884 = 1u64;
                        nat_add(a1883, a1884)?
                    }, (v15, 0u64))?;
                    h1881
                };
                let v17: u64 = f195(v0.clone(), {
                    let (h1885, _) = v16;
                    h1885
                })?;
                let a1886 = f127(v0.clone(), v17)?;
                let a1887 = 256u64;
                if nat_eq(a1886, a1887) {
                    Ok(Adt18::C0(Adt2::C3({
                        let a1888 = v11;
                        let a1889 = 2u64;
                        if nat_eq(a1888, a1889) {
                            f363(f364(v0.clone(), v15)?)
                        } else {
                            Bytes::lit(&[])
                        }
                    }), {
                        let a1890 = v0;
                        length_bytes(a1890)
                    }))
                } else {
                    let v18: u64 = f365({
                        let a1892 = v17;
                        let a1893 = {
                            let (h1891, _) = v16;
                            h1891
                        };
                        nat_sub(a1892, a1893)
                    }, v2);
                    let a1895 = v18;
                    let a1896 = {
                        let (_, h1894) = v16;
                        h1894
                    };
                    if nat_lt(a1895, a1896) {
                        Ok(Adt18::C1(f108(Str::lit("a leading empty line of a block scalar holds more spaces than its first content line (YAML 1.2.2 section 8.1.1.1)"), v15)?))
                    } else {
                        f361(v0, v15, v18, v3, v11)
                    }
                }
            }
        }
        Err(v14) => {
            Ok(Adt18::C1(v14))
        }
    }
}

pub fn f274(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Adt22> {
    let a1897 = f127(v0.clone(), v1)?;
    let a1898 = 34u64;
    if nat_eq(a1897, a1898) {
        f366(v0.clone(), {
            let a1899 = v1;
            let a1900 = 1u64;
            nat_add(a1899, a1900)?
        }, v2, v3, {
            let a1902 = {
                let a1901 = v0;
                length_bytes(a1901)
            };
            let a1903 = 2u64;
            nat_add(a1902, a1903)?
        }, Bytes::lit(&[]))
    } else {
        f367(v0.clone(), {
            let a1904 = v1;
            let a1905 = 1u64;
            nat_add(a1904, a1905)?
        }, v2, v3, {
            let a1907 = {
                let a1906 = v0;
                length_bytes(a1906)
            };
            let a1908 = 2u64;
            nat_add(a1907, a1908)?
        }, Bytes::lit(&[]))
    }
}

pub fn f275(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: List<Bytes>) -> R<Adt18> {
    let m1909 = v3;
    if m1909 == 0 {
        Ok(Adt18::C1(f108(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v6 = m1909 - 1;
        let a1910 = v4;
        let a1911 = 1u64;
        if nat_eq(a1910, a1911) {
            let m1912 = f275(v0.clone(), v1, v2, v6, 0u64, List::<Bytes>::nil())?;
            match m1912 {
                Adt18::C0(v8, v9) => {
                    let m1913 = f368(v0.clone(), v9, v2)?;
                    match m1913 {
                        Ok(v11) => {
                            let v12: u64 = f127(v0.clone(), v11)?;
                            let a1914 = v12;
                            let a1915 = 58u64;
                            if nat_eq(a1914, a1915) {
                                Ok(Adt18::C1(f108(Str::lit("a single-pair mapping inside a flow sequence is not read (YAML 1.2.2 [150])"), v11)?))
                            } else {
                                let a1916 = v12;
                                let a1917 = 93u64;
                                if nat_eq(a1916, a1917) {
                                    Ok(Adt18::C0(Adt2::C7(std::rc::Rc::new(v8), std::rc::Rc::new(Adt2::C6)), {
                                        let a1918 = v11;
                                        let a1919 = 1u64;
                                        nat_add(a1918, a1919)?
                                    }))
                                } else {
                                    let a1920 = v12;
                                    let a1921 = 44u64;
                                    if nat_eq(a1920, a1921) {
                                        let m1924 = f368(v0.clone(), {
                                            let a1922 = v11;
                                            let a1923 = 1u64;
                                            nat_add(a1922, a1923)?
                                        }, v2)?;
                                        match m1924 {
                                            Ok(v14) => {
                                                let a1925 = f127(v0.clone(), v14)?;
                                                let a1926 = 93u64;
                                                if nat_eq(a1925, a1926) {
                                                    Ok(Adt18::C0(Adt2::C7(std::rc::Rc::new(v8), std::rc::Rc::new(Adt2::C6)), {
                                                        let a1927 = v14;
                                                        let a1928 = 1u64;
                                                        nat_add(a1927, a1928)?
                                                    }))
                                                } else {
                                                    let m1929 = f275(v0, v14, v2, v6, 1u64, List::<Bytes>::nil())?;
                                                    match m1929 {
                                                        Adt18::C0(v16, v17) => {
                                                            Ok(Adt18::C0(Adt2::C7(std::rc::Rc::new(v8), std::rc::Rc::new(v16)), v17))
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
                                        Ok(Adt18::C1(f108(Str::lit("a flow sequence entry must be followed by , or ] (YAML 1.2.2 [137])"), v11)?))
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
            let a1930 = v4;
            let a1931 = 2u64;
            if nat_eq(a1930, a1931) {
                let m1932 = f369(v0.clone(), v1, v2)?;
                match m1932 {
                    Adt22::C0(v19, v20) => {
                        let m1933 = f368(v0.clone(), v20, v2)?;
                        match m1933 {
                            Ok(v22) => {
                                if f204({
                                    let a1934 = f127(v0.clone(), v22)?;
                                    let a1935 = 58u64;
                                    nat_eq(a1934, a1935)
                                }) {
                                    Ok(Adt18::C1(f108(Str::lit("a flow mapping entry needs : and a value; a null value is not read (YAML 1.2.2 [142])"), v22)?))
                                } else {
                                    let m1938 = f368(v0.clone(), {
                                        let a1936 = v22;
                                        let a1937 = 1u64;
                                        nat_add(a1936, a1937)?
                                    }, v2)?;
                                    match m1938 {
                                        Ok(v24) => {
                                            let a1939 = f127(v0.clone(), v24)?;
                                            let a1940 = 44u64;
                                            if if nat_eq(a1939, a1940) {
                                                true
                                            } else {
                                                let a1941 = f127(v0.clone(), v24)?;
                                                let a1942 = 125u64;
                                                nat_eq(a1941, a1942)
                                            } {
                                                Ok(Adt18::C1(f203(v24)?))
                                            } else {
                                                let m1943 = f275(v0.clone(), v24, v2, v6, 0u64, List::<Bytes>::nil())?;
                                                match m1943 {
                                                    Adt18::C0(v26, v27) => {
                                                        if f207(v19.clone(), v5.clone()) {
                                                            Ok(Adt18::C1(f108(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                                        } else {
                                                            let m1944 = f368(v0.clone(), v27, v2)?;
                                                            match m1944 {
                                                                Ok(v29) => {
                                                                    let v30: u64 = f127(v0.clone(), v29)?;
                                                                    let a1945 = v30;
                                                                    let a1946 = 125u64;
                                                                    if nat_eq(a1945, a1946) {
                                                                        Ok(Adt18::C0(Adt2::C8(v19, std::rc::Rc::new(v26), std::rc::Rc::new(Adt2::C6)), {
                                                                            let a1947 = v29;
                                                                            let a1948 = 1u64;
                                                                            nat_add(a1947, a1948)?
                                                                        }))
                                                                    } else {
                                                                        let a1949 = v30;
                                                                        let a1950 = 44u64;
                                                                        if nat_eq(a1949, a1950) {
                                                                            let m1953 = f368(v0.clone(), {
                                                                                let a1951 = v29;
                                                                                let a1952 = 1u64;
                                                                                nat_add(a1951, a1952)?
                                                                            }, v2)?;
                                                                            match m1953 {
                                                                                Ok(v32) => {
                                                                                    let a1954 = f127(v0.clone(), v32)?;
                                                                                    let a1955 = 125u64;
                                                                                    if nat_eq(a1954, a1955) {
                                                                                        Ok(Adt18::C0(Adt2::C8(v19, std::rc::Rc::new(v26), std::rc::Rc::new(Adt2::C6)), {
                                                                                            let a1956 = v32;
                                                                                            let a1957 = 1u64;
                                                                                            nat_add(a1956, a1957)?
                                                                                        }))
                                                                                    } else {
                                                                                        let m1958 = f275(v0, v32, v2, v6, 2u64, List::cons(v19.clone(), v5))?;
                                                                                        match m1958 {
                                                                                            Adt18::C0(v34, v35) => {
                                                                                                Ok(Adt18::C0(Adt2::C8(v19, std::rc::Rc::new(v26), std::rc::Rc::new(v34)), v35))
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
                                                                            Ok(Adt18::C1(f108(Str::lit("a flow mapping entry must be followed by , or } (YAML 1.2.2 [140])"), v29)?))
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
                let v36: u64 = f127(v0.clone(), v1)?;
                let a1959 = v36;
                let a1960 = 91u64;
                if if nat_eq(a1959, a1960) {
                    true
                } else {
                    let a1961 = v36;
                    let a1962 = 123u64;
                    nat_eq(a1961, a1962)
                } {
                    let m1965 = f368(v0.clone(), {
                        let a1963 = v1;
                        let a1964 = 1u64;
                        nat_add(a1963, a1964)?
                    }, v2)?;
                    match m1965 {
                        Ok(v38) => {
                            let a1968 = f127(v0.clone(), v38)?;
                            let a1969 = {
                                let a1966 = v36;
                                let a1967 = 2u64;
                                nat_add(a1966, a1967)?
                            };
                            if nat_eq(a1968, a1969) {
                                Ok(Adt18::C0({
                                    let a1970 = v36;
                                    let a1971 = 91u64;
                                    if nat_eq(a1970, a1971) {
                                        Adt2::C4(std::rc::Rc::new(Adt2::C6))
                                    } else {
                                        Adt2::C5(std::rc::Rc::new(Adt2::C6))
                                    }
                                }, {
                                    let a1972 = v38;
                                    let a1973 = 1u64;
                                    nat_add(a1972, a1973)?
                                }))
                            } else {
                                let m1976 = f275(v0, v38, v2, v6, {
                                    let a1974 = v36;
                                    let a1975 = 91u64;
                                    if nat_eq(a1974, a1975) {
                                        1u64
                                    } else {
                                        2u64
                                    }
                                }, List::<Bytes>::nil())?;
                                match m1976 {
                                    Adt18::C0(v40, v41) => {
                                        Ok(Adt18::C0({
                                            let a1977 = v36;
                                            let a1978 = 91u64;
                                            if nat_eq(a1977, a1978) {
                                                Adt2::C4(std::rc::Rc::new(v40))
                                            } else {
                                                Adt2::C5(std::rc::Rc::new(v40))
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
                    let a1979 = v36;
                    let a1980 = 34u64;
                    if if nat_eq(a1979, a1980) {
                        true
                    } else {
                        let a1981 = v36;
                        let a1982 = 39u64;
                        nat_eq(a1981, a1982)
                    } {
                        let m1983 = f274(v0, v1, v2, false)?;
                        match m1983 {
                            Adt22::C0(v43, v44) => {
                                Ok(Adt18::C0(Adt2::C3(v43), v44))
                            }
                            Adt22::C1(v42) => {
                                Ok(Adt18::C1(v42))
                            }
                        }
                    } else {
                        let m1984 = f199(v0.clone(), v1)?;
                        match m1984 {
                            None => {
                                if f204(f276(v0.clone(), v1, true)?) {
                                    Ok(Adt18::C1(f108(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                                } else {
                                    let v46: u64 = f277(v0.clone(), v1, true)?;
                                    let v47: u64 = f196(v0.clone(), v46)?;
                                    let m1987 = {
                                        let a1985 = f127(v0.clone(), v47)?;
                                        let a1986 = 10u64;
                                        if nat_eq(a1985, a1986) {
                                            f368(v0.clone(), v47, v2)?
                                        } else {
                                            Ok::<u64, Bytes>(v47)
                                        }
                                    };
                                    match m1987 {
                                        Ok(v49) => {
                                            let a1988 = f127(v0.clone(), v47)?;
                                            let a1989 = 10u64;
                                            if if if nat_eq(a1988, a1989) {
                                                f204(f358(f127(v0.clone(), v49)?))
                                            } else {
                                                false
                                            } {
                                                f204({
                                                    let a1990 = f127(v0.clone(), v49)?;
                                                    let a1991 = 58u64;
                                                    nat_eq(a1990, a1991)
                                                })
                                            } else {
                                                false
                                            } {
                                                Ok(Adt18::C1(f108(Str::lit("a multi-line plain scalar inside a flow collection is not read (YAML 1.2.2 [135])"), v47)?))
                                            } else {
                                                f280(f279(v0, v1, v46), v1, v46)
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

pub fn f276(v0: Bytes, v1: u64, v2: bool) -> R<bool> {
    let v3: u64 = f127(v0.clone(), v1)?;
    if if f192(v3) {
        true
    } else {
        f193(v3)
    } {
        Ok(false)
    } else {
        let a1992 = v3;
        let a1993 = 45u64;
        if if if nat_eq(a1992, a1993) {
            true
        } else {
            let a1994 = v3;
            let a1995 = 63u64;
            nat_eq(a1994, a1995)
        } {
            true
        } else {
            let a1996 = v3;
            let a1997 = 58u64;
            nat_eq(a1996, a1997)
        } {
            Ok(f272(f127(v0, {
                let a1998 = v1;
                let a1999 = 1u64;
                nat_add(a1998, a1999)?
            })?, v2))
        } else {
            Ok(f204(f370(v3)?))
        }
    }
}

pub fn f277(v0: Bytes, v1: u64, v2: bool) -> R<u64> {
    let (_, h2000) = {
        let (h2001, _) = f220(Fn5::F371(v2, v0.clone()), {
            let a2003 = {
                let a2002 = v0;
                length_bytes(a2002)
            };
            let a2004 = 1u64;
            nat_add(a2003, a2004)?
        }, (v1, v1))?;
        h2001
    };
    Ok(h2000)
}

pub fn f278(v0: Bytes, p2029: u64, v2: u64, p2030: u64, p2031: Bytes) -> R<Adt22> {
    let m2028 = (p2029, (p2030, p2031));
    {
        let mut m2028 = m2028;
        loop {
            let step = {
                let (v1, (v3, v4)) = m2028;
                let m2005 = v3;
                if m2005 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C1(f108(Str::lit("a plain scalar has more lines than the input"), v1)?)))
                } else {
                    let v5 = m2005 - 1;
                    let v6: u64 = f196(v0.clone(), v1)?;
                    let v7: u64 = f127(v0.clone(), v6)?;
                    let a2006 = v7;
                    let a2007 = 58u64;
                    if nat_eq(a2006, a2007) {
                        Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C1(f372(v6)?)))
                    } else {
                        let a2008 = v7;
                        let a2009 = 35u64;
                        if if nat_eq(a2008, a2009) {
                            true
                        } else {
                            let a2010 = v7;
                            let a2011 = 256u64;
                            nat_eq(a2010, a2011)
                        } {
                            Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C0(v4, f202(v0.clone(), v6)?)))
                        } else {
                            let v8: (u64, u64) = f373(v0.clone(), {
                                let a2012 = v6;
                                let a2013 = 1u64;
                                nat_add(a2012, a2013)?
                            })?;
                            let v9: u64 = {
                                let (h2014, _) = v8;
                                h2014
                            };
                            let v10: u64 = f195(v0.clone(), v9)?;
                            let v11: u64 = f196(v0.clone(), v10)?;
                            let v12: u64 = f127(v0.clone(), v11)?;
                            let a2015 = v12;
                            let a2016 = 256u64;
                            if if if if nat_eq(a2015, a2016) {
                                true
                            } else {
                                let a2019 = {
                                    let a2017 = v10;
                                    let a2018 = v9;
                                    nat_sub(a2017, a2018)
                                };
                                let a2020 = v2;
                                nat_lt(a2019, a2020)
                            } {
                                true
                            } else {
                                let a2021 = v12;
                                let a2022 = 35u64;
                                nat_eq(a2021, a2022)
                            } {
                                true
                            } else {
                                f126(v0.clone(), v9)?
                            } {
                                Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C0(v4, {
                                    let a2023 = v6;
                                    let a2024 = 1u64;
                                    nat_add(a2023, a2024)?
                                })))
                            } else {
                                let v13: u64 = f277(v0.clone(), v11, false)?;
                                let a2025 = v13;
                                let a2026 = v11;
                                if nat_eq(a2025, a2026) {
                                    Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C1(f372(v11)?)))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Adt22>>((v13, (v5, f7(List::cons(v4, List::cons(f374({
                                        let (_, h2027) = v8;
                                        h2027
                                    }), List::cons(f279(v0.clone(), v11, v13), List::<Bytes>::nil())))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2028 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f279(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    f43(v0, v1, {
        let a2032 = v2;
        let a2033 = v1;
        nat_sub(a2032, a2033)
    })
}

pub fn f280(v0: Bytes, v1: u64, v2: u64) -> R<Adt18> {
    let a2035 = {
        let a2034 = v0.clone();
        length_bytes(a2034)
    };
    let a2036 = 0u64;
    if if nat_eq(a2035, a2036) {
        true
    } else {
        f207(v0.clone(), List::cons(f8(Str::lit("~")), List::cons(f8(Str::lit("null")), List::cons(f8(Str::lit("Null")), List::cons(f8(Str::lit("NULL")), List::<Bytes>::nil())))))
    } {
        Ok(Adt18::C1(f203(v1)?))
    } else {
        if f207(v0.clone(), List::cons(f8(Str::lit("true")), List::cons(f8(Str::lit("True")), List::cons(f8(Str::lit("TRUE")), List::<Bytes>::nil())))) {
            Ok(Adt18::C0(Adt2::C0(true), v2))
        } else {
            if f207(v0.clone(), List::cons(f8(Str::lit("false")), List::cons(f8(Str::lit("False")), List::cons(f8(Str::lit("FALSE")), List::<Bytes>::nil())))) {
                Ok(Adt18::C0(Adt2::C0(false), v2))
            } else {
                if f207(v0.clone(), List::cons(f8(Str::lit(".inf")), List::cons(f8(Str::lit(".Inf")), List::cons(f8(Str::lit(".INF")), List::cons(f8(Str::lit("+.inf")), List::cons(f8(Str::lit("+.Inf")), List::cons(f8(Str::lit("+.INF")), List::cons(f8(Str::lit("-.inf")), List::cons(f8(Str::lit("-.Inf")), List::cons(f8(Str::lit("-.INF")), List::cons(f8(Str::lit(".nan")), List::cons(f8(Str::lit(".NaN")), List::cons(f8(Str::lit(".NAN")), List::<Bytes>::nil()))))))))))))) {
                    Ok(Adt18::C1(f108(Str::lit("an infinite or not-a-number float is not read: Studio.Value has none (YAML 1.2.2 section 10.3.2)"), v1)?))
                } else {
                    if if f111(v0.clone(), 0u64, Str::lit("0o")) {
                        let a2038 = 2u64;
                        let a2039 = {
                            let a2037 = v0.clone();
                            length_bytes(a2037)
                        };
                        nat_lt(a2038, a2039)
                    } else {
                        false
                    } {
                        f375(v0, 8u64, v1, v2)
                    } else {
                        if if f111(v0.clone(), 0u64, Str::lit("0x")) {
                            let a2041 = 2u64;
                            let a2042 = {
                                let a2040 = v0.clone();
                                length_bytes(a2040)
                            };
                            nat_lt(a2041, a2042)
                        } else {
                            false
                        } {
                            f375(v0, 16u64, v1, v2)
                        } else {
                            let m2043 = f376(v0.clone())?;
                            match m2043 {
                                None => {
                                    Ok(Adt18::C0(Adt2::C3(v0), v2))
                                }
                                Some(v3) => {
                                    let m2044 = f83(v3.clone(), 0u64)?;
                                    match m2044 {
                                        None => {
                                            Ok(Adt18::C1(f377(v1)?))
                                        }
                                        Some(v4) => {
                                            let a2047 = {
                                                let (h2045, _) = v4;
                                                h2045
                                            };
                                            let a2048 = {
                                                let a2046 = v3;
                                                length_bytes(a2046)
                                            };
                                            if nat_eq(a2047, a2048) {
                                                Ok(Adt18::C0({
                                                    let (_, h2049) = v4;
                                                    h2049
                                                }, v2))
                                            } else {
                                                Ok(Adt18::C1(f377(v1)?))
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

pub fn f281(v0: Bytes, v1: u64) -> R<u64> {
    let (h2050, _) = f164(Fn4::F378(v0.clone()), {
        let a2052 = {
            let a2051 = v0;
            length_bytes(a2051)
        };
        let a2053 = 1u64;
        nat_add(a2052, a2053)?
    }, v1)?;
    Ok(h2050)
}

pub fn f282(v0: Bytes, v1: u64) -> R<bool> {
    let a2054 = f127(v0.clone(), v1)?;
    let a2055 = 58u64;
    if nat_eq(a2054, a2055) {
        if f192(f127(v0.clone(), {
            let a2056 = v1;
            let a2057 = 1u64;
            nat_add(a2056, a2057)?
        })?) {
            Ok(true)
        } else {
            Ok(f193(f127(v0, {
                let a2058 = v1;
                let a2059 = 1u64;
                nat_add(a2058, a2059)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f283(v0: Bytes, p2078: u64, p2079: u64, p2080: Bytes) -> R<Bytes> {
    let m2077 = (p2078, (p2079, p2080));
    {
        let mut m2077 = m2077;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2077;
                let m2060 = v2;
                if m2060 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m2060 - 1;
                    let v5: u64 = f132(v0.clone(), v1, {
                        let a2064 = {
                            let a2062 = {
                                let a2061 = v0.clone();
                                length_bytes(a2061)
                            };
                            let a2063 = v1;
                            nat_sub(a2062, a2063)
                        };
                        let a2065 = 1u64;
                        nat_add(a2064, a2065)?
                    })?;
                    let v6: Bytes = {
                        let a2068 = v3;
                        let a2069 = f43(v0.clone(), v1, {
                            let a2066 = v5;
                            let a2067 = v1;
                            nat_sub(a2066, a2067)
                        });
                        append_bytes(a2068, a2069)
                    };
                    let m2072 = {
                        let a2070 = v0.clone();
                        let a2071 = v5;
                        index_bytes(a2070, a2071)
                    };
                    match m2072 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v6))
                        }
                        Some(v7) => {
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                let a2073 = v5;
                                let a2074 = 1u64;
                                nat_add(a2073, a2074)?
                            }, (v4, {
                                let a2075 = v6;
                                let a2076 = f379(v0.clone(), v5, v7)?;
                                append_bytes(a2075, a2076)
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2077 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f284(v0: Fn6, p2084: List<(Bytes, Bytes)>, p2085: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m2083 = (p2084, p2085);
    {
        let mut m2083 = m2083;
        loop {
            let step = {
                let (v1, v2) = m2083;
                let m2081 = v2;
                match m2081.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c2082: Fn6 = v0.clone();
                            c2082.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2083 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f285(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f380(v0, v1)
}

pub fn f286(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f381(v0)
}

pub fn f287(v0: Fn7, p2089: List<Bytes>, p2090: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m2088 = (p2089, p2090);
    {
        let mut m2088 = m2088;
        loop {
            let step = {
                let (v1, v2) = m2088;
                let m2086 = v2;
                match m2086.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c2087: Fn7 = v0.clone();
                            c2087.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2088 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f288(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f382(v1)?, v0))
}

pub fn f289(v0: Bytes, v1: u64) -> R<Bytes> {
    let a2093 = f43(v0.clone(), {
        let a2091 = v1;
        let a2092 = 2u64;
        nat_add(a2091, a2092)?
    }, 2u64);
    let a2094 = Bytes::lit(&[0x30, 0x30]);
    if equal(a2093, a2094) {
        let m2097 = f383(f43(v0, {
            let a2095 = v1;
            let a2096 = 4u64;
            nat_add(a2095, a2096)?
        }, 2u64))?;
        match m2097 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f80(v2.clone(), 0u64)?;
                let a2098 = v3;
                let a2099 = 32u64;
                if nat_lt(a2098, a2099) {
                    if f384(v3) {
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

pub fn f290(v0: u64) -> Bytes {
    let a2100 = v0;
    let a2101 = 34u64;
    if nat_eq(a2100, a2101) {
        Bytes::lit(&[0x22])
    } else {
        let a2102 = v0;
        let a2103 = 92u64;
        if nat_eq(a2102, a2103) {
            Bytes::lit(&[0x5c])
        } else {
            let a2104 = v0;
            let a2105 = 98u64;
            if nat_eq(a2104, a2105) {
                Bytes::lit(&[0x08])
            } else {
                let a2106 = v0;
                let a2107 = 102u64;
                if nat_eq(a2106, a2107) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a2108 = v0;
                    let a2109 = 110u64;
                    if nat_eq(a2108, a2109) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a2110 = v0;
                        let a2111 = 114u64;
                        if nat_eq(a2110, a2111) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a2112 = v0;
                            let a2113 = 116u64;
                            if nat_eq(a2112, a2113) {
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

pub fn f291(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f80(v0, v1)?;
    let a2114 = 47u64;
    let a2115 = v2;
    if if nat_lt(a2114, a2115) {
        let a2116 = v2;
        let a2117 = 58u64;
        nat_lt(a2116, a2117)
    } else {
        false
    } {
        Ok(Some({
            let a2118 = v1;
            let a2119 = 1u64;
            nat_add(a2118, a2119)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f292(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2120 = f80(v0, v1)?;
    let a2121 = 48u64;
    if nat_eq(a2120, a2121) {
        Ok(Some({
            let a2122 = v1;
            let a2123 = 1u64;
            nat_add(a2122, a2123)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f293(v0: Bytes) -> R<u64> {
    let (h2124, _) = f164(Fn4::F385(v0.clone()), {
        let a2125 = v0;
        length_bytes(a2125)
    }, 0u64)?;
    Ok(h2124)
}

pub fn f294(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m2129 = {
        let a2127 = v0;
        let a2128 = {
            let (h2126, _) = v1;
            h2126
        };
        index_bytes(a2127, a2128)
    };
    match m2129 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2131 = {
                    let (h2130, _) = v1;
                    h2130
                };
                let a2132 = 1u64;
                nat_add(a2131, a2132)?
            }, {
                let a2136 = {
                    let a2134 = {
                        let (_, h2133) = v1;
                        h2133
                    };
                    let a2135 = 10u64;
                    nat_mul(a2134, a2135)?
                };
                let a2137 = f386(v2)?;
                nat_add(a2136, a2137)?
            })))
        }
    }
}

pub fn f295(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f387(v0, v1)
}

pub fn f296(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a2138 = v0;
        utf8_encode(a2138)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f64(v1)?, List::<Bytes>::nil())))))
}

pub fn f297(p2141: List<Adt19>, p2142: List<Adt19>) -> List<Adt19> {
    let m2140 = (p2141, p2142);
    {
        let mut m2140 = m2140;
        loop {
            let step = {
                let (v0, v1) = m2140;
                let m2139 = v0;
                match m2139.uncons() {
                    None => {
                        Err::<(List<Adt19>, List<Adt19>), List<Adt19>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt19>, List<Adt19>), List<Adt19>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2140 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f298(v0: u64) -> bool {
    let a2143 = 65u64;
    let a2144 = v0;
    if if nat_le(a2143, a2144) {
        let a2145 = v0;
        let a2146 = 90u64;
        nat_le(a2145, a2146)
    } else {
        false
    } {
        true
    } else {
        let a2147 = 97u64;
        let a2148 = v0;
        if nat_le(a2147, a2148) {
            let a2149 = v0;
            let a2150 = 122u64;
            nat_le(a2149, a2150)
        } else {
            false
        }
    }
}

pub fn f299(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h2151, _) = f164(Fn4::F388(v1, v0.clone()), {
        let a2153 = {
            let a2152 = v0;
            length_bytes(a2152)
        };
        let a2154 = 1u64;
        nat_add(a2153, a2154)?
    }, v2)?;
    Ok(h2151)
}

pub fn f300(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f112(v0.clone(), v1)?;
    let a2155 = v2;
    let a2156 = 48u64;
    if nat_eq(a2155, a2156) {
        let v3: u64 = f112(v0.clone(), {
            let a2157 = v1;
            let a2158 = 1u64;
            nat_add(a2157, a2158)?
        })?;
        let a2159 = v3;
        let a2160 = 88u64;
        if if if nat_eq(a2159, a2160) {
            true
        } else {
            let a2161 = v3;
            let a2162 = 120u64;
            nat_eq(a2161, a2162)
        } {
            f389(f112(v0.clone(), {
                let a2163 = v1;
                let a2164 = 2u64;
                nat_add(a2163, a2164)?
            })?)
        } else {
            false
        } {
            f299(v0, 1u64, {
                let a2165 = v1;
                let a2166 = 2u64;
                nat_add(a2165, a2166)?
            })
        } else {
            f299(v0, 2u64, {
                let a2167 = v1;
                let a2168 = 1u64;
                nat_add(a2167, a2168)?
            })
        }
    } else {
        if f390(v2) {
            f299(v0, 0u64, v1)
        } else {
            Ok(v1)
        }
    }
}

pub fn f301(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f299(v0.clone(), 0u64, v1)?;
    let a2169 = f112(v0.clone(), v2)?;
    let a2170 = 46u64;
    if nat_eq(a2169, a2170) {
        let v3: u64 = f299(v0.clone(), 0u64, {
            let a2171 = v2;
            let a2172 = 1u64;
            nat_add(a2171, a2172)?
        })?;
        let a2173 = v1;
        let a2174 = v2;
        if if nat_lt(a2173, a2174) {
            true
        } else {
            let a2177 = {
                let a2175 = v2;
                let a2176 = 1u64;
                nat_add(a2175, a2176)?
            };
            let a2178 = v3;
            nat_lt(a2177, a2178)
        } {
            f391(v0, v3)
        } else {
            Ok(v1)
        }
    } else {
        let a2179 = v1;
        let a2180 = v2;
        if nat_lt(a2179, a2180) {
            let v4: u64 = f391(v0, v2)?;
            let a2181 = v2;
            let a2182 = v4;
            if nat_lt(a2181, a2182) {
                Ok(v4)
            } else {
                Ok(v1)
            }
        } else {
            Ok(v1)
        }
    }
}

pub fn f302(v0: List<Adt19>, v1: Str) -> bool {
    let m2183 = v0;
    match m2183.uncons() {
        None => {
            false
        }
        Some((v2, _)) => {
            f392(v2, {
                let a2184 = v1;
                utf8_encode(a2184)
            })
        }
    }
}

pub fn f303(p2197: u64, p2198: List<Adt19>, p2199: List<Bytes>, v3: bool) -> Result<List<Adt19>, Bytes> {
    let m2196 = (p2197, (p2198, p2199));
    {
        let mut m2196 = m2196;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2196;
                let m2185 = v0;
                if m2185 == 0 {
                    Err::<(u64, (List<Adt19>, List<Bytes>)), Result<List<Adt19>, Bytes>>(Err::<List<Adt19>, Bytes>(f8(Str::lit("brackets nest deeper than the input is long"))))
                } else {
                    let v4 = m2185 - 1;
                    let m2186 = v1;
                    match m2186.uncons() {
                        None => {
                            Err::<(u64, (List<Adt19>, List<Bytes>)), Result<List<Adt19>, Bytes>>(Err::<List<Adt19>, Bytes>(f8(Str::lit("the input ends inside brackets or before the ; that closes a member (Web IDL, IDL grammar)"))))
                        }
                        Some((v5, v6)) => {
                            let a2187 = f393(v5.clone());
                            let a2188 = 3u64;
                            if nat_eq(a2187, a2188) {
                                Ok::<(u64, (List<Adt19>, List<Bytes>)), Result<List<Adt19>, Bytes>>((v4, (v6, v2)))
                            } else {
                                let v7: Bytes = f394(v5);
                                let v8: Bytes = f395(v7.clone());
                                if !f46(v8.clone(), Bytes::lit(&[])) {
                                    Ok::<(u64, (List<Adt19>, List<Bytes>)), Result<List<Adt19>, Bytes>>((v4, (v6, List::cons(v8, v2))))
                                } else {
                                    if f396(v7.clone()) {
                                        let m2189 = v2;
                                        match m2189.uncons() {
                                            None => {
                                                Err::<(u64, (List<Adt19>, List<Bytes>)), Result<List<Adt19>, Bytes>>(Err::<List<Adt19>, Bytes>(f7(List::cons(f8(Str::lit("the closing bracket ")), List::cons(v7, List::cons(f8(Str::lit(" has no opening bracket (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
                                            }
                                            Some((v9, v10)) => {
                                                if f46(v9.clone(), v7.clone()) {
                                                    if if !v3 {
                                                        let a2191 = {
                                                            let a2190 = v10.clone();
                                                            length_list(a2190)
                                                        };
                                                        let a2192 = 0u64;
                                                        nat_eq(a2191, a2192)
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
                                            if f46(v7, f8(Str::lit(";"))) {
                                                let a2194 = {
                                                    let a2193 = v2.clone();
                                                    length_list(a2193)
                                                };
                                                let a2195 = 0u64;
                                                nat_eq(a2194, a2195)
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
                Ok(next) => { m2196 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f304(v0: List<Adt19>) -> List<Adt19> {
    let m2200 = v0;
    match m2200.uncons() {
        None => {
            List::<Adt19>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f305(v0: u64, v1: u64, v2: List<Adt19>, v3: bool) -> R<Result<(Adt13, List<Adt19>), Bytes>> {
    let m2201 = f308(v2);
    match m2201 {
        Ok(v5) => {
            let v6: bool = if v3 {
                f302(f310(v5.clone()), Str::lit(":"))
            } else {
                false
            };
            let m2202 = if v6 {
                f308(f304(f310(v5.clone())))
            } else {
                Ok::<Adt23, Bytes>(Adt23::C0(Bytes::lit(&[]), f310(v5.clone())))
            };
            match m2202 {
                Ok(v8) => {
                    let m2203 = f309(f310(v8.clone()), Str::lit("{"));
                    match m2203 {
                        Ok(v10) => {
                            let m2208 = {
                                let a2204 = v1;
                                let a2205 = 2u64;
                                if if nat_eq(a2204, a2205) {
                                    true
                                } else {
                                    let a2206 = v1;
                                    let a2207 = 7u64;
                                    nat_eq(a2206, a2207)
                                } {
                                    f397(v0, v10, List::<Adt14>::nil())?
                                } else {
                                    f398(v0, v10, List::<Adt14>::nil())?
                                }
                            };
                            match m2208 {
                                Ok(v12) => {
                                    Ok(Ok::<(Adt13, List<Adt19>), Bytes>((Adt13::C0(v1, f312(v5), f312(v8), {
                                        let (h2209, _) = v12;
                                        h2209
                                    }, List::<Bytes>::nil(), f313(Bytes::lit(&[]))), {
                                        let (_, h2210) = v12;
                                        h2210
                                    })))
                                }
                                Err(v11) => {
                                    Ok(Err::<(Adt13, List<Adt19>), Bytes>(v11))
                                }
                            }
                        }
                        Err(v9) => {
                            Ok(Err::<(Adt13, List<Adt19>), Bytes>(v9))
                        }
                    }
                }
                Err(v7) => {
                    Ok(Err::<(Adt13, List<Adt19>), Bytes>(v7))
                }
            }
        }
        Err(v4) => {
            Ok(Err::<(Adt13, List<Adt19>), Bytes>(v4))
        }
    }
}

pub fn f306(v0: List<Adt19>, v1: Bytes) -> R<Result<(Adt13, List<Adt19>), Bytes>> {
    let m2211 = f308(v0);
    match m2211 {
        Ok(v3) => {
            let m2212 = f399(f310(v3.clone()))?;
            match m2212 {
                Ok(v5) => {
                    Ok(Ok::<(Adt13, List<Adt19>), Bytes>((Adt13::C0(9u64, f312(v3), Bytes::lit(&[]), List::<Adt14>::nil(), List::cons(v1, List::<Bytes>::nil()), f313(Bytes::lit(&[]))), v5)))
                }
                Err(v4) => {
                    Ok(Err::<(Adt13, List<Adt19>), Bytes>(v4))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<(Adt13, List<Adt19>), Bytes>(v2))
        }
    }
}

pub fn f307(v0: Str, v1: List<Adt19>) -> Bytes {
    f7(List::cons(f8(Str::lit("expected ")), List::cons({
        let a2213 = v0;
        utf8_encode(a2213)
    }, List::cons(f8(Str::lit(" but found ")), List::cons(f400(v1), List::cons(f8(Str::lit(" (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
}

pub fn f308(v0: List<Adt19>) -> Result<Adt23, Bytes> {
    let m2214 = v0.clone();
    match m2214.uncons() {
        None => {
            Err::<Adt23, Bytes>(f307(Str::lit("an identifier"), v0))
        }
        Some((v1, v2)) => {
            let a2215 = f393(v1.clone());
            let a2216 = 0u64;
            if nat_eq(a2215, a2216) {
                Ok::<Adt23, Bytes>(Adt23::C0(f401(f394(v1)), v2))
            } else {
                Err::<Adt23, Bytes>(f307(Str::lit("an identifier"), v0))
            }
        }
    }
}

pub fn f309(v0: List<Adt19>, v1: Str) -> Result<List<Adt19>, Bytes> {
    if f302(v0.clone(), v1.clone()) {
        Ok::<List<Adt19>, Bytes>(f304(v0))
    } else {
        Err::<List<Adt19>, Bytes>(f307(v1, v0))
    }
}

pub fn f310(v0: Adt23) -> List<Adt19> {
    let m2217 = v0;
    match m2217 {
        Adt23::C0(_, v2) => {
            v2
        }
    }
}

pub fn f311(p2225: u64, p2226: List<Adt19>, p2227: List<Bytes>) -> Result<(List<Bytes>, List<Adt19>), Bytes> {
    let m2224 = (p2225, (p2226, p2227));
    {
        let mut m2224 = m2224;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2224;
                let m2218 = v0;
                if m2218 == 0 {
                    Err::<(u64, (List<Adt19>, List<Bytes>)), Result<(List<Bytes>, List<Adt19>), Bytes>>(Err::<(List<Bytes>, List<Adt19>), Bytes>(f8(Str::lit("an enumeration has more values than the input has tokens"))))
                } else {
                    let v3 = m2218 - 1;
                    let m2219 = v1.clone();
                    match m2219.uncons() {
                        None => {
                            Err::<(u64, (List<Adt19>, List<Bytes>)), Result<(List<Bytes>, List<Adt19>), Bytes>>(Err::<(List<Bytes>, List<Adt19>), Bytes>(f307(Str::lit("a string"), v1)))
                        }
                        Some((v4, v5)) => {
                            let a2220 = f393(v4.clone());
                            let a2221 = 3u64;
                            if !nat_eq(a2220, a2221) {
                                Err::<(u64, (List<Adt19>, List<Bytes>)), Result<(List<Bytes>, List<Adt19>), Bytes>>(Err::<(List<Bytes>, List<Adt19>), Bytes>(f307(Str::lit("a string"), v1)))
                            } else {
                                if f302(v5.clone(), Str::lit(",")) {
                                    if f302(f304(v5.clone()), Str::lit("}")) {
                                        let m2222 = f309(f304(f304(v5)), Str::lit(";"));
                                        match m2222 {
                                            Ok(v7) => {
                                                Err::<(u64, (List<Adt19>, List<Bytes>)), Result<(List<Bytes>, List<Adt19>), Bytes>>(Ok::<(List<Bytes>, List<Adt19>), Bytes>((f57(List::cons(f394(v4), v2)), v7)))
                                            }
                                            Err(v6) => {
                                                Err::<(u64, (List<Adt19>, List<Bytes>)), Result<(List<Bytes>, List<Adt19>), Bytes>>(Err::<(List<Bytes>, List<Adt19>), Bytes>(v6))
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (List<Adt19>, List<Bytes>)), Result<(List<Bytes>, List<Adt19>), Bytes>>((v3, (f304(v5), List::cons(f394(v4), v2))))
                                    }
                                } else {
                                    if f302(v5.clone(), Str::lit("}")) {
                                        let m2223 = f309(f304(v5), Str::lit(";"));
                                        match m2223 {
                                            Ok(v9) => {
                                                Err::<(u64, (List<Adt19>, List<Bytes>)), Result<(List<Bytes>, List<Adt19>), Bytes>>(Ok::<(List<Bytes>, List<Adt19>), Bytes>((f57(List::cons(f394(v4), v2)), v9)))
                                            }
                                            Err(v8) => {
                                                Err::<(u64, (List<Adt19>, List<Bytes>)), Result<(List<Bytes>, List<Adt19>), Bytes>>(Err::<(List<Bytes>, List<Adt19>), Bytes>(v8))
                                            }
                                        }
                                    } else {
                                        Err::<(u64, (List<Adt19>, List<Bytes>)), Result<(List<Bytes>, List<Adt19>), Bytes>>(Err::<(List<Bytes>, List<Adt19>), Bytes>(f307(Str::lit(", or }"), v5)))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2224 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f312(v0: Adt23) -> Bytes {
    let m2228 = v0;
    match m2228 {
        Adt23::C0(v1, _) => {
            v1
        }
    }
}

pub fn f313(v0: Bytes) -> Adt15 {
    Adt15::C0(0u64, v0, List::<Bytes>::nil(), Bytes::lit(&[]), Bytes::lit(&[]))
}

pub fn f314(v0: u64, v1: List<Adt19>, v2: u64) -> R<Result<Adt24, Bytes>> {
    let m2229 = v0;
    if m2229 == 0 {
        Ok(Err::<Adt24, Bytes>(f8(Str::lit("a type nests deeper than the input is long"))))
    } else {
        let v3 = m2229 - 1;
        let a2230 = v2;
        let a2231 = 0u64;
        if nat_eq(a2230, a2231) {
            let m2232 = f226(v1)?;
            match m2232 {
                Ok(v5) => {
                    if f302(v5.clone(), Str::lit("(")) {
                        let m2233 = f314(v3, f304(v5), 1u64)?;
                        match m2233 {
                            Ok(v7) => {
                                if f302(f315(v7.clone()), Str::lit("?")) {
                                    Ok(Ok::<Adt24, Bytes>(Adt24::C0(Adt15::C0(0u64, Bytes::lit(&[]), f246(f316(v7.clone())), f8(Str::lit("a nullable union type is not translated yet, because the typedef identifier names only the union (Web IDL 2.13.32)")), Bytes::lit(&[])), f304(f315(v7)))))
                                } else {
                                    Ok(Ok::<Adt24, Bytes>(Adt24::C0(f316(v7.clone()), f315(v7))))
                                }
                            }
                            Err(v6) => {
                                Ok(Err::<Adt24, Bytes>(v6))
                            }
                        }
                    } else {
                        let m2234 = v5.clone();
                        match m2234.uncons() {
                            None => {
                                Ok(Err::<Adt24, Bytes>(f307(Str::lit("a type"), v5)))
                            }
                            Some((v8, v9)) => {
                                let v10: Bytes = f394(v8.clone());
                                let a2235 = f393(v8);
                                let a2236 = 0u64;
                                if !nat_eq(a2235, a2236) {
                                    Ok(Err::<Adt24, Bytes>(f307(Str::lit("a type"), v5)))
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
                                        let m2237 = f309(v9, Str::lit("<"));
                                        match m2237 {
                                            Ok(v12) => {
                                                let m2238 = f314(v3, v12, 0u64)?;
                                                match m2238 {
                                                    Ok(v14) => {
                                                        let m2239 = f309(f315(v14.clone()), Str::lit(">"));
                                                        match m2239 {
                                                            Ok(v16) => {
                                                                Ok(f402(f403(Str::lit("List"), f316(v14)), v16))
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
                                        if f46(v10.clone(), f8(Str::lit("async_sequence"))) {
                                            let m2240 = f309(v9, Str::lit("<"));
                                            match m2240 {
                                                Ok(v18) => {
                                                    let m2241 = f314(v3, v18, 0u64)?;
                                                    match m2241 {
                                                        Ok(v20) => {
                                                            let m2242 = f309(f315(v20), Str::lit(">"));
                                                            match m2242 {
                                                                Ok(v22) => {
                                                                    Ok(f402(f404(f8(Str::lit("async_sequence types are not translated, because an async sequence is produced over time and is not a value (Web IDL 2.13.29)"))), v22))
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
                                            if f46(v10.clone(), f8(Str::lit("Promise"))) {
                                                let m2243 = f309(v9, Str::lit("<"));
                                                match m2243 {
                                                    Ok(v24) => {
                                                        let m2244 = f314(v3, v24, 0u64)?;
                                                        match m2244 {
                                                            Ok(v26) => {
                                                                let m2245 = f309(f315(v26), Str::lit(">"));
                                                                match m2245 {
                                                                    Ok(v28) => {
                                                                        Ok(f402(f404(f8(Str::lit("Promise types are not translated, because a promise is a deferred result, not a value (Web IDL 2.13.31)"))), v28))
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
                                                if f46(v10.clone(), f8(Str::lit("record"))) {
                                                    let m2246 = f309(v9, Str::lit("<"));
                                                    match m2246 {
                                                        Ok(v30) => {
                                                            let m2247 = v30.clone();
                                                            match m2247.uncons() {
                                                                None => {
                                                                    Ok(Err::<Adt24, Bytes>(f307(Str::lit("a string type"), v30)))
                                                                }
                                                                Some((v31, v32)) => {
                                                                    if if f46(f394(v31.clone()), f8(Str::lit("DOMString"))) {
                                                                        true
                                                                    } else {
                                                                        if f46(f394(v31.clone()), f8(Str::lit("USVString"))) {
                                                                            true
                                                                        } else {
                                                                            f46(f394(v31.clone()), f8(Str::lit("ByteString")))
                                                                        }
                                                                    } {
                                                                        let m2248 = f309(v32, Str::lit(","));
                                                                        match m2248 {
                                                                            Ok(v34) => {
                                                                                let m2249 = f314(v3, v34, 0u64)?;
                                                                                match m2249 {
                                                                                    Ok(v36) => {
                                                                                        let m2250 = f309(f315(v36.clone()), Str::lit(">"));
                                                                                        match m2250 {
                                                                                            Ok(v38) => {
                                                                                                let m2251 = f405(f394(v31));
                                                                                                match m2251 {
                                                                                                    None => {
                                                                                                        Ok(Err::<Adt24, Bytes>(f307(Str::lit("a string type"), v30)))
                                                                                                    }
                                                                                                    Some(v39) => {
                                                                                                        let v40: Adt15 = f406(f316(v36));
                                                                                                        Ok(f402(Adt15::C0(0u64, f7(List::cons(f8(Str::lit("(List (")), List::cons(f245(v39), List::cons(f8(Str::lit(" * ")), List::cons(f245(v40.clone()), List::cons(f8(Str::lit("))")), List::<Bytes>::nil())))))), f246(v40.clone()), f243(v40), Bytes::lit(&[])), v38))
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
                                                                        Ok(Err::<Adt24, Bytes>(f307(Str::lit("DOMString, USVString or ByteString"), v30)))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Err(v29) => {
                                                            Ok(Err::<Adt24, Bytes>(v29))
                                                        }
                                                    }
                                                } else {
                                                    if f46(v10.clone(), f8(Str::lit("unsigned"))) {
                                                        if f302(v9.clone(), Str::lit("short")) {
                                                            Ok(f402(f313(f8(Str::lit("Nat"))), f304(v9)))
                                                        } else {
                                                            if f302(v9.clone(), Str::lit("long")) {
                                                                let v41: List<Adt19> = f304(v9);
                                                                Ok(f402(f313(f8(Str::lit("Nat"))), if f302(v41.clone(), Str::lit("long")) {
                                                                    f304(v41)
                                                                } else {
                                                                    v41
                                                                }))
                                                            } else {
                                                                Ok(Err::<Adt24, Bytes>(f307(Str::lit("short or long"), v9)))
                                                            }
                                                        }
                                                    } else {
                                                        if f46(v10.clone(), f8(Str::lit("short"))) {
                                                            Ok(f402(f313(f8(Str::lit("Int"))), v9))
                                                        } else {
                                                            if f46(v10.clone(), f8(Str::lit("long"))) {
                                                                Ok(f402(f313(f8(Str::lit("Int"))), if f302(v9.clone(), Str::lit("long")) {
                                                                    f304(v9)
                                                                } else {
                                                                    v9
                                                                }))
                                                            } else {
                                                                if f46(v10.clone(), f8(Str::lit("unrestricted"))) {
                                                                    if if f302(v9.clone(), Str::lit("float")) {
                                                                        true
                                                                    } else {
                                                                        f302(v9.clone(), Str::lit("double"))
                                                                    } {
                                                                        Ok(f402(f404(f8(Str::lit("unrestricted float and unrestricted double are not translated, because its value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.13 and 2.13.15)"))), f304(v9)))
                                                                    } else {
                                                                        Ok(Err::<Adt24, Bytes>(f307(Str::lit("float or double"), v9)))
                                                                    }
                                                                } else {
                                                                    let m2252 = f405(v10.clone());
                                                                    match m2252 {
                                                                        None => {
                                                                            let v42: Bytes = f401(v10);
                                                                            Ok(f402(Adt15::C0(0u64, f187(v42.clone())?, List::cons(v42.clone(), List::<Bytes>::nil()), Bytes::lit(&[]), v42), v9))
                                                                        }
                                                                        Some(v43) => {
                                                                            Ok(f402(v43, v9))
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
            let m2253 = f314(v3, v1, 0u64)?;
            match m2253 {
                Ok(v45) => {
                    let v46: Adt15 = f316(v45.clone());
                    let v48: Adt15 = {
                        let a2254 = f244(v46.clone());
                        let a2255 = 0u64;
                        if nat_eq(a2254, a2255) {
                            if f46(f243(v46.clone()), Bytes::lit(&[])) {
                                if f46(f407(v46.clone()), Bytes::lit(&[])) {
                                    Adt15::C0(1u64, Bytes::lit(&[]), f246(v46), f8(Str::lit("a union member type that is not a named definition gives its constructor no name, so the union is not translated yet (Web IDL 2.13.32)")), Bytes::lit(&[]))
                                } else {
                                    let m2256 = f331(false, f407(v46.clone()))?;
                                    match m2256 {
                                        None => {
                                            Adt15::C0(1u64, Bytes::lit(&[]), f246(v46.clone()), f7(List::cons(f8(Str::lit("the union member ")), List::cons(f407(v46), List::cons(f8(Str::lit(" has no constructor name")), List::<Bytes>::nil())))), Bytes::lit(&[]))
                                        }
                                        Some(v47) => {
                                            if f229(v47.clone()) {
                                                Adt15::C0(1u64, Bytes::lit(&[]), f246(v46), f230(v47), Bytes::lit(&[]))
                                            } else {
                                                Adt15::C0(1u64, f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v47, List::cons(f8(Str::lit(", fields: [")), List::cons(f245(v46.clone()), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f246(v46), Bytes::lit(&[]), Bytes::lit(&[]))
                                            }
                                        }
                                    }
                                }
                            } else {
                                Adt15::C0(1u64, Bytes::lit(&[]), f246(v46.clone()), f243(v46), Bytes::lit(&[]))
                            }
                        } else {
                            v46
                        }
                    };
                    if f302(f315(v45.clone()), Str::lit("or")) {
                        let m2257 = f314(v3, f304(f315(v45)), 2u64)?;
                        match m2257 {
                            Ok(v50) => {
                                Ok(Ok::<Adt24, Bytes>(Adt24::C0(Adt15::C0(1u64, if f46(f245(v48.clone()), Bytes::lit(&[])) {
                                    f245(f316(v50.clone()))
                                } else {
                                    if f46(f245(f316(v50.clone())), Bytes::lit(&[])) {
                                        f245(v48.clone())
                                    } else {
                                        f7(List::cons(f245(v48.clone()), List::cons(f8(Str::lit(",\u{a}")), List::cons(f245(f316(v50.clone())), List::<Bytes>::nil()))))
                                    }
                                }, f152(f246(v48.clone()), f246(f316(v50.clone()))), if f46(f243(v48.clone()), Bytes::lit(&[])) {
                                    f243(f316(v50.clone()))
                                } else {
                                    f243(v48)
                                }, Bytes::lit(&[])), f315(v50))))
                            }
                            Err(v49) => {
                                Ok(Err::<Adt24, Bytes>(v49))
                            }
                        }
                    } else {
                        let a2258 = v2;
                        let a2259 = 2u64;
                        if if nat_eq(a2258, a2259) {
                            f302(f315(v45.clone()), Str::lit(")"))
                        } else {
                            false
                        } {
                            Ok(Ok::<Adt24, Bytes>(Adt24::C0(v48, f304(f315(v45)))))
                        } else {
                            Ok(Err::<Adt24, Bytes>({
                                let a2260 = v2;
                                let a2261 = 1u64;
                                if nat_eq(a2260, a2261) {
                                    f307(Str::lit("or"), f315(v45))
                                } else {
                                    f307(Str::lit("or or )"), f315(v45))
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

pub fn f315(v0: Adt24) -> List<Adt19> {
    let m2262 = v0;
    match m2262 {
        Adt24::C0(_, v2) => {
            v2
        }
    }
}

pub fn f316(v0: Adt24) -> Adt15 {
    let m2263 = v0;
    match m2263 {
        Adt24::C0(v1, _) => {
            v1
        }
    }
}

pub fn f317() -> List<Bytes> {
    List::cons(f8(Str::lit("Prop")), List::cons(f8(Str::lit("Sort")), List::cons(f8(Str::lit("StateRefT")), List::cons(f8(Str::lit("Type")), List::cons(f8(Str::lit("abbrev")), List::cons(f8(Str::lit("at")), List::cons(f8(Str::lit("attribute")), List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("bif")), List::cons(f8(Str::lit("break")), List::cons(f8(Str::lit("by")), List::cons(f8(Str::lit("calc")), List::cons(f8(Str::lit("catch")), List::cons(f8(Str::lit("class")), List::cons(f8(Str::lit("coinductive")), List::cons(f8(Str::lit("continue")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f318() -> List<Bytes> {
    List::cons(f8(Str::lit("def")), List::cons(f8(Str::lit("deriving")), List::cons(f8(Str::lit("do")), List::cons(f8(Str::lit("dsimproc")), List::cons(f8(Str::lit("elab")), List::cons(f8(Str::lit("else")), List::cons(f8(Str::lit("end")), List::cons(f8(Str::lit("example")), List::cons(f8(Str::lit("exists")), List::cons(f8(Str::lit("export")), List::cons(f8(Str::lit("extends")), List::cons(f8(Str::lit("finally")), List::cons(f8(Str::lit("for")), List::cons(f8(Str::lit("forall")), List::cons(f8(Str::lit("from")), List::cons(f8(Str::lit("fun")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f319() -> List<Bytes> {
    List::cons(f8(Str::lit("generalizing")), List::cons(f8(Str::lit("have")), List::cons(f8(Str::lit("haveI")), List::cons(f8(Str::lit("hiding")), List::cons(f8(Str::lit("idbg")), List::cons(f8(Str::lit("if")), List::cons(f8(Str::lit("import")), List::cons(f8(Str::lit("in")), List::cons(f8(Str::lit("include")), List::cons(f8(Str::lit("inductive")), List::cons(f8(Str::lit("inferInstanceAs")), List::cons(f8(Str::lit("infix")), List::cons(f8(Str::lit("infixl")), List::cons(f8(Str::lit("infixr")), List::cons(f8(Str::lit("initialize")), List::cons(f8(Str::lit("instance")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f320() -> List<Bytes> {
    List::cons(f8(Str::lit("let")), List::cons(f8(Str::lit("letI")), List::cons(f8(Str::lit("local")), List::cons(f8(Str::lit("logNamedError")), List::cons(f8(Str::lit("logNamedErrorAt")), List::cons(f8(Str::lit("logNamedWarning")), List::cons(f8(Str::lit("logNamedWarningAt")), List::cons(f8(Str::lit("macro")), List::cons(f8(Str::lit("match")), List::cons(f8(Str::lit("matches")), List::cons(f8(Str::lit("meta")), List::cons(f8(Str::lit("mut")), List::cons(f8(Str::lit("mutual")), List::cons(f8(Str::lit("namespace")), List::cons(f8(Str::lit("nofun")), List::cons(f8(Str::lit("nomatch")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f321() -> List<Bytes> {
    List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("nonrec")), List::cons(f8(Str::lit("notation")), List::cons(f8(Str::lit("omit")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("open")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("postfix")), List::cons(f8(Str::lit("prefix")), List::cons(f8(Str::lit("private")), List::cons(f8(Str::lit("protected")), List::cons(f8(Str::lit("public")), List::cons(f8(Str::lit("renaming")), List::cons(f8(Str::lit("repeat")), List::cons(f8(Str::lit("reprove")), List::cons(f8(Str::lit("return")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f322() -> List<Bytes> {
    List::cons(f8(Str::lit("scoped")), List::cons(f8(Str::lit("seal")), List::cons(f8(Str::lit("section")), List::cons(f8(Str::lit("show")), List::cons(f8(Str::lit("simproc")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("structure")), List::cons(f8(Str::lit("suffices")), List::cons(f8(Str::lit("syntax")), List::cons(f8(Str::lit("then")), List::cons(f8(Str::lit("theorem")), List::cons(f8(Str::lit("throwError")), List::cons(f8(Str::lit("throwErrorAt")), List::cons(f8(Str::lit("throwNamedError")), List::cons(f8(Str::lit("throwNamedErrorAt")), List::cons(f8(Str::lit("try")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f323() -> List<Bytes> {
    List::cons(f8(Str::lit("universe")), List::cons(f8(Str::lit("unless")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("unseal")), List::cons(f8(Str::lit("until")), List::cons(f8(Str::lit("using")), List::cons(f8(Str::lit("variable")), List::cons(f8(Str::lit("where")), List::cons(f8(Str::lit("while")), List::cons(f8(Str::lit("with")), List::cons(f8(Str::lit("reprove")), List::cons(f8(Str::lit("throwError")), List::cons(f8(Str::lit("throwErrorAt")), List::<Bytes>::nil())))))))))))))
}

pub fn f324(v0: List<Adt14>) -> R<List<Adt14>> {
    let m2264 = v0;
    match m2264.uncons() {
        None => {
            Ok(List::<Adt14>::nil())
        }
        Some((v1, v2)) => {
            f408(v1, f324(v2)?)
        }
    }
}

pub fn f325(v0: List<Adt14>, v1: List<Adt14>) -> List<Adt14> {
    let m2265 = v0;
    match m2265.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f325(v3, v1))
        }
    }
}

pub fn f326(v0: Adt13) -> List<Adt14> {
    let m2266 = v0;
    match m2266 {
        Adt13::C0(_, _, _, v4, _, _) => {
            v4
        }
    }
}

pub fn f327(v0: List<Adt13>, v1: u64, v2: Bytes) -> List<Adt14> {
    let m2267 = v0;
    match m2267.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v3, v4)) => {
            let a2268 = f146(v3.clone());
            let a2269 = v1;
            if if nat_eq(a2268, a2269) {
                f46(f148(v3.clone()), v2.clone())
            } else {
                false
            } {
                f325(f326(v3), f327(v4, v1, v2))
            } else {
                f327(v4, v1, v2)
            }
        }
    }
}

pub fn f328(v0: List<Adt13>, v1: List<Adt13>, v2: Bytes) -> List<Adt14> {
    let m2270 = v0;
    match m2270.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v3, v4)) => {
            let a2271 = f146(v3.clone());
            let a2272 = 8u64;
            if if nat_eq(a2271, a2272) {
                f46(f148(v3.clone()), v2.clone())
            } else {
                false
            } {
                f325(f409(v1.clone(), f159(v3)), f328(v4, v1, v2))
            } else {
                f328(v4, v1, v2)
            }
        }
    }
}

pub fn f329(v0: Adt14) -> Adt15 {
    let m2273 = v0;
    match m2273 {
        Adt14::C0(_, v2, _) => {
            v2
        }
    }
}

pub fn f330(v0: Adt14) -> Bytes {
    let m2274 = v0;
    match m2274 {
        Adt14::C0(v1, _, _) => {
            v1
        }
    }
}

pub fn f331(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f410(v1)
    } else {
        f334(v1)
    }
}

pub fn f332(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m2275 = v1;
    match m2275.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m2276 = f331(v0, v2)?;
            match m2276 {
                None => {
                    f332(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f332(v0, v3)?))
                }
            }
        }
    }
}

pub fn f333(v0: Adt14) -> u64 {
    let m2277 = v0;
    match m2277 {
        Adt14::C0(_, _, v3) => {
            v3
        }
    }
}

pub fn f334(v0: Bytes) -> R<Option<Bytes>> {
    let m2278 = f411(v0)?;
    match m2278 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            Ok(Some(f412(v1)?))
        }
    }
}

pub fn f335(v0: bool, v1: Adt2) -> R<Option<Bytes>> {
    let m2279 = v1;
    match m2279 {
        Adt2::C0(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C1(v7, v8) => {
            Ok(Some({
                let a2290 = if f82(v7, {
                    let a2288 = 0u64;
                    let a2289 = v8;
                    nat_lt(a2288, a2289)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a2291 = f209(false, v8)?;
                append_bytes(a2290, a2291)
            }))
        }
        Adt2::C2(v3, v4, v5) => {
            let m2282 = f410({
                let a2280 = f8(Str::lit("n"));
                let a2281 = f210(false, v4, v5)?;
                append_bytes(a2280, a2281)
            })?;
            match m2282 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a2286 = f8(Str::lit("nMinus"));
                        let a2287 = f43(v6.clone(), 1u64, {
                            let a2284 = {
                                let a2283 = v6;
                                length_bytes(a2283)
                            };
                            let a2285 = 1u64;
                            nat_sub(a2284, a2285)
                        });
                        append_bytes(a2286, a2287)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt2::C3(v2) => {
            f331(v0, v2)
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

pub fn f336(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m2294 = {
        let a2292 = v0.clone();
        let a2293 = v1;
        index_bytes(a2292, a2293)
    };
    match m2294 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f51(v2)?;
            let a2295 = v3;
            let a2296 = 128u64;
            if nat_lt(a2295, a2296) {
                Ok(Some((v3, 1u64)))
            } else {
                if f82({
                    let a2297 = 194u64;
                    let a2298 = v3;
                    nat_le(a2297, a2298)
                }, {
                    let a2299 = v3;
                    let a2300 = 223u64;
                    nat_le(a2299, a2300)
                }) {
                    let m2303 = f413(v0, {
                        let a2301 = v1;
                        let a2302 = 1u64;
                        nat_add(a2301, a2302)?
                    })?;
                    match m2303 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f414({
                                let a2308 = {
                                    let a2306 = {
                                        let a2304 = v3;
                                        let a2305 = 192u64;
                                        nat_sub(a2304, a2305)
                                    };
                                    let a2307 = 64u64;
                                    nat_mul(a2306, a2307)?
                                };
                                let a2309 = v4;
                                nat_add(a2308, a2309)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f82({
                        let a2310 = 224u64;
                        let a2311 = v3;
                        nat_le(a2310, a2311)
                    }, {
                        let a2312 = v3;
                        let a2313 = 239u64;
                        nat_le(a2312, a2313)
                    }) {
                        let m2316 = f413(v0.clone(), {
                            let a2314 = v1;
                            let a2315 = 1u64;
                            nat_add(a2314, a2315)?
                        })?;
                        match m2316 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m2319 = f413(v0, {
                                    let a2317 = v1;
                                    let a2318 = 2u64;
                                    nat_add(a2317, a2318)?
                                })?;
                                match m2319 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f414({
                                            let a2328 = {
                                                let a2322 = {
                                                    let a2320 = v3;
                                                    let a2321 = 224u64;
                                                    nat_sub(a2320, a2321)
                                                };
                                                let a2323 = 4096u64;
                                                nat_mul(a2322, a2323)?
                                            };
                                            let a2329 = {
                                                let a2326 = {
                                                    let a2324 = v5;
                                                    let a2325 = 64u64;
                                                    nat_mul(a2324, a2325)?
                                                };
                                                let a2327 = v6;
                                                nat_add(a2326, a2327)?
                                            };
                                            nat_add(a2328, a2329)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f82({
                            let a2330 = 240u64;
                            let a2331 = v3;
                            nat_le(a2330, a2331)
                        }, {
                            let a2332 = v3;
                            let a2333 = 244u64;
                            nat_le(a2332, a2333)
                        }) {
                            let m2336 = f413(v0.clone(), {
                                let a2334 = v1;
                                let a2335 = 1u64;
                                nat_add(a2334, a2335)?
                            })?;
                            match m2336 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m2339 = f413(v0.clone(), {
                                        let a2337 = v1;
                                        let a2338 = 2u64;
                                        nat_add(a2337, a2338)?
                                    })?;
                                    match m2339 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m2342 = f413(v0, {
                                                let a2340 = v1;
                                                let a2341 = 3u64;
                                                nat_add(a2340, a2341)?
                                            })?;
                                            match m2342 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f414({
                                                        let a2355 = {
                                                            let a2345 = {
                                                                let a2343 = v3;
                                                                let a2344 = 240u64;
                                                                nat_sub(a2343, a2344)
                                                            };
                                                            let a2346 = 262144u64;
                                                            nat_mul(a2345, a2346)?
                                                        };
                                                        let a2356 = {
                                                            let a2353 = {
                                                                let a2347 = v7;
                                                                let a2348 = 4096u64;
                                                                nat_mul(a2347, a2348)?
                                                            };
                                                            let a2354 = {
                                                                let a2351 = {
                                                                    let a2349 = v8;
                                                                    let a2350 = 64u64;
                                                                    nat_mul(a2349, a2350)?
                                                                };
                                                                let a2352 = v9;
                                                                nat_add(a2351, a2352)?
                                                            };
                                                            nat_add(a2353, a2354)?
                                                        };
                                                        nat_add(a2355, a2356)?
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

pub fn f337(v0: u64) -> R<bool> {
    f251(v0, f415())
}

pub fn f338(v0: u64, v1: Bytes, p2367: u64, p2368: u64, p2369: bool) -> R<bool> {
    let m2366 = (p2367, (p2368, p2369));
    {
        let mut m2366 = m2366;
        loop {
            let step = {
                let (v2, (v3, v4)) = m2366;
                let m2357 = v3;
                if m2357 == 0 {
                    Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                } else {
                    let v5 = m2357 - 1;
                    let v6: u64 = f416(v1.clone(), v2)?;
                    if v4 {
                        let a2358 = v0;
                        let a2359 = v6;
                        if nat_lt(a2358, a2359) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a2360 = v2;
                                let a2361 = 1u64;
                                nat_add(a2360, a2361)?
                            }, (v5, false)))
                        }
                    } else {
                        let a2362 = v0;
                        let a2363 = v6;
                        if nat_le(a2362, a2363) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(true))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a2364 = v2;
                                let a2365 = 1u64;
                                nat_add(a2364, a2365)?
                            }, (v5, true)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2366 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f339(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    if f104(v0.clone(), v1, v2) {
        Ok(None::<u64>)
    } else {
        let a2371 = v2;
        let a2372 = {
            let a2370 = v0;
            length_bytes(a2370)
        };
        if nat_lt(a2371, a2372) {
            Ok(Some({
                let a2373 = v2;
                let a2374 = 1u64;
                nat_add(a2373, a2374)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f340(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f166(f112(v0, v1)?)? {
        Ok(Some({
            let a2375 = v1;
            let a2376 = 1u64;
            nat_add(a2375, a2376)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f341(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let m2377 = f336(v0, v2)?;
    match m2377 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            let a2378 = v2;
            let a2379 = v1;
            if if nat_eq(a2378, a2379) {
                f417({
                    let (h2380, _) = v3;
                    h2380
                })?
            } else {
                f418({
                    let (h2381, _) = v3;
                    h2381
                })?
            } {
                Ok(Some({
                    let a2383 = v2;
                    let a2384 = {
                        let (_, h2382) = v3;
                        h2382
                    };
                    nat_add(a2383, a2384)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f342(v0: Bytes, v1: bool, v2: u64) -> R<(u64, u64)> {
    let (h2385, _) = f220(Fn5::F419(v1, v0.clone()), {
        let a2387 = {
            let a2386 = v0;
            length_bytes(a2386)
        };
        let a2388 = 1u64;
        nat_add(a2387, a2388)?
    }, (v2, 0u64))?;
    Ok(h2385)
}

pub fn f343(v0: u64) -> R<Bytes> {
    let a2389 = v0;
    let a2390 = 128u64;
    if nat_lt(a2389, a2390) {
        Ok(f420(f421(), v0))
    } else {
        let a2391 = v0;
        let a2392 = 2048u64;
        if nat_lt(a2391, a2392) {
            let a2400 = f420(f422(), {
                let a2398 = {
                    let a2396 = {
                        let a2393 = v0;
                        let a2394 = 64u64;
                        let a2395 = 0u64;
                        nat_quot(a2393, a2394, a2395)
                    };
                    let a2397 = 2u64;
                    nat_sub(a2396, a2397)
                };
                let a2399 = 2u64;
                nat_mul(a2398, a2399)?
            });
            let a2401 = f423(v0)?;
            Ok(append_bytes(a2400, a2401))
        } else {
            let a2402 = v0;
            let a2403 = 65536u64;
            if nat_lt(a2402, a2403) {
                let a2414 = f420(f424(), {
                    let a2407 = {
                        let a2404 = v0;
                        let a2405 = 4096u64;
                        let a2406 = 0u64;
                        nat_quot(a2404, a2405, a2406)
                    };
                    let a2408 = 3u64;
                    nat_mul(a2407, a2408)?
                });
                let a2415 = {
                    let a2412 = f423({
                        let a2409 = v0;
                        let a2410 = 64u64;
                        let a2411 = 0u64;
                        nat_quot(a2409, a2410, a2411)
                    })?;
                    let a2413 = f423(v0)?;
                    append_bytes(a2412, a2413)
                };
                Ok(append_bytes(a2414, a2415))
            } else {
                let a2429 = f420(Bytes::lit(&[0xf0, 0xf1, 0xf2, 0xf3, 0xf4]), {
                    let a2416 = v0;
                    let a2417 = 262144u64;
                    let a2418 = 0u64;
                    nat_quot(a2416, a2417, a2418)
                });
                let a2430 = {
                    let a2427 = f423({
                        let a2419 = v0;
                        let a2420 = 4096u64;
                        let a2421 = 0u64;
                        nat_quot(a2419, a2420, a2421)
                    })?;
                    let a2428 = {
                        let a2425 = f423({
                            let a2422 = v0;
                            let a2423 = 64u64;
                            let a2424 = 0u64;
                            nat_quot(a2422, a2423, a2424)
                        })?;
                        let a2426 = f423(v0)?;
                        append_bytes(a2425, a2426)
                    };
                    append_bytes(a2427, a2428)
                };
                Ok(append_bytes(a2429, a2430))
            }
        }
    }
}

pub fn f344(v0: bool, v1: u64, v2: Bytes, v3: u64) -> R<Option<u64>> {
    f425(v2, v0, v1, v3)
}

pub fn f345(v0: Bytes) -> Result<Option<Adt21>, Bytes> {
    Err::<Option<Adt21>, Bytes>(v0)
}

pub fn f346(p2434: Bytes, p2435: u64) -> R<Result<Bytes, Bytes>> {
    let m2433 = (p2434, p2435);
    {
        let mut m2433 = m2433;
        loop {
            let step = {
                let (v0, v1) = m2433;
                let m2431 = v1;
                if m2431 == 0 {
                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f426(f8(Str::lit("the built-in datatype hierarchy is deeper than XSD 1.1 Part 2 section 3 defines")))))
                } else {
                    let v2 = m2431 - 1;
                    if f207(v0.clone(), List::cons(f8(Str::lit("NMTOKENS")), List::cons(f8(Str::lit("IDREFS")), List::cons(f8(Str::lit("ENTITIES")), List::<Bytes>::nil())))) {
                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("(List Bytes)")))))
                    } else {
                        if f46(v0.clone(), f8(Str::lit("nonNegativeInteger"))) {
                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Nat")))))
                        } else {
                            if f46(v0.clone(), f8(Str::lit("integer"))) {
                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Int")))))
                            } else {
                                if f46(v0.clone(), f8(Str::lit("decimal"))) {
                                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f427())))
                                } else {
                                    if f46(v0.clone(), f8(Str::lit("boolean"))) {
                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Bool")))))
                                    } else {
                                        if f207(v0.clone(), List::cons(f8(Str::lit("float")), List::cons(f8(Str::lit("double")), List::<Bytes>::nil()))) {
                                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f426(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because its value space holds positive and negative zero, the infinities and NaN, which no exact LexLean number type holds (XSD 1.1 Part 2, sections 3.3.4 and 3.3.5)")), List::<Bytes>::nil())))))))
                                        } else {
                                            if f207(v0.clone(), List::cons(f8(Str::lit("anySimpleType")), List::cons(f8(Str::lit("anyAtomicType")), List::<Bytes>::nil()))) {
                                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f426(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because it is the union of every simple value space and has no one LexLean type (XSD 1.1 Part 2, sections 3.2.1 and 3.2.2)")), List::<Bytes>::nil())))))))
                                            } else {
                                                let m2432 = f428(v0.clone())?;
                                                match m2432 {
                                                    None => {
                                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f426(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not a built-in datatype of XSD 1.1 Part 2")), List::<Bytes>::nil())))))))
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
                Ok(next) => { m2433 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f347(v0: Bytes) -> Result<List<Adt2>, Bytes> {
    Err::<List<Adt2>, Bytes>(v0)
}

pub fn f348(v0: Option<Adt2>) -> Option<Bytes> {
    let m2436 = v0;
    match m2436 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m2437 = v1;
            match m2437 {
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

pub fn f349(v0: u64) -> bool {
    f135({
        let a2438 = v0;
        let a2439 = 95u64;
        nat_eq(a2438, a2439)
    }, f135({
        let a2440 = v0;
        let a2441 = 45u64;
        nat_eq(a2440, a2441)
    }, f135({
        let a2442 = v0;
        let a2443 = 32u64;
        nat_eq(a2442, a2443)
    }, f135({
        let a2444 = v0;
        let a2445 = 46u64;
        nat_eq(a2444, a2445)
    }, {
        let a2446 = v0;
        let a2447 = 47u64;
        nat_eq(a2446, a2447)
    }))))
}

pub fn f350(v0: u64) -> bool {
    f82({
        let a2448 = 97u64;
        let a2449 = v0;
        nat_le(a2448, a2449)
    }, {
        let a2450 = v0;
        let a2451 = 122u64;
        nat_le(a2450, a2451)
    })
}

pub fn f351(v0: u64) -> bool {
    f82({
        let a2452 = 65u64;
        let a2453 = v0;
        nat_le(a2452, a2453)
    }, {
        let a2454 = v0;
        let a2455 = 90u64;
        nat_le(a2454, a2455)
    })
}

pub fn f352(v0: u64) -> bool {
    f82({
        let a2456 = 48u64;
        let a2457 = v0;
        nat_le(a2456, a2457)
    }, {
        let a2458 = v0;
        let a2459 = 57u64;
        nat_le(a2458, a2459)
    })
}

pub fn f353(v0: u64) -> bool {
    let a2460 = v0;
    let a2461 = 9u64;
    if if if nat_eq(a2460, a2461) {
        true
    } else {
        let a2462 = v0;
        let a2463 = 10u64;
        nat_eq(a2462, a2463)
    } {
        true
    } else {
        let a2464 = v0;
        let a2465 = 13u64;
        if nat_eq(a2464, a2465) {
            true
        } else {
            let a2466 = 32u64;
            let a2467 = v0;
            if nat_le(a2466, a2467) {
                let a2468 = v0;
                let a2469 = 126u64;
                nat_le(a2468, a2469)
            } else {
                false
            }
        }
    } {
        true
    } else {
        let a2470 = v0;
        let a2471 = 133u64;
        if if nat_eq(a2470, a2471) {
            true
        } else {
            let a2472 = 160u64;
            let a2473 = v0;
            if nat_le(a2472, a2473) {
                let a2474 = v0;
                let a2475 = 55295u64;
                nat_le(a2474, a2475)
            } else {
                false
            }
        } {
            true
        } else {
            let a2476 = 57344u64;
            let a2477 = v0;
            if if nat_le(a2476, a2477) {
                let a2478 = v0;
                let a2479 = 65533u64;
                nat_le(a2478, a2479)
            } else {
                false
            } {
                true
            } else {
                let a2480 = 65536u64;
                let a2481 = v0;
                if nat_le(a2480, a2481) {
                    let a2482 = v0;
                    let a2483 = 1114111u64;
                    nat_le(a2482, a2483)
                } else {
                    false
                }
            }
        }
    }
}

pub fn f354(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f196(v0.clone(), v1)?;
    if f193(f127(v0.clone(), v2)?) {
        Ok(true)
    } else {
        let a2484 = f127(v0, v2)?;
        let a2485 = 35u64;
        Ok(nat_eq(a2484, a2485))
    }
}

pub fn f355(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f429(v0, v1)
}

pub fn f356(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2486 = f127(v0, v1)?;
    let a2487 = 32u64;
    if nat_eq(a2486, a2487) {
        Ok(Some({
            let a2488 = v1;
            let a2489 = 1u64;
            nat_add(a2488, a2489)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f357(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f192(f127(v0, v1)?) {
        Ok(Some({
            let a2490 = v1;
            let a2491 = 1u64;
            nat_add(a2490, a2491)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f358(v0: u64) -> bool {
    let a2492 = v0;
    let a2493 = 44u64;
    if if if nat_eq(a2492, a2493) {
        true
    } else {
        let a2494 = v0;
        let a2495 = 91u64;
        nat_eq(a2494, a2495)
    } {
        true
    } else {
        let a2496 = v0;
        let a2497 = 93u64;
        if nat_eq(a2496, a2497) {
            true
        } else {
            let a2498 = v0;
            let a2499 = 123u64;
            nat_eq(a2498, a2499)
        }
    } {
        true
    } else {
        let a2500 = v0;
        let a2501 = 125u64;
        nat_eq(a2500, a2501)
    }
}

pub fn f359(v0: u64) -> u64 {
    let a2502 = v0;
    let a2503 = 45u64;
    if nat_eq(a2502, a2503) {
        1u64
    } else {
        let a2504 = v0;
        let a2505 = 43u64;
        if nat_eq(a2504, a2505) {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f360(v0: u64) -> u64 {
    let a2506 = 49u64;
    let a2507 = v0;
    if if nat_le(a2506, a2507) {
        let a2508 = v0;
        let a2509 = 57u64;
        nat_le(a2508, a2509)
    } else {
        false
    } {
        let a2510 = v0;
        let a2511 = 48u64;
        nat_sub(a2510, a2511)
    } else {
        0u64
    }
}

pub fn f361(v0: Bytes, v1: u64, v2: u64, v3: bool, v4: u64) -> R<Adt18> {
    let m2512 = f430(v0.clone(), v1, v2)?;
    match m2512 {
        Adt25::C0(v6, v7) => {
            let v8: u64 = v7;
            let v9: u64 = f431(v0.clone(), v8, v2)?;
            if f432(v0.clone(), v8, v2)? {
                let m2516 = f433(v0.clone(), v8, v2, v3, {
                    let a2514 = {
                        let a2513 = v0;
                        length_bytes(a2513)
                    };
                    let a2515 = 2u64;
                    nat_add(a2514, a2515)?
                }, Bytes::lit(&[]), v6)?;
                match m2516 {
                    Adt26::C0(v11, v12, v13) => {
                        Ok(Adt18::C0(Adt2::C3(f7(List::cons(v11, List::cons({
                            let a2517 = v4;
                            let a2518 = 1u64;
                            if nat_eq(a2517, a2518) {
                                Bytes::lit(&[])
                            } else {
                                Bytes::lit(&[0x0a])
                            }
                        }, List::cons({
                            let a2519 = v4;
                            let a2520 = 2u64;
                            if nat_eq(a2519, a2520) {
                                f363(v12)
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
                Ok(Adt18::C0(Adt2::C3({
                    let a2521 = v4;
                    let a2522 = 2u64;
                    if nat_eq(a2521, a2522) {
                        f363(v6)
                    } else {
                        Bytes::lit(&[])
                    }
                }), {
                    let a2523 = f127(v0, v9)?;
                    let a2524 = 256u64;
                    if nat_eq(a2523, a2524) {
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

pub fn f362(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f434(v0, v1)
}

pub fn f363(v0: u64) -> Bytes {
    let m2525 = v0;
    if m2525 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2525 - 1;
        let a2526 = Bytes::lit(&[0x0a]);
        let a2527 = f363(v1);
        append_bytes(a2526, a2527)
    }
}

pub fn f364(v0: Bytes, v1: u64) -> R<u64> {
    let (_, h2528) = {
        let (h2529, _) = f220(Fn5::F435(v0.clone()), {
            let a2531 = {
                let a2530 = v0;
                length_bytes(a2530)
            };
            let a2532 = 1u64;
            nat_add(a2531, a2532)?
        }, (v1, 0u64))?;
        h2529
    };
    Ok(h2528)
}

pub fn f365(v0: u64, v1: u64) -> u64 {
    let a2533 = v0;
    let a2534 = v1;
    if nat_lt(a2533, a2534) {
        v1
    } else {
        v0
    }
}

pub fn f366(v0: Bytes, p2593: u64, v2: u64, v3: bool, p2594: u64, p2595: Bytes) -> R<Adt22> {
    let m2592 = (p2593, (p2594, p2595));
    {
        let mut m2592 = m2592;
        loop {
            let step = {
                let (v1, (v4, v5)) = m2592;
                let m2535 = v4;
                if m2535 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C1(f108(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                } else {
                    let v6 = m2535 - 1;
                    let v7: u64 = f127(v0.clone(), v1)?;
                    let a2536 = v7;
                    let a2537 = 34u64;
                    if nat_eq(a2536, a2537) {
                        Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C0(v5, {
                            let a2538 = v1;
                            let a2539 = 1u64;
                            nat_add(a2538, a2539)?
                        })))
                    } else {
                        let a2540 = v7;
                        let a2541 = 256u64;
                        if nat_eq(a2540, a2541) {
                            Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C1(f108(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                        } else {
                            let a2542 = v7;
                            let a2543 = 92u64;
                            if nat_eq(a2542, a2543) {
                                let v8: u64 = f127(v0.clone(), {
                                    let a2544 = v1;
                                    let a2545 = 1u64;
                                    nat_add(a2544, a2545)?
                                })?;
                                let m2546 = f436(v8)?;
                                match m2546 {
                                    None => {
                                        let a2551 = v8;
                                        let a2552 = 10u64;
                                        if nat_eq(a2551, a2552) {
                                            if v3 {
                                                Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C1(f437(v1)?)))
                                            } else {
                                                let m2555 = f438(v0.clone(), {
                                                    let a2553 = v1;
                                                    let a2554 = 1u64;
                                                    nat_add(a2553, a2554)?
                                                }, v2, v6, 0u64)?;
                                                match m2555 {
                                                    Adt25::C0(v11, v12) => {
                                                        Ok::<(u64, (u64, Bytes)), R<Adt22>>((v12, (v6, {
                                                            let a2556 = v5;
                                                            let a2557 = f363(v11);
                                                            append_bytes(a2556, a2557)
                                                        })))
                                                    }
                                                    Adt25::C1(v10) => {
                                                        Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C1(v10)))
                                                    }
                                                }
                                            }
                                        } else {
                                            let v13: u64 = {
                                                let a2558 = v8;
                                                let a2559 = 120u64;
                                                if nat_eq(a2558, a2559) {
                                                    2u64
                                                } else {
                                                    let a2560 = v8;
                                                    let a2561 = 117u64;
                                                    if nat_eq(a2560, a2561) {
                                                        4u64
                                                    } else {
                                                        let a2562 = v8;
                                                        let a2563 = 85u64;
                                                        if nat_eq(a2562, a2563) {
                                                            8u64
                                                        } else {
                                                            0u64
                                                        }
                                                    }
                                                }
                                            };
                                            let a2564 = v13;
                                            let a2565 = 0u64;
                                            if nat_eq(a2564, a2565) {
                                                Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C1(f108(Str::lit("an escape must be one of YAML 1.2.2 [42] to [62]"), v1)?)))
                                            } else {
                                                let m2568 = f439(v0.clone(), {
                                                    let a2566 = v1;
                                                    let a2567 = 2u64;
                                                    nat_add(a2566, a2567)?
                                                }, v13, 0u64)?;
                                                match m2568 {
                                                    None => {
                                                        Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C1(f108(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                    }
                                                    Some(v14) => {
                                                        let a2569 = 1114111u64;
                                                        let a2570 = v14;
                                                        if if nat_lt(a2569, a2570) {
                                                            true
                                                        } else {
                                                            let a2571 = 55296u64;
                                                            let a2572 = v14;
                                                            if nat_le(a2571, a2572) {
                                                                let a2573 = v14;
                                                                let a2574 = 57343u64;
                                                                nat_le(a2573, a2574)
                                                            } else {
                                                                false
                                                            }
                                                        } {
                                                            Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C1(f108(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                        } else {
                                                            Ok::<(u64, (u64, Bytes)), R<Adt22>>(({
                                                                let a2577 = {
                                                                    let a2575 = v1;
                                                                    let a2576 = 2u64;
                                                                    nat_add(a2575, a2576)?
                                                                };
                                                                let a2578 = v13;
                                                                nat_add(a2577, a2578)?
                                                            }, (v6, {
                                                                let a2579 = v5;
                                                                let a2580 = f343(v14)?;
                                                                append_bytes(a2579, a2580)
                                                            })))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Some(v9) => {
                                        Ok::<(u64, (u64, Bytes)), R<Adt22>>(({
                                            let a2547 = v1;
                                            let a2548 = 2u64;
                                            nat_add(a2547, a2548)?
                                        }, (v6, {
                                            let a2549 = v5;
                                            let a2550 = f343(v9)?;
                                            append_bytes(a2549, a2550)
                                        })))
                                    }
                                }
                            } else {
                                if if f192(v7) {
                                    true
                                } else {
                                    let a2581 = v7;
                                    let a2582 = 10u64;
                                    nat_eq(a2581, a2582)
                                } {
                                    let v15: u64 = f196(v0.clone(), v1)?;
                                    let a2583 = f127(v0.clone(), v15)?;
                                    let a2584 = 10u64;
                                    if nat_eq(a2583, a2584) {
                                        if v3 {
                                            Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C1(f437(v1)?)))
                                        } else {
                                            let m2585 = f438(v0.clone(), v15, v2, v6, 0u64)?;
                                            match m2585 {
                                                Adt25::C0(v17, v18) => {
                                                    Ok::<(u64, (u64, Bytes)), R<Adt22>>((v18, (v6, {
                                                        let a2586 = v5;
                                                        let a2587 = f374(v17);
                                                        append_bytes(a2586, a2587)
                                                    })))
                                                }
                                                Adt25::C1(v16) => {
                                                    Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C1(v16)))
                                                }
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (u64, Bytes)), R<Adt22>>((v15, (v6, {
                                            let a2588 = v5;
                                            let a2589 = f279(v0.clone(), v1, v15);
                                            append_bytes(a2588, a2589)
                                        })))
                                    }
                                } else {
                                    let v19: u64 = f440(v0.clone(), v1)?;
                                    Ok::<(u64, (u64, Bytes)), R<Adt22>>((v19, (v6, {
                                        let a2590 = v5;
                                        let a2591 = f279(v0.clone(), v1, v19);
                                        append_bytes(a2590, a2591)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2592 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f367(v0: Bytes, p2623: u64, v2: u64, v3: bool, p2624: u64, p2625: Bytes) -> R<Adt22> {
    let m2622 = (p2623, (p2624, p2625));
    {
        let mut m2622 = m2622;
        loop {
            let step = {
                let (v1, (v4, v5)) = m2622;
                let m2596 = v4;
                if m2596 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C1(f108(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                } else {
                    let v6 = m2596 - 1;
                    let v7: u64 = f127(v0.clone(), v1)?;
                    let a2597 = v7;
                    let a2598 = 39u64;
                    if nat_eq(a2597, a2598) {
                        let a2601 = f127(v0.clone(), {
                            let a2599 = v1;
                            let a2600 = 1u64;
                            nat_add(a2599, a2600)?
                        })?;
                        let a2602 = 39u64;
                        if nat_eq(a2601, a2602) {
                            Ok::<(u64, (u64, Bytes)), R<Adt22>>(({
                                let a2603 = v1;
                                let a2604 = 2u64;
                                nat_add(a2603, a2604)?
                            }, (v6, {
                                let a2605 = v5;
                                let a2606 = Bytes::lit(&[0x27]);
                                append_bytes(a2605, a2606)
                            })))
                        } else {
                            Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C0(v5, {
                                let a2607 = v1;
                                let a2608 = 1u64;
                                nat_add(a2607, a2608)?
                            })))
                        }
                    } else {
                        let a2609 = v7;
                        let a2610 = 256u64;
                        if nat_eq(a2609, a2610) {
                            Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C1(f108(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                        } else {
                            if if f192(v7) {
                                true
                            } else {
                                let a2611 = v7;
                                let a2612 = 10u64;
                                nat_eq(a2611, a2612)
                            } {
                                let v8: u64 = f196(v0.clone(), v1)?;
                                let a2613 = f127(v0.clone(), v8)?;
                                let a2614 = 10u64;
                                if nat_eq(a2613, a2614) {
                                    if v3 {
                                        Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C1(f437(v1)?)))
                                    } else {
                                        let m2615 = f438(v0.clone(), v8, v2, v6, 0u64)?;
                                        match m2615 {
                                            Adt25::C0(v10, v11) => {
                                                Ok::<(u64, (u64, Bytes)), R<Adt22>>((v11, (v6, {
                                                    let a2616 = v5;
                                                    let a2617 = f374(v10);
                                                    append_bytes(a2616, a2617)
                                                })))
                                            }
                                            Adt25::C1(v9) => {
                                                Err::<(u64, (u64, Bytes)), R<Adt22>>(Ok(Adt22::C1(v9)))
                                            }
                                        }
                                    }
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Adt22>>((v8, (v6, {
                                        let a2618 = v5;
                                        let a2619 = f279(v0.clone(), v1, v8);
                                        append_bytes(a2618, a2619)
                                    })))
                                }
                            } else {
                                let v12: u64 = f441(v0.clone(), v1)?;
                                Ok::<(u64, (u64, Bytes)), R<Adt22>>((v12, (v6, {
                                    let a2620 = v5;
                                    let a2621 = f279(v0.clone(), v1, v12);
                                    append_bytes(a2620, a2621)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2622 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f368(v0: Bytes, v1: u64, v2: u64) -> R<Result<u64, Bytes>> {
    let v3: (u64, bool) = {
        let (h2626, _) = f442(Fn8::F443(v2, v0.clone()), {
            let a2628 = {
                let a2627 = v0.clone();
                length_bytes(a2627)
            };
            let a2629 = 1u64;
            nat_add(a2628, a2629)?
        }, (v1, false))?;
        h2626
    };
    let (_, h2630) = v3;
    if h2630 {
        if f126(v0, {
            let (h2631, _) = v3;
            h2631
        })? {
            Ok(Err::<u64, Bytes>(f108(Str::lit("a document marker cannot appear inside a flow collection (YAML 1.2.2 [206])"), {
                let (h2632, _) = v3;
                h2632
            })?))
        } else {
            Ok(Err::<u64, Bytes>(f108(Str::lit("a line of a flow collection is indented less than its node (YAML 1.2.2 [80])"), {
                let (h2633, _) = v3;
                h2633
            })?))
        }
    } else {
        Ok(Ok::<u64, Bytes>({
            let (h2634, _) = v3;
            h2634
        }))
    }
}

pub fn f369(v0: Bytes, v1: u64, v2: u64) -> R<Adt22> {
    let v3: u64 = f127(v0.clone(), v1)?;
    let a2635 = v3;
    let a2636 = 34u64;
    if if nat_eq(a2635, a2636) {
        true
    } else {
        let a2637 = v3;
        let a2638 = 39u64;
        nat_eq(a2637, a2638)
    } {
        f274(v0, v1, v2, false)
    } else {
        let m2639 = f199(v0.clone(), v1)?;
        match m2639 {
            None => {
                let a2640 = v3;
                let a2641 = 91u64;
                if if nat_eq(a2640, a2641) {
                    true
                } else {
                    let a2642 = v3;
                    let a2643 = 123u64;
                    nat_eq(a2642, a2643)
                } {
                    Ok(Adt22::C1(f108(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [142])"), v1)?))
                } else {
                    if f276(v0.clone(), v1, true)? {
                        let v5: u64 = f277(v0.clone(), v1, true)?;
                        Ok(Adt22::C0(f279(v0, v1, v5), v5))
                    } else {
                        Ok(Adt22::C1(f108(Str::lit("a flow mapping key must be a scalar (YAML 1.2.2 [142])"), v1)?))
                    }
                }
            }
            Some(v4) => {
                Ok(Adt22::C1(v4))
            }
        }
    }
}

pub fn f370(v0: u64) -> R<bool> {
    let a2644 = f444(f8(Str::lit(",[]{}-?:#&*!|>'\"@`")), v0)?;
    let a2645 = 18u64;
    if nat_lt(a2644, a2645) {
        Ok(true)
    } else {
        let a2646 = v0;
        let a2647 = 37u64;
        Ok(nat_eq(a2646, a2647))
    }
}

pub fn f371(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f445(v1, v0, v2)
}

pub fn f372(v0: u64) -> R<Bytes> {
    f108(Str::lit("a mapping value is not allowed here: a plain scalar holds ': ' (YAML 1.2.2 [130])"), v0)
}

pub fn f373(v0: Bytes, v1: u64) -> R<(u64, u64)> {
    let (h2648, _) = f220(Fn5::F446(v0.clone()), {
        let a2650 = {
            let a2649 = v0;
            length_bytes(a2649)
        };
        let a2651 = 1u64;
        nat_add(a2650, a2651)?
    }, (v1, 0u64))?;
    Ok(h2648)
}

pub fn f374(v0: u64) -> Bytes {
    let a2652 = v0;
    let a2653 = 0u64;
    if nat_eq(a2652, a2653) {
        Bytes::lit(&[0x20])
    } else {
        f363(v0)
    }
}

pub fn f375(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt18> {
    let v4: (u64, u64) = f447(v0.clone(), v1, 2u64)?;
    let a2656 = {
        let (h2654, _) = v4;
        h2654
    };
    let a2657 = {
        let a2655 = v0.clone();
        length_bytes(a2655)
    };
    if nat_eq(a2656, a2657) {
        if f139(false, {
            let (_, h2658) = v4;
            h2658
        }) {
            Ok(Adt18::C0(Adt2::C1(false, {
                let (_, h2659) = v4;
                h2659
            }), v3))
        } else {
            Ok(Adt18::C1(f377(v2)?))
        }
    } else {
        Ok(Adt18::C0(Adt2::C3(v0), v3))
    }
}

pub fn f376(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2660 = v0.clone();
        length_bytes(a2660)
    };
    let v2: u64 = f127(v0.clone(), 0u64)?;
    let v3: u64 = {
        let a2661 = v2;
        let a2662 = 43u64;
        if if nat_eq(a2661, a2662) {
            true
        } else {
            let a2663 = v2;
            let a2664 = 45u64;
            nat_eq(a2663, a2664)
        } {
            1u64
        } else {
            0u64
        }
    };
    let v4: u64 = f134(v0.clone(), v3)?;
    let v5: bool = {
        let a2665 = f127(v0.clone(), v4)?;
        let a2666 = 46u64;
        nat_eq(a2665, a2666)
    };
    let v6: u64 = if v5 {
        let a2667 = v4;
        let a2668 = 1u64;
        nat_add(a2667, a2668)?
    } else {
        v4
    };
    let v7: u64 = if v5 {
        f134(v0.clone(), v6)?
    } else {
        v4
    };
    let v8: u64 = f127(v0.clone(), v7)?;
    let v9: bool = {
        let a2669 = v8;
        let a2670 = 101u64;
        if nat_eq(a2669, a2670) {
            true
        } else {
            let a2671 = v8;
            let a2672 = 69u64;
            nat_eq(a2671, a2672)
        }
    };
    let v10: u64 = f127(v0.clone(), {
        let a2673 = v7;
        let a2674 = 1u64;
        nat_add(a2673, a2674)?
    })?;
    let v11: bool = if v9 {
        let a2675 = v10;
        let a2676 = 43u64;
        if nat_eq(a2675, a2676) {
            true
        } else {
            let a2677 = v10;
            let a2678 = 45u64;
            nat_eq(a2677, a2678)
        }
    } else {
        false
    };
    let v12: u64 = if v9 {
        if v11 {
            let a2679 = v7;
            let a2680 = 2u64;
            nat_add(a2679, a2680)?
        } else {
            let a2681 = v7;
            let a2682 = 1u64;
            nat_add(a2681, a2682)?
        }
    } else {
        v7
    };
    let v13: u64 = if v9 {
        f134(v0.clone(), v12)?
    } else {
        v7
    };
    let v14: Bytes = f279(v0.clone(), v3, v4);
    let v15: u64 = f136(v14.clone())?;
    let a2683 = v13;
    let a2684 = v1;
    if if if nat_eq(a2683, a2684) {
        let a2685 = v3;
        let a2686 = v4;
        if nat_lt(a2685, a2686) {
            true
        } else {
            if v5 {
                let a2687 = v6;
                let a2688 = v7;
                nat_lt(a2687, a2688)
            } else {
                false
            }
        }
    } else {
        false
    } {
        if f204(v9) {
            true
        } else {
            let a2689 = v12;
            let a2690 = v13;
            nat_lt(a2689, a2690)
        }
    } else {
        false
    } {
        Ok(Some(f7(List::cons({
            let a2691 = v2;
            let a2692 = 45u64;
            if nat_eq(a2691, a2692) {
                Bytes::lit(&[0x2d])
            } else {
                Bytes::lit(&[])
            }
        }, List::cons({
            let a2694 = v15;
            let a2695 = {
                let a2693 = v14.clone();
                length_bytes(a2693)
            };
            if nat_eq(a2694, a2695) {
                Bytes::lit(&[0x30])
            } else {
                f279(v14.clone(), v15, {
                    let a2696 = v14;
                    length_bytes(a2696)
                })
            }
        }, List::cons({
            let a2697 = v6;
            let a2698 = v7;
            if nat_lt(a2697, a2698) {
                let a2699 = Bytes::lit(&[0x2e]);
                let a2700 = f279(v0.clone(), v6, v7);
                append_bytes(a2699, a2700)
            } else {
                Bytes::lit(&[])
            }
        }, List::cons(if v9 {
            let a2703 = if if v11 {
                let a2701 = v10;
                let a2702 = 45u64;
                nat_eq(a2701, a2702)
            } else {
                false
            } {
                Bytes::lit(&[0x65, 0x2d])
            } else {
                Bytes::lit(&[0x65])
            };
            let a2704 = f279(v0, v12, v13);
            append_bytes(a2703, a2704)
        } else {
            Bytes::lit(&[])
        }, List::<Bytes>::nil())))))))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f377(v0: u64) -> R<Bytes> {
    f108(Str::lit("a number outside the range Studio.Value holds is not read (YAML 1.2.2 section 10.3.2)"), v0)
}

pub fn f378(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f448(v0, v1)
}

pub fn f379(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a2749 = {
            let a2725 = {
                let a2713 = {
                    let a2707 = {
                        let a2705 = v3;
                        let a2706 = 128u8;
                        fixed_u8::bit_and(a2705, a2706)
                    };
                    let a2708 = 128u8;
                    if equal(a2707, a2708) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2714 = {
                    let a2711 = {
                        let a2709 = v3;
                        let a2710 = 64u8;
                        fixed_u8::bit_and(a2709, a2710)
                    };
                    let a2712 = 64u8;
                    if equal(a2711, a2712) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2713, a2714)?
            };
            let a2726 = {
                let a2723 = {
                    let a2717 = {
                        let a2715 = v3;
                        let a2716 = 32u8;
                        fixed_u8::bit_and(a2715, a2716)
                    };
                    let a2718 = 32u8;
                    if equal(a2717, a2718) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2724 = {
                    let a2721 = {
                        let a2719 = v3;
                        let a2720 = 16u8;
                        fixed_u8::bit_and(a2719, a2720)
                    };
                    let a2722 = 16u8;
                    if equal(a2721, a2722) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2723, a2724)?
            };
            nat_add(a2725, a2726)?
        };
        let a2750 = {
            let a2747 = {
                let a2735 = {
                    let a2729 = {
                        let a2727 = v3;
                        let a2728 = 8u8;
                        fixed_u8::bit_and(a2727, a2728)
                    };
                    let a2730 = 8u8;
                    if equal(a2729, a2730) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2736 = {
                    let a2733 = {
                        let a2731 = v3;
                        let a2732 = 4u8;
                        fixed_u8::bit_and(a2731, a2732)
                    };
                    let a2734 = 4u8;
                    if equal(a2733, a2734) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2735, a2736)?
            };
            let a2748 = {
                let a2745 = {
                    let a2739 = {
                        let a2737 = v3;
                        let a2738 = 2u8;
                        fixed_u8::bit_and(a2737, a2738)
                    };
                    let a2740 = 2u8;
                    if equal(a2739, a2740) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2746 = {
                    let a2743 = {
                        let a2741 = v3;
                        let a2742 = 1u8;
                        fixed_u8::bit_and(a2741, a2742)
                    };
                    let a2744 = 1u8;
                    if equal(a2743, a2744) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2745, a2746)?
            };
            nat_add(a2747, a2748)?
        };
        nat_add(a2749, a2750)?
    };
    let a2751 = v4;
    let a2752 = 34u64;
    if nat_eq(a2751, a2752) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a2753 = v4;
        let a2754 = 92u64;
        if nat_eq(a2753, a2754) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a2755 = v4;
            let a2756 = 8u64;
            if nat_eq(a2755, a2756) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a2757 = v4;
                let a2758 = 12u64;
                if nat_eq(a2757, a2758) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a2759 = v4;
                    let a2760 = 10u64;
                    if nat_eq(a2759, a2760) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a2761 = v4;
                        let a2762 = 13u64;
                        if nat_eq(a2761, a2762) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a2763 = v4;
                            let a2764 = 9u64;
                            if nat_eq(a2763, a2764) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a2765 = v4;
                                let a2766 = 32u64;
                                if nat_lt(a2765, a2766) {
                                    let a2767 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a2768 = f449(v2)?;
                                    Ok(append_bytes(a2767, a2768))
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

pub fn f380(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m2769 = v0.clone();
    match m2769.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a2772 = {
                let (h2770, _) = v2;
                h2770
            };
            let a2773 = {
                let (h2771, _) = v1.clone();
                h2771
            };
            if equal(a2772, a2773) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f381(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f450(f451(Fn9::F452, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m2778 = {
        let (h2774, _) = f453(Fn10::F454, {
            let a2776 = {
                let a2775 = v0;
                length_list(a2775)
            };
            let a2777 = 1u64;
            nat_add(a2776, a2777)?
        }, v1)?;
        h2774
    };
    match m2778.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f382(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2783 = {
        let a2780 = f211({
            let (h2779, _) = v0;
            h2779
        })?;
        let a2781 = Bytes::lit(&[0x3a]);
        append_bytes(a2780, a2781)
    };
    let a2784 = {
        let (_, h2782) = v0;
        h2782
    };
    Ok(append_bytes(a2783, a2784))
}

pub fn f383(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2785 = v0.clone();
        length_bytes(a2785)
    };
    let a2789 = {
        let a2786 = v1;
        let a2787 = 2u64;
        let a2788 = 0u64;
        nat_rem(a2786, a2787, a2788)
    };
    let a2790 = 0u64;
    if nat_eq(a2789, a2790) {
        let (_, h2791) = {
            let (h2792, _) = f455(Fn11::F456(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2792
        };
        Ok(h2791)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f384(v0: u64) -> bool {
    let a2793 = v0;
    let a2794 = 8u64;
    if nat_eq(a2793, a2794) {
        true
    } else {
        let a2795 = v0;
        let a2796 = 9u64;
        if nat_eq(a2795, a2796) {
            true
        } else {
            let a2797 = v0;
            let a2798 = 10u64;
            if nat_eq(a2797, a2798) {
                true
            } else {
                let a2799 = v0;
                let a2800 = 12u64;
                if nat_eq(a2799, a2800) {
                    true
                } else {
                    let a2801 = v0;
                    let a2802 = 13u64;
                    nat_eq(a2801, a2802)
                }
            }
        }
    }
}

pub fn f385(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2805 = {
        let a2803 = v0;
        let a2804 = v1;
        index_bytes(a2803, a2804)
    };
    match m2805 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f457(v2)? {
                Ok(Some({
                    let a2806 = v1;
                    let a2807 = 1u64;
                    nat_add(a2806, a2807)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f386(v0: u8) -> R<u64> {
    let a2854 = {
        let v1: u8 = v0;
        let a2852 = {
            let a2828 = {
                let a2816 = {
                    let a2810 = {
                        let a2808 = v1;
                        let a2809 = 128u8;
                        fixed_u8::bit_and(a2808, a2809)
                    };
                    let a2811 = 128u8;
                    if equal(a2810, a2811) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2817 = {
                    let a2814 = {
                        let a2812 = v1;
                        let a2813 = 64u8;
                        fixed_u8::bit_and(a2812, a2813)
                    };
                    let a2815 = 64u8;
                    if equal(a2814, a2815) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2816, a2817)?
            };
            let a2829 = {
                let a2826 = {
                    let a2820 = {
                        let a2818 = v1;
                        let a2819 = 32u8;
                        fixed_u8::bit_and(a2818, a2819)
                    };
                    let a2821 = 32u8;
                    if equal(a2820, a2821) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2827 = {
                    let a2824 = {
                        let a2822 = v1;
                        let a2823 = 16u8;
                        fixed_u8::bit_and(a2822, a2823)
                    };
                    let a2825 = 16u8;
                    if equal(a2824, a2825) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2826, a2827)?
            };
            nat_add(a2828, a2829)?
        };
        let a2853 = {
            let a2850 = {
                let a2838 = {
                    let a2832 = {
                        let a2830 = v1;
                        let a2831 = 8u8;
                        fixed_u8::bit_and(a2830, a2831)
                    };
                    let a2833 = 8u8;
                    if equal(a2832, a2833) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2839 = {
                    let a2836 = {
                        let a2834 = v1;
                        let a2835 = 4u8;
                        fixed_u8::bit_and(a2834, a2835)
                    };
                    let a2837 = 4u8;
                    if equal(a2836, a2837) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2838, a2839)?
            };
            let a2851 = {
                let a2848 = {
                    let a2842 = {
                        let a2840 = v1;
                        let a2841 = 2u8;
                        fixed_u8::bit_and(a2840, a2841)
                    };
                    let a2843 = 2u8;
                    if equal(a2842, a2843) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2849 = {
                    let a2846 = {
                        let a2844 = v1;
                        let a2845 = 1u8;
                        fixed_u8::bit_and(a2844, a2845)
                    };
                    let a2847 = 1u8;
                    if equal(a2846, a2847) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2848, a2849)?
            };
            nat_add(a2850, a2851)?
        };
        nat_add(a2852, a2853)?
    };
    let a2855 = 48u64;
    Ok(nat_sub(a2854, a2855))
}

pub fn f387(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a2856 = v0.clone();
        length_bytes(a2856)
    };
    let a2857 = v1;
    let a2858 = v2;
    if if nat_lt(a2857, a2858) {
        let a2863 = f80(v0, {
            let a2861 = v2;
            let a2862 = {
                let a2859 = v1;
                let a2860 = 1u64;
                nat_add(a2859, a2860)?
            };
            nat_sub(a2861, a2862)
        })?;
        let a2864 = 48u64;
        nat_eq(a2863, a2864)
    } else {
        false
    } {
        Ok(Some({
            let a2865 = v1;
            let a2866 = 1u64;
            nat_add(a2865, a2866)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f388(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f458(v1, v0, v2)
}

pub fn f389(v0: u64) -> bool {
    if f390(v0) {
        true
    } else {
        let a2867 = 65u64;
        let a2868 = v0;
        if if nat_le(a2867, a2868) {
            let a2869 = v0;
            let a2870 = 70u64;
            nat_le(a2869, a2870)
        } else {
            false
        } {
            true
        } else {
            let a2871 = 97u64;
            let a2872 = v0;
            if nat_le(a2871, a2872) {
                let a2873 = v0;
                let a2874 = 102u64;
                nat_le(a2873, a2874)
            } else {
                false
            }
        }
    }
}

pub fn f390(v0: u64) -> bool {
    let a2875 = 48u64;
    let a2876 = v0;
    if nat_le(a2875, a2876) {
        let a2877 = v0;
        let a2878 = 57u64;
        nat_le(a2877, a2878)
    } else {
        false
    }
}

pub fn f391(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f112(v0.clone(), v1)?;
    let a2879 = v2;
    let a2880 = 69u64;
    if if nat_eq(a2879, a2880) {
        true
    } else {
        let a2881 = v2;
        let a2882 = 101u64;
        nat_eq(a2881, a2882)
    } {
        let v3: u64 = f112(v0.clone(), {
            let a2883 = v1;
            let a2884 = 1u64;
            nat_add(a2883, a2884)?
        })?;
        let v4: u64 = {
            let a2885 = v3;
            let a2886 = 43u64;
            if if nat_eq(a2885, a2886) {
                true
            } else {
                let a2887 = v3;
                let a2888 = 45u64;
                nat_eq(a2887, a2888)
            } {
                let a2889 = v1;
                let a2890 = 2u64;
                nat_add(a2889, a2890)?
            } else {
                let a2891 = v1;
                let a2892 = 1u64;
                nat_add(a2891, a2892)?
            }
        };
        let v5: u64 = f299(v0, 0u64, v4)?;
        let a2893 = v4;
        let a2894 = v5;
        if nat_lt(a2893, a2894) {
            Ok(v5)
        } else {
            Ok(v1)
        }
    } else {
        Ok(v1)
    }
}

pub fn f392(v0: Adt19, v1: Bytes) -> bool {
    let a2895 = f393(v0.clone());
    let a2896 = 3u64;
    if !nat_eq(a2895, a2896) {
        f46(f394(v0), v1)
    } else {
        false
    }
}

pub fn f393(v0: Adt19) -> u64 {
    let m2897 = v0;
    match m2897 {
        Adt19::C0(v1, _) => {
            v1
        }
    }
}

pub fn f394(v0: Adt19) -> Bytes {
    let m2898 = v0;
    match m2898 {
        Adt19::C0(_, v2) => {
            v2
        }
    }
}

pub fn f395(v0: Bytes) -> Bytes {
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

pub fn f396(v0: Bytes) -> bool {
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

pub fn f397(p2907: u64, p2908: List<Adt19>, p2909: List<Adt14>) -> R<Result<(List<Adt14>, List<Adt19>), Bytes>> {
    let m2906 = (p2907, (p2908, p2909));
    {
        let mut m2906 = m2906;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2906;
                let m2899 = v0;
                if m2899 == 0 {
                    Err::<(u64, (List<Adt19>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt19>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt19>), Bytes>(f8(Str::lit("a dictionary has more members than the input has tokens")))))
                } else {
                    let v3 = m2899 - 1;
                    if f302(v1.clone(), Str::lit("}")) {
                        let m2900 = f309(f304(v1), Str::lit(";"));
                        match m2900 {
                            Ok(v5) => {
                                Err::<(u64, (List<Adt19>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt19>), Bytes>>>(Ok(Ok::<(List<Adt14>, List<Adt19>), Bytes>((f459(v2, List::<Adt14>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<Adt19>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt19>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt19>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m2901 = f226(v1)?;
                        match m2901 {
                            Ok(v7) => {
                                let v8: bool = f302(v7.clone(), Str::lit("required"));
                                let m2902 = f314(v3, if v8 {
                                    f304(v7)
                                } else {
                                    v7
                                }, 0u64)?;
                                match m2902 {
                                    Ok(v10) => {
                                        let m2903 = f308(f315(v10.clone()));
                                        match m2903 {
                                            Ok(v12) => {
                                                if if f302(f310(v12.clone()), Str::lit("=")) {
                                                    !v8
                                                } else {
                                                    false
                                                } {
                                                    let m2904 = f399(f304(f310(v12.clone())))?;
                                                    match m2904 {
                                                        Ok(v14) => {
                                                            Ok::<(u64, (List<Adt19>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt19>), Bytes>>>((v3, (v14, List::cons(Adt14::C0(f312(v12), f406(f316(v10)), 0u64), v2))))
                                                        }
                                                        Err(v13) => {
                                                            Err::<(u64, (List<Adt19>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt19>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt19>), Bytes>(v13)))
                                                        }
                                                    }
                                                } else {
                                                    let m2905 = f309(f310(v12.clone()), Str::lit(";"));
                                                    match m2905 {
                                                        Ok(v16) => {
                                                            Ok::<(u64, (List<Adt19>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt19>), Bytes>>>((v3, (v16, List::cons(Adt14::C0(f312(v12), f406(f316(v10)), if v8 {
                                                                0u64
                                                            } else {
                                                                1u64
                                                            }), v2))))
                                                        }
                                                        Err(v15) => {
                                                            Err::<(u64, (List<Adt19>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt19>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt19>), Bytes>(v15)))
                                                        }
                                                    }
                                                }
                                            }
                                            Err(v11) => {
                                                Err::<(u64, (List<Adt19>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt19>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt19>), Bytes>(v11)))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Err::<(u64, (List<Adt19>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt19>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt19>), Bytes>(v9)))
                                    }
                                }
                            }
                            Err(v6) => {
                                Err::<(u64, (List<Adt19>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt19>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt19>), Bytes>(v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2906 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f398(p2918: u64, p2919: List<Adt19>, p2920: List<Adt14>) -> R<Result<(List<Adt14>, List<Adt19>), Bytes>> {
    let m2917 = (p2918, (p2919, p2920));
    {
        let mut m2917 = m2917;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2917;
                let m2910 = v0;
                if m2910 == 0 {
                    Err::<(u64, (List<Adt19>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt19>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt19>), Bytes>(f8(Str::lit("an interface has more members than the input has tokens")))))
                } else {
                    let v3 = m2910 - 1;
                    if f302(v1.clone(), Str::lit("}")) {
                        let m2911 = f309(f304(v1), Str::lit(";"));
                        match m2911 {
                            Ok(v5) => {
                                Err::<(u64, (List<Adt19>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt19>), Bytes>>>(Ok(Ok::<(List<Adt14>, List<Adt19>), Bytes>((f459(v2, List::<Adt14>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<Adt19>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt19>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt19>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m2912 = f226(v1)?;
                        match m2912 {
                            Ok(v7) => {
                                let v8: List<Adt19> = if if f302(v7.clone(), Str::lit("stringifier")) {
                                    true
                                } else {
                                    f302(v7.clone(), Str::lit("inherit"))
                                } {
                                    f304(v7.clone())
                                } else {
                                    v7.clone()
                                };
                                let v9: List<Adt19> = if f302(v8.clone(), Str::lit("readonly")) {
                                    f304(v8)
                                } else {
                                    v8
                                };
                                if f302(v9.clone(), Str::lit("attribute")) {
                                    let m2913 = f314(v3, f304(v9), 0u64)?;
                                    match m2913 {
                                        Ok(v11) => {
                                            let m2914 = f308(f315(v11.clone()));
                                            match m2914 {
                                                Ok(v13) => {
                                                    let m2915 = f309(f310(v13.clone()), Str::lit(";"));
                                                    match m2915 {
                                                        Ok(v15) => {
                                                            Ok::<(u64, (List<Adt19>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt19>), Bytes>>>((v3, (v15, List::cons(Adt14::C0(f312(v13), f406(f316(v11)), 0u64), v2))))
                                                        }
                                                        Err(v14) => {
                                                            Err::<(u64, (List<Adt19>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt19>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt19>), Bytes>(v14)))
                                                        }
                                                    }
                                                }
                                                Err(v12) => {
                                                    Err::<(u64, (List<Adt19>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt19>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt19>), Bytes>(v12)))
                                                }
                                            }
                                        }
                                        Err(v10) => {
                                            Err::<(u64, (List<Adt19>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt19>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt19>), Bytes>(v10)))
                                        }
                                    }
                                } else {
                                    let m2916 = f399(v7)?;
                                    match m2916 {
                                        Ok(v17) => {
                                            Ok::<(u64, (List<Adt19>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt19>), Bytes>>>((v3, (v17, v2)))
                                        }
                                        Err(v16) => {
                                            Err::<(u64, (List<Adt19>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt19>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt19>), Bytes>(v16)))
                                        }
                                    }
                                }
                            }
                            Err(v6) => {
                                Err::<(u64, (List<Adt19>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt19>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt19>), Bytes>(v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2917 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f399(v0: List<Adt19>) -> R<Result<List<Adt19>, Bytes>> {
    Ok(f303({
        let a2922 = {
            let a2921 = v0.clone();
            length_list(a2921)
        };
        let a2923 = 1u64;
        nat_add(a2922, a2923)?
    }, v0, List::<Bytes>::nil(), true))
}

pub fn f400(v0: List<Adt19>) -> Bytes {
    let m2924 = v0;
    match m2924.uncons() {
        None => {
            f8(Str::lit("the end of the input"))
        }
        Some((v1, _)) => {
            f7(List::cons(f8(Str::lit("'")), List::cons(f394(v1), List::cons(f8(Str::lit("'")), List::<Bytes>::nil()))))
        }
    }
}

pub fn f401(v0: Bytes) -> Bytes {
    if f180(v0.clone(), f8(Str::lit("_"))) {
        f43(v0.clone(), 1u64, {
            let a2926 = {
                let a2925 = v0;
                length_bytes(a2925)
            };
            let a2927 = 1u64;
            nat_sub(a2926, a2927)
        })
    } else {
        v0
    }
}

pub fn f402(v0: Adt15, v1: List<Adt19>) -> Result<Adt24, Bytes> {
    if f302(v1.clone(), Str::lit("?")) {
        Ok::<Adt24, Bytes>(Adt24::C0(f403(Str::lit("Option"), v0), f304(v1)))
    } else {
        Ok::<Adt24, Bytes>(Adt24::C0(v0, v1))
    }
}

pub fn f403(v0: Str, v1: Adt15) -> Adt15 {
    let v2: Adt15 = f406(v1);
    Adt15::C0(0u64, f7(List::cons(f8(Str::lit("(")), List::cons({
        let a2928 = v0;
        utf8_encode(a2928)
    }, List::cons(f8(Str::lit(" ")), List::cons(f245(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), f246(v2.clone()), f243(v2), Bytes::lit(&[]))
}

pub fn f404(v0: Bytes) -> Adt15 {
    Adt15::C0(0u64, Bytes::lit(&[]), List::<Bytes>::nil(), v0, Bytes::lit(&[]))
}

pub fn f405(v0: Bytes) -> Option<Adt15> {
    f460(v0, f461())
}

pub fn f406(v0: Adt15) -> Adt15 {
    let a2929 = f244(v0.clone());
    let a2930 = 0u64;
    if nat_eq(a2929, a2930) {
        v0
    } else {
        Adt15::C0(0u64, Bytes::lit(&[]), f246(v0.clone()), if f46(f243(v0.clone()), Bytes::lit(&[])) {
            f462()
        } else {
            f243(v0)
        }, Bytes::lit(&[]))
    }
}

pub fn f407(v0: Adt15) -> Bytes {
    let m2931 = v0;
    match m2931 {
        Adt15::C0(_, _, _, _, v5) => {
            v5
        }
    }
}

pub fn f408(v0: Adt14, v1: List<Adt14>) -> R<List<Adt14>> {
    let m2932 = v1;
    match m2932.uncons() {
        None => {
            Ok(List::cons(v0, List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            if f463({
                let a2934 = {
                    let a2933 = f330(v0.clone());
                    length_bytes(a2933)
                };
                let a2935 = 1u64;
                nat_add(a2934, a2935)?
            }, f330(v0.clone()), f330(v2.clone()), 0u64)? {
                Ok(List::cons(v0, List::cons(v2, v3)))
            } else {
                Ok(List::cons(v2, f408(v0, v3)?))
            }
        }
    }
}

pub fn f409(v0: List<Adt13>, v1: Bytes) -> List<Adt14> {
    let m2936 = f157(v0.clone(), 1u64, v1.clone());
    match m2936 {
        None => {
            f327(v0, 6u64, v1)
        }
        Some(v2) => {
            f325(f326(v2), f327(v0, 6u64, v1))
        }
    }
}

pub fn f410(v0: Bytes) -> R<Option<Bytes>> {
    let m2938 = f464({
        let a2937 = v0.clone();
        length_bytes(a2937)
    }, v0.clone(), 0u64, Bytes::lit(&[]))?;
    match m2938 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a2940 = {
                let a2939 = v1.clone();
                length_bytes(a2939)
            };
            let a2941 = 0u64;
            if nat_eq(a2940, a2941) {
                let a2943 = {
                    let a2942 = v0;
                    length_bytes(a2942)
                };
                let a2944 = 0u64;
                if nat_eq(a2943, a2944) {
                    Ok(Some(f8(Str::lit("u"))))
                } else {
                    Ok(None::<Bytes>)
                }
            } else {
                Ok(Some(f412(v1)?))
            }
        }
    }
}

pub fn f411(v0: Bytes) -> R<Option<Bytes>> {
    if f465({
        let a2945 = v0.clone();
        length_bytes(a2945)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f466(v0)?;
        if f467(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m2948 = {
                let a2946 = v1.clone();
                let a2947 = 0u64;
                index_bytes(a2946, a2947)
            };
            match m2948 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f352(f51(v2)?) {
                        Ok(Some({
                            let a2949 = f8(Str::lit("n"));
                            let a2950 = v1;
                            append_bytes(a2949, a2950)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f468({
            let a2951 = v0.clone();
            length_bytes(a2951)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f412(v0: Bytes) -> R<Bytes> {
    if f149(v0.clone(), f469()) {
        f470(v0)
    } else {
        Ok(v0)
    }
}

pub fn f413(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2954 = {
        let a2952 = v0;
        let a2953 = v1;
        index_bytes(a2952, a2953)
    };
    match m2954 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f51(v2)?;
            if f82({
                let a2955 = 128u64;
                let a2956 = v3;
                nat_le(a2955, a2956)
            }, {
                let a2957 = v3;
                let a2958 = 191u64;
                nat_le(a2957, a2958)
            }) {
                Ok(Some({
                    let a2959 = v3;
                    let a2960 = 128u64;
                    nat_sub(a2959, a2960)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f414(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f82({
        let a2961 = v1;
        let a2962 = v0;
        nat_le(a2961, a2962)
    }, f82({
        let a2963 = v0;
        let a2964 = v2;
        nat_le(a2963, a2964)
    }, f135({
        let a2965 = v0;
        let a2966 = 55296u64;
        nat_lt(a2965, a2966)
    }, {
        let a2967 = 57343u64;
        let a2968 = v0;
        nat_lt(a2967, a2968)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f415() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{3}/\u{7f}\u{3}@\u{0}\u{3}\u{7f}}\u{4}\u{0}\u{0}C\u{7f}\u{7f}"))
}

pub fn f416(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a2969 = v1;
        let a2970 = 3u64;
        nat_mul(a2969, a2970)?
    };
    let a2981 = {
        let a2977 = {
            let a2971 = f80(v0.clone(), v2)?;
            let a2972 = 16384u64;
            nat_mul(a2971, a2972)?
        };
        let a2978 = {
            let a2975 = f80(v0.clone(), {
                let a2973 = v2;
                let a2974 = 1u64;
                nat_add(a2973, a2974)?
            })?;
            let a2976 = 128u64;
            nat_mul(a2975, a2976)?
        };
        nat_add(a2977, a2978)?
    };
    let a2982 = f80(v0, {
        let a2979 = v2;
        let a2980 = 2u64;
        nat_add(a2979, a2980)?
    })?;
    nat_add(a2981, a2982)
}

pub fn f417(v0: u64) -> R<bool> {
    f251(v0, f471())
}

pub fn f418(v0: u64) -> R<bool> {
    f251(v0, f472())
}

pub fn f419(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f473(v1, v0, v2)
}

pub fn f420(v0: Bytes, v1: u64) -> Bytes {
    f43(v0, v1, 1u64)
}

pub fn f421() -> Bytes {
    f8(Str::lit("\u{0}\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\u{9}\u{a}\u{b}\u{c}\u{d}\u{e}\u{f}\u{10}\u{11}\u{12}\u{13}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{1b}\u{1c}\u{1d}\u{1e}\u{1f} !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~\u{7f}"))
}

pub fn f422() -> Bytes {
    f8(Str::lit("\u{80}\u{c0}\u{100}\u{140}\u{180}\u{1c0}\u{200}\u{240}\u{280}\u{2c0}\u{300}\u{340}\u{380}\u{3c0}\u{400}\u{440}\u{480}\u{4c0}\u{500}\u{540}\u{580}\u{5c0}\u{600}\u{640}\u{680}\u{6c0}\u{700}\u{740}\u{780}\u{7c0}"))
}

pub fn f423(v0: u64) -> R<Bytes> {
    Ok(f420(f474(), {
        let a2988 = {
            let a2986 = {
                let a2983 = v0;
                let a2984 = 64u64;
                let a2985 = 0u64;
                nat_rem(a2983, a2984, a2985)
            };
            let a2987 = 2u64;
            nat_mul(a2986, a2987)?
        };
        let a2989 = 1u64;
        nat_add(a2988, a2989)?
    }))
}

pub fn f424() -> Bytes {
    f8(Str::lit("\u{800}\u{1000}\u{2000}\u{3000}\u{4000}\u{5000}\u{6000}\u{7000}\u{8000}\u{9000}\u{a000}\u{b000}\u{c000}\u{d000}\u{e000}\u{f000}"))
}

pub fn f425(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<Option<u64>> {
    if f475(f112(v0, v3)?, v1, v2)? {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a2990 = v3;
            let a2991 = 1u64;
            nat_add(a2990, a2991)?
        }))
    }
}

pub fn f426(v0: Bytes) -> Result<Bytes, Bytes> {
    Err::<Bytes, Bytes>(v0)
}

pub fn f427() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f428(v0: Bytes) -> R<Option<Bytes>> {
    let m2992 = f167(v0.clone(), f8(Str::lit(" ")), 0u64)?;
    match m2992 {
        None => {
            let m2993 = f167(v0.clone(), f8(Str::lit(":")), 0u64)?;
            match m2993 {
                None => {
                    let v3: Bytes = f7(List::cons(f8(Str::lit(" ")), List::cons(v0, List::cons(f8(Str::lit(":")), List::<Bytes>::nil()))));
                    let m2994 = f167(f476(), v3.clone(), 0u64)?;
                    match m2994 {
                        None => {
                            Ok(None::<Bytes>)
                        }
                        Some(v4) => {
                            let v5: u64 = {
                                let a2996 = v4;
                                let a2997 = {
                                    let a2995 = v3;
                                    length_bytes(a2995)
                                };
                                nat_add(a2996, a2997)?
                            };
                            let m2998 = f167(f476(), f8(Str::lit(" ")), v5)?;
                            match m2998 {
                                None => {
                                    Ok(None::<Bytes>)
                                }
                                Some(v6) => {
                                    Ok(Some(f43(f476(), v5, {
                                        let a2999 = v6;
                                        let a3000 = v5;
                                        nat_sub(a2999, a3000)
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

pub fn f429(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a3001 = 0u64;
    let a3002 = v1;
    if if nat_lt(a3001, a3002) {
        f204({
            let a3005 = f127(v0, {
                let a3003 = v1;
                let a3004 = 1u64;
                nat_sub(a3003, a3004)
            })?;
            let a3006 = 10u64;
            nat_eq(a3005, a3006)
        })
    } else {
        false
    } {
        Ok(Some({
            let a3007 = v1;
            let a3008 = 1u64;
            nat_sub(a3007, a3008)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f430(v0: Bytes, v1: u64, v2: u64) -> R<Adt25> {
    let v3: (u64, u64) = {
        let (h3009, _) = f220(Fn5::F477(v2, v0.clone()), {
            let a3011 = {
                let a3010 = v0.clone();
                length_bytes(a3010)
            };
            let a3012 = 1u64;
            nat_add(a3011, a3012)?
        }, (v1, 0u64))?;
        h3009
    };
    let v4: u64 = f431(v0.clone(), {
        let (h3013, _) = v3;
        h3013
    }, v2)?;
    let a3014 = f127(v0, v4)?;
    let a3015 = 9u64;
    if if nat_eq(a3014, a3015) {
        let a3019 = {
            let a3017 = v4;
            let a3018 = {
                let (h3016, _) = v3;
                h3016
            };
            nat_sub(a3017, a3018)
        };
        let a3020 = v2;
        nat_lt(a3019, a3020)
    } else {
        false
    } {
        Ok(Adt25::C1(f108(Str::lit("a tab cannot indent a block scalar line (YAML 1.2.2 [63], [70])"), v4)?))
    } else {
        Ok(Adt25::C0({
            let (_, h3021) = v3;
            h3021
        }, {
            let (h3022, _) = v3;
            h3022
        }))
    }
}

pub fn f431(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h3023, _) = f164(Fn4::F478(v2, v0.clone(), v1), {
        let a3025 = {
            let a3024 = v0;
            length_bytes(a3024)
        };
        let a3026 = 1u64;
        nat_add(a3025, a3026)?
    }, v1)?;
    Ok(h3023)
}

pub fn f432(v0: Bytes, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = f431(v0.clone(), v1, v2)?;
    let a3029 = {
        let a3027 = v3;
        let a3028 = v1;
        nat_sub(a3027, a3028)
    };
    let a3030 = v2;
    if nat_eq(a3029, a3030) {
        Ok(f204({
            let a3031 = f127(v0, v3)?;
            let a3032 = 256u64;
            nat_eq(a3031, a3032)
        }))
    } else {
        Ok(false)
    }
}

pub fn f433(v0: Bytes, p3042: u64, v2: u64, v3: bool, p3043: u64, p3044: Bytes, p3045: u64) -> R<Adt26> {
    let m3041 = (p3042, (p3043, (p3044, p3045)));
    {
        let mut m3041 = m3041;
        loop {
            let step = {
                let (v1, (v4, (v5, v6))) = m3041;
                let m3033 = v4;
                if m3033 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<Adt26>>(Ok(Adt26::C1(f108(Str::lit("a block scalar has more lines than the input"), v1)?)))
                } else {
                    let v7 = m3033 - 1;
                    let v8: u64 = f431(v0.clone(), v1, v2)?;
                    let v9: bool = f204(f192(f127(v0.clone(), v8)?));
                    let v10: u64 = f281(v0.clone(), v8)?;
                    let v11: Bytes = f7(List::cons(v5, List::cons(f363(v6), List::cons(f279(v0.clone(), v8, v10), List::<Bytes>::nil()))));
                    let m3034 = f430(v0.clone(), f202(v0.clone(), v10)?, v2)?;
                    match m3034 {
                        Adt25::C0(v13, v14) => {
                            let v15: u64 = v14;
                            let v16: u64 = f431(v0.clone(), v15, v2)?;
                            if f432(v0.clone(), v15, v2)? {
                                Ok::<(u64, (u64, (Bytes, u64))), R<Adt26>>((v15, (v7, ({
                                    let a3037 = v11;
                                    let a3038 = if if if v3 {
                                        v9
                                    } else {
                                        false
                                    } {
                                        f204(f192(f127(v0.clone(), v16)?))
                                    } else {
                                        false
                                    } {
                                        let a3035 = v13;
                                        let a3036 = 0u64;
                                        if nat_eq(a3035, a3036) {
                                            Bytes::lit(&[0x20])
                                        } else {
                                            Bytes::lit(&[])
                                        }
                                    } else {
                                        Bytes::lit(&[0x0a])
                                    };
                                    append_bytes(a3037, a3038)
                                }, v13))))
                            } else {
                                Err::<(u64, (u64, (Bytes, u64))), R<Adt26>>(Ok(Adt26::C0(v11, v13, {
                                    let a3039 = f127(v0.clone(), v16)?;
                                    let a3040 = 256u64;
                                    if nat_eq(a3039, a3040) {
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
                Ok(next) => { m3041 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f434(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f195(v0.clone(), {
        let (h3046, _) = v1;
        h3046
    })?;
    let a3047 = f127(v0, v2)?;
    let a3048 = 10u64;
    if nat_eq(a3047, a3048) {
        Ok(Some(({
            let a3049 = v2;
            let a3050 = 1u64;
            nat_add(a3049, a3050)?
        }, f365({
            let (_, h3051) = v1;
            h3051
        }, {
            let a3053 = v2;
            let a3054 = {
                let (h3052, _) = v1;
                h3052
            };
            nat_sub(a3053, a3054)
        }))))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f435(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f479(v0, v1)
}

pub fn f436(v0: u64) -> R<Option<u64>> {
    let v1: u64 = f444(f8(Str::lit("abtnvfr")), v0)?;
    let a3055 = v1;
    let a3056 = 7u64;
    if nat_lt(a3055, a3056) {
        Ok(Some({
            let a3057 = v1;
            let a3058 = 7u64;
            nat_add(a3057, a3058)?
        }))
    } else {
        let a3059 = v0;
        let a3060 = 48u64;
        if nat_eq(a3059, a3060) {
            Ok(Some(0u64))
        } else {
            let a3061 = v0;
            let a3062 = 9u64;
            if nat_eq(a3061, a3062) {
                Ok(Some(9u64))
            } else {
                let a3063 = v0;
                let a3064 = 101u64;
                if nat_eq(a3063, a3064) {
                    Ok(Some(27u64))
                } else {
                    let a3065 = v0;
                    let a3066 = 78u64;
                    if nat_eq(a3065, a3066) {
                        Ok(Some(133u64))
                    } else {
                        let a3067 = v0;
                        let a3068 = 95u64;
                        if nat_eq(a3067, a3068) {
                            Ok(Some(160u64))
                        } else {
                            let a3069 = v0;
                            let a3070 = 76u64;
                            if nat_eq(a3069, a3070) {
                                Ok(Some(8232u64))
                            } else {
                                let a3071 = v0;
                                let a3072 = 80u64;
                                if nat_eq(a3071, a3072) {
                                    Ok(Some(8233u64))
                                } else {
                                    let a3073 = f444(f8(Str::lit(" \"/\\")), v0)?;
                                    let a3074 = 4u64;
                                    if nat_lt(a3073, a3074) {
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

pub fn f437(v0: u64) -> R<Bytes> {
    f108(Str::lit("an implicit key must be on one line (YAML 1.2.2 [154])"), v0)
}

pub fn f438(v0: Bytes, p3089: u64, v2: u64, p3090: u64, p3091: u64) -> R<Adt25> {
    let m3088 = (p3089, (p3090, p3091));
    {
        let mut m3088 = m3088;
        loop {
            let step = {
                let (v1, (v3, v4)) = m3088;
                let m3075 = v3;
                if m3075 == 0 {
                    Err::<(u64, (u64, u64)), R<Adt25>>(Ok(Adt25::C1(f108(Str::lit("the input ends inside a scalar"), v1)?)))
                } else {
                    let v5 = m3075 - 1;
                    let v6: u64 = {
                        let a3076 = v1;
                        let a3077 = 1u64;
                        nat_add(a3076, a3077)?
                    };
                    let v7: u64 = f195(v0.clone(), v6)?;
                    let v8: u64 = f196(v0.clone(), v7)?;
                    let v9: u64 = f127(v0.clone(), v8)?;
                    if f126(v0.clone(), v6)? {
                        Err::<(u64, (u64, u64)), R<Adt25>>(Ok(Adt25::C1(f108(Str::lit("a document marker cannot appear inside a scalar (YAML 1.2.2 [206])"), v6)?)))
                    } else {
                        let a3078 = v9;
                        let a3079 = 10u64;
                        if nat_eq(a3078, a3079) {
                            Ok::<(u64, (u64, u64)), R<Adt25>>((v8, (v5, {
                                let a3080 = v4;
                                let a3081 = 1u64;
                                nat_add(a3080, a3081)?
                            })))
                        } else {
                            let a3082 = v9;
                            let a3083 = 256u64;
                            if nat_eq(a3082, a3083) {
                                Err::<(u64, (u64, u64)), R<Adt25>>(Ok(Adt25::C1(f108(Str::lit("the input ends inside a quoted scalar (YAML 1.2.2 [109], [120])"), v8)?)))
                            } else {
                                let a3086 = {
                                    let a3084 = v7;
                                    let a3085 = v6;
                                    nat_sub(a3084, a3085)
                                };
                                let a3087 = v2;
                                if nat_lt(a3086, a3087) {
                                    Err::<(u64, (u64, u64)), R<Adt25>>(Ok(Adt25::C1(f108(Str::lit("a continuation line of a scalar is indented less than its node (YAML 1.2.2 [69])"), v7)?)))
                                } else {
                                    Err::<(u64, (u64, u64)), R<Adt25>>(Ok(Adt25::C0(v4, v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3088 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f439(v0: Bytes, p3101: u64, p3102: u64, p3103: u64) -> R<Option<u64>> {
    let m3100 = (p3101, (p3102, p3103));
    {
        let mut m3100 = m3100;
        loop {
            let step = {
                let (v1, (v2, v3)) = m3100;
                let m3092 = v2;
                if m3092 == 0 {
                    Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3092 - 1;
                    let m3093 = f480(f127(v0.clone(), v1)?);
                    match m3093 {
                        None => {
                            Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(None::<u64>))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, u64)), R<Option<u64>>>(({
                                let a3094 = v1;
                                let a3095 = 1u64;
                                nat_add(a3094, a3095)?
                            }, (v4, {
                                let a3098 = {
                                    let a3096 = v3;
                                    let a3097 = 16u64;
                                    nat_mul(a3096, a3097)?
                                };
                                let a3099 = v5;
                                nat_add(a3098, a3099)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3100 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f440(v0: Bytes, v1: u64) -> R<u64> {
    let (h3104, _) = f164(Fn4::F481(v0.clone()), {
        let a3106 = {
            let a3105 = v0;
            length_bytes(a3105)
        };
        let a3107 = 1u64;
        nat_add(a3106, a3107)?
    }, v1)?;
    Ok(h3104)
}

pub fn f441(v0: Bytes, v1: u64) -> R<u64> {
    let (h3108, _) = f164(Fn4::F482(v0.clone()), {
        let a3110 = {
            let a3109 = v0;
            length_bytes(a3109)
        };
        let a3111 = 1u64;
        nat_add(a3110, a3111)?
    }, v1)?;
    Ok(h3108)
}

pub fn f442(v0: Fn8, p3116: u64, p3117: (u64, bool)) -> R<((u64, bool), bool)> {
    let m3115 = (p3116, p3117);
    {
        let mut m3115 = m3115;
        loop {
            let step = {
                let (v1, v2) = m3115;
                let m3112 = v1;
                if m3112 == 0 {
                    Err::<(u64, (u64, bool)), R<((u64, bool), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3112 - 1;
                    let m3114 = {
                        let c3113: Fn8 = v0.clone();
                        c3113.apply(v2)?
                    };
                    match m3114 {
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
                Ok(next) => { m3115 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f443(v0: u64, v1: Bytes, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    f483(v1, v0, v2)
}

pub fn f444(v0: Bytes, v1: u64) -> R<u64> {
    let (h3118, _) = f164(Fn4::F484(v0.clone(), v1), {
        let a3120 = {
            let a3119 = v0;
            length_bytes(a3119)
        };
        let a3121 = 1u64;
        nat_add(a3120, a3121)?
    }, 0u64)?;
    Ok(h3118)
}

pub fn f445(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = {
        let (h3122, _) = v2;
        h3122
    };
    let v4: u64 = f127(v0.clone(), v3)?;
    if f193(v4) {
        Ok(None::<(u64, u64)>)
    } else {
        if f192(v4) {
            Ok(Some(({
                let a3123 = v3;
                let a3124 = 1u64;
                nat_add(a3123, a3124)?
            }, {
                let (_, h3125) = v2;
                h3125
            })))
        } else {
            let a3126 = v4;
            let a3127 = 58u64;
            if if nat_eq(a3126, a3127) {
                f204(f272(f127(v0.clone(), {
                    let a3128 = v3;
                    let a3129 = 1u64;
                    nat_add(a3128, a3129)?
                })?, v1))
            } else {
                false
            } {
                Ok(None::<(u64, u64)>)
            } else {
                let a3130 = v4;
                let a3131 = 35u64;
                if if nat_eq(a3130, a3131) {
                    f192(f127(v0, {
                        let a3132 = v3;
                        let a3133 = 1u64;
                        nat_sub(a3132, a3133)
                    })?)
                } else {
                    false
                } {
                    Ok(None::<(u64, u64)>)
                } else {
                    if if v1 {
                        f358(v4)
                    } else {
                        false
                    } {
                        Ok(None::<(u64, u64)>)
                    } else {
                        Ok(Some(({
                            let a3134 = v3;
                            let a3135 = 1u64;
                            nat_add(a3134, a3135)?
                        }, {
                            let a3136 = v3;
                            let a3137 = 1u64;
                            nat_add(a3136, a3137)?
                        })))
                    }
                }
            }
        }
    }
}

pub fn f446(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f485(v0, v1)
}

pub fn f447(v0: Bytes, v1: u64, v2: u64) -> R<(u64, u64)> {
    let (h3138, _) = f220(Fn5::F486(v1, v0.clone()), {
        let a3140 = {
            let a3139 = v0;
            length_bytes(a3139)
        };
        let a3141 = 1u64;
        nat_add(a3140, a3141)?
    }, (v2, 0u64))?;
    Ok(h3138)
}

pub fn f448(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f193(f127(v0, v1)?) {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3142 = v1;
            let a3143 = 1u64;
            nat_add(a3142, a3143)?
        }))
    }
}

pub fn f449(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3188 = {
            let a3164 = {
                let a3152 = {
                    let a3146 = {
                        let a3144 = v1;
                        let a3145 = 128u8;
                        fixed_u8::bit_and(a3144, a3145)
                    };
                    let a3147 = 128u8;
                    if equal(a3146, a3147) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3153 = {
                    let a3150 = {
                        let a3148 = v1;
                        let a3149 = 64u8;
                        fixed_u8::bit_and(a3148, a3149)
                    };
                    let a3151 = 64u8;
                    if equal(a3150, a3151) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3152, a3153)?
            };
            let a3165 = {
                let a3162 = {
                    let a3156 = {
                        let a3154 = v1;
                        let a3155 = 32u8;
                        fixed_u8::bit_and(a3154, a3155)
                    };
                    let a3157 = 32u8;
                    if equal(a3156, a3157) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3163 = {
                    let a3160 = {
                        let a3158 = v1;
                        let a3159 = 16u8;
                        fixed_u8::bit_and(a3158, a3159)
                    };
                    let a3161 = 16u8;
                    if equal(a3160, a3161) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3162, a3163)?
            };
            nat_add(a3164, a3165)?
        };
        let a3189 = {
            let a3186 = {
                let a3174 = {
                    let a3168 = {
                        let a3166 = v1;
                        let a3167 = 8u8;
                        fixed_u8::bit_and(a3166, a3167)
                    };
                    let a3169 = 8u8;
                    if equal(a3168, a3169) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3175 = {
                    let a3172 = {
                        let a3170 = v1;
                        let a3171 = 4u8;
                        fixed_u8::bit_and(a3170, a3171)
                    };
                    let a3173 = 4u8;
                    if equal(a3172, a3173) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3174, a3175)?
            };
            let a3187 = {
                let a3184 = {
                    let a3178 = {
                        let a3176 = v1;
                        let a3177 = 2u8;
                        fixed_u8::bit_and(a3176, a3177)
                    };
                    let a3179 = 2u8;
                    if equal(a3178, a3179) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3185 = {
                    let a3182 = {
                        let a3180 = v1;
                        let a3181 = 1u8;
                        fixed_u8::bit_and(a3180, a3181)
                    };
                    let a3183 = 1u8;
                    if equal(a3182, a3183) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3184, a3185)?
            };
            nat_add(a3186, a3187)?
        };
        nat_add(a3188, a3189)?
    };
    let a3196 = f160(f487(), {
        let a3190 = v2;
        let a3191 = 16u64;
        let a3192 = 0u64;
        nat_quot(a3190, a3191, a3192)
    });
    let a3197 = f160(f487(), {
        let a3193 = v2;
        let a3194 = 16u64;
        let a3195 = 0u64;
        nat_rem(a3193, a3194, a3195)
    });
    Ok(append_bytes(a3196, a3197))
}

pub fn f450(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f488(Fn12::F489, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f451(v0: Fn9, p3201: List<List<(Bytes, Bytes)>>, p3202: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m3200 = (p3201, p3202);
    {
        let mut m3200 = m3200;
        loop {
            let step = {
                let (v1, v2) = m3200;
                let m3198 = v2;
                match m3198.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c3199: Fn9 = v0.clone();
                            c3199.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3200 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f452(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f453(v0: Fn10, p3207: u64, p3208: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m3206 = (p3207, p3208);
    {
        let mut m3206 = m3206;
        loop {
            let step = {
                let (v1, v2) = m3206;
                let m3203 = v1;
                if m3203 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3203 - 1;
                    let m3205 = {
                        let c3204: Fn10 = v0.clone();
                        c3204.apply(v2.clone())?
                    };
                    match m3205 {
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
                Ok(next) => { m3206 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f454(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f490(v0)
}

pub fn f455(v0: Fn11, p3213: u64, p3214: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m3212 = (p3213, p3214);
    {
        let mut m3212 = m3212;
        loop {
            let step = {
                let (v1, v2) = m3212;
                let m3209 = v1;
                if m3209 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3209 - 1;
                    let m3211 = {
                        let c3210: Fn11 = v0.clone();
                        c3210.apply(v2.clone())?
                    };
                    match m3211 {
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
                Ok(next) => { m3212 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f456(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f491(v0, v1)
}

pub fn f457(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3259 = {
            let a3235 = {
                let a3223 = {
                    let a3217 = {
                        let a3215 = v1;
                        let a3216 = 128u8;
                        fixed_u8::bit_and(a3215, a3216)
                    };
                    let a3218 = 128u8;
                    if equal(a3217, a3218) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3224 = {
                    let a3221 = {
                        let a3219 = v1;
                        let a3220 = 64u8;
                        fixed_u8::bit_and(a3219, a3220)
                    };
                    let a3222 = 64u8;
                    if equal(a3221, a3222) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3223, a3224)?
            };
            let a3236 = {
                let a3233 = {
                    let a3227 = {
                        let a3225 = v1;
                        let a3226 = 32u8;
                        fixed_u8::bit_and(a3225, a3226)
                    };
                    let a3228 = 32u8;
                    if equal(a3227, a3228) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3234 = {
                    let a3231 = {
                        let a3229 = v1;
                        let a3230 = 16u8;
                        fixed_u8::bit_and(a3229, a3230)
                    };
                    let a3232 = 16u8;
                    if equal(a3231, a3232) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3233, a3234)?
            };
            nat_add(a3235, a3236)?
        };
        let a3260 = {
            let a3257 = {
                let a3245 = {
                    let a3239 = {
                        let a3237 = v1;
                        let a3238 = 8u8;
                        fixed_u8::bit_and(a3237, a3238)
                    };
                    let a3240 = 8u8;
                    if equal(a3239, a3240) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3246 = {
                    let a3243 = {
                        let a3241 = v1;
                        let a3242 = 4u8;
                        fixed_u8::bit_and(a3241, a3242)
                    };
                    let a3244 = 4u8;
                    if equal(a3243, a3244) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3245, a3246)?
            };
            let a3258 = {
                let a3255 = {
                    let a3249 = {
                        let a3247 = v1;
                        let a3248 = 2u8;
                        fixed_u8::bit_and(a3247, a3248)
                    };
                    let a3250 = 2u8;
                    if equal(a3249, a3250) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3256 = {
                    let a3253 = {
                        let a3251 = v1;
                        let a3252 = 1u8;
                        fixed_u8::bit_and(a3251, a3252)
                    };
                    let a3254 = 1u8;
                    if equal(a3253, a3254) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3255, a3256)?
            };
            nat_add(a3257, a3258)?
        };
        nat_add(a3259, a3260)?
    };
    let a3261 = 48u64;
    let a3262 = v2;
    if nat_le(a3261, a3262) {
        let a3263 = v2;
        let a3264 = 57u64;
        Ok(nat_le(a3263, a3264))
    } else {
        Ok(false)
    }
}

pub fn f458(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    if f492(v1, f112(v0, v2)?) {
        Ok(Some({
            let a3265 = v2;
            let a3266 = 1u64;
            nat_add(a3265, a3266)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f459(p3269: List<Adt14>, p3270: List<Adt14>) -> List<Adt14> {
    let m3268 = (p3269, p3270);
    {
        let mut m3268 = m3268;
        loop {
            let step = {
                let (v0, v1) = m3268;
                let m3267 = v0;
                match m3267.uncons() {
                    None => {
                        Err::<(List<Adt14>, List<Adt14>), List<Adt14>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt14>, List<Adt14>), List<Adt14>>((v3, List::cons(v2, v1)))
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

pub fn f460(v0: Bytes, p3275: List<(Bytes, Adt15)>) -> Option<Adt15> {
    let m3274 = p3275;
    {
        let mut m3274 = m3274;
        loop {
            let step = {
                let v1 = m3274;
                let m3271 = v1;
                match m3271.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt15)>, Option<Adt15>>(None::<Adt15>)
                    }
                    Some((v2, v3)) => {
                        if f46({
                            let (h3272, _) = v2;
                            h3272
                        }, v0.clone()) {
                            Err::<List<(Bytes, Adt15)>, Option<Adt15>>(Some({
                                let (_, h3273) = v2;
                                h3273
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt15)>, Option<Adt15>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3274 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f461() -> List<(Bytes, Adt15)> {
    List::cons((f8(Str::lit("boolean")), f313(f8(Str::lit("Bool")))), List::cons((f8(Str::lit("byte")), f313(f8(Str::lit("Int")))), List::cons((f8(Str::lit("octet")), f313(f8(Str::lit("Nat")))), List::cons((f8(Str::lit("bigint")), f313(f8(Str::lit("Int")))), List::cons((f8(Str::lit("float")), f313(f427())), List::cons((f8(Str::lit("double")), f313(f427())), List::cons((f8(Str::lit("DOMString")), f313(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("USVString")), f313(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ByteString")), f313(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ArrayBuffer")), f313(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("SharedArrayBuffer")), f313(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("DataView")), f313(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("Int8Array")), f313(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int16Array")), f313(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int32Array")), f313(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("BigInt64Array")), f313(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Uint8Array")), f313(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint16Array")), f313(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint32Array")), f313(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint8ClampedArray")), f313(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("BigUint64Array")), f313(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Float16Array")), f404(f8(Str::lit("Float16Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("Float32Array")), f404(f8(Str::lit("Float32Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("Float64Array")), f404(f8(Str::lit("Float64Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("any")), f404(f8(Str::lit("any is not translated, because it is the union of every Web IDL type and has no one LexLean type (Web IDL 2.13.1)")))), List::cons((f8(Str::lit("object")), f404(f8(Str::lit("object is not translated, because its values are references to JavaScript objects of any shape, which no LexLean type holds (Web IDL 2.13.20)")))), List::cons((f8(Str::lit("symbol")), f404(f8(Str::lit("symbol is not translated, because a symbol value has identity but no content that a LexLean type holds (Web IDL 2.13.21)")))), List::cons((f8(Str::lit("undefined")), f404(f8(Str::lit("undefined is not translated, because it is the type of the absence of a value (Web IDL 2.13.2)")))), List::<(Bytes, Adt15)>::nil()))))))))))))))))))))))))))))
}

pub fn f462() -> Bytes {
    f8(Str::lit("a union type is translated only as the whole body of a typedef, whose identifier names its inductive (Web IDL 2.11 and 2.13.32)"))
}

pub fn f463(p3286: u64, v1: Bytes, v2: Bytes, p3287: u64) -> R<bool> {
    let m3285 = (p3286, p3287);
    {
        let mut m3285 = m3285;
        loop {
            let step = {
                let (v0, v3) = m3285;
                let m3276 = v0;
                if m3276 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(false))
                } else {
                    let v4 = m3276 - 1;
                    let v5: u64 = f493(v1.clone(), v3)?;
                    let v6: u64 = f493(v2.clone(), v3)?;
                    let a3277 = v5;
                    let a3278 = v6;
                    if nat_lt(a3277, a3278) {
                        Err::<(u64, u64), R<bool>>(Ok(true))
                    } else {
                        let a3279 = v6;
                        let a3280 = v5;
                        if nat_lt(a3279, a3280) {
                            Err::<(u64, u64), R<bool>>(Ok(false))
                        } else {
                            let a3281 = v5;
                            let a3282 = 0u64;
                            if nat_eq(a3281, a3282) {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            } else {
                                Ok::<(u64, u64), R<bool>>((v4, {
                                    let a3283 = v3;
                                    let a3284 = 1u64;
                                    nat_add(a3283, a3284)?
                                }))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3285 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f464(p3300: u64, v1: Bytes, p3301: u64, p3302: Bytes) -> R<Option<Bytes>> {
    let m3299 = (p3300, (p3301, p3302));
    {
        let mut m3299 = m3299;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3299;
                let m3288 = v0;
                if m3288 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3288 - 1;
                    let a3290 = {
                        let a3289 = v1.clone();
                        length_bytes(a3289)
                    };
                    let a3291 = v2;
                    if nat_le(a3290, a3291) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3292 = f336(v1.clone(), v2)?;
                        match m3292 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3294 = v2;
                                    let a3295 = {
                                        let (_, h3293) = v5;
                                        h3293
                                    };
                                    nat_add(a3294, a3295)?
                                }, {
                                    let a3297 = v3;
                                    let a3298 = f494(v1.clone(), v2, {
                                        let (h3296, _) = v5;
                                        h3296
                                    });
                                    append_bytes(a3297, a3298)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3299 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f465(p3312: u64, v1: Bytes, p3313: u64) -> R<bool> {
    let m3311 = (p3312, p3313);
    {
        let mut m3311 = m3311;
        loop {
            let step = {
                let (v0, v2) = m3311;
                let m3303 = v0;
                if m3303 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m3303 - 1;
                    let m3306 = {
                        let a3304 = v1.clone();
                        let a3305 = v2;
                        index_bytes(a3304, a3305)
                    };
                    match m3306 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a3307 = f51(v4)?;
                            let a3308 = 128u64;
                            if nat_lt(a3307, a3308) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a3309 = v2;
                                    let a3310 = 1u64;
                                    nat_add(a3309, a3310)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3311 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f466(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f266(v0.clone(), 0u64, {
        let a3314 = v0;
        length_bytes(a3314)
    }, false, Bytes::lit(&[]))?;
    let m3317 = {
        let a3315 = v1.clone();
        let a3316 = 0u64;
        index_bytes(a3315, a3316)
    };
    match m3317 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f51(v2)?;
            if f351(v3) {
                let a3324 = f43({
                    let a3318 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a3318)
                }, {
                    let a3319 = v3;
                    let a3320 = 65u64;
                    nat_sub(a3319, a3320)
                }, 1u64);
                let a3325 = f43(v1.clone(), 1u64, {
                    let a3322 = {
                        let a3321 = v1;
                        length_bytes(a3321)
                    };
                    let a3323 = 1u64;
                    nat_sub(a3322, a3323)
                });
                Ok(append_bytes(a3324, a3325))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f467(v0: Bytes) -> R<bool> {
    let m3328 = {
        let a3326 = v0;
        let a3327 = 0u64;
        index_bytes(a3326, a3327)
    };
    match m3328 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f135(f350(f51(v1)?), f351(f51(v1)?)))
        }
    }
}

pub fn f468(p3341: u64, v1: Bytes, p3342: u64, p3343: Bytes) -> R<Option<Bytes>> {
    let m3340 = (p3341, (p3342, p3343));
    {
        let mut m3340 = m3340;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3340;
                let m3329 = v0;
                if m3329 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3329 - 1;
                    let a3331 = {
                        let a3330 = v1.clone();
                        length_bytes(a3330)
                    };
                    let a3332 = v2;
                    if nat_le(a3331, a3332) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3333 = f336(v1.clone(), v2)?;
                        match m3333 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3335 = v2;
                                    let a3336 = {
                                        let (_, h3334) = v5;
                                        h3334
                                    };
                                    nat_add(a3335, a3336)?
                                }, {
                                    let a3338 = v3;
                                    let a3339 = f495({
                                        let (h3337, _) = v5;
                                        h3337
                                    });
                                    append_bytes(a3338, a3339)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3340 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f469() -> List<Bytes> {
    List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("termination_by")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("admit")), List::cons(f8(Str::lit("below")), List::cons(f8(Str::lit("brecOn")), List::cons(f8(Str::lit("casesOn")), List::cons(f8(Str::lit("ctorElim")), List::cons(f8(Str::lit("ctorElimType")), List::cons(f8(Str::lit("ctorIdx")), List::cons(f8(Str::lit("noConfusion")), List::cons(f8(Str::lit("noConfusionType")), List::cons(f8(Str::lit("rec")), List::cons(f8(Str::lit("recOn")), List::cons(f8(Str::lit("toCtorIdx")), List::<Bytes>::nil())))))))))))))))))))
}

pub fn f470(v0: Bytes) -> R<Bytes> {
    let m3346 = {
        let a3344 = v0.clone();
        let a3345 = 0u64;
        index_bytes(a3344, a3345)
    };
    match m3346 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            let a3352 = {
                let a3347 = f8(Str::lit("u"));
                let a3348 = f496(8u64, f51(v1)?, 4u64, Bytes::lit(&[]));
                append_bytes(a3347, a3348)
            };
            let a3353 = f43(v0.clone(), 1u64, {
                let a3350 = {
                    let a3349 = v0;
                    length_bytes(a3349)
                };
                let a3351 = 1u64;
                nat_sub(a3350, a3351)
            });
            Ok(append_bytes(a3352, a3353))
        }
    }
}

pub fn f471() -> Bytes {
    f8(Str::lit("\u{0}\u{0}:\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{5}\u{7f}\u{0}\u{6}p\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f472() -> Bytes {
    f8(Str::lit("\u{0}\u{0}-\u{0}\u{0}.\u{0}\u{0}0\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}7\u{0}\u{1}7\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@?\u{0}@@\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f473(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f112(v0, {
        let (h3354, _) = v2;
        h3354
    })?;
    let a3356 = 1114111u64;
    let a3357 = {
        let (_, h3355) = v2;
        h3355
    };
    if nat_lt(a3356, a3357) {
        Ok(None::<(u64, u64)>)
    } else {
        let v4: u64 = f497(v3, v1);
        let a3358 = v4;
        let a3359 = 16u64;
        if nat_eq(a3358, a3359) {
            Ok(None::<(u64, u64)>)
        } else {
            Ok(Some(({
                let a3361 = {
                    let (h3360, _) = v2;
                    h3360
                };
                let a3362 = 1u64;
                nat_add(a3361, a3362)?
            }, {
                let a3366 = {
                    let a3364 = {
                        let (_, h3363) = v2;
                        h3363
                    };
                    let a3365 = if v1 {
                        16u64
                    } else {
                        10u64
                    };
                    nat_mul(a3364, a3365)?
                };
                let a3367 = v4;
                nat_add(a3366, a3367)?
            })))
        }
    }
}

pub fn f474() -> Bytes {
    f8(Str::lit("\u{80}\u{81}\u{82}\u{83}\u{84}\u{85}\u{86}\u{87}\u{88}\u{89}\u{8a}\u{8b}\u{8c}\u{8d}\u{8e}\u{8f}\u{90}\u{91}\u{92}\u{93}\u{94}\u{95}\u{96}\u{97}\u{98}\u{99}\u{9a}\u{9b}\u{9c}\u{9d}\u{9e}\u{9f}\u{a0}\u{a1}\u{a2}\u{a3}\u{a4}\u{a5}\u{a6}\u{a7}\u{a8}\u{a9}\u{aa}\u{ab}\u{ac}\u{ad}\u{ae}\u{af}\u{b0}\u{b1}\u{b2}\u{b3}\u{b4}\u{b5}\u{b6}\u{b7}\u{b8}\u{b9}\u{ba}\u{bb}\u{bc}\u{bd}\u{be}\u{bf}"))
}

pub fn f475(v0: u64, v1: bool, v2: u64) -> R<bool> {
    if v1 {
        let a3368 = v0;
        let a3369 = v2;
        if nat_eq(a3368, a3369) {
            Ok(true)
        } else {
            f251(v0, f498())
        }
    } else {
        f251(v0, f499())
    }
}

pub fn f476() -> Bytes {
    f8(Str::lit(" string:anyAtomicType boolean:anyAtomicType decimal:anyAtomicType float:anyAtomicType double:anyAtomicType duration:anyAtomicType dateTime:anyAtomicType time:anyAtomicType date:anyAtomicType gYearMonth:anyAtomicType gYear:anyAtomicType gMonthDay:anyAtomicType gDay:anyAtomicType gMonth:anyAtomicType hexBinary:anyAtomicType base64Binary:anyAtomicType anyURI:anyAtomicType QName:anyAtomicType NOTATION:anyAtomicType normalizedString:string token:normalizedString language:token NMTOKEN:token Name:token NCName:Name ID:NCName IDREF:NCName ENTITY:NCName integer:decimal nonPositiveInteger:integer long:integer nonNegativeInteger:integer negativeInteger:nonPositiveInteger int:long short:int byte:short unsignedLong:nonNegativeInteger positiveInteger:nonNegativeInteger unsignedInt:unsignedLong unsignedShort:unsignedInt unsignedByte:unsignedShort yearMonthDuration:duration dayTimeDuration:duration dateTimeStamp:dateTime NMTOKENS:anySimpleType IDREFS:anySimpleType ENTITIES:anySimpleType anyAtomicType:anySimpleType "))
}

pub fn f477(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f500(v1, v0, v2)
}

pub fn f478(v0: u64, v1: Bytes, v2: u64, v3: u64) -> R<Option<u64>> {
    f501(v1, v2, v0, v3)
}

pub fn f479(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let a3372 = {
        let (h3370, _) = v1;
        h3370
    };
    let a3373 = {
        let a3371 = v0.clone();
        length_bytes(a3371)
    };
    if nat_lt(a3372, a3373) {
        Ok(Some((f202(v0, {
            let (h3374, _) = v1;
            h3374
        })?, {
            let a3376 = {
                let (_, h3375) = v1;
                h3375
            };
            let a3377 = 1u64;
            nat_add(a3376, a3377)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f480(v0: u64) -> Option<u64> {
    let a3378 = 48u64;
    let a3379 = v0;
    if if nat_le(a3378, a3379) {
        let a3380 = v0;
        let a3381 = 57u64;
        nat_le(a3380, a3381)
    } else {
        false
    } {
        Some({
            let a3382 = v0;
            let a3383 = 48u64;
            nat_sub(a3382, a3383)
        })
    } else {
        let a3384 = 97u64;
        let a3385 = v0;
        if if nat_le(a3384, a3385) {
            let a3386 = v0;
            let a3387 = 102u64;
            nat_le(a3386, a3387)
        } else {
            false
        } {
            Some({
                let a3388 = v0;
                let a3389 = 87u64;
                nat_sub(a3388, a3389)
            })
        } else {
            let a3390 = 65u64;
            let a3391 = v0;
            if if nat_le(a3390, a3391) {
                let a3392 = v0;
                let a3393 = 70u64;
                nat_le(a3392, a3393)
            } else {
                false
            } {
                Some({
                    let a3394 = v0;
                    let a3395 = 55u64;
                    nat_sub(a3394, a3395)
                })
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f481(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f502(v0, v1)
}

pub fn f482(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f503(v0, v1)
}

pub fn f483(v0: Bytes, v1: u64, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    let v3: u64 = {
        let (h3396, _) = v2;
        h3396
    };
    let v4: u64 = f127(v0.clone(), v3)?;
    let (_, h3397) = v2;
    if h3397 {
        Ok(None::<(u64, bool)>)
    } else {
        if f192(v4) {
            Ok(Some(({
                let a3398 = v3;
                let a3399 = 1u64;
                nat_add(a3398, a3399)?
            }, false)))
        } else {
            let a3400 = v4;
            let a3401 = 35u64;
            if if nat_eq(a3400, a3401) {
                let a3402 = v3;
                let a3403 = 0u64;
                if if nat_eq(a3402, a3403) {
                    true
                } else {
                    f192(f127(v0.clone(), {
                        let a3404 = v3;
                        let a3405 = 1u64;
                        nat_sub(a3404, a3405)
                    })?)
                } {
                    true
                } else {
                    let a3408 = f127(v0.clone(), {
                        let a3406 = v3;
                        let a3407 = 1u64;
                        nat_sub(a3406, a3407)
                    })?;
                    let a3409 = 10u64;
                    nat_eq(a3408, a3409)
                }
            } else {
                false
            } {
                Ok(Some((f281(v0, v3)?, false)))
            } else {
                let a3410 = v4;
                let a3411 = 10u64;
                if nat_eq(a3410, a3411) {
                    let v5: u64 = {
                        let a3412 = v3;
                        let a3413 = 1u64;
                        nat_add(a3412, a3413)?
                    };
                    let v6: u64 = f195(v0.clone(), v5)?;
                    let v7: u64 = f196(v0.clone(), v6)?;
                    if if f126(v0.clone(), v5)? {
                        true
                    } else {
                        let a3416 = {
                            let a3414 = v6;
                            let a3415 = v5;
                            nat_sub(a3414, a3415)
                        };
                        let a3417 = v1;
                        if if nat_lt(a3416, a3417) {
                            f204(f193(f127(v0.clone(), v7)?))
                        } else {
                            false
                        } {
                            f204({
                                let a3418 = f127(v0, v7)?;
                                let a3419 = 35u64;
                                nat_eq(a3418, a3419)
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

pub fn f484(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f504(v0, v1, v2)
}

pub fn f485(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f196(v0.clone(), {
        let (h3420, _) = v1;
        h3420
    })?;
    let a3421 = f127(v0, v2)?;
    let a3422 = 10u64;
    if nat_eq(a3421, a3422) {
        Ok(Some(({
            let a3423 = v2;
            let a3424 = 1u64;
            nat_add(a3423, a3424)?
        }, {
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

pub fn f486(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f505(v1, v0, v2)
}

pub fn f487() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f488(v0: Fn12, p3431: List<List<(Bytes, Bytes)>>, p3432: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m3430 = (p3431, p3432);
    {
        let mut m3430 = m3430;
        loop {
            let step = {
                let (v1, v2) = m3430;
                let m3428 = v2;
                match m3428.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c3429: Fn12 = v0.clone();
                            c3429.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3430 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f489(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f490(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a3434 = {
        let a3433 = v0.clone();
        length_list(a3433)
    };
    let a3435 = 1u64;
    if nat_le(a3434, a3435) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f506(v0)?))
    }
}

pub fn f491(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h3436, _) = v1;
        h3436
    };
    let m3438 = {
        let (_, h3437) = v1;
        h3437
    };
    match m3438 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m3441 = {
                let a3439 = v0.clone();
                let a3440 = v2;
                index_bytes(a3439, a3440)
            };
            match m3441 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m3446 = {
                        let a3444 = v0;
                        let a3445 = {
                            let a3442 = v2;
                            let a3443 = 1u64;
                            nat_add(a3442, a3443)?
                        };
                        index_bytes(a3444, a3445)
                    };
                    match m3446 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f507(v4)? {
                                f507(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a3447 = v2;
                                    let a3448 = 2u64;
                                    nat_add(a3447, a3448)?
                                }, Some({
                                    let a3453 = v3;
                                    let a3454 = f160(f508(), {
                                        let a3451 = {
                                            let a3449 = f509(v4)?;
                                            let a3450 = 16u64;
                                            nat_mul(a3449, a3450)?
                                        };
                                        let a3452 = f509(v5)?;
                                        nat_add(a3451, a3452)?
                                    });
                                    append_bytes(a3453, a3454)
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

pub fn f492(v0: u64, v1: u64) -> bool {
    let a3455 = v0;
    let a3456 = 0u64;
    if nat_eq(a3455, a3456) {
        f390(v1)
    } else {
        let a3457 = v0;
        let a3458 = 1u64;
        if nat_eq(a3457, a3458) {
            f389(v1)
        } else {
            let a3459 = v0;
            let a3460 = 2u64;
            if nat_eq(a3459, a3460) {
                f510(v1)
            } else {
                f511(v1)
            }
        }
    }
}

pub fn f493(v0: Bytes, v1: u64) -> R<u64> {
    let a3462 = v1;
    let a3463 = {
        let a3461 = v0.clone();
        length_bytes(a3461)
    };
    if nat_lt(a3462, a3463) {
        let a3464 = f112(v0, v1)?;
        let a3465 = 1u64;
        nat_add(a3464, a3465)
    } else {
        Ok(0u64)
    }
}

pub fn f494(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f135(f82(f135(f350(v2), f351(v2)), f82({
        let a3466 = v2;
        let a3467 = 117u64;
        !nat_eq(a3466, a3467)
    }, {
        let a3468 = v2;
        let a3469 = 85u64;
        !nat_eq(a3468, a3469)
    })), f82(f352(v2), {
        let a3470 = 0u64;
        let a3471 = v1;
        nat_lt(a3470, a3471)
    })) {
        f43(v0, v1, 1u64)
    } else {
        let a3472 = v2;
        let a3473 = 65536u64;
        if nat_lt(a3472, a3473) {
            let a3474 = f8(Str::lit("u"));
            let a3475 = f496(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a3474, a3475)
        } else {
            let a3476 = f8(Str::lit("U"));
            let a3477 = f496(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a3476, a3477)
        }
    }
}

pub fn f495(v0: u64) -> Bytes {
    let a3478 = f8(Str::lit("u"));
    let a3479 = f496(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a3478, a3479)
}

pub fn f496(p3496: u64, p3497: u64, p3498: u64, p3499: Bytes) -> Bytes {
    let m3495 = (p3496, (p3497, (p3498, p3499)));
    {
        let mut m3495 = m3495;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m3495;
                let m3480 = v0;
                if m3480 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m3480 - 1;
                    if f82({
                        let a3481 = v1;
                        let a3482 = 0u64;
                        nat_eq(a3481, a3482)
                    }, {
                        let a3483 = v2;
                        let a3484 = 0u64;
                        nat_eq(a3483, a3484)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a3485 = v1;
                            let a3486 = 16u64;
                            let a3487 = 0u64;
                            nat_quot(a3485, a3486, a3487)
                        }, ({
                            let a3488 = v2;
                            let a3489 = 1u64;
                            nat_sub(a3488, a3489)
                        }, {
                            let a3493 = f512({
                                let a3490 = v1;
                                let a3491 = 16u64;
                                let a3492 = 0u64;
                                nat_rem(a3490, a3491, a3492)
                            });
                            let a3494 = v3;
                            append_bytes(a3493, a3494)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m3495 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f497(v0: u64, v1: bool) -> u64 {
    let a3500 = v0;
    let a3501 = 48u64;
    if nat_lt(a3500, a3501) {
        16u64
    } else {
        let a3502 = v0;
        let a3503 = 57u64;
        if nat_le(a3502, a3503) {
            let a3504 = v0;
            let a3505 = 48u64;
            nat_sub(a3504, a3505)
        } else {
            if v1 {
                let a3506 = v0;
                let a3507 = 65u64;
                if nat_lt(a3506, a3507) {
                    16u64
                } else {
                    let a3508 = v0;
                    let a3509 = 70u64;
                    if nat_le(a3508, a3509) {
                        let a3510 = v0;
                        let a3511 = 55u64;
                        nat_sub(a3510, a3511)
                    } else {
                        let a3512 = v0;
                        let a3513 = 97u64;
                        if nat_lt(a3512, a3513) {
                            16u64
                        } else {
                            let a3514 = v0;
                            let a3515 = 102u64;
                            if nat_le(a3514, a3515) {
                                let a3516 = v0;
                                let a3517 = 87u64;
                                nat_sub(a3516, a3517)
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

pub fn f498() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f499() -> Bytes {
    f8(Str::lit("\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{0}]\u{0}\u{0}]\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f500(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f431(v0.clone(), {
        let (h3518, _) = v2;
        h3518
    }, v1)?;
    let a3519 = f127(v0, v3)?;
    let a3520 = 10u64;
    if nat_eq(a3519, a3520) {
        Ok(Some(({
            let a3521 = v3;
            let a3522 = 1u64;
            nat_add(a3521, a3522)?
        }, {
            let a3524 = {
                let (_, h3523) = v2;
                h3523
            };
            let a3525 = 1u64;
            nat_add(a3524, a3525)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f501(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a3526 = f127(v0, v3)?;
    let a3527 = 32u64;
    if if nat_eq(a3526, a3527) {
        let a3530 = {
            let a3528 = v3;
            let a3529 = v1;
            nat_sub(a3528, a3529)
        };
        let a3531 = v2;
        nat_lt(a3530, a3531)
    } else {
        false
    } {
        Ok(Some({
            let a3532 = v3;
            let a3533 = 1u64;
            nat_add(a3532, a3533)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f502(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f127(v0, v1)?;
    let a3534 = v2;
    let a3535 = 34u64;
    if if if if nat_eq(a3534, a3535) {
        true
    } else {
        let a3536 = v2;
        let a3537 = 92u64;
        nat_eq(a3536, a3537)
    } {
        true
    } else {
        f192(v2)
    } {
        true
    } else {
        f193(v2)
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3538 = v1;
            let a3539 = 1u64;
            nat_add(a3538, a3539)?
        }))
    }
}

pub fn f503(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f127(v0, v1)?;
    let a3540 = v2;
    let a3541 = 39u64;
    if if if nat_eq(a3540, a3541) {
        true
    } else {
        f192(v2)
    } {
        true
    } else {
        f193(v2)
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3542 = v1;
            let a3543 = 1u64;
            nat_add(a3542, a3543)?
        }))
    }
}

pub fn f504(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a3545 = v2;
    let a3546 = {
        let a3544 = v0.clone();
        length_bytes(a3544)
    };
    if if nat_lt(a3545, a3546) {
        f204({
            let a3547 = f127(v0, v2)?;
            let a3548 = v1;
            nat_eq(a3547, a3548)
        })
    } else {
        false
    } {
        Ok(Some({
            let a3549 = v2;
            let a3550 = 1u64;
            nat_add(a3549, a3550)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f505(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m3552 = f480(f127(v0, {
        let (h3551, _) = v2;
        h3551
    })?);
    match m3552 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v3) => {
            let a3553 = v3;
            let a3554 = v1;
            if nat_lt(a3553, a3554) {
                Ok(Some(({
                    let a3556 = {
                        let (h3555, _) = v2;
                        h3555
                    };
                    let a3557 = 1u64;
                    nat_add(a3556, a3557)?
                }, {
                    let a3561 = {
                        let a3559 = {
                            let (_, h3558) = v2;
                            h3558
                        };
                        let a3560 = v1;
                        nat_mul(a3559, a3560)?
                    };
                    let a3562 = v3;
                    nat_add(a3561, a3562)?
                })))
            } else {
                Ok(None::<(u64, u64)>)
            }
        }
    }
}

pub fn f506(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f450({
        let (_, h3563) = {
            let (h3564, _) = f513(Fn13::F514, {
                let a3566 = {
                    let a3565 = v0.clone();
                    length_list(a3565)
                };
                let a3567 = 1u64;
                nat_add(a3566, a3567)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h3564
        };
        h3563
    }))
}

pub fn f507(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3612 = {
            let a3588 = {
                let a3576 = {
                    let a3570 = {
                        let a3568 = v1;
                        let a3569 = 128u8;
                        fixed_u8::bit_and(a3568, a3569)
                    };
                    let a3571 = 128u8;
                    if equal(a3570, a3571) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3577 = {
                    let a3574 = {
                        let a3572 = v1;
                        let a3573 = 64u8;
                        fixed_u8::bit_and(a3572, a3573)
                    };
                    let a3575 = 64u8;
                    if equal(a3574, a3575) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3576, a3577)?
            };
            let a3589 = {
                let a3586 = {
                    let a3580 = {
                        let a3578 = v1;
                        let a3579 = 32u8;
                        fixed_u8::bit_and(a3578, a3579)
                    };
                    let a3581 = 32u8;
                    if equal(a3580, a3581) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3587 = {
                    let a3584 = {
                        let a3582 = v1;
                        let a3583 = 16u8;
                        fixed_u8::bit_and(a3582, a3583)
                    };
                    let a3585 = 16u8;
                    if equal(a3584, a3585) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3586, a3587)?
            };
            nat_add(a3588, a3589)?
        };
        let a3613 = {
            let a3610 = {
                let a3598 = {
                    let a3592 = {
                        let a3590 = v1;
                        let a3591 = 8u8;
                        fixed_u8::bit_and(a3590, a3591)
                    };
                    let a3593 = 8u8;
                    if equal(a3592, a3593) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3599 = {
                    let a3596 = {
                        let a3594 = v1;
                        let a3595 = 4u8;
                        fixed_u8::bit_and(a3594, a3595)
                    };
                    let a3597 = 4u8;
                    if equal(a3596, a3597) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3598, a3599)?
            };
            let a3611 = {
                let a3608 = {
                    let a3602 = {
                        let a3600 = v1;
                        let a3601 = 2u8;
                        fixed_u8::bit_and(a3600, a3601)
                    };
                    let a3603 = 2u8;
                    if equal(a3602, a3603) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3609 = {
                    let a3606 = {
                        let a3604 = v1;
                        let a3605 = 1u8;
                        fixed_u8::bit_and(a3604, a3605)
                    };
                    let a3607 = 1u8;
                    if equal(a3606, a3607) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3608, a3609)?
            };
            nat_add(a3610, a3611)?
        };
        nat_add(a3612, a3613)?
    };
    let a3614 = 48u64;
    let a3615 = v2;
    if if nat_le(a3614, a3615) {
        let a3616 = v2;
        let a3617 = 57u64;
        nat_le(a3616, a3617)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a3618 = 97u64;
        let a3619 = v2;
        if nat_le(a3618, a3619) {
            let a3620 = v2;
            let a3621 = 102u64;
            Ok(nat_le(a3620, a3621))
        } else {
            Ok(false)
        }
    }
}

pub fn f508() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f509(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3666 = {
            let a3642 = {
                let a3630 = {
                    let a3624 = {
                        let a3622 = v1;
                        let a3623 = 128u8;
                        fixed_u8::bit_and(a3622, a3623)
                    };
                    let a3625 = 128u8;
                    if equal(a3624, a3625) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3631 = {
                    let a3628 = {
                        let a3626 = v1;
                        let a3627 = 64u8;
                        fixed_u8::bit_and(a3626, a3627)
                    };
                    let a3629 = 64u8;
                    if equal(a3628, a3629) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3630, a3631)?
            };
            let a3643 = {
                let a3640 = {
                    let a3634 = {
                        let a3632 = v1;
                        let a3633 = 32u8;
                        fixed_u8::bit_and(a3632, a3633)
                    };
                    let a3635 = 32u8;
                    if equal(a3634, a3635) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3641 = {
                    let a3638 = {
                        let a3636 = v1;
                        let a3637 = 16u8;
                        fixed_u8::bit_and(a3636, a3637)
                    };
                    let a3639 = 16u8;
                    if equal(a3638, a3639) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3640, a3641)?
            };
            nat_add(a3642, a3643)?
        };
        let a3667 = {
            let a3664 = {
                let a3652 = {
                    let a3646 = {
                        let a3644 = v1;
                        let a3645 = 8u8;
                        fixed_u8::bit_and(a3644, a3645)
                    };
                    let a3647 = 8u8;
                    if equal(a3646, a3647) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3653 = {
                    let a3650 = {
                        let a3648 = v1;
                        let a3649 = 4u8;
                        fixed_u8::bit_and(a3648, a3649)
                    };
                    let a3651 = 4u8;
                    if equal(a3650, a3651) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3652, a3653)?
            };
            let a3665 = {
                let a3662 = {
                    let a3656 = {
                        let a3654 = v1;
                        let a3655 = 2u8;
                        fixed_u8::bit_and(a3654, a3655)
                    };
                    let a3657 = 2u8;
                    if equal(a3656, a3657) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3663 = {
                    let a3660 = {
                        let a3658 = v1;
                        let a3659 = 1u8;
                        fixed_u8::bit_and(a3658, a3659)
                    };
                    let a3661 = 1u8;
                    if equal(a3660, a3661) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3662, a3663)?
            };
            nat_add(a3664, a3665)?
        };
        nat_add(a3666, a3667)?
    };
    let a3668 = v2;
    let a3669 = 57u64;
    if nat_le(a3668, a3669) {
        let a3670 = v2;
        let a3671 = 48u64;
        Ok(nat_sub(a3670, a3671))
    } else {
        let a3672 = v2;
        let a3673 = 87u64;
        Ok(nat_sub(a3672, a3673))
    }
}

pub fn f510(v0: u64) -> bool {
    let a3674 = 48u64;
    let a3675 = v0;
    if nat_le(a3674, a3675) {
        let a3676 = v0;
        let a3677 = 55u64;
        nat_le(a3676, a3677)
    } else {
        false
    }
}

pub fn f511(v0: u64) -> bool {
    if f298(v0) {
        true
    } else {
        if f390(v0) {
            true
        } else {
            let a3678 = v0;
            let a3679 = 95u64;
            if nat_eq(a3678, a3679) {
                true
            } else {
                let a3680 = v0;
                let a3681 = 45u64;
                nat_eq(a3680, a3681)
            }
        }
    }
}

pub fn f512(v0: u64) -> Bytes {
    f43({
        let a3682 = Str::lit("0123456789ABCDEF");
        utf8_encode(a3682)
    }, v0, 1u64)
}

pub fn f513(v0: Fn13, p3687: u64, p3688: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m3686 = (p3687, p3688);
    {
        let mut m3686 = m3686;
        loop {
            let step = {
                let (v1, v2) = m3686;
                let m3683 = v1;
                if m3683 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3683 - 1;
                    let m3685 = {
                        let c3684: Fn13 = v0.clone();
                        c3684.apply(v2.clone())?
                    };
                    match m3685 {
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
                Ok(next) => { m3686 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f514(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f515(v0)
}

pub fn f515(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h3689, _) = v0;
        h3689
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h3690) = v0;
        h3690
    };
    let m3691 = v1;
    match m3691.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m3692 = v4;
            match m3692.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f516(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f516(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3697 = {
            let a3695 = {
                let a3693 = v0.clone();
                length_list(a3693)
            };
            let a3696 = {
                let a3694 = v1.clone();
                length_list(a3694)
            };
            nat_add(a3695, a3696)?
        };
        let a3698 = 1u64;
        nat_add(a3697, a3698)?
    };
    Ok(f517({
        let (_, h3699) = {
            let (_, h3700) = {
                let (h3701, _) = f518(Fn14::F519, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3701
            };
            h3700
        };
        h3699
    }))
}

pub fn f517(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f284(Fn6::F520, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f518(v0: Fn14, p3706: u64, p3707: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3705 = (p3706, p3707);
    {
        let mut m3705 = m3705;
        loop {
            let step = {
                let (v1, v2) = m3705;
                let m3702 = v1;
                if m3702 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3702 - 1;
                    let m3704 = {
                        let c3703: Fn14 = v0.clone();
                        c3703.apply(v2.clone())
                    };
                    match m3704 {
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
                Ok(next) => { m3705 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f519(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f521(v0)
}

pub fn f520(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f521(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h3708, _) = v0.clone();
        h3708
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h3709, _) = {
            let (_, h3710) = v0.clone();
            h3710
        };
        h3709
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h3711) = {
            let (_, h3712) = v0;
            h3712
        };
        h3711
    };
    let m3713 = v1.clone();
    match m3713.uncons() {
        None => {
            let m3714 = v2;
            match m3714.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3715 = v2.clone();
            match m3715.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f522({
                        let (h3716, _) = v8.clone();
                        h3716
                    }, {
                        let (h3717, _) = v6.clone();
                        h3717
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

pub fn f522(v0: Bytes, v1: Bytes) -> bool {
    let a3722 = {
        let a3718 = v0;
        let a3719 = v1;
        compare_bytes(a3718, a3719)
    };
    let a3723 = {
        let a3720 = Bytes::lit(&[]);
        let a3721 = Bytes::lit(&[0x01]);
        compare_bytes(a3720, a3721)
    };
    equal(a3722, a3723)
}

pub fn run(p0: Option<Adt0>, p1: Adt3) -> R<(Adt0, Adt10)> {
    f0(p0, p1)
}
