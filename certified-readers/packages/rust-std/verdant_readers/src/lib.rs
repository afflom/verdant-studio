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
    C0(u64),
    C1(Bytes),
    C2(Bytes),
}

#[derive(Clone)]
pub enum Adt21 {
    C0(Adt2, u64),
    C1(Bytes),
}

#[derive(Clone)]
pub enum Adt22 {
    C0(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt23 {
    C0(Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt24 {
    C0(List<Adt25>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt25 {
    C0(Bytes, Bytes, Bytes, Adt26),
}

#[derive(Clone)]
pub enum Adt26 {
    C0(Bytes, Bytes, Adt17, Adt17, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Adt27 {
    C0(Bytes, List<Bytes>, Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt28 {
    C0(Bytes, Adt17, Adt17, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Adt29 {
    C0(Bytes, u64),
    C1(Bytes),
}

#[derive(Clone)]
pub enum Adt30 {
    C0(Bytes, List<Adt22>),
}

#[derive(Clone)]
pub enum Adt31 {
    C0(Adt15, List<Adt22>),
}

#[derive(Clone)]
pub enum Adt32 {
    C0(Adt23, List<Adt27>),
}

#[derive(Clone)]
pub enum Adt33 {
    C0(List<Adt34>, List<Adt27>, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt34 {
    C0(Bytes, Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt35 {
    C0(u64, u64),
    C1(Bytes),
}

#[derive(Clone)]
pub enum Adt36 {
    C0(Bytes, u64, u64),
    C1(Bytes),
}

#[derive(Clone)]
pub enum Adt37 {
    C0(Bytes, Adt32),
}

#[derive(Clone)]
pub enum Fn0 {
    F58(u8, Bytes),
    F294(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F58(k0, k1) => f58(*k0, k1.clone(), p0),
            Fn0::F294(k0) => f294(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F61(Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn1::F61(k0) => f61(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F91,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn2::F91 => f91(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F148(Bytes),
    F158(u64, Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn3::F148(k0) => f148(k0.clone(), p0),
            Fn3::F158(k0, k1) => Ok(f158(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F150(Bytes),
    F151(Bytes),
    F178(Bytes),
    F220(Bytes),
    F221(Bytes),
    F279(Bytes, Bytes),
    F280(Bytes),
    F281(Bytes, u64),
    F327(Bytes),
    F332(Bytes),
    F334(Bytes),
    F347(Bytes, u64),
    F392(bool, u64, Bytes),
    F426(Bytes),
    F431(Bytes),
    F433(Bytes),
    F459(Bytes, bool),
    F462(u64, Bytes),
    F593(u64, Bytes, u64),
    F597(Bytes),
    F598(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn4::F150(k0) => f150(k0.clone(), p0),
            Fn4::F151(k0) => f151(k0.clone(), p0),
            Fn4::F178(k0) => f178(k0.clone(), p0),
            Fn4::F220(k0) => f220(k0.clone(), p0),
            Fn4::F221(k0) => f221(k0.clone(), p0),
            Fn4::F279(k0, k1) => f279(k0.clone(), k1.clone(), p0),
            Fn4::F280(k0) => f280(k0.clone(), p0),
            Fn4::F281(k0, k1) => f281(k0.clone(), *k1, p0),
            Fn4::F327(k0) => f327(k0.clone(), p0),
            Fn4::F332(k0) => f332(k0.clone(), p0),
            Fn4::F334(k0) => f334(k0.clone(), p0),
            Fn4::F347(k0, k1) => f347(k0.clone(), *k1, p0),
            Fn4::F392(k0, k1, k2) => f392(*k0, *k1, k2.clone(), p0),
            Fn4::F426(k0) => f426(k0.clone(), p0),
            Fn4::F431(k0) => f431(k0.clone(), p0),
            Fn4::F433(k0) => f433(k0.clone(), p0),
            Fn4::F459(k0, k1) => f459(k0.clone(), *k1, p0),
            Fn4::F462(k0, k1) => f462(*k0, k1.clone(), p0),
            Fn4::F593(k0, k1, k2) => f593(*k0, k1.clone(), *k2, p0),
            Fn4::F597(k0) => f597(k0.clone(), p0),
            Fn4::F598(k0) => f598(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F224(Bytes),
    F439(Bytes),
    F455(bool, Bytes),
    F492(bool, Bytes),
    F532(Bytes),
    F546(Bytes),
    F592(u64, Bytes),
    F595(u64, Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn5::F224(k0) => f224(k0.clone(), p0),
            Fn5::F439(k0) => f439(k0.clone(), p0),
            Fn5::F455(k0, k1) => f455(*k0, k1.clone(), p0),
            Fn5::F492(k0, k1) => f492(*k0, k1.clone(), p0),
            Fn5::F532(k0) => f532(k0.clone(), p0),
            Fn5::F546(k0) => f546(k0.clone(), p0),
            Fn5::F592(k0, k1) => f592(*k0, k1.clone(), p0),
            Fn5::F595(k0, k1) => f595(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F420,
    F650,
}

impl Fn6 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn6::F420 => f420(p0, p1),
            Fn6::F650 => f650(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F423,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn7::F423 => f423(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F523(Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn8::F523(k0) => f523(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F543(u64, Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, bool)) -> R<Option<(u64, bool)>> {
        match self {
            Fn9::F543(k0, k1) => f543(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F588,
}

impl Fn10 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn10::F588 => f588(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F590,
}

impl Fn11 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn11::F590 => f590(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F620,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn12::F620 => f620(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F642,
}

impl Fn13 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn13::F642 => f642(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F649,
}

impl Fn14 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn14::F649 => f649(p0),
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
    if f46(v0.clone(), f8(Str::lit("check-yaml"))) {
        f47(v3, v1)
    } else {
        if f46(v0.clone(), f8(Str::lit("read-webidl"))) {
            f48(v3, v1, v2)
        } else {
            if f46(v0.clone(), f8(Str::lit("read-xml"))) {
                f49(v3, v1)
            } else {
                if f46(v0.clone(), f8(Str::lit("read-xsd"))) {
                    f50(v3, v1)
                } else {
                    if f46(v0.clone(), f8(Str::lit("read-xsd-components"))) {
                        f51(v3, v1)
                    } else {
                        if f46(v0, f8(Str::lit("read-yaml"))) {
                            f52(v3, v1)
                        } else {
                            Ok(Adt12::C4(f31()))
                        }
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
    let v1: u64 = f53(v0)?;
    let m192 = {
        let a190 = f54({
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
    let v1: Bytes = f55(v0)?;
    let m222 = f56(v1.clone(), 0u64, {
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
        let (h229, _) = f57(Fn0::F58(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h229
    };
    let v4: u64 = {
        let (h230, _) = {
            let (_, h231) = v3.clone();
            h231
        };
        h230
    };
    Ok(f59(List::cons(f43(v0, v4, {
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
    let m244 = f60(Fn1::F61(v1), None::<Bytes>, v0);
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
    List::cons(f8(Str::lit("admit JSONL")), List::cons(f8(Str::lit("admit-encoding STAGED")), List::cons(f8(Str::lit("admit-registry AUTHORITY ROW CONSTRUCT|none read-OP ARTIFACT")), List::cons(f8(Str::lit("admit-translated FILE NAME read-OP ARG... CITATION...")), List::cons(f8(Str::lit("agenda")), List::cons(f8(Str::lit("build")), List::cons(f8(Str::lit("check")), List::cons(f8(Str::lit("check-yaml FILE")), List::cons(f8(Str::lit("ingest-authority BINDING")), List::cons(f8(Str::lit("ingest-index CSV")), List::cons(f8(Str::lit("merge-gate RULES")), List::cons(f8(Str::lit("next [SKIP...]")), List::cons(f8(Str::lit("propose-encoding SCHEMA NAME CITATION...")), List::cons(f8(Str::lit("publish")), List::cons(f8(Str::lit("read-webidl FILE [IMPORT...]")), List::cons(f8(Str::lit("read-xml FILE")), List::cons(f8(Str::lit("read-xsd FILE")), List::cons(f8(Str::lit("read-xsd-components FILE")), List::cons(f8(Str::lit("read-yaml FILE")), List::cons(f8(Str::lit("record-decision JSONL")), List::cons(f8(Str::lit("record-error REFUSAL TIME RUN OPERATION...")), List::cons(f8(Str::lit("stage-admissions JSONL")), List::cons(f8(Str::lit("stage-bindings JSONL")), List::cons(f8(Str::lit("stage-decisions JSONL")), List::cons(f8(Str::lit("stage-encodings JSONL")), List::cons(f8(Str::lit("stage-engine")), List::cons(f8(Str::lit("stage-mappings JSONL")), List::cons(f8(Str::lit("stage-matrix JSONL")), List::cons(f8(Str::lit("stage-registries JSONL")), List::<Bytes>::nil())))))))))))))))))))))))))))))
}

pub fn f46(v0: Bytes, v1: Bytes) -> bool {
    let a245 = v0;
    let a246 = v1;
    equal(a245, a246)
}

pub fn f47(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m247 = f62(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m247.uncons() {
        None => {
            let m248 = f63(f64(v0, v1))?;
            match m248 {
                Ok(v5) => {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f65(f66(v5)?)?, List::cons(f8(Str::lit(" documents\u{a}")), List::<Bytes>::nil())))))
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

pub fn f48(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<Adt12> {
    let m249 = f62(v0.clone(), List::cons(v1.clone(), v2.clone()));
    match m249.uncons() {
        None => {
            let m250 = f67(f64(v0.clone(), v1), f68(v0, v2))?;
            match m250 {
                Ok(v6) => {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("-- ")), List::cons(f65({
                        let (h251, _) = v6;
                        h251
                    })?, List::cons(f8(Str::lit(" Web IDL definitions translated\u{a}")), List::cons({
                        let (_, h252) = v6;
                        h252
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

pub fn f49(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m253 = f62(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m253.uncons() {
        None => {
            let m254 = f69(f64(v0, v1))?;
            match m254 {
                Ok(v5) => {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("well-formed XML: root ")), List::cons(f70(v5.clone()), List::cons(f8(Str::lit(", ")), List::cons(f65(f71(v5)?)?, List::cons(f8(Str::lit(" elements\u{a}")), List::<Bytes>::nil()))))))))
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

pub fn f50(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m255 = f62(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m255.uncons() {
        None => {
            let m256 = f72(f64(v0, v1))?;
            match m256 {
                Ok(v5) => {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("-- ")), List::cons(f65({
                        let (h257, _) = v5;
                        h257
                    })?, List::cons(f8(Str::lit(" simple types\u{a}")), List::cons({
                        let (_, h258) = v5;
                        h258
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

pub fn f51(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m259 = f62(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::<Bytes>::nil())));
    match m259.uncons() {
        None => {
            let m260 = f73(f74(v0.clone()), f64(v0, f8(Str::lit("model/authorities.toml"))), v1)?;
            match m260 {
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

pub fn f52(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m261 = f62(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m261.uncons() {
        None => {
            let m262 = f63(f64(v0, v1))?;
            match m262 {
                Ok(v5) => {
                    let m263 = f75(v5)?;
                    match m263 {
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

pub fn f53(v0: u8) -> R<u64> {
    let v1: u8 = v0;
    let a308 = {
        let a284 = {
            let a272 = {
                let a266 = {
                    let a264 = v1;
                    let a265 = 128u8;
                    fixed_u8::bit_and(a264, a265)
                };
                let a267 = 128u8;
                if equal(a266, a267) {
                    128u64
                } else {
                    0u64
                }
            };
            let a273 = {
                let a270 = {
                    let a268 = v1;
                    let a269 = 64u8;
                    fixed_u8::bit_and(a268, a269)
                };
                let a271 = 64u8;
                if equal(a270, a271) {
                    64u64
                } else {
                    0u64
                }
            };
            nat_add(a272, a273)?
        };
        let a285 = {
            let a282 = {
                let a276 = {
                    let a274 = v1;
                    let a275 = 32u8;
                    fixed_u8::bit_and(a274, a275)
                };
                let a277 = 32u8;
                if equal(a276, a277) {
                    32u64
                } else {
                    0u64
                }
            };
            let a283 = {
                let a280 = {
                    let a278 = v1;
                    let a279 = 16u8;
                    fixed_u8::bit_and(a278, a279)
                };
                let a281 = 16u8;
                if equal(a280, a281) {
                    16u64
                } else {
                    0u64
                }
            };
            nat_add(a282, a283)?
        };
        nat_add(a284, a285)?
    };
    let a309 = {
        let a306 = {
            let a294 = {
                let a288 = {
                    let a286 = v1;
                    let a287 = 8u8;
                    fixed_u8::bit_and(a286, a287)
                };
                let a289 = 8u8;
                if equal(a288, a289) {
                    8u64
                } else {
                    0u64
                }
            };
            let a295 = {
                let a292 = {
                    let a290 = v1;
                    let a291 = 4u8;
                    fixed_u8::bit_and(a290, a291)
                };
                let a293 = 4u8;
                if equal(a292, a293) {
                    4u64
                } else {
                    0u64
                }
            };
            nat_add(a294, a295)?
        };
        let a307 = {
            let a304 = {
                let a298 = {
                    let a296 = v1;
                    let a297 = 2u8;
                    fixed_u8::bit_and(a296, a297)
                };
                let a299 = 2u8;
                if equal(a298, a299) {
                    2u64
                } else {
                    0u64
                }
            };
            let a305 = {
                let a302 = {
                    let a300 = v1;
                    let a301 = 1u8;
                    fixed_u8::bit_and(a300, a301)
                };
                let a303 = 1u8;
                if equal(a302, a303) {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a304, a305)?
        };
        nat_add(a306, a307)?
    };
    nat_add(a308, a309)
}

pub fn f54(v0: u64) -> List<Str> {
    let a310 = v0;
    let a311 = 0u64;
    if nat_eq(a310, a311) {
        f76()
    } else {
        let a312 = v0;
        let a313 = 1u64;
        if nat_eq(a312, a313) {
            f77()
        } else {
            let a314 = v0;
            let a315 = 2u64;
            if nat_eq(a314, a315) {
                f78()
            } else {
                let a316 = v0;
                let a317 = 3u64;
                if nat_eq(a316, a317) {
                    f79()
                } else {
                    let a318 = v0;
                    let a319 = 4u64;
                    if nat_eq(a318, a319) {
                        f80()
                    } else {
                        let a320 = v0;
                        let a321 = 5u64;
                        if nat_eq(a320, a321) {
                            f81()
                        } else {
                            let a322 = v0;
                            let a323 = 6u64;
                            if nat_eq(a322, a323) {
                                f82()
                            } else {
                                let a324 = v0;
                                let a325 = 7u64;
                                if nat_eq(a324, a325) {
                                    f83()
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

pub fn f55(v0: Bytes) -> R<Bytes> {
    f84(v0.clone(), 0u64, {
        let a326 = v0;
        length_bytes(a326)
    }, false, false, 0u64, Bytes::lit(&[]))
}

pub fn f56(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt2, u64)>> {
    let m327 = v2;
    if m327 == 0 {
        Ok(None::<(Adt2, u64)>)
    } else {
        let v4 = m327 - 1;
        let v5: u64 = f85(v0.clone(), v1)?;
        let a328 = v3;
        let a329 = 1u64;
        if nat_eq(a328, a329) {
            let m330 = f56(v0.clone(), v1, v4, 0u64)?;
            match m330 {
                None => {
                    Ok(None::<(Adt2, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f85(v0.clone(), {
                        let (_, h331) = v6.clone();
                        h331
                    })?;
                    let a332 = v7;
                    let a333 = 44u64;
                    if nat_eq(a332, a333) {
                        let m337 = f56(v0, {
                            let a335 = {
                                let (_, h334) = v6;
                                h334
                            };
                            let a336 = 1u64;
                            nat_add(a335, a336)?
                        }, v4, 1u64)?;
                        match m337 {
                            None => {
                                Ok(None::<(Adt2, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt2::C9(std::rc::Rc::new({
                                    let (h338, _) = v6;
                                    h338
                                }), std::rc::Rc::new({
                                    let (h339, _) = v8;
                                    h339
                                })), {
                                    let (_, h340) = v8;
                                    h340
                                })))
                            }
                        }
                    } else {
                        let a341 = v7;
                        let a342 = 93u64;
                        if nat_eq(a341, a342) {
                            Ok(Some((Adt2::C9(std::rc::Rc::new({
                                let (h343, _) = v6;
                                h343
                            }), std::rc::Rc::new(Adt2::C8)), {
                                let a345 = {
                                    let (_, h344) = v6;
                                    h344
                                };
                                let a346 = 1u64;
                                nat_add(a345, a346)?
                            })))
                        } else {
                            Ok(None::<(Adt2, u64)>)
                        }
                    }
                }
            }
        } else {
            let a347 = v3;
            let a348 = 2u64;
            if nat_eq(a347, a348) {
                let m351 = f86(v0.clone(), {
                    let a349 = v1;
                    let a350 = 1u64;
                    nat_add(a349, a350)?
                })?;
                match m351 {
                    None => {
                        Ok(None::<(Adt2, u64)>)
                    }
                    Some(v9) => {
                        if f87({
                            let a352 = v5;
                            let a353 = 34u64;
                            nat_eq(a352, a353)
                        }, {
                            let a355 = f85(v0.clone(), {
                                let (h354, _) = v9.clone();
                                h354
                            })?;
                            let a356 = 58u64;
                            nat_eq(a355, a356)
                        }) {
                            let m360 = f56(v0.clone(), {
                                let a358 = {
                                    let (h357, _) = v9;
                                    h357
                                };
                                let a359 = 1u64;
                                nat_add(a358, a359)?
                            }, v4, 0u64)?;
                            match m360 {
                                None => {
                                    Ok(None::<(Adt2, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f85(v0.clone(), {
                                        let (_, h361) = v10.clone();
                                        h361
                                    })?;
                                    let a362 = v11;
                                    let a363 = 44u64;
                                    if nat_eq(a362, a363) {
                                        let m367 = f56(v0, {
                                            let a365 = {
                                                let (_, h364) = v10;
                                                h364
                                            };
                                            let a366 = 1u64;
                                            nat_add(a365, a366)?
                                        }, v4, 2u64)?;
                                        match m367 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt2::C10({
                                                    let (_, h368) = v9;
                                                    h368
                                                }, std::rc::Rc::new({
                                                    let (h369, _) = v10;
                                                    h369
                                                }), std::rc::Rc::new({
                                                    let (h370, _) = v12;
                                                    h370
                                                })), {
                                                    let (_, h371) = v12;
                                                    h371
                                                })))
                                            }
                                        }
                                    } else {
                                        let a372 = v11;
                                        let a373 = 125u64;
                                        if nat_eq(a372, a373) {
                                            Ok(Some((Adt2::C10({
                                                let (_, h374) = v9;
                                                h374
                                            }, std::rc::Rc::new({
                                                let (h375, _) = v10;
                                                h375
                                            }), std::rc::Rc::new(Adt2::C8)), {
                                                let a377 = {
                                                    let (_, h376) = v10;
                                                    h376
                                                };
                                                let a378 = 1u64;
                                                nat_add(a377, a378)?
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
                let a379 = v5;
                let a380 = 34u64;
                if nat_eq(a379, a380) {
                    let m383 = f86(v0, {
                        let a381 = v1;
                        let a382 = 1u64;
                        nat_add(a381, a382)?
                    })?;
                    match m383 {
                        None => {
                            Ok(None::<(Adt2, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt2::C5({
                                let (_, h384) = v13;
                                h384
                            }), {
                                let (h385, _) = v13;
                                h385
                            })))
                        }
                    }
                } else {
                    if f46(f43(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                        Ok(Some((Adt2::C0, {
                            let a386 = v1;
                            let a387 = 4u64;
                            nat_add(a386, a387)?
                        })))
                    } else {
                        if f46(f43(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                            Ok(Some((Adt2::C2(true), {
                                let a388 = v1;
                                let a389 = 4u64;
                                nat_add(a388, a389)?
                            })))
                        } else {
                            if f46(f43(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                                Ok(Some((Adt2::C2(false), {
                                    let a390 = v1;
                                    let a391 = 5u64;
                                    nat_add(a390, a391)?
                                })))
                            } else {
                                let a392 = v5;
                                let a393 = 91u64;
                                if nat_eq(a392, a393) {
                                    let a396 = f85(v0.clone(), {
                                        let a394 = v1;
                                        let a395 = 1u64;
                                        nat_add(a394, a395)?
                                    })?;
                                    let a397 = 93u64;
                                    if nat_eq(a396, a397) {
                                        Ok(Some((Adt2::C6(std::rc::Rc::new(Adt2::C8)), {
                                            let a398 = v1;
                                            let a399 = 2u64;
                                            nat_add(a398, a399)?
                                        })))
                                    } else {
                                        let m402 = f56(v0, {
                                            let a400 = v1;
                                            let a401 = 1u64;
                                            nat_add(a400, a401)?
                                        }, v4, 1u64)?;
                                        match m402 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v14) => {
                                                Ok(Some((Adt2::C6(std::rc::Rc::new({
                                                    let (h403, _) = v14;
                                                    h403
                                                })), {
                                                    let (_, h404) = v14;
                                                    h404
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let a405 = v5;
                                    let a406 = 123u64;
                                    if nat_eq(a405, a406) {
                                        let a409 = f85(v0.clone(), {
                                            let a407 = v1;
                                            let a408 = 1u64;
                                            nat_add(a407, a408)?
                                        })?;
                                        let a410 = 125u64;
                                        if nat_eq(a409, a410) {
                                            Ok(Some((Adt2::C7(std::rc::Rc::new(Adt2::C8)), {
                                                let a411 = v1;
                                                let a412 = 2u64;
                                                nat_add(a411, a412)?
                                            })))
                                        } else {
                                            let m415 = f56(v0, {
                                                let a413 = v1;
                                                let a414 = 1u64;
                                                nat_add(a413, a414)?
                                            }, v4, 2u64)?;
                                            match m415 {
                                                None => {
                                                    Ok(None::<(Adt2, u64)>)
                                                }
                                                Some(v15) => {
                                                    Ok(Some((Adt2::C7(std::rc::Rc::new({
                                                        let (h416, _) = v15;
                                                        h416
                                                    })), {
                                                        let (_, h417) = v15;
                                                        h417
                                                    })))
                                                }
                                            }
                                        }
                                    } else {
                                        let m418 = f88(v0, v1)?;
                                        match m418 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v16) => {
                                                Ok(Some(({
                                                    let (_, h419) = v16;
                                                    h419
                                                }, {
                                                    let (h420, _) = v16;
                                                    h420
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

pub fn f57(v0: Fn0, p425: u64, p426: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m424 = (p425, p426);
    {
        let mut m424 = m424;
        loop {
            let step = {
                let (v1, v2) = m424;
                let m421 = v1;
                if m421 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m421 - 1;
                    let m423 = {
                        let c422: Fn0 = v0.clone();
                        c422.apply(v2.clone())?
                    };
                    match m423 {
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
                Ok(next) => { m424 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f58(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f89(v1, v0, v2)
}

pub fn f59(v0: List<Bytes>) -> List<Bytes> {
    f90(Fn2::F91, List::<Bytes>::nil(), v0)
}

pub fn f60(v0: Fn1, p430: Option<Bytes>, p431: List<Bytes>) -> Option<Bytes> {
    let m429 = (p430, p431);
    {
        let mut m429 = m429;
        loop {
            let step = {
                let (v1, v2) = m429;
                let m427 = v2;
                match m427.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c428: Fn1 = v0.clone();
                            c428.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m429 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f61(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m432 = v1;
    match m432 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a435 = {
                    let a433 = v3;
                    let a434 = v0;
                    append_bytes(a433, a434)
                };
                let a436 = v2;
                append_bytes(a435, a436)
            })
        }
    }
}

pub fn f62(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m437 = v1;
    match m437.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m438 = f92(f93(v0.clone(), v2.clone()), v2.clone());
            match m438 {
                None => {
                    List::cons(v2, f62(v0, v3))
                }
                Some(_) => {
                    f62(v0, v3)
                }
            }
        }
    }
}

pub fn f63(v0: Bytes) -> R<Result<List<Adt2>, Bytes>> {
    let v1: Bytes = if f94(v0.clone(), Bytes::lit(&[0xef, 0xbb, 0xbf]), 0u64) {
        f43(v0.clone(), 3u64, {
            let a440 = {
                let a439 = v0;
                length_bytes(a439)
            };
            let a441 = 3u64;
            nat_sub(a440, a441)
        })
    } else {
        v0
    };
    let v2: Bytes = f95(v1)?;
    let v3: u64 = {
        let a442 = v2.clone();
        length_bytes(a442)
    };
    let v4: u64 = f96(v2.clone())?;
    let v5: u64 = f97(v2.clone(), 0u64)?;
    let a443 = v4;
    let a444 = v3;
    if nat_lt(a443, a444) {
        Ok(Err::<List<Adt2>, Bytes>(f98(Str::lit("the input is not a sequence of printable characters in UTF-8 (YAML 1.2.2 [1], section 5.2)"), v4)?))
    } else {
        let m447 = f99(v2.clone(), v5, {
            let a445 = v3;
            let a446 = 1u64;
            nat_add(a445, a446)?
        })?;
        match m447 {
            Ok(v7) => {
                let a448 = v3;
                let a449 = v7;
                if nat_le(a448, a449) {
                    Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
                } else {
                    f100(v2, v7, true, {
                        let a450 = v3;
                        let a451 = 1u64;
                        nat_add(a450, a451)?
                    })
                }
            }
            Err(v6) => {
                Ok(Err::<List<Adt2>, Bytes>(v6))
            }
        }
    }
}

pub fn f64(v0: Adt1, v1: Bytes) -> Bytes {
    let m452 = f92(f93(v0, v1.clone()), v1);
    match m452 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m453 = v2;
            match m453 {
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

pub fn f65(v0: u64) -> R<Bytes> {
    f101(f102(), 10u64, v0)
}

pub fn f66(v0: List<Adt2>) -> R<u64> {
    let m454 = v0;
    match m454.uncons() {
        None => {
            Ok(0u64)
        }
        Some((_, v2)) => {
            let a455 = f66(v2)?;
            let a456 = 1u64;
            nat_add(a455, a456)
        }
    }
}

pub fn f67(v0: Bytes, v1: List<Bytes>) -> R<Result<(u64, Bytes), Bytes>> {
    let m457 = f103(v0)?;
    match m457 {
        Ok(v3) => {
            let m458 = f104(v1)?;
            match m458 {
                Ok(v5) => {
                    let v6: List<Adt13> = f105(v3.clone(), v5);
                    let v7: List<Adt16> = f106({
                        let a460 = {
                            let a459 = v6.clone();
                            length_list(a459)
                        };
                        let a461 = 1u64;
                        nat_add(a460, a461)?
                    }, f107(v6.clone(), v6.clone())?)?;
                    let v8: List<Bytes> = f108({
                        let a463 = {
                            let a462 = v7.clone();
                            length_list(a462)
                        };
                        let a464 = 1u64;
                        nat_add(a463, a464)?
                    }, f109(v3.clone()), List::<Bytes>::nil(), v7.clone());
                    let v9: List<(Bytes, (List<Bytes>, Bytes))> = f110(v7.clone(), v8);
                    let v10: (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>) = f111({
                        let a466 = {
                            let a465 = v9.clone();
                            length_list(a465)
                        };
                        let a467 = 1u64;
                        nat_add(a466, a467)?
                    }, v9, List::<Bytes>::nil(), List::<Bytes>::nil());
                    Ok(Ok::<(u64, Bytes), Bytes>(({
                        let a469 = {
                            let (h468, _) = v10.clone();
                            h468
                        };
                        length_list(a469)
                    }, f7(List::cons(f7(f112(v3, v7, v6)), List::cons(f7(f113({
                        let (_, h470) = v10;
                        h470
                    })), List::cons(f8(Str::lit("-- operations, constants and the other members that are not attributes are behavior, not data, and are not translated (Web IDL 2.5)\u{a}")), List::cons(f114(f115({
                        let (h471, _) = v10;
                        h471
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

pub fn f68(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m472 = v1;
    match m472.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f64(v0.clone(), v2), f68(v0, v3))
        }
    }
}

pub fn f69(v0: Bytes) -> R<Result<Adt17, Bytes>> {
    let v1: Bytes = if f94(v0.clone(), f8(Str::lit("\u{feff}")), 0u64) {
        f43(v0.clone(), 3u64, {
            let a474 = {
                let a473 = v0;
                length_bytes(a473)
            };
            let a475 = 3u64;
            nat_sub(a474, a475)
        })
    } else {
        v0
    };
    let v2: Bytes = f95(v1)?;
    let v3: u64 = {
        let a476 = v2.clone();
        length_bytes(a476)
    };
    let v4: u64 = f116(v2.clone())?;
    let a477 = v4;
    let a478 = v3;
    if nat_lt(a477, a478) {
        Ok(f117(f98(Str::lit("the input is not a sequence of Chars in UTF-8 (XML 1.0 [2], section 4.3.3)"), v4)?))
    } else {
        let m479 = f118(v2.clone())?;
        match m479 {
            Ok(v6) => {
                let m482 = f119(v2.clone(), v6, {
                    let a480 = v3;
                    let a481 = 2u64;
                    nat_add(a480, a481)?
                })?;
                match m482 {
                    Ok(v8) => {
                        if f120(v2.clone(), v8, Str::lit("<!DOCTYPE")) {
                            Ok(f117(f98(Str::lit("a document type declaration is not read; Verdant Studio reads documents that have none (XML 1.0 [28])"), v8)?))
                        } else {
                            let a483 = f121(v2.clone(), v8)?;
                            let a484 = 60u64;
                            if nat_eq(a483, a484) {
                                let m487 = f122(v2.clone(), v8, {
                                    let a485 = v3;
                                    let a486 = 2u64;
                                    nat_add(a485, a486)?
                                }, 0u64)?;
                                match m487 {
                                    Ok(v10) => {
                                        let m491 = f119(v2, {
                                            let (_, h488) = v10;
                                            h488
                                        }, {
                                            let a489 = v3;
                                            let a490 = 2u64;
                                            nat_add(a489, a490)?
                                        })?;
                                        match m491 {
                                            Ok(v12) => {
                                                let a492 = v12;
                                                let a493 = v3;
                                                if nat_eq(a492, a493) {
                                                    Ok(Ok::<Adt17, Bytes>({
                                                        let (h494, _) = v10;
                                                        h494
                                                    }))
                                                } else {
                                                    Ok(f117(f98(Str::lit("only comments, processing instructions and white space may follow the root element (XML 1.0 [1], [27])"), v12)?))
                                                }
                                            }
                                            Err(v11) => {
                                                Ok(f117(v11))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Ok(f117(v9))
                                    }
                                }
                            } else {
                                Ok(f117(f98(Str::lit("a document must have exactly one root element (XML 1.0 [1])"), v8)?))
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(f117(v7))
                    }
                }
            }
            Err(v5) => {
                Ok(f117(v5))
            }
        }
    }
}

pub fn f70(v0: Adt17) -> Bytes {
    let m495 = v0;
    match m495 {
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

pub fn f71(v0: Adt17) -> R<u64> {
    let m496 = v0;
    match m496 {
        Adt17::C0(_, _, r501) => {
            let v3 = (*r501).clone();
            let a497 = 1u64;
            let a498 = f71(v3)?;
            nat_add(a497, a498)
        }
        Adt17::C1(_, _, _) => {
            Ok(0u64)
        }
        Adt17::C2(_) => {
            Ok(0u64)
        }
        Adt17::C3(r502, r503) => {
            let v8 = (*r502).clone();
            let v9 = (*r503).clone();
            let a499 = f71(v8)?;
            let a500 = f71(v9)?;
            nat_add(a499, a500)
        }
        Adt17::C4 => {
            Ok(0u64)
        }
    }
}

pub fn f72(v0: Bytes) -> R<Result<(u64, Bytes), Bytes>> {
    let m504 = f69(v0)?;
    match m504 {
        Ok(v2) => {
            let m505 = v2;
            match m505 {
                Adt17::C0(v3, r517, r518) => {
                    let v4 = (*r517).clone();
                    let v5 = (*r518).clone();
                    let m506 = f124(v4.clone(), List::<(Bytes, Bytes)>::nil());
                    match m506 {
                        Ok(v7) => {
                            let m507 = f125(v7.clone(), v3)?;
                            match m507 {
                                Ok(v9) => {
                                    if f126(v9, Str::lit("schema")) {
                                        let v11: Bytes = {
                                            let m508 = f127(v4, f8(Str::lit("targetNamespace")));
                                            match m508 {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some(v10) => {
                                                    v10
                                                }
                                            }
                                        };
                                        let m509 = f128(v5, v7, v11)?;
                                        match m509 {
                                            Ok(v13) => {
                                                let v14: List<Bytes> = f129(v13.clone());
                                                let m510 = f130(v14.clone());
                                                match m510 {
                                                    None => {
                                                        let m511 = f131(v13.clone(), v14.clone());
                                                        match m511 {
                                                            None => {
                                                                let m515 = f132({
                                                                    let a513 = {
                                                                        let a512 = v14.clone();
                                                                        length_list(a512)
                                                                    };
                                                                    let a514 = 1u64;
                                                                    nat_add(a513, a514)?
                                                                }, v13, List::<Bytes>::nil(), List::<Bytes>::nil());
                                                                match m515 {
                                                                    Ok(v18) => {
                                                                        Ok(Ok::<(u64, Bytes), Bytes>(({
                                                                            let a516 = v14;
                                                                            length_list(a516)
                                                                        }, f114(f115(v18, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))))))
                                                                    }
                                                                    Err(v17) => {
                                                                        Ok(f123(v17))
                                                                    }
                                                                }
                                                            }
                                                            Some(v16) => {
                                                                Ok(f123(f7(List::cons(f8(Str::lit("the type ")), List::cons(v16, List::cons(f8(Str::lit(" is not a simple type of this schema document")), List::<Bytes>::nil()))))))
                                                            }
                                                        }
                                                    }
                                                    Some(v15) => {
                                                        Ok(f123(f7(List::cons(f8(Str::lit("two simple types derive the type name ")), List::cons(v15, List::<Bytes>::nil())))))
                                                    }
                                                }
                                            }
                                            Err(v12) => {
                                                Ok(f123(v12))
                                            }
                                        }
                                    } else {
                                        Ok(f123(f8(Str::lit("the root element must be xs:schema in the XML Schema namespace (XSD 1.1 Part 1, section 3.17.2)"))))
                                    }
                                }
                                Err(v8) => {
                                    Ok(f123(v8))
                                }
                            }
                        }
                        Err(v6) => {
                            Ok(f123(v6))
                        }
                    }
                }
                Adt17::C1(_, _, _) => {
                    Ok(f123(f8(Str::lit("the document has no root element"))))
                }
                Adt17::C2(_) => {
                    Ok(f123(f8(Str::lit("the document has no root element"))))
                }
                Adt17::C3(_, _) => {
                    Ok(f123(f8(Str::lit("the document has no root element"))))
                }
                Adt17::C4 => {
                    Ok(f123(f8(Str::lit("the document has no root element"))))
                }
            }
        }
        Err(v1) => {
            Ok(f123(f7(List::cons(f8(Str::lit("not well-formed XML: ")), List::cons(v1, List::<Bytes>::nil())))))
        }
    }
}

pub fn f73(v0: List<(Bytes, Option<Bytes>)>, v1: Bytes, v2: Bytes) -> R<Adt18> {
    let v3: List<(Bytes, Bytes)> = f133(v1)?;
    f134({
        let a520 = {
            let a519 = v3.clone();
            length_list(a519)
        };
        let a521 = 2u64;
        nat_add(a520, a521)?
    }, v0, v3, List::cons((v2, (f8(Str::lit("root")), Bytes::lit(&[]))), List::<(Bytes, (Bytes, Bytes))>::nil()), List::<Adt19>::nil(), List::<Bytes>::nil())
}

pub fn f74(v0: Adt1) -> List<(Bytes, Option<Bytes>)> {
    f135(match v0 {
        Adt1::C0(h522, _, _, _) => {
            h522
        }
    })
}

pub fn f75(v0: List<Adt2>) -> R<Result<Bytes, Bytes>> {
    let m523 = v0;
    match m523.uncons() {
        None => {
            Ok(Ok::<Bytes, Bytes>(Bytes::lit(&[])))
        }
        Some((v1, v2)) => {
            let m524 = f136(v1)?;
            match m524 {
                Ok(v4) => {
                    let m525 = f75(v2)?;
                    match m525 {
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

pub fn f76() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{9}"), List::cons(Str::lit("\u{a}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{d}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f77() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f78() -> List<Str> {
    List::cons(Str::lit(" "), List::cons(Str::lit("!"), List::cons(Str::lit("\""), List::cons(Str::lit("#"), List::cons(Str::lit("$"), List::cons(Str::lit("%"), List::cons(Str::lit("&"), List::cons(Str::lit("'"), List::cons(Str::lit("("), List::cons(Str::lit(")"), List::cons(Str::lit("*"), List::cons(Str::lit("+"), List::cons(Str::lit(","), List::cons(Str::lit("-"), List::cons(Str::lit("."), List::cons(Str::lit("/"), List::<Str>::nil()))))))))))))))))
}

pub fn f79() -> List<Str> {
    List::cons(Str::lit("0"), List::cons(Str::lit("1"), List::cons(Str::lit("2"), List::cons(Str::lit("3"), List::cons(Str::lit("4"), List::cons(Str::lit("5"), List::cons(Str::lit("6"), List::cons(Str::lit("7"), List::cons(Str::lit("8"), List::cons(Str::lit("9"), List::cons(Str::lit(":"), List::cons(Str::lit(";"), List::cons(Str::lit("<"), List::cons(Str::lit("="), List::cons(Str::lit(">"), List::cons(Str::lit("?"), List::<Str>::nil()))))))))))))))))
}

pub fn f80() -> List<Str> {
    List::cons(Str::lit("@"), List::cons(Str::lit("A"), List::cons(Str::lit("B"), List::cons(Str::lit("C"), List::cons(Str::lit("D"), List::cons(Str::lit("E"), List::cons(Str::lit("F"), List::cons(Str::lit("G"), List::cons(Str::lit("H"), List::cons(Str::lit("I"), List::cons(Str::lit("J"), List::cons(Str::lit("K"), List::cons(Str::lit("L"), List::cons(Str::lit("M"), List::cons(Str::lit("N"), List::cons(Str::lit("O"), List::<Str>::nil()))))))))))))))))
}

pub fn f81() -> List<Str> {
    List::cons(Str::lit("P"), List::cons(Str::lit("Q"), List::cons(Str::lit("R"), List::cons(Str::lit("S"), List::cons(Str::lit("T"), List::cons(Str::lit("U"), List::cons(Str::lit("V"), List::cons(Str::lit("W"), List::cons(Str::lit("X"), List::cons(Str::lit("Y"), List::cons(Str::lit("Z"), List::cons(Str::lit("["), List::cons(Str::lit("\\"), List::cons(Str::lit("]"), List::cons(Str::lit("^"), List::cons(Str::lit("_"), List::<Str>::nil()))))))))))))))))
}

pub fn f82() -> List<Str> {
    List::cons(Str::lit("`"), List::cons(Str::lit("a"), List::cons(Str::lit("b"), List::cons(Str::lit("c"), List::cons(Str::lit("d"), List::cons(Str::lit("e"), List::cons(Str::lit("f"), List::cons(Str::lit("g"), List::cons(Str::lit("h"), List::cons(Str::lit("i"), List::cons(Str::lit("j"), List::cons(Str::lit("k"), List::cons(Str::lit("l"), List::cons(Str::lit("m"), List::cons(Str::lit("n"), List::cons(Str::lit("o"), List::<Str>::nil()))))))))))))))))
}

pub fn f83() -> List<Str> {
    List::cons(Str::lit("p"), List::cons(Str::lit("q"), List::cons(Str::lit("r"), List::cons(Str::lit("s"), List::cons(Str::lit("t"), List::cons(Str::lit("u"), List::cons(Str::lit("v"), List::cons(Str::lit("w"), List::cons(Str::lit("x"), List::cons(Str::lit("y"), List::cons(Str::lit("z"), List::cons(Str::lit("{"), List::cons(Str::lit("|"), List::cons(Str::lit("}"), List::cons(Str::lit("~"), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f84(v0: Bytes, p553: u64, p554: u64, p555: bool, p556: bool, p557: u64, p558: Bytes) -> R<Bytes> {
    let m552 = (p553, (p554, (p555, (p556, (p557, p558)))));
    {
        let mut m552 = m552;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, (v5, v6))))) = m552;
                let m526 = v2;
                if m526 == 0 {
                    Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f137(v0.clone(), v5, v1, v6)))
                } else {
                    let v7 = m526 - 1;
                    let m529 = {
                        let a527 = v0.clone();
                        let a528 = v1;
                        index_bytes(a527, a528)
                    };
                    match m529 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f137(v0.clone(), v5, v1, v6)))
                        }
                        Some(v8) => {
                            let v9: u64 = f53(v8)?;
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                    let a530 = v1;
                                    let a531 = 1u64;
                                    nat_add(a530, a531)?
                                }, (v7, (if v4 {
                                    true
                                } else {
                                    let a532 = v9;
                                    let a533 = 34u64;
                                    !nat_eq(a532, a533)
                                }, (if v4 {
                                    false
                                } else {
                                    let a534 = v9;
                                    let a535 = 92u64;
                                    nat_eq(a534, a535)
                                }, (v5, v6))))))
                            } else {
                                let a536 = v9;
                                let a537 = 32u64;
                                if if if if nat_eq(a536, a537) {
                                    true
                                } else {
                                    let a538 = v9;
                                    let a539 = 9u64;
                                    nat_eq(a538, a539)
                                } {
                                    true
                                } else {
                                    let a540 = v9;
                                    let a541 = 10u64;
                                    nat_eq(a540, a541)
                                } {
                                    true
                                } else {
                                    let a542 = v9;
                                    let a543 = 13u64;
                                    nat_eq(a542, a543)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a544 = v1;
                                        let a545 = 1u64;
                                        nat_add(a544, a545)?
                                    }, (v7, (false, (false, ({
                                        let a546 = v1;
                                        let a547 = 1u64;
                                        nat_add(a546, a547)?
                                    }, f137(v0.clone(), v5, v1, v6)))))))
                                } else {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a548 = v1;
                                        let a549 = 1u64;
                                        nat_add(a548, a549)?
                                    }, (v7, ({
                                        let a550 = v9;
                                        let a551 = 34u64;
                                        nat_eq(a550, a551)
                                    }, (false, (v5, v6))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m552 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f85(v0: Bytes, v1: u64) -> R<u64> {
    let m561 = {
        let a559 = v0;
        let a560 = v1;
        index_bytes(a559, a560)
    };
    match m561 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a606 = {
                let a582 = {
                    let a570 = {
                        let a564 = {
                            let a562 = v3;
                            let a563 = 128u8;
                            fixed_u8::bit_and(a562, a563)
                        };
                        let a565 = 128u8;
                        if equal(a564, a565) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a571 = {
                        let a568 = {
                            let a566 = v3;
                            let a567 = 64u8;
                            fixed_u8::bit_and(a566, a567)
                        };
                        let a569 = 64u8;
                        if equal(a568, a569) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a570, a571)?
                };
                let a583 = {
                    let a580 = {
                        let a574 = {
                            let a572 = v3;
                            let a573 = 32u8;
                            fixed_u8::bit_and(a572, a573)
                        };
                        let a575 = 32u8;
                        if equal(a574, a575) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a581 = {
                        let a578 = {
                            let a576 = v3;
                            let a577 = 16u8;
                            fixed_u8::bit_and(a576, a577)
                        };
                        let a579 = 16u8;
                        if equal(a578, a579) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a580, a581)?
                };
                nat_add(a582, a583)?
            };
            let a607 = {
                let a604 = {
                    let a592 = {
                        let a586 = {
                            let a584 = v3;
                            let a585 = 8u8;
                            fixed_u8::bit_and(a584, a585)
                        };
                        let a587 = 8u8;
                        if equal(a586, a587) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a593 = {
                        let a590 = {
                            let a588 = v3;
                            let a589 = 4u8;
                            fixed_u8::bit_and(a588, a589)
                        };
                        let a591 = 4u8;
                        if equal(a590, a591) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a592, a593)?
                };
                let a605 = {
                    let a602 = {
                        let a596 = {
                            let a594 = v3;
                            let a595 = 2u8;
                            fixed_u8::bit_and(a594, a595)
                        };
                        let a597 = 2u8;
                        if equal(a596, a597) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a603 = {
                        let a600 = {
                            let a598 = v3;
                            let a599 = 1u8;
                            fixed_u8::bit_and(a598, a599)
                        };
                        let a601 = 1u8;
                        if equal(a600, a601) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a602, a603)?
                };
                nat_add(a604, a605)?
            };
            nat_add(a606, a607)
        }
    }
}

pub fn f86(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = f138(v0.clone(), v1, {
        let a609 = {
            let a608 = v0.clone();
            length_bytes(a608)
        };
        let a610 = 1u64;
        nat_add(a609, a610)?
    })?;
    let a611 = f85(v0.clone(), v2)?;
    let a612 = 34u64;
    if nat_eq(a611, a612) {
        Ok(Some(({
            let a613 = v2;
            let a614 = 1u64;
            nat_add(a613, a614)?
        }, f43(v0, v1, {
            let a615 = v2;
            let a616 = v1;
            nat_sub(a615, a616)
        }))))
    } else {
        f139(v0.clone(), v1, {
            let a618 = {
                let a617 = v0;
                length_bytes(a617)
            };
            let a619 = 1u64;
            nat_add(a618, a619)?
        }, Bytes::lit(&[]))
    }
}

pub fn f87(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f88(v0: Bytes, v1: u64) -> R<Option<(u64, Adt2)>> {
    let v2: bool = {
        let a620 = f85(v0.clone(), v1)?;
        let a621 = 45u64;
        nat_eq(a620, a621)
    };
    let v3: u64 = if v2 {
        let a622 = v1;
        let a623 = 1u64;
        nat_add(a622, a623)?
    } else {
        v1
    };
    let v4: u64 = f140(v0.clone(), v3)?;
    let v5: u64 = {
        let a624 = v4;
        let a625 = v3;
        nat_sub(a624, a625)
    };
    let v6: bool = {
        let a626 = f85(v0.clone(), v4)?;
        let a627 = 46u64;
        nat_eq(a626, a627)
    };
    let v7: u64 = if v6 {
        let a628 = v4;
        let a629 = 1u64;
        nat_add(a628, a629)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f140(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f85(v0.clone(), v8)?;
    let v10: bool = {
        let a630 = v9;
        let a631 = 101u64;
        if nat_eq(a630, a631) {
            true
        } else {
            let a632 = v9;
            let a633 = 69u64;
            nat_eq(a632, a633)
        }
    };
    let v11: u64 = f85(v0.clone(), {
        let a634 = v8;
        let a635 = 1u64;
        nat_add(a634, a635)?
    })?;
    let v12: bool = f87(v10, f141({
        let a636 = v11;
        let a637 = 43u64;
        nat_eq(a636, a637)
    }, {
        let a638 = v11;
        let a639 = 45u64;
        nat_eq(a638, a639)
    }));
    let v13: u64 = if v10 {
        if v12 {
            let a640 = v8;
            let a641 = 2u64;
            nat_add(a640, a641)?
        } else {
            let a642 = v8;
            let a643 = 1u64;
            nat_add(a642, a643)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f140(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f43(v0.clone(), v13, {
        let a644 = v14;
        let a645 = v13;
        nat_sub(a644, a645)
    });
    let v16: u64 = f142(v15.clone())?;
    let v17: Bytes = f43(v15.clone(), v16, {
        let a647 = {
            let a646 = v15;
            length_bytes(a646)
        };
        let a648 = v16;
        nat_sub(a647, a648)
    });
    let a649 = v5;
    let a650 = 0u64;
    if if nat_eq(a649, a650) {
        true
    } else {
        if f87({
            let a651 = 1u64;
            let a652 = v5;
            nat_lt(a651, a652)
        }, {
            let a653 = f85(v0.clone(), v3)?;
            let a654 = 48u64;
            nat_eq(a653, a654)
        }) {
            true
        } else {
            if f87(v6, {
                let a655 = v8;
                let a656 = v7;
                nat_eq(a655, a656)
            }) {
                true
            } else {
                f141(if v10 {
                    let a657 = v14;
                    let a658 = v13;
                    nat_eq(a657, a658)
                } else {
                    false
                }, {
                    let a660 = 18u64;
                    let a661 = {
                        let a659 = v17.clone();
                        length_bytes(a659)
                    };
                    nat_lt(a660, a661)
                })
            }
        }
    } {
        Ok(None::<(u64, Adt2)>)
    } else {
        let m665 = {
            let a663 = {
                let a662 = v17.clone();
                length_bytes(a662)
            };
            let a664 = 0u64;
            if nat_eq(a663, a664) {
                Some(0u64)
            } else {
                f143(v17)?
            }
        };
        match m665 {
            None => {
                Ok(None::<(u64, Adt2)>)
            }
            Some(v18) => {
                let m670 = f144(f43(v0.clone(), v3, v5), f43(v0, v7, {
                    let a666 = v8;
                    let a667 = v7;
                    nat_sub(a666, a667)
                }), f87(v12, {
                    let a668 = v11;
                    let a669 = 45u64;
                    nat_eq(a668, a669)
                }), v18)?;
                match m670 {
                    None => {
                        Ok(None::<(u64, Adt2)>)
                    }
                    Some(v19) => {
                        let a673 = 19u64;
                        let a674 = {
                            let a672 = {
                                let (h671, _) = v19.clone();
                                h671
                            };
                            length_bytes(a672)
                        };
                        if nat_lt(a673, a674) {
                            Ok(None::<(u64, Adt2)>)
                        } else {
                            let m676 = f143({
                                let (h675, _) = v19.clone();
                                h675
                            })?;
                            match m676 {
                                None => {
                                    Ok(None::<(u64, Adt2)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f87(v2, {
                                        let a677 = 0u64;
                                        let a678 = v20;
                                        nat_lt(a677, a678)
                                    });
                                    if f145(v21, v20) {
                                        Ok(Some((v14, {
                                            let a680 = {
                                                let (_, h679) = v19.clone();
                                                h679
                                            };
                                            let a681 = 0u64;
                                            if nat_eq(a680, a681) {
                                                Adt2::C3(v21, v20)
                                            } else {
                                                Adt2::C4(v21, v20, {
                                                    let (_, h682) = v19;
                                                    h682
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

pub fn f89(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h683, _) = v2.clone();
        h683
    };
    let v4: u64 = {
        let (h684, _) = {
            let (_, h685) = v2.clone();
            h685
        };
        h684
    };
    let v5: List<Bytes> = {
        let (_, h686) = {
            let (_, h687) = v2;
            h687
        };
        h686
    };
    let m688 = f146(v0.clone(), v3);
    match m688 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a689 = v6;
            let a690 = v1;
            if equal(a689, a690) {
                Ok(Some(({
                    let a691 = v3;
                    let a692 = 1u64;
                    nat_add(a691, a692)?
                }, ({
                    let a693 = v3;
                    let a694 = 1u64;
                    nat_add(a693, a694)?
                }, List::cons(f43(v0, v4, {
                    let a695 = v3;
                    let a696 = v4;
                    nat_sub(a695, a696)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a697 = v3;
                    let a698 = 1u64;
                    nat_add(a697, a698)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f90(v0: Fn2, p702: List<Bytes>, p703: List<Bytes>) -> List<Bytes> {
    let m701 = (p702, p703);
    {
        let mut m701 = m701;
        loop {
            let step = {
                let (v1, v2) = m701;
                let m699 = v2;
                match m699.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c700: Fn2 = v0.clone();
                            c700.apply(v1, v3)
                        }, v4))
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

pub fn f91(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f92(p708: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m707 = p708;
    {
        let mut m707 = m707;
        loop {
            let step = {
                let v0 = m707;
                let m704 = v0;
                match m704.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f46({
                            let (h705, _) = v2;
                            h705
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h706) = v2;
                                h706
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m707 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f93(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Bytes>)> {
    let m712 = {
        let a710 = match v0 {
            Adt1::C0(h709, _, _, _) => {
                h709
            }
        };
        let a711 = f37(v1);
        index_list(a710, a711)
    };
    match m712 {
        None => {
            List::<(Bytes, Option<Bytes>)>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f94(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m717 = {
        let a714 = v0;
        let a715 = v2;
        let a716 = {
            let a713 = v1.clone();
            length_bytes(a713)
        };
        slice_bytes(a714, a715, a716)
    };
    match m717 {
        None => {
            false
        }
        Some(v3) => {
            let a718 = v3;
            let a719 = v1;
            equal(a718, a719)
        }
    }
}

pub fn f95(v0: Bytes) -> R<Bytes> {
    let (_, h720) = {
        let (h721, _) = f147(Fn3::F148(v0.clone()), {
            let a723 = {
                let a722 = v0;
                length_bytes(a722)
            };
            let a724 = 1u64;
            nat_add(a723, a724)?
        }, (0u64, Bytes::lit(&[])))?;
        h721
    };
    Ok(h720)
}

pub fn f96(v0: Bytes) -> R<u64> {
    let (h725, _) = f149(Fn4::F150(v0.clone()), {
        let a727 = {
            let a726 = v0;
            length_bytes(a726)
        };
        let a728 = 1u64;
        nat_add(a727, a728)?
    }, 0u64)?;
    Ok(h725)
}

pub fn f97(v0: Bytes, v1: u64) -> R<u64> {
    let (h729, _) = f149(Fn4::F151(v0.clone()), {
        let a731 = {
            let a730 = v0;
            length_bytes(a730)
        };
        let a732 = 1u64;
        nat_add(a731, a732)?
    }, v1)?;
    Ok(h729)
}

pub fn f98(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a733 = v0;
        utf8_encode(a733)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f65(v1)?, List::<Bytes>::nil())))))
}

pub fn f99(v0: Bytes, p742: u64, p743: u64) -> R<Result<u64, Bytes>> {
    let m741 = (p742, p743);
    {
        let mut m741 = m741;
        loop {
            let step = {
                let (v1, v2) = m741;
                let m734 = v2;
                if m734 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Ok::<u64, Bytes>(v1)))
                } else {
                    let v3 = m734 - 1;
                    let a736 = v1;
                    let a737 = {
                        let a735 = v0.clone();
                        length_bytes(a735)
                    };
                    if if nat_lt(a736, a737) {
                        if f120(v0.clone(), v1, Str::lit("...")) {
                            f152(v0.clone(), v1)?
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        let m740 = f153(v0.clone(), {
                            let a738 = v1;
                            let a739 = 3u64;
                            nat_add(a738, a739)?
                        })?;
                        match m740 {
                            Ok(v5) => {
                                Ok::<(u64, u64), R<Result<u64, Bytes>>>((f97(v0.clone(), v5)?, v3))
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
                Ok(next) => { m741 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f100(v0: Bytes, v1: u64, v2: bool, v3: u64) -> R<Result<List<Adt2>, Bytes>> {
    let m744 = v3;
    if m744 == 0 {
        Ok(Err::<List<Adt2>, Bytes>(f98(Str::lit("the stream holds more documents than it is long"), v1)?))
    } else {
        let v4 = m744 - 1;
        let v5: u64 = {
            let a745 = v0.clone();
            length_bytes(a745)
        };
        let a746 = f154(v0.clone(), v1)?;
        let a747 = 37u64;
        if nat_eq(a746, a747) {
            Ok(Err::<List<Adt2>, Bytes>(f98(Str::lit("a directive is not read (YAML 1.2.2 [82])"), v1)?))
        } else {
            let m754 = if if f120(v0.clone(), v1, Str::lit("---")) {
                f152(v0.clone(), v1)?
            } else {
                false
            } {
                f155(v0.clone(), {
                    let a748 = v1;
                    let a749 = 3u64;
                    nat_add(a748, a749)?
                }, 0u64, {
                    let a750 = v5;
                    let a751 = 2u64;
                    nat_add(a750, a751)?
                }, 1u64, false, 0u64, List::<Bytes>::nil(), Adt20::C0(0u64))?
            } else {
                if v2 {
                    f155(v0.clone(), v1, 0u64, {
                        let a752 = v5;
                        let a753 = 2u64;
                        nat_add(a752, a753)?
                    }, 0u64, false, 0u64, List::<Bytes>::nil(), Adt20::C0(0u64))?
                } else {
                    Adt21::C1(f98(Str::lit("only comments, a document end marker or a document start marker may follow a document (YAML 1.2.2 [207], [211])"), v1)?)
                }
            };
            match m754 {
                Adt21::C0(v7, v8) => {
                    let v9: u64 = f97(v0.clone(), v8)?;
                    let m757 = f99(v0.clone(), v9, {
                        let a755 = v5;
                        let a756 = 1u64;
                        nat_add(a755, a756)?
                    })?;
                    match m757 {
                        Ok(v11) => {
                            let a758 = v5;
                            let a759 = v11;
                            if nat_le(a758, a759) {
                                Ok(Ok::<List<Adt2>, Bytes>(List::cons(v7, List::<Adt2>::nil())))
                            } else {
                                let m762 = f100(v0, v11, f156({
                                    let a760 = v11;
                                    let a761 = v9;
                                    nat_eq(a760, a761)
                                }), v4)?;
                                match m762 {
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
                Adt21::C1(v6) => {
                    Ok(Err::<List<Adt2>, Bytes>(v6))
                }
            }
        }
    }
}

pub fn f101(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a763 = v2;
    let a764 = 0u64;
    if nat_eq(a763, a764) {
        Ok(f157(v0, 0u64))
    } else {
        let (_, h765) = {
            let (h766, _) = f147(Fn3::F158(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h766
        };
        Ok(h765)
    }
}

pub fn f102() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f103(v0: Bytes) -> R<Result<List<Adt13>, Bytes>> {
    let m767 = f159(v0)?;
    match m767 {
        Ok(v2) => {
            f160({
                let a769 = {
                    let a768 = v2.clone();
                    length_list(a768)
                };
                let a770 = 1u64;
                nat_add(a769, a770)?
            }, v2, List::<Adt13>::nil())
        }
        Err(v1) => {
            Ok(Err::<List<Adt13>, Bytes>(v1))
        }
    }
}

pub fn f104(v0: List<Bytes>) -> R<Result<List<Adt13>, Bytes>> {
    let m771 = v0;
    match m771.uncons() {
        None => {
            Ok(Ok::<List<Adt13>, Bytes>(List::<Adt13>::nil()))
        }
        Some((v1, v2)) => {
            let m772 = f103(v1)?;
            match m772 {
                Ok(v4) => {
                    let m773 = f104(v2)?;
                    match m773 {
                        Ok(v6) => {
                            Ok(Ok::<List<Adt13>, Bytes>(f105(v4, v6)))
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

pub fn f105(v0: List<Adt13>, v1: List<Adt13>) -> List<Adt13> {
    let m774 = v0;
    match m774.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f105(v3, v1))
        }
    }
}

pub fn f106(p779: u64, p780: List<Adt16>) -> R<List<Adt16>> {
    let m778 = (p779, p780);
    {
        let mut m778 = m778;
        loop {
            let step = {
                let (v0, v1) = m778;
                let m775 = v0;
                if m775 == 0 {
                    Err::<(u64, List<Adt16>), R<List<Adt16>>>(Ok(v1))
                } else {
                    let v2 = m775 - 1;
                    let v3: List<Adt16> = f161(v1.clone(), v1.clone());
                    let a776 = f162(v3.clone())?;
                    let a777 = f162(v1.clone())?;
                    if nat_eq(a776, a777) {
                        Err::<(u64, List<Adt16>), R<List<Adt16>>>(Ok(v1))
                    } else {
                        Ok::<(u64, List<Adt16>), R<List<Adt16>>>((v2, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m778 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f107(v0: List<Adt13>, v1: List<Adt13>) -> R<List<Adt16>> {
    let m781 = v0;
    match m781.uncons() {
        None => {
            Ok(List::<Adt16>::nil())
        }
        Some((v2, v3)) => {
            if f163(f164(v2.clone())) {
                let v6: Adt16 = {
                    let m782 = f165(v1.clone(), v2.clone())?;
                    match m782 {
                        Ok(v5) => {
                            Adt16::C0(f166(v2), Bytes::lit(&[]), {
                                let (h783, _) = v5;
                                h783
                            }, {
                                let (_, h784) = v5;
                                h784
                            })
                        }
                        Err(v4) => {
                            Adt16::C0(f166(v2), v4, Bytes::lit(&[]), List::<Bytes>::nil())
                        }
                    }
                };
                Ok(List::cons(v6, f107(v3, v1)?))
            } else {
                f107(v3, v1)
            }
        }
    }
}

pub fn f108(p789: u64, p790: List<Bytes>, p791: List<Bytes>, v3: List<Adt16>) -> List<Bytes> {
    let m788 = (p789, (p790, p791));
    {
        let mut m788 = m788;
        loop {
            let step = {
                let (v0, (v1, v2)) = m788;
                let m785 = v0;
                if m785 == 0 {
                    Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v2)
                } else {
                    let v4 = m785 - 1;
                    let m786 = v1;
                    match m786.uncons() {
                        None => {
                            Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v2)
                        }
                        Some((v5, v6)) => {
                            if f167(v5.clone(), v2.clone()) {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v2)))
                            } else {
                                let m787 = f168(v3.clone(), v5.clone());
                                match m787 {
                                    None => {
                                        Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v2)))
                                    }
                                    Some(v7) => {
                                        if f46(f169(v7.clone()), Bytes::lit(&[])) {
                                            Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (f170(f171(v7), v6), List::cons(v5, v2))))
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
                Ok(next) => { m788 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f109(v0: List<Adt13>) -> List<Bytes> {
    let m792 = v0;
    match m792.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let v3: u64 = f164(v1.clone());
            let a793 = v3;
            let a794 = 0u64;
            if if nat_eq(a793, a794) {
                true
            } else {
                let a795 = v3;
                let a796 = 2u64;
                if nat_eq(a795, a796) {
                    true
                } else {
                    let a797 = v3;
                    let a798 = 3u64;
                    if nat_eq(a797, a798) {
                        true
                    } else {
                        let a799 = v3;
                        let a800 = 4u64;
                        if nat_eq(a799, a800) {
                            true
                        } else {
                            let a801 = v3;
                            let a802 = 5u64;
                            if nat_eq(a801, a802) {
                                true
                            } else {
                                let a803 = v3;
                                let a804 = 7u64;
                                if nat_eq(a803, a804) {
                                    true
                                } else {
                                    let a805 = v3;
                                    let a806 = 8u64;
                                    nat_eq(a805, a806)
                                }
                            }
                        }
                    }
                }
            } {
                List::cons(f166(v1), f109(v2))
            } else {
                f109(v2)
            }
        }
    }
}

pub fn f110(v0: List<Adt16>, v1: List<Bytes>) -> List<(Bytes, (List<Bytes>, Bytes))> {
    let m807 = v0;
    match m807.uncons() {
        None => {
            List::<(Bytes, (List<Bytes>, Bytes))>::nil()
        }
        Some((v2, v3)) => {
            if if f167(f172(v2.clone()), v1.clone()) {
                f46(f169(v2.clone()), Bytes::lit(&[]))
            } else {
                false
            } {
                List::cons((f172(v2.clone()), (f171(v2.clone()), f173(v2))), f110(v3, v1))
            } else {
                f110(v3, v1)
            }
        }
    }
}

pub fn f111(p816: u64, p817: List<(Bytes, (List<Bytes>, Bytes))>, p818: List<Bytes>, p819: List<Bytes>) -> (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>) {
    let m815 = (p816, (p817, (p818, p819)));
    {
        let mut m815 = m815;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m815;
                let m808 = v0;
                if m808 == 0 {
                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v3, v1))
                } else {
                    let v4 = m808 - 1;
                    let m809 = f174(v1.clone(), v2.clone());
                    match m809 {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v3, v1))
                        }
                        Some(v5) => {
                            Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v4, ({
                                let (_, h810) = v5.clone();
                                h810
                            }, (List::cons({
                                let (h811, _) = {
                                    let (h812, _) = v5.clone();
                                    h812
                                };
                                h811
                            }, v2), List::cons({
                                let (_, h813) = {
                                    let (h814, _) = v5;
                                    h814
                                };
                                h813
                            }, v3)))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m815 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f112(v0: List<Adt13>, v1: List<Adt16>, v2: List<Adt13>) -> List<Bytes> {
    let m820 = v0;
    match m820.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            let v5: u64 = f164(v3.clone());
            let a821 = v5;
            let a822 = 5u64;
            if if nat_eq(a821, a822) {
                true
            } else {
                let a823 = v5;
                let a824 = 7u64;
                nat_eq(a823, a824)
            } {
                let m827 = f175(v2.clone(), {
                    let a825 = v5;
                    let a826 = 5u64;
                    if nat_eq(a825, a826) {
                        0u64
                    } else {
                        2u64
                    }
                }, f166(v3.clone()));
                match m827 {
                    None => {
                        List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f176(v5), List::cons(f166(v3.clone()), List::cons(f8(Str::lit(": no input defines ")), List::cons(f166(v3), List::cons(f8(Str::lit(", so its members join no definition (Web IDL 2.2 and 2.7)\u{a}")), List::<Bytes>::nil()))))))), f112(v4, v1, v2))
                    }
                    Some(_) => {
                        f112(v4, v1, v2)
                    }
                }
            } else {
                let a828 = v5;
                let a829 = 8u64;
                if nat_eq(a828, a829) {
                    let m830 = f175(v2.clone(), 0u64, f166(v3.clone()));
                    match m830 {
                        None => {
                            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f166(v3.clone()), List::cons(f8(Str::lit(" includes ")), List::cons(f177(v3.clone()), List::cons(f8(Str::lit(": no input defines the interface ")), List::cons(f166(v3), List::cons(f8(Str::lit(" (Web IDL 2.3)\u{a}")), List::<Bytes>::nil())))))))), f112(v4, v1, v2))
                        }
                        Some(_) => {
                            f112(v4, v1, v2)
                        }
                    }
                } else {
                    if f163(v5) {
                        let m831 = f168(v1.clone(), f166(v3.clone()));
                        match m831 {
                            None => {
                                f112(v4, v1, v2)
                            }
                            Some(v8) => {
                                if f46(f169(v8.clone()), Bytes::lit(&[])) {
                                    f112(v4, v1, v2)
                                } else {
                                    List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f166(v3), List::cons(f8(Str::lit(": ")), List::cons(f169(v8), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), f112(v4, v1, v2))
                                }
                            }
                        }
                    } else {
                        f112(v4, v1, v2)
                    }
                }
            }
        }
    }
}

pub fn f113(v0: List<(Bytes, (List<Bytes>, Bytes))>) -> List<Bytes> {
    let m832 = v0;
    match m832.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons({
                let (h833, _) = v1;
                h833
            }, List::cons(f8(Str::lit(": it refers back to itself through its members, and the translator emits non-recursive declarations only\u{a}")), List::<Bytes>::nil())))), f113(v2))
        }
    }
}

pub fn f114(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m834 = v0;
    match m834.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m835 = v3.clone();
            match m835.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a838 = v2;
                    let a839 = {
                        let a836 = v1.clone();
                        let a837 = f114(v3, v1);
                        append_bytes(a836, a837)
                    };
                    append_bytes(a838, a839)
                }
            }
        }
    }
}

pub fn f115(p842: List<Bytes>, p843: List<Bytes>) -> List<Bytes> {
    let m841 = (p842, p843);
    {
        let mut m841 = m841;
        loop {
            let step = {
                let (v0, v1) = m841;
                let m840 = v0;
                match m840.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m841 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f116(v0: Bytes) -> R<u64> {
    let (h844, _) = f149(Fn4::F178(v0.clone()), {
        let a846 = {
            let a845 = v0;
            length_bytes(a845)
        };
        let a847 = 1u64;
        nat_add(a846, a847)?
    }, 0u64)?;
    Ok(h844)
}

pub fn f117(v0: Bytes) -> Result<Adt17, Bytes> {
    Err::<Adt17, Bytes>(v0)
}

pub fn f118(v0: Bytes) -> R<Result<u64, Bytes>> {
    if if f120(v0.clone(), 0u64, Str::lit("<?xml")) {
        f179(f121(v0.clone(), 5u64)?)?
    } else {
        false
    } {
        let m849 = f180(v0.clone(), {
            let a848 = Str::lit("?>");
            utf8_encode(a848)
        }, 5u64)?;
        match m849 {
            None => {
                Ok(f181(f98(Str::lit("the XML declaration is not closed (XML 1.0 [23])"), 0u64)?))
            }
            Some(v1) => {
                let m851 = f180(f43(v0.clone(), 0u64, v1), {
                    let a850 = Str::lit("encoding");
                    utf8_encode(a850)
                }, 5u64)?;
                match m851 {
                    None => {
                        Ok(Ok::<u64, Bytes>({
                            let a852 = v1;
                            let a853 = 2u64;
                            nat_add(a852, a853)?
                        }))
                    }
                    Some(v2) => {
                        let v3: u64 = f182(v0.clone(), {
                            let a854 = v2;
                            let a855 = 8u64;
                            nat_add(a854, a855)?
                        })?;
                        let v4: u64 = f182(v0.clone(), {
                            let a856 = v3;
                            let a857 = 1u64;
                            nat_add(a856, a857)?
                        })?;
                        let _: u64 = f121(v0.clone(), v4)?;
                        let v7: u64 = {
                            let m860 = f180(v0.clone(), f43(v0.clone(), v4, 1u64), {
                                let a858 = v4;
                                let a859 = 1u64;
                                nat_add(a858, a859)?
                            })?;
                            match m860 {
                                None => {
                                    v1
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        if f183(f43(v0, {
                            let a861 = v4;
                            let a862 = 1u64;
                            nat_add(a861, a862)?
                        }, {
                            let a865 = v7;
                            let a866 = {
                                let a863 = v4;
                                let a864 = 1u64;
                                nat_add(a863, a864)?
                            };
                            nat_sub(a865, a866)
                        })) {
                            Ok(Ok::<u64, Bytes>({
                                let a867 = v1;
                                let a868 = 2u64;
                                nat_add(a867, a868)?
                            }))
                        } else {
                            Ok(f181(f98(Str::lit("only the UTF-8 encoding is read (XML 1.0 section 4.3.3)"), 0u64)?))
                        }
                    }
                }
            }
        }
    } else {
        Ok(Ok::<u64, Bytes>(0u64))
    }
}

pub fn f119(v0: Bytes, p873: u64, p874: u64) -> R<Result<u64, Bytes>> {
    let m872 = (p873, p874);
    {
        let mut m872 = m872;
        loop {
            let step = {
                let (v1, v2) = m872;
                let m869 = v2;
                if m869 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f181(f98(Str::lit("the prolog is longer than the input"), v1)?)))
                } else {
                    let v3 = m869 - 1;
                    let v4: u64 = f182(v0.clone(), v1)?;
                    if f120(v0.clone(), v4, Str::lit("<!--")) {
                        let m870 = f184(v0.clone(), v4)?;
                        match m870 {
                            Ok(v6) => {
                                Ok::<(u64, u64), R<Result<u64, Bytes>>>((v6, v3))
                            }
                            Err(v5) => {
                                Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f181(v5)))
                            }
                        }
                    } else {
                        if f120(v0.clone(), v4, Str::lit("<?")) {
                            let m871 = f185(v0.clone(), v4)?;
                            match m871 {
                                Ok(v8) => {
                                    Ok::<(u64, u64), R<Result<u64, Bytes>>>((v8, v3))
                                }
                                Err(v7) => {
                                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f181(v7)))
                                }
                            }
                        } else {
                            Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Ok::<u64, Bytes>(v4)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m872 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f120(v0: Bytes, v1: u64, v2: Str) -> bool {
    f94(v0, {
        let a875 = v2;
        utf8_encode(a875)
    }, v1)
}

pub fn f121(v0: Bytes, v1: u64) -> R<u64> {
    f85(v0, v1)
}

pub fn f122(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Result<(Adt17, u64), Bytes>> {
    let m876 = v2;
    if m876 == 0 {
        Ok(f186(f98(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v4 = m876 - 1;
        let a877 = v3;
        let a878 = 0u64;
        if nat_eq(a877, a878) {
            let v5: u64 = f187(v0.clone(), {
                let a879 = v1;
                let a880 = 1u64;
                nat_add(a879, a880)?
            })?;
            let a883 = v5;
            let a884 = {
                let a881 = v1;
                let a882 = 1u64;
                nat_add(a881, a882)?
            };
            if nat_eq(a883, a884) {
                Ok(f186(f98(Str::lit("an element must start with a Name (XML 1.0 [40])"), v1)?))
            } else {
                let v6: Bytes = f43(v0.clone(), {
                    let a885 = v1;
                    let a886 = 1u64;
                    nat_add(a885, a886)?
                }, {
                    let a889 = v5;
                    let a890 = {
                        let a887 = v1;
                        let a888 = 1u64;
                        nat_add(a887, a888)?
                    };
                    nat_sub(a889, a890)
                });
                let m891 = f188(v0.clone(), v5, v4, List::<Bytes>::nil())?;
                match m891 {
                    Ok(v8) => {
                        if f120(v0.clone(), {
                            let (_, h892) = v8.clone();
                            h892
                        }, Str::lit("/>")) {
                            Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C0(v6, std::rc::Rc::new({
                                let (h893, _) = v8;
                                h893
                            }), std::rc::Rc::new(Adt17::C4)), {
                                let a895 = {
                                    let (_, h894) = v8;
                                    h894
                                };
                                let a896 = 2u64;
                                nat_add(a895, a896)?
                            })))
                        } else {
                            let m900 = f122(v0.clone(), {
                                let a898 = {
                                    let (_, h897) = v8;
                                    h897
                                };
                                let a899 = 1u64;
                                nat_add(a898, a899)?
                            }, v4, 1u64)?;
                            match m900 {
                                Ok(v10) => {
                                    let v11: u64 = {
                                        let a902 = {
                                            let (_, h901) = v10.clone();
                                            h901
                                        };
                                        let a903 = 2u64;
                                        nat_add(a902, a903)?
                                    };
                                    let v12: u64 = f187(v0.clone(), v11)?;
                                    if f46(f43(v0.clone(), v11, {
                                        let a904 = v12;
                                        let a905 = v11;
                                        nat_sub(a904, a905)
                                    }), v6.clone()) {
                                        let v13: u64 = f182(v0.clone(), v12)?;
                                        let a906 = f121(v0, v13)?;
                                        let a907 = 62u64;
                                        if nat_eq(a906, a907) {
                                            Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C0(v6, std::rc::Rc::new({
                                                let (h908, _) = v8;
                                                h908
                                            }), std::rc::Rc::new({
                                                let (h909, _) = v10;
                                                h909
                                            })), {
                                                let a910 = v13;
                                                let a911 = 1u64;
                                                nat_add(a910, a911)?
                                            })))
                                        } else {
                                            Ok(f186(f98(Str::lit("an end tag must close with > (XML 1.0 [42])"), v13)?))
                                        }
                                    } else {
                                        Ok(f186(f98(Str::lit("an end tag must name the element its start tag opened (XML 1.0 WFC: Element Type Match)"), {
                                            let (_, h912) = v10;
                                            h912
                                        })?))
                                    }
                                }
                                Err(v9) => {
                                    Ok(f186(v9))
                                }
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(f186(v7))
                    }
                }
            }
        } else {
            let a913 = f121(v0.clone(), v1)?;
            let a914 = 256u64;
            if nat_eq(a913, a914) {
                Ok(f186(f98(Str::lit("the input ends before an element is closed (XML 1.0 [39])"), v1)?))
            } else {
                if f120(v0.clone(), v1, Str::lit("</")) {
                    Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C4, v1)))
                } else {
                    if f120(v0.clone(), v1, Str::lit("<!--")) {
                        let m915 = f184(v0.clone(), v1)?;
                        match m915 {
                            Ok(v15) => {
                                f122(v0, v15, v4, 1u64)
                            }
                            Err(v14) => {
                                Ok(f186(v14))
                            }
                        }
                    } else {
                        if f120(v0.clone(), v1, Str::lit("<![CDATA[")) {
                            let m919 = f180(v0.clone(), {
                                let a916 = Str::lit("]]>");
                                utf8_encode(a916)
                            }, {
                                let a917 = v1;
                                let a918 = 9u64;
                                nat_add(a917, a918)?
                            })?;
                            match m919 {
                                None => {
                                    Ok(f186(f98(Str::lit("a CDATA section is not closed (XML 1.0 [18])"), v1)?))
                                }
                                Some(v16) => {
                                    let m922 = f122(v0.clone(), {
                                        let a920 = v16;
                                        let a921 = 3u64;
                                        nat_add(a920, a921)?
                                    }, v4, 1u64)?;
                                    match m922 {
                                        Ok(v18) => {
                                            Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C3(std::rc::Rc::new(Adt17::C2(f43(v0, {
                                                let a923 = v1;
                                                let a924 = 9u64;
                                                nat_add(a923, a924)?
                                            }, {
                                                let a927 = v16;
                                                let a928 = {
                                                    let a925 = v1;
                                                    let a926 = 9u64;
                                                    nat_add(a925, a926)?
                                                };
                                                nat_sub(a927, a928)
                                            }))), std::rc::Rc::new({
                                                let (h929, _) = v18;
                                                h929
                                            })), {
                                                let (_, h930) = v18;
                                                h930
                                            })))
                                        }
                                        Err(v17) => {
                                            Ok(f186(v17))
                                        }
                                    }
                                }
                            }
                        } else {
                            if f120(v0.clone(), v1, Str::lit("<?")) {
                                let m931 = f185(v0.clone(), v1)?;
                                match m931 {
                                    Ok(v20) => {
                                        f122(v0, v20, v4, 1u64)
                                    }
                                    Err(v19) => {
                                        Ok(f186(v19))
                                    }
                                }
                            } else {
                                if f120(v0.clone(), v1, Str::lit("<!")) {
                                    Ok(f186(f98(Str::lit("a markup declaration is not allowed in content (XML 1.0 [43])"), v1)?))
                                } else {
                                    let a932 = f121(v0.clone(), v1)?;
                                    let a933 = 60u64;
                                    if nat_eq(a932, a933) {
                                        let m934 = f122(v0.clone(), v1, v4, 0u64)?;
                                        match m934 {
                                            Ok(v22) => {
                                                let m936 = f122(v0, {
                                                    let (_, h935) = v22;
                                                    h935
                                                }, v4, 1u64)?;
                                                match m936 {
                                                    Ok(v24) => {
                                                        Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C3(std::rc::Rc::new({
                                                            let (h937, _) = v22;
                                                            h937
                                                        }), std::rc::Rc::new({
                                                            let (h938, _) = v24;
                                                            h938
                                                        })), {
                                                            let (_, h939) = v24;
                                                            h939
                                                        })))
                                                    }
                                                    Err(v23) => {
                                                        Ok(f186(v23))
                                                    }
                                                }
                                            }
                                            Err(v21) => {
                                                Ok(f186(v21))
                                            }
                                        }
                                    } else {
                                        let m940 = f189(v0.clone(), v1, v4, Bytes::lit(&[]), false, 0u64)?;
                                        match m940 {
                                            Ok(v26) => {
                                                let m942 = f122(v0, {
                                                    let (_, h941) = v26;
                                                    h941
                                                }, v4, 1u64)?;
                                                match m942 {
                                                    Ok(v28) => {
                                                        Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C3(std::rc::Rc::new(Adt17::C2({
                                                            let (h943, _) = v26;
                                                            h943
                                                        })), std::rc::Rc::new({
                                                            let (h944, _) = v28;
                                                            h944
                                                        })), {
                                                            let (_, h945) = v28;
                                                            h945
                                                        })))
                                                    }
                                                    Err(v27) => {
                                                        Ok(f186(v27))
                                                    }
                                                }
                                            }
                                            Err(v25) => {
                                                Ok(f186(v25))
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

pub fn f123(v0: Bytes) -> Result<(u64, Bytes), Bytes> {
    Err::<(u64, Bytes), Bytes>(v0)
}

pub fn f124(v0: Adt17, v1: List<(Bytes, Bytes)>) -> Result<List<(Bytes, Bytes)>, Bytes> {
    let m946 = v0;
    match m946 {
        Adt17::C0(_, _, _) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt17::C1(v2, v3, r951) => {
            let v4 = (*r951).clone();
            let m947 = f124(v4, v1);
            match m947 {
                Ok(v6) => {
                    if f46(v2.clone(), f8(Str::lit("xmlns"))) {
                        if if f46(v3.clone(), f191()) {
                            true
                        } else {
                            f46(v3.clone(), f192())
                        } {
                            f190(f8(Str::lit("the default namespace must not be the xml or xmlns namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Bytes>(List::cons((Bytes::lit(&[]), v3), v6))
                        }
                    } else {
                        if f193(v2.clone(), f8(Str::lit("xmlns:"))) {
                            let v7: Bytes = f43(v2.clone(), 6u64, {
                                let a949 = {
                                    let a948 = v2;
                                    length_bytes(a948)
                                };
                                let a950 = 6u64;
                                nat_sub(a949, a950)
                            });
                            if f46(v7.clone(), f8(Str::lit("xmlns"))) {
                                f190(f8(Str::lit("the prefix xmlns must not be declared (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                            } else {
                                if if f46(v7.clone(), f8(Str::lit("xml"))) {
                                    !f46(v3.clone(), f191())
                                } else {
                                    false
                                } {
                                    f190(f8(Str::lit("the prefix xml may be bound only to its own namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                } else {
                                    if if if f46(v3.clone(), f191()) {
                                        true
                                    } else {
                                        f46(v3.clone(), f192())
                                    } {
                                        !f46(v7.clone(), f8(Str::lit("xml")))
                                    } else {
                                        false
                                    } {
                                        f190(f8(Str::lit("the xml and xmlns namespace names must not be bound to another prefix (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                    } else {
                                        if f46(v3.clone(), Bytes::lit(&[])) {
                                            f190(f8(Str::lit("a prefix must not be undeclared (Namespaces in XML 1.0, section 3, NSC: No Prefix Undeclaring)")))
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
                    f190(v5)
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

pub fn f125(v0: List<(Bytes, Bytes)>, v1: Bytes) -> R<Result<(Bytes, Bytes), Bytes>> {
    let v2: Bytes = f194(v1.clone())?;
    if f46(v2.clone(), Bytes::lit(&[])) {
        let m952 = f195(v0, Bytes::lit(&[]));
        match m952 {
            None => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((Bytes::lit(&[]), v1)))
            }
            Some(v3) => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((v3, v1)))
            }
        }
    } else {
        if f46(v2.clone(), f8(Str::lit("xml"))) {
            Ok(Ok::<(Bytes, Bytes), Bytes>((f191(), f196(v1)?)))
        } else {
            let m953 = f195(v0, v2.clone());
            match m953 {
                None => {
                    Ok(Err::<(Bytes, Bytes), Bytes>(f7(List::cons(f8(Str::lit("the prefix ")), List::cons(v2, List::cons(f8(Str::lit(" is not declared (Namespaces in XML 1.0, section 5, NSC: Prefix Declared)")), List::<Bytes>::nil()))))))
                }
                Some(v4) => {
                    Ok(Ok::<(Bytes, Bytes), Bytes>((v4, f196(v1)?)))
                }
            }
        }
    }
}

pub fn f126(v0: (Bytes, Bytes), v1: Str) -> bool {
    if f46({
        let (h954, _) = v0;
        h954
    }, f197()) {
        f46({
            let (_, h955) = v0;
            h955
        }, {
            let a956 = v1;
            utf8_encode(a956)
        })
    } else {
        false
    }
}

pub fn f127(p960: Adt17, v1: Bytes) -> Option<Bytes> {
    let m959 = p960;
    {
        let mut m959 = m959;
        loop {
            let step = {
                let v0 = m959;
                let m957 = v0;
                match m957 {
                    Adt17::C0(_, _, _) => {
                        Err::<Adt17, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt17::C1(v2, v3, r958) => {
                        let v4 = (*r958).clone();
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
                Ok(next) => { m959 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f128(v0: Adt17, v1: List<(Bytes, Bytes)>, v2: Bytes) -> R<Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>> {
    let m961 = v0;
    match m961 {
        Adt17::C0(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt17::C2(_) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt17::C3(r972, r973) => {
            let v3 = (*r972).clone();
            let v4 = (*r973).clone();
            let m962 = f128(v4, v1.clone(), v2.clone())?;
            match m962 {
                Ok(v6) => {
                    let m963 = v3;
                    match m963 {
                        Adt17::C0(v7, r970, r971) => {
                            let v8 = (*r970).clone();
                            let v9 = (*r971).clone();
                            let m964 = f124(v8.clone(), v1);
                            match m964 {
                                Ok(v11) => {
                                    let m965 = f125(v11.clone(), v7)?;
                                    match m965 {
                                        Ok(v13) => {
                                            if f126(v13, Str::lit("simpleType")) {
                                                let m966 = f127(v8, f8(Str::lit("name")));
                                                match m966 {
                                                    None => {
                                                        Ok(f198(f8(Str::lit("a top-level simpleType must have a name (XSD 1.1 Part 2, section 4.1.2)"))))
                                                    }
                                                    Some(v14) => {
                                                        let m967 = f199(v14.clone(), v9, v11, v2)?;
                                                        match m967 {
                                                            Ok(v16) => {
                                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::cons((f200(v14)?, (match v16.clone() {
                                                                    Adt23::C0(_, h968) => {
                                                                        h968
                                                                    }
                                                                }, match v16 {
                                                                    Adt23::C0(h969, _) => {
                                                                        h969
                                                                    }
                                                                })), v6)))
                                                            }
                                                            Err(v15) => {
                                                                Ok(f198(v15))
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                                            }
                                        }
                                        Err(v12) => {
                                            Ok(f198(v12))
                                        }
                                    }
                                }
                                Err(v10) => {
                                    Ok(f198(v10))
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
                    Ok(f198(v5))
                }
            }
        }
        Adt17::C4 => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
    }
}

pub fn f129(v0: List<(Bytes, (List<Bytes>, Bytes))>) -> List<Bytes> {
    let m974 = v0;
    match m974.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h975, _) = v1;
                h975
            }, f129(v2))
        }
    }
}

pub fn f130(p978: List<Bytes>) -> Option<Bytes> {
    let m977 = p978;
    {
        let mut m977 = m977;
        loop {
            let step = {
                let v0 = m977;
                let m976 = v0;
                match m976.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f167(v1.clone(), v2.clone()) {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v1))
                        } else {
                            Ok::<List<Bytes>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m977 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f131(p984: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<Bytes> {
    let m983 = p984;
    {
        let mut m983 = m983;
        loop {
            let step = {
                let v0 = m983;
                let m979 = v0;
                match m979.uncons() {
                    None => {
                        Err::<List<(Bytes, (List<Bytes>, Bytes))>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        let m982 = f201({
                            let (h980, _) = {
                                let (_, h981) = v2;
                                h981
                            };
                            h980
                        }, v1.clone());
                        match m982 {
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
                Ok(next) => { m983 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f132(p995: u64, p996: List<(Bytes, (List<Bytes>, Bytes))>, p997: List<Bytes>, p998: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m994 = (p995, (p996, (p997, p998)));
    {
        let mut m994 = m994;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m994;
                let m985 = v0;
                if m985 == 0 {
                    let m986 = v1;
                    match m986.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(f202(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                        }
                    }
                } else {
                    let v6 = m985 - 1;
                    let m987 = v1.clone();
                    match m987.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            let m988 = f174(v1, v2.clone());
                            match m988 {
                                None => {
                                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(f202(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                                }
                                Some(v9) => {
                                    Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>((v6, ({
                                        let (_, h989) = v9.clone();
                                        h989
                                    }, (List::cons({
                                        let (h990, _) = {
                                            let (h991, _) = v9.clone();
                                            h991
                                        };
                                        h990
                                    }, v2), List::cons({
                                        let (_, h992) = {
                                            let (h993, _) = v9;
                                            h993
                                        };
                                        h992
                                    }, v3)))))
                                }
                            }
                        }
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

pub fn f133(v0: Bytes) -> R<List<(Bytes, Bytes)>> {
    f203(f204(v0)?, None::<Bytes>)
}

pub fn f134(p1019: u64, v1: List<(Bytes, Option<Bytes>)>, v2: List<(Bytes, Bytes)>, p1020: List<(Bytes, (Bytes, Bytes))>, p1021: List<Adt19>, p1022: List<Bytes>) -> R<Adt18> {
    let m1018 = (p1019, (p1020, (p1021, p1022)));
    {
        let mut m1018 = m1018;
        loop {
            let step = {
                let (v0, (v3, (v4, v5))) = m1018;
                let m999 = v0;
                if m999 == 0 {
                    Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>(Ok(Adt18::C1(f8(Str::lit("the schema documents import and include more documents than the bound authorities hold")))))
                } else {
                    let v6 = m999 - 1;
                    let m1000 = v3;
                    match m1000.uncons() {
                        None => {
                            let m1001 = v5.clone();
                            match m1001.uncons() {
                                None => {
                                    let m1002 = f205(f206(v4, List::<Adt19>::nil()))?;
                                    match m1002 {
                                        Ok(v8) => {
                                            Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>(Ok(Adt18::C2({
                                                let (h1003, _) = v8;
                                                h1003
                                            }, {
                                                let (_, h1004) = v8;
                                                h1004
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
                                let (h1005, _) = v11.clone();
                                h1005
                            };
                            if if f167(v13.clone(), f207(v4.clone())) {
                                true
                            } else {
                                f167(v13.clone(), v5.clone())
                            } {
                                Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>((v6, (v12, (v4, v5))))
                            } else {
                                let m1006 = f208(v1.clone(), v13.clone());
                                match m1006 {
                                    None => {
                                        Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>((v6, (v12, (v4, List::cons(v13, v5)))))
                                    }
                                    Some(v14) => {
                                        let m1007 = v14;
                                        match m1007 {
                                            None => {
                                                Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>(Ok(Adt18::C1(f209(f8(Str::lit("the schema document ")), v13, f8(Str::lit(" is not in the repository"))))))
                                            }
                                            Some(v15) => {
                                                let m1008 = f210(v13.clone(), v15, f211(v2.clone(), v13))?;
                                                match m1008 {
                                                    Ok(v17) => {
                                                        let m1013 = f212(v17.clone(), {
                                                            let (h1009, _) = {
                                                                let (_, h1010) = v11.clone();
                                                                h1010
                                                            };
                                                            h1009
                                                        }, {
                                                            let (_, h1011) = {
                                                                let (_, h1012) = v11;
                                                                h1012
                                                            };
                                                            h1011
                                                        });
                                                        match m1013 {
                                                            None => {
                                                                let m1016 = f213(match v17.clone() {
                                                                    Adt19::C0(_, _, _, _, h1014, _) => {
                                                                        h1014
                                                                    }
                                                                }, match v17.clone() {
                                                                    Adt19::C0(_, _, _, h1015, _, _) => {
                                                                        h1015
                                                                    }
                                                                })?;
                                                                match m1016 {
                                                                    Ok(v20) => {
                                                                        let m1017 = f214(v20, v17.clone(), v2.clone())?;
                                                                        match m1017 {
                                                                            Ok(v22) => {
                                                                                Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>((v6, (f215(v12, v22), (List::cons(v17, v4), v5))))
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
                Ok(next) => { m1018 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f135(v0: List<List<(Bytes, Option<Bytes>)>>) -> List<(Bytes, Option<Bytes>)> {
    let m1023 = v0;
    match m1023.uncons() {
        None => {
            List::<(Bytes, Option<Bytes>)>::nil()
        }
        Some((v1, v2)) => {
            f216(v1, f135(v2))
        }
    }
}

pub fn f136(v0: Adt2) -> R<Result<Bytes, Bytes>> {
    let m1024 = f217(v0.clone());
    match m1024 {
        None => {
            Ok(Ok::<Bytes, Bytes>(f218(v0)?))
        }
        Some(v1) => {
            Ok(Err::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("the node tagged ")), List::cons(v1, List::cons(f8(Str::lit(" has no JSON text: a JSON value carries no tag (RFC 8259 section 3), and a YAML node keeps its tag (YAML 1.2.2 section 3.2.1.1)")), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f137(v0: Bytes, v1: u64, v2: u64, v3: Bytes) -> Bytes {
    let a1027 = v3;
    let a1028 = f43(v0, v1, {
        let a1025 = v2;
        let a1026 = v1;
        nat_sub(a1025, a1026)
    });
    append_bytes(a1027, a1028)
}

pub fn f138(v0: Bytes, p1041: u64, p1042: u64) -> R<u64> {
    let m1040 = (p1041, p1042);
    {
        let mut m1040 = m1040;
        loop {
            let step = {
                let (v1, v2) = m1040;
                let m1029 = v2;
                if m1029 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m1029 - 1;
                    let v4: u64 = f85(v0.clone(), v1)?;
                    let a1030 = v4;
                    let a1031 = 34u64;
                    if nat_eq(a1030, a1031) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a1032 = v4;
                        let a1033 = 92u64;
                        if nat_eq(a1032, a1033) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a1034 = v4;
                            let a1035 = 32u64;
                            if nat_lt(a1034, a1035) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a1036 = v4;
                                let a1037 = 256u64;
                                if nat_eq(a1036, a1037) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a1038 = v1;
                                        let a1039 = 1u64;
                                        nat_add(a1038, a1039)?
                                    }, v3))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1040 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f139(v0: Bytes, p1068: u64, p1069: u64, p1070: Bytes) -> R<Option<(u64, Bytes)>> {
    let m1067 = (p1068, (p1069, p1070));
    {
        let mut m1067 = m1067;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1067;
                let m1043 = v2;
                if m1043 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                } else {
                    let v4 = m1043 - 1;
                    let v5: u64 = f138(v0.clone(), v1, {
                        let a1047 = {
                            let a1045 = {
                                let a1044 = v0.clone();
                                length_bytes(a1044)
                            };
                            let a1046 = v1;
                            nat_sub(a1045, a1046)
                        };
                        let a1048 = 1u64;
                        nat_add(a1047, a1048)?
                    })?;
                    let v6: Bytes = {
                        let a1051 = v3;
                        let a1052 = f43(v0.clone(), v1, {
                            let a1049 = v5;
                            let a1050 = v1;
                            nat_sub(a1049, a1050)
                        });
                        append_bytes(a1051, a1052)
                    };
                    let v7: u64 = f85(v0.clone(), v5)?;
                    let a1053 = v7;
                    let a1054 = 34u64;
                    if nat_eq(a1053, a1054) {
                        Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(Some(({
                            let a1055 = v5;
                            let a1056 = 1u64;
                            nat_add(a1055, a1056)?
                        }, v6))))
                    } else {
                        let a1057 = v7;
                        let a1058 = 92u64;
                        if nat_eq(a1057, a1058) {
                            let m1059 = f219(v0.clone(), v5, v6)?;
                            match m1059 {
                                None => {
                                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                                }
                                Some(v8) => {
                                    let a1062 = {
                                        let (_, h1060) = {
                                            let (_, h1061) = v8.clone();
                                            h1061
                                        };
                                        h1060
                                    };
                                    let a1063 = 0u64;
                                    if nat_eq(a1062, a1063) {
                                        Ok::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(({
                                            let (h1064, _) = v8;
                                            h1064
                                        }, (v4, {
                                            let (h1065, _) = {
                                                let (_, h1066) = v8;
                                                h1066
                                            };
                                            h1065
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
                Ok(next) => { m1067 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f140(v0: Bytes, v1: u64) -> R<u64> {
    let (h1071, _) = f149(Fn4::F220(v0.clone()), {
        let a1073 = {
            let a1072 = v0;
            length_bytes(a1072)
        };
        let a1074 = 1u64;
        nat_add(a1073, a1074)?
    }, v1)?;
    Ok(h1071)
}

pub fn f141(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f142(v0: Bytes) -> R<u64> {
    let (h1075, _) = f149(Fn4::F221(v0.clone()), {
        let a1077 = {
            let a1076 = v0;
            length_bytes(a1076)
        };
        let a1078 = 1u64;
        nat_add(a1077, a1078)?
    }, 0u64)?;
    Ok(h1075)
}

pub fn f143(v0: Bytes) -> R<Option<u64>> {
    if f222(v0.clone())? {
        Ok(Some({
            let (_, h1079) = {
                let (h1080, _) = f223(Fn5::F224(v0.clone()), {
                    let a1081 = v0;
                    length_bytes(a1081)
                }, (0u64, 0u64))?;
                h1080
            };
            h1079
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f144(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f43(v1.clone(), 0u64, {
        let a1083 = {
            let a1082 = v1.clone();
            length_bytes(a1082)
        };
        let a1084 = f225(v1)?;
        nat_sub(a1083, a1084)
    });
    let v5: Bytes = {
        let a1085 = v0;
        let a1086 = v4.clone();
        append_bytes(a1085, a1086)
    };
    let v6: u64 = f142(v5.clone())?;
    let v7: Bytes = f43(v5.clone(), v6, {
        let a1088 = {
            let a1087 = v5;
            length_bytes(a1087)
        };
        let a1089 = v6;
        nat_sub(a1088, a1089)
    });
    let v8: u64 = {
        let a1090 = v4;
        length_bytes(a1090)
    };
    let v9: u64 = {
        let a1091 = v7.clone();
        length_bytes(a1091)
    };
    let a1092 = v9;
    let a1093 = 0u64;
    if nat_eq(a1092, a1093) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f141(v2, {
            let a1094 = v3;
            let a1095 = v8;
            nat_lt(a1094, a1095)
        }) {
            let v10: u64 = if v2 {
                let a1096 = v3;
                let a1097 = v8;
                nat_add(a1096, a1097)?
            } else {
                let a1098 = v8;
                let a1099 = v3;
                nat_sub(a1098, a1099)
            };
            let v11: u64 = f225(v7.clone())?;
            let a1100 = v10;
            let a1101 = v11;
            if nat_le(a1100, a1101) {
                Ok(Some((f43(v7, 0u64, {
                    let a1102 = v9;
                    let a1103 = v10;
                    nat_sub(a1102, a1103)
                }), 0u64)))
            } else {
                let a1106 = {
                    let a1104 = v10;
                    let a1105 = v11;
                    nat_sub(a1104, a1105)
                };
                let a1107 = 19u64;
                if nat_le(a1106, a1107) {
                    Ok(Some((f43(v7, 0u64, {
                        let a1108 = v9;
                        let a1109 = v11;
                        nat_sub(a1108, a1109)
                    }), {
                        let a1110 = v10;
                        let a1111 = v11;
                        nat_sub(a1110, a1111)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a1112 = v3;
                let a1113 = v8;
                nat_sub(a1112, a1113)
            };
            let a1116 = {
                let a1114 = v9;
                let a1115 = v12;
                nat_add(a1114, a1115)?
            };
            let a1117 = 19u64;
            if nat_le(a1116, a1117) {
                Ok(Some(({
                    let a1118 = v7;
                    let a1119 = f226(v12);
                    append_bytes(a1118, a1119)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f145(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1120 = 0u64;
        let a1121 = v1;
        if nat_lt(a1120, a1121) {
            let a1122 = v1;
            let a1123 = 9223372036854775808u64;
            nat_le(a1122, a1123)
        } else {
            false
        }
    } else {
        let a1124 = v1;
        let a1125 = 9223372036854775807u64;
        nat_le(a1124, a1125)
    }
}

pub fn f146(v0: Bytes, v1: u64) -> Option<u8> {
    let a1126 = v0;
    let a1127 = v1;
    index_bytes(a1126, a1127)
}

pub fn f147(v0: Fn3, p1132: u64, p1133: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1131 = (p1132, p1133);
    {
        let mut m1131 = m1131;
        loop {
            let step = {
                let (v1, v2) = m1131;
                let m1128 = v1;
                if m1128 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1128 - 1;
                    let m1130 = {
                        let c1129: Fn3 = v0.clone();
                        c1129.apply(v2.clone())?
                    };
                    match m1130 {
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
                Ok(next) => { m1131 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f148(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f227(v0, v1)
}

pub fn f149(v0: Fn4, p1138: u64, p1139: u64) -> R<(u64, bool)> {
    let m1137 = (p1138, p1139);
    {
        let mut m1137 = m1137;
        loop {
            let step = {
                let (v1, v2) = m1137;
                let m1134 = v1;
                if m1134 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1134 - 1;
                    let m1136 = {
                        let c1135: Fn4 = v0.clone();
                        c1135.apply(v2)?
                    };
                    match m1136 {
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
                Ok(next) => { m1137 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f150(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f228(v0, v1)
}

pub fn f151(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f229(v0, v1)
}

pub fn f152(v0: Bytes, v1: u64) -> R<bool> {
    if if if f120(v0.clone(), v1, Str::lit("---")) {
        true
    } else {
        f120(v0.clone(), v1, Str::lit("..."))
    } {
        if f230(f154(v0.clone(), {
            let a1140 = v1;
            let a1141 = 3u64;
            nat_add(a1140, a1141)?
        })?) {
            true
        } else {
            f231(f154(v0.clone(), {
                let a1142 = v1;
                let a1143 = 3u64;
                nat_add(a1142, a1143)?
            })?)
        }
    } else {
        false
    } {
        let a1144 = f232(v0, v1)?;
        let a1145 = 0u64;
        Ok(nat_eq(a1144, a1145))
    } else {
        Ok(false)
    }
}

pub fn f153(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f233(v0.clone(), v1)?;
    let v3: u64 = f154(v0.clone(), v2)?;
    if if f231(v3) {
        true
    } else {
        let a1146 = v3;
        let a1147 = 35u64;
        if nat_eq(a1146, a1147) {
            let a1148 = v1;
            let a1149 = v2;
            nat_lt(a1148, a1149)
        } else {
            false
        }
    } {
        Ok(Ok::<u64, Bytes>(f234(v0, v2)?))
    } else {
        Ok(Err::<u64, Bytes>(f98(Str::lit("only white space and a comment may follow a node on its line (YAML 1.2.2 [77])"), v2)?))
    }
}

pub fn f154(v0: Bytes, v1: u64) -> R<u64> {
    f85(v0, v1)
}

pub fn f155(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: List<Bytes>, v8: Adt20) -> R<Adt21> {
    let m1150 = v3;
    if m1150 == 0 {
        Ok(Adt21::C1(f98(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v9 = m1150 - 1;
        let a1151 = v4;
        let a1152 = 0u64;
        if nat_eq(a1151, a1152) {
            let v10: u64 = f235(v0.clone(), v1)?;
            let v11: u64 = {
                let a1153 = v10;
                let a1154 = v1;
                nat_sub(a1153, a1154)
            };
            let a1155 = f154(v0.clone(), v10)?;
            let a1156 = 9u64;
            if nat_eq(a1155, a1156) {
                let v12: u64 = f233(v0.clone(), v10)?;
                if if f236(v0.clone(), v12)? {
                    true
                } else {
                    f237(v0.clone(), v12)?
                } {
                    Ok(Adt21::C1(f98(Str::lit("a tab cannot indent a block node (YAML 1.2.2 [63])"), v12)?))
                } else {
                    let m1157 = f238(v0.clone(), v12)?;
                    match m1157 {
                        None => {
                            f239(v0, v12, v2, v8)
                        }
                        Some(v13) => {
                            Ok(Adt21::C1(v13))
                        }
                    }
                }
            } else {
                let a1158 = f154(v0.clone(), v10)?;
                let a1159 = 33u64;
                if if nat_eq(a1158, a1159) {
                    f156(f237(v0.clone(), v10)?)
                } else {
                    false
                } {
                    if f240(v8) {
                        Ok(Adt21::C1(f98(Str::lit("a node holds at most one tag (YAML 1.2.2 [96])"), v10)?))
                    } else {
                        let v14: Adt20 = f241(v0.clone(), v10)?;
                        let m1160 = f242(v14.clone());
                        match m1160 {
                            None => {
                                f155(v0.clone(), f243(v0.clone(), v10)?, v2, v9, 1u64, false, {
                                    let a1162 = {
                                        let a1161 = v0;
                                        length_bytes(a1161)
                                    };
                                    let a1163 = 1u64;
                                    nat_add(a1162, a1163)?
                                }, List::<Bytes>::nil(), v14)
                            }
                            Some(v15) => {
                                Ok(Adt21::C1(v15))
                            }
                        }
                    }
                } else {
                    if f236(v0.clone(), v10)? {
                        f244(v8, true, v10, f155(v0, v10, v2, v9, 2u64, false, v11, List::<Bytes>::nil(), Adt20::C0(0u64))?)
                    } else {
                        let m1166 = {
                            let a1164 = f154(v0.clone(), v10)?;
                            let a1165 = 33u64;
                            if nat_eq(a1164, a1165) {
                                None::<Bytes>
                            } else {
                                f238(v0.clone(), v10)?
                            }
                        };
                        match m1166 {
                            None => {
                                if f237(v0.clone(), v10)? {
                                    f244(v8, false, v10, f155(v0, v10, v2, v9, 3u64, false, v11, List::<Bytes>::nil(), Adt20::C0(0u64))?)
                                } else {
                                    f239(v0, v10, v2, v8)
                                }
                            }
                            Some(v16) => {
                                Ok(Adt21::C1(v16))
                            }
                        }
                    }
                }
            }
        } else {
            let a1167 = v4;
            let a1168 = 1u64;
            if nat_eq(a1167, a1168) {
                let v17: u64 = f233(v0.clone(), v1)?;
                let v18: u64 = f154(v0.clone(), v17)?;
                if if f231(v18) {
                    true
                } else {
                    let a1169 = v18;
                    let a1170 = 35u64;
                    if nat_eq(a1169, a1170) {
                        let a1171 = v1;
                        let a1172 = v17;
                        nat_lt(a1171, a1172)
                    } else {
                        false
                    }
                } {
                    let v19: u64 = f97(v0.clone(), f234(v0.clone(), v17)?)?;
                    let a1174 = {
                        let a1173 = v0.clone();
                        length_bytes(a1173)
                    };
                    let a1175 = v19;
                    if if nat_le(a1174, a1175) {
                        true
                    } else {
                        f152(v0.clone(), v19)?
                    } {
                        f245(v8, v17, v19)
                    } else {
                        let v20: u64 = {
                            let a1176 = f235(v0.clone(), v19)?;
                            let a1177 = v19;
                            nat_sub(a1176, a1177)
                        };
                        let a1178 = v2;
                        let a1179 = v20;
                        if nat_le(a1178, a1179) {
                            f155(v0, v19, v2, v9, 0u64, false, 0u64, List::<Bytes>::nil(), v8)
                        } else {
                            if if if f156(v5) {
                                let a1180 = v20;
                                let a1181 = v6;
                                nat_eq(a1180, a1181)
                            } else {
                                false
                            } {
                                f236(v0.clone(), {
                                    let a1182 = v19;
                                    let a1183 = v20;
                                    nat_add(a1182, a1183)?
                                })?
                            } else {
                                false
                            } {
                                f244(v8, true, v17, f155(v0, {
                                    let a1184 = v19;
                                    let a1185 = v20;
                                    nat_add(a1184, a1185)?
                                }, v2, v9, 2u64, false, v20, List::<Bytes>::nil(), Adt20::C0(0u64))?)
                            } else {
                                f245(v8, v17, v19)
                            }
                        }
                    }
                } else {
                    let a1186 = v18;
                    let a1187 = 33u64;
                    if if nat_eq(a1186, a1187) {
                        f240(v8.clone())
                    } else {
                        false
                    } {
                        Ok(Adt21::C1(f98(Str::lit("a node holds at most one tag (YAML 1.2.2 [96])"), v17)?))
                    } else {
                        let a1188 = v18;
                        let a1189 = 33u64;
                        if if nat_eq(a1188, a1189) {
                            f156(if v5 {
                                f237(v0.clone(), v17)?
                            } else {
                                false
                            })
                        } else {
                            false
                        } {
                            let v21: Adt20 = f241(v0.clone(), v17)?;
                            let m1190 = f242(v21.clone());
                            match m1190 {
                                None => {
                                    f155(v0.clone(), f243(v0, v17)?, v2, v9, 1u64, v5, v6, List::<Bytes>::nil(), v21)
                                }
                                Some(v22) => {
                                    Ok(Adt21::C1(v22))
                                }
                            }
                        } else {
                            if if v5 {
                                f236(v0.clone(), v17)?
                            } else {
                                false
                            } {
                                let a1193 = f246(f247(v0.clone(), v1, v17), 9u64)?;
                                let a1194 = {
                                    let a1191 = v17;
                                    let a1192 = v1;
                                    nat_sub(a1191, a1192)
                                };
                                if nat_lt(a1193, a1194) {
                                    Ok(Adt21::C1(f98(Str::lit("a tab cannot indent a block node (YAML 1.2.2 [63])"), v17)?))
                                } else {
                                    f244(v8, true, v17, f155(v0.clone(), v17, v2, v9, 2u64, false, f232(v0, v17)?, List::<Bytes>::nil(), Adt20::C0(0u64))?)
                                }
                            } else {
                                let m1197 = {
                                    let a1195 = v18;
                                    let a1196 = 33u64;
                                    if nat_eq(a1195, a1196) {
                                        None::<Bytes>
                                    } else {
                                        f238(v0.clone(), v17)?
                                    }
                                };
                                match m1197 {
                                    None => {
                                        if if v5 {
                                            f237(v0.clone(), v17)?
                                        } else {
                                            false
                                        } {
                                            f244(v8, false, v17, f155(v0.clone(), v17, v2, v9, 3u64, false, f232(v0, v17)?, List::<Bytes>::nil(), Adt20::C0(0u64))?)
                                        } else {
                                            f239(v0, v17, v2, v8)
                                        }
                                    }
                                    Some(v23) => {
                                        Ok(Adt21::C1(v23))
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                let a1198 = v4;
                let a1199 = 2u64;
                if nat_eq(a1198, a1199) {
                    let m1204 = f155(v0.clone(), {
                        let a1200 = v1;
                        let a1201 = 1u64;
                        nat_add(a1200, a1201)?
                    }, {
                        let a1202 = v6;
                        let a1203 = 1u64;
                        nat_add(a1202, a1203)?
                    }, v9, 1u64, true, v6, List::<Bytes>::nil(), Adt20::C0(0u64))?;
                    match m1204 {
                        Adt21::C0(v25, v26) => {
                            let v27: u64 = f97(v0.clone(), v26)?;
                            let v28: u64 = {
                                let a1205 = f235(v0.clone(), v27)?;
                                let a1206 = v27;
                                nat_sub(a1205, a1206)
                            };
                            let a1208 = {
                                let a1207 = v0.clone();
                                length_bytes(a1207)
                            };
                            let a1209 = v27;
                            if nat_le(a1208, a1209) {
                                Ok(Adt21::C0(Adt2::C9(std::rc::Rc::new(v25), std::rc::Rc::new(Adt2::C8)), v27))
                            } else {
                                let a1210 = v28;
                                let a1211 = v6;
                                if if nat_eq(a1210, a1211) {
                                    f236(v0.clone(), {
                                        let a1212 = v27;
                                        let a1213 = v28;
                                        nat_add(a1212, a1213)?
                                    })?
                                } else {
                                    false
                                } {
                                    let m1216 = f155(v0, {
                                        let a1214 = v27;
                                        let a1215 = v28;
                                        nat_add(a1214, a1215)?
                                    }, v2, v9, 2u64, false, v6, List::<Bytes>::nil(), Adt20::C0(0u64))?;
                                    match m1216 {
                                        Adt21::C0(v30, v31) => {
                                            Ok(Adt21::C0(Adt2::C9(std::rc::Rc::new(v25), std::rc::Rc::new(v30)), v31))
                                        }
                                        Adt21::C1(v29) => {
                                            Ok(Adt21::C1(v29))
                                        }
                                    }
                                } else {
                                    let a1217 = v6;
                                    let a1218 = v28;
                                    if nat_lt(a1217, a1218) {
                                        Ok(Adt21::C1(f98(Str::lit("a line is indented more than its block sequence entry allows (YAML 1.2.2 [183])"), {
                                            let a1219 = v27;
                                            let a1220 = v28;
                                            nat_add(a1219, a1220)?
                                        })?))
                                    } else {
                                        Ok(Adt21::C0(Adt2::C9(std::rc::Rc::new(v25), std::rc::Rc::new(Adt2::C8)), v27))
                                    }
                                }
                            }
                        }
                        Adt21::C1(v24) => {
                            Ok(Adt21::C1(v24))
                        }
                    }
                } else {
                    let m1221 = f248(v0.clone(), v1)?;
                    match m1221 {
                        Ok(v33) => {
                            let m1222 = f238(v0.clone(), v33)?;
                            match m1222 {
                                None => {
                                    let m1223 = f249(v0.clone(), v33)?;
                                    match m1223 {
                                        None => {
                                            Ok(Adt21::C1(f98(Str::lit("a block mapping entry must be an implicit key and : (YAML 1.2.2 [188])"), v33)?))
                                        }
                                        Some(v35) => {
                                            let v36: Bytes = f250(v0.clone(), v33)?;
                                            if f251(v36.clone(), v7.clone()) {
                                                Ok(Adt21::C1(f98(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                            } else {
                                                let m1228 = f155(v0.clone(), {
                                                    let a1224 = v35;
                                                    let a1225 = 1u64;
                                                    nat_add(a1224, a1225)?
                                                }, {
                                                    let a1226 = v6;
                                                    let a1227 = 1u64;
                                                    nat_add(a1226, a1227)?
                                                }, v9, 1u64, false, v6, List::<Bytes>::nil(), Adt20::C0(0u64))?;
                                                match m1228 {
                                                    Adt21::C0(v38, v39) => {
                                                        let v40: u64 = f97(v0.clone(), v39)?;
                                                        let v41: u64 = {
                                                            let a1229 = f235(v0.clone(), v40)?;
                                                            let a1230 = v40;
                                                            nat_sub(a1229, a1230)
                                                        };
                                                        let a1232 = {
                                                            let a1231 = v0.clone();
                                                            length_bytes(a1231)
                                                        };
                                                        let a1233 = v40;
                                                        if nat_le(a1232, a1233) {
                                                            Ok(Adt21::C0(Adt2::C10(v36, std::rc::Rc::new(v38), std::rc::Rc::new(Adt2::C8)), v40))
                                                        } else {
                                                            let a1234 = v41;
                                                            let a1235 = v6;
                                                            if if if nat_eq(a1234, a1235) {
                                                                f156(f236(v0.clone(), {
                                                                    let a1236 = v40;
                                                                    let a1237 = v41;
                                                                    nat_add(a1236, a1237)?
                                                                })?)
                                                            } else {
                                                                false
                                                            } {
                                                                f156(f152(v0.clone(), v40)?)
                                                            } else {
                                                                false
                                                            } {
                                                                let m1240 = f155(v0, {
                                                                    let a1238 = v40;
                                                                    let a1239 = v41;
                                                                    nat_add(a1238, a1239)?
                                                                }, v2, v9, 3u64, false, v6, List::cons(v36.clone(), v7), Adt20::C0(0u64))?;
                                                                match m1240 {
                                                                    Adt21::C0(v43, v44) => {
                                                                        Ok(Adt21::C0(Adt2::C10(v36, std::rc::Rc::new(v38), std::rc::Rc::new(v43)), v44))
                                                                    }
                                                                    Adt21::C1(v42) => {
                                                                        Ok(Adt21::C1(v42))
                                                                    }
                                                                }
                                                            } else {
                                                                let a1241 = v6;
                                                                let a1242 = v41;
                                                                if nat_lt(a1241, a1242) {
                                                                    Ok(Adt21::C1(f98(Str::lit("a line is indented more than its block mapping entry allows (YAML 1.2.2 [187])"), {
                                                                        let a1243 = v40;
                                                                        let a1244 = v41;
                                                                        nat_add(a1243, a1244)?
                                                                    })?))
                                                                } else {
                                                                    Ok(Adt21::C0(Adt2::C10(v36, std::rc::Rc::new(v38), std::rc::Rc::new(Adt2::C8)), v40))
                                                                }
                                                            }
                                                        }
                                                    }
                                                    Adt21::C1(v37) => {
                                                        Ok(Adt21::C1(v37))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                Some(v34) => {
                                    Ok(Adt21::C1(v34))
                                }
                            }
                        }
                        Err(v32) => {
                            Ok(Adt21::C1(v32))
                        }
                    }
                }
            }
        }
    }
}

pub fn f156(v0: bool) -> bool {
    !v0
}

pub fn f157(v0: Bytes, v1: u64) -> Bytes {
    let m1248 = {
        let a1245 = v0;
        let a1246 = v1;
        let a1247 = 1u64;
        slice_bytes(a1245, a1246, a1247)
    };
    match m1248 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f158(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f252(v1, v0, v2)
}

pub fn f159(v0: Bytes) -> R<Result<List<Adt22>, Bytes>> {
    f253({
        let a1250 = {
            let a1249 = v0.clone();
            length_bytes(a1249)
        };
        let a1251 = 1u64;
        nat_add(a1250, a1251)?
    }, v0, 0u64, List::<Adt22>::nil())
}

pub fn f160(p1259: u64, p1260: List<Adt22>, p1261: List<Adt13>) -> R<Result<List<Adt13>, Bytes>> {
    let m1258 = (p1259, (p1260, p1261));
    {
        let mut m1258 = m1258;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1258;
                let m1252 = v0;
                if m1252 == 0 {
                    Err::<(u64, (List<Adt22>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>(Ok(Err::<List<Adt13>, Bytes>(f8(Str::lit("the input has more definitions than tokens")))))
                } else {
                    let v3 = m1252 - 1;
                    let m1253 = v1.clone();
                    match m1253.uncons() {
                        None => {
                            Err::<(u64, (List<Adt22>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>(Ok(Ok::<List<Adt13>, Bytes>(f254(v2, List::<Adt13>::nil()))))
                        }
                        Some((_, _)) => {
                            let m1254 = f255(v1)?;
                            match m1254 {
                                Ok(v7) => {
                                    let m1255 = f256(v3, v7)?;
                                    match m1255 {
                                        Ok(v9) => {
                                            Ok::<(u64, (List<Adt22>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>((v3, ({
                                                let (_, h1256) = v9;
                                                h1256
                                            }, List::cons({
                                                let (h1257, _) = v9;
                                                h1257
                                            }, v2))))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (List<Adt22>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>(Ok(Err::<List<Adt13>, Bytes>(v8)))
                                        }
                                    }
                                }
                                Err(v6) => {
                                    Err::<(u64, (List<Adt22>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>(Ok(Err::<List<Adt13>, Bytes>(v6)))
                                }
                            }
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

pub fn f161(v0: List<Adt16>, v1: List<Adt16>) -> List<Adt16> {
    let m1262 = v0;
    match m1262.uncons() {
        None => {
            List::<Adt16>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = if f46(f169(v2.clone()), Bytes::lit(&[])) {
                f257(f171(v2.clone()), v1.clone())
            } else {
                f169(v2.clone())
            };
            List::cons(Adt16::C0(f172(v2.clone()), v4, f173(v2.clone()), f171(v2)), f161(v3, v1))
        }
    }
}

pub fn f162(v0: List<Adt16>) -> R<u64> {
    let m1263 = v0;
    match m1263.uncons() {
        None => {
            Ok(0u64)
        }
        Some((v1, v2)) => {
            if f46(f169(v1), Bytes::lit(&[])) {
                f162(v2)
            } else {
                let a1264 = f162(v2)?;
                let a1265 = 1u64;
                nat_add(a1264, a1265)
            }
        }
    }
}

pub fn f163(v0: u64) -> bool {
    let a1266 = v0;
    let a1267 = 0u64;
    if nat_eq(a1266, a1267) {
        true
    } else {
        let a1268 = v0;
        let a1269 = 2u64;
        if nat_eq(a1268, a1269) {
            true
        } else {
            let a1270 = v0;
            let a1271 = 3u64;
            if nat_eq(a1270, a1271) {
                true
            } else {
                let a1272 = v0;
                let a1273 = 4u64;
                if nat_eq(a1272, a1273) {
                    true
                } else {
                    let a1274 = v0;
                    let a1275 = 9u64;
                    nat_eq(a1274, a1275)
                }
            }
        }
    }
}

pub fn f164(v0: Adt13) -> u64 {
    let m1276 = v0;
    match m1276 {
        Adt13::C0(v1, _, _, _, _, _) => {
            v1
        }
    }
}

pub fn f165(v0: List<Adt13>, v1: Adt13) -> R<Result<(Bytes, List<Bytes>), Bytes>> {
    let v2: u64 = f164(v1.clone());
    let v3: Bytes = f258(f166(v1.clone()))?;
    let a1277 = v2;
    let a1278 = 0u64;
    if if nat_eq(a1277, a1278) {
        true
    } else {
        let a1279 = v2;
        let a1280 = 2u64;
        nat_eq(a1279, a1280)
    } {
        let m1281 = f259(v0.clone(), v0.clone(), f166(v1.clone()));
        match m1281 {
            None => {
                let m1285 = f260({
                    let a1283 = {
                        let a1282 = v0.clone();
                        length_list(a1282)
                    };
                    let a1284 = 1u64;
                    nat_add(a1283, a1284)?
                }, v0, v1)?;
                match m1285 {
                    Ok(v5) => {
                        let a1287 = {
                            let a1286 = v5.clone();
                            length_list(a1286)
                        };
                        let a1288 = 0u64;
                        if nat_eq(a1287, a1288) {
                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f261(v3), List::<Bytes>::nil())))
                        } else {
                            let m1289 = f262(f263(f264(v5.clone()))?, v5)?;
                            match m1289 {
                                Ok(v7) => {
                                    Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f265(v3, {
                                        let (h1290, _) = v7;
                                        h1290
                                    }), {
                                        let (_, h1291) = v7;
                                        h1291
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
        let a1292 = v2;
        let a1293 = 3u64;
        if nat_eq(a1292, a1293) {
            let m1294 = f130(f266(v1.clone()));
            match m1294 {
                None => {
                    let m1295 = f267(f263(f266(v1.clone()))?, f268(f266(v1)), List::<Bytes>::nil())?;
                    match m1295 {
                        Ok(v10) => {
                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f269(v3, v10), List::<Bytes>::nil())))
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
            let a1296 = v2;
            let a1297 = 4u64;
            if nat_eq(a1296, a1297) {
                let v12: Adt15 = f270(v1);
                if !f46(f271(v12.clone()), Bytes::lit(&[])) {
                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f271(v12)))
                } else {
                    let a1298 = f272(v12.clone());
                    let a1299 = 1u64;
                    if nat_eq(a1298, a1299) {
                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f269(v3, List::cons(f273(v12.clone()), List::<Bytes>::nil())), f274(v12))))
                    } else {
                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f275(v3, f273(v12.clone())), f274(v12))))
                    }
                }
            } else {
                Ok(Err::<(Bytes, List<Bytes>), Bytes>(f7(f266(v1))))
            }
        }
    }
}

pub fn f166(v0: Adt13) -> Bytes {
    let m1300 = v0;
    match m1300 {
        Adt13::C0(_, v2, _, _, _, _) => {
            v2
        }
    }
}

pub fn f167(v0: Bytes, p1303: List<Bytes>) -> bool {
    let m1302 = p1303;
    {
        let mut m1302 = m1302;
        loop {
            let step = {
                let v1 = m1302;
                let m1301 = v1;
                match m1301.uncons() {
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
                Ok(next) => { m1302 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f168(p1306: List<Adt16>, v1: Bytes) -> Option<Adt16> {
    let m1305 = p1306;
    {
        let mut m1305 = m1305;
        loop {
            let step = {
                let v0 = m1305;
                let m1304 = v0;
                match m1304.uncons() {
                    None => {
                        Err::<List<Adt16>, Option<Adt16>>(None::<Adt16>)
                    }
                    Some((v2, v3)) => {
                        if f46(f172(v2.clone()), v1.clone()) {
                            Err::<List<Adt16>, Option<Adt16>>(Some(v2))
                        } else {
                            Ok::<List<Adt16>, Option<Adt16>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1305 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f169(v0: Adt16) -> Bytes {
    let m1307 = v0;
    match m1307 {
        Adt16::C0(_, v2, _, _) => {
            v2
        }
    }
}

pub fn f170(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m1308 = v0;
    match m1308.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f170(v3, v1))
        }
    }
}

pub fn f171(v0: Adt16) -> List<Bytes> {
    let m1309 = v0;
    match m1309 {
        Adt16::C0(_, _, _, v4) => {
            v4
        }
    }
}

pub fn f172(v0: Adt16) -> Bytes {
    let m1310 = v0;
    match m1310 {
        Adt16::C0(v1, _, _, _) => {
            v1
        }
    }
}

pub fn f173(v0: Adt16) -> Bytes {
    let m1311 = v0;
    match m1311 {
        Adt16::C0(_, _, v3, _) => {
            v3
        }
    }
}

pub fn f174(v0: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)> {
    let m1312 = v0;
    match m1312.uncons() {
        None => {
            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
        }
        Some((v2, v3)) => {
            let m1315 = f201({
                let (h1313, _) = {
                    let (_, h1314) = v2.clone();
                    h1314
                };
                h1313
            }, v1.clone());
            match m1315 {
                None => {
                    Some((({
                        let (h1316, _) = v2;
                        h1316
                    }, {
                        let (_, h1317) = {
                            let (_, h1318) = v2;
                            h1318
                        };
                        h1317
                    }), v3))
                }
                Some(_) => {
                    let m1319 = f174(v3, v1);
                    match m1319 {
                        None => {
                            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h1320, _) = v5;
                                h1320
                            }, List::cons(v2, {
                                let (_, h1321) = v5;
                                h1321
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f175(p1326: List<Adt13>, v1: u64, v2: Bytes) -> Option<Adt13> {
    let m1325 = p1326;
    {
        let mut m1325 = m1325;
        loop {
            let step = {
                let v0 = m1325;
                let m1322 = v0;
                match m1322.uncons() {
                    None => {
                        Err::<List<Adt13>, Option<Adt13>>(None::<Adt13>)
                    }
                    Some((v3, v4)) => {
                        let a1323 = f164(v3.clone());
                        let a1324 = v1;
                        if if nat_eq(a1323, a1324) {
                            f46(f166(v3.clone()), v2.clone())
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
                Ok(next) => { m1325 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f176(v0: u64) -> Bytes {
    let a1327 = v0;
    let a1328 = 5u64;
    if nat_eq(a1327, a1328) {
        f8(Str::lit("partial interface "))
    } else {
        let a1329 = v0;
        let a1330 = 7u64;
        if nat_eq(a1329, a1330) {
            f8(Str::lit("partial dictionary "))
        } else {
            f8(Str::lit(""))
        }
    }
}

pub fn f177(v0: Adt13) -> Bytes {
    let m1331 = v0;
    match m1331 {
        Adt13::C0(_, _, v3, _, _, _) => {
            v3
        }
    }
}

pub fn f178(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f276(v0, v1)
}

pub fn f179(v0: u64) -> R<bool> {
    f277(v0, f278())
}

pub fn f180(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let v3: u64 = {
        let (h1332, _) = f149(Fn4::F279(v1.clone(), v0.clone()), {
            let a1334 = {
                let a1333 = v0.clone();
                length_bytes(a1333)
            };
            let a1335 = 1u64;
            nat_add(a1334, a1335)?
        }, v2)?;
        h1332
    };
    if f94(v0, v1, v3) {
        Ok(Some(v3))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f181(v0: Bytes) -> Result<u64, Bytes> {
    Err::<u64, Bytes>(v0)
}

pub fn f182(v0: Bytes, v1: u64) -> R<u64> {
    let (h1336, _) = f149(Fn4::F280(v0.clone()), {
        let a1338 = {
            let a1337 = v0;
            length_bytes(a1337)
        };
        let a1339 = 1u64;
        nat_add(a1338, a1339)?
    }, v1)?;
    Ok(h1336)
}

pub fn f183(v0: Bytes) -> bool {
    f251(v0, List::cons(f8(Str::lit("UTF-8")), List::cons(f8(Str::lit("UTf-8")), List::cons(f8(Str::lit("UtF-8")), List::cons(f8(Str::lit("Utf-8")), List::cons(f8(Str::lit("uTF-8")), List::cons(f8(Str::lit("uTf-8")), List::cons(f8(Str::lit("utF-8")), List::cons(f8(Str::lit("utf-8")), List::<Bytes>::nil())))))))))
}

pub fn f184(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let m1343 = f180(v0.clone(), {
        let a1340 = Str::lit("--");
        utf8_encode(a1340)
    }, {
        let a1341 = v1;
        let a1342 = 4u64;
        nat_add(a1341, a1342)?
    })?;
    match m1343 {
        None => {
            Ok(f181(f98(Str::lit("a comment is not closed (XML 1.0 [15])"), v1)?))
        }
        Some(v2) => {
            let a1346 = f121(v0, {
                let a1344 = v2;
                let a1345 = 2u64;
                nat_add(a1344, a1345)?
            })?;
            let a1347 = 62u64;
            if nat_eq(a1346, a1347) {
                Ok(Ok::<u64, Bytes>({
                    let a1348 = v2;
                    let a1349 = 3u64;
                    nat_add(a1348, a1349)?
                }))
            } else {
                Ok(f181(f98(Str::lit("-- is not allowed inside a comment (XML 1.0 [15])"), v2)?))
            }
        }
    }
}

pub fn f185(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f187(v0.clone(), {
        let a1350 = v1;
        let a1351 = 2u64;
        nat_add(a1350, a1351)?
    })?;
    let a1354 = v2;
    let a1355 = {
        let a1352 = v1;
        let a1353 = 2u64;
        nat_add(a1352, a1353)?
    };
    if nat_eq(a1354, a1355) {
        Ok(f181(f98(Str::lit("a processing instruction must start with a Name (XML 1.0 [16])"), v1)?))
    } else {
        let a1358 = v2;
        let a1359 = {
            let a1356 = v1;
            let a1357 = 5u64;
            nat_add(a1356, a1357)?
        };
        if if nat_eq(a1358, a1359) {
            f251(f43(v0.clone(), {
                let a1360 = v1;
                let a1361 = 2u64;
                nat_add(a1360, a1361)?
            }, 3u64), List::cons(f8(Str::lit("XML")), List::cons(f8(Str::lit("XMl")), List::cons(f8(Str::lit("XmL")), List::cons(f8(Str::lit("Xml")), List::cons(f8(Str::lit("xML")), List::cons(f8(Str::lit("xMl")), List::cons(f8(Str::lit("xmL")), List::cons(f8(Str::lit("xml")), List::<Bytes>::nil())))))))))
        } else {
            false
        } {
            Ok(f181(f98(Str::lit("the target xml is reserved; an XML declaration may appear only at the very start (XML 1.0 [17], [22])"), v1)?))
        } else {
            let m1363 = f180(v0, {
                let a1362 = Str::lit("?>");
                utf8_encode(a1362)
            }, v2)?;
            match m1363 {
                None => {
                    Ok(f181(f98(Str::lit("a processing instruction is not closed (XML 1.0 [16])"), v1)?))
                }
                Some(v3) => {
                    Ok(Ok::<u64, Bytes>({
                        let a1364 = v3;
                        let a1365 = 2u64;
                        nat_add(a1364, a1365)?
                    }))
                }
            }
        }
    }
}

pub fn f186(v0: Bytes) -> Result<(Adt17, u64), Bytes> {
    Err::<(Adt17, u64), Bytes>(v0)
}

pub fn f187(v0: Bytes, v1: u64) -> R<u64> {
    let (h1366, _) = f149(Fn4::F281(v0.clone(), v1), {
        let a1368 = {
            let a1367 = v0;
            length_bytes(a1367)
        };
        let a1369 = 1u64;
        nat_add(a1368, a1369)?
    }, v1)?;
    Ok(h1366)
}

pub fn f188(v0: Bytes, v1: u64, v2: u64, v3: List<Bytes>) -> R<Result<(Adt17, u64), Bytes>> {
    let m1370 = v2;
    if m1370 == 0 {
        Ok(f186(f98(Str::lit("a tag has more attributes than the input has bytes"), v1)?))
    } else {
        let v4 = m1370 - 1;
        let v5: u64 = f182(v0.clone(), v1)?;
        let a1371 = f121(v0.clone(), v5)?;
        let a1372 = 62u64;
        if if nat_eq(a1371, a1372) {
            true
        } else {
            f120(v0.clone(), v5, Str::lit("/>"))
        } {
            Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C4, v5)))
        } else {
            let a1373 = v5;
            let a1374 = v1;
            if nat_eq(a1373, a1374) {
                Ok(f186(f98(Str::lit("attributes must be separated by white space (XML 1.0 [40])"), v1)?))
            } else {
                let v6: u64 = f187(v0.clone(), v5)?;
                let a1375 = v6;
                let a1376 = v5;
                if nat_eq(a1375, a1376) {
                    Ok(f186(f98(Str::lit("an attribute must start with a Name (XML 1.0 [41])"), v5)?))
                } else {
                    let v7: Bytes = f43(v0.clone(), v5, {
                        let a1377 = v6;
                        let a1378 = v5;
                        nat_sub(a1377, a1378)
                    });
                    if f251(v7.clone(), v3.clone()) {
                        Ok(f186(f98(Str::lit("an attribute name appears twice in one tag (XML 1.0 WFC: Unique Att Spec)"), v5)?))
                    } else {
                        let v8: u64 = f182(v0.clone(), v6)?;
                        let a1379 = f121(v0.clone(), v8)?;
                        let a1380 = 61u64;
                        if nat_eq(a1379, a1380) {
                            let v9: u64 = f182(v0.clone(), {
                                let a1381 = v8;
                                let a1382 = 1u64;
                                nat_add(a1381, a1382)?
                            })?;
                            let v10: u64 = f121(v0.clone(), v9)?;
                            let a1383 = v10;
                            let a1384 = 34u64;
                            if if nat_eq(a1383, a1384) {
                                true
                            } else {
                                let a1385 = v10;
                                let a1386 = 39u64;
                                nat_eq(a1385, a1386)
                            } {
                                let m1389 = f189(v0.clone(), {
                                    let a1387 = v9;
                                    let a1388 = 1u64;
                                    nat_add(a1387, a1388)?
                                }, v4, Bytes::lit(&[]), true, v10)?;
                                match m1389 {
                                    Ok(v12) => {
                                        let m1391 = f188(v0, {
                                            let (_, h1390) = v12;
                                            h1390
                                        }, v4, List::cons(v7.clone(), v3))?;
                                        match m1391 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C1(v7, {
                                                    let (h1392, _) = v12;
                                                    h1392
                                                }, std::rc::Rc::new({
                                                    let (h1393, _) = v14;
                                                    h1393
                                                })), {
                                                    let (_, h1394) = v14;
                                                    h1394
                                                })))
                                            }
                                            Err(v13) => {
                                                Ok(f186(v13))
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(f186(v11))
                                    }
                                }
                            } else {
                                Ok(f186(f98(Str::lit("an attribute value must be quoted (XML 1.0 [10])"), v9)?))
                            }
                        } else {
                            Ok(f186(f98(Str::lit("an attribute needs = and a value (XML 1.0 [25], [41])"), v8)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f189(v0: Bytes, p1434: u64, p1435: u64, p1436: Bytes, v4: bool, v5: u64) -> R<Result<(Bytes, u64), Bytes>> {
    let m1433 = (p1434, (p1435, p1436));
    {
        let mut m1433 = m1433;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1433;
                let m1395 = v2;
                if m1395 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f282(f98(Str::lit("the input ends inside a text run"), v1)?)))
                } else {
                    let v6 = m1395 - 1;
                    let v7: u64 = f121(v0.clone(), v1)?;
                    if if v4 {
                        let a1396 = v7;
                        let a1397 = v5;
                        nat_eq(a1396, a1397)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, {
                            let a1398 = v1;
                            let a1399 = 1u64;
                            nat_add(a1398, a1399)?
                        }))))
                    } else {
                        let a1400 = v7;
                        let a1401 = 256u64;
                        if nat_eq(a1400, a1401) {
                            if v4 {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f282(f98(Str::lit("an attribute value is not closed (XML 1.0 [10])"), v1)?)))
                            } else {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                            }
                        } else {
                            let a1402 = v7;
                            let a1403 = 60u64;
                            if nat_eq(a1402, a1403) {
                                if v4 {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f282(f98(Str::lit("< is not allowed in an attribute value (XML 1.0 [10], WFC: No < in Attribute Values)"), v1)?)))
                                } else {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                                }
                            } else {
                                let a1404 = v7;
                                let a1405 = 38u64;
                                if nat_eq(a1404, a1405) {
                                    let m1406 = f283(v0.clone(), v1)?;
                                    match m1406 {
                                        Ok(v9) => {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let (_, h1407) = v9;
                                                h1407
                                            }, (v6, {
                                                let a1409 = v3;
                                                let a1410 = {
                                                    let (h1408, _) = v9;
                                                    h1408
                                                };
                                                append_bytes(a1409, a1410)
                                            })))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f282(v8)))
                                        }
                                    }
                                } else {
                                    if v4 {
                                        let a1411 = v7;
                                        let a1412 = 9u64;
                                        if if nat_eq(a1411, a1412) {
                                            true
                                        } else {
                                            let a1413 = v7;
                                            let a1414 = 10u64;
                                            nat_eq(a1413, a1414)
                                        } {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let a1415 = v1;
                                                let a1416 = 1u64;
                                                nat_add(a1415, a1416)?
                                            }, (v6, {
                                                let a1417 = v3;
                                                let a1418 = f8(Str::lit(" "));
                                                append_bytes(a1417, a1418)
                                            })))
                                        } else {
                                            let v10: u64 = f284(v0.clone(), true, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v10, (v6, {
                                                let a1421 = v3;
                                                let a1422 = f43(v0.clone(), v1, {
                                                    let a1419 = v10;
                                                    let a1420 = v1;
                                                    nat_sub(a1419, a1420)
                                                });
                                                append_bytes(a1421, a1422)
                                            })))
                                        }
                                    } else {
                                        let a1423 = v7;
                                        let a1424 = 93u64;
                                        if nat_eq(a1423, a1424) {
                                            if f120(v0.clone(), v1, Str::lit("]]>")) {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f282(f98(Str::lit("]]> is not allowed in character data (XML 1.0 [14])"), v1)?)))
                                            } else {
                                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                    let a1425 = v1;
                                                    let a1426 = 1u64;
                                                    nat_add(a1425, a1426)?
                                                }, (v6, {
                                                    let a1427 = v3;
                                                    let a1428 = f8(Str::lit("]"));
                                                    append_bytes(a1427, a1428)
                                                })))
                                            }
                                        } else {
                                            let v11: u64 = f284(v0.clone(), false, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v11, (v6, {
                                                let a1431 = v3;
                                                let a1432 = f43(v0.clone(), v1, {
                                                    let a1429 = v11;
                                                    let a1430 = v1;
                                                    nat_sub(a1429, a1430)
                                                });
                                                append_bytes(a1431, a1432)
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
                Ok(next) => { m1433 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f190(v0: Bytes) -> Result<List<(Bytes, Bytes)>, Bytes> {
    Err::<List<(Bytes, Bytes)>, Bytes>(v0)
}

pub fn f191() -> Bytes {
    f8(Str::lit("http://www.w3.org/XML/1998/namespace"))
}

pub fn f192() -> Bytes {
    f8(Str::lit("http://www.w3.org/2000/xmlns/"))
}

pub fn f193(v0: Bytes, v1: Bytes) -> bool {
    let m1441 = {
        let a1438 = v0;
        let a1439 = 0u64;
        let a1440 = {
            let a1437 = v1.clone();
            length_bytes(a1437)
        };
        slice_bytes(a1438, a1439, a1440)
    };
    match m1441 {
        None => {
            false
        }
        Some(v2) => {
            let a1442 = v2;
            let a1443 = v1;
            equal(a1442, a1443)
        }
    }
}

pub fn f194(v0: Bytes) -> R<Bytes> {
    let m1444 = f180(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1444 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v1) => {
            Ok(f43(v0, 0u64, v1))
        }
    }
}

pub fn f195(p1449: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1448 = p1449;
    {
        let mut m1448 = m1448;
        loop {
            let step = {
                let v0 = m1448;
                let m1445 = v0;
                match m1445.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f46({
                            let (h1446, _) = v2;
                            h1446
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h1447) = v2;
                                h1447
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1448 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f196(v0: Bytes) -> R<Bytes> {
    let m1450 = f180(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1450 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(f43(v0.clone(), {
                let a1451 = v1;
                let a1452 = 1u64;
                nat_add(a1451, a1452)?
            }, {
                let a1456 = {
                    let a1453 = v0;
                    length_bytes(a1453)
                };
                let a1457 = {
                    let a1454 = v1;
                    let a1455 = 1u64;
                    nat_add(a1454, a1455)?
                };
                nat_sub(a1456, a1457)
            }))
        }
    }
}

pub fn f197() -> Bytes {
    f8(Str::lit("http://www.w3.org/2001/XMLSchema"))
}

pub fn f198(v0: Bytes) -> Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes> {
    Err::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v0)
}

pub fn f199(v0: Bytes, v1: Adt17, v2: List<(Bytes, Bytes)>, v3: Bytes) -> R<Result<Adt23, Bytes>> {
    let v4: Bytes = f200(v0.clone())?;
    let m1458 = f285(v1, v2)?;
    match m1458 {
        Ok(v6) => {
            let m1459 = v6;
            match m1459 {
                None => {
                    Ok(f286(f287(f8(Str::lit("a simpleType must have a restriction, list or union child (XSD 1.1 Part 2, section 4.1.2)")), v0)))
                }
                Some(v7) => {
                    let v8: Bytes = match v7.clone() {
                        Adt28::C0(h1460, _, _, _) => {
                            h1460
                        }
                    };
                    let v9: Adt17 = match v7.clone() {
                        Adt28::C0(_, h1461, _, _) => {
                            h1461
                        }
                    };
                    let v10: Adt17 = match v7.clone() {
                        Adt28::C0(_, _, h1462, _) => {
                            h1462
                        }
                    };
                    let v11: List<(Bytes, Bytes)> = match v7 {
                        Adt28::C0(_, _, _, h1463) => {
                            h1463
                        }
                    };
                    if f46(v8.clone(), f8(Str::lit("union"))) {
                        Ok(f286(f287(f8(Str::lit("a union is not translated, because a literal can be valid for more than one member type, so no LexLean inductive keeps its members disjoint (XSD 1.1 Part 2, section 2.4.1.3)")), v0)))
                    } else {
                        if f46(v8, f8(Str::lit("list"))) {
                            let m1464 = f127(v9, f8(Str::lit("itemType")));
                            match m1464 {
                                None => {
                                    Ok(f286(f287(f8(Str::lit("a list with an anonymous item type is not translated yet")), v0)))
                                }
                                Some(v12) => {
                                    let m1465 = f288(v11, v3, v12)?;
                                    match m1465 {
                                        Ok(v14) => {
                                            if f193(match v14.clone() {
                                                Adt23::C0(h1466, _) => {
                                                    h1466
                                                }
                                            }, f8(Str::lit("(List "))) {
                                                Ok(f286(f287(f8(Str::lit("the item type of a list must not itself be a list (XSD 1.1 Part 2, section 4.1.5)")), v0)))
                                            } else {
                                                Ok(Ok::<Adt23, Bytes>(Adt23::C0(f275(v4, f7(List::cons(f8(Str::lit("(List ")), List::cons(match v14.clone() {
                                                    Adt23::C0(h1467, _) => {
                                                        h1467
                                                    }
                                                }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))), match v14 {
                                                    Adt23::C0(_, h1468) => {
                                                        h1468
                                                    }
                                                })))
                                            }
                                        }
                                        Err(v13) => {
                                            Ok(f286(f287(v13, v0)))
                                        }
                                    }
                                }
                            }
                        } else {
                            let m1469 = f127(v9, f8(Str::lit("base")));
                            match m1469 {
                                None => {
                                    Ok(f286(f287(f8(Str::lit("a restriction with an anonymous base type is not translated yet")), v0)))
                                }
                                Some(v15) => {
                                    let m1470 = f288(v11.clone(), v3, v15)?;
                                    match m1470 {
                                        Ok(v17) => {
                                            let m1471 = f289(v10, v11)?;
                                            match m1471 {
                                                Ok(v19) => {
                                                    let v20: List<Adt2> = f290(v19, List::<Bytes>::nil())?;
                                                    let m1472 = v20.clone();
                                                    match m1472.uncons() {
                                                        None => {
                                                            Ok(Ok::<Adt23, Bytes>(Adt23::C0(f275(v4, match v17.clone() {
                                                                Adt23::C0(h1473, _) => {
                                                                    h1473
                                                                }
                                                            }), match v17 {
                                                                Adt23::C0(_, h1474) => {
                                                                    h1474
                                                                }
                                                            })))
                                                        }
                                                        Some((_, _)) => {
                                                            let m1475 = f267(f263(f291(v20.clone()))?, v20, List::<Bytes>::nil())?;
                                                            match m1475 {
                                                                Ok(v24) => {
                                                                    Ok(Ok::<Adt23, Bytes>(Adt23::C0(f269(v4, v24), List::<Bytes>::nil())))
                                                                }
                                                                Err(v23) => {
                                                                    Ok(f286(f287(v23, v0)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Err(v18) => {
                                                    Ok(f286(f287(v18, v0)))
                                                }
                                            }
                                        }
                                        Err(v16) => {
                                            Ok(f286(f287(v16, v0)))
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
            Ok(f286(f287(v5, v0)))
        }
    }
}

pub fn f200(v0: Bytes) -> R<Bytes> {
    f292(v0.clone(), 0u64, {
        let a1476 = v0;
        length_bytes(a1476)
    }, true, Bytes::lit(&[]))
}

pub fn f201(p1479: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m1478 = p1479;
    {
        let mut m1478 = m1478;
        loop {
            let step = {
                let v0 = m1478;
                let m1477 = v0;
                match m1477.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f167(v2.clone(), v1.clone()) {
                            Ok::<List<Bytes>, Option<Bytes>>(v3)
                        } else {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v2))
                        }
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

pub fn f202(v0: Bytes) -> Result<List<Bytes>, Bytes> {
    Err::<List<Bytes>, Bytes>(v0)
}

pub fn f203(v0: List<Bytes>, v1: Option<Bytes>) -> R<List<(Bytes, Bytes)>> {
    let m1480 = v0;
    match m1480.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, v3)) => {
            if f46(v2.clone(), f8(Str::lit("[[authority]]"))) {
                f203(v3, None::<Bytes>)
            } else {
                let m1481 = f293(v2.clone(), f8(Str::lit("url = ")))?;
                match m1481 {
                    None => {
                        let m1482 = f293(v2, f8(Str::lit("artifact = ")))?;
                        match m1482 {
                            None => {
                                f203(v3, v1)
                            }
                            Some(v5) => {
                                let m1483 = v1.clone();
                                match m1483 {
                                    None => {
                                        f203(v3, v1)
                                    }
                                    Some(v6) => {
                                        Ok(List::cons((v6, v5), f203(v3, v1)?))
                                    }
                                }
                            }
                        }
                    }
                    Some(v4) => {
                        f203(v3, Some(v4))
                    }
                }
            }
        }
    }
}

pub fn f204(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a1484 = v0.clone();
        length_bytes(a1484)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h1485, _) = f57(Fn0::F294(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h1485
    };
    let v3: u64 = {
        let (h1486, _) = {
            let (_, h1487) = v2.clone();
            h1487
        };
        h1486
    };
    let v4: List<Bytes> = {
        let (_, h1488) = {
            let (_, h1489) = v2;
            h1489
        };
        h1488
    };
    let a1490 = v3;
    let a1491 = v1;
    if nat_lt(a1490, a1491) {
        Ok(f59(List::cons(f43(v0, v3, {
            let a1492 = v1;
            let a1493 = v3;
            nat_sub(a1492, a1493)
        }), v4)))
    } else {
        Ok(f59(v4))
    }
}

pub fn f205(v0: List<Adt19>) -> R<Result<(u64, Bytes), Bytes>> {
    let m1494 = f295(v0.clone())?;
    match m1494 {
        Ok(v2) => {
            let v3: Adt24 = Adt24::C0(v2.clone(), f296(v0.clone()));
            let m1495 = f297(f298(v0)?, v3, v2)?;
            match m1495 {
                Ok(v5) => {
                    let m1496 = f299(f300(v5), List::<Adt27>::nil());
                    match m1496 {
                        Ok(v7) => {
                            let v8: List<Bytes> = f301(v7.clone());
                            let m1497 = f302(v7.clone(), v8.clone());
                            match m1497 {
                                None => {
                                    let v10: List<(Bytes, List<Bytes>)> = f303(v7.clone(), v8.clone(), f304(v7.clone())?);
                                    let m1503 = f305({
                                        let a1501 = {
                                            let a1500 = v8.clone();
                                            length_list(a1500)
                                        };
                                        let a1502 = 1u64;
                                        nat_add(a1501, a1502)?
                                    }, v7.clone(), f306(v7, f307(v10, v8.clone(), v8, List::<Bytes>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil());
                                    match m1503 {
                                        Ok(v12) => {
                                            Ok(Ok::<(u64, Bytes), Bytes>(({
                                                let a1504 = v12.clone();
                                                length_list(a1504)
                                            }, f114(v12, f8(Str::lit("\u{a}"))))))
                                        }
                                        Err(v11) => {
                                            Ok(Err::<(u64, Bytes), Bytes>(v11))
                                        }
                                    }
                                }
                                Some(v9) => {
                                    Ok(Err::<(u64, Bytes), Bytes>(f209(f8(Str::lit("the type ")), {
                                        let (h1498, _) = v9;
                                        h1498
                                    }, f209(f8(Str::lit(" uses ")), {
                                        let (_, h1499) = v9;
                                        h1499
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

pub fn f206(p1507: List<Adt19>, p1508: List<Adt19>) -> List<Adt19> {
    let m1506 = (p1507, p1508);
    {
        let mut m1506 = m1506;
        loop {
            let step = {
                let (v0, v1) = m1506;
                let m1505 = v0;
                match m1505.uncons() {
                    None => {
                        Err::<(List<Adt19>, List<Adt19>), List<Adt19>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt19>, List<Adt19>), List<Adt19>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1506 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f207(v0: List<Adt19>) -> List<Bytes> {
    let m1509 = v0;
    match m1509.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt19::C0(h1510, _, _, _, _, _) => {
                    h1510
                }
            }, f207(v2))
        }
    }
}

pub fn f208(p1515: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m1514 = p1515;
    {
        let mut m1514 = m1514;
        loop {
            let step = {
                let v0 = m1514;
                let m1511 = v0;
                match m1511.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f46({
                            let (h1512, _) = v2;
                            h1512
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h1513) = v2;
                                h1513
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
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

pub fn f209(v0: Bytes, v1: Bytes, v2: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(v1, List::cons(v2, List::<Bytes>::nil()))))
}

pub fn f210(v0: Bytes, v1: Bytes, v2: Option<Bytes>) -> R<Result<Adt19, Bytes>> {
    let m1516 = f69(v1.clone())?;
    match m1516 {
        Ok(v4) => {
            let m1517 = v4;
            match m1517 {
                Adt17::C0(v5, r1522, r1523) => {
                    let v6 = (*r1522).clone();
                    let v7 = (*r1523).clone();
                    let m1518 = f124(v6.clone(), List::<(Bytes, Bytes)>::nil());
                    match m1518 {
                        Ok(v9) => {
                            let m1519 = f125(v9.clone(), v5)?;
                            match m1519 {
                                Ok(v11) => {
                                    if f126(v11, Str::lit("schema")) {
                                        Ok(Ok::<Adt19, Bytes>(Adt19::C0(v0, v2, {
                                            let m1520 = f127(v6, f8(Str::lit("targetNamespace")));
                                            match m1520 {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some(v12) => {
                                                    v12
                                                }
                                            }
                                        }, v9, v7, {
                                            let a1521 = v1;
                                            length_bytes(a1521)
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
            Ok(Err::<Adt19, Bytes>(f209(v0, f8(Str::lit(" is not well-formed XML: ")), v3)))
        }
    }
}

pub fn f211(p1528: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1527 = p1528;
    {
        let mut m1527 = m1527;
        loop {
            let step = {
                let v0 = m1527;
                let m1524 = v0;
                match m1524.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f46({
                            let (_, h1525) = v2;
                            h1525
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (h1526, _) = v2;
                                h1526
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1527 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f212(v0: Adt19, v1: Bytes, v2: Bytes) -> Option<Bytes> {
    if f46(v1.clone(), f8(Str::lit("root"))) {
        None::<Bytes>
    } else {
        if f46(match v0.clone() {
            Adt19::C0(_, _, h1529, _, _, _) => {
                h1529
            }
        }, v2.clone()) {
            None::<Bytes>
        } else {
            Some(f7(List::cons(match v0.clone() {
                Adt19::C0(h1530, _, _, _, _, _) => {
                    h1530
                }
            }, List::cons(f8(Str::lit(" has the target namespace ")), List::cons(match v0 {
                Adt19::C0(_, _, h1531, _, _, _) => {
                    h1531
                }
            }, List::cons(f8(Str::lit(", but the xs:")), List::cons(v1.clone(), List::cons(f8(Str::lit(" that names it requires ")), List::cons(v2, List::cons(if f46(v1, f8(Str::lit("include"))) {
                f8(Str::lit("; an included document without a target namespace is not adopted into the including one (XSD 1.1 Part 1, section 4.2.3)"))
            } else {
                f8(Str::lit(" (XSD 1.1 Part 1, section 4.2.6.2)"))
            }, List::<Bytes>::nil()))))))))))
        }
    }
}

pub fn f213(v0: Adt17, v1: List<(Bytes, Bytes)>) -> R<Result<List<Adt26>, Bytes>> {
    let m1532 = v0;
    match m1532 {
        Adt17::C0(_, _, _) => {
            Ok(Ok::<List<Adt26>, Bytes>(List::<Adt26>::nil()))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<List<Adt26>, Bytes>(List::<Adt26>::nil()))
        }
        Adt17::C2(_) => {
            Ok(Ok::<List<Adt26>, Bytes>(List::<Adt26>::nil()))
        }
        Adt17::C3(r1543, r1544) => {
            let v2 = (*r1543).clone();
            let v3 = (*r1544).clone();
            let m1533 = f213(v3, v1.clone())?;
            match m1533 {
                Ok(v5) => {
                    let m1534 = v2;
                    match m1534 {
                        Adt17::C0(v6, r1541, r1542) => {
                            let v7 = (*r1541).clone();
                            let v8 = (*r1542).clone();
                            let m1535 = f124(v7.clone(), v1);
                            match m1535 {
                                Ok(v10) => {
                                    let m1536 = f125(v10.clone(), v6)?;
                                    match m1536 {
                                        Ok(v12) => {
                                            if if f46({
                                                let (h1537, _) = v12.clone();
                                                h1537
                                            }, f197()) {
                                                f308(f46({
                                                    let (_, h1538) = v12.clone();
                                                    h1538
                                                }, f8(Str::lit("annotation"))))
                                            } else {
                                                false
                                            } {
                                                Ok(Ok::<List<Adt26>, Bytes>(List::cons(Adt26::C0({
                                                    let (h1539, _) = v12;
                                                    h1539
                                                }, {
                                                    let (_, h1540) = v12;
                                                    h1540
                                                }, v7, v8, v10), v5)))
                                            } else {
                                                Ok(Ok::<List<Adt26>, Bytes>(v5))
                                            }
                                        }
                                        Err(v11) => {
                                            Ok(Err::<List<Adt26>, Bytes>(v11))
                                        }
                                    }
                                }
                                Err(v9) => {
                                    Ok(Err::<List<Adt26>, Bytes>(v9))
                                }
                            }
                        }
                        Adt17::C1(_, _, _) => {
                            Ok(Ok::<List<Adt26>, Bytes>(v5))
                        }
                        Adt17::C2(_) => {
                            Ok(Ok::<List<Adt26>, Bytes>(v5))
                        }
                        Adt17::C3(_, _) => {
                            Ok(Ok::<List<Adt26>, Bytes>(v5))
                        }
                        Adt17::C4 => {
                            Ok(Ok::<List<Adt26>, Bytes>(v5))
                        }
                    }
                }
                Err(v4) => {
                    Ok(Err::<List<Adt26>, Bytes>(v4))
                }
            }
        }
        Adt17::C4 => {
            Ok(Ok::<List<Adt26>, Bytes>(List::<Adt26>::nil()))
        }
    }
}

pub fn f214(v0: List<Adt26>, v1: Adt19, v2: List<(Bytes, Bytes)>) -> R<Result<List<(Bytes, (Bytes, Bytes))>, Bytes>> {
    let m1545 = v0;
    match m1545.uncons() {
        None => {
            Ok(Ok::<List<(Bytes, (Bytes, Bytes))>, Bytes>(List::<(Bytes, (Bytes, Bytes))>::nil()))
        }
        Some((v3, v4)) => {
            let m1546 = f214(v4, v1.clone(), v2.clone())?;
            match m1546 {
                Ok(v6) => {
                    if if f309(v3.clone(), Str::lit("redefine")) {
                        true
                    } else {
                        if f309(v3.clone(), Str::lit("override")) {
                            true
                        } else {
                            f309(v3.clone(), Str::lit("defaultOpenContent"))
                        }
                    } {
                        Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                            Adt19::C0(h1547, _, _, _, _, _) => {
                                h1547
                            }
                        }, List::cons(f8(Str::lit(": xs:")), List::cons(match v3 {
                            Adt26::C0(_, h1548, _, _, _) => {
                                h1548
                            }
                        }, List::cons(f8(Str::lit(" is not translated, because it changes components that other schema documents define (XSD 1.1 Part 1, sections 4.2.4, 4.2.5 and 3.4.2.3)")), List::<Bytes>::nil())))))))
                    } else {
                        if if f309(v3.clone(), Str::lit("import")) {
                            true
                        } else {
                            f309(v3.clone(), Str::lit("include"))
                        } {
                            let m1549 = f310(v3.clone(), Str::lit("schemaLocation"));
                            match m1549 {
                                None => {
                                    Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                                        Adt19::C0(h1550, _, _, _, _, _) => {
                                            h1550
                                        }
                                    }, List::cons(f8(Str::lit(": an xs:")), List::cons(match v3 {
                                        Adt26::C0(_, h1551, _, _, _) => {
                                            h1551
                                        }
                                    }, List::cons(f8(Str::lit(" with no schemaLocation names no document whose components could be read (XSD 1.1 Part 1, section 4.2.6.2)")), List::<Bytes>::nil())))))))
                                }
                                Some(v7) => {
                                    let m1553 = f311(match v1.clone() {
                                        Adt19::C0(_, h1552, _, _, _, _) => {
                                            h1552
                                        }
                                    }, v7)?;
                                    match m1553 {
                                        Ok(v9) => {
                                            let m1555 = f312(v2, v9.clone());
                                            match m1555 {
                                                None => {
                                                    Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                                                        Adt19::C0(h1556, _, _, _, _, _) => {
                                                            h1556
                                                        }
                                                    }, List::cons(f8(Str::lit(": the schema document ")), List::cons(v9, List::cons(f8(Str::lit(" is not a bound authority, so its components cannot be read; stage its binding first (XSD 1.1 Part 1, section 4.2.6.2)")), List::<Bytes>::nil())))))))
                                                }
                                                Some(v10) => {
                                                    Ok(Ok::<List<(Bytes, (Bytes, Bytes))>, Bytes>(List::cons((v10, (match v3.clone() {
                                                        Adt26::C0(_, h1557, _, _, _) => {
                                                            h1557
                                                        }
                                                    }, if f309(v3.clone(), Str::lit("include")) {
                                                        match v1 {
                                                            Adt19::C0(_, _, h1558, _, _, _) => {
                                                                h1558
                                                            }
                                                        }
                                                    } else {
                                                        f313(v3, Str::lit("namespace"), Str::lit(""))
                                                    })), v6)))
                                                }
                                            }
                                        }
                                        Err(v8) => {
                                            Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f209(match v1 {
                                                Adt19::C0(h1554, _, _, _, _, _) => {
                                                    h1554
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

pub fn f215(v0: List<(Bytes, (Bytes, Bytes))>, v1: List<(Bytes, (Bytes, Bytes))>) -> List<(Bytes, (Bytes, Bytes))> {
    let m1559 = v0;
    match m1559.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f215(v3, v1))
        }
    }
}

pub fn f216(v0: List<(Bytes, Option<Bytes>)>, v1: List<(Bytes, Option<Bytes>)>) -> List<(Bytes, Option<Bytes>)> {
    let m1560 = v0;
    match m1560.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f216(v3, v1))
        }
    }
}

pub fn f217(v0: Adt2) -> Option<Bytes> {
    let m1561 = v0;
    match m1561 {
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
        Adt2::C6(r1564) => {
            let v3 = (*r1564).clone();
            f217(v3)
        }
        Adt2::C7(r1565) => {
            let v4 = (*r1565).clone();
            f217(v4)
        }
        Adt2::C8 => {
            None::<Bytes>
        }
        Adt2::C9(r1566, r1567) => {
            let v5 = (*r1566).clone();
            let v6 = (*r1567).clone();
            let m1562 = f217(v5);
            match m1562 {
                None => {
                    f217(v6)
                }
                Some(v7) => {
                    Some(v7)
                }
            }
        }
        Adt2::C10(_, r1568, r1569) => {
            let v9 = (*r1568).clone();
            let v10 = (*r1569).clone();
            let m1563 = f217(v9);
            match m1563 {
                None => {
                    f217(v10)
                }
                Some(v11) => {
                    Some(v11)
                }
            }
        }
    }
}

pub fn f218(v0: Adt2) -> R<Bytes> {
    let m1570 = v0;
    match m1570 {
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
            f314(v4, v5)
        }
        Adt2::C4(v6, v7, v8) => {
            f315(v6, v7, v8)
        }
        Adt2::C5(v9) => {
            f316(v9)
        }
        Adt2::C6(r1571) => {
            let v10 = (*r1571).clone();
            Ok(f317(f318(v10)?))
        }
        Adt2::C7(r1572) => {
            let v11 = (*r1572).clone();
            f319(f320(v11)?)
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

pub fn f219(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f85(v0.clone(), {
        let a1573 = v1;
        let a1574 = 1u64;
        nat_add(a1573, a1574)?
    })?;
    let v4: Bytes = {
        let a1575 = v3;
        let a1576 = 117u64;
        if nat_eq(a1575, a1576) {
            f321(v0, v1)?
        } else {
            f322(v3)
        }
    };
    let a1577 = v4.clone();
    let a1578 = Bytes::lit(&[]);
    if equal(a1577, a1578) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a1579 = v3;
            let a1580 = 117u64;
            if nat_eq(a1579, a1580) {
                let a1581 = v1;
                let a1582 = 6u64;
                nat_add(a1581, a1582)?
            } else {
                let a1583 = v1;
                let a1584 = 2u64;
                nat_add(a1583, a1584)?
            }
        }, ({
            let a1585 = v2;
            let a1586 = v4;
            append_bytes(a1585, a1586)
        }, 0u64))))
    }
}

pub fn f220(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f323(v0, v1)
}

pub fn f221(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f324(v0, v1)
}

pub fn f222(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1587 = v0.clone();
        length_bytes(a1587)
    };
    let a1588 = 0u64;
    let a1589 = v1;
    if nat_lt(a1588, a1589) {
        let a1590 = f325(v0.clone())?;
        let a1591 = v1;
        if nat_eq(a1590, a1591) {
            let a1592 = 1u64;
            let a1593 = v1;
            if nat_lt(a1592, a1593) {
                let m1596 = {
                    let a1594 = v0;
                    let a1595 = 0u64;
                    index_bytes(a1594, a1595)
                };
                match m1596 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1597 = v2;
                        let a1598 = 48u8;
                        Ok(!equal(a1597, a1598))
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

pub fn f223(v0: Fn5, p1603: u64, p1604: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1602 = (p1603, p1604);
    {
        let mut m1602 = m1602;
        loop {
            let step = {
                let (v1, v2) = m1602;
                let m1599 = v1;
                if m1599 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1599 - 1;
                    let m1601 = {
                        let c1600: Fn5 = v0.clone();
                        c1600.apply(v2)?
                    };
                    match m1601 {
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
                Ok(next) => { m1602 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f224(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f326(v0, v1)
}

pub fn f225(v0: Bytes) -> R<u64> {
    let (h1605, _) = f149(Fn4::F327(v0.clone()), {
        let a1607 = {
            let a1606 = v0;
            length_bytes(a1606)
        };
        let a1608 = 1u64;
        nat_add(a1607, a1608)?
    }, 0u64)?;
    Ok(h1605)
}

pub fn f226(v0: u64) -> Bytes {
    let m1609 = v0;
    if m1609 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1609 - 1;
        let a1610 = f8(Str::lit("0"));
        let a1611 = f226(v1);
        append_bytes(a1610, a1611)
    }
}

pub fn f227(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h1612, _) = v1;
        h1612
    };
    let a1614 = {
        let a1613 = v0.clone();
        length_bytes(a1613)
    };
    let a1615 = v2;
    if nat_le(a1614, a1615) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let a1616 = f121(v0.clone(), v2)?;
        let a1617 = 13u64;
        if nat_eq(a1616, a1617) {
            Ok(Some(({
                let a1620 = f121(v0, {
                    let a1618 = v2;
                    let a1619 = 1u64;
                    nat_add(a1618, a1619)?
                })?;
                let a1621 = 10u64;
                if nat_eq(a1620, a1621) {
                    let a1622 = v2;
                    let a1623 = 2u64;
                    nat_add(a1622, a1623)?
                } else {
                    let a1624 = v2;
                    let a1625 = 1u64;
                    nat_add(a1624, a1625)?
                }
            }, {
                let a1627 = {
                    let (_, h1626) = v1;
                    h1626
                };
                let a1628 = f8(Str::lit("\u{a}"));
                append_bytes(a1627, a1628)
            })))
        } else {
            let v4: u64 = {
                let m1629 = f180(v0.clone(), f8(Str::lit("\u{d}")), v2)?;
                match m1629 {
                    None => {
                        let a1630 = v0.clone();
                        length_bytes(a1630)
                    }
                    Some(v3) => {
                        v3
                    }
                }
            };
            Ok(Some((v4, {
                let a1634 = {
                    let (_, h1631) = v1;
                    h1631
                };
                let a1635 = f43(v0, v2, {
                    let a1632 = v4;
                    let a1633 = v2;
                    nat_sub(a1632, a1633)
                });
                append_bytes(a1634, a1635)
            })))
        }
    }
}

pub fn f228(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1636 = f328(v0, v1)?;
    match m1636 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f329({
                let (h1637, _) = v2;
                h1637
            }) {
                Ok(Some({
                    let a1639 = v1;
                    let a1640 = {
                        let (_, h1638) = v2;
                        h1638
                    };
                    nat_add(a1639, a1640)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f229(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1642 = v1;
    let a1643 = {
        let a1641 = v0.clone();
        length_bytes(a1641)
    };
    if if nat_lt(a1642, a1643) {
        f330(v0.clone(), v1)?
    } else {
        false
    } {
        Ok(Some(f234(v0, v1)?))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f230(v0: u64) -> bool {
    let a1644 = v0;
    let a1645 = 32u64;
    if nat_eq(a1644, a1645) {
        true
    } else {
        let a1646 = v0;
        let a1647 = 9u64;
        nat_eq(a1646, a1647)
    }
}

pub fn f231(v0: u64) -> bool {
    let a1648 = v0;
    let a1649 = 10u64;
    if nat_eq(a1648, a1649) {
        true
    } else {
        let a1650 = v0;
        let a1651 = 256u64;
        nat_eq(a1650, a1651)
    }
}

pub fn f232(v0: Bytes, v1: u64) -> R<u64> {
    let a1652 = v1;
    let a1653 = f331(v0, v1)?;
    Ok(nat_sub(a1652, a1653))
}

pub fn f233(v0: Bytes, v1: u64) -> R<u64> {
    let (h1654, _) = f149(Fn4::F332(v0.clone()), {
        let a1656 = {
            let a1655 = v0;
            length_bytes(a1655)
        };
        let a1657 = 1u64;
        nat_add(a1656, a1657)?
    }, v1)?;
    Ok(h1654)
}

pub fn f234(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f333(v0.clone(), v1)?;
    let a1659 = v2;
    let a1660 = {
        let a1658 = v0;
        length_bytes(a1658)
    };
    if nat_lt(a1659, a1660) {
        let a1661 = v2;
        let a1662 = 1u64;
        nat_add(a1661, a1662)
    } else {
        Ok(v2)
    }
}

pub fn f235(v0: Bytes, v1: u64) -> R<u64> {
    let (h1663, _) = f149(Fn4::F334(v0.clone()), {
        let a1665 = {
            let a1664 = v0;
            length_bytes(a1664)
        };
        let a1666 = 1u64;
        nat_add(a1665, a1666)?
    }, v1)?;
    Ok(h1663)
}

pub fn f236(v0: Bytes, v1: u64) -> R<bool> {
    let a1667 = f154(v0.clone(), v1)?;
    let a1668 = 45u64;
    if nat_eq(a1667, a1668) {
        if f230(f154(v0.clone(), {
            let a1669 = v1;
            let a1670 = 1u64;
            nat_add(a1669, a1670)?
        })?) {
            Ok(true)
        } else {
            Ok(f231(f154(v0, {
                let a1671 = v1;
                let a1672 = 1u64;
                nat_add(a1671, a1672)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f237(v0: Bytes, v1: u64) -> R<bool> {
    let m1673 = f249(v0, v1)?;
    match m1673 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f238(v0: Bytes, v1: u64) -> R<Option<Bytes>> {
    let v2: u64 = f154(v0.clone(), v1)?;
    let v3: bool = f156(f335(f154(v0, {
        let a1674 = v1;
        let a1675 = 1u64;
        nat_add(a1674, a1675)?
    })?, true));
    let a1676 = v2;
    let a1677 = 58u64;
    if if nat_eq(a1676, a1677) {
        v3
    } else {
        false
    } {
        Ok(Some(f98(Str::lit("a null key is not read: Studio.Value keys are strings (YAML 1.2.2 section 10.3.2)"), v1)?))
    } else {
        let a1678 = v2;
        let a1679 = 38u64;
        if nat_eq(a1678, a1679) {
            Ok(Some(f98(Str::lit("an anchor is not read (YAML 1.2.2 [101])"), v1)?))
        } else {
            let a1680 = v2;
            let a1681 = 42u64;
            if nat_eq(a1680, a1681) {
                Ok(Some(f98(Str::lit("an alias is not read (YAML 1.2.2 [104])"), v1)?))
            } else {
                let a1682 = v2;
                let a1683 = 33u64;
                if nat_eq(a1682, a1683) {
                    Ok(Some(f98(Str::lit("a tag is not read (YAML 1.2.2 [97])"), v1)?))
                } else {
                    let a1684 = v2;
                    let a1685 = 63u64;
                    if if nat_eq(a1684, a1685) {
                        v3
                    } else {
                        false
                    } {
                        Ok(Some(f98(Str::lit("an explicit key is not read (YAML 1.2.2 [143], [189])"), v1)?))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f239(v0: Bytes, v1: u64, v2: u64, v3: Adt20) -> R<Adt21> {
    let v4: u64 = f154(v0.clone(), v1)?;
    let a1686 = v4;
    let a1687 = 124u64;
    if if nat_eq(a1686, a1687) {
        true
    } else {
        let a1688 = v4;
        let a1689 = 62u64;
        nat_eq(a1688, a1689)
    } {
        let m1690 = f336(v0, v1, v2)?;
        match m1690 {
            Adt21::C0(v6, v7) => {
                let m1691 = f337(Some(v6.clone()));
                match m1691 {
                    None => {
                        Ok(Adt21::C0(v6, v7))
                    }
                    Some(v8) => {
                        f338(v3, v8, true, v1, v7)
                    }
                }
            }
            Adt21::C1(v5) => {
                Ok(Adt21::C1(v5))
            }
        }
    } else {
        let a1692 = v4;
        let a1693 = 34u64;
        if if nat_eq(a1692, a1693) {
            true
        } else {
            let a1694 = v4;
            let a1695 = 39u64;
            nat_eq(a1694, a1695)
        } {
            let m1696 = f339(v0.clone(), v1, v2, false)?;
            match m1696 {
                Adt29::C0(v10, v11) => {
                    let m1697 = f153(v0, v11)?;
                    match m1697 {
                        Ok(v13) => {
                            f338(v3, v10, true, v1, v13)
                        }
                        Err(v12) => {
                            Ok(Adt21::C1(v12))
                        }
                    }
                }
                Adt29::C1(v9) => {
                    Ok(Adt21::C1(v9))
                }
            }
        } else {
            let a1698 = v4;
            let a1699 = 91u64;
            if if nat_eq(a1698, a1699) {
                true
            } else {
                let a1700 = v4;
                let a1701 = 123u64;
                nat_eq(a1700, a1701)
            } {
                let m1705 = f340(v0.clone(), v1, v2, {
                    let a1703 = {
                        let a1702 = v0.clone();
                        length_bytes(a1702)
                    };
                    let a1704 = 2u64;
                    nat_add(a1703, a1704)?
                }, 0u64, List::<Bytes>::nil(), v3)?;
                match m1705 {
                    Adt21::C0(v15, v16) => {
                        let a1706 = f154(v0.clone(), f233(v0.clone(), v16)?)?;
                        let a1707 = 58u64;
                        if nat_eq(a1706, a1707) {
                            Ok(Adt21::C1(f98(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [154])"), v1)?))
                        } else {
                            let m1708 = f153(v0, v16)?;
                            match m1708 {
                                Ok(v18) => {
                                    Ok(Adt21::C0(v15, v18))
                                }
                                Err(v17) => {
                                    Ok(Adt21::C1(v17))
                                }
                            }
                        }
                    }
                    Adt21::C1(v14) => {
                        Ok(Adt21::C1(v14))
                    }
                }
            } else {
                let m1709 = f238(v0.clone(), v1)?;
                match m1709 {
                    None => {
                        if f156(f341(v0.clone(), v1, false)?) {
                            Ok(Adt21::C1(f98(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                        } else {
                            let v20: u64 = f342(v0.clone(), v1, false)?;
                            let m1713 = f343(v0.clone(), v20, v2, {
                                let a1711 = {
                                    let a1710 = v0.clone();
                                    length_bytes(a1710)
                                };
                                let a1712 = 2u64;
                                nat_add(a1711, a1712)?
                            }, f247(v0, v1, v20))?;
                            match m1713 {
                                Adt29::C0(v22, v23) => {
                                    f338(v3, v22, false, v1, v23)
                                }
                                Adt29::C1(v21) => {
                                    Ok(Adt21::C1(v21))
                                }
                            }
                        }
                    }
                    Some(v19) => {
                        Ok(Adt21::C1(v19))
                    }
                }
            }
        }
    }
}

pub fn f240(v0: Adt20) -> bool {
    let m1714 = v0;
    match m1714 {
        Adt20::C0(v2) => {
            let a1715 = 0u64;
            let a1716 = v2;
            nat_lt(a1715, a1716)
        }
        Adt20::C1(_) => {
            true
        }
        Adt20::C2(_) => {
            true
        }
    }
}

pub fn f241(v0: Bytes, v1: u64) -> R<Adt20> {
    let v2: u64 = f344(v0.clone(), v1)?;
    let a1719 = f154(v0.clone(), {
        let a1717 = v1;
        let a1718 = 1u64;
        nat_add(a1717, a1718)?
    })?;
    let a1720 = 60u64;
    if nat_eq(a1719, a1720) {
        let a1721 = f154(v0.clone(), v2)?;
        let a1722 = 62u64;
        if nat_eq(a1721, a1722) {
            let v3: Bytes = f247(v0, {
                let a1723 = v1;
                let a1724 = 2u64;
                nat_add(a1723, a1724)?
            }, v2);
            if f120(v3.clone(), 0u64, Str::lit("tag:yaml.org,2002:")) {
                f345(f247(v3.clone(), 18u64, {
                    let a1725 = v3;
                    length_bytes(a1725)
                }), v1)
            } else {
                Ok(Adt20::C1(v3))
            }
        } else {
            Ok(Adt20::C2(f98(Str::lit("a verbatim tag must end with > (YAML 1.2.2 [98])"), v1)?))
        }
    } else {
        let a1728 = v2;
        let a1729 = {
            let a1726 = v1;
            let a1727 = 1u64;
            nat_add(a1726, a1727)?
        };
        if nat_eq(a1728, a1729) {
            Ok(Adt20::C0(8u64))
        } else {
            let a1732 = f154(v0.clone(), {
                let a1730 = v1;
                let a1731 = 1u64;
                nat_add(a1730, a1731)?
            })?;
            let a1733 = 33u64;
            if nat_eq(a1732, a1733) {
                f345(f247(v0, {
                    let a1734 = v1;
                    let a1735 = 2u64;
                    nat_add(a1734, a1735)?
                }, v2), v1)
            } else {
                let a1742 = f246(f247(v0.clone(), {
                    let a1736 = v1;
                    let a1737 = 1u64;
                    nat_add(a1736, a1737)?
                }, v2), 33u64)?;
                let a1743 = {
                    let a1740 = v2;
                    let a1741 = {
                        let a1738 = v1;
                        let a1739 = 1u64;
                        nat_add(a1738, a1739)?
                    };
                    nat_sub(a1740, a1741)
                };
                if nat_lt(a1742, a1743) {
                    Ok(Adt20::C2(f98(Str::lit("a named tag handle is not read: it needs a TAG directive (YAML 1.2.2 [92], [82])"), v1)?))
                } else {
                    Ok(Adt20::C1(f247(v0, v1, v2)))
                }
            }
        }
    }
}

pub fn f242(v0: Adt20) -> Option<Bytes> {
    let m1744 = v0;
    match m1744 {
        Adt20::C0(_) => {
            None::<Bytes>
        }
        Adt20::C1(_) => {
            None::<Bytes>
        }
        Adt20::C2(v1) => {
            Some(v1)
        }
    }
}

pub fn f243(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f344(v0.clone(), v1)?;
    let a1747 = f154(v0.clone(), {
        let a1745 = v1;
        let a1746 = 1u64;
        nat_add(a1745, a1746)?
    })?;
    let a1748 = 60u64;
    if if nat_eq(a1747, a1748) {
        let a1749 = f154(v0, v2)?;
        let a1750 = 62u64;
        nat_eq(a1749, a1750)
    } else {
        false
    } {
        let a1751 = v2;
        let a1752 = 1u64;
        nat_add(a1751, a1752)
    } else {
        Ok(v2)
    }
}

pub fn f244(v0: Adt20, v1: bool, v2: u64, v3: Adt21) -> R<Adt21> {
    let m1753 = v3;
    match m1753 {
        Adt21::C0(v5, v6) => {
            let v7: Adt2 = if v1 {
                Adt2::C6(std::rc::Rc::new(v5))
            } else {
                Adt2::C7(std::rc::Rc::new(v5))
            };
            let m1754 = v0;
            match m1754 {
                Adt20::C0(v10) => {
                    let a1755 = v10;
                    let a1756 = 0u64;
                    if if if nat_eq(a1755, a1756) {
                        true
                    } else {
                        let a1757 = v10;
                        let a1758 = 8u64;
                        nat_eq(a1757, a1758)
                    } {
                        true
                    } else {
                        let a1759 = v10;
                        let a1760 = if v1 {
                            6u64
                        } else {
                            7u64
                        };
                        nat_eq(a1759, a1760)
                    } {
                        Ok(Adt21::C0(v7, v6))
                    } else {
                        Ok(Adt21::C1(f346(v2)?))
                    }
                }
                Adt20::C1(v9) => {
                    Ok(Adt21::C0(Adt2::C1(v9, std::rc::Rc::new(v7)), v6))
                }
                Adt20::C2(v8) => {
                    Ok(Adt21::C1(v8))
                }
            }
        }
        Adt21::C1(v4) => {
            Ok(Adt21::C1(v4))
        }
    }
}

pub fn f245(v0: Adt20, v1: u64, v2: u64) -> R<Adt21> {
    let m1761 = v0;
    match m1761 {
        Adt20::C0(v5) => {
            let a1762 = v5;
            let a1763 = 1u64;
            if nat_eq(a1762, a1763) {
                Ok(Adt21::C0(Adt2::C5(Bytes::lit(&[])), v2))
            } else {
                let a1764 = v5;
                let a1765 = 8u64;
                if nat_eq(a1764, a1765) {
                    Ok(Adt21::C1(f98(Str::lit("an empty node with the non-specific tag ! is not read (YAML 1.2.2 [100])"), v1)?))
                } else {
                    let a1766 = v5;
                    let a1767 = 0u64;
                    if if nat_eq(a1766, a1767) {
                        true
                    } else {
                        let a1768 = v5;
                        let a1769 = 5u64;
                        nat_eq(a1768, a1769)
                    } {
                        Ok(Adt21::C0(Adt2::C0, v2))
                    } else {
                        Ok(Adt21::C1(f346(v1)?))
                    }
                }
            }
        }
        Adt20::C1(v4) => {
            Ok(Adt21::C0(Adt2::C1(v4, std::rc::Rc::new(Adt2::C0)), v2))
        }
        Adt20::C2(v3) => {
            Ok(Adt21::C1(v3))
        }
    }
}

pub fn f246(v0: Bytes, v1: u64) -> R<u64> {
    let (h1770, _) = f149(Fn4::F347(v0.clone(), v1), {
        let a1772 = {
            let a1771 = v0;
            length_bytes(a1771)
        };
        let a1773 = 1u64;
        nat_add(a1772, a1773)?
    }, 0u64)?;
    Ok(h1770)
}

pub fn f247(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    f43(v0, v1, {
        let a1774 = v2;
        let a1775 = v1;
        nat_sub(a1774, a1775)
    })
}

pub fn f248(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let a1776 = f154(v0.clone(), v1)?;
    let a1777 = 33u64;
    if nat_eq(a1776, a1777) {
        let m1778 = f241(v0.clone(), v1)?;
        match m1778 {
            Adt20::C0(v4) => {
                let a1779 = v4;
                let a1780 = 1u64;
                if if nat_eq(a1779, a1780) {
                    true
                } else {
                    let a1781 = v4;
                    let a1782 = 8u64;
                    nat_eq(a1781, a1782)
                } {
                    Ok(Ok::<u64, Bytes>(f233(v0.clone(), f243(v0, v1)?)?))
                } else {
                    Ok(Err::<u64, Bytes>(f98(Str::lit("a mapping key tagged other than !!str is not read: Studio.Value keys are strings (YAML 1.2.2 section 10.3.2)"), v1)?))
                }
            }
            Adt20::C1(_) => {
                Ok(Err::<u64, Bytes>(f98(Str::lit("a mapping key tagged other than !!str is not read: Studio.Value keys are strings (YAML 1.2.2 section 10.3.2)"), v1)?))
            }
            Adt20::C2(v2) => {
                Ok(Err::<u64, Bytes>(v2))
            }
        }
    } else {
        Ok(Ok::<u64, Bytes>(v1))
    }
}

pub fn f249(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a1783 = f154(v0.clone(), v1)?;
        let a1784 = 33u64;
        if nat_eq(a1783, a1784) {
            f233(v0.clone(), f243(v0.clone(), v1)?)?
        } else {
            v1
        }
    };
    let v3: u64 = f154(v0.clone(), v2)?;
    let a1785 = v3;
    let a1786 = 34u64;
    if if nat_eq(a1785, a1786) {
        true
    } else {
        let a1787 = v3;
        let a1788 = 39u64;
        nat_eq(a1787, a1788)
    } {
        let m1789 = f339(v0.clone(), v2, 0u64, true)?;
        match m1789 {
            Adt29::C0(_, v6) => {
                let v7: u64 = f233(v0.clone(), v6)?;
                if f348(v0, v7)? {
                    Ok(Some(v7))
                } else {
                    Ok(None::<u64>)
                }
            }
            Adt29::C1(_) => {
                Ok(None::<u64>)
            }
        }
    } else {
        if f341(v0.clone(), v2, false)? {
            let v8: u64 = f233(v0.clone(), f342(v0.clone(), v2, false)?)?;
            if f348(v0, v8)? {
                Ok(Some(v8))
            } else {
                Ok(None::<u64>)
            }
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f250(v0: Bytes, v1: u64) -> R<Bytes> {
    let v2: u64 = f154(v0.clone(), v1)?;
    let a1790 = v2;
    let a1791 = 34u64;
    if if nat_eq(a1790, a1791) {
        true
    } else {
        let a1792 = v2;
        let a1793 = 39u64;
        nat_eq(a1792, a1793)
    } {
        let m1794 = f339(v0, v1, 0u64, true)?;
        match m1794 {
            Adt29::C0(v4, _) => {
                Ok(v4)
            }
            Adt29::C1(_) => {
                Ok(Bytes::lit(&[]))
            }
        }
    } else {
        Ok(f247(v0.clone(), v1, f342(v0, v1, false)?))
    }
}

pub fn f251(v0: Bytes, p1797: List<Bytes>) -> bool {
    let m1796 = p1797;
    {
        let mut m1796 = m1796;
        loop {
            let step = {
                let v1 = m1796;
                let m1795 = v1;
                match m1795.uncons() {
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
                Ok(next) => { m1796 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f252(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h1798, _) = v2;
        h1798
    };
    let a1799 = v3;
    let a1800 = 0u64;
    if nat_eq(a1799, a1800) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a1801 = v3;
            let a1802 = v1;
            let a1803 = 0u64;
            nat_quot(a1801, a1802, a1803)
        }, {
            let a1808 = f157(v0, {
                let a1804 = v3;
                let a1805 = v1;
                let a1806 = 0u64;
                nat_rem(a1804, a1805, a1806)
            });
            let a1809 = {
                let (_, h1807) = v2;
                h1807
            };
            append_bytes(a1808, a1809)
        }))
    }
}

pub fn f253(p1864: u64, v1: Bytes, p1865: u64, p1866: List<Adt22>) -> R<Result<List<Adt22>, Bytes>> {
    let m1863 = (p1864, (p1865, p1866));
    {
        let mut m1863 = m1863;
        loop {
            let step = {
                let (v0, (v2, v3)) = m1863;
                let m1810 = v0;
                if m1810 == 0 {
                    Err::<(u64, (u64, List<Adt22>)), R<Result<List<Adt22>, Bytes>>>(Ok(Err::<List<Adt22>, Bytes>(f349(Str::lit("the input has more tokens than bytes"), v2)?)))
                } else {
                    let v4 = m1810 - 1;
                    let v5: u64 = f121(v1.clone(), v2)?;
                    let a1811 = v5;
                    let a1812 = 256u64;
                    if nat_eq(a1811, a1812) {
                        Err::<(u64, (u64, List<Adt22>)), R<Result<List<Adt22>, Bytes>>>(Ok(Ok::<List<Adt22>, Bytes>(f350(v3, List::<Adt22>::nil()))))
                    } else {
                        if f179(v5)? {
                            Ok::<(u64, (u64, List<Adt22>)), R<Result<List<Adt22>, Bytes>>>((v4, ({
                                let a1813 = v2;
                                let a1814 = 1u64;
                                nat_add(a1813, a1814)?
                            }, v3)))
                        } else {
                            if f120(v1.clone(), v2, Str::lit("//")) {
                                let m1815 = f180(v1.clone(), Bytes::lit(&[0x0a]), v2)?;
                                match m1815 {
                                    None => {
                                        Err::<(u64, (u64, List<Adt22>)), R<Result<List<Adt22>, Bytes>>>(Ok(Ok::<List<Adt22>, Bytes>(f350(v3, List::<Adt22>::nil()))))
                                    }
                                    Some(v6) => {
                                        Ok::<(u64, (u64, List<Adt22>)), R<Result<List<Adt22>, Bytes>>>((v4, ({
                                            let a1816 = v6;
                                            let a1817 = 1u64;
                                            nat_add(a1816, a1817)?
                                        }, v3)))
                                    }
                                }
                            } else {
                                if f120(v1.clone(), v2, Str::lit("/*")) {
                                    let m1821 = f180(v1.clone(), {
                                        let a1818 = Str::lit("*/");
                                        utf8_encode(a1818)
                                    }, {
                                        let a1819 = v2;
                                        let a1820 = 2u64;
                                        nat_add(a1819, a1820)?
                                    })?;
                                    match m1821 {
                                        None => {
                                            Err::<(u64, (u64, List<Adt22>)), R<Result<List<Adt22>, Bytes>>>(Ok(Err::<List<Adt22>, Bytes>(f349(Str::lit("a comment is not closed (Web IDL, IDL grammar: comment)"), v2)?)))
                                        }
                                        Some(v7) => {
                                            Ok::<(u64, (u64, List<Adt22>)), R<Result<List<Adt22>, Bytes>>>((v4, ({
                                                let a1822 = v7;
                                                let a1823 = 2u64;
                                                nat_add(a1822, a1823)?
                                            }, v3)))
                                        }
                                    }
                                } else {
                                    let a1824 = v5;
                                    let a1825 = 34u64;
                                    if nat_eq(a1824, a1825) {
                                        let m1828 = f180(v1.clone(), Bytes::lit(&[0x22]), {
                                            let a1826 = v2;
                                            let a1827 = 1u64;
                                            nat_add(a1826, a1827)?
                                        })?;
                                        match m1828 {
                                            None => {
                                                Err::<(u64, (u64, List<Adt22>)), R<Result<List<Adt22>, Bytes>>>(Ok(Err::<List<Adt22>, Bytes>(f349(Str::lit("a string is not closed (Web IDL, IDL grammar: string)"), v2)?)))
                                            }
                                            Some(v8) => {
                                                Ok::<(u64, (u64, List<Adt22>)), R<Result<List<Adt22>, Bytes>>>((v4, ({
                                                    let a1829 = v8;
                                                    let a1830 = 1u64;
                                                    nat_add(a1829, a1830)?
                                                }, List::cons(Adt22::C0(3u64, f43(v1.clone(), {
                                                    let a1831 = v2;
                                                    let a1832 = 1u64;
                                                    nat_add(a1831, a1832)?
                                                }, {
                                                    let a1835 = v8;
                                                    let a1836 = {
                                                        let a1833 = v2;
                                                        let a1834 = 1u64;
                                                        nat_add(a1833, a1834)?
                                                    };
                                                    nat_sub(a1835, a1836)
                                                })), v3))))
                                            }
                                        }
                                    } else {
                                        let v9: u64 = {
                                            let a1837 = v5;
                                            let a1838 = 95u64;
                                            if if nat_eq(a1837, a1838) {
                                                true
                                            } else {
                                                let a1839 = v5;
                                                let a1840 = 45u64;
                                                nat_eq(a1839, a1840)
                                            } {
                                                let a1841 = v2;
                                                let a1842 = 1u64;
                                                nat_add(a1841, a1842)?
                                            } else {
                                                v2
                                            }
                                        };
                                        if f351(f121(v1.clone(), v9)?) {
                                            let v10: u64 = f352(v1.clone(), 3u64, {
                                                let a1843 = v9;
                                                let a1844 = 1u64;
                                                nat_add(a1843, a1844)?
                                            })?;
                                            Ok::<(u64, (u64, List<Adt22>)), R<Result<List<Adt22>, Bytes>>>((v4, (v10, List::cons(Adt22::C0(0u64, f43(v1.clone(), v2, {
                                                let a1845 = v10;
                                                let a1846 = v2;
                                                nat_sub(a1845, a1846)
                                            })), v3))))
                                        } else {
                                            let v11: u64 = {
                                                let a1847 = v5;
                                                let a1848 = 45u64;
                                                if nat_eq(a1847, a1848) {
                                                    let a1849 = v2;
                                                    let a1850 = 1u64;
                                                    nat_add(a1849, a1850)?
                                                } else {
                                                    v2
                                                }
                                            };
                                            let v12: u64 = f353(v1.clone(), v11)?;
                                            let v13: u64 = f354(v1.clone(), v11)?;
                                            let a1851 = v12;
                                            let a1852 = v13;
                                            if nat_lt(a1851, a1852) {
                                                Ok::<(u64, (u64, List<Adt22>)), R<Result<List<Adt22>, Bytes>>>((v4, (v13, List::cons(Adt22::C0(2u64, f43(v1.clone(), v2, {
                                                    let a1853 = v13;
                                                    let a1854 = v2;
                                                    nat_sub(a1853, a1854)
                                                })), v3))))
                                            } else {
                                                let a1855 = v11;
                                                let a1856 = v12;
                                                if nat_lt(a1855, a1856) {
                                                    Ok::<(u64, (u64, List<Adt22>)), R<Result<List<Adt22>, Bytes>>>((v4, (v12, List::cons(Adt22::C0(1u64, f43(v1.clone(), v2, {
                                                        let a1857 = v12;
                                                        let a1858 = v2;
                                                        nat_sub(a1857, a1858)
                                                    })), v3))))
                                                } else {
                                                    if f120(v1.clone(), v2, Str::lit("...")) {
                                                        Ok::<(u64, (u64, List<Adt22>)), R<Result<List<Adt22>, Bytes>>>((v4, ({
                                                            let a1859 = v2;
                                                            let a1860 = 3u64;
                                                            nat_add(a1859, a1860)?
                                                        }, List::cons(Adt22::C0(4u64, f8(Str::lit("..."))), v3))))
                                                    } else {
                                                        Ok::<(u64, (u64, List<Adt22>)), R<Result<List<Adt22>, Bytes>>>((v4, ({
                                                            let a1861 = v2;
                                                            let a1862 = 1u64;
                                                            nat_add(a1861, a1862)?
                                                        }, List::cons(Adt22::C0(4u64, f43(v1.clone(), v2, 1u64)), v3))))
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
                Ok(next) => { m1863 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f254(p1869: List<Adt13>, p1870: List<Adt13>) -> List<Adt13> {
    let m1868 = (p1869, p1870);
    {
        let mut m1868 = m1868;
        loop {
            let step = {
                let (v0, v1) = m1868;
                let m1867 = v0;
                match m1867.uncons() {
                    None => {
                        Err::<(List<Adt13>, List<Adt13>), List<Adt13>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt13>, List<Adt13>), List<Adt13>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1868 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f255(v0: List<Adt22>) -> R<Result<List<Adt22>, Bytes>> {
    if f355(v0.clone(), Str::lit("[")) {
        Ok(f356({
            let a1872 = {
                let a1871 = v0.clone();
                length_list(a1871)
            };
            let a1873 = 1u64;
            nat_add(a1872, a1873)?
        }, f357(v0), List::cons(f8(Str::lit("]")), List::<Bytes>::nil()), false))
    } else {
        Ok(Ok::<List<Adt22>, Bytes>(v0))
    }
}

pub fn f256(v0: u64, v1: List<Adt22>) -> R<Result<(Adt13, List<Adt22>), Bytes>> {
    if f355(v1.clone(), Str::lit("interface")) {
        if f355(f357(v1.clone()), Str::lit("mixin")) {
            f358(v0, 1u64, f357(f357(v1)), false)
        } else {
            f358(v0, 0u64, f357(v1), true)
        }
    } else {
        if f355(v1.clone(), Str::lit("partial")) {
            let v2: List<Adt22> = f357(v1);
            if f355(v2.clone(), Str::lit("interface")) {
                if f355(f357(v2.clone()), Str::lit("mixin")) {
                    f358(v0, 6u64, f357(f357(v2)), false)
                } else {
                    f358(v0, 5u64, f357(v2), false)
                }
            } else {
                if f355(v2.clone(), Str::lit("dictionary")) {
                    f358(v0, 7u64, f357(v2), false)
                } else {
                    if f355(v2.clone(), Str::lit("namespace")) {
                        f359(f357(v2), f8(Str::lit("a namespace holds operations and attributes of no object, so it is behavior, not data (Web IDL 2.6)")))
                    } else {
                        Ok(Err::<(Adt13, List<Adt22>), Bytes>(f360(Str::lit("interface, dictionary or namespace"), v2)))
                    }
                }
            }
        } else {
            if f355(v1.clone(), Str::lit("dictionary")) {
                f358(v0, 2u64, f357(v1), true)
            } else {
                if f355(v1.clone(), Str::lit("enum")) {
                    let m1874 = f361(f357(v1));
                    match m1874 {
                        Ok(v4) => {
                            let m1875 = f362(f363(v4.clone()), Str::lit("{"));
                            match m1875 {
                                Ok(v6) => {
                                    let m1876 = f364(v0, v6, List::<Bytes>::nil());
                                    match m1876 {
                                        Ok(v8) => {
                                            Ok(Ok::<(Adt13, List<Adt22>), Bytes>((Adt13::C0(3u64, f365(v4), Bytes::lit(&[]), List::<Adt14>::nil(), {
                                                let (h1877, _) = v8;
                                                h1877
                                            }, f366(Bytes::lit(&[]))), {
                                                let (_, h1878) = v8;
                                                h1878
                                            })))
                                        }
                                        Err(v7) => {
                                            Ok(Err::<(Adt13, List<Adt22>), Bytes>(v7))
                                        }
                                    }
                                }
                                Err(v5) => {
                                    Ok(Err::<(Adt13, List<Adt22>), Bytes>(v5))
                                }
                            }
                        }
                        Err(v3) => {
                            Ok(Err::<(Adt13, List<Adt22>), Bytes>(v3))
                        }
                    }
                } else {
                    if f355(v1.clone(), Str::lit("typedef")) {
                        let m1879 = f367(v0, f357(v1), 0u64)?;
                        match m1879 {
                            Ok(v10) => {
                                let m1880 = f361(f368(v10.clone()));
                                match m1880 {
                                    Ok(v12) => {
                                        let m1881 = f362(f363(v12.clone()), Str::lit(";"));
                                        match m1881 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt13, List<Adt22>), Bytes>((Adt13::C0(4u64, f365(v12), Bytes::lit(&[]), List::<Adt14>::nil(), List::<Bytes>::nil(), f369(v10)), v14)))
                                            }
                                            Err(v13) => {
                                                Ok(Err::<(Adt13, List<Adt22>), Bytes>(v13))
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(Err::<(Adt13, List<Adt22>), Bytes>(v11))
                                    }
                                }
                            }
                            Err(v9) => {
                                Ok(Err::<(Adt13, List<Adt22>), Bytes>(v9))
                            }
                        }
                    } else {
                        if f355(v1.clone(), Str::lit("callback")) {
                            if f355(f357(v1.clone()), Str::lit("interface")) {
                                f359(f357(f357(v1)), f8(Str::lit("a callback interface is implemented by user code, so it is behavior, not data (Web IDL 2.4)")))
                            } else {
                                f359(f357(v1), f8(Str::lit("a callback function is behavior, not data (Web IDL 2.10)")))
                            }
                        } else {
                            if f355(v1.clone(), Str::lit("namespace")) {
                                f359(f357(v1), f8(Str::lit("a namespace holds operations and attributes of no object, so it is behavior, not data (Web IDL 2.6)")))
                            } else {
                                let m1882 = f361(v1);
                                match m1882 {
                                    Ok(v16) => {
                                        let m1883 = f362(f363(v16.clone()), Str::lit("includes"));
                                        match m1883 {
                                            Ok(v18) => {
                                                let m1884 = f361(v18);
                                                match m1884 {
                                                    Ok(v20) => {
                                                        let m1885 = f362(f363(v20.clone()), Str::lit(";"));
                                                        match m1885 {
                                                            Ok(v22) => {
                                                                Ok(Ok::<(Adt13, List<Adt22>), Bytes>((Adt13::C0(8u64, f365(v16), f365(v20), List::<Adt14>::nil(), List::<Bytes>::nil(), f366(Bytes::lit(&[]))), v22)))
                                                            }
                                                            Err(v21) => {
                                                                Ok(Err::<(Adt13, List<Adt22>), Bytes>(v21))
                                                            }
                                                        }
                                                    }
                                                    Err(v19) => {
                                                        Ok(Err::<(Adt13, List<Adt22>), Bytes>(v19))
                                                    }
                                                }
                                            }
                                            Err(v17) => {
                                                Ok(Err::<(Adt13, List<Adt22>), Bytes>(v17))
                                            }
                                        }
                                    }
                                    Err(v15) => {
                                        Ok(Err::<(Adt13, List<Adt22>), Bytes>(v15))
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

pub fn f257(p1889: List<Bytes>, v1: List<Adt16>) -> Bytes {
    let m1888 = p1889;
    {
        let mut m1888 = m1888;
        loop {
            let step = {
                let v0 = m1888;
                let m1886 = v0;
                match m1886.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1887 = f168(v1.clone(), v2.clone());
                        match m1887 {
                            None => {
                                Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("refers to ")), List::cons(v2, List::cons(f8(Str::lit(", which no input defines (Web IDL 2.13.22 to 2.13.26)")), List::<Bytes>::nil())))))
                            }
                            Some(v4) => {
                                if f46(f169(v4.clone()), Bytes::lit(&[])) {
                                    Ok::<List<Bytes>, Bytes>(v3)
                                } else {
                                    Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("refers to ")), List::cons(v2, List::cons(f8(Str::lit(", which is not translated: ")), List::cons(f169(v4), List::<Bytes>::nil()))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1888 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f258(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f200(v0)?;
    if f167(v1.clone(), f370()) {
        f371(v1)
    } else {
        Ok(v1)
    }
}

pub fn f259(p1895: List<Adt13>, v1: List<Adt13>, v2: Bytes) -> Option<Bytes> {
    let m1894 = p1895;
    {
        let mut m1894 = m1894;
        loop {
            let step = {
                let v0 = m1894;
                let m1890 = v0;
                match m1890.uncons() {
                    None => {
                        Err::<List<Adt13>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v3, v4)) => {
                        let a1891 = f164(v3.clone());
                        let a1892 = 8u64;
                        if if nat_eq(a1891, a1892) {
                            f46(f166(v3.clone()), v2.clone())
                        } else {
                            false
                        } {
                            let m1893 = f175(v1.clone(), 1u64, f177(v3.clone()));
                            match m1893 {
                                None => {
                                    Err::<List<Adt13>, Option<Bytes>>(Some(f177(v3)))
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
                Ok(next) => { m1894 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f260(v0: u64, v1: List<Adt13>, v2: Adt13) -> R<Result<List<Adt14>, Bytes>> {
    let m1896 = v0;
    if m1896 == 0 {
        Ok(Err::<List<Adt14>, Bytes>(f7(List::cons(f8(Str::lit("the inheritance of ")), List::cons(f166(v2), List::cons(f8(Str::lit(" is cyclic (Web IDL 2.2 and 2.7: an interface or dictionary must not inherit from itself)")), List::<Bytes>::nil()))))))
    } else {
        let v3 = m1896 - 1;
        let v4: List<Adt14> = {
            let a1897 = f164(v2.clone());
            let a1898 = 2u64;
            if nat_eq(a1897, a1898) {
                f372(f373(f374(v2.clone()), f375(v1.clone(), 7u64, f166(v2.clone()))))?
            } else {
                f373(f374(v2.clone()), f373(f375(v1.clone(), 5u64, f166(v2.clone())), f376(v1.clone(), v1.clone(), f166(v2.clone()))))
            }
        };
        if f46(f177(v2.clone()), Bytes::lit(&[])) {
            Ok(Ok::<List<Adt14>, Bytes>(v4))
        } else {
            let m1899 = f175(v1.clone(), f164(v2.clone()), f177(v2.clone()));
            match m1899 {
                None => {
                    Ok(Err::<List<Adt14>, Bytes>(f7(List::cons(f8(Str::lit("inherits ")), List::cons(f177(v2.clone()), List::cons({
                        let a1900 = f164(v2);
                        let a1901 = 2u64;
                        if nat_eq(a1900, a1901) {
                            f8(Str::lit(", which no input defines as a dictionary (Web IDL 2.7)"))
                        } else {
                            f8(Str::lit(", which no input defines as an interface (Web IDL 2.2)"))
                        }
                    }, List::<Bytes>::nil()))))))
                }
                Some(v5) => {
                    let m1902 = f260(v3, v1, v5)?;
                    match m1902 {
                        Ok(v7) => {
                            Ok(Ok::<List<Adt14>, Bytes>(f373(v7, v4)))
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

pub fn f261(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: mk, fields: []}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))
}

pub fn f262(v0: bool, v1: List<Adt14>) -> R<Result<(List<Bytes>, List<Bytes>), Bytes>> {
    let m1903 = v1;
    match m1903.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::<Bytes>::nil(), List::<Bytes>::nil())))
        }
        Some((v2, v3)) => {
            let v4: Adt15 = f377(v2.clone());
            if !f46(f271(v4.clone()), Bytes::lit(&[])) {
                Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons(f378(v2), List::cons(f8(Str::lit(": ")), List::cons(f271(v4), List::<Bytes>::nil())))))))
            } else {
                let m1904 = f379(v0, f378(v2.clone()))?;
                match m1904 {
                    None => {
                        Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons(f378(v2), List::cons(f8(Str::lit(" has no field name")), List::<Bytes>::nil()))))))
                    }
                    Some(v5) => {
                        if f167(v5.clone(), f380(v0, f264(v3.clone()))?) {
                            Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("two of its members derive the field name ")), List::cons(v5, List::cons(f8(Str::lit(" (Web IDL 2.2 and 2.7: the members of one definition have distinct identifiers)")), List::<Bytes>::nil()))))))
                        } else {
                            let m1905 = f262(v0, v3)?;
                            match m1905 {
                                Ok(v7) => {
                                    Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v5, List::cons(f8(Str::lit(", type: ")), List::cons({
                                        let a1906 = f381(v2);
                                        let a1907 = 1u64;
                                        if nat_eq(a1906, a1907) {
                                            f7(List::cons(f8(Str::lit("(Option ")), List::cons(f273(v4.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                        } else {
                                            f273(v4.clone())
                                        }
                                    }, List::cons(f8(Str::lit("}")), List::<Bytes>::nil())))))), {
                                        let (h1908, _) = v7;
                                        h1908
                                    }), f170(f274(v4), {
                                        let (_, h1909) = v7;
                                        h1909
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

pub fn f263(p1913: List<Bytes>) -> R<bool> {
    let m1912 = p1913;
    {
        let mut m1912 = m1912;
        loop {
            let step = {
                let v0 = m1912;
                let m1910 = v0;
                match m1910.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m1911 = f382(v1)?;
                        match m1911 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f167(v3, f380(false, v2.clone())?) {
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
                Ok(next) => { m1912 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f264(v0: List<Adt14>) -> List<Bytes> {
    let m1914 = v0;
    match m1914.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f378(v1), f264(v2))
        }
    }
}

pub fn f265(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}")), List::cons(f114(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f266(v0: Adt13) -> List<Bytes> {
    let m1915 = v0;
    match m1915 {
        Adt13::C0(_, _, _, _, v5, _) => {
            v5
        }
    }
}

pub fn f267(v0: bool, v1: List<Adt2>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1916 = v1;
    match m1916.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m1917 = f383(v0, v3.clone())?;
            match m1917 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f384(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f167(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m1918 = f267(v0, v4, List::cons(v5.clone(), v2))?;
                        match m1918 {
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

pub fn f268(v0: List<Bytes>) -> List<Adt2> {
    let m1919 = v0;
    match m1919.uncons() {
        None => {
            List::<Adt2>::nil()
        }
        Some((v1, v2)) => {
            List::cons(Adt2::C5(v1), f268(v2))
        }
    }
}

pub fn f269(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f114(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f270(v0: Adt13) -> Adt15 {
    let m1920 = v0;
    match m1920 {
        Adt13::C0(_, _, _, _, _, v6) => {
            v6
        }
    }
}

pub fn f271(v0: Adt15) -> Bytes {
    let m1921 = v0;
    match m1921 {
        Adt15::C0(_, _, _, v4, _) => {
            v4
        }
    }
}

pub fn f272(v0: Adt15) -> u64 {
    let m1922 = v0;
    match m1922 {
        Adt15::C0(v1, _, _, _, _) => {
            v1
        }
    }
}

pub fn f273(v0: Adt15) -> Bytes {
    let m1923 = v0;
    match m1923 {
        Adt15::C0(_, v2, _, _, _) => {
            v2
        }
    }
}

pub fn f274(v0: Adt15) -> List<Bytes> {
    let m1924 = v0;
    match m1924 {
        Adt15::C0(_, _, v3, _, _) => {
            v3
        }
    }
}

pub fn f275(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f276(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1925 = f328(v0, v1)?;
    match m1925 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f385({
                let (h1926, _) = v2;
                h1926
            })? {
                Ok(Some({
                    let a1928 = v1;
                    let a1929 = {
                        let (_, h1927) = v2;
                        h1927
                    };
                    nat_add(a1928, a1929)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f277(v0: u64, v1: Bytes) -> R<bool> {
    f386(v0, v1.clone(), 0u64, {
        let a1931 = {
            let a1930 = v1;
            length_bytes(a1930)
        };
        let a1932 = 3u64;
        let a1933 = 0u64;
        nat_quot(a1931, a1932, a1933)
    }, true)
}

pub fn f278() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{0}\u{0} "))
}

pub fn f279(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f387(v1, v0, v2)
}

pub fn f280(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f388(v0, v1)
}

pub fn f281(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f389(v0, v1, v2)
}

pub fn f282(v0: Bytes) -> Result<(Bytes, u64), Bytes> {
    Err::<(Bytes, u64), Bytes>(v0)
}

pub fn f283(v0: Bytes, v1: u64) -> R<Result<(Bytes, u64), Bytes>> {
    if f120(v0.clone(), v1, Str::lit("&#")) {
        let v2: bool = {
            let a1936 = f121(v0.clone(), {
                let a1934 = v1;
                let a1935 = 2u64;
                nat_add(a1934, a1935)?
            })?;
            let a1937 = 120u64;
            nat_eq(a1936, a1937)
        };
        let v3: u64 = if v2 {
            let a1938 = v1;
            let a1939 = 3u64;
            nat_add(a1938, a1939)?
        } else {
            let a1940 = v1;
            let a1941 = 2u64;
            nat_add(a1940, a1941)?
        };
        let v4: (u64, u64) = f390(v0.clone(), v2, v3)?;
        let a1943 = {
            let (h1942, _) = v4;
            h1942
        };
        let a1944 = v3;
        if if nat_eq(a1943, a1944) {
            true
        } else {
            let a1946 = f121(v0, {
                let (h1945, _) = v4;
                h1945
            })?;
            let a1947 = 59u64;
            !nat_eq(a1946, a1947)
        } {
            Ok(f282(f98(Str::lit("a character reference must be &#digits; or &#xhex; (XML 1.0 [66])"), v1)?))
        } else {
            if f385({
                let (_, h1948) = v4;
                h1948
            })? {
                Ok(Ok::<(Bytes, u64), Bytes>((f391({
                    let (_, h1949) = v4;
                    h1949
                })?, {
                    let a1951 = {
                        let (h1950, _) = v4;
                        h1950
                    };
                    let a1952 = 1u64;
                    nat_add(a1951, a1952)?
                })))
            } else {
                Ok(f282(f98(Str::lit("a character reference must name a Char (XML 1.0 [66] WFC: Legal Character)"), v1)?))
            }
        }
    } else {
        if f120(v0.clone(), v1, Str::lit("&lt;")) {
            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("<")), {
                let a1953 = v1;
                let a1954 = 4u64;
                nat_add(a1953, a1954)?
            })))
        } else {
            if f120(v0.clone(), v1, Str::lit("&gt;")) {
                Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit(">")), {
                    let a1955 = v1;
                    let a1956 = 4u64;
                    nat_add(a1955, a1956)?
                })))
            } else {
                if f120(v0.clone(), v1, Str::lit("&amp;")) {
                    Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("&")), {
                        let a1957 = v1;
                        let a1958 = 5u64;
                        nat_add(a1957, a1958)?
                    })))
                } else {
                    if f120(v0.clone(), v1, Str::lit("&apos;")) {
                        Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("'")), {
                            let a1959 = v1;
                            let a1960 = 6u64;
                            nat_add(a1959, a1960)?
                        })))
                    } else {
                        if f120(v0, v1, Str::lit("&quot;")) {
                            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("\"")), {
                                let a1961 = v1;
                                let a1962 = 6u64;
                                nat_add(a1961, a1962)?
                            })))
                        } else {
                            Ok(f282(f98(Str::lit("an entity reference names an entity that is not declared, and only the five predefined entities are (XML 1.0 section 4.1, WFC: Entity Declared)"), v1)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f284(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<u64> {
    let (h1963, _) = f149(Fn4::F392(v1, v2, v0.clone()), {
        let a1965 = {
            let a1964 = v0;
            length_bytes(a1964)
        };
        let a1966 = 1u64;
        nat_add(a1965, a1966)?
    }, v3)?;
    Ok(h1963)
}

pub fn f285(p1979: Adt17, v1: List<(Bytes, Bytes)>) -> R<Result<Option<Adt28>, Bytes>> {
    let m1978 = p1979;
    {
        let mut m1978 = m1978;
        loop {
            let step = {
                let v0 = m1978;
                let m1967 = v0;
                match m1967 {
                    Adt17::C0(_, _, _) => {
                        Err::<Adt17, R<Result<Option<Adt28>, Bytes>>>(Ok(Ok::<Option<Adt28>, Bytes>(None::<Adt28>)))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<Adt17, R<Result<Option<Adt28>, Bytes>>>(Ok(Ok::<Option<Adt28>, Bytes>(None::<Adt28>)))
                    }
                    Adt17::C2(_) => {
                        Err::<Adt17, R<Result<Option<Adt28>, Bytes>>>(Ok(Ok::<Option<Adt28>, Bytes>(None::<Adt28>)))
                    }
                    Adt17::C3(r1976, r1977) => {
                        let v2 = (*r1976).clone();
                        let v3 = (*r1977).clone();
                        let m1968 = v2;
                        match m1968 {
                            Adt17::C0(v4, r1974, r1975) => {
                                let v5 = (*r1974).clone();
                                let v6 = (*r1975).clone();
                                let m1969 = f124(v5.clone(), v1.clone());
                                match m1969 {
                                    Ok(v8) => {
                                        let m1970 = f125(v8.clone(), v4)?;
                                        match m1970 {
                                            Ok(v10) => {
                                                if f251({
                                                    let (_, h1971) = v10.clone();
                                                    h1971
                                                }, List::cons(f8(Str::lit("restriction")), List::cons(f8(Str::lit("list")), List::cons(f8(Str::lit("union")), List::<Bytes>::nil())))) {
                                                    if f46({
                                                        let (h1972, _) = v10;
                                                        h1972
                                                    }, f197()) {
                                                        Err::<Adt17, R<Result<Option<Adt28>, Bytes>>>(Ok(Ok::<Option<Adt28>, Bytes>(Some(Adt28::C0({
                                                            let (_, h1973) = v10;
                                                            h1973
                                                        }, v5, v6, v8)))))
                                                    } else {
                                                        Ok::<Adt17, R<Result<Option<Adt28>, Bytes>>>(v3)
                                                    }
                                                } else {
                                                    Ok::<Adt17, R<Result<Option<Adt28>, Bytes>>>(v3)
                                                }
                                            }
                                            Err(v9) => {
                                                Err::<Adt17, R<Result<Option<Adt28>, Bytes>>>(Ok(f393(v9)))
                                            }
                                        }
                                    }
                                    Err(v7) => {
                                        Err::<Adt17, R<Result<Option<Adt28>, Bytes>>>(Ok(f393(v7)))
                                    }
                                }
                            }
                            Adt17::C1(_, _, _) => {
                                Ok::<Adt17, R<Result<Option<Adt28>, Bytes>>>(v3)
                            }
                            Adt17::C2(_) => {
                                Ok::<Adt17, R<Result<Option<Adt28>, Bytes>>>(v3)
                            }
                            Adt17::C3(_, _) => {
                                Ok::<Adt17, R<Result<Option<Adt28>, Bytes>>>(v3)
                            }
                            Adt17::C4 => {
                                Ok::<Adt17, R<Result<Option<Adt28>, Bytes>>>(v3)
                            }
                        }
                    }
                    Adt17::C4 => {
                        Err::<Adt17, R<Result<Option<Adt28>, Bytes>>>(Ok(Ok::<Option<Adt28>, Bytes>(None::<Adt28>)))
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

pub fn f286(v0: Bytes) -> Result<Adt23, Bytes> {
    Err::<Adt23, Bytes>(v0)
}

pub fn f287(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit(" at simpleType ")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f288(v0: List<(Bytes, Bytes)>, v1: Bytes, v2: Bytes) -> R<Result<Adt23, Bytes>> {
    let m1980 = f125(v0, v2.clone())?;
    match m1980 {
        Ok(v4) => {
            if f46({
                let (h1981, _) = v4.clone();
                h1981
            }, f197()) {
                let m1983 = f394({
                    let (_, h1982) = v4;
                    h1982
                }, 16u64)?;
                match m1983 {
                    Ok(v6) => {
                        Ok(Ok::<Adt23, Bytes>(Adt23::C0(v6, List::<Bytes>::nil())))
                    }
                    Err(v5) => {
                        Ok(f286(v5))
                    }
                }
            } else {
                if f46({
                    let (h1984, _) = v4.clone();
                    h1984
                }, v1) {
                    let v7: Bytes = f200({
                        let (_, h1985) = v4;
                        h1985
                    })?;
                    Ok(Ok::<Adt23, Bytes>(Adt23::C0(v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                } else {
                    Ok(f286(f7(List::cons(f8(Str::lit("the type ")), List::cons(v2, List::cons(f8(Str::lit(" is in the namespace ")), List::cons({
                        let (h1986, _) = v4;
                        h1986
                    }, List::cons(f8(Str::lit(", which is neither XML Schema's nor this document's target namespace; imported schema documents are not read yet")), List::<Bytes>::nil()))))))))
                }
            }
        }
        Err(v3) => {
            Ok(f286(v3))
        }
    }
}

pub fn f289(v0: Adt17, v1: List<(Bytes, Bytes)>) -> R<Result<List<Adt2>, Bytes>> {
    let m1987 = v0;
    match m1987 {
        Adt17::C0(_, _, _) => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
        Adt17::C2(_) => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
        Adt17::C3(r1994, r1995) => {
            let v2 = (*r1994).clone();
            let v3 = (*r1995).clone();
            let m1988 = f289(v3, v1.clone())?;
            match m1988 {
                Ok(v5) => {
                    let m1989 = v2;
                    match m1989 {
                        Adt17::C0(v6, r1993, _) => {
                            let v7 = (*r1993).clone();
                            let m1990 = f124(v7.clone(), v1);
                            match m1990 {
                                Ok(v10) => {
                                    let m1991 = f125(v10, v6)?;
                                    match m1991 {
                                        Ok(v12) => {
                                            if f126(v12, Str::lit("enumeration")) {
                                                let m1992 = f127(v7, f8(Str::lit("value")));
                                                match m1992 {
                                                    None => {
                                                        Ok(f395(f8(Str::lit("an enumeration facet must have a value attribute (XSD 1.1 Part 2, section 4.3.5.2)"))))
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
                                            Ok(f395(v11))
                                        }
                                    }
                                }
                                Err(v9) => {
                                    Ok(f395(v9))
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
                    Ok(f395(v4))
                }
            }
        }
        Adt17::C4 => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
    }
}

pub fn f290(v0: List<Adt2>, v1: List<Bytes>) -> R<List<Adt2>> {
    let m1996 = v0;
    match m1996.uncons() {
        None => {
            Ok(List::<Adt2>::nil())
        }
        Some((v2, v3)) => {
            let v4: Bytes = f384(v2.clone())?;
            if f167(v4.clone(), v1.clone()) {
                f290(v3, v1)
            } else {
                Ok(List::cons(v2, f290(v3, List::cons(v4, v1))?))
            }
        }
    }
}

pub fn f291(v0: List<Adt2>) -> List<Bytes> {
    let m1997 = v0;
    match m1997.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1998 = f337(Some(v1));
            match m1998 {
                None => {
                    f291(v2)
                }
                Some(v3) => {
                    List::cons(v3, f291(v2))
                }
            }
        }
    }
}

pub fn f292(v0: Bytes, p2019: u64, p2020: u64, p2021: bool, p2022: Bytes) -> R<Bytes> {
    let m2018 = (p2019, (p2020, (p2021, p2022)));
    {
        let mut m2018 = m2018;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m2018;
                let m1999 = v2;
                if m1999 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1999 - 1;
                    let m2002 = {
                        let a2000 = v0.clone();
                        let a2001 = v1;
                        index_bytes(a2000, a2001)
                    };
                    match m2002 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f53(v6)?;
                            if f396(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a2003 = v1;
                                    let a2004 = 1u64;
                                    nat_add(a2003, a2004)?
                                }, (v5, (true, v4))))
                            } else {
                                if f87(f397(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a2005 = v1;
                                        let a2006 = 1u64;
                                        nat_add(a2005, a2006)?
                                    }, (v5, (false, {
                                        let a2010 = v4;
                                        let a2011 = f43({
                                            let a2007 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a2007)
                                        }, {
                                            let a2008 = v7;
                                            let a2009 = 97u64;
                                            nat_sub(a2008, a2009)
                                        }, 1u64);
                                        append_bytes(a2010, a2011)
                                    }))))
                                } else {
                                    if f141(f397(v7), f141(f398(v7), f399(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a2012 = v1;
                                            let a2013 = 1u64;
                                            nat_add(a2012, a2013)?
                                        }, (v5, (false, {
                                            let a2014 = v4;
                                            let a2015 = f43(v0.clone(), v1, 1u64);
                                            append_bytes(a2014, a2015)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a2016 = v1;
                                            let a2017 = 1u64;
                                            nat_add(a2016, a2017)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2018 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f293(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f193(v0.clone(), v1) {
        let m2023 = f41(v0, 34u8)?;
        match m2023.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m2024 = v3;
                match m2024.uncons() {
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

pub fn f294(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f400(v0, v1)
}

pub fn f295(v0: List<Adt19>) -> R<Result<List<Adt25>, Bytes>> {
    let m2025 = v0;
    match m2025.uncons() {
        None => {
            Ok(Ok::<List<Adt25>, Bytes>(List::<Adt25>::nil()))
        }
        Some((v1, v2)) => {
            let m2028 = f213(match v1.clone() {
                Adt19::C0(_, _, _, _, h2026, _) => {
                    h2026
                }
            }, match v1.clone() {
                Adt19::C0(_, _, _, h2027, _, _) => {
                    h2027
                }
            })?;
            match m2028 {
                Ok(v4) => {
                    let m2029 = f295(v2)?;
                    match m2029 {
                        Ok(v6) => {
                            Ok(Ok::<List<Adt25>, Bytes>(f401(f402(v4, match v1 {
                                Adt19::C0(_, _, h2030, _, _, _) => {
                                    h2030
                                }
                            }), v6)))
                        }
                        Err(v5) => {
                            Ok(Err::<List<Adt25>, Bytes>(v5))
                        }
                    }
                }
                Err(v3) => {
                    Ok(Err::<List<Adt25>, Bytes>(v3))
                }
            }
        }
    }
}

pub fn f296(v0: List<Adt19>) -> List<Bytes> {
    let m2031 = v0;
    match m2031.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt19::C0(_, _, h2032, _, _, _) => {
                    h2032
                }
            }, f296(v2))
        }
    }
}

pub fn f297(v0: u64, v1: Adt24, v2: List<Adt25>) -> R<Result<List<Adt27>, Bytes>> {
    let m2033 = v2;
    match m2033.uncons() {
        None => {
            Ok(Ok::<List<Adt27>, Bytes>(List::<Adt27>::nil()))
        }
        Some((v3, v4)) => {
            let m2034 = f403(v0, v1.clone(), v3)?;
            match m2034 {
                Ok(v6) => {
                    let m2035 = f297(v0, v1, v4)?;
                    match m2035 {
                        Ok(v8) => {
                            Ok(Ok::<List<Adt27>, Bytes>(f404(v6, v8)))
                        }
                        Err(v7) => {
                            Ok(Err::<List<Adt27>, Bytes>(v7))
                        }
                    }
                }
                Err(v5) => {
                    Ok(Err::<List<Adt27>, Bytes>(v5))
                }
            }
        }
    }
}

pub fn f298(v0: List<Adt19>) -> R<u64> {
    let m2036 = v0;
    match m2036.uncons() {
        None => {
            Ok(64u64)
        }
        Some((v1, v2)) => {
            let a2038 = match v1 {
                Adt19::C0(_, _, _, _, _, h2037) => {
                    h2037
                }
            };
            let a2039 = f298(v2)?;
            nat_add(a2038, a2039)
        }
    }
}

pub fn f299(p2047: List<Adt27>, p2048: List<Adt27>) -> Result<List<Adt27>, Bytes> {
    let m2046 = (p2047, p2048);
    {
        let mut m2046 = m2046;
        loop {
            let step = {
                let (v0, v1) = m2046;
                let m2040 = v0;
                match m2040.uncons() {
                    None => {
                        Err::<(List<Adt27>, List<Adt27>), Result<List<Adt27>, Bytes>>(Ok::<List<Adt27>, Bytes>(f405(v1, List::<Adt27>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m2042 = f406(v1.clone(), match v2.clone() {
                            Adt27::C0(h2041, _, _, _) => {
                                h2041
                            }
                        });
                        match m2042 {
                            None => {
                                Ok::<(List<Adt27>, List<Adt27>), Result<List<Adt27>, Bytes>>((v3, List::cons(v2, v1)))
                            }
                            Some(v4) => {
                                if f46(match v4 {
                                    Adt27::C0(_, _, h2043, _) => {
                                        h2043
                                    }
                                }, match v2.clone() {
                                    Adt27::C0(_, _, h2044, _) => {
                                        h2044
                                    }
                                }) {
                                    Ok::<(List<Adt27>, List<Adt27>), Result<List<Adt27>, Bytes>>((v3, v1))
                                } else {
                                    Err::<(List<Adt27>, List<Adt27>), Result<List<Adt27>, Bytes>>(Err::<List<Adt27>, Bytes>(f209(f8(Str::lit("two components derive the type name ")), match v2 {
                                        Adt27::C0(h2045, _, _, _) => {
                                            h2045
                                        }
                                    }, f8(Str::lit(", so their LexLean declarations would collide")))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2046 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f300(v0: List<Adt27>) -> List<Adt27> {
    if if f167(f407(), f408(v0.clone())) {
        let m2049 = f406(v0.clone(), f407());
        match m2049 {
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
        List::cons(Adt27::C0(f407(), List::<Bytes>::nil(), f269(f407(), f409()), f409()), v0)
    } else {
        v0
    }
}

pub fn f301(v0: List<Adt27>) -> List<Bytes> {
    let m2050 = v0;
    match m2050.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt27::C0(h2051, _, _, _) => {
                    h2051
                }
            }, f301(v2))
        }
    }
}

pub fn f302(p2057: List<Adt27>, v1: List<Bytes>) -> Option<(Bytes, Bytes)> {
    let m2056 = p2057;
    {
        let mut m2056 = m2056;
        loop {
            let step = {
                let v0 = m2056;
                let m2052 = v0;
                match m2052.uncons() {
                    None => {
                        Err::<List<Adt27>, Option<(Bytes, Bytes)>>(None::<(Bytes, Bytes)>)
                    }
                    Some((v2, v3)) => {
                        let m2054 = f201(match v2.clone() {
                            Adt27::C0(_, h2053, _, _) => {
                                h2053
                            }
                        }, v1.clone());
                        match m2054 {
                            None => {
                                Ok::<List<Adt27>, Option<(Bytes, Bytes)>>(v3)
                            }
                            Some(v4) => {
                                Err::<List<Adt27>, Option<(Bytes, Bytes)>>(Some((match v2 {
                                    Adt27::C0(h2055, _, _, _) => {
                                        h2055
                                    }
                                }, v4)))
                            }
                        }
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

pub fn f303(v0: List<Adt27>, v1: List<Bytes>, v2: u64) -> List<(Bytes, List<Bytes>)> {
    let m2058 = v1;
    match m2058.uncons() {
        None => {
            List::<(Bytes, List<Bytes>)>::nil()
        }
        Some((v3, v4)) => {
            List::cons((v3.clone(), f410(v2, v0.clone(), f411(v0.clone(), v3), List::<Bytes>::nil())), f303(v0, v4, v2))
        }
    }
}

pub fn f304(v0: List<Adt27>) -> R<u64> {
    let m2059 = v0;
    match m2059.uncons() {
        None => {
            Ok(1u64)
        }
        Some((v1, v2)) => {
            let a2064 = {
                let a2062 = {
                    let a2061 = match v1 {
                        Adt27::C0(_, h2060, _, _) => {
                            h2060
                        }
                    };
                    length_list(a2061)
                };
                let a2063 = 1u64;
                nat_add(a2062, a2063)?
            };
            let a2065 = f304(v2)?;
            nat_add(a2064, a2065)
        }
    }
}

pub fn f305(p2075: u64, v1: List<Adt27>, p2076: List<(List<Bytes>, List<Bytes>)>, p2077: List<Bytes>, p2078: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m2074 = (p2075, (p2076, (p2077, p2078)));
    {
        let mut m2074 = m2074;
        loop {
            let step = {
                let (v0, (v2, (v3, v4))) = m2074;
                let m2066 = v0;
                if m2066 == 0 {
                    let m2067 = v2;
                    match m2067.uncons() {
                        None => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v4))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(f8(Str::lit("the declarations could not be ordered so that each follows the types it uses"))))
                        }
                    }
                } else {
                    let v7 = m2066 - 1;
                    let m2068 = v2.clone();
                    match m2068.uncons() {
                        None => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v4))
                        }
                        Some((_, _)) => {
                            let m2069 = f412(v2, v3.clone());
                            match m2069 {
                                None => {
                                    Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(f8(Str::lit("the declarations could not be ordered so that each follows the types it uses"))))
                                }
                                Some(v10) => {
                                    let m2071 = f413(v1.clone(), {
                                        let (h2070, _) = v10.clone();
                                        h2070
                                    });
                                    match m2071 {
                                        Ok(v12) => {
                                            Ok::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>((v7, ({
                                                let (_, h2072) = v10;
                                                h2072
                                            }, (f414({
                                                let (h2073, _) = v10;
                                                h2073
                                            }, v3), f414(v4, v12)))))
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
                Ok(next) => { m2074 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f306(v0: List<Adt27>, v1: List<List<Bytes>>) -> List<(List<Bytes>, List<Bytes>)> {
    let m2079 = v1;
    match m2079.uncons() {
        None => {
            List::<(List<Bytes>, List<Bytes>)>::nil()
        }
        Some((v2, v3)) => {
            List::cons((v2.clone(), f415(v0.clone(), v2.clone(), v2)), f306(v0, v3))
        }
    }
}

pub fn f307(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> List<List<Bytes>> {
    let m2080 = v2;
    match m2080.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((v4, v5)) => {
            if f167(v4.clone(), v3.clone()) {
                f307(v0, v1, v5, v3)
            } else {
                let v6: List<Bytes> = f416(v0.clone(), v1.clone(), v4);
                List::cons(v6.clone(), f307(v0, v1, v5, f414(v6, v3)))
            }
        }
    }
}

pub fn f308(v0: bool) -> bool {
    !v0
}

pub fn f309(v0: Adt26, v1: Str) -> bool {
    f46(match v0 {
        Adt26::C0(_, h2081, _, _, _) => {
            h2081
        }
    }, f8(v1))
}

pub fn f310(v0: Adt26, v1: Str) -> Option<Bytes> {
    f127(match v0 {
        Adt26::C0(_, _, h2082, _, _) => {
            h2082
        }
    }, f8(v1))
}

pub fn f311(v0: Option<Bytes>, v1: Bytes) -> R<Result<Bytes, Bytes>> {
    let m2083 = f180(v1.clone(), f8(Str::lit("://")), 0u64)?;
    match m2083 {
        None => {
            let v3: Bytes = f209(f8(Str::lit("the relative schema location ")), v1.clone(), f8(Str::lit(" is not resolved, because only a bare file name relative to a bound document's URL is merged (RFC 3986, section 5.2.3)")));
            if if f193(v1.clone(), f8(Str::lit("/"))) {
                true
            } else {
                let m2084 = f180(v1.clone(), f8(Str::lit("..")), 0u64)?;
                match m2084 {
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
                let m2085 = v0;
                match m2085 {
                    None => {
                        Ok(Err::<Bytes, Bytes>(v3))
                    }
                    Some(v5) => {
                        let m2087 = f417(v5.clone(), 0u64, {
                            let a2086 = v5.clone();
                            length_bytes(a2086)
                        }, None::<u64>)?;
                        match m2087 {
                            None => {
                                Ok(Err::<Bytes, Bytes>(v3))
                            }
                            Some(v6) => {
                                Ok(Ok::<Bytes, Bytes>({
                                    let a2090 = f43(v5, 0u64, {
                                        let a2088 = v6;
                                        let a2089 = 1u64;
                                        nat_add(a2088, a2089)?
                                    });
                                    let a2091 = v1;
                                    append_bytes(a2090, a2091)
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

pub fn f312(p2096: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m2095 = p2096;
    {
        let mut m2095 = m2095;
        loop {
            let step = {
                let v0 = m2095;
                let m2092 = v0;
                match m2092.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f46({
                            let (h2093, _) = v2;
                            h2093
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h2094) = v2;
                                h2094
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2095 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f313(v0: Adt26, v1: Str, v2: Str) -> Bytes {
    let m2097 = f310(v0, v1);
    match m2097 {
        None => {
            f8(v2)
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f314(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a2098 = 0u64;
        let a2099 = v1;
        nat_lt(a2098, a2099)
    } else {
        false
    } {
        let a2100 = Bytes::lit(&[0x2d]);
        let a2101 = f65(v1)?;
        Ok(append_bytes(a2100, a2101))
    } else {
        f65(v1)
    }
}

pub fn f315(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f65(v1)?;
    let v4: u64 = {
        let a2102 = v3.clone();
        length_bytes(a2102)
    };
    let v5: Bytes = {
        let a2103 = v2;
        let a2104 = v4;
        if nat_lt(a2103, a2104) {
            f7(List::cons(f43(v3.clone(), 0u64, {
                let a2105 = v4;
                let a2106 = v2;
                nat_sub(a2105, a2106)
            }), List::cons(f8(Str::lit(".")), List::cons(f43(v3, {
                let a2107 = v4;
                let a2108 = v2;
                nat_sub(a2107, a2108)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f226({
                let a2109 = v2;
                let a2110 = v4;
                nat_sub(a2109, a2110)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a2111 = f8(Str::lit("-"));
        let a2112 = v5;
        Ok(append_bytes(a2111, a2112))
    } else {
        Ok(v5)
    }
}

pub fn f316(v0: Bytes) -> R<Bytes> {
    let a2117 = f138(v0.clone(), 0u64, {
        let a2114 = {
            let a2113 = v0.clone();
            length_bytes(a2113)
        };
        let a2115 = 1u64;
        nat_add(a2114, a2115)?
    })?;
    let a2118 = {
        let a2116 = v0.clone();
        length_bytes(a2116)
    };
    if nat_eq(a2117, a2118) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        let a2124 = {
            let a2122 = Bytes::lit(&[0x22]);
            let a2123 = f418(v0.clone(), 0u64, {
                let a2120 = {
                    let a2119 = v0;
                    length_bytes(a2119)
                };
                let a2121 = 1u64;
                nat_add(a2120, a2121)?
            }, Bytes::lit(&[]))?;
            append_bytes(a2122, a2123)
        };
        let a2125 = Bytes::lit(&[0x22]);
        Ok(append_bytes(a2124, a2125))
    }
}

pub fn f317(v0: List<Bytes>) -> Bytes {
    let a2128 = {
        let a2126 = Bytes::lit(&[0x5b]);
        let a2127 = f44(v0, Bytes::lit(&[0x2c]));
        append_bytes(a2126, a2127)
    };
    let a2129 = Bytes::lit(&[0x5d]);
    append_bytes(a2128, a2129)
}

pub fn f318(v0: Adt2) -> R<List<Bytes>> {
    let m2130 = v0;
    match m2130 {
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
        Adt2::C9(r2131, r2132) => {
            let v12 = (*r2131).clone();
            let v13 = (*r2132).clone();
            Ok(List::cons(f218(v12)?, f318(v13)?))
        }
        Adt2::C10(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f319(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f419(Fn6::F420, List::<(Bytes, Bytes)>::nil(), f421(v0)?);
    let v2: List<Bytes> = f422(Fn7::F423, List::<Bytes>::nil(), v1)?;
    let a2135 = {
        let a2133 = Bytes::lit(&[0x7b]);
        let a2134 = f44(v2, Bytes::lit(&[0x2c]));
        append_bytes(a2133, a2134)
    };
    let a2136 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a2135, a2136))
}

pub fn f320(v0: Adt2) -> R<List<(Bytes, Bytes)>> {
    let m2137 = v0;
    match m2137 {
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
        Adt2::C10(v14, r2138, r2139) => {
            let v15 = (*r2138).clone();
            let v16 = (*r2139).clone();
            Ok(List::cons((v14, f218(v15)?), f320(v16)?))
        }
    }
}

pub fn f321(v0: Bytes, v1: u64) -> R<Bytes> {
    let a2142 = f43(v0.clone(), {
        let a2140 = v1;
        let a2141 = 2u64;
        nat_add(a2140, a2141)?
    }, 2u64);
    let a2143 = Bytes::lit(&[0x30, 0x30]);
    if equal(a2142, a2143) {
        let m2146 = f424(f43(v0, {
            let a2144 = v1;
            let a2145 = 4u64;
            nat_add(a2144, a2145)?
        }, 2u64))?;
        match m2146 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f85(v2.clone(), 0u64)?;
                let a2147 = v3;
                let a2148 = 32u64;
                if nat_lt(a2147, a2148) {
                    if f425(v3) {
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

pub fn f322(v0: u64) -> Bytes {
    let a2149 = v0;
    let a2150 = 34u64;
    if nat_eq(a2149, a2150) {
        Bytes::lit(&[0x22])
    } else {
        let a2151 = v0;
        let a2152 = 92u64;
        if nat_eq(a2151, a2152) {
            Bytes::lit(&[0x5c])
        } else {
            let a2153 = v0;
            let a2154 = 98u64;
            if nat_eq(a2153, a2154) {
                Bytes::lit(&[0x08])
            } else {
                let a2155 = v0;
                let a2156 = 102u64;
                if nat_eq(a2155, a2156) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a2157 = v0;
                    let a2158 = 110u64;
                    if nat_eq(a2157, a2158) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a2159 = v0;
                        let a2160 = 114u64;
                        if nat_eq(a2159, a2160) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a2161 = v0;
                            let a2162 = 116u64;
                            if nat_eq(a2161, a2162) {
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

pub fn f323(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f85(v0, v1)?;
    let a2163 = 47u64;
    let a2164 = v2;
    if if nat_lt(a2163, a2164) {
        let a2165 = v2;
        let a2166 = 58u64;
        nat_lt(a2165, a2166)
    } else {
        false
    } {
        Ok(Some({
            let a2167 = v1;
            let a2168 = 1u64;
            nat_add(a2167, a2168)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f324(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2169 = f85(v0, v1)?;
    let a2170 = 48u64;
    if nat_eq(a2169, a2170) {
        Ok(Some({
            let a2171 = v1;
            let a2172 = 1u64;
            nat_add(a2171, a2172)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f325(v0: Bytes) -> R<u64> {
    let (h2173, _) = f149(Fn4::F426(v0.clone()), {
        let a2174 = v0;
        length_bytes(a2174)
    }, 0u64)?;
    Ok(h2173)
}

pub fn f326(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m2178 = {
        let a2176 = v0;
        let a2177 = {
            let (h2175, _) = v1;
            h2175
        };
        index_bytes(a2176, a2177)
    };
    match m2178 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2180 = {
                    let (h2179, _) = v1;
                    h2179
                };
                let a2181 = 1u64;
                nat_add(a2180, a2181)?
            }, {
                let a2185 = {
                    let a2183 = {
                        let (_, h2182) = v1;
                        h2182
                    };
                    let a2184 = 10u64;
                    nat_mul(a2183, a2184)?
                };
                let a2186 = f427(v2)?;
                nat_add(a2185, a2186)?
            })))
        }
    }
}

pub fn f327(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f428(v0, v1)
}

pub fn f328(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m2189 = {
        let a2187 = v0.clone();
        let a2188 = v1;
        index_bytes(a2187, a2188)
    };
    match m2189 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f53(v2)?;
            let a2190 = v3;
            let a2191 = 128u64;
            if nat_lt(a2190, a2191) {
                Ok(Some((v3, 1u64)))
            } else {
                if f87({
                    let a2192 = 194u64;
                    let a2193 = v3;
                    nat_le(a2192, a2193)
                }, {
                    let a2194 = v3;
                    let a2195 = 223u64;
                    nat_le(a2194, a2195)
                }) {
                    let m2198 = f429(v0, {
                        let a2196 = v1;
                        let a2197 = 1u64;
                        nat_add(a2196, a2197)?
                    })?;
                    match m2198 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f430({
                                let a2203 = {
                                    let a2201 = {
                                        let a2199 = v3;
                                        let a2200 = 192u64;
                                        nat_sub(a2199, a2200)
                                    };
                                    let a2202 = 64u64;
                                    nat_mul(a2201, a2202)?
                                };
                                let a2204 = v4;
                                nat_add(a2203, a2204)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f87({
                        let a2205 = 224u64;
                        let a2206 = v3;
                        nat_le(a2205, a2206)
                    }, {
                        let a2207 = v3;
                        let a2208 = 239u64;
                        nat_le(a2207, a2208)
                    }) {
                        let m2211 = f429(v0.clone(), {
                            let a2209 = v1;
                            let a2210 = 1u64;
                            nat_add(a2209, a2210)?
                        })?;
                        match m2211 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m2214 = f429(v0, {
                                    let a2212 = v1;
                                    let a2213 = 2u64;
                                    nat_add(a2212, a2213)?
                                })?;
                                match m2214 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f430({
                                            let a2223 = {
                                                let a2217 = {
                                                    let a2215 = v3;
                                                    let a2216 = 224u64;
                                                    nat_sub(a2215, a2216)
                                                };
                                                let a2218 = 4096u64;
                                                nat_mul(a2217, a2218)?
                                            };
                                            let a2224 = {
                                                let a2221 = {
                                                    let a2219 = v5;
                                                    let a2220 = 64u64;
                                                    nat_mul(a2219, a2220)?
                                                };
                                                let a2222 = v6;
                                                nat_add(a2221, a2222)?
                                            };
                                            nat_add(a2223, a2224)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f87({
                            let a2225 = 240u64;
                            let a2226 = v3;
                            nat_le(a2225, a2226)
                        }, {
                            let a2227 = v3;
                            let a2228 = 244u64;
                            nat_le(a2227, a2228)
                        }) {
                            let m2231 = f429(v0.clone(), {
                                let a2229 = v1;
                                let a2230 = 1u64;
                                nat_add(a2229, a2230)?
                            })?;
                            match m2231 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m2234 = f429(v0.clone(), {
                                        let a2232 = v1;
                                        let a2233 = 2u64;
                                        nat_add(a2232, a2233)?
                                    })?;
                                    match m2234 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m2237 = f429(v0, {
                                                let a2235 = v1;
                                                let a2236 = 3u64;
                                                nat_add(a2235, a2236)?
                                            })?;
                                            match m2237 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f430({
                                                        let a2250 = {
                                                            let a2240 = {
                                                                let a2238 = v3;
                                                                let a2239 = 240u64;
                                                                nat_sub(a2238, a2239)
                                                            };
                                                            let a2241 = 262144u64;
                                                            nat_mul(a2240, a2241)?
                                                        };
                                                        let a2251 = {
                                                            let a2248 = {
                                                                let a2242 = v7;
                                                                let a2243 = 4096u64;
                                                                nat_mul(a2242, a2243)?
                                                            };
                                                            let a2249 = {
                                                                let a2246 = {
                                                                    let a2244 = v8;
                                                                    let a2245 = 64u64;
                                                                    nat_mul(a2244, a2245)?
                                                                };
                                                                let a2247 = v9;
                                                                nat_add(a2246, a2247)?
                                                            };
                                                            nat_add(a2248, a2249)?
                                                        };
                                                        nat_add(a2250, a2251)?
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

pub fn f329(v0: u64) -> bool {
    let a2252 = v0;
    let a2253 = 9u64;
    if if if nat_eq(a2252, a2253) {
        true
    } else {
        let a2254 = v0;
        let a2255 = 10u64;
        nat_eq(a2254, a2255)
    } {
        true
    } else {
        let a2256 = v0;
        let a2257 = 13u64;
        if nat_eq(a2256, a2257) {
            true
        } else {
            let a2258 = 32u64;
            let a2259 = v0;
            if nat_le(a2258, a2259) {
                let a2260 = v0;
                let a2261 = 126u64;
                nat_le(a2260, a2261)
            } else {
                false
            }
        }
    } {
        true
    } else {
        let a2262 = v0;
        let a2263 = 133u64;
        if if nat_eq(a2262, a2263) {
            true
        } else {
            let a2264 = 160u64;
            let a2265 = v0;
            if nat_le(a2264, a2265) {
                let a2266 = v0;
                let a2267 = 55295u64;
                nat_le(a2266, a2267)
            } else {
                false
            }
        } {
            true
        } else {
            let a2268 = 57344u64;
            let a2269 = v0;
            if if nat_le(a2268, a2269) {
                let a2270 = v0;
                let a2271 = 65533u64;
                nat_le(a2270, a2271)
            } else {
                false
            } {
                true
            } else {
                let a2272 = 65536u64;
                let a2273 = v0;
                if nat_le(a2272, a2273) {
                    let a2274 = v0;
                    let a2275 = 1114111u64;
                    nat_le(a2274, a2275)
                } else {
                    false
                }
            }
        }
    }
}

pub fn f330(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f233(v0.clone(), v1)?;
    if f231(f154(v0.clone(), v2)?) {
        Ok(true)
    } else {
        let a2276 = f154(v0, v2)?;
        let a2277 = 35u64;
        Ok(nat_eq(a2276, a2277))
    }
}

pub fn f331(v0: Bytes, v1: u64) -> R<u64> {
    let (h2278, _) = f149(Fn4::F431(v0), {
        let a2279 = v1;
        let a2280 = 1u64;
        nat_add(a2279, a2280)?
    }, v1)?;
    Ok(h2278)
}

pub fn f332(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f432(v0, v1)
}

pub fn f333(v0: Bytes, v1: u64) -> R<u64> {
    let (h2281, _) = f149(Fn4::F433(v0.clone()), {
        let a2283 = {
            let a2282 = v0;
            length_bytes(a2282)
        };
        let a2284 = 1u64;
        nat_add(a2283, a2284)?
    }, v1)?;
    Ok(h2281)
}

pub fn f334(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f434(v0, v1)
}

pub fn f335(v0: u64, v1: bool) -> bool {
    f156(if if f230(v0) {
        true
    } else {
        f231(v0)
    } {
        true
    } else {
        if v1 {
            f435(v0)
        } else {
            false
        }
    })
}

pub fn f336(v0: Bytes, v1: u64, v2: u64) -> R<Adt21> {
    let v3: bool = {
        let a2285 = f154(v0.clone(), v1)?;
        let a2286 = 62u64;
        nat_eq(a2285, a2286)
    };
    let v4: u64 = f154(v0.clone(), {
        let a2287 = v1;
        let a2288 = 1u64;
        nat_add(a2287, a2288)?
    })?;
    let v5: u64 = f154(v0.clone(), {
        let a2289 = v1;
        let a2290 = 2u64;
        nat_add(a2289, a2290)?
    })?;
    let v6: u64 = f436(v4);
    let v7: u64 = f437(v4);
    let v8: bool = {
        let a2291 = 0u64;
        let a2292 = v6;
        if nat_lt(a2291, a2292) {
            true
        } else {
            let a2293 = 0u64;
            let a2294 = v7;
            nat_lt(a2293, a2294)
        }
    };
    let v9: u64 = {
        let a2295 = 0u64;
        let a2296 = v7;
        if nat_lt(a2295, a2296) {
            f436(v5)
        } else {
            0u64
        }
    };
    let v10: u64 = {
        let a2297 = 0u64;
        let a2298 = v6;
        if nat_lt(a2297, a2298) {
            f437(v5)
        } else {
            0u64
        }
    };
    let v11: u64 = {
        let a2299 = v6;
        let a2300 = v9;
        nat_add(a2299, a2300)?
    };
    let v12: u64 = {
        let a2301 = v7;
        let a2302 = v10;
        nat_add(a2301, a2302)?
    };
    let v13: u64 = {
        let a2311 = {
            let a2303 = v1;
            let a2304 = 1u64;
            nat_add(a2303, a2304)?
        };
        let a2312 = {
            let a2309 = if v8 {
                1u64
            } else {
                0u64
            };
            let a2310 = {
                let a2305 = 0u64;
                let a2306 = v9;
                if if nat_lt(a2305, a2306) {
                    true
                } else {
                    let a2307 = 0u64;
                    let a2308 = v10;
                    nat_lt(a2307, a2308)
                } {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a2309, a2310)?
        };
        nat_add(a2311, a2312)?
    };
    let m2313 = f153(v0.clone(), v13)?;
    match m2313 {
        Ok(v15) => {
            let a2314 = 0u64;
            let a2315 = v12;
            if nat_lt(a2314, a2315) {
                f438(v0, v15, {
                    let a2318 = {
                        let a2316 = v2;
                        let a2317 = v12;
                        nat_add(a2316, a2317)?
                    };
                    let a2319 = 1u64;
                    nat_sub(a2318, a2319)
                }, v3, v11)
            } else {
                let v16: (u64, u64) = {
                    let (h2320, _) = f223(Fn5::F439(v0.clone()), {
                        let a2322 = {
                            let a2321 = v0.clone();
                            length_bytes(a2321)
                        };
                        let a2323 = 1u64;
                        nat_add(a2322, a2323)?
                    }, (v15, 0u64))?;
                    h2320
                };
                let v17: u64 = f235(v0.clone(), {
                    let (h2324, _) = v16;
                    h2324
                })?;
                let a2325 = f154(v0.clone(), v17)?;
                let a2326 = 256u64;
                if nat_eq(a2325, a2326) {
                    Ok(Adt21::C0(Adt2::C5({
                        let a2327 = v11;
                        let a2328 = 2u64;
                        if nat_eq(a2327, a2328) {
                            f440(f441(v0.clone(), v15)?)
                        } else {
                            Bytes::lit(&[])
                        }
                    }), {
                        let a2329 = v0;
                        length_bytes(a2329)
                    }))
                } else {
                    let v18: u64 = f442({
                        let a2331 = v17;
                        let a2332 = {
                            let (h2330, _) = v16;
                            h2330
                        };
                        nat_sub(a2331, a2332)
                    }, v2);
                    let a2334 = v18;
                    let a2335 = {
                        let (_, h2333) = v16;
                        h2333
                    };
                    if nat_lt(a2334, a2335) {
                        Ok(Adt21::C1(f98(Str::lit("a leading empty line of a block scalar holds more spaces than its first content line (YAML 1.2.2 section 8.1.1.1)"), v15)?))
                    } else {
                        f438(v0, v15, v18, v3, v11)
                    }
                }
            }
        }
        Err(v14) => {
            Ok(Adt21::C1(v14))
        }
    }
}

pub fn f337(v0: Option<Adt2>) -> Option<Bytes> {
    let m2336 = v0;
    match m2336 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m2337 = v1;
            match m2337 {
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

pub fn f338(v0: Adt20, v1: Bytes, v2: bool, v3: u64, v4: u64) -> R<Adt21> {
    let m2338 = v0;
    match m2338 {
        Adt20::C0(v7) => {
            let a2339 = v7;
            let a2340 = 1u64;
            if if if nat_eq(a2339, a2340) {
                true
            } else {
                let a2341 = v7;
                let a2342 = 8u64;
                nat_eq(a2341, a2342)
            } {
                true
            } else {
                let a2343 = v7;
                let a2344 = 0u64;
                if nat_eq(a2343, a2344) {
                    v2
                } else {
                    false
                }
            } {
                Ok(Adt21::C0(Adt2::C5(v1), v4))
            } else {
                let a2345 = v7;
                let a2346 = 0u64;
                if nat_eq(a2345, a2346) {
                    f443(v1, v3, v4)
                } else {
                    let a2347 = v7;
                    let a2348 = 5u64;
                    if nat_eq(a2347, a2348) {
                        if f444(v1) {
                            Ok(Adt21::C0(Adt2::C0, v4))
                        } else {
                            Ok(Adt21::C1(f346(v3)?))
                        }
                    } else {
                        let a2349 = v7;
                        let a2350 = 2u64;
                        if if nat_eq(a2349, a2350) {
                            f445(v1.clone())?
                        } else {
                            let a2351 = v7;
                            let a2352 = 3u64;
                            if nat_eq(a2351, a2352) {
                                let m2353 = f446(v1.clone())?;
                                match m2353 {
                                    None => {
                                        f447(v1.clone())
                                    }
                                    Some(_) => {
                                        true
                                    }
                                }
                            } else {
                                let a2354 = v7;
                                let a2355 = 4u64;
                                if nat_eq(a2354, a2355) {
                                    if f448(v1.clone()) {
                                        true
                                    } else {
                                        f449(v1.clone())
                                    }
                                } else {
                                    false
                                }
                            }
                        } {
                            f443(v1, v3, v4)
                        } else {
                            Ok(Adt21::C1(f346(v3)?))
                        }
                    }
                }
            }
        }
        Adt20::C1(v6) => {
            Ok(Adt21::C0(Adt2::C1(v6, std::rc::Rc::new(Adt2::C5(v1))), v4))
        }
        Adt20::C2(v5) => {
            Ok(Adt21::C1(v5))
        }
    }
}

pub fn f339(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Adt29> {
    let a2356 = f154(v0.clone(), v1)?;
    let a2357 = 34u64;
    if nat_eq(a2356, a2357) {
        f450(v0.clone(), {
            let a2358 = v1;
            let a2359 = 1u64;
            nat_add(a2358, a2359)?
        }, v2, v3, {
            let a2361 = {
                let a2360 = v0;
                length_bytes(a2360)
            };
            let a2362 = 2u64;
            nat_add(a2361, a2362)?
        }, Bytes::lit(&[]))
    } else {
        f451(v0.clone(), {
            let a2363 = v1;
            let a2364 = 1u64;
            nat_add(a2363, a2364)?
        }, v2, v3, {
            let a2366 = {
                let a2365 = v0;
                length_bytes(a2365)
            };
            let a2367 = 2u64;
            nat_add(a2366, a2367)?
        }, Bytes::lit(&[]))
    }
}

pub fn f340(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: List<Bytes>, v6: Adt20) -> R<Adt21> {
    let m2368 = v3;
    if m2368 == 0 {
        Ok(Adt21::C1(f98(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v7 = m2368 - 1;
        let a2369 = v4;
        let a2370 = 1u64;
        if nat_eq(a2369, a2370) {
            let m2371 = f340(v0.clone(), v1, v2, v7, 0u64, List::<Bytes>::nil(), Adt20::C0(0u64))?;
            match m2371 {
                Adt21::C0(v9, v10) => {
                    let m2372 = f452(v0.clone(), v10, v2)?;
                    match m2372 {
                        Ok(v12) => {
                            let v13: u64 = f154(v0.clone(), v12)?;
                            let a2373 = v13;
                            let a2374 = 58u64;
                            if nat_eq(a2373, a2374) {
                                Ok(Adt21::C1(f98(Str::lit("a single-pair mapping inside a flow sequence is not read (YAML 1.2.2 [150])"), v12)?))
                            } else {
                                let a2375 = v13;
                                let a2376 = 93u64;
                                if nat_eq(a2375, a2376) {
                                    Ok(Adt21::C0(Adt2::C9(std::rc::Rc::new(v9), std::rc::Rc::new(Adt2::C8)), {
                                        let a2377 = v12;
                                        let a2378 = 1u64;
                                        nat_add(a2377, a2378)?
                                    }))
                                } else {
                                    let a2379 = v13;
                                    let a2380 = 44u64;
                                    if nat_eq(a2379, a2380) {
                                        let m2383 = f452(v0.clone(), {
                                            let a2381 = v12;
                                            let a2382 = 1u64;
                                            nat_add(a2381, a2382)?
                                        }, v2)?;
                                        match m2383 {
                                            Ok(v15) => {
                                                let a2384 = f154(v0.clone(), v15)?;
                                                let a2385 = 93u64;
                                                if nat_eq(a2384, a2385) {
                                                    Ok(Adt21::C0(Adt2::C9(std::rc::Rc::new(v9), std::rc::Rc::new(Adt2::C8)), {
                                                        let a2386 = v15;
                                                        let a2387 = 1u64;
                                                        nat_add(a2386, a2387)?
                                                    }))
                                                } else {
                                                    let m2388 = f340(v0, v15, v2, v7, 1u64, List::<Bytes>::nil(), Adt20::C0(0u64))?;
                                                    match m2388 {
                                                        Adt21::C0(v17, v18) => {
                                                            Ok(Adt21::C0(Adt2::C9(std::rc::Rc::new(v9), std::rc::Rc::new(v17)), v18))
                                                        }
                                                        Adt21::C1(v16) => {
                                                            Ok(Adt21::C1(v16))
                                                        }
                                                    }
                                                }
                                            }
                                            Err(v14) => {
                                                Ok(Adt21::C1(v14))
                                            }
                                        }
                                    } else {
                                        Ok(Adt21::C1(f98(Str::lit("a flow sequence entry must be followed by , or ] (YAML 1.2.2 [137])"), v12)?))
                                    }
                                }
                            }
                        }
                        Err(v11) => {
                            Ok(Adt21::C1(v11))
                        }
                    }
                }
                Adt21::C1(v8) => {
                    Ok(Adt21::C1(v8))
                }
            }
        } else {
            let a2389 = v4;
            let a2390 = 2u64;
            if nat_eq(a2389, a2390) {
                let m2391 = f453(v0.clone(), v1, v2)?;
                match m2391 {
                    Adt29::C0(v20, v21) => {
                        let m2392 = f452(v0.clone(), v21, v2)?;
                        match m2392 {
                            Ok(v23) => {
                                let m2406 = {
                                    let a2393 = f154(v0.clone(), v23)?;
                                    let a2394 = 44u64;
                                    if if nat_eq(a2393, a2394) {
                                        true
                                    } else {
                                        let a2395 = f154(v0.clone(), v23)?;
                                        let a2396 = 125u64;
                                        nat_eq(a2395, a2396)
                                    } {
                                        Adt21::C0(Adt2::C0, v23)
                                    } else {
                                        if f156({
                                            let a2397 = f154(v0.clone(), v23)?;
                                            let a2398 = 58u64;
                                            nat_eq(a2397, a2398)
                                        }) {
                                            Adt21::C1(f98(Str::lit("a flow mapping entry must be a key, then : and its value, or a key alone (YAML 1.2.2 [142], [146])"), v23)?)
                                        } else {
                                            let m2401 = f452(v0.clone(), {
                                                let a2399 = v23;
                                                let a2400 = 1u64;
                                                nat_add(a2399, a2400)?
                                            }, v2)?;
                                            match m2401 {
                                                Ok(v25) => {
                                                    let a2402 = f154(v0.clone(), v25)?;
                                                    let a2403 = 44u64;
                                                    if if nat_eq(a2402, a2403) {
                                                        true
                                                    } else {
                                                        let a2404 = f154(v0.clone(), v25)?;
                                                        let a2405 = 125u64;
                                                        nat_eq(a2404, a2405)
                                                    } {
                                                        Adt21::C0(Adt2::C0, v25)
                                                    } else {
                                                        f340(v0.clone(), v25, v2, v7, 0u64, List::<Bytes>::nil(), Adt20::C0(0u64))?
                                                    }
                                                }
                                                Err(v24) => {
                                                    Adt21::C1(v24)
                                                }
                                            }
                                        }
                                    }
                                };
                                match m2406 {
                                    Adt21::C0(v27, v28) => {
                                        if f251(v20.clone(), v5.clone()) {
                                            Ok(Adt21::C1(f98(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                        } else {
                                            let m2407 = f452(v0.clone(), v28, v2)?;
                                            match m2407 {
                                                Ok(v30) => {
                                                    let v31: u64 = f154(v0.clone(), v30)?;
                                                    let a2408 = v31;
                                                    let a2409 = 125u64;
                                                    if nat_eq(a2408, a2409) {
                                                        Ok(Adt21::C0(Adt2::C10(v20, std::rc::Rc::new(v27), std::rc::Rc::new(Adt2::C8)), {
                                                            let a2410 = v30;
                                                            let a2411 = 1u64;
                                                            nat_add(a2410, a2411)?
                                                        }))
                                                    } else {
                                                        let a2412 = v31;
                                                        let a2413 = 44u64;
                                                        if nat_eq(a2412, a2413) {
                                                            let m2416 = f452(v0.clone(), {
                                                                let a2414 = v30;
                                                                let a2415 = 1u64;
                                                                nat_add(a2414, a2415)?
                                                            }, v2)?;
                                                            match m2416 {
                                                                Ok(v33) => {
                                                                    let a2417 = f154(v0.clone(), v33)?;
                                                                    let a2418 = 125u64;
                                                                    if nat_eq(a2417, a2418) {
                                                                        Ok(Adt21::C0(Adt2::C10(v20, std::rc::Rc::new(v27), std::rc::Rc::new(Adt2::C8)), {
                                                                            let a2419 = v33;
                                                                            let a2420 = 1u64;
                                                                            nat_add(a2419, a2420)?
                                                                        }))
                                                                    } else {
                                                                        let m2421 = f340(v0, v33, v2, v7, 2u64, List::cons(v20.clone(), v5), Adt20::C0(0u64))?;
                                                                        match m2421 {
                                                                            Adt21::C0(v35, v36) => {
                                                                                Ok(Adt21::C0(Adt2::C10(v20, std::rc::Rc::new(v27), std::rc::Rc::new(v35)), v36))
                                                                            }
                                                                            Adt21::C1(v34) => {
                                                                                Ok(Adt21::C1(v34))
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                Err(v32) => {
                                                                    Ok(Adt21::C1(v32))
                                                                }
                                                            }
                                                        } else {
                                                            Ok(Adt21::C1(f98(Str::lit("a flow mapping entry must be followed by , or } (YAML 1.2.2 [140])"), v30)?))
                                                        }
                                                    }
                                                }
                                                Err(v29) => {
                                                    Ok(Adt21::C1(v29))
                                                }
                                            }
                                        }
                                    }
                                    Adt21::C1(v26) => {
                                        Ok(Adt21::C1(v26))
                                    }
                                }
                            }
                            Err(v22) => {
                                Ok(Adt21::C1(v22))
                            }
                        }
                    }
                    Adt29::C1(v19) => {
                        Ok(Adt21::C1(v19))
                    }
                }
            } else {
                let v37: u64 = f154(v0.clone(), v1)?;
                let a2422 = v37;
                let a2423 = 33u64;
                if nat_eq(a2422, a2423) {
                    if f240(v6) {
                        Ok(Adt21::C1(f98(Str::lit("a node holds at most one tag (YAML 1.2.2 [96])"), v1)?))
                    } else {
                        let v38: Adt20 = f241(v0.clone(), v1)?;
                        let m2424 = f242(v38.clone());
                        match m2424 {
                            None => {
                                let m2425 = f452(v0.clone(), f243(v0.clone(), v1)?, v2)?;
                                match m2425 {
                                    Ok(v41) => {
                                        let a2426 = f246(f8(Str::lit(",]}:")), f154(v0.clone(), v41)?)?;
                                        let a2427 = 4u64;
                                        if nat_lt(a2426, a2427) {
                                            f245(v38, v41, v41)
                                        } else {
                                            f340(v0, v41, v2, v7, 0u64, List::<Bytes>::nil(), v38)
                                        }
                                    }
                                    Err(v40) => {
                                        Ok(Adt21::C1(v40))
                                    }
                                }
                            }
                            Some(v39) => {
                                Ok(Adt21::C1(v39))
                            }
                        }
                    }
                } else {
                    let a2428 = v37;
                    let a2429 = 91u64;
                    if if nat_eq(a2428, a2429) {
                        true
                    } else {
                        let a2430 = v37;
                        let a2431 = 123u64;
                        nat_eq(a2430, a2431)
                    } {
                        let m2434 = f452(v0.clone(), {
                            let a2432 = v1;
                            let a2433 = 1u64;
                            nat_add(a2432, a2433)?
                        }, v2)?;
                        match m2434 {
                            Ok(v43) => {
                                f244(v6, {
                                    let a2435 = v37;
                                    let a2436 = 91u64;
                                    nat_eq(a2435, a2436)
                                }, v1, {
                                    let a2439 = f154(v0.clone(), v43)?;
                                    let a2440 = {
                                        let a2437 = v37;
                                        let a2438 = 2u64;
                                        nat_add(a2437, a2438)?
                                    };
                                    if nat_eq(a2439, a2440) {
                                        Adt21::C0(Adt2::C8, {
                                            let a2441 = v43;
                                            let a2442 = 1u64;
                                            nat_add(a2441, a2442)?
                                        })
                                    } else {
                                        f340(v0, v43, v2, v7, {
                                            let a2443 = v37;
                                            let a2444 = 91u64;
                                            if nat_eq(a2443, a2444) {
                                                1u64
                                            } else {
                                                2u64
                                            }
                                        }, List::<Bytes>::nil(), Adt20::C0(0u64))?
                                    }
                                })
                            }
                            Err(v42) => {
                                Ok(Adt21::C1(v42))
                            }
                        }
                    } else {
                        let a2445 = v37;
                        let a2446 = 34u64;
                        if if nat_eq(a2445, a2446) {
                            true
                        } else {
                            let a2447 = v37;
                            let a2448 = 39u64;
                            nat_eq(a2447, a2448)
                        } {
                            let m2449 = f339(v0, v1, v2, false)?;
                            match m2449 {
                                Adt29::C0(v45, v46) => {
                                    f338(v6, v45, true, v1, v46)
                                }
                                Adt29::C1(v44) => {
                                    Ok(Adt21::C1(v44))
                                }
                            }
                        } else {
                            let m2450 = f238(v0.clone(), v1)?;
                            match m2450 {
                                None => {
                                    if f156(f341(v0.clone(), v1, true)?) {
                                        Ok(Adt21::C1(f98(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                                    } else {
                                        let v48: u64 = f342(v0.clone(), v1, true)?;
                                        let v49: u64 = f233(v0.clone(), v48)?;
                                        let m2453 = {
                                            let a2451 = f154(v0.clone(), v49)?;
                                            let a2452 = 10u64;
                                            if nat_eq(a2451, a2452) {
                                                f452(v0.clone(), v49, v2)?
                                            } else {
                                                Ok::<u64, Bytes>(v49)
                                            }
                                        };
                                        match m2453 {
                                            Ok(v51) => {
                                                let a2454 = f154(v0.clone(), v49)?;
                                                let a2455 = 10u64;
                                                if if if nat_eq(a2454, a2455) {
                                                    f156(f435(f154(v0.clone(), v51)?))
                                                } else {
                                                    false
                                                } {
                                                    f156({
                                                        let a2456 = f154(v0.clone(), v51)?;
                                                        let a2457 = 58u64;
                                                        nat_eq(a2456, a2457)
                                                    })
                                                } else {
                                                    false
                                                } {
                                                    Ok(Adt21::C1(f98(Str::lit("a multi-line plain scalar inside a flow collection is not read (YAML 1.2.2 [135])"), v49)?))
                                                } else {
                                                    f338(v6, f247(v0, v1, v48), false, v1, v48)
                                                }
                                            }
                                            Err(v50) => {
                                                Ok(Adt21::C1(v50))
                                            }
                                        }
                                    }
                                }
                                Some(v47) => {
                                    Ok(Adt21::C1(v47))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f341(v0: Bytes, v1: u64, v2: bool) -> R<bool> {
    let v3: u64 = f154(v0.clone(), v1)?;
    if if f230(v3) {
        true
    } else {
        f231(v3)
    } {
        Ok(false)
    } else {
        let a2458 = v3;
        let a2459 = 45u64;
        if if if nat_eq(a2458, a2459) {
            true
        } else {
            let a2460 = v3;
            let a2461 = 63u64;
            nat_eq(a2460, a2461)
        } {
            true
        } else {
            let a2462 = v3;
            let a2463 = 58u64;
            nat_eq(a2462, a2463)
        } {
            Ok(f335(f154(v0, {
                let a2464 = v1;
                let a2465 = 1u64;
                nat_add(a2464, a2465)?
            })?, v2))
        } else {
            Ok(f156(f454(v3)?))
        }
    }
}

pub fn f342(v0: Bytes, v1: u64, v2: bool) -> R<u64> {
    let (_, h2466) = {
        let (h2467, _) = f223(Fn5::F455(v2, v0.clone()), {
            let a2469 = {
                let a2468 = v0;
                length_bytes(a2468)
            };
            let a2470 = 1u64;
            nat_add(a2469, a2470)?
        }, (v1, v1))?;
        h2467
    };
    Ok(h2466)
}

pub fn f343(v0: Bytes, p2495: u64, v2: u64, p2496: u64, p2497: Bytes) -> R<Adt29> {
    let m2494 = (p2495, (p2496, p2497));
    {
        let mut m2494 = m2494;
        loop {
            let step = {
                let (v1, (v3, v4)) = m2494;
                let m2471 = v3;
                if m2471 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt29>>(Ok(Adt29::C1(f98(Str::lit("a plain scalar has more lines than the input"), v1)?)))
                } else {
                    let v5 = m2471 - 1;
                    let v6: u64 = f233(v0.clone(), v1)?;
                    let v7: u64 = f154(v0.clone(), v6)?;
                    let a2472 = v7;
                    let a2473 = 58u64;
                    if nat_eq(a2472, a2473) {
                        Err::<(u64, (u64, Bytes)), R<Adt29>>(Ok(Adt29::C1(f456(v6)?)))
                    } else {
                        let a2474 = v7;
                        let a2475 = 35u64;
                        if if nat_eq(a2474, a2475) {
                            true
                        } else {
                            let a2476 = v7;
                            let a2477 = 256u64;
                            nat_eq(a2476, a2477)
                        } {
                            Err::<(u64, (u64, Bytes)), R<Adt29>>(Ok(Adt29::C0(v4, f234(v0.clone(), v6)?)))
                        } else {
                            let v8: (u64, u64) = f457(v0.clone(), {
                                let a2478 = v6;
                                let a2479 = 1u64;
                                nat_add(a2478, a2479)?
                            })?;
                            let v9: u64 = {
                                let (h2480, _) = v8;
                                h2480
                            };
                            let v10: u64 = f235(v0.clone(), v9)?;
                            let v11: u64 = f233(v0.clone(), v10)?;
                            let v12: u64 = f154(v0.clone(), v11)?;
                            let a2481 = v12;
                            let a2482 = 256u64;
                            if if if if nat_eq(a2481, a2482) {
                                true
                            } else {
                                let a2485 = {
                                    let a2483 = v10;
                                    let a2484 = v9;
                                    nat_sub(a2483, a2484)
                                };
                                let a2486 = v2;
                                nat_lt(a2485, a2486)
                            } {
                                true
                            } else {
                                let a2487 = v12;
                                let a2488 = 35u64;
                                nat_eq(a2487, a2488)
                            } {
                                true
                            } else {
                                f152(v0.clone(), v9)?
                            } {
                                Err::<(u64, (u64, Bytes)), R<Adt29>>(Ok(Adt29::C0(v4, {
                                    let a2489 = v6;
                                    let a2490 = 1u64;
                                    nat_add(a2489, a2490)?
                                })))
                            } else {
                                let v13: u64 = f342(v0.clone(), v11, false)?;
                                let a2491 = v13;
                                let a2492 = v11;
                                if nat_eq(a2491, a2492) {
                                    Err::<(u64, (u64, Bytes)), R<Adt29>>(Ok(Adt29::C1(f456(v11)?)))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Adt29>>((v13, (v5, f7(List::cons(v4, List::cons(f458({
                                        let (_, h2493) = v8;
                                        h2493
                                    }), List::cons(f247(v0.clone(), v11, v13), List::<Bytes>::nil())))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2494 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f344(v0: Bytes, v1: u64) -> R<u64> {
    let v2: bool = {
        let a2500 = f154(v0.clone(), {
            let a2498 = v1;
            let a2499 = 1u64;
            nat_add(a2498, a2499)?
        })?;
        let a2501 = 60u64;
        nat_eq(a2500, a2501)
    };
    let (h2502, _) = f149(Fn4::F459(v0.clone(), v2), {
        let a2504 = {
            let a2503 = v0;
            length_bytes(a2503)
        };
        let a2505 = 1u64;
        nat_add(a2504, a2505)?
    }, if v2 {
        let a2506 = v1;
        let a2507 = 2u64;
        nat_add(a2506, a2507)?
    } else {
        let a2508 = v1;
        let a2509 = 1u64;
        nat_add(a2508, a2509)?
    })?;
    Ok(h2502)
}

pub fn f345(v0: Bytes, v1: u64) -> R<Adt20> {
    let v2: u64 = f460(v0, List::cons(f8(Str::lit("str")), List::cons(f8(Str::lit("int")), List::cons(f8(Str::lit("float")), List::cons(f8(Str::lit("bool")), List::cons(f8(Str::lit("null")), List::cons(f8(Str::lit("seq")), List::cons(f8(Str::lit("map")), List::<Bytes>::nil()))))))), 1u64)?;
    let a2510 = v2;
    let a2511 = 0u64;
    if nat_eq(a2510, a2511) {
        Ok(Adt20::C2(f98(Str::lit("a tag outside the YAML 1.2.2 core schema is not read (YAML 1.2.2 section 10.3)"), v1)?))
    } else {
        Ok(Adt20::C0(v2))
    }
}

pub fn f346(v0: u64) -> R<Bytes> {
    f98(Str::lit("a node's content does not match its tag (YAML 1.2.2 section 10.3.2)"), v0)
}

pub fn f347(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f461(v0, v1, v2)
}

pub fn f348(v0: Bytes, v1: u64) -> R<bool> {
    let a2512 = f154(v0.clone(), v1)?;
    let a2513 = 58u64;
    if nat_eq(a2512, a2513) {
        if f230(f154(v0.clone(), {
            let a2514 = v1;
            let a2515 = 1u64;
            nat_add(a2514, a2515)?
        })?) {
            Ok(true)
        } else {
            Ok(f231(f154(v0, {
                let a2516 = v1;
                let a2517 = 1u64;
                nat_add(a2516, a2517)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f349(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a2518 = v0;
        utf8_encode(a2518)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f65(v1)?, List::<Bytes>::nil())))))
}

pub fn f350(p2521: List<Adt22>, p2522: List<Adt22>) -> List<Adt22> {
    let m2520 = (p2521, p2522);
    {
        let mut m2520 = m2520;
        loop {
            let step = {
                let (v0, v1) = m2520;
                let m2519 = v0;
                match m2519.uncons() {
                    None => {
                        Err::<(List<Adt22>, List<Adt22>), List<Adt22>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt22>, List<Adt22>), List<Adt22>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2520 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f351(v0: u64) -> bool {
    let a2523 = 65u64;
    let a2524 = v0;
    if if nat_le(a2523, a2524) {
        let a2525 = v0;
        let a2526 = 90u64;
        nat_le(a2525, a2526)
    } else {
        false
    } {
        true
    } else {
        let a2527 = 97u64;
        let a2528 = v0;
        if nat_le(a2527, a2528) {
            let a2529 = v0;
            let a2530 = 122u64;
            nat_le(a2529, a2530)
        } else {
            false
        }
    }
}

pub fn f352(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h2531, _) = f149(Fn4::F462(v1, v0.clone()), {
        let a2533 = {
            let a2532 = v0;
            length_bytes(a2532)
        };
        let a2534 = 1u64;
        nat_add(a2533, a2534)?
    }, v2)?;
    Ok(h2531)
}

pub fn f353(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f121(v0.clone(), v1)?;
    let a2535 = v2;
    let a2536 = 48u64;
    if nat_eq(a2535, a2536) {
        let v3: u64 = f121(v0.clone(), {
            let a2537 = v1;
            let a2538 = 1u64;
            nat_add(a2537, a2538)?
        })?;
        let a2539 = v3;
        let a2540 = 88u64;
        if if if nat_eq(a2539, a2540) {
            true
        } else {
            let a2541 = v3;
            let a2542 = 120u64;
            nat_eq(a2541, a2542)
        } {
            f463(f121(v0.clone(), {
                let a2543 = v1;
                let a2544 = 2u64;
                nat_add(a2543, a2544)?
            })?)
        } else {
            false
        } {
            f352(v0, 1u64, {
                let a2545 = v1;
                let a2546 = 2u64;
                nat_add(a2545, a2546)?
            })
        } else {
            f352(v0, 2u64, {
                let a2547 = v1;
                let a2548 = 1u64;
                nat_add(a2547, a2548)?
            })
        }
    } else {
        if f464(v2) {
            f352(v0, 0u64, v1)
        } else {
            Ok(v1)
        }
    }
}

pub fn f354(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f352(v0.clone(), 0u64, v1)?;
    let a2549 = f121(v0.clone(), v2)?;
    let a2550 = 46u64;
    if nat_eq(a2549, a2550) {
        let v3: u64 = f352(v0.clone(), 0u64, {
            let a2551 = v2;
            let a2552 = 1u64;
            nat_add(a2551, a2552)?
        })?;
        let a2553 = v1;
        let a2554 = v2;
        if if nat_lt(a2553, a2554) {
            true
        } else {
            let a2557 = {
                let a2555 = v2;
                let a2556 = 1u64;
                nat_add(a2555, a2556)?
            };
            let a2558 = v3;
            nat_lt(a2557, a2558)
        } {
            f465(v0, v3)
        } else {
            Ok(v1)
        }
    } else {
        let a2559 = v1;
        let a2560 = v2;
        if nat_lt(a2559, a2560) {
            let v4: u64 = f465(v0, v2)?;
            let a2561 = v2;
            let a2562 = v4;
            if nat_lt(a2561, a2562) {
                Ok(v4)
            } else {
                Ok(v1)
            }
        } else {
            Ok(v1)
        }
    }
}

pub fn f355(v0: List<Adt22>, v1: Str) -> bool {
    let m2563 = v0;
    match m2563.uncons() {
        None => {
            false
        }
        Some((v2, _)) => {
            f466(v2, {
                let a2564 = v1;
                utf8_encode(a2564)
            })
        }
    }
}

pub fn f356(p2577: u64, p2578: List<Adt22>, p2579: List<Bytes>, v3: bool) -> Result<List<Adt22>, Bytes> {
    let m2576 = (p2577, (p2578, p2579));
    {
        let mut m2576 = m2576;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2576;
                let m2565 = v0;
                if m2565 == 0 {
                    Err::<(u64, (List<Adt22>, List<Bytes>)), Result<List<Adt22>, Bytes>>(Err::<List<Adt22>, Bytes>(f8(Str::lit("brackets nest deeper than the input is long"))))
                } else {
                    let v4 = m2565 - 1;
                    let m2566 = v1;
                    match m2566.uncons() {
                        None => {
                            Err::<(u64, (List<Adt22>, List<Bytes>)), Result<List<Adt22>, Bytes>>(Err::<List<Adt22>, Bytes>(f8(Str::lit("the input ends inside brackets or before the ; that closes a member (Web IDL, IDL grammar)"))))
                        }
                        Some((v5, v6)) => {
                            let a2567 = f467(v5.clone());
                            let a2568 = 3u64;
                            if nat_eq(a2567, a2568) {
                                Ok::<(u64, (List<Adt22>, List<Bytes>)), Result<List<Adt22>, Bytes>>((v4, (v6, v2)))
                            } else {
                                let v7: Bytes = f468(v5);
                                let v8: Bytes = f469(v7.clone());
                                if !f46(v8.clone(), Bytes::lit(&[])) {
                                    Ok::<(u64, (List<Adt22>, List<Bytes>)), Result<List<Adt22>, Bytes>>((v4, (v6, List::cons(v8, v2))))
                                } else {
                                    if f470(v7.clone()) {
                                        let m2569 = v2;
                                        match m2569.uncons() {
                                            None => {
                                                Err::<(u64, (List<Adt22>, List<Bytes>)), Result<List<Adt22>, Bytes>>(Err::<List<Adt22>, Bytes>(f7(List::cons(f8(Str::lit("the closing bracket ")), List::cons(v7, List::cons(f8(Str::lit(" has no opening bracket (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
                                            }
                                            Some((v9, v10)) => {
                                                if f46(v9.clone(), v7.clone()) {
                                                    if if !v3 {
                                                        let a2571 = {
                                                            let a2570 = v10.clone();
                                                            length_list(a2570)
                                                        };
                                                        let a2572 = 0u64;
                                                        nat_eq(a2571, a2572)
                                                    } else {
                                                        false
                                                    } {
                                                        Err::<(u64, (List<Adt22>, List<Bytes>)), Result<List<Adt22>, Bytes>>(Ok::<List<Adt22>, Bytes>(v6))
                                                    } else {
                                                        Ok::<(u64, (List<Adt22>, List<Bytes>)), Result<List<Adt22>, Bytes>>((v4, (v6, v10)))
                                                    }
                                                } else {
                                                    Err::<(u64, (List<Adt22>, List<Bytes>)), Result<List<Adt22>, Bytes>>(Err::<List<Adt22>, Bytes>(f7(List::cons(f8(Str::lit("the closing bracket ")), List::cons(v7, List::cons(f8(Str::lit(" does not match the opening bracket, which needs ")), List::cons(v9, List::cons(f8(Str::lit(" (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))))
                                                }
                                            }
                                        }
                                    } else {
                                        if if v3 {
                                            if f46(v7, f8(Str::lit(";"))) {
                                                let a2574 = {
                                                    let a2573 = v2.clone();
                                                    length_list(a2573)
                                                };
                                                let a2575 = 0u64;
                                                nat_eq(a2574, a2575)
                                            } else {
                                                false
                                            }
                                        } else {
                                            false
                                        } {
                                            Err::<(u64, (List<Adt22>, List<Bytes>)), Result<List<Adt22>, Bytes>>(Ok::<List<Adt22>, Bytes>(v6))
                                        } else {
                                            Ok::<(u64, (List<Adt22>, List<Bytes>)), Result<List<Adt22>, Bytes>>((v4, (v6, v2)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2576 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f357(v0: List<Adt22>) -> List<Adt22> {
    let m2580 = v0;
    match m2580.uncons() {
        None => {
            List::<Adt22>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f358(v0: u64, v1: u64, v2: List<Adt22>, v3: bool) -> R<Result<(Adt13, List<Adt22>), Bytes>> {
    let m2581 = f361(v2);
    match m2581 {
        Ok(v5) => {
            let v6: bool = if v3 {
                f355(f363(v5.clone()), Str::lit(":"))
            } else {
                false
            };
            let m2582 = if v6 {
                f361(f357(f363(v5.clone())))
            } else {
                Ok::<Adt30, Bytes>(Adt30::C0(Bytes::lit(&[]), f363(v5.clone())))
            };
            match m2582 {
                Ok(v8) => {
                    let m2583 = f362(f363(v8.clone()), Str::lit("{"));
                    match m2583 {
                        Ok(v10) => {
                            let m2588 = {
                                let a2584 = v1;
                                let a2585 = 2u64;
                                if if nat_eq(a2584, a2585) {
                                    true
                                } else {
                                    let a2586 = v1;
                                    let a2587 = 7u64;
                                    nat_eq(a2586, a2587)
                                } {
                                    f471(v0, v10, List::<Adt14>::nil())?
                                } else {
                                    f472(v0, v10, List::<Adt14>::nil())?
                                }
                            };
                            match m2588 {
                                Ok(v12) => {
                                    Ok(Ok::<(Adt13, List<Adt22>), Bytes>((Adt13::C0(v1, f365(v5), f365(v8), {
                                        let (h2589, _) = v12;
                                        h2589
                                    }, List::<Bytes>::nil(), f366(Bytes::lit(&[]))), {
                                        let (_, h2590) = v12;
                                        h2590
                                    })))
                                }
                                Err(v11) => {
                                    Ok(Err::<(Adt13, List<Adt22>), Bytes>(v11))
                                }
                            }
                        }
                        Err(v9) => {
                            Ok(Err::<(Adt13, List<Adt22>), Bytes>(v9))
                        }
                    }
                }
                Err(v7) => {
                    Ok(Err::<(Adt13, List<Adt22>), Bytes>(v7))
                }
            }
        }
        Err(v4) => {
            Ok(Err::<(Adt13, List<Adt22>), Bytes>(v4))
        }
    }
}

pub fn f359(v0: List<Adt22>, v1: Bytes) -> R<Result<(Adt13, List<Adt22>), Bytes>> {
    let m2591 = f361(v0);
    match m2591 {
        Ok(v3) => {
            let m2592 = f473(f363(v3.clone()))?;
            match m2592 {
                Ok(v5) => {
                    Ok(Ok::<(Adt13, List<Adt22>), Bytes>((Adt13::C0(9u64, f365(v3), Bytes::lit(&[]), List::<Adt14>::nil(), List::cons(v1, List::<Bytes>::nil()), f366(Bytes::lit(&[]))), v5)))
                }
                Err(v4) => {
                    Ok(Err::<(Adt13, List<Adt22>), Bytes>(v4))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<(Adt13, List<Adt22>), Bytes>(v2))
        }
    }
}

pub fn f360(v0: Str, v1: List<Adt22>) -> Bytes {
    f7(List::cons(f8(Str::lit("expected ")), List::cons({
        let a2593 = v0;
        utf8_encode(a2593)
    }, List::cons(f8(Str::lit(" but found ")), List::cons(f474(v1), List::cons(f8(Str::lit(" (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
}

pub fn f361(v0: List<Adt22>) -> Result<Adt30, Bytes> {
    let m2594 = v0.clone();
    match m2594.uncons() {
        None => {
            Err::<Adt30, Bytes>(f360(Str::lit("an identifier"), v0))
        }
        Some((v1, v2)) => {
            let a2595 = f467(v1.clone());
            let a2596 = 0u64;
            if nat_eq(a2595, a2596) {
                Ok::<Adt30, Bytes>(Adt30::C0(f475(f468(v1)), v2))
            } else {
                Err::<Adt30, Bytes>(f360(Str::lit("an identifier"), v0))
            }
        }
    }
}

pub fn f362(v0: List<Adt22>, v1: Str) -> Result<List<Adt22>, Bytes> {
    if f355(v0.clone(), v1.clone()) {
        Ok::<List<Adt22>, Bytes>(f357(v0))
    } else {
        Err::<List<Adt22>, Bytes>(f360(v1, v0))
    }
}

pub fn f363(v0: Adt30) -> List<Adt22> {
    let m2597 = v0;
    match m2597 {
        Adt30::C0(_, v2) => {
            v2
        }
    }
}

pub fn f364(p2605: u64, p2606: List<Adt22>, p2607: List<Bytes>) -> Result<(List<Bytes>, List<Adt22>), Bytes> {
    let m2604 = (p2605, (p2606, p2607));
    {
        let mut m2604 = m2604;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2604;
                let m2598 = v0;
                if m2598 == 0 {
                    Err::<(u64, (List<Adt22>, List<Bytes>)), Result<(List<Bytes>, List<Adt22>), Bytes>>(Err::<(List<Bytes>, List<Adt22>), Bytes>(f8(Str::lit("an enumeration has more values than the input has tokens"))))
                } else {
                    let v3 = m2598 - 1;
                    let m2599 = v1.clone();
                    match m2599.uncons() {
                        None => {
                            Err::<(u64, (List<Adt22>, List<Bytes>)), Result<(List<Bytes>, List<Adt22>), Bytes>>(Err::<(List<Bytes>, List<Adt22>), Bytes>(f360(Str::lit("a string"), v1)))
                        }
                        Some((v4, v5)) => {
                            let a2600 = f467(v4.clone());
                            let a2601 = 3u64;
                            if !nat_eq(a2600, a2601) {
                                Err::<(u64, (List<Adt22>, List<Bytes>)), Result<(List<Bytes>, List<Adt22>), Bytes>>(Err::<(List<Bytes>, List<Adt22>), Bytes>(f360(Str::lit("a string"), v1)))
                            } else {
                                if f355(v5.clone(), Str::lit(",")) {
                                    if f355(f357(v5.clone()), Str::lit("}")) {
                                        let m2602 = f362(f357(f357(v5)), Str::lit(";"));
                                        match m2602 {
                                            Ok(v7) => {
                                                Err::<(u64, (List<Adt22>, List<Bytes>)), Result<(List<Bytes>, List<Adt22>), Bytes>>(Ok::<(List<Bytes>, List<Adt22>), Bytes>((f59(List::cons(f468(v4), v2)), v7)))
                                            }
                                            Err(v6) => {
                                                Err::<(u64, (List<Adt22>, List<Bytes>)), Result<(List<Bytes>, List<Adt22>), Bytes>>(Err::<(List<Bytes>, List<Adt22>), Bytes>(v6))
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (List<Adt22>, List<Bytes>)), Result<(List<Bytes>, List<Adt22>), Bytes>>((v3, (f357(v5), List::cons(f468(v4), v2))))
                                    }
                                } else {
                                    if f355(v5.clone(), Str::lit("}")) {
                                        let m2603 = f362(f357(v5), Str::lit(";"));
                                        match m2603 {
                                            Ok(v9) => {
                                                Err::<(u64, (List<Adt22>, List<Bytes>)), Result<(List<Bytes>, List<Adt22>), Bytes>>(Ok::<(List<Bytes>, List<Adt22>), Bytes>((f59(List::cons(f468(v4), v2)), v9)))
                                            }
                                            Err(v8) => {
                                                Err::<(u64, (List<Adt22>, List<Bytes>)), Result<(List<Bytes>, List<Adt22>), Bytes>>(Err::<(List<Bytes>, List<Adt22>), Bytes>(v8))
                                            }
                                        }
                                    } else {
                                        Err::<(u64, (List<Adt22>, List<Bytes>)), Result<(List<Bytes>, List<Adt22>), Bytes>>(Err::<(List<Bytes>, List<Adt22>), Bytes>(f360(Str::lit(", or }"), v5)))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2604 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f365(v0: Adt30) -> Bytes {
    let m2608 = v0;
    match m2608 {
        Adt30::C0(v1, _) => {
            v1
        }
    }
}

pub fn f366(v0: Bytes) -> Adt15 {
    Adt15::C0(0u64, v0, List::<Bytes>::nil(), Bytes::lit(&[]), Bytes::lit(&[]))
}

pub fn f367(v0: u64, v1: List<Adt22>, v2: u64) -> R<Result<Adt31, Bytes>> {
    let m2609 = v0;
    if m2609 == 0 {
        Ok(Err::<Adt31, Bytes>(f8(Str::lit("a type nests deeper than the input is long"))))
    } else {
        let v3 = m2609 - 1;
        let a2610 = v2;
        let a2611 = 0u64;
        if nat_eq(a2610, a2611) {
            let m2612 = f255(v1)?;
            match m2612 {
                Ok(v5) => {
                    if f355(v5.clone(), Str::lit("(")) {
                        let m2613 = f367(v3, f357(v5), 1u64)?;
                        match m2613 {
                            Ok(v7) => {
                                if f355(f368(v7.clone()), Str::lit("?")) {
                                    Ok(Ok::<Adt31, Bytes>(Adt31::C0(Adt15::C0(0u64, Bytes::lit(&[]), f274(f369(v7.clone())), f8(Str::lit("a nullable union type is not translated yet, because the typedef identifier names only the union (Web IDL 2.13.32)")), Bytes::lit(&[])), f357(f368(v7)))))
                                } else {
                                    Ok(Ok::<Adt31, Bytes>(Adt31::C0(f369(v7.clone()), f368(v7))))
                                }
                            }
                            Err(v6) => {
                                Ok(Err::<Adt31, Bytes>(v6))
                            }
                        }
                    } else {
                        let m2614 = v5.clone();
                        match m2614.uncons() {
                            None => {
                                Ok(Err::<Adt31, Bytes>(f360(Str::lit("a type"), v5)))
                            }
                            Some((v8, v9)) => {
                                let v10: Bytes = f468(v8.clone());
                                let a2615 = f467(v8);
                                let a2616 = 0u64;
                                if !nat_eq(a2615, a2616) {
                                    Ok(Err::<Adt31, Bytes>(f360(Str::lit("a type"), v5)))
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
                                        let m2617 = f362(v9, Str::lit("<"));
                                        match m2617 {
                                            Ok(v12) => {
                                                let m2618 = f367(v3, v12, 0u64)?;
                                                match m2618 {
                                                    Ok(v14) => {
                                                        let m2619 = f362(f368(v14.clone()), Str::lit(">"));
                                                        match m2619 {
                                                            Ok(v16) => {
                                                                Ok(f476(f477(Str::lit("List"), f369(v14)), v16))
                                                            }
                                                            Err(v15) => {
                                                                Ok(Err::<Adt31, Bytes>(v15))
                                                            }
                                                        }
                                                    }
                                                    Err(v13) => {
                                                        Ok(Err::<Adt31, Bytes>(v13))
                                                    }
                                                }
                                            }
                                            Err(v11) => {
                                                Ok(Err::<Adt31, Bytes>(v11))
                                            }
                                        }
                                    } else {
                                        if f46(v10.clone(), f8(Str::lit("async_sequence"))) {
                                            let m2620 = f362(v9, Str::lit("<"));
                                            match m2620 {
                                                Ok(v18) => {
                                                    let m2621 = f367(v3, v18, 0u64)?;
                                                    match m2621 {
                                                        Ok(v20) => {
                                                            let m2622 = f362(f368(v20), Str::lit(">"));
                                                            match m2622 {
                                                                Ok(v22) => {
                                                                    Ok(f476(f478(f8(Str::lit("async_sequence types are not translated, because an async sequence is produced over time and is not a value (Web IDL 2.13.29)"))), v22))
                                                                }
                                                                Err(v21) => {
                                                                    Ok(Err::<Adt31, Bytes>(v21))
                                                                }
                                                            }
                                                        }
                                                        Err(v19) => {
                                                            Ok(Err::<Adt31, Bytes>(v19))
                                                        }
                                                    }
                                                }
                                                Err(v17) => {
                                                    Ok(Err::<Adt31, Bytes>(v17))
                                                }
                                            }
                                        } else {
                                            if f46(v10.clone(), f8(Str::lit("Promise"))) {
                                                let m2623 = f362(v9, Str::lit("<"));
                                                match m2623 {
                                                    Ok(v24) => {
                                                        let m2624 = f367(v3, v24, 0u64)?;
                                                        match m2624 {
                                                            Ok(v26) => {
                                                                let m2625 = f362(f368(v26), Str::lit(">"));
                                                                match m2625 {
                                                                    Ok(v28) => {
                                                                        Ok(f476(f478(f8(Str::lit("Promise types are not translated, because a promise is a deferred result, not a value (Web IDL 2.13.31)"))), v28))
                                                                    }
                                                                    Err(v27) => {
                                                                        Ok(Err::<Adt31, Bytes>(v27))
                                                                    }
                                                                }
                                                            }
                                                            Err(v25) => {
                                                                Ok(Err::<Adt31, Bytes>(v25))
                                                            }
                                                        }
                                                    }
                                                    Err(v23) => {
                                                        Ok(Err::<Adt31, Bytes>(v23))
                                                    }
                                                }
                                            } else {
                                                if f46(v10.clone(), f8(Str::lit("record"))) {
                                                    let m2626 = f362(v9, Str::lit("<"));
                                                    match m2626 {
                                                        Ok(v30) => {
                                                            let m2627 = v30.clone();
                                                            match m2627.uncons() {
                                                                None => {
                                                                    Ok(Err::<Adt31, Bytes>(f360(Str::lit("a string type"), v30)))
                                                                }
                                                                Some((v31, v32)) => {
                                                                    if if f46(f468(v31.clone()), f8(Str::lit("DOMString"))) {
                                                                        true
                                                                    } else {
                                                                        if f46(f468(v31.clone()), f8(Str::lit("USVString"))) {
                                                                            true
                                                                        } else {
                                                                            f46(f468(v31.clone()), f8(Str::lit("ByteString")))
                                                                        }
                                                                    } {
                                                                        let m2628 = f362(v32, Str::lit(","));
                                                                        match m2628 {
                                                                            Ok(v34) => {
                                                                                let m2629 = f367(v3, v34, 0u64)?;
                                                                                match m2629 {
                                                                                    Ok(v36) => {
                                                                                        let m2630 = f362(f368(v36.clone()), Str::lit(">"));
                                                                                        match m2630 {
                                                                                            Ok(v38) => {
                                                                                                let m2631 = f479(f468(v31));
                                                                                                match m2631 {
                                                                                                    None => {
                                                                                                        Ok(Err::<Adt31, Bytes>(f360(Str::lit("a string type"), v30)))
                                                                                                    }
                                                                                                    Some(v39) => {
                                                                                                        let v40: Adt15 = f480(f369(v36));
                                                                                                        Ok(f476(Adt15::C0(0u64, f7(List::cons(f8(Str::lit("(List (")), List::cons(f273(v39), List::cons(f8(Str::lit(" * ")), List::cons(f273(v40.clone()), List::cons(f8(Str::lit("))")), List::<Bytes>::nil())))))), f274(v40.clone()), f271(v40), Bytes::lit(&[])), v38))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            Err(v37) => {
                                                                                                Ok(Err::<Adt31, Bytes>(v37))
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    Err(v35) => {
                                                                                        Ok(Err::<Adt31, Bytes>(v35))
                                                                                    }
                                                                                }
                                                                            }
                                                                            Err(v33) => {
                                                                                Ok(Err::<Adt31, Bytes>(v33))
                                                                            }
                                                                        }
                                                                    } else {
                                                                        Ok(Err::<Adt31, Bytes>(f360(Str::lit("DOMString, USVString or ByteString"), v30)))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Err(v29) => {
                                                            Ok(Err::<Adt31, Bytes>(v29))
                                                        }
                                                    }
                                                } else {
                                                    if f46(v10.clone(), f8(Str::lit("unsigned"))) {
                                                        if f355(v9.clone(), Str::lit("short")) {
                                                            Ok(f476(f366(f8(Str::lit("Nat"))), f357(v9)))
                                                        } else {
                                                            if f355(v9.clone(), Str::lit("long")) {
                                                                let v41: List<Adt22> = f357(v9);
                                                                Ok(f476(f366(f8(Str::lit("Nat"))), if f355(v41.clone(), Str::lit("long")) {
                                                                    f357(v41)
                                                                } else {
                                                                    v41
                                                                }))
                                                            } else {
                                                                Ok(Err::<Adt31, Bytes>(f360(Str::lit("short or long"), v9)))
                                                            }
                                                        }
                                                    } else {
                                                        if f46(v10.clone(), f8(Str::lit("short"))) {
                                                            Ok(f476(f366(f8(Str::lit("Int"))), v9))
                                                        } else {
                                                            if f46(v10.clone(), f8(Str::lit("long"))) {
                                                                Ok(f476(f366(f8(Str::lit("Int"))), if f355(v9.clone(), Str::lit("long")) {
                                                                    f357(v9)
                                                                } else {
                                                                    v9
                                                                }))
                                                            } else {
                                                                if f46(v10.clone(), f8(Str::lit("unrestricted"))) {
                                                                    if if f355(v9.clone(), Str::lit("float")) {
                                                                        true
                                                                    } else {
                                                                        f355(v9.clone(), Str::lit("double"))
                                                                    } {
                                                                        Ok(f476(f478(f8(Str::lit("unrestricted float and unrestricted double are not translated, because its value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.13 and 2.13.15)"))), f357(v9)))
                                                                    } else {
                                                                        Ok(Err::<Adt31, Bytes>(f360(Str::lit("float or double"), v9)))
                                                                    }
                                                                } else {
                                                                    let m2632 = f479(v10.clone());
                                                                    match m2632 {
                                                                        None => {
                                                                            let v42: Bytes = f475(v10);
                                                                            Ok(f476(Adt15::C0(0u64, f258(v42.clone())?, List::cons(v42.clone(), List::<Bytes>::nil()), Bytes::lit(&[]), v42), v9))
                                                                        }
                                                                        Some(v43) => {
                                                                            Ok(f476(v43, v9))
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
                    Ok(Err::<Adt31, Bytes>(v4))
                }
            }
        } else {
            let m2633 = f367(v3, v1, 0u64)?;
            match m2633 {
                Ok(v45) => {
                    let v46: Adt15 = f369(v45.clone());
                    let v48: Adt15 = {
                        let a2634 = f272(v46.clone());
                        let a2635 = 0u64;
                        if nat_eq(a2634, a2635) {
                            if f46(f271(v46.clone()), Bytes::lit(&[])) {
                                if f46(f481(v46.clone()), Bytes::lit(&[])) {
                                    Adt15::C0(1u64, Bytes::lit(&[]), f274(v46), f8(Str::lit("a union member type that is not a named definition gives its constructor no name, so the union is not translated yet (Web IDL 2.13.32)")), Bytes::lit(&[]))
                                } else {
                                    let m2636 = f379(false, f481(v46.clone()))?;
                                    match m2636 {
                                        None => {
                                            Adt15::C0(1u64, Bytes::lit(&[]), f274(v46.clone()), f7(List::cons(f8(Str::lit("the union member ")), List::cons(f481(v46), List::cons(f8(Str::lit(" has no constructor name")), List::<Bytes>::nil())))), Bytes::lit(&[]))
                                        }
                                        Some(v47) => {
                                            Adt15::C0(1u64, f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v47, List::cons(f8(Str::lit(", fields: [")), List::cons(f273(v46.clone()), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f274(v46), Bytes::lit(&[]), Bytes::lit(&[]))
                                        }
                                    }
                                }
                            } else {
                                Adt15::C0(1u64, Bytes::lit(&[]), f274(v46.clone()), f271(v46), Bytes::lit(&[]))
                            }
                        } else {
                            v46
                        }
                    };
                    if f355(f368(v45.clone()), Str::lit("or")) {
                        let m2637 = f367(v3, f357(f368(v45)), 2u64)?;
                        match m2637 {
                            Ok(v50) => {
                                Ok(Ok::<Adt31, Bytes>(Adt31::C0(Adt15::C0(1u64, if f46(f273(v48.clone()), Bytes::lit(&[])) {
                                    f273(f369(v50.clone()))
                                } else {
                                    if f46(f273(f369(v50.clone())), Bytes::lit(&[])) {
                                        f273(v48.clone())
                                    } else {
                                        f7(List::cons(f273(v48.clone()), List::cons(f8(Str::lit(",\u{a}")), List::cons(f273(f369(v50.clone())), List::<Bytes>::nil()))))
                                    }
                                }, f170(f274(v48.clone()), f274(f369(v50.clone()))), if f46(f271(v48.clone()), Bytes::lit(&[])) {
                                    f271(f369(v50.clone()))
                                } else {
                                    f271(v48)
                                }, Bytes::lit(&[])), f368(v50))))
                            }
                            Err(v49) => {
                                Ok(Err::<Adt31, Bytes>(v49))
                            }
                        }
                    } else {
                        let a2638 = v2;
                        let a2639 = 2u64;
                        if if nat_eq(a2638, a2639) {
                            f355(f368(v45.clone()), Str::lit(")"))
                        } else {
                            false
                        } {
                            Ok(Ok::<Adt31, Bytes>(Adt31::C0(v48, f357(f368(v45)))))
                        } else {
                            Ok(Err::<Adt31, Bytes>({
                                let a2640 = v2;
                                let a2641 = 1u64;
                                if nat_eq(a2640, a2641) {
                                    f360(Str::lit("or"), f368(v45))
                                } else {
                                    f360(Str::lit("or or )"), f368(v45))
                                }
                            }))
                        }
                    }
                }
                Err(v44) => {
                    Ok(Err::<Adt31, Bytes>(v44))
                }
            }
        }
    }
}

pub fn f368(v0: Adt31) -> List<Adt22> {
    let m2642 = v0;
    match m2642 {
        Adt31::C0(_, v2) => {
            v2
        }
    }
}

pub fn f369(v0: Adt31) -> Adt15 {
    let m2643 = v0;
    match m2643 {
        Adt31::C0(v1, _) => {
            v1
        }
    }
}

pub fn f370() -> List<Bytes> {
    List::cons(f8(Str::lit("Bool")), List::cons(f8(Str::lit("ContractViolation")), List::cons(f8(Str::lit("List")), List::cons(f8(Str::lit("Nat")), List::cons(f8(Str::lit("Option")), List::cons(f8(Str::lit("Prod")), List::cons(f8(Str::lit("ReasoningFailure")), List::cons(f8(Str::lit("Result")), List::<Bytes>::nil()))))))))
}

pub fn f371(v0: Bytes) -> R<Bytes> {
    let m2646 = {
        let a2644 = v0.clone();
        let a2645 = 0u64;
        index_bytes(a2644, a2645)
    };
    match m2646 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            let a2652 = {
                let a2647 = f8(Str::lit("u"));
                let a2648 = f482(8u64, f53(v1)?, 4u64, Bytes::lit(&[]));
                append_bytes(a2647, a2648)
            };
            let a2653 = f43(v0.clone(), 1u64, {
                let a2650 = {
                    let a2649 = v0;
                    length_bytes(a2649)
                };
                let a2651 = 1u64;
                nat_sub(a2650, a2651)
            });
            Ok(append_bytes(a2652, a2653))
        }
    }
}

pub fn f372(v0: List<Adt14>) -> R<List<Adt14>> {
    let m2654 = v0;
    match m2654.uncons() {
        None => {
            Ok(List::<Adt14>::nil())
        }
        Some((v1, v2)) => {
            f483(v1, f372(v2)?)
        }
    }
}

pub fn f373(v0: List<Adt14>, v1: List<Adt14>) -> List<Adt14> {
    let m2655 = v0;
    match m2655.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f373(v3, v1))
        }
    }
}

pub fn f374(v0: Adt13) -> List<Adt14> {
    let m2656 = v0;
    match m2656 {
        Adt13::C0(_, _, _, v4, _, _) => {
            v4
        }
    }
}

pub fn f375(v0: List<Adt13>, v1: u64, v2: Bytes) -> List<Adt14> {
    let m2657 = v0;
    match m2657.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v3, v4)) => {
            let a2658 = f164(v3.clone());
            let a2659 = v1;
            if if nat_eq(a2658, a2659) {
                f46(f166(v3.clone()), v2.clone())
            } else {
                false
            } {
                f373(f374(v3), f375(v4, v1, v2))
            } else {
                f375(v4, v1, v2)
            }
        }
    }
}

pub fn f376(v0: List<Adt13>, v1: List<Adt13>, v2: Bytes) -> List<Adt14> {
    let m2660 = v0;
    match m2660.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v3, v4)) => {
            let a2661 = f164(v3.clone());
            let a2662 = 8u64;
            if if nat_eq(a2661, a2662) {
                f46(f166(v3.clone()), v2.clone())
            } else {
                false
            } {
                f373(f484(v1.clone(), f177(v3)), f376(v4, v1, v2))
            } else {
                f376(v4, v1, v2)
            }
        }
    }
}

pub fn f377(v0: Adt14) -> Adt15 {
    let m2663 = v0;
    match m2663 {
        Adt14::C0(_, v2, _) => {
            v2
        }
    }
}

pub fn f378(v0: Adt14) -> Bytes {
    let m2664 = v0;
    match m2664 {
        Adt14::C0(v1, _, _) => {
            v1
        }
    }
}

pub fn f379(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f485(v1)
    } else {
        f382(v1)
    }
}

pub fn f380(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m2665 = v1;
    match m2665.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m2666 = f379(v0, v2)?;
            match m2666 {
                None => {
                    f380(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f380(v0, v3)?))
                }
            }
        }
    }
}

pub fn f381(v0: Adt14) -> u64 {
    let m2667 = v0;
    match m2667 {
        Adt14::C0(_, _, v3) => {
            v3
        }
    }
}

pub fn f382(v0: Bytes) -> R<Option<Bytes>> {
    let m2668 = f486(v0)?;
    match m2668 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            Ok(Some(f487(v1)?))
        }
    }
}

pub fn f383(v0: bool, v1: Adt2) -> R<Option<Bytes>> {
    let m2669 = v1;
    match m2669 {
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
                let a2680 = if f87(v7, {
                    let a2678 = 0u64;
                    let a2679 = v8;
                    nat_lt(a2678, a2679)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a2681 = f314(false, v8)?;
                append_bytes(a2680, a2681)
            }))
        }
        Adt2::C4(v3, v4, v5) => {
            let m2672 = f485({
                let a2670 = f8(Str::lit("n"));
                let a2671 = f315(false, v4, v5)?;
                append_bytes(a2670, a2671)
            })?;
            match m2672 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a2676 = f8(Str::lit("nMinus"));
                        let a2677 = f43(v6.clone(), 1u64, {
                            let a2674 = {
                                let a2673 = v6;
                                length_bytes(a2673)
                            };
                            let a2675 = 1u64;
                            nat_sub(a2674, a2675)
                        });
                        append_bytes(a2676, a2677)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt2::C5(v2) => {
            f379(v0, v2)
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

pub fn f384(v0: Adt2) -> R<Bytes> {
    f218(v0)
}

pub fn f385(v0: u64) -> R<bool> {
    f277(v0, f488())
}

pub fn f386(v0: u64, v1: Bytes, p2692: u64, p2693: u64, p2694: bool) -> R<bool> {
    let m2691 = (p2692, (p2693, p2694));
    {
        let mut m2691 = m2691;
        loop {
            let step = {
                let (v2, (v3, v4)) = m2691;
                let m2682 = v3;
                if m2682 == 0 {
                    Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                } else {
                    let v5 = m2682 - 1;
                    let v6: u64 = f489(v1.clone(), v2)?;
                    if v4 {
                        let a2683 = v0;
                        let a2684 = v6;
                        if nat_lt(a2683, a2684) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a2685 = v2;
                                let a2686 = 1u64;
                                nat_add(a2685, a2686)?
                            }, (v5, false)))
                        }
                    } else {
                        let a2687 = v0;
                        let a2688 = v6;
                        if nat_le(a2687, a2688) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(true))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a2689 = v2;
                                let a2690 = 1u64;
                                nat_add(a2689, a2690)?
                            }, (v5, true)))
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

pub fn f387(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    if f94(v0.clone(), v1, v2) {
        Ok(None::<u64>)
    } else {
        let a2696 = v2;
        let a2697 = {
            let a2695 = v0;
            length_bytes(a2695)
        };
        if nat_lt(a2696, a2697) {
            Ok(Some({
                let a2698 = v2;
                let a2699 = 1u64;
                nat_add(a2698, a2699)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f388(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f179(f121(v0, v1)?)? {
        Ok(Some({
            let a2700 = v1;
            let a2701 = 1u64;
            nat_add(a2700, a2701)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f389(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let m2702 = f328(v0, v2)?;
    match m2702 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            let a2703 = v2;
            let a2704 = v1;
            if if nat_eq(a2703, a2704) {
                f490({
                    let (h2705, _) = v3;
                    h2705
                })?
            } else {
                f491({
                    let (h2706, _) = v3;
                    h2706
                })?
            } {
                Ok(Some({
                    let a2708 = v2;
                    let a2709 = {
                        let (_, h2707) = v3;
                        h2707
                    };
                    nat_add(a2708, a2709)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f390(v0: Bytes, v1: bool, v2: u64) -> R<(u64, u64)> {
    let (h2710, _) = f223(Fn5::F492(v1, v0.clone()), {
        let a2712 = {
            let a2711 = v0;
            length_bytes(a2711)
        };
        let a2713 = 1u64;
        nat_add(a2712, a2713)?
    }, (v2, 0u64))?;
    Ok(h2710)
}

pub fn f391(v0: u64) -> R<Bytes> {
    let a2714 = v0;
    let a2715 = 128u64;
    if nat_lt(a2714, a2715) {
        Ok(f493(f494(), v0))
    } else {
        let a2716 = v0;
        let a2717 = 2048u64;
        if nat_lt(a2716, a2717) {
            let a2725 = f493(f495(), {
                let a2723 = {
                    let a2721 = {
                        let a2718 = v0;
                        let a2719 = 64u64;
                        let a2720 = 0u64;
                        nat_quot(a2718, a2719, a2720)
                    };
                    let a2722 = 2u64;
                    nat_sub(a2721, a2722)
                };
                let a2724 = 2u64;
                nat_mul(a2723, a2724)?
            });
            let a2726 = f496(v0)?;
            Ok(append_bytes(a2725, a2726))
        } else {
            let a2727 = v0;
            let a2728 = 65536u64;
            if nat_lt(a2727, a2728) {
                let a2739 = f493(f497(), {
                    let a2732 = {
                        let a2729 = v0;
                        let a2730 = 4096u64;
                        let a2731 = 0u64;
                        nat_quot(a2729, a2730, a2731)
                    };
                    let a2733 = 3u64;
                    nat_mul(a2732, a2733)?
                });
                let a2740 = {
                    let a2737 = f496({
                        let a2734 = v0;
                        let a2735 = 64u64;
                        let a2736 = 0u64;
                        nat_quot(a2734, a2735, a2736)
                    })?;
                    let a2738 = f496(v0)?;
                    append_bytes(a2737, a2738)
                };
                Ok(append_bytes(a2739, a2740))
            } else {
                let a2754 = f493(Bytes::lit(&[0xf0, 0xf1, 0xf2, 0xf3, 0xf4]), {
                    let a2741 = v0;
                    let a2742 = 262144u64;
                    let a2743 = 0u64;
                    nat_quot(a2741, a2742, a2743)
                });
                let a2755 = {
                    let a2752 = f496({
                        let a2744 = v0;
                        let a2745 = 4096u64;
                        let a2746 = 0u64;
                        nat_quot(a2744, a2745, a2746)
                    })?;
                    let a2753 = {
                        let a2750 = f496({
                            let a2747 = v0;
                            let a2748 = 64u64;
                            let a2749 = 0u64;
                            nat_quot(a2747, a2748, a2749)
                        })?;
                        let a2751 = f496(v0)?;
                        append_bytes(a2750, a2751)
                    };
                    append_bytes(a2752, a2753)
                };
                Ok(append_bytes(a2754, a2755))
            }
        }
    }
}

pub fn f392(v0: bool, v1: u64, v2: Bytes, v3: u64) -> R<Option<u64>> {
    f498(v2, v0, v1, v3)
}

pub fn f393(v0: Bytes) -> Result<Option<Adt28>, Bytes> {
    Err::<Option<Adt28>, Bytes>(v0)
}

pub fn f394(p2759: Bytes, p2760: u64) -> R<Result<Bytes, Bytes>> {
    let m2758 = (p2759, p2760);
    {
        let mut m2758 = m2758;
        loop {
            let step = {
                let (v0, v1) = m2758;
                let m2756 = v1;
                if m2756 == 0 {
                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f499(f8(Str::lit("the built-in datatype hierarchy is deeper than XSD 1.1 Part 2 section 3 defines")))))
                } else {
                    let v2 = m2756 - 1;
                    if f251(v0.clone(), List::cons(f8(Str::lit("NMTOKENS")), List::cons(f8(Str::lit("IDREFS")), List::cons(f8(Str::lit("ENTITIES")), List::<Bytes>::nil())))) {
                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("(List Bytes)")))))
                    } else {
                        if f46(v0.clone(), f8(Str::lit("nonNegativeInteger"))) {
                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Nat")))))
                        } else {
                            if f46(v0.clone(), f8(Str::lit("integer"))) {
                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Int")))))
                            } else {
                                if f46(v0.clone(), f8(Str::lit("decimal"))) {
                                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f500())))
                                } else {
                                    if f46(v0.clone(), f8(Str::lit("boolean"))) {
                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Bool")))))
                                    } else {
                                        if f251(v0.clone(), List::cons(f8(Str::lit("float")), List::cons(f8(Str::lit("double")), List::<Bytes>::nil()))) {
                                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f499(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because its value space holds positive and negative zero, the infinities and NaN, which no exact LexLean number type holds (XSD 1.1 Part 2, sections 3.3.4 and 3.3.5)")), List::<Bytes>::nil())))))))
                                        } else {
                                            if f251(v0.clone(), List::cons(f8(Str::lit("anySimpleType")), List::cons(f8(Str::lit("anyAtomicType")), List::<Bytes>::nil()))) {
                                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f499(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because it is the union of every simple value space and has no one LexLean type (XSD 1.1 Part 2, sections 3.2.1 and 3.2.2)")), List::<Bytes>::nil())))))))
                                            } else {
                                                let m2757 = f501(v0.clone())?;
                                                match m2757 {
                                                    None => {
                                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f499(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not a built-in datatype of XSD 1.1 Part 2")), List::<Bytes>::nil())))))))
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
                Ok(next) => { m2758 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f395(v0: Bytes) -> Result<List<Adt2>, Bytes> {
    Err::<List<Adt2>, Bytes>(v0)
}

pub fn f396(v0: u64) -> bool {
    f141({
        let a2761 = v0;
        let a2762 = 95u64;
        nat_eq(a2761, a2762)
    }, f141({
        let a2763 = v0;
        let a2764 = 45u64;
        nat_eq(a2763, a2764)
    }, f141({
        let a2765 = v0;
        let a2766 = 32u64;
        nat_eq(a2765, a2766)
    }, f141({
        let a2767 = v0;
        let a2768 = 46u64;
        nat_eq(a2767, a2768)
    }, {
        let a2769 = v0;
        let a2770 = 47u64;
        nat_eq(a2769, a2770)
    }))))
}

pub fn f397(v0: u64) -> bool {
    f87({
        let a2771 = 97u64;
        let a2772 = v0;
        nat_le(a2771, a2772)
    }, {
        let a2773 = v0;
        let a2774 = 122u64;
        nat_le(a2773, a2774)
    })
}

pub fn f398(v0: u64) -> bool {
    f87({
        let a2775 = 65u64;
        let a2776 = v0;
        nat_le(a2775, a2776)
    }, {
        let a2777 = v0;
        let a2778 = 90u64;
        nat_le(a2777, a2778)
    })
}

pub fn f399(v0: u64) -> bool {
    f87({
        let a2779 = 48u64;
        let a2780 = v0;
        nat_le(a2779, a2780)
    }, {
        let a2781 = v0;
        let a2782 = 57u64;
        nat_le(a2781, a2782)
    })
}

pub fn f400(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h2783, _) = v1.clone();
        h2783
    };
    let v3: u64 = {
        let (h2784, _) = {
            let (_, h2785) = v1.clone();
            h2785
        };
        h2784
    };
    let v4: List<Bytes> = {
        let (_, h2786) = {
            let (_, h2787) = v1;
            h2787
        };
        h2786
    };
    let m2788 = f146(v0.clone(), v2);
    match m2788 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a2789 = v5;
            let a2790 = 10u8;
            if equal(a2789, a2790) {
                Ok(Some(({
                    let a2791 = v2;
                    let a2792 = 1u64;
                    nat_add(a2791, a2792)?
                }, ({
                    let a2793 = v2;
                    let a2794 = 1u64;
                    nat_add(a2793, a2794)?
                }, List::cons(f502(f43(v0, v3, {
                    let a2795 = v2;
                    let a2796 = v3;
                    nat_sub(a2795, a2796)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a2797 = v2;
                    let a2798 = 1u64;
                    nat_add(a2797, a2798)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f401(v0: List<Adt25>, v1: List<Adt25>) -> List<Adt25> {
    let m2799 = v0;
    match m2799.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f401(v3, v1))
        }
    }
}

pub fn f402(v0: List<Adt26>, v1: Bytes) -> List<Adt25> {
    let m2800 = v0;
    match m2800.uncons() {
        None => {
            List::<Adt25>::nil()
        }
        Some((v2, v3)) => {
            if f167(match v2.clone() {
                Adt26::C0(_, h2801, _, _, _) => {
                    h2801
                }
            }, f503()) {
                let m2802 = f310(v2.clone(), Str::lit("name"));
                match m2802 {
                    None => {
                        f402(v3, v1)
                    }
                    Some(v4) => {
                        List::cons(Adt25::C0(match v2.clone() {
                            Adt26::C0(_, h2803, _, _, _) => {
                                h2803
                            }
                        }, v1.clone(), v4, v2), f402(v3, v1))
                    }
                }
            } else {
                f402(v3, v1)
            }
        }
    }
}

pub fn f403(v0: u64, v1: Adt24, v2: Adt25) -> R<Result<List<Adt27>, Bytes>> {
    let v3: Bytes = f200(match v2.clone() {
        Adt25::C0(_, _, h2804, _) => {
            h2804
        }
    })?;
    let v4: Adt26 = match v2.clone() {
        Adt25::C0(_, _, _, h2805) => {
            h2805
        }
    };
    let v5: Bytes = match v2.clone() {
        Adt25::C0(_, h2806, _, _) => {
            h2806
        }
    };
    let v6: Bytes = match v2.clone() {
        Adt25::C0(h2807, _, _, _) => {
            h2807
        }
    };
    if f46(v6.clone(), f8(Str::lit("complexType"))) {
        f504(v0, v1, v2)
    } else {
        if f46(v6.clone(), f8(Str::lit("simpleType"))) {
            let m2808 = f505(v3, v4, v5)?;
            match m2808 {
                Ok(v8) => {
                    Ok(Ok::<List<Adt27>, Bytes>(List::cons(v8, List::<Adt27>::nil())))
                }
                Err(v7) => {
                    Ok(Err::<List<Adt27>, Bytes>(v7))
                }
            }
        } else {
            if f46(v6.clone(), f8(Str::lit("element"))) {
                let m2809 = f310(v4.clone(), Str::lit("type"));
                match m2809 {
                    None => {
                        let m2810 = f506(v0, v1, v3, v5, v4)?;
                        match m2810 {
                            Ok(v11) => {
                                Ok(Ok::<List<Adt27>, Bytes>(match v11 {
                                    Adt32::C0(_, h2811) => {
                                        h2811
                                    }
                                }))
                            }
                            Err(v10) => {
                                Ok(Err::<List<Adt27>, Bytes>(v10))
                            }
                        }
                    }
                    Some(_) => {
                        Ok(Ok::<List<Adt27>, Bytes>(List::<Adt27>::nil()))
                    }
                }
            } else {
                if f46(v6.clone(), f8(Str::lit("attribute"))) {
                    let m2812 = f310(v4.clone(), Str::lit("type"));
                    match m2812 {
                        None => {
                            let m2813 = f507(v1, v5, v3, v4)?;
                            match m2813 {
                                Ok(v14) => {
                                    Ok(Ok::<List<Adt27>, Bytes>(match v14 {
                                        Adt32::C0(_, h2814) => {
                                            h2814
                                        }
                                    }))
                                }
                                Err(v13) => {
                                    Ok(Err::<List<Adt27>, Bytes>(v13))
                                }
                            }
                        }
                        Some(_) => {
                            Ok(Ok::<List<Adt27>, Bytes>(List::<Adt27>::nil()))
                        }
                    }
                } else {
                    if f46(v6, f8(Str::lit("group"))) {
                        let m2815 = f508(v4)?;
                        match m2815 {
                            Ok(v16) => {
                                let m2816 = f509(v16, List::cons(f8(Str::lit("sequence")), List::cons(f8(Str::lit("choice")), List::cons(f8(Str::lit("all")), List::<Bytes>::nil()))));
                                match m2816 {
                                    None => {
                                        Ok(Err::<List<Adt27>, Bytes>(f510(f8(Str::lit("a model group definition must have a sequence, choice or all child (XSD 1.1 Part 1, section 3.7.2)")), v3)))
                                    }
                                    Some(v17) => {
                                        f511(v0, v1, v3, v5, v17)
                                    }
                                }
                            }
                            Err(v15) => {
                                Ok(Err::<List<Adt27>, Bytes>(v15))
                            }
                        }
                    } else {
                        let m2817 = f508(v4)?;
                        match m2817 {
                            Ok(v19) => {
                                let m2818 = f512(v0, v1, v3, v5, v19, f513())?;
                                match m2818 {
                                    Ok(v21) => {
                                        Ok(Ok::<List<Adt27>, Bytes>(match v21 {
                                            Adt33::C0(_, h2819, _, _, _) => {
                                                h2819
                                            }
                                        }))
                                    }
                                    Err(v20) => {
                                        Ok(Err::<List<Adt27>, Bytes>(v20))
                                    }
                                }
                            }
                            Err(v18) => {
                                Ok(Err::<List<Adt27>, Bytes>(v18))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f404(v0: List<Adt27>, v1: List<Adt27>) -> List<Adt27> {
    let m2820 = v0;
    match m2820.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f404(v3, v1))
        }
    }
}

pub fn f405(p2823: List<Adt27>, p2824: List<Adt27>) -> List<Adt27> {
    let m2822 = (p2823, p2824);
    {
        let mut m2822 = m2822;
        loop {
            let step = {
                let (v0, v1) = m2822;
                let m2821 = v0;
                match m2821.uncons() {
                    None => {
                        Err::<(List<Adt27>, List<Adt27>), List<Adt27>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt27>, List<Adt27>), List<Adt27>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2822 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f406(p2828: List<Adt27>, v1: Bytes) -> Option<Adt27> {
    let m2827 = p2828;
    {
        let mut m2827 = m2827;
        loop {
            let step = {
                let v0 = m2827;
                let m2825 = v0;
                match m2825.uncons() {
                    None => {
                        Err::<List<Adt27>, Option<Adt27>>(None::<Adt27>)
                    }
                    Some((v2, v3)) => {
                        if f46(match v2.clone() {
                            Adt27::C0(h2826, _, _, _) => {
                                h2826
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
                Ok(next) => { m2827 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f407() -> Bytes {
    f8(Str::lit("XmlNode"))
}

pub fn f408(v0: List<Adt27>) -> List<Bytes> {
    let m2829 = v0;
    match m2829.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f414(match v1 {
                Adt27::C0(_, h2830, _, _) => {
                    h2830
                }
            }, f408(v2))
        }
    }
}

pub fn f409() -> List<Bytes> {
    List::cons(f8(Str::lit("    {name: element, fields: [Bytes, Bytes, (List ((Bytes * Bytes) * Bytes)), (List XmlNode)]}")), List::cons(f8(Str::lit("    {name: text, fields: [Bytes]}")), List::<Bytes>::nil()))
}

pub fn f410(p2834: u64, v1: List<Adt27>, p2835: List<Bytes>, p2836: List<Bytes>) -> List<Bytes> {
    let m2833 = (p2834, (p2835, p2836));
    {
        let mut m2833 = m2833;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2833;
                let m2831 = v0;
                if m2831 == 0 {
                    Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v3)
                } else {
                    let v4 = m2831 - 1;
                    let m2832 = v2;
                    match m2832.uncons() {
                        None => {
                            Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v3)
                        }
                        Some((v5, v6)) => {
                            if f167(v5.clone(), v3.clone()) {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v3)))
                            } else {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (f414(f411(v1.clone(), v5.clone()), v6), List::cons(v5, v3))))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2833 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f411(v0: List<Adt27>, v1: Bytes) -> List<Bytes> {
    let m2837 = f406(v0, v1);
    match m2837 {
        None => {
            List::<Bytes>::nil()
        }
        Some(v2) => {
            match v2 {
                Adt27::C0(_, h2838, _, _) => {
                    h2838
                }
            }
        }
    }
}

pub fn f412(v0: List<(List<Bytes>, List<Bytes>)>, v1: List<Bytes>) -> Option<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)> {
    let m2839 = v0;
    match m2839.uncons() {
        None => {
            None::<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)>
        }
        Some((v2, v3)) => {
            let m2841 = f201({
                let (_, h2840) = v2.clone();
                h2840
            }, v1.clone());
            match m2841 {
                None => {
                    Some(({
                        let (h2842, _) = v2;
                        h2842
                    }, v3))
                }
                Some(_) => {
                    let m2843 = f412(v3, v1);
                    match m2843 {
                        None => {
                            None::<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h2844, _) = v5;
                                h2844
                            }, List::cons(v2, {
                                let (_, h2845) = v5;
                                h2845
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f413(v0: List<Adt27>, v1: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m2846 = v1.clone();
    match m2846.uncons() {
        None => {
            Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m2847 = v3;
            match m2847.uncons() {
                None => {
                    let m2850 = f406(v0, v2.clone());
                    match m2850 {
                        None => {
                            Err::<List<Bytes>, Bytes>(f515(v2))
                        }
                        Some(v6) => {
                            if f167(v2.clone(), match v6.clone() {
                                Adt27::C0(_, h2851, _, _) => {
                                    h2851
                                }
                            }) {
                                let m2853 = match v6.clone() {
                                    Adt27::C0(_, _, _, h2852) => {
                                        h2852
                                    }
                                };
                                match m2853.uncons() {
                                    None => {
                                        Err::<List<Bytes>, Bytes>(f515(v2))
                                    }
                                    Some((_, _)) => {
                                        Ok::<List<Bytes>, Bytes>(List::cons(f269(v2, match v6 {
                                            Adt27::C0(_, _, _, h2854) => {
                                                h2854
                                            }
                                        }), List::<Bytes>::nil()))
                                    }
                                }
                            } else {
                                Ok::<List<Bytes>, Bytes>(List::cons(match v6 {
                                    Adt27::C0(_, _, h2855, _) => {
                                        h2855
                                    }
                                }, List::<Bytes>::nil()))
                            }
                        }
                    }
                }
                Some((_, _)) => {
                    f514(v0, v1, {
                        let a2848 = v2;
                        let a2849 = f8(Str::lit("Group"));
                        append_bytes(a2848, a2849)
                    })
                }
            }
        }
    }
}

pub fn f414(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m2856 = v0;
    match m2856.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f414(v3, v1))
        }
    }
}

pub fn f415(v0: List<Adt27>, v1: List<Bytes>, v2: List<Bytes>) -> List<Bytes> {
    let m2857 = v1;
    match m2857.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            f414(f516(f411(v0.clone(), v3), v2.clone()), f415(v0, v4, v2))
        }
    }
}

pub fn f416(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>, v2: Bytes) -> List<Bytes> {
    let m2858 = v1;
    match m2858.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if f46(v3.clone(), v2.clone()) {
                true
            } else {
                if f167(v3.clone(), f517(v0.clone(), v2.clone())) {
                    f167(v2.clone(), f517(v0.clone(), v3.clone()))
                } else {
                    false
                }
            } {
                List::cons(v3, f416(v0, v4, v2))
            } else {
                f416(v0, v4, v2)
            }
        }
    }
}

pub fn f417(v0: Bytes, p2867: u64, p2868: u64, p2869: Option<u64>) -> R<Option<u64>> {
    let m2866 = (p2867, (p2868, p2869));
    {
        let mut m2866 = m2866;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2866;
                let m2859 = v2;
                if m2859 == 0 {
                    Err::<(u64, (u64, Option<u64>)), R<Option<u64>>>(Ok(v3))
                } else {
                    let v4 = m2859 - 1;
                    let v5: u64 = f121(v0.clone(), v1)?;
                    let a2860 = v5;
                    let a2861 = 256u64;
                    if nat_eq(a2860, a2861) {
                        Err::<(u64, (u64, Option<u64>)), R<Option<u64>>>(Ok(v3))
                    } else {
                        Ok::<(u64, (u64, Option<u64>)), R<Option<u64>>>(({
                            let a2862 = v1;
                            let a2863 = 1u64;
                            nat_add(a2862, a2863)?
                        }, (v4, {
                            let a2864 = v5;
                            let a2865 = 47u64;
                            if nat_eq(a2864, a2865) {
                                Some(v1)
                            } else {
                                v3
                            }
                        })))
                    }
                }
            };
            match step {
                Ok(next) => { m2866 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f418(v0: Bytes, p2888: u64, p2889: u64, p2890: Bytes) -> R<Bytes> {
    let m2887 = (p2888, (p2889, p2890));
    {
        let mut m2887 = m2887;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2887;
                let m2870 = v2;
                if m2870 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m2870 - 1;
                    let v5: u64 = f138(v0.clone(), v1, {
                        let a2874 = {
                            let a2872 = {
                                let a2871 = v0.clone();
                                length_bytes(a2871)
                            };
                            let a2873 = v1;
                            nat_sub(a2872, a2873)
                        };
                        let a2875 = 1u64;
                        nat_add(a2874, a2875)?
                    })?;
                    let v6: Bytes = {
                        let a2878 = v3;
                        let a2879 = f43(v0.clone(), v1, {
                            let a2876 = v5;
                            let a2877 = v1;
                            nat_sub(a2876, a2877)
                        });
                        append_bytes(a2878, a2879)
                    };
                    let m2882 = {
                        let a2880 = v0.clone();
                        let a2881 = v5;
                        index_bytes(a2880, a2881)
                    };
                    match m2882 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v6))
                        }
                        Some(v7) => {
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                let a2883 = v5;
                                let a2884 = 1u64;
                                nat_add(a2883, a2884)?
                            }, (v4, {
                                let a2885 = v6;
                                let a2886 = f518(v0.clone(), v5, v7)?;
                                append_bytes(a2885, a2886)
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2887 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f419(v0: Fn6, p2894: List<(Bytes, Bytes)>, p2895: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m2893 = (p2894, p2895);
    {
        let mut m2893 = m2893;
        loop {
            let step = {
                let (v1, v2) = m2893;
                let m2891 = v2;
                match m2891.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c2892: Fn6 = v0.clone();
                            c2892.apply(v1, v3)
                        }, v4))
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

pub fn f420(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f519(v0, v1)
}

pub fn f421(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f520(v0)
}

pub fn f422(v0: Fn7, p2899: List<Bytes>, p2900: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m2898 = (p2899, p2900);
    {
        let mut m2898 = m2898;
        loop {
            let step = {
                let (v1, v2) = m2898;
                let m2896 = v2;
                match m2896.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c2897: Fn7 = v0.clone();
                            c2897.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2898 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f423(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f521(v1)?, v0))
}

pub fn f424(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2901 = v0.clone();
        length_bytes(a2901)
    };
    let a2905 = {
        let a2902 = v1;
        let a2903 = 2u64;
        let a2904 = 0u64;
        nat_rem(a2902, a2903, a2904)
    };
    let a2906 = 0u64;
    if nat_eq(a2905, a2906) {
        let (_, h2907) = {
            let (h2908, _) = f522(Fn8::F523(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2908
        };
        Ok(h2907)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f425(v0: u64) -> bool {
    let a2909 = v0;
    let a2910 = 8u64;
    if nat_eq(a2909, a2910) {
        true
    } else {
        let a2911 = v0;
        let a2912 = 9u64;
        if nat_eq(a2911, a2912) {
            true
        } else {
            let a2913 = v0;
            let a2914 = 10u64;
            if nat_eq(a2913, a2914) {
                true
            } else {
                let a2915 = v0;
                let a2916 = 12u64;
                if nat_eq(a2915, a2916) {
                    true
                } else {
                    let a2917 = v0;
                    let a2918 = 13u64;
                    nat_eq(a2917, a2918)
                }
            }
        }
    }
}

pub fn f426(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2921 = {
        let a2919 = v0;
        let a2920 = v1;
        index_bytes(a2919, a2920)
    };
    match m2921 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f524(v2)? {
                Ok(Some({
                    let a2922 = v1;
                    let a2923 = 1u64;
                    nat_add(a2922, a2923)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f427(v0: u8) -> R<u64> {
    let a2970 = {
        let v1: u8 = v0;
        let a2968 = {
            let a2944 = {
                let a2932 = {
                    let a2926 = {
                        let a2924 = v1;
                        let a2925 = 128u8;
                        fixed_u8::bit_and(a2924, a2925)
                    };
                    let a2927 = 128u8;
                    if equal(a2926, a2927) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2933 = {
                    let a2930 = {
                        let a2928 = v1;
                        let a2929 = 64u8;
                        fixed_u8::bit_and(a2928, a2929)
                    };
                    let a2931 = 64u8;
                    if equal(a2930, a2931) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2932, a2933)?
            };
            let a2945 = {
                let a2942 = {
                    let a2936 = {
                        let a2934 = v1;
                        let a2935 = 32u8;
                        fixed_u8::bit_and(a2934, a2935)
                    };
                    let a2937 = 32u8;
                    if equal(a2936, a2937) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2943 = {
                    let a2940 = {
                        let a2938 = v1;
                        let a2939 = 16u8;
                        fixed_u8::bit_and(a2938, a2939)
                    };
                    let a2941 = 16u8;
                    if equal(a2940, a2941) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2942, a2943)?
            };
            nat_add(a2944, a2945)?
        };
        let a2969 = {
            let a2966 = {
                let a2954 = {
                    let a2948 = {
                        let a2946 = v1;
                        let a2947 = 8u8;
                        fixed_u8::bit_and(a2946, a2947)
                    };
                    let a2949 = 8u8;
                    if equal(a2948, a2949) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2955 = {
                    let a2952 = {
                        let a2950 = v1;
                        let a2951 = 4u8;
                        fixed_u8::bit_and(a2950, a2951)
                    };
                    let a2953 = 4u8;
                    if equal(a2952, a2953) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2954, a2955)?
            };
            let a2967 = {
                let a2964 = {
                    let a2958 = {
                        let a2956 = v1;
                        let a2957 = 2u8;
                        fixed_u8::bit_and(a2956, a2957)
                    };
                    let a2959 = 2u8;
                    if equal(a2958, a2959) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2965 = {
                    let a2962 = {
                        let a2960 = v1;
                        let a2961 = 1u8;
                        fixed_u8::bit_and(a2960, a2961)
                    };
                    let a2963 = 1u8;
                    if equal(a2962, a2963) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2964, a2965)?
            };
            nat_add(a2966, a2967)?
        };
        nat_add(a2968, a2969)?
    };
    let a2971 = 48u64;
    Ok(nat_sub(a2970, a2971))
}

pub fn f428(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a2972 = v0.clone();
        length_bytes(a2972)
    };
    let a2973 = v1;
    let a2974 = v2;
    if if nat_lt(a2973, a2974) {
        let a2979 = f85(v0, {
            let a2977 = v2;
            let a2978 = {
                let a2975 = v1;
                let a2976 = 1u64;
                nat_add(a2975, a2976)?
            };
            nat_sub(a2977, a2978)
        })?;
        let a2980 = 48u64;
        nat_eq(a2979, a2980)
    } else {
        false
    } {
        Ok(Some({
            let a2981 = v1;
            let a2982 = 1u64;
            nat_add(a2981, a2982)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f429(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2985 = {
        let a2983 = v0;
        let a2984 = v1;
        index_bytes(a2983, a2984)
    };
    match m2985 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f53(v2)?;
            if f87({
                let a2986 = 128u64;
                let a2987 = v3;
                nat_le(a2986, a2987)
            }, {
                let a2988 = v3;
                let a2989 = 191u64;
                nat_le(a2988, a2989)
            }) {
                Ok(Some({
                    let a2990 = v3;
                    let a2991 = 128u64;
                    nat_sub(a2990, a2991)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f430(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f87({
        let a2992 = v1;
        let a2993 = v0;
        nat_le(a2992, a2993)
    }, f87({
        let a2994 = v0;
        let a2995 = v2;
        nat_le(a2994, a2995)
    }, f141({
        let a2996 = v0;
        let a2997 = 55296u64;
        nat_lt(a2996, a2997)
    }, {
        let a2998 = 57343u64;
        let a2999 = v0;
        nat_lt(a2998, a2999)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f431(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f525(v0, v1)
}

pub fn f432(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f230(f154(v0, v1)?) {
        Ok(Some({
            let a3000 = v1;
            let a3001 = 1u64;
            nat_add(a3000, a3001)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f433(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f526(v0, v1)
}

pub fn f434(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a3002 = f154(v0, v1)?;
    let a3003 = 32u64;
    if nat_eq(a3002, a3003) {
        Ok(Some({
            let a3004 = v1;
            let a3005 = 1u64;
            nat_add(a3004, a3005)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f435(v0: u64) -> bool {
    let a3006 = v0;
    let a3007 = 44u64;
    if if if nat_eq(a3006, a3007) {
        true
    } else {
        let a3008 = v0;
        let a3009 = 91u64;
        nat_eq(a3008, a3009)
    } {
        true
    } else {
        let a3010 = v0;
        let a3011 = 93u64;
        if nat_eq(a3010, a3011) {
            true
        } else {
            let a3012 = v0;
            let a3013 = 123u64;
            nat_eq(a3012, a3013)
        }
    } {
        true
    } else {
        let a3014 = v0;
        let a3015 = 125u64;
        nat_eq(a3014, a3015)
    }
}

pub fn f436(v0: u64) -> u64 {
    let a3016 = v0;
    let a3017 = 45u64;
    if nat_eq(a3016, a3017) {
        1u64
    } else {
        let a3018 = v0;
        let a3019 = 43u64;
        if nat_eq(a3018, a3019) {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f437(v0: u64) -> u64 {
    let a3020 = 49u64;
    let a3021 = v0;
    if if nat_le(a3020, a3021) {
        let a3022 = v0;
        let a3023 = 57u64;
        nat_le(a3022, a3023)
    } else {
        false
    } {
        let a3024 = v0;
        let a3025 = 48u64;
        nat_sub(a3024, a3025)
    } else {
        0u64
    }
}

pub fn f438(v0: Bytes, v1: u64, v2: u64, v3: bool, v4: u64) -> R<Adt21> {
    let m3026 = f527(v0.clone(), v1, v2)?;
    match m3026 {
        Adt35::C0(v6, v7) => {
            let v8: u64 = v7;
            let v9: u64 = f528(v0.clone(), v8, v2)?;
            if f529(v0.clone(), v8, v2)? {
                let m3030 = f530(v0.clone(), v8, v2, v3, {
                    let a3028 = {
                        let a3027 = v0;
                        length_bytes(a3027)
                    };
                    let a3029 = 2u64;
                    nat_add(a3028, a3029)?
                }, Bytes::lit(&[]), v6)?;
                match m3030 {
                    Adt36::C0(v11, v12, v13) => {
                        Ok(Adt21::C0(Adt2::C5(f7(List::cons(v11, List::cons({
                            let a3031 = v4;
                            let a3032 = 1u64;
                            if nat_eq(a3031, a3032) {
                                Bytes::lit(&[])
                            } else {
                                Bytes::lit(&[0x0a])
                            }
                        }, List::cons({
                            let a3033 = v4;
                            let a3034 = 2u64;
                            if nat_eq(a3033, a3034) {
                                f440(v12)
                            } else {
                                Bytes::lit(&[])
                            }
                        }, List::<Bytes>::nil()))))), v13))
                    }
                    Adt36::C1(v10) => {
                        Ok(Adt21::C1(v10))
                    }
                }
            } else {
                Ok(Adt21::C0(Adt2::C5({
                    let a3035 = v4;
                    let a3036 = 2u64;
                    if nat_eq(a3035, a3036) {
                        f440(v6)
                    } else {
                        Bytes::lit(&[])
                    }
                }), {
                    let a3037 = f154(v0, v9)?;
                    let a3038 = 256u64;
                    if nat_eq(a3037, a3038) {
                        v9
                    } else {
                        v8
                    }
                }))
            }
        }
        Adt35::C1(v5) => {
            Ok(Adt21::C1(v5))
        }
    }
}

pub fn f439(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f531(v0, v1)
}

pub fn f440(v0: u64) -> Bytes {
    let m3039 = v0;
    if m3039 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m3039 - 1;
        let a3040 = Bytes::lit(&[0x0a]);
        let a3041 = f440(v1);
        append_bytes(a3040, a3041)
    }
}

pub fn f441(v0: Bytes, v1: u64) -> R<u64> {
    let (_, h3042) = {
        let (h3043, _) = f223(Fn5::F532(v0.clone()), {
            let a3045 = {
                let a3044 = v0;
                length_bytes(a3044)
            };
            let a3046 = 1u64;
            nat_add(a3045, a3046)?
        }, (v1, 0u64))?;
        h3043
    };
    Ok(h3042)
}

pub fn f442(v0: u64, v1: u64) -> u64 {
    let a3047 = v0;
    let a3048 = v1;
    if nat_lt(a3047, a3048) {
        v1
    } else {
        v0
    }
}

pub fn f443(v0: Bytes, v1: u64, v2: u64) -> R<Adt21> {
    if f444(v0.clone()) {
        Ok(Adt21::C0(Adt2::C0, v2))
    } else {
        if f448(v0.clone()) {
            Ok(Adt21::C0(Adt2::C2(true), v2))
        } else {
            if f449(v0.clone()) {
                Ok(Adt21::C0(Adt2::C2(false), v2))
            } else {
                if f447(v0.clone()) {
                    Ok(Adt21::C1(f98(Str::lit("an infinite or not-a-number float is not read: Studio.Value has none (YAML 1.2.2 section 10.3.2)"), v1)?))
                } else {
                    if if f120(v0.clone(), 0u64, Str::lit("0o")) {
                        let a3050 = 2u64;
                        let a3051 = {
                            let a3049 = v0.clone();
                            length_bytes(a3049)
                        };
                        nat_lt(a3050, a3051)
                    } else {
                        false
                    } {
                        f533(v0, 8u64, v1, v2)
                    } else {
                        if if f120(v0.clone(), 0u64, Str::lit("0x")) {
                            let a3053 = 2u64;
                            let a3054 = {
                                let a3052 = v0.clone();
                                length_bytes(a3052)
                            };
                            nat_lt(a3053, a3054)
                        } else {
                            false
                        } {
                            f533(v0, 16u64, v1, v2)
                        } else {
                            let m3055 = f446(v0.clone())?;
                            match m3055 {
                                None => {
                                    Ok(Adt21::C0(Adt2::C5(v0), v2))
                                }
                                Some(v3) => {
                                    let m3056 = f88(v3.clone(), 0u64)?;
                                    match m3056 {
                                        None => {
                                            Ok(Adt21::C1(f534(v1)?))
                                        }
                                        Some(v4) => {
                                            let a3059 = {
                                                let (h3057, _) = v4;
                                                h3057
                                            };
                                            let a3060 = {
                                                let a3058 = v3;
                                                length_bytes(a3058)
                                            };
                                            if nat_eq(a3059, a3060) {
                                                Ok(Adt21::C0({
                                                    let (_, h3061) = v4;
                                                    h3061
                                                }, v2))
                                            } else {
                                                Ok(Adt21::C1(f534(v1)?))
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

pub fn f444(v0: Bytes) -> bool {
    let a3063 = {
        let a3062 = v0.clone();
        length_bytes(a3062)
    };
    let a3064 = 0u64;
    if nat_eq(a3063, a3064) {
        true
    } else {
        f251(v0, List::cons(f8(Str::lit("~")), List::cons(f8(Str::lit("null")), List::cons(f8(Str::lit("Null")), List::cons(f8(Str::lit("NULL")), List::<Bytes>::nil())))))
    }
}

pub fn f445(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a3065 = v0.clone();
        length_bytes(a3065)
    };
    let v2: u64 = f154(v0.clone(), 0u64)?;
    let v3: u64 = {
        let a3066 = v2;
        let a3067 = 43u64;
        if if nat_eq(a3066, a3067) {
            true
        } else {
            let a3068 = v2;
            let a3069 = 45u64;
            nat_eq(a3068, a3069)
        } {
            1u64
        } else {
            0u64
        }
    };
    let a3070 = v3;
    let a3071 = v1;
    if if nat_lt(a3070, a3071) {
        let a3072 = f140(v0.clone(), v3)?;
        let a3073 = v1;
        nat_eq(a3072, a3073)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a3074 = 2u64;
        let a3075 = v1;
        if nat_lt(a3074, a3075) {
            if if f120(v0.clone(), 0u64, Str::lit("0o")) {
                let a3077 = {
                    let (h3076, _) = f535(v0.clone(), 8u64, 2u64)?;
                    h3076
                };
                let a3078 = v1;
                nat_eq(a3077, a3078)
            } else {
                false
            } {
                Ok(true)
            } else {
                if f120(v0.clone(), 0u64, Str::lit("0x")) {
                    let a3080 = {
                        let (h3079, _) = f535(v0, 16u64, 2u64)?;
                        h3079
                    };
                    let a3081 = v1;
                    Ok(nat_eq(a3080, a3081))
                } else {
                    Ok(false)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f446(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a3082 = v0.clone();
        length_bytes(a3082)
    };
    let v2: u64 = f154(v0.clone(), 0u64)?;
    let v3: u64 = {
        let a3083 = v2;
        let a3084 = 43u64;
        if if nat_eq(a3083, a3084) {
            true
        } else {
            let a3085 = v2;
            let a3086 = 45u64;
            nat_eq(a3085, a3086)
        } {
            1u64
        } else {
            0u64
        }
    };
    let v4: u64 = f140(v0.clone(), v3)?;
    let v5: bool = {
        let a3087 = f154(v0.clone(), v4)?;
        let a3088 = 46u64;
        nat_eq(a3087, a3088)
    };
    let v6: u64 = if v5 {
        let a3089 = v4;
        let a3090 = 1u64;
        nat_add(a3089, a3090)?
    } else {
        v4
    };
    let v7: u64 = if v5 {
        f140(v0.clone(), v6)?
    } else {
        v4
    };
    let v8: u64 = f154(v0.clone(), v7)?;
    let v9: bool = {
        let a3091 = v8;
        let a3092 = 101u64;
        if nat_eq(a3091, a3092) {
            true
        } else {
            let a3093 = v8;
            let a3094 = 69u64;
            nat_eq(a3093, a3094)
        }
    };
    let v10: u64 = f154(v0.clone(), {
        let a3095 = v7;
        let a3096 = 1u64;
        nat_add(a3095, a3096)?
    })?;
    let v11: bool = if v9 {
        let a3097 = v10;
        let a3098 = 43u64;
        if nat_eq(a3097, a3098) {
            true
        } else {
            let a3099 = v10;
            let a3100 = 45u64;
            nat_eq(a3099, a3100)
        }
    } else {
        false
    };
    let v12: u64 = if v9 {
        if v11 {
            let a3101 = v7;
            let a3102 = 2u64;
            nat_add(a3101, a3102)?
        } else {
            let a3103 = v7;
            let a3104 = 1u64;
            nat_add(a3103, a3104)?
        }
    } else {
        v7
    };
    let v13: u64 = if v9 {
        f140(v0.clone(), v12)?
    } else {
        v7
    };
    let v14: Bytes = f247(v0.clone(), v3, v4);
    let v15: u64 = f142(v14.clone())?;
    let a3105 = v13;
    let a3106 = v1;
    if if if nat_eq(a3105, a3106) {
        let a3107 = v3;
        let a3108 = v4;
        if nat_lt(a3107, a3108) {
            true
        } else {
            if v5 {
                let a3109 = v6;
                let a3110 = v7;
                nat_lt(a3109, a3110)
            } else {
                false
            }
        }
    } else {
        false
    } {
        if f156(v9) {
            true
        } else {
            let a3111 = v12;
            let a3112 = v13;
            nat_lt(a3111, a3112)
        }
    } else {
        false
    } {
        Ok(Some(f7(List::cons({
            let a3113 = v2;
            let a3114 = 45u64;
            if nat_eq(a3113, a3114) {
                Bytes::lit(&[0x2d])
            } else {
                Bytes::lit(&[])
            }
        }, List::cons({
            let a3116 = v15;
            let a3117 = {
                let a3115 = v14.clone();
                length_bytes(a3115)
            };
            if nat_eq(a3116, a3117) {
                Bytes::lit(&[0x30])
            } else {
                f247(v14.clone(), v15, {
                    let a3118 = v14;
                    length_bytes(a3118)
                })
            }
        }, List::cons({
            let a3119 = v6;
            let a3120 = v7;
            if nat_lt(a3119, a3120) {
                let a3121 = Bytes::lit(&[0x2e]);
                let a3122 = f247(v0.clone(), v6, v7);
                append_bytes(a3121, a3122)
            } else {
                Bytes::lit(&[])
            }
        }, List::cons(if v9 {
            let a3125 = if if v11 {
                let a3123 = v10;
                let a3124 = 45u64;
                nat_eq(a3123, a3124)
            } else {
                false
            } {
                Bytes::lit(&[0x65, 0x2d])
            } else {
                Bytes::lit(&[0x65])
            };
            let a3126 = f247(v0, v12, v13);
            append_bytes(a3125, a3126)
        } else {
            Bytes::lit(&[])
        }, List::<Bytes>::nil())))))))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f447(v0: Bytes) -> bool {
    f251(v0, List::cons(f8(Str::lit(".inf")), List::cons(f8(Str::lit(".Inf")), List::cons(f8(Str::lit(".INF")), List::cons(f8(Str::lit("+.inf")), List::cons(f8(Str::lit("+.Inf")), List::cons(f8(Str::lit("+.INF")), List::cons(f8(Str::lit("-.inf")), List::cons(f8(Str::lit("-.Inf")), List::cons(f8(Str::lit("-.INF")), List::cons(f8(Str::lit(".nan")), List::cons(f8(Str::lit(".NaN")), List::cons(f8(Str::lit(".NAN")), List::<Bytes>::nil())))))))))))))
}

pub fn f448(v0: Bytes) -> bool {
    f251(v0, List::cons(f8(Str::lit("true")), List::cons(f8(Str::lit("True")), List::cons(f8(Str::lit("TRUE")), List::<Bytes>::nil()))))
}

pub fn f449(v0: Bytes) -> bool {
    f251(v0, List::cons(f8(Str::lit("false")), List::cons(f8(Str::lit("False")), List::cons(f8(Str::lit("FALSE")), List::<Bytes>::nil()))))
}

pub fn f450(v0: Bytes, p3185: u64, v2: u64, v3: bool, p3186: u64, p3187: Bytes) -> R<Adt29> {
    let m3184 = (p3185, (p3186, p3187));
    {
        let mut m3184 = m3184;
        loop {
            let step = {
                let (v1, (v4, v5)) = m3184;
                let m3127 = v4;
                if m3127 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt29>>(Ok(Adt29::C1(f98(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                } else {
                    let v6 = m3127 - 1;
                    let v7: u64 = f154(v0.clone(), v1)?;
                    let a3128 = v7;
                    let a3129 = 34u64;
                    if nat_eq(a3128, a3129) {
                        Err::<(u64, (u64, Bytes)), R<Adt29>>(Ok(Adt29::C0(v5, {
                            let a3130 = v1;
                            let a3131 = 1u64;
                            nat_add(a3130, a3131)?
                        })))
                    } else {
                        let a3132 = v7;
                        let a3133 = 256u64;
                        if nat_eq(a3132, a3133) {
                            Err::<(u64, (u64, Bytes)), R<Adt29>>(Ok(Adt29::C1(f98(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                        } else {
                            let a3134 = v7;
                            let a3135 = 92u64;
                            if nat_eq(a3134, a3135) {
                                let v8: u64 = f154(v0.clone(), {
                                    let a3136 = v1;
                                    let a3137 = 1u64;
                                    nat_add(a3136, a3137)?
                                })?;
                                let m3138 = f536(v8)?;
                                match m3138 {
                                    None => {
                                        let a3143 = v8;
                                        let a3144 = 10u64;
                                        if nat_eq(a3143, a3144) {
                                            if v3 {
                                                Err::<(u64, (u64, Bytes)), R<Adt29>>(Ok(Adt29::C1(f537(v1)?)))
                                            } else {
                                                let m3147 = f538(v0.clone(), {
                                                    let a3145 = v1;
                                                    let a3146 = 1u64;
                                                    nat_add(a3145, a3146)?
                                                }, v2, v6, 0u64)?;
                                                match m3147 {
                                                    Adt35::C0(v11, v12) => {
                                                        Ok::<(u64, (u64, Bytes)), R<Adt29>>((v12, (v6, {
                                                            let a3148 = v5;
                                                            let a3149 = f440(v11);
                                                            append_bytes(a3148, a3149)
                                                        })))
                                                    }
                                                    Adt35::C1(v10) => {
                                                        Err::<(u64, (u64, Bytes)), R<Adt29>>(Ok(Adt29::C1(v10)))
                                                    }
                                                }
                                            }
                                        } else {
                                            let v13: u64 = {
                                                let a3150 = v8;
                                                let a3151 = 120u64;
                                                if nat_eq(a3150, a3151) {
                                                    2u64
                                                } else {
                                                    let a3152 = v8;
                                                    let a3153 = 117u64;
                                                    if nat_eq(a3152, a3153) {
                                                        4u64
                                                    } else {
                                                        let a3154 = v8;
                                                        let a3155 = 85u64;
                                                        if nat_eq(a3154, a3155) {
                                                            8u64
                                                        } else {
                                                            0u64
                                                        }
                                                    }
                                                }
                                            };
                                            let a3156 = v13;
                                            let a3157 = 0u64;
                                            if nat_eq(a3156, a3157) {
                                                Err::<(u64, (u64, Bytes)), R<Adt29>>(Ok(Adt29::C1(f98(Str::lit("an escape must be one of YAML 1.2.2 [42] to [62]"), v1)?)))
                                            } else {
                                                let m3160 = f539(v0.clone(), {
                                                    let a3158 = v1;
                                                    let a3159 = 2u64;
                                                    nat_add(a3158, a3159)?
                                                }, v13, 0u64)?;
                                                match m3160 {
                                                    None => {
                                                        Err::<(u64, (u64, Bytes)), R<Adt29>>(Ok(Adt29::C1(f98(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                    }
                                                    Some(v14) => {
                                                        let a3161 = 1114111u64;
                                                        let a3162 = v14;
                                                        if if nat_lt(a3161, a3162) {
                                                            true
                                                        } else {
                                                            let a3163 = 55296u64;
                                                            let a3164 = v14;
                                                            if nat_le(a3163, a3164) {
                                                                let a3165 = v14;
                                                                let a3166 = 57343u64;
                                                                nat_le(a3165, a3166)
                                                            } else {
                                                                false
                                                            }
                                                        } {
                                                            Err::<(u64, (u64, Bytes)), R<Adt29>>(Ok(Adt29::C1(f98(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                        } else {
                                                            Ok::<(u64, (u64, Bytes)), R<Adt29>>(({
                                                                let a3169 = {
                                                                    let a3167 = v1;
                                                                    let a3168 = 2u64;
                                                                    nat_add(a3167, a3168)?
                                                                };
                                                                let a3170 = v13;
                                                                nat_add(a3169, a3170)?
                                                            }, (v6, {
                                                                let a3171 = v5;
                                                                let a3172 = f391(v14)?;
                                                                append_bytes(a3171, a3172)
                                                            })))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Some(v9) => {
                                        Ok::<(u64, (u64, Bytes)), R<Adt29>>(({
                                            let a3139 = v1;
                                            let a3140 = 2u64;
                                            nat_add(a3139, a3140)?
                                        }, (v6, {
                                            let a3141 = v5;
                                            let a3142 = f391(v9)?;
                                            append_bytes(a3141, a3142)
                                        })))
                                    }
                                }
                            } else {
                                if if f230(v7) {
                                    true
                                } else {
                                    let a3173 = v7;
                                    let a3174 = 10u64;
                                    nat_eq(a3173, a3174)
                                } {
                                    let v15: u64 = f233(v0.clone(), v1)?;
                                    let a3175 = f154(v0.clone(), v15)?;
                                    let a3176 = 10u64;
                                    if nat_eq(a3175, a3176) {
                                        if v3 {
                                            Err::<(u64, (u64, Bytes)), R<Adt29>>(Ok(Adt29::C1(f537(v1)?)))
                                        } else {
                                            let m3177 = f538(v0.clone(), v15, v2, v6, 0u64)?;
                                            match m3177 {
                                                Adt35::C0(v17, v18) => {
                                                    Ok::<(u64, (u64, Bytes)), R<Adt29>>((v18, (v6, {
                                                        let a3178 = v5;
                                                        let a3179 = f458(v17);
                                                        append_bytes(a3178, a3179)
                                                    })))
                                                }
                                                Adt35::C1(v16) => {
                                                    Err::<(u64, (u64, Bytes)), R<Adt29>>(Ok(Adt29::C1(v16)))
                                                }
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (u64, Bytes)), R<Adt29>>((v15, (v6, {
                                            let a3180 = v5;
                                            let a3181 = f247(v0.clone(), v1, v15);
                                            append_bytes(a3180, a3181)
                                        })))
                                    }
                                } else {
                                    let v19: u64 = f540(v0.clone(), v1)?;
                                    Ok::<(u64, (u64, Bytes)), R<Adt29>>((v19, (v6, {
                                        let a3182 = v5;
                                        let a3183 = f247(v0.clone(), v1, v19);
                                        append_bytes(a3182, a3183)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3184 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f451(v0: Bytes, p3215: u64, v2: u64, v3: bool, p3216: u64, p3217: Bytes) -> R<Adt29> {
    let m3214 = (p3215, (p3216, p3217));
    {
        let mut m3214 = m3214;
        loop {
            let step = {
                let (v1, (v4, v5)) = m3214;
                let m3188 = v4;
                if m3188 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt29>>(Ok(Adt29::C1(f98(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                } else {
                    let v6 = m3188 - 1;
                    let v7: u64 = f154(v0.clone(), v1)?;
                    let a3189 = v7;
                    let a3190 = 39u64;
                    if nat_eq(a3189, a3190) {
                        let a3193 = f154(v0.clone(), {
                            let a3191 = v1;
                            let a3192 = 1u64;
                            nat_add(a3191, a3192)?
                        })?;
                        let a3194 = 39u64;
                        if nat_eq(a3193, a3194) {
                            Ok::<(u64, (u64, Bytes)), R<Adt29>>(({
                                let a3195 = v1;
                                let a3196 = 2u64;
                                nat_add(a3195, a3196)?
                            }, (v6, {
                                let a3197 = v5;
                                let a3198 = Bytes::lit(&[0x27]);
                                append_bytes(a3197, a3198)
                            })))
                        } else {
                            Err::<(u64, (u64, Bytes)), R<Adt29>>(Ok(Adt29::C0(v5, {
                                let a3199 = v1;
                                let a3200 = 1u64;
                                nat_add(a3199, a3200)?
                            })))
                        }
                    } else {
                        let a3201 = v7;
                        let a3202 = 256u64;
                        if nat_eq(a3201, a3202) {
                            Err::<(u64, (u64, Bytes)), R<Adt29>>(Ok(Adt29::C1(f98(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                        } else {
                            if if f230(v7) {
                                true
                            } else {
                                let a3203 = v7;
                                let a3204 = 10u64;
                                nat_eq(a3203, a3204)
                            } {
                                let v8: u64 = f233(v0.clone(), v1)?;
                                let a3205 = f154(v0.clone(), v8)?;
                                let a3206 = 10u64;
                                if nat_eq(a3205, a3206) {
                                    if v3 {
                                        Err::<(u64, (u64, Bytes)), R<Adt29>>(Ok(Adt29::C1(f537(v1)?)))
                                    } else {
                                        let m3207 = f538(v0.clone(), v8, v2, v6, 0u64)?;
                                        match m3207 {
                                            Adt35::C0(v10, v11) => {
                                                Ok::<(u64, (u64, Bytes)), R<Adt29>>((v11, (v6, {
                                                    let a3208 = v5;
                                                    let a3209 = f458(v10);
                                                    append_bytes(a3208, a3209)
                                                })))
                                            }
                                            Adt35::C1(v9) => {
                                                Err::<(u64, (u64, Bytes)), R<Adt29>>(Ok(Adt29::C1(v9)))
                                            }
                                        }
                                    }
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Adt29>>((v8, (v6, {
                                        let a3210 = v5;
                                        let a3211 = f247(v0.clone(), v1, v8);
                                        append_bytes(a3210, a3211)
                                    })))
                                }
                            } else {
                                let v12: u64 = f541(v0.clone(), v1)?;
                                Ok::<(u64, (u64, Bytes)), R<Adt29>>((v12, (v6, {
                                    let a3212 = v5;
                                    let a3213 = f247(v0.clone(), v1, v12);
                                    append_bytes(a3212, a3213)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3214 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f452(v0: Bytes, v1: u64, v2: u64) -> R<Result<u64, Bytes>> {
    let v3: (u64, bool) = {
        let (h3218, _) = f542(Fn9::F543(v2, v0.clone()), {
            let a3220 = {
                let a3219 = v0.clone();
                length_bytes(a3219)
            };
            let a3221 = 1u64;
            nat_add(a3220, a3221)?
        }, (v1, false))?;
        h3218
    };
    let (_, h3222) = v3;
    if h3222 {
        if f152(v0, {
            let (h3223, _) = v3;
            h3223
        })? {
            Ok(Err::<u64, Bytes>(f98(Str::lit("a document marker cannot appear inside a flow collection (YAML 1.2.2 [206])"), {
                let (h3224, _) = v3;
                h3224
            })?))
        } else {
            Ok(Err::<u64, Bytes>(f98(Str::lit("a line of a flow collection is indented less than its node (YAML 1.2.2 [80])"), {
                let (h3225, _) = v3;
                h3225
            })?))
        }
    } else {
        Ok(Ok::<u64, Bytes>({
            let (h3226, _) = v3;
            h3226
        }))
    }
}

pub fn f453(v0: Bytes, v1: u64, v2: u64) -> R<Adt29> {
    let a3227 = f154(v0.clone(), v1)?;
    let a3228 = 33u64;
    if nat_eq(a3227, a3228) {
        let m3229 = f248(v0.clone(), v1)?;
        match m3229 {
            Ok(v4) => {
                let m3230 = f452(v0.clone(), v4, v2)?;
                match m3230 {
                    Ok(v6) => {
                        let a3231 = f154(v0.clone(), v6)?;
                        let a3232 = 58u64;
                        if nat_eq(a3231, a3232) {
                            let a3235 = f243(v0, v1)?;
                            let a3236 = {
                                let a3233 = v1;
                                let a3234 = 1u64;
                                nat_add(a3233, a3234)?
                            };
                            if nat_eq(a3235, a3236) {
                                Ok(Adt29::C1(f98(Str::lit("an empty node with the non-specific tag ! is not read (YAML 1.2.2 [100])"), v1)?))
                            } else {
                                Ok(Adt29::C0(Bytes::lit(&[]), v6))
                            }
                        } else {
                            f544(v0, v6, v2)
                        }
                    }
                    Err(v5) => {
                        Ok(Adt29::C1(v5))
                    }
                }
            }
            Err(v3) => {
                Ok(Adt29::C1(v3))
            }
        }
    } else {
        f544(v0, v1, v2)
    }
}

pub fn f454(v0: u64) -> R<bool> {
    let a3237 = f246(f8(Str::lit(",[]{}-?:#&*!|>'\"@`")), v0)?;
    let a3238 = 18u64;
    if nat_lt(a3237, a3238) {
        Ok(true)
    } else {
        let a3239 = v0;
        let a3240 = 37u64;
        Ok(nat_eq(a3239, a3240))
    }
}

pub fn f455(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f545(v1, v0, v2)
}

pub fn f456(v0: u64) -> R<Bytes> {
    f98(Str::lit("a mapping value is not allowed here: a plain scalar holds ': ' (YAML 1.2.2 [130])"), v0)
}

pub fn f457(v0: Bytes, v1: u64) -> R<(u64, u64)> {
    let (h3241, _) = f223(Fn5::F546(v0.clone()), {
        let a3243 = {
            let a3242 = v0;
            length_bytes(a3242)
        };
        let a3244 = 1u64;
        nat_add(a3243, a3244)?
    }, (v1, 0u64))?;
    Ok(h3241)
}

pub fn f458(v0: u64) -> Bytes {
    let a3245 = v0;
    let a3246 = 0u64;
    if nat_eq(a3245, a3246) {
        Bytes::lit(&[0x20])
    } else {
        f440(v0)
    }
}

pub fn f459(v0: Bytes, v1: bool, v2: u64) -> R<Option<u64>> {
    f547(v0, v1, v2)
}

pub fn f460(v0: Bytes, p3251: List<Bytes>, p3252: u64) -> R<u64> {
    let m3250 = (p3251, p3252);
    {
        let mut m3250 = m3250;
        loop {
            let step = {
                let (v1, v2) = m3250;
                let m3247 = v1;
                match m3247.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<u64>>(Ok(0u64))
                    }
                    Some((v3, v4)) => {
                        if f46(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<u64>>(Ok(v2))
                        } else {
                            Ok::<(List<Bytes>, u64), R<u64>>((v4, {
                                let a3248 = v2;
                                let a3249 = 1u64;
                                nat_add(a3248, a3249)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3250 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f461(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a3254 = v2;
    let a3255 = {
        let a3253 = v0.clone();
        length_bytes(a3253)
    };
    if if nat_lt(a3254, a3255) {
        f156({
            let a3256 = f154(v0, v2)?;
            let a3257 = v1;
            nat_eq(a3256, a3257)
        })
    } else {
        false
    } {
        Ok(Some({
            let a3258 = v2;
            let a3259 = 1u64;
            nat_add(a3258, a3259)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f462(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f548(v1, v0, v2)
}

pub fn f463(v0: u64) -> bool {
    if f464(v0) {
        true
    } else {
        let a3260 = 65u64;
        let a3261 = v0;
        if if nat_le(a3260, a3261) {
            let a3262 = v0;
            let a3263 = 70u64;
            nat_le(a3262, a3263)
        } else {
            false
        } {
            true
        } else {
            let a3264 = 97u64;
            let a3265 = v0;
            if nat_le(a3264, a3265) {
                let a3266 = v0;
                let a3267 = 102u64;
                nat_le(a3266, a3267)
            } else {
                false
            }
        }
    }
}

pub fn f464(v0: u64) -> bool {
    let a3268 = 48u64;
    let a3269 = v0;
    if nat_le(a3268, a3269) {
        let a3270 = v0;
        let a3271 = 57u64;
        nat_le(a3270, a3271)
    } else {
        false
    }
}

pub fn f465(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f121(v0.clone(), v1)?;
    let a3272 = v2;
    let a3273 = 69u64;
    if if nat_eq(a3272, a3273) {
        true
    } else {
        let a3274 = v2;
        let a3275 = 101u64;
        nat_eq(a3274, a3275)
    } {
        let v3: u64 = f121(v0.clone(), {
            let a3276 = v1;
            let a3277 = 1u64;
            nat_add(a3276, a3277)?
        })?;
        let v4: u64 = {
            let a3278 = v3;
            let a3279 = 43u64;
            if if nat_eq(a3278, a3279) {
                true
            } else {
                let a3280 = v3;
                let a3281 = 45u64;
                nat_eq(a3280, a3281)
            } {
                let a3282 = v1;
                let a3283 = 2u64;
                nat_add(a3282, a3283)?
            } else {
                let a3284 = v1;
                let a3285 = 1u64;
                nat_add(a3284, a3285)?
            }
        };
        let v5: u64 = f352(v0, 0u64, v4)?;
        let a3286 = v4;
        let a3287 = v5;
        if nat_lt(a3286, a3287) {
            Ok(v5)
        } else {
            Ok(v1)
        }
    } else {
        Ok(v1)
    }
}

pub fn f466(v0: Adt22, v1: Bytes) -> bool {
    let a3288 = f467(v0.clone());
    let a3289 = 3u64;
    if !nat_eq(a3288, a3289) {
        f46(f468(v0), v1)
    } else {
        false
    }
}

pub fn f467(v0: Adt22) -> u64 {
    let m3290 = v0;
    match m3290 {
        Adt22::C0(v1, _) => {
            v1
        }
    }
}

pub fn f468(v0: Adt22) -> Bytes {
    let m3291 = v0;
    match m3291 {
        Adt22::C0(_, v2) => {
            v2
        }
    }
}

pub fn f469(v0: Bytes) -> Bytes {
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

pub fn f470(v0: Bytes) -> bool {
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

pub fn f471(p3300: u64, p3301: List<Adt22>, p3302: List<Adt14>) -> R<Result<(List<Adt14>, List<Adt22>), Bytes>> {
    let m3299 = (p3300, (p3301, p3302));
    {
        let mut m3299 = m3299;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3299;
                let m3292 = v0;
                if m3292 == 0 {
                    Err::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt22>), Bytes>(f8(Str::lit("a dictionary has more members than the input has tokens")))))
                } else {
                    let v3 = m3292 - 1;
                    if f355(v1.clone(), Str::lit("}")) {
                        let m3293 = f362(f357(v1), Str::lit(";"));
                        match m3293 {
                            Ok(v5) => {
                                Err::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>(Ok(Ok::<(List<Adt14>, List<Adt22>), Bytes>((f549(v2, List::<Adt14>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt22>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m3294 = f255(v1)?;
                        match m3294 {
                            Ok(v7) => {
                                let v8: bool = f355(v7.clone(), Str::lit("required"));
                                let m3295 = f367(v3, if v8 {
                                    f357(v7)
                                } else {
                                    v7
                                }, 0u64)?;
                                match m3295 {
                                    Ok(v10) => {
                                        let m3296 = f361(f368(v10.clone()));
                                        match m3296 {
                                            Ok(v12) => {
                                                if if f355(f363(v12.clone()), Str::lit("=")) {
                                                    !v8
                                                } else {
                                                    false
                                                } {
                                                    let m3297 = f473(f357(f363(v12.clone())))?;
                                                    match m3297 {
                                                        Ok(v14) => {
                                                            Ok::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>((v3, (v14, List::cons(Adt14::C0(f365(v12), f480(f369(v10)), 0u64), v2))))
                                                        }
                                                        Err(v13) => {
                                                            Err::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt22>), Bytes>(v13)))
                                                        }
                                                    }
                                                } else {
                                                    let m3298 = f362(f363(v12.clone()), Str::lit(";"));
                                                    match m3298 {
                                                        Ok(v16) => {
                                                            Ok::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>((v3, (v16, List::cons(Adt14::C0(f365(v12), f480(f369(v10)), if v8 {
                                                                0u64
                                                            } else {
                                                                1u64
                                                            }), v2))))
                                                        }
                                                        Err(v15) => {
                                                            Err::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt22>), Bytes>(v15)))
                                                        }
                                                    }
                                                }
                                            }
                                            Err(v11) => {
                                                Err::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt22>), Bytes>(v11)))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Err::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt22>), Bytes>(v9)))
                                    }
                                }
                            }
                            Err(v6) => {
                                Err::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt22>), Bytes>(v6)))
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

pub fn f472(p3311: u64, p3312: List<Adt22>, p3313: List<Adt14>) -> R<Result<(List<Adt14>, List<Adt22>), Bytes>> {
    let m3310 = (p3311, (p3312, p3313));
    {
        let mut m3310 = m3310;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3310;
                let m3303 = v0;
                if m3303 == 0 {
                    Err::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt22>), Bytes>(f8(Str::lit("an interface has more members than the input has tokens")))))
                } else {
                    let v3 = m3303 - 1;
                    if f355(v1.clone(), Str::lit("}")) {
                        let m3304 = f362(f357(v1), Str::lit(";"));
                        match m3304 {
                            Ok(v5) => {
                                Err::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>(Ok(Ok::<(List<Adt14>, List<Adt22>), Bytes>((f549(v2, List::<Adt14>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt22>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m3305 = f255(v1)?;
                        match m3305 {
                            Ok(v7) => {
                                let v8: List<Adt22> = if if f355(v7.clone(), Str::lit("stringifier")) {
                                    true
                                } else {
                                    f355(v7.clone(), Str::lit("inherit"))
                                } {
                                    f357(v7.clone())
                                } else {
                                    v7.clone()
                                };
                                let v9: List<Adt22> = if f355(v8.clone(), Str::lit("readonly")) {
                                    f357(v8)
                                } else {
                                    v8
                                };
                                if f355(v9.clone(), Str::lit("attribute")) {
                                    let m3306 = f367(v3, f357(v9), 0u64)?;
                                    match m3306 {
                                        Ok(v11) => {
                                            let m3307 = f361(f368(v11.clone()));
                                            match m3307 {
                                                Ok(v13) => {
                                                    let m3308 = f362(f363(v13.clone()), Str::lit(";"));
                                                    match m3308 {
                                                        Ok(v15) => {
                                                            Ok::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>((v3, (v15, List::cons(Adt14::C0(f365(v13), f480(f369(v11)), 0u64), v2))))
                                                        }
                                                        Err(v14) => {
                                                            Err::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt22>), Bytes>(v14)))
                                                        }
                                                    }
                                                }
                                                Err(v12) => {
                                                    Err::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt22>), Bytes>(v12)))
                                                }
                                            }
                                        }
                                        Err(v10) => {
                                            Err::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt22>), Bytes>(v10)))
                                        }
                                    }
                                } else {
                                    let m3309 = f473(v7)?;
                                    match m3309 {
                                        Ok(v17) => {
                                            Ok::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>((v3, (v17, v2)))
                                        }
                                        Err(v16) => {
                                            Err::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt22>), Bytes>(v16)))
                                        }
                                    }
                                }
                            }
                            Err(v6) => {
                                Err::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt22>), Bytes>(v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3310 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f473(v0: List<Adt22>) -> R<Result<List<Adt22>, Bytes>> {
    Ok(f356({
        let a3315 = {
            let a3314 = v0.clone();
            length_list(a3314)
        };
        let a3316 = 1u64;
        nat_add(a3315, a3316)?
    }, v0, List::<Bytes>::nil(), true))
}

pub fn f474(v0: List<Adt22>) -> Bytes {
    let m3317 = v0;
    match m3317.uncons() {
        None => {
            f8(Str::lit("the end of the input"))
        }
        Some((v1, _)) => {
            f7(List::cons(f8(Str::lit("'")), List::cons(f468(v1), List::cons(f8(Str::lit("'")), List::<Bytes>::nil()))))
        }
    }
}

pub fn f475(v0: Bytes) -> Bytes {
    if f193(v0.clone(), f8(Str::lit("_"))) {
        f43(v0.clone(), 1u64, {
            let a3319 = {
                let a3318 = v0;
                length_bytes(a3318)
            };
            let a3320 = 1u64;
            nat_sub(a3319, a3320)
        })
    } else {
        v0
    }
}

pub fn f476(v0: Adt15, v1: List<Adt22>) -> Result<Adt31, Bytes> {
    if f355(v1.clone(), Str::lit("?")) {
        Ok::<Adt31, Bytes>(Adt31::C0(f477(Str::lit("Option"), v0), f357(v1)))
    } else {
        Ok::<Adt31, Bytes>(Adt31::C0(v0, v1))
    }
}

pub fn f477(v0: Str, v1: Adt15) -> Adt15 {
    let v2: Adt15 = f480(v1);
    Adt15::C0(0u64, f7(List::cons(f8(Str::lit("(")), List::cons({
        let a3321 = v0;
        utf8_encode(a3321)
    }, List::cons(f8(Str::lit(" ")), List::cons(f273(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), f274(v2.clone()), f271(v2), Bytes::lit(&[]))
}

pub fn f478(v0: Bytes) -> Adt15 {
    Adt15::C0(0u64, Bytes::lit(&[]), List::<Bytes>::nil(), v0, Bytes::lit(&[]))
}

pub fn f479(v0: Bytes) -> Option<Adt15> {
    f550(v0, f551())
}

pub fn f480(v0: Adt15) -> Adt15 {
    let a3322 = f272(v0.clone());
    let a3323 = 0u64;
    if nat_eq(a3322, a3323) {
        v0
    } else {
        Adt15::C0(0u64, Bytes::lit(&[]), f274(v0.clone()), if f46(f271(v0.clone()), Bytes::lit(&[])) {
            f552()
        } else {
            f271(v0)
        }, Bytes::lit(&[]))
    }
}

pub fn f481(v0: Adt15) -> Bytes {
    let m3324 = v0;
    match m3324 {
        Adt15::C0(_, _, _, _, v5) => {
            v5
        }
    }
}

pub fn f482(p3341: u64, p3342: u64, p3343: u64, p3344: Bytes) -> Bytes {
    let m3340 = (p3341, (p3342, (p3343, p3344)));
    {
        let mut m3340 = m3340;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m3340;
                let m3325 = v0;
                if m3325 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m3325 - 1;
                    if f87({
                        let a3326 = v1;
                        let a3327 = 0u64;
                        nat_eq(a3326, a3327)
                    }, {
                        let a3328 = v2;
                        let a3329 = 0u64;
                        nat_eq(a3328, a3329)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a3330 = v1;
                            let a3331 = 16u64;
                            let a3332 = 0u64;
                            nat_quot(a3330, a3331, a3332)
                        }, ({
                            let a3333 = v2;
                            let a3334 = 1u64;
                            nat_sub(a3333, a3334)
                        }, {
                            let a3338 = f553({
                                let a3335 = v1;
                                let a3336 = 16u64;
                                let a3337 = 0u64;
                                nat_rem(a3335, a3336, a3337)
                            });
                            let a3339 = v3;
                            append_bytes(a3338, a3339)
                        }))))
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

pub fn f483(v0: Adt14, v1: List<Adt14>) -> R<List<Adt14>> {
    let m3345 = v1;
    match m3345.uncons() {
        None => {
            Ok(List::cons(v0, List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            if f554({
                let a3347 = {
                    let a3346 = f378(v0.clone());
                    length_bytes(a3346)
                };
                let a3348 = 1u64;
                nat_add(a3347, a3348)?
            }, f378(v0.clone()), f378(v2.clone()), 0u64)? {
                Ok(List::cons(v0, List::cons(v2, v3)))
            } else {
                Ok(List::cons(v2, f483(v0, v3)?))
            }
        }
    }
}

pub fn f484(v0: List<Adt13>, v1: Bytes) -> List<Adt14> {
    let m3349 = f175(v0.clone(), 1u64, v1.clone());
    match m3349 {
        None => {
            f375(v0, 6u64, v1)
        }
        Some(v2) => {
            f373(f374(v2), f375(v0, 6u64, v1))
        }
    }
}

pub fn f485(v0: Bytes) -> R<Option<Bytes>> {
    let m3351 = f555({
        let a3350 = v0.clone();
        length_bytes(a3350)
    }, v0.clone(), 0u64, Bytes::lit(&[]))?;
    match m3351 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a3353 = {
                let a3352 = v1.clone();
                length_bytes(a3352)
            };
            let a3354 = 0u64;
            if nat_eq(a3353, a3354) {
                let a3356 = {
                    let a3355 = v0;
                    length_bytes(a3355)
                };
                let a3357 = 0u64;
                if nat_eq(a3356, a3357) {
                    Ok(Some(f8(Str::lit("u"))))
                } else {
                    Ok(None::<Bytes>)
                }
            } else {
                Ok(Some(f487(v1)?))
            }
        }
    }
}

pub fn f486(v0: Bytes) -> R<Option<Bytes>> {
    if f556({
        let a3358 = v0.clone();
        length_bytes(a3358)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f557(v0)?;
        if f558(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m3361 = {
                let a3359 = v1.clone();
                let a3360 = 0u64;
                index_bytes(a3359, a3360)
            };
            match m3361 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f399(f53(v2)?) {
                        Ok(Some({
                            let a3362 = f8(Str::lit("n"));
                            let a3363 = v1;
                            append_bytes(a3362, a3363)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f559({
            let a3364 = v0.clone();
            length_bytes(a3364)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f487(v0: Bytes) -> R<Bytes> {
    if f167(v0.clone(), f560()) {
        f371(v0)
    } else {
        Ok(v0)
    }
}

pub fn f488() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{3}/\u{7f}\u{3}@\u{0}\u{3}\u{7f}}\u{4}\u{0}\u{0}C\u{7f}\u{7f}"))
}

pub fn f489(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a3365 = v1;
        let a3366 = 3u64;
        nat_mul(a3365, a3366)?
    };
    let a3377 = {
        let a3373 = {
            let a3367 = f85(v0.clone(), v2)?;
            let a3368 = 16384u64;
            nat_mul(a3367, a3368)?
        };
        let a3374 = {
            let a3371 = f85(v0.clone(), {
                let a3369 = v2;
                let a3370 = 1u64;
                nat_add(a3369, a3370)?
            })?;
            let a3372 = 128u64;
            nat_mul(a3371, a3372)?
        };
        nat_add(a3373, a3374)?
    };
    let a3378 = f85(v0, {
        let a3375 = v2;
        let a3376 = 2u64;
        nat_add(a3375, a3376)?
    })?;
    nat_add(a3377, a3378)
}

pub fn f490(v0: u64) -> R<bool> {
    f277(v0, f561())
}

pub fn f491(v0: u64) -> R<bool> {
    f277(v0, f562())
}

pub fn f492(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f563(v1, v0, v2)
}

pub fn f493(v0: Bytes, v1: u64) -> Bytes {
    f43(v0, v1, 1u64)
}

pub fn f494() -> Bytes {
    f8(Str::lit("\u{0}\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\u{9}\u{a}\u{b}\u{c}\u{d}\u{e}\u{f}\u{10}\u{11}\u{12}\u{13}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{1b}\u{1c}\u{1d}\u{1e}\u{1f} !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~\u{7f}"))
}

pub fn f495() -> Bytes {
    f8(Str::lit("\u{80}\u{c0}\u{100}\u{140}\u{180}\u{1c0}\u{200}\u{240}\u{280}\u{2c0}\u{300}\u{340}\u{380}\u{3c0}\u{400}\u{440}\u{480}\u{4c0}\u{500}\u{540}\u{580}\u{5c0}\u{600}\u{640}\u{680}\u{6c0}\u{700}\u{740}\u{780}\u{7c0}"))
}

pub fn f496(v0: u64) -> R<Bytes> {
    Ok(f493(f564(), {
        let a3384 = {
            let a3382 = {
                let a3379 = v0;
                let a3380 = 64u64;
                let a3381 = 0u64;
                nat_rem(a3379, a3380, a3381)
            };
            let a3383 = 2u64;
            nat_mul(a3382, a3383)?
        };
        let a3385 = 1u64;
        nat_add(a3384, a3385)?
    }))
}

pub fn f497() -> Bytes {
    f8(Str::lit("\u{800}\u{1000}\u{2000}\u{3000}\u{4000}\u{5000}\u{6000}\u{7000}\u{8000}\u{9000}\u{a000}\u{b000}\u{c000}\u{d000}\u{e000}\u{f000}"))
}

pub fn f498(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<Option<u64>> {
    if f565(f121(v0, v3)?, v1, v2)? {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3386 = v3;
            let a3387 = 1u64;
            nat_add(a3386, a3387)?
        }))
    }
}

pub fn f499(v0: Bytes) -> Result<Bytes, Bytes> {
    Err::<Bytes, Bytes>(v0)
}

pub fn f500() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f501(v0: Bytes) -> R<Option<Bytes>> {
    let m3388 = f180(v0.clone(), f8(Str::lit(" ")), 0u64)?;
    match m3388 {
        None => {
            let m3389 = f180(v0.clone(), f8(Str::lit(":")), 0u64)?;
            match m3389 {
                None => {
                    let v3: Bytes = f7(List::cons(f8(Str::lit(" ")), List::cons(v0, List::cons(f8(Str::lit(":")), List::<Bytes>::nil()))));
                    let m3390 = f180(f566(), v3.clone(), 0u64)?;
                    match m3390 {
                        None => {
                            Ok(None::<Bytes>)
                        }
                        Some(v4) => {
                            let v5: u64 = {
                                let a3392 = v4;
                                let a3393 = {
                                    let a3391 = v3;
                                    length_bytes(a3391)
                                };
                                nat_add(a3392, a3393)?
                            };
                            let m3394 = f180(f566(), f8(Str::lit(" ")), v5)?;
                            match m3394 {
                                None => {
                                    Ok(None::<Bytes>)
                                }
                                Some(v6) => {
                                    Ok(Some(f43(f566(), v5, {
                                        let a3395 = v6;
                                        let a3396 = v5;
                                        nat_sub(a3395, a3396)
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

pub fn f502(v0: Bytes) -> Bytes {
    if f38(v0.clone(), Bytes::lit(&[0x0d])) {
        f43(v0.clone(), 0u64, {
            let a3398 = {
                let a3397 = v0;
                length_bytes(a3397)
            };
            let a3399 = 1u64;
            nat_sub(a3398, a3399)
        })
    } else {
        v0
    }
}

pub fn f503() -> List<Bytes> {
    List::cons(f8(Str::lit("complexType")), List::cons(f8(Str::lit("simpleType")), List::cons(f8(Str::lit("element")), List::cons(f8(Str::lit("attribute")), List::cons(f8(Str::lit("attributeGroup")), List::cons(f8(Str::lit("group")), List::<Bytes>::nil()))))))
}

pub fn f504(v0: u64, v1: Adt24, v2: Adt25) -> R<Result<List<Adt27>, Bytes>> {
    let v3: Bytes = f200(match v2.clone() {
        Adt25::C0(_, _, h3400, _) => {
            h3400
        }
    })?;
    let v4: (Bytes, Bytes) = (match v2.clone() {
        Adt25::C0(_, h3401, _, _) => {
            h3401
        }
    }, match v2.clone() {
        Adt25::C0(_, _, h3402, _) => {
            h3402
        }
    });
    let v5: bool = f567(match v2.clone() {
        Adt25::C0(_, _, _, h3403) => {
            h3403
        }
    }, Str::lit("abstract"), Str::lit("true"));
    let m3405 = f568(match v1.clone() {
        Adt24::C0(h3404, _) => {
            h3404
        }
    }, v4, f308(v5))?;
    match m3405 {
        Ok(v7) => {
            if v5 {
                let m3406 = v7.clone();
                match m3406.uncons() {
                    None => {
                        Ok(Err::<List<Adt27>, Bytes>(f510(f8(Str::lit("an abstract complex type with no type derived from it in this schema set admits no instance (XSD 1.1 Part 1, sections 3.4.1 and 3.3.4.2)")), v3)))
                    }
                    Some((_, _)) => {
                        let m3407 = f569(v3, v7.clone(), f570(v7.clone())?, f570(v7)?)?;
                        match m3407 {
                            Ok(v11) => {
                                Ok(Ok::<List<Adt27>, Bytes>(List::cons(v11, List::<Adt27>::nil())))
                            }
                            Err(v10) => {
                                Ok(Err::<List<Adt27>, Bytes>(v10))
                            }
                        }
                    }
                }
            } else {
                let m3408 = v7.clone();
                match m3408.uncons() {
                    None => {
                        f571(v0, v1, v3, match v2.clone() {
                            Adt25::C0(_, h3409, _, _) => {
                                h3409
                            }
                        }, match v2 {
                            Adt25::C0(_, _, _, h3410) => {
                                h3410
                            }
                        })
                    }
                    Some((_, _)) => {
                        let v14: Bytes = f572(v3.clone());
                        let m3413 = f571(v0, v1, v14.clone(), match v2.clone() {
                            Adt25::C0(_, h3411, _, _) => {
                                h3411
                            }
                        }, match v2.clone() {
                            Adt25::C0(_, _, _, h3412) => {
                                h3412
                            }
                        })?;
                        match m3413 {
                            Ok(v16) => {
                                let m3415 = f569(v3, List::cons(match v2 {
                                    Adt25::C0(_, _, h3414, _) => {
                                        h3414
                                    }
                                }, v7.clone()), List::cons(v14.clone(), f570(v7.clone())?), List::cons(v14, f570(v7)?))?;
                                match m3415 {
                                    Ok(v18) => {
                                        Ok(Ok::<List<Adt27>, Bytes>(f404(v16, List::cons(v18, List::<Adt27>::nil()))))
                                    }
                                    Err(v17) => {
                                        Ok(Err::<List<Adt27>, Bytes>(v17))
                                    }
                                }
                            }
                            Err(v15) => {
                                Ok(Err::<List<Adt27>, Bytes>(v15))
                            }
                        }
                    }
                }
            }
        }
        Err(v6) => {
            Ok(Err::<List<Adt27>, Bytes>(v6))
        }
    }
}

pub fn f505(v0: Bytes, v1: Adt26, v2: Bytes) -> R<Result<Adt27, Bytes>> {
    let m3418 = f199(v0.clone(), match v1.clone() {
        Adt26::C0(_, _, _, h3416, _) => {
            h3416
        }
    }, match v1 {
        Adt26::C0(_, _, _, _, h3417) => {
            h3417
        }
    }, v2)?;
    match m3418 {
        Ok(v4) => {
            Ok(Ok::<Adt27, Bytes>(Adt27::C0(f200(v0)?, match v4.clone() {
                Adt23::C0(_, h3419) => {
                    h3419
                }
            }, match v4 {
                Adt23::C0(h3420, _) => {
                    h3420
                }
            }, List::<Bytes>::nil())))
        }
        Err(v3) => {
            Ok(Err::<Adt27, Bytes>(v3))
        }
    }
}

pub fn f506(v0: u64, v1: Adt24, v2: Bytes, v3: Bytes, v4: Adt26) -> R<Result<Adt32, Bytes>> {
    let m3421 = v0;
    if m3421 == 0 {
        Ok(Err::<Adt32, Bytes>(f510(f8(Str::lit("the element declarations nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m3421 - 1;
        let m3422 = f310(v4.clone(), Str::lit("type"));
        match m3422 {
            None => {
                let m3426 = f508(v4.clone())?;
                match m3426 {
                    Ok(v10) => {
                        let m3427 = f509(v10, List::cons(f8(Str::lit("complexType")), List::cons(f8(Str::lit("simpleType")), List::<Bytes>::nil())));
                        match m3427 {
                            None => {
                                Ok(Ok::<Adt32, Bytes>(Adt32::C0(Adt23::C0(f574(v4, f407()), List::cons(f407(), List::<Bytes>::nil())), List::<Adt27>::nil())))
                            }
                            Some(v11) => {
                                if f309(v11.clone(), Str::lit("complexType")) {
                                    let m3428 = f571(v5, v1, v2.clone(), v3, v11)?;
                                    match m3428 {
                                        Ok(v13) => {
                                            Ok(Ok::<Adt32, Bytes>(Adt32::C0(Adt23::C0(f574(v4, v2.clone()), List::cons(v2, List::<Bytes>::nil())), v13)))
                                        }
                                        Err(v12) => {
                                            Ok(Err::<Adt32, Bytes>(v12))
                                        }
                                    }
                                } else {
                                    let m3429 = f505(v2.clone(), v11, v3)?;
                                    match m3429 {
                                        Ok(v15) => {
                                            Ok(Ok::<Adt32, Bytes>(Adt32::C0(Adt23::C0(f574(v4, v2.clone()), List::cons(v2, List::<Bytes>::nil())), List::cons(v15, List::<Adt27>::nil()))))
                                        }
                                        Err(v14) => {
                                            Ok(Err::<Adt32, Bytes>(v14))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(v9) => {
                        Ok(Err::<Adt32, Bytes>(v9))
                    }
                }
            }
            Some(v6) => {
                let m3423 = f573(v1, v4.clone(), v6)?;
                match m3423 {
                    Ok(v8) => {
                        Ok(Ok::<Adt32, Bytes>(Adt32::C0(Adt23::C0(f574(v4, match v8.clone() {
                            Adt23::C0(h3424, _) => {
                                h3424
                            }
                        }), match v8 {
                            Adt23::C0(_, h3425) => {
                                h3425
                            }
                        }), List::<Adt27>::nil())))
                    }
                    Err(v7) => {
                        Ok(Err::<Adt32, Bytes>(f510(v7, v2)))
                    }
                }
            }
        }
    }
}

pub fn f507(v0: Adt24, v1: Bytes, v2: Bytes, v3: Adt26) -> R<Result<Adt32, Bytes>> {
    let m3430 = f310(v3.clone(), Str::lit("type"));
    match m3430 {
        None => {
            let m3432 = f508(v3)?;
            match m3432 {
                Ok(v8) => {
                    let m3433 = f509(v8, List::cons(f8(Str::lit("simpleType")), List::<Bytes>::nil()));
                    match m3433 {
                        None => {
                            Ok(Err::<Adt32, Bytes>(f510(f8(Str::lit("an attribute declaration with no type has the type xs:anySimpleType, which is not translated, because it is the union of every simple value space (XSD 1.1 Part 1, section 3.2.2.1; Part 2, section 3.2.1)")), v2)))
                        }
                        Some(v9) => {
                            let m3434 = f505(v2.clone(), v9, v1)?;
                            match m3434 {
                                Ok(v11) => {
                                    Ok(Ok::<Adt32, Bytes>(Adt32::C0(Adt23::C0(v2.clone(), List::cons(v2, List::<Bytes>::nil())), List::cons(v11, List::<Adt27>::nil()))))
                                }
                                Err(v10) => {
                                    Ok(Err::<Adt32, Bytes>(v10))
                                }
                            }
                        }
                    }
                }
                Err(v7) => {
                    Ok(Err::<Adt32, Bytes>(v7))
                }
            }
        }
        Some(v4) => {
            let m3431 = f573(v0, v3, v4)?;
            match m3431 {
                Ok(v6) => {
                    Ok(Ok::<Adt32, Bytes>(Adt32::C0(v6, List::<Adt27>::nil())))
                }
                Err(v5) => {
                    Ok(Err::<Adt32, Bytes>(f510(v5, v2)))
                }
            }
        }
    }
}

pub fn f508(v0: Adt26) -> R<Result<List<Adt26>, Bytes>> {
    f213(match v0.clone() {
        Adt26::C0(_, _, _, h3435, _) => {
            h3435
        }
    }, match v0 {
        Adt26::C0(_, _, _, _, h3436) => {
            h3436
        }
    })
}

pub fn f509(p3440: List<Adt26>, v1: List<Bytes>) -> Option<Adt26> {
    let m3439 = p3440;
    {
        let mut m3439 = m3439;
        loop {
            let step = {
                let v0 = m3439;
                let m3437 = v0;
                match m3437.uncons() {
                    None => {
                        Err::<List<Adt26>, Option<Adt26>>(None::<Adt26>)
                    }
                    Some((v2, v3)) => {
                        if f167(match v2.clone() {
                            Adt26::C0(_, h3438, _, _, _) => {
                                h3438
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
                Ok(next) => { m3439 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f510(v0: Bytes, v1: Bytes) -> Bytes {
    f209(v0, f8(Str::lit(" at ")), v1)
}

pub fn f511(v0: u64, v1: Adt24, v2: Bytes, v3: Bytes, v4: Adt26) -> R<Result<List<Adt27>, Bytes>> {
    let m3441 = v0;
    if m3441 == 0 {
        Ok(Err::<List<Adt27>, Bytes>(f510(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m3441 - 1;
        let m3442 = f508(v4.clone())?;
        match m3442 {
            Ok(v7) => {
                if f309(v4, Str::lit("choice")) {
                    let m3443 = f575(v5, v1, v2.clone(), v3, false, v7, f513())?;
                    match m3443 {
                        Ok(v9) => {
                            let m3447 = f569(v2, f576(match v9.clone() {
                                Adt33::C0(h3444, _, _, _, _) => {
                                    h3444
                                }
                            }), f577(match v9.clone() {
                                Adt33::C0(h3445, _, _, _, _) => {
                                    h3445
                                }
                            }), f578(match v9.clone() {
                                Adt33::C0(h3446, _, _, _, _) => {
                                    h3446
                                }
                            }))?;
                            match m3447 {
                                Ok(v11) => {
                                    Ok(Ok::<List<Adt27>, Bytes>(f404(match v9 {
                                        Adt33::C0(_, h3448, _, _, _) => {
                                            h3448
                                        }
                                    }, List::cons(v11, List::<Adt27>::nil()))))
                                }
                                Err(v10) => {
                                    Ok(Err::<List<Adt27>, Bytes>(v10))
                                }
                            }
                        }
                        Err(v8) => {
                            Ok(Err::<List<Adt27>, Bytes>(v8))
                        }
                    }
                } else {
                    let m3449 = f575(v5, v1, v2.clone(), v3, true, v7, f513())?;
                    match m3449 {
                        Ok(v13) => {
                            let m3451 = f579(v2, match v13.clone() {
                                Adt33::C0(h3450, _, _, _, _) => {
                                    h3450
                                }
                            })?;
                            match m3451 {
                                Ok(v15) => {
                                    Ok(Ok::<List<Adt27>, Bytes>(f404(match v13 {
                                        Adt33::C0(_, h3452, _, _, _) => {
                                            h3452
                                        }
                                    }, List::cons(v15, List::<Adt27>::nil()))))
                                }
                                Err(v14) => {
                                    Ok(Err::<List<Adt27>, Bytes>(v14))
                                }
                            }
                        }
                        Err(v12) => {
                            Ok(Err::<List<Adt27>, Bytes>(v12))
                        }
                    }
                }
            }
            Err(v6) => {
                Ok(Err::<List<Adt27>, Bytes>(v6))
            }
        }
    }
}

pub fn f512(v0: u64, v1: Adt24, v2: Bytes, v3: Bytes, v4: List<Adt26>, v5: Adt33) -> R<Result<Adt33, Bytes>> {
    let m3453 = v0;
    if m3453 == 0 {
        Ok(Err::<Adt33, Bytes>(f510(f8(Str::lit("the attribute group references nest deeper than the schema set has components, so they refer to each other in a cycle (XSD 1.1 Part 1, section 3.6.3)")), v2)))
    } else {
        let v6 = m3453 - 1;
        let m3454 = v4;
        match m3454.uncons() {
            None => {
                Ok(Ok::<Adt33, Bytes>(v5))
            }
            Some((v7, v8)) => {
                if f309(v7.clone(), Str::lit("attribute")) {
                    if f567(v7.clone(), Str::lit("use"), Str::lit("prohibited")) {
                        f512(v6, v1, v2, v3, v8, v5)
                    } else {
                        let v9: bool = f567(v7.clone(), Str::lit("use"), Str::lit("required"));
                        let m3455 = f310(v7.clone(), Str::lit("ref"));
                        match m3455 {
                            None => {
                                let m3465 = f310(v7.clone(), Str::lit("name"));
                                match m3465 {
                                    None => {
                                        Ok(Err::<Adt33, Bytes>(f510(f8(Str::lit("a local attribute declaration must have a name or a ref (XSD 1.1 Part 1, section 3.2.3)")), v2)))
                                    }
                                    Some(v16) => {
                                        let m3468 = f507(v1.clone(), v3.clone(), {
                                            let a3466 = v2.clone();
                                            let a3467 = f200(v16.clone())?;
                                            append_bytes(a3466, a3467)
                                        }, v7)?;
                                        match m3468 {
                                            Ok(v18) => {
                                                f512(v6, v1, v2, v3, v8, f581(v5, v16, v18, v9))
                                            }
                                            Err(v17) => {
                                                Ok(Err::<Adt33, Bytes>(v17))
                                            }
                                        }
                                    }
                                }
                            }
                            Some(v10) => {
                                let m3457 = f125(match v7 {
                                    Adt26::C0(_, _, _, _, h3456) => {
                                        h3456
                                    }
                                }, v10.clone())?;
                                match m3457 {
                                    Ok(v12) => {
                                        if f46({
                                            let (h3458, _) = v12.clone();
                                            h3458
                                        }, f191()) {
                                            Ok(Err::<Adt33, Bytes>(f510(f209(f8(Str::lit("the attribute ")), v10, f8(Str::lit(" is declared by the schema document for the XML namespace, which this schema set does not import (XSD 1.1 Part 1, sections 3.2.2.3 and 4.2.6)"))), v2)))
                                        } else {
                                            let m3459 = f580(v1.clone(), Str::lit("attribute"), v12.clone());
                                            match m3459 {
                                                None => {
                                                    Ok(Err::<Adt33, Bytes>(f510(f209(f8(Str::lit("the attribute ")), v10, f8(Str::lit(" is not declared by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                                }
                                                Some(v13) => {
                                                    let m3463 = f507(v1.clone(), match v13.clone() {
                                                        Adt25::C0(_, h3460, _, _) => {
                                                            h3460
                                                        }
                                                    }, f200({
                                                        let (_, h3461) = v12.clone();
                                                        h3461
                                                    })?, match v13 {
                                                        Adt25::C0(_, _, _, h3462) => {
                                                            h3462
                                                        }
                                                    })?;
                                                    match m3463 {
                                                        Ok(v15) => {
                                                            f512(v6, v1, v2, v3, v8, f581(v5, {
                                                                let (_, h3464) = v12;
                                                                h3464
                                                            }, v15, v9))
                                                        }
                                                        Err(v14) => {
                                                            Ok(Err::<Adt33, Bytes>(v14))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(Err::<Adt33, Bytes>(f510(v11, v2)))
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if f309(v7.clone(), Str::lit("attributeGroup")) {
                        let m3469 = f310(v7.clone(), Str::lit("ref"));
                        match m3469 {
                            None => {
                                Ok(Err::<Adt33, Bytes>(f510(f8(Str::lit("an attribute group reference must have a ref (XSD 1.1 Part 1, section 3.6.2)")), v2)))
                            }
                            Some(v19) => {
                                let m3471 = f125(match v7 {
                                    Adt26::C0(_, _, _, _, h3470) => {
                                        h3470
                                    }
                                }, v19.clone())?;
                                match m3471 {
                                    Ok(v21) => {
                                        let m3472 = f580(v1.clone(), Str::lit("attributeGroup"), v21.clone());
                                        match m3472 {
                                            None => {
                                                Ok(Err::<Adt33, Bytes>(f510(f209(f8(Str::lit("the attribute group ")), v19, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                            }
                                            Some(v22) => {
                                                let m3474 = f508(match v22.clone() {
                                                    Adt25::C0(_, _, _, h3473) => {
                                                        h3473
                                                    }
                                                })?;
                                                match m3474 {
                                                    Ok(v24) => {
                                                        let m3477 = f512(v6, v1.clone(), f200({
                                                            let (_, h3475) = v21;
                                                            h3475
                                                        })?, match v22 {
                                                            Adt25::C0(_, h3476, _, _) => {
                                                                h3476
                                                            }
                                                        }, v24, v5)?;
                                                        match m3477 {
                                                            Ok(v26) => {
                                                                f512(v6, v1, v2, v3, v8, v26)
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
                                            }
                                        }
                                    }
                                    Err(v20) => {
                                        Ok(Err::<Adt33, Bytes>(f510(v20, v2)))
                                    }
                                }
                            }
                        }
                    } else {
                        if f309(v7, Str::lit("anyAttribute")) {
                            f512(v6, v1, v2, v3, v8, f582(v5, Adt34::C0(f8(Str::lit("anyAttribute")), f583(), List::<Bytes>::nil())))
                        } else {
                            f512(v6, v1, v2, v3, v8, v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f513() -> Adt33 {
    Adt33::C0(List::<Adt34>::nil(), List::<Adt27>::nil(), 0u64, 0u64, 0u64)
}

pub fn f514(v0: List<Adt27>, v1: List<Bytes>, v2: Bytes) -> Result<List<Bytes>, Bytes> {
    let m3478 = v1;
    match m3478.uncons() {
        None => {
            Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let m3479 = f406(v0.clone(), v3.clone());
            match m3479 {
                None => {
                    Err::<List<Bytes>, Bytes>(f515(v3))
                }
                Some(v5) => {
                    let m3481 = match v5.clone() {
                        Adt27::C0(_, _, _, h3480) => {
                            h3480
                        }
                    };
                    match m3481.uncons() {
                        None => {
                            Err::<List<Bytes>, Bytes>(f515(v3))
                        }
                        Some((_, _)) => {
                            let m3482 = f514(v0, v4, v2.clone());
                            match m3482 {
                                Ok(v9) => {
                                    Ok::<List<Bytes>, Bytes>(List::cons(f584(v3, match v5 {
                                        Adt27::C0(_, _, _, h3483) => {
                                            h3483
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

pub fn f515(v0: Bytes) -> Bytes {
    f510(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)")), v0)
}

pub fn f516(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m3484 = v0;
    match m3484.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f167(v2.clone(), v1.clone()) {
                f516(v3, v1)
            } else {
                List::cons(v2, f516(v3, v1))
            }
        }
    }
}

pub fn f517(p3489: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m3488 = p3489;
    {
        let mut m3488 = m3488;
        loop {
            let step = {
                let v0 = m3488;
                let m3485 = v0;
                match m3485.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f46({
                            let (h3486, _) = v2;
                            h3486
                        }, v1.clone()) {
                            let (_, h3487) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h3487)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3488 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f518(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a3534 = {
            let a3510 = {
                let a3498 = {
                    let a3492 = {
                        let a3490 = v3;
                        let a3491 = 128u8;
                        fixed_u8::bit_and(a3490, a3491)
                    };
                    let a3493 = 128u8;
                    if equal(a3492, a3493) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3499 = {
                    let a3496 = {
                        let a3494 = v3;
                        let a3495 = 64u8;
                        fixed_u8::bit_and(a3494, a3495)
                    };
                    let a3497 = 64u8;
                    if equal(a3496, a3497) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3498, a3499)?
            };
            let a3511 = {
                let a3508 = {
                    let a3502 = {
                        let a3500 = v3;
                        let a3501 = 32u8;
                        fixed_u8::bit_and(a3500, a3501)
                    };
                    let a3503 = 32u8;
                    if equal(a3502, a3503) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3509 = {
                    let a3506 = {
                        let a3504 = v3;
                        let a3505 = 16u8;
                        fixed_u8::bit_and(a3504, a3505)
                    };
                    let a3507 = 16u8;
                    if equal(a3506, a3507) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3508, a3509)?
            };
            nat_add(a3510, a3511)?
        };
        let a3535 = {
            let a3532 = {
                let a3520 = {
                    let a3514 = {
                        let a3512 = v3;
                        let a3513 = 8u8;
                        fixed_u8::bit_and(a3512, a3513)
                    };
                    let a3515 = 8u8;
                    if equal(a3514, a3515) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3521 = {
                    let a3518 = {
                        let a3516 = v3;
                        let a3517 = 4u8;
                        fixed_u8::bit_and(a3516, a3517)
                    };
                    let a3519 = 4u8;
                    if equal(a3518, a3519) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3520, a3521)?
            };
            let a3533 = {
                let a3530 = {
                    let a3524 = {
                        let a3522 = v3;
                        let a3523 = 2u8;
                        fixed_u8::bit_and(a3522, a3523)
                    };
                    let a3525 = 2u8;
                    if equal(a3524, a3525) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3531 = {
                    let a3528 = {
                        let a3526 = v3;
                        let a3527 = 1u8;
                        fixed_u8::bit_and(a3526, a3527)
                    };
                    let a3529 = 1u8;
                    if equal(a3528, a3529) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3530, a3531)?
            };
            nat_add(a3532, a3533)?
        };
        nat_add(a3534, a3535)?
    };
    let a3536 = v4;
    let a3537 = 34u64;
    if nat_eq(a3536, a3537) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a3538 = v4;
        let a3539 = 92u64;
        if nat_eq(a3538, a3539) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a3540 = v4;
            let a3541 = 8u64;
            if nat_eq(a3540, a3541) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a3542 = v4;
                let a3543 = 12u64;
                if nat_eq(a3542, a3543) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a3544 = v4;
                    let a3545 = 10u64;
                    if nat_eq(a3544, a3545) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a3546 = v4;
                        let a3547 = 13u64;
                        if nat_eq(a3546, a3547) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a3548 = v4;
                            let a3549 = 9u64;
                            if nat_eq(a3548, a3549) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a3550 = v4;
                                let a3551 = 32u64;
                                if nat_lt(a3550, a3551) {
                                    let a3552 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a3553 = f585(v2)?;
                                    Ok(append_bytes(a3552, a3553))
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

pub fn f519(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m3554 = v0.clone();
    match m3554.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a3557 = {
                let (h3555, _) = v2;
                h3555
            };
            let a3558 = {
                let (h3556, _) = v1.clone();
                h3556
            };
            if equal(a3557, a3558) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f520(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f586(f587(Fn10::F588, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m3563 = {
        let (h3559, _) = f589(Fn11::F590, {
            let a3561 = {
                let a3560 = v0;
                length_list(a3560)
            };
            let a3562 = 1u64;
            nat_add(a3561, a3562)?
        }, v1)?;
        h3559
    };
    match m3563.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f521(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a3568 = {
        let a3565 = f316({
            let (h3564, _) = v0;
            h3564
        })?;
        let a3566 = Bytes::lit(&[0x3a]);
        append_bytes(a3565, a3566)
    };
    let a3569 = {
        let (_, h3567) = v0;
        h3567
    };
    Ok(append_bytes(a3568, a3569))
}

pub fn f522(v0: Fn8, p3574: u64, p3575: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m3573 = (p3574, p3575);
    {
        let mut m3573 = m3573;
        loop {
            let step = {
                let (v1, v2) = m3573;
                let m3570 = v1;
                if m3570 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3570 - 1;
                    let m3572 = {
                        let c3571: Fn8 = v0.clone();
                        c3571.apply(v2.clone())?
                    };
                    match m3572 {
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
                Ok(next) => { m3573 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f523(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f591(v0, v1)
}

pub fn f524(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3620 = {
            let a3596 = {
                let a3584 = {
                    let a3578 = {
                        let a3576 = v1;
                        let a3577 = 128u8;
                        fixed_u8::bit_and(a3576, a3577)
                    };
                    let a3579 = 128u8;
                    if equal(a3578, a3579) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3585 = {
                    let a3582 = {
                        let a3580 = v1;
                        let a3581 = 64u8;
                        fixed_u8::bit_and(a3580, a3581)
                    };
                    let a3583 = 64u8;
                    if equal(a3582, a3583) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3584, a3585)?
            };
            let a3597 = {
                let a3594 = {
                    let a3588 = {
                        let a3586 = v1;
                        let a3587 = 32u8;
                        fixed_u8::bit_and(a3586, a3587)
                    };
                    let a3589 = 32u8;
                    if equal(a3588, a3589) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3595 = {
                    let a3592 = {
                        let a3590 = v1;
                        let a3591 = 16u8;
                        fixed_u8::bit_and(a3590, a3591)
                    };
                    let a3593 = 16u8;
                    if equal(a3592, a3593) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3594, a3595)?
            };
            nat_add(a3596, a3597)?
        };
        let a3621 = {
            let a3618 = {
                let a3606 = {
                    let a3600 = {
                        let a3598 = v1;
                        let a3599 = 8u8;
                        fixed_u8::bit_and(a3598, a3599)
                    };
                    let a3601 = 8u8;
                    if equal(a3600, a3601) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3607 = {
                    let a3604 = {
                        let a3602 = v1;
                        let a3603 = 4u8;
                        fixed_u8::bit_and(a3602, a3603)
                    };
                    let a3605 = 4u8;
                    if equal(a3604, a3605) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3606, a3607)?
            };
            let a3619 = {
                let a3616 = {
                    let a3610 = {
                        let a3608 = v1;
                        let a3609 = 2u8;
                        fixed_u8::bit_and(a3608, a3609)
                    };
                    let a3611 = 2u8;
                    if equal(a3610, a3611) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3617 = {
                    let a3614 = {
                        let a3612 = v1;
                        let a3613 = 1u8;
                        fixed_u8::bit_and(a3612, a3613)
                    };
                    let a3615 = 1u8;
                    if equal(a3614, a3615) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3616, a3617)?
            };
            nat_add(a3618, a3619)?
        };
        nat_add(a3620, a3621)?
    };
    let a3622 = 48u64;
    let a3623 = v2;
    if nat_le(a3622, a3623) {
        let a3624 = v2;
        let a3625 = 57u64;
        Ok(nat_le(a3624, a3625))
    } else {
        Ok(false)
    }
}

pub fn f525(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a3626 = 0u64;
    let a3627 = v1;
    if if nat_lt(a3626, a3627) {
        f156({
            let a3630 = f154(v0, {
                let a3628 = v1;
                let a3629 = 1u64;
                nat_sub(a3628, a3629)
            })?;
            let a3631 = 10u64;
            nat_eq(a3630, a3631)
        })
    } else {
        false
    } {
        Ok(Some({
            let a3632 = v1;
            let a3633 = 1u64;
            nat_sub(a3632, a3633)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f526(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f231(f154(v0, v1)?) {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3634 = v1;
            let a3635 = 1u64;
            nat_add(a3634, a3635)?
        }))
    }
}

pub fn f527(v0: Bytes, v1: u64, v2: u64) -> R<Adt35> {
    let v3: (u64, u64) = {
        let (h3636, _) = f223(Fn5::F592(v2, v0.clone()), {
            let a3638 = {
                let a3637 = v0.clone();
                length_bytes(a3637)
            };
            let a3639 = 1u64;
            nat_add(a3638, a3639)?
        }, (v1, 0u64))?;
        h3636
    };
    let v4: u64 = f528(v0.clone(), {
        let (h3640, _) = v3;
        h3640
    }, v2)?;
    let a3641 = f154(v0, v4)?;
    let a3642 = 9u64;
    if if nat_eq(a3641, a3642) {
        let a3646 = {
            let a3644 = v4;
            let a3645 = {
                let (h3643, _) = v3;
                h3643
            };
            nat_sub(a3644, a3645)
        };
        let a3647 = v2;
        nat_lt(a3646, a3647)
    } else {
        false
    } {
        Ok(Adt35::C1(f98(Str::lit("a tab cannot indent a block scalar line (YAML 1.2.2 [63], [70])"), v4)?))
    } else {
        Ok(Adt35::C0({
            let (_, h3648) = v3;
            h3648
        }, {
            let (h3649, _) = v3;
            h3649
        }))
    }
}

pub fn f528(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h3650, _) = f149(Fn4::F593(v2, v0.clone(), v1), {
        let a3652 = {
            let a3651 = v0;
            length_bytes(a3651)
        };
        let a3653 = 1u64;
        nat_add(a3652, a3653)?
    }, v1)?;
    Ok(h3650)
}

pub fn f529(v0: Bytes, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = f528(v0.clone(), v1, v2)?;
    let a3656 = {
        let a3654 = v3;
        let a3655 = v1;
        nat_sub(a3654, a3655)
    };
    let a3657 = v2;
    if nat_eq(a3656, a3657) {
        Ok(f156({
            let a3658 = f154(v0, v3)?;
            let a3659 = 256u64;
            nat_eq(a3658, a3659)
        }))
    } else {
        Ok(false)
    }
}

pub fn f530(v0: Bytes, p3669: u64, v2: u64, v3: bool, p3670: u64, p3671: Bytes, p3672: u64) -> R<Adt36> {
    let m3668 = (p3669, (p3670, (p3671, p3672)));
    {
        let mut m3668 = m3668;
        loop {
            let step = {
                let (v1, (v4, (v5, v6))) = m3668;
                let m3660 = v4;
                if m3660 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<Adt36>>(Ok(Adt36::C1(f98(Str::lit("a block scalar has more lines than the input"), v1)?)))
                } else {
                    let v7 = m3660 - 1;
                    let v8: u64 = f528(v0.clone(), v1, v2)?;
                    let v9: bool = f156(f230(f154(v0.clone(), v8)?));
                    let v10: u64 = f333(v0.clone(), v8)?;
                    let v11: Bytes = f7(List::cons(v5, List::cons(f440(v6), List::cons(f247(v0.clone(), v8, v10), List::<Bytes>::nil()))));
                    let m3661 = f527(v0.clone(), f234(v0.clone(), v10)?, v2)?;
                    match m3661 {
                        Adt35::C0(v13, v14) => {
                            let v15: u64 = v14;
                            let v16: u64 = f528(v0.clone(), v15, v2)?;
                            if f529(v0.clone(), v15, v2)? {
                                Ok::<(u64, (u64, (Bytes, u64))), R<Adt36>>((v15, (v7, ({
                                    let a3664 = v11;
                                    let a3665 = if if if v3 {
                                        v9
                                    } else {
                                        false
                                    } {
                                        f156(f230(f154(v0.clone(), v16)?))
                                    } else {
                                        false
                                    } {
                                        let a3662 = v13;
                                        let a3663 = 0u64;
                                        if nat_eq(a3662, a3663) {
                                            Bytes::lit(&[0x20])
                                        } else {
                                            Bytes::lit(&[])
                                        }
                                    } else {
                                        Bytes::lit(&[0x0a])
                                    };
                                    append_bytes(a3664, a3665)
                                }, v13))))
                            } else {
                                Err::<(u64, (u64, (Bytes, u64))), R<Adt36>>(Ok(Adt36::C0(v11, v13, {
                                    let a3666 = f154(v0.clone(), v16)?;
                                    let a3667 = 256u64;
                                    if nat_eq(a3666, a3667) {
                                        v16
                                    } else {
                                        v15
                                    }
                                })))
                            }
                        }
                        Adt35::C1(v12) => {
                            Err::<(u64, (u64, (Bytes, u64))), R<Adt36>>(Ok(Adt36::C1(v12)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3668 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f531(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f235(v0.clone(), {
        let (h3673, _) = v1;
        h3673
    })?;
    let a3674 = f154(v0, v2)?;
    let a3675 = 10u64;
    if nat_eq(a3674, a3675) {
        Ok(Some(({
            let a3676 = v2;
            let a3677 = 1u64;
            nat_add(a3676, a3677)?
        }, f442({
            let (_, h3678) = v1;
            h3678
        }, {
            let a3680 = v2;
            let a3681 = {
                let (h3679, _) = v1;
                h3679
            };
            nat_sub(a3680, a3681)
        }))))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f532(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f594(v0, v1)
}

pub fn f533(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt21> {
    let v4: (u64, u64) = f535(v0.clone(), v1, 2u64)?;
    let a3684 = {
        let (h3682, _) = v4;
        h3682
    };
    let a3685 = {
        let a3683 = v0.clone();
        length_bytes(a3683)
    };
    if nat_eq(a3684, a3685) {
        if f145(false, {
            let (_, h3686) = v4;
            h3686
        }) {
            Ok(Adt21::C0(Adt2::C3(false, {
                let (_, h3687) = v4;
                h3687
            }), v3))
        } else {
            Ok(Adt21::C1(f534(v2)?))
        }
    } else {
        Ok(Adt21::C0(Adt2::C5(v0), v3))
    }
}

pub fn f534(v0: u64) -> R<Bytes> {
    f98(Str::lit("a number outside the range Studio.Value holds is not read (YAML 1.2.2 section 10.3.2)"), v0)
}

pub fn f535(v0: Bytes, v1: u64, v2: u64) -> R<(u64, u64)> {
    let (h3688, _) = f223(Fn5::F595(v1, v0.clone()), {
        let a3690 = {
            let a3689 = v0;
            length_bytes(a3689)
        };
        let a3691 = 1u64;
        nat_add(a3690, a3691)?
    }, (v2, 0u64))?;
    Ok(h3688)
}

pub fn f536(v0: u64) -> R<Option<u64>> {
    let v1: u64 = f246(f8(Str::lit("abtnvfr")), v0)?;
    let a3692 = v1;
    let a3693 = 7u64;
    if nat_lt(a3692, a3693) {
        Ok(Some({
            let a3694 = v1;
            let a3695 = 7u64;
            nat_add(a3694, a3695)?
        }))
    } else {
        let a3696 = v0;
        let a3697 = 48u64;
        if nat_eq(a3696, a3697) {
            Ok(Some(0u64))
        } else {
            let a3698 = v0;
            let a3699 = 9u64;
            if nat_eq(a3698, a3699) {
                Ok(Some(9u64))
            } else {
                let a3700 = v0;
                let a3701 = 101u64;
                if nat_eq(a3700, a3701) {
                    Ok(Some(27u64))
                } else {
                    let a3702 = v0;
                    let a3703 = 78u64;
                    if nat_eq(a3702, a3703) {
                        Ok(Some(133u64))
                    } else {
                        let a3704 = v0;
                        let a3705 = 95u64;
                        if nat_eq(a3704, a3705) {
                            Ok(Some(160u64))
                        } else {
                            let a3706 = v0;
                            let a3707 = 76u64;
                            if nat_eq(a3706, a3707) {
                                Ok(Some(8232u64))
                            } else {
                                let a3708 = v0;
                                let a3709 = 80u64;
                                if nat_eq(a3708, a3709) {
                                    Ok(Some(8233u64))
                                } else {
                                    let a3710 = f246(f8(Str::lit(" \"/\\")), v0)?;
                                    let a3711 = 4u64;
                                    if nat_lt(a3710, a3711) {
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

pub fn f537(v0: u64) -> R<Bytes> {
    f98(Str::lit("an implicit key must be on one line (YAML 1.2.2 [154])"), v0)
}

pub fn f538(v0: Bytes, p3726: u64, v2: u64, p3727: u64, p3728: u64) -> R<Adt35> {
    let m3725 = (p3726, (p3727, p3728));
    {
        let mut m3725 = m3725;
        loop {
            let step = {
                let (v1, (v3, v4)) = m3725;
                let m3712 = v3;
                if m3712 == 0 {
                    Err::<(u64, (u64, u64)), R<Adt35>>(Ok(Adt35::C1(f98(Str::lit("the input ends inside a scalar"), v1)?)))
                } else {
                    let v5 = m3712 - 1;
                    let v6: u64 = {
                        let a3713 = v1;
                        let a3714 = 1u64;
                        nat_add(a3713, a3714)?
                    };
                    let v7: u64 = f235(v0.clone(), v6)?;
                    let v8: u64 = f233(v0.clone(), v7)?;
                    let v9: u64 = f154(v0.clone(), v8)?;
                    if f152(v0.clone(), v6)? {
                        Err::<(u64, (u64, u64)), R<Adt35>>(Ok(Adt35::C1(f98(Str::lit("a document marker cannot appear inside a scalar (YAML 1.2.2 [206])"), v6)?)))
                    } else {
                        let a3715 = v9;
                        let a3716 = 10u64;
                        if nat_eq(a3715, a3716) {
                            Ok::<(u64, (u64, u64)), R<Adt35>>((v8, (v5, {
                                let a3717 = v4;
                                let a3718 = 1u64;
                                nat_add(a3717, a3718)?
                            })))
                        } else {
                            let a3719 = v9;
                            let a3720 = 256u64;
                            if nat_eq(a3719, a3720) {
                                Err::<(u64, (u64, u64)), R<Adt35>>(Ok(Adt35::C1(f98(Str::lit("the input ends inside a quoted scalar (YAML 1.2.2 [109], [120])"), v8)?)))
                            } else {
                                let a3723 = {
                                    let a3721 = v7;
                                    let a3722 = v6;
                                    nat_sub(a3721, a3722)
                                };
                                let a3724 = v2;
                                if nat_lt(a3723, a3724) {
                                    Err::<(u64, (u64, u64)), R<Adt35>>(Ok(Adt35::C1(f98(Str::lit("a continuation line of a scalar is indented less than its node (YAML 1.2.2 [69])"), v7)?)))
                                } else {
                                    Err::<(u64, (u64, u64)), R<Adt35>>(Ok(Adt35::C0(v4, v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3725 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f539(v0: Bytes, p3738: u64, p3739: u64, p3740: u64) -> R<Option<u64>> {
    let m3737 = (p3738, (p3739, p3740));
    {
        let mut m3737 = m3737;
        loop {
            let step = {
                let (v1, (v2, v3)) = m3737;
                let m3729 = v2;
                if m3729 == 0 {
                    Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3729 - 1;
                    let m3730 = f596(f154(v0.clone(), v1)?);
                    match m3730 {
                        None => {
                            Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(None::<u64>))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, u64)), R<Option<u64>>>(({
                                let a3731 = v1;
                                let a3732 = 1u64;
                                nat_add(a3731, a3732)?
                            }, (v4, {
                                let a3735 = {
                                    let a3733 = v3;
                                    let a3734 = 16u64;
                                    nat_mul(a3733, a3734)?
                                };
                                let a3736 = v5;
                                nat_add(a3735, a3736)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3737 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f540(v0: Bytes, v1: u64) -> R<u64> {
    let (h3741, _) = f149(Fn4::F597(v0.clone()), {
        let a3743 = {
            let a3742 = v0;
            length_bytes(a3742)
        };
        let a3744 = 1u64;
        nat_add(a3743, a3744)?
    }, v1)?;
    Ok(h3741)
}

pub fn f541(v0: Bytes, v1: u64) -> R<u64> {
    let (h3745, _) = f149(Fn4::F598(v0.clone()), {
        let a3747 = {
            let a3746 = v0;
            length_bytes(a3746)
        };
        let a3748 = 1u64;
        nat_add(a3747, a3748)?
    }, v1)?;
    Ok(h3745)
}

pub fn f542(v0: Fn9, p3753: u64, p3754: (u64, bool)) -> R<((u64, bool), bool)> {
    let m3752 = (p3753, p3754);
    {
        let mut m3752 = m3752;
        loop {
            let step = {
                let (v1, v2) = m3752;
                let m3749 = v1;
                if m3749 == 0 {
                    Err::<(u64, (u64, bool)), R<((u64, bool), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3749 - 1;
                    let m3751 = {
                        let c3750: Fn9 = v0.clone();
                        c3750.apply(v2)?
                    };
                    match m3751 {
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
                Ok(next) => { m3752 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f543(v0: u64, v1: Bytes, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    f599(v1, v0, v2)
}

pub fn f544(v0: Bytes, v1: u64, v2: u64) -> R<Adt29> {
    let v3: u64 = f154(v0.clone(), v1)?;
    let a3755 = v3;
    let a3756 = 34u64;
    if if nat_eq(a3755, a3756) {
        true
    } else {
        let a3757 = v3;
        let a3758 = 39u64;
        nat_eq(a3757, a3758)
    } {
        f339(v0, v1, v2, false)
    } else {
        let m3759 = f238(v0.clone(), v1)?;
        match m3759 {
            None => {
                let a3760 = v3;
                let a3761 = 91u64;
                if if nat_eq(a3760, a3761) {
                    true
                } else {
                    let a3762 = v3;
                    let a3763 = 123u64;
                    nat_eq(a3762, a3763)
                } {
                    Ok(Adt29::C1(f98(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [142])"), v1)?))
                } else {
                    if f341(v0.clone(), v1, true)? {
                        let v5: u64 = f342(v0.clone(), v1, true)?;
                        let v6: u64 = f233(v0.clone(), v5)?;
                        let a3764 = f154(v0.clone(), v6)?;
                        let a3765 = 10u64;
                        if nat_eq(a3764, a3765) {
                            let m3766 = f452(v0.clone(), v6, v2)?;
                            match m3766 {
                                Ok(v8) => {
                                    let v9: u64 = f154(v0.clone(), v8)?;
                                    let a3767 = v9;
                                    let a3768 = 58u64;
                                    if if if nat_eq(a3767, a3768) {
                                        true
                                    } else {
                                        let a3769 = v9;
                                        let a3770 = 44u64;
                                        nat_eq(a3769, a3770)
                                    } {
                                        true
                                    } else {
                                        let a3771 = v9;
                                        let a3772 = 125u64;
                                        nat_eq(a3771, a3772)
                                    } {
                                        Ok(Adt29::C0(f247(v0, v1, v5), v5))
                                    } else {
                                        Ok(Adt29::C1(f98(Str::lit("a multi-line plain scalar inside a flow collection is not read (YAML 1.2.2 [135])"), v6)?))
                                    }
                                }
                                Err(v7) => {
                                    Ok(Adt29::C1(v7))
                                }
                            }
                        } else {
                            Ok(Adt29::C0(f247(v0, v1, v5), v5))
                        }
                    } else {
                        Ok(Adt29::C1(f98(Str::lit("a flow mapping key must be a scalar (YAML 1.2.2 [142])"), v1)?))
                    }
                }
            }
            Some(v4) => {
                Ok(Adt29::C1(v4))
            }
        }
    }
}

pub fn f545(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = {
        let (h3773, _) = v2;
        h3773
    };
    let v4: u64 = f154(v0.clone(), v3)?;
    if f231(v4) {
        Ok(None::<(u64, u64)>)
    } else {
        if f230(v4) {
            Ok(Some(({
                let a3774 = v3;
                let a3775 = 1u64;
                nat_add(a3774, a3775)?
            }, {
                let (_, h3776) = v2;
                h3776
            })))
        } else {
            let a3777 = v4;
            let a3778 = 58u64;
            if if nat_eq(a3777, a3778) {
                f156(f335(f154(v0.clone(), {
                    let a3779 = v3;
                    let a3780 = 1u64;
                    nat_add(a3779, a3780)?
                })?, v1))
            } else {
                false
            } {
                Ok(None::<(u64, u64)>)
            } else {
                let a3781 = v4;
                let a3782 = 35u64;
                if if nat_eq(a3781, a3782) {
                    f230(f154(v0, {
                        let a3783 = v3;
                        let a3784 = 1u64;
                        nat_sub(a3783, a3784)
                    })?)
                } else {
                    false
                } {
                    Ok(None::<(u64, u64)>)
                } else {
                    if if v1 {
                        f435(v4)
                    } else {
                        false
                    } {
                        Ok(None::<(u64, u64)>)
                    } else {
                        Ok(Some(({
                            let a3785 = v3;
                            let a3786 = 1u64;
                            nat_add(a3785, a3786)?
                        }, {
                            let a3787 = v3;
                            let a3788 = 1u64;
                            nat_add(a3787, a3788)?
                        })))
                    }
                }
            }
        }
    }
}

pub fn f546(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f600(v0, v1)
}

pub fn f547(v0: Bytes, v1: bool, v2: u64) -> R<Option<u64>> {
    let v3: u64 = f154(v0, v2)?;
    if if if f230(v3) {
        true
    } else {
        f231(v3)
    } {
        true
    } else {
        if v1 {
            let a3789 = v3;
            let a3790 = 62u64;
            nat_eq(a3789, a3790)
        } else {
            f435(v3)
        }
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3791 = v2;
            let a3792 = 1u64;
            nat_add(a3791, a3792)?
        }))
    }
}

pub fn f548(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    if f601(v1, f121(v0, v2)?) {
        Ok(Some({
            let a3793 = v2;
            let a3794 = 1u64;
            nat_add(a3793, a3794)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f549(p3797: List<Adt14>, p3798: List<Adt14>) -> List<Adt14> {
    let m3796 = (p3797, p3798);
    {
        let mut m3796 = m3796;
        loop {
            let step = {
                let (v0, v1) = m3796;
                let m3795 = v0;
                match m3795.uncons() {
                    None => {
                        Err::<(List<Adt14>, List<Adt14>), List<Adt14>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt14>, List<Adt14>), List<Adt14>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3796 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f550(v0: Bytes, p3803: List<(Bytes, Adt15)>) -> Option<Adt15> {
    let m3802 = p3803;
    {
        let mut m3802 = m3802;
        loop {
            let step = {
                let v1 = m3802;
                let m3799 = v1;
                match m3799.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt15)>, Option<Adt15>>(None::<Adt15>)
                    }
                    Some((v2, v3)) => {
                        if f46({
                            let (h3800, _) = v2;
                            h3800
                        }, v0.clone()) {
                            Err::<List<(Bytes, Adt15)>, Option<Adt15>>(Some({
                                let (_, h3801) = v2;
                                h3801
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt15)>, Option<Adt15>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3802 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f551() -> List<(Bytes, Adt15)> {
    List::cons((f8(Str::lit("boolean")), f366(f8(Str::lit("Bool")))), List::cons((f8(Str::lit("byte")), f366(f8(Str::lit("Int")))), List::cons((f8(Str::lit("octet")), f366(f8(Str::lit("Nat")))), List::cons((f8(Str::lit("bigint")), f366(f8(Str::lit("Int")))), List::cons((f8(Str::lit("float")), f366(f500())), List::cons((f8(Str::lit("double")), f366(f500())), List::cons((f8(Str::lit("DOMString")), f366(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("USVString")), f366(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ByteString")), f366(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ArrayBuffer")), f366(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("SharedArrayBuffer")), f366(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("DataView")), f366(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("Int8Array")), f366(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int16Array")), f366(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int32Array")), f366(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("BigInt64Array")), f366(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Uint8Array")), f366(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint16Array")), f366(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint32Array")), f366(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint8ClampedArray")), f366(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("BigUint64Array")), f366(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Float16Array")), f478(f8(Str::lit("Float16Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("Float32Array")), f478(f8(Str::lit("Float32Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("Float64Array")), f478(f8(Str::lit("Float64Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("any")), f478(f8(Str::lit("any is not translated, because it is the union of every Web IDL type and has no one LexLean type (Web IDL 2.13.1)")))), List::cons((f8(Str::lit("object")), f478(f8(Str::lit("object is not translated, because its values are references to JavaScript objects of any shape, which no LexLean type holds (Web IDL 2.13.20)")))), List::cons((f8(Str::lit("symbol")), f478(f8(Str::lit("symbol is not translated, because a symbol value has identity but no content that a LexLean type holds (Web IDL 2.13.21)")))), List::cons((f8(Str::lit("undefined")), f478(f8(Str::lit("undefined is not translated, because it is the type of the absence of a value (Web IDL 2.13.2)")))), List::<(Bytes, Adt15)>::nil()))))))))))))))))))))))))))))
}

pub fn f552() -> Bytes {
    f8(Str::lit("a union type is translated only as the whole body of a typedef, whose identifier names its inductive (Web IDL 2.11 and 2.13.32)"))
}

pub fn f553(v0: u64) -> Bytes {
    f43({
        let a3804 = Str::lit("0123456789ABCDEF");
        utf8_encode(a3804)
    }, v0, 1u64)
}

pub fn f554(p3815: u64, v1: Bytes, v2: Bytes, p3816: u64) -> R<bool> {
    let m3814 = (p3815, p3816);
    {
        let mut m3814 = m3814;
        loop {
            let step = {
                let (v0, v3) = m3814;
                let m3805 = v0;
                if m3805 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(false))
                } else {
                    let v4 = m3805 - 1;
                    let v5: u64 = f602(v1.clone(), v3)?;
                    let v6: u64 = f602(v2.clone(), v3)?;
                    let a3806 = v5;
                    let a3807 = v6;
                    if nat_lt(a3806, a3807) {
                        Err::<(u64, u64), R<bool>>(Ok(true))
                    } else {
                        let a3808 = v6;
                        let a3809 = v5;
                        if nat_lt(a3808, a3809) {
                            Err::<(u64, u64), R<bool>>(Ok(false))
                        } else {
                            let a3810 = v5;
                            let a3811 = 0u64;
                            if nat_eq(a3810, a3811) {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            } else {
                                Ok::<(u64, u64), R<bool>>((v4, {
                                    let a3812 = v3;
                                    let a3813 = 1u64;
                                    nat_add(a3812, a3813)?
                                }))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3814 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f555(p3829: u64, v1: Bytes, p3830: u64, p3831: Bytes) -> R<Option<Bytes>> {
    let m3828 = (p3829, (p3830, p3831));
    {
        let mut m3828 = m3828;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3828;
                let m3817 = v0;
                if m3817 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3817 - 1;
                    let a3819 = {
                        let a3818 = v1.clone();
                        length_bytes(a3818)
                    };
                    let a3820 = v2;
                    if nat_le(a3819, a3820) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3821 = f328(v1.clone(), v2)?;
                        match m3821 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3823 = v2;
                                    let a3824 = {
                                        let (_, h3822) = v5;
                                        h3822
                                    };
                                    nat_add(a3823, a3824)?
                                }, {
                                    let a3826 = v3;
                                    let a3827 = f603(v1.clone(), v2, {
                                        let (h3825, _) = v5;
                                        h3825
                                    });
                                    append_bytes(a3826, a3827)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3828 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f556(p3841: u64, v1: Bytes, p3842: u64) -> R<bool> {
    let m3840 = (p3841, p3842);
    {
        let mut m3840 = m3840;
        loop {
            let step = {
                let (v0, v2) = m3840;
                let m3832 = v0;
                if m3832 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m3832 - 1;
                    let m3835 = {
                        let a3833 = v1.clone();
                        let a3834 = v2;
                        index_bytes(a3833, a3834)
                    };
                    match m3835 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a3836 = f53(v4)?;
                            let a3837 = 128u64;
                            if nat_lt(a3836, a3837) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a3838 = v2;
                                    let a3839 = 1u64;
                                    nat_add(a3838, a3839)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3840 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f557(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f292(v0.clone(), 0u64, {
        let a3843 = v0;
        length_bytes(a3843)
    }, false, Bytes::lit(&[]))?;
    let m3846 = {
        let a3844 = v1.clone();
        let a3845 = 0u64;
        index_bytes(a3844, a3845)
    };
    match m3846 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f53(v2)?;
            if f398(v3) {
                let a3853 = f43({
                    let a3847 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a3847)
                }, {
                    let a3848 = v3;
                    let a3849 = 65u64;
                    nat_sub(a3848, a3849)
                }, 1u64);
                let a3854 = f43(v1.clone(), 1u64, {
                    let a3851 = {
                        let a3850 = v1;
                        length_bytes(a3850)
                    };
                    let a3852 = 1u64;
                    nat_sub(a3851, a3852)
                });
                Ok(append_bytes(a3853, a3854))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f558(v0: Bytes) -> R<bool> {
    let m3857 = {
        let a3855 = v0;
        let a3856 = 0u64;
        index_bytes(a3855, a3856)
    };
    match m3857 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f141(f397(f53(v1)?), f398(f53(v1)?)))
        }
    }
}

pub fn f559(p3870: u64, v1: Bytes, p3871: u64, p3872: Bytes) -> R<Option<Bytes>> {
    let m3869 = (p3870, (p3871, p3872));
    {
        let mut m3869 = m3869;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3869;
                let m3858 = v0;
                if m3858 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3858 - 1;
                    let a3860 = {
                        let a3859 = v1.clone();
                        length_bytes(a3859)
                    };
                    let a3861 = v2;
                    if nat_le(a3860, a3861) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3862 = f328(v1.clone(), v2)?;
                        match m3862 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3864 = v2;
                                    let a3865 = {
                                        let (_, h3863) = v5;
                                        h3863
                                    };
                                    nat_add(a3864, a3865)?
                                }, {
                                    let a3867 = v3;
                                    let a3868 = f604({
                                        let (h3866, _) = v5;
                                        h3866
                                    });
                                    append_bytes(a3867, a3868)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3869 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f560() -> List<Bytes> {
    List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("termination_by")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("admit")), List::cons(f8(Str::lit("below")), List::cons(f8(Str::lit("brecOn")), List::cons(f8(Str::lit("casesOn")), List::cons(f8(Str::lit("ctorElim")), List::cons(f8(Str::lit("ctorElimType")), List::cons(f8(Str::lit("ctorIdx")), List::cons(f8(Str::lit("noConfusion")), List::cons(f8(Str::lit("noConfusionType")), List::cons(f8(Str::lit("rec")), List::cons(f8(Str::lit("recOn")), List::cons(f8(Str::lit("toCtorIdx")), List::<Bytes>::nil())))))))))))))))))))
}

pub fn f561() -> Bytes {
    f8(Str::lit("\u{0}\u{0}:\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{5}\u{7f}\u{0}\u{6}p\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f562() -> Bytes {
    f8(Str::lit("\u{0}\u{0}-\u{0}\u{0}.\u{0}\u{0}0\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}7\u{0}\u{1}7\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@?\u{0}@@\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f563(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f121(v0, {
        let (h3873, _) = v2;
        h3873
    })?;
    let a3875 = 1114111u64;
    let a3876 = {
        let (_, h3874) = v2;
        h3874
    };
    if nat_lt(a3875, a3876) {
        Ok(None::<(u64, u64)>)
    } else {
        let v4: u64 = f605(v3, v1);
        let a3877 = v4;
        let a3878 = 16u64;
        if nat_eq(a3877, a3878) {
            Ok(None::<(u64, u64)>)
        } else {
            Ok(Some(({
                let a3880 = {
                    let (h3879, _) = v2;
                    h3879
                };
                let a3881 = 1u64;
                nat_add(a3880, a3881)?
            }, {
                let a3885 = {
                    let a3883 = {
                        let (_, h3882) = v2;
                        h3882
                    };
                    let a3884 = if v1 {
                        16u64
                    } else {
                        10u64
                    };
                    nat_mul(a3883, a3884)?
                };
                let a3886 = v4;
                nat_add(a3885, a3886)?
            })))
        }
    }
}

pub fn f564() -> Bytes {
    f8(Str::lit("\u{80}\u{81}\u{82}\u{83}\u{84}\u{85}\u{86}\u{87}\u{88}\u{89}\u{8a}\u{8b}\u{8c}\u{8d}\u{8e}\u{8f}\u{90}\u{91}\u{92}\u{93}\u{94}\u{95}\u{96}\u{97}\u{98}\u{99}\u{9a}\u{9b}\u{9c}\u{9d}\u{9e}\u{9f}\u{a0}\u{a1}\u{a2}\u{a3}\u{a4}\u{a5}\u{a6}\u{a7}\u{a8}\u{a9}\u{aa}\u{ab}\u{ac}\u{ad}\u{ae}\u{af}\u{b0}\u{b1}\u{b2}\u{b3}\u{b4}\u{b5}\u{b6}\u{b7}\u{b8}\u{b9}\u{ba}\u{bb}\u{bc}\u{bd}\u{be}\u{bf}"))
}

pub fn f565(v0: u64, v1: bool, v2: u64) -> R<bool> {
    if v1 {
        let a3887 = v0;
        let a3888 = v2;
        if nat_eq(a3887, a3888) {
            Ok(true)
        } else {
            f277(v0, f606())
        }
    } else {
        f277(v0, f607())
    }
}

pub fn f566() -> Bytes {
    f8(Str::lit(" string:anyAtomicType boolean:anyAtomicType decimal:anyAtomicType float:anyAtomicType double:anyAtomicType duration:anyAtomicType dateTime:anyAtomicType time:anyAtomicType date:anyAtomicType gYearMonth:anyAtomicType gYear:anyAtomicType gMonthDay:anyAtomicType gDay:anyAtomicType gMonth:anyAtomicType hexBinary:anyAtomicType base64Binary:anyAtomicType anyURI:anyAtomicType QName:anyAtomicType NOTATION:anyAtomicType normalizedString:string token:normalizedString language:token NMTOKEN:token Name:token NCName:Name ID:NCName IDREF:NCName ENTITY:NCName integer:decimal nonPositiveInteger:integer long:integer nonNegativeInteger:integer negativeInteger:nonPositiveInteger int:long short:int byte:short unsignedLong:nonNegativeInteger positiveInteger:nonNegativeInteger unsignedInt:unsignedLong unsignedShort:unsignedInt unsignedByte:unsignedShort yearMonthDuration:duration dayTimeDuration:duration dateTimeStamp:dateTime NMTOKENS:anySimpleType IDREFS:anySimpleType ENTITIES:anySimpleType anyAtomicType:anySimpleType "))
}

pub fn f567(v0: Adt26, v1: Str, v2: Str) -> bool {
    let m3889 = f310(v0, v1);
    match m3889 {
        None => {
            false
        }
        Some(v3) => {
            f46(v3, f8(v2))
        }
    }
}

pub fn f568(v0: List<Adt25>, v1: (Bytes, Bytes), v2: bool) -> R<Result<List<Bytes>, Bytes>> {
    let m3890 = v0;
    match m3890.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m3891 = f568(v4, v1.clone(), v2)?;
            match m3891 {
                Ok(v6) => {
                    if f46(match v3.clone() {
                        Adt25::C0(h3892, _, _, _) => {
                            h3892
                        }
                    }, f8(Str::lit("complexType"))) {
                        let m3893 = f608(v3.clone())?;
                        match m3893 {
                            Ok(v8) => {
                                let m3894 = v8;
                                match m3894 {
                                    None => {
                                        Ok(Ok::<List<Bytes>, Bytes>(v6))
                                    }
                                    Some(v9) => {
                                        if if f46({
                                            let (h3895, _) = {
                                                let (_, h3896) = v9.clone();
                                                h3896
                                            };
                                            h3895
                                        }, {
                                            let (h3897, _) = v1;
                                            h3897
                                        }) {
                                            if f46({
                                                let (_, h3898) = {
                                                    let (_, h3899) = v9;
                                                    h3899
                                                };
                                                h3898
                                            }, {
                                                let (_, h3900) = v1;
                                                h3900
                                            }) {
                                                if f308(v2) {
                                                    true
                                                } else {
                                                    f309({
                                                        let (h3901, _) = v9;
                                                        h3901
                                                    }, Str::lit("extension"))
                                                }
                                            } else {
                                                false
                                            }
                                        } else {
                                            false
                                        } {
                                            Ok(Ok::<List<Bytes>, Bytes>(List::cons(match v3 {
                                                Adt25::C0(_, _, h3902, _) => {
                                                    h3902
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

pub fn f569(v0: Bytes, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<Adt27, Bytes>> {
    let m3903 = v1.clone();
    match m3903.uncons() {
        None => {
            Ok(Err::<Adt27, Bytes>(f510(f8(Str::lit("a choice with no particle that can occur matches no element sequence, so it has no LexLean constructor (XSD 1.1 Part 1, section 3.8.4.3)")), v0)))
        }
        Some((_, _)) => {
            let m3904 = f609(v1)?;
            match m3904 {
                Ok(v7) => {
                    let v8: List<Bytes> = f610(v7, v2);
                    Ok(Ok::<Adt27, Bytes>(Adt27::C0(v0.clone(), v3, f269(v0, v8.clone()), v8)))
                }
                Err(v6) => {
                    Ok(Err::<Adt27, Bytes>(f510(v6, v0)))
                }
            }
        }
    }
}

pub fn f570(v0: List<Bytes>) -> R<List<Bytes>> {
    let m3905 = v0;
    match m3905.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f200(v1)?, f570(v2)?))
        }
    }
}

pub fn f571(v0: u64, v1: Adt24, v2: Bytes, v3: Bytes, v4: Adt26) -> R<Result<List<Adt27>, Bytes>> {
    let m3906 = v0;
    if m3906 == 0 {
        Ok(Err::<List<Adt27>, Bytes>(f510(f8(Str::lit("the complex types nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m3906 - 1;
        let m3907 = f611(v5, v1, v2.clone(), v3, v4, f513())?;
        match m3907 {
            Ok(v7) => {
                let m3909 = f579(v2, match v7.clone() {
                    Adt33::C0(h3908, _, _, _, _) => {
                        h3908
                    }
                })?;
                match m3909 {
                    Ok(v9) => {
                        Ok(Ok::<List<Adt27>, Bytes>(f404(match v7 {
                            Adt33::C0(_, h3910, _, _, _) => {
                                h3910
                            }
                        }, List::cons(v9, List::<Adt27>::nil()))))
                    }
                    Err(v8) => {
                        Ok(Err::<List<Adt27>, Bytes>(v8))
                    }
                }
            }
            Err(v6) => {
                Ok(Err::<List<Adt27>, Bytes>(v6))
            }
        }
    }
}

pub fn f572(v0: Bytes) -> Bytes {
    let a3911 = v0;
    let a3912 = f8(Str::lit("Content"));
    append_bytes(a3911, a3912)
}

pub fn f573(v0: Adt24, v1: Adt26, v2: Bytes) -> R<Result<Adt23, Bytes>> {
    let m3914 = f125(match v1 {
        Adt26::C0(_, _, _, _, h3913) => {
            h3913
        }
    }, v2.clone())?;
    match m3914 {
        Ok(v4) => {
            if f46({
                let (h3915, _) = v4.clone();
                h3915
            }, f197()) {
                if f46({
                    let (_, h3916) = v4.clone();
                    h3916
                }, f8(Str::lit("anyType"))) {
                    Ok(Ok::<Adt23, Bytes>(Adt23::C0(f407(), List::cons(f407(), List::<Bytes>::nil()))))
                } else {
                    let m3918 = f394({
                        let (_, h3917) = v4;
                        h3917
                    }, 16u64)?;
                    match m3918 {
                        Ok(v6) => {
                            Ok(Ok::<Adt23, Bytes>(Adt23::C0(v6, List::<Bytes>::nil())))
                        }
                        Err(v5) => {
                            Ok(Err::<Adt23, Bytes>(v5))
                        }
                    }
                }
            } else {
                if f167({
                    let (h3919, _) = v4.clone();
                    h3919
                }, match v0.clone() {
                    Adt24::C0(_, h3920) => {
                        h3920
                    }
                }) {
                    let v7: Bytes = f200({
                        let (_, h3921) = v4.clone();
                        h3921
                    })?;
                    let m3922 = f580(v0.clone(), Str::lit("complexType"), v4.clone());
                    match m3922 {
                        None => {
                            let m3923 = f580(v0, Str::lit("simpleType"), v4);
                            match m3923 {
                                None => {
                                    Ok(Err::<Adt23, Bytes>(f209(f8(Str::lit("the type ")), v2, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)")))))
                                }
                                Some(_) => {
                                    Ok(Ok::<Adt23, Bytes>(Adt23::C0(v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                                }
                            }
                        }
                        Some(_) => {
                            Ok(Ok::<Adt23, Bytes>(Adt23::C0(v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                        }
                    }
                } else {
                    Ok(Err::<Adt23, Bytes>(f209(f8(Str::lit("the type ")), v2, f209(f8(Str::lit(" is in the namespace ")), {
                        let (h3924, _) = v4;
                        h3924
                    }, f8(Str::lit(", which no schema document of this set targets (XSD 1.1 Part 1, sections 3.17.6.2 and 4.2.6)"))))))
                }
            }
        }
        Err(v3) => {
            Ok(Err::<Adt23, Bytes>(v3))
        }
    }
}

pub fn f574(v0: Adt26, v1: Bytes) -> Bytes {
    if f567(v0, Str::lit("nillable"), Str::lit("true")) {
        f612(Str::lit("Option"), v1)
    } else {
        v1
    }
}

pub fn f575(p3929: u64, v1: Adt24, v2: Bytes, v3: Bytes, v4: bool, p3930: List<Adt26>, p3931: Adt33) -> R<Result<Adt33, Bytes>> {
    let m3928 = (p3929, (p3930, p3931));
    {
        let mut m3928 = m3928;
        loop {
            let step = {
                let (v0, (v5, v6)) = m3928;
                let m3925 = v0;
                if m3925 == 0 {
                    Err::<(u64, (List<Adt26>, Adt33)), R<Result<Adt33, Bytes>>>(Ok(Err::<Adt33, Bytes>(f510(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2.clone()))))
                } else {
                    let v7 = m3925 - 1;
                    let m3926 = v5;
                    match m3926.uncons() {
                        None => {
                            Err::<(u64, (List<Adt26>, Adt33)), R<Result<Adt33, Bytes>>>(Ok(Ok::<Adt33, Bytes>(v6)))
                        }
                        Some((v8, v9)) => {
                            let m3927 = f613(v7, v1.clone(), v2.clone(), v3.clone(), v4, v8, v6)?;
                            match m3927 {
                                Ok(v11) => {
                                    Ok::<(u64, (List<Adt26>, Adt33)), R<Result<Adt33, Bytes>>>((v7, (v9, v11)))
                                }
                                Err(v10) => {
                                    Err::<(u64, (List<Adt26>, Adt33)), R<Result<Adt33, Bytes>>>(Ok(Err::<Adt33, Bytes>(v10)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3928 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f576(v0: List<Adt34>) -> List<Bytes> {
    let m3932 = v0;
    match m3932.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt34::C0(h3933, _, _) => {
                    h3933
                }
            }, f576(v2))
        }
    }
}

pub fn f577(v0: List<Adt34>) -> List<Bytes> {
    let m3934 = v0;
    match m3934.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt34::C0(_, h3935, _) => {
                    h3935
                }
            }, f577(v2))
        }
    }
}

pub fn f578(v0: List<Adt34>) -> List<Bytes> {
    let m3936 = v0;
    match m3936.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f414(match v1 {
                Adt34::C0(_, _, h3937) => {
                    h3937
                }
            }, f578(v2))
        }
    }
}

pub fn f579(v0: Bytes, v1: List<Adt34>) -> R<Result<Adt27, Bytes>> {
    let m3938 = f609(f576(v1.clone()))?;
    match m3938 {
        Ok(v3) => {
            let v4: List<Bytes> = f577(v1.clone());
            let v7: Bytes = {
                let m3939 = v1.clone();
                match m3939.uncons() {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some((_, _)) => {
                        f209(f8(Str::lit("\u{a}")), f114(f614(v3, v4.clone()), f8(Str::lit(",\u{a}"))), f8(Str::lit("\u{a}  ")))
                    }
                }
            };
            Ok(Ok::<Adt27, Bytes>(Adt27::C0(v0.clone(), f578(v1), f209(f8(Str::lit("structure{\u{a}  name: ")), v0, f209(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), v7, f8(Str::lit("]\u{a}}\u{a}")))), List::cons(f209(f8(Str::lit("    {name: mk, fields: [")), f114(v4, f8(Str::lit(", "))), f8(Str::lit("]}"))), List::<Bytes>::nil()))))
        }
        Err(v2) => {
            Ok(Err::<Adt27, Bytes>(f510(v2, v0)))
        }
    }
}

pub fn f580(v0: Adt24, v1: Str, v2: (Bytes, Bytes)) -> Option<Adt25> {
    f615(match v0 {
        Adt24::C0(h3940, _) => {
            h3940
        }
    }, f8(v1), {
        let (h3941, _) = v2;
        h3941
    }, {
        let (_, h3942) = v2;
        h3942
    })
}

pub fn f581(v0: Adt33, v1: Bytes, v2: Adt32, v3: bool) -> Adt33 {
    f616(f582(v0, Adt34::C0(v1, if v3 {
        let m3945 = match v2.clone() {
            Adt32::C0(h3944, _) => {
                h3944
            }
        };
        match m3945 {
            Adt23::C0(h3943, _) => {
                h3943
            }
        }
    } else {
        f612(Str::lit("Option"), {
            let m3948 = match v2.clone() {
                Adt32::C0(h3947, _) => {
                    h3947
                }
            };
            match m3948 {
                Adt23::C0(h3946, _) => {
                    h3946
                }
            }
        })
    }, {
        let m3951 = match v2.clone() {
            Adt32::C0(h3950, _) => {
                h3950
            }
        };
        match m3951 {
            Adt23::C0(_, h3949) => {
                h3949
            }
        }
    })), match v2 {
        Adt32::C0(_, h3952) => {
            h3952
        }
    })
}

pub fn f582(v0: Adt33, v1: Adt34) -> Adt33 {
    Adt33::C0(f617(match v0.clone() {
        Adt33::C0(h3953, _, _, _, _) => {
            h3953
        }
    }, List::cons(v1, List::<Adt34>::nil())), match v0.clone() {
        Adt33::C0(_, h3954, _, _, _) => {
            h3954
        }
    }, match v0.clone() {
        Adt33::C0(_, _, h3955, _, _) => {
            h3955
        }
    }, match v0.clone() {
        Adt33::C0(_, _, _, h3956, _) => {
            h3956
        }
    }, match v0 {
        Adt33::C0(_, _, _, _, h3957) => {
            h3957
        }
    })
}

pub fn f583() -> Bytes {
    f8(Str::lit("(List ((Bytes * Bytes) * Bytes))"))
}

pub fn f584(v0: Bytes, v1: List<Bytes>, v2: Bytes) -> Bytes {
    f209(f8(Str::lit("inductive{\u{a}  name: ")), v0, f209(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), f114(v1, f8(Str::lit(",\u{a}"))), f209(f8(Str::lit("\u{a}  ],\u{a}  mutual: ")), v2, f8(Str::lit("\u{a}}\u{a}")))))
}

pub fn f585(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4002 = {
            let a3978 = {
                let a3966 = {
                    let a3960 = {
                        let a3958 = v1;
                        let a3959 = 128u8;
                        fixed_u8::bit_and(a3958, a3959)
                    };
                    let a3961 = 128u8;
                    if equal(a3960, a3961) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3967 = {
                    let a3964 = {
                        let a3962 = v1;
                        let a3963 = 64u8;
                        fixed_u8::bit_and(a3962, a3963)
                    };
                    let a3965 = 64u8;
                    if equal(a3964, a3965) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3966, a3967)?
            };
            let a3979 = {
                let a3976 = {
                    let a3970 = {
                        let a3968 = v1;
                        let a3969 = 32u8;
                        fixed_u8::bit_and(a3968, a3969)
                    };
                    let a3971 = 32u8;
                    if equal(a3970, a3971) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3977 = {
                    let a3974 = {
                        let a3972 = v1;
                        let a3973 = 16u8;
                        fixed_u8::bit_and(a3972, a3973)
                    };
                    let a3975 = 16u8;
                    if equal(a3974, a3975) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3976, a3977)?
            };
            nat_add(a3978, a3979)?
        };
        let a4003 = {
            let a4000 = {
                let a3988 = {
                    let a3982 = {
                        let a3980 = v1;
                        let a3981 = 8u8;
                        fixed_u8::bit_and(a3980, a3981)
                    };
                    let a3983 = 8u8;
                    if equal(a3982, a3983) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3989 = {
                    let a3986 = {
                        let a3984 = v1;
                        let a3985 = 4u8;
                        fixed_u8::bit_and(a3984, a3985)
                    };
                    let a3987 = 4u8;
                    if equal(a3986, a3987) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3988, a3989)?
            };
            let a4001 = {
                let a3998 = {
                    let a3992 = {
                        let a3990 = v1;
                        let a3991 = 2u8;
                        fixed_u8::bit_and(a3990, a3991)
                    };
                    let a3993 = 2u8;
                    if equal(a3992, a3993) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3999 = {
                    let a3996 = {
                        let a3994 = v1;
                        let a3995 = 1u8;
                        fixed_u8::bit_and(a3994, a3995)
                    };
                    let a3997 = 1u8;
                    if equal(a3996, a3997) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3998, a3999)?
            };
            nat_add(a4000, a4001)?
        };
        nat_add(a4002, a4003)?
    };
    let a4010 = f157(f618(), {
        let a4004 = v2;
        let a4005 = 16u64;
        let a4006 = 0u64;
        nat_quot(a4004, a4005, a4006)
    });
    let a4011 = f157(f618(), {
        let a4007 = v2;
        let a4008 = 16u64;
        let a4009 = 0u64;
        nat_rem(a4007, a4008, a4009)
    });
    Ok(append_bytes(a4010, a4011))
}

pub fn f586(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f619(Fn12::F620, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f587(v0: Fn10, p4015: List<List<(Bytes, Bytes)>>, p4016: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m4014 = (p4015, p4016);
    {
        let mut m4014 = m4014;
        loop {
            let step = {
                let (v1, v2) = m4014;
                let m4012 = v2;
                match m4012.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c4013: Fn10 = v0.clone();
                            c4013.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m4014 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f588(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f589(v0: Fn11, p4021: u64, p4022: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m4020 = (p4021, p4022);
    {
        let mut m4020 = m4020;
        loop {
            let step = {
                let (v1, v2) = m4020;
                let m4017 = v1;
                if m4017 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4017 - 1;
                    let m4019 = {
                        let c4018: Fn11 = v0.clone();
                        c4018.apply(v2.clone())?
                    };
                    match m4019 {
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
                Ok(next) => { m4020 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f590(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f621(v0)
}

pub fn f591(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h4023, _) = v1;
        h4023
    };
    let m4025 = {
        let (_, h4024) = v1;
        h4024
    };
    match m4025 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m4028 = {
                let a4026 = v0.clone();
                let a4027 = v2;
                index_bytes(a4026, a4027)
            };
            match m4028 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m4033 = {
                        let a4031 = v0;
                        let a4032 = {
                            let a4029 = v2;
                            let a4030 = 1u64;
                            nat_add(a4029, a4030)?
                        };
                        index_bytes(a4031, a4032)
                    };
                    match m4033 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f622(v4)? {
                                f622(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a4034 = v2;
                                    let a4035 = 2u64;
                                    nat_add(a4034, a4035)?
                                }, Some({
                                    let a4040 = v3;
                                    let a4041 = f157(f623(), {
                                        let a4038 = {
                                            let a4036 = f624(v4)?;
                                            let a4037 = 16u64;
                                            nat_mul(a4036, a4037)?
                                        };
                                        let a4039 = f624(v5)?;
                                        nat_add(a4038, a4039)?
                                    });
                                    append_bytes(a4040, a4041)
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

pub fn f592(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f625(v1, v0, v2)
}

pub fn f593(v0: u64, v1: Bytes, v2: u64, v3: u64) -> R<Option<u64>> {
    f626(v1, v2, v0, v3)
}

pub fn f594(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let a4044 = {
        let (h4042, _) = v1;
        h4042
    };
    let a4045 = {
        let a4043 = v0.clone();
        length_bytes(a4043)
    };
    if nat_lt(a4044, a4045) {
        Ok(Some((f234(v0, {
            let (h4046, _) = v1;
            h4046
        })?, {
            let a4048 = {
                let (_, h4047) = v1;
                h4047
            };
            let a4049 = 1u64;
            nat_add(a4048, a4049)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f595(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f627(v1, v0, v2)
}

pub fn f596(v0: u64) -> Option<u64> {
    let a4050 = 48u64;
    let a4051 = v0;
    if if nat_le(a4050, a4051) {
        let a4052 = v0;
        let a4053 = 57u64;
        nat_le(a4052, a4053)
    } else {
        false
    } {
        Some({
            let a4054 = v0;
            let a4055 = 48u64;
            nat_sub(a4054, a4055)
        })
    } else {
        let a4056 = 97u64;
        let a4057 = v0;
        if if nat_le(a4056, a4057) {
            let a4058 = v0;
            let a4059 = 102u64;
            nat_le(a4058, a4059)
        } else {
            false
        } {
            Some({
                let a4060 = v0;
                let a4061 = 87u64;
                nat_sub(a4060, a4061)
            })
        } else {
            let a4062 = 65u64;
            let a4063 = v0;
            if if nat_le(a4062, a4063) {
                let a4064 = v0;
                let a4065 = 70u64;
                nat_le(a4064, a4065)
            } else {
                false
            } {
                Some({
                    let a4066 = v0;
                    let a4067 = 55u64;
                    nat_sub(a4066, a4067)
                })
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f597(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f628(v0, v1)
}

pub fn f598(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f629(v0, v1)
}

pub fn f599(v0: Bytes, v1: u64, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    let v3: u64 = {
        let (h4068, _) = v2;
        h4068
    };
    let v4: u64 = f154(v0.clone(), v3)?;
    let (_, h4069) = v2;
    if h4069 {
        Ok(None::<(u64, bool)>)
    } else {
        if f230(v4) {
            Ok(Some(({
                let a4070 = v3;
                let a4071 = 1u64;
                nat_add(a4070, a4071)?
            }, false)))
        } else {
            let a4072 = v4;
            let a4073 = 35u64;
            if if nat_eq(a4072, a4073) {
                let a4074 = v3;
                let a4075 = 0u64;
                if if nat_eq(a4074, a4075) {
                    true
                } else {
                    f230(f154(v0.clone(), {
                        let a4076 = v3;
                        let a4077 = 1u64;
                        nat_sub(a4076, a4077)
                    })?)
                } {
                    true
                } else {
                    let a4080 = f154(v0.clone(), {
                        let a4078 = v3;
                        let a4079 = 1u64;
                        nat_sub(a4078, a4079)
                    })?;
                    let a4081 = 10u64;
                    nat_eq(a4080, a4081)
                }
            } else {
                false
            } {
                Ok(Some((f333(v0, v3)?, false)))
            } else {
                let a4082 = v4;
                let a4083 = 10u64;
                if nat_eq(a4082, a4083) {
                    let v5: u64 = {
                        let a4084 = v3;
                        let a4085 = 1u64;
                        nat_add(a4084, a4085)?
                    };
                    let v6: u64 = f235(v0.clone(), v5)?;
                    let v7: u64 = f233(v0.clone(), v6)?;
                    if if f152(v0.clone(), v5)? {
                        true
                    } else {
                        let a4088 = {
                            let a4086 = v6;
                            let a4087 = v5;
                            nat_sub(a4086, a4087)
                        };
                        let a4089 = v1;
                        if if nat_lt(a4088, a4089) {
                            f156(f231(f154(v0.clone(), v7)?))
                        } else {
                            false
                        } {
                            f156({
                                let a4090 = f154(v0, v7)?;
                                let a4091 = 35u64;
                                nat_eq(a4090, a4091)
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

pub fn f600(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f233(v0.clone(), {
        let (h4092, _) = v1;
        h4092
    })?;
    let a4093 = f154(v0, v2)?;
    let a4094 = 10u64;
    if nat_eq(a4093, a4094) {
        Ok(Some(({
            let a4095 = v2;
            let a4096 = 1u64;
            nat_add(a4095, a4096)?
        }, {
            let a4098 = {
                let (_, h4097) = v1;
                h4097
            };
            let a4099 = 1u64;
            nat_add(a4098, a4099)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f601(v0: u64, v1: u64) -> bool {
    let a4100 = v0;
    let a4101 = 0u64;
    if nat_eq(a4100, a4101) {
        f464(v1)
    } else {
        let a4102 = v0;
        let a4103 = 1u64;
        if nat_eq(a4102, a4103) {
            f463(v1)
        } else {
            let a4104 = v0;
            let a4105 = 2u64;
            if nat_eq(a4104, a4105) {
                f630(v1)
            } else {
                f631(v1)
            }
        }
    }
}

pub fn f602(v0: Bytes, v1: u64) -> R<u64> {
    let a4107 = v1;
    let a4108 = {
        let a4106 = v0.clone();
        length_bytes(a4106)
    };
    if nat_lt(a4107, a4108) {
        let a4109 = f121(v0, v1)?;
        let a4110 = 1u64;
        nat_add(a4109, a4110)
    } else {
        Ok(0u64)
    }
}

pub fn f603(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f141(f87(f141(f397(v2), f398(v2)), f87({
        let a4111 = v2;
        let a4112 = 117u64;
        !nat_eq(a4111, a4112)
    }, {
        let a4113 = v2;
        let a4114 = 85u64;
        !nat_eq(a4113, a4114)
    })), f87(f399(v2), {
        let a4115 = 0u64;
        let a4116 = v1;
        nat_lt(a4115, a4116)
    })) {
        f43(v0, v1, 1u64)
    } else {
        let a4117 = v2;
        let a4118 = 65536u64;
        if nat_lt(a4117, a4118) {
            let a4119 = f8(Str::lit("u"));
            let a4120 = f482(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a4119, a4120)
        } else {
            let a4121 = f8(Str::lit("U"));
            let a4122 = f482(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a4121, a4122)
        }
    }
}

pub fn f604(v0: u64) -> Bytes {
    let a4123 = f8(Str::lit("u"));
    let a4124 = f482(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a4123, a4124)
}

pub fn f605(v0: u64, v1: bool) -> u64 {
    let a4125 = v0;
    let a4126 = 48u64;
    if nat_lt(a4125, a4126) {
        16u64
    } else {
        let a4127 = v0;
        let a4128 = 57u64;
        if nat_le(a4127, a4128) {
            let a4129 = v0;
            let a4130 = 48u64;
            nat_sub(a4129, a4130)
        } else {
            if v1 {
                let a4131 = v0;
                let a4132 = 65u64;
                if nat_lt(a4131, a4132) {
                    16u64
                } else {
                    let a4133 = v0;
                    let a4134 = 70u64;
                    if nat_le(a4133, a4134) {
                        let a4135 = v0;
                        let a4136 = 55u64;
                        nat_sub(a4135, a4136)
                    } else {
                        let a4137 = v0;
                        let a4138 = 97u64;
                        if nat_lt(a4137, a4138) {
                            16u64
                        } else {
                            let a4139 = v0;
                            let a4140 = 102u64;
                            if nat_le(a4139, a4140) {
                                let a4141 = v0;
                                let a4142 = 87u64;
                                nat_sub(a4141, a4142)
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

pub fn f606() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f607() -> Bytes {
    f8(Str::lit("\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{0}]\u{0}\u{0}]\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f608(v0: Adt25) -> R<Result<Option<(Adt26, (Bytes, Bytes))>, Bytes>> {
    let m4144 = f508(match v0 {
        Adt25::C0(_, _, _, h4143) => {
            h4143
        }
    })?;
    match m4144 {
        Ok(v2) => {
            let m4145 = f509(v2, List::cons(f8(Str::lit("simpleContent")), List::cons(f8(Str::lit("complexContent")), List::<Bytes>::nil())));
            match m4145 {
                None => {
                    Ok(Ok::<Option<(Adt26, (Bytes, Bytes))>, Bytes>(None::<(Adt26, (Bytes, Bytes))>))
                }
                Some(v3) => {
                    let m4146 = f508(v3)?;
                    match m4146 {
                        Ok(v5) => {
                            let m4147 = f509(v5, List::cons(f8(Str::lit("extension")), List::cons(f8(Str::lit("restriction")), List::<Bytes>::nil())));
                            match m4147 {
                                None => {
                                    Ok(Ok::<Option<(Adt26, (Bytes, Bytes))>, Bytes>(None::<(Adt26, (Bytes, Bytes))>))
                                }
                                Some(v6) => {
                                    let m4148 = f310(v6.clone(), Str::lit("base"));
                                    match m4148 {
                                        None => {
                                            Ok(Ok::<Option<(Adt26, (Bytes, Bytes))>, Bytes>(None::<(Adt26, (Bytes, Bytes))>))
                                        }
                                        Some(v7) => {
                                            let m4150 = f125(match v6.clone() {
                                                Adt26::C0(_, _, _, _, h4149) => {
                                                    h4149
                                                }
                                            }, v7)?;
                                            match m4150 {
                                                Ok(v9) => {
                                                    Ok(Ok::<Option<(Adt26, (Bytes, Bytes))>, Bytes>(Some((v6, v9))))
                                                }
                                                Err(v8) => {
                                                    Ok(Err::<Option<(Adt26, (Bytes, Bytes))>, Bytes>(v8))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Err(v4) => {
                            Ok(Err::<Option<(Adt26, (Bytes, Bytes))>, Bytes>(v4))
                        }
                    }
                }
            }
        }
        Err(v1) => {
            Ok(Err::<Option<(Adt26, (Bytes, Bytes))>, Bytes>(v1))
        }
    }
}

pub fn f609(v0: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f632(f263(v0.clone())?, v0, List::<Bytes>::nil())
}

pub fn f610(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m4151 = v0;
    match m4151.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m4152 = v1;
            match m4152.uncons() {
                None => {
                    List::<Bytes>::nil()
                }
                Some((v4, v5)) => {
                    List::cons(f209(f8(Str::lit("    {name: ")), v2, f209(f8(Str::lit(", fields: [")), v4, f8(Str::lit("]}")))), f610(v3, v5))
                }
            }
        }
    }
}

pub fn f611(v0: u64, v1: Adt24, v2: Bytes, v3: Bytes, v4: Adt26, v5: Adt33) -> R<Result<Adt33, Bytes>> {
    let m4153 = v0;
    if m4153 == 0 {
        Ok(Err::<Adt33, Bytes>(f510(f8(Str::lit("the type derivations nest deeper than the schema documents have bytes, so the types derive from each other in a cycle (XSD 1.1 Part 1, section 3.4.6.1)")), v2)))
    } else {
        let v6 = m4153 - 1;
        if f567(v4.clone(), Str::lit("mixed"), Str::lit("true")) {
            Ok(Err::<Adt33, Bytes>(f510(f8(Str::lit("a mixed content type interleaves character data with its elements, which a LexLean field list does not keep in order, so it is not translated (XSD 1.1 Part 1, section 3.4.2.3)")), v2)))
        } else {
            let m4154 = f508(v4)?;
            match m4154 {
                Ok(v8) => {
                    let m4155 = f509(v8.clone(), List::cons(f8(Str::lit("openContent")), List::cons(f8(Str::lit("assert")), List::<Bytes>::nil())));
                    match m4155 {
                        None => {
                            let m4157 = f509(v8.clone(), List::cons(f8(Str::lit("simpleContent")), List::cons(f8(Str::lit("complexContent")), List::<Bytes>::nil())));
                            match m4157 {
                                None => {
                                    let m4158 = f575(v6, v1.clone(), v2.clone(), v3.clone(), true, v8.clone(), v5)?;
                                    match m4158 {
                                        Ok(v11) => {
                                            f512(v6, v1, v2, v3, v8, v11)
                                        }
                                        Err(v10) => {
                                            Ok(Err::<Adt33, Bytes>(v10))
                                        }
                                    }
                                }
                                Some(v12) => {
                                    if f567(v12.clone(), Str::lit("mixed"), Str::lit("true")) {
                                        Ok(Err::<Adt33, Bytes>(f510(f8(Str::lit("a mixed content type interleaves character data with its elements, which a LexLean field list does not keep in order, so it is not translated (XSD 1.1 Part 1, section 3.4.2.3)")), v2)))
                                    } else {
                                        let m4159 = f508(v12.clone())?;
                                        match m4159 {
                                            Ok(v14) => {
                                                let m4160 = f509(v14, List::cons(f8(Str::lit("extension")), List::cons(f8(Str::lit("restriction")), List::<Bytes>::nil())));
                                                match m4160 {
                                                    None => {
                                                        Ok(Err::<Adt33, Bytes>(f510(f8(Str::lit("simpleContent and complexContent must have an extension or a restriction child (XSD 1.1 Part 1, sections 3.4.2.2 and 3.4.2.3)")), v2)))
                                                    }
                                                    Some(v15) => {
                                                        let m4161 = f310(v15.clone(), Str::lit("base"));
                                                        match m4161 {
                                                            None => {
                                                                Ok(Err::<Adt33, Bytes>(f510(f8(Str::lit("an extension or restriction must name its base type (XSD 1.1 Part 1, section 3.4.3)")), v2)))
                                                            }
                                                            Some(v16) => {
                                                                let m4163 = f125(match v15.clone() {
                                                                    Adt26::C0(_, _, _, _, h4162) => {
                                                                        h4162
                                                                    }
                                                                }, v16.clone())?;
                                                                match m4163 {
                                                                    Ok(v18) => {
                                                                        let m4164 = f508(v15.clone())?;
                                                                        match m4164 {
                                                                            Ok(v20) => {
                                                                                let v21: bool = if f46({
                                                                                    let (h4165, _) = v18.clone();
                                                                                    h4165
                                                                                }, f197()) {
                                                                                    f46({
                                                                                        let (_, h4166) = v18.clone();
                                                                                        h4166
                                                                                    }, f8(Str::lit("anyType")))
                                                                                } else {
                                                                                    false
                                                                                };
                                                                                if if f309(v15.clone(), Str::lit("restriction")) {
                                                                                    f308(v21)
                                                                                } else {
                                                                                    false
                                                                                } {
                                                                                    Ok(Err::<Adt33, Bytes>(f510(f209(f8(Str::lit("a restriction of the complex type ")), v16, f8(Str::lit(" is not translated, because the attribute uses it inherits and the content it narrows are not read yet (XSD 1.1 Part 1, sections 3.4.2.2 and 3.4.2.3)"))), v2)))
                                                                                } else {
                                                                                    let m4181 = if if v21 {
                                                                                        true
                                                                                    } else {
                                                                                        f46(match v12 {
                                                                                            Adt26::C0(_, h4167, _, _, _) => {
                                                                                                h4167
                                                                                            }
                                                                                        }, f8(Str::lit("complexContent")))
                                                                                    } {
                                                                                        if v21 {
                                                                                            Ok::<Adt33, Bytes>(v5)
                                                                                        } else {
                                                                                            let m4168 = f580(v1.clone(), Str::lit("complexType"), v18.clone());
                                                                                            match m4168 {
                                                                                                None => {
                                                                                                    Err::<Adt33, Bytes>(f510(f209(f8(Str::lit("the base type ")), v16, f8(Str::lit(" of a complex content extension is not a complex type of this schema set (XSD 1.1 Part 1, section 3.4.2.3)"))), v2.clone()))
                                                                                                }
                                                                                                Some(v22) => {
                                                                                                    let m4172 = f611(v6, v1.clone(), f200({
                                                                                                        let (_, h4169) = v18;
                                                                                                        h4169
                                                                                                    })?, match v22.clone() {
                                                                                                        Adt25::C0(_, h4170, _, _) => {
                                                                                                            h4170
                                                                                                        }
                                                                                                    }, match v22 {
                                                                                                        Adt25::C0(_, _, _, h4171) => {
                                                                                                            h4171
                                                                                                        }
                                                                                                    }, v5)?;
                                                                                                    match m4172 {
                                                                                                        Ok(v24) => {
                                                                                                            Ok::<Adt33, Bytes>(f633(v24))
                                                                                                        }
                                                                                                        Err(v23) => {
                                                                                                            Err::<Adt33, Bytes>(v23)
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        let m4173 = f580(v1.clone(), Str::lit("complexType"), v18.clone());
                                                                                        match m4173 {
                                                                                            None => {
                                                                                                let m4178 = f573(v1.clone(), v15, v16)?;
                                                                                                match m4178 {
                                                                                                    Ok(v29) => {
                                                                                                        Ok::<Adt33, Bytes>(f582(v5, Adt34::C0(f8(Str::lit("value")), match v29.clone() {
                                                                                                            Adt23::C0(h4179, _) => {
                                                                                                                h4179
                                                                                                            }
                                                                                                        }, match v29 {
                                                                                                            Adt23::C0(_, h4180) => {
                                                                                                                h4180
                                                                                                            }
                                                                                                        })))
                                                                                                    }
                                                                                                    Err(v28) => {
                                                                                                        Err::<Adt33, Bytes>(f510(v28, v2.clone()))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            Some(v25) => {
                                                                                                let m4177 = f611(v6, v1.clone(), f200({
                                                                                                    let (_, h4174) = v18;
                                                                                                    h4174
                                                                                                })?, match v25.clone() {
                                                                                                    Adt25::C0(_, h4175, _, _) => {
                                                                                                        h4175
                                                                                                    }
                                                                                                }, match v25 {
                                                                                                    Adt25::C0(_, _, _, h4176) => {
                                                                                                        h4176
                                                                                                    }
                                                                                                }, v5)?;
                                                                                                match m4177 {
                                                                                                    Ok(v27) => {
                                                                                                        Ok::<Adt33, Bytes>(f633(v27))
                                                                                                    }
                                                                                                    Err(v26) => {
                                                                                                        Err::<Adt33, Bytes>(v26)
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    };
                                                                                    match m4181 {
                                                                                        Ok(v31) => {
                                                                                            let m4182 = f575(v6, v1.clone(), v2.clone(), v3.clone(), true, v20.clone(), v31)?;
                                                                                            match m4182 {
                                                                                                Ok(v33) => {
                                                                                                    f512(v6, v1, v2, v3, v20, v33)
                                                                                                }
                                                                                                Err(v32) => {
                                                                                                    Ok(Err::<Adt33, Bytes>(v32))
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        Err(v30) => {
                                                                                            Ok(Err::<Adt33, Bytes>(v30))
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            Err(v19) => {
                                                                                Ok(Err::<Adt33, Bytes>(v19))
                                                                            }
                                                                        }
                                                                    }
                                                                    Err(v17) => {
                                                                        Ok(Err::<Adt33, Bytes>(f510(v17, v2)))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            Err(v13) => {
                                                Ok(Err::<Adt33, Bytes>(v13))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v9) => {
                            Ok(Err::<Adt33, Bytes>(f510(f209(f8(Str::lit("xs:")), match v9 {
                                Adt26::C0(_, h4156, _, _, _) => {
                                    h4156
                                }
                            }, f8(Str::lit(" is not translated, because an open content wildcard or an XPath assertion constrains an instance beyond its fields (XSD 1.1 Part 1, sections 3.4.2.3 and 3.13)"))), v2)))
                        }
                    }
                }
                Err(v7) => {
                    Ok(Err::<Adt33, Bytes>(v7))
                }
            }
        }
    }
}

pub fn f612(v0: Str, v1: Bytes) -> Bytes {
    f209(f8(Str::lit("(")), f8(v0), f209(f8(Str::lit(" ")), v1, f8(Str::lit(")"))))
}

pub fn f613(v0: u64, v1: Adt24, v2: Bytes, v3: Bytes, v4: bool, v5: Adt26, v6: Adt33) -> R<Result<Adt33, Bytes>> {
    let m4183 = v0;
    if m4183 == 0 {
        Ok(Err::<Adt33, Bytes>(f510(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v7 = m4183 - 1;
        if f309(v5.clone(), Str::lit("element")) {
            let m4184 = f634(v7, v1, v2, v3, v5.clone())?;
            match m4184 {
                Ok(v9) => {
                    let m4190 = f635(v5, {
                        let m4189 = {
                            let m4188 = match v9.clone() {
                                Adt37::C0(_, h4187) => {
                                    h4187
                                }
                            };
                            match m4188 {
                                Adt32::C0(h4186, _) => {
                                    h4186
                                }
                            }
                        };
                        match m4189 {
                            Adt23::C0(h4185, _) => {
                                h4185
                            }
                        }
                    });
                    match m4190 {
                        None => {
                            Ok(Ok::<Adt33, Bytes>(v6))
                        }
                        Some(v10) => {
                            Ok(Ok::<Adt33, Bytes>(f616(f582(v6, Adt34::C0(match v9.clone() {
                                Adt37::C0(h4191, _) => {
                                    h4191
                                }
                            }, v10, {
                                let m4196 = {
                                    let m4195 = match v9.clone() {
                                        Adt37::C0(_, h4194) => {
                                            h4194
                                        }
                                    };
                                    match m4195 {
                                        Adt32::C0(h4193, _) => {
                                            h4193
                                        }
                                    }
                                };
                                match m4196 {
                                    Adt23::C0(_, h4192) => {
                                        h4192
                                    }
                                }
                            })), {
                                let m4199 = match v9 {
                                    Adt37::C0(_, h4198) => {
                                        h4198
                                    }
                                };
                                match m4199 {
                                    Adt32::C0(_, h4197) => {
                                        h4197
                                    }
                                }
                            })))
                        }
                    }
                }
                Err(v8) => {
                    Ok(Err::<Adt33, Bytes>(v8))
                }
            }
        } else {
            if f309(v5.clone(), Str::lit("any")) {
                let m4200 = f635(v5, f407());
                match m4200 {
                    None => {
                        Ok(Ok::<Adt33, Bytes>(v6))
                    }
                    Some(v11) => {
                        Ok(Ok::<Adt33, Bytes>(f582(v6, Adt34::C0(f8(Str::lit("any")), v11, List::cons(f407(), List::<Bytes>::nil())))))
                    }
                }
            } else {
                if f309(v5.clone(), Str::lit("group")) {
                    let m4201 = f310(v5.clone(), Str::lit("ref"));
                    match m4201 {
                        None => {
                            Ok(Err::<Adt33, Bytes>(f510(f8(Str::lit("a model group reference must have a ref (XSD 1.1 Part 1, section 3.7.2)")), v2)))
                        }
                        Some(v12) => {
                            let m4203 = f125(match v5.clone() {
                                Adt26::C0(_, _, _, _, h4202) => {
                                    h4202
                                }
                            }, v12.clone())?;
                            match m4203 {
                                Ok(v14) => {
                                    let m4204 = f580(v1, Str::lit("group"), v14.clone());
                                    match m4204 {
                                        None => {
                                            Ok(Err::<Adt33, Bytes>(f510(f209(f8(Str::lit("the model group ")), v12, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                        }
                                        Some(_) => {
                                            let v16: Bytes = f200({
                                                let (_, h4205) = v14.clone();
                                                h4205
                                            })?;
                                            let m4206 = f635(v5, v16.clone());
                                            match m4206 {
                                                None => {
                                                    Ok(Ok::<Adt33, Bytes>(v6))
                                                }
                                                Some(v17) => {
                                                    Ok(Ok::<Adt33, Bytes>(f582(v6, Adt34::C0({
                                                        let (_, h4207) = v14;
                                                        h4207
                                                    }, v17, List::cons(v16, List::<Bytes>::nil())))))
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(v13) => {
                                    Ok(Err::<Adt33, Bytes>(f510(v13, v2)))
                                }
                            }
                        }
                    }
                } else {
                    if if f309(v5.clone(), Str::lit("sequence")) {
                        true
                    } else {
                        if f309(v5.clone(), Str::lit("choice")) {
                            true
                        } else {
                            f309(v5.clone(), Str::lit("all"))
                        }
                    } {
                        if if v4 {
                            if f636(v5.clone()) {
                                f308(f309(v5.clone(), Str::lit("choice")))
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m4208 = f508(v5)?;
                            match m4208 {
                                Ok(v19) => {
                                    f575(v7, v1, v2, v3, true, v19, v6)
                                }
                                Err(v18) => {
                                    Ok(Err::<Adt33, Bytes>(v18))
                                }
                            }
                        } else {
                            let v20: Bytes = match v5.clone() {
                                Adt26::C0(_, h4209, _, _, _) => {
                                    h4209
                                }
                            };
                            let v21: u64 = f637(v6.clone(), v20.clone())?;
                            let v22: Bytes = {
                                let a4210 = v21;
                                let a4211 = 1u64;
                                if nat_eq(a4210, a4211) {
                                    Bytes::lit(&[])
                                } else {
                                    f65(v21)?
                                }
                            };
                            let v23: Bytes = f209(v2, f200(v20.clone())?, v22.clone());
                            let m4212 = f511(v7, v1, v23.clone(), v3, v5.clone())?;
                            match m4212 {
                                Ok(v25) => {
                                    let m4213 = f635(v5, v23.clone());
                                    match m4213 {
                                        None => {
                                            Ok(Ok::<Adt33, Bytes>(f638(v6, v20)?))
                                        }
                                        Some(v26) => {
                                            Ok(Ok::<Adt33, Bytes>(f616(f582(f638(v6, v20.clone())?, Adt34::C0({
                                                let a4214 = v20;
                                                let a4215 = v22;
                                                append_bytes(a4214, a4215)
                                            }, v26, List::cons(v23, List::<Bytes>::nil()))), v25)))
                                        }
                                    }
                                }
                                Err(v24) => {
                                    Ok(Err::<Adt33, Bytes>(v24))
                                }
                            }
                        }
                    } else {
                        Ok(Ok::<Adt33, Bytes>(v6))
                    }
                }
            }
        }
    }
}

pub fn f614(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m4216 = v0;
    match m4216.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m4217 = v1;
            match m4217.uncons() {
                None => {
                    List::<Bytes>::nil()
                }
                Some((v4, v5)) => {
                    List::cons(f209(f8(Str::lit("    {name: ")), v2, f209(f8(Str::lit(", type: ")), v4, f8(Str::lit("}")))), f614(v3, v5))
                }
            }
        }
    }
}

pub fn f615(p4223: List<Adt25>, v1: Bytes, v2: Bytes, v3: Bytes) -> Option<Adt25> {
    let m4222 = p4223;
    {
        let mut m4222 = m4222;
        loop {
            let step = {
                let v0 = m4222;
                let m4218 = v0;
                match m4218.uncons() {
                    None => {
                        Err::<List<Adt25>, Option<Adt25>>(None::<Adt25>)
                    }
                    Some((v4, v5)) => {
                        if if f46(match v4.clone() {
                            Adt25::C0(h4219, _, _, _) => {
                                h4219
                            }
                        }, v1.clone()) {
                            if f46(match v4.clone() {
                                Adt25::C0(_, h4220, _, _) => {
                                    h4220
                                }
                            }, v2.clone()) {
                                f46(match v4.clone() {
                                    Adt25::C0(_, _, h4221, _) => {
                                        h4221
                                    }
                                }, v3.clone())
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            Err::<List<Adt25>, Option<Adt25>>(Some(v4))
                        } else {
                            Ok::<List<Adt25>, Option<Adt25>>(v5)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4222 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f616(v0: Adt33, v1: List<Adt27>) -> Adt33 {
    Adt33::C0(match v0.clone() {
        Adt33::C0(h4224, _, _, _, _) => {
            h4224
        }
    }, f404(match v0.clone() {
        Adt33::C0(_, h4225, _, _, _) => {
            h4225
        }
    }, v1), match v0.clone() {
        Adt33::C0(_, _, h4226, _, _) => {
            h4226
        }
    }, match v0.clone() {
        Adt33::C0(_, _, _, h4227, _) => {
            h4227
        }
    }, match v0 {
        Adt33::C0(_, _, _, _, h4228) => {
            h4228
        }
    })
}

pub fn f617(v0: List<Adt34>, v1: List<Adt34>) -> List<Adt34> {
    let m4229 = v0;
    match m4229.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f617(v3, v1))
        }
    }
}

pub fn f618() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f619(v0: Fn12, p4233: List<List<(Bytes, Bytes)>>, p4234: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m4232 = (p4233, p4234);
    {
        let mut m4232 = m4232;
        loop {
            let step = {
                let (v1, v2) = m4232;
                let m4230 = v2;
                match m4230.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c4231: Fn12 = v0.clone();
                            c4231.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m4232 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f620(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f621(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a4236 = {
        let a4235 = v0.clone();
        length_list(a4235)
    };
    let a4237 = 1u64;
    if nat_le(a4236, a4237) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f639(v0)?))
    }
}

pub fn f622(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4282 = {
            let a4258 = {
                let a4246 = {
                    let a4240 = {
                        let a4238 = v1;
                        let a4239 = 128u8;
                        fixed_u8::bit_and(a4238, a4239)
                    };
                    let a4241 = 128u8;
                    if equal(a4240, a4241) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4247 = {
                    let a4244 = {
                        let a4242 = v1;
                        let a4243 = 64u8;
                        fixed_u8::bit_and(a4242, a4243)
                    };
                    let a4245 = 64u8;
                    if equal(a4244, a4245) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4246, a4247)?
            };
            let a4259 = {
                let a4256 = {
                    let a4250 = {
                        let a4248 = v1;
                        let a4249 = 32u8;
                        fixed_u8::bit_and(a4248, a4249)
                    };
                    let a4251 = 32u8;
                    if equal(a4250, a4251) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4257 = {
                    let a4254 = {
                        let a4252 = v1;
                        let a4253 = 16u8;
                        fixed_u8::bit_and(a4252, a4253)
                    };
                    let a4255 = 16u8;
                    if equal(a4254, a4255) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4256, a4257)?
            };
            nat_add(a4258, a4259)?
        };
        let a4283 = {
            let a4280 = {
                let a4268 = {
                    let a4262 = {
                        let a4260 = v1;
                        let a4261 = 8u8;
                        fixed_u8::bit_and(a4260, a4261)
                    };
                    let a4263 = 8u8;
                    if equal(a4262, a4263) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4269 = {
                    let a4266 = {
                        let a4264 = v1;
                        let a4265 = 4u8;
                        fixed_u8::bit_and(a4264, a4265)
                    };
                    let a4267 = 4u8;
                    if equal(a4266, a4267) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4268, a4269)?
            };
            let a4281 = {
                let a4278 = {
                    let a4272 = {
                        let a4270 = v1;
                        let a4271 = 2u8;
                        fixed_u8::bit_and(a4270, a4271)
                    };
                    let a4273 = 2u8;
                    if equal(a4272, a4273) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4279 = {
                    let a4276 = {
                        let a4274 = v1;
                        let a4275 = 1u8;
                        fixed_u8::bit_and(a4274, a4275)
                    };
                    let a4277 = 1u8;
                    if equal(a4276, a4277) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4278, a4279)?
            };
            nat_add(a4280, a4281)?
        };
        nat_add(a4282, a4283)?
    };
    let a4284 = 48u64;
    let a4285 = v2;
    if if nat_le(a4284, a4285) {
        let a4286 = v2;
        let a4287 = 57u64;
        nat_le(a4286, a4287)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a4288 = 97u64;
        let a4289 = v2;
        if nat_le(a4288, a4289) {
            let a4290 = v2;
            let a4291 = 102u64;
            Ok(nat_le(a4290, a4291))
        } else {
            Ok(false)
        }
    }
}

pub fn f623() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f624(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4336 = {
            let a4312 = {
                let a4300 = {
                    let a4294 = {
                        let a4292 = v1;
                        let a4293 = 128u8;
                        fixed_u8::bit_and(a4292, a4293)
                    };
                    let a4295 = 128u8;
                    if equal(a4294, a4295) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4301 = {
                    let a4298 = {
                        let a4296 = v1;
                        let a4297 = 64u8;
                        fixed_u8::bit_and(a4296, a4297)
                    };
                    let a4299 = 64u8;
                    if equal(a4298, a4299) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4300, a4301)?
            };
            let a4313 = {
                let a4310 = {
                    let a4304 = {
                        let a4302 = v1;
                        let a4303 = 32u8;
                        fixed_u8::bit_and(a4302, a4303)
                    };
                    let a4305 = 32u8;
                    if equal(a4304, a4305) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4311 = {
                    let a4308 = {
                        let a4306 = v1;
                        let a4307 = 16u8;
                        fixed_u8::bit_and(a4306, a4307)
                    };
                    let a4309 = 16u8;
                    if equal(a4308, a4309) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4310, a4311)?
            };
            nat_add(a4312, a4313)?
        };
        let a4337 = {
            let a4334 = {
                let a4322 = {
                    let a4316 = {
                        let a4314 = v1;
                        let a4315 = 8u8;
                        fixed_u8::bit_and(a4314, a4315)
                    };
                    let a4317 = 8u8;
                    if equal(a4316, a4317) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4323 = {
                    let a4320 = {
                        let a4318 = v1;
                        let a4319 = 4u8;
                        fixed_u8::bit_and(a4318, a4319)
                    };
                    let a4321 = 4u8;
                    if equal(a4320, a4321) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4322, a4323)?
            };
            let a4335 = {
                let a4332 = {
                    let a4326 = {
                        let a4324 = v1;
                        let a4325 = 2u8;
                        fixed_u8::bit_and(a4324, a4325)
                    };
                    let a4327 = 2u8;
                    if equal(a4326, a4327) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4333 = {
                    let a4330 = {
                        let a4328 = v1;
                        let a4329 = 1u8;
                        fixed_u8::bit_and(a4328, a4329)
                    };
                    let a4331 = 1u8;
                    if equal(a4330, a4331) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4332, a4333)?
            };
            nat_add(a4334, a4335)?
        };
        nat_add(a4336, a4337)?
    };
    let a4338 = v2;
    let a4339 = 57u64;
    if nat_le(a4338, a4339) {
        let a4340 = v2;
        let a4341 = 48u64;
        Ok(nat_sub(a4340, a4341))
    } else {
        let a4342 = v2;
        let a4343 = 87u64;
        Ok(nat_sub(a4342, a4343))
    }
}

pub fn f625(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f528(v0.clone(), {
        let (h4344, _) = v2;
        h4344
    }, v1)?;
    let a4345 = f154(v0, v3)?;
    let a4346 = 10u64;
    if nat_eq(a4345, a4346) {
        Ok(Some(({
            let a4347 = v3;
            let a4348 = 1u64;
            nat_add(a4347, a4348)?
        }, {
            let a4350 = {
                let (_, h4349) = v2;
                h4349
            };
            let a4351 = 1u64;
            nat_add(a4350, a4351)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f626(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a4352 = f154(v0, v3)?;
    let a4353 = 32u64;
    if if nat_eq(a4352, a4353) {
        let a4356 = {
            let a4354 = v3;
            let a4355 = v1;
            nat_sub(a4354, a4355)
        };
        let a4357 = v2;
        nat_lt(a4356, a4357)
    } else {
        false
    } {
        Ok(Some({
            let a4358 = v3;
            let a4359 = 1u64;
            nat_add(a4358, a4359)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f627(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m4361 = f596(f154(v0, {
        let (h4360, _) = v2;
        h4360
    })?);
    match m4361 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v3) => {
            let a4362 = v3;
            let a4363 = v1;
            if nat_lt(a4362, a4363) {
                Ok(Some(({
                    let a4365 = {
                        let (h4364, _) = v2;
                        h4364
                    };
                    let a4366 = 1u64;
                    nat_add(a4365, a4366)?
                }, {
                    let a4370 = {
                        let a4368 = {
                            let (_, h4367) = v2;
                            h4367
                        };
                        let a4369 = v1;
                        nat_mul(a4368, a4369)?
                    };
                    let a4371 = v3;
                    nat_add(a4370, a4371)?
                })))
            } else {
                Ok(None::<(u64, u64)>)
            }
        }
    }
}

pub fn f628(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f154(v0, v1)?;
    let a4372 = v2;
    let a4373 = 34u64;
    if if if if nat_eq(a4372, a4373) {
        true
    } else {
        let a4374 = v2;
        let a4375 = 92u64;
        nat_eq(a4374, a4375)
    } {
        true
    } else {
        f230(v2)
    } {
        true
    } else {
        f231(v2)
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a4376 = v1;
            let a4377 = 1u64;
            nat_add(a4376, a4377)?
        }))
    }
}

pub fn f629(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f154(v0, v1)?;
    let a4378 = v2;
    let a4379 = 39u64;
    if if if nat_eq(a4378, a4379) {
        true
    } else {
        f230(v2)
    } {
        true
    } else {
        f231(v2)
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a4380 = v1;
            let a4381 = 1u64;
            nat_add(a4380, a4381)?
        }))
    }
}

pub fn f630(v0: u64) -> bool {
    let a4382 = 48u64;
    let a4383 = v0;
    if nat_le(a4382, a4383) {
        let a4384 = v0;
        let a4385 = 55u64;
        nat_le(a4384, a4385)
    } else {
        false
    }
}

pub fn f631(v0: u64) -> bool {
    if f351(v0) {
        true
    } else {
        if f464(v0) {
            true
        } else {
            let a4386 = v0;
            let a4387 = 95u64;
            if nat_eq(a4386, a4387) {
                true
            } else {
                let a4388 = v0;
                let a4389 = 45u64;
                nat_eq(a4388, a4389)
            }
        }
    }
}

pub fn f632(v0: bool, v1: List<Bytes>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m4390 = v1;
    match m4390.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m4391 = f379(v0, v3.clone())?;
            match m4391 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f209(f8(Str::lit("the component name ")), v3, f8(Str::lit(" derives no LexLean name")))))
                }
                Some(v5) => {
                    if f167(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f209(f8(Str::lit("two components derive the name ")), v5, f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n; when those names are not distinct, every name keeps its letters and digits and writes each other character as a C11 universal character name)")))))
                    } else {
                        let m4392 = f632(v0, v4, List::cons(v5.clone(), v2))?;
                        match m4392 {
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

pub fn f633(v0: Adt33) -> Adt33 {
    Adt33::C0(match v0.clone() {
        Adt33::C0(h4393, _, _, _, _) => {
            h4393
        }
    }, match v0 {
        Adt33::C0(_, h4394, _, _, _) => {
            h4394
        }
    }, 0u64, 0u64, 0u64)
}

pub fn f634(v0: u64, v1: Adt24, v2: Bytes, v3: Bytes, v4: Adt26) -> R<Result<Adt37, Bytes>> {
    let m4395 = v0;
    if m4395 == 0 {
        Ok(Err::<Adt37, Bytes>(f510(f8(Str::lit("the element declarations nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m4395 - 1;
        let m4396 = f310(v4.clone(), Str::lit("ref"));
        match m4396 {
            None => {
                let m4407 = f310(v4.clone(), Str::lit("name"));
                match m4407 {
                    None => {
                        Ok(Err::<Adt37, Bytes>(f510(f8(Str::lit("a local element declaration must have a name or a ref (XSD 1.1 Part 1, section 3.3.3)")), v2)))
                    }
                    Some(v12) => {
                        let m4410 = f506(v5, v1, {
                            let a4408 = v2;
                            let a4409 = f200(v12.clone())?;
                            append_bytes(a4408, a4409)
                        }, v3, v4)?;
                        match m4410 {
                            Ok(v14) => {
                                Ok(Ok::<Adt37, Bytes>(Adt37::C0(v12, v14)))
                            }
                            Err(v13) => {
                                Ok(Err::<Adt37, Bytes>(v13))
                            }
                        }
                    }
                }
            }
            Some(v6) => {
                let m4398 = f125(match v4 {
                    Adt26::C0(_, _, _, _, h4397) => {
                        h4397
                    }
                }, v6.clone())?;
                match m4398 {
                    Ok(v8) => {
                        let m4399 = f580(v1.clone(), Str::lit("element"), v8.clone());
                        match m4399 {
                            None => {
                                Ok(Err::<Adt37, Bytes>(f510(f209(f8(Str::lit("the element ")), v6, f8(Str::lit(" is not declared by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                            }
                            Some(v9) => {
                                if if f567(match v9.clone() {
                                    Adt25::C0(_, _, _, h4400) => {
                                        h4400
                                    }
                                }, Str::lit("abstract"), Str::lit("true")) {
                                    true
                                } else {
                                    f640(match v1.clone() {
                                        Adt24::C0(h4401, _) => {
                                            h4401
                                        }
                                    }, v8.clone())?
                                } {
                                    Ok(Err::<Adt37, Bytes>(f510(f209(f8(Str::lit("the element ")), v6, f8(Str::lit(" heads a substitution group, so its particle also admits the members of that group, which is not translated (XSD 1.1 Part 1, section 3.3.6.4)"))), v2)))
                                } else {
                                    let m4405 = f506(v5, v1, f200({
                                        let (_, h4402) = v8.clone();
                                        h4402
                                    })?, match v9.clone() {
                                        Adt25::C0(_, h4403, _, _) => {
                                            h4403
                                        }
                                    }, match v9 {
                                        Adt25::C0(_, _, _, h4404) => {
                                            h4404
                                        }
                                    })?;
                                    match m4405 {
                                        Ok(v11) => {
                                            Ok(Ok::<Adt37, Bytes>(Adt37::C0({
                                                let (_, h4406) = v8;
                                                h4406
                                            }, v11)))
                                        }
                                        Err(v10) => {
                                            Ok(Err::<Adt37, Bytes>(v10))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(Err::<Adt37, Bytes>(f510(v7, v2)))
                    }
                }
            }
        }
    }
}

pub fn f635(v0: Adt26, v1: Bytes) -> Option<Bytes> {
    let v2: Bytes = f313(v0.clone(), Str::lit("maxOccurs"), Str::lit("1"));
    if f46(v2.clone(), f8(Str::lit("0"))) {
        None::<Bytes>
    } else {
        if f46(v2, f8(Str::lit("1"))) {
            if f567(v0, Str::lit("minOccurs"), Str::lit("0")) {
                Some(f612(Str::lit("Option"), v1))
            } else {
                Some(v1)
            }
        } else {
            Some(f612(Str::lit("List"), v1))
        }
    }
}

pub fn f636(v0: Adt26) -> bool {
    if f46(f313(v0.clone(), Str::lit("minOccurs"), Str::lit("1")), f8(Str::lit("1"))) {
        f46(f313(v0, Str::lit("maxOccurs"), Str::lit("1")), f8(Str::lit("1")))
    } else {
        false
    }
}

pub fn f637(v0: Adt33, v1: Bytes) -> R<u64> {
    if f46(v1.clone(), f8(Str::lit("sequence"))) {
        let a4412 = match v0 {
            Adt33::C0(_, _, h4411, _, _) => {
                h4411
            }
        };
        let a4413 = 1u64;
        nat_add(a4412, a4413)
    } else {
        if f46(v1, f8(Str::lit("choice"))) {
            let a4415 = match v0 {
                Adt33::C0(_, _, _, h4414, _) => {
                    h4414
                }
            };
            let a4416 = 1u64;
            nat_add(a4415, a4416)
        } else {
            let a4418 = match v0 {
                Adt33::C0(_, _, _, _, h4417) => {
                    h4417
                }
            };
            let a4419 = 1u64;
            nat_add(a4418, a4419)
        }
    }
}

pub fn f638(v0: Adt33, v1: Bytes) -> R<Adt33> {
    if f46(v1.clone(), f8(Str::lit("sequence"))) {
        Ok(Adt33::C0(match v0.clone() {
            Adt33::C0(h4420, _, _, _, _) => {
                h4420
            }
        }, match v0.clone() {
            Adt33::C0(_, h4421, _, _, _) => {
                h4421
            }
        }, {
            let a4423 = match v0.clone() {
                Adt33::C0(_, _, h4422, _, _) => {
                    h4422
                }
            };
            let a4424 = 1u64;
            nat_add(a4423, a4424)?
        }, match v0.clone() {
            Adt33::C0(_, _, _, h4425, _) => {
                h4425
            }
        }, match v0 {
            Adt33::C0(_, _, _, _, h4426) => {
                h4426
            }
        }))
    } else {
        if f46(v1, f8(Str::lit("choice"))) {
            Ok(Adt33::C0(match v0.clone() {
                Adt33::C0(h4427, _, _, _, _) => {
                    h4427
                }
            }, match v0.clone() {
                Adt33::C0(_, h4428, _, _, _) => {
                    h4428
                }
            }, match v0.clone() {
                Adt33::C0(_, _, h4429, _, _) => {
                    h4429
                }
            }, {
                let a4431 = match v0.clone() {
                    Adt33::C0(_, _, _, h4430, _) => {
                        h4430
                    }
                };
                let a4432 = 1u64;
                nat_add(a4431, a4432)?
            }, match v0 {
                Adt33::C0(_, _, _, _, h4433) => {
                    h4433
                }
            }))
        } else {
            Ok(Adt33::C0(match v0.clone() {
                Adt33::C0(h4434, _, _, _, _) => {
                    h4434
                }
            }, match v0.clone() {
                Adt33::C0(_, h4435, _, _, _) => {
                    h4435
                }
            }, match v0.clone() {
                Adt33::C0(_, _, h4436, _, _) => {
                    h4436
                }
            }, match v0.clone() {
                Adt33::C0(_, _, _, h4437, _) => {
                    h4437
                }
            }, {
                let a4439 = match v0 {
                    Adt33::C0(_, _, _, _, h4438) => {
                        h4438
                    }
                };
                let a4440 = 1u64;
                nat_add(a4439, a4440)?
            }))
        }
    }
}

pub fn f639(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f586({
        let (_, h4441) = {
            let (h4442, _) = f641(Fn13::F642, {
                let a4444 = {
                    let a4443 = v0.clone();
                    length_list(a4443)
                };
                let a4445 = 1u64;
                nat_add(a4444, a4445)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h4442
        };
        h4441
    }))
}

pub fn f640(p4454: List<Adt25>, v1: (Bytes, Bytes)) -> R<bool> {
    let m4453 = p4454;
    {
        let mut m4453 = m4453;
        loop {
            let step = {
                let v0 = m4453;
                let m4446 = v0;
                match m4446.uncons() {
                    None => {
                        Err::<List<Adt25>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if if f46(match v2.clone() {
                            Adt25::C0(h4447, _, _, _) => {
                                h4447
                            }
                        }, f8(Str::lit("element"))) {
                            let m4449 = f310(match v2.clone() {
                                Adt25::C0(_, _, _, h4448) => {
                                    h4448
                                }
                            }, Str::lit("substitutionGroup"));
                            match m4449 {
                                None => {
                                    false
                                }
                                Some(v4) => {
                                    f643({
                                        let m4452 = match v2 {
                                            Adt25::C0(_, _, _, h4451) => {
                                                h4451
                                            }
                                        };
                                        match m4452 {
                                            Adt26::C0(_, _, _, _, h4450) => {
                                                h4450
                                            }
                                        }
                                    }, f644(v4)?, v1.clone())?
                                }
                            }
                        } else {
                            false
                        } {
                            Err::<List<Adt25>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Adt25>, R<bool>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4453 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f641(v0: Fn13, p4459: u64, p4460: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m4458 = (p4459, p4460);
    {
        let mut m4458 = m4458;
        loop {
            let step = {
                let (v1, v2) = m4458;
                let m4455 = v1;
                if m4455 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4455 - 1;
                    let m4457 = {
                        let c4456: Fn13 = v0.clone();
                        c4456.apply(v2.clone())?
                    };
                    match m4457 {
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
                Ok(next) => { m4458 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f642(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f645(v0)
}

pub fn f643(v0: List<(Bytes, Bytes)>, p4468: List<Bytes>, v2: (Bytes, Bytes)) -> R<bool> {
    let m4467 = p4468;
    {
        let mut m4467 = m4467;
        loop {
            let step = {
                let v1 = m4467;
                let m4461 = v1;
                match m4461.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v3, v4)) => {
                        let m4462 = f125(v0.clone(), v3)?;
                        match m4462 {
                            Ok(v6) => {
                                if if f46({
                                    let (h4463, _) = v6;
                                    h4463
                                }, {
                                    let (h4464, _) = v2.clone();
                                    h4464
                                }) {
                                    f46({
                                        let (_, h4465) = v6;
                                        h4465
                                    }, {
                                        let (_, h4466) = v2.clone();
                                        h4466
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
                Ok(next) => { m4467 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f644(v0: Bytes) -> R<List<Bytes>> {
    f41(v0, 32u8)
}

pub fn f645(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h4469, _) = v0;
        h4469
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h4470) = v0;
        h4470
    };
    let m4471 = v1;
    match m4471.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m4472 = v4;
            match m4472.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f646(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f646(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a4477 = {
            let a4475 = {
                let a4473 = v0.clone();
                length_list(a4473)
            };
            let a4476 = {
                let a4474 = v1.clone();
                length_list(a4474)
            };
            nat_add(a4475, a4476)?
        };
        let a4478 = 1u64;
        nat_add(a4477, a4478)?
    };
    Ok(f647({
        let (_, h4479) = {
            let (_, h4480) = {
                let (h4481, _) = f648(Fn14::F649, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h4481
            };
            h4480
        };
        h4479
    }))
}

pub fn f647(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f419(Fn6::F650, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f648(v0: Fn14, p4486: u64, p4487: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m4485 = (p4486, p4487);
    {
        let mut m4485 = m4485;
        loop {
            let step = {
                let (v1, v2) = m4485;
                let m4482 = v1;
                if m4482 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m4482 - 1;
                    let m4484 = {
                        let c4483: Fn14 = v0.clone();
                        c4483.apply(v2.clone())
                    };
                    match m4484 {
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
                Ok(next) => { m4485 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f649(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f651(v0)
}

pub fn f650(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f651(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h4488, _) = v0.clone();
        h4488
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h4489, _) = {
            let (_, h4490) = v0.clone();
            h4490
        };
        h4489
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h4491) = {
            let (_, h4492) = v0;
            h4492
        };
        h4491
    };
    let m4493 = v1.clone();
    match m4493.uncons() {
        None => {
            let m4494 = v2;
            match m4494.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m4495 = v2.clone();
            match m4495.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f652({
                        let (h4496, _) = v8.clone();
                        h4496
                    }, {
                        let (h4497, _) = v6.clone();
                        h4497
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

pub fn f652(v0: Bytes, v1: Bytes) -> bool {
    let a4502 = {
        let a4498 = v0;
        let a4499 = v1;
        compare_bytes(a4498, a4499)
    };
    let a4503 = {
        let a4500 = Bytes::lit(&[]);
        let a4501 = Bytes::lit(&[0x01]);
        compare_bytes(a4500, a4501)
    };
    equal(a4502, a4503)
}

pub fn run(p0: Option<Adt0>, p1: Adt3) -> R<(Adt0, Adt10)> {
    f0(p0, p1)
}
