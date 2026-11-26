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
    C0(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt19 {
    C0(Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt20 {
    C0(Bytes, Adt16, Adt16, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Adt21 {
    C0(Bytes, List<Adt18>),
}

#[derive(Clone)]
pub enum Adt22 {
    C0(Adt14, List<Adt18>),
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
    F336(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn3::F126(k0, k1) => Ok(f126(*k0, k1.clone(), p0)),
            Fn3::F127(k0) => f127(k0.clone(), p0),
            Fn3::F336(k0) => f336(k0.clone(), p0),
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
    F335(Bytes),
    F340(u64, Bytes),
    F428(u64, Bytes, u64),
    F431(Bytes),
    F432(Bytes),
    F436(Bytes),
    F437(Bytes),
    F461(Bytes),
    F467(Bytes),
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
            Fn4::F335(k0) => f335(k0.clone(), p0),
            Fn4::F340(k0, k1) => f340(*k0, k1.clone(), p0),
            Fn4::F428(k0, k1, k2) => f428(*k0, k1.clone(), *k2, p0),
            Fn4::F431(k0) => f431(k0.clone(), p0),
            Fn4::F432(k0) => f432(k0.clone(), p0),
            Fn4::F436(k0) => f436(k0.clone(), p0),
            Fn4::F437(k0) => f437(k0.clone(), p0),
            Fn4::F461(k0) => f461(k0.clone(), p0),
            Fn4::F467(k0) => f467(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F244,
    F479,
}

impl Fn5 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn5::F244 => f244(p0, p1),
            Fn5::F479 => f479(p0, p1),
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
    F373(bool, Bytes),
    F389(Bytes),
    F399(Bytes),
    F427(u64, Bytes),
    F435(u64, Bytes),
    F439(Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn7::F317(k0) => f317(k0.clone(), p0),
            Fn7::F326(k0, k1) => f326(*k0, k1.clone(), p0),
            Fn7::F373(k0, k1) => f373(*k0, k1.clone(), p0),
            Fn7::F389(k0) => f389(k0.clone(), p0),
            Fn7::F399(k0) => f399(k0.clone(), p0),
            Fn7::F427(k0, k1) => f427(*k0, k1.clone(), p0),
            Fn7::F435(k0, k1) => f435(*k0, k1.clone(), p0),
            Fn7::F439(k0) => f439(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F397(u64, Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: (u64, bool)) -> R<Option<(u64, bool)>> {
        match self {
            Fn8::F397(k0, k1) => f397(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F410,
}

impl Fn9 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn9::F410 => f410(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F412,
}

impl Fn10 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn10::F412 => f412(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F443,
}

impl Fn11 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn11::F443 => f443(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F472,
}

impl Fn12 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn12::F472 => f472(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F478,
}

impl Fn13 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn13::F478 => f478(p0),
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
                        Ok(v8) => {
                            let m361 = f102(v2, {
                                let (_, h360) = v8;
                                h360
                            })?;
                            match m361 {
                                Ok(_) => {
                                    Ok(Ok::<Adt17, Bytes>({
                                        let (h362, _) = v8;
                                        h362
                                    }))
                                }
                                Err(v9) => {
                                    Ok(Err::<Adt17, Bytes>(v9))
                                }
                            }
                        }
                        Err(v7) => {
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

pub fn f65(p401: u64, p402: List<Adt15>) -> R<List<Adt15>> {
    let m400 = (p401, p402);
    {
        let mut m400 = m400;
        loop {
            let step = {
                let (v0, v1) = m400;
                let m397 = v0;
                if m397 == 0 {
                    Err::<(u64, List<Adt15>), R<List<Adt15>>>(Ok(v1))
                } else {
                    let v2 = m397 - 1;
                    let v3: List<Adt15> = f107(v1.clone(), v1.clone());
                    let a398 = f108(v3.clone())?;
                    let a399 = f108(v1.clone())?;
                    if nat_eq(a398, a399) {
                        Err::<(u64, List<Adt15>), R<List<Adt15>>>(Ok(v1))
                    } else {
                        Ok::<(u64, List<Adt15>), R<List<Adt15>>>((v2, v3))
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

pub fn f66(v0: List<Adt12>, v1: List<Adt12>) -> R<List<Adt15>> {
    let m403 = v0;
    match m403.uncons() {
        None => {
            Ok(List::<Adt15>::nil())
        }
        Some((v2, v3)) => {
            if f109(f110(v2.clone())) {
                let v6: Adt15 = {
                    let m404 = f111(v1.clone(), v2.clone())?;
                    match m404 {
                        Ok(v5) => {
                            Adt15::C0(f112(v2), Bytes::lit(&[]), {
                                let (h405, _) = v5;
                                h405
                            }, {
                                let (_, h406) = v5;
                                h406
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

pub fn f67(p411: u64, p412: List<Bytes>, p413: List<Bytes>, v3: List<Adt15>) -> List<Bytes> {
    let m410 = (p411, (p412, p413));
    {
        let mut m410 = m410;
        loop {
            let step = {
                let (v0, (v1, v2)) = m410;
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
                Ok(next) => { m410 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f68(v0: List<Adt12>) -> List<Bytes> {
    let m414 = v0;
    match m414.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let v3: u64 = f110(v1.clone());
            let a415 = v3;
            let a416 = 0u64;
            if if nat_eq(a415, a416) {
                true
            } else {
                let a417 = v3;
                let a418 = 2u64;
                if nat_eq(a417, a418) {
                    true
                } else {
                    let a419 = v3;
                    let a420 = 3u64;
                    if nat_eq(a419, a420) {
                        true
                    } else {
                        let a421 = v3;
                        let a422 = 4u64;
                        if nat_eq(a421, a422) {
                            true
                        } else {
                            let a423 = v3;
                            let a424 = 5u64;
                            if nat_eq(a423, a424) {
                                true
                            } else {
                                let a425 = v3;
                                let a426 = 7u64;
                                if nat_eq(a425, a426) {
                                    true
                                } else {
                                    let a427 = v3;
                                    let a428 = 8u64;
                                    nat_eq(a427, a428)
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
    let m429 = v0;
    match m429.uncons() {
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

pub fn f70(p438: u64, p439: List<(Bytes, (List<Bytes>, Bytes))>, p440: List<Bytes>, p441: List<Bytes>) -> (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>) {
    let m437 = (p438, (p439, (p440, p441)));
    {
        let mut m437 = m437;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m437;
                let m430 = v0;
                if m430 == 0 {
                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v3, v1))
                } else {
                    let v4 = m430 - 1;
                    let m431 = f120(v1.clone(), v2.clone());
                    match m431 {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v3, v1))
                        }
                        Some(v5) => {
                            Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v4, ({
                                let (_, h432) = v5.clone();
                                h432
                            }, (List::cons({
                                let (h433, _) = {
                                    let (h434, _) = v5.clone();
                                    h434
                                };
                                h433
                            }, v2), List::cons({
                                let (_, h435) = {
                                    let (h436, _) = v5;
                                    h436
                                };
                                h435
                            }, v3)))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m437 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f71(v0: List<Adt12>, v1: List<Adt15>, v2: List<Adt12>) -> List<Bytes> {
    let m442 = v0;
    match m442.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            let v5: u64 = f110(v3.clone());
            let a443 = v5;
            let a444 = 5u64;
            if if nat_eq(a443, a444) {
                true
            } else {
                let a445 = v5;
                let a446 = 7u64;
                nat_eq(a445, a446)
            } {
                let m449 = f121(v2.clone(), {
                    let a447 = v5;
                    let a448 = 5u64;
                    if nat_eq(a447, a448) {
                        0u64
                    } else {
                        2u64
                    }
                }, f112(v3.clone()));
                match m449 {
                    None => {
                        List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f122(v5), List::cons(f112(v3.clone()), List::cons(f8(Str::lit(": no input defines ")), List::cons(f112(v3), List::cons(f8(Str::lit(", so its members join no definition (Web IDL 2.2 and 2.7)\u{a}")), List::<Bytes>::nil()))))))), f71(v4, v1, v2))
                    }
                    Some(_) => {
                        f71(v4, v1, v2)
                    }
                }
            } else {
                let a450 = v5;
                let a451 = 8u64;
                if nat_eq(a450, a451) {
                    let m452 = f121(v2.clone(), 0u64, f112(v3.clone()));
                    match m452 {
                        None => {
                            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f112(v3.clone()), List::cons(f8(Str::lit(" includes ")), List::cons(f123(v3.clone()), List::cons(f8(Str::lit(": no input defines the interface ")), List::cons(f112(v3), List::cons(f8(Str::lit(" (Web IDL 2.3)\u{a}")), List::<Bytes>::nil())))))))), f71(v4, v1, v2))
                        }
                        Some(_) => {
                            f71(v4, v1, v2)
                        }
                    }
                } else {
                    if f109(v5) {
                        let m453 = f114(v1.clone(), f112(v3.clone()));
                        match m453 {
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
    let m454 = v0;
    match m454.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons({
                let (h455, _) = v1;
                h455
            }, List::cons(f8(Str::lit(": it refers back to itself through its members, and the translator emits non-recursive declarations only\u{a}")), List::<Bytes>::nil())))), f72(v2))
        }
    }
}

pub fn f73(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m456 = v0;
    match m456.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m457 = v3.clone();
            match m457.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a460 = v2;
                    let a461 = {
                        let a458 = v1.clone();
                        let a459 = f73(v3, v1);
                        append_bytes(a458, a459)
                    };
                    append_bytes(a460, a461)
                }
            }
        }
    }
}

pub fn f74(p464: List<Bytes>, p465: List<Bytes>) -> List<Bytes> {
    let m463 = (p464, p465);
    {
        let mut m463 = m463;
        loop {
            let step = {
                let (v0, v1) = m463;
                let m462 = v0;
                match m462.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m463 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f75(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a466 = v2;
    let a467 = 0u64;
    if nat_eq(a466, a467) {
        Ok(f124(v0, 0u64))
    } else {
        let (_, h468) = {
            let (h469, _) = f125(Fn3::F126(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h469
        };
        Ok(h468)
    }
}

pub fn f76() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f77(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m474 = {
        let a471 = v0;
        let a472 = v2;
        let a473 = {
            let a470 = v1.clone();
            length_bytes(a470)
        };
        slice_bytes(a471, a472, a473)
    };
    match m474 {
        None => {
            false
        }
        Some(v3) => {
            let a475 = v3;
            let a476 = v1;
            equal(a475, a476)
        }
    }
}

pub fn f78(v0: Bytes) -> R<Bytes> {
    let (_, h477) = {
        let (h478, _) = f125(Fn3::F127(v0.clone()), {
            let a480 = {
                let a479 = v0;
                length_bytes(a479)
            };
            let a481 = 1u64;
            nat_add(a480, a481)?
        }, (0u64, Bytes::lit(&[])))?;
        h478
    };
    Ok(h477)
}

pub fn f79(v0: Bytes) -> R<u64> {
    let (h482, _) = f128(Fn4::F129(v0.clone()), {
        let a484 = {
            let a483 = v0;
            length_bytes(a483)
        };
        let a485 = 1u64;
        nat_add(a484, a485)?
    }, 0u64)?;
    Ok(h482)
}

pub fn f80(v0: Bytes) -> Result<Adt16, Bytes> {
    Err::<Adt16, Bytes>(v0)
}

pub fn f81(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a486 = v0;
        utf8_encode(a486)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f51(v1)?, List::<Bytes>::nil())))))
}

pub fn f82(v0: Bytes) -> R<Result<u64, Bytes>> {
    if if f84(v0.clone(), 0u64, Str::lit("<?xml")) {
        f130(f85(v0.clone(), 5u64)?)?
    } else {
        false
    } {
        let m488 = f131(v0.clone(), {
            let a487 = Str::lit("?>");
            utf8_encode(a487)
        }, 5u64)?;
        match m488 {
            None => {
                Ok(f132(f81(Str::lit("the XML declaration is not closed (XML 1.0 [23])"), 0u64)?))
            }
            Some(v1) => {
                let m490 = f131(f32(v0.clone(), 0u64, v1), {
                    let a489 = Str::lit("encoding");
                    utf8_encode(a489)
                }, 5u64)?;
                match m490 {
                    None => {
                        Ok(Ok::<u64, Bytes>({
                            let a491 = v1;
                            let a492 = 2u64;
                            nat_add(a491, a492)?
                        }))
                    }
                    Some(v2) => {
                        let v3: u64 = f133(v0.clone(), {
                            let a493 = v2;
                            let a494 = 8u64;
                            nat_add(a493, a494)?
                        })?;
                        let v4: u64 = f133(v0.clone(), {
                            let a495 = v3;
                            let a496 = 1u64;
                            nat_add(a495, a496)?
                        })?;
                        let _: u64 = f85(v0.clone(), v4)?;
                        let v7: u64 = {
                            let m499 = f131(v0.clone(), f32(v0.clone(), v4, 1u64), {
                                let a497 = v4;
                                let a498 = 1u64;
                                nat_add(a497, a498)?
                            })?;
                            match m499 {
                                None => {
                                    v1
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        if f134(f32(v0, {
                            let a500 = v4;
                            let a501 = 1u64;
                            nat_add(a500, a501)?
                        }, {
                            let a504 = v7;
                            let a505 = {
                                let a502 = v4;
                                let a503 = 1u64;
                                nat_add(a502, a503)?
                            };
                            nat_sub(a504, a505)
                        })) {
                            Ok(Ok::<u64, Bytes>({
                                let a506 = v1;
                                let a507 = 2u64;
                                nat_add(a506, a507)?
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

pub fn f83(v0: Bytes, p512: u64, p513: u64) -> R<Result<u64, Bytes>> {
    let m511 = (p512, p513);
    {
        let mut m511 = m511;
        loop {
            let step = {
                let (v1, v2) = m511;
                let m508 = v2;
                if m508 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f132(f81(Str::lit("the prolog is longer than the input"), v1)?)))
                } else {
                    let v3 = m508 - 1;
                    let v4: u64 = f133(v0.clone(), v1)?;
                    if f84(v0.clone(), v4, Str::lit("<!--")) {
                        let m509 = f135(v0.clone(), v4)?;
                        match m509 {
                            Ok(v6) => {
                                Ok::<(u64, u64), R<Result<u64, Bytes>>>((v6, v3))
                            }
                            Err(v5) => {
                                Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f132(v5)))
                            }
                        }
                    } else {
                        if f84(v0.clone(), v4, Str::lit("<?")) {
                            let m510 = f136(v0.clone(), v4)?;
                            match m510 {
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
                Ok(next) => { m511 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f84(v0: Bytes, v1: u64, v2: Str) -> bool {
    f77(v0, {
        let a514 = v2;
        utf8_encode(a514)
    }, v1)
}

pub fn f85(v0: Bytes, v1: u64) -> R<u64> {
    f137(v0, v1)
}

pub fn f86(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Result<(Adt16, u64), Bytes>> {
    let m515 = v2;
    if m515 == 0 {
        Ok(f138(f81(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v4 = m515 - 1;
        let a516 = v3;
        let a517 = 0u64;
        if nat_eq(a516, a517) {
            let v5: u64 = f139(v0.clone(), {
                let a518 = v1;
                let a519 = 1u64;
                nat_add(a518, a519)?
            })?;
            let a522 = v5;
            let a523 = {
                let a520 = v1;
                let a521 = 1u64;
                nat_add(a520, a521)?
            };
            if nat_eq(a522, a523) {
                Ok(f138(f81(Str::lit("an element must start with a Name (XML 1.0 [40])"), v1)?))
            } else {
                let v6: Bytes = f32(v0.clone(), {
                    let a524 = v1;
                    let a525 = 1u64;
                    nat_add(a524, a525)?
                }, {
                    let a528 = v5;
                    let a529 = {
                        let a526 = v1;
                        let a527 = 1u64;
                        nat_add(a526, a527)?
                    };
                    nat_sub(a528, a529)
                });
                let m530 = f140(v0.clone(), v5, v4, List::<Bytes>::nil())?;
                match m530 {
                    Ok(v8) => {
                        if f84(v0.clone(), {
                            let (_, h531) = v8.clone();
                            h531
                        }, Str::lit("/>")) {
                            Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C0(v6, std::rc::Rc::new({
                                let (h532, _) = v8;
                                h532
                            }), std::rc::Rc::new(Adt16::C4)), {
                                let a534 = {
                                    let (_, h533) = v8;
                                    h533
                                };
                                let a535 = 2u64;
                                nat_add(a534, a535)?
                            })))
                        } else {
                            let m539 = f86(v0.clone(), {
                                let a537 = {
                                    let (_, h536) = v8;
                                    h536
                                };
                                let a538 = 1u64;
                                nat_add(a537, a538)?
                            }, v4, 1u64)?;
                            match m539 {
                                Ok(v10) => {
                                    let v11: u64 = {
                                        let a541 = {
                                            let (_, h540) = v10.clone();
                                            h540
                                        };
                                        let a542 = 2u64;
                                        nat_add(a541, a542)?
                                    };
                                    let v12: u64 = f139(v0.clone(), v11)?;
                                    if f35(f32(v0.clone(), v11, {
                                        let a543 = v12;
                                        let a544 = v11;
                                        nat_sub(a543, a544)
                                    }), v6.clone()) {
                                        let v13: u64 = f133(v0.clone(), v12)?;
                                        let a545 = f85(v0, v13)?;
                                        let a546 = 62u64;
                                        if nat_eq(a545, a546) {
                                            Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C0(v6, std::rc::Rc::new({
                                                let (h547, _) = v8;
                                                h547
                                            }), std::rc::Rc::new({
                                                let (h548, _) = v10;
                                                h548
                                            })), {
                                                let a549 = v13;
                                                let a550 = 1u64;
                                                nat_add(a549, a550)?
                                            })))
                                        } else {
                                            Ok(f138(f81(Str::lit("an end tag must close with > (XML 1.0 [42])"), v13)?))
                                        }
                                    } else {
                                        Ok(f138(f81(Str::lit("an end tag must name the element its start tag opened (XML 1.0 WFC: Element Type Match)"), {
                                            let (_, h551) = v10;
                                            h551
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
            let a552 = f85(v0.clone(), v1)?;
            let a553 = 256u64;
            if nat_eq(a552, a553) {
                Ok(f138(f81(Str::lit("the input ends before an element is closed (XML 1.0 [39])"), v1)?))
            } else {
                if f84(v0.clone(), v1, Str::lit("</")) {
                    Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C4, v1)))
                } else {
                    if f84(v0.clone(), v1, Str::lit("<!--")) {
                        let m554 = f135(v0.clone(), v1)?;
                        match m554 {
                            Ok(v15) => {
                                f86(v0, v15, v4, 1u64)
                            }
                            Err(v14) => {
                                Ok(f138(v14))
                            }
                        }
                    } else {
                        if f84(v0.clone(), v1, Str::lit("<![CDATA[")) {
                            let m558 = f131(v0.clone(), {
                                let a555 = Str::lit("]]>");
                                utf8_encode(a555)
                            }, {
                                let a556 = v1;
                                let a557 = 9u64;
                                nat_add(a556, a557)?
                            })?;
                            match m558 {
                                None => {
                                    Ok(f138(f81(Str::lit("a CDATA section is not closed (XML 1.0 [18])"), v1)?))
                                }
                                Some(v16) => {
                                    let m561 = f86(v0.clone(), {
                                        let a559 = v16;
                                        let a560 = 3u64;
                                        nat_add(a559, a560)?
                                    }, v4, 1u64)?;
                                    match m561 {
                                        Ok(v18) => {
                                            Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C3(std::rc::Rc::new(Adt16::C2(f32(v0, {
                                                let a562 = v1;
                                                let a563 = 9u64;
                                                nat_add(a562, a563)?
                                            }, {
                                                let a566 = v16;
                                                let a567 = {
                                                    let a564 = v1;
                                                    let a565 = 9u64;
                                                    nat_add(a564, a565)?
                                                };
                                                nat_sub(a566, a567)
                                            }))), std::rc::Rc::new({
                                                let (h568, _) = v18;
                                                h568
                                            })), {
                                                let (_, h569) = v18;
                                                h569
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
                                let m570 = f136(v0.clone(), v1)?;
                                match m570 {
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
                                    let a571 = f85(v0.clone(), v1)?;
                                    let a572 = 60u64;
                                    if nat_eq(a571, a572) {
                                        let m573 = f86(v0.clone(), v1, v4, 0u64)?;
                                        match m573 {
                                            Ok(v22) => {
                                                let m575 = f86(v0, {
                                                    let (_, h574) = v22;
                                                    h574
                                                }, v4, 1u64)?;
                                                match m575 {
                                                    Ok(v24) => {
                                                        Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C3(std::rc::Rc::new({
                                                            let (h576, _) = v22;
                                                            h576
                                                        }), std::rc::Rc::new({
                                                            let (h577, _) = v24;
                                                            h577
                                                        })), {
                                                            let (_, h578) = v24;
                                                            h578
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
                                        let m579 = f141(v0.clone(), v1, v4, Bytes::lit(&[]), false, 0u64)?;
                                        match m579 {
                                            Ok(v26) => {
                                                let m581 = f86(v0, {
                                                    let (_, h580) = v26;
                                                    h580
                                                }, v4, 1u64)?;
                                                match m581 {
                                                    Ok(v28) => {
                                                        Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C3(std::rc::Rc::new(Adt16::C2({
                                                            let (h582, _) = v26;
                                                            h582
                                                        })), std::rc::Rc::new({
                                                            let (h583, _) = v28;
                                                            h583
                                                        })), {
                                                            let (_, h584) = v28;
                                                            h584
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
    let m585 = v0;
    match m585 {
        Adt16::C0(_, _, _) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt16::C1(v2, v3, r590) => {
            let v4 = (*r590).clone();
            let m586 = f88(v4, v1);
            match m586 {
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
                                let a588 = {
                                    let a587 = v2;
                                    length_bytes(a587)
                                };
                                let a589 = 6u64;
                                nat_sub(a588, a589)
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
        let m591 = f147(v0, Bytes::lit(&[]));
        match m591 {
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
            let m592 = f147(v0, v2.clone());
            match m592 {
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
        let (h593, _) = v0;
        h593
    }, f149()) {
        f35({
            let (_, h594) = v0;
            h594
        }, {
            let a595 = v1;
            utf8_encode(a595)
        })
    } else {
        false
    }
}

pub fn f91(p599: Adt16, v1: Bytes) -> Option<Bytes> {
    let m598 = p599;
    {
        let mut m598 = m598;
        loop {
            let step = {
                let v0 = m598;
                let m596 = v0;
                match m596 {
                    Adt16::C0(_, _, _) => {
                        Err::<Adt16, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt16::C1(v2, v3, r597) => {
                        let v4 = (*r597).clone();
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
                Ok(next) => { m598 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f92(v0: Adt16, v1: List<(Bytes, Bytes)>, v2: Bytes) -> R<Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>> {
    let m600 = v0;
    match m600 {
        Adt16::C0(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt16::C1(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt16::C2(_) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt16::C3(r611, r612) => {
            let v3 = (*r611).clone();
            let v4 = (*r612).clone();
            let m601 = f92(v4, v1.clone(), v2.clone())?;
            match m601 {
                Ok(v6) => {
                    let m602 = v3;
                    match m602 {
                        Adt16::C0(v7, r609, r610) => {
                            let v8 = (*r609).clone();
                            let v9 = (*r610).clone();
                            let m603 = f88(v8.clone(), v1);
                            match m603 {
                                Ok(v11) => {
                                    let m604 = f89(v11.clone(), v7)?;
                                    match m604 {
                                        Ok(v13) => {
                                            if f90(v13, Str::lit("simpleType")) {
                                                let m605 = f91(v8, f8(Str::lit("name")));
                                                match m605 {
                                                    None => {
                                                        Ok(f150(f8(Str::lit("a top-level simpleType must have a name (XSD 1.1 Part 2, section 4.1.2)"))))
                                                    }
                                                    Some(v14) => {
                                                        let m606 = f151(v14.clone(), v9, v11, v2)?;
                                                        match m606 {
                                                            Ok(v16) => {
                                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::cons((f152(v14)?, (match v16.clone() {
                                                                    Adt19::C0(_, h607) => {
                                                                        h607
                                                                    }
                                                                }, match v16 {
                                                                    Adt19::C0(h608, _) => {
                                                                        h608
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
    let m613 = v0;
    match m613.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h614, _) = v1;
                h614
            }, f93(v2))
        }
    }
}

pub fn f94(p617: List<Bytes>) -> Option<Bytes> {
    let m616 = p617;
    {
        let mut m616 = m616;
        loop {
            let step = {
                let v0 = m616;
                let m615 = v0;
                match m615.uncons() {
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
                Ok(next) => { m616 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f95(p623: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<Bytes> {
    let m622 = p623;
    {
        let mut m622 = m622;
        loop {
            let step = {
                let v0 = m622;
                let m618 = v0;
                match m618.uncons() {
                    None => {
                        Err::<List<(Bytes, (List<Bytes>, Bytes))>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        let m621 = f153({
                            let (h619, _) = {
                                let (_, h620) = v2;
                                h620
                            };
                            h619
                        }, v1.clone());
                        match m621 {
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
                Ok(next) => { m622 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f96(p634: u64, p635: List<(Bytes, (List<Bytes>, Bytes))>, p636: List<Bytes>, p637: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m633 = (p634, (p635, (p636, p637)));
    {
        let mut m633 = m633;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m633;
                let m624 = v0;
                if m624 == 0 {
                    let m625 = v1;
                    match m625.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(f154(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                        }
                    }
                } else {
                    let v6 = m624 - 1;
                    let m626 = v1.clone();
                    match m626.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            let m627 = f120(v1, v2.clone());
                            match m627 {
                                None => {
                                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(f154(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                                }
                                Some(v9) => {
                                    Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>((v6, ({
                                        let (_, h628) = v9.clone();
                                        h628
                                    }, (List::cons({
                                        let (h629, _) = {
                                            let (h630, _) = v9.clone();
                                            h630
                                        };
                                        h629
                                    }, v2), List::cons({
                                        let (_, h631) = {
                                            let (h632, _) = v9;
                                            h632
                                        };
                                        h631
                                    }, v3)))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m633 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f97(v0: Bytes) -> R<u64> {
    let (h638, _) = f128(Fn4::F155(v0.clone()), {
        let a640 = {
            let a639 = v0;
            length_bytes(a639)
        };
        let a641 = 1u64;
        nat_add(a640, a641)?
    }, 0u64)?;
    Ok(h638)
}

pub fn f98(v0: Bytes, v1: u64) -> R<u64> {
    let (h642, _) = f128(Fn4::F156(v0.clone()), {
        let a644 = {
            let a643 = v0;
            length_bytes(a643)
        };
        let a645 = 1u64;
        nat_add(a644, a645)?
    }, v1)?;
    Ok(h642)
}

pub fn f99(v0: Bytes, v1: u64) -> R<bool> {
    if if if f84(v0.clone(), v1, Str::lit("---")) {
        true
    } else {
        f84(v0.clone(), v1, Str::lit("..."))
    } {
        if f157(f100(v0.clone(), {
            let a646 = v1;
            let a647 = 3u64;
            nat_add(a646, a647)?
        })?) {
            true
        } else {
            f158(f100(v0.clone(), {
                let a648 = v1;
                let a649 = 3u64;
                nat_add(a648, a649)?
            })?)
        }
    } else {
        false
    } {
        let a650 = f159(v0, v1)?;
        let a651 = 0u64;
        Ok(nat_eq(a650, a651))
    } else {
        Ok(false)
    }
}

pub fn f100(v0: Bytes, v1: u64) -> R<u64> {
    f137(v0, v1)
}

pub fn f101(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: List<Bytes>) -> R<Result<(Adt17, u64), Bytes>> {
    let m652 = v3;
    if m652 == 0 {
        Ok(Err::<(Adt17, u64), Bytes>(f81(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v8 = m652 - 1;
        let a653 = v4;
        let a654 = 0u64;
        if nat_eq(a653, a654) {
            let v9: u64 = f160(v0.clone(), v1)?;
            let v10: u64 = {
                let a655 = v9;
                let a656 = v1;
                nat_sub(a655, a656)
            };
            let a657 = f100(v0.clone(), v9)?;
            let a658 = 9u64;
            if nat_eq(a657, a658) {
                let v11: u64 = f161(v0.clone(), v9)?;
                if if f162(v0.clone(), v11)? {
                    true
                } else {
                    f163(v0.clone(), v11)?
                } {
                    Ok(Err::<(Adt17, u64), Bytes>(f81(Str::lit("a tab cannot indent a block node (YAML 1.2.2 [63])"), v11)?))
                } else {
                    let m659 = f164(v0.clone(), v11)?;
                    match m659 {
                        None => {
                            f165(v0, v11, v2)
                        }
                        Some(v12) => {
                            Ok(Err::<(Adt17, u64), Bytes>(v12))
                        }
                    }
                }
            } else {
                if f162(v0.clone(), v9)? {
                    Ok(f166(true, f101(v0, v9, v2, v8, 2u64, false, v10, List::<Bytes>::nil())?))
                } else {
                    let m660 = f164(v0.clone(), v9)?;
                    match m660 {
                        None => {
                            if f163(v0.clone(), v9)? {
                                Ok(f166(false, f101(v0, v9, v2, v8, 3u64, false, v10, List::<Bytes>::nil())?))
                            } else {
                                f165(v0, v9, v2)
                            }
                        }
                        Some(v13) => {
                            Ok(Err::<(Adt17, u64), Bytes>(v13))
                        }
                    }
                }
            }
        } else {
            let a661 = v4;
            let a662 = 1u64;
            if nat_eq(a661, a662) {
                let v14: u64 = f161(v0.clone(), v1)?;
                let v15: u64 = f100(v0.clone(), v14)?;
                if if f158(v15) {
                    true
                } else {
                    let a663 = v15;
                    let a664 = 35u64;
                    if nat_eq(a663, a664) {
                        let a665 = v1;
                        let a666 = v14;
                        nat_lt(a665, a666)
                    } else {
                        false
                    }
                } {
                    let v16: u64 = f98(v0.clone(), f167(v0.clone(), v14)?)?;
                    let a668 = {
                        let a667 = v0.clone();
                        length_bytes(a667)
                    };
                    let a669 = v16;
                    if if nat_le(a668, a669) {
                        true
                    } else {
                        f99(v0.clone(), v16)?
                    } {
                        Ok(Err::<(Adt17, u64), Bytes>(f168(v14)?))
                    } else {
                        let v17: u64 = {
                            let a670 = f160(v0.clone(), v16)?;
                            let a671 = v16;
                            nat_sub(a670, a671)
                        };
                        let a672 = v2;
                        let a673 = v17;
                        if nat_le(a672, a673) {
                            f101(v0, v16, v2, v8, 0u64, false, 0u64, List::<Bytes>::nil())
                        } else {
                            if if if f169(v5) {
                                let a674 = v17;
                                let a675 = v6;
                                nat_eq(a674, a675)
                            } else {
                                false
                            } {
                                f162(v0.clone(), {
                                    let a676 = v16;
                                    let a677 = v17;
                                    nat_add(a676, a677)?
                                })?
                            } else {
                                false
                            } {
                                Ok(f166(true, f101(v0, {
                                    let a678 = v16;
                                    let a679 = v17;
                                    nat_add(a678, a679)?
                                }, v2, v8, 2u64, false, v17, List::<Bytes>::nil())?))
                            } else {
                                Ok(Err::<(Adt17, u64), Bytes>(f168(v14)?))
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
                        let m680 = f164(v0.clone(), v14)?;
                        match m680 {
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
                                Ok(Err::<(Adt17, u64), Bytes>(v18))
                            }
                        }
                    }
                }
            } else {
                let a681 = v4;
                let a682 = 2u64;
                if nat_eq(a681, a682) {
                    let m687 = f101(v0.clone(), {
                        let a683 = v1;
                        let a684 = 1u64;
                        nat_add(a683, a684)?
                    }, {
                        let a685 = v6;
                        let a686 = 1u64;
                        nat_add(a685, a686)?
                    }, v8, 1u64, true, v6, List::<Bytes>::nil())?;
                    match m687 {
                        Ok(v20) => {
                            let v21: u64 = f98(v0.clone(), {
                                let (_, h688) = v20;
                                h688
                            })?;
                            let v22: u64 = {
                                let a689 = f160(v0.clone(), v21)?;
                                let a690 = v21;
                                nat_sub(a689, a690)
                            };
                            let a692 = {
                                let a691 = v0.clone();
                                length_bytes(a691)
                            };
                            let a693 = v21;
                            if nat_le(a692, a693) {
                                Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C7(std::rc::Rc::new({
                                    let (h694, _) = v20;
                                    h694
                                }), std::rc::Rc::new(Adt17::C6)), v21)))
                            } else {
                                let a695 = v22;
                                let a696 = v6;
                                if if nat_eq(a695, a696) {
                                    f162(v0.clone(), {
                                        let a697 = v21;
                                        let a698 = v22;
                                        nat_add(a697, a698)?
                                    })?
                                } else {
                                    false
                                } {
                                    let m701 = f101(v0, {
                                        let a699 = v21;
                                        let a700 = v22;
                                        nat_add(a699, a700)?
                                    }, v2, v8, 2u64, false, v6, List::<Bytes>::nil())?;
                                    match m701 {
                                        Ok(v24) => {
                                            Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C7(std::rc::Rc::new({
                                                let (h702, _) = v20;
                                                h702
                                            }), std::rc::Rc::new({
                                                let (h703, _) = v24;
                                                h703
                                            })), {
                                                let (_, h704) = v24;
                                                h704
                                            })))
                                        }
                                        Err(v23) => {
                                            Ok(Err::<(Adt17, u64), Bytes>(v23))
                                        }
                                    }
                                } else {
                                    let a705 = v6;
                                    let a706 = v22;
                                    if nat_lt(a705, a706) {
                                        Ok(Err::<(Adt17, u64), Bytes>(f81(Str::lit("a line is indented more than its block sequence entry allows (YAML 1.2.2 [183])"), {
                                            let a707 = v21;
                                            let a708 = v22;
                                            nat_add(a707, a708)?
                                        })?))
                                    } else {
                                        Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C7(std::rc::Rc::new({
                                            let (h709, _) = v20;
                                            h709
                                        }), std::rc::Rc::new(Adt17::C6)), v21)))
                                    }
                                }
                            }
                        }
                        Err(v19) => {
                            Ok(Err::<(Adt17, u64), Bytes>(v19))
                        }
                    }
                } else {
                    let m710 = f164(v0.clone(), v1)?;
                    match m710 {
                        None => {
                            let m711 = f170(v0.clone(), v1)?;
                            match m711 {
                                None => {
                                    Ok(Err::<(Adt17, u64), Bytes>(f81(Str::lit("a block mapping entry must be an implicit key and : (YAML 1.2.2 [188])"), v1)?))
                                }
                                Some(v26) => {
                                    let v27: Bytes = f171(v0.clone(), v1)?;
                                    if f172(v27.clone(), v7.clone()) {
                                        Ok(Err::<(Adt17, u64), Bytes>(f81(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                    } else {
                                        let m716 = f101(v0.clone(), {
                                            let a712 = v26;
                                            let a713 = 1u64;
                                            nat_add(a712, a713)?
                                        }, {
                                            let a714 = v6;
                                            let a715 = 1u64;
                                            nat_add(a714, a715)?
                                        }, v8, 1u64, false, v6, List::<Bytes>::nil())?;
                                        match m716 {
                                            Ok(v29) => {
                                                let v30: u64 = f98(v0.clone(), {
                                                    let (_, h717) = v29;
                                                    h717
                                                })?;
                                                let v31: u64 = {
                                                    let a718 = f160(v0.clone(), v30)?;
                                                    let a719 = v30;
                                                    nat_sub(a718, a719)
                                                };
                                                let a721 = {
                                                    let a720 = v0.clone();
                                                    length_bytes(a720)
                                                };
                                                let a722 = v30;
                                                if nat_le(a721, a722) {
                                                    Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C8(v27, std::rc::Rc::new({
                                                        let (h723, _) = v29;
                                                        h723
                                                    }), std::rc::Rc::new(Adt17::C6)), v30)))
                                                } else {
                                                    let a724 = v31;
                                                    let a725 = v6;
                                                    if if if nat_eq(a724, a725) {
                                                        f169(f162(v0.clone(), {
                                                            let a726 = v30;
                                                            let a727 = v31;
                                                            nat_add(a726, a727)?
                                                        })?)
                                                    } else {
                                                        false
                                                    } {
                                                        f169(f99(v0.clone(), v30)?)
                                                    } else {
                                                        false
                                                    } {
                                                        let m730 = f101(v0, {
                                                            let a728 = v30;
                                                            let a729 = v31;
                                                            nat_add(a728, a729)?
                                                        }, v2, v8, 3u64, false, v6, List::cons(v27.clone(), v7))?;
                                                        match m730 {
                                                            Ok(v33) => {
                                                                Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C8(v27, std::rc::Rc::new({
                                                                    let (h731, _) = v29;
                                                                    h731
                                                                }), std::rc::Rc::new({
                                                                    let (h732, _) = v33;
                                                                    h732
                                                                })), {
                                                                    let (_, h733) = v33;
                                                                    h733
                                                                })))
                                                            }
                                                            Err(v32) => {
                                                                Ok(Err::<(Adt17, u64), Bytes>(v32))
                                                            }
                                                        }
                                                    } else {
                                                        let a734 = v6;
                                                        let a735 = v31;
                                                        if nat_lt(a734, a735) {
                                                            Ok(Err::<(Adt17, u64), Bytes>(f81(Str::lit("a line is indented more than its block mapping entry allows (YAML 1.2.2 [187])"), {
                                                                let a736 = v30;
                                                                let a737 = v31;
                                                                nat_add(a736, a737)?
                                                            })?))
                                                        } else {
                                                            Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C8(v27, std::rc::Rc::new({
                                                                let (h738, _) = v29;
                                                                h738
                                                            }), std::rc::Rc::new(Adt17::C6)), v30)))
                                                        }
                                                    }
                                                }
                                            }
                                            Err(v28) => {
                                                Ok(Err::<(Adt17, u64), Bytes>(v28))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v25) => {
                            Ok(Err::<(Adt17, u64), Bytes>(v25))
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
        let a739 = v0.clone();
        length_bytes(a739)
    };
    let a740 = v2;
    let a741 = v3;
    if if nat_lt(a740, a741) {
        if f84(v0.clone(), v2, Str::lit("...")) {
            f99(v0.clone(), v2)?
        } else {
            false
        }
    } else {
        false
    } {
        let m744 = f173(v0.clone(), {
            let a742 = v2;
            let a743 = 3u64;
            nat_add(a742, a743)?
        })?;
        match m744 {
            Ok(v5) => {
                let v6: u64 = f98(v0, v5)?;
                let a745 = v6;
                let a746 = v3;
                if nat_lt(a745, a746) {
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
        let a747 = v2;
        let a748 = v3;
        if if nat_lt(a747, a748) {
            f99(v0, v2)?
        } else {
            false
        } {
            Ok(Err::<u64, Bytes>(f81(Str::lit("a stream of more than one document is not read (YAML 1.2.2 [211])"), v2)?))
        } else {
            let a749 = v2;
            let a750 = v3;
            if nat_lt(a749, a750) {
                Ok(Err::<u64, Bytes>(f81(Str::lit("only comments may follow the root node (YAML 1.2.2 [207])"), v2)?))
            } else {
                Ok(Ok::<u64, Bytes>(v2))
            }
        }
    }
}

pub fn f103(v0: Adt17) -> R<Bytes> {
    let m751 = v0;
    match m751 {
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
        Adt17::C4(r752) => {
            let v8 = (*r752).clone();
            Ok(f177(f178(v8)?))
        }
        Adt17::C5(r753) => {
            let v9 = (*r753).clone();
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
    let a754 = v0;
    let a755 = v1;
    index_bytes(a754, a755)
}

pub fn f105(v0: Bytes) -> R<Result<List<Adt18>, Bytes>> {
    f181({
        let a757 = {
            let a756 = v0.clone();
            length_bytes(a756)
        };
        let a758 = 1u64;
        nat_add(a757, a758)?
    }, v0, 0u64, List::<Adt18>::nil())
}

pub fn f106(p766: u64, p767: List<Adt18>, p768: List<Adt12>) -> R<Result<List<Adt12>, Bytes>> {
    let m765 = (p766, (p767, p768));
    {
        let mut m765 = m765;
        loop {
            let step = {
                let (v0, (v1, v2)) = m765;
                let m759 = v0;
                if m759 == 0 {
                    Err::<(u64, (List<Adt18>, List<Adt12>)), R<Result<List<Adt12>, Bytes>>>(Ok(Err::<List<Adt12>, Bytes>(f8(Str::lit("the input has more definitions than tokens")))))
                } else {
                    let v3 = m759 - 1;
                    let m760 = v1.clone();
                    match m760.uncons() {
                        None => {
                            Err::<(u64, (List<Adt18>, List<Adt12>)), R<Result<List<Adt12>, Bytes>>>(Ok(Ok::<List<Adt12>, Bytes>(f182(v2, List::<Adt12>::nil()))))
                        }
                        Some((_, _)) => {
                            let m761 = f183(v1)?;
                            match m761 {
                                Ok(v7) => {
                                    let m762 = f184(v3, v7)?;
                                    match m762 {
                                        Ok(v9) => {
                                            Ok::<(u64, (List<Adt18>, List<Adt12>)), R<Result<List<Adt12>, Bytes>>>((v3, ({
                                                let (_, h763) = v9;
                                                h763
                                            }, List::cons({
                                                let (h764, _) = v9;
                                                h764
                                            }, v2))))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (List<Adt18>, List<Adt12>)), R<Result<List<Adt12>, Bytes>>>(Ok(Err::<List<Adt12>, Bytes>(v8)))
                                        }
                                    }
                                }
                                Err(v6) => {
                                    Err::<(u64, (List<Adt18>, List<Adt12>)), R<Result<List<Adt12>, Bytes>>>(Ok(Err::<List<Adt12>, Bytes>(v6)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m765 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f107(v0: List<Adt15>, v1: List<Adt15>) -> List<Adt15> {
    let m769 = v0;
    match m769.uncons() {
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
    let m770 = v0;
    match m770.uncons() {
        None => {
            Ok(0u64)
        }
        Some((v1, v2)) => {
            if f35(f115(v1), Bytes::lit(&[])) {
                f108(v2)
            } else {
                let a771 = f108(v2)?;
                let a772 = 1u64;
                nat_add(a771, a772)
            }
        }
    }
}

pub fn f109(v0: u64) -> bool {
    let a773 = v0;
    let a774 = 0u64;
    if nat_eq(a773, a774) {
        true
    } else {
        let a775 = v0;
        let a776 = 2u64;
        if nat_eq(a775, a776) {
            true
        } else {
            let a777 = v0;
            let a778 = 3u64;
            if nat_eq(a777, a778) {
                true
            } else {
                let a779 = v0;
                let a780 = 4u64;
                if nat_eq(a779, a780) {
                    true
                } else {
                    let a781 = v0;
                    let a782 = 9u64;
                    nat_eq(a781, a782)
                }
            }
        }
    }
}

pub fn f110(v0: Adt12) -> u64 {
    let m783 = v0;
    match m783 {
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
        let a784 = v2;
        let a785 = 0u64;
        if if nat_eq(a784, a785) {
            true
        } else {
            let a786 = v2;
            let a787 = 2u64;
            nat_eq(a786, a787)
        } {
            let m788 = f188(v0.clone(), v0.clone(), f112(v1.clone()));
            match m788 {
                None => {
                    let m792 = f189({
                        let a790 = {
                            let a789 = v0.clone();
                            length_list(a789)
                        };
                        let a791 = 1u64;
                        nat_add(a790, a791)?
                    }, v0, v1)?;
                    match m792 {
                        Ok(v5) => {
                            let a794 = {
                                let a793 = v5.clone();
                                length_list(a793)
                            };
                            let a795 = 0u64;
                            if nat_eq(a794, a795) {
                                Ok(Err::<(Bytes, List<Bytes>), Bytes>(f8(Str::lit("it has no data members, and LexLean refuses a structure with no fields"))))
                            } else {
                                let m796 = f190(f191(f192(v5.clone()))?, v5)?;
                                match m796 {
                                    Ok(v7) => {
                                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f193(v3, {
                                            let (h797, _) = v7;
                                            h797
                                        }), {
                                            let (_, h798) = v7;
                                            h798
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
            let a799 = v2;
            let a800 = 3u64;
            if nat_eq(a799, a800) {
                let m801 = f94(f194(v1.clone()));
                match m801 {
                    None => {
                        let m802 = f195(f191(f194(v1.clone()))?, f194(v1.clone()))?;
                        match m802 {
                            None => {
                                let m803 = f196(f191(f194(v1.clone()))?, f197(f194(v1)), List::<Bytes>::nil())?;
                                match m803 {
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
                let a804 = v2;
                let a805 = 4u64;
                if nat_eq(a804, a805) {
                    let v13: Adt14 = f199(v1);
                    if !f35(f200(v13.clone()), Bytes::lit(&[])) {
                        Ok(Err::<(Bytes, List<Bytes>), Bytes>(f200(v13)))
                    } else {
                        let a806 = f201(v13.clone());
                        let a807 = 1u64;
                        if nat_eq(a806, a807) {
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
    let m808 = v0;
    match m808 {
        Adt12::C0(_, v2, _, _, _, _) => {
            v2
        }
    }
}

pub fn f113(v0: Bytes, p811: List<Bytes>) -> bool {
    let m810 = p811;
    {
        let mut m810 = m810;
        loop {
            let step = {
                let v1 = m810;
                let m809 = v1;
                match m809.uncons() {
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
                Ok(next) => { m810 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f114(p814: List<Adt15>, v1: Bytes) -> Option<Adt15> {
    let m813 = p814;
    {
        let mut m813 = m813;
        loop {
            let step = {
                let v0 = m813;
                let m812 = v0;
                match m812.uncons() {
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
                Ok(next) => { m813 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f115(v0: Adt15) -> Bytes {
    let m815 = v0;
    match m815 {
        Adt15::C0(_, v2, _, _) => {
            v2
        }
    }
}

pub fn f116(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m816 = v0;
    match m816.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f116(v3, v1))
        }
    }
}

pub fn f117(v0: Adt15) -> List<Bytes> {
    let m817 = v0;
    match m817 {
        Adt15::C0(_, _, _, v4) => {
            v4
        }
    }
}

pub fn f118(v0: Adt15) -> Bytes {
    let m818 = v0;
    match m818 {
        Adt15::C0(v1, _, _, _) => {
            v1
        }
    }
}

pub fn f119(v0: Adt15) -> Bytes {
    let m819 = v0;
    match m819 {
        Adt15::C0(_, _, v3, _) => {
            v3
        }
    }
}

pub fn f120(v0: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)> {
    let m820 = v0;
    match m820.uncons() {
        None => {
            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
        }
        Some((v2, v3)) => {
            let m823 = f153({
                let (h821, _) = {
                    let (_, h822) = v2.clone();
                    h822
                };
                h821
            }, v1.clone());
            match m823 {
                None => {
                    Some((({
                        let (h824, _) = v2;
                        h824
                    }, {
                        let (_, h825) = {
                            let (_, h826) = v2;
                            h826
                        };
                        h825
                    }), v3))
                }
                Some(_) => {
                    let m827 = f120(v3, v1);
                    match m827 {
                        None => {
                            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h828, _) = v5;
                                h828
                            }, List::cons(v2, {
                                let (_, h829) = v5;
                                h829
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f121(p834: List<Adt12>, v1: u64, v2: Bytes) -> Option<Adt12> {
    let m833 = p834;
    {
        let mut m833 = m833;
        loop {
            let step = {
                let v0 = m833;
                let m830 = v0;
                match m830.uncons() {
                    None => {
                        Err::<List<Adt12>, Option<Adt12>>(None::<Adt12>)
                    }
                    Some((v3, v4)) => {
                        let a831 = f110(v3.clone());
                        let a832 = v1;
                        if if nat_eq(a831, a832) {
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
                Ok(next) => { m833 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f122(v0: u64) -> Bytes {
    let a835 = v0;
    let a836 = 5u64;
    if nat_eq(a835, a836) {
        f8(Str::lit("partial interface "))
    } else {
        let a837 = v0;
        let a838 = 7u64;
        if nat_eq(a837, a838) {
            f8(Str::lit("partial dictionary "))
        } else {
            f8(Str::lit(""))
        }
    }
}

pub fn f123(v0: Adt12) -> Bytes {
    let m839 = v0;
    match m839 {
        Adt12::C0(_, _, v3, _, _, _) => {
            v3
        }
    }
}

pub fn f124(v0: Bytes, v1: u64) -> Bytes {
    let m843 = {
        let a840 = v0;
        let a841 = v1;
        let a842 = 1u64;
        slice_bytes(a840, a841, a842)
    };
    match m843 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f125(v0: Fn3, p848: u64, p849: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m847 = (p848, p849);
    {
        let mut m847 = m847;
        loop {
            let step = {
                let (v1, v2) = m847;
                let m844 = v1;
                if m844 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m844 - 1;
                    let m846 = {
                        let c845: Fn3 = v0.clone();
                        c845.apply(v2.clone())?
                    };
                    match m846 {
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
                Ok(next) => { m847 = next; }
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

pub fn f128(v0: Fn4, p854: u64, p855: u64) -> R<(u64, bool)> {
    let m853 = (p854, p855);
    {
        let mut m853 = m853;
        loop {
            let step = {
                let (v1, v2) = m853;
                let m850 = v1;
                if m850 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m850 - 1;
                    let m852 = {
                        let c851: Fn4 = v0.clone();
                        c851.apply(v2)?
                    };
                    match m852 {
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
                Ok(next) => { m853 = next; }
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
        let (h856, _) = f128(Fn4::F210(v1.clone(), v0.clone()), {
            let a858 = {
                let a857 = v0.clone();
                length_bytes(a857)
            };
            let a859 = 1u64;
            nat_add(a858, a859)?
        }, v2)?;
        h856
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
    let (h860, _) = f128(Fn4::F211(v0.clone()), {
        let a862 = {
            let a861 = v0;
            length_bytes(a861)
        };
        let a863 = 1u64;
        nat_add(a862, a863)?
    }, v1)?;
    Ok(h860)
}

pub fn f134(v0: Bytes) -> bool {
    f172(v0, List::cons(f8(Str::lit("UTF-8")), List::cons(f8(Str::lit("UTf-8")), List::cons(f8(Str::lit("UtF-8")), List::cons(f8(Str::lit("Utf-8")), List::cons(f8(Str::lit("uTF-8")), List::cons(f8(Str::lit("uTf-8")), List::cons(f8(Str::lit("utF-8")), List::cons(f8(Str::lit("utf-8")), List::<Bytes>::nil())))))))))
}

pub fn f135(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let m867 = f131(v0.clone(), {
        let a864 = Str::lit("--");
        utf8_encode(a864)
    }, {
        let a865 = v1;
        let a866 = 4u64;
        nat_add(a865, a866)?
    })?;
    match m867 {
        None => {
            Ok(f132(f81(Str::lit("a comment is not closed (XML 1.0 [15])"), v1)?))
        }
        Some(v2) => {
            let a870 = f85(v0, {
                let a868 = v2;
                let a869 = 2u64;
                nat_add(a868, a869)?
            })?;
            let a871 = 62u64;
            if nat_eq(a870, a871) {
                Ok(Ok::<u64, Bytes>({
                    let a872 = v2;
                    let a873 = 3u64;
                    nat_add(a872, a873)?
                }))
            } else {
                Ok(f132(f81(Str::lit("-- is not allowed inside a comment (XML 1.0 [15])"), v2)?))
            }
        }
    }
}

pub fn f136(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f139(v0.clone(), {
        let a874 = v1;
        let a875 = 2u64;
        nat_add(a874, a875)?
    })?;
    let a878 = v2;
    let a879 = {
        let a876 = v1;
        let a877 = 2u64;
        nat_add(a876, a877)?
    };
    if nat_eq(a878, a879) {
        Ok(f132(f81(Str::lit("a processing instruction must start with a Name (XML 1.0 [16])"), v1)?))
    } else {
        let a882 = v2;
        let a883 = {
            let a880 = v1;
            let a881 = 5u64;
            nat_add(a880, a881)?
        };
        if if nat_eq(a882, a883) {
            f172(f32(v0.clone(), {
                let a884 = v1;
                let a885 = 2u64;
                nat_add(a884, a885)?
            }, 3u64), List::cons(f8(Str::lit("XML")), List::cons(f8(Str::lit("XMl")), List::cons(f8(Str::lit("XmL")), List::cons(f8(Str::lit("Xml")), List::cons(f8(Str::lit("xML")), List::cons(f8(Str::lit("xMl")), List::cons(f8(Str::lit("xmL")), List::cons(f8(Str::lit("xml")), List::<Bytes>::nil())))))))))
        } else {
            false
        } {
            Ok(f132(f81(Str::lit("the target xml is reserved; an XML declaration may appear only at the very start (XML 1.0 [17], [22])"), v1)?))
        } else {
            let m887 = f131(v0, {
                let a886 = Str::lit("?>");
                utf8_encode(a886)
            }, v2)?;
            match m887 {
                None => {
                    Ok(f132(f81(Str::lit("a processing instruction is not closed (XML 1.0 [16])"), v1)?))
                }
                Some(v3) => {
                    Ok(Ok::<u64, Bytes>({
                        let a888 = v3;
                        let a889 = 2u64;
                        nat_add(a888, a889)?
                    }))
                }
            }
        }
    }
}

pub fn f137(v0: Bytes, v1: u64) -> R<u64> {
    let m892 = {
        let a890 = v0;
        let a891 = v1;
        index_bytes(a890, a891)
    };
    match m892 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a937 = {
                let a913 = {
                    let a901 = {
                        let a895 = {
                            let a893 = v3;
                            let a894 = 128u8;
                            fixed_u8::bit_and(a893, a894)
                        };
                        let a896 = 128u8;
                        if equal(a895, a896) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a902 = {
                        let a899 = {
                            let a897 = v3;
                            let a898 = 64u8;
                            fixed_u8::bit_and(a897, a898)
                        };
                        let a900 = 64u8;
                        if equal(a899, a900) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a901, a902)?
                };
                let a914 = {
                    let a911 = {
                        let a905 = {
                            let a903 = v3;
                            let a904 = 32u8;
                            fixed_u8::bit_and(a903, a904)
                        };
                        let a906 = 32u8;
                        if equal(a905, a906) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a912 = {
                        let a909 = {
                            let a907 = v3;
                            let a908 = 16u8;
                            fixed_u8::bit_and(a907, a908)
                        };
                        let a910 = 16u8;
                        if equal(a909, a910) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a911, a912)?
                };
                nat_add(a913, a914)?
            };
            let a938 = {
                let a935 = {
                    let a923 = {
                        let a917 = {
                            let a915 = v3;
                            let a916 = 8u8;
                            fixed_u8::bit_and(a915, a916)
                        };
                        let a918 = 8u8;
                        if equal(a917, a918) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a924 = {
                        let a921 = {
                            let a919 = v3;
                            let a920 = 4u8;
                            fixed_u8::bit_and(a919, a920)
                        };
                        let a922 = 4u8;
                        if equal(a921, a922) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a923, a924)?
                };
                let a936 = {
                    let a933 = {
                        let a927 = {
                            let a925 = v3;
                            let a926 = 2u8;
                            fixed_u8::bit_and(a925, a926)
                        };
                        let a928 = 2u8;
                        if equal(a927, a928) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a934 = {
                        let a931 = {
                            let a929 = v3;
                            let a930 = 1u8;
                            fixed_u8::bit_and(a929, a930)
                        };
                        let a932 = 1u8;
                        if equal(a931, a932) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a933, a934)?
                };
                nat_add(a935, a936)?
            };
            nat_add(a937, a938)
        }
    }
}

pub fn f138(v0: Bytes) -> Result<(Adt16, u64), Bytes> {
    Err::<(Adt16, u64), Bytes>(v0)
}

pub fn f139(v0: Bytes, v1: u64) -> R<u64> {
    let (h939, _) = f128(Fn4::F212(v0.clone(), v1), {
        let a941 = {
            let a940 = v0;
            length_bytes(a940)
        };
        let a942 = 1u64;
        nat_add(a941, a942)?
    }, v1)?;
    Ok(h939)
}

pub fn f140(v0: Bytes, v1: u64, v2: u64, v3: List<Bytes>) -> R<Result<(Adt16, u64), Bytes>> {
    let m943 = v2;
    if m943 == 0 {
        Ok(f138(f81(Str::lit("a tag has more attributes than the input has bytes"), v1)?))
    } else {
        let v4 = m943 - 1;
        let v5: u64 = f133(v0.clone(), v1)?;
        let a944 = f85(v0.clone(), v5)?;
        let a945 = 62u64;
        if if nat_eq(a944, a945) {
            true
        } else {
            f84(v0.clone(), v5, Str::lit("/>"))
        } {
            Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C4, v5)))
        } else {
            let a946 = v5;
            let a947 = v1;
            if nat_eq(a946, a947) {
                Ok(f138(f81(Str::lit("attributes must be separated by white space (XML 1.0 [40])"), v1)?))
            } else {
                let v6: u64 = f139(v0.clone(), v5)?;
                let a948 = v6;
                let a949 = v5;
                if nat_eq(a948, a949) {
                    Ok(f138(f81(Str::lit("an attribute must start with a Name (XML 1.0 [41])"), v5)?))
                } else {
                    let v7: Bytes = f32(v0.clone(), v5, {
                        let a950 = v6;
                        let a951 = v5;
                        nat_sub(a950, a951)
                    });
                    if f172(v7.clone(), v3.clone()) {
                        Ok(f138(f81(Str::lit("an attribute name appears twice in one tag (XML 1.0 WFC: Unique Att Spec)"), v5)?))
                    } else {
                        let v8: u64 = f133(v0.clone(), v6)?;
                        let a952 = f85(v0.clone(), v8)?;
                        let a953 = 61u64;
                        if nat_eq(a952, a953) {
                            let v9: u64 = f133(v0.clone(), {
                                let a954 = v8;
                                let a955 = 1u64;
                                nat_add(a954, a955)?
                            })?;
                            let v10: u64 = f85(v0.clone(), v9)?;
                            let a956 = v10;
                            let a957 = 34u64;
                            if if nat_eq(a956, a957) {
                                true
                            } else {
                                let a958 = v10;
                                let a959 = 39u64;
                                nat_eq(a958, a959)
                            } {
                                let m962 = f141(v0.clone(), {
                                    let a960 = v9;
                                    let a961 = 1u64;
                                    nat_add(a960, a961)?
                                }, v4, Bytes::lit(&[]), true, v10)?;
                                match m962 {
                                    Ok(v12) => {
                                        let m964 = f140(v0, {
                                            let (_, h963) = v12;
                                            h963
                                        }, v4, List::cons(v7.clone(), v3))?;
                                        match m964 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C1(v7, {
                                                    let (h965, _) = v12;
                                                    h965
                                                }, std::rc::Rc::new({
                                                    let (h966, _) = v14;
                                                    h966
                                                })), {
                                                    let (_, h967) = v14;
                                                    h967
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

pub fn f141(v0: Bytes, p1007: u64, p1008: u64, p1009: Bytes, v4: bool, v5: u64) -> R<Result<(Bytes, u64), Bytes>> {
    let m1006 = (p1007, (p1008, p1009));
    {
        let mut m1006 = m1006;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1006;
                let m968 = v2;
                if m968 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f213(f81(Str::lit("the input ends inside a text run"), v1)?)))
                } else {
                    let v6 = m968 - 1;
                    let v7: u64 = f85(v0.clone(), v1)?;
                    if if v4 {
                        let a969 = v7;
                        let a970 = v5;
                        nat_eq(a969, a970)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, {
                            let a971 = v1;
                            let a972 = 1u64;
                            nat_add(a971, a972)?
                        }))))
                    } else {
                        let a973 = v7;
                        let a974 = 256u64;
                        if nat_eq(a973, a974) {
                            if v4 {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f213(f81(Str::lit("an attribute value is not closed (XML 1.0 [10])"), v1)?)))
                            } else {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                            }
                        } else {
                            let a975 = v7;
                            let a976 = 60u64;
                            if nat_eq(a975, a976) {
                                if v4 {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f213(f81(Str::lit("< is not allowed in an attribute value (XML 1.0 [10], WFC: No < in Attribute Values)"), v1)?)))
                                } else {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                                }
                            } else {
                                let a977 = v7;
                                let a978 = 38u64;
                                if nat_eq(a977, a978) {
                                    let m979 = f214(v0.clone(), v1)?;
                                    match m979 {
                                        Ok(v9) => {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let (_, h980) = v9;
                                                h980
                                            }, (v6, {
                                                let a982 = v3;
                                                let a983 = {
                                                    let (h981, _) = v9;
                                                    h981
                                                };
                                                append_bytes(a982, a983)
                                            })))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f213(v8)))
                                        }
                                    }
                                } else {
                                    if v4 {
                                        let a984 = v7;
                                        let a985 = 9u64;
                                        if if nat_eq(a984, a985) {
                                            true
                                        } else {
                                            let a986 = v7;
                                            let a987 = 10u64;
                                            nat_eq(a986, a987)
                                        } {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let a988 = v1;
                                                let a989 = 1u64;
                                                nat_add(a988, a989)?
                                            }, (v6, {
                                                let a990 = v3;
                                                let a991 = f8(Str::lit(" "));
                                                append_bytes(a990, a991)
                                            })))
                                        } else {
                                            let v10: u64 = f215(v0.clone(), true, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v10, (v6, {
                                                let a994 = v3;
                                                let a995 = f32(v0.clone(), v1, {
                                                    let a992 = v10;
                                                    let a993 = v1;
                                                    nat_sub(a992, a993)
                                                });
                                                append_bytes(a994, a995)
                                            })))
                                        }
                                    } else {
                                        let a996 = v7;
                                        let a997 = 93u64;
                                        if nat_eq(a996, a997) {
                                            if f84(v0.clone(), v1, Str::lit("]]>")) {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f213(f81(Str::lit("]]> is not allowed in character data (XML 1.0 [14])"), v1)?)))
                                            } else {
                                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                    let a998 = v1;
                                                    let a999 = 1u64;
                                                    nat_add(a998, a999)?
                                                }, (v6, {
                                                    let a1000 = v3;
                                                    let a1001 = f8(Str::lit("]"));
                                                    append_bytes(a1000, a1001)
                                                })))
                                            }
                                        } else {
                                            let v11: u64 = f215(v0.clone(), false, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v11, (v6, {
                                                let a1004 = v3;
                                                let a1005 = f32(v0.clone(), v1, {
                                                    let a1002 = v11;
                                                    let a1003 = v1;
                                                    nat_sub(a1002, a1003)
                                                });
                                                append_bytes(a1004, a1005)
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
                Ok(next) => { m1006 = next; }
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
    let m1014 = {
        let a1011 = v0;
        let a1012 = 0u64;
        let a1013 = {
            let a1010 = v1.clone();
            length_bytes(a1010)
        };
        slice_bytes(a1011, a1012, a1013)
    };
    match m1014 {
        None => {
            false
        }
        Some(v2) => {
            let a1015 = v2;
            let a1016 = v1;
            equal(a1015, a1016)
        }
    }
}

pub fn f146(v0: Bytes) -> R<Bytes> {
    let m1017 = f131(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1017 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v1) => {
            Ok(f32(v0, 0u64, v1))
        }
    }
}

pub fn f147(p1022: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1021 = p1022;
    {
        let mut m1021 = m1021;
        loop {
            let step = {
                let v0 = m1021;
                let m1018 = v0;
                match m1018.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h1019, _) = v2;
                            h1019
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h1020) = v2;
                                h1020
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1021 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f148(v0: Bytes) -> R<Bytes> {
    let m1023 = f131(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1023 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(f32(v0.clone(), {
                let a1024 = v1;
                let a1025 = 1u64;
                nat_add(a1024, a1025)?
            }, {
                let a1029 = {
                    let a1026 = v0;
                    length_bytes(a1026)
                };
                let a1030 = {
                    let a1027 = v1;
                    let a1028 = 1u64;
                    nat_add(a1027, a1028)?
                };
                nat_sub(a1029, a1030)
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

pub fn f151(v0: Bytes, v1: Adt16, v2: List<(Bytes, Bytes)>, v3: Bytes) -> R<Result<Adt19, Bytes>> {
    let v4: Bytes = f152(v0.clone())?;
    let m1031 = f216(v1, v2)?;
    match m1031 {
        Ok(v6) => {
            let m1032 = v6;
            match m1032 {
                None => {
                    Ok(f217(f218(f8(Str::lit("a simpleType must have a restriction, list or union child (XSD 1.1 Part 2, section 4.1.2)")), v0)))
                }
                Some(v7) => {
                    let v8: Bytes = match v7.clone() {
                        Adt20::C0(h1033, _, _, _) => {
                            h1033
                        }
                    };
                    let v9: Adt16 = match v7.clone() {
                        Adt20::C0(_, h1034, _, _) => {
                            h1034
                        }
                    };
                    let v10: Adt16 = match v7.clone() {
                        Adt20::C0(_, _, h1035, _) => {
                            h1035
                        }
                    };
                    let v11: List<(Bytes, Bytes)> = match v7 {
                        Adt20::C0(_, _, _, h1036) => {
                            h1036
                        }
                    };
                    if f35(v8.clone(), f8(Str::lit("union"))) {
                        Ok(f217(f218(f8(Str::lit("a union is not translated, because a literal can be valid for more than one member type, so no LexLean inductive keeps its members disjoint (XSD 1.1 Part 2, section 2.4.1.3)")), v0)))
                    } else {
                        if f35(v8, f8(Str::lit("list"))) {
                            let m1037 = f91(v9, f8(Str::lit("itemType")));
                            match m1037 {
                                None => {
                                    Ok(f217(f218(f8(Str::lit("a list with an anonymous item type is not translated yet")), v0)))
                                }
                                Some(v12) => {
                                    let m1038 = f219(v11, v3, v12)?;
                                    match m1038 {
                                        Ok(v14) => {
                                            if f145(match v14.clone() {
                                                Adt19::C0(h1039, _) => {
                                                    h1039
                                                }
                                            }, f8(Str::lit("(List "))) {
                                                Ok(f217(f218(f8(Str::lit("the item type of a list must not itself be a list (XSD 1.1 Part 2, section 4.1.5)")), v0)))
                                            } else {
                                                Ok(Ok::<Adt19, Bytes>(Adt19::C0(f204(v4, f7(List::cons(f8(Str::lit("(List ")), List::cons(match v14.clone() {
                                                    Adt19::C0(h1040, _) => {
                                                        h1040
                                                    }
                                                }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))), match v14 {
                                                    Adt19::C0(_, h1041) => {
                                                        h1041
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
                            let m1042 = f91(v9, f8(Str::lit("base")));
                            match m1042 {
                                None => {
                                    Ok(f217(f218(f8(Str::lit("a restriction with an anonymous base type is not translated yet")), v0)))
                                }
                                Some(v15) => {
                                    let m1043 = f219(v11.clone(), v3, v15)?;
                                    match m1043 {
                                        Ok(v17) => {
                                            let m1044 = f220(v10, v11)?;
                                            match m1044 {
                                                Ok(v19) => {
                                                    let v20: List<Adt17> = f221(v19, List::<Bytes>::nil())?;
                                                    let m1045 = v20.clone();
                                                    match m1045.uncons() {
                                                        None => {
                                                            Ok(Ok::<Adt19, Bytes>(Adt19::C0(f204(v4, match v17.clone() {
                                                                Adt19::C0(h1046, _) => {
                                                                    h1046
                                                                }
                                                            }), match v17 {
                                                                Adt19::C0(_, h1047) => {
                                                                    h1047
                                                                }
                                                            })))
                                                        }
                                                        Some((_, _)) => {
                                                            let m1048 = f196(f191(f222(v20.clone()))?, v20, List::<Bytes>::nil())?;
                                                            match m1048 {
                                                                Ok(v24) => {
                                                                    Ok(Ok::<Adt19, Bytes>(Adt19::C0(f198(v4, v24), List::<Bytes>::nil())))
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
        let a1049 = v0;
        length_bytes(a1049)
    }, true, Bytes::lit(&[]))
}

pub fn f153(p1052: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m1051 = p1052;
    {
        let mut m1051 = m1051;
        loop {
            let step = {
                let v0 = m1051;
                let m1050 = v0;
                match m1050.uncons() {
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
                Ok(next) => { m1051 = next; }
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
    let a1053 = v0;
    let a1054 = 32u64;
    if nat_eq(a1053, a1054) {
        true
    } else {
        let a1055 = v0;
        let a1056 = 9u64;
        nat_eq(a1055, a1056)
    }
}

pub fn f158(v0: u64) -> bool {
    let a1057 = v0;
    let a1058 = 10u64;
    if nat_eq(a1057, a1058) {
        true
    } else {
        let a1059 = v0;
        let a1060 = 256u64;
        nat_eq(a1059, a1060)
    }
}

pub fn f159(v0: Bytes, v1: u64) -> R<u64> {
    let a1061 = v1;
    let a1062 = f226(v0, v1)?;
    Ok(nat_sub(a1061, a1062))
}

pub fn f160(v0: Bytes, v1: u64) -> R<u64> {
    let (h1063, _) = f128(Fn4::F227(v0.clone()), {
        let a1065 = {
            let a1064 = v0;
            length_bytes(a1064)
        };
        let a1066 = 1u64;
        nat_add(a1065, a1066)?
    }, v1)?;
    Ok(h1063)
}

pub fn f161(v0: Bytes, v1: u64) -> R<u64> {
    let (h1067, _) = f128(Fn4::F228(v0.clone()), {
        let a1069 = {
            let a1068 = v0;
            length_bytes(a1068)
        };
        let a1070 = 1u64;
        nat_add(a1069, a1070)?
    }, v1)?;
    Ok(h1067)
}

pub fn f162(v0: Bytes, v1: u64) -> R<bool> {
    let a1071 = f100(v0.clone(), v1)?;
    let a1072 = 45u64;
    if nat_eq(a1071, a1072) {
        if f157(f100(v0.clone(), {
            let a1073 = v1;
            let a1074 = 1u64;
            nat_add(a1073, a1074)?
        })?) {
            Ok(true)
        } else {
            Ok(f158(f100(v0, {
                let a1075 = v1;
                let a1076 = 1u64;
                nat_add(a1075, a1076)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f163(v0: Bytes, v1: u64) -> R<bool> {
    let m1077 = f170(v0, v1)?;
    match m1077 {
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
        let a1078 = v1;
        let a1079 = 1u64;
        nat_add(a1078, a1079)?
    })?, true));
    let a1080 = v2;
    let a1081 = 58u64;
    if if nat_eq(a1080, a1081) {
        v3
    } else {
        false
    } {
        Ok(Some(f81(Str::lit("a null key is not read: Studio.Value has no null (YAML 1.2.2 section 10.3.2)"), v1)?))
    } else {
        let a1082 = v2;
        let a1083 = 38u64;
        if nat_eq(a1082, a1083) {
            Ok(Some(f81(Str::lit("an anchor is not read (YAML 1.2.2 [101])"), v1)?))
        } else {
            let a1084 = v2;
            let a1085 = 42u64;
            if nat_eq(a1084, a1085) {
                Ok(Some(f81(Str::lit("an alias is not read (YAML 1.2.2 [104])"), v1)?))
            } else {
                let a1086 = v2;
                let a1087 = 33u64;
                if nat_eq(a1086, a1087) {
                    Ok(Some(f81(Str::lit("a tag is not read (YAML 1.2.2 [97])"), v1)?))
                } else {
                    let a1088 = v2;
                    let a1089 = 63u64;
                    if if nat_eq(a1088, a1089) {
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

pub fn f165(v0: Bytes, v1: u64, v2: u64) -> R<Result<(Adt17, u64), Bytes>> {
    let v3: u64 = f100(v0.clone(), v1)?;
    let a1090 = v3;
    let a1091 = 124u64;
    if if nat_eq(a1090, a1091) {
        true
    } else {
        let a1092 = v3;
        let a1093 = 62u64;
        nat_eq(a1092, a1093)
    } {
        f230(v0, v1, v2)
    } else {
        let a1094 = v3;
        let a1095 = 34u64;
        if if nat_eq(a1094, a1095) {
            true
        } else {
            let a1096 = v3;
            let a1097 = 39u64;
            nat_eq(a1096, a1097)
        } {
            let m1098 = f231(v0.clone(), v1, v2, false)?;
            match m1098 {
                Ok(v5) => {
                    let m1100 = f173(v0, {
                        let (_, h1099) = v5;
                        h1099
                    })?;
                    match m1100 {
                        Ok(v7) => {
                            Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C3({
                                let (h1101, _) = v5;
                                h1101
                            }), v7)))
                        }
                        Err(v6) => {
                            Ok(Err::<(Adt17, u64), Bytes>(v6))
                        }
                    }
                }
                Err(v4) => {
                    Ok(Err::<(Adt17, u64), Bytes>(v4))
                }
            }
        } else {
            let a1102 = v3;
            let a1103 = 91u64;
            if if nat_eq(a1102, a1103) {
                true
            } else {
                let a1104 = v3;
                let a1105 = 123u64;
                nat_eq(a1104, a1105)
            } {
                let m1109 = f232(v0.clone(), v1, v2, {
                    let a1107 = {
                        let a1106 = v0.clone();
                        length_bytes(a1106)
                    };
                    let a1108 = 2u64;
                    nat_add(a1107, a1108)?
                }, 0u64, List::<Bytes>::nil())?;
                match m1109 {
                    Ok(v9) => {
                        let a1111 = f100(v0.clone(), f161(v0.clone(), {
                            let (_, h1110) = v9.clone();
                            h1110
                        })?)?;
                        let a1112 = 58u64;
                        if nat_eq(a1111, a1112) {
                            Ok(Err::<(Adt17, u64), Bytes>(f81(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [154])"), v1)?))
                        } else {
                            let m1114 = f173(v0, {
                                let (_, h1113) = v9;
                                h1113
                            })?;
                            match m1114 {
                                Ok(v11) => {
                                    Ok(Ok::<(Adt17, u64), Bytes>(({
                                        let (h1115, _) = v9;
                                        h1115
                                    }, v11)))
                                }
                                Err(v10) => {
                                    Ok(Err::<(Adt17, u64), Bytes>(v10))
                                }
                            }
                        }
                    }
                    Err(v8) => {
                        Ok(Err::<(Adt17, u64), Bytes>(v8))
                    }
                }
            } else {
                let m1116 = f164(v0.clone(), v1)?;
                match m1116 {
                    None => {
                        if f169(f233(v0.clone(), v1, false)?) {
                            Ok(Err::<(Adt17, u64), Bytes>(f81(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                        } else {
                            let v13: u64 = f234(v0.clone(), v1, false)?;
                            let m1120 = f235(v0.clone(), v13, v2, {
                                let a1118 = {
                                    let a1117 = v0.clone();
                                    length_bytes(a1117)
                                };
                                let a1119 = 2u64;
                                nat_add(a1118, a1119)?
                            }, f236(v0, v1, v13))?;
                            match m1120 {
                                Ok(v15) => {
                                    let m1122 = f237({
                                        let (h1121, _) = v15;
                                        h1121
                                    }, v1)?;
                                    match m1122 {
                                        Ok(v17) => {
                                            Ok(Ok::<(Adt17, u64), Bytes>((v17, {
                                                let (_, h1123) = v15;
                                                h1123
                                            })))
                                        }
                                        Err(v16) => {
                                            Ok(Err::<(Adt17, u64), Bytes>(v16))
                                        }
                                    }
                                }
                                Err(v14) => {
                                    Ok(Err::<(Adt17, u64), Bytes>(v14))
                                }
                            }
                        }
                    }
                    Some(v12) => {
                        Ok(Err::<(Adt17, u64), Bytes>(v12))
                    }
                }
            }
        }
    }
}

pub fn f166(v0: bool, v1: Result<(Adt17, u64), Bytes>) -> Result<(Adt17, u64), Bytes> {
    let m1124 = v1;
    match m1124 {
        Ok(v3) => {
            Ok::<(Adt17, u64), Bytes>((if v0 {
                Adt17::C4(std::rc::Rc::new({
                    let (h1125, _) = v3;
                    h1125
                }))
            } else {
                Adt17::C5(std::rc::Rc::new({
                    let (h1126, _) = v3;
                    h1126
                }))
            }, {
                let (_, h1127) = v3;
                h1127
            }))
        }
        Err(v2) => {
            Err::<(Adt17, u64), Bytes>(v2)
        }
    }
}

pub fn f167(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f238(v0.clone(), v1)?;
    let a1129 = v2;
    let a1130 = {
        let a1128 = v0;
        length_bytes(a1128)
    };
    if nat_lt(a1129, a1130) {
        let a1131 = v2;
        let a1132 = 1u64;
        nat_add(a1131, a1132)
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
    let a1133 = v2;
    let a1134 = 34u64;
    if if nat_eq(a1133, a1134) {
        true
    } else {
        let a1135 = v2;
        let a1136 = 39u64;
        nat_eq(a1135, a1136)
    } {
        let m1137 = f231(v0.clone(), v1, 0u64, true)?;
        match m1137 {
            Ok(v4) => {
                let v5: u64 = f161(v0.clone(), {
                    let (_, h1138) = v4;
                    h1138
                })?;
                if f239(v0, v5)? {
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
        if f233(v0.clone(), v1, false)? {
            let v6: u64 = f161(v0.clone(), f234(v0.clone(), v1, false)?)?;
            if f239(v0, v6)? {
                Ok(Some(v6))
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
    let a1139 = v2;
    let a1140 = 34u64;
    if if nat_eq(a1139, a1140) {
        true
    } else {
        let a1141 = v2;
        let a1142 = 39u64;
        nat_eq(a1141, a1142)
    } {
        let m1143 = f231(v0, v1, 0u64, true)?;
        match m1143 {
            Ok(v4) => {
                let (h1144, _) = v4;
                Ok(h1144)
            }
            Err(_) => {
                Ok(Bytes::lit(&[]))
            }
        }
    } else {
        Ok(f236(v0.clone(), v1, f234(v0, v1, false)?))
    }
}

pub fn f172(v0: Bytes, p1147: List<Bytes>) -> bool {
    let m1146 = p1147;
    {
        let mut m1146 = m1146;
        loop {
            let step = {
                let v1 = m1146;
                let m1145 = v1;
                match m1145.uncons() {
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
                Ok(next) => { m1146 = next; }
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
        let a1148 = v3;
        let a1149 = 35u64;
        if nat_eq(a1148, a1149) {
            let a1150 = v1;
            let a1151 = v2;
            nat_lt(a1150, a1151)
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
        let a1152 = 0u64;
        let a1153 = v1;
        nat_lt(a1152, a1153)
    } else {
        false
    } {
        let a1154 = Bytes::lit(&[0x2d]);
        let a1155 = f51(v1)?;
        Ok(append_bytes(a1154, a1155))
    } else {
        f51(v1)
    }
}

pub fn f175(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f51(v1)?;
    let v4: u64 = {
        let a1156 = v3.clone();
        length_bytes(a1156)
    };
    let v5: Bytes = {
        let a1157 = v2;
        let a1158 = v4;
        if nat_lt(a1157, a1158) {
            f7(List::cons(f32(v3.clone(), 0u64, {
                let a1159 = v4;
                let a1160 = v2;
                nat_sub(a1159, a1160)
            }), List::cons(f8(Str::lit(".")), List::cons(f32(v3, {
                let a1161 = v4;
                let a1162 = v2;
                nat_sub(a1161, a1162)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f240({
                let a1163 = v2;
                let a1164 = v4;
                nat_sub(a1163, a1164)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1165 = f8(Str::lit("-"));
        let a1166 = v5;
        Ok(append_bytes(a1165, a1166))
    } else {
        Ok(v5)
    }
}

pub fn f176(v0: Bytes) -> R<Bytes> {
    let a1171 = f241(v0.clone(), 0u64, {
        let a1168 = {
            let a1167 = v0.clone();
            length_bytes(a1167)
        };
        let a1169 = 1u64;
        nat_add(a1168, a1169)?
    })?;
    let a1172 = {
        let a1170 = v0.clone();
        length_bytes(a1170)
    };
    if nat_eq(a1171, a1172) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        f242(v0)
    }
}

pub fn f177(v0: List<Bytes>) -> Bytes {
    let a1175 = {
        let a1173 = Bytes::lit(&[0x5b]);
        let a1174 = f33(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1173, a1174)
    };
    let a1176 = Bytes::lit(&[0x5d]);
    append_bytes(a1175, a1176)
}

pub fn f178(v0: Adt17) -> R<List<Bytes>> {
    let m1177 = v0;
    match m1177 {
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
        Adt17::C7(r1178, r1179) => {
            let v10 = (*r1178).clone();
            let v11 = (*r1179).clone();
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
    let a1182 = {
        let a1180 = Bytes::lit(&[0x7b]);
        let a1181 = f33(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1180, a1181)
    };
    let a1183 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1182, a1183))
}

pub fn f180(v0: Adt17) -> R<List<(Bytes, Bytes)>> {
    let m1184 = v0;
    match m1184 {
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
        Adt17::C8(v12, r1185, r1186) => {
            let v13 = (*r1185).clone();
            let v14 = (*r1186).clone();
            Ok(List::cons((v12, f103(v13)?), f180(v14)?))
        }
    }
}

pub fn f181(p1241: u64, v1: Bytes, p1242: u64, p1243: List<Adt18>) -> R<Result<List<Adt18>, Bytes>> {
    let m1240 = (p1241, (p1242, p1243));
    {
        let mut m1240 = m1240;
        loop {
            let step = {
                let (v0, (v2, v3)) = m1240;
                let m1187 = v0;
                if m1187 == 0 {
                    Err::<(u64, (u64, List<Adt18>)), R<Result<List<Adt18>, Bytes>>>(Ok(Err::<List<Adt18>, Bytes>(f248(Str::lit("the input has more tokens than bytes"), v2)?)))
                } else {
                    let v4 = m1187 - 1;
                    let v5: u64 = f85(v1.clone(), v2)?;
                    let a1188 = v5;
                    let a1189 = 256u64;
                    if nat_eq(a1188, a1189) {
                        Err::<(u64, (u64, List<Adt18>)), R<Result<List<Adt18>, Bytes>>>(Ok(Ok::<List<Adt18>, Bytes>(f249(v3, List::<Adt18>::nil()))))
                    } else {
                        if f130(v5)? {
                            Ok::<(u64, (u64, List<Adt18>)), R<Result<List<Adt18>, Bytes>>>((v4, ({
                                let a1190 = v2;
                                let a1191 = 1u64;
                                nat_add(a1190, a1191)?
                            }, v3)))
                        } else {
                            if f84(v1.clone(), v2, Str::lit("//")) {
                                let m1192 = f131(v1.clone(), Bytes::lit(&[0x0a]), v2)?;
                                match m1192 {
                                    None => {
                                        Err::<(u64, (u64, List<Adt18>)), R<Result<List<Adt18>, Bytes>>>(Ok(Ok::<List<Adt18>, Bytes>(f249(v3, List::<Adt18>::nil()))))
                                    }
                                    Some(v6) => {
                                        Ok::<(u64, (u64, List<Adt18>)), R<Result<List<Adt18>, Bytes>>>((v4, ({
                                            let a1193 = v6;
                                            let a1194 = 1u64;
                                            nat_add(a1193, a1194)?
                                        }, v3)))
                                    }
                                }
                            } else {
                                if f84(v1.clone(), v2, Str::lit("/*")) {
                                    let m1198 = f131(v1.clone(), {
                                        let a1195 = Str::lit("*/");
                                        utf8_encode(a1195)
                                    }, {
                                        let a1196 = v2;
                                        let a1197 = 2u64;
                                        nat_add(a1196, a1197)?
                                    })?;
                                    match m1198 {
                                        None => {
                                            Err::<(u64, (u64, List<Adt18>)), R<Result<List<Adt18>, Bytes>>>(Ok(Err::<List<Adt18>, Bytes>(f248(Str::lit("a comment is not closed (Web IDL, IDL grammar: comment)"), v2)?)))
                                        }
                                        Some(v7) => {
                                            Ok::<(u64, (u64, List<Adt18>)), R<Result<List<Adt18>, Bytes>>>((v4, ({
                                                let a1199 = v7;
                                                let a1200 = 2u64;
                                                nat_add(a1199, a1200)?
                                            }, v3)))
                                        }
                                    }
                                } else {
                                    let a1201 = v5;
                                    let a1202 = 34u64;
                                    if nat_eq(a1201, a1202) {
                                        let m1205 = f131(v1.clone(), Bytes::lit(&[0x22]), {
                                            let a1203 = v2;
                                            let a1204 = 1u64;
                                            nat_add(a1203, a1204)?
                                        })?;
                                        match m1205 {
                                            None => {
                                                Err::<(u64, (u64, List<Adt18>)), R<Result<List<Adt18>, Bytes>>>(Ok(Err::<List<Adt18>, Bytes>(f248(Str::lit("a string is not closed (Web IDL, IDL grammar: string)"), v2)?)))
                                            }
                                            Some(v8) => {
                                                Ok::<(u64, (u64, List<Adt18>)), R<Result<List<Adt18>, Bytes>>>((v4, ({
                                                    let a1206 = v8;
                                                    let a1207 = 1u64;
                                                    nat_add(a1206, a1207)?
                                                }, List::cons(Adt18::C0(3u64, f32(v1.clone(), {
                                                    let a1208 = v2;
                                                    let a1209 = 1u64;
                                                    nat_add(a1208, a1209)?
                                                }, {
                                                    let a1212 = v8;
                                                    let a1213 = {
                                                        let a1210 = v2;
                                                        let a1211 = 1u64;
                                                        nat_add(a1210, a1211)?
                                                    };
                                                    nat_sub(a1212, a1213)
                                                })), v3))))
                                            }
                                        }
                                    } else {
                                        let v9: u64 = {
                                            let a1214 = v5;
                                            let a1215 = 95u64;
                                            if if nat_eq(a1214, a1215) {
                                                true
                                            } else {
                                                let a1216 = v5;
                                                let a1217 = 45u64;
                                                nat_eq(a1216, a1217)
                                            } {
                                                let a1218 = v2;
                                                let a1219 = 1u64;
                                                nat_add(a1218, a1219)?
                                            } else {
                                                v2
                                            }
                                        };
                                        if f250(f85(v1.clone(), v9)?) {
                                            let v10: u64 = f251(v1.clone(), 3u64, {
                                                let a1220 = v9;
                                                let a1221 = 1u64;
                                                nat_add(a1220, a1221)?
                                            })?;
                                            Ok::<(u64, (u64, List<Adt18>)), R<Result<List<Adt18>, Bytes>>>((v4, (v10, List::cons(Adt18::C0(0u64, f32(v1.clone(), v2, {
                                                let a1222 = v10;
                                                let a1223 = v2;
                                                nat_sub(a1222, a1223)
                                            })), v3))))
                                        } else {
                                            let v11: u64 = {
                                                let a1224 = v5;
                                                let a1225 = 45u64;
                                                if nat_eq(a1224, a1225) {
                                                    let a1226 = v2;
                                                    let a1227 = 1u64;
                                                    nat_add(a1226, a1227)?
                                                } else {
                                                    v2
                                                }
                                            };
                                            let v12: u64 = f252(v1.clone(), v11)?;
                                            let v13: u64 = f253(v1.clone(), v11)?;
                                            let a1228 = v12;
                                            let a1229 = v13;
                                            if nat_lt(a1228, a1229) {
                                                Ok::<(u64, (u64, List<Adt18>)), R<Result<List<Adt18>, Bytes>>>((v4, (v13, List::cons(Adt18::C0(2u64, f32(v1.clone(), v2, {
                                                    let a1230 = v13;
                                                    let a1231 = v2;
                                                    nat_sub(a1230, a1231)
                                                })), v3))))
                                            } else {
                                                let a1232 = v11;
                                                let a1233 = v12;
                                                if nat_lt(a1232, a1233) {
                                                    Ok::<(u64, (u64, List<Adt18>)), R<Result<List<Adt18>, Bytes>>>((v4, (v12, List::cons(Adt18::C0(1u64, f32(v1.clone(), v2, {
                                                        let a1234 = v12;
                                                        let a1235 = v2;
                                                        nat_sub(a1234, a1235)
                                                    })), v3))))
                                                } else {
                                                    if f84(v1.clone(), v2, Str::lit("...")) {
                                                        Ok::<(u64, (u64, List<Adt18>)), R<Result<List<Adt18>, Bytes>>>((v4, ({
                                                            let a1236 = v2;
                                                            let a1237 = 3u64;
                                                            nat_add(a1236, a1237)?
                                                        }, List::cons(Adt18::C0(4u64, f8(Str::lit("..."))), v3))))
                                                    } else {
                                                        Ok::<(u64, (u64, List<Adt18>)), R<Result<List<Adt18>, Bytes>>>((v4, ({
                                                            let a1238 = v2;
                                                            let a1239 = 1u64;
                                                            nat_add(a1238, a1239)?
                                                        }, List::cons(Adt18::C0(4u64, f32(v1.clone(), v2, 1u64)), v3))))
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
                Ok(next) => { m1240 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f182(p1246: List<Adt12>, p1247: List<Adt12>) -> List<Adt12> {
    let m1245 = (p1246, p1247);
    {
        let mut m1245 = m1245;
        loop {
            let step = {
                let (v0, v1) = m1245;
                let m1244 = v0;
                match m1244.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>((v3, List::cons(v2, v1)))
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

pub fn f183(v0: List<Adt18>) -> R<Result<List<Adt18>, Bytes>> {
    if f254(v0.clone(), Str::lit("[")) {
        Ok(f255({
            let a1249 = {
                let a1248 = v0.clone();
                length_list(a1248)
            };
            let a1250 = 1u64;
            nat_add(a1249, a1250)?
        }, f256(v0), List::cons(f8(Str::lit("]")), List::<Bytes>::nil()), false))
    } else {
        Ok(Ok::<List<Adt18>, Bytes>(v0))
    }
}

pub fn f184(v0: u64, v1: List<Adt18>) -> R<Result<(Adt12, List<Adt18>), Bytes>> {
    if f254(v1.clone(), Str::lit("interface")) {
        if f254(f256(v1.clone()), Str::lit("mixin")) {
            f257(v0, 1u64, f256(f256(v1)), false)
        } else {
            f257(v0, 0u64, f256(v1), true)
        }
    } else {
        if f254(v1.clone(), Str::lit("partial")) {
            let v2: List<Adt18> = f256(v1);
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
                        Ok(Err::<(Adt12, List<Adt18>), Bytes>(f259(Str::lit("interface, dictionary or namespace"), v2)))
                    }
                }
            }
        } else {
            if f254(v1.clone(), Str::lit("dictionary")) {
                f257(v0, 2u64, f256(v1), true)
            } else {
                if f254(v1.clone(), Str::lit("enum")) {
                    let m1251 = f260(f256(v1));
                    match m1251 {
                        Ok(v4) => {
                            let m1252 = f261(f262(v4.clone()), Str::lit("{"));
                            match m1252 {
                                Ok(v6) => {
                                    let m1253 = f263(v0, v6, List::<Bytes>::nil());
                                    match m1253 {
                                        Ok(v8) => {
                                            Ok(Ok::<(Adt12, List<Adt18>), Bytes>((Adt12::C0(3u64, f264(v4), Bytes::lit(&[]), List::<Adt13>::nil(), {
                                                let (h1254, _) = v8;
                                                h1254
                                            }, f265(Bytes::lit(&[]))), {
                                                let (_, h1255) = v8;
                                                h1255
                                            })))
                                        }
                                        Err(v7) => {
                                            Ok(Err::<(Adt12, List<Adt18>), Bytes>(v7))
                                        }
                                    }
                                }
                                Err(v5) => {
                                    Ok(Err::<(Adt12, List<Adt18>), Bytes>(v5))
                                }
                            }
                        }
                        Err(v3) => {
                            Ok(Err::<(Adt12, List<Adt18>), Bytes>(v3))
                        }
                    }
                } else {
                    if f254(v1.clone(), Str::lit("typedef")) {
                        let m1256 = f266(v0, f256(v1), 0u64)?;
                        match m1256 {
                            Ok(v10) => {
                                let m1257 = f260(f267(v10.clone()));
                                match m1257 {
                                    Ok(v12) => {
                                        let m1258 = f261(f262(v12.clone()), Str::lit(";"));
                                        match m1258 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt12, List<Adt18>), Bytes>((Adt12::C0(4u64, f264(v12), Bytes::lit(&[]), List::<Adt13>::nil(), List::<Bytes>::nil(), f268(v10)), v14)))
                                            }
                                            Err(v13) => {
                                                Ok(Err::<(Adt12, List<Adt18>), Bytes>(v13))
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(Err::<(Adt12, List<Adt18>), Bytes>(v11))
                                    }
                                }
                            }
                            Err(v9) => {
                                Ok(Err::<(Adt12, List<Adt18>), Bytes>(v9))
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
                                let m1259 = f260(v1);
                                match m1259 {
                                    Ok(v16) => {
                                        let m1260 = f261(f262(v16.clone()), Str::lit("includes"));
                                        match m1260 {
                                            Ok(v18) => {
                                                let m1261 = f260(v18);
                                                match m1261 {
                                                    Ok(v20) => {
                                                        let m1262 = f261(f262(v20.clone()), Str::lit(";"));
                                                        match m1262 {
                                                            Ok(v22) => {
                                                                Ok(Ok::<(Adt12, List<Adt18>), Bytes>((Adt12::C0(8u64, f264(v16), f264(v20), List::<Adt13>::nil(), List::<Bytes>::nil(), f265(Bytes::lit(&[]))), v22)))
                                                            }
                                                            Err(v21) => {
                                                                Ok(Err::<(Adt12, List<Adt18>), Bytes>(v21))
                                                            }
                                                        }
                                                    }
                                                    Err(v19) => {
                                                        Ok(Err::<(Adt12, List<Adt18>), Bytes>(v19))
                                                    }
                                                }
                                            }
                                            Err(v17) => {
                                                Ok(Err::<(Adt12, List<Adt18>), Bytes>(v17))
                                            }
                                        }
                                    }
                                    Err(v15) => {
                                        Ok(Err::<(Adt12, List<Adt18>), Bytes>(v15))
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

pub fn f185(p1266: List<Bytes>, v1: List<Adt15>) -> Bytes {
    let m1265 = p1266;
    {
        let mut m1265 = m1265;
        loop {
            let step = {
                let v0 = m1265;
                let m1263 = v0;
                match m1263.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1264 = f114(v1.clone(), v2.clone());
                        match m1264 {
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
                Ok(next) => { m1265 = next; }
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

pub fn f188(p1272: List<Adt12>, v1: List<Adt12>, v2: Bytes) -> Option<Bytes> {
    let m1271 = p1272;
    {
        let mut m1271 = m1271;
        loop {
            let step = {
                let v0 = m1271;
                let m1267 = v0;
                match m1267.uncons() {
                    None => {
                        Err::<List<Adt12>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v3, v4)) => {
                        let a1268 = f110(v3.clone());
                        let a1269 = 8u64;
                        if if nat_eq(a1268, a1269) {
                            f35(f112(v3.clone()), v2.clone())
                        } else {
                            false
                        } {
                            let m1270 = f121(v1.clone(), 1u64, f123(v3.clone()));
                            match m1270 {
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
                Ok(next) => { m1271 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f189(v0: u64, v1: List<Adt12>, v2: Adt12) -> R<Result<List<Adt13>, Bytes>> {
    let m1273 = v0;
    if m1273 == 0 {
        Ok(Err::<List<Adt13>, Bytes>(f7(List::cons(f8(Str::lit("the inheritance of ")), List::cons(f112(v2), List::cons(f8(Str::lit(" is cyclic (Web IDL 2.2 and 2.7: an interface or dictionary must not inherit from itself)")), List::<Bytes>::nil()))))))
    } else {
        let v3 = m1273 - 1;
        let v4: List<Adt13> = {
            let a1274 = f110(v2.clone());
            let a1275 = 2u64;
            if nat_eq(a1274, a1275) {
                f276(f277(f278(v2.clone()), f279(v1.clone(), 7u64, f112(v2.clone()))))?
            } else {
                f277(f278(v2.clone()), f277(f279(v1.clone(), 5u64, f112(v2.clone())), f280(v1.clone(), v1.clone(), f112(v2.clone()))))
            }
        };
        if f35(f123(v2.clone()), Bytes::lit(&[])) {
            Ok(Ok::<List<Adt13>, Bytes>(v4))
        } else {
            let m1276 = f121(v1.clone(), f110(v2.clone()), f123(v2.clone()));
            match m1276 {
                None => {
                    Ok(Err::<List<Adt13>, Bytes>(f7(List::cons(f8(Str::lit("inherits ")), List::cons(f123(v2.clone()), List::cons({
                        let a1277 = f110(v2);
                        let a1278 = 2u64;
                        if nat_eq(a1277, a1278) {
                            f8(Str::lit(", which no input defines as a dictionary (Web IDL 2.7)"))
                        } else {
                            f8(Str::lit(", which no input defines as an interface (Web IDL 2.2)"))
                        }
                    }, List::<Bytes>::nil()))))))
                }
                Some(v5) => {
                    let m1279 = f189(v3, v1, v5)?;
                    match m1279 {
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
    let m1280 = v1;
    match m1280.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::<Bytes>::nil(), List::<Bytes>::nil())))
        }
        Some((v2, v3)) => {
            let v4: Adt14 = f281(v2.clone());
            if !f35(f200(v4.clone()), Bytes::lit(&[])) {
                Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons(f282(v2), List::cons(f8(Str::lit(": ")), List::cons(f200(v4), List::<Bytes>::nil())))))))
            } else {
                let m1281 = f283(v0, f282(v2.clone()))?;
                match m1281 {
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
                                let m1282 = f190(v0, v3)?;
                                match m1282 {
                                    Ok(v7) => {
                                        Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v5, List::cons(f8(Str::lit(", type: ")), List::cons({
                                            let a1283 = f285(v2);
                                            let a1284 = 1u64;
                                            if nat_eq(a1283, a1284) {
                                                f7(List::cons(f8(Str::lit("(Option ")), List::cons(f202(v4.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                            } else {
                                                f202(v4.clone())
                                            }
                                        }, List::cons(f8(Str::lit("}")), List::<Bytes>::nil())))))), {
                                            let (h1285, _) = v7;
                                            h1285
                                        }), f116(f203(v4), {
                                            let (_, h1286) = v7;
                                            h1286
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

pub fn f191(p1290: List<Bytes>) -> R<bool> {
    let m1289 = p1290;
    {
        let mut m1289 = m1289;
        loop {
            let step = {
                let v0 = m1289;
                let m1287 = v0;
                match m1287.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m1288 = f286(v1)?;
                        match m1288 {
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
                Ok(next) => { m1289 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f192(v0: List<Adt13>) -> List<Bytes> {
    let m1291 = v0;
    match m1291.uncons() {
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
    let m1292 = v0;
    match m1292 {
        Adt12::C0(_, _, _, _, v5, _) => {
            v5
        }
    }
}

pub fn f195(v0: bool, p1296: List<Bytes>) -> R<Option<Bytes>> {
    let m1295 = p1296;
    {
        let mut m1295 = m1295;
        loop {
            let step = {
                let v1 = m1295;
                let m1293 = v1;
                match m1293.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m1294 = f283(v0, v2)?;
                        match m1294 {
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
                Ok(next) => { m1295 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f196(v0: bool, v1: List<Adt17>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1297 = v1;
    match m1297.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m1298 = f287(v0, v3.clone())?;
            match m1298 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f57(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f113(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m1299 = f196(v0, v4, List::cons(v5.clone(), v2))?;
                        match m1299 {
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
    let m1300 = v0;
    match m1300.uncons() {
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
    let m1301 = v0;
    match m1301 {
        Adt12::C0(_, _, _, _, _, v6) => {
            v6
        }
    }
}

pub fn f200(v0: Adt14) -> Bytes {
    let m1302 = v0;
    match m1302 {
        Adt14::C0(_, _, _, v4, _) => {
            v4
        }
    }
}

pub fn f201(v0: Adt14) -> u64 {
    let m1303 = v0;
    match m1303 {
        Adt14::C0(v1, _, _, _, _) => {
            v1
        }
    }
}

pub fn f202(v0: Adt14) -> Bytes {
    let m1304 = v0;
    match m1304 {
        Adt14::C0(_, v2, _, _, _) => {
            v2
        }
    }
}

pub fn f203(v0: Adt14) -> List<Bytes> {
    let m1305 = v0;
    match m1305 {
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
        let (h1306, _) = v2;
        h1306
    };
    let a1307 = v3;
    let a1308 = 0u64;
    if nat_eq(a1307, a1308) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a1309 = v3;
            let a1310 = v1;
            let a1311 = 0u64;
            nat_quot(a1309, a1310, a1311)
        }, {
            let a1316 = f124(v0, {
                let a1312 = v3;
                let a1313 = v1;
                let a1314 = 0u64;
                nat_rem(a1312, a1313, a1314)
            });
            let a1317 = {
                let (_, h1315) = v2;
                h1315
            };
            append_bytes(a1316, a1317)
        }))
    }
}

pub fn f206(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h1318, _) = v1;
        h1318
    };
    let a1320 = {
        let a1319 = v0.clone();
        length_bytes(a1319)
    };
    let a1321 = v2;
    if nat_le(a1320, a1321) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let a1322 = f85(v0.clone(), v2)?;
        let a1323 = 13u64;
        if nat_eq(a1322, a1323) {
            Ok(Some(({
                let a1326 = f85(v0, {
                    let a1324 = v2;
                    let a1325 = 1u64;
                    nat_add(a1324, a1325)?
                })?;
                let a1327 = 10u64;
                if nat_eq(a1326, a1327) {
                    let a1328 = v2;
                    let a1329 = 2u64;
                    nat_add(a1328, a1329)?
                } else {
                    let a1330 = v2;
                    let a1331 = 1u64;
                    nat_add(a1330, a1331)?
                }
            }, {
                let a1333 = {
                    let (_, h1332) = v1;
                    h1332
                };
                let a1334 = f8(Str::lit("\u{a}"));
                append_bytes(a1333, a1334)
            })))
        } else {
            let v4: u64 = {
                let m1335 = f131(v0.clone(), f8(Str::lit("\u{d}")), v2)?;
                match m1335 {
                    None => {
                        let a1336 = v0.clone();
                        length_bytes(a1336)
                    }
                    Some(v3) => {
                        v3
                    }
                }
            };
            Ok(Some((v4, {
                let a1340 = {
                    let (_, h1337) = v1;
                    h1337
                };
                let a1341 = f32(v0, v2, {
                    let a1338 = v4;
                    let a1339 = v2;
                    nat_sub(a1338, a1339)
                });
                append_bytes(a1340, a1341)
            })))
        }
    }
}

pub fn f207(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1342 = f288(v0, v1)?;
    match m1342 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f289({
                let (h1343, _) = v2;
                h1343
            })? {
                Ok(Some({
                    let a1345 = v1;
                    let a1346 = {
                        let (_, h1344) = v2;
                        h1344
                    };
                    nat_add(a1345, a1346)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f208(v0: u64, v1: Bytes) -> R<bool> {
    f290(v0, v1.clone(), 0u64, {
        let a1348 = {
            let a1347 = v1;
            length_bytes(a1347)
        };
        let a1349 = 3u64;
        let a1350 = 0u64;
        nat_quot(a1348, a1349, a1350)
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
            let a1353 = f85(v0.clone(), {
                let a1351 = v1;
                let a1352 = 2u64;
                nat_add(a1351, a1352)?
            })?;
            let a1354 = 120u64;
            nat_eq(a1353, a1354)
        };
        let v3: u64 = if v2 {
            let a1355 = v1;
            let a1356 = 3u64;
            nat_add(a1355, a1356)?
        } else {
            let a1357 = v1;
            let a1358 = 2u64;
            nat_add(a1357, a1358)?
        };
        let v4: (u64, u64) = f294(v0.clone(), v2, v3)?;
        let a1360 = {
            let (h1359, _) = v4;
            h1359
        };
        let a1361 = v3;
        if if nat_eq(a1360, a1361) {
            true
        } else {
            let a1363 = f85(v0, {
                let (h1362, _) = v4;
                h1362
            })?;
            let a1364 = 59u64;
            !nat_eq(a1363, a1364)
        } {
            Ok(f213(f81(Str::lit("a character reference must be &#digits; or &#xhex; (XML 1.0 [66])"), v1)?))
        } else {
            if f289({
                let (_, h1365) = v4;
                h1365
            })? {
                Ok(Ok::<(Bytes, u64), Bytes>((f295({
                    let (_, h1366) = v4;
                    h1366
                })?, {
                    let a1368 = {
                        let (h1367, _) = v4;
                        h1367
                    };
                    let a1369 = 1u64;
                    nat_add(a1368, a1369)?
                })))
            } else {
                Ok(f213(f81(Str::lit("a character reference must name a Char (XML 1.0 [66] WFC: Legal Character)"), v1)?))
            }
        }
    } else {
        if f84(v0.clone(), v1, Str::lit("&lt;")) {
            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("<")), {
                let a1370 = v1;
                let a1371 = 4u64;
                nat_add(a1370, a1371)?
            })))
        } else {
            if f84(v0.clone(), v1, Str::lit("&gt;")) {
                Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit(">")), {
                    let a1372 = v1;
                    let a1373 = 4u64;
                    nat_add(a1372, a1373)?
                })))
            } else {
                if f84(v0.clone(), v1, Str::lit("&amp;")) {
                    Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("&")), {
                        let a1374 = v1;
                        let a1375 = 5u64;
                        nat_add(a1374, a1375)?
                    })))
                } else {
                    if f84(v0.clone(), v1, Str::lit("&apos;")) {
                        Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("'")), {
                            let a1376 = v1;
                            let a1377 = 6u64;
                            nat_add(a1376, a1377)?
                        })))
                    } else {
                        if f84(v0, v1, Str::lit("&quot;")) {
                            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("\"")), {
                                let a1378 = v1;
                                let a1379 = 6u64;
                                nat_add(a1378, a1379)?
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
    let (h1380, _) = f128(Fn4::F296(v1, v2, v0.clone()), {
        let a1382 = {
            let a1381 = v0;
            length_bytes(a1381)
        };
        let a1383 = 1u64;
        nat_add(a1382, a1383)?
    }, v3)?;
    Ok(h1380)
}

pub fn f216(p1396: Adt16, v1: List<(Bytes, Bytes)>) -> R<Result<Option<Adt20>, Bytes>> {
    let m1395 = p1396;
    {
        let mut m1395 = m1395;
        loop {
            let step = {
                let v0 = m1395;
                let m1384 = v0;
                match m1384 {
                    Adt16::C0(_, _, _) => {
                        Err::<Adt16, R<Result<Option<Adt20>, Bytes>>>(Ok(Ok::<Option<Adt20>, Bytes>(None::<Adt20>)))
                    }
                    Adt16::C1(_, _, _) => {
                        Err::<Adt16, R<Result<Option<Adt20>, Bytes>>>(Ok(Ok::<Option<Adt20>, Bytes>(None::<Adt20>)))
                    }
                    Adt16::C2(_) => {
                        Err::<Adt16, R<Result<Option<Adt20>, Bytes>>>(Ok(Ok::<Option<Adt20>, Bytes>(None::<Adt20>)))
                    }
                    Adt16::C3(r1393, r1394) => {
                        let v2 = (*r1393).clone();
                        let v3 = (*r1394).clone();
                        let m1385 = v2;
                        match m1385 {
                            Adt16::C0(v4, r1391, r1392) => {
                                let v5 = (*r1391).clone();
                                let v6 = (*r1392).clone();
                                let m1386 = f88(v5.clone(), v1.clone());
                                match m1386 {
                                    Ok(v8) => {
                                        let m1387 = f89(v8.clone(), v4)?;
                                        match m1387 {
                                            Ok(v10) => {
                                                if f172({
                                                    let (_, h1388) = v10.clone();
                                                    h1388
                                                }, List::cons(f8(Str::lit("restriction")), List::cons(f8(Str::lit("list")), List::cons(f8(Str::lit("union")), List::<Bytes>::nil())))) {
                                                    if f35({
                                                        let (h1389, _) = v10;
                                                        h1389
                                                    }, f149()) {
                                                        Err::<Adt16, R<Result<Option<Adt20>, Bytes>>>(Ok(Ok::<Option<Adt20>, Bytes>(Some(Adt20::C0({
                                                            let (_, h1390) = v10;
                                                            h1390
                                                        }, v5, v6, v8)))))
                                                    } else {
                                                        Ok::<Adt16, R<Result<Option<Adt20>, Bytes>>>(v3)
                                                    }
                                                } else {
                                                    Ok::<Adt16, R<Result<Option<Adt20>, Bytes>>>(v3)
                                                }
                                            }
                                            Err(v9) => {
                                                Err::<Adt16, R<Result<Option<Adt20>, Bytes>>>(Ok(f297(v9)))
                                            }
                                        }
                                    }
                                    Err(v7) => {
                                        Err::<Adt16, R<Result<Option<Adt20>, Bytes>>>(Ok(f297(v7)))
                                    }
                                }
                            }
                            Adt16::C1(_, _, _) => {
                                Ok::<Adt16, R<Result<Option<Adt20>, Bytes>>>(v3)
                            }
                            Adt16::C2(_) => {
                                Ok::<Adt16, R<Result<Option<Adt20>, Bytes>>>(v3)
                            }
                            Adt16::C3(_, _) => {
                                Ok::<Adt16, R<Result<Option<Adt20>, Bytes>>>(v3)
                            }
                            Adt16::C4 => {
                                Ok::<Adt16, R<Result<Option<Adt20>, Bytes>>>(v3)
                            }
                        }
                    }
                    Adt16::C4 => {
                        Err::<Adt16, R<Result<Option<Adt20>, Bytes>>>(Ok(Ok::<Option<Adt20>, Bytes>(None::<Adt20>)))
                    }
                }
            };
            match step {
                Ok(next) => { m1395 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f217(v0: Bytes) -> Result<Adt19, Bytes> {
    Err::<Adt19, Bytes>(v0)
}

pub fn f218(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit(" at simpleType ")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f219(v0: List<(Bytes, Bytes)>, v1: Bytes, v2: Bytes) -> R<Result<Adt19, Bytes>> {
    let m1397 = f89(v0, v2.clone())?;
    match m1397 {
        Ok(v4) => {
            if f35({
                let (h1398, _) = v4.clone();
                h1398
            }, f149()) {
                let m1400 = f298({
                    let (_, h1399) = v4;
                    h1399
                }, 16u64)?;
                match m1400 {
                    Ok(v6) => {
                        Ok(Ok::<Adt19, Bytes>(Adt19::C0(v6, List::<Bytes>::nil())))
                    }
                    Err(v5) => {
                        Ok(f217(v5))
                    }
                }
            } else {
                if f35({
                    let (h1401, _) = v4.clone();
                    h1401
                }, v1) {
                    let v7: Bytes = f152({
                        let (_, h1402) = v4;
                        h1402
                    })?;
                    Ok(Ok::<Adt19, Bytes>(Adt19::C0(v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                } else {
                    Ok(f217(f7(List::cons(f8(Str::lit("the type ")), List::cons(v2, List::cons(f8(Str::lit(" is in the namespace ")), List::cons({
                        let (h1403, _) = v4;
                        h1403
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
    let m1404 = v0;
    match m1404 {
        Adt16::C0(_, _, _) => {
            Ok(Ok::<List<Adt17>, Bytes>(List::<Adt17>::nil()))
        }
        Adt16::C1(_, _, _) => {
            Ok(Ok::<List<Adt17>, Bytes>(List::<Adt17>::nil()))
        }
        Adt16::C2(_) => {
            Ok(Ok::<List<Adt17>, Bytes>(List::<Adt17>::nil()))
        }
        Adt16::C3(r1411, r1412) => {
            let v2 = (*r1411).clone();
            let v3 = (*r1412).clone();
            let m1405 = f220(v3, v1.clone())?;
            match m1405 {
                Ok(v5) => {
                    let m1406 = v2;
                    match m1406 {
                        Adt16::C0(v6, r1410, _) => {
                            let v7 = (*r1410).clone();
                            let m1407 = f88(v7.clone(), v1);
                            match m1407 {
                                Ok(v10) => {
                                    let m1408 = f89(v10, v6)?;
                                    match m1408 {
                                        Ok(v12) => {
                                            if f90(v12, Str::lit("enumeration")) {
                                                let m1409 = f91(v7, f8(Str::lit("value")));
                                                match m1409 {
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
    let m1413 = v0;
    match m1413.uncons() {
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
    let m1414 = v0;
    match m1414.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1415 = f300(Some(v1));
            match m1415 {
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

pub fn f223(v0: Bytes, p1436: u64, p1437: u64, p1438: bool, p1439: Bytes) -> R<Bytes> {
    let m1435 = (p1436, (p1437, (p1438, p1439)));
    {
        let mut m1435 = m1435;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1435;
                let m1416 = v2;
                if m1416 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1416 - 1;
                    let m1419 = {
                        let a1417 = v0.clone();
                        let a1418 = v1;
                        index_bytes(a1417, a1418)
                    };
                    match m1419 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f41(v6)?;
                            if f301(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1420 = v1;
                                    let a1421 = 1u64;
                                    nat_add(a1420, a1421)?
                                }, (v5, (true, v4))))
                            } else {
                                if f302(f303(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1422 = v1;
                                        let a1423 = 1u64;
                                        nat_add(a1422, a1423)?
                                    }, (v5, (false, {
                                        let a1427 = v4;
                                        let a1428 = f32({
                                            let a1424 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1424)
                                        }, {
                                            let a1425 = v7;
                                            let a1426 = 97u64;
                                            nat_sub(a1425, a1426)
                                        }, 1u64);
                                        append_bytes(a1427, a1428)
                                    }))))
                                } else {
                                    if f304(f303(v7), f304(f305(v7), f306(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1429 = v1;
                                            let a1430 = 1u64;
                                            nat_add(a1429, a1430)?
                                        }, (v5, (false, {
                                            let a1431 = v4;
                                            let a1432 = f32(v0.clone(), v1, 1u64);
                                            append_bytes(a1431, a1432)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1433 = v1;
                                            let a1434 = 1u64;
                                            nat_add(a1433, a1434)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1435 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f224(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1440 = f288(v0, v1)?;
    match m1440 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f307({
                let (h1441, _) = v2;
                h1441
            }) {
                Ok(Some({
                    let a1443 = v1;
                    let a1444 = {
                        let (_, h1442) = v2;
                        h1442
                    };
                    nat_add(a1443, a1444)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f225(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1446 = v1;
    let a1447 = {
        let a1445 = v0.clone();
        length_bytes(a1445)
    };
    if if nat_lt(a1446, a1447) {
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
    let (h1448, _) = f128(Fn4::F309(v0), {
        let a1449 = v1;
        let a1450 = 1u64;
        nat_add(a1449, a1450)?
    }, v1)?;
    Ok(h1448)
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

pub fn f230(v0: Bytes, v1: u64, v2: u64) -> R<Result<(Adt17, u64), Bytes>> {
    let v3: bool = {
        let a1451 = f100(v0.clone(), v1)?;
        let a1452 = 62u64;
        nat_eq(a1451, a1452)
    };
    let v4: u64 = f100(v0.clone(), {
        let a1453 = v1;
        let a1454 = 1u64;
        nat_add(a1453, a1454)?
    })?;
    let v5: u64 = f100(v0.clone(), {
        let a1455 = v1;
        let a1456 = 2u64;
        nat_add(a1455, a1456)?
    })?;
    let v6: u64 = f313(v4);
    let v7: u64 = f314(v4);
    let v8: bool = {
        let a1457 = 0u64;
        let a1458 = v6;
        if nat_lt(a1457, a1458) {
            true
        } else {
            let a1459 = 0u64;
            let a1460 = v7;
            nat_lt(a1459, a1460)
        }
    };
    let v9: u64 = {
        let a1461 = 0u64;
        let a1462 = v7;
        if nat_lt(a1461, a1462) {
            f313(v5)
        } else {
            0u64
        }
    };
    let v10: u64 = {
        let a1463 = 0u64;
        let a1464 = v6;
        if nat_lt(a1463, a1464) {
            f314(v5)
        } else {
            0u64
        }
    };
    let v11: u64 = {
        let a1465 = v6;
        let a1466 = v9;
        nat_add(a1465, a1466)?
    };
    let v12: u64 = {
        let a1467 = v7;
        let a1468 = v10;
        nat_add(a1467, a1468)?
    };
    let v13: u64 = {
        let a1477 = {
            let a1469 = v1;
            let a1470 = 1u64;
            nat_add(a1469, a1470)?
        };
        let a1478 = {
            let a1475 = if v8 {
                1u64
            } else {
                0u64
            };
            let a1476 = {
                let a1471 = 0u64;
                let a1472 = v9;
                if if nat_lt(a1471, a1472) {
                    true
                } else {
                    let a1473 = 0u64;
                    let a1474 = v10;
                    nat_lt(a1473, a1474)
                } {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a1475, a1476)?
        };
        nat_add(a1477, a1478)?
    };
    let m1479 = f173(v0.clone(), v13)?;
    match m1479 {
        Ok(v15) => {
            let a1480 = 0u64;
            let a1481 = v12;
            if nat_lt(a1480, a1481) {
                f315(v0, v15, {
                    let a1484 = {
                        let a1482 = v2;
                        let a1483 = v12;
                        nat_add(a1482, a1483)?
                    };
                    let a1485 = 1u64;
                    nat_sub(a1484, a1485)
                }, v3, v11)
            } else {
                let v16: (u64, u64) = {
                    let (h1486, _) = f316(Fn7::F317(v0.clone()), {
                        let a1488 = {
                            let a1487 = v0.clone();
                            length_bytes(a1487)
                        };
                        let a1489 = 1u64;
                        nat_add(a1488, a1489)?
                    }, (v15, 0u64))?;
                    h1486
                };
                let v17: u64 = f160(v0.clone(), {
                    let (h1490, _) = v16;
                    h1490
                })?;
                let a1491 = f100(v0.clone(), v17)?;
                let a1492 = 256u64;
                if nat_eq(a1491, a1492) {
                    Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C3({
                        let a1493 = v11;
                        let a1494 = 2u64;
                        if nat_eq(a1493, a1494) {
                            f318(f319(v0.clone(), v15)?)
                        } else {
                            Bytes::lit(&[])
                        }
                    }), {
                        let a1495 = v0;
                        length_bytes(a1495)
                    })))
                } else {
                    let v18: u64 = f320({
                        let a1497 = v17;
                        let a1498 = {
                            let (h1496, _) = v16;
                            h1496
                        };
                        nat_sub(a1497, a1498)
                    }, v2);
                    let a1500 = v18;
                    let a1501 = {
                        let (_, h1499) = v16;
                        h1499
                    };
                    if nat_lt(a1500, a1501) {
                        Ok(Err::<(Adt17, u64), Bytes>(f81(Str::lit("a leading empty line of a block scalar holds more spaces than its first content line (YAML 1.2.2 section 8.1.1.1)"), v15)?))
                    } else {
                        f315(v0, v15, v18, v3, v11)
                    }
                }
            }
        }
        Err(v14) => {
            Ok(Err::<(Adt17, u64), Bytes>(v14))
        }
    }
}

pub fn f231(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Result<(Bytes, u64), Bytes>> {
    let a1502 = f100(v0.clone(), v1)?;
    let a1503 = 34u64;
    if nat_eq(a1502, a1503) {
        f321(v0.clone(), {
            let a1504 = v1;
            let a1505 = 1u64;
            nat_add(a1504, a1505)?
        }, v2, v3, {
            let a1507 = {
                let a1506 = v0;
                length_bytes(a1506)
            };
            let a1508 = 2u64;
            nat_add(a1507, a1508)?
        }, Bytes::lit(&[]))
    } else {
        f322(v0.clone(), {
            let a1509 = v1;
            let a1510 = 1u64;
            nat_add(a1509, a1510)?
        }, v2, v3, {
            let a1512 = {
                let a1511 = v0;
                length_bytes(a1511)
            };
            let a1513 = 2u64;
            nat_add(a1512, a1513)?
        }, Bytes::lit(&[]))
    }
}

pub fn f232(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: List<Bytes>) -> R<Result<(Adt17, u64), Bytes>> {
    let m1514 = v3;
    if m1514 == 0 {
        Ok(Err::<(Adt17, u64), Bytes>(f81(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v6 = m1514 - 1;
        let a1515 = v4;
        let a1516 = 1u64;
        if nat_eq(a1515, a1516) {
            let m1517 = f232(v0.clone(), v1, v2, v6, 0u64, List::<Bytes>::nil())?;
            match m1517 {
                Ok(v8) => {
                    let m1519 = f323(v0.clone(), {
                        let (_, h1518) = v8;
                        h1518
                    }, v2)?;
                    match m1519 {
                        Ok(v10) => {
                            let v11: u64 = f100(v0.clone(), v10)?;
                            let a1520 = v11;
                            let a1521 = 58u64;
                            if nat_eq(a1520, a1521) {
                                Ok(Err::<(Adt17, u64), Bytes>(f81(Str::lit("a single-pair mapping inside a flow sequence is not read (YAML 1.2.2 [150])"), v10)?))
                            } else {
                                let a1522 = v11;
                                let a1523 = 93u64;
                                if nat_eq(a1522, a1523) {
                                    Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C7(std::rc::Rc::new({
                                        let (h1524, _) = v8;
                                        h1524
                                    }), std::rc::Rc::new(Adt17::C6)), {
                                        let a1525 = v10;
                                        let a1526 = 1u64;
                                        nat_add(a1525, a1526)?
                                    })))
                                } else {
                                    let a1527 = v11;
                                    let a1528 = 44u64;
                                    if nat_eq(a1527, a1528) {
                                        let m1531 = f323(v0.clone(), {
                                            let a1529 = v10;
                                            let a1530 = 1u64;
                                            nat_add(a1529, a1530)?
                                        }, v2)?;
                                        match m1531 {
                                            Ok(v13) => {
                                                let a1532 = f100(v0.clone(), v13)?;
                                                let a1533 = 93u64;
                                                if nat_eq(a1532, a1533) {
                                                    Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C7(std::rc::Rc::new({
                                                        let (h1534, _) = v8;
                                                        h1534
                                                    }), std::rc::Rc::new(Adt17::C6)), {
                                                        let a1535 = v13;
                                                        let a1536 = 1u64;
                                                        nat_add(a1535, a1536)?
                                                    })))
                                                } else {
                                                    let m1537 = f232(v0, v13, v2, v6, 1u64, List::<Bytes>::nil())?;
                                                    match m1537 {
                                                        Ok(v15) => {
                                                            Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C7(std::rc::Rc::new({
                                                                let (h1538, _) = v8;
                                                                h1538
                                                            }), std::rc::Rc::new({
                                                                let (h1539, _) = v15;
                                                                h1539
                                                            })), {
                                                                let (_, h1540) = v15;
                                                                h1540
                                                            })))
                                                        }
                                                        Err(v14) => {
                                                            Ok(Err::<(Adt17, u64), Bytes>(v14))
                                                        }
                                                    }
                                                }
                                            }
                                            Err(v12) => {
                                                Ok(Err::<(Adt17, u64), Bytes>(v12))
                                            }
                                        }
                                    } else {
                                        Ok(Err::<(Adt17, u64), Bytes>(f81(Str::lit("a flow sequence entry must be followed by , or ] (YAML 1.2.2 [137])"), v10)?))
                                    }
                                }
                            }
                        }
                        Err(v9) => {
                            Ok(Err::<(Adt17, u64), Bytes>(v9))
                        }
                    }
                }
                Err(v7) => {
                    Ok(Err::<(Adt17, u64), Bytes>(v7))
                }
            }
        } else {
            let a1541 = v4;
            let a1542 = 2u64;
            if nat_eq(a1541, a1542) {
                let m1543 = f324(v0.clone(), v1, v2)?;
                match m1543 {
                    Ok(v17) => {
                        let m1545 = f323(v0.clone(), {
                            let (_, h1544) = v17.clone();
                            h1544
                        }, v2)?;
                        match m1545 {
                            Ok(v19) => {
                                if f169({
                                    let a1546 = f100(v0.clone(), v19)?;
                                    let a1547 = 58u64;
                                    nat_eq(a1546, a1547)
                                }) {
                                    Ok(Err::<(Adt17, u64), Bytes>(f81(Str::lit("a flow mapping entry needs : and a value; a null value is not read (YAML 1.2.2 [142])"), v19)?))
                                } else {
                                    let m1550 = f323(v0.clone(), {
                                        let a1548 = v19;
                                        let a1549 = 1u64;
                                        nat_add(a1548, a1549)?
                                    }, v2)?;
                                    match m1550 {
                                        Ok(v21) => {
                                            let a1551 = f100(v0.clone(), v21)?;
                                            let a1552 = 44u64;
                                            if if nat_eq(a1551, a1552) {
                                                true
                                            } else {
                                                let a1553 = f100(v0.clone(), v21)?;
                                                let a1554 = 125u64;
                                                nat_eq(a1553, a1554)
                                            } {
                                                Ok(Err::<(Adt17, u64), Bytes>(f168(v21)?))
                                            } else {
                                                let m1555 = f232(v0.clone(), v21, v2, v6, 0u64, List::<Bytes>::nil())?;
                                                match m1555 {
                                                    Ok(v23) => {
                                                        if f172({
                                                            let (h1556, _) = v17.clone();
                                                            h1556
                                                        }, v5.clone()) {
                                                            Ok(Err::<(Adt17, u64), Bytes>(f81(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                                        } else {
                                                            let m1558 = f323(v0.clone(), {
                                                                let (_, h1557) = v23;
                                                                h1557
                                                            }, v2)?;
                                                            match m1558 {
                                                                Ok(v25) => {
                                                                    let v26: u64 = f100(v0.clone(), v25)?;
                                                                    let a1559 = v26;
                                                                    let a1560 = 125u64;
                                                                    if nat_eq(a1559, a1560) {
                                                                        Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C8({
                                                                            let (h1561, _) = v17;
                                                                            h1561
                                                                        }, std::rc::Rc::new({
                                                                            let (h1562, _) = v23;
                                                                            h1562
                                                                        }), std::rc::Rc::new(Adt17::C6)), {
                                                                            let a1563 = v25;
                                                                            let a1564 = 1u64;
                                                                            nat_add(a1563, a1564)?
                                                                        })))
                                                                    } else {
                                                                        let a1565 = v26;
                                                                        let a1566 = 44u64;
                                                                        if nat_eq(a1565, a1566) {
                                                                            let m1569 = f323(v0.clone(), {
                                                                                let a1567 = v25;
                                                                                let a1568 = 1u64;
                                                                                nat_add(a1567, a1568)?
                                                                            }, v2)?;
                                                                            match m1569 {
                                                                                Ok(v28) => {
                                                                                    let a1570 = f100(v0.clone(), v28)?;
                                                                                    let a1571 = 125u64;
                                                                                    if nat_eq(a1570, a1571) {
                                                                                        Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C8({
                                                                                            let (h1572, _) = v17;
                                                                                            h1572
                                                                                        }, std::rc::Rc::new({
                                                                                            let (h1573, _) = v23;
                                                                                            h1573
                                                                                        }), std::rc::Rc::new(Adt17::C6)), {
                                                                                            let a1574 = v28;
                                                                                            let a1575 = 1u64;
                                                                                            nat_add(a1574, a1575)?
                                                                                        })))
                                                                                    } else {
                                                                                        let m1577 = f232(v0, v28, v2, v6, 2u64, List::cons({
                                                                                            let (h1576, _) = v17.clone();
                                                                                            h1576
                                                                                        }, v5))?;
                                                                                        match m1577 {
                                                                                            Ok(v30) => {
                                                                                                Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C8({
                                                                                                    let (h1578, _) = v17;
                                                                                                    h1578
                                                                                                }, std::rc::Rc::new({
                                                                                                    let (h1579, _) = v23;
                                                                                                    h1579
                                                                                                }), std::rc::Rc::new({
                                                                                                    let (h1580, _) = v30;
                                                                                                    h1580
                                                                                                })), {
                                                                                                    let (_, h1581) = v30;
                                                                                                    h1581
                                                                                                })))
                                                                                            }
                                                                                            Err(v29) => {
                                                                                                Ok(Err::<(Adt17, u64), Bytes>(v29))
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                Err(v27) => {
                                                                                    Ok(Err::<(Adt17, u64), Bytes>(v27))
                                                                                }
                                                                            }
                                                                        } else {
                                                                            Ok(Err::<(Adt17, u64), Bytes>(f81(Str::lit("a flow mapping entry must be followed by , or } (YAML 1.2.2 [140])"), v25)?))
                                                                        }
                                                                    }
                                                                }
                                                                Err(v24) => {
                                                                    Ok(Err::<(Adt17, u64), Bytes>(v24))
                                                                }
                                                            }
                                                        }
                                                    }
                                                    Err(v22) => {
                                                        Ok(Err::<(Adt17, u64), Bytes>(v22))
                                                    }
                                                }
                                            }
                                        }
                                        Err(v20) => {
                                            Ok(Err::<(Adt17, u64), Bytes>(v20))
                                        }
                                    }
                                }
                            }
                            Err(v18) => {
                                Ok(Err::<(Adt17, u64), Bytes>(v18))
                            }
                        }
                    }
                    Err(v16) => {
                        Ok(Err::<(Adt17, u64), Bytes>(v16))
                    }
                }
            } else {
                let v31: u64 = f100(v0.clone(), v1)?;
                let a1582 = v31;
                let a1583 = 91u64;
                if if nat_eq(a1582, a1583) {
                    true
                } else {
                    let a1584 = v31;
                    let a1585 = 123u64;
                    nat_eq(a1584, a1585)
                } {
                    let m1588 = f323(v0.clone(), {
                        let a1586 = v1;
                        let a1587 = 1u64;
                        nat_add(a1586, a1587)?
                    }, v2)?;
                    match m1588 {
                        Ok(v33) => {
                            let a1591 = f100(v0.clone(), v33)?;
                            let a1592 = {
                                let a1589 = v31;
                                let a1590 = 2u64;
                                nat_add(a1589, a1590)?
                            };
                            if nat_eq(a1591, a1592) {
                                Ok(Ok::<(Adt17, u64), Bytes>(({
                                    let a1593 = v31;
                                    let a1594 = 91u64;
                                    if nat_eq(a1593, a1594) {
                                        Adt17::C4(std::rc::Rc::new(Adt17::C6))
                                    } else {
                                        Adt17::C5(std::rc::Rc::new(Adt17::C6))
                                    }
                                }, {
                                    let a1595 = v33;
                                    let a1596 = 1u64;
                                    nat_add(a1595, a1596)?
                                })))
                            } else {
                                let m1599 = f232(v0, v33, v2, v6, {
                                    let a1597 = v31;
                                    let a1598 = 91u64;
                                    if nat_eq(a1597, a1598) {
                                        1u64
                                    } else {
                                        2u64
                                    }
                                }, List::<Bytes>::nil())?;
                                match m1599 {
                                    Ok(v35) => {
                                        Ok(Ok::<(Adt17, u64), Bytes>(({
                                            let a1600 = v31;
                                            let a1601 = 91u64;
                                            if nat_eq(a1600, a1601) {
                                                Adt17::C4(std::rc::Rc::new({
                                                    let (h1602, _) = v35;
                                                    h1602
                                                }))
                                            } else {
                                                Adt17::C5(std::rc::Rc::new({
                                                    let (h1603, _) = v35;
                                                    h1603
                                                }))
                                            }
                                        }, {
                                            let (_, h1604) = v35;
                                            h1604
                                        })))
                                    }
                                    Err(v34) => {
                                        Ok(Err::<(Adt17, u64), Bytes>(v34))
                                    }
                                }
                            }
                        }
                        Err(v32) => {
                            Ok(Err::<(Adt17, u64), Bytes>(v32))
                        }
                    }
                } else {
                    let a1605 = v31;
                    let a1606 = 34u64;
                    if if nat_eq(a1605, a1606) {
                        true
                    } else {
                        let a1607 = v31;
                        let a1608 = 39u64;
                        nat_eq(a1607, a1608)
                    } {
                        let m1609 = f231(v0, v1, v2, false)?;
                        match m1609 {
                            Ok(v37) => {
                                Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C3({
                                    let (h1610, _) = v37;
                                    h1610
                                }), {
                                    let (_, h1611) = v37;
                                    h1611
                                })))
                            }
                            Err(v36) => {
                                Ok(Err::<(Adt17, u64), Bytes>(v36))
                            }
                        }
                    } else {
                        let m1612 = f164(v0.clone(), v1)?;
                        match m1612 {
                            None => {
                                if f169(f233(v0.clone(), v1, true)?) {
                                    Ok(Err::<(Adt17, u64), Bytes>(f81(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                                } else {
                                    let v39: u64 = f234(v0.clone(), v1, true)?;
                                    let v40: u64 = f161(v0.clone(), v39)?;
                                    let m1615 = {
                                        let a1613 = f100(v0.clone(), v40)?;
                                        let a1614 = 10u64;
                                        if nat_eq(a1613, a1614) {
                                            f323(v0.clone(), v40, v2)?
                                        } else {
                                            Ok::<u64, Bytes>(v40)
                                        }
                                    };
                                    match m1615 {
                                        Ok(v42) => {
                                            let a1616 = f100(v0.clone(), v40)?;
                                            let a1617 = 10u64;
                                            if if if nat_eq(a1616, a1617) {
                                                f169(f312(f100(v0.clone(), v42)?))
                                            } else {
                                                false
                                            } {
                                                f169({
                                                    let a1618 = f100(v0.clone(), v42)?;
                                                    let a1619 = 58u64;
                                                    nat_eq(a1618, a1619)
                                                })
                                            } else {
                                                false
                                            } {
                                                Ok(Err::<(Adt17, u64), Bytes>(f81(Str::lit("a multi-line plain scalar inside a flow collection is not read (YAML 1.2.2 [135])"), v40)?))
                                            } else {
                                                let m1620 = f237(f236(v0, v1, v39), v1)?;
                                                match m1620 {
                                                    Ok(v44) => {
                                                        Ok(Ok::<(Adt17, u64), Bytes>((v44, v39)))
                                                    }
                                                    Err(v43) => {
                                                        Ok(Err::<(Adt17, u64), Bytes>(v43))
                                                    }
                                                }
                                            }
                                        }
                                        Err(v41) => {
                                            Ok(Err::<(Adt17, u64), Bytes>(v41))
                                        }
                                    }
                                }
                            }
                            Some(v38) => {
                                Ok(Err::<(Adt17, u64), Bytes>(v38))
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
        let a1621 = v3;
        let a1622 = 45u64;
        if if if nat_eq(a1621, a1622) {
            true
        } else {
            let a1623 = v3;
            let a1624 = 63u64;
            nat_eq(a1623, a1624)
        } {
            true
        } else {
            let a1625 = v3;
            let a1626 = 58u64;
            nat_eq(a1625, a1626)
        } {
            Ok(f229(f100(v0, {
                let a1627 = v1;
                let a1628 = 1u64;
                nat_add(a1627, a1628)?
            })?, v2))
        } else {
            Ok(f169(f325(v3)))
        }
    }
}

pub fn f234(v0: Bytes, v1: u64, v2: bool) -> R<u64> {
    let (_, h1629) = {
        let (h1630, _) = f316(Fn7::F326(v2, v0.clone()), {
            let a1632 = {
                let a1631 = v0;
                length_bytes(a1631)
            };
            let a1633 = 1u64;
            nat_add(a1632, a1633)?
        }, (v1, v1))?;
        h1630
    };
    Ok(h1629)
}

pub fn f235(v0: Bytes, p1658: u64, v2: u64, p1659: u64, p1660: Bytes) -> R<Result<(Bytes, u64), Bytes>> {
    let m1657 = (p1658, (p1659, p1660));
    {
        let mut m1657 = m1657;
        loop {
            let step = {
                let (v1, (v3, v4)) = m1657;
                let m1634 = v3;
                if m1634 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f81(Str::lit("a plain scalar has more lines than the input"), v1)?)))
                } else {
                    let v5 = m1634 - 1;
                    let v6: u64 = f161(v0.clone(), v1)?;
                    let v7: u64 = f100(v0.clone(), v6)?;
                    let a1635 = v7;
                    let a1636 = 58u64;
                    if nat_eq(a1635, a1636) {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f327(v6)?)))
                    } else {
                        let a1637 = v7;
                        let a1638 = 35u64;
                        if if nat_eq(a1637, a1638) {
                            true
                        } else {
                            let a1639 = v7;
                            let a1640 = 256u64;
                            nat_eq(a1639, a1640)
                        } {
                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v4, f167(v0.clone(), v6)?))))
                        } else {
                            let v8: (u64, u64) = f328(v0.clone(), {
                                let a1641 = v6;
                                let a1642 = 1u64;
                                nat_add(a1641, a1642)?
                            })?;
                            let v9: u64 = {
                                let (h1643, _) = v8;
                                h1643
                            };
                            let v10: u64 = f160(v0.clone(), v9)?;
                            let v11: u64 = f161(v0.clone(), v10)?;
                            let v12: u64 = f100(v0.clone(), v11)?;
                            let a1644 = v12;
                            let a1645 = 256u64;
                            if if if if nat_eq(a1644, a1645) {
                                true
                            } else {
                                let a1648 = {
                                    let a1646 = v10;
                                    let a1647 = v9;
                                    nat_sub(a1646, a1647)
                                };
                                let a1649 = v2;
                                nat_lt(a1648, a1649)
                            } {
                                true
                            } else {
                                let a1650 = v12;
                                let a1651 = 35u64;
                                nat_eq(a1650, a1651)
                            } {
                                true
                            } else {
                                f99(v0.clone(), v9)?
                            } {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v4, {
                                    let a1652 = v6;
                                    let a1653 = 1u64;
                                    nat_add(a1652, a1653)?
                                }))))
                            } else {
                                let v13: u64 = f234(v0.clone(), v11, false)?;
                                let a1654 = v13;
                                let a1655 = v11;
                                if nat_eq(a1654, a1655) {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f327(v11)?)))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v13, (v5, f7(List::cons(v4, List::cons(f329({
                                        let (_, h1656) = v8;
                                        h1656
                                    }), List::cons(f236(v0.clone(), v11, v13), List::<Bytes>::nil())))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1657 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f236(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    f32(v0, v1, {
        let a1661 = v2;
        let a1662 = v1;
        nat_sub(a1661, a1662)
    })
}

pub fn f237(v0: Bytes, v1: u64) -> R<Result<Adt17, Bytes>> {
    let a1664 = {
        let a1663 = v0.clone();
        length_bytes(a1663)
    };
    let a1665 = 0u64;
    if if if if nat_eq(a1664, a1665) {
        true
    } else {
        f330(v0.clone(), Str::lit("~"))
    } {
        true
    } else {
        if f330(v0.clone(), Str::lit("null")) {
            true
        } else {
            f330(v0.clone(), Str::lit("Null"))
        }
    } {
        true
    } else {
        f330(v0.clone(), Str::lit("NULL"))
    } {
        Ok(Err::<Adt17, Bytes>(f168(v1)?))
    } else {
        if if if f330(v0.clone(), Str::lit("true")) {
            true
        } else {
            f330(v0.clone(), Str::lit("True"))
        } {
            true
        } else {
            f330(v0.clone(), Str::lit("TRUE"))
        } {
            Ok(Ok::<Adt17, Bytes>(Adt17::C0(true)))
        } else {
            if if if f330(v0.clone(), Str::lit("false")) {
                true
            } else {
                f330(v0.clone(), Str::lit("False"))
            } {
                true
            } else {
                f330(v0.clone(), Str::lit("FALSE"))
            } {
                Ok(Ok::<Adt17, Bytes>(Adt17::C0(false)))
            } else {
                if if if if if f330(v0.clone(), Str::lit(".inf")) {
                    true
                } else {
                    f330(v0.clone(), Str::lit(".Inf"))
                } {
                    true
                } else {
                    if f330(v0.clone(), Str::lit(".INF")) {
                        true
                    } else {
                        f330(v0.clone(), Str::lit("+.inf"))
                    }
                } {
                    true
                } else {
                    if if f330(v0.clone(), Str::lit("+.Inf")) {
                        true
                    } else {
                        f330(v0.clone(), Str::lit("+.INF"))
                    } {
                        true
                    } else {
                        if f330(v0.clone(), Str::lit("-.inf")) {
                            true
                        } else {
                            f330(v0.clone(), Str::lit("-.Inf"))
                        }
                    }
                } {
                    true
                } else {
                    if if f330(v0.clone(), Str::lit("-.INF")) {
                        true
                    } else {
                        f330(v0.clone(), Str::lit(".nan"))
                    } {
                        true
                    } else {
                        if f330(v0.clone(), Str::lit(".NaN")) {
                            true
                        } else {
                            f330(v0.clone(), Str::lit(".NAN"))
                        }
                    }
                } {
                    Ok(Err::<Adt17, Bytes>(f81(Str::lit("an infinite or not-a-number float is not read: Studio.Value has none (YAML 1.2.2 section 10.3.2)"), v1)?))
                } else {
                    if if f84(v0.clone(), 0u64, Str::lit("0o")) {
                        let a1667 = 2u64;
                        let a1668 = {
                            let a1666 = v0.clone();
                            length_bytes(a1666)
                        };
                        nat_lt(a1667, a1668)
                    } else {
                        false
                    } {
                        f331(v0, 8u64, v1)
                    } else {
                        if if f84(v0.clone(), 0u64, Str::lit("0x")) {
                            let a1670 = 2u64;
                            let a1671 = {
                                let a1669 = v0.clone();
                                length_bytes(a1669)
                            };
                            nat_lt(a1670, a1671)
                        } else {
                            false
                        } {
                            f331(v0, 16u64, v1)
                        } else {
                            let m1672 = f332(v0.clone())?;
                            match m1672 {
                                None => {
                                    Ok(Ok::<Adt17, Bytes>(Adt17::C3(v0)))
                                }
                                Some(v2) => {
                                    let m1673 = f333(v2.clone(), 0u64)?;
                                    match m1673 {
                                        None => {
                                            Ok(Err::<Adt17, Bytes>(f334(v1)?))
                                        }
                                        Some(v3) => {
                                            let a1676 = {
                                                let (h1674, _) = v3;
                                                h1674
                                            };
                                            let a1677 = {
                                                let a1675 = v2;
                                                length_bytes(a1675)
                                            };
                                            if nat_eq(a1676, a1677) {
                                                Ok(Ok::<Adt17, Bytes>({
                                                    let (_, h1678) = v3;
                                                    h1678
                                                }))
                                            } else {
                                                Ok(Err::<Adt17, Bytes>(f334(v1)?))
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
    let (h1679, _) = f128(Fn4::F335(v0.clone()), {
        let a1681 = {
            let a1680 = v0;
            length_bytes(a1680)
        };
        let a1682 = 1u64;
        nat_add(a1681, a1682)?
    }, v1)?;
    Ok(h1679)
}

pub fn f239(v0: Bytes, v1: u64) -> R<bool> {
    let a1683 = f100(v0.clone(), v1)?;
    let a1684 = 58u64;
    if nat_eq(a1683, a1684) {
        if f157(f100(v0.clone(), {
            let a1685 = v1;
            let a1686 = 1u64;
            nat_add(a1685, a1686)?
        })?) {
            Ok(true)
        } else {
            Ok(f158(f100(v0, {
                let a1687 = v1;
                let a1688 = 1u64;
                nat_add(a1687, a1688)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f240(v0: u64) -> Bytes {
    let m1689 = v0;
    if m1689 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1689 - 1;
        let a1690 = f8(Str::lit("0"));
        let a1691 = f240(v1);
        append_bytes(a1690, a1691)
    }
}

pub fn f241(v0: Bytes, p1704: u64, p1705: u64) -> R<u64> {
    let m1703 = (p1704, p1705);
    {
        let mut m1703 = m1703;
        loop {
            let step = {
                let (v1, v2) = m1703;
                let m1692 = v2;
                if m1692 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m1692 - 1;
                    let v4: u64 = f137(v0.clone(), v1)?;
                    let a1693 = v4;
                    let a1694 = 34u64;
                    if nat_eq(a1693, a1694) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a1695 = v4;
                        let a1696 = 92u64;
                        if nat_eq(a1695, a1696) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a1697 = v4;
                            let a1698 = 32u64;
                            if nat_lt(a1697, a1698) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a1699 = v4;
                                let a1700 = 256u64;
                                if nat_eq(a1699, a1700) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a1701 = v1;
                                        let a1702 = 1u64;
                                        nat_add(a1701, a1702)?
                                    }, v3))
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

pub fn f242(v0: Bytes) -> R<Bytes> {
    let a1711 = {
        let a1709 = Bytes::lit(&[0x22]);
        let a1710 = {
            let (_, h1706) = {
                let (h1707, _) = f125(Fn3::F336(v0.clone()), {
                    let a1708 = v0;
                    length_bytes(a1708)
                }, (0u64, Bytes::lit(&[])))?;
                h1707
            };
            h1706
        };
        append_bytes(a1709, a1710)
    };
    let a1712 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a1711, a1712))
}

pub fn f243(v0: Fn5, p1716: List<(Bytes, Bytes)>, p1717: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1715 = (p1716, p1717);
    {
        let mut m1715 = m1715;
        loop {
            let step = {
                let (v1, v2) = m1715;
                let m1713 = v2;
                match m1713.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1714: Fn5 = v0.clone();
                            c1714.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1715 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f244(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f337(v0, v1)
}

pub fn f245(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f338(v0)
}

pub fn f246(v0: Fn6, p1721: List<Bytes>, p1722: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1720 = (p1721, p1722);
    {
        let mut m1720 = m1720;
        loop {
            let step = {
                let (v1, v2) = m1720;
                let m1718 = v2;
                match m1718.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1719: Fn6 = v0.clone();
                            c1719.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1720 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f247(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f339(v1)?, v0))
}

pub fn f248(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a1723 = v0;
        utf8_encode(a1723)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f51(v1)?, List::<Bytes>::nil())))))
}

pub fn f249(p1726: List<Adt18>, p1727: List<Adt18>) -> List<Adt18> {
    let m1725 = (p1726, p1727);
    {
        let mut m1725 = m1725;
        loop {
            let step = {
                let (v0, v1) = m1725;
                let m1724 = v0;
                match m1724.uncons() {
                    None => {
                        Err::<(List<Adt18>, List<Adt18>), List<Adt18>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt18>, List<Adt18>), List<Adt18>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1725 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f250(v0: u64) -> bool {
    let a1728 = 65u64;
    let a1729 = v0;
    if if nat_le(a1728, a1729) {
        let a1730 = v0;
        let a1731 = 90u64;
        nat_le(a1730, a1731)
    } else {
        false
    } {
        true
    } else {
        let a1732 = 97u64;
        let a1733 = v0;
        if nat_le(a1732, a1733) {
            let a1734 = v0;
            let a1735 = 122u64;
            nat_le(a1734, a1735)
        } else {
            false
        }
    }
}

pub fn f251(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h1736, _) = f128(Fn4::F340(v1, v0.clone()), {
        let a1738 = {
            let a1737 = v0;
            length_bytes(a1737)
        };
        let a1739 = 1u64;
        nat_add(a1738, a1739)?
    }, v2)?;
    Ok(h1736)
}

pub fn f252(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f85(v0.clone(), v1)?;
    let a1740 = v2;
    let a1741 = 48u64;
    if nat_eq(a1740, a1741) {
        let v3: u64 = f85(v0.clone(), {
            let a1742 = v1;
            let a1743 = 1u64;
            nat_add(a1742, a1743)?
        })?;
        let a1744 = v3;
        let a1745 = 88u64;
        if if if nat_eq(a1744, a1745) {
            true
        } else {
            let a1746 = v3;
            let a1747 = 120u64;
            nat_eq(a1746, a1747)
        } {
            f341(f85(v0.clone(), {
                let a1748 = v1;
                let a1749 = 2u64;
                nat_add(a1748, a1749)?
            })?)
        } else {
            false
        } {
            f251(v0, 1u64, {
                let a1750 = v1;
                let a1751 = 2u64;
                nat_add(a1750, a1751)?
            })
        } else {
            f251(v0, 2u64, {
                let a1752 = v1;
                let a1753 = 1u64;
                nat_add(a1752, a1753)?
            })
        }
    } else {
        if f342(v2) {
            f251(v0, 0u64, v1)
        } else {
            Ok(v1)
        }
    }
}

pub fn f253(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f251(v0.clone(), 0u64, v1)?;
    let a1754 = f85(v0.clone(), v2)?;
    let a1755 = 46u64;
    if nat_eq(a1754, a1755) {
        let v3: u64 = f251(v0.clone(), 0u64, {
            let a1756 = v2;
            let a1757 = 1u64;
            nat_add(a1756, a1757)?
        })?;
        let a1758 = v1;
        let a1759 = v2;
        if if nat_lt(a1758, a1759) {
            true
        } else {
            let a1762 = {
                let a1760 = v2;
                let a1761 = 1u64;
                nat_add(a1760, a1761)?
            };
            let a1763 = v3;
            nat_lt(a1762, a1763)
        } {
            f343(v0, v3)
        } else {
            Ok(v1)
        }
    } else {
        let a1764 = v1;
        let a1765 = v2;
        if nat_lt(a1764, a1765) {
            let v4: u64 = f343(v0, v2)?;
            let a1766 = v2;
            let a1767 = v4;
            if nat_lt(a1766, a1767) {
                Ok(v4)
            } else {
                Ok(v1)
            }
        } else {
            Ok(v1)
        }
    }
}

pub fn f254(v0: List<Adt18>, v1: Str) -> bool {
    let m1768 = v0;
    match m1768.uncons() {
        None => {
            false
        }
        Some((v2, _)) => {
            f344(v2, {
                let a1769 = v1;
                utf8_encode(a1769)
            })
        }
    }
}

pub fn f255(p1782: u64, p1783: List<Adt18>, p1784: List<Bytes>, v3: bool) -> Result<List<Adt18>, Bytes> {
    let m1781 = (p1782, (p1783, p1784));
    {
        let mut m1781 = m1781;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1781;
                let m1770 = v0;
                if m1770 == 0 {
                    Err::<(u64, (List<Adt18>, List<Bytes>)), Result<List<Adt18>, Bytes>>(Err::<List<Adt18>, Bytes>(f8(Str::lit("brackets nest deeper than the input is long"))))
                } else {
                    let v4 = m1770 - 1;
                    let m1771 = v1;
                    match m1771.uncons() {
                        None => {
                            Err::<(u64, (List<Adt18>, List<Bytes>)), Result<List<Adt18>, Bytes>>(Err::<List<Adt18>, Bytes>(f8(Str::lit("the input ends inside brackets or before the ; that closes a member (Web IDL, IDL grammar)"))))
                        }
                        Some((v5, v6)) => {
                            let a1772 = f345(v5.clone());
                            let a1773 = 3u64;
                            if nat_eq(a1772, a1773) {
                                Ok::<(u64, (List<Adt18>, List<Bytes>)), Result<List<Adt18>, Bytes>>((v4, (v6, v2)))
                            } else {
                                let v7: Bytes = f346(v5);
                                let v8: Bytes = f347(v7.clone());
                                if !f35(v8.clone(), Bytes::lit(&[])) {
                                    Ok::<(u64, (List<Adt18>, List<Bytes>)), Result<List<Adt18>, Bytes>>((v4, (v6, List::cons(v8, v2))))
                                } else {
                                    if f348(v7.clone()) {
                                        let m1774 = v2;
                                        match m1774.uncons() {
                                            None => {
                                                Err::<(u64, (List<Adt18>, List<Bytes>)), Result<List<Adt18>, Bytes>>(Err::<List<Adt18>, Bytes>(f7(List::cons(f8(Str::lit("the closing bracket ")), List::cons(v7, List::cons(f8(Str::lit(" has no opening bracket (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
                                            }
                                            Some((v9, v10)) => {
                                                if f35(v9.clone(), v7.clone()) {
                                                    if if !v3 {
                                                        let a1776 = {
                                                            let a1775 = v10.clone();
                                                            length_list(a1775)
                                                        };
                                                        let a1777 = 0u64;
                                                        nat_eq(a1776, a1777)
                                                    } else {
                                                        false
                                                    } {
                                                        Err::<(u64, (List<Adt18>, List<Bytes>)), Result<List<Adt18>, Bytes>>(Ok::<List<Adt18>, Bytes>(v6))
                                                    } else {
                                                        Ok::<(u64, (List<Adt18>, List<Bytes>)), Result<List<Adt18>, Bytes>>((v4, (v6, v10)))
                                                    }
                                                } else {
                                                    Err::<(u64, (List<Adt18>, List<Bytes>)), Result<List<Adt18>, Bytes>>(Err::<List<Adt18>, Bytes>(f7(List::cons(f8(Str::lit("the closing bracket ")), List::cons(v7, List::cons(f8(Str::lit(" does not match the opening bracket, which needs ")), List::cons(v9, List::cons(f8(Str::lit(" (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))))
                                                }
                                            }
                                        }
                                    } else {
                                        if if v3 {
                                            if f35(v7, f8(Str::lit(";"))) {
                                                let a1779 = {
                                                    let a1778 = v2.clone();
                                                    length_list(a1778)
                                                };
                                                let a1780 = 0u64;
                                                nat_eq(a1779, a1780)
                                            } else {
                                                false
                                            }
                                        } else {
                                            false
                                        } {
                                            Err::<(u64, (List<Adt18>, List<Bytes>)), Result<List<Adt18>, Bytes>>(Ok::<List<Adt18>, Bytes>(v6))
                                        } else {
                                            Ok::<(u64, (List<Adt18>, List<Bytes>)), Result<List<Adt18>, Bytes>>((v4, (v6, v2)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1781 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f256(v0: List<Adt18>) -> List<Adt18> {
    let m1785 = v0;
    match m1785.uncons() {
        None => {
            List::<Adt18>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f257(v0: u64, v1: u64, v2: List<Adt18>, v3: bool) -> R<Result<(Adt12, List<Adt18>), Bytes>> {
    let m1786 = f260(v2);
    match m1786 {
        Ok(v5) => {
            let v6: bool = if v3 {
                f254(f262(v5.clone()), Str::lit(":"))
            } else {
                false
            };
            let m1787 = if v6 {
                f260(f256(f262(v5.clone())))
            } else {
                Ok::<Adt21, Bytes>(Adt21::C0(Bytes::lit(&[]), f262(v5.clone())))
            };
            match m1787 {
                Ok(v8) => {
                    let m1788 = f261(f262(v8.clone()), Str::lit("{"));
                    match m1788 {
                        Ok(v10) => {
                            let m1793 = {
                                let a1789 = v1;
                                let a1790 = 2u64;
                                if if nat_eq(a1789, a1790) {
                                    true
                                } else {
                                    let a1791 = v1;
                                    let a1792 = 7u64;
                                    nat_eq(a1791, a1792)
                                } {
                                    f349(v0, v10, List::<Adt13>::nil())?
                                } else {
                                    f350(v0, v10, List::<Adt13>::nil())?
                                }
                            };
                            match m1793 {
                                Ok(v12) => {
                                    Ok(Ok::<(Adt12, List<Adt18>), Bytes>((Adt12::C0(v1, f264(v5), f264(v8), {
                                        let (h1794, _) = v12;
                                        h1794
                                    }, List::<Bytes>::nil(), f265(Bytes::lit(&[]))), {
                                        let (_, h1795) = v12;
                                        h1795
                                    })))
                                }
                                Err(v11) => {
                                    Ok(Err::<(Adt12, List<Adt18>), Bytes>(v11))
                                }
                            }
                        }
                        Err(v9) => {
                            Ok(Err::<(Adt12, List<Adt18>), Bytes>(v9))
                        }
                    }
                }
                Err(v7) => {
                    Ok(Err::<(Adt12, List<Adt18>), Bytes>(v7))
                }
            }
        }
        Err(v4) => {
            Ok(Err::<(Adt12, List<Adt18>), Bytes>(v4))
        }
    }
}

pub fn f258(v0: List<Adt18>, v1: Bytes) -> R<Result<(Adt12, List<Adt18>), Bytes>> {
    let m1796 = f260(v0);
    match m1796 {
        Ok(v3) => {
            let m1797 = f351(f262(v3.clone()))?;
            match m1797 {
                Ok(v5) => {
                    Ok(Ok::<(Adt12, List<Adt18>), Bytes>((Adt12::C0(9u64, f264(v3), Bytes::lit(&[]), List::<Adt13>::nil(), List::cons(v1, List::<Bytes>::nil()), f265(Bytes::lit(&[]))), v5)))
                }
                Err(v4) => {
                    Ok(Err::<(Adt12, List<Adt18>), Bytes>(v4))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<(Adt12, List<Adt18>), Bytes>(v2))
        }
    }
}

pub fn f259(v0: Str, v1: List<Adt18>) -> Bytes {
    f7(List::cons(f8(Str::lit("expected ")), List::cons({
        let a1798 = v0;
        utf8_encode(a1798)
    }, List::cons(f8(Str::lit(" but found ")), List::cons(f352(v1), List::cons(f8(Str::lit(" (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
}

pub fn f260(v0: List<Adt18>) -> Result<Adt21, Bytes> {
    let m1799 = v0.clone();
    match m1799.uncons() {
        None => {
            Err::<Adt21, Bytes>(f259(Str::lit("an identifier"), v0))
        }
        Some((v1, v2)) => {
            let a1800 = f345(v1.clone());
            let a1801 = 0u64;
            if nat_eq(a1800, a1801) {
                Ok::<Adt21, Bytes>(Adt21::C0(f353(f346(v1)), v2))
            } else {
                Err::<Adt21, Bytes>(f259(Str::lit("an identifier"), v0))
            }
        }
    }
}

pub fn f261(v0: List<Adt18>, v1: Str) -> Result<List<Adt18>, Bytes> {
    if f254(v0.clone(), v1.clone()) {
        Ok::<List<Adt18>, Bytes>(f256(v0))
    } else {
        Err::<List<Adt18>, Bytes>(f259(v1, v0))
    }
}

pub fn f262(v0: Adt21) -> List<Adt18> {
    let m1802 = v0;
    match m1802 {
        Adt21::C0(_, v2) => {
            v2
        }
    }
}

pub fn f263(p1810: u64, p1811: List<Adt18>, p1812: List<Bytes>) -> Result<(List<Bytes>, List<Adt18>), Bytes> {
    let m1809 = (p1810, (p1811, p1812));
    {
        let mut m1809 = m1809;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1809;
                let m1803 = v0;
                if m1803 == 0 {
                    Err::<(u64, (List<Adt18>, List<Bytes>)), Result<(List<Bytes>, List<Adt18>), Bytes>>(Err::<(List<Bytes>, List<Adt18>), Bytes>(f8(Str::lit("an enumeration has more values than the input has tokens"))))
                } else {
                    let v3 = m1803 - 1;
                    let m1804 = v1.clone();
                    match m1804.uncons() {
                        None => {
                            Err::<(u64, (List<Adt18>, List<Bytes>)), Result<(List<Bytes>, List<Adt18>), Bytes>>(Err::<(List<Bytes>, List<Adt18>), Bytes>(f259(Str::lit("a string"), v1)))
                        }
                        Some((v4, v5)) => {
                            let a1805 = f345(v4.clone());
                            let a1806 = 3u64;
                            if !nat_eq(a1805, a1806) {
                                Err::<(u64, (List<Adt18>, List<Bytes>)), Result<(List<Bytes>, List<Adt18>), Bytes>>(Err::<(List<Bytes>, List<Adt18>), Bytes>(f259(Str::lit("a string"), v1)))
                            } else {
                                if f254(v5.clone(), Str::lit(",")) {
                                    if f254(f256(v5.clone()), Str::lit("}")) {
                                        let m1807 = f261(f256(f256(v5)), Str::lit(";"));
                                        match m1807 {
                                            Ok(v7) => {
                                                Err::<(u64, (List<Adt18>, List<Bytes>)), Result<(List<Bytes>, List<Adt18>), Bytes>>(Ok::<(List<Bytes>, List<Adt18>), Bytes>((f44(List::cons(f346(v4), v2)), v7)))
                                            }
                                            Err(v6) => {
                                                Err::<(u64, (List<Adt18>, List<Bytes>)), Result<(List<Bytes>, List<Adt18>), Bytes>>(Err::<(List<Bytes>, List<Adt18>), Bytes>(v6))
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (List<Adt18>, List<Bytes>)), Result<(List<Bytes>, List<Adt18>), Bytes>>((v3, (f256(v5), List::cons(f346(v4), v2))))
                                    }
                                } else {
                                    if f254(v5.clone(), Str::lit("}")) {
                                        let m1808 = f261(f256(v5), Str::lit(";"));
                                        match m1808 {
                                            Ok(v9) => {
                                                Err::<(u64, (List<Adt18>, List<Bytes>)), Result<(List<Bytes>, List<Adt18>), Bytes>>(Ok::<(List<Bytes>, List<Adt18>), Bytes>((f44(List::cons(f346(v4), v2)), v9)))
                                            }
                                            Err(v8) => {
                                                Err::<(u64, (List<Adt18>, List<Bytes>)), Result<(List<Bytes>, List<Adt18>), Bytes>>(Err::<(List<Bytes>, List<Adt18>), Bytes>(v8))
                                            }
                                        }
                                    } else {
                                        Err::<(u64, (List<Adt18>, List<Bytes>)), Result<(List<Bytes>, List<Adt18>), Bytes>>(Err::<(List<Bytes>, List<Adt18>), Bytes>(f259(Str::lit(", or }"), v5)))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1809 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f264(v0: Adt21) -> Bytes {
    let m1813 = v0;
    match m1813 {
        Adt21::C0(v1, _) => {
            v1
        }
    }
}

pub fn f265(v0: Bytes) -> Adt14 {
    Adt14::C0(0u64, v0, List::<Bytes>::nil(), Bytes::lit(&[]), Bytes::lit(&[]))
}

pub fn f266(v0: u64, v1: List<Adt18>, v2: u64) -> R<Result<Adt22, Bytes>> {
    let m1814 = v0;
    if m1814 == 0 {
        Ok(Err::<Adt22, Bytes>(f8(Str::lit("a type nests deeper than the input is long"))))
    } else {
        let v3 = m1814 - 1;
        let a1815 = v2;
        let a1816 = 0u64;
        if nat_eq(a1815, a1816) {
            let m1817 = f183(v1)?;
            match m1817 {
                Ok(v5) => {
                    if f254(v5.clone(), Str::lit("(")) {
                        let m1818 = f266(v3, f256(v5), 1u64)?;
                        match m1818 {
                            Ok(v7) => {
                                if f254(f267(v7.clone()), Str::lit("?")) {
                                    Ok(Ok::<Adt22, Bytes>(Adt22::C0(Adt14::C0(0u64, Bytes::lit(&[]), f203(f268(v7.clone())), f8(Str::lit("a nullable union type is not translated yet, because the typedef identifier names only the union (Web IDL 2.13.32)")), Bytes::lit(&[])), f256(f267(v7)))))
                                } else {
                                    Ok(Ok::<Adt22, Bytes>(Adt22::C0(f268(v7.clone()), f267(v7))))
                                }
                            }
                            Err(v6) => {
                                Ok(Err::<Adt22, Bytes>(v6))
                            }
                        }
                    } else {
                        let m1819 = v5.clone();
                        match m1819.uncons() {
                            None => {
                                Ok(Err::<Adt22, Bytes>(f259(Str::lit("a type"), v5)))
                            }
                            Some((v8, v9)) => {
                                let v10: Bytes = f346(v8.clone());
                                let a1820 = f345(v8);
                                let a1821 = 0u64;
                                if !nat_eq(a1820, a1821) {
                                    Ok(Err::<Adt22, Bytes>(f259(Str::lit("a type"), v5)))
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
                                        let m1822 = f261(v9, Str::lit("<"));
                                        match m1822 {
                                            Ok(v12) => {
                                                let m1823 = f266(v3, v12, 0u64)?;
                                                match m1823 {
                                                    Ok(v14) => {
                                                        let m1824 = f261(f267(v14.clone()), Str::lit(">"));
                                                        match m1824 {
                                                            Ok(v16) => {
                                                                Ok(f354(f355(Str::lit("List"), f268(v14)), v16))
                                                            }
                                                            Err(v15) => {
                                                                Ok(Err::<Adt22, Bytes>(v15))
                                                            }
                                                        }
                                                    }
                                                    Err(v13) => {
                                                        Ok(Err::<Adt22, Bytes>(v13))
                                                    }
                                                }
                                            }
                                            Err(v11) => {
                                                Ok(Err::<Adt22, Bytes>(v11))
                                            }
                                        }
                                    } else {
                                        if f35(v10.clone(), f8(Str::lit("async_sequence"))) {
                                            let m1825 = f261(v9, Str::lit("<"));
                                            match m1825 {
                                                Ok(v18) => {
                                                    let m1826 = f266(v3, v18, 0u64)?;
                                                    match m1826 {
                                                        Ok(v20) => {
                                                            let m1827 = f261(f267(v20), Str::lit(">"));
                                                            match m1827 {
                                                                Ok(v22) => {
                                                                    Ok(f354(f356(f8(Str::lit("async_sequence types are not translated, because an async sequence is produced over time and is not a value (Web IDL 2.13.29)"))), v22))
                                                                }
                                                                Err(v21) => {
                                                                    Ok(Err::<Adt22, Bytes>(v21))
                                                                }
                                                            }
                                                        }
                                                        Err(v19) => {
                                                            Ok(Err::<Adt22, Bytes>(v19))
                                                        }
                                                    }
                                                }
                                                Err(v17) => {
                                                    Ok(Err::<Adt22, Bytes>(v17))
                                                }
                                            }
                                        } else {
                                            if f35(v10.clone(), f8(Str::lit("Promise"))) {
                                                let m1828 = f261(v9, Str::lit("<"));
                                                match m1828 {
                                                    Ok(v24) => {
                                                        let m1829 = f266(v3, v24, 0u64)?;
                                                        match m1829 {
                                                            Ok(v26) => {
                                                                let m1830 = f261(f267(v26), Str::lit(">"));
                                                                match m1830 {
                                                                    Ok(v28) => {
                                                                        Ok(f354(f356(f8(Str::lit("Promise types are not translated, because a promise is a deferred result, not a value (Web IDL 2.13.31)"))), v28))
                                                                    }
                                                                    Err(v27) => {
                                                                        Ok(Err::<Adt22, Bytes>(v27))
                                                                    }
                                                                }
                                                            }
                                                            Err(v25) => {
                                                                Ok(Err::<Adt22, Bytes>(v25))
                                                            }
                                                        }
                                                    }
                                                    Err(v23) => {
                                                        Ok(Err::<Adt22, Bytes>(v23))
                                                    }
                                                }
                                            } else {
                                                if f35(v10.clone(), f8(Str::lit("record"))) {
                                                    let m1831 = f261(v9, Str::lit("<"));
                                                    match m1831 {
                                                        Ok(v30) => {
                                                            let m1832 = v30.clone();
                                                            match m1832.uncons() {
                                                                None => {
                                                                    Ok(Err::<Adt22, Bytes>(f259(Str::lit("a string type"), v30)))
                                                                }
                                                                Some((v31, v32)) => {
                                                                    if if f35(f346(v31.clone()), f8(Str::lit("DOMString"))) {
                                                                        true
                                                                    } else {
                                                                        if f35(f346(v31.clone()), f8(Str::lit("USVString"))) {
                                                                            true
                                                                        } else {
                                                                            f35(f346(v31.clone()), f8(Str::lit("ByteString")))
                                                                        }
                                                                    } {
                                                                        let m1833 = f261(v32, Str::lit(","));
                                                                        match m1833 {
                                                                            Ok(v34) => {
                                                                                let m1834 = f266(v3, v34, 0u64)?;
                                                                                match m1834 {
                                                                                    Ok(v36) => {
                                                                                        let m1835 = f261(f267(v36.clone()), Str::lit(">"));
                                                                                        match m1835 {
                                                                                            Ok(v38) => {
                                                                                                let m1836 = f357(f346(v31));
                                                                                                match m1836 {
                                                                                                    None => {
                                                                                                        Ok(Err::<Adt22, Bytes>(f259(Str::lit("a string type"), v30)))
                                                                                                    }
                                                                                                    Some(v39) => {
                                                                                                        let v40: Adt14 = f358(f268(v36));
                                                                                                        Ok(f354(Adt14::C0(0u64, f7(List::cons(f8(Str::lit("(List (")), List::cons(f202(v39), List::cons(f8(Str::lit(" * ")), List::cons(f202(v40.clone()), List::cons(f8(Str::lit("))")), List::<Bytes>::nil())))))), f203(v40.clone()), f200(v40), Bytes::lit(&[])), v38))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            Err(v37) => {
                                                                                                Ok(Err::<Adt22, Bytes>(v37))
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    Err(v35) => {
                                                                                        Ok(Err::<Adt22, Bytes>(v35))
                                                                                    }
                                                                                }
                                                                            }
                                                                            Err(v33) => {
                                                                                Ok(Err::<Adt22, Bytes>(v33))
                                                                            }
                                                                        }
                                                                    } else {
                                                                        Ok(Err::<Adt22, Bytes>(f259(Str::lit("DOMString, USVString or ByteString"), v30)))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Err(v29) => {
                                                            Ok(Err::<Adt22, Bytes>(v29))
                                                        }
                                                    }
                                                } else {
                                                    if f35(v10.clone(), f8(Str::lit("unsigned"))) {
                                                        if f254(v9.clone(), Str::lit("short")) {
                                                            Ok(f354(f265(f8(Str::lit("Nat"))), f256(v9)))
                                                        } else {
                                                            if f254(v9.clone(), Str::lit("long")) {
                                                                let v41: List<Adt18> = f256(v9);
                                                                Ok(f354(f265(f8(Str::lit("Nat"))), if f254(v41.clone(), Str::lit("long")) {
                                                                    f256(v41)
                                                                } else {
                                                                    v41
                                                                }))
                                                            } else {
                                                                Ok(Err::<Adt22, Bytes>(f259(Str::lit("short or long"), v9)))
                                                            }
                                                        }
                                                    } else {
                                                        if f35(v10.clone(), f8(Str::lit("short"))) {
                                                            Ok(f354(f265(f8(Str::lit("Int"))), v9))
                                                        } else {
                                                            if f35(v10.clone(), f8(Str::lit("long"))) {
                                                                Ok(f354(f265(f8(Str::lit("Int"))), if f254(v9.clone(), Str::lit("long")) {
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
                                                                        Ok(f354(f356(f8(Str::lit("unrestricted float and unrestricted double are not translated, because its value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.13 and 2.13.15)"))), f256(v9)))
                                                                    } else {
                                                                        Ok(Err::<Adt22, Bytes>(f259(Str::lit("float or double"), v9)))
                                                                    }
                                                                } else {
                                                                    let m1837 = f357(v10.clone());
                                                                    match m1837 {
                                                                        None => {
                                                                            let v42: Bytes = f353(v10);
                                                                            Ok(f354(Adt14::C0(0u64, f152(v42.clone())?, List::cons(v42.clone(), List::<Bytes>::nil()), Bytes::lit(&[]), v42), v9))
                                                                        }
                                                                        Some(v43) => {
                                                                            Ok(f354(v43, v9))
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
                    Ok(Err::<Adt22, Bytes>(v4))
                }
            }
        } else {
            let m1838 = f266(v3, v1, 0u64)?;
            match m1838 {
                Ok(v45) => {
                    let v46: Adt14 = f268(v45.clone());
                    let v48: Adt14 = {
                        let a1839 = f201(v46.clone());
                        let a1840 = 0u64;
                        if nat_eq(a1839, a1840) {
                            if f35(f200(v46.clone()), Bytes::lit(&[])) {
                                if f35(f359(v46.clone()), Bytes::lit(&[])) {
                                    Adt14::C0(1u64, Bytes::lit(&[]), f203(v46), f8(Str::lit("a union member type that is not a named definition gives its constructor no name, so the union is not translated yet (Web IDL 2.13.32)")), Bytes::lit(&[]))
                                } else {
                                    let m1841 = f283(false, f359(v46.clone()))?;
                                    match m1841 {
                                        None => {
                                            Adt14::C0(1u64, Bytes::lit(&[]), f203(v46.clone()), f7(List::cons(f8(Str::lit("the union member ")), List::cons(f359(v46), List::cons(f8(Str::lit(" has no constructor name")), List::<Bytes>::nil())))), Bytes::lit(&[]))
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
                        let m1842 = f266(v3, f256(f267(v45)), 2u64)?;
                        match m1842 {
                            Ok(v50) => {
                                Ok(Ok::<Adt22, Bytes>(Adt22::C0(Adt14::C0(1u64, if f35(f202(v48.clone()), Bytes::lit(&[])) {
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
                                Ok(Err::<Adt22, Bytes>(v49))
                            }
                        }
                    } else {
                        let a1843 = v2;
                        let a1844 = 2u64;
                        if if nat_eq(a1843, a1844) {
                            f254(f267(v45.clone()), Str::lit(")"))
                        } else {
                            false
                        } {
                            Ok(Ok::<Adt22, Bytes>(Adt22::C0(v48, f256(f267(v45)))))
                        } else {
                            Ok(Err::<Adt22, Bytes>({
                                let a1845 = v2;
                                let a1846 = 1u64;
                                if nat_eq(a1845, a1846) {
                                    f259(Str::lit("or"), f267(v45))
                                } else {
                                    f259(Str::lit("or or )"), f267(v45))
                                }
                            }))
                        }
                    }
                }
                Err(v44) => {
                    Ok(Err::<Adt22, Bytes>(v44))
                }
            }
        }
    }
}

pub fn f267(v0: Adt22) -> List<Adt18> {
    let m1847 = v0;
    match m1847 {
        Adt22::C0(_, v2) => {
            v2
        }
    }
}

pub fn f268(v0: Adt22) -> Adt14 {
    let m1848 = v0;
    match m1848 {
        Adt22::C0(v1, _) => {
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
    let m1849 = v0;
    match m1849.uncons() {
        None => {
            Ok(List::<Adt13>::nil())
        }
        Some((v1, v2)) => {
            f360(v1, f276(v2)?)
        }
    }
}

pub fn f277(v0: List<Adt13>, v1: List<Adt13>) -> List<Adt13> {
    let m1850 = v0;
    match m1850.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f277(v3, v1))
        }
    }
}

pub fn f278(v0: Adt12) -> List<Adt13> {
    let m1851 = v0;
    match m1851 {
        Adt12::C0(_, _, _, v4, _, _) => {
            v4
        }
    }
}

pub fn f279(v0: List<Adt12>, v1: u64, v2: Bytes) -> List<Adt13> {
    let m1852 = v0;
    match m1852.uncons() {
        None => {
            List::<Adt13>::nil()
        }
        Some((v3, v4)) => {
            let a1853 = f110(v3.clone());
            let a1854 = v1;
            if if nat_eq(a1853, a1854) {
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
    let m1855 = v0;
    match m1855.uncons() {
        None => {
            List::<Adt13>::nil()
        }
        Some((v3, v4)) => {
            let a1856 = f110(v3.clone());
            let a1857 = 8u64;
            if if nat_eq(a1856, a1857) {
                f35(f112(v3.clone()), v2.clone())
            } else {
                false
            } {
                f277(f361(v1.clone(), f123(v3)), f280(v4, v1, v2))
            } else {
                f280(v4, v1, v2)
            }
        }
    }
}

pub fn f281(v0: Adt13) -> Adt14 {
    let m1858 = v0;
    match m1858 {
        Adt13::C0(_, v2, _) => {
            v2
        }
    }
}

pub fn f282(v0: Adt13) -> Bytes {
    let m1859 = v0;
    match m1859 {
        Adt13::C0(v1, _, _) => {
            v1
        }
    }
}

pub fn f283(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f362(v1)
    } else {
        f286(v1)
    }
}

pub fn f284(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1860 = v1;
    match m1860.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m1861 = f283(v0, v2)?;
            match m1861 {
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
    let m1862 = v0;
    match m1862 {
        Adt13::C0(_, _, v3) => {
            v3
        }
    }
}

pub fn f286(v0: Bytes) -> R<Option<Bytes>> {
    if f363({
        let a1863 = v0.clone();
        length_bytes(a1863)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f364(v0)?;
        if f365(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m1866 = {
                let a1864 = v1.clone();
                let a1865 = 0u64;
                index_bytes(a1864, a1865)
            };
            match m1866 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f306(f41(v2)?) {
                        Ok(Some({
                            let a1867 = f8(Str::lit("n"));
                            let a1868 = v1;
                            append_bytes(a1867, a1868)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f366({
            let a1869 = v0.clone();
            length_bytes(a1869)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f287(v0: bool, v1: Adt17) -> R<Option<Bytes>> {
    let m1870 = v1;
    match m1870 {
        Adt17::C0(_) => {
            Ok(None::<Bytes>)
        }
        Adt17::C1(v7, v8) => {
            Ok(Some({
                let a1881 = if f302(v7, {
                    let a1879 = 0u64;
                    let a1880 = v8;
                    nat_lt(a1879, a1880)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a1882 = f174(false, v8)?;
                append_bytes(a1881, a1882)
            }))
        }
        Adt17::C2(v3, v4, v5) => {
            let m1873 = f362({
                let a1871 = f8(Str::lit("n"));
                let a1872 = f175(false, v4, v5)?;
                append_bytes(a1871, a1872)
            })?;
            match m1873 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a1877 = f8(Str::lit("nMinus"));
                        let a1878 = f32(v6.clone(), 1u64, {
                            let a1875 = {
                                let a1874 = v6;
                                length_bytes(a1874)
                            };
                            let a1876 = 1u64;
                            nat_sub(a1875, a1876)
                        });
                        append_bytes(a1877, a1878)
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
    let m1885 = {
        let a1883 = v0.clone();
        let a1884 = v1;
        index_bytes(a1883, a1884)
    };
    match m1885 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f41(v2)?;
            let a1886 = v3;
            let a1887 = 128u64;
            if nat_lt(a1886, a1887) {
                Ok(Some((v3, 1u64)))
            } else {
                if f302({
                    let a1888 = 194u64;
                    let a1889 = v3;
                    nat_le(a1888, a1889)
                }, {
                    let a1890 = v3;
                    let a1891 = 223u64;
                    nat_le(a1890, a1891)
                }) {
                    let m1894 = f367(v0, {
                        let a1892 = v1;
                        let a1893 = 1u64;
                        nat_add(a1892, a1893)?
                    })?;
                    match m1894 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f368({
                                let a1899 = {
                                    let a1897 = {
                                        let a1895 = v3;
                                        let a1896 = 192u64;
                                        nat_sub(a1895, a1896)
                                    };
                                    let a1898 = 64u64;
                                    nat_mul(a1897, a1898)?
                                };
                                let a1900 = v4;
                                nat_add(a1899, a1900)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f302({
                        let a1901 = 224u64;
                        let a1902 = v3;
                        nat_le(a1901, a1902)
                    }, {
                        let a1903 = v3;
                        let a1904 = 239u64;
                        nat_le(a1903, a1904)
                    }) {
                        let m1907 = f367(v0.clone(), {
                            let a1905 = v1;
                            let a1906 = 1u64;
                            nat_add(a1905, a1906)?
                        })?;
                        match m1907 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m1910 = f367(v0, {
                                    let a1908 = v1;
                                    let a1909 = 2u64;
                                    nat_add(a1908, a1909)?
                                })?;
                                match m1910 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f368({
                                            let a1919 = {
                                                let a1913 = {
                                                    let a1911 = v3;
                                                    let a1912 = 224u64;
                                                    nat_sub(a1911, a1912)
                                                };
                                                let a1914 = 4096u64;
                                                nat_mul(a1913, a1914)?
                                            };
                                            let a1920 = {
                                                let a1917 = {
                                                    let a1915 = v5;
                                                    let a1916 = 64u64;
                                                    nat_mul(a1915, a1916)?
                                                };
                                                let a1918 = v6;
                                                nat_add(a1917, a1918)?
                                            };
                                            nat_add(a1919, a1920)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f302({
                            let a1921 = 240u64;
                            let a1922 = v3;
                            nat_le(a1921, a1922)
                        }, {
                            let a1923 = v3;
                            let a1924 = 244u64;
                            nat_le(a1923, a1924)
                        }) {
                            let m1927 = f367(v0.clone(), {
                                let a1925 = v1;
                                let a1926 = 1u64;
                                nat_add(a1925, a1926)?
                            })?;
                            match m1927 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m1930 = f367(v0.clone(), {
                                        let a1928 = v1;
                                        let a1929 = 2u64;
                                        nat_add(a1928, a1929)?
                                    })?;
                                    match m1930 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m1933 = f367(v0, {
                                                let a1931 = v1;
                                                let a1932 = 3u64;
                                                nat_add(a1931, a1932)?
                                            })?;
                                            match m1933 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f368({
                                                        let a1946 = {
                                                            let a1936 = {
                                                                let a1934 = v3;
                                                                let a1935 = 240u64;
                                                                nat_sub(a1934, a1935)
                                                            };
                                                            let a1937 = 262144u64;
                                                            nat_mul(a1936, a1937)?
                                                        };
                                                        let a1947 = {
                                                            let a1944 = {
                                                                let a1938 = v7;
                                                                let a1939 = 4096u64;
                                                                nat_mul(a1938, a1939)?
                                                            };
                                                            let a1945 = {
                                                                let a1942 = {
                                                                    let a1940 = v8;
                                                                    let a1941 = 64u64;
                                                                    nat_mul(a1940, a1941)?
                                                                };
                                                                let a1943 = v9;
                                                                nat_add(a1942, a1943)?
                                                            };
                                                            nat_add(a1944, a1945)?
                                                        };
                                                        nat_add(a1946, a1947)?
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
    f208(v0, f369())
}

pub fn f290(v0: u64, v1: Bytes, p1958: u64, p1959: u64, p1960: bool) -> R<bool> {
    let m1957 = (p1958, (p1959, p1960));
    {
        let mut m1957 = m1957;
        loop {
            let step = {
                let (v2, (v3, v4)) = m1957;
                let m1948 = v3;
                if m1948 == 0 {
                    Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                } else {
                    let v5 = m1948 - 1;
                    let v6: u64 = f370(v1.clone(), v2)?;
                    if v4 {
                        let a1949 = v0;
                        let a1950 = v6;
                        if nat_lt(a1949, a1950) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a1951 = v2;
                                let a1952 = 1u64;
                                nat_add(a1951, a1952)?
                            }, (v5, false)))
                        }
                    } else {
                        let a1953 = v0;
                        let a1954 = v6;
                        if nat_le(a1953, a1954) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(true))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a1955 = v2;
                                let a1956 = 1u64;
                                nat_add(a1955, a1956)?
                            }, (v5, true)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1957 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f291(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    if f77(v0.clone(), v1, v2) {
        Ok(None::<u64>)
    } else {
        let a1962 = v2;
        let a1963 = {
            let a1961 = v0;
            length_bytes(a1961)
        };
        if nat_lt(a1962, a1963) {
            Ok(Some({
                let a1964 = v2;
                let a1965 = 1u64;
                nat_add(a1964, a1965)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f292(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f130(f85(v0, v1)?)? {
        Ok(Some({
            let a1966 = v1;
            let a1967 = 1u64;
            nat_add(a1966, a1967)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f293(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let m1968 = f288(v0, v2)?;
    match m1968 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            let a1969 = v2;
            let a1970 = v1;
            if if nat_eq(a1969, a1970) {
                f371({
                    let (h1971, _) = v3;
                    h1971
                })?
            } else {
                f372({
                    let (h1972, _) = v3;
                    h1972
                })?
            } {
                Ok(Some({
                    let a1974 = v2;
                    let a1975 = {
                        let (_, h1973) = v3;
                        h1973
                    };
                    nat_add(a1974, a1975)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f294(v0: Bytes, v1: bool, v2: u64) -> R<(u64, u64)> {
    let (h1976, _) = f316(Fn7::F373(v1, v0.clone()), {
        let a1978 = {
            let a1977 = v0;
            length_bytes(a1977)
        };
        let a1979 = 1u64;
        nat_add(a1978, a1979)?
    }, (v2, 0u64))?;
    Ok(h1976)
}

pub fn f295(v0: u64) -> R<Bytes> {
    let a1980 = v0;
    let a1981 = 128u64;
    if nat_lt(a1980, a1981) {
        Ok(f374(f375(), v0))
    } else {
        let a1982 = v0;
        let a1983 = 2048u64;
        if nat_lt(a1982, a1983) {
            let a1991 = f374(f376(), {
                let a1989 = {
                    let a1987 = {
                        let a1984 = v0;
                        let a1985 = 64u64;
                        let a1986 = 0u64;
                        nat_quot(a1984, a1985, a1986)
                    };
                    let a1988 = 2u64;
                    nat_sub(a1987, a1988)
                };
                let a1990 = 2u64;
                nat_mul(a1989, a1990)?
            });
            let a1992 = f377(v0)?;
            Ok(append_bytes(a1991, a1992))
        } else {
            let a1993 = v0;
            let a1994 = 65536u64;
            if nat_lt(a1993, a1994) {
                let a2005 = f374(f378(), {
                    let a1998 = {
                        let a1995 = v0;
                        let a1996 = 4096u64;
                        let a1997 = 0u64;
                        nat_quot(a1995, a1996, a1997)
                    };
                    let a1999 = 3u64;
                    nat_mul(a1998, a1999)?
                });
                let a2006 = {
                    let a2003 = f377({
                        let a2000 = v0;
                        let a2001 = 64u64;
                        let a2002 = 0u64;
                        nat_quot(a2000, a2001, a2002)
                    })?;
                    let a2004 = f377(v0)?;
                    append_bytes(a2003, a2004)
                };
                Ok(append_bytes(a2005, a2006))
            } else {
                let a2020 = f374(Bytes::lit(&[0xf0, 0xf1, 0xf2, 0xf3, 0xf4]), {
                    let a2007 = v0;
                    let a2008 = 262144u64;
                    let a2009 = 0u64;
                    nat_quot(a2007, a2008, a2009)
                });
                let a2021 = {
                    let a2018 = f377({
                        let a2010 = v0;
                        let a2011 = 4096u64;
                        let a2012 = 0u64;
                        nat_quot(a2010, a2011, a2012)
                    })?;
                    let a2019 = {
                        let a2016 = f377({
                            let a2013 = v0;
                            let a2014 = 64u64;
                            let a2015 = 0u64;
                            nat_quot(a2013, a2014, a2015)
                        })?;
                        let a2017 = f377(v0)?;
                        append_bytes(a2016, a2017)
                    };
                    append_bytes(a2018, a2019)
                };
                Ok(append_bytes(a2020, a2021))
            }
        }
    }
}

pub fn f296(v0: bool, v1: u64, v2: Bytes, v3: u64) -> R<Option<u64>> {
    f379(v2, v0, v1, v3)
}

pub fn f297(v0: Bytes) -> Result<Option<Adt20>, Bytes> {
    Err::<Option<Adt20>, Bytes>(v0)
}

pub fn f298(p2025: Bytes, p2026: u64) -> R<Result<Bytes, Bytes>> {
    let m2024 = (p2025, p2026);
    {
        let mut m2024 = m2024;
        loop {
            let step = {
                let (v0, v1) = m2024;
                let m2022 = v1;
                if m2022 == 0 {
                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f380(f8(Str::lit("the built-in datatype hierarchy is deeper than XSD 1.1 Part 2 section 3 defines")))))
                } else {
                    let v2 = m2022 - 1;
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
                                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f381())))
                                } else {
                                    if f35(v0.clone(), f8(Str::lit("boolean"))) {
                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Bool")))))
                                    } else {
                                        if f172(v0.clone(), List::cons(f8(Str::lit("float")), List::cons(f8(Str::lit("double")), List::<Bytes>::nil()))) {
                                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f380(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because its value space holds positive and negative zero, the infinities and NaN, which no exact LexLean number type holds (XSD 1.1 Part 2, sections 3.3.4 and 3.3.5)")), List::<Bytes>::nil())))))))
                                        } else {
                                            if f172(v0.clone(), List::cons(f8(Str::lit("anySimpleType")), List::cons(f8(Str::lit("anyAtomicType")), List::<Bytes>::nil()))) {
                                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f380(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because it is the union of every simple value space and has no one LexLean type (XSD 1.1 Part 2, sections 3.2.1 and 3.2.2)")), List::<Bytes>::nil())))))))
                                            } else {
                                                let m2023 = f382(v0.clone())?;
                                                match m2023 {
                                                    None => {
                                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f380(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not a built-in datatype of XSD 1.1 Part 2")), List::<Bytes>::nil())))))))
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
                Ok(next) => { m2024 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f299(v0: Bytes) -> Result<List<Adt17>, Bytes> {
    Err::<List<Adt17>, Bytes>(v0)
}

pub fn f300(v0: Option<Adt17>) -> Option<Bytes> {
    let m2027 = v0;
    match m2027 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m2028 = v1;
            match m2028 {
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
        let a2029 = v0;
        let a2030 = 95u64;
        nat_eq(a2029, a2030)
    }, f304({
        let a2031 = v0;
        let a2032 = 45u64;
        nat_eq(a2031, a2032)
    }, f304({
        let a2033 = v0;
        let a2034 = 32u64;
        nat_eq(a2033, a2034)
    }, f304({
        let a2035 = v0;
        let a2036 = 46u64;
        nat_eq(a2035, a2036)
    }, {
        let a2037 = v0;
        let a2038 = 47u64;
        nat_eq(a2037, a2038)
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
        let a2039 = 97u64;
        let a2040 = v0;
        nat_le(a2039, a2040)
    }, {
        let a2041 = v0;
        let a2042 = 122u64;
        nat_le(a2041, a2042)
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
        let a2043 = 65u64;
        let a2044 = v0;
        nat_le(a2043, a2044)
    }, {
        let a2045 = v0;
        let a2046 = 90u64;
        nat_le(a2045, a2046)
    })
}

pub fn f306(v0: u64) -> bool {
    f302({
        let a2047 = 48u64;
        let a2048 = v0;
        nat_le(a2047, a2048)
    }, {
        let a2049 = v0;
        let a2050 = 57u64;
        nat_le(a2049, a2050)
    })
}

pub fn f307(v0: u64) -> bool {
    let a2051 = v0;
    let a2052 = 9u64;
    if if if nat_eq(a2051, a2052) {
        true
    } else {
        let a2053 = v0;
        let a2054 = 10u64;
        nat_eq(a2053, a2054)
    } {
        true
    } else {
        let a2055 = v0;
        let a2056 = 13u64;
        if nat_eq(a2055, a2056) {
            true
        } else {
            let a2057 = 32u64;
            let a2058 = v0;
            if nat_le(a2057, a2058) {
                let a2059 = v0;
                let a2060 = 126u64;
                nat_le(a2059, a2060)
            } else {
                false
            }
        }
    } {
        true
    } else {
        let a2061 = v0;
        let a2062 = 133u64;
        if if nat_eq(a2061, a2062) {
            true
        } else {
            let a2063 = 160u64;
            let a2064 = v0;
            if nat_le(a2063, a2064) {
                let a2065 = v0;
                let a2066 = 55295u64;
                nat_le(a2065, a2066)
            } else {
                false
            }
        } {
            true
        } else {
            let a2067 = 57344u64;
            let a2068 = v0;
            if if nat_le(a2067, a2068) {
                let a2069 = v0;
                let a2070 = 65533u64;
                nat_le(a2069, a2070)
            } else {
                false
            } {
                true
            } else {
                let a2071 = 65536u64;
                let a2072 = v0;
                if nat_le(a2071, a2072) {
                    let a2073 = v0;
                    let a2074 = 1114111u64;
                    nat_le(a2073, a2074)
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
        let a2075 = f100(v0, v2)?;
        let a2076 = 35u64;
        Ok(nat_eq(a2075, a2076))
    }
}

pub fn f309(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f383(v0, v1)
}

pub fn f310(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2077 = f100(v0, v1)?;
    let a2078 = 32u64;
    if nat_eq(a2077, a2078) {
        Ok(Some({
            let a2079 = v1;
            let a2080 = 1u64;
            nat_add(a2079, a2080)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f311(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f157(f100(v0, v1)?) {
        Ok(Some({
            let a2081 = v1;
            let a2082 = 1u64;
            nat_add(a2081, a2082)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f312(v0: u64) -> bool {
    let a2083 = v0;
    let a2084 = 44u64;
    if if if nat_eq(a2083, a2084) {
        true
    } else {
        let a2085 = v0;
        let a2086 = 91u64;
        nat_eq(a2085, a2086)
    } {
        true
    } else {
        let a2087 = v0;
        let a2088 = 93u64;
        if nat_eq(a2087, a2088) {
            true
        } else {
            let a2089 = v0;
            let a2090 = 123u64;
            nat_eq(a2089, a2090)
        }
    } {
        true
    } else {
        let a2091 = v0;
        let a2092 = 125u64;
        nat_eq(a2091, a2092)
    }
}

pub fn f313(v0: u64) -> u64 {
    let a2093 = v0;
    let a2094 = 45u64;
    if nat_eq(a2093, a2094) {
        1u64
    } else {
        let a2095 = v0;
        let a2096 = 43u64;
        if nat_eq(a2095, a2096) {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f314(v0: u64) -> u64 {
    let a2097 = 49u64;
    let a2098 = v0;
    if if nat_le(a2097, a2098) {
        let a2099 = v0;
        let a2100 = 57u64;
        nat_le(a2099, a2100)
    } else {
        false
    } {
        let a2101 = v0;
        let a2102 = 48u64;
        nat_sub(a2101, a2102)
    } else {
        0u64
    }
}

pub fn f315(v0: Bytes, v1: u64, v2: u64, v3: bool, v4: u64) -> R<Result<(Adt17, u64), Bytes>> {
    let m2103 = f384(v0.clone(), v1, v2)?;
    match m2103 {
        Ok(v6) => {
            let v7: u64 = {
                let (_, h2104) = v6;
                h2104
            };
            let v8: u64 = f385(v0.clone(), v7, v2)?;
            if f386(v0.clone(), v7, v2)? {
                let m2109 = f387(v0.clone(), v7, v2, v3, {
                    let a2106 = {
                        let a2105 = v0;
                        length_bytes(a2105)
                    };
                    let a2107 = 2u64;
                    nat_add(a2106, a2107)?
                }, Bytes::lit(&[]), {
                    let (h2108, _) = v6;
                    h2108
                })?;
                match m2109 {
                    Ok(v10) => {
                        Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C3(f7(List::cons({
                            let (h2110, _) = v10.clone();
                            h2110
                        }, List::cons({
                            let a2111 = v4;
                            let a2112 = 1u64;
                            if nat_eq(a2111, a2112) {
                                Bytes::lit(&[])
                            } else {
                                Bytes::lit(&[0x0a])
                            }
                        }, List::cons({
                            let a2113 = v4;
                            let a2114 = 2u64;
                            if nat_eq(a2113, a2114) {
                                f318({
                                    let (h2115, _) = {
                                        let (_, h2116) = v10.clone();
                                        h2116
                                    };
                                    h2115
                                })
                            } else {
                                Bytes::lit(&[])
                            }
                        }, List::<Bytes>::nil()))))), {
                            let (_, h2117) = {
                                let (_, h2118) = v10;
                                h2118
                            };
                            h2117
                        })))
                    }
                    Err(v9) => {
                        Ok(Err::<(Adt17, u64), Bytes>(v9))
                    }
                }
            } else {
                Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C3({
                    let a2119 = v4;
                    let a2120 = 2u64;
                    if nat_eq(a2119, a2120) {
                        f318({
                            let (h2121, _) = v6;
                            h2121
                        })
                    } else {
                        Bytes::lit(&[])
                    }
                }), {
                    let a2122 = f100(v0, v8)?;
                    let a2123 = 256u64;
                    if nat_eq(a2122, a2123) {
                        v8
                    } else {
                        v7
                    }
                })))
            }
        }
        Err(v5) => {
            Ok(Err::<(Adt17, u64), Bytes>(v5))
        }
    }
}

pub fn f316(v0: Fn7, p2128: u64, p2129: (u64, u64)) -> R<((u64, u64), bool)> {
    let m2127 = (p2128, p2129);
    {
        let mut m2127 = m2127;
        loop {
            let step = {
                let (v1, v2) = m2127;
                let m2124 = v1;
                if m2124 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2124 - 1;
                    let m2126 = {
                        let c2125: Fn7 = v0.clone();
                        c2125.apply(v2)?
                    };
                    match m2126 {
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
                Ok(next) => { m2127 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f317(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f388(v0, v1)
}

pub fn f318(v0: u64) -> Bytes {
    let m2130 = v0;
    if m2130 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2130 - 1;
        let a2131 = Bytes::lit(&[0x0a]);
        let a2132 = f318(v1);
        append_bytes(a2131, a2132)
    }
}

pub fn f319(v0: Bytes, v1: u64) -> R<u64> {
    let (_, h2133) = {
        let (h2134, _) = f316(Fn7::F389(v0.clone()), {
            let a2136 = {
                let a2135 = v0;
                length_bytes(a2135)
            };
            let a2137 = 1u64;
            nat_add(a2136, a2137)?
        }, (v1, 0u64))?;
        h2134
    };
    Ok(h2133)
}

pub fn f320(v0: u64, v1: u64) -> u64 {
    let a2138 = v0;
    let a2139 = v1;
    if nat_lt(a2138, a2139) {
        v1
    } else {
        v0
    }
}

pub fn f321(v0: Bytes, p2202: u64, v2: u64, v3: bool, p2203: u64, p2204: Bytes) -> R<Result<(Bytes, u64), Bytes>> {
    let m2201 = (p2202, (p2203, p2204));
    {
        let mut m2201 = m2201;
        loop {
            let step = {
                let (v1, (v4, v5)) = m2201;
                let m2140 = v4;
                if m2140 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f81(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                } else {
                    let v6 = m2140 - 1;
                    let v7: u64 = f100(v0.clone(), v1)?;
                    let a2141 = v7;
                    let a2142 = 34u64;
                    if nat_eq(a2141, a2142) {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v5, {
                            let a2143 = v1;
                            let a2144 = 1u64;
                            nat_add(a2143, a2144)?
                        }))))
                    } else {
                        let a2145 = v7;
                        let a2146 = 256u64;
                        if nat_eq(a2145, a2146) {
                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f81(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                        } else {
                            let a2147 = v7;
                            let a2148 = 92u64;
                            if nat_eq(a2147, a2148) {
                                let v8: u64 = f100(v0.clone(), {
                                    let a2149 = v1;
                                    let a2150 = 1u64;
                                    nat_add(a2149, a2150)?
                                })?;
                                let m2151 = f390(v8);
                                match m2151 {
                                    None => {
                                        let a2156 = v8;
                                        let a2157 = 10u64;
                                        if nat_eq(a2156, a2157) {
                                            if v3 {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f391(v1)?)))
                                            } else {
                                                let m2160 = f392(v0.clone(), {
                                                    let a2158 = v1;
                                                    let a2159 = 1u64;
                                                    nat_add(a2158, a2159)?
                                                }, v2, v6, 0u64)?;
                                                match m2160 {
                                                    Ok(v11) => {
                                                        Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                            let (_, h2161) = v11;
                                                            h2161
                                                        }, (v6, {
                                                            let a2163 = v5;
                                                            let a2164 = f318({
                                                                let (h2162, _) = v11;
                                                                h2162
                                                            });
                                                            append_bytes(a2163, a2164)
                                                        })))
                                                    }
                                                    Err(v10) => {
                                                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(v10)))
                                                    }
                                                }
                                            }
                                        } else {
                                            let v12: u64 = {
                                                let a2165 = v8;
                                                let a2166 = 120u64;
                                                if nat_eq(a2165, a2166) {
                                                    2u64
                                                } else {
                                                    let a2167 = v8;
                                                    let a2168 = 117u64;
                                                    if nat_eq(a2167, a2168) {
                                                        4u64
                                                    } else {
                                                        let a2169 = v8;
                                                        let a2170 = 85u64;
                                                        if nat_eq(a2169, a2170) {
                                                            8u64
                                                        } else {
                                                            0u64
                                                        }
                                                    }
                                                }
                                            };
                                            let a2171 = v12;
                                            let a2172 = 0u64;
                                            if nat_eq(a2171, a2172) {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f81(Str::lit("an escape must be one of YAML 1.2.2 [42] to [62]"), v1)?)))
                                            } else {
                                                let m2175 = f393(v0.clone(), {
                                                    let a2173 = v1;
                                                    let a2174 = 2u64;
                                                    nat_add(a2173, a2174)?
                                                }, v12, 0u64)?;
                                                match m2175 {
                                                    None => {
                                                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f81(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                    }
                                                    Some(v13) => {
                                                        let a2176 = 1114111u64;
                                                        let a2177 = v13;
                                                        if if nat_lt(a2176, a2177) {
                                                            true
                                                        } else {
                                                            let a2178 = 55296u64;
                                                            let a2179 = v13;
                                                            if nat_le(a2178, a2179) {
                                                                let a2180 = v13;
                                                                let a2181 = 57343u64;
                                                                nat_le(a2180, a2181)
                                                            } else {
                                                                false
                                                            }
                                                        } {
                                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f81(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                        } else {
                                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                                let a2184 = {
                                                                    let a2182 = v1;
                                                                    let a2183 = 2u64;
                                                                    nat_add(a2182, a2183)?
                                                                };
                                                                let a2185 = v12;
                                                                nat_add(a2184, a2185)?
                                                            }, (v6, {
                                                                let a2186 = v5;
                                                                let a2187 = f295(v13)?;
                                                                append_bytes(a2186, a2187)
                                                            })))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Some(v9) => {
                                        Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                            let a2152 = v1;
                                            let a2153 = 2u64;
                                            nat_add(a2152, a2153)?
                                        }, (v6, {
                                            let a2154 = v5;
                                            let a2155 = v9;
                                            append_bytes(a2154, a2155)
                                        })))
                                    }
                                }
                            } else {
                                if if f157(v7) {
                                    true
                                } else {
                                    let a2188 = v7;
                                    let a2189 = 10u64;
                                    nat_eq(a2188, a2189)
                                } {
                                    let v14: u64 = f161(v0.clone(), v1)?;
                                    let a2190 = f100(v0.clone(), v14)?;
                                    let a2191 = 10u64;
                                    if nat_eq(a2190, a2191) {
                                        if v3 {
                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f391(v1)?)))
                                        } else {
                                            let m2192 = f392(v0.clone(), v14, v2, v6, 0u64)?;
                                            match m2192 {
                                                Ok(v16) => {
                                                    Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                        let (_, h2193) = v16;
                                                        h2193
                                                    }, (v6, {
                                                        let a2195 = v5;
                                                        let a2196 = f329({
                                                            let (h2194, _) = v16;
                                                            h2194
                                                        });
                                                        append_bytes(a2195, a2196)
                                                    })))
                                                }
                                                Err(v15) => {
                                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(v15)))
                                                }
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v14, (v6, {
                                            let a2197 = v5;
                                            let a2198 = f236(v0.clone(), v1, v14);
                                            append_bytes(a2197, a2198)
                                        })))
                                    }
                                } else {
                                    let v17: u64 = f394(v0.clone(), v1)?;
                                    Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v17, (v6, {
                                        let a2199 = v5;
                                        let a2200 = f236(v0.clone(), v1, v17);
                                        append_bytes(a2199, a2200)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2201 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f322(v0: Bytes, p2234: u64, v2: u64, v3: bool, p2235: u64, p2236: Bytes) -> R<Result<(Bytes, u64), Bytes>> {
    let m2233 = (p2234, (p2235, p2236));
    {
        let mut m2233 = m2233;
        loop {
            let step = {
                let (v1, (v4, v5)) = m2233;
                let m2205 = v4;
                if m2205 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f81(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                } else {
                    let v6 = m2205 - 1;
                    let v7: u64 = f100(v0.clone(), v1)?;
                    let a2206 = v7;
                    let a2207 = 39u64;
                    if nat_eq(a2206, a2207) {
                        let a2210 = f100(v0.clone(), {
                            let a2208 = v1;
                            let a2209 = 1u64;
                            nat_add(a2208, a2209)?
                        })?;
                        let a2211 = 39u64;
                        if nat_eq(a2210, a2211) {
                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                let a2212 = v1;
                                let a2213 = 2u64;
                                nat_add(a2212, a2213)?
                            }, (v6, {
                                let a2214 = v5;
                                let a2215 = Bytes::lit(&[0x27]);
                                append_bytes(a2214, a2215)
                            })))
                        } else {
                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v5, {
                                let a2216 = v1;
                                let a2217 = 1u64;
                                nat_add(a2216, a2217)?
                            }))))
                        }
                    } else {
                        let a2218 = v7;
                        let a2219 = 256u64;
                        if nat_eq(a2218, a2219) {
                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f81(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                        } else {
                            if if f157(v7) {
                                true
                            } else {
                                let a2220 = v7;
                                let a2221 = 10u64;
                                nat_eq(a2220, a2221)
                            } {
                                let v8: u64 = f161(v0.clone(), v1)?;
                                let a2222 = f100(v0.clone(), v8)?;
                                let a2223 = 10u64;
                                if nat_eq(a2222, a2223) {
                                    if v3 {
                                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f391(v1)?)))
                                    } else {
                                        let m2224 = f392(v0.clone(), v8, v2, v6, 0u64)?;
                                        match m2224 {
                                            Ok(v10) => {
                                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                    let (_, h2225) = v10;
                                                    h2225
                                                }, (v6, {
                                                    let a2227 = v5;
                                                    let a2228 = f329({
                                                        let (h2226, _) = v10;
                                                        h2226
                                                    });
                                                    append_bytes(a2227, a2228)
                                                })))
                                            }
                                            Err(v9) => {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(v9)))
                                            }
                                        }
                                    }
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v8, (v6, {
                                        let a2229 = v5;
                                        let a2230 = f236(v0.clone(), v1, v8);
                                        append_bytes(a2229, a2230)
                                    })))
                                }
                            } else {
                                let v11: u64 = f395(v0.clone(), v1)?;
                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v11, (v6, {
                                    let a2231 = v5;
                                    let a2232 = f236(v0.clone(), v1, v11);
                                    append_bytes(a2231, a2232)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2233 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f323(v0: Bytes, v1: u64, v2: u64) -> R<Result<u64, Bytes>> {
    let v3: (u64, bool) = {
        let (h2237, _) = f396(Fn8::F397(v2, v0.clone()), {
            let a2239 = {
                let a2238 = v0.clone();
                length_bytes(a2238)
            };
            let a2240 = 1u64;
            nat_add(a2239, a2240)?
        }, (v1, false))?;
        h2237
    };
    let (_, h2241) = v3;
    if h2241 {
        if f99(v0, {
            let (h2242, _) = v3;
            h2242
        })? {
            Ok(Err::<u64, Bytes>(f81(Str::lit("a document marker cannot appear inside a flow collection (YAML 1.2.2 [206])"), {
                let (h2243, _) = v3;
                h2243
            })?))
        } else {
            Ok(Err::<u64, Bytes>(f81(Str::lit("a line of a flow collection is indented less than its node (YAML 1.2.2 [80])"), {
                let (h2244, _) = v3;
                h2244
            })?))
        }
    } else {
        Ok(Ok::<u64, Bytes>({
            let (h2245, _) = v3;
            h2245
        }))
    }
}

pub fn f324(v0: Bytes, v1: u64, v2: u64) -> R<Result<(Bytes, u64), Bytes>> {
    let v3: u64 = f100(v0.clone(), v1)?;
    let a2246 = v3;
    let a2247 = 34u64;
    if if nat_eq(a2246, a2247) {
        true
    } else {
        let a2248 = v3;
        let a2249 = 39u64;
        nat_eq(a2248, a2249)
    } {
        f231(v0, v1, v2, false)
    } else {
        let m2250 = f164(v0.clone(), v1)?;
        match m2250 {
            None => {
                let a2251 = v3;
                let a2252 = 91u64;
                if if nat_eq(a2251, a2252) {
                    true
                } else {
                    let a2253 = v3;
                    let a2254 = 123u64;
                    nat_eq(a2253, a2254)
                } {
                    Ok(Err::<(Bytes, u64), Bytes>(f81(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [142])"), v1)?))
                } else {
                    if f233(v0.clone(), v1, true)? {
                        let v5: u64 = f234(v0.clone(), v1, true)?;
                        Ok(Ok::<(Bytes, u64), Bytes>((f236(v0, v1, v5), v5)))
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

pub fn f325(v0: u64) -> bool {
    if f312(v0) {
        true
    } else {
        let a2255 = v0;
        let a2256 = 45u64;
        if if if if nat_eq(a2255, a2256) {
            true
        } else {
            let a2257 = v0;
            let a2258 = 63u64;
            nat_eq(a2257, a2258)
        } {
            true
        } else {
            let a2259 = v0;
            let a2260 = 58u64;
            if nat_eq(a2259, a2260) {
                true
            } else {
                let a2261 = v0;
                let a2262 = 35u64;
                nat_eq(a2261, a2262)
            }
        } {
            true
        } else {
            let a2263 = v0;
            let a2264 = 38u64;
            if if nat_eq(a2263, a2264) {
                true
            } else {
                let a2265 = v0;
                let a2266 = 42u64;
                nat_eq(a2265, a2266)
            } {
                true
            } else {
                let a2267 = v0;
                let a2268 = 33u64;
                if nat_eq(a2267, a2268) {
                    true
                } else {
                    let a2269 = v0;
                    let a2270 = 124u64;
                    nat_eq(a2269, a2270)
                }
            }
        } {
            true
        } else {
            let a2271 = v0;
            let a2272 = 62u64;
            if if if nat_eq(a2271, a2272) {
                true
            } else {
                let a2273 = v0;
                let a2274 = 39u64;
                nat_eq(a2273, a2274)
            } {
                true
            } else {
                let a2275 = v0;
                let a2276 = 34u64;
                if nat_eq(a2275, a2276) {
                    true
                } else {
                    let a2277 = v0;
                    let a2278 = 37u64;
                    nat_eq(a2277, a2278)
                }
            } {
                true
            } else {
                let a2279 = v0;
                let a2280 = 64u64;
                if nat_eq(a2279, a2280) {
                    true
                } else {
                    let a2281 = v0;
                    let a2282 = 96u64;
                    nat_eq(a2281, a2282)
                }
            }
        }
    }
}

pub fn f326(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f398(v1, v0, v2)
}

pub fn f327(v0: u64) -> R<Bytes> {
    f81(Str::lit("a mapping value is not allowed here: a plain scalar holds ': ' (YAML 1.2.2 [130])"), v0)
}

pub fn f328(v0: Bytes, v1: u64) -> R<(u64, u64)> {
    let (h2283, _) = f316(Fn7::F399(v0.clone()), {
        let a2285 = {
            let a2284 = v0;
            length_bytes(a2284)
        };
        let a2286 = 1u64;
        nat_add(a2285, a2286)?
    }, (v1, 0u64))?;
    Ok(h2283)
}

pub fn f329(v0: u64) -> Bytes {
    let a2287 = v0;
    let a2288 = 0u64;
    if nat_eq(a2287, a2288) {
        Bytes::lit(&[0x20])
    } else {
        f318(v0)
    }
}

pub fn f330(v0: Bytes, v1: Str) -> bool {
    f35(v0, f8(v1))
}

pub fn f331(v0: Bytes, v1: u64, v2: u64) -> R<Result<Adt17, Bytes>> {
    let v3: (u64, u64) = f400(v0.clone(), v1, 2u64)?;
    let a2291 = {
        let (h2289, _) = v3;
        h2289
    };
    let a2292 = {
        let a2290 = v0.clone();
        length_bytes(a2290)
    };
    if nat_eq(a2291, a2292) {
        if f401(false, {
            let (_, h2293) = v3;
            h2293
        }) {
            Ok(Ok::<Adt17, Bytes>(Adt17::C1(false, {
                let (_, h2294) = v3;
                h2294
            })))
        } else {
            Ok(Err::<Adt17, Bytes>(f334(v2)?))
        }
    } else {
        Ok(Ok::<Adt17, Bytes>(Adt17::C3(v0)))
    }
}

pub fn f332(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2295 = v0.clone();
        length_bytes(a2295)
    };
    let v2: u64 = f100(v0.clone(), 0u64)?;
    let v3: u64 = {
        let a2296 = v2;
        let a2297 = 43u64;
        if if nat_eq(a2296, a2297) {
            true
        } else {
            let a2298 = v2;
            let a2299 = 45u64;
            nat_eq(a2298, a2299)
        } {
            1u64
        } else {
            0u64
        }
    };
    let v4: u64 = f402(v0.clone(), v3)?;
    let v5: bool = {
        let a2300 = f100(v0.clone(), v4)?;
        let a2301 = 46u64;
        nat_eq(a2300, a2301)
    };
    let v6: u64 = if v5 {
        let a2302 = v4;
        let a2303 = 1u64;
        nat_add(a2302, a2303)?
    } else {
        v4
    };
    let v7: u64 = if v5 {
        f402(v0.clone(), v6)?
    } else {
        v4
    };
    let v8: u64 = f100(v0.clone(), v7)?;
    let v9: bool = {
        let a2304 = v8;
        let a2305 = 101u64;
        if nat_eq(a2304, a2305) {
            true
        } else {
            let a2306 = v8;
            let a2307 = 69u64;
            nat_eq(a2306, a2307)
        }
    };
    let v10: u64 = f100(v0.clone(), {
        let a2308 = v7;
        let a2309 = 1u64;
        nat_add(a2308, a2309)?
    })?;
    let v11: bool = if v9 {
        let a2310 = v10;
        let a2311 = 43u64;
        if nat_eq(a2310, a2311) {
            true
        } else {
            let a2312 = v10;
            let a2313 = 45u64;
            nat_eq(a2312, a2313)
        }
    } else {
        false
    };
    let v12: u64 = if v9 {
        if v11 {
            let a2314 = v7;
            let a2315 = 2u64;
            nat_add(a2314, a2315)?
        } else {
            let a2316 = v7;
            let a2317 = 1u64;
            nat_add(a2316, a2317)?
        }
    } else {
        v7
    };
    let v13: u64 = if v9 {
        f402(v0.clone(), v12)?
    } else {
        v7
    };
    let v14: Bytes = f236(v0.clone(), v3, v4);
    let v15: u64 = f403(v14.clone())?;
    let a2318 = v13;
    let a2319 = v1;
    if if if nat_eq(a2318, a2319) {
        let a2320 = v3;
        let a2321 = v4;
        if nat_lt(a2320, a2321) {
            true
        } else {
            if v5 {
                let a2322 = v6;
                let a2323 = v7;
                nat_lt(a2322, a2323)
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
            let a2324 = v12;
            let a2325 = v13;
            nat_lt(a2324, a2325)
        }
    } else {
        false
    } {
        Ok(Some(f7(List::cons({
            let a2326 = v2;
            let a2327 = 45u64;
            if nat_eq(a2326, a2327) {
                Bytes::lit(&[0x2d])
            } else {
                Bytes::lit(&[])
            }
        }, List::cons({
            let a2329 = v15;
            let a2330 = {
                let a2328 = v14.clone();
                length_bytes(a2328)
            };
            if nat_eq(a2329, a2330) {
                Bytes::lit(&[0x30])
            } else {
                f236(v14.clone(), v15, {
                    let a2331 = v14;
                    length_bytes(a2331)
                })
            }
        }, List::cons({
            let a2332 = v6;
            let a2333 = v7;
            if nat_lt(a2332, a2333) {
                let a2334 = Bytes::lit(&[0x2e]);
                let a2335 = f236(v0.clone(), v6, v7);
                append_bytes(a2334, a2335)
            } else {
                Bytes::lit(&[])
            }
        }, List::cons(if v9 {
            let a2338 = if if v11 {
                let a2336 = v10;
                let a2337 = 45u64;
                nat_eq(a2336, a2337)
            } else {
                false
            } {
                Bytes::lit(&[0x65, 0x2d])
            } else {
                Bytes::lit(&[0x65])
            };
            let a2339 = f236(v0, v12, v13);
            append_bytes(a2338, a2339)
        } else {
            Bytes::lit(&[])
        }, List::<Bytes>::nil())))))))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f333(v0: Bytes, v1: u64) -> R<Option<(u64, Adt17)>> {
    let v2: bool = {
        let a2340 = f137(v0.clone(), v1)?;
        let a2341 = 45u64;
        nat_eq(a2340, a2341)
    };
    let v3: u64 = if v2 {
        let a2342 = v1;
        let a2343 = 1u64;
        nat_add(a2342, a2343)?
    } else {
        v1
    };
    let v4: u64 = f402(v0.clone(), v3)?;
    let v5: u64 = {
        let a2344 = v4;
        let a2345 = v3;
        nat_sub(a2344, a2345)
    };
    let v6: bool = {
        let a2346 = f137(v0.clone(), v4)?;
        let a2347 = 46u64;
        nat_eq(a2346, a2347)
    };
    let v7: u64 = if v6 {
        let a2348 = v4;
        let a2349 = 1u64;
        nat_add(a2348, a2349)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f402(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f137(v0.clone(), v8)?;
    let v10: bool = {
        let a2350 = v9;
        let a2351 = 101u64;
        if nat_eq(a2350, a2351) {
            true
        } else {
            let a2352 = v9;
            let a2353 = 69u64;
            nat_eq(a2352, a2353)
        }
    };
    let v11: u64 = f137(v0.clone(), {
        let a2354 = v8;
        let a2355 = 1u64;
        nat_add(a2354, a2355)?
    })?;
    let v12: bool = f302(v10, f304({
        let a2356 = v11;
        let a2357 = 43u64;
        nat_eq(a2356, a2357)
    }, {
        let a2358 = v11;
        let a2359 = 45u64;
        nat_eq(a2358, a2359)
    }));
    let v13: u64 = if v10 {
        if v12 {
            let a2360 = v8;
            let a2361 = 2u64;
            nat_add(a2360, a2361)?
        } else {
            let a2362 = v8;
            let a2363 = 1u64;
            nat_add(a2362, a2363)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f402(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f32(v0.clone(), v13, {
        let a2364 = v14;
        let a2365 = v13;
        nat_sub(a2364, a2365)
    });
    let v16: u64 = f403(v15.clone())?;
    let v17: Bytes = f32(v15.clone(), v16, {
        let a2367 = {
            let a2366 = v15;
            length_bytes(a2366)
        };
        let a2368 = v16;
        nat_sub(a2367, a2368)
    });
    let a2369 = v5;
    let a2370 = 0u64;
    if if nat_eq(a2369, a2370) {
        true
    } else {
        if f302({
            let a2371 = 1u64;
            let a2372 = v5;
            nat_lt(a2371, a2372)
        }, {
            let a2373 = f137(v0.clone(), v3)?;
            let a2374 = 48u64;
            nat_eq(a2373, a2374)
        }) {
            true
        } else {
            if f302(v6, {
                let a2375 = v8;
                let a2376 = v7;
                nat_eq(a2375, a2376)
            }) {
                true
            } else {
                f304(if v10 {
                    let a2377 = v14;
                    let a2378 = v13;
                    nat_eq(a2377, a2378)
                } else {
                    false
                }, {
                    let a2380 = 18u64;
                    let a2381 = {
                        let a2379 = v17.clone();
                        length_bytes(a2379)
                    };
                    nat_lt(a2380, a2381)
                })
            }
        }
    } {
        Ok(None::<(u64, Adt17)>)
    } else {
        let m2385 = {
            let a2383 = {
                let a2382 = v17.clone();
                length_bytes(a2382)
            };
            let a2384 = 0u64;
            if nat_eq(a2383, a2384) {
                Some(0u64)
            } else {
                f404(v17)?
            }
        };
        match m2385 {
            None => {
                Ok(None::<(u64, Adt17)>)
            }
            Some(v18) => {
                let m2390 = f405(f32(v0.clone(), v3, v5), f32(v0, v7, {
                    let a2386 = v8;
                    let a2387 = v7;
                    nat_sub(a2386, a2387)
                }), f302(v12, {
                    let a2388 = v11;
                    let a2389 = 45u64;
                    nat_eq(a2388, a2389)
                }), v18)?;
                match m2390 {
                    None => {
                        Ok(None::<(u64, Adt17)>)
                    }
                    Some(v19) => {
                        let a2393 = 19u64;
                        let a2394 = {
                            let a2392 = {
                                let (h2391, _) = v19.clone();
                                h2391
                            };
                            length_bytes(a2392)
                        };
                        if nat_lt(a2393, a2394) {
                            Ok(None::<(u64, Adt17)>)
                        } else {
                            let m2396 = f404({
                                let (h2395, _) = v19.clone();
                                h2395
                            })?;
                            match m2396 {
                                None => {
                                    Ok(None::<(u64, Adt17)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f302(v2, {
                                        let a2397 = 0u64;
                                        let a2398 = v20;
                                        nat_lt(a2397, a2398)
                                    });
                                    if f401(v21, v20) {
                                        Ok(Some((v14, {
                                            let a2400 = {
                                                let (_, h2399) = v19.clone();
                                                h2399
                                            };
                                            let a2401 = 0u64;
                                            if nat_eq(a2400, a2401) {
                                                Adt17::C1(v21, v20)
                                            } else {
                                                Adt17::C2(v21, v20, {
                                                    let (_, h2402) = v19;
                                                    h2402
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

pub fn f334(v0: u64) -> R<Bytes> {
    f81(Str::lit("a number outside the range Studio.Value holds is not read (YAML 1.2.2 section 10.3.2)"), v0)
}

pub fn f335(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f406(v0, v1)
}

pub fn f336(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f407(v0, v1)
}

pub fn f337(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m2403 = v0.clone();
    match m2403.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a2406 = {
                let (h2404, _) = v2;
                h2404
            };
            let a2407 = {
                let (h2405, _) = v1.clone();
                h2405
            };
            if equal(a2406, a2407) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f338(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f408(f409(Fn9::F410, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m2412 = {
        let (h2408, _) = f411(Fn10::F412, {
            let a2410 = {
                let a2409 = v0;
                length_list(a2409)
            };
            let a2411 = 1u64;
            nat_add(a2410, a2411)?
        }, v1)?;
        h2408
    };
    match m2412.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f339(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2417 = {
        let a2414 = f242({
            let (h2413, _) = v0;
            h2413
        })?;
        let a2415 = Bytes::lit(&[0x3a]);
        append_bytes(a2414, a2415)
    };
    let a2418 = {
        let (_, h2416) = v0;
        h2416
    };
    Ok(append_bytes(a2417, a2418))
}

pub fn f340(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f413(v1, v0, v2)
}

pub fn f341(v0: u64) -> bool {
    if f342(v0) {
        true
    } else {
        let a2419 = 65u64;
        let a2420 = v0;
        if if nat_le(a2419, a2420) {
            let a2421 = v0;
            let a2422 = 70u64;
            nat_le(a2421, a2422)
        } else {
            false
        } {
            true
        } else {
            let a2423 = 97u64;
            let a2424 = v0;
            if nat_le(a2423, a2424) {
                let a2425 = v0;
                let a2426 = 102u64;
                nat_le(a2425, a2426)
            } else {
                false
            }
        }
    }
}

pub fn f342(v0: u64) -> bool {
    let a2427 = 48u64;
    let a2428 = v0;
    if nat_le(a2427, a2428) {
        let a2429 = v0;
        let a2430 = 57u64;
        nat_le(a2429, a2430)
    } else {
        false
    }
}

pub fn f343(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f85(v0.clone(), v1)?;
    let a2431 = v2;
    let a2432 = 69u64;
    if if nat_eq(a2431, a2432) {
        true
    } else {
        let a2433 = v2;
        let a2434 = 101u64;
        nat_eq(a2433, a2434)
    } {
        let v3: u64 = f85(v0.clone(), {
            let a2435 = v1;
            let a2436 = 1u64;
            nat_add(a2435, a2436)?
        })?;
        let v4: u64 = {
            let a2437 = v3;
            let a2438 = 43u64;
            if if nat_eq(a2437, a2438) {
                true
            } else {
                let a2439 = v3;
                let a2440 = 45u64;
                nat_eq(a2439, a2440)
            } {
                let a2441 = v1;
                let a2442 = 2u64;
                nat_add(a2441, a2442)?
            } else {
                let a2443 = v1;
                let a2444 = 1u64;
                nat_add(a2443, a2444)?
            }
        };
        let v5: u64 = f251(v0, 0u64, v4)?;
        let a2445 = v4;
        let a2446 = v5;
        if nat_lt(a2445, a2446) {
            Ok(v5)
        } else {
            Ok(v1)
        }
    } else {
        Ok(v1)
    }
}

pub fn f344(v0: Adt18, v1: Bytes) -> bool {
    let a2447 = f345(v0.clone());
    let a2448 = 3u64;
    if !nat_eq(a2447, a2448) {
        f35(f346(v0), v1)
    } else {
        false
    }
}

pub fn f345(v0: Adt18) -> u64 {
    let m2449 = v0;
    match m2449 {
        Adt18::C0(v1, _) => {
            v1
        }
    }
}

pub fn f346(v0: Adt18) -> Bytes {
    let m2450 = v0;
    match m2450 {
        Adt18::C0(_, v2) => {
            v2
        }
    }
}

pub fn f347(v0: Bytes) -> Bytes {
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

pub fn f348(v0: Bytes) -> bool {
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

pub fn f349(p2459: u64, p2460: List<Adt18>, p2461: List<Adt13>) -> R<Result<(List<Adt13>, List<Adt18>), Bytes>> {
    let m2458 = (p2459, (p2460, p2461));
    {
        let mut m2458 = m2458;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2458;
                let m2451 = v0;
                if m2451 == 0 {
                    Err::<(u64, (List<Adt18>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt18>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt18>), Bytes>(f8(Str::lit("a dictionary has more members than the input has tokens")))))
                } else {
                    let v3 = m2451 - 1;
                    if f254(v1.clone(), Str::lit("}")) {
                        let m2452 = f261(f256(v1), Str::lit(";"));
                        match m2452 {
                            Ok(v5) => {
                                Err::<(u64, (List<Adt18>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt18>), Bytes>>>(Ok(Ok::<(List<Adt13>, List<Adt18>), Bytes>((f414(v2, List::<Adt13>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<Adt18>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt18>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt18>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m2453 = f183(v1)?;
                        match m2453 {
                            Ok(v7) => {
                                let v8: bool = f254(v7.clone(), Str::lit("required"));
                                let m2454 = f266(v3, if v8 {
                                    f256(v7)
                                } else {
                                    v7
                                }, 0u64)?;
                                match m2454 {
                                    Ok(v10) => {
                                        let m2455 = f260(f267(v10.clone()));
                                        match m2455 {
                                            Ok(v12) => {
                                                if if f254(f262(v12.clone()), Str::lit("=")) {
                                                    !v8
                                                } else {
                                                    false
                                                } {
                                                    let m2456 = f351(f256(f262(v12.clone())))?;
                                                    match m2456 {
                                                        Ok(v14) => {
                                                            Ok::<(u64, (List<Adt18>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt18>), Bytes>>>((v3, (v14, List::cons(Adt13::C0(f264(v12), f358(f268(v10)), 0u64), v2))))
                                                        }
                                                        Err(v13) => {
                                                            Err::<(u64, (List<Adt18>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt18>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt18>), Bytes>(v13)))
                                                        }
                                                    }
                                                } else {
                                                    let m2457 = f261(f262(v12.clone()), Str::lit(";"));
                                                    match m2457 {
                                                        Ok(v16) => {
                                                            Ok::<(u64, (List<Adt18>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt18>), Bytes>>>((v3, (v16, List::cons(Adt13::C0(f264(v12), f358(f268(v10)), if v8 {
                                                                0u64
                                                            } else {
                                                                1u64
                                                            }), v2))))
                                                        }
                                                        Err(v15) => {
                                                            Err::<(u64, (List<Adt18>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt18>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt18>), Bytes>(v15)))
                                                        }
                                                    }
                                                }
                                            }
                                            Err(v11) => {
                                                Err::<(u64, (List<Adt18>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt18>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt18>), Bytes>(v11)))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Err::<(u64, (List<Adt18>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt18>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt18>), Bytes>(v9)))
                                    }
                                }
                            }
                            Err(v6) => {
                                Err::<(u64, (List<Adt18>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt18>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt18>), Bytes>(v6)))
                            }
                        }
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

pub fn f350(p2470: u64, p2471: List<Adt18>, p2472: List<Adt13>) -> R<Result<(List<Adt13>, List<Adt18>), Bytes>> {
    let m2469 = (p2470, (p2471, p2472));
    {
        let mut m2469 = m2469;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2469;
                let m2462 = v0;
                if m2462 == 0 {
                    Err::<(u64, (List<Adt18>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt18>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt18>), Bytes>(f8(Str::lit("an interface has more members than the input has tokens")))))
                } else {
                    let v3 = m2462 - 1;
                    if f254(v1.clone(), Str::lit("}")) {
                        let m2463 = f261(f256(v1), Str::lit(";"));
                        match m2463 {
                            Ok(v5) => {
                                Err::<(u64, (List<Adt18>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt18>), Bytes>>>(Ok(Ok::<(List<Adt13>, List<Adt18>), Bytes>((f414(v2, List::<Adt13>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<Adt18>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt18>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt18>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m2464 = f183(v1)?;
                        match m2464 {
                            Ok(v7) => {
                                let v8: List<Adt18> = if if f254(v7.clone(), Str::lit("stringifier")) {
                                    true
                                } else {
                                    f254(v7.clone(), Str::lit("inherit"))
                                } {
                                    f256(v7.clone())
                                } else {
                                    v7.clone()
                                };
                                let v9: List<Adt18> = if f254(v8.clone(), Str::lit("readonly")) {
                                    f256(v8)
                                } else {
                                    v8
                                };
                                if f254(v9.clone(), Str::lit("attribute")) {
                                    let m2465 = f266(v3, f256(v9), 0u64)?;
                                    match m2465 {
                                        Ok(v11) => {
                                            let m2466 = f260(f267(v11.clone()));
                                            match m2466 {
                                                Ok(v13) => {
                                                    let m2467 = f261(f262(v13.clone()), Str::lit(";"));
                                                    match m2467 {
                                                        Ok(v15) => {
                                                            Ok::<(u64, (List<Adt18>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt18>), Bytes>>>((v3, (v15, List::cons(Adt13::C0(f264(v13), f358(f268(v11)), 0u64), v2))))
                                                        }
                                                        Err(v14) => {
                                                            Err::<(u64, (List<Adt18>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt18>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt18>), Bytes>(v14)))
                                                        }
                                                    }
                                                }
                                                Err(v12) => {
                                                    Err::<(u64, (List<Adt18>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt18>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt18>), Bytes>(v12)))
                                                }
                                            }
                                        }
                                        Err(v10) => {
                                            Err::<(u64, (List<Adt18>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt18>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt18>), Bytes>(v10)))
                                        }
                                    }
                                } else {
                                    let m2468 = f351(v7)?;
                                    match m2468 {
                                        Ok(v17) => {
                                            Ok::<(u64, (List<Adt18>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt18>), Bytes>>>((v3, (v17, v2)))
                                        }
                                        Err(v16) => {
                                            Err::<(u64, (List<Adt18>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt18>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt18>), Bytes>(v16)))
                                        }
                                    }
                                }
                            }
                            Err(v6) => {
                                Err::<(u64, (List<Adt18>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt18>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt18>), Bytes>(v6)))
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

pub fn f351(v0: List<Adt18>) -> R<Result<List<Adt18>, Bytes>> {
    Ok(f255({
        let a2474 = {
            let a2473 = v0.clone();
            length_list(a2473)
        };
        let a2475 = 1u64;
        nat_add(a2474, a2475)?
    }, v0, List::<Bytes>::nil(), true))
}

pub fn f352(v0: List<Adt18>) -> Bytes {
    let m2476 = v0;
    match m2476.uncons() {
        None => {
            f8(Str::lit("the end of the input"))
        }
        Some((v1, _)) => {
            f7(List::cons(f8(Str::lit("'")), List::cons(f346(v1), List::cons(f8(Str::lit("'")), List::<Bytes>::nil()))))
        }
    }
}

pub fn f353(v0: Bytes) -> Bytes {
    if f145(v0.clone(), f8(Str::lit("_"))) {
        f32(v0.clone(), 1u64, {
            let a2478 = {
                let a2477 = v0;
                length_bytes(a2477)
            };
            let a2479 = 1u64;
            nat_sub(a2478, a2479)
        })
    } else {
        v0
    }
}

pub fn f354(v0: Adt14, v1: List<Adt18>) -> Result<Adt22, Bytes> {
    if f254(v1.clone(), Str::lit("?")) {
        Ok::<Adt22, Bytes>(Adt22::C0(f355(Str::lit("Option"), v0), f256(v1)))
    } else {
        Ok::<Adt22, Bytes>(Adt22::C0(v0, v1))
    }
}

pub fn f355(v0: Str, v1: Adt14) -> Adt14 {
    let v2: Adt14 = f358(v1);
    Adt14::C0(0u64, f7(List::cons(f8(Str::lit("(")), List::cons({
        let a2480 = v0;
        utf8_encode(a2480)
    }, List::cons(f8(Str::lit(" ")), List::cons(f202(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), f203(v2.clone()), f200(v2), Bytes::lit(&[]))
}

pub fn f356(v0: Bytes) -> Adt14 {
    Adt14::C0(0u64, Bytes::lit(&[]), List::<Bytes>::nil(), v0, Bytes::lit(&[]))
}

pub fn f357(v0: Bytes) -> Option<Adt14> {
    f415(v0, f416())
}

pub fn f358(v0: Adt14) -> Adt14 {
    let a2481 = f201(v0.clone());
    let a2482 = 0u64;
    if nat_eq(a2481, a2482) {
        v0
    } else {
        Adt14::C0(0u64, Bytes::lit(&[]), f203(v0.clone()), if f35(f200(v0.clone()), Bytes::lit(&[])) {
            f417()
        } else {
            f200(v0)
        }, Bytes::lit(&[]))
    }
}

pub fn f359(v0: Adt14) -> Bytes {
    let m2483 = v0;
    match m2483 {
        Adt14::C0(_, _, _, _, v5) => {
            v5
        }
    }
}

pub fn f360(v0: Adt13, v1: List<Adt13>) -> R<List<Adt13>> {
    let m2484 = v1;
    match m2484.uncons() {
        None => {
            Ok(List::cons(v0, List::<Adt13>::nil()))
        }
        Some((v2, v3)) => {
            if f418({
                let a2486 = {
                    let a2485 = f282(v0.clone());
                    length_bytes(a2485)
                };
                let a2487 = 1u64;
                nat_add(a2486, a2487)?
            }, f282(v0.clone()), f282(v2.clone()), 0u64)? {
                Ok(List::cons(v0, List::cons(v2, v3)))
            } else {
                Ok(List::cons(v2, f360(v0, v3)?))
            }
        }
    }
}

pub fn f361(v0: List<Adt12>, v1: Bytes) -> List<Adt13> {
    let m2488 = f121(v0.clone(), 1u64, v1.clone());
    match m2488 {
        None => {
            f279(v0, 6u64, v1)
        }
        Some(v2) => {
            f277(f278(v2), f279(v0, 6u64, v1))
        }
    }
}

pub fn f362(v0: Bytes) -> R<Option<Bytes>> {
    let m2490 = f419({
        let a2489 = v0.clone();
        length_bytes(a2489)
    }, v0, 0u64, Bytes::lit(&[]))?;
    match m2490 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a2492 = {
                let a2491 = v1.clone();
                length_bytes(a2491)
            };
            let a2493 = 0u64;
            if nat_eq(a2492, a2493) {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(v1))
            }
        }
    }
}

pub fn f363(p2503: u64, v1: Bytes, p2504: u64) -> R<bool> {
    let m2502 = (p2503, p2504);
    {
        let mut m2502 = m2502;
        loop {
            let step = {
                let (v0, v2) = m2502;
                let m2494 = v0;
                if m2494 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m2494 - 1;
                    let m2497 = {
                        let a2495 = v1.clone();
                        let a2496 = v2;
                        index_bytes(a2495, a2496)
                    };
                    match m2497 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a2498 = f41(v4)?;
                            let a2499 = 128u64;
                            if nat_lt(a2498, a2499) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a2500 = v2;
                                    let a2501 = 1u64;
                                    nat_add(a2500, a2501)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
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

pub fn f364(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f223(v0.clone(), 0u64, {
        let a2505 = v0;
        length_bytes(a2505)
    }, false, Bytes::lit(&[]))?;
    let m2508 = {
        let a2506 = v1.clone();
        let a2507 = 0u64;
        index_bytes(a2506, a2507)
    };
    match m2508 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f41(v2)?;
            if f305(v3) {
                let a2515 = f32({
                    let a2509 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a2509)
                }, {
                    let a2510 = v3;
                    let a2511 = 65u64;
                    nat_sub(a2510, a2511)
                }, 1u64);
                let a2516 = f32(v1.clone(), 1u64, {
                    let a2513 = {
                        let a2512 = v1;
                        length_bytes(a2512)
                    };
                    let a2514 = 1u64;
                    nat_sub(a2513, a2514)
                });
                Ok(append_bytes(a2515, a2516))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f365(v0: Bytes) -> R<bool> {
    let m2519 = {
        let a2517 = v0;
        let a2518 = 0u64;
        index_bytes(a2517, a2518)
    };
    match m2519 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f304(f303(f41(v1)?), f305(f41(v1)?)))
        }
    }
}

pub fn f366(p2532: u64, v1: Bytes, p2533: u64, p2534: Bytes) -> R<Option<Bytes>> {
    let m2531 = (p2532, (p2533, p2534));
    {
        let mut m2531 = m2531;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2531;
                let m2520 = v0;
                if m2520 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m2520 - 1;
                    let a2522 = {
                        let a2521 = v1.clone();
                        length_bytes(a2521)
                    };
                    let a2523 = v2;
                    if nat_le(a2522, a2523) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m2524 = f288(v1.clone(), v2)?;
                        match m2524 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a2526 = v2;
                                    let a2527 = {
                                        let (_, h2525) = v5;
                                        h2525
                                    };
                                    nat_add(a2526, a2527)?
                                }, {
                                    let a2529 = v3;
                                    let a2530 = f420({
                                        let (h2528, _) = v5;
                                        h2528
                                    });
                                    append_bytes(a2529, a2530)
                                })))
                            }
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

pub fn f367(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2537 = {
        let a2535 = v0;
        let a2536 = v1;
        index_bytes(a2535, a2536)
    };
    match m2537 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f41(v2)?;
            if f302({
                let a2538 = 128u64;
                let a2539 = v3;
                nat_le(a2538, a2539)
            }, {
                let a2540 = v3;
                let a2541 = 191u64;
                nat_le(a2540, a2541)
            }) {
                Ok(Some({
                    let a2542 = v3;
                    let a2543 = 128u64;
                    nat_sub(a2542, a2543)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f368(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f302({
        let a2544 = v1;
        let a2545 = v0;
        nat_le(a2544, a2545)
    }, f302({
        let a2546 = v0;
        let a2547 = v2;
        nat_le(a2546, a2547)
    }, f304({
        let a2548 = v0;
        let a2549 = 55296u64;
        nat_lt(a2548, a2549)
    }, {
        let a2550 = 57343u64;
        let a2551 = v0;
        nat_lt(a2550, a2551)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f369() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{3}/\u{7f}\u{3}@\u{0}\u{3}\u{7f}}\u{4}\u{0}\u{0}C\u{7f}\u{7f}"))
}

pub fn f370(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a2552 = v1;
        let a2553 = 3u64;
        nat_mul(a2552, a2553)?
    };
    let a2564 = {
        let a2560 = {
            let a2554 = f137(v0.clone(), v2)?;
            let a2555 = 16384u64;
            nat_mul(a2554, a2555)?
        };
        let a2561 = {
            let a2558 = f137(v0.clone(), {
                let a2556 = v2;
                let a2557 = 1u64;
                nat_add(a2556, a2557)?
            })?;
            let a2559 = 128u64;
            nat_mul(a2558, a2559)?
        };
        nat_add(a2560, a2561)?
    };
    let a2565 = f137(v0, {
        let a2562 = v2;
        let a2563 = 2u64;
        nat_add(a2562, a2563)?
    })?;
    nat_add(a2564, a2565)
}

pub fn f371(v0: u64) -> R<bool> {
    f208(v0, f421())
}

pub fn f372(v0: u64) -> R<bool> {
    f208(v0, f422())
}

pub fn f373(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f423(v1, v0, v2)
}

pub fn f374(v0: Bytes, v1: u64) -> Bytes {
    f32(v0, v1, 1u64)
}

pub fn f375() -> Bytes {
    f8(Str::lit("\u{0}\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\u{9}\u{a}\u{b}\u{c}\u{d}\u{e}\u{f}\u{10}\u{11}\u{12}\u{13}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{1b}\u{1c}\u{1d}\u{1e}\u{1f} !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~\u{7f}"))
}

pub fn f376() -> Bytes {
    f8(Str::lit("\u{80}\u{c0}\u{100}\u{140}\u{180}\u{1c0}\u{200}\u{240}\u{280}\u{2c0}\u{300}\u{340}\u{380}\u{3c0}\u{400}\u{440}\u{480}\u{4c0}\u{500}\u{540}\u{580}\u{5c0}\u{600}\u{640}\u{680}\u{6c0}\u{700}\u{740}\u{780}\u{7c0}"))
}

pub fn f377(v0: u64) -> R<Bytes> {
    Ok(f374(f424(), {
        let a2571 = {
            let a2569 = {
                let a2566 = v0;
                let a2567 = 64u64;
                let a2568 = 0u64;
                nat_rem(a2566, a2567, a2568)
            };
            let a2570 = 2u64;
            nat_mul(a2569, a2570)?
        };
        let a2572 = 1u64;
        nat_add(a2571, a2572)?
    }))
}

pub fn f378() -> Bytes {
    f8(Str::lit("\u{800}\u{1000}\u{2000}\u{3000}\u{4000}\u{5000}\u{6000}\u{7000}\u{8000}\u{9000}\u{a000}\u{b000}\u{c000}\u{d000}\u{e000}\u{f000}"))
}

pub fn f379(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<Option<u64>> {
    if f425(f85(v0, v3)?, v1, v2)? {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a2573 = v3;
            let a2574 = 1u64;
            nat_add(a2573, a2574)?
        }))
    }
}

pub fn f380(v0: Bytes) -> Result<Bytes, Bytes> {
    Err::<Bytes, Bytes>(v0)
}

pub fn f381() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f382(v0: Bytes) -> R<Option<Bytes>> {
    let m2575 = f131(v0.clone(), f8(Str::lit(" ")), 0u64)?;
    match m2575 {
        None => {
            let m2576 = f131(v0.clone(), f8(Str::lit(":")), 0u64)?;
            match m2576 {
                None => {
                    let v3: Bytes = f7(List::cons(f8(Str::lit(" ")), List::cons(v0, List::cons(f8(Str::lit(":")), List::<Bytes>::nil()))));
                    let m2577 = f131(f426(), v3.clone(), 0u64)?;
                    match m2577 {
                        None => {
                            Ok(None::<Bytes>)
                        }
                        Some(v4) => {
                            let v5: u64 = {
                                let a2579 = v4;
                                let a2580 = {
                                    let a2578 = v3;
                                    length_bytes(a2578)
                                };
                                nat_add(a2579, a2580)?
                            };
                            let m2581 = f131(f426(), f8(Str::lit(" ")), v5)?;
                            match m2581 {
                                None => {
                                    Ok(None::<Bytes>)
                                }
                                Some(v6) => {
                                    Ok(Some(f32(f426(), v5, {
                                        let a2582 = v6;
                                        let a2583 = v5;
                                        nat_sub(a2582, a2583)
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

pub fn f383(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2584 = 0u64;
    let a2585 = v1;
    if if nat_lt(a2584, a2585) {
        f169({
            let a2588 = f100(v0, {
                let a2586 = v1;
                let a2587 = 1u64;
                nat_sub(a2586, a2587)
            })?;
            let a2589 = 10u64;
            nat_eq(a2588, a2589)
        })
    } else {
        false
    } {
        Ok(Some({
            let a2590 = v1;
            let a2591 = 1u64;
            nat_sub(a2590, a2591)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f384(v0: Bytes, v1: u64, v2: u64) -> R<Result<(u64, u64), Bytes>> {
    let v3: (u64, u64) = {
        let (h2592, _) = f316(Fn7::F427(v2, v0.clone()), {
            let a2594 = {
                let a2593 = v0.clone();
                length_bytes(a2593)
            };
            let a2595 = 1u64;
            nat_add(a2594, a2595)?
        }, (v1, 0u64))?;
        h2592
    };
    let v4: u64 = f385(v0.clone(), {
        let (h2596, _) = v3;
        h2596
    }, v2)?;
    let a2597 = f100(v0, v4)?;
    let a2598 = 9u64;
    if if nat_eq(a2597, a2598) {
        let a2602 = {
            let a2600 = v4;
            let a2601 = {
                let (h2599, _) = v3;
                h2599
            };
            nat_sub(a2600, a2601)
        };
        let a2603 = v2;
        nat_lt(a2602, a2603)
    } else {
        false
    } {
        Ok(Err::<(u64, u64), Bytes>(f81(Str::lit("a tab cannot indent a block scalar line (YAML 1.2.2 [63], [70])"), v4)?))
    } else {
        Ok(Ok::<(u64, u64), Bytes>(({
            let (_, h2604) = v3;
            h2604
        }, {
            let (h2605, _) = v3;
            h2605
        })))
    }
}

pub fn f385(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h2606, _) = f128(Fn4::F428(v2, v0.clone(), v1), {
        let a2608 = {
            let a2607 = v0;
            length_bytes(a2607)
        };
        let a2609 = 1u64;
        nat_add(a2608, a2609)?
    }, v1)?;
    Ok(h2606)
}

pub fn f386(v0: Bytes, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = f385(v0.clone(), v1, v2)?;
    let a2612 = {
        let a2610 = v3;
        let a2611 = v1;
        nat_sub(a2610, a2611)
    };
    let a2613 = v2;
    if nat_eq(a2612, a2613) {
        Ok(f169({
            let a2614 = f100(v0, v3)?;
            let a2615 = 256u64;
            nat_eq(a2614, a2615)
        }))
    } else {
        Ok(false)
    }
}

pub fn f387(v0: Bytes, p2629: u64, v2: u64, v3: bool, p2630: u64, p2631: Bytes, p2632: u64) -> R<Result<(Bytes, (u64, u64)), Bytes>> {
    let m2628 = (p2629, (p2630, (p2631, p2632)));
    {
        let mut m2628 = m2628;
        loop {
            let step = {
                let (v1, (v4, (v5, v6))) = m2628;
                let m2616 = v4;
                if m2616 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<Result<(Bytes, (u64, u64)), Bytes>>>(Ok(Err::<(Bytes, (u64, u64)), Bytes>(f81(Str::lit("a block scalar has more lines than the input"), v1)?)))
                } else {
                    let v7 = m2616 - 1;
                    let v8: u64 = f385(v0.clone(), v1, v2)?;
                    let v9: bool = f169(f157(f100(v0.clone(), v8)?));
                    let v10: u64 = f238(v0.clone(), v8)?;
                    let v11: Bytes = f7(List::cons(v5, List::cons(f318(v6), List::cons(f236(v0.clone(), v8, v10), List::<Bytes>::nil()))));
                    let m2617 = f384(v0.clone(), f167(v0.clone(), v10)?, v2)?;
                    match m2617 {
                        Ok(v13) => {
                            let v14: u64 = {
                                let (_, h2618) = v13;
                                h2618
                            };
                            let v15: u64 = f385(v0.clone(), v14, v2)?;
                            if f386(v0.clone(), v14, v2)? {
                                Ok::<(u64, (u64, (Bytes, u64))), R<Result<(Bytes, (u64, u64)), Bytes>>>((v14, (v7, ({
                                    let a2622 = v11;
                                    let a2623 = if if if v3 {
                                        v9
                                    } else {
                                        false
                                    } {
                                        f169(f157(f100(v0.clone(), v15)?))
                                    } else {
                                        false
                                    } {
                                        let a2620 = {
                                            let (h2619, _) = v13;
                                            h2619
                                        };
                                        let a2621 = 0u64;
                                        if nat_eq(a2620, a2621) {
                                            Bytes::lit(&[0x20])
                                        } else {
                                            Bytes::lit(&[])
                                        }
                                    } else {
                                        Bytes::lit(&[0x0a])
                                    };
                                    append_bytes(a2622, a2623)
                                }, {
                                    let (h2624, _) = v13;
                                    h2624
                                }))))
                            } else {
                                Err::<(u64, (u64, (Bytes, u64))), R<Result<(Bytes, (u64, u64)), Bytes>>>(Ok(Ok::<(Bytes, (u64, u64)), Bytes>((v11, ({
                                    let (h2625, _) = v13;
                                    h2625
                                }, {
                                    let a2626 = f100(v0.clone(), v15)?;
                                    let a2627 = 256u64;
                                    if nat_eq(a2626, a2627) {
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
                Ok(next) => { m2628 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f388(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f160(v0.clone(), {
        let (h2633, _) = v1;
        h2633
    })?;
    let a2634 = f100(v0, v2)?;
    let a2635 = 10u64;
    if nat_eq(a2634, a2635) {
        Ok(Some(({
            let a2636 = v2;
            let a2637 = 1u64;
            nat_add(a2636, a2637)?
        }, f320({
            let (_, h2638) = v1;
            h2638
        }, {
            let a2640 = v2;
            let a2641 = {
                let (h2639, _) = v1;
                h2639
            };
            nat_sub(a2640, a2641)
        }))))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f389(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f429(v0, v1)
}

pub fn f390(v0: u64) -> Option<Bytes> {
    let a2642 = v0;
    let a2643 = 48u64;
    if nat_eq(a2642, a2643) {
        Some(Bytes::lit(&[0x00]))
    } else {
        let a2644 = v0;
        let a2645 = 97u64;
        if nat_eq(a2644, a2645) {
            Some(Bytes::lit(&[0x07]))
        } else {
            let a2646 = v0;
            let a2647 = 98u64;
            if nat_eq(a2646, a2647) {
                Some(Bytes::lit(&[0x08]))
            } else {
                let a2648 = v0;
                let a2649 = 116u64;
                if if nat_eq(a2648, a2649) {
                    true
                } else {
                    let a2650 = v0;
                    let a2651 = 9u64;
                    nat_eq(a2650, a2651)
                } {
                    Some(Bytes::lit(&[0x09]))
                } else {
                    let a2652 = v0;
                    let a2653 = 110u64;
                    if nat_eq(a2652, a2653) {
                        Some(Bytes::lit(&[0x0a]))
                    } else {
                        let a2654 = v0;
                        let a2655 = 118u64;
                        if nat_eq(a2654, a2655) {
                            Some(Bytes::lit(&[0x0b]))
                        } else {
                            let a2656 = v0;
                            let a2657 = 102u64;
                            if nat_eq(a2656, a2657) {
                                Some(Bytes::lit(&[0x0c]))
                            } else {
                                let a2658 = v0;
                                let a2659 = 114u64;
                                if nat_eq(a2658, a2659) {
                                    Some(Bytes::lit(&[0x0d]))
                                } else {
                                    let a2660 = v0;
                                    let a2661 = 101u64;
                                    if nat_eq(a2660, a2661) {
                                        Some(Bytes::lit(&[0x1b]))
                                    } else {
                                        let a2662 = v0;
                                        let a2663 = 32u64;
                                        if nat_eq(a2662, a2663) {
                                            Some(Bytes::lit(&[0x20]))
                                        } else {
                                            let a2664 = v0;
                                            let a2665 = 34u64;
                                            if nat_eq(a2664, a2665) {
                                                Some(Bytes::lit(&[0x22]))
                                            } else {
                                                let a2666 = v0;
                                                let a2667 = 47u64;
                                                if nat_eq(a2666, a2667) {
                                                    Some(Bytes::lit(&[0x2f]))
                                                } else {
                                                    let a2668 = v0;
                                                    let a2669 = 92u64;
                                                    if nat_eq(a2668, a2669) {
                                                        Some(Bytes::lit(&[0x5c]))
                                                    } else {
                                                        let a2670 = v0;
                                                        let a2671 = 78u64;
                                                        if nat_eq(a2670, a2671) {
                                                            Some(Bytes::lit(&[0xc2, 0x85]))
                                                        } else {
                                                            let a2672 = v0;
                                                            let a2673 = 95u64;
                                                            if nat_eq(a2672, a2673) {
                                                                Some(Bytes::lit(&[0xc2, 0xa0]))
                                                            } else {
                                                                let a2674 = v0;
                                                                let a2675 = 76u64;
                                                                if nat_eq(a2674, a2675) {
                                                                    Some(Bytes::lit(&[0xe2, 0x80, 0xa8]))
                                                                } else {
                                                                    let a2676 = v0;
                                                                    let a2677 = 80u64;
                                                                    if nat_eq(a2676, a2677) {
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

pub fn f391(v0: u64) -> R<Bytes> {
    f81(Str::lit("an implicit key must be on one line (YAML 1.2.2 [154])"), v0)
}

pub fn f392(v0: Bytes, p2692: u64, v2: u64, p2693: u64, p2694: u64) -> R<Result<(u64, u64), Bytes>> {
    let m2691 = (p2692, (p2693, p2694));
    {
        let mut m2691 = m2691;
        loop {
            let step = {
                let (v1, (v3, v4)) = m2691;
                let m2678 = v3;
                if m2678 == 0 {
                    Err::<(u64, (u64, u64)), R<Result<(u64, u64), Bytes>>>(Ok(Err::<(u64, u64), Bytes>(f81(Str::lit("the input ends inside a scalar"), v1)?)))
                } else {
                    let v5 = m2678 - 1;
                    let v6: u64 = {
                        let a2679 = v1;
                        let a2680 = 1u64;
                        nat_add(a2679, a2680)?
                    };
                    let v7: u64 = f160(v0.clone(), v6)?;
                    let v8: u64 = f161(v0.clone(), v7)?;
                    let v9: u64 = f100(v0.clone(), v8)?;
                    if f99(v0.clone(), v6)? {
                        Err::<(u64, (u64, u64)), R<Result<(u64, u64), Bytes>>>(Ok(Err::<(u64, u64), Bytes>(f81(Str::lit("a document marker cannot appear inside a scalar (YAML 1.2.2 [206])"), v6)?)))
                    } else {
                        let a2681 = v9;
                        let a2682 = 10u64;
                        if nat_eq(a2681, a2682) {
                            Ok::<(u64, (u64, u64)), R<Result<(u64, u64), Bytes>>>((v8, (v5, {
                                let a2683 = v4;
                                let a2684 = 1u64;
                                nat_add(a2683, a2684)?
                            })))
                        } else {
                            let a2685 = v9;
                            let a2686 = 256u64;
                            if nat_eq(a2685, a2686) {
                                Err::<(u64, (u64, u64)), R<Result<(u64, u64), Bytes>>>(Ok(Err::<(u64, u64), Bytes>(f81(Str::lit("the input ends inside a quoted scalar (YAML 1.2.2 [109], [120])"), v8)?)))
                            } else {
                                let a2689 = {
                                    let a2687 = v7;
                                    let a2688 = v6;
                                    nat_sub(a2687, a2688)
                                };
                                let a2690 = v2;
                                if nat_lt(a2689, a2690) {
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
                Ok(next) => { m2691 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f393(v0: Bytes, p2704: u64, p2705: u64, p2706: u64) -> R<Option<u64>> {
    let m2703 = (p2704, (p2705, p2706));
    {
        let mut m2703 = m2703;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2703;
                let m2695 = v2;
                if m2695 == 0 {
                    Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(Some(v3)))
                } else {
                    let v4 = m2695 - 1;
                    let m2696 = f430(f100(v0.clone(), v1)?);
                    match m2696 {
                        None => {
                            Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(None::<u64>))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, u64)), R<Option<u64>>>(({
                                let a2697 = v1;
                                let a2698 = 1u64;
                                nat_add(a2697, a2698)?
                            }, (v4, {
                                let a2701 = {
                                    let a2699 = v3;
                                    let a2700 = 16u64;
                                    nat_mul(a2699, a2700)?
                                };
                                let a2702 = v5;
                                nat_add(a2701, a2702)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2703 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f394(v0: Bytes, v1: u64) -> R<u64> {
    let (h2707, _) = f128(Fn4::F431(v0.clone()), {
        let a2709 = {
            let a2708 = v0;
            length_bytes(a2708)
        };
        let a2710 = 1u64;
        nat_add(a2709, a2710)?
    }, v1)?;
    Ok(h2707)
}

pub fn f395(v0: Bytes, v1: u64) -> R<u64> {
    let (h2711, _) = f128(Fn4::F432(v0.clone()), {
        let a2713 = {
            let a2712 = v0;
            length_bytes(a2712)
        };
        let a2714 = 1u64;
        nat_add(a2713, a2714)?
    }, v1)?;
    Ok(h2711)
}

pub fn f396(v0: Fn8, p2719: u64, p2720: (u64, bool)) -> R<((u64, bool), bool)> {
    let m2718 = (p2719, p2720);
    {
        let mut m2718 = m2718;
        loop {
            let step = {
                let (v1, v2) = m2718;
                let m2715 = v1;
                if m2715 == 0 {
                    Err::<(u64, (u64, bool)), R<((u64, bool), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2715 - 1;
                    let m2717 = {
                        let c2716: Fn8 = v0.clone();
                        c2716.apply(v2)?
                    };
                    match m2717 {
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
                Ok(next) => { m2718 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f397(v0: u64, v1: Bytes, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    f433(v1, v0, v2)
}

pub fn f398(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = {
        let (h2721, _) = v2;
        h2721
    };
    let v4: u64 = f100(v0.clone(), v3)?;
    if f158(v4) {
        Ok(None::<(u64, u64)>)
    } else {
        if f157(v4) {
            Ok(Some(({
                let a2722 = v3;
                let a2723 = 1u64;
                nat_add(a2722, a2723)?
            }, {
                let (_, h2724) = v2;
                h2724
            })))
        } else {
            let a2725 = v4;
            let a2726 = 58u64;
            if if nat_eq(a2725, a2726) {
                f169(f229(f100(v0.clone(), {
                    let a2727 = v3;
                    let a2728 = 1u64;
                    nat_add(a2727, a2728)?
                })?, v1))
            } else {
                false
            } {
                Ok(None::<(u64, u64)>)
            } else {
                let a2729 = v4;
                let a2730 = 35u64;
                if if nat_eq(a2729, a2730) {
                    f157(f100(v0, {
                        let a2731 = v3;
                        let a2732 = 1u64;
                        nat_sub(a2731, a2732)
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
                            let a2733 = v3;
                            let a2734 = 1u64;
                            nat_add(a2733, a2734)?
                        }, {
                            let a2735 = v3;
                            let a2736 = 1u64;
                            nat_add(a2735, a2736)?
                        })))
                    }
                }
            }
        }
    }
}

pub fn f399(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f434(v0, v1)
}

pub fn f400(v0: Bytes, v1: u64, v2: u64) -> R<(u64, u64)> {
    let (h2737, _) = f316(Fn7::F435(v1, v0.clone()), {
        let a2739 = {
            let a2738 = v0;
            length_bytes(a2738)
        };
        let a2740 = 1u64;
        nat_add(a2739, a2740)?
    }, (v2, 0u64))?;
    Ok(h2737)
}

pub fn f401(v0: bool, v1: u64) -> bool {
    if v0 {
        let a2741 = 0u64;
        let a2742 = v1;
        if nat_lt(a2741, a2742) {
            let a2743 = v1;
            let a2744 = 9223372036854775808u64;
            nat_le(a2743, a2744)
        } else {
            false
        }
    } else {
        let a2745 = v1;
        let a2746 = 9223372036854775807u64;
        nat_le(a2745, a2746)
    }
}

pub fn f402(v0: Bytes, v1: u64) -> R<u64> {
    let (h2747, _) = f128(Fn4::F436(v0.clone()), {
        let a2749 = {
            let a2748 = v0;
            length_bytes(a2748)
        };
        let a2750 = 1u64;
        nat_add(a2749, a2750)?
    }, v1)?;
    Ok(h2747)
}

pub fn f403(v0: Bytes) -> R<u64> {
    let (h2751, _) = f128(Fn4::F437(v0.clone()), {
        let a2753 = {
            let a2752 = v0;
            length_bytes(a2752)
        };
        let a2754 = 1u64;
        nat_add(a2753, a2754)?
    }, 0u64)?;
    Ok(h2751)
}

pub fn f404(v0: Bytes) -> R<Option<u64>> {
    if f438(v0.clone())? {
        Ok(Some({
            let (_, h2755) = {
                let (h2756, _) = f316(Fn7::F439(v0.clone()), {
                    let a2757 = v0;
                    length_bytes(a2757)
                }, (0u64, 0u64))?;
                h2756
            };
            h2755
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f405(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f32(v1.clone(), 0u64, {
        let a2759 = {
            let a2758 = v1.clone();
            length_bytes(a2758)
        };
        let a2760 = f440(v1)?;
        nat_sub(a2759, a2760)
    });
    let v5: Bytes = {
        let a2761 = v0;
        let a2762 = v4.clone();
        append_bytes(a2761, a2762)
    };
    let v6: u64 = f403(v5.clone())?;
    let v7: Bytes = f32(v5.clone(), v6, {
        let a2764 = {
            let a2763 = v5;
            length_bytes(a2763)
        };
        let a2765 = v6;
        nat_sub(a2764, a2765)
    });
    let v8: u64 = {
        let a2766 = v4;
        length_bytes(a2766)
    };
    let v9: u64 = {
        let a2767 = v7.clone();
        length_bytes(a2767)
    };
    let a2768 = v9;
    let a2769 = 0u64;
    if nat_eq(a2768, a2769) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f304(v2, {
            let a2770 = v3;
            let a2771 = v8;
            nat_lt(a2770, a2771)
        }) {
            let v10: u64 = if v2 {
                let a2772 = v3;
                let a2773 = v8;
                nat_add(a2772, a2773)?
            } else {
                let a2774 = v8;
                let a2775 = v3;
                nat_sub(a2774, a2775)
            };
            let v11: u64 = f440(v7.clone())?;
            let a2776 = v10;
            let a2777 = v11;
            if nat_le(a2776, a2777) {
                Ok(Some((f32(v7, 0u64, {
                    let a2778 = v9;
                    let a2779 = v10;
                    nat_sub(a2778, a2779)
                }), 0u64)))
            } else {
                let a2782 = {
                    let a2780 = v10;
                    let a2781 = v11;
                    nat_sub(a2780, a2781)
                };
                let a2783 = 19u64;
                if nat_le(a2782, a2783) {
                    Ok(Some((f32(v7, 0u64, {
                        let a2784 = v9;
                        let a2785 = v11;
                        nat_sub(a2784, a2785)
                    }), {
                        let a2786 = v10;
                        let a2787 = v11;
                        nat_sub(a2786, a2787)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a2788 = v3;
                let a2789 = v8;
                nat_sub(a2788, a2789)
            };
            let a2792 = {
                let a2790 = v9;
                let a2791 = v12;
                nat_add(a2790, a2791)?
            };
            let a2793 = 19u64;
            if nat_le(a2792, a2793) {
                Ok(Some(({
                    let a2794 = v7;
                    let a2795 = f240(v12);
                    append_bytes(a2794, a2795)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f406(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f158(f100(v0, v1)?) {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a2796 = v1;
            let a2797 = 1u64;
            nat_add(a2796, a2797)?
        }))
    }
}

pub fn f407(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m2801 = {
        let a2799 = v0.clone();
        let a2800 = {
            let (h2798, _) = v1.clone();
            h2798
        };
        index_bytes(a2799, a2800)
    };
    match m2801 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2803 = {
                    let (h2802, _) = v1.clone();
                    h2802
                };
                let a2804 = 1u64;
                nat_add(a2803, a2804)?
            }, {
                let a2807 = {
                    let (_, h2805) = v1;
                    h2805
                };
                let a2808 = f441(v0, {
                    let (h2806, _) = v1;
                    h2806
                }, v2)?;
                append_bytes(a2807, a2808)
            })))
        }
    }
}

pub fn f408(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f442(Fn11::F443, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f409(v0: Fn9, p2812: List<List<(Bytes, Bytes)>>, p2813: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m2811 = (p2812, p2813);
    {
        let mut m2811 = m2811;
        loop {
            let step = {
                let (v1, v2) = m2811;
                let m2809 = v2;
                match m2809.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c2810: Fn9 = v0.clone();
                            c2810.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2811 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f410(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f411(v0: Fn10, p2818: u64, p2819: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2817 = (p2818, p2819);
    {
        let mut m2817 = m2817;
        loop {
            let step = {
                let (v1, v2) = m2817;
                let m2814 = v1;
                if m2814 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2814 - 1;
                    let m2816 = {
                        let c2815: Fn10 = v0.clone();
                        c2815.apply(v2.clone())?
                    };
                    match m2816 {
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
                Ok(next) => { m2817 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f412(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f444(v0)
}

pub fn f413(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    if f445(v1, f85(v0, v2)?) {
        Ok(Some({
            let a2820 = v2;
            let a2821 = 1u64;
            nat_add(a2820, a2821)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f414(p2824: List<Adt13>, p2825: List<Adt13>) -> List<Adt13> {
    let m2823 = (p2824, p2825);
    {
        let mut m2823 = m2823;
        loop {
            let step = {
                let (v0, v1) = m2823;
                let m2822 = v0;
                match m2822.uncons() {
                    None => {
                        Err::<(List<Adt13>, List<Adt13>), List<Adt13>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt13>, List<Adt13>), List<Adt13>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2823 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f415(v0: Bytes, p2830: List<(Bytes, Adt14)>) -> Option<Adt14> {
    let m2829 = p2830;
    {
        let mut m2829 = m2829;
        loop {
            let step = {
                let v1 = m2829;
                let m2826 = v1;
                match m2826.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt14)>, Option<Adt14>>(None::<Adt14>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h2827, _) = v2;
                            h2827
                        }, v0.clone()) {
                            Err::<List<(Bytes, Adt14)>, Option<Adt14>>(Some({
                                let (_, h2828) = v2;
                                h2828
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt14)>, Option<Adt14>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2829 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f416() -> List<(Bytes, Adt14)> {
    List::cons((f8(Str::lit("boolean")), f265(f8(Str::lit("Bool")))), List::cons((f8(Str::lit("byte")), f265(f8(Str::lit("Int")))), List::cons((f8(Str::lit("octet")), f265(f8(Str::lit("Nat")))), List::cons((f8(Str::lit("bigint")), f265(f8(Str::lit("Int")))), List::cons((f8(Str::lit("float")), f265(f381())), List::cons((f8(Str::lit("double")), f265(f381())), List::cons((f8(Str::lit("DOMString")), f265(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("USVString")), f265(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ByteString")), f265(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ArrayBuffer")), f265(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("SharedArrayBuffer")), f265(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("DataView")), f265(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("Int8Array")), f265(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int16Array")), f265(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int32Array")), f265(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("BigInt64Array")), f265(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Uint8Array")), f265(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint16Array")), f265(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint32Array")), f265(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint8ClampedArray")), f265(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("BigUint64Array")), f265(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Float16Array")), f356(f8(Str::lit("Float16Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("Float32Array")), f356(f8(Str::lit("Float32Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("Float64Array")), f356(f8(Str::lit("Float64Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("any")), f356(f8(Str::lit("any is not translated, because it is the union of every Web IDL type and has no one LexLean type (Web IDL 2.13.1)")))), List::cons((f8(Str::lit("object")), f356(f8(Str::lit("object is not translated, because its values are references to JavaScript objects of any shape, which no LexLean type holds (Web IDL 2.13.20)")))), List::cons((f8(Str::lit("symbol")), f356(f8(Str::lit("symbol is not translated, because a symbol value has identity but no content that a LexLean type holds (Web IDL 2.13.21)")))), List::cons((f8(Str::lit("undefined")), f356(f8(Str::lit("undefined is not translated, because it is the type of the absence of a value (Web IDL 2.13.2)")))), List::<(Bytes, Adt14)>::nil()))))))))))))))))))))))))))))
}

pub fn f417() -> Bytes {
    f8(Str::lit("a union type is translated only as the whole body of a typedef, whose identifier names its inductive (Web IDL 2.11 and 2.13.32)"))
}

pub fn f418(p2841: u64, v1: Bytes, v2: Bytes, p2842: u64) -> R<bool> {
    let m2840 = (p2841, p2842);
    {
        let mut m2840 = m2840;
        loop {
            let step = {
                let (v0, v3) = m2840;
                let m2831 = v0;
                if m2831 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(false))
                } else {
                    let v4 = m2831 - 1;
                    let v5: u64 = f446(v1.clone(), v3)?;
                    let v6: u64 = f446(v2.clone(), v3)?;
                    let a2832 = v5;
                    let a2833 = v6;
                    if nat_lt(a2832, a2833) {
                        Err::<(u64, u64), R<bool>>(Ok(true))
                    } else {
                        let a2834 = v6;
                        let a2835 = v5;
                        if nat_lt(a2834, a2835) {
                            Err::<(u64, u64), R<bool>>(Ok(false))
                        } else {
                            let a2836 = v5;
                            let a2837 = 0u64;
                            if nat_eq(a2836, a2837) {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            } else {
                                Ok::<(u64, u64), R<bool>>((v4, {
                                    let a2838 = v3;
                                    let a2839 = 1u64;
                                    nat_add(a2838, a2839)?
                                }))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2840 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f419(p2855: u64, v1: Bytes, p2856: u64, p2857: Bytes) -> R<Option<Bytes>> {
    let m2854 = (p2855, (p2856, p2857));
    {
        let mut m2854 = m2854;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2854;
                let m2843 = v0;
                if m2843 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m2843 - 1;
                    let a2845 = {
                        let a2844 = v1.clone();
                        length_bytes(a2844)
                    };
                    let a2846 = v2;
                    if nat_le(a2845, a2846) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m2847 = f288(v1.clone(), v2)?;
                        match m2847 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a2849 = v2;
                                    let a2850 = {
                                        let (_, h2848) = v5;
                                        h2848
                                    };
                                    nat_add(a2849, a2850)?
                                }, {
                                    let a2852 = v3;
                                    let a2853 = f447(v1.clone(), v2, {
                                        let (h2851, _) = v5;
                                        h2851
                                    });
                                    append_bytes(a2852, a2853)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2854 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f420(v0: u64) -> Bytes {
    let a2858 = f8(Str::lit("u"));
    let a2859 = f448(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a2858, a2859)
}

pub fn f421() -> Bytes {
    f8(Str::lit("\u{0}\u{0}:\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{5}\u{7f}\u{0}\u{6}p\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f422() -> Bytes {
    f8(Str::lit("\u{0}\u{0}-\u{0}\u{0}.\u{0}\u{0}0\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}7\u{0}\u{1}7\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@?\u{0}@@\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f423(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f85(v0, {
        let (h2860, _) = v2;
        h2860
    })?;
    let a2862 = 1114111u64;
    let a2863 = {
        let (_, h2861) = v2;
        h2861
    };
    if nat_lt(a2862, a2863) {
        Ok(None::<(u64, u64)>)
    } else {
        let v4: u64 = f449(v3, v1);
        let a2864 = v4;
        let a2865 = 16u64;
        if nat_eq(a2864, a2865) {
            Ok(None::<(u64, u64)>)
        } else {
            Ok(Some(({
                let a2867 = {
                    let (h2866, _) = v2;
                    h2866
                };
                let a2868 = 1u64;
                nat_add(a2867, a2868)?
            }, {
                let a2872 = {
                    let a2870 = {
                        let (_, h2869) = v2;
                        h2869
                    };
                    let a2871 = if v1 {
                        16u64
                    } else {
                        10u64
                    };
                    nat_mul(a2870, a2871)?
                };
                let a2873 = v4;
                nat_add(a2872, a2873)?
            })))
        }
    }
}

pub fn f424() -> Bytes {
    f8(Str::lit("\u{80}\u{81}\u{82}\u{83}\u{84}\u{85}\u{86}\u{87}\u{88}\u{89}\u{8a}\u{8b}\u{8c}\u{8d}\u{8e}\u{8f}\u{90}\u{91}\u{92}\u{93}\u{94}\u{95}\u{96}\u{97}\u{98}\u{99}\u{9a}\u{9b}\u{9c}\u{9d}\u{9e}\u{9f}\u{a0}\u{a1}\u{a2}\u{a3}\u{a4}\u{a5}\u{a6}\u{a7}\u{a8}\u{a9}\u{aa}\u{ab}\u{ac}\u{ad}\u{ae}\u{af}\u{b0}\u{b1}\u{b2}\u{b3}\u{b4}\u{b5}\u{b6}\u{b7}\u{b8}\u{b9}\u{ba}\u{bb}\u{bc}\u{bd}\u{be}\u{bf}"))
}

pub fn f425(v0: u64, v1: bool, v2: u64) -> R<bool> {
    if v1 {
        let a2874 = v0;
        let a2875 = v2;
        if nat_eq(a2874, a2875) {
            Ok(true)
        } else {
            f208(v0, f450())
        }
    } else {
        f208(v0, f451())
    }
}

pub fn f426() -> Bytes {
    f8(Str::lit(" string:anyAtomicType boolean:anyAtomicType decimal:anyAtomicType float:anyAtomicType double:anyAtomicType duration:anyAtomicType dateTime:anyAtomicType time:anyAtomicType date:anyAtomicType gYearMonth:anyAtomicType gYear:anyAtomicType gMonthDay:anyAtomicType gDay:anyAtomicType gMonth:anyAtomicType hexBinary:anyAtomicType base64Binary:anyAtomicType anyURI:anyAtomicType QName:anyAtomicType NOTATION:anyAtomicType normalizedString:string token:normalizedString language:token NMTOKEN:token Name:token NCName:Name ID:NCName IDREF:NCName ENTITY:NCName integer:decimal nonPositiveInteger:integer long:integer nonNegativeInteger:integer negativeInteger:nonPositiveInteger int:long short:int byte:short unsignedLong:nonNegativeInteger positiveInteger:nonNegativeInteger unsignedInt:unsignedLong unsignedShort:unsignedInt unsignedByte:unsignedShort yearMonthDuration:duration dayTimeDuration:duration dateTimeStamp:dateTime NMTOKENS:anySimpleType IDREFS:anySimpleType ENTITIES:anySimpleType anyAtomicType:anySimpleType "))
}

pub fn f427(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f452(v1, v0, v2)
}

pub fn f428(v0: u64, v1: Bytes, v2: u64, v3: u64) -> R<Option<u64>> {
    f453(v1, v2, v0, v3)
}

pub fn f429(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let a2878 = {
        let (h2876, _) = v1;
        h2876
    };
    let a2879 = {
        let a2877 = v0.clone();
        length_bytes(a2877)
    };
    if nat_lt(a2878, a2879) {
        Ok(Some((f167(v0, {
            let (h2880, _) = v1;
            h2880
        })?, {
            let a2882 = {
                let (_, h2881) = v1;
                h2881
            };
            let a2883 = 1u64;
            nat_add(a2882, a2883)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f430(v0: u64) -> Option<u64> {
    let a2884 = 48u64;
    let a2885 = v0;
    if if nat_le(a2884, a2885) {
        let a2886 = v0;
        let a2887 = 57u64;
        nat_le(a2886, a2887)
    } else {
        false
    } {
        Some({
            let a2888 = v0;
            let a2889 = 48u64;
            nat_sub(a2888, a2889)
        })
    } else {
        let a2890 = 97u64;
        let a2891 = v0;
        if if nat_le(a2890, a2891) {
            let a2892 = v0;
            let a2893 = 102u64;
            nat_le(a2892, a2893)
        } else {
            false
        } {
            Some({
                let a2894 = v0;
                let a2895 = 87u64;
                nat_sub(a2894, a2895)
            })
        } else {
            let a2896 = 65u64;
            let a2897 = v0;
            if if nat_le(a2896, a2897) {
                let a2898 = v0;
                let a2899 = 70u64;
                nat_le(a2898, a2899)
            } else {
                false
            } {
                Some({
                    let a2900 = v0;
                    let a2901 = 55u64;
                    nat_sub(a2900, a2901)
                })
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f431(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f454(v0, v1)
}

pub fn f432(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f455(v0, v1)
}

pub fn f433(v0: Bytes, v1: u64, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    let v3: u64 = {
        let (h2902, _) = v2;
        h2902
    };
    let v4: u64 = f100(v0.clone(), v3)?;
    let (_, h2903) = v2;
    if h2903 {
        Ok(None::<(u64, bool)>)
    } else {
        if f157(v4) {
            Ok(Some(({
                let a2904 = v3;
                let a2905 = 1u64;
                nat_add(a2904, a2905)?
            }, false)))
        } else {
            let a2906 = v4;
            let a2907 = 35u64;
            if if nat_eq(a2906, a2907) {
                let a2908 = v3;
                let a2909 = 0u64;
                if if nat_eq(a2908, a2909) {
                    true
                } else {
                    f157(f100(v0.clone(), {
                        let a2910 = v3;
                        let a2911 = 1u64;
                        nat_sub(a2910, a2911)
                    })?)
                } {
                    true
                } else {
                    let a2914 = f100(v0.clone(), {
                        let a2912 = v3;
                        let a2913 = 1u64;
                        nat_sub(a2912, a2913)
                    })?;
                    let a2915 = 10u64;
                    nat_eq(a2914, a2915)
                }
            } else {
                false
            } {
                Ok(Some((f238(v0, v3)?, false)))
            } else {
                let a2916 = v4;
                let a2917 = 10u64;
                if nat_eq(a2916, a2917) {
                    let v5: u64 = {
                        let a2918 = v3;
                        let a2919 = 1u64;
                        nat_add(a2918, a2919)?
                    };
                    let v6: u64 = f160(v0.clone(), v5)?;
                    let v7: u64 = f161(v0.clone(), v6)?;
                    if if f99(v0.clone(), v5)? {
                        true
                    } else {
                        let a2922 = {
                            let a2920 = v6;
                            let a2921 = v5;
                            nat_sub(a2920, a2921)
                        };
                        let a2923 = v1;
                        if if nat_lt(a2922, a2923) {
                            f169(f158(f100(v0.clone(), v7)?))
                        } else {
                            false
                        } {
                            f169({
                                let a2924 = f100(v0, v7)?;
                                let a2925 = 35u64;
                                nat_eq(a2924, a2925)
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

pub fn f434(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f161(v0.clone(), {
        let (h2926, _) = v1;
        h2926
    })?;
    let a2927 = f100(v0, v2)?;
    let a2928 = 10u64;
    if nat_eq(a2927, a2928) {
        Ok(Some(({
            let a2929 = v2;
            let a2930 = 1u64;
            nat_add(a2929, a2930)?
        }, {
            let a2932 = {
                let (_, h2931) = v1;
                h2931
            };
            let a2933 = 1u64;
            nat_add(a2932, a2933)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f435(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f456(v1, v0, v2)
}

pub fn f436(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f457(v0, v1)
}

pub fn f437(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f458(v0, v1)
}

pub fn f438(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a2934 = v0.clone();
        length_bytes(a2934)
    };
    let a2935 = 0u64;
    let a2936 = v1;
    if nat_lt(a2935, a2936) {
        let a2937 = f459(v0.clone())?;
        let a2938 = v1;
        if nat_eq(a2937, a2938) {
            let a2939 = 1u64;
            let a2940 = v1;
            if nat_lt(a2939, a2940) {
                let m2943 = {
                    let a2941 = v0;
                    let a2942 = 0u64;
                    index_bytes(a2941, a2942)
                };
                match m2943 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a2944 = v2;
                        let a2945 = 48u8;
                        Ok(!equal(a2944, a2945))
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

pub fn f439(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f460(v0, v1)
}

pub fn f440(v0: Bytes) -> R<u64> {
    let (h2946, _) = f128(Fn4::F461(v0.clone()), {
        let a2948 = {
            let a2947 = v0;
            length_bytes(a2947)
        };
        let a2949 = 1u64;
        nat_add(a2948, a2949)?
    }, 0u64)?;
    Ok(h2946)
}

pub fn f441(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a2994 = {
            let a2970 = {
                let a2958 = {
                    let a2952 = {
                        let a2950 = v3;
                        let a2951 = 128u8;
                        fixed_u8::bit_and(a2950, a2951)
                    };
                    let a2953 = 128u8;
                    if equal(a2952, a2953) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2959 = {
                    let a2956 = {
                        let a2954 = v3;
                        let a2955 = 64u8;
                        fixed_u8::bit_and(a2954, a2955)
                    };
                    let a2957 = 64u8;
                    if equal(a2956, a2957) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2958, a2959)?
            };
            let a2971 = {
                let a2968 = {
                    let a2962 = {
                        let a2960 = v3;
                        let a2961 = 32u8;
                        fixed_u8::bit_and(a2960, a2961)
                    };
                    let a2963 = 32u8;
                    if equal(a2962, a2963) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2969 = {
                    let a2966 = {
                        let a2964 = v3;
                        let a2965 = 16u8;
                        fixed_u8::bit_and(a2964, a2965)
                    };
                    let a2967 = 16u8;
                    if equal(a2966, a2967) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2968, a2969)?
            };
            nat_add(a2970, a2971)?
        };
        let a2995 = {
            let a2992 = {
                let a2980 = {
                    let a2974 = {
                        let a2972 = v3;
                        let a2973 = 8u8;
                        fixed_u8::bit_and(a2972, a2973)
                    };
                    let a2975 = 8u8;
                    if equal(a2974, a2975) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2981 = {
                    let a2978 = {
                        let a2976 = v3;
                        let a2977 = 4u8;
                        fixed_u8::bit_and(a2976, a2977)
                    };
                    let a2979 = 4u8;
                    if equal(a2978, a2979) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2980, a2981)?
            };
            let a2993 = {
                let a2990 = {
                    let a2984 = {
                        let a2982 = v3;
                        let a2983 = 2u8;
                        fixed_u8::bit_and(a2982, a2983)
                    };
                    let a2985 = 2u8;
                    if equal(a2984, a2985) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2991 = {
                    let a2988 = {
                        let a2986 = v3;
                        let a2987 = 1u8;
                        fixed_u8::bit_and(a2986, a2987)
                    };
                    let a2989 = 1u8;
                    if equal(a2988, a2989) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2990, a2991)?
            };
            nat_add(a2992, a2993)?
        };
        nat_add(a2994, a2995)?
    };
    let a2996 = v4;
    let a2997 = 34u64;
    if nat_eq(a2996, a2997) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a2998 = v4;
        let a2999 = 92u64;
        if nat_eq(a2998, a2999) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a3000 = v4;
            let a3001 = 8u64;
            if nat_eq(a3000, a3001) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a3002 = v4;
                let a3003 = 12u64;
                if nat_eq(a3002, a3003) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a3004 = v4;
                    let a3005 = 10u64;
                    if nat_eq(a3004, a3005) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a3006 = v4;
                        let a3007 = 13u64;
                        if nat_eq(a3006, a3007) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a3008 = v4;
                            let a3009 = 9u64;
                            if nat_eq(a3008, a3009) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a3010 = v4;
                                let a3011 = 32u64;
                                if nat_lt(a3010, a3011) {
                                    let a3012 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a3013 = f462(v2)?;
                                    Ok(append_bytes(a3012, a3013))
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

pub fn f442(v0: Fn11, p3017: List<List<(Bytes, Bytes)>>, p3018: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m3016 = (p3017, p3018);
    {
        let mut m3016 = m3016;
        loop {
            let step = {
                let (v1, v2) = m3016;
                let m3014 = v2;
                match m3014.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c3015: Fn11 = v0.clone();
                            c3015.apply(v1, v3)
                        }, v4))
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

pub fn f443(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f444(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a3020 = {
        let a3019 = v0.clone();
        length_list(a3019)
    };
    let a3021 = 1u64;
    if nat_le(a3020, a3021) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f463(v0)?))
    }
}

pub fn f445(v0: u64, v1: u64) -> bool {
    let a3022 = v0;
    let a3023 = 0u64;
    if nat_eq(a3022, a3023) {
        f342(v1)
    } else {
        let a3024 = v0;
        let a3025 = 1u64;
        if nat_eq(a3024, a3025) {
            f341(v1)
        } else {
            let a3026 = v0;
            let a3027 = 2u64;
            if nat_eq(a3026, a3027) {
                f464(v1)
            } else {
                f465(v1)
            }
        }
    }
}

pub fn f446(v0: Bytes, v1: u64) -> R<u64> {
    let a3029 = v1;
    let a3030 = {
        let a3028 = v0.clone();
        length_bytes(a3028)
    };
    if nat_lt(a3029, a3030) {
        let a3031 = f85(v0, v1)?;
        let a3032 = 1u64;
        nat_add(a3031, a3032)
    } else {
        Ok(0u64)
    }
}

pub fn f447(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f304(f302(f304(f303(v2), f305(v2)), f302({
        let a3033 = v2;
        let a3034 = 117u64;
        !nat_eq(a3033, a3034)
    }, {
        let a3035 = v2;
        let a3036 = 85u64;
        !nat_eq(a3035, a3036)
    })), f302(f306(v2), {
        let a3037 = 0u64;
        let a3038 = v1;
        nat_lt(a3037, a3038)
    })) {
        f32(v0, v1, 1u64)
    } else {
        let a3039 = v2;
        let a3040 = 65536u64;
        if nat_lt(a3039, a3040) {
            let a3041 = f8(Str::lit("u"));
            let a3042 = f448(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a3041, a3042)
        } else {
            let a3043 = f8(Str::lit("U"));
            let a3044 = f448(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a3043, a3044)
        }
    }
}

pub fn f448(p3061: u64, p3062: u64, p3063: u64, p3064: Bytes) -> Bytes {
    let m3060 = (p3061, (p3062, (p3063, p3064)));
    {
        let mut m3060 = m3060;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m3060;
                let m3045 = v0;
                if m3045 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m3045 - 1;
                    if f302({
                        let a3046 = v1;
                        let a3047 = 0u64;
                        nat_eq(a3046, a3047)
                    }, {
                        let a3048 = v2;
                        let a3049 = 0u64;
                        nat_eq(a3048, a3049)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a3050 = v1;
                            let a3051 = 16u64;
                            let a3052 = 0u64;
                            nat_quot(a3050, a3051, a3052)
                        }, ({
                            let a3053 = v2;
                            let a3054 = 1u64;
                            nat_sub(a3053, a3054)
                        }, {
                            let a3058 = f466({
                                let a3055 = v1;
                                let a3056 = 16u64;
                                let a3057 = 0u64;
                                nat_rem(a3055, a3056, a3057)
                            });
                            let a3059 = v3;
                            append_bytes(a3058, a3059)
                        }))))
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

pub fn f449(v0: u64, v1: bool) -> u64 {
    let a3065 = v0;
    let a3066 = 48u64;
    if nat_lt(a3065, a3066) {
        16u64
    } else {
        let a3067 = v0;
        let a3068 = 57u64;
        if nat_le(a3067, a3068) {
            let a3069 = v0;
            let a3070 = 48u64;
            nat_sub(a3069, a3070)
        } else {
            if v1 {
                let a3071 = v0;
                let a3072 = 65u64;
                if nat_lt(a3071, a3072) {
                    16u64
                } else {
                    let a3073 = v0;
                    let a3074 = 70u64;
                    if nat_le(a3073, a3074) {
                        let a3075 = v0;
                        let a3076 = 55u64;
                        nat_sub(a3075, a3076)
                    } else {
                        let a3077 = v0;
                        let a3078 = 97u64;
                        if nat_lt(a3077, a3078) {
                            16u64
                        } else {
                            let a3079 = v0;
                            let a3080 = 102u64;
                            if nat_le(a3079, a3080) {
                                let a3081 = v0;
                                let a3082 = 87u64;
                                nat_sub(a3081, a3082)
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

pub fn f450() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f451() -> Bytes {
    f8(Str::lit("\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{0}]\u{0}\u{0}]\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f452(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f385(v0.clone(), {
        let (h3083, _) = v2;
        h3083
    }, v1)?;
    let a3084 = f100(v0, v3)?;
    let a3085 = 10u64;
    if nat_eq(a3084, a3085) {
        Ok(Some(({
            let a3086 = v3;
            let a3087 = 1u64;
            nat_add(a3086, a3087)?
        }, {
            let a3089 = {
                let (_, h3088) = v2;
                h3088
            };
            let a3090 = 1u64;
            nat_add(a3089, a3090)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f453(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a3091 = f100(v0, v3)?;
    let a3092 = 32u64;
    if if nat_eq(a3091, a3092) {
        let a3095 = {
            let a3093 = v3;
            let a3094 = v1;
            nat_sub(a3093, a3094)
        };
        let a3096 = v2;
        nat_lt(a3095, a3096)
    } else {
        false
    } {
        Ok(Some({
            let a3097 = v3;
            let a3098 = 1u64;
            nat_add(a3097, a3098)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f454(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f100(v0, v1)?;
    let a3099 = v2;
    let a3100 = 34u64;
    if if if if nat_eq(a3099, a3100) {
        true
    } else {
        let a3101 = v2;
        let a3102 = 92u64;
        nat_eq(a3101, a3102)
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
            let a3103 = v1;
            let a3104 = 1u64;
            nat_add(a3103, a3104)?
        }))
    }
}

pub fn f455(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f100(v0, v1)?;
    let a3105 = v2;
    let a3106 = 39u64;
    if if if nat_eq(a3105, a3106) {
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
            let a3107 = v1;
            let a3108 = 1u64;
            nat_add(a3107, a3108)?
        }))
    }
}

pub fn f456(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m3110 = f430(f100(v0, {
        let (h3109, _) = v2;
        h3109
    })?);
    match m3110 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v3) => {
            let a3111 = v3;
            let a3112 = v1;
            if nat_lt(a3111, a3112) {
                Ok(Some(({
                    let a3114 = {
                        let (h3113, _) = v2;
                        h3113
                    };
                    let a3115 = 1u64;
                    nat_add(a3114, a3115)?
                }, {
                    let a3119 = {
                        let a3117 = {
                            let (_, h3116) = v2;
                            h3116
                        };
                        let a3118 = v1;
                        nat_mul(a3117, a3118)?
                    };
                    let a3120 = v3;
                    nat_add(a3119, a3120)?
                })))
            } else {
                Ok(None::<(u64, u64)>)
            }
        }
    }
}

pub fn f457(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f137(v0, v1)?;
    let a3121 = 47u64;
    let a3122 = v2;
    if if nat_lt(a3121, a3122) {
        let a3123 = v2;
        let a3124 = 58u64;
        nat_lt(a3123, a3124)
    } else {
        false
    } {
        Ok(Some({
            let a3125 = v1;
            let a3126 = 1u64;
            nat_add(a3125, a3126)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f458(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a3127 = f137(v0, v1)?;
    let a3128 = 48u64;
    if nat_eq(a3127, a3128) {
        Ok(Some({
            let a3129 = v1;
            let a3130 = 1u64;
            nat_add(a3129, a3130)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f459(v0: Bytes) -> R<u64> {
    let (h3131, _) = f128(Fn4::F467(v0.clone()), {
        let a3132 = v0;
        length_bytes(a3132)
    }, 0u64)?;
    Ok(h3131)
}

pub fn f460(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m3136 = {
        let a3134 = v0;
        let a3135 = {
            let (h3133, _) = v1;
            h3133
        };
        index_bytes(a3134, a3135)
    };
    match m3136 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a3138 = {
                    let (h3137, _) = v1;
                    h3137
                };
                let a3139 = 1u64;
                nat_add(a3138, a3139)?
            }, {
                let a3143 = {
                    let a3141 = {
                        let (_, h3140) = v1;
                        h3140
                    };
                    let a3142 = 10u64;
                    nat_mul(a3141, a3142)?
                };
                let a3144 = f468(v2)?;
                nat_add(a3143, a3144)?
            })))
        }
    }
}

pub fn f461(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f469(v0, v1)
}

pub fn f462(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3189 = {
            let a3165 = {
                let a3153 = {
                    let a3147 = {
                        let a3145 = v1;
                        let a3146 = 128u8;
                        fixed_u8::bit_and(a3145, a3146)
                    };
                    let a3148 = 128u8;
                    if equal(a3147, a3148) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3154 = {
                    let a3151 = {
                        let a3149 = v1;
                        let a3150 = 64u8;
                        fixed_u8::bit_and(a3149, a3150)
                    };
                    let a3152 = 64u8;
                    if equal(a3151, a3152) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3153, a3154)?
            };
            let a3166 = {
                let a3163 = {
                    let a3157 = {
                        let a3155 = v1;
                        let a3156 = 32u8;
                        fixed_u8::bit_and(a3155, a3156)
                    };
                    let a3158 = 32u8;
                    if equal(a3157, a3158) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3164 = {
                    let a3161 = {
                        let a3159 = v1;
                        let a3160 = 16u8;
                        fixed_u8::bit_and(a3159, a3160)
                    };
                    let a3162 = 16u8;
                    if equal(a3161, a3162) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3163, a3164)?
            };
            nat_add(a3165, a3166)?
        };
        let a3190 = {
            let a3187 = {
                let a3175 = {
                    let a3169 = {
                        let a3167 = v1;
                        let a3168 = 8u8;
                        fixed_u8::bit_and(a3167, a3168)
                    };
                    let a3170 = 8u8;
                    if equal(a3169, a3170) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3176 = {
                    let a3173 = {
                        let a3171 = v1;
                        let a3172 = 4u8;
                        fixed_u8::bit_and(a3171, a3172)
                    };
                    let a3174 = 4u8;
                    if equal(a3173, a3174) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3175, a3176)?
            };
            let a3188 = {
                let a3185 = {
                    let a3179 = {
                        let a3177 = v1;
                        let a3178 = 2u8;
                        fixed_u8::bit_and(a3177, a3178)
                    };
                    let a3180 = 2u8;
                    if equal(a3179, a3180) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3186 = {
                    let a3183 = {
                        let a3181 = v1;
                        let a3182 = 1u8;
                        fixed_u8::bit_and(a3181, a3182)
                    };
                    let a3184 = 1u8;
                    if equal(a3183, a3184) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3185, a3186)?
            };
            nat_add(a3187, a3188)?
        };
        nat_add(a3189, a3190)?
    };
    let a3197 = f124(f470(), {
        let a3191 = v2;
        let a3192 = 16u64;
        let a3193 = 0u64;
        nat_quot(a3191, a3192, a3193)
    });
    let a3198 = f124(f470(), {
        let a3194 = v2;
        let a3195 = 16u64;
        let a3196 = 0u64;
        nat_rem(a3194, a3195, a3196)
    });
    Ok(append_bytes(a3197, a3198))
}

pub fn f463(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f408({
        let (_, h3199) = {
            let (h3200, _) = f471(Fn12::F472, {
                let a3202 = {
                    let a3201 = v0.clone();
                    length_list(a3201)
                };
                let a3203 = 1u64;
                nat_add(a3202, a3203)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h3200
        };
        h3199
    }))
}

pub fn f464(v0: u64) -> bool {
    let a3204 = 48u64;
    let a3205 = v0;
    if nat_le(a3204, a3205) {
        let a3206 = v0;
        let a3207 = 55u64;
        nat_le(a3206, a3207)
    } else {
        false
    }
}

pub fn f465(v0: u64) -> bool {
    if f250(v0) {
        true
    } else {
        if f342(v0) {
            true
        } else {
            let a3208 = v0;
            let a3209 = 95u64;
            if nat_eq(a3208, a3209) {
                true
            } else {
                let a3210 = v0;
                let a3211 = 45u64;
                nat_eq(a3210, a3211)
            }
        }
    }
}

pub fn f466(v0: u64) -> Bytes {
    f32({
        let a3212 = Str::lit("0123456789ABCDEF");
        utf8_encode(a3212)
    }, v0, 1u64)
}

pub fn f467(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3215 = {
        let a3213 = v0;
        let a3214 = v1;
        index_bytes(a3213, a3214)
    };
    match m3215 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f473(v2)? {
                Ok(Some({
                    let a3216 = v1;
                    let a3217 = 1u64;
                    nat_add(a3216, a3217)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f468(v0: u8) -> R<u64> {
    let a3264 = {
        let v1: u8 = v0;
        let a3262 = {
            let a3238 = {
                let a3226 = {
                    let a3220 = {
                        let a3218 = v1;
                        let a3219 = 128u8;
                        fixed_u8::bit_and(a3218, a3219)
                    };
                    let a3221 = 128u8;
                    if equal(a3220, a3221) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3227 = {
                    let a3224 = {
                        let a3222 = v1;
                        let a3223 = 64u8;
                        fixed_u8::bit_and(a3222, a3223)
                    };
                    let a3225 = 64u8;
                    if equal(a3224, a3225) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3226, a3227)?
            };
            let a3239 = {
                let a3236 = {
                    let a3230 = {
                        let a3228 = v1;
                        let a3229 = 32u8;
                        fixed_u8::bit_and(a3228, a3229)
                    };
                    let a3231 = 32u8;
                    if equal(a3230, a3231) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3237 = {
                    let a3234 = {
                        let a3232 = v1;
                        let a3233 = 16u8;
                        fixed_u8::bit_and(a3232, a3233)
                    };
                    let a3235 = 16u8;
                    if equal(a3234, a3235) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3236, a3237)?
            };
            nat_add(a3238, a3239)?
        };
        let a3263 = {
            let a3260 = {
                let a3248 = {
                    let a3242 = {
                        let a3240 = v1;
                        let a3241 = 8u8;
                        fixed_u8::bit_and(a3240, a3241)
                    };
                    let a3243 = 8u8;
                    if equal(a3242, a3243) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3249 = {
                    let a3246 = {
                        let a3244 = v1;
                        let a3245 = 4u8;
                        fixed_u8::bit_and(a3244, a3245)
                    };
                    let a3247 = 4u8;
                    if equal(a3246, a3247) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3248, a3249)?
            };
            let a3261 = {
                let a3258 = {
                    let a3252 = {
                        let a3250 = v1;
                        let a3251 = 2u8;
                        fixed_u8::bit_and(a3250, a3251)
                    };
                    let a3253 = 2u8;
                    if equal(a3252, a3253) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3259 = {
                    let a3256 = {
                        let a3254 = v1;
                        let a3255 = 1u8;
                        fixed_u8::bit_and(a3254, a3255)
                    };
                    let a3257 = 1u8;
                    if equal(a3256, a3257) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3258, a3259)?
            };
            nat_add(a3260, a3261)?
        };
        nat_add(a3262, a3263)?
    };
    let a3265 = 48u64;
    Ok(nat_sub(a3264, a3265))
}

pub fn f469(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a3266 = v0.clone();
        length_bytes(a3266)
    };
    let a3267 = v1;
    let a3268 = v2;
    if if nat_lt(a3267, a3268) {
        let a3273 = f137(v0, {
            let a3271 = v2;
            let a3272 = {
                let a3269 = v1;
                let a3270 = 1u64;
                nat_add(a3269, a3270)?
            };
            nat_sub(a3271, a3272)
        })?;
        let a3274 = 48u64;
        nat_eq(a3273, a3274)
    } else {
        false
    } {
        Ok(Some({
            let a3275 = v1;
            let a3276 = 1u64;
            nat_add(a3275, a3276)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f470() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f471(v0: Fn12, p3281: u64, p3282: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m3280 = (p3281, p3282);
    {
        let mut m3280 = m3280;
        loop {
            let step = {
                let (v1, v2) = m3280;
                let m3277 = v1;
                if m3277 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3277 - 1;
                    let m3279 = {
                        let c3278: Fn12 = v0.clone();
                        c3278.apply(v2.clone())?
                    };
                    match m3279 {
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
                Ok(next) => { m3280 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f472(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f474(v0)
}

pub fn f473(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3327 = {
            let a3303 = {
                let a3291 = {
                    let a3285 = {
                        let a3283 = v1;
                        let a3284 = 128u8;
                        fixed_u8::bit_and(a3283, a3284)
                    };
                    let a3286 = 128u8;
                    if equal(a3285, a3286) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3292 = {
                    let a3289 = {
                        let a3287 = v1;
                        let a3288 = 64u8;
                        fixed_u8::bit_and(a3287, a3288)
                    };
                    let a3290 = 64u8;
                    if equal(a3289, a3290) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3291, a3292)?
            };
            let a3304 = {
                let a3301 = {
                    let a3295 = {
                        let a3293 = v1;
                        let a3294 = 32u8;
                        fixed_u8::bit_and(a3293, a3294)
                    };
                    let a3296 = 32u8;
                    if equal(a3295, a3296) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3302 = {
                    let a3299 = {
                        let a3297 = v1;
                        let a3298 = 16u8;
                        fixed_u8::bit_and(a3297, a3298)
                    };
                    let a3300 = 16u8;
                    if equal(a3299, a3300) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3301, a3302)?
            };
            nat_add(a3303, a3304)?
        };
        let a3328 = {
            let a3325 = {
                let a3313 = {
                    let a3307 = {
                        let a3305 = v1;
                        let a3306 = 8u8;
                        fixed_u8::bit_and(a3305, a3306)
                    };
                    let a3308 = 8u8;
                    if equal(a3307, a3308) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3314 = {
                    let a3311 = {
                        let a3309 = v1;
                        let a3310 = 4u8;
                        fixed_u8::bit_and(a3309, a3310)
                    };
                    let a3312 = 4u8;
                    if equal(a3311, a3312) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3313, a3314)?
            };
            let a3326 = {
                let a3323 = {
                    let a3317 = {
                        let a3315 = v1;
                        let a3316 = 2u8;
                        fixed_u8::bit_and(a3315, a3316)
                    };
                    let a3318 = 2u8;
                    if equal(a3317, a3318) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3324 = {
                    let a3321 = {
                        let a3319 = v1;
                        let a3320 = 1u8;
                        fixed_u8::bit_and(a3319, a3320)
                    };
                    let a3322 = 1u8;
                    if equal(a3321, a3322) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3323, a3324)?
            };
            nat_add(a3325, a3326)?
        };
        nat_add(a3327, a3328)?
    };
    let a3329 = 48u64;
    let a3330 = v2;
    if nat_le(a3329, a3330) {
        let a3331 = v2;
        let a3332 = 57u64;
        Ok(nat_le(a3331, a3332))
    } else {
        Ok(false)
    }
}

pub fn f474(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h3333, _) = v0;
        h3333
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h3334) = v0;
        h3334
    };
    let m3335 = v1;
    match m3335.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m3336 = v4;
            match m3336.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f475(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f475(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3341 = {
            let a3339 = {
                let a3337 = v0.clone();
                length_list(a3337)
            };
            let a3340 = {
                let a3338 = v1.clone();
                length_list(a3338)
            };
            nat_add(a3339, a3340)?
        };
        let a3342 = 1u64;
        nat_add(a3341, a3342)?
    };
    Ok(f476({
        let (_, h3343) = {
            let (_, h3344) = {
                let (h3345, _) = f477(Fn13::F478, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3345
            };
            h3344
        };
        h3343
    }))
}

pub fn f476(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f243(Fn5::F479, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f477(v0: Fn13, p3350: u64, p3351: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3349 = (p3350, p3351);
    {
        let mut m3349 = m3349;
        loop {
            let step = {
                let (v1, v2) = m3349;
                let m3346 = v1;
                if m3346 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3346 - 1;
                    let m3348 = {
                        let c3347: Fn13 = v0.clone();
                        c3347.apply(v2.clone())
                    };
                    match m3348 {
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
                Ok(next) => { m3349 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f478(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f480(v0)
}

pub fn f479(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f480(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h3352, _) = v0.clone();
        h3352
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h3353, _) = {
            let (_, h3354) = v0.clone();
            h3354
        };
        h3353
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h3355) = {
            let (_, h3356) = v0;
            h3356
        };
        h3355
    };
    let m3357 = v1.clone();
    match m3357.uncons() {
        None => {
            let m3358 = v2;
            match m3358.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3359 = v2.clone();
            match m3359.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f481({
                        let (h3360, _) = v8.clone();
                        h3360
                    }, {
                        let (h3361, _) = v6.clone();
                        h3361
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

pub fn f481(v0: Bytes, v1: Bytes) -> bool {
    let a3366 = {
        let a3362 = v0;
        let a3363 = v1;
        compare_bytes(a3362, a3363)
    };
    let a3367 = {
        let a3364 = Bytes::lit(&[]);
        let a3365 = Bytes::lit(&[0x01]);
        compare_bytes(a3364, a3365)
    };
    equal(a3366, a3367)
}

pub fn run(p0: Option<Adt0>, p1: Adt2) -> R<(Adt0, Adt9)> {
    f0(p0, p1)
}
