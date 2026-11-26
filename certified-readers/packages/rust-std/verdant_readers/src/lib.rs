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
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt21 {
    C0(u64),
    C1(Bytes),
    C2(Bytes),
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
    C0(Bytes, Bytes, Adt17, Adt17, List<(Bytes, Bytes)>),
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
    C0(Bytes, Adt17, Adt17, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Adt32 {
    C0(Bytes, u64),
    C1(Bytes),
}

#[derive(Clone)]
pub enum Adt33 {
    C0(Bytes, List<Adt23>),
}

#[derive(Clone)]
pub enum Adt34 {
    C0(Adt15, List<Adt23>),
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
    F67(u8, Bytes),
    F296(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F67(k0, k1) => f67(*k0, k1.clone(), p0),
            Fn0::F296(k0) => f296(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F70(Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn1::F70(k0) => f70(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F117,
    F298,
    F414,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn2::F117 => f117(p0, p1),
            Fn2::F298 => f298(p0, p1),
            Fn2::F414 => f414(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F191(Bytes, u64, u64),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Adt20)) -> R<(u64, Adt20)> {
        match self {
            Fn3::F191(k0, k1, k2) => f191(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F203(Bytes),
    F213(u64, Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn4::F203(k0) => f203(k0.clone(), p0),
            Fn4::F213(k0, k1) => Ok(f213(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F205(Bytes),
    F206(Bytes),
    F232(Bytes),
    F271(Bytes),
    F272(Bytes),
    F300(Bytes, u64, Bytes, u64),
    F353(Bytes, Bytes),
    F354(Bytes),
    F355(Bytes, u64),
    F392(Bytes),
    F423(Bytes),
    F425(Bytes),
    F437(Bytes, u64),
    F482(bool, u64, Bytes),
    F505(Bytes),
    F537(Bytes),
    F539(Bytes),
    F565(Bytes, bool),
    F568(u64, Bytes),
    F722(u64, Bytes, u64),
    F726(Bytes),
    F727(Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn5::F205(k0) => f205(k0.clone(), p0),
            Fn5::F206(k0) => f206(k0.clone(), p0),
            Fn5::F232(k0) => f232(k0.clone(), p0),
            Fn5::F271(k0) => f271(k0.clone(), p0),
            Fn5::F272(k0) => f272(k0.clone(), p0),
            Fn5::F300(k0, k1, k2, k3) => f300(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn5::F353(k0, k1) => f353(k0.clone(), k1.clone(), p0),
            Fn5::F354(k0) => f354(k0.clone(), p0),
            Fn5::F355(k0, k1) => f355(k0.clone(), *k1, p0),
            Fn5::F392(k0) => f392(k0.clone(), p0),
            Fn5::F423(k0) => f423(k0.clone(), p0),
            Fn5::F425(k0) => f425(k0.clone(), p0),
            Fn5::F437(k0, k1) => f437(k0.clone(), *k1, p0),
            Fn5::F482(k0, k1, k2) => f482(*k0, *k1, k2.clone(), p0),
            Fn5::F505(k0) => f505(k0.clone(), p0),
            Fn5::F537(k0) => f537(k0.clone(), p0),
            Fn5::F539(k0) => f539(k0.clone(), p0),
            Fn5::F565(k0, k1) => f565(k0.clone(), *k1, p0),
            Fn5::F568(k0, k1) => f568(*k0, k1.clone(), p0),
            Fn5::F722(k0, k1, k2) => f722(*k0, k1.clone(), *k2, p0),
            Fn5::F726(k0) => f726(k0.clone(), p0),
            Fn5::F727(k0) => f727(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F275(Bytes),
    F545(Bytes),
    F561(bool, Bytes),
    F599(bool, Bytes),
    F658(Bytes),
    F672(Bytes),
    F721(u64, Bytes),
    F724(u64, Bytes),
}

impl Fn6 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn6::F275(k0) => f275(k0.clone(), p0),
            Fn6::F545(k0) => f545(k0.clone(), p0),
            Fn6::F561(k0, k1) => f561(*k0, k1.clone(), p0),
            Fn6::F599(k0, k1) => f599(*k0, k1.clone(), p0),
            Fn6::F658(k0) => f658(k0.clone(), p0),
            Fn6::F672(k0) => f672(k0.clone(), p0),
            Fn6::F721(k0, k1) => f721(*k0, k1.clone(), p0),
            Fn6::F724(k0, k1) => f724(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F394,
}

impl Fn7 {
    pub fn apply(&self, p0: Adt30) -> R<Adt30> {
        match self {
            Fn7::F394 => f394(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F408,
    F783,
}

impl Fn8 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn8::F408 => f408(p0, p1),
            Fn8::F783 => f783(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F411,
}

impl Fn9 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn9::F411 => f411(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F526,
}

impl Fn10 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn10::F526 => f526(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F529,
}

impl Fn11 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn11::F529 => f529(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F531,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn12::F531 => f531(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F534,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn13::F534 => f534(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F625(Bytes),
}

impl Fn14 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn14::F625(k0) => f625(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F634,
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn15::F634 => f634(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F643,
}

impl Fn16 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn16::F643 => f643(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F645,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn17::F645 => f645(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F647,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn18::F647 => f647(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F650,
}

impl Fn19 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn19::F650 => f650(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F669(u64, Bytes),
}

impl Fn20 {
    pub fn apply(&self, p0: (u64, bool)) -> R<Option<(u64, bool)>> {
        match self {
            Fn20::F669(k0, k1) => f669(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F718,
}

impl Fn21 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn21::F718 => f718(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F753,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn22::F753 => f753(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F769,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn23::F769 => f769(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F778,
}

impl Fn24 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn24::F778 => f778(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F781,
}

impl Fn25 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn25::F781 => f781(p0),
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
            if f32(v0.clone(), f8(Str::lit("read-xml"))) {
                f58(v3, v1)
            } else {
                if f32(v0.clone(), f8(Str::lit("read-xsd"))) {
                    f59(v3, v1)
                } else {
                    if f32(v0.clone(), f8(Str::lit("read-xsd-components"))) {
                        f60(v3, v1)
                    } else {
                        if f32(v0, f8(Str::lit("read-yaml"))) {
                            f61(v3, v1)
                        } else {
                            Ok(Adt12::C4(f31()))
                        }
                    }
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
    let v1: u64 = f62(v0)?;
    let m199 = {
        let a197 = f63({
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
    let v1: Bytes = f64(v0)?;
    let m229 = f65(v1.clone(), 0u64, {
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
        let (h236, _) = f66(Fn0::F67(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h236
    };
    let v4: u64 = {
        let (h237, _) = {
            let (_, h238) = v3.clone();
            h238
        };
        h237
    };
    Ok(f68(List::cons(f46(v0, v4, {
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
    let m251 = f69(Fn1::F70(v1), None::<Bytes>, v0);
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
    List::cons(f8(Str::lit("admit JSONL")), List::cons(f8(Str::lit("admit-encoding STAGED")), List::cons(f8(Str::lit("admit-registry AUTHORITY ROW CONSTRUCT|none read-OP ARTIFACT")), List::cons(f8(Str::lit("admit-translated FILE NAME read-OP ARG... CITATION...")), List::cons(f8(Str::lit("agenda")), List::cons(f8(Str::lit("build")), List::cons(f8(Str::lit("check")), List::cons(f8(Str::lit("check-yaml FILE")), List::cons(f8(Str::lit("ingest-authority BINDING")), List::cons(f8(Str::lit("ingest-index CSV")), List::cons(f8(Str::lit("merge-gate RULES")), List::cons(f8(Str::lit("next [SKIP...]")), List::cons(f8(Str::lit("propose-encoding SCHEMA NAME CITATION...")), List::cons(f8(Str::lit("publish")), List::cons(f8(Str::lit("read-admit NAME read-OP FILE ARG... CITATION...")), List::cons(f8(Str::lit("read-webidl FILE [IMPORT...]")), List::cons(f8(Str::lit("read-xml FILE")), List::cons(f8(Str::lit("read-xsd FILE")), List::cons(f8(Str::lit("read-xsd-components FILE")), List::cons(f8(Str::lit("read-yaml FILE")), List::cons(f8(Str::lit("record-decision JSONL")), List::cons(f8(Str::lit("record-error REFUSAL TIME RUN OPERATION...")), List::cons(f8(Str::lit("stage-admissions JSONL")), List::cons(f8(Str::lit("stage-bindings JSONL")), List::cons(f8(Str::lit("stage-decisions JSONL")), List::cons(f8(Str::lit("stage-encodings JSONL")), List::cons(f8(Str::lit("stage-engine")), List::cons(f8(Str::lit("stage-mappings JSONL")), List::cons(f8(Str::lit("stage-matrix JSONL")), List::cons(f8(Str::lit("stage-registries JSONL")), List::cons(f8(Str::lit("verify-translations ADMISSIONS")), List::<Bytes>::nil())))))))))))))))))))))))))))))))
}

pub fn f49(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m252 = v1;
    match m252.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m253 = f71(f72(v0.clone(), v2.clone()), v2.clone());
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
                        let m255 = f73(v2.clone(), f8(Str::lit("translation")));
                        match m255 {
                            None => {
                                Ok::<List<Adt2>, R<Adt12>>(v3)
                            }
                            Some(v4) => {
                                let v6: Bytes = {
                                    let m256 = f74(f73(v2, f8(Str::lit("construct"))));
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
                                    let m257 = f74(f73(v4.clone(), f8(Str::lit("output_sha256"))));
                                    match m257 {
                                        None => {
                                            Bytes::lit(&[])
                                        }
                                        Some(v7) => {
                                            v7
                                        }
                                    }
                                };
                                let m258 = f75(f76(f73(v4, f8(Str::lit("command")))));
                                match m258.uncons() {
                                    None => {
                                        Err::<List<Adt2>, R<Adt12>>(Ok(Adt12::C4(f7(List::cons(v6, List::cons(f8(Str::lit(" records no reader command\u{a}")), List::<Bytes>::nil()))))))
                                    }
                                    Some((v9, v10)) => {
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
                                                        if f32(f77(v20.clone())?, v8.clone()) {
                                                            Ok::<List<Adt2>, R<Adt12>>(v3)
                                                        } else {
                                                            Err::<List<Adt2>, R<Adt12>>(Ok(Adt12::C4(f7(List::cons(v6, List::cons(f8(Str::lit(": its reader command now prints output with sha256 ")), List::cons(f77(v20)?, List::cons(f8(Str::lit(", and the admission records ")), List::cons(v8, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))))
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
            };
            match step {
                Ok(next) => { m261 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f51(v0: Adt1, v1: Bytes) -> R<List<Adt2>> {
    Ok(f76(f78(v0, v1)?))
}

pub fn f52(v0: List<Bytes>) -> List<Bytes> {
    let m263 = v0;
    match m263.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f79(v1.clone()) {
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
    let m271 = f80(v4);
    match m271 {
        None => {
            Ok(Adt12::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
        }
        Some(v5) => {
            let m272 = f81(f82(List::cons(Adt2::C8(std::rc::Rc::new(v5.clone())), List::<Adt2>::nil())), f83(f51(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m272 {
                None => {
                    f85(v0, v2.clone(), Adt2::C11(f8(Str::lit("translation")), std::rc::Rc::new(Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("command")), std::rc::Rc::new(Adt2::C7(std::rc::Rc::new(f86(v3)))), std::rc::Rc::new(Adt2::C11(f8(Str::lit("output_sha256")), std::rc::Rc::new(Adt2::C6(f77(v1.clone())?)), std::rc::Rc::new(Adt2::C9))))))), std::rc::Rc::new(v5)), f87(v2, f88(v1)?)?, f8(Str::lit("Encoding entered through admit-translated from the verdant_readers command recorded in its admission.")), List::<(Bytes, Bytes)>::nil())
                }
                Some(v6) => {
                    Ok(f84(v6))
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
                        if f79(v1.clone()) {
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
            let m277 = f89(f90(v0, v1))?;
            match m277 {
                Ok(v5) => {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f91(f92(v5)?)?, List::cons(f8(Str::lit(" documents\u{a}")), List::<Bytes>::nil())))))
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
            let m279 = f93(f90(v0.clone(), v1), f94(v0, v2))?;
            match m279 {
                Ok(v6) => {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("-- ")), List::cons(f91({
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
            let m283 = f95(f90(v0, v1))?;
            match m283 {
                Ok(v5) => {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("well-formed XML: root ")), List::cons(f96(v5.clone()), List::cons(f8(Str::lit(", ")), List::cons(f91(f97(v5)?)?, List::cons(f8(Str::lit(" elements\u{a}")), List::<Bytes>::nil()))))))))
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

pub fn f59(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m284 = f49(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m284.uncons() {
        None => {
            let m285 = f98(f90(v0, v1))?;
            match m285 {
                Ok(v5) => {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("-- ")), List::cons(f91({
                        let (h286, _) = v5;
                        h286
                    })?, List::cons(f8(Str::lit(" simple types\u{a}")), List::cons({
                        let (_, h287) = v5;
                        h287
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

pub fn f60(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m288 = f49(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::<Bytes>::nil())));
    match m288.uncons() {
        None => {
            let m289 = f99(f100(v0.clone()), f90(v0, f8(Str::lit("model/authorities.toml"))), v1)?;
            match m289 {
                Adt18::C0(v4) => {
                    Ok(Adt12::C0(v4))
                }
                Adt18::C1(v5) => {
                    Ok(Adt12::C4(f7(List::cons(f8(Str::lit("XSD not translated: ")), List::cons(v5, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
                Adt18::C2(v6, v7) => {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("-- ")), List::cons(f91(v6)?, List::cons(f8(Str::lit(" declarations\u{a}")), List::cons(v7, List::<Bytes>::nil())))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f61(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m290 = f49(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m290.uncons() {
        None => {
            let m291 = f89(f90(v0, v1))?;
            match m291 {
                Ok(v5) => {
                    let m292 = f101(v5)?;
                    match m292 {
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

pub fn f62(v0: u8) -> R<u64> {
    let v1: u8 = v0;
    let a337 = {
        let a313 = {
            let a301 = {
                let a295 = {
                    let a293 = v1;
                    let a294 = 128u8;
                    fixed_u8::bit_and(a293, a294)
                };
                let a296 = 128u8;
                if equal(a295, a296) {
                    128u64
                } else {
                    0u64
                }
            };
            let a302 = {
                let a299 = {
                    let a297 = v1;
                    let a298 = 64u8;
                    fixed_u8::bit_and(a297, a298)
                };
                let a300 = 64u8;
                if equal(a299, a300) {
                    64u64
                } else {
                    0u64
                }
            };
            nat_add(a301, a302)?
        };
        let a314 = {
            let a311 = {
                let a305 = {
                    let a303 = v1;
                    let a304 = 32u8;
                    fixed_u8::bit_and(a303, a304)
                };
                let a306 = 32u8;
                if equal(a305, a306) {
                    32u64
                } else {
                    0u64
                }
            };
            let a312 = {
                let a309 = {
                    let a307 = v1;
                    let a308 = 16u8;
                    fixed_u8::bit_and(a307, a308)
                };
                let a310 = 16u8;
                if equal(a309, a310) {
                    16u64
                } else {
                    0u64
                }
            };
            nat_add(a311, a312)?
        };
        nat_add(a313, a314)?
    };
    let a338 = {
        let a335 = {
            let a323 = {
                let a317 = {
                    let a315 = v1;
                    let a316 = 8u8;
                    fixed_u8::bit_and(a315, a316)
                };
                let a318 = 8u8;
                if equal(a317, a318) {
                    8u64
                } else {
                    0u64
                }
            };
            let a324 = {
                let a321 = {
                    let a319 = v1;
                    let a320 = 4u8;
                    fixed_u8::bit_and(a319, a320)
                };
                let a322 = 4u8;
                if equal(a321, a322) {
                    4u64
                } else {
                    0u64
                }
            };
            nat_add(a323, a324)?
        };
        let a336 = {
            let a333 = {
                let a327 = {
                    let a325 = v1;
                    let a326 = 2u8;
                    fixed_u8::bit_and(a325, a326)
                };
                let a328 = 2u8;
                if equal(a327, a328) {
                    2u64
                } else {
                    0u64
                }
            };
            let a334 = {
                let a331 = {
                    let a329 = v1;
                    let a330 = 1u8;
                    fixed_u8::bit_and(a329, a330)
                };
                let a332 = 1u8;
                if equal(a331, a332) {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a333, a334)?
        };
        nat_add(a335, a336)?
    };
    nat_add(a337, a338)
}

pub fn f63(v0: u64) -> List<Str> {
    let a339 = v0;
    let a340 = 0u64;
    if nat_eq(a339, a340) {
        f102()
    } else {
        let a341 = v0;
        let a342 = 1u64;
        if nat_eq(a341, a342) {
            f103()
        } else {
            let a343 = v0;
            let a344 = 2u64;
            if nat_eq(a343, a344) {
                f104()
            } else {
                let a345 = v0;
                let a346 = 3u64;
                if nat_eq(a345, a346) {
                    f105()
                } else {
                    let a347 = v0;
                    let a348 = 4u64;
                    if nat_eq(a347, a348) {
                        f106()
                    } else {
                        let a349 = v0;
                        let a350 = 5u64;
                        if nat_eq(a349, a350) {
                            f107()
                        } else {
                            let a351 = v0;
                            let a352 = 6u64;
                            if nat_eq(a351, a352) {
                                f108()
                            } else {
                                let a353 = v0;
                                let a354 = 7u64;
                                if nat_eq(a353, a354) {
                                    f109()
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

pub fn f64(v0: Bytes) -> R<Bytes> {
    f110(v0.clone(), 0u64, {
        let a355 = v0;
        length_bytes(a355)
    }, false, false, 0u64, Bytes::lit(&[]))
}

pub fn f65(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt2, u64)>> {
    let m356 = v2;
    if m356 == 0 {
        Ok(None::<(Adt2, u64)>)
    } else {
        let v4 = m356 - 1;
        let v5: u64 = f111(v0.clone(), v1)?;
        let a357 = v3;
        let a358 = 1u64;
        if nat_eq(a357, a358) {
            let m359 = f65(v0.clone(), v1, v4, 0u64)?;
            match m359 {
                None => {
                    Ok(None::<(Adt2, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f111(v0.clone(), {
                        let (_, h360) = v6.clone();
                        h360
                    })?;
                    let a361 = v7;
                    let a362 = 44u64;
                    if nat_eq(a361, a362) {
                        let m366 = f65(v0, {
                            let a364 = {
                                let (_, h363) = v6;
                                h363
                            };
                            let a365 = 1u64;
                            nat_add(a364, a365)?
                        }, v4, 1u64)?;
                        match m366 {
                            None => {
                                Ok(None::<(Adt2, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt2::C10(std::rc::Rc::new({
                                    let (h367, _) = v6;
                                    h367
                                }), std::rc::Rc::new({
                                    let (h368, _) = v8;
                                    h368
                                })), {
                                    let (_, h369) = v8;
                                    h369
                                })))
                            }
                        }
                    } else {
                        let a370 = v7;
                        let a371 = 93u64;
                        if nat_eq(a370, a371) {
                            Ok(Some((Adt2::C10(std::rc::Rc::new({
                                let (h372, _) = v6;
                                h372
                            }), std::rc::Rc::new(Adt2::C9)), {
                                let a374 = {
                                    let (_, h373) = v6;
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
            let a376 = v3;
            let a377 = 2u64;
            if nat_eq(a376, a377) {
                let m380 = f112(v0.clone(), {
                    let a378 = v1;
                    let a379 = 1u64;
                    nat_add(a378, a379)?
                })?;
                match m380 {
                    None => {
                        Ok(None::<(Adt2, u64)>)
                    }
                    Some(v9) => {
                        if f113({
                            let a381 = v5;
                            let a382 = 34u64;
                            nat_eq(a381, a382)
                        }, {
                            let a384 = f111(v0.clone(), {
                                let (h383, _) = v9.clone();
                                h383
                            })?;
                            let a385 = 58u64;
                            nat_eq(a384, a385)
                        }) {
                            let m389 = f65(v0.clone(), {
                                let a387 = {
                                    let (h386, _) = v9;
                                    h386
                                };
                                let a388 = 1u64;
                                nat_add(a387, a388)?
                            }, v4, 0u64)?;
                            match m389 {
                                None => {
                                    Ok(None::<(Adt2, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f111(v0.clone(), {
                                        let (_, h390) = v10.clone();
                                        h390
                                    })?;
                                    let a391 = v11;
                                    let a392 = 44u64;
                                    if nat_eq(a391, a392) {
                                        let m396 = f65(v0, {
                                            let a394 = {
                                                let (_, h393) = v10;
                                                h393
                                            };
                                            let a395 = 1u64;
                                            nat_add(a394, a395)?
                                        }, v4, 2u64)?;
                                        match m396 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt2::C11({
                                                    let (_, h397) = v9;
                                                    h397
                                                }, std::rc::Rc::new({
                                                    let (h398, _) = v10;
                                                    h398
                                                }), std::rc::Rc::new({
                                                    let (h399, _) = v12;
                                                    h399
                                                })), {
                                                    let (_, h400) = v12;
                                                    h400
                                                })))
                                            }
                                        }
                                    } else {
                                        let a401 = v11;
                                        let a402 = 125u64;
                                        if nat_eq(a401, a402) {
                                            Ok(Some((Adt2::C11({
                                                let (_, h403) = v9;
                                                h403
                                            }, std::rc::Rc::new({
                                                let (h404, _) = v10;
                                                h404
                                            }), std::rc::Rc::new(Adt2::C9)), {
                                                let a406 = {
                                                    let (_, h405) = v10;
                                                    h405
                                                };
                                                let a407 = 1u64;
                                                nat_add(a406, a407)?
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
                let a408 = v5;
                let a409 = 34u64;
                if nat_eq(a408, a409) {
                    let m412 = f112(v0, {
                        let a410 = v1;
                        let a411 = 1u64;
                        nat_add(a410, a411)?
                    })?;
                    match m412 {
                        None => {
                            Ok(None::<(Adt2, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt2::C6({
                                let (_, h413) = v13;
                                h413
                            }), {
                                let (h414, _) = v13;
                                h414
                            })))
                        }
                    }
                } else {
                    if f32(f46(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                        Ok(Some((Adt2::C0, {
                            let a415 = v1;
                            let a416 = 4u64;
                            nat_add(a415, a416)?
                        })))
                    } else {
                        if f32(f46(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                            Ok(Some((Adt2::C2(true), {
                                let a417 = v1;
                                let a418 = 4u64;
                                nat_add(a417, a418)?
                            })))
                        } else {
                            if f32(f46(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                                Ok(Some((Adt2::C2(false), {
                                    let a419 = v1;
                                    let a420 = 5u64;
                                    nat_add(a419, a420)?
                                })))
                            } else {
                                let a421 = v5;
                                let a422 = 91u64;
                                if nat_eq(a421, a422) {
                                    let a425 = f111(v0.clone(), {
                                        let a423 = v1;
                                        let a424 = 1u64;
                                        nat_add(a423, a424)?
                                    })?;
                                    let a426 = 93u64;
                                    if nat_eq(a425, a426) {
                                        Ok(Some((Adt2::C7(std::rc::Rc::new(Adt2::C9)), {
                                            let a427 = v1;
                                            let a428 = 2u64;
                                            nat_add(a427, a428)?
                                        })))
                                    } else {
                                        let m431 = f65(v0, {
                                            let a429 = v1;
                                            let a430 = 1u64;
                                            nat_add(a429, a430)?
                                        }, v4, 1u64)?;
                                        match m431 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v14) => {
                                                Ok(Some((Adt2::C7(std::rc::Rc::new({
                                                    let (h432, _) = v14;
                                                    h432
                                                })), {
                                                    let (_, h433) = v14;
                                                    h433
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let a434 = v5;
                                    let a435 = 123u64;
                                    if nat_eq(a434, a435) {
                                        let a438 = f111(v0.clone(), {
                                            let a436 = v1;
                                            let a437 = 1u64;
                                            nat_add(a436, a437)?
                                        })?;
                                        let a439 = 125u64;
                                        if nat_eq(a438, a439) {
                                            Ok(Some((Adt2::C8(std::rc::Rc::new(Adt2::C9)), {
                                                let a440 = v1;
                                                let a441 = 2u64;
                                                nat_add(a440, a441)?
                                            })))
                                        } else {
                                            let m444 = f65(v0, {
                                                let a442 = v1;
                                                let a443 = 1u64;
                                                nat_add(a442, a443)?
                                            }, v4, 2u64)?;
                                            match m444 {
                                                None => {
                                                    Ok(None::<(Adt2, u64)>)
                                                }
                                                Some(v15) => {
                                                    Ok(Some((Adt2::C8(std::rc::Rc::new({
                                                        let (h445, _) = v15;
                                                        h445
                                                    })), {
                                                        let (_, h446) = v15;
                                                        h446
                                                    })))
                                                }
                                            }
                                        }
                                    } else {
                                        let m447 = f114(v0, v1)?;
                                        match m447 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v16) => {
                                                Ok(Some(({
                                                    let (_, h448) = v16;
                                                    h448
                                                }, {
                                                    let (h449, _) = v16;
                                                    h449
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

pub fn f66(v0: Fn0, p454: u64, p455: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m453 = (p454, p455);
    {
        let mut m453 = m453;
        loop {
            let step = {
                let (v1, v2) = m453;
                let m450 = v1;
                if m450 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m450 - 1;
                    let m452 = {
                        let c451: Fn0 = v0.clone();
                        c451.apply(v2.clone())?
                    };
                    match m452 {
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
                Ok(next) => { m453 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f67(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f115(v1, v0, v2)
}

pub fn f68(v0: List<Bytes>) -> List<Bytes> {
    f116(Fn2::F117, List::<Bytes>::nil(), v0)
}

pub fn f69(v0: Fn1, p459: Option<Bytes>, p460: List<Bytes>) -> Option<Bytes> {
    let m458 = (p459, p460);
    {
        let mut m458 = m458;
        loop {
            let step = {
                let (v1, v2) = m458;
                let m456 = v2;
                match m456.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c457: Fn1 = v0.clone();
                            c457.apply(v1, v3)
                        }, v4))
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

pub fn f70(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m461 = v1;
    match m461 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a464 = {
                    let a462 = v3;
                    let a463 = v0;
                    append_bytes(a462, a463)
                };
                let a465 = v2;
                append_bytes(a464, a465)
            })
        }
    }
}

pub fn f71(p470: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m469 = p470;
    {
        let mut m469 = m469;
        loop {
            let step = {
                let v0 = m469;
                let m466 = v0;
                match m466.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h467, _) = v2;
                            h467
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h468) = v2;
                                h468
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m469 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f72(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Bytes>)> {
    let m474 = {
        let a472 = match v0 {
            Adt1::C0(h471, _, _, _) => {
                h471
            }
        };
        let a473 = f40(v1);
        index_list(a472, a473)
    };
    match m474 {
        None => {
            List::<(Bytes, Option<Bytes>)>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f73(p480: Adt2, v1: Bytes) -> Option<Adt2> {
    let m479 = p480;
    {
        let mut m479 = m479;
        loop {
            let step = {
                let v0 = m479;
                let m475 = v0;
                match m475 {
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
                    Adt2::C8(r476) => {
                        let v5 = (*r476).clone();
                        Ok::<Adt2, Option<Adt2>>(v5)
                    }
                    Adt2::C9 => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C10(_, _) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C11(v2, r477, r478) => {
                        let v3 = (*r477).clone();
                        let v4 = (*r478).clone();
                        if f32(v2, v1.clone()) {
                            Err::<Adt2, Option<Adt2>>(Some(v3))
                        } else {
                            Ok::<Adt2, Option<Adt2>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m479 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f74(v0: Option<Adt2>) -> Option<Bytes> {
    let m481 = v0;
    match m481 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m482 = v1;
            match m482 {
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

pub fn f75(v0: List<Adt2>) -> List<Bytes> {
    let m483 = v0;
    match m483.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m484 = f74(Some(v1));
            match m484 {
                None => {
                    f75(v2)
                }
                Some(v3) => {
                    List::cons(v3, f75(v2))
                }
            }
        }
    }
}

pub fn f76(v0: Option<Adt2>) -> List<Adt2> {
    let m485 = v0;
    match m485 {
        None => {
            List::<Adt2>::nil()
        }
        Some(v1) => {
            f118(v1)
        }
    }
}

pub fn f77(v0: Bytes) -> R<Bytes> {
    Ok(f119(f120(v0)?))
}

pub fn f78(v0: Adt1, v1: Bytes) -> R<Option<Adt2>> {
    f121(v0, v1)
}

pub fn f79(v0: Bytes) -> bool {
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

pub fn f80(v0: List<Bytes>) -> Option<Adt2> {
    let m486 = v0;
    match m486.uncons() {
        None => {
            None::<Adt2>
        }
        Some((v1, v2)) => {
            if f32(v1.clone(), f8(Str::lit("req"))) {
                let m487 = v2;
                match m487.uncons() {
                    None => {
                        None::<Adt2>
                    }
                    Some((v3, _)) => {
                        Some(Adt2::C11(f8(Str::lit("req")), std::rc::Rc::new(Adt2::C6(v3)), std::rc::Rc::new(Adt2::C9)))
                    }
                }
            } else {
                if f32(v1.clone(), f8(Str::lit("std"))) {
                    let m488 = v2;
                    match m488.uncons() {
                        None => {
                            None::<Adt2>
                        }
                        Some((v5, v6)) => {
                            Some(Adt2::C11(f8(Str::lit("std")), std::rc::Rc::new(Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("authority")), std::rc::Rc::new(Adt2::C6(v5)), std::rc::Rc::new({
                                let m489 = v6;
                                match m489.uncons() {
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
                        let m490 = v2;
                        match m490.uncons() {
                            None => {
                                None::<Adt2>
                            }
                            Some((v9, v10)) => {
                                let m491 = v10;
                                match m491.uncons() {
                                    None => {
                                        None::<Adt2>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt2::C11(f8(Str::lit("der")), std::rc::Rc::new(Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("check")), std::rc::Rc::new(Adt2::C6(v11)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("inputs")), std::rc::Rc::new(Adt2::C7(std::rc::Rc::new(f86(v12)))), std::rc::Rc::new(Adt2::C11(f8(Str::lit("rule")), std::rc::Rc::new(Adt2::C6(v9)), std::rc::Rc::new(Adt2::C9))))))))), std::rc::Rc::new(Adt2::C9)))
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

pub fn f81(p494: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m493 = p494;
    {
        let mut m493 = m493;
        loop {
            let step = {
                let v0 = m493;
                let m492 = v0;
                match m492.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f122(v2.clone(), v1.clone()) {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v2))
                        } else {
                            Ok::<List<Bytes>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m493 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f82(v0: List<Adt2>) -> List<Bytes> {
    let m495 = v0;
    match m495.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m496 = f73(v1, f8(Str::lit("std")));
            match m496 {
                None => {
                    f82(v2)
                }
                Some(v3) => {
                    List::cons(f123(v3, f8(Str::lit("row"))), f82(v2))
                }
            }
        }
    }
}

pub fn f83(v0: List<Adt2>) -> List<Bytes> {
    let m497 = v0;
    match m497.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f124(f75(f76(f73(v1, f8(Str::lit("defers"))))), f83(v2))
        }
    }
}

pub fn f84(v0: Bytes) -> Adt12 {
    Adt12::C4(f7(List::cons(f8(Str::lit("index row ")), List::cons(v0, List::cons(f8(Str::lit(" is deferred by an owner decision, so nothing that cites it is admitted\u{a}")), List::<Bytes>::nil())))))
}

pub fn f85(v0: Adt1, v1: Bytes, v2: Adt2, v3: Bytes, v4: Bytes, v5: List<(Bytes, Bytes)>) -> R<Adt12> {
    let v6: Bytes = {
        let a498 = f8(Str::lit("Catalog."));
        let a499 = f125(v1.clone())?;
        append_bytes(a498, a499)
    };
    let v7: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f125(v1)?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
    Ok(Adt12::C3(f126(v5, List::cons((v7.clone(), v3), List::cons((f8(Str::lit("model/admissions.json")), f127(Adt2::C7(std::rc::Rc::new(f128(f129(f51(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v6.clone(), Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("construct")), std::rc::Rc::new(Adt2::C6(v6.clone())), std::rc::Rc::new(v2)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f127(Adt2::C7(std::rc::Rc::new(f128(f129(f51(v0.clone(), f8(Str::lit("model/strata.json")))?, f8(Str::lit("path")), v7.clone(), Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("path")), std::rc::Rc::new(Adt2::C6(v7.clone())), std::rc::Rc::new(Adt2::C11(f8(Str::lit("role")), std::rc::Rc::new(Adt2::C6(v4)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("stratum")), std::rc::Rc::new(Adt2::C3(false, 0u64)), std::rc::Rc::new(Adt2::C9)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
        let a503 = f130(f90(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f46(v7.clone(), 4u64, {
            let a501 = {
                let a500 = v7;
                length_bytes(a500)
            };
            let a502 = 4u64;
            nat_sub(a501, a502)
        }))?;
        let a504 = f8(Str::lit("\u{a}"));
        append_bytes(a503, a504)
    }), List::cons(f131(v0, List::cons(v6.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v6, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f86(v0: List<Bytes>) -> Adt2 {
    let m505 = v0;
    match m505.uncons() {
        None => {
            Adt2::C9
        }
        Some((v1, v2)) => {
            Adt2::C10(std::rc::Rc::new(Adt2::C6(v1)), std::rc::Rc::new(f86(v2)))
        }
    }
}

pub fn f87(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\\begin{lexlean}{Catalog.")), List::cons(f125(v0)?, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(v1, List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil())))))))
}

pub fn f88(v0: Bytes) -> R<Bytes> {
    f132({
        let a507 = {
            let a506 = v0.clone();
            length_bytes(a506)
        };
        let a508 = 1u64;
        nat_add(a507, a508)?
    }, v0)
}

pub fn f89(v0: Bytes) -> R<Result<List<Adt2>, Bytes>> {
    let v1: Bytes = if f133(v0.clone(), Bytes::lit(&[0xef, 0xbb, 0xbf]), 0u64) {
        f46(v0.clone(), 3u64, {
            let a510 = {
                let a509 = v0;
                length_bytes(a509)
            };
            let a511 = 3u64;
            nat_sub(a510, a511)
        })
    } else {
        v0
    };
    let v2: Bytes = f134(v1)?;
    let v3: u64 = {
        let a512 = v2.clone();
        length_bytes(a512)
    };
    let v4: u64 = f135(v2.clone())?;
    let v5: u64 = f136(v2.clone(), 0u64)?;
    let a513 = v4;
    let a514 = v3;
    if nat_lt(a513, a514) {
        Ok(Err::<List<Adt2>, Bytes>(f137(Str::lit("the input is not a sequence of printable characters in UTF-8 (YAML 1.2.2 [1], section 5.2)"), v4)?))
    } else {
        let m517 = f138(v2.clone(), v5, {
            let a515 = v3;
            let a516 = 1u64;
            nat_add(a515, a516)?
        })?;
        match m517 {
            Ok(v7) => {
                let a518 = v3;
                let a519 = v7;
                if nat_le(a518, a519) {
                    Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
                } else {
                    f139(v2, v7, true, {
                        let a520 = v3;
                        let a521 = 1u64;
                        nat_add(a520, a521)?
                    })
                }
            }
            Err(v6) => {
                Ok(Err::<List<Adt2>, Bytes>(v6))
            }
        }
    }
}

pub fn f90(v0: Adt1, v1: Bytes) -> Bytes {
    let m522 = f71(f72(v0, v1.clone()), v1);
    match m522 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m523 = v2;
            match m523 {
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

pub fn f91(v0: u64) -> R<Bytes> {
    f140(f141(), 10u64, v0)
}

pub fn f92(v0: List<Adt2>) -> R<u64> {
    let m524 = v0;
    match m524.uncons() {
        None => {
            Ok(0u64)
        }
        Some((_, v2)) => {
            let a525 = f92(v2)?;
            let a526 = 1u64;
            nat_add(a525, a526)
        }
    }
}

pub fn f93(v0: Bytes, v1: List<Bytes>) -> R<Result<(u64, Bytes), Bytes>> {
    let m527 = f142(v0)?;
    match m527 {
        Ok(v3) => {
            let m528 = f143(v1)?;
            match m528 {
                Ok(v5) => {
                    let v6: List<Adt13> = f144(v3.clone(), v5);
                    let v7: List<Adt16> = f145({
                        let a530 = {
                            let a529 = v6.clone();
                            length_list(a529)
                        };
                        let a531 = 2u64;
                        nat_add(a530, a531)?
                    }, List::cons(Adt16::C0(f146(), Bytes::lit(&[]), f147(false), List::<Bytes>::nil()), List::cons(Adt16::C0(f148(), Bytes::lit(&[]), f147(true), List::<Bytes>::nil()), f149(v6.clone(), v6.clone())?)))?;
                    let v8: List<Bytes> = f150({
                        let a533 = {
                            let a532 = v7.clone();
                            length_list(a532)
                        };
                        let a534 = 1u64;
                        nat_add(a533, a534)?
                    }, f151(v3.clone()), List::<Bytes>::nil(), v7.clone());
                    let v9: List<(Bytes, (List<Bytes>, Bytes))> = f152(v7.clone(), v8);
                    let v10: (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>) = f153({
                        let a536 = {
                            let a535 = v9.clone();
                            length_list(a535)
                        };
                        let a537 = 1u64;
                        nat_add(a536, a537)?
                    }, v9, List::<Bytes>::nil(), List::<Bytes>::nil());
                    Ok(Ok::<(u64, Bytes), Bytes>(({
                        let a539 = {
                            let (h538, _) = v10.clone();
                            h538
                        };
                        length_list(a539)
                    }, f7(List::cons(f7(f154(v3, v7, v6)), List::cons(f7(f155({
                        let (_, h540) = v10;
                        h540
                    })), List::cons(f8(Str::lit("-- operations, constants and the other members that are not attributes are behavior, not data, and are not translated (Web IDL 2.5)\u{a}")), List::cons(f156(f157({
                        let (h541, _) = v10;
                        h541
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

pub fn f94(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m542 = v1;
    match m542.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f90(v0.clone(), v2), f94(v0, v3))
        }
    }
}

pub fn f95(v0: Bytes) -> R<Result<Adt17, Bytes>> {
    let v1: Bytes = if f133(v0.clone(), f8(Str::lit("\u{feff}")), 0u64) {
        f46(v0.clone(), 3u64, {
            let a544 = {
                let a543 = v0;
                length_bytes(a543)
            };
            let a545 = 3u64;
            nat_sub(a544, a545)
        })
    } else {
        v0
    };
    let v2: Bytes = f134(v1)?;
    let v3: u64 = {
        let a546 = v2.clone();
        length_bytes(a546)
    };
    let v4: u64 = f158(v2.clone())?;
    let a547 = v4;
    let a548 = v3;
    if nat_lt(a547, a548) {
        Ok(f159(f137(Str::lit("the input is not a sequence of Chars in UTF-8 (XML 1.0 [2], section 4.3.3)"), v4)?))
    } else {
        let m549 = f160(v2.clone())?;
        match m549 {
            Ok(v6) => {
                let m552 = f161(v2.clone(), v6, {
                    let a550 = v3;
                    let a551 = 2u64;
                    nat_add(a550, a551)?
                })?;
                match m552 {
                    Ok(v8) => {
                        if f162(v2.clone(), v8, Str::lit("<!DOCTYPE")) {
                            Ok(f159(f137(Str::lit("a document type declaration is not read; Verdant Studio reads documents that have none (XML 1.0 [28])"), v8)?))
                        } else {
                            let a553 = f163(v2.clone(), v8)?;
                            let a554 = 60u64;
                            if nat_eq(a553, a554) {
                                let m557 = f164(v2.clone(), v8, {
                                    let a555 = v3;
                                    let a556 = 2u64;
                                    nat_add(a555, a556)?
                                }, 0u64)?;
                                match m557 {
                                    Ok(v10) => {
                                        let m561 = f161(v2, {
                                            let (_, h558) = v10;
                                            h558
                                        }, {
                                            let a559 = v3;
                                            let a560 = 2u64;
                                            nat_add(a559, a560)?
                                        })?;
                                        match m561 {
                                            Ok(v12) => {
                                                let a562 = v12;
                                                let a563 = v3;
                                                if nat_eq(a562, a563) {
                                                    Ok(Ok::<Adt17, Bytes>({
                                                        let (h564, _) = v10;
                                                        h564
                                                    }))
                                                } else {
                                                    Ok(f159(f137(Str::lit("only comments, processing instructions and white space may follow the root element (XML 1.0 [1], [27])"), v12)?))
                                                }
                                            }
                                            Err(v11) => {
                                                Ok(f159(v11))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Ok(f159(v9))
                                    }
                                }
                            } else {
                                Ok(f159(f137(Str::lit("a document must have exactly one root element (XML 1.0 [1])"), v8)?))
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(f159(v7))
                    }
                }
            }
            Err(v5) => {
                Ok(f159(v5))
            }
        }
    }
}

pub fn f96(v0: Adt17) -> Bytes {
    let m565 = v0;
    match m565 {
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

pub fn f97(v0: Adt17) -> R<u64> {
    let m566 = v0;
    match m566 {
        Adt17::C0(_, _, r571) => {
            let v3 = (*r571).clone();
            let a567 = 1u64;
            let a568 = f97(v3)?;
            nat_add(a567, a568)
        }
        Adt17::C1(_, _, _) => {
            Ok(0u64)
        }
        Adt17::C2(_) => {
            Ok(0u64)
        }
        Adt17::C3(r572, r573) => {
            let v8 = (*r572).clone();
            let v9 = (*r573).clone();
            let a569 = f97(v8)?;
            let a570 = f97(v9)?;
            nat_add(a569, a570)
        }
        Adt17::C4 => {
            Ok(0u64)
        }
    }
}

pub fn f98(v0: Bytes) -> R<Result<(u64, Bytes), Bytes>> {
    let m574 = f95(v0)?;
    match m574 {
        Ok(v2) => {
            let m575 = v2;
            match m575 {
                Adt17::C0(v3, r587, r588) => {
                    let v4 = (*r587).clone();
                    let v5 = (*r588).clone();
                    let m576 = f166(v4.clone(), List::<(Bytes, Bytes)>::nil());
                    match m576 {
                        Ok(v7) => {
                            let m577 = f167(v7.clone(), v3)?;
                            match m577 {
                                Ok(v9) => {
                                    if f168(v9, Str::lit("schema")) {
                                        let v11: Bytes = {
                                            let m578 = f169(v4, f8(Str::lit("targetNamespace")));
                                            match m578 {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some(v10) => {
                                                    v10
                                                }
                                            }
                                        };
                                        let m579 = f170(v5, v7, v11)?;
                                        match m579 {
                                            Ok(v13) => {
                                                let v14: List<Bytes> = f171(v13.clone());
                                                let m580 = f172(v14.clone());
                                                match m580 {
                                                    None => {
                                                        let m581 = f173(v13.clone(), v14.clone());
                                                        match m581 {
                                                            None => {
                                                                let m585 = f174({
                                                                    let a583 = {
                                                                        let a582 = v14.clone();
                                                                        length_list(a582)
                                                                    };
                                                                    let a584 = 1u64;
                                                                    nat_add(a583, a584)?
                                                                }, v13, List::<Bytes>::nil(), List::<Bytes>::nil());
                                                                match m585 {
                                                                    Ok(v18) => {
                                                                        Ok(Ok::<(u64, Bytes), Bytes>(({
                                                                            let a586 = v14;
                                                                            length_list(a586)
                                                                        }, f156(f157(v18, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))))))
                                                                    }
                                                                    Err(v17) => {
                                                                        Ok(f165(v17))
                                                                    }
                                                                }
                                                            }
                                                            Some(v16) => {
                                                                Ok(f165(f7(List::cons(f8(Str::lit("the type ")), List::cons(v16, List::cons(f8(Str::lit(" is not a simple type of this schema document")), List::<Bytes>::nil()))))))
                                                            }
                                                        }
                                                    }
                                                    Some(v15) => {
                                                        Ok(f165(f7(List::cons(f8(Str::lit("two simple types derive the type name ")), List::cons(v15, List::<Bytes>::nil())))))
                                                    }
                                                }
                                            }
                                            Err(v12) => {
                                                Ok(f165(v12))
                                            }
                                        }
                                    } else {
                                        Ok(f165(f8(Str::lit("the root element must be xs:schema in the XML Schema namespace (XSD 1.1 Part 1, section 3.17.2)"))))
                                    }
                                }
                                Err(v8) => {
                                    Ok(f165(v8))
                                }
                            }
                        }
                        Err(v6) => {
                            Ok(f165(v6))
                        }
                    }
                }
                Adt17::C1(_, _, _) => {
                    Ok(f165(f8(Str::lit("the document has no root element"))))
                }
                Adt17::C2(_) => {
                    Ok(f165(f8(Str::lit("the document has no root element"))))
                }
                Adt17::C3(_, _) => {
                    Ok(f165(f8(Str::lit("the document has no root element"))))
                }
                Adt17::C4 => {
                    Ok(f165(f8(Str::lit("the document has no root element"))))
                }
            }
        }
        Err(v1) => {
            Ok(f165(f7(List::cons(f8(Str::lit("not well-formed XML: ")), List::cons(v1, List::<Bytes>::nil())))))
        }
    }
}

pub fn f99(v0: List<(Bytes, Option<Bytes>)>, v1: Bytes, v2: Bytes) -> R<Adt18> {
    let v3: List<(Bytes, Bytes)> = f175(v1)?;
    f176({
        let a590 = {
            let a589 = v3.clone();
            length_list(a589)
        };
        let a591 = 2u64;
        nat_add(a590, a591)?
    }, v0, v3, List::cons((v2, (f8(Str::lit("root")), Bytes::lit(&[]))), List::<(Bytes, (Bytes, Bytes))>::nil()), List::<Adt19>::nil(), List::<Bytes>::nil())
}

pub fn f100(v0: Adt1) -> List<(Bytes, Option<Bytes>)> {
    f177(match v0 {
        Adt1::C0(h592, _, _, _) => {
            h592
        }
    })
}

pub fn f101(v0: List<Adt2>) -> R<Result<Bytes, Bytes>> {
    let m593 = v0;
    match m593.uncons() {
        None => {
            Ok(Ok::<Bytes, Bytes>(Bytes::lit(&[])))
        }
        Some((v1, v2)) => {
            let m594 = f178(v1)?;
            match m594 {
                Ok(v4) => {
                    let m595 = f101(v2)?;
                    match m595 {
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

pub fn f102() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{9}"), List::cons(Str::lit("\u{a}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{d}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f103() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f104() -> List<Str> {
    List::cons(Str::lit(" "), List::cons(Str::lit("!"), List::cons(Str::lit("\""), List::cons(Str::lit("#"), List::cons(Str::lit("$"), List::cons(Str::lit("%"), List::cons(Str::lit("&"), List::cons(Str::lit("'"), List::cons(Str::lit("("), List::cons(Str::lit(")"), List::cons(Str::lit("*"), List::cons(Str::lit("+"), List::cons(Str::lit(","), List::cons(Str::lit("-"), List::cons(Str::lit("."), List::cons(Str::lit("/"), List::<Str>::nil()))))))))))))))))
}

pub fn f105() -> List<Str> {
    List::cons(Str::lit("0"), List::cons(Str::lit("1"), List::cons(Str::lit("2"), List::cons(Str::lit("3"), List::cons(Str::lit("4"), List::cons(Str::lit("5"), List::cons(Str::lit("6"), List::cons(Str::lit("7"), List::cons(Str::lit("8"), List::cons(Str::lit("9"), List::cons(Str::lit(":"), List::cons(Str::lit(";"), List::cons(Str::lit("<"), List::cons(Str::lit("="), List::cons(Str::lit(">"), List::cons(Str::lit("?"), List::<Str>::nil()))))))))))))))))
}

pub fn f106() -> List<Str> {
    List::cons(Str::lit("@"), List::cons(Str::lit("A"), List::cons(Str::lit("B"), List::cons(Str::lit("C"), List::cons(Str::lit("D"), List::cons(Str::lit("E"), List::cons(Str::lit("F"), List::cons(Str::lit("G"), List::cons(Str::lit("H"), List::cons(Str::lit("I"), List::cons(Str::lit("J"), List::cons(Str::lit("K"), List::cons(Str::lit("L"), List::cons(Str::lit("M"), List::cons(Str::lit("N"), List::cons(Str::lit("O"), List::<Str>::nil()))))))))))))))))
}

pub fn f107() -> List<Str> {
    List::cons(Str::lit("P"), List::cons(Str::lit("Q"), List::cons(Str::lit("R"), List::cons(Str::lit("S"), List::cons(Str::lit("T"), List::cons(Str::lit("U"), List::cons(Str::lit("V"), List::cons(Str::lit("W"), List::cons(Str::lit("X"), List::cons(Str::lit("Y"), List::cons(Str::lit("Z"), List::cons(Str::lit("["), List::cons(Str::lit("\\"), List::cons(Str::lit("]"), List::cons(Str::lit("^"), List::cons(Str::lit("_"), List::<Str>::nil()))))))))))))))))
}

pub fn f108() -> List<Str> {
    List::cons(Str::lit("`"), List::cons(Str::lit("a"), List::cons(Str::lit("b"), List::cons(Str::lit("c"), List::cons(Str::lit("d"), List::cons(Str::lit("e"), List::cons(Str::lit("f"), List::cons(Str::lit("g"), List::cons(Str::lit("h"), List::cons(Str::lit("i"), List::cons(Str::lit("j"), List::cons(Str::lit("k"), List::cons(Str::lit("l"), List::cons(Str::lit("m"), List::cons(Str::lit("n"), List::cons(Str::lit("o"), List::<Str>::nil()))))))))))))))))
}

pub fn f109() -> List<Str> {
    List::cons(Str::lit("p"), List::cons(Str::lit("q"), List::cons(Str::lit("r"), List::cons(Str::lit("s"), List::cons(Str::lit("t"), List::cons(Str::lit("u"), List::cons(Str::lit("v"), List::cons(Str::lit("w"), List::cons(Str::lit("x"), List::cons(Str::lit("y"), List::cons(Str::lit("z"), List::cons(Str::lit("{"), List::cons(Str::lit("|"), List::cons(Str::lit("}"), List::cons(Str::lit("~"), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f110(v0: Bytes, p623: u64, p624: u64, p625: bool, p626: bool, p627: u64, p628: Bytes) -> R<Bytes> {
    let m622 = (p623, (p624, (p625, (p626, (p627, p628)))));
    {
        let mut m622 = m622;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, (v5, v6))))) = m622;
                let m596 = v2;
                if m596 == 0 {
                    Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f179(v0.clone(), v5, v1, v6)))
                } else {
                    let v7 = m596 - 1;
                    let m599 = {
                        let a597 = v0.clone();
                        let a598 = v1;
                        index_bytes(a597, a598)
                    };
                    match m599 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f179(v0.clone(), v5, v1, v6)))
                        }
                        Some(v8) => {
                            let v9: u64 = f62(v8)?;
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                    let a600 = v1;
                                    let a601 = 1u64;
                                    nat_add(a600, a601)?
                                }, (v7, (if v4 {
                                    true
                                } else {
                                    let a602 = v9;
                                    let a603 = 34u64;
                                    !nat_eq(a602, a603)
                                }, (if v4 {
                                    false
                                } else {
                                    let a604 = v9;
                                    let a605 = 92u64;
                                    nat_eq(a604, a605)
                                }, (v5, v6))))))
                            } else {
                                let a606 = v9;
                                let a607 = 32u64;
                                if if if if nat_eq(a606, a607) {
                                    true
                                } else {
                                    let a608 = v9;
                                    let a609 = 9u64;
                                    nat_eq(a608, a609)
                                } {
                                    true
                                } else {
                                    let a610 = v9;
                                    let a611 = 10u64;
                                    nat_eq(a610, a611)
                                } {
                                    true
                                } else {
                                    let a612 = v9;
                                    let a613 = 13u64;
                                    nat_eq(a612, a613)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a614 = v1;
                                        let a615 = 1u64;
                                        nat_add(a614, a615)?
                                    }, (v7, (false, (false, ({
                                        let a616 = v1;
                                        let a617 = 1u64;
                                        nat_add(a616, a617)?
                                    }, f179(v0.clone(), v5, v1, v6)))))))
                                } else {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a618 = v1;
                                        let a619 = 1u64;
                                        nat_add(a618, a619)?
                                    }, (v7, ({
                                        let a620 = v9;
                                        let a621 = 34u64;
                                        nat_eq(a620, a621)
                                    }, (false, (v5, v6))))))
                                }
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

pub fn f111(v0: Bytes, v1: u64) -> R<u64> {
    let m631 = {
        let a629 = v0;
        let a630 = v1;
        index_bytes(a629, a630)
    };
    match m631 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a676 = {
                let a652 = {
                    let a640 = {
                        let a634 = {
                            let a632 = v3;
                            let a633 = 128u8;
                            fixed_u8::bit_and(a632, a633)
                        };
                        let a635 = 128u8;
                        if equal(a634, a635) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a641 = {
                        let a638 = {
                            let a636 = v3;
                            let a637 = 64u8;
                            fixed_u8::bit_and(a636, a637)
                        };
                        let a639 = 64u8;
                        if equal(a638, a639) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a640, a641)?
                };
                let a653 = {
                    let a650 = {
                        let a644 = {
                            let a642 = v3;
                            let a643 = 32u8;
                            fixed_u8::bit_and(a642, a643)
                        };
                        let a645 = 32u8;
                        if equal(a644, a645) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a651 = {
                        let a648 = {
                            let a646 = v3;
                            let a647 = 16u8;
                            fixed_u8::bit_and(a646, a647)
                        };
                        let a649 = 16u8;
                        if equal(a648, a649) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a650, a651)?
                };
                nat_add(a652, a653)?
            };
            let a677 = {
                let a674 = {
                    let a662 = {
                        let a656 = {
                            let a654 = v3;
                            let a655 = 8u8;
                            fixed_u8::bit_and(a654, a655)
                        };
                        let a657 = 8u8;
                        if equal(a656, a657) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a663 = {
                        let a660 = {
                            let a658 = v3;
                            let a659 = 4u8;
                            fixed_u8::bit_and(a658, a659)
                        };
                        let a661 = 4u8;
                        if equal(a660, a661) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a662, a663)?
                };
                let a675 = {
                    let a672 = {
                        let a666 = {
                            let a664 = v3;
                            let a665 = 2u8;
                            fixed_u8::bit_and(a664, a665)
                        };
                        let a667 = 2u8;
                        if equal(a666, a667) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a673 = {
                        let a670 = {
                            let a668 = v3;
                            let a669 = 1u8;
                            fixed_u8::bit_and(a668, a669)
                        };
                        let a671 = 1u8;
                        if equal(a670, a671) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a672, a673)?
                };
                nat_add(a674, a675)?
            };
            nat_add(a676, a677)
        }
    }
}

pub fn f112(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = f180(v0.clone(), v1, {
        let a679 = {
            let a678 = v0.clone();
            length_bytes(a678)
        };
        let a680 = 1u64;
        nat_add(a679, a680)?
    })?;
    let a681 = f111(v0.clone(), v2)?;
    let a682 = 34u64;
    if nat_eq(a681, a682) {
        Ok(Some(({
            let a683 = v2;
            let a684 = 1u64;
            nat_add(a683, a684)?
        }, f46(v0, v1, {
            let a685 = v2;
            let a686 = v1;
            nat_sub(a685, a686)
        }))))
    } else {
        f181(v0.clone(), v1, {
            let a688 = {
                let a687 = v0;
                length_bytes(a687)
            };
            let a689 = 1u64;
            nat_add(a688, a689)?
        }, Bytes::lit(&[]))
    }
}

pub fn f113(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f114(v0: Bytes, v1: u64) -> R<Option<(u64, Adt2)>> {
    let v2: bool = {
        let a690 = f111(v0.clone(), v1)?;
        let a691 = 45u64;
        nat_eq(a690, a691)
    };
    let v3: u64 = if v2 {
        let a692 = v1;
        let a693 = 1u64;
        nat_add(a692, a693)?
    } else {
        v1
    };
    let v4: u64 = f182(v0.clone(), v3)?;
    let v5: u64 = {
        let a694 = v4;
        let a695 = v3;
        nat_sub(a694, a695)
    };
    let v6: bool = {
        let a696 = f111(v0.clone(), v4)?;
        let a697 = 46u64;
        nat_eq(a696, a697)
    };
    let v7: u64 = if v6 {
        let a698 = v4;
        let a699 = 1u64;
        nat_add(a698, a699)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f182(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f111(v0.clone(), v8)?;
    let v10: bool = {
        let a700 = v9;
        let a701 = 101u64;
        if nat_eq(a700, a701) {
            true
        } else {
            let a702 = v9;
            let a703 = 69u64;
            nat_eq(a702, a703)
        }
    };
    let v11: u64 = f111(v0.clone(), {
        let a704 = v8;
        let a705 = 1u64;
        nat_add(a704, a705)?
    })?;
    let v12: bool = f113(v10, f183({
        let a706 = v11;
        let a707 = 43u64;
        nat_eq(a706, a707)
    }, {
        let a708 = v11;
        let a709 = 45u64;
        nat_eq(a708, a709)
    }));
    let v13: u64 = if v10 {
        if v12 {
            let a710 = v8;
            let a711 = 2u64;
            nat_add(a710, a711)?
        } else {
            let a712 = v8;
            let a713 = 1u64;
            nat_add(a712, a713)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f182(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f46(v0.clone(), v13, {
        let a714 = v14;
        let a715 = v13;
        nat_sub(a714, a715)
    });
    let v16: u64 = f184(v15.clone())?;
    let v17: Bytes = f46(v15.clone(), v16, {
        let a717 = {
            let a716 = v15;
            length_bytes(a716)
        };
        let a718 = v16;
        nat_sub(a717, a718)
    });
    let a719 = v5;
    let a720 = 0u64;
    if if nat_eq(a719, a720) {
        true
    } else {
        if f113({
            let a721 = 1u64;
            let a722 = v5;
            nat_lt(a721, a722)
        }, {
            let a723 = f111(v0.clone(), v3)?;
            let a724 = 48u64;
            nat_eq(a723, a724)
        }) {
            true
        } else {
            if f113(v6, {
                let a725 = v8;
                let a726 = v7;
                nat_eq(a725, a726)
            }) {
                true
            } else {
                f183(if v10 {
                    let a727 = v14;
                    let a728 = v13;
                    nat_eq(a727, a728)
                } else {
                    false
                }, {
                    let a730 = 18u64;
                    let a731 = {
                        let a729 = v17.clone();
                        length_bytes(a729)
                    };
                    nat_lt(a730, a731)
                })
            }
        }
    } {
        Ok(None::<(u64, Adt2)>)
    } else {
        let m735 = {
            let a733 = {
                let a732 = v17.clone();
                length_bytes(a732)
            };
            let a734 = 0u64;
            if nat_eq(a733, a734) {
                Some(0u64)
            } else {
                f185(v17)?
            }
        };
        match m735 {
            None => {
                Ok(None::<(u64, Adt2)>)
            }
            Some(v18) => {
                let m740 = f186(f46(v0.clone(), v3, v5), f46(v0, v7, {
                    let a736 = v8;
                    let a737 = v7;
                    nat_sub(a736, a737)
                }), f113(v12, {
                    let a738 = v11;
                    let a739 = 45u64;
                    nat_eq(a738, a739)
                }), v18)?;
                match m740 {
                    None => {
                        Ok(None::<(u64, Adt2)>)
                    }
                    Some(v19) => {
                        let a743 = 19u64;
                        let a744 = {
                            let a742 = {
                                let (h741, _) = v19.clone();
                                h741
                            };
                            length_bytes(a742)
                        };
                        if nat_lt(a743, a744) {
                            Ok(None::<(u64, Adt2)>)
                        } else {
                            let m746 = f185({
                                let (h745, _) = v19.clone();
                                h745
                            })?;
                            match m746 {
                                None => {
                                    Ok(None::<(u64, Adt2)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f113(v2, {
                                        let a747 = 0u64;
                                        let a748 = v20;
                                        nat_lt(a747, a748)
                                    });
                                    if f187(v21, v20) {
                                        Ok(Some((v14, {
                                            let a750 = {
                                                let (_, h749) = v19.clone();
                                                h749
                                            };
                                            let a751 = 0u64;
                                            if nat_eq(a750, a751) {
                                                Adt2::C3(v21, v20)
                                            } else {
                                                Adt2::C4(v21, v20, {
                                                    let (_, h752) = v19;
                                                    h752
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

pub fn f115(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h753, _) = v2.clone();
        h753
    };
    let v4: u64 = {
        let (h754, _) = {
            let (_, h755) = v2.clone();
            h755
        };
        h754
    };
    let v5: List<Bytes> = {
        let (_, h756) = {
            let (_, h757) = v2;
            h757
        };
        h756
    };
    let m758 = f188(v0.clone(), v3);
    match m758 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a759 = v6;
            let a760 = v1;
            if equal(a759, a760) {
                Ok(Some(({
                    let a761 = v3;
                    let a762 = 1u64;
                    nat_add(a761, a762)?
                }, ({
                    let a763 = v3;
                    let a764 = 1u64;
                    nat_add(a763, a764)?
                }, List::cons(f46(v0, v4, {
                    let a765 = v3;
                    let a766 = v4;
                    nat_sub(a765, a766)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a767 = v3;
                    let a768 = 1u64;
                    nat_add(a767, a768)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f116(v0: Fn2, p772: List<Bytes>, p773: List<Bytes>) -> List<Bytes> {
    let m771 = (p772, p773);
    {
        let mut m771 = m771;
        loop {
            let step = {
                let (v1, v2) = m771;
                let m769 = v2;
                match m769.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c770: Fn2 = v0.clone();
                            c770.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m771 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f117(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f118(v0: Adt2) -> List<Adt2> {
    let m774 = v0;
    match m774 {
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
        Adt2::C7(r775) => {
            let v3 = (*r775).clone();
            f118(v3)
        }
        Adt2::C8(_) => {
            List::<Adt2>::nil()
        }
        Adt2::C9 => {
            List::<Adt2>::nil()
        }
        Adt2::C10(r776, r777) => {
            let v1 = (*r776).clone();
            let v2 = (*r777).clone();
            List::cons(v1, f118(v2))
        }
        Adt2::C11(_, _, _) => {
            List::<Adt2>::nil()
        }
    }
}

pub fn f119(v0: Adt20) -> Bytes {
    let a798 = f189(match v0.clone() {
        Adt20::C0(h778, _, _, _, _, _, _, _) => {
            h778
        }
    });
    let a799 = {
        let a796 = f189(match v0.clone() {
            Adt20::C0(_, h779, _, _, _, _, _, _) => {
                h779
            }
        });
        let a797 = {
            let a794 = f189(match v0.clone() {
                Adt20::C0(_, _, h780, _, _, _, _, _) => {
                    h780
                }
            });
            let a795 = {
                let a792 = f189(match v0.clone() {
                    Adt20::C0(_, _, _, h781, _, _, _, _) => {
                        h781
                    }
                });
                let a793 = {
                    let a790 = f189(match v0.clone() {
                        Adt20::C0(_, _, _, _, h782, _, _, _) => {
                            h782
                        }
                    });
                    let a791 = {
                        let a788 = f189(match v0.clone() {
                            Adt20::C0(_, _, _, _, _, h783, _, _) => {
                                h783
                            }
                        });
                        let a789 = {
                            let a786 = f189(match v0.clone() {
                                Adt20::C0(_, _, _, _, _, _, h784, _) => {
                                    h784
                                }
                            });
                            let a787 = f189(match v0 {
                                Adt20::C0(_, _, _, _, _, _, _, h785) => {
                                    h785
                                }
                            });
                            append_bytes(a786, a787)
                        };
                        append_bytes(a788, a789)
                    };
                    append_bytes(a790, a791)
                };
                append_bytes(a792, a793)
            };
            append_bytes(a794, a795)
        };
        append_bytes(a796, a797)
    };
    append_bytes(a798, a799)
}

pub fn f120(v0: Bytes) -> R<Adt20> {
    let v1: u64 = {
        let a800 = v0.clone();
        length_bytes(a800)
    };
    let v2: u64 = {
        let a806 = {
            let a803 = {
                let a801 = v1;
                let a802 = 72u64;
                nat_add(a801, a802)?
            };
            let a804 = 64u64;
            let a805 = 0u64;
            nat_quot(a803, a804, a805)
        };
        let a807 = 64u64;
        nat_mul(a806, a807)?
    };
    let (_, h808) = f190(Fn3::F191(v0, v1, v2), {
        let a809 = v2;
        let a810 = 64u64;
        let a811 = 0u64;
        nat_quot(a809, a810, a811)
    }, (0u64, f192()))?;
    Ok(h808)
}

pub fn f121(v0: Adt1, v1: Bytes) -> R<Option<Adt2>> {
    let m812 = f193(f194(v0.clone(), v1.clone()), v1.clone());
    match m812 {
        None => {
            f43(f90(v0, v1))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f122(v0: Bytes, p815: List<Bytes>) -> bool {
    let m814 = p815;
    {
        let mut m814 = m814;
        loop {
            let step = {
                let v1 = m814;
                let m813 = v1;
                match m813.uncons() {
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
                Ok(next) => { m814 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f123(v0: Adt2, v1: Bytes) -> Bytes {
    let m816 = f74(f73(v0, v1));
    match m816 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f124(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m817 = v0;
    match m817.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f124(v3, v1))
        }
    }
}

pub fn f125(v0: Bytes) -> R<Bytes> {
    f195(v0.clone(), 0u64, {
        let a818 = v0;
        length_bytes(a818)
    }, true, Bytes::lit(&[]))
}

pub fn f126(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m819 = v0;
    match m819.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f126(v3, v1))
        }
    }
}

pub fn f127(v0: Adt2) -> R<Bytes> {
    let a820 = f196(v0)?;
    let a821 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a820, a821))
}

pub fn f128(v0: List<Adt2>) -> Adt2 {
    let m822 = v0;
    match m822.uncons() {
        None => {
            Adt2::C9
        }
        Some((v1, v2)) => {
            Adt2::C10(std::rc::Rc::new(v1), std::rc::Rc::new(f128(v2)))
        }
    }
}

pub fn f129(v0: List<Adt2>, v1: Bytes, v2: Bytes, v3: Adt2) -> List<Adt2> {
    let m823 = v0;
    match m823.uncons() {
        None => {
            List::cons(v3, List::<Adt2>::nil())
        }
        Some((v4, v5)) => {
            if f32(f123(v4.clone(), v1.clone()), v2.clone()) {
                List::cons(v3, f197(v5, v1, v2))
            } else {
                List::cons(v4, f129(v5, v1, v2, v3))
            }
        }
    }
}

pub fn f130(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f156(f198(f199(v0)?, v1)?, f8(Str::lit("\u{a}"))))
}

pub fn f131(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f127(Adt2::C7(std::rc::Rc::new(f86(f200(f124(v1, f75(f51(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f132(p834: u64, p835: Bytes) -> R<Bytes> {
    let m833 = (p834, p835);
    {
        let mut m833 = m833;
        loop {
            let step = {
                let (v0, v1) = m833;
                let m824 = v0;
                if m824 == 0 {
                    Err::<(u64, Bytes), R<Bytes>>(Ok(v1))
                } else {
                    let v2 = m824 - 1;
                    if f53(v1.clone(), f8(Str::lit("-- "))) {
                        let m825 = f201(v1.clone(), f8(Str::lit("\u{a}")))?;
                        match m825 {
                            None => {
                                Err::<(u64, Bytes), R<Bytes>>(Ok(v1))
                            }
                            Some(v3) => {
                                Ok::<(u64, Bytes), R<Bytes>>((v2, f46(v1.clone(), {
                                    let a826 = v3;
                                    let a827 = 1u64;
                                    nat_add(a826, a827)?
                                }, {
                                    let a831 = {
                                        let a828 = v1;
                                        length_bytes(a828)
                                    };
                                    let a832 = {
                                        let a829 = v3;
                                        let a830 = 1u64;
                                        nat_add(a829, a830)?
                                    };
                                    nat_sub(a831, a832)
                                })))
                            }
                        }
                    } else {
                        Err::<(u64, Bytes), R<Bytes>>(Ok(v1))
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

pub fn f133(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m840 = {
        let a837 = v0;
        let a838 = v2;
        let a839 = {
            let a836 = v1.clone();
            length_bytes(a836)
        };
        slice_bytes(a837, a838, a839)
    };
    match m840 {
        None => {
            false
        }
        Some(v3) => {
            let a841 = v3;
            let a842 = v1;
            equal(a841, a842)
        }
    }
}

pub fn f134(v0: Bytes) -> R<Bytes> {
    let (_, h843) = {
        let (h844, _) = f202(Fn4::F203(v0.clone()), {
            let a846 = {
                let a845 = v0;
                length_bytes(a845)
            };
            let a847 = 1u64;
            nat_add(a846, a847)?
        }, (0u64, Bytes::lit(&[])))?;
        h844
    };
    Ok(h843)
}

pub fn f135(v0: Bytes) -> R<u64> {
    let (h848, _) = f204(Fn5::F205(v0.clone()), {
        let a850 = {
            let a849 = v0;
            length_bytes(a849)
        };
        let a851 = 1u64;
        nat_add(a850, a851)?
    }, 0u64)?;
    Ok(h848)
}

pub fn f136(v0: Bytes, v1: u64) -> R<u64> {
    let (h852, _) = f204(Fn5::F206(v0.clone()), {
        let a854 = {
            let a853 = v0;
            length_bytes(a853)
        };
        let a855 = 1u64;
        nat_add(a854, a855)?
    }, v1)?;
    Ok(h852)
}

pub fn f137(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a856 = v0;
        utf8_encode(a856)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f91(v1)?, List::<Bytes>::nil())))))
}

pub fn f138(v0: Bytes, p865: u64, p866: u64) -> R<Result<u64, Bytes>> {
    let m864 = (p865, p866);
    {
        let mut m864 = m864;
        loop {
            let step = {
                let (v1, v2) = m864;
                let m857 = v2;
                if m857 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Ok::<u64, Bytes>(v1)))
                } else {
                    let v3 = m857 - 1;
                    let a859 = v1;
                    let a860 = {
                        let a858 = v0.clone();
                        length_bytes(a858)
                    };
                    if if nat_lt(a859, a860) {
                        if f162(v0.clone(), v1, Str::lit("...")) {
                            f207(v0.clone(), v1)?
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        let m863 = f208(v0.clone(), {
                            let a861 = v1;
                            let a862 = 3u64;
                            nat_add(a861, a862)?
                        })?;
                        match m863 {
                            Ok(v5) => {
                                Ok::<(u64, u64), R<Result<u64, Bytes>>>((f136(v0.clone(), v5)?, v3))
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
                Ok(next) => { m864 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f139(v0: Bytes, v1: u64, v2: bool, v3: u64) -> R<Result<List<Adt2>, Bytes>> {
    let m867 = v3;
    if m867 == 0 {
        Ok(Err::<List<Adt2>, Bytes>(f137(Str::lit("the stream holds more documents than it is long"), v1)?))
    } else {
        let v4 = m867 - 1;
        let v5: u64 = {
            let a868 = v0.clone();
            length_bytes(a868)
        };
        let a869 = f209(v0.clone(), v1)?;
        let a870 = 37u64;
        if nat_eq(a869, a870) {
            Ok(Err::<List<Adt2>, Bytes>(f137(Str::lit("a directive is not read (YAML 1.2.2 [82])"), v1)?))
        } else {
            let m877 = if if f162(v0.clone(), v1, Str::lit("---")) {
                f207(v0.clone(), v1)?
            } else {
                false
            } {
                f210(v0.clone(), {
                    let a871 = v1;
                    let a872 = 3u64;
                    nat_add(a871, a872)?
                }, 0u64, {
                    let a873 = v5;
                    let a874 = 2u64;
                    nat_add(a873, a874)?
                }, 1u64, false, 0u64, List::<Bytes>::nil(), Adt21::C0(0u64))?
            } else {
                if v2 {
                    f210(v0.clone(), v1, 0u64, {
                        let a875 = v5;
                        let a876 = 2u64;
                        nat_add(a875, a876)?
                    }, 0u64, false, 0u64, List::<Bytes>::nil(), Adt21::C0(0u64))?
                } else {
                    Adt22::C1(f137(Str::lit("only comments, a document end marker or a document start marker may follow a document (YAML 1.2.2 [207], [211])"), v1)?)
                }
            };
            match m877 {
                Adt22::C0(v7, v8) => {
                    let v9: u64 = f136(v0.clone(), v8)?;
                    let m880 = f138(v0.clone(), v9, {
                        let a878 = v5;
                        let a879 = 1u64;
                        nat_add(a878, a879)?
                    })?;
                    match m880 {
                        Ok(v11) => {
                            let a881 = v5;
                            let a882 = v11;
                            if nat_le(a881, a882) {
                                Ok(Ok::<List<Adt2>, Bytes>(List::cons(v7, List::<Adt2>::nil())))
                            } else {
                                let m885 = f139(v0, v11, f211({
                                    let a883 = v11;
                                    let a884 = v9;
                                    nat_eq(a883, a884)
                                }), v4)?;
                                match m885 {
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

pub fn f140(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a886 = v2;
    let a887 = 0u64;
    if nat_eq(a886, a887) {
        Ok(f212(v0, 0u64))
    } else {
        let (_, h888) = {
            let (h889, _) = f202(Fn4::F213(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h889
        };
        Ok(h888)
    }
}

pub fn f141() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f142(v0: Bytes) -> R<Result<List<Adt13>, Bytes>> {
    let m890 = f214(v0)?;
    match m890 {
        Ok(v2) => {
            f215({
                let a892 = {
                    let a891 = v2.clone();
                    length_list(a891)
                };
                let a893 = 1u64;
                nat_add(a892, a893)?
            }, v2, List::<Adt13>::nil())
        }
        Err(v1) => {
            Ok(Err::<List<Adt13>, Bytes>(v1))
        }
    }
}

pub fn f143(v0: List<Bytes>) -> R<Result<List<Adt13>, Bytes>> {
    let m894 = v0;
    match m894.uncons() {
        None => {
            Ok(Ok::<List<Adt13>, Bytes>(List::<Adt13>::nil()))
        }
        Some((v1, v2)) => {
            let m895 = f142(v1)?;
            match m895 {
                Ok(v4) => {
                    let m896 = f143(v2)?;
                    match m896 {
                        Ok(v6) => {
                            Ok(Ok::<List<Adt13>, Bytes>(f144(v4, v6)))
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

pub fn f144(v0: List<Adt13>, v1: List<Adt13>) -> List<Adt13> {
    let m897 = v0;
    match m897.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f144(v3, v1))
        }
    }
}

pub fn f145(p902: u64, p903: List<Adt16>) -> R<List<Adt16>> {
    let m901 = (p902, p903);
    {
        let mut m901 = m901;
        loop {
            let step = {
                let (v0, v1) = m901;
                let m898 = v0;
                if m898 == 0 {
                    Err::<(u64, List<Adt16>), R<List<Adt16>>>(Ok(v1))
                } else {
                    let v2 = m898 - 1;
                    let v3: List<Adt16> = f216(v1.clone(), v1.clone());
                    let a899 = f217(v3.clone())?;
                    let a900 = f217(v1.clone())?;
                    if nat_eq(a899, a900) {
                        Err::<(u64, List<Adt16>), R<List<Adt16>>>(Ok(v1))
                    } else {
                        Ok::<(u64, List<Adt16>), R<List<Adt16>>>((v2, v3))
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

pub fn f146() -> Bytes {
    f8(Str::lit("Binary32"))
}

pub fn f147(v0: bool) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(if v0 {
        f148()
    } else {
        f146()
    }, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: bits, type: Nat}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))
}

pub fn f148() -> Bytes {
    f8(Str::lit("Binary64"))
}

pub fn f149(v0: List<Adt13>, v1: List<Adt13>) -> R<List<Adt16>> {
    let m904 = v0;
    match m904.uncons() {
        None => {
            Ok(List::<Adt16>::nil())
        }
        Some((v2, v3)) => {
            if f218(f219(v2.clone())) {
                let v6: Adt16 = {
                    let m905 = f220(v1.clone(), v2.clone())?;
                    match m905 {
                        Ok(v5) => {
                            Adt16::C0(f221(v2), Bytes::lit(&[]), {
                                let (h906, _) = v5;
                                h906
                            }, {
                                let (_, h907) = v5;
                                h907
                            })
                        }
                        Err(v4) => {
                            Adt16::C0(f221(v2), v4, Bytes::lit(&[]), List::<Bytes>::nil())
                        }
                    }
                };
                Ok(List::cons(v6, f149(v3, v1)?))
            } else {
                f149(v3, v1)
            }
        }
    }
}

pub fn f150(p912: u64, p913: List<Bytes>, p914: List<Bytes>, v3: List<Adt16>) -> List<Bytes> {
    let m911 = (p912, (p913, p914));
    {
        let mut m911 = m911;
        loop {
            let step = {
                let (v0, (v1, v2)) = m911;
                let m908 = v0;
                if m908 == 0 {
                    Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v2)
                } else {
                    let v4 = m908 - 1;
                    let m909 = v1;
                    match m909.uncons() {
                        None => {
                            Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v2)
                        }
                        Some((v5, v6)) => {
                            if f122(v5.clone(), v2.clone()) {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v2)))
                            } else {
                                let m910 = f222(v3.clone(), v5.clone());
                                match m910 {
                                    None => {
                                        Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v2)))
                                    }
                                    Some(v7) => {
                                        if f32(f223(v7.clone()), Bytes::lit(&[])) {
                                            Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (f224(f225(v7), v6), List::cons(v5, v2))))
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
                Ok(next) => { m911 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f151(v0: List<Adt13>) -> List<Bytes> {
    let m915 = v0;
    match m915.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let v3: u64 = f219(v1.clone());
            let a916 = v3;
            let a917 = 0u64;
            if if nat_eq(a916, a917) {
                true
            } else {
                let a918 = v3;
                let a919 = 2u64;
                if nat_eq(a918, a919) {
                    true
                } else {
                    let a920 = v3;
                    let a921 = 3u64;
                    if nat_eq(a920, a921) {
                        true
                    } else {
                        let a922 = v3;
                        let a923 = 4u64;
                        if nat_eq(a922, a923) {
                            true
                        } else {
                            let a924 = v3;
                            let a925 = 5u64;
                            if nat_eq(a924, a925) {
                                true
                            } else {
                                let a926 = v3;
                                let a927 = 7u64;
                                if nat_eq(a926, a927) {
                                    true
                                } else {
                                    let a928 = v3;
                                    let a929 = 8u64;
                                    nat_eq(a928, a929)
                                }
                            }
                        }
                    }
                }
            } {
                List::cons(f221(v1), f151(v2))
            } else {
                f151(v2)
            }
        }
    }
}

pub fn f152(v0: List<Adt16>, v1: List<Bytes>) -> List<(Bytes, (List<Bytes>, Bytes))> {
    let m930 = v0;
    match m930.uncons() {
        None => {
            List::<(Bytes, (List<Bytes>, Bytes))>::nil()
        }
        Some((v2, v3)) => {
            if if f122(f226(v2.clone()), v1.clone()) {
                f32(f223(v2.clone()), Bytes::lit(&[]))
            } else {
                false
            } {
                List::cons((f226(v2.clone()), (f225(v2.clone()), f227(v2))), f152(v3, v1))
            } else {
                f152(v3, v1)
            }
        }
    }
}

pub fn f153(p939: u64, p940: List<(Bytes, (List<Bytes>, Bytes))>, p941: List<Bytes>, p942: List<Bytes>) -> (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>) {
    let m938 = (p939, (p940, (p941, p942)));
    {
        let mut m938 = m938;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m938;
                let m931 = v0;
                if m931 == 0 {
                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v3, v1))
                } else {
                    let v4 = m931 - 1;
                    let m932 = f228(v1.clone(), v2.clone());
                    match m932 {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v3, v1))
                        }
                        Some(v5) => {
                            Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v4, ({
                                let (_, h933) = v5.clone();
                                h933
                            }, (List::cons({
                                let (h934, _) = {
                                    let (h935, _) = v5.clone();
                                    h935
                                };
                                h934
                            }, v2), List::cons({
                                let (_, h936) = {
                                    let (h937, _) = v5;
                                    h937
                                };
                                h936
                            }, v3)))))
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

pub fn f154(v0: List<Adt13>, v1: List<Adt16>, v2: List<Adt13>) -> List<Bytes> {
    let m943 = v0;
    match m943.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            let v5: u64 = f219(v3.clone());
            let a944 = v5;
            let a945 = 5u64;
            if if nat_eq(a944, a945) {
                true
            } else {
                let a946 = v5;
                let a947 = 7u64;
                nat_eq(a946, a947)
            } {
                let m950 = f229(v2.clone(), {
                    let a948 = v5;
                    let a949 = 5u64;
                    if nat_eq(a948, a949) {
                        0u64
                    } else {
                        2u64
                    }
                }, f221(v3.clone()));
                match m950 {
                    None => {
                        List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f230(v5), List::cons(f221(v3.clone()), List::cons(f8(Str::lit(": no input defines ")), List::cons(f221(v3), List::cons(f8(Str::lit(", so its members join no definition (Web IDL 2.2 and 2.7)\u{a}")), List::<Bytes>::nil()))))))), f154(v4, v1, v2))
                    }
                    Some(_) => {
                        f154(v4, v1, v2)
                    }
                }
            } else {
                let a951 = v5;
                let a952 = 8u64;
                if nat_eq(a951, a952) {
                    let m953 = f229(v2.clone(), 0u64, f221(v3.clone()));
                    match m953 {
                        None => {
                            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f221(v3.clone()), List::cons(f8(Str::lit(" includes ")), List::cons(f231(v3.clone()), List::cons(f8(Str::lit(": no input defines the interface ")), List::cons(f221(v3), List::cons(f8(Str::lit(" (Web IDL 2.3)\u{a}")), List::<Bytes>::nil())))))))), f154(v4, v1, v2))
                        }
                        Some(_) => {
                            f154(v4, v1, v2)
                        }
                    }
                } else {
                    if f218(v5) {
                        let m954 = f222(v1.clone(), f221(v3.clone()));
                        match m954 {
                            None => {
                                f154(v4, v1, v2)
                            }
                            Some(v8) => {
                                if f32(f223(v8.clone()), Bytes::lit(&[])) {
                                    f154(v4, v1, v2)
                                } else {
                                    List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f221(v3), List::cons(f8(Str::lit(": ")), List::cons(f223(v8), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), f154(v4, v1, v2))
                                }
                            }
                        }
                    } else {
                        f154(v4, v1, v2)
                    }
                }
            }
        }
    }
}

pub fn f155(v0: List<(Bytes, (List<Bytes>, Bytes))>) -> List<Bytes> {
    let m955 = v0;
    match m955.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons({
                let (h956, _) = v1;
                h956
            }, List::cons(f8(Str::lit(": it refers back to itself through its members, and the translator emits non-recursive declarations only\u{a}")), List::<Bytes>::nil())))), f155(v2))
        }
    }
}

pub fn f156(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m957 = v0;
    match m957.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m958 = v3.clone();
            match m958.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a961 = v2;
                    let a962 = {
                        let a959 = v1.clone();
                        let a960 = f156(v3, v1);
                        append_bytes(a959, a960)
                    };
                    append_bytes(a961, a962)
                }
            }
        }
    }
}

pub fn f157(p965: List<Bytes>, p966: List<Bytes>) -> List<Bytes> {
    let m964 = (p965, p966);
    {
        let mut m964 = m964;
        loop {
            let step = {
                let (v0, v1) = m964;
                let m963 = v0;
                match m963.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m964 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f158(v0: Bytes) -> R<u64> {
    let (h967, _) = f204(Fn5::F232(v0.clone()), {
        let a969 = {
            let a968 = v0;
            length_bytes(a968)
        };
        let a970 = 1u64;
        nat_add(a969, a970)?
    }, 0u64)?;
    Ok(h967)
}

pub fn f159(v0: Bytes) -> Result<Adt17, Bytes> {
    Err::<Adt17, Bytes>(v0)
}

pub fn f160(v0: Bytes) -> R<Result<u64, Bytes>> {
    if if f162(v0.clone(), 0u64, Str::lit("<?xml")) {
        f233(f163(v0.clone(), 5u64)?)?
    } else {
        false
    } {
        let m972 = f234(v0.clone(), {
            let a971 = Str::lit("?>");
            utf8_encode(a971)
        }, 5u64)?;
        match m972 {
            None => {
                Ok(f235(f137(Str::lit("the XML declaration is not closed (XML 1.0 [23])"), 0u64)?))
            }
            Some(v1) => {
                let m974 = f234(f46(v0.clone(), 0u64, v1), {
                    let a973 = Str::lit("encoding");
                    utf8_encode(a973)
                }, 5u64)?;
                match m974 {
                    None => {
                        Ok(Ok::<u64, Bytes>({
                            let a975 = v1;
                            let a976 = 2u64;
                            nat_add(a975, a976)?
                        }))
                    }
                    Some(v2) => {
                        let v3: u64 = f236(v0.clone(), {
                            let a977 = v2;
                            let a978 = 8u64;
                            nat_add(a977, a978)?
                        })?;
                        let v4: u64 = f236(v0.clone(), {
                            let a979 = v3;
                            let a980 = 1u64;
                            nat_add(a979, a980)?
                        })?;
                        let _: u64 = f163(v0.clone(), v4)?;
                        let v7: u64 = {
                            let m983 = f234(v0.clone(), f46(v0.clone(), v4, 1u64), {
                                let a981 = v4;
                                let a982 = 1u64;
                                nat_add(a981, a982)?
                            })?;
                            match m983 {
                                None => {
                                    v1
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        if f237(f46(v0, {
                            let a984 = v4;
                            let a985 = 1u64;
                            nat_add(a984, a985)?
                        }, {
                            let a988 = v7;
                            let a989 = {
                                let a986 = v4;
                                let a987 = 1u64;
                                nat_add(a986, a987)?
                            };
                            nat_sub(a988, a989)
                        })) {
                            Ok(Ok::<u64, Bytes>({
                                let a990 = v1;
                                let a991 = 2u64;
                                nat_add(a990, a991)?
                            }))
                        } else {
                            Ok(f235(f137(Str::lit("only the UTF-8 encoding is read (XML 1.0 section 4.3.3)"), 0u64)?))
                        }
                    }
                }
            }
        }
    } else {
        Ok(Ok::<u64, Bytes>(0u64))
    }
}

pub fn f161(v0: Bytes, p996: u64, p997: u64) -> R<Result<u64, Bytes>> {
    let m995 = (p996, p997);
    {
        let mut m995 = m995;
        loop {
            let step = {
                let (v1, v2) = m995;
                let m992 = v2;
                if m992 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f235(f137(Str::lit("the prolog is longer than the input"), v1)?)))
                } else {
                    let v3 = m992 - 1;
                    let v4: u64 = f236(v0.clone(), v1)?;
                    if f162(v0.clone(), v4, Str::lit("<!--")) {
                        let m993 = f238(v0.clone(), v4)?;
                        match m993 {
                            Ok(v6) => {
                                Ok::<(u64, u64), R<Result<u64, Bytes>>>((v6, v3))
                            }
                            Err(v5) => {
                                Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f235(v5)))
                            }
                        }
                    } else {
                        if f162(v0.clone(), v4, Str::lit("<?")) {
                            let m994 = f239(v0.clone(), v4)?;
                            match m994 {
                                Ok(v8) => {
                                    Ok::<(u64, u64), R<Result<u64, Bytes>>>((v8, v3))
                                }
                                Err(v7) => {
                                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f235(v7)))
                                }
                            }
                        } else {
                            Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Ok::<u64, Bytes>(v4)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m995 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f162(v0: Bytes, v1: u64, v2: Str) -> bool {
    f133(v0, {
        let a998 = v2;
        utf8_encode(a998)
    }, v1)
}

pub fn f163(v0: Bytes, v1: u64) -> R<u64> {
    f111(v0, v1)
}

pub fn f164(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Result<(Adt17, u64), Bytes>> {
    let m999 = v2;
    if m999 == 0 {
        Ok(f240(f137(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v4 = m999 - 1;
        let a1000 = v3;
        let a1001 = 0u64;
        if nat_eq(a1000, a1001) {
            let v5: u64 = f241(v0.clone(), {
                let a1002 = v1;
                let a1003 = 1u64;
                nat_add(a1002, a1003)?
            })?;
            let a1006 = v5;
            let a1007 = {
                let a1004 = v1;
                let a1005 = 1u64;
                nat_add(a1004, a1005)?
            };
            if nat_eq(a1006, a1007) {
                Ok(f240(f137(Str::lit("an element must start with a Name (XML 1.0 [40])"), v1)?))
            } else {
                let v6: Bytes = f46(v0.clone(), {
                    let a1008 = v1;
                    let a1009 = 1u64;
                    nat_add(a1008, a1009)?
                }, {
                    let a1012 = v5;
                    let a1013 = {
                        let a1010 = v1;
                        let a1011 = 1u64;
                        nat_add(a1010, a1011)?
                    };
                    nat_sub(a1012, a1013)
                });
                let m1014 = f242(v0.clone(), v5, v4, List::<Bytes>::nil())?;
                match m1014 {
                    Ok(v8) => {
                        if f162(v0.clone(), {
                            let (_, h1015) = v8.clone();
                            h1015
                        }, Str::lit("/>")) {
                            Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C0(v6, std::rc::Rc::new({
                                let (h1016, _) = v8;
                                h1016
                            }), std::rc::Rc::new(Adt17::C4)), {
                                let a1018 = {
                                    let (_, h1017) = v8;
                                    h1017
                                };
                                let a1019 = 2u64;
                                nat_add(a1018, a1019)?
                            })))
                        } else {
                            let m1023 = f164(v0.clone(), {
                                let a1021 = {
                                    let (_, h1020) = v8;
                                    h1020
                                };
                                let a1022 = 1u64;
                                nat_add(a1021, a1022)?
                            }, v4, 1u64)?;
                            match m1023 {
                                Ok(v10) => {
                                    let v11: u64 = {
                                        let a1025 = {
                                            let (_, h1024) = v10.clone();
                                            h1024
                                        };
                                        let a1026 = 2u64;
                                        nat_add(a1025, a1026)?
                                    };
                                    let v12: u64 = f241(v0.clone(), v11)?;
                                    if f32(f46(v0.clone(), v11, {
                                        let a1027 = v12;
                                        let a1028 = v11;
                                        nat_sub(a1027, a1028)
                                    }), v6.clone()) {
                                        let v13: u64 = f236(v0.clone(), v12)?;
                                        let a1029 = f163(v0, v13)?;
                                        let a1030 = 62u64;
                                        if nat_eq(a1029, a1030) {
                                            Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C0(v6, std::rc::Rc::new({
                                                let (h1031, _) = v8;
                                                h1031
                                            }), std::rc::Rc::new({
                                                let (h1032, _) = v10;
                                                h1032
                                            })), {
                                                let a1033 = v13;
                                                let a1034 = 1u64;
                                                nat_add(a1033, a1034)?
                                            })))
                                        } else {
                                            Ok(f240(f137(Str::lit("an end tag must close with > (XML 1.0 [42])"), v13)?))
                                        }
                                    } else {
                                        Ok(f240(f137(Str::lit("an end tag must name the element its start tag opened (XML 1.0 WFC: Element Type Match)"), {
                                            let (_, h1035) = v10;
                                            h1035
                                        })?))
                                    }
                                }
                                Err(v9) => {
                                    Ok(f240(v9))
                                }
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(f240(v7))
                    }
                }
            }
        } else {
            let a1036 = f163(v0.clone(), v1)?;
            let a1037 = 256u64;
            if nat_eq(a1036, a1037) {
                Ok(f240(f137(Str::lit("the input ends before an element is closed (XML 1.0 [39])"), v1)?))
            } else {
                if f162(v0.clone(), v1, Str::lit("</")) {
                    Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C4, v1)))
                } else {
                    if f162(v0.clone(), v1, Str::lit("<!--")) {
                        let m1038 = f238(v0.clone(), v1)?;
                        match m1038 {
                            Ok(v15) => {
                                f164(v0, v15, v4, 1u64)
                            }
                            Err(v14) => {
                                Ok(f240(v14))
                            }
                        }
                    } else {
                        if f162(v0.clone(), v1, Str::lit("<![CDATA[")) {
                            let m1042 = f234(v0.clone(), {
                                let a1039 = Str::lit("]]>");
                                utf8_encode(a1039)
                            }, {
                                let a1040 = v1;
                                let a1041 = 9u64;
                                nat_add(a1040, a1041)?
                            })?;
                            match m1042 {
                                None => {
                                    Ok(f240(f137(Str::lit("a CDATA section is not closed (XML 1.0 [18])"), v1)?))
                                }
                                Some(v16) => {
                                    let m1045 = f164(v0.clone(), {
                                        let a1043 = v16;
                                        let a1044 = 3u64;
                                        nat_add(a1043, a1044)?
                                    }, v4, 1u64)?;
                                    match m1045 {
                                        Ok(v18) => {
                                            Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C3(std::rc::Rc::new(Adt17::C2(f46(v0, {
                                                let a1046 = v1;
                                                let a1047 = 9u64;
                                                nat_add(a1046, a1047)?
                                            }, {
                                                let a1050 = v16;
                                                let a1051 = {
                                                    let a1048 = v1;
                                                    let a1049 = 9u64;
                                                    nat_add(a1048, a1049)?
                                                };
                                                nat_sub(a1050, a1051)
                                            }))), std::rc::Rc::new({
                                                let (h1052, _) = v18;
                                                h1052
                                            })), {
                                                let (_, h1053) = v18;
                                                h1053
                                            })))
                                        }
                                        Err(v17) => {
                                            Ok(f240(v17))
                                        }
                                    }
                                }
                            }
                        } else {
                            if f162(v0.clone(), v1, Str::lit("<?")) {
                                let m1054 = f239(v0.clone(), v1)?;
                                match m1054 {
                                    Ok(v20) => {
                                        f164(v0, v20, v4, 1u64)
                                    }
                                    Err(v19) => {
                                        Ok(f240(v19))
                                    }
                                }
                            } else {
                                if f162(v0.clone(), v1, Str::lit("<!")) {
                                    Ok(f240(f137(Str::lit("a markup declaration is not allowed in content (XML 1.0 [43])"), v1)?))
                                } else {
                                    let a1055 = f163(v0.clone(), v1)?;
                                    let a1056 = 60u64;
                                    if nat_eq(a1055, a1056) {
                                        let m1057 = f164(v0.clone(), v1, v4, 0u64)?;
                                        match m1057 {
                                            Ok(v22) => {
                                                let m1059 = f164(v0, {
                                                    let (_, h1058) = v22;
                                                    h1058
                                                }, v4, 1u64)?;
                                                match m1059 {
                                                    Ok(v24) => {
                                                        Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C3(std::rc::Rc::new({
                                                            let (h1060, _) = v22;
                                                            h1060
                                                        }), std::rc::Rc::new({
                                                            let (h1061, _) = v24;
                                                            h1061
                                                        })), {
                                                            let (_, h1062) = v24;
                                                            h1062
                                                        })))
                                                    }
                                                    Err(v23) => {
                                                        Ok(f240(v23))
                                                    }
                                                }
                                            }
                                            Err(v21) => {
                                                Ok(f240(v21))
                                            }
                                        }
                                    } else {
                                        let m1063 = f243(v0.clone(), v1, v4, Bytes::lit(&[]), false, 0u64)?;
                                        match m1063 {
                                            Ok(v26) => {
                                                let m1065 = f164(v0, {
                                                    let (_, h1064) = v26;
                                                    h1064
                                                }, v4, 1u64)?;
                                                match m1065 {
                                                    Ok(v28) => {
                                                        Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C3(std::rc::Rc::new(Adt17::C2({
                                                            let (h1066, _) = v26;
                                                            h1066
                                                        })), std::rc::Rc::new({
                                                            let (h1067, _) = v28;
                                                            h1067
                                                        })), {
                                                            let (_, h1068) = v28;
                                                            h1068
                                                        })))
                                                    }
                                                    Err(v27) => {
                                                        Ok(f240(v27))
                                                    }
                                                }
                                            }
                                            Err(v25) => {
                                                Ok(f240(v25))
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

pub fn f165(v0: Bytes) -> Result<(u64, Bytes), Bytes> {
    Err::<(u64, Bytes), Bytes>(v0)
}

pub fn f166(v0: Adt17, v1: List<(Bytes, Bytes)>) -> Result<List<(Bytes, Bytes)>, Bytes> {
    let m1069 = v0;
    match m1069 {
        Adt17::C0(_, _, _) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt17::C1(v2, v3, r1074) => {
            let v4 = (*r1074).clone();
            let m1070 = f166(v4, v1);
            match m1070 {
                Ok(v6) => {
                    if f32(v2.clone(), f8(Str::lit("xmlns"))) {
                        if if f32(v3.clone(), f245()) {
                            true
                        } else {
                            f32(v3.clone(), f246())
                        } {
                            f244(f8(Str::lit("the default namespace must not be the xml or xmlns namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Bytes>(List::cons((Bytes::lit(&[]), v3), v6))
                        }
                    } else {
                        if f53(v2.clone(), f8(Str::lit("xmlns:"))) {
                            let v7: Bytes = f46(v2.clone(), 6u64, {
                                let a1072 = {
                                    let a1071 = v2;
                                    length_bytes(a1071)
                                };
                                let a1073 = 6u64;
                                nat_sub(a1072, a1073)
                            });
                            if f32(v7.clone(), f8(Str::lit("xmlns"))) {
                                f244(f8(Str::lit("the prefix xmlns must not be declared (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                            } else {
                                if if f32(v7.clone(), f8(Str::lit("xml"))) {
                                    !f32(v3.clone(), f245())
                                } else {
                                    false
                                } {
                                    f244(f8(Str::lit("the prefix xml may be bound only to its own namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                } else {
                                    if if if f32(v3.clone(), f245()) {
                                        true
                                    } else {
                                        f32(v3.clone(), f246())
                                    } {
                                        !f32(v7.clone(), f8(Str::lit("xml")))
                                    } else {
                                        false
                                    } {
                                        f244(f8(Str::lit("the xml and xmlns namespace names must not be bound to another prefix (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                    } else {
                                        if f32(v3.clone(), Bytes::lit(&[])) {
                                            f244(f8(Str::lit("a prefix must not be undeclared (Namespaces in XML 1.0, section 3, NSC: No Prefix Undeclaring)")))
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
                    f244(v5)
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

pub fn f167(v0: List<(Bytes, Bytes)>, v1: Bytes) -> R<Result<(Bytes, Bytes), Bytes>> {
    let v2: Bytes = f247(v1.clone())?;
    if f32(v2.clone(), Bytes::lit(&[])) {
        let m1075 = f248(v0, Bytes::lit(&[]));
        match m1075 {
            None => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((Bytes::lit(&[]), v1)))
            }
            Some(v3) => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((v3, v1)))
            }
        }
    } else {
        if f32(v2.clone(), f8(Str::lit("xml"))) {
            Ok(Ok::<(Bytes, Bytes), Bytes>((f245(), f249(v1)?)))
        } else {
            let m1076 = f248(v0, v2.clone());
            match m1076 {
                None => {
                    Ok(Err::<(Bytes, Bytes), Bytes>(f7(List::cons(f8(Str::lit("the prefix ")), List::cons(v2, List::cons(f8(Str::lit(" is not declared (Namespaces in XML 1.0, section 5, NSC: Prefix Declared)")), List::<Bytes>::nil()))))))
                }
                Some(v4) => {
                    Ok(Ok::<(Bytes, Bytes), Bytes>((v4, f249(v1)?)))
                }
            }
        }
    }
}

pub fn f168(v0: (Bytes, Bytes), v1: Str) -> bool {
    if f32({
        let (h1077, _) = v0;
        h1077
    }, f250()) {
        f32({
            let (_, h1078) = v0;
            h1078
        }, {
            let a1079 = v1;
            utf8_encode(a1079)
        })
    } else {
        false
    }
}

pub fn f169(p1083: Adt17, v1: Bytes) -> Option<Bytes> {
    let m1082 = p1083;
    {
        let mut m1082 = m1082;
        loop {
            let step = {
                let v0 = m1082;
                let m1080 = v0;
                match m1080 {
                    Adt17::C0(_, _, _) => {
                        Err::<Adt17, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt17::C1(v2, v3, r1081) => {
                        let v4 = (*r1081).clone();
                        if f32(v2, v1.clone()) {
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
                Ok(next) => { m1082 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f170(v0: Adt17, v1: List<(Bytes, Bytes)>, v2: Bytes) -> R<Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>> {
    let m1084 = v0;
    match m1084 {
        Adt17::C0(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt17::C2(_) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt17::C3(r1095, r1096) => {
            let v3 = (*r1095).clone();
            let v4 = (*r1096).clone();
            let m1085 = f170(v4, v1.clone(), v2.clone())?;
            match m1085 {
                Ok(v6) => {
                    let m1086 = v3;
                    match m1086 {
                        Adt17::C0(v7, r1093, r1094) => {
                            let v8 = (*r1093).clone();
                            let v9 = (*r1094).clone();
                            let m1087 = f166(v8.clone(), v1);
                            match m1087 {
                                Ok(v11) => {
                                    let m1088 = f167(v11.clone(), v7)?;
                                    match m1088 {
                                        Ok(v13) => {
                                            if f168(v13, Str::lit("simpleType")) {
                                                let m1089 = f169(v8, f8(Str::lit("name")));
                                                match m1089 {
                                                    None => {
                                                        Ok(f251(f8(Str::lit("a top-level simpleType must have a name (XSD 1.1 Part 2, section 4.1.2)"))))
                                                    }
                                                    Some(v14) => {
                                                        let m1090 = f252(v14.clone(), v9, v11, v2)?;
                                                        match m1090 {
                                                            Ok(v16) => {
                                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::cons((f125(v14)?, (match v16.clone() {
                                                                    Adt24::C0(_, h1091) => {
                                                                        h1091
                                                                    }
                                                                }, match v16 {
                                                                    Adt24::C0(h1092, _) => {
                                                                        h1092
                                                                    }
                                                                })), v6)))
                                                            }
                                                            Err(v15) => {
                                                                Ok(f251(v15))
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                                            }
                                        }
                                        Err(v12) => {
                                            Ok(f251(v12))
                                        }
                                    }
                                }
                                Err(v10) => {
                                    Ok(f251(v10))
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
                    Ok(f251(v5))
                }
            }
        }
        Adt17::C4 => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
    }
}

pub fn f171(v0: List<(Bytes, (List<Bytes>, Bytes))>) -> List<Bytes> {
    let m1097 = v0;
    match m1097.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h1098, _) = v1;
                h1098
            }, f171(v2))
        }
    }
}

pub fn f172(p1101: List<Bytes>) -> Option<Bytes> {
    let m1100 = p1101;
    {
        let mut m1100 = m1100;
        loop {
            let step = {
                let v0 = m1100;
                let m1099 = v0;
                match m1099.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f122(v1.clone(), v2.clone()) {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v1))
                        } else {
                            Ok::<List<Bytes>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1100 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f173(p1107: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<Bytes> {
    let m1106 = p1107;
    {
        let mut m1106 = m1106;
        loop {
            let step = {
                let v0 = m1106;
                let m1102 = v0;
                match m1102.uncons() {
                    None => {
                        Err::<List<(Bytes, (List<Bytes>, Bytes))>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        let m1105 = f253({
                            let (h1103, _) = {
                                let (_, h1104) = v2;
                                h1104
                            };
                            h1103
                        }, v1.clone());
                        match m1105 {
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
                Ok(next) => { m1106 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f174(p1118: u64, p1119: List<(Bytes, (List<Bytes>, Bytes))>, p1120: List<Bytes>, p1121: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m1117 = (p1118, (p1119, (p1120, p1121)));
    {
        let mut m1117 = m1117;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m1117;
                let m1108 = v0;
                if m1108 == 0 {
                    let m1109 = v1;
                    match m1109.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(f254(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                        }
                    }
                } else {
                    let v6 = m1108 - 1;
                    let m1110 = v1.clone();
                    match m1110.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            let m1111 = f228(v1, v2.clone());
                            match m1111 {
                                None => {
                                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(f254(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                                }
                                Some(v9) => {
                                    Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>((v6, ({
                                        let (_, h1112) = v9.clone();
                                        h1112
                                    }, (List::cons({
                                        let (h1113, _) = {
                                            let (h1114, _) = v9.clone();
                                            h1114
                                        };
                                        h1113
                                    }, v2), List::cons({
                                        let (_, h1115) = {
                                            let (h1116, _) = v9;
                                            h1116
                                        };
                                        h1115
                                    }, v3)))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1117 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f175(v0: Bytes) -> R<List<(Bytes, Bytes)>> {
    f255(f199(v0)?, None::<Bytes>)
}

pub fn f176(p1142: u64, v1: List<(Bytes, Option<Bytes>)>, v2: List<(Bytes, Bytes)>, p1143: List<(Bytes, (Bytes, Bytes))>, p1144: List<Adt19>, p1145: List<Bytes>) -> R<Adt18> {
    let m1141 = (p1142, (p1143, (p1144, p1145)));
    {
        let mut m1141 = m1141;
        loop {
            let step = {
                let (v0, (v3, (v4, v5))) = m1141;
                let m1122 = v0;
                if m1122 == 0 {
                    Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>(Ok(Adt18::C1(f8(Str::lit("the schema documents import and include more documents than the bound authorities hold")))))
                } else {
                    let v6 = m1122 - 1;
                    let m1123 = v3;
                    match m1123.uncons() {
                        None => {
                            let m1124 = v5.clone();
                            match m1124.uncons() {
                                None => {
                                    let m1125 = f256(f257(v4, List::<Adt19>::nil()))?;
                                    match m1125 {
                                        Ok(v8) => {
                                            Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>(Ok(Adt18::C2({
                                                let (h1126, _) = v8;
                                                h1126
                                            }, {
                                                let (_, h1127) = v8;
                                                h1127
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
                                let (h1128, _) = v11.clone();
                                h1128
                            };
                            if if f122(v13.clone(), f258(v4.clone())) {
                                true
                            } else {
                                f122(v13.clone(), v5.clone())
                            } {
                                Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>((v6, (v12, (v4, v5))))
                            } else {
                                let m1129 = f259(v1.clone(), v13.clone());
                                match m1129 {
                                    None => {
                                        Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>((v6, (v12, (v4, List::cons(v13, v5)))))
                                    }
                                    Some(v14) => {
                                        let m1130 = v14;
                                        match m1130 {
                                            None => {
                                                Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>(Ok(Adt18::C1(f260(f8(Str::lit("the schema document ")), v13, f8(Str::lit(" is not in the repository"))))))
                                            }
                                            Some(v15) => {
                                                let m1131 = f261(v13.clone(), v15, f262(v2.clone(), v13))?;
                                                match m1131 {
                                                    Ok(v17) => {
                                                        let m1136 = f263(v17.clone(), {
                                                            let (h1132, _) = {
                                                                let (_, h1133) = v11.clone();
                                                                h1133
                                                            };
                                                            h1132
                                                        }, {
                                                            let (_, h1134) = {
                                                                let (_, h1135) = v11;
                                                                h1135
                                                            };
                                                            h1134
                                                        });
                                                        match m1136 {
                                                            None => {
                                                                let m1139 = f264(match v17.clone() {
                                                                    Adt19::C0(_, _, _, _, h1137, _) => {
                                                                        h1137
                                                                    }
                                                                }, match v17.clone() {
                                                                    Adt19::C0(_, _, _, h1138, _, _) => {
                                                                        h1138
                                                                    }
                                                                })?;
                                                                match m1139 {
                                                                    Ok(v20) => {
                                                                        let m1140 = f265(v20, v17.clone(), v2.clone())?;
                                                                        match m1140 {
                                                                            Ok(v22) => {
                                                                                Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>((v6, (f266(v12, v22), (List::cons(v17, v4), v5))))
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
                Ok(next) => { m1141 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f177(v0: List<List<(Bytes, Option<Bytes>)>>) -> List<(Bytes, Option<Bytes>)> {
    let m1146 = v0;
    match m1146.uncons() {
        None => {
            List::<(Bytes, Option<Bytes>)>::nil()
        }
        Some((v1, v2)) => {
            f267(v1, f177(v2))
        }
    }
}

pub fn f178(v0: Adt2) -> R<Result<Bytes, Bytes>> {
    let m1147 = f268(v0.clone());
    match m1147 {
        None => {
            let m1148 = f269(v0.clone())?;
            match m1148 {
                None => {
                    Ok(Ok::<Bytes, Bytes>(f196(v0)?))
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

pub fn f179(v0: Bytes, v1: u64, v2: u64, v3: Bytes) -> Bytes {
    let a1151 = v3;
    let a1152 = f46(v0, v1, {
        let a1149 = v2;
        let a1150 = v1;
        nat_sub(a1149, a1150)
    });
    append_bytes(a1151, a1152)
}

pub fn f180(v0: Bytes, p1165: u64, p1166: u64) -> R<u64> {
    let m1164 = (p1165, p1166);
    {
        let mut m1164 = m1164;
        loop {
            let step = {
                let (v1, v2) = m1164;
                let m1153 = v2;
                if m1153 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m1153 - 1;
                    let v4: u64 = f111(v0.clone(), v1)?;
                    let a1154 = v4;
                    let a1155 = 34u64;
                    if nat_eq(a1154, a1155) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a1156 = v4;
                        let a1157 = 92u64;
                        if nat_eq(a1156, a1157) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a1158 = v4;
                            let a1159 = 32u64;
                            if nat_lt(a1158, a1159) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a1160 = v4;
                                let a1161 = 256u64;
                                if nat_eq(a1160, a1161) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a1162 = v1;
                                        let a1163 = 1u64;
                                        nat_add(a1162, a1163)?
                                    }, v3))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1164 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f181(v0: Bytes, p1192: u64, p1193: u64, p1194: Bytes) -> R<Option<(u64, Bytes)>> {
    let m1191 = (p1192, (p1193, p1194));
    {
        let mut m1191 = m1191;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1191;
                let m1167 = v2;
                if m1167 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                } else {
                    let v4 = m1167 - 1;
                    let v5: u64 = f180(v0.clone(), v1, {
                        let a1171 = {
                            let a1169 = {
                                let a1168 = v0.clone();
                                length_bytes(a1168)
                            };
                            let a1170 = v1;
                            nat_sub(a1169, a1170)
                        };
                        let a1172 = 1u64;
                        nat_add(a1171, a1172)?
                    })?;
                    let v6: Bytes = {
                        let a1175 = v3;
                        let a1176 = f46(v0.clone(), v1, {
                            let a1173 = v5;
                            let a1174 = v1;
                            nat_sub(a1173, a1174)
                        });
                        append_bytes(a1175, a1176)
                    };
                    let v7: u64 = f111(v0.clone(), v5)?;
                    let a1177 = v7;
                    let a1178 = 34u64;
                    if nat_eq(a1177, a1178) {
                        Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(Some(({
                            let a1179 = v5;
                            let a1180 = 1u64;
                            nat_add(a1179, a1180)?
                        }, v6))))
                    } else {
                        let a1181 = v7;
                        let a1182 = 92u64;
                        if nat_eq(a1181, a1182) {
                            let m1183 = f270(v0.clone(), v5, v6)?;
                            match m1183 {
                                None => {
                                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                                }
                                Some(v8) => {
                                    let a1186 = {
                                        let (_, h1184) = {
                                            let (_, h1185) = v8.clone();
                                            h1185
                                        };
                                        h1184
                                    };
                                    let a1187 = 0u64;
                                    if nat_eq(a1186, a1187) {
                                        Ok::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(({
                                            let (h1188, _) = v8;
                                            h1188
                                        }, (v4, {
                                            let (h1189, _) = {
                                                let (_, h1190) = v8;
                                                h1190
                                            };
                                            h1189
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
                Ok(next) => { m1191 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f182(v0: Bytes, v1: u64) -> R<u64> {
    let (h1195, _) = f204(Fn5::F271(v0.clone()), {
        let a1197 = {
            let a1196 = v0;
            length_bytes(a1196)
        };
        let a1198 = 1u64;
        nat_add(a1197, a1198)?
    }, v1)?;
    Ok(h1195)
}

pub fn f183(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f184(v0: Bytes) -> R<u64> {
    let (h1199, _) = f204(Fn5::F272(v0.clone()), {
        let a1201 = {
            let a1200 = v0;
            length_bytes(a1200)
        };
        let a1202 = 1u64;
        nat_add(a1201, a1202)?
    }, 0u64)?;
    Ok(h1199)
}

pub fn f185(v0: Bytes) -> R<Option<u64>> {
    if f273(v0.clone())? {
        Ok(Some({
            let (_, h1203) = {
                let (h1204, _) = f274(Fn6::F275(v0.clone()), {
                    let a1205 = v0;
                    length_bytes(a1205)
                }, (0u64, 0u64))?;
                h1204
            };
            h1203
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f186(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f46(v1.clone(), 0u64, {
        let a1207 = {
            let a1206 = v1.clone();
            length_bytes(a1206)
        };
        let a1208 = f276(v1)?;
        nat_sub(a1207, a1208)
    });
    let v5: Bytes = {
        let a1209 = v0;
        let a1210 = v4.clone();
        append_bytes(a1209, a1210)
    };
    let v6: u64 = f184(v5.clone())?;
    let v7: Bytes = f46(v5.clone(), v6, {
        let a1212 = {
            let a1211 = v5;
            length_bytes(a1211)
        };
        let a1213 = v6;
        nat_sub(a1212, a1213)
    });
    let v8: u64 = {
        let a1214 = v4;
        length_bytes(a1214)
    };
    let v9: u64 = {
        let a1215 = v7.clone();
        length_bytes(a1215)
    };
    let a1216 = v9;
    let a1217 = 0u64;
    if nat_eq(a1216, a1217) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f183(v2, {
            let a1218 = v3;
            let a1219 = v8;
            nat_lt(a1218, a1219)
        }) {
            let v10: u64 = if v2 {
                let a1220 = v3;
                let a1221 = v8;
                nat_add(a1220, a1221)?
            } else {
                let a1222 = v8;
                let a1223 = v3;
                nat_sub(a1222, a1223)
            };
            let v11: u64 = f276(v7.clone())?;
            let a1224 = v10;
            let a1225 = v11;
            if nat_le(a1224, a1225) {
                Ok(Some((f46(v7, 0u64, {
                    let a1226 = v9;
                    let a1227 = v10;
                    nat_sub(a1226, a1227)
                }), 0u64)))
            } else {
                let a1230 = {
                    let a1228 = v10;
                    let a1229 = v11;
                    nat_sub(a1228, a1229)
                };
                let a1231 = 19u64;
                if nat_le(a1230, a1231) {
                    Ok(Some((f46(v7, 0u64, {
                        let a1232 = v9;
                        let a1233 = v11;
                        nat_sub(a1232, a1233)
                    }), {
                        let a1234 = v10;
                        let a1235 = v11;
                        nat_sub(a1234, a1235)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a1236 = v3;
                let a1237 = v8;
                nat_sub(a1236, a1237)
            };
            let a1240 = {
                let a1238 = v9;
                let a1239 = v12;
                nat_add(a1238, a1239)?
            };
            let a1241 = 19u64;
            if nat_le(a1240, a1241) {
                Ok(Some(({
                    let a1242 = v7;
                    let a1243 = f277(v12);
                    append_bytes(a1242, a1243)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f187(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1244 = 0u64;
        let a1245 = v1;
        if nat_lt(a1244, a1245) {
            let a1246 = v1;
            let a1247 = 9223372036854775808u64;
            nat_le(a1246, a1247)
        } else {
            false
        }
    } else {
        let a1248 = v1;
        let a1249 = 9223372036854775807u64;
        nat_le(a1248, a1249)
    }
}

pub fn f188(v0: Bytes, v1: u64) -> Option<u8> {
    let a1250 = v0;
    let a1251 = v1;
    index_bytes(a1250, a1251)
}

pub fn f189(v0: u32) -> Bytes {
    let a1280 = f278({
        let a1252 = f279(v0, 28u32);
        let a1253 = 15u32;
        fixed_u32::bit_and(a1252, a1253)
    });
    let a1281 = {
        let a1278 = f278({
            let a1254 = f279(v0, 24u32);
            let a1255 = 15u32;
            fixed_u32::bit_and(a1254, a1255)
        });
        let a1279 = {
            let a1276 = f278({
                let a1256 = f279(v0, 20u32);
                let a1257 = 15u32;
                fixed_u32::bit_and(a1256, a1257)
            });
            let a1277 = {
                let a1274 = f278({
                    let a1258 = f279(v0, 16u32);
                    let a1259 = 15u32;
                    fixed_u32::bit_and(a1258, a1259)
                });
                let a1275 = {
                    let a1272 = f278({
                        let a1260 = f279(v0, 12u32);
                        let a1261 = 15u32;
                        fixed_u32::bit_and(a1260, a1261)
                    });
                    let a1273 = {
                        let a1270 = f278({
                            let a1262 = f279(v0, 8u32);
                            let a1263 = 15u32;
                            fixed_u32::bit_and(a1262, a1263)
                        });
                        let a1271 = {
                            let a1268 = f278({
                                let a1264 = f279(v0, 4u32);
                                let a1265 = 15u32;
                                fixed_u32::bit_and(a1264, a1265)
                            });
                            let a1269 = f278({
                                let a1266 = v0;
                                let a1267 = 15u32;
                                fixed_u32::bit_and(a1266, a1267)
                            });
                            append_bytes(a1268, a1269)
                        };
                        append_bytes(a1270, a1271)
                    };
                    append_bytes(a1272, a1273)
                };
                append_bytes(a1274, a1275)
            };
            append_bytes(a1276, a1277)
        };
        append_bytes(a1278, a1279)
    };
    append_bytes(a1280, a1281)
}

pub fn f190(v0: Fn3, p1285: u64, p1286: (u64, Adt20)) -> R<(u64, Adt20)> {
    let m1284 = (p1285, p1286);
    {
        let mut m1284 = m1284;
        loop {
            let step = {
                let (v1, v2) = m1284;
                let m1282 = v1;
                if m1282 == 0 {
                    Err::<(u64, (u64, Adt20)), R<(u64, Adt20)>>(Ok(v2))
                } else {
                    let v3 = m1282 - 1;
                    Ok::<(u64, (u64, Adt20)), R<(u64, Adt20)>>((v3, {
                        let c1283: Fn3 = v0.clone();
                        c1283.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1284 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f191(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt20)) -> R<(u64, Adt20)> {
    Ok(({
        let a1288 = {
            let (h1287, _) = v3.clone();
            h1287
        };
        let a1289 = 64u64;
        nat_add(a1288, a1289)?
    }, f280({
        let (_, h1290) = v3;
        h1290
    }, f281(v0, v1, v2, {
        let (h1291, _) = v3;
        h1291
    })?)?))
}

pub fn f192() -> Adt20 {
    Adt20::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f193(p1296: List<(Bytes, Option<Adt2>)>, v1: Bytes) -> Option<Option<Adt2>> {
    let m1295 = p1296;
    {
        let mut m1295 = m1295;
        loop {
            let step = {
                let v0 = m1295;
                let m1292 = v0;
                match m1292.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(None::<Option<Adt2>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h1293, _) = v2;
                            h1293
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(Some({
                                let (_, h1294) = v2;
                                h1294
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(v3)
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

pub fn f194(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Adt2>)> {
    let m1300 = {
        let a1298 = match v0 {
            Adt1::C0(_, _, _, h1297) => {
                h1297
            }
        };
        let a1299 = f40(v1);
        index_list(a1298, a1299)
    };
    match m1300 {
        None => {
            List::<(Bytes, Option<Adt2>)>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f195(v0: Bytes, p1321: u64, p1322: u64, p1323: bool, p1324: Bytes) -> R<Bytes> {
    let m1320 = (p1321, (p1322, (p1323, p1324)));
    {
        let mut m1320 = m1320;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1320;
                let m1301 = v2;
                if m1301 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1301 - 1;
                    let m1304 = {
                        let a1302 = v0.clone();
                        let a1303 = v1;
                        index_bytes(a1302, a1303)
                    };
                    match m1304 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f62(v6)?;
                            if f282(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1305 = v1;
                                    let a1306 = 1u64;
                                    nat_add(a1305, a1306)?
                                }, (v5, (true, v4))))
                            } else {
                                if f113(f283(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1307 = v1;
                                        let a1308 = 1u64;
                                        nat_add(a1307, a1308)?
                                    }, (v5, (false, {
                                        let a1312 = v4;
                                        let a1313 = f46({
                                            let a1309 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1309)
                                        }, {
                                            let a1310 = v7;
                                            let a1311 = 97u64;
                                            nat_sub(a1310, a1311)
                                        }, 1u64);
                                        append_bytes(a1312, a1313)
                                    }))))
                                } else {
                                    if f183(f283(v7), f183(f284(v7), f285(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1314 = v1;
                                            let a1315 = 1u64;
                                            nat_add(a1314, a1315)?
                                        }, (v5, (false, {
                                            let a1316 = v4;
                                            let a1317 = f46(v0.clone(), v1, 1u64);
                                            append_bytes(a1316, a1317)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1318 = v1;
                                            let a1319 = 1u64;
                                            nat_add(a1318, a1319)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1320 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f196(v0: Adt2) -> R<Bytes> {
    let m1325 = v0;
    match m1325 {
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
            f286(v4, v5)
        }
        Adt2::C4(v6, v7, v8) => {
            f287(v6, v7, v8)
        }
        Adt2::C5(v9, v10) => {
            f288(v9, v10)
        }
        Adt2::C6(v11) => {
            f289(v11)
        }
        Adt2::C7(r1326) => {
            let v12 = (*r1326).clone();
            Ok(f290(f291(v12)?))
        }
        Adt2::C8(r1327) => {
            let v13 = (*r1327).clone();
            f292(f293(v13)?)
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

pub fn f197(v0: List<Adt2>, v1: Bytes, v2: Bytes) -> List<Adt2> {
    let m1328 = v0;
    match m1328.uncons() {
        None => {
            List::<Adt2>::nil()
        }
        Some((v3, v4)) => {
            if f32(f123(v3.clone(), v1.clone()), v2.clone()) {
                f197(v4, v1, v2)
            } else {
                List::cons(v3, f197(v4, v1, v2))
            }
        }
    }
}

pub fn f198(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m1329 = v0;
    match m1329.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f53(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f156(f294(f200(List::cons(v1, f295(f44(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f198(v3, v1)?))
            }
        }
    }
}

pub fn f199(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a1330 = v0.clone();
        length_bytes(a1330)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h1331, _) = f66(Fn0::F296(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h1331
    };
    let v3: u64 = {
        let (h1332, _) = {
            let (_, h1333) = v2.clone();
            h1333
        };
        h1332
    };
    let v4: List<Bytes> = {
        let (_, h1334) = {
            let (_, h1335) = v2;
            h1335
        };
        h1334
    };
    let a1336 = v3;
    let a1337 = v1;
    if nat_lt(a1336, a1337) {
        Ok(f68(List::cons(f46(v0, v3, {
            let a1338 = v1;
            let a1339 = v3;
            nat_sub(a1338, a1339)
        }), v4)))
    } else {
        Ok(f68(v4))
    }
}

pub fn f200(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f297(f116(Fn2::F298, List::<Bytes>::nil(), f299(v0)?)))
}

pub fn f201(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a1340 = v0.clone();
        length_bytes(a1340)
    };
    let v3: u64 = {
        let a1341 = v1.clone();
        length_bytes(a1341)
    };
    let v4: u64 = {
        let (h1342, _) = f204(Fn5::F300(v1.clone(), v3, v0.clone(), v2), {
            let a1343 = v2;
            let a1344 = 1u64;
            nat_add(a1343, a1344)?
        }, 0u64)?;
        h1342
    };
    if f133(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f202(v0: Fn4, p1349: u64, p1350: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1348 = (p1349, p1350);
    {
        let mut m1348 = m1348;
        loop {
            let step = {
                let (v1, v2) = m1348;
                let m1345 = v1;
                if m1345 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1345 - 1;
                    let m1347 = {
                        let c1346: Fn4 = v0.clone();
                        c1346.apply(v2.clone())?
                    };
                    match m1347 {
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
                Ok(next) => { m1348 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f203(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f301(v0, v1)
}

pub fn f204(v0: Fn5, p1355: u64, p1356: u64) -> R<(u64, bool)> {
    let m1354 = (p1355, p1356);
    {
        let mut m1354 = m1354;
        loop {
            let step = {
                let (v1, v2) = m1354;
                let m1351 = v1;
                if m1351 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1351 - 1;
                    let m1353 = {
                        let c1352: Fn5 = v0.clone();
                        c1352.apply(v2)?
                    };
                    match m1353 {
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
                Ok(next) => { m1354 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f205(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f302(v0, v1)
}

pub fn f206(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f303(v0, v1)
}

pub fn f207(v0: Bytes, v1: u64) -> R<bool> {
    if if if f162(v0.clone(), v1, Str::lit("---")) {
        true
    } else {
        f162(v0.clone(), v1, Str::lit("..."))
    } {
        if f304(f209(v0.clone(), {
            let a1357 = v1;
            let a1358 = 3u64;
            nat_add(a1357, a1358)?
        })?) {
            true
        } else {
            f305(f209(v0.clone(), {
                let a1359 = v1;
                let a1360 = 3u64;
                nat_add(a1359, a1360)?
            })?)
        }
    } else {
        false
    } {
        let a1361 = f306(v0, v1)?;
        let a1362 = 0u64;
        Ok(nat_eq(a1361, a1362))
    } else {
        Ok(false)
    }
}

pub fn f208(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f307(v0.clone(), v1)?;
    let v3: u64 = f209(v0.clone(), v2)?;
    if if f305(v3) {
        true
    } else {
        let a1363 = v3;
        let a1364 = 35u64;
        if nat_eq(a1363, a1364) {
            let a1365 = v1;
            let a1366 = v2;
            nat_lt(a1365, a1366)
        } else {
            false
        }
    } {
        Ok(Ok::<u64, Bytes>(f308(v0, v2)?))
    } else {
        Ok(Err::<u64, Bytes>(f137(Str::lit("only white space and a comment may follow a node on its line (YAML 1.2.2 [77])"), v2)?))
    }
}

pub fn f209(v0: Bytes, v1: u64) -> R<u64> {
    f111(v0, v1)
}

pub fn f210(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: List<Bytes>, v8: Adt21) -> R<Adt22> {
    let m1367 = v3;
    if m1367 == 0 {
        Ok(Adt22::C1(f137(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v9 = m1367 - 1;
        let a1368 = v4;
        let a1369 = 0u64;
        if nat_eq(a1368, a1369) {
            let v10: u64 = f309(v0.clone(), v1)?;
            let v11: u64 = {
                let a1370 = v10;
                let a1371 = v1;
                nat_sub(a1370, a1371)
            };
            let a1372 = f209(v0.clone(), v10)?;
            let a1373 = 9u64;
            if nat_eq(a1372, a1373) {
                let v12: u64 = f307(v0.clone(), v10)?;
                if if f310(v0.clone(), v12)? {
                    true
                } else {
                    f311(v0.clone(), v12)?
                } {
                    Ok(Adt22::C1(f137(Str::lit("a tab cannot indent a block node (YAML 1.2.2 [63])"), v12)?))
                } else {
                    let m1374 = f312(v0.clone(), v12)?;
                    match m1374 {
                        None => {
                            f313(v0, v12, v2, v8)
                        }
                        Some(v13) => {
                            Ok(Adt22::C1(v13))
                        }
                    }
                }
            } else {
                let a1375 = f209(v0.clone(), v10)?;
                let a1376 = 33u64;
                if if nat_eq(a1375, a1376) {
                    f211(f311(v0.clone(), v10)?)
                } else {
                    false
                } {
                    if f314(v8) {
                        Ok(Adt22::C1(f137(Str::lit("a node holds at most one tag (YAML 1.2.2 [96])"), v10)?))
                    } else {
                        let v14: Adt21 = f315(v0.clone(), v10)?;
                        let m1377 = f316(v14.clone());
                        match m1377 {
                            None => {
                                f210(v0.clone(), f317(v0.clone(), v10)?, v2, v9, 1u64, false, {
                                    let a1379 = {
                                        let a1378 = v0;
                                        length_bytes(a1378)
                                    };
                                    let a1380 = 1u64;
                                    nat_add(a1379, a1380)?
                                }, List::<Bytes>::nil(), v14)
                            }
                            Some(v15) => {
                                Ok(Adt22::C1(v15))
                            }
                        }
                    }
                } else {
                    if f310(v0.clone(), v10)? {
                        f318(v8, true, v10, f210(v0, v10, v2, v9, 2u64, false, v11, List::<Bytes>::nil(), Adt21::C0(0u64))?)
                    } else {
                        let m1383 = {
                            let a1381 = f209(v0.clone(), v10)?;
                            let a1382 = 33u64;
                            if nat_eq(a1381, a1382) {
                                None::<Bytes>
                            } else {
                                f312(v0.clone(), v10)?
                            }
                        };
                        match m1383 {
                            None => {
                                if f311(v0.clone(), v10)? {
                                    f318(v8, false, v10, f210(v0, v10, v2, v9, 3u64, false, v11, List::<Bytes>::nil(), Adt21::C0(0u64))?)
                                } else {
                                    f313(v0, v10, v2, v8)
                                }
                            }
                            Some(v16) => {
                                Ok(Adt22::C1(v16))
                            }
                        }
                    }
                }
            }
        } else {
            let a1384 = v4;
            let a1385 = 1u64;
            if nat_eq(a1384, a1385) {
                let v17: u64 = f307(v0.clone(), v1)?;
                let v18: u64 = f209(v0.clone(), v17)?;
                if if f305(v18) {
                    true
                } else {
                    let a1386 = v18;
                    let a1387 = 35u64;
                    if nat_eq(a1386, a1387) {
                        let a1388 = v1;
                        let a1389 = v17;
                        nat_lt(a1388, a1389)
                    } else {
                        false
                    }
                } {
                    let v19: u64 = f136(v0.clone(), f308(v0.clone(), v17)?)?;
                    let a1391 = {
                        let a1390 = v0.clone();
                        length_bytes(a1390)
                    };
                    let a1392 = v19;
                    if if nat_le(a1391, a1392) {
                        true
                    } else {
                        f207(v0.clone(), v19)?
                    } {
                        f319(v8, v17, v19)
                    } else {
                        let v20: u64 = {
                            let a1393 = f309(v0.clone(), v19)?;
                            let a1394 = v19;
                            nat_sub(a1393, a1394)
                        };
                        let a1395 = v2;
                        let a1396 = v20;
                        if nat_le(a1395, a1396) {
                            f210(v0, v19, v2, v9, 0u64, false, 0u64, List::<Bytes>::nil(), v8)
                        } else {
                            if if if f211(v5) {
                                let a1397 = v20;
                                let a1398 = v6;
                                nat_eq(a1397, a1398)
                            } else {
                                false
                            } {
                                f310(v0.clone(), {
                                    let a1399 = v19;
                                    let a1400 = v20;
                                    nat_add(a1399, a1400)?
                                })?
                            } else {
                                false
                            } {
                                f318(v8, true, v17, f210(v0, {
                                    let a1401 = v19;
                                    let a1402 = v20;
                                    nat_add(a1401, a1402)?
                                }, v2, v9, 2u64, false, v20, List::<Bytes>::nil(), Adt21::C0(0u64))?)
                            } else {
                                f319(v8, v17, v19)
                            }
                        }
                    }
                } else {
                    let a1403 = v18;
                    let a1404 = 33u64;
                    if if nat_eq(a1403, a1404) {
                        f314(v8.clone())
                    } else {
                        false
                    } {
                        Ok(Adt22::C1(f137(Str::lit("a node holds at most one tag (YAML 1.2.2 [96])"), v17)?))
                    } else {
                        let a1405 = v18;
                        let a1406 = 33u64;
                        if if nat_eq(a1405, a1406) {
                            f211(if v5 {
                                f311(v0.clone(), v17)?
                            } else {
                                false
                            })
                        } else {
                            false
                        } {
                            let v21: Adt21 = f315(v0.clone(), v17)?;
                            let m1407 = f316(v21.clone());
                            match m1407 {
                                None => {
                                    f210(v0.clone(), f317(v0, v17)?, v2, v9, 1u64, v5, v6, List::<Bytes>::nil(), v21)
                                }
                                Some(v22) => {
                                    Ok(Adt22::C1(v22))
                                }
                            }
                        } else {
                            if if v5 {
                                f310(v0.clone(), v17)?
                            } else {
                                false
                            } {
                                let a1410 = f320(f321(v0.clone(), v1, v17), 9u64)?;
                                let a1411 = {
                                    let a1408 = v17;
                                    let a1409 = v1;
                                    nat_sub(a1408, a1409)
                                };
                                if nat_lt(a1410, a1411) {
                                    Ok(Adt22::C1(f137(Str::lit("a tab cannot indent a block node (YAML 1.2.2 [63])"), v17)?))
                                } else {
                                    f318(v8, true, v17, f210(v0.clone(), v17, v2, v9, 2u64, false, f306(v0, v17)?, List::<Bytes>::nil(), Adt21::C0(0u64))?)
                                }
                            } else {
                                let m1414 = {
                                    let a1412 = v18;
                                    let a1413 = 33u64;
                                    if nat_eq(a1412, a1413) {
                                        None::<Bytes>
                                    } else {
                                        f312(v0.clone(), v17)?
                                    }
                                };
                                match m1414 {
                                    None => {
                                        if if v5 {
                                            f311(v0.clone(), v17)?
                                        } else {
                                            false
                                        } {
                                            f318(v8, false, v17, f210(v0.clone(), v17, v2, v9, 3u64, false, f306(v0, v17)?, List::<Bytes>::nil(), Adt21::C0(0u64))?)
                                        } else {
                                            f313(v0, v17, v2, v8)
                                        }
                                    }
                                    Some(v23) => {
                                        Ok(Adt22::C1(v23))
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                let a1415 = v4;
                let a1416 = 2u64;
                if nat_eq(a1415, a1416) {
                    let m1421 = f210(v0.clone(), {
                        let a1417 = v1;
                        let a1418 = 1u64;
                        nat_add(a1417, a1418)?
                    }, {
                        let a1419 = v6;
                        let a1420 = 1u64;
                        nat_add(a1419, a1420)?
                    }, v9, 1u64, true, v6, List::<Bytes>::nil(), Adt21::C0(0u64))?;
                    match m1421 {
                        Adt22::C0(v25, v26) => {
                            let v27: u64 = f136(v0.clone(), v26)?;
                            let v28: u64 = {
                                let a1422 = f309(v0.clone(), v27)?;
                                let a1423 = v27;
                                nat_sub(a1422, a1423)
                            };
                            let a1425 = {
                                let a1424 = v0.clone();
                                length_bytes(a1424)
                            };
                            let a1426 = v27;
                            if nat_le(a1425, a1426) {
                                Ok(Adt22::C0(Adt2::C10(std::rc::Rc::new(v25), std::rc::Rc::new(Adt2::C9)), v27))
                            } else {
                                let a1427 = v28;
                                let a1428 = v6;
                                if if nat_eq(a1427, a1428) {
                                    f310(v0.clone(), {
                                        let a1429 = v27;
                                        let a1430 = v28;
                                        nat_add(a1429, a1430)?
                                    })?
                                } else {
                                    false
                                } {
                                    let m1433 = f210(v0, {
                                        let a1431 = v27;
                                        let a1432 = v28;
                                        nat_add(a1431, a1432)?
                                    }, v2, v9, 2u64, false, v6, List::<Bytes>::nil(), Adt21::C0(0u64))?;
                                    match m1433 {
                                        Adt22::C0(v30, v31) => {
                                            Ok(Adt22::C0(Adt2::C10(std::rc::Rc::new(v25), std::rc::Rc::new(v30)), v31))
                                        }
                                        Adt22::C1(v29) => {
                                            Ok(Adt22::C1(v29))
                                        }
                                    }
                                } else {
                                    let a1434 = v6;
                                    let a1435 = v28;
                                    if nat_lt(a1434, a1435) {
                                        Ok(Adt22::C1(f137(Str::lit("a line is indented more than its block sequence entry allows (YAML 1.2.2 [183])"), {
                                            let a1436 = v27;
                                            let a1437 = v28;
                                            nat_add(a1436, a1437)?
                                        })?))
                                    } else {
                                        Ok(Adt22::C0(Adt2::C10(std::rc::Rc::new(v25), std::rc::Rc::new(Adt2::C9)), v27))
                                    }
                                }
                            }
                        }
                        Adt22::C1(v24) => {
                            Ok(Adt22::C1(v24))
                        }
                    }
                } else {
                    let m1438 = f322(v0.clone(), v1)?;
                    match m1438 {
                        Ok(v33) => {
                            let m1439 = f312(v0.clone(), v33)?;
                            match m1439 {
                                None => {
                                    let m1440 = f323(v0.clone(), v33)?;
                                    match m1440 {
                                        None => {
                                            Ok(Adt22::C1(f137(Str::lit("a block mapping entry must be an implicit key and : (YAML 1.2.2 [188])"), v33)?))
                                        }
                                        Some(v35) => {
                                            let v36: Bytes = f324(v0.clone(), v33)?;
                                            if f325(v36.clone(), v7.clone()) {
                                                Ok(Adt22::C1(f137(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                            } else {
                                                let m1445 = f210(v0.clone(), {
                                                    let a1441 = v35;
                                                    let a1442 = 1u64;
                                                    nat_add(a1441, a1442)?
                                                }, {
                                                    let a1443 = v6;
                                                    let a1444 = 1u64;
                                                    nat_add(a1443, a1444)?
                                                }, v9, 1u64, false, v6, List::<Bytes>::nil(), Adt21::C0(0u64))?;
                                                match m1445 {
                                                    Adt22::C0(v38, v39) => {
                                                        let v40: u64 = f136(v0.clone(), v39)?;
                                                        let v41: u64 = {
                                                            let a1446 = f309(v0.clone(), v40)?;
                                                            let a1447 = v40;
                                                            nat_sub(a1446, a1447)
                                                        };
                                                        let a1449 = {
                                                            let a1448 = v0.clone();
                                                            length_bytes(a1448)
                                                        };
                                                        let a1450 = v40;
                                                        if nat_le(a1449, a1450) {
                                                            Ok(Adt22::C0(Adt2::C11(v36, std::rc::Rc::new(v38), std::rc::Rc::new(Adt2::C9)), v40))
                                                        } else {
                                                            let a1451 = v41;
                                                            let a1452 = v6;
                                                            if if if nat_eq(a1451, a1452) {
                                                                f211(f310(v0.clone(), {
                                                                    let a1453 = v40;
                                                                    let a1454 = v41;
                                                                    nat_add(a1453, a1454)?
                                                                })?)
                                                            } else {
                                                                false
                                                            } {
                                                                f211(f207(v0.clone(), v40)?)
                                                            } else {
                                                                false
                                                            } {
                                                                let m1457 = f210(v0, {
                                                                    let a1455 = v40;
                                                                    let a1456 = v41;
                                                                    nat_add(a1455, a1456)?
                                                                }, v2, v9, 3u64, false, v6, List::cons(v36.clone(), v7), Adt21::C0(0u64))?;
                                                                match m1457 {
                                                                    Adt22::C0(v43, v44) => {
                                                                        Ok(Adt22::C0(Adt2::C11(v36, std::rc::Rc::new(v38), std::rc::Rc::new(v43)), v44))
                                                                    }
                                                                    Adt22::C1(v42) => {
                                                                        Ok(Adt22::C1(v42))
                                                                    }
                                                                }
                                                            } else {
                                                                let a1458 = v6;
                                                                let a1459 = v41;
                                                                if nat_lt(a1458, a1459) {
                                                                    Ok(Adt22::C1(f137(Str::lit("a line is indented more than its block mapping entry allows (YAML 1.2.2 [187])"), {
                                                                        let a1460 = v40;
                                                                        let a1461 = v41;
                                                                        nat_add(a1460, a1461)?
                                                                    })?))
                                                                } else {
                                                                    Ok(Adt22::C0(Adt2::C11(v36, std::rc::Rc::new(v38), std::rc::Rc::new(Adt2::C9)), v40))
                                                                }
                                                            }
                                                        }
                                                    }
                                                    Adt22::C1(v37) => {
                                                        Ok(Adt22::C1(v37))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                Some(v34) => {
                                    Ok(Adt22::C1(v34))
                                }
                            }
                        }
                        Err(v32) => {
                            Ok(Adt22::C1(v32))
                        }
                    }
                }
            }
        }
    }
}

pub fn f211(v0: bool) -> bool {
    !v0
}

pub fn f212(v0: Bytes, v1: u64) -> Bytes {
    let m1465 = {
        let a1462 = v0;
        let a1463 = v1;
        let a1464 = 1u64;
        slice_bytes(a1462, a1463, a1464)
    };
    match m1465 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f213(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f326(v1, v0, v2)
}

pub fn f214(v0: Bytes) -> R<Result<List<Adt23>, Bytes>> {
    f327({
        let a1467 = {
            let a1466 = v0.clone();
            length_bytes(a1466)
        };
        let a1468 = 1u64;
        nat_add(a1467, a1468)?
    }, v0, 0u64, List::<Adt23>::nil())
}

pub fn f215(p1476: u64, p1477: List<Adt23>, p1478: List<Adt13>) -> R<Result<List<Adt13>, Bytes>> {
    let m1475 = (p1476, (p1477, p1478));
    {
        let mut m1475 = m1475;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1475;
                let m1469 = v0;
                if m1469 == 0 {
                    Err::<(u64, (List<Adt23>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>(Ok(Err::<List<Adt13>, Bytes>(f8(Str::lit("the input has more definitions than tokens")))))
                } else {
                    let v3 = m1469 - 1;
                    let m1470 = v1.clone();
                    match m1470.uncons() {
                        None => {
                            Err::<(u64, (List<Adt23>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>(Ok(Ok::<List<Adt13>, Bytes>(f328(v2, List::<Adt13>::nil()))))
                        }
                        Some((_, _)) => {
                            let m1471 = f329(v1)?;
                            match m1471 {
                                Ok(v7) => {
                                    let m1472 = f330(v3, v7)?;
                                    match m1472 {
                                        Ok(v9) => {
                                            Ok::<(u64, (List<Adt23>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>((v3, ({
                                                let (_, h1473) = v9;
                                                h1473
                                            }, List::cons({
                                                let (h1474, _) = v9;
                                                h1474
                                            }, v2))))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (List<Adt23>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>(Ok(Err::<List<Adt13>, Bytes>(v8)))
                                        }
                                    }
                                }
                                Err(v6) => {
                                    Err::<(u64, (List<Adt23>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>(Ok(Err::<List<Adt13>, Bytes>(v6)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1475 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f216(v0: List<Adt16>, v1: List<Adt16>) -> List<Adt16> {
    let m1479 = v0;
    match m1479.uncons() {
        None => {
            List::<Adt16>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = if f32(f223(v2.clone()), Bytes::lit(&[])) {
                f331(f225(v2.clone()), v1.clone())
            } else {
                f223(v2.clone())
            };
            List::cons(Adt16::C0(f226(v2.clone()), v4, f227(v2.clone()), f225(v2)), f216(v3, v1))
        }
    }
}

pub fn f217(v0: List<Adt16>) -> R<u64> {
    let m1480 = v0;
    match m1480.uncons() {
        None => {
            Ok(0u64)
        }
        Some((v1, v2)) => {
            if f32(f223(v1), Bytes::lit(&[])) {
                f217(v2)
            } else {
                let a1481 = f217(v2)?;
                let a1482 = 1u64;
                nat_add(a1481, a1482)
            }
        }
    }
}

pub fn f218(v0: u64) -> bool {
    let a1483 = v0;
    let a1484 = 0u64;
    if nat_eq(a1483, a1484) {
        true
    } else {
        let a1485 = v0;
        let a1486 = 2u64;
        if nat_eq(a1485, a1486) {
            true
        } else {
            let a1487 = v0;
            let a1488 = 3u64;
            if nat_eq(a1487, a1488) {
                true
            } else {
                let a1489 = v0;
                let a1490 = 4u64;
                if nat_eq(a1489, a1490) {
                    true
                } else {
                    let a1491 = v0;
                    let a1492 = 9u64;
                    nat_eq(a1491, a1492)
                }
            }
        }
    }
}

pub fn f219(v0: Adt13) -> u64 {
    let m1493 = v0;
    match m1493 {
        Adt13::C0(v1, _, _, _, _, _) => {
            v1
        }
    }
}

pub fn f220(v0: List<Adt13>, v1: Adt13) -> R<Result<(Bytes, List<Bytes>), Bytes>> {
    let v2: u64 = f219(v1.clone());
    let v3: Bytes = f332(f221(v1.clone()))?;
    let a1494 = v2;
    let a1495 = 0u64;
    if if nat_eq(a1494, a1495) {
        true
    } else {
        let a1496 = v2;
        let a1497 = 2u64;
        nat_eq(a1496, a1497)
    } {
        let m1498 = f333(v0.clone(), v0.clone(), f221(v1.clone()));
        match m1498 {
            None => {
                let m1502 = f334({
                    let a1500 = {
                        let a1499 = v0.clone();
                        length_list(a1499)
                    };
                    let a1501 = 1u64;
                    nat_add(a1500, a1501)?
                }, v0, v1)?;
                match m1502 {
                    Ok(v5) => {
                        let a1504 = {
                            let a1503 = v5.clone();
                            length_list(a1503)
                        };
                        let a1505 = 0u64;
                        if nat_eq(a1504, a1505) {
                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f335(v3), List::<Bytes>::nil())))
                        } else {
                            let m1506 = f336(f337(f338(v5.clone()))?, v5)?;
                            match m1506 {
                                Ok(v7) => {
                                    Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f339(v3, {
                                        let (h1507, _) = v7;
                                        h1507
                                    }), {
                                        let (_, h1508) = v7;
                                        h1508
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
        let a1509 = v2;
        let a1510 = 3u64;
        if nat_eq(a1509, a1510) {
            let m1511 = f172(f340(v1.clone()));
            match m1511 {
                None => {
                    let m1512 = f341(f337(f340(v1.clone()))?, f342(f340(v1)), List::<Bytes>::nil())?;
                    match m1512 {
                        Ok(v10) => {
                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f343(v3, v10), List::<Bytes>::nil())))
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
            let a1513 = v2;
            let a1514 = 4u64;
            if nat_eq(a1513, a1514) {
                let v12: Adt15 = f344(v1);
                if !f32(f345(v12.clone()), Bytes::lit(&[])) {
                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f345(v12)))
                } else {
                    let a1515 = f346(v12.clone());
                    let a1516 = 1u64;
                    if nat_eq(a1515, a1516) {
                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f343(v3, List::cons(f347(v12.clone()), List::<Bytes>::nil())), f348(v12))))
                    } else {
                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f349(v3, f347(v12.clone())), f348(v12))))
                    }
                }
            } else {
                Ok(Err::<(Bytes, List<Bytes>), Bytes>(f7(f340(v1))))
            }
        }
    }
}

pub fn f221(v0: Adt13) -> Bytes {
    let m1517 = v0;
    match m1517 {
        Adt13::C0(_, v2, _, _, _, _) => {
            v2
        }
    }
}

pub fn f222(p1520: List<Adt16>, v1: Bytes) -> Option<Adt16> {
    let m1519 = p1520;
    {
        let mut m1519 = m1519;
        loop {
            let step = {
                let v0 = m1519;
                let m1518 = v0;
                match m1518.uncons() {
                    None => {
                        Err::<List<Adt16>, Option<Adt16>>(None::<Adt16>)
                    }
                    Some((v2, v3)) => {
                        if f32(f226(v2.clone()), v1.clone()) {
                            Err::<List<Adt16>, Option<Adt16>>(Some(v2))
                        } else {
                            Ok::<List<Adt16>, Option<Adt16>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1519 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f223(v0: Adt16) -> Bytes {
    let m1521 = v0;
    match m1521 {
        Adt16::C0(_, v2, _, _) => {
            v2
        }
    }
}

pub fn f224(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m1522 = v0;
    match m1522.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f224(v3, v1))
        }
    }
}

pub fn f225(v0: Adt16) -> List<Bytes> {
    let m1523 = v0;
    match m1523 {
        Adt16::C0(_, _, _, v4) => {
            v4
        }
    }
}

pub fn f226(v0: Adt16) -> Bytes {
    let m1524 = v0;
    match m1524 {
        Adt16::C0(v1, _, _, _) => {
            v1
        }
    }
}

pub fn f227(v0: Adt16) -> Bytes {
    let m1525 = v0;
    match m1525 {
        Adt16::C0(_, _, v3, _) => {
            v3
        }
    }
}

pub fn f228(v0: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)> {
    let m1526 = v0;
    match m1526.uncons() {
        None => {
            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
        }
        Some((v2, v3)) => {
            let m1529 = f253({
                let (h1527, _) = {
                    let (_, h1528) = v2.clone();
                    h1528
                };
                h1527
            }, v1.clone());
            match m1529 {
                None => {
                    Some((({
                        let (h1530, _) = v2;
                        h1530
                    }, {
                        let (_, h1531) = {
                            let (_, h1532) = v2;
                            h1532
                        };
                        h1531
                    }), v3))
                }
                Some(_) => {
                    let m1533 = f228(v3, v1);
                    match m1533 {
                        None => {
                            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h1534, _) = v5;
                                h1534
                            }, List::cons(v2, {
                                let (_, h1535) = v5;
                                h1535
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f229(p1540: List<Adt13>, v1: u64, v2: Bytes) -> Option<Adt13> {
    let m1539 = p1540;
    {
        let mut m1539 = m1539;
        loop {
            let step = {
                let v0 = m1539;
                let m1536 = v0;
                match m1536.uncons() {
                    None => {
                        Err::<List<Adt13>, Option<Adt13>>(None::<Adt13>)
                    }
                    Some((v3, v4)) => {
                        let a1537 = f219(v3.clone());
                        let a1538 = v1;
                        if if nat_eq(a1537, a1538) {
                            f32(f221(v3.clone()), v2.clone())
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
                Ok(next) => { m1539 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f230(v0: u64) -> Bytes {
    let a1541 = v0;
    let a1542 = 5u64;
    if nat_eq(a1541, a1542) {
        f8(Str::lit("partial interface "))
    } else {
        let a1543 = v0;
        let a1544 = 7u64;
        if nat_eq(a1543, a1544) {
            f8(Str::lit("partial dictionary "))
        } else {
            f8(Str::lit(""))
        }
    }
}

pub fn f231(v0: Adt13) -> Bytes {
    let m1545 = v0;
    match m1545 {
        Adt13::C0(_, _, v3, _, _, _) => {
            v3
        }
    }
}

pub fn f232(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f350(v0, v1)
}

pub fn f233(v0: u64) -> R<bool> {
    f351(v0, f352())
}

pub fn f234(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let v3: u64 = {
        let (h1546, _) = f204(Fn5::F353(v1.clone(), v0.clone()), {
            let a1548 = {
                let a1547 = v0.clone();
                length_bytes(a1547)
            };
            let a1549 = 1u64;
            nat_add(a1548, a1549)?
        }, v2)?;
        h1546
    };
    if f133(v0, v1, v3) {
        Ok(Some(v3))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f235(v0: Bytes) -> Result<u64, Bytes> {
    Err::<u64, Bytes>(v0)
}

pub fn f236(v0: Bytes, v1: u64) -> R<u64> {
    let (h1550, _) = f204(Fn5::F354(v0.clone()), {
        let a1552 = {
            let a1551 = v0;
            length_bytes(a1551)
        };
        let a1553 = 1u64;
        nat_add(a1552, a1553)?
    }, v1)?;
    Ok(h1550)
}

pub fn f237(v0: Bytes) -> bool {
    f325(v0, List::cons(f8(Str::lit("UTF-8")), List::cons(f8(Str::lit("UTf-8")), List::cons(f8(Str::lit("UtF-8")), List::cons(f8(Str::lit("Utf-8")), List::cons(f8(Str::lit("uTF-8")), List::cons(f8(Str::lit("uTf-8")), List::cons(f8(Str::lit("utF-8")), List::cons(f8(Str::lit("utf-8")), List::<Bytes>::nil())))))))))
}

pub fn f238(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let m1557 = f234(v0.clone(), {
        let a1554 = Str::lit("--");
        utf8_encode(a1554)
    }, {
        let a1555 = v1;
        let a1556 = 4u64;
        nat_add(a1555, a1556)?
    })?;
    match m1557 {
        None => {
            Ok(f235(f137(Str::lit("a comment is not closed (XML 1.0 [15])"), v1)?))
        }
        Some(v2) => {
            let a1560 = f163(v0, {
                let a1558 = v2;
                let a1559 = 2u64;
                nat_add(a1558, a1559)?
            })?;
            let a1561 = 62u64;
            if nat_eq(a1560, a1561) {
                Ok(Ok::<u64, Bytes>({
                    let a1562 = v2;
                    let a1563 = 3u64;
                    nat_add(a1562, a1563)?
                }))
            } else {
                Ok(f235(f137(Str::lit("-- is not allowed inside a comment (XML 1.0 [15])"), v2)?))
            }
        }
    }
}

pub fn f239(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f241(v0.clone(), {
        let a1564 = v1;
        let a1565 = 2u64;
        nat_add(a1564, a1565)?
    })?;
    let a1568 = v2;
    let a1569 = {
        let a1566 = v1;
        let a1567 = 2u64;
        nat_add(a1566, a1567)?
    };
    if nat_eq(a1568, a1569) {
        Ok(f235(f137(Str::lit("a processing instruction must start with a Name (XML 1.0 [16])"), v1)?))
    } else {
        let a1572 = v2;
        let a1573 = {
            let a1570 = v1;
            let a1571 = 5u64;
            nat_add(a1570, a1571)?
        };
        if if nat_eq(a1572, a1573) {
            f325(f46(v0.clone(), {
                let a1574 = v1;
                let a1575 = 2u64;
                nat_add(a1574, a1575)?
            }, 3u64), List::cons(f8(Str::lit("XML")), List::cons(f8(Str::lit("XMl")), List::cons(f8(Str::lit("XmL")), List::cons(f8(Str::lit("Xml")), List::cons(f8(Str::lit("xML")), List::cons(f8(Str::lit("xMl")), List::cons(f8(Str::lit("xmL")), List::cons(f8(Str::lit("xml")), List::<Bytes>::nil())))))))))
        } else {
            false
        } {
            Ok(f235(f137(Str::lit("the target xml is reserved; an XML declaration may appear only at the very start (XML 1.0 [17], [22])"), v1)?))
        } else {
            let m1577 = f234(v0, {
                let a1576 = Str::lit("?>");
                utf8_encode(a1576)
            }, v2)?;
            match m1577 {
                None => {
                    Ok(f235(f137(Str::lit("a processing instruction is not closed (XML 1.0 [16])"), v1)?))
                }
                Some(v3) => {
                    Ok(Ok::<u64, Bytes>({
                        let a1578 = v3;
                        let a1579 = 2u64;
                        nat_add(a1578, a1579)?
                    }))
                }
            }
        }
    }
}

pub fn f240(v0: Bytes) -> Result<(Adt17, u64), Bytes> {
    Err::<(Adt17, u64), Bytes>(v0)
}

pub fn f241(v0: Bytes, v1: u64) -> R<u64> {
    let (h1580, _) = f204(Fn5::F355(v0.clone(), v1), {
        let a1582 = {
            let a1581 = v0;
            length_bytes(a1581)
        };
        let a1583 = 1u64;
        nat_add(a1582, a1583)?
    }, v1)?;
    Ok(h1580)
}

pub fn f242(v0: Bytes, v1: u64, v2: u64, v3: List<Bytes>) -> R<Result<(Adt17, u64), Bytes>> {
    let m1584 = v2;
    if m1584 == 0 {
        Ok(f240(f137(Str::lit("a tag has more attributes than the input has bytes"), v1)?))
    } else {
        let v4 = m1584 - 1;
        let v5: u64 = f236(v0.clone(), v1)?;
        let a1585 = f163(v0.clone(), v5)?;
        let a1586 = 62u64;
        if if nat_eq(a1585, a1586) {
            true
        } else {
            f162(v0.clone(), v5, Str::lit("/>"))
        } {
            Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C4, v5)))
        } else {
            let a1587 = v5;
            let a1588 = v1;
            if nat_eq(a1587, a1588) {
                Ok(f240(f137(Str::lit("attributes must be separated by white space (XML 1.0 [40])"), v1)?))
            } else {
                let v6: u64 = f241(v0.clone(), v5)?;
                let a1589 = v6;
                let a1590 = v5;
                if nat_eq(a1589, a1590) {
                    Ok(f240(f137(Str::lit("an attribute must start with a Name (XML 1.0 [41])"), v5)?))
                } else {
                    let v7: Bytes = f46(v0.clone(), v5, {
                        let a1591 = v6;
                        let a1592 = v5;
                        nat_sub(a1591, a1592)
                    });
                    if f325(v7.clone(), v3.clone()) {
                        Ok(f240(f137(Str::lit("an attribute name appears twice in one tag (XML 1.0 WFC: Unique Att Spec)"), v5)?))
                    } else {
                        let v8: u64 = f236(v0.clone(), v6)?;
                        let a1593 = f163(v0.clone(), v8)?;
                        let a1594 = 61u64;
                        if nat_eq(a1593, a1594) {
                            let v9: u64 = f236(v0.clone(), {
                                let a1595 = v8;
                                let a1596 = 1u64;
                                nat_add(a1595, a1596)?
                            })?;
                            let v10: u64 = f163(v0.clone(), v9)?;
                            let a1597 = v10;
                            let a1598 = 34u64;
                            if if nat_eq(a1597, a1598) {
                                true
                            } else {
                                let a1599 = v10;
                                let a1600 = 39u64;
                                nat_eq(a1599, a1600)
                            } {
                                let m1603 = f243(v0.clone(), {
                                    let a1601 = v9;
                                    let a1602 = 1u64;
                                    nat_add(a1601, a1602)?
                                }, v4, Bytes::lit(&[]), true, v10)?;
                                match m1603 {
                                    Ok(v12) => {
                                        let m1605 = f242(v0, {
                                            let (_, h1604) = v12;
                                            h1604
                                        }, v4, List::cons(v7.clone(), v3))?;
                                        match m1605 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C1(v7, {
                                                    let (h1606, _) = v12;
                                                    h1606
                                                }, std::rc::Rc::new({
                                                    let (h1607, _) = v14;
                                                    h1607
                                                })), {
                                                    let (_, h1608) = v14;
                                                    h1608
                                                })))
                                            }
                                            Err(v13) => {
                                                Ok(f240(v13))
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(f240(v11))
                                    }
                                }
                            } else {
                                Ok(f240(f137(Str::lit("an attribute value must be quoted (XML 1.0 [10])"), v9)?))
                            }
                        } else {
                            Ok(f240(f137(Str::lit("an attribute needs = and a value (XML 1.0 [25], [41])"), v8)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f243(v0: Bytes, p1648: u64, p1649: u64, p1650: Bytes, v4: bool, v5: u64) -> R<Result<(Bytes, u64), Bytes>> {
    let m1647 = (p1648, (p1649, p1650));
    {
        let mut m1647 = m1647;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1647;
                let m1609 = v2;
                if m1609 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f356(f137(Str::lit("the input ends inside a text run"), v1)?)))
                } else {
                    let v6 = m1609 - 1;
                    let v7: u64 = f163(v0.clone(), v1)?;
                    if if v4 {
                        let a1610 = v7;
                        let a1611 = v5;
                        nat_eq(a1610, a1611)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, {
                            let a1612 = v1;
                            let a1613 = 1u64;
                            nat_add(a1612, a1613)?
                        }))))
                    } else {
                        let a1614 = v7;
                        let a1615 = 256u64;
                        if nat_eq(a1614, a1615) {
                            if v4 {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f356(f137(Str::lit("an attribute value is not closed (XML 1.0 [10])"), v1)?)))
                            } else {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                            }
                        } else {
                            let a1616 = v7;
                            let a1617 = 60u64;
                            if nat_eq(a1616, a1617) {
                                if v4 {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f356(f137(Str::lit("< is not allowed in an attribute value (XML 1.0 [10], WFC: No < in Attribute Values)"), v1)?)))
                                } else {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                                }
                            } else {
                                let a1618 = v7;
                                let a1619 = 38u64;
                                if nat_eq(a1618, a1619) {
                                    let m1620 = f357(v0.clone(), v1)?;
                                    match m1620 {
                                        Ok(v9) => {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let (_, h1621) = v9;
                                                h1621
                                            }, (v6, {
                                                let a1623 = v3;
                                                let a1624 = {
                                                    let (h1622, _) = v9;
                                                    h1622
                                                };
                                                append_bytes(a1623, a1624)
                                            })))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f356(v8)))
                                        }
                                    }
                                } else {
                                    if v4 {
                                        let a1625 = v7;
                                        let a1626 = 9u64;
                                        if if nat_eq(a1625, a1626) {
                                            true
                                        } else {
                                            let a1627 = v7;
                                            let a1628 = 10u64;
                                            nat_eq(a1627, a1628)
                                        } {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let a1629 = v1;
                                                let a1630 = 1u64;
                                                nat_add(a1629, a1630)?
                                            }, (v6, {
                                                let a1631 = v3;
                                                let a1632 = f8(Str::lit(" "));
                                                append_bytes(a1631, a1632)
                                            })))
                                        } else {
                                            let v10: u64 = f358(v0.clone(), true, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v10, (v6, {
                                                let a1635 = v3;
                                                let a1636 = f46(v0.clone(), v1, {
                                                    let a1633 = v10;
                                                    let a1634 = v1;
                                                    nat_sub(a1633, a1634)
                                                });
                                                append_bytes(a1635, a1636)
                                            })))
                                        }
                                    } else {
                                        let a1637 = v7;
                                        let a1638 = 93u64;
                                        if nat_eq(a1637, a1638) {
                                            if f162(v0.clone(), v1, Str::lit("]]>")) {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f356(f137(Str::lit("]]> is not allowed in character data (XML 1.0 [14])"), v1)?)))
                                            } else {
                                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                    let a1639 = v1;
                                                    let a1640 = 1u64;
                                                    nat_add(a1639, a1640)?
                                                }, (v6, {
                                                    let a1641 = v3;
                                                    let a1642 = f8(Str::lit("]"));
                                                    append_bytes(a1641, a1642)
                                                })))
                                            }
                                        } else {
                                            let v11: u64 = f358(v0.clone(), false, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v11, (v6, {
                                                let a1645 = v3;
                                                let a1646 = f46(v0.clone(), v1, {
                                                    let a1643 = v11;
                                                    let a1644 = v1;
                                                    nat_sub(a1643, a1644)
                                                });
                                                append_bytes(a1645, a1646)
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
                Ok(next) => { m1647 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f244(v0: Bytes) -> Result<List<(Bytes, Bytes)>, Bytes> {
    Err::<List<(Bytes, Bytes)>, Bytes>(v0)
}

pub fn f245() -> Bytes {
    f8(Str::lit("http://www.w3.org/XML/1998/namespace"))
}

pub fn f246() -> Bytes {
    f8(Str::lit("http://www.w3.org/2000/xmlns/"))
}

pub fn f247(v0: Bytes) -> R<Bytes> {
    let m1651 = f234(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1651 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v1) => {
            Ok(f46(v0, 0u64, v1))
        }
    }
}

pub fn f248(p1656: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1655 = p1656;
    {
        let mut m1655 = m1655;
        loop {
            let step = {
                let v0 = m1655;
                let m1652 = v0;
                match m1652.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h1653, _) = v2;
                            h1653
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h1654) = v2;
                                h1654
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1655 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f249(v0: Bytes) -> R<Bytes> {
    let m1657 = f234(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1657 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(f46(v0.clone(), {
                let a1658 = v1;
                let a1659 = 1u64;
                nat_add(a1658, a1659)?
            }, {
                let a1663 = {
                    let a1660 = v0;
                    length_bytes(a1660)
                };
                let a1664 = {
                    let a1661 = v1;
                    let a1662 = 1u64;
                    nat_add(a1661, a1662)?
                };
                nat_sub(a1663, a1664)
            }))
        }
    }
}

pub fn f250() -> Bytes {
    f8(Str::lit("http://www.w3.org/2001/XMLSchema"))
}

pub fn f251(v0: Bytes) -> Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes> {
    Err::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v0)
}

pub fn f252(v0: Bytes, v1: Adt17, v2: List<(Bytes, Bytes)>, v3: Bytes) -> R<Result<Adt24, Bytes>> {
    let v4: Bytes = f125(v0.clone())?;
    let m1665 = f359(v1, v2)?;
    match m1665 {
        Ok(v6) => {
            let m1666 = v6;
            match m1666 {
                None => {
                    Ok(f360(f361(f8(Str::lit("a simpleType must have a restriction, list or union child (XSD 1.1 Part 2, section 4.1.2)")), v0)))
                }
                Some(v7) => {
                    let v8: Bytes = match v7.clone() {
                        Adt31::C0(h1667, _, _, _) => {
                            h1667
                        }
                    };
                    let v9: Adt17 = match v7.clone() {
                        Adt31::C0(_, h1668, _, _) => {
                            h1668
                        }
                    };
                    let v10: Adt17 = match v7.clone() {
                        Adt31::C0(_, _, h1669, _) => {
                            h1669
                        }
                    };
                    let v11: List<(Bytes, Bytes)> = match v7 {
                        Adt31::C0(_, _, _, h1670) => {
                            h1670
                        }
                    };
                    if f32(v8.clone(), f8(Str::lit("union"))) {
                        Ok(f360(f361(f8(Str::lit("a union is not translated, because a literal can be valid for more than one member type, so no LexLean inductive keeps its members disjoint (XSD 1.1 Part 2, section 2.4.1.3)")), v0)))
                    } else {
                        if f32(v8, f8(Str::lit("list"))) {
                            let m1671 = f169(v9, f8(Str::lit("itemType")));
                            match m1671 {
                                None => {
                                    Ok(f360(f361(f8(Str::lit("a list with an anonymous item type is not translated yet")), v0)))
                                }
                                Some(v12) => {
                                    let m1672 = f362(v11, v3, v12)?;
                                    match m1672 {
                                        Ok(v14) => {
                                            if f53(match v14.clone() {
                                                Adt24::C0(h1673, _) => {
                                                    h1673
                                                }
                                            }, f8(Str::lit("(List "))) {
                                                Ok(f360(f361(f8(Str::lit("the item type of a list must not itself be a list (XSD 1.1 Part 2, section 4.1.5)")), v0)))
                                            } else {
                                                Ok(Ok::<Adt24, Bytes>(Adt24::C0(f349(v4, f7(List::cons(f8(Str::lit("(List ")), List::cons(match v14.clone() {
                                                    Adt24::C0(h1674, _) => {
                                                        h1674
                                                    }
                                                }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))), match v14 {
                                                    Adt24::C0(_, h1675) => {
                                                        h1675
                                                    }
                                                })))
                                            }
                                        }
                                        Err(v13) => {
                                            Ok(f360(f361(v13, v0)))
                                        }
                                    }
                                }
                            }
                        } else {
                            let m1676 = f169(v9, f8(Str::lit("base")));
                            match m1676 {
                                None => {
                                    Ok(f360(f361(f8(Str::lit("a restriction with an anonymous base type is not translated yet")), v0)))
                                }
                                Some(v15) => {
                                    let m1677 = f362(v11.clone(), v3, v15)?;
                                    match m1677 {
                                        Ok(v17) => {
                                            let m1678 = f363(v10, v11)?;
                                            match m1678 {
                                                Ok(v19) => {
                                                    let v20: List<Adt2> = f364(v19, List::<Bytes>::nil())?;
                                                    let m1679 = v20.clone();
                                                    match m1679.uncons() {
                                                        None => {
                                                            Ok(Ok::<Adt24, Bytes>(Adt24::C0(f349(v4, match v17.clone() {
                                                                Adt24::C0(h1680, _) => {
                                                                    h1680
                                                                }
                                                            }), match v17 {
                                                                Adt24::C0(_, h1681) => {
                                                                    h1681
                                                                }
                                                            })))
                                                        }
                                                        Some((_, _)) => {
                                                            let m1682 = f341(f337(f75(v20.clone()))?, v20, List::<Bytes>::nil())?;
                                                            match m1682 {
                                                                Ok(v24) => {
                                                                    Ok(Ok::<Adt24, Bytes>(Adt24::C0(f343(v4, v24), List::<Bytes>::nil())))
                                                                }
                                                                Err(v23) => {
                                                                    Ok(f360(f361(v23, v0)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Err(v18) => {
                                                    Ok(f360(f361(v18, v0)))
                                                }
                                            }
                                        }
                                        Err(v16) => {
                                            Ok(f360(f361(v16, v0)))
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
            Ok(f360(f361(v5, v0)))
        }
    }
}

pub fn f253(p1685: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m1684 = p1685;
    {
        let mut m1684 = m1684;
        loop {
            let step = {
                let v0 = m1684;
                let m1683 = v0;
                match m1683.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f122(v2.clone(), v1.clone()) {
                            Ok::<List<Bytes>, Option<Bytes>>(v3)
                        } else {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v2))
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

pub fn f254(v0: Bytes) -> Result<List<Bytes>, Bytes> {
    Err::<List<Bytes>, Bytes>(v0)
}

pub fn f255(v0: List<Bytes>, v1: Option<Bytes>) -> R<List<(Bytes, Bytes)>> {
    let m1686 = v0;
    match m1686.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, v3)) => {
            if f32(v2.clone(), f8(Str::lit("[[authority]]"))) {
                f255(v3, None::<Bytes>)
            } else {
                let m1687 = f365(v2.clone(), f8(Str::lit("url = ")))?;
                match m1687 {
                    None => {
                        let m1688 = f365(v2, f8(Str::lit("artifact = ")))?;
                        match m1688 {
                            None => {
                                f255(v3, v1)
                            }
                            Some(v5) => {
                                let m1689 = v1.clone();
                                match m1689 {
                                    None => {
                                        f255(v3, v1)
                                    }
                                    Some(v6) => {
                                        Ok(List::cons((v6, v5), f255(v3, v1)?))
                                    }
                                }
                            }
                        }
                    }
                    Some(v4) => {
                        f255(v3, Some(v4))
                    }
                }
            }
        }
    }
}

pub fn f256(v0: List<Adt19>) -> R<Result<(u64, Bytes), Bytes>> {
    let m1690 = f366(v0.clone())?;
    match m1690 {
        Ok(v2) => {
            let v3: Adt25 = Adt25::C0(v2.clone(), f367(v0.clone()));
            let m1691 = f368(f369(v0)?, v3, v2)?;
            match m1691 {
                Ok(v5) => {
                    let m1692 = f370(f371(v5), List::<Adt28>::nil());
                    match m1692 {
                        Ok(v7) => {
                            let v8: List<Bytes> = f372(v7.clone());
                            let m1693 = f373(v7.clone(), v8.clone());
                            match m1693 {
                                None => {
                                    let v10: List<(Bytes, List<Bytes>)> = f374(v7.clone(), v8.clone(), f375(v7.clone())?);
                                    let m1699 = f376({
                                        let a1697 = {
                                            let a1696 = v8.clone();
                                            length_list(a1696)
                                        };
                                        let a1698 = 1u64;
                                        nat_add(a1697, a1698)?
                                    }, v7.clone(), f377(v7, f378(v10, v8.clone(), v8, List::<Bytes>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil());
                                    match m1699 {
                                        Ok(v12) => {
                                            Ok(Ok::<(u64, Bytes), Bytes>(({
                                                let a1700 = v12.clone();
                                                length_list(a1700)
                                            }, f156(v12, f8(Str::lit("\u{a}"))))))
                                        }
                                        Err(v11) => {
                                            Ok(Err::<(u64, Bytes), Bytes>(v11))
                                        }
                                    }
                                }
                                Some(v9) => {
                                    Ok(Err::<(u64, Bytes), Bytes>(f260(f8(Str::lit("the type ")), {
                                        let (h1694, _) = v9;
                                        h1694
                                    }, f260(f8(Str::lit(" uses ")), {
                                        let (_, h1695) = v9;
                                        h1695
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

pub fn f257(p1703: List<Adt19>, p1704: List<Adt19>) -> List<Adt19> {
    let m1702 = (p1703, p1704);
    {
        let mut m1702 = m1702;
        loop {
            let step = {
                let (v0, v1) = m1702;
                let m1701 = v0;
                match m1701.uncons() {
                    None => {
                        Err::<(List<Adt19>, List<Adt19>), List<Adt19>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt19>, List<Adt19>), List<Adt19>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1702 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f258(v0: List<Adt19>) -> List<Bytes> {
    let m1705 = v0;
    match m1705.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt19::C0(h1706, _, _, _, _, _) => {
                    h1706
                }
            }, f258(v2))
        }
    }
}

pub fn f259(p1711: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m1710 = p1711;
    {
        let mut m1710 = m1710;
        loop {
            let step = {
                let v0 = m1710;
                let m1707 = v0;
                match m1707.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h1708, _) = v2;
                            h1708
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h1709) = v2;
                                h1709
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1710 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f260(v0: Bytes, v1: Bytes, v2: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(v1, List::cons(v2, List::<Bytes>::nil()))))
}

pub fn f261(v0: Bytes, v1: Bytes, v2: Option<Bytes>) -> R<Result<Adt19, Bytes>> {
    let m1712 = f95(v1.clone())?;
    match m1712 {
        Ok(v4) => {
            let m1713 = v4;
            match m1713 {
                Adt17::C0(v5, r1718, r1719) => {
                    let v6 = (*r1718).clone();
                    let v7 = (*r1719).clone();
                    let m1714 = f166(v6.clone(), List::<(Bytes, Bytes)>::nil());
                    match m1714 {
                        Ok(v9) => {
                            let m1715 = f167(v9.clone(), v5)?;
                            match m1715 {
                                Ok(v11) => {
                                    if f168(v11, Str::lit("schema")) {
                                        Ok(Ok::<Adt19, Bytes>(Adt19::C0(v0, v2, {
                                            let m1716 = f169(v6, f8(Str::lit("targetNamespace")));
                                            match m1716 {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some(v12) => {
                                                    v12
                                                }
                                            }
                                        }, v9, v7, {
                                            let a1717 = v1;
                                            length_bytes(a1717)
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
            Ok(Err::<Adt19, Bytes>(f260(v0, f8(Str::lit(" is not well-formed XML: ")), v3)))
        }
    }
}

pub fn f262(p1724: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1723 = p1724;
    {
        let mut m1723 = m1723;
        loop {
            let step = {
                let v0 = m1723;
                let m1720 = v0;
                match m1720.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (_, h1721) = v2;
                            h1721
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (h1722, _) = v2;
                                h1722
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1723 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f263(v0: Adt19, v1: Bytes, v2: Bytes) -> Option<Bytes> {
    if f32(v1.clone(), f8(Str::lit("root"))) {
        None::<Bytes>
    } else {
        if f32(match v0.clone() {
            Adt19::C0(_, _, h1725, _, _, _) => {
                h1725
            }
        }, v2.clone()) {
            None::<Bytes>
        } else {
            Some(f7(List::cons(match v0.clone() {
                Adt19::C0(h1726, _, _, _, _, _) => {
                    h1726
                }
            }, List::cons(f8(Str::lit(" has the target namespace ")), List::cons(match v0 {
                Adt19::C0(_, _, h1727, _, _, _) => {
                    h1727
                }
            }, List::cons(f8(Str::lit(", but the xs:")), List::cons(v1.clone(), List::cons(f8(Str::lit(" that names it requires ")), List::cons(v2, List::cons(if f32(v1, f8(Str::lit("include"))) {
                f8(Str::lit("; an included document without a target namespace is not adopted into the including one (XSD 1.1 Part 1, section 4.2.3)"))
            } else {
                f8(Str::lit(" (XSD 1.1 Part 1, section 4.2.6.2)"))
            }, List::<Bytes>::nil()))))))))))
        }
    }
}

pub fn f264(v0: Adt17, v1: List<(Bytes, Bytes)>) -> R<Result<List<Adt27>, Bytes>> {
    let m1728 = v0;
    match m1728 {
        Adt17::C0(_, _, _) => {
            Ok(Ok::<List<Adt27>, Bytes>(List::<Adt27>::nil()))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<List<Adt27>, Bytes>(List::<Adt27>::nil()))
        }
        Adt17::C2(_) => {
            Ok(Ok::<List<Adt27>, Bytes>(List::<Adt27>::nil()))
        }
        Adt17::C3(r1739, r1740) => {
            let v2 = (*r1739).clone();
            let v3 = (*r1740).clone();
            let m1729 = f264(v3, v1.clone())?;
            match m1729 {
                Ok(v5) => {
                    let m1730 = v2;
                    match m1730 {
                        Adt17::C0(v6, r1737, r1738) => {
                            let v7 = (*r1737).clone();
                            let v8 = (*r1738).clone();
                            let m1731 = f166(v7.clone(), v1);
                            match m1731 {
                                Ok(v10) => {
                                    let m1732 = f167(v10.clone(), v6)?;
                                    match m1732 {
                                        Ok(v12) => {
                                            if if f32({
                                                let (h1733, _) = v12.clone();
                                                h1733
                                            }, f250()) {
                                                f379(f32({
                                                    let (_, h1734) = v12.clone();
                                                    h1734
                                                }, f8(Str::lit("annotation"))))
                                            } else {
                                                false
                                            } {
                                                Ok(Ok::<List<Adt27>, Bytes>(List::cons(Adt27::C0({
                                                    let (h1735, _) = v12;
                                                    h1735
                                                }, {
                                                    let (_, h1736) = v12;
                                                    h1736
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
                        Adt17::C1(_, _, _) => {
                            Ok(Ok::<List<Adt27>, Bytes>(v5))
                        }
                        Adt17::C2(_) => {
                            Ok(Ok::<List<Adt27>, Bytes>(v5))
                        }
                        Adt17::C3(_, _) => {
                            Ok(Ok::<List<Adt27>, Bytes>(v5))
                        }
                        Adt17::C4 => {
                            Ok(Ok::<List<Adt27>, Bytes>(v5))
                        }
                    }
                }
                Err(v4) => {
                    Ok(Err::<List<Adt27>, Bytes>(v4))
                }
            }
        }
        Adt17::C4 => {
            Ok(Ok::<List<Adt27>, Bytes>(List::<Adt27>::nil()))
        }
    }
}

pub fn f265(v0: List<Adt27>, v1: Adt19, v2: List<(Bytes, Bytes)>) -> R<Result<List<(Bytes, (Bytes, Bytes))>, Bytes>> {
    let m1741 = v0;
    match m1741.uncons() {
        None => {
            Ok(Ok::<List<(Bytes, (Bytes, Bytes))>, Bytes>(List::<(Bytes, (Bytes, Bytes))>::nil()))
        }
        Some((v3, v4)) => {
            let m1742 = f265(v4, v1.clone(), v2.clone())?;
            match m1742 {
                Ok(v6) => {
                    if if f380(v3.clone(), Str::lit("redefine")) {
                        true
                    } else {
                        if f380(v3.clone(), Str::lit("override")) {
                            true
                        } else {
                            f380(v3.clone(), Str::lit("defaultOpenContent"))
                        }
                    } {
                        Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                            Adt19::C0(h1743, _, _, _, _, _) => {
                                h1743
                            }
                        }, List::cons(f8(Str::lit(": xs:")), List::cons(match v3 {
                            Adt27::C0(_, h1744, _, _, _) => {
                                h1744
                            }
                        }, List::cons(f8(Str::lit(" is not translated, because it changes components that other schema documents define (XSD 1.1 Part 1, sections 4.2.4, 4.2.5 and 3.4.2.3)")), List::<Bytes>::nil())))))))
                    } else {
                        if if f380(v3.clone(), Str::lit("import")) {
                            true
                        } else {
                            f380(v3.clone(), Str::lit("include"))
                        } {
                            let m1745 = f381(v3.clone(), Str::lit("schemaLocation"));
                            match m1745 {
                                None => {
                                    Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                                        Adt19::C0(h1746, _, _, _, _, _) => {
                                            h1746
                                        }
                                    }, List::cons(f8(Str::lit(": an xs:")), List::cons(match v3 {
                                        Adt27::C0(_, h1747, _, _, _) => {
                                            h1747
                                        }
                                    }, List::cons(f8(Str::lit(" with no schemaLocation names no document whose components could be read (XSD 1.1 Part 1, section 4.2.6.2)")), List::<Bytes>::nil())))))))
                                }
                                Some(v7) => {
                                    let m1749 = f382(match v1.clone() {
                                        Adt19::C0(_, h1748, _, _, _, _) => {
                                            h1748
                                        }
                                    }, v7)?;
                                    match m1749 {
                                        Ok(v9) => {
                                            let m1751 = f383(v2, v9.clone());
                                            match m1751 {
                                                None => {
                                                    Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                                                        Adt19::C0(h1752, _, _, _, _, _) => {
                                                            h1752
                                                        }
                                                    }, List::cons(f8(Str::lit(": the schema document ")), List::cons(v9, List::cons(f8(Str::lit(" is not a bound authority, so its components cannot be read; stage its binding first (XSD 1.1 Part 1, section 4.2.6.2)")), List::<Bytes>::nil())))))))
                                                }
                                                Some(v10) => {
                                                    Ok(Ok::<List<(Bytes, (Bytes, Bytes))>, Bytes>(List::cons((v10, (match v3.clone() {
                                                        Adt27::C0(_, h1753, _, _, _) => {
                                                            h1753
                                                        }
                                                    }, if f380(v3.clone(), Str::lit("include")) {
                                                        match v1 {
                                                            Adt19::C0(_, _, h1754, _, _, _) => {
                                                                h1754
                                                            }
                                                        }
                                                    } else {
                                                        f384(v3, Str::lit("namespace"), Str::lit(""))
                                                    })), v6)))
                                                }
                                            }
                                        }
                                        Err(v8) => {
                                            Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f260(match v1 {
                                                Adt19::C0(h1750, _, _, _, _, _) => {
                                                    h1750
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

pub fn f266(v0: List<(Bytes, (Bytes, Bytes))>, v1: List<(Bytes, (Bytes, Bytes))>) -> List<(Bytes, (Bytes, Bytes))> {
    let m1755 = v0;
    match m1755.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f266(v3, v1))
        }
    }
}

pub fn f267(v0: List<(Bytes, Option<Bytes>)>, v1: List<(Bytes, Option<Bytes>)>) -> List<(Bytes, Option<Bytes>)> {
    let m1756 = v0;
    match m1756.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f267(v3, v1))
        }
    }
}

pub fn f268(v0: Adt2) -> Option<Bytes> {
    let m1757 = v0;
    match m1757 {
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
        Adt2::C7(r1760) => {
            let v3 = (*r1760).clone();
            f268(v3)
        }
        Adt2::C8(r1761) => {
            let v4 = (*r1761).clone();
            f268(v4)
        }
        Adt2::C9 => {
            None::<Bytes>
        }
        Adt2::C10(r1762, r1763) => {
            let v5 = (*r1762).clone();
            let v6 = (*r1763).clone();
            let m1758 = f268(v5);
            match m1758 {
                None => {
                    f268(v6)
                }
                Some(v7) => {
                    Some(v7)
                }
            }
        }
        Adt2::C11(_, r1764, r1765) => {
            let v9 = (*r1764).clone();
            let v10 = (*r1765).clone();
            let m1759 = f268(v9);
            match m1759 {
                None => {
                    f268(v10)
                }
                Some(v11) => {
                    Some(v11)
                }
            }
        }
    }
}

pub fn f269(v0: Adt2) -> R<Option<Bytes>> {
    let m1766 = v0;
    match m1766 {
        Adt2::C0 => {
            Ok(None::<Bytes>)
        }
        Adt2::C1(_, r1769) => {
            let v2 = (*r1769).clone();
            f269(v2)
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
            if f385(v18, v19)? {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(f288(v18, v19)?))
            }
        }
        Adt2::C6(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C7(r1770) => {
            let v3 = (*r1770).clone();
            f269(v3)
        }
        Adt2::C8(r1771) => {
            let v4 = (*r1771).clone();
            f269(v4)
        }
        Adt2::C9 => {
            Ok(None::<Bytes>)
        }
        Adt2::C10(r1772, r1773) => {
            let v5 = (*r1772).clone();
            let v6 = (*r1773).clone();
            let m1767 = f269(v5)?;
            match m1767 {
                None => {
                    f269(v6)
                }
                Some(v7) => {
                    Ok(Some(v7))
                }
            }
        }
        Adt2::C11(_, r1774, r1775) => {
            let v9 = (*r1774).clone();
            let v10 = (*r1775).clone();
            let m1768 = f269(v9)?;
            match m1768 {
                None => {
                    f269(v10)
                }
                Some(v11) => {
                    Ok(Some(v11))
                }
            }
        }
    }
}

pub fn f270(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f111(v0.clone(), {
        let a1776 = v1;
        let a1777 = 1u64;
        nat_add(a1776, a1777)?
    })?;
    let v4: Bytes = {
        let a1778 = v3;
        let a1779 = 117u64;
        if nat_eq(a1778, a1779) {
            f386(v0, v1)?
        } else {
            f387(v3)
        }
    };
    let a1780 = v4.clone();
    let a1781 = Bytes::lit(&[]);
    if equal(a1780, a1781) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a1782 = v3;
            let a1783 = 117u64;
            if nat_eq(a1782, a1783) {
                let a1784 = v1;
                let a1785 = 6u64;
                nat_add(a1784, a1785)?
            } else {
                let a1786 = v1;
                let a1787 = 2u64;
                nat_add(a1786, a1787)?
            }
        }, ({
            let a1788 = v2;
            let a1789 = v4;
            append_bytes(a1788, a1789)
        }, 0u64))))
    }
}

pub fn f271(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f388(v0, v1)
}

pub fn f272(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f389(v0, v1)
}

pub fn f273(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1790 = v0.clone();
        length_bytes(a1790)
    };
    let a1791 = 0u64;
    let a1792 = v1;
    if nat_lt(a1791, a1792) {
        let a1793 = f390(v0.clone())?;
        let a1794 = v1;
        if nat_eq(a1793, a1794) {
            let a1795 = 1u64;
            let a1796 = v1;
            if nat_lt(a1795, a1796) {
                let m1799 = {
                    let a1797 = v0;
                    let a1798 = 0u64;
                    index_bytes(a1797, a1798)
                };
                match m1799 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1800 = v2;
                        let a1801 = 48u8;
                        Ok(!equal(a1800, a1801))
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

pub fn f274(v0: Fn6, p1806: u64, p1807: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1805 = (p1806, p1807);
    {
        let mut m1805 = m1805;
        loop {
            let step = {
                let (v1, v2) = m1805;
                let m1802 = v1;
                if m1802 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1802 - 1;
                    let m1804 = {
                        let c1803: Fn6 = v0.clone();
                        c1803.apply(v2)?
                    };
                    match m1804 {
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
                Ok(next) => { m1805 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f275(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f391(v0, v1)
}

pub fn f276(v0: Bytes) -> R<u64> {
    let (h1808, _) = f204(Fn5::F392(v0.clone()), {
        let a1810 = {
            let a1809 = v0;
            length_bytes(a1809)
        };
        let a1811 = 1u64;
        nat_add(a1810, a1811)?
    }, 0u64)?;
    Ok(h1808)
}

pub fn f277(v0: u64) -> Bytes {
    let m1812 = v0;
    if m1812 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1812 - 1;
        let a1813 = f8(Str::lit("0"));
        let a1814 = f277(v1);
        append_bytes(a1813, a1814)
    }
}

pub fn f278(v0: u32) -> Bytes {
    let a1817 = {
        let a1815 = v0;
        let a1816 = 8u32;
        fixed_u32::bit_and(a1815, a1816)
    };
    let a1818 = 0u32;
    if equal(a1817, a1818) {
        let a1821 = {
            let a1819 = v0;
            let a1820 = 4u32;
            fixed_u32::bit_and(a1819, a1820)
        };
        let a1822 = 0u32;
        if equal(a1821, a1822) {
            let a1825 = {
                let a1823 = v0;
                let a1824 = 2u32;
                fixed_u32::bit_and(a1823, a1824)
            };
            let a1826 = 0u32;
            if equal(a1825, a1826) {
                let a1829 = {
                    let a1827 = v0;
                    let a1828 = 1u32;
                    fixed_u32::bit_and(a1827, a1828)
                };
                let a1830 = 0u32;
                if equal(a1829, a1830) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a1833 = {
                    let a1831 = v0;
                    let a1832 = 1u32;
                    fixed_u32::bit_and(a1831, a1832)
                };
                let a1834 = 0u32;
                if equal(a1833, a1834) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a1837 = {
                let a1835 = v0;
                let a1836 = 2u32;
                fixed_u32::bit_and(a1835, a1836)
            };
            let a1838 = 0u32;
            if equal(a1837, a1838) {
                let a1841 = {
                    let a1839 = v0;
                    let a1840 = 1u32;
                    fixed_u32::bit_and(a1839, a1840)
                };
                let a1842 = 0u32;
                if equal(a1841, a1842) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a1845 = {
                    let a1843 = v0;
                    let a1844 = 1u32;
                    fixed_u32::bit_and(a1843, a1844)
                };
                let a1846 = 0u32;
                if equal(a1845, a1846) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a1849 = {
            let a1847 = v0;
            let a1848 = 4u32;
            fixed_u32::bit_and(a1847, a1848)
        };
        let a1850 = 0u32;
        if equal(a1849, a1850) {
            let a1853 = {
                let a1851 = v0;
                let a1852 = 2u32;
                fixed_u32::bit_and(a1851, a1852)
            };
            let a1854 = 0u32;
            if equal(a1853, a1854) {
                let a1857 = {
                    let a1855 = v0;
                    let a1856 = 1u32;
                    fixed_u32::bit_and(a1855, a1856)
                };
                let a1858 = 0u32;
                if equal(a1857, a1858) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a1861 = {
                    let a1859 = v0;
                    let a1860 = 1u32;
                    fixed_u32::bit_and(a1859, a1860)
                };
                let a1862 = 0u32;
                if equal(a1861, a1862) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a1865 = {
                let a1863 = v0;
                let a1864 = 2u32;
                fixed_u32::bit_and(a1863, a1864)
            };
            let a1866 = 0u32;
            if equal(a1865, a1866) {
                let a1869 = {
                    let a1867 = v0;
                    let a1868 = 1u32;
                    fixed_u32::bit_and(a1867, a1868)
                };
                let a1870 = 0u32;
                if equal(a1869, a1870) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a1873 = {
                    let a1871 = v0;
                    let a1872 = 1u32;
                    fixed_u32::bit_and(a1871, a1872)
                };
                let a1874 = 0u32;
                if equal(a1873, a1874) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f279(v0: u32, v1: u32) -> u32 {
    let m1877 = {
        let a1875 = v0;
        let a1876 = v1;
        fixed_u32::shift_right(a1875, a1876)
    };
    match m1877 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f280(v0: Adt20, v1: Adt29) -> R<Adt20> {
    let v2: Adt30 = f393(Fn7::F394, 64u64, Adt30::C0(match v0.clone() {
        Adt20::C0(h1878, _, _, _, _, _, _, _) => {
            h1878
        }
    }, match v0.clone() {
        Adt20::C0(_, h1879, _, _, _, _, _, _) => {
            h1879
        }
    }, match v0.clone() {
        Adt20::C0(_, _, h1880, _, _, _, _, _) => {
            h1880
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, h1881, _, _, _, _) => {
            h1881
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, h1882, _, _, _) => {
            h1882
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, h1883, _, _) => {
            h1883
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h1884, _) => {
            h1884
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, _, h1885) => {
            h1885
        }
    }, v1, 0u64))?;
    Ok(Adt20::C0(f395(match v0.clone() {
        Adt20::C0(h1886, _, _, _, _, _, _, _) => {
            h1886
        }
    }, match v2.clone() {
        Adt30::C0(h1887, _, _, _, _, _, _, _, _, _) => {
            h1887
        }
    }), f395(match v0.clone() {
        Adt20::C0(_, h1888, _, _, _, _, _, _) => {
            h1888
        }
    }, match v2.clone() {
        Adt30::C0(_, h1889, _, _, _, _, _, _, _, _) => {
            h1889
        }
    }), f395(match v0.clone() {
        Adt20::C0(_, _, h1890, _, _, _, _, _) => {
            h1890
        }
    }, match v2.clone() {
        Adt30::C0(_, _, h1891, _, _, _, _, _, _, _) => {
            h1891
        }
    }), f395(match v0.clone() {
        Adt20::C0(_, _, _, h1892, _, _, _, _) => {
            h1892
        }
    }, match v2.clone() {
        Adt30::C0(_, _, _, h1893, _, _, _, _, _, _) => {
            h1893
        }
    }), f395(match v0.clone() {
        Adt20::C0(_, _, _, _, h1894, _, _, _) => {
            h1894
        }
    }, match v2.clone() {
        Adt30::C0(_, _, _, _, h1895, _, _, _, _, _) => {
            h1895
        }
    }), f395(match v0.clone() {
        Adt20::C0(_, _, _, _, _, h1896, _, _) => {
            h1896
        }
    }, match v2.clone() {
        Adt30::C0(_, _, _, _, _, h1897, _, _, _, _) => {
            h1897
        }
    }), f395(match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h1898, _) => {
            h1898
        }
    }, match v2.clone() {
        Adt30::C0(_, _, _, _, _, _, h1899, _, _, _) => {
            h1899
        }
    }), f395(match v0 {
        Adt20::C0(_, _, _, _, _, _, _, h1900) => {
            h1900
        }
    }, match v2 {
        Adt30::C0(_, _, _, _, _, _, _, h1901, _, _) => {
            h1901
        }
    })))
}

pub fn f281(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt29> {
    Ok(Adt29::C0(f396(v0.clone(), v1, v2, v3)?, f396(v0.clone(), v1, v2, {
        let a1902 = v3;
        let a1903 = 4u64;
        nat_add(a1902, a1903)?
    })?, f396(v0.clone(), v1, v2, {
        let a1904 = v3;
        let a1905 = 8u64;
        nat_add(a1904, a1905)?
    })?, f396(v0.clone(), v1, v2, {
        let a1906 = v3;
        let a1907 = 12u64;
        nat_add(a1906, a1907)?
    })?, f396(v0.clone(), v1, v2, {
        let a1908 = v3;
        let a1909 = 16u64;
        nat_add(a1908, a1909)?
    })?, f396(v0.clone(), v1, v2, {
        let a1910 = v3;
        let a1911 = 20u64;
        nat_add(a1910, a1911)?
    })?, f396(v0.clone(), v1, v2, {
        let a1912 = v3;
        let a1913 = 24u64;
        nat_add(a1912, a1913)?
    })?, f396(v0.clone(), v1, v2, {
        let a1914 = v3;
        let a1915 = 28u64;
        nat_add(a1914, a1915)?
    })?, f396(v0.clone(), v1, v2, {
        let a1916 = v3;
        let a1917 = 32u64;
        nat_add(a1916, a1917)?
    })?, f396(v0.clone(), v1, v2, {
        let a1918 = v3;
        let a1919 = 36u64;
        nat_add(a1918, a1919)?
    })?, f396(v0.clone(), v1, v2, {
        let a1920 = v3;
        let a1921 = 40u64;
        nat_add(a1920, a1921)?
    })?, f396(v0.clone(), v1, v2, {
        let a1922 = v3;
        let a1923 = 44u64;
        nat_add(a1922, a1923)?
    })?, f396(v0.clone(), v1, v2, {
        let a1924 = v3;
        let a1925 = 48u64;
        nat_add(a1924, a1925)?
    })?, f396(v0.clone(), v1, v2, {
        let a1926 = v3;
        let a1927 = 52u64;
        nat_add(a1926, a1927)?
    })?, f396(v0.clone(), v1, v2, {
        let a1928 = v3;
        let a1929 = 56u64;
        nat_add(a1928, a1929)?
    })?, f396(v0, v1, v2, {
        let a1930 = v3;
        let a1931 = 60u64;
        nat_add(a1930, a1931)?
    })?))
}

pub fn f282(v0: u64) -> bool {
    f183({
        let a1932 = v0;
        let a1933 = 95u64;
        nat_eq(a1932, a1933)
    }, f183({
        let a1934 = v0;
        let a1935 = 45u64;
        nat_eq(a1934, a1935)
    }, f183({
        let a1936 = v0;
        let a1937 = 32u64;
        nat_eq(a1936, a1937)
    }, f183({
        let a1938 = v0;
        let a1939 = 46u64;
        nat_eq(a1938, a1939)
    }, {
        let a1940 = v0;
        let a1941 = 47u64;
        nat_eq(a1940, a1941)
    }))))
}

pub fn f283(v0: u64) -> bool {
    f113({
        let a1942 = 97u64;
        let a1943 = v0;
        nat_le(a1942, a1943)
    }, {
        let a1944 = v0;
        let a1945 = 122u64;
        nat_le(a1944, a1945)
    })
}

pub fn f284(v0: u64) -> bool {
    f113({
        let a1946 = 65u64;
        let a1947 = v0;
        nat_le(a1946, a1947)
    }, {
        let a1948 = v0;
        let a1949 = 90u64;
        nat_le(a1948, a1949)
    })
}

pub fn f285(v0: u64) -> bool {
    f113({
        let a1950 = 48u64;
        let a1951 = v0;
        nat_le(a1950, a1951)
    }, {
        let a1952 = v0;
        let a1953 = 57u64;
        nat_le(a1952, a1953)
    })
}

pub fn f286(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1954 = 0u64;
        let a1955 = v1;
        nat_lt(a1954, a1955)
    } else {
        false
    } {
        let a1956 = Bytes::lit(&[0x2d]);
        let a1957 = f91(v1)?;
        Ok(append_bytes(a1956, a1957))
    } else {
        f91(v1)
    }
}

pub fn f287(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f91(v1)?;
    let v4: u64 = {
        let a1958 = v3.clone();
        length_bytes(a1958)
    };
    let v5: Bytes = {
        let a1959 = v2;
        let a1960 = v4;
        if nat_lt(a1959, a1960) {
            f7(List::cons(f46(v3.clone(), 0u64, {
                let a1961 = v4;
                let a1962 = v2;
                nat_sub(a1961, a1962)
            }), List::cons(f8(Str::lit(".")), List::cons(f46(v3, {
                let a1963 = v4;
                let a1964 = v2;
                nat_sub(a1963, a1964)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f277({
                let a1965 = v2;
                let a1966 = v4;
                nat_sub(a1965, a1966)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1967 = f8(Str::lit("-"));
        let a1968 = v5;
        Ok(append_bytes(a1967, a1968))
    } else {
        Ok(v5)
    }
}

pub fn f288(v0: bool, v1: u64) -> R<Bytes> {
    let v2: u64 = f397(v0)?;
    let v3: bool = {
        let a1969 = f398(v0)?;
        let a1970 = v1;
        nat_le(a1969, a1970)
    };
    let v4: u64 = {
        let a1974 = {
            let a1971 = v1;
            let a1972 = v2;
            let a1973 = 0u64;
            nat_quot(a1971, a1972, a1973)
        };
        let a1975 = f399(f400(v0))?;
        let a1976 = 0u64;
        nat_rem(a1974, a1975, a1976)
    };
    let v5: u64 = {
        let a1977 = v1;
        let a1978 = v2;
        let a1979 = 0u64;
        nat_rem(a1977, a1978, a1979)
    };
    let a1980 = v4;
    let a1981 = f401(v0)?;
    if nat_eq(a1980, a1981) {
        let a1982 = v5;
        let a1983 = 0u64;
        if nat_eq(a1982, a1983) {
            Ok(f402(v3, f8(Str::lit("inf"))))
        } else {
            Ok(f402(v3, {
                let a1987 = {
                    let a1984 = v2;
                    let a1985 = 2u64;
                    let a1986 = 0u64;
                    nat_quot(a1984, a1985, a1986)
                };
                let a1988 = v5;
                if nat_le(a1987, a1988) {
                    f8(Str::lit("nan"))
                } else {
                    f8(Str::lit("snan"))
                }
            }))
        }
    } else {
        if f113({
            let a1989 = v4;
            let a1990 = 0u64;
            nat_eq(a1989, a1990)
        }, {
            let a1991 = v5;
            let a1992 = 0u64;
            nat_eq(a1991, a1992)
        }) {
            Ok(f402(v3, Bytes::lit(&[0x30])))
        } else {
            let v6: u64 = {
                let a1993 = v4;
                let a1994 = 0u64;
                if nat_eq(a1993, a1994) {
                    v5
                } else {
                    let a1995 = v2;
                    let a1996 = v5;
                    nat_add(a1995, a1996)?
                }
            };
            let v7: u64 = {
                let a1997 = v4;
                let a1998 = 0u64;
                if nat_eq(a1997, a1998) {
                    1u64
                } else {
                    v4
                }
            };
            let v8: u64 = {
                let a2005 = {
                    let a2001 = {
                        let a1999 = v7;
                        let a2000 = 2000u64;
                        nat_add(a1999, a2000)?
                    };
                    let a2002 = f403(v0);
                    nat_sub(a2001, a2002)
                };
                let a2006 = {
                    let a2003 = f404(v0);
                    let a2004 = 1u64;
                    nat_sub(a2003, a2004)
                };
                nat_sub(a2005, a2006)
            };
            Ok(f402(v3, {
                let a2007 = 2000u64;
                let a2008 = v8;
                if nat_le(a2007, a2008) {
                    f405(v6, true, {
                        let a2009 = v8;
                        let a2010 = 2000u64;
                        nat_sub(a2009, a2010)
                    })?
                } else {
                    f405(v6, false, {
                        let a2011 = 2000u64;
                        let a2012 = v8;
                        nat_sub(a2011, a2012)
                    })?
                }
            }))
        }
    }
}

pub fn f289(v0: Bytes) -> R<Bytes> {
    let a2017 = f180(v0.clone(), 0u64, {
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
            let a2023 = f406(v0.clone(), 0u64, {
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

pub fn f290(v0: List<Bytes>) -> Bytes {
    let a2028 = {
        let a2026 = Bytes::lit(&[0x5b]);
        let a2027 = f47(v0, Bytes::lit(&[0x2c]));
        append_bytes(a2026, a2027)
    };
    let a2029 = Bytes::lit(&[0x5d]);
    append_bytes(a2028, a2029)
}

pub fn f291(v0: Adt2) -> R<List<Bytes>> {
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
        Adt2::C10(r2031, r2032) => {
            let v14 = (*r2031).clone();
            let v15 = (*r2032).clone();
            Ok(List::cons(f196(v14)?, f291(v15)?))
        }
        Adt2::C11(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f292(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f407(Fn8::F408, List::<(Bytes, Bytes)>::nil(), f409(v0)?);
    let v2: List<Bytes> = f410(Fn9::F411, List::<Bytes>::nil(), v1)?;
    let a2035 = {
        let a2033 = Bytes::lit(&[0x7b]);
        let a2034 = f47(v2, Bytes::lit(&[0x2c]));
        append_bytes(a2033, a2034)
    };
    let a2036 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a2035, a2036))
}

pub fn f293(v0: Adt2) -> R<List<(Bytes, Bytes)>> {
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
        Adt2::C11(v16, r2038, r2039) => {
            let v17 = (*r2038).clone();
            let v18 = (*r2039).clone();
            Ok(List::cons((v16, f196(v17)?), f293(v18)?))
        }
    }
}

pub fn f294(v0: List<Bytes>) -> R<List<Bytes>> {
    let m2040 = v0;
    match m2040.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f412(v1)?, f294(v2)?))
        }
    }
}

pub fn f295(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m2041 = v0;
    match m2041.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f295(v3, false))
            } else {
                f295(v3, true)
            }
        }
    }
}

pub fn f296(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f413(v0, v1)
}

pub fn f297(v0: List<Bytes>) -> List<Bytes> {
    f116(Fn2::F414, List::<Bytes>::nil(), v0)
}

pub fn f298(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f415(v0, v1)
}

pub fn f299(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f416(f417(f418(v0))?))
}

pub fn f300(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f133(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a2044 = {
            let a2042 = v4;
            let a2043 = v1;
            nat_add(a2042, a2043)?
        };
        let a2045 = v3;
        if nat_lt(a2044, a2045) {
            Ok(Some({
                let a2046 = v4;
                let a2047 = 1u64;
                nat_add(a2046, a2047)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f301(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h2048, _) = v1;
        h2048
    };
    let a2050 = {
        let a2049 = v0.clone();
        length_bytes(a2049)
    };
    let a2051 = v2;
    if nat_le(a2050, a2051) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let a2052 = f163(v0.clone(), v2)?;
        let a2053 = 13u64;
        if nat_eq(a2052, a2053) {
            Ok(Some(({
                let a2056 = f163(v0, {
                    let a2054 = v2;
                    let a2055 = 1u64;
                    nat_add(a2054, a2055)?
                })?;
                let a2057 = 10u64;
                if nat_eq(a2056, a2057) {
                    let a2058 = v2;
                    let a2059 = 2u64;
                    nat_add(a2058, a2059)?
                } else {
                    let a2060 = v2;
                    let a2061 = 1u64;
                    nat_add(a2060, a2061)?
                }
            }, {
                let a2063 = {
                    let (_, h2062) = v1;
                    h2062
                };
                let a2064 = f8(Str::lit("\u{a}"));
                append_bytes(a2063, a2064)
            })))
        } else {
            let v4: u64 = {
                let m2065 = f234(v0.clone(), f8(Str::lit("\u{d}")), v2)?;
                match m2065 {
                    None => {
                        let a2066 = v0.clone();
                        length_bytes(a2066)
                    }
                    Some(v3) => {
                        v3
                    }
                }
            };
            Ok(Some((v4, {
                let a2070 = {
                    let (_, h2067) = v1;
                    h2067
                };
                let a2071 = f46(v0, v2, {
                    let a2068 = v4;
                    let a2069 = v2;
                    nat_sub(a2068, a2069)
                });
                append_bytes(a2070, a2071)
            })))
        }
    }
}

pub fn f302(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2072 = f419(v0, v1)?;
    match m2072 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f420({
                let (h2073, _) = v2;
                h2073
            }) {
                Ok(Some({
                    let a2075 = v1;
                    let a2076 = {
                        let (_, h2074) = v2;
                        h2074
                    };
                    nat_add(a2075, a2076)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f303(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2078 = v1;
    let a2079 = {
        let a2077 = v0.clone();
        length_bytes(a2077)
    };
    if if nat_lt(a2078, a2079) {
        f421(v0.clone(), v1)?
    } else {
        false
    } {
        Ok(Some(f308(v0, v1)?))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f304(v0: u64) -> bool {
    let a2080 = v0;
    let a2081 = 32u64;
    if nat_eq(a2080, a2081) {
        true
    } else {
        let a2082 = v0;
        let a2083 = 9u64;
        nat_eq(a2082, a2083)
    }
}

pub fn f305(v0: u64) -> bool {
    let a2084 = v0;
    let a2085 = 10u64;
    if nat_eq(a2084, a2085) {
        true
    } else {
        let a2086 = v0;
        let a2087 = 256u64;
        nat_eq(a2086, a2087)
    }
}

pub fn f306(v0: Bytes, v1: u64) -> R<u64> {
    let a2088 = v1;
    let a2089 = f422(v0, v1)?;
    Ok(nat_sub(a2088, a2089))
}

pub fn f307(v0: Bytes, v1: u64) -> R<u64> {
    let (h2090, _) = f204(Fn5::F423(v0.clone()), {
        let a2092 = {
            let a2091 = v0;
            length_bytes(a2091)
        };
        let a2093 = 1u64;
        nat_add(a2092, a2093)?
    }, v1)?;
    Ok(h2090)
}

pub fn f308(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f424(v0.clone(), v1)?;
    let a2095 = v2;
    let a2096 = {
        let a2094 = v0;
        length_bytes(a2094)
    };
    if nat_lt(a2095, a2096) {
        let a2097 = v2;
        let a2098 = 1u64;
        nat_add(a2097, a2098)
    } else {
        Ok(v2)
    }
}

pub fn f309(v0: Bytes, v1: u64) -> R<u64> {
    let (h2099, _) = f204(Fn5::F425(v0.clone()), {
        let a2101 = {
            let a2100 = v0;
            length_bytes(a2100)
        };
        let a2102 = 1u64;
        nat_add(a2101, a2102)?
    }, v1)?;
    Ok(h2099)
}

pub fn f310(v0: Bytes, v1: u64) -> R<bool> {
    let a2103 = f209(v0.clone(), v1)?;
    let a2104 = 45u64;
    if nat_eq(a2103, a2104) {
        if f304(f209(v0.clone(), {
            let a2105 = v1;
            let a2106 = 1u64;
            nat_add(a2105, a2106)?
        })?) {
            Ok(true)
        } else {
            Ok(f305(f209(v0, {
                let a2107 = v1;
                let a2108 = 1u64;
                nat_add(a2107, a2108)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f311(v0: Bytes, v1: u64) -> R<bool> {
    let m2109 = f323(v0, v1)?;
    match m2109 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f312(v0: Bytes, v1: u64) -> R<Option<Bytes>> {
    let v2: u64 = f209(v0.clone(), v1)?;
    let v3: bool = f211(f426(f209(v0, {
        let a2110 = v1;
        let a2111 = 1u64;
        nat_add(a2110, a2111)?
    })?, true));
    let a2112 = v2;
    let a2113 = 58u64;
    if if nat_eq(a2112, a2113) {
        v3
    } else {
        false
    } {
        Ok(Some(f137(Str::lit("a null key is not read: Studio.Value keys are strings (YAML 1.2.2 section 10.3.2)"), v1)?))
    } else {
        let a2114 = v2;
        let a2115 = 38u64;
        if nat_eq(a2114, a2115) {
            Ok(Some(f137(Str::lit("an anchor is not read (YAML 1.2.2 [101])"), v1)?))
        } else {
            let a2116 = v2;
            let a2117 = 42u64;
            if nat_eq(a2116, a2117) {
                Ok(Some(f137(Str::lit("an alias is not read (YAML 1.2.2 [104])"), v1)?))
            } else {
                let a2118 = v2;
                let a2119 = 33u64;
                if nat_eq(a2118, a2119) {
                    Ok(Some(f137(Str::lit("a tag is not read (YAML 1.2.2 [97])"), v1)?))
                } else {
                    let a2120 = v2;
                    let a2121 = 63u64;
                    if if nat_eq(a2120, a2121) {
                        v3
                    } else {
                        false
                    } {
                        Ok(Some(f137(Str::lit("an explicit key is not read (YAML 1.2.2 [143], [189])"), v1)?))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f313(v0: Bytes, v1: u64, v2: u64, v3: Adt21) -> R<Adt22> {
    let v4: u64 = f209(v0.clone(), v1)?;
    let a2122 = v4;
    let a2123 = 124u64;
    if if nat_eq(a2122, a2123) {
        true
    } else {
        let a2124 = v4;
        let a2125 = 62u64;
        nat_eq(a2124, a2125)
    } {
        let m2126 = f427(v0, v1, v2)?;
        match m2126 {
            Adt22::C0(v6, v7) => {
                let m2127 = f74(Some(v6.clone()));
                match m2127 {
                    None => {
                        Ok(Adt22::C0(v6, v7))
                    }
                    Some(v8) => {
                        f428(v3, v8, true, v1, v7)
                    }
                }
            }
            Adt22::C1(v5) => {
                Ok(Adt22::C1(v5))
            }
        }
    } else {
        let a2128 = v4;
        let a2129 = 34u64;
        if if nat_eq(a2128, a2129) {
            true
        } else {
            let a2130 = v4;
            let a2131 = 39u64;
            nat_eq(a2130, a2131)
        } {
            let m2132 = f429(v0.clone(), v1, v2, false)?;
            match m2132 {
                Adt32::C0(v10, v11) => {
                    let m2133 = f208(v0, v11)?;
                    match m2133 {
                        Ok(v13) => {
                            f428(v3, v10, true, v1, v13)
                        }
                        Err(v12) => {
                            Ok(Adt22::C1(v12))
                        }
                    }
                }
                Adt32::C1(v9) => {
                    Ok(Adt22::C1(v9))
                }
            }
        } else {
            let a2134 = v4;
            let a2135 = 91u64;
            if if nat_eq(a2134, a2135) {
                true
            } else {
                let a2136 = v4;
                let a2137 = 123u64;
                nat_eq(a2136, a2137)
            } {
                let m2141 = f430(v0.clone(), v1, v2, {
                    let a2139 = {
                        let a2138 = v0.clone();
                        length_bytes(a2138)
                    };
                    let a2140 = 2u64;
                    nat_add(a2139, a2140)?
                }, 0u64, List::<Bytes>::nil(), v3)?;
                match m2141 {
                    Adt22::C0(v15, v16) => {
                        let a2142 = f209(v0.clone(), f307(v0.clone(), v16)?)?;
                        let a2143 = 58u64;
                        if nat_eq(a2142, a2143) {
                            Ok(Adt22::C1(f137(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [154])"), v1)?))
                        } else {
                            let m2144 = f208(v0, v16)?;
                            match m2144 {
                                Ok(v18) => {
                                    Ok(Adt22::C0(v15, v18))
                                }
                                Err(v17) => {
                                    Ok(Adt22::C1(v17))
                                }
                            }
                        }
                    }
                    Adt22::C1(v14) => {
                        Ok(Adt22::C1(v14))
                    }
                }
            } else {
                let m2145 = f312(v0.clone(), v1)?;
                match m2145 {
                    None => {
                        if f211(f431(v0.clone(), v1, false)?) {
                            Ok(Adt22::C1(f137(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                        } else {
                            let v20: u64 = f432(v0.clone(), v1, false)?;
                            let m2149 = f433(v0.clone(), v20, v2, {
                                let a2147 = {
                                    let a2146 = v0.clone();
                                    length_bytes(a2146)
                                };
                                let a2148 = 2u64;
                                nat_add(a2147, a2148)?
                            }, f321(v0, v1, v20))?;
                            match m2149 {
                                Adt32::C0(v22, v23) => {
                                    f428(v3, v22, false, v1, v23)
                                }
                                Adt32::C1(v21) => {
                                    Ok(Adt22::C1(v21))
                                }
                            }
                        }
                    }
                    Some(v19) => {
                        Ok(Adt22::C1(v19))
                    }
                }
            }
        }
    }
}

pub fn f314(v0: Adt21) -> bool {
    let m2150 = v0;
    match m2150 {
        Adt21::C0(v2) => {
            let a2151 = 0u64;
            let a2152 = v2;
            nat_lt(a2151, a2152)
        }
        Adt21::C1(_) => {
            true
        }
        Adt21::C2(_) => {
            true
        }
    }
}

pub fn f315(v0: Bytes, v1: u64) -> R<Adt21> {
    let v2: u64 = f434(v0.clone(), v1)?;
    let a2155 = f209(v0.clone(), {
        let a2153 = v1;
        let a2154 = 1u64;
        nat_add(a2153, a2154)?
    })?;
    let a2156 = 60u64;
    if nat_eq(a2155, a2156) {
        let a2157 = f209(v0.clone(), v2)?;
        let a2158 = 62u64;
        if nat_eq(a2157, a2158) {
            let v3: Bytes = f321(v0, {
                let a2159 = v1;
                let a2160 = 2u64;
                nat_add(a2159, a2160)?
            }, v2);
            if f162(v3.clone(), 0u64, Str::lit("tag:yaml.org,2002:")) {
                f435(f321(v3.clone(), 18u64, {
                    let a2161 = v3;
                    length_bytes(a2161)
                }), v1)
            } else {
                Ok(Adt21::C1(v3))
            }
        } else {
            Ok(Adt21::C2(f137(Str::lit("a verbatim tag must end with > (YAML 1.2.2 [98])"), v1)?))
        }
    } else {
        let a2164 = v2;
        let a2165 = {
            let a2162 = v1;
            let a2163 = 1u64;
            nat_add(a2162, a2163)?
        };
        if nat_eq(a2164, a2165) {
            Ok(Adt21::C0(8u64))
        } else {
            let a2168 = f209(v0.clone(), {
                let a2166 = v1;
                let a2167 = 1u64;
                nat_add(a2166, a2167)?
            })?;
            let a2169 = 33u64;
            if nat_eq(a2168, a2169) {
                f435(f321(v0, {
                    let a2170 = v1;
                    let a2171 = 2u64;
                    nat_add(a2170, a2171)?
                }, v2), v1)
            } else {
                let a2178 = f320(f321(v0.clone(), {
                    let a2172 = v1;
                    let a2173 = 1u64;
                    nat_add(a2172, a2173)?
                }, v2), 33u64)?;
                let a2179 = {
                    let a2176 = v2;
                    let a2177 = {
                        let a2174 = v1;
                        let a2175 = 1u64;
                        nat_add(a2174, a2175)?
                    };
                    nat_sub(a2176, a2177)
                };
                if nat_lt(a2178, a2179) {
                    Ok(Adt21::C2(f137(Str::lit("a named tag handle is not read: it needs a TAG directive (YAML 1.2.2 [92], [82])"), v1)?))
                } else {
                    Ok(Adt21::C1(f321(v0, v1, v2)))
                }
            }
        }
    }
}

pub fn f316(v0: Adt21) -> Option<Bytes> {
    let m2180 = v0;
    match m2180 {
        Adt21::C0(_) => {
            None::<Bytes>
        }
        Adt21::C1(_) => {
            None::<Bytes>
        }
        Adt21::C2(v1) => {
            Some(v1)
        }
    }
}

pub fn f317(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f434(v0.clone(), v1)?;
    let a2183 = f209(v0.clone(), {
        let a2181 = v1;
        let a2182 = 1u64;
        nat_add(a2181, a2182)?
    })?;
    let a2184 = 60u64;
    if if nat_eq(a2183, a2184) {
        let a2185 = f209(v0, v2)?;
        let a2186 = 62u64;
        nat_eq(a2185, a2186)
    } else {
        false
    } {
        let a2187 = v2;
        let a2188 = 1u64;
        nat_add(a2187, a2188)
    } else {
        Ok(v2)
    }
}

pub fn f318(v0: Adt21, v1: bool, v2: u64, v3: Adt22) -> R<Adt22> {
    let m2189 = v3;
    match m2189 {
        Adt22::C0(v5, v6) => {
            let v7: Adt2 = if v1 {
                Adt2::C7(std::rc::Rc::new(v5))
            } else {
                Adt2::C8(std::rc::Rc::new(v5))
            };
            let m2190 = v0;
            match m2190 {
                Adt21::C0(v10) => {
                    let a2191 = v10;
                    let a2192 = 0u64;
                    if if if nat_eq(a2191, a2192) {
                        true
                    } else {
                        let a2193 = v10;
                        let a2194 = 8u64;
                        nat_eq(a2193, a2194)
                    } {
                        true
                    } else {
                        let a2195 = v10;
                        let a2196 = if v1 {
                            6u64
                        } else {
                            7u64
                        };
                        nat_eq(a2195, a2196)
                    } {
                        Ok(Adt22::C0(v7, v6))
                    } else {
                        Ok(Adt22::C1(f436(v2)?))
                    }
                }
                Adt21::C1(v9) => {
                    Ok(Adt22::C0(Adt2::C1(v9, std::rc::Rc::new(v7)), v6))
                }
                Adt21::C2(v8) => {
                    Ok(Adt22::C1(v8))
                }
            }
        }
        Adt22::C1(v4) => {
            Ok(Adt22::C1(v4))
        }
    }
}

pub fn f319(v0: Adt21, v1: u64, v2: u64) -> R<Adt22> {
    let m2197 = v0;
    match m2197 {
        Adt21::C0(v5) => {
            let a2198 = v5;
            let a2199 = 1u64;
            if nat_eq(a2198, a2199) {
                Ok(Adt22::C0(Adt2::C6(Bytes::lit(&[])), v2))
            } else {
                let a2200 = v5;
                let a2201 = 8u64;
                if nat_eq(a2200, a2201) {
                    Ok(Adt22::C1(f137(Str::lit("an empty node with the non-specific tag ! is not read (YAML 1.2.2 [100])"), v1)?))
                } else {
                    let a2202 = v5;
                    let a2203 = 0u64;
                    if if nat_eq(a2202, a2203) {
                        true
                    } else {
                        let a2204 = v5;
                        let a2205 = 5u64;
                        nat_eq(a2204, a2205)
                    } {
                        Ok(Adt22::C0(Adt2::C0, v2))
                    } else {
                        Ok(Adt22::C1(f436(v1)?))
                    }
                }
            }
        }
        Adt21::C1(v4) => {
            Ok(Adt22::C0(Adt2::C1(v4, std::rc::Rc::new(Adt2::C0)), v2))
        }
        Adt21::C2(v3) => {
            Ok(Adt22::C1(v3))
        }
    }
}

pub fn f320(v0: Bytes, v1: u64) -> R<u64> {
    let (h2206, _) = f204(Fn5::F437(v0.clone(), v1), {
        let a2208 = {
            let a2207 = v0;
            length_bytes(a2207)
        };
        let a2209 = 1u64;
        nat_add(a2208, a2209)?
    }, 0u64)?;
    Ok(h2206)
}

pub fn f321(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    f46(v0, v1, {
        let a2210 = v2;
        let a2211 = v1;
        nat_sub(a2210, a2211)
    })
}

pub fn f322(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let a2212 = f209(v0.clone(), v1)?;
    let a2213 = 33u64;
    if nat_eq(a2212, a2213) {
        let m2214 = f315(v0.clone(), v1)?;
        match m2214 {
            Adt21::C0(v4) => {
                let a2215 = v4;
                let a2216 = 1u64;
                if if nat_eq(a2215, a2216) {
                    true
                } else {
                    let a2217 = v4;
                    let a2218 = 8u64;
                    nat_eq(a2217, a2218)
                } {
                    Ok(Ok::<u64, Bytes>(f307(v0.clone(), f317(v0, v1)?)?))
                } else {
                    Ok(Err::<u64, Bytes>(f137(Str::lit("a mapping key tagged other than !!str is not read: Studio.Value keys are strings (YAML 1.2.2 section 10.3.2)"), v1)?))
                }
            }
            Adt21::C1(_) => {
                Ok(Err::<u64, Bytes>(f137(Str::lit("a mapping key tagged other than !!str is not read: Studio.Value keys are strings (YAML 1.2.2 section 10.3.2)"), v1)?))
            }
            Adt21::C2(v2) => {
                Ok(Err::<u64, Bytes>(v2))
            }
        }
    } else {
        Ok(Ok::<u64, Bytes>(v1))
    }
}

pub fn f323(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a2219 = f209(v0.clone(), v1)?;
        let a2220 = 33u64;
        if nat_eq(a2219, a2220) {
            f307(v0.clone(), f317(v0.clone(), v1)?)?
        } else {
            v1
        }
    };
    let v3: u64 = f209(v0.clone(), v2)?;
    let a2221 = v3;
    let a2222 = 34u64;
    if if nat_eq(a2221, a2222) {
        true
    } else {
        let a2223 = v3;
        let a2224 = 39u64;
        nat_eq(a2223, a2224)
    } {
        let m2225 = f429(v0.clone(), v2, 0u64, true)?;
        match m2225 {
            Adt32::C0(_, v6) => {
                let v7: u64 = f307(v0.clone(), v6)?;
                if f438(v0, v7)? {
                    Ok(Some(v7))
                } else {
                    Ok(None::<u64>)
                }
            }
            Adt32::C1(_) => {
                Ok(None::<u64>)
            }
        }
    } else {
        if f431(v0.clone(), v2, false)? {
            let v8: u64 = f307(v0.clone(), f432(v0.clone(), v2, false)?)?;
            if f438(v0, v8)? {
                Ok(Some(v8))
            } else {
                Ok(None::<u64>)
            }
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f324(v0: Bytes, v1: u64) -> R<Bytes> {
    let v2: u64 = f209(v0.clone(), v1)?;
    let a2226 = v2;
    let a2227 = 34u64;
    if if nat_eq(a2226, a2227) {
        true
    } else {
        let a2228 = v2;
        let a2229 = 39u64;
        nat_eq(a2228, a2229)
    } {
        let m2230 = f429(v0, v1, 0u64, true)?;
        match m2230 {
            Adt32::C0(v4, _) => {
                Ok(v4)
            }
            Adt32::C1(_) => {
                Ok(Bytes::lit(&[]))
            }
        }
    } else {
        Ok(f321(v0.clone(), v1, f432(v0, v1, false)?))
    }
}

pub fn f325(v0: Bytes, p2233: List<Bytes>) -> bool {
    let m2232 = p2233;
    {
        let mut m2232 = m2232;
        loop {
            let step = {
                let v1 = m2232;
                let m2231 = v1;
                match m2231.uncons() {
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
                Ok(next) => { m2232 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f326(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2234, _) = v2;
        h2234
    };
    let a2235 = v3;
    let a2236 = 0u64;
    if nat_eq(a2235, a2236) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2237 = v3;
            let a2238 = v1;
            let a2239 = 0u64;
            nat_quot(a2237, a2238, a2239)
        }, {
            let a2244 = f212(v0, {
                let a2240 = v3;
                let a2241 = v1;
                let a2242 = 0u64;
                nat_rem(a2240, a2241, a2242)
            });
            let a2245 = {
                let (_, h2243) = v2;
                h2243
            };
            append_bytes(a2244, a2245)
        }))
    }
}

pub fn f327(p2300: u64, v1: Bytes, p2301: u64, p2302: List<Adt23>) -> R<Result<List<Adt23>, Bytes>> {
    let m2299 = (p2300, (p2301, p2302));
    {
        let mut m2299 = m2299;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2299;
                let m2246 = v0;
                if m2246 == 0 {
                    Err::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>(Ok(Err::<List<Adt23>, Bytes>(f439(Str::lit("the input has more tokens than bytes"), v2)?)))
                } else {
                    let v4 = m2246 - 1;
                    let v5: u64 = f163(v1.clone(), v2)?;
                    let a2247 = v5;
                    let a2248 = 256u64;
                    if nat_eq(a2247, a2248) {
                        Err::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>(Ok(Ok::<List<Adt23>, Bytes>(f440(v3, List::<Adt23>::nil()))))
                    } else {
                        if f233(v5)? {
                            Ok::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>((v4, ({
                                let a2249 = v2;
                                let a2250 = 1u64;
                                nat_add(a2249, a2250)?
                            }, v3)))
                        } else {
                            if f162(v1.clone(), v2, Str::lit("//")) {
                                let m2251 = f234(v1.clone(), Bytes::lit(&[0x0a]), v2)?;
                                match m2251 {
                                    None => {
                                        Err::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>(Ok(Ok::<List<Adt23>, Bytes>(f440(v3, List::<Adt23>::nil()))))
                                    }
                                    Some(v6) => {
                                        Ok::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>((v4, ({
                                            let a2252 = v6;
                                            let a2253 = 1u64;
                                            nat_add(a2252, a2253)?
                                        }, v3)))
                                    }
                                }
                            } else {
                                if f162(v1.clone(), v2, Str::lit("/*")) {
                                    let m2257 = f234(v1.clone(), {
                                        let a2254 = Str::lit("*/");
                                        utf8_encode(a2254)
                                    }, {
                                        let a2255 = v2;
                                        let a2256 = 2u64;
                                        nat_add(a2255, a2256)?
                                    })?;
                                    match m2257 {
                                        None => {
                                            Err::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>(Ok(Err::<List<Adt23>, Bytes>(f439(Str::lit("a comment is not closed (Web IDL, IDL grammar: comment)"), v2)?)))
                                        }
                                        Some(v7) => {
                                            Ok::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>((v4, ({
                                                let a2258 = v7;
                                                let a2259 = 2u64;
                                                nat_add(a2258, a2259)?
                                            }, v3)))
                                        }
                                    }
                                } else {
                                    let a2260 = v5;
                                    let a2261 = 34u64;
                                    if nat_eq(a2260, a2261) {
                                        let m2264 = f234(v1.clone(), Bytes::lit(&[0x22]), {
                                            let a2262 = v2;
                                            let a2263 = 1u64;
                                            nat_add(a2262, a2263)?
                                        })?;
                                        match m2264 {
                                            None => {
                                                Err::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>(Ok(Err::<List<Adt23>, Bytes>(f439(Str::lit("a string is not closed (Web IDL, IDL grammar: string)"), v2)?)))
                                            }
                                            Some(v8) => {
                                                Ok::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>((v4, ({
                                                    let a2265 = v8;
                                                    let a2266 = 1u64;
                                                    nat_add(a2265, a2266)?
                                                }, List::cons(Adt23::C0(3u64, f46(v1.clone(), {
                                                    let a2267 = v2;
                                                    let a2268 = 1u64;
                                                    nat_add(a2267, a2268)?
                                                }, {
                                                    let a2271 = v8;
                                                    let a2272 = {
                                                        let a2269 = v2;
                                                        let a2270 = 1u64;
                                                        nat_add(a2269, a2270)?
                                                    };
                                                    nat_sub(a2271, a2272)
                                                })), v3))))
                                            }
                                        }
                                    } else {
                                        let v9: u64 = {
                                            let a2273 = v5;
                                            let a2274 = 95u64;
                                            if if nat_eq(a2273, a2274) {
                                                true
                                            } else {
                                                let a2275 = v5;
                                                let a2276 = 45u64;
                                                nat_eq(a2275, a2276)
                                            } {
                                                let a2277 = v2;
                                                let a2278 = 1u64;
                                                nat_add(a2277, a2278)?
                                            } else {
                                                v2
                                            }
                                        };
                                        if f441(f163(v1.clone(), v9)?) {
                                            let v10: u64 = f442(v1.clone(), 3u64, {
                                                let a2279 = v9;
                                                let a2280 = 1u64;
                                                nat_add(a2279, a2280)?
                                            })?;
                                            Ok::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>((v4, (v10, List::cons(Adt23::C0(0u64, f46(v1.clone(), v2, {
                                                let a2281 = v10;
                                                let a2282 = v2;
                                                nat_sub(a2281, a2282)
                                            })), v3))))
                                        } else {
                                            let v11: u64 = {
                                                let a2283 = v5;
                                                let a2284 = 45u64;
                                                if nat_eq(a2283, a2284) {
                                                    let a2285 = v2;
                                                    let a2286 = 1u64;
                                                    nat_add(a2285, a2286)?
                                                } else {
                                                    v2
                                                }
                                            };
                                            let v12: u64 = f443(v1.clone(), v11)?;
                                            let v13: u64 = f444(v1.clone(), v11)?;
                                            let a2287 = v12;
                                            let a2288 = v13;
                                            if nat_lt(a2287, a2288) {
                                                Ok::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>((v4, (v13, List::cons(Adt23::C0(2u64, f46(v1.clone(), v2, {
                                                    let a2289 = v13;
                                                    let a2290 = v2;
                                                    nat_sub(a2289, a2290)
                                                })), v3))))
                                            } else {
                                                let a2291 = v11;
                                                let a2292 = v12;
                                                if nat_lt(a2291, a2292) {
                                                    Ok::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>((v4, (v12, List::cons(Adt23::C0(1u64, f46(v1.clone(), v2, {
                                                        let a2293 = v12;
                                                        let a2294 = v2;
                                                        nat_sub(a2293, a2294)
                                                    })), v3))))
                                                } else {
                                                    if f162(v1.clone(), v2, Str::lit("...")) {
                                                        Ok::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>((v4, ({
                                                            let a2295 = v2;
                                                            let a2296 = 3u64;
                                                            nat_add(a2295, a2296)?
                                                        }, List::cons(Adt23::C0(4u64, f8(Str::lit("..."))), v3))))
                                                    } else {
                                                        Ok::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>((v4, ({
                                                            let a2297 = v2;
                                                            let a2298 = 1u64;
                                                            nat_add(a2297, a2298)?
                                                        }, List::cons(Adt23::C0(4u64, f46(v1.clone(), v2, 1u64)), v3))))
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
                Ok(next) => { m2299 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f328(p2305: List<Adt13>, p2306: List<Adt13>) -> List<Adt13> {
    let m2304 = (p2305, p2306);
    {
        let mut m2304 = m2304;
        loop {
            let step = {
                let (v0, v1) = m2304;
                let m2303 = v0;
                match m2303.uncons() {
                    None => {
                        Err::<(List<Adt13>, List<Adt13>), List<Adt13>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt13>, List<Adt13>), List<Adt13>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2304 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f329(v0: List<Adt23>) -> R<Result<List<Adt23>, Bytes>> {
    if f445(v0.clone(), Str::lit("[")) {
        Ok(f446({
            let a2308 = {
                let a2307 = v0.clone();
                length_list(a2307)
            };
            let a2309 = 1u64;
            nat_add(a2308, a2309)?
        }, f447(v0), List::cons(f8(Str::lit("]")), List::<Bytes>::nil()), false))
    } else {
        Ok(Ok::<List<Adt23>, Bytes>(v0))
    }
}

pub fn f330(v0: u64, v1: List<Adt23>) -> R<Result<(Adt13, List<Adt23>), Bytes>> {
    if f445(v1.clone(), Str::lit("interface")) {
        if f445(f447(v1.clone()), Str::lit("mixin")) {
            f448(v0, 1u64, f447(f447(v1)), false)
        } else {
            f448(v0, 0u64, f447(v1), true)
        }
    } else {
        if f445(v1.clone(), Str::lit("partial")) {
            let v2: List<Adt23> = f447(v1);
            if f445(v2.clone(), Str::lit("interface")) {
                if f445(f447(v2.clone()), Str::lit("mixin")) {
                    f448(v0, 6u64, f447(f447(v2)), false)
                } else {
                    f448(v0, 5u64, f447(v2), false)
                }
            } else {
                if f445(v2.clone(), Str::lit("dictionary")) {
                    f448(v0, 7u64, f447(v2), false)
                } else {
                    if f445(v2.clone(), Str::lit("namespace")) {
                        f449(f447(v2), f8(Str::lit("a namespace holds operations and attributes of no object, so it is behavior, not data (Web IDL 2.6)")))
                    } else {
                        Ok(Err::<(Adt13, List<Adt23>), Bytes>(f450(Str::lit("interface, dictionary or namespace"), v2)))
                    }
                }
            }
        } else {
            if f445(v1.clone(), Str::lit("dictionary")) {
                f448(v0, 2u64, f447(v1), true)
            } else {
                if f445(v1.clone(), Str::lit("enum")) {
                    let m2310 = f451(f447(v1));
                    match m2310 {
                        Ok(v4) => {
                            let m2311 = f452(f453(v4.clone()), Str::lit("{"));
                            match m2311 {
                                Ok(v6) => {
                                    let m2312 = f454(v0, v6, List::<Bytes>::nil());
                                    match m2312 {
                                        Ok(v8) => {
                                            Ok(Ok::<(Adt13, List<Adt23>), Bytes>((Adt13::C0(3u64, f455(v4), Bytes::lit(&[]), List::<Adt14>::nil(), {
                                                let (h2313, _) = v8;
                                                h2313
                                            }, f456(Bytes::lit(&[]))), {
                                                let (_, h2314) = v8;
                                                h2314
                                            })))
                                        }
                                        Err(v7) => {
                                            Ok(Err::<(Adt13, List<Adt23>), Bytes>(v7))
                                        }
                                    }
                                }
                                Err(v5) => {
                                    Ok(Err::<(Adt13, List<Adt23>), Bytes>(v5))
                                }
                            }
                        }
                        Err(v3) => {
                            Ok(Err::<(Adt13, List<Adt23>), Bytes>(v3))
                        }
                    }
                } else {
                    if f445(v1.clone(), Str::lit("typedef")) {
                        let m2315 = f457(v0, f447(v1), 0u64)?;
                        match m2315 {
                            Ok(v10) => {
                                let m2316 = f451(f458(v10.clone()));
                                match m2316 {
                                    Ok(v12) => {
                                        let m2317 = f452(f453(v12.clone()), Str::lit(";"));
                                        match m2317 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt13, List<Adt23>), Bytes>((Adt13::C0(4u64, f455(v12), Bytes::lit(&[]), List::<Adt14>::nil(), List::<Bytes>::nil(), f459(v10)), v14)))
                                            }
                                            Err(v13) => {
                                                Ok(Err::<(Adt13, List<Adt23>), Bytes>(v13))
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(Err::<(Adt13, List<Adt23>), Bytes>(v11))
                                    }
                                }
                            }
                            Err(v9) => {
                                Ok(Err::<(Adt13, List<Adt23>), Bytes>(v9))
                            }
                        }
                    } else {
                        if f445(v1.clone(), Str::lit("callback")) {
                            if f445(f447(v1.clone()), Str::lit("interface")) {
                                f449(f447(f447(v1)), f8(Str::lit("a callback interface is implemented by user code, so it is behavior, not data (Web IDL 2.4)")))
                            } else {
                                f449(f447(v1), f8(Str::lit("a callback function is behavior, not data (Web IDL 2.10)")))
                            }
                        } else {
                            if f445(v1.clone(), Str::lit("namespace")) {
                                f449(f447(v1), f8(Str::lit("a namespace holds operations and attributes of no object, so it is behavior, not data (Web IDL 2.6)")))
                            } else {
                                let m2318 = f451(v1);
                                match m2318 {
                                    Ok(v16) => {
                                        let m2319 = f452(f453(v16.clone()), Str::lit("includes"));
                                        match m2319 {
                                            Ok(v18) => {
                                                let m2320 = f451(v18);
                                                match m2320 {
                                                    Ok(v20) => {
                                                        let m2321 = f452(f453(v20.clone()), Str::lit(";"));
                                                        match m2321 {
                                                            Ok(v22) => {
                                                                Ok(Ok::<(Adt13, List<Adt23>), Bytes>((Adt13::C0(8u64, f455(v16), f455(v20), List::<Adt14>::nil(), List::<Bytes>::nil(), f456(Bytes::lit(&[]))), v22)))
                                                            }
                                                            Err(v21) => {
                                                                Ok(Err::<(Adt13, List<Adt23>), Bytes>(v21))
                                                            }
                                                        }
                                                    }
                                                    Err(v19) => {
                                                        Ok(Err::<(Adt13, List<Adt23>), Bytes>(v19))
                                                    }
                                                }
                                            }
                                            Err(v17) => {
                                                Ok(Err::<(Adt13, List<Adt23>), Bytes>(v17))
                                            }
                                        }
                                    }
                                    Err(v15) => {
                                        Ok(Err::<(Adt13, List<Adt23>), Bytes>(v15))
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

pub fn f331(p2325: List<Bytes>, v1: List<Adt16>) -> Bytes {
    let m2324 = p2325;
    {
        let mut m2324 = m2324;
        loop {
            let step = {
                let v0 = m2324;
                let m2322 = v0;
                match m2322.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m2323 = f222(v1.clone(), v2.clone());
                        match m2323 {
                            None => {
                                Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("refers to ")), List::cons(v2, List::cons(f8(Str::lit(", which no input defines (Web IDL 2.13.22 to 2.13.26)")), List::<Bytes>::nil())))))
                            }
                            Some(v4) => {
                                if f32(f223(v4.clone()), Bytes::lit(&[])) {
                                    Ok::<List<Bytes>, Bytes>(v3)
                                } else {
                                    Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("refers to ")), List::cons(v2, List::cons(f8(Str::lit(", which is not translated: ")), List::cons(f223(v4), List::<Bytes>::nil()))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2324 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f332(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f125(v0)?;
    if f122(v1.clone(), f460()) {
        f461(v1)
    } else {
        Ok(v1)
    }
}

pub fn f333(p2331: List<Adt13>, v1: List<Adt13>, v2: Bytes) -> Option<Bytes> {
    let m2330 = p2331;
    {
        let mut m2330 = m2330;
        loop {
            let step = {
                let v0 = m2330;
                let m2326 = v0;
                match m2326.uncons() {
                    None => {
                        Err::<List<Adt13>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v3, v4)) => {
                        let a2327 = f219(v3.clone());
                        let a2328 = 8u64;
                        if if nat_eq(a2327, a2328) {
                            f32(f221(v3.clone()), v2.clone())
                        } else {
                            false
                        } {
                            let m2329 = f229(v1.clone(), 1u64, f231(v3.clone()));
                            match m2329 {
                                None => {
                                    Err::<List<Adt13>, Option<Bytes>>(Some(f231(v3)))
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
                Ok(next) => { m2330 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f334(v0: u64, v1: List<Adt13>, v2: Adt13) -> R<Result<List<Adt14>, Bytes>> {
    let m2332 = v0;
    if m2332 == 0 {
        Ok(Err::<List<Adt14>, Bytes>(f7(List::cons(f8(Str::lit("the inheritance of ")), List::cons(f221(v2), List::cons(f8(Str::lit(" is cyclic (Web IDL 2.2 and 2.7: an interface or dictionary must not inherit from itself)")), List::<Bytes>::nil()))))))
    } else {
        let v3 = m2332 - 1;
        let v4: List<Adt14> = {
            let a2333 = f219(v2.clone());
            let a2334 = 2u64;
            if nat_eq(a2333, a2334) {
                f462(f463(f464(v2.clone()), f465(v1.clone(), 7u64, f221(v2.clone()))))?
            } else {
                f463(f464(v2.clone()), f463(f465(v1.clone(), 5u64, f221(v2.clone())), f466(v1.clone(), v1.clone(), f221(v2.clone()))))
            }
        };
        if f32(f231(v2.clone()), Bytes::lit(&[])) {
            Ok(Ok::<List<Adt14>, Bytes>(v4))
        } else {
            let m2335 = f229(v1.clone(), f219(v2.clone()), f231(v2.clone()));
            match m2335 {
                None => {
                    Ok(Err::<List<Adt14>, Bytes>(f7(List::cons(f8(Str::lit("inherits ")), List::cons(f231(v2.clone()), List::cons({
                        let a2336 = f219(v2);
                        let a2337 = 2u64;
                        if nat_eq(a2336, a2337) {
                            f8(Str::lit(", which no input defines as a dictionary (Web IDL 2.7)"))
                        } else {
                            f8(Str::lit(", which no input defines as an interface (Web IDL 2.2)"))
                        }
                    }, List::<Bytes>::nil()))))))
                }
                Some(v5) => {
                    let m2338 = f334(v3, v1, v5)?;
                    match m2338 {
                        Ok(v7) => {
                            Ok(Ok::<List<Adt14>, Bytes>(f463(v7, v4)))
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

pub fn f335(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: mk, fields: []}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))
}

pub fn f336(v0: bool, v1: List<Adt14>) -> R<Result<(List<Bytes>, List<Bytes>), Bytes>> {
    let m2339 = v1;
    match m2339.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::<Bytes>::nil(), List::<Bytes>::nil())))
        }
        Some((v2, v3)) => {
            let v4: Adt15 = f467(v2.clone());
            if !f32(f345(v4.clone()), Bytes::lit(&[])) {
                Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons(f468(v2), List::cons(f8(Str::lit(": ")), List::cons(f345(v4), List::<Bytes>::nil())))))))
            } else {
                let m2340 = f469(v0, f468(v2.clone()))?;
                match m2340 {
                    None => {
                        Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons(f468(v2), List::cons(f8(Str::lit(" has no field name")), List::<Bytes>::nil()))))))
                    }
                    Some(v5) => {
                        if f122(v5.clone(), f470(v0, f338(v3.clone()))?) {
                            Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("two of its members derive the field name ")), List::cons(v5, List::cons(f8(Str::lit(" (Web IDL 2.2 and 2.7: the members of one definition have distinct identifiers)")), List::<Bytes>::nil()))))))
                        } else {
                            let m2341 = f336(v0, v3)?;
                            match m2341 {
                                Ok(v7) => {
                                    Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v5, List::cons(f8(Str::lit(", type: ")), List::cons({
                                        let a2342 = f471(v2);
                                        let a2343 = 1u64;
                                        if nat_eq(a2342, a2343) {
                                            f7(List::cons(f8(Str::lit("(Option ")), List::cons(f347(v4.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                        } else {
                                            f347(v4.clone())
                                        }
                                    }, List::cons(f8(Str::lit("}")), List::<Bytes>::nil())))))), {
                                        let (h2344, _) = v7;
                                        h2344
                                    }), f224(f348(v4), {
                                        let (_, h2345) = v7;
                                        h2345
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

pub fn f337(p2349: List<Bytes>) -> R<bool> {
    let m2348 = p2349;
    {
        let mut m2348 = m2348;
        loop {
            let step = {
                let v0 = m2348;
                let m2346 = v0;
                match m2346.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m2347 = f472(v1)?;
                        match m2347 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f122(v3, f470(false, v2.clone())?) {
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
                Ok(next) => { m2348 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f338(v0: List<Adt14>) -> List<Bytes> {
    let m2350 = v0;
    match m2350.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f468(v1), f338(v2))
        }
    }
}

pub fn f339(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}")), List::cons(f156(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f340(v0: Adt13) -> List<Bytes> {
    let m2351 = v0;
    match m2351 {
        Adt13::C0(_, _, _, _, v5, _) => {
            v5
        }
    }
}

pub fn f341(v0: bool, v1: List<Adt2>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m2352 = v1;
    match m2352.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m2353 = f473(v0, v3.clone())?;
            match m2353 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f474(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f122(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m2354 = f341(v0, v4, List::cons(v5.clone(), v2))?;
                        match m2354 {
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

pub fn f342(v0: List<Bytes>) -> List<Adt2> {
    let m2355 = v0;
    match m2355.uncons() {
        None => {
            List::<Adt2>::nil()
        }
        Some((v1, v2)) => {
            List::cons(Adt2::C6(v1), f342(v2))
        }
    }
}

pub fn f343(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f156(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f344(v0: Adt13) -> Adt15 {
    let m2356 = v0;
    match m2356 {
        Adt13::C0(_, _, _, _, _, v6) => {
            v6
        }
    }
}

pub fn f345(v0: Adt15) -> Bytes {
    let m2357 = v0;
    match m2357 {
        Adt15::C0(_, _, _, v4, _) => {
            v4
        }
    }
}

pub fn f346(v0: Adt15) -> u64 {
    let m2358 = v0;
    match m2358 {
        Adt15::C0(v1, _, _, _, _) => {
            v1
        }
    }
}

pub fn f347(v0: Adt15) -> Bytes {
    let m2359 = v0;
    match m2359 {
        Adt15::C0(_, v2, _, _, _) => {
            v2
        }
    }
}

pub fn f348(v0: Adt15) -> List<Bytes> {
    let m2360 = v0;
    match m2360 {
        Adt15::C0(_, _, v3, _, _) => {
            v3
        }
    }
}

pub fn f349(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f350(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2361 = f419(v0, v1)?;
    match m2361 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f475({
                let (h2362, _) = v2;
                h2362
            })? {
                Ok(Some({
                    let a2364 = v1;
                    let a2365 = {
                        let (_, h2363) = v2;
                        h2363
                    };
                    nat_add(a2364, a2365)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f351(v0: u64, v1: Bytes) -> R<bool> {
    f476(v0, v1.clone(), 0u64, {
        let a2367 = {
            let a2366 = v1;
            length_bytes(a2366)
        };
        let a2368 = 3u64;
        let a2369 = 0u64;
        nat_quot(a2367, a2368, a2369)
    }, true)
}

pub fn f352() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{0}\u{0} "))
}

pub fn f353(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f477(v1, v0, v2)
}

pub fn f354(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f478(v0, v1)
}

pub fn f355(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f479(v0, v1, v2)
}

pub fn f356(v0: Bytes) -> Result<(Bytes, u64), Bytes> {
    Err::<(Bytes, u64), Bytes>(v0)
}

pub fn f357(v0: Bytes, v1: u64) -> R<Result<(Bytes, u64), Bytes>> {
    if f162(v0.clone(), v1, Str::lit("&#")) {
        let v2: bool = {
            let a2372 = f163(v0.clone(), {
                let a2370 = v1;
                let a2371 = 2u64;
                nat_add(a2370, a2371)?
            })?;
            let a2373 = 120u64;
            nat_eq(a2372, a2373)
        };
        let v3: u64 = if v2 {
            let a2374 = v1;
            let a2375 = 3u64;
            nat_add(a2374, a2375)?
        } else {
            let a2376 = v1;
            let a2377 = 2u64;
            nat_add(a2376, a2377)?
        };
        let v4: (u64, u64) = f480(v0.clone(), v2, v3)?;
        let a2379 = {
            let (h2378, _) = v4;
            h2378
        };
        let a2380 = v3;
        if if nat_eq(a2379, a2380) {
            true
        } else {
            let a2382 = f163(v0, {
                let (h2381, _) = v4;
                h2381
            })?;
            let a2383 = 59u64;
            !nat_eq(a2382, a2383)
        } {
            Ok(f356(f137(Str::lit("a character reference must be &#digits; or &#xhex; (XML 1.0 [66])"), v1)?))
        } else {
            if f475({
                let (_, h2384) = v4;
                h2384
            })? {
                Ok(Ok::<(Bytes, u64), Bytes>((f481({
                    let (_, h2385) = v4;
                    h2385
                })?, {
                    let a2387 = {
                        let (h2386, _) = v4;
                        h2386
                    };
                    let a2388 = 1u64;
                    nat_add(a2387, a2388)?
                })))
            } else {
                Ok(f356(f137(Str::lit("a character reference must name a Char (XML 1.0 [66] WFC: Legal Character)"), v1)?))
            }
        }
    } else {
        if f162(v0.clone(), v1, Str::lit("&lt;")) {
            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("<")), {
                let a2389 = v1;
                let a2390 = 4u64;
                nat_add(a2389, a2390)?
            })))
        } else {
            if f162(v0.clone(), v1, Str::lit("&gt;")) {
                Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit(">")), {
                    let a2391 = v1;
                    let a2392 = 4u64;
                    nat_add(a2391, a2392)?
                })))
            } else {
                if f162(v0.clone(), v1, Str::lit("&amp;")) {
                    Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("&")), {
                        let a2393 = v1;
                        let a2394 = 5u64;
                        nat_add(a2393, a2394)?
                    })))
                } else {
                    if f162(v0.clone(), v1, Str::lit("&apos;")) {
                        Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("'")), {
                            let a2395 = v1;
                            let a2396 = 6u64;
                            nat_add(a2395, a2396)?
                        })))
                    } else {
                        if f162(v0, v1, Str::lit("&quot;")) {
                            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("\"")), {
                                let a2397 = v1;
                                let a2398 = 6u64;
                                nat_add(a2397, a2398)?
                            })))
                        } else {
                            Ok(f356(f137(Str::lit("an entity reference names an entity that is not declared, and only the five predefined entities are (XML 1.0 section 4.1, WFC: Entity Declared)"), v1)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f358(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<u64> {
    let (h2399, _) = f204(Fn5::F482(v1, v2, v0.clone()), {
        let a2401 = {
            let a2400 = v0;
            length_bytes(a2400)
        };
        let a2402 = 1u64;
        nat_add(a2401, a2402)?
    }, v3)?;
    Ok(h2399)
}

pub fn f359(p2415: Adt17, v1: List<(Bytes, Bytes)>) -> R<Result<Option<Adt31>, Bytes>> {
    let m2414 = p2415;
    {
        let mut m2414 = m2414;
        loop {
            let step = {
                let v0 = m2414;
                let m2403 = v0;
                match m2403 {
                    Adt17::C0(_, _, _) => {
                        Err::<Adt17, R<Result<Option<Adt31>, Bytes>>>(Ok(Ok::<Option<Adt31>, Bytes>(None::<Adt31>)))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<Adt17, R<Result<Option<Adt31>, Bytes>>>(Ok(Ok::<Option<Adt31>, Bytes>(None::<Adt31>)))
                    }
                    Adt17::C2(_) => {
                        Err::<Adt17, R<Result<Option<Adt31>, Bytes>>>(Ok(Ok::<Option<Adt31>, Bytes>(None::<Adt31>)))
                    }
                    Adt17::C3(r2412, r2413) => {
                        let v2 = (*r2412).clone();
                        let v3 = (*r2413).clone();
                        let m2404 = v2;
                        match m2404 {
                            Adt17::C0(v4, r2410, r2411) => {
                                let v5 = (*r2410).clone();
                                let v6 = (*r2411).clone();
                                let m2405 = f166(v5.clone(), v1.clone());
                                match m2405 {
                                    Ok(v8) => {
                                        let m2406 = f167(v8.clone(), v4)?;
                                        match m2406 {
                                            Ok(v10) => {
                                                if f325({
                                                    let (_, h2407) = v10.clone();
                                                    h2407
                                                }, List::cons(f8(Str::lit("restriction")), List::cons(f8(Str::lit("list")), List::cons(f8(Str::lit("union")), List::<Bytes>::nil())))) {
                                                    if f32({
                                                        let (h2408, _) = v10;
                                                        h2408
                                                    }, f250()) {
                                                        Err::<Adt17, R<Result<Option<Adt31>, Bytes>>>(Ok(Ok::<Option<Adt31>, Bytes>(Some(Adt31::C0({
                                                            let (_, h2409) = v10;
                                                            h2409
                                                        }, v5, v6, v8)))))
                                                    } else {
                                                        Ok::<Adt17, R<Result<Option<Adt31>, Bytes>>>(v3)
                                                    }
                                                } else {
                                                    Ok::<Adt17, R<Result<Option<Adt31>, Bytes>>>(v3)
                                                }
                                            }
                                            Err(v9) => {
                                                Err::<Adt17, R<Result<Option<Adt31>, Bytes>>>(Ok(f483(v9)))
                                            }
                                        }
                                    }
                                    Err(v7) => {
                                        Err::<Adt17, R<Result<Option<Adt31>, Bytes>>>(Ok(f483(v7)))
                                    }
                                }
                            }
                            Adt17::C1(_, _, _) => {
                                Ok::<Adt17, R<Result<Option<Adt31>, Bytes>>>(v3)
                            }
                            Adt17::C2(_) => {
                                Ok::<Adt17, R<Result<Option<Adt31>, Bytes>>>(v3)
                            }
                            Adt17::C3(_, _) => {
                                Ok::<Adt17, R<Result<Option<Adt31>, Bytes>>>(v3)
                            }
                            Adt17::C4 => {
                                Ok::<Adt17, R<Result<Option<Adt31>, Bytes>>>(v3)
                            }
                        }
                    }
                    Adt17::C4 => {
                        Err::<Adt17, R<Result<Option<Adt31>, Bytes>>>(Ok(Ok::<Option<Adt31>, Bytes>(None::<Adt31>)))
                    }
                }
            };
            match step {
                Ok(next) => { m2414 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f360(v0: Bytes) -> Result<Adt24, Bytes> {
    Err::<Adt24, Bytes>(v0)
}

pub fn f361(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit(" at simpleType ")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f362(v0: List<(Bytes, Bytes)>, v1: Bytes, v2: Bytes) -> R<Result<Adt24, Bytes>> {
    let m2416 = f167(v0, v2.clone())?;
    match m2416 {
        Ok(v4) => {
            if f32({
                let (h2417, _) = v4.clone();
                h2417
            }, f250()) {
                let m2419 = f484({
                    let (_, h2418) = v4;
                    h2418
                }, 16u64)?;
                match m2419 {
                    Ok(v6) => {
                        Ok(Ok::<Adt24, Bytes>(Adt24::C0(v6, List::<Bytes>::nil())))
                    }
                    Err(v5) => {
                        Ok(f360(v5))
                    }
                }
            } else {
                if f32({
                    let (h2420, _) = v4.clone();
                    h2420
                }, v1) {
                    let v7: Bytes = f125({
                        let (_, h2421) = v4;
                        h2421
                    })?;
                    Ok(Ok::<Adt24, Bytes>(Adt24::C0(v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                } else {
                    Ok(f360(f7(List::cons(f8(Str::lit("the type ")), List::cons(v2, List::cons(f8(Str::lit(" is in the namespace ")), List::cons({
                        let (h2422, _) = v4;
                        h2422
                    }, List::cons(f8(Str::lit(", which is neither XML Schema's nor this document's target namespace; imported schema documents are not read yet")), List::<Bytes>::nil()))))))))
                }
            }
        }
        Err(v3) => {
            Ok(f360(v3))
        }
    }
}

pub fn f363(v0: Adt17, v1: List<(Bytes, Bytes)>) -> R<Result<List<Adt2>, Bytes>> {
    let m2423 = v0;
    match m2423 {
        Adt17::C0(_, _, _) => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
        Adt17::C2(_) => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
        Adt17::C3(r2430, r2431) => {
            let v2 = (*r2430).clone();
            let v3 = (*r2431).clone();
            let m2424 = f363(v3, v1.clone())?;
            match m2424 {
                Ok(v5) => {
                    let m2425 = v2;
                    match m2425 {
                        Adt17::C0(v6, r2429, _) => {
                            let v7 = (*r2429).clone();
                            let m2426 = f166(v7.clone(), v1);
                            match m2426 {
                                Ok(v10) => {
                                    let m2427 = f167(v10, v6)?;
                                    match m2427 {
                                        Ok(v12) => {
                                            if f168(v12, Str::lit("enumeration")) {
                                                let m2428 = f169(v7, f8(Str::lit("value")));
                                                match m2428 {
                                                    None => {
                                                        Ok(f485(f8(Str::lit("an enumeration facet must have a value attribute (XSD 1.1 Part 2, section 4.3.5.2)"))))
                                                    }
                                                    Some(v13) => {
                                                        Ok(Ok::<List<Adt2>, Bytes>(List::cons(Adt2::C6(v13), v5)))
                                                    }
                                                }
                                            } else {
                                                Ok(Ok::<List<Adt2>, Bytes>(v5))
                                            }
                                        }
                                        Err(v11) => {
                                            Ok(f485(v11))
                                        }
                                    }
                                }
                                Err(v9) => {
                                    Ok(f485(v9))
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
                    Ok(f485(v4))
                }
            }
        }
        Adt17::C4 => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
    }
}

pub fn f364(v0: List<Adt2>, v1: List<Bytes>) -> R<List<Adt2>> {
    let m2432 = v0;
    match m2432.uncons() {
        None => {
            Ok(List::<Adt2>::nil())
        }
        Some((v2, v3)) => {
            let v4: Bytes = f474(v2.clone())?;
            if f122(v4.clone(), v1.clone()) {
                f364(v3, v1)
            } else {
                Ok(List::cons(v2, f364(v3, List::cons(v4, v1))?))
            }
        }
    }
}

pub fn f365(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f53(v0.clone(), v1) {
        let m2433 = f44(v0, 34u8)?;
        match m2433.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m2434 = v3;
                match m2434.uncons() {
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

pub fn f366(v0: List<Adt19>) -> R<Result<List<Adt26>, Bytes>> {
    let m2435 = v0;
    match m2435.uncons() {
        None => {
            Ok(Ok::<List<Adt26>, Bytes>(List::<Adt26>::nil()))
        }
        Some((v1, v2)) => {
            let m2438 = f264(match v1.clone() {
                Adt19::C0(_, _, _, _, h2436, _) => {
                    h2436
                }
            }, match v1.clone() {
                Adt19::C0(_, _, _, h2437, _, _) => {
                    h2437
                }
            })?;
            match m2438 {
                Ok(v4) => {
                    let m2439 = f366(v2)?;
                    match m2439 {
                        Ok(v6) => {
                            Ok(Ok::<List<Adt26>, Bytes>(f486(f487(v4, match v1 {
                                Adt19::C0(_, _, h2440, _, _, _) => {
                                    h2440
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

pub fn f367(v0: List<Adt19>) -> List<Bytes> {
    let m2441 = v0;
    match m2441.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt19::C0(_, _, h2442, _, _, _) => {
                    h2442
                }
            }, f367(v2))
        }
    }
}

pub fn f368(v0: u64, v1: Adt25, v2: List<Adt26>) -> R<Result<List<Adt28>, Bytes>> {
    let m2443 = v2;
    match m2443.uncons() {
        None => {
            Ok(Ok::<List<Adt28>, Bytes>(List::<Adt28>::nil()))
        }
        Some((v3, v4)) => {
            let m2444 = f488(v0, v1.clone(), v3)?;
            match m2444 {
                Ok(v6) => {
                    let m2445 = f368(v0, v1, v4)?;
                    match m2445 {
                        Ok(v8) => {
                            Ok(Ok::<List<Adt28>, Bytes>(f489(v6, v8)))
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

pub fn f369(v0: List<Adt19>) -> R<u64> {
    let m2446 = v0;
    match m2446.uncons() {
        None => {
            Ok(64u64)
        }
        Some((v1, v2)) => {
            let a2448 = match v1 {
                Adt19::C0(_, _, _, _, _, h2447) => {
                    h2447
                }
            };
            let a2449 = f369(v2)?;
            nat_add(a2448, a2449)
        }
    }
}

pub fn f370(p2457: List<Adt28>, p2458: List<Adt28>) -> Result<List<Adt28>, Bytes> {
    let m2456 = (p2457, p2458);
    {
        let mut m2456 = m2456;
        loop {
            let step = {
                let (v0, v1) = m2456;
                let m2450 = v0;
                match m2450.uncons() {
                    None => {
                        Err::<(List<Adt28>, List<Adt28>), Result<List<Adt28>, Bytes>>(Ok::<List<Adt28>, Bytes>(f490(v1, List::<Adt28>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m2452 = f491(v1.clone(), match v2.clone() {
                            Adt28::C0(h2451, _, _, _) => {
                                h2451
                            }
                        });
                        match m2452 {
                            None => {
                                Ok::<(List<Adt28>, List<Adt28>), Result<List<Adt28>, Bytes>>((v3, List::cons(v2, v1)))
                            }
                            Some(v4) => {
                                if f32(match v4 {
                                    Adt28::C0(_, _, h2453, _) => {
                                        h2453
                                    }
                                }, match v2.clone() {
                                    Adt28::C0(_, _, h2454, _) => {
                                        h2454
                                    }
                                }) {
                                    Ok::<(List<Adt28>, List<Adt28>), Result<List<Adt28>, Bytes>>((v3, v1))
                                } else {
                                    Err::<(List<Adt28>, List<Adt28>), Result<List<Adt28>, Bytes>>(Err::<List<Adt28>, Bytes>(f260(f8(Str::lit("two components derive the type name ")), match v2 {
                                        Adt28::C0(h2455, _, _, _) => {
                                            h2455
                                        }
                                    }, f8(Str::lit(", so their LexLean declarations would collide")))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2456 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f371(v0: List<Adt28>) -> List<Adt28> {
    if if f122(f492(), f493(v0.clone())) {
        let m2459 = f491(v0.clone(), f492());
        match m2459 {
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
        List::cons(Adt28::C0(f492(), List::<Bytes>::nil(), f343(f492(), f494()), f494()), v0)
    } else {
        v0
    }
}

pub fn f372(v0: List<Adt28>) -> List<Bytes> {
    let m2460 = v0;
    match m2460.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt28::C0(h2461, _, _, _) => {
                    h2461
                }
            }, f372(v2))
        }
    }
}

pub fn f373(p2467: List<Adt28>, v1: List<Bytes>) -> Option<(Bytes, Bytes)> {
    let m2466 = p2467;
    {
        let mut m2466 = m2466;
        loop {
            let step = {
                let v0 = m2466;
                let m2462 = v0;
                match m2462.uncons() {
                    None => {
                        Err::<List<Adt28>, Option<(Bytes, Bytes)>>(None::<(Bytes, Bytes)>)
                    }
                    Some((v2, v3)) => {
                        let m2464 = f253(match v2.clone() {
                            Adt28::C0(_, h2463, _, _) => {
                                h2463
                            }
                        }, v1.clone());
                        match m2464 {
                            None => {
                                Ok::<List<Adt28>, Option<(Bytes, Bytes)>>(v3)
                            }
                            Some(v4) => {
                                Err::<List<Adt28>, Option<(Bytes, Bytes)>>(Some((match v2 {
                                    Adt28::C0(h2465, _, _, _) => {
                                        h2465
                                    }
                                }, v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2466 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f374(v0: List<Adt28>, v1: List<Bytes>, v2: u64) -> List<(Bytes, List<Bytes>)> {
    let m2468 = v1;
    match m2468.uncons() {
        None => {
            List::<(Bytes, List<Bytes>)>::nil()
        }
        Some((v3, v4)) => {
            List::cons((v3.clone(), f495(v2, v0.clone(), f496(v0.clone(), v3), List::<Bytes>::nil())), f374(v0, v4, v2))
        }
    }
}

pub fn f375(v0: List<Adt28>) -> R<u64> {
    let m2469 = v0;
    match m2469.uncons() {
        None => {
            Ok(1u64)
        }
        Some((v1, v2)) => {
            let a2474 = {
                let a2472 = {
                    let a2471 = match v1 {
                        Adt28::C0(_, h2470, _, _) => {
                            h2470
                        }
                    };
                    length_list(a2471)
                };
                let a2473 = 1u64;
                nat_add(a2472, a2473)?
            };
            let a2475 = f375(v2)?;
            nat_add(a2474, a2475)
        }
    }
}

pub fn f376(p2485: u64, v1: List<Adt28>, p2486: List<(List<Bytes>, List<Bytes>)>, p2487: List<Bytes>, p2488: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m2484 = (p2485, (p2486, (p2487, p2488)));
    {
        let mut m2484 = m2484;
        loop {
            let step = {
                let (v0, (v2, (v3, v4))) = m2484;
                let m2476 = v0;
                if m2476 == 0 {
                    let m2477 = v2;
                    match m2477.uncons() {
                        None => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v4))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(f8(Str::lit("the declarations could not be ordered so that each follows the types it uses"))))
                        }
                    }
                } else {
                    let v7 = m2476 - 1;
                    let m2478 = v2.clone();
                    match m2478.uncons() {
                        None => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v4))
                        }
                        Some((_, _)) => {
                            let m2479 = f497(v2, v3.clone());
                            match m2479 {
                                None => {
                                    Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(f8(Str::lit("the declarations could not be ordered so that each follows the types it uses"))))
                                }
                                Some(v10) => {
                                    let m2481 = f498(v1.clone(), {
                                        let (h2480, _) = v10.clone();
                                        h2480
                                    });
                                    match m2481 {
                                        Ok(v12) => {
                                            Ok::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>((v7, ({
                                                let (_, h2482) = v10;
                                                h2482
                                            }, (f499({
                                                let (h2483, _) = v10;
                                                h2483
                                            }, v3), f499(v4, v12)))))
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
                Ok(next) => { m2484 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f377(v0: List<Adt28>, v1: List<List<Bytes>>) -> List<(List<Bytes>, List<Bytes>)> {
    let m2489 = v1;
    match m2489.uncons() {
        None => {
            List::<(List<Bytes>, List<Bytes>)>::nil()
        }
        Some((v2, v3)) => {
            List::cons((v2.clone(), f500(v0.clone(), v2.clone(), v2)), f377(v0, v3))
        }
    }
}

pub fn f378(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> List<List<Bytes>> {
    let m2490 = v2;
    match m2490.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((v4, v5)) => {
            if f122(v4.clone(), v3.clone()) {
                f378(v0, v1, v5, v3)
            } else {
                let v6: List<Bytes> = f501(v0.clone(), v1.clone(), v4);
                List::cons(v6.clone(), f378(v0, v1, v5, f499(v6, v3)))
            }
        }
    }
}

pub fn f379(v0: bool) -> bool {
    !v0
}

pub fn f380(v0: Adt27, v1: Str) -> bool {
    f32(match v0 {
        Adt27::C0(_, h2491, _, _, _) => {
            h2491
        }
    }, f8(v1))
}

pub fn f381(v0: Adt27, v1: Str) -> Option<Bytes> {
    f169(match v0 {
        Adt27::C0(_, _, h2492, _, _) => {
            h2492
        }
    }, f8(v1))
}

pub fn f382(v0: Option<Bytes>, v1: Bytes) -> R<Result<Bytes, Bytes>> {
    let m2493 = f234(v1.clone(), f8(Str::lit("://")), 0u64)?;
    match m2493 {
        None => {
            let v3: Bytes = f260(f8(Str::lit("the relative schema location ")), v1.clone(), f8(Str::lit(" is not resolved, because only a bare file name relative to a bound document's URL is merged (RFC 3986, section 5.2.3)")));
            if if f53(v1.clone(), f8(Str::lit("/"))) {
                true
            } else {
                let m2494 = f234(v1.clone(), f8(Str::lit("..")), 0u64)?;
                match m2494 {
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
                let m2495 = v0;
                match m2495 {
                    None => {
                        Ok(Err::<Bytes, Bytes>(v3))
                    }
                    Some(v5) => {
                        let m2497 = f502(v5.clone(), 0u64, {
                            let a2496 = v5.clone();
                            length_bytes(a2496)
                        }, None::<u64>)?;
                        match m2497 {
                            None => {
                                Ok(Err::<Bytes, Bytes>(v3))
                            }
                            Some(v6) => {
                                Ok(Ok::<Bytes, Bytes>({
                                    let a2500 = f46(v5, 0u64, {
                                        let a2498 = v6;
                                        let a2499 = 1u64;
                                        nat_add(a2498, a2499)?
                                    });
                                    let a2501 = v1;
                                    append_bytes(a2500, a2501)
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

pub fn f383(p2506: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m2505 = p2506;
    {
        let mut m2505 = m2505;
        loop {
            let step = {
                let v0 = m2505;
                let m2502 = v0;
                match m2502.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h2503, _) = v2;
                            h2503
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h2504) = v2;
                                h2504
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2505 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f384(v0: Adt27, v1: Str, v2: Str) -> Bytes {
    let m2507 = f381(v0, v1);
    match m2507 {
        None => {
            f8(v2)
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f385(v0: bool, v1: u64) -> R<bool> {
    let a2514 = {
        let a2511 = {
            let a2508 = v1;
            let a2509 = f397(v0)?;
            let a2510 = 0u64;
            nat_quot(a2508, a2509, a2510)
        };
        let a2512 = f399(f400(v0))?;
        let a2513 = 0u64;
        nat_rem(a2511, a2512, a2513)
    };
    let a2515 = f401(v0)?;
    Ok(!nat_eq(a2514, a2515))
}

pub fn f386(v0: Bytes, v1: u64) -> R<Bytes> {
    let a2518 = f46(v0.clone(), {
        let a2516 = v1;
        let a2517 = 2u64;
        nat_add(a2516, a2517)?
    }, 2u64);
    let a2519 = Bytes::lit(&[0x30, 0x30]);
    if equal(a2518, a2519) {
        let m2522 = f503(f46(v0, {
            let a2520 = v1;
            let a2521 = 4u64;
            nat_add(a2520, a2521)?
        }, 2u64))?;
        match m2522 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f111(v2.clone(), 0u64)?;
                let a2523 = v3;
                let a2524 = 32u64;
                if nat_lt(a2523, a2524) {
                    if f504(v3) {
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

pub fn f387(v0: u64) -> Bytes {
    let a2525 = v0;
    let a2526 = 34u64;
    if nat_eq(a2525, a2526) {
        Bytes::lit(&[0x22])
    } else {
        let a2527 = v0;
        let a2528 = 92u64;
        if nat_eq(a2527, a2528) {
            Bytes::lit(&[0x5c])
        } else {
            let a2529 = v0;
            let a2530 = 98u64;
            if nat_eq(a2529, a2530) {
                Bytes::lit(&[0x08])
            } else {
                let a2531 = v0;
                let a2532 = 102u64;
                if nat_eq(a2531, a2532) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a2533 = v0;
                    let a2534 = 110u64;
                    if nat_eq(a2533, a2534) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a2535 = v0;
                        let a2536 = 114u64;
                        if nat_eq(a2535, a2536) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a2537 = v0;
                            let a2538 = 116u64;
                            if nat_eq(a2537, a2538) {
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

pub fn f388(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f111(v0, v1)?;
    let a2539 = 47u64;
    let a2540 = v2;
    if if nat_lt(a2539, a2540) {
        let a2541 = v2;
        let a2542 = 58u64;
        nat_lt(a2541, a2542)
    } else {
        false
    } {
        Ok(Some({
            let a2543 = v1;
            let a2544 = 1u64;
            nat_add(a2543, a2544)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f389(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2545 = f111(v0, v1)?;
    let a2546 = 48u64;
    if nat_eq(a2545, a2546) {
        Ok(Some({
            let a2547 = v1;
            let a2548 = 1u64;
            nat_add(a2547, a2548)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f390(v0: Bytes) -> R<u64> {
    let (h2549, _) = f204(Fn5::F505(v0.clone()), {
        let a2550 = v0;
        length_bytes(a2550)
    }, 0u64)?;
    Ok(h2549)
}

pub fn f391(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m2554 = {
        let a2552 = v0;
        let a2553 = {
            let (h2551, _) = v1;
            h2551
        };
        index_bytes(a2552, a2553)
    };
    match m2554 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2556 = {
                    let (h2555, _) = v1;
                    h2555
                };
                let a2557 = 1u64;
                nat_add(a2556, a2557)?
            }, {
                let a2561 = {
                    let a2559 = {
                        let (_, h2558) = v1;
                        h2558
                    };
                    let a2560 = 10u64;
                    nat_mul(a2559, a2560)?
                };
                let a2562 = f506(v2)?;
                nat_add(a2561, a2562)?
            })))
        }
    }
}

pub fn f392(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f507(v0, v1)
}

pub fn f393(v0: Fn7, p2566: u64, p2567: Adt30) -> R<Adt30> {
    let m2565 = (p2566, p2567);
    {
        let mut m2565 = m2565;
        loop {
            let step = {
                let (v1, v2) = m2565;
                let m2563 = v1;
                if m2563 == 0 {
                    Err::<(u64, Adt30), R<Adt30>>(Ok(v2))
                } else {
                    let v3 = m2563 - 1;
                    Ok::<(u64, Adt30), R<Adt30>>((v3, {
                        let c2564: Fn7 = v0.clone();
                        c2564.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2565 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f394(v0: Adt30) -> R<Adt30> {
    f508(v0)
}

pub fn f395(v0: u32, v1: u32) -> u32 {
    let m2570 = {
        let a2568 = f509(v0);
        let a2569 = f509(v1);
        fixed_u64::checked_add(a2568, a2569)
    };
    match m2570 {
        None => {
            0u32
        }
        Some(v2) => {
            f510(v2)
        }
    }
}

pub fn f396(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a2573 = {
        let a2571 = v3;
        let a2572 = 8u64;
        nat_add(a2571, a2572)?
    };
    let a2574 = v2;
    if nat_eq(a2573, a2574) {
        Ok(f511({
            let a2577 = {
                let a2575 = v1;
                let a2576 = 8u64;
                nat_mul(a2575, a2576)?
            };
            let a2578 = 4294967296u64;
            let a2579 = 0u64;
            nat_quot(a2577, a2578, a2579)
        }))
    } else {
        let a2582 = {
            let a2580 = v3;
            let a2581 = 4u64;
            nat_add(a2580, a2581)?
        };
        let a2583 = v2;
        if nat_eq(a2582, a2583) {
            Ok(f511({
                let a2586 = {
                    let a2584 = v1;
                    let a2585 = 8u64;
                    nat_mul(a2584, a2585)?
                };
                let a2587 = 4294967296u64;
                let a2588 = 0u64;
                nat_rem(a2586, a2587, a2588)
            }))
        } else {
            Ok(f512(f513(v0.clone(), v1, v3), f513(v0.clone(), v1, {
                let a2589 = v3;
                let a2590 = 1u64;
                nat_add(a2589, a2590)?
            }), f513(v0.clone(), v1, {
                let a2591 = v3;
                let a2592 = 2u64;
                nat_add(a2591, a2592)?
            }), f513(v0, v1, {
                let a2593 = v3;
                let a2594 = 3u64;
                nat_add(a2593, a2594)?
            })))
        }
    }
}

pub fn f397(v0: bool) -> R<u64> {
    f399({
        let a2595 = f404(v0);
        let a2596 = 1u64;
        nat_sub(a2595, a2596)
    })
}

pub fn f398(v0: bool) -> R<u64> {
    f399(if v0 {
        63u64
    } else {
        31u64
    })
}

pub fn f399(v0: u64) -> R<u64> {
    let m2597 = v0;
    if m2597 == 0 {
        Ok(1u64)
    } else {
        let v1 = m2597 - 1;
        let a2598 = 2u64;
        let a2599 = f399(v1)?;
        nat_mul(a2598, a2599)
    }
}

pub fn f400(v0: bool) -> u64 {
    if v0 {
        11u64
    } else {
        8u64
    }
}

pub fn f401(v0: bool) -> R<u64> {
    let a2600 = f399(f400(v0))?;
    let a2601 = 1u64;
    Ok(nat_sub(a2600, a2601))
}

pub fn f402(v0: bool, v1: Bytes) -> Bytes {
    if v0 {
        let a2602 = Bytes::lit(&[0x2d]);
        let a2603 = v1;
        append_bytes(a2602, a2603)
    } else {
        v1
    }
}

pub fn f403(v0: bool) -> u64 {
    if v0 {
        1023u64
    } else {
        127u64
    }
}

pub fn f404(v0: bool) -> u64 {
    if v0 {
        53u64
    } else {
        24u64
    }
}

pub fn f405(v0: u64, v1: bool, v2: u64) -> R<Bytes> {
    if v1 {
        f514(f515(v2, f516(3u64, v0), 2u64)?)
    } else {
        let v3: Bytes = f514(f515(v2, f516(3u64, v0), 5u64)?)?;
        let v4: u64 = {
            let a2604 = v3.clone();
            length_bytes(a2604)
        };
        let v5: Bytes = {
            let a2605 = v2;
            let a2606 = v4;
            if nat_lt(a2605, a2606) {
                f46(v3.clone(), 0u64, {
                    let a2607 = v4;
                    let a2608 = v2;
                    nat_sub(a2607, a2608)
                })
            } else {
                Bytes::lit(&[0x30])
            }
        };
        let v6: Bytes = f517({
            let a2609 = v2;
            let a2610 = v4;
            if nat_lt(a2609, a2610) {
                f46(v3, {
                    let a2611 = v4;
                    let a2612 = v2;
                    nat_sub(a2611, a2612)
                }, v2)
            } else {
                let a2615 = f518({
                    let a2613 = v2;
                    let a2614 = v4;
                    nat_sub(a2613, a2614)
                });
                let a2616 = v3;
                append_bytes(a2615, a2616)
            }
        }, {
            let a2617 = v2;
            let a2618 = 1u64;
            nat_add(a2617, a2618)?
        })?;
        let a2620 = {
            let a2619 = v6.clone();
            length_bytes(a2619)
        };
        let a2621 = 0u64;
        if nat_eq(a2620, a2621) {
            Ok(v5)
        } else {
            Ok(f7(List::cons(v5, List::cons(Bytes::lit(&[0x2e]), List::cons(v6, List::<Bytes>::nil())))))
        }
    }
}

pub fn f406(v0: Bytes, p2640: u64, p2641: u64, p2642: Bytes) -> R<Bytes> {
    let m2639 = (p2640, (p2641, p2642));
    {
        let mut m2639 = m2639;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2639;
                let m2622 = v2;
                if m2622 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m2622 - 1;
                    let v5: u64 = f180(v0.clone(), v1, {
                        let a2626 = {
                            let a2624 = {
                                let a2623 = v0.clone();
                                length_bytes(a2623)
                            };
                            let a2625 = v1;
                            nat_sub(a2624, a2625)
                        };
                        let a2627 = 1u64;
                        nat_add(a2626, a2627)?
                    })?;
                    let v6: Bytes = {
                        let a2630 = v3;
                        let a2631 = f46(v0.clone(), v1, {
                            let a2628 = v5;
                            let a2629 = v1;
                            nat_sub(a2628, a2629)
                        });
                        append_bytes(a2630, a2631)
                    };
                    let m2634 = {
                        let a2632 = v0.clone();
                        let a2633 = v5;
                        index_bytes(a2632, a2633)
                    };
                    match m2634 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v6))
                        }
                        Some(v7) => {
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                let a2635 = v5;
                                let a2636 = 1u64;
                                nat_add(a2635, a2636)?
                            }, (v4, {
                                let a2637 = v6;
                                let a2638 = f519(v0.clone(), v5, v7)?;
                                append_bytes(a2637, a2638)
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2639 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f407(v0: Fn8, p2646: List<(Bytes, Bytes)>, p2647: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m2645 = (p2646, p2647);
    {
        let mut m2645 = m2645;
        loop {
            let step = {
                let (v1, v2) = m2645;
                let m2643 = v2;
                match m2643.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c2644: Fn8 = v0.clone();
                            c2644.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2645 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f408(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f520(v0, v1)
}

pub fn f409(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f521(v0)
}

pub fn f410(v0: Fn9, p2651: List<Bytes>, p2652: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m2650 = (p2651, p2652);
    {
        let mut m2650 = m2650;
        loop {
            let step = {
                let (v1, v2) = m2650;
                let m2648 = v2;
                match m2648.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c2649: Fn9 = v0.clone();
                            c2649.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2650 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f411(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f522(v1)?, v0))
}

pub fn f412(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(f523(v0.clone(), 0u64, {
        let a2653 = v0;
        length_bytes(a2653)
    }, Bytes::lit(&[]))?, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
}

pub fn f413(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h2654, _) = v1.clone();
        h2654
    };
    let v3: u64 = {
        let (h2655, _) = {
            let (_, h2656) = v1.clone();
            h2656
        };
        h2655
    };
    let v4: List<Bytes> = {
        let (_, h2657) = {
            let (_, h2658) = v1;
            h2658
        };
        h2657
    };
    let m2659 = f188(v0.clone(), v2);
    match m2659 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a2660 = v5;
            let a2661 = 10u8;
            if equal(a2660, a2661) {
                Ok(Some(({
                    let a2662 = v2;
                    let a2663 = 1u64;
                    nat_add(a2662, a2663)?
                }, ({
                    let a2664 = v2;
                    let a2665 = 1u64;
                    nat_add(a2664, a2665)?
                }, List::cons(f524(f46(v0, v3, {
                    let a2666 = v2;
                    let a2667 = v3;
                    nat_sub(a2666, a2667)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a2668 = v2;
                    let a2669 = 1u64;
                    nat_add(a2668, a2669)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f414(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f415(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m2670 = v0.clone();
    match m2670.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a2671 = v2;
            let a2672 = v1.clone();
            if equal(a2671, a2672) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f416(v0: List<(Bytes, ())>) -> List<Bytes> {
    f297(f525(Fn10::F526, List::<Bytes>::nil(), v0))
}

pub fn f417(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f527(f528(Fn11::F529, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m2677 = {
        let (h2673, _) = f530(Fn12::F531, {
            let a2675 = {
                let a2674 = v0;
                length_list(a2674)
            };
            let a2676 = 1u64;
            nat_add(a2675, a2676)?
        }, v1)?;
        h2673
    };
    match m2677.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f418(v0: List<Bytes>) -> List<(Bytes, ())> {
    f532(f533(Fn13::F534, List::<(Bytes, ())>::nil(), v0))
}

pub fn f419(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m2680 = {
        let a2678 = v0.clone();
        let a2679 = v1;
        index_bytes(a2678, a2679)
    };
    match m2680 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f62(v2)?;
            let a2681 = v3;
            let a2682 = 128u64;
            if nat_lt(a2681, a2682) {
                Ok(Some((v3, 1u64)))
            } else {
                if f113({
                    let a2683 = 194u64;
                    let a2684 = v3;
                    nat_le(a2683, a2684)
                }, {
                    let a2685 = v3;
                    let a2686 = 223u64;
                    nat_le(a2685, a2686)
                }) {
                    let m2689 = f535(v0, {
                        let a2687 = v1;
                        let a2688 = 1u64;
                        nat_add(a2687, a2688)?
                    })?;
                    match m2689 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f536({
                                let a2694 = {
                                    let a2692 = {
                                        let a2690 = v3;
                                        let a2691 = 192u64;
                                        nat_sub(a2690, a2691)
                                    };
                                    let a2693 = 64u64;
                                    nat_mul(a2692, a2693)?
                                };
                                let a2695 = v4;
                                nat_add(a2694, a2695)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f113({
                        let a2696 = 224u64;
                        let a2697 = v3;
                        nat_le(a2696, a2697)
                    }, {
                        let a2698 = v3;
                        let a2699 = 239u64;
                        nat_le(a2698, a2699)
                    }) {
                        let m2702 = f535(v0.clone(), {
                            let a2700 = v1;
                            let a2701 = 1u64;
                            nat_add(a2700, a2701)?
                        })?;
                        match m2702 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m2705 = f535(v0, {
                                    let a2703 = v1;
                                    let a2704 = 2u64;
                                    nat_add(a2703, a2704)?
                                })?;
                                match m2705 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f536({
                                            let a2714 = {
                                                let a2708 = {
                                                    let a2706 = v3;
                                                    let a2707 = 224u64;
                                                    nat_sub(a2706, a2707)
                                                };
                                                let a2709 = 4096u64;
                                                nat_mul(a2708, a2709)?
                                            };
                                            let a2715 = {
                                                let a2712 = {
                                                    let a2710 = v5;
                                                    let a2711 = 64u64;
                                                    nat_mul(a2710, a2711)?
                                                };
                                                let a2713 = v6;
                                                nat_add(a2712, a2713)?
                                            };
                                            nat_add(a2714, a2715)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f113({
                            let a2716 = 240u64;
                            let a2717 = v3;
                            nat_le(a2716, a2717)
                        }, {
                            let a2718 = v3;
                            let a2719 = 244u64;
                            nat_le(a2718, a2719)
                        }) {
                            let m2722 = f535(v0.clone(), {
                                let a2720 = v1;
                                let a2721 = 1u64;
                                nat_add(a2720, a2721)?
                            })?;
                            match m2722 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m2725 = f535(v0.clone(), {
                                        let a2723 = v1;
                                        let a2724 = 2u64;
                                        nat_add(a2723, a2724)?
                                    })?;
                                    match m2725 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m2728 = f535(v0, {
                                                let a2726 = v1;
                                                let a2727 = 3u64;
                                                nat_add(a2726, a2727)?
                                            })?;
                                            match m2728 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f536({
                                                        let a2741 = {
                                                            let a2731 = {
                                                                let a2729 = v3;
                                                                let a2730 = 240u64;
                                                                nat_sub(a2729, a2730)
                                                            };
                                                            let a2732 = 262144u64;
                                                            nat_mul(a2731, a2732)?
                                                        };
                                                        let a2742 = {
                                                            let a2739 = {
                                                                let a2733 = v7;
                                                                let a2734 = 4096u64;
                                                                nat_mul(a2733, a2734)?
                                                            };
                                                            let a2740 = {
                                                                let a2737 = {
                                                                    let a2735 = v8;
                                                                    let a2736 = 64u64;
                                                                    nat_mul(a2735, a2736)?
                                                                };
                                                                let a2738 = v9;
                                                                nat_add(a2737, a2738)?
                                                            };
                                                            nat_add(a2739, a2740)?
                                                        };
                                                        nat_add(a2741, a2742)?
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

pub fn f420(v0: u64) -> bool {
    let a2743 = v0;
    let a2744 = 9u64;
    if if if nat_eq(a2743, a2744) {
        true
    } else {
        let a2745 = v0;
        let a2746 = 10u64;
        nat_eq(a2745, a2746)
    } {
        true
    } else {
        let a2747 = v0;
        let a2748 = 13u64;
        if nat_eq(a2747, a2748) {
            true
        } else {
            let a2749 = 32u64;
            let a2750 = v0;
            if nat_le(a2749, a2750) {
                let a2751 = v0;
                let a2752 = 126u64;
                nat_le(a2751, a2752)
            } else {
                false
            }
        }
    } {
        true
    } else {
        let a2753 = v0;
        let a2754 = 133u64;
        if if nat_eq(a2753, a2754) {
            true
        } else {
            let a2755 = 160u64;
            let a2756 = v0;
            if nat_le(a2755, a2756) {
                let a2757 = v0;
                let a2758 = 55295u64;
                nat_le(a2757, a2758)
            } else {
                false
            }
        } {
            true
        } else {
            let a2759 = 57344u64;
            let a2760 = v0;
            if if nat_le(a2759, a2760) {
                let a2761 = v0;
                let a2762 = 65533u64;
                nat_le(a2761, a2762)
            } else {
                false
            } {
                true
            } else {
                let a2763 = 65536u64;
                let a2764 = v0;
                if nat_le(a2763, a2764) {
                    let a2765 = v0;
                    let a2766 = 1114111u64;
                    nat_le(a2765, a2766)
                } else {
                    false
                }
            }
        }
    }
}

pub fn f421(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f307(v0.clone(), v1)?;
    if f305(f209(v0.clone(), v2)?) {
        Ok(true)
    } else {
        let a2767 = f209(v0, v2)?;
        let a2768 = 35u64;
        Ok(nat_eq(a2767, a2768))
    }
}

pub fn f422(v0: Bytes, v1: u64) -> R<u64> {
    let (h2769, _) = f204(Fn5::F537(v0), {
        let a2770 = v1;
        let a2771 = 1u64;
        nat_add(a2770, a2771)?
    }, v1)?;
    Ok(h2769)
}

pub fn f423(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f538(v0, v1)
}

pub fn f424(v0: Bytes, v1: u64) -> R<u64> {
    let (h2772, _) = f204(Fn5::F539(v0.clone()), {
        let a2774 = {
            let a2773 = v0;
            length_bytes(a2773)
        };
        let a2775 = 1u64;
        nat_add(a2774, a2775)?
    }, v1)?;
    Ok(h2772)
}

pub fn f425(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f540(v0, v1)
}

pub fn f426(v0: u64, v1: bool) -> bool {
    f211(if if f304(v0) {
        true
    } else {
        f305(v0)
    } {
        true
    } else {
        if v1 {
            f541(v0)
        } else {
            false
        }
    })
}

pub fn f427(v0: Bytes, v1: u64, v2: u64) -> R<Adt22> {
    let v3: bool = {
        let a2776 = f209(v0.clone(), v1)?;
        let a2777 = 62u64;
        nat_eq(a2776, a2777)
    };
    let v4: u64 = f209(v0.clone(), {
        let a2778 = v1;
        let a2779 = 1u64;
        nat_add(a2778, a2779)?
    })?;
    let v5: u64 = f209(v0.clone(), {
        let a2780 = v1;
        let a2781 = 2u64;
        nat_add(a2780, a2781)?
    })?;
    let v6: u64 = f542(v4);
    let v7: u64 = f543(v4);
    let v8: bool = {
        let a2782 = 0u64;
        let a2783 = v6;
        if nat_lt(a2782, a2783) {
            true
        } else {
            let a2784 = 0u64;
            let a2785 = v7;
            nat_lt(a2784, a2785)
        }
    };
    let v9: u64 = {
        let a2786 = 0u64;
        let a2787 = v7;
        if nat_lt(a2786, a2787) {
            f542(v5)
        } else {
            0u64
        }
    };
    let v10: u64 = {
        let a2788 = 0u64;
        let a2789 = v6;
        if nat_lt(a2788, a2789) {
            f543(v5)
        } else {
            0u64
        }
    };
    let v11: u64 = {
        let a2790 = v6;
        let a2791 = v9;
        nat_add(a2790, a2791)?
    };
    let v12: u64 = {
        let a2792 = v7;
        let a2793 = v10;
        nat_add(a2792, a2793)?
    };
    let v13: u64 = {
        let a2802 = {
            let a2794 = v1;
            let a2795 = 1u64;
            nat_add(a2794, a2795)?
        };
        let a2803 = {
            let a2800 = if v8 {
                1u64
            } else {
                0u64
            };
            let a2801 = {
                let a2796 = 0u64;
                let a2797 = v9;
                if if nat_lt(a2796, a2797) {
                    true
                } else {
                    let a2798 = 0u64;
                    let a2799 = v10;
                    nat_lt(a2798, a2799)
                } {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a2800, a2801)?
        };
        nat_add(a2802, a2803)?
    };
    let m2804 = f208(v0.clone(), v13)?;
    match m2804 {
        Ok(v15) => {
            let a2805 = 0u64;
            let a2806 = v12;
            if nat_lt(a2805, a2806) {
                f544(v0, v15, {
                    let a2809 = {
                        let a2807 = v2;
                        let a2808 = v12;
                        nat_add(a2807, a2808)?
                    };
                    let a2810 = 1u64;
                    nat_sub(a2809, a2810)
                }, v3, v11)
            } else {
                let v16: (u64, u64) = {
                    let (h2811, _) = f274(Fn6::F545(v0.clone()), {
                        let a2813 = {
                            let a2812 = v0.clone();
                            length_bytes(a2812)
                        };
                        let a2814 = 1u64;
                        nat_add(a2813, a2814)?
                    }, (v15, 0u64))?;
                    h2811
                };
                let v17: u64 = f309(v0.clone(), {
                    let (h2815, _) = v16;
                    h2815
                })?;
                let a2816 = f209(v0.clone(), v17)?;
                let a2817 = 256u64;
                if nat_eq(a2816, a2817) {
                    Ok(Adt22::C0(Adt2::C6({
                        let a2818 = v11;
                        let a2819 = 2u64;
                        if nat_eq(a2818, a2819) {
                            f546(f547(v0.clone(), v15)?)
                        } else {
                            Bytes::lit(&[])
                        }
                    }), {
                        let a2820 = v0;
                        length_bytes(a2820)
                    }))
                } else {
                    let v18: u64 = f548({
                        let a2822 = v17;
                        let a2823 = {
                            let (h2821, _) = v16;
                            h2821
                        };
                        nat_sub(a2822, a2823)
                    }, v2);
                    let a2825 = v18;
                    let a2826 = {
                        let (_, h2824) = v16;
                        h2824
                    };
                    if nat_lt(a2825, a2826) {
                        Ok(Adt22::C1(f137(Str::lit("a leading empty line of a block scalar holds more spaces than its first content line (YAML 1.2.2 section 8.1.1.1)"), v15)?))
                    } else {
                        f544(v0, v15, v18, v3, v11)
                    }
                }
            }
        }
        Err(v14) => {
            Ok(Adt22::C1(v14))
        }
    }
}

pub fn f428(v0: Adt21, v1: Bytes, v2: bool, v3: u64, v4: u64) -> R<Adt22> {
    let m2827 = v0;
    match m2827 {
        Adt21::C0(v7) => {
            let a2828 = v7;
            let a2829 = 1u64;
            if if if nat_eq(a2828, a2829) {
                true
            } else {
                let a2830 = v7;
                let a2831 = 8u64;
                nat_eq(a2830, a2831)
            } {
                true
            } else {
                let a2832 = v7;
                let a2833 = 0u64;
                if nat_eq(a2832, a2833) {
                    v2
                } else {
                    false
                }
            } {
                Ok(Adt22::C0(Adt2::C6(v1), v4))
            } else {
                let a2834 = v7;
                let a2835 = 0u64;
                if nat_eq(a2834, a2835) {
                    f549(v1, v3, v4)
                } else {
                    let a2836 = v7;
                    let a2837 = 5u64;
                    if nat_eq(a2836, a2837) {
                        if f550(v1) {
                            Ok(Adt22::C0(Adt2::C0, v4))
                        } else {
                            Ok(Adt22::C1(f436(v3)?))
                        }
                    } else {
                        let a2838 = v7;
                        let a2839 = 2u64;
                        if if nat_eq(a2838, a2839) {
                            f551(v1.clone())?
                        } else {
                            let a2840 = v7;
                            let a2841 = 3u64;
                            if nat_eq(a2840, a2841) {
                                let m2842 = f552(v1.clone())?;
                                match m2842 {
                                    None => {
                                        f553(v1.clone())
                                    }
                                    Some(_) => {
                                        true
                                    }
                                }
                            } else {
                                let a2843 = v7;
                                let a2844 = 4u64;
                                if nat_eq(a2843, a2844) {
                                    if f554(v1.clone()) {
                                        true
                                    } else {
                                        f555(v1.clone())
                                    }
                                } else {
                                    false
                                }
                            }
                        } {
                            f549(v1, v3, v4)
                        } else {
                            Ok(Adt22::C1(f436(v3)?))
                        }
                    }
                }
            }
        }
        Adt21::C1(v6) => {
            Ok(Adt22::C0(Adt2::C1(v6, std::rc::Rc::new(Adt2::C6(v1))), v4))
        }
        Adt21::C2(v5) => {
            Ok(Adt22::C1(v5))
        }
    }
}

pub fn f429(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Adt32> {
    let a2845 = f209(v0.clone(), v1)?;
    let a2846 = 34u64;
    if nat_eq(a2845, a2846) {
        f556(v0.clone(), {
            let a2847 = v1;
            let a2848 = 1u64;
            nat_add(a2847, a2848)?
        }, v2, v3, {
            let a2850 = {
                let a2849 = v0;
                length_bytes(a2849)
            };
            let a2851 = 2u64;
            nat_add(a2850, a2851)?
        }, Bytes::lit(&[]))
    } else {
        f557(v0.clone(), {
            let a2852 = v1;
            let a2853 = 1u64;
            nat_add(a2852, a2853)?
        }, v2, v3, {
            let a2855 = {
                let a2854 = v0;
                length_bytes(a2854)
            };
            let a2856 = 2u64;
            nat_add(a2855, a2856)?
        }, Bytes::lit(&[]))
    }
}

pub fn f430(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: List<Bytes>, v6: Adt21) -> R<Adt22> {
    let m2857 = v3;
    if m2857 == 0 {
        Ok(Adt22::C1(f137(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v7 = m2857 - 1;
        let a2858 = v4;
        let a2859 = 1u64;
        if nat_eq(a2858, a2859) {
            let m2860 = f430(v0.clone(), v1, v2, v7, 0u64, List::<Bytes>::nil(), Adt21::C0(0u64))?;
            match m2860 {
                Adt22::C0(v9, v10) => {
                    let m2861 = f558(v0.clone(), v10, v2)?;
                    match m2861 {
                        Ok(v12) => {
                            let v13: u64 = f209(v0.clone(), v12)?;
                            let a2862 = v13;
                            let a2863 = 58u64;
                            if nat_eq(a2862, a2863) {
                                Ok(Adt22::C1(f137(Str::lit("a single-pair mapping inside a flow sequence is not read (YAML 1.2.2 [150])"), v12)?))
                            } else {
                                let a2864 = v13;
                                let a2865 = 93u64;
                                if nat_eq(a2864, a2865) {
                                    Ok(Adt22::C0(Adt2::C10(std::rc::Rc::new(v9), std::rc::Rc::new(Adt2::C9)), {
                                        let a2866 = v12;
                                        let a2867 = 1u64;
                                        nat_add(a2866, a2867)?
                                    }))
                                } else {
                                    let a2868 = v13;
                                    let a2869 = 44u64;
                                    if nat_eq(a2868, a2869) {
                                        let m2872 = f558(v0.clone(), {
                                            let a2870 = v12;
                                            let a2871 = 1u64;
                                            nat_add(a2870, a2871)?
                                        }, v2)?;
                                        match m2872 {
                                            Ok(v15) => {
                                                let a2873 = f209(v0.clone(), v15)?;
                                                let a2874 = 93u64;
                                                if nat_eq(a2873, a2874) {
                                                    Ok(Adt22::C0(Adt2::C10(std::rc::Rc::new(v9), std::rc::Rc::new(Adt2::C9)), {
                                                        let a2875 = v15;
                                                        let a2876 = 1u64;
                                                        nat_add(a2875, a2876)?
                                                    }))
                                                } else {
                                                    let m2877 = f430(v0, v15, v2, v7, 1u64, List::<Bytes>::nil(), Adt21::C0(0u64))?;
                                                    match m2877 {
                                                        Adt22::C0(v17, v18) => {
                                                            Ok(Adt22::C0(Adt2::C10(std::rc::Rc::new(v9), std::rc::Rc::new(v17)), v18))
                                                        }
                                                        Adt22::C1(v16) => {
                                                            Ok(Adt22::C1(v16))
                                                        }
                                                    }
                                                }
                                            }
                                            Err(v14) => {
                                                Ok(Adt22::C1(v14))
                                            }
                                        }
                                    } else {
                                        Ok(Adt22::C1(f137(Str::lit("a flow sequence entry must be followed by , or ] (YAML 1.2.2 [137])"), v12)?))
                                    }
                                }
                            }
                        }
                        Err(v11) => {
                            Ok(Adt22::C1(v11))
                        }
                    }
                }
                Adt22::C1(v8) => {
                    Ok(Adt22::C1(v8))
                }
            }
        } else {
            let a2878 = v4;
            let a2879 = 2u64;
            if nat_eq(a2878, a2879) {
                let m2880 = f559(v0.clone(), v1, v2)?;
                match m2880 {
                    Adt32::C0(v20, v21) => {
                        let m2881 = f558(v0.clone(), v21, v2)?;
                        match m2881 {
                            Ok(v23) => {
                                let m2895 = {
                                    let a2882 = f209(v0.clone(), v23)?;
                                    let a2883 = 44u64;
                                    if if nat_eq(a2882, a2883) {
                                        true
                                    } else {
                                        let a2884 = f209(v0.clone(), v23)?;
                                        let a2885 = 125u64;
                                        nat_eq(a2884, a2885)
                                    } {
                                        Adt22::C0(Adt2::C0, v23)
                                    } else {
                                        if f211({
                                            let a2886 = f209(v0.clone(), v23)?;
                                            let a2887 = 58u64;
                                            nat_eq(a2886, a2887)
                                        }) {
                                            Adt22::C1(f137(Str::lit("a flow mapping entry must be a key, then : and its value, or a key alone (YAML 1.2.2 [142], [146])"), v23)?)
                                        } else {
                                            let m2890 = f558(v0.clone(), {
                                                let a2888 = v23;
                                                let a2889 = 1u64;
                                                nat_add(a2888, a2889)?
                                            }, v2)?;
                                            match m2890 {
                                                Ok(v25) => {
                                                    let a2891 = f209(v0.clone(), v25)?;
                                                    let a2892 = 44u64;
                                                    if if nat_eq(a2891, a2892) {
                                                        true
                                                    } else {
                                                        let a2893 = f209(v0.clone(), v25)?;
                                                        let a2894 = 125u64;
                                                        nat_eq(a2893, a2894)
                                                    } {
                                                        Adt22::C0(Adt2::C0, v25)
                                                    } else {
                                                        f430(v0.clone(), v25, v2, v7, 0u64, List::<Bytes>::nil(), Adt21::C0(0u64))?
                                                    }
                                                }
                                                Err(v24) => {
                                                    Adt22::C1(v24)
                                                }
                                            }
                                        }
                                    }
                                };
                                match m2895 {
                                    Adt22::C0(v27, v28) => {
                                        if f325(v20.clone(), v5.clone()) {
                                            Ok(Adt22::C1(f137(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                        } else {
                                            let m2896 = f558(v0.clone(), v28, v2)?;
                                            match m2896 {
                                                Ok(v30) => {
                                                    let v31: u64 = f209(v0.clone(), v30)?;
                                                    let a2897 = v31;
                                                    let a2898 = 125u64;
                                                    if nat_eq(a2897, a2898) {
                                                        Ok(Adt22::C0(Adt2::C11(v20, std::rc::Rc::new(v27), std::rc::Rc::new(Adt2::C9)), {
                                                            let a2899 = v30;
                                                            let a2900 = 1u64;
                                                            nat_add(a2899, a2900)?
                                                        }))
                                                    } else {
                                                        let a2901 = v31;
                                                        let a2902 = 44u64;
                                                        if nat_eq(a2901, a2902) {
                                                            let m2905 = f558(v0.clone(), {
                                                                let a2903 = v30;
                                                                let a2904 = 1u64;
                                                                nat_add(a2903, a2904)?
                                                            }, v2)?;
                                                            match m2905 {
                                                                Ok(v33) => {
                                                                    let a2906 = f209(v0.clone(), v33)?;
                                                                    let a2907 = 125u64;
                                                                    if nat_eq(a2906, a2907) {
                                                                        Ok(Adt22::C0(Adt2::C11(v20, std::rc::Rc::new(v27), std::rc::Rc::new(Adt2::C9)), {
                                                                            let a2908 = v33;
                                                                            let a2909 = 1u64;
                                                                            nat_add(a2908, a2909)?
                                                                        }))
                                                                    } else {
                                                                        let m2910 = f430(v0, v33, v2, v7, 2u64, List::cons(v20.clone(), v5), Adt21::C0(0u64))?;
                                                                        match m2910 {
                                                                            Adt22::C0(v35, v36) => {
                                                                                Ok(Adt22::C0(Adt2::C11(v20, std::rc::Rc::new(v27), std::rc::Rc::new(v35)), v36))
                                                                            }
                                                                            Adt22::C1(v34) => {
                                                                                Ok(Adt22::C1(v34))
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                Err(v32) => {
                                                                    Ok(Adt22::C1(v32))
                                                                }
                                                            }
                                                        } else {
                                                            Ok(Adt22::C1(f137(Str::lit("a flow mapping entry must be followed by , or } (YAML 1.2.2 [140])"), v30)?))
                                                        }
                                                    }
                                                }
                                                Err(v29) => {
                                                    Ok(Adt22::C1(v29))
                                                }
                                            }
                                        }
                                    }
                                    Adt22::C1(v26) => {
                                        Ok(Adt22::C1(v26))
                                    }
                                }
                            }
                            Err(v22) => {
                                Ok(Adt22::C1(v22))
                            }
                        }
                    }
                    Adt32::C1(v19) => {
                        Ok(Adt22::C1(v19))
                    }
                }
            } else {
                let v37: u64 = f209(v0.clone(), v1)?;
                let a2911 = v37;
                let a2912 = 33u64;
                if nat_eq(a2911, a2912) {
                    if f314(v6) {
                        Ok(Adt22::C1(f137(Str::lit("a node holds at most one tag (YAML 1.2.2 [96])"), v1)?))
                    } else {
                        let v38: Adt21 = f315(v0.clone(), v1)?;
                        let m2913 = f316(v38.clone());
                        match m2913 {
                            None => {
                                let m2914 = f558(v0.clone(), f317(v0.clone(), v1)?, v2)?;
                                match m2914 {
                                    Ok(v41) => {
                                        let a2915 = f320(f8(Str::lit(",]}:")), f209(v0.clone(), v41)?)?;
                                        let a2916 = 4u64;
                                        if nat_lt(a2915, a2916) {
                                            f319(v38, v41, v41)
                                        } else {
                                            f430(v0, v41, v2, v7, 0u64, List::<Bytes>::nil(), v38)
                                        }
                                    }
                                    Err(v40) => {
                                        Ok(Adt22::C1(v40))
                                    }
                                }
                            }
                            Some(v39) => {
                                Ok(Adt22::C1(v39))
                            }
                        }
                    }
                } else {
                    let a2917 = v37;
                    let a2918 = 91u64;
                    if if nat_eq(a2917, a2918) {
                        true
                    } else {
                        let a2919 = v37;
                        let a2920 = 123u64;
                        nat_eq(a2919, a2920)
                    } {
                        let m2923 = f558(v0.clone(), {
                            let a2921 = v1;
                            let a2922 = 1u64;
                            nat_add(a2921, a2922)?
                        }, v2)?;
                        match m2923 {
                            Ok(v43) => {
                                f318(v6, {
                                    let a2924 = v37;
                                    let a2925 = 91u64;
                                    nat_eq(a2924, a2925)
                                }, v1, {
                                    let a2928 = f209(v0.clone(), v43)?;
                                    let a2929 = {
                                        let a2926 = v37;
                                        let a2927 = 2u64;
                                        nat_add(a2926, a2927)?
                                    };
                                    if nat_eq(a2928, a2929) {
                                        Adt22::C0(Adt2::C9, {
                                            let a2930 = v43;
                                            let a2931 = 1u64;
                                            nat_add(a2930, a2931)?
                                        })
                                    } else {
                                        f430(v0, v43, v2, v7, {
                                            let a2932 = v37;
                                            let a2933 = 91u64;
                                            if nat_eq(a2932, a2933) {
                                                1u64
                                            } else {
                                                2u64
                                            }
                                        }, List::<Bytes>::nil(), Adt21::C0(0u64))?
                                    }
                                })
                            }
                            Err(v42) => {
                                Ok(Adt22::C1(v42))
                            }
                        }
                    } else {
                        let a2934 = v37;
                        let a2935 = 34u64;
                        if if nat_eq(a2934, a2935) {
                            true
                        } else {
                            let a2936 = v37;
                            let a2937 = 39u64;
                            nat_eq(a2936, a2937)
                        } {
                            let m2938 = f429(v0, v1, v2, false)?;
                            match m2938 {
                                Adt32::C0(v45, v46) => {
                                    f428(v6, v45, true, v1, v46)
                                }
                                Adt32::C1(v44) => {
                                    Ok(Adt22::C1(v44))
                                }
                            }
                        } else {
                            let m2939 = f312(v0.clone(), v1)?;
                            match m2939 {
                                None => {
                                    if f211(f431(v0.clone(), v1, true)?) {
                                        Ok(Adt22::C1(f137(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                                    } else {
                                        let v48: u64 = f432(v0.clone(), v1, true)?;
                                        let v49: u64 = f307(v0.clone(), v48)?;
                                        let m2942 = {
                                            let a2940 = f209(v0.clone(), v49)?;
                                            let a2941 = 10u64;
                                            if nat_eq(a2940, a2941) {
                                                f558(v0.clone(), v49, v2)?
                                            } else {
                                                Ok::<u64, Bytes>(v49)
                                            }
                                        };
                                        match m2942 {
                                            Ok(v51) => {
                                                let a2943 = f209(v0.clone(), v49)?;
                                                let a2944 = 10u64;
                                                if if if nat_eq(a2943, a2944) {
                                                    f211(f541(f209(v0.clone(), v51)?))
                                                } else {
                                                    false
                                                } {
                                                    f211({
                                                        let a2945 = f209(v0.clone(), v51)?;
                                                        let a2946 = 58u64;
                                                        nat_eq(a2945, a2946)
                                                    })
                                                } else {
                                                    false
                                                } {
                                                    Ok(Adt22::C1(f137(Str::lit("a multi-line plain scalar inside a flow collection is not read (YAML 1.2.2 [135])"), v49)?))
                                                } else {
                                                    f428(v6, f321(v0, v1, v48), false, v1, v48)
                                                }
                                            }
                                            Err(v50) => {
                                                Ok(Adt22::C1(v50))
                                            }
                                        }
                                    }
                                }
                                Some(v47) => {
                                    Ok(Adt22::C1(v47))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f431(v0: Bytes, v1: u64, v2: bool) -> R<bool> {
    let v3: u64 = f209(v0.clone(), v1)?;
    if if f304(v3) {
        true
    } else {
        f305(v3)
    } {
        Ok(false)
    } else {
        let a2947 = v3;
        let a2948 = 45u64;
        if if if nat_eq(a2947, a2948) {
            true
        } else {
            let a2949 = v3;
            let a2950 = 63u64;
            nat_eq(a2949, a2950)
        } {
            true
        } else {
            let a2951 = v3;
            let a2952 = 58u64;
            nat_eq(a2951, a2952)
        } {
            Ok(f426(f209(v0, {
                let a2953 = v1;
                let a2954 = 1u64;
                nat_add(a2953, a2954)?
            })?, v2))
        } else {
            Ok(f211(f560(v3)?))
        }
    }
}

pub fn f432(v0: Bytes, v1: u64, v2: bool) -> R<u64> {
    let (_, h2955) = {
        let (h2956, _) = f274(Fn6::F561(v2, v0.clone()), {
            let a2958 = {
                let a2957 = v0;
                length_bytes(a2957)
            };
            let a2959 = 1u64;
            nat_add(a2958, a2959)?
        }, (v1, v1))?;
        h2956
    };
    Ok(h2955)
}

pub fn f433(v0: Bytes, p2984: u64, v2: u64, p2985: u64, p2986: Bytes) -> R<Adt32> {
    let m2983 = (p2984, (p2985, p2986));
    {
        let mut m2983 = m2983;
        loop {
            let step = {
                let (v1, (v3, v4)) = m2983;
                let m2960 = v3;
                if m2960 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C1(f137(Str::lit("a plain scalar has more lines than the input"), v1)?)))
                } else {
                    let v5 = m2960 - 1;
                    let v6: u64 = f307(v0.clone(), v1)?;
                    let v7: u64 = f209(v0.clone(), v6)?;
                    let a2961 = v7;
                    let a2962 = 58u64;
                    if nat_eq(a2961, a2962) {
                        Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C1(f562(v6)?)))
                    } else {
                        let a2963 = v7;
                        let a2964 = 35u64;
                        if if nat_eq(a2963, a2964) {
                            true
                        } else {
                            let a2965 = v7;
                            let a2966 = 256u64;
                            nat_eq(a2965, a2966)
                        } {
                            Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C0(v4, f308(v0.clone(), v6)?)))
                        } else {
                            let v8: (u64, u64) = f563(v0.clone(), {
                                let a2967 = v6;
                                let a2968 = 1u64;
                                nat_add(a2967, a2968)?
                            })?;
                            let v9: u64 = {
                                let (h2969, _) = v8;
                                h2969
                            };
                            let v10: u64 = f309(v0.clone(), v9)?;
                            let v11: u64 = f307(v0.clone(), v10)?;
                            let v12: u64 = f209(v0.clone(), v11)?;
                            let a2970 = v12;
                            let a2971 = 256u64;
                            if if if if nat_eq(a2970, a2971) {
                                true
                            } else {
                                let a2974 = {
                                    let a2972 = v10;
                                    let a2973 = v9;
                                    nat_sub(a2972, a2973)
                                };
                                let a2975 = v2;
                                nat_lt(a2974, a2975)
                            } {
                                true
                            } else {
                                let a2976 = v12;
                                let a2977 = 35u64;
                                nat_eq(a2976, a2977)
                            } {
                                true
                            } else {
                                f207(v0.clone(), v9)?
                            } {
                                Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C0(v4, {
                                    let a2978 = v6;
                                    let a2979 = 1u64;
                                    nat_add(a2978, a2979)?
                                })))
                            } else {
                                let v13: u64 = f432(v0.clone(), v11, false)?;
                                let a2980 = v13;
                                let a2981 = v11;
                                if nat_eq(a2980, a2981) {
                                    Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C1(f562(v11)?)))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Adt32>>((v13, (v5, f7(List::cons(v4, List::cons(f564({
                                        let (_, h2982) = v8;
                                        h2982
                                    }), List::cons(f321(v0.clone(), v11, v13), List::<Bytes>::nil())))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2983 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f434(v0: Bytes, v1: u64) -> R<u64> {
    let v2: bool = {
        let a2989 = f209(v0.clone(), {
            let a2987 = v1;
            let a2988 = 1u64;
            nat_add(a2987, a2988)?
        })?;
        let a2990 = 60u64;
        nat_eq(a2989, a2990)
    };
    let (h2991, _) = f204(Fn5::F565(v0.clone(), v2), {
        let a2993 = {
            let a2992 = v0;
            length_bytes(a2992)
        };
        let a2994 = 1u64;
        nat_add(a2993, a2994)?
    }, if v2 {
        let a2995 = v1;
        let a2996 = 2u64;
        nat_add(a2995, a2996)?
    } else {
        let a2997 = v1;
        let a2998 = 1u64;
        nat_add(a2997, a2998)?
    })?;
    Ok(h2991)
}

pub fn f435(v0: Bytes, v1: u64) -> R<Adt21> {
    let v2: u64 = f566(v0, List::cons(f8(Str::lit("str")), List::cons(f8(Str::lit("int")), List::cons(f8(Str::lit("float")), List::cons(f8(Str::lit("bool")), List::cons(f8(Str::lit("null")), List::cons(f8(Str::lit("seq")), List::cons(f8(Str::lit("map")), List::<Bytes>::nil()))))))), 1u64)?;
    let a2999 = v2;
    let a3000 = 0u64;
    if nat_eq(a2999, a3000) {
        Ok(Adt21::C2(f137(Str::lit("a tag outside the YAML 1.2.2 core schema is not read (YAML 1.2.2 section 10.3)"), v1)?))
    } else {
        Ok(Adt21::C0(v2))
    }
}

pub fn f436(v0: u64) -> R<Bytes> {
    f137(Str::lit("a node's content does not match its tag (YAML 1.2.2 section 10.3.2)"), v0)
}

pub fn f437(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f567(v0, v1, v2)
}

pub fn f438(v0: Bytes, v1: u64) -> R<bool> {
    let a3001 = f209(v0.clone(), v1)?;
    let a3002 = 58u64;
    if nat_eq(a3001, a3002) {
        if f304(f209(v0.clone(), {
            let a3003 = v1;
            let a3004 = 1u64;
            nat_add(a3003, a3004)?
        })?) {
            Ok(true)
        } else {
            Ok(f305(f209(v0, {
                let a3005 = v1;
                let a3006 = 1u64;
                nat_add(a3005, a3006)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f439(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a3007 = v0;
        utf8_encode(a3007)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f91(v1)?, List::<Bytes>::nil())))))
}

pub fn f440(p3010: List<Adt23>, p3011: List<Adt23>) -> List<Adt23> {
    let m3009 = (p3010, p3011);
    {
        let mut m3009 = m3009;
        loop {
            let step = {
                let (v0, v1) = m3009;
                let m3008 = v0;
                match m3008.uncons() {
                    None => {
                        Err::<(List<Adt23>, List<Adt23>), List<Adt23>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt23>, List<Adt23>), List<Adt23>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3009 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f441(v0: u64) -> bool {
    let a3012 = 65u64;
    let a3013 = v0;
    if if nat_le(a3012, a3013) {
        let a3014 = v0;
        let a3015 = 90u64;
        nat_le(a3014, a3015)
    } else {
        false
    } {
        true
    } else {
        let a3016 = 97u64;
        let a3017 = v0;
        if nat_le(a3016, a3017) {
            let a3018 = v0;
            let a3019 = 122u64;
            nat_le(a3018, a3019)
        } else {
            false
        }
    }
}

pub fn f442(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h3020, _) = f204(Fn5::F568(v1, v0.clone()), {
        let a3022 = {
            let a3021 = v0;
            length_bytes(a3021)
        };
        let a3023 = 1u64;
        nat_add(a3022, a3023)?
    }, v2)?;
    Ok(h3020)
}

pub fn f443(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f163(v0.clone(), v1)?;
    let a3024 = v2;
    let a3025 = 48u64;
    if nat_eq(a3024, a3025) {
        let v3: u64 = f163(v0.clone(), {
            let a3026 = v1;
            let a3027 = 1u64;
            nat_add(a3026, a3027)?
        })?;
        let a3028 = v3;
        let a3029 = 88u64;
        if if if nat_eq(a3028, a3029) {
            true
        } else {
            let a3030 = v3;
            let a3031 = 120u64;
            nat_eq(a3030, a3031)
        } {
            f569(f163(v0.clone(), {
                let a3032 = v1;
                let a3033 = 2u64;
                nat_add(a3032, a3033)?
            })?)
        } else {
            false
        } {
            f442(v0, 1u64, {
                let a3034 = v1;
                let a3035 = 2u64;
                nat_add(a3034, a3035)?
            })
        } else {
            f442(v0, 2u64, {
                let a3036 = v1;
                let a3037 = 1u64;
                nat_add(a3036, a3037)?
            })
        }
    } else {
        if f570(v2) {
            f442(v0, 0u64, v1)
        } else {
            Ok(v1)
        }
    }
}

pub fn f444(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f442(v0.clone(), 0u64, v1)?;
    let a3038 = f163(v0.clone(), v2)?;
    let a3039 = 46u64;
    if nat_eq(a3038, a3039) {
        let v3: u64 = f442(v0.clone(), 0u64, {
            let a3040 = v2;
            let a3041 = 1u64;
            nat_add(a3040, a3041)?
        })?;
        let a3042 = v1;
        let a3043 = v2;
        if if nat_lt(a3042, a3043) {
            true
        } else {
            let a3046 = {
                let a3044 = v2;
                let a3045 = 1u64;
                nat_add(a3044, a3045)?
            };
            let a3047 = v3;
            nat_lt(a3046, a3047)
        } {
            f571(v0, v3)
        } else {
            Ok(v1)
        }
    } else {
        let a3048 = v1;
        let a3049 = v2;
        if nat_lt(a3048, a3049) {
            let v4: u64 = f571(v0, v2)?;
            let a3050 = v2;
            let a3051 = v4;
            if nat_lt(a3050, a3051) {
                Ok(v4)
            } else {
                Ok(v1)
            }
        } else {
            Ok(v1)
        }
    }
}

pub fn f445(v0: List<Adt23>, v1: Str) -> bool {
    let m3052 = v0;
    match m3052.uncons() {
        None => {
            false
        }
        Some((v2, _)) => {
            f572(v2, {
                let a3053 = v1;
                utf8_encode(a3053)
            })
        }
    }
}

pub fn f446(p3066: u64, p3067: List<Adt23>, p3068: List<Bytes>, v3: bool) -> Result<List<Adt23>, Bytes> {
    let m3065 = (p3066, (p3067, p3068));
    {
        let mut m3065 = m3065;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3065;
                let m3054 = v0;
                if m3054 == 0 {
                    Err::<(u64, (List<Adt23>, List<Bytes>)), Result<List<Adt23>, Bytes>>(Err::<List<Adt23>, Bytes>(f8(Str::lit("brackets nest deeper than the input is long"))))
                } else {
                    let v4 = m3054 - 1;
                    let m3055 = v1;
                    match m3055.uncons() {
                        None => {
                            Err::<(u64, (List<Adt23>, List<Bytes>)), Result<List<Adt23>, Bytes>>(Err::<List<Adt23>, Bytes>(f8(Str::lit("the input ends inside brackets or before the ; that closes a member (Web IDL, IDL grammar)"))))
                        }
                        Some((v5, v6)) => {
                            let a3056 = f573(v5.clone());
                            let a3057 = 3u64;
                            if nat_eq(a3056, a3057) {
                                Ok::<(u64, (List<Adt23>, List<Bytes>)), Result<List<Adt23>, Bytes>>((v4, (v6, v2)))
                            } else {
                                let v7: Bytes = f574(v5);
                                let v8: Bytes = f575(v7.clone());
                                if !f32(v8.clone(), Bytes::lit(&[])) {
                                    Ok::<(u64, (List<Adt23>, List<Bytes>)), Result<List<Adt23>, Bytes>>((v4, (v6, List::cons(v8, v2))))
                                } else {
                                    if f576(v7.clone()) {
                                        let m3058 = v2;
                                        match m3058.uncons() {
                                            None => {
                                                Err::<(u64, (List<Adt23>, List<Bytes>)), Result<List<Adt23>, Bytes>>(Err::<List<Adt23>, Bytes>(f7(List::cons(f8(Str::lit("the closing bracket ")), List::cons(v7, List::cons(f8(Str::lit(" has no opening bracket (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
                                            }
                                            Some((v9, v10)) => {
                                                if f32(v9.clone(), v7.clone()) {
                                                    if if !v3 {
                                                        let a3060 = {
                                                            let a3059 = v10.clone();
                                                            length_list(a3059)
                                                        };
                                                        let a3061 = 0u64;
                                                        nat_eq(a3060, a3061)
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
                                                let a3063 = {
                                                    let a3062 = v2.clone();
                                                    length_list(a3062)
                                                };
                                                let a3064 = 0u64;
                                                nat_eq(a3063, a3064)
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
                Ok(next) => { m3065 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f447(v0: List<Adt23>) -> List<Adt23> {
    let m3069 = v0;
    match m3069.uncons() {
        None => {
            List::<Adt23>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f448(v0: u64, v1: u64, v2: List<Adt23>, v3: bool) -> R<Result<(Adt13, List<Adt23>), Bytes>> {
    let m3070 = f451(v2);
    match m3070 {
        Ok(v5) => {
            let v6: bool = if v3 {
                f445(f453(v5.clone()), Str::lit(":"))
            } else {
                false
            };
            let m3071 = if v6 {
                f451(f447(f453(v5.clone())))
            } else {
                Ok::<Adt33, Bytes>(Adt33::C0(Bytes::lit(&[]), f453(v5.clone())))
            };
            match m3071 {
                Ok(v8) => {
                    let m3072 = f452(f453(v8.clone()), Str::lit("{"));
                    match m3072 {
                        Ok(v10) => {
                            let m3077 = {
                                let a3073 = v1;
                                let a3074 = 2u64;
                                if if nat_eq(a3073, a3074) {
                                    true
                                } else {
                                    let a3075 = v1;
                                    let a3076 = 7u64;
                                    nat_eq(a3075, a3076)
                                } {
                                    f577(v0, v10, List::<Adt14>::nil())?
                                } else {
                                    f578(v0, v10, List::<Adt14>::nil())?
                                }
                            };
                            match m3077 {
                                Ok(v12) => {
                                    Ok(Ok::<(Adt13, List<Adt23>), Bytes>((Adt13::C0(v1, f455(v5), f455(v8), {
                                        let (h3078, _) = v12;
                                        h3078
                                    }, List::<Bytes>::nil(), f456(Bytes::lit(&[]))), {
                                        let (_, h3079) = v12;
                                        h3079
                                    })))
                                }
                                Err(v11) => {
                                    Ok(Err::<(Adt13, List<Adt23>), Bytes>(v11))
                                }
                            }
                        }
                        Err(v9) => {
                            Ok(Err::<(Adt13, List<Adt23>), Bytes>(v9))
                        }
                    }
                }
                Err(v7) => {
                    Ok(Err::<(Adt13, List<Adt23>), Bytes>(v7))
                }
            }
        }
        Err(v4) => {
            Ok(Err::<(Adt13, List<Adt23>), Bytes>(v4))
        }
    }
}

pub fn f449(v0: List<Adt23>, v1: Bytes) -> R<Result<(Adt13, List<Adt23>), Bytes>> {
    let m3080 = f451(v0);
    match m3080 {
        Ok(v3) => {
            let m3081 = f579(f453(v3.clone()))?;
            match m3081 {
                Ok(v5) => {
                    Ok(Ok::<(Adt13, List<Adt23>), Bytes>((Adt13::C0(9u64, f455(v3), Bytes::lit(&[]), List::<Adt14>::nil(), List::cons(v1, List::<Bytes>::nil()), f456(Bytes::lit(&[]))), v5)))
                }
                Err(v4) => {
                    Ok(Err::<(Adt13, List<Adt23>), Bytes>(v4))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<(Adt13, List<Adt23>), Bytes>(v2))
        }
    }
}

pub fn f450(v0: Str, v1: List<Adt23>) -> Bytes {
    f7(List::cons(f8(Str::lit("expected ")), List::cons({
        let a3082 = v0;
        utf8_encode(a3082)
    }, List::cons(f8(Str::lit(" but found ")), List::cons(f580(v1), List::cons(f8(Str::lit(" (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
}

pub fn f451(v0: List<Adt23>) -> Result<Adt33, Bytes> {
    let m3083 = v0.clone();
    match m3083.uncons() {
        None => {
            Err::<Adt33, Bytes>(f450(Str::lit("an identifier"), v0))
        }
        Some((v1, v2)) => {
            let a3084 = f573(v1.clone());
            let a3085 = 0u64;
            if nat_eq(a3084, a3085) {
                Ok::<Adt33, Bytes>(Adt33::C0(f581(f574(v1)), v2))
            } else {
                Err::<Adt33, Bytes>(f450(Str::lit("an identifier"), v0))
            }
        }
    }
}

pub fn f452(v0: List<Adt23>, v1: Str) -> Result<List<Adt23>, Bytes> {
    if f445(v0.clone(), v1.clone()) {
        Ok::<List<Adt23>, Bytes>(f447(v0))
    } else {
        Err::<List<Adt23>, Bytes>(f450(v1, v0))
    }
}

pub fn f453(v0: Adt33) -> List<Adt23> {
    let m3086 = v0;
    match m3086 {
        Adt33::C0(_, v2) => {
            v2
        }
    }
}

pub fn f454(p3094: u64, p3095: List<Adt23>, p3096: List<Bytes>) -> Result<(List<Bytes>, List<Adt23>), Bytes> {
    let m3093 = (p3094, (p3095, p3096));
    {
        let mut m3093 = m3093;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3093;
                let m3087 = v0;
                if m3087 == 0 {
                    Err::<(u64, (List<Adt23>, List<Bytes>)), Result<(List<Bytes>, List<Adt23>), Bytes>>(Err::<(List<Bytes>, List<Adt23>), Bytes>(f8(Str::lit("an enumeration has more values than the input has tokens"))))
                } else {
                    let v3 = m3087 - 1;
                    let m3088 = v1.clone();
                    match m3088.uncons() {
                        None => {
                            Err::<(u64, (List<Adt23>, List<Bytes>)), Result<(List<Bytes>, List<Adt23>), Bytes>>(Err::<(List<Bytes>, List<Adt23>), Bytes>(f450(Str::lit("a string"), v1)))
                        }
                        Some((v4, v5)) => {
                            let a3089 = f573(v4.clone());
                            let a3090 = 3u64;
                            if !nat_eq(a3089, a3090) {
                                Err::<(u64, (List<Adt23>, List<Bytes>)), Result<(List<Bytes>, List<Adt23>), Bytes>>(Err::<(List<Bytes>, List<Adt23>), Bytes>(f450(Str::lit("a string"), v1)))
                            } else {
                                if f445(v5.clone(), Str::lit(",")) {
                                    if f445(f447(v5.clone()), Str::lit("}")) {
                                        let m3091 = f452(f447(f447(v5)), Str::lit(";"));
                                        match m3091 {
                                            Ok(v7) => {
                                                Err::<(u64, (List<Adt23>, List<Bytes>)), Result<(List<Bytes>, List<Adt23>), Bytes>>(Ok::<(List<Bytes>, List<Adt23>), Bytes>((f68(List::cons(f574(v4), v2)), v7)))
                                            }
                                            Err(v6) => {
                                                Err::<(u64, (List<Adt23>, List<Bytes>)), Result<(List<Bytes>, List<Adt23>), Bytes>>(Err::<(List<Bytes>, List<Adt23>), Bytes>(v6))
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (List<Adt23>, List<Bytes>)), Result<(List<Bytes>, List<Adt23>), Bytes>>((v3, (f447(v5), List::cons(f574(v4), v2))))
                                    }
                                } else {
                                    if f445(v5.clone(), Str::lit("}")) {
                                        let m3092 = f452(f447(v5), Str::lit(";"));
                                        match m3092 {
                                            Ok(v9) => {
                                                Err::<(u64, (List<Adt23>, List<Bytes>)), Result<(List<Bytes>, List<Adt23>), Bytes>>(Ok::<(List<Bytes>, List<Adt23>), Bytes>((f68(List::cons(f574(v4), v2)), v9)))
                                            }
                                            Err(v8) => {
                                                Err::<(u64, (List<Adt23>, List<Bytes>)), Result<(List<Bytes>, List<Adt23>), Bytes>>(Err::<(List<Bytes>, List<Adt23>), Bytes>(v8))
                                            }
                                        }
                                    } else {
                                        Err::<(u64, (List<Adt23>, List<Bytes>)), Result<(List<Bytes>, List<Adt23>), Bytes>>(Err::<(List<Bytes>, List<Adt23>), Bytes>(f450(Str::lit(", or }"), v5)))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3093 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f455(v0: Adt33) -> Bytes {
    let m3097 = v0;
    match m3097 {
        Adt33::C0(v1, _) => {
            v1
        }
    }
}

pub fn f456(v0: Bytes) -> Adt15 {
    Adt15::C0(0u64, v0, List::<Bytes>::nil(), Bytes::lit(&[]), Bytes::lit(&[]))
}

pub fn f457(v0: u64, v1: List<Adt23>, v2: u64) -> R<Result<Adt34, Bytes>> {
    let m3098 = v0;
    if m3098 == 0 {
        Ok(Err::<Adt34, Bytes>(f8(Str::lit("a type nests deeper than the input is long"))))
    } else {
        let v3 = m3098 - 1;
        let a3099 = v2;
        let a3100 = 0u64;
        if nat_eq(a3099, a3100) {
            let m3101 = f329(v1)?;
            match m3101 {
                Ok(v5) => {
                    if f445(v5.clone(), Str::lit("(")) {
                        let m3102 = f457(v3, f447(v5), 1u64)?;
                        match m3102 {
                            Ok(v7) => {
                                if f445(f458(v7.clone()), Str::lit("?")) {
                                    Ok(Ok::<Adt34, Bytes>(Adt34::C0(Adt15::C0(0u64, Bytes::lit(&[]), f348(f459(v7.clone())), f8(Str::lit("a nullable union type is not translated yet, because the typedef identifier names only the union (Web IDL 2.13.32)")), Bytes::lit(&[])), f447(f458(v7)))))
                                } else {
                                    Ok(Ok::<Adt34, Bytes>(Adt34::C0(f459(v7.clone()), f458(v7))))
                                }
                            }
                            Err(v6) => {
                                Ok(Err::<Adt34, Bytes>(v6))
                            }
                        }
                    } else {
                        let m3103 = v5.clone();
                        match m3103.uncons() {
                            None => {
                                Ok(Err::<Adt34, Bytes>(f450(Str::lit("a type"), v5)))
                            }
                            Some((v8, v9)) => {
                                let v10: Bytes = f574(v8.clone());
                                let a3104 = f573(v8);
                                let a3105 = 0u64;
                                if !nat_eq(a3104, a3105) {
                                    Ok(Err::<Adt34, Bytes>(f450(Str::lit("a type"), v5)))
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
                                        let m3106 = f452(v9, Str::lit("<"));
                                        match m3106 {
                                            Ok(v12) => {
                                                let m3107 = f457(v3, v12, 0u64)?;
                                                match m3107 {
                                                    Ok(v14) => {
                                                        let m3108 = f452(f458(v14.clone()), Str::lit(">"));
                                                        match m3108 {
                                                            Ok(v16) => {
                                                                Ok(f582(f583(Str::lit("List"), f459(v14)), v16))
                                                            }
                                                            Err(v15) => {
                                                                Ok(Err::<Adt34, Bytes>(v15))
                                                            }
                                                        }
                                                    }
                                                    Err(v13) => {
                                                        Ok(Err::<Adt34, Bytes>(v13))
                                                    }
                                                }
                                            }
                                            Err(v11) => {
                                                Ok(Err::<Adt34, Bytes>(v11))
                                            }
                                        }
                                    } else {
                                        if f32(v10.clone(), f8(Str::lit("async_sequence"))) {
                                            let m3109 = f452(v9, Str::lit("<"));
                                            match m3109 {
                                                Ok(v18) => {
                                                    let m3110 = f457(v3, v18, 0u64)?;
                                                    match m3110 {
                                                        Ok(v20) => {
                                                            let m3111 = f452(f458(v20), Str::lit(">"));
                                                            match m3111 {
                                                                Ok(v22) => {
                                                                    Ok(f582(f584(f8(Str::lit("async_sequence types are not translated, because an async sequence is produced over time and is not a value (Web IDL 2.13.29)"))), v22))
                                                                }
                                                                Err(v21) => {
                                                                    Ok(Err::<Adt34, Bytes>(v21))
                                                                }
                                                            }
                                                        }
                                                        Err(v19) => {
                                                            Ok(Err::<Adt34, Bytes>(v19))
                                                        }
                                                    }
                                                }
                                                Err(v17) => {
                                                    Ok(Err::<Adt34, Bytes>(v17))
                                                }
                                            }
                                        } else {
                                            if f32(v10.clone(), f8(Str::lit("Promise"))) {
                                                let m3112 = f452(v9, Str::lit("<"));
                                                match m3112 {
                                                    Ok(v24) => {
                                                        let m3113 = f457(v3, v24, 0u64)?;
                                                        match m3113 {
                                                            Ok(v26) => {
                                                                let m3114 = f452(f458(v26), Str::lit(">"));
                                                                match m3114 {
                                                                    Ok(v28) => {
                                                                        Ok(f582(f584(f8(Str::lit("Promise types are not translated, because a promise is a deferred result, not a value (Web IDL 2.13.31)"))), v28))
                                                                    }
                                                                    Err(v27) => {
                                                                        Ok(Err::<Adt34, Bytes>(v27))
                                                                    }
                                                                }
                                                            }
                                                            Err(v25) => {
                                                                Ok(Err::<Adt34, Bytes>(v25))
                                                            }
                                                        }
                                                    }
                                                    Err(v23) => {
                                                        Ok(Err::<Adt34, Bytes>(v23))
                                                    }
                                                }
                                            } else {
                                                if f32(v10.clone(), f8(Str::lit("record"))) {
                                                    let m3115 = f452(v9, Str::lit("<"));
                                                    match m3115 {
                                                        Ok(v30) => {
                                                            let m3116 = v30.clone();
                                                            match m3116.uncons() {
                                                                None => {
                                                                    Ok(Err::<Adt34, Bytes>(f450(Str::lit("a string type"), v30)))
                                                                }
                                                                Some((v31, v32)) => {
                                                                    if if f32(f574(v31.clone()), f8(Str::lit("DOMString"))) {
                                                                        true
                                                                    } else {
                                                                        if f32(f574(v31.clone()), f8(Str::lit("USVString"))) {
                                                                            true
                                                                        } else {
                                                                            f32(f574(v31.clone()), f8(Str::lit("ByteString")))
                                                                        }
                                                                    } {
                                                                        let m3117 = f452(v32, Str::lit(","));
                                                                        match m3117 {
                                                                            Ok(v34) => {
                                                                                let m3118 = f457(v3, v34, 0u64)?;
                                                                                match m3118 {
                                                                                    Ok(v36) => {
                                                                                        let m3119 = f452(f458(v36.clone()), Str::lit(">"));
                                                                                        match m3119 {
                                                                                            Ok(v38) => {
                                                                                                let m3120 = f585(f574(v31));
                                                                                                match m3120 {
                                                                                                    None => {
                                                                                                        Ok(Err::<Adt34, Bytes>(f450(Str::lit("a string type"), v30)))
                                                                                                    }
                                                                                                    Some(v39) => {
                                                                                                        let v40: Adt15 = f586(f459(v36));
                                                                                                        Ok(f582(Adt15::C0(0u64, f7(List::cons(f8(Str::lit("(List (")), List::cons(f347(v39), List::cons(f8(Str::lit(" * ")), List::cons(f347(v40.clone()), List::cons(f8(Str::lit("))")), List::<Bytes>::nil())))))), f348(v40.clone()), f345(v40), Bytes::lit(&[])), v38))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            Err(v37) => {
                                                                                                Ok(Err::<Adt34, Bytes>(v37))
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    Err(v35) => {
                                                                                        Ok(Err::<Adt34, Bytes>(v35))
                                                                                    }
                                                                                }
                                                                            }
                                                                            Err(v33) => {
                                                                                Ok(Err::<Adt34, Bytes>(v33))
                                                                            }
                                                                        }
                                                                    } else {
                                                                        Ok(Err::<Adt34, Bytes>(f450(Str::lit("DOMString, USVString or ByteString"), v30)))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Err(v29) => {
                                                            Ok(Err::<Adt34, Bytes>(v29))
                                                        }
                                                    }
                                                } else {
                                                    if f32(v10.clone(), f8(Str::lit("unsigned"))) {
                                                        if f445(v9.clone(), Str::lit("short")) {
                                                            Ok(f582(f456(f8(Str::lit("Nat"))), f447(v9)))
                                                        } else {
                                                            if f445(v9.clone(), Str::lit("long")) {
                                                                let v41: List<Adt23> = f447(v9);
                                                                Ok(f582(f456(f8(Str::lit("Nat"))), if f445(v41.clone(), Str::lit("long")) {
                                                                    f447(v41)
                                                                } else {
                                                                    v41
                                                                }))
                                                            } else {
                                                                Ok(Err::<Adt34, Bytes>(f450(Str::lit("short or long"), v9)))
                                                            }
                                                        }
                                                    } else {
                                                        if f32(v10.clone(), f8(Str::lit("short"))) {
                                                            Ok(f582(f456(f8(Str::lit("Int"))), v9))
                                                        } else {
                                                            if f32(v10.clone(), f8(Str::lit("long"))) {
                                                                Ok(f582(f456(f8(Str::lit("Int"))), if f445(v9.clone(), Str::lit("long")) {
                                                                    f447(v9)
                                                                } else {
                                                                    v9
                                                                }))
                                                            } else {
                                                                if f32(v10.clone(), f8(Str::lit("unrestricted"))) {
                                                                    if f445(v9.clone(), Str::lit("float")) {
                                                                        Ok(f582(f587(false), f447(v9)))
                                                                    } else {
                                                                        if f445(v9.clone(), Str::lit("double")) {
                                                                            Ok(f582(f587(true), f447(v9)))
                                                                        } else {
                                                                            Ok(Err::<Adt34, Bytes>(f450(Str::lit("float or double"), v9)))
                                                                        }
                                                                    }
                                                                } else {
                                                                    let m3121 = f585(v10.clone());
                                                                    match m3121 {
                                                                        None => {
                                                                            let v42: Bytes = f581(v10);
                                                                            Ok(f582(Adt15::C0(0u64, f332(v42.clone())?, List::cons(v42.clone(), List::<Bytes>::nil()), Bytes::lit(&[]), v42), v9))
                                                                        }
                                                                        Some(v43) => {
                                                                            Ok(f582(v43, v9))
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
                    Ok(Err::<Adt34, Bytes>(v4))
                }
            }
        } else {
            let m3122 = f457(v3, v1, 0u64)?;
            match m3122 {
                Ok(v45) => {
                    let v46: Adt15 = f459(v45.clone());
                    let v48: Adt15 = {
                        let a3123 = f346(v46.clone());
                        let a3124 = 0u64;
                        if nat_eq(a3123, a3124) {
                            if f32(f345(v46.clone()), Bytes::lit(&[])) {
                                if f32(f588(v46.clone()), Bytes::lit(&[])) {
                                    Adt15::C0(1u64, Bytes::lit(&[]), f348(v46), f8(Str::lit("a union member type that is not a named definition gives its constructor no name, so the union is not translated yet (Web IDL 2.13.32)")), Bytes::lit(&[]))
                                } else {
                                    let m3125 = f469(false, f588(v46.clone()))?;
                                    match m3125 {
                                        None => {
                                            Adt15::C0(1u64, Bytes::lit(&[]), f348(v46.clone()), f7(List::cons(f8(Str::lit("the union member ")), List::cons(f588(v46), List::cons(f8(Str::lit(" has no constructor name")), List::<Bytes>::nil())))), Bytes::lit(&[]))
                                        }
                                        Some(v47) => {
                                            Adt15::C0(1u64, f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v47, List::cons(f8(Str::lit(", fields: [")), List::cons(f347(v46.clone()), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f348(v46), Bytes::lit(&[]), Bytes::lit(&[]))
                                        }
                                    }
                                }
                            } else {
                                Adt15::C0(1u64, Bytes::lit(&[]), f348(v46.clone()), f345(v46), Bytes::lit(&[]))
                            }
                        } else {
                            v46
                        }
                    };
                    if f445(f458(v45.clone()), Str::lit("or")) {
                        let m3126 = f457(v3, f447(f458(v45)), 2u64)?;
                        match m3126 {
                            Ok(v50) => {
                                Ok(Ok::<Adt34, Bytes>(Adt34::C0(Adt15::C0(1u64, if f32(f347(v48.clone()), Bytes::lit(&[])) {
                                    f347(f459(v50.clone()))
                                } else {
                                    if f32(f347(f459(v50.clone())), Bytes::lit(&[])) {
                                        f347(v48.clone())
                                    } else {
                                        f7(List::cons(f347(v48.clone()), List::cons(f8(Str::lit(",\u{a}")), List::cons(f347(f459(v50.clone())), List::<Bytes>::nil()))))
                                    }
                                }, f224(f348(v48.clone()), f348(f459(v50.clone()))), if f32(f345(v48.clone()), Bytes::lit(&[])) {
                                    f345(f459(v50.clone()))
                                } else {
                                    f345(v48)
                                }, Bytes::lit(&[])), f458(v50))))
                            }
                            Err(v49) => {
                                Ok(Err::<Adt34, Bytes>(v49))
                            }
                        }
                    } else {
                        let a3127 = v2;
                        let a3128 = 2u64;
                        if if nat_eq(a3127, a3128) {
                            f445(f458(v45.clone()), Str::lit(")"))
                        } else {
                            false
                        } {
                            Ok(Ok::<Adt34, Bytes>(Adt34::C0(v48, f447(f458(v45)))))
                        } else {
                            Ok(Err::<Adt34, Bytes>({
                                let a3129 = v2;
                                let a3130 = 1u64;
                                if nat_eq(a3129, a3130) {
                                    f450(Str::lit("or"), f458(v45))
                                } else {
                                    f450(Str::lit("or or )"), f458(v45))
                                }
                            }))
                        }
                    }
                }
                Err(v44) => {
                    Ok(Err::<Adt34, Bytes>(v44))
                }
            }
        }
    }
}

pub fn f458(v0: Adt34) -> List<Adt23> {
    let m3131 = v0;
    match m3131 {
        Adt34::C0(_, v2) => {
            v2
        }
    }
}

pub fn f459(v0: Adt34) -> Adt15 {
    let m3132 = v0;
    match m3132 {
        Adt34::C0(v1, _) => {
            v1
        }
    }
}

pub fn f460() -> List<Bytes> {
    List::cons(f8(Str::lit("Bool")), List::cons(f8(Str::lit("ContractViolation")), List::cons(f8(Str::lit("List")), List::cons(f8(Str::lit("Nat")), List::cons(f8(Str::lit("Option")), List::cons(f8(Str::lit("Prod")), List::cons(f8(Str::lit("ReasoningFailure")), List::cons(f8(Str::lit("Result")), List::<Bytes>::nil()))))))))
}

pub fn f461(v0: Bytes) -> R<Bytes> {
    let m3135 = {
        let a3133 = v0.clone();
        let a3134 = 0u64;
        index_bytes(a3133, a3134)
    };
    match m3135 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            let a3141 = {
                let a3136 = f8(Str::lit("u"));
                let a3137 = f589(8u64, f62(v1)?, 4u64, Bytes::lit(&[]));
                append_bytes(a3136, a3137)
            };
            let a3142 = f46(v0.clone(), 1u64, {
                let a3139 = {
                    let a3138 = v0;
                    length_bytes(a3138)
                };
                let a3140 = 1u64;
                nat_sub(a3139, a3140)
            });
            Ok(append_bytes(a3141, a3142))
        }
    }
}

pub fn f462(v0: List<Adt14>) -> R<List<Adt14>> {
    let m3143 = v0;
    match m3143.uncons() {
        None => {
            Ok(List::<Adt14>::nil())
        }
        Some((v1, v2)) => {
            f590(v1, f462(v2)?)
        }
    }
}

pub fn f463(v0: List<Adt14>, v1: List<Adt14>) -> List<Adt14> {
    let m3144 = v0;
    match m3144.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f463(v3, v1))
        }
    }
}

pub fn f464(v0: Adt13) -> List<Adt14> {
    let m3145 = v0;
    match m3145 {
        Adt13::C0(_, _, _, v4, _, _) => {
            v4
        }
    }
}

pub fn f465(v0: List<Adt13>, v1: u64, v2: Bytes) -> List<Adt14> {
    let m3146 = v0;
    match m3146.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v3, v4)) => {
            let a3147 = f219(v3.clone());
            let a3148 = v1;
            if if nat_eq(a3147, a3148) {
                f32(f221(v3.clone()), v2.clone())
            } else {
                false
            } {
                f463(f464(v3), f465(v4, v1, v2))
            } else {
                f465(v4, v1, v2)
            }
        }
    }
}

pub fn f466(v0: List<Adt13>, v1: List<Adt13>, v2: Bytes) -> List<Adt14> {
    let m3149 = v0;
    match m3149.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v3, v4)) => {
            let a3150 = f219(v3.clone());
            let a3151 = 8u64;
            if if nat_eq(a3150, a3151) {
                f32(f221(v3.clone()), v2.clone())
            } else {
                false
            } {
                f463(f591(v1.clone(), f231(v3)), f466(v4, v1, v2))
            } else {
                f466(v4, v1, v2)
            }
        }
    }
}

pub fn f467(v0: Adt14) -> Adt15 {
    let m3152 = v0;
    match m3152 {
        Adt14::C0(_, v2, _) => {
            v2
        }
    }
}

pub fn f468(v0: Adt14) -> Bytes {
    let m3153 = v0;
    match m3153 {
        Adt14::C0(v1, _, _) => {
            v1
        }
    }
}

pub fn f469(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f592(v1)
    } else {
        f472(v1)
    }
}

pub fn f470(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3154 = v1;
    match m3154.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3155 = f469(v0, v2)?;
            match m3155 {
                None => {
                    f470(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f470(v0, v3)?))
                }
            }
        }
    }
}

pub fn f471(v0: Adt14) -> u64 {
    let m3156 = v0;
    match m3156 {
        Adt14::C0(_, _, v3) => {
            v3
        }
    }
}

pub fn f472(v0: Bytes) -> R<Option<Bytes>> {
    let m3157 = f593(v0)?;
    match m3157 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            Ok(Some(f594(v1)?))
        }
    }
}

pub fn f473(v0: bool, v1: Adt2) -> R<Option<Bytes>> {
    let m3158 = v1;
    match m3158 {
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
                let a3169 = if f113(v9, {
                    let a3167 = 0u64;
                    let a3168 = v10;
                    nat_lt(a3167, a3168)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a3170 = f286(false, v10)?;
                append_bytes(a3169, a3170)
            }))
        }
        Adt2::C4(v3, v4, v5) => {
            let m3161 = f592({
                let a3159 = f8(Str::lit("n"));
                let a3160 = f287(false, v4, v5)?;
                append_bytes(a3159, a3160)
            })?;
            match m3161 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a3165 = f8(Str::lit("nMinus"));
                        let a3166 = f46(v6.clone(), 1u64, {
                            let a3163 = {
                                let a3162 = v6;
                                length_bytes(a3162)
                            };
                            let a3164 = 1u64;
                            nat_sub(a3163, a3164)
                        });
                        append_bytes(a3165, a3166)
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
            f469(v0, v2)
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

pub fn f474(v0: Adt2) -> R<Bytes> {
    f196(v0)
}

pub fn f475(v0: u64) -> R<bool> {
    f351(v0, f595())
}

pub fn f476(v0: u64, v1: Bytes, p3181: u64, p3182: u64, p3183: bool) -> R<bool> {
    let m3180 = (p3181, (p3182, p3183));
    {
        let mut m3180 = m3180;
        loop {
            let step = {
                let (v2, (v3, v4)) = m3180;
                let m3171 = v3;
                if m3171 == 0 {
                    Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                } else {
                    let v5 = m3171 - 1;
                    let v6: u64 = f596(v1.clone(), v2)?;
                    if v4 {
                        let a3172 = v0;
                        let a3173 = v6;
                        if nat_lt(a3172, a3173) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a3174 = v2;
                                let a3175 = 1u64;
                                nat_add(a3174, a3175)?
                            }, (v5, false)))
                        }
                    } else {
                        let a3176 = v0;
                        let a3177 = v6;
                        if nat_le(a3176, a3177) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(true))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a3178 = v2;
                                let a3179 = 1u64;
                                nat_add(a3178, a3179)?
                            }, (v5, true)))
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

pub fn f477(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    if f133(v0.clone(), v1, v2) {
        Ok(None::<u64>)
    } else {
        let a3185 = v2;
        let a3186 = {
            let a3184 = v0;
            length_bytes(a3184)
        };
        if nat_lt(a3185, a3186) {
            Ok(Some({
                let a3187 = v2;
                let a3188 = 1u64;
                nat_add(a3187, a3188)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f478(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f233(f163(v0, v1)?)? {
        Ok(Some({
            let a3189 = v1;
            let a3190 = 1u64;
            nat_add(a3189, a3190)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f479(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let m3191 = f419(v0, v2)?;
    match m3191 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            let a3192 = v2;
            let a3193 = v1;
            if if nat_eq(a3192, a3193) {
                f597({
                    let (h3194, _) = v3;
                    h3194
                })?
            } else {
                f598({
                    let (h3195, _) = v3;
                    h3195
                })?
            } {
                Ok(Some({
                    let a3197 = v2;
                    let a3198 = {
                        let (_, h3196) = v3;
                        h3196
                    };
                    nat_add(a3197, a3198)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f480(v0: Bytes, v1: bool, v2: u64) -> R<(u64, u64)> {
    let (h3199, _) = f274(Fn6::F599(v1, v0.clone()), {
        let a3201 = {
            let a3200 = v0;
            length_bytes(a3200)
        };
        let a3202 = 1u64;
        nat_add(a3201, a3202)?
    }, (v2, 0u64))?;
    Ok(h3199)
}

pub fn f481(v0: u64) -> R<Bytes> {
    let a3203 = v0;
    let a3204 = 128u64;
    if nat_lt(a3203, a3204) {
        Ok(f600(f601(), v0))
    } else {
        let a3205 = v0;
        let a3206 = 2048u64;
        if nat_lt(a3205, a3206) {
            let a3214 = f600(f602(), {
                let a3212 = {
                    let a3210 = {
                        let a3207 = v0;
                        let a3208 = 64u64;
                        let a3209 = 0u64;
                        nat_quot(a3207, a3208, a3209)
                    };
                    let a3211 = 2u64;
                    nat_sub(a3210, a3211)
                };
                let a3213 = 2u64;
                nat_mul(a3212, a3213)?
            });
            let a3215 = f603(v0)?;
            Ok(append_bytes(a3214, a3215))
        } else {
            let a3216 = v0;
            let a3217 = 65536u64;
            if nat_lt(a3216, a3217) {
                let a3228 = f600(f604(), {
                    let a3221 = {
                        let a3218 = v0;
                        let a3219 = 4096u64;
                        let a3220 = 0u64;
                        nat_quot(a3218, a3219, a3220)
                    };
                    let a3222 = 3u64;
                    nat_mul(a3221, a3222)?
                });
                let a3229 = {
                    let a3226 = f603({
                        let a3223 = v0;
                        let a3224 = 64u64;
                        let a3225 = 0u64;
                        nat_quot(a3223, a3224, a3225)
                    })?;
                    let a3227 = f603(v0)?;
                    append_bytes(a3226, a3227)
                };
                Ok(append_bytes(a3228, a3229))
            } else {
                let a3243 = f600(Bytes::lit(&[0xf0, 0xf1, 0xf2, 0xf3, 0xf4]), {
                    let a3230 = v0;
                    let a3231 = 262144u64;
                    let a3232 = 0u64;
                    nat_quot(a3230, a3231, a3232)
                });
                let a3244 = {
                    let a3241 = f603({
                        let a3233 = v0;
                        let a3234 = 4096u64;
                        let a3235 = 0u64;
                        nat_quot(a3233, a3234, a3235)
                    })?;
                    let a3242 = {
                        let a3239 = f603({
                            let a3236 = v0;
                            let a3237 = 64u64;
                            let a3238 = 0u64;
                            nat_quot(a3236, a3237, a3238)
                        })?;
                        let a3240 = f603(v0)?;
                        append_bytes(a3239, a3240)
                    };
                    append_bytes(a3241, a3242)
                };
                Ok(append_bytes(a3243, a3244))
            }
        }
    }
}

pub fn f482(v0: bool, v1: u64, v2: Bytes, v3: u64) -> R<Option<u64>> {
    f605(v2, v0, v1, v3)
}

pub fn f483(v0: Bytes) -> Result<Option<Adt31>, Bytes> {
    Err::<Option<Adt31>, Bytes>(v0)
}

pub fn f484(p3248: Bytes, p3249: u64) -> R<Result<Bytes, Bytes>> {
    let m3247 = (p3248, p3249);
    {
        let mut m3247 = m3247;
        loop {
            let step = {
                let (v0, v1) = m3247;
                let m3245 = v1;
                if m3245 == 0 {
                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f606(f8(Str::lit("the built-in datatype hierarchy is deeper than XSD 1.1 Part 2 section 3 defines")))))
                } else {
                    let v2 = m3245 - 1;
                    if f325(v0.clone(), List::cons(f8(Str::lit("NMTOKENS")), List::cons(f8(Str::lit("IDREFS")), List::cons(f8(Str::lit("ENTITIES")), List::<Bytes>::nil())))) {
                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("(List Bytes)")))))
                    } else {
                        if f32(v0.clone(), f8(Str::lit("nonNegativeInteger"))) {
                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Nat")))))
                        } else {
                            if f32(v0.clone(), f8(Str::lit("integer"))) {
                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Int")))))
                            } else {
                                if f32(v0.clone(), f8(Str::lit("decimal"))) {
                                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f607())))
                                } else {
                                    if f32(v0.clone(), f8(Str::lit("boolean"))) {
                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Bool")))))
                                    } else {
                                        if f325(v0.clone(), List::cons(f8(Str::lit("float")), List::cons(f8(Str::lit("double")), List::<Bytes>::nil()))) {
                                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f606(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because its value space holds positive and negative zero, the infinities and NaN, which no exact LexLean number type holds (XSD 1.1 Part 2, sections 3.3.4 and 3.3.5)")), List::<Bytes>::nil())))))))
                                        } else {
                                            if f325(v0.clone(), List::cons(f8(Str::lit("anySimpleType")), List::cons(f8(Str::lit("anyAtomicType")), List::<Bytes>::nil()))) {
                                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f606(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because it is the union of every simple value space and has no one LexLean type (XSD 1.1 Part 2, sections 3.2.1 and 3.2.2)")), List::<Bytes>::nil())))))))
                                            } else {
                                                let m3246 = f608(v0.clone())?;
                                                match m3246 {
                                                    None => {
                                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f606(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not a built-in datatype of XSD 1.1 Part 2")), List::<Bytes>::nil())))))))
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
                Ok(next) => { m3247 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f485(v0: Bytes) -> Result<List<Adt2>, Bytes> {
    Err::<List<Adt2>, Bytes>(v0)
}

pub fn f486(v0: List<Adt26>, v1: List<Adt26>) -> List<Adt26> {
    let m3250 = v0;
    match m3250.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f486(v3, v1))
        }
    }
}

pub fn f487(v0: List<Adt27>, v1: Bytes) -> List<Adt26> {
    let m3251 = v0;
    match m3251.uncons() {
        None => {
            List::<Adt26>::nil()
        }
        Some((v2, v3)) => {
            if f122(match v2.clone() {
                Adt27::C0(_, h3252, _, _, _) => {
                    h3252
                }
            }, f609()) {
                let m3253 = f381(v2.clone(), Str::lit("name"));
                match m3253 {
                    None => {
                        f487(v3, v1)
                    }
                    Some(v4) => {
                        List::cons(Adt26::C0(match v2.clone() {
                            Adt27::C0(_, h3254, _, _, _) => {
                                h3254
                            }
                        }, v1.clone(), v4, v2), f487(v3, v1))
                    }
                }
            } else {
                f487(v3, v1)
            }
        }
    }
}

pub fn f488(v0: u64, v1: Adt25, v2: Adt26) -> R<Result<List<Adt28>, Bytes>> {
    let v3: Bytes = f125(match v2.clone() {
        Adt26::C0(_, _, h3255, _) => {
            h3255
        }
    })?;
    let v4: Adt27 = match v2.clone() {
        Adt26::C0(_, _, _, h3256) => {
            h3256
        }
    };
    let v5: Bytes = match v2.clone() {
        Adt26::C0(_, h3257, _, _) => {
            h3257
        }
    };
    let v6: Bytes = match v2.clone() {
        Adt26::C0(h3258, _, _, _) => {
            h3258
        }
    };
    if f32(v6.clone(), f8(Str::lit("complexType"))) {
        f610(v0, v1, v2)
    } else {
        if f32(v6.clone(), f8(Str::lit("simpleType"))) {
            let m3259 = f611(v3, v4, v5)?;
            match m3259 {
                Ok(v8) => {
                    Ok(Ok::<List<Adt28>, Bytes>(List::cons(v8, List::<Adt28>::nil())))
                }
                Err(v7) => {
                    Ok(Err::<List<Adt28>, Bytes>(v7))
                }
            }
        } else {
            if f32(v6.clone(), f8(Str::lit("element"))) {
                let m3260 = f381(v4.clone(), Str::lit("type"));
                match m3260 {
                    None => {
                        let m3261 = f612(v0, v1, v3, v5, v4)?;
                        match m3261 {
                            Ok(v11) => {
                                Ok(Ok::<List<Adt28>, Bytes>(match v11 {
                                    Adt35::C0(_, h3262) => {
                                        h3262
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
                    let m3263 = f381(v4.clone(), Str::lit("type"));
                    match m3263 {
                        None => {
                            let m3264 = f613(v1, v5, v3, v4)?;
                            match m3264 {
                                Ok(v14) => {
                                    Ok(Ok::<List<Adt28>, Bytes>(match v14 {
                                        Adt35::C0(_, h3265) => {
                                            h3265
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
                        let m3266 = f614(v4)?;
                        match m3266 {
                            Ok(v16) => {
                                let m3267 = f615(v16, List::cons(f8(Str::lit("sequence")), List::cons(f8(Str::lit("choice")), List::cons(f8(Str::lit("all")), List::<Bytes>::nil()))));
                                match m3267 {
                                    None => {
                                        Ok(Err::<List<Adt28>, Bytes>(f616(f8(Str::lit("a model group definition must have a sequence, choice or all child (XSD 1.1 Part 1, section 3.7.2)")), v3)))
                                    }
                                    Some(v17) => {
                                        f617(v0, v1, v3, v5, v17)
                                    }
                                }
                            }
                            Err(v15) => {
                                Ok(Err::<List<Adt28>, Bytes>(v15))
                            }
                        }
                    } else {
                        let m3268 = f614(v4)?;
                        match m3268 {
                            Ok(v19) => {
                                let m3269 = f618(v0, v1, v3, v5, v19, f619())?;
                                match m3269 {
                                    Ok(v21) => {
                                        Ok(Ok::<List<Adt28>, Bytes>(match v21 {
                                            Adt36::C0(_, h3270, _, _, _) => {
                                                h3270
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

pub fn f489(v0: List<Adt28>, v1: List<Adt28>) -> List<Adt28> {
    let m3271 = v0;
    match m3271.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f489(v3, v1))
        }
    }
}

pub fn f490(p3274: List<Adt28>, p3275: List<Adt28>) -> List<Adt28> {
    let m3273 = (p3274, p3275);
    {
        let mut m3273 = m3273;
        loop {
            let step = {
                let (v0, v1) = m3273;
                let m3272 = v0;
                match m3272.uncons() {
                    None => {
                        Err::<(List<Adt28>, List<Adt28>), List<Adt28>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt28>, List<Adt28>), List<Adt28>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3273 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f491(p3279: List<Adt28>, v1: Bytes) -> Option<Adt28> {
    let m3278 = p3279;
    {
        let mut m3278 = m3278;
        loop {
            let step = {
                let v0 = m3278;
                let m3276 = v0;
                match m3276.uncons() {
                    None => {
                        Err::<List<Adt28>, Option<Adt28>>(None::<Adt28>)
                    }
                    Some((v2, v3)) => {
                        if f32(match v2.clone() {
                            Adt28::C0(h3277, _, _, _) => {
                                h3277
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
                Ok(next) => { m3278 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f492() -> Bytes {
    f8(Str::lit("XmlNode"))
}

pub fn f493(v0: List<Adt28>) -> List<Bytes> {
    let m3280 = v0;
    match m3280.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f499(match v1 {
                Adt28::C0(_, h3281, _, _) => {
                    h3281
                }
            }, f493(v2))
        }
    }
}

pub fn f494() -> List<Bytes> {
    List::cons(f8(Str::lit("    {name: element, fields: [Bytes, Bytes, (List ((Bytes * Bytes) * Bytes)), (List XmlNode)]}")), List::cons(f8(Str::lit("    {name: text, fields: [Bytes]}")), List::<Bytes>::nil()))
}

pub fn f495(p3285: u64, v1: List<Adt28>, p3286: List<Bytes>, p3287: List<Bytes>) -> List<Bytes> {
    let m3284 = (p3285, (p3286, p3287));
    {
        let mut m3284 = m3284;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3284;
                let m3282 = v0;
                if m3282 == 0 {
                    Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v3)
                } else {
                    let v4 = m3282 - 1;
                    let m3283 = v2;
                    match m3283.uncons() {
                        None => {
                            Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v3)
                        }
                        Some((v5, v6)) => {
                            if f122(v5.clone(), v3.clone()) {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v3)))
                            } else {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (f499(f496(v1.clone(), v5.clone()), v6), List::cons(v5, v3))))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3284 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f496(v0: List<Adt28>, v1: Bytes) -> List<Bytes> {
    let m3288 = f491(v0, v1);
    match m3288 {
        None => {
            List::<Bytes>::nil()
        }
        Some(v2) => {
            match v2 {
                Adt28::C0(_, h3289, _, _) => {
                    h3289
                }
            }
        }
    }
}

pub fn f497(v0: List<(List<Bytes>, List<Bytes>)>, v1: List<Bytes>) -> Option<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)> {
    let m3290 = v0;
    match m3290.uncons() {
        None => {
            None::<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)>
        }
        Some((v2, v3)) => {
            let m3292 = f253({
                let (_, h3291) = v2.clone();
                h3291
            }, v1.clone());
            match m3292 {
                None => {
                    Some(({
                        let (h3293, _) = v2;
                        h3293
                    }, v3))
                }
                Some(_) => {
                    let m3294 = f497(v3, v1);
                    match m3294 {
                        None => {
                            None::<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h3295, _) = v5;
                                h3295
                            }, List::cons(v2, {
                                let (_, h3296) = v5;
                                h3296
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f498(v0: List<Adt28>, v1: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m3297 = v1.clone();
    match m3297.uncons() {
        None => {
            Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3298 = v3;
            match m3298.uncons() {
                None => {
                    let m3301 = f491(v0, v2.clone());
                    match m3301 {
                        None => {
                            Err::<List<Bytes>, Bytes>(f621(v2))
                        }
                        Some(v6) => {
                            if f122(v2.clone(), match v6.clone() {
                                Adt28::C0(_, h3302, _, _) => {
                                    h3302
                                }
                            }) {
                                let m3304 = match v6.clone() {
                                    Adt28::C0(_, _, _, h3303) => {
                                        h3303
                                    }
                                };
                                match m3304.uncons() {
                                    None => {
                                        Err::<List<Bytes>, Bytes>(f621(v2))
                                    }
                                    Some((_, _)) => {
                                        Ok::<List<Bytes>, Bytes>(List::cons(f343(v2, match v6 {
                                            Adt28::C0(_, _, _, h3305) => {
                                                h3305
                                            }
                                        }), List::<Bytes>::nil()))
                                    }
                                }
                            } else {
                                Ok::<List<Bytes>, Bytes>(List::cons(match v6 {
                                    Adt28::C0(_, _, h3306, _) => {
                                        h3306
                                    }
                                }, List::<Bytes>::nil()))
                            }
                        }
                    }
                }
                Some((_, _)) => {
                    f620(v0, v1, {
                        let a3299 = v2;
                        let a3300 = f8(Str::lit("Group"));
                        append_bytes(a3299, a3300)
                    })
                }
            }
        }
    }
}

pub fn f499(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m3307 = v0;
    match m3307.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f499(v3, v1))
        }
    }
}

pub fn f500(v0: List<Adt28>, v1: List<Bytes>, v2: List<Bytes>) -> List<Bytes> {
    let m3308 = v1;
    match m3308.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            f499(f622(f496(v0.clone(), v3), v2.clone()), f500(v0, v4, v2))
        }
    }
}

pub fn f501(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>, v2: Bytes) -> List<Bytes> {
    let m3309 = v1;
    match m3309.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if f32(v3.clone(), v2.clone()) {
                true
            } else {
                if f122(v3.clone(), f623(v0.clone(), v2.clone())) {
                    f122(v2.clone(), f623(v0.clone(), v3.clone()))
                } else {
                    false
                }
            } {
                List::cons(v3, f501(v0, v4, v2))
            } else {
                f501(v0, v4, v2)
            }
        }
    }
}

pub fn f502(v0: Bytes, p3318: u64, p3319: u64, p3320: Option<u64>) -> R<Option<u64>> {
    let m3317 = (p3318, (p3319, p3320));
    {
        let mut m3317 = m3317;
        loop {
            let step = {
                let (v1, (v2, v3)) = m3317;
                let m3310 = v2;
                if m3310 == 0 {
                    Err::<(u64, (u64, Option<u64>)), R<Option<u64>>>(Ok(v3))
                } else {
                    let v4 = m3310 - 1;
                    let v5: u64 = f163(v0.clone(), v1)?;
                    let a3311 = v5;
                    let a3312 = 256u64;
                    if nat_eq(a3311, a3312) {
                        Err::<(u64, (u64, Option<u64>)), R<Option<u64>>>(Ok(v3))
                    } else {
                        Ok::<(u64, (u64, Option<u64>)), R<Option<u64>>>(({
                            let a3313 = v1;
                            let a3314 = 1u64;
                            nat_add(a3313, a3314)?
                        }, (v4, {
                            let a3315 = v5;
                            let a3316 = 47u64;
                            if nat_eq(a3315, a3316) {
                                Some(v1)
                            } else {
                                v3
                            }
                        })))
                    }
                }
            };
            match step {
                Ok(next) => { m3317 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f503(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a3321 = v0.clone();
        length_bytes(a3321)
    };
    let a3325 = {
        let a3322 = v1;
        let a3323 = 2u64;
        let a3324 = 0u64;
        nat_rem(a3322, a3323, a3324)
    };
    let a3326 = 0u64;
    if nat_eq(a3325, a3326) {
        let (_, h3327) = {
            let (h3328, _) = f624(Fn14::F625(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h3328
        };
        Ok(h3327)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f504(v0: u64) -> bool {
    let a3329 = v0;
    let a3330 = 8u64;
    if nat_eq(a3329, a3330) {
        true
    } else {
        let a3331 = v0;
        let a3332 = 9u64;
        if nat_eq(a3331, a3332) {
            true
        } else {
            let a3333 = v0;
            let a3334 = 10u64;
            if nat_eq(a3333, a3334) {
                true
            } else {
                let a3335 = v0;
                let a3336 = 12u64;
                if nat_eq(a3335, a3336) {
                    true
                } else {
                    let a3337 = v0;
                    let a3338 = 13u64;
                    nat_eq(a3337, a3338)
                }
            }
        }
    }
}

pub fn f505(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3341 = {
        let a3339 = v0;
        let a3340 = v1;
        index_bytes(a3339, a3340)
    };
    match m3341 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f626(v2)? {
                Ok(Some({
                    let a3342 = v1;
                    let a3343 = 1u64;
                    nat_add(a3342, a3343)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f506(v0: u8) -> R<u64> {
    let a3390 = {
        let v1: u8 = v0;
        let a3388 = {
            let a3364 = {
                let a3352 = {
                    let a3346 = {
                        let a3344 = v1;
                        let a3345 = 128u8;
                        fixed_u8::bit_and(a3344, a3345)
                    };
                    let a3347 = 128u8;
                    if equal(a3346, a3347) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3353 = {
                    let a3350 = {
                        let a3348 = v1;
                        let a3349 = 64u8;
                        fixed_u8::bit_and(a3348, a3349)
                    };
                    let a3351 = 64u8;
                    if equal(a3350, a3351) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3352, a3353)?
            };
            let a3365 = {
                let a3362 = {
                    let a3356 = {
                        let a3354 = v1;
                        let a3355 = 32u8;
                        fixed_u8::bit_and(a3354, a3355)
                    };
                    let a3357 = 32u8;
                    if equal(a3356, a3357) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3363 = {
                    let a3360 = {
                        let a3358 = v1;
                        let a3359 = 16u8;
                        fixed_u8::bit_and(a3358, a3359)
                    };
                    let a3361 = 16u8;
                    if equal(a3360, a3361) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3362, a3363)?
            };
            nat_add(a3364, a3365)?
        };
        let a3389 = {
            let a3386 = {
                let a3374 = {
                    let a3368 = {
                        let a3366 = v1;
                        let a3367 = 8u8;
                        fixed_u8::bit_and(a3366, a3367)
                    };
                    let a3369 = 8u8;
                    if equal(a3368, a3369) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3375 = {
                    let a3372 = {
                        let a3370 = v1;
                        let a3371 = 4u8;
                        fixed_u8::bit_and(a3370, a3371)
                    };
                    let a3373 = 4u8;
                    if equal(a3372, a3373) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3374, a3375)?
            };
            let a3387 = {
                let a3384 = {
                    let a3378 = {
                        let a3376 = v1;
                        let a3377 = 2u8;
                        fixed_u8::bit_and(a3376, a3377)
                    };
                    let a3379 = 2u8;
                    if equal(a3378, a3379) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3385 = {
                    let a3382 = {
                        let a3380 = v1;
                        let a3381 = 1u8;
                        fixed_u8::bit_and(a3380, a3381)
                    };
                    let a3383 = 1u8;
                    if equal(a3382, a3383) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3384, a3385)?
            };
            nat_add(a3386, a3387)?
        };
        nat_add(a3388, a3389)?
    };
    let a3391 = 48u64;
    Ok(nat_sub(a3390, a3391))
}

pub fn f507(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a3392 = v0.clone();
        length_bytes(a3392)
    };
    let a3393 = v1;
    let a3394 = v2;
    if if nat_lt(a3393, a3394) {
        let a3399 = f111(v0, {
            let a3397 = v2;
            let a3398 = {
                let a3395 = v1;
                let a3396 = 1u64;
                nat_add(a3395, a3396)?
            };
            nat_sub(a3397, a3398)
        })?;
        let a3400 = 48u64;
        nat_eq(a3399, a3400)
    } else {
        false
    } {
        Ok(Some({
            let a3401 = v1;
            let a3402 = 1u64;
            nat_add(a3401, a3402)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f508(v0: Adt30) -> R<Adt30> {
    let v1: u32 = f627(match v0.clone() {
        Adt30::C0(_, _, _, _, _, _, _, _, _, h3403) => {
            h3403
        }
    });
    let v2: u32 = f395(match v0.clone() {
        Adt30::C0(_, _, _, _, _, _, _, h3404, _, _) => {
            h3404
        }
    }, f395(f628(match v0.clone() {
        Adt30::C0(_, _, _, _, h3405, _, _, _, _, _) => {
            h3405
        }
    }), f395(f629(match v0.clone() {
        Adt30::C0(_, _, _, _, h3406, _, _, _, _, _) => {
            h3406
        }
    }, match v0.clone() {
        Adt30::C0(_, _, _, _, _, h3407, _, _, _, _) => {
            h3407
        }
    }, match v0.clone() {
        Adt30::C0(_, _, _, _, _, _, h3408, _, _, _) => {
            h3408
        }
    }), f395(v1, {
        let m3411 = match v0.clone() {
            Adt30::C0(_, _, _, _, _, _, _, _, h3410, _) => {
                h3410
            }
        };
        match m3411 {
            Adt29::C0(h3409, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3409
            }
        }
    }))));
    let v3: u32 = f395(f630(match v0.clone() {
        Adt30::C0(h3412, _, _, _, _, _, _, _, _, _) => {
            h3412
        }
    }), f631(match v0.clone() {
        Adt30::C0(h3413, _, _, _, _, _, _, _, _, _) => {
            h3413
        }
    }, match v0.clone() {
        Adt30::C0(_, h3414, _, _, _, _, _, _, _, _) => {
            h3414
        }
    }, match v0.clone() {
        Adt30::C0(_, _, h3415, _, _, _, _, _, _, _) => {
            h3415
        }
    }));
    Ok(Adt30::C0(f395(v2, v3), match v0.clone() {
        Adt30::C0(h3416, _, _, _, _, _, _, _, _, _) => {
            h3416
        }
    }, match v0.clone() {
        Adt30::C0(_, h3417, _, _, _, _, _, _, _, _) => {
            h3417
        }
    }, match v0.clone() {
        Adt30::C0(_, _, h3418, _, _, _, _, _, _, _) => {
            h3418
        }
    }, f395(match v0.clone() {
        Adt30::C0(_, _, _, h3419, _, _, _, _, _, _) => {
            h3419
        }
    }, v2), match v0.clone() {
        Adt30::C0(_, _, _, _, h3420, _, _, _, _, _) => {
            h3420
        }
    }, match v0.clone() {
        Adt30::C0(_, _, _, _, _, h3421, _, _, _, _) => {
            h3421
        }
    }, match v0.clone() {
        Adt30::C0(_, _, _, _, _, _, h3422, _, _, _) => {
            h3422
        }
    }, f632(match v0.clone() {
        Adt30::C0(_, _, _, _, _, _, _, _, h3423, _) => {
            h3423
        }
    }), {
        let a3425 = match v0 {
            Adt30::C0(_, _, _, _, _, _, _, _, _, h3424) => {
                h3424
            }
        };
        let a3426 = 1u64;
        nat_add(a3425, a3426)?
    }))
}

pub fn f509(v0: u32) -> u64 {
    let m3428 = {
        let a3427 = v0;
        fixed_u64::convert(i128::from(a3427))
    };
    match m3428 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f510(v0: u64) -> u32 {
    let m3432 = {
        let a3431 = {
            let a3429 = v0;
            let a3430 = 4294967295u64;
            fixed_u64::bit_and(a3429, a3430)
        };
        fixed_u32::convert(i128::from(a3431))
    };
    match m3432 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f511(v0: u64) -> u32 {
    let (_, h3433) = {
        let (_, h3434) = f633(Fn15::F634, 32u64, (v0, (1u32, 0u32)));
        h3434
    };
    h3433
}

pub fn f512(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a3439 = f635(v0, 24u32);
    let a3440 = {
        let a3437 = f635(v1, 16u32);
        let a3438 = {
            let a3435 = f635(v2, 8u32);
            let a3436 = v3;
            fixed_u32::bit_or(a3435, a3436)
        };
        fixed_u32::bit_or(a3437, a3438)
    };
    fixed_u32::bit_or(a3439, a3440)
}

pub fn f513(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a3441 = v2;
    let a3442 = v1;
    if nat_lt(a3441, a3442) {
        let m3445 = {
            let a3443 = v0;
            let a3444 = v2;
            index_bytes(a3443, a3444)
        };
        match m3445 {
            None => {
                0u32
            }
            Some(v3) => {
                let m3447 = {
                    let a3446 = v3;
                    fixed_u32::convert(i128::from(a3446))
                };
                match m3447 {
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
        let a3448 = v2;
        let a3449 = v1;
        if nat_eq(a3448, a3449) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f514(v0: List<u64>) -> R<Bytes> {
    let m3450 = f636(v0, List::<u64>::nil());
    match m3450.uncons() {
        None => {
            Ok(Bytes::lit(&[0x30]))
        }
        Some((v1, v2)) => {
            let a3451 = f91(v1)?;
            let a3452 = f637(v2)?;
            Ok(append_bytes(a3451, a3452))
        }
    }
}

pub fn f515(p3455: u64, p3456: List<u64>, v2: u64) -> R<List<u64>> {
    let m3454 = (p3455, p3456);
    {
        let mut m3454 = m3454;
        loop {
            let step = {
                let (v0, v1) = m3454;
                let m3453 = v0;
                if m3453 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v1))
                } else {
                    let v3 = m3453 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, f638(v1, v2, 0u64)?))
                }
            };
            match step {
                Ok(next) => { m3454 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f516(v0: u64, v1: u64) -> List<u64> {
    let m3457 = v0;
    if m3457 == 0 {
        List::<u64>::nil()
    } else {
        let v2 = m3457 - 1;
        let a3458 = v1;
        let a3459 = 0u64;
        if nat_eq(a3458, a3459) {
            List::<u64>::nil()
        } else {
            List::cons({
                let a3460 = v1;
                let a3461 = f639();
                let a3462 = 0u64;
                nat_rem(a3460, a3461, a3462)
            }, f516(v2, {
                let a3463 = v1;
                let a3464 = f639();
                let a3465 = 0u64;
                nat_quot(a3463, a3464, a3465)
            }))
        }
    }
}

pub fn f517(p3476: Bytes, p3477: u64) -> R<Bytes> {
    let m3475 = (p3476, p3477);
    {
        let mut m3475 = m3475;
        loop {
            let step = {
                let (v0, v1) = m3475;
                let m3466 = v1;
                if m3466 == 0 {
                    Err::<(Bytes, u64), R<Bytes>>(Ok(v0))
                } else {
                    let v2 = m3466 - 1;
                    let a3470 = f111(v0.clone(), {
                        let a3468 = {
                            let a3467 = v0.clone();
                            length_bytes(a3467)
                        };
                        let a3469 = 1u64;
                        nat_sub(a3468, a3469)
                    })?;
                    let a3471 = 48u64;
                    if nat_eq(a3470, a3471) {
                        Ok::<(Bytes, u64), R<Bytes>>((f46(v0.clone(), 0u64, {
                            let a3473 = {
                                let a3472 = v0;
                                length_bytes(a3472)
                            };
                            let a3474 = 1u64;
                            nat_sub(a3473, a3474)
                        }), v2))
                    } else {
                        Err::<(Bytes, u64), R<Bytes>>(Ok(v0))
                    }
                }
            };
            match step {
                Ok(next) => { m3475 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f518(v0: u64) -> Bytes {
    let m3478 = v0;
    if m3478 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m3478 - 1;
        let a3479 = Bytes::lit(&[0x30]);
        let a3480 = f518(v1);
        append_bytes(a3479, a3480)
    }
}

pub fn f519(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a3525 = {
            let a3501 = {
                let a3489 = {
                    let a3483 = {
                        let a3481 = v3;
                        let a3482 = 128u8;
                        fixed_u8::bit_and(a3481, a3482)
                    };
                    let a3484 = 128u8;
                    if equal(a3483, a3484) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3490 = {
                    let a3487 = {
                        let a3485 = v3;
                        let a3486 = 64u8;
                        fixed_u8::bit_and(a3485, a3486)
                    };
                    let a3488 = 64u8;
                    if equal(a3487, a3488) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3489, a3490)?
            };
            let a3502 = {
                let a3499 = {
                    let a3493 = {
                        let a3491 = v3;
                        let a3492 = 32u8;
                        fixed_u8::bit_and(a3491, a3492)
                    };
                    let a3494 = 32u8;
                    if equal(a3493, a3494) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3500 = {
                    let a3497 = {
                        let a3495 = v3;
                        let a3496 = 16u8;
                        fixed_u8::bit_and(a3495, a3496)
                    };
                    let a3498 = 16u8;
                    if equal(a3497, a3498) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3499, a3500)?
            };
            nat_add(a3501, a3502)?
        };
        let a3526 = {
            let a3523 = {
                let a3511 = {
                    let a3505 = {
                        let a3503 = v3;
                        let a3504 = 8u8;
                        fixed_u8::bit_and(a3503, a3504)
                    };
                    let a3506 = 8u8;
                    if equal(a3505, a3506) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3512 = {
                    let a3509 = {
                        let a3507 = v3;
                        let a3508 = 4u8;
                        fixed_u8::bit_and(a3507, a3508)
                    };
                    let a3510 = 4u8;
                    if equal(a3509, a3510) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3511, a3512)?
            };
            let a3524 = {
                let a3521 = {
                    let a3515 = {
                        let a3513 = v3;
                        let a3514 = 2u8;
                        fixed_u8::bit_and(a3513, a3514)
                    };
                    let a3516 = 2u8;
                    if equal(a3515, a3516) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3522 = {
                    let a3519 = {
                        let a3517 = v3;
                        let a3518 = 1u8;
                        fixed_u8::bit_and(a3517, a3518)
                    };
                    let a3520 = 1u8;
                    if equal(a3519, a3520) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3521, a3522)?
            };
            nat_add(a3523, a3524)?
        };
        nat_add(a3525, a3526)?
    };
    let a3527 = v4;
    let a3528 = 34u64;
    if nat_eq(a3527, a3528) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a3529 = v4;
        let a3530 = 92u64;
        if nat_eq(a3529, a3530) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a3531 = v4;
            let a3532 = 8u64;
            if nat_eq(a3531, a3532) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a3533 = v4;
                let a3534 = 12u64;
                if nat_eq(a3533, a3534) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a3535 = v4;
                    let a3536 = 10u64;
                    if nat_eq(a3535, a3536) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a3537 = v4;
                        let a3538 = 13u64;
                        if nat_eq(a3537, a3538) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a3539 = v4;
                            let a3540 = 9u64;
                            if nat_eq(a3539, a3540) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a3541 = v4;
                                let a3542 = 32u64;
                                if nat_lt(a3541, a3542) {
                                    let a3543 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a3544 = f640(v2)?;
                                    Ok(append_bytes(a3543, a3544))
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

pub fn f520(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m3545 = v0.clone();
    match m3545.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a3548 = {
                let (h3546, _) = v2;
                h3546
            };
            let a3549 = {
                let (h3547, _) = v1.clone();
                h3547
            };
            if equal(a3548, a3549) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f521(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f641(f642(Fn16::F643, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m3554 = {
        let (h3550, _) = f644(Fn17::F645, {
            let a3552 = {
                let a3551 = v0;
                length_list(a3551)
            };
            let a3553 = 1u64;
            nat_add(a3552, a3553)?
        }, v1)?;
        h3550
    };
    match m3554.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f522(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a3559 = {
        let a3556 = f289({
            let (h3555, _) = v0;
            h3555
        })?;
        let a3557 = Bytes::lit(&[0x3a]);
        append_bytes(a3556, a3557)
    };
    let a3560 = {
        let (_, h3558) = v0;
        h3558
    };
    Ok(append_bytes(a3559, a3560))
}

pub fn f523(v0: Bytes, p3586: u64, p3587: u64, p3588: Bytes) -> R<Bytes> {
    let m3585 = (p3586, (p3587, p3588));
    {
        let mut m3585 = m3585;
        loop {
            let step = {
                let (v1, (v2, v3)) = m3585;
                let m3561 = v2;
                if m3561 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m3561 - 1;
                    let m3564 = {
                        let a3562 = v0.clone();
                        let a3563 = v1;
                        index_bytes(a3562, a3563)
                    };
                    match m3564 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v6: u64 = f62(v5)?;
                            if f183({
                                let a3565 = v6;
                                let a3566 = 34u64;
                                nat_eq(a3565, a3566)
                            }, {
                                let a3567 = v6;
                                let a3568 = 92u64;
                                nat_eq(a3567, a3568)
                            }) {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a3569 = v1;
                                    let a3570 = 1u64;
                                    nat_add(a3569, a3570)?
                                }, (v4, {
                                    let a3573 = v3;
                                    let a3574 = {
                                        let a3571 = f8(Str::lit("\\"));
                                        let a3572 = f46(v0.clone(), v1, 1u64);
                                        append_bytes(a3571, a3572)
                                    };
                                    append_bytes(a3573, a3574)
                                })))
                            } else {
                                let a3575 = v6;
                                let a3576 = 10u64;
                                if nat_eq(a3575, a3576) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a3577 = v1;
                                        let a3578 = 1u64;
                                        nat_add(a3577, a3578)?
                                    }, (v4, {
                                        let a3579 = v3;
                                        let a3580 = f8(Str::lit("\\n"));
                                        append_bytes(a3579, a3580)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a3581 = v1;
                                        let a3582 = 1u64;
                                        nat_add(a3581, a3582)?
                                    }, (v4, {
                                        let a3583 = v3;
                                        let a3584 = f46(v0.clone(), v1, 1u64);
                                        append_bytes(a3583, a3584)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3585 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f524(v0: Bytes) -> Bytes {
    if f41(v0.clone(), Bytes::lit(&[0x0d])) {
        f46(v0.clone(), 0u64, {
            let a3590 = {
                let a3589 = v0;
                length_bytes(a3589)
            };
            let a3591 = 1u64;
            nat_sub(a3590, a3591)
        })
    } else {
        v0
    }
}

pub fn f525(v0: Fn10, p3595: List<Bytes>, p3596: List<(Bytes, ())>) -> List<Bytes> {
    let m3594 = (p3595, p3596);
    {
        let mut m3594 = m3594;
        loop {
            let step = {
                let (v1, v2) = m3594;
                let m3592 = v2;
                match m3592.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c3593: Fn10 = v0.clone();
                            c3593.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3594 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f526(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h3597, _) = v1;
        h3597
    }, v0)
}

pub fn f527(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f646(Fn18::F647, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f528(v0: Fn11, p3601: List<List<(Bytes, ())>>, p3602: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m3600 = (p3601, p3602);
    {
        let mut m3600 = m3600;
        loop {
            let step = {
                let (v1, v2) = m3600;
                let m3598 = v2;
                match m3598.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c3599: Fn11 = v0.clone();
                            c3599.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3600 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f529(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f530(v0: Fn12, p3607: u64, p3608: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m3606 = (p3607, p3608);
    {
        let mut m3606 = m3606;
        loop {
            let step = {
                let (v1, v2) = m3606;
                let m3603 = v1;
                if m3603 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3603 - 1;
                    let m3605 = {
                        let c3604: Fn12 = v0.clone();
                        c3604.apply(v2.clone())?
                    };
                    match m3605 {
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
                Ok(next) => { m3606 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f531(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f648(v0)
}

pub fn f532(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f649(Fn19::F650, List::<(Bytes, ())>::nil(), v0)
}

pub fn f533(v0: Fn13, p3612: List<(Bytes, ())>, p3613: List<Bytes>) -> List<(Bytes, ())> {
    let m3611 = (p3612, p3613);
    {
        let mut m3611 = m3611;
        loop {
            let step = {
                let (v1, v2) = m3611;
                let m3609 = v2;
                match m3609.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c3610: Fn13 = v0.clone();
                            c3610.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3611 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f534(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f535(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3616 = {
        let a3614 = v0;
        let a3615 = v1;
        index_bytes(a3614, a3615)
    };
    match m3616 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f62(v2)?;
            if f113({
                let a3617 = 128u64;
                let a3618 = v3;
                nat_le(a3617, a3618)
            }, {
                let a3619 = v3;
                let a3620 = 191u64;
                nat_le(a3619, a3620)
            }) {
                Ok(Some({
                    let a3621 = v3;
                    let a3622 = 128u64;
                    nat_sub(a3621, a3622)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f536(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f113({
        let a3623 = v1;
        let a3624 = v0;
        nat_le(a3623, a3624)
    }, f113({
        let a3625 = v0;
        let a3626 = v2;
        nat_le(a3625, a3626)
    }, f183({
        let a3627 = v0;
        let a3628 = 55296u64;
        nat_lt(a3627, a3628)
    }, {
        let a3629 = 57343u64;
        let a3630 = v0;
        nat_lt(a3629, a3630)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f537(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f651(v0, v1)
}

pub fn f538(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f304(f209(v0, v1)?) {
        Ok(Some({
            let a3631 = v1;
            let a3632 = 1u64;
            nat_add(a3631, a3632)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f539(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f652(v0, v1)
}

pub fn f540(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a3633 = f209(v0, v1)?;
    let a3634 = 32u64;
    if nat_eq(a3633, a3634) {
        Ok(Some({
            let a3635 = v1;
            let a3636 = 1u64;
            nat_add(a3635, a3636)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f541(v0: u64) -> bool {
    let a3637 = v0;
    let a3638 = 44u64;
    if if if nat_eq(a3637, a3638) {
        true
    } else {
        let a3639 = v0;
        let a3640 = 91u64;
        nat_eq(a3639, a3640)
    } {
        true
    } else {
        let a3641 = v0;
        let a3642 = 93u64;
        if nat_eq(a3641, a3642) {
            true
        } else {
            let a3643 = v0;
            let a3644 = 123u64;
            nat_eq(a3643, a3644)
        }
    } {
        true
    } else {
        let a3645 = v0;
        let a3646 = 125u64;
        nat_eq(a3645, a3646)
    }
}

pub fn f542(v0: u64) -> u64 {
    let a3647 = v0;
    let a3648 = 45u64;
    if nat_eq(a3647, a3648) {
        1u64
    } else {
        let a3649 = v0;
        let a3650 = 43u64;
        if nat_eq(a3649, a3650) {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f543(v0: u64) -> u64 {
    let a3651 = 49u64;
    let a3652 = v0;
    if if nat_le(a3651, a3652) {
        let a3653 = v0;
        let a3654 = 57u64;
        nat_le(a3653, a3654)
    } else {
        false
    } {
        let a3655 = v0;
        let a3656 = 48u64;
        nat_sub(a3655, a3656)
    } else {
        0u64
    }
}

pub fn f544(v0: Bytes, v1: u64, v2: u64, v3: bool, v4: u64) -> R<Adt22> {
    let m3657 = f653(v0.clone(), v1, v2)?;
    match m3657 {
        Adt38::C0(v6, v7) => {
            let v8: u64 = v7;
            let v9: u64 = f654(v0.clone(), v8, v2)?;
            if f655(v0.clone(), v8, v2)? {
                let m3661 = f656(v0.clone(), v8, v2, v3, {
                    let a3659 = {
                        let a3658 = v0;
                        length_bytes(a3658)
                    };
                    let a3660 = 2u64;
                    nat_add(a3659, a3660)?
                }, Bytes::lit(&[]), v6)?;
                match m3661 {
                    Adt39::C0(v11, v12, v13) => {
                        Ok(Adt22::C0(Adt2::C6(f7(List::cons(v11, List::cons({
                            let a3662 = v4;
                            let a3663 = 1u64;
                            if nat_eq(a3662, a3663) {
                                Bytes::lit(&[])
                            } else {
                                Bytes::lit(&[0x0a])
                            }
                        }, List::cons({
                            let a3664 = v4;
                            let a3665 = 2u64;
                            if nat_eq(a3664, a3665) {
                                f546(v12)
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
                Ok(Adt22::C0(Adt2::C6({
                    let a3666 = v4;
                    let a3667 = 2u64;
                    if nat_eq(a3666, a3667) {
                        f546(v6)
                    } else {
                        Bytes::lit(&[])
                    }
                }), {
                    let a3668 = f209(v0, v9)?;
                    let a3669 = 256u64;
                    if nat_eq(a3668, a3669) {
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

pub fn f545(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f657(v0, v1)
}

pub fn f546(v0: u64) -> Bytes {
    let m3670 = v0;
    if m3670 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m3670 - 1;
        let a3671 = Bytes::lit(&[0x0a]);
        let a3672 = f546(v1);
        append_bytes(a3671, a3672)
    }
}

pub fn f547(v0: Bytes, v1: u64) -> R<u64> {
    let (_, h3673) = {
        let (h3674, _) = f274(Fn6::F658(v0.clone()), {
            let a3676 = {
                let a3675 = v0;
                length_bytes(a3675)
            };
            let a3677 = 1u64;
            nat_add(a3676, a3677)?
        }, (v1, 0u64))?;
        h3674
    };
    Ok(h3673)
}

pub fn f548(v0: u64, v1: u64) -> u64 {
    let a3678 = v0;
    let a3679 = v1;
    if nat_lt(a3678, a3679) {
        v1
    } else {
        v0
    }
}

pub fn f549(v0: Bytes, v1: u64, v2: u64) -> R<Adt22> {
    if f550(v0.clone()) {
        Ok(Adt22::C0(Adt2::C0, v2))
    } else {
        if f554(v0.clone()) {
            Ok(Adt22::C0(Adt2::C2(true), v2))
        } else {
            if f555(v0.clone()) {
                Ok(Adt22::C0(Adt2::C2(false), v2))
            } else {
                if f553(v0.clone()) {
                    Ok(Adt22::C1(f137(Str::lit("an infinite or not-a-number float is not read: Studio.Value has none (YAML 1.2.2 section 10.3.2)"), v1)?))
                } else {
                    if if f162(v0.clone(), 0u64, Str::lit("0o")) {
                        let a3681 = 2u64;
                        let a3682 = {
                            let a3680 = v0.clone();
                            length_bytes(a3680)
                        };
                        nat_lt(a3681, a3682)
                    } else {
                        false
                    } {
                        f659(v0, 8u64, v1, v2)
                    } else {
                        if if f162(v0.clone(), 0u64, Str::lit("0x")) {
                            let a3684 = 2u64;
                            let a3685 = {
                                let a3683 = v0.clone();
                                length_bytes(a3683)
                            };
                            nat_lt(a3684, a3685)
                        } else {
                            false
                        } {
                            f659(v0, 16u64, v1, v2)
                        } else {
                            let m3686 = f552(v0.clone())?;
                            match m3686 {
                                None => {
                                    Ok(Adt22::C0(Adt2::C6(v0), v2))
                                }
                                Some(v3) => {
                                    let m3687 = f114(v3.clone(), 0u64)?;
                                    match m3687 {
                                        None => {
                                            Ok(Adt22::C1(f660(v1)?))
                                        }
                                        Some(v4) => {
                                            let a3690 = {
                                                let (h3688, _) = v4;
                                                h3688
                                            };
                                            let a3691 = {
                                                let a3689 = v3;
                                                length_bytes(a3689)
                                            };
                                            if nat_eq(a3690, a3691) {
                                                Ok(Adt22::C0({
                                                    let (_, h3692) = v4;
                                                    h3692
                                                }, v2))
                                            } else {
                                                Ok(Adt22::C1(f660(v1)?))
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

pub fn f550(v0: Bytes) -> bool {
    let a3694 = {
        let a3693 = v0.clone();
        length_bytes(a3693)
    };
    let a3695 = 0u64;
    if nat_eq(a3694, a3695) {
        true
    } else {
        f325(v0, List::cons(f8(Str::lit("~")), List::cons(f8(Str::lit("null")), List::cons(f8(Str::lit("Null")), List::cons(f8(Str::lit("NULL")), List::<Bytes>::nil())))))
    }
}

pub fn f551(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a3696 = v0.clone();
        length_bytes(a3696)
    };
    let v2: u64 = f209(v0.clone(), 0u64)?;
    let v3: u64 = {
        let a3697 = v2;
        let a3698 = 43u64;
        if if nat_eq(a3697, a3698) {
            true
        } else {
            let a3699 = v2;
            let a3700 = 45u64;
            nat_eq(a3699, a3700)
        } {
            1u64
        } else {
            0u64
        }
    };
    let a3701 = v3;
    let a3702 = v1;
    if if nat_lt(a3701, a3702) {
        let a3703 = f182(v0.clone(), v3)?;
        let a3704 = v1;
        nat_eq(a3703, a3704)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a3705 = 2u64;
        let a3706 = v1;
        if nat_lt(a3705, a3706) {
            if if f162(v0.clone(), 0u64, Str::lit("0o")) {
                let a3708 = {
                    let (h3707, _) = f661(v0.clone(), 8u64, 2u64)?;
                    h3707
                };
                let a3709 = v1;
                nat_eq(a3708, a3709)
            } else {
                false
            } {
                Ok(true)
            } else {
                if f162(v0.clone(), 0u64, Str::lit("0x")) {
                    let a3711 = {
                        let (h3710, _) = f661(v0, 16u64, 2u64)?;
                        h3710
                    };
                    let a3712 = v1;
                    Ok(nat_eq(a3711, a3712))
                } else {
                    Ok(false)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f552(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a3713 = v0.clone();
        length_bytes(a3713)
    };
    let v2: u64 = f209(v0.clone(), 0u64)?;
    let v3: u64 = {
        let a3714 = v2;
        let a3715 = 43u64;
        if if nat_eq(a3714, a3715) {
            true
        } else {
            let a3716 = v2;
            let a3717 = 45u64;
            nat_eq(a3716, a3717)
        } {
            1u64
        } else {
            0u64
        }
    };
    let v4: u64 = f182(v0.clone(), v3)?;
    let v5: bool = {
        let a3718 = f209(v0.clone(), v4)?;
        let a3719 = 46u64;
        nat_eq(a3718, a3719)
    };
    let v6: u64 = if v5 {
        let a3720 = v4;
        let a3721 = 1u64;
        nat_add(a3720, a3721)?
    } else {
        v4
    };
    let v7: u64 = if v5 {
        f182(v0.clone(), v6)?
    } else {
        v4
    };
    let v8: u64 = f209(v0.clone(), v7)?;
    let v9: bool = {
        let a3722 = v8;
        let a3723 = 101u64;
        if nat_eq(a3722, a3723) {
            true
        } else {
            let a3724 = v8;
            let a3725 = 69u64;
            nat_eq(a3724, a3725)
        }
    };
    let v10: u64 = f209(v0.clone(), {
        let a3726 = v7;
        let a3727 = 1u64;
        nat_add(a3726, a3727)?
    })?;
    let v11: bool = if v9 {
        let a3728 = v10;
        let a3729 = 43u64;
        if nat_eq(a3728, a3729) {
            true
        } else {
            let a3730 = v10;
            let a3731 = 45u64;
            nat_eq(a3730, a3731)
        }
    } else {
        false
    };
    let v12: u64 = if v9 {
        if v11 {
            let a3732 = v7;
            let a3733 = 2u64;
            nat_add(a3732, a3733)?
        } else {
            let a3734 = v7;
            let a3735 = 1u64;
            nat_add(a3734, a3735)?
        }
    } else {
        v7
    };
    let v13: u64 = if v9 {
        f182(v0.clone(), v12)?
    } else {
        v7
    };
    let v14: Bytes = f321(v0.clone(), v3, v4);
    let v15: u64 = f184(v14.clone())?;
    let a3736 = v13;
    let a3737 = v1;
    if if if nat_eq(a3736, a3737) {
        let a3738 = v3;
        let a3739 = v4;
        if nat_lt(a3738, a3739) {
            true
        } else {
            if v5 {
                let a3740 = v6;
                let a3741 = v7;
                nat_lt(a3740, a3741)
            } else {
                false
            }
        }
    } else {
        false
    } {
        if f211(v9) {
            true
        } else {
            let a3742 = v12;
            let a3743 = v13;
            nat_lt(a3742, a3743)
        }
    } else {
        false
    } {
        Ok(Some(f7(List::cons({
            let a3744 = v2;
            let a3745 = 45u64;
            if nat_eq(a3744, a3745) {
                Bytes::lit(&[0x2d])
            } else {
                Bytes::lit(&[])
            }
        }, List::cons({
            let a3747 = v15;
            let a3748 = {
                let a3746 = v14.clone();
                length_bytes(a3746)
            };
            if nat_eq(a3747, a3748) {
                Bytes::lit(&[0x30])
            } else {
                f321(v14.clone(), v15, {
                    let a3749 = v14;
                    length_bytes(a3749)
                })
            }
        }, List::cons({
            let a3750 = v6;
            let a3751 = v7;
            if nat_lt(a3750, a3751) {
                let a3752 = Bytes::lit(&[0x2e]);
                let a3753 = f321(v0.clone(), v6, v7);
                append_bytes(a3752, a3753)
            } else {
                Bytes::lit(&[])
            }
        }, List::cons(if v9 {
            let a3756 = if if v11 {
                let a3754 = v10;
                let a3755 = 45u64;
                nat_eq(a3754, a3755)
            } else {
                false
            } {
                Bytes::lit(&[0x65, 0x2d])
            } else {
                Bytes::lit(&[0x65])
            };
            let a3757 = f321(v0, v12, v13);
            append_bytes(a3756, a3757)
        } else {
            Bytes::lit(&[])
        }, List::<Bytes>::nil())))))))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f553(v0: Bytes) -> bool {
    f325(v0, List::cons(f8(Str::lit(".inf")), List::cons(f8(Str::lit(".Inf")), List::cons(f8(Str::lit(".INF")), List::cons(f8(Str::lit("+.inf")), List::cons(f8(Str::lit("+.Inf")), List::cons(f8(Str::lit("+.INF")), List::cons(f8(Str::lit("-.inf")), List::cons(f8(Str::lit("-.Inf")), List::cons(f8(Str::lit("-.INF")), List::cons(f8(Str::lit(".nan")), List::cons(f8(Str::lit(".NaN")), List::cons(f8(Str::lit(".NAN")), List::<Bytes>::nil())))))))))))))
}

pub fn f554(v0: Bytes) -> bool {
    f325(v0, List::cons(f8(Str::lit("true")), List::cons(f8(Str::lit("True")), List::cons(f8(Str::lit("TRUE")), List::<Bytes>::nil()))))
}

pub fn f555(v0: Bytes) -> bool {
    f325(v0, List::cons(f8(Str::lit("false")), List::cons(f8(Str::lit("False")), List::cons(f8(Str::lit("FALSE")), List::<Bytes>::nil()))))
}

pub fn f556(v0: Bytes, p3816: u64, v2: u64, v3: bool, p3817: u64, p3818: Bytes) -> R<Adt32> {
    let m3815 = (p3816, (p3817, p3818));
    {
        let mut m3815 = m3815;
        loop {
            let step = {
                let (v1, (v4, v5)) = m3815;
                let m3758 = v4;
                if m3758 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C1(f137(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                } else {
                    let v6 = m3758 - 1;
                    let v7: u64 = f209(v0.clone(), v1)?;
                    let a3759 = v7;
                    let a3760 = 34u64;
                    if nat_eq(a3759, a3760) {
                        Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C0(v5, {
                            let a3761 = v1;
                            let a3762 = 1u64;
                            nat_add(a3761, a3762)?
                        })))
                    } else {
                        let a3763 = v7;
                        let a3764 = 256u64;
                        if nat_eq(a3763, a3764) {
                            Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C1(f137(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                        } else {
                            let a3765 = v7;
                            let a3766 = 92u64;
                            if nat_eq(a3765, a3766) {
                                let v8: u64 = f209(v0.clone(), {
                                    let a3767 = v1;
                                    let a3768 = 1u64;
                                    nat_add(a3767, a3768)?
                                })?;
                                let m3769 = f662(v8)?;
                                match m3769 {
                                    None => {
                                        let a3774 = v8;
                                        let a3775 = 10u64;
                                        if nat_eq(a3774, a3775) {
                                            if v3 {
                                                Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C1(f663(v1)?)))
                                            } else {
                                                let m3778 = f664(v0.clone(), {
                                                    let a3776 = v1;
                                                    let a3777 = 1u64;
                                                    nat_add(a3776, a3777)?
                                                }, v2, v6, 0u64)?;
                                                match m3778 {
                                                    Adt38::C0(v11, v12) => {
                                                        Ok::<(u64, (u64, Bytes)), R<Adt32>>((v12, (v6, {
                                                            let a3779 = v5;
                                                            let a3780 = f546(v11);
                                                            append_bytes(a3779, a3780)
                                                        })))
                                                    }
                                                    Adt38::C1(v10) => {
                                                        Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C1(v10)))
                                                    }
                                                }
                                            }
                                        } else {
                                            let v13: u64 = {
                                                let a3781 = v8;
                                                let a3782 = 120u64;
                                                if nat_eq(a3781, a3782) {
                                                    2u64
                                                } else {
                                                    let a3783 = v8;
                                                    let a3784 = 117u64;
                                                    if nat_eq(a3783, a3784) {
                                                        4u64
                                                    } else {
                                                        let a3785 = v8;
                                                        let a3786 = 85u64;
                                                        if nat_eq(a3785, a3786) {
                                                            8u64
                                                        } else {
                                                            0u64
                                                        }
                                                    }
                                                }
                                            };
                                            let a3787 = v13;
                                            let a3788 = 0u64;
                                            if nat_eq(a3787, a3788) {
                                                Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C1(f137(Str::lit("an escape must be one of YAML 1.2.2 [42] to [62]"), v1)?)))
                                            } else {
                                                let m3791 = f665(v0.clone(), {
                                                    let a3789 = v1;
                                                    let a3790 = 2u64;
                                                    nat_add(a3789, a3790)?
                                                }, v13, 0u64)?;
                                                match m3791 {
                                                    None => {
                                                        Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C1(f137(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                    }
                                                    Some(v14) => {
                                                        let a3792 = 1114111u64;
                                                        let a3793 = v14;
                                                        if if nat_lt(a3792, a3793) {
                                                            true
                                                        } else {
                                                            let a3794 = 55296u64;
                                                            let a3795 = v14;
                                                            if nat_le(a3794, a3795) {
                                                                let a3796 = v14;
                                                                let a3797 = 57343u64;
                                                                nat_le(a3796, a3797)
                                                            } else {
                                                                false
                                                            }
                                                        } {
                                                            Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C1(f137(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                        } else {
                                                            Ok::<(u64, (u64, Bytes)), R<Adt32>>(({
                                                                let a3800 = {
                                                                    let a3798 = v1;
                                                                    let a3799 = 2u64;
                                                                    nat_add(a3798, a3799)?
                                                                };
                                                                let a3801 = v13;
                                                                nat_add(a3800, a3801)?
                                                            }, (v6, {
                                                                let a3802 = v5;
                                                                let a3803 = f481(v14)?;
                                                                append_bytes(a3802, a3803)
                                                            })))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Some(v9) => {
                                        Ok::<(u64, (u64, Bytes)), R<Adt32>>(({
                                            let a3770 = v1;
                                            let a3771 = 2u64;
                                            nat_add(a3770, a3771)?
                                        }, (v6, {
                                            let a3772 = v5;
                                            let a3773 = f481(v9)?;
                                            append_bytes(a3772, a3773)
                                        })))
                                    }
                                }
                            } else {
                                if if f304(v7) {
                                    true
                                } else {
                                    let a3804 = v7;
                                    let a3805 = 10u64;
                                    nat_eq(a3804, a3805)
                                } {
                                    let v15: u64 = f307(v0.clone(), v1)?;
                                    let a3806 = f209(v0.clone(), v15)?;
                                    let a3807 = 10u64;
                                    if nat_eq(a3806, a3807) {
                                        if v3 {
                                            Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C1(f663(v1)?)))
                                        } else {
                                            let m3808 = f664(v0.clone(), v15, v2, v6, 0u64)?;
                                            match m3808 {
                                                Adt38::C0(v17, v18) => {
                                                    Ok::<(u64, (u64, Bytes)), R<Adt32>>((v18, (v6, {
                                                        let a3809 = v5;
                                                        let a3810 = f564(v17);
                                                        append_bytes(a3809, a3810)
                                                    })))
                                                }
                                                Adt38::C1(v16) => {
                                                    Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C1(v16)))
                                                }
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (u64, Bytes)), R<Adt32>>((v15, (v6, {
                                            let a3811 = v5;
                                            let a3812 = f321(v0.clone(), v1, v15);
                                            append_bytes(a3811, a3812)
                                        })))
                                    }
                                } else {
                                    let v19: u64 = f666(v0.clone(), v1)?;
                                    Ok::<(u64, (u64, Bytes)), R<Adt32>>((v19, (v6, {
                                        let a3813 = v5;
                                        let a3814 = f321(v0.clone(), v1, v19);
                                        append_bytes(a3813, a3814)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3815 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f557(v0: Bytes, p3846: u64, v2: u64, v3: bool, p3847: u64, p3848: Bytes) -> R<Adt32> {
    let m3845 = (p3846, (p3847, p3848));
    {
        let mut m3845 = m3845;
        loop {
            let step = {
                let (v1, (v4, v5)) = m3845;
                let m3819 = v4;
                if m3819 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C1(f137(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                } else {
                    let v6 = m3819 - 1;
                    let v7: u64 = f209(v0.clone(), v1)?;
                    let a3820 = v7;
                    let a3821 = 39u64;
                    if nat_eq(a3820, a3821) {
                        let a3824 = f209(v0.clone(), {
                            let a3822 = v1;
                            let a3823 = 1u64;
                            nat_add(a3822, a3823)?
                        })?;
                        let a3825 = 39u64;
                        if nat_eq(a3824, a3825) {
                            Ok::<(u64, (u64, Bytes)), R<Adt32>>(({
                                let a3826 = v1;
                                let a3827 = 2u64;
                                nat_add(a3826, a3827)?
                            }, (v6, {
                                let a3828 = v5;
                                let a3829 = Bytes::lit(&[0x27]);
                                append_bytes(a3828, a3829)
                            })))
                        } else {
                            Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C0(v5, {
                                let a3830 = v1;
                                let a3831 = 1u64;
                                nat_add(a3830, a3831)?
                            })))
                        }
                    } else {
                        let a3832 = v7;
                        let a3833 = 256u64;
                        if nat_eq(a3832, a3833) {
                            Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C1(f137(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                        } else {
                            if if f304(v7) {
                                true
                            } else {
                                let a3834 = v7;
                                let a3835 = 10u64;
                                nat_eq(a3834, a3835)
                            } {
                                let v8: u64 = f307(v0.clone(), v1)?;
                                let a3836 = f209(v0.clone(), v8)?;
                                let a3837 = 10u64;
                                if nat_eq(a3836, a3837) {
                                    if v3 {
                                        Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C1(f663(v1)?)))
                                    } else {
                                        let m3838 = f664(v0.clone(), v8, v2, v6, 0u64)?;
                                        match m3838 {
                                            Adt38::C0(v10, v11) => {
                                                Ok::<(u64, (u64, Bytes)), R<Adt32>>((v11, (v6, {
                                                    let a3839 = v5;
                                                    let a3840 = f564(v10);
                                                    append_bytes(a3839, a3840)
                                                })))
                                            }
                                            Adt38::C1(v9) => {
                                                Err::<(u64, (u64, Bytes)), R<Adt32>>(Ok(Adt32::C1(v9)))
                                            }
                                        }
                                    }
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Adt32>>((v8, (v6, {
                                        let a3841 = v5;
                                        let a3842 = f321(v0.clone(), v1, v8);
                                        append_bytes(a3841, a3842)
                                    })))
                                }
                            } else {
                                let v12: u64 = f667(v0.clone(), v1)?;
                                Ok::<(u64, (u64, Bytes)), R<Adt32>>((v12, (v6, {
                                    let a3843 = v5;
                                    let a3844 = f321(v0.clone(), v1, v12);
                                    append_bytes(a3843, a3844)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3845 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f558(v0: Bytes, v1: u64, v2: u64) -> R<Result<u64, Bytes>> {
    let v3: (u64, bool) = {
        let (h3849, _) = f668(Fn20::F669(v2, v0.clone()), {
            let a3851 = {
                let a3850 = v0.clone();
                length_bytes(a3850)
            };
            let a3852 = 1u64;
            nat_add(a3851, a3852)?
        }, (v1, false))?;
        h3849
    };
    let (_, h3853) = v3;
    if h3853 {
        if f207(v0, {
            let (h3854, _) = v3;
            h3854
        })? {
            Ok(Err::<u64, Bytes>(f137(Str::lit("a document marker cannot appear inside a flow collection (YAML 1.2.2 [206])"), {
                let (h3855, _) = v3;
                h3855
            })?))
        } else {
            Ok(Err::<u64, Bytes>(f137(Str::lit("a line of a flow collection is indented less than its node (YAML 1.2.2 [80])"), {
                let (h3856, _) = v3;
                h3856
            })?))
        }
    } else {
        Ok(Ok::<u64, Bytes>({
            let (h3857, _) = v3;
            h3857
        }))
    }
}

pub fn f559(v0: Bytes, v1: u64, v2: u64) -> R<Adt32> {
    let a3858 = f209(v0.clone(), v1)?;
    let a3859 = 33u64;
    if nat_eq(a3858, a3859) {
        let m3860 = f322(v0.clone(), v1)?;
        match m3860 {
            Ok(v4) => {
                let m3861 = f558(v0.clone(), v4, v2)?;
                match m3861 {
                    Ok(v6) => {
                        let a3862 = f209(v0.clone(), v6)?;
                        let a3863 = 58u64;
                        if nat_eq(a3862, a3863) {
                            let a3866 = f317(v0, v1)?;
                            let a3867 = {
                                let a3864 = v1;
                                let a3865 = 1u64;
                                nat_add(a3864, a3865)?
                            };
                            if nat_eq(a3866, a3867) {
                                Ok(Adt32::C1(f137(Str::lit("an empty node with the non-specific tag ! is not read (YAML 1.2.2 [100])"), v1)?))
                            } else {
                                Ok(Adt32::C0(Bytes::lit(&[]), v6))
                            }
                        } else {
                            f670(v0, v6, v2)
                        }
                    }
                    Err(v5) => {
                        Ok(Adt32::C1(v5))
                    }
                }
            }
            Err(v3) => {
                Ok(Adt32::C1(v3))
            }
        }
    } else {
        f670(v0, v1, v2)
    }
}

pub fn f560(v0: u64) -> R<bool> {
    let a3868 = f320(f8(Str::lit(",[]{}-?:#&*!|>'\"@`")), v0)?;
    let a3869 = 18u64;
    if nat_lt(a3868, a3869) {
        Ok(true)
    } else {
        let a3870 = v0;
        let a3871 = 37u64;
        Ok(nat_eq(a3870, a3871))
    }
}

pub fn f561(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f671(v1, v0, v2)
}

pub fn f562(v0: u64) -> R<Bytes> {
    f137(Str::lit("a mapping value is not allowed here: a plain scalar holds ': ' (YAML 1.2.2 [130])"), v0)
}

pub fn f563(v0: Bytes, v1: u64) -> R<(u64, u64)> {
    let (h3872, _) = f274(Fn6::F672(v0.clone()), {
        let a3874 = {
            let a3873 = v0;
            length_bytes(a3873)
        };
        let a3875 = 1u64;
        nat_add(a3874, a3875)?
    }, (v1, 0u64))?;
    Ok(h3872)
}

pub fn f564(v0: u64) -> Bytes {
    let a3876 = v0;
    let a3877 = 0u64;
    if nat_eq(a3876, a3877) {
        Bytes::lit(&[0x20])
    } else {
        f546(v0)
    }
}

pub fn f565(v0: Bytes, v1: bool, v2: u64) -> R<Option<u64>> {
    f673(v0, v1, v2)
}

pub fn f566(v0: Bytes, p3882: List<Bytes>, p3883: u64) -> R<u64> {
    let m3881 = (p3882, p3883);
    {
        let mut m3881 = m3881;
        loop {
            let step = {
                let (v1, v2) = m3881;
                let m3878 = v1;
                match m3878.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<u64>>(Ok(0u64))
                    }
                    Some((v3, v4)) => {
                        if f32(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<u64>>(Ok(v2))
                        } else {
                            Ok::<(List<Bytes>, u64), R<u64>>((v4, {
                                let a3879 = v2;
                                let a3880 = 1u64;
                                nat_add(a3879, a3880)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3881 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f567(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a3885 = v2;
    let a3886 = {
        let a3884 = v0.clone();
        length_bytes(a3884)
    };
    if if nat_lt(a3885, a3886) {
        f211({
            let a3887 = f209(v0, v2)?;
            let a3888 = v1;
            nat_eq(a3887, a3888)
        })
    } else {
        false
    } {
        Ok(Some({
            let a3889 = v2;
            let a3890 = 1u64;
            nat_add(a3889, a3890)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f568(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f674(v1, v0, v2)
}

pub fn f569(v0: u64) -> bool {
    if f570(v0) {
        true
    } else {
        let a3891 = 65u64;
        let a3892 = v0;
        if if nat_le(a3891, a3892) {
            let a3893 = v0;
            let a3894 = 70u64;
            nat_le(a3893, a3894)
        } else {
            false
        } {
            true
        } else {
            let a3895 = 97u64;
            let a3896 = v0;
            if nat_le(a3895, a3896) {
                let a3897 = v0;
                let a3898 = 102u64;
                nat_le(a3897, a3898)
            } else {
                false
            }
        }
    }
}

pub fn f570(v0: u64) -> bool {
    let a3899 = 48u64;
    let a3900 = v0;
    if nat_le(a3899, a3900) {
        let a3901 = v0;
        let a3902 = 57u64;
        nat_le(a3901, a3902)
    } else {
        false
    }
}

pub fn f571(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f163(v0.clone(), v1)?;
    let a3903 = v2;
    let a3904 = 69u64;
    if if nat_eq(a3903, a3904) {
        true
    } else {
        let a3905 = v2;
        let a3906 = 101u64;
        nat_eq(a3905, a3906)
    } {
        let v3: u64 = f163(v0.clone(), {
            let a3907 = v1;
            let a3908 = 1u64;
            nat_add(a3907, a3908)?
        })?;
        let v4: u64 = {
            let a3909 = v3;
            let a3910 = 43u64;
            if if nat_eq(a3909, a3910) {
                true
            } else {
                let a3911 = v3;
                let a3912 = 45u64;
                nat_eq(a3911, a3912)
            } {
                let a3913 = v1;
                let a3914 = 2u64;
                nat_add(a3913, a3914)?
            } else {
                let a3915 = v1;
                let a3916 = 1u64;
                nat_add(a3915, a3916)?
            }
        };
        let v5: u64 = f442(v0, 0u64, v4)?;
        let a3917 = v4;
        let a3918 = v5;
        if nat_lt(a3917, a3918) {
            Ok(v5)
        } else {
            Ok(v1)
        }
    } else {
        Ok(v1)
    }
}

pub fn f572(v0: Adt23, v1: Bytes) -> bool {
    let a3919 = f573(v0.clone());
    let a3920 = 3u64;
    if !nat_eq(a3919, a3920) {
        f32(f574(v0), v1)
    } else {
        false
    }
}

pub fn f573(v0: Adt23) -> u64 {
    let m3921 = v0;
    match m3921 {
        Adt23::C0(v1, _) => {
            v1
        }
    }
}

pub fn f574(v0: Adt23) -> Bytes {
    let m3922 = v0;
    match m3922 {
        Adt23::C0(_, v2) => {
            v2
        }
    }
}

pub fn f575(v0: Bytes) -> Bytes {
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

pub fn f576(v0: Bytes) -> bool {
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

pub fn f577(p3931: u64, p3932: List<Adt23>, p3933: List<Adt14>) -> R<Result<(List<Adt14>, List<Adt23>), Bytes>> {
    let m3930 = (p3931, (p3932, p3933));
    {
        let mut m3930 = m3930;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3930;
                let m3923 = v0;
                if m3923 == 0 {
                    Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt23>), Bytes>(f8(Str::lit("a dictionary has more members than the input has tokens")))))
                } else {
                    let v3 = m3923 - 1;
                    if f445(v1.clone(), Str::lit("}")) {
                        let m3924 = f452(f447(v1), Str::lit(";"));
                        match m3924 {
                            Ok(v5) => {
                                Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Ok::<(List<Adt14>, List<Adt23>), Bytes>((f675(v2, List::<Adt14>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt23>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m3925 = f329(v1)?;
                        match m3925 {
                            Ok(v7) => {
                                let v8: bool = f445(v7.clone(), Str::lit("required"));
                                let m3926 = f457(v3, if v8 {
                                    f447(v7)
                                } else {
                                    v7
                                }, 0u64)?;
                                match m3926 {
                                    Ok(v10) => {
                                        let m3927 = f451(f458(v10.clone()));
                                        match m3927 {
                                            Ok(v12) => {
                                                if if f445(f453(v12.clone()), Str::lit("=")) {
                                                    !v8
                                                } else {
                                                    false
                                                } {
                                                    let m3928 = f579(f447(f453(v12.clone())))?;
                                                    match m3928 {
                                                        Ok(v14) => {
                                                            Ok::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>((v3, (v14, List::cons(Adt14::C0(f455(v12), f586(f459(v10)), 0u64), v2))))
                                                        }
                                                        Err(v13) => {
                                                            Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt23>), Bytes>(v13)))
                                                        }
                                                    }
                                                } else {
                                                    let m3929 = f452(f453(v12.clone()), Str::lit(";"));
                                                    match m3929 {
                                                        Ok(v16) => {
                                                            Ok::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>((v3, (v16, List::cons(Adt14::C0(f455(v12), f586(f459(v10)), if v8 {
                                                                0u64
                                                            } else {
                                                                1u64
                                                            }), v2))))
                                                        }
                                                        Err(v15) => {
                                                            Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt23>), Bytes>(v15)))
                                                        }
                                                    }
                                                }
                                            }
                                            Err(v11) => {
                                                Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt23>), Bytes>(v11)))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt23>), Bytes>(v9)))
                                    }
                                }
                            }
                            Err(v6) => {
                                Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt23>), Bytes>(v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3930 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f578(p3942: u64, p3943: List<Adt23>, p3944: List<Adt14>) -> R<Result<(List<Adt14>, List<Adt23>), Bytes>> {
    let m3941 = (p3942, (p3943, p3944));
    {
        let mut m3941 = m3941;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3941;
                let m3934 = v0;
                if m3934 == 0 {
                    Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt23>), Bytes>(f8(Str::lit("an interface has more members than the input has tokens")))))
                } else {
                    let v3 = m3934 - 1;
                    if f445(v1.clone(), Str::lit("}")) {
                        let m3935 = f452(f447(v1), Str::lit(";"));
                        match m3935 {
                            Ok(v5) => {
                                Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Ok::<(List<Adt14>, List<Adt23>), Bytes>((f675(v2, List::<Adt14>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt23>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m3936 = f329(v1)?;
                        match m3936 {
                            Ok(v7) => {
                                let v8: List<Adt23> = if if f445(v7.clone(), Str::lit("stringifier")) {
                                    true
                                } else {
                                    f445(v7.clone(), Str::lit("inherit"))
                                } {
                                    f447(v7.clone())
                                } else {
                                    v7.clone()
                                };
                                let v9: List<Adt23> = if f445(v8.clone(), Str::lit("readonly")) {
                                    f447(v8)
                                } else {
                                    v8
                                };
                                if f445(v9.clone(), Str::lit("attribute")) {
                                    let m3937 = f457(v3, f447(v9), 0u64)?;
                                    match m3937 {
                                        Ok(v11) => {
                                            let m3938 = f451(f458(v11.clone()));
                                            match m3938 {
                                                Ok(v13) => {
                                                    let m3939 = f452(f453(v13.clone()), Str::lit(";"));
                                                    match m3939 {
                                                        Ok(v15) => {
                                                            Ok::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>((v3, (v15, List::cons(Adt14::C0(f455(v13), f586(f459(v11)), 0u64), v2))))
                                                        }
                                                        Err(v14) => {
                                                            Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt23>), Bytes>(v14)))
                                                        }
                                                    }
                                                }
                                                Err(v12) => {
                                                    Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt23>), Bytes>(v12)))
                                                }
                                            }
                                        }
                                        Err(v10) => {
                                            Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt23>), Bytes>(v10)))
                                        }
                                    }
                                } else {
                                    let m3940 = f579(v7)?;
                                    match m3940 {
                                        Ok(v17) => {
                                            Ok::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>((v3, (v17, v2)))
                                        }
                                        Err(v16) => {
                                            Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt23>), Bytes>(v16)))
                                        }
                                    }
                                }
                            }
                            Err(v6) => {
                                Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt23>), Bytes>(v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3941 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f579(v0: List<Adt23>) -> R<Result<List<Adt23>, Bytes>> {
    Ok(f446({
        let a3946 = {
            let a3945 = v0.clone();
            length_list(a3945)
        };
        let a3947 = 1u64;
        nat_add(a3946, a3947)?
    }, v0, List::<Bytes>::nil(), true))
}

pub fn f580(v0: List<Adt23>) -> Bytes {
    let m3948 = v0;
    match m3948.uncons() {
        None => {
            f8(Str::lit("the end of the input"))
        }
        Some((v1, _)) => {
            f7(List::cons(f8(Str::lit("'")), List::cons(f574(v1), List::cons(f8(Str::lit("'")), List::<Bytes>::nil()))))
        }
    }
}

pub fn f581(v0: Bytes) -> Bytes {
    if f53(v0.clone(), f8(Str::lit("_"))) {
        f46(v0.clone(), 1u64, {
            let a3950 = {
                let a3949 = v0;
                length_bytes(a3949)
            };
            let a3951 = 1u64;
            nat_sub(a3950, a3951)
        })
    } else {
        v0
    }
}

pub fn f582(v0: Adt15, v1: List<Adt23>) -> Result<Adt34, Bytes> {
    if f445(v1.clone(), Str::lit("?")) {
        Ok::<Adt34, Bytes>(Adt34::C0(f583(Str::lit("Option"), v0), f447(v1)))
    } else {
        Ok::<Adt34, Bytes>(Adt34::C0(v0, v1))
    }
}

pub fn f583(v0: Str, v1: Adt15) -> Adt15 {
    let v2: Adt15 = f586(v1);
    Adt15::C0(0u64, f7(List::cons(f8(Str::lit("(")), List::cons({
        let a3952 = v0;
        utf8_encode(a3952)
    }, List::cons(f8(Str::lit(" ")), List::cons(f347(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), f348(v2.clone()), f345(v2), Bytes::lit(&[]))
}

pub fn f584(v0: Bytes) -> Adt15 {
    Adt15::C0(0u64, Bytes::lit(&[]), List::<Bytes>::nil(), v0, Bytes::lit(&[]))
}

pub fn f585(v0: Bytes) -> Option<Adt15> {
    f676(v0, f677())
}

pub fn f586(v0: Adt15) -> Adt15 {
    let a3953 = f346(v0.clone());
    let a3954 = 0u64;
    if nat_eq(a3953, a3954) {
        v0
    } else {
        Adt15::C0(0u64, Bytes::lit(&[]), f348(v0.clone()), if f32(f345(v0.clone()), Bytes::lit(&[])) {
            f678()
        } else {
            f345(v0)
        }, Bytes::lit(&[]))
    }
}

pub fn f587(v0: bool) -> Adt15 {
    let v1: Bytes = if v0 {
        f148()
    } else {
        f146()
    };
    Adt15::C0(0u64, v1.clone(), List::cons(v1.clone(), List::<Bytes>::nil()), Bytes::lit(&[]), v1)
}

pub fn f588(v0: Adt15) -> Bytes {
    let m3955 = v0;
    match m3955 {
        Adt15::C0(_, _, _, _, v5) => {
            v5
        }
    }
}

pub fn f589(p3972: u64, p3973: u64, p3974: u64, p3975: Bytes) -> Bytes {
    let m3971 = (p3972, (p3973, (p3974, p3975)));
    {
        let mut m3971 = m3971;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m3971;
                let m3956 = v0;
                if m3956 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m3956 - 1;
                    if f113({
                        let a3957 = v1;
                        let a3958 = 0u64;
                        nat_eq(a3957, a3958)
                    }, {
                        let a3959 = v2;
                        let a3960 = 0u64;
                        nat_eq(a3959, a3960)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a3961 = v1;
                            let a3962 = 16u64;
                            let a3963 = 0u64;
                            nat_quot(a3961, a3962, a3963)
                        }, ({
                            let a3964 = v2;
                            let a3965 = 1u64;
                            nat_sub(a3964, a3965)
                        }, {
                            let a3969 = f679({
                                let a3966 = v1;
                                let a3967 = 16u64;
                                let a3968 = 0u64;
                                nat_rem(a3966, a3967, a3968)
                            });
                            let a3970 = v3;
                            append_bytes(a3969, a3970)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m3971 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f590(v0: Adt14, v1: List<Adt14>) -> R<List<Adt14>> {
    let m3976 = v1;
    match m3976.uncons() {
        None => {
            Ok(List::cons(v0, List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            if f680({
                let a3978 = {
                    let a3977 = f468(v0.clone());
                    length_bytes(a3977)
                };
                let a3979 = 1u64;
                nat_add(a3978, a3979)?
            }, f468(v0.clone()), f468(v2.clone()), 0u64)? {
                Ok(List::cons(v0, List::cons(v2, v3)))
            } else {
                Ok(List::cons(v2, f590(v0, v3)?))
            }
        }
    }
}

pub fn f591(v0: List<Adt13>, v1: Bytes) -> List<Adt14> {
    let m3980 = f229(v0.clone(), 1u64, v1.clone());
    match m3980 {
        None => {
            f465(v0, 6u64, v1)
        }
        Some(v2) => {
            f463(f464(v2), f465(v0, 6u64, v1))
        }
    }
}

pub fn f592(v0: Bytes) -> R<Option<Bytes>> {
    let m3982 = f681({
        let a3981 = v0.clone();
        length_bytes(a3981)
    }, v0.clone(), 0u64, Bytes::lit(&[]))?;
    match m3982 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a3984 = {
                let a3983 = v1.clone();
                length_bytes(a3983)
            };
            let a3985 = 0u64;
            if nat_eq(a3984, a3985) {
                let a3987 = {
                    let a3986 = v0;
                    length_bytes(a3986)
                };
                let a3988 = 0u64;
                if nat_eq(a3987, a3988) {
                    Ok(Some(f8(Str::lit("u"))))
                } else {
                    Ok(None::<Bytes>)
                }
            } else {
                Ok(Some(f594(v1)?))
            }
        }
    }
}

pub fn f593(v0: Bytes) -> R<Option<Bytes>> {
    if f682({
        let a3989 = v0.clone();
        length_bytes(a3989)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f683(v0)?;
        if f684(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m3992 = {
                let a3990 = v1.clone();
                let a3991 = 0u64;
                index_bytes(a3990, a3991)
            };
            match m3992 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f285(f62(v2)?) {
                        Ok(Some({
                            let a3993 = f8(Str::lit("n"));
                            let a3994 = v1;
                            append_bytes(a3993, a3994)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f685({
            let a3995 = v0.clone();
            length_bytes(a3995)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f594(v0: Bytes) -> R<Bytes> {
    if f122(v0.clone(), f686()) {
        f461(v0)
    } else {
        Ok(v0)
    }
}

pub fn f595() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{3}/\u{7f}\u{3}@\u{0}\u{3}\u{7f}}\u{4}\u{0}\u{0}C\u{7f}\u{7f}"))
}

pub fn f596(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a3996 = v1;
        let a3997 = 3u64;
        nat_mul(a3996, a3997)?
    };
    let a4008 = {
        let a4004 = {
            let a3998 = f111(v0.clone(), v2)?;
            let a3999 = 16384u64;
            nat_mul(a3998, a3999)?
        };
        let a4005 = {
            let a4002 = f111(v0.clone(), {
                let a4000 = v2;
                let a4001 = 1u64;
                nat_add(a4000, a4001)?
            })?;
            let a4003 = 128u64;
            nat_mul(a4002, a4003)?
        };
        nat_add(a4004, a4005)?
    };
    let a4009 = f111(v0, {
        let a4006 = v2;
        let a4007 = 2u64;
        nat_add(a4006, a4007)?
    })?;
    nat_add(a4008, a4009)
}

pub fn f597(v0: u64) -> R<bool> {
    f351(v0, f687())
}

pub fn f598(v0: u64) -> R<bool> {
    f351(v0, f688())
}

pub fn f599(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f689(v1, v0, v2)
}

pub fn f600(v0: Bytes, v1: u64) -> Bytes {
    f46(v0, v1, 1u64)
}

pub fn f601() -> Bytes {
    f8(Str::lit("\u{0}\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\u{9}\u{a}\u{b}\u{c}\u{d}\u{e}\u{f}\u{10}\u{11}\u{12}\u{13}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{1b}\u{1c}\u{1d}\u{1e}\u{1f} !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~\u{7f}"))
}

pub fn f602() -> Bytes {
    f8(Str::lit("\u{80}\u{c0}\u{100}\u{140}\u{180}\u{1c0}\u{200}\u{240}\u{280}\u{2c0}\u{300}\u{340}\u{380}\u{3c0}\u{400}\u{440}\u{480}\u{4c0}\u{500}\u{540}\u{580}\u{5c0}\u{600}\u{640}\u{680}\u{6c0}\u{700}\u{740}\u{780}\u{7c0}"))
}

pub fn f603(v0: u64) -> R<Bytes> {
    Ok(f600(f690(), {
        let a4015 = {
            let a4013 = {
                let a4010 = v0;
                let a4011 = 64u64;
                let a4012 = 0u64;
                nat_rem(a4010, a4011, a4012)
            };
            let a4014 = 2u64;
            nat_mul(a4013, a4014)?
        };
        let a4016 = 1u64;
        nat_add(a4015, a4016)?
    }))
}

pub fn f604() -> Bytes {
    f8(Str::lit("\u{800}\u{1000}\u{2000}\u{3000}\u{4000}\u{5000}\u{6000}\u{7000}\u{8000}\u{9000}\u{a000}\u{b000}\u{c000}\u{d000}\u{e000}\u{f000}"))
}

pub fn f605(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<Option<u64>> {
    if f691(f163(v0, v3)?, v1, v2)? {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a4017 = v3;
            let a4018 = 1u64;
            nat_add(a4017, a4018)?
        }))
    }
}

pub fn f606(v0: Bytes) -> Result<Bytes, Bytes> {
    Err::<Bytes, Bytes>(v0)
}

pub fn f607() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f608(v0: Bytes) -> R<Option<Bytes>> {
    let m4019 = f234(v0.clone(), f8(Str::lit(" ")), 0u64)?;
    match m4019 {
        None => {
            let m4020 = f234(v0.clone(), f8(Str::lit(":")), 0u64)?;
            match m4020 {
                None => {
                    let v3: Bytes = f7(List::cons(f8(Str::lit(" ")), List::cons(v0, List::cons(f8(Str::lit(":")), List::<Bytes>::nil()))));
                    let m4021 = f234(f692(), v3.clone(), 0u64)?;
                    match m4021 {
                        None => {
                            Ok(None::<Bytes>)
                        }
                        Some(v4) => {
                            let v5: u64 = {
                                let a4023 = v4;
                                let a4024 = {
                                    let a4022 = v3;
                                    length_bytes(a4022)
                                };
                                nat_add(a4023, a4024)?
                            };
                            let m4025 = f234(f692(), f8(Str::lit(" ")), v5)?;
                            match m4025 {
                                None => {
                                    Ok(None::<Bytes>)
                                }
                                Some(v6) => {
                                    Ok(Some(f46(f692(), v5, {
                                        let a4026 = v6;
                                        let a4027 = v5;
                                        nat_sub(a4026, a4027)
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

pub fn f609() -> List<Bytes> {
    List::cons(f8(Str::lit("complexType")), List::cons(f8(Str::lit("simpleType")), List::cons(f8(Str::lit("element")), List::cons(f8(Str::lit("attribute")), List::cons(f8(Str::lit("attributeGroup")), List::cons(f8(Str::lit("group")), List::<Bytes>::nil()))))))
}

pub fn f610(v0: u64, v1: Adt25, v2: Adt26) -> R<Result<List<Adt28>, Bytes>> {
    let v3: Bytes = f125(match v2.clone() {
        Adt26::C0(_, _, h4028, _) => {
            h4028
        }
    })?;
    let v4: (Bytes, Bytes) = (match v2.clone() {
        Adt26::C0(_, h4029, _, _) => {
            h4029
        }
    }, match v2.clone() {
        Adt26::C0(_, _, h4030, _) => {
            h4030
        }
    });
    let v5: bool = f693(match v2.clone() {
        Adt26::C0(_, _, _, h4031) => {
            h4031
        }
    }, Str::lit("abstract"), Str::lit("true"));
    let m4033 = f694(match v1.clone() {
        Adt25::C0(h4032, _) => {
            h4032
        }
    }, v4, f379(v5))?;
    match m4033 {
        Ok(v7) => {
            if v5 {
                let m4034 = v7.clone();
                match m4034.uncons() {
                    None => {
                        Ok(Err::<List<Adt28>, Bytes>(f616(f8(Str::lit("an abstract complex type with no type derived from it in this schema set admits no instance (XSD 1.1 Part 1, sections 3.4.1 and 3.3.4.2)")), v3)))
                    }
                    Some((_, _)) => {
                        let m4035 = f695(v3, v7.clone(), f696(v7.clone())?, f696(v7)?)?;
                        match m4035 {
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
                let m4036 = v7.clone();
                match m4036.uncons() {
                    None => {
                        f697(v0, v1, v3, match v2.clone() {
                            Adt26::C0(_, h4037, _, _) => {
                                h4037
                            }
                        }, match v2 {
                            Adt26::C0(_, _, _, h4038) => {
                                h4038
                            }
                        })
                    }
                    Some((_, _)) => {
                        let v14: Bytes = f698(v3.clone());
                        let m4041 = f697(v0, v1, v14.clone(), match v2.clone() {
                            Adt26::C0(_, h4039, _, _) => {
                                h4039
                            }
                        }, match v2.clone() {
                            Adt26::C0(_, _, _, h4040) => {
                                h4040
                            }
                        })?;
                        match m4041 {
                            Ok(v16) => {
                                let m4043 = f695(v3, List::cons(match v2 {
                                    Adt26::C0(_, _, h4042, _) => {
                                        h4042
                                    }
                                }, v7.clone()), List::cons(v14.clone(), f696(v7.clone())?), List::cons(v14, f696(v7)?))?;
                                match m4043 {
                                    Ok(v18) => {
                                        Ok(Ok::<List<Adt28>, Bytes>(f489(v16, List::cons(v18, List::<Adt28>::nil()))))
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

pub fn f611(v0: Bytes, v1: Adt27, v2: Bytes) -> R<Result<Adt28, Bytes>> {
    let m4046 = f252(v0.clone(), match v1.clone() {
        Adt27::C0(_, _, _, h4044, _) => {
            h4044
        }
    }, match v1 {
        Adt27::C0(_, _, _, _, h4045) => {
            h4045
        }
    }, v2)?;
    match m4046 {
        Ok(v4) => {
            Ok(Ok::<Adt28, Bytes>(Adt28::C0(f125(v0)?, match v4.clone() {
                Adt24::C0(_, h4047) => {
                    h4047
                }
            }, match v4 {
                Adt24::C0(h4048, _) => {
                    h4048
                }
            }, List::<Bytes>::nil())))
        }
        Err(v3) => {
            Ok(Err::<Adt28, Bytes>(v3))
        }
    }
}

pub fn f612(v0: u64, v1: Adt25, v2: Bytes, v3: Bytes, v4: Adt27) -> R<Result<Adt35, Bytes>> {
    let m4049 = v0;
    if m4049 == 0 {
        Ok(Err::<Adt35, Bytes>(f616(f8(Str::lit("the element declarations nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m4049 - 1;
        let m4050 = f381(v4.clone(), Str::lit("type"));
        match m4050 {
            None => {
                let m4054 = f614(v4.clone())?;
                match m4054 {
                    Ok(v10) => {
                        let m4055 = f615(v10, List::cons(f8(Str::lit("complexType")), List::cons(f8(Str::lit("simpleType")), List::<Bytes>::nil())));
                        match m4055 {
                            None => {
                                Ok(Ok::<Adt35, Bytes>(Adt35::C0(Adt24::C0(f700(v4, f492()), List::cons(f492(), List::<Bytes>::nil())), List::<Adt28>::nil())))
                            }
                            Some(v11) => {
                                if f380(v11.clone(), Str::lit("complexType")) {
                                    let m4056 = f697(v5, v1, v2.clone(), v3, v11)?;
                                    match m4056 {
                                        Ok(v13) => {
                                            Ok(Ok::<Adt35, Bytes>(Adt35::C0(Adt24::C0(f700(v4, v2.clone()), List::cons(v2, List::<Bytes>::nil())), v13)))
                                        }
                                        Err(v12) => {
                                            Ok(Err::<Adt35, Bytes>(v12))
                                        }
                                    }
                                } else {
                                    let m4057 = f611(v2.clone(), v11, v3)?;
                                    match m4057 {
                                        Ok(v15) => {
                                            Ok(Ok::<Adt35, Bytes>(Adt35::C0(Adt24::C0(f700(v4, v2.clone()), List::cons(v2, List::<Bytes>::nil())), List::cons(v15, List::<Adt28>::nil()))))
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
                let m4051 = f699(v1, v4.clone(), v6)?;
                match m4051 {
                    Ok(v8) => {
                        Ok(Ok::<Adt35, Bytes>(Adt35::C0(Adt24::C0(f700(v4, match v8.clone() {
                            Adt24::C0(h4052, _) => {
                                h4052
                            }
                        }), match v8 {
                            Adt24::C0(_, h4053) => {
                                h4053
                            }
                        }), List::<Adt28>::nil())))
                    }
                    Err(v7) => {
                        Ok(Err::<Adt35, Bytes>(f616(v7, v2)))
                    }
                }
            }
        }
    }
}

pub fn f613(v0: Adt25, v1: Bytes, v2: Bytes, v3: Adt27) -> R<Result<Adt35, Bytes>> {
    let m4058 = f381(v3.clone(), Str::lit("type"));
    match m4058 {
        None => {
            let m4060 = f614(v3)?;
            match m4060 {
                Ok(v8) => {
                    let m4061 = f615(v8, List::cons(f8(Str::lit("simpleType")), List::<Bytes>::nil()));
                    match m4061 {
                        None => {
                            Ok(Err::<Adt35, Bytes>(f616(f8(Str::lit("an attribute declaration with no type has the type xs:anySimpleType, which is not translated, because it is the union of every simple value space (XSD 1.1 Part 1, section 3.2.2.1; Part 2, section 3.2.1)")), v2)))
                        }
                        Some(v9) => {
                            let m4062 = f611(v2.clone(), v9, v1)?;
                            match m4062 {
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
            let m4059 = f699(v0, v3, v4)?;
            match m4059 {
                Ok(v6) => {
                    Ok(Ok::<Adt35, Bytes>(Adt35::C0(v6, List::<Adt28>::nil())))
                }
                Err(v5) => {
                    Ok(Err::<Adt35, Bytes>(f616(v5, v2)))
                }
            }
        }
    }
}

pub fn f614(v0: Adt27) -> R<Result<List<Adt27>, Bytes>> {
    f264(match v0.clone() {
        Adt27::C0(_, _, _, h4063, _) => {
            h4063
        }
    }, match v0 {
        Adt27::C0(_, _, _, _, h4064) => {
            h4064
        }
    })
}

pub fn f615(p4068: List<Adt27>, v1: List<Bytes>) -> Option<Adt27> {
    let m4067 = p4068;
    {
        let mut m4067 = m4067;
        loop {
            let step = {
                let v0 = m4067;
                let m4065 = v0;
                match m4065.uncons() {
                    None => {
                        Err::<List<Adt27>, Option<Adt27>>(None::<Adt27>)
                    }
                    Some((v2, v3)) => {
                        if f122(match v2.clone() {
                            Adt27::C0(_, h4066, _, _, _) => {
                                h4066
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
                Ok(next) => { m4067 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f616(v0: Bytes, v1: Bytes) -> Bytes {
    f260(v0, f8(Str::lit(" at ")), v1)
}

pub fn f617(v0: u64, v1: Adt25, v2: Bytes, v3: Bytes, v4: Adt27) -> R<Result<List<Adt28>, Bytes>> {
    let m4069 = v0;
    if m4069 == 0 {
        Ok(Err::<List<Adt28>, Bytes>(f616(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m4069 - 1;
        let m4070 = f614(v4.clone())?;
        match m4070 {
            Ok(v7) => {
                if f380(v4, Str::lit("choice")) {
                    let m4071 = f701(v5, v1, v2.clone(), v3, false, v7, f619())?;
                    match m4071 {
                        Ok(v9) => {
                            let m4075 = f695(v2, f702(match v9.clone() {
                                Adt36::C0(h4072, _, _, _, _) => {
                                    h4072
                                }
                            }), f703(match v9.clone() {
                                Adt36::C0(h4073, _, _, _, _) => {
                                    h4073
                                }
                            }), f704(match v9.clone() {
                                Adt36::C0(h4074, _, _, _, _) => {
                                    h4074
                                }
                            }))?;
                            match m4075 {
                                Ok(v11) => {
                                    Ok(Ok::<List<Adt28>, Bytes>(f489(match v9 {
                                        Adt36::C0(_, h4076, _, _, _) => {
                                            h4076
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
                    let m4077 = f701(v5, v1, v2.clone(), v3, true, v7, f619())?;
                    match m4077 {
                        Ok(v13) => {
                            let m4079 = f705(v2, match v13.clone() {
                                Adt36::C0(h4078, _, _, _, _) => {
                                    h4078
                                }
                            })?;
                            match m4079 {
                                Ok(v15) => {
                                    Ok(Ok::<List<Adt28>, Bytes>(f489(match v13 {
                                        Adt36::C0(_, h4080, _, _, _) => {
                                            h4080
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

pub fn f618(v0: u64, v1: Adt25, v2: Bytes, v3: Bytes, v4: List<Adt27>, v5: Adt36) -> R<Result<Adt36, Bytes>> {
    let m4081 = v0;
    if m4081 == 0 {
        Ok(Err::<Adt36, Bytes>(f616(f8(Str::lit("the attribute group references nest deeper than the schema set has components, so they refer to each other in a cycle (XSD 1.1 Part 1, section 3.6.3)")), v2)))
    } else {
        let v6 = m4081 - 1;
        let m4082 = v4;
        match m4082.uncons() {
            None => {
                Ok(Ok::<Adt36, Bytes>(v5))
            }
            Some((v7, v8)) => {
                if f380(v7.clone(), Str::lit("attribute")) {
                    if f693(v7.clone(), Str::lit("use"), Str::lit("prohibited")) {
                        f618(v6, v1, v2, v3, v8, v5)
                    } else {
                        let v9: bool = f693(v7.clone(), Str::lit("use"), Str::lit("required"));
                        let m4083 = f381(v7.clone(), Str::lit("ref"));
                        match m4083 {
                            None => {
                                let m4093 = f381(v7.clone(), Str::lit("name"));
                                match m4093 {
                                    None => {
                                        Ok(Err::<Adt36, Bytes>(f616(f8(Str::lit("a local attribute declaration must have a name or a ref (XSD 1.1 Part 1, section 3.2.3)")), v2)))
                                    }
                                    Some(v16) => {
                                        let m4096 = f613(v1.clone(), v3.clone(), {
                                            let a4094 = v2.clone();
                                            let a4095 = f125(v16.clone())?;
                                            append_bytes(a4094, a4095)
                                        }, v7)?;
                                        match m4096 {
                                            Ok(v18) => {
                                                f618(v6, v1, v2, v3, v8, f707(v5, v16, v18, v9))
                                            }
                                            Err(v17) => {
                                                Ok(Err::<Adt36, Bytes>(v17))
                                            }
                                        }
                                    }
                                }
                            }
                            Some(v10) => {
                                let m4085 = f167(match v7 {
                                    Adt27::C0(_, _, _, _, h4084) => {
                                        h4084
                                    }
                                }, v10.clone())?;
                                match m4085 {
                                    Ok(v12) => {
                                        if f32({
                                            let (h4086, _) = v12.clone();
                                            h4086
                                        }, f245()) {
                                            Ok(Err::<Adt36, Bytes>(f616(f260(f8(Str::lit("the attribute ")), v10, f8(Str::lit(" is declared by the schema document for the XML namespace, which this schema set does not import (XSD 1.1 Part 1, sections 3.2.2.3 and 4.2.6)"))), v2)))
                                        } else {
                                            let m4087 = f706(v1.clone(), Str::lit("attribute"), v12.clone());
                                            match m4087 {
                                                None => {
                                                    Ok(Err::<Adt36, Bytes>(f616(f260(f8(Str::lit("the attribute ")), v10, f8(Str::lit(" is not declared by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                                }
                                                Some(v13) => {
                                                    let m4091 = f613(v1.clone(), match v13.clone() {
                                                        Adt26::C0(_, h4088, _, _) => {
                                                            h4088
                                                        }
                                                    }, f125({
                                                        let (_, h4089) = v12.clone();
                                                        h4089
                                                    })?, match v13 {
                                                        Adt26::C0(_, _, _, h4090) => {
                                                            h4090
                                                        }
                                                    })?;
                                                    match m4091 {
                                                        Ok(v15) => {
                                                            f618(v6, v1, v2, v3, v8, f707(v5, {
                                                                let (_, h4092) = v12;
                                                                h4092
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
                                        Ok(Err::<Adt36, Bytes>(f616(v11, v2)))
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if f380(v7.clone(), Str::lit("attributeGroup")) {
                        let m4097 = f381(v7.clone(), Str::lit("ref"));
                        match m4097 {
                            None => {
                                Ok(Err::<Adt36, Bytes>(f616(f8(Str::lit("an attribute group reference must have a ref (XSD 1.1 Part 1, section 3.6.2)")), v2)))
                            }
                            Some(v19) => {
                                let m4099 = f167(match v7 {
                                    Adt27::C0(_, _, _, _, h4098) => {
                                        h4098
                                    }
                                }, v19.clone())?;
                                match m4099 {
                                    Ok(v21) => {
                                        let m4100 = f706(v1.clone(), Str::lit("attributeGroup"), v21.clone());
                                        match m4100 {
                                            None => {
                                                Ok(Err::<Adt36, Bytes>(f616(f260(f8(Str::lit("the attribute group ")), v19, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                            }
                                            Some(v22) => {
                                                let m4102 = f614(match v22.clone() {
                                                    Adt26::C0(_, _, _, h4101) => {
                                                        h4101
                                                    }
                                                })?;
                                                match m4102 {
                                                    Ok(v24) => {
                                                        let m4105 = f618(v6, v1.clone(), f125({
                                                            let (_, h4103) = v21;
                                                            h4103
                                                        })?, match v22 {
                                                            Adt26::C0(_, h4104, _, _) => {
                                                                h4104
                                                            }
                                                        }, v24, v5)?;
                                                        match m4105 {
                                                            Ok(v26) => {
                                                                f618(v6, v1, v2, v3, v8, v26)
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
                                        Ok(Err::<Adt36, Bytes>(f616(v20, v2)))
                                    }
                                }
                            }
                        }
                    } else {
                        if f380(v7, Str::lit("anyAttribute")) {
                            f618(v6, v1, v2, v3, v8, f708(v5, Adt37::C0(f8(Str::lit("anyAttribute")), f709(), List::<Bytes>::nil())))
                        } else {
                            f618(v6, v1, v2, v3, v8, v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f619() -> Adt36 {
    Adt36::C0(List::<Adt37>::nil(), List::<Adt28>::nil(), 0u64, 0u64, 0u64)
}

pub fn f620(v0: List<Adt28>, v1: List<Bytes>, v2: Bytes) -> Result<List<Bytes>, Bytes> {
    let m4106 = v1;
    match m4106.uncons() {
        None => {
            Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let m4107 = f491(v0.clone(), v3.clone());
            match m4107 {
                None => {
                    Err::<List<Bytes>, Bytes>(f621(v3))
                }
                Some(v5) => {
                    let m4109 = match v5.clone() {
                        Adt28::C0(_, _, _, h4108) => {
                            h4108
                        }
                    };
                    match m4109.uncons() {
                        None => {
                            Err::<List<Bytes>, Bytes>(f621(v3))
                        }
                        Some((_, _)) => {
                            let m4110 = f620(v0, v4, v2.clone());
                            match m4110 {
                                Ok(v9) => {
                                    Ok::<List<Bytes>, Bytes>(List::cons(f710(v3, match v5 {
                                        Adt28::C0(_, _, _, h4111) => {
                                            h4111
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

pub fn f621(v0: Bytes) -> Bytes {
    f616(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)")), v0)
}

pub fn f622(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m4112 = v0;
    match m4112.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f122(v2.clone(), v1.clone()) {
                f622(v3, v1)
            } else {
                List::cons(v2, f622(v3, v1))
            }
        }
    }
}

pub fn f623(p4117: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m4116 = p4117;
    {
        let mut m4116 = m4116;
        loop {
            let step = {
                let v0 = m4116;
                let m4113 = v0;
                match m4113.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h4114, _) = v2;
                            h4114
                        }, v1.clone()) {
                            let (_, h4115) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h4115)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4116 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f624(v0: Fn14, p4122: u64, p4123: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m4121 = (p4122, p4123);
    {
        let mut m4121 = m4121;
        loop {
            let step = {
                let (v1, v2) = m4121;
                let m4118 = v1;
                if m4118 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4118 - 1;
                    let m4120 = {
                        let c4119: Fn14 = v0.clone();
                        c4119.apply(v2.clone())?
                    };
                    match m4120 {
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
                Ok(next) => { m4121 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f625(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f711(v0, v1)
}

pub fn f626(v0: u8) -> R<bool> {
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
    let a4170 = 48u64;
    let a4171 = v2;
    if nat_le(a4170, a4171) {
        let a4172 = v2;
        let a4173 = 57u64;
        Ok(nat_le(a4172, a4173))
    } else {
        Ok(false)
    }
}

pub fn f627(v0: u64) -> u32 {
    let a4174 = v0;
    let a4175 = 32u64;
    if nat_lt(a4174, a4175) {
        let a4176 = v0;
        let a4177 = 16u64;
        if nat_lt(a4176, a4177) {
            let a4178 = v0;
            let a4179 = 8u64;
            if nat_lt(a4178, a4179) {
                let a4180 = v0;
                let a4181 = 4u64;
                if nat_lt(a4180, a4181) {
                    let a4182 = v0;
                    let a4183 = 2u64;
                    if nat_lt(a4182, a4183) {
                        let a4184 = v0;
                        let a4185 = 1u64;
                        if nat_lt(a4184, a4185) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a4186 = v0;
                        let a4187 = 3u64;
                        if nat_lt(a4186, a4187) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a4188 = v0;
                    let a4189 = 6u64;
                    if nat_lt(a4188, a4189) {
                        let a4190 = v0;
                        let a4191 = 5u64;
                        if nat_lt(a4190, a4191) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a4192 = v0;
                        let a4193 = 7u64;
                        if nat_lt(a4192, a4193) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a4194 = v0;
                let a4195 = 12u64;
                if nat_lt(a4194, a4195) {
                    let a4196 = v0;
                    let a4197 = 10u64;
                    if nat_lt(a4196, a4197) {
                        let a4198 = v0;
                        let a4199 = 9u64;
                        if nat_lt(a4198, a4199) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a4200 = v0;
                        let a4201 = 11u64;
                        if nat_lt(a4200, a4201) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a4202 = v0;
                    let a4203 = 14u64;
                    if nat_lt(a4202, a4203) {
                        let a4204 = v0;
                        let a4205 = 13u64;
                        if nat_lt(a4204, a4205) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a4206 = v0;
                        let a4207 = 15u64;
                        if nat_lt(a4206, a4207) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a4208 = v0;
            let a4209 = 24u64;
            if nat_lt(a4208, a4209) {
                let a4210 = v0;
                let a4211 = 20u64;
                if nat_lt(a4210, a4211) {
                    let a4212 = v0;
                    let a4213 = 18u64;
                    if nat_lt(a4212, a4213) {
                        let a4214 = v0;
                        let a4215 = 17u64;
                        if nat_lt(a4214, a4215) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a4216 = v0;
                        let a4217 = 19u64;
                        if nat_lt(a4216, a4217) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a4218 = v0;
                    let a4219 = 22u64;
                    if nat_lt(a4218, a4219) {
                        let a4220 = v0;
                        let a4221 = 21u64;
                        if nat_lt(a4220, a4221) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a4222 = v0;
                        let a4223 = 23u64;
                        if nat_lt(a4222, a4223) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a4224 = v0;
                let a4225 = 28u64;
                if nat_lt(a4224, a4225) {
                    let a4226 = v0;
                    let a4227 = 26u64;
                    if nat_lt(a4226, a4227) {
                        let a4228 = v0;
                        let a4229 = 25u64;
                        if nat_lt(a4228, a4229) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a4230 = v0;
                        let a4231 = 27u64;
                        if nat_lt(a4230, a4231) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a4232 = v0;
                    let a4233 = 30u64;
                    if nat_lt(a4232, a4233) {
                        let a4234 = v0;
                        let a4235 = 29u64;
                        if nat_lt(a4234, a4235) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a4236 = v0;
                        let a4237 = 31u64;
                        if nat_lt(a4236, a4237) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a4238 = v0;
        let a4239 = 48u64;
        if nat_lt(a4238, a4239) {
            let a4240 = v0;
            let a4241 = 40u64;
            if nat_lt(a4240, a4241) {
                let a4242 = v0;
                let a4243 = 36u64;
                if nat_lt(a4242, a4243) {
                    let a4244 = v0;
                    let a4245 = 34u64;
                    if nat_lt(a4244, a4245) {
                        let a4246 = v0;
                        let a4247 = 33u64;
                        if nat_lt(a4246, a4247) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a4248 = v0;
                        let a4249 = 35u64;
                        if nat_lt(a4248, a4249) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a4250 = v0;
                    let a4251 = 38u64;
                    if nat_lt(a4250, a4251) {
                        let a4252 = v0;
                        let a4253 = 37u64;
                        if nat_lt(a4252, a4253) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a4254 = v0;
                        let a4255 = 39u64;
                        if nat_lt(a4254, a4255) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a4256 = v0;
                let a4257 = 44u64;
                if nat_lt(a4256, a4257) {
                    let a4258 = v0;
                    let a4259 = 42u64;
                    if nat_lt(a4258, a4259) {
                        let a4260 = v0;
                        let a4261 = 41u64;
                        if nat_lt(a4260, a4261) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a4262 = v0;
                        let a4263 = 43u64;
                        if nat_lt(a4262, a4263) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a4264 = v0;
                    let a4265 = 46u64;
                    if nat_lt(a4264, a4265) {
                        let a4266 = v0;
                        let a4267 = 45u64;
                        if nat_lt(a4266, a4267) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a4268 = v0;
                        let a4269 = 47u64;
                        if nat_lt(a4268, a4269) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a4270 = v0;
            let a4271 = 56u64;
            if nat_lt(a4270, a4271) {
                let a4272 = v0;
                let a4273 = 52u64;
                if nat_lt(a4272, a4273) {
                    let a4274 = v0;
                    let a4275 = 50u64;
                    if nat_lt(a4274, a4275) {
                        let a4276 = v0;
                        let a4277 = 49u64;
                        if nat_lt(a4276, a4277) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a4278 = v0;
                        let a4279 = 51u64;
                        if nat_lt(a4278, a4279) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a4280 = v0;
                    let a4281 = 54u64;
                    if nat_lt(a4280, a4281) {
                        let a4282 = v0;
                        let a4283 = 53u64;
                        if nat_lt(a4282, a4283) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a4284 = v0;
                        let a4285 = 55u64;
                        if nat_lt(a4284, a4285) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a4286 = v0;
                let a4287 = 60u64;
                if nat_lt(a4286, a4287) {
                    let a4288 = v0;
                    let a4289 = 58u64;
                    if nat_lt(a4288, a4289) {
                        let a4290 = v0;
                        let a4291 = 57u64;
                        if nat_lt(a4290, a4291) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a4292 = v0;
                        let a4293 = 59u64;
                        if nat_lt(a4292, a4293) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a4294 = v0;
                    let a4295 = 62u64;
                    if nat_lt(a4294, a4295) {
                        let a4296 = v0;
                        let a4297 = 61u64;
                        if nat_lt(a4296, a4297) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a4298 = v0;
                        let a4299 = 63u64;
                        if nat_lt(a4298, a4299) {
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

pub fn f628(v0: u32) -> u32 {
    let a4302 = {
        let a4300 = f712(v0, 6u32, 26u32);
        let a4301 = f712(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a4300, a4301)
    };
    let a4303 = f712(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a4302, a4303)
}

pub fn f629(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4309 = {
        let a4304 = v0;
        let a4305 = v1;
        fixed_u32::bit_and(a4304, a4305)
    };
    let a4310 = {
        let a4307 = {
            let a4306 = v0;
            fixed_u32::bit_not(a4306)
        };
        let a4308 = v2;
        fixed_u32::bit_and(a4307, a4308)
    };
    fixed_u32::bit_xor(a4309, a4310)
}

pub fn f630(v0: u32) -> u32 {
    let a4313 = {
        let a4311 = f712(v0, 2u32, 30u32);
        let a4312 = f712(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a4311, a4312)
    };
    let a4314 = f712(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a4313, a4314)
}

pub fn f631(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4323 = {
        let a4319 = {
            let a4315 = v0;
            let a4316 = v1;
            fixed_u32::bit_and(a4315, a4316)
        };
        let a4320 = {
            let a4317 = v0;
            let a4318 = v2;
            fixed_u32::bit_and(a4317, a4318)
        };
        fixed_u32::bit_xor(a4319, a4320)
    };
    let a4324 = {
        let a4321 = v1;
        let a4322 = v2;
        fixed_u32::bit_and(a4321, a4322)
    };
    fixed_u32::bit_xor(a4323, a4324)
}

pub fn f632(v0: Adt29) -> Adt29 {
    Adt29::C0(match v0.clone() {
        Adt29::C0(_, h4325, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4325
        }
    }, match v0.clone() {
        Adt29::C0(_, _, h4326, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4326
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, h4327, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4327
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, h4328, _, _, _, _, _, _, _, _, _, _, _) => {
            h4328
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, h4329, _, _, _, _, _, _, _, _, _, _) => {
            h4329
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, h4330, _, _, _, _, _, _, _, _, _) => {
            h4330
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, h4331, _, _, _, _, _, _, _, _) => {
            h4331
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, h4332, _, _, _, _, _, _, _) => {
            h4332
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, _, h4333, _, _, _, _, _, _) => {
            h4333
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, _, _, h4334, _, _, _, _, _) => {
            h4334
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, _, _, _, h4335, _, _, _, _) => {
            h4335
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, _, _, _, _, h4336, _, _, _) => {
            h4336
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h4337, _, _) => {
            h4337
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4338, _) => {
            h4338
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h4339) => {
            h4339
        }
    }, f395(f713(match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4340, _) => {
            h4340
        }
    }), f395(match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, _, h4341, _, _, _, _, _, _) => {
            h4341
        }
    }, f395(f714(match v0.clone() {
        Adt29::C0(_, h4342, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4342
        }
    }), match v0 {
        Adt29::C0(h4343, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4343
        }
    }))))
}

pub fn f633(v0: Fn15, p4347: u64, p4348: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m4346 = (p4347, p4348);
    {
        let mut m4346 = m4346;
        loop {
            let step = {
                let (v1, v2) = m4346;
                let m4344 = v1;
                if m4344 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m4344 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c4345: Fn15 = v0.clone();
                        c4345.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m4346 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f634(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h4349, _) = v0;
        h4349
    };
    let v2: u32 = {
        let (h4350, _) = {
            let (_, h4351) = v0;
            h4351
        };
        h4350
    };
    let v3: u32 = {
        let (_, h4352) = {
            let (_, h4353) = v0;
            h4353
        };
        h4352
    };
    ({
        let a4354 = v1;
        let a4355 = 2u64;
        let a4356 = 0u64;
        nat_quot(a4354, a4355, a4356)
    }, (f635(v2, 1u32), {
        let a4360 = {
            let a4357 = v1;
            let a4358 = 2u64;
            let a4359 = 0u64;
            nat_rem(a4357, a4358, a4359)
        };
        let a4361 = 1u64;
        if nat_eq(a4360, a4361) {
            let a4362 = v3;
            let a4363 = v2;
            fixed_u32::bit_or(a4362, a4363)
        } else {
            v3
        }
    }))
}

pub fn f635(v0: u32, v1: u32) -> u32 {
    let m4366 = {
        let a4364 = v0;
        let a4365 = v1;
        fixed_u32::shift_left(a4364, a4365)
    };
    match m4366 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f636(p4369: List<u64>, p4370: List<u64>) -> List<u64> {
    let m4368 = (p4369, p4370);
    {
        let mut m4368 = m4368;
        loop {
            let step = {
                let (v0, v1) = m4368;
                let m4367 = v0;
                match m4367.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m4368 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f637(v0: List<u64>) -> R<Bytes> {
    let m4371 = v0;
    match m4371.uncons() {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some((v1, v2)) => {
            let a4372 = f715(v1)?;
            let a4373 = f637(v2)?;
            Ok(append_bytes(a4372, a4373))
        }
    }
}

pub fn f638(v0: List<u64>, v1: u64, v2: u64) -> R<List<u64>> {
    let m4374 = v0;
    match m4374.uncons() {
        None => {
            let a4375 = v2;
            let a4376 = 0u64;
            if nat_eq(a4375, a4376) {
                Ok(List::<u64>::nil())
            } else {
                Ok(List::cons(v2, List::<u64>::nil()))
            }
        }
        Some((v3, v4)) => {
            let v5: u64 = {
                let a4379 = {
                    let a4377 = v3;
                    let a4378 = v1;
                    nat_mul(a4377, a4378)?
                };
                let a4380 = v2;
                nat_add(a4379, a4380)?
            };
            Ok(List::cons({
                let a4381 = v5;
                let a4382 = f639();
                let a4383 = 0u64;
                nat_rem(a4381, a4382, a4383)
            }, f638(v4, v1, {
                let a4384 = v5;
                let a4385 = f639();
                let a4386 = 0u64;
                nat_quot(a4384, a4385, a4386)
            })?))
        }
    }
}

pub fn f639() -> u64 {
    1000000000u64
}

pub fn f640(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4431 = {
            let a4407 = {
                let a4395 = {
                    let a4389 = {
                        let a4387 = v1;
                        let a4388 = 128u8;
                        fixed_u8::bit_and(a4387, a4388)
                    };
                    let a4390 = 128u8;
                    if equal(a4389, a4390) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4396 = {
                    let a4393 = {
                        let a4391 = v1;
                        let a4392 = 64u8;
                        fixed_u8::bit_and(a4391, a4392)
                    };
                    let a4394 = 64u8;
                    if equal(a4393, a4394) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4395, a4396)?
            };
            let a4408 = {
                let a4405 = {
                    let a4399 = {
                        let a4397 = v1;
                        let a4398 = 32u8;
                        fixed_u8::bit_and(a4397, a4398)
                    };
                    let a4400 = 32u8;
                    if equal(a4399, a4400) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4406 = {
                    let a4403 = {
                        let a4401 = v1;
                        let a4402 = 16u8;
                        fixed_u8::bit_and(a4401, a4402)
                    };
                    let a4404 = 16u8;
                    if equal(a4403, a4404) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4405, a4406)?
            };
            nat_add(a4407, a4408)?
        };
        let a4432 = {
            let a4429 = {
                let a4417 = {
                    let a4411 = {
                        let a4409 = v1;
                        let a4410 = 8u8;
                        fixed_u8::bit_and(a4409, a4410)
                    };
                    let a4412 = 8u8;
                    if equal(a4411, a4412) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4418 = {
                    let a4415 = {
                        let a4413 = v1;
                        let a4414 = 4u8;
                        fixed_u8::bit_and(a4413, a4414)
                    };
                    let a4416 = 4u8;
                    if equal(a4415, a4416) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4417, a4418)?
            };
            let a4430 = {
                let a4427 = {
                    let a4421 = {
                        let a4419 = v1;
                        let a4420 = 2u8;
                        fixed_u8::bit_and(a4419, a4420)
                    };
                    let a4422 = 2u8;
                    if equal(a4421, a4422) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4428 = {
                    let a4425 = {
                        let a4423 = v1;
                        let a4424 = 1u8;
                        fixed_u8::bit_and(a4423, a4424)
                    };
                    let a4426 = 1u8;
                    if equal(a4425, a4426) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4427, a4428)?
            };
            nat_add(a4429, a4430)?
        };
        nat_add(a4431, a4432)?
    };
    let a4439 = f212(f716(), {
        let a4433 = v2;
        let a4434 = 16u64;
        let a4435 = 0u64;
        nat_quot(a4433, a4434, a4435)
    });
    let a4440 = f212(f716(), {
        let a4436 = v2;
        let a4437 = 16u64;
        let a4438 = 0u64;
        nat_rem(a4436, a4437, a4438)
    });
    Ok(append_bytes(a4439, a4440))
}

pub fn f641(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f717(Fn21::F718, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f642(v0: Fn16, p4444: List<List<(Bytes, Bytes)>>, p4445: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m4443 = (p4444, p4445);
    {
        let mut m4443 = m4443;
        loop {
            let step = {
                let (v1, v2) = m4443;
                let m4441 = v2;
                match m4441.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c4442: Fn16 = v0.clone();
                            c4442.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m4443 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f643(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f644(v0: Fn17, p4450: u64, p4451: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m4449 = (p4450, p4451);
    {
        let mut m4449 = m4449;
        loop {
            let step = {
                let (v1, v2) = m4449;
                let m4446 = v1;
                if m4446 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4446 - 1;
                    let m4448 = {
                        let c4447: Fn17 = v0.clone();
                        c4447.apply(v2.clone())?
                    };
                    match m4448 {
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
                Ok(next) => { m4449 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f645(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f719(v0)
}

pub fn f646(v0: Fn18, p4455: List<List<(Bytes, ())>>, p4456: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m4454 = (p4455, p4456);
    {
        let mut m4454 = m4454;
        loop {
            let step = {
                let (v1, v2) = m4454;
                let m4452 = v2;
                match m4452.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c4453: Fn18 = v0.clone();
                            c4453.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m4454 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f647(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f648(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a4458 = {
        let a4457 = v0.clone();
        length_list(a4457)
    };
    let a4459 = 1u64;
    if nat_le(a4458, a4459) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f720(v0)?))
    }
}

pub fn f649(v0: Fn19, p4463: List<(Bytes, ())>, p4464: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m4462 = (p4463, p4464);
    {
        let mut m4462 = m4462;
        loop {
            let step = {
                let (v1, v2) = m4462;
                let m4460 = v2;
                match m4460.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c4461: Fn19 = v0.clone();
                            c4461.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m4462 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f650(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f651(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a4465 = 0u64;
    let a4466 = v1;
    if if nat_lt(a4465, a4466) {
        f211({
            let a4469 = f209(v0, {
                let a4467 = v1;
                let a4468 = 1u64;
                nat_sub(a4467, a4468)
            })?;
            let a4470 = 10u64;
            nat_eq(a4469, a4470)
        })
    } else {
        false
    } {
        Ok(Some({
            let a4471 = v1;
            let a4472 = 1u64;
            nat_sub(a4471, a4472)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f652(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f305(f209(v0, v1)?) {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a4473 = v1;
            let a4474 = 1u64;
            nat_add(a4473, a4474)?
        }))
    }
}

pub fn f653(v0: Bytes, v1: u64, v2: u64) -> R<Adt38> {
    let v3: (u64, u64) = {
        let (h4475, _) = f274(Fn6::F721(v2, v0.clone()), {
            let a4477 = {
                let a4476 = v0.clone();
                length_bytes(a4476)
            };
            let a4478 = 1u64;
            nat_add(a4477, a4478)?
        }, (v1, 0u64))?;
        h4475
    };
    let v4: u64 = f654(v0.clone(), {
        let (h4479, _) = v3;
        h4479
    }, v2)?;
    let a4480 = f209(v0, v4)?;
    let a4481 = 9u64;
    if if nat_eq(a4480, a4481) {
        let a4485 = {
            let a4483 = v4;
            let a4484 = {
                let (h4482, _) = v3;
                h4482
            };
            nat_sub(a4483, a4484)
        };
        let a4486 = v2;
        nat_lt(a4485, a4486)
    } else {
        false
    } {
        Ok(Adt38::C1(f137(Str::lit("a tab cannot indent a block scalar line (YAML 1.2.2 [63], [70])"), v4)?))
    } else {
        Ok(Adt38::C0({
            let (_, h4487) = v3;
            h4487
        }, {
            let (h4488, _) = v3;
            h4488
        }))
    }
}

pub fn f654(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h4489, _) = f204(Fn5::F722(v2, v0.clone(), v1), {
        let a4491 = {
            let a4490 = v0;
            length_bytes(a4490)
        };
        let a4492 = 1u64;
        nat_add(a4491, a4492)?
    }, v1)?;
    Ok(h4489)
}

pub fn f655(v0: Bytes, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = f654(v0.clone(), v1, v2)?;
    let a4495 = {
        let a4493 = v3;
        let a4494 = v1;
        nat_sub(a4493, a4494)
    };
    let a4496 = v2;
    if nat_eq(a4495, a4496) {
        Ok(f211({
            let a4497 = f209(v0, v3)?;
            let a4498 = 256u64;
            nat_eq(a4497, a4498)
        }))
    } else {
        Ok(false)
    }
}

pub fn f656(v0: Bytes, p4508: u64, v2: u64, v3: bool, p4509: u64, p4510: Bytes, p4511: u64) -> R<Adt39> {
    let m4507 = (p4508, (p4509, (p4510, p4511)));
    {
        let mut m4507 = m4507;
        loop {
            let step = {
                let (v1, (v4, (v5, v6))) = m4507;
                let m4499 = v4;
                if m4499 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<Adt39>>(Ok(Adt39::C1(f137(Str::lit("a block scalar has more lines than the input"), v1)?)))
                } else {
                    let v7 = m4499 - 1;
                    let v8: u64 = f654(v0.clone(), v1, v2)?;
                    let v9: bool = f211(f304(f209(v0.clone(), v8)?));
                    let v10: u64 = f424(v0.clone(), v8)?;
                    let v11: Bytes = f7(List::cons(v5, List::cons(f546(v6), List::cons(f321(v0.clone(), v8, v10), List::<Bytes>::nil()))));
                    let m4500 = f653(v0.clone(), f308(v0.clone(), v10)?, v2)?;
                    match m4500 {
                        Adt38::C0(v13, v14) => {
                            let v15: u64 = v14;
                            let v16: u64 = f654(v0.clone(), v15, v2)?;
                            if f655(v0.clone(), v15, v2)? {
                                Ok::<(u64, (u64, (Bytes, u64))), R<Adt39>>((v15, (v7, ({
                                    let a4503 = v11;
                                    let a4504 = if if if v3 {
                                        v9
                                    } else {
                                        false
                                    } {
                                        f211(f304(f209(v0.clone(), v16)?))
                                    } else {
                                        false
                                    } {
                                        let a4501 = v13;
                                        let a4502 = 0u64;
                                        if nat_eq(a4501, a4502) {
                                            Bytes::lit(&[0x20])
                                        } else {
                                            Bytes::lit(&[])
                                        }
                                    } else {
                                        Bytes::lit(&[0x0a])
                                    };
                                    append_bytes(a4503, a4504)
                                }, v13))))
                            } else {
                                Err::<(u64, (u64, (Bytes, u64))), R<Adt39>>(Ok(Adt39::C0(v11, v13, {
                                    let a4505 = f209(v0.clone(), v16)?;
                                    let a4506 = 256u64;
                                    if nat_eq(a4505, a4506) {
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
                Ok(next) => { m4507 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f657(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f309(v0.clone(), {
        let (h4512, _) = v1;
        h4512
    })?;
    let a4513 = f209(v0, v2)?;
    let a4514 = 10u64;
    if nat_eq(a4513, a4514) {
        Ok(Some(({
            let a4515 = v2;
            let a4516 = 1u64;
            nat_add(a4515, a4516)?
        }, f548({
            let (_, h4517) = v1;
            h4517
        }, {
            let a4519 = v2;
            let a4520 = {
                let (h4518, _) = v1;
                h4518
            };
            nat_sub(a4519, a4520)
        }))))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f658(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f723(v0, v1)
}

pub fn f659(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt22> {
    let v4: (u64, u64) = f661(v0.clone(), v1, 2u64)?;
    let a4523 = {
        let (h4521, _) = v4;
        h4521
    };
    let a4524 = {
        let a4522 = v0.clone();
        length_bytes(a4522)
    };
    if nat_eq(a4523, a4524) {
        if f187(false, {
            let (_, h4525) = v4;
            h4525
        }) {
            Ok(Adt22::C0(Adt2::C3(false, {
                let (_, h4526) = v4;
                h4526
            }), v3))
        } else {
            Ok(Adt22::C1(f660(v2)?))
        }
    } else {
        Ok(Adt22::C0(Adt2::C6(v0), v3))
    }
}

pub fn f660(v0: u64) -> R<Bytes> {
    f137(Str::lit("a number outside the range Studio.Value holds is not read (YAML 1.2.2 section 10.3.2)"), v0)
}

pub fn f661(v0: Bytes, v1: u64, v2: u64) -> R<(u64, u64)> {
    let (h4527, _) = f274(Fn6::F724(v1, v0.clone()), {
        let a4529 = {
            let a4528 = v0;
            length_bytes(a4528)
        };
        let a4530 = 1u64;
        nat_add(a4529, a4530)?
    }, (v2, 0u64))?;
    Ok(h4527)
}

pub fn f662(v0: u64) -> R<Option<u64>> {
    let v1: u64 = f320(f8(Str::lit("abtnvfr")), v0)?;
    let a4531 = v1;
    let a4532 = 7u64;
    if nat_lt(a4531, a4532) {
        Ok(Some({
            let a4533 = v1;
            let a4534 = 7u64;
            nat_add(a4533, a4534)?
        }))
    } else {
        let a4535 = v0;
        let a4536 = 48u64;
        if nat_eq(a4535, a4536) {
            Ok(Some(0u64))
        } else {
            let a4537 = v0;
            let a4538 = 9u64;
            if nat_eq(a4537, a4538) {
                Ok(Some(9u64))
            } else {
                let a4539 = v0;
                let a4540 = 101u64;
                if nat_eq(a4539, a4540) {
                    Ok(Some(27u64))
                } else {
                    let a4541 = v0;
                    let a4542 = 78u64;
                    if nat_eq(a4541, a4542) {
                        Ok(Some(133u64))
                    } else {
                        let a4543 = v0;
                        let a4544 = 95u64;
                        if nat_eq(a4543, a4544) {
                            Ok(Some(160u64))
                        } else {
                            let a4545 = v0;
                            let a4546 = 76u64;
                            if nat_eq(a4545, a4546) {
                                Ok(Some(8232u64))
                            } else {
                                let a4547 = v0;
                                let a4548 = 80u64;
                                if nat_eq(a4547, a4548) {
                                    Ok(Some(8233u64))
                                } else {
                                    let a4549 = f320(f8(Str::lit(" \"/\\")), v0)?;
                                    let a4550 = 4u64;
                                    if nat_lt(a4549, a4550) {
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

pub fn f663(v0: u64) -> R<Bytes> {
    f137(Str::lit("an implicit key must be on one line (YAML 1.2.2 [154])"), v0)
}

pub fn f664(v0: Bytes, p4565: u64, v2: u64, p4566: u64, p4567: u64) -> R<Adt38> {
    let m4564 = (p4565, (p4566, p4567));
    {
        let mut m4564 = m4564;
        loop {
            let step = {
                let (v1, (v3, v4)) = m4564;
                let m4551 = v3;
                if m4551 == 0 {
                    Err::<(u64, (u64, u64)), R<Adt38>>(Ok(Adt38::C1(f137(Str::lit("the input ends inside a scalar"), v1)?)))
                } else {
                    let v5 = m4551 - 1;
                    let v6: u64 = {
                        let a4552 = v1;
                        let a4553 = 1u64;
                        nat_add(a4552, a4553)?
                    };
                    let v7: u64 = f309(v0.clone(), v6)?;
                    let v8: u64 = f307(v0.clone(), v7)?;
                    let v9: u64 = f209(v0.clone(), v8)?;
                    if f207(v0.clone(), v6)? {
                        Err::<(u64, (u64, u64)), R<Adt38>>(Ok(Adt38::C1(f137(Str::lit("a document marker cannot appear inside a scalar (YAML 1.2.2 [206])"), v6)?)))
                    } else {
                        let a4554 = v9;
                        let a4555 = 10u64;
                        if nat_eq(a4554, a4555) {
                            Ok::<(u64, (u64, u64)), R<Adt38>>((v8, (v5, {
                                let a4556 = v4;
                                let a4557 = 1u64;
                                nat_add(a4556, a4557)?
                            })))
                        } else {
                            let a4558 = v9;
                            let a4559 = 256u64;
                            if nat_eq(a4558, a4559) {
                                Err::<(u64, (u64, u64)), R<Adt38>>(Ok(Adt38::C1(f137(Str::lit("the input ends inside a quoted scalar (YAML 1.2.2 [109], [120])"), v8)?)))
                            } else {
                                let a4562 = {
                                    let a4560 = v7;
                                    let a4561 = v6;
                                    nat_sub(a4560, a4561)
                                };
                                let a4563 = v2;
                                if nat_lt(a4562, a4563) {
                                    Err::<(u64, (u64, u64)), R<Adt38>>(Ok(Adt38::C1(f137(Str::lit("a continuation line of a scalar is indented less than its node (YAML 1.2.2 [69])"), v7)?)))
                                } else {
                                    Err::<(u64, (u64, u64)), R<Adt38>>(Ok(Adt38::C0(v4, v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4564 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f665(v0: Bytes, p4577: u64, p4578: u64, p4579: u64) -> R<Option<u64>> {
    let m4576 = (p4577, (p4578, p4579));
    {
        let mut m4576 = m4576;
        loop {
            let step = {
                let (v1, (v2, v3)) = m4576;
                let m4568 = v2;
                if m4568 == 0 {
                    Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(Some(v3)))
                } else {
                    let v4 = m4568 - 1;
                    let m4569 = f725(f209(v0.clone(), v1)?);
                    match m4569 {
                        None => {
                            Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(None::<u64>))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, u64)), R<Option<u64>>>(({
                                let a4570 = v1;
                                let a4571 = 1u64;
                                nat_add(a4570, a4571)?
                            }, (v4, {
                                let a4574 = {
                                    let a4572 = v3;
                                    let a4573 = 16u64;
                                    nat_mul(a4572, a4573)?
                                };
                                let a4575 = v5;
                                nat_add(a4574, a4575)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4576 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f666(v0: Bytes, v1: u64) -> R<u64> {
    let (h4580, _) = f204(Fn5::F726(v0.clone()), {
        let a4582 = {
            let a4581 = v0;
            length_bytes(a4581)
        };
        let a4583 = 1u64;
        nat_add(a4582, a4583)?
    }, v1)?;
    Ok(h4580)
}

pub fn f667(v0: Bytes, v1: u64) -> R<u64> {
    let (h4584, _) = f204(Fn5::F727(v0.clone()), {
        let a4586 = {
            let a4585 = v0;
            length_bytes(a4585)
        };
        let a4587 = 1u64;
        nat_add(a4586, a4587)?
    }, v1)?;
    Ok(h4584)
}

pub fn f668(v0: Fn20, p4592: u64, p4593: (u64, bool)) -> R<((u64, bool), bool)> {
    let m4591 = (p4592, p4593);
    {
        let mut m4591 = m4591;
        loop {
            let step = {
                let (v1, v2) = m4591;
                let m4588 = v1;
                if m4588 == 0 {
                    Err::<(u64, (u64, bool)), R<((u64, bool), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4588 - 1;
                    let m4590 = {
                        let c4589: Fn20 = v0.clone();
                        c4589.apply(v2)?
                    };
                    match m4590 {
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
                Ok(next) => { m4591 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f669(v0: u64, v1: Bytes, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    f728(v1, v0, v2)
}

pub fn f670(v0: Bytes, v1: u64, v2: u64) -> R<Adt32> {
    let v3: u64 = f209(v0.clone(), v1)?;
    let a4594 = v3;
    let a4595 = 34u64;
    if if nat_eq(a4594, a4595) {
        true
    } else {
        let a4596 = v3;
        let a4597 = 39u64;
        nat_eq(a4596, a4597)
    } {
        f429(v0, v1, v2, false)
    } else {
        let m4598 = f312(v0.clone(), v1)?;
        match m4598 {
            None => {
                let a4599 = v3;
                let a4600 = 91u64;
                if if nat_eq(a4599, a4600) {
                    true
                } else {
                    let a4601 = v3;
                    let a4602 = 123u64;
                    nat_eq(a4601, a4602)
                } {
                    Ok(Adt32::C1(f137(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [142])"), v1)?))
                } else {
                    if f431(v0.clone(), v1, true)? {
                        let v5: u64 = f432(v0.clone(), v1, true)?;
                        let v6: u64 = f307(v0.clone(), v5)?;
                        let a4603 = f209(v0.clone(), v6)?;
                        let a4604 = 10u64;
                        if nat_eq(a4603, a4604) {
                            let m4605 = f558(v0.clone(), v6, v2)?;
                            match m4605 {
                                Ok(v8) => {
                                    let v9: u64 = f209(v0.clone(), v8)?;
                                    let a4606 = v9;
                                    let a4607 = 58u64;
                                    if if if nat_eq(a4606, a4607) {
                                        true
                                    } else {
                                        let a4608 = v9;
                                        let a4609 = 44u64;
                                        nat_eq(a4608, a4609)
                                    } {
                                        true
                                    } else {
                                        let a4610 = v9;
                                        let a4611 = 125u64;
                                        nat_eq(a4610, a4611)
                                    } {
                                        Ok(Adt32::C0(f321(v0, v1, v5), v5))
                                    } else {
                                        Ok(Adt32::C1(f137(Str::lit("a multi-line plain scalar inside a flow collection is not read (YAML 1.2.2 [135])"), v6)?))
                                    }
                                }
                                Err(v7) => {
                                    Ok(Adt32::C1(v7))
                                }
                            }
                        } else {
                            Ok(Adt32::C0(f321(v0, v1, v5), v5))
                        }
                    } else {
                        Ok(Adt32::C1(f137(Str::lit("a flow mapping key must be a scalar (YAML 1.2.2 [142])"), v1)?))
                    }
                }
            }
            Some(v4) => {
                Ok(Adt32::C1(v4))
            }
        }
    }
}

pub fn f671(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = {
        let (h4612, _) = v2;
        h4612
    };
    let v4: u64 = f209(v0.clone(), v3)?;
    if f305(v4) {
        Ok(None::<(u64, u64)>)
    } else {
        if f304(v4) {
            Ok(Some(({
                let a4613 = v3;
                let a4614 = 1u64;
                nat_add(a4613, a4614)?
            }, {
                let (_, h4615) = v2;
                h4615
            })))
        } else {
            let a4616 = v4;
            let a4617 = 58u64;
            if if nat_eq(a4616, a4617) {
                f211(f426(f209(v0.clone(), {
                    let a4618 = v3;
                    let a4619 = 1u64;
                    nat_add(a4618, a4619)?
                })?, v1))
            } else {
                false
            } {
                Ok(None::<(u64, u64)>)
            } else {
                let a4620 = v4;
                let a4621 = 35u64;
                if if nat_eq(a4620, a4621) {
                    f304(f209(v0, {
                        let a4622 = v3;
                        let a4623 = 1u64;
                        nat_sub(a4622, a4623)
                    })?)
                } else {
                    false
                } {
                    Ok(None::<(u64, u64)>)
                } else {
                    if if v1 {
                        f541(v4)
                    } else {
                        false
                    } {
                        Ok(None::<(u64, u64)>)
                    } else {
                        Ok(Some(({
                            let a4624 = v3;
                            let a4625 = 1u64;
                            nat_add(a4624, a4625)?
                        }, {
                            let a4626 = v3;
                            let a4627 = 1u64;
                            nat_add(a4626, a4627)?
                        })))
                    }
                }
            }
        }
    }
}

pub fn f672(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f729(v0, v1)
}

pub fn f673(v0: Bytes, v1: bool, v2: u64) -> R<Option<u64>> {
    let v3: u64 = f209(v0, v2)?;
    if if if f304(v3) {
        true
    } else {
        f305(v3)
    } {
        true
    } else {
        if v1 {
            let a4628 = v3;
            let a4629 = 62u64;
            nat_eq(a4628, a4629)
        } else {
            f541(v3)
        }
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a4630 = v2;
            let a4631 = 1u64;
            nat_add(a4630, a4631)?
        }))
    }
}

pub fn f674(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    if f730(v1, f163(v0, v2)?) {
        Ok(Some({
            let a4632 = v2;
            let a4633 = 1u64;
            nat_add(a4632, a4633)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f675(p4636: List<Adt14>, p4637: List<Adt14>) -> List<Adt14> {
    let m4635 = (p4636, p4637);
    {
        let mut m4635 = m4635;
        loop {
            let step = {
                let (v0, v1) = m4635;
                let m4634 = v0;
                match m4634.uncons() {
                    None => {
                        Err::<(List<Adt14>, List<Adt14>), List<Adt14>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt14>, List<Adt14>), List<Adt14>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m4635 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f676(v0: Bytes, p4642: List<(Bytes, Adt15)>) -> Option<Adt15> {
    let m4641 = p4642;
    {
        let mut m4641 = m4641;
        loop {
            let step = {
                let v1 = m4641;
                let m4638 = v1;
                match m4638.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt15)>, Option<Adt15>>(None::<Adt15>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h4639, _) = v2;
                            h4639
                        }, v0.clone()) {
                            Err::<List<(Bytes, Adt15)>, Option<Adt15>>(Some({
                                let (_, h4640) = v2;
                                h4640
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt15)>, Option<Adt15>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4641 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f677() -> List<(Bytes, Adt15)> {
    List::cons((f8(Str::lit("boolean")), f456(f8(Str::lit("Bool")))), List::cons((f8(Str::lit("byte")), f456(f8(Str::lit("Int")))), List::cons((f8(Str::lit("octet")), f456(f8(Str::lit("Nat")))), List::cons((f8(Str::lit("bigint")), f456(f8(Str::lit("Int")))), List::cons((f8(Str::lit("float")), f456(f607())), List::cons((f8(Str::lit("double")), f456(f607())), List::cons((f8(Str::lit("DOMString")), f456(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("USVString")), f456(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ByteString")), f456(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ArrayBuffer")), f456(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("SharedArrayBuffer")), f456(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("DataView")), f456(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("Int8Array")), f456(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int16Array")), f456(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int32Array")), f456(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("BigInt64Array")), f456(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Uint8Array")), f456(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint16Array")), f456(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint32Array")), f456(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint8ClampedArray")), f456(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("BigUint64Array")), f456(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Float16Array")), f584(f8(Str::lit("Float16Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("Float32Array")), f731(false)), List::cons((f8(Str::lit("Float64Array")), f731(true)), List::cons((f8(Str::lit("any")), f584(f8(Str::lit("any is not translated, because it is the union of every Web IDL type and has no one LexLean type (Web IDL 2.13.1)")))), List::cons((f8(Str::lit("object")), f584(f8(Str::lit("object is not translated, because its values are references to JavaScript objects of any shape, which no LexLean type holds (Web IDL 2.13.20)")))), List::cons((f8(Str::lit("symbol")), f584(f8(Str::lit("symbol is not translated, because a symbol value has identity but no content that a LexLean type holds (Web IDL 2.13.21)")))), List::cons((f8(Str::lit("undefined")), f584(f8(Str::lit("undefined is not translated, because it is the type of the absence of a value (Web IDL 2.13.2)")))), List::<(Bytes, Adt15)>::nil()))))))))))))))))))))))))))))
}

pub fn f678() -> Bytes {
    f8(Str::lit("a union type is translated only as the whole body of a typedef, whose identifier names its inductive (Web IDL 2.11 and 2.13.32)"))
}

pub fn f679(v0: u64) -> Bytes {
    f46({
        let a4643 = Str::lit("0123456789ABCDEF");
        utf8_encode(a4643)
    }, v0, 1u64)
}

pub fn f680(p4654: u64, v1: Bytes, v2: Bytes, p4655: u64) -> R<bool> {
    let m4653 = (p4654, p4655);
    {
        let mut m4653 = m4653;
        loop {
            let step = {
                let (v0, v3) = m4653;
                let m4644 = v0;
                if m4644 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(false))
                } else {
                    let v4 = m4644 - 1;
                    let v5: u64 = f732(v1.clone(), v3)?;
                    let v6: u64 = f732(v2.clone(), v3)?;
                    let a4645 = v5;
                    let a4646 = v6;
                    if nat_lt(a4645, a4646) {
                        Err::<(u64, u64), R<bool>>(Ok(true))
                    } else {
                        let a4647 = v6;
                        let a4648 = v5;
                        if nat_lt(a4647, a4648) {
                            Err::<(u64, u64), R<bool>>(Ok(false))
                        } else {
                            let a4649 = v5;
                            let a4650 = 0u64;
                            if nat_eq(a4649, a4650) {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            } else {
                                Ok::<(u64, u64), R<bool>>((v4, {
                                    let a4651 = v3;
                                    let a4652 = 1u64;
                                    nat_add(a4651, a4652)?
                                }))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4653 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f681(p4668: u64, v1: Bytes, p4669: u64, p4670: Bytes) -> R<Option<Bytes>> {
    let m4667 = (p4668, (p4669, p4670));
    {
        let mut m4667 = m4667;
        loop {
            let step = {
                let (v0, (v2, v3)) = m4667;
                let m4656 = v0;
                if m4656 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m4656 - 1;
                    let a4658 = {
                        let a4657 = v1.clone();
                        length_bytes(a4657)
                    };
                    let a4659 = v2;
                    if nat_le(a4658, a4659) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m4660 = f419(v1.clone(), v2)?;
                        match m4660 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a4662 = v2;
                                    let a4663 = {
                                        let (_, h4661) = v5;
                                        h4661
                                    };
                                    nat_add(a4662, a4663)?
                                }, {
                                    let a4665 = v3;
                                    let a4666 = f733(v1.clone(), v2, {
                                        let (h4664, _) = v5;
                                        h4664
                                    });
                                    append_bytes(a4665, a4666)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4667 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f682(p4680: u64, v1: Bytes, p4681: u64) -> R<bool> {
    let m4679 = (p4680, p4681);
    {
        let mut m4679 = m4679;
        loop {
            let step = {
                let (v0, v2) = m4679;
                let m4671 = v0;
                if m4671 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m4671 - 1;
                    let m4674 = {
                        let a4672 = v1.clone();
                        let a4673 = v2;
                        index_bytes(a4672, a4673)
                    };
                    match m4674 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a4675 = f62(v4)?;
                            let a4676 = 128u64;
                            if nat_lt(a4675, a4676) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a4677 = v2;
                                    let a4678 = 1u64;
                                    nat_add(a4677, a4678)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4679 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f683(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f195(v0.clone(), 0u64, {
        let a4682 = v0;
        length_bytes(a4682)
    }, false, Bytes::lit(&[]))?;
    let m4685 = {
        let a4683 = v1.clone();
        let a4684 = 0u64;
        index_bytes(a4683, a4684)
    };
    match m4685 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f62(v2)?;
            if f284(v3) {
                let a4692 = f46({
                    let a4686 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a4686)
                }, {
                    let a4687 = v3;
                    let a4688 = 65u64;
                    nat_sub(a4687, a4688)
                }, 1u64);
                let a4693 = f46(v1.clone(), 1u64, {
                    let a4690 = {
                        let a4689 = v1;
                        length_bytes(a4689)
                    };
                    let a4691 = 1u64;
                    nat_sub(a4690, a4691)
                });
                Ok(append_bytes(a4692, a4693))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f684(v0: Bytes) -> R<bool> {
    let m4696 = {
        let a4694 = v0;
        let a4695 = 0u64;
        index_bytes(a4694, a4695)
    };
    match m4696 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f183(f283(f62(v1)?), f284(f62(v1)?)))
        }
    }
}

pub fn f685(p4709: u64, v1: Bytes, p4710: u64, p4711: Bytes) -> R<Option<Bytes>> {
    let m4708 = (p4709, (p4710, p4711));
    {
        let mut m4708 = m4708;
        loop {
            let step = {
                let (v0, (v2, v3)) = m4708;
                let m4697 = v0;
                if m4697 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m4697 - 1;
                    let a4699 = {
                        let a4698 = v1.clone();
                        length_bytes(a4698)
                    };
                    let a4700 = v2;
                    if nat_le(a4699, a4700) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m4701 = f419(v1.clone(), v2)?;
                        match m4701 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a4703 = v2;
                                    let a4704 = {
                                        let (_, h4702) = v5;
                                        h4702
                                    };
                                    nat_add(a4703, a4704)?
                                }, {
                                    let a4706 = v3;
                                    let a4707 = f734({
                                        let (h4705, _) = v5;
                                        h4705
                                    });
                                    append_bytes(a4706, a4707)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4708 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f686() -> List<Bytes> {
    List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("termination_by")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("admit")), List::cons(f8(Str::lit("below")), List::cons(f8(Str::lit("brecOn")), List::cons(f8(Str::lit("casesOn")), List::cons(f8(Str::lit("ctorElim")), List::cons(f8(Str::lit("ctorElimType")), List::cons(f8(Str::lit("ctorIdx")), List::cons(f8(Str::lit("noConfusion")), List::cons(f8(Str::lit("noConfusionType")), List::cons(f8(Str::lit("rec")), List::cons(f8(Str::lit("recOn")), List::cons(f8(Str::lit("toCtorIdx")), List::<Bytes>::nil())))))))))))))))))))
}

pub fn f687() -> Bytes {
    f8(Str::lit("\u{0}\u{0}:\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{5}\u{7f}\u{0}\u{6}p\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f688() -> Bytes {
    f8(Str::lit("\u{0}\u{0}-\u{0}\u{0}.\u{0}\u{0}0\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}7\u{0}\u{1}7\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@?\u{0}@@\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f689(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f163(v0, {
        let (h4712, _) = v2;
        h4712
    })?;
    let a4714 = 1114111u64;
    let a4715 = {
        let (_, h4713) = v2;
        h4713
    };
    if nat_lt(a4714, a4715) {
        Ok(None::<(u64, u64)>)
    } else {
        let v4: u64 = f735(v3, v1);
        let a4716 = v4;
        let a4717 = 16u64;
        if nat_eq(a4716, a4717) {
            Ok(None::<(u64, u64)>)
        } else {
            Ok(Some(({
                let a4719 = {
                    let (h4718, _) = v2;
                    h4718
                };
                let a4720 = 1u64;
                nat_add(a4719, a4720)?
            }, {
                let a4724 = {
                    let a4722 = {
                        let (_, h4721) = v2;
                        h4721
                    };
                    let a4723 = if v1 {
                        16u64
                    } else {
                        10u64
                    };
                    nat_mul(a4722, a4723)?
                };
                let a4725 = v4;
                nat_add(a4724, a4725)?
            })))
        }
    }
}

pub fn f690() -> Bytes {
    f8(Str::lit("\u{80}\u{81}\u{82}\u{83}\u{84}\u{85}\u{86}\u{87}\u{88}\u{89}\u{8a}\u{8b}\u{8c}\u{8d}\u{8e}\u{8f}\u{90}\u{91}\u{92}\u{93}\u{94}\u{95}\u{96}\u{97}\u{98}\u{99}\u{9a}\u{9b}\u{9c}\u{9d}\u{9e}\u{9f}\u{a0}\u{a1}\u{a2}\u{a3}\u{a4}\u{a5}\u{a6}\u{a7}\u{a8}\u{a9}\u{aa}\u{ab}\u{ac}\u{ad}\u{ae}\u{af}\u{b0}\u{b1}\u{b2}\u{b3}\u{b4}\u{b5}\u{b6}\u{b7}\u{b8}\u{b9}\u{ba}\u{bb}\u{bc}\u{bd}\u{be}\u{bf}"))
}

pub fn f691(v0: u64, v1: bool, v2: u64) -> R<bool> {
    if v1 {
        let a4726 = v0;
        let a4727 = v2;
        if nat_eq(a4726, a4727) {
            Ok(true)
        } else {
            f351(v0, f736())
        }
    } else {
        f351(v0, f737())
    }
}

pub fn f692() -> Bytes {
    f8(Str::lit(" string:anyAtomicType boolean:anyAtomicType decimal:anyAtomicType float:anyAtomicType double:anyAtomicType duration:anyAtomicType dateTime:anyAtomicType time:anyAtomicType date:anyAtomicType gYearMonth:anyAtomicType gYear:anyAtomicType gMonthDay:anyAtomicType gDay:anyAtomicType gMonth:anyAtomicType hexBinary:anyAtomicType base64Binary:anyAtomicType anyURI:anyAtomicType QName:anyAtomicType NOTATION:anyAtomicType normalizedString:string token:normalizedString language:token NMTOKEN:token Name:token NCName:Name ID:NCName IDREF:NCName ENTITY:NCName integer:decimal nonPositiveInteger:integer long:integer nonNegativeInteger:integer negativeInteger:nonPositiveInteger int:long short:int byte:short unsignedLong:nonNegativeInteger positiveInteger:nonNegativeInteger unsignedInt:unsignedLong unsignedShort:unsignedInt unsignedByte:unsignedShort yearMonthDuration:duration dayTimeDuration:duration dateTimeStamp:dateTime NMTOKENS:anySimpleType IDREFS:anySimpleType ENTITIES:anySimpleType anyAtomicType:anySimpleType "))
}

pub fn f693(v0: Adt27, v1: Str, v2: Str) -> bool {
    let m4728 = f381(v0, v1);
    match m4728 {
        None => {
            false
        }
        Some(v3) => {
            f32(v3, f8(v2))
        }
    }
}

pub fn f694(v0: List<Adt26>, v1: (Bytes, Bytes), v2: bool) -> R<Result<List<Bytes>, Bytes>> {
    let m4729 = v0;
    match m4729.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m4730 = f694(v4, v1.clone(), v2)?;
            match m4730 {
                Ok(v6) => {
                    if f32(match v3.clone() {
                        Adt26::C0(h4731, _, _, _) => {
                            h4731
                        }
                    }, f8(Str::lit("complexType"))) {
                        let m4732 = f738(v3.clone())?;
                        match m4732 {
                            Ok(v8) => {
                                let m4733 = v8;
                                match m4733 {
                                    None => {
                                        Ok(Ok::<List<Bytes>, Bytes>(v6))
                                    }
                                    Some(v9) => {
                                        if if f32({
                                            let (h4734, _) = {
                                                let (_, h4735) = v9.clone();
                                                h4735
                                            };
                                            h4734
                                        }, {
                                            let (h4736, _) = v1;
                                            h4736
                                        }) {
                                            if f32({
                                                let (_, h4737) = {
                                                    let (_, h4738) = v9;
                                                    h4738
                                                };
                                                h4737
                                            }, {
                                                let (_, h4739) = v1;
                                                h4739
                                            }) {
                                                if f379(v2) {
                                                    true
                                                } else {
                                                    f380({
                                                        let (h4740, _) = v9;
                                                        h4740
                                                    }, Str::lit("extension"))
                                                }
                                            } else {
                                                false
                                            }
                                        } else {
                                            false
                                        } {
                                            Ok(Ok::<List<Bytes>, Bytes>(List::cons(match v3 {
                                                Adt26::C0(_, _, h4741, _) => {
                                                    h4741
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

pub fn f695(v0: Bytes, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<Adt28, Bytes>> {
    let m4742 = v1.clone();
    match m4742.uncons() {
        None => {
            Ok(Err::<Adt28, Bytes>(f616(f8(Str::lit("a choice with no particle that can occur matches no element sequence, so it has no LexLean constructor (XSD 1.1 Part 1, section 3.8.4.3)")), v0)))
        }
        Some((_, _)) => {
            let m4743 = f739(v1)?;
            match m4743 {
                Ok(v7) => {
                    let v8: List<Bytes> = f740(v7, v2);
                    Ok(Ok::<Adt28, Bytes>(Adt28::C0(v0.clone(), v3, f343(v0, v8.clone()), v8)))
                }
                Err(v6) => {
                    Ok(Err::<Adt28, Bytes>(f616(v6, v0)))
                }
            }
        }
    }
}

pub fn f696(v0: List<Bytes>) -> R<List<Bytes>> {
    let m4744 = v0;
    match m4744.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f125(v1)?, f696(v2)?))
        }
    }
}

pub fn f697(v0: u64, v1: Adt25, v2: Bytes, v3: Bytes, v4: Adt27) -> R<Result<List<Adt28>, Bytes>> {
    let m4745 = v0;
    if m4745 == 0 {
        Ok(Err::<List<Adt28>, Bytes>(f616(f8(Str::lit("the complex types nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m4745 - 1;
        let m4746 = f741(v5, v1, v2.clone(), v3, v4, f619())?;
        match m4746 {
            Ok(v7) => {
                let m4748 = f705(v2, match v7.clone() {
                    Adt36::C0(h4747, _, _, _, _) => {
                        h4747
                    }
                })?;
                match m4748 {
                    Ok(v9) => {
                        Ok(Ok::<List<Adt28>, Bytes>(f489(match v7 {
                            Adt36::C0(_, h4749, _, _, _) => {
                                h4749
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

pub fn f698(v0: Bytes) -> Bytes {
    let a4750 = v0;
    let a4751 = f8(Str::lit("Content"));
    append_bytes(a4750, a4751)
}

pub fn f699(v0: Adt25, v1: Adt27, v2: Bytes) -> R<Result<Adt24, Bytes>> {
    let m4753 = f167(match v1 {
        Adt27::C0(_, _, _, _, h4752) => {
            h4752
        }
    }, v2.clone())?;
    match m4753 {
        Ok(v4) => {
            if f32({
                let (h4754, _) = v4.clone();
                h4754
            }, f250()) {
                if f32({
                    let (_, h4755) = v4.clone();
                    h4755
                }, f8(Str::lit("anyType"))) {
                    Ok(Ok::<Adt24, Bytes>(Adt24::C0(f492(), List::cons(f492(), List::<Bytes>::nil()))))
                } else {
                    let m4757 = f484({
                        let (_, h4756) = v4;
                        h4756
                    }, 16u64)?;
                    match m4757 {
                        Ok(v6) => {
                            Ok(Ok::<Adt24, Bytes>(Adt24::C0(v6, List::<Bytes>::nil())))
                        }
                        Err(v5) => {
                            Ok(Err::<Adt24, Bytes>(v5))
                        }
                    }
                }
            } else {
                if f122({
                    let (h4758, _) = v4.clone();
                    h4758
                }, match v0.clone() {
                    Adt25::C0(_, h4759) => {
                        h4759
                    }
                }) {
                    let v7: Bytes = f125({
                        let (_, h4760) = v4.clone();
                        h4760
                    })?;
                    let m4761 = f706(v0.clone(), Str::lit("complexType"), v4.clone());
                    match m4761 {
                        None => {
                            let m4762 = f706(v0, Str::lit("simpleType"), v4);
                            match m4762 {
                                None => {
                                    Ok(Err::<Adt24, Bytes>(f260(f8(Str::lit("the type ")), v2, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)")))))
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
                    Ok(Err::<Adt24, Bytes>(f260(f8(Str::lit("the type ")), v2, f260(f8(Str::lit(" is in the namespace ")), {
                        let (h4763, _) = v4;
                        h4763
                    }, f8(Str::lit(", which no schema document of this set targets (XSD 1.1 Part 1, sections 3.17.6.2 and 4.2.6)"))))))
                }
            }
        }
        Err(v3) => {
            Ok(Err::<Adt24, Bytes>(v3))
        }
    }
}

pub fn f700(v0: Adt27, v1: Bytes) -> Bytes {
    if f693(v0, Str::lit("nillable"), Str::lit("true")) {
        f742(Str::lit("Option"), v1)
    } else {
        v1
    }
}

pub fn f701(p4768: u64, v1: Adt25, v2: Bytes, v3: Bytes, v4: bool, p4769: List<Adt27>, p4770: Adt36) -> R<Result<Adt36, Bytes>> {
    let m4767 = (p4768, (p4769, p4770));
    {
        let mut m4767 = m4767;
        loop {
            let step = {
                let (v0, (v5, v6)) = m4767;
                let m4764 = v0;
                if m4764 == 0 {
                    Err::<(u64, (List<Adt27>, Adt36)), R<Result<Adt36, Bytes>>>(Ok(Err::<Adt36, Bytes>(f616(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2.clone()))))
                } else {
                    let v7 = m4764 - 1;
                    let m4765 = v5;
                    match m4765.uncons() {
                        None => {
                            Err::<(u64, (List<Adt27>, Adt36)), R<Result<Adt36, Bytes>>>(Ok(Ok::<Adt36, Bytes>(v6)))
                        }
                        Some((v8, v9)) => {
                            let m4766 = f743(v7, v1.clone(), v2.clone(), v3.clone(), v4, v8, v6)?;
                            match m4766 {
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
                Ok(next) => { m4767 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f702(v0: List<Adt37>) -> List<Bytes> {
    let m4771 = v0;
    match m4771.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt37::C0(h4772, _, _) => {
                    h4772
                }
            }, f702(v2))
        }
    }
}

pub fn f703(v0: List<Adt37>) -> List<Bytes> {
    let m4773 = v0;
    match m4773.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt37::C0(_, h4774, _) => {
                    h4774
                }
            }, f703(v2))
        }
    }
}

pub fn f704(v0: List<Adt37>) -> List<Bytes> {
    let m4775 = v0;
    match m4775.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f499(match v1 {
                Adt37::C0(_, _, h4776) => {
                    h4776
                }
            }, f704(v2))
        }
    }
}

pub fn f705(v0: Bytes, v1: List<Adt37>) -> R<Result<Adt28, Bytes>> {
    let m4777 = f739(f702(v1.clone()))?;
    match m4777 {
        Ok(v3) => {
            let v4: List<Bytes> = f703(v1.clone());
            let v7: Bytes = {
                let m4778 = v1.clone();
                match m4778.uncons() {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some((_, _)) => {
                        f260(f8(Str::lit("\u{a}")), f156(f744(v3, v4.clone()), f8(Str::lit(",\u{a}"))), f8(Str::lit("\u{a}  ")))
                    }
                }
            };
            Ok(Ok::<Adt28, Bytes>(Adt28::C0(v0.clone(), f704(v1), f260(f8(Str::lit("structure{\u{a}  name: ")), v0, f260(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), v7, f8(Str::lit("]\u{a}}\u{a}")))), List::cons(f260(f8(Str::lit("    {name: mk, fields: [")), f156(v4, f8(Str::lit(", "))), f8(Str::lit("]}"))), List::<Bytes>::nil()))))
        }
        Err(v2) => {
            Ok(Err::<Adt28, Bytes>(f616(v2, v0)))
        }
    }
}

pub fn f706(v0: Adt25, v1: Str, v2: (Bytes, Bytes)) -> Option<Adt26> {
    f745(match v0 {
        Adt25::C0(h4779, _) => {
            h4779
        }
    }, f8(v1), {
        let (h4780, _) = v2;
        h4780
    }, {
        let (_, h4781) = v2;
        h4781
    })
}

pub fn f707(v0: Adt36, v1: Bytes, v2: Adt35, v3: bool) -> Adt36 {
    f746(f708(v0, Adt37::C0(v1, if v3 {
        let m4784 = match v2.clone() {
            Adt35::C0(h4783, _) => {
                h4783
            }
        };
        match m4784 {
            Adt24::C0(h4782, _) => {
                h4782
            }
        }
    } else {
        f742(Str::lit("Option"), {
            let m4787 = match v2.clone() {
                Adt35::C0(h4786, _) => {
                    h4786
                }
            };
            match m4787 {
                Adt24::C0(h4785, _) => {
                    h4785
                }
            }
        })
    }, {
        let m4790 = match v2.clone() {
            Adt35::C0(h4789, _) => {
                h4789
            }
        };
        match m4790 {
            Adt24::C0(_, h4788) => {
                h4788
            }
        }
    })), match v2 {
        Adt35::C0(_, h4791) => {
            h4791
        }
    })
}

pub fn f708(v0: Adt36, v1: Adt37) -> Adt36 {
    Adt36::C0(f747(match v0.clone() {
        Adt36::C0(h4792, _, _, _, _) => {
            h4792
        }
    }, List::cons(v1, List::<Adt37>::nil())), match v0.clone() {
        Adt36::C0(_, h4793, _, _, _) => {
            h4793
        }
    }, match v0.clone() {
        Adt36::C0(_, _, h4794, _, _) => {
            h4794
        }
    }, match v0.clone() {
        Adt36::C0(_, _, _, h4795, _) => {
            h4795
        }
    }, match v0 {
        Adt36::C0(_, _, _, _, h4796) => {
            h4796
        }
    })
}

pub fn f709() -> Bytes {
    f8(Str::lit("(List ((Bytes * Bytes) * Bytes))"))
}

pub fn f710(v0: Bytes, v1: List<Bytes>, v2: Bytes) -> Bytes {
    f260(f8(Str::lit("inductive{\u{a}  name: ")), v0, f260(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), f156(v1, f8(Str::lit(",\u{a}"))), f260(f8(Str::lit("\u{a}  ],\u{a}  mutual: ")), v2, f8(Str::lit("\u{a}}\u{a}")))))
}

pub fn f711(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h4797, _) = v1;
        h4797
    };
    let m4799 = {
        let (_, h4798) = v1;
        h4798
    };
    match m4799 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m4802 = {
                let a4800 = v0.clone();
                let a4801 = v2;
                index_bytes(a4800, a4801)
            };
            match m4802 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m4807 = {
                        let a4805 = v0;
                        let a4806 = {
                            let a4803 = v2;
                            let a4804 = 1u64;
                            nat_add(a4803, a4804)?
                        };
                        index_bytes(a4805, a4806)
                    };
                    match m4807 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f748(v4)? {
                                f748(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a4808 = v2;
                                    let a4809 = 2u64;
                                    nat_add(a4808, a4809)?
                                }, Some({
                                    let a4814 = v3;
                                    let a4815 = f212(f749(), {
                                        let a4812 = {
                                            let a4810 = f750(v4)?;
                                            let a4811 = 16u64;
                                            nat_mul(a4810, a4811)?
                                        };
                                        let a4813 = f750(v5)?;
                                        nat_add(a4812, a4813)?
                                    });
                                    append_bytes(a4814, a4815)
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

pub fn f712(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4816 = f279(v0, v1);
    let a4817 = f635(v0, v2);
    fixed_u32::bit_or(a4816, a4817)
}

pub fn f713(v0: u32) -> u32 {
    let a4820 = {
        let a4818 = f712(v0, 17u32, 15u32);
        let a4819 = f712(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a4818, a4819)
    };
    let a4821 = f279(v0, 10u32);
    fixed_u32::bit_xor(a4820, a4821)
}

pub fn f714(v0: u32) -> u32 {
    let a4824 = {
        let a4822 = f712(v0, 7u32, 25u32);
        let a4823 = f712(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a4822, a4823)
    };
    let a4825 = f279(v0, 3u32);
    fixed_u32::bit_xor(a4824, a4825)
}

pub fn f715(v0: u64) -> R<Bytes> {
    let v1: Bytes = f91(v0)?;
    let a4829 = f518({
        let a4827 = 9u64;
        let a4828 = {
            let a4826 = v1.clone();
            length_bytes(a4826)
        };
        nat_sub(a4827, a4828)
    });
    let a4830 = v1;
    Ok(append_bytes(a4829, a4830))
}

pub fn f716() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f717(v0: Fn21, p4834: List<List<(Bytes, Bytes)>>, p4835: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m4833 = (p4834, p4835);
    {
        let mut m4833 = m4833;
        loop {
            let step = {
                let (v1, v2) = m4833;
                let m4831 = v2;
                match m4831.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c4832: Fn21 = v0.clone();
                            c4832.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m4833 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f718(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f719(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a4837 = {
        let a4836 = v0.clone();
        length_list(a4836)
    };
    let a4838 = 1u64;
    if nat_le(a4837, a4838) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f751(v0)?))
    }
}

pub fn f720(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f527({
        let (_, h4839) = {
            let (h4840, _) = f752(Fn22::F753, {
                let a4842 = {
                    let a4841 = v0.clone();
                    length_list(a4841)
                };
                let a4843 = 1u64;
                nat_add(a4842, a4843)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h4840
        };
        h4839
    }))
}

pub fn f721(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f754(v1, v0, v2)
}

pub fn f722(v0: u64, v1: Bytes, v2: u64, v3: u64) -> R<Option<u64>> {
    f755(v1, v2, v0, v3)
}

pub fn f723(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let a4846 = {
        let (h4844, _) = v1;
        h4844
    };
    let a4847 = {
        let a4845 = v0.clone();
        length_bytes(a4845)
    };
    if nat_lt(a4846, a4847) {
        Ok(Some((f308(v0, {
            let (h4848, _) = v1;
            h4848
        })?, {
            let a4850 = {
                let (_, h4849) = v1;
                h4849
            };
            let a4851 = 1u64;
            nat_add(a4850, a4851)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f724(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f756(v1, v0, v2)
}

pub fn f725(v0: u64) -> Option<u64> {
    let a4852 = 48u64;
    let a4853 = v0;
    if if nat_le(a4852, a4853) {
        let a4854 = v0;
        let a4855 = 57u64;
        nat_le(a4854, a4855)
    } else {
        false
    } {
        Some({
            let a4856 = v0;
            let a4857 = 48u64;
            nat_sub(a4856, a4857)
        })
    } else {
        let a4858 = 97u64;
        let a4859 = v0;
        if if nat_le(a4858, a4859) {
            let a4860 = v0;
            let a4861 = 102u64;
            nat_le(a4860, a4861)
        } else {
            false
        } {
            Some({
                let a4862 = v0;
                let a4863 = 87u64;
                nat_sub(a4862, a4863)
            })
        } else {
            let a4864 = 65u64;
            let a4865 = v0;
            if if nat_le(a4864, a4865) {
                let a4866 = v0;
                let a4867 = 70u64;
                nat_le(a4866, a4867)
            } else {
                false
            } {
                Some({
                    let a4868 = v0;
                    let a4869 = 55u64;
                    nat_sub(a4868, a4869)
                })
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f726(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f757(v0, v1)
}

pub fn f727(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f758(v0, v1)
}

pub fn f728(v0: Bytes, v1: u64, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    let v3: u64 = {
        let (h4870, _) = v2;
        h4870
    };
    let v4: u64 = f209(v0.clone(), v3)?;
    let (_, h4871) = v2;
    if h4871 {
        Ok(None::<(u64, bool)>)
    } else {
        if f304(v4) {
            Ok(Some(({
                let a4872 = v3;
                let a4873 = 1u64;
                nat_add(a4872, a4873)?
            }, false)))
        } else {
            let a4874 = v4;
            let a4875 = 35u64;
            if if nat_eq(a4874, a4875) {
                let a4876 = v3;
                let a4877 = 0u64;
                if if nat_eq(a4876, a4877) {
                    true
                } else {
                    f304(f209(v0.clone(), {
                        let a4878 = v3;
                        let a4879 = 1u64;
                        nat_sub(a4878, a4879)
                    })?)
                } {
                    true
                } else {
                    let a4882 = f209(v0.clone(), {
                        let a4880 = v3;
                        let a4881 = 1u64;
                        nat_sub(a4880, a4881)
                    })?;
                    let a4883 = 10u64;
                    nat_eq(a4882, a4883)
                }
            } else {
                false
            } {
                Ok(Some((f424(v0, v3)?, false)))
            } else {
                let a4884 = v4;
                let a4885 = 10u64;
                if nat_eq(a4884, a4885) {
                    let v5: u64 = {
                        let a4886 = v3;
                        let a4887 = 1u64;
                        nat_add(a4886, a4887)?
                    };
                    let v6: u64 = f309(v0.clone(), v5)?;
                    let v7: u64 = f307(v0.clone(), v6)?;
                    if if f207(v0.clone(), v5)? {
                        true
                    } else {
                        let a4890 = {
                            let a4888 = v6;
                            let a4889 = v5;
                            nat_sub(a4888, a4889)
                        };
                        let a4891 = v1;
                        if if nat_lt(a4890, a4891) {
                            f211(f305(f209(v0.clone(), v7)?))
                        } else {
                            false
                        } {
                            f211({
                                let a4892 = f209(v0, v7)?;
                                let a4893 = 35u64;
                                nat_eq(a4892, a4893)
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

pub fn f729(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f307(v0.clone(), {
        let (h4894, _) = v1;
        h4894
    })?;
    let a4895 = f209(v0, v2)?;
    let a4896 = 10u64;
    if nat_eq(a4895, a4896) {
        Ok(Some(({
            let a4897 = v2;
            let a4898 = 1u64;
            nat_add(a4897, a4898)?
        }, {
            let a4900 = {
                let (_, h4899) = v1;
                h4899
            };
            let a4901 = 1u64;
            nat_add(a4900, a4901)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f730(v0: u64, v1: u64) -> bool {
    let a4902 = v0;
    let a4903 = 0u64;
    if nat_eq(a4902, a4903) {
        f570(v1)
    } else {
        let a4904 = v0;
        let a4905 = 1u64;
        if nat_eq(a4904, a4905) {
            f569(v1)
        } else {
            let a4906 = v0;
            let a4907 = 2u64;
            if nat_eq(a4906, a4907) {
                f759(v1)
            } else {
                f760(v1)
            }
        }
    }
}

pub fn f731(v0: bool) -> Adt15 {
    let v1: Bytes = if v0 {
        f148()
    } else {
        f146()
    };
    Adt15::C0(0u64, f7(List::cons(f8(Str::lit("(List ")), List::cons(v1.clone(), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))), List::cons(v1, List::<Bytes>::nil()), Bytes::lit(&[]), Bytes::lit(&[]))
}

pub fn f732(v0: Bytes, v1: u64) -> R<u64> {
    let a4909 = v1;
    let a4910 = {
        let a4908 = v0.clone();
        length_bytes(a4908)
    };
    if nat_lt(a4909, a4910) {
        let a4911 = f163(v0, v1)?;
        let a4912 = 1u64;
        nat_add(a4911, a4912)
    } else {
        Ok(0u64)
    }
}

pub fn f733(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f183(f113(f183(f283(v2), f284(v2)), f113({
        let a4913 = v2;
        let a4914 = 117u64;
        !nat_eq(a4913, a4914)
    }, {
        let a4915 = v2;
        let a4916 = 85u64;
        !nat_eq(a4915, a4916)
    })), f113(f285(v2), {
        let a4917 = 0u64;
        let a4918 = v1;
        nat_lt(a4917, a4918)
    })) {
        f46(v0, v1, 1u64)
    } else {
        let a4919 = v2;
        let a4920 = 65536u64;
        if nat_lt(a4919, a4920) {
            let a4921 = f8(Str::lit("u"));
            let a4922 = f589(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a4921, a4922)
        } else {
            let a4923 = f8(Str::lit("U"));
            let a4924 = f589(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a4923, a4924)
        }
    }
}

pub fn f734(v0: u64) -> Bytes {
    let a4925 = f8(Str::lit("u"));
    let a4926 = f589(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a4925, a4926)
}

pub fn f735(v0: u64, v1: bool) -> u64 {
    let a4927 = v0;
    let a4928 = 48u64;
    if nat_lt(a4927, a4928) {
        16u64
    } else {
        let a4929 = v0;
        let a4930 = 57u64;
        if nat_le(a4929, a4930) {
            let a4931 = v0;
            let a4932 = 48u64;
            nat_sub(a4931, a4932)
        } else {
            if v1 {
                let a4933 = v0;
                let a4934 = 65u64;
                if nat_lt(a4933, a4934) {
                    16u64
                } else {
                    let a4935 = v0;
                    let a4936 = 70u64;
                    if nat_le(a4935, a4936) {
                        let a4937 = v0;
                        let a4938 = 55u64;
                        nat_sub(a4937, a4938)
                    } else {
                        let a4939 = v0;
                        let a4940 = 97u64;
                        if nat_lt(a4939, a4940) {
                            16u64
                        } else {
                            let a4941 = v0;
                            let a4942 = 102u64;
                            if nat_le(a4941, a4942) {
                                let a4943 = v0;
                                let a4944 = 87u64;
                                nat_sub(a4943, a4944)
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

pub fn f736() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f737() -> Bytes {
    f8(Str::lit("\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{0}]\u{0}\u{0}]\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f738(v0: Adt26) -> R<Result<Option<(Adt27, (Bytes, Bytes))>, Bytes>> {
    let m4946 = f614(match v0 {
        Adt26::C0(_, _, _, h4945) => {
            h4945
        }
    })?;
    match m4946 {
        Ok(v2) => {
            let m4947 = f615(v2, List::cons(f8(Str::lit("simpleContent")), List::cons(f8(Str::lit("complexContent")), List::<Bytes>::nil())));
            match m4947 {
                None => {
                    Ok(Ok::<Option<(Adt27, (Bytes, Bytes))>, Bytes>(None::<(Adt27, (Bytes, Bytes))>))
                }
                Some(v3) => {
                    let m4948 = f614(v3)?;
                    match m4948 {
                        Ok(v5) => {
                            let m4949 = f615(v5, List::cons(f8(Str::lit("extension")), List::cons(f8(Str::lit("restriction")), List::<Bytes>::nil())));
                            match m4949 {
                                None => {
                                    Ok(Ok::<Option<(Adt27, (Bytes, Bytes))>, Bytes>(None::<(Adt27, (Bytes, Bytes))>))
                                }
                                Some(v6) => {
                                    let m4950 = f381(v6.clone(), Str::lit("base"));
                                    match m4950 {
                                        None => {
                                            Ok(Ok::<Option<(Adt27, (Bytes, Bytes))>, Bytes>(None::<(Adt27, (Bytes, Bytes))>))
                                        }
                                        Some(v7) => {
                                            let m4952 = f167(match v6.clone() {
                                                Adt27::C0(_, _, _, _, h4951) => {
                                                    h4951
                                                }
                                            }, v7)?;
                                            match m4952 {
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

pub fn f739(v0: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f761(f337(v0.clone())?, v0, List::<Bytes>::nil())
}

pub fn f740(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m4953 = v0;
    match m4953.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m4954 = v1;
            match m4954.uncons() {
                None => {
                    List::<Bytes>::nil()
                }
                Some((v4, v5)) => {
                    List::cons(f260(f8(Str::lit("    {name: ")), v2, f260(f8(Str::lit(", fields: [")), v4, f8(Str::lit("]}")))), f740(v3, v5))
                }
            }
        }
    }
}

pub fn f741(v0: u64, v1: Adt25, v2: Bytes, v3: Bytes, v4: Adt27, v5: Adt36) -> R<Result<Adt36, Bytes>> {
    let m4955 = v0;
    if m4955 == 0 {
        Ok(Err::<Adt36, Bytes>(f616(f8(Str::lit("the type derivations nest deeper than the schema documents have bytes, so the types derive from each other in a cycle (XSD 1.1 Part 1, section 3.4.6.1)")), v2)))
    } else {
        let v6 = m4955 - 1;
        if f693(v4.clone(), Str::lit("mixed"), Str::lit("true")) {
            Ok(Err::<Adt36, Bytes>(f616(f8(Str::lit("a mixed content type interleaves character data with its elements, which a LexLean field list does not keep in order, so it is not translated (XSD 1.1 Part 1, section 3.4.2.3)")), v2)))
        } else {
            let m4956 = f614(v4)?;
            match m4956 {
                Ok(v8) => {
                    let m4957 = f615(v8.clone(), List::cons(f8(Str::lit("openContent")), List::cons(f8(Str::lit("assert")), List::<Bytes>::nil())));
                    match m4957 {
                        None => {
                            let m4959 = f615(v8.clone(), List::cons(f8(Str::lit("simpleContent")), List::cons(f8(Str::lit("complexContent")), List::<Bytes>::nil())));
                            match m4959 {
                                None => {
                                    let m4960 = f701(v6, v1.clone(), v2.clone(), v3.clone(), true, v8.clone(), v5)?;
                                    match m4960 {
                                        Ok(v11) => {
                                            f618(v6, v1, v2, v3, v8, v11)
                                        }
                                        Err(v10) => {
                                            Ok(Err::<Adt36, Bytes>(v10))
                                        }
                                    }
                                }
                                Some(v12) => {
                                    if f693(v12.clone(), Str::lit("mixed"), Str::lit("true")) {
                                        Ok(Err::<Adt36, Bytes>(f616(f8(Str::lit("a mixed content type interleaves character data with its elements, which a LexLean field list does not keep in order, so it is not translated (XSD 1.1 Part 1, section 3.4.2.3)")), v2)))
                                    } else {
                                        let m4961 = f614(v12.clone())?;
                                        match m4961 {
                                            Ok(v14) => {
                                                let m4962 = f615(v14, List::cons(f8(Str::lit("extension")), List::cons(f8(Str::lit("restriction")), List::<Bytes>::nil())));
                                                match m4962 {
                                                    None => {
                                                        Ok(Err::<Adt36, Bytes>(f616(f8(Str::lit("simpleContent and complexContent must have an extension or a restriction child (XSD 1.1 Part 1, sections 3.4.2.2 and 3.4.2.3)")), v2)))
                                                    }
                                                    Some(v15) => {
                                                        let m4963 = f381(v15.clone(), Str::lit("base"));
                                                        match m4963 {
                                                            None => {
                                                                Ok(Err::<Adt36, Bytes>(f616(f8(Str::lit("an extension or restriction must name its base type (XSD 1.1 Part 1, section 3.4.3)")), v2)))
                                                            }
                                                            Some(v16) => {
                                                                let m4965 = f167(match v15.clone() {
                                                                    Adt27::C0(_, _, _, _, h4964) => {
                                                                        h4964
                                                                    }
                                                                }, v16.clone())?;
                                                                match m4965 {
                                                                    Ok(v18) => {
                                                                        let m4966 = f614(v15.clone())?;
                                                                        match m4966 {
                                                                            Ok(v20) => {
                                                                                let v21: bool = if f32({
                                                                                    let (h4967, _) = v18.clone();
                                                                                    h4967
                                                                                }, f250()) {
                                                                                    f32({
                                                                                        let (_, h4968) = v18.clone();
                                                                                        h4968
                                                                                    }, f8(Str::lit("anyType")))
                                                                                } else {
                                                                                    false
                                                                                };
                                                                                if if f380(v15.clone(), Str::lit("restriction")) {
                                                                                    f379(v21)
                                                                                } else {
                                                                                    false
                                                                                } {
                                                                                    Ok(Err::<Adt36, Bytes>(f616(f260(f8(Str::lit("a restriction of the complex type ")), v16, f8(Str::lit(" is not translated, because the attribute uses it inherits and the content it narrows are not read yet (XSD 1.1 Part 1, sections 3.4.2.2 and 3.4.2.3)"))), v2)))
                                                                                } else {
                                                                                    let m4983 = if if v21 {
                                                                                        true
                                                                                    } else {
                                                                                        f32(match v12 {
                                                                                            Adt27::C0(_, h4969, _, _, _) => {
                                                                                                h4969
                                                                                            }
                                                                                        }, f8(Str::lit("complexContent")))
                                                                                    } {
                                                                                        if v21 {
                                                                                            Ok::<Adt36, Bytes>(v5)
                                                                                        } else {
                                                                                            let m4970 = f706(v1.clone(), Str::lit("complexType"), v18.clone());
                                                                                            match m4970 {
                                                                                                None => {
                                                                                                    Err::<Adt36, Bytes>(f616(f260(f8(Str::lit("the base type ")), v16, f8(Str::lit(" of a complex content extension is not a complex type of this schema set (XSD 1.1 Part 1, section 3.4.2.3)"))), v2.clone()))
                                                                                                }
                                                                                                Some(v22) => {
                                                                                                    let m4974 = f741(v6, v1.clone(), f125({
                                                                                                        let (_, h4971) = v18;
                                                                                                        h4971
                                                                                                    })?, match v22.clone() {
                                                                                                        Adt26::C0(_, h4972, _, _) => {
                                                                                                            h4972
                                                                                                        }
                                                                                                    }, match v22 {
                                                                                                        Adt26::C0(_, _, _, h4973) => {
                                                                                                            h4973
                                                                                                        }
                                                                                                    }, v5)?;
                                                                                                    match m4974 {
                                                                                                        Ok(v24) => {
                                                                                                            Ok::<Adt36, Bytes>(f762(v24))
                                                                                                        }
                                                                                                        Err(v23) => {
                                                                                                            Err::<Adt36, Bytes>(v23)
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        let m4975 = f706(v1.clone(), Str::lit("complexType"), v18.clone());
                                                                                        match m4975 {
                                                                                            None => {
                                                                                                let m4980 = f699(v1.clone(), v15, v16)?;
                                                                                                match m4980 {
                                                                                                    Ok(v29) => {
                                                                                                        Ok::<Adt36, Bytes>(f708(v5, Adt37::C0(f8(Str::lit("value")), match v29.clone() {
                                                                                                            Adt24::C0(h4981, _) => {
                                                                                                                h4981
                                                                                                            }
                                                                                                        }, match v29 {
                                                                                                            Adt24::C0(_, h4982) => {
                                                                                                                h4982
                                                                                                            }
                                                                                                        })))
                                                                                                    }
                                                                                                    Err(v28) => {
                                                                                                        Err::<Adt36, Bytes>(f616(v28, v2.clone()))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            Some(v25) => {
                                                                                                let m4979 = f741(v6, v1.clone(), f125({
                                                                                                    let (_, h4976) = v18;
                                                                                                    h4976
                                                                                                })?, match v25.clone() {
                                                                                                    Adt26::C0(_, h4977, _, _) => {
                                                                                                        h4977
                                                                                                    }
                                                                                                }, match v25 {
                                                                                                    Adt26::C0(_, _, _, h4978) => {
                                                                                                        h4978
                                                                                                    }
                                                                                                }, v5)?;
                                                                                                match m4979 {
                                                                                                    Ok(v27) => {
                                                                                                        Ok::<Adt36, Bytes>(f762(v27))
                                                                                                    }
                                                                                                    Err(v26) => {
                                                                                                        Err::<Adt36, Bytes>(v26)
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    };
                                                                                    match m4983 {
                                                                                        Ok(v31) => {
                                                                                            let m4984 = f701(v6, v1.clone(), v2.clone(), v3.clone(), true, v20.clone(), v31)?;
                                                                                            match m4984 {
                                                                                                Ok(v33) => {
                                                                                                    f618(v6, v1, v2, v3, v20, v33)
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
                                                                        Ok(Err::<Adt36, Bytes>(f616(v17, v2)))
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
                            Ok(Err::<Adt36, Bytes>(f616(f260(f8(Str::lit("xs:")), match v9 {
                                Adt27::C0(_, h4958, _, _, _) => {
                                    h4958
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

pub fn f742(v0: Str, v1: Bytes) -> Bytes {
    f260(f8(Str::lit("(")), f8(v0), f260(f8(Str::lit(" ")), v1, f8(Str::lit(")"))))
}

pub fn f743(v0: u64, v1: Adt25, v2: Bytes, v3: Bytes, v4: bool, v5: Adt27, v6: Adt36) -> R<Result<Adt36, Bytes>> {
    let m4985 = v0;
    if m4985 == 0 {
        Ok(Err::<Adt36, Bytes>(f616(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v7 = m4985 - 1;
        if f380(v5.clone(), Str::lit("element")) {
            let m4986 = f763(v7, v1, v2, v3, v5.clone())?;
            match m4986 {
                Ok(v9) => {
                    let m4992 = f764(v5, {
                        let m4991 = {
                            let m4990 = match v9.clone() {
                                Adt40::C0(_, h4989) => {
                                    h4989
                                }
                            };
                            match m4990 {
                                Adt35::C0(h4988, _) => {
                                    h4988
                                }
                            }
                        };
                        match m4991 {
                            Adt24::C0(h4987, _) => {
                                h4987
                            }
                        }
                    });
                    match m4992 {
                        None => {
                            Ok(Ok::<Adt36, Bytes>(v6))
                        }
                        Some(v10) => {
                            Ok(Ok::<Adt36, Bytes>(f746(f708(v6, Adt37::C0(match v9.clone() {
                                Adt40::C0(h4993, _) => {
                                    h4993
                                }
                            }, v10, {
                                let m4998 = {
                                    let m4997 = match v9.clone() {
                                        Adt40::C0(_, h4996) => {
                                            h4996
                                        }
                                    };
                                    match m4997 {
                                        Adt35::C0(h4995, _) => {
                                            h4995
                                        }
                                    }
                                };
                                match m4998 {
                                    Adt24::C0(_, h4994) => {
                                        h4994
                                    }
                                }
                            })), {
                                let m5001 = match v9 {
                                    Adt40::C0(_, h5000) => {
                                        h5000
                                    }
                                };
                                match m5001 {
                                    Adt35::C0(_, h4999) => {
                                        h4999
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
            if f380(v5.clone(), Str::lit("any")) {
                let m5002 = f764(v5, f492());
                match m5002 {
                    None => {
                        Ok(Ok::<Adt36, Bytes>(v6))
                    }
                    Some(v11) => {
                        Ok(Ok::<Adt36, Bytes>(f708(v6, Adt37::C0(f8(Str::lit("any")), v11, List::cons(f492(), List::<Bytes>::nil())))))
                    }
                }
            } else {
                if f380(v5.clone(), Str::lit("group")) {
                    let m5003 = f381(v5.clone(), Str::lit("ref"));
                    match m5003 {
                        None => {
                            Ok(Err::<Adt36, Bytes>(f616(f8(Str::lit("a model group reference must have a ref (XSD 1.1 Part 1, section 3.7.2)")), v2)))
                        }
                        Some(v12) => {
                            let m5005 = f167(match v5.clone() {
                                Adt27::C0(_, _, _, _, h5004) => {
                                    h5004
                                }
                            }, v12.clone())?;
                            match m5005 {
                                Ok(v14) => {
                                    let m5006 = f706(v1, Str::lit("group"), v14.clone());
                                    match m5006 {
                                        None => {
                                            Ok(Err::<Adt36, Bytes>(f616(f260(f8(Str::lit("the model group ")), v12, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                        }
                                        Some(_) => {
                                            let v16: Bytes = f125({
                                                let (_, h5007) = v14.clone();
                                                h5007
                                            })?;
                                            let m5008 = f764(v5, v16.clone());
                                            match m5008 {
                                                None => {
                                                    Ok(Ok::<Adt36, Bytes>(v6))
                                                }
                                                Some(v17) => {
                                                    Ok(Ok::<Adt36, Bytes>(f708(v6, Adt37::C0({
                                                        let (_, h5009) = v14;
                                                        h5009
                                                    }, v17, List::cons(v16, List::<Bytes>::nil())))))
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(v13) => {
                                    Ok(Err::<Adt36, Bytes>(f616(v13, v2)))
                                }
                            }
                        }
                    }
                } else {
                    if if f380(v5.clone(), Str::lit("sequence")) {
                        true
                    } else {
                        if f380(v5.clone(), Str::lit("choice")) {
                            true
                        } else {
                            f380(v5.clone(), Str::lit("all"))
                        }
                    } {
                        if if v4 {
                            if f765(v5.clone()) {
                                f379(f380(v5.clone(), Str::lit("choice")))
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m5010 = f614(v5)?;
                            match m5010 {
                                Ok(v19) => {
                                    f701(v7, v1, v2, v3, true, v19, v6)
                                }
                                Err(v18) => {
                                    Ok(Err::<Adt36, Bytes>(v18))
                                }
                            }
                        } else {
                            let v20: Bytes = match v5.clone() {
                                Adt27::C0(_, h5011, _, _, _) => {
                                    h5011
                                }
                            };
                            let v21: u64 = f766(v6.clone(), v20.clone())?;
                            let v22: Bytes = {
                                let a5012 = v21;
                                let a5013 = 1u64;
                                if nat_eq(a5012, a5013) {
                                    Bytes::lit(&[])
                                } else {
                                    f91(v21)?
                                }
                            };
                            let v23: Bytes = f260(v2, f125(v20.clone())?, v22.clone());
                            let m5014 = f617(v7, v1, v23.clone(), v3, v5.clone())?;
                            match m5014 {
                                Ok(v25) => {
                                    let m5015 = f764(v5, v23.clone());
                                    match m5015 {
                                        None => {
                                            Ok(Ok::<Adt36, Bytes>(f767(v6, v20)?))
                                        }
                                        Some(v26) => {
                                            Ok(Ok::<Adt36, Bytes>(f746(f708(f767(v6, v20.clone())?, Adt37::C0({
                                                let a5016 = v20;
                                                let a5017 = v22;
                                                append_bytes(a5016, a5017)
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

pub fn f744(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m5018 = v0;
    match m5018.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m5019 = v1;
            match m5019.uncons() {
                None => {
                    List::<Bytes>::nil()
                }
                Some((v4, v5)) => {
                    List::cons(f260(f8(Str::lit("    {name: ")), v2, f260(f8(Str::lit(", type: ")), v4, f8(Str::lit("}")))), f744(v3, v5))
                }
            }
        }
    }
}

pub fn f745(p5025: List<Adt26>, v1: Bytes, v2: Bytes, v3: Bytes) -> Option<Adt26> {
    let m5024 = p5025;
    {
        let mut m5024 = m5024;
        loop {
            let step = {
                let v0 = m5024;
                let m5020 = v0;
                match m5020.uncons() {
                    None => {
                        Err::<List<Adt26>, Option<Adt26>>(None::<Adt26>)
                    }
                    Some((v4, v5)) => {
                        if if f32(match v4.clone() {
                            Adt26::C0(h5021, _, _, _) => {
                                h5021
                            }
                        }, v1.clone()) {
                            if f32(match v4.clone() {
                                Adt26::C0(_, h5022, _, _) => {
                                    h5022
                                }
                            }, v2.clone()) {
                                f32(match v4.clone() {
                                    Adt26::C0(_, _, h5023, _) => {
                                        h5023
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
                Ok(next) => { m5024 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f746(v0: Adt36, v1: List<Adt28>) -> Adt36 {
    Adt36::C0(match v0.clone() {
        Adt36::C0(h5026, _, _, _, _) => {
            h5026
        }
    }, f489(match v0.clone() {
        Adt36::C0(_, h5027, _, _, _) => {
            h5027
        }
    }, v1), match v0.clone() {
        Adt36::C0(_, _, h5028, _, _) => {
            h5028
        }
    }, match v0.clone() {
        Adt36::C0(_, _, _, h5029, _) => {
            h5029
        }
    }, match v0 {
        Adt36::C0(_, _, _, _, h5030) => {
            h5030
        }
    })
}

pub fn f747(v0: List<Adt37>, v1: List<Adt37>) -> List<Adt37> {
    let m5031 = v0;
    match m5031.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f747(v3, v1))
        }
    }
}

pub fn f748(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a5076 = {
            let a5052 = {
                let a5040 = {
                    let a5034 = {
                        let a5032 = v1;
                        let a5033 = 128u8;
                        fixed_u8::bit_and(a5032, a5033)
                    };
                    let a5035 = 128u8;
                    if equal(a5034, a5035) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a5041 = {
                    let a5038 = {
                        let a5036 = v1;
                        let a5037 = 64u8;
                        fixed_u8::bit_and(a5036, a5037)
                    };
                    let a5039 = 64u8;
                    if equal(a5038, a5039) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a5040, a5041)?
            };
            let a5053 = {
                let a5050 = {
                    let a5044 = {
                        let a5042 = v1;
                        let a5043 = 32u8;
                        fixed_u8::bit_and(a5042, a5043)
                    };
                    let a5045 = 32u8;
                    if equal(a5044, a5045) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a5051 = {
                    let a5048 = {
                        let a5046 = v1;
                        let a5047 = 16u8;
                        fixed_u8::bit_and(a5046, a5047)
                    };
                    let a5049 = 16u8;
                    if equal(a5048, a5049) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a5050, a5051)?
            };
            nat_add(a5052, a5053)?
        };
        let a5077 = {
            let a5074 = {
                let a5062 = {
                    let a5056 = {
                        let a5054 = v1;
                        let a5055 = 8u8;
                        fixed_u8::bit_and(a5054, a5055)
                    };
                    let a5057 = 8u8;
                    if equal(a5056, a5057) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a5063 = {
                    let a5060 = {
                        let a5058 = v1;
                        let a5059 = 4u8;
                        fixed_u8::bit_and(a5058, a5059)
                    };
                    let a5061 = 4u8;
                    if equal(a5060, a5061) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a5062, a5063)?
            };
            let a5075 = {
                let a5072 = {
                    let a5066 = {
                        let a5064 = v1;
                        let a5065 = 2u8;
                        fixed_u8::bit_and(a5064, a5065)
                    };
                    let a5067 = 2u8;
                    if equal(a5066, a5067) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a5073 = {
                    let a5070 = {
                        let a5068 = v1;
                        let a5069 = 1u8;
                        fixed_u8::bit_and(a5068, a5069)
                    };
                    let a5071 = 1u8;
                    if equal(a5070, a5071) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a5072, a5073)?
            };
            nat_add(a5074, a5075)?
        };
        nat_add(a5076, a5077)?
    };
    let a5078 = 48u64;
    let a5079 = v2;
    if if nat_le(a5078, a5079) {
        let a5080 = v2;
        let a5081 = 57u64;
        nat_le(a5080, a5081)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a5082 = 97u64;
        let a5083 = v2;
        if nat_le(a5082, a5083) {
            let a5084 = v2;
            let a5085 = 102u64;
            Ok(nat_le(a5084, a5085))
        } else {
            Ok(false)
        }
    }
}

pub fn f749() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f750(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a5130 = {
            let a5106 = {
                let a5094 = {
                    let a5088 = {
                        let a5086 = v1;
                        let a5087 = 128u8;
                        fixed_u8::bit_and(a5086, a5087)
                    };
                    let a5089 = 128u8;
                    if equal(a5088, a5089) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a5095 = {
                    let a5092 = {
                        let a5090 = v1;
                        let a5091 = 64u8;
                        fixed_u8::bit_and(a5090, a5091)
                    };
                    let a5093 = 64u8;
                    if equal(a5092, a5093) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a5094, a5095)?
            };
            let a5107 = {
                let a5104 = {
                    let a5098 = {
                        let a5096 = v1;
                        let a5097 = 32u8;
                        fixed_u8::bit_and(a5096, a5097)
                    };
                    let a5099 = 32u8;
                    if equal(a5098, a5099) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a5105 = {
                    let a5102 = {
                        let a5100 = v1;
                        let a5101 = 16u8;
                        fixed_u8::bit_and(a5100, a5101)
                    };
                    let a5103 = 16u8;
                    if equal(a5102, a5103) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a5104, a5105)?
            };
            nat_add(a5106, a5107)?
        };
        let a5131 = {
            let a5128 = {
                let a5116 = {
                    let a5110 = {
                        let a5108 = v1;
                        let a5109 = 8u8;
                        fixed_u8::bit_and(a5108, a5109)
                    };
                    let a5111 = 8u8;
                    if equal(a5110, a5111) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a5117 = {
                    let a5114 = {
                        let a5112 = v1;
                        let a5113 = 4u8;
                        fixed_u8::bit_and(a5112, a5113)
                    };
                    let a5115 = 4u8;
                    if equal(a5114, a5115) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a5116, a5117)?
            };
            let a5129 = {
                let a5126 = {
                    let a5120 = {
                        let a5118 = v1;
                        let a5119 = 2u8;
                        fixed_u8::bit_and(a5118, a5119)
                    };
                    let a5121 = 2u8;
                    if equal(a5120, a5121) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a5127 = {
                    let a5124 = {
                        let a5122 = v1;
                        let a5123 = 1u8;
                        fixed_u8::bit_and(a5122, a5123)
                    };
                    let a5125 = 1u8;
                    if equal(a5124, a5125) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a5126, a5127)?
            };
            nat_add(a5128, a5129)?
        };
        nat_add(a5130, a5131)?
    };
    let a5132 = v2;
    let a5133 = 57u64;
    if nat_le(a5132, a5133) {
        let a5134 = v2;
        let a5135 = 48u64;
        Ok(nat_sub(a5134, a5135))
    } else {
        let a5136 = v2;
        let a5137 = 87u64;
        Ok(nat_sub(a5136, a5137))
    }
}

pub fn f751(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f641({
        let (_, h5138) = {
            let (h5139, _) = f768(Fn23::F769, {
                let a5141 = {
                    let a5140 = v0.clone();
                    length_list(a5140)
                };
                let a5142 = 1u64;
                nat_add(a5141, a5142)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h5139
        };
        h5138
    }))
}

pub fn f752(v0: Fn22, p5147: u64, p5148: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m5146 = (p5147, p5148);
    {
        let mut m5146 = m5146;
        loop {
            let step = {
                let (v1, v2) = m5146;
                let m5143 = v1;
                if m5143 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m5143 - 1;
                    let m5145 = {
                        let c5144: Fn22 = v0.clone();
                        c5144.apply(v2.clone())?
                    };
                    match m5145 {
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
                Ok(next) => { m5146 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f753(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f770(v0)
}

pub fn f754(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f654(v0.clone(), {
        let (h5149, _) = v2;
        h5149
    }, v1)?;
    let a5150 = f209(v0, v3)?;
    let a5151 = 10u64;
    if nat_eq(a5150, a5151) {
        Ok(Some(({
            let a5152 = v3;
            let a5153 = 1u64;
            nat_add(a5152, a5153)?
        }, {
            let a5155 = {
                let (_, h5154) = v2;
                h5154
            };
            let a5156 = 1u64;
            nat_add(a5155, a5156)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f755(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a5157 = f209(v0, v3)?;
    let a5158 = 32u64;
    if if nat_eq(a5157, a5158) {
        let a5161 = {
            let a5159 = v3;
            let a5160 = v1;
            nat_sub(a5159, a5160)
        };
        let a5162 = v2;
        nat_lt(a5161, a5162)
    } else {
        false
    } {
        Ok(Some({
            let a5163 = v3;
            let a5164 = 1u64;
            nat_add(a5163, a5164)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f756(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m5166 = f725(f209(v0, {
        let (h5165, _) = v2;
        h5165
    })?);
    match m5166 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v3) => {
            let a5167 = v3;
            let a5168 = v1;
            if nat_lt(a5167, a5168) {
                Ok(Some(({
                    let a5170 = {
                        let (h5169, _) = v2;
                        h5169
                    };
                    let a5171 = 1u64;
                    nat_add(a5170, a5171)?
                }, {
                    let a5175 = {
                        let a5173 = {
                            let (_, h5172) = v2;
                            h5172
                        };
                        let a5174 = v1;
                        nat_mul(a5173, a5174)?
                    };
                    let a5176 = v3;
                    nat_add(a5175, a5176)?
                })))
            } else {
                Ok(None::<(u64, u64)>)
            }
        }
    }
}

pub fn f757(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f209(v0, v1)?;
    let a5177 = v2;
    let a5178 = 34u64;
    if if if if nat_eq(a5177, a5178) {
        true
    } else {
        let a5179 = v2;
        let a5180 = 92u64;
        nat_eq(a5179, a5180)
    } {
        true
    } else {
        f304(v2)
    } {
        true
    } else {
        f305(v2)
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a5181 = v1;
            let a5182 = 1u64;
            nat_add(a5181, a5182)?
        }))
    }
}

pub fn f758(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f209(v0, v1)?;
    let a5183 = v2;
    let a5184 = 39u64;
    if if if nat_eq(a5183, a5184) {
        true
    } else {
        f304(v2)
    } {
        true
    } else {
        f305(v2)
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a5185 = v1;
            let a5186 = 1u64;
            nat_add(a5185, a5186)?
        }))
    }
}

pub fn f759(v0: u64) -> bool {
    let a5187 = 48u64;
    let a5188 = v0;
    if nat_le(a5187, a5188) {
        let a5189 = v0;
        let a5190 = 55u64;
        nat_le(a5189, a5190)
    } else {
        false
    }
}

pub fn f760(v0: u64) -> bool {
    if f441(v0) {
        true
    } else {
        if f570(v0) {
            true
        } else {
            let a5191 = v0;
            let a5192 = 95u64;
            if nat_eq(a5191, a5192) {
                true
            } else {
                let a5193 = v0;
                let a5194 = 45u64;
                nat_eq(a5193, a5194)
            }
        }
    }
}

pub fn f761(v0: bool, v1: List<Bytes>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m5195 = v1;
    match m5195.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m5196 = f469(v0, v3.clone())?;
            match m5196 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f260(f8(Str::lit("the component name ")), v3, f8(Str::lit(" derives no LexLean name")))))
                }
                Some(v5) => {
                    if f122(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f260(f8(Str::lit("two components derive the name ")), v5, f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n; when those names are not distinct, every name keeps its letters and digits and writes each other character as a C11 universal character name)")))))
                    } else {
                        let m5197 = f761(v0, v4, List::cons(v5.clone(), v2))?;
                        match m5197 {
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

pub fn f762(v0: Adt36) -> Adt36 {
    Adt36::C0(match v0.clone() {
        Adt36::C0(h5198, _, _, _, _) => {
            h5198
        }
    }, match v0 {
        Adt36::C0(_, h5199, _, _, _) => {
            h5199
        }
    }, 0u64, 0u64, 0u64)
}

pub fn f763(v0: u64, v1: Adt25, v2: Bytes, v3: Bytes, v4: Adt27) -> R<Result<Adt40, Bytes>> {
    let m5200 = v0;
    if m5200 == 0 {
        Ok(Err::<Adt40, Bytes>(f616(f8(Str::lit("the element declarations nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m5200 - 1;
        let m5201 = f381(v4.clone(), Str::lit("ref"));
        match m5201 {
            None => {
                let m5212 = f381(v4.clone(), Str::lit("name"));
                match m5212 {
                    None => {
                        Ok(Err::<Adt40, Bytes>(f616(f8(Str::lit("a local element declaration must have a name or a ref (XSD 1.1 Part 1, section 3.3.3)")), v2)))
                    }
                    Some(v12) => {
                        let m5215 = f612(v5, v1, {
                            let a5213 = v2;
                            let a5214 = f125(v12.clone())?;
                            append_bytes(a5213, a5214)
                        }, v3, v4)?;
                        match m5215 {
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
                let m5203 = f167(match v4 {
                    Adt27::C0(_, _, _, _, h5202) => {
                        h5202
                    }
                }, v6.clone())?;
                match m5203 {
                    Ok(v8) => {
                        let m5204 = f706(v1.clone(), Str::lit("element"), v8.clone());
                        match m5204 {
                            None => {
                                Ok(Err::<Adt40, Bytes>(f616(f260(f8(Str::lit("the element ")), v6, f8(Str::lit(" is not declared by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                            }
                            Some(v9) => {
                                if if f693(match v9.clone() {
                                    Adt26::C0(_, _, _, h5205) => {
                                        h5205
                                    }
                                }, Str::lit("abstract"), Str::lit("true")) {
                                    true
                                } else {
                                    f771(match v1.clone() {
                                        Adt25::C0(h5206, _) => {
                                            h5206
                                        }
                                    }, v8.clone())?
                                } {
                                    Ok(Err::<Adt40, Bytes>(f616(f260(f8(Str::lit("the element ")), v6, f8(Str::lit(" heads a substitution group, so its particle also admits the members of that group, which is not translated (XSD 1.1 Part 1, section 3.3.6.4)"))), v2)))
                                } else {
                                    let m5210 = f612(v5, v1, f125({
                                        let (_, h5207) = v8.clone();
                                        h5207
                                    })?, match v9.clone() {
                                        Adt26::C0(_, h5208, _, _) => {
                                            h5208
                                        }
                                    }, match v9 {
                                        Adt26::C0(_, _, _, h5209) => {
                                            h5209
                                        }
                                    })?;
                                    match m5210 {
                                        Ok(v11) => {
                                            Ok(Ok::<Adt40, Bytes>(Adt40::C0({
                                                let (_, h5211) = v8;
                                                h5211
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
                        Ok(Err::<Adt40, Bytes>(f616(v7, v2)))
                    }
                }
            }
        }
    }
}

pub fn f764(v0: Adt27, v1: Bytes) -> Option<Bytes> {
    let v2: Bytes = f384(v0.clone(), Str::lit("maxOccurs"), Str::lit("1"));
    if f32(v2.clone(), f8(Str::lit("0"))) {
        None::<Bytes>
    } else {
        if f32(v2, f8(Str::lit("1"))) {
            if f693(v0, Str::lit("minOccurs"), Str::lit("0")) {
                Some(f742(Str::lit("Option"), v1))
            } else {
                Some(v1)
            }
        } else {
            Some(f742(Str::lit("List"), v1))
        }
    }
}

pub fn f765(v0: Adt27) -> bool {
    if f32(f384(v0.clone(), Str::lit("minOccurs"), Str::lit("1")), f8(Str::lit("1"))) {
        f32(f384(v0, Str::lit("maxOccurs"), Str::lit("1")), f8(Str::lit("1")))
    } else {
        false
    }
}

pub fn f766(v0: Adt36, v1: Bytes) -> R<u64> {
    if f32(v1.clone(), f8(Str::lit("sequence"))) {
        let a5217 = match v0 {
            Adt36::C0(_, _, h5216, _, _) => {
                h5216
            }
        };
        let a5218 = 1u64;
        nat_add(a5217, a5218)
    } else {
        if f32(v1, f8(Str::lit("choice"))) {
            let a5220 = match v0 {
                Adt36::C0(_, _, _, h5219, _) => {
                    h5219
                }
            };
            let a5221 = 1u64;
            nat_add(a5220, a5221)
        } else {
            let a5223 = match v0 {
                Adt36::C0(_, _, _, _, h5222) => {
                    h5222
                }
            };
            let a5224 = 1u64;
            nat_add(a5223, a5224)
        }
    }
}

pub fn f767(v0: Adt36, v1: Bytes) -> R<Adt36> {
    if f32(v1.clone(), f8(Str::lit("sequence"))) {
        Ok(Adt36::C0(match v0.clone() {
            Adt36::C0(h5225, _, _, _, _) => {
                h5225
            }
        }, match v0.clone() {
            Adt36::C0(_, h5226, _, _, _) => {
                h5226
            }
        }, {
            let a5228 = match v0.clone() {
                Adt36::C0(_, _, h5227, _, _) => {
                    h5227
                }
            };
            let a5229 = 1u64;
            nat_add(a5228, a5229)?
        }, match v0.clone() {
            Adt36::C0(_, _, _, h5230, _) => {
                h5230
            }
        }, match v0 {
            Adt36::C0(_, _, _, _, h5231) => {
                h5231
            }
        }))
    } else {
        if f32(v1, f8(Str::lit("choice"))) {
            Ok(Adt36::C0(match v0.clone() {
                Adt36::C0(h5232, _, _, _, _) => {
                    h5232
                }
            }, match v0.clone() {
                Adt36::C0(_, h5233, _, _, _) => {
                    h5233
                }
            }, match v0.clone() {
                Adt36::C0(_, _, h5234, _, _) => {
                    h5234
                }
            }, {
                let a5236 = match v0.clone() {
                    Adt36::C0(_, _, _, h5235, _) => {
                        h5235
                    }
                };
                let a5237 = 1u64;
                nat_add(a5236, a5237)?
            }, match v0 {
                Adt36::C0(_, _, _, _, h5238) => {
                    h5238
                }
            }))
        } else {
            Ok(Adt36::C0(match v0.clone() {
                Adt36::C0(h5239, _, _, _, _) => {
                    h5239
                }
            }, match v0.clone() {
                Adt36::C0(_, h5240, _, _, _) => {
                    h5240
                }
            }, match v0.clone() {
                Adt36::C0(_, _, h5241, _, _) => {
                    h5241
                }
            }, match v0.clone() {
                Adt36::C0(_, _, _, h5242, _) => {
                    h5242
                }
            }, {
                let a5244 = match v0 {
                    Adt36::C0(_, _, _, _, h5243) => {
                        h5243
                    }
                };
                let a5245 = 1u64;
                nat_add(a5244, a5245)?
            }))
        }
    }
}

pub fn f768(v0: Fn23, p5250: u64, p5251: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m5249 = (p5250, p5251);
    {
        let mut m5249 = m5249;
        loop {
            let step = {
                let (v1, v2) = m5249;
                let m5246 = v1;
                if m5246 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m5246 - 1;
                    let m5248 = {
                        let c5247: Fn23 = v0.clone();
                        c5247.apply(v2.clone())?
                    };
                    match m5248 {
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
                Ok(next) => { m5249 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f769(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f772(v0)
}

pub fn f770(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h5252, _) = v0;
        h5252
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h5253) = v0;
        h5253
    };
    let m5254 = v1;
    match m5254.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m5255 = v4;
            match m5255.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f773(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f771(p5264: List<Adt26>, v1: (Bytes, Bytes)) -> R<bool> {
    let m5263 = p5264;
    {
        let mut m5263 = m5263;
        loop {
            let step = {
                let v0 = m5263;
                let m5256 = v0;
                match m5256.uncons() {
                    None => {
                        Err::<List<Adt26>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if if f32(match v2.clone() {
                            Adt26::C0(h5257, _, _, _) => {
                                h5257
                            }
                        }, f8(Str::lit("element"))) {
                            let m5259 = f381(match v2.clone() {
                                Adt26::C0(_, _, _, h5258) => {
                                    h5258
                                }
                            }, Str::lit("substitutionGroup"));
                            match m5259 {
                                None => {
                                    false
                                }
                                Some(v4) => {
                                    f774({
                                        let m5262 = match v2 {
                                            Adt26::C0(_, _, _, h5261) => {
                                                h5261
                                            }
                                        };
                                        match m5262 {
                                            Adt27::C0(_, _, _, _, h5260) => {
                                                h5260
                                            }
                                        }
                                    }, f775(v4)?, v1.clone())?
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
                Ok(next) => { m5263 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f772(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h5265, _) = v0;
        h5265
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h5266) = v0;
        h5266
    };
    let m5267 = v1;
    match m5267.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m5268 = v4;
            match m5268.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f776(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f773(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a5273 = {
            let a5271 = {
                let a5269 = v0.clone();
                length_list(a5269)
            };
            let a5272 = {
                let a5270 = v1.clone();
                length_list(a5270)
            };
            nat_add(a5271, a5272)?
        };
        let a5274 = 1u64;
        nat_add(a5273, a5274)?
    };
    Ok(f532({
        let (_, h5275) = {
            let (_, h5276) = {
                let (h5277, _) = f777(Fn24::F778, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h5277
            };
            h5276
        };
        h5275
    }))
}

pub fn f774(v0: List<(Bytes, Bytes)>, p5285: List<Bytes>, v2: (Bytes, Bytes)) -> R<bool> {
    let m5284 = p5285;
    {
        let mut m5284 = m5284;
        loop {
            let step = {
                let v1 = m5284;
                let m5278 = v1;
                match m5278.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v3, v4)) => {
                        let m5279 = f167(v0.clone(), v3)?;
                        match m5279 {
                            Ok(v6) => {
                                if if f32({
                                    let (h5280, _) = v6;
                                    h5280
                                }, {
                                    let (h5281, _) = v2.clone();
                                    h5281
                                }) {
                                    f32({
                                        let (_, h5282) = v6;
                                        h5282
                                    }, {
                                        let (_, h5283) = v2.clone();
                                        h5283
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
                Ok(next) => { m5284 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f775(v0: Bytes) -> R<List<Bytes>> {
    f44(v0, 32u8)
}

pub fn f776(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a5290 = {
            let a5288 = {
                let a5286 = v0.clone();
                length_list(a5286)
            };
            let a5289 = {
                let a5287 = v1.clone();
                length_list(a5287)
            };
            nat_add(a5288, a5289)?
        };
        let a5291 = 1u64;
        nat_add(a5290, a5291)?
    };
    Ok(f779({
        let (_, h5292) = {
            let (_, h5293) = {
                let (h5294, _) = f780(Fn25::F781, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h5294
            };
            h5293
        };
        h5292
    }))
}

pub fn f777(v0: Fn24, p5299: u64, p5300: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m5298 = (p5299, p5300);
    {
        let mut m5298 = m5298;
        loop {
            let step = {
                let (v1, v2) = m5298;
                let m5295 = v1;
                if m5295 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m5295 - 1;
                    let m5297 = {
                        let c5296: Fn24 = v0.clone();
                        c5296.apply(v2.clone())
                    };
                    match m5297 {
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
                Ok(next) => { m5298 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f778(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f782(v0)
}

pub fn f779(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f407(Fn8::F783, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f780(v0: Fn25, p5305: u64, p5306: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m5304 = (p5305, p5306);
    {
        let mut m5304 = m5304;
        loop {
            let step = {
                let (v1, v2) = m5304;
                let m5301 = v1;
                if m5301 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m5301 - 1;
                    let m5303 = {
                        let c5302: Fn25 = v0.clone();
                        c5302.apply(v2.clone())
                    };
                    match m5303 {
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
                Ok(next) => { m5304 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f781(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f784(v0)
}

pub fn f782(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h5307, _) = v0.clone();
        h5307
    };
    let v2: List<(Bytes, ())> = {
        let (h5308, _) = {
            let (_, h5309) = v0.clone();
            h5309
        };
        h5308
    };
    let v3: List<(Bytes, ())> = {
        let (_, h5310) = {
            let (_, h5311) = v0;
            h5311
        };
        h5310
    };
    let m5312 = v1.clone();
    match m5312.uncons() {
        None => {
            let m5313 = v2;
            match m5313.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m5314 = v2.clone();
            match m5314.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f785({
                        let (h5315, _) = v8.clone();
                        h5315
                    }, {
                        let (h5316, _) = v6.clone();
                        h5316
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

pub fn f783(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f784(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h5317, _) = v0.clone();
        h5317
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h5318, _) = {
            let (_, h5319) = v0.clone();
            h5319
        };
        h5318
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h5320) = {
            let (_, h5321) = v0;
            h5321
        };
        h5320
    };
    let m5322 = v1.clone();
    match m5322.uncons() {
        None => {
            let m5323 = v2;
            match m5323.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m5324 = v2.clone();
            match m5324.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f785({
                        let (h5325, _) = v8.clone();
                        h5325
                    }, {
                        let (h5326, _) = v6.clone();
                        h5326
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

pub fn f785(v0: Bytes, v1: Bytes) -> bool {
    let a5331 = {
        let a5327 = v0;
        let a5328 = v1;
        compare_bytes(a5327, a5328)
    };
    let a5332 = {
        let a5329 = Bytes::lit(&[]);
        let a5330 = Bytes::lit(&[0x01]);
        compare_bytes(a5329, a5330)
    };
    equal(a5331, a5332)
}

pub fn run(p0: Option<Adt0>, p1: Adt3) -> R<(Adt0, Adt10)> {
    f0(p0, p1)
}
