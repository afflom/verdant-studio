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
    F222(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F44(k0, k1) => f44(*k0, k1.clone(), p0),
            Fn0::F222(k0) => f222(k0.clone(), p0),
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
    F447(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn3::F128(k0, k1) => Ok(f128(*k0, k1.clone(), p0)),
            Fn3::F129(k0) => f129(k0.clone(), p0),
            Fn3::F447(k0) => f447(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F131(Bytes),
    F170(Bytes),
    F171(Bytes),
    F206(Bytes, Bytes),
    F207(Bytes),
    F209(Bytes, u64),
    F309(bool, u64, Bytes),
    F341(Bytes),
    F343(Bytes),
    F352(Bytes, u64),
    F362(u64, Bytes),
    F421(Bytes),
    F423(Bytes),
    F537(u64, Bytes, u64),
    F540(Bytes),
    F541(Bytes),
    F545(Bytes),
    F546(Bytes),
    F572(Bytes),
    F576(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn4::F131(k0) => f131(k0.clone(), p0),
            Fn4::F170(k0) => f170(k0.clone(), p0),
            Fn4::F171(k0) => f171(k0.clone(), p0),
            Fn4::F206(k0, k1) => f206(k0.clone(), k1.clone(), p0),
            Fn4::F207(k0) => f207(k0.clone(), p0),
            Fn4::F209(k0, k1) => f209(k0.clone(), *k1, p0),
            Fn4::F309(k0, k1, k2) => f309(*k0, *k1, k2.clone(), p0),
            Fn4::F341(k0) => f341(k0.clone(), p0),
            Fn4::F343(k0) => f343(k0.clone(), p0),
            Fn4::F352(k0, k1) => f352(k0.clone(), *k1, p0),
            Fn4::F362(k0, k1) => f362(*k0, k1.clone(), p0),
            Fn4::F421(k0) => f421(k0.clone(), p0),
            Fn4::F423(k0) => f423(k0.clone(), p0),
            Fn4::F537(k0, k1, k2) => f537(*k0, k1.clone(), *k2, p0),
            Fn4::F540(k0) => f540(k0.clone(), p0),
            Fn4::F541(k0) => f541(k0.clone(), p0),
            Fn4::F545(k0) => f545(k0.clone(), p0),
            Fn4::F546(k0) => f546(k0.clone(), p0),
            Fn4::F572(k0) => f572(k0.clone(), p0),
            Fn4::F576(k0) => f576(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F358,
    F590,
}

impl Fn5 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn5::F358 => f358(p0, p1),
            Fn5::F590 => f590(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F361,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn6::F361 => f361(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F395(bool, Bytes),
    F429(Bytes),
    F438(bool, Bytes),
    F496(Bytes),
    F506(Bytes),
    F536(u64, Bytes),
    F544(u64, Bytes),
    F548(Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn7::F395(k0, k1) => f395(*k0, k1.clone(), p0),
            Fn7::F429(k0) => f429(k0.clone(), p0),
            Fn7::F438(k0, k1) => f438(*k0, k1.clone(), p0),
            Fn7::F496(k0) => f496(k0.clone(), p0),
            Fn7::F506(k0) => f506(k0.clone(), p0),
            Fn7::F536(k0, k1) => f536(*k0, k1.clone(), p0),
            Fn7::F544(k0, k1) => f544(*k0, k1.clone(), p0),
            Fn7::F548(k0) => f548(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F504(u64, Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: (u64, bool)) -> R<Option<(u64, bool)>> {
        match self {
            Fn8::F504(k0, k1) => f504(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F516,
}

impl Fn9 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn9::F516 => f516(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F518,
}

impl Fn10 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn10::F518 => f518(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F552,
}

impl Fn11 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn11::F552 => f552(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F581,
}

impl Fn12 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn12::F581 => f581(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F589,
}

impl Fn13 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn13::F589 => f589(p0),
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
    List::cons(f8(Str::lit("admit JSONL")), List::cons(f8(Str::lit("admit-encoding STAGED")), List::cons(f8(Str::lit("admit-translated FILE NAME read-OP ARG... CITATION...")), List::cons(f8(Str::lit("agenda")), List::cons(f8(Str::lit("build")), List::cons(f8(Str::lit("check")), List::cons(f8(Str::lit("ingest-authority BINDING")), List::cons(f8(Str::lit("ingest-index CSV")), List::cons(f8(Str::lit("merge-gate RULES")), List::cons(f8(Str::lit("next [SKIP...]")), List::cons(f8(Str::lit("propose-encoding SCHEMA NAME CITATION...")), List::cons(f8(Str::lit("publish")), List::cons(f8(Str::lit("read-webidl FILE [IMPORT...]")), List::cons(f8(Str::lit("read-xml FILE")), List::cons(f8(Str::lit("read-xsd FILE")), List::cons(f8(Str::lit("read-xsd-components FILE")), List::cons(f8(Str::lit("read-yaml FILE")), List::cons(f8(Str::lit("record-decision JSONL")), List::cons(f8(Str::lit("stage-admissions JSONL")), List::cons(f8(Str::lit("stage-bindings JSONL")), List::cons(f8(Str::lit("stage-decisions JSONL")), List::cons(f8(Str::lit("stage-encodings JSONL")), List::cons(f8(Str::lit("stage-engine")), List::cons(f8(Str::lit("stage-mappings JSONL")), List::cons(f8(Str::lit("stage-matrix JSONL")), List::<Bytes>::nil())))))))))))))))))))))))))
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
        let m355 = f103(v2.clone(), v5, {
            let a353 = v3;
            let a354 = 1u64;
            nat_add(a353, a354)?
        })?;
        match m355 {
            Ok(v7) => {
                let a356 = v3;
                let a357 = v7;
                if nat_le(a356, a357) {
                    Ok(Ok::<List<Adt19>, Bytes>(List::<Adt19>::nil()))
                } else {
                    f104(v2, v7, true, {
                        let a358 = v3;
                        let a359 = 1u64;
                        nat_add(a358, a359)?
                    })
                }
            }
            Err(v6) => {
                Ok(Err::<List<Adt19>, Bytes>(v6))
            }
        }
    }
}

pub fn f59(v0: List<Adt19>) -> R<Bytes> {
    let m360 = v0;
    match m360.uncons() {
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
    let m366 = f106(v0.clone(), v3);
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

pub fn f61(v0: Fn2, p380: List<Bytes>, p381: List<Bytes>) -> List<Bytes> {
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

pub fn f62(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f63(p386: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
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

pub fn f64(v0: Bytes) -> R<Result<List<Adt12>, Bytes>> {
    let m387 = f107(v0)?;
    match m387 {
        Ok(v2) => {
            f108({
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

pub fn f65(v0: List<Bytes>) -> R<Result<List<Adt12>, Bytes>> {
    let m391 = v0;
    match m391.uncons() {
        None => {
            Ok(Ok::<List<Adt12>, Bytes>(List::<Adt12>::nil()))
        }
        Some((v1, v2)) => {
            let m392 = f64(v1)?;
            match m392 {
                Ok(v4) => {
                    let m393 = f65(v2)?;
                    match m393 {
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
    let m394 = v0;
    match m394.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f66(v3, v1))
        }
    }
}

pub fn f67(p399: u64, p400: List<Adt15>) -> R<List<Adt15>> {
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
                    let v3: List<Adt15> = f109(v1.clone(), v1.clone());
                    let a396 = f110(v3.clone())?;
                    let a397 = f110(v1.clone())?;
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

pub fn f68(v0: List<Adt12>, v1: List<Adt12>) -> R<List<Adt15>> {
    let m401 = v0;
    match m401.uncons() {
        None => {
            Ok(List::<Adt15>::nil())
        }
        Some((v2, v3)) => {
            if f111(f112(v2.clone())) {
                let v6: Adt15 = {
                    let m402 = f113(v1.clone(), v2.clone())?;
                    match m402 {
                        Ok(v5) => {
                            Adt15::C0(f114(v2), Bytes::lit(&[]), {
                                let (h403, _) = v5;
                                h403
                            }, {
                                let (_, h404) = v5;
                                h404
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

pub fn f69(p409: u64, p410: List<Bytes>, p411: List<Bytes>, v3: List<Adt15>) -> List<Bytes> {
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
                            if f115(v5.clone(), v2.clone()) {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v2)))
                            } else {
                                let m407 = f116(v3.clone(), v5.clone());
                                match m407 {
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
                Ok(next) => { m408 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f70(v0: List<Adt12>) -> List<Bytes> {
    let m412 = v0;
    match m412.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let v3: u64 = f112(v1.clone());
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
                List::cons(f114(v1), f70(v2))
            } else {
                f70(v2)
            }
        }
    }
}

pub fn f71(v0: List<Adt15>, v1: List<Bytes>) -> List<(Bytes, (List<Bytes>, Bytes))> {
    let m427 = v0;
    match m427.uncons() {
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

pub fn f72(p436: u64, p437: List<(Bytes, (List<Bytes>, Bytes))>, p438: List<Bytes>, p439: List<Bytes>) -> (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>) {
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
                    let m429 = f122(v1.clone(), v2.clone());
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

pub fn f73(v0: List<Adt12>, v1: List<Adt15>, v2: List<Adt12>) -> List<Bytes> {
    let m440 = v0;
    match m440.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            let v5: u64 = f112(v3.clone());
            let a441 = v5;
            let a442 = 5u64;
            if if nat_eq(a441, a442) {
                true
            } else {
                let a443 = v5;
                let a444 = 7u64;
                nat_eq(a443, a444)
            } {
                let m447 = f123(v2.clone(), {
                    let a445 = v5;
                    let a446 = 5u64;
                    if nat_eq(a445, a446) {
                        0u64
                    } else {
                        2u64
                    }
                }, f114(v3.clone()));
                match m447 {
                    None => {
                        List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f124(v5), List::cons(f114(v3.clone()), List::cons(f8(Str::lit(": no input defines ")), List::cons(f114(v3), List::cons(f8(Str::lit(", so its members join no definition (Web IDL 2.2 and 2.7)\u{a}")), List::<Bytes>::nil()))))))), f73(v4, v1, v2))
                    }
                    Some(_) => {
                        f73(v4, v1, v2)
                    }
                }
            } else {
                let a448 = v5;
                let a449 = 8u64;
                if nat_eq(a448, a449) {
                    let m450 = f123(v2.clone(), 0u64, f114(v3.clone()));
                    match m450 {
                        None => {
                            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f114(v3.clone()), List::cons(f8(Str::lit(" includes ")), List::cons(f125(v3.clone()), List::cons(f8(Str::lit(": no input defines the interface ")), List::cons(f114(v3), List::cons(f8(Str::lit(" (Web IDL 2.3)\u{a}")), List::<Bytes>::nil())))))))), f73(v4, v1, v2))
                        }
                        Some(_) => {
                            f73(v4, v1, v2)
                        }
                    }
                } else {
                    if f111(v5) {
                        let m451 = f116(v1.clone(), f114(v3.clone()));
                        match m451 {
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
    let m452 = v0;
    match m452.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons({
                let (h453, _) = v1;
                h453
            }, List::cons(f8(Str::lit(": it refers back to itself through its members, and the translator emits non-recursive declarations only\u{a}")), List::<Bytes>::nil())))), f74(v2))
        }
    }
}

pub fn f75(v0: List<Bytes>, v1: Bytes) -> Bytes {
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
                        let a457 = f75(v3, v1);
                        append_bytes(a456, a457)
                    };
                    append_bytes(a458, a459)
                }
            }
        }
    }
}

pub fn f76(p462: List<Bytes>, p463: List<Bytes>) -> List<Bytes> {
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

pub fn f77(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a464 = v2;
    let a465 = 0u64;
    if nat_eq(a464, a465) {
        Ok(f126(v0, 0u64))
    } else {
        let (_, h466) = {
            let (h467, _) = f127(Fn3::F128(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h467
        };
        Ok(h466)
    }
}

pub fn f78() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f79(v0: Bytes, v1: Bytes, v2: u64) -> bool {
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

pub fn f80(v0: Bytes) -> R<Bytes> {
    let (_, h475) = {
        let (h476, _) = f127(Fn3::F129(v0.clone()), {
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

pub fn f81(v0: Bytes) -> R<u64> {
    let (h480, _) = f130(Fn4::F131(v0.clone()), {
        let a482 = {
            let a481 = v0;
            length_bytes(a481)
        };
        let a483 = 1u64;
        nat_add(a482, a483)?
    }, 0u64)?;
    Ok(h480)
}

pub fn f82(v0: Bytes) -> Result<Adt16, Bytes> {
    Err::<Adt16, Bytes>(v0)
}

pub fn f83(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a484 = v0;
        utf8_encode(a484)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f52(v1)?, List::<Bytes>::nil())))))
}

pub fn f84(v0: Bytes) -> R<Result<u64, Bytes>> {
    if if f86(v0.clone(), 0u64, Str::lit("<?xml")) {
        f132(f87(v0.clone(), 5u64)?)?
    } else {
        false
    } {
        let m486 = f133(v0.clone(), {
            let a485 = Str::lit("?>");
            utf8_encode(a485)
        }, 5u64)?;
        match m486 {
            None => {
                Ok(f134(f83(Str::lit("the XML declaration is not closed (XML 1.0 [23])"), 0u64)?))
            }
            Some(v1) => {
                let m488 = f133(f32(v0.clone(), 0u64, v1), {
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
                        let v3: u64 = f135(v0.clone(), {
                            let a491 = v2;
                            let a492 = 8u64;
                            nat_add(a491, a492)?
                        })?;
                        let v4: u64 = f135(v0.clone(), {
                            let a493 = v3;
                            let a494 = 1u64;
                            nat_add(a493, a494)?
                        })?;
                        let _: u64 = f87(v0.clone(), v4)?;
                        let v7: u64 = {
                            let m497 = f133(v0.clone(), f32(v0.clone(), v4, 1u64), {
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
                        if f136(f32(v0, {
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

pub fn f85(v0: Bytes, p510: u64, p511: u64) -> R<Result<u64, Bytes>> {
    let m509 = (p510, p511);
    {
        let mut m509 = m509;
        loop {
            let step = {
                let (v1, v2) = m509;
                let m506 = v2;
                if m506 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f134(f83(Str::lit("the prolog is longer than the input"), v1)?)))
                } else {
                    let v3 = m506 - 1;
                    let v4: u64 = f135(v0.clone(), v1)?;
                    if f86(v0.clone(), v4, Str::lit("<!--")) {
                        let m507 = f137(v0.clone(), v4)?;
                        match m507 {
                            Ok(v6) => {
                                Ok::<(u64, u64), R<Result<u64, Bytes>>>((v6, v3))
                            }
                            Err(v5) => {
                                Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f134(v5)))
                            }
                        }
                    } else {
                        if f86(v0.clone(), v4, Str::lit("<?")) {
                            let m508 = f138(v0.clone(), v4)?;
                            match m508 {
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
                Ok(next) => { m509 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f86(v0: Bytes, v1: u64, v2: Str) -> bool {
    f79(v0, {
        let a512 = v2;
        utf8_encode(a512)
    }, v1)
}

pub fn f87(v0: Bytes, v1: u64) -> R<u64> {
    f139(v0, v1)
}

pub fn f88(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Result<(Adt16, u64), Bytes>> {
    let m513 = v2;
    if m513 == 0 {
        Ok(f140(f83(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v4 = m513 - 1;
        let a514 = v3;
        let a515 = 0u64;
        if nat_eq(a514, a515) {
            let v5: u64 = f141(v0.clone(), {
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
                Ok(f140(f83(Str::lit("an element must start with a Name (XML 1.0 [40])"), v1)?))
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
                let m528 = f142(v0.clone(), v5, v4, List::<Bytes>::nil())?;
                match m528 {
                    Ok(v8) => {
                        if f86(v0.clone(), {
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
                            let m537 = f88(v0.clone(), {
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
                                    let v12: u64 = f141(v0.clone(), v11)?;
                                    if f35(f32(v0.clone(), v11, {
                                        let a541 = v12;
                                        let a542 = v11;
                                        nat_sub(a541, a542)
                                    }), v6.clone()) {
                                        let v13: u64 = f135(v0.clone(), v12)?;
                                        let a543 = f87(v0, v13)?;
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
                                            Ok(f140(f83(Str::lit("an end tag must close with > (XML 1.0 [42])"), v13)?))
                                        }
                                    } else {
                                        Ok(f140(f83(Str::lit("an end tag must name the element its start tag opened (XML 1.0 WFC: Element Type Match)"), {
                                            let (_, h549) = v10;
                                            h549
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
            let a550 = f87(v0.clone(), v1)?;
            let a551 = 256u64;
            if nat_eq(a550, a551) {
                Ok(f140(f83(Str::lit("the input ends before an element is closed (XML 1.0 [39])"), v1)?))
            } else {
                if f86(v0.clone(), v1, Str::lit("</")) {
                    Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C4, v1)))
                } else {
                    if f86(v0.clone(), v1, Str::lit("<!--")) {
                        let m552 = f137(v0.clone(), v1)?;
                        match m552 {
                            Ok(v15) => {
                                f88(v0, v15, v4, 1u64)
                            }
                            Err(v14) => {
                                Ok(f140(v14))
                            }
                        }
                    } else {
                        if f86(v0.clone(), v1, Str::lit("<![CDATA[")) {
                            let m556 = f133(v0.clone(), {
                                let a553 = Str::lit("]]>");
                                utf8_encode(a553)
                            }, {
                                let a554 = v1;
                                let a555 = 9u64;
                                nat_add(a554, a555)?
                            })?;
                            match m556 {
                                None => {
                                    Ok(f140(f83(Str::lit("a CDATA section is not closed (XML 1.0 [18])"), v1)?))
                                }
                                Some(v16) => {
                                    let m559 = f88(v0.clone(), {
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
                                            Ok(f140(v17))
                                        }
                                    }
                                }
                            }
                        } else {
                            if f86(v0.clone(), v1, Str::lit("<?")) {
                                let m568 = f138(v0.clone(), v1)?;
                                match m568 {
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
                                    let a569 = f87(v0.clone(), v1)?;
                                    let a570 = 60u64;
                                    if nat_eq(a569, a570) {
                                        let m571 = f88(v0.clone(), v1, v4, 0u64)?;
                                        match m571 {
                                            Ok(v22) => {
                                                let m573 = f88(v0, {
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
                                                        Ok(f140(v23))
                                                    }
                                                }
                                            }
                                            Err(v21) => {
                                                Ok(f140(v21))
                                            }
                                        }
                                    } else {
                                        let m577 = f143(v0.clone(), v1, v4, Bytes::lit(&[]), false, 0u64)?;
                                        match m577 {
                                            Ok(v26) => {
                                                let m579 = f88(v0, {
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
    let m583 = v0;
    match m583 {
        Adt16::C0(_, _, _) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt16::C1(v2, v3, r588) => {
            let v4 = (*r588).clone();
            let m584 = f90(v4, v1);
            match m584 {
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
                                let a586 = {
                                    let a585 = v2;
                                    length_bytes(a585)
                                };
                                let a587 = 6u64;
                                nat_sub(a586, a587)
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
        let m589 = f149(v0, Bytes::lit(&[]));
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
            Ok(Ok::<(Bytes, Bytes), Bytes>((f145(), f150(v1)?)))
        } else {
            let m590 = f149(v0, v2.clone());
            match m590 {
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
        let (h591, _) = v0;
        h591
    }, f151()) {
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

pub fn f93(p597: Adt16, v1: Bytes) -> Option<Bytes> {
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

pub fn f94(v0: Adt16, v1: List<(Bytes, Bytes)>, v2: Bytes) -> R<Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>> {
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
            let m599 = f94(v4, v1.clone(), v2.clone())?;
            match m599 {
                Ok(v6) => {
                    let m600 = v3;
                    match m600 {
                        Adt16::C0(v7, r607, r608) => {
                            let v8 = (*r607).clone();
                            let v9 = (*r608).clone();
                            let m601 = f90(v8.clone(), v1);
                            match m601 {
                                Ok(v11) => {
                                    let m602 = f91(v11.clone(), v7)?;
                                    match m602 {
                                        Ok(v13) => {
                                            if f92(v13, Str::lit("simpleType")) {
                                                let m603 = f93(v8, f8(Str::lit("name")));
                                                match m603 {
                                                    None => {
                                                        Ok(f152(f8(Str::lit("a top-level simpleType must have a name (XSD 1.1 Part 2, section 4.1.2)"))))
                                                    }
                                                    Some(v14) => {
                                                        let m604 = f153(v14.clone(), v9, v11, v2)?;
                                                        match m604 {
                                                            Ok(v16) => {
                                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::cons((f154(v14)?, (match v16.clone() {
                                                                    Adt22::C0(_, h605) => {
                                                                        h605
                                                                    }
                                                                }, match v16 {
                                                                    Adt22::C0(h606, _) => {
                                                                        h606
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
    let m611 = v0;
    match m611.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h612, _) = v1;
                h612
            }, f95(v2))
        }
    }
}

pub fn f96(p615: List<Bytes>) -> Option<Bytes> {
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
                        if f115(v1.clone(), v2.clone()) {
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

pub fn f97(p621: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<Bytes> {
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
                        let m619 = f155({
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

pub fn f98(p632: u64, p633: List<(Bytes, (List<Bytes>, Bytes))>, p634: List<Bytes>, p635: List<Bytes>) -> Result<List<Bytes>, Bytes> {
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
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(f156(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
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
                            let m625 = f122(v1, v2.clone());
                            match m625 {
                                None => {
                                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(f156(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
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

pub fn f99(v0: Bytes) -> R<List<(Bytes, Bytes)>> {
    f157(f158(v0)?, None::<Bytes>)
}

pub fn f100(p656: u64, v1: List<(Bytes, Option<Bytes>)>, v2: List<(Bytes, Bytes)>, p657: List<(Bytes, (Bytes, Bytes))>, p658: List<Adt18>, p659: List<Bytes>) -> R<Adt17> {
    let m655 = (p656, (p657, (p658, p659)));
    {
        let mut m655 = m655;
        loop {
            let step = {
                let (v0, (v3, (v4, v5))) = m655;
                let m636 = v0;
                if m636 == 0 {
                    Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt18>, List<Bytes>))), R<Adt17>>(Ok(Adt17::C1(f8(Str::lit("the schema documents import and include more documents than the bound authorities hold")))))
                } else {
                    let v6 = m636 - 1;
                    let m637 = v3;
                    match m637.uncons() {
                        None => {
                            let m638 = v5.clone();
                            match m638.uncons() {
                                None => {
                                    let m639 = f159(f160(v4, List::<Adt18>::nil()))?;
                                    match m639 {
                                        Ok(v8) => {
                                            Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt18>, List<Bytes>))), R<Adt17>>(Ok(Adt17::C2({
                                                let (h640, _) = v8;
                                                h640
                                            }, {
                                                let (_, h641) = v8;
                                                h641
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
                                let (h642, _) = v11.clone();
                                h642
                            };
                            if if f115(v13.clone(), f161(v4.clone())) {
                                true
                            } else {
                                f115(v13.clone(), v5.clone())
                            } {
                                Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt18>, List<Bytes>))), R<Adt17>>((v6, (v12, (v4, v5))))
                            } else {
                                let m643 = f162(v1.clone(), v13.clone());
                                match m643 {
                                    None => {
                                        Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt18>, List<Bytes>))), R<Adt17>>((v6, (v12, (v4, List::cons(v13, v5)))))
                                    }
                                    Some(v14) => {
                                        let m644 = v14;
                                        match m644 {
                                            None => {
                                                Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt18>, List<Bytes>))), R<Adt17>>(Ok(Adt17::C1(f163(f8(Str::lit("the schema document ")), v13, f8(Str::lit(" is not in the repository"))))))
                                            }
                                            Some(v15) => {
                                                let m645 = f164(v13.clone(), v15, f165(v2.clone(), v13))?;
                                                match m645 {
                                                    Ok(v17) => {
                                                        let m650 = f166(v17.clone(), {
                                                            let (h646, _) = {
                                                                let (_, h647) = v11.clone();
                                                                h647
                                                            };
                                                            h646
                                                        }, {
                                                            let (_, h648) = {
                                                                let (_, h649) = v11;
                                                                h649
                                                            };
                                                            h648
                                                        });
                                                        match m650 {
                                                            None => {
                                                                let m653 = f167(match v17.clone() {
                                                                    Adt18::C0(_, _, _, _, h651, _) => {
                                                                        h651
                                                                    }
                                                                }, match v17.clone() {
                                                                    Adt18::C0(_, _, _, h652, _, _) => {
                                                                        h652
                                                                    }
                                                                })?;
                                                                match m653 {
                                                                    Ok(v20) => {
                                                                        let m654 = f168(v20, v17.clone(), v2.clone())?;
                                                                        match m654 {
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
                Ok(next) => { m655 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f101(v0: Bytes) -> R<u64> {
    let (h660, _) = f130(Fn4::F170(v0.clone()), {
        let a662 = {
            let a661 = v0;
            length_bytes(a661)
        };
        let a663 = 1u64;
        nat_add(a662, a663)?
    }, 0u64)?;
    Ok(h660)
}

pub fn f102(v0: Bytes, v1: u64) -> R<u64> {
    let (h664, _) = f130(Fn4::F171(v0.clone()), {
        let a666 = {
            let a665 = v0;
            length_bytes(a665)
        };
        let a667 = 1u64;
        nat_add(a666, a667)?
    }, v1)?;
    Ok(h664)
}

pub fn f103(v0: Bytes, p676: u64, p677: u64) -> R<Result<u64, Bytes>> {
    let m675 = (p676, p677);
    {
        let mut m675 = m675;
        loop {
            let step = {
                let (v1, v2) = m675;
                let m668 = v2;
                if m668 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Ok::<u64, Bytes>(v1)))
                } else {
                    let v3 = m668 - 1;
                    let a670 = v1;
                    let a671 = {
                        let a669 = v0.clone();
                        length_bytes(a669)
                    };
                    if if nat_lt(a670, a671) {
                        if f86(v0.clone(), v1, Str::lit("...")) {
                            f172(v0.clone(), v1)?
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        let m674 = f173(v0.clone(), {
                            let a672 = v1;
                            let a673 = 3u64;
                            nat_add(a672, a673)?
                        })?;
                        match m674 {
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
                Ok(next) => { m675 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f104(v0: Bytes, v1: u64, v2: bool, v3: u64) -> R<Result<List<Adt19>, Bytes>> {
    let m678 = v3;
    if m678 == 0 {
        Ok(Err::<List<Adt19>, Bytes>(f83(Str::lit("the stream holds more documents than it is long"), v1)?))
    } else {
        let v4 = m678 - 1;
        let v5: u64 = {
            let a679 = v0.clone();
            length_bytes(a679)
        };
        let a680 = f174(v0.clone(), v1)?;
        let a681 = 37u64;
        if nat_eq(a680, a681) {
            Ok(Err::<List<Adt19>, Bytes>(f83(Str::lit("a directive is not read (YAML 1.2.2 [82])"), v1)?))
        } else {
            let m688 = if if f86(v0.clone(), v1, Str::lit("---")) {
                f172(v0.clone(), v1)?
            } else {
                false
            } {
                f175(v0.clone(), {
                    let a682 = v1;
                    let a683 = 3u64;
                    nat_add(a682, a683)?
                }, 0u64, {
                    let a684 = v5;
                    let a685 = 2u64;
                    nat_add(a684, a685)?
                }, 1u64, false, 0u64, List::<Bytes>::nil())?
            } else {
                if v2 {
                    f175(v0.clone(), v1, 0u64, {
                        let a686 = v5;
                        let a687 = 2u64;
                        nat_add(a686, a687)?
                    }, 0u64, false, 0u64, List::<Bytes>::nil())?
                } else {
                    Adt20::C1(f83(Str::lit("only comments, a document end marker or a document start marker may follow a document (YAML 1.2.2 [207], [211])"), v1)?)
                }
            };
            match m688 {
                Adt20::C0(v7, v8) => {
                    let v9: u64 = f102(v0.clone(), v8)?;
                    let m691 = f103(v0.clone(), v9, {
                        let a689 = v5;
                        let a690 = 1u64;
                        nat_add(a689, a690)?
                    })?;
                    match m691 {
                        Ok(v11) => {
                            let a692 = v5;
                            let a693 = v11;
                            if nat_le(a692, a693) {
                                Ok(Ok::<List<Adt19>, Bytes>(List::cons(v7, List::<Adt19>::nil())))
                            } else {
                                let m696 = f104(v0, v11, f176({
                                    let a694 = v11;
                                    let a695 = v9;
                                    nat_eq(a694, a695)
                                }), v4)?;
                                match m696 {
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
    f177(v0)
}

pub fn f106(v0: Bytes, v1: u64) -> Option<u8> {
    let a697 = v0;
    let a698 = v1;
    index_bytes(a697, a698)
}

pub fn f107(v0: Bytes) -> R<Result<List<Adt21>, Bytes>> {
    f178({
        let a700 = {
            let a699 = v0.clone();
            length_bytes(a699)
        };
        let a701 = 1u64;
        nat_add(a700, a701)?
    }, v0, 0u64, List::<Adt21>::nil())
}

pub fn f108(p709: u64, p710: List<Adt21>, p711: List<Adt12>) -> R<Result<List<Adt12>, Bytes>> {
    let m708 = (p709, (p710, p711));
    {
        let mut m708 = m708;
        loop {
            let step = {
                let (v0, (v1, v2)) = m708;
                let m702 = v0;
                if m702 == 0 {
                    Err::<(u64, (List<Adt21>, List<Adt12>)), R<Result<List<Adt12>, Bytes>>>(Ok(Err::<List<Adt12>, Bytes>(f8(Str::lit("the input has more definitions than tokens")))))
                } else {
                    let v3 = m702 - 1;
                    let m703 = v1.clone();
                    match m703.uncons() {
                        None => {
                            Err::<(u64, (List<Adt21>, List<Adt12>)), R<Result<List<Adt12>, Bytes>>>(Ok(Ok::<List<Adt12>, Bytes>(f179(v2, List::<Adt12>::nil()))))
                        }
                        Some((_, _)) => {
                            let m704 = f180(v1)?;
                            match m704 {
                                Ok(v7) => {
                                    let m705 = f181(v3, v7)?;
                                    match m705 {
                                        Ok(v9) => {
                                            Ok::<(u64, (List<Adt21>, List<Adt12>)), R<Result<List<Adt12>, Bytes>>>((v3, ({
                                                let (_, h706) = v9;
                                                h706
                                            }, List::cons({
                                                let (h707, _) = v9;
                                                h707
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
                Ok(next) => { m708 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f109(v0: List<Adt15>, v1: List<Adt15>) -> List<Adt15> {
    let m712 = v0;
    match m712.uncons() {
        None => {
            List::<Adt15>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = if f35(f117(v2.clone()), Bytes::lit(&[])) {
                f182(f119(v2.clone()), v1.clone())
            } else {
                f117(v2.clone())
            };
            List::cons(Adt15::C0(f120(v2.clone()), v4, f121(v2.clone()), f119(v2)), f109(v3, v1))
        }
    }
}

pub fn f110(v0: List<Adt15>) -> R<u64> {
    let m713 = v0;
    match m713.uncons() {
        None => {
            Ok(0u64)
        }
        Some((v1, v2)) => {
            if f35(f117(v1), Bytes::lit(&[])) {
                f110(v2)
            } else {
                let a714 = f110(v2)?;
                let a715 = 1u64;
                nat_add(a714, a715)
            }
        }
    }
}

pub fn f111(v0: u64) -> bool {
    let a716 = v0;
    let a717 = 0u64;
    if nat_eq(a716, a717) {
        true
    } else {
        let a718 = v0;
        let a719 = 2u64;
        if nat_eq(a718, a719) {
            true
        } else {
            let a720 = v0;
            let a721 = 3u64;
            if nat_eq(a720, a721) {
                true
            } else {
                let a722 = v0;
                let a723 = 4u64;
                if nat_eq(a722, a723) {
                    true
                } else {
                    let a724 = v0;
                    let a725 = 9u64;
                    nat_eq(a724, a725)
                }
            }
        }
    }
}

pub fn f112(v0: Adt12) -> u64 {
    let m726 = v0;
    match m726 {
        Adt12::C0(v1, _, _, _, _, _) => {
            v1
        }
    }
}

pub fn f113(v0: List<Adt12>, v1: Adt12) -> R<Result<(Bytes, List<Bytes>), Bytes>> {
    let v2: u64 = f112(v1.clone());
    let v3: Bytes = f183(f114(v1.clone()))?;
    let a727 = v2;
    let a728 = 0u64;
    if if nat_eq(a727, a728) {
        true
    } else {
        let a729 = v2;
        let a730 = 2u64;
        nat_eq(a729, a730)
    } {
        let m731 = f184(v0.clone(), v0.clone(), f114(v1.clone()));
        match m731 {
            None => {
                let m735 = f185({
                    let a733 = {
                        let a732 = v0.clone();
                        length_list(a732)
                    };
                    let a734 = 1u64;
                    nat_add(a733, a734)?
                }, v0, v1)?;
                match m735 {
                    Ok(v5) => {
                        let a737 = {
                            let a736 = v5.clone();
                            length_list(a736)
                        };
                        let a738 = 0u64;
                        if nat_eq(a737, a738) {
                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f186(v3), List::<Bytes>::nil())))
                        } else {
                            let m739 = f187(f188(f189(v5.clone()))?, v5)?;
                            match m739 {
                                Ok(v7) => {
                                    Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f190(v3, {
                                        let (h740, _) = v7;
                                        h740
                                    }), {
                                        let (_, h741) = v7;
                                        h741
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
        let a742 = v2;
        let a743 = 3u64;
        if nat_eq(a742, a743) {
            let m744 = f96(f191(v1.clone()));
            match m744 {
                None => {
                    let m745 = f192(f188(f191(v1.clone()))?, f193(f191(v1)), List::<Bytes>::nil())?;
                    match m745 {
                        Ok(v10) => {
                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f194(v3, v10), List::<Bytes>::nil())))
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
            let a746 = v2;
            let a747 = 4u64;
            if nat_eq(a746, a747) {
                let v12: Adt14 = f195(v1);
                if !f35(f196(v12.clone()), Bytes::lit(&[])) {
                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f196(v12)))
                } else {
                    let a748 = f197(v12.clone());
                    let a749 = 1u64;
                    if nat_eq(a748, a749) {
                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f194(v3, List::cons(f198(v12.clone()), List::<Bytes>::nil())), f199(v12))))
                    } else {
                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f200(v3, f198(v12.clone())), f199(v12))))
                    }
                }
            } else {
                Ok(Err::<(Bytes, List<Bytes>), Bytes>(f7(f191(v1))))
            }
        }
    }
}

pub fn f114(v0: Adt12) -> Bytes {
    let m750 = v0;
    match m750 {
        Adt12::C0(_, v2, _, _, _, _) => {
            v2
        }
    }
}

pub fn f115(v0: Bytes, p753: List<Bytes>) -> bool {
    let m752 = p753;
    {
        let mut m752 = m752;
        loop {
            let step = {
                let v1 = m752;
                let m751 = v1;
                match m751.uncons() {
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
                Ok(next) => { m752 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f116(p756: List<Adt15>, v1: Bytes) -> Option<Adt15> {
    let m755 = p756;
    {
        let mut m755 = m755;
        loop {
            let step = {
                let v0 = m755;
                let m754 = v0;
                match m754.uncons() {
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
                Ok(next) => { m755 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f117(v0: Adt15) -> Bytes {
    let m757 = v0;
    match m757 {
        Adt15::C0(_, v2, _, _) => {
            v2
        }
    }
}

pub fn f118(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m758 = v0;
    match m758.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f118(v3, v1))
        }
    }
}

pub fn f119(v0: Adt15) -> List<Bytes> {
    let m759 = v0;
    match m759 {
        Adt15::C0(_, _, _, v4) => {
            v4
        }
    }
}

pub fn f120(v0: Adt15) -> Bytes {
    let m760 = v0;
    match m760 {
        Adt15::C0(v1, _, _, _) => {
            v1
        }
    }
}

pub fn f121(v0: Adt15) -> Bytes {
    let m761 = v0;
    match m761 {
        Adt15::C0(_, _, v3, _) => {
            v3
        }
    }
}

pub fn f122(v0: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)> {
    let m762 = v0;
    match m762.uncons() {
        None => {
            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
        }
        Some((v2, v3)) => {
            let m765 = f155({
                let (h763, _) = {
                    let (_, h764) = v2.clone();
                    h764
                };
                h763
            }, v1.clone());
            match m765 {
                None => {
                    Some((({
                        let (h766, _) = v2;
                        h766
                    }, {
                        let (_, h767) = {
                            let (_, h768) = v2;
                            h768
                        };
                        h767
                    }), v3))
                }
                Some(_) => {
                    let m769 = f122(v3, v1);
                    match m769 {
                        None => {
                            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h770, _) = v5;
                                h770
                            }, List::cons(v2, {
                                let (_, h771) = v5;
                                h771
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f123(p776: List<Adt12>, v1: u64, v2: Bytes) -> Option<Adt12> {
    let m775 = p776;
    {
        let mut m775 = m775;
        loop {
            let step = {
                let v0 = m775;
                let m772 = v0;
                match m772.uncons() {
                    None => {
                        Err::<List<Adt12>, Option<Adt12>>(None::<Adt12>)
                    }
                    Some((v3, v4)) => {
                        let a773 = f112(v3.clone());
                        let a774 = v1;
                        if if nat_eq(a773, a774) {
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
                Ok(next) => { m775 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f124(v0: u64) -> Bytes {
    let a777 = v0;
    let a778 = 5u64;
    if nat_eq(a777, a778) {
        f8(Str::lit("partial interface "))
    } else {
        let a779 = v0;
        let a780 = 7u64;
        if nat_eq(a779, a780) {
            f8(Str::lit("partial dictionary "))
        } else {
            f8(Str::lit(""))
        }
    }
}

pub fn f125(v0: Adt12) -> Bytes {
    let m781 = v0;
    match m781 {
        Adt12::C0(_, _, v3, _, _, _) => {
            v3
        }
    }
}

pub fn f126(v0: Bytes, v1: u64) -> Bytes {
    let m785 = {
        let a782 = v0;
        let a783 = v1;
        let a784 = 1u64;
        slice_bytes(a782, a783, a784)
    };
    match m785 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f127(v0: Fn3, p790: u64, p791: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m789 = (p790, p791);
    {
        let mut m789 = m789;
        loop {
            let step = {
                let (v1, v2) = m789;
                let m786 = v1;
                if m786 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m786 - 1;
                    let m788 = {
                        let c787: Fn3 = v0.clone();
                        c787.apply(v2.clone())?
                    };
                    match m788 {
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
                Ok(next) => { m789 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f128(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f201(v1, v0, v2)
}

pub fn f129(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f202(v0, v1)
}

pub fn f130(v0: Fn4, p796: u64, p797: u64) -> R<(u64, bool)> {
    let m795 = (p796, p797);
    {
        let mut m795 = m795;
        loop {
            let step = {
                let (v1, v2) = m795;
                let m792 = v1;
                if m792 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m792 - 1;
                    let m794 = {
                        let c793: Fn4 = v0.clone();
                        c793.apply(v2)?
                    };
                    match m794 {
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
                Ok(next) => { m795 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f131(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f203(v0, v1)
}

pub fn f132(v0: u64) -> R<bool> {
    f204(v0, f205())
}

pub fn f133(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let v3: u64 = {
        let (h798, _) = f130(Fn4::F206(v1.clone(), v0.clone()), {
            let a800 = {
                let a799 = v0.clone();
                length_bytes(a799)
            };
            let a801 = 1u64;
            nat_add(a800, a801)?
        }, v2)?;
        h798
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
    let (h802, _) = f130(Fn4::F207(v0.clone()), {
        let a804 = {
            let a803 = v0;
            length_bytes(a803)
        };
        let a805 = 1u64;
        nat_add(a804, a805)?
    }, v1)?;
    Ok(h802)
}

pub fn f136(v0: Bytes) -> bool {
    f208(v0, List::cons(f8(Str::lit("UTF-8")), List::cons(f8(Str::lit("UTf-8")), List::cons(f8(Str::lit("UtF-8")), List::cons(f8(Str::lit("Utf-8")), List::cons(f8(Str::lit("uTF-8")), List::cons(f8(Str::lit("uTf-8")), List::cons(f8(Str::lit("utF-8")), List::cons(f8(Str::lit("utf-8")), List::<Bytes>::nil())))))))))
}

pub fn f137(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let m809 = f133(v0.clone(), {
        let a806 = Str::lit("--");
        utf8_encode(a806)
    }, {
        let a807 = v1;
        let a808 = 4u64;
        nat_add(a807, a808)?
    })?;
    match m809 {
        None => {
            Ok(f134(f83(Str::lit("a comment is not closed (XML 1.0 [15])"), v1)?))
        }
        Some(v2) => {
            let a812 = f87(v0, {
                let a810 = v2;
                let a811 = 2u64;
                nat_add(a810, a811)?
            })?;
            let a813 = 62u64;
            if nat_eq(a812, a813) {
                Ok(Ok::<u64, Bytes>({
                    let a814 = v2;
                    let a815 = 3u64;
                    nat_add(a814, a815)?
                }))
            } else {
                Ok(f134(f83(Str::lit("-- is not allowed inside a comment (XML 1.0 [15])"), v2)?))
            }
        }
    }
}

pub fn f138(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f141(v0.clone(), {
        let a816 = v1;
        let a817 = 2u64;
        nat_add(a816, a817)?
    })?;
    let a820 = v2;
    let a821 = {
        let a818 = v1;
        let a819 = 2u64;
        nat_add(a818, a819)?
    };
    if nat_eq(a820, a821) {
        Ok(f134(f83(Str::lit("a processing instruction must start with a Name (XML 1.0 [16])"), v1)?))
    } else {
        let a824 = v2;
        let a825 = {
            let a822 = v1;
            let a823 = 5u64;
            nat_add(a822, a823)?
        };
        if if nat_eq(a824, a825) {
            f208(f32(v0.clone(), {
                let a826 = v1;
                let a827 = 2u64;
                nat_add(a826, a827)?
            }, 3u64), List::cons(f8(Str::lit("XML")), List::cons(f8(Str::lit("XMl")), List::cons(f8(Str::lit("XmL")), List::cons(f8(Str::lit("Xml")), List::cons(f8(Str::lit("xML")), List::cons(f8(Str::lit("xMl")), List::cons(f8(Str::lit("xmL")), List::cons(f8(Str::lit("xml")), List::<Bytes>::nil())))))))))
        } else {
            false
        } {
            Ok(f134(f83(Str::lit("the target xml is reserved; an XML declaration may appear only at the very start (XML 1.0 [17], [22])"), v1)?))
        } else {
            let m829 = f133(v0, {
                let a828 = Str::lit("?>");
                utf8_encode(a828)
            }, v2)?;
            match m829 {
                None => {
                    Ok(f134(f83(Str::lit("a processing instruction is not closed (XML 1.0 [16])"), v1)?))
                }
                Some(v3) => {
                    Ok(Ok::<u64, Bytes>({
                        let a830 = v3;
                        let a831 = 2u64;
                        nat_add(a830, a831)?
                    }))
                }
            }
        }
    }
}

pub fn f139(v0: Bytes, v1: u64) -> R<u64> {
    let m834 = {
        let a832 = v0;
        let a833 = v1;
        index_bytes(a832, a833)
    };
    match m834 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a879 = {
                let a855 = {
                    let a843 = {
                        let a837 = {
                            let a835 = v3;
                            let a836 = 128u8;
                            fixed_u8::bit_and(a835, a836)
                        };
                        let a838 = 128u8;
                        if equal(a837, a838) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a844 = {
                        let a841 = {
                            let a839 = v3;
                            let a840 = 64u8;
                            fixed_u8::bit_and(a839, a840)
                        };
                        let a842 = 64u8;
                        if equal(a841, a842) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a843, a844)?
                };
                let a856 = {
                    let a853 = {
                        let a847 = {
                            let a845 = v3;
                            let a846 = 32u8;
                            fixed_u8::bit_and(a845, a846)
                        };
                        let a848 = 32u8;
                        if equal(a847, a848) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a854 = {
                        let a851 = {
                            let a849 = v3;
                            let a850 = 16u8;
                            fixed_u8::bit_and(a849, a850)
                        };
                        let a852 = 16u8;
                        if equal(a851, a852) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a853, a854)?
                };
                nat_add(a855, a856)?
            };
            let a880 = {
                let a877 = {
                    let a865 = {
                        let a859 = {
                            let a857 = v3;
                            let a858 = 8u8;
                            fixed_u8::bit_and(a857, a858)
                        };
                        let a860 = 8u8;
                        if equal(a859, a860) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a866 = {
                        let a863 = {
                            let a861 = v3;
                            let a862 = 4u8;
                            fixed_u8::bit_and(a861, a862)
                        };
                        let a864 = 4u8;
                        if equal(a863, a864) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a865, a866)?
                };
                let a878 = {
                    let a875 = {
                        let a869 = {
                            let a867 = v3;
                            let a868 = 2u8;
                            fixed_u8::bit_and(a867, a868)
                        };
                        let a870 = 2u8;
                        if equal(a869, a870) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a876 = {
                        let a873 = {
                            let a871 = v3;
                            let a872 = 1u8;
                            fixed_u8::bit_and(a871, a872)
                        };
                        let a874 = 1u8;
                        if equal(a873, a874) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a875, a876)?
                };
                nat_add(a877, a878)?
            };
            nat_add(a879, a880)
        }
    }
}

pub fn f140(v0: Bytes) -> Result<(Adt16, u64), Bytes> {
    Err::<(Adt16, u64), Bytes>(v0)
}

pub fn f141(v0: Bytes, v1: u64) -> R<u64> {
    let (h881, _) = f130(Fn4::F209(v0.clone(), v1), {
        let a883 = {
            let a882 = v0;
            length_bytes(a882)
        };
        let a884 = 1u64;
        nat_add(a883, a884)?
    }, v1)?;
    Ok(h881)
}

pub fn f142(v0: Bytes, v1: u64, v2: u64, v3: List<Bytes>) -> R<Result<(Adt16, u64), Bytes>> {
    let m885 = v2;
    if m885 == 0 {
        Ok(f140(f83(Str::lit("a tag has more attributes than the input has bytes"), v1)?))
    } else {
        let v4 = m885 - 1;
        let v5: u64 = f135(v0.clone(), v1)?;
        let a886 = f87(v0.clone(), v5)?;
        let a887 = 62u64;
        if if nat_eq(a886, a887) {
            true
        } else {
            f86(v0.clone(), v5, Str::lit("/>"))
        } {
            Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C4, v5)))
        } else {
            let a888 = v5;
            let a889 = v1;
            if nat_eq(a888, a889) {
                Ok(f140(f83(Str::lit("attributes must be separated by white space (XML 1.0 [40])"), v1)?))
            } else {
                let v6: u64 = f141(v0.clone(), v5)?;
                let a890 = v6;
                let a891 = v5;
                if nat_eq(a890, a891) {
                    Ok(f140(f83(Str::lit("an attribute must start with a Name (XML 1.0 [41])"), v5)?))
                } else {
                    let v7: Bytes = f32(v0.clone(), v5, {
                        let a892 = v6;
                        let a893 = v5;
                        nat_sub(a892, a893)
                    });
                    if f208(v7.clone(), v3.clone()) {
                        Ok(f140(f83(Str::lit("an attribute name appears twice in one tag (XML 1.0 WFC: Unique Att Spec)"), v5)?))
                    } else {
                        let v8: u64 = f135(v0.clone(), v6)?;
                        let a894 = f87(v0.clone(), v8)?;
                        let a895 = 61u64;
                        if nat_eq(a894, a895) {
                            let v9: u64 = f135(v0.clone(), {
                                let a896 = v8;
                                let a897 = 1u64;
                                nat_add(a896, a897)?
                            })?;
                            let v10: u64 = f87(v0.clone(), v9)?;
                            let a898 = v10;
                            let a899 = 34u64;
                            if if nat_eq(a898, a899) {
                                true
                            } else {
                                let a900 = v10;
                                let a901 = 39u64;
                                nat_eq(a900, a901)
                            } {
                                let m904 = f143(v0.clone(), {
                                    let a902 = v9;
                                    let a903 = 1u64;
                                    nat_add(a902, a903)?
                                }, v4, Bytes::lit(&[]), true, v10)?;
                                match m904 {
                                    Ok(v12) => {
                                        let m906 = f142(v0, {
                                            let (_, h905) = v12;
                                            h905
                                        }, v4, List::cons(v7.clone(), v3))?;
                                        match m906 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C1(v7, {
                                                    let (h907, _) = v12;
                                                    h907
                                                }, std::rc::Rc::new({
                                                    let (h908, _) = v14;
                                                    h908
                                                })), {
                                                    let (_, h909) = v14;
                                                    h909
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

pub fn f143(v0: Bytes, p949: u64, p950: u64, p951: Bytes, v4: bool, v5: u64) -> R<Result<(Bytes, u64), Bytes>> {
    let m948 = (p949, (p950, p951));
    {
        let mut m948 = m948;
        loop {
            let step = {
                let (v1, (v2, v3)) = m948;
                let m910 = v2;
                if m910 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f210(f83(Str::lit("the input ends inside a text run"), v1)?)))
                } else {
                    let v6 = m910 - 1;
                    let v7: u64 = f87(v0.clone(), v1)?;
                    if if v4 {
                        let a911 = v7;
                        let a912 = v5;
                        nat_eq(a911, a912)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, {
                            let a913 = v1;
                            let a914 = 1u64;
                            nat_add(a913, a914)?
                        }))))
                    } else {
                        let a915 = v7;
                        let a916 = 256u64;
                        if nat_eq(a915, a916) {
                            if v4 {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f210(f83(Str::lit("an attribute value is not closed (XML 1.0 [10])"), v1)?)))
                            } else {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                            }
                        } else {
                            let a917 = v7;
                            let a918 = 60u64;
                            if nat_eq(a917, a918) {
                                if v4 {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f210(f83(Str::lit("< is not allowed in an attribute value (XML 1.0 [10], WFC: No < in Attribute Values)"), v1)?)))
                                } else {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                                }
                            } else {
                                let a919 = v7;
                                let a920 = 38u64;
                                if nat_eq(a919, a920) {
                                    let m921 = f211(v0.clone(), v1)?;
                                    match m921 {
                                        Ok(v9) => {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let (_, h922) = v9;
                                                h922
                                            }, (v6, {
                                                let a924 = v3;
                                                let a925 = {
                                                    let (h923, _) = v9;
                                                    h923
                                                };
                                                append_bytes(a924, a925)
                                            })))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f210(v8)))
                                        }
                                    }
                                } else {
                                    if v4 {
                                        let a926 = v7;
                                        let a927 = 9u64;
                                        if if nat_eq(a926, a927) {
                                            true
                                        } else {
                                            let a928 = v7;
                                            let a929 = 10u64;
                                            nat_eq(a928, a929)
                                        } {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let a930 = v1;
                                                let a931 = 1u64;
                                                nat_add(a930, a931)?
                                            }, (v6, {
                                                let a932 = v3;
                                                let a933 = f8(Str::lit(" "));
                                                append_bytes(a932, a933)
                                            })))
                                        } else {
                                            let v10: u64 = f212(v0.clone(), true, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v10, (v6, {
                                                let a936 = v3;
                                                let a937 = f32(v0.clone(), v1, {
                                                    let a934 = v10;
                                                    let a935 = v1;
                                                    nat_sub(a934, a935)
                                                });
                                                append_bytes(a936, a937)
                                            })))
                                        }
                                    } else {
                                        let a938 = v7;
                                        let a939 = 93u64;
                                        if nat_eq(a938, a939) {
                                            if f86(v0.clone(), v1, Str::lit("]]>")) {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f210(f83(Str::lit("]]> is not allowed in character data (XML 1.0 [14])"), v1)?)))
                                            } else {
                                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                    let a940 = v1;
                                                    let a941 = 1u64;
                                                    nat_add(a940, a941)?
                                                }, (v6, {
                                                    let a942 = v3;
                                                    let a943 = f8(Str::lit("]"));
                                                    append_bytes(a942, a943)
                                                })))
                                            }
                                        } else {
                                            let v11: u64 = f212(v0.clone(), false, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v11, (v6, {
                                                let a946 = v3;
                                                let a947 = f32(v0.clone(), v1, {
                                                    let a944 = v11;
                                                    let a945 = v1;
                                                    nat_sub(a944, a945)
                                                });
                                                append_bytes(a946, a947)
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
                Ok(next) => { m948 = next; }
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
    let m956 = {
        let a953 = v0;
        let a954 = 0u64;
        let a955 = {
            let a952 = v1.clone();
            length_bytes(a952)
        };
        slice_bytes(a953, a954, a955)
    };
    match m956 {
        None => {
            false
        }
        Some(v2) => {
            let a957 = v2;
            let a958 = v1;
            equal(a957, a958)
        }
    }
}

pub fn f148(v0: Bytes) -> R<Bytes> {
    let m959 = f133(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m959 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v1) => {
            Ok(f32(v0, 0u64, v1))
        }
    }
}

pub fn f149(p964: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m963 = p964;
    {
        let mut m963 = m963;
        loop {
            let step = {
                let v0 = m963;
                let m960 = v0;
                match m960.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h961, _) = v2;
                            h961
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h962) = v2;
                                h962
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m963 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f150(v0: Bytes) -> R<Bytes> {
    let m965 = f133(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m965 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(f32(v0.clone(), {
                let a966 = v1;
                let a967 = 1u64;
                nat_add(a966, a967)?
            }, {
                let a971 = {
                    let a968 = v0;
                    length_bytes(a968)
                };
                let a972 = {
                    let a969 = v1;
                    let a970 = 1u64;
                    nat_add(a969, a970)?
                };
                nat_sub(a971, a972)
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
    let m973 = f213(v1, v2)?;
    match m973 {
        Ok(v6) => {
            let m974 = v6;
            match m974 {
                None => {
                    Ok(f214(f215(f8(Str::lit("a simpleType must have a restriction, list or union child (XSD 1.1 Part 2, section 4.1.2)")), v0)))
                }
                Some(v7) => {
                    let v8: Bytes = match v7.clone() {
                        Adt27::C0(h975, _, _, _) => {
                            h975
                        }
                    };
                    let v9: Adt16 = match v7.clone() {
                        Adt27::C0(_, h976, _, _) => {
                            h976
                        }
                    };
                    let v10: Adt16 = match v7.clone() {
                        Adt27::C0(_, _, h977, _) => {
                            h977
                        }
                    };
                    let v11: List<(Bytes, Bytes)> = match v7 {
                        Adt27::C0(_, _, _, h978) => {
                            h978
                        }
                    };
                    if f35(v8.clone(), f8(Str::lit("union"))) {
                        Ok(f214(f215(f8(Str::lit("a union is not translated, because a literal can be valid for more than one member type, so no LexLean inductive keeps its members disjoint (XSD 1.1 Part 2, section 2.4.1.3)")), v0)))
                    } else {
                        if f35(v8, f8(Str::lit("list"))) {
                            let m979 = f93(v9, f8(Str::lit("itemType")));
                            match m979 {
                                None => {
                                    Ok(f214(f215(f8(Str::lit("a list with an anonymous item type is not translated yet")), v0)))
                                }
                                Some(v12) => {
                                    let m980 = f216(v11, v3, v12)?;
                                    match m980 {
                                        Ok(v14) => {
                                            if f147(match v14.clone() {
                                                Adt22::C0(h981, _) => {
                                                    h981
                                                }
                                            }, f8(Str::lit("(List "))) {
                                                Ok(f214(f215(f8(Str::lit("the item type of a list must not itself be a list (XSD 1.1 Part 2, section 4.1.5)")), v0)))
                                            } else {
                                                Ok(Ok::<Adt22, Bytes>(Adt22::C0(f200(v4, f7(List::cons(f8(Str::lit("(List ")), List::cons(match v14.clone() {
                                                    Adt22::C0(h982, _) => {
                                                        h982
                                                    }
                                                }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))), match v14 {
                                                    Adt22::C0(_, h983) => {
                                                        h983
                                                    }
                                                })))
                                            }
                                        }
                                        Err(v13) => {
                                            Ok(f214(f215(v13, v0)))
                                        }
                                    }
                                }
                            }
                        } else {
                            let m984 = f93(v9, f8(Str::lit("base")));
                            match m984 {
                                None => {
                                    Ok(f214(f215(f8(Str::lit("a restriction with an anonymous base type is not translated yet")), v0)))
                                }
                                Some(v15) => {
                                    let m985 = f216(v11.clone(), v3, v15)?;
                                    match m985 {
                                        Ok(v17) => {
                                            let m986 = f217(v10, v11)?;
                                            match m986 {
                                                Ok(v19) => {
                                                    let v20: List<Adt19> = f218(v19, List::<Bytes>::nil())?;
                                                    let m987 = v20.clone();
                                                    match m987.uncons() {
                                                        None => {
                                                            Ok(Ok::<Adt22, Bytes>(Adt22::C0(f200(v4, match v17.clone() {
                                                                Adt22::C0(h988, _) => {
                                                                    h988
                                                                }
                                                            }), match v17 {
                                                                Adt22::C0(_, h989) => {
                                                                    h989
                                                                }
                                                            })))
                                                        }
                                                        Some((_, _)) => {
                                                            let m990 = f192(f188(f219(v20.clone()))?, v20, List::<Bytes>::nil())?;
                                                            match m990 {
                                                                Ok(v24) => {
                                                                    Ok(Ok::<Adt22, Bytes>(Adt22::C0(f194(v4, v24), List::<Bytes>::nil())))
                                                                }
                                                                Err(v23) => {
                                                                    Ok(f214(f215(v23, v0)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Err(v18) => {
                                                    Ok(f214(f215(v18, v0)))
                                                }
                                            }
                                        }
                                        Err(v16) => {
                                            Ok(f214(f215(v16, v0)))
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
            Ok(f214(f215(v5, v0)))
        }
    }
}

pub fn f154(v0: Bytes) -> R<Bytes> {
    f220(v0.clone(), 0u64, {
        let a991 = v0;
        length_bytes(a991)
    }, true, Bytes::lit(&[]))
}

pub fn f155(p994: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m993 = p994;
    {
        let mut m993 = m993;
        loop {
            let step = {
                let v0 = m993;
                let m992 = v0;
                match m992.uncons() {
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
                Ok(next) => { m993 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f156(v0: Bytes) -> Result<List<Bytes>, Bytes> {
    Err::<List<Bytes>, Bytes>(v0)
}

pub fn f157(v0: List<Bytes>, v1: Option<Bytes>) -> R<List<(Bytes, Bytes)>> {
    let m995 = v0;
    match m995.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, v3)) => {
            if f35(v2.clone(), f8(Str::lit("[[authority]]"))) {
                f157(v3, None::<Bytes>)
            } else {
                let m996 = f221(v2.clone(), f8(Str::lit("url = ")))?;
                match m996 {
                    None => {
                        let m997 = f221(v2, f8(Str::lit("artifact = ")))?;
                        match m997 {
                            None => {
                                f157(v3, v1)
                            }
                            Some(v5) => {
                                let m998 = v1.clone();
                                match m998 {
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
        let a999 = v0.clone();
        length_bytes(a999)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h1000, _) = f43(Fn0::F222(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h1000
    };
    let v3: u64 = {
        let (h1001, _) = {
            let (_, h1002) = v2.clone();
            h1002
        };
        h1001
    };
    let v4: List<Bytes> = {
        let (_, h1003) = {
            let (_, h1004) = v2;
            h1004
        };
        h1003
    };
    let a1005 = v3;
    let a1006 = v1;
    if nat_lt(a1005, a1006) {
        Ok(f45(List::cons(f32(v0, v3, {
            let a1007 = v1;
            let a1008 = v3;
            nat_sub(a1007, a1008)
        }), v4)))
    } else {
        Ok(f45(v4))
    }
}

pub fn f159(v0: List<Adt18>) -> R<Result<(u64, Bytes), Bytes>> {
    let m1009 = f223(v0.clone())?;
    match m1009 {
        Ok(v2) => {
            let v3: Adt23 = Adt23::C0(v2.clone(), f224(v0.clone()));
            let m1010 = f225(f226(v0)?, v3, v2)?;
            match m1010 {
                Ok(v5) => {
                    let m1011 = f227(f228(v5), List::<Adt26>::nil());
                    match m1011 {
                        Ok(v7) => {
                            let v8: List<Bytes> = f229(v7.clone());
                            let m1012 = f230(v7.clone(), v8.clone());
                            match m1012 {
                                None => {
                                    let v10: List<(Bytes, List<Bytes>)> = f231(v7.clone(), v8.clone(), f232(v7.clone())?);
                                    let m1018 = f233({
                                        let a1016 = {
                                            let a1015 = v8.clone();
                                            length_list(a1015)
                                        };
                                        let a1017 = 1u64;
                                        nat_add(a1016, a1017)?
                                    }, v7.clone(), f234(v7, f235(v10, v8.clone(), v8, List::<Bytes>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil());
                                    match m1018 {
                                        Ok(v12) => {
                                            Ok(Ok::<(u64, Bytes), Bytes>(({
                                                let a1019 = v12.clone();
                                                length_list(a1019)
                                            }, f75(v12, f8(Str::lit("\u{a}"))))))
                                        }
                                        Err(v11) => {
                                            Ok(Err::<(u64, Bytes), Bytes>(v11))
                                        }
                                    }
                                }
                                Some(v9) => {
                                    Ok(Err::<(u64, Bytes), Bytes>(f163(f8(Str::lit("the type ")), {
                                        let (h1013, _) = v9;
                                        h1013
                                    }, f163(f8(Str::lit(" uses ")), {
                                        let (_, h1014) = v9;
                                        h1014
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

pub fn f160(p1022: List<Adt18>, p1023: List<Adt18>) -> List<Adt18> {
    let m1021 = (p1022, p1023);
    {
        let mut m1021 = m1021;
        loop {
            let step = {
                let (v0, v1) = m1021;
                let m1020 = v0;
                match m1020.uncons() {
                    None => {
                        Err::<(List<Adt18>, List<Adt18>), List<Adt18>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt18>, List<Adt18>), List<Adt18>>((v3, List::cons(v2, v1)))
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

pub fn f161(v0: List<Adt18>) -> List<Bytes> {
    let m1024 = v0;
    match m1024.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt18::C0(h1025, _, _, _, _, _) => {
                    h1025
                }
            }, f161(v2))
        }
    }
}

pub fn f162(p1030: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m1029 = p1030;
    {
        let mut m1029 = m1029;
        loop {
            let step = {
                let v0 = m1029;
                let m1026 = v0;
                match m1026.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h1027, _) = v2;
                            h1027
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h1028) = v2;
                                h1028
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1029 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f163(v0: Bytes, v1: Bytes, v2: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(v1, List::cons(v2, List::<Bytes>::nil()))))
}

pub fn f164(v0: Bytes, v1: Bytes, v2: Option<Bytes>) -> R<Result<Adt18, Bytes>> {
    let m1031 = f53(v1.clone())?;
    match m1031 {
        Ok(v4) => {
            let m1032 = v4;
            match m1032 {
                Adt16::C0(v5, r1037, r1038) => {
                    let v6 = (*r1037).clone();
                    let v7 = (*r1038).clone();
                    let m1033 = f90(v6.clone(), List::<(Bytes, Bytes)>::nil());
                    match m1033 {
                        Ok(v9) => {
                            let m1034 = f91(v9.clone(), v5)?;
                            match m1034 {
                                Ok(v11) => {
                                    if f92(v11, Str::lit("schema")) {
                                        Ok(Ok::<Adt18, Bytes>(Adt18::C0(v0, v2, {
                                            let m1035 = f93(v6, f8(Str::lit("targetNamespace")));
                                            match m1035 {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some(v12) => {
                                                    v12
                                                }
                                            }
                                        }, v9, v7, {
                                            let a1036 = v1;
                                            length_bytes(a1036)
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

pub fn f165(p1043: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1042 = p1043;
    {
        let mut m1042 = m1042;
        loop {
            let step = {
                let v0 = m1042;
                let m1039 = v0;
                match m1039.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (_, h1040) = v2;
                            h1040
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (h1041, _) = v2;
                                h1041
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1042 = next; }
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
            Adt18::C0(_, _, h1044, _, _, _) => {
                h1044
            }
        }, v2.clone()) {
            None::<Bytes>
        } else {
            Some(f7(List::cons(match v0.clone() {
                Adt18::C0(h1045, _, _, _, _, _) => {
                    h1045
                }
            }, List::cons(f8(Str::lit(" has the target namespace ")), List::cons(match v0 {
                Adt18::C0(_, _, h1046, _, _, _) => {
                    h1046
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
    let m1047 = v0;
    match m1047 {
        Adt16::C0(_, _, _) => {
            Ok(Ok::<List<Adt25>, Bytes>(List::<Adt25>::nil()))
        }
        Adt16::C1(_, _, _) => {
            Ok(Ok::<List<Adt25>, Bytes>(List::<Adt25>::nil()))
        }
        Adt16::C2(_) => {
            Ok(Ok::<List<Adt25>, Bytes>(List::<Adt25>::nil()))
        }
        Adt16::C3(r1058, r1059) => {
            let v2 = (*r1058).clone();
            let v3 = (*r1059).clone();
            let m1048 = f167(v3, v1.clone())?;
            match m1048 {
                Ok(v5) => {
                    let m1049 = v2;
                    match m1049 {
                        Adt16::C0(v6, r1056, r1057) => {
                            let v7 = (*r1056).clone();
                            let v8 = (*r1057).clone();
                            let m1050 = f90(v7.clone(), v1);
                            match m1050 {
                                Ok(v10) => {
                                    let m1051 = f91(v10.clone(), v6)?;
                                    match m1051 {
                                        Ok(v12) => {
                                            if if f35({
                                                let (h1052, _) = v12.clone();
                                                h1052
                                            }, f151()) {
                                                f236(f35({
                                                    let (_, h1053) = v12.clone();
                                                    h1053
                                                }, f8(Str::lit("annotation"))))
                                            } else {
                                                false
                                            } {
                                                Ok(Ok::<List<Adt25>, Bytes>(List::cons(Adt25::C0({
                                                    let (h1054, _) = v12;
                                                    h1054
                                                }, {
                                                    let (_, h1055) = v12;
                                                    h1055
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
    let m1060 = v0;
    match m1060.uncons() {
        None => {
            Ok(Ok::<List<(Bytes, (Bytes, Bytes))>, Bytes>(List::<(Bytes, (Bytes, Bytes))>::nil()))
        }
        Some((v3, v4)) => {
            let m1061 = f168(v4, v1.clone(), v2.clone())?;
            match m1061 {
                Ok(v6) => {
                    if if f237(v3.clone(), Str::lit("redefine")) {
                        true
                    } else {
                        if f237(v3.clone(), Str::lit("override")) {
                            true
                        } else {
                            f237(v3.clone(), Str::lit("defaultOpenContent"))
                        }
                    } {
                        Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                            Adt18::C0(h1062, _, _, _, _, _) => {
                                h1062
                            }
                        }, List::cons(f8(Str::lit(": xs:")), List::cons(match v3 {
                            Adt25::C0(_, h1063, _, _, _) => {
                                h1063
                            }
                        }, List::cons(f8(Str::lit(" is not translated, because it changes components that other schema documents define (XSD 1.1 Part 1, sections 4.2.4, 4.2.5 and 3.4.2.3)")), List::<Bytes>::nil())))))))
                    } else {
                        if if f237(v3.clone(), Str::lit("import")) {
                            true
                        } else {
                            f237(v3.clone(), Str::lit("include"))
                        } {
                            let m1064 = f238(v3.clone(), Str::lit("schemaLocation"));
                            match m1064 {
                                None => {
                                    Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                                        Adt18::C0(h1065, _, _, _, _, _) => {
                                            h1065
                                        }
                                    }, List::cons(f8(Str::lit(": an xs:")), List::cons(match v3 {
                                        Adt25::C0(_, h1066, _, _, _) => {
                                            h1066
                                        }
                                    }, List::cons(f8(Str::lit(" with no schemaLocation names no document whose components could be read (XSD 1.1 Part 1, section 4.2.6.2)")), List::<Bytes>::nil())))))))
                                }
                                Some(v7) => {
                                    let m1068 = f239(match v1.clone() {
                                        Adt18::C0(_, h1067, _, _, _, _) => {
                                            h1067
                                        }
                                    }, v7)?;
                                    match m1068 {
                                        Ok(v9) => {
                                            let m1070 = f240(v2, v9.clone());
                                            match m1070 {
                                                None => {
                                                    Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                                                        Adt18::C0(h1071, _, _, _, _, _) => {
                                                            h1071
                                                        }
                                                    }, List::cons(f8(Str::lit(": the schema document ")), List::cons(v9, List::cons(f8(Str::lit(" is not a bound authority, so its components cannot be read; stage its binding first (XSD 1.1 Part 1, section 4.2.6.2)")), List::<Bytes>::nil())))))))
                                                }
                                                Some(v10) => {
                                                    Ok(Ok::<List<(Bytes, (Bytes, Bytes))>, Bytes>(List::cons((v10, (match v3.clone() {
                                                        Adt25::C0(_, h1072, _, _, _) => {
                                                            h1072
                                                        }
                                                    }, if f237(v3.clone(), Str::lit("include")) {
                                                        match v1 {
                                                            Adt18::C0(_, _, h1073, _, _, _) => {
                                                                h1073
                                                            }
                                                        }
                                                    } else {
                                                        f241(v3, Str::lit("namespace"), Str::lit(""))
                                                    })), v6)))
                                                }
                                            }
                                        }
                                        Err(v8) => {
                                            Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f163(match v1 {
                                                Adt18::C0(h1069, _, _, _, _, _) => {
                                                    h1069
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
    let m1074 = v0;
    match m1074.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f169(v3, v1))
        }
    }
}

pub fn f170(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f242(v0, v1)
}

pub fn f171(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f243(v0, v1)
}

pub fn f172(v0: Bytes, v1: u64) -> R<bool> {
    if if if f86(v0.clone(), v1, Str::lit("---")) {
        true
    } else {
        f86(v0.clone(), v1, Str::lit("..."))
    } {
        if f244(f174(v0.clone(), {
            let a1075 = v1;
            let a1076 = 3u64;
            nat_add(a1075, a1076)?
        })?) {
            true
        } else {
            f245(f174(v0.clone(), {
                let a1077 = v1;
                let a1078 = 3u64;
                nat_add(a1077, a1078)?
            })?)
        }
    } else {
        false
    } {
        let a1079 = f246(v0, v1)?;
        let a1080 = 0u64;
        Ok(nat_eq(a1079, a1080))
    } else {
        Ok(false)
    }
}

pub fn f173(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f247(v0.clone(), v1)?;
    let v3: u64 = f174(v0.clone(), v2)?;
    if if f245(v3) {
        true
    } else {
        let a1081 = v3;
        let a1082 = 35u64;
        if nat_eq(a1081, a1082) {
            let a1083 = v1;
            let a1084 = v2;
            nat_lt(a1083, a1084)
        } else {
            false
        }
    } {
        Ok(Ok::<u64, Bytes>(f248(v0, v2)?))
    } else {
        Ok(Err::<u64, Bytes>(f83(Str::lit("only white space and a comment may follow a node on its line (YAML 1.2.2 [77])"), v2)?))
    }
}

pub fn f174(v0: Bytes, v1: u64) -> R<u64> {
    f139(v0, v1)
}

pub fn f175(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: List<Bytes>) -> R<Adt20> {
    let m1085 = v3;
    if m1085 == 0 {
        Ok(Adt20::C1(f83(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v8 = m1085 - 1;
        let a1086 = v4;
        let a1087 = 0u64;
        if nat_eq(a1086, a1087) {
            let v9: u64 = f249(v0.clone(), v1)?;
            let v10: u64 = {
                let a1088 = v9;
                let a1089 = v1;
                nat_sub(a1088, a1089)
            };
            let a1090 = f174(v0.clone(), v9)?;
            let a1091 = 9u64;
            if nat_eq(a1090, a1091) {
                let v11: u64 = f247(v0.clone(), v9)?;
                if if f250(v0.clone(), v11)? {
                    true
                } else {
                    f251(v0.clone(), v11)?
                } {
                    Ok(Adt20::C1(f83(Str::lit("a tab cannot indent a block node (YAML 1.2.2 [63])"), v11)?))
                } else {
                    let m1092 = f252(v0.clone(), v11)?;
                    match m1092 {
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
                    Ok(f254(true, f175(v0, v9, v2, v8, 2u64, false, v10, List::<Bytes>::nil())?))
                } else {
                    let m1093 = f252(v0.clone(), v9)?;
                    match m1093 {
                        None => {
                            if f251(v0.clone(), v9)? {
                                Ok(f254(false, f175(v0, v9, v2, v8, 3u64, false, v10, List::<Bytes>::nil())?))
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
            let a1094 = v4;
            let a1095 = 1u64;
            if nat_eq(a1094, a1095) {
                let v14: u64 = f247(v0.clone(), v1)?;
                let v15: u64 = f174(v0.clone(), v14)?;
                if if f245(v15) {
                    true
                } else {
                    let a1096 = v15;
                    let a1097 = 35u64;
                    if nat_eq(a1096, a1097) {
                        let a1098 = v1;
                        let a1099 = v14;
                        nat_lt(a1098, a1099)
                    } else {
                        false
                    }
                } {
                    let v16: u64 = f102(v0.clone(), f248(v0.clone(), v14)?)?;
                    let a1101 = {
                        let a1100 = v0.clone();
                        length_bytes(a1100)
                    };
                    let a1102 = v16;
                    if if nat_le(a1101, a1102) {
                        true
                    } else {
                        f172(v0.clone(), v16)?
                    } {
                        Ok(Adt20::C0(Adt19::C0, v16))
                    } else {
                        let v17: u64 = {
                            let a1103 = f249(v0.clone(), v16)?;
                            let a1104 = v16;
                            nat_sub(a1103, a1104)
                        };
                        let a1105 = v2;
                        let a1106 = v17;
                        if nat_le(a1105, a1106) {
                            f175(v0, v16, v2, v8, 0u64, false, 0u64, List::<Bytes>::nil())
                        } else {
                            if if if f176(v5) {
                                let a1107 = v17;
                                let a1108 = v6;
                                nat_eq(a1107, a1108)
                            } else {
                                false
                            } {
                                f250(v0.clone(), {
                                    let a1109 = v16;
                                    let a1110 = v17;
                                    nat_add(a1109, a1110)?
                                })?
                            } else {
                                false
                            } {
                                Ok(f254(true, f175(v0, {
                                    let a1111 = v16;
                                    let a1112 = v17;
                                    nat_add(a1111, a1112)?
                                }, v2, v8, 2u64, false, v17, List::<Bytes>::nil())?))
                            } else {
                                Ok(Adt20::C0(Adt19::C0, v16))
                            }
                        }
                    }
                } else {
                    if if v5 {
                        f250(v0.clone(), v14)?
                    } else {
                        false
                    } {
                        let a1115 = f255(f256(v0.clone(), v1, v14), 9u64)?;
                        let a1116 = {
                            let a1113 = v14;
                            let a1114 = v1;
                            nat_sub(a1113, a1114)
                        };
                        if nat_lt(a1115, a1116) {
                            Ok(Adt20::C1(f83(Str::lit("a tab cannot indent a block node (YAML 1.2.2 [63])"), v14)?))
                        } else {
                            Ok(f254(true, f175(v0.clone(), v14, v2, v8, 2u64, false, f246(v0, v14)?, List::<Bytes>::nil())?))
                        }
                    } else {
                        let m1117 = f252(v0.clone(), v14)?;
                        match m1117 {
                            None => {
                                if if v5 {
                                    f251(v0.clone(), v14)?
                                } else {
                                    false
                                } {
                                    Ok(f254(false, f175(v0.clone(), v14, v2, v8, 3u64, false, f246(v0, v14)?, List::<Bytes>::nil())?))
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
                let a1118 = v4;
                let a1119 = 2u64;
                if nat_eq(a1118, a1119) {
                    let m1124 = f175(v0.clone(), {
                        let a1120 = v1;
                        let a1121 = 1u64;
                        nat_add(a1120, a1121)?
                    }, {
                        let a1122 = v6;
                        let a1123 = 1u64;
                        nat_add(a1122, a1123)?
                    }, v8, 1u64, true, v6, List::<Bytes>::nil())?;
                    match m1124 {
                        Adt20::C0(v20, v21) => {
                            let v22: u64 = f102(v0.clone(), v21)?;
                            let v23: u64 = {
                                let a1125 = f249(v0.clone(), v22)?;
                                let a1126 = v22;
                                nat_sub(a1125, a1126)
                            };
                            let a1128 = {
                                let a1127 = v0.clone();
                                length_bytes(a1127)
                            };
                            let a1129 = v22;
                            if nat_le(a1128, a1129) {
                                Ok(Adt20::C0(Adt19::C9(std::rc::Rc::new(v20), std::rc::Rc::new(Adt19::C8)), v22))
                            } else {
                                let a1130 = v23;
                                let a1131 = v6;
                                if if nat_eq(a1130, a1131) {
                                    f250(v0.clone(), {
                                        let a1132 = v22;
                                        let a1133 = v23;
                                        nat_add(a1132, a1133)?
                                    })?
                                } else {
                                    false
                                } {
                                    let m1136 = f175(v0, {
                                        let a1134 = v22;
                                        let a1135 = v23;
                                        nat_add(a1134, a1135)?
                                    }, v2, v8, 2u64, false, v6, List::<Bytes>::nil())?;
                                    match m1136 {
                                        Adt20::C0(v25, v26) => {
                                            Ok(Adt20::C0(Adt19::C9(std::rc::Rc::new(v20), std::rc::Rc::new(v25)), v26))
                                        }
                                        Adt20::C1(v24) => {
                                            Ok(Adt20::C1(v24))
                                        }
                                    }
                                } else {
                                    let a1137 = v6;
                                    let a1138 = v23;
                                    if nat_lt(a1137, a1138) {
                                        Ok(Adt20::C1(f83(Str::lit("a line is indented more than its block sequence entry allows (YAML 1.2.2 [183])"), {
                                            let a1139 = v22;
                                            let a1140 = v23;
                                            nat_add(a1139, a1140)?
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
                    let m1141 = f252(v0.clone(), v1)?;
                    match m1141 {
                        None => {
                            let m1142 = f257(v0.clone(), v1)?;
                            match m1142 {
                                None => {
                                    Ok(Adt20::C1(f83(Str::lit("a block mapping entry must be an implicit key and : (YAML 1.2.2 [188])"), v1)?))
                                }
                                Some(v28) => {
                                    let v29: Bytes = f258(v0.clone(), v1)?;
                                    if f208(v29.clone(), v7.clone()) {
                                        Ok(Adt20::C1(f83(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                    } else {
                                        let m1147 = f175(v0.clone(), {
                                            let a1143 = v28;
                                            let a1144 = 1u64;
                                            nat_add(a1143, a1144)?
                                        }, {
                                            let a1145 = v6;
                                            let a1146 = 1u64;
                                            nat_add(a1145, a1146)?
                                        }, v8, 1u64, false, v6, List::<Bytes>::nil())?;
                                        match m1147 {
                                            Adt20::C0(v31, v32) => {
                                                let v33: u64 = f102(v0.clone(), v32)?;
                                                let v34: u64 = {
                                                    let a1148 = f249(v0.clone(), v33)?;
                                                    let a1149 = v33;
                                                    nat_sub(a1148, a1149)
                                                };
                                                let a1151 = {
                                                    let a1150 = v0.clone();
                                                    length_bytes(a1150)
                                                };
                                                let a1152 = v33;
                                                if nat_le(a1151, a1152) {
                                                    Ok(Adt20::C0(Adt19::C10(v29, std::rc::Rc::new(v31), std::rc::Rc::new(Adt19::C8)), v33))
                                                } else {
                                                    let a1153 = v34;
                                                    let a1154 = v6;
                                                    if if if nat_eq(a1153, a1154) {
                                                        f176(f250(v0.clone(), {
                                                            let a1155 = v33;
                                                            let a1156 = v34;
                                                            nat_add(a1155, a1156)?
                                                        })?)
                                                    } else {
                                                        false
                                                    } {
                                                        f176(f172(v0.clone(), v33)?)
                                                    } else {
                                                        false
                                                    } {
                                                        let m1159 = f175(v0, {
                                                            let a1157 = v33;
                                                            let a1158 = v34;
                                                            nat_add(a1157, a1158)?
                                                        }, v2, v8, 3u64, false, v6, List::cons(v29.clone(), v7))?;
                                                        match m1159 {
                                                            Adt20::C0(v36, v37) => {
                                                                Ok(Adt20::C0(Adt19::C10(v29, std::rc::Rc::new(v31), std::rc::Rc::new(v36)), v37))
                                                            }
                                                            Adt20::C1(v35) => {
                                                                Ok(Adt20::C1(v35))
                                                            }
                                                        }
                                                    } else {
                                                        let a1160 = v6;
                                                        let a1161 = v34;
                                                        if nat_lt(a1160, a1161) {
                                                            Ok(Adt20::C1(f83(Str::lit("a line is indented more than its block mapping entry allows (YAML 1.2.2 [187])"), {
                                                                let a1162 = v33;
                                                                let a1163 = v34;
                                                                nat_add(a1162, a1163)?
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

pub fn f176(v0: bool) -> bool {
    !v0
}

pub fn f177(v0: Adt19) -> R<Bytes> {
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
            f259(v4, v5)
        }
        Adt19::C4(v6, v7, v8) => {
            f260(v6, v7, v8)
        }
        Adt19::C5(v9) => {
            f261(v9)
        }
        Adt19::C6(r1165) => {
            let v10 = (*r1165).clone();
            Ok(f262(f263(v10)?))
        }
        Adt19::C7(r1166) => {
            let v11 = (*r1166).clone();
            f264(f265(v11)?)
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

pub fn f178(p1221: u64, v1: Bytes, p1222: u64, p1223: List<Adt21>) -> R<Result<List<Adt21>, Bytes>> {
    let m1220 = (p1221, (p1222, p1223));
    {
        let mut m1220 = m1220;
        loop {
            let step = {
                let (v0, (v2, v3)) = m1220;
                let m1167 = v0;
                if m1167 == 0 {
                    Err::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>(Ok(Err::<List<Adt21>, Bytes>(f266(Str::lit("the input has more tokens than bytes"), v2)?)))
                } else {
                    let v4 = m1167 - 1;
                    let v5: u64 = f87(v1.clone(), v2)?;
                    let a1168 = v5;
                    let a1169 = 256u64;
                    if nat_eq(a1168, a1169) {
                        Err::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>(Ok(Ok::<List<Adt21>, Bytes>(f267(v3, List::<Adt21>::nil()))))
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
                                        Err::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>(Ok(Ok::<List<Adt21>, Bytes>(f267(v3, List::<Adt21>::nil()))))
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
                                            Err::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>(Ok(Err::<List<Adt21>, Bytes>(f266(Str::lit("a comment is not closed (Web IDL, IDL grammar: comment)"), v2)?)))
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
                                                Err::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>(Ok(Err::<List<Adt21>, Bytes>(f266(Str::lit("a string is not closed (Web IDL, IDL grammar: string)"), v2)?)))
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
                                        if f268(f87(v1.clone(), v9)?) {
                                            let v10: u64 = f269(v1.clone(), 3u64, {
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
                                            let v12: u64 = f270(v1.clone(), v11)?;
                                            let v13: u64 = f271(v1.clone(), v11)?;
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

pub fn f179(p1226: List<Adt12>, p1227: List<Adt12>) -> List<Adt12> {
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

pub fn f180(v0: List<Adt21>) -> R<Result<List<Adt21>, Bytes>> {
    if f272(v0.clone(), Str::lit("[")) {
        Ok(f273({
            let a1229 = {
                let a1228 = v0.clone();
                length_list(a1228)
            };
            let a1230 = 1u64;
            nat_add(a1229, a1230)?
        }, f274(v0), List::cons(f8(Str::lit("]")), List::<Bytes>::nil()), false))
    } else {
        Ok(Ok::<List<Adt21>, Bytes>(v0))
    }
}

pub fn f181(v0: u64, v1: List<Adt21>) -> R<Result<(Adt12, List<Adt21>), Bytes>> {
    if f272(v1.clone(), Str::lit("interface")) {
        if f272(f274(v1.clone()), Str::lit("mixin")) {
            f275(v0, 1u64, f274(f274(v1)), false)
        } else {
            f275(v0, 0u64, f274(v1), true)
        }
    } else {
        if f272(v1.clone(), Str::lit("partial")) {
            let v2: List<Adt21> = f274(v1);
            if f272(v2.clone(), Str::lit("interface")) {
                if f272(f274(v2.clone()), Str::lit("mixin")) {
                    f275(v0, 6u64, f274(f274(v2)), false)
                } else {
                    f275(v0, 5u64, f274(v2), false)
                }
            } else {
                if f272(v2.clone(), Str::lit("dictionary")) {
                    f275(v0, 7u64, f274(v2), false)
                } else {
                    if f272(v2.clone(), Str::lit("namespace")) {
                        f276(f274(v2), f8(Str::lit("a namespace holds operations and attributes of no object, so it is behavior, not data (Web IDL 2.6)")))
                    } else {
                        Ok(Err::<(Adt12, List<Adt21>), Bytes>(f277(Str::lit("interface, dictionary or namespace"), v2)))
                    }
                }
            }
        } else {
            if f272(v1.clone(), Str::lit("dictionary")) {
                f275(v0, 2u64, f274(v1), true)
            } else {
                if f272(v1.clone(), Str::lit("enum")) {
                    let m1231 = f278(f274(v1));
                    match m1231 {
                        Ok(v4) => {
                            let m1232 = f279(f280(v4.clone()), Str::lit("{"));
                            match m1232 {
                                Ok(v6) => {
                                    let m1233 = f281(v0, v6, List::<Bytes>::nil());
                                    match m1233 {
                                        Ok(v8) => {
                                            Ok(Ok::<(Adt12, List<Adt21>), Bytes>((Adt12::C0(3u64, f282(v4), Bytes::lit(&[]), List::<Adt13>::nil(), {
                                                let (h1234, _) = v8;
                                                h1234
                                            }, f283(Bytes::lit(&[]))), {
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
                    if f272(v1.clone(), Str::lit("typedef")) {
                        let m1236 = f284(v0, f274(v1), 0u64)?;
                        match m1236 {
                            Ok(v10) => {
                                let m1237 = f278(f285(v10.clone()));
                                match m1237 {
                                    Ok(v12) => {
                                        let m1238 = f279(f280(v12.clone()), Str::lit(";"));
                                        match m1238 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt12, List<Adt21>), Bytes>((Adt12::C0(4u64, f282(v12), Bytes::lit(&[]), List::<Adt13>::nil(), List::<Bytes>::nil(), f286(v10)), v14)))
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
                        if f272(v1.clone(), Str::lit("callback")) {
                            if f272(f274(v1.clone()), Str::lit("interface")) {
                                f276(f274(f274(v1)), f8(Str::lit("a callback interface is implemented by user code, so it is behavior, not data (Web IDL 2.4)")))
                            } else {
                                f276(f274(v1), f8(Str::lit("a callback function is behavior, not data (Web IDL 2.10)")))
                            }
                        } else {
                            if f272(v1.clone(), Str::lit("namespace")) {
                                f276(f274(v1), f8(Str::lit("a namespace holds operations and attributes of no object, so it is behavior, not data (Web IDL 2.6)")))
                            } else {
                                let m1239 = f278(v1);
                                match m1239 {
                                    Ok(v16) => {
                                        let m1240 = f279(f280(v16.clone()), Str::lit("includes"));
                                        match m1240 {
                                            Ok(v18) => {
                                                let m1241 = f278(v18);
                                                match m1241 {
                                                    Ok(v20) => {
                                                        let m1242 = f279(f280(v20.clone()), Str::lit(";"));
                                                        match m1242 {
                                                            Ok(v22) => {
                                                                Ok(Ok::<(Adt12, List<Adt21>), Bytes>((Adt12::C0(8u64, f282(v16), f282(v20), List::<Adt13>::nil(), List::<Bytes>::nil(), f283(Bytes::lit(&[]))), v22)))
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

pub fn f182(p1246: List<Bytes>, v1: List<Adt15>) -> Bytes {
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

pub fn f183(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f154(v0)?;
    if f115(v1.clone(), f287()) {
        f288(v1)
    } else {
        Ok(v1)
    }
}

pub fn f184(p1252: List<Adt12>, v1: List<Adt12>, v2: Bytes) -> Option<Bytes> {
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

pub fn f185(v0: u64, v1: List<Adt12>, v2: Adt12) -> R<Result<List<Adt13>, Bytes>> {
    let m1253 = v0;
    if m1253 == 0 {
        Ok(Err::<List<Adt13>, Bytes>(f7(List::cons(f8(Str::lit("the inheritance of ")), List::cons(f114(v2), List::cons(f8(Str::lit(" is cyclic (Web IDL 2.2 and 2.7: an interface or dictionary must not inherit from itself)")), List::<Bytes>::nil()))))))
    } else {
        let v3 = m1253 - 1;
        let v4: List<Adt13> = {
            let a1254 = f112(v2.clone());
            let a1255 = 2u64;
            if nat_eq(a1254, a1255) {
                f289(f290(f291(v2.clone()), f292(v1.clone(), 7u64, f114(v2.clone()))))?
            } else {
                f290(f291(v2.clone()), f290(f292(v1.clone(), 5u64, f114(v2.clone())), f293(v1.clone(), v1.clone(), f114(v2.clone()))))
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
                    let m1259 = f185(v3, v1, v5)?;
                    match m1259 {
                        Ok(v7) => {
                            Ok(Ok::<List<Adt13>, Bytes>(f290(v7, v4)))
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

pub fn f186(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: mk, fields: []}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))
}

pub fn f187(v0: bool, v1: List<Adt13>) -> R<Result<(List<Bytes>, List<Bytes>), Bytes>> {
    let m1260 = v1;
    match m1260.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::<Bytes>::nil(), List::<Bytes>::nil())))
        }
        Some((v2, v3)) => {
            let v4: Adt14 = f294(v2.clone());
            if !f35(f196(v4.clone()), Bytes::lit(&[])) {
                Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons(f295(v2), List::cons(f8(Str::lit(": ")), List::cons(f196(v4), List::<Bytes>::nil())))))))
            } else {
                let m1261 = f296(v0, f295(v2.clone()))?;
                match m1261 {
                    None => {
                        Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons(f295(v2), List::cons(f8(Str::lit(" has no field name")), List::<Bytes>::nil()))))))
                    }
                    Some(v5) => {
                        if f115(v5.clone(), f297(v0, f189(v3.clone()))?) {
                            Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("two of its members derive the field name ")), List::cons(v5, List::cons(f8(Str::lit(" (Web IDL 2.2 and 2.7: the members of one definition have distinct identifiers)")), List::<Bytes>::nil()))))))
                        } else {
                            let m1262 = f187(v0, v3)?;
                            match m1262 {
                                Ok(v7) => {
                                    Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v5, List::cons(f8(Str::lit(", type: ")), List::cons({
                                        let a1263 = f298(v2);
                                        let a1264 = 1u64;
                                        if nat_eq(a1263, a1264) {
                                            f7(List::cons(f8(Str::lit("(Option ")), List::cons(f198(v4.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                        } else {
                                            f198(v4.clone())
                                        }
                                    }, List::cons(f8(Str::lit("}")), List::<Bytes>::nil())))))), {
                                        let (h1265, _) = v7;
                                        h1265
                                    }), f118(f199(v4), {
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

pub fn f188(p1270: List<Bytes>) -> R<bool> {
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
                        let m1268 = f299(v1)?;
                        match m1268 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f115(v3, f297(false, v2.clone())?) {
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

pub fn f189(v0: List<Adt13>) -> List<Bytes> {
    let m1271 = v0;
    match m1271.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f295(v1), f189(v2))
        }
    }
}

pub fn f190(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}")), List::cons(f75(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f191(v0: Adt12) -> List<Bytes> {
    let m1272 = v0;
    match m1272 {
        Adt12::C0(_, _, _, _, v5, _) => {
            v5
        }
    }
}

pub fn f192(v0: bool, v1: List<Adt19>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1273 = v1;
    match m1273.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m1274 = f300(v0, v3.clone())?;
            match m1274 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f105(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f115(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m1275 = f192(v0, v4, List::cons(v5.clone(), v2))?;
                        match m1275 {
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

pub fn f193(v0: List<Bytes>) -> List<Adt19> {
    let m1276 = v0;
    match m1276.uncons() {
        None => {
            List::<Adt19>::nil()
        }
        Some((v1, v2)) => {
            List::cons(Adt19::C5(v1), f193(v2))
        }
    }
}

pub fn f194(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f75(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f195(v0: Adt12) -> Adt14 {
    let m1277 = v0;
    match m1277 {
        Adt12::C0(_, _, _, _, _, v6) => {
            v6
        }
    }
}

pub fn f196(v0: Adt14) -> Bytes {
    let m1278 = v0;
    match m1278 {
        Adt14::C0(_, _, _, v4, _) => {
            v4
        }
    }
}

pub fn f197(v0: Adt14) -> u64 {
    let m1279 = v0;
    match m1279 {
        Adt14::C0(v1, _, _, _, _) => {
            v1
        }
    }
}

pub fn f198(v0: Adt14) -> Bytes {
    let m1280 = v0;
    match m1280 {
        Adt14::C0(_, v2, _, _, _) => {
            v2
        }
    }
}

pub fn f199(v0: Adt14) -> List<Bytes> {
    let m1281 = v0;
    match m1281 {
        Adt14::C0(_, _, v3, _, _) => {
            v3
        }
    }
}

pub fn f200(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f201(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h1282, _) = v2;
        h1282
    };
    let a1283 = v3;
    let a1284 = 0u64;
    if nat_eq(a1283, a1284) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a1285 = v3;
            let a1286 = v1;
            let a1287 = 0u64;
            nat_quot(a1285, a1286, a1287)
        }, {
            let a1292 = f126(v0, {
                let a1288 = v3;
                let a1289 = v1;
                let a1290 = 0u64;
                nat_rem(a1288, a1289, a1290)
            });
            let a1293 = {
                let (_, h1291) = v2;
                h1291
            };
            append_bytes(a1292, a1293)
        }))
    }
}

pub fn f202(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h1294, _) = v1;
        h1294
    };
    let a1296 = {
        let a1295 = v0.clone();
        length_bytes(a1295)
    };
    let a1297 = v2;
    if nat_le(a1296, a1297) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let a1298 = f87(v0.clone(), v2)?;
        let a1299 = 13u64;
        if nat_eq(a1298, a1299) {
            Ok(Some(({
                let a1302 = f87(v0, {
                    let a1300 = v2;
                    let a1301 = 1u64;
                    nat_add(a1300, a1301)?
                })?;
                let a1303 = 10u64;
                if nat_eq(a1302, a1303) {
                    let a1304 = v2;
                    let a1305 = 2u64;
                    nat_add(a1304, a1305)?
                } else {
                    let a1306 = v2;
                    let a1307 = 1u64;
                    nat_add(a1306, a1307)?
                }
            }, {
                let a1309 = {
                    let (_, h1308) = v1;
                    h1308
                };
                let a1310 = f8(Str::lit("\u{a}"));
                append_bytes(a1309, a1310)
            })))
        } else {
            let v4: u64 = {
                let m1311 = f133(v0.clone(), f8(Str::lit("\u{d}")), v2)?;
                match m1311 {
                    None => {
                        let a1312 = v0.clone();
                        length_bytes(a1312)
                    }
                    Some(v3) => {
                        v3
                    }
                }
            };
            Ok(Some((v4, {
                let a1316 = {
                    let (_, h1313) = v1;
                    h1313
                };
                let a1317 = f32(v0, v2, {
                    let a1314 = v4;
                    let a1315 = v2;
                    nat_sub(a1314, a1315)
                });
                append_bytes(a1316, a1317)
            })))
        }
    }
}

pub fn f203(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1318 = f301(v0, v1)?;
    match m1318 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f302({
                let (h1319, _) = v2;
                h1319
            })? {
                Ok(Some({
                    let a1321 = v1;
                    let a1322 = {
                        let (_, h1320) = v2;
                        h1320
                    };
                    nat_add(a1321, a1322)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f204(v0: u64, v1: Bytes) -> R<bool> {
    f303(v0, v1.clone(), 0u64, {
        let a1324 = {
            let a1323 = v1;
            length_bytes(a1323)
        };
        let a1325 = 3u64;
        let a1326 = 0u64;
        nat_quot(a1324, a1325, a1326)
    }, true)
}

pub fn f205() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{0}\u{0} "))
}

pub fn f206(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f304(v1, v0, v2)
}

pub fn f207(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f305(v0, v1)
}

pub fn f208(v0: Bytes, p1329: List<Bytes>) -> bool {
    let m1328 = p1329;
    {
        let mut m1328 = m1328;
        loop {
            let step = {
                let v1 = m1328;
                let m1327 = v1;
                match m1327.uncons() {
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
                Ok(next) => { m1328 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f209(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f306(v0, v1, v2)
}

pub fn f210(v0: Bytes) -> Result<(Bytes, u64), Bytes> {
    Err::<(Bytes, u64), Bytes>(v0)
}

pub fn f211(v0: Bytes, v1: u64) -> R<Result<(Bytes, u64), Bytes>> {
    if f86(v0.clone(), v1, Str::lit("&#")) {
        let v2: bool = {
            let a1332 = f87(v0.clone(), {
                let a1330 = v1;
                let a1331 = 2u64;
                nat_add(a1330, a1331)?
            })?;
            let a1333 = 120u64;
            nat_eq(a1332, a1333)
        };
        let v3: u64 = if v2 {
            let a1334 = v1;
            let a1335 = 3u64;
            nat_add(a1334, a1335)?
        } else {
            let a1336 = v1;
            let a1337 = 2u64;
            nat_add(a1336, a1337)?
        };
        let v4: (u64, u64) = f307(v0.clone(), v2, v3)?;
        let a1339 = {
            let (h1338, _) = v4;
            h1338
        };
        let a1340 = v3;
        if if nat_eq(a1339, a1340) {
            true
        } else {
            let a1342 = f87(v0, {
                let (h1341, _) = v4;
                h1341
            })?;
            let a1343 = 59u64;
            !nat_eq(a1342, a1343)
        } {
            Ok(f210(f83(Str::lit("a character reference must be &#digits; or &#xhex; (XML 1.0 [66])"), v1)?))
        } else {
            if f302({
                let (_, h1344) = v4;
                h1344
            })? {
                Ok(Ok::<(Bytes, u64), Bytes>((f308({
                    let (_, h1345) = v4;
                    h1345
                })?, {
                    let a1347 = {
                        let (h1346, _) = v4;
                        h1346
                    };
                    let a1348 = 1u64;
                    nat_add(a1347, a1348)?
                })))
            } else {
                Ok(f210(f83(Str::lit("a character reference must name a Char (XML 1.0 [66] WFC: Legal Character)"), v1)?))
            }
        }
    } else {
        if f86(v0.clone(), v1, Str::lit("&lt;")) {
            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("<")), {
                let a1349 = v1;
                let a1350 = 4u64;
                nat_add(a1349, a1350)?
            })))
        } else {
            if f86(v0.clone(), v1, Str::lit("&gt;")) {
                Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit(">")), {
                    let a1351 = v1;
                    let a1352 = 4u64;
                    nat_add(a1351, a1352)?
                })))
            } else {
                if f86(v0.clone(), v1, Str::lit("&amp;")) {
                    Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("&")), {
                        let a1353 = v1;
                        let a1354 = 5u64;
                        nat_add(a1353, a1354)?
                    })))
                } else {
                    if f86(v0.clone(), v1, Str::lit("&apos;")) {
                        Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("'")), {
                            let a1355 = v1;
                            let a1356 = 6u64;
                            nat_add(a1355, a1356)?
                        })))
                    } else {
                        if f86(v0, v1, Str::lit("&quot;")) {
                            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("\"")), {
                                let a1357 = v1;
                                let a1358 = 6u64;
                                nat_add(a1357, a1358)?
                            })))
                        } else {
                            Ok(f210(f83(Str::lit("an entity reference names an entity that is not declared, and only the five predefined entities are (XML 1.0 section 4.1, WFC: Entity Declared)"), v1)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f212(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<u64> {
    let (h1359, _) = f130(Fn4::F309(v1, v2, v0.clone()), {
        let a1361 = {
            let a1360 = v0;
            length_bytes(a1360)
        };
        let a1362 = 1u64;
        nat_add(a1361, a1362)?
    }, v3)?;
    Ok(h1359)
}

pub fn f213(p1375: Adt16, v1: List<(Bytes, Bytes)>) -> R<Result<Option<Adt27>, Bytes>> {
    let m1374 = p1375;
    {
        let mut m1374 = m1374;
        loop {
            let step = {
                let v0 = m1374;
                let m1363 = v0;
                match m1363 {
                    Adt16::C0(_, _, _) => {
                        Err::<Adt16, R<Result<Option<Adt27>, Bytes>>>(Ok(Ok::<Option<Adt27>, Bytes>(None::<Adt27>)))
                    }
                    Adt16::C1(_, _, _) => {
                        Err::<Adt16, R<Result<Option<Adt27>, Bytes>>>(Ok(Ok::<Option<Adt27>, Bytes>(None::<Adt27>)))
                    }
                    Adt16::C2(_) => {
                        Err::<Adt16, R<Result<Option<Adt27>, Bytes>>>(Ok(Ok::<Option<Adt27>, Bytes>(None::<Adt27>)))
                    }
                    Adt16::C3(r1372, r1373) => {
                        let v2 = (*r1372).clone();
                        let v3 = (*r1373).clone();
                        let m1364 = v2;
                        match m1364 {
                            Adt16::C0(v4, r1370, r1371) => {
                                let v5 = (*r1370).clone();
                                let v6 = (*r1371).clone();
                                let m1365 = f90(v5.clone(), v1.clone());
                                match m1365 {
                                    Ok(v8) => {
                                        let m1366 = f91(v8.clone(), v4)?;
                                        match m1366 {
                                            Ok(v10) => {
                                                if f208({
                                                    let (_, h1367) = v10.clone();
                                                    h1367
                                                }, List::cons(f8(Str::lit("restriction")), List::cons(f8(Str::lit("list")), List::cons(f8(Str::lit("union")), List::<Bytes>::nil())))) {
                                                    if f35({
                                                        let (h1368, _) = v10;
                                                        h1368
                                                    }, f151()) {
                                                        Err::<Adt16, R<Result<Option<Adt27>, Bytes>>>(Ok(Ok::<Option<Adt27>, Bytes>(Some(Adt27::C0({
                                                            let (_, h1369) = v10;
                                                            h1369
                                                        }, v5, v6, v8)))))
                                                    } else {
                                                        Ok::<Adt16, R<Result<Option<Adt27>, Bytes>>>(v3)
                                                    }
                                                } else {
                                                    Ok::<Adt16, R<Result<Option<Adt27>, Bytes>>>(v3)
                                                }
                                            }
                                            Err(v9) => {
                                                Err::<Adt16, R<Result<Option<Adt27>, Bytes>>>(Ok(f310(v9)))
                                            }
                                        }
                                    }
                                    Err(v7) => {
                                        Err::<Adt16, R<Result<Option<Adt27>, Bytes>>>(Ok(f310(v7)))
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
                Ok(next) => { m1374 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f214(v0: Bytes) -> Result<Adt22, Bytes> {
    Err::<Adt22, Bytes>(v0)
}

pub fn f215(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit(" at simpleType ")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f216(v0: List<(Bytes, Bytes)>, v1: Bytes, v2: Bytes) -> R<Result<Adt22, Bytes>> {
    let m1376 = f91(v0, v2.clone())?;
    match m1376 {
        Ok(v4) => {
            if f35({
                let (h1377, _) = v4.clone();
                h1377
            }, f151()) {
                let m1379 = f311({
                    let (_, h1378) = v4;
                    h1378
                }, 16u64)?;
                match m1379 {
                    Ok(v6) => {
                        Ok(Ok::<Adt22, Bytes>(Adt22::C0(v6, List::<Bytes>::nil())))
                    }
                    Err(v5) => {
                        Ok(f214(v5))
                    }
                }
            } else {
                if f35({
                    let (h1380, _) = v4.clone();
                    h1380
                }, v1) {
                    let v7: Bytes = f154({
                        let (_, h1381) = v4;
                        h1381
                    })?;
                    Ok(Ok::<Adt22, Bytes>(Adt22::C0(v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                } else {
                    Ok(f214(f7(List::cons(f8(Str::lit("the type ")), List::cons(v2, List::cons(f8(Str::lit(" is in the namespace ")), List::cons({
                        let (h1382, _) = v4;
                        h1382
                    }, List::cons(f8(Str::lit(", which is neither XML Schema's nor this document's target namespace; imported schema documents are not read yet")), List::<Bytes>::nil()))))))))
                }
            }
        }
        Err(v3) => {
            Ok(f214(v3))
        }
    }
}

pub fn f217(v0: Adt16, v1: List<(Bytes, Bytes)>) -> R<Result<List<Adt19>, Bytes>> {
    let m1383 = v0;
    match m1383 {
        Adt16::C0(_, _, _) => {
            Ok(Ok::<List<Adt19>, Bytes>(List::<Adt19>::nil()))
        }
        Adt16::C1(_, _, _) => {
            Ok(Ok::<List<Adt19>, Bytes>(List::<Adt19>::nil()))
        }
        Adt16::C2(_) => {
            Ok(Ok::<List<Adt19>, Bytes>(List::<Adt19>::nil()))
        }
        Adt16::C3(r1390, r1391) => {
            let v2 = (*r1390).clone();
            let v3 = (*r1391).clone();
            let m1384 = f217(v3, v1.clone())?;
            match m1384 {
                Ok(v5) => {
                    let m1385 = v2;
                    match m1385 {
                        Adt16::C0(v6, r1389, _) => {
                            let v7 = (*r1389).clone();
                            let m1386 = f90(v7.clone(), v1);
                            match m1386 {
                                Ok(v10) => {
                                    let m1387 = f91(v10, v6)?;
                                    match m1387 {
                                        Ok(v12) => {
                                            if f92(v12, Str::lit("enumeration")) {
                                                let m1388 = f93(v7, f8(Str::lit("value")));
                                                match m1388 {
                                                    None => {
                                                        Ok(f312(f8(Str::lit("an enumeration facet must have a value attribute (XSD 1.1 Part 2, section 4.3.5.2)"))))
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
                                            Ok(f312(v11))
                                        }
                                    }
                                }
                                Err(v9) => {
                                    Ok(f312(v9))
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
                    Ok(f312(v4))
                }
            }
        }
        Adt16::C4 => {
            Ok(Ok::<List<Adt19>, Bytes>(List::<Adt19>::nil()))
        }
    }
}

pub fn f218(v0: List<Adt19>, v1: List<Bytes>) -> R<List<Adt19>> {
    let m1392 = v0;
    match m1392.uncons() {
        None => {
            Ok(List::<Adt19>::nil())
        }
        Some((v2, v3)) => {
            let v4: Bytes = f105(v2.clone())?;
            if f115(v4.clone(), v1.clone()) {
                f218(v3, v1)
            } else {
                Ok(List::cons(v2, f218(v3, List::cons(v4, v1))?))
            }
        }
    }
}

pub fn f219(v0: List<Adt19>) -> List<Bytes> {
    let m1393 = v0;
    match m1393.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1394 = f313(Some(v1));
            match m1394 {
                None => {
                    f219(v2)
                }
                Some(v3) => {
                    List::cons(v3, f219(v2))
                }
            }
        }
    }
}

pub fn f220(v0: Bytes, p1415: u64, p1416: u64, p1417: bool, p1418: Bytes) -> R<Bytes> {
    let m1414 = (p1415, (p1416, (p1417, p1418)));
    {
        let mut m1414 = m1414;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1414;
                let m1395 = v2;
                if m1395 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1395 - 1;
                    let m1398 = {
                        let a1396 = v0.clone();
                        let a1397 = v1;
                        index_bytes(a1396, a1397)
                    };
                    match m1398 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f42(v6)?;
                            if f314(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1399 = v1;
                                    let a1400 = 1u64;
                                    nat_add(a1399, a1400)?
                                }, (v5, (true, v4))))
                            } else {
                                if f315(f316(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1401 = v1;
                                        let a1402 = 1u64;
                                        nat_add(a1401, a1402)?
                                    }, (v5, (false, {
                                        let a1406 = v4;
                                        let a1407 = f32({
                                            let a1403 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1403)
                                        }, {
                                            let a1404 = v7;
                                            let a1405 = 97u64;
                                            nat_sub(a1404, a1405)
                                        }, 1u64);
                                        append_bytes(a1406, a1407)
                                    }))))
                                } else {
                                    if f317(f316(v7), f317(f318(v7), f319(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1408 = v1;
                                            let a1409 = 1u64;
                                            nat_add(a1408, a1409)?
                                        }, (v5, (false, {
                                            let a1410 = v4;
                                            let a1411 = f32(v0.clone(), v1, 1u64);
                                            append_bytes(a1410, a1411)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1412 = v1;
                                            let a1413 = 1u64;
                                            nat_add(a1412, a1413)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1414 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f221(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f147(v0.clone(), v1) {
        let m1419 = f30(v0, 34u8)?;
        match m1419.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m1420 = v3;
                match m1420.uncons() {
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

pub fn f222(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f320(v0, v1)
}

pub fn f223(v0: List<Adt18>) -> R<Result<List<Adt24>, Bytes>> {
    let m1421 = v0;
    match m1421.uncons() {
        None => {
            Ok(Ok::<List<Adt24>, Bytes>(List::<Adt24>::nil()))
        }
        Some((v1, v2)) => {
            let m1424 = f167(match v1.clone() {
                Adt18::C0(_, _, _, _, h1422, _) => {
                    h1422
                }
            }, match v1.clone() {
                Adt18::C0(_, _, _, h1423, _, _) => {
                    h1423
                }
            })?;
            match m1424 {
                Ok(v4) => {
                    let m1425 = f223(v2)?;
                    match m1425 {
                        Ok(v6) => {
                            Ok(Ok::<List<Adt24>, Bytes>(f321(f322(v4, match v1 {
                                Adt18::C0(_, _, h1426, _, _, _) => {
                                    h1426
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

pub fn f224(v0: List<Adt18>) -> List<Bytes> {
    let m1427 = v0;
    match m1427.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt18::C0(_, _, h1428, _, _, _) => {
                    h1428
                }
            }, f224(v2))
        }
    }
}

pub fn f225(v0: u64, v1: Adt23, v2: List<Adt24>) -> R<Result<List<Adt26>, Bytes>> {
    let m1429 = v2;
    match m1429.uncons() {
        None => {
            Ok(Ok::<List<Adt26>, Bytes>(List::<Adt26>::nil()))
        }
        Some((v3, v4)) => {
            let m1430 = f323(v0, v1.clone(), v3)?;
            match m1430 {
                Ok(v6) => {
                    let m1431 = f225(v0, v1, v4)?;
                    match m1431 {
                        Ok(v8) => {
                            Ok(Ok::<List<Adt26>, Bytes>(f324(v6, v8)))
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

pub fn f226(v0: List<Adt18>) -> R<u64> {
    let m1432 = v0;
    match m1432.uncons() {
        None => {
            Ok(64u64)
        }
        Some((v1, v2)) => {
            let a1434 = match v1 {
                Adt18::C0(_, _, _, _, _, h1433) => {
                    h1433
                }
            };
            let a1435 = f226(v2)?;
            nat_add(a1434, a1435)
        }
    }
}

pub fn f227(p1443: List<Adt26>, p1444: List<Adt26>) -> Result<List<Adt26>, Bytes> {
    let m1442 = (p1443, p1444);
    {
        let mut m1442 = m1442;
        loop {
            let step = {
                let (v0, v1) = m1442;
                let m1436 = v0;
                match m1436.uncons() {
                    None => {
                        Err::<(List<Adt26>, List<Adt26>), Result<List<Adt26>, Bytes>>(Ok::<List<Adt26>, Bytes>(f325(v1, List::<Adt26>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m1438 = f326(v1.clone(), match v2.clone() {
                            Adt26::C0(h1437, _, _, _) => {
                                h1437
                            }
                        });
                        match m1438 {
                            None => {
                                Ok::<(List<Adt26>, List<Adt26>), Result<List<Adt26>, Bytes>>((v3, List::cons(v2, v1)))
                            }
                            Some(v4) => {
                                if f35(match v4 {
                                    Adt26::C0(_, _, h1439, _) => {
                                        h1439
                                    }
                                }, match v2.clone() {
                                    Adt26::C0(_, _, h1440, _) => {
                                        h1440
                                    }
                                }) {
                                    Ok::<(List<Adt26>, List<Adt26>), Result<List<Adt26>, Bytes>>((v3, v1))
                                } else {
                                    Err::<(List<Adt26>, List<Adt26>), Result<List<Adt26>, Bytes>>(Err::<List<Adt26>, Bytes>(f163(f8(Str::lit("two components derive the type name ")), match v2 {
                                        Adt26::C0(h1441, _, _, _) => {
                                            h1441
                                        }
                                    }, f8(Str::lit(", so their LexLean declarations would collide")))))
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

pub fn f228(v0: List<Adt26>) -> List<Adt26> {
    if if f115(f327(), f328(v0.clone())) {
        let m1445 = f326(v0.clone(), f327());
        match m1445 {
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
        List::cons(Adt26::C0(f327(), List::<Bytes>::nil(), f194(f327(), f329()), f329()), v0)
    } else {
        v0
    }
}

pub fn f229(v0: List<Adt26>) -> List<Bytes> {
    let m1446 = v0;
    match m1446.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt26::C0(h1447, _, _, _) => {
                    h1447
                }
            }, f229(v2))
        }
    }
}

pub fn f230(p1453: List<Adt26>, v1: List<Bytes>) -> Option<(Bytes, Bytes)> {
    let m1452 = p1453;
    {
        let mut m1452 = m1452;
        loop {
            let step = {
                let v0 = m1452;
                let m1448 = v0;
                match m1448.uncons() {
                    None => {
                        Err::<List<Adt26>, Option<(Bytes, Bytes)>>(None::<(Bytes, Bytes)>)
                    }
                    Some((v2, v3)) => {
                        let m1450 = f155(match v2.clone() {
                            Adt26::C0(_, h1449, _, _) => {
                                h1449
                            }
                        }, v1.clone());
                        match m1450 {
                            None => {
                                Ok::<List<Adt26>, Option<(Bytes, Bytes)>>(v3)
                            }
                            Some(v4) => {
                                Err::<List<Adt26>, Option<(Bytes, Bytes)>>(Some((match v2 {
                                    Adt26::C0(h1451, _, _, _) => {
                                        h1451
                                    }
                                }, v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1452 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f231(v0: List<Adt26>, v1: List<Bytes>, v2: u64) -> List<(Bytes, List<Bytes>)> {
    let m1454 = v1;
    match m1454.uncons() {
        None => {
            List::<(Bytes, List<Bytes>)>::nil()
        }
        Some((v3, v4)) => {
            List::cons((v3.clone(), f330(v2, v0.clone(), f331(v0.clone(), v3), List::<Bytes>::nil())), f231(v0, v4, v2))
        }
    }
}

pub fn f232(v0: List<Adt26>) -> R<u64> {
    let m1455 = v0;
    match m1455.uncons() {
        None => {
            Ok(1u64)
        }
        Some((v1, v2)) => {
            let a1460 = {
                let a1458 = {
                    let a1457 = match v1 {
                        Adt26::C0(_, h1456, _, _) => {
                            h1456
                        }
                    };
                    length_list(a1457)
                };
                let a1459 = 1u64;
                nat_add(a1458, a1459)?
            };
            let a1461 = f232(v2)?;
            nat_add(a1460, a1461)
        }
    }
}

pub fn f233(p1471: u64, v1: List<Adt26>, p1472: List<(List<Bytes>, List<Bytes>)>, p1473: List<Bytes>, p1474: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m1470 = (p1471, (p1472, (p1473, p1474)));
    {
        let mut m1470 = m1470;
        loop {
            let step = {
                let (v0, (v2, (v3, v4))) = m1470;
                let m1462 = v0;
                if m1462 == 0 {
                    let m1463 = v2;
                    match m1463.uncons() {
                        None => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v4))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(f8(Str::lit("the declarations could not be ordered so that each follows the types it uses"))))
                        }
                    }
                } else {
                    let v7 = m1462 - 1;
                    let m1464 = v2.clone();
                    match m1464.uncons() {
                        None => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v4))
                        }
                        Some((_, _)) => {
                            let m1465 = f332(v2, v3.clone());
                            match m1465 {
                                None => {
                                    Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(f8(Str::lit("the declarations could not be ordered so that each follows the types it uses"))))
                                }
                                Some(v10) => {
                                    let m1467 = f333(v1.clone(), {
                                        let (h1466, _) = v10.clone();
                                        h1466
                                    });
                                    match m1467 {
                                        Ok(v12) => {
                                            Ok::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>((v7, ({
                                                let (_, h1468) = v10;
                                                h1468
                                            }, (f334({
                                                let (h1469, _) = v10;
                                                h1469
                                            }, v3), f334(v4, v12)))))
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
                Ok(next) => { m1470 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f234(v0: List<Adt26>, v1: List<List<Bytes>>) -> List<(List<Bytes>, List<Bytes>)> {
    let m1475 = v1;
    match m1475.uncons() {
        None => {
            List::<(List<Bytes>, List<Bytes>)>::nil()
        }
        Some((v2, v3)) => {
            List::cons((v2.clone(), f335(v0.clone(), v2.clone(), v2)), f234(v0, v3))
        }
    }
}

pub fn f235(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> List<List<Bytes>> {
    let m1476 = v2;
    match m1476.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((v4, v5)) => {
            if f115(v4.clone(), v3.clone()) {
                f235(v0, v1, v5, v3)
            } else {
                let v6: List<Bytes> = f336(v0.clone(), v1.clone(), v4);
                List::cons(v6.clone(), f235(v0, v1, v5, f334(v6, v3)))
            }
        }
    }
}

pub fn f236(v0: bool) -> bool {
    !v0
}

pub fn f237(v0: Adt25, v1: Str) -> bool {
    f35(match v0 {
        Adt25::C0(_, h1477, _, _, _) => {
            h1477
        }
    }, f8(v1))
}

pub fn f238(v0: Adt25, v1: Str) -> Option<Bytes> {
    f93(match v0 {
        Adt25::C0(_, _, h1478, _, _) => {
            h1478
        }
    }, f8(v1))
}

pub fn f239(v0: Option<Bytes>, v1: Bytes) -> R<Result<Bytes, Bytes>> {
    let m1479 = f133(v1.clone(), f8(Str::lit("://")), 0u64)?;
    match m1479 {
        None => {
            let v3: Bytes = f163(f8(Str::lit("the relative schema location ")), v1.clone(), f8(Str::lit(" is not resolved, because only a bare file name relative to a bound document's URL is merged (RFC 3986, section 5.2.3)")));
            if if f147(v1.clone(), f8(Str::lit("/"))) {
                true
            } else {
                let m1480 = f133(v1.clone(), f8(Str::lit("..")), 0u64)?;
                match m1480 {
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
                let m1481 = v0;
                match m1481 {
                    None => {
                        Ok(Err::<Bytes, Bytes>(v3))
                    }
                    Some(v5) => {
                        let m1483 = f337(v5.clone(), 0u64, {
                            let a1482 = v5.clone();
                            length_bytes(a1482)
                        }, None::<u64>)?;
                        match m1483 {
                            None => {
                                Ok(Err::<Bytes, Bytes>(v3))
                            }
                            Some(v6) => {
                                Ok(Ok::<Bytes, Bytes>({
                                    let a1486 = f32(v5, 0u64, {
                                        let a1484 = v6;
                                        let a1485 = 1u64;
                                        nat_add(a1484, a1485)?
                                    });
                                    let a1487 = v1;
                                    append_bytes(a1486, a1487)
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

pub fn f240(p1492: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1491 = p1492;
    {
        let mut m1491 = m1491;
        loop {
            let step = {
                let v0 = m1491;
                let m1488 = v0;
                match m1488.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h1489, _) = v2;
                            h1489
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h1490) = v2;
                                h1490
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1491 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f241(v0: Adt25, v1: Str, v2: Str) -> Bytes {
    let m1493 = f238(v0, v1);
    match m1493 {
        None => {
            f8(v2)
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f242(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1494 = f301(v0, v1)?;
    match m1494 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f338({
                let (h1495, _) = v2;
                h1495
            }) {
                Ok(Some({
                    let a1497 = v1;
                    let a1498 = {
                        let (_, h1496) = v2;
                        h1496
                    };
                    nat_add(a1497, a1498)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f243(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1500 = v1;
    let a1501 = {
        let a1499 = v0.clone();
        length_bytes(a1499)
    };
    if if nat_lt(a1500, a1501) {
        f339(v0.clone(), v1)?
    } else {
        false
    } {
        Ok(Some(f248(v0, v1)?))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f244(v0: u64) -> bool {
    let a1502 = v0;
    let a1503 = 32u64;
    if nat_eq(a1502, a1503) {
        true
    } else {
        let a1504 = v0;
        let a1505 = 9u64;
        nat_eq(a1504, a1505)
    }
}

pub fn f245(v0: u64) -> bool {
    let a1506 = v0;
    let a1507 = 10u64;
    if nat_eq(a1506, a1507) {
        true
    } else {
        let a1508 = v0;
        let a1509 = 256u64;
        nat_eq(a1508, a1509)
    }
}

pub fn f246(v0: Bytes, v1: u64) -> R<u64> {
    let a1510 = v1;
    let a1511 = f340(v0, v1)?;
    Ok(nat_sub(a1510, a1511))
}

pub fn f247(v0: Bytes, v1: u64) -> R<u64> {
    let (h1512, _) = f130(Fn4::F341(v0.clone()), {
        let a1514 = {
            let a1513 = v0;
            length_bytes(a1513)
        };
        let a1515 = 1u64;
        nat_add(a1514, a1515)?
    }, v1)?;
    Ok(h1512)
}

pub fn f248(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f342(v0.clone(), v1)?;
    let a1517 = v2;
    let a1518 = {
        let a1516 = v0;
        length_bytes(a1516)
    };
    if nat_lt(a1517, a1518) {
        let a1519 = v2;
        let a1520 = 1u64;
        nat_add(a1519, a1520)
    } else {
        Ok(v2)
    }
}

pub fn f249(v0: Bytes, v1: u64) -> R<u64> {
    let (h1521, _) = f130(Fn4::F343(v0.clone()), {
        let a1523 = {
            let a1522 = v0;
            length_bytes(a1522)
        };
        let a1524 = 1u64;
        nat_add(a1523, a1524)?
    }, v1)?;
    Ok(h1521)
}

pub fn f250(v0: Bytes, v1: u64) -> R<bool> {
    let a1525 = f174(v0.clone(), v1)?;
    let a1526 = 45u64;
    if nat_eq(a1525, a1526) {
        if f244(f174(v0.clone(), {
            let a1527 = v1;
            let a1528 = 1u64;
            nat_add(a1527, a1528)?
        })?) {
            Ok(true)
        } else {
            Ok(f245(f174(v0, {
                let a1529 = v1;
                let a1530 = 1u64;
                nat_add(a1529, a1530)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f251(v0: Bytes, v1: u64) -> R<bool> {
    let m1531 = f257(v0, v1)?;
    match m1531 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f252(v0: Bytes, v1: u64) -> R<Option<Bytes>> {
    let v2: u64 = f174(v0.clone(), v1)?;
    let v3: bool = f176(f344(f174(v0, {
        let a1532 = v1;
        let a1533 = 1u64;
        nat_add(a1532, a1533)?
    })?, true));
    let a1534 = v2;
    let a1535 = 58u64;
    if if nat_eq(a1534, a1535) {
        v3
    } else {
        false
    } {
        Ok(Some(f83(Str::lit("a null key is not read: Studio.Value has no null (YAML 1.2.2 section 10.3.2)"), v1)?))
    } else {
        let a1536 = v2;
        let a1537 = 38u64;
        if nat_eq(a1536, a1537) {
            Ok(Some(f83(Str::lit("an anchor is not read (YAML 1.2.2 [101])"), v1)?))
        } else {
            let a1538 = v2;
            let a1539 = 42u64;
            if nat_eq(a1538, a1539) {
                Ok(Some(f83(Str::lit("an alias is not read (YAML 1.2.2 [104])"), v1)?))
            } else {
                let a1540 = v2;
                let a1541 = 33u64;
                if nat_eq(a1540, a1541) {
                    Ok(Some(f83(Str::lit("a tag is not read (YAML 1.2.2 [97])"), v1)?))
                } else {
                    let a1542 = v2;
                    let a1543 = 63u64;
                    if if nat_eq(a1542, a1543) {
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
    let v3: u64 = f174(v0.clone(), v1)?;
    let a1544 = v3;
    let a1545 = 124u64;
    if if nat_eq(a1544, a1545) {
        true
    } else {
        let a1546 = v3;
        let a1547 = 62u64;
        nat_eq(a1546, a1547)
    } {
        f345(v0, v1, v2)
    } else {
        let a1548 = v3;
        let a1549 = 34u64;
        if if nat_eq(a1548, a1549) {
            true
        } else {
            let a1550 = v3;
            let a1551 = 39u64;
            nat_eq(a1550, a1551)
        } {
            let m1552 = f346(v0.clone(), v1, v2, false)?;
            match m1552 {
                Adt30::C0(v5, v6) => {
                    let m1553 = f173(v0, v6)?;
                    match m1553 {
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
            let a1554 = v3;
            let a1555 = 91u64;
            if if nat_eq(a1554, a1555) {
                true
            } else {
                let a1556 = v3;
                let a1557 = 123u64;
                nat_eq(a1556, a1557)
            } {
                let m1561 = f347(v0.clone(), v1, v2, {
                    let a1559 = {
                        let a1558 = v0.clone();
                        length_bytes(a1558)
                    };
                    let a1560 = 2u64;
                    nat_add(a1559, a1560)?
                }, 0u64, List::<Bytes>::nil())?;
                match m1561 {
                    Adt20::C0(v10, v11) => {
                        let a1562 = f174(v0.clone(), f247(v0.clone(), v11)?)?;
                        let a1563 = 58u64;
                        if nat_eq(a1562, a1563) {
                            Ok(Adt20::C1(f83(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [154])"), v1)?))
                        } else {
                            let m1564 = f173(v0, v11)?;
                            match m1564 {
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
                let m1565 = f252(v0.clone(), v1)?;
                match m1565 {
                    None => {
                        if f176(f348(v0.clone(), v1, false)?) {
                            Ok(Adt20::C1(f83(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                        } else {
                            let v15: u64 = f349(v0.clone(), v1, false)?;
                            let m1569 = f350(v0.clone(), v15, v2, {
                                let a1567 = {
                                    let a1566 = v0.clone();
                                    length_bytes(a1566)
                                };
                                let a1568 = 2u64;
                                nat_add(a1567, a1568)?
                            }, f256(v0, v1, v15))?;
                            match m1569 {
                                Adt30::C0(v17, v18) => {
                                    f351(v17, v1, v18)
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
    let m1570 = v1;
    match m1570 {
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
    let (h1571, _) = f130(Fn4::F352(v0.clone(), v1), {
        let a1573 = {
            let a1572 = v0;
            length_bytes(a1572)
        };
        let a1574 = 1u64;
        nat_add(a1573, a1574)?
    }, 0u64)?;
    Ok(h1571)
}

pub fn f256(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    f32(v0, v1, {
        let a1575 = v2;
        let a1576 = v1;
        nat_sub(a1575, a1576)
    })
}

pub fn f257(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f174(v0.clone(), v1)?;
    let a1577 = v2;
    let a1578 = 34u64;
    if if nat_eq(a1577, a1578) {
        true
    } else {
        let a1579 = v2;
        let a1580 = 39u64;
        nat_eq(a1579, a1580)
    } {
        let m1581 = f346(v0.clone(), v1, 0u64, true)?;
        match m1581 {
            Adt30::C0(_, v5) => {
                let v6: u64 = f247(v0.clone(), v5)?;
                if f353(v0, v6)? {
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
        if f348(v0.clone(), v1, false)? {
            let v7: u64 = f247(v0.clone(), f349(v0.clone(), v1, false)?)?;
            if f353(v0, v7)? {
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
    let v2: u64 = f174(v0.clone(), v1)?;
    let a1582 = v2;
    let a1583 = 34u64;
    if if nat_eq(a1582, a1583) {
        true
    } else {
        let a1584 = v2;
        let a1585 = 39u64;
        nat_eq(a1584, a1585)
    } {
        let m1586 = f346(v0, v1, 0u64, true)?;
        match m1586 {
            Adt30::C0(v4, _) => {
                Ok(v4)
            }
            Adt30::C1(_) => {
                Ok(Bytes::lit(&[]))
            }
        }
    } else {
        Ok(f256(v0.clone(), v1, f349(v0, v1, false)?))
    }
}

pub fn f259(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1587 = 0u64;
        let a1588 = v1;
        nat_lt(a1587, a1588)
    } else {
        false
    } {
        let a1589 = Bytes::lit(&[0x2d]);
        let a1590 = f52(v1)?;
        Ok(append_bytes(a1589, a1590))
    } else {
        f52(v1)
    }
}

pub fn f260(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f52(v1)?;
    let v4: u64 = {
        let a1591 = v3.clone();
        length_bytes(a1591)
    };
    let v5: Bytes = {
        let a1592 = v2;
        let a1593 = v4;
        if nat_lt(a1592, a1593) {
            f7(List::cons(f32(v3.clone(), 0u64, {
                let a1594 = v4;
                let a1595 = v2;
                nat_sub(a1594, a1595)
            }), List::cons(f8(Str::lit(".")), List::cons(f32(v3, {
                let a1596 = v4;
                let a1597 = v2;
                nat_sub(a1596, a1597)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f354({
                let a1598 = v2;
                let a1599 = v4;
                nat_sub(a1598, a1599)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1600 = f8(Str::lit("-"));
        let a1601 = v5;
        Ok(append_bytes(a1600, a1601))
    } else {
        Ok(v5)
    }
}

pub fn f261(v0: Bytes) -> R<Bytes> {
    let a1606 = f355(v0.clone(), 0u64, {
        let a1603 = {
            let a1602 = v0.clone();
            length_bytes(a1602)
        };
        let a1604 = 1u64;
        nat_add(a1603, a1604)?
    })?;
    let a1607 = {
        let a1605 = v0.clone();
        length_bytes(a1605)
    };
    if nat_eq(a1606, a1607) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        f356(v0)
    }
}

pub fn f262(v0: List<Bytes>) -> Bytes {
    let a1610 = {
        let a1608 = Bytes::lit(&[0x5b]);
        let a1609 = f33(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1608, a1609)
    };
    let a1611 = Bytes::lit(&[0x5d]);
    append_bytes(a1610, a1611)
}

pub fn f263(v0: Adt19) -> R<List<Bytes>> {
    let m1612 = v0;
    match m1612 {
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
        Adt19::C9(r1613, r1614) => {
            let v12 = (*r1613).clone();
            let v13 = (*r1614).clone();
            Ok(List::cons(f177(v12)?, f263(v13)?))
        }
        Adt19::C10(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f264(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f357(Fn5::F358, List::<(Bytes, Bytes)>::nil(), f359(v0)?);
    let v2: List<Bytes> = f360(Fn6::F361, List::<Bytes>::nil(), v1)?;
    let a1617 = {
        let a1615 = Bytes::lit(&[0x7b]);
        let a1616 = f33(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1615, a1616)
    };
    let a1618 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1617, a1618))
}

pub fn f265(v0: Adt19) -> R<List<(Bytes, Bytes)>> {
    let m1619 = v0;
    match m1619 {
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
        Adt19::C10(v14, r1620, r1621) => {
            let v15 = (*r1620).clone();
            let v16 = (*r1621).clone();
            Ok(List::cons((v14, f177(v15)?), f265(v16)?))
        }
    }
}

pub fn f266(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a1622 = v0;
        utf8_encode(a1622)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f52(v1)?, List::<Bytes>::nil())))))
}

pub fn f267(p1625: List<Adt21>, p1626: List<Adt21>) -> List<Adt21> {
    let m1624 = (p1625, p1626);
    {
        let mut m1624 = m1624;
        loop {
            let step = {
                let (v0, v1) = m1624;
                let m1623 = v0;
                match m1623.uncons() {
                    None => {
                        Err::<(List<Adt21>, List<Adt21>), List<Adt21>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt21>, List<Adt21>), List<Adt21>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1624 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f268(v0: u64) -> bool {
    let a1627 = 65u64;
    let a1628 = v0;
    if if nat_le(a1627, a1628) {
        let a1629 = v0;
        let a1630 = 90u64;
        nat_le(a1629, a1630)
    } else {
        false
    } {
        true
    } else {
        let a1631 = 97u64;
        let a1632 = v0;
        if nat_le(a1631, a1632) {
            let a1633 = v0;
            let a1634 = 122u64;
            nat_le(a1633, a1634)
        } else {
            false
        }
    }
}

pub fn f269(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h1635, _) = f130(Fn4::F362(v1, v0.clone()), {
        let a1637 = {
            let a1636 = v0;
            length_bytes(a1636)
        };
        let a1638 = 1u64;
        nat_add(a1637, a1638)?
    }, v2)?;
    Ok(h1635)
}

pub fn f270(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f87(v0.clone(), v1)?;
    let a1639 = v2;
    let a1640 = 48u64;
    if nat_eq(a1639, a1640) {
        let v3: u64 = f87(v0.clone(), {
            let a1641 = v1;
            let a1642 = 1u64;
            nat_add(a1641, a1642)?
        })?;
        let a1643 = v3;
        let a1644 = 88u64;
        if if if nat_eq(a1643, a1644) {
            true
        } else {
            let a1645 = v3;
            let a1646 = 120u64;
            nat_eq(a1645, a1646)
        } {
            f363(f87(v0.clone(), {
                let a1647 = v1;
                let a1648 = 2u64;
                nat_add(a1647, a1648)?
            })?)
        } else {
            false
        } {
            f269(v0, 1u64, {
                let a1649 = v1;
                let a1650 = 2u64;
                nat_add(a1649, a1650)?
            })
        } else {
            f269(v0, 2u64, {
                let a1651 = v1;
                let a1652 = 1u64;
                nat_add(a1651, a1652)?
            })
        }
    } else {
        if f364(v2) {
            f269(v0, 0u64, v1)
        } else {
            Ok(v1)
        }
    }
}

pub fn f271(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f269(v0.clone(), 0u64, v1)?;
    let a1653 = f87(v0.clone(), v2)?;
    let a1654 = 46u64;
    if nat_eq(a1653, a1654) {
        let v3: u64 = f269(v0.clone(), 0u64, {
            let a1655 = v2;
            let a1656 = 1u64;
            nat_add(a1655, a1656)?
        })?;
        let a1657 = v1;
        let a1658 = v2;
        if if nat_lt(a1657, a1658) {
            true
        } else {
            let a1661 = {
                let a1659 = v2;
                let a1660 = 1u64;
                nat_add(a1659, a1660)?
            };
            let a1662 = v3;
            nat_lt(a1661, a1662)
        } {
            f365(v0, v3)
        } else {
            Ok(v1)
        }
    } else {
        let a1663 = v1;
        let a1664 = v2;
        if nat_lt(a1663, a1664) {
            let v4: u64 = f365(v0, v2)?;
            let a1665 = v2;
            let a1666 = v4;
            if nat_lt(a1665, a1666) {
                Ok(v4)
            } else {
                Ok(v1)
            }
        } else {
            Ok(v1)
        }
    }
}

pub fn f272(v0: List<Adt21>, v1: Str) -> bool {
    let m1667 = v0;
    match m1667.uncons() {
        None => {
            false
        }
        Some((v2, _)) => {
            f366(v2, {
                let a1668 = v1;
                utf8_encode(a1668)
            })
        }
    }
}

pub fn f273(p1681: u64, p1682: List<Adt21>, p1683: List<Bytes>, v3: bool) -> Result<List<Adt21>, Bytes> {
    let m1680 = (p1681, (p1682, p1683));
    {
        let mut m1680 = m1680;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1680;
                let m1669 = v0;
                if m1669 == 0 {
                    Err::<(u64, (List<Adt21>, List<Bytes>)), Result<List<Adt21>, Bytes>>(Err::<List<Adt21>, Bytes>(f8(Str::lit("brackets nest deeper than the input is long"))))
                } else {
                    let v4 = m1669 - 1;
                    let m1670 = v1;
                    match m1670.uncons() {
                        None => {
                            Err::<(u64, (List<Adt21>, List<Bytes>)), Result<List<Adt21>, Bytes>>(Err::<List<Adt21>, Bytes>(f8(Str::lit("the input ends inside brackets or before the ; that closes a member (Web IDL, IDL grammar)"))))
                        }
                        Some((v5, v6)) => {
                            let a1671 = f367(v5.clone());
                            let a1672 = 3u64;
                            if nat_eq(a1671, a1672) {
                                Ok::<(u64, (List<Adt21>, List<Bytes>)), Result<List<Adt21>, Bytes>>((v4, (v6, v2)))
                            } else {
                                let v7: Bytes = f368(v5);
                                let v8: Bytes = f369(v7.clone());
                                if !f35(v8.clone(), Bytes::lit(&[])) {
                                    Ok::<(u64, (List<Adt21>, List<Bytes>)), Result<List<Adt21>, Bytes>>((v4, (v6, List::cons(v8, v2))))
                                } else {
                                    if f370(v7.clone()) {
                                        let m1673 = v2;
                                        match m1673.uncons() {
                                            None => {
                                                Err::<(u64, (List<Adt21>, List<Bytes>)), Result<List<Adt21>, Bytes>>(Err::<List<Adt21>, Bytes>(f7(List::cons(f8(Str::lit("the closing bracket ")), List::cons(v7, List::cons(f8(Str::lit(" has no opening bracket (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
                                            }
                                            Some((v9, v10)) => {
                                                if f35(v9.clone(), v7.clone()) {
                                                    if if !v3 {
                                                        let a1675 = {
                                                            let a1674 = v10.clone();
                                                            length_list(a1674)
                                                        };
                                                        let a1676 = 0u64;
                                                        nat_eq(a1675, a1676)
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
                                                let a1678 = {
                                                    let a1677 = v2.clone();
                                                    length_list(a1677)
                                                };
                                                let a1679 = 0u64;
                                                nat_eq(a1678, a1679)
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
                Ok(next) => { m1680 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f274(v0: List<Adt21>) -> List<Adt21> {
    let m1684 = v0;
    match m1684.uncons() {
        None => {
            List::<Adt21>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f275(v0: u64, v1: u64, v2: List<Adt21>, v3: bool) -> R<Result<(Adt12, List<Adt21>), Bytes>> {
    let m1685 = f278(v2);
    match m1685 {
        Ok(v5) => {
            let v6: bool = if v3 {
                f272(f280(v5.clone()), Str::lit(":"))
            } else {
                false
            };
            let m1686 = if v6 {
                f278(f274(f280(v5.clone())))
            } else {
                Ok::<Adt28, Bytes>(Adt28::C0(Bytes::lit(&[]), f280(v5.clone())))
            };
            match m1686 {
                Ok(v8) => {
                    let m1687 = f279(f280(v8.clone()), Str::lit("{"));
                    match m1687 {
                        Ok(v10) => {
                            let m1692 = {
                                let a1688 = v1;
                                let a1689 = 2u64;
                                if if nat_eq(a1688, a1689) {
                                    true
                                } else {
                                    let a1690 = v1;
                                    let a1691 = 7u64;
                                    nat_eq(a1690, a1691)
                                } {
                                    f371(v0, v10, List::<Adt13>::nil())?
                                } else {
                                    f372(v0, v10, List::<Adt13>::nil())?
                                }
                            };
                            match m1692 {
                                Ok(v12) => {
                                    Ok(Ok::<(Adt12, List<Adt21>), Bytes>((Adt12::C0(v1, f282(v5), f282(v8), {
                                        let (h1693, _) = v12;
                                        h1693
                                    }, List::<Bytes>::nil(), f283(Bytes::lit(&[]))), {
                                        let (_, h1694) = v12;
                                        h1694
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

pub fn f276(v0: List<Adt21>, v1: Bytes) -> R<Result<(Adt12, List<Adt21>), Bytes>> {
    let m1695 = f278(v0);
    match m1695 {
        Ok(v3) => {
            let m1696 = f373(f280(v3.clone()))?;
            match m1696 {
                Ok(v5) => {
                    Ok(Ok::<(Adt12, List<Adt21>), Bytes>((Adt12::C0(9u64, f282(v3), Bytes::lit(&[]), List::<Adt13>::nil(), List::cons(v1, List::<Bytes>::nil()), f283(Bytes::lit(&[]))), v5)))
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

pub fn f277(v0: Str, v1: List<Adt21>) -> Bytes {
    f7(List::cons(f8(Str::lit("expected ")), List::cons({
        let a1697 = v0;
        utf8_encode(a1697)
    }, List::cons(f8(Str::lit(" but found ")), List::cons(f374(v1), List::cons(f8(Str::lit(" (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
}

pub fn f278(v0: List<Adt21>) -> Result<Adt28, Bytes> {
    let m1698 = v0.clone();
    match m1698.uncons() {
        None => {
            Err::<Adt28, Bytes>(f277(Str::lit("an identifier"), v0))
        }
        Some((v1, v2)) => {
            let a1699 = f367(v1.clone());
            let a1700 = 0u64;
            if nat_eq(a1699, a1700) {
                Ok::<Adt28, Bytes>(Adt28::C0(f375(f368(v1)), v2))
            } else {
                Err::<Adt28, Bytes>(f277(Str::lit("an identifier"), v0))
            }
        }
    }
}

pub fn f279(v0: List<Adt21>, v1: Str) -> Result<List<Adt21>, Bytes> {
    if f272(v0.clone(), v1.clone()) {
        Ok::<List<Adt21>, Bytes>(f274(v0))
    } else {
        Err::<List<Adt21>, Bytes>(f277(v1, v0))
    }
}

pub fn f280(v0: Adt28) -> List<Adt21> {
    let m1701 = v0;
    match m1701 {
        Adt28::C0(_, v2) => {
            v2
        }
    }
}

pub fn f281(p1709: u64, p1710: List<Adt21>, p1711: List<Bytes>) -> Result<(List<Bytes>, List<Adt21>), Bytes> {
    let m1708 = (p1709, (p1710, p1711));
    {
        let mut m1708 = m1708;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1708;
                let m1702 = v0;
                if m1702 == 0 {
                    Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Err::<(List<Bytes>, List<Adt21>), Bytes>(f8(Str::lit("an enumeration has more values than the input has tokens"))))
                } else {
                    let v3 = m1702 - 1;
                    let m1703 = v1.clone();
                    match m1703.uncons() {
                        None => {
                            Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Err::<(List<Bytes>, List<Adt21>), Bytes>(f277(Str::lit("a string"), v1)))
                        }
                        Some((v4, v5)) => {
                            let a1704 = f367(v4.clone());
                            let a1705 = 3u64;
                            if !nat_eq(a1704, a1705) {
                                Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Err::<(List<Bytes>, List<Adt21>), Bytes>(f277(Str::lit("a string"), v1)))
                            } else {
                                if f272(v5.clone(), Str::lit(",")) {
                                    if f272(f274(v5.clone()), Str::lit("}")) {
                                        let m1706 = f279(f274(f274(v5)), Str::lit(";"));
                                        match m1706 {
                                            Ok(v7) => {
                                                Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Ok::<(List<Bytes>, List<Adt21>), Bytes>((f45(List::cons(f368(v4), v2)), v7)))
                                            }
                                            Err(v6) => {
                                                Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Err::<(List<Bytes>, List<Adt21>), Bytes>(v6))
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>((v3, (f274(v5), List::cons(f368(v4), v2))))
                                    }
                                } else {
                                    if f272(v5.clone(), Str::lit("}")) {
                                        let m1707 = f279(f274(v5), Str::lit(";"));
                                        match m1707 {
                                            Ok(v9) => {
                                                Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Ok::<(List<Bytes>, List<Adt21>), Bytes>((f45(List::cons(f368(v4), v2)), v9)))
                                            }
                                            Err(v8) => {
                                                Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Err::<(List<Bytes>, List<Adt21>), Bytes>(v8))
                                            }
                                        }
                                    } else {
                                        Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Err::<(List<Bytes>, List<Adt21>), Bytes>(f277(Str::lit(", or }"), v5)))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1708 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f282(v0: Adt28) -> Bytes {
    let m1712 = v0;
    match m1712 {
        Adt28::C0(v1, _) => {
            v1
        }
    }
}

pub fn f283(v0: Bytes) -> Adt14 {
    Adt14::C0(0u64, v0, List::<Bytes>::nil(), Bytes::lit(&[]), Bytes::lit(&[]))
}

pub fn f284(v0: u64, v1: List<Adt21>, v2: u64) -> R<Result<Adt29, Bytes>> {
    let m1713 = v0;
    if m1713 == 0 {
        Ok(Err::<Adt29, Bytes>(f8(Str::lit("a type nests deeper than the input is long"))))
    } else {
        let v3 = m1713 - 1;
        let a1714 = v2;
        let a1715 = 0u64;
        if nat_eq(a1714, a1715) {
            let m1716 = f180(v1)?;
            match m1716 {
                Ok(v5) => {
                    if f272(v5.clone(), Str::lit("(")) {
                        let m1717 = f284(v3, f274(v5), 1u64)?;
                        match m1717 {
                            Ok(v7) => {
                                if f272(f285(v7.clone()), Str::lit("?")) {
                                    Ok(Ok::<Adt29, Bytes>(Adt29::C0(Adt14::C0(0u64, Bytes::lit(&[]), f199(f286(v7.clone())), f8(Str::lit("a nullable union type is not translated yet, because the typedef identifier names only the union (Web IDL 2.13.32)")), Bytes::lit(&[])), f274(f285(v7)))))
                                } else {
                                    Ok(Ok::<Adt29, Bytes>(Adt29::C0(f286(v7.clone()), f285(v7))))
                                }
                            }
                            Err(v6) => {
                                Ok(Err::<Adt29, Bytes>(v6))
                            }
                        }
                    } else {
                        let m1718 = v5.clone();
                        match m1718.uncons() {
                            None => {
                                Ok(Err::<Adt29, Bytes>(f277(Str::lit("a type"), v5)))
                            }
                            Some((v8, v9)) => {
                                let v10: Bytes = f368(v8.clone());
                                let a1719 = f367(v8);
                                let a1720 = 0u64;
                                if !nat_eq(a1719, a1720) {
                                    Ok(Err::<Adt29, Bytes>(f277(Str::lit("a type"), v5)))
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
                                        let m1721 = f279(v9, Str::lit("<"));
                                        match m1721 {
                                            Ok(v12) => {
                                                let m1722 = f284(v3, v12, 0u64)?;
                                                match m1722 {
                                                    Ok(v14) => {
                                                        let m1723 = f279(f285(v14.clone()), Str::lit(">"));
                                                        match m1723 {
                                                            Ok(v16) => {
                                                                Ok(f376(f377(Str::lit("List"), f286(v14)), v16))
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
                                            let m1724 = f279(v9, Str::lit("<"));
                                            match m1724 {
                                                Ok(v18) => {
                                                    let m1725 = f284(v3, v18, 0u64)?;
                                                    match m1725 {
                                                        Ok(v20) => {
                                                            let m1726 = f279(f285(v20), Str::lit(">"));
                                                            match m1726 {
                                                                Ok(v22) => {
                                                                    Ok(f376(f378(f8(Str::lit("async_sequence types are not translated, because an async sequence is produced over time and is not a value (Web IDL 2.13.29)"))), v22))
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
                                                let m1727 = f279(v9, Str::lit("<"));
                                                match m1727 {
                                                    Ok(v24) => {
                                                        let m1728 = f284(v3, v24, 0u64)?;
                                                        match m1728 {
                                                            Ok(v26) => {
                                                                let m1729 = f279(f285(v26), Str::lit(">"));
                                                                match m1729 {
                                                                    Ok(v28) => {
                                                                        Ok(f376(f378(f8(Str::lit("Promise types are not translated, because a promise is a deferred result, not a value (Web IDL 2.13.31)"))), v28))
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
                                                    let m1730 = f279(v9, Str::lit("<"));
                                                    match m1730 {
                                                        Ok(v30) => {
                                                            let m1731 = v30.clone();
                                                            match m1731.uncons() {
                                                                None => {
                                                                    Ok(Err::<Adt29, Bytes>(f277(Str::lit("a string type"), v30)))
                                                                }
                                                                Some((v31, v32)) => {
                                                                    if if f35(f368(v31.clone()), f8(Str::lit("DOMString"))) {
                                                                        true
                                                                    } else {
                                                                        if f35(f368(v31.clone()), f8(Str::lit("USVString"))) {
                                                                            true
                                                                        } else {
                                                                            f35(f368(v31.clone()), f8(Str::lit("ByteString")))
                                                                        }
                                                                    } {
                                                                        let m1732 = f279(v32, Str::lit(","));
                                                                        match m1732 {
                                                                            Ok(v34) => {
                                                                                let m1733 = f284(v3, v34, 0u64)?;
                                                                                match m1733 {
                                                                                    Ok(v36) => {
                                                                                        let m1734 = f279(f285(v36.clone()), Str::lit(">"));
                                                                                        match m1734 {
                                                                                            Ok(v38) => {
                                                                                                let m1735 = f379(f368(v31));
                                                                                                match m1735 {
                                                                                                    None => {
                                                                                                        Ok(Err::<Adt29, Bytes>(f277(Str::lit("a string type"), v30)))
                                                                                                    }
                                                                                                    Some(v39) => {
                                                                                                        let v40: Adt14 = f380(f286(v36));
                                                                                                        Ok(f376(Adt14::C0(0u64, f7(List::cons(f8(Str::lit("(List (")), List::cons(f198(v39), List::cons(f8(Str::lit(" * ")), List::cons(f198(v40.clone()), List::cons(f8(Str::lit("))")), List::<Bytes>::nil())))))), f199(v40.clone()), f196(v40), Bytes::lit(&[])), v38))
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
                                                                        Ok(Err::<Adt29, Bytes>(f277(Str::lit("DOMString, USVString or ByteString"), v30)))
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
                                                        if f272(v9.clone(), Str::lit("short")) {
                                                            Ok(f376(f283(f8(Str::lit("Nat"))), f274(v9)))
                                                        } else {
                                                            if f272(v9.clone(), Str::lit("long")) {
                                                                let v41: List<Adt21> = f274(v9);
                                                                Ok(f376(f283(f8(Str::lit("Nat"))), if f272(v41.clone(), Str::lit("long")) {
                                                                    f274(v41)
                                                                } else {
                                                                    v41
                                                                }))
                                                            } else {
                                                                Ok(Err::<Adt29, Bytes>(f277(Str::lit("short or long"), v9)))
                                                            }
                                                        }
                                                    } else {
                                                        if f35(v10.clone(), f8(Str::lit("short"))) {
                                                            Ok(f376(f283(f8(Str::lit("Int"))), v9))
                                                        } else {
                                                            if f35(v10.clone(), f8(Str::lit("long"))) {
                                                                Ok(f376(f283(f8(Str::lit("Int"))), if f272(v9.clone(), Str::lit("long")) {
                                                                    f274(v9)
                                                                } else {
                                                                    v9
                                                                }))
                                                            } else {
                                                                if f35(v10.clone(), f8(Str::lit("unrestricted"))) {
                                                                    if if f272(v9.clone(), Str::lit("float")) {
                                                                        true
                                                                    } else {
                                                                        f272(v9.clone(), Str::lit("double"))
                                                                    } {
                                                                        Ok(f376(f378(f8(Str::lit("unrestricted float and unrestricted double are not translated, because its value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.13 and 2.13.15)"))), f274(v9)))
                                                                    } else {
                                                                        Ok(Err::<Adt29, Bytes>(f277(Str::lit("float or double"), v9)))
                                                                    }
                                                                } else {
                                                                    let m1736 = f379(v10.clone());
                                                                    match m1736 {
                                                                        None => {
                                                                            let v42: Bytes = f375(v10);
                                                                            Ok(f376(Adt14::C0(0u64, f183(v42.clone())?, List::cons(v42.clone(), List::<Bytes>::nil()), Bytes::lit(&[]), v42), v9))
                                                                        }
                                                                        Some(v43) => {
                                                                            Ok(f376(v43, v9))
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
            let m1737 = f284(v3, v1, 0u64)?;
            match m1737 {
                Ok(v45) => {
                    let v46: Adt14 = f286(v45.clone());
                    let v48: Adt14 = {
                        let a1738 = f197(v46.clone());
                        let a1739 = 0u64;
                        if nat_eq(a1738, a1739) {
                            if f35(f196(v46.clone()), Bytes::lit(&[])) {
                                if f35(f381(v46.clone()), Bytes::lit(&[])) {
                                    Adt14::C0(1u64, Bytes::lit(&[]), f199(v46), f8(Str::lit("a union member type that is not a named definition gives its constructor no name, so the union is not translated yet (Web IDL 2.13.32)")), Bytes::lit(&[]))
                                } else {
                                    let m1740 = f296(false, f381(v46.clone()))?;
                                    match m1740 {
                                        None => {
                                            Adt14::C0(1u64, Bytes::lit(&[]), f199(v46.clone()), f7(List::cons(f8(Str::lit("the union member ")), List::cons(f381(v46), List::cons(f8(Str::lit(" has no constructor name")), List::<Bytes>::nil())))), Bytes::lit(&[]))
                                        }
                                        Some(v47) => {
                                            Adt14::C0(1u64, f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v47, List::cons(f8(Str::lit(", fields: [")), List::cons(f198(v46.clone()), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f199(v46), Bytes::lit(&[]), Bytes::lit(&[]))
                                        }
                                    }
                                }
                            } else {
                                Adt14::C0(1u64, Bytes::lit(&[]), f199(v46.clone()), f196(v46), Bytes::lit(&[]))
                            }
                        } else {
                            v46
                        }
                    };
                    if f272(f285(v45.clone()), Str::lit("or")) {
                        let m1741 = f284(v3, f274(f285(v45)), 2u64)?;
                        match m1741 {
                            Ok(v50) => {
                                Ok(Ok::<Adt29, Bytes>(Adt29::C0(Adt14::C0(1u64, if f35(f198(v48.clone()), Bytes::lit(&[])) {
                                    f198(f286(v50.clone()))
                                } else {
                                    if f35(f198(f286(v50.clone())), Bytes::lit(&[])) {
                                        f198(v48.clone())
                                    } else {
                                        f7(List::cons(f198(v48.clone()), List::cons(f8(Str::lit(",\u{a}")), List::cons(f198(f286(v50.clone())), List::<Bytes>::nil()))))
                                    }
                                }, f118(f199(v48.clone()), f199(f286(v50.clone()))), if f35(f196(v48.clone()), Bytes::lit(&[])) {
                                    f196(f286(v50.clone()))
                                } else {
                                    f196(v48)
                                }, Bytes::lit(&[])), f285(v50))))
                            }
                            Err(v49) => {
                                Ok(Err::<Adt29, Bytes>(v49))
                            }
                        }
                    } else {
                        let a1742 = v2;
                        let a1743 = 2u64;
                        if if nat_eq(a1742, a1743) {
                            f272(f285(v45.clone()), Str::lit(")"))
                        } else {
                            false
                        } {
                            Ok(Ok::<Adt29, Bytes>(Adt29::C0(v48, f274(f285(v45)))))
                        } else {
                            Ok(Err::<Adt29, Bytes>({
                                let a1744 = v2;
                                let a1745 = 1u64;
                                if nat_eq(a1744, a1745) {
                                    f277(Str::lit("or"), f285(v45))
                                } else {
                                    f277(Str::lit("or or )"), f285(v45))
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

pub fn f285(v0: Adt29) -> List<Adt21> {
    let m1746 = v0;
    match m1746 {
        Adt29::C0(_, v2) => {
            v2
        }
    }
}

pub fn f286(v0: Adt29) -> Adt14 {
    let m1747 = v0;
    match m1747 {
        Adt29::C0(v1, _) => {
            v1
        }
    }
}

pub fn f287() -> List<Bytes> {
    List::cons(f8(Str::lit("Bool")), List::cons(f8(Str::lit("ContractViolation")), List::cons(f8(Str::lit("List")), List::cons(f8(Str::lit("Nat")), List::cons(f8(Str::lit("Option")), List::cons(f8(Str::lit("Prod")), List::cons(f8(Str::lit("ReasoningFailure")), List::cons(f8(Str::lit("Result")), List::<Bytes>::nil()))))))))
}

pub fn f288(v0: Bytes) -> R<Bytes> {
    let m1750 = {
        let a1748 = v0.clone();
        let a1749 = 0u64;
        index_bytes(a1748, a1749)
    };
    match m1750 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            let a1756 = {
                let a1751 = f8(Str::lit("u"));
                let a1752 = f382(8u64, f42(v1)?, 4u64, Bytes::lit(&[]));
                append_bytes(a1751, a1752)
            };
            let a1757 = f32(v0.clone(), 1u64, {
                let a1754 = {
                    let a1753 = v0;
                    length_bytes(a1753)
                };
                let a1755 = 1u64;
                nat_sub(a1754, a1755)
            });
            Ok(append_bytes(a1756, a1757))
        }
    }
}

pub fn f289(v0: List<Adt13>) -> R<List<Adt13>> {
    let m1758 = v0;
    match m1758.uncons() {
        None => {
            Ok(List::<Adt13>::nil())
        }
        Some((v1, v2)) => {
            f383(v1, f289(v2)?)
        }
    }
}

pub fn f290(v0: List<Adt13>, v1: List<Adt13>) -> List<Adt13> {
    let m1759 = v0;
    match m1759.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f290(v3, v1))
        }
    }
}

pub fn f291(v0: Adt12) -> List<Adt13> {
    let m1760 = v0;
    match m1760 {
        Adt12::C0(_, _, _, v4, _, _) => {
            v4
        }
    }
}

pub fn f292(v0: List<Adt12>, v1: u64, v2: Bytes) -> List<Adt13> {
    let m1761 = v0;
    match m1761.uncons() {
        None => {
            List::<Adt13>::nil()
        }
        Some((v3, v4)) => {
            let a1762 = f112(v3.clone());
            let a1763 = v1;
            if if nat_eq(a1762, a1763) {
                f35(f114(v3.clone()), v2.clone())
            } else {
                false
            } {
                f290(f291(v3), f292(v4, v1, v2))
            } else {
                f292(v4, v1, v2)
            }
        }
    }
}

pub fn f293(v0: List<Adt12>, v1: List<Adt12>, v2: Bytes) -> List<Adt13> {
    let m1764 = v0;
    match m1764.uncons() {
        None => {
            List::<Adt13>::nil()
        }
        Some((v3, v4)) => {
            let a1765 = f112(v3.clone());
            let a1766 = 8u64;
            if if nat_eq(a1765, a1766) {
                f35(f114(v3.clone()), v2.clone())
            } else {
                false
            } {
                f290(f384(v1.clone(), f125(v3)), f293(v4, v1, v2))
            } else {
                f293(v4, v1, v2)
            }
        }
    }
}

pub fn f294(v0: Adt13) -> Adt14 {
    let m1767 = v0;
    match m1767 {
        Adt13::C0(_, v2, _) => {
            v2
        }
    }
}

pub fn f295(v0: Adt13) -> Bytes {
    let m1768 = v0;
    match m1768 {
        Adt13::C0(v1, _, _) => {
            v1
        }
    }
}

pub fn f296(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f385(v1)
    } else {
        f299(v1)
    }
}

pub fn f297(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1769 = v1;
    match m1769.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m1770 = f296(v0, v2)?;
            match m1770 {
                None => {
                    f297(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f297(v0, v3)?))
                }
            }
        }
    }
}

pub fn f298(v0: Adt13) -> u64 {
    let m1771 = v0;
    match m1771 {
        Adt13::C0(_, _, v3) => {
            v3
        }
    }
}

pub fn f299(v0: Bytes) -> R<Option<Bytes>> {
    let m1772 = f386(v0)?;
    match m1772 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            Ok(Some(f387(v1)?))
        }
    }
}

pub fn f300(v0: bool, v1: Adt19) -> R<Option<Bytes>> {
    let m1773 = v1;
    match m1773 {
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
                let a1784 = if f315(v7, {
                    let a1782 = 0u64;
                    let a1783 = v8;
                    nat_lt(a1782, a1783)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a1785 = f259(false, v8)?;
                append_bytes(a1784, a1785)
            }))
        }
        Adt19::C4(v3, v4, v5) => {
            let m1776 = f385({
                let a1774 = f8(Str::lit("n"));
                let a1775 = f260(false, v4, v5)?;
                append_bytes(a1774, a1775)
            })?;
            match m1776 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a1780 = f8(Str::lit("nMinus"));
                        let a1781 = f32(v6.clone(), 1u64, {
                            let a1778 = {
                                let a1777 = v6;
                                length_bytes(a1777)
                            };
                            let a1779 = 1u64;
                            nat_sub(a1778, a1779)
                        });
                        append_bytes(a1780, a1781)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt19::C5(v2) => {
            f296(v0, v2)
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

pub fn f301(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m1788 = {
        let a1786 = v0.clone();
        let a1787 = v1;
        index_bytes(a1786, a1787)
    };
    match m1788 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f42(v2)?;
            let a1789 = v3;
            let a1790 = 128u64;
            if nat_lt(a1789, a1790) {
                Ok(Some((v3, 1u64)))
            } else {
                if f315({
                    let a1791 = 194u64;
                    let a1792 = v3;
                    nat_le(a1791, a1792)
                }, {
                    let a1793 = v3;
                    let a1794 = 223u64;
                    nat_le(a1793, a1794)
                }) {
                    let m1797 = f388(v0, {
                        let a1795 = v1;
                        let a1796 = 1u64;
                        nat_add(a1795, a1796)?
                    })?;
                    match m1797 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f389({
                                let a1802 = {
                                    let a1800 = {
                                        let a1798 = v3;
                                        let a1799 = 192u64;
                                        nat_sub(a1798, a1799)
                                    };
                                    let a1801 = 64u64;
                                    nat_mul(a1800, a1801)?
                                };
                                let a1803 = v4;
                                nat_add(a1802, a1803)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f315({
                        let a1804 = 224u64;
                        let a1805 = v3;
                        nat_le(a1804, a1805)
                    }, {
                        let a1806 = v3;
                        let a1807 = 239u64;
                        nat_le(a1806, a1807)
                    }) {
                        let m1810 = f388(v0.clone(), {
                            let a1808 = v1;
                            let a1809 = 1u64;
                            nat_add(a1808, a1809)?
                        })?;
                        match m1810 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m1813 = f388(v0, {
                                    let a1811 = v1;
                                    let a1812 = 2u64;
                                    nat_add(a1811, a1812)?
                                })?;
                                match m1813 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f389({
                                            let a1822 = {
                                                let a1816 = {
                                                    let a1814 = v3;
                                                    let a1815 = 224u64;
                                                    nat_sub(a1814, a1815)
                                                };
                                                let a1817 = 4096u64;
                                                nat_mul(a1816, a1817)?
                                            };
                                            let a1823 = {
                                                let a1820 = {
                                                    let a1818 = v5;
                                                    let a1819 = 64u64;
                                                    nat_mul(a1818, a1819)?
                                                };
                                                let a1821 = v6;
                                                nat_add(a1820, a1821)?
                                            };
                                            nat_add(a1822, a1823)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f315({
                            let a1824 = 240u64;
                            let a1825 = v3;
                            nat_le(a1824, a1825)
                        }, {
                            let a1826 = v3;
                            let a1827 = 244u64;
                            nat_le(a1826, a1827)
                        }) {
                            let m1830 = f388(v0.clone(), {
                                let a1828 = v1;
                                let a1829 = 1u64;
                                nat_add(a1828, a1829)?
                            })?;
                            match m1830 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m1833 = f388(v0.clone(), {
                                        let a1831 = v1;
                                        let a1832 = 2u64;
                                        nat_add(a1831, a1832)?
                                    })?;
                                    match m1833 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m1836 = f388(v0, {
                                                let a1834 = v1;
                                                let a1835 = 3u64;
                                                nat_add(a1834, a1835)?
                                            })?;
                                            match m1836 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f389({
                                                        let a1849 = {
                                                            let a1839 = {
                                                                let a1837 = v3;
                                                                let a1838 = 240u64;
                                                                nat_sub(a1837, a1838)
                                                            };
                                                            let a1840 = 262144u64;
                                                            nat_mul(a1839, a1840)?
                                                        };
                                                        let a1850 = {
                                                            let a1847 = {
                                                                let a1841 = v7;
                                                                let a1842 = 4096u64;
                                                                nat_mul(a1841, a1842)?
                                                            };
                                                            let a1848 = {
                                                                let a1845 = {
                                                                    let a1843 = v8;
                                                                    let a1844 = 64u64;
                                                                    nat_mul(a1843, a1844)?
                                                                };
                                                                let a1846 = v9;
                                                                nat_add(a1845, a1846)?
                                                            };
                                                            nat_add(a1847, a1848)?
                                                        };
                                                        nat_add(a1849, a1850)?
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

pub fn f302(v0: u64) -> R<bool> {
    f204(v0, f390())
}

pub fn f303(v0: u64, v1: Bytes, p1861: u64, p1862: u64, p1863: bool) -> R<bool> {
    let m1860 = (p1861, (p1862, p1863));
    {
        let mut m1860 = m1860;
        loop {
            let step = {
                let (v2, (v3, v4)) = m1860;
                let m1851 = v3;
                if m1851 == 0 {
                    Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                } else {
                    let v5 = m1851 - 1;
                    let v6: u64 = f391(v1.clone(), v2)?;
                    if v4 {
                        let a1852 = v0;
                        let a1853 = v6;
                        if nat_lt(a1852, a1853) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a1854 = v2;
                                let a1855 = 1u64;
                                nat_add(a1854, a1855)?
                            }, (v5, false)))
                        }
                    } else {
                        let a1856 = v0;
                        let a1857 = v6;
                        if nat_le(a1856, a1857) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(true))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a1858 = v2;
                                let a1859 = 1u64;
                                nat_add(a1858, a1859)?
                            }, (v5, true)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1860 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f304(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    if f79(v0.clone(), v1, v2) {
        Ok(None::<u64>)
    } else {
        let a1865 = v2;
        let a1866 = {
            let a1864 = v0;
            length_bytes(a1864)
        };
        if nat_lt(a1865, a1866) {
            Ok(Some({
                let a1867 = v2;
                let a1868 = 1u64;
                nat_add(a1867, a1868)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f305(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f132(f87(v0, v1)?)? {
        Ok(Some({
            let a1869 = v1;
            let a1870 = 1u64;
            nat_add(a1869, a1870)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f306(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let m1871 = f301(v0, v2)?;
    match m1871 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            let a1872 = v2;
            let a1873 = v1;
            if if nat_eq(a1872, a1873) {
                f392({
                    let (h1874, _) = v3;
                    h1874
                })?
            } else {
                f393({
                    let (h1875, _) = v3;
                    h1875
                })?
            } {
                Ok(Some({
                    let a1877 = v2;
                    let a1878 = {
                        let (_, h1876) = v3;
                        h1876
                    };
                    nat_add(a1877, a1878)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f307(v0: Bytes, v1: bool, v2: u64) -> R<(u64, u64)> {
    let (h1879, _) = f394(Fn7::F395(v1, v0.clone()), {
        let a1881 = {
            let a1880 = v0;
            length_bytes(a1880)
        };
        let a1882 = 1u64;
        nat_add(a1881, a1882)?
    }, (v2, 0u64))?;
    Ok(h1879)
}

pub fn f308(v0: u64) -> R<Bytes> {
    let a1883 = v0;
    let a1884 = 128u64;
    if nat_lt(a1883, a1884) {
        Ok(f396(f397(), v0))
    } else {
        let a1885 = v0;
        let a1886 = 2048u64;
        if nat_lt(a1885, a1886) {
            let a1894 = f396(f398(), {
                let a1892 = {
                    let a1890 = {
                        let a1887 = v0;
                        let a1888 = 64u64;
                        let a1889 = 0u64;
                        nat_quot(a1887, a1888, a1889)
                    };
                    let a1891 = 2u64;
                    nat_sub(a1890, a1891)
                };
                let a1893 = 2u64;
                nat_mul(a1892, a1893)?
            });
            let a1895 = f399(v0)?;
            Ok(append_bytes(a1894, a1895))
        } else {
            let a1896 = v0;
            let a1897 = 65536u64;
            if nat_lt(a1896, a1897) {
                let a1908 = f396(f400(), {
                    let a1901 = {
                        let a1898 = v0;
                        let a1899 = 4096u64;
                        let a1900 = 0u64;
                        nat_quot(a1898, a1899, a1900)
                    };
                    let a1902 = 3u64;
                    nat_mul(a1901, a1902)?
                });
                let a1909 = {
                    let a1906 = f399({
                        let a1903 = v0;
                        let a1904 = 64u64;
                        let a1905 = 0u64;
                        nat_quot(a1903, a1904, a1905)
                    })?;
                    let a1907 = f399(v0)?;
                    append_bytes(a1906, a1907)
                };
                Ok(append_bytes(a1908, a1909))
            } else {
                let a1923 = f396(Bytes::lit(&[0xf0, 0xf1, 0xf2, 0xf3, 0xf4]), {
                    let a1910 = v0;
                    let a1911 = 262144u64;
                    let a1912 = 0u64;
                    nat_quot(a1910, a1911, a1912)
                });
                let a1924 = {
                    let a1921 = f399({
                        let a1913 = v0;
                        let a1914 = 4096u64;
                        let a1915 = 0u64;
                        nat_quot(a1913, a1914, a1915)
                    })?;
                    let a1922 = {
                        let a1919 = f399({
                            let a1916 = v0;
                            let a1917 = 64u64;
                            let a1918 = 0u64;
                            nat_quot(a1916, a1917, a1918)
                        })?;
                        let a1920 = f399(v0)?;
                        append_bytes(a1919, a1920)
                    };
                    append_bytes(a1921, a1922)
                };
                Ok(append_bytes(a1923, a1924))
            }
        }
    }
}

pub fn f309(v0: bool, v1: u64, v2: Bytes, v3: u64) -> R<Option<u64>> {
    f401(v2, v0, v1, v3)
}

pub fn f310(v0: Bytes) -> Result<Option<Adt27>, Bytes> {
    Err::<Option<Adt27>, Bytes>(v0)
}

pub fn f311(p1928: Bytes, p1929: u64) -> R<Result<Bytes, Bytes>> {
    let m1927 = (p1928, p1929);
    {
        let mut m1927 = m1927;
        loop {
            let step = {
                let (v0, v1) = m1927;
                let m1925 = v1;
                if m1925 == 0 {
                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f402(f8(Str::lit("the built-in datatype hierarchy is deeper than XSD 1.1 Part 2 section 3 defines")))))
                } else {
                    let v2 = m1925 - 1;
                    if f208(v0.clone(), List::cons(f8(Str::lit("NMTOKENS")), List::cons(f8(Str::lit("IDREFS")), List::cons(f8(Str::lit("ENTITIES")), List::<Bytes>::nil())))) {
                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("(List Bytes)")))))
                    } else {
                        if f35(v0.clone(), f8(Str::lit("nonNegativeInteger"))) {
                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Nat")))))
                        } else {
                            if f35(v0.clone(), f8(Str::lit("integer"))) {
                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Int")))))
                            } else {
                                if f35(v0.clone(), f8(Str::lit("decimal"))) {
                                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f403())))
                                } else {
                                    if f35(v0.clone(), f8(Str::lit("boolean"))) {
                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Bool")))))
                                    } else {
                                        if f208(v0.clone(), List::cons(f8(Str::lit("float")), List::cons(f8(Str::lit("double")), List::<Bytes>::nil()))) {
                                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f402(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because its value space holds positive and negative zero, the infinities and NaN, which no exact LexLean number type holds (XSD 1.1 Part 2, sections 3.3.4 and 3.3.5)")), List::<Bytes>::nil())))))))
                                        } else {
                                            if f208(v0.clone(), List::cons(f8(Str::lit("anySimpleType")), List::cons(f8(Str::lit("anyAtomicType")), List::<Bytes>::nil()))) {
                                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f402(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because it is the union of every simple value space and has no one LexLean type (XSD 1.1 Part 2, sections 3.2.1 and 3.2.2)")), List::<Bytes>::nil())))))))
                                            } else {
                                                let m1926 = f404(v0.clone())?;
                                                match m1926 {
                                                    None => {
                                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f402(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not a built-in datatype of XSD 1.1 Part 2")), List::<Bytes>::nil())))))))
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
                Ok(next) => { m1927 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f312(v0: Bytes) -> Result<List<Adt19>, Bytes> {
    Err::<List<Adt19>, Bytes>(v0)
}

pub fn f313(v0: Option<Adt19>) -> Option<Bytes> {
    let m1930 = v0;
    match m1930 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m1931 = v1;
            match m1931 {
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

pub fn f314(v0: u64) -> bool {
    f317({
        let a1932 = v0;
        let a1933 = 95u64;
        nat_eq(a1932, a1933)
    }, f317({
        let a1934 = v0;
        let a1935 = 45u64;
        nat_eq(a1934, a1935)
    }, f317({
        let a1936 = v0;
        let a1937 = 32u64;
        nat_eq(a1936, a1937)
    }, f317({
        let a1938 = v0;
        let a1939 = 46u64;
        nat_eq(a1938, a1939)
    }, {
        let a1940 = v0;
        let a1941 = 47u64;
        nat_eq(a1940, a1941)
    }))))
}

pub fn f315(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f316(v0: u64) -> bool {
    f315({
        let a1942 = 97u64;
        let a1943 = v0;
        nat_le(a1942, a1943)
    }, {
        let a1944 = v0;
        let a1945 = 122u64;
        nat_le(a1944, a1945)
    })
}

pub fn f317(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f318(v0: u64) -> bool {
    f315({
        let a1946 = 65u64;
        let a1947 = v0;
        nat_le(a1946, a1947)
    }, {
        let a1948 = v0;
        let a1949 = 90u64;
        nat_le(a1948, a1949)
    })
}

pub fn f319(v0: u64) -> bool {
    f315({
        let a1950 = 48u64;
        let a1951 = v0;
        nat_le(a1950, a1951)
    }, {
        let a1952 = v0;
        let a1953 = 57u64;
        nat_le(a1952, a1953)
    })
}

pub fn f320(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h1954, _) = v1.clone();
        h1954
    };
    let v3: u64 = {
        let (h1955, _) = {
            let (_, h1956) = v1.clone();
            h1956
        };
        h1955
    };
    let v4: List<Bytes> = {
        let (_, h1957) = {
            let (_, h1958) = v1;
            h1958
        };
        h1957
    };
    let m1959 = f106(v0.clone(), v2);
    match m1959 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a1960 = v5;
            let a1961 = 10u8;
            if equal(a1960, a1961) {
                Ok(Some(({
                    let a1962 = v2;
                    let a1963 = 1u64;
                    nat_add(a1962, a1963)?
                }, ({
                    let a1964 = v2;
                    let a1965 = 1u64;
                    nat_add(a1964, a1965)?
                }, List::cons(f405(f32(v0, v3, {
                    let a1966 = v2;
                    let a1967 = v3;
                    nat_sub(a1966, a1967)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a1968 = v2;
                    let a1969 = 1u64;
                    nat_add(a1968, a1969)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f321(v0: List<Adt24>, v1: List<Adt24>) -> List<Adt24> {
    let m1970 = v0;
    match m1970.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f321(v3, v1))
        }
    }
}

pub fn f322(v0: List<Adt25>, v1: Bytes) -> List<Adt24> {
    let m1971 = v0;
    match m1971.uncons() {
        None => {
            List::<Adt24>::nil()
        }
        Some((v2, v3)) => {
            if f115(match v2.clone() {
                Adt25::C0(_, h1972, _, _, _) => {
                    h1972
                }
            }, f406()) {
                let m1973 = f238(v2.clone(), Str::lit("name"));
                match m1973 {
                    None => {
                        f322(v3, v1)
                    }
                    Some(v4) => {
                        List::cons(Adt24::C0(match v2.clone() {
                            Adt25::C0(_, h1974, _, _, _) => {
                                h1974
                            }
                        }, v1.clone(), v4, v2), f322(v3, v1))
                    }
                }
            } else {
                f322(v3, v1)
            }
        }
    }
}

pub fn f323(v0: u64, v1: Adt23, v2: Adt24) -> R<Result<List<Adt26>, Bytes>> {
    let v3: Bytes = f154(match v2.clone() {
        Adt24::C0(_, _, h1975, _) => {
            h1975
        }
    })?;
    let v4: Adt25 = match v2.clone() {
        Adt24::C0(_, _, _, h1976) => {
            h1976
        }
    };
    let v5: Bytes = match v2.clone() {
        Adt24::C0(_, h1977, _, _) => {
            h1977
        }
    };
    let v6: Bytes = match v2.clone() {
        Adt24::C0(h1978, _, _, _) => {
            h1978
        }
    };
    if f35(v6.clone(), f8(Str::lit("complexType"))) {
        f407(v0, v1, v2)
    } else {
        if f35(v6.clone(), f8(Str::lit("simpleType"))) {
            let m1979 = f408(v3, v4, v5)?;
            match m1979 {
                Ok(v8) => {
                    Ok(Ok::<List<Adt26>, Bytes>(List::cons(v8, List::<Adt26>::nil())))
                }
                Err(v7) => {
                    Ok(Err::<List<Adt26>, Bytes>(v7))
                }
            }
        } else {
            if f35(v6.clone(), f8(Str::lit("element"))) {
                let m1980 = f238(v4.clone(), Str::lit("type"));
                match m1980 {
                    None => {
                        let m1981 = f409(v0, v1, v3, v5, v4)?;
                        match m1981 {
                            Ok(v11) => {
                                Ok(Ok::<List<Adt26>, Bytes>(match v11 {
                                    Adt31::C0(_, h1982) => {
                                        h1982
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
                    let m1983 = f238(v4.clone(), Str::lit("type"));
                    match m1983 {
                        None => {
                            let m1984 = f410(v1, v5, v3, v4)?;
                            match m1984 {
                                Ok(v14) => {
                                    Ok(Ok::<List<Adt26>, Bytes>(match v14 {
                                        Adt31::C0(_, h1985) => {
                                            h1985
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
                        let m1986 = f411(v4)?;
                        match m1986 {
                            Ok(v16) => {
                                let m1987 = f412(v16, List::cons(f8(Str::lit("sequence")), List::cons(f8(Str::lit("choice")), List::cons(f8(Str::lit("all")), List::<Bytes>::nil()))));
                                match m1987 {
                                    None => {
                                        Ok(Err::<List<Adt26>, Bytes>(f413(f8(Str::lit("a model group definition must have a sequence, choice or all child (XSD 1.1 Part 1, section 3.7.2)")), v3)))
                                    }
                                    Some(v17) => {
                                        f414(v0, v1, v3, v5, v17)
                                    }
                                }
                            }
                            Err(v15) => {
                                Ok(Err::<List<Adt26>, Bytes>(v15))
                            }
                        }
                    } else {
                        let m1988 = f411(v4)?;
                        match m1988 {
                            Ok(v19) => {
                                let m1989 = f415(v0, v1, v3, v5, v19, f416())?;
                                match m1989 {
                                    Ok(v21) => {
                                        Ok(Ok::<List<Adt26>, Bytes>(match v21 {
                                            Adt32::C0(_, h1990, _, _, _) => {
                                                h1990
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

pub fn f324(v0: List<Adt26>, v1: List<Adt26>) -> List<Adt26> {
    let m1991 = v0;
    match m1991.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f324(v3, v1))
        }
    }
}

pub fn f325(p1994: List<Adt26>, p1995: List<Adt26>) -> List<Adt26> {
    let m1993 = (p1994, p1995);
    {
        let mut m1993 = m1993;
        loop {
            let step = {
                let (v0, v1) = m1993;
                let m1992 = v0;
                match m1992.uncons() {
                    None => {
                        Err::<(List<Adt26>, List<Adt26>), List<Adt26>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt26>, List<Adt26>), List<Adt26>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1993 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f326(p1999: List<Adt26>, v1: Bytes) -> Option<Adt26> {
    let m1998 = p1999;
    {
        let mut m1998 = m1998;
        loop {
            let step = {
                let v0 = m1998;
                let m1996 = v0;
                match m1996.uncons() {
                    None => {
                        Err::<List<Adt26>, Option<Adt26>>(None::<Adt26>)
                    }
                    Some((v2, v3)) => {
                        if f35(match v2.clone() {
                            Adt26::C0(h1997, _, _, _) => {
                                h1997
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
                Ok(next) => { m1998 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f327() -> Bytes {
    f8(Str::lit("XmlNode"))
}

pub fn f328(v0: List<Adt26>) -> List<Bytes> {
    let m2000 = v0;
    match m2000.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f334(match v1 {
                Adt26::C0(_, h2001, _, _) => {
                    h2001
                }
            }, f328(v2))
        }
    }
}

pub fn f329() -> List<Bytes> {
    List::cons(f8(Str::lit("    {name: element, fields: [Bytes, Bytes, (List ((Bytes * Bytes) * Bytes)), (List XmlNode)]}")), List::cons(f8(Str::lit("    {name: text, fields: [Bytes]}")), List::<Bytes>::nil()))
}

pub fn f330(p2005: u64, v1: List<Adt26>, p2006: List<Bytes>, p2007: List<Bytes>) -> List<Bytes> {
    let m2004 = (p2005, (p2006, p2007));
    {
        let mut m2004 = m2004;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2004;
                let m2002 = v0;
                if m2002 == 0 {
                    Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v3)
                } else {
                    let v4 = m2002 - 1;
                    let m2003 = v2;
                    match m2003.uncons() {
                        None => {
                            Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v3)
                        }
                        Some((v5, v6)) => {
                            if f115(v5.clone(), v3.clone()) {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v3)))
                            } else {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (f334(f331(v1.clone(), v5.clone()), v6), List::cons(v5, v3))))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2004 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f331(v0: List<Adt26>, v1: Bytes) -> List<Bytes> {
    let m2008 = f326(v0, v1);
    match m2008 {
        None => {
            List::<Bytes>::nil()
        }
        Some(v2) => {
            match v2 {
                Adt26::C0(_, h2009, _, _) => {
                    h2009
                }
            }
        }
    }
}

pub fn f332(v0: List<(List<Bytes>, List<Bytes>)>, v1: List<Bytes>) -> Option<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)> {
    let m2010 = v0;
    match m2010.uncons() {
        None => {
            None::<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)>
        }
        Some((v2, v3)) => {
            let m2012 = f155({
                let (_, h2011) = v2.clone();
                h2011
            }, v1.clone());
            match m2012 {
                None => {
                    Some(({
                        let (h2013, _) = v2;
                        h2013
                    }, v3))
                }
                Some(_) => {
                    let m2014 = f332(v3, v1);
                    match m2014 {
                        None => {
                            None::<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h2015, _) = v5;
                                h2015
                            }, List::cons(v2, {
                                let (_, h2016) = v5;
                                h2016
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f333(v0: List<Adt26>, v1: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m2017 = v1.clone();
    match m2017.uncons() {
        None => {
            Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m2018 = v3;
            match m2018.uncons() {
                None => {
                    let m2021 = f326(v0, v2.clone());
                    match m2021 {
                        None => {
                            Err::<List<Bytes>, Bytes>(f418(v2))
                        }
                        Some(v6) => {
                            if f115(v2.clone(), match v6.clone() {
                                Adt26::C0(_, h2022, _, _) => {
                                    h2022
                                }
                            }) {
                                let m2024 = match v6.clone() {
                                    Adt26::C0(_, _, _, h2023) => {
                                        h2023
                                    }
                                };
                                match m2024.uncons() {
                                    None => {
                                        Err::<List<Bytes>, Bytes>(f418(v2))
                                    }
                                    Some((_, _)) => {
                                        Ok::<List<Bytes>, Bytes>(List::cons(f194(v2, match v6 {
                                            Adt26::C0(_, _, _, h2025) => {
                                                h2025
                                            }
                                        }), List::<Bytes>::nil()))
                                    }
                                }
                            } else {
                                Ok::<List<Bytes>, Bytes>(List::cons(match v6 {
                                    Adt26::C0(_, _, h2026, _) => {
                                        h2026
                                    }
                                }, List::<Bytes>::nil()))
                            }
                        }
                    }
                }
                Some((_, _)) => {
                    f417(v0, v1, {
                        let a2019 = v2;
                        let a2020 = f8(Str::lit("Group"));
                        append_bytes(a2019, a2020)
                    })
                }
            }
        }
    }
}

pub fn f334(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m2027 = v0;
    match m2027.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f334(v3, v1))
        }
    }
}

pub fn f335(v0: List<Adt26>, v1: List<Bytes>, v2: List<Bytes>) -> List<Bytes> {
    let m2028 = v1;
    match m2028.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            f334(f419(f331(v0.clone(), v3), v2.clone()), f335(v0, v4, v2))
        }
    }
}

pub fn f336(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>, v2: Bytes) -> List<Bytes> {
    let m2029 = v1;
    match m2029.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if f35(v3.clone(), v2.clone()) {
                true
            } else {
                if f115(v3.clone(), f420(v0.clone(), v2.clone())) {
                    f115(v2.clone(), f420(v0.clone(), v3.clone()))
                } else {
                    false
                }
            } {
                List::cons(v3, f336(v0, v4, v2))
            } else {
                f336(v0, v4, v2)
            }
        }
    }
}

pub fn f337(v0: Bytes, p2038: u64, p2039: u64, p2040: Option<u64>) -> R<Option<u64>> {
    let m2037 = (p2038, (p2039, p2040));
    {
        let mut m2037 = m2037;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2037;
                let m2030 = v2;
                if m2030 == 0 {
                    Err::<(u64, (u64, Option<u64>)), R<Option<u64>>>(Ok(v3))
                } else {
                    let v4 = m2030 - 1;
                    let v5: u64 = f87(v0.clone(), v1)?;
                    let a2031 = v5;
                    let a2032 = 256u64;
                    if nat_eq(a2031, a2032) {
                        Err::<(u64, (u64, Option<u64>)), R<Option<u64>>>(Ok(v3))
                    } else {
                        Ok::<(u64, (u64, Option<u64>)), R<Option<u64>>>(({
                            let a2033 = v1;
                            let a2034 = 1u64;
                            nat_add(a2033, a2034)?
                        }, (v4, {
                            let a2035 = v5;
                            let a2036 = 47u64;
                            if nat_eq(a2035, a2036) {
                                Some(v1)
                            } else {
                                v3
                            }
                        })))
                    }
                }
            };
            match step {
                Ok(next) => { m2037 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f338(v0: u64) -> bool {
    let a2041 = v0;
    let a2042 = 9u64;
    if if if nat_eq(a2041, a2042) {
        true
    } else {
        let a2043 = v0;
        let a2044 = 10u64;
        nat_eq(a2043, a2044)
    } {
        true
    } else {
        let a2045 = v0;
        let a2046 = 13u64;
        if nat_eq(a2045, a2046) {
            true
        } else {
            let a2047 = 32u64;
            let a2048 = v0;
            if nat_le(a2047, a2048) {
                let a2049 = v0;
                let a2050 = 126u64;
                nat_le(a2049, a2050)
            } else {
                false
            }
        }
    } {
        true
    } else {
        let a2051 = v0;
        let a2052 = 133u64;
        if if nat_eq(a2051, a2052) {
            true
        } else {
            let a2053 = 160u64;
            let a2054 = v0;
            if nat_le(a2053, a2054) {
                let a2055 = v0;
                let a2056 = 55295u64;
                nat_le(a2055, a2056)
            } else {
                false
            }
        } {
            true
        } else {
            let a2057 = 57344u64;
            let a2058 = v0;
            if if nat_le(a2057, a2058) {
                let a2059 = v0;
                let a2060 = 65533u64;
                nat_le(a2059, a2060)
            } else {
                false
            } {
                true
            } else {
                let a2061 = 65536u64;
                let a2062 = v0;
                if nat_le(a2061, a2062) {
                    let a2063 = v0;
                    let a2064 = 1114111u64;
                    nat_le(a2063, a2064)
                } else {
                    false
                }
            }
        }
    }
}

pub fn f339(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f247(v0.clone(), v1)?;
    if f245(f174(v0.clone(), v2)?) {
        Ok(true)
    } else {
        let a2065 = f174(v0, v2)?;
        let a2066 = 35u64;
        Ok(nat_eq(a2065, a2066))
    }
}

pub fn f340(v0: Bytes, v1: u64) -> R<u64> {
    let (h2067, _) = f130(Fn4::F421(v0), {
        let a2068 = v1;
        let a2069 = 1u64;
        nat_add(a2068, a2069)?
    }, v1)?;
    Ok(h2067)
}

pub fn f341(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f422(v0, v1)
}

pub fn f342(v0: Bytes, v1: u64) -> R<u64> {
    let (h2070, _) = f130(Fn4::F423(v0.clone()), {
        let a2072 = {
            let a2071 = v0;
            length_bytes(a2071)
        };
        let a2073 = 1u64;
        nat_add(a2072, a2073)?
    }, v1)?;
    Ok(h2070)
}

pub fn f343(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f424(v0, v1)
}

pub fn f344(v0: u64, v1: bool) -> bool {
    f176(if if f244(v0) {
        true
    } else {
        f245(v0)
    } {
        true
    } else {
        if v1 {
            f425(v0)
        } else {
            false
        }
    })
}

pub fn f345(v0: Bytes, v1: u64, v2: u64) -> R<Adt20> {
    let v3: bool = {
        let a2074 = f174(v0.clone(), v1)?;
        let a2075 = 62u64;
        nat_eq(a2074, a2075)
    };
    let v4: u64 = f174(v0.clone(), {
        let a2076 = v1;
        let a2077 = 1u64;
        nat_add(a2076, a2077)?
    })?;
    let v5: u64 = f174(v0.clone(), {
        let a2078 = v1;
        let a2079 = 2u64;
        nat_add(a2078, a2079)?
    })?;
    let v6: u64 = f426(v4);
    let v7: u64 = f427(v4);
    let v8: bool = {
        let a2080 = 0u64;
        let a2081 = v6;
        if nat_lt(a2080, a2081) {
            true
        } else {
            let a2082 = 0u64;
            let a2083 = v7;
            nat_lt(a2082, a2083)
        }
    };
    let v9: u64 = {
        let a2084 = 0u64;
        let a2085 = v7;
        if nat_lt(a2084, a2085) {
            f426(v5)
        } else {
            0u64
        }
    };
    let v10: u64 = {
        let a2086 = 0u64;
        let a2087 = v6;
        if nat_lt(a2086, a2087) {
            f427(v5)
        } else {
            0u64
        }
    };
    let v11: u64 = {
        let a2088 = v6;
        let a2089 = v9;
        nat_add(a2088, a2089)?
    };
    let v12: u64 = {
        let a2090 = v7;
        let a2091 = v10;
        nat_add(a2090, a2091)?
    };
    let v13: u64 = {
        let a2100 = {
            let a2092 = v1;
            let a2093 = 1u64;
            nat_add(a2092, a2093)?
        };
        let a2101 = {
            let a2098 = if v8 {
                1u64
            } else {
                0u64
            };
            let a2099 = {
                let a2094 = 0u64;
                let a2095 = v9;
                if if nat_lt(a2094, a2095) {
                    true
                } else {
                    let a2096 = 0u64;
                    let a2097 = v10;
                    nat_lt(a2096, a2097)
                } {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a2098, a2099)?
        };
        nat_add(a2100, a2101)?
    };
    let m2102 = f173(v0.clone(), v13)?;
    match m2102 {
        Ok(v15) => {
            let a2103 = 0u64;
            let a2104 = v12;
            if nat_lt(a2103, a2104) {
                f428(v0, v15, {
                    let a2107 = {
                        let a2105 = v2;
                        let a2106 = v12;
                        nat_add(a2105, a2106)?
                    };
                    let a2108 = 1u64;
                    nat_sub(a2107, a2108)
                }, v3, v11)
            } else {
                let v16: (u64, u64) = {
                    let (h2109, _) = f394(Fn7::F429(v0.clone()), {
                        let a2111 = {
                            let a2110 = v0.clone();
                            length_bytes(a2110)
                        };
                        let a2112 = 1u64;
                        nat_add(a2111, a2112)?
                    }, (v15, 0u64))?;
                    h2109
                };
                let v17: u64 = f249(v0.clone(), {
                    let (h2113, _) = v16;
                    h2113
                })?;
                let a2114 = f174(v0.clone(), v17)?;
                let a2115 = 256u64;
                if nat_eq(a2114, a2115) {
                    Ok(Adt20::C0(Adt19::C5({
                        let a2116 = v11;
                        let a2117 = 2u64;
                        if nat_eq(a2116, a2117) {
                            f430(f431(v0.clone(), v15)?)
                        } else {
                            Bytes::lit(&[])
                        }
                    }), {
                        let a2118 = v0;
                        length_bytes(a2118)
                    }))
                } else {
                    let v18: u64 = f432({
                        let a2120 = v17;
                        let a2121 = {
                            let (h2119, _) = v16;
                            h2119
                        };
                        nat_sub(a2120, a2121)
                    }, v2);
                    let a2123 = v18;
                    let a2124 = {
                        let (_, h2122) = v16;
                        h2122
                    };
                    if nat_lt(a2123, a2124) {
                        Ok(Adt20::C1(f83(Str::lit("a leading empty line of a block scalar holds more spaces than its first content line (YAML 1.2.2 section 8.1.1.1)"), v15)?))
                    } else {
                        f428(v0, v15, v18, v3, v11)
                    }
                }
            }
        }
        Err(v14) => {
            Ok(Adt20::C1(v14))
        }
    }
}

pub fn f346(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Adt30> {
    let a2125 = f174(v0.clone(), v1)?;
    let a2126 = 34u64;
    if nat_eq(a2125, a2126) {
        f433(v0.clone(), {
            let a2127 = v1;
            let a2128 = 1u64;
            nat_add(a2127, a2128)?
        }, v2, v3, {
            let a2130 = {
                let a2129 = v0;
                length_bytes(a2129)
            };
            let a2131 = 2u64;
            nat_add(a2130, a2131)?
        }, Bytes::lit(&[]))
    } else {
        f434(v0.clone(), {
            let a2132 = v1;
            let a2133 = 1u64;
            nat_add(a2132, a2133)?
        }, v2, v3, {
            let a2135 = {
                let a2134 = v0;
                length_bytes(a2134)
            };
            let a2136 = 2u64;
            nat_add(a2135, a2136)?
        }, Bytes::lit(&[]))
    }
}

pub fn f347(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: List<Bytes>) -> R<Adt20> {
    let m2137 = v3;
    if m2137 == 0 {
        Ok(Adt20::C1(f83(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v6 = m2137 - 1;
        let a2138 = v4;
        let a2139 = 1u64;
        if nat_eq(a2138, a2139) {
            let m2140 = f347(v0.clone(), v1, v2, v6, 0u64, List::<Bytes>::nil())?;
            match m2140 {
                Adt20::C0(v8, v9) => {
                    let m2141 = f435(v0.clone(), v9, v2)?;
                    match m2141 {
                        Ok(v11) => {
                            let v12: u64 = f174(v0.clone(), v11)?;
                            let a2142 = v12;
                            let a2143 = 58u64;
                            if nat_eq(a2142, a2143) {
                                Ok(Adt20::C1(f83(Str::lit("a single-pair mapping inside a flow sequence is not read (YAML 1.2.2 [150])"), v11)?))
                            } else {
                                let a2144 = v12;
                                let a2145 = 93u64;
                                if nat_eq(a2144, a2145) {
                                    Ok(Adt20::C0(Adt19::C9(std::rc::Rc::new(v8), std::rc::Rc::new(Adt19::C8)), {
                                        let a2146 = v11;
                                        let a2147 = 1u64;
                                        nat_add(a2146, a2147)?
                                    }))
                                } else {
                                    let a2148 = v12;
                                    let a2149 = 44u64;
                                    if nat_eq(a2148, a2149) {
                                        let m2152 = f435(v0.clone(), {
                                            let a2150 = v11;
                                            let a2151 = 1u64;
                                            nat_add(a2150, a2151)?
                                        }, v2)?;
                                        match m2152 {
                                            Ok(v14) => {
                                                let a2153 = f174(v0.clone(), v14)?;
                                                let a2154 = 93u64;
                                                if nat_eq(a2153, a2154) {
                                                    Ok(Adt20::C0(Adt19::C9(std::rc::Rc::new(v8), std::rc::Rc::new(Adt19::C8)), {
                                                        let a2155 = v14;
                                                        let a2156 = 1u64;
                                                        nat_add(a2155, a2156)?
                                                    }))
                                                } else {
                                                    let m2157 = f347(v0, v14, v2, v6, 1u64, List::<Bytes>::nil())?;
                                                    match m2157 {
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
            let a2158 = v4;
            let a2159 = 2u64;
            if nat_eq(a2158, a2159) {
                let m2160 = f436(v0.clone(), v1, v2)?;
                match m2160 {
                    Adt30::C0(v19, v20) => {
                        let m2161 = f435(v0.clone(), v20, v2)?;
                        match m2161 {
                            Ok(v22) => {
                                let m2175 = {
                                    let a2162 = f174(v0.clone(), v22)?;
                                    let a2163 = 44u64;
                                    if if nat_eq(a2162, a2163) {
                                        true
                                    } else {
                                        let a2164 = f174(v0.clone(), v22)?;
                                        let a2165 = 125u64;
                                        nat_eq(a2164, a2165)
                                    } {
                                        Adt20::C0(Adt19::C0, v22)
                                    } else {
                                        if f176({
                                            let a2166 = f174(v0.clone(), v22)?;
                                            let a2167 = 58u64;
                                            nat_eq(a2166, a2167)
                                        }) {
                                            Adt20::C1(f83(Str::lit("a flow mapping entry must be a key, then : and its value, or a key alone (YAML 1.2.2 [142], [146])"), v22)?)
                                        } else {
                                            let m2170 = f435(v0.clone(), {
                                                let a2168 = v22;
                                                let a2169 = 1u64;
                                                nat_add(a2168, a2169)?
                                            }, v2)?;
                                            match m2170 {
                                                Ok(v24) => {
                                                    let a2171 = f174(v0.clone(), v24)?;
                                                    let a2172 = 44u64;
                                                    if if nat_eq(a2171, a2172) {
                                                        true
                                                    } else {
                                                        let a2173 = f174(v0.clone(), v24)?;
                                                        let a2174 = 125u64;
                                                        nat_eq(a2173, a2174)
                                                    } {
                                                        Adt20::C0(Adt19::C0, v24)
                                                    } else {
                                                        f347(v0.clone(), v24, v2, v6, 0u64, List::<Bytes>::nil())?
                                                    }
                                                }
                                                Err(v23) => {
                                                    Adt20::C1(v23)
                                                }
                                            }
                                        }
                                    }
                                };
                                match m2175 {
                                    Adt20::C0(v26, v27) => {
                                        if f208(v19.clone(), v5.clone()) {
                                            Ok(Adt20::C1(f83(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                        } else {
                                            let m2176 = f435(v0.clone(), v27, v2)?;
                                            match m2176 {
                                                Ok(v29) => {
                                                    let v30: u64 = f174(v0.clone(), v29)?;
                                                    let a2177 = v30;
                                                    let a2178 = 125u64;
                                                    if nat_eq(a2177, a2178) {
                                                        Ok(Adt20::C0(Adt19::C10(v19, std::rc::Rc::new(v26), std::rc::Rc::new(Adt19::C8)), {
                                                            let a2179 = v29;
                                                            let a2180 = 1u64;
                                                            nat_add(a2179, a2180)?
                                                        }))
                                                    } else {
                                                        let a2181 = v30;
                                                        let a2182 = 44u64;
                                                        if nat_eq(a2181, a2182) {
                                                            let m2185 = f435(v0.clone(), {
                                                                let a2183 = v29;
                                                                let a2184 = 1u64;
                                                                nat_add(a2183, a2184)?
                                                            }, v2)?;
                                                            match m2185 {
                                                                Ok(v32) => {
                                                                    let a2186 = f174(v0.clone(), v32)?;
                                                                    let a2187 = 125u64;
                                                                    if nat_eq(a2186, a2187) {
                                                                        Ok(Adt20::C0(Adt19::C10(v19, std::rc::Rc::new(v26), std::rc::Rc::new(Adt19::C8)), {
                                                                            let a2188 = v32;
                                                                            let a2189 = 1u64;
                                                                            nat_add(a2188, a2189)?
                                                                        }))
                                                                    } else {
                                                                        let m2190 = f347(v0, v32, v2, v6, 2u64, List::cons(v19.clone(), v5))?;
                                                                        match m2190 {
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
                let v36: u64 = f174(v0.clone(), v1)?;
                let a2191 = v36;
                let a2192 = 91u64;
                if if nat_eq(a2191, a2192) {
                    true
                } else {
                    let a2193 = v36;
                    let a2194 = 123u64;
                    nat_eq(a2193, a2194)
                } {
                    let m2197 = f435(v0.clone(), {
                        let a2195 = v1;
                        let a2196 = 1u64;
                        nat_add(a2195, a2196)?
                    }, v2)?;
                    match m2197 {
                        Ok(v38) => {
                            let a2200 = f174(v0.clone(), v38)?;
                            let a2201 = {
                                let a2198 = v36;
                                let a2199 = 2u64;
                                nat_add(a2198, a2199)?
                            };
                            if nat_eq(a2200, a2201) {
                                Ok(Adt20::C0({
                                    let a2202 = v36;
                                    let a2203 = 91u64;
                                    if nat_eq(a2202, a2203) {
                                        Adt19::C6(std::rc::Rc::new(Adt19::C8))
                                    } else {
                                        Adt19::C7(std::rc::Rc::new(Adt19::C8))
                                    }
                                }, {
                                    let a2204 = v38;
                                    let a2205 = 1u64;
                                    nat_add(a2204, a2205)?
                                }))
                            } else {
                                let m2208 = f347(v0, v38, v2, v6, {
                                    let a2206 = v36;
                                    let a2207 = 91u64;
                                    if nat_eq(a2206, a2207) {
                                        1u64
                                    } else {
                                        2u64
                                    }
                                }, List::<Bytes>::nil())?;
                                match m2208 {
                                    Adt20::C0(v40, v41) => {
                                        Ok(Adt20::C0({
                                            let a2209 = v36;
                                            let a2210 = 91u64;
                                            if nat_eq(a2209, a2210) {
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
                    let a2211 = v36;
                    let a2212 = 34u64;
                    if if nat_eq(a2211, a2212) {
                        true
                    } else {
                        let a2213 = v36;
                        let a2214 = 39u64;
                        nat_eq(a2213, a2214)
                    } {
                        let m2215 = f346(v0, v1, v2, false)?;
                        match m2215 {
                            Adt30::C0(v43, v44) => {
                                Ok(Adt20::C0(Adt19::C5(v43), v44))
                            }
                            Adt30::C1(v42) => {
                                Ok(Adt20::C1(v42))
                            }
                        }
                    } else {
                        let m2216 = f252(v0.clone(), v1)?;
                        match m2216 {
                            None => {
                                if f176(f348(v0.clone(), v1, true)?) {
                                    Ok(Adt20::C1(f83(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                                } else {
                                    let v46: u64 = f349(v0.clone(), v1, true)?;
                                    let v47: u64 = f247(v0.clone(), v46)?;
                                    let m2219 = {
                                        let a2217 = f174(v0.clone(), v47)?;
                                        let a2218 = 10u64;
                                        if nat_eq(a2217, a2218) {
                                            f435(v0.clone(), v47, v2)?
                                        } else {
                                            Ok::<u64, Bytes>(v47)
                                        }
                                    };
                                    match m2219 {
                                        Ok(v49) => {
                                            let a2220 = f174(v0.clone(), v47)?;
                                            let a2221 = 10u64;
                                            if if if nat_eq(a2220, a2221) {
                                                f176(f425(f174(v0.clone(), v49)?))
                                            } else {
                                                false
                                            } {
                                                f176({
                                                    let a2222 = f174(v0.clone(), v49)?;
                                                    let a2223 = 58u64;
                                                    nat_eq(a2222, a2223)
                                                })
                                            } else {
                                                false
                                            } {
                                                Ok(Adt20::C1(f83(Str::lit("a multi-line plain scalar inside a flow collection is not read (YAML 1.2.2 [135])"), v47)?))
                                            } else {
                                                f351(f256(v0, v1, v46), v1, v46)
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

pub fn f348(v0: Bytes, v1: u64, v2: bool) -> R<bool> {
    let v3: u64 = f174(v0.clone(), v1)?;
    if if f244(v3) {
        true
    } else {
        f245(v3)
    } {
        Ok(false)
    } else {
        let a2224 = v3;
        let a2225 = 45u64;
        if if if nat_eq(a2224, a2225) {
            true
        } else {
            let a2226 = v3;
            let a2227 = 63u64;
            nat_eq(a2226, a2227)
        } {
            true
        } else {
            let a2228 = v3;
            let a2229 = 58u64;
            nat_eq(a2228, a2229)
        } {
            Ok(f344(f174(v0, {
                let a2230 = v1;
                let a2231 = 1u64;
                nat_add(a2230, a2231)?
            })?, v2))
        } else {
            Ok(f176(f437(v3)?))
        }
    }
}

pub fn f349(v0: Bytes, v1: u64, v2: bool) -> R<u64> {
    let (_, h2232) = {
        let (h2233, _) = f394(Fn7::F438(v2, v0.clone()), {
            let a2235 = {
                let a2234 = v0;
                length_bytes(a2234)
            };
            let a2236 = 1u64;
            nat_add(a2235, a2236)?
        }, (v1, v1))?;
        h2233
    };
    Ok(h2232)
}

pub fn f350(v0: Bytes, p2261: u64, v2: u64, p2262: u64, p2263: Bytes) -> R<Adt30> {
    let m2260 = (p2261, (p2262, p2263));
    {
        let mut m2260 = m2260;
        loop {
            let step = {
                let (v1, (v3, v4)) = m2260;
                let m2237 = v3;
                if m2237 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f83(Str::lit("a plain scalar has more lines than the input"), v1)?)))
                } else {
                    let v5 = m2237 - 1;
                    let v6: u64 = f247(v0.clone(), v1)?;
                    let v7: u64 = f174(v0.clone(), v6)?;
                    let a2238 = v7;
                    let a2239 = 58u64;
                    if nat_eq(a2238, a2239) {
                        Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f439(v6)?)))
                    } else {
                        let a2240 = v7;
                        let a2241 = 35u64;
                        if if nat_eq(a2240, a2241) {
                            true
                        } else {
                            let a2242 = v7;
                            let a2243 = 256u64;
                            nat_eq(a2242, a2243)
                        } {
                            Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C0(v4, f248(v0.clone(), v6)?)))
                        } else {
                            let v8: (u64, u64) = f440(v0.clone(), {
                                let a2244 = v6;
                                let a2245 = 1u64;
                                nat_add(a2244, a2245)?
                            })?;
                            let v9: u64 = {
                                let (h2246, _) = v8;
                                h2246
                            };
                            let v10: u64 = f249(v0.clone(), v9)?;
                            let v11: u64 = f247(v0.clone(), v10)?;
                            let v12: u64 = f174(v0.clone(), v11)?;
                            let a2247 = v12;
                            let a2248 = 256u64;
                            if if if if nat_eq(a2247, a2248) {
                                true
                            } else {
                                let a2251 = {
                                    let a2249 = v10;
                                    let a2250 = v9;
                                    nat_sub(a2249, a2250)
                                };
                                let a2252 = v2;
                                nat_lt(a2251, a2252)
                            } {
                                true
                            } else {
                                let a2253 = v12;
                                let a2254 = 35u64;
                                nat_eq(a2253, a2254)
                            } {
                                true
                            } else {
                                f172(v0.clone(), v9)?
                            } {
                                Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C0(v4, {
                                    let a2255 = v6;
                                    let a2256 = 1u64;
                                    nat_add(a2255, a2256)?
                                })))
                            } else {
                                let v13: u64 = f349(v0.clone(), v11, false)?;
                                let a2257 = v13;
                                let a2258 = v11;
                                if nat_eq(a2257, a2258) {
                                    Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f439(v11)?)))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Adt30>>((v13, (v5, f7(List::cons(v4, List::cons(f441({
                                        let (_, h2259) = v8;
                                        h2259
                                    }), List::cons(f256(v0.clone(), v11, v13), List::<Bytes>::nil())))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2260 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f351(v0: Bytes, v1: u64, v2: u64) -> R<Adt20> {
    let a2265 = {
        let a2264 = v0.clone();
        length_bytes(a2264)
    };
    let a2266 = 0u64;
    if if nat_eq(a2265, a2266) {
        true
    } else {
        f208(v0.clone(), List::cons(f8(Str::lit("~")), List::cons(f8(Str::lit("null")), List::cons(f8(Str::lit("Null")), List::cons(f8(Str::lit("NULL")), List::<Bytes>::nil())))))
    } {
        Ok(Adt20::C0(Adt19::C0, v2))
    } else {
        if f208(v0.clone(), List::cons(f8(Str::lit("true")), List::cons(f8(Str::lit("True")), List::cons(f8(Str::lit("TRUE")), List::<Bytes>::nil())))) {
            Ok(Adt20::C0(Adt19::C2(true), v2))
        } else {
            if f208(v0.clone(), List::cons(f8(Str::lit("false")), List::cons(f8(Str::lit("False")), List::cons(f8(Str::lit("FALSE")), List::<Bytes>::nil())))) {
                Ok(Adt20::C0(Adt19::C2(false), v2))
            } else {
                if f208(v0.clone(), List::cons(f8(Str::lit(".inf")), List::cons(f8(Str::lit(".Inf")), List::cons(f8(Str::lit(".INF")), List::cons(f8(Str::lit("+.inf")), List::cons(f8(Str::lit("+.Inf")), List::cons(f8(Str::lit("+.INF")), List::cons(f8(Str::lit("-.inf")), List::cons(f8(Str::lit("-.Inf")), List::cons(f8(Str::lit("-.INF")), List::cons(f8(Str::lit(".nan")), List::cons(f8(Str::lit(".NaN")), List::cons(f8(Str::lit(".NAN")), List::<Bytes>::nil()))))))))))))) {
                    Ok(Adt20::C1(f83(Str::lit("an infinite or not-a-number float is not read: Studio.Value has none (YAML 1.2.2 section 10.3.2)"), v1)?))
                } else {
                    if if f86(v0.clone(), 0u64, Str::lit("0o")) {
                        let a2268 = 2u64;
                        let a2269 = {
                            let a2267 = v0.clone();
                            length_bytes(a2267)
                        };
                        nat_lt(a2268, a2269)
                    } else {
                        false
                    } {
                        f442(v0, 8u64, v1, v2)
                    } else {
                        if if f86(v0.clone(), 0u64, Str::lit("0x")) {
                            let a2271 = 2u64;
                            let a2272 = {
                                let a2270 = v0.clone();
                                length_bytes(a2270)
                            };
                            nat_lt(a2271, a2272)
                        } else {
                            false
                        } {
                            f442(v0, 16u64, v1, v2)
                        } else {
                            let m2273 = f443(v0.clone())?;
                            match m2273 {
                                None => {
                                    Ok(Adt20::C0(Adt19::C5(v0), v2))
                                }
                                Some(v3) => {
                                    let m2274 = f444(v3.clone(), 0u64)?;
                                    match m2274 {
                                        None => {
                                            Ok(Adt20::C1(f445(v1)?))
                                        }
                                        Some(v4) => {
                                            let a2277 = {
                                                let (h2275, _) = v4;
                                                h2275
                                            };
                                            let a2278 = {
                                                let a2276 = v3;
                                                length_bytes(a2276)
                                            };
                                            if nat_eq(a2277, a2278) {
                                                Ok(Adt20::C0({
                                                    let (_, h2279) = v4;
                                                    h2279
                                                }, v2))
                                            } else {
                                                Ok(Adt20::C1(f445(v1)?))
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

pub fn f352(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f446(v0, v1, v2)
}

pub fn f353(v0: Bytes, v1: u64) -> R<bool> {
    let a2280 = f174(v0.clone(), v1)?;
    let a2281 = 58u64;
    if nat_eq(a2280, a2281) {
        if f244(f174(v0.clone(), {
            let a2282 = v1;
            let a2283 = 1u64;
            nat_add(a2282, a2283)?
        })?) {
            Ok(true)
        } else {
            Ok(f245(f174(v0, {
                let a2284 = v1;
                let a2285 = 1u64;
                nat_add(a2284, a2285)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f354(v0: u64) -> Bytes {
    let m2286 = v0;
    if m2286 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2286 - 1;
        let a2287 = f8(Str::lit("0"));
        let a2288 = f354(v1);
        append_bytes(a2287, a2288)
    }
}

pub fn f355(v0: Bytes, p2301: u64, p2302: u64) -> R<u64> {
    let m2300 = (p2301, p2302);
    {
        let mut m2300 = m2300;
        loop {
            let step = {
                let (v1, v2) = m2300;
                let m2289 = v2;
                if m2289 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m2289 - 1;
                    let v4: u64 = f139(v0.clone(), v1)?;
                    let a2290 = v4;
                    let a2291 = 34u64;
                    if nat_eq(a2290, a2291) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a2292 = v4;
                        let a2293 = 92u64;
                        if nat_eq(a2292, a2293) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a2294 = v4;
                            let a2295 = 32u64;
                            if nat_lt(a2294, a2295) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a2296 = v4;
                                let a2297 = 256u64;
                                if nat_eq(a2296, a2297) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a2298 = v1;
                                        let a2299 = 1u64;
                                        nat_add(a2298, a2299)?
                                    }, v3))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2300 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f356(v0: Bytes) -> R<Bytes> {
    let a2308 = {
        let a2306 = Bytes::lit(&[0x22]);
        let a2307 = {
            let (_, h2303) = {
                let (h2304, _) = f127(Fn3::F447(v0.clone()), {
                    let a2305 = v0;
                    length_bytes(a2305)
                }, (0u64, Bytes::lit(&[])))?;
                h2304
            };
            h2303
        };
        append_bytes(a2306, a2307)
    };
    let a2309 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a2308, a2309))
}

pub fn f357(v0: Fn5, p2313: List<(Bytes, Bytes)>, p2314: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m2312 = (p2313, p2314);
    {
        let mut m2312 = m2312;
        loop {
            let step = {
                let (v1, v2) = m2312;
                let m2310 = v2;
                match m2310.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c2311: Fn5 = v0.clone();
                            c2311.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2312 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f358(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f448(v0, v1)
}

pub fn f359(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f449(v0)
}

pub fn f360(v0: Fn6, p2318: List<Bytes>, p2319: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m2317 = (p2318, p2319);
    {
        let mut m2317 = m2317;
        loop {
            let step = {
                let (v1, v2) = m2317;
                let m2315 = v2;
                match m2315.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c2316: Fn6 = v0.clone();
                            c2316.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2317 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f361(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f450(v1)?, v0))
}

pub fn f362(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f451(v1, v0, v2)
}

pub fn f363(v0: u64) -> bool {
    if f364(v0) {
        true
    } else {
        let a2320 = 65u64;
        let a2321 = v0;
        if if nat_le(a2320, a2321) {
            let a2322 = v0;
            let a2323 = 70u64;
            nat_le(a2322, a2323)
        } else {
            false
        } {
            true
        } else {
            let a2324 = 97u64;
            let a2325 = v0;
            if nat_le(a2324, a2325) {
                let a2326 = v0;
                let a2327 = 102u64;
                nat_le(a2326, a2327)
            } else {
                false
            }
        }
    }
}

pub fn f364(v0: u64) -> bool {
    let a2328 = 48u64;
    let a2329 = v0;
    if nat_le(a2328, a2329) {
        let a2330 = v0;
        let a2331 = 57u64;
        nat_le(a2330, a2331)
    } else {
        false
    }
}

pub fn f365(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f87(v0.clone(), v1)?;
    let a2332 = v2;
    let a2333 = 69u64;
    if if nat_eq(a2332, a2333) {
        true
    } else {
        let a2334 = v2;
        let a2335 = 101u64;
        nat_eq(a2334, a2335)
    } {
        let v3: u64 = f87(v0.clone(), {
            let a2336 = v1;
            let a2337 = 1u64;
            nat_add(a2336, a2337)?
        })?;
        let v4: u64 = {
            let a2338 = v3;
            let a2339 = 43u64;
            if if nat_eq(a2338, a2339) {
                true
            } else {
                let a2340 = v3;
                let a2341 = 45u64;
                nat_eq(a2340, a2341)
            } {
                let a2342 = v1;
                let a2343 = 2u64;
                nat_add(a2342, a2343)?
            } else {
                let a2344 = v1;
                let a2345 = 1u64;
                nat_add(a2344, a2345)?
            }
        };
        let v5: u64 = f269(v0, 0u64, v4)?;
        let a2346 = v4;
        let a2347 = v5;
        if nat_lt(a2346, a2347) {
            Ok(v5)
        } else {
            Ok(v1)
        }
    } else {
        Ok(v1)
    }
}

pub fn f366(v0: Adt21, v1: Bytes) -> bool {
    let a2348 = f367(v0.clone());
    let a2349 = 3u64;
    if !nat_eq(a2348, a2349) {
        f35(f368(v0), v1)
    } else {
        false
    }
}

pub fn f367(v0: Adt21) -> u64 {
    let m2350 = v0;
    match m2350 {
        Adt21::C0(v1, _) => {
            v1
        }
    }
}

pub fn f368(v0: Adt21) -> Bytes {
    let m2351 = v0;
    match m2351 {
        Adt21::C0(_, v2) => {
            v2
        }
    }
}

pub fn f369(v0: Bytes) -> Bytes {
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

pub fn f370(v0: Bytes) -> bool {
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

pub fn f371(p2360: u64, p2361: List<Adt21>, p2362: List<Adt13>) -> R<Result<(List<Adt13>, List<Adt21>), Bytes>> {
    let m2359 = (p2360, (p2361, p2362));
    {
        let mut m2359 = m2359;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2359;
                let m2352 = v0;
                if m2352 == 0 {
                    Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt21>), Bytes>(f8(Str::lit("a dictionary has more members than the input has tokens")))))
                } else {
                    let v3 = m2352 - 1;
                    if f272(v1.clone(), Str::lit("}")) {
                        let m2353 = f279(f274(v1), Str::lit(";"));
                        match m2353 {
                            Ok(v5) => {
                                Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>(Ok(Ok::<(List<Adt13>, List<Adt21>), Bytes>((f452(v2, List::<Adt13>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt21>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m2354 = f180(v1)?;
                        match m2354 {
                            Ok(v7) => {
                                let v8: bool = f272(v7.clone(), Str::lit("required"));
                                let m2355 = f284(v3, if v8 {
                                    f274(v7)
                                } else {
                                    v7
                                }, 0u64)?;
                                match m2355 {
                                    Ok(v10) => {
                                        let m2356 = f278(f285(v10.clone()));
                                        match m2356 {
                                            Ok(v12) => {
                                                if if f272(f280(v12.clone()), Str::lit("=")) {
                                                    !v8
                                                } else {
                                                    false
                                                } {
                                                    let m2357 = f373(f274(f280(v12.clone())))?;
                                                    match m2357 {
                                                        Ok(v14) => {
                                                            Ok::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>((v3, (v14, List::cons(Adt13::C0(f282(v12), f380(f286(v10)), 0u64), v2))))
                                                        }
                                                        Err(v13) => {
                                                            Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt21>), Bytes>(v13)))
                                                        }
                                                    }
                                                } else {
                                                    let m2358 = f279(f280(v12.clone()), Str::lit(";"));
                                                    match m2358 {
                                                        Ok(v16) => {
                                                            Ok::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>((v3, (v16, List::cons(Adt13::C0(f282(v12), f380(f286(v10)), if v8 {
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
                Ok(next) => { m2359 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f372(p2371: u64, p2372: List<Adt21>, p2373: List<Adt13>) -> R<Result<(List<Adt13>, List<Adt21>), Bytes>> {
    let m2370 = (p2371, (p2372, p2373));
    {
        let mut m2370 = m2370;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2370;
                let m2363 = v0;
                if m2363 == 0 {
                    Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt21>), Bytes>(f8(Str::lit("an interface has more members than the input has tokens")))))
                } else {
                    let v3 = m2363 - 1;
                    if f272(v1.clone(), Str::lit("}")) {
                        let m2364 = f279(f274(v1), Str::lit(";"));
                        match m2364 {
                            Ok(v5) => {
                                Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>(Ok(Ok::<(List<Adt13>, List<Adt21>), Bytes>((f452(v2, List::<Adt13>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt21>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m2365 = f180(v1)?;
                        match m2365 {
                            Ok(v7) => {
                                let v8: List<Adt21> = if if f272(v7.clone(), Str::lit("stringifier")) {
                                    true
                                } else {
                                    f272(v7.clone(), Str::lit("inherit"))
                                } {
                                    f274(v7.clone())
                                } else {
                                    v7.clone()
                                };
                                let v9: List<Adt21> = if f272(v8.clone(), Str::lit("readonly")) {
                                    f274(v8)
                                } else {
                                    v8
                                };
                                if f272(v9.clone(), Str::lit("attribute")) {
                                    let m2366 = f284(v3, f274(v9), 0u64)?;
                                    match m2366 {
                                        Ok(v11) => {
                                            let m2367 = f278(f285(v11.clone()));
                                            match m2367 {
                                                Ok(v13) => {
                                                    let m2368 = f279(f280(v13.clone()), Str::lit(";"));
                                                    match m2368 {
                                                        Ok(v15) => {
                                                            Ok::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>((v3, (v15, List::cons(Adt13::C0(f282(v13), f380(f286(v11)), 0u64), v2))))
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
                                    let m2369 = f373(v7)?;
                                    match m2369 {
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
                Ok(next) => { m2370 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f373(v0: List<Adt21>) -> R<Result<List<Adt21>, Bytes>> {
    Ok(f273({
        let a2375 = {
            let a2374 = v0.clone();
            length_list(a2374)
        };
        let a2376 = 1u64;
        nat_add(a2375, a2376)?
    }, v0, List::<Bytes>::nil(), true))
}

pub fn f374(v0: List<Adt21>) -> Bytes {
    let m2377 = v0;
    match m2377.uncons() {
        None => {
            f8(Str::lit("the end of the input"))
        }
        Some((v1, _)) => {
            f7(List::cons(f8(Str::lit("'")), List::cons(f368(v1), List::cons(f8(Str::lit("'")), List::<Bytes>::nil()))))
        }
    }
}

pub fn f375(v0: Bytes) -> Bytes {
    if f147(v0.clone(), f8(Str::lit("_"))) {
        f32(v0.clone(), 1u64, {
            let a2379 = {
                let a2378 = v0;
                length_bytes(a2378)
            };
            let a2380 = 1u64;
            nat_sub(a2379, a2380)
        })
    } else {
        v0
    }
}

pub fn f376(v0: Adt14, v1: List<Adt21>) -> Result<Adt29, Bytes> {
    if f272(v1.clone(), Str::lit("?")) {
        Ok::<Adt29, Bytes>(Adt29::C0(f377(Str::lit("Option"), v0), f274(v1)))
    } else {
        Ok::<Adt29, Bytes>(Adt29::C0(v0, v1))
    }
}

pub fn f377(v0: Str, v1: Adt14) -> Adt14 {
    let v2: Adt14 = f380(v1);
    Adt14::C0(0u64, f7(List::cons(f8(Str::lit("(")), List::cons({
        let a2381 = v0;
        utf8_encode(a2381)
    }, List::cons(f8(Str::lit(" ")), List::cons(f198(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), f199(v2.clone()), f196(v2), Bytes::lit(&[]))
}

pub fn f378(v0: Bytes) -> Adt14 {
    Adt14::C0(0u64, Bytes::lit(&[]), List::<Bytes>::nil(), v0, Bytes::lit(&[]))
}

pub fn f379(v0: Bytes) -> Option<Adt14> {
    f453(v0, f454())
}

pub fn f380(v0: Adt14) -> Adt14 {
    let a2382 = f197(v0.clone());
    let a2383 = 0u64;
    if nat_eq(a2382, a2383) {
        v0
    } else {
        Adt14::C0(0u64, Bytes::lit(&[]), f199(v0.clone()), if f35(f196(v0.clone()), Bytes::lit(&[])) {
            f455()
        } else {
            f196(v0)
        }, Bytes::lit(&[]))
    }
}

pub fn f381(v0: Adt14) -> Bytes {
    let m2384 = v0;
    match m2384 {
        Adt14::C0(_, _, _, _, v5) => {
            v5
        }
    }
}

pub fn f382(p2401: u64, p2402: u64, p2403: u64, p2404: Bytes) -> Bytes {
    let m2400 = (p2401, (p2402, (p2403, p2404)));
    {
        let mut m2400 = m2400;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m2400;
                let m2385 = v0;
                if m2385 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m2385 - 1;
                    if f315({
                        let a2386 = v1;
                        let a2387 = 0u64;
                        nat_eq(a2386, a2387)
                    }, {
                        let a2388 = v2;
                        let a2389 = 0u64;
                        nat_eq(a2388, a2389)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a2390 = v1;
                            let a2391 = 16u64;
                            let a2392 = 0u64;
                            nat_quot(a2390, a2391, a2392)
                        }, ({
                            let a2393 = v2;
                            let a2394 = 1u64;
                            nat_sub(a2393, a2394)
                        }, {
                            let a2398 = f456({
                                let a2395 = v1;
                                let a2396 = 16u64;
                                let a2397 = 0u64;
                                nat_rem(a2395, a2396, a2397)
                            });
                            let a2399 = v3;
                            append_bytes(a2398, a2399)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m2400 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f383(v0: Adt13, v1: List<Adt13>) -> R<List<Adt13>> {
    let m2405 = v1;
    match m2405.uncons() {
        None => {
            Ok(List::cons(v0, List::<Adt13>::nil()))
        }
        Some((v2, v3)) => {
            if f457({
                let a2407 = {
                    let a2406 = f295(v0.clone());
                    length_bytes(a2406)
                };
                let a2408 = 1u64;
                nat_add(a2407, a2408)?
            }, f295(v0.clone()), f295(v2.clone()), 0u64)? {
                Ok(List::cons(v0, List::cons(v2, v3)))
            } else {
                Ok(List::cons(v2, f383(v0, v3)?))
            }
        }
    }
}

pub fn f384(v0: List<Adt12>, v1: Bytes) -> List<Adt13> {
    let m2409 = f123(v0.clone(), 1u64, v1.clone());
    match m2409 {
        None => {
            f292(v0, 6u64, v1)
        }
        Some(v2) => {
            f290(f291(v2), f292(v0, 6u64, v1))
        }
    }
}

pub fn f385(v0: Bytes) -> R<Option<Bytes>> {
    let m2411 = f458({
        let a2410 = v0.clone();
        length_bytes(a2410)
    }, v0.clone(), 0u64, Bytes::lit(&[]))?;
    match m2411 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a2413 = {
                let a2412 = v1.clone();
                length_bytes(a2412)
            };
            let a2414 = 0u64;
            if nat_eq(a2413, a2414) {
                let a2416 = {
                    let a2415 = v0;
                    length_bytes(a2415)
                };
                let a2417 = 0u64;
                if nat_eq(a2416, a2417) {
                    Ok(Some(f8(Str::lit("u"))))
                } else {
                    Ok(None::<Bytes>)
                }
            } else {
                Ok(Some(f387(v1)?))
            }
        }
    }
}

pub fn f386(v0: Bytes) -> R<Option<Bytes>> {
    if f459({
        let a2418 = v0.clone();
        length_bytes(a2418)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f460(v0)?;
        if f461(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m2421 = {
                let a2419 = v1.clone();
                let a2420 = 0u64;
                index_bytes(a2419, a2420)
            };
            match m2421 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f319(f42(v2)?) {
                        Ok(Some({
                            let a2422 = f8(Str::lit("n"));
                            let a2423 = v1;
                            append_bytes(a2422, a2423)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f462({
            let a2424 = v0.clone();
            length_bytes(a2424)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f387(v0: Bytes) -> R<Bytes> {
    if f115(v0.clone(), f463()) {
        f288(v0)
    } else {
        Ok(v0)
    }
}

pub fn f388(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2427 = {
        let a2425 = v0;
        let a2426 = v1;
        index_bytes(a2425, a2426)
    };
    match m2427 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f42(v2)?;
            if f315({
                let a2428 = 128u64;
                let a2429 = v3;
                nat_le(a2428, a2429)
            }, {
                let a2430 = v3;
                let a2431 = 191u64;
                nat_le(a2430, a2431)
            }) {
                Ok(Some({
                    let a2432 = v3;
                    let a2433 = 128u64;
                    nat_sub(a2432, a2433)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f389(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f315({
        let a2434 = v1;
        let a2435 = v0;
        nat_le(a2434, a2435)
    }, f315({
        let a2436 = v0;
        let a2437 = v2;
        nat_le(a2436, a2437)
    }, f317({
        let a2438 = v0;
        let a2439 = 55296u64;
        nat_lt(a2438, a2439)
    }, {
        let a2440 = 57343u64;
        let a2441 = v0;
        nat_lt(a2440, a2441)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f390() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{3}/\u{7f}\u{3}@\u{0}\u{3}\u{7f}}\u{4}\u{0}\u{0}C\u{7f}\u{7f}"))
}

pub fn f391(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a2442 = v1;
        let a2443 = 3u64;
        nat_mul(a2442, a2443)?
    };
    let a2454 = {
        let a2450 = {
            let a2444 = f139(v0.clone(), v2)?;
            let a2445 = 16384u64;
            nat_mul(a2444, a2445)?
        };
        let a2451 = {
            let a2448 = f139(v0.clone(), {
                let a2446 = v2;
                let a2447 = 1u64;
                nat_add(a2446, a2447)?
            })?;
            let a2449 = 128u64;
            nat_mul(a2448, a2449)?
        };
        nat_add(a2450, a2451)?
    };
    let a2455 = f139(v0, {
        let a2452 = v2;
        let a2453 = 2u64;
        nat_add(a2452, a2453)?
    })?;
    nat_add(a2454, a2455)
}

pub fn f392(v0: u64) -> R<bool> {
    f204(v0, f464())
}

pub fn f393(v0: u64) -> R<bool> {
    f204(v0, f465())
}

pub fn f394(v0: Fn7, p2460: u64, p2461: (u64, u64)) -> R<((u64, u64), bool)> {
    let m2459 = (p2460, p2461);
    {
        let mut m2459 = m2459;
        loop {
            let step = {
                let (v1, v2) = m2459;
                let m2456 = v1;
                if m2456 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2456 - 1;
                    let m2458 = {
                        let c2457: Fn7 = v0.clone();
                        c2457.apply(v2)?
                    };
                    match m2458 {
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
                Ok(next) => { m2459 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f395(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f466(v1, v0, v2)
}

pub fn f396(v0: Bytes, v1: u64) -> Bytes {
    f32(v0, v1, 1u64)
}

pub fn f397() -> Bytes {
    f8(Str::lit("\u{0}\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\u{9}\u{a}\u{b}\u{c}\u{d}\u{e}\u{f}\u{10}\u{11}\u{12}\u{13}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{1b}\u{1c}\u{1d}\u{1e}\u{1f} !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~\u{7f}"))
}

pub fn f398() -> Bytes {
    f8(Str::lit("\u{80}\u{c0}\u{100}\u{140}\u{180}\u{1c0}\u{200}\u{240}\u{280}\u{2c0}\u{300}\u{340}\u{380}\u{3c0}\u{400}\u{440}\u{480}\u{4c0}\u{500}\u{540}\u{580}\u{5c0}\u{600}\u{640}\u{680}\u{6c0}\u{700}\u{740}\u{780}\u{7c0}"))
}

pub fn f399(v0: u64) -> R<Bytes> {
    Ok(f396(f467(), {
        let a2467 = {
            let a2465 = {
                let a2462 = v0;
                let a2463 = 64u64;
                let a2464 = 0u64;
                nat_rem(a2462, a2463, a2464)
            };
            let a2466 = 2u64;
            nat_mul(a2465, a2466)?
        };
        let a2468 = 1u64;
        nat_add(a2467, a2468)?
    }))
}

pub fn f400() -> Bytes {
    f8(Str::lit("\u{800}\u{1000}\u{2000}\u{3000}\u{4000}\u{5000}\u{6000}\u{7000}\u{8000}\u{9000}\u{a000}\u{b000}\u{c000}\u{d000}\u{e000}\u{f000}"))
}

pub fn f401(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<Option<u64>> {
    if f468(f87(v0, v3)?, v1, v2)? {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a2469 = v3;
            let a2470 = 1u64;
            nat_add(a2469, a2470)?
        }))
    }
}

pub fn f402(v0: Bytes) -> Result<Bytes, Bytes> {
    Err::<Bytes, Bytes>(v0)
}

pub fn f403() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f404(v0: Bytes) -> R<Option<Bytes>> {
    let m2471 = f133(v0.clone(), f8(Str::lit(" ")), 0u64)?;
    match m2471 {
        None => {
            let m2472 = f133(v0.clone(), f8(Str::lit(":")), 0u64)?;
            match m2472 {
                None => {
                    let v3: Bytes = f7(List::cons(f8(Str::lit(" ")), List::cons(v0, List::cons(f8(Str::lit(":")), List::<Bytes>::nil()))));
                    let m2473 = f133(f469(), v3.clone(), 0u64)?;
                    match m2473 {
                        None => {
                            Ok(None::<Bytes>)
                        }
                        Some(v4) => {
                            let v5: u64 = {
                                let a2475 = v4;
                                let a2476 = {
                                    let a2474 = v3;
                                    length_bytes(a2474)
                                };
                                nat_add(a2475, a2476)?
                            };
                            let m2477 = f133(f469(), f8(Str::lit(" ")), v5)?;
                            match m2477 {
                                None => {
                                    Ok(None::<Bytes>)
                                }
                                Some(v6) => {
                                    Ok(Some(f32(f469(), v5, {
                                        let a2478 = v6;
                                        let a2479 = v5;
                                        nat_sub(a2478, a2479)
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

pub fn f405(v0: Bytes) -> Bytes {
    if f470(v0.clone(), Bytes::lit(&[0x0d])) {
        f32(v0.clone(), 0u64, {
            let a2481 = {
                let a2480 = v0;
                length_bytes(a2480)
            };
            let a2482 = 1u64;
            nat_sub(a2481, a2482)
        })
    } else {
        v0
    }
}

pub fn f406() -> List<Bytes> {
    List::cons(f8(Str::lit("complexType")), List::cons(f8(Str::lit("simpleType")), List::cons(f8(Str::lit("element")), List::cons(f8(Str::lit("attribute")), List::cons(f8(Str::lit("attributeGroup")), List::cons(f8(Str::lit("group")), List::<Bytes>::nil()))))))
}

pub fn f407(v0: u64, v1: Adt23, v2: Adt24) -> R<Result<List<Adt26>, Bytes>> {
    let v3: Bytes = f154(match v2.clone() {
        Adt24::C0(_, _, h2483, _) => {
            h2483
        }
    })?;
    let v4: (Bytes, Bytes) = (match v2.clone() {
        Adt24::C0(_, h2484, _, _) => {
            h2484
        }
    }, match v2.clone() {
        Adt24::C0(_, _, h2485, _) => {
            h2485
        }
    });
    let v5: bool = f471(match v2.clone() {
        Adt24::C0(_, _, _, h2486) => {
            h2486
        }
    }, Str::lit("abstract"), Str::lit("true"));
    let m2488 = f472(match v1.clone() {
        Adt23::C0(h2487, _) => {
            h2487
        }
    }, v4, f236(v5))?;
    match m2488 {
        Ok(v7) => {
            if v5 {
                let m2489 = v7.clone();
                match m2489.uncons() {
                    None => {
                        Ok(Err::<List<Adt26>, Bytes>(f413(f8(Str::lit("an abstract complex type with no type derived from it in this schema set admits no instance (XSD 1.1 Part 1, sections 3.4.1 and 3.3.4.2)")), v3)))
                    }
                    Some((_, _)) => {
                        let m2490 = f473(v3, v7.clone(), f474(v7.clone())?, f474(v7)?)?;
                        match m2490 {
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
                let m2491 = v7.clone();
                match m2491.uncons() {
                    None => {
                        f475(v0, v1, v3, match v2.clone() {
                            Adt24::C0(_, h2492, _, _) => {
                                h2492
                            }
                        }, match v2 {
                            Adt24::C0(_, _, _, h2493) => {
                                h2493
                            }
                        })
                    }
                    Some((_, _)) => {
                        let v14: Bytes = f476(v3.clone());
                        let m2496 = f475(v0, v1, v14.clone(), match v2.clone() {
                            Adt24::C0(_, h2494, _, _) => {
                                h2494
                            }
                        }, match v2.clone() {
                            Adt24::C0(_, _, _, h2495) => {
                                h2495
                            }
                        })?;
                        match m2496 {
                            Ok(v16) => {
                                let m2498 = f473(v3, List::cons(match v2 {
                                    Adt24::C0(_, _, h2497, _) => {
                                        h2497
                                    }
                                }, v7.clone()), List::cons(v14.clone(), f474(v7.clone())?), List::cons(v14, f474(v7)?))?;
                                match m2498 {
                                    Ok(v18) => {
                                        Ok(Ok::<List<Adt26>, Bytes>(f324(v16, List::cons(v18, List::<Adt26>::nil()))))
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

pub fn f408(v0: Bytes, v1: Adt25, v2: Bytes) -> R<Result<Adt26, Bytes>> {
    let m2501 = f153(v0.clone(), match v1.clone() {
        Adt25::C0(_, _, _, h2499, _) => {
            h2499
        }
    }, match v1 {
        Adt25::C0(_, _, _, _, h2500) => {
            h2500
        }
    }, v2)?;
    match m2501 {
        Ok(v4) => {
            Ok(Ok::<Adt26, Bytes>(Adt26::C0(f154(v0)?, match v4.clone() {
                Adt22::C0(_, h2502) => {
                    h2502
                }
            }, match v4 {
                Adt22::C0(h2503, _) => {
                    h2503
                }
            }, List::<Bytes>::nil())))
        }
        Err(v3) => {
            Ok(Err::<Adt26, Bytes>(v3))
        }
    }
}

pub fn f409(v0: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: Adt25) -> R<Result<Adt31, Bytes>> {
    let m2504 = v0;
    if m2504 == 0 {
        Ok(Err::<Adt31, Bytes>(f413(f8(Str::lit("the element declarations nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m2504 - 1;
        let m2505 = f238(v4.clone(), Str::lit("type"));
        match m2505 {
            None => {
                let m2509 = f411(v4.clone())?;
                match m2509 {
                    Ok(v10) => {
                        let m2510 = f412(v10, List::cons(f8(Str::lit("complexType")), List::cons(f8(Str::lit("simpleType")), List::<Bytes>::nil())));
                        match m2510 {
                            None => {
                                Ok(Ok::<Adt31, Bytes>(Adt31::C0(Adt22::C0(f478(v4, f327()), List::cons(f327(), List::<Bytes>::nil())), List::<Adt26>::nil())))
                            }
                            Some(v11) => {
                                if f237(v11.clone(), Str::lit("complexType")) {
                                    let m2511 = f475(v5, v1, v2.clone(), v3, v11)?;
                                    match m2511 {
                                        Ok(v13) => {
                                            Ok(Ok::<Adt31, Bytes>(Adt31::C0(Adt22::C0(f478(v4, v2.clone()), List::cons(v2, List::<Bytes>::nil())), v13)))
                                        }
                                        Err(v12) => {
                                            Ok(Err::<Adt31, Bytes>(v12))
                                        }
                                    }
                                } else {
                                    let m2512 = f408(v2.clone(), v11, v3)?;
                                    match m2512 {
                                        Ok(v15) => {
                                            Ok(Ok::<Adt31, Bytes>(Adt31::C0(Adt22::C0(f478(v4, v2.clone()), List::cons(v2, List::<Bytes>::nil())), List::cons(v15, List::<Adt26>::nil()))))
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
                let m2506 = f477(v1, v4.clone(), v6)?;
                match m2506 {
                    Ok(v8) => {
                        Ok(Ok::<Adt31, Bytes>(Adt31::C0(Adt22::C0(f478(v4, match v8.clone() {
                            Adt22::C0(h2507, _) => {
                                h2507
                            }
                        }), match v8 {
                            Adt22::C0(_, h2508) => {
                                h2508
                            }
                        }), List::<Adt26>::nil())))
                    }
                    Err(v7) => {
                        Ok(Err::<Adt31, Bytes>(f413(v7, v2)))
                    }
                }
            }
        }
    }
}

pub fn f410(v0: Adt23, v1: Bytes, v2: Bytes, v3: Adt25) -> R<Result<Adt31, Bytes>> {
    let m2513 = f238(v3.clone(), Str::lit("type"));
    match m2513 {
        None => {
            let m2515 = f411(v3)?;
            match m2515 {
                Ok(v8) => {
                    let m2516 = f412(v8, List::cons(f8(Str::lit("simpleType")), List::<Bytes>::nil()));
                    match m2516 {
                        None => {
                            Ok(Err::<Adt31, Bytes>(f413(f8(Str::lit("an attribute declaration with no type has the type xs:anySimpleType, which is not translated, because it is the union of every simple value space (XSD 1.1 Part 1, section 3.2.2.1; Part 2, section 3.2.1)")), v2)))
                        }
                        Some(v9) => {
                            let m2517 = f408(v2.clone(), v9, v1)?;
                            match m2517 {
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
            let m2514 = f477(v0, v3, v4)?;
            match m2514 {
                Ok(v6) => {
                    Ok(Ok::<Adt31, Bytes>(Adt31::C0(v6, List::<Adt26>::nil())))
                }
                Err(v5) => {
                    Ok(Err::<Adt31, Bytes>(f413(v5, v2)))
                }
            }
        }
    }
}

pub fn f411(v0: Adt25) -> R<Result<List<Adt25>, Bytes>> {
    f167(match v0.clone() {
        Adt25::C0(_, _, _, h2518, _) => {
            h2518
        }
    }, match v0 {
        Adt25::C0(_, _, _, _, h2519) => {
            h2519
        }
    })
}

pub fn f412(p2523: List<Adt25>, v1: List<Bytes>) -> Option<Adt25> {
    let m2522 = p2523;
    {
        let mut m2522 = m2522;
        loop {
            let step = {
                let v0 = m2522;
                let m2520 = v0;
                match m2520.uncons() {
                    None => {
                        Err::<List<Adt25>, Option<Adt25>>(None::<Adt25>)
                    }
                    Some((v2, v3)) => {
                        if f115(match v2.clone() {
                            Adt25::C0(_, h2521, _, _, _) => {
                                h2521
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
                Ok(next) => { m2522 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f413(v0: Bytes, v1: Bytes) -> Bytes {
    f163(v0, f8(Str::lit(" at ")), v1)
}

pub fn f414(v0: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: Adt25) -> R<Result<List<Adt26>, Bytes>> {
    let m2524 = v0;
    if m2524 == 0 {
        Ok(Err::<List<Adt26>, Bytes>(f413(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m2524 - 1;
        let m2525 = f411(v4.clone())?;
        match m2525 {
            Ok(v7) => {
                if f237(v4, Str::lit("choice")) {
                    let m2526 = f479(v5, v1, v2.clone(), v3, false, v7, f416())?;
                    match m2526 {
                        Ok(v9) => {
                            let m2530 = f473(v2, f480(match v9.clone() {
                                Adt32::C0(h2527, _, _, _, _) => {
                                    h2527
                                }
                            }), f481(match v9.clone() {
                                Adt32::C0(h2528, _, _, _, _) => {
                                    h2528
                                }
                            }), f482(match v9.clone() {
                                Adt32::C0(h2529, _, _, _, _) => {
                                    h2529
                                }
                            }))?;
                            match m2530 {
                                Ok(v11) => {
                                    Ok(Ok::<List<Adt26>, Bytes>(f324(match v9 {
                                        Adt32::C0(_, h2531, _, _, _) => {
                                            h2531
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
                    let m2532 = f479(v5, v1, v2.clone(), v3, true, v7, f416())?;
                    match m2532 {
                        Ok(v13) => {
                            let m2534 = f483(v2, match v13.clone() {
                                Adt32::C0(h2533, _, _, _, _) => {
                                    h2533
                                }
                            })?;
                            match m2534 {
                                Ok(v15) => {
                                    Ok(Ok::<List<Adt26>, Bytes>(f324(match v13 {
                                        Adt32::C0(_, h2535, _, _, _) => {
                                            h2535
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

pub fn f415(v0: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: List<Adt25>, v5: Adt32) -> R<Result<Adt32, Bytes>> {
    let m2536 = v0;
    if m2536 == 0 {
        Ok(Err::<Adt32, Bytes>(f413(f8(Str::lit("the attribute group references nest deeper than the schema set has components, so they refer to each other in a cycle (XSD 1.1 Part 1, section 3.6.3)")), v2)))
    } else {
        let v6 = m2536 - 1;
        let m2537 = v4;
        match m2537.uncons() {
            None => {
                Ok(Ok::<Adt32, Bytes>(v5))
            }
            Some((v7, v8)) => {
                if f237(v7.clone(), Str::lit("attribute")) {
                    if f471(v7.clone(), Str::lit("use"), Str::lit("prohibited")) {
                        f415(v6, v1, v2, v3, v8, v5)
                    } else {
                        let v9: bool = f471(v7.clone(), Str::lit("use"), Str::lit("required"));
                        let m2538 = f238(v7.clone(), Str::lit("ref"));
                        match m2538 {
                            None => {
                                let m2548 = f238(v7.clone(), Str::lit("name"));
                                match m2548 {
                                    None => {
                                        Ok(Err::<Adt32, Bytes>(f413(f8(Str::lit("a local attribute declaration must have a name or a ref (XSD 1.1 Part 1, section 3.2.3)")), v2)))
                                    }
                                    Some(v16) => {
                                        let m2551 = f410(v1.clone(), v3.clone(), {
                                            let a2549 = v2.clone();
                                            let a2550 = f154(v16.clone())?;
                                            append_bytes(a2549, a2550)
                                        }, v7)?;
                                        match m2551 {
                                            Ok(v18) => {
                                                f415(v6, v1, v2, v3, v8, f485(v5, v16, v18, v9))
                                            }
                                            Err(v17) => {
                                                Ok(Err::<Adt32, Bytes>(v17))
                                            }
                                        }
                                    }
                                }
                            }
                            Some(v10) => {
                                let m2540 = f91(match v7 {
                                    Adt25::C0(_, _, _, _, h2539) => {
                                        h2539
                                    }
                                }, v10.clone())?;
                                match m2540 {
                                    Ok(v12) => {
                                        if f35({
                                            let (h2541, _) = v12.clone();
                                            h2541
                                        }, f145()) {
                                            Ok(Err::<Adt32, Bytes>(f413(f163(f8(Str::lit("the attribute ")), v10, f8(Str::lit(" is declared by the schema document for the XML namespace, which this schema set does not import (XSD 1.1 Part 1, sections 3.2.2.3 and 4.2.6)"))), v2)))
                                        } else {
                                            let m2542 = f484(v1.clone(), Str::lit("attribute"), v12.clone());
                                            match m2542 {
                                                None => {
                                                    Ok(Err::<Adt32, Bytes>(f413(f163(f8(Str::lit("the attribute ")), v10, f8(Str::lit(" is not declared by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                                }
                                                Some(v13) => {
                                                    let m2546 = f410(v1.clone(), match v13.clone() {
                                                        Adt24::C0(_, h2543, _, _) => {
                                                            h2543
                                                        }
                                                    }, f154({
                                                        let (_, h2544) = v12.clone();
                                                        h2544
                                                    })?, match v13 {
                                                        Adt24::C0(_, _, _, h2545) => {
                                                            h2545
                                                        }
                                                    })?;
                                                    match m2546 {
                                                        Ok(v15) => {
                                                            f415(v6, v1, v2, v3, v8, f485(v5, {
                                                                let (_, h2547) = v12;
                                                                h2547
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
                                        Ok(Err::<Adt32, Bytes>(f413(v11, v2)))
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if f237(v7.clone(), Str::lit("attributeGroup")) {
                        let m2552 = f238(v7.clone(), Str::lit("ref"));
                        match m2552 {
                            None => {
                                Ok(Err::<Adt32, Bytes>(f413(f8(Str::lit("an attribute group reference must have a ref (XSD 1.1 Part 1, section 3.6.2)")), v2)))
                            }
                            Some(v19) => {
                                let m2554 = f91(match v7 {
                                    Adt25::C0(_, _, _, _, h2553) => {
                                        h2553
                                    }
                                }, v19.clone())?;
                                match m2554 {
                                    Ok(v21) => {
                                        let m2555 = f484(v1.clone(), Str::lit("attributeGroup"), v21.clone());
                                        match m2555 {
                                            None => {
                                                Ok(Err::<Adt32, Bytes>(f413(f163(f8(Str::lit("the attribute group ")), v19, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                            }
                                            Some(v22) => {
                                                let m2557 = f411(match v22.clone() {
                                                    Adt24::C0(_, _, _, h2556) => {
                                                        h2556
                                                    }
                                                })?;
                                                match m2557 {
                                                    Ok(v24) => {
                                                        let m2560 = f415(v6, v1.clone(), f154({
                                                            let (_, h2558) = v21;
                                                            h2558
                                                        })?, match v22 {
                                                            Adt24::C0(_, h2559, _, _) => {
                                                                h2559
                                                            }
                                                        }, v24, v5)?;
                                                        match m2560 {
                                                            Ok(v26) => {
                                                                f415(v6, v1, v2, v3, v8, v26)
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
                                        Ok(Err::<Adt32, Bytes>(f413(v20, v2)))
                                    }
                                }
                            }
                        }
                    } else {
                        if f237(v7, Str::lit("anyAttribute")) {
                            f415(v6, v1, v2, v3, v8, f486(v5, Adt33::C0(f8(Str::lit("anyAttribute")), f487(), List::<Bytes>::nil())))
                        } else {
                            f415(v6, v1, v2, v3, v8, v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f416() -> Adt32 {
    Adt32::C0(List::<Adt33>::nil(), List::<Adt26>::nil(), 0u64, 0u64, 0u64)
}

pub fn f417(v0: List<Adt26>, v1: List<Bytes>, v2: Bytes) -> Result<List<Bytes>, Bytes> {
    let m2561 = v1;
    match m2561.uncons() {
        None => {
            Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let m2562 = f326(v0.clone(), v3.clone());
            match m2562 {
                None => {
                    Err::<List<Bytes>, Bytes>(f418(v3))
                }
                Some(v5) => {
                    let m2564 = match v5.clone() {
                        Adt26::C0(_, _, _, h2563) => {
                            h2563
                        }
                    };
                    match m2564.uncons() {
                        None => {
                            Err::<List<Bytes>, Bytes>(f418(v3))
                        }
                        Some((_, _)) => {
                            let m2565 = f417(v0, v4, v2.clone());
                            match m2565 {
                                Ok(v9) => {
                                    Ok::<List<Bytes>, Bytes>(List::cons(f488(v3, match v5 {
                                        Adt26::C0(_, _, _, h2566) => {
                                            h2566
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

pub fn f418(v0: Bytes) -> Bytes {
    f413(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)")), v0)
}

pub fn f419(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m2567 = v0;
    match m2567.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f115(v2.clone(), v1.clone()) {
                f419(v3, v1)
            } else {
                List::cons(v2, f419(v3, v1))
            }
        }
    }
}

pub fn f420(p2572: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m2571 = p2572;
    {
        let mut m2571 = m2571;
        loop {
            let step = {
                let v0 = m2571;
                let m2568 = v0;
                match m2568.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h2569, _) = v2;
                            h2569
                        }, v1.clone()) {
                            let (_, h2570) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h2570)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2571 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f421(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f489(v0, v1)
}

pub fn f422(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f244(f174(v0, v1)?) {
        Ok(Some({
            let a2573 = v1;
            let a2574 = 1u64;
            nat_add(a2573, a2574)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f423(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f490(v0, v1)
}

pub fn f424(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2575 = f174(v0, v1)?;
    let a2576 = 32u64;
    if nat_eq(a2575, a2576) {
        Ok(Some({
            let a2577 = v1;
            let a2578 = 1u64;
            nat_add(a2577, a2578)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f425(v0: u64) -> bool {
    let a2579 = v0;
    let a2580 = 44u64;
    if if if nat_eq(a2579, a2580) {
        true
    } else {
        let a2581 = v0;
        let a2582 = 91u64;
        nat_eq(a2581, a2582)
    } {
        true
    } else {
        let a2583 = v0;
        let a2584 = 93u64;
        if nat_eq(a2583, a2584) {
            true
        } else {
            let a2585 = v0;
            let a2586 = 123u64;
            nat_eq(a2585, a2586)
        }
    } {
        true
    } else {
        let a2587 = v0;
        let a2588 = 125u64;
        nat_eq(a2587, a2588)
    }
}

pub fn f426(v0: u64) -> u64 {
    let a2589 = v0;
    let a2590 = 45u64;
    if nat_eq(a2589, a2590) {
        1u64
    } else {
        let a2591 = v0;
        let a2592 = 43u64;
        if nat_eq(a2591, a2592) {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f427(v0: u64) -> u64 {
    let a2593 = 49u64;
    let a2594 = v0;
    if if nat_le(a2593, a2594) {
        let a2595 = v0;
        let a2596 = 57u64;
        nat_le(a2595, a2596)
    } else {
        false
    } {
        let a2597 = v0;
        let a2598 = 48u64;
        nat_sub(a2597, a2598)
    } else {
        0u64
    }
}

pub fn f428(v0: Bytes, v1: u64, v2: u64, v3: bool, v4: u64) -> R<Adt20> {
    let m2599 = f491(v0.clone(), v1, v2)?;
    match m2599 {
        Adt34::C0(v6, v7) => {
            let v8: u64 = v7;
            let v9: u64 = f492(v0.clone(), v8, v2)?;
            if f493(v0.clone(), v8, v2)? {
                let m2603 = f494(v0.clone(), v8, v2, v3, {
                    let a2601 = {
                        let a2600 = v0;
                        length_bytes(a2600)
                    };
                    let a2602 = 2u64;
                    nat_add(a2601, a2602)?
                }, Bytes::lit(&[]), v6)?;
                match m2603 {
                    Adt35::C0(v11, v12, v13) => {
                        Ok(Adt20::C0(Adt19::C5(f7(List::cons(v11, List::cons({
                            let a2604 = v4;
                            let a2605 = 1u64;
                            if nat_eq(a2604, a2605) {
                                Bytes::lit(&[])
                            } else {
                                Bytes::lit(&[0x0a])
                            }
                        }, List::cons({
                            let a2606 = v4;
                            let a2607 = 2u64;
                            if nat_eq(a2606, a2607) {
                                f430(v12)
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
                    let a2608 = v4;
                    let a2609 = 2u64;
                    if nat_eq(a2608, a2609) {
                        f430(v6)
                    } else {
                        Bytes::lit(&[])
                    }
                }), {
                    let a2610 = f174(v0, v9)?;
                    let a2611 = 256u64;
                    if nat_eq(a2610, a2611) {
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

pub fn f429(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f495(v0, v1)
}

pub fn f430(v0: u64) -> Bytes {
    let m2612 = v0;
    if m2612 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2612 - 1;
        let a2613 = Bytes::lit(&[0x0a]);
        let a2614 = f430(v1);
        append_bytes(a2613, a2614)
    }
}

pub fn f431(v0: Bytes, v1: u64) -> R<u64> {
    let (_, h2615) = {
        let (h2616, _) = f394(Fn7::F496(v0.clone()), {
            let a2618 = {
                let a2617 = v0;
                length_bytes(a2617)
            };
            let a2619 = 1u64;
            nat_add(a2618, a2619)?
        }, (v1, 0u64))?;
        h2616
    };
    Ok(h2615)
}

pub fn f432(v0: u64, v1: u64) -> u64 {
    let a2620 = v0;
    let a2621 = v1;
    if nat_lt(a2620, a2621) {
        v1
    } else {
        v0
    }
}

pub fn f433(v0: Bytes, p2680: u64, v2: u64, v3: bool, p2681: u64, p2682: Bytes) -> R<Adt30> {
    let m2679 = (p2680, (p2681, p2682));
    {
        let mut m2679 = m2679;
        loop {
            let step = {
                let (v1, (v4, v5)) = m2679;
                let m2622 = v4;
                if m2622 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f83(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                } else {
                    let v6 = m2622 - 1;
                    let v7: u64 = f174(v0.clone(), v1)?;
                    let a2623 = v7;
                    let a2624 = 34u64;
                    if nat_eq(a2623, a2624) {
                        Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C0(v5, {
                            let a2625 = v1;
                            let a2626 = 1u64;
                            nat_add(a2625, a2626)?
                        })))
                    } else {
                        let a2627 = v7;
                        let a2628 = 256u64;
                        if nat_eq(a2627, a2628) {
                            Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f83(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                        } else {
                            let a2629 = v7;
                            let a2630 = 92u64;
                            if nat_eq(a2629, a2630) {
                                let v8: u64 = f174(v0.clone(), {
                                    let a2631 = v1;
                                    let a2632 = 1u64;
                                    nat_add(a2631, a2632)?
                                })?;
                                let m2633 = f497(v8)?;
                                match m2633 {
                                    None => {
                                        let a2638 = v8;
                                        let a2639 = 10u64;
                                        if nat_eq(a2638, a2639) {
                                            if v3 {
                                                Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f498(v1)?)))
                                            } else {
                                                let m2642 = f499(v0.clone(), {
                                                    let a2640 = v1;
                                                    let a2641 = 1u64;
                                                    nat_add(a2640, a2641)?
                                                }, v2, v6, 0u64)?;
                                                match m2642 {
                                                    Adt34::C0(v11, v12) => {
                                                        Ok::<(u64, (u64, Bytes)), R<Adt30>>((v12, (v6, {
                                                            let a2643 = v5;
                                                            let a2644 = f430(v11);
                                                            append_bytes(a2643, a2644)
                                                        })))
                                                    }
                                                    Adt34::C1(v10) => {
                                                        Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(v10)))
                                                    }
                                                }
                                            }
                                        } else {
                                            let v13: u64 = {
                                                let a2645 = v8;
                                                let a2646 = 120u64;
                                                if nat_eq(a2645, a2646) {
                                                    2u64
                                                } else {
                                                    let a2647 = v8;
                                                    let a2648 = 117u64;
                                                    if nat_eq(a2647, a2648) {
                                                        4u64
                                                    } else {
                                                        let a2649 = v8;
                                                        let a2650 = 85u64;
                                                        if nat_eq(a2649, a2650) {
                                                            8u64
                                                        } else {
                                                            0u64
                                                        }
                                                    }
                                                }
                                            };
                                            let a2651 = v13;
                                            let a2652 = 0u64;
                                            if nat_eq(a2651, a2652) {
                                                Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f83(Str::lit("an escape must be one of YAML 1.2.2 [42] to [62]"), v1)?)))
                                            } else {
                                                let m2655 = f500(v0.clone(), {
                                                    let a2653 = v1;
                                                    let a2654 = 2u64;
                                                    nat_add(a2653, a2654)?
                                                }, v13, 0u64)?;
                                                match m2655 {
                                                    None => {
                                                        Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f83(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                    }
                                                    Some(v14) => {
                                                        let a2656 = 1114111u64;
                                                        let a2657 = v14;
                                                        if if nat_lt(a2656, a2657) {
                                                            true
                                                        } else {
                                                            let a2658 = 55296u64;
                                                            let a2659 = v14;
                                                            if nat_le(a2658, a2659) {
                                                                let a2660 = v14;
                                                                let a2661 = 57343u64;
                                                                nat_le(a2660, a2661)
                                                            } else {
                                                                false
                                                            }
                                                        } {
                                                            Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f83(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                        } else {
                                                            Ok::<(u64, (u64, Bytes)), R<Adt30>>(({
                                                                let a2664 = {
                                                                    let a2662 = v1;
                                                                    let a2663 = 2u64;
                                                                    nat_add(a2662, a2663)?
                                                                };
                                                                let a2665 = v13;
                                                                nat_add(a2664, a2665)?
                                                            }, (v6, {
                                                                let a2666 = v5;
                                                                let a2667 = f308(v14)?;
                                                                append_bytes(a2666, a2667)
                                                            })))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Some(v9) => {
                                        Ok::<(u64, (u64, Bytes)), R<Adt30>>(({
                                            let a2634 = v1;
                                            let a2635 = 2u64;
                                            nat_add(a2634, a2635)?
                                        }, (v6, {
                                            let a2636 = v5;
                                            let a2637 = f308(v9)?;
                                            append_bytes(a2636, a2637)
                                        })))
                                    }
                                }
                            } else {
                                if if f244(v7) {
                                    true
                                } else {
                                    let a2668 = v7;
                                    let a2669 = 10u64;
                                    nat_eq(a2668, a2669)
                                } {
                                    let v15: u64 = f247(v0.clone(), v1)?;
                                    let a2670 = f174(v0.clone(), v15)?;
                                    let a2671 = 10u64;
                                    if nat_eq(a2670, a2671) {
                                        if v3 {
                                            Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f498(v1)?)))
                                        } else {
                                            let m2672 = f499(v0.clone(), v15, v2, v6, 0u64)?;
                                            match m2672 {
                                                Adt34::C0(v17, v18) => {
                                                    Ok::<(u64, (u64, Bytes)), R<Adt30>>((v18, (v6, {
                                                        let a2673 = v5;
                                                        let a2674 = f441(v17);
                                                        append_bytes(a2673, a2674)
                                                    })))
                                                }
                                                Adt34::C1(v16) => {
                                                    Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(v16)))
                                                }
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (u64, Bytes)), R<Adt30>>((v15, (v6, {
                                            let a2675 = v5;
                                            let a2676 = f256(v0.clone(), v1, v15);
                                            append_bytes(a2675, a2676)
                                        })))
                                    }
                                } else {
                                    let v19: u64 = f501(v0.clone(), v1)?;
                                    Ok::<(u64, (u64, Bytes)), R<Adt30>>((v19, (v6, {
                                        let a2677 = v5;
                                        let a2678 = f256(v0.clone(), v1, v19);
                                        append_bytes(a2677, a2678)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2679 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f434(v0: Bytes, p2710: u64, v2: u64, v3: bool, p2711: u64, p2712: Bytes) -> R<Adt30> {
    let m2709 = (p2710, (p2711, p2712));
    {
        let mut m2709 = m2709;
        loop {
            let step = {
                let (v1, (v4, v5)) = m2709;
                let m2683 = v4;
                if m2683 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f83(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                } else {
                    let v6 = m2683 - 1;
                    let v7: u64 = f174(v0.clone(), v1)?;
                    let a2684 = v7;
                    let a2685 = 39u64;
                    if nat_eq(a2684, a2685) {
                        let a2688 = f174(v0.clone(), {
                            let a2686 = v1;
                            let a2687 = 1u64;
                            nat_add(a2686, a2687)?
                        })?;
                        let a2689 = 39u64;
                        if nat_eq(a2688, a2689) {
                            Ok::<(u64, (u64, Bytes)), R<Adt30>>(({
                                let a2690 = v1;
                                let a2691 = 2u64;
                                nat_add(a2690, a2691)?
                            }, (v6, {
                                let a2692 = v5;
                                let a2693 = Bytes::lit(&[0x27]);
                                append_bytes(a2692, a2693)
                            })))
                        } else {
                            Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C0(v5, {
                                let a2694 = v1;
                                let a2695 = 1u64;
                                nat_add(a2694, a2695)?
                            })))
                        }
                    } else {
                        let a2696 = v7;
                        let a2697 = 256u64;
                        if nat_eq(a2696, a2697) {
                            Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f83(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                        } else {
                            if if f244(v7) {
                                true
                            } else {
                                let a2698 = v7;
                                let a2699 = 10u64;
                                nat_eq(a2698, a2699)
                            } {
                                let v8: u64 = f247(v0.clone(), v1)?;
                                let a2700 = f174(v0.clone(), v8)?;
                                let a2701 = 10u64;
                                if nat_eq(a2700, a2701) {
                                    if v3 {
                                        Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f498(v1)?)))
                                    } else {
                                        let m2702 = f499(v0.clone(), v8, v2, v6, 0u64)?;
                                        match m2702 {
                                            Adt34::C0(v10, v11) => {
                                                Ok::<(u64, (u64, Bytes)), R<Adt30>>((v11, (v6, {
                                                    let a2703 = v5;
                                                    let a2704 = f441(v10);
                                                    append_bytes(a2703, a2704)
                                                })))
                                            }
                                            Adt34::C1(v9) => {
                                                Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(v9)))
                                            }
                                        }
                                    }
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Adt30>>((v8, (v6, {
                                        let a2705 = v5;
                                        let a2706 = f256(v0.clone(), v1, v8);
                                        append_bytes(a2705, a2706)
                                    })))
                                }
                            } else {
                                let v12: u64 = f502(v0.clone(), v1)?;
                                Ok::<(u64, (u64, Bytes)), R<Adt30>>((v12, (v6, {
                                    let a2707 = v5;
                                    let a2708 = f256(v0.clone(), v1, v12);
                                    append_bytes(a2707, a2708)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2709 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f435(v0: Bytes, v1: u64, v2: u64) -> R<Result<u64, Bytes>> {
    let v3: (u64, bool) = {
        let (h2713, _) = f503(Fn8::F504(v2, v0.clone()), {
            let a2715 = {
                let a2714 = v0.clone();
                length_bytes(a2714)
            };
            let a2716 = 1u64;
            nat_add(a2715, a2716)?
        }, (v1, false))?;
        h2713
    };
    let (_, h2717) = v3;
    if h2717 {
        if f172(v0, {
            let (h2718, _) = v3;
            h2718
        })? {
            Ok(Err::<u64, Bytes>(f83(Str::lit("a document marker cannot appear inside a flow collection (YAML 1.2.2 [206])"), {
                let (h2719, _) = v3;
                h2719
            })?))
        } else {
            Ok(Err::<u64, Bytes>(f83(Str::lit("a line of a flow collection is indented less than its node (YAML 1.2.2 [80])"), {
                let (h2720, _) = v3;
                h2720
            })?))
        }
    } else {
        Ok(Ok::<u64, Bytes>({
            let (h2721, _) = v3;
            h2721
        }))
    }
}

pub fn f436(v0: Bytes, v1: u64, v2: u64) -> R<Adt30> {
    let v3: u64 = f174(v0.clone(), v1)?;
    let a2722 = v3;
    let a2723 = 34u64;
    if if nat_eq(a2722, a2723) {
        true
    } else {
        let a2724 = v3;
        let a2725 = 39u64;
        nat_eq(a2724, a2725)
    } {
        f346(v0, v1, v2, false)
    } else {
        let m2726 = f252(v0.clone(), v1)?;
        match m2726 {
            None => {
                let a2727 = v3;
                let a2728 = 91u64;
                if if nat_eq(a2727, a2728) {
                    true
                } else {
                    let a2729 = v3;
                    let a2730 = 123u64;
                    nat_eq(a2729, a2730)
                } {
                    Ok(Adt30::C1(f83(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [142])"), v1)?))
                } else {
                    if f348(v0.clone(), v1, true)? {
                        let v5: u64 = f349(v0.clone(), v1, true)?;
                        let v6: u64 = f247(v0.clone(), v5)?;
                        let a2731 = f174(v0.clone(), v6)?;
                        let a2732 = 10u64;
                        if nat_eq(a2731, a2732) {
                            let m2733 = f435(v0.clone(), v6, v2)?;
                            match m2733 {
                                Ok(v8) => {
                                    let v9: u64 = f174(v0.clone(), v8)?;
                                    let a2734 = v9;
                                    let a2735 = 58u64;
                                    if if if nat_eq(a2734, a2735) {
                                        true
                                    } else {
                                        let a2736 = v9;
                                        let a2737 = 44u64;
                                        nat_eq(a2736, a2737)
                                    } {
                                        true
                                    } else {
                                        let a2738 = v9;
                                        let a2739 = 125u64;
                                        nat_eq(a2738, a2739)
                                    } {
                                        Ok(Adt30::C0(f256(v0, v1, v5), v5))
                                    } else {
                                        Ok(Adt30::C1(f83(Str::lit("a multi-line plain scalar inside a flow collection is not read (YAML 1.2.2 [135])"), v6)?))
                                    }
                                }
                                Err(v7) => {
                                    Ok(Adt30::C1(v7))
                                }
                            }
                        } else {
                            Ok(Adt30::C0(f256(v0, v1, v5), v5))
                        }
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

pub fn f437(v0: u64) -> R<bool> {
    let a2740 = f255(f8(Str::lit(",[]{}-?:#&*!|>'\"@`")), v0)?;
    let a2741 = 18u64;
    if nat_lt(a2740, a2741) {
        Ok(true)
    } else {
        let a2742 = v0;
        let a2743 = 37u64;
        Ok(nat_eq(a2742, a2743))
    }
}

pub fn f438(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f505(v1, v0, v2)
}

pub fn f439(v0: u64) -> R<Bytes> {
    f83(Str::lit("a mapping value is not allowed here: a plain scalar holds ': ' (YAML 1.2.2 [130])"), v0)
}

pub fn f440(v0: Bytes, v1: u64) -> R<(u64, u64)> {
    let (h2744, _) = f394(Fn7::F506(v0.clone()), {
        let a2746 = {
            let a2745 = v0;
            length_bytes(a2745)
        };
        let a2747 = 1u64;
        nat_add(a2746, a2747)?
    }, (v1, 0u64))?;
    Ok(h2744)
}

pub fn f441(v0: u64) -> Bytes {
    let a2748 = v0;
    let a2749 = 0u64;
    if nat_eq(a2748, a2749) {
        Bytes::lit(&[0x20])
    } else {
        f430(v0)
    }
}

pub fn f442(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt20> {
    let v4: (u64, u64) = f507(v0.clone(), v1, 2u64)?;
    let a2752 = {
        let (h2750, _) = v4;
        h2750
    };
    let a2753 = {
        let a2751 = v0.clone();
        length_bytes(a2751)
    };
    if nat_eq(a2752, a2753) {
        if f508(false, {
            let (_, h2754) = v4;
            h2754
        }) {
            Ok(Adt20::C0(Adt19::C3(false, {
                let (_, h2755) = v4;
                h2755
            }), v3))
        } else {
            Ok(Adt20::C1(f445(v2)?))
        }
    } else {
        Ok(Adt20::C0(Adt19::C5(v0), v3))
    }
}

pub fn f443(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2756 = v0.clone();
        length_bytes(a2756)
    };
    let v2: u64 = f174(v0.clone(), 0u64)?;
    let v3: u64 = {
        let a2757 = v2;
        let a2758 = 43u64;
        if if nat_eq(a2757, a2758) {
            true
        } else {
            let a2759 = v2;
            let a2760 = 45u64;
            nat_eq(a2759, a2760)
        } {
            1u64
        } else {
            0u64
        }
    };
    let v4: u64 = f509(v0.clone(), v3)?;
    let v5: bool = {
        let a2761 = f174(v0.clone(), v4)?;
        let a2762 = 46u64;
        nat_eq(a2761, a2762)
    };
    let v6: u64 = if v5 {
        let a2763 = v4;
        let a2764 = 1u64;
        nat_add(a2763, a2764)?
    } else {
        v4
    };
    let v7: u64 = if v5 {
        f509(v0.clone(), v6)?
    } else {
        v4
    };
    let v8: u64 = f174(v0.clone(), v7)?;
    let v9: bool = {
        let a2765 = v8;
        let a2766 = 101u64;
        if nat_eq(a2765, a2766) {
            true
        } else {
            let a2767 = v8;
            let a2768 = 69u64;
            nat_eq(a2767, a2768)
        }
    };
    let v10: u64 = f174(v0.clone(), {
        let a2769 = v7;
        let a2770 = 1u64;
        nat_add(a2769, a2770)?
    })?;
    let v11: bool = if v9 {
        let a2771 = v10;
        let a2772 = 43u64;
        if nat_eq(a2771, a2772) {
            true
        } else {
            let a2773 = v10;
            let a2774 = 45u64;
            nat_eq(a2773, a2774)
        }
    } else {
        false
    };
    let v12: u64 = if v9 {
        if v11 {
            let a2775 = v7;
            let a2776 = 2u64;
            nat_add(a2775, a2776)?
        } else {
            let a2777 = v7;
            let a2778 = 1u64;
            nat_add(a2777, a2778)?
        }
    } else {
        v7
    };
    let v13: u64 = if v9 {
        f509(v0.clone(), v12)?
    } else {
        v7
    };
    let v14: Bytes = f256(v0.clone(), v3, v4);
    let v15: u64 = f510(v14.clone())?;
    let a2779 = v13;
    let a2780 = v1;
    if if if nat_eq(a2779, a2780) {
        let a2781 = v3;
        let a2782 = v4;
        if nat_lt(a2781, a2782) {
            true
        } else {
            if v5 {
                let a2783 = v6;
                let a2784 = v7;
                nat_lt(a2783, a2784)
            } else {
                false
            }
        }
    } else {
        false
    } {
        if f176(v9) {
            true
        } else {
            let a2785 = v12;
            let a2786 = v13;
            nat_lt(a2785, a2786)
        }
    } else {
        false
    } {
        Ok(Some(f7(List::cons({
            let a2787 = v2;
            let a2788 = 45u64;
            if nat_eq(a2787, a2788) {
                Bytes::lit(&[0x2d])
            } else {
                Bytes::lit(&[])
            }
        }, List::cons({
            let a2790 = v15;
            let a2791 = {
                let a2789 = v14.clone();
                length_bytes(a2789)
            };
            if nat_eq(a2790, a2791) {
                Bytes::lit(&[0x30])
            } else {
                f256(v14.clone(), v15, {
                    let a2792 = v14;
                    length_bytes(a2792)
                })
            }
        }, List::cons({
            let a2793 = v6;
            let a2794 = v7;
            if nat_lt(a2793, a2794) {
                let a2795 = Bytes::lit(&[0x2e]);
                let a2796 = f256(v0.clone(), v6, v7);
                append_bytes(a2795, a2796)
            } else {
                Bytes::lit(&[])
            }
        }, List::cons(if v9 {
            let a2799 = if if v11 {
                let a2797 = v10;
                let a2798 = 45u64;
                nat_eq(a2797, a2798)
            } else {
                false
            } {
                Bytes::lit(&[0x65, 0x2d])
            } else {
                Bytes::lit(&[0x65])
            };
            let a2800 = f256(v0, v12, v13);
            append_bytes(a2799, a2800)
        } else {
            Bytes::lit(&[])
        }, List::<Bytes>::nil())))))))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f444(v0: Bytes, v1: u64) -> R<Option<(u64, Adt19)>> {
    let v2: bool = {
        let a2801 = f139(v0.clone(), v1)?;
        let a2802 = 45u64;
        nat_eq(a2801, a2802)
    };
    let v3: u64 = if v2 {
        let a2803 = v1;
        let a2804 = 1u64;
        nat_add(a2803, a2804)?
    } else {
        v1
    };
    let v4: u64 = f509(v0.clone(), v3)?;
    let v5: u64 = {
        let a2805 = v4;
        let a2806 = v3;
        nat_sub(a2805, a2806)
    };
    let v6: bool = {
        let a2807 = f139(v0.clone(), v4)?;
        let a2808 = 46u64;
        nat_eq(a2807, a2808)
    };
    let v7: u64 = if v6 {
        let a2809 = v4;
        let a2810 = 1u64;
        nat_add(a2809, a2810)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f509(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f139(v0.clone(), v8)?;
    let v10: bool = {
        let a2811 = v9;
        let a2812 = 101u64;
        if nat_eq(a2811, a2812) {
            true
        } else {
            let a2813 = v9;
            let a2814 = 69u64;
            nat_eq(a2813, a2814)
        }
    };
    let v11: u64 = f139(v0.clone(), {
        let a2815 = v8;
        let a2816 = 1u64;
        nat_add(a2815, a2816)?
    })?;
    let v12: bool = f315(v10, f317({
        let a2817 = v11;
        let a2818 = 43u64;
        nat_eq(a2817, a2818)
    }, {
        let a2819 = v11;
        let a2820 = 45u64;
        nat_eq(a2819, a2820)
    }));
    let v13: u64 = if v10 {
        if v12 {
            let a2821 = v8;
            let a2822 = 2u64;
            nat_add(a2821, a2822)?
        } else {
            let a2823 = v8;
            let a2824 = 1u64;
            nat_add(a2823, a2824)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f509(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f32(v0.clone(), v13, {
        let a2825 = v14;
        let a2826 = v13;
        nat_sub(a2825, a2826)
    });
    let v16: u64 = f510(v15.clone())?;
    let v17: Bytes = f32(v15.clone(), v16, {
        let a2828 = {
            let a2827 = v15;
            length_bytes(a2827)
        };
        let a2829 = v16;
        nat_sub(a2828, a2829)
    });
    let a2830 = v5;
    let a2831 = 0u64;
    if if nat_eq(a2830, a2831) {
        true
    } else {
        if f315({
            let a2832 = 1u64;
            let a2833 = v5;
            nat_lt(a2832, a2833)
        }, {
            let a2834 = f139(v0.clone(), v3)?;
            let a2835 = 48u64;
            nat_eq(a2834, a2835)
        }) {
            true
        } else {
            if f315(v6, {
                let a2836 = v8;
                let a2837 = v7;
                nat_eq(a2836, a2837)
            }) {
                true
            } else {
                f317(if v10 {
                    let a2838 = v14;
                    let a2839 = v13;
                    nat_eq(a2838, a2839)
                } else {
                    false
                }, {
                    let a2841 = 18u64;
                    let a2842 = {
                        let a2840 = v17.clone();
                        length_bytes(a2840)
                    };
                    nat_lt(a2841, a2842)
                })
            }
        }
    } {
        Ok(None::<(u64, Adt19)>)
    } else {
        let m2846 = {
            let a2844 = {
                let a2843 = v17.clone();
                length_bytes(a2843)
            };
            let a2845 = 0u64;
            if nat_eq(a2844, a2845) {
                Some(0u64)
            } else {
                f511(v17)?
            }
        };
        match m2846 {
            None => {
                Ok(None::<(u64, Adt19)>)
            }
            Some(v18) => {
                let m2851 = f512(f32(v0.clone(), v3, v5), f32(v0, v7, {
                    let a2847 = v8;
                    let a2848 = v7;
                    nat_sub(a2847, a2848)
                }), f315(v12, {
                    let a2849 = v11;
                    let a2850 = 45u64;
                    nat_eq(a2849, a2850)
                }), v18)?;
                match m2851 {
                    None => {
                        Ok(None::<(u64, Adt19)>)
                    }
                    Some(v19) => {
                        let a2854 = 19u64;
                        let a2855 = {
                            let a2853 = {
                                let (h2852, _) = v19.clone();
                                h2852
                            };
                            length_bytes(a2853)
                        };
                        if nat_lt(a2854, a2855) {
                            Ok(None::<(u64, Adt19)>)
                        } else {
                            let m2857 = f511({
                                let (h2856, _) = v19.clone();
                                h2856
                            })?;
                            match m2857 {
                                None => {
                                    Ok(None::<(u64, Adt19)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f315(v2, {
                                        let a2858 = 0u64;
                                        let a2859 = v20;
                                        nat_lt(a2858, a2859)
                                    });
                                    if f508(v21, v20) {
                                        Ok(Some((v14, {
                                            let a2861 = {
                                                let (_, h2860) = v19.clone();
                                                h2860
                                            };
                                            let a2862 = 0u64;
                                            if nat_eq(a2861, a2862) {
                                                Adt19::C3(v21, v20)
                                            } else {
                                                Adt19::C4(v21, v20, {
                                                    let (_, h2863) = v19;
                                                    h2863
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

pub fn f445(v0: u64) -> R<Bytes> {
    f83(Str::lit("a number outside the range Studio.Value holds is not read (YAML 1.2.2 section 10.3.2)"), v0)
}

pub fn f446(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a2865 = v2;
    let a2866 = {
        let a2864 = v0.clone();
        length_bytes(a2864)
    };
    if if nat_lt(a2865, a2866) {
        f176({
            let a2867 = f174(v0, v2)?;
            let a2868 = v1;
            nat_eq(a2867, a2868)
        })
    } else {
        false
    } {
        Ok(Some({
            let a2869 = v2;
            let a2870 = 1u64;
            nat_add(a2869, a2870)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f447(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f513(v0, v1)
}

pub fn f448(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m2871 = v0.clone();
    match m2871.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a2874 = {
                let (h2872, _) = v2;
                h2872
            };
            let a2875 = {
                let (h2873, _) = v1.clone();
                h2873
            };
            if equal(a2874, a2875) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f449(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f514(f515(Fn9::F516, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m2880 = {
        let (h2876, _) = f517(Fn10::F518, {
            let a2878 = {
                let a2877 = v0;
                length_list(a2877)
            };
            let a2879 = 1u64;
            nat_add(a2878, a2879)?
        }, v1)?;
        h2876
    };
    match m2880.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f450(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2885 = {
        let a2882 = f356({
            let (h2881, _) = v0;
            h2881
        })?;
        let a2883 = Bytes::lit(&[0x3a]);
        append_bytes(a2882, a2883)
    };
    let a2886 = {
        let (_, h2884) = v0;
        h2884
    };
    Ok(append_bytes(a2885, a2886))
}

pub fn f451(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    if f519(v1, f87(v0, v2)?) {
        Ok(Some({
            let a2887 = v2;
            let a2888 = 1u64;
            nat_add(a2887, a2888)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f452(p2891: List<Adt13>, p2892: List<Adt13>) -> List<Adt13> {
    let m2890 = (p2891, p2892);
    {
        let mut m2890 = m2890;
        loop {
            let step = {
                let (v0, v1) = m2890;
                let m2889 = v0;
                match m2889.uncons() {
                    None => {
                        Err::<(List<Adt13>, List<Adt13>), List<Adt13>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt13>, List<Adt13>), List<Adt13>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2890 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f453(v0: Bytes, p2897: List<(Bytes, Adt14)>) -> Option<Adt14> {
    let m2896 = p2897;
    {
        let mut m2896 = m2896;
        loop {
            let step = {
                let v1 = m2896;
                let m2893 = v1;
                match m2893.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt14)>, Option<Adt14>>(None::<Adt14>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h2894, _) = v2;
                            h2894
                        }, v0.clone()) {
                            Err::<List<(Bytes, Adt14)>, Option<Adt14>>(Some({
                                let (_, h2895) = v2;
                                h2895
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt14)>, Option<Adt14>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2896 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f454() -> List<(Bytes, Adt14)> {
    List::cons((f8(Str::lit("boolean")), f283(f8(Str::lit("Bool")))), List::cons((f8(Str::lit("byte")), f283(f8(Str::lit("Int")))), List::cons((f8(Str::lit("octet")), f283(f8(Str::lit("Nat")))), List::cons((f8(Str::lit("bigint")), f283(f8(Str::lit("Int")))), List::cons((f8(Str::lit("float")), f283(f403())), List::cons((f8(Str::lit("double")), f283(f403())), List::cons((f8(Str::lit("DOMString")), f283(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("USVString")), f283(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ByteString")), f283(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ArrayBuffer")), f283(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("SharedArrayBuffer")), f283(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("DataView")), f283(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("Int8Array")), f283(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int16Array")), f283(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int32Array")), f283(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("BigInt64Array")), f283(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Uint8Array")), f283(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint16Array")), f283(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint32Array")), f283(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint8ClampedArray")), f283(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("BigUint64Array")), f283(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Float16Array")), f378(f8(Str::lit("Float16Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("Float32Array")), f378(f8(Str::lit("Float32Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("Float64Array")), f378(f8(Str::lit("Float64Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("any")), f378(f8(Str::lit("any is not translated, because it is the union of every Web IDL type and has no one LexLean type (Web IDL 2.13.1)")))), List::cons((f8(Str::lit("object")), f378(f8(Str::lit("object is not translated, because its values are references to JavaScript objects of any shape, which no LexLean type holds (Web IDL 2.13.20)")))), List::cons((f8(Str::lit("symbol")), f378(f8(Str::lit("symbol is not translated, because a symbol value has identity but no content that a LexLean type holds (Web IDL 2.13.21)")))), List::cons((f8(Str::lit("undefined")), f378(f8(Str::lit("undefined is not translated, because it is the type of the absence of a value (Web IDL 2.13.2)")))), List::<(Bytes, Adt14)>::nil()))))))))))))))))))))))))))))
}

pub fn f455() -> Bytes {
    f8(Str::lit("a union type is translated only as the whole body of a typedef, whose identifier names its inductive (Web IDL 2.11 and 2.13.32)"))
}

pub fn f456(v0: u64) -> Bytes {
    f32({
        let a2898 = Str::lit("0123456789ABCDEF");
        utf8_encode(a2898)
    }, v0, 1u64)
}

pub fn f457(p2909: u64, v1: Bytes, v2: Bytes, p2910: u64) -> R<bool> {
    let m2908 = (p2909, p2910);
    {
        let mut m2908 = m2908;
        loop {
            let step = {
                let (v0, v3) = m2908;
                let m2899 = v0;
                if m2899 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(false))
                } else {
                    let v4 = m2899 - 1;
                    let v5: u64 = f520(v1.clone(), v3)?;
                    let v6: u64 = f520(v2.clone(), v3)?;
                    let a2900 = v5;
                    let a2901 = v6;
                    if nat_lt(a2900, a2901) {
                        Err::<(u64, u64), R<bool>>(Ok(true))
                    } else {
                        let a2902 = v6;
                        let a2903 = v5;
                        if nat_lt(a2902, a2903) {
                            Err::<(u64, u64), R<bool>>(Ok(false))
                        } else {
                            let a2904 = v5;
                            let a2905 = 0u64;
                            if nat_eq(a2904, a2905) {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            } else {
                                Ok::<(u64, u64), R<bool>>((v4, {
                                    let a2906 = v3;
                                    let a2907 = 1u64;
                                    nat_add(a2906, a2907)?
                                }))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2908 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f458(p2923: u64, v1: Bytes, p2924: u64, p2925: Bytes) -> R<Option<Bytes>> {
    let m2922 = (p2923, (p2924, p2925));
    {
        let mut m2922 = m2922;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2922;
                let m2911 = v0;
                if m2911 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m2911 - 1;
                    let a2913 = {
                        let a2912 = v1.clone();
                        length_bytes(a2912)
                    };
                    let a2914 = v2;
                    if nat_le(a2913, a2914) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m2915 = f301(v1.clone(), v2)?;
                        match m2915 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a2917 = v2;
                                    let a2918 = {
                                        let (_, h2916) = v5;
                                        h2916
                                    };
                                    nat_add(a2917, a2918)?
                                }, {
                                    let a2920 = v3;
                                    let a2921 = f521(v1.clone(), v2, {
                                        let (h2919, _) = v5;
                                        h2919
                                    });
                                    append_bytes(a2920, a2921)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2922 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f459(p2935: u64, v1: Bytes, p2936: u64) -> R<bool> {
    let m2934 = (p2935, p2936);
    {
        let mut m2934 = m2934;
        loop {
            let step = {
                let (v0, v2) = m2934;
                let m2926 = v0;
                if m2926 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m2926 - 1;
                    let m2929 = {
                        let a2927 = v1.clone();
                        let a2928 = v2;
                        index_bytes(a2927, a2928)
                    };
                    match m2929 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a2930 = f42(v4)?;
                            let a2931 = 128u64;
                            if nat_lt(a2930, a2931) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a2932 = v2;
                                    let a2933 = 1u64;
                                    nat_add(a2932, a2933)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2934 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f460(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f220(v0.clone(), 0u64, {
        let a2937 = v0;
        length_bytes(a2937)
    }, false, Bytes::lit(&[]))?;
    let m2940 = {
        let a2938 = v1.clone();
        let a2939 = 0u64;
        index_bytes(a2938, a2939)
    };
    match m2940 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f42(v2)?;
            if f318(v3) {
                let a2947 = f32({
                    let a2941 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a2941)
                }, {
                    let a2942 = v3;
                    let a2943 = 65u64;
                    nat_sub(a2942, a2943)
                }, 1u64);
                let a2948 = f32(v1.clone(), 1u64, {
                    let a2945 = {
                        let a2944 = v1;
                        length_bytes(a2944)
                    };
                    let a2946 = 1u64;
                    nat_sub(a2945, a2946)
                });
                Ok(append_bytes(a2947, a2948))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f461(v0: Bytes) -> R<bool> {
    let m2951 = {
        let a2949 = v0;
        let a2950 = 0u64;
        index_bytes(a2949, a2950)
    };
    match m2951 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f317(f316(f42(v1)?), f318(f42(v1)?)))
        }
    }
}

pub fn f462(p2964: u64, v1: Bytes, p2965: u64, p2966: Bytes) -> R<Option<Bytes>> {
    let m2963 = (p2964, (p2965, p2966));
    {
        let mut m2963 = m2963;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2963;
                let m2952 = v0;
                if m2952 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m2952 - 1;
                    let a2954 = {
                        let a2953 = v1.clone();
                        length_bytes(a2953)
                    };
                    let a2955 = v2;
                    if nat_le(a2954, a2955) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m2956 = f301(v1.clone(), v2)?;
                        match m2956 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a2958 = v2;
                                    let a2959 = {
                                        let (_, h2957) = v5;
                                        h2957
                                    };
                                    nat_add(a2958, a2959)?
                                }, {
                                    let a2961 = v3;
                                    let a2962 = f522({
                                        let (h2960, _) = v5;
                                        h2960
                                    });
                                    append_bytes(a2961, a2962)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2963 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f463() -> List<Bytes> {
    List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("termination_by")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("admit")), List::cons(f8(Str::lit("below")), List::cons(f8(Str::lit("brecOn")), List::cons(f8(Str::lit("casesOn")), List::cons(f8(Str::lit("ctorElim")), List::cons(f8(Str::lit("ctorElimType")), List::cons(f8(Str::lit("ctorIdx")), List::cons(f8(Str::lit("noConfusion")), List::cons(f8(Str::lit("noConfusionType")), List::cons(f8(Str::lit("rec")), List::cons(f8(Str::lit("recOn")), List::cons(f8(Str::lit("toCtorIdx")), List::<Bytes>::nil())))))))))))))))))))
}

pub fn f464() -> Bytes {
    f8(Str::lit("\u{0}\u{0}:\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{5}\u{7f}\u{0}\u{6}p\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f465() -> Bytes {
    f8(Str::lit("\u{0}\u{0}-\u{0}\u{0}.\u{0}\u{0}0\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}7\u{0}\u{1}7\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@?\u{0}@@\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f466(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f87(v0, {
        let (h2967, _) = v2;
        h2967
    })?;
    let a2969 = 1114111u64;
    let a2970 = {
        let (_, h2968) = v2;
        h2968
    };
    if nat_lt(a2969, a2970) {
        Ok(None::<(u64, u64)>)
    } else {
        let v4: u64 = f523(v3, v1);
        let a2971 = v4;
        let a2972 = 16u64;
        if nat_eq(a2971, a2972) {
            Ok(None::<(u64, u64)>)
        } else {
            Ok(Some(({
                let a2974 = {
                    let (h2973, _) = v2;
                    h2973
                };
                let a2975 = 1u64;
                nat_add(a2974, a2975)?
            }, {
                let a2979 = {
                    let a2977 = {
                        let (_, h2976) = v2;
                        h2976
                    };
                    let a2978 = if v1 {
                        16u64
                    } else {
                        10u64
                    };
                    nat_mul(a2977, a2978)?
                };
                let a2980 = v4;
                nat_add(a2979, a2980)?
            })))
        }
    }
}

pub fn f467() -> Bytes {
    f8(Str::lit("\u{80}\u{81}\u{82}\u{83}\u{84}\u{85}\u{86}\u{87}\u{88}\u{89}\u{8a}\u{8b}\u{8c}\u{8d}\u{8e}\u{8f}\u{90}\u{91}\u{92}\u{93}\u{94}\u{95}\u{96}\u{97}\u{98}\u{99}\u{9a}\u{9b}\u{9c}\u{9d}\u{9e}\u{9f}\u{a0}\u{a1}\u{a2}\u{a3}\u{a4}\u{a5}\u{a6}\u{a7}\u{a8}\u{a9}\u{aa}\u{ab}\u{ac}\u{ad}\u{ae}\u{af}\u{b0}\u{b1}\u{b2}\u{b3}\u{b4}\u{b5}\u{b6}\u{b7}\u{b8}\u{b9}\u{ba}\u{bb}\u{bc}\u{bd}\u{be}\u{bf}"))
}

pub fn f468(v0: u64, v1: bool, v2: u64) -> R<bool> {
    if v1 {
        let a2981 = v0;
        let a2982 = v2;
        if nat_eq(a2981, a2982) {
            Ok(true)
        } else {
            f204(v0, f524())
        }
    } else {
        f204(v0, f525())
    }
}

pub fn f469() -> Bytes {
    f8(Str::lit(" string:anyAtomicType boolean:anyAtomicType decimal:anyAtomicType float:anyAtomicType double:anyAtomicType duration:anyAtomicType dateTime:anyAtomicType time:anyAtomicType date:anyAtomicType gYearMonth:anyAtomicType gYear:anyAtomicType gMonthDay:anyAtomicType gDay:anyAtomicType gMonth:anyAtomicType hexBinary:anyAtomicType base64Binary:anyAtomicType anyURI:anyAtomicType QName:anyAtomicType NOTATION:anyAtomicType normalizedString:string token:normalizedString language:token NMTOKEN:token Name:token NCName:Name ID:NCName IDREF:NCName ENTITY:NCName integer:decimal nonPositiveInteger:integer long:integer nonNegativeInteger:integer negativeInteger:nonPositiveInteger int:long short:int byte:short unsignedLong:nonNegativeInteger positiveInteger:nonNegativeInteger unsignedInt:unsignedLong unsignedShort:unsignedInt unsignedByte:unsignedShort yearMonthDuration:duration dayTimeDuration:duration dateTimeStamp:dateTime NMTOKENS:anySimpleType IDREFS:anySimpleType ENTITIES:anySimpleType anyAtomicType:anySimpleType "))
}

pub fn f470(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a2983 = v0.clone();
        length_bytes(a2983)
    };
    let v3: u64 = {
        let a2984 = v1.clone();
        length_bytes(a2984)
    };
    let a2985 = v3;
    let a2986 = v2;
    if nat_le(a2985, a2986) {
        let m2992 = {
            let a2989 = v0;
            let a2990 = {
                let a2987 = v2;
                let a2988 = v3;
                nat_sub(a2987, a2988)
            };
            let a2991 = v3;
            slice_bytes(a2989, a2990, a2991)
        };
        match m2992 {
            None => {
                false
            }
            Some(v4) => {
                let a2993 = v4;
                let a2994 = v1;
                equal(a2993, a2994)
            }
        }
    } else {
        false
    }
}

pub fn f471(v0: Adt25, v1: Str, v2: Str) -> bool {
    let m2995 = f238(v0, v1);
    match m2995 {
        None => {
            false
        }
        Some(v3) => {
            f35(v3, f8(v2))
        }
    }
}

pub fn f472(v0: List<Adt24>, v1: (Bytes, Bytes), v2: bool) -> R<Result<List<Bytes>, Bytes>> {
    let m2996 = v0;
    match m2996.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m2997 = f472(v4, v1.clone(), v2)?;
            match m2997 {
                Ok(v6) => {
                    if f35(match v3.clone() {
                        Adt24::C0(h2998, _, _, _) => {
                            h2998
                        }
                    }, f8(Str::lit("complexType"))) {
                        let m2999 = f526(v3.clone())?;
                        match m2999 {
                            Ok(v8) => {
                                let m3000 = v8;
                                match m3000 {
                                    None => {
                                        Ok(Ok::<List<Bytes>, Bytes>(v6))
                                    }
                                    Some(v9) => {
                                        if if f35({
                                            let (h3001, _) = {
                                                let (_, h3002) = v9.clone();
                                                h3002
                                            };
                                            h3001
                                        }, {
                                            let (h3003, _) = v1;
                                            h3003
                                        }) {
                                            if f35({
                                                let (_, h3004) = {
                                                    let (_, h3005) = v9;
                                                    h3005
                                                };
                                                h3004
                                            }, {
                                                let (_, h3006) = v1;
                                                h3006
                                            }) {
                                                if f236(v2) {
                                                    true
                                                } else {
                                                    f237({
                                                        let (h3007, _) = v9;
                                                        h3007
                                                    }, Str::lit("extension"))
                                                }
                                            } else {
                                                false
                                            }
                                        } else {
                                            false
                                        } {
                                            Ok(Ok::<List<Bytes>, Bytes>(List::cons(match v3 {
                                                Adt24::C0(_, _, h3008, _) => {
                                                    h3008
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

pub fn f473(v0: Bytes, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<Adt26, Bytes>> {
    let m3009 = v1.clone();
    match m3009.uncons() {
        None => {
            Ok(Err::<Adt26, Bytes>(f413(f8(Str::lit("a choice with no particle that can occur matches no element sequence, so it has no LexLean constructor (XSD 1.1 Part 1, section 3.8.4.3)")), v0)))
        }
        Some((_, _)) => {
            let m3010 = f527(v1)?;
            match m3010 {
                Ok(v7) => {
                    let v8: List<Bytes> = f528(v7, v2);
                    Ok(Ok::<Adt26, Bytes>(Adt26::C0(v0.clone(), v3, f194(v0, v8.clone()), v8)))
                }
                Err(v6) => {
                    Ok(Err::<Adt26, Bytes>(f413(v6, v0)))
                }
            }
        }
    }
}

pub fn f474(v0: List<Bytes>) -> R<List<Bytes>> {
    let m3011 = v0;
    match m3011.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f154(v1)?, f474(v2)?))
        }
    }
}

pub fn f475(v0: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: Adt25) -> R<Result<List<Adt26>, Bytes>> {
    let m3012 = v0;
    if m3012 == 0 {
        Ok(Err::<List<Adt26>, Bytes>(f413(f8(Str::lit("the complex types nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m3012 - 1;
        let m3013 = f529(v5, v1, v2.clone(), v3, v4, f416())?;
        match m3013 {
            Ok(v7) => {
                let m3015 = f483(v2, match v7.clone() {
                    Adt32::C0(h3014, _, _, _, _) => {
                        h3014
                    }
                })?;
                match m3015 {
                    Ok(v9) => {
                        Ok(Ok::<List<Adt26>, Bytes>(f324(match v7 {
                            Adt32::C0(_, h3016, _, _, _) => {
                                h3016
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

pub fn f476(v0: Bytes) -> Bytes {
    let a3017 = v0;
    let a3018 = f8(Str::lit("Content"));
    append_bytes(a3017, a3018)
}

pub fn f477(v0: Adt23, v1: Adt25, v2: Bytes) -> R<Result<Adt22, Bytes>> {
    let m3020 = f91(match v1 {
        Adt25::C0(_, _, _, _, h3019) => {
            h3019
        }
    }, v2.clone())?;
    match m3020 {
        Ok(v4) => {
            if f35({
                let (h3021, _) = v4.clone();
                h3021
            }, f151()) {
                if f35({
                    let (_, h3022) = v4.clone();
                    h3022
                }, f8(Str::lit("anyType"))) {
                    Ok(Ok::<Adt22, Bytes>(Adt22::C0(f327(), List::cons(f327(), List::<Bytes>::nil()))))
                } else {
                    let m3024 = f311({
                        let (_, h3023) = v4;
                        h3023
                    }, 16u64)?;
                    match m3024 {
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
                    let (h3025, _) = v4.clone();
                    h3025
                }, match v0.clone() {
                    Adt23::C0(_, h3026) => {
                        h3026
                    }
                }) {
                    let v7: Bytes = f154({
                        let (_, h3027) = v4.clone();
                        h3027
                    })?;
                    let m3028 = f484(v0.clone(), Str::lit("complexType"), v4.clone());
                    match m3028 {
                        None => {
                            let m3029 = f484(v0, Str::lit("simpleType"), v4);
                            match m3029 {
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
                        let (h3030, _) = v4;
                        h3030
                    }, f8(Str::lit(", which no schema document of this set targets (XSD 1.1 Part 1, sections 3.17.6.2 and 4.2.6)"))))))
                }
            }
        }
        Err(v3) => {
            Ok(Err::<Adt22, Bytes>(v3))
        }
    }
}

pub fn f478(v0: Adt25, v1: Bytes) -> Bytes {
    if f471(v0, Str::lit("nillable"), Str::lit("true")) {
        f530(Str::lit("Option"), v1)
    } else {
        v1
    }
}

pub fn f479(p3035: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: bool, p3036: List<Adt25>, p3037: Adt32) -> R<Result<Adt32, Bytes>> {
    let m3034 = (p3035, (p3036, p3037));
    {
        let mut m3034 = m3034;
        loop {
            let step = {
                let (v0, (v5, v6)) = m3034;
                let m3031 = v0;
                if m3031 == 0 {
                    Err::<(u64, (List<Adt25>, Adt32)), R<Result<Adt32, Bytes>>>(Ok(Err::<Adt32, Bytes>(f413(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2.clone()))))
                } else {
                    let v7 = m3031 - 1;
                    let m3032 = v5;
                    match m3032.uncons() {
                        None => {
                            Err::<(u64, (List<Adt25>, Adt32)), R<Result<Adt32, Bytes>>>(Ok(Ok::<Adt32, Bytes>(v6)))
                        }
                        Some((v8, v9)) => {
                            let m3033 = f531(v7, v1.clone(), v2.clone(), v3.clone(), v4, v8, v6)?;
                            match m3033 {
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
                Ok(next) => { m3034 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f480(v0: List<Adt33>) -> List<Bytes> {
    let m3038 = v0;
    match m3038.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt33::C0(h3039, _, _) => {
                    h3039
                }
            }, f480(v2))
        }
    }
}

pub fn f481(v0: List<Adt33>) -> List<Bytes> {
    let m3040 = v0;
    match m3040.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt33::C0(_, h3041, _) => {
                    h3041
                }
            }, f481(v2))
        }
    }
}

pub fn f482(v0: List<Adt33>) -> List<Bytes> {
    let m3042 = v0;
    match m3042.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f334(match v1 {
                Adt33::C0(_, _, h3043) => {
                    h3043
                }
            }, f482(v2))
        }
    }
}

pub fn f483(v0: Bytes, v1: List<Adt33>) -> R<Result<Adt26, Bytes>> {
    let m3044 = f527(f480(v1.clone()))?;
    match m3044 {
        Ok(v3) => {
            let v4: List<Bytes> = f481(v1.clone());
            let v7: Bytes = {
                let m3045 = v1.clone();
                match m3045.uncons() {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some((_, _)) => {
                        f163(f8(Str::lit("\u{a}")), f75(f532(v3, v4.clone()), f8(Str::lit(",\u{a}"))), f8(Str::lit("\u{a}  ")))
                    }
                }
            };
            Ok(Ok::<Adt26, Bytes>(Adt26::C0(v0.clone(), f482(v1), f163(f8(Str::lit("structure{\u{a}  name: ")), v0, f163(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), v7, f8(Str::lit("]\u{a}}\u{a}")))), List::cons(f163(f8(Str::lit("    {name: mk, fields: [")), f75(v4, f8(Str::lit(", "))), f8(Str::lit("]}"))), List::<Bytes>::nil()))))
        }
        Err(v2) => {
            Ok(Err::<Adt26, Bytes>(f413(v2, v0)))
        }
    }
}

pub fn f484(v0: Adt23, v1: Str, v2: (Bytes, Bytes)) -> Option<Adt24> {
    f533(match v0 {
        Adt23::C0(h3046, _) => {
            h3046
        }
    }, f8(v1), {
        let (h3047, _) = v2;
        h3047
    }, {
        let (_, h3048) = v2;
        h3048
    })
}

pub fn f485(v0: Adt32, v1: Bytes, v2: Adt31, v3: bool) -> Adt32 {
    f534(f486(v0, Adt33::C0(v1, if v3 {
        let m3051 = match v2.clone() {
            Adt31::C0(h3050, _) => {
                h3050
            }
        };
        match m3051 {
            Adt22::C0(h3049, _) => {
                h3049
            }
        }
    } else {
        f530(Str::lit("Option"), {
            let m3054 = match v2.clone() {
                Adt31::C0(h3053, _) => {
                    h3053
                }
            };
            match m3054 {
                Adt22::C0(h3052, _) => {
                    h3052
                }
            }
        })
    }, {
        let m3057 = match v2.clone() {
            Adt31::C0(h3056, _) => {
                h3056
            }
        };
        match m3057 {
            Adt22::C0(_, h3055) => {
                h3055
            }
        }
    })), match v2 {
        Adt31::C0(_, h3058) => {
            h3058
        }
    })
}

pub fn f486(v0: Adt32, v1: Adt33) -> Adt32 {
    Adt32::C0(f535(match v0.clone() {
        Adt32::C0(h3059, _, _, _, _) => {
            h3059
        }
    }, List::cons(v1, List::<Adt33>::nil())), match v0.clone() {
        Adt32::C0(_, h3060, _, _, _) => {
            h3060
        }
    }, match v0.clone() {
        Adt32::C0(_, _, h3061, _, _) => {
            h3061
        }
    }, match v0.clone() {
        Adt32::C0(_, _, _, h3062, _) => {
            h3062
        }
    }, match v0 {
        Adt32::C0(_, _, _, _, h3063) => {
            h3063
        }
    })
}

pub fn f487() -> Bytes {
    f8(Str::lit("(List ((Bytes * Bytes) * Bytes))"))
}

pub fn f488(v0: Bytes, v1: List<Bytes>, v2: Bytes) -> Bytes {
    f163(f8(Str::lit("inductive{\u{a}  name: ")), v0, f163(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), f75(v1, f8(Str::lit(",\u{a}"))), f163(f8(Str::lit("\u{a}  ],\u{a}  mutual: ")), v2, f8(Str::lit("\u{a}}\u{a}")))))
}

pub fn f489(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a3064 = 0u64;
    let a3065 = v1;
    if if nat_lt(a3064, a3065) {
        f176({
            let a3068 = f174(v0, {
                let a3066 = v1;
                let a3067 = 1u64;
                nat_sub(a3066, a3067)
            })?;
            let a3069 = 10u64;
            nat_eq(a3068, a3069)
        })
    } else {
        false
    } {
        Ok(Some({
            let a3070 = v1;
            let a3071 = 1u64;
            nat_sub(a3070, a3071)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f490(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f245(f174(v0, v1)?) {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3072 = v1;
            let a3073 = 1u64;
            nat_add(a3072, a3073)?
        }))
    }
}

pub fn f491(v0: Bytes, v1: u64, v2: u64) -> R<Adt34> {
    let v3: (u64, u64) = {
        let (h3074, _) = f394(Fn7::F536(v2, v0.clone()), {
            let a3076 = {
                let a3075 = v0.clone();
                length_bytes(a3075)
            };
            let a3077 = 1u64;
            nat_add(a3076, a3077)?
        }, (v1, 0u64))?;
        h3074
    };
    let v4: u64 = f492(v0.clone(), {
        let (h3078, _) = v3;
        h3078
    }, v2)?;
    let a3079 = f174(v0, v4)?;
    let a3080 = 9u64;
    if if nat_eq(a3079, a3080) {
        let a3084 = {
            let a3082 = v4;
            let a3083 = {
                let (h3081, _) = v3;
                h3081
            };
            nat_sub(a3082, a3083)
        };
        let a3085 = v2;
        nat_lt(a3084, a3085)
    } else {
        false
    } {
        Ok(Adt34::C1(f83(Str::lit("a tab cannot indent a block scalar line (YAML 1.2.2 [63], [70])"), v4)?))
    } else {
        Ok(Adt34::C0({
            let (_, h3086) = v3;
            h3086
        }, {
            let (h3087, _) = v3;
            h3087
        }))
    }
}

pub fn f492(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h3088, _) = f130(Fn4::F537(v2, v0.clone(), v1), {
        let a3090 = {
            let a3089 = v0;
            length_bytes(a3089)
        };
        let a3091 = 1u64;
        nat_add(a3090, a3091)?
    }, v1)?;
    Ok(h3088)
}

pub fn f493(v0: Bytes, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = f492(v0.clone(), v1, v2)?;
    let a3094 = {
        let a3092 = v3;
        let a3093 = v1;
        nat_sub(a3092, a3093)
    };
    let a3095 = v2;
    if nat_eq(a3094, a3095) {
        Ok(f176({
            let a3096 = f174(v0, v3)?;
            let a3097 = 256u64;
            nat_eq(a3096, a3097)
        }))
    } else {
        Ok(false)
    }
}

pub fn f494(v0: Bytes, p3107: u64, v2: u64, v3: bool, p3108: u64, p3109: Bytes, p3110: u64) -> R<Adt35> {
    let m3106 = (p3107, (p3108, (p3109, p3110)));
    {
        let mut m3106 = m3106;
        loop {
            let step = {
                let (v1, (v4, (v5, v6))) = m3106;
                let m3098 = v4;
                if m3098 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<Adt35>>(Ok(Adt35::C1(f83(Str::lit("a block scalar has more lines than the input"), v1)?)))
                } else {
                    let v7 = m3098 - 1;
                    let v8: u64 = f492(v0.clone(), v1, v2)?;
                    let v9: bool = f176(f244(f174(v0.clone(), v8)?));
                    let v10: u64 = f342(v0.clone(), v8)?;
                    let v11: Bytes = f7(List::cons(v5, List::cons(f430(v6), List::cons(f256(v0.clone(), v8, v10), List::<Bytes>::nil()))));
                    let m3099 = f491(v0.clone(), f248(v0.clone(), v10)?, v2)?;
                    match m3099 {
                        Adt34::C0(v13, v14) => {
                            let v15: u64 = v14;
                            let v16: u64 = f492(v0.clone(), v15, v2)?;
                            if f493(v0.clone(), v15, v2)? {
                                Ok::<(u64, (u64, (Bytes, u64))), R<Adt35>>((v15, (v7, ({
                                    let a3102 = v11;
                                    let a3103 = if if if v3 {
                                        v9
                                    } else {
                                        false
                                    } {
                                        f176(f244(f174(v0.clone(), v16)?))
                                    } else {
                                        false
                                    } {
                                        let a3100 = v13;
                                        let a3101 = 0u64;
                                        if nat_eq(a3100, a3101) {
                                            Bytes::lit(&[0x20])
                                        } else {
                                            Bytes::lit(&[])
                                        }
                                    } else {
                                        Bytes::lit(&[0x0a])
                                    };
                                    append_bytes(a3102, a3103)
                                }, v13))))
                            } else {
                                Err::<(u64, (u64, (Bytes, u64))), R<Adt35>>(Ok(Adt35::C0(v11, v13, {
                                    let a3104 = f174(v0.clone(), v16)?;
                                    let a3105 = 256u64;
                                    if nat_eq(a3104, a3105) {
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
                Ok(next) => { m3106 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f495(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f249(v0.clone(), {
        let (h3111, _) = v1;
        h3111
    })?;
    let a3112 = f174(v0, v2)?;
    let a3113 = 10u64;
    if nat_eq(a3112, a3113) {
        Ok(Some(({
            let a3114 = v2;
            let a3115 = 1u64;
            nat_add(a3114, a3115)?
        }, f432({
            let (_, h3116) = v1;
            h3116
        }, {
            let a3118 = v2;
            let a3119 = {
                let (h3117, _) = v1;
                h3117
            };
            nat_sub(a3118, a3119)
        }))))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f496(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f538(v0, v1)
}

pub fn f497(v0: u64) -> R<Option<u64>> {
    let v1: u64 = f255(f8(Str::lit("abtnvfr")), v0)?;
    let a3120 = v1;
    let a3121 = 7u64;
    if nat_lt(a3120, a3121) {
        Ok(Some({
            let a3122 = v1;
            let a3123 = 7u64;
            nat_add(a3122, a3123)?
        }))
    } else {
        let a3124 = v0;
        let a3125 = 48u64;
        if nat_eq(a3124, a3125) {
            Ok(Some(0u64))
        } else {
            let a3126 = v0;
            let a3127 = 9u64;
            if nat_eq(a3126, a3127) {
                Ok(Some(9u64))
            } else {
                let a3128 = v0;
                let a3129 = 101u64;
                if nat_eq(a3128, a3129) {
                    Ok(Some(27u64))
                } else {
                    let a3130 = v0;
                    let a3131 = 78u64;
                    if nat_eq(a3130, a3131) {
                        Ok(Some(133u64))
                    } else {
                        let a3132 = v0;
                        let a3133 = 95u64;
                        if nat_eq(a3132, a3133) {
                            Ok(Some(160u64))
                        } else {
                            let a3134 = v0;
                            let a3135 = 76u64;
                            if nat_eq(a3134, a3135) {
                                Ok(Some(8232u64))
                            } else {
                                let a3136 = v0;
                                let a3137 = 80u64;
                                if nat_eq(a3136, a3137) {
                                    Ok(Some(8233u64))
                                } else {
                                    let a3138 = f255(f8(Str::lit(" \"/\\")), v0)?;
                                    let a3139 = 4u64;
                                    if nat_lt(a3138, a3139) {
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

pub fn f498(v0: u64) -> R<Bytes> {
    f83(Str::lit("an implicit key must be on one line (YAML 1.2.2 [154])"), v0)
}

pub fn f499(v0: Bytes, p3154: u64, v2: u64, p3155: u64, p3156: u64) -> R<Adt34> {
    let m3153 = (p3154, (p3155, p3156));
    {
        let mut m3153 = m3153;
        loop {
            let step = {
                let (v1, (v3, v4)) = m3153;
                let m3140 = v3;
                if m3140 == 0 {
                    Err::<(u64, (u64, u64)), R<Adt34>>(Ok(Adt34::C1(f83(Str::lit("the input ends inside a scalar"), v1)?)))
                } else {
                    let v5 = m3140 - 1;
                    let v6: u64 = {
                        let a3141 = v1;
                        let a3142 = 1u64;
                        nat_add(a3141, a3142)?
                    };
                    let v7: u64 = f249(v0.clone(), v6)?;
                    let v8: u64 = f247(v0.clone(), v7)?;
                    let v9: u64 = f174(v0.clone(), v8)?;
                    if f172(v0.clone(), v6)? {
                        Err::<(u64, (u64, u64)), R<Adt34>>(Ok(Adt34::C1(f83(Str::lit("a document marker cannot appear inside a scalar (YAML 1.2.2 [206])"), v6)?)))
                    } else {
                        let a3143 = v9;
                        let a3144 = 10u64;
                        if nat_eq(a3143, a3144) {
                            Ok::<(u64, (u64, u64)), R<Adt34>>((v8, (v5, {
                                let a3145 = v4;
                                let a3146 = 1u64;
                                nat_add(a3145, a3146)?
                            })))
                        } else {
                            let a3147 = v9;
                            let a3148 = 256u64;
                            if nat_eq(a3147, a3148) {
                                Err::<(u64, (u64, u64)), R<Adt34>>(Ok(Adt34::C1(f83(Str::lit("the input ends inside a quoted scalar (YAML 1.2.2 [109], [120])"), v8)?)))
                            } else {
                                let a3151 = {
                                    let a3149 = v7;
                                    let a3150 = v6;
                                    nat_sub(a3149, a3150)
                                };
                                let a3152 = v2;
                                if nat_lt(a3151, a3152) {
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
                Ok(next) => { m3153 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f500(v0: Bytes, p3166: u64, p3167: u64, p3168: u64) -> R<Option<u64>> {
    let m3165 = (p3166, (p3167, p3168));
    {
        let mut m3165 = m3165;
        loop {
            let step = {
                let (v1, (v2, v3)) = m3165;
                let m3157 = v2;
                if m3157 == 0 {
                    Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3157 - 1;
                    let m3158 = f539(f174(v0.clone(), v1)?);
                    match m3158 {
                        None => {
                            Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(None::<u64>))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, u64)), R<Option<u64>>>(({
                                let a3159 = v1;
                                let a3160 = 1u64;
                                nat_add(a3159, a3160)?
                            }, (v4, {
                                let a3163 = {
                                    let a3161 = v3;
                                    let a3162 = 16u64;
                                    nat_mul(a3161, a3162)?
                                };
                                let a3164 = v5;
                                nat_add(a3163, a3164)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3165 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f501(v0: Bytes, v1: u64) -> R<u64> {
    let (h3169, _) = f130(Fn4::F540(v0.clone()), {
        let a3171 = {
            let a3170 = v0;
            length_bytes(a3170)
        };
        let a3172 = 1u64;
        nat_add(a3171, a3172)?
    }, v1)?;
    Ok(h3169)
}

pub fn f502(v0: Bytes, v1: u64) -> R<u64> {
    let (h3173, _) = f130(Fn4::F541(v0.clone()), {
        let a3175 = {
            let a3174 = v0;
            length_bytes(a3174)
        };
        let a3176 = 1u64;
        nat_add(a3175, a3176)?
    }, v1)?;
    Ok(h3173)
}

pub fn f503(v0: Fn8, p3181: u64, p3182: (u64, bool)) -> R<((u64, bool), bool)> {
    let m3180 = (p3181, p3182);
    {
        let mut m3180 = m3180;
        loop {
            let step = {
                let (v1, v2) = m3180;
                let m3177 = v1;
                if m3177 == 0 {
                    Err::<(u64, (u64, bool)), R<((u64, bool), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3177 - 1;
                    let m3179 = {
                        let c3178: Fn8 = v0.clone();
                        c3178.apply(v2)?
                    };
                    match m3179 {
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
                Ok(next) => { m3180 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f504(v0: u64, v1: Bytes, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    f542(v1, v0, v2)
}

pub fn f505(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = {
        let (h3183, _) = v2;
        h3183
    };
    let v4: u64 = f174(v0.clone(), v3)?;
    if f245(v4) {
        Ok(None::<(u64, u64)>)
    } else {
        if f244(v4) {
            Ok(Some(({
                let a3184 = v3;
                let a3185 = 1u64;
                nat_add(a3184, a3185)?
            }, {
                let (_, h3186) = v2;
                h3186
            })))
        } else {
            let a3187 = v4;
            let a3188 = 58u64;
            if if nat_eq(a3187, a3188) {
                f176(f344(f174(v0.clone(), {
                    let a3189 = v3;
                    let a3190 = 1u64;
                    nat_add(a3189, a3190)?
                })?, v1))
            } else {
                false
            } {
                Ok(None::<(u64, u64)>)
            } else {
                let a3191 = v4;
                let a3192 = 35u64;
                if if nat_eq(a3191, a3192) {
                    f244(f174(v0, {
                        let a3193 = v3;
                        let a3194 = 1u64;
                        nat_sub(a3193, a3194)
                    })?)
                } else {
                    false
                } {
                    Ok(None::<(u64, u64)>)
                } else {
                    if if v1 {
                        f425(v4)
                    } else {
                        false
                    } {
                        Ok(None::<(u64, u64)>)
                    } else {
                        Ok(Some(({
                            let a3195 = v3;
                            let a3196 = 1u64;
                            nat_add(a3195, a3196)?
                        }, {
                            let a3197 = v3;
                            let a3198 = 1u64;
                            nat_add(a3197, a3198)?
                        })))
                    }
                }
            }
        }
    }
}

pub fn f506(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f543(v0, v1)
}

pub fn f507(v0: Bytes, v1: u64, v2: u64) -> R<(u64, u64)> {
    let (h3199, _) = f394(Fn7::F544(v1, v0.clone()), {
        let a3201 = {
            let a3200 = v0;
            length_bytes(a3200)
        };
        let a3202 = 1u64;
        nat_add(a3201, a3202)?
    }, (v2, 0u64))?;
    Ok(h3199)
}

pub fn f508(v0: bool, v1: u64) -> bool {
    if v0 {
        let a3203 = 0u64;
        let a3204 = v1;
        if nat_lt(a3203, a3204) {
            let a3205 = v1;
            let a3206 = 9223372036854775808u64;
            nat_le(a3205, a3206)
        } else {
            false
        }
    } else {
        let a3207 = v1;
        let a3208 = 9223372036854775807u64;
        nat_le(a3207, a3208)
    }
}

pub fn f509(v0: Bytes, v1: u64) -> R<u64> {
    let (h3209, _) = f130(Fn4::F545(v0.clone()), {
        let a3211 = {
            let a3210 = v0;
            length_bytes(a3210)
        };
        let a3212 = 1u64;
        nat_add(a3211, a3212)?
    }, v1)?;
    Ok(h3209)
}

pub fn f510(v0: Bytes) -> R<u64> {
    let (h3213, _) = f130(Fn4::F546(v0.clone()), {
        let a3215 = {
            let a3214 = v0;
            length_bytes(a3214)
        };
        let a3216 = 1u64;
        nat_add(a3215, a3216)?
    }, 0u64)?;
    Ok(h3213)
}

pub fn f511(v0: Bytes) -> R<Option<u64>> {
    if f547(v0.clone())? {
        Ok(Some({
            let (_, h3217) = {
                let (h3218, _) = f394(Fn7::F548(v0.clone()), {
                    let a3219 = v0;
                    length_bytes(a3219)
                }, (0u64, 0u64))?;
                h3218
            };
            h3217
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f512(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f32(v1.clone(), 0u64, {
        let a3221 = {
            let a3220 = v1.clone();
            length_bytes(a3220)
        };
        let a3222 = f549(v1)?;
        nat_sub(a3221, a3222)
    });
    let v5: Bytes = {
        let a3223 = v0;
        let a3224 = v4.clone();
        append_bytes(a3223, a3224)
    };
    let v6: u64 = f510(v5.clone())?;
    let v7: Bytes = f32(v5.clone(), v6, {
        let a3226 = {
            let a3225 = v5;
            length_bytes(a3225)
        };
        let a3227 = v6;
        nat_sub(a3226, a3227)
    });
    let v8: u64 = {
        let a3228 = v4;
        length_bytes(a3228)
    };
    let v9: u64 = {
        let a3229 = v7.clone();
        length_bytes(a3229)
    };
    let a3230 = v9;
    let a3231 = 0u64;
    if nat_eq(a3230, a3231) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f317(v2, {
            let a3232 = v3;
            let a3233 = v8;
            nat_lt(a3232, a3233)
        }) {
            let v10: u64 = if v2 {
                let a3234 = v3;
                let a3235 = v8;
                nat_add(a3234, a3235)?
            } else {
                let a3236 = v8;
                let a3237 = v3;
                nat_sub(a3236, a3237)
            };
            let v11: u64 = f549(v7.clone())?;
            let a3238 = v10;
            let a3239 = v11;
            if nat_le(a3238, a3239) {
                Ok(Some((f32(v7, 0u64, {
                    let a3240 = v9;
                    let a3241 = v10;
                    nat_sub(a3240, a3241)
                }), 0u64)))
            } else {
                let a3244 = {
                    let a3242 = v10;
                    let a3243 = v11;
                    nat_sub(a3242, a3243)
                };
                let a3245 = 19u64;
                if nat_le(a3244, a3245) {
                    Ok(Some((f32(v7, 0u64, {
                        let a3246 = v9;
                        let a3247 = v11;
                        nat_sub(a3246, a3247)
                    }), {
                        let a3248 = v10;
                        let a3249 = v11;
                        nat_sub(a3248, a3249)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a3250 = v3;
                let a3251 = v8;
                nat_sub(a3250, a3251)
            };
            let a3254 = {
                let a3252 = v9;
                let a3253 = v12;
                nat_add(a3252, a3253)?
            };
            let a3255 = 19u64;
            if nat_le(a3254, a3255) {
                Ok(Some(({
                    let a3256 = v7;
                    let a3257 = f354(v12);
                    append_bytes(a3256, a3257)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f513(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m3261 = {
        let a3259 = v0.clone();
        let a3260 = {
            let (h3258, _) = v1.clone();
            h3258
        };
        index_bytes(a3259, a3260)
    };
    match m3261 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a3263 = {
                    let (h3262, _) = v1.clone();
                    h3262
                };
                let a3264 = 1u64;
                nat_add(a3263, a3264)?
            }, {
                let a3267 = {
                    let (_, h3265) = v1;
                    h3265
                };
                let a3268 = f550(v0, {
                    let (h3266, _) = v1;
                    h3266
                }, v2)?;
                append_bytes(a3267, a3268)
            })))
        }
    }
}

pub fn f514(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f551(Fn11::F552, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f515(v0: Fn9, p3272: List<List<(Bytes, Bytes)>>, p3273: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m3271 = (p3272, p3273);
    {
        let mut m3271 = m3271;
        loop {
            let step = {
                let (v1, v2) = m3271;
                let m3269 = v2;
                match m3269.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c3270: Fn9 = v0.clone();
                            c3270.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3271 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f516(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f517(v0: Fn10, p3278: u64, p3279: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m3277 = (p3278, p3279);
    {
        let mut m3277 = m3277;
        loop {
            let step = {
                let (v1, v2) = m3277;
                let m3274 = v1;
                if m3274 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3274 - 1;
                    let m3276 = {
                        let c3275: Fn10 = v0.clone();
                        c3275.apply(v2.clone())?
                    };
                    match m3276 {
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
                Ok(next) => { m3277 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f518(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f553(v0)
}

pub fn f519(v0: u64, v1: u64) -> bool {
    let a3280 = v0;
    let a3281 = 0u64;
    if nat_eq(a3280, a3281) {
        f364(v1)
    } else {
        let a3282 = v0;
        let a3283 = 1u64;
        if nat_eq(a3282, a3283) {
            f363(v1)
        } else {
            let a3284 = v0;
            let a3285 = 2u64;
            if nat_eq(a3284, a3285) {
                f554(v1)
            } else {
                f555(v1)
            }
        }
    }
}

pub fn f520(v0: Bytes, v1: u64) -> R<u64> {
    let a3287 = v1;
    let a3288 = {
        let a3286 = v0.clone();
        length_bytes(a3286)
    };
    if nat_lt(a3287, a3288) {
        let a3289 = f87(v0, v1)?;
        let a3290 = 1u64;
        nat_add(a3289, a3290)
    } else {
        Ok(0u64)
    }
}

pub fn f521(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f317(f315(f317(f316(v2), f318(v2)), f315({
        let a3291 = v2;
        let a3292 = 117u64;
        !nat_eq(a3291, a3292)
    }, {
        let a3293 = v2;
        let a3294 = 85u64;
        !nat_eq(a3293, a3294)
    })), f315(f319(v2), {
        let a3295 = 0u64;
        let a3296 = v1;
        nat_lt(a3295, a3296)
    })) {
        f32(v0, v1, 1u64)
    } else {
        let a3297 = v2;
        let a3298 = 65536u64;
        if nat_lt(a3297, a3298) {
            let a3299 = f8(Str::lit("u"));
            let a3300 = f382(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a3299, a3300)
        } else {
            let a3301 = f8(Str::lit("U"));
            let a3302 = f382(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a3301, a3302)
        }
    }
}

pub fn f522(v0: u64) -> Bytes {
    let a3303 = f8(Str::lit("u"));
    let a3304 = f382(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a3303, a3304)
}

pub fn f523(v0: u64, v1: bool) -> u64 {
    let a3305 = v0;
    let a3306 = 48u64;
    if nat_lt(a3305, a3306) {
        16u64
    } else {
        let a3307 = v0;
        let a3308 = 57u64;
        if nat_le(a3307, a3308) {
            let a3309 = v0;
            let a3310 = 48u64;
            nat_sub(a3309, a3310)
        } else {
            if v1 {
                let a3311 = v0;
                let a3312 = 65u64;
                if nat_lt(a3311, a3312) {
                    16u64
                } else {
                    let a3313 = v0;
                    let a3314 = 70u64;
                    if nat_le(a3313, a3314) {
                        let a3315 = v0;
                        let a3316 = 55u64;
                        nat_sub(a3315, a3316)
                    } else {
                        let a3317 = v0;
                        let a3318 = 97u64;
                        if nat_lt(a3317, a3318) {
                            16u64
                        } else {
                            let a3319 = v0;
                            let a3320 = 102u64;
                            if nat_le(a3319, a3320) {
                                let a3321 = v0;
                                let a3322 = 87u64;
                                nat_sub(a3321, a3322)
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

pub fn f524() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f525() -> Bytes {
    f8(Str::lit("\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{0}]\u{0}\u{0}]\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f526(v0: Adt24) -> R<Result<Option<(Adt25, (Bytes, Bytes))>, Bytes>> {
    let m3324 = f411(match v0 {
        Adt24::C0(_, _, _, h3323) => {
            h3323
        }
    })?;
    match m3324 {
        Ok(v2) => {
            let m3325 = f412(v2, List::cons(f8(Str::lit("simpleContent")), List::cons(f8(Str::lit("complexContent")), List::<Bytes>::nil())));
            match m3325 {
                None => {
                    Ok(Ok::<Option<(Adt25, (Bytes, Bytes))>, Bytes>(None::<(Adt25, (Bytes, Bytes))>))
                }
                Some(v3) => {
                    let m3326 = f411(v3)?;
                    match m3326 {
                        Ok(v5) => {
                            let m3327 = f412(v5, List::cons(f8(Str::lit("extension")), List::cons(f8(Str::lit("restriction")), List::<Bytes>::nil())));
                            match m3327 {
                                None => {
                                    Ok(Ok::<Option<(Adt25, (Bytes, Bytes))>, Bytes>(None::<(Adt25, (Bytes, Bytes))>))
                                }
                                Some(v6) => {
                                    let m3328 = f238(v6.clone(), Str::lit("base"));
                                    match m3328 {
                                        None => {
                                            Ok(Ok::<Option<(Adt25, (Bytes, Bytes))>, Bytes>(None::<(Adt25, (Bytes, Bytes))>))
                                        }
                                        Some(v7) => {
                                            let m3330 = f91(match v6.clone() {
                                                Adt25::C0(_, _, _, _, h3329) => {
                                                    h3329
                                                }
                                            }, v7)?;
                                            match m3330 {
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

pub fn f527(v0: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f556(f188(v0.clone())?, v0, List::<Bytes>::nil())
}

pub fn f528(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m3331 = v0;
    match m3331.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m3332 = v1;
            match m3332.uncons() {
                None => {
                    List::<Bytes>::nil()
                }
                Some((v4, v5)) => {
                    List::cons(f163(f8(Str::lit("    {name: ")), v2, f163(f8(Str::lit(", fields: [")), v4, f8(Str::lit("]}")))), f528(v3, v5))
                }
            }
        }
    }
}

pub fn f529(v0: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: Adt25, v5: Adt32) -> R<Result<Adt32, Bytes>> {
    let m3333 = v0;
    if m3333 == 0 {
        Ok(Err::<Adt32, Bytes>(f413(f8(Str::lit("the type derivations nest deeper than the schema documents have bytes, so the types derive from each other in a cycle (XSD 1.1 Part 1, section 3.4.6.1)")), v2)))
    } else {
        let v6 = m3333 - 1;
        if f471(v4.clone(), Str::lit("mixed"), Str::lit("true")) {
            Ok(Err::<Adt32, Bytes>(f413(f8(Str::lit("a mixed content type interleaves character data with its elements, which a LexLean field list does not keep in order, so it is not translated (XSD 1.1 Part 1, section 3.4.2.3)")), v2)))
        } else {
            let m3334 = f411(v4)?;
            match m3334 {
                Ok(v8) => {
                    let m3335 = f412(v8.clone(), List::cons(f8(Str::lit("openContent")), List::cons(f8(Str::lit("assert")), List::<Bytes>::nil())));
                    match m3335 {
                        None => {
                            let m3337 = f412(v8.clone(), List::cons(f8(Str::lit("simpleContent")), List::cons(f8(Str::lit("complexContent")), List::<Bytes>::nil())));
                            match m3337 {
                                None => {
                                    let m3338 = f479(v6, v1.clone(), v2.clone(), v3.clone(), true, v8.clone(), v5)?;
                                    match m3338 {
                                        Ok(v11) => {
                                            f415(v6, v1, v2, v3, v8, v11)
                                        }
                                        Err(v10) => {
                                            Ok(Err::<Adt32, Bytes>(v10))
                                        }
                                    }
                                }
                                Some(v12) => {
                                    if f471(v12.clone(), Str::lit("mixed"), Str::lit("true")) {
                                        Ok(Err::<Adt32, Bytes>(f413(f8(Str::lit("a mixed content type interleaves character data with its elements, which a LexLean field list does not keep in order, so it is not translated (XSD 1.1 Part 1, section 3.4.2.3)")), v2)))
                                    } else {
                                        let m3339 = f411(v12.clone())?;
                                        match m3339 {
                                            Ok(v14) => {
                                                let m3340 = f412(v14, List::cons(f8(Str::lit("extension")), List::cons(f8(Str::lit("restriction")), List::<Bytes>::nil())));
                                                match m3340 {
                                                    None => {
                                                        Ok(Err::<Adt32, Bytes>(f413(f8(Str::lit("simpleContent and complexContent must have an extension or a restriction child (XSD 1.1 Part 1, sections 3.4.2.2 and 3.4.2.3)")), v2)))
                                                    }
                                                    Some(v15) => {
                                                        let m3341 = f238(v15.clone(), Str::lit("base"));
                                                        match m3341 {
                                                            None => {
                                                                Ok(Err::<Adt32, Bytes>(f413(f8(Str::lit("an extension or restriction must name its base type (XSD 1.1 Part 1, section 3.4.3)")), v2)))
                                                            }
                                                            Some(v16) => {
                                                                let m3343 = f91(match v15.clone() {
                                                                    Adt25::C0(_, _, _, _, h3342) => {
                                                                        h3342
                                                                    }
                                                                }, v16.clone())?;
                                                                match m3343 {
                                                                    Ok(v18) => {
                                                                        let m3344 = f411(v15.clone())?;
                                                                        match m3344 {
                                                                            Ok(v20) => {
                                                                                let v21: bool = if f35({
                                                                                    let (h3345, _) = v18.clone();
                                                                                    h3345
                                                                                }, f151()) {
                                                                                    f35({
                                                                                        let (_, h3346) = v18.clone();
                                                                                        h3346
                                                                                    }, f8(Str::lit("anyType")))
                                                                                } else {
                                                                                    false
                                                                                };
                                                                                if if f237(v15.clone(), Str::lit("restriction")) {
                                                                                    f236(v21)
                                                                                } else {
                                                                                    false
                                                                                } {
                                                                                    Ok(Err::<Adt32, Bytes>(f413(f163(f8(Str::lit("a restriction of the complex type ")), v16, f8(Str::lit(" is not translated, because the attribute uses it inherits and the content it narrows are not read yet (XSD 1.1 Part 1, sections 3.4.2.2 and 3.4.2.3)"))), v2)))
                                                                                } else {
                                                                                    let m3361 = if if v21 {
                                                                                        true
                                                                                    } else {
                                                                                        f35(match v12 {
                                                                                            Adt25::C0(_, h3347, _, _, _) => {
                                                                                                h3347
                                                                                            }
                                                                                        }, f8(Str::lit("complexContent")))
                                                                                    } {
                                                                                        if v21 {
                                                                                            Ok::<Adt32, Bytes>(v5)
                                                                                        } else {
                                                                                            let m3348 = f484(v1.clone(), Str::lit("complexType"), v18.clone());
                                                                                            match m3348 {
                                                                                                None => {
                                                                                                    Err::<Adt32, Bytes>(f413(f163(f8(Str::lit("the base type ")), v16, f8(Str::lit(" of a complex content extension is not a complex type of this schema set (XSD 1.1 Part 1, section 3.4.2.3)"))), v2.clone()))
                                                                                                }
                                                                                                Some(v22) => {
                                                                                                    let m3352 = f529(v6, v1.clone(), f154({
                                                                                                        let (_, h3349) = v18;
                                                                                                        h3349
                                                                                                    })?, match v22.clone() {
                                                                                                        Adt24::C0(_, h3350, _, _) => {
                                                                                                            h3350
                                                                                                        }
                                                                                                    }, match v22 {
                                                                                                        Adt24::C0(_, _, _, h3351) => {
                                                                                                            h3351
                                                                                                        }
                                                                                                    }, v5)?;
                                                                                                    match m3352 {
                                                                                                        Ok(v24) => {
                                                                                                            Ok::<Adt32, Bytes>(f557(v24))
                                                                                                        }
                                                                                                        Err(v23) => {
                                                                                                            Err::<Adt32, Bytes>(v23)
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        let m3353 = f484(v1.clone(), Str::lit("complexType"), v18.clone());
                                                                                        match m3353 {
                                                                                            None => {
                                                                                                let m3358 = f477(v1.clone(), v15, v16)?;
                                                                                                match m3358 {
                                                                                                    Ok(v29) => {
                                                                                                        Ok::<Adt32, Bytes>(f486(v5, Adt33::C0(f8(Str::lit("value")), match v29.clone() {
                                                                                                            Adt22::C0(h3359, _) => {
                                                                                                                h3359
                                                                                                            }
                                                                                                        }, match v29 {
                                                                                                            Adt22::C0(_, h3360) => {
                                                                                                                h3360
                                                                                                            }
                                                                                                        })))
                                                                                                    }
                                                                                                    Err(v28) => {
                                                                                                        Err::<Adt32, Bytes>(f413(v28, v2.clone()))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            Some(v25) => {
                                                                                                let m3357 = f529(v6, v1.clone(), f154({
                                                                                                    let (_, h3354) = v18;
                                                                                                    h3354
                                                                                                })?, match v25.clone() {
                                                                                                    Adt24::C0(_, h3355, _, _) => {
                                                                                                        h3355
                                                                                                    }
                                                                                                }, match v25 {
                                                                                                    Adt24::C0(_, _, _, h3356) => {
                                                                                                        h3356
                                                                                                    }
                                                                                                }, v5)?;
                                                                                                match m3357 {
                                                                                                    Ok(v27) => {
                                                                                                        Ok::<Adt32, Bytes>(f557(v27))
                                                                                                    }
                                                                                                    Err(v26) => {
                                                                                                        Err::<Adt32, Bytes>(v26)
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    };
                                                                                    match m3361 {
                                                                                        Ok(v31) => {
                                                                                            let m3362 = f479(v6, v1.clone(), v2.clone(), v3.clone(), true, v20.clone(), v31)?;
                                                                                            match m3362 {
                                                                                                Ok(v33) => {
                                                                                                    f415(v6, v1, v2, v3, v20, v33)
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
                                                                        Ok(Err::<Adt32, Bytes>(f413(v17, v2)))
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
                            Ok(Err::<Adt32, Bytes>(f413(f163(f8(Str::lit("xs:")), match v9 {
                                Adt25::C0(_, h3336, _, _, _) => {
                                    h3336
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

pub fn f530(v0: Str, v1: Bytes) -> Bytes {
    f163(f8(Str::lit("(")), f8(v0), f163(f8(Str::lit(" ")), v1, f8(Str::lit(")"))))
}

pub fn f531(v0: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: bool, v5: Adt25, v6: Adt32) -> R<Result<Adt32, Bytes>> {
    let m3363 = v0;
    if m3363 == 0 {
        Ok(Err::<Adt32, Bytes>(f413(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v7 = m3363 - 1;
        if f237(v5.clone(), Str::lit("element")) {
            let m3364 = f558(v7, v1, v2, v3, v5.clone())?;
            match m3364 {
                Ok(v9) => {
                    let m3370 = f559(v5, {
                        let m3369 = {
                            let m3368 = match v9.clone() {
                                Adt36::C0(_, h3367) => {
                                    h3367
                                }
                            };
                            match m3368 {
                                Adt31::C0(h3366, _) => {
                                    h3366
                                }
                            }
                        };
                        match m3369 {
                            Adt22::C0(h3365, _) => {
                                h3365
                            }
                        }
                    });
                    match m3370 {
                        None => {
                            Ok(Ok::<Adt32, Bytes>(v6))
                        }
                        Some(v10) => {
                            Ok(Ok::<Adt32, Bytes>(f534(f486(v6, Adt33::C0(match v9.clone() {
                                Adt36::C0(h3371, _) => {
                                    h3371
                                }
                            }, v10, {
                                let m3376 = {
                                    let m3375 = match v9.clone() {
                                        Adt36::C0(_, h3374) => {
                                            h3374
                                        }
                                    };
                                    match m3375 {
                                        Adt31::C0(h3373, _) => {
                                            h3373
                                        }
                                    }
                                };
                                match m3376 {
                                    Adt22::C0(_, h3372) => {
                                        h3372
                                    }
                                }
                            })), {
                                let m3379 = match v9 {
                                    Adt36::C0(_, h3378) => {
                                        h3378
                                    }
                                };
                                match m3379 {
                                    Adt31::C0(_, h3377) => {
                                        h3377
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
            if f237(v5.clone(), Str::lit("any")) {
                let m3380 = f559(v5, f327());
                match m3380 {
                    None => {
                        Ok(Ok::<Adt32, Bytes>(v6))
                    }
                    Some(v11) => {
                        Ok(Ok::<Adt32, Bytes>(f486(v6, Adt33::C0(f8(Str::lit("any")), v11, List::cons(f327(), List::<Bytes>::nil())))))
                    }
                }
            } else {
                if f237(v5.clone(), Str::lit("group")) {
                    let m3381 = f238(v5.clone(), Str::lit("ref"));
                    match m3381 {
                        None => {
                            Ok(Err::<Adt32, Bytes>(f413(f8(Str::lit("a model group reference must have a ref (XSD 1.1 Part 1, section 3.7.2)")), v2)))
                        }
                        Some(v12) => {
                            let m3383 = f91(match v5.clone() {
                                Adt25::C0(_, _, _, _, h3382) => {
                                    h3382
                                }
                            }, v12.clone())?;
                            match m3383 {
                                Ok(v14) => {
                                    let m3384 = f484(v1, Str::lit("group"), v14.clone());
                                    match m3384 {
                                        None => {
                                            Ok(Err::<Adt32, Bytes>(f413(f163(f8(Str::lit("the model group ")), v12, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                        }
                                        Some(_) => {
                                            let v16: Bytes = f154({
                                                let (_, h3385) = v14.clone();
                                                h3385
                                            })?;
                                            let m3386 = f559(v5, v16.clone());
                                            match m3386 {
                                                None => {
                                                    Ok(Ok::<Adt32, Bytes>(v6))
                                                }
                                                Some(v17) => {
                                                    Ok(Ok::<Adt32, Bytes>(f486(v6, Adt33::C0({
                                                        let (_, h3387) = v14;
                                                        h3387
                                                    }, v17, List::cons(v16, List::<Bytes>::nil())))))
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(v13) => {
                                    Ok(Err::<Adt32, Bytes>(f413(v13, v2)))
                                }
                            }
                        }
                    }
                } else {
                    if if f237(v5.clone(), Str::lit("sequence")) {
                        true
                    } else {
                        if f237(v5.clone(), Str::lit("choice")) {
                            true
                        } else {
                            f237(v5.clone(), Str::lit("all"))
                        }
                    } {
                        if if v4 {
                            if f560(v5.clone()) {
                                f236(f237(v5.clone(), Str::lit("choice")))
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m3388 = f411(v5)?;
                            match m3388 {
                                Ok(v19) => {
                                    f479(v7, v1, v2, v3, true, v19, v6)
                                }
                                Err(v18) => {
                                    Ok(Err::<Adt32, Bytes>(v18))
                                }
                            }
                        } else {
                            let v20: Bytes = match v5.clone() {
                                Adt25::C0(_, h3389, _, _, _) => {
                                    h3389
                                }
                            };
                            let v21: u64 = f561(v6.clone(), v20.clone())?;
                            let v22: Bytes = {
                                let a3390 = v21;
                                let a3391 = 1u64;
                                if nat_eq(a3390, a3391) {
                                    Bytes::lit(&[])
                                } else {
                                    f52(v21)?
                                }
                            };
                            let v23: Bytes = f163(v2, f154(v20.clone())?, v22.clone());
                            let m3392 = f414(v7, v1, v23.clone(), v3, v5.clone())?;
                            match m3392 {
                                Ok(v25) => {
                                    let m3393 = f559(v5, v23.clone());
                                    match m3393 {
                                        None => {
                                            Ok(Ok::<Adt32, Bytes>(f562(v6, v20)?))
                                        }
                                        Some(v26) => {
                                            Ok(Ok::<Adt32, Bytes>(f534(f486(f562(v6, v20.clone())?, Adt33::C0({
                                                let a3394 = v20;
                                                let a3395 = v22;
                                                append_bytes(a3394, a3395)
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

pub fn f532(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m3396 = v0;
    match m3396.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m3397 = v1;
            match m3397.uncons() {
                None => {
                    List::<Bytes>::nil()
                }
                Some((v4, v5)) => {
                    List::cons(f163(f8(Str::lit("    {name: ")), v2, f163(f8(Str::lit(", type: ")), v4, f8(Str::lit("}")))), f532(v3, v5))
                }
            }
        }
    }
}

pub fn f533(p3403: List<Adt24>, v1: Bytes, v2: Bytes, v3: Bytes) -> Option<Adt24> {
    let m3402 = p3403;
    {
        let mut m3402 = m3402;
        loop {
            let step = {
                let v0 = m3402;
                let m3398 = v0;
                match m3398.uncons() {
                    None => {
                        Err::<List<Adt24>, Option<Adt24>>(None::<Adt24>)
                    }
                    Some((v4, v5)) => {
                        if if f35(match v4.clone() {
                            Adt24::C0(h3399, _, _, _) => {
                                h3399
                            }
                        }, v1.clone()) {
                            if f35(match v4.clone() {
                                Adt24::C0(_, h3400, _, _) => {
                                    h3400
                                }
                            }, v2.clone()) {
                                f35(match v4.clone() {
                                    Adt24::C0(_, _, h3401, _) => {
                                        h3401
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
                Ok(next) => { m3402 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f534(v0: Adt32, v1: List<Adt26>) -> Adt32 {
    Adt32::C0(match v0.clone() {
        Adt32::C0(h3404, _, _, _, _) => {
            h3404
        }
    }, f324(match v0.clone() {
        Adt32::C0(_, h3405, _, _, _) => {
            h3405
        }
    }, v1), match v0.clone() {
        Adt32::C0(_, _, h3406, _, _) => {
            h3406
        }
    }, match v0.clone() {
        Adt32::C0(_, _, _, h3407, _) => {
            h3407
        }
    }, match v0 {
        Adt32::C0(_, _, _, _, h3408) => {
            h3408
        }
    })
}

pub fn f535(v0: List<Adt33>, v1: List<Adt33>) -> List<Adt33> {
    let m3409 = v0;
    match m3409.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f535(v3, v1))
        }
    }
}

pub fn f536(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f563(v1, v0, v2)
}

pub fn f537(v0: u64, v1: Bytes, v2: u64, v3: u64) -> R<Option<u64>> {
    f564(v1, v2, v0, v3)
}

pub fn f538(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let a3412 = {
        let (h3410, _) = v1;
        h3410
    };
    let a3413 = {
        let a3411 = v0.clone();
        length_bytes(a3411)
    };
    if nat_lt(a3412, a3413) {
        Ok(Some((f248(v0, {
            let (h3414, _) = v1;
            h3414
        })?, {
            let a3416 = {
                let (_, h3415) = v1;
                h3415
            };
            let a3417 = 1u64;
            nat_add(a3416, a3417)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f539(v0: u64) -> Option<u64> {
    let a3418 = 48u64;
    let a3419 = v0;
    if if nat_le(a3418, a3419) {
        let a3420 = v0;
        let a3421 = 57u64;
        nat_le(a3420, a3421)
    } else {
        false
    } {
        Some({
            let a3422 = v0;
            let a3423 = 48u64;
            nat_sub(a3422, a3423)
        })
    } else {
        let a3424 = 97u64;
        let a3425 = v0;
        if if nat_le(a3424, a3425) {
            let a3426 = v0;
            let a3427 = 102u64;
            nat_le(a3426, a3427)
        } else {
            false
        } {
            Some({
                let a3428 = v0;
                let a3429 = 87u64;
                nat_sub(a3428, a3429)
            })
        } else {
            let a3430 = 65u64;
            let a3431 = v0;
            if if nat_le(a3430, a3431) {
                let a3432 = v0;
                let a3433 = 70u64;
                nat_le(a3432, a3433)
            } else {
                false
            } {
                Some({
                    let a3434 = v0;
                    let a3435 = 55u64;
                    nat_sub(a3434, a3435)
                })
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f540(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f565(v0, v1)
}

pub fn f541(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f566(v0, v1)
}

pub fn f542(v0: Bytes, v1: u64, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    let v3: u64 = {
        let (h3436, _) = v2;
        h3436
    };
    let v4: u64 = f174(v0.clone(), v3)?;
    let (_, h3437) = v2;
    if h3437 {
        Ok(None::<(u64, bool)>)
    } else {
        if f244(v4) {
            Ok(Some(({
                let a3438 = v3;
                let a3439 = 1u64;
                nat_add(a3438, a3439)?
            }, false)))
        } else {
            let a3440 = v4;
            let a3441 = 35u64;
            if if nat_eq(a3440, a3441) {
                let a3442 = v3;
                let a3443 = 0u64;
                if if nat_eq(a3442, a3443) {
                    true
                } else {
                    f244(f174(v0.clone(), {
                        let a3444 = v3;
                        let a3445 = 1u64;
                        nat_sub(a3444, a3445)
                    })?)
                } {
                    true
                } else {
                    let a3448 = f174(v0.clone(), {
                        let a3446 = v3;
                        let a3447 = 1u64;
                        nat_sub(a3446, a3447)
                    })?;
                    let a3449 = 10u64;
                    nat_eq(a3448, a3449)
                }
            } else {
                false
            } {
                Ok(Some((f342(v0, v3)?, false)))
            } else {
                let a3450 = v4;
                let a3451 = 10u64;
                if nat_eq(a3450, a3451) {
                    let v5: u64 = {
                        let a3452 = v3;
                        let a3453 = 1u64;
                        nat_add(a3452, a3453)?
                    };
                    let v6: u64 = f249(v0.clone(), v5)?;
                    let v7: u64 = f247(v0.clone(), v6)?;
                    if if f172(v0.clone(), v5)? {
                        true
                    } else {
                        let a3456 = {
                            let a3454 = v6;
                            let a3455 = v5;
                            nat_sub(a3454, a3455)
                        };
                        let a3457 = v1;
                        if if nat_lt(a3456, a3457) {
                            f176(f245(f174(v0.clone(), v7)?))
                        } else {
                            false
                        } {
                            f176({
                                let a3458 = f174(v0, v7)?;
                                let a3459 = 35u64;
                                nat_eq(a3458, a3459)
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

pub fn f543(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f247(v0.clone(), {
        let (h3460, _) = v1;
        h3460
    })?;
    let a3461 = f174(v0, v2)?;
    let a3462 = 10u64;
    if nat_eq(a3461, a3462) {
        Ok(Some(({
            let a3463 = v2;
            let a3464 = 1u64;
            nat_add(a3463, a3464)?
        }, {
            let a3466 = {
                let (_, h3465) = v1;
                h3465
            };
            let a3467 = 1u64;
            nat_add(a3466, a3467)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f544(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f567(v1, v0, v2)
}

pub fn f545(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f568(v0, v1)
}

pub fn f546(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f569(v0, v1)
}

pub fn f547(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a3468 = v0.clone();
        length_bytes(a3468)
    };
    let a3469 = 0u64;
    let a3470 = v1;
    if nat_lt(a3469, a3470) {
        let a3471 = f570(v0.clone())?;
        let a3472 = v1;
        if nat_eq(a3471, a3472) {
            let a3473 = 1u64;
            let a3474 = v1;
            if nat_lt(a3473, a3474) {
                let m3477 = {
                    let a3475 = v0;
                    let a3476 = 0u64;
                    index_bytes(a3475, a3476)
                };
                match m3477 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a3478 = v2;
                        let a3479 = 48u8;
                        Ok(!equal(a3478, a3479))
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

pub fn f548(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f571(v0, v1)
}

pub fn f549(v0: Bytes) -> R<u64> {
    let (h3480, _) = f130(Fn4::F572(v0.clone()), {
        let a3482 = {
            let a3481 = v0;
            length_bytes(a3481)
        };
        let a3483 = 1u64;
        nat_add(a3482, a3483)?
    }, 0u64)?;
    Ok(h3480)
}

pub fn f550(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a3528 = {
            let a3504 = {
                let a3492 = {
                    let a3486 = {
                        let a3484 = v3;
                        let a3485 = 128u8;
                        fixed_u8::bit_and(a3484, a3485)
                    };
                    let a3487 = 128u8;
                    if equal(a3486, a3487) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3493 = {
                    let a3490 = {
                        let a3488 = v3;
                        let a3489 = 64u8;
                        fixed_u8::bit_and(a3488, a3489)
                    };
                    let a3491 = 64u8;
                    if equal(a3490, a3491) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3492, a3493)?
            };
            let a3505 = {
                let a3502 = {
                    let a3496 = {
                        let a3494 = v3;
                        let a3495 = 32u8;
                        fixed_u8::bit_and(a3494, a3495)
                    };
                    let a3497 = 32u8;
                    if equal(a3496, a3497) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3503 = {
                    let a3500 = {
                        let a3498 = v3;
                        let a3499 = 16u8;
                        fixed_u8::bit_and(a3498, a3499)
                    };
                    let a3501 = 16u8;
                    if equal(a3500, a3501) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3502, a3503)?
            };
            nat_add(a3504, a3505)?
        };
        let a3529 = {
            let a3526 = {
                let a3514 = {
                    let a3508 = {
                        let a3506 = v3;
                        let a3507 = 8u8;
                        fixed_u8::bit_and(a3506, a3507)
                    };
                    let a3509 = 8u8;
                    if equal(a3508, a3509) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3515 = {
                    let a3512 = {
                        let a3510 = v3;
                        let a3511 = 4u8;
                        fixed_u8::bit_and(a3510, a3511)
                    };
                    let a3513 = 4u8;
                    if equal(a3512, a3513) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3514, a3515)?
            };
            let a3527 = {
                let a3524 = {
                    let a3518 = {
                        let a3516 = v3;
                        let a3517 = 2u8;
                        fixed_u8::bit_and(a3516, a3517)
                    };
                    let a3519 = 2u8;
                    if equal(a3518, a3519) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3525 = {
                    let a3522 = {
                        let a3520 = v3;
                        let a3521 = 1u8;
                        fixed_u8::bit_and(a3520, a3521)
                    };
                    let a3523 = 1u8;
                    if equal(a3522, a3523) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3524, a3525)?
            };
            nat_add(a3526, a3527)?
        };
        nat_add(a3528, a3529)?
    };
    let a3530 = v4;
    let a3531 = 34u64;
    if nat_eq(a3530, a3531) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a3532 = v4;
        let a3533 = 92u64;
        if nat_eq(a3532, a3533) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a3534 = v4;
            let a3535 = 8u64;
            if nat_eq(a3534, a3535) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a3536 = v4;
                let a3537 = 12u64;
                if nat_eq(a3536, a3537) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a3538 = v4;
                    let a3539 = 10u64;
                    if nat_eq(a3538, a3539) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a3540 = v4;
                        let a3541 = 13u64;
                        if nat_eq(a3540, a3541) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a3542 = v4;
                            let a3543 = 9u64;
                            if nat_eq(a3542, a3543) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a3544 = v4;
                                let a3545 = 32u64;
                                if nat_lt(a3544, a3545) {
                                    let a3546 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a3547 = f573(v2)?;
                                    Ok(append_bytes(a3546, a3547))
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

pub fn f551(v0: Fn11, p3551: List<List<(Bytes, Bytes)>>, p3552: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m3550 = (p3551, p3552);
    {
        let mut m3550 = m3550;
        loop {
            let step = {
                let (v1, v2) = m3550;
                let m3548 = v2;
                match m3548.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c3549: Fn11 = v0.clone();
                            c3549.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3550 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f552(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f553(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a3554 = {
        let a3553 = v0.clone();
        length_list(a3553)
    };
    let a3555 = 1u64;
    if nat_le(a3554, a3555) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f574(v0)?))
    }
}

pub fn f554(v0: u64) -> bool {
    let a3556 = 48u64;
    let a3557 = v0;
    if nat_le(a3556, a3557) {
        let a3558 = v0;
        let a3559 = 55u64;
        nat_le(a3558, a3559)
    } else {
        false
    }
}

pub fn f555(v0: u64) -> bool {
    if f268(v0) {
        true
    } else {
        if f364(v0) {
            true
        } else {
            let a3560 = v0;
            let a3561 = 95u64;
            if nat_eq(a3560, a3561) {
                true
            } else {
                let a3562 = v0;
                let a3563 = 45u64;
                nat_eq(a3562, a3563)
            }
        }
    }
}

pub fn f556(v0: bool, v1: List<Bytes>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m3564 = v1;
    match m3564.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m3565 = f296(v0, v3.clone())?;
            match m3565 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f163(f8(Str::lit("the component name ")), v3, f8(Str::lit(" derives no LexLean name")))))
                }
                Some(v5) => {
                    if f115(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f163(f8(Str::lit("two components derive the name ")), v5, f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n; when those names are not distinct, every name keeps its letters and digits and writes each other character as a C11 universal character name)")))))
                    } else {
                        let m3566 = f556(v0, v4, List::cons(v5.clone(), v2))?;
                        match m3566 {
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

pub fn f557(v0: Adt32) -> Adt32 {
    Adt32::C0(match v0.clone() {
        Adt32::C0(h3567, _, _, _, _) => {
            h3567
        }
    }, match v0 {
        Adt32::C0(_, h3568, _, _, _) => {
            h3568
        }
    }, 0u64, 0u64, 0u64)
}

pub fn f558(v0: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: Adt25) -> R<Result<Adt36, Bytes>> {
    let m3569 = v0;
    if m3569 == 0 {
        Ok(Err::<Adt36, Bytes>(f413(f8(Str::lit("the element declarations nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m3569 - 1;
        let m3570 = f238(v4.clone(), Str::lit("ref"));
        match m3570 {
            None => {
                let m3581 = f238(v4.clone(), Str::lit("name"));
                match m3581 {
                    None => {
                        Ok(Err::<Adt36, Bytes>(f413(f8(Str::lit("a local element declaration must have a name or a ref (XSD 1.1 Part 1, section 3.3.3)")), v2)))
                    }
                    Some(v12) => {
                        let m3584 = f409(v5, v1, {
                            let a3582 = v2;
                            let a3583 = f154(v12.clone())?;
                            append_bytes(a3582, a3583)
                        }, v3, v4)?;
                        match m3584 {
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
                let m3572 = f91(match v4 {
                    Adt25::C0(_, _, _, _, h3571) => {
                        h3571
                    }
                }, v6.clone())?;
                match m3572 {
                    Ok(v8) => {
                        let m3573 = f484(v1.clone(), Str::lit("element"), v8.clone());
                        match m3573 {
                            None => {
                                Ok(Err::<Adt36, Bytes>(f413(f163(f8(Str::lit("the element ")), v6, f8(Str::lit(" is not declared by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                            }
                            Some(v9) => {
                                if if f471(match v9.clone() {
                                    Adt24::C0(_, _, _, h3574) => {
                                        h3574
                                    }
                                }, Str::lit("abstract"), Str::lit("true")) {
                                    true
                                } else {
                                    f575(match v1.clone() {
                                        Adt23::C0(h3575, _) => {
                                            h3575
                                        }
                                    }, v8.clone())?
                                } {
                                    Ok(Err::<Adt36, Bytes>(f413(f163(f8(Str::lit("the element ")), v6, f8(Str::lit(" heads a substitution group, so its particle also admits the members of that group, which is not translated (XSD 1.1 Part 1, section 3.3.6.4)"))), v2)))
                                } else {
                                    let m3579 = f409(v5, v1, f154({
                                        let (_, h3576) = v8.clone();
                                        h3576
                                    })?, match v9.clone() {
                                        Adt24::C0(_, h3577, _, _) => {
                                            h3577
                                        }
                                    }, match v9 {
                                        Adt24::C0(_, _, _, h3578) => {
                                            h3578
                                        }
                                    })?;
                                    match m3579 {
                                        Ok(v11) => {
                                            Ok(Ok::<Adt36, Bytes>(Adt36::C0({
                                                let (_, h3580) = v8;
                                                h3580
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
                        Ok(Err::<Adt36, Bytes>(f413(v7, v2)))
                    }
                }
            }
        }
    }
}

pub fn f559(v0: Adt25, v1: Bytes) -> Option<Bytes> {
    let v2: Bytes = f241(v0.clone(), Str::lit("maxOccurs"), Str::lit("1"));
    if f35(v2.clone(), f8(Str::lit("0"))) {
        None::<Bytes>
    } else {
        if f35(v2, f8(Str::lit("1"))) {
            if f471(v0, Str::lit("minOccurs"), Str::lit("0")) {
                Some(f530(Str::lit("Option"), v1))
            } else {
                Some(v1)
            }
        } else {
            Some(f530(Str::lit("List"), v1))
        }
    }
}

pub fn f560(v0: Adt25) -> bool {
    if f35(f241(v0.clone(), Str::lit("minOccurs"), Str::lit("1")), f8(Str::lit("1"))) {
        f35(f241(v0, Str::lit("maxOccurs"), Str::lit("1")), f8(Str::lit("1")))
    } else {
        false
    }
}

pub fn f561(v0: Adt32, v1: Bytes) -> R<u64> {
    if f35(v1.clone(), f8(Str::lit("sequence"))) {
        let a3586 = match v0 {
            Adt32::C0(_, _, h3585, _, _) => {
                h3585
            }
        };
        let a3587 = 1u64;
        nat_add(a3586, a3587)
    } else {
        if f35(v1, f8(Str::lit("choice"))) {
            let a3589 = match v0 {
                Adt32::C0(_, _, _, h3588, _) => {
                    h3588
                }
            };
            let a3590 = 1u64;
            nat_add(a3589, a3590)
        } else {
            let a3592 = match v0 {
                Adt32::C0(_, _, _, _, h3591) => {
                    h3591
                }
            };
            let a3593 = 1u64;
            nat_add(a3592, a3593)
        }
    }
}

pub fn f562(v0: Adt32, v1: Bytes) -> R<Adt32> {
    if f35(v1.clone(), f8(Str::lit("sequence"))) {
        Ok(Adt32::C0(match v0.clone() {
            Adt32::C0(h3594, _, _, _, _) => {
                h3594
            }
        }, match v0.clone() {
            Adt32::C0(_, h3595, _, _, _) => {
                h3595
            }
        }, {
            let a3597 = match v0.clone() {
                Adt32::C0(_, _, h3596, _, _) => {
                    h3596
                }
            };
            let a3598 = 1u64;
            nat_add(a3597, a3598)?
        }, match v0.clone() {
            Adt32::C0(_, _, _, h3599, _) => {
                h3599
            }
        }, match v0 {
            Adt32::C0(_, _, _, _, h3600) => {
                h3600
            }
        }))
    } else {
        if f35(v1, f8(Str::lit("choice"))) {
            Ok(Adt32::C0(match v0.clone() {
                Adt32::C0(h3601, _, _, _, _) => {
                    h3601
                }
            }, match v0.clone() {
                Adt32::C0(_, h3602, _, _, _) => {
                    h3602
                }
            }, match v0.clone() {
                Adt32::C0(_, _, h3603, _, _) => {
                    h3603
                }
            }, {
                let a3605 = match v0.clone() {
                    Adt32::C0(_, _, _, h3604, _) => {
                        h3604
                    }
                };
                let a3606 = 1u64;
                nat_add(a3605, a3606)?
            }, match v0 {
                Adt32::C0(_, _, _, _, h3607) => {
                    h3607
                }
            }))
        } else {
            Ok(Adt32::C0(match v0.clone() {
                Adt32::C0(h3608, _, _, _, _) => {
                    h3608
                }
            }, match v0.clone() {
                Adt32::C0(_, h3609, _, _, _) => {
                    h3609
                }
            }, match v0.clone() {
                Adt32::C0(_, _, h3610, _, _) => {
                    h3610
                }
            }, match v0.clone() {
                Adt32::C0(_, _, _, h3611, _) => {
                    h3611
                }
            }, {
                let a3613 = match v0 {
                    Adt32::C0(_, _, _, _, h3612) => {
                        h3612
                    }
                };
                let a3614 = 1u64;
                nat_add(a3613, a3614)?
            }))
        }
    }
}

pub fn f563(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f492(v0.clone(), {
        let (h3615, _) = v2;
        h3615
    }, v1)?;
    let a3616 = f174(v0, v3)?;
    let a3617 = 10u64;
    if nat_eq(a3616, a3617) {
        Ok(Some(({
            let a3618 = v3;
            let a3619 = 1u64;
            nat_add(a3618, a3619)?
        }, {
            let a3621 = {
                let (_, h3620) = v2;
                h3620
            };
            let a3622 = 1u64;
            nat_add(a3621, a3622)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f564(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a3623 = f174(v0, v3)?;
    let a3624 = 32u64;
    if if nat_eq(a3623, a3624) {
        let a3627 = {
            let a3625 = v3;
            let a3626 = v1;
            nat_sub(a3625, a3626)
        };
        let a3628 = v2;
        nat_lt(a3627, a3628)
    } else {
        false
    } {
        Ok(Some({
            let a3629 = v3;
            let a3630 = 1u64;
            nat_add(a3629, a3630)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f565(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f174(v0, v1)?;
    let a3631 = v2;
    let a3632 = 34u64;
    if if if if nat_eq(a3631, a3632) {
        true
    } else {
        let a3633 = v2;
        let a3634 = 92u64;
        nat_eq(a3633, a3634)
    } {
        true
    } else {
        f244(v2)
    } {
        true
    } else {
        f245(v2)
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3635 = v1;
            let a3636 = 1u64;
            nat_add(a3635, a3636)?
        }))
    }
}

pub fn f566(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f174(v0, v1)?;
    let a3637 = v2;
    let a3638 = 39u64;
    if if if nat_eq(a3637, a3638) {
        true
    } else {
        f244(v2)
    } {
        true
    } else {
        f245(v2)
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3639 = v1;
            let a3640 = 1u64;
            nat_add(a3639, a3640)?
        }))
    }
}

pub fn f567(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m3642 = f539(f174(v0, {
        let (h3641, _) = v2;
        h3641
    })?);
    match m3642 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v3) => {
            let a3643 = v3;
            let a3644 = v1;
            if nat_lt(a3643, a3644) {
                Ok(Some(({
                    let a3646 = {
                        let (h3645, _) = v2;
                        h3645
                    };
                    let a3647 = 1u64;
                    nat_add(a3646, a3647)?
                }, {
                    let a3651 = {
                        let a3649 = {
                            let (_, h3648) = v2;
                            h3648
                        };
                        let a3650 = v1;
                        nat_mul(a3649, a3650)?
                    };
                    let a3652 = v3;
                    nat_add(a3651, a3652)?
                })))
            } else {
                Ok(None::<(u64, u64)>)
            }
        }
    }
}

pub fn f568(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f139(v0, v1)?;
    let a3653 = 47u64;
    let a3654 = v2;
    if if nat_lt(a3653, a3654) {
        let a3655 = v2;
        let a3656 = 58u64;
        nat_lt(a3655, a3656)
    } else {
        false
    } {
        Ok(Some({
            let a3657 = v1;
            let a3658 = 1u64;
            nat_add(a3657, a3658)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f569(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a3659 = f139(v0, v1)?;
    let a3660 = 48u64;
    if nat_eq(a3659, a3660) {
        Ok(Some({
            let a3661 = v1;
            let a3662 = 1u64;
            nat_add(a3661, a3662)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f570(v0: Bytes) -> R<u64> {
    let (h3663, _) = f130(Fn4::F576(v0.clone()), {
        let a3664 = v0;
        length_bytes(a3664)
    }, 0u64)?;
    Ok(h3663)
}

pub fn f571(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m3668 = {
        let a3666 = v0;
        let a3667 = {
            let (h3665, _) = v1;
            h3665
        };
        index_bytes(a3666, a3667)
    };
    match m3668 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a3670 = {
                    let (h3669, _) = v1;
                    h3669
                };
                let a3671 = 1u64;
                nat_add(a3670, a3671)?
            }, {
                let a3675 = {
                    let a3673 = {
                        let (_, h3672) = v1;
                        h3672
                    };
                    let a3674 = 10u64;
                    nat_mul(a3673, a3674)?
                };
                let a3676 = f577(v2)?;
                nat_add(a3675, a3676)?
            })))
        }
    }
}

pub fn f572(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f578(v0, v1)
}

pub fn f573(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3721 = {
            let a3697 = {
                let a3685 = {
                    let a3679 = {
                        let a3677 = v1;
                        let a3678 = 128u8;
                        fixed_u8::bit_and(a3677, a3678)
                    };
                    let a3680 = 128u8;
                    if equal(a3679, a3680) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3686 = {
                    let a3683 = {
                        let a3681 = v1;
                        let a3682 = 64u8;
                        fixed_u8::bit_and(a3681, a3682)
                    };
                    let a3684 = 64u8;
                    if equal(a3683, a3684) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3685, a3686)?
            };
            let a3698 = {
                let a3695 = {
                    let a3689 = {
                        let a3687 = v1;
                        let a3688 = 32u8;
                        fixed_u8::bit_and(a3687, a3688)
                    };
                    let a3690 = 32u8;
                    if equal(a3689, a3690) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3696 = {
                    let a3693 = {
                        let a3691 = v1;
                        let a3692 = 16u8;
                        fixed_u8::bit_and(a3691, a3692)
                    };
                    let a3694 = 16u8;
                    if equal(a3693, a3694) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3695, a3696)?
            };
            nat_add(a3697, a3698)?
        };
        let a3722 = {
            let a3719 = {
                let a3707 = {
                    let a3701 = {
                        let a3699 = v1;
                        let a3700 = 8u8;
                        fixed_u8::bit_and(a3699, a3700)
                    };
                    let a3702 = 8u8;
                    if equal(a3701, a3702) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3708 = {
                    let a3705 = {
                        let a3703 = v1;
                        let a3704 = 4u8;
                        fixed_u8::bit_and(a3703, a3704)
                    };
                    let a3706 = 4u8;
                    if equal(a3705, a3706) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3707, a3708)?
            };
            let a3720 = {
                let a3717 = {
                    let a3711 = {
                        let a3709 = v1;
                        let a3710 = 2u8;
                        fixed_u8::bit_and(a3709, a3710)
                    };
                    let a3712 = 2u8;
                    if equal(a3711, a3712) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3718 = {
                    let a3715 = {
                        let a3713 = v1;
                        let a3714 = 1u8;
                        fixed_u8::bit_and(a3713, a3714)
                    };
                    let a3716 = 1u8;
                    if equal(a3715, a3716) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3717, a3718)?
            };
            nat_add(a3719, a3720)?
        };
        nat_add(a3721, a3722)?
    };
    let a3729 = f126(f579(), {
        let a3723 = v2;
        let a3724 = 16u64;
        let a3725 = 0u64;
        nat_quot(a3723, a3724, a3725)
    });
    let a3730 = f126(f579(), {
        let a3726 = v2;
        let a3727 = 16u64;
        let a3728 = 0u64;
        nat_rem(a3726, a3727, a3728)
    });
    Ok(append_bytes(a3729, a3730))
}

pub fn f574(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f514({
        let (_, h3731) = {
            let (h3732, _) = f580(Fn12::F581, {
                let a3734 = {
                    let a3733 = v0.clone();
                    length_list(a3733)
                };
                let a3735 = 1u64;
                nat_add(a3734, a3735)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h3732
        };
        h3731
    }))
}

pub fn f575(p3744: List<Adt24>, v1: (Bytes, Bytes)) -> R<bool> {
    let m3743 = p3744;
    {
        let mut m3743 = m3743;
        loop {
            let step = {
                let v0 = m3743;
                let m3736 = v0;
                match m3736.uncons() {
                    None => {
                        Err::<List<Adt24>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if if f35(match v2.clone() {
                            Adt24::C0(h3737, _, _, _) => {
                                h3737
                            }
                        }, f8(Str::lit("element"))) {
                            let m3739 = f238(match v2.clone() {
                                Adt24::C0(_, _, _, h3738) => {
                                    h3738
                                }
                            }, Str::lit("substitutionGroup"));
                            match m3739 {
                                None => {
                                    false
                                }
                                Some(v4) => {
                                    f582({
                                        let m3742 = match v2 {
                                            Adt24::C0(_, _, _, h3741) => {
                                                h3741
                                            }
                                        };
                                        match m3742 {
                                            Adt25::C0(_, _, _, _, h3740) => {
                                                h3740
                                            }
                                        }
                                    }, f583(v4)?, v1.clone())?
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
                Ok(next) => { m3743 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f576(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3747 = {
        let a3745 = v0;
        let a3746 = v1;
        index_bytes(a3745, a3746)
    };
    match m3747 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f584(v2)? {
                Ok(Some({
                    let a3748 = v1;
                    let a3749 = 1u64;
                    nat_add(a3748, a3749)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f577(v0: u8) -> R<u64> {
    let a3796 = {
        let v1: u8 = v0;
        let a3794 = {
            let a3770 = {
                let a3758 = {
                    let a3752 = {
                        let a3750 = v1;
                        let a3751 = 128u8;
                        fixed_u8::bit_and(a3750, a3751)
                    };
                    let a3753 = 128u8;
                    if equal(a3752, a3753) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3759 = {
                    let a3756 = {
                        let a3754 = v1;
                        let a3755 = 64u8;
                        fixed_u8::bit_and(a3754, a3755)
                    };
                    let a3757 = 64u8;
                    if equal(a3756, a3757) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3758, a3759)?
            };
            let a3771 = {
                let a3768 = {
                    let a3762 = {
                        let a3760 = v1;
                        let a3761 = 32u8;
                        fixed_u8::bit_and(a3760, a3761)
                    };
                    let a3763 = 32u8;
                    if equal(a3762, a3763) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3769 = {
                    let a3766 = {
                        let a3764 = v1;
                        let a3765 = 16u8;
                        fixed_u8::bit_and(a3764, a3765)
                    };
                    let a3767 = 16u8;
                    if equal(a3766, a3767) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3768, a3769)?
            };
            nat_add(a3770, a3771)?
        };
        let a3795 = {
            let a3792 = {
                let a3780 = {
                    let a3774 = {
                        let a3772 = v1;
                        let a3773 = 8u8;
                        fixed_u8::bit_and(a3772, a3773)
                    };
                    let a3775 = 8u8;
                    if equal(a3774, a3775) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3781 = {
                    let a3778 = {
                        let a3776 = v1;
                        let a3777 = 4u8;
                        fixed_u8::bit_and(a3776, a3777)
                    };
                    let a3779 = 4u8;
                    if equal(a3778, a3779) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3780, a3781)?
            };
            let a3793 = {
                let a3790 = {
                    let a3784 = {
                        let a3782 = v1;
                        let a3783 = 2u8;
                        fixed_u8::bit_and(a3782, a3783)
                    };
                    let a3785 = 2u8;
                    if equal(a3784, a3785) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3791 = {
                    let a3788 = {
                        let a3786 = v1;
                        let a3787 = 1u8;
                        fixed_u8::bit_and(a3786, a3787)
                    };
                    let a3789 = 1u8;
                    if equal(a3788, a3789) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3790, a3791)?
            };
            nat_add(a3792, a3793)?
        };
        nat_add(a3794, a3795)?
    };
    let a3797 = 48u64;
    Ok(nat_sub(a3796, a3797))
}

pub fn f578(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a3798 = v0.clone();
        length_bytes(a3798)
    };
    let a3799 = v1;
    let a3800 = v2;
    if if nat_lt(a3799, a3800) {
        let a3805 = f139(v0, {
            let a3803 = v2;
            let a3804 = {
                let a3801 = v1;
                let a3802 = 1u64;
                nat_add(a3801, a3802)?
            };
            nat_sub(a3803, a3804)
        })?;
        let a3806 = 48u64;
        nat_eq(a3805, a3806)
    } else {
        false
    } {
        Ok(Some({
            let a3807 = v1;
            let a3808 = 1u64;
            nat_add(a3807, a3808)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f579() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f580(v0: Fn12, p3813: u64, p3814: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m3812 = (p3813, p3814);
    {
        let mut m3812 = m3812;
        loop {
            let step = {
                let (v1, v2) = m3812;
                let m3809 = v1;
                if m3809 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3809 - 1;
                    let m3811 = {
                        let c3810: Fn12 = v0.clone();
                        c3810.apply(v2.clone())?
                    };
                    match m3811 {
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
                Ok(next) => { m3812 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f581(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f585(v0)
}

pub fn f582(v0: List<(Bytes, Bytes)>, p3822: List<Bytes>, v2: (Bytes, Bytes)) -> R<bool> {
    let m3821 = p3822;
    {
        let mut m3821 = m3821;
        loop {
            let step = {
                let v1 = m3821;
                let m3815 = v1;
                match m3815.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v3, v4)) => {
                        let m3816 = f91(v0.clone(), v3)?;
                        match m3816 {
                            Ok(v6) => {
                                if if f35({
                                    let (h3817, _) = v6;
                                    h3817
                                }, {
                                    let (h3818, _) = v2.clone();
                                    h3818
                                }) {
                                    f35({
                                        let (_, h3819) = v6;
                                        h3819
                                    }, {
                                        let (_, h3820) = v2.clone();
                                        h3820
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
                Ok(next) => { m3821 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f583(v0: Bytes) -> R<List<Bytes>> {
    f30(v0, 32u8)
}

pub fn f584(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3867 = {
            let a3843 = {
                let a3831 = {
                    let a3825 = {
                        let a3823 = v1;
                        let a3824 = 128u8;
                        fixed_u8::bit_and(a3823, a3824)
                    };
                    let a3826 = 128u8;
                    if equal(a3825, a3826) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3832 = {
                    let a3829 = {
                        let a3827 = v1;
                        let a3828 = 64u8;
                        fixed_u8::bit_and(a3827, a3828)
                    };
                    let a3830 = 64u8;
                    if equal(a3829, a3830) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3831, a3832)?
            };
            let a3844 = {
                let a3841 = {
                    let a3835 = {
                        let a3833 = v1;
                        let a3834 = 32u8;
                        fixed_u8::bit_and(a3833, a3834)
                    };
                    let a3836 = 32u8;
                    if equal(a3835, a3836) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3842 = {
                    let a3839 = {
                        let a3837 = v1;
                        let a3838 = 16u8;
                        fixed_u8::bit_and(a3837, a3838)
                    };
                    let a3840 = 16u8;
                    if equal(a3839, a3840) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3841, a3842)?
            };
            nat_add(a3843, a3844)?
        };
        let a3868 = {
            let a3865 = {
                let a3853 = {
                    let a3847 = {
                        let a3845 = v1;
                        let a3846 = 8u8;
                        fixed_u8::bit_and(a3845, a3846)
                    };
                    let a3848 = 8u8;
                    if equal(a3847, a3848) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3854 = {
                    let a3851 = {
                        let a3849 = v1;
                        let a3850 = 4u8;
                        fixed_u8::bit_and(a3849, a3850)
                    };
                    let a3852 = 4u8;
                    if equal(a3851, a3852) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3853, a3854)?
            };
            let a3866 = {
                let a3863 = {
                    let a3857 = {
                        let a3855 = v1;
                        let a3856 = 2u8;
                        fixed_u8::bit_and(a3855, a3856)
                    };
                    let a3858 = 2u8;
                    if equal(a3857, a3858) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3864 = {
                    let a3861 = {
                        let a3859 = v1;
                        let a3860 = 1u8;
                        fixed_u8::bit_and(a3859, a3860)
                    };
                    let a3862 = 1u8;
                    if equal(a3861, a3862) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3863, a3864)?
            };
            nat_add(a3865, a3866)?
        };
        nat_add(a3867, a3868)?
    };
    let a3869 = 48u64;
    let a3870 = v2;
    if nat_le(a3869, a3870) {
        let a3871 = v2;
        let a3872 = 57u64;
        Ok(nat_le(a3871, a3872))
    } else {
        Ok(false)
    }
}

pub fn f585(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h3873, _) = v0;
        h3873
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h3874) = v0;
        h3874
    };
    let m3875 = v1;
    match m3875.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m3876 = v4;
            match m3876.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f586(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f586(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3881 = {
            let a3879 = {
                let a3877 = v0.clone();
                length_list(a3877)
            };
            let a3880 = {
                let a3878 = v1.clone();
                length_list(a3878)
            };
            nat_add(a3879, a3880)?
        };
        let a3882 = 1u64;
        nat_add(a3881, a3882)?
    };
    Ok(f587({
        let (_, h3883) = {
            let (_, h3884) = {
                let (h3885, _) = f588(Fn13::F589, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3885
            };
            h3884
        };
        h3883
    }))
}

pub fn f587(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f357(Fn5::F590, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f588(v0: Fn13, p3890: u64, p3891: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3889 = (p3890, p3891);
    {
        let mut m3889 = m3889;
        loop {
            let step = {
                let (v1, v2) = m3889;
                let m3886 = v1;
                if m3886 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3886 - 1;
                    let m3888 = {
                        let c3887: Fn13 = v0.clone();
                        c3887.apply(v2.clone())
                    };
                    match m3888 {
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
                Ok(next) => { m3889 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f589(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f591(v0)
}

pub fn f590(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f591(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h3892, _) = v0.clone();
        h3892
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h3893, _) = {
            let (_, h3894) = v0.clone();
            h3894
        };
        h3893
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h3895) = {
            let (_, h3896) = v0;
            h3896
        };
        h3895
    };
    let m3897 = v1.clone();
    match m3897.uncons() {
        None => {
            let m3898 = v2;
            match m3898.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3899 = v2.clone();
            match m3899.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f592({
                        let (h3900, _) = v8.clone();
                        h3900
                    }, {
                        let (h3901, _) = v6.clone();
                        h3901
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

pub fn f592(v0: Bytes, v1: Bytes) -> bool {
    let a3906 = {
        let a3902 = v0;
        let a3903 = v1;
        compare_bytes(a3902, a3903)
    };
    let a3907 = {
        let a3904 = Bytes::lit(&[]);
        let a3905 = Bytes::lit(&[0x01]);
        compare_bytes(a3904, a3905)
    };
    equal(a3906, a3907)
}

pub fn run(p0: Option<Adt0>, p1: Adt2) -> R<(Adt0, Adt9)> {
    f0(p0, p1)
}
